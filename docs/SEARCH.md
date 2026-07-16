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
    vacuous members their catalog kept).  The exact sample gauge is
    **4 excess spellings / 3 collided buckets / 7 collided members /
    max bucket 3**;
  - rung 2: **GEN 3,395 / CLEAN 2,345 / BEHAVIORS 1,068** (flagged:
    1,039 rebuilds, 11 vacuous; 44s) — behaviors match the
    playground's 1,068 EXACTLY, rev = 2 spellings (their post-R1
    number), id = 4; spellings-per-behavior 2.20 (playground post-R1:
    2.21).  The exact sample gauge is **1,277 excess spellings / 596
    collided buckets / 1,873 collided members / max bucket 18**: under
    this battery, 54% of the clean address space is redundant beyond one
    representative per observed behavior.  This remains an observational
    quotient until laws/bracket proves individual bucket edges;
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
  baked-in list.  The append four are now the first checked value of that
  parameter: explicit root scope and driver Theory compile to `TrsProfile`,
  then enter the oracle as `NRTrs`. A model's defining-equation lemmas plus
  the word/memory laws are then a profile choice, not a code change.

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
  rewrite table is passed-in DATA.  The initial
  `(NRAppend append Nil Cons)` route applies the append four at
  neutral-formation time; the later `NRTrs TrsProfile` route consumes a
  generic ordered algebraic profile and interprets successful RHSs through
  the ordinary symbolic evaluator.  The laws driver now uses `NRTrs` from its
  checked root scope; `NRAppend` remains only as an explicit compatibility
  constructor for older callers.  The lia normalizer is a later VALUE of the
  same parameter (deliberately deferred: G3 measured it unnecessary at these
  rungs).
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
  Its canon profile now comes through `checked_profile.shard`: load the CHECK
  closure and sidecars, recover the root `RCtx`, join explicit names to
  `Theory`, and compile once for every self/G3/mine/artifact consumer.
- **Corpus pins (run output; laws.shard rides kernel/driver so, like
  tools/prove, it is NOT a check target — the known kernel/types
  tc_infer measure gap lives in that closure):**
  - **PROFILE-PIN:** `LAWS PROFILE APPEND 4 CHECKED GENERIC` precedes every
    mode, proving the oracle did not fall back to the compatibility rule.
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

**General task seam (LANDED 2026-07-15).**  The lazy evaluator was
already vocabulary-independent; only this first rev test adapter was
specialized.  `su_expect_expr` now compares arbitrary ground Shard values
lazily, and `su_drive_query` executes an arbitrary query containing the root
sketch while retaining exact consistent-subspace settlement.  The dynamic
`typed_superpose` adapter connects that seam to the reflected/routed `TeSpace`.
On the full mlx86 four-operation task it settles 1,728 candidates as 140
terminal regions and 63 demanded-choice forks, finding the same six ranks as
the exhaustive census.  The optional census audit agrees exactly.  No x86,
Wasm, imp, or task constructor occurs in the executor.

**Design notes.** Call-by-need vs the kernel's call-by-value: for the
total, grammar-typed fragments searched here the results agree
(totality), and the AGREE gate polices it empirically — recorded,
not assumed. The consulted-choice-set memo and shared arena are described in
component 2 below; region/fork counts remain the stable algorithmic baseline
independent of that evaluation-work optimization.

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


### Slice 8 — the CANON-SUBSUMPTION CENSUS (LANDED 2026-07-12)

**Standing-use #1 built: rule subsumption as absence proofs by
exhaustion.** tools/search/subsume.shard — the instrument version of
slice 3's hand-derived C8⊃R1 discovery. Every candidate of the arc's
two censused spaces (the rev FULL grammar at d1/d2, the structural
list catalog at rungs 1/2) is judged by cn_e (kernel/canon consumed,
never modified); flag sets deduplicate to ordered rule identities
(the CViol number+tag pair); the census tallies DISTINCT FLAG SETS
and every claim is arithmetic over that tally: RULE r TOTAL/UNIQUE
(UNIQUE 0 with TOTAL > 0 = locally redundant — deleting r changes no
verdict on the fragment, proven by exhausting it), and PAIR a COVERS
b (count(b ∧ ¬a) = 0 — subsumption witnessed across b's whole flag
count). Both a corpus CHECK target (33/0 — the import graph is
census/catalog's, no driver) and a run pin; the four fragments sweep
in ~10s.

**THE RESULT — pinned, and it is a clean negative:**

    rev-full d1: GEN 108  CLEAN 56   FLAGGED 52   SETS 9  (4 rules)
    rev-full d2: GEN 7788 CLEAN 1736 FLAGGED 6052 SETS 47 (6 rules)
    catalog r1:  GEN 20   CLEAN 17   FLAGGED 3    SETS 2  (2 rules)
    catalog r2:  GEN 3395 CLEAN 2345 FLAGGED 1050 SETS 3  (2 rules)

**No LOCALLY REDUNDANT rule and no COVERS pair on any fragment**:
every kernel rule that fires has unique witnesses at every rung —
candidates only it prices. The kernel ledger carries no internal
redundancy on these spaces; slice 3's C8⊃R1 was the kernel's rule
subsuming the PLAYGROUND's mined rule, and the intra-kernel analogue
does not exist here. Corroborations along the way: CLEAN counts match
the census/catalog pins exactly (56 / 1,736 / 17 / 2,345), and the
catalog-rung-2 tally decomposes slice 3's head-flag counts precisely
(1,039 rebuild + 11 match-only + 3 carrying BOTH = FLAGGED 1,050;
C10 match TOTAL 14 UNIQUE 11).

**Why a negative result is the right instrument output:** the census
re-measures on every corpus sweep. When the canon arc prices new
rules post-merge (the contextual-PE family is queued for exactly
that), a rule whose UNIQUE hits zero across fragments — or a COVERS
pair — changes the pinned lines and surfaces in the diff: admission
evidence, measured against the real ledger term-by-term. Evidence is
PER FRAGMENT by design (a rule redundant on list-shaped code may be
load-bearing elsewhere); the lines are input to the canon arc's
judgement, never deletions from here (canon owns kernel/canon and
CANON.md).


### Ruling + ARC CLOSE (2026-07-12, USER)

**The joint compile⊗exec task is SKIPPED — not adopted.** The
playground has not closed it: several attempts over hours, no
success yet. "No reason to adopt yet" (USER). It stays outside the
repo until the playground demonstrates a working instance; nothing
here blocks on it.

**The arc is complete and merge-ready.** Every ladder rung S1–S8 and
every gate G1–G5 has a landed, corpus-pinned instance; the exit
criterion (slice 5.4's certified bracket) is met; both queued
standing uses landed as instruments (slice 7 the false-equivalence
hunter, slice 8 the canon-subsumption census). What merges, by
accumulation tier:

- **meta/** — sketch (holes, count/unrank/rank — D9/D10 as ratified);
  spell grown to the full renderer (D11); invoke grown extend_fn (the
  interface ruling); rewrite grown typed ordered first-order profiles,
  structural application, and fuel-bounded normalization — durable "stdlib
  for manipulating shard".
- **tools/search/** — the instrument suite: search (ground pin),
  census (G1/G2), catalog (G5), sym (S4a), laws (S5 + traces + synth
  + bracket), render_gate (D11 reload), superpose (S4b + memo), hunt
  (standing-use #2), subsume (standing-use #1), rev/rev_obj/hunt_obj
  plumbing, gen/ artifacts (rev_synth, cat_bracket — both check
  targets, G4 continuous).
- **examples/** — sketch_pin, spell_pin (kernel-computed pins).
- **corpus** — 224 check targets green at the 57-line FAIL baseline
  (unchanged from cut); 8 search pin sections replay the whole
  instrument suite every run.

**The post-merge queue, gathered from the slice records:**

1. Contextual-PE evidence for CANON.md (USER-scheduled, in MAIN —
   see the TODO above; subsume.shard is the standing admission-
   evidence instrument for whatever the canon arc prices).
2. The S4a comparator's ctor-vs-atom refusal (4 live SYMERR witnesses
   pinned by the hunter) — extend or re-message, next S4a consumer.
3. Hunter engine: move the ground battery onto evm_call_pure once its
   per-invocation dispatch rebuild (the named first engine lever) is
   fixed; coverage extensions (per-tree object roots for kernel/meta/
   models, target-variant claims, refined-type filtered enumeration).
4. Superpose memo: two-level index when a d3-scale consumer arrives;
   trie/value-keyed coupling factorization with the first coupled
   task (separate component, as ruled).
5. Rebuild-family generative exclusion (per-pin sub-grammar variants)
   when the residue matters at a deeper rung; NRLia as a rule value
   with its first consumer task.
6. The joint compile⊗exec task — deferred per the ruling above.


## MODEL-FRAGMENT RESEARCH SPIKE (2026-07-14)

The new north star is automated refinement: for low-complexity shard
functions, search should routinely produce a small imp program and a
kernel-replayed wasm refinement.  The search focus must stay generic.  A
task may state its signature, vocabulary, observations, and proof contract;
it must not hand the engine a function-specific sketch that amounts to the
implementation.

### First component: typed imp expression search

`tools/search/imp_expr.shard` is the first model-fragment engine.  A task
module supplies only:

    search_local_kinds : () -> List IKind
    search_result_kind : () -> IKind
    search_constants   : () -> List Int
    search_ops         : () -> List IOp
    search_depth       : () -> Int
    search_probe       : IExp -> Option (List (Option Int))
    search_target      : () -> List (Option Int)
    search_witness     : () -> IExp

The engine constructs the same-kind straight-line `IBin` grammar from the
signature.  Candidate programs are imp values, not generated host code.
Every member passes `meta/sketch` count/unrank/rank round trips and the task's
combined well-kindedness + observation probe; `None` is a loud G1 failure,
never a filtered spelling.  Behavior keys are complete vectors.  The census
uses an exact lexicographic tree (no digest/collision qualification), reports
the lowest rank, and renders it through `meta/spell`.  The known-certified
witness is only a regression/non-emptiness gate at this rung: it must rank,
round-trip, and occur among the observational solutions.

Repeated calls use the new `meta/invoke/prepared.shard` surface.  It retains
the EVM name index, effect analysis, and translated function table and also
lets a caller hoist a fixed `FnDef` lookup.  This is a sibling opt-in layer,
not an expansion of `meta/invoke/mod.req.shard`: exposing EVM table types in
that interface creates a checker import cycle and widens the interface's
deliberately smaller trust floor.  The old one-shot API is unchanged.

### Measurements and the first better-than-existing refinement

The corpus-pinned `imp_add1` task is deliberately small and exact:

    depth 1; 52 candidates; 17 behaviors
    2 solutions; BEST 6 = (IBin U32 IAdd (ILoc 0) (IConst 1))

`gen/imp_add1_refinement.shard` ties rank 6 into the existing
spec ⊑ imp ⊑ wasm tower.  Its final wasm statement replays in the checker.

The exploratory `imp_mix` task is the first nested rung:

    depth 2; 19,205 candidates; 626 behaviors; 246 solutions
    old structured witness: rank 4,760
    BEST 6: (IBin U32 IAdd (ILoc 0) (ILoc 1))

This is not merely a shorter spelling of the same lowering.  `lg_mix x y =
2*x + (y-x)` is algebraically `x+y`.  The searched result therefore removes
the old implementation's intermediate wrap obligations.
`gen/imp_mix_refinement.shard` proves that rank-6 expression lowers to the
three-instruction wasm add and refines `lg_mix` with only the two final-result
fit premises; the existing structured imp theorem carries ten fit premises.
The algebra step was found by the ordinary proof solver and stored in a
machine-owned sidecar; the checker only replays it.

The depth-2 exact sweep is still expensive (about 97 seconds initially;
about 73 seconds after the exact-tree/prepared-context changes; combining
wf+observation into one probe left it around 71 seconds).  This negative
performance result is useful: neither top-level dispatch reconstruction nor
the linear behavior map is now the dominant cost.  Re-evaluating all 19,205
syntax trees over the full nine-point oracle is.

### General scope-to-grammar layer

`tools/search/typed_grammar.shard` removes the imp vocabulary from grammar
construction.  Its input is the kernel's loaded `Module`, one or more named
hole environments, the expected root type and binder environment, and a
depth bound.  An environment contains reflected heads, typed atoms, and
optional typed template rules; the common path populates its heads from a
resolved `RCtx`.  Constructor, function, and extern heads are reflected from
their real kernel signatures.  Matching a result type scheme against the
hole type determines the parameter types; unresolved child-only polymorphism
is refused rather than guessed.  The output is an ordinary exact
`meta/sketch` grammar, so count/rank/unrank remain shared infrastructure.

There are three intentionally separate policy levels:

1. **Availability:** the root task's explicit bare-item `use` scope selects
   constructor/call heads.  A merged module closure is not a scope: it also
   contains transitive implementation details.  Module aliases remain usable
   by the oracle but add no candidate productions.
2. **Hole admissibility:** expected `Type`, innermost-first BVar types, atom
   pools, enabled structural forms, and depth decide what can fill each hole.
3. **Semantic domain:** `search_probe : Candidate -> Option Observation`
   decides which typed programs are meaningful.  `None` is rejection; any
   Shard value can be the opaque observation key.

`TgRule` is the non-ISA escape hatch.  It contains a result type scheme, an
arbitrary kernel `Expr` template with local sketch holes, and a `TgSlot` for
each hole.  A slot supplies its expected type scheme and any binder types to
prepend while filling that hole; the rule also states its depth cost.  Thus
`If`, `Let`, `Match` arms, native primitives, and domain-specific binding
forms can use one mechanism.  The convenience `if` switch is itself lowered
to a `TgRule`, not special-cased in the work-list builder.  Recursive rules
must consume depth, template holes and slots must agree exactly, and all
result-determined type variables are substituted into child/binder types.

Type is not the whole hole property.  `ILoc 0` and `IConst 0` both contain an
`Int`, but their useful literal domains differ.  The advanced `tg_build_env`
surface therefore takes named `TgZone`s, each with its own reflected heads,
atoms, and rules.  Ordinary `TgSlot` children inherit their zone;
`TgSlotIn` routes a child to another named zone while retaining its true
kernel type and binder environment.  A `TgRoute` does the same for an
argument of an ordinary reflected constructor/call head, keyed by the full
head QName and zero-based argument index.  The old `tg_build` call is a
single-zone convenience wrapper.  This gives field roles, fixed structural
skeletons, lvalue/rvalue contexts, pattern-only syntax, or restricted operand
classes a common mechanism without inventing nominal pseudo-types or ISA
cases in the engine.

Dynamic tasks expose the same facility as a `search_environment : () ->
TgScopeEnv`.  The transport deliberately contains local symbols rather than
fabricated QNames: `typed_expr` resolves every `TgScopeCtor`/`TgScopeCall`
against the task's explicit bare-item `use` scope, verifies that it denotes a
real constructor/function in the loaded module, and only then builds the
internal typed environment.  Named routes are validated for existing zones,
heads, argument indices, and duplicate keys before grammar construction.
Consequently changing from x86 to Wasm—or to an application ADT—changes the
task context and environment value, not an engine-side ISA table.

A task may now also expose the optional, explicit
`search_canon_profile : () -> List Symbol`.  These are not trusted rewrite
names and imports do not grant them automatically.  The dynamic runner loads
the task's CHECK closure, resolves the ordered names through that root file's
real `RCtx`, admits only proven claims or granted requirements, compiles their
typed equations through `meta/rewrite`, and partitions them without loss.  The
exact separable fragment shapes `TeSpace`; valid deep or conjunctive rules are
retained as residual search constraints.  Failure at scope, evidence, or
profile validation is fatal; absence of the function means an empty profile.

`tools/search/typed_rule_probe.shard` is the first binder regression.  A
polymorphic `Let` template adds an `Int` BVar only to its body hole.  With
root/body atoms `{0,1}`, an RHS routed to the `{0}` literal zone, and depth one
it has exactly five members; the witness
`(Let (0) (BVar 0))` is rank 4 and round-trips.  A zero-cost recursive rule is
rejected before construction, as is any slot routed to a missing zone.

`tools/search/typed_expr.shard` is the exhaustive dynamic census consumer.  It
infers Candidate and opaque Observation from the task's protocol, derives
heads from the root file's actual use scope, and independently runs every
closed candidate through `kernel/types.tc_infer` before observation.  A task
may additionally provide `search_screen : Candidate -> Bool`; this typed,
optional **census accelerator** runs before the expensive opaque probe and is
counted separately from semantic-domain rejection.  The selected witness
still must pass the screen, match the complete target observation,
round-trip through rank/unrank, and occur in the final solution set.

`tools/search/typed_superpose.shard` consumes exactly the same first-class
`TeSpace`—the same reflected scope, routes, root sketch, grammar, and rank
space—but uses S4b SUPERPOSE as the search algorithm.  It evaluates
`search_probe(root-sketch)` under a partial assignment, forks only at the
first demanded open hole, and compares the resulting arbitrary Shard value
to `Some(search_target())` lazily from left to right.  A mismatch kills the
whole consistent subspace; holes untouched by a passing query remain
don't-cares.  Singleton grammar entries are transparent in evaluation,
consistent-subspace counting, and region templates, so fixed routed
skeletons do not manufacture one-way branches or hide descendant choices.
Residual checked reductions are classified over that same partial assignment.
A definite redex kills its whole consistent subspace; a blocked constraint is
allowed to wait while the semantic query tries to reject a still larger
region, and demands its own hole only after the query passes.

Every run requires exact `FOUND + KILLED = TOTAL`, validates one eager,
kernel-typed representative per passing region against the full probe, and
requires the task witness to belong to a passing region.  An explicit
`audit` argument adds `typed_expr`'s complete sweep afterward and proves that
every enumerative solution is region-covered and that the two solution
counts agree.  `search_screen` is used only inside that optional audit; it is
never consulted by the branch-and-prune drive.  On the small generic imp task
the audit records:

    total 114; found 2; killed 112; regions 80; forks 59
    exhaustive agreement: accepted 52; rejected 62; solutions 2
    BEST/WITNESS 10

### Pure Shard function-body benchmarks

`tools/search/pure_program.shard` opens the same lazy engine to ordinary
recursive Shard definitions.  A `PsTask` supplies a candidate QName and
signature, a kernel `Expr` body containing grammar holes, and one closed
observation query.  Recursive calls to the synthetic QName re-enter the body
inside SUPERPOSE, so the query can force just enough of a candidate to reject
an entire consistent subtree.  The protocol is independent of any ISA and is
also more general than the closed-value dynamic task protocol: the searched
artifact is a function body with parameters, binders, matches, and structural
recursion.

Passing regions have a deliberately independent backstop.  The runner fills
one representative, injects its `FnDef` into the real loaded object module,
checks the body against the declared parameter/result types with
`kernel/types`, and repeats the whole closed query through `meta/invoke`.
Untyped grammars may construct malformed intermediate data; if a ground call
then has no reduction (for example `le(Int, Nil)`), SUPERPOSE rejects that
candidate region instead of treating the stuck program as an engine fault.
The type/invocation gate prevents such a region from ever being accepted.

`tools/search/pure_tasks.shard` now ports three exact playground questions,
retaining their duplicate-rich full grammars rather than baking in the later
dialects.  `pure_bench.shard` pins the complete counts and known witnesses:

    insertion sort d0: total 9,072; found 0; regions 157; steps 1,672
    insertion sort d1: total 1,533,168; found 8; regions 1,517; steps 17,167
    sorted-list merge d1: total 5,263,380; found 4; regions 1,562; steps 30,070
    PExpr evaluator d1: total 10,077,696; found 200; regions 1,792; steps 11,314

The depth-0 insertion result is an exact absence certificate over its grammar.
Merge adds nested pattern matching and two different structurally decreasing
recursive calls; the evaluator adds tree recursion and arbitrary ADT
constructors.  Together with the existing `rev` task, these provide a pure
Shard progression on which theorem-backed formation pressure, stronger test
batteries, and proof-producing refinement can be developed without an ISA
encoding obscuring the result.

#### Checked formation pressure on supplied grammars

`meta/search` now gives already-supplied program grammars the same
quotient-first path that reflected grammar builders use.  The general
`ms_filter_formation(root, grammar, formation)` pass walks arbitrary static
`Match`/`Let`/`If`/constructor/call skeletons, intersects each reachable hole
with the separable root/argument constraints derived by `meta/rewrite`, and
returns another ordinary `Expr` + `Grammar`.  It neither knows nor encodes
append, lists, or the benchmark dialect.

Hole identity remains semantic.  Repeated occurrences under the same
formation state stay one shared choice; if one hole is demanded under two
incompatible formation states, v1 refuses loudly instead of cloning away the
correlation or unsafely unioning constraints.  After filtering, `g_wf` and
the exact grammar counter run again.  Non-separable checked rules remain in
the residual `MsPlan` and can still prune partial regions during SUPERPOSE.

`tools/search/pure_deep.shard` exercises the complete path:

1. load four append requirements from the checked object-module closure;
2. compile and partition the profile into formation and residual pieces;
3. apply formation to the full, task-supplied insertion-sort grammar;
4. run lazy semantic settlement over only the quotient; and
5. independently type and invoke one representative per passing region.

The full depth-2 result reproduces the playground quotient and solution floor
without an append-specific task grammar:

    raw 37,347,981,552; removed 32,878,101,552; quotient 4,469,880,000
    found 32; killed 4,469,879,968; regions 28,721; forks 5,969
    Shard evaluator steps 165,629; playground evaluator steps 38,994

Thus theorem formation removes 88.0% of the spellings and retains exact
coverage and the same 32 solutions.  The equal census does **not** imply equal
search work, however.  A counter-for-counter replay of the Rust playground
shows 4,745 splits and 22,841 prunes at d2, versus Shard's 5,969 splits and
28,689 prunes.  Rust also records 38,994 of its coarser evaluator steps,
290,231 memo hits, and 10,575 graph nodes.  Shard records 165,629 syntax-level
steps.  Treating that difference as merely a 4.25x interpreter constant was
wrong: the step definitions differ, and the 25% larger decision tree is an
algorithmic delta before host execution speed enters the comparison.

Depth 3 validates the harder scale without making a long interpreter run a
default gate.  The checked profile maps 22,140,821,944,106,047,728 raw terms
to exactly 104,277,392,481,024,192 canonical terms.  A 5,000-job probe settles
26,612,572,263,529,286 of them (25.5%) in 4,138 terminal regions, 862 forks,
and 38,420 evaluator steps.  Budget exhaustion is a first-class partial
census, not an error: `SETTLED + PENDING = TOTAL`, and every settled region is
still counted exactly.

For scale, the playground settles that entire d3 quotient in 349,732
evaluator steps: 83,553 splits, 406,365 prunes, 5,555,097 memo hits, and
28,800 graph nodes (10.7 seconds in the measured release run).  The Shard
probe therefore must not be presented as if extrapolation differed only by a
linear Shard-versus-Rust execution factor.  Three gaps are now tracked
separately: formation-equivalent grammars still induce a different deeper
fork tree; Shard routes fewer stable subcomputations through graph nodes; and
its hot runtime keys are structural Shard values rather than compiled numeric
identities.

    bin/shard_eval run tools/search/pure_deep.shard               # full d2
    bin/shard_eval run tools/search/pure_deep.shard 3             # full d3
    bin/shard_eval run tools/search/pure_deep.shard probe 3 5000  # bounded d3

The shared executor tables now move toward their intended asymptotics:
node-result and `(expression, environment)` indexes are persistent tries, and
result rows sharing one consulted-hole signature are grouped and indexed by
their exact choice key.  Hashes select buckets only; full environment/choice
equality remains authoritative.  In addition, each drive compiles the stable
`Grammar` once into `SuG`, an indexed operational scope used for demanded-hole
lookup while retaining the original grammar for proof, constraints, and exact
counting.  On the identical d3/5,000-job probe this reduced measured wall time
from about 26 seconds to 23.9 seconds without changing any region, fork, or
step count.

This is a local execution improvement, not the next search-research rung.
Retrofitting every Rust thunk boundary onto the present tree-valued evaluator
cuts abstract step counts but loses wall time to structural expression
interning and persistent memo-key construction.  A compiled expression graph
therefore remains part of Shard's general compilation/optimization path, not
a prerequisite for categorical search scale.

The checked append-canonical rev run isolates that distinction.  It starts
from `rev_grammar`, authenticates the same four requirements as the sort run,
and uses the generic supplied-grammar formation pass—never the hand-written
`dial_grammar`:

    d3: RAW 40,435,308; TOTAL 2,597,487
        FOUND 1; REGIONS 390; FORKS 143; STEPS 3,969
    d4: RAW 1,090,009,422,036,588; TOTAL 2,248,987,364,187
        FOUND 1; REGIONS 1,726; FORKS 639; STEPS 14,207

Both spaces and the unique textbook witness reproduce the playground's
append-canonical rows.  More importantly, d4 uses exactly the playground's
published 639 demanded-choice forks.  Its 14,207 Shard syntax steps and the
playground's 4,095 evaluator steps are different accounting/hosting constants
around the same decision structure, not an algorithmic-scale gap.  The run
completes in roughly five seconds on the current compiled evaluator.  This
corrects the earlier inference from the incomplete sort d3 probe: the lazy
executor and separable theorem quotient already compose at the expected
trillion-candidate scale.  The missing categorical work begins where the
playground adds contextual, sibling-relational, vocabulary, and algebraic
formation pressure beyond those four append rules.

    bin/shard_eval run tools/search/rev_deep.shard 3
    bin/shard_eval run tools/search/rev_deep.shard 4

The next missing category is now represented in `meta/search`, rather than in
a task dialect.  `ms_filter_match_context` reads arbitrary `Match`/`Pat`
structure and turns constructor-arm definitional equations into a second exact
grammar quotient.  A nullary arm excludes the outer scrutinee spelling; a
binderful arm excludes the exact constructor rebuild throughout the generated
arm subtree.  Correlated siblings are subtracted as disjoint products—for
example, removing `(h,t)` from `A x B` becomes
`{h} x (B-{t}) | (A-{h}) x B`—so no valid program is lost through an unsound
pair of independent exclusions.  Constructor names, binder counts, and
rebuild expressions all come from the supplied sketch; none is built into the
search module.

The generated grammar was checked against rev's old hand-written
`dial_grammar` oracle:

    depth 1: 56       candidates, exact member/rank audit
    depth 2: 1,736    candidates, exact member/rank audit
    depth 3: 1,512,056 candidates, exact count

It also composes with SUPERPOSE at the scale rung:

    rev d3 contextual: TOTAL 1,512,056
        FOUND 1; REGIONS 414; FORKS 143; STEPS 4,509
    rev d4 contextual: TOTAL 1,143,161,209,736
        FOUND 1; REGIONS 1,886; FORKS 639; STEPS 15,807

The unchanged fork counts are expected here: append formation had already
made the rev witness unique at every demanded choice.  The stronger evidence
comes from applying the same generic pass to the independent insertion-sort
grammar.  The complete depth-2 run changes:

    append quotient:  TOTAL 4,469,880,000; FOUND 32
                      REGIONS 28,721; FORKS 5,969; STEPS 165,629
    plus arm context: TOTAL 1,844,522,064; FOUND 8
                      REGIONS 14,249; FORKS 2,985; STEPS 91,102

Thus one scope-derived quotient removes four equivalent solution spellings
per behavior and nearly halves the actual decision tree.  At depth 3 its exact
space is 40,589,595,233,432,784 candidates.  A 5,000-job contextual probe
settles 17,878,750,522,262,628 of them (44.0%) in 4,131 regions, 869 forks,
and 39,223 evaluator steps, versus 25.5% of the append-only quotient for
similar work.  This is the intended additive path: reflected scope facts
compile to the existing `Grammar`; exact counting, rank/unrank, residual
theorems, and lazy evaluation need no task-specific executor branch.

V1 keeps two boundaries loud.  A grammar hole shared under incompatible arm
contexts is refused, and an exclusion that can start at a proper hole-bearing
subtemplate inside a production is refused rather than silently
under-filtered.  The latter needs the full state product at every static node;
`context_formation_probe.shard` pins that refusal.  A fixed nested excluded
value simply removes its entire production.  Ordinary head-plus-hole
constructor products, including correlated sibling exclusions, are exact.

    bin/shard_eval run tools/search/context_formation_probe.shard
    bin/shard_eval run tools/search/rev_deep.shard context 4
    bin/shard_eval run tools/search/pure_deep.shard context 2
    bin/shard_eval run tools/search/pure_deep.shard context-probe 3 5000

Comparison basis pressure now enters through the same checked-profile path.
`std/order` proves the polymorphic involution

    if (le a b) x y = if (lt b a) y x

by splitting the first comparison and discharging the complementary `lt` pin
with LIA.  The pure-program object imports that theorem, and the search driver
adds its name to an explicit five-rule profile.  Although the equation spans
an `If`, its lhs branches are unconstrained parameters: formation therefore
projects it exactly to “an `If` condition may not have root `le`.”  Existing
parent/argument formation removes those alternatives before ranking; no
sort-specific condition table or new executor case is involved.

The composition has exact counts:

    append profile only:               d2 4,469,880,000
                                        d3 104,277,392,481,024,192
    plus checked order involution:      d2 2,979,920,000
                                        d3 69,518,261,654,016,128
    plus match context and involution:  d2 1,229,681,376
                                        d3 27,059,730,155,621,856

The complete depth-2 order run leaves four spellings and settles the space in
7,407 regions, 1,554 forks, and 53,701 steps.  The context-only run needed
14,249 regions, 2,985 forks, and 91,102 steps for eight spellings.  Thus a
single general theorem halves the remaining solution gauge and nearly halves
the decision tree again.  A depth-3 5,000-job probe settles
12,080,236,917,486,516 of 27,059,730,155,621,856 candidates in 4,131 regions,
869 forks, and 39,284 steps.

Formation may remove the spelling supplied as a task's certification witness.
`ps_normalize_witness` now rewrites that witness through the same authenticated
profile first, and every formation stage immediately requires the resulting
witness to rank in its exact grammar.  The order experiment exercises this:
the original textbook `le` witness becomes the equivalent swapped-branch `lt`
representative before the grammar gate.

Repeated-variable pressure now has its first general implementation.
`TrsRule` admits nonlinear algebraic LHSs with ordinary first-order semantics:
the first occurrence binds a term and later occurrences require structural
equality.  Concrete normalization already had that equality check; the lazy
constraint engine now carries the same binding environment over partial
grammar terms.  Shared hole syntax proves equality immediately, distinct open
holes block on an exact choice, and assigned equal/unequal regions become
Redex/Clear.  Nonlinear rules never enter the old separable projection (which
would unsoundly turn `lt x x` into “ban every lt”); lossless partition retains
them as residual rules.  Context-free prepared match facts are skipped for
this tier, while stable whole-domain verdicts remain available.

`std/order` supplies checked `lt a a = False` and `int_eq a a = True` claims.
They are selected by name from the same object closure as the append and order
laws—there is no comparison-specific table in the engine.  On the complete
context+order depth-2 sort run, the two residual rules reject 87,834,384 of
1,229,681,376 programs (7.14%) and reduce the decision tree from 7,407 regions,
1,554 forks, and 53,701 steps to 6,851 regions, 1,438 forks, and 50,450 steps;
the same four semantic representatives remain.

At depth 3 the 5,000-job probe attributes 483,209,467,064,676 candidates to
the two constraints, but frontier coverage is essentially unchanged and costs
about 6% more steps: the diagonal equality relation still has to split both
independent operand holes.  That is useful diagnosis rather than a disguised
win.  The next categorical improvement is an exact correlated grammar product
that removes repeated-variable diagonals before ordinary search ranking, or a
relation-aware branch schedule; stable symmetric operand orientation still
needs a reviewed syntax order.  Removing `int_eq` wholesale remains a task
vocabulary choice unless an in-budget equivalent is proved representable.

    bin/shard_eval run tools/search/pure_deep.shard order 2
    bin/shard_eval run tools/search/pure_deep.shard order-probe 3 5000
    bin/shard_eval run tools/search/pure_deep.shard nonlinear 2
    bin/shard_eval run tools/search/pure_deep.shard nonlinear-probe 3 5000

The first dynamic theorem-filtered task searches ordinary closed Shard list
expressions over `Nil`, `Cons`, bit literals, and the real `std/list append` at
depth five.  Its four selected append requirements are authenticated from the
task scope and remove theorem-redex spellings before rank construction:

    CANON RULES 4
    RAW 210,066,388,900; REMOVED 210,066,388,837; TOTAL 63
    FOUND 1; KILLED 62; REGIONS 6; FORKS 8; STEPS 72
    BEST/WITNESS 17 = [0, 1]

The optional exhaustive audit visits only the 63-member quotient and agrees
on its unique solution.  This is the intended composition: theorem-backed
formation pressure first, then lazy semantic narrowing—not a 210-billion-term
filtering sweep.

The generic imp task deliberately admits all `Int` atoms at both `ILoc` and
`IConst`; the imp kind checker supplies the semantic distinction.  Its
exhaustive behavior census remains:

    typed_imp_add1: depth 2; generated 114; accepted 52; rejected 62
    17 behaviors; 2 solutions
    BEST 10 = (IBin U32 IAdd (ILoc 0) (IConst 1))

The unchanged engine also searches a different model and a parametric data
shape.  `typed_wasm_add1.shard` exposes generic `Nil`/`Cons` plus only
`LocalGet`, `I32Const`, `I32Bin`, and `BAdd`.  Reflection instantiates
`List a` at `List Instr`; depth four is all zero-to-three-instruction bodies:

    typed_wasm_add1: depth 4; generated/accepted 156; 7 behaviors
    2 solutions
    BEST 25 = [LocalGet 0, I32Const 1, I32Bin BAdd]

Wasm traps remain inner `Option` observation cells in this task rather than
engine-level rejections.  Changing that policy would change only the probe.
This cross-model result is the important scope test: neither general search
component contains an imp or Wasm name.

The same engine now reproduces the calculator search from
`~/workspace/mlx86`, this time over the real Shard x86_64 model.  Inspection of
the historical source matters: despite the calculator name and a four-arm
switch, its checked-in `op` was fixed to zero.  The actual benchmark was
therefore byte addition over forty deterministic `(a,b)` pairs.  A 512-byte
x86 genome ran with zeroed registers against bytes `a`, `b`, `op`, and `o` at
addresses 0 through 3.

`typed_x86_calculator.shard` transcribes the old LCG and all forty addition
pairs, while moving the search boundary to the current model's ordinary SysV
entry registers.  Its `TgScopeEnv` routes an exact two-cell instruction list
and operand roles through the bare-item x86 scope; the semantic choices remain
the move/binop/register/operator heads.  No x86 name was added to
`typed_expr` or `typed_grammar`:

    depth 4; total 729; found 8; killed 721
    197 terminal regions; 109 demanded-choice forks
    BEST 38 = [XMovRR RAX RDI, XBin XAdd RAX (SReg RSI)]

The earlier flat scope generated 7,318 trees and rejected 6,561 merely to
recover the 757 length-at-most-two programs.  Routing expresses the intended
structural domain directly: no ragged tail is generated, and the full 729 are
typed and observable.  The eight solutions are useful census evidence in
miniature: move-vs-add-from-zero and whether the sum is first accumulated in
`RAX`, `RDI`, or `RSI` are gauge spellings of the same behavior.  The minimum
rank chooses the expected `mov rax,rdi; add rax,rsi` representative.

`gen/x86_calculator_refinement.shard` is the G4 half.  It proves the searched
body computes `wrap64(a+b)` for arbitrary integers and separately replays the
old ABI as a six-instruction load/add/store program from a zero register file,
with input bytes at addresses 0/1 and output at 3.  The old encoding had
absolute 32-bit addresses; the current x86_64 model deliberately has
register-indirect addressing, so the faithful witness materializes pointers 1
and 3.  The memory proof closes through `std/mem`'s public framing theorems,
not its representation.

`typed_x86_calculator4.shard` then restores the switch that was present in the
mlx86 source but disabled at the sample site: opcode 0/1/2/3 selects unsigned
byte add/sub/mul/div over the same forty deterministic rows.  Its environment
is an actual structured x86 program—nested `XBlock`/`XBrIf`/`XBr` control,
register moves, `XBin`, and `XDivU`—with only three selector literals and
three arithmetic heads left as semantic holes.  This is environment
composition, not an encoded calculator production set:

    depth 40; holes 90; total 1,728 = 4^3 * 3^3
    found 6; killed 1,722; 140 terminal regions; 63 demanded-choice forks
    6 solutions; BEST/WITNESS 183; exact settlement and eager gates OK

The actual search narrows directly against all forty rows and never calls
`search_screen`.  A separate `audit` run then reproduced the old census
exactly—1,722 screened, six accepted, zero rejected—and proved its six ranks
are covered by the passing regions.  The four high-information historical
rows remain useful only as that enumerative audit's accelerator.  The six
solutions are precisely the `3!` gauge symmetry of permuting the add/sub/mul
tests while keeping division as the default.
`gen/x86_calculator4_refinement.shard` fixes the rank-183 instruction tree,
proves its add/sub/mul arms for arbitrary integers, replays the guarded
division arm, proves identity with the task witness, and kernel-checks the
complete forty-row historical contract.

The candidate need not be an ADT program.  `typed_shard_call.shard` exposes
the ordinary Shard function `lg_add1`, `True`/`False`, and the generic `If`
rule.  Its 24 closed expressions execute normally through the probe; the
unique target is `BEST 5 = (lg_add1 2)`.  Such a witness cannot be returned as
an ordinary `Int`-typed expression without reducing to `3`, so the protocol
also accepts `search_witness_rank : () -> Int`.  Rank/unrank reconstructs and
checks the syntax before execution.  ADT-language tasks retain the more
readable `search_witness : () -> Candidate` form.

The harder reflected `typed_imp_mix` census reaches the same kind-valid space
as the specialized task, but exposes the cost of a general syntactic domain:

    depth 3; generated 38,994; accepted 19,205; rejected 19,789
    626 behaviors; 246 solutions
    BEST 7 = (IBin U32 IAdd (ILoc 0) (ILoc 1)); witness rank 9,516

The full run took roughly 160 seconds with the current compiled engine,
versus about 71 seconds for the specialized 19,205-member sweep.  The extra
members are all kernel-well-typed—most differ only by an out-of-range `ILoc`
integer—but the general consumer still performs `tc_infer` and rank roundtrip
on every one before the semantic probe.  This is a useful boundary, not a
reason to weaken final checking: the next optimization should either reuse a
checked typed-grammar invariant or make exhaustive per-member typechecking a
small-corpus audit mode while always rechecking selected/refinement-bound
candidates.

Two boundaries remain explicit.  `TgScopeEnv` now transports reflected
constructor/call heads, atoms, named zones, and argument routes, but arbitrary
`TgRule` templates still do not have a stable dynamic task-file codec.  That
future codec must preserve full kernel `Type`/`Expr` identity instead of
growing a symbol-name pseudo-ISA.  Also, `meta/sketch` rank requires ordered
alternatives not to overlap structurally.  The exhaustive consumer gates
`rank(unrank(i)) = i`, but a future public rule codec should diagnose obvious
overlapping template shapes before a sweep.

### Theorem scope and canonicalization pressure

The first reflected grammar did **not** yet reproduce the playground's most
important scaling lever.  `RCtx` already carried claim/requirement/axiom item
names through the same strict `use` machinery as constructor and call heads,
but `tg_scope_heads` intentionally discarded non-executable items.  Meanwhile
kernel/canon's C7 append recognizer remained a fixed global shape check.  A
post-generation `cn_e` gate would verify canonicality, but would still pay to
generate the enormous noncanonical space; it is not the quotient-first result
measured by the playground.

`tools/search/theorem_scope.shard` now supplies the missing checked join.  A
caller gives an explicit ordered profile of bare theorem names.  Each name is
resolved through the task's real `RCtx`, must denote a claim item in its
`Module`, and must have a full-QName entry in the kernel driver's accumulated
`Theory`.  The captured `TgCanonLicense` retains the checked `Goal`'s binder
types and left-to-right equation plus exact accepted provenance (`Proven` or
`GrantedRequirement`).  Because `Theory.Axiom` alone also represents authored
and upstream axioms, capture re-reads the closure's declaration kinds and
rejects every Axiom entry not produced from a granted requirement.  V1 also
rejects duplicate and premise-bearing rules.  It never silently promotes every
in-scope equality into a rewrite rule.

`theorem_scope_probe.shard` exercises this path against the real `std/list`
interface and the real checker pipeline, including sidecars.  The explicit
scope captures the four ratified append requirements with parameter arities
`1,3,3,1`; all arrive as granted-interface entries.  The probe also pins that
an existing but out-of-scope requirement, an in-scope conditional requirement,
an authored premise-free kernel axiom, and a duplicate selection are refused:

    THEOREM-SCOPE-PROBE APPEND 4 GRANTED TYPED META-TRS NF3 SCOPE-GATES OK

The same run now precedes that line with the quotient-first formation pin:

    THEOREM-FORMATION APPEND 4 RAW 243 CANON 31 REDUNDANT 212 EXACT

It now also runs a generated cumulative-profile census before that full-profile
pin.  At depth 2 every prefix is exhaustively audited against the raw normal
subset; at depth 3 intermediate prefixes are exact formation-grammar counts and
the selected full profile receives the exhaustive audit:

    depth 2: 243 -> 147 (-96) -> 111 (-36) -> 39 (-72) -> 31 (-8)
    depth 3: 59295 -> 21612 (-37683) -> 10992 (-10620)
                   -> 120 (-10872) -> 94 (-26, AUDITED)

The order is the explicit reviewed profile order: nil-left, cons, association,
nil-right.  Marginals are therefore cumulative and may reflect overlap with
earlier rules; they are not an order-independent property of an equation.

It then feeds that identical checked `TrsProfile` to symbolic neutral
formation—without constructing `NRAppend`—and pins both a capture/substitution
rewrite and a constructor-producing RHS whose nested call re-enters the same
profile:

    THEOREM-SYMBOLIC APPEND PROFILE RHS-REENTRY OK

The first reusable compiler target has now graduated to `meta/rewrite`.
`TrsRule` retains a full citation QName, parameter types, and oriented kernel
`Expr` pair.  Its constructor validates the premise-free algebraic v1
fragment: rooted, non-reflexive algebraic LHSs; in-range RHS variables drawn
from the LHS; and no free variables or binding forms. Repeated parameters are
structural-equality constraints. `TrsProfile`
preserves reviewed rule order and rejects duplicate citations.  The generic
engine supplies root application, deterministic preorder rewriting through
ordinary subject binding forms, normality testing, and fuel-bounded normal
forms with an honest exhaustion result.  It does not authenticate equations,
infer subject types, or claim orientation, termination, or confluence.
`trs_empty_profile` and `trs_profile_snoc` now provide validated construction
for generated cumulative or ablated profile families without manipulating the
transparent profile constructor at consumers.

`tg_compile_canon` is the narrow join: after `theorem_scope` authenticates an
explicit checked license list, it compiles that list into the generic profile.
The scope probe now also normalizes a nested append expression to `BVar 0` in
exactly three rewrites.  `rewrite_probe.shard` separately imports only the
graduated module and pins validation, ordered rewriting below a binder, normal
forms, duplicate rejection, and fuel exhaustion.  Thus the additional proof
base can automatically fuel a reusable term-level canonicalizer after explicit
profile selection; it is no longer just a standalone license report.

The first quotient-first use has now landed as well.  `trs_formation` projects
an exact **separable** formation profile from the same validated LHSs.  A rule
may either forbid an entire root, or have exactly one shallow rooted child
discriminator whose children are all metavariables.  Multiple discriminators
would denote a conjunction, and a deep discriminator would require additional
tree state; both are rejected rather than independently excluded and
over-pruned.  Shapes retain Ctor/Call arity, so the projection remains exact
outside the typed producer too.

`tg_build_formation` threads those clauses through the signature-driven typed
grammar.  Every generated node applies the global root exclusions; when a head
is admitted, its argument holes receive the root exclusions derived for their
positions.  Consequently theorem-redex programs are absent from `sk_count`,
not generated and filtered later.  The four checked append laws derive, rather
than hand-code, `Nil`/`Cons`/`append` exclusions for the left operand and `Nil`
for the right.  Over the probe's depth-2 list-expression grammar, exhaustive
rank/unrank checks establish:

- the raw grammar has 243 unique members;
- exactly 31 are normal under the compiled theorem profile;
- the formation grammar has those same 31 members, each normal and rankable in
  the raw grammar; and
- 212 theorem-redex spellings never enter the quotient grammar.

This path is now a normal dynamic-search input rather than only a probe API.
`search_canon_profile` selects the checked equations, and both the exhaustive
typed census and the superposed runner build through
`tg_build_env_formation`.  The superposed report counts the unfiltered grammar
without enumerating it and prints checked rule count, raw count, removed
spellings, and filtered total before its region metrics.

An observational program law is categorically different from an ordinary
term equality, but it is not a different search engine.  If the checker has
proved only

    search_probe(candidate_lhs) = search_probe(candidate_rhs)

then replacing `candidate_lhs` below an arbitrary candidate constructor is
unsound: that constructor need not preserve the observer's equivalence.
`search_observer_profile : () -> List Symbol` therefore captures exactly this
premise-free, unary-observer theorem shape through the same checked scope and
provenance path.  The common observer call is removed only after the theorem
has been authenticated.  Its candidate equation compiles to a distinct
`TrsRootProfile`, not a `TrsProfile`; the types prevent passing an observational
law to the recursive rewriter by accident.

`TrsFormationPlan` combines the two licensed pressures.  Ordinary congruence
formation applies at every generated node.  Root-profile formation is consumed
exactly once at the whole candidate, although its shallow argument exclusions
still constrain that root's child holes.  The dynamic runners load both
profiles from one checked closure and report `CANON RULES` and `OBSERVER RULES`
separately.  `typed_observer_value.shard` pins the distinction on a generic ADT:

    CANON RULES 0; OBSERVER RULES 1
    RAW 4; REMOVED 1; TOTAL 3

The law removes `Leaf(Noise 1)` in favor of `Leaf(Keep 1)`, while
`Wrap(Leaf(Noise 1))` deliberately remains.  This is the reusable foundation
for ISA refinement laws: the observer can be an interpreter, refinement
relation encoded as an exact result, or another task-specific semantic map,
without putting an ISA name in the engine.

The lossless second tier now lives in `meta/search`.  Partitioning a complete
profile leaves separable rules in `TrsFormationPlan` and preserves every other
validated rule, in order, as an `MsPlan`.  `ms_check` interprets a grammar
sketch plus partial choice assignment and returns `Clear`, `Blocked hole`, or
`Redex citation`; ordinary rules scan every subterm while `TrsRootProfile`
rules remain candidate-root-only. Left-linear rules use prepared wildcard
facts; nonlinear algebraic rules use equality-correlated environments, so the
matcher does not invent an independent hole approximation for coupled
patterns.

An unassigned multi-alternative hole is no longer automatically a demand.
`meta/search` interprets every alternative under the current partial assignment
and promotes only unanimous facts: all clear becomes `Clear`, and all reducible
becomes `Redex` only when one citation is valid across the complete domain.
Mixed results, different citations, or a blocked descendant remain `Blocked`
on the outer hole, preserving disjoint region accounting.  The fold stops as
soon as disagreement is established; singleton holes remain transparent.

`constraint_superpose_probe.shard` measures the resulting lazy behavior on
three two-member domains under a semantic query that demands none of them:

    all clear:  FOUND 2, REGIONS 1, FORKS 0
    all redex:  KILLED 2, CONSTRAINT KILLED 2, REGIONS 1, FORKS 0
    mixed:      FOUND 1, KILLED 1, REGIONS 2, FORKS 1

Thus grammar vocabulary that cannot contain a residual redex remains a true
don't-care, while an unavoidable checked redex kills the full hole domain.
`ms_prepare` now amortizes the stable part of this analysis.  It extracts every
non-variable pattern state from the selected residual rules and classifies that
state against every complete grammar-hole domain.  Only unconditional
`Yes`/`No` results are retained in `MsPrepared`; blocked results are omitted
because descendant assignments may refine them.  Facts are indexed first by
grammar hole, so a recursive check scans only that hole's pattern row rather
than the complete preparation table.  SUPERPOSE prepares once at
the public drive boundary and reuses those facts throughout its recursive
region loop.  The generic probe compiles 12 such facts and checks that direct
and prepared classification agree on clear, redex, and mixed domains.

Preparation now composes those match facts one step further as well.  For each
grammar hole it runs the complete rule-ordered classifier once and retains an
unconditional whole-tree `Clear` or common-citation `Redex`; candidate-root
observer facts are compiled into a separate table so their proof domain cannot
leak below the root.  The probe obtains four whole-tree and four root-only
facts.  Its mixed domain produces neither, remains blocked, and is still
settled lazily after refinement.  A later compiled layer could preserve those
blocked dependencies as a decision DAG rather than rerunning them, but it must
keep rule order, common citations, and the exact demanded-hole choice.

`typed_observer_conjunctive.shard` is the non-ISA end-to-end pin.  Its candidate
is simply `Trio Tagged Tagged Tagged`, with independent `Keep`/`Noise` choices.
The checked observer theorem has two simultaneous child discriminators:

    Trio(Noise a, Noise b, rest)  ~observe~  Trio(Keep a, Keep b, rest)

Independent child exclusions would incorrectly remove the two mixed terms.
Formation therefore removes none, while one residual rule removes exactly the
two-member all-Noise-prefix subtree without demanding `rest`:

    RAW/TOTAL 8; DEFERRED RULES 1
    FOUND 6; KILLED 2; CONSTRAINT KILLED 2
    exhaustive agreement: accepted 6; constrained 2; solutions 6

This is the same engine contract as append or the calculator: a reflected
environment and hole language, checked reductions with explicit application
domains, and an observation/refinement target.  The example happens to be a
tiny datatype so the exact settlement is easy to audit.

The remaining differences are proof and constraint tiers rather than
list-search versus machine-search engines:

- candidate equality supplies unrestricted congruence; observer equality is
  initially root-only and needs checked contextual/congruence closure before it
  may descend;
- shallow separable redexes compile directly to hole exclusions; deep,
  conjunctive, and repeated-variable patterns now prune exact partial regions
  as residual constraints. A regular-tree/relational grammar product could
  move more of that work into quotient formation;
- partial, effectful, or fuel-bounded interpreters need conditions or a
  refinement theorem that says when the observation is stable; and
- a useful orientation must stay inside the selected grammar and cost budget,
  or provide an explicit representability certificate.

Consequently the general target remains one pipeline: reflected environment
and hole language, an explicit observation/refinement boundary, checked
reduction profiles with their application domains, a pre-miner, quotient-first
grammar construction, and lazy semantic narrowing.  The next general mining
step is to bucket terms by exact observer behavior, propose oriented schemas,
and submit the resulting observer equations to the checker; deeper algebraic
discoveries can immediately enter the residual tier, including nonlinear
discoveries whose repeated bindings are now checked for partial-term equality,
instead of being installed as unsound shallow filters.

`profile_census.shard` is the reusable measurement join over this mechanism.
It accepts the same reflected heads, atoms, binders, result type, depth, and
ordered `TrsRule` list as the generic typed grammar—not an append-specific ISA
table.  For each cumulative prefix it projects formation constraints, builds
the quotient-first grammar, and reports exact count and marginal reduction.
`PcAuditEvery` additionally enumerates raw terms and proves every formed term
ranks back into the raw normal subset at every prefix; `PcAuditEndpoint` pays
that cost only for the selected full profile.  This distinction mattered at
the first harder rung: auditing every depth-3 prefix ran for 8.5 minutes
without finishing, while endpoint audit completed the full census in
3m19s on the same compiled evaluator.

The result is already useful profile-selection evidence.  The full checked
append profile removes 59,201 of 59,295 depth-3 terms, a roughly 631x reduction
before enumeration.  Nil-left dominates both rungs; association narrowly
overtakes cons at depth 3; nil-right remains small but uniquely useful.  The
ordering is stable here, while the changed marginal magnitudes demonstrate why
the engine should measure rules in the actual scope and hole policy instead of
relying on a universal hand ranking.

Two boundaries remain.  Arbitrary `TgRule` templates are loudly refused by the
formation-aware path because their multi-level static structure needs the full
regular-tree automaton product; checked rewrite profiles can nevertheless
retain such structure as exact residual constraints after grammar
construction.  Ordinary typed grammar behavior is unchanged.
Symbolic neutral formation now accepts the same profile through `NRTrs`, with
generic `Ctor`/`Call`/literal matching, repeated-value equality, and ordinary
symbolic RHS evaluation.  The full laws driver now loads that profile from its own checked
root scope and uses the generic route for self proofs, both G3 rungs, proof
traces, and artifact regeneration; all prior verdict and byte-identity pins
remain unchanged. Binding patterns, correlated formation products, and
decision-procedure normalization remain outside this first-order tier.
Orientation, permission to consume granted requirements, termination, and
confluence remain reviewed profile gates as specified by CANON.md §6.

### Census-driven theorem pre-mining

The playground's real flywheel was stronger than importing an existing lemma
family.  Its catalog measured the gap between syntax and behavior, inspected
high-collision buckets, separated contextual respellings from genuinely
different algorithms, bought the cheapest licensed rule, and re-ran the same
census.  The decisive historical measurements were:

- canonical list programs grew from `19 / 7,790 / 653,491,008` at rungs
  `1 / 2 / 3`, while observed behaviors grew only `13 / about 1,100 /
  at least 4,453,248`; spellings per behavior rose `1.5 -> about 7 -> about
  147`;
- at rung 2, 6,630 of 7,790 forms—85%—were contextual respellings rather
  than new algorithms;
- the first mined generative rule preserved all 1,068 rung-2 behaviors while
  cutting 7,790 forms to 2,356, and cut rung-3 forms from 653M to 58M with
  bit-identical battery behavior;
- contextual partial evaluation ultimately collapsed the stack machine's
  `80 -> 24 -> 8` solution spellings to one, reducing exact-settlement work
  from 450,492 to 21,551 steps over a `2.54e24`-candidate space.

The main repository can go further because conjectures need not remain
playground observations.  A general pre-miner should run before a hard search:

1. Census a shallow instance of the same `TgEnv` and hole policy.  Exact rungs
   retain complete observation vectors; larger rungs use deterministic
   rank-sampling and report confidence separately.
2. Keep the minimum-rank representative, spelling multiplicity, and several
   structurally diverse exemplars per observed bucket.  Rank proof attempts by
   prospective collision mass removed, not merely by term size.
3. Turn representative/member pairs into typed equivalence or refinement
   goals and run the structural-induction oracle.  `Proven` emits a generated
   claim plus replayable proof; `Refuted` contributes its counterexample to the
   battery and rebuckets; `Undecided` records the stuck neutral equations.
4. Census recurring undecided subgoals.  High-frequency frontiers are explicit
   auxiliary-lemma conjectures: pre-mine and prove those smaller statements,
   then retry the parent refinements.  This is the theorem analogue of using
   collision mass to choose a grammar quotient.
5. Classify every proven equality before feeding it back.  A typed,
   well-oriented, high-coverage algebraic equality may enter a ratified canon
   profile; a contextual definitional equality belongs in partial evaluation;
   an equivalence between genuinely different algorithms remains a catalog
   edge/refinement theorem and must not impose a global spelling convention.
6. Re-run the raw/profile census and pin forms, observed behaviors, proof-closed
   buckets, unresolved buckets, and sample-gauge statistics.  A proposed rule
   is purchased only when its claimed behavior preservation replays through
   the kernel at the censusable rungs.

`catalog.shard` now prints the first general measurement needed by this loop:
`SAMPLE-GAUGE` gives exact excess spellings, collided buckets, collided
members, and maximum bucket size for its battery.  The existing rung-1
generated bracket proves all four excess spellings are genuine equivalences;
rung 2 is the first useful mining corpus because it has 1,277 observed excess
spellings.  This metric must remain visibly battery-relative: a collision
proposes a theorem, never licenses one.

`laws.shard mine N` now turns that proposal into an exact proof census.  In
each behavior bucket it compares every non-representative member with the
minimum-rank representative, using the same symbolic evaluator, append
theory, and structural-induction license as the generated rung-1 bracket:

    MINE rung 1: CLEAN 17 / BUCKETS 13 / EDGES 4
                 PROVEN 4 / REFUTED 0 / UNDECIDED 0
    MINE rung 2: CLEAN 2345 / BUCKETS 1068 / EDGES 1277
                 PROVEN 1242 / REFUTED 0 / UNDECIDED 35
                 FRONTIER PERMUTATION 35 / OTHER 0 / MISSING 0

Thus 97.3% of the rung-2 sample gauge already closes in the theorem oracle.
The current battery floor is 1,068 behaviors and the star-to-representative
proof ceiling is 1,103 functions, before proving any auxiliary lemma.  These
are oracle proof skeletons rather than a committed rung-2 kernel artifact;
the rung-1 generated bracket remains the fully replayed exact result.

This census also found and repaired an important general evaluator gap.
Originally only 757 edges proved and 520 were undecided.  A structural IH
recognized an opaque child slot, but after case-splitting that child it lost
the fact that the resulting constructor shape was the same strict subterm.
`SVCtor` now carries branch-local split provenance (`-1` for ordinary
computed constructors), exactly mirroring the playground's shape-owner rule.
The IH accepts a shape only when its origin slot is beyond the goal binders;
the depth-0 goal shape still cannot cite the goal itself.  That one general
change proves 485 more edges and is pinned by the corpus proof census.

Undecided verdicts now retain the first exact residual equation at which
symbolic refinement stopped.  `laws.shard front N MEMBER REP` prints its
stable symbolic spelling (neutral head, blocker, slot ids, and constructor
origin); speculative neutral-argument comparisons restore the prior frontier
when the outer blocker can still split, so the retained equation is the real
terminal obligation rather than an abandoned congruence probe.  Flattening
the already-right-associated append neutrals and comparing their atom bags is
diagnostic only—it classifies a conjecture and grants no rewrite.

That classifier makes the first remaining frontier exact: all 35 are reversed
or permuted append spines over recursive results on a tail and its tail; none
has another residual shape.  For example, two functions share their base
cases and differ by
`f(x2) ++ f(x4)` versus `g(x4) ++ g(x2)`.  Their outputs happen to inhabit a
commuting submonoid (in the smallest pair, repetitions of the last element),
but global list append is not commutative.  The next useful pre-mining
component is therefore conditional/range theorem discovery from recurring
stuck equations—not an unsound global commutativity rule and not another
task-specific template.

That component now has its first complete census.  `meta/census` is the
general exact weighted-key substrate: it records support, prospective mass,
first occurrence, bounded distinct exemplars, and a deterministic
mass/support/first ranking.  `tools/search/frontier.shard` supplies the
search-specific structural keys: full QNames except for caller-declared head
roles, separate alpha-renaming for universal atoms and data slots, and
orientation-independent residual equations.  Mining accumulates these keys
during the existing proof pass, so no second 1,277-edge oracle sweep is needed.

At rung 2 the result is highly regular:

    AUXILIARY SIGNATURES 5 SUPPORT 35 MASS 35
    AUX rank 1..5:       SUPPORT 7 MASS 7 each
    COMMUTATION BASES 1 SUPPORT 35 MASS 35

The second line of analysis is deliberately diagnostic.  It flattens only an
already-classified append permutation, finds pairs of distinct atoms whose
relative order changes, and ranks the smaller commutation basis; it neither
changes a verdict nor installs a rewrite.  All five exact residual contexts
reduce to one typed candidate schema (where `f` is the bucket representative):

    append (f (Cons h t)) (f t) = append (f t) (f (Cons h t))

`laws.shard range N REP` materializes that schema as an ordinary `LLaw` and
runs the same symbolic oracle.  Representative 295 (and the other sampled
owners) is honestly still `Undecided`, with one permutation/commutation basis:

    MINE-RANGE UNDECIDED FRONTIER PERMUTATION COMMUTATION-BASES 1

This rules out “just census the residuals” as the next step and makes the proof
gap precise.  More case budget merely expands the same permutation.  The
needed general facility is induction over a derived relation or range
invariant, so that the smaller commutation theorem can be assumed on strict
subterms and then replayed as a checked auxiliary claim.  Only after such a
claim proves may parent edges be retried or a profile-pressure experiment be
considered; observational support 35 is ranking evidence, not a canon license.

### Soundness boundary and next experiment

Observation selects candidates; it does not prove refinement.  G4 is closed
today by the two checked pin artifacts.  The engine does not yet generically
render its chosen `IExp` into an owned proof artifact, nor does it synthesize
the spec⊑imp proof for an arbitrary task.  The task's certified witness is a
temporary gate, not a claim that search has solved proof discovery.

The next proof experiment should give `LLaw` a general derived-relation
induction path: nominate a structural parameter, generalize the remaining
parameters, retain the root relation as an IH schema, and permit an application
only when provenance proves the nominated argument is a strict subterm.  The
range-commutation candidate is the first regression target.  If it proves, the
engine should retry its 35 dependent parent edges and render the accepted
auxiliary claim for kernel replay.  Only a typed, well-oriented theorem that
survives that path may be considered by the shared profile-pressure census.
The theorem quotient remains proof-licensed and task-independent; the
observational quotient remains battery-relative.  Keeping those identities
separate lets them compose without mistaking test equivalence for theorem
equality.

### Playground transfer: contextual generation pressure

`catalog_pressure.shard` makes the playground's first mined generative rule an
explicit ablation of the current catalog builder.  `CatPolicy` is an ordinary
Shard value whose first control governs recursive calls on binders pinned to
`Nil`; the ordinary `cat_grammar` API selects the safe default.  The experiment
counts the policy-off space, proves every production candidate ranks into it,
and sweeps the production space once.  Each candidate is then placed in three
nested layers:
production, no C8 violation, and fully `cn_e`-clean.  The complete behavior-key
sets—not only their cardinalities—must agree between all three terminating
layers:

    rung 1: R1-OFF-GEN 20 / PIN-SAFE 20 / C8-NORMAL 19 / CANON 17
            BEHAVIORS 13 / 13 / 13 EXACT
            SPELLINGS C8-NORMAL rev 0 / id 3; CANON rev 0 / id 2
    rung 2: R1-OFF-GEN 9435 / PIN-SAFE 3395 / C8-NORMAL 2356 / CANON 2345
            BEHAVIORS 1068 / 1068 / 1068 EXACT
            SPELLINGS C8-NORMAL rev 2 / id 6; CANON rev 2 / id 4

The exact transferred junction is the playground's post-R1 result:
**2,356 forms, 1,068 behaviors, and 2 / 6 rev / id spellings**.  The current
engine exposes its pressure more finely: pin-aware formation first avoids 6,040
policy-off members, C8's
remaining rebuild discipline removes 1,039 spellings without losing a sampled
behavior, and the newer C10 vacuous-match pressure removes 11 more, again with
the behavior set unchanged.  At rung 1, C8-normal also reproduces the old
`19 programs = exactly 13 behaviors` catalog boundary before C10 tightens it to
17 spellings.

`R1-OFF-GEN` is deliberately not called `RAW`: the playground's raw twin also
disabled append orientation, pin-normal-form, and match-order constraints and
therefore measured 150 and 69,567,550 forms at depths 1 and 2.  This ablation
isolates one causal policy.  It also does not execute the policy-off members,
some of which contain the very nonterminating `f(Nil)` recursion R1 excludes;
their count is a generation-pressure measurement, while behavior equality is
asserted only across the production/C8/canon layers that the current fragment
declares total.

### Transition-window mining and checked ISA sequence pressure

The first imperative/ISA mining rung now uses the same reflected task boundary
rather than an instruction-specific engine.  `transition_mine.shard` consumes
an ordinary `typed_expr` scope whose observation is an exact `List Int` key,
checks and ranks every grammar member, retains the complete accepted corpus,
and orients each collided behavior toward its least structural-cost member.
The initial x86 adapter supplies every zero-to-two-instruction register XOR
sequence over three scoped registers—91 programs, with no x86 name in the
miner:

    TOTAL 91; BEHAVIORS 55; COLLIDED-BUCKETS 19
    EXCESS 36; MAX-BUCKET 7

`meta/antiunify` is the reusable schema-contraction layer.  It computes a
simultaneous least-general generalization of two directed equations with one
mismatch table shared across both sides.  Repeated roles therefore remain one
nonlinear metavariable even when the correlation crosses the equation.  It
also reports variable pairs that differ in every concrete support as
*empirical guard proposals*.  Those guards carry no proof authority.

The transition miner anti-unifies pairs of concrete collision edges, rejects
malformed or non-decreasing orientations, and replays each candidate schema
against the complete shallow grammar.  Every matching LHS must have a
representable RHS with the same exact behavior key.  The top contracted basis
contains stronger versions of the motivating double-self-XOR example:

    xor d,s ; xor d,d  ->  xor d,d       removes 9 / 91
    xor d,d ; xor e,d  ->  xor d,d       removes 9 / 91
    xor d,s ; xor d,s  ->  []            removes 6 / 91, guard d != s

Thus `(xor r,r ; xor r,r) -> xor r,r` is not installed as a bespoke rule; it
is one instance of the first mined schema.  The third result also pins why
blind anti-unification is unsound: admitting `d = s` changes zeroing into the
identity transformer.  Complete-census replay is stronger than inspecting two
examples but remains battery-relative evidence, so every report still labels
these schemas proposals rather than licenses.

The first proposal has crossed the proof boundary.  `std/bits` now proves
`bxor_self` from the kernel recurrence by well-founded induction.  The x86
peephole module proves generic `rget/rset` laws and then
`xseq_xor_self_absorbs` for arbitrary destination, source, tail, module,
register file, and memory.  Its two nonnegative premises expose the model's
valid-word boundary.  Its fuel is deliberately shifted:

    eval (S^(3+f)) [xor d,s; xor d,d]++tail
      = eval (S^(2+f)) [xor d,d]++tail

Removing a list cell removes one structural fuel unit.  Same-fuel equality at
the exhaustion boundary would be false.  The transition task therefore uses
a sequence-length-normalized fuel observer, proves its sample register files
satisfy the word premises, and derives the exact premise-free observer theorem
selected by `search_observer_profile`.

That theorem enters through the existing checked scope/provenance path and the
ordinary nonlinear residual matcher.  No new ISA rule channel was added:

    CANON RULES 0; OBSERVER RULES 1; DEFERRED RULES 1
    RAW/TOTAL 91; CONSTRAINT KILLED 9
    AUDIT accepted 82; constrained 9; exhaustive agreement OK

`REMOVED` remains zero because this correlated deep pattern is intentionally a
residual constraint rather than a separable formation clause.  The result is
nevertheless real narrowing pressure: all nine instances are rejected by the
checked theorem before semantic acceptance, including the three literal
double-self-XOR programs.

The 91-program figures above record the initial depth-4/two-instruction mining
configuration.  The task has since grown to the depth-5 experiment described
below; the earlier tables remain the baseline that produced the proposal.

#### Checked structural-spine closure

The contextual gap is now closed for a deliberately small, general structural
fragment.  `meta/rewrite` has three distinct equation domains:

* `TrsProfile`: ordinary candidate equality, valid below every constructor;
* `TrsRootProfile`: exact observer equality, valid only for the whole program;
* `TrsSpineProfile`: contextual observer equality, valid at the root and then
  only down one authenticated repeated-constructor child.

A spine path is not configuration data.  The reusable `trs_inspect_spine`
examines a transparent binary `FnDef` and accepts exactly a structural right
action with one base arm and one constructor arm:

    plug prefix suffix =
      match prefix with
        Leaf       -> suffix
        C fields   -> C fields[child := plug child suffix]

It derives the context QName, constructor QName, and recursive child index
from the locally-nameless body.  A regression derives `Cons/1` from an
append-shaped function and refuses a function which recurses in two fields.
The recognizer is in `meta/`; checked Theory/provenance joining remains in
`tools/search/theorem_scope.shard`.

The checked theorem shape is:

    search_probe (plug prefix lhs) = search_probe (plug prefix rhs)

`prefix` must be an otherwise-unused theorem parameter.  Capture refuses a
fixed prefix, different prefixes on the two sides, premise-bearing laws, and
the subtle correlated case where the prefix parameter occurs again inside the
local lhs or rhs.  The stripped local equation retains other parameters such
as a sequence `tail`, allowing a window law to match in the middle of a
program.  Tasks select the transparent context and ordered laws through the
optional `search_spine_context` and `search_spine_profile` functions.

`meta/search` validates all three domains in one `MsPlan`.  Existing two-domain
callers still use `ms_plan`; `ms_plan_spine` adds the third profile.  Its
partial matcher uses the existing left-linear cache and nonlinear equality
environment, treats an unassigned grammar hole by exact consensus, and enters
only the inspected constructor child.  It never upgrades observer equality to
unrestricted congruence.

The x86 transition task now contains a transparent `xtw_plug` and a proven
prefix-and-tail law for the mined XOR/self-XOR schema.  The contextual proof
factors through a total transition projection for the searched register-XOR
fragment; the earlier `xeval_seq` theorem remains the full-model semantic
kernel for the original root experiment.  The contextual theorem is
premise-free and is checked before capture:

    probe (prefix ++ [xor d,s; xor d,d] ++ tail)
      = probe (prefix ++ [xor d,d] ++ tail)

At depth 5 the explicit scope contains every length-zero-through-three
sequence: `1 + 9 + 81 + 729 = 820` programs.  The spine rule removes 162:

    root window:       9 + 81 = 90
    one-step prefix:       81
    overlap:                9
    union:          90 + 81 - 9 = 162

Thus 72 reductions are genuinely new contextual pressure which no root-only
profile can see.  Enumerative and lazy runs agree exactly:

    TYPED: SPINE RULES 1; RAW 820; ACCEPTED 658; CONSTRAINT KILLED 162
    SUPERPOSED: SPINE RULES 1; REGIONS 772; FORKS 387; CONSTRAINT KILLED 162
    AUDIT accepted 658; constrained 162; exhaustive agreement OK

`REMOVED` is still zero: a nonlinear variable-length window is residual
pressure, not a separable formation clause.  The next independent gap remains
checked conditional rules for the mined `d != s` cancellation schema.  The
next spine-specific refinements are stable whole-hole verdict caching and a
multi-arm/multi-spine inspector for recursive datatypes with more than one
structural branch.
