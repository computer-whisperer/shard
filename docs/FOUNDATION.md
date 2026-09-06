# FOUNDATION.md — the shard V2 foundation: a Lean-parity logic over a lowerable core

> **STATUS: DRAFT v0.1 (Fable), 2026-09-05 — a proposal for review, not law.**
> Written after the user's 2026-09-05 direction ("the bar I am tempted to set
> here is parity with Lean … genuinely more ambitious … careful about things
> like HOF in the central language, since the ability to statically lower
> shard programs is still an important tenet"). Companion and counter-proposal
> to GPT-6's `SHARD_FOUNDATION_PROPOSAL_v0.3.md` and
> `SHARD_BOOTSTRAP_ADDENDUM_v0.3.md` (repo root, untracked); their decision
> IDs D01–D13 and B06–B16 are answered one by one in §9 so the three
> documents can be read side by side. Nothing here is implemented. Sizes are
> estimates and say so. Ratification turns this file into the ledger; until
> then every "law" below is a proposed law.

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
specification: universe-polymorphic dependent type theory with an
impredicative proof-irrelevant `Prop`, strictly positive inductive
families with recursors, `Quot`, and the three standard axioms),
written in first-order shard — itself an E program over L-as-data —
compilable by shard's own lowering and, until then, interpreted by the
first-order evaluator the Rust bootstrap already is. On top of K,
keep shard's identity as a **lowerable core**: an executable fragment
**E** — first-order, total by verified measure, statically
instantiated, never closure-bearing — that is the only thing that
lowers to hardware, and that is exactly the language every existing
`fn`, model, and refinement proof already lives in. Everything shard
proves today is restated verbatim; every special kernel rule becomes
a library theorem or an untrusted tactic; the trust floor shrinks
from "35k lines and twelve special rules" to "one type checker,
three axioms, and the Rust host". Parity with Lean is made testable:
**an exported Lean declaration checks in K**. Elaboration and tactics
are the real project and are staged as untrusted meta-layer work.

Two consumers, one engine: an independent mathematical engine that
is not fundamentally incapable of anything agents do in Lean; and a
more capable, LLM-fluent platform for establishing the facts the
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
  theorems carry dozens of proof-DSL gotchas ("chain-intermediate
  rewrite-with has no trailing refl"). That is the cost of a bespoke
  proof language with no training corpus, paid by every LLM author.
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
   and Mathlib as explicit kernel terms; `lean4checker`, Lean4Lean and
   `nanoda` are small independent checkers of the same rules. A
   rule-exact K can be differentially tested against Lean's own kernel
   on hundreds of thousands of declarations. No foundation of our own
   design gets an oracle like that, and the 2026 holes show what a
   from-scratch checker without one costs.
2. **The library.** With rule-exact K, Mathlib arrives by export, not
   by re-derivation. That is D07's "evidence transport" made cheap.
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
future capability: a HOL core refuses the first dependent statement an
agent writes (`Vector α n`, a structure with proof fields, a
universe-polymorphic construction), which is exactly the "fundamentally
incapable" failure the bar rules out; and because its pedigree argument
inverts once the Lean oracle is counted (§1.3). Two claims made for HOL
in that exchange were also weaker than stated: statements migrate
verbatim under CIC as well (`p x = true` is a proposition), and the
soundness risk of a from-scratch dependent kernel is bounded by
differential testing that HOL-of-our-own cannot have. What survives
from the HOL argument is recorded in §3.4, §5 and §6.

---

## 2. The languages, named precisely

Four things that are today conflated get names. The rest of the
document is written in these terms.

| name | what it is | who checks it | lowers? |
|---|---|---|---|
| **L** — the logic | Lean-rule dependent type theory: terms, types, propositions, proofs | K | never |
| **E** — the executable fragment | the subset of L definitions that are shard *programs*; today's entire object language | K (as L) + the fragment classifier | yes, and only E |
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
kernel" (user, 2026-09-05).

---

## 3. K — the kernel, rule by rule

### 3.1 The normative reference

Mario Carneiro, *The Type Theory of Lean* (2019) is the rule
specification; the Lean 4 kernel source (`src/kernel`, pinned at a
named release) is the behavioral reference; Lean4Lean (arXiv
2403.14064) is the reference for what is and is not proven about
those rules (in particular: normalization is *not* a theorem of the
implemented system; K therefore budgets, §3.3). K's own rule document
is a restatement of these in shard's terms, versioned; v1 declares
**zero departures**.

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
plus ω inaccessible cardinals). The
present 15 `kernel/facts.shard` axioms are **retired as axioms** (§4.3).

Kernel extensions: GMP-accelerated `Nat` operations on literals
(`add sub mul div mod gcd beq ble land lor xor shiftLeft shiftRight
pow log2`), each with a defining recursion in the library that the
acceleration must agree with — the same shape and the same pedigree
as Lean, and the same conformance discipline as today's prim tables.

### 3.3 Resource outcomes

`check(env, decl, limits) → Accepted | Rejected | Exhausted`.
`Exhausted` is never acceptance and never rejection. This is Lean's
`maxHeartbeats` made a first-class verdict; it is also the fuel
doctrine of `TOTALITY.md` applied to the checker itself. K is a total
shard program with an explicit budget, as `check_sequent` is today.

### 3.4 What is *not* in K (the surviving HOL discipline)

No unification, no elaboration, no implicit arguments, no typeclass
resolution, no tactics, no simp, no arithmetic decision procedure, no
measure recognizer, no filesystem, no process-global state. K takes
an environment value and a declaration and returns a verdict. The
elaborator and every tactic are untrusted producers of L terms (§6).
This is GPT-6's D09 "small authority" and shard's existing
"verify, never search" law, stated once.

### 3.5 Size and the oracle gate

Reference points: `nanoda_lib` (Rust) and `lean4checker` are a few
thousand lines each. Estimate for K in shard: **6–10k lines** including
level arithmetic, inductive admission with recursor generation, and the
Nat extensions; the recursor generator is the largest single piece.
Gate F1 (§10): K checks `lean4export` output of Lean's `Init` prelude
declaration-for-declaration, accepts what Lean accepts, and rejects a
mutation battery Lean rejects.

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
| `(fn NAME params RET body)` with `(measure …)` | **`fn` = an E definition**: body must pass the fragment classifier (§5); recursion structural (compiled to recursors) or by explicit measure with kernel-checked descent proofs; `fn.eq_N` defining equations generated as theorems | KEPT — the executable keyword, now with a precise meaning |
| — | `def` = any L definition, `noncomputable` allowed, never lowers | NEW |
| `(extern …)` | `opaque` constant with an E-typed signature plus the bolt-axiom pattern of `BOUNDARIES.md`; reachable-extern ledger unchanged | KEPT |
| `(sig fn …)` / `(sig type …)` in `mod.req` | **environment views**: the consumer's environment carries the constant without its body (K cannot unfold it) and carries the interface theorems as checked constants; the impl's environment has the body. Sound because the consumer's environment is a weakening of the impl's (fewer definitional equations, same identities) | KEPT — mechanism unchanged, now stated in K's terms |
| `(refine BASE PRED)`, `refine_val`, `refine_try`, `refine-fact` | `Subtype` `{x : B // p x = true}`; `.val`; `decide`-based downcast; the invariant is the proof field | KEPT as sugar over the library; the kernel registry and the three intercepts RETIRED |
| `(claim NAME GOAL PROOF)` | `theorem NAME : PROP := PROOF` | KEPT; goals may be any `Prop` |
| `(axiom NAME (kind …) GOAL)` | `axiom` with the kind tag; **K tracks the axiom closure natively** (Lean's `#print axioms`); scope gate and kind gate unchanged | KEPT; the ledger's cites walk becomes a kernel query |
| `(requirement …)` / `(fulfills …)` | unchanged: a stated `Prop` and its later proof, same-goal single source of truth | KEPT |
| `(bin …)` with `entry/externs/trusts/requires`, LEDGER block, MET/UNMET, artifact-claim forms | unchanged | KEPT |
| `(import …)` / `(use …)` / QName identity / selective loading / req-scope gate / canonical dedup | unchanged; a resolved declaration's identity is its QName plus content hash in the environment | KEPT |
| `auto` + `.auto.shard` sidecars | sidecars carry **proof terms** (or replay scripts, open Q2 §11); LS-law 1 (replay is the referee) verbatim | KEPT, respelled |

### 4.2 Proof forms (`kernel/proof.shard` roster, every constructor)

| today (kernel rule) | V2 (where it lives) |
|---|---|
| `Refl` | `rfl` — K's defeq |
| `Steps` + `Unfold` | `unfold`/`delta` tactic → explicit `Eq.mpr` over a defining equation, or defeq |
| `Reduce`, `Simp` (+ `stop` fence) | `simp only [...]`-class tactic (meta); fences = the simp set |
| `Compute` (+ `stop`) | `decide` / `rfl` by K's whnf with Nat-literal acceleration; fences = `irreducible` marks |
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

### 4.3 Trust floor before and after

| roster item today (`TCB.md`) | V2 |
|---|---|
| Rust bootstrap interpreter (execution authority) | KEPT (B06 accepted, §8) |
| checker sources `kernel/*.shard` (~35k lines) | K sources (est. 6–10k) + the image loader; typer, reducer twins, rewriter, tactics, measure gate, arith, desugar RETIRED from the roster |
| `kernel/facts.shard` — 15 axioms (mod range, euclidean completion, ring laws, bitwise recurrences, shifts) | **retired as axioms**: `Int` is an inductive over `Nat`; `Nat` ops are defined by recursion; every one of the 15 is a library theorem (Lean proves all of them); the kernel's Nat-literal acceleration is the only "prim" left, tied by conformance to the definitions |
| per-artifact trust scopes (bolts, bridges, kinds) | KEPT verbatim |
| prim tables ×2 (+ the untracked PrimTag fast path = 3) | ONE definition per Nat op in the library; ONE acceleration table in K; ONE E evaluator (§5.4); conformance sweeps definition-vs-acceleration and evaluator-vs-K |
| — | three axioms `propext`, `Quot.sound`, `Classical.choice`, tracked per declaration |

Net: the floor loses fifteen hand-written arithmetic axioms and twelve
special rules, and gains a well-studied three-axiom base with an
external oracle.

### 4.4 Infrastructure

| today | V2 |
|---|---|
| CERT.md dialect (change/exact-conv, base+patch, validators, Runs) | conversion forms become tactics (§4.2); **validators are unchanged** — an E function `valid_P` plus an L theorem `valid_P_sound`, cited per program (this is §10.2 of GPT-6's proposal and CERT.md §4 saying the same thing) |
| STORAGE S1/S2 (text caches, hashed closures) | native: L terms are hash-consed DAGs, environments are content-addressed; S3's arena is the default representation, not a parked item |
| `tools/prove` ladder, `meta/search` engine, `meta/sketch` holes | tactics and metaprograms over L terms with native metavariables (§7); LS-laws 1–3 verbatim |
| `tools/impc`, `models/imp`, `models/x86`, `models/linux`, byte-tie, gates | unchanged E libraries and unchanged gates; their theorem *statements* verbatim (Bool equations are propositions); their proofs re-derived (§12) |
| `tools/shardfmt`, `tools/canon`, `tools/digest` | s-expression lexical layer KEPT; new heads added to the canon grammar |
| `eval.shard` → `check.shard` tower | retired as the mandatory route; retained as a differential path (§8) |
| `tools/explain`, tracer | rebuilt over elaborator goal states (the tactic layer's job) |

---

## 5. E — the executable fragment and the static-lowering law

This section answers the HOF concern directly. **L has lambdas,
quantifiers, and higher-order functions without restriction. E does
not have closures, ever.** The rule that makes both true at once:

### 5.1 Types (E-types)

An E-type is: `Nat`, `Int`, `Bool`, `Char`; a `type`-declared inductive
(parameters only, no indices) applied to E-types; a `Subtype` of an
E-type whose predicate is an E function (`refine`; the proof is
erased); `structure`s whose fields are E-types. No `Sort`, no `Pi`
value, no `Prop`-carrying data at runtime. *Open (Q4): indexed
inductives with erasable indices, later.*

### 5.2 Terms (E-bodies)

Constructor application; `match` with exhaustive patterns over
E-types; fully-applied calls to `fn`s and to library primitives; `let`;
`if`; literals; `decide` on decidable propositions is permitted and
lowers to the Bool it computes. Proof-typed arguments are permitted
and erased. No `Classical.choice`, no proof used computationally, no
`Sort`- or `Pi`-typed value in any position.

### 5.3 Static higher-order: templates, not closures

A `fn` **may** declare a parameter of function type. It is then a
**template**: every call to it inside an artifact's closure must pass
a *named `fn`* (not a lambda, not a partial application, not a value
of function type read from data) at that position, and the lowering
**instantiates** the template per distinct argument tuple. No E
inductive may have a function-typed field; no `fn` returns a function
type. This is the ratified issue-#4 direction ("parametric modules +
instantiate", the static-lowering principle of `ISA.md`) with the
kernel unchanged: the template is an ordinary L definition, its
instantiations are definitionally equal to the specialized body, and
`map`/`fold`/`filter` become one definition each with one proof each.
Polymorphism is monomorphized at the same step, as today.

**Enforcement.** The fragment classifier runs at `fn` declaration
(loud, early, untrusted); the lowering re-checks it on the artifact's
closure (the authority — an artifact that would need a closure is
refused, never approximated). A `def` that happens to satisfy E is
still not a `fn`; the keyword is the author's declaration of intent
to lower.

### 5.4 One evaluator

The reference evaluator `ev` for E is one shard program (today's
`eval.shard` `ev`, environment machine with TCO). It is the definition
of "run". K's whnf on E-terms and `ev` must agree; that is the
conformance sweep's V2 form (today's `compute_expr` ≡ `ev` ≡ `eval.rs`
becomes `K.whnf` ≡ `ev` ≡ Rust). Later, a theorem: `ev` correct with
respect to K's definitional equality on E, the one remaining
large trusted computation (Candle's `cv` shows it can be verified).

### 5.5 Totality

Every `fn` is total; that is a *theorem* of its admission (its
recursor or its `WellFounded.fix` with a checked descent proof). The
`TOTALITY.md` regime carries over as elaboration: the author writes
`(measure E)`; the elaborator emits the descent obligations; they are
discharged by tactics (untrusted) into proof terms K checks; the
offline `admit` classifier stays advisory and out of the trust path.
Unbounded processes take Int fuel, as today. Structural descent is
compiled to recursors and needs no obligation.

### 5.6 What E buys the lowering program

Nothing in `IMP.md`, `MEMORY.md`, `X86.md` or `COVERAGE.md` changes
its object: the generic spec→imp compiler consumes `fn`s, which are
the same first-order total programs as today, now with a precise
membership test. The counted heap, calls and stack, the `except`
clause, register allocation — all unchanged rungs. What changes is the
*certificate* side: Theorem A/B are theorems in L with proof terms,
and B's per-function certificates are produced by tactics rather than
by a text generator (§12).

---

## 6. Elaboration and the surface — the real project

### 6.1 The elaborator (untrusted, meta/)

Staged, each stage useful on its own:

- **Stage 0 — explicit terms.** Fully explicit L terms in s-expression
  form, checked by K. Enough for the oracle gate and for generated
  proofs (validators, reflection). Painful to write by hand;
  that is fine for a quarter.
- **Stage 1 — inference.** Implicit arguments, first-order unification
  with metavariables, universe inference, `let`/`have`, structural
  recursion compilation to recursors, `match` compilation, definitional
  equations (`fn.eq_N`), `noConfusion`/`injection` derivation.
- **Stage 2 — tactics.** A goal-state framework with a named local
  context and the core set: `intro apply exact rfl rw simp only cases
  induction have show change decide omega`-class reflection,
  `termination_by`/`decreasing_by`. The `tools/prove` ladder and the
  search engine become tactics in this framework.
- **Stage 3 — typeclasses and coercions.** Needed for law-bearing
  hierarchies and for Mathlib-style statements; also the point where
  `Decidable` bridges Bool and Prop cleanly.
- **Stage 4 — the compatibility layer.** Today's proof-DSL step
  vocabulary as tactics, so existing scripts replay where their
  semantics were tactic-shaped. Replay percentage is measured, not
  promised.

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
```

*Open (Q6): whether to also accept Lean's concrete syntax as a
second reader for L-only files.* My lean: no in v1 — agents adapt to
syntax in one session and to semantics never; the tooling investment
is in the s-expression canon.

### 6.3 The LLM-first gate

An agent with no memory files and no gotcha list proves a
`tb_len`-class theorem (§12) from `LANGUAGE.md` alone. That gate, not
line counts, is the measure of the surface.

---

## 7. Metavariables, sketches, search (D11/D12)

L terms carry a native metavariable node with a local context and a
substitution (Lean's `mvar` + `MetavarContext`). K refuses any
declaration containing one. `meta/sketch`'s reserved-call holes
migrate to native metavariables; sharing one metavariable at two
positions is the correlation primitive it already has, now
binder-safe. Exact rank/count, stratified grammars, dominance
pruning, "blocked is not UNSAT", and the observer-profile distinctions
of `SEARCH.md` stay library policy. Persistent workspaces, branch
merge semantics and cache keys (GPT-6 §13.10, B15) are deferred to
the meta layer's own ledger; nothing in K depends on them.

---

## 8. Execution and trust (the bootstrap addendum, answered)

**Rust is the enduring bootstrap facility, not an architected-in
component** (user, 2026-09-05). It is an E-evaluator — the narrow
interpreter it already is — kept so that a fresh machine can run shard
from Cargo and sources. It never executes anything but E, it never
gains semantics, and it needs no image format beyond what E's own
loader produces. Because K, `ev`, the elaborator and the tactics are E
programs (§2), the routes that execute K are, in order of preference
as they come online:

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

The trust roster does not change by choosing among them: Rust has
always been trusted-by-review as the executor of shard programs
(`TCB.md` lists it first), `eval.shard` was never verified either, and
the C-class dissolution is a law about artifacts that stands untouched.

Positions on the addendum, in that light:

- **B06 accepted** as stated above: a reviewed, uncertified Rust
  evaluator may execute K for acceptance-grade verdicts. Not new trust.
- **B07/B08 rejected as written.** "Expand Rust wherever it removes
  nested interpretation" and "maintain a Rust implementation of the
  executable semantics" are not adopted. Rust executes E, nothing
  else; parsing, resolution, granted interfaces, use scopes,
  elaboration and typing happen in shard, once. The two-name-systems
  drift the addendum warns of cannot start because there is one name
  system.
- **B11 accepted.** The tower is retained as route 4, the differential.
- **The oracle is not a Rust question.** Differential testing against
  Lean happens on exported terms fed to K; it needs no Rust parsing.

---

## 9. Positions on GPT-6's decisions

| ID | GPT-6 v0.3 | this proposal |
|---|---|---|
| D01 | Shard-owned, Lean-informed dependent foundation; departures allowed | **Lean-rule-exact K**; departures are dated decisions priced against the oracle; zero in v1 |
| D02 | versioned rule package | accepted; Carneiro + pinned Lean release as the normative reference |
| D03 | fixed conversion, no equality reflection, budgets | accepted; explicit-conversion policy as elaboration discipline |
| D04 | E0 executable view, erasure as a theorem-bearing pass | **replaced by E** (§5): a syntactic fragment with a keyword, not a second IR; erasure is trivial because proofs never enter E |
| D05 | intensional program identity | accepted (QName + content hash; validators and observation relations unchanged) |
| D06 | proof graphs from day one | accepted; native in K's term representation |
| D07 | Lean math coverage as long-term target; transport optional | **transport by export is cheap under rule-exactness**; the parity gate is the arc's measured goal |
| D08 | migrate meanings, rewrite proof text | accepted; statements verbatim (§12) |
| D09 | embeddable engine, small authority | accepted as design constraints on K and the elaborator (environment as a value, no I/O); public API deferred |
| D10 | one identity system, environment vs workspace | accepted |
| D11 | native contextual holes with dependent telescopes | accepted as Lean-style metavariables (§7); the substitution/closure obligation is Lean's, already studied |
| D12 | reusable partial-construction proofs | accepted as library work in meta |
| D13 | runtime linking of engine/meta | deferred; a product decision after F5 |
| B06 | uncertified Rust as acceptance authority | accepted |
| B07 | expand Rust execution capability | **rejected**: Rust executes E, nothing else; it is the bootstrap facility, not architecture (§8) |
| B08 | share resolved frontend artifacts where economical | **moot**: there is one front-end, in shard; Rust never parses or resolves |
| B09 | provisional execution without admission | accepted |
| B10–B12 | conformance, CI cadence, rollout | accepted in spirit; the ladder is §10 |
| B13 | public prepared invocation | deferred; the import-cycle wart is fixed by an opaque `InvokeCtx` in meta/invoke's own interface regardless |
| B14 | hole semantics in shard, not Rust | accepted (trivially: Rust has no semantics) |
| B15 | branch isolation / cache keys | deferred to the meta ledger |
| B16 | runtime linking bounded | deferred |

---

## 10. The pathfinder ladder

Each rung is a gate with a falsifiable outcome, in the project's
usual form. No rung ports a proof corpus.

- **F0 — this ledger ratified.** The rule document written (a
  restatement of Carneiro in shard's terms, with the Lean release
  pinned); Q1–Q8 (§11) answered.
- **F1 — K.** K in shard, run directly on Rust. Gate: `Init`'s
  `lean4export` checks declaration-for-declaration with identical
  accept/reject verdicts and axiom closures; a mutation battery
  (universe collapse, scope capture, forged recursor, non-positive
  inductive, `Prop`-to-data elimination, cyclic definition, the six
  2026 exploits restated) rejected; size and per-declaration cost
  measured.
- **F2 — direct execution.** Rust runs K directly (route 3, §8);
  `eval.shard` demoted to the differential route; the load floor
  measured. No new Rust capability is added to pass this rung; if K
  needs something E lacks, the fix is to K.
- **F3 — E and `ev`.** The fragment classifier; `ev` as the one
  evaluator; conformance `K.whnf ≡ ev ≡ Rust` on a value matrix and on
  `examples/calc`; `calc` restated as `fn`s with its theorems as
  explicit terms (Stage 0).
- **F4 — Stage 1–2 elaboration.** `std/list`, `std/order`, `std/nat`,
  `std/div` re-proved with `induction`/`cases`/`simp`/`rw`/`rfl`;
  `Int`'s 15 former axioms proved as theorems; `mod.req` views working
  (a consumer cannot unfold an interface constant).
- **F5 — certified arithmetic.** `omega`-class reflection; `std/bits`
  re-proved; the kernel arith backend deleted.
- **F6 — the compatibility layer.** Replay percentage of the old
  proof DSL over `std/` measured and recorded; what does not replay is
  re-proved by agents.
- **F7 — the parity gate.** A Mathlib export subset (`Mathlib.Data.Nat`
  closure first) checks in K; then the whole export as a performance
  goal (interpreted K may need compilation — §8's end state).
- **F8 — the coverage pathfinder.** `tb_len` re-proved in V2 with a
  named context, `∃` conclusions, and hypothesis-aware `simp`;
  measured against 717 lines. Then the coverage arc unparks on V2:
  the C2b/B kits' statements verbatim, proofs re-derived, B-1c's
  generator redesigned as a tactic.
- **F9 — the LLM-first gate** (§6.3).

Performance decision rule (GPT-6 §16.1, accepted): baselines and
budgets are agreed before each rung runs; no speedup is inferred from
architecture.

---

## 11. Open questions for the back-and-forth

Numbered so GPT-6 and the user can answer by ID.

- **Q1 — Exactness.** Any departure from Lean's rules wanted in v1?
  (Candidates raised and priced: none. Structure eta, `String`
  literals, non-cumulativity, `Nat` extensions all adopted as Lean
  has them.) Each future departure forfeits the oracle for the
  declarations it touches; say so in the decision.
- **Q2 — Proof objects in sidecars.** Explicit proof-term DAGs
  (checked, large, cacheable) or tactic scripts re-elaborated at check
  (small, readable, deterministic but expensive)? Lean stores terms.
  My lean: sources carry tactic scripts; the build elaborates and
  checks; content-addressed term sidecars are the pin, replay per
  LS-law 1.
- **Q3 — `Int` and words.** Follow Lean: `Int` inductive over `Nat`,
  `UInt64`/`BitVec` as structures over `Fin`, kernel acceleration on
  `Nat` only. `std/word`'s constructions map onto these; confirm.
- **Q4 — Indexed E-types.** v1 E-types have no indices. Allow
  erasable indices (e.g. length-indexed buffers whose index is
  computable from the value) later? Not needed by any rung named.
- **Q5 — Template rule details.** Partial application of a named `fn`
  to E-arguments as a template argument (creates no closure if
  specialized)? My lean: v1 no; `instantiate` sugar covers it.
- **Q6 — Lean concrete syntax as a second reader.** My lean: no in v1.
- **Q7 — Mathlib policy.** Rule-exactness makes the *export* check
  free; *using* Mathlib from S needs namespace mapping and Stage 3.
  Which subset first? (Reals/intervals for Arc C is the obvious
  consumer.)
- **Q8 — Ordering against the coverage arc.** The arc stays parked
  through F1–F7; it resumes at F8 on V2. Confirm, or name a rung to
  pull forward.

---

## 12. Migration, priced

- **Statements: verbatim.** Every `claim`/`requirement` goal today is
  `∀ params, premises → l = r` over E-terms; in L it is the same
  proposition (Bool equations are propositions). Definitions, types,
  models, `mod.req` surfaces, `bin` declarations: untouched.
- **Proof text: disposable, re-derived by agents** with the
  compatibility layer as the first pass. Corpus at `5abc600`: 5,309
  claims, of which ~4,900 hand-authored; the generated certificate
  files (`std/sha256` 23.5 MB, `impgen_*_out`) are not migrated — they
  are superseded by validator proofs and tactic-generated terms, which
  is what CERT.md §1 asked for.
- **The coverage arc's kits** (fra_kit ~900, rth_kit 1,070, tb/tbh
  kits): statements verbatim; proofs re-derived at F8; the generators
  (`gen_fra.py`, `gen_rth.py`, the B-1c design) become tactics.
- **Docs.** `TCB.md` roster rewritten (§4.3); `TOTALITY.md` becomes
  the E-totality section of this ledger; `CERT.md` §3/§7 superseded,
  §4 (validators) carried; `LANGUAGE.md` §10 replaced by the tactic
  surface; `SEARCH.md`'s LS-laws carried.

---

## 13. Risks, stated once

- **Elaborator scale** — the real cost; staged (§6.1) so each stage is
  usable and nothing is trusted.
- **Interpreted-K performance** at Mathlib scale — F7's second half is
  a performance goal that may need compiled K; nothing before it needs
  Mathlib-scale throughput.
- **Definitional-equality unpredictability** — explicit conversion
  policy and budgets; `Exhausted` is a verdict, never a hang.
- **The Bool/Prop seam** — handled Lean's way (`Decidable`, `decide`);
  E keeps Bool predicates so lowering never sees `Prop`.
- **Fragment erosion** — agents writing L where they meant E; the
  keyword and the loud classifier are the defense, and the lowering is
  the authority.
- **Trust in the oracle** — Lean's kernel has had bugs; agreement with
  it is strong evidence, not proof. The mutation battery and the
  independent checkers (Lean4Lean, nanoda) are the second leg.

---

## References

- M. Carneiro, *The Type Theory of Lean*, 2019 (the rule specification).
- M. Carneiro, *Lean4Lean: Towards a Verified Typechecker for Lean, in Lean*, arXiv:2403.14064 (what is and is not proven; normalization caveat).
- Lean 4 kernel sources, `src/kernel`, pinned release to be named at F0.
- `lean4export` (leanprover), `lean4checker` (leanprover), `nanoda_lib` (ammkrn) — export format and independent checkers.
- Candle (CakeML) — verified HOL Light with a compute primitive; the end-state shape for a verified K.
- Repo: `docs/TCB.md`, `docs/TOTALITY.md`, `docs/CERT.md`, `docs/LANGUAGE.md` §10, `docs/COVERAGE.md`, `kernel/proof.shard`, the 2026-07-24/25 and 2026-09-02 kernel-survey records (memory), `SHARD_FOUNDATION_PROPOSAL_v0.3.md`, `SHARD_BOOTSTRAP_ADDENDUM_v0.3.md`.
