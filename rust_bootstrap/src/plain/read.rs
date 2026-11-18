//! Reading: types, terms, patterns and declarations as
//! `v3/kernel/classify.shard` reads them (its `read_*` functions, by name
//! where one corresponds), with the tables in K's and the E table's place.

use std::rc::Rc;

use lexpr::Value;

use super::facts::{Fact, Flag, Role};
use super::settle::{self, c_type, pat_types};
use super::{
    Decl, EInfo, ETy, Lit, Name, Native, NativeKind, Pat, R, Res, Scope, Tables, Tm, Unread, append, as_list,
    builtin_type, head_sym, rep_row, strip_at, unread,
};

pub struct Cx<'a> {
    pub t: &'a Tables,
    pub sc: &'a Scope,
}

/// A bound name: a runtime variable with its static type as far as the
/// declarations fix it, or an erased one — a proof, a type parameter; true
/// where it is a proposition or a predicate.
#[derive(Clone, Debug)]
enum Bd {
    Var(String, Option<ETy>),
    Ghost(String, bool),
}

/// The reading context: the bound names outermost first, the fn being read
/// with its written binders' flags (its own call is read by them).
#[derive(Clone, Default)]
struct Rd {
    bound: Vec<Bd>,
    own: Option<(Name, Vec<Flag>)>,
}

enum Hit {
    Var(usize),
    Ghost(bool),
    None,
}

impl Rd {
    fn find(&self, n: &str) -> Hit {
        let mut i = 0;
        for b in self.bound.iter().rev() {
            match b {
                Bd::Var(m, _) => {
                    if m == n {
                        return Hit::Var(i);
                    }
                    i += 1;
                }
                Bd::Ghost(m, p) => {
                    if m == n {
                        return Hit::Ghost(*p);
                    }
                }
            }
        }
        Hit::None
    }
    fn bind(&self, n: &str, t: Option<ETy>) -> Rd {
        let mut r = self.clone();
        r.bound.push(Bd::Var(n.to_string(), t));
        r
    }
    fn ghost(&self, n: &str) -> Rd {
        let mut r = self.clone();
        r.bound.push(Bd::Ghost(n.to_string(), false));
        r
    }
    /// the runtime variables' types, outermost first (the innermost last)
    fn tys(&self) -> Vec<Option<ETy>> {
        self.bound
            .iter()
            .filter_map(|b| match b {
                Bd::Var(_, t) => Some(t.clone()),
                Bd::Ghost(_, _) => None,
            })
            .collect()
    }
}

// -----------------------------------------------------------------------------
// The primitive table and the registry (v3/kernel/prog.shard)
// -----------------------------------------------------------------------------

const PRIMS: &[(&str, u32, usize)] = &[
    ("=", 0, 2),
    ("+", 1, 2),
    ("-", 2, 2),
    ("<", 3, 2),
    ("<=", 4, 2),
    ("band", 6, 2),
    ("bshr", 7, 2),
    ("*", 8, 2),
    ("mod", 9, 2),
    ("/", 10, 2),
    ("tmod", 11, 2),
    ("ediv", 12, 2),
    ("bor", 13, 2),
    ("bxor", 14, 2),
    ("bshl", 15, 2),
    ("Int.tdiv", 18, 2),
    ("Int.tmod", 19, 2),
    ("Int.ediv", 20, 2),
    ("Int.emod", 21, 2),
    ("Nat.sub", 22, 2),
    ("Nat.land", 23, 2),
    ("Nat.lor", 24, 2),
    ("Nat.xor", 25, 2),
    ("Nat.shiftLeft", 26, 2),
    ("Nat.shiftRight", 27, 2),
    ("Nat.add", 28, 2),
    ("Nat.mul", 29, 2),
    ("Nat.div", 30, 2),
    ("Nat.mod", 31, 2),
    ("Nat.gcd", 32, 2),
    ("Nat.pow", 33, 2),
    ("Nat.beq", 34, 2),
    ("Nat.ble", 35, 2),
    ("Nat.decEq", 36, 2),
    ("Nat.decLt", 37, 2),
    ("Nat.decLe", 38, 2),
];

pub fn prim_find(n: &str) -> Option<(u32, usize)> {
    PRIMS.iter().find(|(p, _, _)| *p == n).map(|(_, c, k)| (*c, *k))
}

/// The first of the citation's candidates the table knows.
fn prim_of(sc: &Scope, n: &str) -> Option<(Name, usize)> {
    PRIMS
        .iter()
        .filter_map(|(p, _, k)| sc.rank(p, n).map(|r| (r, *p, *k)))
        .min_by_key(|(r, _, _)| *r)
        .map(|(_, p, k)| (p.to_string(), k))
}

/// The registry's rows: a resolved operation of Init → the table's entry.
pub fn realization_of(c: &str) -> Option<&'static str> {
    Some(match c {
        "Int.add" | "Nat.add" => "+",
        "Int.sub" => "-",
        "Int.mul" | "Nat.mul" => "*",
        "Int.decEq" | "Nat.decEq" => "=",
        "Int.decLt" | "Nat.decLt" => "<",
        "Int.decLe" | "Nat.decLe" => "<=",
        _ => return None,
    })
}

pub fn cmp_prim(p: &str) -> bool {
    matches!(p, "=" | "<" | "<=")
}

/// An expression row's result type, and the rows that are the identity.
pub fn realization_type(c: &str) -> Option<ETy> {
    match c {
        "Int.natAbs" | "Fin.val" => Some(ETy::Con("Nat".into(), vec![])),
        "Int.ofNat" => Some(ETy::Con("Int".into(), vec![])),
        _ => None,
    }
}

fn row_field_proj(sn: &str) -> Option<&'static str> {
    match sn {
        "Fin" => Some("Fin.val"),
        "Subtype" => Some("Subtype.val"),
        _ => None,
    }
}

fn decision_test(id: &str) -> Option<&'static str> {
    realization_of(id).filter(|p| cmp_prim(p))
}

pub fn bool_true() -> Tm {
    Tm::Ctor("Bool.true".into(), vec![])
}
pub fn bool_false() -> Tm {
    Tm::Ctor("Bool.false".into(), vec![])
}
fn dec_true() -> Tm {
    Tm::Ctor("Decidable.isTrue".into(), vec![])
}
fn dec_false() -> Tm {
    Tm::Ctor("Decidable.isFalse".into(), vec![])
}

/// `a = b` at Init's Bool as the test an `if` reads.
pub fn bool_eq_cond(a: Tm, b: Tm) -> Tm {
    if b == bool_true() {
        a
    } else if a == bool_true() {
        b
    } else {
        Tm::If(Box::new(a), Box::new(b.clone()), Box::new(Tm::If(Box::new(b), Box::new(bool_false()), Box::new(bool_true()))))
    }
}

// -----------------------------------------------------------------------------
// Values of the source
// -----------------------------------------------------------------------------

fn int_of(v: &Value) -> R<Option<crate::ast::IntLit>> {
    // a literal past the host parser's range (mod.rs prelex)
    if head_sym(v) == Some("__big") {
        let digits = as_list(v).and_then(|i| i.get(1).and_then(|d| d.as_str()));
        return match digits.and_then(|d| d.parse::<crate::ast::IntLit>().ok()) {
            Some(n) => Ok(Some(n)),
            None => unread("bad_literal", format!("{v}")),
        };
    }
    if let Some(n) = v.as_i64() {
        return Ok(Some(n.into()));
    }
    if let Some(n) = v.as_u64() {
        return Ok(Some(n.into()));
    }
    if v.is_number() {
        return unread("bad_literal", format!("{v}: past the bootstrap parser's integer range"));
    }
    Ok(None)
}

fn num_lit(k: crate::ast::IntLit) -> Lit {
    use num_traits::Signed;
    if k.is_negative() { Lit::Int(k) } else { Lit::Nat(k) }
}

fn list_of(v: &Value) -> Vec<&Value> {
    if v.is_null() { Vec::new() } else { as_list(v).unwrap_or_default() }
}

fn is_type_sym(v: &Value) -> bool {
    v.as_symbol() == Some("Type")
}

/// `Prop`, or a function type into it.
fn is_pred_sort(v: &Value) -> bool {
    if let Some(x) = v.as_symbol() {
        return x == "Prop";
    }
    let items = list_of(v);
    match items.first().and_then(|h| h.as_symbol()) {
        Some("->") => items.last().is_some_and(|l| is_pred_sort(l)),
        _ => false,
    }
}

fn prop_spelling(h: &str) -> bool {
    matches!(h, "=" | "!=" | "<" | "<=" | ">" | ">=" | "iff" | "and" | "or" | "not" | "exists")
}

// -----------------------------------------------------------------------------
// The file's heads, before any body
// -----------------------------------------------------------------------------

/// `(type HEAD (C …)…)`: the type's written name and its constructors' names.
pub fn type_head_ctors(f: &Value) -> Option<(String, Vec<String>)> {
    let items = as_list(f)?;
    let name = match items.get(1)? {
        v if v.as_symbol().is_some() => v.as_symbol()?.to_string(),
        v => head_sym(v)?.to_string(),
    };
    let ctors = items[2..].iter().filter_map(|c| head_sym(c)).map(|s| s.to_string()).collect();
    Some((name, ctors))
}

fn type_params(head: &Value) -> Vec<String> {
    if head.as_symbol().is_some() {
        return Vec::new();
    }
    list_of(head)[1..].iter().filter_map(|p| p.as_symbol()).map(|s| s.to_string()).collect()
}

/// the binders that are arguments: all but `(x Type)`
fn value_binders(bs: &[&Value]) -> usize {
    bs.iter().filter(|b| !list_of(b).get(1).is_some_and(|ty| is_type_sym(ty))).count()
}

/// a binder whose written type is a function type: its fn is a template
fn function_binder(bs: &[&Value]) -> bool {
    bs.iter().any(|b| list_of(b).get(1).is_some_and(|ty| head_sym(ty) == Some("->")))
}

pub fn pre_register(t: &mut Tables, sc: &Scope, f: &Value) {
    let items = list_of(f);
    let Some(h) = items.first().and_then(|h| h.as_symbol()) else { return };
    match h {
        "fn" | "extern" => {
            let (Some(n), Some(bs)) = (items.get(1).and_then(|v| v.as_symbol()), items.get(2)) else { return };
            let bs = list_of(bs);
            let id = sc.qualify(n);
            let info = if h == "extern" {
                EInfo::Ext(value_binders(&bs))
            } else if function_binder(&bs) {
                EInfo::Template
            } else {
                EInfo::Fn(value_binders(&bs))
            };
            t.register(&id, &sc.tag, info);
        }
        "sig" => match items.get(1).and_then(|v| v.as_symbol()) {
            Some("fn") => {
                let (Some(n), Some(bs)) = (items.get(2).and_then(|v| v.as_symbol()), items.get(3)) else { return };
                let id = sc.qualify(n);
                // a fn may take a sig fn's identity; a sig fn never takes a fn's
                if t.einfo(&id).is_none() {
                    t.register(&id, &sc.tag, EInfo::Sig(value_binders(&list_of(bs))));
                }
            }
            Some("type") => {
                let Some(head) = items.get(2) else { return };
                let Some(n) = head.as_symbol().or_else(|| head_sym(head)) else { return };
                let id = sc.qualify(n);
                if t.einfo(&id).is_none() {
                    t.register(&id, &sc.tag, EInfo::SigType(type_params(head).len()));
                }
            }
            _ => {}
        },
        "type" => {
            let Some((n, ctors)) = type_head_ctors(f) else { return };
            let id = sc.qualify(&n);
            let nparams = type_params(items[1]).len();
            let cids: Vec<Name> = ctors.iter().map(|c| append(&id, c)).collect();
            for (i, c) in items[2..].iter().enumerate() {
                let nfields = list_of(c).len().saturating_sub(1);
                t.register(&cids[i], &sc.tag, EInfo::Ctor { ind: id.clone(), ord: i, nfields });
            }
            t.register(&id, &sc.tag, EInfo::Ind { nparams, ctors: cids });
        }
        _ => {}
    }
}

/// The declarations of the logic (their names and kinds) and the types.
pub fn declare(t: &mut Tables, sc: &Rc<Scope>, f: &Value) {
    let items = list_of(f);
    let Some(h) = items.first().and_then(|h| h.as_symbol()) else { return };
    let kind = match h {
        "def" | "abbrev" => Some(NativeKind::Def),
        "theorem" => Some(NativeKind::Theorem),
        "axiom" => Some(NativeKind::Axiom),
        "opaque" => Some(NativeKind::Opaque),
        "structure" => Some(NativeKind::Structure),
        "inductive" => Some(NativeKind::Inductive),
        _ => None,
    };
    if let Some(kind) = kind {
        let Some(n) = items.get(1).and_then(|v| v.as_symbol().or_else(|| head_sym(v))) else { return };
        // (def NAME BINDERS RET VALUE), (opaque NAME BINDERS RET), (axiom NAME TYPE): the
        // constant's type ends in Prop where its written result does
        let prop = match kind {
            NativeKind::Def | NativeKind::Opaque => items.get(3).is_some_and(|r| is_pred_sort(r)),
            NativeKind::Axiom => items.get(2).is_some_and(|r| is_pred_sort(r)),
            _ => false,
        };
        let form = if kind == NativeKind::Def { Some((sc.clone(), f.clone())) } else { None };
        t.add_native(sc.qualify(n), Native { module: sc.tag.clone(), kind, prop, form });
        return;
    }
    match h {
        "type" => {
            let Some((n, _)) = type_head_ctors(f) else { return };
            let id = sc.qualify(&n);
            match read_etype_decl(&Cx { t, sc }, &id, &items) {
                Ok(d) => t.add_decl(d),
                Err(u) => t.unread.push((id, u)),
            }
        }
        "sig" if items.get(1).and_then(|v| v.as_symbol()) == Some("type") => {
            let Some(head) = items.get(2) else { return };
            let Some(n) = head.as_symbol().or_else(|| head_sym(head)) else { return };
            let id = sc.qualify(n);
            if t.find_decl(&id).is_none() {
                t.add_decl(Decl::SigType { name: id, arity: type_params(head).len() });
            }
        }
        _ => {}
    }
}

/// (type HEAD (CTOR FIELD-TYPE…)…): the field types over the declared parameters.
fn read_etype_decl(cx: &Cx, id: &str, items: &[&Value]) -> R<Decl> {
    let params = type_params(items[1]);
    let mut ctors = Vec::new();
    for c in &items[2..] {
        let c = list_of(c);
        let Some(cn) = c.first().and_then(|v| v.as_symbol()) else {
            return unread("bad_constructor", id);
        };
        let mut fts = Vec::new();
        for ft in &c[1..] {
            fts.push(read_ety(cx, &params, ft)?);
        }
        ctors.push((append(id, cn), fts));
    }
    Ok(Decl::Ind { name: id.to_string(), nparams: params.len(), ctors })
}

/// The identity a realize form gives a program: its target, resolved.
fn realize_target(t: &Tables, sc: &Scope, items: &[&Value]) -> Option<Name> {
    let n = items.get(1)?.as_symbol()?;
    t.k_name(sc, n)
}

/// A supplied realize: (realize NAME BINDERS RET …). The derived view,
/// (realize NAME (view)), has a list that is no binder list in BINDERS' place.
fn realize_supplied<'a>(items: &[&'a Value]) -> Option<Vec<&'a Value>> {
    if items.len() < 5 {
        return None;
    }
    Some(list_of(items[2]))
}

/// (realize NAME …): the realized constant's identity as a function head of
/// this file — its arity the written binders', or for the derived view the
/// count of its runtime binders.
pub fn pre_realize(t: &mut Tables, sc: &Rc<Scope>, f: &Value) {
    match head_sym(f) {
        Some("realize") => {}
        // a derivation writes functions this reader does not generate
        Some("derive") => {
            t.unread.push((sc.tag.clone(), Unread("derive", format!("{f}"))));
            return;
        }
        _ => return,
    }
    let items = list_of(f);
    let Some(k) = realize_target(t, sc, &items) else { return };
    let info = match realize_supplied(&items) {
        Some(bs) => {
            if function_binder(&bs) {
                EInfo::Template
            } else {
                EInfo::Fn(value_binders(&bs))
            }
        }
        None => {
            // the derived view: the program is the erasure of the constant's own value,
            // which K holds and the source does not write
            t.unread.push((k.clone(), Unread("view", "(realize NAME (view)): the program is not written".into())));
            match t.facts.get(&k) {
                Some(Fact::Def { binders: Some(bs), .. }) => EInfo::Fn(bs.iter().filter(|b| b.role == Role::Keep).count()),
                _ => return,
            }
        }
    };
    t.register(&k, &sc.tag, info);
}

// -----------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------

/// A type name's identity (classify.shard type_ident).
fn type_ident(cx: &Cx, n: &str) -> R<Name> {
    if builtin_type(n) {
        return Ok(n.to_string());
    }
    match cx.t.k_resolve(cx.sc, n, true) {
        Res::Name(k) => return Ok(k),
        Res::Ambiguous(a, b) => return unread("ambiguous_type", format!("{n}: {a} and {b}")),
        Res::None => {}
    }
    let hs = cx.t.e_types(cx.sc, n);
    match hs.as_slice() {
        [(id, _)] => Ok(id.clone()),
        [(a, _), (b, _), ..] => unread("ambiguous_type", format!("{n}: {a} and {b}")),
        [] => unread("unknown_type", n),
    }
}

/// A name that is a definition standing for a type: the E type its value is.
fn alias_etype(cx: &Cx, id: Name, depth: usize) -> ETy {
    if depth < 16 {
        if let Some(Native { kind: NativeKind::Def, form: Some((sc, form)), .. }) = cx.t.natives.get(&id) {
            let items = list_of(form);
            if items.len() == 5 && list_of(items[2]).is_empty() {
                if let Ok(t) = read_ety_at(&Cx { t: cx.t, sc }, &[], items[4], depth + 1) {
                    return t;
                }
            }
        }
    }
    ETy::Con(id, vec![])
}

pub fn read_ety(cx: &Cx, tps: &[String], s: &Value) -> R<ETy> {
    read_ety_at(cx, tps, s, 0)
}

fn read_ety_at(cx: &Cx, tps: &[String], s: &Value, depth: usize) -> R<ETy> {
    if let Some(x) = s.as_symbol() {
        if let Some(i) = tps.iter().position(|p| p == x) {
            return Ok(ETy::Var(i));
        }
        return Ok(alias_etype(cx, type_ident(cx, x)?, depth));
    }
    let items = list_of(s);
    let Some(x) = items.first().and_then(|h| h.as_symbol()) else {
        return unread("bad_type", format!("{s}"));
    };
    if x == "->" {
        return unread("function_value", "(-> …): a function-typed binder makes a template");
    }
    let (n, ex) = (strip_at(x), x.starts_with('@'));
    let id = type_ident(cx, n)?;
    // (Decidable P): nothing of the proposition is in E — the type is its two cells
    if id == "Decidable" {
        return Ok(ETy::Con(id, vec![]));
    }
    // a type's written arguments that are types: its parameters' roles select them
    let flags = match cx.t.facts.get(&id) {
        Some(Fact::Ind { params, .. }) => params.clone(),
        _ => None,
    };
    let targs: Vec<&Value> = match flags {
        Some(flags) => written_data(&flags, ex, &items[1..], &id)?,
        None => items[1..].to_vec(),
    };
    let mut ts = Vec::new();
    for a in targs {
        ts.push(read_ety_at(cx, tps, a, depth)?);
    }
    Ok(ETy::Con(id, ts))
}

/// Is a binder's written type a proposition? Decided on its head.
fn prop_type(cx: &Cx, r: &Rd, s: &Value) -> bool {
    if let Some(x) = s.as_symbol() {
        return prop_head(cx, r, strip_at(x));
    }
    let items = list_of(s);
    let Some(h) = items.first().and_then(|h| h.as_symbol()) else { return false };
    if prop_spelling(h) {
        return true;
    }
    if h == "->" || h == "forall" {
        return items.last().is_some_and(|l| prop_type(cx, r, l));
    }
    prop_head(cx, r, strip_at(h))
}

fn prop_head(cx: &Cx, r: &Rd, h: &str) -> bool {
    match r.find(h) {
        Hit::Ghost(p) => p,
        Hit::Var(_) => false,
        Hit::None => match cx.t.k_name(cx.sc, h) {
            Some(k) => match (cx.t.facts.get(&k), cx.t.natives.get(&k)) {
                (_, Some(nv)) => nv.prop,
                (Some(f), None) => f.prop(),
                (None, None) => false,
            },
            None => false,
        },
    }
}

// -----------------------------------------------------------------------------
// The arguments written and the arguments that run
// -----------------------------------------------------------------------------

/// A call's runtime arguments among those written, by the head's binders:
/// a type and a proof are written and erased, an implicit binder is not
/// written — and one that is runtime data has no reading (LAW rule 24).
fn written_data<'a>(flags: &[Flag], ex: bool, args: &[&'a Value], id: &str) -> R<Vec<&'a Value>> {
    let mut out = Vec::new();
    let mut it = args.iter();
    for f in flags {
        if ex || f.explicit {
            let Some(a) = it.next() else {
                return unread("unsaturated", format!("{id} is given fewer arguments than its signature has binders"));
            };
            if f.role == Role::Keep {
                out.push(*a);
            }
        } else if f.role == Role::Keep {
            return unread("unwritten_argument", format!("{id}: a binder that is implicit and runtime data"));
        }
    }
    if it.next().is_some() {
        return unread("unsaturated", format!("{id} is given more arguments than its signature has binders"));
    }
    Ok(out)
}

#[derive(Clone, Copy)]
enum Kind {
    Ctor,
    Call,
    Ext,
    Prim,
}

// -----------------------------------------------------------------------------
// Terms
// -----------------------------------------------------------------------------

fn rd_type(cx: &Cx, r: &Rd, t: &Tm) -> Option<ETy> {
    c_type(cx.t, &r.tys(), t)
}

fn read_et(cx: &Cx, r: &Rd, s: &Value) -> R<Tm> {
    if let Some(k) = int_of(s)? {
        return Ok(Tm::Lit(num_lit(k)));
    }
    // "…": a String — its UTF-8 bytes as Init's List cells
    if let Some(bs) = s.as_str() {
        return Ok(Tm::Lit(Lit::Str(bs.as_bytes().to_vec())));
    }
    if let Some(x) = s.as_symbol() {
        return read_esym(cx, r, x);
    }
    let items = list_of(s);
    let Some(head) = items.first() else { return unread("empty_application", "()") };
    let Some(h) = head.as_symbol() else { return unread("bad_head", format!("{head}")) };
    let args = &items[1..];
    match h {
        "match" => read_ematch(cx, r, args),
        "let" => read_elet(cx, r, args),
        "if" => read_eif(cx, r, args),
        "dif" => read_edif(cx, r, args),
        "absurd" => {
            if cx.t.init_const(cx.sc, "absurd", "absurd") {
                Ok(Tm::Absurd)
            } else {
                unread("absurd_e_first", "a refuted row is Init's absurd")
            }
        }
        "proj" => read_eproj(cx, r, args),
        "quote" => unread("no_quote", "a symbol is written (sym \"x\"): there is no quote form"),
        "list" => {
            let (nl, cs) = list_in_scope(cx)?;
            let mut acc = Tm::Ctor(nl, vec![]);
            for a in args.iter().rev() {
                acc = Tm::Ctor(cs.clone(), vec![read_et(cx, r, a)?, acc]);
            }
            Ok(acc)
        }
        "fun" => unread("function_value", "a lambda in a runtime position"),
        _ => read_eapp(cx, r, h, args),
    }
}

fn read_all(cx: &Cx, r: &Rd, args: &[&Value]) -> R<Vec<Tm>> {
    args.iter().map(|a| read_et(cx, r, a)).collect()
}

/// The two-constructor type `List` resolves to: its nil, its cons.
fn list_in_scope(cx: &Cx) -> R<(Name, Name)> {
    let id = type_ident(cx, "List")?;
    match cx.t.einfo(&id) {
        Some(EInfo::Ind { ctors, .. }) if ctors.len() == 2 => Ok((ctors[0].clone(), ctors[1].clone())),
        _ => unread("no_list", "(list …) needs a two-constructor List in scope"),
    }
}

/// A bare name: bound, else a constructor without fields, else a fn of no
/// parameters as its call.
fn read_esym(cx: &Cx, r: &Rd, x: &str) -> R<Tm> {
    match r.find(x) {
        Hit::Var(i) => return Ok(Tm::BVar(i)),
        Hit::Ghost(_) => return unread("erased_in_runtime", x),
        Hit::None => {}
    }
    let hs = cx.t.e_values(cx.sc, x);
    match hs.as_slice() {
        [(id, EInfo::Ctor { nfields: 0, .. })] => Ok(Tm::Ctor(id.clone(), vec![])),
        [(id, EInfo::Ctor { .. })] => unread("unsaturated", id.clone()),
        [(id, EInfo::Fn(0))] | [(id, EInfo::Sig(0))] => Ok(Tm::Call(id.clone(), vec![])),
        [(id, _)] => unread("function_value", id.clone()),
        [] => {
            if prim_of(cx.sc, x).is_some() {
                unread("function_value", x)
            } else if cx.t.init_const(cx.sc, x, "Nat.zero") {
                Ok(Tm::Lit(Lit::Nat(0.into())))
            } else {
                unread("unbound_name", x)
            }
        }
        [(a, _), (b, _), ..] => unread("ambiguous_head", format!("{x}: {a} and {b}")),
    }
}

/// (HEAD E…): a constructor, a function or sig fn, an extern, a primitive —
/// saturated. `@HEAD` names every binder; `%` is `mod`'s other spelling.
fn read_eapp(cx: &Cx, r: &Rd, h0: &str, args: &[&Value]) -> R<Tm> {
    let (h, ex) = (strip_at(h0), h0.starts_with('@'));
    let h = if h == "%" { "mod" } else { h };
    match r.find(h) {
        Hit::Var(_) => return unread("bad_head", format!("{h} is a variable: there are no function values")),
        Hit::Ghost(_) => return unread("erased_in_runtime", h),
        Hit::None => {}
    }
    let hs = cx.t.e_values(cx.sc, h);
    match hs.as_slice() {
        [(id, EInfo::Ctor { nfields, .. })] => read_known(cx, r, id, *nfields, ex, args, Kind::Ctor),
        [(id, EInfo::Fn(k))] | [(id, EInfo::Sig(k))] => read_known(cx, r, id, *k, ex, args, Kind::Call),
        [(id, EInfo::Ext(k))] => read_known(cx, r, id, *k, ex, args, Kind::Ext),
        [(id, EInfo::Template)] => unread("template", format!("{id}: a call of a template takes an instance")),
        [(id, _)] => unread("bad_head", format!("{id} is a type")),
        [] => {
            if let Some((id, k)) = prim_of(cx.sc, h) {
                return read_prim_head(cx, r, &id, k, args);
            }
            if let Some(res) = init_row(cx, r, h, args) {
                return res;
            }
            if let Some(res) = k_head(cx, r, ex, h, args) {
                return res;
            }
            unread("unknown_head", h)
        }
        [(a, _), (b, _), ..] => unread("ambiguous_head", format!("{h}: {a} and {b}")),
    }
}

/// A head the E table knows: its runtime arguments among those written.
fn read_known(cx: &Cx, r: &Rd, id: &str, k: usize, ex: bool, args: &[&Value], kind: Kind) -> R<Tm> {
    let flags = match &r.own {
        Some((own, flags)) if own == id => Some(flags.clone()),
        _ => cx.t.k_flags(id),
    };
    match flags {
        Some(flags) => {
            let ss = written_data(&flags, ex, args, id)?;
            read_head_args(cx, r, id, ss.len(), &ss, kind)
        }
        None => {
            if ex {
                unread("bad_head", format!("{id}: @ names every binder of a constant of K"))
            } else {
                read_head_args(cx, r, id, k, args, kind)
            }
        }
    }
}

fn read_head_args(cx: &Cx, r: &Rd, id: &str, k: usize, args: &[&Value], kind: Kind) -> R<Tm> {
    if k != args.len() {
        return unread("unsaturated", format!("{id} takes {k} arguments, given {}", args.len()));
    }
    let ts = read_all(cx, r, args)?;
    let id = id.to_string();
    Ok(match kind {
        Kind::Ctor => Tm::Ctor(id, ts),
        Kind::Call => Tm::Call(id, ts),
        Kind::Ext => Tm::Ext(id, ts),
        Kind::Prim => Tm::Prim(id, ts),
    })
}

/// A primitive written by name: the table's entry; a decision of the
/// registry is a value here — Decidable's cell by its test.
fn read_prim_head(cx: &Cx, r: &Rd, id: &str, k: usize, args: &[&Value]) -> R<Tm> {
    match decision_test(id) {
        Some(p) => read_decision(cx, r, p, args),
        None => read_head_args(cx, r, realization_of(id).unwrap_or(id), k, args, Kind::Prim),
    }
}

fn read_decision(cx: &Cx, r: &Rd, p: &str, args: &[&Value]) -> R<Tm> {
    let test = read_head_args(cx, r, p, 2, args, Kind::Prim)?;
    Ok(Tm::If(Box::new(test), Box::new(dec_true()), Box::new(dec_false())))
}

/// A head no table knows: a constant of Init with a fixed program (the registry's rows).
fn init_row(cx: &Cx, r: &Rd, h: &str, args: &[&Value]) -> Option<R<Tm>> {
    if matches!(h, "and" | "or" | "not") {
        return Some(read_bool_row(cx, r, h, args));
    }
    let k = cx.t.k_name(cx.sc, h)?;
    match k.as_str() {
        "Bool.and" => return Some(read_bool_row(cx, r, "and", args)),
        "Bool.or" => return Some(read_bool_row(cx, r, "or", args)),
        "Bool.not" => return Some(read_bool_row(cx, r, "not", args)),
        "Nat.succ" => {
            return Some(match read_all(cx, r, args) {
                Ok(ts) if ts.len() == 1 => {
                    let mut ts = ts;
                    ts.push(Tm::Lit(Lit::Nat(1.into())));
                    Ok(Tm::Prim("Nat.add".into(), ts))
                }
                Ok(_) => unread("unsaturated", "Nat.succ takes 1 argument"),
                Err(u) => Err(u),
            });
        }
        _ => {}
    }
    if let Some(p) = realization_of(&k) {
        return Some(if cmp_prim(p) { read_decision(cx, r, p, args) } else { read_head_args(cx, r, p, 2, args, Kind::Prim) });
    }
    if realization_type(&k).is_some() {
        return Some(read_head_args(cx, r, &k, 1, args, Kind::Prim));
    }
    None
}

fn read_bool_row(cx: &Cx, r: &Rd, op: &str, args: &[&Value]) -> R<Tm> {
    let mut ts = read_all(cx, r, args)?;
    match (op, ts.len()) {
        ("not", 1) => Ok(Tm::If(Box::new(ts.remove(0)), Box::new(bool_false()), Box::new(bool_true()))),
        ("and", 2) => {
            let b = ts.remove(1);
            Ok(Tm::If(Box::new(ts.remove(0)), Box::new(b), Box::new(bool_false())))
        }
        ("or", 2) => {
            let b = ts.remove(1);
            Ok(Tm::If(Box::new(ts.remove(0)), Box::new(bool_true()), Box::new(b)))
        }
        _ => unread("unsaturated", format!("{op}: the wrong number of arguments")),
    }
}

/// A head that is a constant of K and no entry of the E table: `dite` and
/// `ite` written with their decision, a refuted row, Bool's decidable
/// equality, a row's constructor, a projection function.
fn k_head(cx: &Cx, r: &Rd, ex: bool, n: &str, args: &[&Value]) -> Option<R<Tm>> {
    let k = cx.t.k_name(cx.sc, n)?;
    match k.as_str() {
        "dite" => return Some(read_ite(cx, r, ex, true, args)),
        "ite" => return Some(read_ite(cx, r, ex, false, args)),
        "absurd" | "False.elim" => return Some(Ok(Tm::Absurd)),
        "instDecidableEqBool" => {
            return Some(
                read_bool_eq(cx, r, args).map(|test| Tm::If(Box::new(test), Box::new(dec_true()), Box::new(dec_false()))),
            );
        }
        _ => {}
    }
    if let Some(nv) = cx.t.natives.get(&k) {
        return match nv.kind {
            NativeKind::Structure | NativeKind::Inductive => None,
            _ => Some(unread("no_realization", k)),
        };
    }
    match cx.t.facts.get(&k)? {
        Fact::Ctor { ind, .. } => {
            if rep_row(ind) {
                Some(read_row_ctor(cx, r, ex, &k, args))
            } else {
                None
            }
        }
        Fact::Def { proj: Some(p), .. } => Some(read_proj_fn(cx, r, ex, &k, p, args)),
        Fact::Def { proj: None, .. } | Fact::Other { .. } => Some(unread("no_realization", k)),
        Fact::Ind { .. } => None,
    }
}

/// A row's constructor: its one runtime argument.
fn read_row_ctor(cx: &Cx, r: &Rd, ex: bool, k: &str, args: &[&Value]) -> R<Tm> {
    let Some(flags) = cx.t.k_flags(k) else {
        return unread("no_realization", format!("{k}: the registry's row does not describe this structure"));
    };
    let ss = written_data(&flags, ex, args, k)?;
    match ss.as_slice() {
        [a] => read_et(cx, r, a),
        _ => unread("no_realization", format!("{k}: a row's constructor has exactly one runtime field")),
    }
}

/// A projection function applied to its structure exactly.
fn read_proj_fn(cx: &Cx, r: &Rd, ex: bool, k: &str, p: &super::facts::Proj, args: &[&Value]) -> R<Tm> {
    let want = if ex { p.lams.len() } else { p.lams.iter().filter(|b| **b).count() };
    if want < args.len() {
        return unread("function_value", format!("{k}: a projection applied past its structure"));
    }
    if want != args.len() || args.is_empty() {
        return unread("unsaturated", format!("{k}: a projection is applied to its structure"));
    }
    read_proj(cx, r, &p.structure, p.index, args[args.len() - 1])
}

/// Field i of the structure sn, of the subject x.
fn read_proj(cx: &Cx, r: &Rd, sn: &str, i: usize, x: &Value) -> R<Tm> {
    let Some(c) = cx.t.struct_ctor(sn) else { return unread("no_realization", format!("{sn} is not a structure")) };
    let Some(flags) = cx.t.ctor_data_flags(&c) else { return unread("no_realization", c) };
    if i >= flags.len() || !flags[i] {
        return unread("erased_in_runtime", format!("{sn}: a projection of an erased field"));
    }
    let j = flags[..i].iter().filter(|b| **b).count();
    let nrt = flags.iter().filter(|b| **b).count();
    let x1 = read_et(cx, r, x)?;
    if rep_row(sn) {
        if nrt != 1 {
            return unread("no_realization", format!("{sn}: the registry's row does not describe this structure"));
        }
        // kept by name while the body is typed
        return Ok(match row_field_proj(sn) {
            Some(p) => Tm::Prim(p.into(), vec![x1]),
            None => x1,
        });
    }
    match cx.t.einfo(&c) {
        Some(EInfo::Ctor { .. }) => {
            Ok(Tm::Match(Box::new(x1), vec![(Pat::Ctor(c, vec![Pat::Var; nrt]), Tm::BVar(nrt - 1 - j))]))
        }
        _ => unread("no_realization", format!("{sn} is not an E type")),
    }
}

/// (proj S i E): field i of the structure S.
fn read_eproj(cx: &Cx, r: &Rd, args: &[&Value]) -> R<Tm> {
    match args {
        [s, i, x] if s.as_symbol().is_some() && i.as_u64().is_some() => {
            let sx = s.as_symbol().unwrap();
            match cx.t.k_resolve(cx.sc, sx, true) {
                Res::Name(sn) => read_proj(cx, r, &sn, i.as_u64().unwrap() as usize, x),
                _ => unread("unknown_head", format!("{sx} is not a structure of K")),
            }
        }
        _ => unread("bad_shape", "(proj STRUCTURE INDEX E)"),
    }
}

/// (@dite T C DEC (fun ((h C)) A) (fun ((h (Not C))) B)) and (@ite T C DEC A B).
fn read_ite(cx: &Cx, r: &Rd, ex: bool, dep: bool, args: &[&Value]) -> R<Tm> {
    if !ex {
        return unread("unwritten_argument", "ite and dite take their decision as an instance argument");
    }
    match args {
        [_, _, d, a, b] => {
            let c = read_cond(cx, r, d)?;
            let ta = read_branch(cx, r, dep, a)?;
            let tb = read_branch(cx, r, dep, b)?;
            Ok(Tm::If(Box::new(c), Box::new(ta), Box::new(tb)))
        }
        _ => unread("unsaturated", "@ite and @dite take five arguments"),
    }
}

fn read_branch(cx: &Cx, r: &Rd, dep: bool, s: &Value) -> R<Tm> {
    if !dep {
        return read_et(cx, r, s);
    }
    let items = list_of(s);
    if items.len() == 3 && items[0].as_symbol() == Some("fun") {
        let bs = list_of(items[1]);
        if bs.len() == 1 {
            if let Some(x) = list_of(bs[0]).first().and_then(|v| v.as_symbol()) {
                return read_et(cx, &r.ghost(x), items[2]);
            }
        }
    }
    unread("function_value", "a dite's branch is (fun ((h P)) E)")
}

/// A decision where an `if` reads it: a decision of the registry written by
/// name is its bare test, Bool's decidable equality the test over its
/// operands; anything else is its value, whose cell the `if` reads.
fn read_cond(cx: &Cx, r: &Rd, s: &Value) -> R<Tm> {
    let items = list_of(s);
    if let Some(h0) = items.first().and_then(|h| h.as_symbol()) {
        let n = strip_at(h0);
        if matches!(r.find(n), Hit::None) && cx.t.e_values(cx.sc, n).is_empty() {
            if let Some(k) = cx.t.k_name(cx.sc, n) {
                if k == "instDecidableEqBool" {
                    return read_bool_eq(cx, r, &items[1..]);
                }
                if let Some(p) = decision_test(&k) {
                    return read_head_args(cx, r, p, 2, &items[1..], Kind::Prim);
                }
            }
        }
    }
    read_et(cx, r, s)
}

fn read_bool_eq(cx: &Cx, r: &Rd, args: &[&Value]) -> R<Tm> {
    let mut ts = read_all(cx, r, args)?;
    if ts.len() != 2 {
        return unread("unsaturated", "instDecidableEqBool takes 2 arguments");
    }
    let b = ts.remove(1);
    Ok(bool_eq_cond(ts.remove(0), b))
}

/// (dif h C A B): the `if` on C, its hypothesis a proof in both branches.
fn read_edif(cx: &Cx, r: &Rd, args: &[&Value]) -> R<Tm> {
    match args {
        [hn, c, a, b] if hn.as_symbol().is_some() => {
            let r1 = r.ghost(hn.as_symbol().unwrap());
            let tc = read_et(cx, r, c)?;
            let ta = read_et(cx, &r1, a)?;
            let tb = read_et(cx, &r1, b)?;
            Ok(Tm::If(Box::new(tc), Box::new(ta), Box::new(tb)))
        }
        _ => unread("bad_shape", "(dif h C A B)"),
    }
}

fn read_eif(cx: &Cx, r: &Rd, args: &[&Value]) -> R<Tm> {
    match args {
        [c, a, b] => {
            let tc = read_cond(cx, r, c)?;
            let ta = read_et(cx, r, a)?;
            let tb = read_et(cx, r, b)?;
            Ok(Tm::If(Box::new(tc), Box::new(ta), Box::new(tb)))
        }
        _ => unread("bad_shape", "(if E E E)"),
    }
}

/// (let ((x E)…) E): sequential. A binding may state its type, (x TYPE E):
/// at a proposition it is a proof and vanishes; and a let of several
/// bindings is a let in a let, one binding each.
fn read_elet(cx: &Cx, r: &Rd, args: &[&Value]) -> R<Tm> {
    let [binds, body] = args else { return unread("bad_shape", "(let ((x E)…) E)") };
    let mut r1 = r.clone();
    let mut rhss = Vec::new();
    for b in list_of(binds) {
        let b = list_of(b);
        let (x, e) = match b.as_slice() {
            [x, e] if x.as_symbol().is_some() => (x.as_symbol().unwrap(), *e),
            [x, ty, e] if x.as_symbol().is_some() => {
                if prop_type(cx, &r1, ty) {
                    r1 = r1.ghost(x.as_symbol().unwrap());
                    continue;
                }
                (x.as_symbol().unwrap(), *e)
            }
            _ => return unread("bad_shape", "a binding is (NAME E)"),
        };
        let t = read_et(cx, &r1, e)?;
        r1 = r1.bind(x, rd_type(cx, &r1, &t));
        rhss.push(t);
    }
    let b = read_et(cx, &r1, body)?;
    if rhss.is_empty() {
        return Ok(b);
    }
    let mut acc = b;
    for v in rhss.into_iter().rev() {
        acc = Tm::Let(vec![v], Box::new(acc));
    }
    Ok(acc)
}

fn has_numeral_row(arms: &[&Value]) -> bool {
    arms.iter().any(|a| list_of(a).first().is_some_and(|p| p.is_number() || head_sym(p) == Some("__big")))
}

/// (match E (PAT E)…): each arm's variables bound for its body, typed from
/// the scrutinee's static type.
fn read_ematch(cx: &Cx, r: &Rd, args: &[&Value]) -> R<Tm> {
    let Some((s, arms)) = args.split_first() else { return unread("bad_shape", "(match E (PAT E)…)") };
    let t = read_et(cx, r, s)?;
    if has_numeral_row(arms) {
        return read_ematch_lit(cx, r, t, arms);
    }
    let st = rd_type(cx, r, &t);
    let mut out = Vec::new();
    for a in arms {
        let a = list_of(a);
        let [p, b] = a.as_slice() else { return unread("bad_shape", "an arm is (PAT E)") };
        let (pat, names, ghosts) = read_epat(cx, r, &st, p)?;
        let mut r1 = r.clone();
        for g in ghosts {
            r1 = r1.ghost(&g);
        }
        let tys = pat_types(cx.t, &st, &pat);
        for (i, n) in names.iter().enumerate() {
            r1 = r1.bind(n, tys.get(i).cloned().flatten());
        }
        out.push((pat, read_et(cx, &r1, b)?));
    }
    Ok(Tm::Match(Box::new(t), out))
}

fn numeral_type(t: &Option<ETy>) -> bool {
    match t {
        None => true,
        Some(ETy::Con(n, args)) => args.is_empty() && (n == "Nat" || n == "Int"),
        Some(_) => false,
    }
}

/// Numeral rows: the scrutinee once, a test per
/// row in row order, the last row `_` or a name for the scrutinee.
fn read_ematch_lit(cx: &Cx, r: &Rd, t: Tm, arms: &[&Value]) -> R<Tm> {
    let st = rd_type(cx, r, &t);
    if !numeral_type(&st) {
        return unread("pattern_type", "a numeral row matches a value read at Nat or Int");
    }
    if matches!(t, Tm::BVar(_)) {
        return read_lit_rows(cx, r, &t, arms);
    }
    let r1 = r.bind("match scrutinee", st);
    let chain = read_lit_rows(cx, &r1, &Tm::BVar(0), arms)?;
    Ok(Tm::Let(vec![t], Box::new(chain)))
}

fn read_lit_rows(cx: &Cx, r: &Rd, x: &Tm, arms: &[&Value]) -> R<Tm> {
    let Some((arm, rest)) = arms.split_first() else {
        return unread("match_not_exhaustive", "numeral rows cover no type");
    };
    let a = list_of(arm);
    let [p, body] = a.as_slice() else { return unread("literal_rows", "a literal match's row is (PAT E)") };
    if let Some(k) = int_of(p)? {
        let b = read_et(cx, r, body)?;
        let e = read_lit_rows(cx, r, x, rest)?;
        let test = Tm::Prim("=".into(), vec![x.clone(), Tm::Lit(num_lit(k))]);
        return Ok(Tm::If(Box::new(test), Box::new(b), Box::new(e)));
    }
    match p.as_symbol() {
        Some(v) if rest.is_empty() => {
            if v == "_" {
                read_et(cx, r, body)
            } else {
                let b = read_et(cx, &r.bind(v, rd_type(cx, r, x)), body)?;
                Ok(Tm::Let(vec![x.clone()], Box::new(b)))
            }
        }
        _ => unread("literal_rows", "a literal match is numeral rows closed by one last row"),
    }
}

// -----------------------------------------------------------------------------
// Patterns: the variables in capture order (leftmost first)
// -----------------------------------------------------------------------------

type PRes = (Pat, Vec<String>, Vec<String>);

/// The constructor a name is under the scrutinee's inductive, with its
/// written fields' number.
fn ctor_at(cx: &Cx, st: &Option<ETy>, n: &str) -> Option<(Name, usize)> {
    let Some(ETy::Con(ind, _)) = st else { return None };
    let Some(EInfo::Ind { ctors, .. }) = cx.t.einfo(ind) else { return None };
    let id = append(ind, n);
    if ctors.contains(&id) {
        let k = pat_fields(cx, &id).len();
        Some((id, k))
    } else {
        None
    }
}

/// A constructor's fields as a pattern writes them.
fn pat_fields(cx: &Cx, id: &str) -> Vec<bool> {
    match cx.t.ctor_data_flags(id) {
        Some(f) => f,
        None => match cx.t.einfo(id) {
            Some(EInfo::Ctor { nfields, .. }) => vec![true; *nfields],
            _ => Vec::new(),
        },
    }
}

fn read_epat(cx: &Cx, r: &Rd, st: &Option<ETy>, s: &Value) -> R<PRes> {
    if let Some(k) = int_of(s)? {
        return Ok((Pat::Lit(num_lit(k)), vec![], vec![]));
    }
    if s.as_str().is_some() {
        return unread("pattern_literal", "a string is not a pattern");
    }
    if let Some(x) = s.as_symbol() {
        let cs = cx.t.e_ctors(cx.sc, x);
        return match cs.as_slice() {
            [] => match ctor_at(cx, st, x) {
                // under the scrutinee's inductive: its constructor where that has no
                // fields, else a variable
                Some((id, 0)) => Ok((Pat::Ctor(id, vec![]), vec![], vec![])),
                Some(_) => Ok((Pat::Var, vec![x.to_string()], vec![])),
                None => {
                    if cx.t.e_values(cx.sc, x).is_empty() {
                        Ok((Pat::Var, vec![x.to_string()], vec![]))
                    } else {
                        unread("pattern_name", format!("{x} is a function: a pattern variable may not shadow one"))
                    }
                }
            },
            [(id, EInfo::Ctor { nfields: 0, .. })] => Ok((Pat::Ctor(id.clone(), vec![]), vec![], vec![])),
            [(id, _)] => unread("pattern_arity", id.clone()),
            [(a, _), (b, _), ..] => unread("ambiguous_head", format!("{x}: {a} and {b}")),
        };
    }
    let items = list_of(s);
    let Some(h) = items.first().and_then(|h| h.as_symbol()) else {
        return unread("bad_pattern", "a pattern is _, NAME, NUMERAL or (CTOR PAT…)");
    };
    let subs = &items[1..];
    if h == "quote" {
        return unread("no_quote", "a symbol is written (sym \"x\"): there is no quote form");
    }
    let cs = cx.t.e_ctors(cx.sc, h);
    match cs.as_slice() {
        [] => match ctor_at(cx, st, h) {
            Some((id, _)) => read_ctor_pat(cx, r, st, &id, subs),
            None => {
                // a row's constructor: its pattern is its one runtime field's pattern
                if let Some(k) = cx.t.k_name(cx.sc, h) {
                    if let Some(Fact::Ctor { ind, .. }) = cx.t.facts.get(&k) {
                        if rep_row(ind) {
                            return read_row_pat(cx, r, &k, subs);
                        }
                    }
                }
                unread("unknown_head", format!("{h} is not a constructor"))
            }
        },
        [(id, _)] => read_ctor_pat(cx, r, st, id, subs),
        [(a, _), (b, _), ..] => unread("ambiguous_head", format!("{h}: {a} and {b}")),
    }
}

fn read_row_pat(cx: &Cx, r: &Rd, k: &str, subs: &[&Value]) -> R<PRes> {
    let Some(flags) = cx.t.ctor_data_flags(k) else { return unread("bad_pattern", k) };
    if flags.len() != subs.len() {
        return unread("pattern_arity", k);
    }
    let (mut ps, names, ghosts) = read_epats(cx, r, &[], &flags, subs)?;
    if ps.len() != 1 {
        return unread("bad_pattern", format!("{k}: a row's constructor has exactly one runtime field"));
    }
    Ok((ps.remove(0), names, ghosts))
}

fn read_ctor_pat(cx: &Cx, r: &Rd, st: &Option<ETy>, id: &str, subs: &[&Value]) -> R<PRes> {
    let flags = pat_fields(cx, id);
    if flags.len() != subs.len() {
        return unread("pattern_arity", format!("{id} has {} fields, the pattern {}", flags.len(), subs.len()));
    }
    let sts: Vec<Option<ETy>> = match settle::ctor_field_types(cx.t, st, id) {
        Some(fts) => fts.into_iter().map(Some).collect(),
        None => Vec::new(),
    };
    let (ps, names, ghosts) = read_epats(cx, r, &sts, &flags, subs)?;
    Ok((Pat::Ctor(id.to_string(), ps), names, ghosts))
}

/// The fields' patterns in order: a runtime field's at its type, an erased
/// field's a name bound as a proof.
fn read_epats(cx: &Cx, r: &Rd, sts: &[Option<ETy>], flags: &[bool], subs: &[&Value]) -> R<(Vec<Pat>, Vec<String>, Vec<String>)> {
    let (mut ps, mut names, mut ghosts) = (Vec::new(), Vec::new(), Vec::new());
    let mut rt = 0;
    for (s, flag) in subs.iter().zip(flags) {
        if !*flag {
            match s.as_symbol() {
                Some(x) => ghosts.push(x.to_string()),
                None => return unread("bad_pattern", "an erased field's pattern is a name"),
            }
            continue;
        }
        let st = sts.get(rt).cloned().flatten();
        rt += 1;
        let (p, ns, gs) = read_epat(cx, r, &st, s)?;
        ps.push(p);
        names.extend(ns);
        ghosts.extend(gs);
    }
    Ok((ps, names, ghosts))
}

// -----------------------------------------------------------------------------
// The declarations
// -----------------------------------------------------------------------------

struct Sig {
    names: Vec<String>,
    types: Vec<ETy>,
    ntp: usize,
    bound: Vec<Bd>,
    flags: Vec<Flag>,
    ret: ETy,
}

/// ((x TYPE)…) RET: the runtime binders' names and E types in order;
/// `(T Type)` declares a type parameter; a binder
/// whose type is a proposition is a proof, written and no parameter of the
/// program. With them each written binder's flags, which a call is read by.
fn read_sig(cx: &Cx, bs: &[&Value], ret: &Value) -> R<Sig> {
    let mut sig = Sig { names: vec![], types: vec![], ntp: 0, bound: vec![], flags: vec![], ret: ETy::Var(0) };
    let mut tps: Vec<String> = Vec::new();
    for b in bs {
        let b = list_of(b);
        let (x, ty) = match b.as_slice() {
            [x, ty] if x.as_symbol().is_some() => (x.as_symbol().unwrap(), *ty),
            _ => return unread("bad_binder", "a binder is (x TYPE)"),
        };
        if is_type_sym(ty) {
            tps.push(x.to_string());
            sig.bound.push(Bd::Ghost(x.to_string(), false));
            sig.flags.push(Flag { explicit: false, role: Role::Drop });
            continue;
        }
        let pred = is_pred_sort(ty);
        if pred || prop_type(cx, &Rd { bound: sig.bound.clone(), own: None }, ty) {
            sig.bound.push(Bd::Ghost(x.to_string(), pred));
            sig.flags.push(Flag { explicit: true, role: Role::Proof });
            continue;
        }
        if head_sym(ty) == Some("->") {
            return unread("template", "a function-typed binder: the fn is a template");
        }
        let t = read_ety(cx, &tps, ty)?;
        if let ETy::Con(k, _) = &t {
            if matches!(cx.t.natives.get(k), Some(nv) if nv.kind == NativeKind::Structure) {
                return unread("template", format!("a binder at the structure {k}: a package makes the fn a template"));
            }
        }
        sig.names.push(x.to_string());
        sig.bound.push(Bd::Var(x.to_string(), Some(t.clone())));
        sig.types.push(t);
        sig.flags.push(Flag { explicit: true, role: Role::Keep });
    }
    sig.ret = read_ety(cx, &tps, ret)?;
    sig.ntp = tps.len();
    Ok(sig)
}

/// What a form declares a program for: the identity, its binders, its
/// result, the forms after the result.
fn fn_parts<'a>(t: &Tables, sc: &Scope, items: &[&'a Value]) -> Option<(Name, Vec<&'a Value>, &'a Value, Vec<&'a Value>)> {
    match items.first()?.as_symbol()? {
        "fn" => {
            let n = items.get(1)?.as_symbol()?;
            Some((sc.qualify(n), list_of(items.get(2)?), items.get(3)?, items[4..].to_vec()))
        }
        "realize" => {
            let bs = realize_supplied(items)?;
            let k = realize_target(t, sc, items)?;
            // the equations it cites are K's
            let mut rest = items[4..].to_vec();
            if rest.last().is_some_and(|l| head_sym(l) == Some("equations")) {
                rest.pop();
            }
            Some((k, bs, items[3], rest))
        }
        _ => None,
    }
}

/// Every signature of the file, before any body.
pub fn signature(t: &mut Tables, sc: &Rc<Scope>, f: &Value) {
    let items = list_of(f);
    let Some(h) = items.first().and_then(|h| h.as_symbol()) else { return };
    match h {
        "fn" | "realize" => {
            let Some((id, bs, ret, _)) = fn_parts(t, sc, &items) else { return };
            if matches!(t.einfo(&id), Some(EInfo::Template) | None) {
                return;
            }
            match read_sig(&Cx { t, sc }, &bs, ret) {
                Ok(sig) => {
                    t.flags.insert(id.clone(), sig.flags);
                    t.add_decl(Decl::Fn { name: id, ntp: sig.ntp, params: sig.types, ret: sig.ret, body: None });
                }
                Err(u) => {
                    if u.0 == "template" {
                        let tag = sc.tag.clone();
                        t.register(&id, &tag, EInfo::Template);
                    } else {
                        t.unread.push((id, u));
                    }
                }
            }
        }
        "extern" => {
            let (Some(n), Some(bs), Some(ret)) = (items.get(1).and_then(|v| v.as_symbol()), items.get(2), items.get(3)) else { return };
            let id = sc.qualify(n);
            match read_sig(&Cx { t, sc }, &list_of(bs), ret) {
                Ok(sig) => t.add_decl(Decl::Extern { name: id, ntp: sig.ntp, params: sig.types, ret: sig.ret }),
                Err(u) => t.unread.push((id, u)),
            }
        }
        "sig" if items.get(1).and_then(|v| v.as_symbol()) == Some("fn") => {
            let (Some(n), Some(bs), Some(ret)) = (items.get(2).and_then(|v| v.as_symbol()), items.get(3), items.get(4)) else { return };
            let id = sc.qualify(n);
            // the implementation's fn stands under the sig fn's identity where it is loaded
            if matches!(t.find_decl(&id), Some(Decl::Fn { .. })) {
                return;
            }
            match read_sig(&Cx { t, sc }, &list_of(bs), ret) {
                Ok(sig) => t.add_decl(Decl::Sig { name: id, ntp: sig.ntp, params: sig.types, ret: sig.ret }),
                Err(u) => t.unread.push((id, u)),
            }
        }
        _ => {}
    }
}

/// The forms after RET: BODY, or (measure M) BODY — the measure is the definer's.
fn body_form<'a>(rest: &[&'a Value]) -> Option<&'a Value> {
    match rest {
        [b] => Some(b),
        [m, b] if head_sym(m) == Some("measure") => Some(b),
        _ => None,
    }
}

/// A fn's body, or a supplied realize's: read under its binders, its
/// operators settled by their operands' types, its rows written out.
pub fn body(t: &mut Tables, sc: &Rc<Scope>, f: &Value) {
    let items = list_of(f);
    if !matches!(items.first().and_then(|h| h.as_symbol()), Some("fn" | "realize")) {
        return;
    }
    let Some((id, bs, ret, rest)) = fn_parts(t, sc, &items) else { return };
    if !matches!(t.find_decl(&id), Some(Decl::Fn { body: None, .. })) {
        return;
    }
    let res = (|| {
        let cx = Cx { t, sc };
        let sig = read_sig(&cx, &bs, ret)?;
        let Some(b) = body_form(&rest) else { return unread("bad_shape", "NAME ((x TYPE)…) RET (measure M)? BODY") };
        let r = Rd { bound: sig.bound, own: Some((id.clone(), sig.flags)) };
        let t0 = read_et(&cx, &r, b)?;
        let tys: Vec<Option<ETy>> = sig.types.iter().cloned().map(Some).collect();
        let t1 = settle::settle_ops(t, &tys, &Some(sig.ret.clone()), t0)?;
        Ok(settle::expand_rows(t1))
    })();
    match res {
        Ok(tm) => {
            if let Some(Decl::Fn { body, .. }) = t.decls.get_mut(&id) {
                *body = Some(tm);
            }
        }
        Err(u) => t.unread.push((id, u)),
    }
}
