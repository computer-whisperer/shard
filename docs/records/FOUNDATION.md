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
The phase-2 and phase-3 memos are listed at their sections (§4.7–4.11);
the last, `SHARD_V3_TRAJECTORY_REVIEW_2026-10-06.md` (R72–R77, on
`7e99261`), is a trajectory review rather than a memo on one slice.
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

### 4.7 R42–R48 (implementation review after the first `Init` replay, on `d6f1fd6`, answered 2026-09-12)

The memo: `docs/archive/foundation-v3/SHARD_V3_IMPLEMENTATION_REVIEW_v0.1.md`
("harden admission; keep phase 2 moving"; the K/L/E architecture not
reopened). Every source observation was verified against the tree
before answering; R42 was **executed through K** before the fix, which
the memo had not done. All seven accepted; R44 required a ruling.

| ID | disposition | where | notes |
|---|---|---|---|
| R42 hash-only accelerator authorization | accept — **FIXED 2026-09-12; CLOSED on CI pipeline 461** (full export: all 20 pinned structurally, closures identical) | `add.shard` `pin_if_matches`/`ref_closure_ok`; `accel_pins.shard` = reference closures, GENERATED by `gen_pins.sh` through `refgen.shard`; hostile battery 7b–7d; `TCB.md` item 5 | Reproduced first: the candidate `Nat.add a b := 1781693077243641776` (its literal one subtraction from the committed pin — the 61-bit hash is affine in a trailing literal, so every pin had a one-line preimage) was admitted, **pinned**, and `Nat.add 2 3 = 5` by `rfl` was **accepted** beside `Nat.add = fun a b => 1781693077243641776` by `rfl`: a contradiction in that environment. FOUNDATION §3.2 already required "signature and defining equations K validates … never a name-based match", so the fix is conformance, not amendment. The fix: the admitted declaration's **identity closure** — itself and, transitively, every definition, opaque, axiom and inductive type its type and value cite — compared structurally (kind, level parameters, type, value; an inductive's parameter count, block, constructors in order) against reference declarations generated from the export: 206 rows over the 20 candidates (theorems excluded — proof irrelevance: a proof can only make a reduction stick, never change a value; constructors and recursors through their inductive; quotients K-fixed); measured in the pinned Lean at 509 constants with theorems, 3,643 shared nodes, 6,889 as trees. Hostile case 7 (`Nat.add a b := a`) had passed by hash mismatch, never by authorization. Found while generating the table: the affine hash **collides on ordinary `Init` terms**, not only under construction — `List.cons`'s type and `List α → α → List α` (the inner two domains swapped; `App` adds the same constant to each child's hash, so the swap cancels exactly) share one hash, so the generator numbers term nodes by a counter and finds them by structural equality. The memo tables were already hash-bucketed with structural confirmation; the pin table was K's only hash-final decision. The structural hash stays as a fingerprint (reports, the fuel-monotonicity case), documented as never an authority (`LANGUAGE.md` §3). Alternative considered: a collision-resistant hash over the same closure — deferred to phase 3's content store, chosen with it. |
| R43 seal annotated terms and sessions, not only `CheckedEnv` | accept — **ingestion landed 2026-09-12 (slice 3)**: K's raw entry `check` rebuilds every node it is handed (`expr.shard` `ingest_expr`; hostile battery 16, eight cases); the constructor's sealing waits for `kernel/env`'s view (slice 4) and the toolchain loading under it | `LANGUAGE.md` §3.5's sealing by the module surface; an ingestion step that reconstructs cached metadata for raw submissions; the memo's regression list (conflicting positive ids, false flags, stale hashes, foreign nodes, a fabricated record; snapshot forks; failed checks not polluting later ones) | confirmed: `expr_eq`'s positive-id shortcut and the bare constructor — the hostile battery itself builds `(CheckedEnv IMEmpty False Nil 0)`. Interim limitation recorded: raw-API callers are reviewed toolchain code. |
| R44 one `let` semantics for L and E | accept — **RULED sequential 2026-09-12** (user); landed 2026-09-12 | `LANGUAGE.md` §5.4, §6.2, §13 item 5, §12's row; `prog.shard` `ELet`; `ev` (slice 5 implements it) | measured before ruling: 30,611 `let` groups in the tree, 162 with two or more bindings, **0** whose meaning changes under sequential binding (the scanner's one hit, `add.shard`, is a quoted symbol). Sequential everywhere costs nothing and needs no migration; the bootstrap's parallel evaluator gives identical results on every existing source, so the toolchain profile changes meaning without changing a result. |
| R45 runnable Stage-0 programs distinct from admitted definitions and checked realizations | accept — **all three tests landed**: `RUNNABLE fn NAME` per declaration, distinct from `ACCEPT` (slice 5); `ev_test` (the self-recursive candidate exhausts at any fuel, never a value) and `v3/pins/loader/ev_no_l_meaning` (a theorem citing a `fn` finds no constant); the third at slice 5b (2026-09-13): the same body as a `fn` is `RUNNABLE` with no L meaning, as a `realize` it is `REALIZE` with an equation a theorem cites and a body `ev` runs under the L identity (`loader_test`; `realize_native`, `realize_supplied`) | `LANGUAGE.md` §0, §6.2, §6.7, §7; a status kind in the driver's per-declaration output; the three named tests (a self-recursive candidate exhausts and gains no equations; a value discharges no proof requirement; a checked realization distinguishable from a linked function) | ruling A restated as API discipline; nothing new to decide |
| R46 route trust labels; memory and outlier costs | accept — ongoing | `t0_full.sh` already asserts exit codes and the pinned verdict line; retained-memory by category and the 25-minute `WellFounded.partialExtrinsicFix₃_eq_partialExtrinsicFix` outlier when performance next matters (phase 4's T6) | no new certification prerequisite |
| R47 prefix imports and compatibility omissions as bounded bring-up mechanisms | accept — wording; landed 2026-09-12 (`LANGUAGE.md` §3, §3.1, §12.6) | `LANGUAGE.md` §3: an imported declaration's identity is the pin plus its name plus its content; the prefix is only the scope the load recorded, so enlarging it changes no earlier declaration; §12's AT RISK rows gain owner, consumer and regression | the draft had conflated the load with the declaration |
| R48 ratified rules vs demonstrated coverage vs open obligations | accept — status text; landed 2026-09-12 (the banner; `v3/README.md` "Open obligations") | FOUNDATION's banner → a pointer to `v3/README.md`; `v3/README.md` lists the open obligations beside the phase-1 result (R42 until green on CI, R43 until enforced, the Stage-0 `fn` gap) | the banner still said phase 0 open and nothing implemented |

### 4.8 R49–R54 (checkpoint memo on the phase-2 close-out, on `b45cc22`, answered 2026-09-13)

The memo: `docs/archive/foundation-v3/SHARD_V3_PHASE2_CHECKPOINT_MEMO_v0.1.md`
("continue the implementation plan; do not reopen the foundation";
bounded requests over admission, canonicalization and execution
boundaries, plus positions on `v3/CANON.md` §9's five open items).
Every source observation was verified against the tree before
answering; R49 was **probed through K** before the fix (the memo had
traced it in source only). Five accepted, one amended (R53); landed as
slice 9 (`v3/README.md`). The user's go-ahead on the leans:
"Your leans look reasonable, go ahead with the slice."

| ID | disposition | where | notes |
|---|---|---|---|
| R49 accelerator authorization must validate the kind before exemptions | accept — **FIXED 2026-09-13 (slice 9); CLOSED on CI pipeline 477** (`4699cbe`, 2026-09-14: the full replay 2,418 s / 30.6 GB, all 20 pinned, closures identical — criterion E) | `add.shard` `ref_matches` (exempt kinds pass only under a name without a reference row: `ref_exempt`), `pin_if_matches` (a root must have a row: `ref_rooted`); hostile battery 7e (a theorem named `Nat.add` admitted and not pinned; a theorem head never applicable — `function_expected`; the matcher refuses a constructor, a recursor, a quotient and a theorem under `Nat.add`'s row; an exempt kind under a name without a row passes); `LANGUAGE.md` §13 item 31 | the probe before the fix: the theorem was PINNED; every use of it — a theorem `Nat.add 2 3 = 5`, one `= 6`, a definition `Nat.add 2 3 : Nat` — refused `function_expected`, since K infers a head's type before it reduces an application and a theorem's type is not a Pi. An authorization hole, not a false theorem, on this trace; recorded as the memo asked, not as a completed exploit. The candidate name stays admissible for other kinds (the memo's "do not ban the name") |
| R50 canonicalization must preserve required evaluation | accept — contract text, landed 2026-09-13 | `v3/CANON.md` §1 "The execution profile" (the observation relation: value and World trace under adequate fuel; fuel, time and allocation unobserved; the discarding rules C2, C3, C10–C12 hold where E is total and World-threaded — Stage 1 and phase 4, both before the phase-6 gate; the advisory recognizer reports "not applicable under Stage 0"; search's representative replacement under the same scope), §4 C1's bounded outcome, the third note on scope, §8 stage 1's six fixtures, §9 item 12 (RULED) | the rules refuse source spellings, never rewrite a program, and the rewriter is untrusted — but the memo is right that refusing a dead binding or an equal-branch `if` asks the author to delete a computation, which changes behavior under strict evaluation unless that computation is total and effect-free; the linear World makes a dead binding effect-free once phase 4's check exists |
| R51 primitive work needs budgets and distinct resource outcomes | accept — **landed 2026-09-13 (slice 9)** | `ev.shard`: `PrimRes` (`PrimVal`, `PrimGuard`, `PrimOut nat_size\|nat_count`), `HExhausted`, `EvExhausted`, `RunExhausted`; `Nat.shiftLeft` with the zero shortcut first, the count cap, the size preflight, then `n · 2^k` by squaring (K's form; the 62-bit loop gone); `Nat.pow`'s limits now exhaustion; `load.shard` prints `RUN: exhausted RESOURCE in F`, exit 3 with fuel; `tc.shard` `nat_apply` op 14: `Nat.shiftRight a b` is zero without the power when `8·bytes(a) ≤ b`; `prims_test` 127 cases (zero shift by 10^12 = 0; a count past 2^32 exhausted `nat_count`; a size past 2^27 bytes exhausted `nat_size` before any work; the negatives distinct: a negative operand `guard`), `ev_test` 57; `LANGUAGE.md` §6.1, §6.2, §6.4, §9, §12.4's row, §13 item 32; `CANON.md` C1 | verified worse than the memo said: `ev`'s `shiftLeft` had none of the three guards K's `nat_apply` has, and K's own `shiftRight` computed `2^k` for any `k` (a shift of 1 by 2^40 would ask the host for 128 GB). No cache exists in `ev`, so the memo's retry clause holds by construction; K's `Exhausted` is never cached (§9.4). A sum or product is still measured after the operation: its size is bounded by its operands', which are literals under the cap |
| R52 make the sealed K boundary the actual phase-3 opener | accept — the completion criterion, landed 2026-09-13 | `LANGUAGE.md` §13 item 26 (the criterion: the first `meta/` consumer imports the view only, never the constructor, `env_pin`, the admission path or memo state, transitively or by a profile shortcut; the forged-node fixtures on the public entry; a raw-construction client fixture; `RUNNABLE`, `REALIZE` and pending obligations separately represented across the seal), §11's row; `v3/README.md` "Open obligations" | already the plan (slice 6's deferral, item 26); the memo's addition is the acceptance test — "not merely a new view file whose consumers still import the implementation" — now the opener's gate wording. No code before the opener |
| R53 frontend parity must state and test its information-preserving scope | **amend** — uniqueness enforced, not a second comparison; landed 2026-09-13 | `kernel/test/parity_test.sh` (each closure checked for one declaration per short name among the heads `fn`/`extern`/`sig`, among the types, among the constructors, and no constructor named like a head — a failing closure fails the gate before its dumps are compared); `test/reader_kit.shard` `take_line` → `first_line`; `dump.shard` and `dump.rs` headers (the projection and its omissions: the measure clause, the literal's kind); `LANGUAGE.md` §10 item 1, §13 item 33 | the sweep found the memo's case live: `take_line` declared twice with different bodies (`json.shard`'s by a literal arm, the kit's by an `if`), in three closures whose dumps carried both lines while every call printed the same — so the tie could not say which the bootstrap ("first definition wins") or the reader resolved. Under the uniqueness check the `a.f`/`b.f` fixture the memo asks for is refused by construction; the amendment declines a validated identity mapping between the two loaders' name schemes as a third artifact to validate |
| R54 positive conformance requires successful completion, not only equal output | accept — **landed 2026-09-13 (slice 9)** | `v3/t0_full.sh` `tie` (each engine's status captured and required 0, the tails printed on failure; a `T0:` verdict line required of each log; then `cmp`); `kernel/test/t0_gate_test.sh` (stub engines under a throwaway export: both complete and agree passes; both fail identically, one fails, identical logs without the verdict line — each fails) | the fixture's own status was already checked by `t0_fixture_test.sh`; the prefix's was not, and both ties discarded it with `\|\| true`. The full replay's status, verdict and closures were already required and are unchanged |


### 4.9 R55–R62 (the LANGUAGE.md ratification memo, on `dabfa42`, answered 2026-09-14)

The memo: `docs/archive/foundation-v3/SHARD_V3_LANGUAGE_RATIFICATION_MEMO_v0.1.md`
("ratify most of the language direction, amend the contracts
identified below, and distinguish the current Stage-0 profile from
permanent language rules; continue the existing implementation plan";
eight requests R55–R62 and a recommended disposition — ratify /
profile / amend — for each of `LANGUAGE.md` §13's 33 items). Every
source observation was verified against the tree; four of the memo's
"reasoning examples, not executed" were **probed live** before
answering (R56, R58, R59, R61 — the probes under a throwaway root,
now pins). All eight accepted; landed as slice 10 (`v3/README.md`).
The user's go-ahead: "agreed with your leans, proceed with the slice."

| ID | disposition | where | notes |
|---|---|---|---|
| R55 make the bootstrap profile explicit; no directory-based authority | accept — contract text, landed 2026-09-14 (slice 10); **DISSOLVED the same day** by the one-E ruling (§9: phase 3 opened) — the profile is retired, there is nothing left to name | `LANGUAGE.md` §8 "What the profile is" (a named, bounded compatibility mechanism for bring-up; recorded per module; placement grants no privilege; ordinary S the destination; a profile transition recognized when a file crosses the boundary), §6.7's profile paragraph, §13 items 8 and 16 amended; `loader.shard` `RModule` carries the flag — `MODULE … profile=toolchain` | already the phase-3 guard ("the profile is bring-up, never a dialect"); no marker form, since §8's bootstrap reason stands; the cache test is the store's (phase 3); the "no privilege" half is item 26's seal criterion; the small ordinary-S library with an operation and a theorem is phase 3's Stage 1 (`fn` = `def` + `realize`) |
| R56 a typed, representation-independent discriminator for `if` | accept — **FIXED 2026-09-14 (slice 10); CLOSED on CI pipeline 479** (`db2dee3`); probed live first | `classify.shard` `if_bad` (the condition's static type where the declarations fix it: a `sig type` is `private_if`, an inductive of other than two constructors or `Int`/`Nat`/`Symbol` is `if_type`; a type parameter unchecked at Stage 0, stated); pins `if_private`, `if_type`, `if_one`, `if_ok`; `LANGUAGE.md` §6.2 (the observation is the type's, `ev`'s bit its implementation), §6.7 item 5, §13 item 9 amended; `CANON.md` C9's domain | **live, and worse than the memo said:** a consumer's `(if h 1 0)` on a view's `sig type Handle` was RUNNABLE and the implementation's constructor order decided the branch at run time; a three-constructor type branched by ordinal parity (`Tri.B` then, `Tri.C` else); a one-constructor type never branched. The leak check covered `match` scrutinees only; the condition was never typed. The toolchain's own 20 closures classify clean under the new rule (parity 0 differ) |
| R57 distinguish module matching, operational linkage and logical instantiation | accept the distinction as contract text; the construction deferred to the phase-3 two-instance gate | `LANGUAGE.md` §6.5 evidence binding (three statuses; the checked instance record inherits the `fulfills` proofs' assumptions; at Stage 0 the records name the parts and compose nothing), §6.6's `DISCHARGE` kinds, §11's row, §13 items 12, 13 and 18 amended; `loader_test` (the kinds `type` / `fn` / `pending` distinct; `IMPL PARAM lib.push fn` — the sig fn stays a parameter after an E match) | the records already distinguished the statuses (`DISCHARGE NAME fn` re-admits the parameter; `proved` / `pending` for a requirement); what is missing is the composite — a consumer's `params=` and an implementation's `fulfills` closures are never joined, and an extra axiom under a `fulfills` reaches no consumer's closure. That is the two-instance gate already in §11, now with the memo's acceptance tests in its row |
| R58 separate realization evidence, progress, applicability and execution trust | accept — **FIXED 2026-09-14 (slice 10); CLOSED on CI pipeline 479**; probed live first | `loader.shard` `realize_roots` (a pending measure recorded once at its constant, `PENDING NAME measure`, and carried by every realization whose body reaches it: `REALIZE … pending=ROOT,…`; `Load` gains the roots table; `prog.shard` `calls_of`); pin `realize_pending_via`, `loader_test`; `LANGUAGE.md` §7.2 (a realization's evidence in its parts; a retained candidate versus a completed realization), §7.5 (the structural-only sentence corrected; the record), §7.1 (the derived view generates no equations; its implementation a bring-up trust dependency), §6.4 (the executor's correspondence is the primitive suite's conformance, not an exemption), §13 items 22 and 23 amended; `docs/TCB.md` bring-up item (7) | **live, exactly the memo's counterexample:** `def f n = 0` with the executable body `f (Nat.add n 1)` under `(measure n)` — the equation is true of the constant function, accepted by `rfl`; the realization attached with `PENDING main.f measure`; a second realization `g` calling `f` attached with no obligation at all. Now `g` and `h` (through `g`) carry `pending=main.f`. Nothing at Stage 0 requires the guarantee, so the narrower guarantee is the visible set; Stage 1's admission and the lowering read it |
| R59 validated entry codecs and an explicit World/handler contract | accept — the codec half **FIXED 2026-09-14 (slice 10); CLOSED on CI pipeline 479**, probed live first; the World half a stated Stage-0 limit until phase 4 | `ev.shard` `list_ctors` by identity and element type (the prelude's `(List Int)`, Init's `(List Nat)` and `(List Int)`); pin `entry_shape`, `entry_test` (a `Tree` with the list arities and a `(List Bool)` are `bad_entry`; `(List Nat)` decodes by identity); `LANGUAGE.md` §9 (successful decoding establishes the codec's type; the World's identity a stated limit), §13 item 29 amended | **live:** `(type Tree (Empty) (Branch Tree Tree))` passed as an entry and `abc` arrived as `Branch 97 (Branch 98 (Branch 99 Empty))` — a value of no type, `stuck no_arm` at the first match on a field; a three-constructor type served as the World. The public wire (`ByteArray`/`String`) is §11's Stage-1 row with the first host-facing library (R60) |
| R60 turn the small authoring gaps into positive Stage-1 commitments | accept — the Stage-1 plan's row, landed 2026-09-14 | `LANGUAGE.md` §11's new row (list literals, the negative-numeral rule, named-field construction and update with the dependent-field caveat, a `Name` literal that forges no identity, the scoped fresh-name supply, the byte and text adapters — each with a first example before the broad port); §12.6 unchanged as the owner table | no code now; the AT RISK rows already named the consumers and regressions; the memo adds the fresh-name supply and the dependent-update rule. Widths above 64 and `(lib …)` stay where §12.6 has them |
| R61 keep identities and validation reusable beyond the current layout | accept with wording; the collision claim corrected, probed live first | `LANGUAGE.md` §3 (the construction is not collision-free; a collision is detected — `duplicate_name` for E, K's `already_declared` for L — never conflated), §13 items 1 and 15 clarified, §11's programmatic-validation row (T6, phase 4); pins `qualified_collision`, `qualified_collision_l` | **probed:** `a.shard` declaring `b.c` and `a/b.shard` declaring `c` both name `a.b.c`; both were already refused deterministically (the E case ends the second file, the L case refuses the second declaration and the citation resolves to the first), so the behavior stood and §3's "two modules can never collide" was false as written. `NAME.realize_N` stays a profile (item 4); the projection stays within its stated domain (item 33) |
| R62 consolidate LANGUAGE.md into one current normative account | accept — landed 2026-09-14 (slice 10) | `LANGUAGE.md`: the status preamble cut to a declaration (revision, stage, normative parent, scope, the record's location); the seven conflicts fixed in their original paragraphs — §3.3 and §6.5 (the seal's boundary is K), §7.2 (one equation per case-tree leaf), §8's table (the profile keeps flat scope), §6.4 (`gen_fresh` out of the table), §6.3 (Stage 0 has declared E types and the static reconstruction, not Stage 1's typing), §7.5 (structural or a retained candidate), §6.2 (`ev`'s definition without the old tree's frontier loop; §6.7 is the machine); the slice-5 measurements marked history | the chronology the preamble carried is records §9's; nothing deleted, nothing upgraded. The memo's recommended disposition for each of the 33 items (eighteen ratify, seven profile, eight amend) is the ratifier's input, recorded here and not in §13, which stays the implementation's list until the user's pass |

GPT-6's recommended dispositions of `LANGUAGE.md` §13, for the
ratifier: **ratify** 2, 3, 5, 7, 11, 19, 24, 26, 28, 30, 31, 32 (and
1 and 10 with the R61 clarifications, 15 as an implementation choice,
23 with its amendment, 17 and 21 as ratify-or-profile); **profile**
4, 6, 14, 20, 25, 27, 33; **amend** 8, 9, 12, 13, 16, 18, 22, 29 —
eighteen, seven and eight; every amendment is now in the item's text
(slice 10). My leans agreed with all 33.

### 4.10 R63–R71 (the single-frontend memo, on `3062f23`, answered 2026-09-18)

The memo: `docs/archive/foundation-v3/SHARD_V3_SINGLE_FRONTEND_REVIEW_MEMO_v0.1.md`
("proceed with the 3.16 single-frontend refactor; do not replace the
logical foundation again; strengthen the construction, erasure and
partial-result contracts while the refactor is still small"; nine
requests on slice 3.16's design, revision 1). Its source observations
checked against the tree before answering: `Bool` and `Decidable`
have separate cells in `ev.shard` (`init_bool_cell`, `dec_cell`) —
**revision 1's rule 3 was wrong** beyond an `if` condition; an `if`
in a `fn` is `Decidable.rec` today with the proof field in scope, so
`count`'s obligation has its hypothesis, and **revision 1's plain
`ite` would have dropped it**; revision 1's unchanged-hash gate
contradicted that same rule; `assign_checked` installs an assignment
when type inference gives nothing, deliberately for a value with a
metavariable inside and also, not deliberately, when K refuses a
closed value; `Nat.sub`, `Nat.div`, `Nat.mod` are in the primitive
table already. Not verified (the memo ran no reproducer either): the
`df_branches`/`tr_minors` scope asymmetry — that code is deleted by
the slice, and rule 2 states the contract for its replacement. The
user's ruling: "Agreed on all three leans, revise the design" (the
discharge path inside 3.16; R68 at I's opener; the flagged E-first
fallback until 3.17). Landed as the design's revision 2
(`LANGUAGE.md` §8.4, §13 item 49).

| ID | disposition | where | fixture |
|---|---|---|---|
| R63 the pre-definition a contextual construction object | accept | §8.4 slice 3.16 "the shape": the `PreDef` record, never an admitted declaration; the invariant adopted in the memo's wording; the erasure's implementation a stated trust (§7.1) — landing 1 | G1, G9 |
| R64 matcher descriptions as lowering inputs; forcing | accept, **stronger** — the description is bound by regenerating the matcher from it and comparing with the admitted value (the generator is deterministic: a check, not a trust); forcing is `EMatch`'s and `EIf`'s rule, now stated and tested | rule 1 — landings 1–2 | G2, G9 |
| R65 branch facts in every position; one descent proof completed | accept — `ite` compiled dependently inside a recursive function; obligations closed over every local at the call in any position; `(fulfills f.dec_N PROOF)` for `count`. Narrower guarantee stated: no `scrutinee = pattern` equation for a computed scrutinee yet | rules 2, 8 — landing 3 | G4 |
| R66 realization-directed erasure; the `Bool`/`Decidable` bridge | accept — the explicit conversion `(if ⟦inst⟧ true false)`, identity only where the entry returns a `Bool` cell; the table as a realization registry; `Nat.sub`/`div`/`mod` selected once resolved (revision 1's refusal reversed) | rules 3–4 — landing 2 | G1, G3 |
| R67 partial outcomes; tentative against validated assignments | accept at the shared boundary — K's outcome through `kw.shard`; `UStuck`, `UExhausted`; typed/tentative assignments; a closed value K refuses is `UNo` | rule 9 — landing 1 | G6 |
| R68 a hole's context across binder closure | **defer** to I's opener (slice 3.20; 3.19 when written — deriving took that number on 2026-10-01) — narrower guarantee: incomplete, never unsound; no API outside `elab.shard` takes an `MCtx` across a binder close until then | rule 10 | G5 at 3.19 |
| R69 contextual obligations, acyclic discharge, dependency scheduling | accept the discharge and its acyclicity (the closure instrument over the proof's constants; `fulfills_cycle`); **defer** dependency-directed wake-up (the retry fixpoint stands while loads are small) | rule 8 — landing 3 | G4, G7 |
| R70 a fallback may weaken guarantees, not reinterpret | **amend** — a `RUNNABLE` callee is a local of its signature type, so the body elaborates once and erases (`route=typed`); the E-first fallback stays, flagged `route=e_first`, only for a body head without an L identity until slice 3.17 empties that set; a typed-route error is always a refusal; the E-first route's named consumers are `v3/kernel/**` and the old tree's ports | rule 6 — landing 3 | G8 |
| R71 semantic gates, not frozen hashes | accept — exact agreement is the migration alarm; a moved hash is listed with its cause; the next three consumers share `PreDef`, the branch scopes and the outcomes | "Landings and gates", rule 11 | the battery |

### 4.11 R72–R77 (the trajectory review, on `7e99261`, answered 2026-10-06)

The review: `docs/archive/foundation-v3/SHARD_V3_TRAJECTORY_REVIEW_2026-10-06.md`
("V3 is moving toward a useful system. Confidence in its logical
foundation has grown faster than confidence in its authoring
experience"; six findings over the tree at slice 3.21's close, four
reproductions through route 3 — a numeric reading, a load-order
dependence, a redundant `reduce`, an irrelevant row under an explicit
certificate — and an authoring assessment; "keep the core
architecture"). Three of its findings are the three items slice 3.21's
as-built left open (the compound literal at `Nat`, Init's visibility
per load, `reduce_stuck` on a normal target); the review adds the
order-dependence demonstration, the observation that a specification
and its implementation inherit one wrong numeric reading while both
proofs check, and the positional certificate's fragility. Its script
was not executed here (the session's permission classifier refused
code from an outside document); each observation was checked against
the code's mechanism, and the slice's pins reproduce findings 1–3.
The user's ruling: "Agreed with your leans … proceed with 3.21b"
(2026-10-06): findings 1, 2 and the `reduce` policy as a short slice
before 3.22; content-keyed certificates and witness goals into 3.22's
design; findings 4 and 5 weighed at the next arc boundary; finding 6
nothing to decide now.

| ID | disposition | where | fixture |
|---|---|---|---|
| R72 elaboration establishes a meaning the author did not intend (`(- 1 2)` at `Int` is 0) | **accept, fixed** — an arithmetic operator under an expected `Nat` or `Int` takes that type's identity and reads both operands at it, as Lean's `binop%` does; a comparison's operands decide by the first that is not numeric-closed; an author's boundary (`Int.ofNat (- 1 2)`) holds. An elaborator defect against law §5.2's order, not a design change | §8.4 slice 3.21b rule 1 | `op_expected` |
| R73 a module's meaning depends on what else has been loaded | **accept, fixed** — each imported declaration carries its admission ordinal; a module's horizon is the farthest of its own `(import Init NAME)` and its imports'; resolution sees Init below the horizon only. The checked environment stays shared; indexed dependencies (the review's eventual step) not taken | rule 2 | `init_horizon`, `init_horizon_beyond` |
| R74 authored proof scripts and certificate IR have different maintenance needs | **accept in part** — `reduce` succeeds unchanged where nothing reduces (the refusal `reduce_stuck` retired; `reduce_budget` stays); a stored certificate is keyed by its rows' statements, not their positions — 3.22's design, with the dependent witness goal (`apply Eq.trans` without its middle term). The ruling that the source block is I stands: the review's "authored node against resolved certificate" is the sidecar's contract, one grammar | rule 3; 3.22 | `by_reduce` (expect changed) |
| R75 feedback cost is a primary language-design constraint | **accept the priority, defer the work** — a load that streams the second fixture pays ~16 s on the development machine before any check (37 s on the review's); the loader pins pay it 212 times (CI's suite is that entrypoint). A checked snapshot of the Init prefix is the cheaper first lever, the persistent session (T9) after it; weighed at the arc boundary after 3.22. `LANGUAGE.md`'s status block refreshed (I is §8.4's since 3.20; items 1–38 ratified 2026-09-15); `docs/LEAN.md` is phase 3's deliverable by §10.5, not yet written | — | — |
| R76 composition through execution and representation boundaries | **accept the sequencing** — the connected path of law §12.4 before broad migration; at the boundary after 3.22 | — | — |
| R77 first-order residual execution needs a source-composition workload | **noted** — the discriminating experiment (two callback-using libraries and a third-party composition) is the reopening condition's workload; nothing to decide now | law §4.3 | — |

## 5. Findings and corrections made along the way

- **The hash-only accelerator pin (2026-09-12, GPT-6 R42; §4.7).**
  Phase 1's `pin_if_matches` enabled a shortcut on a 61-bit structural
  hash match alone, against §3.2's letter. Executed through K before the
  fix: a `Nat.add` whose body is one literal, chosen by one subtraction,
  was pinned and `Nat.add 2 3 = 5` accepted by `rfl`. Fixed the same day
  by the structural comparison of the identity closure against generated
  reference declarations; the regression is hostile battery 7b–7d. The
  same affine hash collides on ordinary `Init` terms (`List.cons`'s type
  against `List α → α → List α`): it keys buckets and nothing else.
- **The raw entry trusted its caller's node data (2026-09-12, GPT-6
  R43; §4.7).** A term handed to K wears an id, a hash, a loose-variable
  range and two flags its author wrote, and every shortcut consumed
  them: executed before the fix, a theorem `0 = 1` whose two literals
  wore one positive id was ACCEPTED through `check` — `quick_is_def_eq`'s
  `expr_eq` returned true on the ids. The seventh `0 = 1` hole of the
  year (the six of `docs/TCB.md` were the old tree's), contained to the
  raw API whose callers were reviewed toolchain code. Fixed the same
  day: `check` rebuilds every node of a submitted declaration from its
  structure (`ingest_expr`), a tree walk the import's own lineage never
  pays; the regression is hostile battery 16 (forged ids, a claimed
  range, a claimed flag, a stale hash, snapshot forks, independent
  construction, no trace after a failure).

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
  §3.2 (`hash_expr`; `accel_pins.shard` generated from the pinned export
  — the hash table, superseded 2026-09-12 by reference declarations
  compared structurally: R42, §4.7;
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
  min): 20 rows, the fifteen old hashes unchanged (the hash table; its
  rows became reference declarations 2026-09-12, §4.7). The string-literal
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
- **2026-09-12 — GPT-6's implementation review (R42–R48, on
  `d6f1fd6`; §4.7).** Verified against the tree, R42 executed through K:
  real, and worse than stated (a one-subtraction preimage). Fixed the
  same day — the identity closure compared structurally against
  reference declarations, 206 rows, the collision and a re-spelled
  dependency in the hostile battery — before slice 2, as a phase-1
  closure defect. R44 ruled sequential (0 sites depend on parallel
  binding); R47 and R48 as doc fixes; R43 and R45 ride slices 3 and 5.
- **2026-09-12 — slice 2 landed: the s-expression and Stage-0 readers
  into K.** `sexpr.shard` (§2 to the letter; `'` an identifier
  character, the quote macro at a token's start only; the profile's
  `-7` a flag) and `reader.shard` (§4–§5 S → L; scope resolution as
  data the loader builds; `structure` projections over `proj`; the
  reducibility height from the environment as environment.cpp does).
  Scoped to S → L: a `fn`'s body is inseparable from head
  classification, so its reader lands with the classifier at slice 5.
  The direct S → L → K path is exercised without any import: a world
  (`Nat`, `Eq.{u}`, structures, `type`s) declared from S text, `rfl`
  theorems verified by K, projections and `let` reducing, every
  refusal with its declared reason, T8's origin-only invariance
  (whitespace, comments, binder names change no declaration and no
  admitted identity). `v3/pins/reader/` opens the new tree's corpus: 12
  S files, each carrying `;; expect:` in a header line, replayed from
  the empty environment. 13 entrypoints, 0 failed. **CI (pipeline 461,
  `9d2ae0e`):** green — the full-export replay on compiled K in 2,499 s
  at a 31.0 GB peak, all 20 accelerator candidates pinned by structural
  comparison (R42's closing gate), closures identical, byte-ties
  identical. Pipelines 457–460 had failed in the runner before any
  project code ran (the `c-srv3` host rebooted; its Ceph mount raced
  the MDS and failed; the kubelet's dependency kept that node's API
  server down; the single control-plane endpoint took every Cilium
  agent with it — a cluster matter, diagnosed and reported the same
  day).
- **2026-09-12 — slice 3 landed: the loader.** `loader.shard` (§3,
  §3.1–3.3, §9: package root, modules by path, `import` once with the
  transitive closure visible and nothing opened, `(import Init NAME)`
  streaming the export through NAME after the meta line matches the
  pin — two nested prefixes loaded once, on from where the first
  stopped —, `use` and `(use P a b)`, `(trusts …)` as the per-file
  policy over every admitted declaration's axiom closure with a
  declaration outside it dropped and the environment kept, `sorry`
  pending and its citation the obligation, a `type` opening its own
  namespace) and `load.shard`, the driver printing §3.2's records. The
  rules §3.1 left open are written as §3.3 and §13 items 10–11. K's raw
  entry now ingests (R43, §5). Tests: `loader_pins_test` over
  `v3/pins/loader/` (20 package roots, each `main.shard` with its
  `;; expect:` header: the scope rules, cycles, missing and directory
  imports, the Init prefix and its nesting, a target past the chunks,
  policy in and across files, `sorry`, `type`, an E form deferred, a
  false theorem refused) and `loader_test` (the records' text, nested
  Init loads, the wrong-pin fixture, visibility across two root files,
  the root's spelling changing no record). 15 entrypoints, 0 failed,
  2 s on route 3; T0 on the fixture unchanged. **CI (pipeline 464,
  `77ebcab`):** green — the full-export replay on compiled K in 2,499 s
  at a 31.0 GB peak, all 20 candidates pinned, closures identical,
  byte-ties identical, 15 entrypoints. Cleanup beside it:
  `is_ws`/`is_digit`/`starts_with_lit` had three copies (json, sexpr, a
  test) under a bootstrap that silently keeps the first — one each in
  `util.shard`; sexpr's comment-skipping `skip_ws` is `skip_blank`.
- **2026-09-12 — slice 4 landed: views.** §6.5's three conditions at
  Stage 0, the mechanics written as `LANGUAGE.md` §6.6 before the
  code and ratification items 12–14. The old tree's structural opacity
  (two same-named typedefs in one closure, a preferring lookup) cannot
  live in K's one-name environment; the rule that replaces it is one
  environment per role: consumers see the view's parameters (`sig
  type`, `sig fn`, `requirement` as axiom-kind constants, the policy's
  third class, named on each record's `params=`); the implementation
  `DIR/BASE.shard` is checked in a fork of the loader's state from the
  view's fork point (after its directives), the view replayed with the
  implementation's `type` at each `sig type` (parameter count), the
  `fn`'s E signature against each `sig fn` (`expr_eq` of the Π-types,
  the parameter then admitted — a `fn` has no L meaning at Stage 0),
  each `requirement` discharged by `fulfills` with the statement
  re-read where the types are concrete (proved or pending), the
  implementation's forms consumed in file order up to the implementing
  one so a private helper declared before it is admitted first; a
  view's theorem is not re-checked, its closure binds it. The fork
  point is kept on the view's module record, so the check runs whether
  or not a consumer loaded the view first. The req-scope gate refuses
  a view's plain-file import before the file is read; `fn` and bodies
  are refused in a view, parameters outside one. Tests: 14 view cases
  in `v3/pins/loader/` (a `;; roots:` header names the files the
  loader is given; `impl-error` joins the expectation words) and five
  in `loader_test`; 15 entrypoints, 0 failed. What waits: the seal of
  `CheckedEnv` behind `kernel/env`'s view needs the toolchain's own
  sources under this loader (slice 6); the private-equality leak is
  refused at classification (slice 5). **CI (pipeline 466, `2f7f032`):**
  green — 15 entrypoints, the full-export replay on compiled K in
  2,459 s at a 31.0 GB peak, all 20 candidates pinned, closures
  identical, byte-ties identical.
- **2026-09-12 — slice 5 landed: the classifier, `ev`, route 2's
  byte-tie.** `LANGUAGE.md` §6.2–6.4 built as §6.7 (written first;
  ratification items 15–19). Reading a `fn` is classifying it
  (`kernel/classify.shard`): a file's E heads pre-registered before
  any body, resolution through one suffix table (a citation's hits are
  the visible identities it is a suffix of; in S those among §3.1's
  scope candidates; the profile flat), every head one kind and
  saturated, the escape rule, exhaustiveness by the pattern matrix,
  the private-equality leak refused where the scrutinee's static type
  is a `sig type`; the toolchain profile = `kernel/` and `meta/`, E
  only. `kernel/ev.shard`: `ev` as a machine with an explicit
  continuation over a private linked representation (constructor tags
  carrying the second-constructor bit, fuel per function entry, stuck
  reasons naming the function, the machine stopping at an extern);
  `run_prog` performs the host's six externs over the prelude's wire
  cells and resumes — §6.2's frontier loop without a search. Views at
  run time: the implementation's `fn` and `type` take the view's
  identities in the fork and at merge — the substitution is the link
  (an `ELink` declaration was tried and removed). Records: `RUNNABLE`
  (R45's status kind), the classifier's `REFUSE … reason: message`.
  Findings: K's own sources (19 modules, 5,670 declarations) load
  under the profile with no refusal — every kernel `match` exhaustive,
  no ambiguous head — and one missing import (`import.shard` used
  `write_line` without `host.shard`; the bootstrap's flat scope hid
  it); the view pins had used `Init`'s `List.nil` in E bodies, which
  is not E-eligible at phase 2 (§11), so they use native `type`s and
  numerals now. **Route 2 byte-ties route 3 on the fixture**: K
  interpreted by `ev`, hosted on the bootstrap, 184 lines identical,
  exit 0, 26 s against 0.33 s (`kernel/test/route2_test.sh`). Tests:
  `ev_test` (35), 19 classifier pins (53 in all), three run cases in
  `loader_test`; 17 entrypoints, 0 failed. A review after the build
  (an Opus subagent; every finding verified before acting) caught the
  pre-registration counting S's `(T Type)` binders as arguments — a
  real bug on an untested path, fixed — the `Nat` shifts keeping the
  host's 64-bit guard (total now), and four hardenings of the typed
  pass: a constructor of another type than the scrutinee's known type
  (`pattern_type`), type arguments inferred so a polymorphic wrapper
  cannot launder a sig-typed value past the leak check, a pattern
  variable named like a visible function refused (`pattern_name`: 12
  such shadowings in 5,670 functions, renamed), the measure term
  classified beside the body; and a file module colliding with a
  directory module refused (`module_collision`). Dropped: `gen_fresh`.
  Deferred to slice 5b: `realize` in both forms and R45's third test,
  with the E→L translation question (§6.7). Cleanup beside the slice:
  `hex_val` unified in `util.shard` (json and sexpr each had one);
  `hash_name`/`name_key` moved to `name.shard`; `nested.shard`'s
  pattern variables `n1`/`n2` renamed (they shadowed `util.shard`'s
  name helpers); `reader.shard`'s `self.` symbol built from its bytes,
  since the V3 lexer reads a trailing dot as a universe suffix — found
  by loading every kernel entrypoint under the classifier, which reads
  all of them now and classifies every closure clean (`loader_test`'s:
  25 modules, 6,184 declarations) — the matrix found two non-exhaustive
  matches in the classifier's own first draft, one in `reader.shard`
  (`elab_proj` lacked the pending arm) and one in the reader kit
  (`step_form` lacked the parameter arm). **CI (pipeline 468, `d76b7a2`):** green — 17 entrypoints, route 2's byte-tie in 46 s on the runner, the full-export replay on compiled K in 2,489 s at a 31.0 GB peak, all 20 candidates pinned, closures identical, byte-ties identical.
- **Direction checkpoint (2026-09-13, user).** Before slice 5b the
  user asked for a step back: "it appears we are on track to build a
  second language and try to import shard into it, rather than
  building a more expressive type and proof system into shard", then,
  after the account, "the main worry was that we would end up with two
  completely separate languages, L and E, rather than L being a
  superset of E … something like marrying rust and lean into the same
  'language' with a binder system". Reviewed against the law: E is by
  definition a fragment of L (§2: "the L declarations that are shard
  programs"; the classifier is the *fragment* classifier); a `fn` at
  phase 3 is a `def` plus its executable structure under one identity,
  its equations theorems K proves by unfolding, never axioms about a
  foreign semantics; `realize` is the route for constants *not born as
  a `fn`* (Lean's own `List.append`, a `def` written in raw L), and the
  Stage-0 ruling of 2026-09-07 made it temporarily the only bridge,
  which is what made 5b read as a binding layer. Ruled: **continue as
  planned**, with two guards carried forward — (1) at Stage 1 `def` and
  `fn` share one term grammar and one elaborator (`fn` = a `def` that
  also passes the classifier; if phase 3 elaborated `fn` bodies but
  left `def` bodies in explicit L there would be two dialects for
  real), stated in the phase 3 opener before code; (2) the toolchain
  profile is bring-up, not a dialect: its spellings never gain L
  identities of their own, and the split closes when the toolchain's
  sources port to S under the naming law. Slice 5b proceeds on the
  leans stated in the same exchange (below).
- **Slice 5b (2026-09-13): `realize`** — `LANGUAGE.md` §7 built as
  §7.5, written before the code. A runtime check settled the biggest
  risk first: on the 3,000-line fixture K proves by `Eq.refl` the
  equations a supplied realization states — `Nat.add n (succ m) = succ
  (Nat.add n m)`, `List.append (cons x t) ys = cons x (List.append t
  ys)`, `List.concat` both cases, `ite c h t e = Decidable.rec (fun _
  => a) (fun _ => e) (fun _ => t) h`, `Array.push as x = Array.rec (fun
  l => Array.mk (List.concat l x)) as` — and refuses the wrong one
  (`conversion`); 0.36 s for nine. Decisions taken on the leans stated
  to the user (§13 items 20–24): the equations' types from the
  classifier and K by first-order matching over the callee's
  telescope; Init's E-eligible inductives as E types on first citation
  (`erase.shard`; `Nat`/`Int` excluded by name — `nat_constructor`);
  K's accelerated `Nat` set in the primitive table under its
  identities plus the three decisions, an entry never realized
  (`realize_primitive`); one equation per leaf of the case tree,
  compiled column by column (a column's variable replaced by the
  constructor term everywhere — nested patterns state nested left-hand
  sides — a variable row bound to it), a non-variable `match` and an
  `if` the recursor with a constant motive; descent syntactic, callees
  realized earlier. Found while building: `pre_realize` cannot resolve
  an imported target before the file's directives are loaded (the
  heads are registered in a second pass after them, and the realized
  head is ensured before its body is read for a native target); the
  slice-5 typed pass typed S numerals as `Int` (now `Nat`); the E
  table's `mod_visible` needed the `Init` sentinel; the `ev_unknown_head`
  pin's premise (Init's `List.nil` not E-eligible) dissolved with the
  slice and the pin now cites a name that resolves to nothing. The
  `LATER` record removed (a `fulfills` outside an implementation check
  is `fulfills_outside_impl`); `classify.shard`'s E table split into
  `etable.shard`; `read_efn`'s tail shared as `read_fn_rest`. **Review
  after the build** (an Opus subagent with a soundness brief; every
  finding reproduced before acting): (1) a real hole — a `fn` whose
  module-qualified identity coincides with an admitted constant (a
  file `List.shard` declaring `append`) passed `check_calls` as a
  realized callee and the equations were stated about Lean's
  `List.append` while `ev` ran the `fn` — closed at the root: a `fn`
  or `extern` may not take an admitted constant's identity
  (`name_taken`, `realize_name_taken`); (2) `(measure E)` had switched
  the descent check off entirely, so `twice n = twice n` was attached
  pending — a self-call on the parameters themselves is now refused
  under any measure (`realize_measure_loop`); (3) a split did not
  substitute the body's environment, so an arm mentioning the matched
  parameter stated an ill-scoped equation K refused (`fvar_in_type`)
  — fixed, with the kept binders whose types mention the split
  variable re-opened (`realize_param_use`); (4) the `equations` clause
  could not name a universe-polymorphic theorem and reported an
  unresolvable name as `bad_shape` — the theorem is taken at the
  constant's levels, the reasons are `universe_arity` and
  `unknown_constant`, and a pin exercises the clause
  (`realize_equations`); (5) `ev`'s `Nat.pow`/`add`/`mul` lacked K's
  literal-size guards (stuck now where K exhausts); (6) `Prod`/`Sum`
  were silently ineligible (their result sort a `max` the binders
  solve; fixed) and `Fin` is documented as phase 3's; (7) refusal
  records now name the target's identity (`List.concat`, as the
  `REALIZE` line does) and carry the reason symbols `realize_kind`,
  `realize_primitive`, `unknown_constant` instead of one
  `realize_target`; (8) the derived view's minor opener guessed `Nat`
  for a binder K could not type — an error now; dead helpers removed
  (`roles_of`, `ev_flag`, `unwords`, `type_names`). Left as noted: a
  refused `realize` leaves its pre-registered head, so a consumer
  `fn` is `RUNNABLE` and unlinkable (as after a refused `fn`, the load
  already failing). Tests: 21 `realize_*` pins (74 in all), three
  `loader_test` cases (R45's third test), 21 `ev_test` cases (56 in
  all). **CI (pipeline 470, `6e11748`):** green — 17 entrypoints,
  route 2's byte-tie in 47 s on the runner, the full-export replay on
  compiled K in 2,458 s at a 31.0 GB peak, all 20 candidates pinned,
  closures identical, byte-ties identical.
- **Slice 6 (2026-09-13): calc's program half under `ev`, frontend
  parity, the seal deferred.** Opened with a design report and five
  rulings (user: "1: commit the 1.0MB fixture, 2: port all, 3: yes, 4:
  defer, 5: accept"): the `Int`-bearing fixture committed
  (`init_prefix_int.ndjson`, the export through the `Int` inductive at
  line 17,812; loads in about 2 s), all thirteen calc files ported, the
  old side's driver in the old tree (LAYOUT forbids `v3/` importing
  it), the seal deferred, a bootstrap `dump` verb accepted. Written to
  `LANGUAGE.md` first (banner; §6.6's deferral; §10 items 1 and 2 as
  built; §11; §12.1 and §12.6 rows; §13 items 25–28). **Calc:**
  `v3/examples/calc/` is the program half in S, one file per old file,
  the claims a header line each — verbatim under Init's `Int`, `List`,
  `Option` and `Bool` with `use` lines for the modules and the types'
  namespaces it cites, the measure proofs reduced to their terms,
  `Int.ediv`/`Int.emod`, its own `list.shard` for `append` and `len`,
  its own pair where `drive` returns one (`Prod` lies past the
  fixture); 53 declarations `RUNNABLE`, 0 refused. An S program names
  no wire cell at phase 2 (the prelude's `List` beside Init's is what
  §3.1's candidate rule refuses), so the drivers sit outside the
  program: `kernel/test/calc_harness.shard` loads the package and
  calls `ev` with values built as data, rendering in the old tree's
  value syntax; the old side is the tower's expression mode over
  `examples/calc/calc_differential.shard` — `eval direct`'s flat
  resolver cannot follow `std/list`'s directory-module imports —
  driven case by case by `kernel/test/calc_test.sh`: 34 cases per input
  line and 13 folds over one input set of 21 lines, **byte-identical,
  727 lines a side** (the port in 7 s, the old tree in 154 s).
  **Frontend parity:** `kernel/dump.shard` and `load.shard --dump`
  print a `Prog` as one canonical text (§10 item 1: sorted lines, last
  components, `?i` by first occurrence, sequential indices, literal
  chains, no measure clause), `rust_bootstrap/src/dump.rs` and `eval
  dump` the bootstrap's Module in the same text — its parallel `let`
  resolved to binders and re-indexed sequentially; `kernel/test/
  parity_test.sh` compares them over every toolchain closure:
  **byte-identical, 18 closures, 61,080 declaration lines, 37 s** —
  TCB bring-up item 2 retired (`docs/TCB.md`). **Findings, all fixed
  and pinned:** (1) the driver's own closure had never been classified
  — `rz_open_ctor`'s wildcard arm sat one match too deep, a real
  non-exhaustive match the bootstrap tolerated because its callers
  pass constructors only (found by the classifier through the parity
  run; a paren-placement slip of 5b); (2) a `type` whose constructor
  bears its name — v2's commonest idiom — was `ambiguous_type` in a
  binder: an E-type position now takes the type (§13 item 28;
  `type_ctor_name`); (3) a file whose reading failed after its heads
  were pre-registered was re-read by the next root's import and
  reported `duplicate_name`: it is recorded as failed and a later
  import says `import_failed` (§3.3; `retry`, `loader_test`); (4)
  `LANGUAGE.md` §12.1 still said the profile gains `use` lines at
  slice 6 against §13 item 16 — withdrawn. **The seal (§13 item 26):**
  `kernel/env`'s view would export the environment's mutators, so it
  is no seal; the boundary is K, and sealing K — fifteen files as one
  directory module, a view of the 87 functions and twenty types the
  rest of the toolchain uses, a bootstrap resolver that follows a
  directory import, a rule refusing direct imports into a sealed
  directory — is the phase-3 opener's, when `meta/` is the first client
  outside `kernel/` and the view can list what it needs. **Left for
  slice 7's close-out:** of T1's three phase-2 fixtures only the
  decision tag is covered; the branch-local proof needs `Fin` (phase
  3) and the checked entry needs the driver's argument validation (§9),
  which it does not have — phase 2 cannot close without a ruling on
  them. Tests: 19 entrypoints, 0 failed, 237 s (route 2's byte-tie 21
  s; calc 161 s; parity 37 s). **CI (pipeline 472, `e20e2ec`):** green — 19 entrypoints, the calc differential in 385 s on the runner (the port under `ev` 15 s, the old tree's tower 370 s), parity in 118 s, route 2's byte-tie in 52 s, the full-export replay on compiled K in 2,473 s at a 29.6 GB peak, all 20 candidates pinned, closures identical, byte-ties identical (engine 189 s, corpus 1,501 s, v3 3,870 s; the pipeline 4,061 s).
- **2026-09-13 — slice 7 landed: the phase-2 close-out.** The design
  report re-derived the slice from the law (§12.4 item 2's gate: T1's
  three fixtures, T5, T8's replay half, the four conformance suites)
  and asked five rulings; the user: "1: defer, 2: build, 3: yes, 4:
  slice 8, 5: we can leave the README note for now." **Built:** the
  **checked entry** of `LANGUAGE.md` §9 — `run_prog` validates the
  driver's `-- ARG…` against the entry's parameters before the World
  (`Int` a signed decimal, `Nat` an unsigned one, a two-constructor
  list type taking the bytes), refuses a malformed argument as
  `RunArg` by its position and text (`RUN: argument N REASON: TEXT`,
  exit 6; never a file span), an unservable parameter or a missing
  World as `bad_entry`; an entry with the World alone still reads the
  raw list through `get_args`, the entry declaring its own
  precondition; pins `entry` (the profile) and `entry_s` (S),
  `kernel/test/entry_test.shard`, 13 cases. The **primitive suite** —
  `kernel/test/prims_test.shard`, every one of the table's 39 entries
  with a positive, a negative and a boundary case fixed by hand, 120
  cases — found that the host's refusal of a primitive call is fatal
  (the bootstrap falls through to its effect handler), so every guard
  is `ev`'s: `Nat.pow x 0` divided its size estimate by the exponent
  through a strict `bool_and` and died; `sym_of_chars` guarded "bytes"
  where the host decodes UTF-8, so a lone 255 died; `Nat.sub` sat
  outside the non-negative guard. All three fixed (`utf8_ok` is the
  new guard). **The T5 audit**, the battery's eight fixtures to their
  evidence: the consumer over the view alone and with the
  implementation linked (`loader_test`, slice 5); the private-equality
  leak refused (`ev_private_match`, `ev_launder`); two same-spelled
  nominal types never conflated — new pin `same_spelled`, Init's
  `Bool` beside `main.Bool`, the bare citation `ambiguous_type`, each
  citable in full, which needed one rule: **`Init.NAME` cites the
  imported NAME explicitly** (§3.1, §13 item 30; the reader's
  `init_cited`, the E table's `eresolve`) — the import's prefix had
  been display only, and a shadowed imported name had no spelling at
  all; an import identified only by declared mapping (`init`,
  `init_nested`, `init_not_found`, the wrong-pin case); physical
  relocation changing no identity — the root's location, whitespace,
  comments, binder names (slice 2, `loader_test`), while a native
  declaration moved across files is a rename by the naming law (§13
  item 1), by design; a realization attached to an imported
  declaration with a theorem about the original still usable — new
  pin `realize_theorem`, a theorem about `List.append` stated before
  its `realize` and cited after it; the **imported**-theorem form
  waits for a prefix that reaches one (`ite_self` at export line
  18,116, past the `Int` fixture; §11); identification by spelling
  refused (`realize_name_taken`; §3: no form identifies two
  identities); **two validated instances of one interface** — not
  expressible under §6.6's one implementation per view directory (§13
  item 14), carried to phase 3 with item 14's siblings (§11).
  **T8's replay half:** direct P (exact-term theorems checked by K)
  and origin-only invariance at slice 2, the route recorded on every
  log since phase 1; the sidecar, store and migration-patch items are
  phase 3–4 by the law's own table. **Rulings:** T1's branch-local
  proof is carried to phase 3 — `Fin`, `dite` and `Nat.decLt` all lie
  inside the `Int` fixture, so the export is not what blocks it; `Fin
  n` needs a value parameter at E, which §7.5's eligibility rule
  (types and propositions only) does not admit, and building that
  ahead of Stage 1's typing was declined (§11); the canonical S form
  (CANON's rule set for S) is **slice 8, the last of phase 2**; the
  viewer keeps a README note rather than V3 cards. **Doc rows (law
  §10.5, phase 2):** `tools/zed-shard`'s highlights and outline know
  the V3 keywords and forms (0.3.0); `shard-viewer/README.md` says
  what the map shows under `v3/`; `.gitlab-ci.yml` and `run_corpus.sh`
  name the phase-2 gate; `TCB.md`'s V3 roster cites the conformance
  suites; the root README's arc line. The loader tests share
  `kernel/test/loader_kit.shard`. Tests: 21 entrypoints, 0 failed, 245
  s (parity 20 closures, 67,976 declaration lines, 43 s; calc 6 s and
  155 s; route 2's byte-tie 20 s). **CI (pipeline 474, `bed273c`):** green — 21 entrypoints, the calc differential byte-identical in 401 s on the runner (the port under `ev` 16 s, the old tree's tower 385 s), parity byte-identical over 20 closures and 67,976 declaration lines in 128 s, route 2's byte-tie identical in 52 s, the full-export replay on compiled K in 2,478 s at a 29.6 GB peak, all 20 candidates pinned, closures identical, byte-ties identical (engine 194 s, corpus 1,547 s, v3 3,949 s; the pipeline 4,145 s).
- **2026-09-13 — slice 8 landed: the canonical form of S
  (`v3/CANON.md`).** The design report re-derived the slice from law
  §10.5's CANON row (the rule set rewritten for S; §7 superseded by
  §8.3; the law carried) and §12.5 (the fmt gate on V3 at phase 6),
  measured the tree (the old `shardfmt --check` over 210 V3 files: 109
  canonical, 98 drifting, 3 unreadable — `v3/kernel` 1 of 30, its
  3,103 of 17,745 lines over 80 columns; the universe suffix `Eq.{1}`
  is "unreadable source" to the old reader; the toolchain's 163
  `let`s, 27 multi-binding, 10 nested) and asked six rulings; the
  user: "Your leans look reasonable, let's proceed with that." **The
  rulings:** a file of its own beside `LANGUAGE.md`, superseding
  `docs/CANON.md` for the V3 tree; the rule set alone, the E
  recognizer the phase-6 slice's first step (the old law's precedent:
  the document ratified, then its first slice); C3 recast — the flat
  sequential `let` is canonical, a let whose body is a let merges,
  order stays meaning, dead bindings refused; C9 generalized — a match
  over any two-constructor type whose arms bind nothing is an `if`,
  since `if`'s tag rule reaches every such type; no reformat now; §7's
  Merkle spec superseded by §8.3 with the store, its within-component
  ordering lesson kept as a note. **The document:** what carries (the
  thesis, the architecture, the goal-position exemption — now the L
  forms — the exclusions, the certificate taxonomy and escape-rule
  criterion, the verification discipline, the depth price); the three
  layers of S (layout over the text; the term tier over `ETerm`'s nine
  formers; the L layer); shardfmt's layout law carried with a keep
  table for the V3 heads and the reader's lexical rules (the V3
  formatter is a printer over `kernel/sexpr.shard`); C1–C12 recast in
  one table (C6 a reader rule by construction; C7's `canon-rules`
  reserved for phase 3); eight new L rules (arrows for a non-dependent
  Pi, `Prop`/`Type` over `Sort`, flat application, the universe suffix
  in L and never in E-type positions, flat `let`, `exact`/`sorry`,
  binders with default markers omitted, the header block's order —
  the Init import, imports and `use` lines sorted, `trusts`); why the
  parity text is not this form; canonical S injective into L up to the
  quotient; the ratchet on V3 (stages 0–4, the gate at phase 6, the
  tools per the manifest); §9's eleven decisions, six ruled and five
  open (normal-form levels, `_` for an unused variable, the header
  order, the reserved form, declaration order). Doc rows: the old
  CANON's banner; `LANGUAGE.md`'s banner, §11, §12.5, §13's pointer;
  the V3 README.
- **PHASE 2 CLOSED (2026-09-13, slice 8) — pending the ratification
  pass.** Every gate item of §12.4 item 2 has its evidence or its
  ruling below; law §10.5's phase-2 doc rows are all written. What
  remains before phase 3 opens is the pass the law's review loop
  requires at a phase boundary: the user's and GPT-6's, by commit SHA,
  over `v3/LANGUAGE.md` §13 (items 1–30) and `v3/CANON.md` §9 (items
  7–11 open). The phase-3 opener then states its two guards — one term
  grammar for `def` and `fn`; the profile is bring-up, never a dialect
  — and takes up the seal of K (§13 item 26).
- **2026-09-13 — slice 9: GPT-6's checkpoint memo on the closed phase
  (R49–R54, on `b45cc22`), answered in §4.8 and landed.** The user:
  "New GPT-6 review memo -- take a look" → the report verified every
  source claim, probed R49 through K (pinned; never applicable), found
  R51 worse than stated (`shiftLeft` unguarded under `ev`; K's
  `shiftRight` unbounded in its count) and R53's case live
  (`take_line` twice), and proposed the dispositions; "Your leans look
  reasonable, go ahead with the slice." Landed: the kind check in the
  matcher with hostile 7e; the three-valued primitive outcome with the
  guards mirrored from K and `EvExhausted` through the driver; the
  parity harness's injectivity check and the twin renamed; `t0_full.sh`
  requiring completion with a stub-engine test; `CANON.md`'s execution
  profile (§1, §9 item 12 RULED) and GPT-6's positions on items 7–11;
  the seal's completion criterion in §13 item 26; §13 items 31–33.
  Tests: 22 entrypoints, 0 failed (parity 20 closures, 68,014 declaration lines, 43 s, every projection injective; calc 6 s and 155 s; route 2's byte-tie 21 s; the gate test's five scenarios). CI (pipeline 477, `4699cbe`, 2026-09-14): green — 22 entrypoints, calc byte-identical in 418 s, parity 119 s injective, route 2 53 s, both byte-ties complete, the full replay 2,418 s at 30.6 GB, all 20 pinned, closures identical (v3 3,916 s; the pipeline 4,065 s).
- **2026-09-14 — slice 10: GPT-6's ratification memo on
  `LANGUAGE.md` (R55–R62, on `dabfa42`), answered in §4.9 and
  landed.** The user: "GPT-6 just finished it's review of LANGUAGE.md
  -- take a look at the new memo" → the report verified every source
  observation, found all seven rows of R62's conflict table real, and
  probed four of the memo's "reasoning examples" live: an `if` on a
  view's opaque type decided by the implementation's constructor order
  (R56; a three-constructor type by ordinal parity), the increasing
  recursion whose true equation proves while its pending measure
  reaches no caller (R58), a `Tree` with the list arities taking bytes
  into its fields (R59), the qualified-name collision refused but
  documented as impossible (R61); "agreed with your leans, proceed
  with the slice." Landed: the classifier types an `if`'s condition
  (`private_if`, `if_type`); a pending measure is recorded once and
  carried by every realization reaching it (`REALIZE … pending=`);
  the entry's byte-list codecs by identity; `MODULE … profile=`;
  eight pins; `LANGUAGE.md` consolidated under R62 with R55, R57,
  R60 and R61 as contract text and nine §13 items amended in place;
  `CANON.md` C9's domain; TCB's bring-up item (7); the memo archived.
  Tests: 22 entrypoints, 0 failed (parity 20 closures, 68,076
  declaration lines, 42 s, every projection injective — the first run
  caught this slice's own `names_union` twin, resolved to
  `axioms.shard`'s). **CI (pipeline 479, `db2dee3`, 2026-09-14): green** — 22 entrypoints, the calc differential byte-identical in 399 s on the runner (the port under `ev` 15 s, the old tree 384 s), parity byte-identical over 20 closures and 68,076 declaration lines in 132 s with every projection injective, route 2's byte-tie in 51 s, the gate test's five scenarios, the byte-ties with both engines complete (the fixture 301 lines, chunks 0–4 3,395 lines), the full-export replay on compiled K in 2,494 s at a 30.7 GB peak, all 20 candidates pinned, closures identical (engine 164 s, corpus 1,537 s, v3 3,938 s; the pipeline 4,104 s) — R56, R58 and R59 closed on the full gate.
- **2026-09-14 — phase 3 opened: the one-E ruling (slice 3.1, the
  design on disk).** After slice 10 the user: "one point is that I
  think it might be good to reschedule things so that the rust
  bootstrap gets built up to full E sooner. If we schedule that next,
  then we can simplify the spec and allow the kernel to use the full
  E internally. How big of a task do you estimate that as?" The sizing
  from the tree (2026-09-14): the bootstrap is 3,549 lines of Rust and
  already reads strings as byte lists, symbols, `(list …)` and
  any-sign numerals, and already skips `use`; its delta is explicit
  `((T Type))` binders, sequential `let` and directory imports. The
  kernel is 30 files, 17,859 lines: 210 functions with an auto-bound
  type variable, 7,710 bare constructor citations (needing only `use`
  lines), 1,494 symbol sites, 1,649 string literals, 719 `(list …)`,
  1,245 operator primitive sites, one negative numeral. The V3 side
  carries the `profile` flag at 22 + 77 + 8 + 20 sites. The estimate:
  about five slices, the Rust part the smallest, the cost in ruling
  four E rules the spec had left to Stage 1 (symbols, strings, list
  sugar, operators) and in the migration under the byte-tie gates; in
  exchange §8 and everything on the profile flag goes, and R55
  dissolves. The user: "agreed, your leans sound reasonable, let's do
  it." Ruled and written: `LANGUAGE.md` §8 (the one E: built-in types
  keyed on `Init` in scope, the profile's literal rules for every
  file, explicit binders, `use` everywhere, one primitive table with
  both spellings, the wire unchanged, sequential `let`, the
  bootstrap's delta; the old table decided row by row; the slice
  order 3.2 bootstrap → 3.3 toolchain by tool → 3.4 the V3 side → 3.5
  documents), §13 items 34–38 (items 8 and 16 withdrawn), §11 and §12
  rows, law §9.2 amended and §12.4 item 3 reordered, the V3 README's
  phase-3 section. The kernel imports no `Init` (the two grep hits
  were the loader's handling of the form), so its `type` forms stay E
  only until the flip under rule 1, unchanged. The bootstrap's
  old-tree special forms `ty`/`tv` have no V3 use (checked) and stay
  for the old tree's `bin/check`.
- **2026-09-14 — slice 3.2 landed: the bootstrap to the one E.**
  `rust_bootstrap/src/load.rs`: a `(T Type)` binder is a type
  parameter (`load_params_in_scope` returns the scope for the result
  type; the parameterized head and the auto-bound name stay for the
  old tree); `let` scoped sequentially; the V3 L forms skipped beside
  `import`/`use`/`sig`; a `Scope` of declared constructors and heads
  against which a dotted citation canonicalizes to the longest declared
  suffix (`Stack.mk` → `mk`, `kernel.json.hex_val` → `hex_val`).
  `eval.rs`: the `let` lowering counts the earlier right-hand sides as
  bindings, not temporaries. `dump.rs`: the parallel-to-sequential
  renumbering (§13 item 25's `resolve`/`print` pair) deleted as dead
  — the printer prints indices as loaded; `Nat` a built-in beside
  `Int`/`Symbol`; a type head by its last component. `bin/eval.rs`
  `resolve_closure`: a directory import visits the view then
  `DIR/BASE.shard`. Two gaps the sizing had not listed surfaced on the
  first directory-module tie and were closed in the slice: the
  bootstrap refused a `theorem` form (the one E puts L forms beside E
  forms in every file), and refused `Stack.mk` in a pattern (a dotted
  citation is ordinary under `use`). A third finding came from the
  full suite: calc's differential runs the OLD tree's kernel on the
  bootstrap, and the old `kernel/module.shard` declares a data type
  named `Type` with `(t Type)` runtime parameters — the first cut of
  the binder rule dropped them (every calc line differed); the rule
  now applies only where `Type` is the sort, that is, not a declared
  type of the closure (`Scope::type_is_sort`; a test). `parity_test.sh`
  gains the directory-module case (`view_basic`: main + view + implementation,
  tied byte for byte against `load.shard --dump` over the same roots).
  44 bootstrap tests; parity 21 closures / 68,080 lines / 44 s; route
  2 byte-identical; 22 entrypoints, 0 failed.
- **2026-09-14 — slice 3.3 landed: the toolchain migrated to the one
  E.** `classify.shard`: `(T Type)` is a type binder in every file
  (`read_params`, `pre_head`) — under the profile it had been read as
  a runtime parameter typed by an auto-bound variable named `Type`, a
  silent arity error the dump would have shown. The migration tool (a
  deterministic pass, kept outside the tree as a one-off; its rules
  recorded in `LANGUAGE.md` §8.3): for each of the 50 files under
  `v3/kernel/`, `(use M)` for every module of the transitive import
  closure and `(use M.T)` for every closure type whose constructors
  the file cites bare, inserted after the last `import`; `(X Type)`
  binders prepended in first-occurrence order where a binder or result
  type cites a name that is neither a declared type of the closure nor
  a built-in. Result: 1,195 `use` lines in 49 files (`loader` and
  `realize` 52 each, `classify` 44; `prelude` none), 13 functions in
  `util` (7), `intmap` (5) and `nested` (1) — the sizing's 210 had
  counted `(a A)` binders over declared types like `inductive.shard`'s
  `A`. Gated as a whole, not per file: parity 21 closures byte-
  identical (the `use` lines are inert under both readers' flat
  resolution, the binders read identically), route 2 and calc
  byte-identical, 22 entrypoints, 0 failed. Open with §13 item 7: the
  per-type `use` lines are what the flat rule was hiding; whether
  `(use M)` should open M's constructors is the ratifier's.
- **2026-09-14 — slices 3.4 and 3.5 landed: the V3 side to the one E,
  the documents closed.** `sexpr.shard`: the `profile` flag gone, `-7`
  a numeral everywhere (the L reader already refused a negative in an
  L position). `etable.shard`: `eresolve` over the scope's candidates
  for every file. `classify.shard`: the flag gone from every signature
  (77 sites); `type_ident` = the built-ins `Int`/`Nat`/`Symbol`, then
  K's constants through the scope, then the E-only types of the scope;
  no auto-bound type variables and no parameterized head; the literal
  kind is the sign; `"…"` and `(list …)` resolve the `List` in scope
  and take its first and second constructors by ordinal
  (`list_in_scope`; `no_list` when none); `Rd` loses the flag.
  `loader.shard`: `Fx` and `RModule` lose their flags, `profile_form`
  and `is_profile` gone, `type_is_e_only` (a `type` over a built-in
  without an L constant in scope, or another E-only type, is E only;
  the declared type excepted), `open_types` (every `type` opens its
  namespace at pre-registration), `read_all` without the flag.
  `prog.shard`: `LStr` carries its `List`'s nil and cons; `ev.shard`
  links a string as its own `List`'s cells; `dump.shard` prints the
  chain with those constructors' short names; `erase.shard`'s scan
  treats a string literal as a citation of `List`; `realize.shard`'s
  messages. The prelude's `Nat` deleted. Pins: `ev_string`,
  `ev_symbol`, `ev_list_sugar` positive; `ev_profile` migrated (`use`
  lines, `(T Type)`); `ev_profile_form` now `read-error
  unknown_constant` (a `def` citing `Int` without `Init`); the `entry`
  pin's kernel-like files gain their `use` lines. **Three findings,
  each caught by an existing gate:** (1) the first cut refused every
  L form in a file that sees no `Init` (`no_init`) and keyed a
  `type`'s E-only status on the same — the `basic` pin, whose
  `a.shard` declares its own `inductive Nat` and `Eq` with no import
  at all, killed it: L needs no `Init`, and E-only is decided per
  `type` by its fields; (2) with the flat rule gone, a `type` opened
  its namespace only for the rest of its file (`open_type` on the L
  path) while heads were pre-registered for the whole file — `ev.shard`
  cites `PrimVal` before `PrimRes` is declared — so the kernel's own
  closures failed until every `type` opens at pre-registration, and
  `form_name` returned `Anon` for a parenthesized head `(type (Lst T)
  …)`, so parametric types opened nothing (`type_name` through
  `type_head`); (3) resolving a type name through the E table before
  K's constants found `main.Bool` alone where `Init`'s `Bool` was not
  registered, so `same_spelled`'s ambiguity vanished — K's constants
  are consulted first. Gates: loader pins 87 cases, loader and entry
  tests, parity byte-identical over 21 closures and 68,075 lines (the
  prelude's `Nat` line gone), route 2 and calc byte-identical, 22
  entrypoints, 0 failed. **Measured:** the V3 loads run about twice as
  long — parity 95 s against 44 s at 3.3 — with each citation now
  checked against the candidates of fifty-odd opened prefixes
  (`hits_in_scope`'s `mem_name` over `scope_candidates`); a prune of
  the `use` lines to the prefixes each file cites, or an indexed
  candidate check, is the follow-up. **A fourth finding, from CI:**
  pipeline 483 (slice 3.3) passed the 22 entrypoints and failed at
  `v3/build.sh` — route 1's compiled K is lowered by the old tree's
  `tools/lower` on the compiled engine, whose front end is the old
  `kernel/reader.shard`; it read the migrated `(T Type)` binders as
  runtime parameters, so `s_len`, `s_append`, … took an extra C
  parameter and every call had "too few arguments". Fixed in the old
  reader with the bootstrap's rule — `Type` is the sort unless a data
  type `Type` is in scope (`parse_param_items`, `type_is_sort`: that
  reader's scope of types is its constructor set, so the test is for
  `module.shard`'s `TCon`); `parse_externdef`'s result type sees the
  binders too. A first cut computed the flag from the current file's
  typedefs: the compiled engine's byte-tie passed (its loader is
  compiled in and never ran the edited reader), the Rust-hosted tower
  failed — every old-tree file with `(t Type)` parameters but without
  the `Type` typedef was read with the binders dropped. Bisected with
  the arm neutralized and the flag forced, then rewritten over the
  constructor scope; both engines tie on the probe.
- **2026-09-14 — slice 3.6 landed: the load time recovered.** The
  3.4 measurement (parity 95 s against 44 s) had a guessed cause —
  `scope_candidates` building fifty names per citation for
  `hits_in_scope`'s membership test. Replaced by `cited_in_scope`
  (`etable.shard`: `name_above` strips the cited suffix off the hit's
  identity and the prefix left is tested against the module, the
  opened prefixes and the empty prefix; `Init.NAME` takes the bare
  imported name only): byte-identical, and parity unchanged at 93 s —
  the guess was wrong. Measured instead on one closure
  (`loader_test`'s, 25 modules): `type_ident` with K's `resolve`
  first, 11 s; with the E table first, 6 s. The profile never resolved
  a binder's type through K; the one-E `type_ident` did so for every
  binder and field. The order matters only where a native type can
  stand beside an imported one, which needs `Init` in scope, so K goes
  first exactly there (`init_seen`, `type_ident_k`) and the E table
  first elsewhere (`type_ident_e`), the same answers everywhere: the
  87 loader pins (`same_spelled`'s ambiguity included), route 2, calc,
  parity 21 closures in 53 s. The structural check is kept as the
  simpler code. Still open: the `use` lines are the flat mirror
  (1,195); a prune to the prefixes each file cites, once the S reader
  can report an unused `use`. The local suite (`v3/test.sh`) never builds
  route 1 — `v3/build.sh` and `t0_full.sh` are CI's — which is why
  three local gates were green over a broken build; the sizing had
  listed the bootstrap and the V3 reader as the two readers of the
  toolchain and missed the third. The profile's ten-row table, as
  history:

  | in the profile | in S | decided |
  |---|---|---|
  | the prelude's names `Nil Cons True False Some None Z S Pair` are the toolchain's own E types, unrelated to `Init`'s | `Init`'s `List.nil` … | unchanged; cited under `use` |
  | type parameters by the parenthesized head or an auto-bound bare type variable | explicit `((T Type) …)` binders | explicit binders everywhere |
  | `"…"` is the `(List Int)` of its UTF-8 bytes | K's `String` literal | the byte list of the `List` in scope, every file, until `String`'s realization |
  | numerals are `Int`; `-7` is a numeral | numerals are `Nat`; negatives are constructor terms | any integer in E, the binder's type; L unchanged |
  | `(quote X)` and `'X` are `Symbol` literals | no symbols | `Symbol` a built-in E type, L identity `String` |
  | `(list a b c)` is list sugar | none | the `List` in scope's chain |
  | a file `import` also opens the imported module | `import` never opens; `use` does | `use` lines everywhere |
  | the primitive names of `docs/LANGUAGE.md` §8 | the naming-law spellings | one table, both spellings |
  | a `type` is E only; `def`, `theorem`, `inductive` refused (`profile_form`) | a `type` enters K and E | per `type` by its fields; K checks any L form |
  | the profile is the `kernel/` and `meta/` directories | every other file is S | withdrawn: one E for every file |
- **2026-09-15 — the route-1 fix's second finding, from CI: the
  corpus.** Pipelines 485 (`6f0f2dc`) and 486 (`0012446`) failed their
  corpus job on 54 targets, every one a `__totality__` failure with
  one cause: `kernel/reader/parse_param_items FAIL: call to
  parse_param_item has no (struct …) clause`. The fix had split the
  binder case into a helper that called back into `parse_param_items`
  — a mutual SCC whose new member declared no `(struct …)` clause
  (`docs/TOTALITY.md` §4: each member declares its own; a callee
  without one fails the site), and whose call back passed its own
  parameter unchanged, so no clause could have verified it. Every
  corpus target that imports the old reader carried the one
  unverified obligation. Fixed by folding the helper back in — the
  only recursion is the list's own descent, `type_binder` a
  predicate on the binder's type form — reproduced and rerun on one
  pin (`pins/proof/nested_measure.shard`: 15/1 → 15/0, both reader
  obligations OK), `bin/rebuild.sh` (byte-tie, STUCKCTL),
  `v3/build.sh` with the compiled `t0`'s fixture tie (301 lines),
  `tools/canon/hash.shard` (runtime `(t Type)` parameters, 31/0) and
  the calc differential. The gate the local checks had missed: an
  edit to an old-tree kernel file needs the measure checker over one
  corpus target that imports it — `bin/shard_eval run
  kernel/check.shard pins/proof/nested_measure.shard` — beside the
  rebuild's byte-tie, which never runs the obligations. Both
  pipelines' `v3` jobs passed the 22 entrypoints and built route 1
  (the reader fix's own gate, green: lower, codegen, cc, the byte-ties
  with both engines) and were in the full replay when this landed.
- **2026-09-15 — phase 3's slices green on CI.** Pipeline 482
  (`345116f`, slice 3.2): green — engine 123 s, corpus 1,778 s, v3
  4,296 s (22 entrypoints, calc byte-identical in 395 s on the runner
  with the port under `ev` 15 s, parity byte-identical over 21
  closures and 68,080 lines in 130 s with every projection injective,
  route 2's byte-tie in 53 s, route 1 built, the full-export replay on
  compiled K in 2,564 s at a 30.6 GB peak, T0 accepted 57,977, all 20
  candidates pinned, closures identical); the pipeline 4,422 s.
  Pipeline 483 (`2c3de64`, slice 3.3): corpus green, v3 failed at
  `v3/build.sh` — the fourth finding. Pipeline 484 (`16a2346`)
  cancelled for the runner. Pipeline 485 (`6f0f2dc`, the reader fix):
  v3 green — route 1 built (lower, codegen, cc), calc 397 s, parity
  261 s over 68,075 lines, route 2 78 s, replay 2,559 s at 30.6 GB,
  20 pinned, closures identical (v3 4,343 s); corpus red on 54
  targets — the fifth finding. Pipeline 486 (`0012446`, slice 3.6):
  v3 green — calc 448 s, parity 148 s over 68,100 lines, route 2
  62 s, replay 2,544 s at 30.7 GB, 20 pinned (v3 4,137 s); corpus red
  the same way. **Pipeline 487 (`f644ca3`, the totality fix): green
  end to end** — engine 305 s, corpus 1,563 s with `CORPUS ==
  BASELINE` and no diff lines, v3 3,975 s (22 entrypoints, calc
  398 s, parity 138 s over 21 closures and 68,100 lines, route 2
  52 s, route 1 built, replay 2,468 s at a 30.6 GB peak, T0 accepted
  57,977, 20 pinned, closures identical); the pipeline 4,283 s. The
  one-E ruling's slices 3.1–3.6 stand on the full gate: every reader
  of the toolchain — the bootstrap, the V3 reader and the old reader
  under route 1 — reads the one E, and the corpus is at its baseline.
- **2026-09-15 — the user's ratification pass over `LANGUAGE.md` §13
  items 1–38.** "Reading through the 1–38 items, this mostly sounds
  reasonable. Are there any in particular you would call out or have
  opinions over?" — six callouts and two stale texts, all agreed:
  item 7 RULED — `(use M)` does not open M's types' constructors
  (Lean's `open` does not; an implicit opening turns every same-named
  constructor pair into a collision that arrives with the `use` line);
  item 26 — the seal lands before the first `meta/` consumer (Stage 1
  and I), directly after the `use`-line prune; item 37 — the string
  flip is a sized slice with its own gates, never an automatic
  consequence of `String`'s realization (1,649 sites); item 35 —
  the rule to watch, pinned both ways (`type_e_only`,
  `type_flip`: the same `(type Foo (MkFoo Nat))` E only without
  `Init`, in K with it; 90 loader pins); item 6 — ratified as
  bring-up, the name-set import expected at the first library; item
  38 — `+` on two `Nat` values is Stage 1's question, recorded. Stale:
  item 25 described the bootstrap's parallel-to-sequential renumbering
  that slice 3.2 deleted — rewritten; items 9 and 17 said "the
  profile's `Bool`" — the prelude's. The rest ratified as written,
  the R57 trio (12, 13, 18) included. `CANON.md` §9 items 7–11
  unchanged.
- **2026-09-15 — slice 3.7 landed: the `use` lines pruned.** The
  loader judges each `(use P)` once its file is loaded (`loader.shard`
  `unused_uses`, the `UNUSED MODULE use=P,…` record): unused when no
  symbol token of the file's forms names, under P, a declaration K or
  the E table holds. Chosen over threading a used-prefix set through
  every resolution site (the reader's `Cx`, the classifier's `Rd`,
  erase and realize — the profile flag's 127 sites again) and over a
  textual guess (a re-implementation of the resolver): the token test
  is the flat mirror of resolution, and its one inexactness — a
  binder or pattern variable spelled like a declaration — keeps a
  line, never drops one. A first cut scanned every prefix against
  every token and cost 60% of a load (parity 80 s against 51 s);
  rewritten over the E table's suffix index (each token's hits, the
  prefix above the token in each identity marked) and a
  last-component bucket of the native K declarations, with the K
  scan only for a prefix neither marks — 59 s, and the tool's own
  20 loads 45 s against 70 s. The one-off tool (scratchpad) ran the
  20 toolchain closures, required every closure to judge each file
  the same (50 files, 0 disagreements) and deleted the flagged lines:
  218 from 38 files; 979 kept — 357 module opens, 623 `(use M.T)`
  type opens (§13 item 7's cost, now counted); the rewritten check
  reproduced the first cut's 193 flagged lines outside the two files
  carrying code edits exactly, and a second run finds nothing. Spot
  checks by hand: `kernel.accel_pins` flagged in `etable.shard` (none
  of its 4,813 names appears bare there), `kernel.env` likewise.
  Gates: 90 loader pins, parity byte-identical over 21 closures and
  68,195 lines, route 2 and calc byte-identical, 22 entrypoints,
  `v3/build.sh` and the compiled `t0`'s fixture tie (301 lines).
  Documents: `LANGUAGE.md` §8.1 rule 4, §8.3, §13 item 7; the V3 and
  kernel READMEs. Next: the K seal (item 26).
- **2026-09-15 — the ratification pass and slice 3.7 green on CI.**
  Pipeline 489 (`b83b4be`): engine 122 s, corpus 1,492 s at the
  baseline, v3 4,005 s — 22 entrypoints, calc 375 s, parity 148 s
  over 21 closures and 68,100 lines (the two item-35 pins among the
  90), route 2 52 s, replay 2,569 s at a 30.6 GB peak, T0 accepted
  57,977, 20 pinned, closures identical; the pipeline 4,129 s.
  Pipeline 490 (`9cd81dc`, the prune): engine 147 s, corpus 1,525 s
  at the baseline, v3 3,910 s — 22 entrypoints, calc 385 s, parity
  161 s over 68,195 lines, route 2 53 s, route 1 built, replay
  2,434 s at 30.6 GB, 20 pinned, closures identical; the pipeline
  4,059 s. The pruned toolchain loads on every reader and every
  route; the `UNUSED` record is silent on CI's closures.
- **2026-09-16 — slice 3.8, the K seal: the design ruled, landing 1 of
  3.** The design report re-derived the surface from the pruned tree:
  169 K names cited by the toolchain's non-test files (89 functions,
  20 types, 60 constructors), 92 of them the data vocabulary (`name
  level expr decl intmap json`) and the rest the trust operations. The
  user ruled the four leans: (1) nine trust files sealed, not fifteen —
  the data types public as Lean's `Expr` is beside `Environment.add`,
  the ingestion at `check` being what makes them safe; (2) `k/k.shard`
  a facade of one-line wrappers, over extending the fork check to
  match a signature in an imported private file, and over merging;
  (3) a view may import a plain file outside its own directory (the
  req-scope gate refined); (4) the internals' tests move inside the
  directory, the hostile battery stays outside as a client. Landing 1:
  the nine files moved (`git mv`), 22 files' paths and `use` lines
  rewritten; `verdict.shard` split out of `env` (`Resource`, `Reason`,
  `Outcome`), `import` (`Verdict`) and `tc` (`Failure`, `KRes`) —
  `Outcome` made parametric so the vocabulary names no sealed type;
  `RawEnv` deleted (nothing used it); the sealed-directory rule
  (`private_module`, `loader.shard` `sealed_above`) with pins
  `private_module` and `private_inside`; the prune lint dropped 12
  lines the rewrite left. Two findings: the leak pins `ev_private_match`
  and `ev_launder` imported the implementation directly and the new
  rule refused them first — the consumer now sits inside the
  directory, the one place an implementation's constructor is still
  nameable, and the leak is still refused there; and an implementation
  whose `type` is E only cannot stand for a `sig type` (my first pin
  used an `Int` field with no `Init`), refused as `bad_shape:
  unknown_constant` — a message to improve at landing 2. Gates: 92
  loader pins, parity byte-identical over 21 closures and 68,214
  lines, route 2 and calc byte-identical, 22 entrypoints, `v3/build.sh`
  and the compiled `t0`'s fixture tie (301 lines).
- **2026-09-17 — slice 3.8, the K seal: landing 2 of 3.** The facade
  lean (decision 2 of the design report) failed on a fact the report
  missed: the fork check matches a `sig type NAME` only against a form
  declared in the implementation file under the view's identity
  (`impl_form`, `form_name`), and `kernel.k.env.CheckedEnv` can never
  be `kernel.k.CheckedEnv`; wrappers serve functions, not types, short
  of boxing. Put to the user with three options; ruled: one module
  across the sealed directory (§13 item 39). Built: `Scope` carries an
  identity prefix beside the module tag (`sc_module`, `sc_tag`; the
  classifier's and realize's registrations by the tag), `Fx` the same
  (`fx_ident`, `sealed_ident`), the private files' `fn`/`type` forms
  recorded at load (`ld_forms`), the fork check matching by lookup
  once an import has loaded the declaration (`match_private` in
  `discharge` and after every consumed form in `advance`), the E
  signatures compared (`check_signature` over `classify_form`'s `ESig`
  and the fn's `EFn`), `check_arity` by lookup; the req-scope gate
  refined (`view_dir`); `env_empty`. The view `k/mod.req.shard`: seven
  sig types, sixty sig fns generated from the sources for the surface
  the non-test consumers cite (57 names), plus `env_empty`, the four
  accessors and the name constants the outside tests use; `k/k.shard`
  the nine imports; nine consumers rewritten to `(import "k")`, the
  private files' sibling opens dropped, five tests moved to `k/test/`,
  the scripts' globs. Findings, each from a gate: (1) a `sig fn` over
  E-only types is unreadable in K (`bad_shape: unknown_constant:
  InductiveType`) — an E parameter only, `sig_e_only` (item 40); (2)
  landing 1's `impl_type_e_only` refusal was wrong, since K's own
  seven types are E-only — matched by the E type's arity
  (`impl_type_arity`), the pin flipped to `ok`; (3) `erase.shard` built
  a `Ctx` and destructured a `Loc` by their constructors — `ctx_new`,
  `loc_ctx`, `loc_st`, `loc_fvar` added to `tc.shard` and the view;
  (4) parity and route 2 must give the loader `v3/kernel/k` as a
  second root for K's consumers (the directory's check links the
  implementation, as the bootstrap's flat closure holds it), for no
  other closure, and never for a test inside; (5) the leak pins
  `ev_private_match`/`ev_launder` could not survive: inside the
  directory the type is concrete and the view's parameter for it
  `already_declared` — retired, the seal closing the case, the
  classifier's `private_match` kept as defense in depth. Gates: 92
  loader pins, K's own check 67 discharges and 0 errors, parity
  byte-identical over 21 closures and 69,031 lines, route 2 and calc
  byte-identical, 22 entrypoints, `v3/build.sh` through the old chain
  with `(import "k")` and the compiled `t0`'s fixture tie (301
  lines); the prune lint quiet.
- **2026-09-17 — slice 3.8, the K seal: landing 3 of 3, the seal
  complete (§13 item 26).** The hostile battery split at the seal:
  the 45 cases that speak through `check` moved outside as a client
  of the view (`e0` is `env_empty`, the budgets `limits`), the nine
  that call the accelerator's matcher directly (7c, 7d, the five 7e
  rows: `accel_ref`, `accel_ref_closure`, `ref_matches`) inside as
  `k/test/accel_test.shard` with the helpers they need copied — the
  two sides cannot share a kit, since a file that imports the view
  and one that imports the implementation cannot meet in one
  environment (the view's parameter and the concrete type
  `already_declared`). The view's last additions: `Limits`,
  `check_with`, `limits`, `nm_string`; `cv_name`, `cv_lparams`,
  `cv_type`, `cval_eq` moved from `add.shard` to the public
  `decl.shard` (accessors beside their data; `cv_name` and `cval_eq`
  left the view). The R52 fixture: `kernel/test/k_client_test.shard`
  (eight cases: the empty environment holds nothing; a raw inductive
  checked into it; the result holds and pins Nat; a definition checked
  over it, both held; a loose-variable declaration refused and the
  environment untouched) and the pin `k_client_reach` (a client naming
  `CheckedEnv`'s constructor: `unknown_head`) — the pins test gained a
  `;; root:` header so a case's files can sit under the package root.
  Gates: 93 loader pins, K's own check 69 discharges and 0 errors,
  parity byte-identical over 23 closures and 80,433 lines, route 2 and
  calc byte-identical, 24 entrypoints, `v3/build.sh` and the compiled
  `t0`'s fixture tie (301 lines); the prune lint quiet (985 `use`
  lines). Item 26's criterion: the structural parts built and pinned;
  the consumer half — the first `meta/` consumer importing the view
  only — enforced by the sealed-directory rule for every file outside
  `k/` rather than awaited. Slice 3.8 closed; next: Stage 1, I.
- **2026-09-17 — the four follow-ups after the seal, ruled; slice 3.9
  records, landing a.** The user's question on V3's shape mid-flight
  ("what is going well, what could be better, where should we focus
  review attention?") was answered with an assessment (records §9 of
  this date, in brief: the byte-tie gates and the review loop are the
  design's strength; the triple-reader tax on K's sources, the `use`
  ceremony, the E-only rule's reach into K's API, positional records
  and tests that bypass the V3 loader are the costs) and four
  build-differently items, all agreed: records with named fields in S
  first, a public fixture kit of raw declarations, a runner mode that
  loads K's clients through the V3 loader, a LANGUAGE.md consolidation
  at the phase-3 close. Records, landing a: v2's form exactly
  (`(record NAME (ctor CTOR)? (FIELD TYPE)+)`, `FIELD_of`,
  `with_FIELD`, `make`, `with`), chosen over accessors in the type's
  namespace (item 42's alternative) for the parity projection's sake
  and route 1's old reader; expanded at the s-expression level —
  `kernel/record.shard` at the loader's two read sites, `load.rs`
  `expand_records` in the bootstrap — no law family at Stage 0. Four
  name collisions found by the gates, none by reasoning: `fields_of`,
  `RcRes` (the classifier's), `Rec` (the loader's record type) and
  `syms` (a pattern variable in `use_form`, `pattern_name`); the flat
  bootstrap had shadowed the first three silently (a `NoMatchArm` on a
  value the wrong `fields_of` received) where the V3 loader refuses —
  the gate worked as designed. Pins `record_basic`, `record_make`;
  parity's 24th closure. Gates: 95 loader pins, parity byte-identical
  over 24 closures and 80,676 lines, route 2 and calc, 24 entrypoints;
  the bootstrap's 44 tests. **Reversed the same day, on the pilot's
  evidence:** planning the loader's four records against v2's
  `FIELD_of` names found four shared field names (`init`, `hash`,
  `view`, `file` — duplicate accessors in one file) and six accessor
  names already taken by functions of the closure (`root_of`,
  `env_of`, `params_of`, `module_of`, `parts_of`, `closure_of`);
  closure-wide accessor names do not survive one file with several
  records. The accessors now live in the record's namespace
  (`Load.env`, `Load.with_env`), opened for the file as any type's is,
  and `with` names its record; the V3 reader needed nothing new
  (dotted heads and the opened namespace already resolve them), the
  bootstrap gained the reverse suffix rule (`resolve_in`: a bare
  citation of a dotted declared head, one such declaration), and the
  dump prints a head by its declared spelling — the identity beyond
  the module tag (`dm_head`, threaded through the term printer) — so
  the parity projection stays injective across records. The generated
  patterns had bound the field names, which now shadowed the accessors
  (`pattern_name` on every record): `fld_FIELD`. Item 42 rewritten
  with the evidence. Gates: 95 loader pins, parity byte-identical over
  24 closures and 80,705 lines, 24 entrypoints, the bootstrap's 44
  tests. **Landing b, the same day — the loader's records
  (`624c6bf`).** `Load` (14 fields), `Fx` (9), `Mod` (7) and `Im` (6)
  rewritten as records: `load0` and the four constructions by `make`,
  the mutators (`ld_with_env`, `ld_rec`, `fx_open`, `im_mark`, …) by
  `with`, the 36 hand accessors deleted and their call sites renamed
  to `Load.env`, `Fx.parts`, `Mod.ok`, `Im.fx` across the loader,
  `load.shard`, the loader kit and the pins test; the wrappers with a
  meaning of their own kept (`ld_recs` reverses, `fx_file` and
  `im_file` join). The first run refused eight functions with
  `pattern_name`: the automatic open of a type's whole namespace had
  turned `recs`, `pending`, `parts`, `forms`, `file` and `root` into
  visible functions of their files, and `load.shard`'s stale `(use
  kernel.loader.Load)` did the same there. Ruled and built: a type's
  automatic open covers its constructors only (`open_types` with
  `OpenSome`; item 7 amended), a record's accessors cited qualified or
  opened by `(use M.T)`; the stale opens dropped. Gates: 95 loader
  pins, parity byte-identical over 24 closures and 80,900 lines, route
  2 and calc byte-identical, 24 entrypoints; the prune lint quiet at
  994 lines. **CI (2026-09-17):** the v3 jobs of 493, 494 and 496 and
  496's corpus were evicted by the runner's taint manager after their
  tests had passed (a node event), and the retries failed at setup —
  the runner cannot reach crates.io or the apt index; the cluster's
  network, not the tree (as with pipelines 457–460 on 2026-09-12).
  CI evidence for the seal's landings 2 and 3 and for records waits
  on the runner; the local gates stand.
- **2026-09-17 — slice 3.10, the test kits (the second follow-up).**
  A census of the test files' definitions found `run` and `main` in
  19 files, `Case` in 17, and the raw builders (`u nat zero lit x_
  val arrow type1 st0 cv nat_decl eq_decl eq_nat refl_nat nat_add`)
  in five to eight each, byte-identical modulo the name constants.
  Two kits under `kernel/test/`, public files both sides of the seal
  can import: `case_kit.shard` (`Case`, `run_cases LABEL cases fails
  w`) and `decls_kit.shard` (the builders and `env_after`, `accepted`,
  `refused` polymorphic in the environment type, so no sealed type is
  named; the names under the kit's own constants `nat_n zero_n succ_n
  eq_n eq_refl_n`, since the view's `nm_nat` and tc's are two
  identities one closure cannot hold). The migration deleted a
  definition only where its body was the kit's exactly — 17 tests on
  the case kit, 8 on the declarations kit — and left
  `inductive_test`'s three variant builders, and each test's `has`,
  `e0`, `e_nat`, `e_eq` (they call `check` and `env_find`, which are
  the view's on one side and tc's on the other). Three tests whose
  `main` had another shape kept calling the deleted `run` until the
  gate said so (`unknown extern run`). Then the lint dropped 40 `use`
  lines the deletions had orphaned. Gates: 24 entrypoints, parity
  byte-identical over 24 closures and 81,145 lines, 95 loader pins.
- **2026-09-17 — slice 3.11, K's tests through the V3 loader (the
  third follow-up).** Measured first: the hostile battery takes 10 s
  through `load.shard --run` against under a second on the bootstrap;
  the seven K-facing tests 46 s in all. `kernel/test/k_clients_test.sh`
  runs each with the V3 loader — the two clients outside `k/` with the
  directory as a second root, the five inside with the implementation
  in their closure — and requires exit 0 and the `failures = 0` line;
  an entrypoint of `v3/test.sh`'s script loop. The bootstrap's flat
  resolution enforces no seal, so this is the first gate at which a
  test that reached around the view fails by running, not only by
  parity's projection. `v3/test.sh`'s header, which still called the
  suite "the gate for the toolchain profile", rewritten. Three of the
  four follow-ups landed the same day (records, the kits, this); the
  LANGUAGE.md consolidation is the phase-3 close's.
- **2026-09-17 — the seal's landings 2 and 3, records, the kits and
  the V3-loader gate green on CI.** Pipeline 501 (`189fc5a`, retried
  after the runner outage: two node evictions, then no crates.io, then
  no DNS; the user fixed the cluster twice): engine 145 s, corpus
  1,493 s at the baseline, v3 3,923 s — 25 entrypoints, calc 376 s,
  `k_clients_test` 7 tests in 141 s (46 s locally), parity 219 s over
  24 closures and 81,145 lines, every projection injective, route 2
  57 s, route 1 built, the compiled `t0`'s fixture and chunk ties,
  replay 2,383 s at a 30.6 GB peak, T0 accepted 57,977, 20 pinned,
  closures identical. Everything since `e19eb12` (the seal's second
  landing) now stands on the full gate: the one-module seal, the
  view's E-signature check, records on both readers, the kits, and
  the seven K-facing tests loaded and run by the V3 loader on CI.
- **2026-09-17 — slice 3.12, Stage 1's design on disk (LANGUAGE.md
  §8.4, §8.5; §13 items 43–46).** The report cut Stage 1 and I into
  six slices against what exists — `realize.shard`'s translation is
  already the E-to-L elaborator, lacking only the definition itself —
  and put five leans: eligibility automatic and transitive with a
  refusal once eligible; the recursor directly, `NAME.eq_N` by
  `Eq.refl`, deeper recursion and measures to 3.14; operator
  identities by operand type only where `ev` and K agree (`-`, `/`,
  `mod` at `Nat` refused: the bootstrap has no static types, so a
  resolution to `Nat.sub` could tie neither parity nor route 2);
  measures `RUNNABLE` until 3.14; one grammar as a union with `tr`
  the elaborator. The user agreed and steered on one point: v2's
  module abstraction — a consumer never resolves the implementation
  to reason about the surface; the requirements stand in for the
  lemmas — deserves a stated stance in V3. Verified in the loader
  that the line holds today (`check_impl` merges the records and the
  E table, never an L declaration) and named the three places Stage
  1 and I would cross it: the fork's new definitions and equations,
  the checked instance built by re-elaboration instead of by
  substitution over P, and deriving on a `sig type` in a consumer.
  §8.5 states the stance; item 46 records it with the rejected
  alternative (exporting `eq_N` through a view automatically, which
  welds every consumer proof to today's body). The old tree's shape,
  for sizing: 13,229 `fn`s, half non-recursive, a quarter structural,
  a quarter under a measure. Next: slice 3.13.
- **2026-09-17 — slice 3.13, `fn` = `def` + `realize` (LANGUAGE.md §8.4,
  §8.5 as built; §13 items 43–46).** `kernel/define.shard` (about 400
  lines) and the loader's definition path; realize.shard's translation
  state became a record carrying the definition context, its self-call
  rule and typed operator identities added, its recursor application
  and checks factored out for the new file. Found on the way, each a
  gate's finding: the local fixtures ended at the `Int` inductive, so
  no `Int.add` — the Int fixture now runs through `Int.decEq` (35,371
  lines) and the pins stream it (`init_not_found` asks for `Int.tdiv`);
  `Eq` is declared after `Nat` in the export, so a definition under a
  prefix through `Nat` gets no equations rather than a refusal; a
  self-call has no static type until the fn is in the E table, so it
  enters the table before its definition is attempted, and `+ - *` on
  two `Nat` operands type as `Nat` statically; a numeral at an
  expected `Int` is `Int.ofNat` (law §5.2's one coercion), which
  `List.sum`'s nil case needed; `recursion_depth` reclassified from a
  refusal to an obstacle when calc's `parse_rest` hit it (slice 3.14's
  shape, not an author's error); a sig type's parameter is now an E
  type on the L-to-E side too (`l_to_etype`), so a consumer's fn over
  a view's opaque type is defined — the `view_*` pins' consumers had
  been refused `realize_signature`; `Nat`/`Int` need Init's constant in
  scope on the E-to-L side (a bare `mk_const Nat` had reached K); the
  parity injectivity check caught `SgOk`/`SgErr` twice (classify.shard
  has them) and the flat bootstrap had hidden a `count_pis` twin inside
  the seal; the V3 loader refused `define.shard` where the bootstrap
  had run it — `PIdent` uncited without its `use`, and `(use
  kernel.realize.DefRec)` opening the record's field names into
  pattern variables (`pattern_name`) — so the file cites the accessors
  qualified and constructs the context through `def_rec`; the
  bootstrap cannot dump a file that matches Init's constructors, so
  `v3/std/list.shard` is not a parity closure (define_test is its
  gate). Twenty pins flipped on the first run, all understood: nine
  refusals of a definition's equations without `Eq`, four consumers
  over sig types, the toolchain-side `Int` reaching K, and three whose
  premise was Stage 0's (`ev_no_l_meaning`, `realize_fn` now use a
  measure as the obstacle; `realize_no_l_identity` uses `band`); three
  loader unit cases likewise (R57's fork status is `defined`; R45's
  third test's fn is `DEFINE`). Gates: 103 loader pins, `define_test`
  (std/list's `List.sum` with `eq_1`/`eq_2` and two theorems; calc's
  `eval` with `eq_1..3` and the claim `eval_add`), parity byte-identical
  over 24 closures and 81,586 lines, route 2 and calc byte-identical,
  K's seven clients through the V3 loader, 26 entrypoints. Next: slice
  3.14 (course-of-values and `WellFounded.fix`).
- **2026-09-17 — slice 3.14, course-of-values, measures and the
  obligation class (LANGUAGE.md §8.4 rules as built; §9; §13 item
  47).** The design's three leans (Lean's `brecOn` scheme for every
  structural recursion, `WellFounded.fix` with the decreasing facts as
  obligation parameters, mutual recursion deferred) ruled and built
  the same day. Findings: the export's `List.below` and `Nat.below`
  decode to `PProd (motive tail) tail_ih` — a single recursive field's
  pair is the whole table, no unit terminator — so the generator and
  the entry reader both changed from the shape first assumed; K's
  instantiation beta-reduces the motive applications in a reduced
  table type (`(λ x . Nat) tail` becomes `Nat`), so an entry cannot be
  read off the type and is derived positionally from the constructor's
  recursive fields, K's check of the definition holding the shape;
  `Init`'s `brecOn` goes through `brecOn.go`, irrelevant to a client;
  a native `Tree` got `Tree.below`/`Tree.brecOn` generated and a
  depth-two recursion over it defined with three equations at the
  first try after the shape fix; the obligations a branch collected
  were lost when the branch's context was restored (`tr_restore`
  keeps them); `InvImage.wf` sits at export line 100,851, past the
  fixture through `Int.decEq`, so the Int fixture now runs through it
  (5.2 MB, 874 KB compressed); a file without `Init`'s `PProd` cannot
  have a table — the prelude's `List` recursions in K's own sources
  had reached K with an unknown constant — so `PProd` reachability is
  an obstacle; a `rec_fields` helper shadowed `record.shard`'s in the
  flat bootstrap (the record expansion crashed with a `RecDef` in a
  match with no arm) and `ArgsRes`/`ArgsOk` twinned `ev.shard`'s —
  both renamed; the rewrite dropped `(use kernel.name)` and the V3
  loader refused every `Name` in the file; the `def`-in-E-forms route
  matched the old result shape. The mutual pin first used `Nat.succ`,
  which is no E constructor (`nat_constructor`), and moved to lists.
  Calc under the V3 loader: twelve functions defined (`parse_rest`,
  `parse`, `flush`, `eval`, `eval_opt`, list's `len` and `append`, the
  spec's accessors), thirteen `RUNNABLE` with the obstacle named
  (`no_l_identity` for the byte predicates, `no_l_meaning` downstream).
  Gates: 107 loader pins, `define_test`, parity byte-identical over 24
  closures and 82,101 lines, route 2 and calc byte-identical, K's
  clients through the V3 loader, 26 entrypoints. Next: slice 3.15.
- **2026-09-17 — slice 3.13 green on CI.** Pipeline 504 (`958274f`):
  engine 176 s, corpus 1,480 s at the baseline, v3 4,112 s — 26
  entrypoints, calc 403 s, `define_test` 2 files, `k_clients_test`
  132 s, parity 235 s over 24 closures and 81,586 lines, route 2
  54 s, route 1 built, replay 2,408 s at a 30.6 GB peak, T0 accepted
  57,977, 20 pinned, closures identical; the pipeline 4,289 s. The
  first Stage-1 definitions stand on the full gate.
- **2026-09-17 — slice 3.14 green on CI.** Pipeline 505 (`9397823`):
  engine 147 s, corpus 1,525 s at the baseline, v3 4,459 s — 26
  entrypoints, calc 418 s, `define_test` 2 files (calc's
  `parse_rest` by course-of-values among them), `k_clients_test`
  127 s, parity 207 s over 24 closures and 82,101 lines, route 2 53 s,
  route 1 built, replay 2,448 s at a 30.6 GB peak, T0 accepted 57,977,
  20 pinned, closures identical; the pipeline 4,608 s. Course-of-values,
  the measured definitions and the obligation class stand on the full
  gate.
- **Slice 3.15 (2026-09-17 ruled, 2026-09-18 built): the elaborator
  above K** — the L-side ergonomics of law §5.1–5.2 (`LANGUAGE.md`
  §5.1 rewritten, §4, §5.3, §8.4's slice-3.15 rules 1–6, §13 item
  48). Three landings: `672826c` the metavariable nodes (`MVar`,
  `LMVar`; K's first edit since the seal, refusing arms only, hostile
  17a–d); `1c12de8` the elaborator (`kernel/unify.shard`,
  `kernel/elab.shard`, the reader split into `scope.shard` and
  `reader.shard`, `W` into `kw.shard`, K's view exporting
  `is_def_eq`): implicit arguments, universe inference, the numeral
  rule with the one `Nat → Int` coercion, the operator spellings and
  `if` in statements, `@`, `exists`; the phase-2 pins rewritten to the
  spelling (45 files), a `type`'s parameters and a `fn`'s type
  parameters implicit; the third landing the constructor-structure
  bundle after a native inductive (`T.casesOn`, and for a type
  without parameters `T.noConfusionType`, `T.noConfusion`, `T.c.inj`,
  Lean 4.33's shapes decoded from the export) and forward references
  parked and retried. Decided on the way: `-` `/` `mod` at `Nat` have
  identities in L only; two K refusals (a false theorem, a universe
  collapse) and §8.5's `view_rfl` now stop at the elaborator as
  `type_mismatch`; a native inductive with universe parameters or
  parameters gets `casesOn` alone (the `HEq` shape later); the
  assignment of a metavariable is typed, which is where a universe is
  inferred; a Miller pattern `?motive t` is solved, so `casesOn` works
  from the expected type. Not in the slice: a `match` in a statement,
  `Bool`/`Decidable` bridging (calc's `is_digit` stays `RUNNABLE`),
  E's rename of `lt le int_eq`, `Fin`/`UInt*` numerals, holes `_`,
  the generator's move to an `aux.shard`. Gates: 115 loader pins,
  reader tests, define_test, parity byte-identical over 24 closures
  and 83,436 lines, route 2 and calc byte-identical, K's clients
  through the V3 loader, 26 entrypoints. **Pipeline 508 (`672826c`,
  the metavariable nodes) green end to end 2026-09-18:** corpus
  1,482 s at the baseline, v3 4,367 s (26 entrypoints, calc 420 s,
  define_test, k_clients_test 125 s, parity 206 s over 24 closures
  and 82,190 lines, route 2 52 s, route 1 built, the full replay
  2,393 s at 32.2 GB peak, T0 accepted 57,977, 20 pinned, closures
  identical) — K's terms carry the nodes and K's verdicts are
  unchanged over the whole export. **Pipeline 509 (`dc6d654`, the
  elaborator and the bundle) green end to end 2026-09-18:** engine
  162 s, corpus 1,512 s at the baseline, v3 4,510 s (26 entrypoints,
  calc 444 s, define_test, k_clients_test 118 s, parity 211 s over 24
  closures and 83,436 lines, route 2 53 s, route 1 built, the full
  replay 2,433 s at 32.2 GB peak, T0 accepted 57,977, 20 pinned,
  closures identical). The slice stands on the full gate.
- **2026-09-18 — slice 3.16 reordered and designed: one front end**
  (`LANGUAGE.md` §8.4's slice-3.16 design, §13 item 49). Scoping the
  planned 3.16 (R60's facilities) against calc's spec file — 12
  functions defined, 13 `RUNNABLE`, from three roots: two `Bool`-valued
  comparisons and `(- c 48)`, whose literal has no type in the
  classifier's typing — showed that slice 3.15 left a `fn`'s body on
  the phase-2 route: E typed by `c_type`, then translated **up** into
  L by `tr`. The user asked whether that was a surface issue or
  something missed at V3's start. Answer recorded: the concept stands
  (the checkpoint of 2026-09-13 above; law §2, §4, §5.1 put the
  classifier after elaboration), the build order reversed it (the
  Stage-0 ruling made E need its own reader and typing first), and
  guard 1 was written at slice 3.12 against `tr` — met in wording,
  missed in substance, and not flagged when 3.15 built the real
  elaborator. A probe before the design (each function as a `fn` and
  as a `def` with the derived view, the two `--dump`s compared):
  `flush` byte-identical with one L hash; a recursive value is
  `brecOn` and refused, as §7.3 said; a hand-written pre-definition
  erases to the classifier's program up to `Nat.add` for `+`;
  `Int.add` refused (no inverse in the operator table); `parse_rest`'s
  nested patterns come back as a case tree with the fall-through eight
  times (same meaning by reading; not run). Also found: `Add` resolves
  to calc's constructor in a `fn` body and is ambiguous in a `def` —
  two resolvers. Ruled: Lean's shape — a pre-definition in `Expr`
  with matcher constants and the self-name a local, erased to E and
  compiled to K; `tr` deleted; the E-first route for the toolchain's
  sources until they port; `if` on a `Bool` as `ite (c = true)`
  (slice 3.15's `cond` reversed: not Lean's elaboration and past
  every fixture). Rejected: keeping `tr`'s skeleton with the
  elaborator called at the leaves (a third hybrid); reading E off K's
  finished value (loses recursion and patterns); a second term
  language with a `match` node (E with types again). The order after
  it: 3.17 the porting facilities, 3.18 bytes, text and deriving, I
  from 3.19. Cleanup seen and left for the slice: `define.shard`
  opens `kernel.level.Level` twice.
- **2026-09-19 — slice 3.16 landing 1: matchers, `match` in L terms,
  the constructor by expected type, the outcomes** (`LANGUAGE.md`
  §8.4's as-built; rules 1, 5, 9). `kernel/matcher.shard` generates a
  matcher in Lean's shape by first-match column splitting and K checks
  it; the elaborator checks it into its walk's environment as it is
  made and the reader hands it to the loader before its owner. **The
  design's side table was dropped on the way:** a matcher's type
  already spells every row's patterns, so the description is read back
  off the admitted declaration and the binding R64 asks for is
  `mt_is_matcher` — regenerate from the rows read back, compare with
  the admitted value. Nothing to lose in a view's fork, which retires
  that named risk. Found while building: a variable bound at one split
  must be **refined** by the later splits on its fields, or the
  fall-through alternative's argument is not the motive's
  (`app_type_mismatch` from K on the first nested-pattern matcher; the
  leaf now applies the splits in order); slice 3.13's fall-back of a
  `def` through the classifier **hid that refusal** behind a second
  successful reading — exactly R70's hazard, met a landing early — and
  is deleted (`def_e_forms`; the elaborator reads `match`, `if` and an
  untyped `let` itself); K's refusal to type a projection whose
  structure is still being declared is `invalid_projection`, not
  `unknown_constant`, so "K cannot type it yet" is decided by whether
  the value cites an undeclared constant (`cites_undeclared`), which
  is what keeps a structure's dependent field readable now that a
  closed value K refuses is no longer installed; a pattern variable
  named like a constructor of K's data (`D`) is a constructor pattern
  to the bootstrap — the elaborator's own sources avoid the capital.
  Deferred inside the landing: the `PreDef` record, to its first
  consumer (landing 2). Cleanup: `scope.shard` opened
  `kernel.decl.ConstantInfo` twice; `reason_of` moved from the loader
  to `kw.shard` with the loader's two opens it no longer needs.
  Gates: 123 loader pins, reader tests, `unify_test` (8), `matcher_test`
  (9), define_test, parity byte-identical over 26 closures and 97,077
  lines with no unused opens, route 2 byte-identical, K's clients
  through the V3 loader, the T0 fixture, 28 entrypoints.
- **2026-09-19 — slice 3.16 landing 2: the pre-definition, the E
  projection, the tie** (`LANGUAGE.md` §8.4's as-built; rules 3, 4,
  11 and "the shape"). `kernel/predef.shard` reads a `fn` once into a
  `PreDef` (the self-name a local); the erasure learned the matcher
  application, the self-call, `decide` as an explicit conversion, the
  `Int` rows of the realization registry; and for this landing every
  function the old route defines is **also** read and erased on the
  new one and the two programs compared (`tie_differs` a refusal).
  **The tie holds** for every function defined in calc's files,
  `v3/std/list.shard` and the 125 pins — nested patterns,
  course-of-values and measured functions included — after what it
  found: (1) `if` on a value of a two-constructor type of the
  program's (`kernel.util.bool_and` over the prelude's `Bool`, a
  pin's `Dec`) had no L reading in the elaborator — now the `match` it
  abbreviates; (2) `(list …)` — now by the expected type; (3) **a real
  two-resolver difference**: `append` in calc's `apply_action` named
  calc's own function to the classifier (Init's `List.append` has no
  E realization, so it was never a candidate) and is ambiguous to the
  one resolver, as the law's §3.1 says and as Lean would say — the
  three calc files now open `Init.List` selectively (`cons nil`);
  calc's differential byte-identical after. Decided on the way:
  `decide` erases to the conversion **always** (`ev` has three
  two-valued cells — the scope's `Bool`, Init's, `Decidable`'s — and
  no consumer needs the identity case yet); an operator's identity is
  the type of its first operand that is not a numeral; a bound head
  inserts its implicits. The erasure stays in `realize.shard` until
  `tr` goes (it shares `Tr`). G2's forcing half and G3 run under `ev`
  (`project_test`); G1 and G8 need the flip. Gates: 125 loader pins
  with the tie armed, `project_test` (8), `matcher_test`, `unify_test`,
  define_test, calc byte-identical over 21 inputs, parity
  byte-identical over 27 closures and 104,557 lines, route 2, K's
  clients, the T0 fixture, 29 entrypoints. **Pipelines 513
  (`77baa91`, landing 1) and 514 (`ca87d5f`, landing 2) green end to
  end 2026-09-19:** 513 — engine 177 s, corpus 1,525 s at the
  baseline, v3 4,941 s (28 entrypoints, calc 420 s, k_clients_test
  123 s, parity 258 s over 26 closures and 97,077 lines, route 2 54 s,
  route 1 built, the full replay 2,564 s at 32.2 GB peak, T0 accepted
  57,977, 20 pinned, closures identical); 514 — engine 134 s, corpus
  1,565 s, v3 4,930 s (29 entrypoints, calc 416 s, k_clients_test
  124 s, parity 291 s over 27 closures and 104,557 lines, route 2
  55 s, route 1 built, the replay 2,503 s at 32.2 GB, T0 accepted
  57,977, 20 pinned, closures identical). Both landings stand on the
  full gate.
- **2026-09-19 — slice 3.16 landing 3: the flip** (`LANGUAGE.md`
  §8.4's as-built; rules 2, 4, 6, 7, 8). A `fn` is read once
  (`predef.shard`, now with the recursion clause and callee-locals) and
  its program is `erase_predef`'s; the classifier types only a function
  whose signature has no L reading, a body with a symbol or string
  literal, or a head past the Init prefix (`route=e_first`, flagged);
  a typed-route error is a refusal. **Decided before the deletion, and
  written to §8.4 first:** `define.shard` is not slice 3.14's table
  bookkeeping re-pointed but Lean's compilation — the matcher stays the
  constant K checked and its motive carries the course-of-values table
  (addArg), a self-call is the table's entry found by type over an
  abstract motive (the real motive is constant, so it cannot tell
  entries apart). It worked on the first load of `std/list.shard` and
  lifted slice 3.14's `recursion_depth` obstacle. **Found on the way,
  each by a gate or a probe:** (1) frontend parity caught landing 2's
  `if`-as-a-match in `kernel.util.bool_and` — `if` on a two-constructor
  type is now `casesOn`, erased back to `if`; (2) parity caught
  `Nat.add` where the old front end says `+` — `Nat`'s five
  integer-identical operations select the integer entries; (3) with a
  short Init prefix the elaborator cited `instDecidableEqBool` without
  checking it exists and K refused at admission — the bridge's
  constants are checked; (4) a probe written to verify a sentence of
  the as-built (a self-call under a `match` in an argument) was
  refused: the definition stood but its unfold equation is not
  `Eq.refl` at a local — a `fn`'s equation is now stated where K
  decides it, the definition checked into the walk's environment
  first (`define_arg_match`); (5) the survey of E-first fallbacks
  showed the literal check running before the signature's, mislabelling
  the toolchain's functions, and calc's trace functions falling back
  whole behind `show_nat` — a fallback function whose signature reads
  is a callee-local for its callers. Rule 8: `count.dec_1` is `∀ p (h :
  ¬ p.1 = 0), …` and `Nat.sub_lt (Nat.pos_of_ne_zero h)
  (Nat.zero_lt_succ 0)` closes it, both lemmas inside the prefix
  through `InvImage.wf` (the named risk did not bite); the closure with
  discharged obligations replaced is the acyclicity check, and the
  records' account is rewritten on success. `tr`, the tie, the E-term
  case tree and `DefRec` deleted (realize.shard 1,846 → 578 lines; the
  erasure in `erasure.shard`). **The slice's gate met:** calc's spec
  file 25 of 25 defined (its Init import extended to `InvImage.wf` for
  `parse_tail`), `calc_test` byte-identical over 21 inputs with the
  erasures as the programs, the twelve functions the old route defined
  keep their equation names (compared against `ca87d5f`'s output),
  every `fn`'s hash moved for one cause (its value cites
  `NAME.match_N`). Eleven pins moved to the elaborator's reason or the
  new outcome; seven new (G1, G4, G7 ×2, G8, G9, `define_arg_match`).
  Gates: 132 loader pins, `define_test` (5 checks), calc, parity
  byte-identical over 27 closures with no unused opens, route 2, K's
  clients, the unit tests, the full suite. Not done here and named in
  the as-built: the scrutinee equation in a measured obligation,
  mutual recursion's definition, dependency-directed wake-up.
- **2026-09-19 — pipeline 515 green on `087001d`: slice 3.16's landing
  3 on the full gate.** Engine 133 s, corpus 1,535 s at the baseline,
  v3 5,168 s: 29 entrypoints with 0 failed, `define_test` 5 checks,
  parity byte-identical over 27 closures and 104,060 declarations in
  322 s with every projection injective, route 1's `v3/bin/t0` built
  from the rebuilt `define.shard`, `erasure.shard` and `realize.shard`
  (the local suite never builds it), the byte-tie on the fixture, the
  full replay 2,449 s at 32.2 GB — T0 accepted 57,977, closures
  identical against the oracle, all 20 accelerator candidates pinned.
  Slice 3.16 is closed.
- **2026-09-19 — slice 3.17's design, scoped by probes** (`LANGUAGE.md`
  §8.4, §13 item 50; no code). Five scratch files through the loader
  under the pins' prefix. A record with `make` out of order and `with`:
  6 functions defined, `(= (Pt.x (Pt.with_x v p)) v)` by `Eq.refl`
  over a local `p` — v2's law family needs no generator. `lt` and `<`:
  one hash. A literal pattern at `Nat` and at `Int`: `no_l_identity
  route=e_first` (0 rows in `v3/`, 53 in the old tree). `Symbol`:
  `e_only_type`; 2,905 `(quote …)` sites in `v3/`, all in functions
  E-first by signature; `String` in the prefix, `String.decEq` past
  it. calc's `show_nat`: **the as-built's cause was wrong** — it holds
  no `"…"`; `Int.ediv` lies past `(import Init Int.decEq)`. Under
  `Int.emod` (1,142 declarations) it is `measure_type`; with
  `(measure (Int.natAbs n))` it falls back again because the measure
  is erased and `Int.natAbs` has no program (`(realize Int.natAbs
  (view))`: `no_realization`). `has_eq`/`confusion_reachable`: their
  callee `env_find` has no L reading — the port's, and recorded
  `no_l_identity` where the as-built says `no_l_meaning`. The rename:
  943 sites in 60 files of `v3/`; the v2 chain that compiles
  `v3/kernel/**` on route 1 names the three primitives at 73 sites in
  10 files of `kernel/` and 3 in `codegen.shard`. Leans and the three
  rulings wanted are in the design.
- **2026-10-01 — slice 3.17 ruled and built: the porting facilities**
  (`LANGUAGE.md` §8.4's as-built, §13 item 50). The user agreed with
  the three leans (`"…"`, symbols and `Name` literals to slice 3.18;
  the rename staged). Built in one session, each part probed through
  the loader before its pin was written:
  - *Numeral rows.* `elab_match_lit` rewrites a match with a numeral
    row to the `if` chain over the scrutinee's name. Probe: `code`,
    `kind` (`Nat`), `of_sum` (a computed scrutinee, bound by a `let`)
    `DEFINE`d; `(= (code 9) 30)` and four more by `Eq.refl`; dumps
    `(if (= #0 0) 10 (if (= #0 5) 20 30))`. **Reversed from the
    design's text** (a `Nat` numeral as a constructor pattern through
    the matcher, read back as E's literal row): a `succ` chain as deep
    as the numeral, nothing E can run, and a read-back that is shape
    recognition. Refusals: `match_not_exhaustive`, `literal_rows`,
    `pattern_type` — the last a tightening: `(match b (0 1) (_ 2))` at
    a `Bool` was `RUNNABLE` (the classifier never typed a literal row)
    and is refused where the signature reads.
  - *Records.* The five laws by `Eq.refl` (pin `record_laws`).
    `make`/`with` over a `structure`: `Span` with `ok : lo ≤ hi` —
    `(with Span s (hi h) (ok p))` accepted and `Span.lo` of it is the
    subject's by `Eq.refl`; `(with Span s (hi h))` is `dependent_update`
    naming `ok`. A structure with parameters: `record_make` (limit).
  - *The rename.* E's table is `= < <=`; 27 sites in 16 files migrated
    by `v3/tools/rename_cmp.py`; `renamed_primitive` from both routes
    (pins `rename_refused`, `rename_refused_e`); modules under
    `kernel` keep the old spellings. The bootstrap: `prim.rs` and
    `eval.rs` read both, `eval dump` prints V3's. **Found:** the
    native table is tied to `kernel/reduce.shard`'s by a conformance
    sweep whose name lists are the spec — the new names are
    native-only, so they got their own spec list (`NATIVE_ALIASES`)
    and a sweep against the names they stand for over the full pair
    matrix (`docs/TCB.md` item 5). The old kernel is untouched.
  - *Fresh names.* `v3/std/fresh.shard`, generic in the prefix type:
    14 functions defined, `Fresh.next_ne` accepted with no axiom on
    the first load (import through `Nat.succ_ne_self`, 1,445
    declarations).
  - *The E-first-by-body set.* `Int.natAbs` as a registry expression
    row; calc's app file imports through `InvImage.wf`: 17 of 17
    defined, `show_nat.dec_1` pending. `has_eq` is `no_l_meaning`
    (a view's `sig fn` hit was falling through to "not a function");
    `confusion_reachable` stays `no_l_identity` — it quotes symbols,
    which the scoping entry above missed.
  - *Gates, local:* 146 loader pins (14 new; `match_literal`
    narrowed), `define_test` 7 checks, calc byte-identical over 21
    inputs (port 18 s, old tree 154 s), parity byte-identical over 27
    closures and 104,232 declarations in 132 s, route 2, K's clients
    (7 in 55 s), the suite's 29 entrypoints with 0 failed, the
    bootstrap's 46 unit tests. Route 1 and the corpus are CI's.
- **2026-10-01 — pipeline 535 green on `62ffe94`: slice 3.17 on the
  full gate.** engine 39 s; corpus 1,667 s, the FAIL set at the
  baseline — the bootstrap reading `= < <=` moves nothing in the old
  tree; v3 5,468 s: 29 entrypoints with 0 failed, `define_test` 7
  checks, parity byte-identical over 27 closures and 104,232
  declarations in 325 s, **route 1's native build** of the kernel
  sources this slice changed (`v3/build.sh`: lowered, generated,
  compiled, `OK: v3/bin/t0` — the gate no local run reaches), the
  fixture byte-tie, replay 2,403 s at 32.2 GB peak, T0 accepted
  57,977 with 0 rejected, closures identical to the oracle, all 20
  accelerator candidates pinned. Slice 3.17 is closed. *CI itself:*
  GitLab created no pipeline for the push for over two hours (its
  background queue lagged: the push event of 17:57 UTC was recorded
  at 19:49); 535 was started by hand (`glab ci run -b main`) and sat
  `waiting_for_resource` on a free resource group until the queue
  drained at about 20:24. The late push pipeline 536, a duplicate on
  the same commit running a second replay beside the first, was
  cancelled.
- **2026-10-02 — slice 3.18 ruled and built: bytes and text**
  (`LANGUAGE.md` §8.4's design and as-built, §13 item 51). *The
  ruling (2026-10-01, the user on five leans):* `String`, `ByteArray`
  and `UInt8` run as the byte list (rejected: a packed buffer now —
  two executors and a table of trusted primitives for the lowering's
  benefit; the interned atom — every string interned for good);
  `ByteArray` on all six externs (rejected: `List UInt8`, `String`
  for paths); symbols and `Name` literals deferred to the toolchain's
  port (item 36 amended); the flip staged like the rename (item 37
  amended); deriving split to slice 3.19, I to 3.20+. *Built:* six
  type rows of the realization registry (`Fin`, `BitVec`, `UInt8`,
  `Array`, `ByteArray`, `String`, each its one runtime field, never an
  E inductive); a projection function as its projection; the literal's
  bytes as `Init`'s `List` cells with `bad_string` and `no_string`;
  the driver's extern codec by declared type; `v3/std/bytes.shard`,
  `v3/std/host.shard`, `v3/examples/calc/calc_main.shard`. *Found
  while building:* a structure's projection in a `fn` body was
  refused before this slice; the E-first fallback reported the
  classifier's refusal and hid the typed route's obstacle (now
  carried); a second reader over the kernel's diff found that an
  E-first body could take a row's value apart with the
  representation's constructors and that `write` and `write_line`
  ignored their declared result — both reproduced, fixed and pinned
  (`row_e_match`, `wire_bad_result`); slice 3.17's as-built said `Int.toNat` was absent from the
  export (it is at line 552,397; corrected in place). *Not built:*
  the checked decoder `String.fromUTF8?` (its proof is I's); rows for
  `Char` and the wider `UInt`s; a link-time check of an extern's
  signature. *Gates:* 156 loader pins (10 new: `row_types`, `row_e_first`, `row_e_match`, `struct_proj`, `str_e_first`, `str_no_string`, `str_bad_utf8`, `wire_s`, `wire_bad`, `wire_bad_result`; `ev_string` rewritten), `rows_test` (10 values under `ev`), `wire_test` (15 checks against the host), `calc_main_test` (byte-identical to the model world over 21 inputs), `define_test` (9 checks: the two libraries added), calc's differential byte-identical over 21 inputs, parity byte-identical over 28 closures and 111,681 declarations, route 2, K's clients, the full suite's 32 entrypoints with 0 failed. `v3/build.sh` built these sources into a scratch binary that byte-ties the interpreter on the 3,000-line fixture. The full replay and the corpus are CI's.
- **2026-10-02 — pipeline 538 green on `682c20f`: slice 3.18 on the
  full gate.** engine 35 s; corpus 1,471 s, job succeeded; v3 6,112 s: 32 entrypoints with 0 failed, `define_test` 9 checks, `wire_test` 15 checks, calc's host program byte-identical to the model world, parity byte-identical over 28 closures and 111,681 declarations in 356 s, route 1's native build of the kernel with this slice's sources (`OK: v3/bin/t0`), the fixture and the 100,000-line byte-ties identical, replay 2,458 s at 32.2 GB peak, T0 accepted 57,977 with 0 rejected, closures identical to the oracle, all 20 accelerator candidates pinned. Slice 3.18 is closed. The v3 job is 644 s longer than pipeline 535's: the three tests that load the libraries through line 263,515 of the export, and calc's differential on a slower runner (364 s for the old tree's side, 220 s locally).
- **2026-10-02 — slice 3.19 ruled and built: deriving under a
  declared policy** (`LANGUAGE.md` §8.4's design and as-built, §13
  item 52; law §5.1). *The ruling (the user on five leans):* a
  derivation table for the use site (rejected: named functions only;
  instance resolution pulled forward); one policy, `structural`,
  written at the derivation (rejected: a package default); the
  ordering `compare` without laws (rejected: laws generated before
  I); the rendering as bytes in canonical S (rejected: Lean's `Repr`,
  a `String`, deferring it); types without parameters and Init's
  `List`, `Option`, `Prod` at closed arguments (rejected: parametric
  and nested types now). *Built:* `(derive TYPE CAPABILITY…)` —
  `kernel/derive.shard` writes `fn` source the loader reads as the
  file's own (`NS.ctorIdx`, `NS.decEq`, `NS.compare`,
  `NS.renderOnto`/`NS.render`); the table in the scope's visible set,
  consulted by the elaborator's decision lookup; `(CAPABILITY by
  NAME)` for a hand-written procedure, at any closed type — a view
  exports a decision as a `sig fn` and its consumer registers it;
  `v3/std/derive.shard` (fourteen entries: `Bool` derived; `UInt8`,
  `ByteArray`, `String` by hand over Init's lemmas; `Nat`'s and
  `Int`'s renderers), `v3/examples/derive/`. *General rules the slice
  needed:* a `match` generalizes a local scrutinee in its expected
  type; an operator in an E-type position takes its levels by
  unification; the classifier reads `(Decidable P)` as the type's two
  cells. *Found while building, each fixed and pinned:* an `if` whose
  condition is a self-call left the self-local in the proposition (K:
  `fvar_in_value`); under a measure the same call was two obligations
  (one per statement now); `and`, `or`, `not` on `Bool`s cited
  constants the export does not have (`Bool.and`, `Bool.or`,
  `Bool.not`, each with a registry row; `Bool`'s equality in general
  position likewise); `=` at an inductive in an E-first body was
  accepted and stuck at run (`prim_type`); a refusal after an
  unreadable signature hid the cause. *Found and left:* an explicit
  numeral argument is unassigned when a later argument is checked.
  *Found by a second reader over the kernel's diff, each reproduced,
  fixed and pinned:* a registry row spoke for any constant bearing
  its name (a module's `Bool.or` ran as Init's while K proved the
  module's: `registry_native`); a `Decidable` value had two runtime
  representations, since before the slice — a match by constructor on
  `(Nat.decEq a b)` took the wrong row; a decision is its test only
  where an `if` reads it and `(if TEST isTrue isFalse)` as a value
  (`decidable_cells`); a derivation took a hand-written `T.ctorIdx`
  or `T.compare` for its own (`derive_name_taken`); a `by` entry's
  visibility depended on load order (`derive_visible`); the match
  rule broke `_` sub-patterns and proofs over a hypothesis about the
  scrutinee (the local bound in scope; the constant motive as
  fallback); `(@List.nil (Prod Nat Nat))` failed on an unnormalized
  level (older; `unify_level`); a self-call in a branch under a
  self-call condition under a measure (`dec_2 fvar_in_type`); late
  refusals for a type without constructors, a proposition, a value
  argument, `Nat` (`derive_refusals`). The same reader checked the
  generated `decEq`, `compare` and `renderOnto` on 1- to 5-constructor
  types on the host against K and found them correct.
  *Departures from the leans:* no instance constants (`Ord T` holds a
  function: Stage 3's); a function-valued field refused under
  `render`; wider — a one-constructor type with parameters derives,
  the library's three types have orderings and two of them renderings.
  *Not built:* the ordering's laws; parametric types with two
  constructors, nested and mutual types (the toolchain's `SExpr` and
  `Expr`); deriving in the toolchain's own sources (the bootstrap
  generates nothing: with route 1's chain). *Cleanup:* an unused open
  in `realize.shard` left by slice 3.18. *Gates:* 176 loader pins (20 new), `define_test` (12 checks), `derive_test` (14 checks), `wire_test` (15 checks), calc byte-identical over 21 inputs, its host program byte-identical to the model world, parity byte-identical over 28 closures and 112,857 declarations, route 2, K's clients, the full suite's 33 entrypoints; route 1's closure is K's own and none of its files changed. The full replay and the corpus are CI's.
- **2026-10-02 — pipeline 540 green on `fdd1429`: slice 3.19 on the
  full gate.** engine 31 s; corpus 1,476 s at the baseline; v3 6,936 s: 33 entrypoints with 0 failed, `define_test` 12 checks, `derive_test` 14 checks, `wire_test` 15 checks, calc's host program byte-identical to the model world, parity byte-identical over 28 closures and 112,857 declarations in 453 s, route 1's native build of the kernel (`OK: v3/bin/t0`), the fixture and the 100,000-line byte-ties identical, replay 2,474 s at 32.2 GB peak, T0 accepted 57,977 with 0 rejected, closures identical to the oracle, all 20 accelerator candidates pinned. Slice 3.19 is closed. The v3 job is 824 s longer than pipeline 538's, 806 s of it in the test suite (3,841 s against 3,035 s): `derive_test` is new at 387 s, the `.shard` entrypoints (the loader pins among them, 20 new) take 253 s more, parity 97 s more, `define_test` 55 s more, K's clients 18 s more.
- **2026-10-06 — slice 3.20 ruled (2026-10-05) and built: I's opener, the
  certificate core** (`LANGUAGE.md` §8.4's design and as-built, §13
  item 53; law §7). *The ruling (the user on six leans):* three
  slices — 3.20 the core, 3.21 `simp_only` and `arith`, 3.22 the
  producers (rejected: one slice; the forms before the API); the
  tactic block in source is I (rejected: a tactic surface distinct
  from I); no subterm order — `induction` carries its recursor, `wf`
  is `WellFounded.fix` over a measure with every descent fact a goal
  (rejected: v2's `subterm-induct` and `(below)`; the porting cost
  accepted); computation is conversion — `reduce` a `show` of the
  computed normal form (rejected: implicit lemma sets); calc's claims
  by hand (rejected: the migration tool first); the API sequential at
  the opener (rejected: the transactional graph now). *Built:*
  `kernel/tactic.shard` — `(by STEP…)` for a theorem and a fulfills,
  I's data `Node`, a goal a metavariable over the closed statement
  (R68 settled by construction), the forms `intro exact rfl have show
  apply cases induction wf decide unfold reduce rw sorry`, `goal_of`
  and `applicable`; a measured function's equations by
  `WellFounded.fix_eq` (`define.shard`); 44 of calc's 100 claims as
  blocks in five files. *Found:* matching by conversion is useless
  (syntactic now); a spine's prefix whnf's to a lambda; K's whnf
  reduces past one step (function applications masked); the
  recursor's argument order; seven name collisions with other kernel
  files under flat resolution. *Found by a second reader, fixed:*
  `cases X H` on a local dropped X from the arm's context; `unfold` of a
  plain `def` refused (one delta step by hand); `(occ K)` counted spine
  prefixes; `reduce` at a closed occurrence with an opaque condition ran
  unbounded (ground occurrences only, 64 unfoldings); doubled goal text.
  18 pins, `tactic_test.sh` (48 checks); the gates of every slice (the
  README for the figures).
- **2026-10-06 — pipeline 542 green on `7238cd3`: slice 3.20 on the
  full gate.** engine 41 s; corpus 1,460 s at the baseline; v3 8,138 s: 34 entrypoints with 0 failed, `tactic_test` 48 checks, `define_test` 12 checks, `derive_test` 14 checks, `wire_test` 15 checks, calc's host program byte-identical to the model world, parity byte-identical over 28 closures and 114,377 declarations in 476 s, route 1's native build of the kernel (`OK: v3/bin/t0`), the fixture byte-tie identical, replay 2,514 s at 32.2 GB peak, T0 accepted 57,977 with 0 rejected, closures identical to the oracle, all 20 accelerator candidates pinned. Slice 3.20 is closed. The v3 job is 1,202 s longer than pipeline 540's, 1,157 s of it in the test suite (4,998 s against 3,841 s): the `.shard` entrypoints take 685 s more (the loader pins among them, 18 new, each now streaming the second fixture), `tactic_test` is new at 238 s, calc's two tests take 184 s more (`calc_spec.shard` imports through `WellFounded.fix_eq`), parity 23 s more. The job's limit is 3 h (10,800 s).
- **2026-10-06 — slice 3.21 built: `arith` and `simp_only`, and calc's
  100 claims** (`LANGUAGE.md` §8.4's design and as-built, §13 item
  54; law §7; under the ruling of 2026-10-05). *Designed here:*
  `arith` is a Farkas certificate over stated rows — the goal
  negated, the context's linear hypotheses, the facts given, the rows
  the atoms bring (a `Nat` atom's `0 ≤ ↑a`; a literal quotient's two
  bounds, the remainder as `x - k * (x / k)`) —, carried by the node
  (`(farkas K…)`) or reconstructed by Fourier–Motzkin elimination
  over the rows (bounded; item 53's second lean: a function of the
  node and the goal), P from `Lean.Omega`'s lemmas with K's conversion
  the arithmetic on closed data; `simp_only` rewrites to a fixpoint
  over its rule list, premises by assumption, nothing implicit; one
  spelling — a goal's statement and a lemma's equation enter in the
  source's spelling of Init's class methods. *Found by the consumer
  and built:* **a matcher under a measure carries its equation** —
  `parse_tail`'s two descent obligations quantified the pattern's
  variables freely, false as stated, so the function could never be
  total and slice 3.20's theorems over it stood on two parameters; the
  obligation is stated under `skip_ws cs = cons c rest` now and the
  equations over the same shape. `reduce` takes a decided case
  analysis one step and leaves a proposition; a type's name wins over
  its constructor's where a type is expected; the pending record
  shows the sorried goals. *The consumer:* calc's 100 claims are
  theorems, the capstone `run cs = spec_run cs` under no parameter,
  the three measured functions discharged. *Found by a second reader, fixed:* `simp_only`
  under a self-feeding rule did not end (`simp_self`); `Ne` at the
  wrong universe; a descent obligation is the comparison at `Nat`
  under the source's names now, where it printed every variable `x`
  over the relation's instance. *Left, stated:* Init's
  visibility is the load's; obligations over the arguments' tuple;
  `reduce_stuck` on a normal target; no cuts, no case splits, no
  `only`. *The test runner:* `v3/test.sh` runs its entrypoints eight
  at a time. Gates: 212 loader pins (18 new), `tactic_test` (110 checks: calc's 100 claims by name, the four descents discharged), `define_test` (12 checks), `derive_test` (14 checks), `wire_test` (15 checks), calc byte-identical over 21 inputs and its host program byte-identical to the model world, parity byte-identical over 28 closures and 116,127 declarations, route 2, K's clients, the full suite's 34 entrypoints (the last full run had 33 pass and parity refuse one missing open in `define.shard`; parity and K's clients were rerun green after it). `v3/test.sh` runs the entrypoints eight at a time now (they are independent processes; the output is printed in the list's order): the suite's wall clock is its longest entrypoint's, the loader pins' (1,161 s here). None of the files route 1 compiles changed; the full replay and the corpus are CI's.
- **2026-10-06 — slice 3.21b built: GPT-6's trajectory review, findings
  1–3 (`LANGUAGE.md` §8.4's design and as-built, §13 item 55; §4.11).**
  An arithmetic operator under an expected `Nat` or `Int` takes that
  type's identity and reads both operands at it — `(- 1 2)` at `Int`
  was `Int.ofNat (Nat.sub 1 2)` = 0 and K proved it so; a comparison's
  operands decide by the first that is not numeric-closed (`(<= (- 0 5)
  x)` at x's `Int`). Each imported declaration carries its admission
  ordinal and a module sees Init below its own horizon — the farthest
  of its `(import Init NAME)` and its imports' — so a module's meaning
  no longer depends on the load's order (the review loaded two
  independent roots in both orders; one order was `ambiguous_name`).
  `reduce` succeeds unchanged where nothing reduces. Gates: 215 loader pins (3 new; `one_meaning` restated, `by_reduce`'s expectation changed), `tactic_test` (110 checks: calc's 100 claims), `define_test` (12 checks), `derive_test` (14 checks), `wire_test` (15 checks), calc byte-identical over 21 inputs and its host program byte-identical to the model world, parity byte-identical over 28 closures and 116,229 declarations, route 2, K's clients, the full suite's 34 entrypoints (33 passed; the pins test refused `one_meaning`'s old statement of the truncating reading — the one meaning change the rule makes in the closure —, and was rerun green after the pin's restatement). None of the files route 1 compiles changed; the full replay and the corpus are CI's.
  The review filed under `docs/archive/foundation-v3/`.
- **2026-10-07 — slice 3.22 ruled and landing 1 built: the producers'
  ground (`LANGUAGE.md` §8.4's design and as-built, §13 item 56; law
  §7.5).** The user agreed with three leans: the engine emits `(arith
  only …)` with the certificate written; the store is a CI artifact;
  the engine's reach is measured over calc's 100 claims. Landing 1: a
  theorem's proof may be `auto`, replayed from the sidecar
  `FILE.auto.shard` keyed by the statement's fingerprint, pending with
  the fingerprint shown where no entry fits, never searched at build;
  `(arith only FACT… (farkas K…))` takes no context row; the witness
  goal (`apply Eq.trans` without its middle term) closed over the
  context and assigned by pattern unification from a premise's block.
  Gates: 221 loader pins (6 new, one renamed), `tactic_test` (110 checks: calc's 100 claims), `define_test` (12 checks), `derive_test` (14 checks), `wire_test` (15 checks), calc byte-identical over 21 inputs and its host program byte-identical to the model world, parity byte-identical over 28 closures and 116,395 declarations, route 2, K's clients, the full suite's 34 entrypoints with 0 failed. None of the files route 1 compiles changed; the full replay and the corpus are CI's.
- **2026-10-13 — slice 4.1 designed: the native toolchain, route 1
  over the kernel's drivers** (`v3/LANGUAGE.md` §8.6 "Slice 4.1"; §13
  item 65 for ratification; phase 4's opener on the user's "find a
  near-term slot … or should we just do it next?"). The probe at the
  close, on scratch copies: `t0` lowers in 81 s; `load.shard`'s
  closure (55 files, 32,644 lines; route 1 today 29 and 13,497) is
  refused at source 30, `matcher.shard`, where v2's reader reads the
  capitalised local `M` as a constructor — renamed, the whole matcher
  closure lowers in 50 s; 26 such binders in 4 files; records the next
  class (7 declarations, 73 accessors, 48 `with`, 11 `make` in 7
  files); the dialects otherwise identical, every extern in `rt.h`; the
  bootstrap's parse of the closure 369 ms. The design: the V3 loader's
  projection as the pre-pass (`--emit`), one build for every driver and
  harness, a byte-tie entrypoint, CI's bootstrap suite the authority
  with the native suite compared beside it, the old tree untouched,
  the binaries engines. Documents only.
- **2026-10-13 — slice 3.30 landing 4 built: the close — phase 3
  closed** (`v3/LANGUAGE.md` §8.4 "as built, landing 4"; §13 items
  39–42, 44–46 and 48, and item 64's as-built notes of landings 2 and
  3, ratified 2026-10-13 — the user: "Read through this, and all of that looks good to ratify to me" — item 46 with its reason
  recorded: "the requirements holding the opaque boundary of the module is an important boundary to keep so that internal mutations that preserve requirements are safe without rippling to all consumers"; item 42's stated cost found live at the close (route
  1's reader expands a record under v2's accessor names; the kernel's
  drivers wait on a pre-pass) and noted at the item; the phase-3
  close-out ledger below closed; this landing's commit the SHA for the
  phase-3 checkpoint memo, run through `codex` on the user's steer and
  answered in §4 as every memo before it; phase 4 open, its first
  slice designed at the boundary). Documents only.
- **2026-10-13 — slice 3.30 landing 3 built: the consolidation
  (`v3/LANGUAGE.md` §8.4 "as built, landing 3", the slice table, the
  title and the status block, §13 item 64's as-built note; this
  record's §9.1; the V3 README).** Rule 6 by its lean: the design and
  as-built texts of slices 3.13–3.29 — 4,391 lines — moved verbatim
  from §8.4 to §9.1 below, one `#### Slice 3.N` heading per slice the
  only addition (the moved text's SHA-256 prefix `a500557b5ca452f3` equal
  before the cut and after the paste); §8.4 keeps 1,115 lines of
  per-facility current statements, each rule numbered as its slice
  numbered it with its as-built note beside it, so the 151 `§8.4
  slice N rule K` pointers in the kernel, `LEAN.md` and the READMEs
  still name a rule; the slice table the index, each row ending in
  its record's heading; the title "S, L, E and I as phase 3 built
  them", the status block rewritten for the close. `LANGUAGE.md`
  7,818 lines to 4,544; this file 3,223 to 7,654. The departure
  stated: the current statements are new prose, the record the
  authority where they differ; GPT-6's memo over the SHA is the
  review. No kernel change, no test run.
- **2026-10-13 — pipeline 595 green on `556b896`: engine 53 s; corpus 1,472 s against 1,474, the old tree's sources untouched; v3 4,650 s of a 3 h limit, 98 s over pipeline 593's 4,552 (the export 130 s; the index driver built by route 1 in 11 s and the export's index built in 96 s on the runner; 41 entrypoints with 0 failed in 1,439 s against 1,425 — `store_test` 1,108 s against 1,097 (calc's five leaves stored in 563 s against 569: 467 declarations; replayed in 89 s against 90), parity byte-identical over 31 closures and 136,763 declarations in 869 s against 860 over 136,735 — the 28 declarations more the landing's kernel growth, `fulfills_misnamed` and its two helpers —, `tactic_lib_test` 775 against 774, `define_test` 702 against 725, `path_test` 695 against 679 (load 74 s, run 81 s, store 82 s, verify 14 s), `calc_test` 646 against 630, `engine_test` 522 against 510, `std_test` 439 against 444 — the std root's 8 files and 356 declarations loaded and stored in 238 s and K's replay 198 s, against 248 and 194 —, `wire_test` 398 against 399, `derive_test` 310 against 309, `calc_main_test` 292 against 308, `template_test` 287 against 296 (load 100 s, run 98 s, dump 89 s; 80 instances), `k_clients_test` 200 against 201, `tactic_test` 149 against 153, the loader pins' entrypoint 140 against 150 (264 cases against 261 — the three of the landing, one of them an implementation directory), `rows_test` 138 against 133, `matcher_test` 73 against 70, `route2_test` 71 against 71 — the runner's variance both ways; route 1's native build of the kernel 420 s against 424; the replay 2,489 s at 32.3 GB against 2,398 at 32.3 — K's sealed module untouched by the landing, the 91 s the runner's); T0 accepted 57,977, 20 pinned, closures identical. Landing 2 of slice 3.30 holds on the full gate.**
- **2026-10-13 — slice 3.30 landing 2 built: T9 again (`v3/LANGUAGE.md`
  §8.4 "as built, landing 2", the slice table, §13 item 64's as-built
  note; `docs/records/T9-2026-10-13.md`; `docs/LEAN.md`; the V3 README;
  the pins README).** Rule 5's run: a fresh agent with `LANGUAGE.md` and
  `LEAN.md` and nothing else (its reading set checked against its
  transcript) wrote the nine tasks — **pass on the count as written**:
  9 of 9 accepted, 0 interventions, refusals before success (iii) 2
  (the author's) and (ix b) 1 unintended, (vi)'s three pointers, (ix a)
  naming the step, both instances and the goal, (ix b) naming the call
  by its obligation; 10 runs, 232 s, about 50 minutes; verified by a
  second load of the nine files (245 accepted, 0 refused, 0 errors).
  Two findings against the kernel, fixed with pins: the author's — a
  `fulfills` of an obligation of a `fn` refused above pointed at §6.6's
  directory (`fulfills_outside_impl`); `loader.shard`'s
  `fulfills_misnamed` now names the cause (`fulfills_refused`,
  `fulfills_not_obligation`, `fulfills_outside_impl`; pins
  `fulfills_refused` under the harness's new `load-error-after` kind,
  `fulfills_not_obligation`); and one found scoping the fix — inside an
  implementation directory a measured `fn`'s obligation could not be
  discharged (the implementation path knew requirements only,
  `fulfills_unknown`, the obligation pending), now discharged as in a
  plain file (pin `view_fulfills_obligation`). Fourteen gaps: ten
  `LEAN.md` rows or additions (unary negation new; `decreasing_by`'s
  discharge with the goal's shape; `PARAM`/`DISCHARGE`/`UNUSED` in the
  records list; the wide horizon; a guessed lemma's binders; explicit
  theorem binders; `(unfold f)` before `dif_pos`; `Subtype.mk` both
  forms, measured; `(occ K EXPECT)`'s two causes with `tactic.shard`'s
  message amended; the self-call at the parameters), one a spelling in
  slice 3.28's rule 5 corrected, one half the author's reading, one the
  brief's wording, one the finding; `LEAN.md` 109 rows. Gates: the
  pins' entrypoint 264 cases, 0 failed, 54 s; the full suite 41
  entrypoints, 0 failed, in 536 s (parity byte-identical over 31 closures and 136,763 declarations in 326 s — the 28 declarations more the landing's kernel growth; `store_test` 415 s, `tactic_lib_test` 279 s, the loader pins' entrypoint 56 s for 264 cases; a quiet machine). Route 1's kernel build, the
  full replay and the corpus are CI's.
- **2026-10-13 — pipeline 593 green on `9543ab3`: engine 51 s; corpus 1,474 s against 1,534, the job green and the old tree's sources untouched; v3 4,552 s of a 3 h limit, 118 s under pipeline 589's 4,670 (the export 130 s; the index driver built by route 1 in 12 s and the export's index built in 97 s on the runner; 41 entrypoints with 0 failed in 1,425 s against 1,478 — `store_test` 1,097 s against 1,139 (calc's five leaves stored in 569 s against 571: 467 declarations; replayed in 90 s against 97), parity byte-identical over 31 closures and 136,735 declarations in 860 s against 901 — the same count, the kernel untouched by the landing —, `tactic_lib_test` 774 against 805 (30 checks), `define_test` 725 against 760, `path_test` 679 against 729 (load 75 s, run 80 s, store 81 s, verify 14 s), `calc_test` 630 against 672, `engine_test` 510 against 531 (54 of 100 claims), `std_test` 444 against 467 — the std root's 8 files and 356 declarations loaded and stored in 248 s and K's replay 194 s, against 252 and 213 —, `wire_test` 399 against 417, `derive_test` 309 against 317, `calc_main_test` 308 against 315, `template_test` 296 against 304 (load 105 s, run 102 s, dump 89 s; 80 instances), `k_clients_test` 201 against 203, `tactic_test` 153 against 168, the loader pins' entrypoint 150 against 143 (261 cases against 260 — the one more is `realize_imported`, which loads Init's `List.append_nil` on demand), `rows_test` 133 against 141, `matcher_test` 70 against 70, `route2_test` 71 against 75 — every entrypoint but the pins' a few percent faster, the runner's variance; route 1's native build of the kernel 424 s against 423; the replay 2,398 s at 32.3 GB against 2,453 at 32.3 — K's sealed module untouched); T0 accepted 57,977, 20 pinned, closures identical. Landing 1 of slice 3.30 holds on the full gate.**
- **2026-10-13 — slice 3.30 landing 1 built: the gate ledger, §11's rows, pin `realize_imported`, the four banners** (`v3/LANGUAGE.md` §8.4 "as built, landing 1", §11; the slice table; this section's phase-3 close-out ledger below; the V3 README; the pins README; `docs/TOTALITY.md`, `docs/REFINEMENT.md`, `docs/STORAGE.md`, `docs/CERT.md`). Rule 1: the ledger as phase 2's, each item with its evidence and its state at the landing; §11's ten phase-3 rows rewritten — landed at their slices (Stage 2 at 3.20–3.22 with the law's `change` read as `show`, the Init import at 5b/3.18/3.26, lambda lifting at 3.28, T1's branch-local proof at 3.24, R60's facilities at 3.15–3.18), carried by rule 3's rulings (the two instances and R57's checked instance to phase 5, the `String` flip to phase 4), Stage 3 as item 61 ruled it. Rule 2: pin `realize_imported` (261 cases) — Init's `List.append_nil` cited before a supplied realization of `List.append` is attached, the same theorem and a theorem that cited it before cited after it, every record `ACCEPT`, the three statements one fingerprint; `(realize List.append (view))` refused `recursion_structure`, a recursive Init body being supplied as `examples/path` supplies `List.length`'s; no kernel change. Rule 4: a banner after each document's STATUS line in `TCB.md`'s shape — `TOTALITY.md` superseded by law §4.5, `REFINEMENT.md` by §4.1, `STORAGE.md` by §7.5 and §8.4, `CERT.md` amended (§3 and §7 superseded, §4's validators and §10's laws carried) — each naming the V3 answer by slice. Gates run: the loader pins' entrypoint 261 cases, 0 failed, 134 s (under a load of 31 from other sessions' jobs); the full suite's 41 entrypoints, 0 failed, in 816 s under the same load (parity byte-identical over 31 closures and 136,735 declarations in 383 s; the suite's 548 s of landing 2 on a quiet machine). Route 1's kernel build, the full replay and the corpus are CI's.**
- **2026-10-13 — item 63's as-built readings ratified (the user: "Those look reasonable; ratify them"): landing 1's guard by structure then unification, the equation hypothesis before the reverted dependents, the sidecars regenerated from nothing; landing 2's one renderer with the source's binder spelling, `Producers`, the E-first library and its six theorems, the fingerprint's independence from the module, the two producers' `ACCEPT` records equal but for the axiom closure — slice 3.29 closed on pipelines 588 and 589. Slice 3.30 designed at the boundary, on "Let's proceed to the next slice": the phase-3 close-out (`v3/LANGUAGE.md` §8.4 "Slice 3.30", seven rules, four landings; §13 item 64 for ratification). Measured first: law §12.4 item 3's list against the tree — every item covered and the gates T2, T3, T9 small and T10 met; the carried T5 fixture under Init on demand (`List.append_nil` cited before and after a supplied realization of `List.append`, every record `ACCEPT`, 39 s; the `(view)` form refused `recursion_structure`, as a recursive Init body is); the law's `change` absent and `show` its form; `TOTALITY.md`, `REFINEMENT.md`, `STORAGE.md` and `CERT.md` without a banner; §13 items 39–42, 44–46 and 48 without a ruling marker; `LANGUAGE.md` 7,579 lines of which §8.4 is 4,437 in twenty "as built" sections, its title still "at Stage 0 (phase 2 draft)". The rulings asked: the two-instance gate and R57's instance (lean: carried to phase 5's first consumer with two implementations), the `String` flip (lean: phase 4 with route 1's chain), T9 again's nine tasks and count, the consolidation's shape (lean: the slice texts moved verbatim to records §9, per-facility current statements kept).**
- **2026-10-13 — pipeline 589 green on `6d18c33`: engine 51 s; corpus 1,534 s against 1,463, the job green and the old tree untouched; v3 4,670 s of a 3 h limit, 13 s under pipeline 588's 4,683 (the export 130 s; the index driver built by route 1 in 12 s and the export's index built in 98 s on the runner; 41 entrypoints with 0 failed in 1,478 s against 1,428 — `store_test` 1,139 s against 1,094 (calc's five leaves stored in 571 s against 565: 467 declarations; replayed in 97 s against 90), `tactic_lib_test`, the 41st, 805 s on the runner against 280 standalone on the dev box (30 checks: the build replays the six theorems with the library loaded and never run, `prove` regenerates the sidecar through the library byte-identical, the engine's and the library's sidecars differ and replay to the same `ACCEPT` records with both stores verified by K alone, the older schema reported and replayed, the changed hints replayed), parity byte-identical over 31 closures and 136,735 declarations in 901 s against 852 over 135,940 — the 795 declarations more are the landing's kernel growth —, `define_test` 760 against 713, `path_test` 729 against 675 (load 74 s, run 93 s, store 83 s, verify 16 s), `calc_test` 672 against 638, `engine_test` 531 against 504, `std_test` 467 against 428 — the std root's 8 files and 356 declarations loaded and stored in 252 s and K's replay 213 s, against 242 and 183 —, `wire_test` 417 against 423, `derive_test` 317 against 312, `calc_main_test` 315 against 291, `template_test` 304 against 316 (load 113 s, run 99 s, dump 92 s; 80 instances), `k_clients_test` 203 against 201, `tactic_test` 168 against 143, the loader pins' entrypoint 143 against 141 (260 cases against 259), `rows_test` 141 against 126, `matcher_test` 70 against 66, `route2_test` 75 against 69 — the entrypoints beside the 41st a few percent slower, the suite's wall time still `store_test`'s; route 1's native build of the kernel 423 s against 424; the replay 2,453 s at 32.3 GB against 2,524 at 32.3); T0 accepted 57,977, 20 pinned, closures identical. Landing 2 of slice 3.29 holds on the full gate; the slice is closed, its as-built readings (§13 item 63's note) for ratification.**
- **2026-10-13 — pipeline 588 green on `1b93c82`: engine 53 s; corpus 1,463 s at the baseline; v3 4,683 s of a 3 h limit, 98 s over pipeline 585's 4,585 (the export 132 s; the index driver built by route 1 in 11 s and the export's index built in 97 s on the runner; 40 entrypoints with 0 failed in 1,428 s against 1,450 — `store_test` 1,094 s against 1,114 (calc's five leaves stored in 565 s against 564: 467 declarations; replayed in 90 s against 90), parity byte-identical over 31 closures and 135,940 declarations in 852 s against 843 over 135,867 — the 73 declarations more are the landing's kernel growth —, `define_test` 713 against 715, `path_test` 675 against 690 (load 69 s, run 87 s, store 80 s, verify 14 s), `calc_test` 638 against 639, `engine_test` 504 against 498 (the regenerated sidecar byte-identical; 54 of 100 claims), `std_test` 428 against 435 — the std root's 8 files and 356 declarations loaded and stored in 242 s and K's replay 183 s, against 249 and 184 —, `wire_test` 423 against 423, `template_test` 316 against 310 (load 109 s, run 105 s, dump 102 s; 80 instances), `derive_test` 312 against 316, `calc_main_test` 291 against 312, `k_clients_test` 201 against 204, `tactic_test` 143 against 151 (calc's 100 claims under the dependent revert), the loader pins' entrypoint 141 against 143 (259 cases against 254), `rows_test` 126, `matcher_test` 66, `route2_test` 69 against 70; route 1's native build of the kernel 424 s against 419; the replay 2,524 s at 32.3 GB against 2,414 at 32.3 — K's sealed module untouched by the landing, the 110 s the runner's); T0 accepted 57,977, 20 pinned, closures identical. Landing 1 of slice 3.29 holds on the full gate.**
- **2026-10-12 — slice 3.29 landing 2 built: the tactic library's interface, the delegation under `prove`, the example library, the 41st entrypoint, the T10 record — the slice's close-out** (`v3/LANGUAGE.md` §8.4 "as built, landing 2" and the close-out, the slice table, §13 item 63's note; `docs/records/T10-2026-10-12.md`; `docs/LEAN.md`'s rows; the kernel, pins and V3 READMEs). Rule 1: a tactic is `(fn NAME ((goal TEXT) (hints TEXT)) TEXT)`; `tactic.shard`'s `goal_sx` renders the goal as `(goal ((NAME TYPE)…) TARGET (applicable SYM…))` with the one renderer the refusals use, whose binders now read as source writes them; `ev.shard`'s `text_type`/`val_of_text`/`text_of_val` carry text through the entry codec's cells. Rule 2: `reader.shard`'s `Producers` (the sidecar's entries, the loaded programs, the fuel) built by the loader at the three proof sites; `tactic_named` reads `(tactic M.FN)` out of the hints once; `run_tactic` finds the fn in the programs, checks its signature, runs it under `ev` and reads the answer as one block; the block replayed through `tac_prove` before `RDSolved`; refusals `PENDING NAME auto_unsolved fp=N tactic=M.FN: REASON` (unknown name, wrong signature with the signature, a refused step with its goal, no answer, an answer that does not read, several forms, `ev: stuck|exhausted`), two tactics or `(tactic)` refused at the delegation, `(by (tactic …))` `unknown_step`; `load.shard` replays the sidecar only; `prove --measure` runs both producers, `RTactic` → `TACTIC NAME M.FN solved|unsolved`; `prove.shard --fuel N`, `Load.fuel`. Rules 8–9: `v3/examples/tactic/bool_split.shard` (E-first over `kernel/sexpr.shard`: the context's `Bool` locals split by nested `cases` with `(decide)` at the leaves, a reflexive `(<= x x)` or `(= x x)` by an `exact` term), `use_split.shard` (six theorems delegated to it, two beyond the design: `and_assoc`, `eq_self`) with its committed sidecar, `kernel/test/tactic_lib_test.sh` the 41st entrypoint (the replay with no producer run; the regeneration byte-identical — a statement over Init's names alone fingerprints the same in every module; the engine's sidecar for the same statements, the two differing, replaying to the same `ACCEPT` records but for the axiom closure — the proof's: the engine's `arith` on `propext` and `Quot.sound`, the library's `exact` on nothing —, both stored and verified by K alone; the schema-1 copy reported; the hints-changed copy replayed; `--measure` 6 of 6 by each producer); pin `auto_tactic` (260 cases). Gates: the loader pins' entrypoint 260 cases, 0 failed, 58 s; `tactic_test` 110 checks, 0 failed, 52 s; `engine_test` 24 checks, 0 failed, 182 s; parity byte-identical over 31 closures and 136,735 declarations in 313 s; `tactic_lib_test` 30 checks, 0 failed, 280 s; the six touched kernel files clean under the V3 loader; the full suite's 41 entrypoints, 0 failed, in 548 s. Route 1's kernel build, the full replay and the corpus are CI's.
- **2026-10-12 — slice 3.29 landing 1 built: the refusal pointers, the sidecar's schema, the guarded occurrence, the dependent motive, seven pins** (`v3/LANGUAGE.md` §8.4 "as built, landing 1", the slice table, §13 item 63's note; the kernel and pins READMEs; `docs/LEAN.md`'s notation row). Rule 6: `tactic.shard`'s `cases` and `induction` revert the locals whose types mention the eliminated local into the motive (`ctx_dependents`, transitive in the context's order) after the equation hypothesis, the eliminator's result applied back to them, and each arm re-opens them after the fields under their names (`cases_minors`' `depn`); the eliminated local leaves the arm's context unless an equation names it; an induction hypothesis quantifies over them; `wf` unchanged. Rule 5: `(rw LEMMA (occ K EXPECT))` — `rw_occ_check` elaborates EXPECT at the lemma's carrier type and compares it with the instance found by structure, then by `el_unify`; refused `rw_occ_changed` showing both. Rule 4: `loader.shard`'s `sidecar_schema` (2) and `sidecar_split` — a sidecar's first form `(sidecar N)`, no form schema 1 (replayed, recorded `RSidecar` → `SIDECAR FILE schema=N current=2`), a newer schema the load error `sidecar_schema`; `prove.shard` writes `(sidecar 2)` after the header comment whenever it rewrites a file whole and reads the kept entries past the old form; the committed sidecars (`examples/auto`, pin `auto_core`) regenerated from nothing by `prove` (an entry still valid is never rewritten) — the engine closes `auto_core`'s `u` by `rfl` now. Rule 3: `unknown_step`'s two messages and `lean_form`'s notation row name the library route (`(auto (tactic M.FN))`). Pins `by_cases_dependent`, `by_sibling_names`, `by_exact_external`, `by_rw_occ_guard`, `auto_schema_old` (slice 3.22's sidecar kept as the schema-1 artifact), `auto_schema_future`, `auto_hints_changed`; the pins list's duplicated `arith_eq_bare` entry removed. Gates: the loader pins' entrypoint 259 cases, 0 failed, 58 s; `tactic_test` 110 checks, 0 failed, 52 s (calc's 100 claims unchanged under the revert); `engine_test` 24 checks, 0 failed, 179 s (the regenerated sidecar byte-identical but for the fingerprints; 54 of 100 claims); parity byte-identical over 31 closures and 135,940 declarations in 308 s; the full suite's 40 entrypoints, 0 failed, in 530 s. Route 1's kernel build, the full replay and the corpus are CI's.
- **2026-10-12 — item 63 ratified as designed (the user: "This looks good, and the limit is okay for now. Ratify it and continue"): the four leans — the interface data at the boundary, one shot; the producer named in the source's delegation and run by `prove` only; the sidecar's schema form; the guarded occurrence with its expected subterm — and rule 10's limit, a library composing at I's level until route 1 compiles user files. Slice 3.29 landing 1 opens: rules 3–7's kernel half — the refusal pointers, `(sidecar N)`, `(occ K EXPECT)`, dependents reverted under `cases` and `induction`, the pins.** (`v3/LANGUAGE.md` §8.4 slice 3.29 and §13 item 63 marked ratified; the slice table; the V3 README.)
- **2026-10-12 — item 62's as-built departures ratified (the user: "Those departures are fine, ratify them"): landing 1's instance names with the type arguments and `instance_clash`, the declaration-time check at placeholder shapes; landing 2's lifted package field under the taker's name, the written static type checked by its E reading, Lean's binder infos in the structure reader; landing 3's E-first limits and the budget's parameter — slice 3.28 closed on pipeline 585. Slice 3.29 designed at the boundary, on "proceed with the next slice": the tactic library, T10 (`v3/LANGUAGE.md` §8.4 "Slice 3.29", ten rules, two landings; §13 item 63 for ratification). Measured first (three probes, 25 s a run): `cases` keeps a dependent hypothesis speaking of the old local (the zero arm of `(n : Nat) (h : (< 0 n)) ⊢ (<= 1 n)` cannot close — T10's dependent motive is this gap); an unknown step is refused with the roster and no pointer to a producer; `ev`'s entry codec already carries text in and a value out. The leans: a tactic is a `fn` over text (the goal in as one S form with its named context, target and the applicable forms, a block of I out), the kernel replaying the answer as a hand-written block — rejected: the sealed environment crossing into the library, a host callback, a round-trip protocol; the producer named in the source's delegation `(auto (tactic M.FN) HINT…)` and run by `prove` only; the sidecar's schema form `(sidecar N)`, old sidecars reported and newer ones refused; the guarded occurrence `(occ K EXPECT)`. In the rules: dependents reverted into the motive; the refusals with the library pointer; sibling names, external P through `exact` and the settings clause pinned; both producers' solutions replayed, stored and verified by K alone; `v3/examples/tactic/` with `bool_split`, a `Bool` decision procedure as an E library; `tactic_lib_test.sh` the 41st entrypoint; the T10 record.**
- **2026-10-11 — pipeline 585 green on `09cf6e7`: engine 55 s; corpus 1,463 s at the baseline; v3 4,585 s of a 3 h limit, 38 s under pipeline 583's 4,623 (the export 131 s; the index driver built by route 1 in 11 s and the export's index built in 95 s on the runner; 40 entrypoints with 0 failed in 1,450 s against 1,440 — `store_test` 1,114 s against 1,111 (calc's five leaves stored in 564 s against 574: 467 declarations, as at landing 2; replayed in 90 s against 89), parity byte-identical over 31 closures and 135,867 declarations in 843 s against 799 over 135,443 — the 424 declarations more are the landing's kernel growth —, `define_test` 715 against 722, `path_test` 690 against 675, `calc_test` 639 against 636, `engine_test` 498 against 494, `std_test` 435 against 442 — the std root's 8 files and 356 declarations loaded and stored in 249 s and K's replay 184 s, against 249 and 190 —, `wire_test` 423 against 432, `derive_test` 316 against 307, `calc_main_test` 312 against 302, `template_test` 310 against 309 (load 111 s, run 104 s, dump 94 s; 80 instances in the artifact against 15, 81 lambdas and function types in the sources against 17, none in the programs), `k_clients_test` 204 against 189, `tactic_test` 151 against 140, the loader pins' entrypoint 143 against 141 (254 cases against 251), `route2_test` 70 against 68; route 1's native build of the kernel 419 s against 422; the replay 2,414 s at 32.3 GB against 2,459 at 31.3); T0 accepted 57,977, 20 pinned, closures identical. Landing 3 of slice 3.28 holds on the full gate; the slice is closed, its as-built departures (§13 item 62's note) for ratification.**
- **2026-10-11 — slice 3.28 landing 3 built: the budget and the workload, the growth, budget and E-first pins, the E-first instance, `LEAN.md`'s rows, the T2 and T3 record — the slice's close-out** (`v3/LANGUAGE.md` §8.4 "as built, landing 3", the slice table, §13 item 62's note with the close-out; the V3 README; the kernel and pins READMEs; `docs/LEAN.md`; `docs/records/T2-T3-2026-10-11.md`; law §10.5's `LEAN.md` row). Rule 4's budget: `Load.budget` (`instance_budget` 1,024, the driver's `--instance-budget N`), the next instance past it refused `specialization_budget` under the taker's name (`over_budget`, both routes) — a refusal, never a proof. T3's workload: `examples/template/workload.shard`, 64 instances at 64 lifted lambdas in one load (`List.map`, `filter`, `any`, `all` × 16), 509 over (1 2 3) as the entry's last three digits. Pins `template_growth` (`realize_descent` at the declaration), `template_budget` (the harness's `case_budget` 2), `template_e_first`. The E-first instance (rule 3's last sentence): `EITemplate` carries the written binders' E signatures (`EBind`, `tm_ebinds`, `register_template`); the classifier reads a template call by them (`read_template_app`, `read_static_arg`: a name with a program and no type parameters, bare or partially applied) and `take_instances` rewrites the read body — the type arguments by unification, an open one refused, the name by `etable.shard`'s `instance_name_of` (the one formula, which `erasure.shard`'s `instance_name` calls) — answering `COkT` with the demands; the loader's `build_e_demands` builds the program half alone (`build_program` split from `build_instance`, `build_instance_e`, `etype_to_l` by `solve_levels`), records `INSTANCE … equations=` with none. Refused E-first with pointers: a lambda, a bound variable, a registry constant, a polymorphic fn, a template, a constructor, an extern, a package parameter. The entry's `main` takes `List.map` at `digit` itself. `LEAN.md`: four same-as-Lean rows, five refused rows, three shard-only rows. Departures for ratification in item 62's note. Gates: the loader pins' entrypoint 254 cases 0 failed in 53 s (55 s inside the suite); `template_test` 46 checks 0 failed in 101 s (80 instances in the artifact — landing 2's 15, the workload's 64, the entry's E-first one —, 81 lambdas and function types in the sources, none in the programs; load 35 s, run 34 s, dump 33 s); `std_test` 201 checks 0 failed in 152 s (356 declarations stored in 88 s, replayed by K alone in 63 s); `define_test` 12 checks 0 failed in 252 s; parity byte-identical over 31 closures and 135,867 declarations in 311 s (424 more than at landing 2, the kernel's growth); the five touched kernel files clean under the V3 loader; the full suite's 40 entrypoints with 0 failed in 519 s (`store_test` 399 s, `parity_test` 311 s, `define_test` 252 s, `path_test` 234 s, `calc_test` 227 s). Route 1's kernel build, the full replay and the corpus are CI's.**
- **2026-10-11 — pipeline 583 green on `8bbaf58`: engine 55 s; corpus 1,471 s at the baseline; v3 4,623 s of a 3 h limit, 83 s over pipeline 581's 4,540 (the export 131 s; the index driver built by route 1 in 11 s and the export's index built in 95 s on the runner; 40 entrypoints with 0 failed in 1,440 s against 1,424 — `store_test` 1,111 s against 1,084 (calc's five leaves stored in 574 s against 550: 467 declarations against 447, the 20 more being `std/list`'s growth, which the leaves import; replayed in 89 s against 90), parity byte-identical over 31 closures and 135,443 declarations in 799 s against 791 over 135,063 — the 380 declarations more are the landing's kernel growth —, `define_test` 722 against 721, `path_test` 675 against 693, `calc_test` 636 against 650, `engine_test` 494 against 501, `std_test` 442 against 441 — the std root's 8 files and 356 declarations loaded and stored in 249 s and K's replay 190 s, against 242 and 197 over 336 —, `wire_test` 432 against 428, `template_test` 309 s, the new entrypoint (load 103 s, run 105 s, dump 101 s; 15 instances in the artifact, 17 lambdas and function types in the sources, none in the programs), `derive_test` 307 against 318, `calc_main_test` 302 against 304, `k_clients_test` 189 against 194, `tactic_test` 140 against 142, the loader pins' entrypoint 141 against 145 (251 cases against 250), `route2_test` 68 against 67; route 1's native build of the kernel 422 s against 399; the replay 2,459 s at 31.3 GB against 2,414 at 32.3); T0 accepted 57,977, 20 pinned, closures identical. Landing 2 of slice 3.28 holds on the full gate. Landing 3 — rule 4's budget and the workload, the growth and budget pins, the E-first instance, `LEAN.md`'s rows, the T2 and T3 records and the close-out — is open.**
- **2026-10-11 — slice 3.28 landing 2 built: realize templates, packages, the six list templates, the examples and the entrypoint** (`v3/LANGUAGE.md` §8.4 "as built, landing 2"). A supplied `realize` with a function-typed binder is a template (`realize_template`; the written static type checked by its E reading; `TEMPLATE NAME static=f equations=NAME.realize_N`), `template.shard`'s `Template` over a fn's pre-definition or a realize's result type, levels and equations, a realize body taking instances; `std/list`'s `List.map`, `filter`, `foldl`, `foldr`, `any`, `all` templates (`filter`'s rows by `List.filter_cons`); packages — `package_of`, `BStatic 0`, `package_shape`, `erase_package_proj` by the reduction, a package's field passed on (`package_field_shape`); the structure reader with Lean's binder infos and an earlier field applied in a later one's type; two landing-1 defects fixed (a polymorphic recursive template refused at its declaration; `check_template`'s type parameters); the E-first instance deferred to landing 3. `v3/examples/template/` (`add_offset`, `named`, `package`, `template_main` printing `9662014`), `kernel/test/template_test.sh` the 40th entrypoint with the artifact check over `--dump`, pin `template_package`. Gates: the loader pins' entrypoint 251 cases 0 failed in 56 s; `std_test` 201 checks 0 failed in 151 s (the std root's 8 files and 356 declarations, 20 more than at landing 1 — the six templates' twelve equations and `filter`'s two —, stored in 86 s and replayed by K alone in 65 s); `template_test` 40 checks 0 failed in 102 s (15 instances in the artifact, 17 lambdas and function types in the sources, none in the programs; load 35 s, run 33 s, dump 34 s); parity byte-identical over 31 closures and 135,443 declarations in 297 s; the seven touched kernel files clean under the V3 loader (no refusal, no unused open); the full suite's 40 entrypoints in 552 s with one failure, `define_test`'s two stale counts (28 and 20 defined became 34 and 26, the six templates), corrected and rerun green (12 checks 0 failed in 259 s).**
- **2026-10-11 — pipeline 581 green on `e98e504`: engine 56 s; corpus 1,491 s at the baseline; v3 4,540 s of a 3 h limit, 12 s under pipeline 576's 4,552 (the export 132 s; the index driver built by route 1 in 12 s and the export's index built in 96 s on the runner; 39 entrypoints with 0 failed in 1,424 s against 1,407 — `store_test` 1,084 s against 1,076 (calc's five leaves stored in 550 s, their 447 declarations replayed in 90 s), parity byte-identical over 31 closures and 135,063 declarations in 791 s against 758 over 134,028 — the 1,035 declarations more are the landing's kernel growth —, `define_test` 721 against 713, `path_test` 693 against 682, `calc_test` 650 against 644, `engine_test` 501 against 500, `std_test` 441 against 433 — the std root's 8 files and 336 declarations loaded and stored in 242 s and K's replay 197 s, against 240 and 191 —, `wire_test` 428 against 427, `derive_test` 318 against 309, `calc_main_test` 304 against 305, `k_clients_test` 194 against 192, `tactic_test` 142 against 139, the loader pins' entrypoint 145 against 134 (250 cases against 243), `route2_test` 67 against 71; route 1's native build of the kernel 399 s against 419; the replay 2,414 s at 32.3 GB against 2,419 at 32.3); T0 accepted 57,977, 20 pinned, closures identical. Landing 1 of slice 3.28 holds on the full gate; landing 2 — packages, the six list templates, the examples and the entrypoint — is open.**
- **2026-10-11 — slice 3.28 landing 1 built: the static role, the escape
  rule, instances by shape (`LANGUAGE.md` §8.4 "as built, landing 1",
  the slice table; §13 item 62's as-built note).** Rules 1–3 on the
  typed route: `erase.shard`'s `BStatic k`; a `fn` with one is a
  template — checked once at its declaration by an erasure at
  placeholder shapes (the escape rule and the self-call at other shapes
  refused there), defined in K as any `fn`, kept with its roles,
  `TEMPLATE NAME static=f,… equations=…`; a call takes the instance at
  the static arguments' shapes — a name with a program or a registry
  constant, a named partial application with its runtime arguments
  captured, a lambda lifted to `CALLER.lam_N` with the captured locals
  leading (its program and its L definition), a static parameter
  passed on — at closed type arguments, named
  `MODULE.T_flat[_TYPE…].of.SHAPE…`; `kernel/template.shard` builds the
  instance: the program (the template's body at the shapes, the
  captured values its leading parameters), the L definition unfolding
  to the template applied, the equations the template's instantiated and
  proven by them; the loader admits the lifted lambdas, the definitions
  and the equations, adds the programs, records `INSTANCE NAME of=T
  shapes=g/1,… equations=…`, builds nested demands to a fixpoint,
  reuses a name only at the same arguments (`instance_clash`
  otherwise). The escape rule's pointers on both routes. Measured: a
  theorem over an instance by `Eq.refl`, a generic theorem applied at an
  instance, an entry run through instances. Limits stated: a polymorphic
  taker, the E-first instance, the realize template (landing 2). Pins
  `template_lift`, `template_named`, `template_partial`,
  `template_passed`, `template_escape`, `template_transform`. Gates run:
  the loader pins' entrypoint 250 cases 0 failed, parity byte-identical
  over 31 closures and 135,063 declarations in 315 s, the full suite's 39 entrypoints with 0 failed in 568 s
  (`store_test` 437 s, `parity_test` 315 s, `define_test` 273 s, `path_test` 259 s, `calc_test` 251 s). Route 1's kernel build, the full replay and the corpus are CI's.
- **2026-10-11 — item 62 ratified by the user as designed** ("That looks
  reasonable -- ratify it and let's begin implementing."), the four
  leans taken: instances named by their flattened shapes under the
  taking module (`main.List_map.of.main_add_offset_lam_1_1`, the derive
  precedent) rather than by an ordinal; the budget 1,024 instances per
  template per load; six list templates in `std/list` rather than `map`
  alone; a package as a structure with a function-typed field, no new
  form. `LANGUAGE.md`'s item 62 carries it. Landing 1 — rules 1–3 — is
  open.
- **2026-10-10 — slice 3.28 designed: the lambda profile (`LANGUAGE.md`
  §8.4 slice 3.28; §13 item 62, ratified 2026-10-11).** The user's "continue
  to the next slice" after item 61. Law §4.3's profile and §4.1's static
  package as one design under T2 and T3. Measured by four probe files
  under the on-demand loader on the bootstrap (16 s a run): the law's
  `add_offset` is refused before its lambda is reached (`List.map` has
  no realization and cannot be realized: its function binder is
  `realize_signature`); a `fn` with a binder `(f (-> Int Int))` is
  `no_l_identity` on the typed route and `unknown_type: ->` E-first; a
  `def` with the same binder, a `def` whose value is a lambda and a
  theorem over them are accepted; the type-growing recursion is refused
  today as `realize_descent: a recursive call at other type arguments
  (polymorphic recursion)` — T3's first half holds since slice 3.16; the
  old tree has no higher-order list functions and no lambda, so §12
  changes no row. The rules: a function-typed binder is a static
  parameter and its declaration a template (no keyword); the escape rule
  at the erasure's refusal sites with the pointer on both routes;
  instances by shape — the identity and the captured count, never the
  captured values —, a closed lambda lifted to `CALLER.lam_N` with the
  captured locals leading, the instance owned by the taking module and
  named by the derive precedent's flattening, one program per distinct
  tuple per module, its L meaning a generated definition unfolding to
  the template applied and its equations the template's instantiated;
  the shape set closed (a static argument is passed on, never
  transformed) with `realize_descent` extended to static shapes and an
  `instance_budget` of 1,024; packages as structures with function-typed
  fields, never E types, read by whnf to the constructor; six of Init's
  list functions as templates in `std/list`; T2 and T3 as
  `template_test.sh` with the artifact check over `--dump` and nine
  pins; three landings; route 1 unchanged, stated as the limit. The
  leans for the ratifier: the instance naming by flattened shapes over
  an ordinal, the budget's size, six list templates over `map` alone,
  packages with no new form.
- **2026-10-10 — item 61's as-built notes ratified by the user: landing
  3's four readings, landing 4's departure and the two kernel fixes,
  without reservation** ("Looks good, call it ratified. Then let's
  continue to the next slice."); `LANGUAGE.md`'s item 61 carries it.
  Every §13 item through 61 is ratified. Slice 3.27 is closed on
  pipeline 576. The boundary after it is open: slice 3.28 — T2 and T3,
  law §4.3's lambda profile — is the next design, for ratification
  before it is built; T10 is slice 3.29.
- **2026-10-10 — pipeline 576 green on `2ece3aa`: engine 54 s; corpus 1,471 s at the baseline; v3 4,552 s of a 3 h limit, 32 s under pipeline 574's 4,584 (the export 132 s; the index driver built by route 1 in 11 s and the export's index built in 99 s on the runner; 39 entrypoints with 0 failed in 1,407 s against 1,424 — `store_test` 1,076 s against 1,090 (calc's five leaves stored in 552 s, their 447 declarations replayed in 86 s), parity byte-identical over 31 closures and 134,028 declarations in 758 s against 777 over 133,990, `define_test` 713 against 717, `path_test` 682 against 708, `calc_test` 644 against 652, `engine_test` 500 against 511, `std_test` 433 against 456 — the std root's 8 files and 336 declarations loaded and stored in 240 s and K's replay 191 s, against 261 and 193 —, `wire_test` 427 against 449, `derive_test` 309 against 314, `calc_main_test` 305 against 299, `k_clients_test` 192 against 189, `tactic_test` 139 against 150, the loader pins' entrypoint 134 against 135 (243 cases against 235), `route2_test` 71 against 72; route 1's native build of the kernel 419 s against 409; the replay 2,419 s at 32.3 GB against 2,448 at 32.3); T0 accepted 57,977, 20 pinned, closures identical. Landing 4 of slice 3.27 holds on the full gate; the slice's close-out — item 61's as-built readings and the landing-4 departure for ratification — is open.**
- **2026-10-10 — slice 3.27 landing 4 built: `docs/LEAN.md` and T9 small
  (`LANGUAGE.md` §8.4 "as built, landing 4", the slice table; §13 item
  61's as-built note; law §10.5's `LEAN.md` row; the V3 README; the
  kernel and pins READMEs; the top README's doc table).** `docs/LEAN.md`:
  law §5.4's three lists as tables, 92 rows — the same as Lean, refused
  with the reason, the ruling and the pointer, shard-only — with the
  loader's command line, the records and which refusals end a file in
  the header; every row stated from a run (five rows written from recall
  were probed and corrected before the gate). The refusals the lists
  name carry their pointer: `lean_form` for a Lean keyword with no form
  here (`reader.shard`'s table), `inhabited_default` for a `NAME!`
  (`loader.shard`'s `lean_pointer`, before the horizon's refusal),
  `lean_symbol` for `==`/`&&`/`||`, a `!`-prefixed name, `by` in a term
  and Lean's Unicode spellings, `infix_form` for an operator symbol
  where an argument is expected (`elab.shard`'s `tele`). T9 small run
  against the two documents (`docs/records/T9-small-2026-10-10.md`,
  the count agreed before the run): **pass** — 0 interventions, 1
  refusal before success, 7 loader runs, 11 minutes; (ii) measured on
  route 2 since route 1 compiles the kernel's bring-up dialect only
  (a stated departure). Ten gaps reported, each disposed of: seven as
  `LEAN.md` rows; two kernel fixes — a bare zero-parameter `fn` is its
  call on the E-first route as on the typed route (`classify.shard`'s
  `read_esym`, pin `zero_arity_bare`; the `function_value` message
  names the arity) and the `get!` pointer names the Option form this
  pin declares (`List.get?` is no constant of it); one the author's
  misreading of two ordinals. Pins `lean_form`, `lean_notation`,
  `inhabited_default`, `inhabited_default_by`, `infix_form`,
  `lean_symbol`, `lean_by_term`, `zero_arity_bare` (243 cases). Gates run: the 243 loader pins 0 failed in 53 s, `unify_test` 0 failed, `tactic_test` 110 checks (calc's 100 claims) 0 failed in 51 s, parity byte-identical over 31 closures and 134,028 declarations in 293 s, the full suite's 39 entrypoints with 0 failed in 580 s (`store_test` 436 s, `parity_test` 329 s, `define_test` 298 s, `path_test` 283 s, `calc_test` 283 s). Route 1's kernel build, the full replay and the corpus are CI's.
- **2026-10-10 — pipeline 574 green on `d910b9f`: engine 54 s; corpus 1,498 s at the baseline; v3 4,584 s of a 3 h limit, 155 s over pipeline 572's 4,429 (the export 129 s; the index driver built by route 1 in 12 s and the export's index built in 95 s on the runner; 39 entrypoints with 0 failed in 1,424 s against 1,281 — `store_test` 1,090 s against 960 (calc's five leaves stored in 559 s, their 447 declarations replayed in 89 s), parity byte-identical over 31 closures and 133,990 declarations in 777 s against 750, `define_test` 717 against 648, `path_test` 708 against 676, `calc_test` 652 against 609, `engine_test` 511 against 410, `std_test` 456 against 309 — the std root's 8 files and 336 declarations loaded and stored in 261 s and K's replay 193 s, against 174 and 134 for 6 files and 211 —, `wire_test` 449 against 440, `derive_test` 314 against 323, `calc_main_test` 299, `k_clients_test` 189 against 196, `tactic_test` 150 against 116, the loader pins' entrypoint 135 against 138 (235 cases), `route2_test` 72 against 74; route 1's native build of the kernel 409 s against 424; the replay 2,448 s at 32.3 GB against 2,378 at 32.3); T0 accepted 57,977, 20 pinned, closures identical. Landing 3 of slice 3.27 holds on the full gate; landing 4 — `docs/LEAN.md` and T9 small — is open.**
- **2026-10-10 — slice 3.27 landing 3 built: `std/list` and `std/bits`
  (`LANGUAGE.md` §8.4 "as built, landing 3").** Rules 6–9. `v3/std/list.shard`:
  eight of Init's list functions realized by supplied bodies with their
  equations (`take` and `drop` by cases on the count), `Option.getD`
  through its own matcher's definition — the derived view unfolds Lean's
  `NAME.match_N` at its arguments (refused before, accepted after: rule 6's
  measure) —, `List.get` under the dependent match: a `match` generalizes
  the locals whose types mention the scrutinee (Lean's revert; the arm
  re-binds them under the context's names, the matcher's result applied
  back), the nil row refuted by Init's `absurd`, which erases to a stuck
  row (`EAbsurd`/`LAbsurd`; `absurd_needs_init` on the E-first route);
  the realizer's leaf binders ordered after the fields they name; a
  realize's binders E types (`Fin`). The old module's 24 requirements as
  `list_*` theorems citing Init; `sum_append` at `Nat`; the ghost
  refinement — `Sorted` = `Subtype (List.Pairwise Nat.le)`, `sorted_insert`
  keeping `insert_sorted` — with `Subtype` a row of the registry (a row
  constructor's parameters opened unclassified before its fields).
  `v3/std/facts.shard`: the three recurrences at `Nat`, subtraction-free,
  the Euclidean split by 2 (Init's `*_div_two`) joined by `arith` to the
  low bit decided by four cases through Init's `*_mod_two_eq_one`.
  `v3/std/bits.shard`: the old 37 requirements citing Init at `Nat`, `2 ^
  k` one spelling with `Nat.pow` (a table row), the width instances
  through a `Nat.pow` hypothesis, the masks and literal shifts. Calc's
  `examples/calc/list.shard` retired onto the library: seven files
  re-spelled, Init's append laws, one `rfl` equation re-oriented
  (`length t + 1`); the reducer reads a realization's equations through
  the realizing module, the loader marks that `use` needed. Pins
  `dependent_match`, `matcher_view`, `refined_row`, `absurd_e_first`.
  Findings for `LEAN.md`: `arith` reads `Nat.sub` and `2 ^ k` as atoms; an
  exponent is not inferred from a literal; `cases`/`induction` generalize
  the target only; a `fn` signature cannot name `@Subtype T P`; a `def`
  has no E body. Gates run: the 235 loader pins 0 failed in 56 s, `tactic_test` 110 checks (calc's 100 claims) 0 failed in 53 s, `calc_test` byte-identical over 21 inputs in 249 s, `calc_main_test` byte-identical to the model world in 112 s, `engine_test` 24 checks 0 failed (54 of calc's 100 claims unaided, unchanged) in 193 s, `define_test` 12 checks 0 failed in 246 s, `derive_test` 14 checks 0 failed, `unify_test` 0 failed, `k_clients_test` 7 tests in 78 s, `std_test` 181 checks 0 failed in 167 s (the root's 8 files and 336 declarations loaded and stored in 97 s, K's replay 70 s), `path_test` 18 checks 0 failed in 251 s, parity byte-identical over 31 closures and 133,990 declarations in 278 s, the full suite's 39 entrypoints with 0 failed in 559 s (`store_test` 424 s, `parity_test` 301 s, `define_test` 278 s, `calc_test` 265 s, `path_test` 257 s). Route 1's kernel build, the full replay and the corpus are CI's.
- **2026-10-10 — pipeline 572 green on `bd30a48`: engine 55 s; corpus 1,511 s at the baseline; v3 4,429 s of a 3 h limit, 87 s under pipeline 570's 4,516 (the export 129 s; the index driver built by route 1 in 12 s and the export's index built in 93 s on the runner; 39 entrypoints with 0 failed in 1,281 s against 1,256 — `store_test` 960 s against 968, parity byte-identical over 31 closures and 133,785 declarations in 750 s against 751, `path_test` 676 against 681, `define_test` 648 against 656, `calc_test` 609 against 611, `wire_test` 440 against 436, `engine_test` 410 against 405, `derive_test` 323 against 317, `std_test` 309 s new — the std root loaded and stored in 174 s and K's replay of its 211 declarations 134 s, against 69 and 48 on the dev box —, `k_clients_test` 196 against 193, the loader pins' entrypoint 138 against 130 (one case more, `arith_eq_bare`, under `Int.lt_mul_ediv_self_add`'s demand), `tactic_test` 116 against 113, `route2_test` 74 against 76; route 1's native build of the kernel 424 s; the replay 2,378 s at 32.3 GB against 2,544 at 31.5); T0 accepted 57,977, 20 pinned, closures identical. Landing 2 of slice 3.27 holds on the full gate; landing 3 — `std/list` and `std/bits`, calc's `list.shard` retired onto the library — is open.**
- **2026-10-10 — slice 3.27 landing 2 built: the records (`LANGUAGE.md`
  §8.4 "as built, landing 2"; law §10.3 row 8 corrected in place).**
  `v3/std/facts.shard`: the twelve former axioms as theorems by Init's
  names (horizon `Nat.shiftRight_zero`, 1,440 declarations, 38 s).
  `v3/std/migration.shard`: one theorem per row of law §10.3 (twenty and
  a `def`), two rows corrected on the pin — `List.get?` is no constant of
  v4.33.1 (`l[i]?` is `GetElem?.getElem?`, `List.get?Internal` by
  definition), `String` is a structure over its UTF-8 `ByteArray` with
  its validity proof — and the records' evidence, thirty-nine theorems for
  `order`, `nat`, `div`, `arith` (horizon `List.getD`, 1,706 declarations,
  41 s; `String.toList_ofList` measured at 3,282 declarations and 277 s,
  cited in the record, not loaded). `v3/std/README.md`: the six
  per-interface records, `list`'s and `bits`' Init names checked against
  the index, their files at landing 3. `kernel/test/std_test.sh`, the 39th
  entrypoint: six files in one process, 211 declarations stored and
  replayed by K alone, 76 checks. `v3/std/list.shard` (V3's own
  `List.sum`) deleted; `define_test`'s expectation on it removed. Fixed
  with pins: the instance table's dispatch never reached the any-type row
  for `Int`/`Nat`/`Bool`/`List` carriers (`BEq Int` had no instance;
  `sp_inst_typed` falls through); `arith`'s equation path equated two
  evaluation terms with coefficient lists of different lengths — K refused
  the ill-typed term (`app_type_mismatch`), `lin_eq` compares entry for
  entry, the differing case proved as two inequalities (pin
  `arith_eq_bare`). For `LEAN.md`: bare `true`/`some`/`cons` are patterns'
  only, `==`/`&&`/`||` are no operator symbols. Gates: the 231 loader pins 0 failed in 56 s, `std_test` 76 checks 0 failed in 117 s (the root loaded and stored in 69 s, K's replay 48 s; 158 s inside the suite), `tactic_test` 110 checks (calc's 100 claims) 0 failed in 43 s, `engine_test` 24 checks 0 failed (54 of calc's 100 claims unaided, unchanged), `define_test` 12 checks 0 failed, `derive_test` 14 checks, `unify_test` 0 failed, `k_clients_test` 7 tests in 70 s, parity byte-identical over 31 closures and 133,785 declarations in 292 s, the full suite's 39 entrypoints with 0 failed in 640 s (`store_test` 505 s, `parity_test` 364 s, `path_test` 316 s, `calc_test` 300 s, `define_test` 297 s). Route 1's kernel build, the full replay and the corpus are CI's.
- **2026-10-10 — pipeline 570 green on `52b960c`: engine 56 s; corpus 1,494 s at the baseline; v3 4,516 s of a 3 h limit, 59 s over pipeline 565's 4,457 (the export 129 s; the index driver built by route 1 in 11 s and the export's index built in 95 s on the runner; 38 entrypoints with 0 failed in 1,256 s against 1,253 — `store_test` 968 s against 962, parity byte-identical over 31 closures and 133,762 declarations in 751 s against 743, `path_test` 681 against 691, `define_test` 656 against 670, `calc_test` 611 against 603, `wire_test` 436 against 453, `engine_test` 405 against 409, `derive_test` 317 against 311, `k_clients_test` 193 against 196, the loader pins' entrypoint 130 against 106 (five cases more, and every demand 39 declarations larger by `sp_kit`), `tactic_test` 113, `route2_test` 76 against 73; route 1's native build of the kernel 414 s; the replay 2,544 s at 31.5 GB against 2,479 at 32.3); T0 accepted 57,977, 20 pinned, closures identical. Landing 1 of slice 3.27 holds on the full gate; landing 2 — the records — is open.**
- **2026-10-10 — slice 3.27 landing 1 built: the seams (`LANGUAGE.md`
  §8.4 "as built, landing 1").** Rule 2 as a table, not a transparency:
  slice 3.21's reading of a class method under Init's instance moved
  out of `arith.shard` into `kernel/spelling.shard` with the bitwise
  and append rows, and the unifier reads both sides through it before
  K's whnf — the probe's `witness_open` (`apply Nat.zero_and` on
  `Nat.land 0 b = 0`) closes, K's whnf having computed the closed side
  to `0`; `rw`/`simp_only` needed nothing, every goal and lemma having
  entered in the one spelling since 3.21. Rule 3: `sp_instance`, the
  table of the pin's instance constants at `Nat`, `Int`, `Bool`, `List`
  and the compositions (`Zero.ofOfNat0` over `instOfNat 0`, the `instH*`
  lifts, `instBEqOfDecidableEq`), asked for an application's binders,
  at a declaration's finish and at a tactic opening's premises;
  `min`/`max` as `Min.min`/`Max.max`; `sp_kit` in the loader's demand
  (the table's constants are no token). Rule 4: `/` at `Int` is
  `Int.ediv`. Rule 5: `Fx.above` from the walk, `above_horizon` with the
  ordinal, the horizon and the import to write, the name matched as a
  token of the message (a tactic step wraps it). Cleanup: `mk_app_n`
  to expr.shard. Five pins, one unify case. Gates: the 230 loader pins 0 failed in 54 s, `unify_test` 0 failed, `tactic_test` 110 checks (calc's 100 claims) 0 failed in 44 s, `define_test` 12 checks 0 failed in 241 s, `engine_test` 24 checks 0 failed (54 of calc's 100 claims unaided, unchanged), `derive_test` 14 checks, `wire_test` 15 checks, `k_clients_test` 7 tests in 72 s, parity byte-identical over 31 closures and 133,762 declarations in 350 s, the full suite's 38 entrypoints with 0 failed in 553 s (under contention with the targeted gates; `store_test` 400 s, `parity_test` 333 s, `define_test` 308 s, `calc_test` 298 s, `path_test` 289 s). Route 1's kernel build, the full replay and the corpus are CI's.
- **2026-10-10 — item 61 ratified by the user, as designed.** "Your
  leans sound good for calc's list and moving the tool to tier-5. Let's
  ratify the design as proposed and proceed." Both leans stand: calc's
  `examples/calc/list.shard` retires onto `std/list` at landing 3, its
  claims re-spelled and its sidecar regenerated; the migration tool's
  tier 0 moves to phase 5 with the records as its calibration data, the
  departure from law §12.3 recorded in item 61(l). Every item through
  61 is ratified. Landing 1 (the seams) opens.
- **2026-10-09 — slice 3.27 designed: the library arc (`LANGUAGE.md`
  §8.4 slice 3.27; §13 item 61, ratified 2026-10-10).**
  The user's "continue with the next arc" after items 59 and 60. What
  phase 3 still owes (law §12.4) gathered into one design under the
  user's 3.25-boundary principle — no duplication of Init. Measured by
  three probe files under the on-demand loader on the bootstrap:
  twelve of the fifteen former axioms of `kernel/facts.shard` are
  theorems by Init's names in twelve lines (47 s, 1,395 declarations
  admitted; `Int.emod_nonneg`, `Int.emod_lt_of_pos`, `Int.emod_def`
  under `arith`, `Int.ediv_emod_unique`'s two halves, `Int.mul_comm`,
  `Int.mul_assoc`, `Int.mul_add`, `Nat.shiftLeft_zero`/`_succ`,
  `Nat.shiftRight_zero`/`_succ_inside`); six of Init's list functions
  realized by supplied bodies with their equations (`take`/`drop` by
  `cases` on the count, which Init recurses on first) and seven of its
  list theorems cited (29 s, 1,385 declarations); five of `std/bits`'
  statements by Init's names (54 s, 1,887). Three seams found: `/` at
  `Int` elaborates to `Int.div`, absent from the pin (`Int.ediv` is
  the Euclidean one the law names); a lemma stated through Init's
  instance (`0 &&& x`) cannot be `apply`d to V3's spelling
  (`witness_open`: the argument under the instance is not determined)
  while `exact` goes through K's conversion; a name above the horizon
  is `unknown_constant`, so the author needs the ordinal
  (`Nat.zero_and` 23,368, `List.take_cons` 54,202, `Nat.min` 10,228).
  Beside them: `List.sum` at `Int` is `instance_needed`, and
  `List.length_take` states `min`, which V3 cannot write. Eleven
  rules: Init is the library; one spelling at the unifier and the
  matcher; instances by table; `/` is `Int.ediv`; the horizon
  refusal's pointer; supplied bodies with equations by `rfl` or
  `cases`, the matcher through its definition; the dependent match at
  `List.get`; the six modules as records (`std/order`, `nat`, `div`,
  `arith` have no file); the fifteen in `std/facts.shard`; `LEAN.md`;
  T9 small. Four landings: the seams; the records and `std_test.sh`;
  `std/list` and `std/bits` with calc's `list.shard` retired onto the
  library; `LEAN.md` and T9. T2/T3 and T10 deferred to slices 3.28 and
  3.29 as language slices; the migration tool's tier 0 to phase 5 with
  these records as its calibration, a stated departure from law §12.3.
  The probe files were scratch and are not committed.
- **2026-10-09 — items 59 and 60 ratified by the user, as amended at slice
  3.26 landing 3** (the shared Init load's four decisions on today's
  carrier, Init on demand's six), without reservation; `LANGUAGE.md`'s
  status block carries them. Every §13 item through 60 is ratified; item
  57 stands ratified and retired. The boundary after slice 3.26 is open:
  the library arc's design slice is next, for ratification before it is
  built.
- **2026-10-09 — pipeline 565 green on `3254941`: engine 52 s; corpus 1,500 s at the baseline; v3 4,457 s of a 3 h limit, 36 s over pipeline 563's 4,421 (the export 133 s; the index driver built by route 1 in 11 s and the export's index built in 98 s on the runner; 38 entrypoints with 0 failed in 1,253 s against 1,255 — the suite's wall clock unchanged, as the drop predicts: at 563 the receipt already had no writer, so every load checked its closure there too; `store_test` 962 s against 967, parity byte-identical over 31 closures and 133,392 declarations in 743 s against 745, `path_test` 691 against 667, `define_test` 670 against 657, `calc_test` 603 against 600, `wire_test` 453 against 436, `engine_test` 409 against 392, `derive_test` 311 against 318, `k_clients_test` 196, the loader pins' entrypoint 106 against 109, `route2_test` 73; route 1's native build of the kernel 416 s — `t0.shard` changed, its receipt writer gone; the replay 2,479 s at 32.3 GB); T0 accepted 57,977, 20 pinned, closures identical. Landing 3 of slice 3.26 holds on the full gate; the slice is closed. Items 59 and 60, as amended, await ratification.**
- **2026-10-09 — slice 3.26 landing 3: the receipt dropped, the slice closed (`LANGUAGE.md` §8.4 "as built, landing 3"; §13 item 57 retired, items 59 and 60 amended; the user's ruling on the measurement).** With every load checking the closure it cites (landing 2), the suite's entrypoints of many short loads doubled on the runner at pipeline 563 (`wire_test` 155 s to 436, `define_test` 337 to 657, `engine_test` 267 to 392; `wire_test` 65 s to 150 locally), each process checking its demand where the receipt had admitted the fixture's prefix; a hand-written receipt over the export admitted a demand in nine tenths to four fifths of the checked time (calc 9 s against 10 s, `std/bytes` 20 s against 25 s) — the walk on the bootstrap and K's ingestion are the cost, and a receipt skips neither; its only writer would have been the full replay, which CI runs after the suite. The user ruled the receipt dropped. Deleted: `load.shard --init-receipt` and `prove.shard`'s, the loader's receipt (`Load.receipt`, `receipt_mode`, `receipt_parse`, `load_with_receipt`; a driver starts from `ld_start`), `t0.shard --receipt` and its writer, the loader pins test's flag, `v3/test.sh`'s `INIT_RECEIPT` and the eight shell tests' pass-through; `init_receipt_stale` is no refusal. K's admit mode (`k/add.shard` `admit_decl_pinning`, `run_admitting`) stays for the loader's second admission of records it checked into K's run of Init a moment ago (`initload.shard` `od_admit`), its comments saying so. Slice 3.23's text and §13 item 57 stand marked retired; items 59(a), 59(d) and 60(b), (e), (f) are amended in place. The lever that remains is item 60(f)'s loader native. Gates: the 225 loader pins 0 failed in 46 s (43 s inside the suite), `loader_test` 0 failed in 8 s, `t0_fixture_test` and `route2_test` byte-identical (route 2 in 26 s), the full suite's 38 entrypoints with 0 failed in 620 s against 455 s at landing 2 — every entrypoint about a third slower while the pins' entrypoint held at 43 s against 42 s, so contention on the machine, the per-load cost unchanged (parity byte-identical over 31 closures and 133,392 declarations in 354 s; `store_test` 484 s, the suite's wall clock; `path_test` 314 s; `define_test` 303 s). `t0.shard` is route 1's: the kernel build, the full replay and the corpus are CI's.
- **2026-10-09 — pipeline 563 green on `615a98b`: engine 52 s; corpus 1,487 s at the baseline; v3 4,421 s of a 3 h limit, 205 s shorter than pipeline 561's 4,626 (the export 132 s; the index driver built by route 1 in 12 s and the export's index built in 97 s on the runner; 38 entrypoints with 0 failed in 1,255 s against 1,518 — `store_test` 967 s against 1,160, `path_test` 667 against 788, parity byte-identical over 31 closures and 133,465 declarations in 745 s, `calc_test` 600 against 645, `index_test` under 30 s against 164; the entrypoints made of many short loads rose, each process checking its demand's closure where the receipt admitted the fixture's prefix: `define_test` 657 s against 337, `wire_test` 436 against 155 (locally 150 against 65), `engine_test` 392 against 267, `derive_test` 318 against 204, the loader pins' entrypoint 109 against 89; route 1's native build of the kernel 419 s; the replay 2,438 s at 32.4 GB); T0 accepted 57,977, 20 pinned, closures identical. Landing 2 of slice 3.26 holds on the full gate; landing 3 is open, and the checked-against-admitted difference above is the receipt's measured value for it.**
- **2026-10-09 — slice 3.26 landing 2 built: the loader on demand, the
  fixtures deleted (`LANGUAGE.md` §8.4 "as built, landing 2"; item 60
  amended).** `kernel/initload.shard`: the export opened by its index,
  names by binary search, the closure of the demanded records walked by
  range reads — blocks highest first, each read once and scanned from
  its record line back, a cited constant resolved by its name id through
  a third table, the declaration table, added to the index for it — and
  fed to K in export order, checked; what K was fed persists. The loader
  demands, after a file's directives, every token's candidates
  (resolution's, pruned of prefixes the export declares nothing under),
  the gates' kits — each gate's own now — and an inductive name's
  satellites, below the horizon; K's run of Init alone persists in the
  Init state and is carried across the roots of a process, each demand
  checked into it once and admitted into the loader's environment, which
  holds the natives too (the first form, re-rooting from the state before
  any demand, re-checked every pin's closure: 1,761 s against 42 s).
  `name_taken` from the table under the file's identity; the release gate
  demands what the store cites and refuses a stored name the export
  declares (`TAKEN`). Found running it: the `List.sum` collision item
  59(c) was reopened on is no collision — `v3/std/list.shard` declares
  `std.list.List.sum`, every declared name carrying the module path
  (§3.1 says so), K holds both — and the live shape is a module named as
  an Init namespace (`registry_native`'s `Bool`): the ledger corrected in
  item 59(c), slice 3.25's text and here; three names the kits lacked
  found by refusal (`Lean.Omega.Int.mul_congr`; a token's `@` sigil; the
  `init_not_found` pin asking for a name the fixture lacked, not the
  export). Measured: the loads of calc's five files, auto, bytes, list,
  derive and path 10–34 s each on the bootstrap, checked, in parallel;
  the loader pins 225 cases in 42 s checked (33 s admitted at 3.25);
  `index_test` 9 s against 125 s; the index one file of 19.2 MB, built
  natively in 36 s. Gates: the full suite's 38 entrypoints with 0 failed in 455 s against 613 s at landing 1 (parity byte-identical over 31 closures and 133,465 declarations in 267 s; `store_test` 350 s, the suite's wall clock; `path_test` 228 s; `define_test` 230 s; `calc_test` 215 s; `k_clients_test` 69 s; the loader pins' entrypoint 42 s). None of the files route 1 compiles
  changed but host.shard at landing 1; the full replay and the corpus are
  CI's.
- **2026-10-09 — pipeline 561 green on `97009cc`: engine 437 s — the native engine rebuilt, `tools/codegen/rt.h` and the codegen table being in its stamp and both carrying `read_range` now; corpus 1,455 s at the baseline; v3 4,626 s of a 3 h limit, 16 s over pipeline 559's 4,610 (38 entrypoints with 0 failed in 1,518 s; `index_test` 164 s, the fixtures' index built in 70 s on the runner; `store_test` 1,160 s; `path_test` 788 s; parity byte-identical over 31 closures and 130,762 declarations in 701 s; `engine_test` 267 s; route 1's native build of the kernel 422 s; the export 122 s; the replay 2,493 s at 32.4 GB); T0 accepted 57,977, 20 pinned, closures identical. Landing 1 of slice 3.26 holds on the full gate; landings 2 and 3 are open.**
- **2026-10-09 — slice 3.26 ruled and landing 1 built: Init on demand
  (`LANGUAGE.md` §8.4 slice 3.26; §13 item 60).** At the boundary after
  3.25 the library arc's design was discussed: citing Init wherever
  Init has the statement costs a far lemma's position in the export
  under the prefix stream, and the closure-fixture cutter proposed for
  it was rejected by the user as a workaround ("What is stopping us
  from ingesting the whole thing in a timely manner, or caching it such
  that we only need to ingest it once in a given environment?"). The
  answer measured: the loader parses the export in shard at 0.8 MB/s;
  the whole export checked under the interpreter is 76 min and 8 GB;
  the receipt caches K's verdict, not the environment. Three designs
  weighed — an environment image (Lean's `.olean`), a session server,
  demand-driven loading from an indexed export — and the third ruled
  ("C sounds like the reasonable approach, compounded with faster base
  operation once we regularly compile K"), after a kill-test on the
  loading unit: a declaration's line-level closure is under one
  percent of the export (`Int.ediv_emod_unique` 690 records / 35,076
  lines; `Nat.and_comm` 746 / 42,468; `List.sum` 25 / 686), a
  block-level closure a third of it (15,482 blocks / 2,240,409 lines
  for the same lemma; a shared node lives in whichever block first
  needed it); a theorem admitted by its statement alone would be five
  to ten times smaller again (143 / 3,098) and is deferred as a K
  change. Landing 1: `kernel/index.shard` and `initindex.shard` (the
  record table and the name table, built by one pass, read by range,
  verified by a second pass), `host.shard`'s `read_range` in the
  bootstrap and the chain's runtime, `test/init_index.sh`,
  `test/index_test.sh` (the 38th entrypoint). The export's 57,977
  records and 59,433 names (347,714,179 bytes) index in 952 s and verify in
  866 s on the bootstrap; the fixtures' in 46 s and 63 s. Compiled by
  route 1 (`v3/build.sh v3/kernel/initindex.shard v3/bin/initindex`,
  4 s; the pass's state rewritten as a plain type with accessors when the
  chain refused the record sugar) the driver indexes the export in 68 s
  and verifies it in 71 s, the tables byte-identical to the bootstrap's. Found: the
  export's keys are alphabetical within a line, so an id is found by
  its key; the driver's first parity run refused a pattern variable
  shadowing `n2` and an unused open, fixed. Gates: `index_test` 10
  checks in 125 s, parity byte-identical over 31 closures and 130,579
  declarations in 338 s, the full suite's 38 entrypoints with 0 failed
  in 613 s.
- **2026-10-09 — pipeline 559 green on `d0ee4f3`: engine 53 s; corpus 1,448 s at the baseline; v3 4,610 s of a 3 h limit — 891 s shorter than pipeline 556's 5,501 s (37 entrypoints with 0 failed in 1,491 s; the loader pins' entrypoint 87 s with 225 cases against 2,227 s; `store_test` 1,157 s, the suite's wall clock now; `path_test` 774 s; parity byte-identical over 30 closures and 130,551 declarations in 704 s; `engine_test` 254 s; route 1's native build of the kernel; the replay 2,498 s at 32.4 GB); T0 accepted 57,977, 20 pinned, closures identical. Slice 3.25 is closed.**
- **2026-10-09 — slice 3.25 built: the shared Init load (`LANGUAGE.md`
  §8.4 slice 3.25; §13 item 59).** Ruled at the boundary after R75 and
  R76 (item 57's stated lever, the user: "Agreed with A first, then C's
  design"). `loader.shard` `init_all` streams the export to its end and
  `ld_reroot` starts a fresh root load from a load's Init state; the
  loader pins' entrypoint streams the two fixtures once and runs its 225
  cases on it — 33 s under the receipt from 783 s (the stream 18 s, the
  cases 15 s), 57 s with the stream checked. The shared environment
  exposed three pins that had leaned on the stream stopping short, and
  the rule behind them: a module's Init horizon is the only measure of
  what it sees — every availability gate now asks the scope (`elab.shard`
  `el_sees`; `arith`'s kit, the `WellFounded` kit, a `"…"` literal's
  `String`), an `(import Init NAME)` is answered from the ordinal table
  with no fall-back to the stream's count — and Init's names are Init's:
  a declaration named as an Init constant is refused wherever the
  environment holds it (`name_taken`; K's `already_declared`), which the
  release gate always does; a lazy driver that stopped short accepts
  what the gate refuses, the one disagreement, stated. `registry_native`
  is that refusal now; the finding it pinned (a registry row never
  speaks for a native declaration bearing its name) stands for a lazy
  driver and `define_test.sh` reads that dump. Drivers stream lazily as
  before; `path_test`'s four loads are four processes, out of reach.
  Found the same day, reading the export for the library arc's design,
  and taken for a live case: `v3/std/list.shard` declares `(fn List.sum
  …)` and `List.sum` is an Init definition at export line 882,911 —
  item 59(c)'s rejected name index was reopened on it. Corrected at
  slice 3.26 landing 2 by running it: the file declares
  `std.list.List.sum`, every declared name carrying the module path
  (§3.1), and K holds both; the index stands on rule 5 instead.
- **2026-10-09 — items 57 and 58 ratified by the user** (the Init cache's
  five decisions, the connected path's three), without reservation;
  `LANGUAGE.md`'s status block carries them. Every §13 item through 58 is
  ratified. The boundary after R75 and R76 is open: the next slice is to be
  ruled.
- **2026-10-09 — pipeline 556 green on `d0dfb7b`: engine 56 s; corpus 1,487 s at the baseline; v3 5,501 s of a 3 h limit (37 entrypoints with 0 failed; `path_test` 18 checks in 792 s — the path composes on the runner: load 52 s, run 51 s, store 50 s, verify by K alone 134 s —; the loader pins' entrypoint 2,227 s with 225 cases; `store_test` 1,163 s; `engine_test` 265 s; parity byte-identical over 30 closures and 130,489 declarations in 697 s; route 1's native build of the kernel; the replay 2,534 s at 32.4 GB); T0 accepted 57,977, 20 pinned, closures identical. Slice 3.24 is closed.**
- **2026-10-09 — slice 3.24 built: the connected path (`LANGUAGE.md`
  §8.4 slice 3.24; §13 item 58; GPT-6's R76, §4.11).** The second
  boundary item after 3.22, ruled as one slice. `(dif h C T F)` is the
  dependent if with its hypothesis named (`kernel/elab.shard`
  `elab_dif`: `dite C dec (λ h. T) (λ h. F)`), the one phase-3 joint of
  law §12.4's path that was missing (T1's branch-local proof, carried
  since slice 7; slice 3.18's `Fin` row removed the value-parameter
  obstacle, the source form remained). `v3/examples/path/` is the path
  on one program: `List.length` and `List.get?Internal` realized by
  supplied bodies with their equations proved (the index's structural
  recursion as an `if`, its equation by cases in the `equations`
  clause); `at xs i : Option (Fin (List.length xs))` built from the
  branch's proof; `pick` with a dependent parameter type; `at_some`,
  `at_none` through I by `dif_pos`/`dif_neg`; the entry on a raw
  argument; 56 declarations stored and accepted by K alone.
  `kernel/test/path_test.sh` (18 checks) runs the path in order and
  breaks each joint by name: a wrong body (`realize_2` refused
  `conversion`), the proof outside its branch (`unknown_constant`) or
  dropped (`type_mismatch`), an order line renamed (MISTIED), a stored
  proof replaced (REJECT), a non-number at the entry (exit 6). The
  cost under the Init receipt: load 18 s, run 23 s, store 18 s,
  verify 47 s — the Init stream four times, the path's own work
  seconds. Found, open: the dependent match (a local whose type
  mentions the scrutinee generalized), which `List.get`'s own
  realization needs; a `(view)` of `Option.getD` refused for its
  matcher. Pins `dif_ok`, `dif_type`, `dif_needs_init`, `dif_scope`.
  Gates: the 225 loader pins (four new) byte-identical in 783 s under the receipt, parity byte-identical over 30 closures and 130,489 declarations in 265 s, K's clients 7 of 7, `path_test` 18 checks in 279 s, the full suite's 37 entrypoints 0 failed (855 s).
- **2026-10-09 — pipeline 554 green on `7ad18a6`: engine 55 s; corpus 1,479 s at the baseline; v3 5,455 s of a 3 h limit — 939 s shorter than pipeline 552's 6,394 s: 36 entrypoints with 0 failed, the loader pins' entrypoint 2,188 s against 3,327 s under the receipt, `store_test` 1,156 s against 1,349 s, `engine_test` 263 s against 357 s, parity byte-identical over 30 closures and 130,445 declarations in 712 s, route 1's native build of the kernel, the replay 2,518 s at 32.4 GB; T0 accepted 57,977, 20 pinned, closures identical. The pins' entrypoint is the suite's wall clock still. Slice 3.23 is closed.**
- **2026-10-08 — slice 3.23 built: the Init cache (`LANGUAGE.md` §8.4
  slice 3.23; §13 item 57; GPT-6's R75, §4.11).** Ruled at the boundary
  after 3.22 (items 54–56 ratified the same day): the feedback-cost
  lever first and shallow, the connected path (R76) after it. The cost
  measured over the two fixture chunks (3,523 declarations, 14 MB):
  reading and parsing 9.2 s, the tables 13.1 s, K's check of every
  declaration 47.2 s, the check skipped 17.2 s — the check is 72
  percent and every on-disk form must build the same tables, so a
  serialized environment was rejected for a receipt over the export.
  `t0.shard --receipt FILE` writes `PATH BYTES` per chunk after a run
  that accepted every record; `k/add.shard`'s `admit_decl_pinning`
  inserts an axiom, definition, theorem or opaque without the typing
  judgments (inductive blocks and the quotient checked as always; the
  watermark and the pins as on the checked path), the import's run
  carrying the mode; `load.shard --init-receipt FILE` (and
  `prove.shard`'s, the loader pins test's) streams a listed chunk in
  admit mode and refuses one not listed at its byte count
  (`init_receipt_stale`); `kernel/test/init_receipt.sh` keeps
  `v3/.cache/init.receipt` current against a hash of K's sources and
  the fixture, `v3/test.sh` running it once and exporting
  `INIT_RECEIPT`; `verify_release` and `t0_full.sh` take no receipt.
  The auto example loads in 8.3 s against 13.7 s, the records
  byte-identical; a wrong byte count refuses the load, an unreadable
  receipt exits 2. The 221 loader pins pass byte-identical under the receipt in 809 s on the development machine (beside parity and the define test; 1,361 s without the receipt beside the full suite — the clean comparison is pipeline 553's against 552's 3,327 s on the runner); parity byte-identical over 30 closures and 130,445 declarations; route 1's native driver, rebuilt locally, ties the interpreter on the first chunk and writes the receipt; the full suite's 36 entrypoints 0 failed in 927 s, the pins entrypoint its wall clock still.
- **2026-10-08 — pipeline 552 green on `8abe8c5`: slice 3.22's landing 3 on
  the full gate; slice 3.22 closed.** engine 57 s; corpus 1,470 s at the baseline; v3 6,394 s: 36 entrypoints with 0 failed, `store_test` 18 checks in 1,349 s (the example's 23 declarations stored once and accepted by K alone, the four tamper cases refused, calc's 376 declarations stored into `v3/release/calc` in 462 s and every one accepted by K alone in 136 s), `engine_test` 24 checks in 357 s, `tactic_test` 110 checks, `define_test` 12 checks, `derive_test` 14 checks, `wire_test` 15 checks, calc's host program byte-identical to the model world, parity byte-identical over 30 closures and 130,293 declarations in 693 s, route 1's native build of the kernel (`OK: v3/bin/t0`), the fixture byte-tie identical, replay 2,449 s at 32.2 GB peak, T0 accepted 57,977 with 0 rejected, closures identical to the oracle, all 20 accelerator candidates pinned; the release store uploaded as the job's artifact (379 files under `v3/release/`, 1.2 MB zipped). The v3 job is 40 s shorter than pipeline 550's; the loader pins' entrypoint (3,327 s) is the suite's wall clock still, and `store_test` runs inside it.
- **2026-10-08 — slice 3.22, landing 3 built: the store and
  `verify_release` (`LANGUAGE.md` §8.4's rule 7 and as-built; law
  §7.5).** `kernel/store.shard` renders an admitted constant of K's
  environment as the records K ingests (lean4export's NDJSON, what
  `k/import.shard` reads), the tables shared by strict structural
  identity, an inductive type as its block with K's recursors;
  `load.shard --store DIR` writes a clean load's constants in admission
  order, one file per declaration by content fingerprint (`FP-K` past
  a collision), `order` appended and files shared across loads.
  `kernel/verify_release.shard` is K, the host and the JSON reader
  alone: the Init chunks as one stream, each store file as its own
  stream over the environment so far, each accepted declaration tied
  to its order line's name and fingerprint. `kernel/test/store_test.sh`:
  the example's 23 declarations stored once and accepted by K alone; a
  forged proof REJECTED, a broken record MALFORMED, a missing file
  MISSING, a renamed declaration MISTIED; calc's five leaves stored
  into `v3/release/calc` and replayed — 376 declarations, every one
  accepted, 3.5 MB, the v3 job's artifact. Found: three matchers of
  the example share a fingerprint (one term, three names); with the
  import's tables persisting across files a dropped record resolved to
  an earlier file's node and a dropped name record landed a theorem
  under a shorter name — per-file streams and the tie answer both.
  Not stored: the implementation forks of views (open until a consumer
  needs it). Gates: 221 loader pins, `store_test` (18 checks in 495 s: the example's 23 declarations stored once, shared on a second load and accepted by K alone; a forged proof REJECTED, a broken record MALFORMED, a missing file MISSING, a renamed declaration MISTIED; calc's 376 declarations stored into `v3/release/calc` and every one accepted by K alone in 54 s), `engine_test` (24 checks: 54 of 100), `tactic_test` (110 checks: calc's 100 claims), `define_test` (12 checks), `derive_test` (14 checks), `wire_test` (15 checks), calc byte-identical over 21 inputs and its host program byte-identical to the model world, parity byte-identical over 30 closures and 130,293 declarations in 270 s (`prove.shard` and `verify_release.shard` added as roots), route 2, K's clients, the full suite's 36 entrypoints with 0 failed. None of the files route 1 compiles changed; the full replay and the corpus are CI's.
- **2026-10-07 — pipeline 550 green on `c98c2cd`: slice 3.22's landing 2 on
  the full gate.** engine 54 s; corpus 1,478 s at the baseline; v3 6,434 s: 35 entrypoints with 0 failed, `engine_test` 24 checks in 366 s (the example's eleven theorems from the sidecar, the regeneration byte-identical, 54 of calc's 100 claims closed unaided), `tactic_test` 110 checks, `define_test` 12 checks, `derive_test` 14 checks, `wire_test` 15 checks, calc's host program byte-identical to the model world, parity byte-identical over 28 closures and 116,957 declarations in 619 s, route 1's native build of the kernel (`OK: v3/bin/t0`), the fixture byte-tie identical, replay 2,504 s at 31.4 GB peak, T0 accepted 57,977 with 0 rejected, closures identical to the oracle, all 20 accelerator candidates pinned. Landing 2 is closed. The v3 job is 103 s shorter than pipeline 548's; the loader pins' entrypoint (3,318 s) is the suite's wall clock still, and `engine_test` runs inside it.
- **2026-10-07 — slice 3.22, landing 2 built: the engine and `prove`
  (`LANGUAGE.md` §8.4's rules 5–6 and as-built; law §7.3, §7.5).**
  `kernel/engine.shard` composes `tac_open`, `applicable` and
  `tac_step` into a ladder, every node confirmed as it is chosen;
  `kernel/prove.shard` loads under the mode `solve`, replays each
  block the engine finds before accepting it, and rewrites the
  sidecars whole; `--measure` runs the engine on every hand-written
  statement. `v3/examples/auto/` has eleven of twelve theorems closed
  with its sidecar committed and regenerated byte-identical; the
  engine closes 54 of calc's 100 claims unaided (the set recorded
  in `engine_calc.txt`). Found: a measured function's unfolding
  equation runs a rewriter without end (excluded from the engine's
  rules); a copy of a file is another requirement, its fingerprints
  carrying the module's name. Gates: 221 loader pins, `engine_test` (24 checks: the example's eleven theorems from the sidecar, the regeneration byte-identical, the 54 of 100), `tactic_test` (110 checks: calc's 100 claims), `define_test` (12 checks), `derive_test` (14 checks), `wire_test` (15 checks), calc byte-identical over 21 inputs and its host program byte-identical to the model world, parity byte-identical over 28 closures and 116,957 declarations, route 2, K's clients, the full suite's 35 entrypoints (the full run had 34 pass and parity refuse `engine.shard`'s opens and two pattern names, and the reader kit's match of the two new readings; parity, `engine_test`, K's clients and the reader tests were rerun green after the fixes). None of the files route 1 compiles changed; the full replay and the corpus are CI's.
- **2026-10-07 — pipeline 548 green on `2be8a72`: slice 3.22's landing 1 on
  the full gate.** engine 56 s; corpus 1,431 s at the baseline; v3 6,537 s: 34 entrypoints with 0 failed, `tactic_test` 110 checks (the 100 claims), `define_test` 12 checks, `derive_test` 14 checks, `wire_test` 15 checks, calc's host program byte-identical to the model world, parity byte-identical over 28 closures and 116,395 declarations in 600 s, route 1's native build of the kernel (`OK: v3/bin/t0`), the fixture byte-tie identical, replay 2,438 s at 32.7 GB peak, T0 accepted 57,977 with 0 rejected, closures identical to the oracle, all 20 accelerator candidates pinned. The v3 job is 278 s longer than pipeline 546's, 161 s of it the loader pins' entrypoint (3,298 s: six pins more), the suite's wall clock.
- **2026-10-07 — pipeline 546 green on `408a73c`: slice 3.21b on the
  full gate.** engine 28 s; corpus 1,485 s at the baseline; v3 6,259 s: 34 entrypoints with 0 failed, `tactic_test` 110 checks (the 100 claims), `define_test` 12 checks, `derive_test` 14 checks, `wire_test` 15 checks, calc's host program byte-identical to the model world, parity byte-identical over 28 closures and 116,229 declarations in 606 s, route 1's native build of the kernel (`OK: v3/bin/t0`), the fixture byte-tie identical, replay 2,514 s at 32.2 GB peak, T0 accepted 57,977 with 0 rejected, closures identical to the oracle, all 20 accelerator candidates pinned. Slice 3.21b is closed. The v3 job is 204 s longer than pipeline 544's, 185 s of it the loader pins' entrypoint (3,137 s against 2,952 s: three pins more, each streaming the second fixture), which is the suite's wall clock.
- **2026-10-06 — pipeline 544 green on `a299187`: slice 3.21 on the
  full gate.** engine 28 s; corpus 1,509 s at the baseline; v3 6,055 s: 34 entrypoints with 0 failed, `tactic_test` 110 checks (the 100 claims), `define_test` 12 checks, `derive_test` 14 checks, `wire_test` 15 checks, calc's host program byte-identical to the model world, parity byte-identical over 28 closures and 116,127 declarations in 608 s, route 1's native build of the kernel (`OK: v3/bin/t0`), the fixture byte-tie identical, replay 2,483 s at 32.1 GB peak, T0 accepted 57,977 with 0 rejected, closures identical to the oracle, all 20 accelerator candidates pinned. Slice 3.21 is closed. The v3 job is 2,083 s shorter than pipeline 542's, the test suite 2,046 s of it (2,952 s against 4,998 s) under the runner of eight: the suite's wall clock is the loader pins' entrypoint's (2,952 s for the 212 pins in one process, against 1,161 s on the development machine), so a wider runner gains nothing and the next cut is that entrypoint's. The entrypoints of 30 s and more sum to 6,177 s of process time: beside the pins, calc's two tests 951 s, parity 608 s, `define_test` 547 s, `derive_test` 422 s, `wire_test` 187 s, K's clients 171 s, `tactic_test` 157 s. The job's limit is 3 h (10,800 s).
- **Phase 3 close-out ledger (2026-10-13; opened at slice 3.30
  landing 1, closed at landing 4 — phase 3 closed 2026-10-13).** Each item of law §12.4 item 3 and
  its gates, the evidence, the status (`v3/LANGUAGE.md` §8.4 slice
  3.30's table as measured at the design, carried forward landing by
  landing):

  | gate item | evidence | status |
  |---|---|---|
  | the loader to the whole of E, the profile retired | slices 3.1–3.5; pipeline 487 | covered |
  | Stage 1 | slices 3.13–3.19 | covered |
  | Stage 2: the I elaborator, the goal graph (law §7.3), core tactics | slices 3.20–3.22: `goal_of`, `applicable`, `step`, `elaborate` as E functions over one derivation; the law's `change` is `show`; holes shared across goals and incremental goal states are T4's and T6's, phase 4 | covered |
  | certified arithmetic | slice 3.21 (`arith`) | covered |
  | the `Init` import with E realizations attached | slices 5b, 3.18, 3.26 | covered |
  | `std/list`, `order`, `nat`, `div`, `bits`, `arith` | slice 3.27: `list` and `bits` as files, the rest migration records (item 61) | covered as ruled |
  | the fifteen former axioms as theorems | `std/facts.shard` | covered |
  | the migration table validated | `std/migration.shard`, `std_test` | covered |
  | one arbitrary-`Prop` ghost refinement | `std/list`'s `Pairwise` subtype; T9 small (iv) | covered |
  | one static law-bearing package | slice 3.28 | covered |
  | `LEAN.md` | slice 3.27 landing 4; 108 rows | covered |
  | the first release bundle with retained P | slice 3.22 landing 3 | covered |
  | the pipeline measured | every pipeline's record above | covered |
  | T2, T3 | `records/T2-T3-2026-10-11.md` | met |
  | T9 small | `records/T9-small-2026-10-10.md` | met |
  | T9 again at the close (law §5.4) | slice 3.30 rule 5: nine tasks, the count agreed at the ratification | **met at landing 2**: 9 of 9 on the count, two kernel findings fixed (`records/T9-2026-10-13.md`) |
  | T10 | `records/T10-2026-10-12.md` | met |
  | Stage 3 (typeclasses, coercions) | the fixed instance table (3.27), the coercion (3.15); search the door | as ruled (item 61) |
  | §11: T1's branch-local proof | slice 3.24 | covered; the row rewritten at landing 1 |
  | §11: T5's imported-theorem form | pin `realize_imported` | **closed at landing 1** |
  | §11: T5's two instances; R57's checked instance | no consumer with two implementations of one view | **carried to phase 5** (ruled 2026-10-13, item 64) |
  | §11: `Symbol`'s `String` flip | open since 3.18 | **carried to phase 4** with route 1's chain (ruled 2026-10-13, item 64) |
  | §11: R60's facilities; lambda lifting | slices 3.15–3.18; slice 3.28 | covered; the rows rewritten at landing 1 |
  | law §10.5's phase-3 doc rows | `LEAN.md` written; `TOTALITY.md`, `REFINEMENT.md`, `STORAGE.md` superseded and `CERT.md` amended by banner | **done at landing 1** |
  | the LANGUAGE.md consolidation (ruled 2026-09-17) | rule 6's shape, ratified | **done at landing 3**: the slice texts §9.1 below, the current statements §8.4 |
  | §13 for ratification | items 39–42, 44–46, 48 without a ruling marker; 64 ratified 2026-10-13 | **ratified 2026-10-13 at landing 4** (the user's pass; item 46 with its reason recorded); every item of §13 ruled |
- **Phase 2 close-out ledger (2026-09-13; closed at slice 8).** Each
  item of §12.4 item 2's gate, its evidence, its status:

  | gate item | evidence | status |
  |---|---|---|
  | T1: a decision tag with erased payload | `realize_view` (`dite`, `ite` derived); `ev_test`'s `Nat.decLt` cell | covered (slice 5b) |
  | T1: a branch-local bound proof | `Fin` inside the `Int` fixture; `Fin n`'s value parameter is not an E parameter at Stage 0 | **carried to phase 3** (ruling 2026-09-13; `LANGUAGE.md` §11) |
  | T1: raw versus checked arguments | checked: `entry`, `entry_s`, `entry_test` (slice 7); preconditioned: a `realize`'s erased binders, the `REALIZE` record (5b) | covered (7) |
  | T5: eight fixtures | as audited above; `same_spelled` and `realize_theorem` new | six covered; **two carried to phase 3** (§11) |
  | T8: the replay half | direct P and origin-only invariance (2); the route recorded (phase 1) | covered |
  | conformance 1: frontend parity | `parity_test.sh`, every toolchain closure byte-identical (6) | covered |
  | conformance 2: execution parity | route 2's byte-tie (5); calc's differential (6); the primitive suite (7) | covered |
  | conformance 3: checker parity | routes 1 and 3 byte-tied over the whole export (phase 1) | covered |
  | conformance 4: independent pins | `t0_expected.txt`; the pins' `;; expect:` headers, fixed by hand | covered |
  | law §10.5's doc rows at phase 2 | `LANGUAGE.md` (slices 1–7), zed, the viewer's README, the CI and corpus headers, TCB (7); `v3/CANON.md` (8) | covered |
  | `LANGUAGE.md` §13 and `CANON.md` §9, for ratification | items 1–33 (31–33 from slice 9; 1, 8, 9, 12, 13, 15, 16, 18, 22, 23, 29 amended or clarified in place at slice 10 under GPT-6's ratification memo, §4.9, which recommends a disposition for each); seven ruled and five open, GPT-6's positions on the five recorded (§4.8) | **outstanding**: the user's pass over the consolidated text |

### 9.1 Phase 3's slices 3.13–3.29 — the designs and the as-built texts of `v3/LANGUAGE.md` §8.4, moved here verbatim at slice 3.30 landing 3 (2026-10-13)

Moved by rule 6 of slice 3.30 (§8.4; §13 item 64, ratified 2026-10-13): each slice's design and "as built" sections as they stood in `LANGUAGE.md` §8.4 at `e05bad9`, the text unchanged — the `####` heading lines below are this record's index and nothing else was added or rewritten (the moved text's SHA-256 prefix, taken before the cut and after the paste, is `a500557b5ca452f3`). §8.4 keeps, per facility, the current statement of each rule with its as-built note beside it, numbered as the slice numbered it, so `§8.4 slice N rule K` in the kernel's comments and messages, `LEAN.md`'s rows and the pins' README names a rule there and its full text here. Cross-references inside the moved text ("above", "below", "the design below", "§8.4 rule 3") read as they did in §8.4's chronology; the slice table in §8.4 is the index, each row pointing here. Slice 3.30's own text stays in §8.4 until the close.

#### Slice 3.13

**The rules of slice 3.13**, decided here (the user's ruling of
2026-09-17 on the five leans; §13 item 43):

1. **Eligibility is automatic and transitive, as a `type`'s is** (§8.1
   rule 1). A `fn` gets its L half when every binder and return type
   has an L identity and every head of its body does — a constructor
   of an E-eligible inductive, a `fn` with a definition, a realized
   constant, a `sig fn` with an L parameter (not an E parameter, item
   40), a table entry with an identity under rule 3. Otherwise it stays
   `RUNNABLE` as today and the record names the first obstacle. Once
   eligible, a body that fails translation or K is a **refusal**,
   never a silent fall-back to `RUNNABLE`: a typo must not quietly
   degrade a definition to a body no theorem can cite. K's own sources
   import no `Init`, so nothing inside the seal changes at this slice
   (their `fn`s enter L at the flip, rule 1). **As built:** the
   obstacles are `e_only_type` (a binder or result type with no L
   identity here), `no_l_meaning` (a callee that is `RUNNABLE`, an
   extern), `no_l_identity` (an operator with no identity at its
   operand types, a string or symbol literal), `unknown_constant` (a
   constructor of an E-only type; a primitive's identity past the
   Init prefix), `measure_pending` (rule 4), `numeral_pattern` (slice
   3.15's) and `recursion_depth` (slice 3.14's; rule 2) — `RUNNABLE fn
   NAME why=OBSTACLE`; every other failure is `REFUSE NAME reason`
   through the classifier's record. The equations are stated only where
   `Eq` and `Eq.refl` are in scope (an Init prefix that ends before
   them gives `DEFINE NAME equations=` empty: the definition alone).
   The fn enters the E table before its definition is attempted, so a
   self-call types statically.
2. **The definition's value uses the recursor directly.** The binders
   are lambdas; the leading case tree (§7.5's `compile`) is nested
   `T.rec` with a constant motive, the reverse of the derived view's
   erasure of a recursor; a self-call on an **immediate** constructor
   field of the matched parameter is the induction hypothesis of the
   same recursor. A self-call on a deeper field (`List.concat` on `t`
   under `(cons x (cons z t))`) is `recursion_depth` at this slice —
   an **obstacle** (`RUNNABLE … why=recursion_depth`), since it is
   slice 3.14's shape and not an author's error (calc's `parse_rest`
   is one); a self-call not on a field of a matched parameter is
   `realize_descent` as today, a refusal. Nested and non-leading
   matches split by the recursor with a constant motive and open their
   hypotheses unused; a match on a variable whose constructor an
   enclosing split fixed is resolved statically (the matrix knows it),
   never a second recursor. The
   equations are `NAME.eq_N`, one per leaf as §7.5 states them, proven
   by `Eq.refl` — K unfolds the definition and iota-reduces the
   recursor on the constructor-headed left-hand side — and the body is
   attached through the realize path, so `ev` runs it under the L
   identity and the `REALIZE`/`PENDING` machinery (R58) is the same
   for every function. The record is `DEFINE NAME equations=NAME.eq_1,…`
   beside the `ACCEPT` of the definition and of each equation.
3. **Operator spellings get L identities by operand type, only where
   `ev` already agrees with K** (settles item 38): `+` and `*` at
   `Nat` are `Nat.add` and `Nat.mul`; `+`, `-` and `*` at `Int` are
   `Int.add`, `Int.sub` and `Int.mul`; `int_eq`, `lt` and `le`
   (`=`, `<`, `<=` since slice 3.17) are
   `Nat.decEq`/`Nat.decLt`/`Nat.decLe` at `Nat` and `Int.decEq`/
   `Int.decLt`/`Int.decLe` at `Int`. `-` at `Nat` is refused
   (`nat_operator`, pointer `Nat.sub`): `ev` computes the integer
   difference and `Nat.sub` truncates, and no dump could tie the two.
   `/` and `mod` at `Nat` are refused likewise (`ev`'s `/` is stuck at
   zero, `Nat.div` is total); at `Int` they have no identity (rule 5,
   unchanged). Bitwise and symbol entries have none. **As built:** the
   operand types are the classifier's static types, and `+ - *` on two
   `Nat` operands type as `Nat` statically (`prim_static`), so a nested
   `(+ n (+ n n))` resolves; the comparisons' static result type stays
   the prelude's `Bool` — the translation of an `if` reads the
   decision's inductive off K's type of `Nat.decLt a b`, so no flip
   was needed (`ev`'s cells unchanged, item 17). **Numerals** (law
   §5.2's one coercion): a non-negative numeral at an expected `Int`
   is `Int.ofNat n` and a negative one `Int.negSucc (-n-1)`, where
   `Init`'s `Int.ofNat` is in scope; at `Nat`, or with no expected
   type, K's `Nat` literal as before — so `(match xs (nil 0) …)` at
   `Int` states `Int.ofNat 0`; since slice 3.15 a theorem about it
   writes `0` too (§5.3).
4. **A `fn` with a non-structural measure stays `RUNNABLE`** at this
   slice, reason `measure_pending`; slice 3.14 gives it
   `WellFounded.fix`.
5. **A `def` in the E forms** is elaborated by the same `tr` with the
   declared type as the expected type; the explicit-L path is
   unchanged. **As built:** the reader's explicit-L path is tried
   first; when it fails on a `def`, the form is read as a `fn`'s
   signature and body by the classifier and defined the same way (the
   definition only — no E body, no equations); when that fails too the
   reader's own error stands. A body mixing explicit-L forms inside
   the E forms is not 3.15's either (its §5.1 `if` and operators are
   the L side's; a `match` in a statement stays refused). The pin
   `def_match` shows a `def` with a `match`. **Superseded at slice
   3.16 landing 1 (2026-09-19):** the elaborator reads `match`, `if`
   and a body's untyped `let` itself, so the fall-back through the
   classifier is **deleted** (`def_e_forms`; it had begun to hide the
   reader's own error behind a second reading) and `def_match` goes
   through a matcher.

**Not in 3.13:** nested and numeral patterns beyond the leading case
tree (`equation_form`, as for a `realize`), mutual recursion,
lambda lifting, any change to the reader.

#### Slice 3.14

**The rules of slice 3.14** (the user's ruling of 2026-09-17 on the
three leans; §13 item 47):

1. **One scheme for all structural recursion, Lean's own.** A
   structurally recursive `fn` is `T.brecOn` applied to a functional
   `λ x below . body` whose table `below : T.below motive x` holds the
   results at every sub-structure; every runtime split of a variable
   that owns a table generalizes the table in the split's motive (`λ
   x' . T.below motive x' → R`), so inside a branch the table's type
   reduces to the pair nest and each recursive field's entry — its
   value and its own sub-table — is a typed projection; a self-call on
   any such field, at any depth, is the entry's value applied to the
   call's other arguments. 3.13's direct-recursor path is deleted, not
   kept beside it. `Init`'s inductives use `Init`'s `below` and
   `brecOn` at solved levels; a native inductive gets `T.below` and
   `T.brecOn` generated as monomorphic definitions on first need,
   under Lean's names, admitted before the function (Lean generates
   them at declaration time and only for recursive types, which is why
   the export holds no `Option.below`). The equations stay `Eq.refl`:
   `brecOn` unfolds to the recursor and reduces on a constructor-headed
   argument. `recursion_depth` survives only for a self-call on a
   variable bound outside the leading case tree. **As built:** the
   table's shape is Lean's, read off the export's `List.below` and
   `Nat.below` — the pairs `PProd (motive r_k) (below r_k)` over a
   constructor's recursive fields, right-nested by `PProd` with no
   unit, one field's pair the table itself, no field `PUnit`; a
   field's entry is derived positionally (K's instantiation
   beta-reduces the motive applications in the reduced table type, so
   the field cannot be read off it), and K's check of the definition
   is what holds the shape to Lean's. `Init`'s `brecOn` goes through a
   `brecOn.go` helper the definition never sees. A generated pair
   (`T.below`, `T.brecOn`) is two `ACCEPT` records before the first
   function that needs it; a file without `Init`'s `PProd` in scope
   cannot have a table, so its structural recursions stay `RUNNABLE`
   (`below_unreached`) — K's own sources included, whose types are
   E-only first anyway.
2. **A measure becomes `WellFounded.fix`, the decreasing facts
   admitted obligations.** `(fn f PARAMS RET (measure M) BODY)` with
   `M` a `Nat`-valued E term over the parameters is `WellFounded.fix
   (InvImage.wf m (WellFoundedRelation.wf Nat.lt_wfRel)) F p` over
   the data parameters packed as a `PProd` nest `p` (a single
   parameter is itself), `m` the measure over the tuple and `F : ∀ p,
   (∀ q, InvImage … q p → RET) → RET` the body with each self-call
   `rec (tuple of the arguments) f.dec_N …`. Each `f.dec_N` is an
   **obligation**: an axiom-kind constant `∀ (the locals in scope at
   the call), InvImage Nat.lt_wfRel.rel m (tuple) p`, admitted before
   the definition as the **fourth policy class** beside view
   parameters (§9) — `PARAM f.dec_N obligation`; the function's
   record is `DEFINE f equations= pending=f` with `PENDING f measure`
   naming the obligations, every dependent's `params=` reaches them
   (R57/R58: nothing is claimed that is not visible), and a later
   `(fulfills f.dec_N PROOF)` discharges one as an implementation
   discharges a requirement (the form is I's, phase 3). No equations:
   `WellFounded.fix` reduces through `Acc.rec` on a proof, which K
   does not compute, so `f.eq_N` for a measured function is I's
   (`WellFounded.fix_eq`). An `Int`-valued measure is the obstacle
   `measure_type` with the pointer to a `Nat` measure — the law never
   inserts `Int.toNat` (§5.2). **As built:** the obligation is
   quantified over every local in scope at the call, in binding order
   — the type parameters, the tuple, the fields of the enclosing
   splits (each split's equation `scrutinee = pattern` after its
   fields, since slice 3.21: a matcher under a measure carries it),
   the `let`-bound values as `let`s — and the proof term
   applies the constant to the lambda-bound ones; the obligations a
   branch collects survive the branch's context being restored; the
   Int fixture runs through `InvImage.wf` so the pins reach it; a
   prefix that does not is the obstacle `wf_unreached`.
3. **Mutual recursion is an obstacle** (`mutual_recursion`: a callee
   pre-registered in the file and not yet defined), taken with slice
   3.15. A forward reference to a later-defined function is the same
   obstacle.

#### Slice 3.15

**The rules of slice 3.15** (the L-side ergonomics of law §5.1–5.2;
the user's ruling of 2026-09-17 on the leans, §13 item 48; built
2026-09-18):

1. **Metavariables are nodes** — `MVar` in `Expr`, `LMVar` in `Level`
   (law §6: native metavariables that K refuses). K gained refusing
   arms only: `infer` on the node is `Malformed metavariable`
   (type_checker.cpp l. 342), the raw entry refuses a declaration
   carrying one (`mvar_in_type`, `mvar_in_value`, beside the fvar
   checks), the node data carries `has_mvar` (levels included), and
   the exporter never emits one, so the import is unchanged. The
   first K edit since the seal (§13 item 26); hostile pins 17a–d.
2. **The elaborator is a Meta layer above K** (`kernel/elab.shard`
   over `unify.shard`, `scope.shard`, `kw.shard`): bidirectional, over
   K's locals, first-order unification with typed assignments, K's
   `whnf` and `is_def_eq` through the view for the closed residues —
   §5.1 states the rules. The reader (`reader.shard`) reads the forms
   over it; nothing of it crosses into K.
3. **The spelling changes it makes** (the phase-2 pins rewritten, 45
   files): an implicit binder's argument is inserted, never written —
   `(Eq.{1} Nat a b)` is `(Eq a b)`, `(Eq.refl.{1} Nat a)` is
   `(Eq.refl a)`, `(List.cons.{0} a x t)` is `(List.cons x t)`;
   `@NAME` where the old spelling is wanted. A `type`'s parameters are
   implicit in its constructors and a `fn`'s type parameters in its L
   signature, equations and obligations, so a theorem cites
   `(len xs)` and `(Pair2.mk a b)` as a body does (guard 1). A
   universe not written is inferred. A statement's `0` at `Int` is
   `Int.ofNat 0`, its `(+ a b)` at `Int` is `Int.add a b`, its `(= a
   b)` is `Eq` with the type inferred (`std/list.shard`, calc's
   `eval_add`). Two refusals moved from K to the elaborator: a false
   theorem's proof and a universe collapse are `type_mismatch` before
   K sees them (K's own refusals stay pinned in the hostile battery),
   as is a consumer's `Eq.refl` through a view parameter (§8.5's
   `view_rfl`: the unifier asks K's `is_def_eq` over the consumer's
   environment, where the parameter is opaque). `universe_args_required`
   is gone; `unsolved_universe` is its Stage-1 counterpart.
4. **`if` and the operators in a statement** are §5.1's: `ite` with
   the decision of the proposition's head, the identities by the
   first operand's type, `-` `/` `mod` at `Nat` included in L.
   **Not in 3.15:** a `match` inside a statement (the pointer is
   `T.casesOn` or a helper `def`), `Bool` against `Decidable` bridging
   (`decide`; calc's `is_digit` stays `RUNNABLE` for it, slice 3.16),
   E's rename of `lt le int_eq` to `< <= =` (3.17 with the tool
   migration), `UInt*`/`Fin` numerals (with their E realizations),
   explicit holes `_`.
5. **The constructor-structure bundle is generated after a native
   inductive's admission**, in Lean 4.33's shapes read off the export
   (`define.shard`'s `confusion_bundle`; `loader.shard`'s
   `bundle_after`): `T.casesOn` (the recursor with the hypotheses
   unused, universe polymorphic) for every native inductive whose
   recursor eliminates into every universe; and, for a type without
   parameters, indices, universe parameters or dependent fields, where
   `Init`'s `Eq`, `Eq.refl`, `Eq.ndrec`, `And` and `And.intro` are in
   scope, `T.noConfusionType` (two nested `casesOn`s: the same
   constructor gives `(∀ f_eqs, P) → P` with `Eq` on each field pair,
   different ones `P`), `T.noConfusion` (`Eq.ndrec` over a `casesOn`
   whose minors apply the continuation to `Eq.refl`s) and `T.c.inj`
   per constructor with fields (`Eq (c fs) (c fs') → And …`, a term
   proof through `noConfusion`, the fields implicit as Lean's). Each is
   an `ACCEPT` record K checked; where the conditions fail nothing is
   generated and the type stands (K's own sources: no `Eq`, as Lean's
   Prelude before it). A parametric type's shape — the parameters
   generalized twice, `HEq` on the fields, `eq_of_heq` — is a later
   slice's; `injection` is I's tactic over these. A theorem cites
   `(Color.noConfusion h)` at `False`, `(Tree.Node.inj h)`, or
   `(Tree.casesOn t …)` with the motive found from the expected type
   (a Miller pattern `?motive t` in the unifier, the bound-variable
   arguments elaborated before the propagation as Lean's
   `elabAsElim` orders them). Pin `elab_confusion`.
6. **A forward reference is parked, never an obstacle** (3.14 rule 3
   revised): a `fn` whose callee is not yet loaded (`mutual_recursion`
   from the translation) or whose callee is itself parked
   (`callee_pending`, the callee named) enters the E table and waits
   with no record; after every definition the parked ones are tried
   again to a fixpoint; at the file's end the ones still parked are
   `RUNNABLE` with the obstacle they waited on — a true mutual pair
   stays `mutual_recursion` (pin `define_mutual`), a fn behind a
   `RUNNABLE` callee `no_l_meaning`. A `realize` never waits (it is
   its own declaration's): a pending callee is `no_l_meaning` there.
   Pin `define_forward`. Mutual structural recursion proper (Lean's
   packed `brecOn`) has no consumer yet and stays out.

**As built (2026-09-18, the elaborator):** `unify.shard` — `MCtx`
(declarations with name, type, telescope and kind; assignments;
`inst_mvars`/`inst_level` the instantiation; `unify` with `assign`
typed through K's `infer` on a closed value and `infer_light` — a
constant, local or metavariable head's telescope walked — on an open
one; `mc_drop_fvars` at every binder close, so a metavariable that
survives a `fun` cannot later capture its variable). `elab.shard` —
`El` (K's world, the metavariable context, the scope context with the
bound names and their fvars), `elab` returning term and type, `tele`
(the telescope into slots: an inserted metavariable or a written
argument's placeholder), `elab_args` (propagate, elaborate, numerals
last, instantiate), `check_expected` (unify, else the coercion, else
`type_mismatch`), `elab_op`/`op_identity`, `elab_if`/`decision_of`,
`el_finish` (the closed term; the first unsolved metavariable named).
The pins `elab_implicit`, `elab_numeral`, `elab_unsolved`,
`elab_instance`, `elab_no_decision`, `elab_mismatch`; the reader pin
`refuse_unsolved_universe`.

#### Slice 3.16

**Slice 3.16 — one front end: the pre-definition and its two
projections** (DESIGN, 2026-09-18, revision 2 the same day after
GPT-6's single-frontend memo R63–R71, records §4.10; the user's
rulings of that day on the reorder, the shape and the memo's leans;
§13 item 49; built in landings, each with its as-built below).

*Why.* Slice 3.15 built the elaborator law §5.1 describes, and left a
`fn`'s body on the older route: the classifier types it with E's own
first-order typing (`c_type`), and `realize.shard`'s `tr` translates
the E term **up** into L. The law's direction is the other one (§2,
§4: E is a fragment of L; the classifier is the *fragment*
classifier, a Stage-1 item after elaboration). The upward route
exists because the Stage-0 ruling of 2026-09-07 made `fn` E-only, so
E needed a reader and a typing before any L existed for it. Guard 1
above was written against `tr` and was met in wording only. The
symptoms are two type systems and two resolvers: at the slice's
opening calc's spec file has 12 functions defined and 13 `RUNNABLE`,
from three roots — `is_digit` and `is_ws` (a comparison is a `Bool`
in E and a proposition in L), `digit_val` (`(- c 48)`: a literal has
no type in `c_type`, so the operator finds no identity) — and `Add`
names calc's constructor in a `fn` body and is `ambiguous_name` with
Init's class in a `def`.

*The probe* (2026-09-18; a scratch file through the loader with
`--dump`, each function once as a `fn` and once as a `def` with the
derived view of §7.1). A non-recursive `match` (`flush`): the two E
programs byte-identical, the two L values one hash. Structural
recursion: K's value is `brecOn` and §7.1 refuses it, as §7.3 said it
must (Lean compiles its pre-definition, never the kernel term); the
hand-written pre-definition — `casesOn`, the self-call a constant —
erases to the classifier's program up to the primitive's spelling
(`Nat.add` for `+`). At `Int` the erasure refuses `Int.add`: the
operator table has no inverse. Nested patterns (`parse_rest`): the
erased `casesOn` tree means the same by reading (not run) and is not
the program — four arms become a tree with the fall-through eight
times. So neither extreme: E cannot be read off K's finished term,
and L should not be translated up from E.

*The invariant* (R63's wording, adopted): resolve and type the
intended computation **once**; preserve the structure needed to
justify it and to execute it; derive the mathematical and the
executable representation from that common construction. Neither
output is reconstructed from the other after information has been
discarded. The common input is not itself a proof that the two
projections correspond: the erasure rule's implementation stays a
stated bring-up trust dependency (§7.1, R58) until a checked
translation or route 1's proof covers it.

*The shape.* S is elaborated once, by `elab.shard`, into a
**pre-definition** — a named record (`PreDef`), not a bare body
(R63): the declared identity and type; the binder telescope (K's
locals); the body, an ordinary `Expr` over them; the **self-local**
when the function is recursive (its name bound to a local of the
declared Pi type); the **matcher descriptions** the body's matchers
were generated from (rule 1); the **callee-locals** standing for
`RUNNABLE` callees (rule 6); the obligations owed; the resolved
environment it was elaborated against. A pre-definition is never an
admitted declaration: a body well typed *assuming* a local for its
own name is provisional until the K projection admits it, and no
operation takes a `PreDef` where a `ConstantInfo` is wanted. Two
projections consume the record — neither re-resolves a name or
re-reads S:

- **toward E** — erasure (§7.1's rule, generalized): types, proofs
  and erased binders dropped, a matcher application back to the
  `EMatch` of its description, `ite`/`dite` to `EIf`, the self-local
  and a callee-local to the `ECall`, a resolved operation to its
  realization (rule 4). This **is** the function's program: for a
  function on this route the classifier's typing and `tr` do not
  run.
- **toward K** — the matchers expanded to their `casesOn` trees and
  the recursion compiled as slices 3.13–3.14 ruled (`T.rec` with a
  constant motive, `T.brecOn` with the table generalized at each
  split, `WellFounded.fix` with the `f.dec_N` obligations),
  `define.shard`'s machinery re-pointed from E terms and E types to
  the pre-definition and K's types. Unavailable while a callee-local
  is open (rule 6).

The classifier becomes what the law calls it: a check on the
elaborated term that it lies in the fragment (§6.3's refusals — a
function value in a runtime position, a noncomputable head, a callee
without a realization — raised by the erasure), not a second type
system. There is no new term language: the body is `Expr`.

**The rules of slice 3.16:**

1. **Matchers, their descriptions bound by regeneration** (R64). A
   `match` elaborates to `OWNER.match_N` applied to a motive, the
   scrutinees and one alternative per row (Lean's shape: `(motive :
   D… → Sort v) → (d : D)… → (alt_i : ∀ pattern variables, motive
   pattern_i)… → motive d…`; an alternative without variables is
   `motive pattern_i`). The matcher is a definition K checks,
   admitted before its owner with an `ACCEPT` record as the bundle's
   constants are: a `casesOn` tree by first-match column splitting
   (the compilation `df_matrix` does today, stated over K's types), a
   fall-through row's alternative applied at every leaf it reaches.
   The **description** — the scrutinee types, the rows' patterns in
   order, each alternative's telescope — **is the matcher's type**
   (as built, landing 1; the design's side table dropped): each
   alternative's type spells its row's patterns as a term over its
   variables, so the rows are read back off the admitted declaration
   (`mt_describe`) and nothing beside it can go stale, be lost in a
   view's fork (§6.6) or be trusted by a name's spelling. A projection
   that meets a matcher application **regenerates** the definition
   from the rows read back and compares it with the admitted value
   (`mt_is_matcher`); a constant that fails the comparison is not a
   matcher — `matcher_description` to a projection, never a
   name-based guess and never an eager call. The generator is
   deterministic, so the binding is a check, not a trust.
   **Forcing:** the E program evaluates the scrutinees once, left to
   right, then the first matching row's arm only — `EMatch`'s rule
   (§6.2); an alternative is control flow in E and an argument only in
   L, where nothing runs. The elaborator cites a matcher with the
   motive from the expected type (slice 3.15 rule 5's Miller pattern
   gives the constant motive); a dependent motive is a later slice —
   slice 3.19 (rule 9) for a scrutinee that is a local the expected
   type mentions.
   **A `match` in a statement or a `def` is the same form** — slice
   3.15's open item closes here. Literal patterns are the obstacle
   `literal_pattern` until a consumer under the naming law has one.
2. **The self-local, and branch facts in every position** (R65). A
   `fn` with a `(measure …)` is elaborated with its name bound to a
   local of its Pi type. The K projection keeps today's guarantee and
   extends it: inside a recursive function an `ite c inst a b` is
   compiled as the **dependent** form (`dite`; today's `Decidable.rec`
   with its proof field in scope — Lean's well-founded preprocessing
   does the same), so `h : c` or `h : ¬c` is a local of the branch
   wherever the `if` stands — under a `let`, in an argument, under a
   matcher's alternative — and a matcher's alternative has its
   pattern variables in scope the same way. Each `f.dec_N` is closed
   over **every** local in scope at its call (slice 3.14 rule 2's
   as-built, now position-independent); sibling branches share
   nothing. **Narrower guarantee, stated:** for a `match` on a
   computed scrutinee the equation `scrutinee = pattern` is not yet a
   local (a variable scrutinee is substituted, as today); an
   obligation that needs it is reported with the pointer to bind the
   scrutinee in a `let` — built when a consumer has one. Outside a
   recursive function `ite` stays `ite`.
3. **`Bool` and propositions meet in the elaborator, Lean's way, and
   the conversion is explicit in E** (R66). A proposition where a
   `Bool` is expected is `Decidable.decide p` with the decision
   `decision_of` finds (else `no_decision`); a `Bool` where a
   proposition is expected — an `if`'s condition — is `c = true` with
   `Bool`'s `decEq`. This **changes slice 3.15 rule 4**: `if` on a
   `Bool` is `ite (c = true) …`, not `cond` (Lean's own elaboration;
   `cond` sits at export line 151,585, past every fixture). Erasure:
   `Bool` and `Decidable` have **different cells** in `ev`
   (`init_bool_cell`, `dec_cell`), and only the second-constructor
   rule of `EIf` lets either drive an `if`. So `decide p inst` erases
   to the conversion `(if ⟦inst⟧ true false)` over Init's `Bool`
   constructors — no new primitive — and is the identity only where
   the realization of `inst` already returns a `Bool` cell (the table
   says which: rule 4); the decision of `c = true` in a condition
   position erases to `c`. A shared two-valued representation is a
   later optimization, never assumed.
4. **Realization selection, not an inverse of spellings** (R66). The
   operator table is the first form of a small static realization
   registry: a resolved L operation at its instantiated types → the
   executable entry, its result cell (an integer, a `Bool` cell, a
   `Decidable` cell) and its stated trust. (Since slice 3.19 a
   decision's entry is its *test*, read where the decision is an
   `if`'s condition; as a value the decision is `(if TEST isTrue
   isFalse)`, `Decidable`'s own cells — a match by constructor on the
   bare test took the wrong row.) `Int.add`, `Int.sub`,
   `Int.mul`, `Int`'s and `Nat`'s decisions map to the entries §8.4
   rule 3 maps from; the `Nat` identities to the entries under their
   own names. **Once `-` `/` `mod` at `Nat` have resolved to
   `Nat.sub`, `Nat.div`, `Nat.mod`, the erasure selects those
   entries** (saturating; total at a zero divisor) — this **reverses
   revision 1's refusal**, which protected the E-first reading of the
   spelling and has no reason to survive the direction change. The
   E-first route keeps its own semantics and its `nat_operator`
   refusal for the sources that stay on it (rule 6). An operator
   means the same in a `fn`, a `def` and a theorem once its operand
   type is fixed.
5. **One resolver.** A `fn` body's names resolve as a `def`'s do
   (`scope.shard`). Where a bare name is ambiguous and exactly one
   candidate is a constructor of the expected type's head inductive,
   that constructor is meant (§3.1's "Stage 1 resolves constructors
   by expected type", now built) — `Add` in calc's bodies and in its
   theorem alike.
6. **Which route; a fallback weakens the guarantee and never
   reinterprets** (R70). A function takes this route when its
   signature elaborates (every binder and return type has an L
   identity), decided **before** its body is read. On the route:
   - a callee that is `RUNNABLE` (no K constant) is bound as a
     **callee-local** of its elaborated signature type — the body
     still elaborates once, with every other choice resolved as it
     would be, and erases; the K projection is unavailable, the
     record is `RUNNABLE … why=callee_runnable route=typed` with the
     callee named, and nothing is re-read by the classifier. A callee
     whose own signature has no L reading is the head obstacle below;
   - a **head without an L identity before slice 3.17** — a symbol or
     a `"…"` literal in the body, a toolchain-prelude spelling — is
     the one case that still falls to the E-first route whole, because
     no L term exists to project: `RUNNABLE … why=no_l_identity
     route=e_first`, machine-readable, the set of such functions in
     the tree named in the as-built and **emptied by 3.17** (then this
     bullet is deleted with its code);
   - anything else that fails is a **refusal** (`type_mismatch` and
     the rest of §5.1's): a typed-route error never becomes an
     E-first success, and a `fn` body is typed against its signature
     from this slice on (§12.1's "return types unchecked" row).
   A function whose signature has no L reading — every function of
   the toolchain's own sources, which name the prelude's types —
   never enters the route: `route=e_first` is theirs until they port
   under the naming law, and the route is deleted then (guard 2; the
   named consumers are `v3/kernel/**` and the old tree's ports, no
   per-feature switch). Frontend parity and route 2's byte-tie are
   over those sources and do not move. A request for an admitted
   definition (a theorem citing the function, `realize`'s callee
   check) refuses a `RUNNABLE` result as today (`no_l_meaning`).
7. **`realize` with a supplied body** takes the same route: the body
   elaborated against the realized constant's type, its program the
   erasure, its equations per leaf as §7.5 has them. With rules 1–6
   `tr` has no caller and is **deleted**, with `c_type`'s use as a
   typing for L (`arg_types`, `prim_typed_identity`, `ctor_etargs`).
   The derived view is the E projection applied to a constant's
   value, unchanged in what it accepts.
8. **One descent obligation discharged end to end** (R65, R69).
   `(fulfills f.dec_N PROOF)` after the function, outside a view:
   PROOF elaborated as a term against the obligation's statement and
   checked by K in the environment **without** `f.dec_N` and without
   anything whose `params=` reaches it — the closure instrument
   (`axioms.shard`) over the proof's constants is the acyclicity
   check, so a proof resting on its own obligation, directly or
   through a second obligation or a theorem, is `fulfills_cycle` and
   closes nothing. On success the obligation's record is
   `DISCHARGE f.dec_N`, the function's `pending=` and every
   dependent's `params=` lose exactly that name (the closure
   recomputed, never a suppressed warning), and any assumption the
   proof itself used stays in the account. The fixture is
   `define_measure`'s `count`: its `dec_1` printed with `h : ¬(n =
   0)` in the statement, discharged by a handwritten term over Init's
   library (no `omega`, no I), the pending sets empty after. An
   obligation stays an axiom-kind constant of the fourth class for
   provisional checking and is never an ambient axiom (slice 3.14
   rule 2). Dependency-directed wake-up of parked work is **deferred**
   (the retry fixpoint of slice 3.15 rule 6 stands while loads are
   small); what a parked function waits for — a declaration, a proof
   — is already distinct in its record.
9. **Partial outcomes at the shared boundary** (R67). `kw.shard`'s
   wrappers return K's outcome — the value, or K's refusal with its
   reason, or exhaustion — not `None`. `unify` gains two results
   beside `UYes`/`UNo`: `UStuck` (blocked on a named metavariable;
   retry after assignment) and `UExhausted` (the depth or K's
   budget); neither is ever read as inequality, and `check_expected`
   reports them as `unify_stuck`/`unify_exhausted`, not
   `type_mismatch`. An assignment is recorded **typed** or
   **tentative**: a value with a metavariable inside whose type
   `infer_light` cannot give is tentative, its typing constraint kept
   and checked when the value closes; a **closed** value K refuses to
   type is `UNo` with K's reason (today it is installed — corrected
   here). `el_finish` refuses an outstanding tentative assignment as
   it refuses an unsolved metavariable. Failure leaves the context as
   it was (the transaction of slice 3.15 stands). The vocabulary is
   for I and search to share; private `Option` helpers below the
   boundary stay.
10. **Holes across a binder: the narrower guarantee, stated** (R68,
    **deferred to I's opener**, slice 3.20 — 3.19 when written; deriving took that number at item 51). Today `mc_drop_fvars`
    removes a closing binder's local from every unsolved
    metavariable's telescope, so a hole cannot be filled with that
    variable after its binder closes. That is **incomplete, never
    unsound** (the assignment is refused; K checks the closed term),
    and the elaborator of this slice does not need more. The
    contextual-hole contract — a stable creation telescope, each
    occurrence an instantiation of it, delayed assignment — is built
    once, in the shared `MCtx`, as the first item of I; until then no
    API outside `elab.shard` may take an `MCtx` across a binder
    close, which is what keeps callers off the deferred capability.
11. **The facilities that are small once there is one place for
    them:** `(list a b c)` by the expected type's `List` (no expected
    type: `unsolved_implicit`, never a default element type); a
    negative numeral is already §5.3's. Record update, the fresh-name
    convention and E's rename of `lt le int_eq` are slice 3.17; the
    byte and text adapters, deriving and — moved there by item 50's
    ruling — `"…"` in a `fn` body, symbols and `Name` literals are
    slice 3.18, the wire's type ruled first.
    The three consumers after it — a branch-local refinement, a
    dependent-safe record update, a contextual proof step — use the
    `PreDef` record, the branch scopes of rule 2 and the outcomes of
    rule 9; none gets a representation of its own (R71).

**Landings and gates** (R71: exact agreement is the first migration
alarm where the representation is meant to be unchanged; every
mismatch is investigated; an intended change is accepted with its
stated comparison and the migrated references, never by editing the
expected output until the test passes). (1) The `PreDef` record;
matchers with descriptions and the regeneration check; `match` in L
terms; constructors by expected type; rule 9's wrappers and outcomes.
(2) The E projection generalized (matcher, self-local, callee-local,
the `decide` conversion, realization selection, `Bool`'s `if`) with a
**tie test**: for every function defined today in calc's files and
`v3/std/list.shard`, the erasure of the pre-definition against the
classifier's program, byte for byte up to rule 4's entries. (3) The
flip: `fn` and supplied `realize` on the route, `define.shard`
re-pointed, `tr` deleted, rule 8's discharge; then `(list …)`. Every
landing keeps the loader pins, parity, route 2, K's clients and T0
green. **The slice's gate:** calc's spec file with all 25 functions
defined; `calc_test` byte-identical against the old tree with the
programs now the erasures; the equations' names unchanged; a value
hash that moves is listed with its cause (a function whose body has
an `if` is expected to keep `Decidable.rec`'s shape and its hash —
rule 2 — where it is recursive, and to move to `ite` where it is
not); and the boundary fixtures:

| fixture | pins | asks |
|---|---|---|
| G1 one source meaning | the same scoped expression in a `fn`, a `def` and a theorem: numerals, `Add`, a nested `match`, `-` at `Nat` and at `Int` | R63, R66, R70 |
| G2 matcher identity and forcing | a swapped description refused (`matcher_description`); a missing one refused; an unselected arm that would exhaust is not run; a computed scrutinee evaluated once | R64 |
| G3 `Bool` beyond `if` | both values of a `decide` returned, stored in a field, matched against `true`/`false`, passed to `and`, used as a condition | R66 |
| G4 a completed measured definition | `count.dec_1` with its hypothesis; discharged; pending sets empty; the same call moved under a `let` and into an argument | R65, R69 |
| G6 outcomes | a low depth gives `unify_exhausted` and a higher one succeeds; a blocked unification names its metavariable; a refused closed assignment leaves the context unchanged | R67 |
| G7 acyclic closure | a proof citing its own obligation, and a two-obligation cycle, close nothing | R69 |
| G8 no reinterpretation | a `RUNNABLE` callee added to a body changes the record and no resolved operator, literal or constructor in the dump; a typed-route error stays a refusal | R70 |
| G9 descriptions through a view | a function with a `match` defined inside an implementation check: projected in the fork, the description present | R63, R64 |

G5 (contextual delayed filling) is rule 10's and opens slice 3.20 (I's opener).

**Risks named before code.** The table generalization of slice 3.14
was written over E patterns and E types; re-pointing it is the
largest piece and the landing where a full context window may go.
A matcher's universe (`Sort v`) is one more level for the elaborator
to infer at each `match`. Rule 8 needs a lemma of Init's library that
the fixture prefix may not reach (`Nat.sub_one_lt` or its kin): the
fixture is extended or the proof goes through what the prefix has.
The slice is larger than revision 1 by rules 8 and 9 and the
fixtures; landing 3 may split in two. If the tie test of landing 2
fails on a shape the probe did not cover, the design returns here
before landing 3.

**As built, landing 1 (2026-09-19): matchers, `match` in L terms, the
constructor by expected type, the outcomes.** `kernel/matcher.shard` —
`MtPat` (a variable with its slot, a constructor on its fields),
`mt_generate` (the motive, the scrutinees and the alternatives as K
locals; `mt_compile`, the `casesOn` tree by first-match column
splitting — a row with a variable at the split column kept under every
constructor with the variable bound to the constructor term, and at a
leaf every variable bound before a later split **refined** by it, so
the alternative's argument is the term the motive's argument has
there; `match_not_exhaustive`, `match_indexed`, `match_eliminator`),
the definition an `abbrev` over the owner's universe parameters and
one more for the motive; `mt_describe`/`mt_is_matcher` (rule 1's
read-back and regeneration check). `elab.shard` — `elab_match` (the
constant motive from the expected type, which must be determined:
`match_motive` otherwise, with the pointer), `el_pat` (a pattern at its
type: `_`, a name — the type's field-less constructor of that name,
else a variable —, a constructor on its sub-patterns;
`literal_pattern`, `pattern_constructor`, `pattern_arity`), the
matcher's parameters the binders in scope its scrutinee's type
mentions, the matcher **checked into the walk's environment as it is
made** (`kw.shard`'s `w_admit`: K's `check` seeded above the walk's
node ids, the walk's then raised above the new environment's) and
handed to the reader with the declaration (`El`'s `Gen`, `el_aux`,
`reader.shard`'s `rd_ok`), so the loader admits `OWNER.match_N` before
its owner with an `ACCEPT` record of its own. A `match` in a
requirement's statement is `match_in_requirement` (a view parameter is
one declaration); in an inductive's or a structure's types K's
`unknown_constant` answers (no consumer). Rule 5: `scope.shard`'s
`resolve_ctor` (the name under the inductive's own prefix first, then
the scope's candidates) — every pattern head, and a term's head where
the plain resolution is ambiguous and the expected type's head is an
inductive. A body's untyped `let` binding `(x TERM)` is read. Rule 9:
`kw.shard`'s `w_infer_r`, `w_whnf_r`, `w_is_def_eq_r` answer `WVal` or
`WFail` with K's `Failure`; `unify.shard`'s `URes` has `UStuck` (a
failure under an unassigned metavariable applied to what is no
pattern) and `UExhausted` (the depth, or a spent resource of K's);
an assignment is typed or **tentative** (`mc_tentative`,
`tentative_bad` asked by `el_finish`: `tentative_assignment`); a
closed value K refuses is no longer installed — except one citing a
constant the form is still declaring (`cites_undeclared`: an
inductive's name in its constructors, a structure's in its
projections), which K cannot type before the admission and the
admission checks; `check_expected` and `elab_slot` report
`unify_stuck` / `unify_exhausted` apart from `type_mismatch`. **Not
in landing 1:** the `PreDef` record (its first consumer is landing
2's erasure). Tests: `kernel/test/unify_test.shard` (G6),
`kernel/test/matcher_test.shard` (G2's identity half: the pins'
matchers recognized with their rows, a hand-written equal one
recognized, an imposter of the same type and name not); pins
`match_def`, `match_statement`, `match_poly`, `match_imposter`,
`match_not_exhaustive`, `match_literal`, `match_motive`,
`ctor_expected` (G1's constructor half).

**As built, landing 2 (2026-09-19): the pre-definition, the E
projection, the tie.** `kernel/predef.shard` — `PreDef` (identity,
declared type, the binders' locals, the **self-local**, the finished
body, the elaboration state with the matchers already in its
environment) and `read_predef` (a `fn` form read once: a `(T Type)`
binder implicit, the function's short name bound to a local of the
declared type, the body at the result type; `PdSkip` a signature with
no L reading, `PdErr` a refusal on the route); never a `Declaration`
for the owner. The erasure (still in `realize.shard` beside `tr`,
which it shares its state with; its own file when `tr` goes, landing
3) gained: a **matcher application** → the `EMatch` of the rows
`mt_is_matcher` read back and checked, each alternative's lambdas the
row's variables, an erased field's variable dropped (`erase_matcher`,
`mt_epat`); the **self-local** → `ECall` with the binders' runtime
flags (`EKCall`); `Decidable.decide p inst` → `(if ⟦inst⟧ true false)`
over Init's `Bool` — always, no identity case yet (rule 3's "where the
entry returns a `Bool` cell" waits for a consumer: `ev` has three
two-valued cells, the scope's `Bool`, Init's and `Decidable`'s);
`instDecidableEqBool c true` → `⟦c⟧`; `Int.ofNat n` → `⟦n⟧` and
`Int.negSucc` of a literal → the negative literal (one integer at run
time); rule 4's `Int` rows (`realization_of`: `Int.add`/`sub`/`mul`,
`Int.decEq`/`decLt`/`decLe` → `+ - * int_eq lt le`). `erase_predef` —
the binders' roles from their locals' types, the body erased, the
function's `EFn`. In the elaborator: a proposition where a `Bool` is
expected is `decide` with `decision_of`'s decision, a `Bool` where a
proposition is expected is `c = true` (both in `check_expected`,
beside the `Nat → Int` coercion); `if` on a `Bool` is `ite (c = true)`
with `instDecidableEqBool`; **`if` on a value of a two-constructor
inductive of the program's is the `match` it abbreviates** (§6.2's tag
rule in L: the second constructor takes the then-branch — found by
the tie, on `kernel.util.bool_and` over the prelude's `Bool`); an
operator whose first operand is a numeral takes its identity from the
other operand's type (`(le 48 c)` at `c`'s `Int` — slice 3.15 rule 3's
"first operand" amended: the first that is not a numeral); `(list a
b …)` is the constructor chain of the expected type's list (an
inductive with a field-less and a two-field constructor; no expected
type: `unsolved_implicit`); a bound head inserts its implicit binders
as a constant does (a polymorphic self-call). **The tie** (the
loader's `define_try`, `realize.shard`'s `tie_check`; gone at landing
3): every function the old route defines is also read as a
pre-definition and erased, and the erasure compared with the
classifier's program term for term — a primitive by the operation it
performs (`+` and `Nat.add` one addition), an `if` on a two-constructor
type equal to the match it abbreviates; a difference, or a stage of
the new route failing, is the refusal `tie_differs`. **It holds for
every function defined in calc's files, `v3/std/list.shard` and all
125 loader pins**, nested patterns, course-of-values and measured
functions included. Tests: `kernel/test/project_test.shard` (G3: both
values of a `decide` returned, matched, stored, used as a condition
and as a branch; G2's forcing: the chosen arm only); pins
`project_bool`, `elab_bridge` (`is_digit 53 = true` by `Eq.refl`
through `decide` and `ite (c = true)`). G1 and G8 wait for the flip:
today a `fn` with `(le 48 c)` has no L half to compare.

**As built, landing 3 (2026-09-19): the flip.** A `fn` is read once and
its program is the erasure; the classifier types no function that has
an L reading. *Decided before the old route was deleted:* the design's
"`define.shard`'s machinery re-pointed" is Lean's own structural
compilation rather than a port of slice 3.14's table bookkeeping, which
was written over E patterns and a matrix the pre-definition no longer
has.

- **The loader** (`load_fn`): `read_predef`, then `erase_predef`'s
  `EFn` — the program — then `define_pd`. `DEFINE` with the
  equations; or `RUNNABLE … why=… route=typed` when K's half cannot be
  reached (`below_unreached`, `wf_unreached`, `measure_type`,
  `callee_runnable`) — the program is still the erasure; or a refusal
  (`type_mismatch`, `match_not_exhaustive`, `ambiguous_name`,
  `pattern_constructor`, `if_type`, `private_if`, `realize_descent`,
  `realize_recursion` …: the classifier's `unsaturated`,
  `function_value`, `nonexhaustive`, `ambiguous_head`, `pattern_type`
  are its own, for the sources still on its route). The `RUNNABLE`
  record carries the route (`route=typed`, `route=e_first`, none for
  an extern or an E-only type). The matchers a body generated are
  admitted before their owner (`ACCEPT NAME.match_N`), the auxiliaries
  of a native inductive before the function is read again.
- **Rule 6 as built.** `route=e_first`, the classifier's program: a
  signature with no L reading (`why=e_only_type` — every function of
  `v3/kernel/**` but a handful); after the signature elaborates, a
  `(quote S)` or a `"…"` anywhere in the body (checked on the form,
  before the body is read), a literal pattern, a name the environment
  lacks that is no function — an operator's identity past the Init
  prefix (`Nat.add` under `(import Init Nat)`, `ite` under a prefix
  that stops before it, a `casesOn` not yet generated), a toolchain
  spelling — all `why=no_l_identity`; a callee that is an extern or
  was itself E-first by signature, `why=no_l_meaning`. A name that is
  nothing at all is still a refusal, the classifier's
  (`unknown_head`). **The set in the tree today:** calc's `show_nat`
  (`Int.ediv` and `Int.emod` lie past its file's Init prefix — the
  first cut of this sentence said a `"…"`, which the function does not
  contain; corrected 2026-09-19 at slice 3.17's scoping),
  `kernel.define.has_eq` and `confusion_reachable` (K's view's
  `env_find` answers an E-only type); slice 3.17 below says
  what empties it and what does not (as built there: `show_nat` is
  defined; `has_eq` reads `no_l_meaning`; `confusion_reachable`
  quotes symbols and stays `no_l_identity` until slice 3.18). A
  **callee-local**: a callee with a program and no K constant whose
  signature elaborates — `RUNNABLE` on the typed route, or E-first
  for its body alone — is bound, under the name the body writes, to a
  local of its signature's type, the signature elaborated in the
  callee's own scope inside the caller's walk; the body is read again
  with it, erased with the call in it, and recorded `why=callee_runnable
  route=typed`. calc's `show_ascii`, `step` and the trace functions
  are such (their root `show_nat`). A callee later in the file parks
  the function (slice 3.15's retry); at the file's end the parked
  functions are each other's callee-locals — mutual recursion is two
  programs by erasure and no definition (`define_mutual`).
- **The K projection** (`define.shard`, rebuilt; the E-term matrix,
  `Known`, the table splits and `df_*` deleted). A body that never
  cites the self-local is the value under its binders, matchers and
  `ite` as elaborated. **Structural:** `T.brecOn` applied to `λ x
  below others . body'` over the binders' own locals. **The matcher
  stays a constant and its motive carries the table** (Lean's
  `MatcherApp.addArg`): a matcher application whose scrutinee is a
  local that a table's type mentions is re-instantiated at `λ s .
  B[s] → motive s` — its universe K's sort of that arrow — each
  alternative takes the refined table `B[pattern_i]` as a last
  binder, and the application is applied to the table; nothing is
  unfolded. **A self-call is found by type:** each table is kept with
  its type over an *abstract* motive `C` (the real motive is constant
  in its argument, so `motive r` cannot tell one entry from another);
  the search reads K's normal form — `C y'` with `y'` the argument
  (syntactically, else by K's conversion) is the entry, a `PProd` is
  searched through both projections, a `T.below C t` unfolds exactly
  when `t` is a constructor application — so the shape of `below` is
  never assumed, and a self-call under a match in an argument or a
  `let`'s value finds its entry as one in tail position does (slice
  3.14's `recursion_depth` obstacle is gone). No entry:
  `realize_descent`. **Measured:**
  `WellFounded.fix` over the data binders' tuple, the body with each
  binder the tuple's component; a self-call `rec (tuple args)
  (f.dec_N locals…)` over every local in scope — pattern variables,
  `let`s (as `let`s in the statement), branch proofs; the measure's
  type is checked (`measure_type`) before `WellFounded`'s
  reachability (`wf_unreached`). **`ite` inside a recursive function**
  is `Decidable.rec` with a constant motive and the proof a local of
  each branch, wherever it stands — but only where a self-call sits
  under it: a subterm that does not cite the self-local is left as
  elaborated. A function with a measure clause that never calls
  itself is a plain definition, not `PENDING`.
- **The equations** (`realize.shard`'s `pj_equations`, shared with
  `realize`): the leading matchers' rows against the scrutinee term —
  bind, clash (anywhere in the row: the row is gone from the
  specialized matrix), or stuck on a universally quantified local,
  which is split over its constructors in order — no reduction by K.
  The leaves and the names `f.eq_N` are slice 3.13's; the statements
  now cite the matcher constants, so their hashes moved with the
  definitions'. **A `fn`'s leaf is stated where K decides it by
  conversion** (the definition is checked into the walk's environment
  first): a structural recursion whose leaf leaves the structural
  argument a local, or keeps a self-call under a matcher still stuck,
  unfolds only by a case analysis — that equation is I's, the leaf
  keeps its number, and the function is defined without it
  (`define_arg_match`; slice 3.14 left such a function `RUNNABLE`).
  A realization's equations are all stated (`realize_wrong` is K's
  refusal).
- **`if` on a two-constructor inductive is `T.casesOn`**, not a
  matcher (landing 2's as-built reversed): the erasure reads a
  two-constructor case analysis whose alternatives ignore their
  fields back as `EIf`, so the toolchain's `bool_and`/`bool_or` keep
  the old front end's program byte for byte, and the equations do not
  split on an `if`. The elaborator refuses `if_type` and `private_if`
  itself and checks that the `Bool` bridge's constants (`Eq`,
  `Decidable`, `instDecidableEqBool`, `Decidable.decide`) exist
  before it cites them (a short Init prefix is `no_l_identity`, not
  K's refusal at admission).
- **Rule 4 as built:** `Nat.add`, `Nat.mul`, `Nat.decEq`, `Nat.decLt`,
  `Nat.decLe` select the integer entries `+ * int_eq lt le` — one
  operation on naturals, and the classifier's spelling, so frontend
  parity is byte for byte; `Nat.sub`, `Nat.div`, `Nat.mod` keep their
  own (truncating, total) entries. `define_nat_operator` is `ok`.
- **Rule 7:** a supplied `realize` binds its signature's names to the
  realized constant's own telescope (an erased binder has none),
  elaborates the body at the result's L type, takes the erasure as
  the program, checks descent on it, and states the equations of its
  leading matchers (`Eq.refl`, or the clause's theorem); its matchers
  are owned by `MODULE.NAME.realize` and admitted first. A `RUNNABLE`
  callee is `no_l_meaning`, an E primitive without an L identity
  `no_l_identity`, a callee without a realization the erasure's
  `no_realization` (was `realize_order`). **`tr` is deleted** with
  what only it used — the typed application through `c_type`
  (`arg_types`, `ctor_etargs`, `apply_const_e`, the first-order
  matching of binder types), the E-term case tree, the old equations,
  `DefRec`/`BelowInfo`/`WfInfo` — and the tie. The erasure is
  `kernel/erasure.shard` (with the walk's state `Tr`);
  `realize.shard` is the supplied form, the descent check and the
  equations; `prim_typed_identity` and `c_type` remain the
  classifier's own.
- **Rule 8 as built.** `(fulfills f.dec_N PROOF)` in a file outside a
  view: PROOF read against the obligation's statement as the theorem
  `f.dec_N.proof`, checked by K. Every closure the loader computes
  replaces a discharged obligation by what its proof rests on (to a
  fixpoint), and the theorem's closure is the acyclicity check: it
  reaches `f.dec_N` — directly, through `f` (whose value cites it),
  through a theorem, or through an obligation discharged earlier by a
  proof that rests on this one — `fulfills_cycle`, nothing admitted.
  On success `DISCHARGE f.dec_N proved`, and the account is
  recomputed in the records: every `ACCEPT` that named the obligation
  names the proof's own account instead, and a function with no open
  obligation is no longer `PENDING` nor in any `pending=`. The
  fixture: `count.dec_1 : ∀ p (h : ¬ p.1 = 0), InvImage … ⟨p.1 - 1,
  p.2 + 1⟩ p`, by `Nat.sub_lt (Nat.pos_of_ne_zero h)
  (Nat.zero_lt_succ 0)` — both in the prefix through `InvImage.wf`.
- **The gate.** calc's spec file: 25 of 25 functions defined
  (`parse_tail` by its measure, two obligations `PENDING`; the file's
  Init import now reaches `InvImage.wf`); `calc_test` byte-identical
  over 21 inputs with the erasures as the programs; frontend parity
  byte-identical over 27 closures; the equations' names unchanged.
  **Hashes:** every `fn`'s definition and equations moved — the value
  cites `NAME.match_N` where it held a `T.rec` tree (the cause, for
  all of them); K's own constants and Init's are untouched (T0).
  Fixtures: G1 `one_meaning`, G4 `discharge_measure` (+ the account
  read in `define_test.sh`), G7 `discharge_self_cycle` and
  `discharge_two_cycle`, G8 `route_callee` (+ the dumps compared in
  `define_test.sh`), G9 `match_in_impl`; G2, G3, G6 are landings 1–2's;
  G5 opens slice 3.20 (I's opener). Eleven pins moved to the elaborator's reason or
  the new outcome, each with its cause in its header.
- **Open:** a `match` on a computed scrutinee inside a measured
  function states its obligation without `scrutinee = pattern` (rule
  2's narrower guarantee, unchanged); mutual recursion is not
  compiled; dependency-directed wake-up is deferred (rule 8).

#### Slice 3.17

**Slice 3.17 — the porting facilities** (design 2026-09-19, scoped by
probes through the loader before any code; RULED 2026-10-01 — the
user agreed with the three leans, rules 1, 2 and 5; built the same
day, the as-built below; §13 item 50). The slices table gives this slice R60's row (§11): record
update and order-free construction, `Name` literals, symbols and `"…"`
in a body, the fresh-name convention, E's rename of `lt le int_eq`.
Measured against the tree after slice 3.16, the row splits into what
already works, what is small, and two entries that do not belong here.

*What the probes found* (scratch files, the pins' Init prefix):

- **Records work on the typed route today.** `(record Pt (x Nat) (y
  Nat))` with `make` out of order and `with`: six functions `DEFINE`d,
  a `def` using `make` accepted, and v2's law family needs no
  generator — `(= (Pt.x (Pt.with_x v p)) v)` and `(= (Pt.y (bump p))
  (Pt.y p))` over a *local* `p` are `Eq.refl` (K unfolds a structure's
  recursor on a local through its projections, Lean's rule). What is
  left of R60's entry is its caveat — a changed dependent field an
  explicit obligation — which no `record` can state (its fields are
  positional, non-dependent types); it becomes real when `make`/`with`
  reach a `structure`.
- **`lt le int_eq` and `< <= =` already elaborate to one term**
  (`lt3` and `lt3b`: one hash). The classifier knows only the old
  spellings (`unknown_head: =` in an E-first function).
- **A literal pattern** (`(match n (0 1) (_ 2))`, at `Nat` and at
  `Int`) is `no_l_identity route=e_first`. 0 rows in `v3/`, 53 in the
  old tree.
- **`Symbol` has no L reading** (`e_only_type`); 2,905 `(quote …)`
  sites in `v3/`, every one in a function that is E-first by its
  signature anyway (K's sources see no `Init`, so `Int` has no L
  constant there). Outside the toolchain the old tree names `Symbol`
  in 4 files. `String` is in the pins' prefix and `"zero"` is a
  `String` in a `def`; `String.decEq` is past the prefix.
- **The E-first-by-body set is not what landing 3's as-built said.**
  calc's `show_nat` has no `"…"`: `Int.ediv`/`Int.emod` lie past
  `(import Init Int.decEq)`; with `(import Init Int.emod)` it is on the
  typed route (`measure_type`: its measure is an `Int`). `has_eq` and
  `confusion_reachable` call `env_find`, whose signature has no L
  reading until K's sources port — no facility of this slice changes
  that, and their record should read `no_l_meaning`, as the as-built
  describes that case, not `no_l_identity`.
- **The rename reaches the v2 chain.** `v3/kernel/**` is hosted by the
  Rust bootstrap (one evaluator for both trees) and compiled on route
  1 by `tools/lower` and `tools/codegen`, which read it with the old
  tree's kernel: 73 sites in 10 files of `kernel/` and 3 in
  `codegen.shard` name the three primitives, and `=` there is the
  claim language's equality. 943 sites in 60 files of `v3/` would
  move.

*The rules* (the design's leans, ruled 2026-10-01; rule 3 was built
differently from this text and is rewritten in place, the first
reading kept as rejected):

1. **`"…"` in a `fn` body moves to slice 3.18, with the flip.** Since
   slice 3.16 a `fn` body is an L position, and in an L position
   `"…"` is already `LitStr : String` (§5.3). Reading it as the byte
   list in a `fn` and a `String` in a `def` is two meanings for one
   text under one elaborator — guard 1. It keeps `no_l_identity
   route=e_first` until item 37's flip gives it the one meaning
   everywhere. Rejected: list-literal sugar over the bytes by the
   expected type (twenty lines, and wrong for exactly one slice).
2. **Symbols move to slice 3.18 too.** Item 36 rules `Symbol`'s L
   identity `String`. Assigned now, a `String` binder erases to the
   atom cell; at 3.18 `"…"` is a `String` as well, so the atom would
   have to *be* `String`'s E representation — the wire's ruling made
   by a side effect. No function in the tree would change route by
   it. `Name` literals go with them (K's `Name` holds the atoms; a
   literal over an E-only type helps no typed function), stated there
   as loader-level sugar over the `Name` in scope — plain data, no
   node, declaration or environment identity to forge (`name.shard`
   carries none). Rejected: a native `structure Symbol` (reverses
   item 36 for no consumer).
3. **Numeral rows on the typed route** are Lean's value patterns, one
   mechanism at `Nat` and at `Int`: the scrutinee once, then `(if (= x
   K) BODY …)` per row in order — the `ite` on the type's decision, as
   any `if` is read — and the last row `_` or a name for the
   scrutinee. No matcher is generated, and the erasure is the `if`
   chain (`(if (= #0 0) 10 (if (= #0 5) 20 30))`), the scrutinee
   bound by a `let` when it is not a local. A match of numeral rows
   with no last row is `match_not_exhaustive`. *Rejected, the design's
   first reading:* a numeral at `Nat` as the constructor pattern
   through the matcher, both types read back as E's literal row —
   because a numeral is then a `succ` chain as deep as its value (48
   nested `casesOn` for an ASCII code, against the pattern reader's
   depth), E has no successor pattern so the constructor reading buys
   a program nothing, and reading an `ite` chain back as a literal
   row is recognition of a shape a user also writes by hand. The
   classifier's literal rows stay what an E-first function runs.
4. **Records: the first example is the deliverable.** A pin
   (`record_laws`) with the three probes above as theorems; `make` and
   `with` over a `structure` whose later field's type mentions an
   earlier one: updating the earlier field without the later is the
   refusal `dependent_update`, naming the fields to supply — never an
   inserted cast.
5. **The rename, staged by who reads the file.** E's table takes `<
   <= =` as the primitives' names and the classifier accepts them; the
   Rust bootstrap accepts both spellings (it hosts the old tree); the
   files no v2 tool reads — `v3/std`, `v3/examples`, `v3/pins` —
   migrate by tool now, and the old spellings are refused there.
   `v3/kernel/**` migrates when route 1's chain is V3's own, since
   until then the old tree's kernel reads it: teaching the v2 kernel
   an alias whose `=` collides with the claim language is a change to
   the old trusted base for a spelling. Rejected: a rewriting shim in
   `build.sh` (a derived source between the text and the compiler);
   the full migration now (route 1 is testable only on CI).
6. **The fresh-name convention** is a library, not a form:
   `v3/std/fresh.shard` — a `Fresh` record (a prefix and a counter),
   `Fresh.next` returning the name and the advanced supply — with a
   theorem that two successive names differ, and the convention
   (§12.1's `gen_fresh` row) that a function needing names takes and
   returns the supply.
7. **The E-first-by-body set.** calc's app file imports through
   `Int.emod`, which puts `show_nat` on the typed route as
   `measure_type`. Its definition needs a `Nat` measure, and the
   probe found the gap: the measure is erased with the body, the
   prefix holds `Int.natAbs` (not `Int.toNat`), and `Int.natAbs` has
   no program — `(realize Int.natAbs (view))` is `no_realization`,
   its value matching on `Int`'s constructors, which E's `Int` does
   not have. The lean: a realization-registry row for `Int.natAbs`
   (and `Int.toNat` where the prefix holds it) as an E expression over
   the existing primitives — no new primitive — so `show_nat` is
   defined with its obligations `PENDING`. A callee whose signature
   has no L reading is recorded `no_l_meaning` and is the port's, not
   this slice's; rule 6's bullet stays for `"…"` and symbols until
   3.18 and is deleted there.

*Gates:* the loader pins with the new ones (`literal_pattern`,
`record_laws`, `dependent_update`, `rename_refused`), `define_test`
with calc's app file added, calc's differential and frontend parity
byte-identical, the full suite, CI for route 1.

**Slice 3.17 as built** (2026-10-01):

- **Rule 3.** `elab.shard`'s `elab_match_lit`: a `match` with a
  numeral row is rewritten to the `if` chain over the scrutinee's name
  — a local is its own name, anything else is bound once under a name
  no source symbol can spell — and elaborated as any term, in a `fn`
  and in a `def` alike. Refusals on the route: `match_not_exhaustive`
  (no last row), `literal_rows` (a row after the catch-all, a
  constructor row beside the numerals), `pattern_type` (a numeral row
  at a type other than `Nat` or `Int` — the classifier never checked a
  literal against the scrutinee's type, so `(match b (0 1) (_ 2))` at
  a `Bool` was `RUNNABLE` before this slice and is refused now where
  the signature has an L reading). `literal_pattern` stays the
  E-first fallback for a numeral *under a constructor* and for a
  symbol pattern (the old tree has 53 numeral rows and one nested
  numeral). A `fn` whose body is a literal match has one equation,
  as a body `if` does; the rows compute in K (`Eq.refl`).
- **Rule 4.** Pin `record_laws`: get after set, get of another field,
  set after set, a field through a `with` in a function, a field of a
  `make` — five theorems, each `Eq.refl`. `make` and `with` reach a
  `structure` of the file (`record.shard`, at the s-expression level
  as for a record): `make` is `NAME.mk` on the values in field order;
  `with` is `NAME.mk` on each field's entry or its projection of the
  subject, the subject bound once when it is not a name.
  `dependent_update` is syntactic and conservative: a field kept whose
  type's *source* names an updated field (a binder inside the type
  that reuses the field's name is refused too). It is a read error,
  like `record_make`: the expansion precedes the reading. **Limit:**
  a structure with parameters is out of the sugar's reach — its
  constructor takes them — and `make`/`with` on it are
  `record_make`/`record_with` with that pointer. The bootstrap reads
  no `structure` (an L form), so parity is untouched.
- **Rule 5.** E's table names the three entries `= < <=`
  (`prog.shard`); the erasure's registry rows emit them. The
  spellings before the rename are read as those entries in a module
  whose identity begins with `kernel` (`scope.shard`'s
  `toolchain_scope` — the toolchain's sources, the one tree route 1's
  v2 chain compiles) and are refused everywhere else as
  `renamed_primitive` with the new spelling, by the elaborator on the
  typed route and by the classifier on the E-first one; they stay
  reserved there, not free names, until the toolchain migrates. The
  Rust bootstrap reads both (`prim.rs`, `eval.rs`) and `eval dump`
  prints the V3 spelling whichever the source wrote, so frontend
  parity compares one name. **Found while building:** the bootstrap's
  native table and `kernel/reduce.shard`'s object table are tied by a
  conformance sweep (`lib.rs`); `= < <=` are native-only names, which
  the old kernel's table does not carry — no old-tree `fn` body
  writes them, and `=` there is a claim's head, which neither table
  sees. Stated in `prim.rs`; the sweep's lists are unchanged. The
  tool is `v3/tools/rename_cmp.py` (a list head only, never inside a
  string, a comment or a `quote`; `--check` for a migrated tree): 27
  sites in 16 files of `v3/std`, `v3/examples` and `v3/pins`. By its
  count `v3/kernel` has 665 sites in 52 files still to move (the
  scoping's 943 in 60 was a grep that also counted comments, strings
  and quoted symbols).
- **Rule 6.** `v3/std/fresh.shard`: the records `FreshName`, `Fresh`
  and `Minted`, `Fresh.start`, `Fresh.next`, and `Fresh.next_ne` —
  the names of two successive steps differ, through
  `Nat.succ_ne_self` and `congrArg` on the index (no axiom). The
  prefix's type is a parameter `P`, not the design's unstated one:
  text has no L reading before slice 3.18, and the toolchain's port
  will instantiate it with K's `Name`. 14 functions defined.
- **Rule 7.** The registry has expression rows (`erasure.shard`'s
  `realization_expr`): `Int.natAbs a` erases to `(let (a) (if (< #0 0)
  (- 0 #0) #0))`. calc's app file imports through `InvImage.wf` and
  `show_nat` takes the measure `(Int.natAbs n)`: 17 of 17 functions
  defined, `show_nat.dec_1` the one pending obligation, nothing
  `RUNNABLE`. `Int.toNat` has no row: it lies at line 552,397 of the export,
  far past the test prefix (corrected at slice 3.18 — this said it was
  not in the export).
  A callee that is a view's `sig fn` the environment lacks is
  `no_l_meaning` (`loader.shard`'s `callee_hits`; pin
  `route_sig_callee`): `has_eq` reads so now. `confusion_reachable`
  stays `no_l_identity`, correctly — its body quotes symbols; the
  scoping note above named it beside `has_eq` for the callee alone.
- **Gates run:** 146 loader pins (14 new: `literal_pattern`,
  `literal_open`, `literal_type`, `literal_rows`, `literal_nested`,
  `rename_refused`, `rename_refused_e`, `rename_ok`, `record_laws`,
  `struct_make`, `dependent_update`, `struct_params`,
  `measure_natabs`, `route_sig_callee`; `match_literal` narrowed to
  the nested numeral), `define_test` (7 checks: calc's app file and
  the fresh-name library added), calc's differential byte-identical
  over 21 inputs, the full suite and the bootstrap's unit tests
  (records §9 for the figures). Route 1 is CI's.

#### Slice 3.18

**Slice 3.18 — bytes and text: the wire's type, `String`'s
realization, the flip** (the user's ruling of 2026-10-01 on five
leans, the design the same day; §13 item 51). The slices table gave
this slice the byte and text adapters, item 37's flip, the literals
moved from 3.17 and deriving, "the wire's type ruled first".

*The ruling.* (1) **The representation is the byte list**: `UInt8`
runs as an integer, a `ByteArray` as the list of them, a `String` as
its `ByteArray` with the validity proof erased — no executor gains a
kind of value. Rejected: a packed buffer now (a fourth value in the
bootstrap and in `ev`, and a table of primitives each *trusted* to
match Lean's function; its benefit is the lowering's, behind the same
types); the interned atom (item 36's reading taken literally: every
string ever built interned, a file's contents included). (2) **The
externs carry `ByteArray`**, all six; text leaves by its bytes and
enters by a checked decoder. Rejected: `List UInt8` (list cells in the
host contract for good), `String` for paths and arguments (UTF-8
validation inside the boundary). (3) **Symbols are deferred** — they
and `Name` literals stay out of a `fn` body; item 36 is amended.
(4) **The flip is staged like the rename** — every file outside the
toolchain's own sources now, `v3/kernel/**` with route 1's chain; item
37's "at once" is amended. (5) **Deriving is its own slice**, 3.19; I
moves to 3.20.

*What the probes found* (scratch files; the pins' prefix and a longer
one cut from the export):

- **In a `def`, `"…"` is already a `String`**; a `fn` whose signature
  names `String`, `ByteArray` or `UInt8` is `no_l_identity
  route=e_first`. The obstacle is one binder: `UInt8` is a structure
  over `BitVec 8`, `BitVec w` over `Fin (2 ^ w)`, and a *value*
  parameter is neither a type nor a proposition, so §7.5's eligibility
  refuses `BitVec` and everything built on it.
- **A structure's projection in a `fn` body is refused today**:
  `(fn lo2 ((s Span)) Nat (Span.lo s))` is `REFUSE unknown_head:
  Span.lo` — the erasure has no realization for the projection
  function, the function falls to the classifier, and the classifier
  knows no L constant. Slice 3.17's structures were reachable from a
  `fn` by constructor and by pattern only.
- **An import costs its position in the export.** `(import Init
  NAME)` admits the prefix through NAME: `String` is line 9,848 (277
  declarations), `String.toByteArray` 78,604 — both inside the pins'
  prefix (100,851 lines, 5 s). `ByteArray.data` is 169,395,
  `UInt8.toBitVec` 250,325, `String.casesOn` 250,602, `String.append`
  253,597, `UInt8.casesOn` 263,243, `UInt8.toNat` 263,515: a load
  through there admits 3,400 declarations in 40 s on the bootstrap.
  `ByteArray.validateUTF8` is line 2,619,670 of 6,490,422.
- **Init's operations on these types have no derived view**: they are
  stated through instance projections (`HMod.hMod`, `++`) and loops
  the erasure refuses (`recursion_structure`). `UInt8.ofNat n` is
  `⟨BitVec.ofNat 8 n⟩`, which unfolds to `⟨⟨⟨n % 2 ^ 8, _⟩⟩⟩`.
- **The driver performs an extern by its short name alone**
  (`ev.shard`'s `perform`): prelude cells whatever the program
  declared.
- **Outside the toolchain's sources one function holds a string
  literal** (the pin `ev_string`); `v3/std` and `v3/examples` hold
  none.

*The rules:*

1. **Type realizations: six rows of the registry.** `Fin`, `BitVec`,
   `UInt8`, `Array`, `ByteArray` and `String` are **represented by
   their one runtime field** (law §4.4, "a runtime representation
   different from the constructors"; the simulation is the identity on
   that field, the erased fields its invariant). In E: the type keeps
   its name (`ByteArray`, not `(List UInt8)` — the lowering may give
   it another layout behind the same signatures), its type arguments
   kept and its value arguments dropped (`Fin n` is `Fin`); the
   constructor is its runtime argument, the field's projection its
   subject, a case analysis or a constructor pattern binds the field
   to the scrutinee. So a `UInt8` is an integer below 256, a
   `ByteArray` and an `Array α` are `Init`'s `List` cells, a `String`
   is its UTF-8 bytes. A row's type is **never an E inductive**: the
   classifier cannot cite its constructor (`unknown_head`), so the
   typed route and the E-first route cannot disagree on the cells.
   The erasure checks the shape at each use — one constructor, exactly
   one runtime field — and is `no_realization` otherwise; a row cannot
   misdescribe its type. No row for an **operation**: an operation is
   an S function over the constructors and projections, tied to
   Init's by a theorem where one is wanted. Rejected: the rule for
   every one-field structure (a native `structure` or `type` would
   change cells under the E-first functions that build it); rows by
   operation (`UInt8.ofNat` ↦ `mod`, each a trusted line).
2. **A projection function is its projection**, for every structure:
   a constant whose value is `λ params self. self.i`, fully applied,
   erases as that projection of its last argument — the match on the
   constructor for an E inductive, the subject for a row of rule 1.
3. **The literal.** Outside the toolchain's sources `"…"` is a
   `String` in every position. On the typed route it is K's literal,
   erased to its bytes as `Init`'s `List` cells — one literal, built
   once at link; in an E-first body the same literal, its static type
   `String`; `no_string` where the environment has no `String`. A
   literal whose bytes are not UTF-8 is `bad_string`: K reads a
   literal by decoding it, and only on valid bytes is the encoding of
   the decoded text the bytes written. In a module whose identity
   begins with `kernel` the rule before the flip holds (the byte list
   of the `List` in scope; a body holding one stays E-first), until
   route 1's chain is V3's own and the tool migrates the toolchain.
4. **The wire by declared type.** The driver builds and reads each
   value of an extern by the E type the program declared — a codec by
   identity, as §9's entry is: bytes at the prelude's `(List Int)` or
   at `ByteArray`, a list at the prelude's `List` or `Init`'s, an
   option, a pair (`Pair` or `Prod`), a flag (either `Bool`). The
   toolchain's signatures are `kernel/host.shard`'s, unchanged; a
   program under the naming law declares
   `get_args : World → Prod (List ByteArray) World`,
   `read_file : ByteArray → World → Prod (Option ByteArray) World`,
   `write`, `write_line : ByteArray → World → World`,
   `write_file : ByteArray → ByteArray → World → Prod Bool World`,
   `exit : Int → World → World`. A declared type the host's value
   does not fit is stuck (`extern`), as a value that was no byte list
   is today. The checked entry takes a `ByteArray` parameter.
5. **Symbols.** `(quote x)` in a function whose signature has an L
   reading keeps `no_l_identity route=e_first`. The direction, for
   the ruling that the toolchain's port will need: `Symbol` its own
   type over `String`, represented by the atom — a seventh row whose
   simulation is interning — rather than `String` itself, which rule
   1 makes a byte list.
6. **The libraries and the consumer.** `v3/std/bytes.shard` (bytes
   and numbers, a byte array and its list), `v3/std/text.shard` (a
   text's bytes; built into `bytes.shard`, the as-built below),
   `v3/std/host.shard` (the `World` and the six
   externs of rule 4), and calc's loop closed against the host:
   `v3/examples/calc/calc_main.shard`, an S program that reads its
   arguments and prints each line's result — the consumer §12.6 names
   ("calc's app step"). Its gate: its output on the differential's
   inputs is the model world's (`run_world`, which the differential
   already ties to the old tree), byte for byte.

*Not in this slice:* the checked decoder (`String.fromUTF8?` needs a
validator with its proof; no consumer turns input bytes into a
`String`, and the proof is I's); rows for `Char` and the wider
`UInt`s; `String`'s own operations (append, length, the decision);
a packed buffer; deriving (slice 3.19).

**Slice 3.18 as built** (2026-10-02):

- **Rule 1.** `erase.shard`'s `rep_row` names the six types.
  `l_to_etype` gives a row its own name over its type arguments
  (`row_param_roles`: a value or a proposition parameter dropped) and
  `ind_eligible` answers no for it, so the E table never holds its
  constructor. `erasure.shard`: the constructor is its runtime
  argument, a `Proj` its subject, `T.casesOn` and `T.rec` a `let` of
  the scrutinee (`erase_row_cases`), a constructor pattern in a
  matcher's row its field's pattern — `row_field` checking at each use
  that the structure has one constructor and one runtime field. The
  classifier's pass treats a row's type as a known type with no
  constructor of its own: a constructor pattern on it is
  `pattern_type`, an `if` on it `if_type` (pin `row_e_match`) — an
  E-first body may pass a row's value on, never read its cells. As
  dumped: `(UInt8.ofBitVec (BitVec.ofFin (Fin.mk (Nat.mod n 256)
  PROOF)))` is `(Nat.mod #0 256)`; `ByteArray.mk (Array.mk l)`, its
  pattern and `String.toByteArray s` are `#0`. `Array` was an E
  inductive with a `mk` cell since slice 5b and is a row now (pin
  `realize_view`: `Array.toList` and `Array.push` by their views, as
  before).
- **Rule 2.** `proj_fn_app`: a definition whose value under exactly
  as many lambdas as it has arguments is `self.i`. `(Span.lo s)` in a
  `fn` is `(match #0 ((mk _ _) #1))` and the function is defined (pin
  `struct_proj`). The rule is also what lets a body call Init's
  `String.toByteArray`, `Array.toList`, `Fin.val` and
  `ByteArray.data` with no `realize`.
- **Rule 3.** A `LitStr` erases to the one literal over `List.nil` and
  `List.cons`. A literal whose bytes are not UTF-8 is `bad_string` at
  the **elaborator**, in a `def` as in a `fn` (one text, one meaning;
  `utf8_ok`, moved from `ev.shard` to `util.shard`), and the erasure
  keeps the check for the bytes it emits; the classifier reads `"…"`
  the same way outside the toolchain's sources, `no_string` without a
  `String` in the environment; the linker interns Init's
  cells whether or not the program declares them. The pre-definition
  keeps a string-holding body E-first in the toolchain's sources only.
  No file of the toolchain changed meaning and the bootstrap is
  untouched: parity compares what it compared. Migrated: the pin
  `ev_string`, the one function outside the toolchain that held a
  literal (it was `(fn g () (List Nat) "abc")`, `type_mismatch` now).
- **Rule 4.** `ev.shard`: a linked extern carries its declared types
  and `perform` builds and reads each value by them (`wb_tags` for
  bytes, `wl_tags`, `wo_tags`, `wp_first`, `wf_tags`); the result's
  type is decided **before the host is asked** — its codec, and the
  World's token returned at the type it was passed at (the extern's
  last parameter's): a `write_file` whose declared result has no
  codec writes nothing, a `write_line` declared to return a pair
  prints nothing. An extern whose declared type does not fit is
  stuck, `RUN: stuck extern in main.write_line` (pins `wire_bad`,
  `wire_bad_result`). The checked entry takes a `ByteArray` (pin
  `wire_s`'s `copy`). **Not built: a check at link.** An extern's
  signature is checked where it is performed, because `ev` is pure and
  stops at any extern — a program may declare one the host does not
  have (`define_runnable`'s `read_byte`, `ev_test`'s `shout`).
- **Rule 5.** As designed; the elaborator's and the pre-definition's
  messages name the deferral.
- **Rule 6, built as two libraries, not three.** `v3/std/bytes.shard`
  holds text's one adapter too (`Bytes.ofString`): a `text.shard` of
  one function was a file for its name. In it: Init's `BitVec.toNat`
  and `UInt8.toNat` by their derived views (`(fn UInt8.toNat 0 (UInt8)
  Nat (BitVec.toNat 8 #0))` — the width is an argument of Init's
  function and stays one); `Byte.ofNat`, with `Byte.ofNat_eq :
  Byte.ofNat n = UInt8.ofNat n` and `Byte.toNat_ofNat`, both
  `Eq.refl` (K unfolds Init's instance projections, which the erasure
  cannot follow); `Bytes.ofList` and `Bytes.toList` with both round
  trips (`ofList_toList` is K's structure eta); `Bytes.ofNats`,
  `Bytes.toNats`. Two realized, eight defined, four theorems, no
  axiom. `v3/std/host.shard`: the `World` and the six externs.
  `v3/examples/calc/calc_main.shard`: `codes_of` and `bytes_of`
  defined, `drive_host` typed with a callee-local, `perform` and
  `main` `no_l_meaning`. Run with the differential's 21 lines as its
  arguments it prints the model world's lines, byte for byte
  (`kernel/test/calc_main_test.sh` decodes `run_world`'s value from the
  harness).
- **Found by review** (a second reader over the kernel's diff, its
  probes re-run before the fixes). *An E-first body could take a row
  apart*: `(match b (List.nil 0) ((List.cons h t) h))` on a
  `ByteArray` was `RUNNABLE`, the classifier's pass skipping a type
  it had no entry for — the cells agreed, so nothing ran wrong, but
  the layout was no longer the registry's to change. *`write` and
  `write_line` ignored the declared result*: declared to return a
  pair, the line was printed and the program stuck at the next
  `match`; and a pair's second component was never compared with the
  World's type. Both closed as above. Left as it was, and older than
  this slice: an E-first body is not checked against its declared
  result (§12.1's "return types unchecked" row), so one can return a
  `(List Nat)` as a `ByteArray` and `write` emits each number's low
  byte.
- **Found while building.**
  *The fallback hid the obstacle.* `(fn cat … (ByteArray.mk (Array.mk
  (List.append a b))))` was `REFUSE unknown_head: ByteArray.mk`: the
  typed route's obstacle was `List.append` with no realization, the
  function fell to the classifier, and the classifier's refusal — a
  row's constructor — was true and beside the point. A classifier
  refusal after a typed-route obstacle now carries it (`loader.shard`'s
  `load_e_after`): `… — read E-first after the typed route's
  no_realization: List.append: no realization attached`.
  *An import costs its position.* The libraries import through
  `UInt8.toNat` (40 s a load on the bootstrap, against 5 s): the test
  prefix has a second chunk, `fixtures/init_prefix_str_tail.ndjson`
  (lines 100,852 to 263,515 of the export, 8.6 MB; to line 274,616
  since slice 3.21, for `arith`'s product and quotient), given after the
  first by the three tests that load them; the pins stay on the first.
- **Open, stated.** The checked decoder (`String.fromUTF8?`: a
  validator and its proof against `ByteArray.IsValidUTF8`, with I).
  Rows for `Char`, the wider `UInt`s and `USize`. `String`'s own
  operations. A `realize` by a proven-equal S function, so that a body
  calls Init's name (`UInt8.ofNat`) rather than a sibling
  (`Byte.ofNat`) — the supplied form's body is E and cannot build a
  row's value. The toolchain's flip and symbols, with route 1's chain.
  The six rows and the literal's bytes are lines of the erasure and
  trusted with it (§7.1).
- **Gates run:** 156 loader pins (10 new: `row_types`, `row_e_first`, `row_e_match`, `struct_proj`, `str_e_first`, `str_no_string`, `str_bad_utf8`, `wire_s`, `wire_bad`, `wire_bad_result`; `ev_string` rewritten), `rows_test` (10 values under `ev`), `wire_test` (15 checks against the host), `calc_main_test` (byte-identical to the model world over 21 inputs), `define_test` (9 checks: the two libraries added), calc's differential byte-identical over 21 inputs, parity byte-identical over 28 closures and 111,681 declarations, route 2, K's clients, the full suite's 32 entrypoints with 0 failed. `v3/build.sh` built these sources into a scratch binary that byte-ties the interpreter on the 3,000-line fixture.

#### Slice 3.19

**Slice 3.19 — deriving under a declared policy** (the user's ruling
of 2026-10-02 on five leans, the design the same day; §13 item 52).
Law §5.1 gives a new type decidable equality, an ordering and a
diagnostic rendering without a hand-written instance, as *generation
under a declared policy*. Before this slice `(if (= a b) …)` at a
type of the program's was `no_decision`: the elaborator's lookup knew
`Nat`, `Int` and `Bool`.

*The ruling.* (1) **A use site finds a derived operation through a
derivation table**: one entry per type and capability, no search, a
second visible entry a refusal; `=` in an `if` or where a `Bool` is
expected consults it. Rejected: named functions only (the type is
not "usable in an `if`", and every site is rewritten at Stage 3);
instance resolution pulled forward (Stage 3 is a slice of its own).
(2) **One named policy, `structural`, written at the derivation** —
constructors in declaration order, fields left to right,
lexicographic: Lean's own `deriving Ord`. Rejected: a package default
(V3 has no package-level form, and one policy needs no default).
(3) **The ordering is `compare`, returning Init's `Ordering`, with no
laws.** Rejected: the laws now — each is a proof term over the squared
or cubed constructor cases without I, an ordinary tactic proof with
it; they come with the first verified keyed map. (4) **The rendering
is bytes in canonical S** — `(TPair (TNum 3) TPlus)`. Rejected:
Lean's `Repr` (a pretty-printer document type and its layout engine);
a `String` (a computed `String` needs `String.append`, whose equation
is I's); deferring it. (5) **Supported now: a type without
parameters, directly recursive or not, and Init's `List`, `Option`
and `Prod` at closed arguments.** Rejected: user types with
parameters and types nested through `List` now (the constructor-
structure bundle of a parametric type and recursion through a nested
recursor first — the toolchain's `SExpr` and `Expr` wait for them).

*The rules.*

1. **The form.** `(derive TYPE CAPABILITY…)`, a declaration of the
   file it stands in. TYPE is a closed type: a name, or an inductive
   applied to closed types — `Tok`, `(List Tok)`, `(Prod Nat Tok)`.
   A capability is `eq`, `(ord structural)` or `render`, each
   generated; or `(eq by NAME)`, `(ord by NAME)`, `(render by NAME)`,
   each registering a procedure written by hand, after its type is
   checked (`derive_by`) — at any closed type, since it needs no
   constructor. The generated functions live in the
   deriving module under the type's mangled name — its head's last
   component, then its arguments' — so `Tok.decEq`, `List_Tok.compare`.
2. **Generation is source.** A derivation writes `fn` forms and the
   loader reads them as if the file held them at that point: the one
   front end elaborates them, K checks the definitions and their
   equations, the programs are their erasures, and each has its
   ordinary `DEFINE` record. Nothing is trusted and no second route
   to a program exists (guard 1). What the generator reads of the
   type it reads from K's environment — the constructors and their
   fields' types at the type's arguments.
3. **`eq`** is `NS.decEq ((a T) (b T)) (Decidable (= a b))`: a match
   on `a`, then on `b`, a row for every pair of constructors. The
   same constructor decides its fields left to right by their
   procedures — `Nat.decEq`, `Int.decEq`, `instDecidableEqBool`, the
   table's entry, or the function itself on a recursive field —
   through `dite`, the result `isTrue` by congruence from the fields'
   equations and `isFalse` by the constructor's injectivity (its
   `C.inj`, or the projections of a one-constructor type); two
   different constructors are `isFalse` through `NS.ctorIdx`, the
   constructor's index, generated beside it (Lean 4.33 gives every
   inductive one) as a derivation of its own, `cap=idx`, which later
   derivations at the type share through the table. The rows are
   quadratic in the constructors, as Lean's handler was before its
   index rewrite. A generated variable is no symbol a source can
   write (`x 1`), so no constructor of a field's type captures it.
4. **`(ord structural)`** is `NS.compare ((a T) (b T)) Ordering`:
   the same constructor compares its fields left to right, the first
   that is not `eq` decides; different constructors compare by
   `NS.ctorIdx`. A `Nat` or `Int` field is
   `(if (< x y) lt (if (= x y) eq gt))`, which is Init's `compare`
   there unfolded; any other field its table entry. The declaration
   order *is* the convention: it is in the value (the indices), so
   reordering the constructors changes the function, its hash and
   every theorem that rests on it.
5. **`render`** is `NS.renderOnto ((a T) (rest (List Nat))) (List Nat)`
   — the value's text before `rest`, linear in its size — and
   `NS.render ((a T)) ByteArray`, which is `Bytes.ofNats` of it on
   the empty rest (`v3/std/bytes.shard`; without the library in scope
   the derivation is `derive_needs`). A constructor with fields is
   `(NAME F…)`, one without is `NAME`, the name its last component. A
   field renders by its table entry; `Nat`, `Int`, `UInt8` and
   `String` have theirs in the library. A field that is erased — a
   proof, a type — or whose type is a view's opaque type prints `_`
   (law §5.1: "an opaque placeholder rather than claim to inspect
   every value").
6. **The table.** An entry is (type, capability, procedure, deriving
   module); its policy is in its record. The type is keyed in normal
   form — a definition that stands for a type is the type. An entry
   is visible in a file exactly when its deriving module and its
   procedure are — both in the file's import closure, whatever was
   loaded before the file — so it resolves as a name does and
   crosses no view (§8.5). A view exports a
   capability as what it is, a `sig fn` of the capability's type, and
   a consumer registers it: `(derive lib.Handle (eq by
   lib.Handle.same))`. `eq` at `Nat`, `Int` and `Bool` are the
   elaborator's own rows, as before; `ord` at `Nat` and `Int` is
   rule 4's expression. Everything else is an entry a `derive` made:
   the library derives `Bool`, registers `UInt8`, `ByteArray` and
   `String`'s hand-written procedures and the renderers of `Nat` and
   `Int` (`v3/std/derive.shard`); a program derives its own types and
   the instantiations it uses — `(derive (List Tok) eq)` — because E
   has no function values: `List`'s equality cannot take its
   element's as an argument, so each instantiation is its own
   first-order function. Each derivation is recorded: `DERIVE PROC
   type=T cap=C policy=P fields=PROC,…` — the field procedures it
   selected, which the definition also cites, so they are in its
   dependencies and its hash (law §5.2: a selection is recorded,
   never rediscovered).
7. **The use site.** `(if (= a b) …)` and `(= a b)` where a `Bool`
   is expected, at a type with a visible `eq` entry, take the entry's
   procedure as the decision; two visible entries are
   `ambiguous_decision`; none is `no_decision`, now with the pointer
   to `(derive T eq)`. `compare` and `render` have no operator
   spelling: a program calls `Tok.compare` and `Tok.render` by name
   until Stage 3 resolves them.
8. **Refusals**, the type untouched in every case (law §5.1):
   `derive_type` — TYPE is not a closed inductive type visible here
   (a view's sig type has nothing to generate from: the module derives
   and exports, the consumer registers, §8.5), or it is a
   proposition, `Nat` or `Int` (the integer), or an inductive applied
   to a value, `(Fin 3)`;
   `derive_shape` — an indexed or mutual type, a type without
   constructors, a type nested through another (a field `(List T)`
   of `T`), a field whose type depends on
   an earlier one, an erased field under `eq` or `ord`; `derive_field`
   — a field's type has no entry for the capability, naming the
   constructor, the type and the form to write; a function-valued
   field (no decidable equality; and the type has no program to
   render); `derive_needs` — a constant the generated source cites is
   not in the environment (`Decidable`, `Ordering`, `congrArg`, a
   constructor's `inj` for a type with parameters, the library);
   `derive_duplicate` — an entry for the type and capability is
   already visible; `derive_by` — the named procedure's type is not
   the capability's; `name_taken` with `derive_failed` — a constant
   already bears a name the derivation generates: it is never taken
   for the derivation's, and nothing is registered.
9. **What the elaborator gains for it**, each a general rule. A
   `match` on a local the expected type mentions generalizes it —
   the motive `λ d. RET[d]`, each row read at RET with its pattern
   (Lean's rule for the expected type; the motive was constant
   before, so a result typed by its scrutinee could not be written) —
   which is how `decEq`'s `Decidable (= a b)` is refined row by row.
   A hypothesis about the scrutinee is not generalized (Lean reverts
   those too): a match whose rows cannot be read at the generalized
   type is read with the constant motive, as before. An operator inside an
   E-type position — a `fn`'s `(Decidable (= a b))` — takes its
   levels by unification, not the position's level 0.

**Slice 3.19 as built** (2026-10-02):

- **Rules 1, 2 and 6.** `kernel/derive.shard` is the generator: it
  reads the form, elaborates TYPE, opens each constructor at the
  type's arguments through K, selects a procedure per capability and
  field, and writes the `fn` forms as s-expressions.
  `loader.shard`'s `load_derive` hands each to `load_decl` — the path
  of a form of the file — after registering its head as the file's
  own heads were (an E-first body finds the function too), and
  registers the entry once K has the definition: the table is
  `Load.derived`, carried to the elaborator in the scope's visible set
  (`scope.shard` `Derived`, `derived_for`), the record `DERIVE`. A
  generated function that was not defined registers nothing
  (`derive_failed`, beside its own record). The generated source cites
  every constant by its identity — an imported one under `Init.` —
  and the type the same way, so it reads the same whatever the file
  opened: a type whose constructor bears its name
  (`(type Card (Card …))`) is ambiguous written bare in a term. A
  `derive` stands before its first use; all its capabilities are
  analysed before anything is generated, so one refused field refuses
  the form.
- **Rule 3.** As designed; `(derive Tok eq)` at
  `(type Tok (TNum Nat) (TPlus) (TPair Tok Tok))` erases to the match
  on both arguments an author would write, `(if (Tok.decEq #3 #1) (if
  (Tok.decEq #2 #0) (isTrue) (isFalse)) (isFalse))` under the pair's
  row. Distinct constructors: `Nat.ne_of_beq_eq_false` at the two
  index terms, `congrArg NS.ctorIdx h` its premise — no `noConfusion`,
  so Init's parametric types need none of the bundle. Injectivity:
  `C.inj` where the environment has it (the bundle's, Init's
  `List.cons.inj` and `Option.some.inj`), else the projections of a
  one-constructor type (`Prod`, whose `Prod.mk.inj` lies at line
  1,985,485 of the export). **Wider than the ruling's fifth lean:** a
  one-constructor type with parameters derives through its
  projections (pin `derive_params`'s `Wrap`); with two constructors it
  is `derive_needs`, naming the missing `inj`. Init's own
  parameterless types derive as the program's do (`Bool`, `Ordering`).
- **Rule 4.** As designed; the wildcard row of a constructor that is
  neither first nor last compares `(NS.ctorIdx b)` with its own index.
  `Ordering.casesOn` is among the constants the generated source needs
  (line 172,785 of the export).
- **Rule 5.** As designed. A list renders by its constructors,
  `(cons a (cons b nil))`, not as `(list a b)`. **Narrower than the
  ruling's fourth lean:** a function-valued field is refused
  (`derive_field`), not printed `_` — a type that holds a function is
  not an E type, so it has no program and no value to render.
- **Rule 7.** As designed, and one refusal the design did not have:
  on the E-first route — a body that calls an extern, until phase 4's
  World — `=` is the classifier's integer primitive, and at an
  inductive it was accepted and stuck at run. It is `prim_type` now,
  with the pointer to call the decision by name (pin
  `derive_e_first`).
- **The library** (`v3/std/derive.shard`, importing through
  `UInt8.toNat`): `Bool`'s ordering and rendering derived; `UInt8`'s
  equality by its bit vector's number (`BitVec.eq_of_toNat_eq` and K's
  structure eta), `ByteArray`'s by its list's — `(derive (List UInt8)
  eq (ord structural))` — and `String`'s by its bytes'
  (`String.toByteArray_inj`), each with its ordering, registered by
  `(CAPABILITY by NAME)`; the renderers of `Nat` (decimal, its measure
  discharged by a term over `Nat.div_lt_self`), `Int`, `UInt8` and
  `String` (quoted, the reader's escapes). Fourteen procedures and
  one constructor index, 26 functions defined, no pending obligation
  and no axiom of the library's — `propext` reaches the decimal
  renderer and its two callers through Init's `Nat` lemmas in the
  descent's proof. **Wider than the
  lean**, which gave these three types equality only. `ByteArray` has
  no renderer.
- **Not built.** Lean's instance constants: `T.decEq` has `DecidableEq
  T`'s type unfolded and is the constant Stage 3 registers; `instOrdT
  : Ord T` would hold a function, has no program and no reader before
  Stage 3, which writes it in one line over `T.compare` — the lean
  said the generated constants would have Lean's instance types, and
  for the ordering that waits. The ordering's laws (the ruling). An
  entry crossing a view by itself: the view states the procedure and
  the consumer registers it (below). Types with parameters and two
  constructors or more, nested and mutual types (the ruling). A
  package default for the policy. Operator spellings for `compare`
  and `render`. **The toolchain's own sources cannot derive yet**:
  the bootstrap reads their E half and generates nothing (`derive` is
  its `UnknownForm`), so `v3/kernel/**` keeps its hand-written
  `*_eq` and `show_*` until route 1's chain is V3's own — as the flip
  and the rename wait (items 37, 50).
- **Found while building.** (a) *An `if` whose condition is a
  self-call was refused*: `(if (f x) …)` is `ite (f x = true) inst …`,
  and `define.shard`'s `pj_ite` rewrote the decision and left the
  proposition holding the self-local — `K refused the definition
  (fvar_in_value)`. Both go through the walk now (pin `if_rec_call`).
  (b) *Under a measure that call asked for two proofs of one fact* —
  it stands in the proposition and in the decision: an obligation is
  named once per statement (`obl_stating`), so `f.dec_2` no longer
  duplicates `f.dec_1`, nor do two identical calls in one scope (pin
  `measure_if_call`). (c) *`and`, `or`, `not` on `Bool`s cited
  constants the export does not have*: Lean 4.33's are `Bool.and`,
  `Bool.or`, `Bool.not`; a `def` was `unknown_constant: and`, a `fn`
  `unknown_head: and`. The elaborator cites the real names and the
  registry has their rows — `(if a b false)`, `(if a true b)`, `(if a
  false true)`, the `if` each definition is — and `a = b` at `Bool`
  in general position, the test `(if a b (if b false true))`, which a
  derived equality's `Bool` field needs (it was `no_realization` off
  `= true`). Evidence as the registry's other
  rows have it: every row by `Eq.refl` on K's side (pin `bool_ops`),
  the programs run by `derive_test.sh`. (d) *A `fn`'s result
  `(Decidable (= a b))` could not be read*: an operator inside an
  E-type position took the position's level 0 for `Eq` (rule 9). (e)
  *The refusal for (d) named the wrong thing*: a signature with no L
  reading falls to the classifier, whose `unknown_type: =` hid the
  cause; a classifier refusal after a skipped signature now carries
  what the signature's reading met (`load_e_skipped`), as slice 3.18
  did for a body; so does one after an unknown constant that is no
  function — `(or a b)` under a prefix that stops before `Bool.or`
  was `unknown_head: and` and now names `unknown_constant: Bool.or`. (f) *An explicit numeral argument is still
  unassigned when a later argument is checked* (§5.1's "numerals
  last"): `(@Nat.ne_of_beq_eq_false 0 1 (Eq.refl false) h)` is
  `type_mismatch` at the third argument. **Left as it is**; the
  generator writes the index terms, which are closed.
- **Across a view** (rule 6; §8.5's "the module derives and
  exports", pin `derive_sig`): the implementation derives its type
  and defines `Handle.same` as the derived decision; the view states
  `(sig fn Handle.same ((a Handle) (b Handle)) (Decidable (= a b)))`;
  the consumer registers it with `(eq by lib.Handle.same)` — a
  parameter there, accepted as any constant of the capability's type —
  and its own `Job` over a `Handle` derives through the entry, its
  decision calling the parameter. For the view to hold that signature
  the classifier's type reader takes `(Decidable P)` as the type's two
  cells, its proposition unread (the erasure's reading; it was
  `unknown_type: =`).
- **Rule 9** is general: a proof by cases is a `match` in S now (pin
  `match_dependent`: `not (not b) = b`, one row per constructor).
- **Found by a second reader over the diff**, each reproduced, fixed
  and pinned. (a) *A registry row spoke for any constant bearing its
  name*: a module `Bool` declaring `or` under a prefix that stops
  before Init's ran as the row's `(if a true b)` while K proved the
  module's function — a clean load, a program that disagreed with
  its definition. The name-keyed rows now hold for a constant that is
  not a native declaration (`erase_const`'s `k`; pin
  `registry_native`). (b) *A `Decidable` had two runtime
  representations, since before the slice*: the registry's decisions
  erased to the bare test in every position, so a match by
  constructor on `(Nat.decEq a b)` took the catch-all row while K
  proved the other; the slice made such values ordinary (a `fn` may
  return one). A decision is now its test only where an `if` reads it
  — `ite`, `dite`, `decide`, a `casesOn` read back as an `if`
  (`erase_cond`) — and `(if TEST isTrue isFalse)` as a value; the
  programs of every existing `if` are unchanged (pin
  `decidable_cells`, run on the host against K's theorems). (c) *A
  derivation took a constant that bore its name for its own*: a
  hand-written `T.ctorIdx` or `T.compare` was reused or registered
  under `policy=structural`. A generated name that is taken is
  `name_taken`, the index is shared through the table only, and a
  failed form registers nothing (pin `derive_name_taken`). (d) *An
  entry registered by `(CAPABILITY by NAME)` was visible wherever the
  procedure was*, so a file's reading depended on what the loader had
  been given before it; an entry carries its deriving module (pin
  `derive_visible`). (e) *Rule 9 broke rows it should read and proofs
  that loaded before*: a `_` sub-pattern's local was outside the
  scope a metavariable's value may mention (it is bound now, under a
  name no source spells); a row that used a hypothesis about the
  scrutinee no longer typed (the constant motive is the fallback).
  (f) *`eq` at a container of a `Prod` was refused by its own
  source*: `(@List.nil (Prod Nat Nat))` asks `Type ?u` of `Sort (max
  1 1)`, and level unification compared a closed level unnormalized —
  older than the slice, fixed in `unify_level`. (g) *A self-call in a
  branch under a self-call condition, under a measure*: its
  obligation quantified the branch's hypothesis, which states the
  function's own value (`dec_2 fvar_in_type`); the obligation is
  stated without such locals. (h) What had no derivation was refused
  late or obscurely: a type without constructors, a proposition, an
  argument that is a value, `Nat` — each refused up front with its
  reason (pin `derive_refusals`); a generated variable could be read
  as a constructor of the field's type; the table's key was not
  normalized. **Left, and known:** a let bound to a self-call under
  a measure still leaves a later call's obligation open-termed
  (`fvar_in_type`, as before the slice); a measured function whose
  result type depends on its arguments is `app_type_mismatch` with no
  pointer; `erase_bool_eq` writes its second operand's code twice
  (one path runs).
- **Cleanup.** `realize.shard` opened `kernel.prog.EType` and used
  none of its constructors since slice 3.18 moved `etype_eq` (the
  loader's `UNUSED` record said so on every load); removed.
- **Gates run.** 176 loader pins (20 new: `derive_eq`,
  `derive_containers`, `derive_field`, `derive_nested`,
  `derive_params`, `derive_policy`, `derive_duplicate`, `derive_by`,
  `derive_needs`, `derive_e_first`, `derive_sig`, `derive_name_taken`,
  `derive_refusals`, `derive_visible`, `registry_native`,
  `decidable_cells`, `match_dependent`, `if_rec_call`,
  `measure_if_call`, `bool_ops`); `define_test.sh` (12 checks);
  `derive_test.sh` (14 checks: the library's 14 procedures, the
  example program's ten lines on the host — every row of Init's Bool
  connectives and of `Bool`'s equality among them, the same list K
  decides; `v3/examples/derive/derive_main.shard` — and
  `convention.shard`: two types with the same constructors in
  opposite order compare oppositely, and a type over a hand-written
  ordering records the procedure it selected, T1's fixture); the
  gates of every slice (records §9 for the figures).

#### Slice 3.20

**Slice 3.20 — I's opener: the certificate core** (the user's ruling
of 2026-10-05 on six leans, the design 2026-10-06; §13 item 53). Law
§7 fixes the shape: I is a certificate language, replay elaborates it
to P without search and K checks P; §7.2 names the vocabulary and the
replay contract; §7.3 wants the elaborator exposed as E functions.
Before this slice a `theorem`'s proof was `(exact TERM)` or `sorry`
(131 theorems in V3, every one an explicit term) and `by` was
`reserved_form`.

*The ruling.* (1) **Three slices**, a gate between each: 3.20 I's
data, `(by …)` in a `theorem`, `elaborate(I)` to P, the forms with no
engine behind them, and `goal_of`/`step` as E functions over one
derivation; 3.21 the two forms that carry an engine — `simp_only` (a
bounded rewriter over a lemma list) and `arith` (the Farkas
certificate through `Lean.Omega`'s lemmas) — and calc's 100 claims
closed; 3.22 the producers — `tools/prove` re-pointed at I, the
sidecar and the pin store (law §7.5), the engine. Rejected: one slice
(no gate for a full window or more); the forms before the API (the
API is cheap when I's data is shaped for it from the start). (2) **The
tactic block in source is I.** What an author writes inside `(by …)`
is I's concrete syntax; a node that omits a consequential choice
names the versioned default reconstruction (`(rw LEMMA)` with no
occurrence: the first match in the traversal of rule 7, every
occurrence of it), and the pin records what was written. Rejected: a
tactic surface distinct from I, translated down (two languages, two
versions). The user's condition — "if it doesn't invite too many
compromises" — is rule 2's test: a form is in I only if its elaboration
is a function of the node and the goal. (3) **No subterm order.**
`induction` carries its recursor; `wf` is `WellFounded.fix` over
`(measure f)` through `InvImage.wf`, the strong hypothesis stated as
`∀ y, f y < f x → P y`, every descent fact a goal. Rejected: v2's
`subterm-induct` and `(below)` (a V3-only logical rule in a
Lean-rule-exact target; `(below)` discharged `⊰` syntactically, which K
cannot check). Porting cost accepted by the user: 52 `(below)` and 8
`subterm-induct` sites re-spell at port time. (4) **Computation is
conversion.** `reduce` is a `show` of the normal form the elaborator
computes, checked by K's conversion — no `Eq.refl` chain, no
reflection until phase 4; `unfold f` is `rw` by `f`'s equations;
`decide` is `of_decide_eq_true (Eq.refl true)` with the decision the
derivation table or the elaborator's lookup gives. Rejected: a `simp`
with implicit lemma sets (search at replay). Porting cost accepted:
v2's `reduce`/`simp`/`compute` (38,000 sites) are one node here.
(5) **Calc's claims by hand**, re-spelled in V3's idiom — Prop
hypotheses `(<= 0 x)` where v2 wrote `(= (le 0 x) True)` — as the
worked-examples set; §12.3's migration tool waits for the 4,900-claim
port and is calibrated on `std`. (6) **The API is sequential at the
opener**: I's data has the law's graph shape (explicit child goals,
scope-safe locals, display names aside), the API is `goal_of`,
`applicable`, `step`, `elaborate` over one derivation; the
transactional `attempt`/`commit`, snapshots, owners and parallel
regions of law §6 wait for the engine's slice, where they have a
caller; nothing is stored — K checks P at every build, the content
store comes with the producers.

*The rules.*

1. **The form.** A `theorem`'s proof is `(exact TERM)`, `sorry`, or
   `(by STEP…)`; a `fulfills` takes the same three. A block is a
   sequence of steps on **one goal**: each step but the last turns the
   goal into one goal; the last closes it. A step that opens several
   goals carries each one's block inline, so a derivation is a tree
   whose children are explicit (law §7.3): `(cases x ((zero STEP…)
   (succ (n) STEP…)))`. A block that ends with the goal open is
   `goal_open`; a step after a closing one is `goal_closed`. Every
   refusal names the step by its index in its block and shows the goal
   it faced, as `goal_of` renders it (rule 10).
2. **A goal is a closed proposition.** `Goal` is a metavariable whose
   type is the goal's statement closed over its named context — `∀
   (x : Nat) (h : x = 1), P x` — and an occurrence of a goal in a term
   is the metavariable applied to the context's locals (R68, rule 10
   of slice 3.16: the stable creation telescope, each occurrence an
   instantiation). Closing a binder over a goal then abstracts the
   occurrence's arguments, never the hole, so a goal is filled after
   its binders closed with no delayed assignment: the filling is a
   lambda over the telescope, beta-reduced at instantiation. The
   named context is the Pi's binders; display names are the binders'
   names and identity is the fvar a step opens them as. The theorem's
   own binders are in the first goal's context, as Lean's are.
3. **A step is a function of the node and the goal.** `step(goal,
   node)` opens the goal's context as locals, acts on the target, and
   answers `StepDone(term)` — the goal's filling, a closed term over
   the subgoals' metavariables applied to their telescopes (the
   construction record of law §7.3, as data) — with the subgoals in
   order, or a refusal. `elaborate(I)` runs `step` on each node of
   the tree depth-first and instantiates; the result is P, one closed
   proof term K checks as the theorem's value. No node consults a
   changing table or runs a search: `rw`'s match, `apply`'s
   unification and `reduce`'s normal form are deterministic in the
   node, the goal and the environment.
4. **The vocabulary of 3.20.** `(intro NAME…)` — the target is a Pi
   after whnf, each name takes one binder (`intro_no_pi`). `(exact
   TERM)` — the term at the target; `rfl` — `Eq.refl` at `a = b` by
   K's conversion (`rfl_failed` shows both sides after whnf); `(have
   NAME PROP STEP…)` — the cut: PROP a proposition, its block a child
   goal under the current context, the rest of the block continues
   with `NAME : PROP` in the context; `(show PROP)` — the target
   replaced by PROP where K's conversion agrees (`show_mismatch`);
   `(apply TERM STEP-BLOCK…)` — TERM elaborated without an expected
   type, its type's Pi-telescope opened with a metavariable per
   binder for exactly arity(TERM) − arity(target) binders after whnf,
   the conclusion unified with the target; each metavariable still
   unassigned whose type is a proposition is a child goal in binder
   order, its block the next in the list (`apply_goals` names the
   count and the premises when the list is short or long); a
   non-proposition left unassigned is `apply_unsolved` (a witness
   goal is the graph's, after 3.20); `(cases TERM ((CTOR (NAME…)
   STEP…)…))` — TERM a local of an inductive type without indices
   (`cases_indexed` otherwise: `injection`/`subst` are later forms),
   the motive the target generalized over it (slice 3.19's rule 9),
   `T.casesOn` applied, each minor premise's telescope opened under
   the names given (`cases_fields` on a count mismatch), one block
   per constructor in declaration order, each constructor named once
   (`cases_rows`); `(induction TERM ((CTOR (NAME… IH…) STEP…)…))` —
   the same over `T.rec`, the recursive fields' hypotheses named
   after the fields; `(wf MEASURE TERM IH STEP…)` — TERM a local, the
   measure a term over it into `Nat`, `WellFounded.fix (InvImage.wf
   MEASURE Nat.lt_wfRel.wf)` with the target as motive, the block
   under `IH : ∀ y, MEASURE[y] < MEASURE[TERM] → target[y]`;
   `(decide)` — the target's decision through the table of slice 3.19
   or the elaborator's lookup at `Nat`, `Int`, `Bool`, the term
   `of_decide_eq_true (Eq.refl true)`, K's conversion the work
   (`decide_failed`, `no_decision`); `(unfold NAME…)`, `(reduce
   SIDE?)` and `(rw …)` — rules 6 and 7; `sorry` — rule 9.
5. **Rewriting is `Eq.mpr`.** A rewrite of the target by `h : a = b`
   abstracts every chosen occurrence of `a` into a motive `λ x.
   target[x]` and fills the goal with `Eq.mpr (congrArg motive h)
   ?g'` where `?g' : target[b]`; the occurrence path is the motive —
   the law's guarded path is a position in a term view with its
   expected subterm, and the motive carries both. `congrArg` and
   `Eq.mpr` are Init's (lines 3,464 and 14,083 of the export, inside
   the test prefix). A rewrite at a hypothesis (`(rw LEMMA (at H))`)
   is Eq.mp on the hypothesis under a `have` of the same name (later
   in 3.20 if calc needs it; otherwise 3.21).
6. **`unfold` and `reduce`.** `(unfold f)` rewrites each occurrence of
   `f` at full arity by the first of `f`'s equations (`f.eq_N`, slice
   3.13) whose left side matches it first-order, to a fixpoint under a
   budget; an equation proven by `Eq.refl` is applied by conversion
   and contributes no term, an equation that is propositional (a
   measured `fn`'s, slice 3.14) is a rewrite by rule 5; a `def` with
   no equations is delta-reduced by conversion. No match anywhere is
   `unfold_stuck`, with the pointer "no equation of f matches; cases
   on its argument first". `(reduce)`, `(reduce lhs)`, `(reduce rhs)`
   — the whole target or one side of an equation replaced by its
   normal form: K's whnf at every position (beta, iota, projections,
   `Nat` literals through K's accelerators), every `fn` by its
   equations as `unfold` applies them (never by delta: a `fn`'s value
   is the recursor form, the equations are its meaning), every `def`
   and `abbrev` by delta, to a fixpoint under a budget
   (`reduce_budget`). The normal form is `show`n; a propositional
   equation on the way makes the step a rewrite of rule 5 and the
   `show` follows. This is one node for v2's `reduce`, `simp` and
   `compute`, whose distinction was the old evaluator's.
7. **`rw`.** `(rw LEMMA)`, `(rw (<- LEMMA))`: LEMMA a term — a name,
   or a name applied to arguments — elaborated without an expected
   type; its type after whnf opened as `apply` opens, to an equation
   `a = b` (or `b = a` under `<-`; `rw_not_eq` otherwise). `a`, with
   its metavariables, is matched first-order against the target's
   subterms in **pre-order, left to right, the function before its
   arguments**, the first match instantiating the metavariables, and
   every occurrence of the instantiated `a` is rewritten (Lean's
   `rw`), by rule 5. `(rw LEMMA (occ K))` rewrites the K-th matching
   occurrence only, and `(rw LEMMA (occ K EXPECT))` guards it with the
   instance expected there (slice 3.29 rule 5, `rw_occ_changed`). The
   lemma's premises left unassigned are child goals, positional blocks
   as `apply`'s: `(rw LEMMA (occ K EXPECT?)? STEP-BLOCK…)`.
   `rw_no_match` shows the pattern and the target. A rewrite whose
   result is `a = a` is not closed by `rw` (Lean's `rw` tries `rfl`;
   here the author writes it — one node, one meaning).
8. **Universes and levels.** A goal's statement is a `Prop`; a
   motive over a Pi into `Prop`; `congrArg`'s levels come from the
   types of `a` and the target by unification, as any constant's do
   (slice 3.15).
9. **`sorry` in a block** closes its goal and makes the theorem
   pending (`PENDING NAME sorry`), as a top-level `sorry` does:
   nothing is declared, every citation is a pending obligation. The
   block is elaborated whole, so every other step is checked, and the
   record shows each sorried goal as `goal_of` renders it.
10. **The E functions** (`kernel/tactic.shard`, exported through the
    loader's module): `goal_of : Goal → the context (name, type)
    list and the target`, rendered in S by the dumper; `applicable :
    Goal → the node heads whose precondition holds` (a Pi target
    admits `intro`, an equation admits `rfl` and `rw`, an inductive
    local admits `cases`…; not a completeness claim); `step : Goal →
    Node → StepRes`; `elaborate : Goal → I → P or refusal`. All are
    `fn`s, so an engine in E can compose them (3.22). `Node` is I's
    data type: one constructor per form of rule 4 with its arguments
    as data (names, S terms, child blocks), read from the S form by
    `read_node` — the concrete syntax of rule 1 and nothing else.
11. **Records.** `PROOF NAME nodes=N` on an accepted theorem proven by
    a block (the ACCEPT record stands); `PENDING NAME sorry goals=K`
    with the goals under it. The DEFINE/REALIZE records are unchanged.
12. **Not in 3.20**, each named: `simp_only`, `arith` (3.21);
    `injection`, `subst`, `cases` on an indexed family or on `Eq`;
    `exists`/`constructor`/`left`/`right` (an `exact` with the
    constructor serves); `intro` through a `let`; `wf` over a measure
    into `Int` or over a lexicographic pair; a witness goal (data
    hole); `rw` under binders where the motive would capture
    (`rw_motive`: Lean's "motive is not type correct"); `by` inside a
    term (`(by …)` as an L term); the Rust bootstrap reads `by` as
    `UnknownForm`, so a theorem under `v3/kernel/**` stays on
    `exact` until route 1's chain is V3's own.

**Slice 3.20 as built** (2026-10-06):

- **Rules 1 and 10.** `kernel/tactic.shard` is the opener: `Node` is
  I's data (one constructor per form, the arms and the premise blocks
  nodes of their own kinds), `read_block` reads `(by STEP…)`,
  `tac_step` runs one node on one goal, `tac_block` runs a block's
  nodes depth-first — the children of a branching node from the
  blocks the step pairs with them, in the goals' order — and
  `tac_prove` instantiates the root; `reader.shard`'s `read_proof`
  hands a `by` to it, for a `theorem` and for a `fulfills` alike.
  `goal_of` renders a goal (`(x : Nat) (h : (= x 1)) ⊢ (= x 1)` —
  bound names from the binders, `Eq` and the comparisons under their
  operator spellings); `applicable` lists the forms whose
  precondition the goal meets. A refusal names the step's index in
  its block and shows the goal it faced. **A premise's one-step block
  may be written as the step:** `(apply Eq.symm (exact h))` — a list
  headed by a form's name is one step, so the common case needs no
  second parenthesis. The arms of `cases` and `induction` come in one
  list, `(cases x ((zero STEP…) (succ (n) STEP…)))`, and may name the
  equation: `(cases (is_digit c) hd (ARM…))` gives each arm `hd :
  (= (is_digit c) false)` and so on — the motive is `λ d. x = d →
  target[d]` and the eliminator's result is applied to `Eq.refl x`
  (Lean's `cases h : x`); calc's lexer lemmas need it where the split
  term reappears under the unfolding of a function. The record of
  rule 11 was not added: `ACCEPT` stands for a theorem proven by a
  block as for one proven by a term, and the pending record carries
  the count (`PENDING NAME sorry goals=2`).
- **Rule 2.** As designed. The goal's metavariable has the closed
  statement as its type and no telescope; a step opens the first
  `nctx` binders as K locals under the binders' names, acts on the
  target and closes the filling over them, so every assignment is a
  closed term and R68's delayed filling never arises. A split or an
  induction on a local drops it from the children's context when no
  other local's type mentions it.
- **Rules 3 and 4.** As designed, with these shapes: `(wf X MEASURE IH
  STEP…)` — X the local first, the measure a `Nat` term over it, the
  hypothesis `IH : ∀ y, (< MEASURE[y] MEASURE[X']) → target[y]` stated
  with `Nat.lt` so it reads as the arithmetic a descent fact proves;
  `WellFounded.fix.{u,0}` at `InvImage.wf MEASURE Nat.lt_wfRel.wf`
  as `define.shard` builds it for a function. `apply` opens exactly
  arity(TERM) − arity(target) binders and refuses a premise whose
  statement mentions an argument the target leaves open
  (`apply_unsolved`, with the pointer: give it through the first
  premise — `(apply (Eq.trans h1) (exact h2))`); `decide` builds
  `of_decide_eq_true` at the decision the elaborator's lookup or the
  derivation table gives and checks `Decidable.decide` against
  `true` by K's conversion before K sees the theorem.
- **Rule 5.** As designed. `rw` matches the lemma's side
  **syntactically** (`tac_match`: structural descent, a metavariable
  assigned through unify.shard's typed `assign`, levels structurally)
  — never by K's conversion, which would make `app nil ?ys` an
  instance of `app (cons x r) nil` by computing both sides, a match
  and a useless one. An application is matched with its whole spine;
  a prefix of a spine is no subterm. A rewrite at a hypothesis (`(rw
  LEMMA (at H))`) was not built: calc's one need (`skipws_head_nonws`)
  is met by `show` of the convertible target and a reversed rewrite.
- **Rule 6, and the equations of a measured function.** `reduce`
  finds the first redex of the target in pre-order: an application of
  a constant with equations by the first equation that matches; any
  other application, a projection or a redex by K's whnf where it
  progresses — the result differs, its head is neither a stuck
  eliminator (a recursor, a case analysis, `brecOn` and its helpers,
  a matcher, a fixpoint, a projection of one) nor a lambda (a
  function constant's value is no reduction of it) — and never an
  inductive's or a constructor's application, a theorem's, or a
  proposition's. K's whnf reduces a head as far as it goes, past the
  one step wanted — an `ite` on a decided condition to its branch,
  then the branch's function unfolded to its recursor form — so every
  application of a constant with equations inside the subterm is
  masked by a local during the whnf and restored after. **Two kinds
  of equation:** a computation rule has a constructor or a literal in
  its left side (`app (cons x r) ys = …`) and `reduce` applies it
  anywhere; an unfolding (`count_down n = ite …`, a measured
  function's one leaf) `reduce` applies only to a closed occurrence —
  a ground computation ends, the budget bounding it — and `unfold`
  applies once to each occurrence present when the step starts, never
  to one a rewrite introduces. A definitional equation (by `Eq.refl`)
  is applied by conversion and contributes no term; a propositional
  one is a rewrite of rule 5 with a continuation goal, the previous
  goal assigned. **A measured function now has its equations:**
  `define.shard` states each leaf's as before and proves it by
  `WellFounded.fix_eq` at the instance — the definition's value at the
  leaf's arguments is `WellFounded.fix α C r hwf F t`, and `fix_eq`
  there has the type `fix … t = F t (λ y _. fix … y)`, whose sides K
  converts to the leaf's (the left by delta, the right by beta, the
  tuple's projections and the matcher at a constructor, each self-call
  by delta); the descent obligations are admitted into the walk before
  the definition. `WellFounded.fix_eq` lies at line 107,407 of the
  export, past the pins' prefix, so the pins test and the files that
  need it stream the second fixture; `calc_spec.shard` imports through
  it, and `parse_tail` has `eq_1`.
- **Rule 7.** As designed: `(rw LEMMA)`, `(rw (<- LEMMA))`, `(occ K)`
  with the count in the refusal, the premises as positional blocks
  (`(rw if_pos (exact h))`; `if_pos`'s instance argument is assigned
  by the match). The traversal is pre-order with the function before
  its arguments over whole applications.
- **Rules 8 and 9.** As designed.
- **Rule 12.** As listed; `cases` on a term that is not a local
  generalizes its occurrences in the target (an `if`'s condition,
  after `reduce` has exposed it).
- **Not built, found on the way:** a witness goal (`apply Eq.trans`
  without the middle term); `rw` at a hypothesis; `induction …
  generalizing` (the statement quantifies what the hypothesis must
  vary over, and `intro` follows the split — `lex_num`); `show`'s
  display of a stuck `Decidable.rec` is long where a `Bool` decision
  is unfolded by hand; the elaborator's refusal of an unsolved
  universe inside a block carries no step index.
- **Found by the probes:** the recursor takes its major after the
  minors and `casesOn` before them; a name shared with another kernel
  file's (`form_head`, `open_fields`, `sort_level`, `StepRes`, `RwOk`,
  `mem_expr`, `RdOk`) shadows it under the bootstrap's flat resolution
  and fails far from the collision — every name in `tactic.shard` is
  checked against the closure.
- **Found by a second reader over the diff, each reproduced and
  fixed:** `(cases X H …)` on a local dropped `X` from the arm's
  context while `H` still named it (K: `fvar_in_value`, no step index)
  — the local stays when an equation is named; `(induction X H …)`
  would guard every hypothesis by the equation, so it is
  `induction_eq`; `unfold` and `reduce` of a `def` with no equations
  were refused where rule 6 promised delta — K's whnf takes `double
  x` past `x + x` to a stuck recursor, so one delta step is made by
  hand (`delta_once`) where whnf makes no progress; `(occ K)` counted
  prefixes of application spines the traversal never offers — an
  occurrence is a whole spine now; `reduce` on a closed occurrence
  whose condition no reduction decides (`count_down (g 5)` with `g`
  opaque) ran for minutes — an unfolding applies only to a **ground**
  occurrence (no local, every constant a definition, a constructor, a
  recursor, an inductive or a quotient), under a budget of 64
  unfoldings; a refusal rendered its goal twice and a term as a
  sequent. Two pin comments were wrong. *Noted, not changed:* `(occ
  K)` counts the occurrences of the **first** instance, as Lean's `rw`
  does; Init's arithmetic lemmas speak `HAdd.hAdd` where V3 writes
  `Nat.add`, so a rewrite by them needs a `show` first — syntactic
  matching by design; `(by)` with no step is `bad_proof`.
- **The consumer.** 44 of calc's 100 claims are theorems by blocks in
  five files (`tactic_test.sh`): the nine ground spec tests and the
  eleven reconciliation tests by `(reduce) rfl` (parse_tail, measured,
  by its `fix_eq` equation at closed arguments — 20 s for the nine
  under the bootstrap, against K's own conversion through `Acc.rec`
  before the equation existed); the lexer's structural lemmas
  (`lex_num` by induction with the accumulator quantified and the
  split's equation named, `lex_digit_head`, the three head lemmas by
  `if_pos`/`if_neg`, `skip_ws_idem`, `head_skipws_false`,
  `skipws_head_nonws`, `parse_tail_nil_test`); the digit type's
  (`is_digit_code` a ten-way split closed by `decide`); `codes_append`
  by induction. Of the 52 whose lemma closure needs no arithmetic the
  rest wait on `show`'s `Int` measure, Init's inequality lemmas or
  3.21; the 48 others need `arith`. The worked-examples file of the
  ruling's fifth lean is these five, one claim of each shape among
  them.

#### Slice 3.21

**Slice 3.21 — the two forms that carry an engine: `arith` and
`simp_only`** (under the ruling of 2026-10-05, item 53's first lean;
the design 2026-10-06; §13 item 54). Slice 3.20 left calc's claims
that need arithmetic (48 of 100) and every chain of rewrites written
one step at a time. Law §7.1 draws the line this slice works on:
whatever searches is a producer of I and what it emits is the result;
replay computes and matches, and K checks P.

*The rules.*

1. **`arith` is a Farkas certificate over stated rows.** `(arith
   FACT…)` closes a goal that follows from linear arithmetic over
   `Int` and `Nat`. Its **rows**, in order: the goal negated; each
   hypothesis of the goal's context whose statement is linear (rule
   2), in context order; each FACT — a term, a hypothesis' name or a
   lemma applied — whose type is linear (`arith_fact` otherwise); and
   the rows the atoms bring (rule 4). A certificate is one integer per
   row — non-negative on an inequality, any sign on an equation —
   under which the rows sum to a constant inequality that is false
   (`0 ≤ -1`). `(arith FACT… (farkas K…))` carries the certificate (a
   row past the list's end has multiplier 0): replay checks the sum
   and builds P, no elimination runs (`arith_certificate` shows the
   sum that is left). Without the clause the node names the
   **default reconstruction** (item 53's second lean): Fourier–Motzkin
   elimination over the rows in atom order under a budget of 512
   derived rows (`arith_budget`), the equations eliminated first by
   substitution; what it finds is the certificate, and the two forms
   elaborate through the same builder. `arith_failed` shows the rows
   in normal form. The elimination is exact over the rationals and
   integer-tight only in the strict comparisons (`a < b` is `a + 1 ≤
   b`): no cut, no case split — `2x = 1` is not refuted, a `≠`
   hypothesis is no row. Those are a producer's (`omega`, 3.22),
   which emits this node.
2. **Linear statements.** `a ≤ b`, `a < b`, `a = b` and their
   negations at `Int` or `Nat`, under V3's spellings (`Int.le`,
   `Nat.lt`, `Eq`) and Init's (`LE.le`, `LT.lt`, `GE.ge`, `GT.gt`,
   `Ne`, `Not`), where the sides are built from `+`, `-` (at `Int`),
   unary `-`, `*` with a literal on one side, literals, and **atoms**
   — any other term, two atoms the same when they are the same term
   after the spellings of rule 3 are made one. `False` as a goal has
   no negation row. A goal `a = b` is two certificates (`a ≤ b` and `b
   ≤ a`, `Int.le_antisymm`) unless the sides have one normal form,
   when it is their two evaluations composed and no certificate; a
   goal `¬ P` is P's row and the certificate.
3. **One spelling.** Init's lemmas speak `HAdd.hAdd Int Int Int _ a
   b`, `OfNat.ofNat Int 5 _`, `Nat.cast n`; V3 writes `Int.add a b`,
   `Int.ofNat 5`. `arith` reads both: a class method at `Int` or `Nat`
   under Init's instance is its function, throughout a row — inside
   an atom too — and K's conversion is the proof that the two are one
   (the instances unfold). So `(arith (Int.emod_nonneg n h))` and a
   row from the context meet on the same atoms.
4. **What an atom brings.** A `Nat` term enters the integers under
   `Int.ofNat` (`Int.natCast_add`, `Int.natCast_mul` push the cast to
   the atoms), and each `Nat` atom `a` brings the row `0 ≤ ↑a`. `x /
   k` at `Int` with `k` a positive literal is an atom that brings `k *
   (x / k) ≤ x` and `x < k * (x / k) + k` (`Int.mul_ediv_self_le`,
   `Int.lt_mul_ediv_self_add`), and `x % k` is `x - k * (x / k)`
   (`Int.emod_def`): the quotient and the remainder by a literal are
   linear arithmetic, as in `omega` and as v2's `div-facts` gave
   them. `Nat` subtraction, division at `Nat`, `min`, `max`, `natAbs`
   and `toNat` are atoms. These rows come after the facts, in the
   order the atoms first occur.
5. **P is `Lean.Omega`'s.** The atoms are a list `v`; each side `t`
   gets a linear combination `l` (a constant and a coefficient list,
   closed data) and a proof `t = l.eval v` built on the term's
   structure from `LinearCombo.coordinate_eval`, `add_eval`,
   `sub_eval`, `neg_eval`, `mul_eval_of_const_left/right` and the
   congruences of `Lean.Omega.Int`; a row `0 ≤ l.eval v` or `l.eval v
   = 0` is `Constraint.addInequality_sat` or `addEquality_sat`; the
   certificate is a fold of `combo_sat'`; the false constant is
   `normalize_sat` and `Constraint.not_sat'_of_isImpossible (Eq.refl
   true)`. K's conversion does the arithmetic on the closed data
   (each intermediate combination is stated as a literal, so one step
   is one bounded computation) and nothing else: no ring normalizer,
   no reflection of the terms. The kit lies in the export through
   line 91,131 but for the product (`mul_eval_of_const_left`, line
   271,373) and the quotient's bounds (line 274,616): a file whose
   rows have neither imports as before; one that has them imports
   through `Int.lt_mul_ediv_self_add` (`arith_unreached` names the
   constant and the line). The second fixture grows to that line.
6. **`simp_only` is a bounded rewriter over a lemma list.**
   `(simp_only RULE…)`, a RULE a LEMMA or `(<- LEMMA)` as `rw` takes
   them: until no rule applies, the first rule in list order with a
   match in the target is applied as `rw` applies it — the first
   match in the traversal, every occurrence of that instance — under
   a budget of 256 rewrites (`simp_budget`: a rule that rewrites its
   own result). A rule's premises are discharged by **assumption** —
   a hypothesis of the context with the premise's statement — and a
   rule with a premise no hypothesis states does not apply there
   (law §7.2's side-condition policy: stated, not searched). No rule
   applies at the start is `simp_no_progress`. Nothing is implicit:
   no lemma set, no arithmetic, no computation — `(reduce)` before or
   after is its own node. A function's name is no rule (`unfold`, or
   name its equation `f.eq_N`).
7. **`applicable`** lists `arith` where the target is linear and
   `simp_only` always; a premise's one-step block may be either.
8. **Not in 3.21**, each named: an `only` restriction of `arith`'s
   context rows (a budget refusal is the signal); cuts and case
   splits; `Nat` subtraction; non-literal products; `simp_only` at a
   hypothesis, with side selectors or with a discharger other than
   assumption; the producers (3.22).

**Slice 3.21 as built** (2026-10-06):

- **Rules 1, 2 and 5.** As designed. `kernel/arith.shard` is the
  arithmetic — the spelling (rule 3), the rows, the certificate's
  check, the elimination, and P from `Lean.Omega`'s lemmas —, and
  `tactic.shard` holds the node (`NArith`: the facts as S terms, the
  certificates) and the step, which elaborates the facts, hands the
  context's locals over (a local whose statement is no comparison is
  passed by) and has K type the term before it is the goal's filling
  (`arith_internal` would name the step; none has been seen). The
  kit's last lemma in the plain case is `Lean.Omega.Int.ofNat_lt_of_lt`
  (line 91,131 of the export), inside the pins' first fixture. An
  equation goal takes a `(farkas …)` clause per direction, the `≤`
  first (a direction without one is reconstructed); a weight of zero
  leaves its row out. A refusal shows the rows in normal form over the atoms: `[0 <= -1 +
  -1*x + 1*y] [0 <= 0 + -1*x + 1*y]`.
- **Rule 3, wider than designed: one spelling is the proof layer's.**
  The first `(rw (if_neg hn))` after `(cases (Int.decLt n 10) …)`
  found no match: the hypothesis the split brings is Init's `¬ (LT.lt
  Int Int.instLTInt n 10)` and the target's condition is `Int.lt n
  10`. So every **new goal's statement** enters in the source's
  spelling (`tac_goal`: the context and the target, K converting the
  occurrence), and so do the two sides of **a lemma's equation** in
  `rw`, `simp_only` and `reduce` (`eq_of_lemma`) and a premise
  `simp_only` looks up. The 3.20 note that a rewrite by one of Init's
  arithmetic lemmas "needs a `show` first" no longer holds.
- **Rule 4.** As designed, and `Nat.succ x` under the cast is `x + 1`
  (K converts the two; `Nat.lt a b` unfolds to it).
- **Rule 6.** As designed: `NSimp`, `simp_loop`; a match whose
  premise no hypothesis states is passed by and the next subterm is
  tried; a rule whose match leaves an argument unassigned, changes
  nothing or has a motive K does not type is passed by for the next
  rule.
- **Rule 7.** As designed.
- **A matcher under a measure carries its equation** (`define.shard`,
  `realize.shard`; not in the design — calc's `parse_tail` could not
  be made total without it). Slice 3.14 stated a self-call's descent
  obligation over every local in scope; in a row of `(match (skip_ws
  cs) …)` the locals are the pattern's variables and nothing ties them
  to the scrutinee, so `parse_tail.dec_1` read `∀ p c rest, … → ∀ n
  rest2, len rest2 < len p.2` — false, and `parse_tail` could never
  leave `PENDING measure` (v2 gave each site the premise `skip_ws cs =
  Cons c rest`). Now, in a measured function, a matcher's motive is `λ
  s' . s = s' → motive s'`, each row's last binder `h : s = pattern`,
  and the application is applied to `Eq.refl s` — Lean's `match h : s
  with`, by `addArg` on the matcher K checked (`pj_match_eq`) — so `h`
  is a local in scope and the obligation is stated under it — and as
  the comparison it is, `measure[arguments] < measure[parameters]` at
  `Nat`, where slice 3.14 stated the relation's instance `InvImage …
  (tuple) p`; the rows' variables carry the source's names and a
  function of one binder its binder's: `∀ xs x r, xs = cons x r → ∀ y
  s, r = cons y s → len3 s < len3 xs`. The
  function's equations are stated over the same shape (`PkEqn`: the
  body walked once more with self-calls left as calls, so K converts
  `F t (λ y _. fix y)` to the leaf's body as before); the leaf walker
  reads the extra argument, so a leading matcher on a binder still
  gives one equation per constructor. The program is untouched (it is
  the erasure of the body as written). `parse_tail`, `show` (now under
  `(measure (Int.natAbs n))`) and `show_nat` are `DISCHARGE … proved`:
  **nothing a claim of calc's cites is pending.** Slice 3.20's
  theorems over `parse_tail` had been accepted under those two
  parameters.
- **`reduce`, two repairs** the capstone's proofs needed. (1) A case
  analysis at a decided scrutinee takes one step: a matcher at `cons c
  rst` whose row is an `if` on an open condition was no progress (K's
  whnf goes on into the row and ends at a stuck `Decidable.rec`); the
  alternatives — an `ite`'s branches, a matcher's rows — are masked by
  locals while K picks one (`alt_step`). (2) A proposition is no
  computation: `reduce` unfolded `Nat.lt a b` to `Nat.le (Nat.succ a)
  b`; it leaves a comparison as it is now (`unfold` by name still
  unfolds one). Three of 3.20's proofs lost a last `(reduce lhs)` that
  had nothing left to do — `reduce_stuck` on an already normal target
  makes a proof depend on how far the step before went (*noted*).
- **A type and its constructor under one name** (`(type CalcState
  (CalcState …))`, §13 item 28) were `ambiguous_name` in a theorem's
  binder; where a type is expected — a binder's type, an argument at a
  sort — the name is the type's (`elab.shard`).
- **The pending record shows its goals** (slice 3.20's rule 9, which
  that slice left at the count): `PENDING main.t sorry goals=2 [(n :
  Nat) ⊢ (= n n)] [(n : Nat) ⊢ (= 1 1)]`. It is how a block is
  written: a `sorry`, a load, the goal. Goals print `Int` literals and
  `+ - *` as the source writes them.
- **The fixture.** `init_prefix_str_tail.ndjson` grows by 11,101
  lines, to line 274,616 of the export (`Int.lt_mul_ediv_self_add`).
- **The consumer: calc's 100 claims are theorems**, file for file
  (`kernel/test/tactic_test.sh`: five loads cover the fourteen files,
  110 checks), with the capstone `run_eq_spec : run cs = spec_run cs`
  under no parameter. Re-spelled in V3's idiom: a hypothesis is a
  proposition (`(<= c 32)`, `(Not (= c 43))`); an induction's
  statement quantifies what its hypothesis must vary over and `intro`
  follows the split; an `if` on a comparison is split on its decision
  (`(cases (Int.decEq c 43) ((isFalse (h) …) (isTrue (h) …)))`); a
  hypothesis about a split term is an implication introduced after
  the split. `loop_eq` is restructured around `parse_num`'s result
  (`arm_plus`, `arm_minus` take the lexer through one operator and one
  number), so its hypothesis is cited under `ptd`, the fact that
  discharges `parse_tail`'s own descent. Twelve lemmas the old proofs
  inlined or took from `std` are stated: `append_nil_right`,
  `append_assoc` (`list.shard`), `natAbs_of_nonneg`, `natAbs_lt`
  (`natabs.shard`: Init's are at line 672,885, past the fixtures),
  `is_ws_true`, `is_ws_false`, three `pr_*_none`, `nonnum_none`, the
  two arms. 45 steps are `arith`, none with a certificate written,
  and 18 are `simp_only`.
- **Found on the way, not changed.** Init's names are visible as far
  as *any* loaded module imported: `calc_proof.shard`'s
  `decide_eq_true` became `ambiguous_name` (with
  `Bool.decide_eq_true`) once another file in the load imported
  further; it cites `Init.decide_eq_true` now. A measured function of
  several binders has its obligations stated over the arguments'
  tuple `p` (`len rest2 < len (proj PProd 1 p)`; `PProd.snd` is at
  line 2,429,884). There is no `by_cases`: the
  split on a decision instance serves.
- **Found by a second reader over the diff, each reproduced and
  fixed, the first three pinned.** `simp_only` with a rule whose right
  side holds its left (`r = mk (fst r) (snd r)` given without its
  argument) put the term back twice at every step: the target doubled,
  no budget of rewrites was reached and the load did not end — it is
  refused at once (`simp_self`). `Ne a b` was rewritten to `Not
  (Eq.{1} …)` whatever Ne's universe, and `intro` then built a term K
  refused (a statement over `Ne Nat Int`; `by_ne_level`). A descent
  obligation printed every pattern variable as `x` and its fact as the
  relation's instance, where `reduce` stepped into the relation and
  `arith` read no comparison (the bullet above; `define_match_eq`). A
  generated binder's name printed as a bare number (`(fun (20 : Nat)
  …)`: `x20` now). A class method at a local instance was rewritten
  as Init's (it stays as written). `v3/test.sh`'s fallback for a
  missing result printed the shell's error. *Noted, not changed:* a
  type and its constructor of one name are still ambiguous where
  nothing is expected (the first operand of `=`); a compound literal
  as a first operand elaborates at `Nat` (`(<= (- 0 5) x)` — the
  elaborator's, older than the slice).
- **Not built**, as rule 8 lists; and no pin for 3.20's fifth finding
  (a ground occurrence under an opaque constant): a pin has no way to
  declare one.

#### Slice 3.21b

**Slice 3.21b — the trajectory review's findings 1–3** (GPT-6's
review of 2026-10-06 over the tree at `7e99261`, records §4.11; the
user's ruling 2026-10-06 on the leans; §13 item 55). Three of the
review's six findings are the three items slice 3.21's as-built left
open, with what the review added: a specification and its
implementation inherit one unintended numeric reading while both
proofs check; a module's meaning depends on the load's order; a proof
is tied to how far the step before it went.

*The rules.*

1. **The expected type decides an arithmetic operator.** Where `+ - *
   / mod %` is read at an expected type whose normal form is `Nat` or
   `Int`, the operator's identity is that type's and both operands are
   read at it — a `Nat` local under `Int` is cast at the leaf
   (`Int.add (Int.ofNat n) 1`), as Lean's `binop%` reads `(n + 1 :
   Int)`. Where the expected type decides nothing — none, a
   metavariable, not numeric, a comparison's operands — the identity
   is the first operand's that is not **numeric-closed** (a numeral, or
   an arithmetic operator over numeric-closed operands), the other
   operand read at it; all numeric-closed is law §5.2's default,
   `Nat`. A boundary the author writes holds: `(Int.ofNat (- 1 2))` is
   `Nat`'s subtraction, 0. Before, the operation was chosen from the
   first operand that was not a numeral and the result coerced:
   `(- 1 2)` at `Int` was `Int.ofNat (Nat.sub 1 2)`, 0, and `(<= (- 0
   5) x)` read at `Nat` against x's `Int`. The rule is an elaborator
   fix under law §5.2's order (constraints before defaulting), not a
   design change.
2. **Init's visibility is the module's horizon.** Each imported
   declaration carries its ordinal at admission (the loader's
   declaration → ordinal table, keyed as the environment is). A
   module's horizon is the count admitted at the farthest of its own
   `(import Init NAME)` and its imports' horizons; a module that imports
   no Init has none. Resolution sees an imported declaration below the
   horizon only — whatever else the load admitted, in whatever order —
   and a constant with no ordinal (K's own, admitted with the
   environment) wherever Init is. The checked environment stays one and
   shared; what changes is what a scope sees of it. The slice-3.21
   workaround (`Init.decide_eq_true` in calc_proof) stays as written.
3. **`reduce` is a normalization: it succeeds unchanged where nothing
   reduces.** The refusal `reduce_stuck` is retired; `reduce_budget`
   stays. No step requires progress today, so none is given a
   progress-requiring variant (the review's "separate operation" waits
   for a procedure that needs the contract).

**Slice 3.21b as built** (2026-10-06):

- **Rule 1.** `elab.shard`: `op_expected` (the identity from the
  expected type, for an arithmetic operator), `is_arith_op`,
  `is_num_closed`/`all_num_closed` (in `is_numeral`'s place in the
  operand rule). The pin `op_expected`: `(- 1 2)` at `Int` is `Int.sub
  1 2` and equals `-1`; `(+ n 1)` at `Int` with `n : Nat` is `Int.add
  (Int.ofNat n) 1`; `(<= (- 0 5) x)` is `Int.le (Int.sub 0 5) x`;
  `(Int.ofNat (- 1 2))` is `Int.ofNat (Nat.sub 1 2)` and equals 0;
  `(- n 1)` at `Nat` is `Nat.sub n 1`. One file of the closure changed
  meaning under the rule, and the full suite found it: the pin
  `one_meaning` (slice 3.16's G1) stated `(+ a (- n 1))` at `Int` with
  `n : Nat` as a truncating `Nat` difference cast after (2 at `n = 0`);
  it is `a + (↑n - 1)` now (1), as Lean reads it, and the pin says so.
  Parity byte-identical, calc's 100 claims and 21 inputs as before.
- **Rule 2.** `scope.shard`: `VisOnly`'s second field is the horizon
  (an `Int`; `-1` none) and a sixth field the ordinal table;
  `init_within`, `ords_find`, `ords_add`. `loader.shard`: `Load.inits`
  filled at each accepted record of the stream (`ld_add_inits`, the
  record's constants at the record's ordinal), `init_horizon` (the
  ordinal past the import's target, or the count admitted where the
  target has none), `Fx.init` and `Mod.init` the horizon, `fx_see`
  taking the farther. A tail-fixture load measured at 16.1 s with the
  table against 16.4 s without: no cost. The pins `init_horizon` (two
  roots, the wider loaded first — the review's failing order) and
  `init_horizon_beyond` (a declaration past the module's horizon,
  admitted by another module, is `unknown_constant`).
- **Rule 3.** `tactic.shard`'s `reduce_loop` ends at a target with no
  redex; its `made` count is gone. The pin `by_reduce` expects `ok`
  now (reduce at a variable leaves the target; the proof closes by
  `rfl`).
- **Not built**: a per-module Init prefix held as the review's
  "indexed declaration dependencies"; a progress-requiring `reduce`.

#### Slice 3.22

**Slice 3.22 — the producers: `auto`, the sidecar, the engine and the
store** (the user's ruling of 2026-10-07 on three leans; §13 item 56;
law §7.3, §7.5, §6). The third of the slices ruled on 2026-10-05. Law
§7.5 fixes the shape: sources carry tactic blocks or an `auto`
delegation; the build elaborates I to P and K checks P; engine-written
I lives in the sidecar and the pin store, keyed by the resolved
requirement; no tool writes into a source file; a stale pin is a
pending obligation, never re-searched at verification (T8). Two items
carried from GPT-6's trajectory review (item 55): a stored certificate
keyed by its rows' statements, and the dependent witness goal.

*The ruling.* (1) **The engine's output is `(arith only FACT… (farkas
K…))`, the certificate written**: the rows are the goal and the named
facts in list order, nothing from the context, so an unrelated `have`
added later shifts no weight. Rejected: emitting `(arith)` and
reconstructing at every replay — cheaper to write, but it moves a
consequential choice into the reconstruction (law §7.2 carries such
choices). (2) **The store is a CI artifact, not committed**: the build
writes it when asked, CI verifies the bundle, the repository holds
sidecars only. Rejected: committing P (megabytes that change at every
kernel edit). (3) **The engine's reach is measured over calc's 100
claim statements**, the hand proofs staying as the worked examples;
this replaces the slice table's B-1c consumer, the old tree's
generator.

*The rules.*

1. **`auto` is a delegation; the build never searches.** A `theorem`'s
   proof may be `auto` or `(auto HINT…)`. The loader reads the sidecar
   `FILE.auto.shard` beside the source — `(proof-for NAME FINGERPRINT
   (by STEP…))` forms after the schema form `(sidecar N)` (slice 3.29
   rule 4), the file machine-owned and rewritten whole by
   `prove` — and replays the entry's block as it would a hand-written
   one, under the same reader and the same checks. No entry is
   `PENDING NAME auto_missing`; an entry whose fingerprint is not the
   theorem's is `PENDING NAME auto_stale`; a block that fails is the
   refusal it would be inline. Nothing in the loader searches, and no
   tool writes into a source file (law §7.5).
2. **The key is the resolved requirement.** The entry's FINGERPRINT is
   the theorem's content fingerprint — the elaborated statement closed
   over its binders, as the ACCEPT record carries it — so a changed
   numeric reading, coercion or parameter is a different requirement
   and a changed proof strategy is not.
3. **`(arith only FACT… (farkas K…))`.** The rows are the goal negated,
   then each FACT in list order, then the rows the atoms bring (slice
   3.21 rule 4); the context's hypotheses enter only as facts named. A
   certificate indexes those rows. Everything else is slice 3.21's
   `arith`.
4. **The witness goal.** In `(apply TERM (STEP…)…)`, a premise whose
   statement holds an unassigned metavariable is a witness goal: the
   premises' blocks run in the order given, `exact` and `rfl` assign
   through unification, and a metavariable still open when the step's
   blocks have run is `witness_open`. `apply Eq.trans` without its
   middle term has it assigned by the first premise's `exact`.
5. **The engine is an E library over the fixed API.**
   `kernel/engine.shard` composes `goal_of`, `applicable` and `step`
   (law §7.3) into a ladder, cheapest first: `intro`; `rfl`; `(reduce)
   rfl`; `decide`; `(arith only …)` over the context's comparisons;
   `(simp_only EQN…)` over the equations of the functions in the goal,
   then `rfl` or `arith`; `induction` on each inductive parameter in
   turn, the arms closed by the same ladder with the hypothesis among
   the rules. Hints accelerate and add no language: `(induct X)`,
   `(lemmas NAME…)`, `(budget N)`. A failed attempt leaves nothing: the
   elaborator's state is a value, so the transactional invariant of
   law §6 holds without an `attempt`/`commit` API, which waits for a
   concurrent caller.
6. **`prove` is a driver.** `kernel/prove.shard` loads the module as
   `load.shard` does, runs the engine on each `auto` theorem without a
   valid entry, rewrites the sidecar whole (entries it did not solve
   kept, unsolved theorems left pending), and prints a record per
   theorem: `PROVED NAME` with the block, or `UNSOLVED NAME` with the
   last goal. The entry carries the engine's version; a version other
   than the current is reported, not refused (law §7.2).
7. **The store is K's export format.** `load.shard --store DIR` writes
   each accepted declaration as records in the format K ingests
   (`kernel/k/import.shard`), one file per declaration named by its
   content fingerprint, deduplicated; `verify_release DIR` is K
   replaying the store's records after the pinned Init export with no
   elaborator loaded — the T0 machinery over a second stream.

*Landings.* 1: rules 1–4 (`only`, the witness goal, `auto` with
sidecar replay, the pending reasons); gate: the pins and the
hand-written calc claims unchanged. 2: rules 5–6 (the engine, `prove`,
`v3/examples/auto/` with its machine-owned sidecar, the count over
calc). 3: rule 7 (the store and `verify_release` on calc, in CI).

**Slice 3.22 as built, landing 1** (2026-10-07):

- **Rule 1.** `reader.shard`: a proof is `(exact TERM)`, `(by STEP…)`,
  `auto`, `(auto HINT…)` or `sorry` (`read_auto`); `read_decl_autos`
  takes the file's entries, `read_fulfills` and `read_discharge` too —
  a `fulfills` and a descent's discharge may be `auto` (the entry for
  a discharge names the obligation, `f.dec_1`). `loader.shard` reads
  `FILE.auto.shard` once per file whose forms hold an `auto` proof
  (`read_sidecar`; `Fx.autos`); a sidecar that does not read or holds
  a form of another shape is the file's `sidecar_malformed`, the
  theorem pending. The pending reasons carry the fingerprint an entry
  needs: `auto_missing fp=N`, `auto_stale fp=N`.
- **Rule 2.** The fingerprint is `hash_expr` of the closed statement —
  what `identity_hash` gives a theorem, so the ACCEPT record and the
  sidecar agree.
- **Rule 3.** `NArith` carries `only`; under it `step_arith` hands no
  context row to `ar_prove`. The pin `arith_only` is the review's
  probe D with the certificate kept across the unrelated `have`.
- **Rule 4.** `premise_goals` closes each unassigned datum
  metavariable over the context as a goal is closed (`witness_close`:
  the closed metavariable of kind `witness`, the original assigned its
  application to the context), then makes the proposition premises
  goals — their statements may hold the witness's application now;
  the implicit arguments the elaborator inserted (`Eq.trans`'s `{b}`)
  are not among the opened binders and are closed from the applied
  term; `tac_open` zonks a goal's statement, so a block after the
  assigning one reads the witness's value; `tac_block` refuses
  `witness_open` after a step's child blocks have run. A pattern
  (`?B x1 … xn` against a term over the opened locals) is what the
  unifier assigns, and the assignment closes. An instance argument
  left open is `instance_needed`, as before.
- **Found on the way:** `(apply Eq.trans sorry sorry)` reads `sorry`
  where a premise's block must be a list: `(sorry)`.

**Slice 3.22 as built, landing 2** (2026-10-07):

- **Rule 5.** `kernel/engine.shard`: `engine_prove` takes the closed
  statement and the hints and answers a block or a refusal's text;
  `eng_solve` is the ladder — `eng_intro` over every leading binder
  (named after the binder, `hK` where it has none), `eng_close` (`rfl`;
  `(reduce)` then `rfl`; `decide`; `eng_arith`: `(arith only FACT…)`
  with the context's comparisons as the facts and the certificate
  `ar_prove` found written — `ARes` carries the certificates now),
  `eng_simp_close` (`(simp_only EQN… HYP… LEMMA…)` then `rfl` or
  `arith`), `eng_induct` on the hinted local then on each local of an
  inductive type the target mentions, the arms named after the
  constructors' fields with `ih_FIELD` for a recursive one, each arm
  by the ladder one level down (three levels). Two decisions beyond
  the design, each from a case: *the induction is tried before the
  introductions as well* — `len (rev_onto xs acc) = len xs + len acc`
  quantifies the accumulator, and introducing it first left a
  hypothesis too weak, so the binders are kept in the target and the
  hypothesis quantifies them (Lean's `generalizing`); *the rule set
  excludes an unfolding equation whose right side mentions the
  function* — `show n = … show (n / 10) …`, every argument a variable,
  matched its own result without end and calc_show's measurement did
  not finish (an equation with a constructor pattern, `len (cons x r)
  = 1 + len r`, stays: it stops at a variable). A quantified
  hypothesis is a rule too. Every node is confirmed by `tac_step` as
  it is chosen; `node_sx` renders the block.
- **Rule 6.** `kernel/prove.shard`: the loader's mode (`Load.mode`:
  `off`, `solve`, `measure`; `read_decl_autos` and the reader's proof
  path take it). Under `solve` an `auto` without a valid entry is the
  engine's and its block is replayed through `tac_prove` before the
  theorem is accepted (`RDSolved` → the record `PROVED NAME fp=N (by
  …)`); unsolved is `PENDING NAME auto_unsolved fp=N GOAL`. The
  sidecars are rewritten whole (`write_sidecars`: the file's existing
  entries not superseded, then the new ones in the file's order, under
  a header with the engine's version), the exit code refused + errors
  + unsolved. Under `measure` every hand-written proof is read as the
  build reads it and the engine is also run on the statement (`ENGINE
  NAME solved|unsolved`, `ENGINE: solved S of N`).
- **The example.** `v3/examples/auto/auto.shard`: twelve theorems,
  every proof `auto`, one with the hint `(induct xs)`; eleven closed
  (`rfl`, `decide`, `arith only` with the certificate, three
  inductions with `simp_only` over the equations and the hypothesis,
  the generalized accumulator), `beyond` (`x ≤ x * x`) pending
  `auto_missing` at build with the fingerprint an entry needs. The
  sidecar `auto.auto.shard` is committed; `kernel/test/engine_test.sh`
  replays it under `load.shard` (no `PROVED` record: the build does
  not search), regenerates it into a scratch copy and requires the
  blocks byte-identical (the fingerprints masked: a statement's
  fingerprint carries its module's name, and the copy is another
  module — found by the test).
- **The count over calc** (the third ruling): the engine closes
  **54 of calc's 100 claims** unaided (`engine_calc.txt` lists them;
  `engine_test.sh` requires that exact set, so a change in the
  engine's reach is a report, not a drift). What it does not close:
  the lemmas over `show`'s well-founded recursion, the case analyses
  that need an equation name, the chains of rewrites with lemmas the
  statement does not mention, the descent obligations. The claim list
  `calc_claims.txt` is shared with `tactic_test.sh`.
- **Not built:** `(lemmas …)` beyond the facts and rules it adds
  (no instantiation); a `cases` form in the ladder; an engine-written
  `fulfills` in the example.

**Slice 3.22 as built, landing 3** (2026-10-08):

- **Rule 7, the store.** `kernel/store.shard` renders an admitted
  constant of K's environment as the records K ingests — lean4export's
  NDJSON, format 3.1.0, field for field what `k/import.shard` reads:
  the name, level and expression tables the declaration cites,
  numbered from 1 in first-use order (0 is `anonymous` and `zero`, as
  the exporter's), then the declaration record. Table nodes are shared
  by *strict* structural identity (binder names and infos compared,
  which `expr_eq` ignores), so a record carries the term as elaborated
  and two runs over one elaboration write one text. An inductive type
  is written as its block — every type, constructor and recursor K
  generated for it (`T.rec`, then `MAIN.rec_1…` for the nested
  auxiliaries), with the fields the import validates against K's own
  regeneration, T0's validation a second time. `load.shard --store
  DIR`: after a clean load, each ACCEPT's constant and each PARAM K
  holds (a view's parameter, the axiom K admitted for it) is written
  in admission order — `DIR/ID.ndjson` per declaration, `DIR/order`
  listing `ID NAME`. ID is the content fingerprint (`identity_hash`,
  a report's fingerprint, never an authority), or `FP-K` for the
  first free K when a file of that name holds another text; a file
  already holding the text is shared and listed once; `order` is
  appended, never rewritten, so several loads fill one store in
  sequence and a shared module's declarations come from the first
  load that admitted them. A failed load writes nothing; nothing in
  the loader changed, the writer reads the records and the
  environment after the load.
- **Rule 7, `verify_release`.** `kernel/verify_release.shard [-v] DIR
  CHUNK…` imports K, the host and the JSON reader and nothing else:
  the Init chunks replay as one stream (the T0 machinery), then each
  store file in `order`'s order as *its own stream over the
  environment so far* — fresh tables, as the file was written, so a
  record a file lacks is MALFORMED there and never read from an
  earlier file's table (found by the test: with the tables persisting
  across files a dropped expression record resolved to the previous
  file's node). Each accepted declaration is held to its order line:
  the name and the content fingerprint must be the line's, else
  MISTIED — a dropped name record lands a theorem under a shorter
  name, the import reading a missing table index as anonymous. The
  gate's guarantee is K's acceptance; the tie says the bundle is the
  build's. `RELEASE: declarations N  accepted A  rejected … malformed
  X  mistied Y  missing Z`; the exit code is the declarations not
  accepted + malformed + mistied + missing + the chunks' failures.
- **The gate.** `kernel/test/store_test.sh` (the suite's 36th
  entrypoint): the example's 23 declarations stored once, a second
  load sharing all 23, K alone accepting them after the Init fixture;
  a theorem whose value is replaced by its statement REJECTED, an
  expression record removed MALFORMED, a file removed MISSING, a name
  record removed MISTIED, each with a non-zero exit; then calc's five
  leaves stored in sequence into `v3/release/calc` and replayed —
  **376 declarations, every one accepted by K alone**, 3.5 MB. The
  store is the v3 job's artifact (`.gitlab-ci.yml`), ignored by git,
  never committed (the ruling's second lean).
- **Found on the way:** three matchers of the example share a
  fingerprint (`len.match_1`, `app.match_1`, `rev_onto.match_1` are
  one term under three names — the hash covers the type and the value,
  not the name), so the collision suffix is exercised on the first
  store written.
- **Not built:** the implementation forks of views (§6.6) are checked
  and recorded under IMPL but not stored — their constants live in
  the fork, not the load's environment, and a linear stream cannot
  redeclare a parameter's name as its implementation; a store over
  forks is a branch per view, open until a consumer needs it. The
  store is per root; a release across roots is not a case yet.

#### Slice 3.23

**Slice 3.23 — the Init cache: K's verdict on the pinned export,
admitted under a receipt** (ruled 2026-10-08 on GPT-6's R75; §13 item
57; records §4.11). The boundary after 3.22 held two items accepted
from the trajectory review with their work deferred: the feedback
cost (R75) and the connected path (R76). The user ruled the cache
first, shallow — "something I expect us to iterate as we optimize and
rework things" — and the path after it.

*Retired at slice 3.26 landing 3 (2026-10-09, the user's ruling).* The
cache answered the cost of streaming the export's prefix through K,
which slice 3.26 removed: a load reads and checks the closure it
cites, seconds. Measured there with a hand-written receipt over the
export, admission saved a tenth to a fifth of a demanded load (calc 9 s
against 10 s, `std/bytes` 20 s against 25 s): the walk and K's
ingestion are the cost, and a receipt skips neither. Its only writer
would have been the full replay, which nobody runs locally and which
CI runs after the suite. Rejected because: a cache whose measured
value is seconds a load for a developer holding a 41-minute replay,
kept at the price of an admit-unchecked switch in K's run with no
consumer. What stays of the slice: `admit_decl_pinning` and the run's
admit mode, repurposed at 3.26 landing 2 for the loader's second
admission of records it checked a moment ago (`od_admit`). The rules
below are the slice as it was built.

*What the cost is.* Every load streams the pinned export through K up
to the deepest name its modules cite (§3.1), and K checks every
declaration on the way. Measured on the development machine over the
two fixture chunks (3,523 declarations, 14 MB):

| step | wall |
|---|---|
| reading and JSON-parsing every record | 9.2 s |
| and building the name, level and expression tables | 13.1 s |
| and K's check of every declaration (the T0 driver) | 47.2 s |
| the same with the typing judgments skipped (admit mode, below) | 17.2 s |

The check is 72 percent of a load's Init cost; the parse and the
tables are the rest, and any on-disk form must build the same tables.
A serialized environment therefore had a ceiling of a few seconds per
load against a second format behind K's seal; the lever taken skips
the check and keeps the export as the only form.

**The rules of slice 3.23:**

1. **The receipt.** `t0.shard --receipt FILE`, after a run that
   accepted every record of its chunks (nothing rejected, exhausted,
   mismatched or unsupported), writes one line per chunk: `PATH
   BYTES`. A run with anything not accepted writes nothing. A line
   starting with `#` is the writer's and is not read by the loader.
2. **Admission.** `k/add.shard`'s `admit_decl_pinning` is the
   environment effect of `check_decl_pinning` with the typing
   judgments of an axiom, definition, theorem or opaque skipped: the
   constant is inserted as the declaration states it, the node
   watermark and the accelerator pins exactly as on the checked path.
   An inductive block and the quotient take the checked path. The
   import's run state carries the mode (`run_admitting`); nothing else
   in K changes, and `check` — the raw entry of §3.5 — is untouched.
3. **The loader.** `load.shard --init-receipt FILE` (and
   `prove.shard`'s, the loader pins test's) sets the receipt on the
   load; a chunk listed with its byte count streams in admit mode, a
   chunk not listed or listed with another byte count is refused
   (`init_receipt_stale`) before any record of it is read. Without a
   receipt every chunk is checked, as before. The records and the
   counts print exactly as without the flag — the loader pins' 212
   expected outputs are the equivalence gate.
4. **Currency.** `kernel/test/init_receipt.sh` keeps
   `v3/.cache/init.receipt` current and prints its path: K's identity
   for the receipt is a hash of the sealed module's sources, the
   kernel files it imports and the fixture, written as the receipt's
   `# k=` header; a mismatch regenerates (one T0 run over the
   fixture). `v3/test.sh` runs it once and exports `INIT_RECEIPT`;
   the shell tests add `--init-receipt` when it is set. The cache
   directory is ignored by git; the receipt is never committed and
   never an artifact.
5. **The gate is untouched.** `verify_release` takes no receipt: the
   release bundle is replayed by K alone over the export, as rule 7 of
   slice 3.22 states. The T0 full replay on CI checks the whole export
   as before; the receipt is a cache of that verdict for the
   development loop and the suite.

**Slice 3.23 as built** (2026-10-08): as the rules. The auto example
(`--root v3`, Init streamed 1,509 declarations into the second chunk)
loads in 8.3 s under the receipt against 13.7 s without, the records
byte-identical; a receipt with a wrong byte count refuses the load
(`LOAD-ERROR … init_receipt_stale`, modules 0, exit 1); an unreadable
receipt exits 2 before any load. The 221 loader pins pass byte-identical under the receipt in 809 s on the development machine (beside parity and the define test; 1,361 s without the receipt beside the full suite — the clean comparison is pipeline 553's against 552's 3,327 s on the runner); parity byte-identical over 30 closures and 130,445 declarations; route 1's native driver, rebuilt locally, ties the interpreter on the first chunk and writes the receipt; the full suite's 36 entrypoints 0 failed in 927 s, the pins entrypoint its wall clock still.

#### Slice 3.24

**Slice 3.24 — the connected path: law §12.4's path assembled, the
branch-local proof joint built, each joint broken** (ruled 2026-10-09 on
GPT-6's R76; §13 item 58; law §12.4, §12.5 T1, T5). The second boundary
item after 3.22. Law §12.4 names the first connected path — *one
imported logical declaration → one checked E realization → one caller
using a branch-local proof → a claim constructed through I → its
retained P verified without the elaborator → repeated execution through
a prepared handle* — assembled incrementally as each capability lands,
then broken deliberately at each joint, "to learn whether the
interfaces compose at tolerable cost before a large library, the full
engine or a certified host exists". Where the joints stand: an
imported declaration enters by citation and a `(realize NAME (view))`
attaches Init's own body (slices 5b, 3.18); a claim through I is
slices 3.20–3.21; retained P verified by K alone is slice 3.22's rule
7 with its tamper cases; the prepared handle is phase 4's (T6). The
one phase-3 joint still missing is the **caller with a branch-local
proof** — T1's "a `dite` whose `h` is used only in a `Fin.mk` field",
carried since slice 7 because `Fin n` needed a value parameter at E.
Slice 3.18 removed that obstacle (`Fin n` is `Fin`, `Fin.mk i h` erases
to `i`); what is still missing is the source form — an `if` elaborates
to `ite`, so its branches have no hypothesis a program can cite, and
the dependent form is elaborated only inside a recursive function for
the descent obligations (slice 3.13 rule 2), its hypothesis never
named in source. The erasure already reads `dite` back as an `if`
with the branches applied to their erased proof.

**The rules of slice 3.24:**

1. **`(dif h C T F)` — the dependent if, its hypothesis named.** `C`
   is elaborated as an `if`'s condition is (a proposition with a
   decision the elaborator knows, or a `Bool` as `C = true` with
   `Bool`'s decidable equality); `T` is elaborated under a local
   `h : C` and `F` under `h : ¬C`, each at the form's expected type
   (the then-branch's own type when the position leaves it open, as
   an `if`'s); the term is `dite C dec (λ h. T) (λ h. F)`, Lean's
   elaboration of `if h : c then t else e`. The name comes first, as
   `fn` and `let` put a binder's name first; `if` is not overloaded
   with a fourth argument, which a reader would have to count. A
   value of a two-constructor inductive is not a condition here
   (`dif_type`): there is no proposition to name. On the E-first
   route (a file without Init) the form is refused by name
   (`dif_needs_init`): `Decidable` and `dite` are Init's. The
   erasure is the existing one; the equations of a measured
   function see the branch as they see an `if`'s.
2. **The example, `v3/examples/path/`.** The path on one program: an
   imported Init function with its realization attached; a
   bounds-checked access `at xs i` that returns the element under
   `(dif h (< i (List.length xs)) …)` through `Fin.mk i h` and
   `none` otherwise; a theorem about it constructed through I; an
   entry that runs it on raw arguments; the store written from the
   load and replayed by K alone.
3. **The test, `kernel/test/path_test.sh`, one entrypoint.** The path
   in order — load, run on a valid argument, store, verify by K alone
   — then the law's five breaks, each required to fail by name: a
   wrong executable body in a `realize` (refused at the equation
   check), missing bound evidence (the `Fin.mk` outside its branch,
   or with the hypothesis dropped, refused by the elaborator), a
   mismatched revision (the sidecar's entry stale, the store's order
   line mistied), a tampered result (a stored proof replaced, rejected
   by K), an invalid raw argument (refused at the entry, exit 6). The
   breaks slice 3.22 already tests are re-exercised on this example,
   not re-implemented.
4. **The measurement.** The cost of the whole path on the example,
   load through verify, is R76's number; the as-built reports it.

*Not in this slice, stated:* the prepared handle (T6) and the
World-alias fixture are phase 4's; the example stays pure.

**Slice 3.24 as built** (2026-10-09): as the rules, with what the
path found.

- **Rule 1.** `elab_dif` in `kernel/elab.shard`: the condition as
  `elab_if` reads it; each branch elaborated under its hypothesis as
  an `el_local`, closed by `el_lams`, the result type held to be free
  of the hypothesis (`dif_type` otherwise); `dite` applied by
  `op_apply_explicit` at the expected type. The classifier refuses
  the form by name on the E-first route (`dif_needs_init`). Pins:
  `dif_ok` (a `Fin.mk` field, a `Bool` condition, both branches'
  hypotheses in theorems by `dif_pos`/`dif_neg`), `dif_type`,
  `dif_needs_init`, `dif_scope` (the hypothesis outside its branch:
  the typed route's `unknown_constant: h`, carried by the E-first
  refusal that follows).
- **Rule 2, the example.** `at` returns `Option (Fin (List.length
  xs))`; `pick` takes the index, its parameter type dependent, and
  reads through `List.get?Internal`; `read` composes them; the entry
  prints the digit at a raw index of a fixed list. `List.length` and
  `List.get?Internal` are realized by supplied bodies with their
  equations proved: `get?Internal`'s structural recursion on the
  index is an `if` (`Nat`'s constructors are not E patterns), its
  second equation by cases on the index, named in the `equations`
  clause. The claims `at_some`, `at_none` are `(by (unfold at) (rw
  (dif_pos h)) rfl)` and its `dif_neg` twin. Fifty-six declarations
  stored, every one accepted by K alone.
- **Rule 3, the test.** `kernel/test/path_test.sh`, the suite's 37th
  entrypoint, 18 checks: the path in order, then the five breaks — a
  wrong body refused at `List.length.realize_2` (`conversion`), the
  hypothesis cited outside its branch (`unknown_constant: h`), the
  `Fin.mk` without its proof (a `type_mismatch`: a function of the
  bound where a `Fin` is expected), the order line naming another
  declaration (MISTIED), the stored proof replaced by its statement
  (REJECT), a non-number at the entry (exit 6).
- **Rule 4, the cost.** On the development machine under the Init
  receipt: load 18 s, load and run 23 s, load and store 18 s, verify
  by K alone 47 s (45 s of it the Init fixture's replay, the 56
  declarations the rest); the test 279 s. The path composes; its cost
  is the Init stream four times over, which slice 3.23 halved and a
  load shared across a process's roots (slice 3.25) removes for the
  pins; a driver pays it once per load.
- **Found on the way, open:** `List.get`'s own realization — the
  access with the `Fin` as its argument — needs a match that
  generalizes a local whose type mentions the scrutinee (`i : Fin
  (List.length xs)` under `match xs`), so that the nil row can refute
  its bound; today only the expected type is generalized (slice 3.19
  rule 9), and slice 3.13 rule 2's narrower guarantee states the
  gap. The path reads through `List.get?Internal` on the index's
  value instead, the proof carried by the type. The dependent match
  has its consumer now; it is not built here. Also: a `(realize NAME
  (view))` of an Init definition whose body is a matcher
  (`Option.getD`) is refused for want of the matcher's realization —
  Lean's `Option.getD.match_1` is not what K regenerates from its
  type — so the example writes its own `or_zero`.

#### Slice 3.25

**Slice 3.25 — the shared Init load: the stream replayable, the
horizon the module's, Init's names Init's** (ruled 2026-10-09 at the
boundary after R75 and R76, the lever item 57 stated; §13 item 59). The
loader pins' entrypoint re-streamed Init once per case in one process:
2,227 s of the v3 job's 5,501 s on the runner, 783 s locally, the
suite's wall clock since slice 3.20 and the first cost the library arc
would multiply. The export is one fixed library; a process that loads
many roots has no reason to read it more than once. Three rules:

1. **The Init stream is a prefix of a load and replayable.** `init_all`
   streams every chunk to its end (the record `INIT (all): N
   declarations admitted`); `ld_reroot` starts a fresh root load from a
   load's Init state — the checked environment, the stream's state, the
   ordinal table, the chunks and the receipt carried, everything of the
   old root reset to `load0`'s. A driver's lazy stream is unchanged: a
   program pays the prefix its imports reach, once.
2. **The horizon is the module's, never the stream's.** `(import Init
   NAME)` is answered from the ordinal table — NAME admitted = NAME in
   the table — and a module's horizon is its imports' ordinals and its
   imported modules' horizons (slice 3.21b rule 2), with no fall-back
   to how far the process happened to stream. Every gate that asks
   whether an Init constant is available to a module asks the scope
   (`visible`: below the module's horizon), not the environment: the
   elaborator's `first_missing` (an `if`'s `Decidable`, `dite`, `Not`,
   `Int.ofNat`), `arith`'s kit, the measure's `WellFounded` kit, a
   `"…"` literal's `String`. **Init's names are Init's:** a declaration
   bearing the name of an Init constant is refused (`name_taken`)
   wherever the environment holds that constant, as K admits no second
   constant under a name (`already_declared`) and the release gate
   holds the whole export; a driver whose lazy stream stopped short
   accepts what the gate refuses — the one place the lazy stream and the
   gate disagree, stated, closed at the gate. (The claim made the same
   day that `v3/std/list.shard`'s `List.sum` was such a case was wrong:
   that file declares `std.list.List.sum`, as §3.1 says every declared
   name carries the module path, and K holds it beside Init's
   `List.sum`; the live shape is a module named as an Init namespace —
   the pin `registry_native`'s `Bool` — which slice 3.26 rule 5 refuses
   from the name table wherever it loads.)
3. **The loader pins' entrypoint streams once.** The two fixtures are
   streamed to their end under the receipt, each case loaded fresh on
   that state by `ld_reroot`; a case's records are its own module's,
   by rule 2. The three cases that leaned on the stream stopping short
   are restated: `str_no_string` and `arith_unreached` hold by the
   horizon, `registry_native` is the refusal of rule 2 (the finding it
   pinned stands for a lazy driver, `define_test.sh` reads that dump).

**Slice 3.25 as built** (2026-10-09): as the rules.

- **Rule 1.** `kernel/loader.shard`: `init_all` is `init_through` with
  the target `Anon`, which the stream's end answers; `ld_reroot l root`
  is `load0` with the environment, the Init state, the ordinals and the
  receipt of `l`. Nothing of a driver changed (`load.shard`,
  `prove.shard` stream lazily as before).
- **Rule 2.** `init_through` consults `ords_find` on the ordinal table,
  not `env_find`; `init_horizon` has no fall-back (a name not in the
  table leaves the module's horizon). `elab.shard` `el_sees` (in the
  environment and `visible` from the module's scope) is the one
  availability test: `first_missing` and `int_of_nat_visible` through
  it, `tactic.shard`'s `has_env_const` replaced by it, `arith.shard`'s
  `ar_has` and the linearization's environment parameter replaced by
  the elaborator state, `classify.shard`'s `read_estr` asking
  `visible` before the environment. `derive.shard`'s `dv_has` already
  asked both. The loader's `name_taken` stays the environment's: K's
  rule.
- **Rule 3.** `kernel/test/loader_pins_test.shard` builds one load of
  the two fixtures under the receipt, streams it by `init_all` (its
  records printed first, `INIT (all): 3523 declarations admitted`),
  and runs each case on `ld_reroot` of it through the kit's
  `load_all`; the three pins restated as rule 3 says. The entrypoint:
  225 cases in 33 s under the receipt against 783 s (the Init stream
  18 s of it; the cases 15 s), and 57 s without a receipt, the
  checked stream once.
- **Not changed, stated:** the shell tests are one load each and stay
  lazy; `path_test`'s four loads of the Init stream (slice 3.24 rule
  4) are four processes, which a shared load in one process does not
  reach — the persistent session of law §9.3's prepared handle is
  phase 4's.

#### Slice 3.26

**Slice 3.26 — Init on demand: the export indexed, a module's closure
read by range, no fixture** (ruled 2026-10-09 at the boundary after
slice 3.25, on the library arc's first question; §13 item 60). The
library arc cites Init wherever Init has the statement, and an import
costs its position in the export (slice 3.17's measurement above): a
far lemma such as `Int.ediv_emod_unique` at line 3,409,839 would admit
half the export. The loader parses the export in shard under the
bootstrap at 0.8 MB/s (the two fixtures, 14.3 MB, in 18 s under the
receipt); the export is 333 MB; the whole of it checked under the
interpreter is 76 minutes and 8 GB resident; and the receipt caches
K's verdict, not the environment. Measured on the export: the closure
of `Int.ediv_emod_unique` — every line its type and value reach,
through the hash-consed nodes — is 690 records and 35,076 lines, under
one percent of the export; `Nat.and_comm` 746 and 42,468; `List.sum`
25 and 686. A closure cut at the granularity of a record's block (the
lines a record emits before its own) is 15,482 blocks and 2,240,409
lines for the same lemma, a third of the export: a shared node lives in
whichever block first needed it. So the unit is the line, and a load
reads what it cites. Six rules:

1. **The index.** One pass over the export (`kernel/index.shard`;
   `initindex.shard EXPORT INDEX NAMES`) writes two tables beside the
   receipt, never loaded whole: the record table — one fixed-width row
   per record in export order: its kind, the byte offset and length of
   its block, and the first name, level and expression id the block
   holds (or, holding none, the next id the export assigns), so the
   block holding an id is the last row whose first id is at most it —
   and the name table, one fixed-width row per declaration name as
   `show_name` renders it, sorted bytewise with its ordinal, so a name
   is a binary search of range reads. Node ids are the export's,
   assigned in emission order (checked over the export), which is what
   makes a row per record enough. The header rows carry the export's
   byte count; an index over another export is refused. The pass
   refuses an ambiguous rendering (two names rendering alike) and a
   name past the row's width.
2. **The range read.** `host.shard` gains `read_range path off n`: at
   most n bytes from off, fewer at the file's end, `None` where the
   file cannot be read — beside `read_file`, in the bootstrap's handler
   and the chain's runtime. The loader reads the export itself
   (`init.ndjson`); the 20,000-line chunks stay the replay's.
3. **The load.** `(import Init NAME)` takes NAME's ordinal from the
   name table as the module's horizon (slice 3.21b rule 2 unchanged).
   A module's demand is every identifier of its forms under the
   candidates resolution tries (`scope_candidates`: the module's
   prefix, the opened prefixes, `Init.NAME`, the bare name) and the kit
   lists the gates hold (the elaborator's `first_missing` lists, the
   `WellFounded` kit, `arith`'s kit, the derivations', `String`),
   filtered to the names the table places below the horizon. The
   loader walks each demanded declaration's closure by range reads —
   its record line, the nodes its type and value reach, their names
   and levels, the records of the constants they name — sorts the
   lines into export order and feeds K that sub-stream through the
   path the prefix stream took (`process_record`), checked (admitted
   under a receipt, as ruled; the receipt was dropped at landing 3).
   Node ids stay the export's; K's tables are sparse; the records are
   byte-identical to the stream's.
4. **The law unchanged.** Visibility is the ordinal below the horizon
   (`init_within`); a materialized record carries its true ordinal, so
   a far name one module pulled in stays invisible to a module with a
   nearer horizon. Resolution asks the environment first
   (`const_arity`), so a name not materialized is simply absent; a
   name the elaborator constructs that no kit list names is a refusal
   by that name (`unknown_constant`) and a defect of the list, which
   frontend parity finds. No materialize-and-retry: one mechanism.
5. **Init's names are Init's, from the index.** A native declaration
   bearing a name the name table holds is refused (`name_taken`)
   wherever it loads — the driver and the release gate agree; §13 item
   59(c)'s index, as reopened on `List.sum`.
6. **No fixture.** The prefix fixtures (`init_prefix_int.ndjson`,
   `init_prefix_str_tail.ndjson`, `init_prefix_3000.ndjson`) and their
   dependents go to the export, which `v3/export.sh` provides on every
   machine; a closure checked costs seconds (K's check about 7 ms a
   record: the fixture's 24 s over 3,523), so no test needs the
   receipt (ruled a speed lever here; dropped at landing 3, measured at
   a tenth to a fifth of a demanded load); `init_index.sh` keeps the
   index current as `init_receipt.sh` kept the receipt.

Three landings: (1) the index, the range read, a test that reads every
record back by its offset and every name by its search; (2) the loader
on demand — the prefix stream, the stream state and the chunk list
deleted, the demand set, the walk and the sub-stream, `name_taken` from
the table, the fixtures deleted — under the loader pins, parity and the
suite; (3) the receipt over the export, CI in the order export, build,
index, suite, replay. Weighed and deferred, each with the consumer that
would reopen it: a theorem admitted by its statement alone (its closure
five to ten times smaller — 143 records and 3,098 lines for
`Int.ediv_emod_unique` — but a change to K's admit path, which builds
the declaration from the node tables); the loader native (its files are
not route 1's); a serialized environment (Lean's `.olean`), whose only
consumer, the replay, checks. Rejected: a native parse primitive (a
second JSON reader for a second per load), a materialize-and-retry loop
(a second mechanism), the exporter emitting the index (upstream at a
pinned commit; offsets are a property of the text held here).

**Slice 3.26 as built, landing 1** (2026-10-09): the index and the
range read, as rules 1 and 2.

- **Rule 1.** `kernel/index.shard` is the library — the two formats,
  the pass (`ix_build`: windows of the export by range, every line
  placed, the name lines entered, the record rows rendered, the name
  rows sorted), the readers the loader will use (`ix_read_header`,
  `ix_read_row`, `ix_lookup`: the binary search of the name table;
  `ix_block_of_id`: the last row whose first id is at most the id) and
  the check (`ix_verify`: a second pass over the export comparing every
  record's row and looking every name of it up, the export's byte count
  against the header's, the name table's order). `kernel/initindex.shard`
  is the driver, `test/init_index.sh` keeps `v3/.cache/init.index` and
  `init.names` current over the export (stamped by the export's pin and
  size and the index's sources: a kernel edit does not stale it).
  Measured: the two fixtures (3,523 records, 3,985 names, 14.3 MB)
  build in 46 s and verify in 63 s; the export (347,714,179 bytes, 57,977
  records, 59,433 names) builds in 952 s and verifies in 866 s on the
  bootstrap — once per environment. Compiled by route 1 (`v3/build.sh v3/kernel/initindex.shard
  v3/bin/initindex`, 4 s with `bin/shard_eval` booting the chain; the
  pass's state is a plain type with accessors, as K's `Run`, since the
  chain compiles no record sugar) the driver indexes the export in 68 s
  and verifies it in 71 s, the tables byte-identical to the bootstrap's;
  `init_index.sh` prefers the binary when it is newer than the sources. The index is 62 bytes a record and
  218 a name: 3.6 MB and 13 MB over the export.
- **Rule 2.** `read_range` in `host.shard`, the bootstrap's handler
  (`eval.rs`) and the chain's runtime (`tools/codegen/rt.h`,
  `rt_read_range`); the export's single file is read, never a chunk.
- **The test.** `test/index_test.sh` (the suite's 38th entrypoint) builds
  the fixtures' index, verifies it, requires the record count K's stream
  admits and the name rows of a definition, an inductive block's type
  and its recursor, and refuses three tampered inputs by name: a name
  row's ordinal changed, a record row's length changed, the export a
  byte longer than the header says. The driver is in parity's closures.
- **Found on the way:** export lines carry their keys in alphabetical
  order, so an expression line reads `{"app":…,"ie":N}` and the id is
  found by its key, not its position; the fixtures' records are 3,523
  as K counts them, and the quotient's four records each name their own
  constant.

**Slice 3.26 as built, landing 2** (2026-10-09): the loader on demand,
as rules 3–6; the fixtures deleted.

- **Rule 3, the walk** (`kernel/initload.shard`): the export and its
  index opened at a process's first need (`od_open`: the meta line
  against the pin, the index's header, the export's size the header's);
  a name's ordinal by binary search, memoized, the search's top ten
  levels of rows cached (`od_lookup`); the closure of a set of
  demanded records walked by range reads (`od_load`): the blocks
  scheduled highest first, a block read once and scanned from its
  record line backwards, every pending line parsed and its name, level
  and expression ids pushed — a child precedes its parent, so in the
  same block it comes later in the scan and in an earlier block it
  schedules that block — and a constant an expression cites resolved by
  its name id through the **declaration table** (the third table, 9
  bytes a name id, added here: without it a constant's record is found
  only by rendering its name, which needs the name's ancestors, which
  come later in the walk) to the record declaring it, which is pushed.
  The lines come out ascending (sorted only when a push went upward)
  and go through K's `process_record`: node lines into K's tables,
  sparse, the ids the export's; records checked, or admitted under a
  receipt. What K has been fed persists (`OdDone`), so a line is fed
  once in a process. **The demand** (`loader.shard` `init_prepare`,
  after a file's directives): every token of its forms under
  resolution's candidates (`scope_candidates`, the token's `@` sigil
  dropped, the candidates under a prefix the export declares nothing
  under skipped — the file's own identity, an opened native module),
  the gates' kits — K's and the pins' (`k_kit`), the elaborator's
  (`el_kit`), the tactics' (`tc_kit`), arith's (`ar_all_kit`), the
  derivations' (`dv_kit`), the definer's (`df_kit`) — and, for an
  inductive block's names among them, their satellites; the names the
  table places below the horizon, their closure through K. `(import
  Init NAME)` takes NAME's ordinal from the name table (`init_through`;
  a name the export does not declare is `init_name_not_found`) and the
  record reads `INIT NAME: horizon H, N declarations admitted`.
- **K's run of Init alone.** The loader's Init state holds K's run fed
  Init's records and nothing else; each demand is checked into it
  (`od_load`) and then admitted into the loader's own environment,
  which holds the natives beside Init (`od_admit`: the same lines over
  the run's tables, without the typing judgments, as under a receipt).
  `ld_reroot` starts a fresh root on that run's environment, so a
  process loading many roots loads each record once: the loader pins'
  entrypoint runs its 225 cases in 42 s, every record checked, against
  33 s admitted under the receipt and 57 s checked at slice 3.25; the
  first form of this landing, re-rooting from the state before any
  demand, took 1,761 s.
- **Rule 4.** Unchanged in code: `init_within` on the true ordinals
  (`ld_add_inits` enters each admitted constant with its ordinal);
  `ACCEPT … init=H` carries the module's horizon. Three constructed
  names the kits lacked were found as the law says, by a refusal:
  `Lean.Omega.Int.mul_congr` (arith composes it from the operator;
  `calc_app`'s descent proof and the `arith_div` pin), the `@` sigil
  on a token (`std/derive.shard`'s `@String.toByteArray_inj`), and the
  `init_not_found` pin, which asked for `Int.tdiv` past the fixture's
  end and now asks for a name the export lacks.
- **Rule 5** (`name_taken`): a file's `Fx.taken` is every name the
  export declares under the file's identity (`od_names_under`, one
  search and a forward read), and a declaration — the file's own, a
  derivation's — bearing one is refused `name_taken`; the release gate
  looks each stored name up in the table and refuses it as `TAKEN`.
  Found running it: the `List.sum` case item 59(c) was reopened on is
  no case — `v3/std/list.shard` declares `std.list.List.sum` (§3.1), K
  holds both — and the live shape is `registry_native`'s module `Bool`;
  `define_test` now requires that refusal where it used to read the
  lazy driver's acceptance.
- **Rule 6.** The three prefix fixtures and `init_receipt.sh` are
  deleted; every consumer reads the export and its index (`test/
  init_index.sh` prints both paths; `v3/test.sh` keeps the index
  current where it kept the receipt): the shell tests through
  `--init EXPORT --init-index INDEX`, the loader kit's tests through
  `kit_load0`, `calc_harness.shard EXPORT INDEX INPUTS`,
  `verify_release.shard [-v] DIR EXPORT INDEX` (its demand: every
  constant the store's records cite, resolved through the index and
  checked, never admitted), `route2_test` and `t0_fixture_test` on the
  export's first 3,000 lines against the full oracle, `index_test` on
  its first 20,000 (519 records, 691 names, 3,153 name ids; 9 s
  against 125 s). The receipt stays a flag (`--init-receipt`, the
  export listed at its byte count), with no writer until landing 3.
- **Measured** (bootstrap, checked, no receipt; the loads in parallel):
  `examples/calc/calc.shard` 12 s (its demand through `Int.decEq`: 667
  records), `calc_spec` 29 s, `calc_app` 28 s, `natabs` 26 s,
  `calc_proof` 34 s (through `Lean.Omega.Int.ofNat_lt_of_lt`: 1,274
  records), `examples/auto` 24 s, `std/bytes` 24 s (through
  `UInt8.toNat`: 1,305), `std/list` 10 s, `std/derive` 26 s,
  `examples/path` 22 s (through `List.get?Internal`: 1,155); the
  nested-import pin 1 s. The index is one file of 19,208,769 bytes,
  built by the route-1 driver in 36 s and verified in 36 s (the two
  tables of landing 1: 68 s and 71 s).
- **Gates run:** the 225 loader pins 0 failed in 42 s, `loader_test`
  0 failed, the full suite's 38 entrypoints with 0 failed in 455 s against 613 s at landing 1 (parity byte-identical over 31 closures and 133,465 declarations in 267 s; `store_test` 350 s, the suite's wall clock; `path_test` 228 s; `define_test` 230 s; `calc_test` 215 s; `k_clients_test` 69 s; the loader pins' entrypoint 42 s). None of the files route 1 compiles changed; the full replay and the corpus are CI's.
- **Found on the way:** K's `unknown_constant` refusal names the
  declaration, not the constant it lacks (a kit's defect was found by
  instrumenting a scratch copy of K to carry the name) — a diagnostic
  worth K's refusal carrying it, deferred; the block offset of landing
  1's record table was the newline before a block's first line (one
  byte early, the verify tolerating it), fixed with the format.

**Slice 3.26 as built, landing 3** (2026-10-09): the receipt dropped,
the slice closed. The landing as planned gave the receipt its writer
over the export; pipeline 563 and a measurement retired it instead.

- **The measurement.** With every load checking the closure it cites,
  the entrypoints made of many short loads doubled on the runner at
  landing 2 (`wire_test` 155 s to 436, `define_test` 337 to 657,
  `engine_test` 267 to 392; `wire_test` 65 s to 150 locally), each
  process checking its demand where the receipt had admitted the
  fixture's prefix. A hand-written receipt over the export (`PATH
  BYTES`) admitted a demand in nine tenths to four fifths of the
  checked time: calc 9 s against 10 s, `std/bytes` 20 s against 25 s.
  The walk on the bootstrap and K's ingestion are the cost; a receipt
  skips neither. The user ruled the receipt dropped.
- **Deleted.** `load.shard --init-receipt` and `prove.shard`'s, the
  loader's receipt (`Load.receipt`, `receipt_mode`, `receipt_parse`,
  `load_with_receipt`), `t0.shard --receipt` and its writer, the
  loader pins test's flag, `v3/test.sh`'s `INIT_RECEIPT` and the
  shell tests' pass-through. A driver starts from `ld_start` (the
  export and its index named, or no Init). `init_receipt_stale` is no
  refusal. K's admit mode (`admit_decl_pinning`, `run_admitting`)
  stays for the loader's second admission of records it checked into
  K's run of Init a moment ago (`od_admit`), its comments saying so;
  slice 3.23's text stands as history, marked retired, and §13 item
  57 with it.
- **The lever that remains** is item 60(f)'s loader native: the walk
  is the cost of a demanded load, and its files are not route 1's.
- **CI's order** stands as landing 2 forced it: export, the index
  driver, the index, the suite, the kernel build, the replay.
- **Gates run:** the 225 loader pins 0 failed in 46 s (43 s inside the suite), `loader_test` 0 failed in 8 s, `t0_fixture_test` and `route2_test` byte-identical (route 2 in 26 s), the full suite's 38 entrypoints with 0 failed in 620 s against 455 s at landing 2 — every entrypoint about a third slower while the pins' entrypoint held at 43 s against 42 s, so contention on the machine, the per-load cost unchanged (parity byte-identical over 31 closures and 133,392 declarations in 354 s; `store_test` 484 s, the suite's wall clock; `path_test` 314 s; `define_test` 303 s). `t0.shard` is route 1's: the kernel build, the full replay and the corpus are CI's.

#### Slice 3.27

**Slice 3.27 — the library arc: Init is the library, the old modules
become records, the seams the probes found are closed** (designed
2026-10-09 at the boundary after slice 3.26, on the user's "continue
with the next arc"; §13 item 61 — **ratified 2026-10-10 as designed,
both leans taken: calc's `list.shard` retires onto `std/list` at
landing 3; the migration tool's tier 0 is phase 5's opener**). What law §12.4's phase 3 still owes after slices 3.9–3.26:
`std/list`, `order`, `nat`, `div`, `bits`, `arith` under the naming
law; the fifteen former axioms of `kernel/facts.shard` as theorems; the
migration table validated; one arbitrary-`Prop` ghost refinement; one
static law-bearing package; `docs/LEAN.md`; the gates T2, T3, T9
(small) and T10. The user's hesitation at the 3.25 boundary stands as
the arc's principle: the library must not duplicate work already in
Lean's `Init`. Measured by probes before this design (three scratch
files under the on-demand loader, the bootstrap, 2026-10-09): twelve of
the fifteen former axioms are theorems by Init's names in one file of
twelve lines (47 s; 1,395 declarations admitted through the horizon
`Nat.shiftRight_zero`, ordinal 55,829); six of Init's list functions
realized by supplied bodies and seven of its list theorems cited (29 s;
1,385 declarations); five of `std/bits`' statements by Init's names
(54 s; 1,887 declarations). Three seams found on the way, each a defect
or a gap of the elaborator, not of the law: `/` at `Int` elaborates to
`Int.div`, which this pin does not hold (`Int.ediv` is Euclidean
division; `%` already goes to `Int.emod`); a lemma Init states through
an instance (`0 &&& x = 0` is `HAnd.hAnd Nat Nat Nat _ 0 x = 0`) cannot
be `apply`d to a goal in V3's spelling (`Nat.land 0 b = 0`) — the
argument under the instance application is "not determined by the
goal" (`witness_open`), while `(exact (Nat.zero_and b))` goes through
K's conversion, which unfolds the instance; and a cited name above the
module's horizon is `unknown_constant`, so the author must know the
export's ordinals to write the import (`Nat.zero_and` is 23,368,
`List.take_cons` 54,202, `Nat.min` 10,228). Found beside them: Init's
instance-polymorphic functions (`List.sum` under `[Add α] [Zero α]`)
are refused `instance_needed` at a concrete type, and `List.length_take`
states `min`, for which V3 has no operator. Eleven rules:

1. **Init is the library.** A statement Init has is cited by Init's
   name; `v3/std` declares only what Init lacks, in Init's namespaces
   under the naming law (law §5.3); an old module every declaration of
   which is Init's or `arith`'s becomes a migration record and no file.
   V3's own `List.sum` (slice 3.13's first library file) goes: it is
   Init's `List.sum` under rule 3.
2. **One spelling, everywhere.** Slice 3.21 rule 3 made `arith` read a
   class method at `Int` or `Nat` under Init's instance as its
   function; the elaborator's unifier and the rewriter's matcher
   (`apply`, `rw`, `simp_only`, the engine's ladder) do the same: an
   instance constant and a class projection applied to it unfold during
   unification and matching, as Lean's `instances` transparency does,
   so `HAdd.hAdd Nat Nat Nat (instHAdd Nat instAddNat) a b` meets
   `Nat.add a b`, `HAnd.hAnd … Nat.instAndOp` meets `Nat.land`,
   `HAppend.hAppend … List.instAppend` meets `List.append`, and a lemma
   in either spelling applies to a term in the other. V3's operator rows
   keep their function spellings (slice 3.13 rule 3); the probe's
   `witness_open` is this rule's regression.
3. **Instances by table, not search** (Stage 3's first step, law §5.1).
   An instance-implicit binder `[C α]` at a concrete `α` of the
   library's types (`Nat`, `Int`, `Bool`, `List`, `Option`, `Prod`,
   `String`, `Char`, `Fin`, `UInt8`, `ByteArray`) resolves from a fixed
   table keyed by class and type to the pin's instance constant
   (`instAddNat`, `Int.instAdd`, `instMinNat`, `List.instAppend`,
   `instDecidableEqNat`, `Int.instDecidableEq`, `instBEqOfDecidableEq`
   over a decidable equality) or to a composed term the table spells
   (`Zero Int` is `Zero.ofOfNat0` over `instOfNat`); `(min a b)` and
   `(max a b)` at `Nat` and `Int` are `Min.min`/`Max.max` under the
   table's instance — Init's one spelling, an atom to `arith` as slice
   3.21 rule 4 has it. A class or type the table lacks is
   `instance_needed` with today's pointer (write `@` and the instance).
   General instance search stays Stage 3's door, wake condition = a
   class the table cannot hold.
4. **`/` at `Int` is `Int.ediv`.** The operator row (`op_at`) says
   `Int.div`; the pin has no such constant and law §10.3's row says
   Euclidean. Fixed with a pin; the row validated by `std/migration.shard`.
5. **The horizon refusal points.** A cited name the name table holds
   above the module's horizon is refused `above_horizon NAME: ordinal
   N, the horizon H — (import Init NAME)`, the import to write, where
   today it is `unknown_constant`. The horizon law (slice 3.21b rule 2,
   3.26 rule 4) is unchanged: the author chooses the import; the
   loader, which has the table, says which.
6. **Init's functions realized by supplied bodies**, slice 3.24's
   precedent, with their equations theorems K proves: by `rfl` where
   the body follows Init's recursion (`List.length`, `List.append`,
   `List.reverseAux`, `List.reverse`), by `cases` on the argument Init
   recurses on first where the body recurses on another (`List.take`
   and `List.drop` recurse on the count first: `take n [] = []` is by
   cases on `n`) — until the derived view reads Lean's compiled
   recursion (deferred below). The matcher of an Init definition
   (`Option.getD.match_1`, slice 3.24's open item) is realized through
   the matcher's own definition — a definition over `casesOn`, a view
   of it derived as any definition's — measured at landing 3; the
   fallback is a supplied body as today.
7. **The dependent match**, slice 3.24's other open item, lands at
   `List.get`: a `match` generalizes the locals whose types mention the
   scrutinee (slice 3.19 rule 9 generalizes the expected type only), so
   that under `match xs` with `i : Fin (List.length xs)` the nil row
   refutes `i`'s bound; slice 3.13 rule 2's narrower guarantee gets its
   consumer. `List.get`, `List.getD`, `List.get?Internal` and
   `List.getElem?` are then realized as rule 6 has them.
8. **The modules as records.** For each of the six, a per-interface
   migration record (law §10.1's "per-interface record") in
   `v3/std/README.md`: every old declaration, its V3 spelling or Init's
   name, the connecting evidence (a theorem of `std/facts.shard` or
   `std/migration.shard`, or "cited by name"), the class of law §10.2.
   `std/order` (22 claims about `Int`'s order: `Int.le_refl`,
   `Int.lt_irrefl`, `Int.le_trans`, …, the `succ`/`pred` shifts by
   `arith`), `std/nat` (`add_nat`, `int_of_nat`, `half_nat` are
   `Nat.add`, `Int.ofNat`, `Nat.div n 2`), `std/div` (seven
   requirements over the literal divisor 10: `arith`'s quotient and
   remainder rows, slice 3.21 rule 4; `div_nonneg` is
   `Int.ediv_nonneg`) and `std/arith` (seven index identities: `arith`)
   have no V3 file. `std/list.shard` holds the realizations of rules 6
   and 7, `List.Pairwise`'s ghost refinement (the arbitrary-`Prop`
   `Subtype` phase 3 owes: a sorted list as `Subtype (List.Pairwise
   (· ≤ ·))` with an insertion keeping it) and what Init lacks;
   `std/bits.shard` holds the three bitwise recurrences and the
   width material Init lacks (`2^32`, `2^64` instances of
   `Nat.or_lt_two_pow`, `Nat.xor_lt_two_pow`,
   `Nat.and_two_pow_sub_one_eq_mod`), the old 2,135 lines gone: Init
   has `Nat.zero_and`, `Nat.and_zero`, `Nat.zero_or`, `Nat.zero_xor`,
   `Nat.xor_self`, `Nat.and_le_left`, `Nat.and_le_right`,
   `Nat.shiftLeft_eq`, `Nat.shiftRight_eq_div_pow` and the masks.
9. **The fifteen former axioms as theorems**, `v3/std/facts.shard`,
   the successor of `kernel/facts.shard` and theorem-only — the old
   statement on the new spelling, its proof the citation:

   | former axiom | the theorem's evidence | status |
   |---|---|---|
   | `mod_lo` (`0 < d → 0 ≤ n mod d`) | `Int.emod_nonneg` (its premise `d ≠ 0` by `Int.ne_of_gt`) | proved by the probe |
   | `mod_hi` | `Int.emod_lt_of_pos` | proved |
   | `ediv_mod_id` | `Int.emod_def` under `arith` | proved |
   | `div_unique`, `mod_unique` | `Int.ediv_emod_unique`, its two halves | proved |
   | `mul_comm`, `mul_assoc`, `mul_dist` | `Int.mul_comm`, `Int.mul_assoc`, `Int.mul_add` | proved |
   | `bshl_z`, `bshl_s`, `bshr_z`, `bshr_s` | `Nat.shiftLeft_zero`, `Nat.shiftLeft_succ`, `Nat.shiftRight_zero`, `Nat.shiftRight_succ_inside` — at `Nat`, the `0 ≤` premises gone (the typed class of law §10.2) | proved |
   | `band_rec`, `bor_rec`, `bxor_rec` (the low bit arithmetically, `a = 2·(a/2) + a%2`) | by `Nat.eq_of_testBit_eq` with `Nat.testBit_and`/`or`/`xor`, `Nat.testBit_succ`, `Nat.testBit_zero` and `Nat.mod_two_eq_zero_or_one`; or one unfolding of `Nat.bitwise` by its `WellFounded` equation (`Nat.bitwise_rec_lemma`), then `cases` on the two low bits and `arith` — the shorter route is `LEAN.md`'s example of a well-founded definition's equation | landing 3 |

10. **`docs/LEAN.md`** (law §5.4): the three lists — the same as Lean,
    refused with the pointer and the phase, shard-only — in dozens of
    rows, written from the probes' findings and the landings': Init's
    binders are the pin's (`(exact (NAME a b))` over-applied is
    `function_expected` with the type shown; `(apply NAME)` takes any
    shape); the horizon import by the refusal's pointer; `@` with the
    instance where rule 3's table has none; a `cases` arm `(zero ()
    STEP…)` names its fields in a list even when there are none; `/`
    and `%` Euclidean; `=` a proposition, `==` a `Bool`; numerals by
    position; `fn` = `def` + `realize`; `measure`; `dif`; `use` =
    `open`; `import Init NAME` = the horizon. The seam document's length
    is a symptom (law §5.4).
11. **T9 small** (law §5.4, §12.5), at landing 4: a fresh agent with
    `LANGUAGE.md` and `LEAN.md` and nothing else — no transcript, no
    kernel source — (i) proves a `sum_list_append`-class theorem (law
    §5.3's schematic) over `std/list`; (ii) writes a `fn` that lowers
    (route 1's `v3/build.sh` on its file); (iii) cites one Init theorem
    by guess from the naming grammar; (iv) states a ghost invariant
    (`Subtype` over `List.Pairwise`); (v) proves a branch under `dif`;
    (vi) writes a refused Lean form (`notation`, `partial`, `get!`) and
    receives the pointer. Scored by the refusals and interventions on
    the record, the count agreed before the run (law §12.5's rule);
    each finding a `LEAN.md` row. T9 runs again at phase 3's close.

Four landings. (1) **The seams**: rules 2–5 in the elaborator, the
unifier and the loader, each with a pin (`apply` of `Nat.zero_and`;
`List.sum` at `Int`; `(min a b)`; `(/ n d)` at `Int`; a name above the
horizon), under the loader pins, parity and the suite. (2) **The
records**: `std/facts.shard` (the twelve), `std/migration.shard` (one
theorem per row of law §10.3 — `Int.ediv_zero`, `Int.emod_zero`,
`Nat.sub` saturating, `Int.tdiv`/`Int.tmod` named, the `Decidable`
bridges, `Nat.land` on `Nat`), `std/README.md`'s six records,
`kernel/test/std_test.sh` as the 39th entrypoint — the std root loaded
in one process (the Init run shared, slice 3.25), every file stored and
accepted by K alone (slice 3.22's gate), `define_test`'s `List.sum`
expectation replaced. (3) **`std/list` and `std/bits`**: rules 6–9 —
the realizations, `List.get` under the dependent match, the matcher,
`List.sum` from Init under rule 3, the ghost refinement, the three
recurrences; calc's `examples/calc/list.shard` (its own `append` and
`len` "until `v3/std` exists") retires onto `std/list` as the library's
first consumer — calc's claims re-spelled `List.length`/`List.append`,
the `auto` sidecar regenerated by `prove.shard`, byte-identical
thereafter. (4) **`LEAN.md` and T9 small**, rules 10–11; the findings
folded in.

Weighed and deferred, each with the consumer that reopens it: **T2 and
T3** (law §4.3's lambda profile — closed lambdas as template arguments,
named partial application, a captured value as a runtime parameter;
`add_offset`; the static law-bearing package with no runtime
dictionary; the type-growing recursion refused loudly) are a language
slice, not a library one, and become **slice 3.28** with its design at
this arc's close; **T10** (an ordinary E library contributing an
I-producing tactic through law §7.3's API — `engine.shard` is that
library today, inside the kernel) becomes **slice 3.29**; the derived
view of Lean's compiled structural recursion (rule 6's lever; consumer:
the count of supplied bodies, twelve at landing 3, growing with the
bulk port); the loader native (item 60(f); the std entrypoint will be
the next multi-load cost); general instance search (rule 3's door); the
migration tool's tier 0 (law §12.3) — phase 5's opener, these records
its calibration data: the law's "calibrated on `std` in phase 3" is
read so, because the six modules mostly vanish under rule 1 and a tool
that re-spells them would mostly delete, stated here as a departure.
Rejected: a V3 declaration beside an Init one for an executable
spelling (`List.sum`, calc's `len` and `append`; law §4.4's "no second
mathematical declaration"); a V3 function spelling for `min`
(`Nat.min` is Init's own second spelling, ordinal 10,228, not the
instance's); porting `std/bits`' proofs (Init has the theorems); a
materialize-and-retry at the horizon (slice 3.26's one mechanism).

**Slice 3.27 as built, landing 1 (2026-10-10) — the seams.** Rules
2–5, in the elaborator, the unifier and the loader, each with a pin.
- **Rule 2 as built — the one spelling is a table, not a transparency.**
  Slice 3.21 rule 3's reading of a class method under Init's closed
  instance as its function (`ar_canon`, arith.shard) is the shared
  module `kernel/spelling.shard` (`sp_canon`), with the rows the
  library needs added: `HAnd`/`AndOp`, `HOr`/`OrOp`, `HXor`/`XorOp`,
  `HShiftLeft`/`ShiftLeft`, `HShiftRight`/`ShiftRight` at `Nat` to
  `Nat.land`, `lor`, `xor`, `shiftLeft`, `shiftRight`; `HAppend`/`Append`
  at `List α` to `List.append`. The unifier reads both sides through it
  at a structural mismatch under a metavariable, before K's whnf
  (`unify_fallback`): K's whnf of `0 &&& ?n` against `Nat.land 0 b`
  computes the closed side to `0` and the metavariable is never
  assigned — the probe's `witness_open`; read as `Nat.land 0 ?n` the two
  unify structurally. The rewriter needed nothing new: since slice 3.21
  every goal's statement (`tac_goal`) and every lemma's equation
  (`eq_of_lemma`) enter in the one spelling, so `rw` and `simp_only`
  across the two spellings follow from the rows alone. The mechanism
  the rule's text names — an instance constant and a class projection
  unfolding, Lean's `instances` transparency — would need the
  environment at every mismatch and would unfold Init's definitions
  past the instance (K's whnf has no smart unfolding: `Nat.add a ?b`
  becomes a stuck `Nat.rec`); the table is the same reading `arith`
  has trusted since 3.21, closed over the pin's names, and K rechecks
  every term it shapes. A class the table lacks is the door's wake
  condition, as rule 3 has it for instances.
- **Rule 3 as built.** `sp_instance`: the type of an instance-implicit
  metavariable, closed, to the pin's instance — at `Nat`: `instAddNat`,
  `instSubNat`, `instMulNat`, `Nat.instDiv`, `Nat.instMod`, `instLTNat`,
  `instLENat`, `instMinNat`, `Nat.instMax`, `instDecidableEqNat`,
  `Nat.instAndOp`/`instOrOp`/`instXorOp`/`instShiftLeft`/`instShiftRight`,
  `Zero` as `Zero.ofOfNat0 Nat (instOfNatNat 0)`; at `Int`:
  `Int.instAdd`, `instSub`, `instMul`, `instDiv`, `instMod`, `instNegInt`,
  `instLTInt`, `instLEInt`, `instMin`, `instMax`, `instDecidableEq`,
  `Zero` as `Zero.ofOfNat0 Int (instOfNat 0)`; at `Bool`:
  `instDecidableEqBool`; at `List α`: `List.instAppend`,
  `instDecidableEqList` over the element's; at any type `BEq` as
  `instBEqOfDecidableEq` over its `DecidableEq`; the heterogeneous
  `HAdd`…`HAppend` at one type as `instHAdd`…`instHAppendOfAppend` over
  the homogeneous row; `OfNat Nat n`/`OfNat Int n` as `instOfNatNat n`/
  `instOfNat n`. Asked (`el_solve_slots`) for an application's own
  binders once the expected type has had its say, for every
  metavariable when a declaration finishes (`el_finish`), and for a
  tactic opening's premises (`witness_close`); anything left is
  `instance_needed` with the pointer as before (`elab_instance`: `ite`'s
  `Decidable True` is no row — `if` has its own decisions, slice 3.16).
  `(min a b)`/`(max a b)` are operator rows to `Min.min`/`Max.max`, their
  instance from the table; `min` and `max` join the operator symbols,
  which take precedence over a declared constant of the same name as
  `+` does (no V3 file declares either). The table's constants are no
  token of the file that needs them: `sp_kit` lists every name the two
  tables write and joins the loader's demand (`init_kit`), loaded where
  the module's horizon admits it — 39 declarations on the probe file.
- **Rule 4 as built.** `op_at`: `/` at `Int` is `Int.ediv`; `%` was
  `Int.emod` already; at `Nat` the pair stays `Nat.div`/`Nat.mod`.
- **Rule 5 as built.** `init_resolve` keeps the names the name table
  places at or above the module's horizon (`Fx.above`, with their
  ordinals; the walk already looked them up to skip them), and a
  reading's `unknown_constant` on one of them is `above_horizon`:
  `NAME is declaration N of the export, at or above the module's
  horizon H (import Init X): write (import Init NAME)`. The name is
  matched as a token of the refusal's message (the elaborator's
  detail, or a tactic step's `step 1: NAME — the goal: …`), itself or
  under a prefix an opened namespace supplies (`zero_and` under `(use
  Init.Nat)` names `Nat.zero_and`). `init_horizon_beyond` now expects
  `above_horizon`.
- **Cleanup beside the landing:** `mk_app_n` moved from matcher.shard
  to expr.shard, where its users are; arith's `ar_cast`, `ar_not`,
  `ar_carrier` are the shared module's `sp_cast`, `sp_not`, `sp_carrier`.
- **Pins:** `op_int_div`, `one_spelling` (`apply`, `rw`, `simp_only`
  across the two spellings at the bitwise operators and `List.append`),
  `instance_table` (`List.sum` at `Int` and `Nat`, `min`/`max`,
  `List.length_take`), `init_above_horizon` (never loaded, cited bare),
  `init_above_horizon_by` (inside a by block); `unify_test` gains the
  one-spelling case in both orders. **Gates run:** the 230 loader pins 0 failed in 54 s, `unify_test` 0 failed, `tactic_test` 110 checks (calc's 100 claims) 0 failed in 44 s, `define_test` 12 checks 0 failed in 241 s, `engine_test` 24 checks 0 failed (54 of calc's 100 claims unaided, unchanged), `derive_test` 14 checks, `wire_test` 15 checks, `k_clients_test` 7 tests in 72 s, parity byte-identical over 31 closures and 133,762 declarations in 350 s, the full suite's 38 entrypoints with 0 failed in 553 s (under contention with the targeted gates; `store_test` 400 s, `parity_test` 333 s, `define_test` 308 s, `calc_test` 298 s, `path_test` 289 s). Route 1's kernel build, the full replay and the corpus are CI's.

**Slice 3.27 as built, landing 2 (2026-10-10) — the records.** Rules
1, 8 and 9's twelve, with the gate the design named. `v3/std/facts.shard`:
the twelve former axioms as theorems under their old names — `mod_lo`,
`mod_hi` by `Int.emod_nonneg` (its premise `d ≠ 0` by `Int.ne_of_gt`) and
`Int.emod_lt_of_pos`; `ediv_mod_id` by `arith` over `Int.emod_def`;
`div_unique` and `mod_unique` as the two halves of
`@Int.ediv_emod_unique n d r q`, the divisor's positivity and the
identity's orientation two `have`s by `arith`; `mul_comm`, `mul_assoc`,
`mul_dist` by Init's ring laws; `bshl_z`, `bshl_s`, `bshr_z`, `bshr_s` at
`Nat` by the shift lemmas, the `0 ≤` premises the type, the two with
implicit binders written `@` — twelve lines, the horizon
`Nat.shiftRight_zero` (1,440 declarations admitted, 38 s on the
bootstrap). `v3/std/migration.shard`: one theorem per row of law §10.3
(twenty, with a `def Nonneg` for row 11), each the row's V3 spelling and
the exceptional behavior its third column names — `Int.tdiv x 0 = 0`,
`Int.tmod x 0 = x`, `x / 0 = 0`, `x % 0 = x` by Init's theorems; the
decision bridges `decide (a ≤ b) = true ↔ a ≤ b` and `BEq.beq a b = true ↔
a = b` by `decide_eq_true_iff`; `Bool.and_eq_true`; `Nat.zero_sub`;
`Nat.and_le_right` as the unpremised bound; the rest `Eq.refl` — and, after
the rows, the records' evidence: `std/order`'s claims (twenty-one; the `if`
swap waits for a consumer), `std/nat`'s four, `std/div`'s seven and
`std/arith`'s seven as theorems, Init's name where one exists and `arith`
where the claim is an entailment, so each record is checked rather than
asserted (horizon `List.getD`, 1,706 declarations, 41 s). Two rows
corrected on the pin: row 8's `List.get?` is no constant of Lean v4.33.1 —
the index `l[i]?` is `GetElem?.getElem?` under `List`'s instance,
`List.get?Internal` by definition, `List.getD` the named default (law
§10.3's row amended in place, dated); row 14's `String` is a structure
over its UTF-8 `ByteArray` and the proof it is valid (`String.ofByteArray
b h`), the view's round trip `String.toList_ofList` cited in the record
and not loaded: its closure is 3,282 declarations of the export, 277 s,
against the file's 1,700 without it. `v3/std/README.md`: the six
per-interface records (law §10.1) — every declaration of the old
`mod.req.shard`, its V3 spelling or Init's name, the evidence, the class —
`order` (22), `nat` (7), `div` (7, and the two it cited from the old
`kernel/facts.shard`), `arith` (7) with no file; `list` (29)
and `bits` (38) with Init's names each checked against the export's
index, their files at landing 3. `kernel/test/std_test.sh`, the 39th
entrypoint: the std root's six files loaded in one process (Init on
demand, shared), 211 declarations stored and replayed by K alone, the
records' theorems held by name under no axiom but Lean's own (`propext`,
`Quot.sound`, `Classical.choice` — what the cited Init theorems carry):
76 checks. Slice 3.13's `v3/std/list.shard` with V3's own `List.sum` is
deleted (rule 1) and `define_test`'s expectation on it removed; the
library's files are `std_test`'s. **Found and fixed on the way, each with
a pin.** (i) The instance table's dispatch (landing 1) tried the type's
own rows and stopped: `BEq Int` reached the `Int` rows, which have no
`BEq`, and never the any-type row — `sp_inst_typed` falls through to
`sp_inst_any`; the `instance_table` pin gains `BEq.beq` at `Int` and
`Nat`. (ii) `arith`'s equation path (slice 3.21): `lin_same` read a
missing trailing coefficient as 0 while the two evaluation terms it then
equated by `Eq.trans` carry their exact lists, and K's conversion of
`[1]` against `[1, 0]` is stuck at `0 * x` for a symbolic `x` — the
elaborator built an ill-typed term and K refused it
(`app_type_mismatch` on `n = 10 * (n / 10) + n % 10` and on
`((b + 1) + y - 1) - y = b`, a bare variable against a sum whose atoms were
collected after it), the boundary doing what law §3 says it does.
`lin_eq` compares entry for entry; the differing case takes the
two-inequality path, which was sound (pin `arith_eq_bare`, six shapes).
**Found and recorded for `LEAN.md` (landing 4), not changed:** a bare
`true`, `some`, `cons` resolves in a pattern only — in a term the
constructor is written under its namespace (`Bool.true`, `Option.some`,
`List.cons`), item 7's ruling that `use` opens no constructors meeting
law §5.3's "lowercase constructors"; `==`, `&&`, `||` are not operator
symbols of the elaborator — `and`, `or`, `not` at `Bool` operands are
`Bool.and`, `Bool.or`, `Bool.not`. **Gates run:** the 231 loader pins 0 failed in 56 s, `std_test` 76 checks 0 failed in 117 s (the root loaded and stored in 69 s, K's replay 48 s; 158 s inside the suite), `tactic_test` 110 checks (calc's 100 claims) 0 failed in 43 s, `engine_test` 24 checks 0 failed (54 of calc's 100 claims unaided, unchanged), `define_test` 12 checks 0 failed, `derive_test` 14 checks, `unify_test` 0 failed, `k_clients_test` 7 tests in 70 s, parity byte-identical over 31 closures and 133,785 declarations in 292 s, the full suite's 39 entrypoints with 0 failed in 640 s (`store_test` 505 s, `parity_test` 364 s, `path_test` 316 s, `calc_test` 300 s, `define_test` 297 s). Route 1's kernel build, the full replay and the corpus are CI's.

**Slice 3.27 as built, landing 3 (2026-10-10) — `std/list` and `std/bits`.**
Rules 6–9 built, calc retired onto the library, five pins. **Rule 6, the
realizations** (`v3/std/list.shard`): `List.length`, `List.append`,
`List.reverseAux`, `List.reverse`, `List.take`, `List.drop`,
`List.get?Internal` and `List.getD` by supplied bodies, their equations
theorems K proves — by `rfl` where the body follows Init's recursion, by
`cases` on the count where Init recurses on it first (`take n [] = []`
is `take_nil_eq`, a term-level match on `n` with `Eq.refl` at both
constructors), by cases on the index for `get?Internal` (slice 3.24's
path); `List.getD` is one leaf, `Option.getD (List.get?Internal xs i) d`,
K unfolding `as[i]?` through `GetElem?`'s instance (law §10.3 row 8).
**The matcher of an Init definition realized through its own
definition**, as the rule said it would be measured: `(realize
Option.getD (view))` is refused before this landing — `Option.getD.match_1:
no realization attached` — and accepted after, the derived view
(`erasure.shard`) unfolding a definition named `NAME.match_N` that has no
descriptor of this run at its arguments (`beta_head`) and erasing the
`casesOn` it is, an application whose head is a lambda now beta-reduced
on the way; the view attaches no equations, the E body being the value's
erasure. **Rule 7, the dependent match** (`elab.shard`): a `match` on a
local generalizes, beside the expected type (slice 3.19 rule 9), every
local in scope whose type mentions the scrutinee or one of these,
outermost first — Lean's revert: the motive is the expected type under a
Pi over them, each arm reads its body with them re-bound at the pattern's
type under the names the context binds them by (a realize's telescope
local carries Lean's binder name, `a._@._internal._hyg.0`; the body knows
it as `i`), and the matcher's result is applied back to them. The
dependent reading is tried first, then the expected type's, then the
constant motive; the first reading's error is the one reported. Under
`match xs` with `i : Fin (List.length xs)` the nil row sees `i : Fin
(List.length List.nil)` and refutes it: **the refuted row is Init's
`absurd`** (or `False.elim`) applied to a proof and its negation, an L
term of the row's type — `(absurd (Fin.isLt i) (Nat.not_lt_zero (Fin.val
i)))` — erased to `EAbsurd`, a node of `ETerm` that `ev` lowers to
`LAbsurd` and is stuck `unreachable` if a program K did not warrant ever
reaches it; the E-first route refuses the form by name
(`absurd_needs_init`), as it refuses `dif`. What followed from the
generalization: a matcher applied past its alternatives (the generalized
locals) is read by the erasure with each alternative instantiated at
them under its fields (`alt_with_extra`) and by the realizer's leaf
splitting, which took one extra argument (the measured function's
`Eq.refl`) and takes any number now; and a leaf's binders are ordered so
that a kept binder re-typed by the split — `i : Fin (List.length (x ::
t))` — follows the fields it names (`leaf_binders`: the binders the
split left, the fields, the re-typed ones), where the as-built of slice
3.16 put every kept binder before the fields and the equation of a
dependent `fn` was `fvar_in_type`. `List.get` is realized so, its two
equations `get_nil_eq` (proved by the refutation itself) and
`get_cons_eq` (a term-level match on the `Fin`, then on its value under
its bound — the dependent match again —, `Eq.refl` at both), its cons
row `dif` on `Fin.val i = 0` with the bound carried into the tail by
`Nat.sub_lt_right_of_lt_add`. **A realize's binders are E types**: the
`Fin` index is written `Fin`, the registry's row (`(i Fin)`), since an E
type names no term; the signature check said so (`realize_signature:
… another type than the L type's erasure`). **Rule 8, what Init lacks**:
`sum_append` at `Nat` by induction (Init's `List.sum_append` asks for
`Std.LawfulLeftIdentity`, which the table does not hold), and the ghost
refinement phase 3 owes — `Sorted` is `(@Subtype (List Nat) (List.Pairwise
Nat.le))`, `sorted_insert` keeps the proof by `insert_sorted`, proved by
induction on the list with the hypothesis stated as an implication
(`cases` and `induction` generalize the target only) over `mem_insert`,
`List.pairwise_cons` and `List.mem_cons` as iffs (both inductives are
indexed families: `cases` on a hypothesis of either is a later form),
the `if` split by `cases` on `Nat.lt_or_ge` and `if_pos`/`if_neg`. For
that, **`Subtype` is a row of the registry** (law §10.3 row 11; `erase.shard`
`rep_row`): one constructor, one runtime field, `Subtype.mk v h` runs as
`v`, `Subtype.val s` as `s`; a row constructor's parameters are opened
unclassified before its fields (`tele_of_fields`) — `Subtype`'s
predicate `p : α → Prop` is neither a type, a proposition nor data, and
the field walk refused the row on it. The carrier is named by a `def`
(`(def Sorted () Type …)`): a `fn` signature reads `(@Subtype (List Nat)
P)` at `Sort 0` and `(Subtype P)` with its carrier unsolved — a finding,
the def the idiom until it is fixed. **The old `std/list`'s 24
requirements** are `list_*` theorems citing Init by name (`apply` for the
implicit-binder ones, `exact` with the arguments for the explicit), so the
record is checked; `length_nil`, `length_cons` and `reverse_nil` by
`Eq.refl` rather than past-horizon citations. **Rule 9, the three
recurrences** (`std/facts.shard`, horizon `Nat.and_mod_two_eq_one`):
`band_rec`, `bor_rec`, `bxor_rec` at `Nat`, each the Euclidean split of
the operation by 2 — Init's `and_div_two`, `or_div_two`, `xor_div_two` for
the quotient; `land_low`, `lor_low`, `lxor_low` for the remainder, four
cases of `a % 2` and `b % 2` on `Nat.mod_two_eq_zero_or_one` with the
operation's low bit decided by Init's `and_mod_two_eq_one`,
`or_mod_two_eq_one`, `xor_mod_two_eq_one` iffs (the xor case needs
`¬(p = 1 ↔ q = 1)`: `intro` on the negation, `Iff.refl` after rewriting
both sides, `absurd`) — joined by `arith` over `Nat.div_add_mod`. Stated
subtraction-free, the product moved to the left (`lor a b + p·q = 2·lor' +
p + q`), since `arith` reads `Nat.sub` as an atom. **`std/bits.shard`**
(horizon `Nat.le_sub_one_iff_lt`): the old module's 37 requirements as
theorems under the old names citing Init at `Nat`, `pow2 k` as `(Nat.pow
2 k)` — **one spelling with Init's `2 ^ k`** by a row of the table for
`HPow`/`Pow` at `Nat` (`spelling.shard`; `Nat.pow` in the kit), without
which `Nat.or_lt_two_pow ha hb` could not read `a < Nat.pow 2 32` —, the
width instances at 32 and 64 bits through a `Nat.pow` hypothesis (a
literal does not determine the exponent: `2 ^ ?n` against `4294967296`
is refused by the unifier, `exact` between `Nat.pow 2 32` and the literal
is K's conversion), the masks at 8, 32 and 64, the literal shifts (one
step by `Eq.refl`, a count by `Nat.shiftLeft_eq` with the power rewritten
to its literal and `arith`). **Calc retired onto the library**: its
`examples/calc/list.shard` deleted, seven files `(use std.list)` and
`(import "../../std/list.shard")`, `append`/`len` re-spelled
`List.append`/`List.length`, its two append laws Init's `List.append_nil`
and `List.append_assoc`, and one `rfl`-proved equation re-oriented —
`List.length (x :: t)` is `length t + 1` where calc's `len` was `1 + len
t`, and the two are not convertible. Two kernel facilities for that:
**the reducer reads a realization's equations** (`tactic.shard`
`equations_of`: a fn's `f.eq_k`, else `M.f.realize_k` for the module M
that attached the realization — this module, or one it opens), so
`(reduce)` over `List.length (cons h t)` steps by `realize_2` as it
stepped by calc's own `len.eq_2`, where K's whnf had unfolded Init's
structural recursion into its `brecOn` encoding (`proj 0 (PProd.mk …)`
atoms to `arith`, `rw_no_match` on the appended list); and **the loader
marks a use needed when a token of the file hits an identity the module
realized** (`mark_hits`: the entry's module beside the prefix above the
token), so calc's `(use std.list)`, under which it runs Init's
`List.append`, is not recorded UNUSED for the prune. **The gate**:
`std_test.sh` holds the three new files' theorems and the realizations'
equations by name (181 checks), `Option.getD`'s view by
its record. **Pins**: `dependent_match` (the fn, the term-level match
under a bound, `List.get` realized), `matcher_view`, `refined_row`
(`Pos` = `Subtype Nat (Nat.le 1)`, `bump` running as its value),
`absurd_e_first` (the pointer). **Findings for `LEAN.md` at landing 4**:
`arith` reads `Nat.sub` and `2 ^ k` as atoms (state subtraction-free;
rewrite a literal power first); an exponent is not inferred from a
literal; `@NAME` with an unknown implicit order is a guess — prefer
unification from a hypothesis; `cases`/`induction` generalize the target
only — state the hypothesis as an implication; `cases` on an indexed
family's hypothesis is refused — use Init's iff; a `fn` signature cannot
name `@Subtype T P` — name the carrier by a `def`; `--run` takes a World
entry; `List.length (x :: t)` is `length t + 1`; a `def` has no E body —
a value a program calls is a `fn`. Measured: the std root's three new
files load in 82 s alone (facts' demand 1,705 declarations at horizon
56,062; bits' 256; list's 68). Gates run: the 235 loader pins 0 failed in 56 s, `tactic_test` 110 checks (calc's 100 claims) 0 failed in 53 s, `calc_test` byte-identical over 21 inputs in 249 s, `calc_main_test` byte-identical to the model world in 112 s, `engine_test` 24 checks 0 failed (54 of calc's 100 claims unaided, unchanged) in 193 s, `define_test` 12 checks 0 failed in 246 s, `derive_test` 14 checks 0 failed, `unify_test` 0 failed, `k_clients_test` 7 tests in 78 s, `std_test` 181 checks 0 failed in 167 s (the root's 8 files and 336 declarations loaded and stored in 97 s, K's replay 70 s), `path_test` 18 checks 0 failed in 251 s, parity byte-identical over 31 closures and 133,990 declarations in 278 s, the full suite's 39 entrypoints with 0 failed in 559 s (`store_test` 424 s, `parity_test` 301 s, `define_test` 278 s, `calc_test` 265 s, `path_test` 257 s). Route 1's kernel build, the full replay and the corpus are CI's.

**Slice 3.27 as built, landing 4 (2026-10-10) — `LEAN.md` and T9 small.** Rules 10 and 11, with what the gate found.

- **Rule 10, `docs/LEAN.md`.** The three lists of law §5.4 as tables, 92 rows: the same as Lean (the names and the naming grammar cited by guess, `use` as `open`, the binders and `@`, the connectives and `Bool`'s, `if`/`dif`/`match`, numerals and the one coercion, the operators by type with `/` and `%` Euclidean, the one spelling of `2 ^ k` and the bitwise operators, the list accesses by Init's names, the instance table, structures and `Subtype`/`Fin`, `def`/`theorem`/`fn`, the equation names, the policy, the steps of `by` against Lean's tactics, `cases` on a term, the anonymous constructor, the list literal, the conversion idiom for a `fn`'s step); refused with the reason, the ruling and the pointer (the Lean keywords, `instance`/`class` and the reserved forms, a file import, `NAME!`, infix, `==`/`&&` and Lean's Unicode spellings, `by` in a term, a `Fin` numeral, `omega`'s reach against `arith`'s, `simp`/`ring`/`aesop` against replay without search, `cases` on an indexed family, `induction … generalizing`, a guessed implicit order, a theorem over-applied, an exponent against a literal, function values, a `def` called from a `fn`, `@Subtype` in a `fn` signature, `absurd` and `dif` on the E-first route, `nat_operator`, `Array`/`IO`/`do`, the horizon, `#eval`); shard-only (`fn`, `realize`, the output idiom, externs and the World entry, the imports and the horizon, views, `trusts`, the data sugar, `derive`, `dif`, `by` as I, `auto`, the store, the spelling tables, the registry's rows, the refusal vocabulary). The header gives the loader's command line, the records and which refusals end a file. Every row stated from a run: the rows written from recall were probed before the gate and five corrected (`bad_import` for a Lean file import; the exponent finding's shape is `type_mismatch` showing `2 ^ ?n` against the literal; `by` in a term is no term; `#eval`'s message; the Unicode symbols reach the elaborator as `unknown_constant`).
- **The refusals the lists name carry their pointer** (four kernel edits, eight pins). A Lean keyword with no form here — `partial`, `unsafe`, the notation and macro layer, `open`, `lemma`/`example`, `mutual`, `termination_by`/`decreasing_by`, `noncomputable`/`private`/`protected`, `universe`, `deriving`, `implemented_by`, `attribute`/`set_option` — is `lean_form` with the V3 spelling or the law's reason (`reader.shard`'s `lean_form` table; `unknown_form` stays for a head that is neither Lean's nor V3's; `instance`/`class` point to the instance table). A name ending in `!` is `inhabited_default` with the Option form and the supplied-default form, on both routes and before the horizon's refusal (`loader.shard`'s `lean_pointer`: `List.get!` above the horizon must not be answered "import it"; the Option form is named where the export declares it — `List.get?` is no constant of this pin, T9's finding 3). `==`, `&&`, `||`, a `!`-prefixed name, `by` inside a term and Lean's `∀ ∃ λ → ↔ ¬ ∧ ∨ ≤ ≥ ≠ ℕ ℤ ×` are `lean_symbol` with the spelling (`lean_symbol_pointer`; they lex as symbols and reach the elaborator as `unknown_constant`). An operator symbol where an argument is expected — `(n + 1)` — is `infix_form` with the prefix spelling (`elab.shard`'s `tele`, where it was `function_expected: applied to 2 more arguments than its type has: Nat`). Before this landing `partial` and `notation` were a bare `unknown_form: partial`, `List.get!` the E-first route's `unknown_head`, `==` likewise — law §12.5's T9 row asks for the `fn`/`def` pointer on a refused `notation`.
- **Rule 11, T9 small** (`docs/records/T9-small-2026-10-10.md`, the report verbatim with the count agreed before the run). DEPARTURE from the design's (ii): "a `fn` that lowers" is measured on route 2 — route 1's `v3/build.sh` compiles the kernel's bring-up dialect and refuses a V3 user file at its first form (`malformed declaration (realize List.length ...)`, measured on `examples/path/path.shard`); a user `fn` lowers when the typed route defines it (`DEFINE` with its equations) and `ev` runs it under `--run`. The count: 0 interventions; at most 2 refusals before success on (i)–(v); (vi) succeeds when its three forms receive their pointers; pass = all six accepted with no workaround from a forbidden source. The agent: fresh, with `LANGUAGE.md`, `LEAN.md`, the library it imports (`v3/std/`) and the loader's command line; forbidden `v3/kernel/`, `v3/pins/`, `v3/examples/`, the other docs, the records, memory, the transcript. **Result: pass** — 0 interventions, 1 refusal before success (task ii), 7 loader runs, 11 minutes; `sum_list_append` at `Int` by induction with the conversion idiom and `arith`, four Init theorems cited by guess and every one resolved (`Int.zero_add`, `Int.add_assoc`, `Nat.lt_succ_self`, `List.length_map` by `apply`), `SortedInt` as `@Subtype (List Int) (List.Pairwise Int.le)` named by a `def` with its empty value and two theorems, `safe_index` under `dif` with both branches proved by `dif_pos`/`dif_neg`, a `main` entry printing the count below its argument, the three refused forms with their pointers; verified by a second load of the four files (196 accepted, 0 refused) and the entry's output. **Ten gaps reported, each disposed of**: seven are `LEAN.md` rows or header sentences (which refusals end a file; the list literal; the inherited horizon and `INIT -:`; the output idiom through `Nat.renderOnto` and `Bytes.ofNats`; a successful `--run` prints no records; an arm's fields then its hypotheses; the conversion idiom for a `fn`'s step); two are kernel fixes — a zero-parameter `fn` written bare in argument position is its call on the E-first route as on the typed route (`classify.shard`'s `read_esym`; before, `function_value` where the typed route read the constant as the value it is — the one refusal the author met; pin `zero_arity_bare`; the `function_value` message names the arity) and the `inhabited_default` pointer's name; one is the author's misreading of two ordinals (noted in the header: the loader's horizon is the ordinal past the name). T9 runs again at phase 3's close, with the then-current `LEAN.md`. Gates run: the 243 loader pins 0 failed in 53 s, `unify_test` 0 failed, `tactic_test` 110 checks (calc's 100 claims) 0 failed in 51 s, parity byte-identical over 31 closures and 134,028 declarations in 293 s, the full suite's 39 entrypoints with 0 failed in 580 s (`store_test` 436 s, `parity_test` 329 s, `define_test` 298 s, `path_test` 283 s, `calc_test` 283 s). Route 1's kernel build, the full replay and the corpus are CI's..

#### Slice 3.28

**Slice 3.28 — the lambda profile: templates, lifted lambdas and
static packages (T2, T3)** (designed 2026-10-10 at the boundary after
slice 3.27, on the user's "continue to the next slice"; §13 item 62 —
**ratified 2026-10-11 as designed, the four leans taken: instances
named by their flattened shapes, the budget 1,024, six list templates,
packages as structures with no new form**). Law §4.3 names the profile E
supports in v1, in order: closed lambdas as template arguments; named
partial application to known E values; non-escaping value capture by
lambda lifting, the captured value an ordinary runtime parameter and
never a specialization key; templates — a `fn` with function-typed
parameters, specialized per distinct static argument tuple, its
theorem proven once in L over arbitrary `f`, each instance
definitionally the specialized body or bridged by a generated equation
where the arity changed; the escape rule; enforcement by the classifier
at declaration and by the lowering on the post-specialization closure;
finite specialization, a budget a refusal; the dynamic tier a named
door. Law §4.1 adds the static law-bearing package — operations plus
laws, selected statically, its operation projections specialized and
its laws erased, no runtime dictionary. The gates are T2 (a closed
lambda, a named partial application, the captured-value `add_offset`
of law §5.3's example, a static law-bearing package with no runtime
dictionary, the generic theorem reused; the risk column: a source
restriction mistaken for an artifact restriction) and T3 (the
type-growing recursion refuses loudly; a large finite workload within
limits; the risk column: totality mistaken for compiler termination).

**Measured before this design** (four probe files under the on-demand
loader on the bootstrap, 2026-10-10, 16 s a run): (i) the law's
`add_offset` is refused before its lambda is reached —
`no_realization: List.map: no realization attached — realize it first`
on the typed route, `unknown_head` E-first — and `List.map` cannot be
realized: its function binder is `realize_signature: f: a binder that
is neither a type parameter, a proposition nor an E type`; (ii) a `fn`
with a binder `(f (-> Int Int))` is `no_l_identity: a binder whose type
is not an E type` on the typed route and `unknown_type: ->` E-first;
(iii) a `def` with the same binder, a `def` whose value is a lambda and
a theorem over them are accepted — L has function values, only E lacks
them, which is the law's split; (iv) the type-growing recursion (`grow`
over `(List a)` calling itself over `(List (List a))`) is refused
today, `realize_descent: a recursive call at other type arguments
(polymorphic recursion)` — `define.shard`'s self-call check since slice
3.16 — so T3's first half holds and this slice pins it; (v) the old
tree's `std/list` has no `map`, `filter` or `fold` and no file of the
old tree writes a lambda: v2 had no function values either, so §12
changes no row (this slice adds and drops nothing).

**The rules.**

1. **The static role.** A binder of a `fn` or of a supplied `realize`
   whose type is a function type — a Pi whose domain is runtime data,
   after the erased-Pi rule has dropped the proposition domains — is a
   **static parameter**, a fourth role beside the type parameter, the
   erased evidence and the runtime datum (`erase.shard`'s `BRole` gains
   it); so is a binder of a package type (rule 5). A declaration with a
   static parameter is a **template**. Its L half is what it is today:
   the definition with its equations, elaborated once (`List.map f xs`
   is an ordinary L term, and a theorem over it is proven once, over
   arbitrary `f`); its E half is no program of its own but one per
   instance (rule 3). A template is not `RUNNABLE` and has no `REALIZE`
   record; its record is `TEMPLATE NAME static=f,…`. No keyword: the law
   says "a `fn` with function-typed parameters", and the binder's type
   is the whole signal.
2. **The escape rule, at the positions the erasure already refuses.**
   In a template's body a static parameter `f` occurs as a call head
   applied to its full arity, `(f x …)`, or as a static argument of a
   template call (rule 3(d)); a package parameter's projections stand
   as call heads or as the closed values rule 5 names. Anywhere else —
   an argument of a non-template, a constructor field, a `let` value, a
   `match` scrutinee, a return position, an application to fewer
   arguments than its arity — the refusal is `function_value` with law
   §4.3's pointer, as `erasure.shard` says today for a lambda in a
   runtime position and for a variable applied to a runtime value; the
   E-first route's `unknown_type: ->` becomes the same refusal with the
   pointer.
3. **Instances, by shape.** A call of a template from an E body — a
   `fn`'s, a supplied `realize`'s, an instance's — supplies for each
   static parameter a **static argument**, one of: (a) the name of a
   `fn`, a realized constant or an instance, of the parameter's arity;
   (b) a **named partial application** `(g a …)` of such a name to
   runtime terms of the caller, the remaining arity the parameter's;
   (c) a **closed lambda** `(fun ((x T) …) BODY)` — closed up to the
   caller's runtime parameters and locals, which it captures: it is
   **lifted** to a generated `fn` under the caller's name,
   `CALLER.lam_N` (N in order of occurrence in the body), the captured
   locals its leading parameters in order of first occurrence and the
   lambda's binders after them, its body the lambda's; the argument is
   then case (b), `(CALLER.lam_N captured …)`, or case (a) when nothing
   is captured; (d) a static parameter of the enclosing template,
   passed on unchanged — the call is instantiated when the enclosing
   template is (instantiation is transitive and happens at the
   outermost E body); (e) a package constant (rule 5). The **shape** of
   a static argument is the identity it names and its captured count;
   the captured values are runtime data and never part of the key (law
   §4.3). An **instance** is the template at a tuple of shapes, one E
   program per distinct tuple per module — the taking module owns it,
   as a module owns the derivations it takes (slice 3.19) — named by
   the derive precedent's flattening, `MODULE.TEMPLATE_flat.of.SHAPE…`
   (`main.List_map.of.main_add_offset_lam_1_1`: the lifted lambda's
   identity, then its captured count), a second taking of the same
   tuple reusing it. The instance's E program: its parameters are the
   captured values of each static argument, group by group in parameter
   order, then the template's runtime parameters; its body the
   template's L body erased with each static parameter bound to an
   entry carrying its shape — slice 3.16 rule 6's callee-local entry
   (`EKCall id flags`) extended by the captured slots — so `(f x)`
   erases to `(ECall g (c … x))`, and the template's recursion at its
   own static parameters is a call of the same instance. Its L meaning:
   a generated definition of the same name, `… := fun c … r … =>
   List.map (g c …) r …`, admitted to K — the instance is
   **definitionally** the specialized body (delta and beta), which is
   how a caller's L body, written with the lambda, and its E program,
   calling the instance, correspond; its equations `….eq_N` are the
   template's equations instantiated at the static arguments, each a
   theorem K checks (the template's `eq_N` applied, under the unfolding
   — the generated equation the law names where the arity changed),
   registered as a realization's equations are, so the reducer and the
   rewriter read them (slice 3.27's lever) and a theorem proven over
   arbitrary `f` applies to the instance's uses by unification through
   the unfolding — T2's "generic theorem reused". Record: `INSTANCE
   NAME of=TEMPLATE shapes=g/1,… equations=…`. On the E-first route (a
   body with no L reading, `main` over a World) the instance is taken
   with no generated definition, the route having no L half.
4. **Finite specialization (T3).** The shapes a load can write are
   finite: a static argument is a name, a partial application of a
   name, a lifted lambda or a parameter passed on, never a term built
   from a static parameter — `(f a)` or `(fun ((y U)) (f y))` as a
   static argument inside a template is refused (`function_value`: a
   static parameter is passed on, not transformed), which closes the
   shape set. The type-growing recursion stays `realize_descent` (probe
   (iv)), and a template calling itself at another shape is refused by
   the same check extended from type arguments to static arguments (`a
   recursive call at other static arguments`). Beside the closure, law
   §4.3's budget: `instance_budget`, 1,024 instances of one template
   per load, past which `specialization_budget` refuses, naming the
   template — a refusal, never a proof of anything. A workload example
   takes many instances in one file and runs under `ev` (rule 7).
5. **Static law-bearing packages.** A `structure` with a
   function-typed field — `(structure Monoid ((a Type)) (op (-> a a
   a)) (unit a) (op_assoc (forall ((x y z a)) (= (op (op x y) z) (op x
   (op y z))))) (unit_left (forall ((x a)) (= (op unit x) x))))` — the
   fields directly after the binders, as §4 (a spelling corrected at
   slice 3.30 landing 2) — is
   a **package type**: never an E type (`ind_eligible` says no, as
   today), so no runtime value of it exists — the law's "no runtime
   dictionary". A **package constant** is a `def` of a package type
   whose value K's whnf reads as the constructor applied: its function
   fields static arguments in rule 3's senses (a name, a partial
   application, a lambda lifted under the constant's name), its
   proposition fields erased, its value fields closed E terms. A binder
   of a package type is a static parameter (rule 1); its shape is the
   constant's identity. In a template `(Monoid.op m x y)` erases at the
   instance as the field's shape applied — a call head —, `(Monoid.unit
   m)` as the closed value's E term, and a law field is never in a
   runtime position. The generic theorem over `m` is proven once and
   applies at the constant by unification (T2's package half). Outside
   a template a package constant's projection erases the same way
   (`(Monoid.op IntAdd 1 2)` is `(Int.add 1 2)`) — by the reduction,
   not by a table.
6. **Init's higher-order list functions as templates in `std/list`.**
   Slice 3.27's rule 6 extended: `List.map`, `List.filter`,
   `List.foldl`, `List.foldr`, `List.any`, `List.all` realized by
   supplied bodies whose function binder is a static parameter, their
   equations per case-tree leaf proven by K as today's are, held by
   `std_test.sh`; their definitions are Init's (ordinals 98 to 2,907).
   Law §5.3's `add_offset` is then written as the law writes it.
7. **The gates: T2 and T3 as a test, and the pins.**
   `kernel/test/template_test.sh`, the 40th entrypoint, over
   `v3/examples/template/`: `add_offset` (a lifted lambda with a
   captured value), a named `fn` and a named partial application as
   arguments, a package at `Int` with a generic theorem proven once over
   `m` and applied at the constant, a workload file taking 64 instances;
   each loaded, the entries run under `--run` with their outputs
   compared, and the programs dumped (`--dump`) for the **artifact**
   check — no program line holds a lambda, a partial application or a
   function-typed parameter while the source files hold the lambdas —
   T2's risk column checked on the artifact, not read off the source.
   Pins: `template_lift`, `template_named`, `template_partial`,
   `template_passed`, `template_escape` (a lambda in a constructor
   field), `template_transform` (`(f a)` as a static argument),
   `template_package`, `template_growth` (`f[A]` recursing at `f[List
   A]`, T3's canonical refusal, by name), `template_budget`.
   `docs/LEAN.md`'s function-value row becomes rows of what is now the
   same as Lean (`List.map (fun x => …) xs`, `f ∘ g` as a lambda, a
   package as a structure) and of what stays refused (a lambda in data,
   currying, a function-valued field at runtime, a static argument
   built from a parameter); the kernel README's rows; records §9. Gates
   run per landing: the pins, `std_test`, `define_test`, parity, the
   full suite; CI's pipeline per landing.
8. **Route 1 unchanged.** The law's authority on the post-specialization
   closure is the lowering; route 1 compiles the kernel's bring-up
   dialect only (the departure recorded at slice 3.27 landing 4), so
   this slice's artifact check is on the E programs `ev` runs and
   `--dump` prints — first-order by construction, the programs route 1
   will consume when it compiles user files (phase 4). Stated as the
   slice's limit, not solved here.

**Landings.** (1) Rules 1–3: the static role, the escape rule with its
pointers on both routes, instances by shape with lifted lambdas and
named partial applications, the generated definitions and the
instantiated equations, the records; pins `template_lift`,
`template_named`, `template_partial`, `template_passed`,
`template_escape`, `template_transform`. (2) Rules 5–6 and the example:
packages, the six list templates in `std/list`, `v3/examples/template/`
with `add_offset` and the package, `template_test.sh`; pin
`template_package`. (3) Rule 4's budget and the workload,
`template_growth` and `template_budget`, `LEAN.md`'s rows, the §12.5
gate records for T2 and T3, the close-out.

**As built, landing 1 (2026-10-11): the static role, the escape rule,
instances by shape.** Rules 1–3 on the typed route. `erase.shard`'s
`BRole` gains `BStatic k` (`fn_arity`: a Pi chain of E-typed domains
over an E result, `k` its length; a proposition or a Sort domain is no
static parameter); `erase_predef` answers `PdEraseTemplate` for a `fn`
with one — no program — and the loader (`load_template`) checks the
template's body once at its declaration (`template.shard`'s
`check_template`: the body erased with each static parameter at a
placeholder shape, so a static parameter in a runtime position, applied
as a static argument or under a lambda, and a self-call at other shapes
are refused where the law has the classifier enforce the escape rule),
defines it in K as any `fn` (the definition, its equations), registers
it as `EITemplate` and keeps its pre-definition with its roles
(`Load.templates`); the record `TEMPLATE NAME static=f,…
equations=…`. A template's call in a body (`erasure.shard`'s
`erase_template_call`, by `is_template` at the constant, by
`EKTemplate` at a template's own self-local) erases to the instance's
call at the captured values group by group then the data: a type
argument must be closed (`l_to_etype` with no type local — a
polymorphic taker is this landing's limit, refused `function_value`),
a static argument's shape is read by `static_shape` — (a) a name with
a program (`shape_kind`: an `EFn` of the table, or a constant of the
realization registry, whose instance calls the primitive — a decision
through `erase_dec_value`), (b) `(g a …)` with `g`'s type and erased
arguments kept closed and its runtime arguments captured with their
closed types (`shape_args`; the trailing binders exactly the parameter's
arity), (c) a lambda lifted (`lift_lambda`: the caller's runtime locals
it mentions in order of first occurrence, then its binders opened at
the arity — eta past the lambdas written —; the program `EFn` over
their closed E types, the L definition the same lambda over the
captured locals, `DefnDecl` under `CALLER.lam_N`; a lambda over a type
parameter, a proof, a function with no K constant, the enclosing
function, a static parameter or the template it stands in is refused
by name), (d) a static parameter passed on (`passed_shape`, by its
`EKShape` entry at the enclosing instance). The instance is demanded of
the loader (`Tr.demands`, deduplicated by name) with the type
arguments and the shapes; its name `MODULE.T_flat[_TYPE…].of.SHAPE…`
— the template flattened **with its type arguments** (derive's
precedent; a template without type parameters as the design wrote it:
`main.main_twice.of.main_add_off_lam_1_1`), then each shape as its
identity flattened and its captured count. `template.shard` builds it
(`build_instance`): the program — the template's body at the type
arguments (`subst_types`), erased over entries where each static
binder is `EKShape shape slots` and the slots are `EKVar` entries
under ids below zero (the instance's leading parameters), the
self-local `EKTemplate` with the template's own binders carried by
`Tr` (`tr_own`: its constant is not in the pre-definition's walk), a
structural measure shifted past the captured parameters —; the L
definition `λ captured… binders…, T types… (g leading… captured…)…
binders…` with the template's data and erased binders re-opened at
the substitution (`rebuild_binders`), admitted as a `DefnDecl`; the
equations `NAME.eq_N` from `T.eq_N` — its binders opened, the type and
static positions' locals replaced by the type arguments and the
shapes' terms, the rest re-opened at the substitution
(`reopen_locals`), the statement over the instance applied, the proof
`T.eq_N` at the same arguments, which K accepts through the instance's
unfolding. The loader (`build_demands`) admits the lifted lambdas'
definitions first (`admit_lifted`: once — a body read again after an
auxiliary regenerates the same —, `DEFINE CALLER.lam_N equations=`),
then each instance's definition and equations, adds the program, records
`INSTANCE NAME of=T shapes=g/1,… equations=…`, then the instances the
instance's own body demanded, to a fixpoint; a name already built is
reused when the demand's arguments agree (`demand_same`) and refused
`instance_clash` when they differ — never silently reused. The escape
rule's pointers (rule 2) at `erasure.shard`'s sites and on the E-first
route: a `(-> …)` type in an E signature and a `fun` in an E-first body
are `function_value` naming the template route, where they were
`unknown_type: ->` and `unknown_head: fun`; a template cited from an
E-first body is refused the same way, since this landing takes
instances on the typed route only (rule 3's E-first instance, with no
generated definition, is deferred to landing 2 with the examples; a
`realize` body lifting a lambda or taking an instance, and a `realize`
with a function-typed binder, are refused until landing 2's realize
templates). Measured: a theorem over the instance by `Eq.refl` through
the unfolding (`inc2 x = twice inc x`), a generic theorem over the
template applied at the instance under `apply`, an entry run under
`--run` through a typed `fn` calling an instance and a lifted lambda
(`01`). The six pins; the loader pins' entrypoint 250 cases 0 failed;
parity byte-identical over 31 closures and 135,063 declarations; the full suite's 39 entrypoints with 0
failed in 568 s (`store_test` 437 s, `parity_test` 315 s, `define_test` 273 s, `path_test` 259 s, `calc_test` 251 s). Route 1's kernel build, the full replay and the
corpus are CI's.

**As built, landing 2 (2026-10-11): realize templates, packages, the six
list templates, the examples and the entrypoint.** Rules 5–6 and rule 1's
realize half. A supplied `realize` with a function-typed binder is a
template (`realize.shard`'s `realize_template`): its body elaborated once
against the constant's telescope and its equations proven over the
arbitrary function as any realization's — the L half —, its written static
type `(-> D… R)` checked against the L type's domains and result as E
types (`static_signature`; `erase.shard`'s `static_etypes`, of which
`fn_arity` is the count), no program; the loader
(`load_realize_template`) checks the body at placeholder shapes
(`check_template`), admits the equations, registers `EITemplate` and
records `TEMPLATE NAME static=f equations=NAME.realize_1,…`.
`template.shard`'s `Template` carries either kind — `TmFn` with the
pre-definition, `TmRealize` with the result's L type, the levels and the
equations' identities as the realizing module declared them —; an instance
of a realize template is built as a fn's, its equations `NAME.eq_N` from
`T.realize_N`, and a realize template's self-call names its constant
(`erase_const_at` routes the walk's owner to `erase_template_call`). A
realize body that lifts a lambda or demands an instance now takes them
(`RzOk` carries them; `load_realized` builds them before the program is
added; `admit_theorems` shared with the template's path). `std/list`
realizes `List.map`, `filter`, `foldl`, `foldr`, `any`, `all` (rule 6):
five by `Eq.refl` per leaf, `filter`'s rows through the `equations` clause
— Init's cons row is a match on the test and the body's an `if`, equal by
`List.filter_cons`, not by reduction. Packages (rule 5): `erase.shard`'s
`package_of` — a structure of one constructor and no indices that is no E
type, its fields read by `binder_role` over the parameters' locals: a
function field static, a proposition a law, an E-typed field a value, a
type field no package —; `binder_role` answers `BStatic 0` for a binder of
a package type (a function type has arity at least 1), so a package
parameter is a static parameter of arity 0 and passed on as any. A package
constant as a static argument (`erasure.shard`'s `package_shape`): a closed
term whose whnf is the constructor applied, the shape its identity with
its arguments and nothing captured (`IntAdd/0`). A projection of a package
(`erase_package_proj`: from the projection function applied past its
lambdas — `proj_fn_app` now returns the arguments past the subject — and
from `Proj`): the subject a package parameter at its shape or a closed
package term, its fields the constructor's arguments after whnf
(`package_subject`); the field's term applied to the arguments and erased
— a name a call, a lambda beta-reduced, a value field its closed term, a
law field `erased_in_runtime` —; at the declaration's check the
placeholder's field by its role. A package's function field as a static
argument of another template (`package_field_shape`: `(List.map (Monoid.op
m (Monoid.unit m)) xs)` in a template over `m`) is the field's term at the
constant, the parameter replaced by the constant in the arguments, read as
any static argument — a lambda field applied is beta-reduced and lifted
under the taking body's name, not the constant's (departure for the
close-out). The structure reader makes a structure's parameters implicit
in its constructor and projections and explicit in its type, as Lean
declares them (`implicit_prefix`) — so `(Monoid.op m x y)` and `(Monoid.mk
Int.add 0 …)` read as the design wrote them —, and an earlier field applied
in a later field's type resolves as the projection alias it is
(`elab_app_sym`; it was `unknown_constant`). Two landing-1 defects found
and fixed: a polymorphic recursive template was refused at its declaration
(`template_args` refused the self-call's type arguments — the template's
own type parameters — as an open type; now a template inside a template's
body is taken at the enclosing template's type parameters, the template
itself only at them exactly, rule 4's `realize_descent` at a type built
from them), and `check_template` passed the instance's name where the
walk's type parameters go, a type error the bootstrap never saw. The
E-first instance (rule 3's last sentence) is deferred to landing 3: the
E-first reader carries no types, and an instance needs the type arguments
from the arguments' E types; an E-first body calls a typed fn that took
the instance (the entry's idiom). `v3/examples/template/`: `add_offset`
(law §5.3's, as the law writes it), `named` (a fn, a partial application,
the registry's `Int.add` under `foldl` and `foldr`, a predicate under
`filter`, `any` and `all`), `package` (`Monoid` at `Int` by a name and by a
lambda, `fold_m` over `m` with its generic theorems applied at the
constants by `apply`, `scale_all` passing the operation on),
`template_main` (the entry, `9662014`). `kernel/test/template_test.sh`,
the 40th entrypoint: the load, the records, the run, the artifact check
over `--dump` — no `fun` and no `->` in any program line while the sources
hold them, a partial application unwritable in the dump's grammar; the
instances and the lifted lambda programs of the artifact, the templates
none. Pin `template_package`. Gates: the loader pins' entrypoint 251 cases 0 failed in 56 s; `std_test` 201 checks 0 failed in 151 s (the std root's 8 files and 356 declarations, 20 more than at landing 1 — the six templates' twelve equations and `filter`'s two —, stored in 86 s and replayed by K alone in 65 s); `template_test` 40 checks 0 failed in 102 s (15 instances in the artifact, 17 lambdas and function types in the sources, none in the programs; load 35 s, run 33 s, dump 34 s); parity byte-identical over 31 closures and 135,443 declarations in 297 s; the seven touched kernel files clean under the V3 loader (no refusal, no unused open); the full suite's 40 entrypoints in 552 s with one failure, `define_test`'s two stale counts (28 and 20 defined became 34 and 26, the six templates), corrected and rerun green (12 checks 0 failed in 259 s).

**As built, landing 3 (2026-10-11): the budget and the workload, the growth,
budget and E-first pins, the E-first instance, `LEAN.md`'s rows, the T2 and
T3 record — the slice's close-out.** Rule 4's budget: `Load.budget`
(`loader.shard`'s `instance_budget`, 1,024; carried across re-roots; set for a
load by the driver's `--instance-budget N`); at a demand the template's
instances in `Load.instances` are counted (`instances_of`) and the next past
the budget is refused `specialization_budget` under the taker's name
(`over_budget`, shared by both routes) — a refusal, never a proof. T3's
workload: `v3/examples/template/workload.shard` takes 64 instances in one
file — `List.map`, `filter`, `any` and `all` at 16 lifted lambdas each: 64
lifted fns and 64 instances with their 128 equations built and checked by K in
one load (33 s on the bootstrap with `std/list`); `workload` combines every
instance's result (509 over `(1 2 3)`), the entry's last three digits by `/`
and `mod` at `Int`. Pins `template_growth` (`f[A]` recursing at `f[List A]`:
`realize_descent` at the declaration — landing 2's check, the pin T3's
canonical refusal) and `template_budget` (the harness loads that case at a
budget of 2, `case_budget`, so the third shape of `twice` is refused under its
taker). The E-first instance (rule 3's last sentence): a template called in a
body with no L reading takes an instance with a program and no generated
definition. The E table's `EITemplate` carries the template's written binders'
E signatures (`etable.shard`'s `EBind` — a static binder's domains and result
over the type parameters, a data binder's type, a package binder;
`template.shard`'s `tm_ebinds` over `static_etypes`, at registration by
`loader.shard`'s `register_template`, one path for a fn's and a realize's
template), so the classifier reads a template call (`read_template_app`): a
data position as any term, a static position as a shape (`read_static_arg`) —
a name of the E table with a program and no type parameters, bare or applied
to runtime terms short of its arity, transiently the name applied to its
captured terms. Once the body is read, `take_instances` walks it with its
types (a match's variables from the scrutinee, a let's from its value, as
`c_type` reads them), solves the type arguments by unification of the binders'
E types against the arguments' (a static binder's domains and result against
the name's trailing parameter types and result, a data binder's type against
its argument's static type), refuses a type argument left open
(`function_value`: a polymorphic taker is the typed route's), names the
instance by the one formula (`etable.shard`'s `instance_name_of`, which
`erasure.shard`'s `instance_name` now calls) and rewrites the call to the
instance at the captured values then the data; the classifier answers `COkT`
with the demands (`EDemand`), and the loader (`load_e_first` →
`build_e_demands`) builds each from the template's program half alone
(`template.shard`'s `build_program`, split out of `build_instance`;
`build_instance_e`; the E types to L by `etype_to_l` — every E type in Type,
a constant's levels by `solve_levels`), reuses a name already built at the
same arguments, holds the budget, admits what the template's body lifts and
builds what it demands as typed, and records `INSTANCE NAME of=T shapes=…
equations=` with no equation. Refused E-first with pointers: a lambda (nothing
lifts it in a body with no L reading — name it as a fn and pass the name, or
give the body an L reading), a bound variable, a constant of the realization
registry (`Int.add`: name a fn over it), a polymorphic fn (its type arguments
are not written E-first), a template, a constructor, an extern, a package
parameter. The entry `template_main.main` takes `List.map` at `digit` over
the results itself (`List_map_Int_UInt8.of.…digit_0`, no equations); pin
`template_e_first` takes `twice` at a name and at a named partial application
and the polymorphic `mapi` at `Int` — ten pins, the design's nine and this.
`LEAN.md`: the function-value row became four rows of what is now the same as
Lean (a lambda under the six, `f ∘ g` as a lambda, a fn over a function, a
package as a structure), five of what stays refused (a function in a value
position, a static argument built from a parameter, the type-growing
recursion, the budget, the E-first forms) and three shard-only rows
(templates and instances, the package, `--instance-budget`).
`docs/records/T2-T3-2026-10-11.md` records T2 and T3 against law §12.5's risk
columns, the count fixed by the design before the build. Gates: the loader pins' entrypoint 254 cases 0 failed in 53 s (55 s inside the suite); `template_test` 46 checks 0 failed in 101 s (80 instances in the artifact — landing 2's 15, the workload's 64, the entry's E-first one —, 81 lambdas and function types in the sources, none in the programs; load 35 s, run 34 s, dump 33 s); `std_test` 201 checks 0 failed in 152 s (356 declarations stored in 88 s, replayed by K alone in 63 s); `define_test` 12 checks 0 failed in 252 s; parity byte-identical over 31 closures and 135,867 declarations in 311 s (424 more than at landing 2, the kernel's growth); the five touched kernel files clean under the V3 loader; the full suite's 40 entrypoints with 0 failed in 519 s (`store_test` 399 s, `parity_test` 311 s, `define_test` 252 s, `path_test` 234 s, `calc_test` 227 s). Route 1's kernel build, the full replay and the corpus are CI's.

**Weighed and deferred,** each with the consumer that reopens it: the
dynamic tier (escaping closures, function values in data, dispatch on
functions) — law §4.3's named door, sealed variants the alternative,
wake = a named workload with a demonstrated cost; instances shared
across modules (each module takes its own, as with derivations, and the
store holds them per file; a consumer measuring the duplication reopens
it); an extern as a static argument (it takes a World; §4.7's effect-use
contract is phase 4's); a lambda returning a lambda and a function-typed
field of an E type (the escape rule; a consumer names the workload);
route 1 over user files (phase 4). **Rejected:** a runtime closure or
dictionary representation (law §4.1, §4.3); the captured values in the
specialization key (law §4.3: never); an instance per call site (the
shape tuple dedupes); a `template` keyword (the binder's type is the
signal, one grammar); untyped lambda binders (`(fun ((x T)) …)` is L's
form, §5.1).

#### Slice 3.29

**Slice 3.29 — the tactic library: T10, the proof IR's gate**
(designed 2026-10-12 at the boundary after slice 3.28, on the user's
"proceed with the next slice"; §13 item 63 — **ratified 2026-10-12 as
designed, the four leans taken and rule 10's limit accepted: a library
composes at I's level until route 1 compiles user files (the user: "the
limit is okay for now")**). Law
§12.5's T10 row names the gate: an ordinary E library contributing an
I-producing tactic through the fixed interface, its evidence checked
through the same path as built-in tooling, while a proposed grammar
extension receives the documented refusal; replay unchanged under
changed ambient simp and instance settings; a dependent motive; reused
local names in sibling branches; a guarded occurrence whose source
changed; an external P accepted through `exact`; an I schema upgrade
separate from a foundation change; one engine's solution checked
identically by another — the risk column: an IR only its producer can
read. Law §7.3 (the goal graph API as E functions over E data; a search
engine any E program composing them), §7.2 (the replay contract; the
four versions), §7.5 (sources carry tactic blocks or an `auto`
delegation naming the policy; no tool writes into a source file; a
stale pin is a pending obligation) and §5.3 departure 6 (the syntax ban
scoped: E libraries producing I through the published APIs are the
extension model; R40) fix the shape.

*Where the slice starts.* Slices 3.20–3.22 built I's core, the two
engine-carrying forms and the producers: `kernel/tactic.shard` holds
the API's four functions over one derivation (`goal_of`, `applicable`,
`tac_step`, `tac_prove`), `kernel/engine.shard` the ladder — a producer
of I composing them in-process — and `kernel/prove.shard` the driver
(the engine on each `auto` without a valid entry, the sidecar rewritten
whole). The engine is the one producer, and it is wired into the reader
(`reader.shard` imports `engine.shard`; `read_auto` calls
`engine_prove`): a library has no door. The sidecar's header names the
engine's version in a comment the loader never reads. **Measured before
this design** (three probe files under the on-demand loader, the
bootstrap, 2026-10-12, 25 s a run): (i) `cases` on a local with a
hypothesis over it leaves the hypothesis speaking of the old local —
the zero arm of `(n : Nat) (h : (< 0 n)) ⊢ (<= 1 n)` faces `(h : (< 0
n)) ⊢ (<= 1 Nat.zero)`, which no step closes (slice 3.20's as-built:
"a dependent would be left speaking of the old local, so then it
stays"); T10's dependent motive is this gap; (ii) an unknown step is
refused by name with I's roster and no pointer to a producer; (iii)
`ev` already carries text in through the entry's codec and a value out
(`run_prog`'s `arg_in`; `ev` the pure call), so a library's function
runs on route 2 with nothing new in `ev`.

*The ruling asked for* (four leans; the rest decided in the rules).
(1) **The fixed interface is data at the boundary, one shot**: a
tactic is a `fn` taking the goal as S text and answering a block as S
text; the kernel elaborates and K checks what comes back, as it does a
hand-written block. Rejected: the elaborator's state crossing into the
library (the environment is sealed — a `CheckedEnv` is built by K
alone, §13 item 26 — and a value conversion of Init's 58,000
declarations per call); a host callback (law §7.3: never); a
round-trip protocol (law §6's `attempt`/`commit` has no concurrent
caller, as at 3.20). In-process composition of `step` is the kernel's
engine's until route 1 compiles user files (phase 4); the law's "any
E program composing these" reads at this phase as: the engine is such a
program, a library composes at I's level. (2) **The producer is named
in the source's delegation**: `(auto (tactic M.FN) HINT…)`. Rejected: a
`--tactic` flag on `prove` (law §7.5: the source carries the delegation
naming the policy); `(by (tactic M.FN))` run at build (the build never
searches). (3) **The sidecar carries I's schema**: `(sidecar N)` as its
first form, N = 2 from this slice; a sidecar without it is schema 1,
replayed and reported, never refused; one above the current is refused.
Rejected: the engine's version as the schema (law §7.2 keeps four
versions apart; the implementation's stays in the comment); a version
per entry. (4) **The guarded occurrence carries its expected subterm**:
`(occ K EXPECT)`. Rejected: a positional path (law §7.2: "not a
pretty-printed index").

*The rules.*

1. **The tactic signature.** A tactic is a `fn` of an ordinary module,
   `(fn NAME ((goal TEXT) (hints TEXT)) TEXT)`, TEXT any byte type the
   entry codec carries (§9: the prelude's `(List Int)`, Init's `(List
   Nat)` or `(List Int)`, `ByteArray`), no World. The goal arrives as
   one S form, `(goal ((NAME TYPE)…) TARGET (applicable SYM…))`: the
   named context in order, the target, and the forms whose
   precondition the goal meets as `applicable` answers them (a hint,
   not a completeness claim); every term as the elaborator renders a
   goal — bound names, `=` and the comparisons under their operators,
   `Int` literals as source writes them — with the binder spelling made
   S's (`(forall ((x T)) …)`, `(fun ((x T)) …)`, `(let ((x V)) …)`;
   `goal_of`'s display keeps `(x : T) … ⊢` at the top only). The hints
   arrive as the S text of the forms after `(tactic M.FN)` in the
   delegation, `()` when none. The answer is a block `(STEP…)` in I's
   concrete syntax, or the empty text for no answer. The library sees
   no `El`, no `Expr`, no environment: it plans over data, and what it
   plans is checked.
2. **The delegation names it.** `(auto (tactic M.FN) HINT…)` in a
   `theorem`, a `fulfills` or a discharge: M.FN the module-qualified
   name of a `fn` in the load — the file imports the library as any
   module. Under `prove` (mode `solve`) a theorem so delegated without
   a valid entry runs the tactic under `ev` on the loaded programs
   (route 2; the goal text through the entry's codec; the driver's
   fuel), reads the answer as a block and replays it through
   `tac_prove` before the theorem is accepted — the engine's path and
   a hand-written block's; the sidecar entry it writes is `(proof-for
   NAME FP (by STEP…))`, indistinguishable at the pin (law §7.5). A
   name not in the load or not of rule 1's signature is `PENDING NAME
   auto_unsolved fp=N tactic=M.FN: REASON` with the signature in the
   message; a refused answer `… tactic=M.FN step K: REASON — the goal:
   …` (the library author's feedback); the empty answer `… no answer`;
   a run out of fuel or stuck `… ev: REASON`. Under `load.shard` the
   delegation is the producer's and unread, as the hints are: the
   build replays the sidecar only. Under `--measure` the engine runs
   as today, and the tactic when named: `ENGINE NAME solved|unsolved`
   and `TACTIC NAME M.FN solved|unsolved`.
3. **The refusals (the row's "documented refusal").** An unknown step
   in a block is `unknown_step` with I's roster and the pointer: a new
   tactic is an E library producing these forms, named by `(auto
   (tactic M.FN))` (§8.4 slice 3.29); `notation` and its kin keep
   `lean_form`'s pointer, naming the library route. A `(tactic …)`
   inside a hand-written `by` is `unknown_step` too: a producer is
   named in a delegation, never inside I.
4. **The schema.** A sidecar's first form is `(sidecar N)`: I's schema
   version — the roster of forms, their arguments and the
   reconstruction defaults — 2 from this slice (rule 5's guard and
   rule 2's delegation are the change). A sidecar without the form is
   schema 1 (every sidecar written before this slice), replayed as
   before and reported once per file, `SIDECAR FILE schema=1
   current=2` (the counts unchanged: a report, not a refusal); one at
   a schema above the current is `sidecar_schema`, the file's load
   error and its theorems pending — the future is unreadable. `prove`
   writes the current. Law §7.2's four versions in V3: K's logical
   rules (the kernel's sources, pinned by T0 against the export); I's
   schema (this form); P's encoding (the store's format 3.1.0); the
   producer's implementation (the engine's version in the sidecar's
   comment, a library's in its own module). A foundation change — a
   new export, a new K — moves none of these; the gate shows a
   schema-1 sidecar replaying under schema 2 with the records equal but
   for the report.
5. **The guard.** `(rw LEMMA (occ K EXPECT) STEP-BLOCK…)`: EXPECT a term
   elaborated at the matched instance's type; the instance the first
   match found must be EXPECT after zonk (syntactically, as `rw`
   matches), else `rw_occ_changed` showing both — the target's source
   changed since the block was written and the K-th occurrence is
   another instance. `(occ K)` alone stays as built (the K-th
   occurrence of the first instance, Lean's). The engine writes no
   `occ`; a library may; the pin writes one by hand.
6. **The dependent motive.** Under `cases` and `induction`, every local
   of the context whose type mentions the eliminated local
   (transitively), and the equation hypothesis when named, is reverted
   into the motive — `λ x. (h : T[x]) → … → target[x]` — and each arm
   re-introduces them under their names after the constructor's
   fields, so the arm sees `h : T[CTOR fields…]` (Lean's `cases`
   reverts dependents). The eliminated local leaves the arm's context
   (today it stays when a dependent exists). `wf` is unchanged: its
   hypothesis quantifies.
7. **Sibling names, external P, settings: pinned, nothing built.** Two
   arms naming the same fields and the same `have`, each filling
   closing over its own locals — a goal is scope-safe by construction
   (an arm opens fresh locals; display names are the binders', slice
   3.20 rule 2). A theorem closed by `(exact TERM)` with TERM a proof
   term from outside the elaborator's producers: one of Init's theorems
   applied (Lean's P through the export) and one the example library
   builds as text (`(exact (Nat.le_refl n))`), accepted through the one
   check. A sidecar replaying unchanged when the source's hints changed
   and when an unrelated lemma module is imported: an entry carries its
   rules (`simp_only RULE…`, `arith only FACT…`), hints are the
   producer's and unread at replay, there is no ambient simp set, and
   Init's instances are closed by the export (slice 3.27 rule 3's
   table).
8. **One solution checked identically.** The example's theorems are
   solved twice, by the engine and by the library, and the test replays
   each sidecar under `load.shard` to the same `ACCEPT` records (the
   fingerprints the statements'), stores each with `--store` and
   verifies each by K alone (`verify_release`): the proof terms differ,
   the acceptance does not, and no producer is consulted at replay.
9. **The example and the gate.** `v3/examples/tactic/`: `bool_split.shard`,
   an ordinary E library importing `kernel/sexpr.shard` for the S
   reader and printer — a decision procedure for goals over `Bool`
   locals: it reads the goal, collects the context's `Bool` locals and
   answers nested `(cases b ((false () …) (true () …)))` with every
   leaf `(decide)`, complete for its fragment; a target `(<= x x)` or
   `(= x x)` by an `exact` term (rule 7's external P); about 80 lines.
   `use_split.shard`: theorems delegated `(auto (tactic
   examples.tactic.bool_split.solve))` — commutativity of `&&`, De
   Morgan, `(|| a (! a)) = true`, `(<= n n)` — with its committed
   sidecar. `kernel/test/tactic_lib_test.sh`, the suite's 41st
   entrypoint: the replay (no `PROVED`; every theorem `ACCEPT`); the
   regeneration byte-identical to the committed sidecar but for the
   fingerprints (as `engine_test.sh`); the engine's sidecar for the
   same statements regenerated under `(auto)` in a scratch copy, both
   replayed, stored and verified (rule 8); the schema-1 copy reported
   (rule 4); the hints-changed copy replayed (rule 7). Pins:
   `by_cases_dependent`, `by_sibling_names`, `by_exact_external` (ok);
   `by_rw_occ_guard` (`rw_occ_changed`); `auto_tactic` (an entry a
   tactic wrote replays with the library imported and never run);
   `auto_schema_old` (ok, reported); `auto_schema_future`
   (`sidecar_schema`); `auto_hints_changed` (ok); `by_unknown_step` and
   `lean_notation` with their pointers (expectations unchanged). The
   record `docs/records/T10-DATE.md` against law §12.5's row, each
   clause with its pin or test; `docs/LEAN.md` gains the row for a
   tactic extension (Lean's `macro`/`elab` tactic → an E library named
   by the delegation) and the syntax row's pointer.
10. **Not in 3.29**, named: a library composing `step` in-process
    (route 1 over user files, phase 4; the engine stays the kernel's);
    `attempt`/`commit`, owners and snapshots (law §6; a concurrent
    caller first); incremental goal states (T6, phase 4); `rw` at a
    hypothesis; a tactic with World access (a tactic is pure); several
    tactics in one delegation (name one; a library composes its own).

*Landings.* 1: rules 3–7's kernel half — the pointers, the schema, the
guard, the dependent revert, the pins; gate: the pins, `tactic_test`
and `engine_test` unchanged. 2: rules 1, 2, 8 and 9 — the interface,
the delegation under `prove`, the example library, the entrypoint, the
T10 record, `LEAN.md`'s rows.

*As built, landing 1 (2026-10-12).* Rules 3–7's kernel half as
designed, with three readings recorded for ratification at the
close-out. (i) The guard compares the instance found with EXPECT by
structure first and by the elaborator's unification second
(`rw_occ_check`: `expr_eq`, then `el_unify`, K's conversion under it),
so a literal spelled one way in the source and another in the lemma
never trips it; `rw_occ_changed` shows both instances. (ii) The
equation hypothesis precedes the reverted dependents in the motive —
`λ d. (= x d) → (h : T[d]) → … → target[d]` — and an arm re-introduces
the fields, then `H`, then the dependents under their names
(`cases_minors` opens them as it opens the fields; `ctx_dependents`
collects them transitively in the context's order); with an equation
name the eliminated local stays, the equation naming it, without one
it leaves; an induction hypothesis quantifies over the dependents —
calc's 100 claims replay unchanged. (iii) A sidecar's first form
`(sidecar N)` is read by `sidecar_split` before its entries, and
`prove` writes `(sidecar 2)` after the header comment whenever it
rewrites a file whole; an entry still valid is never rewritten, so the
schema form appears only when a file is written — the two committed
sidecars (`examples/auto`, pin `auto_core`) were regenerated by `prove`
from nothing, and the engine now closes `auto_core`'s `u` by `rfl`
where the slice-3.22 entry held an induction; pin `auto_schema_old`
keeps that older sidecar as the schema-1 artifact it is. The refusal
pointers: `unknown_step`'s two messages and `lean_form`'s notation row
end with "a new tactic is an E library producing these forms, named by
(auto (tactic M.FN)) (§8.4 slice 3.29)"; `(tactic …)` inside a hand
`by` is `unknown_step`. Measured as pinned: the dependent motive
closes the probe's theorems (`by_cases_dependent`, three: the zero
arm's `h : (< 0 Nat.zero)` refuted by `arith`); sibling names and the
external P held before the landing (`by_sibling_names`,
`by_exact_external`, loaded first, nothing built); `by_rw_occ_guard`'s
`moved` is refused with both instances shown; `auto_schema_old`
replays with `SIDECAR main.shard schema=1 current=2`;
`auto_schema_future`'s `(sidecar 3)` is the load error
`sidecar_schema`; `auto_hints_changed` replays an entry written under
`(induct n)` with the source now saying `(budget 1)`. Gates: the
loader pins' entrypoint 259 cases, 0 failed, 58 s (the list's
duplicated `arith_eq_bare` entry removed); `tactic_test` 110 checks,
0 failed, 52 s; `engine_test` 24 checks, 0 failed, 179 s (the
regenerated sidecar byte-identical but for the fingerprints, 54 of
100 claims); parity byte-identical over 31 closures and 135,940
declarations in 308 s; the full suite's 40 entrypoints, 0 failed, in 530 s.

*As built, landing 2 (2026-10-12).* Rules 1, 2, 8 and 9 as designed,
with the readings recorded for ratification at the close-out. (i) The
goal's terms are rendered by the one renderer the refusals use
(`tc_show`), whose binders now read as source writes them —
`(forall ((x T)) P)`, `(fun ((x T)) e)` — in every message as well,
one spelling; `goal_sx` frames them as `(goal ((NAME TYPE)…) TARGET
(applicable SYM…))`. (ii) The producers' inputs travel together:
`reader.shard`'s `Producers` carries a file's sidecar entries, the
programs loaded so far (`Load.etab`) and the fuel (`Load.fuel`:
`default_fuel`, now the loader's, or `prove.shard --fuel N`), built by
the loader at the three proof sites (a theorem, a `fulfills`, a
discharge); `run_tactic` finds M.FN in the programs (`prog_find`),
requires two parameters and a result of byte types (`ev.shard`'s
`text_type`, `list_ctors`' judgment) and no type parameter, makes the
goal as `engine_prove` does, carries the texts through `val_of_text`
and `text_of_val` (the entry codec's cells) and runs `ev` on the
loaded programs; the answer is one form, the block's steps, read by
`read_all`. (iii) The refusals as designed, each measured: an unknown
name, a wrong signature (the signature shown), a refused answer
(`tactic=M.FN: decide_failed: step 1: … — the goal: …`), the empty
answer (`no answer`), an answer that does not read (`the answer does
not read: unterminated_list at byte 1 of (`), two forms (`the answer
is not one block (STEP…): ()()`), a stuck or exhausted run (`ev: …`),
two tactics in one delegation (rule 10's refusal), `(tactic)` without
a name; `(by (tactic …))` is `unknown_step`. (iv) `--measure` runs the
engine as today and the tactic when named, `RDMeasured` carrying the
tactic's name and result, the loader recording `TACTIC NAME M.FN
solved|unsolved` beside `ENGINE`. (v) The example library is E-first
(`RUNNABLE … route=e_first`, as the kernel's own S reader it imports
is), six theorems rather than four — `and_assoc` (three locals) and
`eq_self` (the `(= x x)` term) added —, and a statement over Init's
names alone has the same fingerprint in every module, so the
entrypoint's regenerated sidecar is byte-identical to the committed
one with nothing masked (`engine_test`'s mask is for a statement that
names the module's own constants); the engine closes the same six
(`and_comm` by nested inductions, `de_morgan`, `excluded` and
`and_assoc` by one induction and `rfl`, `le_self` by `arith`, `eq_self`
by `rfl`), and the two sidecars replay to the same six `ACCEPT`
records but for the axiom closure, which is the proof's and not the
statement's — the engine's `le_self` by `arith` rests on `propext` and
`Quot.sound` (Omega's kit), the library's `(exact (Nat.le_refl n))` on
nothing; rule 8's "the same `ACCEPT` records" holds in the name, the
module, the fingerprint (the `hash` field), the horizon and the
parameters, and the gate compares those and reports the two closures.
(vi) Pin
`auto_tactic`'s library is a constant tactic of the pin's root (a
pin imports nothing outside it), typed over Init's `(List Int)`;
`bool_split` imports `kernel/sexpr.shard` by relative path, so the
entrypoint's scratch copy one level under `v3` rewrites that path.
Gates: the loader pins' entrypoint 260 cases, 0 failed, 58 s; `tactic_test` 110 checks, 0 failed, 52 s; `engine_test` 24 checks, 0 failed, 182 s; parity byte-identical over 31 closures and 136,735 declarations in 313 s; `tactic_lib_test` 30 checks, 0 failed, 280 s; the six touched kernel files clean under the V3 loader; the full suite's 41 entrypoints, 0 failed, in 548 s.

**The close-out (2026-10-12): the two landings are built and hold on
the full gate (pipelines 588 and 589, 2026-10-13); the as-built
readings above — landing 1's guard by structure then unification, the
equation hypothesis before the reverted dependents, the sidecars
regenerated from nothing; landing 2's one renderer with the source's
binder spelling, `Producers`, the E-first library and the six theorems,
the fingerprint's independence from the module — ratified
2026-10-13 (§13 item 63's note).**

## 10. Related records

`docs/COVERAGE.md` and `docs/records/COVERAGE.md` (the coverage arc,
PARKED at B-1b 2026-09-05; resumes at phase 7 on V3); `docs/TCB.md`
(the roster this replaces); `docs/TOTALITY.md` (the measure regime,
carried as elaboration); `docs/CERT.md` (validators carried; §3/§7
superseded); `docs/SEARCH.md` (LS-laws carried; the lock-step law
becomes joint search under one metavariable context); `docs/MEMORY.md`
D8 (fail families); `docs/BOUNDARIES.md` (the World discipline, with
the R25 correction owed); `docs/FLOATS.md` (kept as ours).
