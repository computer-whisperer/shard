//! The primitive operations, as a table over `Expr` values. The evaluator
//! applies a call whose head is no declared `fn` through `eval.rs`
//! `apply_other`: its fast arms first, then this table. A name or an
//! argument shape the table does not cover leaves the call stuck.
//!
//! The comparisons (`=`, `sym_eq`, `<`, `<=`) return the cells `True` /
//! `False`, and the symbol bridge speaks `Cons` / `Nil` cells — the names
//! the runtime gives its cells, under which the reader hands the evaluator
//! the constructors so named (`plain/out.rs`): those of Init's `Bool` and
//! `List`, or a file's own.

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
    use Expr::{IntLit, SymLit};

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
        // complement (BigInt's operators). A shift count outside 0..64
        // leaves the call stuck.
        ("band", [IntLit(a), IntLit(b)])                              => Some(IntLit(a & b)),
        ("bor",  [IntLit(a), IntLit(b)])                              => Some(IntLit(a | b)),
        ("bxor", [IntLit(a), IntLit(b)])                              => Some(IntLit(a ^ b)),
        ("bshl", [IntLit(a), IntLit(b)]) if shift_amount(b).is_some() => Some(IntLit(a << shift_amount(b)?)),
        ("bshr", [IntLit(a), IntLit(b)]) if shift_amount(b).is_some() => Some(IntLit(a >> shift_amount(b)?)),

        // Equality and comparison: `True` / `False` (the module header).
        ("=", [IntLit(a), IntLit(b)]) => Some(bool_ctor(a == b)),
        ("sym_eq", [SymLit(a), SymLit(b)]) => Some(bool_ctor(a == b)),
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

        // Symbol ↔ bytes, the bridge the V3 reader is written on: a
        // `Symbol` is otherwise opaque (`quote` literals and `sym_eq` only).
        // The list is the name's UTF-8 BYTES — the representation of a
        // string literal, of the host calls and of the compiled bridge in
        // v3/kernel/rt.h.
        //   sym_of_chars : (List Int) -> Symbol     stuck unless the list is valid UTF-8
        //   chars_of_sym : Symbol     -> (List Int) its inverse
        ("sym_of_chars", [list])     => decode_char_list(list).map(SymLit),
        ("chars_of_sym", [SymLit(s)]) => Some(encode_char_list(s)),

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

/// The shift guard: a shift count must be in `0..64`.
/// None outside that range (the call stays stuck).
fn shift_amount(b: &Int) -> Option<u64> {
    b.to_u64().filter(|k| *k < 64)
}

/// Decode a `(List Int)` value — a `Cons`/`Nil` spine of
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

/// Encode a symbol name as the `(List Int)` of its UTF-8 bytes — what a
/// `"…"` string literal loads as.
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
