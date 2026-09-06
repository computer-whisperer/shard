# FOUNDATION.md — the shard V2 foundation: a Lean-parity logic over a lowerable core

> **STATUS: DRAFT v0.2 (Fable), 2026-09-06 — a proposal for review, not law.**
> v0.1 (2026-09-05, commit `5d95b81`) was reviewed by GPT-6
> (`SHARD_FOUNDATION_FEEDBACK_v0.1.md`, repo root, untracked; amendments
> R1–R8, tests T1–T8, edits 11.1–11.8). v0.2 folds in R1, R3–R8 and the
> substance of R2, records the user's 2026-09-06 rulings (§11), and adds
> two sections the user asked for: the Lean review (§13) and the
> Foundation-arc plan with the sibling `v2/` tree (§14). Companion and
> counter-proposal to GPT-6's `SHARD_FOUNDATION_PROPOSAL_v0.3.md` and
> `SHARD_BOOTSTRAP_ADDENDUM_v0.3.md`; their decision IDs D01–D13 and
> B06–B16 are answered in §9. Nothing here is implemented. Sizes are
> estimates and say so. Ratification turns this file into the ledger;
> until then every "law" below is a proposed law.

Evidence baseline: the tree at `5abc600` (B-1b complete, coverage arc
parked), `docs/TCB.md`, `docs/TOTALITY.md`, `docs/CERT.md`,
`docs/LANGUAGE.md` §10, `kernel/proof.shard`'s Step/Proof roster, and the
2026-07-24/25 and 2026-09-02 kernel surveys (six confirmed 0=1 holes, all
fixed). The measured B-1b numbers (tb_len 717 / tb_perim 957 / tb_app 1,148
canonical lines per function) are the immediate provocation.

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
Everything shard proves today is restated under an explicit crosswalk;
every special kernel rule becomes a library theorem or an untrusted
tactic; the trusted code shrinks from "35k lines and twelve special
rules" to K plus the E evaluator plus the Rust host, and the logical
assumptions shrink from fifteen hand-written arithmetic axioms to Lean's
three. Parity with Lean is made testable: **an exported Lean declaration
checks in K**. Elaboration and tactics are the real project and are
staged as untrusted meta-layer work.

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
   harness rather than as independent kernels (R6). A rule-exact K can
   be differentially tested against Lean's kernel on hundreds of
   thousands of declarations. No foundation of our own design gets an
   oracle like that, and the 2026 holes show what a from-scratch
   checker without one costs. The oracle validates closed L checking
   only: E execution, contextual holes, module views and lowering need
   their own tests (§10).
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

Four things that are today conflated get names. The rest of the
document is written in these terms.

| name | what it is | who checks it | lowers? |
|---|---|---|---|
| **L** — the logic | Lean-rule dependent type theory: terms, types, propositions, proofs | K | never |
| **E** — the executable fragment | the subset of L definitions that are shard *programs*; today's entire object language | K (as L) + the fragment classifier + the lowering | yes, and only E |
| **P** — proof objects | terms of L whose type is a `Prop` | K | never (erased) |
| **S** — the surface | s-expression forms, Lean-flavored; elaborated to L by an untrusted elaborator | nothing (untrusted) | — |

**The trichotomy survives unchanged.** Requirements are `Prop`s in L;
algorithms are E definitions; low-level spellings are E definitions in
a model's vocabulary; the refinement relation is a theorem in L. What
changes is that requirements may now be *any* proposition, not a
conditional equation.

**Programs are still data.** An E definition is an L constant with a
body; `meta/` inspects it as a term. Quotation, sketching and search
operate on L terms with metavariables (§7).

**The toolchain is E; the logic is data (proposed law).** L's terms,
levels, declarations and environments are ordinary `type`s declared in
E. K is a set of `fn`s over that data: first-order, total by verified
measure, closure-free, checkable, and lowerable by shard's own
lowering. The same holds for `ev`, the elaborator, and every tactic.
Higher-order functions, dependent types and `Prop` exist only *inside
the values* the toolchain manipulates; nothing the toolchain runs on
depends on them. This is what lets the whole toolchain compile and
dissolve like every other artifact (§8), and it is the exact sense in
which "HOF exist in what the logical core reasons about, not in the
kernel" (user, 2026-09-05). The statement concerns the toolchain's
*implementation*, not the mathematical content of its inputs; the
initial toolchain stays in the first-order E profile, and source-level
lambda conveniences (§5.3) lower into that profile before any E program
exists — nothing about them adds runtime closures to `ev` or L rules to
Rust (GPT-6 11.1, accepted).

**E has no function values (proposed law).** Lambdas, partial
applications and template arguments in E source are eliminated **by
elaboration** — lambda lifting plus specialization — so that no E
program, and therefore no `ev` value, no Rust value, and no artifact,
ever contains a function value or an indirect call. A `fn` either
lowers or is refused; there is no "interpreted-only" status for a `fn`
(§5.3, the one place v0.2 declines GPT-6's R2).

**The L/E correspondence is by defining equations (proposed law).** An
E definition's *executable structure* is its source body; its *L
meaning* is that body's elaboration (recursors for structural
recursion, `WellFounded.fix` for measure recursion); the *bridge* is the
set of defining equations `f.eq_N`, each a theorem of L — by `rfl` for
structural recursion, by the equation lemma for well-founded recursion.
This is OVERVIEW.md §7's "every defining equation in the system is a
theorem", restated for V2, and it replaces v0.1's literal
`K.whnf ≡ ev` claim (R1; §5.4).

---

## 3. K — the kernel, rule by rule

### 3.1 The normative reference

The rule specification is Mario Carneiro's *The Type Theory of Lean*
(2019) **reconciled with** the pinned Lean 4 kernel release and with
Lean4Lean (arXiv 2403.14064), which records the additions since the
thesis — nested inductives and structure eta among them — and which is
the reference for what is and is not proven about those rules (R6).
K's own rule document is a restatement of the reconciled inventory in
shard's terms, versioned; v1 declares **zero departures**. Normalization
is not a theorem of the implemented system; K therefore budgets (§3.3).

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
exact list is **reconciled with the pinned release at F0** (Lean 4's
inventory at the time of writing: `add sub mul div mod gcd beq ble
land lor xor shiftLeft shiftRight pow log2`), each with its logical
declaration identity, defining recursion in the library, accepted
argument shape, and a correspondence test — the same shape and pedigree
as Lean, and the same conformance discipline as today's prim tables.

### 3.3 Resource outcomes

`check(env, decl, limits) → Accepted | Rejected | Exhausted`.
`Exhausted` is never acceptance and never rejection, and must remain
distinguishable from logical rejection and from "unsupported
declaration". This is Lean's `maxHeartbeats` made a first-class
verdict; it is also the fuel doctrine of `TOTALITY.md` applied to the
checker itself. K is a total shard program with an explicit budget, as
`check_sequent` is today.

### 3.4 What is *not* in K (the surviving HOL discipline)

No unification, no elaboration, no implicit arguments, no typeclass
resolution, no tactics, no simp, no arithmetic decision procedure, no
measure recognizer, no filesystem, no process-global state. K takes
an environment value and a declaration and returns a verdict. The
elaborator and every tactic are untrusted producers of L terms (§6).
This is GPT-6's D09 "small authority" and shard's existing
"verify, never search" law, stated once.

### 3.5 Size and the oracle gate

Reference points: `nanoda_lib` (Rust) and Lean4Lean's checker are a few
thousand lines each; `lean4checker` is *not* a size reference (it
replays through Lean's kernel). Estimate for K in shard: **6–10k
lines** including level arithmetic, inductive admission with recursor
generation, and the Nat extensions; the recursor generator is the
largest single piece. The figure is an unmeasured estimate and **not a
gate** — a size gate would encourage hiding admission logic in
"untrusted" helpers (R6). The gate is behavioral: K accepts
`lean4export` output of Lean's `Init` prelude declaration-for-
declaration with identical accept/reject verdicts and axiom closures,
and rejects a hostile battery whose members each carry an independently
specified reason for rejection (§10).

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
| `(claim NAME GOAL PROOF)` | `theorem NAME : PROP := PROOF` | KEPT; goals may be any `Prop` |
| `(axiom NAME (kind …) GOAL)` | `axiom` with the kind tag; **K tracks the axiom closure natively** (Lean's `#print axioms`); scope gate and kind gate unchanged | KEPT; the ledger's cites walk becomes a kernel query |
| `(requirement …)` / `(fulfills …)` | unchanged: a stated `Prop` and its later proof, same-goal single source of truth | KEPT |
| `(bin …)` with `entry/externs/trusts/requires`, LEDGER block, MET/UNMET, artifact-claim forms | unchanged | KEPT |
| `(import …)` / `(use …)` / QName identity / selective loading / req-scope gate / canonical dedup | unchanged; a resolved declaration's identity is its QName plus content hash in the environment; statement identity, evidence identity and assumption closure are three distinct records | KEPT |
| `auto` + `.auto.shard` sidecars | sources carry tactic scripts; sidecars carry **checked proof-term DAGs** (Q2 RULED); LS-law 1 (replay is the referee) verbatim | KEPT, respelled |

### 4.2 Proof forms (`kernel/proof.shard` roster, every constructor)

| today (kernel rule) | V2 (where it lives) |
|---|---|
| `Refl` | `rfl` — K's defeq |
| `Steps` + `Unfold` | `unfold`/`delta` tactic → explicit `Eq.mpr` over a defining equation, or defeq |
| `Reduce`, `Simp` (+ `stop` fence) | `simp only [...]`-class tactic (meta); fences = the simp set |
| `Compute` (+ `stop`) | `decide` / `rfl` by K's whnf with Nat-literal acceleration for structural definitions; the **verified `decide`** of §5.4 for measure-recursive ones; fences = `irreducible` marks |
| `Rewrite` (+ `Occ`), `RewriteWith` (+ obligations) | `rw`/`rewrite` tactics producing `Eq.mpr`/`congrArg` terms; occurrence selection is tactic syntax |
| `Change`, `ExactConv` (CERT.md §3) | `change`/`show`/`exact` — defeq at the named site, budgeted (§3.3); the ratified *explicit* conversion policy becomes elaboration policy: irreducible by default, unfolding named |
| `Induct`, `Induct2` | `induction` → the inductive's recursor (K-generated) |
| `CaseOn` | `cases` → `casesOn` |
| `WfInduct` (Int measure) | `WellFounded.fix` over `Int`/`Nat` measure via `Acc`; measure descent = a proof term, verified never searched |
| `SubtermInduct`, `Below` | course-of-values recursion (`brecOn`-style) derived from the recursor; `below` = the derived strong-induction lemma |
| `Have` | `have` — a `let`-bound proof |
| `Absurd` (head clash) | `noConfusion` theorems (elaborator-derived from recursors) + `nomatch`/`absurd` |
| `Inject` | `injection` → `noConfusion` |
| `FinSplit` | `decide` over `Fin`/interval + `omega`-class reflection |
| `ByTheory` lia/eqdec/ord/farkas/reflect/arith | **certified reflection**: `omega`-class decision procedure in meta that emits proof terms, and/or a shard-proven `farkas_ok : Rows → Cert → Bool` with a soundness theorem, applied by `decide`. The kernel arith backend is DELETED |
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

Logical assumptions and trusted code are different records (R6); the
table keeps them apart.

| roster item today (`TCB.md`) | V2 |
|---|---|
| Rust bootstrap interpreter (execution authority) | KEPT as the enduring bootstrap executor of E (§8); execution dependencies recorded per route |
| checker sources `kernel/*.shard` (~35k lines) | K sources (est. 6–10k) + the E loader + `ev`; typer, reducer twins, rewriter, tactics, measure gate, arith, desugar RETIRED from the roster |
| `kernel/facts.shard` — 15 axioms (mod range, euclidean completion, ring laws, bitwise recurrences, shifts) | **retired as axioms**: `Int` is an inductive over `Nat`; `Nat` ops are defined by recursion; every one of the 15 is a library theorem (Lean proves all of them); the kernel's Nat-literal acceleration is the only "prim" left, tied by conformance to the definitions |
| per-artifact trust scopes (bolts, bridges, kinds) | KEPT verbatim |
| prim tables ×2 (+ the untracked PrimTag fast path = 3) | ONE definition per Nat op in the library; ONE acceleration table in K; ONE E evaluator (§5.4); conformance sweeps definition-vs-acceleration and evaluator-vs-K-on-equations |
| — (logical assumptions) | three axioms `propext`, `Quot.sound`, `Classical.choice`, tracked per declaration |

Net: the trusted code loses twelve special rules and three reducers;
the assumptions lose fifteen hand-written arithmetic axioms and gain a
well-studied three-axiom base with an external oracle.

### 4.4 Infrastructure

| today | V2 |
|---|---|
| CERT.md dialect (change/exact-conv, base+patch, validators, Runs) | conversion forms become tactics (§4.2); **validators are unchanged** — an E function `valid_P` plus an L theorem `valid_P_sound`, cited per program (GPT-6 §10.2 and CERT.md §4 say the same thing) |
| STORAGE S1/S2 (text caches, hashed closures) | native: L terms are hash-consed DAGs, environments are content-addressed; S3's arena is the default representation, not a parked item |
| `tools/prove` ladder, `meta/search` engine, `meta/sketch` holes | tactics and metaprograms over L terms with native metavariables (§7); LS-laws 1–3 verbatim |
| `tools/impc`, `models/imp`, `models/x86`, `models/linux`, byte-tie, gates | unchanged E libraries and unchanged gates; their theorem *statements* verbatim under the crosswalk (§12); their proofs re-derived |
| `tools/shardfmt`, `tools/canon`, `tools/digest` | s-expression lexical layer KEPT; new heads added to the canon grammar |
| `eval.shard` → `check.shard` tower | retired as the mandatory route; retained as the differential route (§8) |
| `tools/explain`, tracer | rebuilt over elaborator goal states (the tactic layer's job) |
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
`if`; literals; `decide` on decidable propositions is permitted and
lowers to the Bool its selected decision procedure computes — that
procedure must itself be E (R1). Proof-typed arguments are permitted
and erased. No `Classical.choice`, no proof used computationally, no
`Sort`- or `Pi`-typed value in any position of an *elaborated* E body.

### 5.3 The lambda profile: static forms, eliminated by elaboration

**Supported in v1, in this order of introduction** (Q5 RULED
2026-09-06; GPT-6 3.3 accepted):

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
   each instance is definitionally the specialized body.

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

**Finite specialization (GPT-6 3.6, accepted).** Runtime totality does
not imply finite monomorphization: `f[A](n,x)` recursing at
`f[List A]` decreases its measure and still generates an unbounded
family. Specialization keys distinguish static code identity from
dynamic values; the elaborator refuses non-terminating or over-budget
specialization loudly, as Rust reports its recursion limit; an
exhausted specialization budget is a refusal, not a proof the program
is unlowerable.

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
alternative. If a consumer ever appears that cannot be defunctionalized,
the mechanism is bounded and C-like — a closure is a counted cell with a
code index and captured fields, an indirect call is a jump table — and
it opens as a separately priced profile with its own MEMORY.md rung.
Wake condition: a named consumer, priced against sealed variants.

### 5.4 One evaluator, and the correspondence it must satisfy

The reference evaluator `ev` for E is one shard program (today's
`eval.shard` `ev`, environment machine with TCO). It is the definition
of "run". Its relationship to K is **not** literal agreement with
`K.whnf` (R1; well-founded definitions do not reduce definitionally,
and whnf stops at an outer constructor while `ev` computes a value).
The contract is:

- `ev` computes by the defining equations `f.eq_N`; each is a theorem
  of L (§2). `ev`'s correctness statement is "for E-typed inputs,
  `ev f args ⇓ v` implies `f args = v` in L, and under adequate
  resources such a `v` exists" — the two-sided form GPT-6 2.3 asks for.
- Execution-route conformance (`ev` interpreted by Rust, `ev` compiled,
  K's whnf where it applies) compares *results and observations under
  explicit resource conditions*, never budgets or representations.
- Once `ev`'s correctness theorem exists (it is an ingredient of route
  1 in §8 anyway), a **verified `decide`** follows: a reflection tactic
  runs `ev` natively on a closed E term and cites the theorem. This
  replaces Lean's trusted `native_decide` with a proven one and unsticks
  `decide` on measure-recursive definitions (§13).

### 5.5 Totality

Every `fn` is total; that is a *theorem* of its admission (its
recursor or its `WellFounded.fix` with a checked descent proof). The
`TOTALITY.md` regime carries over as elaboration: the author writes
`(measure E)`; the elaborator emits the descent obligations; they are
discharged by tactics (untrusted) into proof terms K checks; the
offline `admit` classifier stays advisory and out of the trust path.
Unbounded processes take Int fuel, as today. Structural descent is
compiled to recursors and needs no obligation.

### 5.6 Erasure and executable-view obligations (R1, accepted)

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

### 5.7 What E buys the lowering program

Nothing in `IMP.md`, `MEMORY.md`, `X86.md` or `COVERAGE.md` changes
its object: the generic spec→imp compiler consumes post-specialization
`fn`s, which are the same first-order total programs as today, now with
a precise membership test. The counted heap, calls and stack, the
`except` clause, register allocation — all unchanged rungs. What
changes is the *certificate* side: Theorem A/B are theorems in L with
proof terms, and B's per-function certificates are produced by tactics
rather than by a text generator (§12).

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
- **Stage 2 — tactics.** A goal-state framework with a named local
  context and the core set: `intro apply exact rfl rw simp only cases
  induction have show change decide omega`-class reflection,
  `termination_by`/`decreasing_by`. The `tools/prove` ladder and the
  search engine become tactics in this framework.
- **Stage 3 — typeclasses and coercions.** Needed for law-bearing
  hierarchies and for Mathlib-style statements; also the point where
  `Decidable` bridges Bool and Prop cleanly. Explicit instantiation
  stays the preferred spelling (§13).
- **Stage 4 — the compatibility layer, optional.** Today's proof-DSL
  step vocabulary as tactics, built only if phase 3's measurements
  (§14) show it beats agent rewriting; expected to be skipped.

### 6.2 The surface S

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

;; V2 (schematic)
(theorem len_append ((xs (List α)) (ys (List α)))
  (= (len (append xs ys)) (+ (len xs) (len ys)))
  (by (induction xs) (simp [append len])))

;; a statement today's logic cannot state, in the same surface
(theorem dot_bound ((n Nat) (u v (Vector Real n)))
  (-> (forall ((i (Fin n))) (<= (abs (get u i)) 1))
      (<= (abs (dot u v)) (norm1 v)))
  (by ...))

;; the executable/logical split is one keyword
(fn  len ((xs (List α))) Nat (measure (struct xs)) (match xs (Nil 0) ((Cons _ t) (+ 1 (len t)))))
(def spec_sorted ((xs (List Int))) Prop (forall ((i j (Fin (len xs)))) (-> (< i j) (<= (get xs i) (get xs j)))))

;; a static lambda: eliminated by elaboration, no function value in E
(fn add_offset ((k Int) (xs (List Int))) (List Int) (map (fun (x) (+ x k)) xs))
```

Binders are explicit: no auto-bound implicits, no silent
generalization of unknown names (§13). *Q6 RULED: no Lean
concrete-syntax reader in v1.*

### 6.3 The LLM-first gate

An agent with no memory files and no gotcha list proves a
`tb_len`-class theorem (§14) from `LANGUAGE.md` alone. That gate, not
line counts, is the measure of the surface; it runs early on small
tasks and again at the end (R7).

---

## 7. Metavariables, sketches, search (D11/D12; R3 accepted)

L terms carry a native metavariable node with a declared local
telescope, a dependent expected type, and, at each occurrence, an
explicit typed substitution from the occurrence context into that
telescope (`?h : [Ψ ⊢ A]`, `?h[σ]`). K refuses any declaration
containing one. The engine — in meta, not in K — exposes:

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
quantified L theorem. Persistent workspaces and branch merge semantics
are deferred to the meta layer's own ledger; branch/snapshot isolation
and context-keyed caches are required from the first implementation.
Test T4 (§10) is the pathfinder.

---

## 8. Execution and trust (the bootstrap addendum, answered)

**Rust is the enduring bootstrap facility, not an architected-in
component** (user, 2026-09-05). It implements E's operational semantics
and does not define a competing source language or any L proof rule
(GPT-6 11.7 wording, accepted). It is kept so that a fresh machine can
run shard from Cargo and sources. It never executes anything but E,
and it needs no image format beyond what E's own loader produces.
Because K, `ev`, the elaborator and the tactics are E programs (§2),
the routes that execute K are, in order of preference as they come
online:

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
manifest (GPT-6 11.7). The logical rule authority does not change by
choosing among them: Rust has always been trusted-by-review as the
executor of shard programs (`TCB.md` lists it first), `eval.shard` was
never verified either, and the C-class dissolution is a law about
artifacts that stands untouched.

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

## 9. Positions on GPT-6's decisions

| ID | GPT-6 v0.3 | this proposal |
|---|---|---|
| D01 | Shard-owned, Lean-informed dependent foundation; departures allowed | **Lean-rule-exact K**; departures are dated decisions priced against the oracle; zero in v1 |
| D02 | versioned rule package | accepted; Carneiro reconciled with the pinned Lean release and Lean4Lean |
| D03 | fixed conversion, no equality reflection, budgets | accepted; explicit-conversion policy as elaboration discipline; tactic transparency is not opacity and not conversion (§4.2) |
| D04 | E0 executable view, erasure as a theorem-bearing pass | **replaced by E** (§5): a syntactic fragment with a keyword, not a second IR; erasure is a small specified transformation with the obligations of §5.6, and the L/E correspondence is by defining equations (§2) |
| D05 | intensional program identity | accepted (QName + content hash; statement, evidence and assumption identities distinct; validators and observation relations unchanged) |
| D06 | proof graphs from day one | accepted; native in K's term representation |
| D07 | Lean math coverage as long-term target; transport optional | **the kernel check is cheap under rule-exactness; integration is priced work** (§1.3, Q7); the parity gate is the arc's measured goal |
| D08 | migrate meanings, rewrite proof text | accepted; statements under an explicit crosswalk with pinned hashes (§12) |
| D09 | embeddable engine, small authority | accepted as design constraints on K and the elaborator (environment as a value, no I/O); the minimum prepared-invocation contract is exercised early by T6 |
| D10 | one identity system, environment vs workspace | accepted |
| D11 | native contextual holes with dependent telescopes | accepted as §7: Lean-style metavariables with a specified substitution/closure obligation and five-way outcomes, in meta |
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
one number for all costs (R7).

| test | experiment | what failure reveals |
|---|---|---|
| **T0 — oracle** | K checks `Init`'s export declaration-for-declaration; accept/reject verdicts and axiom closures identical to Lean's; a hostile battery, each member with an independently specified reason for rejection (universe collapse, scope capture, forged recursor, non-positive inductive, `Prop`-to-data elimination, cyclic definition, the six 2026 exploits restated) | a rule mismatch, an input-mapping error, or a budget difference; disagreement is investigated, never settled by tool name |
| **T1 — realization** | one structural fn, one measure fn, one subtype-producing fn, one `decide` case: L meaning, executable view, and the equation bridge established | a missing erasure/recursion/representation bridge |
| **T2 — static lambda** | a closed lambda, a named partial application, and the captured-value `add_offset` example lower to the first-order profile; the theorem `map_k k xs = map (fun x (+ x k)) xs` holds by specialization | source restriction mistaken for artifact restriction; wrong capture or specialization |
| **T3 — bounded specialization** | the type-growing recursion example and a large finite specialization workload refuse or complete within agreed limits, loudly | runtime totality mistaken for compiler termination or acceptable code size |
| **T4 — contextual holes** | a shared hole under renamed binders; a dependent expected type; a blocked comparison; final closure; one theorem reused across fillings | a native node without a coherent open-construction discipline |
| **T5 — abstraction** | a consumer checks with the public view alone and with the implementation linked, typing and obligations stable; a proposition that secretly depended on a private equality is rejected at the view | invalid environment weakening |
| **T6 — embedding** | an in-process client constructs a declaration, prepares once, invokes repeatedly, transforms with `meta/`, checks a claim, and requests a lowering, under one declaration identity; preparation count and marshaling measured | CLI dependence, duplicate environments, hidden preparation cost |
| **T7 — search fidelity** | a small ground-truth candidate space with correlated holes; a root-only observer rewrite invalid in a nested position; a heuristic timeout that must not count as UNSAT | wrong pruning, counting, or reuse of observational equalities |
| **T8 — replay and trust** | cold replay of terms, assumptions, declaration identity and artifact bytes on reviewed Rust-hosted K; `Exhausted` never yields a receipt; the execution route is recorded | pending evidence, stale caches, or an unstated trust transition |
| **T9 — the LLM-first gate** | §6.3, early on small tasks and again at the end | a surface that needs folklore |

Performance decision rule (GPT-6 §16.1, accepted): baselines and
budgets are agreed before each rung runs; no speedup is inferred from
architecture.

---

## 11. Q1–Q8 — RULED 2026-09-06

The user confirmed the leans below ("Your leans sound reasonable";
sibling tree ruling in §14). They are recorded as rulings of this
draft, subject to the ratification of the whole.

- **Q1 — Exactness.** No intentional departures in v1. Pin the release;
  reconcile the thesis, Lean4Lean and the kernel; reconcile the Nat
  acceleration list against the pin; resource outcomes and open-engine
  services are specified separately.
- **Q2 — Sidecars.** Sources carry tactic scripts; the build elaborates
  and checks; content-addressed **proof-term DAGs** are the sidecar
  pin, replayed per LS-law 1. Statement, evidence and assumption
  identities stay distinct.
- **Q3 — Int and words.** Follow Lean: `Int` inductive over `Nat`,
  `UInt64`/`BitVec` as structures over `Fin`, kernel acceleration on
  `Nat` only. `std/word` maps onto these; every existing operation gets
  a crosswalk row (§12) validated on division, remainder, shifts,
  widths and boundaries — shared names prove nothing.
- **Q4 — Indexed E-types.** Deferred; one indexed L interface realized
  through an ordinary E representation is tested early.
- **Q5 — Lambdas.** The static profile of §5.3, eliminated by
  elaboration; no interpreted-only `fn`; the dynamic tier is a named
  door with sealed variants (#12) as the ratified alternative.
- **Q6 — Second syntax.** No Lean concrete-syntax reader in v1; an
  explicit-term importer for exports is enough.
- **Q7 — Mathlib.** Small exports early as an independent input source
  (T0); larger replay and definition bridging priced; reals/intervals
  for Arc C is the first integration consumer; library breadth never
  displaces the shard-specific tests.
- **Q8 — Coverage ordering.** The arc stays parked; a bounded
  `tb_len`-class rung is pulled forward into phase 4 (§14), after the
  minimal elaboration and arithmetic slices and before any bulk port.

---

## 12. Migration, priced

**Layers, by what actually changes:**

| layer | content at `5abc600` | treatment |
|---|---|---|
| code | ~10.6k `fn`/`type` bodies | mechanical surface rewrite by one tool; no judgment |
| statements | ~5.3k goals, 307 requirements, 49 axioms | text verbatim under the primitive crosswalk; **meaning preserved by an explicit per-interface migration record** (R8): old declaration, new declaration, changed definitions, assumption changes, and the evidence or review connecting them; intentional strengthening or weakening is a separate, dated decision; statement hashes pinned in the port manifest |
| proofs | ~4.9k hand claims | solver first, cheap agents second, strong agents third (§14) |
| generated certs | `std/sha256` 23.5 MB, `impgen_*_out`, probe blocks | not ported; superseded by validators and tactic-generated terms, which is what CERT.md §1 asked for |
| toolchain | K, `ev`, elaborator, tactics, loader, canon, prove, explain | new code, the arc's own product, front-loaded |
| docs | `TCB.md` roster (§4.3); `TOTALITY.md` → the E-totality section here; `CERT.md` §3/§7 superseded, §4 carried; `LANGUAGE.md` §10 → the tactic surface; `SEARCH.md`'s LS-laws carried | rewritten or carried, per file |

**The primitive crosswalk (initial rows; every row validated in phase
3 before any statement is declared migrated):**

| shard today | L / Lean name | note |
|---|---|---|
| `+ - *` on Int | `Int.add/sub/mul` | |
| `/`, `tmod` | `Int.tdiv`, `Int.tmod` | truncating; today's runtime corner |
| `ediv`, `mod` | `Int.ediv`, `Int.emod` | Euclidean; today's proof-surface division |
| `int_eq`, `le`, `lt` | `decide (a = b)`, `Int.ble`/`decide (a ≤ b)`, `Int.blt` | Bool-valued in E; `Decidable` bridges to `Prop` |
| `band bor bxor bshl bshr` (premised `0 ≤`) | `Nat.land/lor/xor/shiftLeft/shiftRight` on the nonneg carrier | the 0≤ premises become the `Nat` type |
| `Nat` former (literal-packed) | `Nat` | kernel literal ops |
| `(refine Int nonneg)` etc. | `Subtype` | |
| `Word`/`U8`… (std/word) | `UInt*`/`BitVec` over `Fin` | Q3 |
| `Bytes` = `(List U8)` | `List UInt8` | |
| `sym_eq`, `chars_of_sym`, `sym_of_chars` | `String`/`Char` | toolchain-internal today |
| `gen_fresh` | toolchain-internal | never in a statement |

**The coverage arc's kits** (fra_kit ~900, rth_kit 1,070, tb/tbh
kits): statements under the crosswalk; proofs re-derived in phase 4/5;
the generators (`gen_fra.py`, `gen_rth.py`, the B-1c design) become
tactics.

---

## 13. The Lean review — what not to copy, where to be better

The kernel rules are copied exactly by decision (§1.3). The mistakes
worth avoiding are almost all outside the kernel, so parity costs none
of the improvements below.

### 13.1 Do not copy: Lean's real trust holes

- **The compiler bridge.** `implemented_by`, `@[extern]`, `@[csimp]`,
  `native_decide`, `partial`, `unsafe`. Every one lets executed code
  diverge from the checked definition, and most historical Lean
  unsoundness reports live there rather than in the core rules.
  Shard's identity is the opposite: E execution is the definition, the
  lowering is a theorem, the artifact is byte-tied. No `fn` may be
  partial, opaque-with-default, or implemented elsewhere. Externs exist
  only as World bolts with ledger axioms, as today.
- **Ambient trust.** In Lean anything in the environment is citable,
  `sorry` is an ordinary term, and axiom use surfaces only when someone
  runs `#print axioms`. Keep the axiom-scope gate, kind tags,
  per-artifact trusts and the ledger; K's native axiom closure makes
  the ledger cheaper, not optional.
- **Auto-bound implicits and hidden arguments.** A typo becomes a
  universally quantified variable; `@`-mode and coercion insertion hide
  what a term means. For agent authors this is the worst error class.
  V2's surface has explicit binders, explicit instantiation where it
  matters, and named construction, per the ratified LLM-first
  principle. (Today's loader auto-generalizes unknown binder names to
  type variables — the same footgun; it goes.)
- **Heartbeats as the only budget.** Exhaustion in Lean is an error
  indistinguishable from failure; `Exhausted` is a verdict here.
- **A trusted C++ kernel.** Ours is E, and route 1 compiles it through
  its own proven chain.
- **Everything compiles unless marked `noncomputable`, through a
  black-box code generator.** `fn` versus `def` makes intent checkable
  and gives the lowering a total, first-order input.

### 13.2 Be better, given exact rules

- **Verified `decide` instead of trusted `native_decide`** (§5.4).
  Well-founded definitions do not reduce in Lean's kernel, so `decide`
  is stuck on exactly the functions programmers write and the escape
  hatch trusts the compiler. We have `ev` and will have its correctness
  theorem for route 1 anyway; reflection through it is fast and adds no
  trust. Candle's compute primitive is the precedent, proven rather
  than primitive.
- **Interface views as the review surface.** Lean is only now growing
  a module system; `private` is name mangling. `mod.req` views checked
  as well-formed environments (§4.1) are a better design and the gate
  already exists.
- **Proof identity and maintenance economics.** Lean sources carry
  only tactic scripts, so library churn is partly tactic
  nondeterminism. The sidecar law pins the term DAG and re-elaborates
  scripts only on change, content-addressed from day one.
- **Cost semantics.** Lean has no notion of an executable's resources;
  Runs/RunsWithin and the artifact-claim forms carry over.
- **No codata, no partial functions, every equation a theorem.**
  Shard's totality ruling is stricter than Lean's and stays so.
- **Deterministic, budgeted elaboration** with `Exhausted` reported at
  the elaboration layer too, so a proof that "sometimes times out" does
  not exist as a category.

### 13.3 Kernel-level departures considered and declined for v1

Universe cumulativity (would remove `ULift` noise; forfeits the
oracle); dropping `String` literals from the kernel; anything about
`Quot`; induction-recursion, induction-induction, coinduction and
higher inductive types (non-goals; codata is already ruled out). Each
is priced at "loses differential testing for every declaration it
touches" and none has a consumer.

### 13.4 Documented, not fixed

Lean's declarative definitional equality is undecidable and the kernel
algorithm is a sound, incomplete approximation with unfinished
metatheory (Carneiro 2019; Lean4Lean). Parity means we specify the
*algorithm* as the rule and budget it. This is a property of the bar
we chose, recorded so nobody later mistakes agreement with Lean for a
completeness theorem.

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
  proofs and tactic output; never ported.

The port is the pruning opportunity the slimming arc was waiting for.

### 14.3 The porting pipeline (bulk work by cheaper agents)

The existing auto-proof-solver architecture with an LLM as the solver
of last resort:

1. **Solver tier.** Every PORT claim is attempted by the V2 solver with
   a generic script (induction, `simp`, arithmetic reflection, the
   ported `tools/prove` ladder). The old engine re-solved 172 of 182 on
   an earlier corpus; this tier should carry most of the count.
2. **Cheap-agent tier.** Unsolved claims go out in batches with a fixed
   **context pack**: `LANGUAGE.md` (V2), the crosswalk, a
   worked-examples file with one ported claim of every proof shape, the
   module's interface, and the old proof as a structural hint. The
   checker's output is the only feedback loop; budgets are explicit.
3. **Strong-agent tier**, then a ruling for the residue.

Two mechanical guards make the pipeline safe regardless of who solves:
a porting agent **cannot** change a statement, touch an interface, add
an axiom, weaken a requirement, expose a private definition, or alter a
primitive — the manifest pins statement hashes, the axiom-scope and
kind gates refuse, and the view check (T5) refuses interface drift; and
T9 runs *before* the bulk, since a cheap agent with a gotcha list is a
bad agent. The context pack is calibrated on `std` in phase 3, with the
solver/cheap/strong split measured on real numbers before the bulk
begins.

### 14.4 Phases and gates

0. **Ledger.** This document ratified as `FOUNDATION.md` law; the Lean
   release pinned; the rule inventory written; the port manifest
   drafted; `LAYOUT.md` gains `v2/`; GPT-6's review recorded.
1. **K.** `v2/kernel`: K in narrow-compatible E, run directly on Rust
   (route 3). Gate: T0.
2. **Front-end and `ev`.** The V2 loader and reader (s-expr → explicit
   L terms), environment views, the fragment classifier, `ev`;
   `examples/calc` as the first program with explicit-term proofs.
   Gates: T1, T5, T8, conformance on a value matrix.
3. **Elaboration and the first library.** Stages 1–2, core tactics,
   certified arithmetic; `std/list`, `order`, `nat`, `div`, `bits`,
   `arith` re-proved in `v2/std`; the 15 former axioms proved as
   theorems; the porting pipeline built and **measured here**. Gates:
   T2, T3, T9 (small form), the solver/cheap/strong split recorded.
4. **Entrenchment tests.** Before the environment and executable
   representation are sealed: a `tb_len`-class compiler proof with its
   dependencies (the F8a rung, Q8), contextual holes (T4), search
   fidelity (T7), prepared invocation (T6). Measured against 717 lines
   and against the B-1c generator design.
5. **Bulk port by manifest.** Remaining `std` (the floats family as its
   own line — it is the corpus long pole, #37, #39), `models/imp`,
   `x86`, `linux`, `meta`, tools, apps; REGENERATE families produced by
   validators and tactics. Gate: the V2 corpus green on every PORT
   file; every ARCHIVE decision recorded.
6. **The flip.** `v2/` becomes the tree; CI, `bin/`, docs, README and
   memory move; the old tree is archived in history. Gate: the fmt gate
   and the whole DEFAULT corpus on V2.
7. **Resume.** The coverage arc unparks on V2 with B-1c as a tactic;
   Mathlib export at scale as a performance goal (route 1's consumer).

Serial on main, one gate per phase, CI green behind each, generated
files never hand-patched, kernel sources frozen during corpus runs —
the standing laws apply unchanged.

---

## 15. Risks, stated once

- **Elaborator scale** — the real cost; staged (§6.1) so each stage is
  usable and nothing is trusted.
- **Interpreted-K performance** at Mathlib scale — phase 7's second
  half is a performance goal that may need compiled K; nothing before
  it needs Mathlib-scale throughput.
- **Definitional-equality unpredictability** — explicit conversion
  policy and budgets; `Exhausted` is a verdict, never a hang.
- **The Bool/Prop seam** — handled Lean's way (`Decidable`, `decide`);
  E keeps Bool predicates so lowering never sees `Prop`.
- **Fragment erosion** — agents writing L where they meant E; the
  keyword, the loud classifier, and the lowering as authority.
- **Specialization blow-up** — the finite-specialization discipline and
  its refusal (§5.3, T3).
- **Trust in the oracle** — Lean's kernel has had bugs; agreement with
  it is strong evidence, not proof. The hostile battery and the
  independent checkers (Lean4Lean, nanoda) are the second leg.
- **Port drift** — the manifest's pinned statement hashes and the
  migration record (§12) are the only defense against a nearby weaker
  theorem being counted as the old obligation.

---

## References

- M. Carneiro, *The Type Theory of Lean*, 2019 (the rule specification, to be reconciled with the pin).
- M. Carneiro, *Lean4Lean: Verifying a Typechecker for Lean, in Lean*, arXiv:2403.14064 (the additions since the thesis; bounded checking; what is and is not proven).
- Lean 4 kernel sources, `src/kernel`, release pinned at phase 0.
- `lean4export` (export format), `lean4checker`/`leanchecker` (replay through Lean's kernel — comparison harness), `nanoda_lib` (independent checker).
- Lean Language Reference: *Recursive Definitions* (two-stage treatment; well-founded computation vs definitional reduction); *Tactic Reference*, `decide +kernel`.
- Candle (CakeML) — verified HOL Light with a compute primitive; the end-state shape for a verified K and the precedent for §5.4's verified `decide`.
- Repo: `docs/TCB.md`, `docs/TOTALITY.md`, `docs/CERT.md`, `docs/LANGUAGE.md` §10, `docs/COVERAGE.md`, `docs/SEARCH.md`, `kernel/proof.shard`, `meta/sketch/mod.req.shard`, `meta/invoke/prepared.shard`, `tools/search/theorem_scope.shard`, the 2026-07-24/25 and 2026-09-02 kernel-survey records (memory); `SHARD_FOUNDATION_PROPOSAL_v0.3.md`, `SHARD_BOOTSTRAP_ADDENDUM_v0.3.md`, `SHARD_FOUNDATION_FEEDBACK_v0.1.md` (repo root).
