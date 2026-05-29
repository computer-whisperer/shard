# v2 → v3 Revisit Ledger

Decisions made during v2 design under uncertainty. Each entry is: what
was chosen, why now, and what would push us to revisit in v3 rather
than retrofit during v2.

v2 is a prototype. If a choice here proves painful in practice, this
is where to start when planning v3.

## Architecture / Trust

### Locally-nameless term representation
- **Chose:** hybrid `FVar Symbol` + `BVar Int` (de Bruijn). See
  `kernel/term.sexp`.
- **Why now:** `subst` (the hot path) stays simple; free vars keep
  names in intermediate goals (matters for LLM-in-the-loop authoring);
  capture-free by construction for the common substitution operation.
- **Revisit if:** the kernel ends up dominated by off-by-one bugs in
  `shift`/`open_many`, or the open/close discipline causes recurring
  confusion. Alternatives: full de Bruijn (uniform but unreadable raw),
  names + α-rename (heavy fresh-name plumbing in a first-order pure
  setting).

### Trusted-by-review Rust component
- **Chose:** Rust evaluator audited rather than proven correct.
- **Why now:** proving the bootstrap evaluator is its own project,
  and at this scale code review is tractable.
- **Revisit if:** TCB anxiety becomes load-bearing for users, or if
  we want to bootstrap a proof of the Rust evaluator.

### No termination check in narrow
- **Chose:** narrow trusts authors; nonterminating code just hangs.
  Termination admissibility lives in the full-language checker.
- **Why now:** narrow is a small audited substrate; a syntactic check
  tangles with mutual recursion and adds trusted Rust code.
- **Revisit if:** we burn debugging time on bootstrap-kernel loops a
  cheap syntactic check would have caught.

## Language Surface

### sexp file format
- **Chose:** s-expressions for all source and artifact files
  (placeholder extension `.sexp`).
- **Why now:** off-the-shelf parsers, hand-writable, LLM-fluent,
  zero parser TCB beyond the sexp library itself.
- **Revisit if:** paren density makes large kernel files unreadable.
  Next step would be a Rust-like surface syntax with a parser written
  in the object language, lowering to canonical sexp.

### `if` as a core form
- **Chose:** primitive form, not match-on-Bool sugar.
- **Why now:** match-on-Bool was painful even in 50 lines of
  experimentation; `if` is universal and trivial in the evaluator.
- **Revisit if:** ever — hard to imagine being wrong.

### Wildcard `_` in patterns and ignored binder positions
- **Chose:** allowed in both positions, same character.
- **Why now:** load-bearing for non-trivial pattern code; conventional.

### Parallel `let` only
- **Chose:** all RHSs see outer env; sequential expressible as
  nested `let`.
- **Why now:** cleaner for equational reasoning (no order dependency).
- **Revisit if:** common idioms force ugly nesting; could add `let*`
  surface sugar.

### Pattern binding order: innermost-first
- **Chose:** `match_pat` accumulates bindings such that
  `bindings[k]` corresponds to `BVar k` — the rightmost (last
  encountered) PVar binds the innermost (lowest) index.
- **Why now:** consistent with `open_many` lookup by BVar index;
  standard convention.
- **Revisit if:** the inversion produces awkward arithmetic when
  writing the surface→core elaborator.

### Erased polymorphism in narrow
- **Chose:** type variables permitted syntactically; no narrow-level
  parametricity check.
- **Why now:** lets the kernel use generic data structures cleanly;
  the full-language checker enforces parametricity for theorems.
- **State (slice 31):** polymorphic fns and Goals now have surface
  syntax (`(fn (NAME T1 T2) …)`, `(tv T)` in claim bodies). The
  kernel needed no changes — Expr pattern matching is type-agnostic,
  so polymorphic lemmas state and cite at concrete types via the
  ordinary pat-var Rewrite path. Reverse tower (`list_lemmas.sexp`)
  is now stated once over `(List T)` and demonstrated reused at
  `(List Int)` and `(List Symbol)` via one-step Rewrite citations.
- **Revisit if:** subtle parametricity violations creep into the
  bootstrap kernel and create real bugs.

## Primitives

### Native `Int` (bignum) + `Symbol`, no fixed-width
- **Chose:** arbitrary-precision `Int` and interned `Symbol` as the
  only primitive value types. Modular / fixed-width arithmetic is a
  library wrapper around `Int` (`mod`, bitwise ops as primitives).
- **Why now:** simpler semantics, fewer reasoning rules, decidable
  BitVec theory available via SMT later.
- **Revisit if:** SMT integration is cleaner with BitVec primitives,
  or modular-heavy targets push library performance unacceptably.

### Primitive comparisons return user `Bool`
- **Chose:** `int_eq` / `sym_eq` return the user-defined `Bool` ADT.
  Rust evaluator learns the `True` / `False` ctor names via a module
  header directive.
- **Why now:** keeps call sites natural (`(if (int_eq a b) ... ...)`);
  avoids 0/1-Int wrapping ceremony.
- **Revisit if:** multiple incompatible `Bool` definitions need to
  coexist, or the coupling pinches when bootstrapping variants.

### Primitives reachable from the kernel's reducer
- **Chose:** the kernel's `step` (and through it `simp_expr`) handles
  primitive calls via a `try_step_prim` helper. When `lookup_fn`
  fails on a Call value's symbol, `try_step_prim` pattern-matches
  the args against the primitive shapes (two IntLit / two SymLit /
  …) and invokes the primitive *in narrow code* (e.g. literal
  `(+ a b)` inside `try_step_prim`'s body). The Rust evaluator's
  stuck-and-intercept then dispatches it to `prim::try_apply`.
- **Why now:** stuck-and-intercept on the OUTER level handles Calls
  the Rust eval is asked to execute as code. But when the kernel's
  step is reducing an Expr VALUE (data) that happens to be a Call
  to a primitive, the Rust eval never sees a `+` to execute — it
  sees a Ctor value with "+" in it. The kernel has to do its own
  primitive dispatch on that data. Discovered at slice 8 when the
  first user-fn proof fell over because `(double 5)` correctly
  unfolded to `(+ 5 5)` but then stuck (no kernel handling of `+`).
- **Cost:** the kernel duplicates the primitive table (names and
  argument-shape patterns). Each primitive added in `src/prim.rs`
  also needs a clause in `try_step_prim`.
- **Revisit if:** the primitive set grows beyond a dozen or two,
  or if we want primitives to be discoverable rather than
  hardcoded. The cleaner long-term fix is for the kernel to call
  out to an extern (`try_prim_step`) that the Rust runtime
  intercepts, putting the primitive table on one side only. That
  introduces a runtime hook the kernel's reducer doesn't currently
  need; weighed against the table-duplication cost, deferred.

### Primitive call protocol: stuck-and-intercept
- **Chose:** the narrow reducer treats Call'd symbols with no `FnDef`
  as stuck (returns `None`). The Rust runtime, driving the reducer,
  is responsible for recognizing primitive call sites in the stuck
  expression and applying them itself.
- **Why now:** keeps narrow free of any primitive-name list (no
  coupling to the Rust primitive set); the narrow code we wrote does
  not need to know what primitives exist or do.
- **Revisit if:** Rust's "scan for primitive call sites" pass becomes
  awkward, or if we want the narrow reducer to be runnable
  standalone (e.g., in a self-test). Alternatives: emit a `PrimApply`
  marker form the reducer can construct, or carry a primitive-name
  list on `Module` so the reducer can route calls explicitly.

### Library maps, not primitive
- **Chose:** maps as a library type (balanced tree of pairs).
- **Why now:** keeps Rust TCB smaller; matches "primitives only when
  semantics demand it."
- **Revisit if:** kernel memo / environment access measures as a real
  bottleneck. Promotion to a Rust-backed primitive is the expected
  path.

## Proof Language

### `Theory` as flat ordered cons-list
- **Chose:** `Theory` is `(TheoryEmpty)` / `(TheoryCons name claim rest)` —
  later theorems cite earlier ones by name; order prevents circularity.
- **Why now:** matches v1's working approach; the kernel only needs
  citation-by-name and ordering at this stage.
- **Revisit when:** refinement composition and cross-artifact reuse
  start mattering. The natural upgrade is a content-addressed
  artifact store with dependency tracking — a separate concern from
  the kernel ADT, but it would replace `Theory`.

### `Reduce` and `Simp` are now split — Simp guarded by gated δ
- **History:** between slices 7c and 13, `Reduce` was wired as full
  δ+ι (driving `simp_expr` = `step` to fixed point). `Simp` was
  stubbed. Slice 13b surfaced the gap: IH-consuming inductive
  proofs need a reducer that fires Matches/Ifs but STOPS at
  recursive sub-calls, so a Rewrite-via-IH can match the exposed
  subterm. Slice 14 split them. Slice 30 guarded the δ side.
- **State (after slice 30):**
  - `Reduce side` is ι-only: drives `simp_iota_expr`, which uses
    `step_iota` — Match firing, If dispatch, Let opening, descend
    into Ctor/Call args. NEVER unfolds a Call (user fn or primitive).
  - `Simp side` is ι + *gated* δ: drives `simp_expr` (now backed
    by `step_smart` + a list-based memo). A user-fn Call only
    unfolds if `step_head` would take a one-step reduction on the
    unfolded body — i.e., the body's head is a Match with a
    value-headed scrutinee, an If with True/False condition, a
    Let, or a primitive Call with all-value args. Otherwise the
    Call stays surface, and Simp tries to step its args. Primitives
    at the head always reduce.
- **Why the gate is *head-only* (not full `step`):** using full
  `step` as the gate let `(append (append (Cons _ _) ys) zs)`
  unfold its OUTER call (because `step` recurses through Match
  scrutinees into the inner Call, which itself steps). The
  resulting Match-on-stuck-scrut never composed back to the
  surface form the IH wanted. The head-only check is precise:
  commit only when the unfolded body reduces *at the head* in one
  move. Fewer unfoldings, but the ones that happen are exactly
  the ones the author wanted.
- **What this buys:** the slice-29 LCF helper-lemma tax collapses.
  The reverse tower shrank from 10 lemmas (with 6 per-ctor `_step`
  helpers) to 4 — author drives recursive-fn reduction with
  `(Simp Both)` directly. v1's "blowup" liability is also
  mitigated: the gate refuses unfoldings whose result wouldn't
  immediately progress, bounding pathological re-substitution.
- **What this does NOT solve:** general non-termination of
  recursive fns. If a fn's recursion has no halting structure on
  ground inputs (e.g. closed `(loop_forever)`), Simp still
  diverges. The gate is necessary, not sufficient.
- **Revisit if:** authors find the gate too conservative (a real
  proof wants Simp to push past one of the cases it refuses), or
  too permissive (a real blowup case where memoization isn't
  enough). The structurally-shared / hash-cons memo is the v3
  successor; the list-based memo here is a deliberate
  quadratic-cost placeholder.

### Simp memoization: list-based, quadratic (placeholder)
- **Chose (slice 30):** `simp_expr_loop` carries a
  `(List (Pair Expr Expr))` memo through its fixed-point recursion;
  the public `simp_expr` wraps it with `Nil` and discards the result
  table. Lookup is linear via `expr_eq`; insert is `Cons`.
- **Why now:** the v1 lesson on memoization is real; an unmemoized
  `simp_expr` would re-traverse substituted subterms in pathological
  call-graphs. List-based memo gives correctness today without
  inventing a hashable Expr representation or a runtime-provided
  map primitive.
- **Cost:** O(n²) per simp_expr call where n is the number of
  distinct sub-reductions performed. Each top-level reducer step
  appends one entry and scans all prior entries. Acceptable at v2's
  proof-obligation scale; will not survive larger reductions.
- **Scope:** only the OUTER simp_expr loop is memoized.
  `step_smart`'s internal recursion (Ctor args, Match scrutinees,
  step_smart_list) does NOT thread the memo — narrow has no
  monadic bind, so threading would multiply every step_smart_* fn's
  signature. The outer memo catches "same Expr appears multiple
  times as a top-level reducer target."
- **Revisit when:** structurally-shared / content-addressed Expr
  storage exists (hash-cons via Symbol interning extended to
  whole-Expr fingerprints, or a Rust-side hash-map primitive). At
  that point both the memo data structure and the granularity (full
  step_smart recursion, not just the outer loop) should be revisited
  together.

### LCF helper-lemma discipline — RESOLVED (slice 29 → slice 30)
- **State (slice 29):** the kernel's reducer couldn't always do the
  targeted reduction a proof wanted. `Unfold` is greedy on the
  outermost matching Call, and `Reduce` (ι-only) doesn't step
  Calls. A proof with nested `(append (append _ _) _)` shape
  exposed this — the outer unfold blocked before the inner Call
  became value-headed. The slice-29 workaround was to prove one
  helper lemma per ctor arm of each recursive fn (~5 LOC each,
  mechanical) and cite them via Rewrite.
- **Resolved (slice 30):** the gated-δ Simp can now do the
  targeted reduction directly. `(Simp Lhs)` on `(append (Cons h t)
  ys)` reduces to `(Cons h (append t ys))` and stops at the IH-
  blocked inner call. The 6 helper lemmas the reverse tower needed
  (append/rev/fast × Nil/Cons) all collapse into `(Simp …)` steps.
  The slice-29 author burden is gone going forward.
- **Tradeoff:** the gate is conservative — a proof that wanted Simp
  to push *past* an IH-blocked subterm (rare; the author usually
  wants the opposite) would now need an explicit Unfold + Reduce
  + lemma-cite chain. The conservative direction is the right
  default for IH-style proofs.

### `ByTheory` cert payload under-specified per theory
- **Chose:** `(Cert Symbol Expr)` — theory name plus a payload
  encoded as an `Expr`. Concrete payload shapes (e.g. LRAT-style
  trace for LIA, bitblasting transcript for BV) are pinned only
  when each theory's checker is implemented.
- **Why now:** reserves the proof leaf so the kernel grammar is
  stable; defers cert encoding per theory.
- **Revisit if:** `Expr` is too restrictive for some theory's
  certificate format (e.g., one wants opaque binary blobs); promote
  the payload to a more general carrier.

### `eqdec` — equality-reflection backend (slice 33)
- **Chose:** a second `ByTheory` backend (`kernel/eqdec.sexp`) that
  decides `(int_eq a b) = True` via `lia_decide a b` and
  `(sym_eq a b) = True` via `expr_eq a b`. Fixed orientation
  (comparison on LHS, `True` on RHS); only the `= True` direction;
  any other head / arity / RHS shape → `False`. Motivated by finite
  maps needing reflexivity `int_eq k k = True`, which the reducer
  leaves stuck on a variable (`int_eq` only fires on closed IntLits).
- **Why now:** decided-not-axiomatized keeps the audit ledger empty,
  and it's the smallest possible second backend — proves the
  pluggable-`ByTheory` slot generalizes past LIA.
- **Two caveats (both the standard erased-types caveat, surfaced by
  the slice-33 soundness review):**
    - `sym_eq`'s decider is `expr_eq`, which returns `True` for ANY
      two structurally-identical Exprs (equal FVars, equal Calls,
      etc.) — broader than the runtime `sym_eq` primitive, which only
      fires on two `SymLit`s. This is *required*: reflexivity on a
      symbol VARIABLE (`sym_eq x x`, x an FVar) is the whole point and
      can't be expressed by restricting to `SymLit`. Sound for
      well-typed terms (where `sym_eq`'s args are symbols, so
      syntactic identity ⟹ equal symbol ⟹ `True`); a compound-term
      `sym_eq` is ill-typed and never arises in well-typed proofs. No
      `sym_eq`-via-eqdec lemma is authored yet, so this path is
      currently unexercised.
    - `int_eq` over opaque atoms inherits LIA's "atoms are assumed
      integer-typed" assumption (see `lia_collect`). Not new to eqdec.
- **Revisit if:** the full-language type checker lands and we want the
  backend to *enforce* operand types rather than rely on well-typed
  inputs; or if a `sym_eq`-over-compound goal ever surfaces (it would
  expose the breadth gap). Also a natural home to widen to other
  reflected primitives (`lt`/`le` reflected into Bool, etc.).

### `ord` — order-reflection backend (slice 35)
- **Chose:** a third `ByTheory` backend (`kernel/ord.sexp`) that
  decides `(lt a b) = True` and `(le a b) = True` by reusing LIA's
  canonicalizer on `(b - a)`: accept iff the difference reduces to a
  lone constant `c` with `c >= 1` (strict) or `c >= 0` (non-strict).
  Only the `= True` direction; fixed orientation; variables surviving
  in the difference → reject (not a tautology). Motivated by the M3
  loop invariant, whose guard/bounds reasoning is inequality, not
  equality (LIA decides equalities only).
- **Why now:** the M3 capstone (slice 34) confirmed concretely that
  the loop invariant needs `lt`/`le`. Smallest backend that unblocks
  it; reuses LIA's polynomial machinery, so it's ~30 lines.
- **Caveats (same family as lia/eqdec):** opaque atoms assumed
  integer-typed (inherited from `lia_collect`); the threshold check
  `c >= threshold` uses the concrete `le` PRIMITIVE on known integers
  — deciding a symbolic order fact by concrete arithmetic, sound and
  not circular. Conditional order facts (transitivity, bounds under a
  hypothesis) are NOT proven — they arrive as premises, like eqdec's
  disequalities.
- **Revisit if:** a proof needs to PROVE a conditional inequality
  (e.g. derive `a < c` from `a < b` and `b <= c` without the chain
  being a closed-form constant) — that wants a richer LIA/Presburger
  fragment or premise-aware deduction in the backend. Also the natural
  place to add strict/non-strict mixing or `(lt a b) = False` ⟺
  `(le b a) = True` if a proof needs the negated form.

### `farkas` — linear-integer entailment via certificate (slice 37, 38, 41)
- **Slice 41 extension:** also decides EQUALITY conclusions
  `premises ⊢ (int_eq a b) = True`, two-sided — prove `a <= b` AND
  `b <= a`, each its own Farkas refutation (reusing farkas_refute /
  farkas_finish). Cert payload becomes a pair of multiplier lists
  `(list le_mults ge_mults)`. Sound over ℤ (antisymmetry); the M3
  mirror index arithmetic (e.g. `i = j = p ⊢ i+j-p = p`) needs it.
- **Slice 38 extension:** also decides DISEQUALITY conclusions
  `premises ⊢ (int_eq a b) = False`, by negating to the equality
  `a = b` (an any-sign constraint) and refuting against the premises —
  e.g. `(lt a b)=True ⊢ (int_eq a b)=False`. The goal multiplier's
  nonneg requirement is conditional: enforced for `lt`/`le` goals
  (inequality negation), any-sign for `int_eq=False` goals (equality
  negation). This is the M3 enabler turning loop bounds into the
  `(int_eq p i)=False` premises `read_swap_other` consumes.
- **Chose:** a fourth `ByTheory` backend that decides
  `premises ⊢ (lt|le a b) = True` by CHECKING a Farkas combination:
  the cert payload `(list G M0 M1 …)` supplies nonnegative multipliers
  (G on the negated goal, Mk on premise k), and the kernel verifies
  `G·¬goal + Σ Mk·premise_k` canonicalizes to a constant `< 0`. The
  search for multipliers is the untrusted proposer's job; the kernel
  only checks. First real user of the `Cert` payload (lia/eqdec/ord
  ignore it — tautologies need no witness).
- **Why now:** the M3 loop invariant needs conditional inequality
  reasoning (`p < i ⊢ p < i+1`) that the tautology backends can't do
  (they ignore premises). This is exactly TRANSFER's "SMT-as-
  certificate, small per-theory checker, don't trust the solver's
  bare unsat" — induction stays in the kernel, the decidable leaf is
  a checked Farkas witness.
- **Two soundness-critical guards (verified by the slice-37 review):**
  (1) inequality-derived constraints (lt/le and the negated goal) take
  only NONNEGATIVE multipliers; equality-derived constraints
  (int_eq=True, plain equations) take any sign. (2) the combination
  must canonicalize to a lone constant strictly `< 0`. A sign error in
  either would be unsound; both are covered by Rust mirrors incl. the
  crux pair (neg multiplier rejected on an inequality, accepted on an
  equality).
- **Caveats:** conclusions are order facts only (equality conclusions
  stay with lia, or need a future two-sided combination); opaque atoms
  assumed integer-typed (inherited from lia_collect); a non-linear
  premise is usable only with multiplier 0. **i64 overflow:** the
  multiplier arithmetic uses the host `+`/`*` (i64, like lia/ord) — the
  kernel trusts these don't overflow. Not farkas-specific and not a
  clean exploit path (variable cancellation uses the same wrapping
  arithmetic), but if `Int` is meant to be mathematical integers, the
  primitives should eventually be checked/bignum.
- **Revisit if:** a proof needs equality conclusions by entailment
  (add the two-sided refutation), or the overflow caveat becomes
  load-bearing (move arithmetic primitives to checked/bignum).

### RewriteWith — single-match only (Insts shipped slice 32)
- **Chose (slice 27):** the conditional rewrite proof step landed
  with two restrictions:
    - Match is **single-occurrence only**. There's no `all_occ`
      variant. Even with `Both`, the kernel takes the first match
      (lhs preferred) and produces a single binding env; rhs is
      consulted only if lhs has no match.
    - Non-Nil `(List Inst)` was rejected (`False`).
- **Updated (slice 32):** Insts pre-instantiation now works in both
  Rewrite and RewriteWith. The single-match-only restriction
  remains; see "Insts pre-instantiation" below for the Insts
  mechanism.
- **Revisit when:** a real proof needs to rewrite multiple
  occurrences of a conditional pattern in one step (rare in
  practice — most authoring threads the rewrite into a Steps
  prefix).

### Insts pre-instantiation (slice 32)
- **Chose (slice 32):** an `(Inst NAME EXPR)` in a Rewrite or
  RewriteWith step pre-instantiates one of the cited Goal's
  ∀-binders before the conclusion pattern match runs. The kernel's
  `split_params_by_insts` walks cited_params in introduction order;
  each param is either pinned by an Inst (binding = the Inst's
  Expr) or left for capture-matching (binding = fresh FVar,
  added to pat_vars).
- **Why now:** without Insts, citing a lemma whose pattern can't
  cover all its ∀-binders was impossible — the rewriter would
  substitute the cited equation with fresh FVars that never
  appeared in the goal. Concrete blocker: lemmas with a "pivot"
  binder appearing only on the RHS (e.g., the LIA identity
  `∀ pivot a. a = (a - pivot) + pivot` in `examples/lia_basics.sexp`
  — `pivot` is invisible to the LHS pattern, so the user must
  pin it). Unblocks the natural transitivity-shaped lemma
  pattern that was on the v2 deferred list.
- **Validation:** `all_insts_named` rejects an Inst that names a
  Param not in cited_params (returns None / False). Duplicates
  within Insts are first-match-wins via `find_inst` — later
  duplicates are silently ignored. Could tighten to "reject
  duplicates" if it becomes an authoring footgun.
- **Cost:** ~40 NCNB in `check.sexp` (three helpers + reworked
  Rewrite / RewriteWith arms). Kernel growth.
- **Revisit if:** Insts ergonomics need more polish (e.g.,
  positional Insts instead of by-name; or scope-checking against
  declared tparams). The by-name form is more verbose but
  unambiguous, matching Rust's `Foo::<T = Bar>` rather than
  `Foo<Bar>`.

### Open-vs-closed Goal forms (binary's storage convention)
- **State (slice 27):** the kernel uses two conventions for the
  Goal ADT depending on context:
    - Top-level Sequent being proved: ∀-bound vars are *opened* to
      FVars matching the Param names. Required so steps like
      Induct (substitutes by name) and Rewrite (matches by name)
      fire correctly.
    - Goal stored in the Theory as a Proven/Axiom entry: ∀-bound
      vars are *closed* to BVars, innermost-first. Required so
      `resolve_eq` / the Rewrite + RewriteWith arms can open them
      to fresh FVars at citation time.
- **Binary's bridge:** authors write claim Goals in FVar form
  (friendlier). After a claim PASSES, the binary calls
  `close_goal_for_storage` (kernel helper) on the goal value before
  consing it onto the running Theory.
- **Why now:** slice 27 surfaced this — RewriteWith couldn't
  exercise the binary's path without the close step (FVar-form
  Goals stored in theory don't open correctly). The Rust tests
  always wrote stored Goals directly in BVar form, so the gap
  hadn't shown up.
- **Revisit when:** a different convention emerges (e.g., if we
  add explicit `(open …)` / `(close …)` forms to the proof
  language and want everything in one canonical shape).

### Rewrite pattern-matching descends into `Match` arm bodies
- **Chose:** v2's rewriter descends under binders (v1 refused to,
  to dodge variable capture). Locally-nameless makes this capture-
  safe: pattern variables are tracked out-of-band, and BVars in the
  target match structurally.
- **Why now:** unlocks rewrites v1 couldn't express; depth-tracking
  machinery already exists in `shift` / `open_many`.
- **Revisit if:** matching-under-binders produces surprising
  performance or correctness issues. Falling back to v1's stance
  is a small edit.

### Implicit type instantiation by default in full language
- **Chose:** ML-style inference; explicit instantiation available
  when needed.
- **Why now:** matches the TRANSFER mandate; explicit annotation is
  the rare case.
- **Revisit if:** inference ambiguities accumulate in practice.

## Foreign Boundaries

### `ExternDef` as a separate `Module` field
- **Chose:** extended `Module` from `(types fns)` to `(types fns externs)`;
  `ExternDef` is its own ADT with signature only (no body).
- **Why now:** keeps `FnDef` cleanly "fully-defined function with a
  body" and makes the audit query "what externs does this module
  declare?" a one-liner. Alternative was `FnDef` with `(Option Expr)`
  body where `None` = extern.
- **Revisit if:** we want to attach per-extern metadata (linkage
  info, model bindings, axioms) and the separate-list shape starts
  fighting that. The `Option`-body variant would let a richer `FnDef`
  carry all of it uniformly.

### `TheoryEntry` tagged `Proven` vs `Axiom`
- **Chose:** `(type TheoryEntry (Proven Symbol Goal) (Axiom Symbol Goal))`
  rather than untagged `(Symbol Goal)` with a parallel set of axiom
  names.
- **Why now:** makes the audit boundary visible at every theory
  access — every code path that looks at a theory entry has to
  acknowledge whether it's looking at a proven theorem or an axiom.
- **Revisit if:** we add more entry kinds (bridging axiom, deferred
  obligation, etc.) and the binary sum gets awkward. Then a richer
  enum or a record with kind tags is the upgrade.

### Bridging-axiom distinction not in v2
- **Chose:** all axioms are `Axiom` entries; no separate tag for
  "this axiom is the model-mirrors-reality kind."
- **Why now:** the distinction is auditing UX, not kernel semantics;
  not worth the additional ADT shape until the audit tool exists.
- **Revisit when:** the audit ledger tool lands and bridging axioms
  start outnumbering operational ones in the modellable-extern
  pattern. Add a `BridgingAxiom` constructor or a tag on `Axiom`.

## Implementation Conventions

### Manual Option-bind in narrow (no do-notation)
- **Status:** narrow has no monadic bind / do-notation, so chained
  `(Option …) → (Option …)` pipelines collapse into pyramids of
  `(match … ((Some x) …) (None None))`. `do_induct` has an 8-level
  nested destructure that would be ~5 lines with `bind_opt`.
- **Why now:** narrow is deliberately minimal; bind/do is a full-
  language concern (it wants HOF, i.e. `apply$`).
- **Revisit when:** the kernel grows enough Option-chaining that the
  pyramids become a maintenance problem. The fix is in the full
  language: define `bind_opt` and rewrite the kernel using it once
  the kernel is dogfooded onto the full evaluator.

### Fresh-symbol generation as an effectful primitive
- **Chose:** `(gen_fresh)` is a runtime-provided primitive that returns
  a unique `Symbol` each call. Used by the kernel when opening binders
  (Induct / CaseOn) to mint fresh FVar names.
- **Why now:** the pure alternative — threading a counter through
  every binder-introducing operation — is exactly the `Pair`-cascade
  ugliness from option 1 of the capture-avoidance comparison, and
  it'd pollute the entire kernel's signatures. A single effectful
  primitive is a small, localized exception that keeps the rest of
  the kernel pure-shaped.
- **Revisit if:** we ever want to formally reason about the kernel
  inside the system itself — at that point the effectful primitive
  becomes a soundness obstacle, and we'd switch to explicit counter
  threading (probably with `let` to soften the boilerplate, or with
  the full language's HOF to abstract the threading).

### Single-pass `subst`; env values not re-substituted
- **Chose:** `subst` replaces FVars once; values in env taken verbatim.
- **Why now:** standard proof-assistant convention; iterated subst
  available by repeated calls.
- **Revisit if:** iterated substitution becomes the common case.

### Language and project not yet named
- **Chose:** placeholder `.sexp` extension; project remains
  "proving_bootstrap_test_v2".
- **Revisit:** once the kernel takes shape and a name suggests itself.

## CLI / Tooling

### Proof-file surface syntax: light sugar on value-construction sexp
- **Chose (slice 23, sugared slices 25 + 28):** `(claim NAME GOAL PROOF)`
  where GOAL and PROOF are narrow expressions parsed against the
  kernel's ctor set and evaluated. Surface sugar:
    - `'foo` → `(quote foo)` → `SymLit foo` (lexpr's reader handles
      `'` automatically).
    - `(list a b c)` → `(Cons a (Cons b (Cons c Nil)))` (slice 25;
      reserves `list` as a name).
    - `(ty NAME a1 a2 …)` → `(TCon 'NAME (list a1 a2 …))`, with
      bare symbols inside `ty` treated as 0-ary type names; e.g.
      `(ty List Int)` → nested TCons (slice 28; reserves `ty`).
- **Why now:** reuses `load::expr_from_value` plus the existing
  evaluator. The kernel's ctor application IS the value-construction
  syntax. The two sugars cut a typical claim by ~50% of LOC and
  bring authoring close to a "math content" / "syntactic noise"
  ratio that's acceptable for hand-writing.
- **What's still verbose:** `(TCon 'Int (list))` for base types
  (the `(list)` for empty type args is unavoidable without further
  sugar); the `Cons` / `Nil` / `Some` / `None` Pair chain in
  more-elaborate proofs.
- **Slice 26 stress test:** ported `(add_nat n Z) = n` from Rust
  test to sexp. 95-LOC Rust test (with BVar-index comments) →
  21-LOC sexp claim content + 13-LOC user module. The sexp version
  drops the manual BVar reasoning the Rust test needed (the loader
  does it). No new sugar needs surfaced — the slice 25 sugars
  carried a full Induct + Case + Unfold + Reduce + Rewrite + IH
  proof without further friction.
- **Revisit when:** real proof authoring surfaces specific pain
  points. Cheap next steps if needed: a unary type ctor shorthand
  (e.g., `Int` as a bare symbol meaning `(TCon 'Int (list))`), or
  a `(pair a b)` form. Heavier: a separate proof-script surface
  syntax that lowers to canonical claim sexp.

### Expr-value vs. source-Expr distinction in claim bodies
- **State (slice 26):** claim bodies talk about *Expr values* — the
  Ctor-tree representation of source terms — not source-level
  terms directly. So the source-Z (a Nat ctor application) is
  written as `(Ctor 'Z (list))` in a claim body, not bare `Z`.
  Same for Calls: `(double 5)` is the SOURCE, but inside a claim
  body it's `(Call 'double (list (IntLit 5)))` (an Expr value).
  This caught implementation correctly on the first inductive
  port but is the most likely surface-syntax footgun for
  newcomers.
- **Why:** the claim language is a meta-language *about* user
  source. The kernel's Expr ADT has `Ctor`, `Call`, `FVar`, etc.
  as ctors of Expr; claim bodies build *those*, not the user's
  ctors directly. Conflating the two would require a layered
  parser that knows when it's parsing meta vs object.
- **Revisit when:** newcomer confusion is a real pattern, or once
  a higher-level proof-script surface lowers source-form to
  Expr-form automatically (e.g., a `(:source Z)` form expanding
  to `(Ctor 'Z (list))`).

### Proof-file module syntax: parse-but-error
- **Chose (slice 23):** `(module NAME)` is recognized as a top-level
  form but rejected with "not yet implemented in v1".
- **Why now:** locks in the syntax so v1 proof files won't need
  rewriting when the directory-tree loader lands. Avoids implementing
  the loader (with name resolution, cycle detection, path
  resolution) before the kernel's scale demands it.
- **Revisit when:** a proof artifact wants to span multiple files
  with cross-references. Likely concurrent with introducing
  hierarchical names (which v1 supports via Symbols already, but
  nothing produces or relies on hierarchical Symbol contents yet).

### `(use-module "path")` accumulates, does not replace
- **Chose (slice 24):** each `(use-module "path/to/file.sexp")`
  directive loads that file as a Rust `ast::Module` and *merges*
  its types / fns / externs into a running user-module accumulator.
  All subsequent claims (until end-of-run) see the merged module
  as the `m` arg to `check_sequent`. The accumulator persists across
  files in a single binary invocation (no per-file reset).
- **Why now:** matches Rust's `mod` semantics in spirit (declarations
  bring items into scope, additively). Lets a proof file pull in
  multiple lib files cumulatively without manual concatenation.
  Last-replaces would force any claim file using lib A and lib B
  to manually concat them upstream.
- **Cost:** name conflicts across modules are silent — first
  declaration wins on lookup_fn / lookup_typedef. A real scaling
  story wants explicit imports / namespacing; for v2 this likely
  unifies with `(module …)` and hierarchical names.
- **Revisit when:** any of: real conflicts arise; per-file isolation
  becomes desirable (e.g., for parallel checking); or the (module …)
  loader lands and subsumes use-module's role.

### ast::Module → runtime-value conversion in the binary
- **Chose (slice 24):** the mechanical Rust-AST → runtime-Ctor-value
  conversion (Expr/Pat/Type/CtorDef/FnDef/ExternDef/TypeDef/Module)
  lives in `src/bin/check.rs`. ~110 LOC, no public API change.
- **Why now:** the binary is the only caller; promoting to lib /
  ast.rs would grow the trusted core for a single consumer.
- **Revisit when:** a second caller appears (e.g., tests that want
  to load a user module from disk rather than building with nval
  helpers, or a future REPL). At that point promote to a
  `pub module_value::to_value` API.

### Kernel loader is a flat path list
- **Chose:** the kernel's seven `.sexp` files are loaded by a
  hardcoded list in `lib.rs::load_kernel_from`. Tests and the
  `check` binary share that list.
- **Why now:** the kernel itself doesn't yet use the (module …)
  directive (and `module` isn't implemented anyway). Migrating the
  kernel to a module-tree layout is a separable slice.
- **Revisit when:** the directory-tree loader lands. Migrating the
  kernel to `kernel/mod.sexp` becomes a consistency cleanup.

### User modules see the kernel's ctors during parsing
- **Chose (slice 29):** `module_from_paths_with_base(paths, Some(&kernel))`
  passes the kernel as a parsing base, so user-module fn bodies
  and patterns can reference stdlib ctors (Nil, Cons, Some, None,
  True, False, Pair) without redeclaring those types.
- **Why now:** surfaced as soon as the first user module
  (`examples/list_lib.sexp`) tried to pattern-match on `Cons`. The
  alternative — forcing every user module to copy stdlib's type
  decls — fights every layer of UX.
- **Revisit if:** name clashes between kernel and user ctors
  become an authoring concern. Today's resolution is "first
  declaration wins"; could move to explicit imports later.

### `check` binary seeds the user-module value with kernel types
- **Chose (slice 29):** when constructing the user-module value
  passed as the `m` arg to `check_sequent`, the binary starts the
  accumulator with `kernel.types.clone()` (all kernel-internal
  typedefs, including stdlib + AST types like Expr/Pat/Goal).
  Subsequent (use-module …) directives extend this.
- **Why now:** without the seed, `do_induct`'s `lookup_typedef`
  on the user-module value can't find `List` when inducting on
  `(List Int)`. Top-level claims that talk about polymorphic types
  declared in stdlib (the common case) would fail.
- **Cost:** the runtime user-module value is bigger by ~25
  typedefs. None of them block lookup or interfere structurally;
  the kernel walks types until matching the requested name.
- **Revisit if:** the seed creates ambiguities (e.g., a user
  declares a type with the same name as a kernel type) or grows
  large enough to matter for lookup performance.

### Type-parameter symbols become TVar at load time
- **Chose (slice 29):** `load_type_in_scope` accepts a list of
  declared type-parameter names and emits `TVar` (not `TCon`) for
  matching bare symbols inside that typedef's ctor field types.
  E.g., `(type (List T) (Cons T (List T)))` produces
  `CtorDef Cons [TVar "T", TCon "List" [TVar "T"]]`.
- **Why now:** the kernel's `type_subst` only fires on `TVar`. The
  previous behavior emitted `TCon "T" []` uniformly, which meant
  inducting over a polymorphic loaded type couldn't substitute
  field types correctly → no IH generated → inductive proofs over
  `(List Int)`-style instances silently failed.
- **Extended (slice 31):** the same scope discipline now covers
  fn-signature type parameters via `(fn (NAME T1 T2) PARAMS RET
  BODY)`. The fn's tparams are passed to `load_type_in_scope` when
  parsing param and return types; `extern` mirrors the form.

### Polymorphic-fn syntax and `(tv T)` claim-body sugar
- **Chose (slice 31):**
  - `(fn (NAME T1 T2 …) PARAMS RET BODY)` for polymorphic fns;
    `(fn NAME PARAMS RET BODY)` remains monomorphic. Parameterized
    head mirrors `(type (NAME T1 T2) …)`.
  - `(tv NAME)` builds a `TVar` value in claim bodies. Drop-in for
    `(ty …)` — write `(ty List (tv T))` instead of the explicit
    `(TCon 'List (list (TVar 'T)))`. Reserves `tv` as a special form.
- **Why now:** the v2 mandate's headline polymorphism item ("prove
  `append_nil` once over `List<T>`") needs polymorphic *fn*
  signatures *and* polymorphic *Goal* params. The slice was almost
  entirely loader work — verified by a Rust probe that built a
  polymorphic Goal by hand (`probe_polymorphic_append_nil_right`)
  and watched it pass without kernel changes. Expr pattern matching
  is type-agnostic, so polymorphic lemmas instantiate at concrete
  types via the ordinary pat-var Rewrite path.
- **Cost:** ~50 NCNB in `load.rs` (parameterized-head parsing for
  fn/extern, new `load_tv` special form). No kernel growth.
- **What it buys:** `examples/list_lemmas.sexp`'s claims are now
  stated once over `(List T)` and the capstone `fast_eq_rev` is
  demonstrated reused at `(List Int)` and `(List Symbol)` via
  one-step `Rewrite (Lemma 'fast_eq_rev) Lr Lhs True (list)`. Real
  proof reuse, working today.
- **Revisit if:** the (tv …) form's lack of scope-checking starts
  catching typos as TVars silently (a (tv typo) in a position where
  the user meant a concrete type). Today's resolution is "explicit
  is good"; could add a tparam-scope check at claim time later.

### LCF helper-lemma discipline (per-ctor step lemmas) — historical
- **History:** during slice 29 the kernel's unguarded `Simp` over-
  reduced (no head-only gate; recursive sub-calls were chased to
  forms that didn't match the IH). The workaround was to prove one
  helper lemma per ctor arm of each recursive fn (~5 LOC each,
  `Unfold + Reduce + Refl`) and cite them via Rewrite for surgical
  per-arm reductions. The reverse tower used 6 such helpers.
- **Resolved (slice 30):** the new head-only-gated Simp does the
  per-arm reduction directly. See "Reduce and Simp are now split"
  above. The reverse tower (`examples/list_lemmas.sexp`) now uses
  4 lemmas (down from 10) and drives ctor-case reductions with
  `(Simp Both)` instead of `Unfold + Reduce + per-arm-lemma cites`.
- **What remains true:** when the kernel still can't reduce
  (Simp's gate is conservative, won't push past stuck heads), the
  helper-lemma pattern remains available — just rarely needed.

### Two-step Nat induction (`Induct2`) — kernel addition (slice 50)
- **What:** a fourth branching Proof, `(Induct2 var (list (Case 'Z …)
  (Case 'SZ …) (Case 'SS …)))`. Splits a Nat-shaped var into Z, (S Z),
  and (S (S k)) — the SS arm carrying the IH at k. Needed for functions
  that recurse two-at-a-time (`half_nat`: `half (S (S k)) = S (half k)`),
  where single-step `Induct` only ever yields the IH at k, so the S(S k)
  case can never reach P(k). First user: `half_bound` (n-1 ≤ 2·⌊n/2⌋),
  the loop-completion bound for the M3 capstone.
- **Why a kernel change (TCB cost) rather than an encoding:** two-step
  induction can be hand-encoded with a `(Pair (P n) (P (S n)))` carrier
  + projection lemmas, but it's ~8 fragile lemmas per use (the proofs
  must match Simp's exact output shapes). `Induct2` is ~90 NCNB once,
  reusable for any floor/ceil/parity proof. The user chose the kernel
  addition over the per-use encoding.
- **Soundness:** every Nat is Z, (S Z), or (S (S k)), so the three arms
  cover all values — PROVIDED the type is exactly a nullary ctor + a
  unary recursive ctor. A THIRD ctor would leave values uncovered, so
  `do_induct2` REJECTS unless `is_two_ctors` holds (exactly two ctors;
  one nullary "zero" + one unary-recursive "succ", found generically).
  The SS arm's IH is P(k) only (built by the same `build_ih` as
  single-step Induct). Guarded by Rust tests `check_seq_induct2_*`
  (accepts a true claim; rejects a false arm, a missing arm, and a
  three-ctor type).
- **Revisit if:** we need strong/course-of-values induction (IH for all
  m < n) or k-step for k>2 — `Induct2` is deliberately the minimal
  Nat-specific form. A general well-founded induction is the bigger,
  later addition (it needs the order predicate in the IH goal).
