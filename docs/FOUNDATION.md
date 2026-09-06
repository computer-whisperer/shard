# FOUNDATION.md — the shard V2 foundation

> **STATUS: DRAFT v0.5 (Fable), 2026-09-06 — the normative contract, proposed, not yet ratified.**
> This document states the rules. The history, the user's rulings by
> date, the rejected alternatives, and the round-by-round positions on
> GPT-6's review IDs (D01–D13, B06–B16, R1–R28) live in
> [`records/FOUNDATION.md`](records/FOUNDATION.md). When the two
> disagree, this document wins and the record is corrected. Nothing here
> is implemented; sizes are estimates and say so. Ratification turns
> "proposed law" into law.

Evidence baseline: the tree at `5abc600` (coverage arc parked at B-1b),
`docs/TCB.md`, `docs/TOTALITY.md`, `docs/CERT.md`, `docs/SEARCH.md`,
`docs/BOUNDARIES.md`, `docs/FLOATS.md`, `docs/LANGUAGE.md` §10,
`kernel/proof.shard`, and the 2026 kernel surveys.

---

## 1. The decision and the bar

**The decision.** Replace the equational, first-order, tactic-in-kernel
checker with a kernel **K** that implements **Lean 4's kernel rules
exactly**, written in first-order shard — an E program over L-as-data —
compilable by shard's own lowering and, until then, interpreted by the
first-order evaluator the Rust bootstrap already is. Keep shard's
identity as a **lowerable core**: an executable fragment **E**,
first-order after elaboration, total by verified measure, never
closure-bearing, the only thing that lowers to hardware. Between the
agent-facing tactics and K sits a **proof IR (I)**, a stable
hole-bearing certificate language that search engines navigate and pins
store. One name per mathematical object across the whole system. Every
special kernel rule becomes a library theorem or an untrusted tactic;
the trusted code shrinks to K, the E evaluator and the Rust host; the
logical assumptions shrink from fifteen hand-written arithmetic axioms
to Lean's three.

**The bar** is parity with Lean, not compatibility: in a few years the
engine must not be fundamentally incapable of the reasoning agents wish
to exploit. Parity is testable: **an exported Lean declaration checks
in K** (T0). The goal is one engine with two consumers — an independent
mathematical engine, and a more capable, LLM-fluent platform for the
facts the refinement and lowering program needs — a more powerful
shard, not today's shard with a Lean engine bolted on.

**Why exactness.** Each departure from Lean's rules forfeits, for every
declaration it touches, the differential oracle (`lean4export` plus
independent checkers), the library (Lean's `Init` and Mathlib arrive by
export), and the corpus (agents' intuitions transfer). v1 declares zero
departures; a future departure is a dated decision that names the
forfeit. Agreement with Lean is strong differential evidence for closed
L checking under an identified rule/version/input profile; it is not a
completeness theorem, and it validates nothing about E execution,
holes, views or lowering, which have their own tests (§12.5).

---

## 2. The languages

| name | what it is | who checks it | lowers? |
|---|---|---|---|
| **L** — the logic | Lean-rule dependent type theory: terms, types, propositions, proofs | K | never |
| **E** — the executable fragment | the L declarations that are shard *programs*; today's entire object language | K (as L) + the fragment classifier + the lowering | yes, and only E |
| **P** — proof objects | L terms whose type is a `Prop` | K | never (erased) |
| **I** — the proof IR | a versioned, hole-bearing certificate language elaborated deterministically into P (§7) | its elaborator (untrusted); the result is checked by K | — |
| **S** — the surface | s-expression forms, Lean-flavored; elaborated to L, and tactic blocks to I, by an untrusted elaborator | nothing (untrusted) | — |

An environment is written `Env`; E always means the executable fragment.

**Laws (proposed):**

- **The trichotomy survives.** Requirements are `Prop`s in L, now any
  proposition; algorithms are E definitions; low-level spellings are E
  definitions in a model's vocabulary; refinement is a theorem in L.
- **Programs are data.** An E definition is an L constant with a body;
  `meta/` inspects it as a term. Quotation, sketching and search operate
  on L terms and on I with one metavariable mechanism (§6, §7.4).
- **The toolchain is E; the logic is data.** L's terms, levels,
  declarations and environments are `type`s declared in E. K, `ev`, the
  elaborators and every tactic are `fn`s over that data: first-order,
  total by verified measure, closure-free, lowerable. Higher-order
  functions, dependent types and `Prop` exist only inside the values the
  toolchain manipulates. The initial toolchain stays in the first-order
  E profile; source-level lambda conveniences lower into it (§4.3).
- **E has no function values.** Lambdas, partial applications and
  template arguments are eliminated by elaboration — lambda lifting plus
  specialization — so no E program, `ev` value, Rust value or artifact
  contains a function value or an indirect call. A `fn` either lowers or
  is refused; there is no interpreted-only `fn`.
- **One name per mathematical object.** A `fn` body, a `def` body and a
  `theorem` refer to the same constant by the same spelling (§5.3). No
  executable spelling beside a logical spelling of one operation.
- **The L/E correspondence is by defining equations.** An E
  definition's executable structure is its source body; its L meaning is
  the body's elaboration (recursors; `WellFounded.fix`); the bridge is
  the equations `f.eq_N`, theorems of L (§4.4).

---

## 3. K — the kernel

### 3.1 Declarative rules and a bounded procedure

Two specifications, kept apart. **Declarative:** the judgments
`Env; Γ ⊢ t : T` and `Env; Γ ⊢ t ≡ u` as given by Carneiro, *The Type
Theory of Lean* (2019), reconciled with the pinned Lean 4 kernel release
and with Lean4Lean (arXiv 2403.14064), which records the additions since
the thesis (nested inductives, structure eta) and what is and is not
proven. **Operational:** `check(env, decl, limits) → outcome` (§3.3),
the bounded procedure of the initial compatibility profile. The
implementation obligation is that successful checking implies the
declared judgment under the environment's well-formedness assumptions;
this document does not claim it proved. Algorithmic incompleteness,
unsupported inputs and exhausted resources are limitations of the
procedure, never definitions of the logic. K's rule document restates
the reconciled inventory in shard's terms, versioned.

### 3.2 The inventory

Universe levels `zero succ max imax` and parameters; `Sort u`
non-cumulative; `Prop = Sort 0` impredicative. Terms: bound and free
variables (locally nameless), `Sort`, `Pi`, `lambda`, `app`, `let`,
constant with universe arguments, `Nat` literal, `String` literal,
projection. Definitional equality: beta, delta, iota, zeta, eta
(functions and structures), proof irrelevance, Nat-literal computation,
Quot computation. Declarations: `definition`, `theorem` (opaque for
unfolding), `opaque`, `axiom` (tracked), `inductive` (strict
positivity, universe constraints, mutual and nested per Lean's
admission fragment; K generates and validates recursors), `Quot`.
Axioms of the standard profile and no others: `propext`, `Quot.sound`,
`Classical.choice`; consistency pedigree = Lean's model (ZFC plus ω
inaccessibles). The fifteen `kernel/facts.shard` axioms become library
theorems.

Kernel extensions: GMP-accelerated `Nat` operations on literals,
reconciled with the pinned release at phase 0, each **bound to a fixed
admitted declaration identity** whose signature and defining equations
K validates before the shortcut is enabled; never a name-based match.

### 3.3 Outcomes and the meaning of each negative

```
check(env, decl, limits)
  → Accepted(receipt)
  | Rejected(Malformed | RuleViolation | Unsupported | UnresolvedObligation | ConversionNotEstablished)
  | Exhausted(resource, site)
```

`Exhausted` is never acceptance and never rejection. `Malformed` and
`RuleViolation` are definitive for the submitted input. `Unsupported`,
`UnresolvedObligation` and `ConversionNotEstablished` say only that this
procedure did not establish the judgment: not inequality, not
unsatisfiability, prunable only as a revisitable heuristic (§7.4).
Every negative names its **subject** (this declaration, this node, this
branch, this program/proof pair) and never widens to "all completions
of a region" unless a separate certified-pruning claim says so.

### 3.4 What is not in K

No unification, elaboration, implicit arguments, typeclass resolution,
tactics, simp, arithmetic procedure, measure recognizer, filesystem, or
process-global state. K takes an environment value and a declaration
and returns a verdict. Everything else is an untrusted producer of
L terms.

### 3.5 Inputs: raw versus checked

K is a library; clients construct syntax directly. **Raw** declarations
and environments are one type; **checked** environments are another,
produced only by admission, immutable, never forgeable from a
similarly shaped record or a deserialized receipt. Admission validates
dependent contexts in order, all type and universe fields, constructor
and recursor metadata, references in opaque bodies, cyclic groups, and
the assumptions consumed. A node identity does not carry a type
independently of its context. Programmatic construction through the API
is a normal test route (T0).

### 3.6 Size and the oracle gate

Estimate: 6–10k lines of shard (reference points: `nanoda_lib`,
Lean4Lean's checker; `lean4checker` replays through Lean's kernel and is
not a size reference). The figure is not a gate. The gate is T0: K
accepts `lean4export` of Lean's `Init` declaration-for-declaration with
identical verdicts and axiom closures, rejects a hostile battery whose
members each carry an independently specified declarative reason, and
logs its supported scope. Disagreements are investigated by rule,
mapping and resources; no winner by tool name; no upstream bug is
reproduced for a parity score.

---

## 4. E — the executable fragment

**L has lambdas, quantifiers and higher-order functions without
restriction. E has no function values, ever.** Both hold because the
surface's higher-order conveniences are eliminated by elaboration.

### 4.1 Relevance roles

Executable eligibility is decided after classifying every input, field
and intermediate value by role:

| role | examples | treatment |
|---|---|---|
| **static structure** | type arguments, known function identities, selected operation packages | resolved or specialized before execution; finite specialization required |
| **erased evidence** | a bound proof, a `Prop` witness, an algebraic law | checked in L; erased under the specified translation; provenance retained |
| **runtime data** | integers, buffers, constructor tags, captured values, a decision's tag | preserved through an explicit representation; resources accounted |

Roles are not sorts; one value may carry fields of different roles. A
type parameter used only for checking erases; one that determines
layout specializes. Consequences:

- A supported carrier `A` may be refined by **any** well-formed L
  proposition, `{x : A // P x}`, with the proof erased; deciding `P` is a
  separate capability needed only when the program tests membership at
  runtime. `{p : Program // Realizes p spec}` with an undecidable
  `Realizes` is a legitimate E value: the validator architecture's
  certified-program type.
- A `Decidable P` value keeps its tag and erases its proof payload;
  `(if h (< i (List.length xs)) (List.get xs i h) none)` computes the
  decision once, discharges the access obligation by `h`, and carries
  no runtime proof. Short-circuiting and evaluation order of composite
  decisions are specified, not inferred from the mathematics.
- A statically selected law-bearing package (operations plus laws)
  specializes its operation projections and erases its laws — the
  package analogue of templates; no runtime dictionary.
- Elimination from `Prop` into data happens only where the translation
  says: decision tags, equality transport, `False.elim` on checked
  evidence. A missing proof justifies none of them.
- "No indices" means no index-dependent **runtime layouts** in v1.
  Value-indexed logical interfaces (`Fin n`, `BitVec w`, proof-indexed
  access) are realized through ordinary E representations with erased
  proofs; general indexed inductives are L-only until a consumer prices
  a layout.

### 4.2 Types and terms

E-types: `Nat`, `Int`, `Bool`, `Char`; `type`-declared inductives
(parameters only, no function-typed fields) applied to E-types;
subtypes of E-types per §4.1; structures of E-types. E-bodies:
constructor application; exhaustive `match`; fully applied calls to
`fn`s and library primitives; `let`; `if` on a decidable proposition
(elaborated through the `Decidable` instance — for core types fixed by
the library, not a consequential selection); `decide`; literals;
proof-typed arguments (erased). No `Classical.choice`, no `Sort`- or
`Pi`-typed value in an elaborated E body.

### 4.3 The lambda profile

Supported in v1, in order of introduction: **closed lambdas** as
template arguments; **named partial application** to known E values;
**non-escaping value capture** by lambda lifting, the captured value an
ordinary runtime parameter, never a specialization key; **templates**, a
`fn` with function-typed parameters specialized per distinct static
argument tuple, its theorem proven once in L over arbitrary `f`, each
instance definitionally the specialized body or bridged by a generated
equation where arity changed. **Escape rule:** a function-typed
expression occurs only as a call head or a template argument; never in
a constructor field, a return position, a data structure, or across a
statically unknown call. **Enforcement:** the classifier at declaration
(loud, untrusted); the lowering on the post-specialization closure (the
authority; refusal, never approximation). **Finite specialization:** a
specialization budget is a refusal, not a proof of unlowerability;
`f[A]` recursing at `f[List A]` is the canonical refusal. **Totality
through templates** needs no rule: every E definition is total by L
admission and a template is total for all arguments of its function
type. **The dynamic tier** (escaping closures, function values in data,
runtime dispatch on functions) is a named door: sealed variants (#12)
are the ratified alternative; wake condition = a named workload with a
demonstrated cost or compositional disadvantage under manual sealed
variants.

### 4.4 Realization: `fn`, `realize`, `ev`, and evaluation evidence

**Declaration and realization are two things.** `fn` requests logical
admission and an executable realization together. `realize` attaches a
checked executable realization to an **existing** admitted L
declaration under its existing identity, without redefining it: either
a supported executable view derived from the definition and checked, or
a separately supplied E implementation related to it by a theorem. An
imported or mathematical `def` is not executable by default and gains
no compiler guarantee silently. One mathematical identity may have
several implementation identities linked by evidence; choosing among
them changes execution cost and the realization record, never the
requirement. An imported name is never identified with a native
declaration by spelling.

**The correspondence is semantic, not provenance.** The realization
relation is stated over the **resolved executable structure** — its
cases, callees, representations and recursion — and discharged by
K-checked equation lemmas `f.eq_N` plus `ev`'s general theorem; a
manifest of matching hashes establishes provenance only. A fresh but
semantically wrong executable view must fail acceptance while its
manifest passes (T1). Successful-result correctness and progress are
distinct obligations; `f x = f x` justifies no looping implementation;
mutually recursive groups need a joint well-founded argument.

**`ev` is the definition of "run"**: one shard program, fuelled,
structurally recursive over program data. Its correctness statement is
two-sided: `ev p args n = some v` implies `f args = v` in L, and under
adequate fuel such a `v` exists. Execution routes are compared on
results and observations under explicit resources, never on budgets or
representations.

**Evaluation reflection.** A native value is not a proof. The evidence
for a particular result is `rfl : ev p args n = some v`, which K
establishes by definitional computation because `ev` is fuelled and
structural, unlike the measure-recursive `f` it interprets; `ev`'s
theorem then yields `f args = v`. The I node carries `p`'s resolved
identity, arguments, fuel and result. No native oracle exists. Cost is a
**measurement**: compiling K removes the outer interpreter, not the
inner one; the reflective route is compared against direct execution,
equation-based proof-producing evaluation, and domain witnesses with
proved checkers (§9.4). A validated accelerator for `ev` inside K would
be explicit execution trust with the fixed-identity treatment of §3.2.

### 4.5 Totality

Every `fn` is total as a theorem of its admission: a recursor, or
`WellFounded.fix` with a checked descent proof. The author writes
`(measure E)`; the elaborator emits obligations; tactics discharge them
into I and P; the offline `admit` classifier stays advisory. `Nat`
measures carry no nonnegativity obligation. Unbounded processes take
fuel. Structural descent compiles to recursors and needs no obligation.

### 4.6 Erasure obligations

| construct | obligation |
|---|---|
| proof arguments and fields | removed; E bodies eliminate `Prop` only per §4.1 |
| subtypes | carrier = the base type; introduction checked at elaboration |
| `Decidable` values | tag kept, payload erased |
| `decide` | the selected procedure is an E function |
| impossible branches | removed on checked evidence only |
| static packages | operations specialized, laws erased |
| recursion | source body is the structure; recursor / `WellFounded.fix` is the meaning; equations are the bridge |

### 4.7 Effects: World, threading, and the effect-use contract

World threading stays the source discipline: effects are externs that
thread a World token, uninterpreted in proofs, given by bolt axioms,
performed by the host at run time. **Threading alone is not the
faithfulness argument.** Endpoint clock monotonicity does not imply
unique use (two writes consuming one `w` still advance the endpoint).
Faithful execution requires (i) a **well-threadedness check** on every
`bin` closure — affine use of each World token along every branch,
mutually exclusive branches may share an incoming token, two sequential
uses may not; (ii) a coherent pure model of state, histories and
observations from which the effect laws are derived where possible, with
assumed law bundles tested on small concrete models; (iii) a trace
relation as the effect semantics, so no transformation duplicates,
eliminates, memoizes or reorders a host call because it looks pure.
Pure evaluation refuses reachable externs; effectful execution takes an
explicit handler contract. No linear types in K, no monads, no `do`.
`BOUNDARIES.md`'s inference is corrected accordingly.

### 4.8 Replacement with evidence

Shard permits independently optimized implementations — the refinement
chain is replacement with evidence — and refuses any unaccounted gap
between checked meaning and deployed computation. A transformation
carries source and destination identities, preconditions, the relation
preserved (exact equality, representation simulation, an error bound,
preserved effects or resources — never interchangeable Booleans) and
evidence. Two approximate passes need a composition argument;
`O(p) = O(q)` licenses nothing outside the root observation. Linking
compiled E implementations of K, `ev` or `meta/` into an application is
valid; inserting an interpreter because a lowering failed is not.

### 4.9 What E buys the lowering program

The generic spec→imp compiler consumes post-specialization `fn`s: the
same first-order total programs as today, with a membership test. The
counted heap, calls and stack, the `except` clause, register allocation
are unchanged rungs. What changes: Theorem A/B are L theorems with P
terms, and per-function certificates are I emitted by tactics.

---

## 5. Elaboration and the surface

### 5.1 The elaborator (untrusted), staged

**Stage 0** explicit L terms. **Stage 1** inference: implicit arguments,
first-order unification with metavariables, universe inference,
structural recursion to recursors, `match` compilation, `f.eq_N`,
`noConfusion`/`injection`, the fragment classifier, lambda lifting and
specialization. **Stage 2** the I elaborator, the goal graph (§7.3) and
core tactics (`intro apply exact rfl rw simp_only cases induction have
show change decide`, arithmetic reflection, `termination_by`/
`decreasing_by`); `tools/prove` and the search engine become I
producers. **Stage 3** typeclasses and coercions, consequential
selections scoped and recorded. Compatibility with the old proof DSL is
the re-spelling tier of the port (§12.3), not a stage.

### 5.2 The resolved requirement

Inference may **reconstruct** what the declared inputs determine (an
element type, a universe level). It may not **select** among
semantically consequential alternatives — an ordering, a numeric
interpretation, an implementation package, a coercion route — except
under a declared scope, and every selection is recorded in the
**resolved requirement**: proposition, bound parameters, referenced
declarations, selected instances and coercions, policy and environment
dependencies. Changing any of these is a different requirement and is
reported as such; changing a proof strategy is not. Unknown identifiers
never become parameters. Declared synthesis holes remain fillable
without changing the task. The proof-solving agent does not redefine
the target.

### 5.3 The naming law (RULED 2026-09-06)

**Lean's names, conventions and theorem-naming grammar by default**, as
an explicit decision, because they give the author a complete mental
map — above all the naming grammar that lets a never-seen lemma be
cited by guess. `docs/LEAN.md` holds the map and must fit in dozens of
rows.

Adopted: (1) names and namespaces (`List.length`, `Option.getD`,
`Subtype.val`, `Fin`), namespaces = shard's qualified identities, `use`
= `open`; (2) lowercase constructors and Bool literals (`some none cons
nil true false`; `True`/`False` are propositions); (3) one propositional
set of connectives (`= <= < and or not -> iff forall exists fun`),
`Decidable` bridging in E, `== && ||` for Bool values only; (4) `/` and
`%` on `Int` Euclidean, `Int.tdiv`/`Int.tmod` named, `Nat` subtraction
saturating, words wrapping (§10.4); (5) sizes and indices are `Nat`;
(6) Mathlib's theorem-naming grammar and `f.eq_N`/`f.eq_def`.

Departures: (1) Rust-flavored declaration keywords `fn def type
inductive structure sig theorem realize`; (2) the s-expression prefix
surface; (3) no effect notation; (4) refusals with a pointer to the
replacement — `get!`-style defaults → `get?`/`getD`; `Inhabited`
defaults; `partial` → `measure`; `unsafe`; `implemented_by` → §4.8;
auto-bound implicits → explicit binders; declaration-order instance
selection → §5.2 — each refusal scoped to source sugar, E eligibility,
deployment profile or assumption policy, never to the corresponding L
abstraction (a mathematical monad or `Inhabited` structure is fine);
(5) shard-only vocabulary: `measure`, `mod.req`/`sig` views,
`requirement`/`fulfills`, `bin`, `trusts`, `requires`, World externs,
models, the artifact-claim forms.

Schematic surface (type parameters bound explicitly; implicitness is a
Stage 1 attribute):

```sexp
(theorem List.length_append ((α Type) (xs ys (List α)))
  (= (List.length (List.append xs ys)) (+ (List.length xs) (List.length ys)))
  (by (induction xs)
      (case nil (simp_only [List.append List.length]))
      (case cons (x t ih) (simp_only [List.append List.length ih]) omega)))

(fn  List.length ((α Type) (xs (List α))) Nat (measure (struct xs))
  (match xs (nil 0) ((cons _ t) (+ 1 (List.length t)))))
(def Sorted ((xs (List Int))) Prop
  (forall ((i j (Fin (List.length xs)))) (-> (< i j) (<= (List.get xs i) (List.get xs j)))))
(fn add_offset ((k Int) (xs (List Int))) (List Int)
  (List.map (fun (x) (if (<= x 0) x (+ x k))) xs))
```

### 5.4 `docs/LEAN.md` and the LLM-first gate

`LEAN.md` has three lists: what is the same as Lean; what is refused,
why, at which phase, and what to write instead; what is shard-only.
**T9:** an agent with `LANGUAGE.md` and `LEAN.md` and nothing else
proves a `tb_len`-class theorem, writes a `fn` that lowers, uses one
mathematical name, a ghost invariant and a branch proof without
undocumented E/L workarounds, and receives the shard form when it writes
a refused Lean form. Runs early on small tasks and again at the end;
the seam document's length is a symptom, not the metric.

---

## 6. Metavariables and workspaces

L terms and I carry native metavariables with a declared local
telescope, a dependent expected type, and explicit typed substitutions
at occurrences (`?h : [Ψ ⊢ A]`, `?h[σ]`). K refuses any declaration
containing one. The engine, in meta, provides declaration, occurrence,
assignment (scope, dependent types, universes, direct and indirect
cycles; fresh subholes under an acyclic discipline), open validation
with five outcomes (*validated*, *blocked*, *invalid*, *exhausted*,
*closed-accepted*), and final closure by K recheck.

**Hole kinds.** Sharing IDs, telescopes, substitutions and transactions
across L and I does not identify an unknown L term with an unknown I
derivation or with a goal-polymorphic recipe. Three relationships are
distinct and declared: one closed proof; one contextual template
`?h : [n : Nat ⊢ n = n]` instantiated by substitution; one recipe
(`rfl`) reused at different goals producing different terms.

**The goal graph.** Subgoals depend on each other (a witness hole fixes
the proposition of its proof hole; a type hole fixes later term holes);
the graph records dependencies, assigns one owner per obligation, and
invalidates affected checks after a committed assignment. It is the unit
for parallel agents: independent regions on explicit base snapshots,
merge only of validated compatible patches, never by display name.

**Transactional invariant.** `attempt(snapshot, request, limits) →
Completed(patch, evidence) | Blocked(obligations, proposed_patch) |
Invalid(reason) | Exhausted(resource)`; `commit(snapshot, checked_patch)
→ snapshot'`. A failed or exhausted operation leaves no hidden
assignment. Caches bind environment, telescope, universe and
metavariable assignments, and policy; nothing from one branch
constrains another. Host effects stay outside speculative logic.

The substitution/closure obligation — a validated open derivation with a
well-typed filling instantiates to a valid closed judgment — is
specified for shard's representation; unsound pruning discards
solutions without producing an invalid theorem, which K cannot catch.
Search claims stay separate: candidate correctness, exact enumeration,
region emptiness, representative replacement, optimality. Dependent
spaces are not Cartesian (choose `n ∈ {0,1,2}`, then `i : Fin n`: three
pairs, not six); a count states what it counts.

---

## 7. The proof IR (I)

### 7.1 Three levels

```
tactics ──search, LLM-fluent──▶ I ──deterministic elaboration──▶ P ──▶ K
```

I is a certificate language, not a tactic language. Anything that
searches — `simp` discovering its lemma set, `omega` finding a
certificate, a search engine, an agent — is a **producer of I**, and
what it emits is the result, not the search. Replay elaborates I to P
without search and K checks P. `exact TERM` guarantees reach: every P
is expressible, imported Lean proofs are `exact` nodes; reach does not
guarantee a compact or searchable I form, which is measured.

### 7.2 Vocabulary and the replay contract

v1 vocabulary: `intro exact apply rfl unfold rw simp_only cases
induction have show change decide reflect arith wf sorry` — today's
roster generalized to formulas and a named context, merged where the new
logic makes old distinctions unnecessary, extended through versioned
adapters that produce P and are still checked against the fixed target.

**Replay contract.** Each I node either carries its consequential
choices explicitly or names a versioned, bounded reconstruction with a
fixed input and dependency set: `rw` carries lemma identity,
instantiation, direction and a **guarded occurrence path** (a defined
position in a term view with its expected subterm), not a
pretty-printed index; `induction` carries the motive, generalized
variables and recursor; `simp_only` carries its lemma list, orientation
conventions, side-condition policy and budget; `apply` carries its
instantiations. Reconstruction may compute and match; it never consults
a changing instance database, discovers lemmas, or runs unrecorded
search. **Versioning:** the logical rule version, the I schema and
reconstruction version, the canonical P encoding and the tactic
implementation version are four different things; a new I constructor
changes no mathematical truth; a reconstruction-version change that
yields a different valid P is reported as an evidence change and
migrated explicitly, never a silent overwrite.

### 7.3 The goal graph API

I is a derivation **graph** with explicit child-goal references,
scope-safe local identities (display names are aids), and reconstructed
P boundaries. The I elaborator exposes, as E functions over E data:
`goal_of(node)` (the goal state at a node: named context, target,
metavariables — incremental, not a full-prefix replay per action);
`applicable(goal, env, policy)` (I forms whose side conditions hold,
within a declared fragment and budget — an empty result is not a
completeness claim); `step(goal, form)` (subgoals or an outcome per
§3.3, with the term builder returned **as data**, a defunctionalized
construction record, never a host callback); `elaborate(I)` (the P
DAG). A search engine is any E program composing these; an engine
improvement never changes what a pin means; any engine's solution is
checked identically by any other (T10). `tools/explain` and the tracer
become renderers of goal states.

### 7.4 Holes in I, and refutation scope

A partial proof is I with metavariables whose expected types are goal
states; shared holes are correlated choices; grammars over I forms are
`meta/sketch` grammars; exact counting, stratification and dominance
carry. The lock-step law of `SEARCH.md` becomes joint search over (E
term with holes, I with holes) under one metavariable context.
**Refutation scope:** `not ValidPair(program, this_proof)` does not
imply `not exists proof`. A rejection names its subject; certified
pruning attaches reason, snapshot, region, guards and claim type; a
failed proof step preserves a valid program candidate; heuristic
failure and exhausted enumeration never count as `UNSAT`.

### 7.5 Pins and release evidence (RULED 2026-09-06)

Sources carry tactic blocks. The build elaborates tactics to I, I to P,
and K checks P. **A pin stores I and the required P dependency closure**
in a deduplicated content store: I for explanation, navigation and
reconstruction; P for direct independent checking. `verify_release`
loads retained P and checks the fixed target and policy without any
elaborator; `reconstruct` elaborates pinned I with its version and
compares the resulting P. Warm verification never reruns I. P's
external encoding is canonical — independent of arena addresses, hash
iteration order, allocation history and schedule; scope-safe binding;
ordered graph references; universe arguments and immutable declaration
references recorded; display names in a separate map. Proof identity is
by canonical serialization, never by normalization, proof irrelevance or
a user `BEq`. Hand-written and engine-written proofs are
indistinguishable at the pin; a better engine may later replace a hand
proof with no statement changing. `sorry` is reported loudly and never
accepted. LS-law 1 (replay is the referee) stands.

---

## 8. Identity, views, and acceptance

### 8.1 Six acceptance records

The requirement and its semantic dependencies; the public interface;
the implementation and its executable view; the evidence and its
assumption closure; the compiled realization and target observations;
the checking engine, version and execution dependencies. Assumption
policy is checked at every acceptance boundary: two proofs of one
proposition may have different assumption sets and proof irrelevance
does not merge the records. The artifact-claim forms of `MEMORY.md` D8
and the trust ledger are unchanged.

### 8.2 Views

A `mod.req` is an **environment view** with three checked conditions:
**view validity** (the interface is a well-formed environment on its
own; every exported type and statement typed by exported declarations
and equalities alone — today's req-scope gate); **implementation
matching** (the impl environment extends the view's signature with
bodies of the declared types and proofs of every requirement; bodyless
signature constants and required laws are represented in L as
parameters, as permitted assumptions awaiting an instance, or as opaque
declarations backed by checked evidence — hiding a body manufactures no
proof); **evidence binding** (an exported theorem stays bound to the
checked implementation or to explicit parameters). A consumer checks
against the view alone; K cannot unfold a view constant. Three meanings
of "do not unfold" stay separate: interface opacity (the view), tactic
transparency (elaborator policy), kernel conversion (never modified to
emulate either).

### 8.3 Identity

A declaration's identity is its qualified name within a **logical
package root** plus the content hash of its revision; physical location
is separate, so relocating `v2/` at the flip changes no identity. An
abstract interface slot and the concrete body filling it are two keys.
A concrete definition with a checked body is immutable; changing the
body is a new revision, and a proof that unfolded the old body is not
retargeted. Nominal types never collapse by shape or spelling; an
imported declaration is identified with a native one only by a declared,
validated mapping. Names are hygienic; evidence references survive
display renaming.

### 8.4 Dependency classes and invalidation

At least: logical declaration and type dependencies; body-unfolding
dependencies; evidence and assumption dependencies; executable-build
dependencies (including a build that inspected an I derivation or a
proof body to choose a representation). A proof-only change re-runs
acceptance policy and invalidates only builds that inspected the proof.
An implementation-only change invalidates prepared and inlined
execution and any client proof that unfolded the body, and no abstract
client proof. Nothing is invalidated because all files live under one
directory hash.

---

## 9. Execution and trust

### 9.1 Rust and the four routes

Rust is the enduring bootstrap facility, not architecture: it
implements E's operational semantics, defines no independently evolving
front-end and no L rule, executes E only, and is trusted by review as it
always was. Routes for executing K, in order of preference as they come
online: (1) K compiled by shard's own lowering — a proven artifact;
(2) K interpreted by compiled `ev`; (3) K interpreted by the Rust
bootstrap directly (today's `eval direct`); (4) the full tower as the
differential, never mandatory. Execution dependencies differ by route
and are recorded (§8.1). A claim that Rust is verified is a separate
claim with separate evidence.

### 9.2 The cold bootstrap route

The V2 toolchain's own sources (K, `ev`, loader, elaborators) are
written in the **narrow-compatible E profile** — first-order, no
surface sugar needing elaboration. The Rust loader reads them exactly
as it reads `kernel/*.shard` today (this is its entire parsing role,
tested as such); the loaded toolchain then resolves, elaborates and
checks everything else, including its own inline proofs, which until
then are pending claims under the reviewed host, never assumptions.

### 9.3 Embedding boundaries

Erasure moves obligations to boundaries. A host supplying raw bytes, a
pointer or an integer has supplied no invariant; every library entry is
either **checked** (raw inputs → validation or error → invocation) or
**preconditioned** (the caller assumes the stated invariant, recorded
in the contract). A parser returns a validated representation or an
error; an `Expr`, `Env` or I node built as data is not thereby
well-typed, checked or accepted (§3.5). A **prepared handle** binds the
entry, its declaration and environment revision, the selected
realization, argument and result representations and the execution
policy; a workspace edit never retargets it; release, invalid
arguments, cancellation and reentrancy are specified. Bulk buffers use
explicit views with format, shape, length, alignment, aliasing and
lifetime rules, never literal term trees; a proof about buffer contents
is valid for a state, so calls take read-only ownership, a version
relation, or explicit pre/postconditions.

### 9.4 Budgets and reclamation

Resource limits apply to decoding, allocation, literal sizes, term and
dependency depth, primitives, reconstruction and checking, not only to
checker steps; an uninterruptible host primitive gets a size guard.
Logical fuel and host timeouts are different records; if fuel
monotonicity of successful results holds it is stated and tested.
Exhaustion never mutates committed state, is never cached as a negative,
and never poisons a checked environment. Long-lived contexts have
explicit ownership of terms, checked declarations, snapshots, evidence
caches and prepared executables; releasing a branch makes its unshared
data reclaimable; a durable P archive needs no live decoded graph;
cache policy is distinct from authoritative state. Reflection cost is
measured (§4.4), not inferred from compilation.

---

## 10. Migration

### 10.1 Layers

| layer | content at `5abc600` | treatment |
|---|---|---|
| code | ~10.6k `fn`/`type` bodies | typed migration by one tool (§10.2) |
| statements | ~5.3k goals, 307 requirements, 49 axioms | under the migration table; meaning preserved by a per-interface record: old and new declaration, changed definitions, assumption changes, the connecting evidence; strengthening or weakening is a dated decision; resolved requirements pinned |
| proofs | ~4.9k hand claims | re-spelled into I where I-shaped; solver, cheap agents, strong agents for the rest (§12.3) |
| generated certs | `std/sha256`, `impgen_*_out`, probe blocks | not ported; superseded by validators and tactic-emitted I |
| toolchain | K, `ev`, elaborators, tactics, loader, canon, prove, explain | new code, front-loaded |
| docs | `TCB.md` roster; `TOTALITY.md` → §4.5; `CERT.md` §3/§7 superseded, §4 carried; `LANGUAGE.md` §10 → the surface and I; `SEARCH.md` LS-laws carried; `BOUNDARIES.md` corrected per §4.7 | rewritten or carried |

### 10.2 Three migration classes

| class | example | record |
|---|---|---|
| name-only | `Some` → `some`, `len` → `List.length` | resolution-preserving rename |
| typed representation change | `Int` length → `Nat` length; Bool predicate → proposition plus decision | typed transformation with a correspondence; `Int`→`Nat` only where the value is a size and no sign logic exists (`capacity - used` then `< 0` is not a size); a Bool-returning function keeps a `decide` bridge |
| deliberate behavior change | stuck or trapping zero division → a defined value | an approved contract change with an affected-use review |

The tool labels each edit by what it does. **Equality is not one
relation**: syntactic identity, mathematical equality, executable
Boolean comparison, bit-pattern equality, observational equivalence and
approximate relations are recorded separately; floats are the stress
case (IEEE `==` is not reflexive at NaN; bit identity distinguishes
signed zeros; a tolerance relation is not transitive and is never a
congruence or a key). Imported `Float` theorems are not aliased onto
`FLOATS.md`'s model by name. The machine port: a trapping target
division needs a proven guard, an alternate path, or refusal — a
lowering premise change, the mathematics reusable.

### 10.3 The migration table

The right column is the language; the left exists for the tool and the
records. Exceptional-input behavior is part of each operation's
meaning. Every row validated in phase 3 before any statement is
declared migrated.

| old spelling | V2 spelling | exceptional behavior | class |
|---|---|---|---|
| `+ - *` on Int | `+ - *` | none | name |
| `/`, `tmod` | `Int.tdiv`, `Int.tmod` | `x/0 = 0`, `tmod x 0 = x` (was stuck/trap) | behavior |
| `ediv`, `mod` | `/`, `%` | `x/0 = 0`, `x % 0 = x` (was stuck) | behavior |
| `int_eq`, `le`, `lt` | `=`, `<=`, `<` (Prop; `Decidable` in E); `==` for Bool values | none | typed |
| `andb orb notb` | `and or not` (Prop); `&& \|\| !` for Bool values | | typed |
| `True`/`False` (Bool ctors) | `true`/`false`; `True`/`False` are propositions | | name |
| `Some None Cons Nil Pair` | `some none cons nil Prod.mk` | | name |
| `len append rev inth memb` | `List.length` (`Nat`), `List.append`, `List.reverse`, `List.get?`, `List.contains`/`List.Mem` | `get?` is `Option`; `getD` is the named default | typed |
| `band bor bxor bshl bshr` (premised `0 ≤`) | `Nat.land lor xor shiftLeft shiftRight` | none on `Nat` | typed |
| `Nat` former | `Nat` | `Nat.sub` saturates | name |
| `(refine Int nonneg)` | `Nat` where a size; `Subtype` otherwise | | typed |
| `std/word` widths | `UInt*`/`BitVec` over `Fin` | wrap | typed |
| `Bytes` | `List UInt8`; opaque `Bytes` kept as ours | | typed |
| `(record …)`, `F_of`/`with_F` | `structure`, projections | | name |
| `sym_eq chars_of_sym sym_of_chars gen_fresh` | `String`/`Char`; toolchain-internal | | name |

### 10.4 Conventions (Q9)

Copy a convention when it is the discipline's consensus and right for
shard, not because Lean did it. Copied: the totalizations of the
mathematical primitives (`x / 0 = 0`, `x % 0 = x`, `Nat` subtraction
saturating, words wrapping) — the cross-prover consensus, unconditional
lemmas where possible, imported statements keep their meaning, and
shard's honest signal for a violated invariant is a proof obligation,
not a panic. Consequence: a compiled program that divides by zero
returns zero; **division by zero is not a fail family** (D8 keeps
overflow, oom, stack as properties of the lowering); a program that
wants a trap uses `checked_div : Int → Int → Option Int` or
`div_pos (x y : Int) (h : 0 < y)`, separately named with bridge
theorems; a totalized primitive creates no nonzero-divisor obligation
by itself. Not copied: conveniences that hide failure or choice (§5.3).
Kept as ours where stronger: `FLOATS.md`'s proven float formats (Lean's
model is a comparison reference), `Str` over validated bytes, the
measure regime, `mod.req`.

---

## 11. The Lean review — borrow, refuse, improve

**Refused as defaults:** silent task drift (auto-bound implicits,
order-dependent instances); ambient trust (policy is mandatory at the
boundary, not available on request); unaccounted replacement
(`implemented_by`, `extern`, `partial`, `unsafe`, each refused for its
own reason; `csimp`-style theorem-directed rewriting is legitimate);
hidden speculative state; conveniences that hide failure; exhaustion as
an indistinguishable error.

**Borrowed and strengthened:** dependent abstraction, proof terms, local
inference, typeclasses, broad simplification, classical reasoning,
quotients, theorem-directed computation. Lean has stored proof terms,
proof-producing evaluation (`cbv`, `decide_cbv`), a module system with
public/private scopes, and a kernel-reducible float model; shard's
strengthening is their **integration in one engine**: evaluation
reflection with per-invocation evidence; views as the review surface;
six acceptance records with dependency classes; the proof IR; cost and
observation contracts; `fn`/`def`; no codata, no partial functions,
every equation a theorem; a route to a certified checker. E ownership is
integration and a route, not a correctness proof.

**Not reopened in v1:** the totality and process policies; kernel-level
departures (cumulativity, `String` literals, `Quot`,
induction-recursion, induction-induction, coinduction, HITs) — each
priced at "loses the oracle for every declaration it touches".

**Declarative rules and experiments.** Conversion is instrumented before
it is extended; conversion plans and expected-type-directed checking are
experiments under unchanged rules; presentations with fewer definitional
equalities (Lean4Less) are research; no v1 departure is authorized.

---

## 12. The Foundation arc

### 12.1 The sibling tree (RULED)

V2 is built in `v2/` — `v2/kernel`, `v2/meta`, then `v2/std` —
committed to main, while the old tree keeps checking the old corpus and
serves as the oracle for ported modules. The **logical package root** is
declared at phase 0 so that the flip (phase 6) is a deployment change.
`LAYOUT.md` gains the `v2/` rule at phase 0.

### 12.2 The port manifest

Every file gets one label before any agent touches it: **PORT**,
**ARCHIVE** (stays in history; candidates from the slimming census:
`tools/wasmgen`, `tools/x86gen`, `models/riscv`, `models/pio`,
`models/wasm`, the frozen `impgen` oracles, diff drivers — each family
the user's call; small regression fixtures retained), **REGENERATE**
(generated certificate text replaced by validator proofs and
tactic-emitted I).

### 12.3 The porting pipeline

Tier 0, the migration tool: the naming law and the typed migration
classes (§10.2) applied with per-edit labels; old proofs whose steps are
I-shaped re-spelled into I with named hypotheses. Tier 1, the solver:
a generic script (induction, `simp_only`, arithmetic reflection, the
ported `tools/prove` ladder). Tier 2, cheap agents with a fixed context
pack: `LANGUAGE.md`, `LEAN.md`, the migration table, a worked-examples
file with one ported claim of every proof shape, the module's
interface, the old proof as a structural hint; the goal graph's output
is the only feedback. Tier 3, strong agents, then a ruling. Guards: a
porting agent cannot change a resolved requirement, touch an interface,
add an axiom, weaken a requirement, expose a private definition, alter a
primitive, or change a selection — the manifest, the axiom gates and the
view check refuse; T9 runs before the bulk. Calibrated on `std` in
phase 3.

### 12.4 Phases

0. **Ledger.** This document ratified; the Lean release pinned; the rule
   inventory and procedure written; the package root, the relevance
   rules, the I reconstruction contract and the core-library identity
   policy (§4.4: Lean's `Init` for the shared mathematical types)
   decided; the port manifest drafted; `LAYOUT.md` gains `v2/`; the
   translations that remain trusted during bring-up named.
1. **K.** `v2/kernel`, narrow-compatible E, route 3; raw-input
   validation and immutable checked environments; decoding and
   primitive budgets; the concrete cold-start route. Gate: T0.
2. **Front-end and `ev`.** Loader and reader to explicit L; views; the
   fragment classifier with relevance roles; `ev`; `examples/calc`.
   Gates: T1 (including a decision tag with erased payload, a
   branch-local proof, raw versus checked arguments), T5, T8 replay
   half, conformance.
3. **Elaboration, I, the first library.** Stages 1–2; the I elaborator
   and goal graph; core tactics; certified arithmetic; the `Init`
   import with E realizations attached; `std/list`, `order`, `nat`,
   `div`, `bits`, `arith` under the naming law; the fifteen former
   axioms as theorems; the migration table validated; one arbitrary-Prop
   ghost refinement; one static law-bearing package; `LEAN.md`; the
   first release bundle with retained P; the pipeline measured. Gates:
   T2, T3, T9 (small), T10.
4. **Entrenchment.** A `tb_len`-class compiler proof; coupled
   program/proof holes and refutation scope (T4, T7); incremental goal
   states; prepared invocation, stale handles, buffer rules and
   reclamation (T6); evaluation reflection and its cost comparison (T8);
   the World-use and model tests before any effectful certificate is
   called ported.
5. **Bulk port** by manifest and migration class; floats as their own
   line; REGENERATE families produced by validators and tactics. Gate:
   the V2 corpus green on every PORT file; every ARCHIVE decision
   recorded.
6. **The flip.** `v2/` becomes the tree without identity changes; cold
   replay of accepted P and selected I reconstructions; CI, `bin/`,
   docs, README and memory move. Gate: the fmt gate and the DEFAULT
   corpus on V2.
7. **Resume.** The coverage arc unparks with B-1c as an I-emitting
   tactic; Mathlib export at scale as a measured performance goal;
   optional kernel experiments stay off the critical path.

Serial on main, one gate per phase, CI green behind each, generated
files never hand-patched, kernel sources frozen during corpus runs.

### 12.5 The acceptance battery

The count is agreed before any result is read — source lines, unique
evidence nodes, warm work, cold load, peak memory, emitted code,
interventions — never one number.

| test | experiment | failure reveals |
|---|---|---|
| **T0 oracle and raw checking** | `Init` export declaration-for-declaration; hostile battery with declarative reasons (universe collapse, scope capture, forged recursor, non-positive inductive, illicit `Prop` elimination, cyclic definition, same-spelled non-core `Nat.add`, the six 2026 exploits); direct malformed construction through the API; invalid context and inductive metadata; normalized-universe cases; fixed-identity primitive validation; scope logged | rule mismatch, mapping error, budget difference, forgeable inputs |
| **T1 realization and migration** | structural, measure, subtype-producing and `decide` cases with their equation bridges; a fresh-but-wrong executable view fails while its manifest passes; an omitted branch, a wrong callee, a tautological equation for a looping recursion; a decision tag with erased payload; a branch-local bound proof; an arbitrary-Prop subtype; `Nat`-underflow-sensitive code; zero-divisor target behavior | a missing bridge; provenance mistaken for correspondence; a mislabeled migration |
| **T2 static abstraction** | closed lambda, named partial application, captured-value `add_offset`; a static law-bearing package with no runtime dictionary; the generic theorem reused | source restriction mistaken for artifact restriction |
| **T3 bounded specialization** | the type-growing recursion refuses loudly; a large finite workload within limits | totality mistaken for compiler termination |
| **T4 holes** | shared hole under renamed binders; dependent expected type; template versus recipe; witness/proof dependency; blocked comparison; closure; failure and exhaustion leave the snapshot unchanged; forked branches on one hole do not merge by name | incoherent open-construction discipline; hidden state |
| **T5 views and identity** | consumer with the view alone and with the impl linked; a private-equality leak refused; two validated instances of one interface; physical relocation changes no identity; two same-spelled nominal types not conflated; an import identified only by declared mapping | invalid weakening; identity drift |
| **T6 embedding** | construct, prepare once, invoke repeatedly, transform, check, lower under one identity; erased-invariant argument validation; a stale handle refused after an implementation edit; a buffer shorter than its shape; mutation after validation; bounded live state after many fork/fail/release cycles | CLI dependence; hidden preparation; wrong invalidation; leaks |
| **T7 search fidelity** | correlated holes over ground truth; the `n / Fin n` dependent count; a root-only rewrite invalid nested; a failed proof preserving a valid program; an empty bounded `applicable` not treated as `UNSAT`; a cache entry refused under a changed context; approximate relations not silently transitive | wrong pruning, counting or reuse |
| **T8 acceptance and replay** | direct P verification without I; reconstruction-version drift reported, not overwritten; canonical P encoding stable across allocation orders; a tampered reflection value refused and correct evidence replaying cold; a prohibited-axiom proof failing policy under an identical proposition; `Exhausted` never a receipt; the route recorded | pending evidence, stale caches, an unstated trust transition |
| **T9 authoring** | §5.4 | folklore; task drift; a seam in the wrong place |
| **T10 proof IR** | replay unchanged under changed ambient simp and instance settings; a dependent motive; reused local names in sibling branches; a guarded occurrence whose source changed; an external P accepted through `exact`; an I schema upgrade separate from a foundation change; one engine's solution checked identically by another | an IR only its producer can read |

Performance rule: baselines and budgets agreed before a rung runs;
conversion instrumented on one compiler certificate, one theorem and one
search operation before any extension; no speedup inferred from
architecture.

---

## 13. Risks

Elaborator scale (staged, untrusted); interpreted-K and reflective-`ev`
performance (measured; route 1 is the remedy, not a promise);
definitional-equality unpredictability (explicit policy, budgets,
`Exhausted`); the Bool/Prop seam (`Decidable`; E keeps Bool at
runtime); fragment erosion (keyword, classifier, lowering as
authority); specialization blow-up (T3); IR ossification (`exact`
reach; versioning; measured navigability); oracle trust (differential
evidence, not proof; the hostile battery and independent checkers);
port drift (pinned resolved requirements and migration records);
identity drift at the flip (the package root, T5); effect
unfaithfulness (§4.7, tested before effectful ports).

---

## References

Carneiro, *The Type Theory of Lean* (2019); Carneiro, *Lean4Lean*
(arXiv 2403.14064); the Lean 4 kernel sources at the pinned release;
`lean4export`, `lean4checker`/`leanchecker` (replay harness),
`nanoda_lib` (independent checker); the Lean Language Reference
(recursive definitions; the tactic reference incl. `decide +kernel`,
`cbv`, `decide_cbv`; validating proofs; modules; natural numbers;
subtypes; floating-point numbers; headers and signatures; instance
synthesis; simplifier configuration; the 4.33 release notes); Vaishnav,
*Lean4Less* (research precedent); Candle (CakeML). Repository:
`docs/TCB.md`, `docs/TOTALITY.md`, `docs/CERT.md`, `docs/SEARCH.md`,
`docs/BOUNDARIES.md`, `docs/FLOATS.md`, `docs/MEMORY.md`,
`docs/LANGUAGE.md`, `docs/COVERAGE.md`, `kernel/proof.shard`,
`meta/sketch/mod.req.shard`, `meta/invoke/prepared.shard`,
`tools/search/theorem_scope.shard`; the GPT-6 documents under `docs/archive/foundation-v2/`, listed in
`records/FOUNDATION.md`.
