shard program search — SEARCH.md
=================================

STATUS: RATIFIED (user review 2026-07-11) — the scope ledger for the
meta-search arc: a lasting, in-repo replication of the search
playground's basic behaviors, built under meta/ against the real
kernel, the real canonical dialect, and the real proof machinery.
User rulings on record: the durable-home identity (§1), the
clone-first graduation methodology (§4), the performance posture
(D7), D1 = reserved-head Call encoding, D2 = meta/sketch in meta/
from day one. D3–D5 and D8 stand as written (ratified defaults);
per-slice check-ins with the user per house norm. Development lives
in the shard.search worktree (branch search-arc, cut 2026-07-11);
this file is the scope authority.

The evidence base is ~/workspace/playground/shard_search_playground
(read as data, never touched). Its README is the measurement record:
needed narrowing over shard terms, lemma-quotiented grammars,
law-directed verdicts, the canonical-program catalog, and the mined
canon rules that CANON.md §13 turned into C11/C12. Numbers cited
below are from that README.


## 1. Why a lasting version: the durable home

The playground proved five things (its "Lessons for shard" section):
superposition is an executor strategy, sketches are meta-layer work,
the generator/recognizer duality earns its keep, canonicalization is
the whole cost of search, and the memo is the whole game.

The lasting version is built in shard because shard is where anything
we intend to DEPEND on lives (USER RULING 2026-07-11). Two facts
retire the speed anxiety up front:

- **The constant factor is a closing gap, not a given.** shard's
  performance gap with C/Rust is an active target, and closing it
  ENTIRELY is a success criterion for the language itself. A search
  engine written in shard rides that trajectory (a flagship consumer
  for the lowering arc), rather than hedging against it.
- **The domain's swings dwarf the constant factor anyway.** Measured
  across every playground arc: a demo either settles in under a
  second or is intractable, and which of the two it is is decided by
  the quotient/oracle match (the two-curve gap), never by evaluator
  throughput. A 100x hosting factor turns 0.5s into 50s and turns
  intractable into intractable; it does not move the boundary.

What the playground structurally CANNOT do is couple. It re-implements
an approximation of the dialect (its own canon flags), remaps
requirements by hand, deliberately stopped short of the proof finish
line ("rendering each Proven region's trace as a shard proof term …
deliberately not built yet"), and its counts ride machine words that
overflow. Every one of those gaps is a thing this repo already owns:

- **The real dialect.** kernel/canon.shard's recognizers (C1–C11) and
  tools/canon's fixpoint are the ground truth the playground's
  `canon`/`dialect` flags approximate. A generated candidate can be
  GATED canonical — generate, then assert `cn_e` returns Nil — instead
  of hoped canonical.
- **The real requirements.** mod.req.shard surfaces parse natively;
  the laws oracle needs no remapping hacks.
- **The real proof machinery.** tools/prove's sidecar format, merge
  discipline, and bin/shard_check replay exist. The playground's
  missing finish line — Proven verdicts becoming zero-trust
  certificates — is REACHABLE here, and it is the single biggest
  reason to build in-repo at all.
- **Exact counting.** Int is bignum; the playground's u64→u128→panic
  ladder dissolves. Counts are exact at any rung, with no guard code.
- **The census flywheel.** The README's closing cycle — census →
  classify residue → name the rule and license → implement at the
  priced tier → re-measure with behaviors preserved — is the loop
  that produced C11. Running it against the REAL dialect, with
  kernel-certified equivalence brackets, is CANON.md's measurement
  instrument from §13 onward.

So the arc's identity: **the durable home for program search —
kernel-coupled, proof-finished — whose primary deliverable is the
meta/ vocabulary for navigating the space of shard code.** The
playground stays what it is: the throwaway exploration lab whose
findings graduate here.


## 2. What the repo already owns (the mapping)

| playground piece | in-repo owner |
|---|---|
| eval.rs (ground CBV evaluator) | kernel/evm.shard via meta/invoke's `evm_call_pure` — already the plan-step engine |
| canon flags / dialect grammar | kernel/canon.shard recognizers + tools/canon fixpoint (the ground truth) |
| four append lemmas, lia normalizer, ring rules | std/list, std/arith, kernel/facts — proven, typed, citable |
| law parsing (mod.req remap) | the module system's own req surfaces |
| proof finish line (absent) | tools/prove sidecars + bin/shard_check replay |
| u64/u128 counting | native bignum Int |
| SplitMix64 sampling | std/rng (or a sibling PRNG added under the same discipline) |
| recognizer side of the duality | meta/shape precedent (recognizers as a meta library; §6ae line) |
| behavior fingerprints (128-bit digests) | std/sha256 (hx_digest precedent) |

The pieces with NO in-repo owner — the sketch vocabulary, the
generators, the engines, the symbolic evaluator — are the arc.


## 3. The subsystem ledger

Each subsystem gets: what it buys, what it costs to maintain
(coupling), and a proposed verdict.

**S1. meta/sketch — the term-with-holes vocabulary.**
Sketch representation, per-hole grammar tables (alternative lists),
exact candidate counting, rank/unrank (the space is rank-addressable,
so sampling and slicing are free), fill/render, hole sharing (one
HoleId at two positions = same subtree, the correlation primitive).
*Buys:* the foundation every other subsystem consumes; lesson 2
verbatim ("a meta/ sketch vocabulary … is all shard needs").
*Costs:* pure data + arithmetic; near-zero coupling.
*Verdict:* **BUILD, first.**

**S2. Dialect grammar builders — the generator side of the duality.**
Grammar constructors that speak the canonical dialect as generation
constraints: C4 arm order, C5 exhaustiveness/no-dead-arms, decided
control excluded (C2/C11's generator image = the pinned normal form),
theory quotients where the lemma set is proven (append operand
constraints; ring spines later). Gated, not trusted: G1/G2 below.
*Buys:* lesson 3 — every recognizer run backwards; searchable spaces
at all; the measurement instrument that prices every future C-rule
(each new rule's generation-side payoff becomes a number).
*Costs:* the REAL maintenance item of the arc — a third speaker of
the canon rules (kernel recognizer, tools/canon fixer, generator).
Drift between speakers is the failure mode; the census gates are the
alarm. This coupling is also the point: it is CANON.md's flywheel.
*Verdict:* **BUILD**, with the gates as a non-negotiable part of the
subsystem, not an afterthought.

**S3. Ground engine — enumerate/unrank + evaluate.**
Candidates run against I/O batteries through the real evaluator
(evm_call_pure on the filled, closed term). Early-exit across tests,
absence proofs by exhaustion at small depth, the cross-check for
everything cleverer.
*Buys:* the baseline oracle; exactness for free; tiny.
*Costs:* near-zero (rides kernel/evm).
*Verdict:* **BUILD, first** (with S1).

**S4a. Symbolic evaluator — per-candidate three-valued verdicts.**
Symbolic values for ∀-binders, case splits allocating one shape per
ctor (symbolic ADTs over the type table), canonical neutrals (fn- and
prim-headed; join by same-head-equal-args congruence), verdicts
Proven / Refuted / Undecided with a split budget, and the two IH
licenses (D5). NOTE the decomposition this ledger makes that the
playground did not: the playground built its laws mode ON the
superposed candidate engine, but per-candidate symbolic evaluation
does not need candidate superposition — one term, one goal, case-split
regions only. That is a far smaller machine, and it is the one the
catalog and the proof finish line actually need.
*Buys:* the laws oracle (S5), catalog refinement (S7), and the input
to proof rendering (S6). One symbolic case refutes what ground
testing kills one value at a time.
*Costs:* the largest NEW engineering item that is not deferrable; no
existing shard code does this. Kinship with tools/prove is real (the
same case-split + steps + refl shape; its unhinted case-on discovery
already scans stuck frontiers) — shared machinery graduates to meta/
only when both speak it, per the hygiene-pass ruling; do not force
premature unification.
*Verdict:* **BUILD**, as its own slice, after S1–S3 prove the
plumbing.

**S4b. Superposed candidate executor — needed narrowing proper.**
The choices-map machine: shared thunk graph, consulted-choice-set
memo, fork on demanded holes, prefix kills, don't-cares.
*Buys:* essentially every fast settlement in the playground is a
narrowing result — enumeration is the engine that dies (1,181× at
rev d3; the only reason 10^15+ spaces settle at all). This is the
core engine of the "basic search behaviors" being replicated.
*Costs:* the largest single machine in the arc. The memo is
mutable-arena-shaped in Rust, and a pure-functional rendering
(structural keys, the two-level consulted-set index) is a real
engineering item — an item to engineer, not a reason to doubt the
tier (D6/D7).
*Verdict:* **BUILD, late in the ladder** — sequenced after
S1–S3/S4a because those rungs are its parts bin (vocabulary,
grammars, the evaluation substrate, neutrals), not because its value
or its hosting is in question. No performance go/no-go.

**S5. Laws oracle — requirements as the search oracle.**
Parse a requirement, bind its ∀-binders symbolically, remap the
subject fn to the sketch, compare goal sides under S4a. Rule sets for
canonical neutrals drawn ONLY from proven, typed equations (D4).
*Buys:* proofs instead of tests; the impostor problem (six
under-constraining examples "solving" sort) dissolves at the root.
*Costs:* thin over S4a + the module system.
*Verdict:* **BUILD** (same slice family as S4a).

**S6. Proof rendering — the finish line.**
A Proven verdict's trace becomes shard proof text: case-on/induct,
steps of unfold / rewrite-lemma / refl, IH citations — emitted as a
generated .shard program (the synthesized fns + claims/fulfills) with
its .auto.shard sidecar, replayed through bin/shard_check. A Proven
that fails replay is a HARD failure (G4). The search's verdicts
become zero-trust certificates; the engines stay outside the trust
surface entirely, like the compiled chain.
*Buys:* the thing the playground deliberately lacks; the arc's
unique payoff. Search output = a shard program the kernel has
checked, not a claim about one.
*Costs:* moderate; rides tools/prove's sidecar format and the proof
DSL. The render layer is coupled to proof-form syntax (stable, and
already machine-written by tools/prove daily).
*Verdict:* **BUILD** — and treat it as the arc's exit criterion, not
an optional extra.

**S7. Catalog / census — the CANON.md flywheel.**
Enumerate a canonical rung, battery-bucket by behavior digest
(std/sha256 over rendered output vectors; two-level fingerprinting
per the census lesson — silence is not success), refine the bracket
with S4a equivalence proofs (floor rises on refuted pairs, ceiling
falls on proven ones), and — with S6 — CERTIFY the bracket at small
rungs the way the playground certified d1 (19 programs = exactly 13
functions). Sampling instruments (rank-addressable + std/rng) later.
*Buys:* the census loop that mined C11/C12, now running against the
real dialect with kernel-checked equivalences; spellings-per-behavior
against the REAL rule ledger is the metric CANON.md optimizes.
*Costs:* thin composition of S1/S2/S3/S4a; battery data is data.
*Verdict:* **BUILD** (enumerate+bucket early; refinement after S4a).

**S8. Tasks and batteries.**
The concrete sketches (rev, sort's insert, the calculator, …), their
I/O batteries, their requirement oracles. Data plus thin drivers in
the tool bin. The oracle lessons travel with them: batteries need the
input shapes that separate impostors ([2,1,3] for insert; deep-push-
then-add for the VM), and every task should state whether its oracle
is tests (floor only) or laws (certifiable).
*Verdict:* **BUILD** (small; grows by accretion).

**S9. Gates.**
- **G1 canonicality:** every generated candidate at pinned small
  depths passes kernel `cn_e` = Nil and is a tools/canon fixpoint.
  The three-speakers drift alarm.
- **G2 quotient exactness:** at d1–d2, the dialect grammar's candidate
  set equals the raw grammar's normal forms, censused term-by-term
  (the --canon-verify discipline); behaviors preserved within the
  one-rung depth price (the raw twin's measured law).
- **G3 cross-engine agreement:** law-Proven ⊆ ground-test passers; no
  passer Refuted; ground and symbolic partitions consistent wherever
  both run.
- **G4 kernel replay:** every Proven renders and replays green
  through bin/shard_check. Non-negotiable.
- **G5 pinned counts:** canonical-program counts and behavior-bucket
  counts at fixed rungs pinned in run_corpus.sh (the census-gate
  discipline). A canon-rule change moves them; intentional moves
  re-pin with the change, like tools/canon's census today.
*Verdict:* **BUILD**, incrementally with their subsystems.

**Explicitly OUT (not built, not maintained):**
the OE engine (the literature control lost the races; its lesson is
recorded), the damascene race UI, the vanity/IC comparison, wedge
parallelism (until a compiled engine exists to parallelize),
hashprune/opportunistic pruning (an S4b follow-on; enters scope only
with it), u128 counter machinery (bignum), and playground parity as
a goal — the playground keeps modes this version never grows.


## 4. Placement: clone first, graduate into meta/

The development vehicle (USER RULING 2026-07-11): **build a clone of
the program-search toy as a tool, and graduate pieces into meta/ as
clean buckets emerge** — the tools/low/shape → meta/shape history,
adopted this time as deliberate methodology rather than discovered
after the fact. The arc's primary durable deliverable is the meta/
vocabulary for navigating the space of shard code (terms with holes,
grammars, enumerate/count/rank, fingerprints, symbolic verdicts);
the search tool is the forcing function that mines that vocabulary
and the example that proves each bucket before it graduates.

- **tools/search** — the clone: tasks, batteries, census drivers,
  CLI, corpus-gate entry points, and the engines while they are
  still finding their shape.
- **meta/sketch** — the one candidate for day-one meta/ residence,
  with its reuse story stated up front: term-with-holes +
  enumerate/count/rank/fill is lesson 3's ask ("shipping
  enumerate/count beside each recognizer would make every
  certified-lowering shape a search dialect for free") and
  meta/shape's natural sibling. Confirmed at D2: day-one meta/.
- **meta/search** (or finer buckets) — populated by graduation, not
  up front. A piece moves when its bucket is clean and a second
  consumer exists or is concrete, per the hygiene-pass ruling.
- Proof rendering (S6) starts in the tool; anything tools/prove
  later also speaks graduates the same way.

Trust posture, stated once: **the engines are never the soundness
authority.** Ground/symbolic agreement is a gate; the kernel replay
of rendered proofs is the only certificate; everything else is a
differentially-gated accelerator, exactly the compiled-chain regime.


## 5. Decision points

**D1 — hole representation (RESOLVED 2026-07-11 — ruling a).** A
reserved-head encoding inside kernel Expr —
`(Call (:: meta sketch hole) (IntLit k))` — so every existing Expr
walker, recognizer, and renderer works on partial terms unchanged,
with loud classifier helpers in meta/sketch; the kernel never checks
a sketch (holes are filled before any check). Rejected: (b) a
parallel SketchExpr type (duplicates every walker, cuts sketches off
from the real recognizers); (c) holes as reserved FVars (a numbered
hole id rides better as an IntLit argument).

**D2 — placement and graduation (RESOLVED 2026-07-11).** §4's
clone-first methodology stands, AND meta/sketch starts in meta/ from
day one on its stated reuse story. Trust tier: no engine output is
ever load-bearing without G4.

**D3 — evaluation substrate.** Ground evaluation via kernel/evm
(`evm_call_pure`, the meta/invoke precedent) — single hosted
interpretation, no engine-private evaluator to keep in agreement.
The symbolic evaluator (S4a) is necessarily new code; it shares the
term vocabulary but not the machine.

**D4 — oracle rule sets.** Canonical-neutral rewrites are drawn ONLY
from proven requirements, carrying their binder types as side
conditions (the append_nil_right lesson, learned three separate
times: an escaping operand needs its type known; spine-rebuilding
rewrites are fail-consistent). First cut: the append four + the lia
normalizer under std/arith's stated license. Ring rules (CANON.md
D9's predicted set) when a task demands them. Three certificate
categories, all kernel-replayable: definitional, lemma-cited,
decision-procedure-backed.

**D5 — induction licenses.** The two mechanical licenses the
playground validated: (i) the compiler-arc license — a stuck
recursive call whose subject is a wholly symbolic value produced by
case-splitting a goal binder, at split depth ≥ 1, rewrites by the
goal equation; (ii) the catalog license — the subject may be a
partially decided slot SHAPE at slot depth ≥ 1 (strong induction;
depth 0 excluded so the goal cannot cite itself). Both are search
heuristics only — the rendered proof faces the kernel's own induction
forms, and G4 is the judge.

**D6 — engine sequencing.** S4b is IN SCOPE — it is the engine
behind every fast playground settlement — and sequenced late only
because the earlier rungs are its parts bin. There is no performance
gate on it: constant factors do not decide tractability in this
domain (D7), so the only open question about S4b is build order.

**D7 — performance posture (USER RULING 2026-07-11).** The in-shard
version is the durable one: shard is not considered successful until
the C/Rust gap is entirely closed, and this arc builds on that
trajectory rather than hedging against it. The measured domain
structure backs it — demos settle in under a second or not at all;
tractability is decided by the quotient/oracle match, not evaluator
throughput. Consequence: **no scoping decision in this arc is made
on performance grounds.** When something is actually slow, the order
of attack is engine-level economies first (the evm/meta-invoke
precedent: 10min → 1.5s was machinery, not hosting), then the
lowering arc's native compilation (search as a flagship lib
consumer — the C-class dissolution).

**D8 — census discipline.** Behavior digests via std/sha256 over
rendered battery outputs; two-level fingerprints where the task has
an inner function; batteries as versioned data in tools/search;
counts pinned per G5. Sampling instruments ride rank-addressability +
std/rng under a fixed seed (bit-identical reruns).


## 6. The slice ladder

1. **S1 + S3:** meta/sketch vocabulary, exact counts, rank/unrank,
   fill; ground engine on evm_call_pure; the rev accumulator task
   with its battery; first pins in run_corpus.sh. Small, proves the
   plumbing end to end.
2. **S2:** dialect grammar builders over the real C-rules; G1 + G2
   gates at d1–d2. The duality demonstrated on the real ledger.
3. **S7-lite:** enumerate + battery-bucket a rung of the structural
   list fragment; first spellings-per-behavior numbers against the
   real dialect; G5 pins. (First feedback into CANON.md.)
4. **S4a + S5:** the symbolic evaluator and the laws oracle; append
   theory first; G3.
5. **S6:** proof rendering + kernel replay; G4. Exit criterion: one
   law-certified synthesis (rev against its interface) and one
   certified catalog bracket (the d1 "19 = exactly 13") land as
   corpus-pinned, kernel-checked artifacts.
6. **S4b:** the superposed executor — the choices-map machine over
   the by-now-proven vocabulary; G3 extends to three-way agreement;
   hashprune-style pruning follows it, not precedes it.

Each slice lands ratified-scope-first, gates with it, corpus
DIFF-clean, per house discipline. Graduation into meta/ happens at
slice boundaries when a bucket is clean (D2), not on a schedule.


## 7. Non-goals, stated once

No UI. No second evaluator to keep honest (the
ground path IS kernel/evm). No unproven rewrite ever enters a neutral
join. No engine verdict is ever cited without its G4 certificate. And
no obligation, ever, to keep up with the playground — it explores,
this version consolidates.


## 8. The arc record (slice ledger)

### Slice 1 — meta/sketch + the ground engine (LANDED 2026-07-11)

**What landed.**

- **meta/sketch** (day-one meta/ residence per D2): the D1 reserved-head
  hole encoding with its loud three-way classifier (HVHole / HVNot /
  HVBad — a hole-headed call with any other argument shape is refused,
  never skipped; the reserved head is the qname of the `hole` builder
  itself), hole collection with sharing-dedup, closedness, verbatim
  fill (single pass, no binder shifting — the fill contract is stated
  in the header), and the grammar layer: per-hole alternative tables
  (GEnt/Grammar) under THE STRATIFICATION LAW — entry ids strictly
  ascending, an alternative's holes strictly greater than its entry's
  id, one owner per hole. The law is what makes every pass total by
  plain structural folds: counting = one reverse fold (memo), deciding
  = one forward fold (live digits), closing = one reverse fold — no
  fuel, no cycle detection, no mutual SCC beyond the standard
  Expr/list/arms walkers. Exact bignum counts; rank-addressed unrank
  under the documented ADDRESSING LAW (distinct holes ascending, last
  = least-significant digit; alternatives = consecutive intervals in
  list order).
- **examples/sketch_pin.shard** — 14 kernel-computed claims: counts,
  unrank at interval edges, sharing-is-one-decision (root sharing and
  in-alternative sharing both pinned), the verbatim-fill contract, and
  the refusal family (stratification violation at wf AND at count,
  cross-entry ownership, malformed hole, out-of-range index).
- **tools/search** (the D2 clone): the ground engine — unrank → FnDef
  → inject into the loaded object module's fn list → battery through
  meta/invoke's invoke_fd (evm_call_pure; D3 honored, no
  engine-private evaluator) with early exit across tests. Injection
  rides the fact that evm_call_pure rebuilds its dispatch tables from
  the Module's fn LIST (the stale FnTrie field is never consulted on
  that path). A stuck / type-wrong candidate (the untyped-grammar
  regime) fails its test through the whole-or-nothing value decode —
  probed explicitly before the engine was built on it.
- **The rev task**, replicating the playground's measurement record
  against the real kernel: base hole leaf-only {acc, Nil, xs} at every
  depth, step hole {acc, h, t, Nil} + Cons/append over fresh
  depth-(d-1) sub-holes (grammar extracted from the playground source,
  read as data). Space 3·S(d), S(0)=4, S(d)=4+2·S(d-1)²: **COUNT 108 /
  SOLUTIONS 1 at d1 (0.6s), COUNT 7788 / SOLUTIONS 13 at d2 (15s)** —
  the published counts and solution-set sizes, exactly. d1's single
  solution index 8 hand-decodes to the textbook `(Cons h acc)`.

**Gates.** The 14 pin claims replay under bin/shard_check (kernel-
computed, G5's claim-layer half); the tool's COUNT/SOL/SOLUTIONS lines
are corpus-pinned in run_corpus.sh (the diff-tool half — an addressing
or grammar change moves them and fails the FAIL-set/output diff);
three new corpus targets (sketch_pin, rev_obj, search) check green;
corpus FAIL-set diff clean against fails-base.txt.

**Gotchas recorded.**

- Int-measured recursion (`(measure d)` / countdown loops) needs
  solver-generated descent sidecars: `bin/shard_eval run
  tools/prove/prove.shard FILE` writes the .auto.shard; struct
  measures need nothing.
- shardfmt is a stdout filter and REFORMATS multi-line ctor spines;
  format before hand-diffing anything.
- evm_call_pure rebuilds ixt/efftab/fntab per invocation. At d2 that
  is 4×7,788 rebuilds ≈ 15s wall — fine for the pin, and the obvious
  first engine economy when a rung is actually slow (D7's
  order-of-attack): a staged pure-invocation surface on meta/invoke
  ("prepare a module once, invoke many"). Not built; noted.

**Implementation-discovered decision points (user attention wanted).**

- **D9 (open) — rank, the inverse.** S1 says "rank/unrank"; slice 1
  ships count + unrank. candidate→index needs an alternative-matching
  discipline (top-level structural non-overlap of alternatives) to be
  well-defined; no consumer needs it yet. Proposal: defer to the
  slice that consumes it (dedup/citation), with the non-overlap check
  added to g_wf at that point.
- **D10 (open) — cross-entry hole sharing.** The correlation primitive
  is licensed at the sketch root and within an alternative (both
  pinned); a hole reachable from two DIFFERENT entries is refused
  (g_wf + a live-merge guard), because it breaks sum-of-products
  counting. The playground's skeleton-level sharing maps to root
  sharing, so nothing measured is lost at this rung; S4b's choices
  map will want the general form and should bring its own counting
  story when it arrives.
- **D11 (open) — solution rendering.** SOL pin lines carry candidate
  INDICES (deterministic under the addressing law), not source text.
  Rendering an arbitrary synthesized Expr (Match/Ctor/BVar) as source
  is exactly the S6-adjacent renderer meta/spell's canonical-expr
  subset does not cover; it should land once, with proof rendering,
  not as a one-off printer here.


### Slice 2 — dialect grammar + the G1/G2 census (LANDED 2026-07-11)

**What landed.**

- **The constraint set, fixed empirically.** Before any builder was
  written, a cn_e probe judged nine hand-picked candidates through the
  REAL recognizer (kernel/canon, consumed read-only). Every prediction
  confirmed, and one divergence from the playground surfaced: the real
  ledger is STRICTER than the playground's canon flag — C8 'respell
  bans the scrutinee var in a binder-less arm, so the dialect base
  hole is {acc, Nil} (the playground's canon grammar kept xs; its
  separate "dialect" flag is already IN the kernel ledger). The full
  set on this fragment: C8 'respell (base), C8 'rebuild — the exact
  Cons(h, t) point excluded at EVERY position, C7 nil_left / cons /
  assoc (append left = atoms only), C7 nil_right (append right ≠ Nil
  leaf).
- **meta/sketch grew sk_rank** (D9 resolved by its first consumer, the
  census): template-vs-candidate matching with holes as binders
  (sharing = expr_eq consistency), a forward match fold + a reverse
  digit-composition fold mirroring unrank's decide/close, exact bignum
  composition. Six new pin claims (20 total): round-trips at interval
  edges, rank under root sharing, non-member and share-break refusals.
- **THE RANK ORDERING DISCIPLINE** (found as a latent trap before it
  fired): sk_rank is first-structural-match with NO backtracking into
  later alternatives, so a point-exclusion split must list its
  CONCRETE-headed alternative (Cons(h, B')) BEFORE the hole-headed one
  (Cons(A', B)) — a hole binds anything structurally, and the general
  alternative would capture Cons(h, X) candidates and fail deeper.
  Documented in the module header; a backtracking rank stays unbuilt
  until a grammar needs it.
- **The dialect rev grammar** (tools/search/rev.shard): leaf-list
  kinds (full / minus-h / minus-t / minus-Nil) + the split Cons pair +
  atoms-only append-left, every variant differing from full by exactly
  one leaf. Predicted counts D(d) = 4 + (D(d-1)²−1) + 3·(D(d-1)−1):
  56 at d1, 1,736 at d2 — both confirmed exactly by the census.
- **tools/search/census.shard — the G1/G2 gate.** Pass A sweeps the
  dialect space: rank∘unrank = id per candidate (injectivity), cn_e =
  Nil (G1), rank-into-full + unrank-back (dialect ⊆ full), battery
  (solution census). Pass B sweeps the full space: every cn_e-clean
  candidate must rank into the dialect and unrank back expr_eq
  (clean ⊆ dialect). Counts printed and pinned. **Measured: FULL 108
  DIALECT 56 CLEAN 56 (d1, 0.7s); FULL 7788 DIALECT 1736 CLEAN 1736
  (d2, 12.8s); G1 OK, G2 OK, SOLUTIONS dialect 1 at both depths** —
  the 13 full d2 solutions collapse to exactly the textbook. Quotient
  exactness against the real ledger, censused term-by-term.

**Gates.** G1 + G2 censused at d1–d2 inside the corpus (the
three-speakers drift alarm: a kernel C-rule change moves the pinned
FULL/DIALECT/CLEAN lines); 20 sketch_pin claims kernel-replayed; five
corpus targets; corpus FAIL-set diff clean. G1's second half (the
tools/canon REWRITER fixpoint) needs source rendering and lands with
D11 — recorded as pending, not skipped silently.

**Gotchas recorded.**

- **The canon advisory judges the ROOT file's fns only.** Checked
  through a pin file, a meta module's own advisories are silent —
  check meta modules DIRECTLY during development or C-violations hide
  until someone roots them.
- The C4 house consequence: result types declare their Err ctor FIRST
  so guard-style Err-first matching is declaration order; Option
  matches go None-first; Expr walkers put Ctor before Call (kernel
  declaration order).
- Deep match towers hand-balanced: count frames, or lean on the
  reader's per-form missing-paren report (it names the fn and the
  deficit).

**Playground comparison, recorded once:** playground canon-flag rev =
87 / 2,787 (d1/d2); the real kernel dialect = 56 / 1,736 — the delta
is exactly the contextual C8 tier the playground priced as separate
"dialect" rules and CANON.md later ratified into the ledger. The
census flywheel's first in-repo turn agrees with §1's premise: the
real dialect is measurably tighter than the approximation.


### Slice 3 — the catalog census, S7-lite (LANDED 2026-07-11)

**What landed: tools/search/catalog.shard** — the first WHOLE-BODY
grammar (control flow inside the grammar) and the first
spellings-per-behavior numbers against the real dialect.

- **The fragment:** f : (List Int) -> (List Int) over match / Cons /
  Nil / append / structural recursion — no comparisons, arithmetic, or
  literals; matches at tail positions only; typed by construction
  (Cons's Int slot = pin heads; recursion only on destructured tails,
  so every candidate is total). THE FRAGMENT INVARIANT that keeps the
  builder simple: at most one matchable (unpinned) list var exists on
  any path — matching pins one, destructuring adds one, and Nil arms
  legitimately have zero. Budgets: rung d = match depth AND expr
  depth; matches spend match budget only (arms keep the expr budget);
  leaves and (f t) are atoms.
- **The two-tier method, made explicit.** The GENERATIVE tier carries
  the position-local rules (leaf respell filter, no f-calls on
  pinned-Nil binders, pin-head Int slots, atoms-only append-left,
  Nil-free append-right, unpinned scrutinees, decl-ordered exhaustive
  arms). The FILTERED tier is measured, not excluded: every generated
  candidate is judged by cn_e, and the flagged families are TALLIED
  per (rule, detail) — the generate → census → classify loop running
  against the real ledger. The content-shaped residue (exact rebuild
  points through shared sub-holes; C10m vacuous matches) is exactly
  what a product grammar cannot exclude without content machinery.
- **Sub-hole SHARING pays for itself:** Cons alternatives and
  append-right reference one shared full operand sub-grammar
  (in-entry sharing is licensed — alternatives are exclusive), which
  collapses what would be per-pin sub-grammar iteration (a mutual-SCC
  measure fight) into a SINGLE self-recursive builder on measure m+e.
- **Measured, corpus-pinned (G5):**
  - rung 1: **GEN 20 / CLEAN 17 / BEHAVIORS 13** (flagged: 2 vacuous,
    1 rebuild) — ties exactly to the playground's certified "19
    programs = exactly 13 functions" (their 19 = our 17 clean + the 2
    vacuous members their catalog kept);
  - rung 2: **GEN 3,395 / CLEAN 2,345 / BEHAVIORS 1,068** (flagged:
    1,039 rebuilds, 11 vacuous; 44s) — behaviors match the
    playground's 1,068 EXACTLY, rev = 2 spellings (their post-R1
    number), id = 4; spellings-per-behavior 2.20 (playground post-R1:
    2.21).
  - Rank/unrank round-trips verified per candidate across the whole
    sweep (the first multi-sub-hole, Match-shaped alternatives through
    sk_rank).

**The slice's discovery, fed back:** the kernel's C8 'respell
SUBSUMES the playground's mined R1 rule — (f t) under a pinned-Nil t
is a *mention* of the scrutinee var in a binder-less arm, so the whole
"no recursion on a pinned-Nil variable" family (8,016 of 13,428
generated members before the filter — 60%!) is priced position-locally
by the real ledger. What the playground learned by mining, the ledger
already knew; the census confirmed it by measurement.

**Gotchas recorded.**

- The measure solver cannot see through `(if …)` expressions in
  recursive-call ARGUMENTS — branch the CALL, not the argument, and
  the obligations become plain farkas arithmetic.
- Int-measured fns whose decrease needs parameter nonnegativity must
  GUARD it (`(if (lt m 0) (Err …) …)`) — loud dead code that hands the
  solver its hypotheses.
- Battery decode failures are treated as builder bugs (loud refusal),
  not test failures: this fragment is typed and total by construction,
  so a stuck candidate means the generator drifted.

**Open (rolls forward):** the rebuild family (1,039 at rung 2) is
position-local in principle — generative exclusion needs per-pin
sub-grammar variants, i.e. either a bounded unroll (pins ≤ rung) or a
variant mechanism in meta/sketch; deferred until the residue matters
at a deeper rung. D10/D11 unchanged. Behavior digests (D8 sha256)
enter with the sampling instruments, not the exact rungs — full
output vectors are the bucket keys here (exact, no collision caveat).


### Recorded 2026-07-11 — future arc: MODEL-FRAGMENT SEARCH (noted, not scoped)

Raised during slice-3 review: search *within a given ISA model* —
refine an existing high-level shard program into (e.g.) the wasm
model's instruction vocabulary, and a proven compiler falls out.
Feasibility was assessed against this arc's architecture. Nothing
here scopes that arc, but two of its design consequences bind THIS
arc's remaining slices, so they are pinned now:

- **S4a PIN — neutrals stay task-agnostic.** Canonical neutrals are
  keyed off the type table and qnames (as S4a already states);
  nothing in the symbolic evaluator may hardcode the rev/list
  vocabulary. Model-state neutrals (stack/locals/memory over symbolic
  words) must arrive as a rule-set instance, never a rewrite of the
  machine.
- **S5/D4 PIN — rule sets are DATA.** The oracle's rewrite table is a
  passed-in parameter (proven, typed equations per D4), not a
  baked-in list; "the append four + lia" is the first VALUE of that
  parameter. A model's defining-equation lemmas plus the word/memory
  laws are then a rule-set choice, not a code change.

The rest of the assessment, recorded for the future ledger. The
candidate space is nearly free: models are ordinary shard libraries
(ISA ruling), candidates are DATA — no injection, pass them to the
model's stepper — and the ground battery is the lowering statement
instanced at test points, spec(x) = DEC(model_run(c, ENC(x))), the
LOWERING §6ah ENC/DEC form. The oracle is S5 retargeted, not
redesigned. Granularity resolves to FRAGMENTS, not whole programs:
the source program's structure induces the sketch (one hole per
source form), search mines LOWERING.md-shaped fragments, and the
existing composition machinery (portable certs, the pw walk, the
five gates) assembles programs without caring who authored a
fragment. A cheaper tier sits above raw instruction search:
whitelist = the proven fragment COMBINATORS themselves, so search is
applicability search and the proof is assembled from certs; raw
instruction search fires only where no combinator path exists —
exactly where new fragments are worth mining. Trust posture
unchanged (S6/G4 verbatim). The whitelist question has a structural
answer already: a meta/sketch Grammar has NO ambient scope — every
alternative lists its heads explicitly; grammar builders ARE the
whitelist mechanism. One S7-full item is named by this note: the
SIGNATURE-DRIVEN GRAMMAR BUILDER — scope spec (qnames with types,
ctor whitelist, root type) → stratified typed grammar; cat_g is its
prototype with the ilist typing hardcoded.


### Posture ruling — this ledger is a LIVING document (USER, 2026-07-11)

Recorded after slice 4: **SEARCH.md is the initial suggestion, open to
mutation as the arc learns** — not a frozen contract. The unit of
value is not playground parity (the playground is a SPIKE: it shows
what is possible and what fits the language, not what must be
adopted). The unit of value is **components composable into engines,
plus first-party engines that earn their keep**, judiciated into the
repo at three admission tiers with three different bars:

- **kernel / CANON.md** — core-language mutations that make
  frontier-pushing search possible at all; highest bar,
  census-evidence-driven (the arc's first step, already taken);
- **meta/** — "stdlib for manipulating shard": the posture is EAGER;
  durable, surface-disciplined components graduate here (meta/sketch
  day one; extend_fn; the S4a value machine once a second consumer
  speaks it, per the hygiene-pass ruling);
- **tools/** — instruments, useful in any way, and the EASIEST tier
  to drop later; experiments land here first (D2's clone-first is a
  special case of this posture).

Program search as a capability has standing uses beyond this arc's
tasks, three named: **identifying redundancies in canon** (rule
subsumption as absence proofs over rungs — slice 3's C8⊃R1 discovery
was a hand-derived instance; the instrument version automates it),
**finding false equivalence proofs** (adversarial refutation of
CLAIMED theorems — the comparator and ground battery pointed at the
kernel's own proof outputs; a zerocase-class bug would light up
instantly), and **automating compilation of small arbitrary shard
functions** (the model-fragment note above). Mutations to this ledger
are recorded in §8 with reasoning; reversals of user rulings still go
to the user first.


### Interlude — extend_fn (LANDED 2026-07-11, user ruling)

Slice-review ruling: candidate injection must not lean on
evm_call_pure ignoring a stale FnTrie (an implementation property,
not a surface guarantee). **meta/invoke grew `extend_fn`** — fn list
and dispatch trie updated TOGETHER, None on a qname already bound —
and tools/search's inject/fns_snoc were deleted; all three engines
consume the surface fn with loud Err paths on collision. Pins
byte-identical, corpus DIFF-clean. Two other review rulings recorded:
the rank ordering discipline stands as simplest-first (a backtracking
ranker is the known fallback if a grammar ever needs it), and
generation-side filtering is STRICTLY PREFERRED wherever a rule is
expressible generatively (the slice-3 method is the norm, not an
optimization).


### Slice 4 — S4a + S5: the symbolic evaluator and the laws oracle (LANDED 2026-07-11)

**What landed.**

- **tools/search/sym.shard — the S4a machine** (27/0 as a corpus check
  target). Value domain SV: ground ints/symbols/ctors + SVSym
  (Many-class ∀-atom: distinct atoms refute), SVData (undecided ∀-data
  value, case-splittable through a typed slot), SVNeu (stuck call,
  same-head/equal-args congruence, never refutes). Strict (CBV)
  evaluation matching kernel/evm; dispatch = the module's own FnTrie
  (extend_fn's consistency contract earning its keep), then
  try_step_prim on ground args, then neutral formation. Case-split
  shapes come from the module's OWN type table (lookup_typedef +
  instantiated ctor fields, one shape per ctor in decl order, fresh
  symbols per field). Three-valued comparison: ground decides,
  Many-atoms refute by identity, disagreements SPLIT within a
  depth budget (all cases prove ⇒ Proven; any case refutes ⇒ Refuted;
  else Undecided), ctor fields refute injectively, neutral args only
  ever join. Split-resume is pure: substitute the shape into the
  value tree and RE-CALL every neutral through sv_call (idempotent
  when still stuck; re-normalizes through the rule table when
  unstuck).
- **The two future-arc pins, honored day one:** nothing in sym.shard
  knows a task vocabulary (type-table shapes, qname heads), and the
  rewrite table is passed-in DATA — `(NRAppend append Nil Cons)` over
  the OBJECT module's own qnames is v1's only rule family (the append
  four applied at neutral-formation time, right-nested Nil-free
  spines). The lia normalizer is a later VALUE of the same parameter
  (deliberately deferred: G3 measured it unnecessary at these rungs).
- **tools/search/laws.shard — the S5 oracle.** Laws come from the REAL
  interface through the kernel's single goal seam (read_file +
  parse_decls + parse_goal_r under file_rctx — the tools/prove
  precedent); premised goals refused (v1). The subject fn is remapped
  to the candidate; each goal side is wrapped as a synthesized FnDef
  (extend_fn) so stuckness always converts at a call boundary and
  resume re-enters through sv_call; ∀-binders instantiate by TYPE
  (atoms for Int/Symbol/tyvars — the ∀-tyvar-instantiates-at-Int
  refutation license — depth-0 slots for data). Verdicts compose over
  the law set: any Refuted refutes, all Proven proves, else Undecided.
- **Corpus pins (run output; laws.shard rides kernel/driver so, like
  tools/prove, it is NOT a check target — the known kernel/types
  tc_infer measure gap lives in that closure):**
  - **SELF-PIN:** std/list's own rev and len symbolically PROVE their
    own interface requirements (rev_nil/rev_cons, len_nil/len_cons) —
    the interface's implementation is the first candidate the oracle
    judges, and it needs zero splits (congruence + append canon).
  - **G3 rung 1:** CLEAN 17 / PROVEN 0 / REFUTED 17 / UNDECIDED 0 —
    rung 1 has no rev spelling (slice 3's catalog), and the oracle
    fully decided the rung. Sub-second.
  - **G3 rung 2:** CLEAN 2,345 / PROVEN 2 / REFUTED 2,343 /
    **UNDECIDED 0** — the two Proven candidates are EXACTLY the
    catalog's two rev spellings; the symbolic partition equals the
    ground-battery partition term-for-term with no undecided residue.
    19s. The impostor problem is dead at this rung: proofs, not
    tests, and nothing escaped either way. G3 violations (a Proven
    non-passer, a Refuted passer) are exit-1 failures inside the
    tool, not statistics.

**Decisions the implementation surfaced (recorded, not relitigated):**

- **Goals carry FVar binders.** parse_goal represents ∀-binder
  occurrences as (FVar name) — the sequent machinery opens them by
  name. The oracle lowers them to BVars under the wrapper fn
  (depth-aware walk: match arms and lets shift by their bind counts).
- **No cardinality analysis at all** (departure from the playground,
  licensed by kernel semantics): shard has no uninhabited types, so
  vacuity is gone; sym-vs-ctor and sym-vs-sym decide by splitting
  within budget; zero-ctor (opaque) typedefs simply cannot split and
  stay Undecided. The only cardinality fact used is a constant: Int
  and Symbol are Many.
- **Fuel discipline:** the evaluator and comparator are two separate
  SCCs, each carrying ONE uniform Int measure — every mutual edge
  passes fuel-1 behind a loud guard (the slice-3 mixed-measure wall
  dodged by design). Fuel-out is Undecided, NEVER Refuted. Engine
  parameters (budget 3, fuel 100k) are pinned task data in
  laws.shard.
- **D5 licenses deferred to their consumers** (catalog refinement /
  compiler goals) — G3 at these rungs measured them unnecessary:
  per-candidate rev/len laws close by congruence + append canon
  alone, exactly as the ledger's S4a note predicted ("one term, one
  goal, case-split regions only").

**Gotchas recorded.**

- kernel/driver (and anything importing kernel/checker) exposes the
  PRE-EXISTING kernel/types tc_infer measure gap under any new check
  root — keep driver-riding tools out of TARGETS (tools/prove
  precedent) and keep evaluator-tier tools off the checker import
  (sym.shard mirrors inst_ctor_fields locally rather than importing
  the checker closure).
- C1 prices `(- 0 1)` as a foldable ground call: spell negative
  literals `-1`.
- Deep match towers again: build one fn at a time against the
  reader's per-form paren report.

**Open (rolls forward):** lia canon (NRLia) enters as a rule VALUE
when a task's laws need stuck arithmetic joined beyond congruence
(len-shaped goals at deeper rungs); the D5 licenses land with S7
refinement; D10/D11 unchanged. Next per the ladder: **S6 — proof
rendering + kernel replay (G4)**, the arc's exit criterion; it will
grow TRACE RECORDING in this slice's machine (the comparator knows
its splits and joins; it does not yet write them down) and render
Proven verdicts as replayable shard proof text.


### Slice 5, component 1 — the source renderer (D11 RESOLVED, LANDED 2026-07-11)

First component landed under the living-ledger posture (S6
decomposed: renderer → trace recording → proof rendering, each
independently consumable).

**What landed: meta/spell grew the FULL Expr renderer** — the module
whose header always promised it ("canonical spelling … for programs
that generate shard"). sp_e/sp_arms/sp_pat/sp_ty/sp_fn over the whole
term vocabulary, Doc-based like the module's existing pr_e, flat
layout composed with meta/format's fmt for canonical bytes. The
contract, pinned in the header and in examples/spell_pin.shard
(12 kernel-computed claims):

- **binder naming law**: ordinal = env depth at introduction, so
  BVar j under n binders spells x(n-1-j) — the law xsym's header
  already stated, now load-bearing; pattern binds count left to
  right, parallel-let RHSs render at the outer depth;
- **heads resolve through a caller-supplied qname→symbol table** —
  the caller owns scope policy, a miss is a loud SpRErr, never a
  guessed spelling;
- nullary ctors bare, nullary calls parenthesized, negative literals
  literal (the C1 lesson).

**tools/search/gen/rev_synth.shard — the first synthesized programs
to live in the repo as ordinary source.** The two law-Proven rev
candidates (rank addresses 62 and 347) rendered as rev_c62/rev_c347
with (measure (struct x0)) clauses, corpus CHECK TARGET (the kernel
verifies the measures), cn_e-clean by construction. Gated by
tools/search/render_gate.shard both ways, corpus-pinned: **REGEN**
(re-render from the grammar → byte-identical to the committed
artifact — the sidecar discipline; `emit` mode re-pins deliberately)
and **RELOAD** (the artifact through the real reader/resolver →
bodies expr_eq to the unranked candidates, self-calls remapped).
Rank-addressed names carry provenance.

**Worth reading in the artifact:** rev_c347 is the needless-split
twin — `match x2 (Nil x0) …` where the Nil arm's x0 equals the erased
expression under the arm hypotheses. That is EXACTLY the playground's
R3 contextual-PE family, now sitting in our repo as checked source —
live evidence for the canon flywheel (cross-arc: CANON.md owns any
rule that would price it).

**Placement notes:** rm_e/rm_es/rm_arms moved from laws.shard to
rev.shard (shared candidate plumbing — the render gate remaps
self-calls with the same walker the oracle remaps subjects with).
render_gate is BOTH a check target (52/0 — no driver import) and a
run pin; the artifact is a check target.

**Open (rolls forward):** slice 5's remaining components — trace
recording in the comparator, then proof-text rendering (claims +
fulfills citing rev_nil/rev_cons over the rendered fns) and G4 kernel
replay. The renderer's qname policy renders BARE names only; explicit
(:: path name) head spelling is unneeded until a consumer emits into
a scope it does not control.


### Slice 5, component 2 — trace recording (LANDED 2026-07-11)

**What landed: the S4a comparator writes down its proof skeletons.**
sym.shard grew the trace vocabulary — `TrRefl` (the sides joined by
evaluation + congruence alone; renders as compute + refl), `TrSplit`
(slot id, slot type, one `TrCase` per ctor in declaration order, each
carrying the ctor and its fresh shape fields so the renderer can bind
case binders), `TrSeq` (independent sub-comparisons' traces in
comparison order — ctor fields, neutral args — nested by the
renderer). CpR/CaR carry the trace; a trace is meaningful for PROVEN
verdicts only (Refuted/Undecided paths return placeholders no
consumer may render). laws.shard's `LVdProv` now carries the law's
trace, and a `trace` mode prints pinned shape lines (part of the
default corpus suite).

**All eight shape predictions, committed before the run, confirmed
exactly:**

    TRACE std_rev  rev_nil  REFL        TRACE rev_c62  rev_nil  REFL
    TRACE std_rev  rev_cons REFL        TRACE rev_c62  rev_cons REFL
    TRACE std_len  len_nil  REFL        TRACE rev_c347 rev_nil  REFL
    TRACE std_len  len_cons REFL        TRACE rev_c347 rev_cons (SPLIT 0 (Nil REFL) (Cons REFL))

The needless-split twin needs exactly the case its own body
introduced, and nothing else in the pinned set needs any case at all —
the append canon fired only at formation, never load-bearing for a
join, so every REFL leaf renders as compute + refl with no lemma
citations. That is the measured basis for the render component's v1
scope: the split/compute/refl fragment, with G4 replay as the
tripwire that promotes lemma-citing leaves to a feature the moment a
task actually needs them.

**Design note (the linearization question, answered by construction):**
a comparison is a tree of independent sub-joins while a proof is one
nested sequence of case-ons — TrSeq records the sub-joins in
comparison order and defers the nesting to the renderer, which
sequences them innermost-last. For the pinned set every TrSeq
collapses (tr_seq drops refl members), so the question stays
theoretical until a richer task exercises it.


### Slice 5, component 3 — proof rendering + G4 replay (LANDED 2026-07-11)

**THE EXIT CRITERION'S SYNTHESIS HALF IS MET.** The generated artifact
(tools/search/gen/rev_synth.shard) now carries four machine-rendered
CLAIMS — rev_c62 and rev_c347 each proving rev_nil and rev_cons — and
**all four replay green through bin/shard_check**. Search output is a
shard program the kernel has checked, not a claim about one; and
because the artifact is a corpus check TARGET, G4 runs continuously,
on every sweep, forever.

**What landed in laws.shard (the emit/regen path; render_gate keeps
the driver-free RELOAD half):**

- **Claim rendering**: goals are the interface's OWN equations —
  binders monomorphized at Int, heads respelled through the resolver
  table (rev → the artifact fn), rendered by meta/spell. Proofs render
  from the component-2 traces: REFL leaves as (steps ((compute both))
  refl), splits as case-on (the proof DSL takes the bare type NAME;
  the checker's type gate re-derives the args), under-case leaves
  rewriting the case fact into both sides first.
- **THE REPLAY TWIN — the component's real discovery.** The oracle
  evaluates in the open RUN closure; replay is CHECK-side, where
  std/list's impl sits behind its module surface (the
  surface-discipline rule working as designed). rev_c347's Nil case
  measured it: the oracle joined by evaluation, but replay left
  `(append Nil (Cons h Nil))` unreduced. The renderer therefore loads
  a CHECK-MODE twin of the object closure (interfaces sealed, the
  candidate open — exactly replay's evaluation model), reduces each
  leaf's case-substituted goal sides with the kernel's own reducer,
  and emits THE SURFACE-DISCIPLINE TAIL — the interface's own
  defining-equation lemma (append_nil_left, cited via use scope) —
  exactly where the twin says evaluation alone will not close.
  Zero-site rewrites fail LOUDLY (measured — citation resolution is
  lazy, so the earlier "no-op" reading was wrong), which is why the
  tail is per-leaf, never uniform.
- **The D4 certificate categories, realized**: REFL leaves are
  definitional certificates; tail-bearing leaves are lemma-cited ones
  — and the CHOOSER is the twin, not a heuristic.

**v1 boundary, loud by construction:** TrSeq and nested splits refuse
at render; a leaf whose tail needs more than append_nil_left fails at
G4. Both are feature requests with a measured trigger, not silent
gaps.

**Open (rolls forward):** the exit criterion's second half — the
certified catalog BRACKET (the rung-1 "17 clean = exactly 13
functions" with S4a equivalence proofs + the D5 catalog license) —
remains; with it, S6 closes and the ladder's next rung is S4b.


### Slice 5, component 4 — the certified bracket (LANDED 2026-07-11): THE EXIT CRITERION IS MET

**gen/cat_bracket.shard replays 6/6 through bin/shard_check: 17 clean
rung-1 candidates = EXACTLY 13 functions, kernel-certified on every
corpus sweep.** Measured first (the throwaway probe): 4 multi-member
buckets; one pair (append xs xs vs its match twin) proves by a plain
split, the other three (recursive identity ≡ xs; the always-Nil
family) are the induction-hard family — and representatives must be
the MINIMAL bucket members (proofs close member → leaf; the reversed
direction leaves residue no budget closes).

**What landed:**

- **D5 license (ii) as a rule VALUE** (sym.shard `NRIh p q min`): a
  stuck call (p X) over a split-allocated slot (id ≥ min — goal
  binders sit below, split shapes above, so the subterm condition is
  an integer compare) evaluates as (q X). The rule-table-as-data pin
  pays again: the license is an entry, not a machine change, and no
  other oracle run sees it.
- **The equivalence mode is zero new machinery**: a synthesized law
  ∀xs. f xs = g xs (both candidates injected via extend_fn, the rep's
  self-calls remapped) drives the SAME law_verdict/trace/render path.
- **Induct rendering + the simp/compute discipline (two measured
  kernel-reducer facts):** check-side compute unfolds SAME-MODULE
  recursive calls one level into stuck-match residuals — destroying
  the (hyp ih) rewrite site — while simp's head-gate keeps stuck
  calls AS CALLS (the word-former gotcha, now load-bearing); but simp
  strands bare-body unfolds (the identity fn) that compute closes. So
  an ih leaf is exactly `(simp both) (rewrite (hyp ih) lr both true
  ()) (compute both)`, and the ih simulation in the leaf detector
  runs on the RAW substituted sides (a reduced residual buries
  p-sites under stuck matches the rendered simp never creates).
- **The bx_append twin**: the candidates' vocabulary cites std/list
  append — SEALED at check, so neither the floor's ground vectors nor
  eq leaves would reduce. The artifact carries bx_append (same body,
  same-module, open) with ONE bridging claim (bx_append ≡ append,
  induct over the interface's own defining equations); rendered
  candidates and the replay twin speak bx_append; the oracle keeps
  std append. The floor then closes as ONE computational claim:
  `(bd_uniq (bracket_vectors)) = True` — the kernel evaluates all 13
  representatives over the battery and checks pairwise distinctness
  itself.

**The certified statement, in claims:** bx_append_eq (the bridge) +
bracket_floor (distinctness ⇒ ≥13) + four ceiling claims (members ≡
minimal reps ⇒ ≤13), each rendered from oracle traces and none
hand-written. BRACKET REGEN pins byte-identical re-emission in the
laws suite.

**S6 CLOSES.** Both exit-criterion artifacts land as corpus-pinned,
kernel-checked source: a law-certified synthesis (rev_c62/rev_c347
proving std/list's own rev laws) and the certified d1-equivalent
bracket. The ladder's remaining rung is S4b (the superposed
executor); the playground's trie-factored/value-keyed findings are
recorded above as its likely design-of-record question.


### Rulings recorded 2026-07-11 (post-exit-criterion review, USER)

- **S4b builds AS RATIFIED** — the choices-map machine with the
  consulted-choice-set memo. The playground's value-blindness finding
  (zero memo hits on coupled tasks; the trie-factored fix) is scoped
  OUT of S4b's core: coupling factorization and trie-shared narrowing
  enter as a SEPARATE composition component with the first coupled
  task. "We are always free to build other engines later."
- **D10 RESOLVED — engine-side.** Cross-entry correlation lives in
  S4b's choices map; meta/sketch's grammar vocabulary stays
  exact-counting (the refusal is load-bearing for every census gate)
  and does not grow a correlated tier.
- **Sequencing:** S4b first; the FALSE-EQUIVALENCE-PROOF HUNTER
  after it; canon-redundancy and the joint task behind those.


### TODO — for CANON.md, post-merge (cross-arc evidence, USER-scheduled)

The CONTEXTUAL-PE rule family (the playground's R2/R3): under an
arm's hypotheses (x ≡ Nil inside the Nil arm), any subterm that can
take an evaluation step is a redex — canonical means contextually
normal. Definitional license; subsumes dead-var, decided-control,
and needless-split rules as special cases. Evidence for the
flywheel: (1) the playground measured 85% of its post-dialect d2
space as contextually-provable respelling, and its stack-d4 residual
fell 8 → 1 under R3; (2) THIS repo holds live exhibits —
gen/rev_synth.shard's rev_c347 is a needless-split twin committed as
kernel-checked source (its inner Nil arm's value equals the erased
expression under the arm hypotheses; the bracket PROVES it equal to
the textbook spelling), and the rung-2 catalog census tallies 1,039
rebuild-family members the rule family would price. Whether any of
this enters the ledger as C-rules is the canon arc's decision (canon
owns kernel/canon and CANON.md); the census machinery here is ready
to re-measure whatever it prices. To be taken up in MAIN once this
arc merges.


### Slice 6 — S4b, the superposed executor (LANDED 2026-07-11)

**The last ladder rung. tools/search/superpose.shard** — named
SUPERPOSE, the ledger's own word: "narrow" is the bootstrap/
inner-kernel shard DIALECT's name and stays free (USER, mid-slice).

**What landed: the choices-map machine, built AS RATIFIED.** One
evaluation runs all candidates at once: the sketch evaluates under a
partial assignment; an unassigned hole BLOCKS and the region forks
once per alternative; a failing test prunes everything consistent
with the partial assignment in one evaluation; a passing region's
unconsulted holes are don't-cares. Pure-functionally: a thunk arena
(binary trie keyed by node id, mod/ediv addressing) with CALL-BY-NEED
update (forcing overwrites with an indirection), LAZY evaluation
(args allocate as thunks; only scrutinee spines force — laziness is
what makes don't-cares real), holes as meta/sketch's own reserved
heads (the grammar IS the hole table — the D10 ruling realized:
correlation lives in the choices map, the vocabulary stays
exact-counting), and consistent-counting as a product over reachable
holes against the sk_count memo.

**Measured, corpus-pinned, settlement EXACT both depths:**

    SUPERPOSE rev DEPTH 1: TOTAL 108   FOUND 1  KILLED 107   REGIONS 26  FORKS 8
    SUPERPOSE rev DEPTH 2: TOTAL 7788  FOUND 13 KILLED 7775  REGIONS 443 FORKS 133

443 superposed evaluations settle what enumeration pays 7,788 for —
a 17.6× region reduction at d2 (the playground's leverage curve,
reproduced in kind). **AGREE extends G3 three ways** and any drift
exits 1 inside the tool: found coverage equals the enumerative
engine's solution count; every enumerative solution lies in a found
region (membership via the rank matcher against the region's
partially-filled template — match_e reused verbatim); every found
region's representative (don't-cares at alternative 0) passes the
kernel/evm battery.

**Design notes.** Call-by-need vs the kernel's call-by-value: for the
total, grammar-typed fragments searched here the results agree
(totality), and the AGREE gate polices it empirically — recorded,
not assumed. The consulted-choice-set MEMO (cross-region thunk
sharing, the ratified second half of S4b's core) is the next lever
INSIDE this component: v1 runs per-region arenas, correctness gated
first; the fork counts above are the baseline it will be measured
against.

**THE LADDER IS COMPLETE.** S1–S8 and every gate G1–G5 have landed
instances; the exit criterion was met in slice 5. What remains in
the arc's queue is by ruling, not ladder: the memo lever, then the
false-equivalence-proof hunter, with canon-redundancy and the joint
task behind them.


### Slice 6, component 2 — the consulted-choice-set memo (LANDED 2026-07-11)

**S4b's ratified second half.** The arena is now shared across every
region of a drive; thunks INTERN by (expr, env) so distinct regions
reach the same nodes; a completed forcing records which holes it
consulted at which choices, and any later region agreeing on exactly
that set replays the result without evaluating. A forcing that
consulted NOTHING is region-independent and updates to a plain
call-by-need indirection, paid once per drive (test inputs, closure
values). Blocked forcings record nothing (incomplete evaluations
never enter the memo).

**Measured against the pinned pre-memo baseline, settlement and
verdicts BIT-IDENTICAL (regions, forks, found, killed, AGREE):**

    d1: STEPS 896  -> 623    (1.4×)
    d2: STEPS 29,008 -> 12,651  (2.3×)

Re-evaluation halves at d2 and the leverage compounds with depth
(the playground's d3 settlements are where the memo pays 1,000×+;
d2's fork tree is shallow enough that most sharing is within-region).
**Recorded honestly: wall time ROSE ~30% at these depths** — the
pure-functional memo/intern probes (assoc walks) cost more per step
than the steps they save at d2 scale. That is exactly the "real
engineering item" the ledger named for the mutable-arena-shaped memo;
the two-level index and cheaper keys are the known follow-up when a
d3-scale consumer arrives, and STEPS (the algorithmic quantity) is
what the pin tracks.


### Slice 7 — the FALSE-EQUIVALENCE-PROOF HUNTER (LANDED 2026-07-12)

**The queue's next ruling: standing-use #2 built.** tools/search/
hunt.shard (+ hunt_obj.shard, the run-mode object closure of all 13
std modules) points the ground battery and the S4a comparator at the
kernel's OWN proof outputs: every requirement of the 13 std interface
files (the proven public surface) and every claim of the 14 in-closure
impl files (the internal lemma surface, where parlet-class bugs
historically lived) — 291 claimed theorems, swept end to end in ~77s.

**The machinery** (thin driver over existing components, as ruled):
goals parse through the kernel's single goal seam (parse_goal_r +
file_rctx per swept file — the tools/prove precedent); ∀-binders
enumerate over typed palettes (Int wide/narrow two-tier, Symbol,
tyvars at Int by the binder_sv license; plain declared data to
structural depth 2 via the type table, per-field products bounded
BEFORE they are built); REFINED types are refused (raw base
enumeration would not respect the predicate — std/str skips are the
mechanism working); premises FILTER vectors; each active vector's
sides run through the kernel's own reducer in the open run closure;
unpremised laws additionally pass the S4a comparator (rules table
empty). Verdict discipline: fuel-out/off-domain conclusions are
STUCK (never refuted), tolerated to a cap of 8 per law so
partial-domain laws still report reducible coverage; ground REFUTED
against symbolic PROVEN exits 1 (G3 — engine contradiction is a hard
failure); REFUTED lines are FINDINGS and the tool exits 0, per the
arc ruling (issues surfaced now are worked after merge).

**THE RESULT — pinned:**

    HUNT TOTAL LAWS 291 PASS 262 REF 0 SYMREF 0 VAC 0 STUCK 22
      SKIP 7 SYMP 117 SYMU 76 SYME 4 SYMN 94

**Zero refutations.** No false equivalence proof exists in the std
tree at this battery (up to 4,096 vectors per law, both palettes).
117 laws decided by BOTH engines with zero disagreements extends the
G3 record to the real interface surface. What the instrument DID
surface, recorded as queued questions rather than fixed mid-slice:

- **The S4a comparator's ctor-vs-atom refusal is reachable from real
  interfaces** (SYME 4: bytes blen_is_len / of_list_len, mem
  mapval_len / mapu8_len — length laws mixing a data-typed slot walk
  with atom arithmetic). CpRErr was designed as an invariant guard
  for the rev task; whether these configurations are legal (comparator
  incompleteness to extend) or the guard is right and the message
  wrong is an S4a question for the next consumer.
- **The 12 word shift laws are partial-domain theorems**: bshl/bshr/
  bsshr refuse to step at negative shift counts, so u8_shl_val-class
  goals hold by shared-subterm algebra with NO reduction route at
  k < 0. All 12 pass every reducible vector (PASS 10 each) and stick
  on exactly the negative-k rows — ground evaluation cannot decide a
  theorem there, reported honestly, not a soundness issue.
- **The sha256 class is out of ground reach by fuel policy** (STUCK
  at cap; its ground pins already replay as corpus compute claims).
  The engine follow-up is recorded in the tool header: move the
  battery onto evm_call_pure (the D3 substrate, fuel-free on total
  fns) once its per-invocation dispatch rebuild — the ledger's named
  first engine lever — is fixed. run_expr (small-step substitution,
  O(steps x term-size), no sharing) prices a 20k-step fuel-out on a
  sha-sized residual at MINUTES; fuel 2000 with the stuck cap prices
  the whole sweep at 77s. A first hunter draft SEGFAULTED the
  compiled engine by materializing H8's 8^8 field product before the
  battery cap looked at it — the per-field bound inside ge_fields is
  the fix, and "bound products before building them" is now enumerator
  law.

**Out of scope, recorded:** target-variant impl files (mem.wasm/
mem.x86/rng.wasm/str.wasm — their claims cite model closures this
object root does not carry, ~41 claims), the kernel/facts axiom set
(already differentially pinned by examples/facts_probe.shard), and
kernel/meta/models claims (the natural extension: per-tree object
roots, same driver). The corpus pin (run_corpus.sh) replays the full
sweep every run; any new REFUTED line changes the pinned output.
