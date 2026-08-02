shard program search — SEARCH.md
=================================

> Path note (2026-07-18): file paths in this ledger are as-landed history; the repo was reorganized — decode old `examples/` paths via [LAYOUT.md](LAYOUT.md).

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
them as residual rules.  Preparation now also compiles the *linear relaxation*
of every nonlinear lhs: repeated variables are treated as independent
wildcards, and only a relaxed `No` is consumed as conclusive.  Relaxed
`Yes`/`Blocked` still enter the exact binding environment and guards.  This
recovers stable head/shape/type facts and can see a later static mismatch past
an earlier equality block without weakening the correlated rule.

Partial structural equality also takes unanimous alternative domains
seriously.  If every member of an open Cartesian domain is unequal it returns
`No`; if every member is equal it returns `Yes`; mixed or internally blocked
domains still block on the outer choice.  The nonlinear probe pins both
improvements: `Triple(x,x,0)` is immediately clear against
`Triple(open,open,1)`, disjoint two-member atom domains are unequal without a
fork, and overlapping domains retain the original exact block/equal/unequal
behavior.  Three relaxed facts are compiled for its three grammar holes.
The complete depth-2 nonlinear sort report remains exactly
`6851 regions / 1438 forks / 50450 steps`; on the fixed 5,000-job depth-3
frontier the schedule moves slightly to `4125 / 875 / 41735` while settling
928,781 more candidates.  This is useful early classification, not the missing
diagonal representation, and its extra linear scan cost remains visible.

The original-id region vocabulary is now in `meta/sketch`, below the search
policy.  `SkRegion` combines fixed grammar choices with forbidden choices;
`sk_region_count` performs the exact reverse grammar fold under both, and
`sk_region_subtract_cube` subtracts a conjunction without cloning a grammar.
For the nine-member product `Pair(h0,h1)`, subtracting `h0=1,h1=2` yields one
hit and the disjoint complement

    h0 != 1                  count 6
    h0  = 1, h1 != 2         count 2

`region_probe.shard` pins `RAW 9 = HIT 1 + COMPLEMENT 6+2`, all over the
original hole ids, plus the zero count obtained by forbidding every choice of
one hole.  Regions are independently addressable as well:
`sk_region_rank`/`sk_region_unrank` preserve the ordinary grammar ordering after
removing excluded alternatives, and `sk_region_member` is the corresponding
exact predicate.  The probe pins a two-member non-contiguous slice, its rank
round trip, and rejection of an excluded candidate.  A nested unequal-weight
grammar also pins `4 -> 3 -> 2`: excluding one of a child hole's three choices
updates its parent's recursive alternative, then fixing that parent to the
recursive alternative preserves the exact child count.  This is intentionally
a foundation rather than a reported search win:
`ms_check_region` and `ms_check_prepared_region` now consume those fixed and
forbidden choices directly over the original grammar.  Full-domain prepared
facts remain sound under restriction; an uncached match sees only the allowed
alternatives, and a one-member remainder becomes transparent.  The nonlinear
probe fixes its left atom and then proves the theorem Clear when the equal
right choice is forbidden, Redex when the unequal right choice is forbidden,
in both direct and prepared modes.  Ordinary assignment-only search retains
the exact semantic-first x86 control report (`742 / 372 / 25312`).

SUPERPOSE jobs now carry `SkRegion` end to end.  Fixed choices alone feed the
lazy evaluator and its consulted-choice memo; forks skip forbidden choices;
theorem checking sees the complete region; killed and passing subspaces use
`sk_region_count`; passing-region membership and representatives use restricted
rank/unrank.  The mixed residual-rule integration probe starts the driver from
each non-contiguous singleton remainder and pins clear/redex settlement as
`1/1` with no fork.  The ordinary rev, pure-program, calculator, and default
semantic-first paths retain their exact controls.

The first general relational partition is now available as
`ms_equal_partition`.  It repeatedly applies the same partial structural
equality used by nonlinear matching, but classifies a blocked hole's allowed
choices before deciding how to refine.  Choices with stable equal or unequal
verdicts are coalesced by forbidding the opposite class; only choices that
still block become fixed pending jobs.  Consequently two three-choice atom
holes partition their nine-member product into three exact diagonal cubes and
three two-member unequal row regions (`EQ 3 + NE 6`, four relational split
boundaries), rather than nine ground cases.  The nonlinear probe pins exact
coverage, region counts, and per-region cardinalities.  This is deliberately
equality vocabulary rather than an x86 transform table; distinctness premises
consume the unequal side, while repeated pattern variables consume the equal
side.

`ms_partition_prepared` lifts the same operation to a complete prepared theorem
plan.  It groups redex alternatives only when their checked theorem citation
agrees, groups the Clear complement independently, and keeps still-blocked
children as fixed pending jobs.  The nonlinear probe now pins both the raw
equality partition and the cited plan partition at `EQ 3 + NE 6` in `3+3`
regions and four split boundaries.  SUPERPOSE can consume that partition in a
general theorem-first mode: cited redex regions settle before evaluation, and
only the exact Clear complement reaches behavioral narrowing.  Typed tasks opt
in with `search_narrowing_strategy = theorem_first`; semantic-first remains the
default because a behavioral failure can sometimes kill a broader parent than
any theorem partition.

The x86 transition census now opts into theorem-first scheduling.  Its exact
264-program checked quotient and exhaustive agreement are unchanged, while
terminal regions fall from 742 to 625 and evaluator steps from 25,312 to
23,267.  Relational split boundaries rise from 372 to 411: the current finite
partition spends 39 additional theorem-side decisions to obtain 117 fewer
terminal regions.  This is the first actual engine-level gain from the
original-id negative-region representation, and it applies to any prepared
Shard theorem plan rather than to XOR syntax.

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
win.  The playground later tried an exact cloned twin-grammar product for this
relation and removed it after the cloned hole identities fragmented narrowing
memo reuse.  The categorical next step is therefore a relation-aware region
state over the original grammar DAG: assignments plus lazy equality/difference
restrictions, with exact region counts and no new grammar identities.  Stable
symmetric operand orientation still needs a reviewed syntax order.  Removing
`int_eq` wholesale remains a task vocabulary choice unless an in-budget
equivalent is proved representable.

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

An exhaustive impossibility census may instead declare
`search_expect_empty : () -> Bool` and return `True`.  This replaces the
positive witness with a checked `SOLUTIONS = 0` gate: the driver still settles
the complete space, retains theorem and representative accounting, and an
optional `audit` must independently agree with zero enumerative solutions.
The mode is intentionally rejected by cardinality-free first-result search,
where stopping cannot prove absence.  A false or malformed hook is fatal, as
is finding even one solution.

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
and orients strict reductions toward every least structural-cost member of a
collided behavior.  Keeping the complete minimum-cost set avoids introducing
an arbitrary rank-based gauge choice before schema mining.
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
The opt-in `meta/antiunify/typed.shard` companion recovers the type of every
generalized role with the kernel type synthesizer, in theorem-binder order;
ambiguous or inconsistent role types are refused.

The initial transition miner anti-unified pairs of concrete collision edges,
rejected malformed or non-decreasing orientations, and replayed each candidate
schema against the complete shallow grammar.  Every matching LHS had to have a
representable RHS with the same exact behavior key.  The top contracted basis
contained stronger versions of the motivating double-self-XOR example:

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

#### Whole gauge-orbit contraction

The depth-5 space makes pairwise proposal construction an algorithmic problem,
not a Shard-versus-native constant factor.  Its proof-free mining adapter has
the same scope, observer, and four register files as the checked task, without
loading the peephole proof closure:

    TOTAL 820; BEHAVIORS 182; COLLIDED-BUCKETS 146
    EXCESS 638; MAX-BUCKET 37

The default miner now treats a *gauge orbit* as two simultaneous invariants:

* identical non-leaf constructor/call topology; and
* the same equality partition among aligned leaves.

Register, immediate, enum, and other leaf identities may be renamed, but a
repeated role cannot silently split and two distinct roles cannot collapse.
Edges are partitioned by that relation before generalization.  For each whole
orbit the miner marks every aligned leaf position which varies in any support,
abstracts all correlated varying roles at once, and retains singleton scoped
leaves as constants.  Thus the sole allowed `XXor` remains concrete while all
register roles seen under renaming become typed `Reg` metavariables.  Guards
are inferred from the complete orbit, not a lucky pair.

This fixes two sources of artificial gauge choice.  Every minimum-cost bucket
representative contributes strict size-decreasing edges, and whole-orbit
abstraction cannot leave `RAX` or `RSI` pinned merely because two selected
supports happened to agree there.  Complete-census replay and type recovery
then run once per unique schema.  On the 820-program rung:

    pairwise gauge seeds:       595 partially pinned schemas
    whole-orbit abstraction:    111 unique schemas
    complete replay accepted:   111 / 111 proposals

The highest-ranked low-complexity result is the motivating family itself,
with no instruction-specific template in the miner:

    [xor d,s; xor d,s] -> []
    d : Reg; s : Reg; guard d != s
    removes 6 corpus members; structural gain 72

The ranking uses total removed structure, then support, then shorter theorem
LHS.  Longer three-instruction cancellations and reorderings remain in the
same 111-proposal worklist rather than displacing the simplest proof target.
`transition_mine TASK` uses whole gauge orbits; `TASK orbit` retains the
topology-only pair experiment, and `TASK all` retains exhaustive cross-shape
pairing.  The latter modes are conjecture-expansion tools, not the scalable
default.  None of the 111 replayed schemas is a search license until a checked
theorem authenticates it.

Whole-orbit support now also drives a bounded relational pre-mining pass for
Int-valued roles.  `meta/antiunify` enumerates normalized affine invariants
with unit coefficients and at most six terms.  The baseline guard family holds
the structural disequalities plus the tight interval hull of every Int role;
each multi-role affine equality becomes a separate alternative atop that
baseline.  Unrelated equalities are deliberately not conjoined: two
observations can imply many arithmetic coincidences, and a large conjunction
would overfit while appearing selective.  Every alternative is instead
replayed against the complete accepted corpus by the same exact-key validator
used for structural schemas.  Pairwise diagnostic modes retain structural
guards only because two-example numeric evidence is too weak.

The miner now closes that classification loop through the same checked profile
loader as `typed_expr`.  It retains the application domain of each rule
(`everywhere`, root observer, or authenticated spine) and compares proposals
with `TrsConditionedRule` values, never theorem names.  Coverage is
alpha-invariant and type-aware.  More importantly, it is a theorem-domain
*superset* test: a general checked rule may instantiate an extra tail binder to
the proposal's `Nil`, and an unused stripped prefix binder is harmless.  Every
mined metavariable must still have a type-correct theorem source, and every
theorem guard must hold throughout the proposal's empirical guard domain.
The regression additionally refuses binder-type drift and refuses using a
guarded theorem for an unguarded proposal.

On the proof-bearing x86 task, the existing checked absorber profile covers
four finite orbit schemas:

    VALIDATED-PROPOSALS 111; AUTHENTICATED 4; PROOF-WORKLIST 107

Three are distinct-register absorber windows at different concrete tails,
authenticated by `xtw_xor_self_absorbs_distinct`; the equal-register orbit is
authenticated by the unconditional `xtw_xor_self_absorbs_spine`.  The report
prints these under `AUTHENTICATED BASIS` with citation and domain.  The
double-XOR cancellation remains first under `PROOF WORKLIST`, correctly
reflecting that its full-state theorem exists but its observer-prefix invariant
closure is not yet a checked spine license.

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
fixed prefix, different prefixes on the two sides, unsupported premises, and
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
pressure, not a separable formation clause.

Prepared search now carries stable spine verdicts alongside its existing
everywhere and root tables.  A cache entry is keyed by the checked spine-rule
name and grammar hole, because two rules may own different authenticated
traversal domains.  Preparation records only unanimous `Clear` or
same-citation `Redex`; mixed and descendant-blocked alternative sets remain
live decisions.  `guard_probe.shard` constructs one mixed, one all-clear, one
all-redex, and a nested authenticated-spine region.  It observes exactly four
cached facts and pins direct/prepared agreement, including a redex reached
strictly below the checked constructor path and a cached child reached below
an already materialized parent.  The remaining structural extension is a
multi-arm/multi-spine inspector for recursive datatypes with more than one
structural branch.

#### Checked structural-distinctness guards

The first conditional-rule fragment is now represented without embedding a
predicate evaluator or an ISA-specific register test in `meta/search`.
`TrsConditionedRule` pairs an ordinary validated equation with a list of
`TrsGuard`s; the first guard is structural distinctness between two bound rule
variables.  Spine rules carry this wrapper, so premise logic is orthogonal to
the authenticated traversal domain.  An empty guard list is the existing
unconditional behavior.

The checked source premise has the narrow shape:

    (int_eq (disc x) (disc y)) = False

`disc` is not trusted by name.  `meta/rewrite` inspects its transparent unary
`FnDef`, requires a match over the complete constructor set of its input type,
and requires every nullary constructor arm to return a different Int literal.
The theorem layer then lowers the premise to `TrsDistinct(disc,x,y)`.  Missing
constructors, foreign constructors, duplicate constructors/codes, other
predicates, and non-variable arguments are refused.  Rule validation also
requires both guarded variables to occur in the lhs.

Guard evaluation is lazy over the nonlinear match environment:

* definitely equal bindings falsify the guard and leave the candidate Clear;
* definitely different bindings enable the cited reduction;
* an unresolved grammar hole returns Blocked on that exact hole.

`guard_probe.shard` pins all four cases, including a shared open hole which is
known equal without choosing an alternative and a mixed open hole which blocks
until assignment.  `spine_probe.shard` separately pins finite-discriminator
coverage/injectivity and rejects a duplicate code.

The x86 task exercises the full checked path with `xtw_reg_code` and an
`int_eq(...)=False` premise on a contextual absorber theorem.  That premise is
deliberately stronger than necessary; the unconditional absorber follows it
in the ordered profile, so equal-register cases fall through and total
pressure remains 162:

    SPINE RULES 2; DEFERRED RULES 2
    RAW 820; ACCEPTED 658; CONSTRAINT KILLED 162

The guarded absorber remains a plumbing theorem, but its two semantic
prerequisites have now graduated into reusable libraries.  `std/bits` proves
right cancellation

    bxor (bxor a b) b = a

on the kernel primitive's nonnegative domain, by recurrence plus a checked
quotient/remainder reconstruction.  The x86 model exposes a complete
`reg_code` discriminator and proves both distinct-register `rget/rset` framing
and `rset rs r (rget rs r) = rs`.  Those facts compose into the full-state,
arbitrary-tail theorem

    xeval ([xor d,s; xor d,s] ++ tail) = xeval tail

under the checked `reg_code d != reg_code s` guard and the model's ordinary
nonnegative word premises.  No ISA-specific search mechanism was added.

That observer boundary is now closed by a finite, compositional invariant.
`xtw_word_regs` normalizes every field of the complete register record through
`wrap64`; equality with the original record states that the machine state is
already word-normal.  Generic `rget`/`rset` commutation isolates the finite
record layout, XOR closure preserves normalization, and the abstract
transition proves the invariant for every `XInstr` (instructions outside the
reflected XOR fragment are its specified no-ops).  Induction lifts that result
through an arbitrary prefix.  The four census files discharge the fixed-point
premise once, after which `xtw_xor_pair_cancels_spine` exposes the premise-free
observer equation with only the already-checked structural distinctness guard.

The new rule removes 108 programs by itself: six two-instruction roots and
102 length-three programs across the two possible window positions.  Six of
those length-three programs also contain the absorber window, so the composed
profile removes `162 + 108 - 6 = 264` distinct candidates.  Enumerative and
lazy paths agree exactly:

    SPINE RULES 3; RAW 820; ACCEPTED 556; CONSTRAINT KILLED 264
    SEMANTIC-FIRST: REGIONS 742; FORKS 372; STEPS 25312
    THEOREM-FIRST: REGIONS 625; FORKS 411; STEPS 23267
    AUDIT accepted 556; constrained 264; exhaustive agreement OK

Thus one mined guarded family adds 102 unique reductions.  The theorem-first
partition exposes that quotient before behavioral narrowing without cloning
the grammar (`REMOVED 0` still denotes formation-time removal); correlated
variable-length window formation remains a separate relational-grammar
problem.

The nonlinear linear-relaxation and equality-domain consensus leave this
particular report exactly unchanged (`742 / 372 / 25312`).  That is the
expected control result: every routed register domain has the same three
members, so the duplicate-XOR laws describe genuinely mixed diagonals rather
than a hidden shape mismatch or disjoint domain.  The new pressure helps other
scopes early without manufacturing an x86 win; this task now isolates the next
engine boundary cleanly—the region/job language must be able to retain a
choice exclusion or equality relation lazily over the original hole ids.

The miner's proof classifier now respects that same contextual domain.  A
spine-authenticated rule retains its inspected `TrsSpine` descriptor; reusable
`meta/antiunify` coverage strips zero or more common constructor layers only
when both sides use the checked constructor, the recursive child exists, and
every non-child field is `expr_eq`.  Local-fragment coverage may ignore schema
binders used only by the stripped context, but every binder still present in
the local equation keeps the existing alpha/type check and theorem guards must
still be entailed.  Negative probes pin wrong constructors, wrong or absent
children, changed non-child fields, and missing guards.  Compound guard
bindings are considered definitely distinct only when both are closed ground
terms; unequal open syntax containing proposal binders is refused because a
later substitution could make the terms equal.

That changes no search license and no candidate count; it repairs the mining
queue's accounting.  The same 111 replay-valid proposals move from
9 root-shaped recognitions / 102 apparent proof targets to:

    AUTHENTICATED 26; PROOF-WORKLIST 85

The classifier remains deliberately conservative when a theorem metavariable
matches a compound term containing fresh proposal binders.  General closure
there needs checked substitution typing under the proposal telescope; merely
dismissing those binders would be unsound.  Search already applies the theorem
to such concrete grammar regions, so this is a proof-worklist precision
frontier, not missing narrowing pressure.  The miner also still classifies an
`everywhere` theorem at the proposal root; recognizing it through an arbitrary
common `Expr` context needs a general checked zipper rather than borrowing the
more restrictive structural-spine projector.

#### Checked affine Int equality and order guards

Conditional spine rules now cover finite arithmetic relations without cloning
one theorem per literal tuple.  `meta/rewrite` owns a small typed
`TrsIntExpr` vocabulary over rule variables: Int literals, variables,
addition, and subtraction.  `TrsIntEq` compares two such expressions and
`TrsIntLe` supplies non-strict order.  This is an explicit relation language,
not an embedded Shard evaluator; rule validation requires every referenced
variable to occur in the lhs and to have the checked `Int` parameter type.

`theorem_scope` lowers a direct checked premise such as

    (= (+ d1 (+ d2 1)) out)

or

    (= (le 0 d1) True)

to that vocabulary.  Order capture deliberately accepts the checked Boolean
shape `le(left,right)=True`; strict order and false-order complements are not
silently reinterpreted.  Calls outside the affine fragment, malformed
arithmetic, non-Int variables, rhs-only variables, and other proposition
shapes are refused.  Structural distinctness remains the other supported
premise form, and mixed guard lists retain theorem order.  Ordinary canon and
root-observer profiles remain premise-free; the guarded vocabulary is attached
only to the authenticated contextual domain that already knows how to check it
at each candidate site.

Arithmetic in a theorem RHS already fits the ordinary kernel `Expr` template.
After substituting matched rule variables, `trs_inst` now folds ground core
`+` and `-` calls bottom-up.  Thus a single checked replacement such as
`delay=(d1+d2+1)` emits the literal delay when its inputs are literal, while an
open expression or a foreign function remains exact syntax.  Search rejection
itself only needs the cited lhs, but this definitional fold makes the same rule
usable by generic normalizers and source-emitting consumers.

Ground matching evaluates the relation exactly.  Lazy matching resolves Int
bindings through the current grammar region and returns `Blocked(h)` at the
first undecided value.  No arithmetic-specific scheduler was added:
`ms_partition_prepared` classifies that hole's allowed alternatives, groups
same-citation redex choices, and represents the complementary choices with
the existing forbidden-choice regions.

`affine_guard_probe.shard` pins the whole join for `left + 1 = right`.  Over a
3-by-4 literal product, the one checked relation covers all 12 members as
three cited singleton reductions plus three coalesced clear rows:

    RHS GROUND-FOLD; RAW 12; REDUX 3; CLEAR 9
    REGIONS 3 + 3; RELATIONAL FORKS 4

`int_order_guard_probe.shard` similarly pins `0 <= delay` over a grammar that
contains negative, zero, and positive literals: the negative member is clear,
zero is a cited redex, and an open delay blocks on its original hole.
This closes the vocabulary gap exposed by clamped delay semantics: a checked
cycle-conservation rule can carry `0 <= d1` and `0 <= d2` alongside its affine
equality, so the rule remains universally sound over Int while firing across
the task's whole nonnegative delay alphabet.

This is the intended delay-alphabet scaling shape for PIO-style cycle
conservation: theorem count is independent of the literal alphabet, and
unreachable combinations disappear when the relation is intersected with the
task's actual grammar region.

The transition miner now completes the other half of that join.  It discovers
stable unit-coefficient affine relations and per-role intervals from the
complete support of each whole gauge orbit, emits each relation as an
empirical proposal domain, and revalidates that domain over the full census
before classification.  The theorem classifier symbolically normalizes
`TrsIntEq` and `TrsIntLe` through the proposal's alpha-renaming.  Equality
requires the exact empirical relation; order uses ordinary bound entailment,
so an observed `d >= 2` domain can cover a checked theorem requiring only
`d >= 0`, but never the reverse.  Structural guards cannot accidentally
authenticate arithmetic.  As everywhere else in the miner, surviving
evidence is still not a search license—only a checked conditioned rule can
cross that boundary.

`transition_affine_probe.shard` pins the evidence discipline with two supports
that imply both a useful five-variable conservation relation and an accidental
four-variable coincidence.  Both are pre-mined; complete-census replay keeps
the useful guarded family with support two and rejects the coincidence on a
third member.  `antiunify_probe.shard` separately checks that an alpha-matched
checked theorem with two nonnegative premises and the five-variable equality
covers the first family.  Its inferred interval hull entails the weaker
nonnegative bounds, refuses an excessive lower bound, and cannot justify the
theorem without the equality relation.  The established 820-member x86 rung
has no Int-valued generalized roles and remains unchanged at 111 unique
schemas and 111 validated proposals (26 authenticated in the proof-bearing
task).

#### Cardinality-free first-result synthesis

Exact candidate weights are a census requirement, not a prerequisite for
finding one program.  Typed tasks may now select `search_result_mode = first`;
the SUPERPOSE driver then retains the same partial regions, lazy evaluator,
checked formation, and residual theorem plan, but stops at the first passing
region without computing raw, filtered, killed, or passing candidate
cardinalities.  `search_drive_budget` bounds region decisions explicitly.
Reports mark `CARDINALITY UNCOUNTED` and validate both the productive region
representative and the supplied witness semantically rather than assigning
either an exact rank.

Expected-empty tasks remain on the exhaustive path.  A region budget or first
passing result can establish existence but cannot certify nonexistence, so
`search_expect_empty` is rejected when `search_result_mode = first`.

`meta/sketch` owns the supporting general operations.  `SkProductivity` is a
saturating bottom-up grammar analysis with one bit per original alternative;
it removes zero-cardinality depth-boundary productions without renumbering
choices.  `sk_region_first` uses that analysis to choose the first productive
member of a fixed/forbidden region without bignum arithmetic, including the
case where an earlier syntactic production is barren.  The known-entry region
operation applies a choice when the caller already has the grammar entry, so
an indexed scheduler does not rescan the full grammar at every fork.

Large occurrence grammars exposed two unrelated quadratic setup paths.  The
typed builder now uses an amortized-linear FIFO work queue and resolves every
admitted reflected declaration once per build.  Whole-grammar
well-formedness now validates ids, ownership, and stratification through a
persistent integer set rather than performing a full id-list scan for every
child reference.  These changes preserve grammar order and exact ranks: the
routed four-function x86 control remains `TOTAL 1728`, `REGIONS 140`, `FORKS
63`, and witness rank `183`.

`typed_list_first.shard` pins theorem-backed first-result behavior, while
`region_probe.shard` pins fixed and forbidden regions, nested restrictions,
productive representative selection past a barren production, and preservation
of original hole ids.  The template-free x86 calculator task admits only a
plain reflected constructor scope, four integer atoms, and depth 20; it does
not provide a routed `TgScopeEnv` or a calculator control-flow skeleton.

#### Regular schemas and lazy occurrences

First-result search no longer needs to materialize the complete occurrence
grammar.  `meta/sketch` now separates a `SkSchemaGrammar`'s reusable production
states from structural occurrence addresses.  A schema alternative contains
an expression template plus child references `(local hole, edge, child
schema)`.  The address

```text
child(parent, edge) = 2^edge * (2 * parent + 1)
```

is positive and reversibly identifies the complete parent path.  Consequently
two positions governed by the same schema consult the same production policy
but remain distinct region choices.  No mutable occurrence allocator or eager
hole table is required.  Schema well-formedness checks template coverage,
edge ownership, and child-schema references; saturating productivity operates
once over the schema graph, including productive cycles with a finite base and
rejection of ungrounded cycles.

`typed_grammar` interns the full state that can affect production generation:
expected type, binder-type environment, remaining depth, scope zone, root-only
formation state, and inherited child exclusions.  Its existing environment,
route, reflected-signature, and formation machinery is reused to construct
each unique schema exactly once.  Equal schemas never imply equal choices.

SUPERPOSE exposes materialized grammars and schema grammars through the same
read-only demanded-alternative interface.  Representative construction now
recurses only through the chosen productive candidate instead of scanning the
whole occurrence grammar.  Counted census remains on `Grammar`; first-result
tasks use schemas for both semantic evaluation and theorem-backed narrowing.
Separable theorems are represented directly in schema formation states.

Residual matching in `meta/search` now consumes an `MsLanguage`: either an
exact occurrence grammar or a regular schema grammar.  Schema preparation is
exact without eager occurrence facts; a matcher resolves only the structural
occurrences it demands.  When a semantic pass still leaves the theorem plan
blocked, SUPERPOSE partitions that relation through the same lazy language,
fixing one productive occurrence choice at a time.  Exact cardinality and
citation-coalesced census partitions remain explicitly flat-grammar
capabilities rather than being approximated on a schema.

The depth-2 checked list search preserves its exact narrowing trace—four forks,
39 evaluator steps, and the independently chosen value `(Cons 0 (Cons 1
Nil))`—while replacing 13 occurrence holes with nine schemas.  The genuine
flat depth-20 four-function x86 calculator compresses 202,177 possible
occurrence holes to 136 schemas.  A one-region probe reaches its first semantic
fork in roughly four seconds rather than remaining in eager grammar setup.
At 1,000 regions its stable trace is 652 semantic failures, 652 terminals, 348
forks, and 134,057 evaluator steps.  Indexing schema lookup does not materially
change that runtime, which locates the remaining cost in semantic evaluation
and branch refinement rather than grammar construction.

`typed_observer_conjunctive_first.shard` pins the nonseparable case.  Its one
checked observer theorem relates two independent constructor positions, stays
deferred, and kills a lazy constraint region before the engine returns a clear
representative.  This is the residual-theorem counterpart of
`typed_list_first.shard`'s formation-pressure and occurrence-independence pin.

## Slice 0 — pre-arc refactors for the heuristic tier (2026-07-25, more-search branch)

RATIFIED DIRECTION (user, 2026-07-25): the next arc adds HEURISTIC search
engines (rip-up/re-route-class exploration, graded cost, region
perturbation) beside the exact tier, on the measured PIO evidence that
exhaustive settlement stops short of deep winners (~37.9k decisions, no
DME). The trust posture is unchanged — heuristic engines are exploration
policies; G4 stays the only certificate; the claim ladder (exact census /
first-result existence / heuristic existence) types what each mode may
pin. Before the arc: harden the seams the survey found soft. Landed:

- **0a (three commits):** the pre-SkRegion cc_* counting path, the CcR
  wrapper, the unread `memo` thread through all five drive entry points
  (plus the g_counts layers feeding it in three consumers), and the dead
  su_tpl/su_rep/su_member assignment-template family are DELETED; the
  legacy per-test rev battery (su_expect/su_test/su_tests/su_drive) is
  retired — su_run drives the pinned rev suite through the GENERIC query
  path (settlement bit-identical both depths; STEPS re-pinned 623/12,651
  -> 489/7,777). search_screen under first-result mode is now a loud
  refusal (it was silently dropped).
  SURVEY CORRECTIONS, recorded: (1) su_first_* is NOT a sk_region_first
  duplicate — it is the newer root-directed, schema-aware representative
  closure; its graduation into the region-algebra home belongs to the
  relation-aware region slice. (2) There is no killed-region list to
  release: long-run retention lives in the arena memo/node trie (rows
  from killed regions are never evicted) and the DFS sibling frontier —
  shard#19 updated; the release lever is a substrate-internal
  sa_forget-style memo scoping op (slice 0c).
- **0b-1:** the task-protocol hook scan. te_task_scan (typed_expr, wired
  at te_config so every engine passes through it) owns the KNOWN-HOOK
  table and the structural refusal matrix: unknown search_*-named fns,
  witness multiplicity (previously priority-shadowed silently), solo
  spine hooks, and flat vocabulary under search_environment all refuse
  loudly. The search_ prefix is now RESERVED for protocol hooks; task
  helpers renamed to task_* (10 files), 8 tasks shed vestigial flat
  hooks (all pins unchanged — the hooks were unread). protocol_probe
  pins the matrix; typed_expr and typed_superpose are ALSO check targets
  now (42902ae closed the tc_infer gap that excluded them).

- **0b-2:** the engine-internal task RECORD. te_load_task (typed_expr)
  resolves EVERY protocol hook exactly once into the TeTask record —
  protocol signatures, config, canon/observer/spine profiles, screen,
  result mode, narrowing strategy, drive budget, invoked target and
  witness — and derives the task's CLAIM-LADDER rung: TeClaim =
  census / empty (exhaustive absence) / first (existence under budget);
  the heuristic rung joins this enum later and can never feed census
  pins or absence claims. The mode x hook legality matrix lives in the
  loader (expect_empty⊗first, rank-witness⊗first, screen⊗first all
  refuse at load, messages unchanged); the only engine-side legality
  check left is typed_superpose's audit-flag⊗first (a CLI matter, now
  refused BEFORE the drive instead of after it). Both engines consume
  the record: typed_expr's driver chain collapsed onto it and the
  census engine now LOUDLY REFUSES first-result tasks (claim-type
  mismatch, previously a silent census); typed_superpose lost
  ts_result_mode/ts_drive_fuel/ts_theorem_first (moved into the loader
  as te_result_mode/te_drive_budget/te_narrowing_strategy + te_task_fuel)
  plus its whole ts_with_* resolution chain, the DEAD pre-schema
  first path (ts_find_space/ts_finish_first/ts_first_report — unreachable
  since the schema drive landed) and its te_space_count duplicate
  (~560 lines net). su_find_query stays in superpose.shard: it is a thin
  adapter over the shared su_find_query_prepared core, completing the
  drive/find x materialized/schema surface. transition_mine still rides
  te_protocol+te_config only (no record fields it needs yet); it joins
  the record when the heuristic hooks land, and is now a CHECK TARGET
  like the other engines. protocol_probe pins the claim ladder
  (every mode x witness cell + both screen-legality directions,
  refusal messages byte-exact). Every settlement pin bit-identical
  across the whole battery (census, audits, first-mode schema drives,
  deep pio_dme).

- **0c:** arena memo scoping — the release lever, plus the measurement
  that RESIZED the problem. Ops (superpose substrate, drives untouched):
  sa_forget_holes drops every row whose consulted-hole set touches a
  ripped hole (the perturbation-loop companion); sa_forget_mm flushes
  the whole table (an episode boundary); sa_mm_rows counts live rows
  (report-boundary traversal). All are VERDICT-NEUTRAL BY CONSTRUCTION:
  the memo is agreement-keyed, so a forgotten row can only cost a
  re-forcing — arena_probe pins the full contract (same-id zero-step
  hits, untouched-hole spare, fresh-id re-evaluation with re-record,
  flush sparing region-independent indirections). Rows also shed their
  stored consulted pair list (SuRow keeps key+result; the pair list is
  zip(SuMe holes, key), reconstructed at probe time) — whole battery
  bit-identical including STEPS.
  THE MEASUREMENT (800-decision pio_dme_free, depth 12, reports
  bit-identical old vs new): live-set peak 2.49 GB -> 2.30 GB (the row
  diet's real share: ~7.5%), but peak RSS 27.1 GB -> 26.5 GB (~2%),
  because RSS is HEAP EXTENT, not live data: ~326 GB allocation churn
  across 54 GCs, extent ~11x live, live_peak == live_last (the live
  set saturates). shard#19's ~12 MB/decision attribution to memo-row
  payload is therefore CORRECTED on the issue: the 67.6 GB CI plateau
  is allocator/GC extent policy in the NATIVE runtime (GC pacing /
  extent return), main-worktree scope — substrate hygiene cannot move
  it. The forget ops stand as the heuristic tier's episode/rip-up seam
  (live-set hygiene for long heterogeneous runs), not as an RSS fix.

SLICE 0 COMPLETE. The arc proper opens with RELATION-AWARE REGION
STATE built at the meta/sketch region-algebra layer (engine-agnostic by
construction), with PIO P5c-2 and the routing/regalloc task family as
its measured consumers. Heuristic-tier hooks (cost, neighborhood, seed)
join the TeTask record and te_hooks table when the first heuristic
engine lands.

## Arc opener — relation-aware region state (design RATIFIED 2026-07-25)

> **Outcome (recorded 2026-07-28, USER ruling):** the find-mode
> application of relational splits was REFUTED at scale and the trick
> SET ASIDE — frozen at current scope, no further investment, find-mode
> application closed permanently.  The census/exclusion win (the
> coloring probe's 15 exact splits) stands, and theorem-first STEERING
> emerged as the rung's real deliverable.  Full evidence and the ruling
> in the R5 record below.  This block is kept as ratified history.

The measured problem (this ledger's own boundary statement, and PIO
P5c-2's gating line): relations between holes — repeated-variable
equality in nonlinear rules, structural-distinctness guards — can only
be applied by ground refinement today, so the diagonal of `lt x x`
costs one region per value (the pinned EQ 3 + NE 6 is six regions and
four forks for two facts), and relation-driven splits multiply the
region frontier. The playground's cloned twin-grammar product is the
recorded dead end (cloned hole identities fragmented memo reuse).

**The design (user-ratified after review):**

- **Representation:** `SkRegion` gains a third field, `(List SkRel)`,
  with `SkRelEq`/`SkRelNe` over PARALLEL HOLE VECTORS (singleton
  vectors = plain edges). Vector `ne` is tuple-difference — the form
  that keeps the algebra CLOSED under propagation: when both endpoints
  of a deep `ne` fix the same alternative, the event descends to the
  sub-hole vector pair (a disjunction stays one event); `eq` descends
  pointwise (a conjunction splits into edges). Edges are between holes
  of the UNCHANGED grammar — no new hole identities, and relations
  (like forbids) never enter consulted-choice memo keys.
- **Exact counting or loud refusal, never approximation:** relation
  events partition into connected components; unrelated holes keep the
  ordinary product fold (masked to 1 at related closures). Per
  component: contract `eq` by union-find — v1 requires INDEX-ALIGNED
  DOMAIN ISOMORPHISM within a class (class count = the shared
  restricted domain; non-isomorphic domains refuse; intersection
  counting is the recorded extension) — then count `ne` by
  INCLUSION–EXCLUSION over ne events (exact for arbitrary graphs,
  2^events per component, explicit cap ~12 as a loud refusal).
  Endpoints must be live through fixed chains and no endpoint may lie
  inside another relation class's domain closure (drive-introduced
  relations satisfy both by construction; refusals guard hand-built
  regions). `rank`/`unrank` REFUSE relational regions in v1;
  `member` is direct (cube check + relation evaluation), not via rank.
- **Verdicts:** a new arm beside `MsBlocked` carries the blocked HOLE
  PAIR when the residual obligation is exactly hole-vs-hole equality;
  drives answer it with a RELATIONAL SPLIT (two children: ∧eq, ∧ne)
  instead of per-value forks. Anything not expressible as a pair falls
  back to ground refinement — soundness never depends on the new path.
- **v1 vocabulary is structural eq/ne ONLY.** Affine relations over
  literal-valued holes (the PIO delay-conservation quotient) are the
  NAMED FOLLOW-ON rung — the SkRel enum and per-component counter
  interface are designed for the extension; the P5c-2-cont measurement
  runs the delay-normal WRAPPER grammar where the delay gauge is
  declared away structurally and the firing relational pressure is the
  nonlinear MovOp family (structural eq).
- **Consumers (deviation from the ratified list, user-approved):**
  measured on PIO P5c-2-cont plus the two EXISTING relational pins —
  sort nonlinear (6851/1438/50450 + the d3 probe) and the x86
  transition window (742/372/25312 semantic-first, 625/411/23267
  theorem-first; its register-distinctness guards are regalloc-shaped
  ne pressure). A REAL routing/regalloc task lands with the first
  heuristic engine (it is that rung's benchmark); building a contrived
  one now just to tick the admission box was rejected.
- **su_first_*** graduates into the region-algebra home in this slice
  (the 0a survey correction), relational-aware, shared by both drives.
- **Heuristic seam:** the region release op (un-fix holes, keep/drop
  relations by policy) pairs with 0c's sa_forget_holes as the
  rip-up vocabulary. **TABOO = FORBIDDEN REGION (user ruling,
  2026-07-25):** the heuristic tier's learned-nogood store is a list
  of forbidden REGIONS (cube + relations) consulted by region overlap
  — in/undetermined/out with no predicate evaluation — with the
  relational region state as its representation language. Arbitrary
  `Candidate -> Bool` predicates stay where search_screen sits
  (closed-candidate audit acceleration): unproven, uncountable,
  unintrospectable code is sound nowhere else. A learned taboo that
  earns generality promotes into the exact tier through the existing
  miner → schema → checked theorem pipeline. Not prioritized; lands
  with the heuristic engine.

Rungs: R1 meta/sketch vocabulary+count+ops+probe pins; R2 meta/search
pair verdicts + relational partitions; R3 drive consumption + re-pins;
R4 su_first_* graduation; R5 measured consumers (re-pins + the
watchdogged P5c-2-cont deep run). Orientation of symmetric operands
(which gauge twin a census prefers) remains a separate reviewed
decision, out of scope here.

### R1 — the relational region algebra (LANDED 2026-07-26)

**What landed (meta/sketch).** `SkRel` (`SkRelEq`/`SkRelNe` over parallel
hole vectors) and the three-field `SkRegion`; structural wf; the
template-alignment walker (bijective sub-hole renaming — one walker
serving the iso door, class counting, and descent); the by-index
isomorphism door; three-valued pair evaluation (equal / differ /
blocked / descend); fixpoint normalization (eq fixes its open partner;
both-fixed deep pairs descend — eq pointwise, ne to the sub-vector as
ONE tuple event; a last ne pair against a fixed CLOSED template
converts to an ordinary forbid); the validated doors
`sk_region_relate_eq`/`_ne`, the relational split
(`sk_region_split_pair`), and the rip-up seam (`sk_region_release`,
keep/drop relations by policy — sa_forget_holes' region-side partner).
Exact counting: connected components over events; eq contraction by
recursive class counting through aligned templates (restrictions
intersect naturally at every depth); ne by inclusion–exclusion over ne
events (`count(ne_e ∧ rest) = count(rest) − count(allEq_e ∧ rest)`,
exact for arbitrary relation graphs); the unrelated product rides the
ordinary fold with endpoint factors masked to 1. `member` is now
DIRECT (the same first-match forward walk rank uses, without counts,
plus expr_eq over bound subtrees) for cube and relational regions
alike; `rank`/`unrank` refuse relational regions. The v1 boundaries
all refuse loudly: non-isomorphic domains at the door, an endpoint not
unconditionally live under the region's fixed chains, an endpoint
inside another endpoint's domain closure (reachable only by hand-built
regions — door-made relations cannot construct it), a component past
the 12-ne-event cap.

**Pinned (region_probe, second output line, all counts hand-computed
first):** EQ-3 (the diagonal is ONE region), NE-6, CLIQUE-6 (3! by
inclusion–exclusion, 2^3 terms), DEEP-4/12 (the (Atom | Wrap ·) domain
diagonalizes: 4 = 1 + 3), DESCEND-6 (both sides fix Wrap: the ne event
becomes the sub-pair tuple event, 9 − 3), PROPAGATE (choose h0=2 under
eq fixes h1=2 and discharges the event), FORBID-2, SPLIT-3+6,
RELEASE-6/9, and the five refusals. The relational first-productive
representative backtracks over related holes only (the clique's
greedy representative is pinned deterministic under the newest-first
event order). Whole existing battery bit-identical: region_probe line
1, the superpose rev pin (443/133/7,777 STEPS), the nonlinear/
constraint probes, typed_superpose as a check target (323/0).

**The prove blocker this rung surfaced (fixed on main, #20):** the 12
new fuel-measured fns could not mint descent sidecars — tools/prove's
whole SOLVE path was broken by a dangling
`(use (:: kernel term chars_of_sym))` (a bare-item use aliasing a
REDUCER PRIM through a module that never defined it), harmless until
dfb8631 keyed the prim table by CORE identity, invisible to CI because
check only replays. Fixed on main (02daf2f, cherry-picked here), the
lethal class swept (prove's line was the only instance), and two
follow-ups recorded on the issue: a loud refusal for bare-item uses of
nonexistent exports, and a corpus pin that actually SOLVES something.

### R2 — pair-carrying verdicts + relational partitions (LANDED 2026-07-26)

**What landed (meta/search).** The three matcher result types gained the
pair arm: `MsMatchBlockedPair` / `MsNMatchBlockedPair` /
`MsBlockedPair`.  The MsMatch arm carries a CONTRACT: it may be produced
only when the whole comparison is decided by exactly one hole-pair
equality (left == right IFF the two subtrees agree).  That contract is
enforced by the event lattice (`MsEqEvent` + `ms_eq_event_join`): the
equality walkers (list/arms/Let) accumulate one blocked event, and a
second DISTINCT event degrades a pair to its first hole — exactly the
hole the pre-relational walkers reported, so every degradation point is
bit-identical with the old behavior.  Only a repeat of the same
unordered pair survives the join.  The pair is born in `ms_equal`'s
both-open-heads case: the existing domain scan still runs first (an
all-unequal product still decides No without a fork — the documented
disjoint-shapes property), and only the mixed outcome upgrades from the
outer hole to the exact pair.  A partner behind a fixed selection chain
is left to the ordinary scan (sound, less sharp).

`ms_equal_partition` consumes the pair TERMINALLY via
`sk_region_split_pair`: the eq child goes whole to the equal list, the
ne child to the unequal list, no re-evaluation — licensed by the IFF
contract.  Any refusal (iso door, counting) falls back to the factored
ground step (`mse_ground`); soundness never rides the new path.
`MsEqPartitionOk` now carries ground forks and relational splits as
separate counters.  At the verdict layer the pair is a BRANCHING HINT,
not an IFF (msn short-circuits; rules and guards accumulate
first-event-wins via `ms_event_first`), produced by the repeated-
variable env comparison in `msn_match` and by `TrsDistinct` guards over
two open holes, and carried through `ms_conditioned_root_language` /
`ms_rules_root` / `ms_merge`.  Blocked-kind verdicts were never stored
as stable facts, so relations never enter prepared facts by
construction.  The alt-consensus scanners (spine/anywhere/equal-alts)
degrade nested pairs to the outer hole as before.

**R2 degradation discipline (R3's worklist).** The prepared plan
partition (`ms_partition_prepared`) and the superpose drives GROUND
pairs to their first hole: the prepared evaluator cannot see relations
yet, so a relational split there would re-block on the same pair
forever.  The flip points are grep-able: `ms_verdict_ground` (the
compatibility door, used at su_theory_partition) plus the explicit
`MsBlockedPair` degradation arms in mspp_partition_go and the two
drive verdict matches in superpose.shard.  typed_superpose/typed_expr/
pure_program treat a pair like Blocked on their error paths.

**Pinned (nonlinear_constraint_probe, all counts hand-computed first):**
the both-open equality verdict is `MsBlockedPair 0 1` direct AND
prepared; the diagonal partition is EQ3+NE6 in **1+1 regions, 1
relational split, 0 ground forks** (was 3+3 regions, 4 forks);
non-isomorphic overlapping domains ({A0,A1} vs {A1,A2}) refuse at the
door and ground-fall-back to 1+3/F2S0; the two-pair conjunction
(Pair(h0,h2) vs Pair(h1,h3), four 3-atom holes) degrades its second
pair, grounds h0 then h1, and splits each diagonal child relationally —
9+72 in 3+6 regions, F4S3; a TrsDistinct guard over two open holes
pairs through the conditioned verdict; and the PREPARED plan partition
still grounds (3+3/F4 — the degradation pin R3 will deliberately
re-pin).  Whole existing battery bit-identical: region_probe both
lines, rev_deep d3 (390/143/3,969), the ground rev pin (108, 7788),
constraint/guard/affine/int-order probes, typed_observer_conjunctive
audit, typed_superpose as a check target (323/0).

### R3 — relation-aware evaluation + drive consumption (LANDED 2026-07-26)

**What landed.**  The evaluator can now SEE a region's relation events, so
a relational split child makes progress instead of re-blocking on its own
pair.  meta/sketch gained the entailment decision `sk_rels_decide`
(`Eq`/`Ne`/`Unknown`): entailed equality is reachability over the
pointwise edges of eq events (each eq event is a conjunction); a ne event
entails a pair only when it is a SINGLETON (tuple-difference over two or
more pairs is a disjunction), lifted across eq classes; everything else —
including every vector ne — answers Unknown, the sound pre-relational
behavior.  meta/search threads `(List SkRel)` beside the assignment
through the whole verdict layer (equality walkers, nonlinear matcher,
guards, rules/spine/anywhere scans, `ms_check_facts`), and `ms_equal`'s
both-open-heads case consults the decision BEFORE the domain scan: an
entailed answer holds for every region member, so the IFF contract
carries it directly.  The region wrappers (`ms_check_region`,
`ms_check_prepared_region`, `ms_check_prepared_restricted`) extract the
region's rels; the bare-assignment entries (`ms_check`,
`ms_check_prepared`, `ms_conditioned_root`, …) pass none and stay
bit-identical.  The law that keeps memo reuse sound is now three-sided
and holds BY CONSTRUCTION: the semantic evaluator sees only
`sk_region_choices` (relations, like forbids, never reach
consulted-choice memo keys), constraint verdicts are computed fresh per
region (never memoized), and the prepare-time fact compilers evaluate
with NO relations (facts must be reusable under every region).

**Drive consumption.**  Both R2 degradation points flipped:
`mspp_partition_go` answers a pair verdict with `sk_region_split_pair`
and RE-QUEUES both children (the verdict pair is a hint, not an IFF —
relation-aware re-evaluation decides the pair inside each child), and
`su_theory_partition` does the same in the find/theorem path, with the
factored `su_theory_ground` as the door/count-refusal fallback.
`MsPlanPartitionOk` carries ground forks and relational splits
separately (mirroring the equality partition); at the SuS drive boundary
both fold into the existing branch-boundary counter.  The R2
compatibility door `ms_verdict_ground` is DELETED — its only consumer
was the flip site.  Progress terminates: every split adds the singleton
edge that decides its own pair in both children, and a child made empty
by later contradictory edges (a vector-ne violation the decision cannot
see) counts to zero exactly and is dropped.  `su_region_rep` now rides
`sk_region_first` instead of unrank-0 — identical on cube regions,
defined on relational passing regions, so the representative/AGREE gates
accept relational solutions.

**Pinned (all counts hand-computed first, confirmed on the first run).**
region_probe line 2 gained the decision pins
(DECIDE-TRANS/NE-LIFT/VEC-UNKNOWN/REFL).  The nonlinear probe pins
relation-aware checking end to end: an eq relational region drives the
prepared verdict to the cited Redex and a ne region to Clear
(REL-EQ-REDEX / REL-NE-CLEAR — the split-child progress property); the
distinct guard is decided by eq/ne relation events (GUARD-REL-EQ/NE);
the prepared plan partition now SPLITS the diagonal — 3+6 members in
1+1 regions, 0 forks, 1 split (the R2 degradation pin was 3+3 in 3
regions, F4) — and the non-isomorphic plan case falls back to ground
(1+3 in 1+2, F2S0).  constraint_superpose_probe gained the DRIVE-level
pin: a nonlinear diagonal rule before a constant-pass query settles as
FOUND 6 + KILLED/CONSTRAINED 3 in 2 terminal regions and ONE branch
boundary (the relational split), and the relational passing region
yields an in-region first-walk representative (DIAG-SPLIT … REL-SOL-REP).

**The measurement — the named census pins do NOT move.**  All three
ratified consumers are BIT-IDENTICAL under R3: the depth-2 nonlinear
sort census re-ran to exactly 6,851 regions / 1,438 forks / 50,450 steps
(FOUND 4, constraint-killed 87,834,384); the x86 transition window
(theorem-first) to exactly 625 / 411 / 23,267 with enumerative agreement;
the PIO transition window to exactly 45 / 44 / 2,408 (verified against a
fresh 653fa78 baseline run).  The arc opener's "expected to drop" prior
is refuted for these task shapes, and the reason is structural: a drive
split fires only when a PAIR VERDICT survives to a partition site, but
in the sort census the semantic-first schedule demands the operand holes
(forking them) before any pass-with-blocked-pair state arises, and the
two window grammars pack the guarded operands inside whole-instruction
alternatives, where nested pairs degrade to the outer-hole consensus by
design.  The machinery is exercised and pinned at every level (matcher,
partition, theorem path, drive, solutions) by the probes above; whether
a MEASURED consumer benefits is a property of task shape — R5 should
select for shapes whose pairs survive demand (theorem-first over
operand-level holes, e.g. the delay-normal PIO wrapper's nonlinear MovOp
family) rather than expect the existing pins to drop.

**Battery.**  Everything else bit-identical: region_probe line 1, the
superpose rev pin (443/133/7,777), rev_deep d3 (390/143/3,969), the
pure benchmarks, guard/affine/int-order/constraint/spine/symbolic
probes (the affine plan partition now also pins splits = 0), the
typed audits (append/observer/imp/pio-square/dme-window), first-mode
tasks, and the check targets (meta/sketch, meta/search 8/0, superpose
33/0, typed_superpose 323/0, typed_expr 323/0, pure_program 33/0);
new measure sidecars machine-minted by prove for the decision kit, the
threaded SCCs, and the su_theory SCC.

### R3b — find-mode goes count-free + the coloring shape pin (LANDED 2026-07-26)

**Why (the reappraisal after R3's null census result).**  The question on
the table was whether the relational tier still promises applicable gains
on routing-style problems.  Two findings answer it.  FIRST, the P5c-2-cont
wrapper shape-check: unlike the transition windows, the delay-normal
wrapper grammar is NOT whole-instruction-packed — its slot zones route
`PDFDrive`'s polarity (the MovOp) and hold, and `PDFJump`'s
condition/target/balance, into their OWN sub-zones, so the nonlinear MovOp
family can form hole pairs.  But the task is first-mode over REGULAR
SCHEMAS (57 schemas at depth 12), and the R1 relational region algebra and
split doors are flat-Grammar-only — so R4's schema-aware graduation is the
GATING rung for the R5 deep run to exercise relations at all, not optional
plumbing.  SECOND, the routing shape itself wins, measured below.

**Find-mode is now count-free end to end.**  R3 had introduced exact
child counting into `su_theory_partition`'s split screening — the one
counting site in the otherwise cardinality-free find path, and the site
where the inclusion–exclusion cap (~12 ne events/component) would silently
degrade dense relational components to ground refinement exactly where
relations matter most.  The screening now rides the first-productive walk
(`su_child_alive`): a successful walk witnesses inhabitance directly with
no cap; a walk failure is indistinguishable from genuine emptiness, so it
routes to the ground fallback rather than dropping the child — soundness
never rides the walk.  `su_region_first` bridges relational regions to the
sketch-side walk (`sk_region_first` backtracks over related holes;
`su_first_alt` is relation-blind until R4 graduates it), closing a latent
representative bug: the SuG-side walk could have emitted a NON-MEMBER
representative for a relational passing region.  Relational regions arise
only on flat grammars today, so `sug_raw` is the real Grammar on that
branch.  The census path deliberately keeps exact counting — census
claims need the cardinalities, and a cap refusal there falls back to
ground inside `mspp` as before.

**The coloring probe (tools/search/coloring_probe.shard) — the minimal
routing-shaped consumer, all pins hand-computed and confirmed on the
first run.**  Graph coloring is the shape the tier is for: per-variable
register holes, ONE nonlinear diagonal rule per interference edge
(`Cfg(..x..x..)` — a proper coloring is a normal form), and a
constraint-dominated drive, so every pair survives to the partition.
Census: the P4 path (4 vars × 3 registers) settles 24 proper colorings +
57 killed in FOUR terminal regions and THREE relational splits; the K3
triangle settles 6 (= 3!) + 21 in 4/3 — the drive-level twin of
region_probe's CLIQUE-6.  Terminal regions scale with EDGES, not with
value tuples.  Find: K6 × 6 registers succeeds PAST the census cap (the
all-ne leaf carries 15 ne edges in one component) in 16 regions / 15
splits, with the representative verified in-region; K4 × 3 (uncolorable)
terminates EMPTY through the walk-refusal ground fallback.  Verdict for
the reappraisal: the design stands — the R3 null result was about task
shape, and on the routing shape the relational schedule is the exact
O(edges) decision tree.  Controls stay bit-identical: both first-mode
typed tasks, the DIAG-SPLIT pin, the superpose rev pin (443/133/7,777),
the x86 window (625/411/23,267, agreement OK), and the driver check
targets (superpose 33/0, typed_superpose 323/0, pure_program 33/0).

### R4 — the graduated first walk + the schema-side relational algebra (LANDED 2026-07-26)

**Why this is the R5 gate.**  The P5c-2-cont consumer is first-mode over
REGULAR SCHEMAS; until this rung, the relational region machinery
(doors, normalization, the first walk) was materialized-Grammar-only, so
the schema find drive could only ground its pairs.  R4 makes the
relational algebra language-parametric and graduates superpose's
`su_first_*` family into it.

**The language interface (meta/sketch).**  `SkLang` (materialized
Grammar | regular schema grammar) answers the three questions the
relational machinery asks: a hole's instantiated alternatives
(`skl_alts` — flat entry lookup / `sk_schema_alts_at`), endpoint
isomorphism (`skl_iso` — the recursive flat walker / SCHEMA-ID EQUALITY,
which holds by construction: two occurrences of one schema id
instantiate index-aligned identical domains at every depth), and
per-alternative productivity (`skl_alt_productive` over `SkLangProd`).
The whole normalization SCC (`skr_pair_eval`, the eq/ne pair walkers,
`skr_norm_rels`/`skr_norm_go`, the ne-singleton forbid conversion, the
region-edit helpers) now takes the language; descent needs no
schema-special code — aligning two instantiations of one template pairs
the child occurrences positionally.  The doors and split gained `_lang`
entries (`sk_region_relate_eq_lang`/`_ne_lang`/`sk_region_split_pair_lang`,
`sk_region_norm_lang`); the materialized-Grammar names remain as
validating wrappers, so every existing call site is untouched.
Exact counting and rank/unrank stay materialized-Grammar-only —
first-mode never counts.

**The graduated walk.**  ONE demand-driven representative walk
(`sk_region_first_lang`) replaces both the flat entries-walk
(`skr_first_plain`/`skr_first_rel_*`, deleted) and superpose's
`su_first_expr` family (deleted): recurse from the root, at each open
hole take the first allowed PRODUCTIVE alternative and recurse into its
instantiation — touching only the selected candidate's occurrences,
which is exactly what a lazy schema language requires; relational
regions normalize, then backtrack over related holes only.  Per-hole
choices are traversal-order-independent, so flat representatives are
bit-identical with both old walkers.  `su_region_first` is now a thin
delegate over `sug_lang` (SuG retains its schema grammar, so the drives
hand the doors their language directly), and `su_theory_partition`
splits through `sk_region_split_pair_lang` — the schema find drive
shares the whole R3 split path.  `su_fork` normalizes RELATIONAL
children before pushing them (cube children skip at zero cost): fixing
a relation endpoint propagates — the eq partner gets the same choice, a
decided ne becomes a forbid, a contradiction is detected as Empty — so
no job carries a stale undischarged event past a fixed endpoint.

**Pinned.**  region_probe line 3 (REGION-PROBE-SCHEMA-REL): the eq door
propagates through the first walk (Pair(A0,A0)), ne separates
(Pair(A0,A1)), the split yields both children, a cross-schema relate
refuses at the door, and a DEEP fixed pair descends to its
child-occurrence tuple with the backtracker resolving it (the walk pins
the descended event's normalized order: it equates (6,10) and separates
(3,5) — a verified ne-region member).  constraint_superpose_probe's
SCHEMA-DIAG-SPLIT line drives the same nonlinear diagonal over a
regular schema end to end: the single-alternative root hole is
TRANSPARENT (a one-member domain chases through `ms_select`), so the
occurrence pair (1,2) splits at the empty region — CONSTRAINED 1
(the eq child cited), 2 terminal regions, ONE boundary, and the
relational passing region's representative Pair(A0,A1) from the
graduated walk.  Whole battery bit-identical: region_probe lines 1-2,
the coloring probe (K6 16R/S15 rides the graduated walk), DIAG-SPLIT,
rev (443/133/7,777), rev_deep d3 (390/143/3,969), the x86 window
(625/411/23,267, agreement OK), the PIO window (45/44), both
first-mode typed tasks, guard/affine/int-order/constraint probes, and
the check targets (meta/sketch, meta/search 8/0, superpose 33/0,
typed_superpose 323/0, typed_expr 323/0, pure_program 33/0).

### R5 — measured consumers: the deep-run A/B, the routing task, and the split verdict (COMPLETE 2026-07-28)

R5 was scoped as "measured consumers (re-pins + the watchdogged
P5c-2-cont deep run)".  What it actually measured, in order:

**The PIO 12k A/B (CI job 509, pipeline 158, 2026-07-27).**  The
deep-run law rode in from the PIO branch's two post-merge CI commits
(cherry-picked d114044/54fc99b): retention saturates ~67.6 GB
(67,639,812 kB byte-flat) around 6k decisions — FLAT RSS IS HEALTHY
SEARCH, not a stall; per-decision cost ~quadratic (T_CI(n) ≈
1.32e-6·n² min); a timed-out job is TOTAL LOSS (the engine reports
only at exit).  History: job 346 = the old engine's 38k rung died at
the 48h timeout; job 336 = old engine 12k, on-time 11,672s.  The R4
engine's 12k rung came back BIT-IDENTICAL to job 336 on every counter
(killed 8,191 / constraint-regions 0 / terminal 8,191 / forks 3,809 /
steps 8,225,025 / nodes 5,048,430 / exprs 186) at +0.4% wall — the
relational machinery costs ~0.4% when dormant, and the semantic-first
null is confirmed at depth: constraint-regions 0 means the theorem
layer never fires, so no blocked verdicts, no pairs, no splits.
**NEVER re-fire 38k under semantic-first** (= job 346's 48h loss).

**Theorem-first find scheduling (9ed1c2f) — and why PIO could not
consume it.**  The find path SILENTLY DROPPED `task_theorem_first_of`
(only the counted census drive consumed it); the fix adds
`su_find_query_schema_theorem_first` (a count-free
`su_theory_partition` prepass over the initial orbit) behind the
`su_find_query_schema_ordered` dispatcher, threaded via `ts_first`.
Probe-verified, smoke bit-identical on the False path.  Then the
deeper discovery: pio_dme_free carries ZERO theorem rules BY
DOCUMENTED DESIGN (window rewrites are unsound under absolute jump
targets; the delay quotient is baked into the grammar normal form), so
constraint-regions 0 is STRUCTURAL, not scheduling — a theorem-first
12k run would be job 509 bit-for-bit and was NOT fired.  USER RULED
(2026-07-27): build an actual routing problem instead of forcing the
PIO branch to demo the additions.

**The swap-network routing task (f1840cf) + the SPLITS counter
(5914ee4).**  tools/search/tasks/swap_route{,_model}.shard: 4
registers, adjacent transpositions Sw0/Sw1/Sw2, target = reversal
(longest element of S4, 6 swaps minimum), find-mode + theorem_first at
depth 7, two proven spine laws (nonlinear s.s cancellation ∀s;
Sw2.Sw0→Sw0.Sw2 disjoint ordering), 15-claim model tower.  GREEN:
FOUND the canonical 6-word s0s1s0s2s1s0.  A/B at depth 7:
theorem-first 13/116/168/742 (killed/constrained/forks/steps) vs
semantic-first 54/86/78/4,994 — the theorem layer prunes
pre-evaluation at 6.7× fewer steps.  The SPLITS counter separates
ground forks from relational splits on the find path (SuFind +
SuTheoryPartitionOk; counted SuS keeps its combined fold
deliberately); probe re-pins prove the attribution (coloring K6 =
forks 0 / splits 15; schema-diag = SPLITS-1).

**SPLITS-0 root cause and the skl_iso structural fix (b019bda).**  The
first swap_route runs showed splits 0 under theorem-first.  Loud-tracer
bisect: pair verdicts WERE produced at fixed-spine windows, but
`sk_region_split_pair_lang` refused every one ("relation endpoints
have non-isomorphic domains") and the silent ground fallback ate them.
Real cause: schema interning keys on REMAINING DEPTH
(`tg_task_state_eq` includes the depth), so a depth-independent leaf
alphabet gets a fresh schema id per spine level, and skl_iso's
schema-id-equality check refuses every cross-depth pair.  Fix
(door-side, option B): on id mismatch, skl_iso falls back to
STRUCTURAL state equality — identical alternative templates AND
identical refs including targets — parity with the flat branch; spine
states still correctly refuse.  First live splits: swap_route
theorem-first 59/78/174/SPLITS 6/4,542 steps, same BEST;
semantic-first bit-identical 54/86/78/0/4,994.  Splits are
STRUCTURALLY theorem-first-only: passing regions have their demanded
holes fixed, so no open pairs survive to the juncture under
semantic-first.  Economics flag raised immediately: the coarser
ne-children push work to the evaluator (742→4,542 steps vs the pure
ground-fork theorem tree).

**The k=4 scaling ladder (170aec4; CI job swap-route-deep; pipelines
165/166/167 + 170, 2026-07-28).**  swap_route5{,_model}.shard: 5
registers, reversal of S5 (10 swaps minimum), depth 11, cancellation +
three ordering laws, 30 new claims; the model reuses swr_swap_head +
invol via import.  Three arms: splits (more-search HEAD), ground forks
only (branch swap-ground = HEAD + revert of b019bda — THROWAWAY
baseline, never merge), semantic-first (SWAP_SCHED sed-deletes the
narrowing hook).  Budget 5k: splits RED exhaust 4,285/9,157/8,358
forks/SPLITS 10/326,459 steps ~56min; ground GREEN FOUND
656/9,205/9,205/0/34,004 ~19min (descending-runs canonical word);
semantic RED exhaust 1,276/2,467/1,257/0/141,015 ~5min.  Splits at
budget 20k: GREEN, the SAME word as ground — 4,772/9,157/8,439/SPLITS
STILL 10/358,082 steps ~55min.  **Final ladder verdict: identical
outcome; splits need >5k budget where ground needs <5k; 10.5× steps,
~2.9× wall; splits frozen at 10 across budgets 500/5k/20k — the
mechanism contributes nothing productive in find mode, pure
early-ne-child overhead.  The k-trend refutes the bigger-alphabet
hypothesis (k=3: 6×; k=4: 10.5× and outcome-changing).**

**USER RULING (2026-07-28).**  The relational-split trick is SET
ASIDE: frozen at current scope, no further investment (no guarded
relations), find-mode application closed permanently; its verdict on
census/exclusion shapes is deferred to the routing tier.
Theorem-first STEERING is the rung's deliverable — the schedule-level
win (ground-fork theorem-first FOUND where semantic-first exhausted)
is what the theorem layer was built for.  Not wasted: the arc's point
is opening new optimization problems and adding navigation tricks;
this one is banked.  Deferred cheap follow-up: a REFUSALS counter on
the silent split-fallback edges.

## The routing tier opens — the PCB demo slice (2026-07-28; FIRST GREEN RUN 2026-07-30)

USER STEER: focus shifts to the root routing problem; a PCB routing
demo is the problem statement.  Landed 40ab30a (+ CI fix eaefd3f,
iteration 2 in 6c602cb):

**The model (tools/search/tasks/pcb_route_model.shard).**  8×8 grid,
cells y*8+x, single layer, occupancy as a cell list, PMove =
MvN/E/S/W.  The load-bearing trick — new trick #1 of the tier — is the
BOUNDED WALK: `pcb_walk` checks the Manhattan floor against remaining
budget BEFORE demanding the move list, so the su engine gets
admissible A*-style pruning purely through lazy observation, zero
engine features.  No theorem laws BY DESIGN: interior path rewrites
are unsound under occupancy (the same structural fact as
pio_dme_free), so this instrument exercises the semantic/heuristic
tier, not the theorem layer.

**The driver (tools/search/pcb_route_probe.shard).**  Engine-as-
library: probes call `su_find_query_schema` directly; the model module
is ALSO loaded at runtime (resolve_closure + build_module_d +
prelude_ctors) because the su evaluator resolves the query's pcb_walk
call against a Module value.  Hand-built depth-bounded move-list
schema grammar (entries ascending by id).  Iterative deepening from
the Manhattan distance (+2 per level, 5 levels) supplies
shortest-first; the outer loop is the ratified LNS shape — greedy in
queue order, on failure rip the OLDEST routed net, requeue at the
back, retry the failed net first, rip-cap fuel.  Certificate = full
replay (every route re-walks legally against the keepouts) + a
duplicate-free cell union.  CLAIM LADDER: heuristic existence only —
never census pins, never absence claims.

**The budget-economics law (measured in blood, twice).**  A refutation
level explores a wander-tree EXPONENTIAL IN ITS SLACK (walks may
revisit cells; the Manhattan bound prunes only against remaining
budget) — size per-call budgets for the REFUTATIONS, not the finds.
Iteration 1 (flat 500k budget) sat at the ~67.6GB retention plateau
for six CPU-hours on the dev box before being killed — which also
minted the standing rule that UNVALIDATED ENGINE-RUN CONFIGURATIONS
FIRE ON CI, never the dev box (the generic ENGINE_RUN:1 job carries
the RSS watchdog + artifact capture).  The first CI run (pipeline 174,
flat 50k budget) reproduced the SAME plateau — it is structure-bound,
not budget-bound — ate the full 8h job timeout without completing, and
the pod kill lost the artifact.  Iteration 2 (6c602cb): the driver is
World-threaded to stream one evidence line per engine call
(net/depth/budget/outcome/steps/forks + ROUTED/RIP events); the flat
budget became a ramp (1000·2^level, sizing each level's give-up to its
slack); the CI job gained an internal watchdog-enforced deadline
(ENGINE_RUN_SECS, default 7h) below the pod timeout so artifacts
always upload.

**The measured slack ladder (pipeline 182, the first complete trace).**
Iteration 2 completed in ~6.9h and failed at the rip cap, delivering
the tier's first empirical cost curve: at ~1.4k evaluator steps/sec,
refuting slack 4 costs ~29k steps, slack 6 ~156k, slack 8 ~783k, and a
slack-8 FIND is out of reach (>10M steps per BUDGET give-up; the
su_find budget parameter counts DECISIONS, not evaluator steps —
~600× apart at depth 13).  The trace also showed the rip loop's
failure mode: F's length-13 detour (slack 8) unreachable → rip-oldest
evicts R → F re-takes row 1 → R blocked → an exact 2-cycle burns the
fuel.  And it measured a policy error: deepening past a BUDGET give-up
burned 20M of the run's 35M steps (the next level is strictly bigger).

**Iteration 3 (c267a2d) — sized from the curve, GREEN (pipeline 184,
2026-07-30).**  The wall shrinks to a single keepout at (4,2) so F's
forced detour is length 9 through row 3 (slack 4); levels 5→4 (worst
refutation slack 6); the ladder deepens ONLY on EMPTY — a true
refutation licenses the next level, a BUDGET give-up stops the net
(the BUDGET-STOP arm).  The run landed in a 325s CI job, line-for-line
the predicted story: F routes row 1 (d=5), R's ladder refutes d=1..7,
ONE rip, R routes len 1 + D len 2, F re-routes row 3 at d=9 (FOUND at
328,796 steps / 356 forks), certificate replay passes —
**PCB-ROUTE 8x8 KEEP-1 NETS-3 RIPS-1 WIRE-12 CERT-DISJOINT-OK**.
The demo is pinned in CORPUS_LONG (heuristic existence only, per the
claim ladder).  The routing tier has its problem statement, its first
working LNS driver, and its first cost model.

**Self-avoiding walks (1d124ab) — the model-side pruning rung, GREEN
(pipeline 191, 2026-07-30).**  The wander-tree curve above was
dominated by walks revisiting their own cells, so the model's
recursion now threads the departed cell into the occupancy: every
accepted walk is a simple path, never-reverse is subsumed (the
previous cell is the newest occ entry), branching ≤ 3.  Completeness
at every ladder rung is preserved by loop erasure on a bipartite grid:
erasing a loop from a legal walk yields a legal simple path with the
same endpoints, the same PARITY (grid cycles are even), and a subset
of its cells — so a depth-d EMPTY over simple paths refutes ALL walks
of length ≤ d, and the shortest walk is already simple.  Pinned by
`pcb_walk_revisit_pin` (E-then-W back onto the start: was `(Some 0)`,
now `None`).  MEASURED, line-for-line against pipeline 184 (same
instance, same budgets): per-slack-2 refutation growth fell from
~×5–9 to ~×2.2–2.4 (R's ladder 449/1,724/3,787/8,600 steps at slack
0/2/4/6, was 449/3,932/24,383/130,825 — the gain compounds with
slack: 2.3× at slack 2, 6.4× at slack 4, 15.2× at slack 6); the
slack-4 FIND fell 6.9× (328,796 → 47,983 steps, 356 → 62 forks); the
whole demo 550k → 98k evaluator steps, engine job 325s → 135s.
Slack-0 calls pay a small tax for the longer occupancy list (F d=5
FOUND: 1,933 → 2,341 steps) — pcb_mem is linear and the walk's own
cells now join the scan.  THE LAW THIS LANDS: observation-side
pruning is the heuristic tier's first-class lever — a one-line model
change moved the cost curve more than any budget policy could, and
the engine needed nothing.

**Goal-directed move ordering (eaa0554) — the branching heuristic,
GREEN (pipeline 193, 2026-07-30).**  Verified engine fact: the find
loop is a LIFO stack and `su_fork` pushes alternative 0 on top, so
schema alternative order IS the DFS exploration order — a branching
heuristic is a DRIVER-side grammar choice, zero engine change.
`pcbp_move_order` builds each net's move alphabet with the
toward-goal moves first (larger-|delta| axis leading) and the move
opposite the primary axis last; static per net, since the grammar
tracks remaining length, never the walk's cell.  MEASURED vs
pipeline 191: every EMPTY line BIT-IDENTICAL (a refutation exhausts
the tree in any order — the prediction and its self-check), every
FOUND cheaper: F's slack-4 find 4.0× again (47,983 → 12,054 steps,
forks 62 → 18), R 288 → 192, D 712 → 476, whole demo 98k → 61k
steps, engine job 135s → 60s.  Cumulative from the wander-tree
baseline (184): the slack-4 find is 27× cheaper (328,796 → 12,054),
the demo 9× (550k → 61k), the job 5.4× (325s → 60s).  The two levers
compose cleanly because they act on different quantities: pruning
shrinks the tree (EMPTY and FOUND), ordering only shortens the walk
to the first solution (FOUND alone).

**The refutation census (6110390 + 828216f) — kills carry attribution
out of the engine (pipelines 196/198, 2026-07-30).**  The engine
growth rung.  Before it, every killed region surrendered one count;
now `SuTFail` carries a `SuCensusKey` — a SHALLOW summary of the
value that failed to match `want` (mismatching head + first Int
payload) — and the find loop aggregates keys into a capped multiset
(16 rows + spilled counter) surfaced in all three SuFind arms.
Telemetry only: nothing soundness-bearing reads it, the counted
drive discards keys.  The model encodes attribution: `pcb_walk`
returns `PWalkR` (WalkBlocked carries the blocking cell;
WalkFloor/WalkOff payload-free so the pruning bulk buckets into
single rows; the goal check moved to the engine's want-match, so
wrong-end walks die at the Int field as end@CELL).  Second consumer
per the hygiene ruling: typed_superpose exhaust reports append
census-rows/spilled (swap deep-run attribution).  TWO MEASURED
LESSONS: (1) the lazy comparison kills at the FIRST differing
observation, so a census must not deep-force terminals — pipeline
196 then showed the raw arena peek blind (every payload b@?): a
refusal payload sits behind a REGION-DEPENDENT thunk whose forcing
consulted hole choices, so the value lives in the choice-keyed memo,
not the node.  The fix (828216f): bounded re-force (fuel 256)
against the current arena, DISCARDING the advanced arena — memo hits
replay in a few steps and the discard keeps the census strictly
behavior-neutral (pipeline 198: every steps= value bit-identical to
196; both pinned engine probes bit-identical locally).  (2)
Self-avoidance kills attribute to the walk's OWN cells, which are
unoccupied by construction and so never map to a routed net —
ownership filtering excludes them with no special case.

**Census-directed rip victim (620877f) — the driver rung, GREEN
(pipeline 200, 2026-07-30).**  A failed net's DEEPEST census hands
its b@CELL rows to the rip loop, which maps each blocking cell to
the routed net owning it, sums kill counts per net, and rips the
heaviest blocker (strictly-greater displaces → oldest wins ties;
zero attribution → rip-oldest fallback; the RIP line prints the
convicting weight).  The instance gained a decoy net A along the
bottom edge, routed first, making the OLDEST net the WRONG victim
for the first time.  The run played exactly as designed: R's ladder
census named F's row-1 cells (b@11/12/13/14, summed weight 5 for F
vs 0 for A and D), **RIP 9->14 w=5** convicted F directly where
rip-oldest would have burned a rip and a re-route on A, and the demo
ended **PCB-ROUTE 8x8 KEEP-1 NETS-4 RIPS-1 WIRE-14 CERT-DISJOINT-OK**
in an 82s job.  The 2-cycle failure mode measured in pipeline 182 is
now structurally answered: the loop rips what actually blocks.

**Step-denominated give-ups (98e673e) — the budget knob joins the
measurement unit, GREEN (pipeline 204, 2026-07-31).**  The find
loop's fuel counts DECISIONS, ~600× from evaluator steps at depth
(pipeline 182: 16k decisions = 10.1M steps) — give-ups sized in it
were guesswork.  `su_find_query_prepared` gains `step_cap` (0 =
uncapped, existing entries unchanged; checked per region pop,
overshoot bounded by su_efuel); `su_find_query_schema_steps` is the
capped schema entry; the PCB ramp converts to 32000·2^level
EVALUATOR STEPS, base sized so the tight rung (F's re-route d=7
EMPTY, ~38k steps at level-1 cap 64k) completes — a give-up should
only ever mean pathology, never a healthy refutation cut short.
Validation: every steps= value in the 204 trace BIT-IDENTICAL to
200, b= prints the step cap, same conviction (w=5) and final line.
TWO PROOF-LAYER GOTCHAS RE-MEASURED: (1) factoring the fuel check
behind a helper call makes the loop's measure obligations unsolvable
— nonneg needs the bare `(lt fuel 1)` hypothesis visible, so the
step-cap check sits as a SECOND guard under the untouched fuel
guard; (2) prove's committed-sidecar staleness (known, still
unresolved): a changed guard invalidates sidecar proofs but prove
skips existing entries — delete-then-resolve.

THE GROWTH SEQUENCE (user-approved 2026-07-30) IS COMPLETE through
rung 3: (1) goal-directed ordering, (2) refutation census +
census-directed ripping, (3) step budgets.  (4) best-first frontier
stays HELD until a measured need — ordering + census left the demo's
finds at ~15k steps.

**The model scales — multi-pin nets on a parametric board (053cb28),
GREEN (pipeline 208, 2026-07-31).**  USER STEER: "a single path
trace is just too easy."  The board goes parametric (W×H threaded
through step/manhattan/walk; instance 12×12) and nets become PIN
LISTS: the first pin seeds the net's TREE, every later pin is a TAP
routed to the goal set = any tree cell.  The walk checks set
membership at the STEP — landing on a goal cell with moves remaining
refuses (WalkCross, cell in the census; at minimal ladder depth no
found route can cross, so the check is pure pruning), landing as the
list ends is (WalkDone), the single ground want; pcb_dist_set
(min-Manhattan to the set) keeps the floor admissible; loop-erasure
completeness holds per tap.  TWO SOUNDNESS HOLES the direction
change exposed, both closed BEFORE first fire: (a) taps route FROM
the new pin, so a contested cell can be the START — which the step
loop never inspects; pcb_walk_from refuses an occupied start as
(WalkBlocked start), and a blocked SEED synthesizes the same census
row driver-side; (b) +2 deepening is UNSOUND for mixed-parity goal
sets (a tree spans both grid parities) — the ladder steps +1 unless
every goal shares the start's parity.  The certificate got strictly
stronger: replay rebuilds every tree tap by tap and RE-DERIVES all
cells from the moves, never trusting search-side bookkeeping.
MEASURED (pipeline 208, 72s job, line-for-line the designed story):
the occupied-start conviction costs 82 steps and ZERO forks per
ladder level (the walk refuses before the engine demands the hole —
the cheapest possible attribution), RIP 13 w=1 convicts F; the
3-pin T net routes trunk (d=5) then taps into the 6-cell tree at
its floor (d=3, mixed parity handled); board-size calibration: F's
slack-2 refutation 29,152 steps on 12×12 vs 37,984 on 8×8 —
SLACK-BOUNDED, NOT BOARD-BOUNDED, the curve transfers; the slack-4
find ~2× (28,917 vs 14,931 — more wandering room in DFS order
before the row-3 route).  Final:
**PCB-ROUTE 12x12 KEEP-1 NETS-5 RIPS-1 WIRE-22 CERT-DISJOINT-OK**.
Deferred by design: the whole-tree-as-one-query rung (a tree
grammar instead of a move list) builds on the goal-set observation
when per-query hardness is wanted.

**The congested instance — compounding rips, two-real-blocker
conviction (42c2b1b), GREEN (pipeline 215, 2026-07-31, job 637 at
210s / ~177k steps).**  Pure instance design on landed machinery
(only the rip-fuel constant 12→16 and the report string changed):
9 keepouts (a 2×4 component block at cols 2-3 × rows 4-7 plus a
corridor plug at (4,6)) and 7 nets — decoy A, corridor wall C
(5,2)-(5,8), west strap S (1,8)-(1,9), 3-pin bus B (keeps tree-tap
+ mixed-parity paths in the pinned demo), filler D, sandwiched T
(4,4)-(4,7), squatter E (5,1)-(5,3).  The trace played the designed
story LINE FOR LINE, every census row at its hand-counted value:

- **Rip 1 discriminates two REAL blockers.**  T's tap is walled
  (component W, C's column E, plug N); four EMPTY levels; the d=9
  census carries b@89:1 + b@101:2 (C, w=3) against b@97:1 (S strap,
  w=1) — the strap's single kill is the one hand-enumerated prefix
  [S,W,W,W].  RIP 29 w=3: the first conviction where the census
  weighs real-vs-real rather than culprit-vs-decoy.  Hand-verified
  by exhaustively enumerating the pocket's DFS prefixes — walls +
  the admissible floor choke each level to a handful of forks
  (d=9: 22 forks), which is what makes exact prediction feasible.
- **Rip 2 exists only because of rip 1.**  T re-routes d=5 through
  C's freed corridor; E's greedy d=2 route then squats C's freed
  SEED cell 29; C's retry hits the occupied-pin short-circuit (RIP
  17 w=1 with NO engine call — the synthesized census); E relocates
  west at d=4 while C detours col 6 at d=8.  The displacement
  cascade is the LNS dynamic the rip loop was built for, now pinned.
- **Census-cap sighting (first live spill):** the d=9 census hit the
  16-row cap with +7 spilled.  The conviction rows (97, 89, 101)
  survived because kill rows surface in DFS-encounter order and the
  owned kills come early; near the cap, weights can UNDERCOUNT — if
  a future instance's conviction goes wrong, check spill first.
- **Cost calibration, one prediction corrected:** forks stayed tiny
  everywhere (2-22) but steps did not scale with forks — T's d=9
  EMPTY cost 61,128 steps on 22 forks.  Per-fork evaluator cost
  grows with depth × occupancy-list length (linear pcb_mem scans
  over ~30 cells).  The congested run totals ~177k steps vs the
  old single-conflict demo's ~74k: congestion is cheap in FORKS
  (attribution keeps ladders shallow) but not free in steps.

Final: **PCB-ROUTE 12x12 KEEP-9 NETS-7 RIPS-2 WIRE-29
CERT-DISJOINT-OK** (WIRE = A2+C8+S1+B6+D3+T5+E4).

**The detour instrument — the frontier rung adjudicated on a
measurement (7d83021 + c8ed545), GREEN (pipeline 220, 2026-07-31,
job 647 at 2870s / ~5.05M steps).**  The HELD best-first frontier
(growth item 4) gets its measured-need gate:
tools/search/pcb_detour_probe.shard, a single-net instrument on a
CONCAVE TRAP FACING THE MOVE ORDER — a C-shaped cave (bar x=7
y=2..6, arms (5,2)(6,2)/(5,6)(6,6), mouth west), net (10,4) seeded,
tap from (1,4) east under order [E,S,N,W]; the through-wall
Manhattan floor cannot see the cave is closed; min route exactly 15
(slack 6, around either arm).  A single-gap straight wall was
rejected on paper: after a wall block the order's secondary move
HUGS the wall to the gap — concavity defeats wall-following.
Driver change: pcbp_route_all/pcbp_route_net thread levels+budget0
as arguments (behavior-neutral; the congested demo passes the old
constants).  Instrument NOT pinned (48-min job; instrument, not
demo) — check target only.  Two-shot on CI per the standing law:
pipeline 217 BUDGET-STOPped d=13 at a 1.024M cap (the paper cost
estimate was ~20x low; measured ~1.9-2.6k steps/fork at depth) and
the caps were resized once from that evidence (base 256k -> 4M).
Measured (220, structural story line-for-line):

- **EMPTY curve 14,031 -> 458,855 -> 4,301,216 steps (12 -> 238 ->
  1,666 forks) at slack 0/2/4** — x33 then x9.4 per slack-2; the
  census shows the trap working (b@53 mouth re-entry thrash, arm and
  bar rows, deep spills).
- **FOUND d=15 = 277,155 steps / 116 forks — CHEAP, the mechanism
  prediction corrected.**  The instrument was designed to price
  find-level mislead; measured, the cave SELF-LIMITS at the find
  depth (slack spent entering leaves no wander room, exits are
  floor-killed) and the south detour is order-aligned (S precedes
  N).  The exponential is NOT at the find level: 94.5% of the run
  is pre-find EMPTY refutation.
- **Frontier WITHOUT dominance: NO WIN, measured.**  An f-ordered
  path-space search expands the same admissible prefixes the EMPTY
  levels do; the ladder's re-expansion overhead is only ~11% (the
  d=13 tree is 90% of the EMPTY sum).  IDA*-style deepening is
  already near-optimal in path-space — the bare frontier rung
  should NOT be built.
- **Frontier + first-arrival cell dominance (the Lee/A* pair): the
  measured ~30-40x.**  66 cells with f <= 15 on this board
  (hand-derived: west block 28, x=0 column 5, cave 6, arm flanks 4,
  bar column 2, east side 21) vs 2,032 engine forks = ~31x in
  expansions, ~38x in steps at measured per-fork cost — and
  refutation collapses from path-count to cell-count.  This is
  admission tier 3 (state dominance = hashprune's checked
  successor) with a price tag: find-mode may take dominance
  UNCHECKED (replay stays the only certificate), but the EMPTYs'
  exact-refutation status needs the CHECKED form (per-domain
  soundness proof; on the grid, the distance-field argument).

Final: **PCB-DETOUR 12x12 KEEP-9 NETS-1 RIPS-0 WIRE-15
CERT-DISJOINT-OK** — and the adjudication: the rung worth building
is DOMINANCE-ENABLED best-first, not the bare frontier.

**Dominance-enabled best-first — the A* drive (05701c6), GREEN A/B
(pipeline 223, 2026-08-01, job 653 at 2849s): 20x steps, 52x
expansions.**  The adjudicated rung, built as a DRIVE POLICY over
existing components (the ratified heuristic-tier shape — no engine
rebuild).  `su_find_query_schema_astar`: the LIFO job stack becomes
a deterministic skew-heap frontier (f asc, then g DESC — deeper
first among ties — then push sequence) with a first-arrival closed
set.  The engine stays domain-blind: the task supplies a SCORE
TEMPLATE expression rendered per region by `su_cut_expr` (decided
choices instantiate their alternative templates recursively; every
OPEN hole becomes the task's CUT expression — applied to the
template itself, this inlines the decided prefix in one walk),
evaluated hole-free in a FRESH DISCARDED arena (the census re-force
discipline; hook steps fold into steps= so A/B cost stays honest)
and decoded structurally as (ctor g h key).  key = Some cells joins
first-arrival dominance, None abstains — dead prefixes must abstain
or score last, so a poisoned key can never close a live state.
TRUST POSTURE: the hook steers ORDER and PRUNING only; query/want
evaluation stays the sole arbiter of found/killed and replay stays
the only certificate.  Dominated pops count into killed and surface
as a synthesized `dominated` census row; **exhaustion with
dominated>0 returns SuFindBudget, never SuFindEmpty** — an
incomplete search cannot masquerade as an exact refutation (CHECKED
dominance, the per-domain soundness proof that upgrades pruned
exhausts back to exact, is the named follow-on).

- **Two design bugs caught on paper before first fire:** (a) the
  ladder grammar's separate move-alphabet hole cuts to Nil = a list
  where a PMove belongs — the astar arm's grammar bakes the move
  INTO each alternative (5 alts per list entry; edges ascend, the
  schema validator requires per-entry uniqueness), so every open
  hole is a tail-list hole and the cut is always type-correct;
  (b) a goal-ended OPEN prefix renders identically to its own
  Nil-completion under the cut — a key there lets the parent close
  the state its found-candidate child needs — so exact goal
  arrivals ABSTAIN (terminal states carry no dominance value).
- **MEASURED (223; the ladder arm re-ran BIT-IDENTICAL to 220 — the
  regression self-check for the threading + engine additions):**
  ASTAR-FOUND d=21 len 15 at **248,408 steps / 39 forks / 59
  dominated** (census sym:59), route replay-certified, one query
  replacing the whole 4-level ladder.  Ladder total 5,051,257 →
  248,408 steps = **20.3x**; expansions 2,032 → 39 = **52x**; the
  single query is cheaper than the ladder's final FOUND level alone
  (277,155).  Expansions landed UNDER the hand-derived 66-cell Lee
  bound — the goal pops mid-band, before the f<=15 shell exhausts.
  Optimality of len 15 rides the hook's admissible h
  (heuristic-tier property; the certificate is replay as ever).
- **Second consumer (hygiene rule), NAMED at admission:**
  cost-ordered program search — the x86 window tasks with score =
  instruction count (h = 0 is admissible), dominance key = a
  machine-state fingerprint; wiring deferred to the swap/x86
  measurement slice.
- **Known limit, recorded:** cell-keyed dominance under prefix
  commitment can miss (the first-arrival prefix may block the only
  continuation) — acceptable for heuristic existence; the LNS loop
  treats it as any failed find.  The instrument stays a check
  target, not a pin (the ladder arm alone is ~48 min); whether the
  astar arm becomes a slim pinned demo is an open call.

**The time axis opens — the space-time formulation (10634ab), GREEN
(pipeline 228, 2026-08-01, job 663 at 170s).**  USER STEER: the
intended end-problem is Tenstorrent-class — a grid of compute
engines + configurable DMA over a NoC, optimized OVER TIME; the gap
analysis named four axes (time, cost objectives, capacity sharing,
placement coupling) and ratified time first.  The formulation landed
with ZERO engine changes — the task-parametricity the A* drive
promised, now proven by its second live consumer.

**The model (tools/search/tasks/pcb_time_model.shard).**  Occupancy
becomes SPACE-TIME CELLS st = t*(W*H) + c in a plain Int list — the
entire Int-set machinery (pcb_mem, census payloads, ownership scans)
transfers verbatim, and a census row now names WHERE and WHEN.
PTMove gains TmH (hold); packets occupy one cell per tick and VANISH
on arrival (transfer semantics — freed cells are what make
time-sharing the point); keepouts stay purely spatial; conflicts are
VERTEX-only (head-to-tail convoys legal, as on a streaming NoC).
The walk checks blocked-BEFORE-goal — an arrival-slot conflict is
real in time.  Two spatial results RETIRE soundly: self-avoidance
(space-time revisits can be necessary; dropping the pruning needs no
lemma — the schema enumerates all move lists outright, so EMPTY
stays an exact conditional refutation with the loop-erasure argument
simply gone) and parity deepening (a hold flips arrival parity;
ladders step +1 unconditionally).  The admissible Manhattan floor
SURVIVES (distance falls at most 1 per tick, 0 on holds).  V1 scope
cuts, each a deliberate lean: vertex conflicts only (swap check = a
follow-on knob), two-pin nets (multicast-in-time = a later rung),
injection at tick 0 with leading holds occupying the source, unit
packets (durations/flit-trains = where capacity rungs meet time).

**The dominance payoff.**  For one net against fixed trajectories,
(cell, tick) is the COMPLETE state — every arrival there has cost
exactly t and an identical future — so the A* drive's first-arrival
dominance becomes PURE DUPLICATE DETECTION: zero loss, the recorded
prefix-commitment limit vanishes for this task, and per-net search
collapses from exponential path-count to at most W*H*T states.
Same-cell-different-tick arrivals correctly never dominate (earlier
is not better until the corridor clears).  This is also the
checked-dominance rung's cleanest venue: domination by state
IDENTITY, not inequality.

**The instrument (tools/search/pcb_time_probe.shard).**  8x8 board,
FULL wall at x=4 with a single gap at (4,3), two west-east nets —
an instance SPATIALLY UNROUTABLE BY CONSTRUCTION (both wires need
the gap cell; the spatial demo would rip-cycle to FAIL) that routes
once time is real.  A's d=5 route is UNIQUE, so its trajectory is
hand-fixed (row 3, ticks 0..5) and B's conflict schedule is
deterministic.  Measured (228, the predicted story line for line,
32/0 + 111/0 checks, all pins hand-derived before first fire):

- **Ladder arm:** B refutes d=4/5 on geometry (min 6 spatial moves
  through the gap), d=6 on the TIME conflict — every length-6 route
  needs the gap's only free west neighbor (3,3) at t=2, where A is
  IN TRANSIT.  The census convicted A's transit sts b@90 = (2,3)@t1
  and b@155 = (3,3)@t2 at every level (prediction refined: the
  shallow-dying prefixes mint them per level, not only at d=6); no
  spill (12 rows < 16 cap); wall rows tick-resolved.  d=7 FOUND len
  7 with EXACTLY ONE hold — parity-provable in advance (6 spatial
  moves minimum, 7 spatial breaks parity, 2 holds leave 5) — B
  convoying ONE CELL BEHIND A down row 3 and crossing A's goal cell
  at t=6, after A vanished.  EMPTY curve 3,937 / 11,957 / 62,493
  steps (4/10/42 forks), FOUND 32,229 / 24.
  **PCB-TIME 8x8 KEEP-7 NETS-2 MAKESPAN-7 TICKS-12 HOLDS-1
  CERT-DISJOINT-OK.**
- **Astar arm (one query per net, dmax 11, st keys):** A 22,102
  steps / 6 forks / 0 dominated (predicted exactly); B 89,489 / 15
  forks / 17 DOMINATED — more duplicate (cell,tick) arrivals merged
  than states expanded, the dedup load real even at this scale.
  Same report line.  B's query vs B's ladder: 1.24x steps, 5.3x
  expansions; trivial A is CHEAPER on the ladder (score-template
  render cost per pop at dmax 11).  At this instance's slack the
  arms are near step-parity — the divergence pricing remains the
  detour instrument's 20.3x; THIS instrument's deliverable is the
  axis (spatial-infeasible instance routed, hold chosen by search,
  convoy legal, time-resolved census).
- **Cross-check:** the ladder's EMPTYs at 4/5/6 are EXACT (no
  dominance in that arm), certifying min arrival 7 for this
  priority order; the astar first goal pop agrees.  Joint
  (priority-free) optimality is NOT claimed.
- **Replay certificate strengthened for time:** both trajectories
  re-walk against keepouts AND each other (arrival conflicts refuse
  inside the walk), st union duplicate-free, everything re-derived
  from moves.

The demo is pinned in CORPUS_LONG (~170s job; heuristic existence
only).  NO rip loop in v1 — this instance never rips; the
rip-in-time rung wants an instance that needs one (census
attribution already carries tick resolution, so conviction
transfers).  Named follow-ons from the ratified sequencing: rung B =
real costs + min-cost claims (weighted moves, MIN arm, checked
dominance for the bound's EMPTY side), then capacity/negotiated
congestion and the global-objective LNS on that substrate; nearer
term, a rip-in-time instance and the swap-conflict knob.

**Rung B — weighted costs + min-cost claims (7ca9985), GREEN
(pipeline 231, 2026-08-01, job 669 at 236s): the predicted story
line for line, the report line character for character.**  The
ratified next step after the time axis: real move prices, a
MIN-COST claim with an exact bound side, and the checked-dominance
upgrade that lets a dominance-pruned exhaust claim EMPTY.  This is
the substrate the capacity/PathFinder rung and the global-objective
LNS ride on.

**The model (tools/search/tasks/pcb_cost_model.shard).**  Weighted
pricing on the space-time walk: cmove per compass hop PLUS a
per-cell toll on the cell entered, chold per hold; the walk carries
a COST BUDGET in place of the tick budget, with the admissible cost
floor cmove*manhattan checked at entry and the arrival step's own
price checked before WalkDone (a tolled goal hop can overdraw a
budget the floor accepted; mid-walk overdrafts need no check —
cmove >= 1 makes the next entry floor positive against a negative
remainder).  The toll list is the capacity seam: PathFinder-style
negotiated congestion prices shared cells by inflating exactly this
input (in the model now; its first live instance arrives with that
rung).  Parameter requirements recorded: cmove >= 1, chold >= 1,
tolls >= 0; tolls are paid on entry only.  The A* score hook: g =
accumulated cost, h = cmove*manhattan — admissible AND CONSISTENT
(a toward-move drops h by exactly cmove and costs at least cmove;
holds and away-moves only raise f), so the first goal pop is the
minimum-cost route within the depth cap.  KEY CLAIM ARITHMETIC: a
depth cap dmax with cmin*(dmax+1) > B makes a budget-B enumeration
exhaustive (longer routes cost more by arithmetic), and cost bounds
are MONOTONE — so ONE EMPTY at C-1 is a complete min-cost bound for
a cost-C witness, no ladder climb.  Zero engine changes on the
model side; the walk/score are the third live consumers of the
task-parametric drive.

**The engine change — su_find_query_schema_astar_checked.**  A
dom_exact flag threaded through su_astar_go; the ONE branch that
changes is heap exhaustion: with the flag, a genuinely drained
frontier returns SuFindEmpty even with dominated>0 (fuel and
step-cap exits stay SuFindBudget in both entries; the plain entry
passes False and reproduces old behavior exactly — time probe
regression 111/0).  Calling the checked entry is an ASSERTION of
the task's dominance-soundness pedigree, documented per model: for
the cost walk, (1) continuation behavior factors through (cell,
tick, remaining budget, suffix) and is MONOTONE in remaining
budget; (2) under consistent h, the first pop at a (cell,tick) key
carries minimal g (equal-f ties safe: a strictly cheaper arrival
still queued behind an ancestor has strictly smaller f), i.e.
maximal remainder — so every completion of a dominated arrival is a
completion of the dominator, and pruning loses no route of cost
<= B and no minimum.  Same epistemics as the model-side floor
arguments backing ladder EMPTYs (documented hand argument + model
claims), NOT a mechanized engine proof — and the instrument
cross-checks it against the dominance-free arm.  Second consumer
named at admission (hygiene rule): the cost-ordered x86 window
search (g = instruction count, h = 0 trivially consistent,
machine-state fingerprint keys), where a checked EMPTY is a
proven-minimal-program claim — the superoptimizer-shaped payoff.

**The instrument (tools/search/pcb_cost_probe.shard).**  The 8x8
wall gains a SECOND gap — near (4,3) contested by A, far (4,6)
free but two moves longer — and weights cmove=2, chold=5 make the
OBJECTIVES DISAGREE: B's fastest route is the one-hold convoy
through the near gap (makespan 7, cost 17), its cheapest is 8
spatial moves (cost 16, makespan 8; parity kills 7-move arrivals,
holds cost more than the detour).  The Pareto pair is the
Tenstorrent-shaped tension (latency vs energy) on a 2-net toy.
Measured (231; 48/0 + 141/0 checks, 16 instance pins hand-derived
before first fire — including the DOMINATED TWIN [E,H]/[H,E], both
live at st 163 g=7, hand-guaranteeing the checked arm exercises
dominance):

- **The fast arm re-runs the time story on the two-gap board:** B's
  unit ladder EMPTY 3,733 / 11,324 / 59,100 steps (4/10/42 forks)
  at d=4/5/6 with the transit convictions b@90 + b@155 at every
  level, FOUND d=7 at 30,463 / 24 — cost re-derived 17.  (d=6 was
  62,493 on the one-gap board at the same 42 forks — the shorter
  keepout list trims steps.)
- **The cost find:** one A* query on the cost score popped a
  cost-16 len-8 route at 111,246 steps / 14 forks / 14 DOMINATED —
  as many duplicate (cell,tick) arrivals merged as states expanded.
  Both cost-16 families pinned legal at budget 16 exactly; either
  is a valid witness (cost/len/holds are family-invariant).
- **The bounds, three grades live:** A's EMPTY at 9 = 143 steps,
  ZERO FORKS, census floor:1 — the entry floor refuses before the
  move hole is demanded, the cheapest possible MIN bound.  B's
  exact EMPTY at 15 (ladder grammar dmax 7, no dominance) = 47,206
  / 34, and its census is the SAME conviction set as the makespan
  d=6 EMPTY — the cost bound re-convicts A's transit,
  tick-resolved.  B's CHECKED EMPTY at 15 = 61,807 / 9 forks / 14
  dominated — the checked entry's FIRST LIVE EMPTY, agreeing with
  the exact arm (the in-driver cross-check would fail the run
  otherwise).  Witness costs are verified = bound+1 in-driver
  before the claim prints.
- **Honest reading:** the checked arm spent 3.8x fewer expansions
  (9 vs 34) but 1.31x MORE steps than the exact ladder at this
  scale — score-template render cost per pop, as on the time
  instrument.  The rung's deliverable is the CLAIM MACHINERY (a
  dominance-pruned exhaust returning a cross-checked exact bound),
  not a speedup at toy slack; divergence pricing remains the
  detour's 20.3x.  Where the checked EMPTY pays is where the
  exact ladder is unaffordable — the detour-scale boards and the
  x86 window minimality claims.

Final, both claims verified in-driver:
**PCB-COST 8x8 KEEP-6 NETS-2 FAST-MAKESPAN-7 FAST-COST-17
CHEAP-COST-16 CHEAP-MAKESPAN-8 MINCOST-A-10 MINCOST-B-16
DOM-CHECKED-AGREE CERT-DISJOINT-OK.**  Pinned in CORPUS_LONG
(~236s job).  All MIN claims are per priority order (A fixed
first); joint optimality is NOT claimed.  Named follow-ons: the
capacity rung (negotiated congestion pricing the toll input,
PathFinder-style) and the global-objective LNS on this substrate;
nearer term, rip-in-time and the swap-conflict knob still stand.

**The capacity rung — negotiated congestion (eab0e0c), GREEN
(pipeline 234, 2026-08-01, job 675 at 192s): the negotiation story
exact, the report line character for character.**  PathFinder's
core idea transplanted onto the cost substrate, and the toll
input's first live consumer: nets are routed INDEPENDENTLY — no
priority order, no hard occupancy between nets, no victim
selection — and conflicts are PRICED instead of forbidden.  Where
the rip-up driver picks a victim by census conviction, negotiation
lets the cost structure pick.  ZERO engine changes and ZERO model
changes: the whole rung is one driver + instance probe
(tools/search/pcb_cap_probe.shard), which also means the
engine-addition hygiene rule (second consumer) is not triggered.

**The loop** (driver-level, iteration fuel, mirrors the rip loop's
shape): each round routes every net with one plain-astar cost
query against the CURRENT toll list, occ = Nil — other nets are
invisible except through prices (h = cmove*manhattan stays
admissible and consistent under any nonnegative tolls, so the
first goal pop is still the toll-inclusive minimum).  Conflicts
are read off the trajectories at SPACE-TIME granularity — a
shared st is a real conflict, the same cell at different ticks is
legal time-sharing, not congestion — and every conflicted CELL's
toll is inflated by the number of nets meeting there (+2 on the
2-net board).  Pure history, updated between rounds; no
present-sharing term.  Convergence = zero shared sts; the
certificate is rung B's cross-occupied cost replay at budget =
claimed cost exactly, with costs re-derived from moves and tolls
STRIPPED — negotiation tolls are search scaffolding, never the
instance.  Claim grade: heuristic existence only.

**The instrument: rung B's two-gap board reused deliberately**, so
the negotiated outcome is directly comparable to the certified
minima.  Measured (234; 156/0 checks, 15 instance pins; every pop
of rounds 1 and 2 hand-simulated in advance against the heap law —
f asc, g DESC, earlier push seq first, children pushed in
alternative order):

- **Round 1 (tolls empty):** both nets take their unconstrained
  optima and collide: A [E,E,E,E,E] cost 10 at 29,575 steps / 6
  forks / 0 dominated, B [E,N,E,E,E,S] cost 12 at 39,112 / 7 / 2 —
  fork AND dominated counts exactly as the pop simulation
  predicted.  B rides A's row-3 corridor AT THE SAME TICKS: SHARE
  n=4 cells[ 27 28 29 30 ] (sts 155/220/285/350, t2..t5), the
  exact predicted conflict set — which also confirms the witness
  identity (any other cost-12 member shares cell 26 too).  Each
  cell's toll += 2.
- **Round 2 (tolls 27/28/29/30 = 2): the prices discriminate by
  alternative cost.**  A's near family is FORCED through all four
  tolled cells (28's only free neighbors are 27 and 29; 30 is the
  goal — the goal toll is unavoidable even on the far detour:
  18 near vs 24 far) — A re-found [E,E,E,E,E] (true cost 10,
  unchanged) at 103,883 / 19 forks / 32 dominated.  B's best near
  dodge still pays 27/28/29 (18); the far gap pays nothing (16) —
  B diverted to a len-8 cost-16 far route at 143,296 / 21 forks /
  24 dominated (hand simulation said 20/22 — one expansion and two
  duplicate pops adrift in the f=16 band; every claim-bearing beat
  exact).  Zero shared sts: CONVERGED iters=2.
- **The punchline the board reuse buys:** the NO-PRIORITY
  negotiated outcome lands exactly on the per-priority minima rung
  B certified — the driver asserts COST-A = 10 and COST-B = 16
  (bound + 1) before printing, an instance consistency stamp, not
  a new optimality claim.  A stayed because its detour is dear, B
  moved because its detour is cheap, and nobody chose a victim.

Final: **PCB-CAP 8x8 KEEP-6 NETS-2 ITERS-2 TOLLED-4 COST-A-10
COST-B-16 MAKESPAN-8 CERT-DISJOINT-OK.**  Pinned in CORPUS_LONG
(~192s job, ~316k engine steps).  Named follow-ons from this rung:
the general N-net negotiation loop and a congested-class instance
head-to-head vs the rip driver (price-directed vs census-directed
conflict resolution on one board); SPACE-TIME tolls (keying PToll
on st, letting negotiation separate nets in time, not just space —
the spatial toll cannot express "contested at tick 3 only", so
spatially-forced instances currently need the conflict to resolve
spatially); a present-sharing term (Gauss-Seidel within a round).
The global-objective LNS on this substrate remains the ratified
next step.

**The global-objective LNS (2eb4d79), GREEN (pipeline 239,
2026-08-01, job 685 at 371s): the trace line for line, and the pop
simulation EXACT on all twelve inner queries.**  The ratified LNS
shape's cost-driven successor: every prior driver is
feasibility-first and order-taking (each net optimal given its
predecessors, the total never consulted); this is the first driver
on the substrate where the SUM is the acceptance criterion.  Zero
engine changes, zero model changes; one probe
(tools/search/pcb_lns_probe.shard).  It also delivers the banked
astar-LNS migration: the LNS inner solver is the A* cost tap.

**The loop.**  State = a priority order.  A move swaps an ADJACENT
PAIR and re-routes the whole order with the A* cost solver under
hard occupancy; accept iff the total TRUE cost strictly improves; a
full sweep with no accept = LOCAL-OPT.  The pair swap is the
smallest genuine neighborhood — ripping a single net and
re-inserting it against everyone is a provable no-op (its old route
stays feasible, and every cheaper route was already blocked by a
predecessor, a subset of what it now faces).  Greedy strict descent
is monotone: no cycling, no tabu store needed yet.

**A finding, learned deriving the instance:** under vertex-only
conflicts, every net with ANY free adjacent cell — including its
own previous cell — can yield by a two-move wiggle, so the unit
yield price is UNIVERSALLY min(chold, 2*cmove).  Asymmetric yield
PRICES are impossible (the tunnel instance first sketched for this
slice fails — the retreat-step wiggle is unblockable), and two-net
order swaps therefore always tie.  Strict order gaps need
asymmetric yield COUNTS — cascades.  (This is also why the two-gap
board is order-tied at 26, checked on paper.)

**The instrument: a ZERO-KEEPOUT cascade.**  Three straight nets,
pure net-net interaction: A (4,0)->(4,5) vertical crosses B
(1,3)->(6,3) at 28@t3 and C (0,4)->(6,4) at 36@t4, both
equidistant.  A's two blockers are CONSECUTIVE on its path, so ONE
+2 delay by A clears both crossings (+4); any order routing A
before B lets A commit and pushes TWO yields onto others (+8).
All six order totals hand-derived (bases 32): A-before-B = 40,
B-before-A = 36 — and 36 is the joint optimum on paper (A and B
both want 28@t3, someone pays >= 4; 36 achieves exactly one
yield).  The greedy-natural initial order (A,B,C) is globally
wrong from the wrong end: the net that should absorb the delay is
the one greedy sends first.  Measured (239; 156/0 checks, 14
pins, every pop of all twelve queries simulated in advance):

- **INIT (A,B,C) = 40:** A straight 20,249 steps / 6 forks / 0
  dominated; B's row-2 dodge (cost 14) 53,696 / 8 / 3; C taxed to
  16 — blocked at 36@t4 AND fenced by B's dodge at 30@t7 —
  92,257 / 9 / 4.  Every count exactly as simulated.
- **Swap ACCEPT -> (B,A,C) = 36:** B straight 6/0; A's one-delay
  wiggle [S,S,E,W,S,S,S] (cost 14, taking 28@t5 and 36@t6) 8/3;
  C rides FREE at 12, 7/0.
- **Re-swap REJECT at 40** — re-run BYTE-IDENTICAL to INIT
  (20,249 / 53,696 / 92,257): the determinism self-check,
  predicted in the header.
- **Tie REJECT: (B,C,A) = 36** — the strict gate refuses equal
  totals; C second is cheaper (41,993 — smaller occupancy to
  render) and A third finds the same one-delay witness class.
- **LOCAL-OPT best=36**, INIT/BEST assertions and the n-net
  cross-replay certificate (each net at budget = claimed cost
  exactly against the union of the others, costs re-derived,
  duplicate-free st union) all passed in-run.

Final: **PCB-LNS 8x8 KEEP-0 NETS-3 INIT-40 BEST-36 ACCEPTED-1
REJECTED-2 LOCAL-OPT CERT-DISJOINT-OK.**  Pinned in CORPUS_LONG
(~371s job, ~599k engine steps).  LOCAL-OPT is local w.r.t. this
neighborhood AND the heuristic inner routes; the joint-optimum
argument is paper, not machine.  Named follow-ons: the JOINT
product query (two nets' routes as one schema candidate — the
exact-tier joint MIN bound that would upgrade the paper argument);
larger destroy sets + annealing-style acceptance (the moment the
tabu=forbidden-region ruling activates); a congested many-net
instance where the descent takes several accepted moves; the
weighted/lexicographic makespan-cost objective.

**Task-authoring gotchas banked while building the swap + PCB
instruments:** te_import_ctx admits only bare item uses as candidate
heads (in-file type decls are invisible — models live in sibling
modules); proof-DSL case-on refuses parametric type args like
(List Int) — use induct, which also substitutes; compute inlines
helpers into match trees destroying rewrite patterns — use controlled
(unfold F lhs)(reduce lhs) ×2 rounds; the Euclidean division surface
name is `ediv` (pairs with `mod`); schema entry ids must be
nonnegative and strictly ascending.
