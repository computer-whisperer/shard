//! Native primitive operations. The narrow reducer treats unknown
//! `Call` names as stuck; the outer evaluator catches stuck calls
//! here and applies the native operation when args fit the expected
//! shape.
//!
//! Primitives that return a user `Bool` (`int_eq`, `sym_eq`, `lt`,
//! `le`) hardcode the constructor names `True` / `False`. This
//! couples the runtime to the stdlib's `(type Bool (False) (True))`
//! definition — see REVISIT.md, "Primitive comparisons return user
//! Bool". A future module-header directive will let the bool ctor
//! names be configurable.
//!
//! `gen_fresh` is the documented effectful primitive — REVISIT.md,
//! "Fresh-symbol generation as an effectful primitive". Implemented
//! with a global atomic counter; sequential and unique within a
//! process.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::ast::{Expr, Symbol};

/// Try to apply a primitive to fully-reduced args. Returns `Some(e)`
/// if `name` is a known primitive and the args fit the expected
/// shape; `None` otherwise (the call stays stuck, which the
/// evaluator surfaces as `UnknownCall`).
pub fn try_apply(name: &str, args: &[Expr]) -> Option<Expr> {
    use Expr::{IntLit, SymLit};

    match (name, args) {
        // Integer arithmetic. `mod` uses Euclidean remainder so the
        // result is always non-negative for a positive modulus,
        // matching the convention the modular-arithmetic library
        // wrappers will rely on.
        ("+",   [IntLit(a), IntLit(b)])                  => Some(IntLit(a + b)),
        ("-",   [IntLit(a), IntLit(b)])                  => Some(IntLit(a - b)),
        ("*",   [IntLit(a), IntLit(b)])                  => Some(IntLit(a * b)),
        ("/",   [IntLit(a), IntLit(b)]) if *b != 0       => Some(IntLit(a / b)),
        ("mod", [IntLit(a), IntLit(b)]) if *b != 0       => Some(IntLit(a.rem_euclid(*b))),

        // Bitwise on non-negative ints. Shifts by 64+ or negative
        // amounts are caller errors; we leave the call stuck so the
        // bug surfaces rather than wrapping silently.
        ("band", [IntLit(a), IntLit(b)])                              => Some(IntLit(a & b)),
        ("bor",  [IntLit(a), IntLit(b)])                              => Some(IntLit(a | b)),
        ("bxor", [IntLit(a), IntLit(b)])                              => Some(IntLit(a ^ b)),
        ("bshl", [IntLit(a), IntLit(b)]) if (0..64).contains(b)       => Some(IntLit(a << b)),
        ("bshr", [IntLit(a), IntLit(b)]) if (0..64).contains(b)       => Some(IntLit(a >> b)),

        // Equality / comparison — return user-defined `Bool` ctor.
        ("int_eq", [IntLit(a), IntLit(b)]) => Some(bool_ctor(a == b)),
        ("sym_eq", [SymLit(a), SymLit(b)]) => Some(bool_ctor(a == b)),
        ("lt",     [IntLit(a), IntLit(b)]) => Some(bool_ctor(a < b)),
        ("le",     [IntLit(a), IntLit(b)]) => Some(bool_ctor(a <= b)),

        // Fresh symbol generation. Zero-arity; each call yields a
        // unique Symbol of the form "_fresh<N>". Effectful.
        ("gen_fresh", []) => Some(SymLit(gen_fresh_name())),

        _ => None,
    }
}

fn bool_ctor(b: bool) -> Expr {
    Expr::Ctor(
        if b { "True" } else { "False" }.into(),
        Vec::new(),
    )
}

static FRESH_COUNTER: AtomicU64 = AtomicU64::new(0);

fn gen_fresh_name() -> Symbol {
    let n = FRESH_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("_fresh{n}")
}
