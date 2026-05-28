//! Evaluator over the narrow object language.
//!
//! Strategy: call-by-value. Reduce each argument before opening a
//! user-fn's body with them or handing them to a native primitive.
//! `match` evaluates its scrutinee, walks arms left-to-right looking
//! for the first that fits, and opens the chosen arm's body with the
//! captured bindings. `let` evaluates RHSs in the outer scope
//! (parallel let), then opens the body with the resulting values.
//! `if` reduces its condition to a `True`/`False` ctor and dispatches.
//!
//! The locally-nameless discipline: `open_many` puts values into the
//! body wherever it had `BVar k` references. Bindings are passed
//! *innermost-first* (BVar 0 first), matching the term-language
//! convention.

use crate::ast::{Arm, Expr, Module, Pat};
use crate::prim;

#[derive(Debug)]
pub enum EvalError {
    UnknownCall(String),
    ArityMismatch {
        name: String,
        expected: usize,
        got: usize,
    },
    UnboundBVar(u32),
    NoMatchArm(String),
    IfNonBool(String),
    Unimplemented(&'static str),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::UnknownCall(name) => write!(f, "unknown call: {name}"),
            EvalError::ArityMismatch { name, expected, got } => {
                write!(f, "arity mismatch calling {name}: expected {expected}, got {got}")
            }
            EvalError::UnboundBVar(k) => write!(f, "unbound BVar({k}) — body not opened"),
            EvalError::NoMatchArm(v) => write!(f, "no match arm fired for value {v}"),
            EvalError::IfNonBool(v) => write!(f, "if condition not True/False: {v}"),
            EvalError::Unimplemented(what) => write!(f, "not yet implemented: {what}"),
        }
    }
}

impl std::error::Error for EvalError {}

/// Reduce `e` to normal form within the context of `m`'s definitions.
pub fn eval(m: &Module, e: &Expr) -> Result<Expr, EvalError> {
    match e {
        Expr::IntLit(_) | Expr::SymLit(_) | Expr::FVar(_) => Ok(e.clone()),

        Expr::Ctor(name, args) => {
            let evaled: Vec<Expr> = args.iter().map(|a| eval(m, a)).collect::<Result<_, _>>()?;
            Ok(Expr::Ctor(name.clone(), evaled))
        }

        Expr::Call(name, args) => {
            let evaled: Vec<Expr> = args.iter().map(|a| eval(m, a)).collect::<Result<_, _>>()?;
            apply_call(m, name, &evaled)
        }

        Expr::If(c, t, e) => {
            let c_val = eval(m, c)?;
            if is_true_ctor(&c_val) {
                eval(m, t)
            } else if is_false_ctor(&c_val) {
                eval(m, e)
            } else {
                Err(EvalError::IfNonBool(format!("{c_val:?}")))
            }
        }

        Expr::Match(scrut, arms) => {
            let v = eval(m, scrut)?;
            for arm in arms {
                if let Some(bindings) = match_pat(&arm.pat, &v) {
                    let opened = open_many(&bindings, &arm.body);
                    return eval(m, &opened);
                }
            }
            Err(EvalError::NoMatchArm(format!("{v:?}")))
        }

        Expr::Let(rhss, body) => {
            // Parallel let: RHSs evaluated in outer scope.
            let evaled: Vec<Expr> = rhss.iter().map(|e| eval(m, e)).collect::<Result<_, _>>()?;
            // Innermost-first: last declaration is BVar 0.
            let bindings: Vec<Expr> = evaled.into_iter().rev().collect();
            let opened = open_many(&bindings, body);
            eval(m, &opened)
        }

        Expr::BVar(k) => Err(EvalError::UnboundBVar(*k)),
    }
}

fn apply_call(m: &Module, name: &str, args: &[Expr]) -> Result<Expr, EvalError> {
    // User-defined fn? Open its body with the args, then keep going.
    if let Some(fd) = m.lookup_fn(name) {
        if fd.params.len() != args.len() {
            return Err(EvalError::ArityMismatch {
                name: name.into(),
                expected: fd.params.len(),
                got: args.len(),
            });
        }
        // Reverse so bindings[0] fills BVar 0 (the LAST parameter).
        let bindings: Vec<Expr> = args.iter().rev().cloned().collect();
        let opened = open_many(&bindings, &fd.body);
        return eval(m, &opened);
    }
    if let Some(out) = prim::try_apply(name, args) {
        return Ok(out);
    }
    Err(EvalError::UnknownCall(name.into()))
}

fn is_true_ctor(e: &Expr) -> bool {
    matches!(e, Expr::Ctor(n, a) if a.is_empty() && n == "True")
}

fn is_false_ctor(e: &Expr) -> bool {
    matches!(e, Expr::Ctor(n, a) if a.is_empty() && n == "False")
}

// -----------------------------------------------------------------------------
// Pattern matching. Mirrors kernel/reduce.sexp:match_pat.
//
// Convention: bindings collected innermost-first. The LAST PVar
// encountered (rightmost in the pattern) becomes `BVar 0` in the arm
// body; earlier PVars get higher indices.
// -----------------------------------------------------------------------------

fn match_pat(p: &Pat, v: &Expr) -> Option<Vec<Expr>> {
    let mut acc = Vec::new();
    if match_pat_acc(p, v, &mut acc) {
        Some(acc)
    } else {
        None
    }
}

fn match_pat_acc(p: &Pat, v: &Expr, acc: &mut Vec<Expr>) -> bool {
    match p {
        Pat::PVar => {
            // Prepend so acc[0] is always the most recently captured
            // (= innermost binder = BVar 0).
            acc.insert(0, v.clone());
            true
        }
        Pat::PInt(n) => matches!(v, Expr::IntLit(m) if m == n),
        Pat::PSym(s) => matches!(v, Expr::SymLit(t) if t == s),
        Pat::PCtor(cn, sub_pats) => {
            if let Expr::Ctor(vc, vargs) = v {
                if cn == vc && sub_pats.len() == vargs.len() {
                    for (sp, sv) in sub_pats.iter().zip(vargs.iter()) {
                        if !match_pat_acc(sp, sv, acc) {
                            return false;
                        }
                    }
                    return true;
                }
            }
            false
        }
    }
}

fn pat_arity(p: &Pat) -> u32 {
    match p {
        Pat::PVar => 1,
        Pat::PCtor(_, sub_pats) => sub_pats.iter().map(pat_arity).sum(),
        Pat::PInt(_) | Pat::PSym(_) => 0,
    }
}

// -----------------------------------------------------------------------------
// Binder opening (locally-nameless). Mirrors kernel/term.sexp:open_many_at.
//
// `bindings[k]` fills BVar k for k in [0, len). Outer BVars
// (k >= len) shift down by len. Recursion under inner binders bumps
// `depth` by the count of binders introduced.
// -----------------------------------------------------------------------------

fn open_many(bindings: &[Expr], e: &Expr) -> Expr {
    open_many_at(0, bindings, e)
}

fn open_many_at(depth: u32, bindings: &[Expr], e: &Expr) -> Expr {
    match e {
        Expr::BVar(k) => {
            if *k < depth {
                e.clone()
            } else {
                let idx = (k - depth) as usize;
                if idx < bindings.len() {
                    // The value moves under `depth` more binders. For
                    // bindings without their own free BVars (true of
                    // every evaluated value the evaluator produces),
                    // shifting is a no-op. We rely on that invariant
                    // here; if it ever breaks we'll need a Rust
                    // `shift` mirroring kernel/term.sexp.
                    bindings[idx].clone()
                } else {
                    Expr::BVar(k - bindings.len() as u32)
                }
            }
        }
        Expr::FVar(_) | Expr::IntLit(_) | Expr::SymLit(_) => e.clone(),
        Expr::Call(name, args) => Expr::Call(
            name.clone(),
            args.iter().map(|a| open_many_at(depth, bindings, a)).collect(),
        ),
        Expr::Ctor(name, args) => Expr::Ctor(
            name.clone(),
            args.iter().map(|a| open_many_at(depth, bindings, a)).collect(),
        ),
        Expr::If(c, t, e2) => Expr::If(
            Box::new(open_many_at(depth, bindings, c)),
            Box::new(open_many_at(depth, bindings, t)),
            Box::new(open_many_at(depth, bindings, e2)),
        ),
        Expr::Match(scrut, arms) => Expr::Match(
            Box::new(open_many_at(depth, bindings, scrut)),
            arms.iter()
                .map(|a| Arm {
                    pat: a.pat.clone(),
                    body: open_many_at(depth + pat_arity(&a.pat), bindings, &a.body),
                })
                .collect(),
        ),
        Expr::Let(rhss, body) => Expr::Let(
            rhss.iter().map(|e| open_many_at(depth, bindings, e)).collect(),
            Box::new(open_many_at(depth + rhss.len() as u32, bindings, body)),
        ),
    }
}
