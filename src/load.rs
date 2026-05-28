//! Source loader: parses sexp text into the AST, resolving identifiers
//! to BVars or Calls based on lexical scope, and constructor names
//! against the module's declared types.
//!
//! Top-level forms recognized:
//!   (type NAME                 (CTOR FIELDTYPES…)…)
//!   (type (NAME TYPEPARAMS…)   (CTOR FIELDTYPES…)…)
//!   (fn   NAME ((P TY)…) RET BODY)
//!   (extern NAME ((P TY)…) RET)
//!
//! Within a body, identifiers resolve in this order:
//!   1. Local binding (parameter, pattern var, let-bound) → BVar
//!   2. Constructor name (any arity, including bare zero-arg) → Ctor
//!   3. Anything else at the head of a list → Call
//!   4. Anything else as a bare identifier → FVar
//!
//! Reserved special forms (override the head-symbol lookup):
//!   if, match, let, quote, list, ty
//!
//! `list` expands at parse time to a Cons/Nil chain — `(list a b c)`
//! becomes `(Cons a (Cons b (Cons c Nil)))`. `(list)` is `Nil`.
//! Sexp reader macro `'foo` is handled by lexpr itself as a rewrite
//! to `(quote foo)`.
//!
//! `ty` builds a Type *value* (the `(TCon Symbol (List Type))` shape):
//! `(ty Int)` → `(TCon 'Int (list))`; `(ty List Int)` → nested TCons;
//! bare symbols inside `ty` are interpreted as 0-ary type names, not
//! as FVars. Outside `ty`, types are written via explicit `TCon` /
//! `TVar` ctor applications.
//!
//! Module loading is two-pass: first scan to collect type/ctor names,
//! then load bodies with that knowledge. Lets us recognize `Nil`,
//! `Cons`, etc. uniformly regardless of declaration order.
//!
//! BVar convention is innermost-first: the LAST item in any binding
//! group (last fn param, last pattern var, last let binding) becomes
//! `BVar 0`. See REVISIT.md, "Pattern binding order: innermost-first".

use std::collections::HashSet;

use lexpr::Value;
use lexpr::parse::Parser;

use crate::ast::*;

#[derive(Debug)]
pub enum LoadError {
    Parse(String),
    UnknownForm(String),
    BadShape(String),
    Io(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Parse(s) => write!(f, "parse error: {s}"),
            LoadError::UnknownForm(s) => write!(f, "unknown top-level form: {s}"),
            LoadError::BadShape(s) => write!(f, "bad shape: {s}"),
            LoadError::Io(s) => write!(f, "io error: {s}"),
        }
    }
}

impl std::error::Error for LoadError {}

/// Load a module from multiple source files, concatenating their
/// contents. Order does not matter for resolution (two-pass loader),
/// but file order is preserved in any later iteration over `fns`,
/// `types`, etc.
pub fn module_from_paths<P: AsRef<std::path::Path>>(paths: &[P]) -> Result<Module, LoadError> {
    module_from_paths_with_base(paths, None)
}

/// Like `module_from_paths` but additionally treats `base`'s ctor
/// names as in-scope when parsing the loaded module's fn bodies and
/// patterns. Used by the `check` binary to let user modules reference
/// kernel stdlib types (List / Cons / Nil, Option / Some / None, …)
/// without redeclaring them.
///
/// The loaded module's own types are still produced as declared; the
/// base is consulted only for ctor *name resolution* during parsing.
pub fn module_from_paths_with_base<P: AsRef<std::path::Path>>(
    paths: &[P],
    base: Option<&Module>,
) -> Result<Module, LoadError> {
    let mut combined = String::new();
    for p in paths {
        let p = p.as_ref();
        let contents = std::fs::read_to_string(p)
            .map_err(|e| LoadError::Io(format!("reading {}: {}", p.display(), e)))?;
        combined.push_str(&contents);
        combined.push('\n');
    }
    module_from_str_with_base(&combined, base)
}

/// Parse and load a module from sexp source text. Multiple top-level
/// forms are accepted (any mix of `type` / `fn` / `extern`).
pub fn module_from_str(src: &str) -> Result<Module, LoadError> {
    module_from_str_with_base(src, None)
}

/// Same as `module_from_str` with an optional `base` module whose
/// ctor names augment the in-scope ctor set during parsing.
pub fn module_from_str_with_base(
    src: &str,
    base: Option<&Module>,
) -> Result<Module, LoadError> {
    // Parse all top-level forms once; load in two passes so types are
    // known before bodies reference their ctors.
    let values = parse_all(src)?;

    let mut module = Module::default();

    // Pass 1: type definitions (so ctor names are known for bodies).
    for v in &values {
        let parts = as_list(v)?;
        let head = parts
            .first()
            .ok_or_else(|| LoadError::BadShape("empty top-level form".into()))?;
        if as_symbol(head)? == "type" {
            module.types.push(load_type_def(&parts[1..])?);
        }
    }

    let mut ctors = ctor_set(&module);
    if let Some(b) = base {
        ctors.extend(ctor_set(b));
    }

    // Pass 2: fns and externs. Skip types (already loaded).
    for v in &values {
        let parts = as_list(v)?;
        let head_sym = as_symbol(parts[0])?;
        match head_sym {
            "type" => {}
            "fn" => module.fns.push(load_fn_def(&parts[1..], &ctors)?),
            "extern" => module.externs.push(load_extern_def(&parts[1..])?),
            other => return Err(LoadError::UnknownForm(other.into())),
        }
    }

    Ok(module)
}

/// Parse a single expression against a module's ctor set.
pub fn expr_from_str(src: &str, module: &Module) -> Result<Expr, LoadError> {
    let v: Value = src
        .parse()
        .map_err(|e: lexpr::parse::Error| LoadError::Parse(e.to_string()))?;
    expr_from_value(&v, module)
}

/// Convert a pre-parsed `lexpr::Value` to a narrow `Expr` against the
/// module's ctor set. The string-input variant `expr_from_str` is a
/// thin wrapper around this. Useful when the caller has already walked
/// the sexp surface (e.g., the `check` binary scanning a proof file
/// for `(claim …)` forms).
pub fn expr_from_value(v: &Value, module: &Module) -> Result<Expr, LoadError> {
    let ctors = ctor_set(module);
    let mut ctx = LoadCtx::new();
    load_expr(v, &mut ctx, &ctors)
}

fn parse_all(src: &str) -> Result<Vec<Value>, LoadError> {
    let mut parser = Parser::from_str(src);
    let mut out = Vec::new();
    loop {
        match parser.next_value() {
            Ok(Some(v)) => out.push(v),
            Ok(None) => return Ok(out),
            Err(e) => return Err(LoadError::Parse(e.to_string())),
        }
    }
}

fn ctor_set(module: &Module) -> HashSet<Symbol> {
    module
        .types
        .iter()
        .flat_map(|td| td.ctors.iter().map(|cd| cd.name.clone()))
        .collect()
}

// -----------------------------------------------------------------------------
// Top-level forms
// -----------------------------------------------------------------------------

/// `(type NAME (CTOR FIELDS…)…)` or `(type (NAME PARAMS…) (CTOR FIELDS…)…)`.
fn load_type_def(parts: &[&Value]) -> Result<TypeDef, LoadError> {
    if parts.is_empty() {
        return Err(LoadError::BadShape("type: missing name and ctors".into()));
    }
    let (name, params) = if let Some(sym) = parts[0].as_symbol() {
        (sym.to_string(), Vec::new())
    } else {
        // `(NAME P1 P2 …)` head form
        let head = as_list(parts[0])?;
        let name = as_symbol(head[0])?.to_string();
        let mut ps = Vec::with_capacity(head.len() - 1);
        for p in &head[1..] {
            ps.push(as_symbol(p)?.to_string());
        }
        (name, ps)
    };

    let mut ctors = Vec::new();
    for cv in &parts[1..] {
        let cp = as_list(cv)?;
        if cp.is_empty() {
            return Err(LoadError::BadShape("ctor: missing name".into()));
        }
        let cname = as_symbol(cp[0])?.to_string();
        let mut fields = Vec::with_capacity(cp.len() - 1);
        for f in &cp[1..] {
            // Field types are parsed in the scope of the typedef's
            // params — bare `T` in `(Cons T (List T))` for `(type
            // (List T) …)` is a TVar, not a TCon "T". The kernel's
            // `type_subst` only fires on TVar, so getting this
            // distinction right is what lets `do_induct` produce
            // proper IHs for recursive fields.
            fields.push(load_type_in_scope(f, &params)?);
        }
        ctors.push(CtorDef {
            name: cname,
            fields,
        });
    }
    Ok(TypeDef {
        name,
        params,
        ctors,
    })
}

/// `fn NAME ((P TY) …) RET BODY`
fn load_fn_def(parts: &[&Value], ctors: &HashSet<Symbol>) -> Result<FnDef, LoadError> {
    if parts.len() != 4 {
        return Err(LoadError::BadShape(format!(
            "fn: expected (fn NAME PARAMS RET BODY), got {} parts after `fn`",
            parts.len()
        )));
    }
    let name = as_symbol(parts[0])?.to_string();
    let (param_names, param_types) = load_params(parts[1])?;
    let ret = load_type(parts[2])?;

    let mut ctx = LoadCtx::new();
    for n in &param_names {
        ctx.push(n.clone());
    }
    let body = load_expr(parts[3], &mut ctx, ctors)?;

    Ok(FnDef {
        name,
        params: param_types,
        ret,
        body,
    })
}

/// `extern NAME ((P TY) …) RET`
fn load_extern_def(parts: &[&Value]) -> Result<ExternDef, LoadError> {
    if parts.len() != 3 {
        return Err(LoadError::BadShape(format!(
            "extern: expected (extern NAME PARAMS RET), got {} parts after `extern`",
            parts.len()
        )));
    }
    let name = as_symbol(parts[0])?.to_string();
    let (_, param_types) = load_params(parts[1])?;
    let ret = load_type(parts[2])?;
    Ok(ExternDef {
        name,
        params: param_types,
        ret,
    })
}

/// `((P1 T1) (P2 T2) …)` → (names, types) in introduction order.
fn load_params(v: &Value) -> Result<(Vec<Symbol>, Vec<Type>), LoadError> {
    let items = as_list(v)?;
    let mut names = Vec::with_capacity(items.len());
    let mut types = Vec::with_capacity(items.len());
    for item in items {
        let pair = as_list(item)?;
        if pair.len() != 2 {
            return Err(LoadError::BadShape(format!(
                "param: expected (NAME TYPE), got {pair:?}"
            )));
        }
        names.push(as_symbol(pair[0])?.to_string());
        types.push(load_type(pair[1])?);
    }
    Ok((names, types))
}

/// Type expression. Bare symbol `T` → `(TCon T ())`; applied form
/// `(T A B …)` → `(TCon T (A B …))`. `TVar` is not produced here —
/// callers that have type parameters in scope should use
/// `load_type_in_scope` instead.
fn load_type(v: &Value) -> Result<Type, LoadError> {
    load_type_in_scope(v, &[])
}

/// Like `load_type` but treats bare symbols matching one of
/// `tparams` as `TVar`s. Used when parsing the field types of a
/// `(type (NAME PARAMS…) …)` declaration — the kernel's
/// `type_subst` only fires on `TVar`, so the param/TVar
/// correspondence is what makes generic ctors substitute correctly
/// at induction time.
fn load_type_in_scope(v: &Value, tparams: &[Symbol]) -> Result<Type, LoadError> {
    if let Some(sym) = v.as_symbol() {
        if tparams.iter().any(|p| p == sym) {
            return Ok(Type::TVar(sym.to_string()));
        }
        return Ok(Type::TCon(sym.to_string(), Vec::new()));
    }
    let parts = as_list(v)?;
    let head_sym = as_symbol(parts[0])?;
    // Head of an applied form is a type constructor name, never a
    // type variable (variables aren't applicable).
    let mut args = Vec::with_capacity(parts.len() - 1);
    for a in &parts[1..] {
        args.push(load_type_in_scope(a, tparams)?);
    }
    Ok(Type::TCon(head_sym.to_string(), args))
}

// -----------------------------------------------------------------------------
// Expression loading with lexical scope
// -----------------------------------------------------------------------------

struct LoadCtx {
    /// Names in scope; last element is innermost (= BVar 0).
    locals: Vec<Symbol>,
}

impl LoadCtx {
    fn new() -> Self {
        Self { locals: Vec::new() }
    }

    fn push(&mut self, name: Symbol) {
        self.locals.push(name);
    }

    fn truncate(&mut self, len: usize) {
        self.locals.truncate(len);
    }

    fn depth(&self) -> usize {
        self.locals.len()
    }

    fn lookup(&self, name: &str) -> Option<u32> {
        self.locals
            .iter()
            .rev()
            .position(|n| n == name)
            .map(|i| i as u32)
    }
}

fn load_expr(v: &Value, ctx: &mut LoadCtx, ctors: &HashSet<Symbol>) -> Result<Expr, LoadError> {
    // Integer literal
    if let Some(n) = v.as_i64() {
        return Ok(Expr::IntLit(n));
    }
    // Bare identifier
    if let Some(sym) = v.as_symbol() {
        // Local binding takes precedence — pattern vars shadow ctors.
        if let Some(i) = ctx.lookup(sym) {
            return Ok(Expr::BVar(i));
        }
        if ctors.contains(sym) {
            return Ok(Expr::Ctor(sym.to_string(), Vec::new()));
        }
        return Ok(Expr::FVar(sym.to_string()));
    }
    // List form: special form, ctor application, or call
    let parts = as_list(v)?;
    let head = parts
        .first()
        .ok_or_else(|| LoadError::BadShape("empty list expression".into()))?;
    let head_sym = as_symbol(head)?;
    match head_sym {
        "if" => load_if(&parts[1..], ctx, ctors),
        "match" => load_match(&parts[1..], ctx, ctors),
        "let" => load_let(&parts[1..], ctx, ctors),
        "quote" => load_quote(&parts[1..]),
        "list" => load_list(&parts[1..], ctx, ctors),
        "ty" => load_ty(&parts[1..], ctx, ctors),
        _ => {
            let mut args = Vec::with_capacity(parts.len() - 1);
            for a in &parts[1..] {
                args.push(load_expr(a, ctx, ctors)?);
            }
            if ctors.contains(head_sym) {
                Ok(Expr::Ctor(head_sym.to_string(), args))
            } else {
                Ok(Expr::Call(head_sym.to_string(), args))
            }
        }
    }
}

/// `(if C T E)`
fn load_if(parts: &[&Value], ctx: &mut LoadCtx, ctors: &HashSet<Symbol>) -> Result<Expr, LoadError> {
    if parts.len() != 3 {
        return Err(LoadError::BadShape(format!(
            "if: expected (if C T E), got {} args",
            parts.len()
        )));
    }
    let c = load_expr(parts[0], ctx, ctors)?;
    let t = load_expr(parts[1], ctx, ctors)?;
    let e = load_expr(parts[2], ctx, ctors)?;
    Ok(Expr::If(Box::new(c), Box::new(t), Box::new(e)))
}

/// `(match SCRUT ARM…)`
fn load_match(
    parts: &[&Value],
    ctx: &mut LoadCtx,
    ctors: &HashSet<Symbol>,
) -> Result<Expr, LoadError> {
    if parts.is_empty() {
        return Err(LoadError::BadShape("match: expected (match SCRUT ARMS…)".into()));
    }
    let scrut = load_expr(parts[0], ctx, ctors)?;
    let mut arms = Vec::with_capacity(parts.len() - 1);
    for av in &parts[1..] {
        arms.push(load_arm(av, ctx, ctors)?);
    }
    Ok(Expr::Match(Box::new(scrut), arms))
}

/// `(PAT BODY)`. Pushes pattern bindings for body load; restores after.
fn load_arm(
    v: &Value,
    ctx: &mut LoadCtx,
    ctors: &HashSet<Symbol>,
) -> Result<Arm, LoadError> {
    let parts = as_list(v)?;
    if parts.len() != 2 {
        return Err(LoadError::BadShape("arm: expected (PAT BODY)".into()));
    }
    let saved = ctx.depth();
    let pat = load_pat(parts[0], ctx, ctors)?;
    let body = load_expr(parts[1], ctx, ctors)?;
    ctx.truncate(saved);
    Ok(Arm { pat, body })
}

/// Pattern. Pushes a PVar binding to `ctx` for each `PVar` it
/// introduces (left-to-right order; the last PVar in the pattern
/// becomes `BVar 0`).
fn load_pat(
    v: &Value,
    ctx: &mut LoadCtx,
    ctors: &HashSet<Symbol>,
) -> Result<Pat, LoadError> {
    if let Some(n) = v.as_i64() {
        return Ok(Pat::PInt(n));
    }
    if let Some(sym) = v.as_symbol() {
        if ctors.contains(sym) {
            // Bare zero-arg ctor pattern
            return Ok(Pat::PCtor(sym.to_string(), Vec::new()));
        }
        // PVar — including `_`, conventionally for an ignored binding.
        ctx.push(sym.to_string());
        return Ok(Pat::PVar);
    }
    let parts = as_list(v)?;
    if parts.is_empty() {
        return Err(LoadError::BadShape("empty pattern".into()));
    }
    // (quote SYM) → PSym
    if parts.len() == 2 {
        if let Some(h) = parts[0].as_symbol() {
            if h == "quote" {
                if let Some(s) = parts[1].as_symbol() {
                    return Ok(Pat::PSym(s.to_string()));
                }
            }
        }
    }
    // Constructor application with sub-patterns
    let head_sym = as_symbol(parts[0])?;
    if !ctors.contains(head_sym) {
        return Err(LoadError::BadShape(format!(
            "unknown ctor in pattern: {head_sym}"
        )));
    }
    let mut sub_pats = Vec::with_capacity(parts.len() - 1);
    for p in &parts[1..] {
        sub_pats.push(load_pat(p, ctx, ctors)?);
    }
    Ok(Pat::PCtor(head_sym.to_string(), sub_pats))
}

/// `(let ((N1 E1) (N2 E2) …) BODY)`. Parallel let: RHSs evaluated
/// in the outer scope; body sees all bindings.
fn load_let(
    parts: &[&Value],
    ctx: &mut LoadCtx,
    ctors: &HashSet<Symbol>,
) -> Result<Expr, LoadError> {
    if parts.len() != 2 {
        return Err(LoadError::BadShape("let: expected (let BINDINGS BODY)".into()));
    }
    let bindings = as_list(parts[0])?;
    let mut rhss = Vec::with_capacity(bindings.len());
    let mut names = Vec::with_capacity(bindings.len());
    for b in &bindings {
        let bp = as_list(b)?;
        if bp.len() != 2 {
            return Err(LoadError::BadShape(
                "let binding: expected (NAME EXPR)".into(),
            ));
        }
        names.push(as_symbol(bp[0])?.to_string());
        // Parallel: RHS in current (outer) scope
        rhss.push(load_expr(bp[1], ctx, ctors)?);
    }
    let saved = ctx.depth();
    for n in names {
        ctx.push(n);
    }
    let body = load_expr(parts[1], ctx, ctors)?;
    ctx.truncate(saved);
    Ok(Expr::Let(rhss, Box::new(body)))
}

/// `(list E1 E2 …)` → `(Cons E1 (Cons E2 (… Nil)))`. Empty list
/// produces `Nil`. The `Cons` / `Nil` ctor names are hardcoded;
/// callers don't need them to appear in the module's ctor set
/// (though stdlib.sexp declares them so they normally do).
fn load_list(
    parts: &[&Value],
    ctx: &mut LoadCtx,
    ctors: &HashSet<Symbol>,
) -> Result<Expr, LoadError> {
    let mut acc = Expr::Ctor("Nil".into(), Vec::new());
    for p in parts.iter().rev() {
        let head = load_expr(p, ctx, ctors)?;
        acc = Expr::Ctor("Cons".into(), vec![head, acc]);
    }
    Ok(acc)
}

/// `(ty NAME a1 a2 …)` builds a Type *value* `(TCon 'NAME (list a1 a2 …))`.
/// Each `ai` is recursively interpreted as a Type — a bare symbol is
/// treated as a 0-ary type name `(TCon 'Foo (list))`, and a list form
/// `(ty …)` recurses. Other list forms (e.g., explicit `(TVar 'A)`)
/// fall through to the normal expression loader.
///
/// Avoids the verbosity of `(TCon 'List (list (TCon 'Int (list))))`
/// vs. the compact `(ty List Int)`. Reserves `ty` as a special form.
fn load_ty(
    parts: &[&Value],
    ctx: &mut LoadCtx,
    ctors: &HashSet<Symbol>,
) -> Result<Expr, LoadError> {
    if parts.is_empty() {
        return Err(LoadError::BadShape(
            "(ty NAME args…) requires a name".into(),
        ));
    }
    let name = as_symbol(parts[0])?.to_string();
    let mut args = Vec::with_capacity(parts.len() - 1);
    for a in &parts[1..] {
        args.push(load_ty_arg(a, ctx, ctors)?);
    }
    let mut args_chain = Expr::Ctor("Nil".into(), Vec::new());
    for it in args.into_iter().rev() {
        args_chain = Expr::Ctor("Cons".into(), vec![it, args_chain]);
    }
    Ok(Expr::Ctor(
        "TCon".into(),
        vec![Expr::SymLit(name), args_chain],
    ))
}

/// One Type argument inside `(ty …)`. Bare symbol → 0-ary TCon;
/// anything else → normal load_expr (which dispatches to `ty` /
/// handles explicit `TCon` / `TVar` ctor applications).
fn load_ty_arg(
    v: &Value,
    ctx: &mut LoadCtx,
    ctors: &HashSet<Symbol>,
) -> Result<Expr, LoadError> {
    if let Some(sym) = v.as_symbol() {
        return Ok(Expr::Ctor(
            "TCon".into(),
            vec![
                Expr::SymLit(sym.to_string()),
                Expr::Ctor("Nil".into(), Vec::new()),
            ],
        ));
    }
    load_expr(v, ctx, ctors)
}

/// `(quote SYM)` → `SymLit`
fn load_quote(parts: &[&Value]) -> Result<Expr, LoadError> {
    if parts.len() != 1 {
        return Err(LoadError::BadShape(format!(
            "quote: expected (quote SYM), got {} args",
            parts.len()
        )));
    }
    let s = as_symbol(parts[0])?;
    Ok(Expr::SymLit(s.to_string()))
}

// -----------------------------------------------------------------------------
// lexpr Value helpers
// -----------------------------------------------------------------------------

fn as_symbol(v: &Value) -> Result<&str, LoadError> {
    v.as_symbol()
        .ok_or_else(|| LoadError::BadShape(format!("expected symbol, got {v}")))
}

fn as_list(v: &Value) -> Result<Vec<&Value>, LoadError> {
    v.list_iter()
        .map(|it| it.collect::<Vec<_>>())
        .ok_or_else(|| LoadError::BadShape(format!("expected list, got {v}")))
}
