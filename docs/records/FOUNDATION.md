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
- **Open in phase 1:** the memo tables, the full-export run and its
  cost measurement, the six accelerator pins declared 24k–700k lines
  into the export, the `use`-free toolchain profile's gate moving to
  CI.

## 9. Related records

`docs/COVERAGE.md` and `docs/records/COVERAGE.md` (the coverage arc,
PARKED at B-1b 2026-09-05; resumes at phase 7 on V3); `docs/TCB.md`
(the roster this replaces); `docs/TOTALITY.md` (the measure regime,
carried as elaboration); `docs/CERT.md` (validators carried; §3/§7
superseded); `docs/SEARCH.md` (LS-laws carried; the lock-step law
becomes joint search under one metavariable context); `docs/MEMORY.md`
D8 (fail families); `docs/BOUNDARIES.md` (the World discipline, with
the R25 correction owed); `docs/FLOATS.md` (kept as ours).
