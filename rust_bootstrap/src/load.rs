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
//! A binder `(T Type)` declares a type parameter in scope for the binders
//! after it and the result — the one E's spelling (v3/LANGUAGE.md §8.1
//! rule 3, phase 3 slice 3.2) — wherever `Type` is the sort, that is, not
//! a declared type of the closure: the old tree's `kernel/module.shard`
//! declares a data type `Type` and binds `(t Type)` runtime parameters,
//! which stay runtime parameters. The parameterized head `(fn (NAME T…)
//! …)` and the auto-bound bare type name are the old tree's spellings and
//! stay accepted. A `let` binds SEQUENTIALLY (§5.4, RULED 2026-09-12):
//! each right-hand side sees the bindings before it.
//!
//! Within a body, identifiers resolve in this order:
//!   1. Local binding (parameter, pattern var, let-bound) → BVar
//!   2. Constructor name (any arity, including bare zero-arg) → Ctor
//!   3. Anything else at the head of a list → Call
//!   4. Anything else as a bare identifier → FVar
//!
//! A dotted citation — `Stack.mk`, `json.hex_val`, `kernel.json.hex_val` —
//! resolves to the declared constructor or head it ENDS in (the longest
//! suffix that is a declared name), the flat mirror of the V3 reader's
//! suffix table (v3/LANGUAGE.md §6.7, §8.1 rule 4): this loader has no
//! module paths, so a citation is canonicalized to the declared spelling
//! at load and the evaluator sees one name per declaration. A citation
//! that matches nothing is kept as written (a primitive, or unknown).
//!
//! Reserved special forms (override the head-symbol lookup):
//!   if, match, let, quote, list, ty
//!
//! `list` expands at parse time to a Cons/Nil chain — `(list a b c)`
//! becomes `(Cons a (Cons b (Cons c Nil)))`. `(list)` is `Nil`.
//! A string literal `"x+y"` is the same Cons/Nil chain over its
//! Unicode-scalar codepoints — `String ≡ (List Int)`.
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
    let values = expand_records(parse_all(src)?)?;

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
    // the declared heads, so a dotted call citation canonicalizes (above)
    let mut heads: HashSet<Symbol> = HashSet::new();
    for v in &values {
        let parts = as_list(v)?;
        let h = as_symbol(parts[0])?;
        if (h == "fn" || h == "extern") && parts.len() > 1 {
            if let Some(n) = parts[1].as_symbol() {
                heads.insert(n.to_string());
            } else if let Some(mut it) = parts[1].list_iter() {
                if let Some(n) = it.next().and_then(|x| x.as_symbol()) {
                    heads.insert(n.to_string());
                }
            }
        }
    }
    let mut types: HashSet<Symbol> = module.types.iter().map(|t| t.name.clone()).collect();
    if let Some(b) = base {
        types.extend(b.types.iter().map(|t| t.name.clone()));
    }
    let ctors = Scope { ctors, heads, types };

    // Pass 2: fns and externs. Skip types (already loaded). The proof-
    // script forms (`claim`, `import`, `use-module`) are the check
    // binary's concern, not the module loader's — a co-located topic
    // file mixes code (type/fn/extern) and proofs (claim) plus
    // dependency directives (import); here we load only the CODE part
    // and skip the rest. The check binary processes the same file's
    // imports + claims separately (see bin/check.rs process_file).
    for v in &values {
        let parts = as_list(v)?;
        let head_sym = as_symbol(parts[0])?;
        match head_sym {
            "type" => {}
            "fn" => module.fns.push(load_fn_def(&parts[1..], &ctors)?),
            "extern" => module.externs.push(load_extern_def(&parts[1..], &ctors)?),
            // `app` is the entrypoint declaration consumed by the
            // `check app` driver (state + init + update), not the module
            // loader — skip it here exactly like claim/import.
            // `use`/`sig`/`requirement`/`fulfills`/`bin` are likewise
            // check-layer declarations with no runtime semantics: `use`
            // scoping matters only to STRICT check-time resolution (host
            // dispatch is name-keyed), and the contract forms are the
            // checker driver's concern. Skipping them lets `eval direct`
            // run apps that carry them (e.g. tools/prove). The V3 L forms
            // (`def theorem abbrev opaque inductive structure realize
            // trusts`; v3/LANGUAGE.md §4) are K's, never this executor's:
            // under the one E (§8.1) a file holds them beside its E forms
            // and this loader reads the E half (slice 3.2).
            "claim" | "axiom" | "import" | "use-module" | "app" | "cli" | "use"
            | "sig" | "requirement" | "fulfills" | "bin"
            | "def" | "theorem" | "abbrev" | "opaque" | "inductive" | "structure"
            | "realize" | "trusts" => {}
            other => return Err(LoadError::UnknownForm(other.into())),
        }
    }

    Ok(module)
}

/// The declared names a citation resolves against: the constructors and the
/// heads (fns and externs). A dotted citation canonicalizes to the longest
/// suffix that is declared (the module header above).
pub struct Scope {
    ctors: HashSet<Symbol>,
    heads: HashSet<Symbol>,
    types: HashSet<Symbol>,
}

impl Scope {
    fn of_module(module: &Module) -> Scope {
        let heads = module
            .fns
            .iter()
            .map(|f| f.name.clone())
            .chain(module.externs.iter().map(|e| e.name.clone()))
            .collect();
        let types = module.types.iter().map(|t| t.name.clone()).collect();
        Scope { ctors: ctor_set(module), heads, types }
    }
    /// `Type` is the sort — a `(T Type)` binder is a type parameter — unless
    /// the closure declares a type of that name (the old tree does).
    fn type_is_sort(&self) -> bool {
        !self.types.contains("Type")
    }
    /// The declared constructor a citation names, canonicalized.
    fn ctor(&self, cit: &str) -> Option<String> {
        resolve_in(&self.ctors, cit)
    }
    /// The declared head a citation names, canonicalized; a citation that
    /// matches nothing is kept as written.
    fn head(&self, cit: &str) -> String {
        resolve_in(&self.heads, cit).unwrap_or_else(|| cit.to_string())
    }
}

fn resolve_in(declared: &HashSet<Symbol>, cit: &str) -> Option<String> {
    if declared.contains(cit) {
        return Some(cit.to_string());
    }
    let mut rest = cit;
    while let Some(i) = rest.find('.') {
        rest = &rest[i + 1..];
        if declared.contains(rest) {
            return Some(rest.to_string());
        }
    }
    None
}

/// Parse a single expression against a module's ctor set.
pub fn expr_from_str(src: &str, module: &Module) -> Result<Expr, LoadError> {
    let v: Value = src
        .parse()
        .map_err(|e: lexpr::parse::Error| LoadError::Parse(e.to_string()))?;
    expr_from_value(&v, module)
}

/// Records (v3/LANGUAGE.md §4; kernel/record.shard is the V3 reader's twin, tied by
/// frontend parity): `(record NAME (ctor CTOR)? (FIELD TYPE)+)` — NAME may be
/// `(NAME P…)` — expands before any form is read into the positional type
/// `(type NAME (CTOR TYPE…))` (CTOR defaults to MkNAME), an accessor `FIELD_of`
/// and an updater `with_FIELD` per field; `(make NAME (FIELD V)…)` — every field
/// exactly once — and `(with E (FIELD V)…)` — chained updaters, a later entry
/// outermost — are rewritten in every subterm, nested values first. The V3
/// reader resolves `make` against the current file's records; this flat loader
/// sees the closure's, a looseness the V3 gate refuses.
struct RecDef {
    name: String,
    params: Vec<String>,
    ctor: String,
    fields: Vec<(String, Value)>,
}
fn sym(s: &str) -> Value {
    Value::symbol(s)
}
fn vlist(items: Vec<Value>) -> Value {
    Value::list(items)
}
fn is_head(v: &Value, h: &str) -> bool {
    match v.list_iter() {
        Some(mut it) => it.next().and_then(|x| x.as_symbol()) == Some(h),
        None => false,
    }
}
fn parse_record(v: &Value) -> Result<RecDef, LoadError> {
    let parts = as_list(v)?;
    let bad = |m: &str| LoadError::BadShape(format!("record: {}", m));
    if parts.len() < 3 {
        return Err(bad("(record NAME (ctor CTOR)? (FIELD TYPE)…)"));
    }
    let (name, params) = if let Some(n) = parts[1].as_symbol() {
        (n.to_string(), Vec::new())
    } else {
        let hd = as_list(parts[1])?;
        let n = as_symbol(hd[0])?.to_string();
        let mut ps = Vec::new();
        for p in &hd[1..] {
            ps.push(as_symbol(p)?.to_string());
        }
        (n, ps)
    };
    let mut entries: Vec<&Value> = parts[2..].to_vec();
    let mut ctor = format!("Mk{}", name);
    if let Some(first) = entries.first() {
        let e = as_list(first)?;
        if e.len() == 2 && e[0].as_symbol() == Some("ctor") {
            ctor = as_symbol(e[1])?.to_string();
            entries.remove(0);
        }
    }
    let mut fields: Vec<(String, Value)> = Vec::new();
    for e in entries {
        let e = as_list(e)?;
        if e.len() != 2 {
            return Err(bad(&format!("{}: a field is (FIELD TYPE)", name)));
        }
        let f = as_symbol(e[0])?;
        if f == "ctor" {
            return Err(bad(&format!("{}: ctor is not a field name", name)));
        }
        if fields.iter().any(|(g, _)| g == f) {
            return Err(bad(&format!("{}: a field twice", name)));
        }
        fields.push((f.to_string(), e[1].clone()));
    }
    if fields.is_empty() {
        return Err(bad(&format!("{}: at least one field", name)));
    }
    Ok(RecDef { name, params, ctor, fields })
}
fn head_form(rc: &RecDef) -> Value {
    if rc.params.is_empty() {
        sym(&rc.name)
    } else {
        let mut items = vec![sym(&rc.name)];
        items.extend(rc.params.iter().map(|p| sym(p)));
        vlist(items)
    }
}
fn tparam_binders(rc: &RecDef) -> Vec<Value> {
    rc.params.iter().map(|p| vlist(vec![sym(p), sym("Type")])).collect()
}
fn pattern(rc: &RecDef) -> Value {
    let mut items = vec![sym(&rc.ctor)];
    items.extend(rc.fields.iter().map(|(f, _)| sym(f)));
    vlist(items)
}
fn generate(rc: &RecDef) -> Vec<Value> {
    let mut out = Vec::new();
    let mut ctor_form = vec![sym(&rc.ctor)];
    ctor_form.extend(rc.fields.iter().map(|(_, t)| t.clone()));
    out.push(vlist(vec![sym("type"), head_form(rc), vlist(ctor_form)]));
    for (f, t) in &rc.fields {
        // (fn FIELD_of ((P Type)… (r NAME)) TYPE (match r ((CTOR f…) FIELD)))
        let mut binders = tparam_binders(rc);
        binders.push(vlist(vec![sym("r"), head_form(rc)]));
        out.push(vlist(vec![
            sym("fn"),
            sym(&format!("{}_of", f)),
            vlist(binders),
            t.clone(),
            vlist(vec![sym("match"), sym("r"), vlist(vec![pattern(rc), sym(f)])]),
        ]));
        // (fn with_FIELD ((P Type)… (new_FIELD TYPE) (r NAME)) NAME (match r ((CTOR f…) (CTOR … new_FIELD …))))
        let nv = format!("new_{}", f);
        let mut binders = tparam_binders(rc);
        binders.push(vlist(vec![sym(&nv), t.clone()]));
        binders.push(vlist(vec![sym("r"), head_form(rc)]));
        let mut build = vec![sym(&rc.ctor)];
        build.extend(rc.fields.iter().map(|(g, _)| if g == f { sym(&nv) } else { sym(g) }));
        out.push(vlist(vec![
            sym("fn"),
            sym(&format!("with_{}", f)),
            vlist(binders),
            head_form(rc),
            vlist(vec![sym("match"), sym("r"), vlist(vec![pattern(rc), vlist(build)])]),
        ]));
    }
    out
}
fn rewrite(recs: &[RecDef], v: &Value) -> Result<Value, LoadError> {
    let items: Vec<&Value> = match v.list_iter() {
        Some(it) => it.collect(),
        None => return Ok(v.clone()),
    };
    if items.is_empty() {
        return Ok(v.clone());
    }
    let mut items1: Vec<Value> = Vec::with_capacity(items.len());
    for x in &items {
        items1.push(rewrite(recs, x)?);
    }
    let bad = |m: String| LoadError::BadShape(m);
    match items1[0].as_symbol() {
        Some("make") => {
            let n = items1
                .get(1)
                .and_then(|x| x.as_symbol())
                .ok_or_else(|| bad("(make NAME (FIELD V)…)".into()))?;
            let rc = recs
                .iter()
                .find(|r| r.name == n)
                .ok_or_else(|| bad(format!("make {}: no such record", n)))?;
            let mut given: Vec<(String, Value)> = Vec::new();
            for e in &items1[2..] {
                let e = as_list(e)?;
                if e.len() != 2 {
                    return Err(bad(format!("make {}: an entry is (FIELD V)", n)));
                }
                given.push((as_symbol(e[0])?.to_string(), e[1].clone()));
            }
            if given.len() != rc.fields.len() {
                return Err(bad(format!("make {}: every field exactly once", n)));
            }
            let mut out = vec![sym(&rc.ctor)];
            for (f, _) in &rc.fields {
                let vs: Vec<&Value> = given.iter().filter(|(g, _)| g == f).map(|(_, v)| v).collect();
                if vs.len() != 1 {
                    return Err(bad(format!("make {}: every field exactly once", n)));
                }
                out.push(vs[0].clone());
            }
            Ok(vlist(out))
        }
        Some("with") => {
            if items1.len() < 2 {
                return Err(bad("(with E (FIELD V)…)".into()));
            }
            let mut acc = items1[1].clone();
            for e in &items1[2..] {
                let e = as_list(e)?;
                if e.len() != 2 {
                    return Err(bad("(with E (FIELD V)…)".into()));
                }
                let f = as_symbol(e[0])?;
                acc = vlist(vec![sym(&format!("with_{}", f)), e[1].clone(), acc]);
            }
            Ok(acc)
        }
        _ => Ok(vlist(items1)),
    }
}
fn expand_records(values: Vec<Value>) -> Result<Vec<Value>, LoadError> {
    let mut recs: Vec<RecDef> = Vec::new();
    for v in &values {
        if is_head(v, "record") {
            recs.push(parse_record(v)?);
        }
    }
    if recs.is_empty() {
        return Ok(values);
    }
    let mut out = Vec::with_capacity(values.len());
    for v in &values {
        if is_head(v, "record") {
            out.extend(generate(&parse_record(v)?));
        } else {
            out.push(rewrite(&recs, v)?);
        }
    }
    Ok(out)
}

/// Convert a pre-parsed `lexpr::Value` to a narrow `Expr` against the
/// module's ctor set. The string-input variant `expr_from_str` is a
/// thin wrapper around this. Useful when the caller has already walked
/// the sexp surface (e.g., the `check` binary scanning a proof file
/// for `(claim …)` forms).
pub fn expr_from_value(v: &Value, module: &Module) -> Result<Expr, LoadError> {
    let scope = Scope::of_module(module);
    let mut ctx = LoadCtx::new();
    load_expr(v, &mut ctx, &scope)
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

/// `(fn NAME PARAMS RET BODY)` — monomorphic.
/// `(fn (NAME T1 T2 …) PARAMS RET BODY)` — polymorphic. T1, T2… are
/// type parameters in scope of PARAMS, RET, and the body's type
/// positions. The parameterized-head form mirrors `(type (NAME P…) …)`.
///
/// The body itself doesn't carry type annotations, so tparams only
/// affect how PARAMS and RET are loaded — bare T inside a param/ret
/// type becomes `TVar T` rather than `TCon T []`. This is what the
/// kernel's `type_subst` needs to substitute correctly when the fn's
/// signature is referenced by a polymorphic Goal (see do_induct).
fn load_fn_def(parts: &[&Value], ctors: &Scope) -> Result<FnDef, LoadError> {
    // `(fn NAME PARAMS RET (measure E P…) BODY)` — the optional totality
    // clause (issue #1). Its obligations are generated and checked by the
    // in-shard gate (`check admit` / the check-mode flip); the engine only
    // has to step over it to reach the body.
    let has_measure = parts.len() == 5
        && parts[3]
            .list_iter()
            .and_then(|mut it| it.next())
            .and_then(|h| h.as_symbol())
            == Some("measure");
    if parts.len() != 4 && !has_measure {
        return Err(LoadError::BadShape(format!(
            "fn: expected (fn NAME PARAMS RET BODY) or \
             (fn (NAME T1…) PARAMS RET BODY), got {} parts after `fn`",
            parts.len()
        )));
    }
    let (name, tparams) = if let Some(sym) = parts[0].as_symbol() {
        (sym.to_string(), Vec::new())
    } else {
        let head = as_list(parts[0])?;
        if head.is_empty() {
            return Err(LoadError::BadShape(
                "fn: empty parameterized head ()".into(),
            ));
        }
        let n = as_symbol(head[0])?.to_string();
        let mut ts = Vec::with_capacity(head.len() - 1);
        for t in &head[1..] {
            ts.push(as_symbol(t)?.to_string());
        }
        (n, ts)
    };
    let (param_names, param_types, tps) = load_params_in_scope(parts[1], &tparams, ctors.type_is_sort())?;
    let ret = load_type_in_scope(parts[2], &tps)?;

    let mut ctx = LoadCtx::new();
    for n in &param_names {
        ctx.push(n.clone());
    }
    let body = load_expr(parts[if has_measure { 4 } else { 3 }], &mut ctx, ctors)?;

    Ok(FnDef {
        name,
        params: param_types,
        ret,
        body,
    })
}

/// `(extern NAME PARAMS RET)` — monomorphic.
/// `(extern (NAME T1…) PARAMS RET)` — polymorphic. Same parameterized-
/// head convention as `fn` and `type`.
fn load_extern_def(parts: &[&Value], ctors: &Scope) -> Result<ExternDef, LoadError> {
    if parts.len() != 3 {
        return Err(LoadError::BadShape(format!(
            "extern: expected (extern NAME PARAMS RET) or \
             (extern (NAME T1…) PARAMS RET), got {} parts after `extern`",
            parts.len()
        )));
    }
    let (name, tparams) = if let Some(sym) = parts[0].as_symbol() {
        (sym.to_string(), Vec::new())
    } else {
        let head = as_list(parts[0])?;
        if head.is_empty() {
            return Err(LoadError::BadShape(
                "extern: empty parameterized head ()".into(),
            ));
        }
        let n = as_symbol(head[0])?.to_string();
        let mut ts = Vec::with_capacity(head.len() - 1);
        for t in &head[1..] {
            ts.push(as_symbol(t)?.to_string());
        }
        (n, ts)
    };
    let (_, param_types, tps) = load_params_in_scope(parts[1], &tparams, ctors.type_is_sort())?;
    let ret = load_type_in_scope(parts[2], &tps)?;
    Ok(ExternDef {
        name,
        params: param_types,
        ret,
    })
}

/// `((P1 T1) (P2 T2) …)` → (names, types, type parameters). Each type is
/// parsed against the type parameters in scope — the head's, plus every
/// `(T Type)` binder before it (v3/LANGUAGE.md §8.1 rule 3): such a binder
/// is a type parameter, never a runtime parameter, and bare symbols
/// matching one become `TVar`. The returned list is the scope for the
/// result type.
fn load_params_in_scope(
    v: &Value,
    tparams: &[Symbol],
    type_is_sort: bool,
) -> Result<(Vec<Symbol>, Vec<Type>, Vec<Symbol>), LoadError> {
    let items = as_list(v)?;
    let mut tps: Vec<Symbol> = tparams.to_vec();
    let mut names = Vec::with_capacity(items.len());
    let mut types = Vec::with_capacity(items.len());
    for item in items {
        let pair = as_list(item)?;
        if pair.len() != 2 {
            return Err(LoadError::BadShape(format!(
                "param: expected (NAME TYPE), got {pair:?}"
            )));
        }
        let name = as_symbol(pair[0])?.to_string();
        if type_is_sort && pair[1].as_symbol() == Some("Type") {
            tps.push(name);
            continue;
        }
        names.push(name);
        types.push(load_type_in_scope(pair[1], &tps)?);
    }
    Ok((names, types, tps))
}

/// Type expression. Bare symbol `T` → `(TCon T ())`; applied form
/// `(T A B …)` → `(TCon T (A B …))`. `TVar` is not produced here —
/// callers that have type parameters in scope should use
/// `load_type_in_scope` instead.
///
/// Treats bare symbols matching one of `tparams` as `TVar`s. Used
/// when parsing the field types of a `(type (NAME PARAMS…) …)` or
/// the param/return types of a `(fn (NAME T…) …)` declaration —
/// the kernel's `type_subst` only fires on `TVar`, so the param/TVar
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

fn load_expr(v: &Value, ctx: &mut LoadCtx, ctors: &Scope) -> Result<Expr, LoadError> {
    // Integer literal
    if let Some(n) = int_lit_of(v)? {
        return Ok(Expr::IntLit(n));
    }
    // Bare identifier
    if let Some(sym) = v.as_symbol() {
        // Local binding takes precedence — pattern vars shadow ctors.
        if let Some(i) = ctx.lookup(sym) {
            return Ok(Expr::BVar(i));
        }
        if let Some(c) = ctors.ctor(sym) {
            return Ok(Expr::Ctor(c, Vec::new()));
        }
        return Ok(Expr::FVar(sym.to_string()));
    }
    // String literal: "x+y" → (Cons 120 (Cons 43 (Cons 121 Nil))), the list
    // of the string's UTF-8 BYTES (issue #2 Phase 3 — one meaning end to
    // end: literals, the extern wire, and Bytes payloads all speak bytes).
    // String ≡ (List Int) (docs/LANGUAGE.md), so the std/list lemma library
    // applies to strings unchanged; ASCII literals are unaffected. Checked
    // after symbols so it cannot shadow identifier resolution; valid only
    // in expression position (patterns over strings destructure Cons/Nil).
    if let Some(s) = v.as_str() {
        let mut acc = Expr::Ctor("Nil".into(), Vec::new());
        for b in s.bytes().rev() {
            acc = Expr::Ctor("Cons".into(), vec![Expr::IntLit(b.into()), acc]);
        }
        return Ok(acc);
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
        "tv" => load_tv(&parts[1..]),
        _ => {
            let mut args = Vec::with_capacity(parts.len() - 1);
            for a in &parts[1..] {
                args.push(load_expr(a, ctx, ctors)?);
            }
            if let Some(c) = ctors.ctor(head_sym) {
                Ok(Expr::Ctor(c, args))
            } else {
                Ok(Expr::Call(ctors.head(head_sym), args))
            }
        }
    }
}

/// `(if C T E)`
fn load_if(parts: &[&Value], ctx: &mut LoadCtx, ctors: &Scope) -> Result<Expr, LoadError> {
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
    ctors: &Scope,
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
    ctors: &Scope,
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
    ctors: &Scope,
) -> Result<Pat, LoadError> {
    if let Some(n) = int_lit_of(v)? {
        return Ok(Pat::PInt(n));
    }
    if let Some(sym) = v.as_symbol() {
        if let Some(c) = ctors.ctor(sym) {
            // Bare zero-arg ctor pattern
            return Ok(Pat::PCtor(c, Vec::new()));
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
    let Some(c) = ctors.ctor(head_sym) else {
        return Err(LoadError::BadShape(format!(
            "unknown ctor in pattern: {head_sym}"
        )));
    };
    let mut sub_pats = Vec::with_capacity(parts.len() - 1);
    for p in &parts[1..] {
        sub_pats.push(load_pat(p, ctx, ctors)?);
    }
    Ok(Pat::PCtor(c, sub_pats))
}

/// `(let ((N1 E1) (N2 E2) …) BODY)`. Sequential let (v3/LANGUAGE.md §5.4,
/// RULED 2026-09-12; the bootstrap since phase 3 slice 3.2): each
/// right-hand side sees the bindings before it, the body sees them all,
/// the last innermost. (The tree was measured before the ruling: no
/// existing `let` group's meaning changes.)
fn load_let(
    parts: &[&Value],
    ctx: &mut LoadCtx,
    ctors: &Scope,
) -> Result<Expr, LoadError> {
    if parts.len() != 2 {
        return Err(LoadError::BadShape("let: expected (let BINDINGS BODY)".into()));
    }
    let bindings = as_list(parts[0])?;
    let mut rhss = Vec::with_capacity(bindings.len());
    let saved = ctx.depth();
    for b in &bindings {
        let bp = as_list(b)?;
        if bp.len() != 2 {
            ctx.truncate(saved);
            return Err(LoadError::BadShape(
                "let binding: expected (NAME EXPR)".into(),
            ));
        }
        let name = as_symbol(bp[0])?.to_string();
        match load_expr(bp[1], ctx, ctors) {
            Ok(rhs) => rhss.push(rhs),
            Err(e) => {
                ctx.truncate(saved);
                return Err(e);
            }
        }
        ctx.push(name);
    }
    let body = load_expr(parts[1], ctx, ctors);
    ctx.truncate(saved);
    Ok(Expr::Let(rhss, Box::new(body?)))
}

/// `(list E1 E2 …)` → `(Cons E1 (Cons E2 (… Nil)))`. Empty list
/// produces `Nil`. The `Cons` / `Nil` ctor names are hardcoded;
/// callers don't need them to appear in the module's ctor set
/// (though stdlib.shard declares them so they normally do).
fn load_list(
    parts: &[&Value],
    ctx: &mut LoadCtx,
    ctors: &Scope,
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
    ctors: &Scope,
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
    ctors: &Scope,
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

/// `(tv NAME)` builds a Type *value* `(TVar 'NAME)` — a type variable.
/// Drop-in companion to `(ty …)` for polymorphic claim bodies; lets
/// authors write `(ty List (tv T))` instead of the verbose explicit
/// `(TCon 'List (list (TVar 'T)))`. Reserves `tv` as a special form.
///
/// Whether `NAME` is "in scope" as a tparam is a meta-language
/// question — the kernel just sees a TVar value and uses it for
/// type_subst (in do_induct) and structural Equality (in
/// type_eq, which never fires on TVar = TVar since type_subst always
/// substitutes them first). Authoring discipline keeps tparams
/// consistent across a polymorphic claim's Params.
fn load_tv(parts: &[&Value]) -> Result<Expr, LoadError> {
    if parts.len() != 1 {
        return Err(LoadError::BadShape(format!(
            "tv: expected (tv NAME), got {} args",
            parts.len()
        )));
    }
    let name = as_symbol(parts[0])?.to_string();
    Ok(Expr::Ctor("TVar".into(), vec![Expr::SymLit(name)]))
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

/// Integer literal, if `v` is a number. `Int` is arbitrary-precision, but
/// lexpr parses literals beyond u64/i64 range as lossy f64 — reject those
/// loudly rather than load a corrupted value. (The self-hosted reader
/// builds ints by arithmetic, so it has no such ceiling.)
fn int_lit_of(v: &Value) -> Result<Option<crate::ast::IntLit>, LoadError> {
    if let Some(n) = v.as_i64() {
        return Ok(Some(n.into()));
    }
    if let Some(n) = v.as_u64() {
        return Ok(Some(n.into()));
    }
    if v.is_number() {
        return Err(LoadError::BadShape(format!(
            "integer literal {v} exceeds the bootstrap parser's range (i64/u64); \
             non-integer numerics are not in the language"
        )));
    }
    Ok(None)
}

fn as_list(v: &Value) -> Result<Vec<&Value>, LoadError> {
    v.list_iter()
        .map(|it| it.collect::<Vec<_>>())
        .ok_or_else(|| LoadError::BadShape(format!("expected list, got {v}")))
}
