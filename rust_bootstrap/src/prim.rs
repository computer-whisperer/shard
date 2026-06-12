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

use num_traits::{Signed, ToPrimitive, Zero};

use crate::ast::{Expr, IntLit as Int, Symbol};

/// Try to apply a primitive to fully-reduced args. Returns `Some(e)`
/// if `name` is a known primitive and the args fit the expected
/// shape; `None` otherwise (the call stays stuck, which the
/// evaluator surfaces as `UnknownCall`).
///
/// CONFORMANCE: this table and kernel/reduce.shard's `try_step_prim`
/// are the two primitive tables (eval.shard's interpreter reuses the
/// latter). The `prim_conformance_*` tests in lib.rs sweep both over a
/// value matrix — when adding/removing a name here, update the spec
/// lists there (SHARED_INT2 / OBJECT_ONLY_*) or the sweep fails.
pub fn try_apply(name: &str, args: &[Expr]) -> Option<Expr> {
    use Expr::{IntLit, SymLit};

    match (name, args) {
        // Integer arithmetic — arbitrary precision (BigInt), matching the
        // documented mathematical `Int`. `/` truncates toward zero (as the
        // old i64 `/` did); `mod` uses Euclidean remainder so the result is
        // always non-negative for a positive modulus, matching the
        // convention the modular-arithmetic library wrappers will rely on.
        ("+",   [IntLit(a), IntLit(b)])                    => Some(IntLit(a + b)),
        ("-",   [IntLit(a), IntLit(b)])                    => Some(IntLit(a - b)),
        ("*",   [IntLit(a), IntLit(b)])                    => Some(IntLit(a * b)),
        ("/",   [IntLit(a), IntLit(b)]) if !b.is_zero()    => Some(IntLit(a / b)),
        ("mod", [IntLit(a), IntLit(b)]) if !b.is_zero()    => Some(IntLit(rem_euclid(a, b))),

        // The explicitly-paired division variants (semantics in the operator
        // NAME, never in context): `/`+`tmod` is the truncating pair (both
        // round toward zero; tmod's sign follows the dividend — BigInt's `%`),
        // `ediv`+`mod` the Euclidean pair (0 <= mod < |b|). The mixed pair
        // `/`+`mod` is NOT an identity pair on negatives — see std/div.
        ("tmod", [IntLit(a), IntLit(b)]) if !b.is_zero()   => Some(IntLit(a % b)),
        ("ediv", [IntLit(a), IntLit(b)]) if !b.is_zero()   => Some(IntLit((a - rem_euclid(a, b)) / b)),

        // Bitwise on non-negative ints. Shifts by 64+ or negative amounts
        // are caller errors; we keep the i64-era guard (call stays stuck)
        // so the Rust table and kernel/reduce.shard's mirror agree.
        ("band", [IntLit(a), IntLit(b)])                              => Some(IntLit(a & b)),
        ("bor",  [IntLit(a), IntLit(b)])                              => Some(IntLit(a | b)),
        ("bxor", [IntLit(a), IntLit(b)])                              => Some(IntLit(a ^ b)),
        ("bshl", [IntLit(a), IntLit(b)]) if shift_amount(b).is_some() => Some(IntLit(a << shift_amount(b)?)),
        ("bshr", [IntLit(a), IntLit(b)]) if shift_amount(b).is_some() => Some(IntLit(a >> shift_amount(b)?)),

        // Equality / comparison — return user-defined `Bool` ctor.
        ("int_eq", [IntLit(a), IntLit(b)]) => Some(bool_ctor(a == b)),
        ("sym_eq", [SymLit(a), SymLit(b)]) => Some(bool_ctor(a == b)),
        ("lt",     [IntLit(a), IntLit(b)]) => Some(bool_ctor(a < b)),
        ("le",     [IntLit(a), IntLit(b)]) => Some(bool_ctor(a <= b)),

        // Fresh symbol generation. Zero-arity; each call yields a
        // unique Symbol of the form "_fresh<N>". Effectful.
        ("gen_fresh", []) => Some(SymLit(gen_fresh_name())),

        // Symbol ↔ characters. The bridge a self-hosted parser needs:
        // text is made of symbols, but `Symbol` is otherwise opaque
        // (only `quote` literals and `sym_eq`). Both total and pure.
        // The char list is the name's UTF-8 BYTES (issue #2 Phase 3 —
        // the same representation "…" literals and the extern wire use,
        // and what rt.h's compiled bridge always spoke).
        //   sym_of_chars : (List Int) -> Symbol   (UTF-8 bytes in)
        //   chars_of_sym : Symbol     -> (List Int) (the inverse)
        ("sym_of_chars", [list])     => decode_char_list(list).map(SymLit),
        ("chars_of_sym", [SymLit(s)]) => Some(encode_char_list(s)),

        _ => None,
    }
}

/// Euclidean remainder: in `[0, |b|)`, like `i64::rem_euclid`. BigInt's
/// `%` truncates (sign follows the dividend), so fix up negatives.
fn rem_euclid(a: &Int, b: &Int) -> Int {
    let r = a % b;
    if r.is_negative() { r + b.abs() } else { r }
}

/// The i64-era shift guard: a shift amount must be in `0..64`.
/// None outside that range (the call stays stuck).
fn shift_amount(b: &Int) -> Option<u64> {
    b.to_u64().filter(|k| *k < 64)
}

/// Decode an object `(List Int)` value — a `Cons`/`Nil` spine of
/// `IntLit` UTF-8 bytes — into a String. None if the spine is malformed,
/// an element is outside 0..256, or the bytes are not valid UTF-8
/// (symbol names stay valid host strings; the call stays stuck).
fn decode_char_list(e: &Expr) -> Option<String> {
    let mut bytes = Vec::new();
    let mut cur = e;
    loop {
        match cur {
            Expr::Ctor(n, a) if n == "Nil" && a.is_empty() => {
                return String::from_utf8(bytes).ok();
            }
            Expr::Ctor(n, a) if n == "Cons" && a.len() == 2 => {
                match &a[0] {
                    Expr::IntLit(cp) => bytes.push(cp.to_u8()?),
                    _ => return None,
                }
                cur = &a[1];
            }
            _ => return None,
        }
    }
}

/// Encode a symbol name as the object `(List Int)` of its UTF-8 bytes —
/// the same representation a `"…"` string literal lowers to.
fn encode_char_list(s: &str) -> Expr {
    let mut acc = Expr::Ctor("Nil".into(), Vec::new());
    for b in s.bytes().rev() {
        acc = Expr::Ctor("Cons".into(), vec![Expr::IntLit(b.into()), acc]);
    }
    acc
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
