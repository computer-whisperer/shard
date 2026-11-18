//! A read body's static types, its operators by their operands' types, its
//! rows written out (`v3/kernel/classify.shard`: c_type, settle_ops,
//! expand_rows — the same functions by name).
//!
//! The variables' types are a slice, the outermost first: `BVar i` is at
//! `tys[len - 1 - i]`.

use super::read::{bool_eq_cond, bool_false, bool_true, cmp_prim, prim_find, realization_type};
use super::{Decl, EInfo, ETy, Lit, Pat, R, Tables, Tm, rep_row, unread};

fn con(n: &str) -> ETy {
    ETy::Con(n.to_string(), vec![])
}

fn is_named(t: &Option<ETy>, n: &str) -> bool {
    matches!(t, Some(ETy::Con(m, args)) if m == n && args.is_empty())
}

fn subst_ty(args: &[ETy], t: &ETy) -> ETy {
    match t {
        ETy::Var(i) => args.get(*i).cloned().unwrap_or(ETy::Var(*i)),
        ETy::Con(n, ts) => ETy::Con(n.clone(), ts.iter().map(|x| subst_ty(args, x)).collect()),
    }
}

/// One more than the greatest type parameter a type names.
fn tvar_count(t: &ETy) -> usize {
    match t {
        ETy::Var(i) => i + 1,
        ETy::Con(_, ts) => ts.iter().map(tvar_count).max().unwrap_or(0),
    }
}

type Sub = Vec<(usize, ETy)>;

/// A type parameter is bound once, by the first type that reaches it.
fn unify_ty(d: &ETy, a: &ETy, acc: &mut Sub) {
    match d {
        ETy::Var(i) => {
            if !acc.iter().any(|(j, _)| j == i) {
                acc.push((*i, a.clone()));
            }
        }
        ETy::Con(_, ds) => {
            if let ETy::Con(_, as_) = a {
                for (d1, a1) in ds.iter().zip(as_) {
                    unify_ty(d1, a1, acc);
                }
            }
        }
    }
}

/// The type parameters a head's declared types take from its arguments' static types.
fn unify_all(t: &Tables, tys: &[Option<ETy>], decl: &[ETy], args: &[Tm]) -> Sub {
    let mut acc = Vec::new();
    for (d, a) in decl.iter().zip(args) {
        if let Some(at) = c_type(t, tys, a) {
            unify_ty(d, &at, &mut acc);
        }
    }
    acc
}

/// The parameter list 0..n-1 from the bindings, an unbound one left as itself.
fn params_from(acc: &Sub, n: usize) -> Vec<ETy> {
    (0..n).map(|i| acc.iter().find(|(j, _)| *j == i).map(|(_, t)| t.clone()).unwrap_or(ETy::Var(i))).collect()
}

/// Constructor c's field types under the scrutinee's type arguments, when
/// its inductive is the scrutinee's.
pub fn ctor_field_types(t: &Tables, st: &Option<ETy>, c: &str) -> Option<Vec<ETy>> {
    let Some(ETy::Con(ind, args)) = st else { return None };
    let Some(Decl::Ind { ctors, .. }) = t.find_decl(ind) else { return None };
    let (_, fts) = ctors.iter().find(|(n, _)| n == c)?;
    Some(fts.iter().map(|ft| subst_ty(args, ft)).collect())
}

/// A pattern's variables' types in capture order, from the scrutinee's type.
pub fn pat_types(t: &Tables, st: &Option<ETy>, p: &Pat) -> Vec<Option<ETy>> {
    match p {
        Pat::Var => vec![st.clone()],
        Pat::Lit(_) => vec![],
        Pat::Ctor(c, subs) => {
            let fts = ctor_field_types(t, st, c);
            let mut out = Vec::new();
            for (i, sp) in subs.iter().enumerate() {
                let ft = fts.as_ref().and_then(|f| f.get(i).cloned());
                out.extend(pat_types(t, &ft, sp));
            }
            out
        }
    }
}

/// A primitive's result type head (prog.shard prim_result).
fn prim_result(code: u32) -> ETy {
    match code {
        0 | 3 | 4 | 34 | 35 => con("Bool"),
        36..=38 => con("Decidable"),
        22..=33 => con("Nat"),
        _ => con("Int"),
    }
}

/// The static E type of a term, as far as the declarations determine it.
pub fn c_type(t: &Tables, tys: &[Option<ETy>], tm: &Tm) -> Option<ETy> {
    match tm {
        Tm::BVar(i) => tys.len().checked_sub(1 + *i).and_then(|k| tys[k].clone()),
        Tm::Lit(Lit::Nat(_)) => Some(con("Nat")),
        Tm::Lit(Lit::Int(_)) => Some(con("Int")),
        Tm::Lit(Lit::Str(_)) => Some(con("String")),
        Tm::Ctor(c, args) => {
            let Some(EInfo::Ctor { ind, .. }) = t.einfo(c) else { return None };
            match t.ctor_fields(c) {
                Some((_, np, fts)) => Some(ETy::Con(ind.clone(), params_from(&unify_all(t, tys, fts, args), np))),
                None => Some(ETy::Con(ind.clone(), vec![])),
            }
        }
        Tm::Call(f, args) | Tm::Ext(f, args) => match (tm, t.find_decl(f)) {
            (Tm::Call(..), Some(Decl::Fn { params, ret, .. }))
            | (Tm::Call(..), Some(Decl::Sig { params, ret, .. }))
            | (Tm::Ext(..), Some(Decl::Extern { params, ret, .. })) => {
                Some(subst_ty(&params_from(&unify_all(t, tys, params, args), tvar_count(ret)), ret))
            }
            _ => None,
        },
        Tm::Prim(n, args) => match prim_find(n) {
            Some((code, _)) => {
                // + - * at two Nat operands is Nat, else the table's result type
                if matches!(code, 1 | 2 | 8)
                    && args.len() == 2
                    && is_named(&c_type(t, tys, &args[0]), "Nat")
                    && is_named(&c_type(t, tys, &args[1]), "Nat")
                {
                    Some(con("Nat"))
                } else {
                    Some(prim_result(code))
                }
            }
            // a row kept by name while the body is typed: a subtype's value is at its base
            None => {
                if n == "Subtype.val" {
                    match args.first().map(|x| c_type(t, tys, x)) {
                        Some(Some(ETy::Con(_, targs))) if targs.len() == 1 => Some(targs[0].clone()),
                        _ => None,
                    }
                } else {
                    realization_type(n)
                }
            }
        },
        Tm::Match(s, arms) => {
            // the first arm that is not a refuted row
            let (p, b) = arms.iter().find(|(_, b)| !matches!(b, Tm::Absurd))?;
            let mut tys1 = tys.to_vec();
            tys1.extend(pat_types(t, &c_type(t, tys, s), p));
            c_type(t, &tys1, b)
        }
        Tm::Let(rs, b) => {
            let mut tys1 = tys.to_vec();
            for r in rs {
                let ty = c_type(t, &tys1, r);
                tys1.push(ty);
            }
            c_type(t, &tys1, b)
        }
        Tm::If(_, b, _) => c_type(t, tys, b),
        Tm::Absurd => None,
    }
}

// -----------------------------------------------------------------------------
// The operators of a body, by their operands' types
// -----------------------------------------------------------------------------

fn arith_op(p: &str) -> bool {
    matches!(p, "+" | "-" | "*" | "/" | "mod")
}

/// The bit-level five, whose entry is their operands' type's too: Init's at
/// Nat, the bare one at Int and where no type is known.
fn bit_op(p: &str) -> bool {
    matches!(p, "band" | "bor" | "bxor") || shift_op(p)
}

fn shift_op(p: &str) -> bool {
    matches!(p, "bshl" | "bshr")
}

/// The type an operator's second operand is read at, the first's being `ty`:
/// a shift's count is a Nat.
fn second_type(p: &str, ty: &Option<ETy>) -> Option<ETy> {
    if shift_op(p) { Some(con("Nat")) } else { ty.clone() }
}

/// A numeral, or an operator over numerals: it has no type of its own.
fn lit_only(t: &Tm) -> bool {
    match t {
        Tm::Lit(Lit::Nat(_)) => true,
        Tm::Prim(p, args) if args.len() == 2 && arith_op(p) => lit_only(&args[0]) && lit_only(&args[1]),
        _ => false,
    }
}

fn arith_entry(p: &str, ty: &Option<ETy>) -> String {
    let e = if is_named(ty, "Nat") {
        match p {
            "-" => "Nat.sub",
            "/" => "Nat.div",
            "mod" => "Nat.mod",
            "band" => "Nat.land",
            "bor" => "Nat.lor",
            "bxor" => "Nat.xor",
            "bshl" => "Nat.shiftLeft",
            "bshr" => "Nat.shiftRight",
            _ => p,
        }
    } else if is_named(ty, "Int") {
        match p {
            "/" => "Int.ediv",
            "mod" => "Int.emod",
            _ => p,
        }
    } else {
        p
    };
    e.to_string()
}

/// The operand type an entry written by name states.
fn prim_ptypes(p: &str) -> Vec<ETy> {
    match prim_find(p) {
        Some((22..=38, _)) => vec![con("Nat"), con("Nat")],
        Some((18..=21, _)) => vec![con("Int"), con("Int")],
        _ => vec![],
    }
}

/// A head's parameter types under the type arguments its result type takes
/// from the type expected of the call.
fn ptypes_under(ps: &[ETy], rt: &ETy, exp: &Option<ETy>) -> Vec<ETy> {
    match exp {
        Some(e) => {
            let mut sub = Vec::new();
            unify_ty(rt, e, &mut sub);
            let n = ps.iter().chain(std::iter::once(rt)).map(tvar_count).max().unwrap_or(0);
            let args = params_from(&sub, n);
            ps.iter().map(|p| subst_ty(&args, p)).collect()
        }
        None => ps.to_vec(),
    }
}

fn head_ptypes(t: &Tables, f: &str, exp: &Option<ETy>) -> Vec<ETy> {
    match t.find_decl(f) {
        Some(Decl::Fn { params, ret, .. }) | Some(Decl::Sig { params, ret, .. }) | Some(Decl::Extern { params, ret, .. }) => {
            ptypes_under(params, ret, exp)
        }
        _ => vec![],
    }
}

fn ctor_ptypes(t: &Tables, c: &str, exp: &Option<ETy>) -> Vec<ETy> {
    match t.ctor_fields(c) {
        Some((ind, _, fts)) => match exp {
            Some(ETy::Con(ind2, targs)) if ind == ind2 => fts.iter().map(|ft| subst_ty(targs, ft)).collect(),
            _ => fts.clone(),
        },
        None => vec![],
    }
}

fn settle_args(t: &Tables, tys: &[Option<ETy>], pts: &[ETy], args: Vec<Tm>) -> R<Vec<Tm>> {
    args.into_iter().enumerate().map(|(i, a)| settle_ops(t, tys, &pts.get(i).cloned(), a)).collect()
}

/// An operator's meaning is its operands' type: `-` at Nat is Nat.sub; `/`
/// and `mod` are Nat.div and Nat.mod there, Int.ediv and Int.emod at Int; a
/// comparison where Init's Bool is expected is the decision's value. A
/// numeral takes the type of the operand beside it, else the type expected
/// of the operation.
pub fn settle_ops(t: &Tables, tys: &[Option<ETy>], exp: &Option<ETy>, tm: Tm) -> R<Tm> {
    Ok(match tm {
        Tm::BVar(_) | Tm::Lit(_) | Tm::Absurd => tm,
        Tm::Ctor(c, args) => {
            let pts = ctor_ptypes(t, &c, exp);
            Tm::Ctor(c, settle_args(t, tys, &pts, args)?)
        }
        Tm::Call(f, args) => {
            let pts = head_ptypes(t, &f, exp);
            Tm::Call(f, settle_args(t, tys, &pts, args)?)
        }
        Tm::Ext(f, args) => {
            let pts = head_ptypes(t, &f, exp);
            Tm::Ext(f, settle_args(t, tys, &pts, args)?)
        }
        Tm::Prim(p, args) => settle_prim(t, tys, exp, p, args)?,
        Tm::Match(s, arms) => {
            let s1 = settle_ops(t, tys, &None, *s)?;
            let st = c_type(t, tys, &s1);
            let mut out = Vec::new();
            for (p, b) in arms {
                let mut tys1 = tys.to_vec();
                tys1.extend(pat_types(t, &st, &p));
                let b1 = settle_ops(t, &tys1, exp, b)?;
                out.push((p, b1));
            }
            Tm::Match(Box::new(s1), out)
        }
        Tm::Let(rs, b) => {
            let mut tys1 = tys.to_vec();
            let mut out = Vec::new();
            for v in rs {
                let v1 = settle_ops(t, &tys1, &None, v)?;
                tys1.push(c_type(t, &tys1, &v1));
                out.push(v1);
            }
            Tm::Let(out, Box::new(settle_ops(t, &tys1, exp, *b)?))
        }
        Tm::If(c, a, b) => Tm::If(
            Box::new(settle_ops(t, tys, &None, *c)?),
            Box::new(settle_ops(t, tys, exp, *a)?),
            Box::new(settle_ops(t, tys, exp, *b)?),
        ),
    })
}

fn settle_prim(t: &Tables, tys: &[Option<ETy>], exp: &Option<ETy>, p: String, mut args: Vec<Tm>) -> R<Tm> {
    if args.len() == 2 && (arith_op(&p) || bit_op(&p) || cmp_prim(&p)) {
        let b = args.pop().unwrap();
        let a = args.pop().unwrap();
        // under an expected Nat or Int the operation is that type's and both operands are
        // read at it (a shift's count at Nat)
        if !cmp_prim(&p) && (is_named(exp, "Nat") || is_named(exp, "Int")) {
            let a2 = settle_ops(t, tys, exp, a)?;
            return Ok(Tm::Prim(arith_entry(&p, exp), vec![a2, settle_ops(t, tys, &second_type(&p, exp), b)?]));
        }
        // no numeric type expected: the operation is the first operand's type's — of the
        // second where the first is numerals alone — and the other operand is read at it
        let (ty, a2, b2) = if lit_only(&a) && !lit_only(&b) {
            let b1 = settle_ops(t, tys, &None, b)?;
            let ty = c_type(t, tys, &b1);
            (ty.clone(), settle_ops(t, tys, &ty, a)?, b1)
        } else {
            let a1 = settle_ops(t, tys, &None, a)?;
            let ty = c_type(t, tys, &a1);
            (ty.clone(), a1, settle_ops(t, tys, &second_type(&p, &ty), b)?)
        };
        if cmp_prim(&p) {
            let cond = cmp_cond(t, &p, &ty, a2, b2)?;
            return Ok(if is_named(exp, "Bool") { Tm::If(Box::new(cond), Box::new(bool_true()), Box::new(bool_false())) } else { cond });
        }
        return Ok(Tm::Prim(arith_entry(&p, &ty), vec![a2, b2]));
    }
    let pts = prim_ptypes(&p);
    Ok(Tm::Prim(p, settle_args(t, tys, &pts, args)?))
}

/// A comparison as the test an `if` reads: the primitive at Nat and Int;
/// `=` at Init's Bool the test over its operands. At a type with a derived
/// equality it is the call of that decision — which this reader does not
/// read yet, and says so.
fn cmp_cond(t: &Tables, p: &str, ty: &Option<ETy>, a: Tm, b: Tm) -> R<Tm> {
    if p == "=" && is_named(ty, "Bool") {
        return Ok(bool_eq_cond(a, b));
    }
    if let Some(ETy::Con(n, _)) = ty {
        if rep_row(n) || matches!(t.einfo(n), Some(EInfo::Ind { .. })) {
            return unread("derived_equality", format!("{p} at {n}: a derived decision's call"));
        }
    }
    Ok(Tm::Prim(p.to_string(), vec![a, b]))
}

/// The rows kept by name while the body was typed, written out: `Int.ofNat
/// x` and a row's projection are their argument, a row with an expression is
/// the expression over its argument bound once.
pub fn expand_rows(tm: Tm) -> Tm {
    let all = |ts: Vec<Tm>| -> Vec<Tm> { ts.into_iter().map(expand_rows).collect() };
    match tm {
        Tm::BVar(_) | Tm::Lit(_) | Tm::Absurd => tm,
        Tm::Ctor(c, args) => Tm::Ctor(c, all(args)),
        Tm::Call(f, args) => Tm::Call(f, all(args)),
        Tm::Ext(f, args) => Tm::Ext(f, all(args)),
        Tm::Prim(p, mut args) if args.len() == 1 => {
            let a = expand_rows(args.pop().unwrap());
            match p.as_str() {
                "Int.ofNat" | "Fin.val" | "Subtype.val" => a,
                // |n|: (if (< n 0) (- 0 n) n)
                "Int.natAbs" => {
                    let zero = || Tm::Lit(Lit::Nat(0.into()));
                    Tm::Let(
                        vec![a],
                        Box::new(Tm::If(
                            Box::new(Tm::Prim("<".into(), vec![Tm::BVar(0), zero()])),
                            Box::new(Tm::Prim("-".into(), vec![zero(), Tm::BVar(0)])),
                            Box::new(Tm::BVar(0)),
                        )),
                    )
                }
                _ => Tm::Prim(p, vec![a]),
            }
        }
        Tm::Prim(p, args) => Tm::Prim(p, all(args)),
        Tm::Match(s, arms) => Tm::Match(Box::new(expand_rows(*s)), arms.into_iter().map(|(p, b)| (p, expand_rows(b))).collect()),
        Tm::Let(rs, b) => Tm::Let(all(rs), Box::new(expand_rows(*b))),
        Tm::If(c, a, b) => Tm::If(Box::new(expand_rows(*c)), Box::new(expand_rows(*a)), Box::new(expand_rows(*b))),
    }
}
