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

// The ENVIRONMENT is a persistent (Rc-shared) cons list, innermost-first:
// the head is `BVar 0`. Entering a binder CONSES the new bindings onto the
// shared tail in O(new bindings) — the flat-vector representation this
// replaces cloned the ENTIRE environment on every fired match arm
// (`binds.extend(env.iter().cloned())`), which the profile showed as the
// single largest block of host time (SmallVec extend + Rc churn + frees).
// Lookup walks `k` links; environments are shallow (parameters + enclosing
// match depth), so the walk is short where the clone was O(depth) always.
#[derive(Clone)]
struct EnvNode {
    v: Val,
    next: Env,
}
type Env = Option<Rc<EnvNode>>;

#[inline]
fn env_cons(v: Val, next: Env) -> Env {
    Some(Rc::new(EnvNode { v, next }))
}

#[inline]
fn env_lookup(env: &Env, k: u32) -> Option<&Val> {
    let mut cur = env;
    let mut k = k;
    while let Some(node) = cur {
        if k == 0 {
            return Some(&node.v);
        }
        k -= 1;
        cur = &node.next;
    }
    None
}

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

// ---- SHARD_PROF: opt-in shard-level call attribution (measurement only). ---
// With SHARD_PROF set, every user-fn dispatch and every primitive application
// is counted by name; `prof_dump` prints the top counts. This is the host's
// view of WHICH SHARD FNS the run consists of — when the program being run is
// eval.shard hosting an app, the counts attribute the engine's own cost
// (ev / match_val / trie_lookup / …), which no native profiler can see.
thread_local! {
    static PROF: RefCell<Option<HashMap<String, u64>>> =
        RefCell::new(std::env::var("SHARD_PROF").is_ok().then(HashMap::new));
}

#[inline]
fn prof_count(name: &str) {
    PROF.with(|p| {
        if let Some(map) = p.borrow_mut().as_mut() {
            *map.entry(name.to_string()).or_insert(0) += 1;
        }
    });
}

/// Like `prof_count` but prefixes "prim:" — and only builds the string when
/// profiling is actually enabled (a `format!` at the call site would allocate
/// on every primitive application even with SHARD_PROF off).
#[inline]
fn prof_count_prim(name: &str) {
    PROF.with(|p| {
        if let Some(map) = p.borrow_mut().as_mut() {
            *map.entry(format!("prim:{name}")).or_insert(0) += 1;
        }
    });
}

/// Print the SHARD_PROF counts (no-op unless SHARD_PROF is set).
pub fn prof_dump() {
    PROF.with(|p| {
        if let Some(map) = p.borrow().as_ref() {
            let mut v: Vec<_> = map.iter().collect();
            v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            let total: u64 = v.iter().map(|&(_, n)| n).sum();
            eprintln!("== SHARD_PROF: {total} dispatches ==");
            for (name, n) in v.iter().take(40) {
                eprintln!("{n:>14}  {name}");
            }
        }
    });
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

// -----------------------------------------------------------------------------
// LOWERED PROGRAM. The ast::Expr tree is lowered ONCE per `eval` into an IR
// where everything per-step-expensive is precomputed:
//   - integer literals are pre-boxed (`Rc<IntLit>`) — evaluating one is an Rc
//     clone, where the AST walk cloned the BigInt heap allocation every time;
//   - ctor / symbol names are INTERNED `Rc<str>` — evaluating a Ctor shares
//     the name instead of `Rc::from(&str)` (alloc + memcpy) per evaluation,
//     and pattern-match name tests are `Rc::ptr_eq` (same interner on both
//     sides) with a string fallback only for values built outside it;
//   - call heads are RESOLVED: a user fn becomes an index into the lowered
//     fn table (no per-call HashMap+SipHash lookup), and the measured-hot
//     primitives carry a PrimTag so dispatch is a jump, not a strcmp chain.
// First definition wins on duplicate fn names (matches the old
// `build_fn_index` / `Module::lookup_fn` semantics).
// -----------------------------------------------------------------------------

enum IExpr {
    Int(Rc<crate::ast::IntLit>),
    Sym(Rc<str>),
    FVar(Rc<str>),
    BVar(u32),
    Ctor(Rc<str>, Box<[IExpr]>),
    CallFn(u32, Box<[IExpr]>),
    CallOther(PrimTag, Rc<str>, Box<[IExpr]>),
    If(Box<IExpr>, Box<IExpr>, Box<IExpr>),
    Match(Box<IExpr>, Box<[IArm]>),
    Let(Box<[IExpr]>, Box<IExpr>),
}

#[derive(Clone, Copy)]
enum PrimTag {
    IntEq,
    Le,
    Lt,
    Add,
    Sub,
    Mul,
    SymEq,
    Other,
}

struct IArm {
    pat: IPat,
    body: IExpr,
}

enum IPat {
    Var,
    Int(crate::ast::IntLit),
    Sym(Rc<str>),
    Ctor(Rc<str>, Box<[IPat]>),
}

struct IFn {
    name: Rc<str>,
    arity: usize,
    body: IExpr,
}

struct Prog {
    fns: Vec<IFn>,
}

// The CANONICAL name interner — process-wide (per thread). EVERY runtime
// name `Rc<str>` is produced here: the lowerer (ctor/sym names + patterns),
// `val_of_value_expr` (primitive + effect-handler results), and the cached
// Bool values. That makes pointer identity COMPLETE for names: two equal
// name strings are always the same Rc, so pattern-match name tests are pure
// pointer compares with no string fallback (see `match_pat`).
thread_local! {
    static INTERN: RefCell<std::collections::HashSet<Rc<str>>> =
        RefCell::new(std::collections::HashSet::new());
}

fn intern(s: &str) -> Rc<str> {
    INTERN.with(|t| {
        let mut set = t.borrow_mut();
        match set.get(s) {
            Some(r) => r.clone(),
            None => {
                let r: Rc<str> = Rc::from(s);
                set.insert(r.clone());
                r
            }
        }
    })
}

struct Lowerer<'a> {
    names: HashMap<&'a str, Rc<str>>,
    fnidx: HashMap<&'a str, u32>,
}

impl<'a> Lowerer<'a> {
    fn intern(&mut self, s: &'a str) -> Rc<str> {
        // L1 cache over the global interner, keyed by the AST string slice.
        self.names.entry(s).or_insert_with(|| intern(s)).clone()
    }

    fn lower(&mut self, e: &'a Expr) -> IExpr {
        match e {
            Expr::IntLit(n) => IExpr::Int(Rc::new(n.clone())),
            Expr::SymLit(s) => IExpr::Sym(self.intern(s)),
            Expr::FVar(s) => IExpr::FVar(self.intern(s)),
            Expr::BVar(k) => IExpr::BVar(*k),
            Expr::Ctor(n, args) => IExpr::Ctor(self.intern(n), self.lower_list(args)),
            Expr::Call(n, args) => {
                let largs = self.lower_list(args);
                match self.fnidx.get(n.as_str()) {
                    Some(&i) => IExpr::CallFn(i, largs),
                    None => {
                        let tag = match n.as_str() {
                            "int_eq" => PrimTag::IntEq,
                            "le" => PrimTag::Le,
                            "lt" => PrimTag::Lt,
                            "+" => PrimTag::Add,
                            "-" => PrimTag::Sub,
                            "*" => PrimTag::Mul,
                            "sym_eq" => PrimTag::SymEq,
                            _ => PrimTag::Other,
                        };
                        IExpr::CallOther(tag, self.intern(n), largs)
                    }
                }
            }
            Expr::If(c, t, el) => IExpr::If(
                Box::new(self.lower(c)),
                Box::new(self.lower(t)),
                Box::new(self.lower(el)),
            ),
            Expr::Match(scrut, arms) => IExpr::Match(
                Box::new(self.lower(scrut)),
                arms.iter()
                    .map(|a| IArm { pat: self.lower_pat(&a.pat), body: self.lower(&a.body) })
                    .collect(),
            ),
            Expr::Let(rhss, body) => {
                IExpr::Let(self.lower_list(rhss), Box::new(self.lower(body)))
            }
        }
    }

    fn lower_list(&mut self, es: &'a [Expr]) -> Box<[IExpr]> {
        es.iter().map(|e| self.lower(e)).collect()
    }

    fn lower_pat(&mut self, p: &'a Pat) -> IPat {
        match p {
            Pat::PVar => IPat::Var,
            Pat::PInt(n) => IPat::Int(n.clone()),
            Pat::PSym(s) => IPat::Sym(self.intern(s)),
            Pat::PCtor(n, sub) => IPat::Ctor(
                self.intern(n),
                sub.iter().map(|sp| self.lower_pat(sp)).collect(),
            ),
        }
    }
}

/// Lower the module's fn table + the target expression. Fn indices are
/// assigned before any body is lowered, so recursive (and mutually
/// referencing) calls resolve.
fn lower_program<'a>(m: &'a Module, e: &'a Expr) -> (Prog, IExpr) {
    let mut lo = Lowerer { names: HashMap::new(), fnidx: HashMap::with_capacity(m.fns.len()) };
    let mut firsts: Vec<&'a FnDef> = Vec::with_capacity(m.fns.len());
    for f in &m.fns {
        if !lo.fnidx.contains_key(f.name.as_str()) {
            lo.fnidx.insert(f.name.as_str(), firsts.len() as u32);
            firsts.push(f);
        }
    }
    let fns = firsts
        .iter()
        .map(|f| IFn {
            name: lo.intern(&f.name),
            arity: f.params.len(),
            body: lo.lower(&f.body),
        })
        .collect();
    let ie = lo.lower(e);
    (Prog { fns }, ie)
}

/// Reduce `e` to normal form within the context of `m`'s definitions.
pub fn eval(m: &Module, e: &Expr) -> Result<Expr, EvalError> {
    let (prog, ie) = lower_program(m, e);
    let v = eval_ir(&prog, &None, &ie)?;
    Ok(val_to_expr(&v))
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
fn eval_ir<'a>(prog: &'a Prog, env0: &Env, e0: &'a IExpr) -> Result<Val, EvalError> {
    let mut env: Env = env0.clone(); // O(1): shares the spine
    let mut e: &'a IExpr = e0;
    loop {
        match e {
            IExpr::Int(n) => return Ok(Val::Int(n.clone())),
            IExpr::Sym(s) => return Ok(Val::Sym(s.clone())),
            IExpr::FVar(s) => return Ok(Val::FVar(s.clone())),

            // A bound variable indexes into the environment (innermost-first).
            IExpr::BVar(k) => {
                return env_lookup(&env, *k)
                    .cloned()
                    .ok_or(EvalError::UnboundBVar(*k))
            }

            IExpr::Ctor(name, args) => {
                let vals = eval_iargs(prog, &env, args)?;
                // Nat former (kernel/stdlib.shard): ground Z/S packs to its
                // nonneg literal — the unique ground Nat value. This engine is
                // flat-core (names ARE identity, cf. the True/False tests
                // below), so the gate is the bare name. A symbolic or negative
                // argument never packs.
                match &**name {
                    "Z" if vals.is_empty() => {
                        return Ok(Val::Int(Rc::new(num_traits::Zero::zero())))
                    }
                    "S" if vals.len() == 1 => {
                        if let Val::Int(n) = &vals[0] {
                            if !num_traits::Signed::is_negative(&**n) {
                                return Ok(Val::Int(Rc::new((**n).clone() + 1)));
                            }
                        }
                    }
                    _ => {}
                }
                return Ok(Val::Ctor(name.clone(), Rc::from(vals.as_slice())));
            }

            IExpr::CallFn(i, args) => {
                // TAIL-LOOP into the fn body in a fresh env of the argument
                // values — the body is closed but for its params. Arguments
                // evaluate left-to-right DIRECTLY into env nodes (no staging
                // vector); consing forward leaves the LAST argument at the
                // head, i.e. env[0] = BVar 0 = last parameter.
                let fd = &prog.fns[*i as usize];
                prof_count(&fd.name);
                if fd.arity != args.len() {
                    return Err(EvalError::ArityMismatch {
                        name: fd.name.to_string(),
                        expected: fd.arity,
                        got: args.len(),
                    });
                }
                let mut next: Env = None;
                for a in args.iter() {
                    next = env_cons(eval_ir(prog, &env, a)?, next);
                }
                env = next;
                e = &fd.body;
            }

            IExpr::CallOther(tag, name, args) => {
                let vals = eval_iargs(prog, &env, args)?;
                return apply_other(*tag, name, vals);
            }

            IExpr::If(c, t, el) => match eval_ir(prog, &env, c)? {
                Val::Ctor(ref n, ref a) if a.is_empty() && &**n == "True" => e = t,
                Val::Ctor(ref n, ref a) if a.is_empty() && &**n == "False" => e = el,
                other => return Err(EvalError::IfNonBool(format!("{:?}", val_to_expr(&other)))),
            },

            IExpr::Match(scrut, arms) => {
                let v = eval_ir(prog, &env, scrut)?;
                let mut next: Option<&'a IExpr> = None;
                for arm in arms {
                    let mut binds: Vals = SmallVec::new();
                    if match_pat(&arm.pat, &v, &mut binds) {
                        // env' = bindings ++ outer env, innermost-first.
                        // `binds` is in capture order (leftmost PVar first);
                        // consing forward leaves the LAST capture at the head
                        // (= BVar 0), and the outer env is SHARED, not cloned.
                        for b in binds {
                            env = env_cons(b, env);
                        }
                        next = Some(&arm.body);
                        break;
                    }
                }
                match next {
                    Some(body) => e = body,
                    None => return Err(EvalError::NoMatchArm(format!("{:?}", val_to_expr(&v)))),
                }
            }

            IExpr::Let(rhss, body) => {
                // Parallel let: RHSs evaluated in the OUTER scope (`env`),
                // consed onto a separate extension so later RHSs cannot see
                // earlier bindings; forward order leaves the LAST binding at
                // the head (= BVar 0).
                let mut next: Env = env.clone();
                for a in rhss.iter() {
                    next = env_cons(eval_ir(prog, &env, a)?, next);
                }
                env = next;
                e = body;
            }
        }
    }
}

fn eval_iargs<'a>(prog: &'a Prog, env: &Env, args: &'a [IExpr]) -> Result<Vals, EvalError> {
    let mut vals = Vals::with_capacity(args.len());
    for a in args {
        vals.push(eval_ir(prog, env, a)?);
    }
    Ok(vals)
}

// A Call whose head is NOT a user fn: a primitive, an effectful extern, or an
// unknown. Leaf operation (no tail position), so the eval loop returns its
// result directly. The user-fn case is handled inline in `eval_ir` so it can
// tail-loop into the body.
thread_local! {
    // The two Bool values, built once: the general path allocated a fresh
    // `Rc<str>` ctor name per comparison result (millions per checking run).
    // Names go through the canonical interner so they ptr-match patterns.
    static TRUE_V: Val = Val::Ctor(intern("True"), Rc::from([].as_slice()));
    static FALSE_V: Val = Val::Ctor(intern("False"), Rc::from([].as_slice()));
}

#[inline]
fn bool_val(b: bool) -> Val {
    if b { TRUE_V.with(Val::clone) } else { FALSE_V.with(Val::clone) }
}

// Small-integer Val cache. Checker workloads are dominated by small ints
// (char codes, indices, de Bruijn arithmetic); without this every `+`/`-`
// result allocates a fresh BigInt + Rc. Covers [0, 1024].
thread_local! {
    static SMALL_INTS: Vec<Val> = (0..=1024)
        .map(|k| Val::Int(Rc::new(crate::ast::IntLit::from(k))))
        .collect();
}

#[inline]
fn int_val_i64(n: i64) -> Val {
    if (0..=1024).contains(&n) {
        SMALL_INTS.with(|v| v[n as usize].clone())
    } else {
        Val::Int(Rc::new(crate::ast::IntLit::from(n)))
    }
}

/// i64 view of both operands, when they fit (the overwhelmingly common case).
#[inline]
fn both_i64(a: &crate::ast::IntLit, b: &crate::ast::IntLit) -> Option<(i64, i64)> {
    use num_traits::ToPrimitive;
    Some((a.to_i64()?, b.to_i64()?))
}

fn apply_other(tag: PrimTag, name: &str, args: Vals) -> Result<Val, EvalError> {
    // FAST PATH: the measured-hottest primitives, applied directly on `Val`s
    // and dispatched by the PrimTag assigned at lowering (no strcmp). The
    // general path below converts every argument Val→Expr (allocating) and
    // the result back — pure overhead for these. Arms mirror prim.rs exactly
    // (same value shapes, BigInt comparison semantics); only UNGUARDED prims
    // are tagged (the division family keeps its b=0 stuck-guard in the
    // table). Non-matching shapes fall through to the general path, which
    // reproduces the old behavior bit for bit.
    match (tag, args.as_slice()) {
        (PrimTag::IntEq, [Val::Int(a), Val::Int(b)]) => {
            prof_count_prim("int_eq");
            return Ok(bool_val(a == b));
        }
        (PrimTag::Le, [Val::Int(a), Val::Int(b)]) => {
            prof_count_prim("le");
            return Ok(bool_val(a <= b));
        }
        (PrimTag::Lt, [Val::Int(a), Val::Int(b)]) => {
            prof_count_prim("lt");
            return Ok(bool_val(a < b));
        }
        (PrimTag::Add, [Val::Int(a), Val::Int(b)]) => {
            prof_count_prim("+");
            // i64 fast path with checked math: overflow (or a true BigInt
            // operand) falls back to BigInt — results are identical, BigInt
            // is the semantics either way.
            if let Some(r) = both_i64(a, b).and_then(|(x, y)| x.checked_add(y)) {
                return Ok(int_val_i64(r));
            }
            return Ok(Val::Int(Rc::new(&**a + &**b)));
        }
        (PrimTag::Sub, [Val::Int(a), Val::Int(b)]) => {
            prof_count_prim("-");
            if let Some(r) = both_i64(a, b).and_then(|(x, y)| x.checked_sub(y)) {
                return Ok(int_val_i64(r));
            }
            return Ok(Val::Int(Rc::new(&**a - &**b)));
        }
        (PrimTag::Mul, [Val::Int(a), Val::Int(b)]) => {
            prof_count_prim("*");
            if let Some(r) = both_i64(a, b).and_then(|(x, y)| x.checked_mul(y)) {
                return Ok(int_val_i64(r));
            }
            return Ok(Val::Int(Rc::new(&**a * &**b)));
        }
        (PrimTag::SymEq, [Val::Sym(a), Val::Sym(b)]) => {
            prof_count_prim("sym_eq");
            return Ok(bool_val(Rc::ptr_eq(a, b) || a == b));
        }
        _ => {}
    }
    // Primitive: cross to the `Expr`-typed primitive table at the
    // boundary (primitive arguments are small — ints, syms, or the
    // occasional char list, all O(arg)).
    let arg_exprs: Vec<Expr> = args.iter().map(val_to_expr).collect();
    if let Some(out) = prim::try_apply(name, &arg_exprs) {
        prof_count_prim(name);
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
// Convention: bindings collected in CAPTURE order (leftmost PVar first).
// The caller conses them onto the environment forward, which leaves the
// LAST (rightmost) PVar at the head — i.e. `BVar 0` — matching the de
// Bruijn convention. (The old flat-vector env wanted innermost-first and
// paid an O(n) front-insert per capture to get it.)
// -----------------------------------------------------------------------------

// Name tests are PURE POINTER COMPARES: every runtime name — lowered
// patterns and Ctor evals, bool_val's cached True/False, prim and
// effect-handler results via val_of_value_expr — goes through the one
// canonical interner, so string-equal names are Rc-identical by
// construction. The debug_assert pins that invariant in debug builds.
fn match_pat(p: &IPat, v: &Val, acc: &mut Vals) -> bool {
    match p {
        IPat::Var => {
            acc.push(v.clone());
            true
        }
        IPat::Int(n) => matches!(v, Val::Int(m) if &**m == n),
        IPat::Sym(s) => match v {
            Val::Sym(t) => {
                debug_assert!(Rc::ptr_eq(t, s) == (**t == **s), "non-canonical Sym name");
                Rc::ptr_eq(t, s)
            }
            _ => false,
        },
        IPat::Ctor(cn, sub_pats) => {
            if let Val::Ctor(vc, vargs) = v {
                debug_assert!(Rc::ptr_eq(vc, cn) == (**vc == **cn), "non-canonical Ctor name");
                if Rc::ptr_eq(vc, cn) && sub_pats.len() == vargs.len() {
                    for (sp, sv) in sub_pats.iter().zip(vargs.iter()) {
                        if !match_pat(sp, sv, acc) {
                            return false;
                        }
                    }
                    return true;
                }
            }
            // Nat former VIEW: a nonneg literal IS a ground Nat, so it
            // matches Z/S structurally (0 is Z, n>=1 is (S (n-1)), recursing
            // for deep patterns). Negatives are ill-typed garbage: no match
            // (this closed-world engine reports NoMatchArm, loudly).
            if let Val::Int(m) = v {
                match &**cn {
                    "Z" => return sub_pats.is_empty() && num_traits::Zero::is_zero(&**m),
                    "S" if sub_pats.len() == 1 => {
                        if num_traits::Signed::is_positive(&**m) {
                            return match_pat(
                                &sub_pats[0],
                                &Val::Int(Rc::new((**m).clone() - 1)),
                                acc,
                            );
                        }
                        return false;
                    }
                    _ => {}
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
        // Names are INTERNED — pattern matching relies on every runtime name
        // being canonical (pointer compares, no string fallback).
        Expr::SymLit(s) => Val::Sym(intern(s)),
        Expr::FVar(s) => Val::FVar(intern(s)),
        Expr::Ctor(n, args) => Val::Ctor(
            intern(n),
            args.iter().map(val_of_value_expr).collect::<Vec<_>>().into(),
        ),
        // Primitives only ever return values; any other shape is a bug
        // in the primitive table, not reachable input.
        other => unreachable!("primitive returned a non-value expr: {other:?}"),
    }
}
