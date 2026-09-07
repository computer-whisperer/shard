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
//! Binding convention (unchanged at the loader, locally-nameless / de
//! Bruijn): `BVar 0` = the most recently bound value. Values live on ONE
//! STACK (see `Stack` below); entering a binder PUSHES its freshly-bound
//! values, which reproduces the de Bruijn shift without any renumbering,
//! and the lowerer turns each index into the physical slot past the
//! temporaries in flight. A user fn's body is closed except for its
//! parameters, so a call evaluates the body in a frame of just the
//! argument values (the last one on top).
//!
//! There are no lambdas in the narrow language (calls are saturated,
//! functions are top-level), so no closures are needed: a `Val` is
//! always a fully-evaluated, CLOSED term — note it has no `BVar`
//! variant, so a value structurally cannot carry a free index. That is
//! exactly the invariant the substitution machine relied on by hand.

use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ptr::NonNull;
use std::rc::Rc;

use crate::ast::{Expr, FnDef, Module, Pat};
use crate::prim;

// The ENVIRONMENT is ONE VALUE STACK shared by the whole evaluation.
// Entering a binder PUSHES its freshly-bound values, and values in flight
// — a call's arguments, a let's RHSs, a constructor's fields — are pushed
// onto the same stack as they are computed: NO ALLOCATION per binding (an
// Rc-consed list here was half the run time of a kernel replay: malloc/free
// = 49% of samples, ~5 nodes per dispatch) and NO MOVE per call (a separate
// operand stack, tried next, spent 8% moving arguments onto the frame).
// The loader's de Bruijn index counts BINDINGS only, so the lowerer turns
// it into a PHYSICAL slot from the top — the binding's distance plus the
// temporaries in flight above it at that point in the expression, known
// statically (see `Lowerer::layout`). Every `eval_ir` call records the
// stack height at entry and restores it before returning, so a
// sub-evaluation leaves the caller's stack exactly as it found it; a tail
// call drains the frame this invocation owns out from under the arguments
// it just pushed (they slide down into place; nothing moves when the
// invocation owns no frame yet, the common non-tail call). A fn body is
// closed but for its parameters, so a slot never reaches below its own
// frame into the caller's; the bounds check in the BVar arm is the loud
// failure for a body that was never opened.
type Stack = Vec<Val>;

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

/// A fully-evaluated, closed value, SIXTEEN BYTES: a tag and one word.
/// Every result of the machine is one of these, so it comes back in two
/// registers and goes onto the stack with one store (at 40 bytes — two
/// fat pointers in the constructor arm — the moves through returns and
/// pushes were a quarter of the samples). Recursive children are
/// refcount-shared so cloning a value (variable lookup, pattern capture)
/// is O(1) — the shared structure that makes this an environment machine
/// rather than a substitution machine.
#[derive(Clone)]
enum Val {
    /// An integer that fits a machine word — every index, code, hash and
    /// counter a checker computes. Unboxed: no allocation, no refcount.
    Int(i64),
    /// An integer that does NOT fit an i64 (never one that does: `int_val`
    /// is the only constructor, so `Int` and `Big` never compare equal and
    /// a `Big` is never zero). Rc keeps the clone O(1).
    Big(Rc<crate::ast::IntLit>),
    /// An interned name id (see `intern`): equal names are equal ids.
    Sym(u32),
    FVar(u32),
    /// A constructor application behind one thin pointer (see `CtorRef`).
    Ctor(CtorRef),
}

const _: () = assert!(std::mem::size_of::<Val>() == 16);

/// A constructor value: ONE allocation holding a header (refcount, name
/// id, field count) with the fields inline after it, reached through a
/// thin pointer. `Rc<[Val]>` would be a fat pointer (and the name a second
/// one), which is what kept `Val` at 40 bytes. Non-atomic refcount, like
/// `Rc`; `!Send` like `Rc` (NonNull). Dropping the last reference drops
/// the fields in order and frees the block.
struct CtorRef {
    ptr: NonNull<CtorHeader>,
}

#[repr(C)]
struct CtorHeader {
    rc: Cell<usize>,
    name: u32,
    len: u32,
}

impl CtorRef {
    fn layout(len: usize) -> Layout {
        let size = std::mem::size_of::<CtorHeader>() + len * std::mem::size_of::<Val>();
        let align = std::mem::align_of::<CtorHeader>().max(std::mem::align_of::<Val>());
        Layout::from_size_align(size, align).expect("constructor layout")
    }

    /// The header is 16 bytes and 8-aligned, so the fields start right
    /// after it at `Val`'s alignment.
    #[inline]
    fn fields_ptr(&self) -> *mut Val {
        unsafe { self.ptr.as_ptr().add(1) as *mut Val }
    }

    fn new<I: ExactSizeIterator<Item = Val>>(name: u32, fields: I) -> CtorRef {
        let len = fields.len();
        let layout = Self::layout(len);
        unsafe {
            let p = alloc(layout) as *mut CtorHeader;
            if p.is_null() {
                handle_alloc_error(layout);
            }
            p.write(CtorHeader { rc: Cell::new(1), name, len: len as u32 });
            let f = p.add(1) as *mut Val;
            let mut i = 0;
            for v in fields {
                assert!(i < len, "constructor field iterator overran its length");
                f.add(i).write(v);
                i += 1;
            }
            assert!(i == len, "constructor field iterator ended short");
            CtorRef { ptr: NonNull::new_unchecked(p) }
        }
    }

    #[inline]
    fn header(&self) -> &CtorHeader {
        unsafe { self.ptr.as_ref() }
    }

    #[inline]
    fn name(&self) -> u32 {
        self.header().name
    }

    #[inline]
    fn len(&self) -> usize {
        self.header().len as usize
    }

    #[inline]
    fn fields(&self) -> &[Val] {
        unsafe { std::slice::from_raw_parts(self.fields_ptr(), self.len()) }
    }
}

impl Clone for CtorRef {
    #[inline]
    fn clone(&self) -> CtorRef {
        let h = self.header();
        h.rc.set(h.rc.get() + 1);
        CtorRef { ptr: self.ptr }
    }
}

impl Drop for CtorRef {
    #[inline]
    fn drop(&mut self) {
        let h = self.header();
        let rc = h.rc.get() - 1;
        h.rc.set(rc);
        if rc == 0 {
            let len = self.len();
            unsafe {
                let f = self.fields_ptr();
                for i in 0..len {
                    std::ptr::drop_in_place(f.add(i));
                }
                dealloc(self.ptr.as_ptr() as *mut u8, Self::layout(len));
            }
        }
    }
}

#[inline]
fn ctor0(name: u32) -> Val {
    Val::Ctor(CtorRef::new(name, std::iter::empty()))
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
//   - ctor / symbol names are INTERNED ids (`intern`) — evaluating a Ctor
//     or a symbol carries a u32, and every pattern-match name test is an
//     integer compare;
//   - call heads are RESOLVED: a user fn becomes an index into the lowered
//     fn table (no per-call HashMap+SipHash lookup), and the measured-hot
//     primitives carry a PrimTag so dispatch is a jump, not a strcmp chain.
// First definition wins on duplicate fn names (matches the old
// `build_fn_index` / `Module::lookup_fn` semantics).
// -----------------------------------------------------------------------------

enum IExpr {
    Int(Val),
    Sym(u32),
    FVar(u32),
    BVar(u32),
    /// A zero-argument constructor, built once at lowering: evaluating it
    /// is a clone (the general arm allocated an empty Rc slice per visit).
    Ctor0(Val),
    Ctor(u32, Box<[IExpr]>),
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
    SymOfChars,
    CharsOfSym,
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
    Sym(u32),
    /// A constructor pattern; the flag says every sub-pattern is a variable
    /// (the common shape), so a match binds the fields with one slice copy.
    Ctor(u32, Box<[IPat]>, bool),
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
// name is an id from here: the lowerer (ctor/sym names + patterns),
// `val_of_value_expr` (primitive + effect-handler results), and the cached
// Bool values. Equal name strings are the same id, so every name test in
// the machine is an integer compare. The six names the machine itself
// knows are interned first, at fixed ids.
struct Names {
    ids: HashMap<Rc<str>, u32>,
    strs: Vec<Rc<str>>,
}

const TRUE_ID: u32 = 0;
const FALSE_ID: u32 = 1;
const Z_ID: u32 = 2;
const S_ID: u32 = 3;
const CONS_ID: u32 = 4;
const NIL_ID: u32 = 5;

impl Names {
    fn new() -> Names {
        let mut n = Names { ids: HashMap::new(), strs: Vec::new() };
        for (i, s) in ["True", "False", "Z", "S", "Cons", "Nil"].iter().enumerate() {
            assert_eq!(n.intern(s), i as u32);
        }
        n
    }

    fn intern(&mut self, s: &str) -> u32 {
        if let Some(&i) = self.ids.get(s) {
            return i;
        }
        let r: Rc<str> = Rc::from(s);
        let i = self.strs.len() as u32;
        self.strs.push(r.clone());
        self.ids.insert(r, i);
        i
    }
}

thread_local! {
    static NAMES: RefCell<Names> = RefCell::new(Names::new());
}

fn intern(s: &str) -> u32 {
    NAMES.with(|n| n.borrow_mut().intern(s))
}

fn name_str(id: u32) -> Rc<str> {
    NAMES.with(|n| n.borrow().strs[id as usize].clone())
}

struct Lowerer<'a> {
    names: HashMap<&'a str, u32>,
    fnidx: HashMap<&'a str, u32>,
    /// The stack layout at the point being lowered, bottom to top: `true`
    /// for a binding (a parameter, a pattern capture, a let binding),
    /// `false` for a temporary in flight (an argument already computed for
    /// the call being lowered). A de Bruijn index counts bindings only;
    /// the physical slot counts both.
    layout: Vec<bool>,
}

/// Captures a pattern introduces (the loader binds one name per `PVar`,
/// left to right).
fn count_pvars(p: &Pat) -> usize {
    match p {
        Pat::PVar => 1,
        Pat::PCtor(_, sub) => sub.iter().map(count_pvars).sum(),
        Pat::PInt(_) | Pat::PSym(_) => 0,
    }
}

impl<'a> Lowerer<'a> {
    fn intern(&mut self, s: &'a str) -> u32 {
        // L1 cache over the global interner, keyed by the AST string slice.
        *self.names.entry(s).or_insert_with(|| intern(s))
    }

    /// The physical slot (from the top of the stack) of the `k`-th binding
    /// from the top, skipping the temporaries in flight. An index the
    /// layout does not reach is a loader bug; it becomes an out-of-range
    /// slot, which the machine reports as UnboundBVar.
    fn slot(&self, k: u32) -> u32 {
        let mut left = k;
        for (p, is_binding) in self.layout.iter().rev().enumerate() {
            if *is_binding {
                if left == 0 {
                    return p as u32;
                }
                left -= 1;
            }
        }
        u32::MAX
    }

    fn lower(&mut self, e: &'a Expr) -> IExpr {
        match e {
            Expr::IntLit(n) => IExpr::Int(int_val(n.clone())),
            Expr::SymLit(s) => IExpr::Sym(self.intern(s)),
            Expr::FVar(s) => IExpr::FVar(self.intern(s)),
            Expr::BVar(k) => IExpr::BVar(self.slot(*k)),
            // Nat former (kernel/stdlib.shard): a bare `Z` IS the literal 0
            // (the eval arm's packing rule, decided here once).
            Expr::Ctor(n, args) if args.is_empty() && n == "Z" => IExpr::Int(Val::Int(0)),
            Expr::Ctor(n, args) if args.is_empty() => IExpr::Ctor0(ctor0(self.intern(n))),
            Expr::Ctor(n, args) => IExpr::Ctor(self.intern(n), self.lower_args(args)),
            Expr::Call(n, args) => {
                let largs = self.lower_args(args);
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
                            "sym_of_chars" => PrimTag::SymOfChars,
                            "chars_of_sym" => PrimTag::CharsOfSym,
                            _ => PrimTag::Other,
                        };
                        IExpr::CallOther(tag, Rc::from(n.as_str()), largs)
                    }
                }
            }
            Expr::If(c, t, el) => IExpr::If(
                Box::new(self.lower(c)),
                Box::new(self.lower(t)),
                Box::new(self.lower(el)),
            ),
            // The scrutinee's and the condition's values go to a Rust local,
            // never onto the stack: the arms see the outer layout, plus the
            // captures a fired arm pushes.
            Expr::Match(scrut, arms) => {
                let scrut = Box::new(self.lower(scrut));
                let arms = arms
                    .iter()
                    .map(|a| {
                        let pat = self.lower_pat(&a.pat);
                        let n = count_pvars(&a.pat);
                        let depth = self.layout.len();
                        self.layout.resize(depth + n, true);
                        let body = self.lower(&a.body);
                        self.layout.truncate(depth);
                        IArm { pat, body }
                    })
                    .collect();
                IExpr::Match(scrut, arms)
            }
            // Parallel let: the RHSs are computed in flight (each sees the
            // outer bindings past the earlier RHSs), then BECOME the
            // bindings in place — the same slots, relabelled.
            Expr::Let(rhss, body) => {
                let rhs = self.lower_args(rhss);
                let depth = self.layout.len();
                self.layout.resize(depth + rhss.len(), true);
                let body = Box::new(self.lower(body));
                self.layout.truncate(depth);
                IExpr::Let(rhs, body)
            }
        }
    }

    /// Lower a list of values computed in flight, left to right: each is
    /// lowered with the earlier ones already on the stack as temporaries.
    fn lower_args(&mut self, es: &'a [Expr]) -> Box<[IExpr]> {
        let depth = self.layout.len();
        let out = es
            .iter()
            .map(|e| {
                let ie = self.lower(e);
                self.layout.push(false);
                ie
            })
            .collect();
        self.layout.truncate(depth);
        out
    }

    fn lower_pat(&mut self, p: &'a Pat) -> IPat {
        match p {
            Pat::PVar => IPat::Var,
            Pat::PInt(n) => IPat::Int(int_val(n.clone())),
            Pat::PSym(s) => IPat::Sym(self.intern(s)),
            Pat::PCtor(n, sub) => {
                let flat = sub.iter().all(|sp| matches!(sp, Pat::PVar));
                IPat::Ctor(
                    self.intern(n),
                    sub.iter().map(|sp| self.lower_pat(sp)).collect(),
                    flat,
                )
            }
        }
    }
}

/// Lower the module's fn table + the target expression. Fn indices are
/// assigned before any body is lowered, so recursive (and mutually
/// referencing) calls resolve.
fn lower_program<'a>(m: &'a Module, e: &'a Expr) -> (Prog, IExpr) {
    let mut lo = Lowerer {
        names: HashMap::new(),
        fnidx: HashMap::with_capacity(m.fns.len()),
        layout: Vec::new(),
    };
    let mut firsts: Vec<&'a FnDef> = Vec::with_capacity(m.fns.len());
    for f in &m.fns {
        if !lo.fnidx.contains_key(f.name.as_str()) {
            lo.fnidx.insert(f.name.as_str(), firsts.len() as u32);
            firsts.push(f);
        }
    }
    let fns = firsts
        .iter()
        .map(|f| {
            // A body's frame is its parameters, the last one on top.
            lo.layout.clear();
            lo.layout.resize(f.params.len(), true);
            IFn { name: Rc::from(f.name.as_str()), arity: f.params.len(), body: lo.lower(&f.body) }
        })
        .collect();
    lo.layout.clear();
    let ie = lo.lower(e);
    (Prog { fns }, ie)
}

/// Reduce `e` to normal form within the context of `m`'s definitions.
pub fn eval(m: &Module, e: &Expr) -> Result<Expr, EvalError> {
    let (prog, ie) = lower_program(m, e);
    let mut st: Stack = Vec::with_capacity(4096);
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
fn eval_ir<'a>(prog: &'a Prog, st: &mut Stack, e0: &'a IExpr) -> EResult<Val> {
    let base = st.len();
    let r = eval_loop(prog, st, base, e0);
    st.truncate(base);
    r
}

/// Evaluate one sub-expression. A LEAF — a bound variable, a literal, a
/// prebuilt constructor — is answered here without entering the machine
/// (most arguments and scrutinees are variables); anything else recurses.
#[inline(always)]
fn eval_sub<'a>(prog: &'a Prog, st: &mut Stack, e: &'a IExpr) -> EResult<Val> {
    match e {
        IExpr::BVar(k) => {
            let n = st.len();
            let k = *k as usize;
            if k < n {
                Ok(st[n - 1 - k].clone())
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

/// Evaluate `args` left to right, pushing each value; they end up at
/// `st[mark..]` in order (the LAST one on top). Each argument's slots were
/// lowered with the earlier ones counted as temporaries above the frame.
#[inline]
fn push_args<'a>(prog: &'a Prog, st: &mut Stack, args: &'a [IExpr]) -> EResult<()> {
    for a in args {
        let v = eval_sub(prog, st, a)?;
        st.push(v);
    }
    Ok(())
}

fn eval_loop<'a>(prog: &'a Prog, st: &mut Stack, base: usize, e0: &'a IExpr) -> EResult<Val> {
    let mut e: &'a IExpr = e0;
    loop {
        match e {
            IExpr::Int(v) => return Ok(v.clone()),
            IExpr::Sym(s) => return Ok(Val::Sym(s.clone())),
            IExpr::FVar(s) => return Ok(Val::FVar(s.clone())),
            IExpr::Ctor0(v) => return Ok(v.clone()),

            // A bound variable's slot, from the top of the stack.
            IExpr::BVar(k) => {
                let n = st.len();
                let k = *k as usize;
                if k < n {
                    return Ok(st[n - 1 - k].clone());
                }
                return fail(EvalError::UnboundBVar(k as u32));
            }

            IExpr::Ctor(name, args) => {
                let mark = st.len();
                push_args(prog, st, args)?;
                // Nat former (kernel/stdlib.shard): ground Z/S packs to its
                // nonneg literal — the unique ground Nat value (`Z` itself is
                // lowered to the literal). This engine is flat-core (names ARE
                // identity, cf. the True/False tests below), so the gate is the
                // bare name. A symbolic or negative argument never packs. (An
                // early return leaves the field on the stack; `eval_ir` pops
                // it.)
                if *name == S_ID && args.len() == 1 {
                    match &st[mark] {
                        Val::Int(n) if *n >= 0 => {
                            return Ok(match n.checked_add(1) {
                                Some(m) => Val::Int(m),
                                None => int_val(crate::ast::IntLit::from(*n) + 1),
                            });
                        }
                        Val::Big(n) if !num_traits::Signed::is_negative(&**n) => {
                            return Ok(int_val((**n).clone() + 1));
                        }
                        _ => {}
                    }
                }
                // One allocation, the fields moved off the stack.
                return Ok(Val::Ctor(CtorRef::new(*name, st.drain(mark..))));
            }

            IExpr::CallFn(i, args) => {
                // TAIL-LOOP into the fn body: the arguments are pushed above
                // whatever this invocation owns (its current frame, captures,
                // let bindings — all dead once the call is made), which is
                // then drained out from under them; they slide down to become
                // the callee's frame, the LAST argument on top: BVar 0 = last
                // parameter. A fresh invocation owns nothing yet, so its first
                // call moves nothing.
                let fd = &prog.fns[*i as usize];
                prof_count(&fd.name);
                if fd.arity != args.len() {
                    return fail(EvalError::ArityMismatch {
                        name: fd.name.to_string(),
                        expected: fd.arity,
                        got: args.len(),
                    });
                }
                let mark = st.len();
                push_args(prog, st, args)?;
                if mark > base {
                    st.drain(base..mark);
                }
                e = &fd.body;
            }

            IExpr::CallOther(tag, name, args) => {
                let mark = st.len();
                push_args(prog, st, args)?;
                return apply_other(*tag, name, &st[mark..]);
            }

            IExpr::If(c, t, el) => match eval_sub(prog, st, c)? {
                Val::Ctor(ref c) if c.len() == 0 && c.name() == TRUE_ID => e = t,
                Val::Ctor(ref c) if c.len() == 0 && c.name() == FALSE_ID => e = el,
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
                    let mark = st.len();
                    if match_pat(&arm.pat, &v, st) {
                        next = Some(&arm.body);
                        break;
                    }
                    st.truncate(mark);
                }
                match next {
                    Some(body) => e = body,
                    None => return fail(EvalError::NoMatchArm(format!("{:?}", val_to_expr(&v)))),
                }
            }

            IExpr::Let(rhss, body) => {
                // Parallel let: every RHS is computed against the outer
                // bindings (the lowerer counted the earlier RHSs as
                // temporaries), and the values pushed ARE the bindings, the
                // LAST one on top (= BVar 0).
                push_args(prog, st, rhss)?;
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
    // The two Bool values, built once (the general path builds a fresh
    // constructor block per comparison result — millions per checking run).
    static TRUE_V: Val = ctor0(TRUE_ID);
    static FALSE_V: Val = ctor0(FALSE_ID);
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

/// The table's `decode_char_list` on a value: a Cons/Nil spine of bytes
/// (each 0..256) that is valid UTF-8; None on any other shape (the call
/// then takes the general path and stays stuck, as the table decides).
fn decode_byte_list(v: &Val) -> Option<String> {
    let mut bytes = Vec::new();
    let mut cur = v;
    loop {
        match cur {
            Val::Ctor(c) if c.name() == NIL_ID && c.len() == 0 => {
                return String::from_utf8(bytes).ok();
            }
            Val::Ctor(c) if c.name() == CONS_ID && c.len() == 2 => {
                let f = c.fields();
                match &f[0] {
                    Val::Int(b) if (0..256).contains(b) => bytes.push(*b as u8),
                    _ => return None,
                }
                cur = &f[1];
            }
            _ => return None,
        }
    }
}

/// The table's `encode_char_list`: the name's UTF-8 bytes as a Cons/Nil
/// spine of ints, built back to front.
fn encode_byte_list(s: &str) -> Val {
    let mut acc = ctor0(NIL_ID);
    for b in s.bytes().rev() {
        acc = Val::Ctor(CtorRef::new(CONS_ID, [Val::Int(b as i64), acc].into_iter()));
    }
    acc
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
            return Ok(bool_val(a == b));
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
        // The symbol ↔ bytes bridge the reader lives on: the general path
        // would rebuild the whole byte list as an Expr and back.
        (PrimTag::SymOfChars, [v]) => {
            if let Some(s) = decode_byte_list(v) {
                prof_count_prim("sym_of_chars");
                return Ok(Val::Sym(intern(&s)));
            }
        }
        (PrimTag::CharsOfSym, [Val::Sym(id)]) => {
            prof_count_prim("chars_of_sym");
            return Ok(encode_byte_list(&name_str(*id)));
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

// Name tests are INTEGER COMPARES: every runtime name — lowered patterns
// and Ctor evals, bool_val's cached True/False, prim and effect-handler
// results via val_of_value_expr — is an id from the one canonical
// interner, so string-equal names are the same id by construction.
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
        IPat::Sym(s) => matches!(v, Val::Sym(t) if t == s),
        IPat::Ctor(cn, sub_pats, flat) => {
            if let Val::Ctor(c) = v {
                if c.name() == *cn && sub_pats.len() == c.len() {
                    if *flat {
                        acc.extend_from_slice(c.fields());
                        return true;
                    }
                    for (sp, sv) in sub_pats.iter().zip(c.fields().iter()) {
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
                Val::Int(m) => {
                    if *cn == Z_ID {
                        return sub_pats.is_empty() && *m == 0;
                    }
                    if *cn == S_ID && sub_pats.len() == 1 {
                        return *m > 0 && match_pat(&sub_pats[0], &Val::Int(m - 1), acc);
                    }
                }
                // A Big is never zero (canonical), so only S can match it.
                Val::Big(m) => {
                    if *cn == Z_ID {
                        return false;
                    }
                    if *cn == S_ID && sub_pats.len() == 1 {
                        if num_traits::Signed::is_positive(&**m) {
                            return match_pat(&sub_pats[0], &int_val((**m).clone() - 1), acc);
                        }
                        return false;
                    }
                }
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
        Val::Sym(s) => Expr::SymLit(name_str(*s).to_string()),
        Val::FVar(s) => Expr::FVar(name_str(*s).to_string()),
        Val::Ctor(c) => {
            Expr::Ctor(name_str(c.name()).to_string(), c.fields().iter().map(val_to_expr).collect())
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
        Expr::Ctor(n, args) => Val::Ctor(CtorRef::new(intern(n), args.iter().map(val_of_value_expr))),
        // Primitives only ever return values; any other shape is a bug
        // in the primitive table, not reachable input.
        other => unreachable!("primitive returned a non-value expr: {other:?}"),
    }
}
