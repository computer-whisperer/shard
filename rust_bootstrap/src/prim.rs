//! The primitive operations, as a table over `Expr` values. The evaluator
//! applies a call whose head is no declared `fn` through `eval.rs`
//! `apply_other`: its fast arms first, then this table. A name or an
//! argument shape the table does not cover leaves the call stuck.
//!
//! The comparisons (`=`, `<`, `<=`) return the cells `True` / `False` — the
//! names the runtime gives its cells, under which the reader hands the
//! evaluator the constructors of Init's `Bool` (`plain/out.rs`).

use num_traits::{Signed, ToPrimitive, Zero};

use crate::ast::{Expr, IntLit as Int};

/// Apply a primitive to evaluated arguments: `Some(e)` if `name` is a
/// primitive and the arguments fit its shape, `None` otherwise (the call
/// stays stuck, which the evaluator reports as an error).
///
/// CONFORMANCE: the toolchain's own primitive table is v3/kernel/prog.shard's,
/// run by `ev`; v3/kernel/test/prims_test.shard compares the two. lib.rs
/// sweeps this table against the evaluator's fast arms over a value matrix —
/// when adding or removing a name here, update its SHARED_INT2.
pub fn try_apply(name: &str, args: &[Expr]) -> Option<Expr> {
    use Expr::IntLit;

    match (name, args) {
        // Integer arithmetic, arbitrary precision. `/` truncates toward
        // zero; `mod` is the Euclidean remainder, never negative. A zero
        // divisor leaves the call stuck.
        ("+",   [IntLit(a), IntLit(b)])                    => Some(IntLit(a + b)),
        ("-",   [IntLit(a), IntLit(b)])                    => Some(IntLit(a - b)),
        ("*",   [IntLit(a), IntLit(b)])                    => Some(IntLit(a * b)),
        ("/",   [IntLit(a), IntLit(b)]) if !b.is_zero()    => Some(IntLit(a / b)),
        ("mod", [IntLit(a), IntLit(b)]) if !b.is_zero()    => Some(IntLit(rem_euclid(a, b))),

        // The explicitly-paired division variants (semantics in the operator
        // NAME, never in context): `/`+`tmod` is the truncating pair (both
        // round toward zero; tmod's sign follows the dividend — BigInt's `%`),
        // `ediv`+`mod` the Euclidean pair (0 <= mod < |b|). The mixed pair
        // `/`+`mod` is NOT an identity pair on negatives.
        ("tmod", [IntLit(a), IntLit(b)]) if !b.is_zero()   => Some(IntLit(a % b)),
        ("ediv", [IntLit(a), IntLit(b)]) if !b.is_zero()   => Some(IntLit((a - rem_euclid(a, b)) / b)),

        // Bitwise operations; a negative operand is read in two's
        // complement (BigInt's operators). A shift is by any count that is a
        // natural number — the engine's side of Init's `Int.shiftLeft` and
        // `Int.shiftRight` (v3/kernel/prog.shard realization_of): m · 2^n, and
        // ⌊m / 2^n⌋. A negative count leaves the call stuck; a left shift past
        // the evaluator's sizes is stuck as `Nat.shiftLeft` is.
        ("band", [IntLit(a), IntLit(b)])                       => Some(IntLit(a & b)),
        ("bor",  [IntLit(a), IntLit(b)])                       => Some(IntLit(a | b)),
        ("bxor", [IntLit(a), IntLit(b)])                       => Some(IntLit(a ^ b)),
        ("bshl", [IntLit(a), IntLit(b)]) if !b.is_negative()   => shl_any(a, b).map(IntLit),
        ("bshr", [IntLit(a), IntLit(b)]) if !b.is_negative()   => Some(IntLit(shr_any(a, b))),

        // Equality and comparison: `True` / `False` (the module header).
        ("=", [IntLit(a), IntLit(b)]) => Some(bool_ctor(a == b)),
        ("<",   [IntLit(a), IntLit(b)]) => Some(bool_ctor(a < b)),
        ("<=",  [IntLit(a), IntLit(b)]) => Some(bool_ctor(a <= b)),

        // Init's spellings (v3/kernel/prog.shard's entries 18–38, as v3/kernel/ev.shard
        // applies them): what the reader writes for Init's operators.
        // Int's divisions are total — a zero divisor gives 0, a remainder by zero the
        // dividend.
        ("Int.tdiv", [IntLit(a), IntLit(b)]) => Some(IntLit(if b.is_zero() { Int::zero() } else { a / b })),
        ("Int.tmod", [IntLit(a), IntLit(b)]) => Some(IntLit(if b.is_zero() { a.clone() } else { a % b })),
        ("Int.ediv", [IntLit(a), IntLit(b)]) => {
            Some(IntLit(if b.is_zero() { Int::zero() } else { (a - rem_euclid(a, b)) / b }))
        }
        ("Int.emod", [IntLit(a), IntLit(b)]) => Some(IntLit(if b.is_zero() { a.clone() } else { rem_euclid(a, b) })),
        // K's accelerated Nat set: defined on two naturals (a negative operand leaves the
        // call stuck), total where Lean's are, and stuck where K's resource limits stop the
        // work — a result past 2^27 bytes, a count past 32 bits.
        (n, [IntLit(a), IntLit(b)]) if n.starts_with("Nat.") && !a.is_negative() && !b.is_negative() => nat_prim(n, a, b),

        _ => None,
    }
}

/// A constructor without fields, by its identity: Init's Bool and Decidable cells.
fn cell(name: &str) -> Expr {
    Expr::Ctor(name.into(), Vec::new())
}

/// The bytes a natural takes (`ev.shard` ev_nat_bytes): what K's size limit counts.
fn nat_bytes(n: &Int) -> u64 {
    n.bits().div_ceil(8)
}

const NAT_MAX_SIZE: u64 = 134_217_728;
const COUNT_MAX: u64 = 4_294_967_295;

/// A natural under K's literal size, else stuck.
fn nat_sized(r: Int) -> Option<Expr> {
    (nat_bytes(&r) <= NAT_MAX_SIZE).then_some(Expr::IntLit(r))
}

fn nat_prim(name: &str, a: &Int, b: &Int) -> Option<Expr> {
    use num_traits::One;
    let int = |n: Int| Some(Expr::IntLit(n));
    match name {
        "Nat.sub" => int(if a < b { Int::zero() } else { a - b }),
        "Nat.land" => int(a & b),
        "Nat.lor" => int(a | b),
        "Nat.xor" => int(a ^ b),
        "Nat.shiftLeft" => {
            if a.is_zero() {
                return int(Int::zero());
            }
            let k = b.to_u64().filter(|k| *k <= COUNT_MAX)?;
            if nat_bytes(a) + k / 8 + 1 > NAT_MAX_SIZE {
                return None;
            }
            int(a << k)
        }
        "Nat.shiftRight" => int(match b.to_u64() {
            Some(k) => a >> k,
            None => Int::zero(),
        }),
        "Nat.add" => nat_sized(a + b),
        "Nat.mul" => nat_sized(a * b),
        "Nat.div" => int(if b.is_zero() { Int::zero() } else { a / b }),
        "Nat.mod" => int(if b.is_zero() { a.clone() } else { a % b }),
        "Nat.gcd" => {
            let (mut x, mut y) = (a.clone(), b.clone());
            while !y.is_zero() {
                let r = &x % &y;
                x = y;
                y = r;
            }
            int(x)
        }
        "Nat.pow" => {
            let k = b.to_u64().filter(|k| *k <= COUNT_MAX)?;
            if k >= 1 && *a > Int::one() && NAT_MAX_SIZE / k < nat_bytes(a) {
                return None;
            }
            nat_sized(num_traits::pow(a.clone(), k as usize))
        }
        "Nat.beq" => Some(cell(if a == b { "Bool.true" } else { "Bool.false" })),
        "Nat.ble" => Some(cell(if a <= b { "Bool.true" } else { "Bool.false" })),
        "Nat.decEq" => Some(cell(if a == b { "Decidable.isTrue" } else { "Decidable.isFalse" })),
        "Nat.decLt" => Some(cell(if a < b { "Decidable.isTrue" } else { "Decidable.isFalse" })),
        "Nat.decLe" => Some(cell(if a <= b { "Decidable.isTrue" } else { "Decidable.isFalse" })),
        _ => None,
    }
}

/// Euclidean remainder: in `[0, |b|)`, like `i64::rem_euclid`. BigInt's
/// `%` truncates (sign follows the dividend), so fix up negatives.
pub(crate) fn rem_euclid(a: &Int, b: &Int) -> Int {
    let r = a % b;
    if r.is_negative() { r + b.abs() } else { r }
}

/// `a · 2^k` for a count `k >= 0`, under `Nat.shiftLeft`'s guards: the count
/// within 32 bits and the result within the literal size, else None (stuck).
fn shl_any(a: &Int, k: &Int) -> Option<Int> {
    if a.is_zero() {
        return Some(Int::zero());
    }
    let k = k.to_u64().filter(|k| *k <= COUNT_MAX)?;
    if nat_bytes(a) + k / 8 + 1 > NAT_MAX_SIZE {
        return None;
    }
    Some(a << k)
}

/// `⌊a / 2^k⌋` for a count `k >= 0`: BigInt's shift rounds toward −∞, and a
/// count past 64 bits leaves the sign alone.
fn shr_any(a: &Int, k: &Int) -> Int {
    match k.to_u64() {
        Some(k) => a >> k,
        None => if a.is_negative() { Int::from(-1) } else { Int::zero() },
    }
}

fn bool_ctor(b: bool) -> Expr {
    Expr::Ctor(
        if b { "True" } else { "False" }.into(),
        Vec::new(),
    )
}
