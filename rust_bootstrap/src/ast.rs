//! Rust mirror of the narrow object-language ADTs. Kept in lockstep
//! with kernel/{term,module,proof}.shard.
//!
//! Locally-nameless: `FVar(name)` is a free variable by name;
//! `BVar(idx)` is a bound variable as a 0-based de Bruijn index,
//! 0 = innermost.
//!
//! MVP scope note: every variant is defined so the AST is complete,
//! but only the subset used by the first slice is exercised by the
//! evaluator. The unused variants are placeholders for upcoming
//! slices and will be wired through eval/load as we grow.

pub type Symbol = String;

/// Integer values. MVP uses `i64`; we'll swap to arbitrary-precision
/// (`num_bigint::BigInt`) before the narrow code starts relying on
/// `mod`/bitwise semantics. Tracked in REVISIT (Native Int + Symbol).
pub type IntLit = i64;

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

impl Module {
    pub fn lookup_fn(&self, name: &str) -> Option<&FnDef> {
        self.fns.iter().find(|f| f.name == name)
    }
}
