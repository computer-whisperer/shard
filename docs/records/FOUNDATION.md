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

## 10. Related records

`docs/COVERAGE.md` and `docs/records/COVERAGE.md` (the coverage arc,
PARKED at B-1b 2026-09-05; resumes at phase 7 on V3); `docs/TCB.md`
(the roster this replaces); `docs/TOTALITY.md` (the measure regime,
carried as elaboration); `docs/CERT.md` (validators carried; §3/§7
superseded); `docs/SEARCH.md` (LS-laws carried; the lock-step law
becomes joint search under one metavariable context); `docs/MEMORY.md`
D8 (fail families); `docs/BOUNDARIES.md` (the World discipline, with
the R25 correction owed); `docs/FLOATS.md` (kept as ours).
