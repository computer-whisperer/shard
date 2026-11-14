//! The program AST: what the reader builds and the evaluator runs.
//! `v3/kernel/prog.shard` has the same shape with every name resolved to an
//! identity; `eval dump` and `load.shard --dump` print the two as one text
//! (`dump.rs`).
//!
//! Locally nameless: `BVar(idx)` is a bound variable as a 0-based de Bruijn
//! index, 0 = innermost; `FVar(name)` is a bare identifier that names
//! neither a binding nor a constructor, and evaluates to itself.

pub type Symbol = String;

/// Integer values are arbitrary-precision. A machine integer wraps silently
/// in a release build, and the checker's arithmetic runs through these
/// primitives: a wrapped result there is a wrong verdict.
pub type IntLit = num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    FVar(Symbol),
    BVar(u32),
    Ctor(Symbol, Vec<Expr>),
    Call(Symbol, Vec<Expr>),
    Match(Box<Expr>, Vec<Arm>),
    Let(Vec<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    IntLit(IntLit),
    SymLit(Symbol),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arm {
    pub pat: Pat,
    pub body: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pat {
    PVar,
    PCtor(Symbol, Vec<Pat>),
    PInt(IntLit),
    PSym(Symbol),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    TCon(Symbol, Vec<Type>),
    TVar(Symbol),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CtorDef {
    pub name: Symbol,
    pub fields: Vec<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeDef {
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub ctors: Vec<CtorDef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FnDef {
    pub name: Symbol,
    pub params: Vec<Type>, // length = arity
    pub ret: Type,
    pub body: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternDef {
    pub name: Symbol,
    pub params: Vec<Type>,
    pub ret: Type,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Module {
    pub types: Vec<TypeDef>,
    pub fns: Vec<FnDef>,
    pub externs: Vec<ExternDef>,
}
