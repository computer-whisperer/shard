# FOUNDATION.md — the shard V2 foundation: a Lean-parity logic over a lowerable core

> **STATUS: DRAFT v0.4 (Fable), 2026-09-06 — a proposal for review, not law.**
> v0.1 (2026-09-05, `5d95b81`) was reviewed by GPT-6 as
> `SHARD_FOUNDATION_FEEDBACK_v0.1.md` (R1–R8); v0.2 (`d6b25f1`) folded
> that in and was reviewed as `SHARD_FOUNDATION_FOLLOWUP_MEMO_v0.1.md`
> (R9–R16). v0.3 (`524b59c`) answered R9–R16 (§17), rewrote the Lean
> review (§13), added the **proof IR** (§16) and the conventions ruling
> (§12.1, Q9). v0.4 adds **the naming law** (§6.4, RULED 2026-09-06 —
> one name per mathematical object across `fn`/`def`/`theorem`, Lean's
> names and naming grammar by explicit decision, five deliberate
> departures, a refusal list with pointers, `docs/LEAN.md` as the seam
> document in the T9 gate) and turns §12.1's crosswalk into a pure
> old-to-new migration table.
> The GPT-6 documents live in the repo root, untracked. Companion and
> counter-proposal to GPT-6's `SHARD_FOUNDATION_PROPOSAL_v0.3.md` and
> `SHARD_BOOTSTRAP_ADDENDUM_v0.3.md`; their decision IDs D01–D13 and
> B06–B16 are answered in §9. Nothing here is implemented. Sizes are
> estimates and say so. Ratification turns this file into the ledger;
> until then every "law" below is a proposed law.

Evidence baseline: the tree at `5abc600` (B-1b complete, coverage arc
parked), `docs/TCB.md`, `docs/TOTALITY.md`, `docs/CERT.md`,
`docs/SEARCH.md`, `docs/LANGUAGE.md` §10, `kernel/proof.shard`'s
Step/Proof roster, and the 2026-07-24/25 and 2026-09-02 kernel surveys
(six confirmed 0=1 holes, all fixed). The measured B-1b numbers (tb_len
717 / tb_perim 957 / tb_app 1,148 canonical lines per function) are the
immediate provocation.

---

## 0. The proposal in one paragraph

Replace the equational, first-order, tactic-in-kernel checker with a
kernel **K** that implements **Lean 4's kernel rules exactly** (Carneiro's
specification as reconciled with the pinned Lean 4 kernel and Lean4Lean:
universe-polymorphic dependent type theory with an impredicative
proof-irrelevant `Prop`, strictly positive inductive families with
recursors, `Quot`, and the three standard axioms), written in first-order
shard — itself an E program over L-as-data — compilable by shard's own
lowering and, until then, interpreted by the first-order evaluator the
Rust bootstrap already is. On top of K, keep shard's identity as a
**lowerable core**: an executable fragment **E** — first-order after
elaboration, total by verified measure, never closure-bearing — that is
the only thing that lowers to hardware, and that is exactly the language
every existing `fn`, model, and refinement proof already lives in.
Between the agent-facing tactics and K sits a **proof IR** (§16): a
stable, hole-bearing certificate language that search engines navigate
and pins store. Everything shard proves today is restated under an
explicit crosswalk; every special kernel rule becomes a library theorem
or an untrusted tactic; the trusted code shrinks from "35k lines and
twelve special rules" to K plus the E evaluator plus the Rust host, and
the logical assumptions shrink from fifteen hand-written arithmetic
axioms to Lean's three. Parity with Lean is made testable: **an exported
Lean declaration checks in K**. Elaboration and tactics are the real
project and are staged as untrusted meta-layer work.

Two consumers, one engine: an independent mathematical engine that is
not fundamentally incapable of anything agents do in Lean; and a more
capable, LLM-fluent platform for establishing the facts the
refinement/lowering program needs.

---

## 1. Why now, and why this shape

### 1.1 The MVP scars, measured from the inside

- **Every claim is a conditional equation.** No `∃`, no `∀` inside a
  premise, no formula that is not `l = r`. Predicates are Bool
  functions; every fact is `(= (p x) True)`; every conjunction is an
  `andb` chain with peel lemmas. B-1b's ghost twins exist because a
  conclusion cannot say "there exists a ghost heap such that".
- **The tactic zoo is kernel.** `fin-split`, `div-facts`,
  `refine-fact`, `inject`, `below`, the measure gate, the arith
  backend, the Nat-literal view — each a soundness surface. All six
  0=1 holes of 2026 lived in special rules, dispatch tables, or
  text-derived gates; none in the equational logic itself.
- **The typer was bolted on.** The checker began untyped;
  `kernel/types.shard` gates a rewriting engine instead of being it.
- **Positional everything.** Hypothesis, premise, Farkas-slot and
  occurrence indices. The author's working notes for three B-1b
  theorems carry dozens of proof-DSL gotchas. That is the cost of a
  bespoke proof language with no training corpus, paid by every LLM
  author.
- **No proof objects.** A proof is a script replayed; nothing to
  share, hash, or transport. STORAGE caches text because there is
  nothing else.
- **Four evaluators for one semantics** (reduce step/simp, ceval
  compute, evm/ev run, eval.rs); the conformance sweep covers two.

### 1.2 The bar

Lean is the corpus agents are trained on and the language the labs
footnote. The bar is **parity, not compatibility**: in a few years we
must not find the engine fundamentally incapable of the reasoning
agents wish to exploit — dependent structures, sized data, law-bearing
hierarchies, universe-polymorphic constructions, quotients, classical
reasoning, mathematics libraries.

### 1.3 Why Lean's rules *exactly*, not "Lean-informed"

GPT-6's D01 recommends a Lean-informed foundation with justified
departures. This proposal is stricter: **K implements Lean's kernel
rules exactly**, and every departure is a dated decision that names
what it forfeits. Three reasons:

1. **The oracle.** `lean4export` emits every declaration of Lean core
   and Mathlib as explicit kernel terms. Lean4Lean and `nanoda` are
   independent checkers of the same rules; `lean4checker`/`leanchecker`
   replay through Lean's own kernel and serve as the comparison
   harness rather than as independent kernels. A rule-exact K can be
   differentially tested against Lean's kernel on hundreds of thousands
   of declarations. No foundation of our own design gets an oracle like
   that, and the 2026 holes show what a from-scratch checker without
   one costs. The oracle validates closed L checking only, under an
   identified rule/version/input profile, and supplies no completeness
   theorem (R10): E execution, contextual holes, module views and
   lowering need their own tests (§10). Agreement with Lean is strong
   differential evidence, not a quantified soundness bound.
2. **The library.** With rule-exact K, Mathlib's declarations arrive
   by export and check without translation. What export does *not*
   give for free is integration: namespace mapping, elaboration-level
   use from S, and the definitional bridges between Lean's `Nat`/`Int`
   and shard's E types are priced work (§11 Q7).
3. **The corpus.** Agents' Lean intuitions about what is provable,
   what reduces, and what a statement means transfer without a
   translation table.

### 1.4 Rejected alternative: a HOL-class kernel (REJECTED-because, on record)

Considered seriously in the 2026-09-05 exchange. It is the shape shard
already has (shallow embedding, definitions as equations, Candle-style
compute primitive), it has a ten-rule kernel with the best verification
pedigree in the field, its metavariables are trivial, and shard's
present mathematics is first-order over inductive data and Int, where it
loses nothing. Rejected because the question is not present fit but
future capability: a HOL core cannot state natively the first dependent
construction an agent writes (`Vector α n`, a structure with proof
fields, a universe-polymorphic definition) — encodings through
predicates and packages exist, but they are a different convenience
trade-off and precisely the transport tax the bar rules out; and because
its pedigree argument inverts once the Lean oracle is counted (§1.3).
Two claims made for HOL in that exchange were also weaker than stated:
statements migrate under CIC as well (`p x = true` is a proposition),
and the soundness risk of a from-scratch dependent kernel is bounded by
differential testing that a HOL-of-our-own cannot have. What survives
from the HOL argument is recorded in §3.4, §5 and §6.

---

## 2. The languages, named precisely

Five things that are today conflated get names. The rest of the
document is written in these terms.

| name | what it is | who checks it | lowers? |
|---|---|---|---|
| **L** — the logic | Lean-rule dependent type theory: terms, types, propositions, proofs | K | never |
| **E** — the executable fragment | the subset of L definitions that are shard *programs*; today's entire object language | K (as L) + the fragment classifier + the lowering | yes, and only E |
| **P** — proof objects | terms of L whose type is a `Prop` | K | never (erased) |
| **I** — the proof IR | a stable, hole-bearing certificate language elaborated deterministically into P (§16); what search engines navigate and pins store | its elaborator (untrusted); the result is checked by K | — |
| **S** — the surface | s-expression forms, Lean-flavored; elaborated to L, and tactic blocks to I, by an untrusted elaborator | nothing (untrusted) | — |

**The trichotomy survives unchanged.** Requirements are `Prop`s in L;
algorithms are E definitions; low-level spellings are E definitions in
a model's vocabulary; the refinement relation is a theorem in L. What
changes is that requirements may now be *any* proposition, not a
conditional equation.

**Programs are still data.** An E definition is an L constant with a
body; `meta/` inspects it as a term. Quotation, sketching and search
operate on L terms and on I with the same metavariable mechanism (§7,
§16).

**The toolchain is E; the logic is data (proposed law).** L's terms,
levels, declarations and environments are ordinary `type`s declared in
E. K is a set of `fn`s over that data: first-order, total by verified
measure, closure-free, checkable, and lowerable by shard's own
lowering. The same holds for `ev`, the elaborator, the I elaborator,
and every tactic. Higher-order functions, dependent types and `Prop`
exist only *inside the values* the toolchain manipulates; nothing the
toolchain runs on depends on them. This is what lets the whole
toolchain compile and dissolve like every other artifact (§8), and it
is the exact sense in which "HOF exist in what the logical core is able
to reason about, not in the kernel" (user, 2026-09-05). The statement
concerns the toolchain's *implementation*, not the mathematical content
of its inputs; the initial toolchain stays in the first-order E
profile, and source-level lambda conveniences (§5.3) lower into that
profile before any E program exists — nothing about them adds runtime
closures to `ev` or L rules to Rust.

**E has no function values (proposed law).** Lambdas, partial
applications and template arguments in E source are eliminated **by
elaboration** — lambda lifting plus specialization — so that no E
program, and therefore no `ev` value, no Rust value, and no artifact,
ever contains a function value or an indirect call. A `fn` either
lowers or is refused; there is no "interpreted-only" status for a `fn`
(§5.3; GPT-6 withdrew the request in its follow-up).

**One name per mathematical object (proposed law, §6.4).** A `fn`
body, a `def` body and a `theorem` statement refer to the same constant
by the same spelling; the E classifier changes no name; there is no
"executable spelling" and "logical spelling" of one operation. The
split-brain shard has today — `/` truncating beside `ediv`, `int_eq`
and `le` as Bool functions beside `=` and the proof surface, `andb`
chains beside conjunction — is resolved by adopting one vocabulary
(§6.4), not by mapping between two.

**The L/E correspondence is by defining equations (proposed law).** An
E definition's *executable structure* is its source body; its *L
meaning* is that body's elaboration (recursors for structural
recursion, `WellFounded.fix` for measure recursion); the *bridge* is the
set of defining equations `f.eq_N`, each a theorem of L — by `rfl` for
structural recursion, by the equation lemma for well-founded recursion.
This is OVERVIEW.md §7's "every defining equation in the system is a
theorem", restated for V2 (R1; §5.4).

---

## 3. K — the kernel, rule by rule

### 3.1 The normative reference: declarative rules and a bounded procedure (R10)

Two specifications, kept apart:

- **Declarative:** the judgments `E; Γ ⊢ t : T` and `E; Γ ⊢ t ≡ u`,
  as given by Mario Carneiro's *The Type Theory of Lean* (2019)
  **reconciled with** the pinned Lean 4 kernel release and with
  Lean4Lean (arXiv 2403.14064), which records the additions since the
  thesis — nested inductives and structure eta among them — and which
  is the reference for what is and is not proven about those rules.
- **Operational:** `check(env, decl, limits) → outcome` (§3.3), the
  bounded procedure of the initial compatibility profile.

The implementation obligation is that **successful checking implies
the declared judgment** under the environment's well-formedness
assumptions; this document does not claim that obligation proved for
shard. Algorithmic incompleteness, unsupported inputs, and exhausted
resources are explicit limitations of the procedure, never definitions
of the logic. K's rule document restates the reconciled declarative
inventory in shard's terms, versioned; v1 declares **zero departures**.
Normalization is not a theorem of the implemented system; K therefore
budgets.

### 3.2 The inventory

Universe levels: `zero`, `succ`, `max`, `imax`, parameters;
level constraints checked, `Sort u` non-cumulative, `Prop = Sort 0`
impredicative.

Terms: bound variable, free variable (locally nameless, as today),
`Sort`, `Pi`, `lambda`, `app`, `let`, constant with universe
arguments, `Nat` literal, `String` literal, projection.

Judgments: well-formed environment; type inference for closed terms;
definitional equality with **beta, delta (constant unfolding), iota
(recursor computation), zeta (let), eta (functions and structures),
proof irrelevance, Nat-literal computation, Quot computation**.

Declarations: `definition` (body checked, unfoldable), `theorem`
(body checked, treated as opaque for unfolding), `opaque` (body
checked, never unfolded), `axiom` (no body; tracked), `inductive`
(strict positivity, universe constraints, mutual and nested per Lean's
admission fragment; K *generates and validates* the recursor and its
computation rules, as Lean's kernel does), `Quot` primitives.

Axioms admitted by the standard library profile, and no others:
`propext`, `Quot.sound`, `Classical.choice`. Their consistency
pedigree is Lean's model (Carneiro 2019: consistent relative to ZFC
plus ω inaccessible cardinals). The present 15 `kernel/facts.shard`
axioms are **retired as axioms** (§4.3).

Kernel extensions: GMP-accelerated `Nat` operations on literals. The
exact list is **reconciled with the pinned release at phase 0** (Lean
4's inventory at the time of writing: `add sub mul div mod gcd beq ble
land lor xor shiftLeft shiftRight pow log2`). Each accelerated
operation is **bound to a fixed admitted declaration identity** whose
signature and defining equations K validates before the shortcut is
enabled — never a name-based match (R10; this is the QName lesson of
2026-07-25's hole 5, and Lean4Lean's own safeguard). Tests: an
unrelated same-spelled definition is not accelerated; an altered
primitive body is refused.

### 3.3 Resource outcomes and the meaning of each negative

```
check(env, decl, limits)
  → Accepted(receipt)
  | Rejected(Malformed | Unsupported | UnresolvedObligation | ConversionNotEstablished | RuleViolation)
  | Exhausted(resource, site)
```

`Exhausted` is never acceptance and never rejection. Of the rejections,
only `Malformed` and `RuleViolation` (a positivity failure, a universe
violation, a scope escape) are **definitive**: the input can never be
accepted. `ConversionNotEstablished` and `UnresolvedObligation` say
that *this* procedure did not establish the judgment; they are not
theorems of inequality or of unsatisfiability, and a search consumer
may prune on them only as a revisitable heuristic, never as `UNSAT`
(R10, R13; §16). This is Lean's `maxHeartbeats` made a first-class
verdict and the fuel doctrine of `TOTALITY.md` applied to the checker;
K is a total shard program with an explicit budget, as `check_sequent`
is today.

### 3.4 What is *not* in K (the surviving HOL discipline)

No unification, no elaboration, no implicit arguments, no typeclass
resolution, no tactics, no simp, no arithmetic decision procedure, no
measure recognizer, no filesystem, no process-global state. K takes
an environment value and a declaration and returns a verdict. The
elaborator, the I elaborator and every tactic are untrusted producers
of L terms (§6, §16). This is GPT-6's D09 "small authority" and shard's
existing "verify, never search" law, stated once.

### 3.5 Size and the oracle gate

Reference points: `nanoda_lib` (Rust) and Lean4Lean's checker are a few
thousand lines each; `lean4checker` is *not* a size reference (it
replays through Lean's kernel). Estimate for K in shard: **6–10k
lines** including level arithmetic, inductive admission with recursor
generation, and the Nat extensions; the recursor generator is the
largest single piece. The figure is an unmeasured estimate and **not a
gate** — a size gate would encourage hiding admission logic in
"untrusted" helpers. The gate is behavioral (T0, §10): K accepts
`lean4export` output of Lean's `Init` prelude declaration-for-
declaration with identical accept/reject verdicts and axiom closures,
rejects a hostile battery whose members each carry an independently
specified reason for rejection, and logs the supported input and
version scope. When shard and Lean disagree, the rule, the input
mapping and the resource conditions are investigated; no winner is
picked by tool name.

---

## 4. The crosswalk — every current rule, kept, respelled, or retired

This is the section the decision turns on. Left column: the mechanism
as it exists at `5abc600`. Right column: its V2 home.

### 4.1 Declarations

| today | V2 | status |
|---|---|---|
| `(type NAME ctors…)` — first-order ADT, parameters | `type` = an inductive in `Type` with parameters only, no indices, no function-typed fields; **E-eligible by construction** | KEPT (spelling unchanged, meaning = the E-inductive class) |
| — | `inductive` = any Lean inductive family (indices, `Prop`-valued, universe-polymorphic, nested/mutual) | NEW (L only) |
| `(record …)` + generated `F_of`/`with_F` laws | `structure`; projections and eta are kernel-native; the generated laws become `rfl` | KEPT as sugar; the law generator RETIRED |
| `(fn NAME params RET body)` with `(measure …)` | **`fn` = an E definition**: body must pass the fragment classifier (§5); recursion structural (compiled to recursors) or by explicit measure with kernel-checked descent proofs; `fn.eq_N` defining equations generated as theorems (§2's correspondence law) | KEPT — the executable keyword, now with a precise meaning |
| — | `def` = any L definition, `noncomputable` allowed, never lowers | NEW |
| `(extern …)` | `opaque` constant with an E-typed signature plus the bolt-axiom pattern of `BOUNDARIES.md`; reachable-extern ledger unchanged, computed on the post-specialization call graph | KEPT |
| `(sig fn …)` / `(sig type …)` in `mod.req` | **environment views** with three checked conditions (R4): (i) *view validity* — the interface is itself a well-formed environment, every exported type and statement well-typed using only exported declarations and equalities (today's req-scope gate enforces exactly this by construction); (ii) *implementation matching* — the impl environment extends the view's signature with bodies of the declared types and proofs of every requirement; (iii) *evidence binding* — an exported theorem stays bound to the checked implementation or to explicit parameters, never becomes unconditional by body hiding. The consumer checks against the view alone; K cannot unfold a view constant. Body removal alone is not the argument; (i)–(iii) are | KEPT — mechanism unchanged, conditions now stated |
| `(refine BASE PRED)`, `refine_val`, `refine_try`, `refine-fact` | `Subtype` `{x : B // p x = true}`; `.val`; `decide`-based downcast; the invariant is the proof field | KEPT as sugar over the library; the kernel registry and the three intercepts RETIRED |
| `(claim NAME GOAL PROOF)` | `theorem NAME : PROP := PROOF`, PROOF a tactic block elaborating to I (§16) | KEPT; goals may be any `Prop` |
| `(axiom NAME (kind …) GOAL)` | `axiom` with the kind tag; **K tracks the axiom closure natively** (Lean's `#print axioms`); scope gate and kind gate unchanged; **assumption policy is checked at every acceptance boundary** (R12): a proof using a prohibited axiom fails policy even when the proposition is identical | KEPT; the ledger's cites walk becomes a kernel query |
| `(requirement …)` / `(fulfills …)` | unchanged: a stated `Prop` and its later proof, same-goal single source of truth; the requirement is **frozen as resolved** (R11, §6.2) | KEPT |
| `(bin …)` with `entry/externs/trusts/requires`, LEDGER block, MET/UNMET, artifact-claim forms | unchanged; the acceptance manifest keeps six distinct records — requirement and its semantic dependencies, public interface, implementation and executable view, evidence and assumption closure, compiled realization and target observations, checking engine and execution dependencies (R12) | KEPT |
| `(import …)` / `(use …)` / QName identity / selective loading / req-scope gate / canonical dedup | unchanged; a resolved declaration's identity is its QName plus content hash in the environment; statement identity, evidence identity and assumption closure are three distinct records; a private implementation change invalidates inlined/prepared execution but not abstract client proofs, and an evidence-only change re-runs acceptance policy without rebuilding machine code (R12, T6) | KEPT |
| `auto` + `.auto.shard` sidecars | sources carry tactic blocks; sidecars carry **I plus the checked term DAG's hash** (§16; Q2 RULED as amended); LS-law 1 (replay is the referee) verbatim | KEPT, respelled |

### 4.2 Proof forms (`kernel/proof.shard` roster, every constructor)

Every row's V2 home is an I form or a tactic; the I vocabulary of §16.2
is, deliberately, this roster generalized.

| today (kernel rule) | V2 (where it lives) |
|---|---|
| `Refl` | `rfl` — K's defeq |
| `Steps` + `Unfold` | `unfold`/`delta` (I form) → explicit `Eq.mpr` over a defining equation, or defeq |
| `Reduce`, `Simp` (+ `stop` fence) | `simp only [...]` (I form with its lemma list and budget; the tactic `simp` that *discovers* the list emits it) |
| `Compute` (+ `stop`) | `decide` / `rfl` by K's whnf for structural definitions; the **evaluation reflection** of §5.4 for measure-recursive ones (I form carries the fuel and result) |
| `Rewrite` (+ `Occ`), `RewriteWith` (+ obligations) | `rw` (I form: lemma, instantiation, occurrence, direction) → `Eq.mpr`/`congrArg` terms |
| `Change`, `ExactConv` (CERT.md §3) | `change`/`show`/`exact` — defeq at the named site, budgeted; the ratified *explicit* conversion policy becomes elaboration policy: irreducible by default, unfolding named |
| `Induct`, `Induct2` | `induction` (I form) → the inductive's recursor (K-generated) |
| `CaseOn` | `cases` (I form) → `casesOn` |
| `WfInduct` (Int measure) | `WellFounded.fix` over `Int`/`Nat` measure via `Acc`; measure descent = a proof term, verified never searched |
| `SubtermInduct`, `Below` | course-of-values recursion (`brecOn`-style) derived from the recursor; `below` = the derived strong-induction lemma |
| `Have` | `have` — a `let`-bound proof |
| `Absurd` (head clash) | `noConfusion` theorems (elaborator-derived from recursors) + `nomatch`/`absurd` |
| `Inject` | `injection` → `noConfusion` |
| `FinSplit` | `decide` over `Fin`/interval + arithmetic reflection |
| `ByTheory` lia/eqdec/ord/farkas/reflect/arith | **certified reflection**: an `omega`-class procedure in meta that emits an I form carrying its certificate, and/or a shard-proven `farkas_ok : Rows → Cert → Bool` with a soundness theorem, applied by `decide`. The kernel arith backend is DELETED |
| `DivFacts` | library theorems about `Nat.div`/`Int.emod` (Lean has them; kernel-accelerated on literals) |
| `RefineFact` | `Subtype.property` |
| `Inspect`, `Admit` | tactic-level dev aids (`sorry` reported loudly, never accepted) |
| positional `Hyp k` / `Premise k` / Farkas slots | named local context; no positional citation survives |

**Every row's kernel code is deleted.** The kernel keeps typing,
defeq, inductive admission, and axiom tracking; nothing else.

**Three meanings of "do not unfold", kept separate (R4):** *interface
opacity* limits what a client may rely on and is enforced by the view
(the constant has no body); *tactic transparency* controls what a
proof-producing tool attempts and is a policy of the elaborator;
*kernel conversion* decides whether a submitted term is valid and is
never modified to emulate either of the others. Lean's `decide +kernel`
ignoring transparency settings is the reminder that an irreducibility
mark is not a boundary.

### 4.3 Trust before and after

Logical assumptions and trusted code are different records; the table
keeps them apart. A checker written in E is not more trustworthy by
language choice; what E ownership buys is integration, dogfooding, and
the route-1 certified execution path (§8).

| roster item today (`TCB.md`) | V2 |
|---|---|
| Rust bootstrap interpreter (execution authority) | KEPT as the enduring bootstrap executor of E (§8); execution dependencies recorded per route |
| checker sources `kernel/*.shard` (~35k lines) | K sources (est. 6–10k) + the E loader + `ev`; typer, reducer twins, rewriter, tactics, measure gate, arith, desugar RETIRED from the roster |
| `kernel/facts.shard` — 15 axioms (mod range, euclidean completion, ring laws, bitwise recurrences, shifts) | **retired as axioms**: `Int` is an inductive over `Nat`; `Nat` ops are defined by recursion; every one of the 15 is a library theorem (Lean proves all of them); the kernel's Nat-literal acceleration is the only "prim" left, bound to fixed declaration identities and tied by conformance to the definitions |
| per-artifact trust scopes (bolts, bridges, kinds) | KEPT verbatim |
| prim tables ×2 (+ the untracked PrimTag fast path = 3) | ONE definition per Nat op in the library; ONE acceleration table in K; ONE E evaluator (§5.4); conformance sweeps definition-vs-acceleration and evaluator-vs-equations |
| — (logical assumptions) | three axioms `propext`, `Quot.sound`, `Classical.choice`, tracked per declaration |

Net: the trusted code loses twelve special rules and three reducers;
the assumptions lose fifteen hand-written arithmetic axioms and gain a
well-studied three-axiom base with an external oracle.

### 4.4 Infrastructure

| today | V2 |
|---|---|
| CERT.md dialect (change/exact-conv, base+patch, validators, Runs) | conversion forms become I forms (§4.2); **validators are unchanged** — an E function `valid_P` plus an L theorem `valid_P_sound`, cited per program (GPT-6 §10.2 and CERT.md §4 say the same thing) |
| STORAGE S1/S2 (text caches, hashed closures) | native: L terms are hash-consed DAGs, environments are content-addressed; S3's arena is the default representation, not a parked item |
| `tools/prove` ladder, `meta/search` engine, `meta/sketch` holes | tactics and search engines over I and over L terms with native metavariables (§7, §16); LS-laws 1–3 verbatim |
| `tools/impc`, `models/imp`, `models/x86`, `models/linux`, byte-tie, gates | unchanged E libraries and unchanged gates; their theorem *statements* verbatim under the crosswalk (§12); their proofs re-derived |
| `tools/shardfmt`, `tools/canon`, `tools/digest` | s-expression lexical layer KEPT; new heads added to the canon grammar |
| `eval.shard` → `check.shard` tower | retired as the mandatory route; retained as the differential route (§8) |
| `tools/explain`, tracer | rebuilt over I goal states (§16.3) |
| `meta/invoke/prepared.shard` (import cycle) | an opaque prepared-context type in meta/invoke's own interface; exercised by test T6 (R5) |

---

## 5. E — the executable fragment and the static-lowering law

This section answers the HOF concern directly. **L has lambdas,
quantifiers, and higher-order functions without restriction. E does
not have function values, ever.** The rule that makes both true at
once is that the surface's higher-order conveniences are eliminated by
elaboration (§2), so "first-order" is a property of every E program,
not a restriction on what an author may write.

### 5.1 Types (E-types)

An E-type is: `Nat`, `Int`, `Bool`, `Char`; a `type`-declared inductive
(parameters only, no indices) applied to E-types; a `Subtype` of an
E-type whose predicate is an E function (`refine`; the proof is
erased); `structure`s whose fields are E-types. No `Sort`, no `Pi`
value, no `Prop`-carrying data at runtime. Indexed inductives with
erasable indices are deferred (Q4 RULED: one indexed L interface
realized through an ordinary E representation is tested early; no
intrinsically indexed execution IR in v1).

### 5.2 Terms (E-bodies)

Constructor application; `match` with exhaustive patterns over
E-types; fully-applied calls to `fn`s and to library primitives; `let`;
`if`; literals. **`if` takes a decidable proposition**, as in Lean:
`(if (<= a b) x y)` in a `fn` elaborates through the `Decidable`
instance for `Int.le` to the E function `Int.ble`, so a program and a
theorem write the same `<=` (§6.4). `decide` on a decidable proposition
is likewise permitted and lowers to the Bool its selected decision
procedure computes — that procedure must itself be E (R1); for the
core types the instance is fixed by the library, not a consequential
selection in the sense of §6.2. Proof-typed arguments are permitted and
erased. No `Classical.choice`, no proof used computationally, no
`Sort`- or `Pi`-typed value in any position of an *elaborated* E body.

### 5.3 The lambda profile: static forms, eliminated by elaboration

**Supported in v1, in this order of introduction** (Q5 RULED
2026-09-06; GPT-6 R2/R15 accepted):

1. **Closed lambdas** as template arguments: `(map (fun x (+ x 1)) xs)`
   becomes a named helper plus a specialization of `map`.
2. **Named partial application** to lexically known E arguments.
3. **Non-escaping value capture**: `(map (fun x (+ x k)) xs)` with `k` a
   `let`- or parameter-bound E value becomes `map_k k xs` by lambda
   lifting — the captured value is an ordinary **runtime parameter**,
   never a specialization key; one code shape per static lambda, not
   one per value of `k`. Captures are variables already bound by a
   strict `let`, never expressions, so evaluation points are preserved
   trivially.
4. **Templates**: a `fn` may declare function-typed parameters; every
   use in an artifact's closure passes one of the forms above; the
   elaborator specializes per distinct *static* argument tuple. The
   theorem about `map f xs` is proven once in L over arbitrary `f`;
   each instance is definitionally the specialized body — where
   definitional equality does not suffice (a lifted capture changes the
   arity), the transformation's propositional equation is generated
   alongside (R15).

**The escape rule.** A function-typed expression may occur only as the
head of a call or as an argument in a template position; never in a
constructor field, a return position, a data structure, or across a
call whose callee is not statically known. No E inductive has a
function-typed field; no `fn` returns a function type.

**Enforcement.** The fragment classifier runs at `fn` declaration
(loud, early, untrusted); the lowering re-checks the post-specialization
closure (the authority — an artifact that would need a function value
is refused, never approximated). Successful evaluation under `ev` is
not a realization: `ev` only ever sees post-elaboration E, so there is
nothing to fall back to. A `def` that happens to satisfy E is still not
a `fn`; the keyword is the author's declaration of intent to lower.

**Finite specialization.** Runtime totality does not imply finite
monomorphization: `f[A](n,x)` recursing at `f[List A]` decreases its
measure and still generates an unbounded family. Specialization keys
distinguish static code identity from dynamic values; the elaborator
refuses non-terminating or over-budget specialization loudly, as Rust
reports its recursion limit; an exhausted specialization budget is a
refusal, not a proof the program is unlowerable.

**Totality through templates needs no new rule.** Every E definition is
total by L admission. A template is total for every argument of its
function type because the argument is a parameter, not a call it can
recurse through; a named `fn` that tries to recurse through a template
it is passed to is refused at its own admission, as Lean refuses an
unapplied recursive reference. The classifier adds no totality logic of
its own.

**The dynamic tier is a named door, not a v1 feature.** Escaping
closures, function values in data, and dispatch on a function chosen at
runtime need a runtime representation. No rung named needs them; every
real use in this tree defunctionalizes into a sum type plus a dispatch
`match`, which is issue #12's sealed variants and the ratified
alternative. If it opens, the mechanism is bounded and C-like — a
closure is a counted cell with a code index and captured fields, an
indirect call is a jump table — and it opens as a separately priced
profile with its own MEMORY.md rung. **Wake condition (R15):** a named
workload with a demonstrated cost or compositional disadvantage under
manual sealed variants — not a proof that defunctionalization is
impossible; possible and worthwhile are different questions.

### 5.4 One evaluator, the correspondence it must satisfy, and evaluation reflection (R9)

The reference evaluator `ev` for E is one shard program (today's
`eval.shard` `ev`, environment machine with TCO, **fuelled**: a
structurally recursive E function over program data and a budget). It
is the definition of "run". Its relationship to K is **not** literal
agreement with `K.whnf` (well-founded definitions do not reduce
definitionally, and whnf stops at an outer constructor while `ev`
computes a value). The contract:

- `ev` computes by the defining equations `f.eq_N`; each is a theorem
  of L (§2). `ev`'s correctness statement is two-sided: for E-typed
  inputs, `ev p args n = Some v` implies `f args = v` in L, and under
  adequate fuel such a `v` exists. Progress and correctness are distinct
  obligations; `f x = f x` never justifies a looping implementation.
- Execution-route conformance (`ev` interpreted by Rust, `ev` compiled,
  K's whnf where it applies) compares *results and observations under
  explicit resource conditions*, never budgets or representations.
- **Evaluation reflection, the mechanism.** A native value is not a
  proof (R9). The evidence for a particular result is
  `rfl : ev p args n = Some v`, which K *does* establish by definitional
  computation because `ev` is fuelled and structural — unlike the
  measure-recursive `f` it interprets. The general correctness theorem
  then yields `f args = v`. The I form carries `p`'s resolved identity,
  the arguments, the fuel and the result; K recomputes `ev` through its
  whnf with the Nat-literal acceleration. **No native oracle is
  introduced**: the cost is that K interprets the interpreter, which is
  slow until route 1 (§8) compiles K and makes its whnf native. A
  validated accelerator for `ev` inside K is possible later, with
  exactly the fixed-identity validation and conformance treatment the
  Nat operations get, and would be recorded as explicit execution
  trust, never as a footnote. Lean's `cbv`/`decide_cbv` are the
  proof-producing precedent per current documentation; this is not a
  feature Lean lacks, it is the mechanism shard commits to for E.
- Evidence is bound to the resolved program body (its hash), arguments,
  result encoding, environment, and the realization equations. Because
  `f.eq_N` are generated from the hashed body at elaboration, "a stale
  executable view with fresh equations" is a manifest-drift state the
  manifest refuses (T1), not a runtime state.

### 5.5 Totality

Every `fn` is total; that is a *theorem* of its admission (its
recursor or its `WellFounded.fix` with a checked descent proof). The
`TOTALITY.md` regime carries over as elaboration: the author writes
`(measure E)`; the elaborator emits the descent obligations; they are
discharged by tactics (untrusted) into I and then proof terms K checks;
the offline `admit` classifier stays advisory and out of the trust
path. Unbounded processes take Int fuel, as today. Structural descent is
compiled to recursors and needs no obligation.

### 5.6 Erasure and executable-view obligations (R1)

E's erasure is far simpler than general dependent extraction, but it is
a specified transformation with obligations, not "trivial":

| construct | obligation |
|---|---|
| proof arguments and proof fields | removed; executable behavior cannot depend on them because E bodies may not eliminate `Prop` into data |
| subtypes | carrier = the base E-type; introduction obligations are checked at elaboration, never at runtime |
| `decide` | the selected decision procedure is an E function; a Bool result type does not establish that |
| dependent casts | none in v1 E (no indices); if Q4 opens, each cast erases or is refused |
| impossible branches | removed only on checked evidence (a `noConfusion` or `absurd` term), never on a pending claim |
| structural / well-founded recursion | the executable structure is the source body; the L meaning is the recursor/`WellFounded.fix` elaboration; the bridge is the equation lemmas |

Test T1 (§10) exercises one of each before any compiler closure moves.

### 5.7 Replacement is allowed with evidence, never silently (R15)

Shard does not forbid independently optimized implementations — the
whole refinement chain *is* evidence-backed replacement. What it
refuses is an unaccounted gap between the checked meaning and the
deployed computation. A transformation carries source identity,
destination identity, preconditions, the **relation preserved** (exact
equality, representation simulation, an error bound, preservation of
selected effects or resources — not interchangeable Booleans), and
evidence. Two approximate passes need a composition argument for their
combined error; `O(p) = O(q)` licenses no replacement outside the root
observation; the root/spine/congruence distinctions of `SEARCH.md`
carry. Linking compiled E implementations of K, `ev` or `meta/` into an
application is valid behavior; silently inserting an interpreter
because a requested lowering failed is not.

### 5.8 What E buys the lowering program

Nothing in `IMP.md`, `MEMORY.md`, `X86.md` or `COVERAGE.md` changes
its object: the generic spec→imp compiler consumes post-specialization
`fn`s, which are the same first-order total programs as today, now with
a precise membership test. The counted heap, calls and stack, the
`except` clause, register allocation — all unchanged rungs. What
changes is the *certificate* side: Theorem A/B are theorems in L with
proof terms, and B's per-function certificates are produced by tactics
emitting I rather than by a text generator (§12, §16).

---

## 6. Elaboration and the surface — the real project

### 6.1 The elaborator (untrusted, meta/)

Staged, each stage useful on its own:

- **Stage 0 — explicit terms.** Fully explicit L terms in s-expression
  form, checked by K. Enough for the oracle gate and for generated
  proofs (validators, reflection). Painful to write by hand; that is
  fine for a quarter.
- **Stage 1 — inference.** Implicit arguments, first-order unification
  with metavariables, universe inference, `let`/`have`, structural
  recursion compilation to recursors, `match` compilation, definitional
  equations (`fn.eq_N`), `noConfusion`/`injection` derivation, the
  fragment classifier, lambda lifting and specialization (§5.3).
- **Stage 2 — the proof IR and its tactics.** The I elaborator (§16)
  and a goal-state framework with a named local context; the core
  tactics `intro apply exact rfl rw simp only cases induction have
  show change decide`, arithmetic reflection, `termination_by`/
  `decreasing_by`. The `tools/prove` ladder and the search engine
  become I producers in this framework.
- **Stage 3 — typeclasses and coercions.** Needed for law-bearing
  hierarchies and for Mathlib-style statements; also the point where
  `Decidable` bridges Bool and Prop cleanly. Explicit instantiation
  stays the preferred spelling; consequential selections are scoped and
  recorded (§6.2).
- **Stage 4 — the compatibility layer.** No longer a separate layer:
  I's v1 vocabulary *is* today's proof roster generalized (§16.2), so
  the port of existing proofs is a re-spelling into I plus agent
  re-derivation where the old semantics were rewriter-shaped (§14.3).

### 6.2 The surface S, and the resolved requirement (R11)

S stays **s-expressions** — `shardfmt`, `canon`, `digest`, the corpus
gate, and the LLM-fluency of the lexical layer all rest on it — and
its *forms* become Lean's names and shapes. Schematic (not final
syntax):

```sexp
;; today
(claim len_append
  (goal ((xs (List T)) (ys (List T))) ()
        (= (len (append xs ys)) (+ (len xs) (len ys))))
  (induct xs ((case Nil (steps ((simp both)) refl))
              (case Cons (steps ((simp both) (rewrite (hyp ih) lr lhs true ())) refl)))))

;; V2 (schematic; names under the naming law of §6.4)
(theorem List.length_append ((xs ys (List α)))
  (= (List.length (List.append xs ys)) (+ (List.length xs) (List.length ys)))
  (by (induction xs)
      (case nil (simp [List.append List.length]))
      (case cons (x t ih) (simp [List.append List.length ih]) omega)))

;; a statement today's logic cannot state, in the same surface
(theorem dot_bound ((n Nat) (u v (Vector Real n)))
  (-> (forall ((i (Fin n))) (<= (abs (get u i)) 1))
      (<= (abs (dot u v)) (norm1 v)))
  (by ...))

;; the executable/logical split is one keyword
(fn  List.length ((xs (List α))) Nat (measure (struct xs))
  (match xs (nil 0) ((cons _ t) (+ 1 (List.length t)))))
(def Sorted ((xs (List Int))) Prop
  (forall ((i j (Fin (List.length xs)))) (-> (< i j) (<= (List.get xs i) (List.get xs j)))))

;; a static lambda: eliminated by elaboration, no function value in E;
;; `if` on a decidable proposition, the same `<=` as in a theorem
(fn add_offset ((k Int) (xs (List Int))) (List Int)
  (List.map (fun (x) (if (<= x 0) x (+ x k))) xs))
```

**Reconstruction versus selection (R11).** Concise source and fully
resolved meaning are separate views. Inference may *reconstruct*
information determined by the declared inputs and expected type (a
list's element type, a universe level). It may not *select* among
semantically consequential alternatives — an ordering, a numeric
interpretation, an implementation package, a coercion route — except
under a declared scope, and every such selection is recorded in the
**resolved requirement**: proposition, bound parameters, referenced
declarations, selected instances and coercions, and policy/environment
dependencies. A source edit or import change that alters any of these
is a *different requirement* and is reported as such; changing only a
proof strategy leaves it unchanged. Unknown identifiers never become
parameters (no auto-bound implicits); explicitly declared synthesis
holes (§7) remain fillable without changing the task. The
proof-solving agent does not get to redefine the target while solving
it. *Q6 RULED: no Lean concrete-syntax reader in v1.*

### 6.3 The LLM-first gate

An agent with no memory files and no gotcha list, given `LANGUAGE.md`
and `docs/LEAN.md` (§6.4) and nothing else, proves a `tb_len`-class
theorem (§14) and writes a `fn` that lowers. That gate, not line
counts, is the measure of the surface; it runs early on small tasks
and again at the end. Structured exhaustion and replayable evidence
are the guarantees; fast completion is not. If `LEAN.md` has to grow
past a few pages for the gate to pass, the seam is in the wrong place.

### 6.4 The naming law (RULED 2026-09-06)

The user's question: "as the primary author of shard source within the
overall system we are building, what would be the least confusing
paradigms to adopt here? If the answer is that Lean is already the
ideal form, then I am fine with that, but I would rather it be an
explicit decision." The decision, explicit: **Lean's names, conventions
and naming grammar by default**, because they give the author a
*complete* mental map — above all the theorem-naming grammar
(`List.length_append`, `Nat.add_comm`, `Int.emod_nonneg`, the `_of_`,
`_iff`, `_left`, `_self` suffixes), mechanical enough that a lemma
never seen can be cited by guess, which is the guessable-names clause
of the LLM-first principle already solved. A second full vocabulary
would put a translation at every token; the map in `docs/LEAN.md` must
fit in dozens of rows. Each departure below is one explicit decision.
The damascene precedent is the model: the common things under the
names the agent expects, the parts that fail scrutiny refused with a
pointer, one short document for the seam.

**Adopted from Lean, as the discipline's consensus rather than as
Lean-isms:**

1. **Names and namespaces.** `List.length`, `List.map`, `Option.getD`,
   `Subtype.val`, `Fin`, `decide`; namespaces are shard's qualified
   identities (module paths). `use` brings short names into scope as
   `open` does; the resolution is recorded, so canonical form may keep
   the short spelling.
2. **Lowercase constructors and Bool literals.** `some`, `none`,
   `cons`, `nil`, `true`, `false`. Today's capitalized `Some`/`Nil` and
   Bool constructors `True`/`False` collide head-on with Lean, where
   `True` is a proposition; this rename removes a whole class of
   confusion and is mechanical.
3. **One set of connectives, propositional first.** `=`, `<=`, `<`,
   `and`, `or`, `not`, `->`, `iff`, `forall`, `exists`, `fun` — Lean's
   own ASCII spellings. E's `if` and `decide` bridge through
   `Decidable` (§5.2), so a `fn` and a `theorem` write the same
   `(<= a b)`. `==`, `&&`, `||` exist for Bool *values* and are rarely
   needed. `int_eq`, `le`, `lt`, `andb` and their kin disappear.
4. **Operators and conventions** (Q9, §12.1): `/` and `%` on `Int` are
   Euclidean, `Int.tdiv`/`Int.tmod` the named truncating forms; `Nat`
   subtraction saturates; words wrap.
5. **Sizes and indices are `Nat`.** Today lengths are `Int` with
   nonneg refinements and lemmas; Lean's choice simplifies our own
   totality regime — a `Nat` measure needs no nonnegativity obligation,
   which deletes half of today's measure work — and `omega` handles the
   `Nat`/`Int` mixing. The one adoption that is an outright improvement
   on what we have.
6. **Theorem naming grammar and equation lemmas.** Mathlib's
   convention; `f.eq_N` and `f.eq_def` for generated defining equations.

**Deliberate departures, each with its reason:**

1. **Declaration keywords are Rust-flavored:** `fn`, `def`, `type`,
   `inductive`, `structure`, `sig`, `theorem`. `fn` versus `def` is the
   E/L split, a concept Lean lacks; Rust is the other corpus the author
   carries, so the keyword reads correctly on sight.
2. **The surface is s-expression prefix form:** `(<= a b)`,
   `(List.map f xs)`, `(match xs (nil …) ((cons h t) …))`. The canon
   and tooling investment; the map is trivial (Q6).
3. **No effect notation.** No `do`, no `IO`, no monads; World threading
   stays explicit because that is what makes a `bin` an honest
   artifact. One paragraph in `LEAN.md`.
4. **Refusals, each with a pointer to the replacement** (§13.1):
   `get!`-style default-valued access (→ `get?`, `getD`), `Inhabited`
   defaults, `partial` (→ `measure`), `unsafe`, `implemented_by` (→
   evidence-backed replacement, §5.7), auto-bound implicits (→ explicit
   binders), declaration-order instance selection (→ scoped selection,
   §6.2). An agent who writes the Lean form is refused with the shard
   form named — the hardened-error clause of the LLM-first principle
   applied at the seam.
5. **Shard-only vocabulary**, with nothing to map to: `measure`,
   `mod.req` and `sig` views, `requirement`/`fulfills`, `bin`,
   `trusts`, `requires`, World externs, models, the artifact-claim
   forms.

**`docs/LEAN.md`** is the seam document, written at phase 3 and kept
short: (i) what is the same as Lean and needs no explanation; (ii) what
is refused, why, and what to write instead; (iii) what is shard-only.
It is an input to T9 and to the porting context pack (§14.3).

**Corpus impact.** Constructors, Bool literals, `std/list` and
`std/order` names, comparison and equality functions, and `Int`-typed
sizes all change; every one is a mechanical rename the migration tool
performs before any agent sees a file (§12, §14.3 tier 0). The measure
regime gets simpler. The names in this document's own examples are to
be read under this law.

---

## 7. Metavariables, sketches, search (D11/D12; R3, R13)

L terms carry a native metavariable node with a declared local
telescope, a dependent expected type, and, at each occurrence, an
explicit typed substitution from the occurrence context into that
telescope (`?h : [Ψ ⊢ A]`, `?h[σ]`). The same mechanism serves I (§16).
K refuses any declaration containing one. The engine — in meta, not in
K — exposes:

- hole declaration, occurrence, assignment (checking scope, dependent
  expected types, universe constraints, and direct/indirect cycles
  through terms, types and substitutions; fresh subholes only under the
  acyclic dependency discipline);
- open validation with five distinct outcomes: *open construction
  validated*, *blocked on obligations*, *invalid construction*,
  *resource exhausted*, *closed evidence accepted*. Open validation
  never asserts a filling exists; a hole of type `False` is an unsolved
  obligation; generalizing a hole into a hypothesis changes the task
  and never satisfies the original closed requirement;
- final closure: the fully instantiated declaration is rechecked by K
  (cached open results are not trusted).

**Transactional invariant (R13).** An unsuccessful speculative
operation leaves the caller's semantic workspace unchanged unless it
returns an explicit patch the caller elects to commit:

```
attempt(snapshot, request, limits) → Completed(patch, evidence)
                                    | Blocked(obligations, proposed_patch)
                                    | Invalid(reason)
                                    | Exhausted(resource)
commit(snapshot, checked_patch) → snapshot'
```

A thrown failure or a timeout never leaves a hidden assignment.
Context-sensitive caches bind the environment, local telescope,
universe and metavariable assignments, and policy; a result from one
branch never constrains another. Persistent values and rollback
journals are both admissible implementations. The invariant governs
semantic workspace state only; host effects stay outside speculative
logic.

The substitution/closure obligation — a validated open derivation with
a well-typed filling of its residual obligations instantiates to a
valid closed judgment — is specified for shard's representation;
Lean's metavariables are the design reference, not a proof that this
obligation is discharged. It matters beyond admission: an unsound
pruning result discards solutions without ever producing an invalid
theorem, which K cannot catch.

`meta/sketch`'s reserved-call holes migrate to native metavariables;
sharing one metavariable at two positions is the correlation primitive
it already has, now binder-safe. Exact rank/count, stratified grammars,
dominance pruning, "blocked is not UNSAT", and the observer-profile
distinctions of `SEARCH.md` stay library policy; a theorem about all
fillings of a template under stated constraints is an ordinary
quantified L theorem and does not establish that a filling exists.
Search APIs keep candidate correctness, exact enumeration, region
emptiness, representative replacement and optimality as separate
claims. Persistent workspaces and branch merge semantics are deferred
to the meta layer's own ledger; branch/snapshot isolation and
context-keyed caches are required from the first implementation. Tests
T4 and T7 (§10) are the pathfinders.

---

## 8. Execution and trust (the bootstrap addendum, answered)

**Rust is the enduring bootstrap facility, not an architected-in
component** (user, 2026-09-05). It implements E's operational semantics
and does not define a competing source language or any L proof rule.
It is kept so that a fresh machine can run shard from Cargo and
sources. It never executes anything but E, and it needs no image format
beyond what E's own loader produces. Because K, `ev`, the elaborator
and the tactics are E programs (§2), the routes that execute K are, in
order of preference as they come online:

1. **K compiled** by shard's own lowering — a proven artifact, the
   Candle-shaped end state; a consumer of the coverage program, not a
   prerequisite of this one.
2. **K interpreted by `ev`**, `ev` itself compiled.
3. **K interpreted by the Rust bootstrap directly** — legitimate only
   because K is E and Rust evaluates E; this is today's `eval direct`,
   costs nothing architecturally, and removes the tower tax while
   route 1 is built.
4. **The full tower**, Rust interpreting `ev` interpreting K — the
   differential cross-check, never the mandatory route.

The four routes share one checking logic; **their execution
dependencies differ by route and are recorded** in the acceptance
manifest. The logical rule authority does not change by choosing among
them: Rust has always been trusted-by-review as the executor of shard
programs (`TCB.md` lists it first), `eval.shard` was never verified
either, and the C-class dissolution is a law about artifacts that
stands untouched.

**The cold bootstrap route, concretely.** The V2 toolchain's own
sources (K, `ev`, the loader, the elaborator) are written in the
*narrow-compatible* E profile — first-order, no surface sugar that
needs elaboration — exactly as `kernel/*.shard` is today. The Rust
loader reads them as it reads the kernel now; the loaded toolchain then
resolves, elaborates and checks everything else. No new Rust capability
is required for a clean checkout to reach a running K. K's own inline
proofs are checked by K once it runs; until then they are pending
claims under the reviewed host, never installed as assumptions.

Positions on the addendum, in that light:

- **B06 accepted**: a reviewed, uncertified Rust evaluator may execute
  K for acceptance-grade verdicts. Not new trust; a claim that Rust
  itself is verified is a separate claim with separate evidence.
- **B07/B08 rejected as written.** Rust executes E, nothing else;
  parsing, resolution, granted interfaces, use scopes, elaboration and
  typing happen in shard, once.
- **B11 accepted.** The tower is retained as route 4, the differential.
- **The oracle is not a Rust question.** Differential testing against
  Lean happens on exported terms fed to K; it needs no Rust parsing.

---

## 9. Positions on GPT-6's decisions D01–D13, B06–B16

(R1–R16 are answered in §17.)

| ID | GPT-6 v0.3 | this proposal |
|---|---|---|
| D01 | Shard-owned, Lean-informed dependent foundation; departures allowed | **Lean-rule-exact K**; departures are dated decisions priced against the oracle; zero in v1 |
| D02 | versioned rule package | accepted; declarative rules and the bounded procedure specified separately (§3.1) |
| D03 | fixed conversion, no equality reflection, budgets | accepted; explicit-conversion policy as elaboration discipline; tactic transparency is not opacity and not conversion (§4.2) |
| D04 | E0 executable view, erasure as a theorem-bearing pass | **replaced by E** (§5): a syntactic fragment with a keyword, not a second IR; erasure is a small specified transformation with the obligations of §5.6, and the L/E correspondence is by defining equations (§2) |
| D05 | intensional program identity | accepted (QName + content hash; statement, evidence and assumption identities distinct; validators and observation relations unchanged) |
| D06 | proof graphs from day one | accepted; native in K's term representation; I is the navigable layer above (§16) |
| D07 | Lean math coverage as long-term target; transport optional | **the kernel check is cheap under rule-exactness; integration is priced work** (§1.3, Q7); the parity gate is the arc's measured goal |
| D08 | migrate meanings, rewrite proof text | accepted; statements under an explicit crosswalk with pinned hashes and per-interface migration records (§12) |
| D09 | embeddable engine, small authority | accepted as design constraints on K and the elaborator (environment as a value, no I/O); the minimum prepared-invocation contract is exercised early by T6 |
| D10 | one identity system, environment vs workspace | accepted |
| D11 | native contextual holes with dependent telescopes | accepted as §7: Lean-style metavariables with a specified substitution/closure obligation, five-way outcomes, and transactional state, in meta |
| D12 | reusable partial-construction proofs | accepted as ordinary quantified L theorems; T4/T7 |
| D13 | runtime linking of engine/meta | accepted in principle: an application may link compiled E implementations of K, `ev` and `meta/`; never implicit, never a fallback from failed lowering; API deferred |
| B06 | uncertified Rust as acceptance authority | accepted |
| B07 | expand Rust execution capability | **rejected**: Rust executes E, nothing else; it is the bootstrap facility, not architecture (§8) |
| B08 | share resolved frontend artifacts where economical | **moot**: there is one front-end, in shard; Rust never parses or resolves |
| B09 | provisional execution without admission | accepted |
| B10–B12 | conformance, CI cadence, rollout | accepted in spirit; the ladder is §10/§14 |
| B13 | public prepared invocation | accepted as T6 with the opaque prepared-context fix |
| B14 | hole semantics in shard, not Rust | accepted (trivially: Rust has no semantics) |
| B15 | branch isolation / cache keys | the basic isolation is required from the first implementation (§7); merge semantics deferred |
| B16 | runtime linking bounded | accepted with D13 |

---

## 10. Gates and the acceptance battery

The ladder itself is the arc plan in §14 (phases 0–7). This section
fixes what each gate measures. Before interpreting any result, the
count is agreed: source lines, unique proof nodes, warm checking work,
cold loading, peak memory, emitted code, or author interventions — never
one number for all costs.

| test | experiment | what failure reveals |
|---|---|---|
| **T0 — oracle** | K checks `Init`'s export declaration-for-declaration; accept/reject verdicts and axiom closures identical to Lean's; the supported input/version scope logged; a hostile battery, each member with an independently specified *declarative* reason for rejection (universe collapse, scope capture, forged recursor, non-positive inductive, `Prop`-to-data elimination, cyclic definition, a same-spelled non-core `Nat.add`, the six 2026 exploits restated) | a rule mismatch, an input-mapping error, or a budget difference; disagreement is investigated, never settled by tool name |
| **T1 — realization** | one structural fn, one measure fn, one subtype-producing fn, one `decide` case: L meaning, executable view, and the equation bridge established; a manifest whose body hash and equation set disagree is refused; a progress case is retained | a missing erasure/recursion/representation bridge; manifest drift |
| **T2 — static lambda** | a closed lambda, a named partial application, and the captured-value `add_offset` example lower to the first-order profile; the theorem `map_k k xs = map (fun x (+ x k)) xs` holds by specialization or by the generated equation | source restriction mistaken for artifact restriction; wrong capture or specialization |
| **T3 — bounded specialization** | the type-growing recursion example and a large finite specialization workload refuse or complete within agreed limits, loudly | runtime totality mistaken for compiler termination or acceptable code size |
| **T4 — contextual holes** | a shared hole under renamed binders; a dependent expected type; a blocked comparison; final closure; one theorem reused across fillings; **failure after a tentative assignment and exhaustion after partial progress leave the snapshot unchanged; forked branches assigning the same originating hole do not leak** | a native node without a coherent open-construction discipline; hidden state |
| **T5 — abstraction** | a consumer checks with the public view alone and with the implementation linked, typing and obligations stable; a proposition that secretly depended on a private equality is rejected at the view | invalid environment weakening |
| **T6 — embedding** | an in-process client constructs a declaration, prepares once, invokes repeatedly, transforms with `meta/`, checks a claim, and requests a lowering, under one declaration identity; preparation count and marshaling measured; **an implementation change invalidates prepared/inlined execution but not abstract client proofs** | CLI dependence, duplicate environments, hidden preparation cost, wrong invalidation |
| **T7 — search fidelity** | a small ground-truth candidate space with correlated holes; a root-only observer rewrite invalid in a nested position; a heuristic timeout that must not count as UNSAT; **a cache entry reused under a changed context is refused; two error licenses or two root-only equalities do not compose into an unrestricted equivalence** | wrong pruning, counting, or reuse of observational equalities |
| **T8 — replay and trust** | cold replay of I, terms, assumptions, declaration identity and artifact bytes on reviewed Rust-hosted K; `Exhausted` never yields a receipt; the execution route is recorded; **an evaluation-reflection result with a tampered value is refused, and correct evidence replays cold without the originating tactic; the same proposition proved under permitted assumptions and under a prohibited axiom — the second fails policy** | pending evidence, stale caches, an unstated trust transition, a native-result oracle |
| **T9 — the LLM-first gate** | §6.3, with `LANGUAGE.md` + `docs/LEAN.md` as the only inputs, early on small tasks and again at the end; **a misspelled requirement identifier fails; an import that changes a selected ordering is reported as a requirement change; changing a proof strategy leaves the resolved requirement unchanged; a declared synthesis hole remains solvable; a refused Lean form is answered with the shard form** | a surface that needs folklore; silent task drift; a seam document that cannot stay short |
| **T10 — proof-IR navigability** | §16.4: a search engine reads a pinned I, elaborates a prefix to its goal state, enumerates applicable I forms, fills a hole, and the result replays; an I-level solution found by one engine is checked identically by another | an IR that only its producer can read |

Performance decision rule (GPT-6 §16.1, accepted): baselines and
budgets are agreed before each rung runs; no speedup is inferred from
architecture. Conversion is **instrumented** (unique evidence nodes,
conversion work, cold/warm cost, repair after a small change) on one
compiler certificate, one mathematical theorem and one contextual-search
operation before any conversion extension is considered (R16).

---

## 11. Q1–Q9 — rulings

Q1–Q8 were confirmed by the user on 2026-09-06 ("Your leans sound
reasonable"; sibling tree in §14). Q9 is ruled under the user's
delegation of cross-discipline defaults ("I would defer to your call in
cases like this for which position is most reasonable given shard's
aims", 2026-09-06). All are subject to the ratification of the whole.

- **Q1 — Exactness.** No intentional departures in v1. Pin the release;
  reconcile the thesis, Lean4Lean and the kernel; reconcile the Nat
  acceleration list against the pin and bind each entry to a fixed
  declaration identity; resource outcomes and open-engine services are
  specified separately.
- **Q2 — Sidecars.** Sources carry tactic blocks; the build elaborates
  tactics to **I**, I to terms, and K checks the terms; the sidecar pin
  is **I plus the checked term DAG's hash** (§16.5), replayed per
  LS-law 1 without re-running any tactic. Statement, evidence and
  assumption identities stay distinct.
- **Q3 — Int and words.** Follow Lean: `Int` inductive over `Nat`,
  `UInt64`/`BitVec` as structures over `Fin`, kernel acceleration on
  `Nat` only. `std/word` maps onto these; every existing operation gets
  a crosswalk row (§12) validated on division, remainder, shifts,
  widths and boundaries — shared names prove nothing.
- **Q4 — Indexed E-types.** Deferred; one indexed L interface realized
  through an ordinary E representation is tested early.
- **Q5 — Lambdas.** The static profile of §5.3, eliminated by
  elaboration; no interpreted-only `fn`; the dynamic tier is a named
  door with sealed variants (#12) as the ratified alternative and the
  R15 wake condition.
- **Q6 — Second syntax.** No Lean concrete-syntax reader in v1; an
  explicit-term importer for exports is enough.
- **Q7 — Mathlib.** Small exports early as an independent input source
  (T0); larger replay and definition bridging priced; reals/intervals
  for Arc C is the first integration consumer; library breadth never
  displaces the shard-specific tests.
- **Q8 — Coverage ordering.** The arc stays parked; a bounded
  `tb_len`-class rung is pulled forward into phase 4 (§14), after the
  minimal elaboration and arithmetic slices and before any bulk port.
- **Q9 — Mathematical conventions (RULED by Fable, delegated).** See
  §12.1.

---

## 12. Migration, priced

**Layers, by what actually changes:**

| layer | content at `5abc600` | treatment |
|---|---|---|
| code | ~10.6k `fn`/`type` bodies | mechanical surface rewrite by one tool; no judgment |
| statements | ~5.3k goals, 307 requirements, 49 axioms | text verbatim under the primitive crosswalk; **meaning preserved by an explicit per-interface migration record** (R8): old declaration, new declaration, changed definitions, assumption changes, and the evidence or review connecting them; intentional strengthening or weakening is a separate, dated decision; statement hashes pinned in the port manifest |
| proofs | ~4.9k hand claims | re-spelled into I where the old step was I-shaped; solver, cheap agents, strong agents for the rest (§14.3) |
| generated certs | `std/sha256` 23.5 MB, `impgen_*_out`, probe blocks | not ported; superseded by validators and tactic-generated I, which is what CERT.md §1 asked for |
| toolchain | K, `ev`, elaborator, I elaborator, tactics, loader, canon, prove, explain | new code, the arc's own product, front-loaded |
| docs | `TCB.md` roster (§4.3); `TOTALITY.md` → the E-totality section here; `CERT.md` §3/§7 superseded, §4 carried; `LANGUAGE.md` §10 → the tactic surface and I; `SEARCH.md`'s LS-laws carried and §16 added | rewritten or carried, per file |

### 12.1 Conventions: what to copy from Lean and what not (Q9, R14)

The user's rule (2026-09-06): copy a convention when it is the
discipline's consensus and right for shard's aims, not because Lean
did it — "if we were making our own Matlab I would still emphasize
that indexing starts at 0." Applied:

**Copy — the totalizations of the mathematical primitives.** `Nat.sub`
saturates at zero; `Int.ediv x 0 = 0`, `Int.emod x 0 = x`, likewise
`tdiv`/`tmod`; fixed-width words wrap. These are not Lean's quirks but
the cross-prover consensus (Isabelle, HOL, Coq's `Nat` all define
`n / 0 = 0`), and they are right for shard for three reasons: the
lemma library becomes unconditional where it can be (`(a*b)/b = a`
needs `b ≠ 0` anyway; `(a+b)-b = a` does not); every imported statement
keeps its meaning; and shard's honest signal for a violated invariant is
a **proof obligation**, not a runtime panic — Rust panics on `x/0`
because Rust has no proofs. The compiled-artifact consequence is
stated plainly: a V2 program that divides by zero returns zero, where
today's compiled chains trap. "Division by zero" is therefore **not a
fail family** under MEMORY.md D8; the families stay overflow, oom,
stack, which are properties of the *lowering*, not of the mathematics.
A program that wants to fail on a zero divisor says so with a named
operation.

**Provide, as separately named E operations with bridge theorems (R14):**
`checked_div : Int → Int → Option Int`, `div_pos (x y : Int) (h : 0 < y)`
and their kin for subtraction underflow, lookup, and finite-width
overflow. They never reinterpret the primitive; migrated statements
mean what they meant. Proved preconditions may delete the checks
through an established realization.

**Do not copy — application-facing conveniences that hide failure or
choice:** `panic!`-style default-valued indexing (`a[i]!` with an
`Inhabited` default); `Inhabited`-backed `opaque` defaults; `partial`;
auto-bound implicits (§6.2); instance selection by declaration order
without a declared scope (§6.2). E indexing is `Option`-valued or
proof-indexed; a default-valued read is the named `getD`. The full
refusal list with pointers is §6.4.

**Keep ours where ours is stronger:** floats (`docs/FLOATS.md`'s proven
`std/f32`/`f64` versus Lean's opaque `Float`), `Str` as
`(refine Bytes utf8_valid)`, the measure regime, `mod.req`; Lean's
counterparts are mapped for import only.

**The migration table.** Under the naming law (§6.4) there is no
surviving "shard spelling": the right column *is* the language, and
the left column exists only for the migration tool and the
per-interface migration records. Exceptional-input behavior is part of
each operation's meaning and is recorded per row. Initial rows; every
row validated in phase 3 before any statement is declared migrated:

| old spelling (at `5abc600`) | V2 spelling | exceptional behavior | note |
|---|---|---|---|
| `+ - *` on Int | `+ - *` (`Int.add/sub/mul`) | none | |
| `/`, `tmod` | `Int.tdiv`, `Int.tmod` | `x/0 = 0`, `tmod x 0 = x` (today: stuck / trap) | truncating, now the *named* form |
| `ediv`, `mod` | `/`, `%` (`Int.ediv`, `Int.emod`) | `x/0 = 0`, `x % 0 = x` (today: stuck) | Euclidean, now the *operator* |
| `int_eq`, `le`, `lt` (Bool fns) | `=`, `<=`, `<` (Prop; `Decidable` in E), `==` for Bool values | none | the Bool/Prop split leaves the surface |
| `andb`, `orb`, `notb` | `and`, `or`, `not` (Prop; decided in E); `&&`, `\|\|`, `!` for Bool values | | |
| `True`/`False` (Bool ctors) | `true`/`false`; `True`/`False` become the propositions | | mechanical rename, corpus-wide |
| `Some`/`None`, `Cons`/`Nil`, `Pair` | `some`/`none`, `cons`/`nil`, `Prod.mk` | | lowercase constructors |
| `len`, `append`, `rev`, `inth`, `memb` | `List.length` (`Nat`), `List.append`, `List.reverse`, `List.get?`, `List.contains`/`List.Mem` | `get?` is `Option`, never a default; `getD` is the named default form | sizes become `Nat` |
| `band bor bxor bshl bshr` (premised `0 ≤`) | `Nat.land/lor/xor/shiftLeft/shiftRight` | none on `Nat` | the 0≤ premises become the `Nat` type |
| `Nat` former (literal-packed) | `Nat` | `Nat.sub` saturates | kernel literal ops |
| `(refine Int nonneg)` etc. | `Nat` where it is a size; `Subtype` otherwise | | |
| `Word`/`U8`… (std/word) | `UInt*`/`BitVec` over `Fin` | wrap at width | Q3 |
| `Bytes` = `(List U8)` | `List UInt8` (opaque `Bytes` kept as ours) | | |
| `(record …)`, `F_of`/`with_F` | `structure`, projections | | |
| `sym_eq`, `chars_of_sym`, `sym_of_chars` | `String`/`Char` | | toolchain-internal today |
| `gen_fresh` | toolchain-internal | | never in a statement |

**The coverage arc's kits** (fra_kit ~900, rth_kit 1,070, tb/tbh
kits): statements under the crosswalk; proofs re-derived in phase 4/5;
the generators (`gen_fra.py`, `gen_rth.py`, the B-1c design) become
I-emitting tactics.

---

## 13. The Lean review — what to borrow, refuse, and improve

Rewritten in v0.3 along GPT-6's structure (follow-up memo §10) with its
factual corrections taken: earlier drafts overstated several
contrasts. The kernel rules are copied exactly by decision (§1.3); a
convention can be right for Lean and wrong for shard without being a
soundness bug, and the differences below are engineering contracts,
not a catalogue of Lean defects.

### 13.1 Defaults shard refuses

- **Silent task drift.** Unknown identifiers never become parameters;
  consequential instance and coercion selections are scoped and
  recorded in the resolved requirement (§6.2). Lean's auto-bound
  implicits and declaration-order instance selection are deliberate
  conveniences we decline as defaults.
- **Ambient trust.** Artifact acceptance includes the transitive
  assumption closure and the connection to the intended bytes (§4.1,
  R12). A pending dependency, a changed statement interpretation, or a
  native computation result is not accepted because a tool reported
  success. Keep the axiom-scope gate, kind tags, per-artifact trusts and
  the ledger; K's native axiom closure makes the ledger cheaper, not
  optional. Lean has `#print axioms` and stored proofs; the difference
  is that policy is *mandatory at the boundary* here.
- **Unaccounted replacement** (§5.7). Not implementation freedom — the
  refinement chain is replacement with evidence — but any gap between
  checked meaning and deployed computation that carries no relation and
  no evidence. Lean's `implemented_by` and `@[extern]` substitute a
  runtime implementation without establishing its relation to the
  logical one and are documented as execution risks distinct from
  logical inconsistency; `@[csimp]` is theorem-directed and is a
  different, legitimate mechanism; `partial` and `unsafe` have their
  own safeguards. Shard's E discipline admits none of the four as
  defaults, and says why per mechanism rather than lumping them.
- **Hidden speculative state.** Meta operations are transactional (§7).
  Failure and exhaustion never leave assignments behind or justify
  pruning; blocked, invalid, exhausted, open-valid and closed-accepted
  stay distinct outcomes (§3.3).
- **Convenience conventions that hide failure** (§12.1, §6.4):
  default-valued indexing, `Inhabited` defaults, `partial`. Every
  refusal is paired with the shard form, and writing the Lean form is
  refused with that form named.
- **Exhaustion as an error.** Lean's heartbeat limit is an error
  indistinguishable from failure; here `Exhausted` is a verdict at every
  layer, K and elaborator alike.

### 13.2 Mechanisms to borrow and strengthen

Borrow dependent abstraction, proof terms, local inference, typeclasses,
broad simplification, classical reasoning, quotients, and theorem-
directed computation; none is undesirable because it is powerful. Per
current documentation Lean already has stored proof terms in compiled
environments, proof-producing evaluation (`cbv`, `decide_cbv`), and an
opt-in module system with public/private scopes and controlled body
exposure; these are references to learn from, not absent features to
claim as inventions.

What shard strengthens is their **integration in one embeddable
engine**:

- **Evaluation reflection with per-invocation evidence** (§5.4): the
  general evaluator theorem plus `rfl` on the fuelled `ev` run, bound to
  the exact program and environment; no native-result oracle, ever.
- **Views as the review surface** (§4.1): interfaces that are
  well-formed environments with implementation matching and evidence
  binding, so a `mod.req` is what a reviewer reads and what a consumer
  is limited to.
- **Fine-grained identity**: requirement, interface, implementation,
  evidence, realization and engine are distinct records (§4.1), so a
  proof-only change re-runs policy without rebuilding code and an
  implementation change invalidates exactly the inlined execution.
- **The proof IR** (§16): a stable, navigable certificate layer between
  tactics and terms, so search engines are first-class consumers and
  pins are replayable without re-running tactics.
- **Cost and observation contracts in the compilation workflow**
  (Runs/RunsWithin, the artifact-claim forms). This is a workflow
  difference, not a claim about what Lean's logic can express.
- **`fn` versus `def`**: executable intent is checkable and the
  lowering's input is a total, first-order program, rather than a
  code generator deciding what compiles.
- **No codata, no partial functions, every equation a theorem** —
  stricter than Lean's totality story, and kept.
- **A route to a certified checker** (§8 route 1) through shard's own
  lowering. E ownership is not a correctness proof by itself; it is
  what makes that route reachable.

### 13.3 Boundaries not reopened in v1

K and the toolchain are E programs operating on L-as-data.
Applications are intended to compile. The static lambda forms elaborate
into first-order E; there is no interpreted-only `fn` fallback; broader
closure representations wait for the R15 wake condition. Classical
principles are not rejected because some uses have no executable
realization — E admission, erasure and realization enforce the
computational boundary. The totality and process policies are not
reopened. Kernel-level departures considered and declined: universe
cumulativity, dropping `String` literals, anything about `Quot`,
induction-recursion, induction-induction, coinduction, higher inductive
types — each priced at "loses differential testing for every
declaration it touches", none with a consumer.

### 13.4 Declarative rules, the checking procedure, and experiments (R10, R16)

The declarative judgments are stated separately from the bounded
checking procedure (§3.1). Acceptance must implement the judgments;
failure to establish conversion is not a theorem of inequality (§3.3);
compatibility with Lean is tested under an identified rule/version/
input profile and supplies no completeness proof.

Conversion is **instrumented before it is extended** (§10). Two
experiments are recorded, both under unchanged v1 rules and off the
critical path: *conversion plans* (a producer supplies selected
unfoldings, instantiations and shared intermediates; K validates with
the existing rules; evidence size measured) and *expected-type-directed
checking* (using known expected types to guide checking of explicit
evidence; Lean4Lean's inference-forward design is a reference, not a
mandate). A later research question — whether some implicit
definitional equalities are better presented as explicit transports, as
Lean4Less explores by translation — is noted as research with explicit
translation and assurance costs, and is not a prerequisite for
anything. No v1 departure is authorized by this section.

---

## 14. The Foundation arc — the plan

### 14.1 Structure: the sibling tree (RULED 2026-09-06)

V2 is built in a sibling top-level tree **`v2/`** in this repository —
`v2/kernel`, `v2/meta`, and, as the details settle, `v2/std` —
committed to main as always, while the existing tree keeps checking the
existing corpus and serves as the oracle for ported modules. Files are
ported across module by module against a manifest; the flip (phase 6)
renames `v2/` into place and archives the old tree in git history. A
separate repository was considered and declined: cleaner for the
archive question, worse for shared CI, tooling and history.
`LAYOUT.md` gains the `v2/` rule at phase 0.

### 14.2 The port manifest: PORT / ARCHIVE / REGENERATE

Every file in the live tree gets exactly one label before any agent
touches it, in a record file beside this ledger:

- **PORT** — the file's declarations move to `v2/` under §12's layers.
- **ARCHIVE** — the file stays in history and is not a V2 obligation.
  Candidates from the slimming census (~560k tokens of closed-arc
  tooling beside ~620k of live core): `tools/wasmgen`, `tools/x86gen`,
  `models/riscv`, `models/pio`, `models/wasm`, the frozen `impgen`
  oracles, diff drivers. Each family is the user's call.
- **REGENERATE** — generated certificate text replaced by validator
  proofs and tactic-emitted I; never ported.

The port is the pruning opportunity the slimming arc was waiting for.

### 14.3 The porting pipeline (bulk work by cheaper agents)

The existing auto-proof-solver architecture with an LLM as the solver
of last resort, now targeting I (§16):

0. **Re-spelling tier.** The migration tool applies the naming law
   (§6.4) to every file mechanically — constructors, Bool literals,
   `std` names, comparison and equality forms, `Int` sizes to `Nat` —
   and re-spells old proofs whose steps are I-shaped (unfold, rewrite
   with a lemma at an occurrence, case split, induction, a Farkas
   certificate) into I with named hypotheses; whatever replays is done.
   This replaces the optional compatibility layer of v0.2.
1. **Solver tier.** Every remaining PORT claim is attempted by the V2
   solver with a generic script (induction, `simp`, arithmetic
   reflection, the ported `tools/prove` ladder). The old engine
   re-solved 172 of 182 on an earlier corpus; this tier should carry
   most of the count.
2. **Cheap-agent tier.** Unsolved claims go out in batches with a fixed
   **context pack**: `LANGUAGE.md` (V2), the crosswalk, a
   worked-examples file with one ported claim of every proof shape, the
   module's interface, and the old proof as a structural hint. The
   checker's output — goal states from the I elaborator — is the only
   feedback loop; budgets are explicit.
3. **Strong-agent tier**, then a ruling for the residue.

Two mechanical guards make the pipeline safe regardless of who solves:
a porting agent **cannot** change a statement, touch an interface, add
an axiom, weaken a requirement, expose a private definition, alter a
primitive, or change a selected instance — the manifest pins the
resolved requirement (§6.2), the axiom-scope and kind gates refuse, and
the view check (T5) refuses interface drift; and T9 runs *before* the
bulk, since a cheap agent with a gotcha list is a bad agent. The
context pack is calibrated on `std` in phase 3, with the tier split
measured on real numbers before the bulk begins.

### 14.4 Phases and gates

0. **Ledger.** This document ratified as `FOUNDATION.md` law; the Lean
   release pinned; the declarative rule inventory and the procedure
   written; the port manifest drafted; `LAYOUT.md` gains `v2/`; the
   GPT-6 reviews recorded.
1. **K.** `v2/kernel`: K in narrow-compatible E, run directly on Rust
   (route 3). Gate: T0.
2. **Front-end and `ev`.** The V2 loader and reader (s-expr → explicit
   L terms), environment views, the fragment classifier, `ev`;
   `examples/calc` as the first program with explicit-term proofs.
   Gates: T1, T5, T8 (replay half), conformance on a value matrix.
3. **Elaboration, the proof IR, the first library.** Stages 1–2
   including the I elaborator and goal-state API (§16), core tactics,
   certified arithmetic; `std/list`, `order`, `nat`, `div`, `bits`,
   `arith` re-proved in `v2/std` **under the naming law** (§6.4:
   `List.length` over `Nat`, lowercase constructors, propositional
   connectives); the 15 former axioms proved as theorems; the migration
   table validated including exceptional behavior; `docs/LEAN.md`
   written; the porting pipeline built and **measured here**. Gates:
   T2, T3, T9 (small form), T10, the tier split recorded.
4. **Entrenchment tests.** Before the environment and executable
   representation are sealed: a `tb_len`-class compiler proof with its
   dependencies (Q8), contextual holes (T4), search fidelity (T7),
   prepared invocation (T6), evaluation reflection (T8's second half).
   Measured against 717 lines and against the B-1c generator design.
5. **Bulk port by manifest.** Remaining `std` (the floats family as its
   own line — it is the corpus long pole, #37, #39), `models/imp`,
   `x86`, `linux`, `meta`, tools, apps; REGENERATE families produced by
   validators and tactics. Gate: the V2 corpus green on every PORT
   file; every ARCHIVE decision recorded.
6. **The flip.** `v2/` becomes the tree; CI, `bin/`, docs, README and
   memory move; the old tree is archived in history. Gate: the fmt gate
   and the whole DEFAULT corpus on V2.
7. **Resume.** The coverage arc unparks on V2 with B-1c as an
   I-emitting tactic; Mathlib export at scale as a performance goal
   (route 1's consumer).

Serial on main, one gate per phase, CI green behind each, generated
files never hand-patched, kernel sources frozen during corpus runs —
the standing laws apply unchanged.

---

## 15. Risks, stated once

- **Elaborator scale** — the real cost; staged (§6.1) so each stage is
  usable and nothing is trusted.
- **Interpreted-K performance** at Mathlib scale — phase 7's second
  half is a performance goal that may need compiled K; nothing before
  it needs Mathlib-scale throughput. Evaluation reflection (§5.4) is
  slow for the same reason until route 1.
- **Definitional-equality unpredictability** — explicit conversion
  policy and budgets; `Exhausted` is a verdict, never a hang.
- **The Bool/Prop seam** — handled Lean's way (`Decidable`, `decide`);
  E keeps Bool predicates so lowering never sees `Prop`.
- **Fragment erosion** — agents writing L where they meant E; the
  keyword, the loud classifier, and the lowering as authority.
- **Specialization blow-up** — the finite-specialization discipline and
  its refusal (§5.3, T3).
- **IR ossification** — an I vocabulary that cannot reach some proofs;
  the `exact` escape (§16.2) guarantees reach, and I is versioned.
- **Trust in the oracle** — Lean's kernel has had bugs; agreement with
  it is strong evidence, not proof. The hostile battery and the
  independent checkers (Lean4Lean, nanoda) are the second leg.
- **Port drift** — the manifest's pinned resolved requirements and the
  migration record (§12) are the only defense against a nearby weaker
  theorem being counted as the old obligation.

---

## 16. Proof search and the proof IR (I)

The user's question (2026-09-06): the old proof DSL was something the
search engine could navigate, and the `auto` sidecars recorded any
solution any tool found; if the Lean way is tactics producing
certificates at build time, what keeps the ability to build better and
better search engines, and should the pinned IR be navigable with the
same template-and-hole system the E search uses?

### 16.1 Three levels, not two

Lean has two levels: tactics (search, agent-facing, unstable across
versions) and proof terms (K's input, large, not a search grain). Shard
keeps a third in the middle, and it is the level shard already has:

```
tactics  ──search, LLM-fluent──▶  I (the proof IR)  ──deterministic elaboration──▶  P (terms)  ──▶ K
```

**I is a certificate language, not a tactic language.** Every I form is
a fixed, named, budgeted, *deterministic* step from a goal state to
subgoals plus a term builder: rewrite with *this* lemma at *this*
occurrence with *these* instantiations; induction on *this* variable
with *these* cases; `simp only` with *this* lemma list and budget;
arithmetic with *this* certificate; evaluation reflection with *this*
fuel and result; `exact` with *this* term. Anything that searches — a
`simp` that discovers its lemma set, an `omega` that finds its
certificate, an `apply?`, a search engine, an agent — is a **producer of
I**, and what it emits is the search *result*, not the search. This is
today's architecture exactly (a Farkas `by` carries its certificate; a
`rewrite` carries its `inst`s; sidecars hold solutions, LS-law 1 says
replay is the referee), generalized to a term-producing calculus. It is
also why "check never searches" survives: replay elaborates I to terms
with no search, and K checks the terms.

### 16.2 The I vocabulary

I's v1 vocabulary is §4.2's roster generalized to formulas and a named
context: `intro`, `exact`, `apply`, `rfl`, `unfold`, `rw` (lemma,
instantiation, occurrence, direction), `simp_only` (lemma list, budget),
`cases`, `induction`, `have`, `show`/`change` (budgeted conversion),
`decide`, `reflect` (§5.4: program identity, args, fuel, result),
`arith` (certificate), `wf` (measure, descent evidence), `sorry`
(rejected at acceptance, kept for authoring). Two guarantees:

- **Reach.** `exact TERM` means every proof term is expressible in I;
  no proof is outside the calculus, and imported Lean proofs are
  `exact` nodes.
- **Stability.** I is versioned with the foundation; an I form's
  elaboration is a specification, and adding a form never changes an
  existing pin's meaning. Tactic *implementations* may change freely;
  they only produce I.

### 16.3 Goal states are data, and the API is public

The I elaborator exposes, as ordinary E functions over ordinary E data:
`goal_of(prefix)` (elaborate a prefix of I against a theorem and return
the goal state: named context, target, metavariables); `applicable(goal,
env, policy)` (the I forms whose side conditions hold — lemma matching
by unification against the target, the deterministic half of §7's
engine); `step(goal, form)` (subgoals, or a definitive refusal, or a
blocked/exhausted outcome per §3.3); `elaborate(I)` (the term DAG). No
monad over elaborator internals, no process-global state: a search
engine is another E program composing these, exactly as today's engine
composes the sequent API. This is the concrete answer to "how do we keep
building better engines": engines are consumers of a stable I plus a
public goal-state API; an engine improvement never changes what a pin
means, and any engine's solution is checked identically by any other
(T10). Today's `tools/explain` and tracer become renderers of these goal
states.

### 16.4 One hole system for programs, statements, and proofs

Yes to the user's suggestion: the template-and-hole mechanism of §7 is
the same for L terms and for I, because I is E data and L terms are E
data. A partial proof is an I with metavariables; a hole's expected
type is the goal state at its position (computed by `goal_of` on the
prefix); shared holes are correlated choices; grammars over I forms
are `meta/sketch` grammars; exact counting, stratification, dominance
and the "blocked is not UNSAT" law carry verbatim. The lock-step law of
`SEARCH.md` — joint search over (implementation, proof) pairs, each side
pruning the other — becomes joint search over (E term with holes, I with
holes) under one metavariable context, which is strictly easier than
today because the proof side's goal states are typed by the same
metavariables as the program side's holes. Theorem-first steering (a
goal state refutes a candidate program early) is `goal_of` returning a
definitive refusal per §3.3.

### 16.5 What the pin stores

`foo.auto.shard` (or its V2 name) stores, per `auto` claim, the **I**
(small, readable, navigable, the thing any engine wrote) and the
**content hash of the checked term DAG** it elaborates to. Replay
re-elaborates I deterministically and K re-checks; the hash detects
drift between what was pinned and what elaborates now. The term DAG
itself is a content-addressed cache (STORAGE's successor), not the unit
of identity. Tactic scripts in *source* files elaborate to I at build
time and are pinned the same way, so hand-written and engine-written
proofs are indistinguishable at the pin, which is what lets a better
engine later re-solve a hand proof and replace it without any statement
changing.

### 16.6 What this costs

An I elaborator alongside the term elaborator (Stage 2 owns both); a
discipline that every search-bearing tactic emits its result into I
rather than directly into terms; and a versioning rule for I. What it
buys is the property the user asked to retain, made stronger than it is
today: the search engine never depended on the kernel's rewriter being
the proof language, and now it does not depend on the tactic framework
either.

---

## 17. Response to GPT-6's follow-up memo, R9–R16

Recorded as requested: accept / amend / defer, with the section and
test affected.

| ID | disposition | where | notes |
|---|---|---|---|
| R9 | **amend** | §5.4, §13.2; T1, T8 | mechanism named: per-invocation evidence is `rfl` on the fuelled `ev` run, checked by K's definitional computation; no native oracle; `cbv`/`decide_cbv` acknowledged as the precedent; the "stale executable view" test is a manifest-drift refusal because equations are generated from the hashed body |
| R10 | **accept** | §3.1, §3.3, §13.4; T0 | declarative judgments and the bounded procedure separated; each negative outcome's meaning to search stated; accelerated Nat ops bound to fixed identities |
| R11 | **accept** | §6.2; T9 | reconstruction vs selection; the resolved requirement as the frozen task; the three tests |
| R12 | **accept** | §4.1; T6, T8 | six acceptance records; policy at every boundary; invalidation granularity; both tests |
| R13 | **accept** | §7; T4, T7 | the transactional invariant and attempt/commit shape; the four tests |
| R14 | **amend** | §12.1; the crosswalk | conventions ruled (Q9): Lean's totalizations copied as the discipline's consensus; checked/preconditioned wrappers separately named; division by zero is not a fail family; exceptional behavior a crosswalk column |
| R15 | **accept** | §5.3, §5.7, §13.1 | "no unaccounted replacement" replaces "no fn implemented elsewhere"; relations explicit; the dynamic-tier wake condition as proposed; `csimp` distinguished |
| R16 | **defer, recorded** | §10, §13.4 | conversion instrumented first; conversion plans and expected-type-directed checking as experiments under unchanged rules; Lean4Less noted as research; no v1 departure |

Corrections from the memo's §9 taken in §13: the "most historical
unsoundness" sentence deleted (no dataset); stored proof terms and
proof-producing evaluation acknowledged; the module-system contrast
updated; `partial`/`unsafe`/`implemented_by`/`csimp` distinguished;
"deterministic elaboration" replaced by structured exhaustion plus
replayable evidence; E ownership described as integration and a route,
not a correctness proof; oracle agreement described as differential
evidence, not a bound; no claim about what Lean's logic cannot express.

Two items not taken as written: the "stored executable view" record
(kept as a derived view, §2), and the runtime form of the stale-view
test (kept as manifest drift, §5.4).

---

## References

- M. Carneiro, *The Type Theory of Lean*, 2019 (the declarative rule specification, to be reconciled with the pin).
- M. Carneiro, *Lean4Lean: Verifying a Typechecker for Lean, in Lean*, arXiv:2403.14064 (the additions since the thesis; the declarative/algorithmic distinction; bounded checking; primitive validation before acceleration; what is and is not proven).
- Lean 4 kernel sources, `src/kernel`, release pinned at phase 0.
- `lean4export` (export format), `lean4checker`/`leanchecker` (replay through Lean's kernel — comparison harness), `nanoda_lib` (independent checker).
- Lean Language Reference: *Recursive Definitions*; *Tactic Reference* (`decide +kernel`, `cbv`, `decide_cbv`); *Validating a Lean Proof*; *Source Files and Modules*; *Natural Numbers*; *Headers and Signatures* (automatic implicit parameters); *Instance Synthesis*. Live documentation; the release pin governs.
- R. Vaishnav, *Lean4Less: Eliminating Definitional Equalities from Lean via an Extensional-to-Intensional Translation* — research precedent only (§13.4).
- Candle (CakeML) — verified HOL Light with a compute primitive; the end-state shape for a verified K.
- Repo: `docs/TCB.md`, `docs/TOTALITY.md`, `docs/CERT.md`, `docs/SEARCH.md` (LS-laws 1–3, the lock-step law), `docs/LANGUAGE.md` §10, `docs/COVERAGE.md`, `docs/MEMORY.md` D8, `kernel/proof.shard`, `meta/sketch/mod.req.shard`, `meta/invoke/prepared.shard`, `tools/search/theorem_scope.shard`, the 2026-07-24/25 and 2026-09-02 kernel-survey records (memory); `SHARD_FOUNDATION_PROPOSAL_v0.3.md`, `SHARD_BOOTSTRAP_ADDENDUM_v0.3.md`, `SHARD_FOUNDATION_FEEDBACK_v0.1.md`, `SHARD_FOUNDATION_FOLLOWUP_MEMO_v0.1.md` (repo root).
