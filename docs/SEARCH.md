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
