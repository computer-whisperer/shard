//! Builders for narrow-language *runtime values*.
//!
//! Every function here returns an `ast::Expr` that represents a value
//! of some narrow type (Expr, Type, FnDef, Module, Sequent, …). They
//! are the obvious replacement for writing nested `(Ctor (quote Foo)
//! (Cons … (Cons … (Nil))))` by hand in sexp source.
//!
//! See `docs/LANGUAGE.md` for the language; see `kernel/term.sexp`
//! and `kernel/proof.sexp` for the narrow ADT declarations these
//! builders mirror.
//!
//! Naming: a builder is named after the *narrow* ctor it constructs.
//! `intlit(5)` builds the narrow Expr value `(IntLit 5)`, which in
//! Rust storage is `Ctor("IntLit", [IntLit(5)])`. The inner Rust
//! `IntLit` is the literal Int field; the outer `Ctor("IntLit", …)`
//! is the narrow IntLit constructor value. The same level-distinction
//! shows up throughout — see the comment at the top of
//! `kernel/term.sexp`'s Expr declaration if it's confusing.
//!
//! Only used from tests. Marked `#[cfg(test)]` at the parent's
//! declaration in `lib.rs`.

// Many builders won't be used by the very first test that touches
// nval; they're a palette to be filled out as proof tests grow. The
// alternative — add them lazily as needed — fragments review.
#![allow(dead_code)]

use crate::ast::Expr;

fn ctor(name: &str, args: Vec<Expr>) -> Expr {
    Expr::Ctor(name.into(), args)
}

/// Build a narrow `(List X)` value from a Vec, threading Cons/Nil.
pub fn list(items: Vec<Expr>) -> Expr {
    let mut acc = ctor("Nil", vec![]);
    for it in items.into_iter().rev() {
        acc = ctor("Cons", vec![it, acc]);
    }
    acc
}

// -----------------------------------------------------------------
// Narrow Expr values (the language's Expr ADT).
// -----------------------------------------------------------------

pub fn fvar(name: &str) -> Expr   { ctor("FVar",   vec![Expr::SymLit(name.into())]) }
pub fn bvar(k: i64) -> Expr       { ctor("BVar",   vec![Expr::IntLit(k)]) }
pub fn intlit(n: i64) -> Expr     { ctor("IntLit", vec![Expr::IntLit(n)]) }
pub fn symlit(s: &str) -> Expr    { ctor("SymLit", vec![Expr::SymLit(s.into())]) }
pub fn ctor_app(name: &str, args: Vec<Expr>) -> Expr {
    ctor("Ctor", vec![Expr::SymLit(name.into()), list(args)])
}
pub fn call(name: &str, args: Vec<Expr>) -> Expr {
    ctor("Call", vec![Expr::SymLit(name.into()), list(args)])
}
pub fn if_expr(c: Expr, t: Expr, e: Expr) -> Expr {
    ctor("If", vec![c, t, e])
}

// -----------------------------------------------------------------
// Narrow Type values.
// -----------------------------------------------------------------

pub fn tcon(name: &str, args: Vec<Expr>) -> Expr {
    ctor("TCon", vec![Expr::SymLit(name.into()), list(args)])
}
pub fn ty_int() -> Expr { tcon("Int", vec![]) }
pub fn ty_sym() -> Expr { tcon("Symbol", vec![]) }

// -----------------------------------------------------------------
// Module pieces.
// -----------------------------------------------------------------

pub fn ctor_def(name: &str, field_types: Vec<Expr>) -> Expr {
    ctor("CtorDef", vec![Expr::SymLit(name.into()), list(field_types)])
}
pub fn type_def(name: &str, params: Vec<&str>, ctors: Vec<Expr>) -> Expr {
    let params_list = list(
        params.into_iter().map(|p| Expr::SymLit(p.into())).collect(),
    );
    ctor("TypeDef", vec![Expr::SymLit(name.into()), params_list, list(ctors)])
}
pub fn fn_def(name: &str, params: Vec<Expr>, ret: Expr, body: Expr) -> Expr {
    ctor("FnDef", vec![Expr::SymLit(name.into()), list(params), ret, body])
}
pub fn module(types: Vec<Expr>, fns: Vec<Expr>, externs: Vec<Expr>) -> Expr {
    ctor("Module", vec![list(types), list(fns), list(externs)])
}

// -----------------------------------------------------------------
// Equation / Param / Goal / Sequent.
// -----------------------------------------------------------------

pub fn equation(l: Expr, r: Expr) -> Expr  { ctor("Equation", vec![l, r]) }
pub fn param(name: &str, ty: Expr) -> Expr { ctor("Param", vec![Expr::SymLit(name.into()), ty]) }
pub fn goal(params: Vec<Expr>, premises: Vec<Expr>, eq: Expr) -> Expr {
    ctor("Goal", vec![list(params), list(premises), eq])
}
pub fn sequent(params: Vec<Expr>, hyps: Vec<Expr>, premises: Vec<Expr>, eq: Expr) -> Expr {
    ctor("Sequent", vec![list(params), list(hyps), list(premises), eq])
}

// -----------------------------------------------------------------
// Theory.
// -----------------------------------------------------------------

pub fn theory_empty() -> Expr { ctor("TheoryEmpty", vec![]) }
pub fn theory_cons(entry: Expr, rest: Expr) -> Expr {
    ctor("TheoryCons", vec![entry, rest])
}
pub fn proven(name: &str, g: Expr) -> Expr {
    ctor("Proven", vec![Expr::SymLit(name.into()), g])
}
pub fn axiom(name: &str, g: Expr) -> Expr {
    ctor("Axiom", vec![Expr::SymLit(name.into()), g])
}

// -----------------------------------------------------------------
// Proof tree / Step / Side.
// -----------------------------------------------------------------

pub fn refl() -> Expr { ctor("Refl", vec![]) }
pub fn steps(stps: Vec<Expr>, rest: Expr) -> Expr {
    ctor("Steps", vec![list(stps), rest])
}
pub fn unfold(name: &str, side: Expr) -> Expr {
    ctor("Unfold", vec![Expr::SymLit(name.into()), side])
}
pub fn reduce(side: Expr) -> Expr { ctor("Reduce", vec![side]) }
pub fn simp(side: Expr) -> Expr   { ctor("Simp",   vec![side]) }

pub fn side_lhs() -> Expr  { ctor("Lhs",  vec![]) }
pub fn side_rhs() -> Expr  { ctor("Rhs",  vec![]) }
pub fn side_both() -> Expr { ctor("Both", vec![]) }
