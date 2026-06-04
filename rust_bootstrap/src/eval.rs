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

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use smallvec::SmallVec;

use crate::ast::{Expr, FnDef, Module, Pat};
use crate::prim;

// Most environments and argument lists are tiny (1–5 entries), but each one
// used to be a heap-allocated Vec built and freed per reduction step — ~28% of
// runtime was that alloc/free churn (see profile). SmallVec keeps the common
// small case on the stack; it spills to the heap only for larger lists.
type Vals = SmallVec<[Val; 8]>;

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
    Effect(String),
    Unimplemented(&'static str),
}

// A run-time EFFECT HANDLER: performs real I/O for a stuck call to a declared
// `extern` symbol with no body. Installed ONLY while a program runs (see
// eval::set_effect_handler / bin/check.rs run_program); proof-checking never
// installs one, so during `check` an extern call falls through to UnknownCall
// here and is never reached — the proof reducer treats it as data, leaving it
// stuck/uninterpreted. So this hook is completely inert during checking and
// cannot affect soundness. The handler is a trusted boundary (docs/BOUNDARIES.md).
type EffectHandler = Box<dyn FnMut(&str, &[Expr]) -> Result<Expr, String>>;
thread_local! {
    static EFFECTS: RefCell<Option<EffectHandler>> = const { RefCell::new(None) };
}

/// Install (`Some`) or clear (`None`) the thread's run-time effect handler.
pub fn set_effect_handler(handler: Option<EffectHandler>) {
    EFFECTS.with(|e| *e.borrow_mut() = handler);
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
            EvalError::Effect(msg) => write!(f, "effect error: {msg}"),
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
    // BigInt clones are O(digits); Rc keeps Val's clone O(1) like the others.
    Int(Rc<crate::ast::IntLit>),
    Sym(Rc<str>),
    FVar(Rc<str>),
    Ctor(Rc<str>, Rc<[Val]>),
}

/// Reduce `e` to normal form within the context of `m`'s definitions.
pub fn eval(m: &Module, e: &Expr) -> Result<Expr, EvalError> {
    // Build a name → FnDef index ONCE. The bootstrapped kernel module has
    // hundreds of fns and every shard Call used to do a linear `lookup_fn`
    // scan over all of them — a fixed O(#fns) tax on every reduction step.
    // O(1) lookup makes the whole double-interpretation dramatically faster.
    let idx = build_fn_index(m);
    let v = eval_env(m, &idx, &[], e)?;
    Ok(val_to_expr(&v))
}

/// name → FnDef, first definition wins (matches `Module::lookup_fn`'s `.find`).
fn build_fn_index(m: &Module) -> HashMap<&str, &FnDef> {
    let mut idx: HashMap<&str, &FnDef> = HashMap::with_capacity(m.fns.len());
    for f in &m.fns {
        idx.entry(f.name.as_str()).or_insert(f);
    }
    idx
}

// Tail-call optimized: the iteration of a tail-recursive shard fn (e.g. the
// app reducer's `compute_expr` loop, or any direct-style loop) reduces to a
// `continue` here, NOT a Rust recursive call. Without this, a long shard
// reduction chain keeps one Rust stack frame per step alive — and each frame
// pins its `env`, so every intermediate term stays reachable → O(steps) stack
// AND heap. Sub-evaluations that are NOT in tail position (a Call's args, an
// If condition, a Match scrutinee, a Let's RHSs) still recurse, bounded by
// term depth. The four tail positions — user-fn body, taken If branch, fired
// Match arm, Let body — loop instead, freeing the prior frame (and its env, so
// the old term is dropped) before the next step.
fn eval_env<'a>(
    m: &'a Module,
    idx: &HashMap<&'a str, &'a FnDef>,
    env0: &[Val],
    e0: &'a Expr,
) -> Result<Val, EvalError> {
    let mut env: Vals = env0.iter().cloned().collect();
    let mut e: &'a Expr = e0;
    loop {
        match e {
            Expr::IntLit(n) => return Ok(Val::Int(Rc::new(n.clone()))),
            Expr::SymLit(s) => return Ok(Val::Sym(Rc::from(s.as_str()))),
            Expr::FVar(s) => return Ok(Val::FVar(Rc::from(s.as_str()))),

            // A bound variable indexes into the environment (innermost-first).
            Expr::BVar(k) => {
                return env
                    .get(*k as usize)
                    .cloned()
                    .ok_or(EvalError::UnboundBVar(*k))
            }

            Expr::Ctor(name, args) => {
                let vals = eval_args(m, idx, &env, args)?;
                return Ok(Val::Ctor(Rc::from(name.as_str()), Rc::from(vals.as_slice())));
            }

            Expr::Call(name, args) => {
                let vals = eval_args(m, idx, &env, args)?;
                // User-defined fn? TAIL-LOOP into its body in a fresh env of the
                // (reversed) argument values — the body is closed but for its
                // params. Primitives / externs are leaf operations: return.
                if let Some(fd) = idx.get(name.as_str()).copied() {
                    if fd.params.len() != vals.len() {
                        return Err(EvalError::ArityMismatch {
                            name: name.clone(),
                            expected: fd.params.len(),
                            got: vals.len(),
                        });
                    }
                    let mut next = vals;
                    next.reverse(); // env[0] = BVar 0 = LAST parameter
                    env = next;
                    e = &fd.body;
                    continue;
                }
                return apply_builtin(name, vals);
            }

            Expr::If(c, t, el) => match eval_env(m, idx, &env, c)? {
                Val::Ctor(ref n, ref a) if a.is_empty() && &**n == "True" => e = t,
                Val::Ctor(ref n, ref a) if a.is_empty() && &**n == "False" => e = el,
                other => return Err(EvalError::IfNonBool(format!("{:?}", val_to_expr(&other)))),
            },

            Expr::Match(scrut, arms) => {
                let v = eval_env(m, idx, &env, scrut)?;
                let mut next: Option<&'a Expr> = None;
                for arm in arms {
                    let mut binds: Vals = SmallVec::new();
                    if match_pat(&arm.pat, &v, &mut binds) {
                        // env' = bindings (innermost-first) ++ outer env.
                        binds.extend(env.iter().cloned());
                        env = binds;
                        next = Some(&arm.body);
                        break;
                    }
                }
                match next {
                    Some(body) => e = body,
                    None => return Err(EvalError::NoMatchArm(format!("{:?}", val_to_expr(&v)))),
                }
            }

            Expr::Let(rhss, body) => {
                // Parallel let: RHSs evaluated in the outer scope.
                let mut vals = eval_args(m, idx, &env, rhss)?;
                vals.reverse(); // innermost-first: last binding becomes BVar 0
                vals.extend(env.iter().cloned());
                env = vals;
                e = body;
            }
        }
    }
}

fn eval_args<'a>(
    m: &'a Module,
    idx: &HashMap<&'a str, &'a FnDef>,
    env: &[Val],
    args: &'a [Expr],
) -> Result<Vals, EvalError> {
    args.iter().map(|a| eval_env(m, idx, env, a)).collect()
}

// A Call whose head is NOT a user fn: a primitive, an effectful extern, or an
// unknown. Leaf operation (no tail position), so the eval loop returns its
// result directly. The user-fn case is handled inline in `eval_env` so it can
// tail-loop into the body.
fn apply_builtin(name: &str, args: Vals) -> Result<Val, EvalError> {
    // Primitive: cross to the `Expr`-typed primitive table at the
    // boundary (primitive arguments are small — ints, syms, or the
    // occasional char list, all O(arg)).
    let arg_exprs: Vec<Expr> = args.iter().map(val_to_expr).collect();
    if let Some(out) = prim::try_apply(name, &arg_exprs) {
        return Ok(val_of_value_expr(&out));
    }
    // Effectful extern: a registered run-time handler performs the real I/O
    // (read a line, write bytes, …) and returns the result value. Present only
    // during `run`; absent during `check`, where this point isn't reached for
    // externs (the proof reducer keeps them stuck as data). See EFFECTS above.
    if let Some(result) = EFFECTS.with(|e| {
        e.borrow_mut().as_mut().map(|h| h(name, &arg_exprs))
    }) {
        return result.map(|out| val_of_value_expr(&out)).map_err(EvalError::Effect);
    }
    Err(EvalError::UnknownCall(name.into()))
}

// -----------------------------------------------------------------------------
// Pattern matching. Mirrors kernel/reduce.shard:match_pat.
//
// Convention: bindings collected innermost-first. The LAST PVar
// encountered (rightmost in the pattern) becomes `BVar 0` in the arm
// body; earlier PVars get higher indices.
// -----------------------------------------------------------------------------

fn match_pat(p: &Pat, v: &Val, acc: &mut Vals) -> bool {
    match p {
        Pat::PVar => {
            // Prepend so acc[0] is the most recently captured (= BVar 0).
            acc.insert(0, v.clone());
            true
        }
        Pat::PInt(n) => matches!(v, Val::Int(m) if &**m == n),
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
        Val::Int(n) => Expr::IntLit((**n).clone()),
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
        Expr::IntLit(n) => Val::Int(Rc::new(n.clone())),
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
