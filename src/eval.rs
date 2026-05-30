//! Evaluator over the narrow object language — an ENVIRONMENT MACHINE.
//!
//! Call-by-value. Rather than substituting argument values into a fn
//! body (the old `open_many`, which deep-copied the body *and* every
//! captured value on each call → O(n²) on programs that thread a long
//! list, like the self-hosted parser), we evaluate the body in place
//! against an environment of values. Values are reference-counted
//! (`Rc`), so capturing a list tail in a pattern or looking up a
//! variable is O(1) — no structural copying on the hot path.
//!
//! Binding convention (unchanged, locally-nameless / de Bruijn): the
//! environment is innermost-first, `env[0]` = `BVar 0`. Entering a
//! binder PREPENDS its freshly-bound values; this reproduces the de
//! Bruijn shift (existing indices move up by the number of new
//! binders) without any renumbering. A user fn's body is closed except
//! for its parameters, so a call evaluates the body in a FRESH
//! environment of just the (reversed) argument values.
//!
//! There are no lambdas in the narrow language (calls are saturated,
//! functions are top-level), so no closures are needed: a `Val` is
//! always a fully-evaluated, CLOSED term — note it has no `BVar`
//! variant, so a value structurally cannot carry a free index. That is
//! exactly the invariant the substitution machine relied on by hand.

use std::rc::Rc;

use crate::ast::{Expr, Module, Pat};
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

/// A fully-evaluated, closed value. Recursive children are `Rc`-shared
/// so cloning a value (variable lookup, pattern capture) is O(1) — the
/// shared-structure that makes this an environment machine rather than
/// a substitution machine.
#[derive(Clone)]
enum Val {
    Int(i64),
    Sym(Rc<str>),
    FVar(Rc<str>),
    Ctor(Rc<str>, Rc<[Val]>),
}

/// Reduce `e` to normal form within the context of `m`'s definitions.
pub fn eval(m: &Module, e: &Expr) -> Result<Expr, EvalError> {
    let v = eval_env(m, &[], e)?;
    Ok(val_to_expr(&v))
}

fn eval_env(m: &Module, env: &[Val], e: &Expr) -> Result<Val, EvalError> {
    match e {
        Expr::IntLit(n) => Ok(Val::Int(*n)),
        Expr::SymLit(s) => Ok(Val::Sym(Rc::from(s.as_str()))),
        Expr::FVar(s) => Ok(Val::FVar(Rc::from(s.as_str()))),

        // A bound variable indexes into the environment (innermost-first).
        Expr::BVar(k) => env
            .get(*k as usize)
            .cloned()
            .ok_or(EvalError::UnboundBVar(*k)),

        Expr::Ctor(name, args) => {
            let vals = eval_args(m, env, args)?;
            Ok(Val::Ctor(Rc::from(name.as_str()), vals.into()))
        }

        Expr::Call(name, args) => {
            let vals = eval_args(m, env, args)?;
            apply_call(m, name, vals)
        }

        Expr::If(c, t, e2) => match eval_env(m, env, c)? {
            Val::Ctor(ref n, ref a) if a.is_empty() && &**n == "True" => eval_env(m, env, t),
            Val::Ctor(ref n, ref a) if a.is_empty() && &**n == "False" => eval_env(m, env, e2),
            other => Err(EvalError::IfNonBool(format!("{:?}", val_to_expr(&other)))),
        },

        Expr::Match(scrut, arms) => {
            let v = eval_env(m, env, scrut)?;
            for arm in arms {
                let mut binds: Vec<Val> = Vec::new();
                if match_pat(&arm.pat, &v, &mut binds) {
                    // env' = bindings (innermost-first) ++ outer env.
                    binds.extend_from_slice(env);
                    return eval_env(m, &binds, &arm.body);
                }
            }
            Err(EvalError::NoMatchArm(format!("{:?}", val_to_expr(&v))))
        }

        Expr::Let(rhss, body) => {
            // Parallel let: RHSs evaluated in the outer scope.
            let mut vals = eval_args(m, env, rhss)?;
            vals.reverse(); // innermost-first: last binding becomes BVar 0
            vals.extend_from_slice(env);
            eval_env(m, &vals, body)
        }
    }
}

fn eval_args(m: &Module, env: &[Val], args: &[Expr]) -> Result<Vec<Val>, EvalError> {
    args.iter().map(|a| eval_env(m, env, a)).collect()
}

fn apply_call(m: &Module, name: &str, mut args: Vec<Val>) -> Result<Val, EvalError> {
    // User-defined fn? Evaluate its body in a fresh environment of the
    // (reversed) arguments — the body is closed except for its params.
    if let Some(fd) = m.lookup_fn(name) {
        if fd.params.len() != args.len() {
            return Err(EvalError::ArityMismatch {
                name: name.into(),
                expected: fd.params.len(),
                got: args.len(),
            });
        }
        args.reverse(); // env[0] = BVar 0 = LAST parameter
        return eval_env(m, &args, &fd.body);
    }
    // Primitive: cross to the `Expr`-typed primitive table at the
    // boundary (primitive arguments are small — ints, syms, or the
    // occasional char list, all O(arg)).
    let arg_exprs: Vec<Expr> = args.iter().map(val_to_expr).collect();
    if let Some(out) = prim::try_apply(name, &arg_exprs) {
        return Ok(val_of_value_expr(&out));
    }
    Err(EvalError::UnknownCall(name.into()))
}

// -----------------------------------------------------------------------------
// Pattern matching. Mirrors kernel/reduce.sexp:match_pat.
//
// Convention: bindings collected innermost-first. The LAST PVar
// encountered (rightmost in the pattern) becomes `BVar 0` in the arm
// body; earlier PVars get higher indices.
// -----------------------------------------------------------------------------

fn match_pat(p: &Pat, v: &Val, acc: &mut Vec<Val>) -> bool {
    match p {
        Pat::PVar => {
            // Prepend so acc[0] is the most recently captured (= BVar 0).
            acc.insert(0, v.clone());
            true
        }
        Pat::PInt(n) => matches!(v, Val::Int(m) if m == n),
        Pat::PSym(s) => matches!(v, Val::Sym(t) if &**t == s.as_str()),
        Pat::PCtor(cn, sub_pats) => {
            if let Val::Ctor(vc, vargs) = v {
                if &**vc == cn.as_str() && sub_pats.len() == vargs.len() {
                    for (sp, sv) in sub_pats.iter().zip(vargs.iter()) {
                        if !match_pat(sp, sv, acc) {
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

// -----------------------------------------------------------------------------
// Value ⇄ Expr at the boundaries (final result; primitive table).
// -----------------------------------------------------------------------------

fn val_to_expr(v: &Val) -> Expr {
    match v {
        Val::Int(n) => Expr::IntLit(*n),
        Val::Sym(s) => Expr::SymLit(s.to_string()),
        Val::FVar(s) => Expr::FVar(s.to_string()),
        Val::Ctor(n, args) => {
            Expr::Ctor(n.to_string(), args.iter().map(val_to_expr).collect())
        }
    }
}

/// Convert a closed VALUE expr (as produced by `prim::try_apply`:
/// IntLit / SymLit / FVar / Ctor over values) into a `Val`.
fn val_of_value_expr(e: &Expr) -> Val {
    match e {
        Expr::IntLit(n) => Val::Int(*n),
        Expr::SymLit(s) => Val::Sym(Rc::from(s.as_str())),
        Expr::FVar(s) => Val::FVar(Rc::from(s.as_str())),
        Expr::Ctor(n, args) => Val::Ctor(
            Rc::from(n.as_str()),
            args.iter().map(val_of_value_expr).collect::<Vec<_>>().into(),
        ),
        // Primitives only ever return values; any other shape is a bug
        // in the primitive table, not reachable input.
        other => unreachable!("primitive returned a non-value expr: {other:?}"),
    }
}
