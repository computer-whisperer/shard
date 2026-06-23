# Refinement — structural invariants as types

> Status legend: **[BUILT]** in the kernel and exercised by the corpus ·
> **[DECIDED]** ratified, not yet built · **[FUTURE]** anticipated, deliberately
> deferred. Keep these honest — this doc records *why* the system has the shape
> it does, so a later change starts from intent, not from re-derivation.

See also: `OVERVIEW.md` §8 (trusted-core contraction — the arc this completes),
`TOTALITY.md` (the companion admissibility system; refinement predicates must be
total, so the two systems interlock), `BOUNDARIES.md` (the audit boundary the
opaque modules present), issue #2 (strings), issue #15 (the contraction).

---

## 1. The problem this closes

An **invariant** is a proposition `P(x)` that holds for every inhabitant of a
type. shard has two ways to maintain one today, and both are weaker than the
language can afford:

- **Compositional / by-construction** (the opaque-module discipline). A value
  can only be made through controlled makers, so every value *that exists* is
  good — but the property is **not statable as a fact about the type**. The
  module itself cannot prove `∀ x:T, P(x)`, because its own constructor admits
  bad inhabitants; only the *exports* are fenced. This is the Rust newtype
  ceiling (`str`, `NonZeroU8`): the compiler enforces "go through the smart
  constructor" but **cannot check `∀ x, P(x)`** — that part is trusted
  discipline, and bounds become runtime panics.
- **Threaded premises** (the convention). State `P(x)` as a hypothesis on every
  lemma that needs it, and re-establish it at every step. This is OCaml's
  "validity lives nowhere" — the property is real but lives in the prose, not
  the type.

The corpus pays for this, measurably and by name:

- `std/word/mod.req.shard:16` and `std/bytes/mod.req.shard:15` carry comments
  saying their range facts are "COMPOSITIONAL … not the forall-inhabitant range
  that would need **the refinement type (designed follow-up)**." The code asks
  for this feature in writing.
- **~145** auto-generated `(le 0 (size …))` non-negativity premises across the
  `*.auto.shard` measure obligations — every one is just "this measure is a
  `Nat`," re-proved at the use site instead of read off the return type.
- **12** Word range obligations (`u8_made_lo/hi`, …, `i32_wrap_lo/hi`): two per
  width, stated over a *maker-headed* expression `(u8 nn)`, never over a bare
  `u : U8`.
- **~120 lines** of byte-range *laundering* in `std/bytes` — `bytes_ok` and the
  three lemmas `mod_byte_id` / `u8val_u8_id` / `roundtrip`, plus the defensive
  re-mask in `bidx` (`u8_val (u8 (raw_at …))`) — all of which exist *only* to
  re-derive a range that a refinement-typed element would carry for free.

Every one of those obligations is currently discharged by **farkas/lia**. The
opportunity is to state the invariant once, on the type, and let the existing
arithmetic backends discharge it automatically.

This is a place where shard can do what C/Rust/Liquid/Lean cannot do *all at
once* — see §8.

## 2. The primitive: constructor-obligation ⊣ destructor-grant

Every invariant mechanism in the design space is one idea:

> **A type whose values carry a proposition. Introduction discharges it as an
> obligation; elimination grants it as a fact.**

The two surface forms are the unary and the relational specializations of this:

| | carrier | proposition | intro | elim |
|---|---|---|---|---|
| **unary refinement** `(refine BASE PRED)` | the bare `BASE` value | `PRED(carrier)` | a checked/proven cast (no wrapper) | projection (identity) + the fact |
| **relational invariant** `(type T (C f…) (invariant Q))` | the constructor fields | `Q(f…)` | the constructor `C` | the match + the fact |

The unary form is the degenerate single-field case whose "constructor" is the
trivial inclusion `BASE ↪ T` and whose "destructor" is the trivial projection
`T → BASE` — so its values *are* their carriers at runtime (no wrapper, identity
coercion). If the unary metatheory and obligation machinery are built as exactly
that special case, the relational form is an additive extension later, **not a
rework**. That is the property this design exists to guarantee — invariants are
"difficult to rework later," so the two forms must share one foundation from
day one.

The soundness story is the standard **proof-irrelevant subset type**: the elim
fact never lies because every intro discharged it; no closed term of the refined
type exists without a discharged obligation; the (erased) evidence does not
participate in equality. It is a **conservative extension** — no new closed proof
of `False`. The risk is checker-implementation bugs, not the concept: small
surface, verify-don't-search, the posture used for every kernel addition.

## 3. The unary refinement type `(refine BASE PRED)` [DECIDED, the v1]

`PRED` is the QName of a **total** `BASE → Bool` function (a named Bool fn — no
first-class functions, matching the rest of the language). A refined type is
**nominal**: it is declared, named, and flows like any other type.

### 3.1 Representation [BUILT]

The `Type` AST is **unchanged** (`Type = (TCon QName (List Type)) | (TVar
Symbol)`). A refined type `R` flows as an ordinary `(TCon R …)`, so unification,
resolution, occurs-check, and well-formedness need no surgery. The refinement
`(BASE, PRED)` is recorded in the module's **type registry** alongside the
datatype definitions (`TypeDef`); the three rules below are the *only* code that
consults it. A refined type is, structurally, like an **opaque type with no
public constructors plus a registry entry** — which is exactly why it stacks
under `(sig type R)`: the interface shows opaque `R`, the impl carries the
`(refine …)` entry. (v1 refined types are **monomorphic** — `Nat`, `U8`, `Str`
all are; parameterized refinements like `Fin N` are §7.)

Surface declaration: `(type R (refine BASE PRED))` — a `type` whose body is the
reserved `refine` clause instead of constructor clauses. Opaque form is the usual
`(sig type R)` in the interface + the `(type R (refine …))` in the impl + the
`use`-glob rebind, identical to how `std/word`/`std/bytes` are hidden.

### 3.2 Introduction — two doors [refined-return BUILT · downcast BUILT]

Proofs live in `claim`s, not in expression position, but values are *constructed*
in runtime function bodies. So "intro requires a proof" cannot mean an inline
proof term. There are two sound doors, and the corpus needs both:

- **[BUILT, slice 2b]** **Decidable downcast** — `(refine_try R EXPR) :
  (Option R)` where `EXPR : BASE`. The obligation is discharged by
  **computation**: on a ground `EXPR` the reducer evaluates `PRED EXPR` and
  yields `(Some EXPR)` iff it is `True`, else `None`. No proof. This is the
  I/O-boundary *validator* — `utf8_decode b = (refine_try Str b)`, exactly Rust's
  `str::from_utf8 → Result`. It threads the evidence *into the data*: in the
  `(Some s)` branch the consumer already holds an `R` with the invariant baked
  in. The type `R` is written bare (`(refine_try Small n)`); the reader carries
  it as a nullary-ctor MARKER `(Ctor R Nil)` in arg0 so the term stays an
  ordinary `Call` — `resolve`'s `Ctor` arm runs `rhead` on `R` (→ the type's
  qname) for free, the typer reads `R` back, and the reducer (both `step_call`
  and the big-step `ceval_call` that `compute` uses) computes `PRED`. See
  `examples/refine_try.shard` (pass) / `examples/refine_try_rejects.shard`
  (must-fail: non-refined marker, wrong base type).
- **[BUILT]** **Refined return type** — `(fn f (a…) R body)`. A refined return
  type `R = (refine BASE PRED)` **emits the obligation**
  `∀a, (= (PRED (f a)) True)` — the proof that `f`'s result really lands in `R`.
  The body is ordinary `BASE`-typed code (`tc_fn_body_ret` relaxes the
  body↔return unification); it is admitted at `R` *because the obligation holds*.
  The proof is supplied by a **separate top-level form `(returns f PROOF)`** —
  not a fn clause, so a recursive refined-return fn keeps its `(measure …)`
  clause unchanged. The driver (`rr_outcome`) **enforces** it: a missing or
  rejected proof is a HARD failure.

  **Circularity guard (load-bearing).** The obligation is checked against a
  module where `f`'s entire recursive SCC has its return lowered `R → BASE`
  (`rr_deref`). So inside the proof `(f a)` and any mutually-recursive sibling
  type as `BASE`, and `refine-fact` — which only fires on a head that is a
  refined type — *cannot* grant their invariants. Without this, a body could
  "prove" its own predicate by `refine-fact` on itself (`big : Small = 99` citing
  `(big) : Small`): pure circularity. The de-refinement is the analogue of the
  measure gate's SCC-opacity, and is exactly why the goal is `PRED (f a)` (no
  `refine_val`: `f` is `BASE` in this context).

  This is the *computed-provably-valid* door — `u8`/`u8_add` (obligation closes
  via `mod_lo`/`mod_hi`), `size_sexpr : Nat` (obligation `0 ≤ size`). For a
  *mutually-recursive* size family the obligation is the same honest structural
  (subterm) induction the standalone `*_nonneg` lemma needs today — the
  refinement moves the FACT to every use site for free, it does not make the
  definition-site proof disappear. The proof may be authored or found by
  `tools/prove`.

  Crucially this is **not** flow-sensitive VC generation (the Liquid/F* machinery
  we reject in §8). The obligation is a plain extensional claim `PRED(f a) =
  True`; a branching body is discharged by `unfold` + `case-on` + `farkas` — the
  *proof tactics* handle the control flow, the type checker does not. The checker
  only **emits** the claim and **grants** the elim fact; the discharge reuses the
  entire existing proof apparatus.

A third, purely-symbolic proof-carrying intro (`refine_mk v PROOF` in claim
context) is **[FUTURE]** — deferred until a client needs to construct a refined
value by abstract proof rather than by computing or by a refined-return fn.

### 3.3 Elimination [BUILT]

- **Projection** — `(refine_val s) : BASE` for `s : R`. Identity at runtime;
  retypes `R → BASE`. (The opaque `std/str` re-exports this as `bytes_of`.) There
  is **no implicit subtyping** — going down to `BASE` is always the explicit
  `refine_val`, going up to `R` is always an intro (which always costs an
  obligation). Explicit both ways keeps the checker a synthesizer with no
  coercion lattice.
- **Fact** — the cut step `(refine-fact s)` materializes the premise
  `(= (PRED (refine_val s)) True)` into the current sequent (then continues, the
  fact appended last — the `div-facts` / `have` shape). Explicit-but-cheap:
  consistent with shard's "materialize facts on demand" discipline rather than
  silently flooding every sequent that mentions an `R`.

### 3.4 Runtime and erasure [DECIDED]

A refined value **is** its carrier. `refine_val` is identity; `refine_try` is
compute-predicate-then-`Option`-wrap; a refined-return fn adds nothing. **Zero
new runtime data**, so the compiled chain (rt.h / lower / codegen) stays trivial
and equality is by carrier (proof irrelevance). The only non-shard touch is a
`load.rs` tolerance so the Rust evaluator skips the `(refine …)` declaration
clause for direct `eval run` (the shape of the C1 "load.rs tolerates measure
clause" change).

## 4. The relational generalization [DECIDED, not built]

`(type T (C f₁ … fₙ) (invariant Q))` attaches an invariant proposition `Q` over
a *datatype's* constructor fields. Applying `C` emits the obligation `Q(f₁…fₙ) =
True`; matching `(C f₁…fₙ)` grants it. This is the same primitive (§2) with an
n-ary relational proposition and a genuine constructor/wrapper (unlike the unary
form's identity inclusion).

It is the home for the corpus's *relational, multi-field* invariants that a unary
subset type cannot express — e.g. snake's `inv` = `nonempty body ∧ all_in_range
body ∧ no_dup body ∧ pos_in_range food ∧ food-off-body` (a property over the body
*list* and the food *field* together), currently threaded as a five-conjunct
premise through every `step_*` requirement. With the invariant on the
constructor, `step` re-proves `inv` once and consumers match to get it free.

**Designed now, built later.** The §3 unary form ships first (it covers `Nat`,
the whole Word/byte family, `Str`); the relational form follows when a consumer
(snake, or a `Date`/`sorted`/`gcd-reduced` client) needs it. The point of writing
it down now is to keep §3's representation and obligation machinery a clean subset
so this is additive.

## 5. The trusted core and the totality interlock

**Trusted** (a bug here can be a soundness bug):

- the **downcast reduction** `refine_try` — wrapping `(Some EXPR)` only when
  `PRED EXPR ⇝ True` is what makes the elim fact true for downcast-introduced
  values;
- the **refined-return obligation gate** `rr_outcome` (BUILT, enforced) — the
  claim it generates (`∀a, PRED(f a) = True`, checked with `f`'s SCC de-refined)
  is what makes the elim fact true for return-introduced values; a missed or
  mis-stated obligation is unsound, so the gate must cover *every* refined-return
  fn in the closure, fail hard on a missing/rejected proof, AND de-refine the
  SCC so the proof cannot cite the very refinement it is establishing
  (`rr_deref` — the circularity guard, pinned by
  `examples/refine_circular_rejects.shard`);
- the **elim-fact step** `refine-fact` — it asserts `PRED(refine_val s)` for an
  arbitrary `s : R`, sound *only* because every intro above discharged it;
- the **registry lookup** that ties a refined `(TCon R …)` to its `(BASE, PRED)`.

**Not trusted** (advisory / re-checked): the obligation *proofs* (re-checked by
`check_sequent`); the `tools/prove` search that finds them.

**Predicate totality is a prerequisite, not an option.** `refine-fact` hands out
`PRED x = True`; if `PRED` does not provably terminate, that fact is meaningless
and the extension is unsound. So a refinement predicate must be **measure-admitted
total** (`TOTALITY.md`). Consequences for sequencing (§6): the bounded-integer
predicates (`le`, `lt`, `andb` of them) are trivially total, so `Nat`/Word ship
with no totality work; `utf8_ok` currently has the open non-structural-recursion
TODO (issue #1), so **`Str` is gated on giving `utf8_ok` a verified measure
first** — String starts with a totality proof, not the type.

## 6. Where it gets used — the migration order [DECIDED]

The evidence flips the obvious order: **String is the hardest client, not the
first.** The feature is validated on the easy, high-volume, fully-automatable
arithmetic cases — where a bug surfaces immediately and farkas discharges
everything — before the harder `Str`.

1. **The feature.**
   - **[BUILT]** the elim core: the type registry (`(type R (refine BASE PRED))`
     parsed + the `TypeDef` 4th field + declaration-time `BASE -> Bool` check),
     `refine_val` typing (`R -> BASE`), and the `refine-fact` proof step (which
     INFERS the term's type and requires its head to equal the named `R`).
     Validated on `examples/refine_basic.shard` (a toy `Small = (refine Int
     is_small)`, the invariant proven on a ∀-bound `s : Small`) +
     `examples/refine_rejects.shard` (borrow-another's-predicate and
     non-refined-type are refused). `check_sequent` still proves its own
     termination (13 sites MEASURED OK).
   - **[BUILT]** the refined-return intro door (slice 2): `tc_fn_body_ret` admits
     a `BASE`-typed body at a refined return `R`; the `refine_val` reduction
     (identity, hoisted in `try_step_prim`) lets the obligation see through the
     body; the `(returns f PROOF)` top-level form carries the proof; and the
     ENFORCED driver gate `rr_outcome` emits a HARD failure for every
     refined-return fn in the closure with a missing or rejected proof. The gate
     is closure-wide (not target-only) because `refine-fact` can exploit *any*
     refined-return fn's contract. Validated on `examples/refine_return.shard`
     (a `three : Small` discharged) + `examples/refine_return_rejects.shard`
     (a body that violates the predicate, and a fn with no `(returns …)`, both
     refused). The soundness COUPLING — the `tc_fn_body_ret` relaxation is only
     sound because `rr_outcome` enforces — is the load-bearing invariant here.
     The gate de-refines `f`'s SCC before checking (`rr_deref`) so the proof
     cannot cite the refinement it is establishing; pinned by
     `examples/refine_circular_rejects.shard`.
   - **[BUILT, slice 2b]** the `refine_try` decidable downcast. The type is
     carried as the nullary-ctor marker `(Ctor R Nil)` in arg0 so the reducer
     can find `PRED`; reduces in both `step_call` and the big-step `ceval_call`.
     Both intro doors now exist; with this `Str`'s `utf8_decode` is expressible.
2. **`Nat`** — a refined nonneg-`Int` `(refine Int (le 0 _))` on the size
   functions. The corpus today carries **~400** `(le 0 (size …))` lemma citations
   plus **~150** measure `#nonneg` sidecar obligations; making the size functions
   return `Nat` makes the nonneg FACT free at every one of those USE sites (via
   the elim door / a Nat-typed measure satisfying its own nonneg obligation).
   Caveat learned in slice 3 scoping: the DEFINITION-site proof is NOT eliminated
   — the circularity guard de-refines the (mutually recursive) size SCC, so each
   `(returns …)` proof is the same honest subterm-induction the standalone
   `*_nonneg` lemma uses today, merely relocated. The win is removing the
   repeated downstream citations, not the one hard proof. Still the best first
   real client (highest volume, no totality blocker), but it is NOT
   "fully automatic", and realizing the measure-side win needs the measure gate
   to recognize a Nat-typed measure expression and skip its `#nonneg` obligation
   (a new, trust-critical measure-machinery feature — scope it explicitly).
3. **Word** — `U8 … i64` as refined `Int` (`(refine Int u8_range)`, etc.). The
   forall-inhabitant range becomes a free elim fact, deleting the **12** range
   obligations, the **~120 lines** of byte laundering, and the `bidx` re-mask.
   This retroactively pays back what the contraction arc just shipped — Word's
   "mature form is the opaque sig hiding a refined Int" (OVERVIEW §8).
4. **`utf8_ok` totality** — a self-contained measure for the RFC-3629 validator
   (the `Str` prerequisite, §5).
5. **`Str`** — the opaque `std/str` module over `(refine Bytes utf8_valid)`. The
   original target of issue #2 Phase 4: `bytes_of = refine_val`, `utf8_decode =
   refine_try Str`, and the module's validity *requirement* is finally
   **fulfilled** (via `refine-fact`) rather than assumed-by-construction —
   structural validity delivered through the opaque interface.
6. **Relational invariants** (§4) — snake `inv`, etc. [FUTURE]

## 7. Why this beats C/Rust/Liquid/Lean

- **C**: invariants are comments + asserts. No static guarantee.
- **Rust**: `str`/`NonZeroU8` are compositional only — private ctor + `unsafe`-
  justified discipline. The compiler **cannot check `∀ x, P(x)`**; it trusts it.
  Bounds → runtime panics.
- **Liquid Haskell / F\***: have refinement types, but discharged by a **trusted
  SMT oracle** via search — flaky, timeouts, and the VC generator is in the TCB.
- **Lean / Coq**: subset types `{x // P x}` with explicit/tactic proofs, but lack
  the *erasure + automatic-for-arithmetic + opaque-hiding* combination.

shard's point in the space: invariants are **structural** (∀-over-type),
**machine-checked by the native proof system**, with obligations **discharged —
automatically via farkas for the pervasive arithmetic cases** — **erased at
runtime**, and **hideable behind opaque interfaces**, with **no external oracle
in the TCB**. The `Nat` retrofit (§6.2) is the crisp demonstration: a function
that *returns a provably-non-negative int*, checked and propagated for free —
which Rust cannot state, Liquid does only with trusted SMT, and Lean does without
the erasure/automation/hiding combination.

## 8. Alternatives considered and rejected

| structure | verdict | why |
|---|---|---|
| Liquid-style ambient refinements + flow-sensitive VC / SMT | reject | needs search over path conditions (against verify-don't-search); the VC generator becomes TCB; we keep proof-*producing* farkas/lia |
| full Σ-types / proof-carrying values `{x // p x}` | reject for v1 | a step toward dependent types (no, per OVERVIEW); our proof-*irrelevant* collapse is simpler and sufficient since predicates are decidable |
| indexed types / GADTs (`Vec n`) | reject | type-level data = a big dependent move; `(refine Int (lt _ n))` recovers `Fin` without indices |
| capability / ghost tokens | reject (status quo) | "thread a `(Valid x)` premise" is exactly what we replace; no type-level guarantee |
| pure compositional (opaque module, no type feature) | insufficient (where we are) | the Rust newtype ceiling — *cannot state* the forall-inhabitant fact |

## 9. Where the code lives [BUILT through slice 2b]

Almost entirely shard kernel (the Rust side is an evaluator/loader only):

- `kernel/module.shard` — the refined-type registry (extend `TypeDef`, or a
  parallel `(QName → (BASE, PRED))` table on `Module`); lookup helper.
- `kernel/reader.shard` / `kernel/loader.shard` — parse `(type R (refine BASE
  PRED))` into the registry; route the `refine` type body. [2b BUILT]
  `elab_refine_try` rewraps `(refine_try R EXPR)`'s bare type name (a `(FVar R)`)
  as the marker `(Ctor R Nil)` — in `elaborate_form`'s default arm, AFTER the
  recursive `elaborate_list`, so it adds no measure-delegation site.
- `kernel/types.shard` — [BUILT] typing for `refine_val` (`R → BASE`) and
  `tc_fn_body_ret` (admit a `BASE`-typed body at a refined return `R`);
  [2b BUILT] typing for `refine_try` (decode `R` from the `(Ctor R Nil)` marker,
  check `EXPR : BASE`, return `(Option R)`). A refined `(TCon R …)` is already a
  first-class `TCon` elsewhere (no change needed). Both `refine_val` and
  `refine_try` are RESERVED names, checked before `tc_fn_sig`.
- `kernel/reduce.shard` — [BUILT] `refine_val` (identity, hoisted in
  `try_step_prim` so it fires for any carrier shape); [2b BUILT] `refine_try` in
  BOTH reducers — small-step `step_call` (`refine_try_step` → `(if (PRED EXPR)
  (Some EXPR) None)`, reusing the `If` machinery) and the big-step `ceval_call`
  that `compute` drives (evaluate `PRED EXPR` to a Bool, wrap `Some` only on
  `True`; one new recursive site, measure updated). Reserved before trie lookup.
- `kernel/proof.shard` — the `RefineFact` `Proof` ctor (Expr + continuation,
  fact appended last — the `DivFacts`/`Have` shape).
- `kernel/proof_reader.shard` — parse `(refine-fact EXPR)`.
- `kernel/checker.shard` — `do_refine_fact`: build the `(= (PRED (refine_val
  EXPR)) True)` premise and continue.
- `kernel/driver.shard` — [BUILT] `rr_outcome`: the ENFORCED refined-return gate
  (mirrors the `mc_outcome` measure path — walks all srcs, builds each obligation
  goal, emits a HARD `COFail` on missing/rejected proof). `rr_deref` de-refines
  the fn's SCC (return `R → BASE`) and the obligation is checked against THAT
  module (the circularity guard); the proof can still unfold the bodies. Plus the
  `RefineFact` arms in the `Proof`-walk family (`resolve_proof` /
  `cites_of_proof` / `proof_has_admit` / `proof_has_inspect`). The
  `(returns f PROOF)` form is whitelisted in the reader's `skip_form`.
- `kernel/desugar.shard` — `RefineFact` premise-count arm if the desugarer counts
  premises (as for `DivFacts`/`Have`).
- `rust_bootstrap/src/load.rs` — tolerate/skip the `(refine …)` declaration clause
  for direct `eval run` (the C1-shaped change).
- Pins: `examples/refine_basic.shard` + `examples/refine_rejects.shard` (slice 1
  — projection, the elim fact, and the borrow-another's-predicate /
  non-refined-type must-fails); `examples/refine_return.shard` +
  `examples/refine_return_rejects.shard` (slice 2 — a discharged refined return,
  and the predicate-escaping-body / missing-`(returns …)` must-fails);
  `examples/refine_circular_rejects.shard` (the circularity-guard pin);
  `examples/refine_try.shard` + `examples/refine_try_rejects.shard` (slice 2b —
  the Some/None downcast both ways, and the non-refined-marker / wrong-base-type
  must-fails). Then the `Nat`/Word/`Str` retrofits become their own corpus
  regressions.

## 10. Open / deferred

- **[FUTURE]** the relational `(invariant …)` form (§4); the symbolic
  proof-carrying intro `refine_mk` (§3.2); **parameterized** refined types
  (`Fin N`, a refinement whose predicate mentions a type/value parameter) — v1 is
  monomorphic (§3.1).
- **[FUTURE]** auto-discharge of the refined-return obligation by the `tools/prove`
  sidecar (the arithmetic cases are pure farkas; wiring them into the generator
  removes the last hand-proof from the `Nat`/Word retrofits).
- **[FUTURE]** interaction with `subterm-induct` / measures when a refined type is
  the recursion carrier (the *carrier* is what descends; note when first hit).
