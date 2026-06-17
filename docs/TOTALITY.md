# Totality — the measure-descent admissibility system

> Status legend: **[BUILT]** in the kernel and exercised by the corpus ·
> **[DECIDED]** ratified, not yet built · **[FUTURE]** anticipated, deliberately
> deferred. Keep these honest — this doc is the record of *why* the system has
> the shape it does, so a later change starts from intent, not from re-derivation.

See also: `OVERVIEW.md` §"Definitions are admitted" (the one-paragraph intent),
`REVISIT.md` (the dated decision log — the 2026-06-12 ratification and the
2026-06-17 refinement this doc describes), issue #1 (the tracker).

---

## 1. The problem this closes

`unfold` treats every `fn` body as a total function's defining equation, so `fn`
is the largest axiom generator in the language. With no admissibility check, a
non-terminating definition mints an inconsistent theorem:

```
(fn liar ((n Int)) Int (+ (liar n) 1))   ;; ⊢ liar 0 = liar 0 + 1  →  farkas: 0 = 1
```

This is the same bug class as primitive-name shadowing (`qualified-identity`):
the trusted core hands the untrusted regime a false premise. Totality closes it
by admitting a recursive `fn` into the logic **only** when it provably terminates.

## 2. The primitive: nonnegative-Int measure descent

One primitive, ratified 2026-06-12. A recursive definition enters the logic iff
**every recursive call, under its path condition, strictly decreases an Int
measure that stays ≥ 0**. Well-foundedness of bounded integer descent has the
external pedigree we demand of axioms (the same trust floor as admitting `Int`
itself). Consequences, all ratified:

- **No partiality.** No `partial-fn` caste, no codata. Genuinely unbounded
  processes (interpreter, reducer fixpoints, event loops) take an Int fuel
  parameter and are measure-admitted on it — *clocked* big-step semantics
  (CakeML precedent); exhaustion is loud refusal (the sound direction); the
  unfueled meaning is recoverable as ∃-fuel theorems.
- **Int, not unary Peano.** Matches the corpus idiom (Int counters + WfInduct)
  and avoids pushing executable loops onto Peano fuel.

## 3. The architecture principle: discover offline, verify at check time

This is the load-bearing decision and the reason for everything in §4–§5. It is
the **same principle as the sidecar proof system** (`auto-proof-solver`):

> Proof *search* runs offline (`tools/prove`); its result is a **stored
> artifact** (the sidecar); check time only **verifies** the stored proof.
> Search is never in the trust path.

Totality has the identical shape. The "discovered" thing is *which argument
descends* / *what the measure is*. So:

- **Discovery is offline and advisory.** `admit` (kernel/admit.shard) is the
  `tools/prove` of totality: it classifies every fn's recursion
  (AdNonRec / AdStruct / AdMutual / AdFlag / AdUnresolved) and *can suggest* the
  descent. It is **not** in the trust path.
- **The result is a stored artifact in source** — the `(measure …)` clause.
- **The check-time gate only verifies that clause. It never searches.**

### 3.1 Why the recognizer must stay out of the TCB

A descent recognizer *inside the gate* is trusted code: a bug that accepts a
non-descending recursion is a **soundness** bug — it re-opens the `0=1` door.
A recognizer used only as an offline suggester is **not** TCB: its bugs only
mislead the author, who must still commit a `(measure …)` clause that the small,
stable verifier checks. **Moving discovery out of the gate shrinks the TCB.**

A second reason (the author's stated principle, 2026-06-17): *if a later update
can change what an "auto-" mechanism finds, prefer explicit bookkeeping.*
Strengthening a recognizer to catch more cases is exactly that instability.
(Note the failure is *safe*: a sound recognizer regressing accepts *fewer* fns →
loud check failure, never silent admission of a non-terminating fn. The argument
against an in-gate recognizer is TCB size + verdict drift, not silent unsoundness.)

## 4. The verify-don't-search contract [DECIDED 2026-06-17]

Every recursive fn carries an **explicit declaration** of its descent. The gate
verifies the declaration; it never discovers one. Two clause forms:

- **Structural** — `(measure (struct ARG))` *(form [DECIDED], not yet built)*.
  The author names the descending parameter. The checker verifies, per recursive
  call, that the argument in `ARG`'s position is a **strict subterm** of the
  parameter — a small, decidable, *stable* check, **no proof required**. This is
  the sidecar discipline at minimal cost: the *which-argument* is the stored
  decision; the checker replays it. TCB shrinks to "is this term a subterm of
  that one," far smaller and more auditable than "search all positions for a
  consistent descending designation across a mutual SCC."
- **Numeric** — `(measure E proof0 proof1 …)` **[BUILT]**. The author gives an
  Int measure `E` and one proof per recursive call site (pre-order). The kernel
  emits the decrease + nonnegativity obligations and discharges them through the
  ordinary claim pipeline (the untrusted regime; the prover already enumerates
  farkas certificates for exactly this shape).

`admit`'s classifier can **generate** the `(struct …)` declarations as an
offline migration aid; the author commits the output to source. This supersedes
the 2026-06-12 "structural auto-recognized, zero-annotation" plan: structural
descent is now *explicitly declared and verified*, not auto-accepted at check
time. (Migration cost: the ~468 currently-auto structural/mutual fns each gain a
one-token `(struct …)`. Chosen over a frozen in-gate recognizer to keep discovery
out of the TCB; chosen over full measure+proof everywhere to avoid proof cost on
trivial list recursion.)

## 5. The single-fn numeric gate [BUILT, report-only]

The driver is `mc_check_fn` (kernel/driver.shard). For a fn `F` with
`(measure E proofs…)` (parsed into `MCl`):

1. **SCC** of `F` from Tarjan (`ad_sccs`), looked up with `mc_scc_of`.
2. **Site collection** — `mc_walk` walks `F`'s body, recording each recursive
   call as `MSiteS i bnds tbs hyps args`: pre-order index, in-scope binders,
   typed binders, accumulated **path-condition hyps** (if/match conditions,
   outermost-first), and the call's arg forms. Path-condition collection is
   **soundness-critical trusted code** — a missed condition admits a false
   decrease.
3. **Circularity guard** — `mc_opaque m scc` drops the *whole SCC* from the
   module and rebuilds the trie, so the fns under admission are stuck terms to
   their own termination proof (a fn may not prove its own totality by unfolding
   itself).
4. **Citable theory** — `mc_theory_for_scc`: stratified citation. An obligation
   for SCC `scc` may cite fact `L` iff `L`'s statement **cannot reach** `scc`
   through the call graph (`mc_reaches`). This is the precise, order-independent
   statement of "an unproven lemma may not prove itself, directly or indirectly"
   — it unifies the old granted-only + same-closure-ban sources.
5. **Per-site obligations** — for each site, two goals checked via
   `mc_check_one` (parse → `tc_check_goal` → desugar proof → `check_sequent`):
   - decrease: `(= (lt E[args] E) True)` under the hyps
   - nonneg: `(= (le 0 E[args]) True)`

   where `E[args] = mc_subst sub msr` substitutes caller params → call args.

Report-only today: a failure prints a `MEASURED … FAIL` line. The pins
`examples/measure_clause.shard` carry intentional cheats (false decrease / false
nonneg) that must FAIL.

## 6. The mutual extension [DECIDED, not built]

The single-fn gate already operates at SCC granularity (steps 3–4 drop and
filter the *whole* SCC). The only gap: it checks **self-calls only**, ignoring
calls to other SCC members. The extension closes that with **no new syntax** —
each member keeps its own `(measure Eᵢ …)` clause.

For every SCC-internal call `Fᵢ → Fⱼ` at args `A` under path condition `P`:

- decrease: `P ⟹ Eⱼ[A] < Eᵢ[caller params]`  — *callee's* measure on the left
- nonneg:   `P ⟹ 0 ≤ Eⱼ[A]`

Self-calls are the special case `j = i` (exactly today's obligation).

**Soundness (common measure).** Along any internal call chain `c₀→c₁→…`, set
`mₖ = E_{fn(cₖ)}[args of cₖ]`. Each step's decrease obligation gives
`mₖ₊₁ < mₖ`; nonneg gives `mₖ ≥ 0`. A strictly descending nonneg-Int sequence is
finite → no infinite internal chain. Same well-order argument as the single-fn
case, with the callee's measure supplying the LHS.

**Why common-measure suffices for v1.** The unresolved kernel SCCs are
AST-recursions (tc_unify, the elaborator, the checker core); each takes a natural
*common* measure = total AST size — every internal edge passes a smaller
subterm, so one shared Int drops. Lexicographic per-member ranks are a clean
*additive* generalization of the same `callee < caller` shape, deferred to
[FUTURE] (see §8).

### 6.1 Discharging the measure's nonneg: subterm induction [BUILT]

A common-measure = total AST size makes every *decrease* obligation trivial
(`child < parent` by farkas), but the **nonneg** obligation `0 ≤ E` requires
proving the size function is nonnegative — and for an AST whose constructor
carries a *list of itself* (`Ctor QName (List Expr)`), `size` is mutually
recursive with `size_list`, so `size_nonneg` and `size_list_nonneg` are mutually
recursive *lemmas*. Shallow `induct` cannot prove them: inducting on an `Expr`
yields an IH only for same-type fields (`build_ihs`), never for the `Expr`s
nested inside a `(List Expr)` field; and a mutual lemma citation is a cycle that
stratified citation rejects.

The fix is **well-founded induction along the canonical structural subterm
order**, exposed as two proof primitives:

- `(subterm-induct VAR PF)` — like `wf-induct`, but the order is `y ⊰ x`
  (`y` a strict structural sub-value of `x`) instead of an Int measure. One
  subgoal, strong IH at Hyp 0: `∀P'. premises(P') → (subterm_below x' x) →
  goal(P')`. Reaches nested occurrences the shallow IH cannot. Gated to datatype
  vars (⊰ is only well-founded on an inductive type).
- `(below)` — discharges the IH's ordering premise `(subterm_below a x) = True`
  by checking `a` is a strict structural sub-term of `x` (resolved to ctor form
  via substitution, or an in-scope `x = CTORFORM` hyp). The ONLY discharger of
  `subterm_below`, which is otherwise inert (no reduction rule, unprovable by
  farkas/simp/refl).

`size_nonneg` then proves by `(subterm-induct e)`: its `Ctor`/`Call` case walks
the child list by ordinary `(induct args)`, pulling each head's `0 ≤ size a` from
the strong IH (head ⊰ node, via `below`); `size_list_nonneg` cites `size_nonneg`
one-directionally. The cycle is gone — and totality itself stays the numeric
size measure (no phase tags, no `struct_size` in the TCB).

**Why this is the right generalization, not a gadget.** `⊰` is the *same*
well-founded order shallow `induct` already trusts — `do_induct` is the special
case "case-split + immediate same-type-field IHs." `wf-induct` (Int order) and
`subterm-induct` (⊰ order) are one principle at two orders; an eventual roster
audit should merge them (and recognize `do_induct` as an instance). It stays
inside verify-don't-search: `below` *decides* a syntactic fact, it does not
search. In the term representation, object containers (`List`/`Pair`/…) are
themselves nested `Ctor` nodes, so the proper-subterm check is generic — no
per-container logic.

Pins: `examples/subterm_induct.shard` (5/5 — `tsize_nonneg`/`tsize_list_nonneg`
over `Tm = Leaf | Node (List Tm)`); `examples/subterm_induct_rejects.shard`
(`below` refuses reflexive `a ⊰ a`; the gate refuses a non-datatype var).

## 7. Cross-call resolution and cycle-readiness [DECIDED]

shard forbids import cycles **today**, so all SCC members share one module. We
nonetheless build the gate cycle-ready, because the substance is already
module-agnostic (it is expressed over QNames + the call graph: Tarjan, opacity,
and stratified citation never look at module boundaries; the soundness argument
references call edges, not modules). Four decisions make the gate cycle-safe at
near-zero cost:

1. **Resolve cross-call heads to QNames via use-scope, test `∈ scc`** (option
   "b"), resolving each head **in the caller-member's scope**. The purely
   syntactic short-name test is rejected: it has a shadowing hole *today* (a
   sibling name shadowed by an import → the real sibling call is qualified and
   gets missed → unsound), and it is the *only* gate assumption that would break
   under cross-module cycles. Option (b) is both sound now and cycle-ready.
   Fail-safe: an unresolvable head must emit an obligation (or refuse), never skip.
2. **Gather sibling measures keyed by QName** (`qname → (pnames, Eⱼ, modpath)`),
   not "the other clauses in this file."
3. **Per-member resolution scope and check context** — never assume a single
   shared scope across the SCC.
4. **No `assert SCC ⊆ one module`** anywhere — the SCC is whatever Tarjan emits.

**What genuinely defers to "when cycles are enabled":**

- **Loader two-phase name resolution** [FUTURE] — the real prerequisite, and it
  lives *outside* the gate. Acyclic imports give a topological elaboration order
  (a module fully elaborated before its importers); cycles require Rust's
  crate-style approach: collect all item signatures across the cyclic group, then
  elaborate bodies against the complete name environment. The gate runs
  post-merge, downstream of this, so loader changes don't ripple in.
- **Pre-elaborated measures for cross-module composition** [FUTURE] — a
  cross-call goal mixes names from two scopes (RHS `Eᵢ` caller, LHS `Eⱼ` callee).
  Today one shared `gctx` parses it because single-module. Under cross-module
  cycles the measure expressions must be elaborated to QNames before composing
  the goal so it parses scope-independently. Bounded, localized to obligation
  construction — a known change, not a landmine.

## 8. Enforcement predicate (Phase D) [DECIDED]

The flip from report-only to refusal. The predicate is:

> **Every recursive SCC carries an explicit, verified measure clause.**

There is **no auto-recognition exemption** — `admit` stays advisory/offline
(§3). A structural SCC satisfies the predicate via its verified `(struct …)`
declarations (§4); a non-structural SCC via its numeric `(measure E proofs…)`.
Phase-D prerequisites: the mutual extension (§6) and the `(struct …)` verifier
(§4) must be built first.

## 9. The trusted core (what to audit)

**Trusted** (a bug here can be a soundness bug):

- the obligation emitter — path-condition collection in `mc_walk`, arg
  substitution `mc_subst`, goal construction (`mc_goal_s`);
- the `(struct …)` subterm verifier [DECIDED];
- the circularity guard `mc_opaque`;
- stratified-citation reachability `mc_reaches` / `mc_theory_for_scc`;
- cross-call QName resolution + `∈ scc` test [DECIDED] (a miss is unsound);
- the structural subterm order [BUILT, §6.1]: `subterm_below`'s
  well-foundedness (the same ⊰ `do_induct` already trusts) and `do_below`'s
  strict-subterm decision (`expr_proper_subterm`) — a `below` that accepted a
  non-subterm would make `subterm-induct` unsound;
- and everything `check_sequent` already is.

**Not trusted** (advisory or untrusted-regime): `admit`'s classifier; the prover
/ farkas certificate search; the proofs themselves (re-checked by `check_sequent`).

## 10. Open / deferred work

- **[DECIDED, not built]** mutual extension (§6); `(struct …)` form + verifier
  (§4); enforcement flip (§8).
- **[FUTURE]** lexicographic per-member ranks for large/heterogeneous SCCs
  (esp. accidental cross-module ones); **sidecar files** for measures, if the
  in-source clause burden warrants moving discovery results out-of-band; the
  **trie↔fns consistency bridge** required by the data-weighted SCCs
  (`tj_visit`/`tj_neis`, `loader/visit`, and `mc_reaches`) — these saturate a
  visited-set over the FnTrie and need a nested-trie / enumeration lemma the flat
  worklist measures (`led_close`/`call_close`) did not; **reducer fuel** for the
  genuinely-partial loops (`run_expr`/`compute_expr_loop`/`simp_expr_loop`/
  `simp_iota_expr`/`ceval`), a Timeout return-type change per §2; cross-module
  pre-elaborated measures (§7).

## 11. Where the code lives

- `kernel/driver.shard` — the gate: `mc_check_fn` and the `mc_*` family
  (site walk, obligation construction, stratified citation, `led_close` /
  `call_close` data-weighted worklist measures).
- `kernel/admit.shard` — the offline classifier: Tarjan (`ad_sccs`), the
  structural/mutual recognition (`ad_pick` / `ad_verify`), the report renderer.
- `kernel/checker.shard` — `do_subterm_induct` / `build_subterm_subgoal`
  (subterm induction, §6.1) and `do_below` / `expr_proper_subterm` (the ⊰
  discharge); the `Proof` ctors `SubtermInduct` / `Below` live in
  `kernel/proof.shard`, parsed in `kernel/proof_reader.shard`.
- `examples/measure_clause.shard` — must-fail cheat pins;
  `examples/measure_import_synth.shard` — imported-scrutinee binder-typing canary;
  `examples/subterm_induct{,_rejects}.shard` — subterm-induction + soundness pins.
