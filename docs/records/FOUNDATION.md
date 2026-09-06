# records/FOUNDATION.md — the V2 foundation: history, rulings, and the review loop

> The dated record behind `docs/FOUNDATION.md` (the normative contract).
> Everything here is history or argument: the user's words, the
> decisions by date, the rejected alternatives with their reasons, and
> the round-by-round positions on GPT-6's review IDs. Nothing here is
> law that the normative document does not also state; when they
> disagree, the normative document wins and this file is corrected.
> Opened 2026-09-05; split out of the normative document at v0.5
> (2026-09-06) per the ledger-split convention (`docs/records/`).

## 1. Version history

| version | date | commit | what changed |
|---|---|---|---|
| v0.1 | 2026-09-05 | `5d95b81` | first proposal: Lean-rule-exact K in shard, E as the lowerable core, four execution routes, crosswalk, pathfinder ladder, Q1–Q8 |
| v0.2 | 2026-09-06 | `d6b25f1` | GPT-6 feedback R1–R8 folded; Q1–Q8 ruled; static lambda profile; Lean review; Foundation-arc plan with the sibling `v2/` tree |
| v0.3 | 2026-09-06 | `524b59c` | GPT-6 follow-up R9–R16 answered; evaluation reflection with per-invocation evidence; declarative rules vs the bounded procedure; conventions ruled (Q9); the proof IR (I) |
| v0.4 | 2026-09-06 | `23094d7` | the naming law (one name per mathematical object; Lean's names by explicit decision; five departures; `docs/LEAN.md`); §12.1 as a pure migration table |
| v0.5 | 2026-09-06 | this commit | GPT-6 integration review R17–R28 folded; the normative/records split; the two rulings of 2026-09-06 (Lean's `Init` as the core library identity; durable P closure for releases) |

The GPT-6 documents (archived under `docs/archive/foundation-v2/` at v0.5; they were untracked root files until then):
`SHARD_FOUNDATION_PROPOSAL_v0.3.md`, `SHARD_BOOTSTRAP_ADDENDUM_v0.3.md`,
`SHARD_PROPOSAL_CHANGES_v0.3.md` (its proposal, D01–D13, B06–B16);
`SHARD_FOUNDATION_FEEDBACK_v0.1.md` (R1–R8, on v0.1);
`SHARD_FOUNDATION_FOLLOWUP_MEMO_v0.1.md` (R9–R16, on v0.2);
`SHARD_FOUNDATION_INTEGRATION_REVIEW_v0.1.md` (R17–R28, on v0.4).
The user points GPT-6 at each commit by SHA; positions are answered by
ID so the documents read side by side.

## 2. The user's rulings, by date (quote-grade)

**2026-09-05 — the reopening.** "The existing kernel has some
properties that reflect the early MVP nature of what we tried to build
first, and I think there are some solid arguments to work on a V2 now."
"Even today most of the innovations I see coming out of the major AI
labs involve a footnote in Lean … the increasingly central role of
mathematical proof structures in software development, and not just the
refinement proofs we built the initial versions of shard for. The bar I
am tempted to set here is parity with Lean. Not that we have to be 1:1
compatible with it, or even constrain ourselves to the same logical
structures and capabilities, but that in a few years we won't find our
system fundamentally incapable of the kinds of reasoning LM agents may
wish to exploit." "I do think we should be genuinely more ambitious."

**2026-09-05 — the kernel's implementation language.** "The kernel
should still all be written in first-order shard, able to be compiled,
and able to be interpreted by its first-order shard evaluator. I think
the higher order functions exist in what the logical core is able to
reason about, not that the kernel itself should become dependent on
them to run." Rust: "the enduring bootstrap facility rather than an
architected-in requirement." → the law "the toolchain is E; the logic
is data"; the four execution routes.

**2026-09-06 — the primary goal.** "Empowering the logical engine in
shard to be used in widely more problems and scenarios, all being
useful as both an independent math engine as well as a more
sophisticated and llm-friendly platform for establishing the facts
necessary for what we have been doing in shard up until now. I still
want to be careful about things like HOF in the central language, since
the ability to statically lower shard programs is still an important
tenet."

**2026-09-06 — the HOF ban.** "Back when we started shard, I saw HOF as
an unnecessary complexity that resisted the static lowering vision … I
still see shard as primarily a compiled language, and we need to hold
the line that normal shard does not require a complex evaluation
runtime, but I admit the original ban was possibly too harsh." → the
static lambda profile; no function values in E; the dynamic tier as a
named door.

**2026-09-06 — the leans and the sibling tree.** "Your leans sound
reasonable" (Q1–Q8). "A v2 dir for kernel, meta, and perhaps std as we
work out the details seems defensible to me."

**2026-09-06 — the mega-arc and the Lean review.** "Making sure we have
a clean mega-arc plan for this replacement (given it will probably
partially invalidate every single shard file we have written so far),
and that we take a moment to review if there are any core mistakes in
Lean we should take care to not copy ourselves. … I am confident we can
arrange bulk porting via cheaper llm agents if we can set up the
context right, but that is the thing to manage. … we have the
opportunity to be better than lean if we see an opportunity for it."

**2026-09-06 — conventions, delegated.** "I don't think completely
copying lean here is mandatory, but it may make sense in a lot of
situations. The classic example is Matlab, where if we were trying to
make our own version of it I would still emphasize that indexing starts
at 0. You and gpt-6 are in a better position than any human to rule on
cross-discipline reasonable defaults, so I would defer to your call in
cases like this for which position is most reasonable given shard's
aims." → Q9 ruled by Fable (Lean's totalizations copied as the
discipline's consensus; division by zero is not a fail family).

**2026-09-06 — proof search.** "The previous proof DSL was something we
made the search engine be able to navigate, and the auto files
reflected any solution body we were able to generate by one tool or
another … One thing I want to make sure we retain is the ability to
build better and better proof search engines over time … should we
make sure to make that IR something that our search engines can
navigate, with a template and hole system to match the E search
system?" → the proof IR (I).

**2026-09-06 — the naming law.** "I want to avoid a potential
splitbrain where the same mathematical operator is spelled different
ways between def and fn operations, and in different subsections of the
language. I think we should aim for building a more powerful shard
overall, not today's shard with a lean engine bolted on. Rewriting all
existing shard files is already in-scope." "LLMs are plenty capable of
authoring through a minor translation table, so the root question
becomes what *should* the spellings be … as the primary author of shard
source within the overall system we are building, what would be the
least confusing paradigms to adopt here? If the answer is that Lean is
already the ideal form, then I am fine with that, but I would rather it
be an explicit decision." The damascene precedent: "we made all the
core machinery *feel* similar to the implementing agent, while not
literally being an html engine. We implemented the common things under
similar names, refused to implement the parts of html that didn't
survive our contextual scrutiny, and built documentation to guide
agents throughout." → the naming law; `docs/LEAN.md`.

**2026-09-06 — v0.5.** "Agreed on both leans" (R18: the shared
mathematical types are Lean's exported `Init` declarations with E
realizations attached; R23: releases retain the full P closure in a
deduplicated store). "Yes we can archive the rest of the context in
records/."

## 3. Rejected alternatives (REJECTED-because, on record)

**A HOL-class kernel (2026-09-05).** Proposed by Fable first: simply
typed polymorphic λ-calculus, equality primitive, ten rules, three
axioms, Candle-style compute primitive; the shape shard already has
(shallow embedding, definitions as equations), the best verification
pedigree in the field, trivial metavariables, and no loss on shard's
present first-order mathematics. Reversed by Fable the same day after
the user's framing: the question is future capability, not present
fit; a HOL core cannot state natively the first dependent construction
an agent writes (`Vector α n`, a structure with proof fields, a
universe-polymorphic definition) — encodings exist but are precisely
the transport tax the bar rules out; and its pedigree argument inverts
once the Lean oracle is counted (`lean4export` plus independent
checkers give differential testing no foundation of our own can have).
Two claims made for HOL were weaker than stated: statements migrate
under CIC as well (`p x = true` is a proposition), and the soundness
risk of a from-scratch dependent kernel is bounded by that oracle. What
survived: K contains no search, no elaboration, no tactics; the
explicit-conversion policy; Rust executes E only.

**A Lean-informed foundation with departures (GPT-6 D01).** Rejected in
favor of rule-exactness: each departure forfeits the oracle and the
library for every declaration it touches; v1 declares zero departures;
future departures are dated decisions that name the forfeit.

**An interpreted-only `fn` status (GPT-6 R2, later withdrawn).** A
`fn` either lowers or is refused; lambdas are eliminated by elaboration
before any E program exists, so there is nothing to interpret.

**Rust as a maintained execution backend (GPT-6 B07/B08).** Rust
executes E and defines no independently evolving front-end; the initial
loader reads the narrow-compatible toolchain sources exactly as it
reads the kernel today.

**A separate repository for V2.** Cleaner for the archive question,
worse for shared CI, tooling and history; the sibling `v2/` tree with a
logical package root independent of the physical path (R24) was chosen.

**Universe cumulativity, dropping `String` literals, changes to `Quot`,
induction-recursion, induction-induction, coinduction, higher inductive
types.** Each priced at "loses differential testing for every
declaration it touches"; none has a consumer.

**Division by zero as a fail family (Q9).** The mathematical primitives
follow the cross-prover convention; a program that wants a trap uses
the named checked or preconditioned operation.

**"Statements migrate verbatim / mechanical rename" as a blanket claim
(v0.1–v0.4).** Replaced by typed migration classes (R19).

**Pinning only I plus a P hash (v0.3 Q2).** Replaced by durable P
closure plus I (R23, ruled 2026-09-06).

## 4. GPT-6's decisions and this proposal's positions

### 4.1 D01–D13, B06–B16 (its proposal, answered at v0.1–v0.2)

| ID | GPT-6 | position |
|---|---|---|
| D01 | Lean-informed dependent foundation, departures allowed | Lean-rule-exact K; departures dated and priced; zero in v1 |
| D02 | versioned rule package | accepted; declarative rules and the bounded procedure specified separately |
| D03 | fixed conversion, no equality reflection, budgets | accepted; explicit-conversion policy as elaboration discipline; transparency ≠ opacity ≠ conversion |
| D04 | E0 executable view, erasure as a theorem-bearing pass | replaced by E with a keyword; erasure a specified small transformation; correspondence by defining equations |
| D05 | intensional program identity | accepted |
| D06 | proof graphs from day one | accepted; I above them |
| D07 | Lean math coverage as target; transport optional | the kernel check is cheap under exactness; integration priced; Lean's `Init` is the core library identity (v0.5) |
| D08 | migrate meanings, rewrite proof text | accepted; typed migration classes and per-interface records |
| D09 | embeddable engine, small authority | accepted as constraints on K and the elaborator; T6 exercises the API |
| D10 | one identity system | accepted; six acceptance records; package root |
| D11 | native contextual holes | accepted as metavariables with transactional workspaces and hole kinds |
| D12 | reusable partial-construction proofs | accepted as quantified L theorems |
| D13 | runtime linking of engine/meta | accepted in principle, never implicit |
| B06 | uncertified Rust as acceptance authority | accepted |
| B07 | expand Rust execution capability | rejected: Rust executes E only |
| B08 | share resolved frontend artifacts | moot: one front-end, in shard |
| B09 | provisional execution without admission | accepted |
| B10–B12 | conformance, CI cadence, rollout | accepted in spirit; the ladder is the arc plan |
| B13 | public prepared invocation | accepted as T6 with the opaque prepared-context fix |
| B14 | hole semantics in shard | accepted |
| B15 | branch isolation / cache keys | isolation required from the first implementation; merge deferred |
| B16 | runtime linking bounded | accepted with D13 |

### 4.2 R1–R8 (feedback on v0.1, answered at v0.2)

| ID | disposition | notes |
|---|---|---|
| R1 L/E correspondence | accept | `K.whnf ≡ ev` withdrawn; defining equations are the bridge; erasure obligations table |
| R2 lambda profile | substance accepted; interpreted-only `fn` declined (withdrawn by GPT-6 at R9–R16) | static forms eliminated by elaboration |
| R3 contextual holes | accept | outcome types and the closure obligation, in meta |
| R4 module views | accept | view validity, implementation matching, evidence binding; three meanings of "do not unfold" |
| R5 embedding consumer | accept | T6 |
| R6 factual corrections | accept | lean4checker is not an independent kernel; thesis reconciled with Lean4Lean and the pin; export is not free integration; size not a gate |
| R7 ordering | accept | F1/F2 merged; tb_len rung pulled forward; compatibility layer optional, then replaced by the re-spelling tier |
| R8 migration meaning | accept | per-interface migration records with pinned resolved requirements |

### 4.3 R9–R16 (follow-up on v0.2, answered at v0.3)

| ID | disposition | notes |
|---|---|---|
| R9 evaluation evidence | amend | per-invocation evidence is `rfl` on the fuelled `ev` run; no native oracle; `cbv`/`decide_cbv` acknowledged |
| R10 declarative vs procedure | accept | separated; negative outcomes enumerated; Nat accelerators bound to fixed identities |
| R11 resolved requirements | accept | reconstruction vs selection; the frozen task |
| R12 acceptance records | accept | six records; policy at every boundary |
| R13 transactional workspaces | accept | attempt/commit |
| R14 totalization vs error policy | amend | Q9 ruled; checked wrappers named; exceptional behavior in the migration table |
| R15 evidence-backed replacement | accept | "no unaccounted replacement"; the dynamic-tier wake condition |
| R16 conversion experiments | defer, recorded | instrument first; conversion plans and expected-type checking as experiments; Lean4Less as research |

Corrections taken at v0.3: the unsourced "most historical Lean
unsoundness lives in the compiler bridge" deleted; stored proof terms
and proof-producing evaluation acknowledged; the module-system contrast
updated; `partial`/`unsafe`/`implemented_by`/`csimp` distinguished; E
ownership described as integration and a route, not a correctness
proof; oracle agreement as differential evidence, not a bound.

### 4.4 R17–R28 (integration review on v0.4, answered at v0.5)

| ID | disposition | where in the normative document | notes |
|---|---|---|---|
| R17 relevance roles, ghost invariants | accept | §4.1–4.2, §4.6 | subtypes over arbitrary Prop with erased proof; decidability only for runtime membership tests; decision tags kept, payloads erased; static law-bearing packages specialize ops and erase laws; dependent `if h : …`; "no Prop elimination into data" replaced by the relevance rule |
| R18 executable attachment | accept; RULED | §4.4, §10 | the shared mathematical types are Lean's exported `Init` declarations, imported once, with E realizations attached; `fn` = declaration + realization; `realize` attaches to an existing declaration; an imported name is never identified with a native one by spelling |
| R19 typed migration | accept | §10.2 | three classes; the `capacity - used` example; the equality-relation table; the `idiv` zero guard as a lowering premise |
| R20 provenance ≠ correspondence | accept | §4.4 | the realization relation is over the resolved executable structure, discharged by K-checked equation lemmas plus `ev`'s theorem; T1's fresh-but-wrong generator |
| R21 I replay contract | accept | §7.2–7.3 | explicit consequential choices or versioned bounded reconstruction; graph with scope-safe references; builders as data; I versioned separately |
| R22 hole kinds, refutation scope | accept | §6, §7.4 | template ≠ recipe; goal graph with dependencies; negatives carry subject and scope; dependent counting |
| R23 durable P | accept; RULED | §7.5 | releases retain the P closure in a deduplicated store; I for navigation and reconstruction; canonical P encoding; dependency classes |
| R24 identity across the flip | accept | §8.3 | logical package root; abstract slot ≠ body hash; nominal types not conflated |
| R25 World model | accept | §4.7 | BOUNDARIES.md's "monotonic ⇒ no reuse" is a false inference (verified: two writes consuming one `w` still advance the endpoint); well-threadedness is an explicit check with a trace relation |
| R26 boundaries | accept | §9.3 | checked vs preconditioned entries; handles bind a snapshot; buffer contracts |
| R27 budgets, reclamation | accept | §9.4 | reflective `ev` under compiled K is a measurement; fuel is not every resource; reclamation rules |
| R28 raw API validation | accept | §3.5 | raw vs checked environments as distinct types; T0 gains direct malformed construction |

Editorial items taken at v0.5: `Env` for the environment (E is the
executable fragment only); the type variable bound explicitly in the
examples; "no Prop elimination into data" replaced by the relevance
rule; "no indices" stated as "no index-dependent runtime layouts";
refusals scoped to source sugar, E eligibility, deployment profile or
assumption policy; "Rust never parses" replaced by the exact initial
loading profile; the stale "Lean's opaque Float" contrast removed
(FLOATS.md already cites Lean 4.33's kernel-reducible model);
"verbatim", "mechanical", "unchanged", "no search" used only where the
contract supports them; the normative/records split itself.

## 5. Findings and corrections made along the way

- The measured provocation: B-1b's per-function certificates (tb_len
  717, tb_perim 957, tb_app 1,148 canonical lines), the ghost twins
  forced by equation-only conclusions, and dozens of proof-DSL gotchas
  in the author's working notes for three theorems.
- The six 2026 kernel holes (2026-07-24/25, 2026-09-02) all lived in
  special rules, dispatch tables or text-derived gates; they become the
  hostile battery in T0.
- `K.whnf ≡ ev` (v0.1) was false: well-founded definitions do not
  reduce definitionally. Corrected to the defining-equation bridge.
- "Verified decide instead of trusted native_decide" (v0.2) overstated:
  Lean has proof-producing `cbv`/`decide_cbv`; and a native value is not
  a proof — the per-invocation evidence is `rfl` on the fuelled `ev` run.
- lean4checker mis-cited as an independent kernel (v0.1); it replays
  through Lean's kernel.
- BOUNDARIES.md's clock inference (R25) and v0.4's Float contrast
  (R19/§15) — both verified against the files and corrected.
- "Subtype predicates must be E functions" (v0.2–v0.4) was too strong;
  certified-program values need arbitrary-Prop refinements (R17).

## 6. The naming law's derivation (2026-09-06)

The question was put to the author of the shard corpus. The answer:
Lean's names, conventions and theorem-naming grammar are the least
confusing baseline because they give the author a complete mental map,
above all the naming grammar (`List.length_append`, `Nat.add_comm`,
`Int.emod_nonneg`, the `_of_`/`_iff`/`_left`/`_self` suffixes) that
lets a never-seen lemma be cited by guess — the guessable-names clause
of the LLM-first principle already solved. A second full vocabulary
would put a translation at every token; `docs/LEAN.md` must fit in
dozens of rows. Six adoptions (names and namespaces; lowercase
constructors and Bool literals; one propositional set of connectives
with `Decidable` bridging in E; Euclidean `/` and `%`; `Nat` sizes,
which delete the nonnegativity half of the measure regime; the naming
grammar and `f.eq_N`) and five departures (Rust-flavored declaration
keywords; the s-expression prefix surface; no effect notation;
refusals with pointers; shard-only vocabulary). The normative statement
is `docs/FOUNDATION.md` §5.3.

## 7. Related records

`docs/COVERAGE.md` and `docs/records/COVERAGE.md` (the coverage arc,
PARKED at B-1b 2026-09-05; resumes at phase 7 on V2); `docs/TCB.md`
(the roster this replaces); `docs/TOTALITY.md` (the measure regime,
carried as elaboration); `docs/CERT.md` (validators carried; §3/§7
superseded); `docs/SEARCH.md` (LS-laws carried; the lock-step law
becomes joint search under one metavariable context); `docs/MEMORY.md`
D8 (fail families); `docs/BOUNDARIES.md` (the World discipline, with
the R25 correction owed); `docs/FLOATS.md` (kept as ours).
