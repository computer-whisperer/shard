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
//! environment is innermost-first, `BVar 0` = the most recently bound
//! value. It is kept on a binding STACK (see `Stacks` below); entering a
//! binder PUSHES its freshly-bound values, which reproduces the de
//! Bruijn shift (existing indices move up by the number of new binders)
//! without any renumbering. A user fn's body is closed except for its
//! parameters, so a call evaluates the body in a frame of just the
//! argument values (the last one on top).
//!
//! There are no lambdas in the narrow language (calls are saturated,
//! functions are top-level), so no closures are needed: a `Val` is
//! always a fully-evaluated, CLOSED term — note it has no `BVar`
//! variant, so a value structurally cannot carry a free index. That is
//! exactly the invariant the substitution machine relied on by hand.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{Expr, FnDef, Module, Pat};
use crate::prim;

// The ENVIRONMENT is ONE BINDING STACK shared by the whole evaluation,
// innermost-first from the top: `BVar 0` is the top of `fr`. Entering a
// binder PUSHES its freshly-bound values, which reproduces the de Bruijn
// shift with no renumbering and NO ALLOCATION. This replaces an Rc-consed
// list that heap-allocated one node per bound value — per argument, per
// pattern capture (wildcards included), per let binding — which `perf`
// showed as half the run time of a kernel replay (malloc/free = 49% of
// samples, ~5 nodes per dispatch).
//
// Values in flight — a call's arguments, a let's RHSs, a constructor's
// fields — are evaluated straight onto a second, OPERAND stack `op`, never
// onto `fr`: an argument is evaluated against the frame that is current
// when its call began, and a match or let inside it must find that frame
// exactly at the top of `fr`. (Pushing operands onto `fr` itself was tried
// and read outer bindings at the wrong offset.) A call then moves its
// arguments from `op` onto `fr` in one drain. Every `eval_ir` call records
// both heights at entry and restores them before returning, so a
// sub-evaluation leaves the caller's stacks exactly as it found them; a
// tail call truncates `fr` to its entry height first, dropping the frame it
// replaces. A fn body is closed but for its parameters, so an index never
// reaches below its own frame into the caller's; the bounds check in the
// BVar arm is the loud failure for a body that was never opened.
struct Stacks {
    fr: Vec<Val>,
    op: Vec<Val>,
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
// Read once; with profiling off a dispatch pays one relaxed load, not a
// thread-local borrow.
static PROF_ON: std::sync::LazyLock<bool> =
    std::sync::LazyLock::new(|| std::env::var("SHARD_PROF").is_ok());

#[inline]
fn prof_count(name: &str) {
    if !*PROF_ON {
        return;
    }
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
    if !*PROF_ON {
        return;
    }
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
            let top = std::env::var("SHARD_PROF_TOP").ok().and_then(|s| s.parse().ok()).unwrap_or(40);
            for (name, n) in v.iter().take(top) {
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

// Inside the machine an error is boxed: `Result<Val, Box<EvalError>>` is
// 40 bytes (Val's niche holds the discriminant) where the unboxed one was
// 56, and every sub-evaluation returns one. Errors are rare and terminal.
type EResult<T> = Result<T, Box<EvalError>>;

#[cold]
#[inline(never)]
fn fail<T>(e: EvalError) -> EResult<T> {
    Err(Box::new(e))
}

/// A fully-evaluated, closed value. Recursive children are `Rc`-shared
/// so cloning a value (variable lookup, pattern capture) is O(1) — the
/// shared-structure that makes this an environment machine rather than
/// a substitution machine.
#[derive(Clone)]
enum Val {
    /// An integer that fits a machine word — every index, code, hash and
    /// counter a checker computes. Unboxed: no allocation, no refcount.
    Int(i64),
    /// An integer that does NOT fit an i64 (never one that does: `int_val`
    /// is the only constructor, so `Int` and `Big` never compare equal and
    /// a `Big` is never zero). Rc keeps the clone O(1).
    Big(Rc<crate::ast::IntLit>),
    Sym(Rc<str>),
    FVar(Rc<str>),
    Ctor(Rc<str>, Rc<[Val]>),
}

/// The one way an arbitrary-precision result becomes a `Val`: canonical
/// (see `Val::Big`).
#[inline]
fn int_val(n: crate::ast::IntLit) -> Val {
    use num_traits::ToPrimitive;
    match n.to_i64() {
        Some(i) => Val::Int(i),
        None => Val::Big(Rc::new(n)),
    }
}

// -----------------------------------------------------------------------------
// LOWERED PROGRAM. The ast::Expr tree is lowered ONCE per `eval` into an IR
// where everything per-step-expensive is precomputed:
//   - integer literals are prebuilt `Val`s — evaluating one is a clone (a
//     word copy for an i64), where the AST walk cloned the BigInt every time;
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
    Int(Val),
    Sym(Rc<str>),
    FVar(Rc<str>),
    BVar(u32),
    /// A zero-argument constructor, built once at lowering: evaluating it
    /// is a clone (the general arm allocated an empty Rc slice per visit).
    Ctor0(Val),
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
    Band,
    Bor,
    Bxor,
    Bshl,
    Bshr,
    Mod,
    Other,
}

struct IArm {
    pat: IPat,
    body: IExpr,
}

enum IPat {
    Var,
    /// A literal pattern, prebuilt canonical (`int_val`), so the test is a
    /// word compare for an i64 and never confuses the two integer shapes.
    Int(Val),
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
    /// The interned Bool names, for the If test (pointer compares: every
    /// runtime name is canonical, see `intern`).
    true_name: Rc<str>,
    false_name: Rc<str>,
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
            Expr::IntLit(n) => IExpr::Int(int_val(n.clone())),
            Expr::SymLit(s) => IExpr::Sym(self.intern(s)),
            Expr::FVar(s) => IExpr::FVar(self.intern(s)),
            Expr::BVar(k) => IExpr::BVar(*k),
            // Nat former (kernel/stdlib.shard): a bare `Z` IS the literal 0
            // (the eval arm's packing rule, decided here once).
            Expr::Ctor(n, args) if args.is_empty() && n == "Z" => IExpr::Int(Val::Int(0)),
            Expr::Ctor(n, args) if args.is_empty() => {
                IExpr::Ctor0(Val::Ctor(self.intern(n), Rc::from([].as_slice())))
            }
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
                            "band" => PrimTag::Band,
                            "bor" => PrimTag::Bor,
                            "bxor" => PrimTag::Bxor,
                            "bshl" => PrimTag::Bshl,
                            "bshr" => PrimTag::Bshr,
                            "mod" => PrimTag::Mod,
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
            Pat::PInt(n) => IPat::Int(int_val(n.clone())),
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
    let true_name = intern("True");
    let false_name = intern("False");
    (Prog { fns, true_name, false_name }, ie)
}

/// Reduce `e` to normal form within the context of `m`'s definitions.
pub fn eval(m: &Module, e: &Expr) -> Result<Expr, EvalError> {
    let (prog, ie) = lower_program(m, e);
    let mut st = Stacks { fr: Vec::with_capacity(4096), op: Vec::with_capacity(1024) };
    let v = eval_ir(&prog, &mut st, &ie).map_err(|b| *b)?;
    Ok(val_to_expr(&v))
}

// Tail-call optimized: the iteration of a tail-recursive shard fn (e.g. the
// app reducer's `compute_expr` loop, or any direct-style loop) reduces to a
// `continue` here, NOT a Rust recursive call. Without this, a long shard
// reduction chain keeps one Rust stack frame per step alive — and each frame
// pins its bindings, so every intermediate term stays reachable → O(steps)
// stack AND heap. Sub-evaluations that are NOT in tail position (a Call's
// args, an If condition, a Match scrutinee, a Let's RHSs) still recurse,
// bounded by term depth. The four tail positions — user-fn body, taken If
// branch, fired Match arm, Let body — loop instead; a user-fn call truncates
// the binding stack to this invocation's entry height first, so the frame
// it replaces (and the terms only it reached) is dropped before the next
// step.
fn eval_ir<'a>(prog: &'a Prog, st: &mut Stacks, e0: &'a IExpr) -> EResult<Val> {
    let fb = st.fr.len();
    let ob = st.op.len();
    let r = eval_loop(prog, st, fb, e0);
    st.fr.truncate(fb);
    st.op.truncate(ob);
    r
}

/// Evaluate one sub-expression. A LEAF — a bound variable, a literal, a
/// prebuilt constructor — is answered here without entering the machine
/// (most arguments and scrutinees are variables); anything else recurses.
#[inline(always)]
fn eval_sub<'a>(prog: &'a Prog, st: &mut Stacks, e: &'a IExpr) -> EResult<Val> {
    match e {
        IExpr::BVar(k) => {
            let n = st.fr.len();
            let k = *k as usize;
            if k < n {
                Ok(st.fr[n - 1 - k].clone())
            } else {
                fail(EvalError::UnboundBVar(k as u32))
            }
        }
        IExpr::Int(v) => Ok(v.clone()),
        IExpr::Sym(s) => Ok(Val::Sym(s.clone())),
        IExpr::Ctor0(v) => Ok(v.clone()),
        _ => eval_ir(prog, st, e),
    }
}

/// Evaluate `args` left to right against the current frame, pushing each
/// value onto the operand stack; they end up at `st.op[mark..]` in order.
#[inline]
fn push_args<'a>(prog: &'a Prog, st: &mut Stacks, args: &'a [IExpr]) -> EResult<()> {
    for a in args {
        let v = eval_sub(prog, st, a)?;
        st.op.push(v);
    }
    Ok(())
}

fn eval_loop<'a>(
    prog: &'a Prog,
    st: &mut Stacks,
    base: usize,
    e0: &'a IExpr,
) -> EResult<Val> {
    let mut e: &'a IExpr = e0;
    loop {
        match e {
            IExpr::Int(v) => return Ok(v.clone()),
            IExpr::Sym(s) => return Ok(Val::Sym(s.clone())),
            IExpr::FVar(s) => return Ok(Val::FVar(s.clone())),
            IExpr::Ctor0(v) => return Ok(v.clone()),

            // A bound variable indexes the binding stack from its top.
            IExpr::BVar(k) => {
                let n = st.fr.len();
                let k = *k as usize;
                if k < n {
                    return Ok(st.fr[n - 1 - k].clone());
                }
                return fail(EvalError::UnboundBVar(k as u32));
            }

            IExpr::Ctor(name, args) => {
                let mark = st.op.len();
                push_args(prog, st, args)?;
                // Nat former (kernel/stdlib.shard): ground Z/S packs to its
                // nonneg literal — the unique ground Nat value (`Z` itself is
                // lowered to the literal). This engine is flat-core (names ARE
                // identity, cf. the True/False tests below), so the gate is the
                // bare name. A symbolic or negative argument never packs. (An
                // early return leaves the fields on the operand stack;
                // `eval_ir` pops them.)
                match &**name {
                    "S" if args.len() == 1 => match &st.op[mark] {
                        Val::Int(n) if *n >= 0 => {
                            return Ok(match n.checked_add(1) {
                                Some(m) => Val::Int(m),
                                None => int_val(crate::ast::IntLit::from(*n) + 1),
                            })
                        }
                        Val::Big(n) if !num_traits::Signed::is_negative(&**n) => {
                            return Ok(int_val((**n).clone() + 1))
                        }
                        _ => {}
                    },
                    _ => {}
                }
                // One allocation, the fields moved off the operand stack
                // (Drain is TrustedLen, so this is `from_iter_exact`).
                let fields: Rc<[Val]> = st.op.drain(mark..).collect();
                return Ok(Val::Ctor(name.clone(), fields));
            }

            IExpr::CallFn(i, args) => {
                // TAIL-LOOP into the fn body: the arguments are evaluated
                // against the current frame onto the operand stack; then the
                // frame this invocation owns is dropped and the arguments
                // move onto the binding stack as the callee's frame. In
                // order, so the LAST argument ends on top: BVar 0 = last
                // parameter.
                let fd = &prog.fns[*i as usize];
                prof_count(&fd.name);
                if fd.arity != args.len() {
                    return fail(EvalError::ArityMismatch {
                        name: fd.name.to_string(),
                        expected: fd.arity,
                        got: args.len(),
                    });
                }
                let mark = st.op.len();
                push_args(prog, st, args)?;
                st.fr.truncate(base);
                st.fr.extend(st.op.drain(mark..));
                e = &fd.body;
            }

            IExpr::CallOther(tag, name, args) => {
                let mark = st.op.len();
                push_args(prog, st, args)?;
                return apply_other(*tag, name, &st.op[mark..]);
            }

            IExpr::If(c, t, el) => match eval_sub(prog, st, c)? {
                Val::Ctor(ref n, ref a) if a.is_empty() && Rc::ptr_eq(n, &prog.true_name) => e = t,
                Val::Ctor(ref n, ref a) if a.is_empty() && Rc::ptr_eq(n, &prog.false_name) => e = el,
                other => return fail(EvalError::IfNonBool(format!("{:?}", val_to_expr(&other)))),
            },

            IExpr::Match(scrut, arms) => {
                let v = eval_sub(prog, st, scrut)?;
                let mut next: Option<&'a IExpr> = None;
                for arm in arms {
                    // Captures are pushed as they are matched, in capture
                    // order (leftmost PVar first), so the LAST capture ends
                    // on top (= BVar 0) above the outer bindings, which stay;
                    // a failed arm's partial captures are popped again.
                    let mark = st.fr.len();
                    if match_pat(&arm.pat, &v, &mut st.fr) {
                        next = Some(&arm.body);
                        break;
                    }
                    st.fr.truncate(mark);
                }
                match next {
                    Some(body) => e = body,
                    None => return fail(EvalError::NoMatchArm(format!("{:?}", val_to_expr(&v)))),
                }
            }

            IExpr::Let(rhss, body) => {
                // Parallel let: every RHS is evaluated against the OUTER
                // frame onto the operand stack before any is bound, so later
                // RHSs cannot see earlier bindings; moved over in order, the
                // LAST binding ends on top (= BVar 0).
                let mark = st.op.len();
                push_args(prog, st, rhss)?;
                st.fr.extend(st.op.drain(mark..));
                e = body;
            }
        }
    }
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

/// A shift amount the table accepts (`prim::shift_amount`): 0..64.
#[inline]
fn shift_ok(k: i64) -> bool {
    (0..64).contains(&k)
}

fn apply_other(tag: PrimTag, name: &str, args: &[Val]) -> EResult<Val> {
    // FAST PATH: the measured-hottest primitives on two machine integers,
    // dispatched by the PrimTag assigned at lowering (no strcmp). Each arm
    // computes on i64 with checked arithmetic and hands an overflow to the
    // table's own BigInt operator through `int_val`, so results are the
    // table's bit for bit. The guarded prims (`mod`'s b≠0, the shifts'
    // 0..64 amount) repeat their table guard as an arm guard, so a guard
    // failure falls through to the general path and stays stuck exactly as
    // before. Anything not two `Val::Int`s — a `Big` operand, a non-integer,
    // an untagged name — takes the general path below, which converts every
    // argument Val→Expr and the result back (allocating; the reason the hot
    // arms exist). The bitwise ops on i64 are exact for every i64 pair (the
    // table's BigInt treats negatives as two's complement); the right shift
    // keeps negatives on the table's operator.
    use crate::ast::IntLit as Big;
    match (tag, args) {
        (PrimTag::IntEq, [Val::Int(x), Val::Int(y)]) => {
            prof_count_prim("int_eq");
            return Ok(bool_val(x == y));
        }
        (PrimTag::Le, [Val::Int(x), Val::Int(y)]) => {
            prof_count_prim("le");
            return Ok(bool_val(x <= y));
        }
        (PrimTag::Lt, [Val::Int(x), Val::Int(y)]) => {
            prof_count_prim("lt");
            return Ok(bool_val(x < y));
        }
        (PrimTag::Add, [Val::Int(x), Val::Int(y)]) => {
            prof_count_prim("+");
            return Ok(match x.checked_add(*y) {
                Some(r) => Val::Int(r),
                None => int_val(Big::from(*x) + Big::from(*y)),
            });
        }
        (PrimTag::Sub, [Val::Int(x), Val::Int(y)]) => {
            prof_count_prim("-");
            return Ok(match x.checked_sub(*y) {
                Some(r) => Val::Int(r),
                None => int_val(Big::from(*x) - Big::from(*y)),
            });
        }
        (PrimTag::Mul, [Val::Int(x), Val::Int(y)]) => {
            prof_count_prim("*");
            return Ok(match x.checked_mul(*y) {
                Some(r) => Val::Int(r),
                None => int_val(Big::from(*x) * Big::from(*y)),
            });
        }
        (PrimTag::SymEq, [Val::Sym(a), Val::Sym(b)]) => {
            prof_count_prim("sym_eq");
            return Ok(bool_val(Rc::ptr_eq(a, b) || a == b));
        }
        (PrimTag::Band, [Val::Int(x), Val::Int(y)]) => {
            prof_count_prim("band");
            return Ok(Val::Int(x & y));
        }
        (PrimTag::Bor, [Val::Int(x), Val::Int(y)]) => {
            prof_count_prim("bor");
            return Ok(Val::Int(x | y));
        }
        (PrimTag::Bxor, [Val::Int(x), Val::Int(y)]) => {
            prof_count_prim("bxor");
            return Ok(Val::Int(x ^ y));
        }
        (PrimTag::Bshl, [Val::Int(x), Val::Int(k)]) if shift_ok(*k) => {
            prof_count_prim("bshl");
            // |x| < 2^63 and k < 64: the i128 product is exact.
            let r = (*x as i128) << *k;
            return Ok(match i64::try_from(r) {
                Ok(r) => Val::Int(r),
                Err(_) => int_val(Big::from(r)),
            });
        }
        (PrimTag::Bshr, [Val::Int(x), Val::Int(k)]) if *x >= 0 && shift_ok(*k) => {
            prof_count_prim("bshr");
            return Ok(Val::Int(x >> k));
        }
        (PrimTag::Bshr, [Val::Int(x), Val::Int(k)]) if shift_ok(*k) => {
            prof_count_prim("bshr");
            return Ok(int_val(Big::from(*x) >> (*k as u64)));
        }
        (PrimTag::Mod, [Val::Int(x), Val::Int(y)]) if *y != 0 => {
            prof_count_prim("mod");
            return Ok(match x.checked_rem_euclid(*y) {
                Some(r) => Val::Int(r),
                None => int_val(prim::rem_euclid(&Big::from(*x), &Big::from(*y))),
            });
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
        return result.map(|out| val_of_value_expr(&out)).map_err(|m| Box::new(EvalError::Effect(m)));
    }
    fail(EvalError::UnknownCall(name.into()))
}

// -----------------------------------------------------------------------------
// Pattern matching. Mirrors kernel/reduce.shard:match_pat.
//
// Convention: bindings are pushed onto the value stack in CAPTURE order
// (leftmost PVar first), which leaves the LAST (rightmost) PVar on top —
// i.e. `BVar 0` — matching the de Bruijn convention.
// -----------------------------------------------------------------------------

// Name tests are PURE POINTER COMPARES: every runtime name — lowered
// patterns and Ctor evals, bool_val's cached True/False, prim and
// effect-handler results via val_of_value_expr — goes through the one
// canonical interner, so string-equal names are Rc-identical by
// construction. The debug_assert pins that invariant in debug builds.
fn match_pat(p: &IPat, v: &Val, acc: &mut Vec<Val>) -> bool {
    match p {
        IPat::Var => {
            acc.push(v.clone());
            true
        }
        IPat::Int(n) => match (n, v) {
            (Val::Int(a), Val::Int(b)) => a == b,
            (Val::Big(a), Val::Big(b)) => **a == **b,
            _ => false,
        },
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
            match v {
                Val::Int(m) => match &**cn {
                    "Z" => return sub_pats.is_empty() && *m == 0,
                    "S" if sub_pats.len() == 1 => {
                        return *m > 0 && match_pat(&sub_pats[0], &Val::Int(m - 1), acc);
                    }
                    _ => {}
                },
                // A Big is never zero (canonical), so only S can match it.
                Val::Big(m) => match &**cn {
                    "Z" => return false,
                    "S" if sub_pats.len() == 1 => {
                        if num_traits::Signed::is_positive(&**m) {
                            return match_pat(&sub_pats[0], &int_val((**m).clone() - 1), acc);
                        }
                        return false;
                    }
                    _ => {}
                },
                _ => {}
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
        Val::Int(n) => Expr::IntLit(crate::ast::IntLit::from(*n)),
        Val::Big(n) => Expr::IntLit((**n).clone()),
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
        Expr::IntLit(n) => int_val(n.clone()),
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
