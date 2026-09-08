# records/FOUNDATION.md — the V3 foundation: history, rulings, and the review loop

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
| v0.2 | 2026-09-06 | `d6b25f1` | GPT-6 feedback R1–R8 folded; Q1–Q8 ruled; static lambda profile; Lean review; Foundation-arc plan with the sibling `v2/` tree (renamed `v3/` at v0.6) |
| v0.3 | 2026-09-06 | `524b59c` | GPT-6 follow-up R9–R16 answered; evaluation reflection with per-invocation evidence; declarative rules vs the bounded procedure; conventions ruled (Q9); the proof IR (I) |
| v0.4 | 2026-09-06 | `23094d7` | the naming law (one name per mathematical object; Lean's names by explicit decision; five departures; `docs/LEAN.md`); §12.1 as a pure migration table |
| v0.5 | 2026-09-06 | `985776b` | GPT-6 integration review R17–R28 folded; the normative/records split; the two rulings of 2026-09-06 (Lean's `Init` as the core library identity; durable P closure for releases) |
| v0.6 | 2026-09-06 | `150ceeb`, `f14bde8` (§10.5) | **renumbered V2 → V3** across the project (the tree is the lineage's v2; the sibling tree is `v3/`; the archive is `foundation-v3/`); GPT-6 final clarifications R29–R34 folded (role-aware elimination, selection discharges applicability, the `Init` identity stated at `realize`, canonical serialization as the storage boundary, World ownership through aggregates, older snapshot ≠ invalid handle); the first connected path |
| v0.7 | 2026-09-06 | `68c0e08` | Fable's last review folded (§10.5 documentation disposition landed at `f14bde8`): source spans structural; types get realizations (`Array`, `String`, `ByteArray`, `UInt*`; the validated-bytes string = `String`'s realization); numeric literals and the `Nat`/`Int` seam (one automatic coercion); `Quot` erasure row; deriving at Stage 1; CANON's law carried; two user rulings — no tool writes into a source file (engine I lives in sidecars + the pin store), no user-defined notation/macros in v1 |
| v0.8 | 2026-09-06 | `28809ec` | GPT-6 pre-ratification refinements R35–R41 folded (imported identity vs view vs representation, with the phase 0 shared-type inventory; conditional quotient erasure; deriving under a declared policy; occurrence-aware provenance; literal defaulting order and coercion placement; the syntax ban scoped to the grammar; migration as an explicit authoring action); the `Fin` numeral ruling (imported meaning kept, oversized source literals refused); GPT-6: "ratify the design and implement the first connected path" |
| RATIFIED | 2026-09-06 | this commit | user: "Ratify it and start on phase 0"; phase 0 opened — the pin (Lean v4.33.1 `819816b`; lean4export, lean4lean, nanoda_lib, lean4checker heads), the package root, the shared-type inventory and the port manifest drafted under `v3/`, the trusted bring-up translations named in `TCB.md`, the §10.5 phase 0 banner pass |

The GPT-6 documents (archived under `docs/archive/foundation-v3/` — named `foundation-v2/` at v0.5, renamed with the renumbering at v0.6; each memo was an untracked root file until folded in; the memos say "V2" because they predate the renumbering):
`SHARD_FOUNDATION_PROPOSAL_v0.3.md`, `SHARD_BOOTSTRAP_ADDENDUM_v0.3.md`,
`SHARD_PROPOSAL_CHANGES_v0.3.md` (its proposal, D01–D13, B06–B16);
`SHARD_FOUNDATION_FEEDBACK_v0.1.md` (R1–R8, on v0.1);
`SHARD_FOUNDATION_FOLLOWUP_MEMO_v0.1.md` (R9–R16, on v0.2);
`SHARD_FOUNDATION_INTEGRATION_REVIEW_v0.1.md` (R17–R28, on v0.4);
`SHARD_FOUNDATION_FINAL_CLARIFICATIONS_v0.1.md` (R29–R34, on v0.5);
`SHARD_FOUNDATION_PRERATIFICATION_TWEAKS_v0.1.md` (R35–R41, on v0.7).
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

**2026-09-06 (the number).** Surveying the documentation that ratification
touches surfaced that the repo already calls today's shard "v2"
(`archive/TRANSFER.md` is the v1→v2 handoff; `REVISIT.md` is titled "v2 →
v3 Revisit Ledger" and says "v2 is a prototype … planning v3";
`archive/M3-V1-VS-V2.md`), while the foundation called itself V2 with a
`v2/` tree. User: "My original thought was kernel v2, but calling this
v3 across the entire project is defensible -- let's do it." The
foundation is V3, the sibling tree is `v3/`, and the archive directory
was renamed; the user's earlier quotes above keep their "V2"/"v2"
wording as spoken.

**2026-09-06 (v0.7, two rulings).** On where engine-authored proofs
live: "agreed on no automatic injections into the source. The
implementing agent should be responsible for editing and maintaining
tactics or 'auto' delegation in the original source, and the
sidecars+pin store are where generated I content should be maintained."
On user-defined notation and macros: "agree with refusing in v1". Both
in §7.5 and §5.3 (departure 6).

**2026-09-06 (v0.8, `Fin` numerals).** GPT-6 recommended keeping Lean's
imported instance (an oversized numeral reduces modulo the bound) and
documenting it; my lean was one step stricter within the naming law's
refusal scope — the L instance keeps its meaning, the elaborator refuses
a source literal whose written magnitude reaches a statically known
bound, with the pointer to the explicit wrapping construction or
`Fin.mk`. User: "your stricter point sounds reasonble, let's fold all
this in." §5.2.

**2026-09-06 (ratification).** "Ratify it and start on phase 0." The
contract is law at v0.8; GPT-6's R1–R41 are all folded; the review loop
continues on code and measured behavior, not on the contract.

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

**A separate repository for V3.** Cleaner for the archive question,
worse for shared CI, tooling and history; the sibling `v3/` tree with a
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

### 4.5 R29–R34 (final clarifications on v0.5, answered at v0.6)

GPT-6's framing: "the design is ready for implementation. These
clarifications prevent a handful of ambiguous sentences from becoming
incompatible implementations." All six accepted as wording folded into
the existing sections and tests; no new architecture layer.

| ID | disposition | where in the normative document | notes |
|---|---|---|---|
| R29 role-aware elimination | accept | §4.1–4.2, T1 | K authorizes logical elimination; E realizes supported valid constructions and grants no elimination rule; `Decidable` case analysis is elimination of data; the residual `Sort`/`Pi`/choice restriction applies to runtime roles only; the dependent-`if` example repaired to return `Option` in both branches |
| R30 selection discharges applicability | accept | §4.4, T1, T8 | registration ≠ selection; a conditional realization does not discharge its own condition; approximations not selected for exact contracts; the selection record binds implementation, applicability evidence and policy |
| R31 `Init` identity at `realize` | accept | §4.4, §5.3, T5 | the ruling stated normatively beside `realize`; the §5.3 example now realizes the imported `List.length` and uses a native `sum_list` for the `fn`/`theorem` illustration (the old example redeclared an imported name, which §4.4 forbids) |
| R32 canonical serialization boundary | accept | §7.5, T8, T10 | storage representation, not a per-edit whole-graph operation; incremental digests permitted; cached admissions bound to validated contexts |
| R33 World ownership | accept | §4.7, phase 4 | ownership through aggregates, projections, helpers and exclusive branches; names do not make tokens; the box/unwrap-twice fixture; erased references consume nothing; the two model tests retained |
| R34 older snapshot ≠ invalid handle | accept | §8.4, §9.3, T6 | a handle keeps denoting its revision; a new revision retargets and revokes nothing; release, revocation and lifetime govern; T6's ambiguous clause replaced |

Also taken: GPT-6's "first connected path" as a paragraph under §12.4
(inside the phases, not a new one).

### 4.6 R35–R41 (pre-ratification refinements on v0.7, answered at v0.8)

GPT-6's framing: keep every v0.7 addition; make them obey the contracts
already chosen; "after these edits, the recommendation is to ratify the
design and implement the first connected path". All seven accepted.

| ID | disposition | where in the normative document | notes |
|---|---|---|---|
| R35 imported type ≠ view ≠ representation | accept | §4.4, §10.3, phase 0 | my "`String` is `List Char` in L" was a remembered model — the current Lean reference models `String` over `ByteArray` with a UTF-8 proof; the pin decides, the document no longer restates shapes; the shared-type inventory is a phase 0 deliverable |
| R36 conditional quotient erasure | accept | §4.6 | supported carrier and operations only; the respect obligation is discharged then erased, never removed; representative comparison is not quotient equality; the `% 2` fixture |
| R37 deriving under policy | accept | §5.1 | my "canonical ordering where one exists" contradicted §5.2 — an ordering is a convention; the policy is recorded in the generated declaration's dependencies; precise refusals |
| R38 occurrence-aware provenance | accept | §5.1, T0/T8/T9 | shared nodes, generated obligations, API-built declarations; best available origin, "unavailable" never invented; origins outside P identity |
| R39 defaulting order, coercion placement, `Fin` numerals | accept; `Fin` RULED (stricter than GPT-6's lean) | §5.2, T1/T9 | the `Int.ofNat (1 - 2)` hazard; coercion sites recorded; no retroactive re-resolution |
| R40 the syntax ban scoped | accept | §5.3 departure 6, T10 | grammar and elaboration hooks only; E libraries producing L/I through the APIs are the extension model |
| R41 source ownership vs migration | accept | §7.5, §12.3, T8 | migration emits patches; application is a distinct recorded command the agent runs; never a side effect of proof success or failure |

Also taken: §10.5's note that ratification is not passing a gate, that
a superseded banner names the tree the old text still describes, and
that a superseded issue keeps its capability attached to the
replacement gate.

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
- The v0.4–v0.5 dependent-`if` example returned an element in one branch
  and `none` in the other (R29); the v0.4–v0.5 surface example
  redeclared the imported `List.length` as a `fn`, which §4.4 itself
  forbids (R31). Both repaired at v0.6.
- The foundation was numbered V2 for five drafts while the tree was
  already the lineage's v2 (§2, 2026-09-06). Renumbered V3 at v0.6.
- v0.6's §4.2 E-type list omitted `String`, `UInt*`, `Array` and `Fin`
  while §10.3 used all four, and §10.4 kept "`Str` over validated
  bytes" beside §10.3's `String` — two string types under a one-name
  law. Resolved at v0.7 by "types get realizations too" (§4.4).
- Through v0.6 every realization in §4.4 was a function; the
  representation swap that the README calls the project's signature
  move had no library-level statement. §4.4 now states it for the
  library's own types, with the counted heap as the general case.
- FOUNDATION §3.2 (v0.1–v0.8) called theorems "opaque for unfolding";
  the pinned kernel unfolds them (`has_value`). Corrected at ratification
  + phase 0, 2026-09-06, from the sources.
- v0.7 restated Lean's `String` as `List Char` from memory (R35); the
  pinned export decides shapes and the contract now says so instead of
  restating any. v0.7 also filed derived orderings under reconstruction
  while §5.2 calls orderings consequential (R37).
- Through v0.6 the document was silent on numeric-literal typing and
  coercions, the port's largest practical seam (`Int` everywhere today;
  `Nat` sizes under the naming law), and on source spans (#8), which
  are an I-schema field and cannot be retrofitted after pins exist.

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

## 7. Phase 0 record (opened 2026-09-06)

- **The pin:** Lean v4.33.1 (`819816b2e0a3bf405af45ae5c7af2491d8f5bee6`,
  released 2026-08-21, the latest stable at pinning; the toolchain is
  installed locally under elan). Tool heads as of pinning:
  lean4export `411dce7db58a`, lean4lean `8223d223ed98`, nanoda_lib
  `05055695879d`, lean4checker `91a7f0e8e9df`. Recorded in
  `v3/README.md`; phase 1 validates each against the release.
- **The package root:** `v3/` — `v3/A/B.shard` is `A.B` (LAYOUT.md).
- **The shared-type inventory** (`v3/INVENTORY.md`), read from the
  pinned `Init/Prelude.lean`: `String` is `ofByteArray (toByteArray :
  ByteArray) (isValidUTF8 : …)` — R35 confirmed against the pin;
  `Array` is `mk (toList : List α)`; `ByteArray` is `mk (data : Array
  UInt8)`; `UInt8` is `ofBitVec (toBitVec : BitVec 8)`; `BitVec w` is
  `ofFin (toFin : Fin (2 ^ w))`; `Char` is `val : UInt32` plus a
  validity proof; `Float` is `ofModel (toModel : Float.Model)`.
- **The port manifest** (`v3/MANIFEST.md`): REGENERATE ≈700k lines,
  ARCHIVE (proposed) ≈200k, PORT ≈300k of ≈1.22M. The proposed ARCHIVE
  families await the user's ruling: the sha256 weld/shani/xchain
  articles and the sha256sum dispatch/shani x86 articles (Arc B's
  hand-authored fast path), `models/wasm`, `models/riscv`,
  `models/pio`, `tools/wasmgen`, `tools/x86gen`, `tools/impgen`'s
  frozen oracles, `examples/snake_game` (v1).
- **The trusted bring-up translations** are named in `docs/TCB.md`.
- **K's rule inventory** (2026-09-06, user ruling: shard declarations with
  the rules as comments, not prose): `v3/kernel/{prelude,name,level,expr,
  decl,env}.shard` plus `v3/kernel/README.md` (the reconciliation ledger,
  20 items beyond the thesis, and the procedure). Reconciled against the
  pinned `src/kernel` sources (fetched at `819816b`), Carneiro 2019 and
  Lean4Lean. Gate: the closure loads and runs under the Rust bootstrap
  (`eval direct` on a probe entrypoint importing `env.shard`; exit 0) —
  the instrument for the toolchain profile; `bin/check` is not (it
  resolves type names through path-derived `use` lines the package root
  does not share, LAYOUT.md).
- **The toolchain-source profile, fixed (phase 0):** today's `(type …)`
  forms with the stdlib names the Rust loader has built in (`Nil Cons True
  False Some None Z S`), indices as `Int` — exactly what `kernel/*.shard`
  is; `v3/kernel/prelude.shard` is the stdlib copy. The naming law governs
  S; the V3 reader carries this profile at the flip (FOUNDATION §9.2).
- **Two findings against the pin while writing the inventory:** (1)
  FOUNDATION §3.2 said theorems are "opaque for unfolding" — the pinned
  kernel's `constant_info::has_value()` is `is_theorem() || is_definition()`,
  so theorems ARE delta-reducible; §3.2 corrected. (2) at v4.33.1 a string
  literal expands to `String.ofList (List.cons Char (Char.ofNat c) …)`, not
  `String.mk …` (`inductive.cpp` l. 1368, 1394); the inventory says so.

## 8. Phase 1 record (opened 2026-09-06)

- **Slices landed 2026-09-06** (all under `v3/kernel`, all green under
  `v3/test.sh`): 1 level + term utilities; 2 the checker core
  (`tc.shard`); 3 inductive admission and recursor generation; 4
  `check(env, decl)` with quotients; 5 the lean4export import and the
  T0 driver.
- **First T0 evidence:** the first 20,000 lines of the v4.33.1 `Init`
  export (519 declarations, `Init.Prelude` into `Init.Core`) accepted
  with zero rejections and zero generated-constant mismatches, in 18 s
  on route 3. The 3,000-line prefix (116 declarations) is a committed
  fixture.
- **Tool bump:** `lean4export` head pins v4.34.0-rc2; the v4.33.0-bump
  commit `15f6055` built against v4.33.1 produces the export.
- **Pin behaviours a naive expectation gets wrong, now fixtures:**
  `whnf` continues from zeta into `reduce_nat` (`let x := zero; succ x`
  is the literal 1); `infer_app` returns the unreduced codomain;
  `normalize` does not re-sort after `mk_imax`; a `Type` field in a
  `Type` inductive fails the universe check before the result-shape
  check.
- **Slices 6–8 (2026-09-06):** nested inductives (`nested.shard`, with
  a `Lean.Syntax`-shaped two-level fixture); the fixed-identity pins of
  §3.2 (`hash_expr`; `accel_pins.shard` generated from the pinned export;
  a Nat/String literal is typed and an accelerator fires only under a
  pinned identity — the same-spelled non-core `Nat.add` case leaves the
  accelerator off, fixture in the hostile battery); the hostile battery
  (27 refusals, each for its declared reason); `check_with` carrying
  §3.3's limits. Cost work: the environment store is an IntMap by name
  hash (was an association list), a 4-ary trie on bit operations (the
  bootstrap's mod/ediv are bignum divisions), stream parsing, a changed
  flag from `whnf_core` instead of structural re-comparison.
- **T0 through `Lean.Syntax` (2026-09-06):** the export prefix to line
  78,503 — 1,375 declarations — accepted 1,375 / rejected 0 /
  mismatched 0; `Lean.Syntax` (two levels of nesting through `Array`
  and `List`) admitted, and its three exported recursors
  (`Lean.Syntax.rec`, `rec_1`, `rec_2`) equal to K's generated ones
  field for field and rule for rule.
- **T0 through export line 180,000 (2026-09-06):** chunks 0–8 (2,418
  declarations) accepted 2,417 / rejected 0 / mismatched 1, then a crash
  in chunk 9. Both findings fixed with fixtures: (1) `MonadReaderOf.rec`
  — `consume_type_annotations` lacked `semiOutParam`; the pin's list is
  `Expr.consumeTypeAnnotations` in `Lean/Expr.lean` (exported into the
  C++ kernel), optParam/autoParam at arity 2 and outParam/semiOutParam
  at arity 1, and the first Init class carrying `semiOutParam` on a
  parameter is `MonadReaderOf` (the outParam classes before it passed);
  (2) `Nat.pow` with exponent 0 — the size guard evaluated `(ediv max b)`
  under `bool_and`, which is a strict function in E, and the bootstrap's
  `ediv` by zero is a stuck primitive that surfaces as the entrypoint
  error "expected a World as the last argument". Rule for K's authors:
  a guard over a partial primitive is a nested `if`, never a `bool_and`
  operand.
- **Sharing measurement (2026-09-06):** per 20,000-line chunk of the
  export, the DAG size (distinct expression-table entries reachable
  from each declaration) against the expanded tree size. DAG size is
  flat at 20k–50k nodes per chunk; tree size is 74k (chunk 0), 454k
  (chunk 4), 19.5M (chunk 18), 27.3M (chunk 24, two declarations); the
  tree/DAG ratio runs 2.75 → 720, and the check time per chunk under the
  plain tree representation tracks the tree size (13 s, 118 s for
  chunks 0 and 4) with no correlation to DAG size. Conclusion: the
  representation must see the export's sharing; the tree cannot check
  Init. The representation ruling is pending (annotated nodes with
  identity, as the pin, versus a hash-consed index arena).
- **Representation RULED and built (2026-09-07):** user: "The annotated
  nodes with identity approach sounds like the correct one to pursue."
  Chosen because it is the pin's own mechanism (expr.h's cached hash,
  loose-bvar range and flags; pointer identity as the equality fast
  path; expr_map memo tables keyed structurally) and changes only
  construction sites, never a match; rejected-because for the
  hash-consed arena: the pin does not hash-cons, O(1) equality is a
  property it never had, and in E every match would become a trie
  lookup and every construction a persistent-trie insert. Every `Expr`
  constructor's first field is a `D` (id, hash, loose-bvar range,
  has_fvar, has_lparam); id 0 is anonymous, ids > 0 are minted from
  `St`'s node counter and are unique within an environment lineage —
  the `CheckedEnv` carries a node-id watermark and every check seeds its
  counter above it (a client restarting the state at zero would
  otherwise mint colliding ids into an environment that keeps them).
  Traversals prune by the cached range and flags exactly as the pin's
  instantiate/abstract/instantiate_lparams do. Then the pin's memo
  tables (type_checker.h `state`: infer per mode, whnf_core filled only
  when neither cheap flag is set, whnf for the non-easy kinds, unfold,
  the success and failure pair sets ordered by hash), keyed on the hash
  and verified by structural equality, reset at every environment
  extension because the pin builds a fresh checker per `tc()` call.
  Cost over chunks 0–4 of the export, identical verdicts throughout
  (519 / 908 / 1,153 / 1,400 / 1,570 accepted, 0 mismatched):

  | chunk | tree | ids + flags | + memo tables |
  |---|---|---|---|
  | 0 | 13 s | 8.2 s | 7.8 s |
  | 1 | 28 s | 22.5 s | 10.7 s |
  | 2 | 48 s | 36.9 s | 13.4 s |
  | 3 | — | 39.0 s | 14.0 s |
  | 4 | 118 s | 92.7 s | 18.6 s |

  The per-chunk time is now flat and tracks the DAG size (20k–50k nodes
  per chunk), as the sharing measurement predicted. Two findings on the
  way: a memo hit's "changed" verdict must be structural, not identity
  (a distinct but equal key made `whnf_core` loop to depth exhaustion —
  the nested-inductive fixture caught it); and a `fn` named like a
  constructor shadows it in the bootstrap. The port is audited by a
  constructor-arity checker over pattern and expression position (the
  bootstrap builds a short constructor silently and fails at the first
  match) before the suite: 9 entrypoints, the 3,000-line fixture with
  identical verdicts.
- **Recursion-depth accounting (2026-09-07):** the run over chunks
  0–24 (line 500,000) accepted 4,378 declarations, 0 mismatched, with
  one exhaustion — `ByteArray.utf8DecodeChar?_utf8EncodeChar_append`
  (chunk 21) ran out of recursion depth in `whnf_core`, and its
  dependent theorem was rejected as unknown_constant in consequence.
  Two departures from the pin's budget, both fixed: (1) K's `whnf`
  unfold loop, `lazy_delta_reduction` and `lazy_delta_proj_reduction`
  recursed with depth − 1 per iteration, where the pin's are `while
  (true)` loops with no `scope_rec_depth` of their own (the pin's three
  guard sites are infer_type_core, whnf_core and is_def_eq_core) — the
  loops now keep the depth and take the heartbeat budget as their
  measure; (2) the kernel's limit is `maxRecDepth` (512) times
  `g_kernel_rec_depth_factor` (16) — runtime/interrupt.cpp: "the kernel
  recurses substantially deeper than the elaborator did for the same
  term … we therefore let the kernel reach a generous multiple" — so
  `budget_depth` is 8,192, not 512. Verified: chunks 0–21 (line
  440,000) accepted 4,367 / rejected 0 / exhausted 0 / mismatched 0;
  the formerly exhausted declaration checks in about 20 s. Cost of the
  large chunks: 18 (the 19.5M-node tree) 143 s, 21 77 s — checkable,
  not yet cheap.
- **Where the large declarations spend their time (2026-09-07):** chunk
  18's 140 s is three private omega-style proofs of
  `Array.extract_append_extract` (66 / 52 / 27 s; trees of 12.5M / 5.5M
  / 1.3M nodes over DAGs of 24.5k / 11.3k / 5.7k). A differential
  dispatch profile (chunks 0–18 minus 0–17) shows the checker's
  procedures run DAG-many times (infer +127k, is_def_eq +81k, whnf_core
  +525k, instantiate calls +172k) — the memo tables hold — while the
  work inside them is large: instantiate averages 68 visits and 36 new
  nodes per call (11.8M visits), and structural equality runs 37M
  calls, about 75 nodes per memo lookup, because these terms have big
  open parts under up to 15 nested binders. The pin's per-traversal
  cache (replace_rec_fn, keyed on node and offset) was built and
  measured — it works (a shared open subterm is rebuilt once) but the
  outer instantiate it would help costs only 23k rebuilds by the DAG
  model; unconditionally it made chunks 0–4 67 → 96 s and chunk 18 140
  → 207 s through trie inserts (about 300 dispatches each at the
  interpreter's ~4.4M dispatches/s), and no threshold recovered it, so
  it is not in the tree. Conclusion: no sharing blowup remains; the
  large declarations cost the interpreter's constant factor on the
  pin's own algorithm (about 3.5 ms per DAG node against 0.3 ms on
  ordinary declarations). Remaining route-3 levers are constant
  factors: name equality (20M calls in the chunk) and level-list
  equality (10M) before a hash short-circuit, the linear `nth` on the
  substitution (12.8M), and the JSON reader as the flat floor.
- **The interpreter's constant, measured and cut (2026-09-07):** a
  fresh look at the same profile from the host side. `perf` on a replay
  of chunks 0–4 put 49% of samples in libc malloc/free: the bootstrap
  evaluator's environment was a persistent cons list allocating one Rc
  node per bound value — per argument, per pattern capture (wildcards
  included), per let binding, about five per dispatch — and the bitwise
  and `mod` primitives took the general Val→Expr→Val path (a Vec and
  several BigInt clones per call, ~240M calls over chunks 0–18). Landed
  in `rust_bootstrap` (8b1030e): bindings on one value stack with a
  separate operand stack for in-flight values (pushing operands onto
  the binding stack read outer bindings at the wrong offset when a
  match sat inside an argument — v3/test.sh caught it), the six
  primitives on the tagged fast path with the table's guards repeated
  (a new test sweeps every shared two-Int primitive through `eval::eval`
  against the table), mimalloc as the allocator. In K (430b16a): the
  IntMap trie ends in leaves, so a walk is ≈ log₄(table size) levels
  instead of the full 24-bit key width (the trie was a quarter of all
  dispatches, mostly in the per-declaration memo tables of a few hundred
  entries). Chunks 0–4: 67.7 s → 51.1 (stack) → 46.9 (prims) → 32.5
  (operand stack) → 30.4 (mimalloc) → 25.8 (leaves); chunks 0–21: 724 s
  → 280 s; verdicts identical at every step (4,367 / 0 / 0 / 0 through
  chunk 21; chunks 0–24 accepted 4,380 / 0 / 0 / 0, 416 s).
  What remains is the evaluator's own dispatch: 52% of samples in
  `eval_ir` itself, 11% pattern matching, 6% moving arguments from the
  operand stack to the frame, 9% Rc clone/drop, libc 1%. Levers left
  there, each a few percent: unboxed small ints (every int is an
  `Rc<BigInt>`; hash values allocate), a flat-pattern fast path in
  `match_pat`, pointer compares for the If's True/False. The v2 compiler
  chain does not take K's sources (tools/lower on `t0.shard` falls
  through a run-mode match on `Pair` at once), so the compiled route
  stays V3's own. Per-file dispatch share on chunks 0–4 after the cut:
  primitives 49.5%, expr.shard 19.9%, util.shard 14.7% (bool_or,
  int_list_eq, rev_onto — largely the reader's), json.shard 6.4%,
  tc.shard 4.3%, intmap.shard 3.7%.
- **The evaluator, round two, and the compiled route (2026-09-07,
  late):** user: "Let's continue digging for performance improvements —
  especially in the rust evaluator engine. We need this to be fast
  enough to get through the v3 arc without needing the compiled system.
  That or we update the c-based compile chain to support the v3
  kernel." Six more evaluator steps, each landed on identical verdicts
  (chunks 0–4, from 25.8 s): leaves answered without entering the
  machine, prebuilt zero-argument constructors, boxed errors (ec9ce61,
  23.1 s); unboxed i64 integers with `Val::Big` only beyond a word
  (7a5fce8, 20.0 s); flat constructor patterns bound by one slice copy
  (0918a41, 19.5 s); one value stack with the lowerer computing each
  variable's physical slot past the temporaries in flight — no operand
  stack, nothing moved (2e89439, 18.1 s); a SIXTEEN-BYTE value —
  constructor blocks behind one thin pointer with the fields inline,
  interned name ids — so every result returns in registers (0622881,
  12.7 s); the reader's symbol ↔ bytes bridge on the fast path (fa21ff9,
  12.6 s); `if` fused with a comparison condition (916e81f, 12.0 s).
  Measured and NOT kept: moving a binding's only use out of its slot
  instead of cloning fires at 42% of read sites and changes nothing —
  refcount traffic on reads is not the cost. Totals for the day: chunks
  0–4 67.7 s → 12.0 s; chunks 0–24 830 s → 184 s (4,380 / 0 / 0 / 0);
  chunks 0–49 (line 1,000,000) 8,511 / 0 / 0 / 0 in 340 s at a 1.2 GB
  peak (2.6 GB at chunk 20 this morning). What remains in the
  interpreter is the tree walk itself (the machine loop and pattern
  matching, then constructor allocation and freeing); the next multiple
  there would be a bytecode or closure-compiled machine, a rewrite for
  perhaps 1.5×.
  **The compiled route works today.** A read-only investigation of this
  morning's chain failure found the cause in `kernel/resolve.shard`:
  `is_core_path` recognises `core` only as the exact path
  `kernel/stdlib`, so a tool named by an absolute path (as I had run it,
  from the scratch directory) imports a non-core prelude and its `Pair`
  patterns never match the engine's values (#41). Named relatively from
  the repo root, `tools/lower` + `tools/codegen` + cc build
  `v3/kernel/t0.shard` into a native binary in 17 s, the lower output
  byte-ties against the Rust interpreter, and the binary's verbose
  per-declaration output on chunks 0–4 is byte-identical to `eval
  direct`'s: chunks 0–4 in 2.2 s, chunks 0–24 in 40 s (4,380 / 0 / 0 /
  0). **The full export (325 chunks, 6,490,422 lines): 57,977
  declarations accepted, 0 rejected, 0 exhausted, 0 mismatched, 0
  unsupported, in 12 min 17 s** on the compiled K — a run the
  investigating agent started on its own; I let it finish under a memory
  cap. Peak resident set 30.2 GB (measured on the next run, by
  `v3/t0_full.sh`'s watchdog; the export tables hold every referenced
  node). §9.1
  ranks this engine first — route 1, K compiled by shard's own lowering
  — with its proof still to come; the Rust interpreter (route 3) stays
  the authority, and at today's ratio (about 5×) would replay the whole
  export in roughly an hour. The route question is the user's: compiled
  K for the export's verdicts with the interpreter confirming prefixes
  and the lower byte-tie, or the interpreter alone.
- **Compiled K RULED (2026-09-07, late):** user: "Compiled K sounds like
  a fine approach for now. Our CI runners on gitlab have much higher ram
  budgets since they run on local hardware. We can also kick off a new
  shard_eval build." Chose-because: the whole export in minutes, byte-tied
  to the authority on prefixes; rejected-because (for the interpreter
  alone): about an hour per full replay with nothing left in the tree
  walk short of a bytecode rewrite. Landed: `v3/build.sh` (the chain into
  `v3/bin/t0`, boot engine `bin/shard_eval` or the Rust interpreter),
  `v3/export.sh` (elan + Lean v4.33.1 + lean4export@15f6055 into
  `.shard-cache/v3-export/`, reused by a pin marker), `v3/t0_full.sh`
  (fixture and chunks 0–4 verbose byte-tie, then the full replay under an
  RSS watchdog against `v3/t0_expected.txt` = the 57,977 line; locally
  745 s, 30.2 GB peak; the first CI pass, pipeline 448 on the high-cpu
  runner: 2,564 s, 30.4 GB peak, the export built there in 74 s), and the
  `v3` CI job (gate stage, high-cpu, needs the engine artifacts, the
  export built fresh each run: a runner cache of the 3.5 GB export tree
  was tried on the first pass and its upload ran past 45 minutes against
  a 77 s rebuild, so it was dropped). `bin/shard_eval` rebuilt on the dev
  box (stamp a3c9a930…, 16 s fast-boot; its lower and codegen outputs on
  K byte-identical to the Aug 2 engine's). The interpreter is still the
  authority; the compiled artifact becomes route 1 proper when the
  lowering is proven.
- **Phase 1 against the law (2026-09-07, late):** user: "Let's dig into
  the remaining phase 1 work" → the remainder re-derived from §12.4
  phase 1, §3.5, §3.6, §9.4 and §10.5 rather than from the running list;
  then "agreed, let's do 1 through 4 now". (1) **Pins:** five candidates
  had no row, not six — `Char.ofNat` and `String.ofList` (export line
  77k, chunk 3), `Nat.lor` and `Nat.shiftLeft` (183k–186k, chunk 9),
  `Nat.xor` (line 702,093, chunk 35); `gen_pins.sh` regenerated the
  table on the interpreter over chunks 0–35 (6,181 declarations, 4.5
  min): 20 rows, the fifteen old hashes unchanged. The string-literal
  expansion had been enabled on the `String` pin alone while naming
  `String.ofList`, `Char.ofNat`, `List` and `Char` — §3.2's letter says
  the expansion is enabled only under the identities it names, so
  `string_lit_expansion` now gates all three expansion sites on the
  five pins (Unsupported `string_literal_unpinned_expansion`
  otherwise); the battery gained "8b a string literal under a
  non-pinned String is Unsupported". (2) **Axiom closures — the gap:**
  §3.6's gate is "identical verdicts *and axiom closures*"; T0 compared
  verdicts only — K tracked axioms at admission but computed no closure,
  and no oracle existed. Landed: `v3/kernel/axioms.shard` — the relation
  of `src/Lean/Util/CollectAxioms.lean` at the pin (an axiom contributes
  itself and its type's constants; a definition, theorem or opaque its
  type's and value's; a quotient nothing; a constructor and a recursor
  their type's; an inductive its type's and its constructors), computed
  as the least fixpoint: an inductive block — the only cycle a checked
  environment holds — is resolved as a unit by in-block reachability,
  everything else memoized (the memo outside the CheckedEnv, §9.4),
  expressions walked once per DAG node by id; the driver's `-a` prints
  one `AX name: …` line per admitted constant (per quotient record, the
  constant it names). The oracle `v3/axioms.lean` computes the same
  relation over Lean's own environment and `getUsedConstants` as a
  Kleene fixpoint in module order — NOT by calling `collectAxioms`:
  that function's memo plants a sentinel for the constant in progress,
  so an inductive's cached closure depends on whether it or one of its
  constructors was visited first (order-dependent; §3.6 compares by
  rule, not by tool name). 65,994 constants, 11 passes, 15 s; built by
  `export.sh` into `init.axioms`. `v3/t0_axioms_cmp.sh` normalizes both
  sides (axioms sorted per line), joins by name and requires every K
  constant in the oracle with the same closure. Evidence: the fixture
  181/181 identical (now `t0_fixture_test.sh`'s second assertion; the
  oracle slice committed as `fixtures/init_prefix_3000.axioms`), chunks
  0–4 1,813/1,813 identical at 12.6 s against 12.0 s without; the
  interpreter and the compiled K byte-tie on `-v -a`; `t0_full.sh`
  byte-ties with closures, replays with `-a` and gates on the
  comparison as well as the verdict line — the whole export's
  comparison runs in the CI job (its first pass pending at this
  writing). `Init` declares seven axioms: `Classical.choice`,
  `propext`, `Quot.sound`, `Lean.trustCompiler`, `Lean.ofReduceNat`,
  `sorryAx`, `Lean.ofReduceBool`; 40,861 of the 65,994 constants have a
  non-empty closure. (3) **Fuel monotonicity (§9.4)** stated in the
  kernel README and tested: the battery's 12b/12c exhaust `c_deep` at
  depth 2 and at one heartbeat, then admit the same identity at 64 /
  1,000 and at the default. (4) **The doc rows due at phase 1 (§10.5):**
  TCB.md carries the V3 roster (K; the Rust host as authority, the
  compiled K a differential artifact; the oracle evidence; retained P —
  none yet; the fixed-identity primitives) and the four-routes lifetime
  note; `tools/lower/DESIGN.md` the route-1 banner; `.gitlab-ci.yml` and
  `run_corpus.sh` say which tree each gate covers. Also: the `v3` job's
  export cache dropped — its 3.5 GB, 18,569-file upload ran past 45
  minutes against a 77 s rebuild. Deferred and recorded: §3.5's
  never-forgeable checked environment — `CheckedEnv`'s constructor is
  callable by any client until V3 has a module surface (phase 2); the
  hostile battery itself forges an empty one.
- **The interpreter's own full replay (2026-09-07, late):** user: "go
  ahead on the full replay run". Route 3 on the dev box, detached under a
  40 GB cgroup cap, with `-a`: 57,977 accepted / 0 rejected / 0
  exhausted / 0 mismatched / 0 unsupported — the pinned line — in 4,591 s
  (76.5 min; user 4,537 s), peak resident set **8.0 GB** against the
  compiled K's 30.2 GB for the same replay (the bootstrap's sixteen-byte
  values and inline constructor blocks against the C runtime's boxed
  cells), and every one of its 59,433 admitted constants' closures
  identical to the oracle's. Its log is byte-identical to the compiled
  K's log from the runner (pipeline 452; 60,084 lines — every verdict
  line and every closure — once the chunk-path headers are normalized):
  the full-export byte-tie between the authority and the compiled
  artifact, where the gate scripts tie a 100,000-line prefix. One
  declaration took a third of the run:
  `WellFounded.partialExtrinsicFix₃_eq_partialExtrinsicFix` (chunk 255,
  ≈25 min interpreted); the rest of the export ran at eight chunks a
  minute. Axiom use across the export's constants (K's lines): `propext`
  38,131, `Quot.sound` 24,737, `Classical.choice` 11,655,
  `Lean.trustCompiler` 5, `sorryAx` / `Lean.ofReduceNat` /
  `Lean.ofReduceBool` 1 each; 38,455 non-empty closures. Two defects in
  the comparison script surfaced on the way, both in the instrument, not
  in K or the oracle: (1) the CI image's awk is mawk, which has no
  `asort`, so the first closure-gate pipeline (451) failed with zero
  constants on both sides — the sort is now an insertion sort in POSIX
  awk; (2) the join used `:` as its separator and 23 of `Init`'s
  constants are syntax-category names containing colons
  (`Array.term__[:_]::_]`, `List.term_<+:_::_`, the simproc-declaration
  commands), reported as differing — the separator is a TAB, `-u` on the
  key is gone (a duplicate name with two closures now shows as a
  difference), and the script was made to fail on a tampered closure
  and on a constant the oracle lacks before being trusted again. The
  oracle listed 91 auxiliary constants (`.eq_1`, `.congr_simp`) under
  two modules; it now lists each once (65,994).
- **The closure gate's first full pass on the runner (2026-09-08,
  pipeline 453 on 08cb592):** the export and the oracle built in 49 s
  and 49 s, both byte-ties identical with closures, the replay exit 0
  in 2,478 s at a 29.7 GB peak, the pinned verdict line, and all 59,433
  closures identical to the oracle's — "T0 FULL == PINNED, closures
  identical". T0 as §3.6 states it — verdicts and axiom closures over
  the whole of `Init`, the hostile battery, the scope logged — now holds
  on both routes 1 and 3, with the two routes' full logs byte-identical.
- **Open in phase 1:** §3.5's forgeability, deferred to the module
  surface (above).

## 9. Phase 2 record (opened 2026-09-07)

- **Opened** after T0 closed on both routes (§8). The phase per
  §12.4 item 2: the loader and reader to explicit L, views, the
  fragment classifier with relevance roles, `ev`, `examples/calc`;
  gates T1 (a decision tag with erased payload, a branch-local proof,
  raw versus checked arguments), T5, T8's replay half, conformance
  (B10's four suites, §4.1 above).
- **The dig (2026-09-07), what shaped the plan:** (1) a `fn` with a
  `match` body has no L meaning at Stage 0 — match compilation and
  structural recursion to recursors are Stage 1, phase 3. (2) In the
  export `List.length`, `List.get`, `List.map`, `Nat.add` and `Nat.sub`
  are `brecOn` applied to a generated `_f` functional, so no executable
  view is readable off the kernel term; Lean's `eq_N` lemmas are in the
  export only where `Init` realized them, and late (`List.length.eq_2`
  at line 1,370,217, `List.length.eq_1` at 5,465,298, `Nat.add.eq_1`
  absent) — the shared core's realizations are supplied bodies with
  `rfl` bridges. (3) The base environment must be a declared prefix of
  the export: §3.5 and §7.5 forbid receipts, the whole export costs
  12 min compiled and 76 min interpreted per load, and chunk 0 (519
  declarations, 8 s) holds `ite` (line 2,727), `dite` (2,780),
  `Nat.decLt` (5,492), `List.length` (10,558), `List.get` (10,970),
  `Decidable.decide` (13,280) and `WellFounded.fix` (14,624). (4) calc
  is 51 fns and 9 types in about 1,000 lines plus 100 claims in about
  2,100; the claims need I.
- **Rulings (user, 2026-09-07): "Agreed on A and the program half."**
  (A) Stage 0 strictly: a `fn` is E only at phase 2, its L value
  pending until phase 3, when `fn` = `def` + `realize`; the explicit-L
  forms are the route into K; T1's items are explicit-L fixtures.
  Rejected: pulling the first-order match/recursion translation into
  phase 2 — the elaborator's hardest piece built without the gates
  that test it. (B) calc's program half under `ev`, differential
  against the old tree; its claims wait for phase 3 (`v3/MANIFEST.md`
  amended).
- **Slice 1 (2026-09-07): the design on disk.** `v3/LANGUAGE.md` — S,
  L and E at Stage 0: lexical syntax with the `.{u v}` universe suffix;
  modules and identity (native K names carry the module path, imported
  names do not — the import is their identity; the `Init` import is a
  dependency-ordered prefix named by its last declaration, its identity
  the pin plus that name; the content hash is over L, never S); the
  keyword table (`axiom` and `opaque` added to the law's list; reserved
  forms refused with the phase named); explicit-L terms and levels; the
  E term language (today's, parallel `let`); `ev`'s contract per node
  (pure; the tag rule for `if`; one unit of fuel per call; stuck never
  a value); the classifier's five Stage-0 checks; the primitive table
  keyed on identity, the profile's and the naming law's spellings as
  distinct entries where the meaning differs; views with the three
  conditions (view parameters are axiom-kind constants to K and a
  distinct policy class; `CheckedEnv` becomes a `sig type`, closing
  §3.5); `realize` in two forms — the derived view (erasure of a
  non-recursive L body) and the supplied body with per-arm
  `NAME.realize_N` equations proven by `rfl` at Stage 0 — plus the
  reserved type-representation form; the toolchain profile as a table
  of reader rules; policy (the standard three axioms; `trusts`; view
  parameters) and entries (checked versus preconditioned); the four
  conformance suites; the deferred table by phase; nine ratification
  items. `v3/kernel/prog.shard` — E programs as data with the rules as
  comments (the phase-0 precedent): `EType ELit EPat ETerm EArm ERec
  ECtorDef EDecl Prog Val EvRes`, `prog_find`; loads and runs under the
  bootstrap (`test/prog_test.shard`). The ladder: 2 the s-expression and
  Stage-0 readers into K (the hostile battery in S; T8's direct P
  verification; identity invariant under origin-only change); 3 the
  loader; 4 views; 5 the classifier, `ev` and route 2's byte-tie; 6
  calc's program half and frontend parity; 7 the doc rows, records, CI.
- **The v2 compatibility ledger (2026-09-07, user: "keep an eye on
  where the compatibility breaks are and what features we may be
  unintentionally losing"):** `v3/LANGUAGE.md` §12 lists every v2
  feature with its fate — carried, re-spelled, changed, deferred (with
  the phase), dropped (with the replacement), or AT RISK (nothing in
  the law or the draft provides it yet). Eleven AT RISK rows to watch:
  symbols in S (the toolchain's tags and identifiers); the `(list …)`
  literal (9,455 sites); the extern wire's byte convention under the
  naming law; negative numerals until Stage 3's `Neg`; `gen_fresh`
  (an effectful primitive, ten kernel files, no place in §4.7);
  record updaters `with_F` and order-free `make` (237 sites, no
  Stage-0 form); `std/word` widths beyond 64; `S^`/`inline`/`chain`
  if an I form still needs literal towers; `(lib …)`; the
  subterm-order strong induction (`subterm-induct`/`(below)`) and
  `fin-split` as I steps; no totality check on any `fn` during Stage
  0. Two draft fixes fell out of writing it: selective `(use PREFIX
  NAME…)` and the req-scope gate, both now in the draft.

## 10. Related records

`docs/COVERAGE.md` and `docs/records/COVERAGE.md` (the coverage arc,
PARKED at B-1b 2026-09-05; resumes at phase 7 on V3); `docs/TCB.md`
(the roster this replaces); `docs/TOTALITY.md` (the measure regime,
carried as elaboration); `docs/CERT.md` (validators carried; §3/§7
superseded); `docs/SEARCH.md` (LS-laws carried; the lock-step law
becomes joint search under one metavariable context); `docs/MEMORY.md`
D8 (fail families); `docs/BOUNDARIES.md` (the World discipline, with
the R25 correction owed); `docs/FLOATS.md` (kept as ours).
