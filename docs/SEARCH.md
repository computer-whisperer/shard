shard program search — SEARCH.md
=================================

> **STATUS (reset 2026-08-22): RECORD + LAW.** the meta-search and more-search arcs are CLOSED; the lock-step section ("The lock-step arc", near the end) is the engine's law, merged to main 2026-08-22; LS-law 3's decision is re-scoped as #27. The backlog is the GitHub issue tracker (labels `arc:coverage` / `parked` / `debt`; the goal = #23, the prune arc = #24) — any "next arc/rung" pointer below is history unless it names an issue.

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

> **Moved to [records/SEARCH.md](records/SEARCH.md) (2026-09-02, the ledger split: LAW stays here, dated RECORDS live under docs/records/ with their section numbers unchanged).** Cited as `SEARCH.md §…` everywhere; open records/SEARCH.md for 8. The arc record (slice ledger)… through The routing tier opens — the PCB demo sl… (5 sections).

## The lock-step arc — joint code+proof search over ISA models (opened 2026-08-02)

STATUS: this section is the arc's ledger home (home ratified in
conversation 2026-08-02); the rulings recorded below were made by the
user 2026-08-02 unless dated otherwise. Development home: the
more-search worktree (branch more-search). Cross-thread coordination
per the thread-division agreement (memory file), as AMENDED by the
certificate reframe below. Arc slices are numbered LS0… (this ledger
already has an unrelated "Slice 0").

**Mission (user, quote-grade).** "The main goal is advancing what
shard can refine by open-ended search rather than what dedicated
fixed-function lowering and proving machinery is designed to do. A
pipeline for automatically compiling any shard application to any ISA
model is the holy grail here, and I see this as a small step towards
that problem." The lock-step framing: finding candidate refinements
is one hard problem, finding candidate proofs is a second, and
finding them TOGETHER ought to be slightly easier than one then the
other — each side prunes the other. Side projects taken
opportunistically; the main goal governs scope calls.

**Trust posture (unchanged).** In-search proof signals are pruning
heuristics; the replayed proof through the untrusted-proof path is
the ONLY certificate. Nothing in this arc grows the trusted core.

**Measured precedents the arc stands on:** theorem-first steering is
proof-shaped pruning and was outcome-changing at 6.7x (swap_route,
R5); the typed imp expression search found the first
better-than-existing refinement (model-fragment spike, above);
straight-line ISA piece theorems close by ONE compute-both (ISA.md
§7); the A* cost drive with checked dominance turned exhaustion into
proven-minimality machinery (the PCB cost rung); the census channel
attributes refutations. Emission speaks the ratified conversion
dialect (CERT.md §10) from birth.

### LS-law 1 — the certificate reframe (RULED 2026-08-02; supersedes the fixed-point discipline)

A committed proof sidecar stands on REPLAY: it goes through the full
untrusted-proof path at check time, and that is the entirety of its
validity. "The current tools/prove would emit something different"
is provenance trivia, not staleness — sidecars aging relative to the
solver is NOT a defect. The event that matters is a sidecar FAILING
REPLAY (a kernel/checker change), and the check gate already owns
that. Consequences, recorded as law:

- Solver-reproduction is a test of the TOOL, scoped to its pinned
  fixtures (the prove-regen corpus pin, 42af308: two fixtures
  re-solved from scratch each corpus run, byte-identity demanded —
  that gates tool health and determinism). It is NEVER a repo
  invariant over committed sidecars.
- The repo-wide fixed-point re-solve obligation after ladder changes
  is RETIRED — and with it most of the prove single-owner-window
  rationale in the thread-division agreement: a prove change is an
  edit to one tool file plus its fixtures, coordinated like any other
  shared file. (The 2026-08-02 absorption pass stands as one-time
  cleanup of provenance CONFUSION — the #21-verification mystery
  diff — not as invariant installation.)
- REJECTED-because: byte-identical regen as a standing repo invariant
  was considered and rejected — validity lives in replay, and the
  invariant would tax every solver improvement with repo-wide churn,
  blocking exactly the incidental prove rework this arc wants free.
- Accepted residue: regenerating a file's sidecar re-renders the
  whole machine-owned file, so a regen forced by one new claim churns
  neighbors cosmetically. Replay is the referee.

### LS-law 2 — the prove decomposition: the oracle kit, not the loop (Fork 2 CLOSED 2026-08-02)

The standing ruling "tools/prove gets composed of parts — one part
naturally lives in meta/" resolves to extracting the MINIMAL common
piece. The deciding contrast, recorded: prove's solve loop is
generate-and-test over a hand-curated finite candidate list — fixed
cheapest-first order, first success wins, binary check_sequent as the
only signal (plus one bespoke residual-reader, the case-on discovery
rung) and NO exhaustion semantics: it can fail to find, never certify
absence. The engine searches schema-defined open spaces with a cost
drive, checked dominance, census attribution, and checked-EMPTY
exhaustion that upgrades "didn't find" into a claim.
REJECTED-because: lifting prove's loop/enumerators into meta/ would
enshrine a lesser duplicate of the engine and sap the pressure to
build proof search into the engine itself — the arc's mission.

What graduates — **meta/proofgen, the oracle kit** (name RULED
2026-08-02; `meta/solve` REJECTED-because: it collides with the solve_*
vocabulary that STAYS in prove, and proofgen rhymes with the imp/impgen
vocabulary-vs-generator precedent):

- the ATTEMPT KERNEL: sequent + module + candidate proof text →
  checked verdict via check_sequent — in-process, World-free,
  sidecar-free;
- the TERMINAL CLOSER MOVES: refl / compute / simp / arith as
  primitive proof moves;
- the FARKAS CERT CLOSER: searched multiplier vectors over the
  sequent slot table, kept CLOSED like an LP call — callers invoke
  it, they never enumerate its steps.

Explicitly NO enumeration policy and NO ladder ordering — the
module header must say so (the meta/rewrite "mechanism, not policy"
precedent). meta/lin (COMPUTED certs, the generator-side kit) stays a
separate discipline; no merge — computed-because-the-generator-knows
and searched-against-the-checker are different animals that happen to
emit the same claim form.

tools/prove KEEPS: its ladder as tool-private POLICY over the moves;
the clever rungs (conditional citation rww_*, premise mining mn_*,
induction/case assembly, case-on discovery disc_*); measure solving
(ms_*); and the shell (auto-decl discovery, RunState file-order
threading, sidecar render/persist). Behavior unchanged; the corpus
pin gates the extraction as zero-delta at the tool level.

### LS-law 3 — the convergence hypothesis and the reach benchmark

Hypothesis (named, not committed): prove's clever rungs are
hand-tuned instances of engine-general mechanisms — conditional
citation and mining of theorem-plan steering, case-on discovery of
refutation-attributed residual reading. As engine reach grows they
retire one by one, until tools/prove is shell + policy over engine
calls.

The meter: **the sidecar corpus as benchmark.** Every goal the ladder
ever won (the committed sidecars), PLUS the 13 opaque-type entries
the 2026-08-02 absorption measured as beyond the ladder (get_set_* /
cat_len / slice_len_exact / lookup_insert_eq / insert_shadow /
str_valid / str_len_nonneg / blen_is_len / list_of_cat / gs_i /
gso_i), PLUS frnd_pos_wf (shard#18: 2092 formatted lines, a 7-deep
premised cascade — measured UNSOLVED (search exhausted) by the full
ladder 2026-08-02). A harness feeds these goals to the engine and
scores reach — pure instrumentation, ZERO coupling to prove.

The gate: rewriting tools/prove on the engine is DEFERRED until the
engine re-solves an agreed fraction of the ladder's corpus AND at
least one goal the ladder cannot touch (the fraction is set when the
harness exists and the baseline is measured — deliberately not now).
REJECTED-because (rewrite-now, 2026-08-02): a production consumer
exports stability pressure onto the exact surface this arc is about
to bend (joint code+proof states in schema space), and the engine
starts behind the tuned rungs, so the ladder would persist as
fallback — two systems, now coupled. shard#18 RESTATED: the
acceptance test is ENGINE reach — grow it until frnd_pos_wf falls;
the benchmark is the progress meter.

### The pilot (Fork 1 RULED 2026-08-02): x86 window minimality

Wire the cost-ordered window search to the A* drive: candidates =
instruction windows over the x86 model fragment; score = instruction
count (h=0, admissible); dominance = checked machine-state
fingerprints; per-candidate verification = piece theorems closed
through meta/proofgen moves (straight-line pieces close by one
compute-both); exhaustion at a budget level = checked-EMPTY, which is
the proven-minimality claim for the window class. Deep runs fire on
CI only (standing law). A design doc precedes any build — instance
vocabulary, claim forms, corpus pinning, CI placement — presented for
ratification as LS3 opens.

### Side explorations + the slice ladder

**Regalloc is SANCTIONED as a side exploration here** (user: the main
thread ruled it out of Arc B's staging — it deserves its own arc —
but kicking the problem around here first is fine). Fit note:
regalloc is coloring-shaped, and census/exclusion coloring was the
relational-split machinery's one demonstrated win shape.

- **LS0 — the prove/impgen anneal: COMPLETE 2026-08-02.** Record:
  shard#21 fixed 19527ac (the 5→7-arg apply_rewrite_with_env drift;
  prove was executed nowhere = the blind-spot class); sidecar
  absorption ec4a67c (7 files to fixed point; 13 opaque-type
  refusals = the benchmark's near tier); the prove-regen corpus pin
  42af308 (the corpus now EXECUTES prove; #21's class dies in CI);
  impgen #22 fixed e22f179 (refuse-not-red on revisited symbolic
  addresses; no safe-duplicate grain exists — the one-sweep law is
  body-level). Single-owner window opened and closed same-day;
  pipeline #250 green at e22f179.
- **LS1 — extract meta/proofgen** (the oracle kit above). Code motion +
  import rewiring, tool behavior unchanged, corpus pin green.
  Ordinary same-file coordination (no special window, per LS-law 1).
- **LS2 — the reach benchmark harness**: feed the sidecar corpus's
  goals to the engine, measure the baseline, set the LS-law 3 gate
  fraction. Instrumentation only.
- **LS3 — the pilot**: design doc → ratification → build.

**LS2 RECORD (LANDED 2026-08-02).** The harness = tools/reach
(run-mode): per claim/fulfills goal of each target file, five bounded
attempts (the meta/proofgen moves + the farkas closer) against the
EMPTY theory, verdict per goal
(refl|compute|simp|arith0|farkas|OPEN|NOGOAL|PARSEFAIL), TOTAL per
file. Fulfills goals are recovered from the target + same-module
mod.req.shard (the dir-form mod.req/ expansion is not covered and
reports NOGOAL). Instrumentation only — reads, attempts, writes
nothing.

The measured POLICY-FREE FLOOR (busy box, counts not timings), 20
benchmark files, 368 goals: **164/368 (44.6%) close on bare moves**
(133 compute + 19 farkas + 12 arith0; zero NOGOAL/PARSEFAIL).
Restricted to the LADDER-WON corpus (the 196 claim-keyed sidecar
entries): **120/196 (61.2%)** — the ladder's policy earns the other
76 (unfold chains, lemma citations, induction, cond/mining). The far
tier: **0/13 opaque-type entries** and **frnd_pos_wf OPEN** —
measured, matching the absorption findings. Notable rows: std/word
74/86 (the defining-equation fulfills tier is compute-dominated);
std/float kit 1/68 (the float kit is almost entirely
beyond bare moves). One scoring artifact: kernel/reader's sidecar
doc-comment yields a phantom "NAME" key — score against real entry
names only. CI: the reach moves-floor pin rides the default corpus
tier (0/4 cond_mine + 5/16 auto_demo exact — a deliberate closer
change updates the pin with it; anything else drifting it is a bug).

**LS-law 3 gate fraction — RULED 2026-08-02: the lean adopted — 90%
of the ladder-won corpus + at least one far-tier entry.** (Baseline
context: bare moves are free — any engine backend inherits 61% of the
ladder corpus by construction — so the gate sits well above the
floor. The non-adopted option's cost text stands as the
REJECTED-because record.)
- *90% of the ladder-won corpus + at least one far-tier entry
  (ADOPTED).*
  Demands real search-side synthesis (the open 39% is exactly the
  citation/induction/chain shapes) and the far-tier close proves
  capability the ladder lacks. Cost: the rewrite starts before full
  parity — acceptable because policy stays in the tool and the ladder
  remains available as one policy during migration.
- *100% parity + at least one far-tier entry.* The clean
  strictly-dominates story. Cost: the last wins are quirky
  rung-specific shapes (the div-facts arm, disc case-on discovery);
  chasing the tail delays the rewrite for goals that stay served
  either way.

**LS3 DESIGN (drafted 2026-08-02; RATIFIED AS DRAFTED 2026-08-02,
both fork leans adopted — build opened as LS3-i).**

*Naming.* The pilot's unit is a **SPAN** — a straight-line XInstr
sequence (no XBlock/XLoop/XBr/XBrIf/XCall/XRet/XSyscall). "Window" is
already load-bearing x86 vocabulary (X86.md §23: the MEMORY window
[xmemlo, xmemhi)); reusing it for instruction sequences would collide
with a ratified law's name.

*Object and task shape (the imp_expr precedent).* A span task module
supplies only: the instruction VOCABULARY (species subset × register
subset × immediate universe), the TARGET span, the PROBE set (pinned
input vectors: Regs × Mem smallcases), and a depth/length budget. The
engine side stays generic — no task hands the engine its answer.

*Drive wiring (the pre-admitted second consumer).* The cost rung
admitted this exact consumer at su_find_query_schema_astar_checked's
birth: g = span length, h = 0 (trivially consistent), dominance keys
= the machine-state FINGERPRINT (final Regs+Mem behavior vector over
the probe set, trap/exhaustion loud). Dominance-soundness pedigree,
documented here per the hygiene rule: spans are straight-line and
deterministic, so continuation behavior factors EXACTLY through the
intermediate machine state — any completion of a dominated arrival is
verbatim a completion of the dominator, and equal-fingerprint arrivals
at greater-or-equal g lose nothing. (Cleaner than the PCB case: no
budget monotonicity argument needed, the factoring is exact.)
Expected ENGINE CHANGES: ZERO — walk + score + fingerprint are a new
task-parametric consumer; any missing seam becomes its own announced
slice, never an inline engine edit.

*The two-phase verdict (the lock-step thesis in miniature).*
In-search, fingerprint equality on the probe vectors FILTERS
candidates — heuristic, untrusted, exactly the claim-ladder posture.
Every surviving find must then close its PIECE THEOREM: an xeval_seq
equality in the peephole form — tail-polymorphic, length-normalized
fuel with the exact cost delta (same-fuel replacement at the
exhaustion boundary is false — peephole.shard's header law), premises
from the fixed schema v1 (nonneg registers, reg_code disequalities)
— attempted through meta/proofgen moves plus a CITATION POOL seeded
with the peephole laws (the xpp_import pin already proves the search
tier can cite them). A find whose theorem does not close is REPORTED,
never accepted — the proof side prunes the code side, which is the
arc's thesis operating.

*The claims.* A ruled find emits: (1) the equivalence theorem
(kernel-replayed, the only certificate); (2) on checked EMPTY at
every length below the winner: the PROVEN-MINIMALITY record — scoped
honestly to the task's declared vocabulary, operand universe, and
probe-respecting candidates, in the PCB claim grammar (bound = the
EMPTY level, witness cost verified = bound+1 in-driver before the
claim prints). Instance pins hand-derived before first fire; deep
runs CI-only (standing law).

*Seeds.* LS3a — the peephole REGRESSION seeds: the xor-pair-cancels /
xor-self-absorbs shapes, already hand-proven; search must find AND
close them (the imp_expr witness-gate pattern: a known-certified
witness gates non-emptiness). LS3b — REAL MATERIAL: spans mined from
std/sha256/impgen_x86_out.shard (species mix measured 2026-08-02:
XStore8 711, XBin32-add 636, XBin32-sub 526, XMovRR 359, XLoad8 251,
XBin32-mul 149 …) at modest scope (length 2-4, small register set) —
a strictly-shorter proven-equivalent span, or its proven-minimality,
is the pilot's payoff either way.

*Success criteria.* (a) LS3a seeds found + closed end-to-end;
(b) at least one LS3b span improved-or-proven-minimal under a checked
EMPTY; (c) the proof-attempt telemetry (which spans close on bare
moves vs need citations vs refuse) feeds the reach benchmark — LS3
measures the same gradient the LS-law 3 gate is scored on.

*Slices.* LS3-i: the span task engine + LS3a seeds, corpus-pinned.
LS3-ii: LS3b real-material instances + the measured record here.
LS3-iii (only if demanded by refusals): citation-pool/move growth —
each landing measured against the reach baseline.

**LS3 forks — RULED 2026-08-02, both leans adopted:**
- *Fork LS3-A — first-fire scope.* (1) ADOPTED: LS3a regression seeds
  ONLY in the first landing, LS3b as its own slice (the witness-gate
  discipline wants the regression arm green before real material —
  and the PCB record shows first-fire pins are cheapest to derive on
  known shapes). (2) REJECTED-because: both arms in one slice pays
  off earlier, but a red LS3b instance would then block the whole
  landing.
- *Fork LS3-B — the proof side of found spans.* (1) ADOPTED: attempt
  via proofgen moves + the peephole citation pool, growing the
  pool/moves only on measured refusals (this IS the open-ended-search
  mission; growth lands as ordinary shared-file work under LS-law 1).
  (2) REJECTED-because: proof text from per-species-pair templates
  (generator style) is predictable, but it is exactly the
  fixed-function machinery the mission statement de-prioritizes, and
  its coverage would stop at the template table.

**LS3-i RECORD (LANDED 2026-08-02 — the span task engine + the LS3a
seeds; first fire pending on CI).** The task model =
tools/search/tasks/x86_span_model.shard: fingerprint = outcome tag +
ALL FIFTEEN registers per probe file (trap/exhaustion loud), under a
FIXED caller-supplied fuel — deriving fuel from the span would force
the candidate's spine structurally and defeat semantic narrowing;
XspScore g = length / h = 0 / key = fingerprint (structurally
decoded, the model owns its vocabulary); the checked-dominance
pedigree and its MEMORY-FREE VOCABULARY requirement live in the
model's header. The driver = tools/search/x86_span_probe.shard: nine
baked XOR templates over {RAX, RDI, RSI} (tail-only holes, so the
score render's Nil cut stays type-correct — the pcbt_grammar_a
discipline), checked A* find at cap = target length (the target
itself is the witness gate), checked EMPTY below the winner, and the
phase-2 piece theorem constructed as goal TEXT (schema v1: concrete
instructions, symbolic f/m/tail/rs/mem, one nonneg premise per
distinct register operand in first-occurrence order), parsed against
the model file's own scope, attempted through bare moves then pg_cite
over the pool {xseq_xor_pair_cancels, xseq_xor_self_absorbs} with
discharge-assignment enumeration ((2n+1)^k candidates, first hit
wins). meta/proofgen grew the CITATION CLOSER — pg_cite +
pg_premise / pg_premise_compute / pg_texts_sp — moves, not policy;
the pool and the enumeration live in the probe (LS-law 2 held).

MEASURED lesson banked at the pin: a forward (compute both) premise
discharge OVER-REDUCES — it unfolds rget into a stuck match on
symbolic rs, after which the goal premise no longer matches. The
generic fix is pg_premise_compute: rewrite the goal's premise
BACKWARD into the True side (rl rhs), then compute BOTH sides to the
common normal form — absorbs defined-head mismatches (the xsrc case)
with no stop list.

Local validation (all green 2026-08-02): probe closure 392/0 incl.
both hand-derived piece pins (byte-for-byte the theorems the driver
constructs); the phase-2 smoke closes both seeds' theorems by
citation at runtime (PAIR → CITE-xseq_xor_pair_cancels, SELF →
CITE-xseq_xor_self_absorbs); prove fixtures regen byte-identical;
reach floor pins unchanged (0/4, 5/16); model + probe
shardfmt-canonical. Trust posture: the pool's theory entries are
admitted Proven on the corpus gate on peephole.shard, goals parsed
from the loaded closure's own source bytes (stated in the probe
header).

**THE FIRST FIRE'S FINDING (CI pipeline 266, 2026-08-02 — the
cut-render aliasing kill).** The first fire returned FIND-EMPTY on
the PAIR seed (witness gate broken, 191,895 steps / 10 forks), and
the census named the killer exactly: `h@False:45 sym:dominated:46` —
45 + 46 = all 91 spans, with the WINNER among the dominated. The
diagnosis (reproduced locally byte-for-byte, licensed by the CI
measurement): the score render replaces open tail holes with the cut
expression (Nil), so a partial region and its own Nil-completion
render IDENTICALLY — the parent pops first, inserts the shared
fingerprint key into the checked-dominance closed set, and the closed
set then kills the actual winner as a duplicate arrival. pcb's score
hook never hit this because its goal arrivals ABSTAIN (key None).
The fix is the same discipline made goal-aware: xsp_score now takes
the goal fingerprint and abstains on a match, so winners never enter
the closed set; on NON-goal keys the parent/child aliasing is benign
— the aliased Nil-child's complete fingerprint IS the closed key,
which differs from the goal, so the kill is itself the refutation,
and the extensions live on in the parent's other forks (the model
header carries this as the ALIASING LAW, with pins: score_gh /
score_keyed / score_abstain). Post-fix, the cap-2 PAIR find pops the
winner at 20,438 steps / 1 fork.

**THE OFFICIAL RECORD (CI pipeline 268, 2026-08-02, ENGINE EXIT 0,
peak RSS 2.43 GB, 216 s job — the exact predicted report, and
step-count-identical to the local validation run, so the engine is
deterministic on this task):**
  X86-SPAN PAIR TARGET-LEN-2 FOUND-LEN-0 EMPTY-VACUOUS
    PROOF-CITE-xseq_xor_pair_cancels CERT-FP-OK   (20,438 steps / 1
    fork — the g=0 pop is the winner)
  X86-SPAN SELF TARGET-LEN-2 FOUND-LEN-1 EMPTY-AT-0
    PROOF-CITE-xseq_xor_self_absorbs CERT-FP-OK   (find 193,915
    steps / 10 forks; the checked EMPTY at cap 0 = 1,843 steps / 0
    forks)
  X86-SPAN LS3A OK
Both success criteria of the LS3a arm hold: seeds found AND closed
end-to-end, with the proof telemetry landing in the CITATION tier
(bare moves refuse both — the reach gradient measured on searched
material). LS3-i is CLOSED; LS3-ii (the sha-mined real-material arm)
is next per the fork ruling.

**LS3-ii RECORD (LANDED 2026-08-02 — the real-material arm; first
fire pending on CI).** Six instances mined from the generated
std/sha256/impgen_x86_out.shard — the imp->x86 leg's actual output.
Mining scope: maximal MEMORY-FREE straight-line runs (measured
inventory: 54 maximal runs, lengths 2-58; full species mix XBin32
1424 / XStore8 711 / XMovRR 359 / XLoad8 251 / XBin 231 / XMovRI 143
/ XShrI 111 / XRorI32 40 / XMovRR32 32 + 40 control) at the ratified
modest scope (length 2-3 here). Each instance searches over its OWN
instruction alphabet — the honest v1 vocabulary: minimality is
claimed over exactly the target's distinct instructions.

The set (tools/search/tasks/x86_span_instances.shard; hand-derived
expected minima pinned in-driver): I1 [mov32 RAX RAX; and RAX 255]
tie_x_shtotw@3 — the 32-bit self-truncation is subsumed by the byte
mask, expect FOUND len 1; I2 [mov RAX 0; mov RAX R8] tie_x_shround@30
— the dead constant store, expect FOUND len 1; I3 [xor RAX R15; mov
R15 RAX] @43, I4 [add32 R12 1; mul32 R15 256] @1, I6 [mov R15 R8;
ror32 R15 25] @39 — expect MINIMAL at 2; I5 [add32 R15 RAX; add32
R12 1; mul32 R15 256] @4 — expect MINIMAL at 3. PROVENANCE IS
CORPUS-CHECKED (tools/search/x86_span_mine_pins.shard): each instance
= a pinned slice of ix_sstmts(ibody_of(it_*_fn)) computed from the
imp pipeline directly, while the artifact's own tie claims pin the
same output as its stored lists — an impgen change that moves the
material turns the pins red and forces a re-mine (the regen-canon
contract for mined instances).

Machinery grown for real material (all in the LS3a files, so the seed
pins gate regressions): the probe's species layer generalized to the
full memory-free straight-line set (XBin/XBin32 all ops + SImm
sources, XMovRR/RR32/RI, XRorI32, XShrI/XShlI/XShlI32) across
templates, decode, goal-SExpr, operands; per-target alphabet
extraction; template-parametric grammar + checked-A* entry
(xspp_run_astar_t); the model's probe files now give ALL FIFTEEN
registers distinct nonzero values — file 3's low-32 parts exceed 255
(v_i = 2^33(i+1) + 2^24(i+1) + (2i+3)), which is what keeps a bare
32-bit truncation from fingerprint-aliasing a masked target (caught
on paper during file design, before it could alias I1's find).

Phase 2 on strictly-shorter winners runs against the SAME two-lemma
peephole pool — expected verdict for I1/I2: honest PROOF-REFUSED
(no mask or dead-store law exists yet). That refusal telemetry is a
deliverable: per the ratified fork LS3-B, LS3-iii grows pool/moves on
exactly these measured refusals. Refusals report as
IMPROVE-CANDIDATE, never accepted, never fatal; a red witness gate,
replay, expected-length pin, or EMPTY anomaly IS fatal. Expected
summary line: X86-SPAN-MINE LS3B OK IMPROVED-0 CANDIDATE-2
MINIMAL-4.

Local validation: mine-probe closure 394/0; provenance pins 838/0;
the new-species phase-2 smoke constructs and parses both improvement
goals (verdict REFUSED, no PARSEFAIL); all five files
shardfmt-canonical.

**THE OFFICIAL RECORD (CI pipeline 271, 2026-08-02, ENGINE EXIT 0,
peak RSS 2.49 GB, 176 s — every hand-derived prediction exact):**
  I1 TARGET-LEN-2 FOUND-LEN-1 EMPTY-AT-0 IMPROVE-CANDIDATE
     PROOF-REFUSED  (find 19,383 steps / 3 forks; EMPTY 2,067 / 0)
  I2 TARGET-LEN-2 FOUND-LEN-1 EMPTY-AT-0 IMPROVE-CANDIDATE
     PROOF-REFUSED  (16,932 / 3; 1,462 / 0)
  I3 TARGET-LEN-2 FOUND-LEN-2 EMPTY-AT-1 MINIMAL  (16,507 / 3;
     6,212 / 1)
  I4 TARGET-LEN-2 FOUND-LEN-2 EMPTY-AT-1 MINIMAL  (17,710 / 3;
     5,909 / 1)
  I5 TARGET-LEN-3 FOUND-LEN-3 EMPTY-AT-2 MINIMAL  (84,691 / 11;
     29,152 / 4)
  I6 TARGET-LEN-2 FOUND-LEN-2 EMPTY-AT-1 MINIMAL  (17,711 / 3;
     7,076 / 1)
  X86-SPAN-MINE LS3B OK IMPROVED-0 CANDIDATE-2 MINIMAL-4
What this says: the engine FOUND both real improvements in the
sha256 leg's emitted code (the subsumed truncation, the dead
constant store) and PROVED four workhorse shapes minimal over their
own alphabets under checked EMPTYs; the two improvements stand as
candidates because the two-lemma pool honestly refuses their piece
theorems — the measured refusal telemetry LS3-iii grows on (a mask
law and a dead-store law are the named gaps). The pilot's success
criterion (b) is met by the four minimality records; criterion (c)
by the refusal telemetry. Alongside, the LS3a seed probe re-fired
green under the all-live files (pipeline 272, EXIT 0, peak 2.09 GB:
PAIR FOUND-LEN-0 at 17,174 steps / 1 fork, SELF FOUND-LEN-1 +
EMPTY-AT-0 at 166,730 / 10 + 1,461 / 0, both CITE-closed) — the
regression arm holds under the LS3b file generalization. LS3-ii is
CLOSED; LS3-iii (pool growth on the two named refusals) is next.

**LS3-iii RECORD (LANDED 2026-08-02 — pool growth on the two
measured refusals; the ratified growth loop closing once).** The two
laws LS3-ii's refusal telemetry named, now proven in
models/x86/peephole.shard (ordinary shared-file work under LS-law 1;
the file also received a separate pure-format canonicalization
commit — it predated the formatted-repo sweep):

- **xseq_movri_movrr_dead** — [mov d, imm; mov d, s] = [mov d, s]
  under the reg_code disequality guard (the same decidable guard
  vocabulary as the xor pair law). Proof: rget_rset_other +
  rset_shadow, one have.
- **xseq_mov32_byte_mask_absorbs** — [mov32 d, d; and d, 255] =
  [and d, 255] premised only on nonneg (rget rs d). Proof: both
  sides collapse to mod (rget rs d) 256 through the std/bits
  mask/mod bridge (mask_word32, mask_byte) plus the new
  **mod_word32_byte** — mod (mod a 2^32) 256 = mod a 256,
  PREMISE-FREE, proved by mod_unique at the decomposition exhibited
  from the two euclidean identities (q = ediv a 256 − 2^24·ediv a
  2^32; the farkas cert (1 1 -1)/(1 -1 1) over the two
  ediv_mod_id haves). Nested-mod collapse is x86 mask-seam glue for
  now; promote to std/bits deliberately if a second consumer
  appears.

The pool (xspp_pool) grew to four; the citation-side plumbing was
already generic — zero probe-machinery changes beyond the pool list
and the model's two use lines (citations parse against the model's
scope). The I1/I2 piece theorems land as hand pins in the mine probe
(xsm_i1/i2_piece_pin — byte-for-byte the driver's constructed goals,
corpus-checked citability), and the mine probe's expected report
flips: I1/I2 IMPROVED PROOF-CITE-* — the summary becomes IMPROVED-2
CANDIDATE-0 MINIMAL-4.

Local validation: peephole closure 313/0 (all three laws green — the
mod_unique farkas vectors closed first try); the phase-2 smoke now
closes BOTH improvements at runtime (I1 →
CITE-xseq_mov32_byte_mask_absorbs, I2 → CITE-xseq_movri_movrr_dead);
the reach moves floor is UNCHANGED (0/4, 5/16 — pool growth is task
policy, not a proofgen move, exactly as LS-law 2 prescribes).

**THE OFFICIAL RECORD (CI pipeline 276, 2026-08-02, ENGINE EXIT 0,
peak RSS 2.42 GB): X86-SPAN-MINE LS3B OK IMPROVED-2 CANDIDATE-0
MINIMAL-4** — I1 IMPROVED PROOF-CITE-xseq_mov32_byte_mask_absorbs,
I2 IMPROVED PROOF-CITE-xseq_movri_movrr_dead, I3-I6 MINIMAL, with
every search step count IDENTICAL to the LS3-ii record (the search
side untouched; only the proof side flipped — exactly the isolation
the two-phase design promises). The pilot now has the full arc's
thesis end-to-end on real material: search FOUND the rewrites,
checked EMPTYs bound them, and the kernel-replayed piece theorems
ACCEPT them — two proven improvements to the sha256 x86 leg's
emitted code, earned by one turn of the measured-refusal growth
loop. LS3-iii is CLOSED, and with it the LS3 pilot's success
criteria (a), (b), (c) are all met.

Non-goals, stated once: no prove-on-engine rewrite before the LS-law
3 gate; no kernel/type-system growth; no repo-wide sidecar sweeps
(LS-law 1).

**LS4 — the reach ladder (opened 2026-08-02, user: option 3 — the
LS-law 3 gate work). THE ATTRIBUTION CENSUS (measured 2026-08-02) +
the rung design — RATIFIED 2026-08-02, both fork leans adopted
(user: "agreed on both leans"); the build opened as LS4-i.**

*The census method.* Every OPEN goal in the LS2 baseline that has a
committed sidecar entry also has a committed PROOF TEXT — the shape
the engine must learn is written down. The census crosses the LS2
per-goal verdicts with the sidecar proofs and classifies each open
proof by its proof-form features (citation / have-cut / induction
/ case-on / div-facts / bare steering). One-time measurement,
method recorded here; re-derivable from the baseline log + the
sidecars.

*An accounting correction to the LS2 record (surfaced, not silent):
the "196 claim-keyed entries" conflated three populations.* Matching
sidecar entry names against real benchmark goals gives 195, not 196
— the extra key is the phantom "NAME" from kernel/reader's sidecar
doc-comment, exactly the scoring artifact the LS2 record itself
flags. And 13 of the 195 are the far-tier opaque-type entries —
committed sidecars, but HAND-proven (the absorption's measured
ladder refusals), so they were never ladder-won. Corrected
accounting: **true ladder-won corpus = 182 entries, bare-moves
floor = 120/182 (65.9%), open = 62; far tier = 13 + frnd_pos_wf,
all open (unchanged).** The RULED gate re-reads against the honest
denominator with its meaning intact: **90% of the ladder-won 182 =
164 engine-closed (the engine must add 44 of the 62 open), plus at
least one far-tier entry.** No re-litigation — same ruling, correct
populations.

*The census (75 open sidecar-backed goals = 62 ladder + 13 far).*

| bucket | shape | ladder | far | total |
|---|---|---|---|---|
| D | induction / case-on (per-arm sub-proofs) | 36 | 1 | 37 |
| C | have-cut cascades + citations (the lo/hi bound family: 12 of the 17 are *_lo/*_hi pairs over word/rng/mem/bytes) | 16 | 1 | 17 |
| B | citation chains ± unfold (no cuts, no induction) | 4 | 10 | 14 |
| F | div-facts gateway + farkas cert | 4 | 0 | 4 |
| A | bare steering (unfold/premise-rewrite only) | 2 | 1 | 3 |

*The design-shaping fact:* closing EVERYTHING outside bucket D
yields 26 < 44 — **the gate is unreachable without induction/case
search.** Conversely the far tier is citation-shaped (10 of 13 in
bucket B: the opaque-type surface-lemma proofs), so the gate's
far-tier leg plausibly falls to the earliest rung.

*The rung ladder (each rung = engine growth measured against the
benchmark; refusal telemetry names the next growth — the LS3
discipline):*

- **LS4-i — the theory-backed citation rung.** An engine proof-search
  task: candidates = citation chains (rewrite-with lemma, direction,
  side, occurrence, premise discharges) up to a depth budget,
  interleaved with unfold/stop steering, terminal close by
  compute/refl/arith — searched against a REAL theory, not
  TheoryEmpty. Oracle = pg_check per candidate (in-search heuristic
  signal; replay stays the only certificate — reach scores, writes
  nothing). The reach harness grows an engine backend beside
  reach_one (the seam its header reserved). div-facts joins the
  citable move vocabulary (bucket F's measured need — proofgen
  growth on measured refusal, LS-law 2's pattern). Targets: buckets
  B + A + F (10 ladder goals) and the far tier's citation shapes —
  the probable first far-tier close.
- **LS4-ii — the cut rung (DISSOLVED 2026-08-08 by ruling — folded
  INTO LS4-iii; the fold ruling sits below the LS4-i record).** As
  originally ratified: have-cut synthesis over a comparison schema,
  each cut discharged by the LS4-i machinery, targeting bucket C
  (16 ladder). The LS4-i measurement mooted it as a standalone
  slice: div-facts + farkas closed 13/17 of C natively, leaving
  ≤5 gate points in its whole target population.
- **LS4-iii — the induction rung (now also the fold's landing
  seat).** induct / case-on as branching moves; each arm's subgoal
  re-enters the same search space, and LS4-ii's premised-citation
  and discharge machinery lands HERE as the arms' substrate. The
  mandatory rung (the census fact) and the largest design surface —
  its own design block before build, informed by LS4-i telemetry.

Gate measurement closes the ladder; the prove-on-engine rewrite
decision then unblocks per LS-law 3. frnd_pos_wf stays the beyond-
gate acceptance test (shard#18) — not gated on.

*Fork LS4-A — the theory the engine searches against (RULED
2026-08-02: the lean ADOPTED; the alternative's cost text stands as
the REJECTED-because record).*
- *The target module's own import-closure theory — what check itself
  replays against (ADOPTED).* Honest and curation-free: the engine
  faces exactly prove's setting, and pool-size pressure becomes
  measured census output (which pools exhaust, where cost
  concentrates) instead of a hand-tuned inventory. Cost: large
  pools make depth-k citation search expensive — bounded budgets +
  CI-only deep runs absorb it, and pool-pruning can land later as
  task POLICY if the census demands it.
- *Per-file curated lemma pools (the span pilot's phase-2 shape).*
  Cheap and immediate. Cost: curation is fixed-function policy in
  the mechanism's seat — the pilot licensed it as TASK policy, but
  the reach benchmark exists to measure the ENGINE, and a curated
  benchmark measures the curator.

*Fork LS4-B — rung order (RULED 2026-08-02: the lean ADOPTED).*
- *Citation first (ADOPTED).* Smallest new machinery (the pilot's
  phase-2 enumeration, engine-ized with real theory), the probable
  far-tier close, and the substrate the other rungs re-enter —
  induction arms close by citation/compute, so LS4-i is sequencing,
  not deferral.
- *Induction first.* Attacks the mandatory bucket immediately.
  Cost: its arms still need the citation substrate, so it front-
  loads the largest design surface while blocked on the smaller
  one's mechanics.

**LS4-i BUILD (landed 2026-08-02).** The architectural finding that
shaped it: the su schema drive evaluates score hooks through the
engine's REFLECTED evaluator over Expr-encoded data — pushing the
checker through that seam would mean Expr-encoding whole Modules and
interpreting the rewrite machinery per node. The honest direction is
the reverse: **the engine grew a proof-native drive** —
tools/search/prove_drive.shard, a sibling engine file with the same
discipline (uniform-cost A* ≡ breadth-first at h=0, checked dominance
on EXACT state keys, census-grade counters, the EMPTY/budget verdict
split) and NATIVE successors: the checker's own apply_step and
div_facts_checked are the move semantics, so in-search states are
exact kernel sequents at native speed. Winner acceptance = the full
assembled Proof re-checked by check_sequent from the ORIGINAL sequent
(the claim-ladder law; in-search states are guidance only).

- *Moves (all state/theory-derived, LS-law 2 honored):* Rewrite over
  every UNPREMISED theory entry (the kernel's Rewrite semantics
  itself excludes premised ones — RewriteWith discharge search is
  LS4-ii) × {lr,rl} × {lhs,rhs} at occ all; Unfold over the goal
  equation's call heads × side; premise rewrites over the sequent's
  own premises; Simp per side; div-facts injection at the equation's
  literal-divisor ediv/mod sites (≤ max_divs per chain). Closers per
  node = the floor moves natively (refl / compute-both / simp-both /
  arith0 / the farkas cert closer).
- *Dominance:* key = mix-hash + rendered goal equation + div history
  — an exact factoring (premises/params are functions of base + div
  history; hyps never change: no branching moves in this rung).
  Closable nodes return before key insertion, so the span pilot's
  cut-render aliasing law cannot bite. A drained frontier is an
  exact refutation over the alphabet (OPEN-drained); a depth-cap
  exit with a live frontier is the bounded refutation
  (OPEN-depthcap); pop-cap exits are OPEN-budget, never refutations.
- *The reach backend* (the seam reach's header reserved): per goal,
  floor moves first (tags unchanged), then the drive against the
  goal's REAL in-file theory — theory_base_st + decls_split +
  theory_step, prove's own recovery pattern, per the ruled Fork
  LS4-A. Lemma citations are kernel-level QNames against the theory;
  surface-scope citability (use lines) is an emit-time concern
  deferred to the prove-on-engine rewrite. tools/reach/
  bench_engine.shard = the self-contained 20-file benchmark runner
  for the CI engine-run job. Budgets (census-grade constants, stated
  with the record): depth cap 6, pop cap 4000, div cap 1.
- *Local validation:* all three files check green (closure 73/0);
  the floor pin is byte-identical (0/4 cond_mine, 5/16 auto_demo);
  entry-resolution probe confirms the two-mains import pattern
  (p26 scratch, untracked). Two bounded smokes: std/map — floor 2/5
  → engine 4/5, **lookup_insert_eq engine-d3-p7 = a FAR-TIER entry
  closed by the drive**, lookup_insert_neq engine-d3-p11 (bucket A),
  insert_shadow OPEN-depthcap-p59 (bucket D, refused exactly as the
  census predicts); std/div — floor 0/5 → engine 3/5, mod_10_lo /
  mod_10_hi engine-d1-p2 (the div-facts move + farkas closer, bucket
  F's shape), plus ediv_mod_10_id engine-d2-p5 (not sidecar-backed —
  a bonus), div_lt / div_nonneg honest depth-cap refutations.

**First fire (CI 288, 2026-08-02) — killed by the 7h deadline
watchdog with an EMPTY log: the budget was measured in the wrong
unit.** Diagnosis chain, all measured: (a) a local streaming probe
(p27 scratch) proved World writes STREAM to a redirected log — so the
empty artifact means the run never completed even the FIRST benchmark
file (wasm_rev; reach writes one block per file); (b) a staged cost
probe on wasm_rev's rev_loop_worker (p28 scratch) measured setup +
floor as seconds and the drive at **~400 applies per pop** (the
alphabet width on wasm-class goals) and ~650-1000 applies/sec in the
early regime — and per-pop cost DEGRADES superlinearly as sequents
and the closed set grow, which is what turned pops=4000 into
hours-per-goal on big material. The pop cap bounds the frontier, not
the WORK: per-pop work = alphabet width × expression size, wildly
non-comparable across theories. The resize (landed with this
record): **an APPLY CAP — total successor applications per goal, the
uniform work budget — at 50,000** (finds cost hundreds-to-thousands
of applies in every smoke: map a84/a224, div ~2k; sink-class
refusals bound near a minute), applies joins every engine tag
(-aN, census-grade), and the benchmark list is reordered CHEAP-FIRST
so a killed run still yields every completed file (writes stream).
Smoke verdicts under the cap are IDENTICAL (map 4/5, div 3/5; the
div depthcap refutations complete at ~20k applies — still exact).
The pop cap (4000) stays as the frontier guard; the deadline for
refires is set explicitly (the user's CI calibration 2026-08-02:
long runs are safe, the failure mode is order-of-magnitude runaway —
prefer raising ENGINE_RUN_SECS over squeezing budgets when the
measurement wants the full run).

The official record = the CI benchmark refire (ENGINE_RUN_TARGET
tools/reach/bench_engine.shard), scored against the corrected
baseline: floor 120/182 ladder + 0/13 far; the gate needs 164/182 +
≥1 far-tier. OPEN-budget rows are attempts the work cap cut — they
claim nothing and are re-runnable at a higher cap by ruling.

**Second fire (CI 298): lost to the 8h JOB timeout** — the fire-time
ENGINE_RUN_SECS:43200 override sat ABOVE the job's timeout, so the
internal watchdog (whose purpose is ending the script while artifacts
can still upload) never fired; GitLab uploads nothing on a job
timeout. Pairing rule now in the yml (9130bc2, engine-run timeout →
24h): overrides keep ENGINE_RUN_SECS under the job timeout minus
upload slack.

**THE HEAD RECORD (CI 299, 2026-08-03, watchdog-safe: the 12h
internal deadline fired as designed and the partial artifact landed;
15 of 20 files complete, cheap-first order + streaming writes = every
completed file scoreable).** Headline: **the gate's far-tier leg is
satisfied SIX times over** — blen_is_len, cat_len, get_set, gs_i,
lookup_insert_eq, str_len_nonneg, all post-#16 opaque-type entries
the ladder measurably cannot touch, closed by theory-backed chains
(gs_i: a FIVE-move chain, 121 pops / 14,057 applies; cat_len: d3, 91
pops / 42,212 applies). **30 engine closes** on the 15 files: 16
ladder + 6 far + 8 sidecar-unbacked bonuses (incl. len_word_bytes at
the full depth cap d6, 190 pops). Bucket verdicts vs the census's
in-log populations: **F 4/4 (complete — div-facts + farkas exactly as
designed), A 2/3, B 8/14, C 7/11, D 1/25** — the D near-zero confirms
the census's induction-mandatory arithmetic precisely (the one D
close, add_zero_right, found a citation chain its sidecar's induction
never needed). cond_probe (the conditional-citation probe) closed at
d2 by an alternative unpremised route. Ladder tally on the 15 files:
68/96 seen closed. Refusals: 35 OPEN-budget (the 50k work cap — the
re-runnable tranche), 18 depthcap, 17 drained (exact refutations).
Cost telemetry: 163 goals in 12h ≈ the degraded-regime apply cost
dominating; the pop-cap→apply-cap unit change held (every find ≤
50k applies, most ≤ 5k).

**The tail (fired as tools/reach/bench_engine_tail.shard):**
sha256.stream / word / kernel/reader / wasm_rev, then float kit LAST
— kit contributes ZERO gate-relevant goals (float.auto is
measure-keyed only: no ladder members, no far tier), so a watchdog
cut there costs only far-horizon telemetry. The official gate read =
this head record + the tail's ladder tally.

**THE OFFICIAL RECORD (CI 299 head + CI 302 tail, 2026-08-03/04;
tail watchdog cut exactly float kit — all 19 gate-relevant files
complete). LS4-i CLOSED.**

- **Ladder: 142/182 (78.0%)** — the engine added 22 ladder closes
  over the 120 floor (65.9%). The gate needs 164 (90%): 22 to go.
- **Far tier: 6/13 closed — the gate's far-tier leg is MET** (needs
  ≥1): blen_is_len, cat_len, get_set, gs_i, lookup_insert_eq,
  str_len_nonneg.
- **37 engine closes total** (22 ladder + 6 far + 9 sidecar-unbacked
  bonuses, incl. len_word_bytes at the full d6 depth cap and
  shc_ediv64_ub in the stream module). Notable: word's ENTIRE lo/hi
  made-bound family (u8/u16/u32 ×2) fell at d2 ~8 pops each — the
  div-facts move + farkas closer absorbed what the ladder does with
  have-cascades citing premised mod_lo/mod_hi.
- **Bucket verdicts (census-open populations): F 4/4, A 2/3 (str_valid
  far refused), B 8/14, C 13/17, D 1/37.** Every find ≤ 50k applies,
  most ≤ 5k; refusals: 61 budget / 20 depthcap / 19 drained.
- **The residue names the path (the census's arithmetic, now
  measured):** the 40 open ladder goals = 35 induction-shaped (D) +
  2 B (of_list_id, get_set_get) + 3 C. LS4-ii's premised-citation
  machinery can contribute AT MOST ~5 gate points — **the gate
  hinges on LS4-iii: ≥17 of the 35 open D goals must fall.**
  Surfaced for ruling at the next slice boundary: fold LS4-ii's
  RewriteWith/discharge machinery INTO the induction rung (it is the
  arms' discharge substrate) rather than running it as its own
  gate-measured slice.
- Float kit stays unmeasured this round (zero gate goals; the far
  horizon toward frnd_pos_wf/#18 — a cheaper dedicated fire by
  ruling if wanted).

**THE FOLD RULING (2026-08-08, user: "The fold sounds reasonable, we
can rework the ratified ordering with the new information").** LS4-ii
is DISSOLVED as a standalone gate-measured slice; its
RewriteWith/premised-citation and discharge machinery folds INTO
LS4-iii as the induction arms' substrate. The rework's ground: the
LS4-i measurement showed LS4-ii's whole target population is worth
≤5 gate points (C 13/17 already closed; B's premised residue = 2),
while the gate needs 22 and 35 of the 40 open ladder goals are
induction-shaped. REJECTED-because (the standalone slice): it kept
the one-rung-one-measurement cadence but spent a benchmark round on
a rung that cannot move the gate. The ladder is now LS4-i (CLOSED) →
LS4-iii (the fold rung, design block before build per the original
ratification).

**LS4-iii DESIGN (drafted 2026-08-08; forks LS4-C/LS4-D RULED same
day — user: "That looks good, let's dig in" — both leans ADOPTED;
the build opened).** Evidence base: the 35 open D-bucket
sidecar proofs, read in full.

*The measured shape of the population:*
- **All 35 proofs branch AT THE ROOT** — 26 `(induct VAR)` on a goal
  param, 9 `(case-on SCRUT Bool)`. Zero proofs take rewrite steps
  before the branching move. subterm-induct / wf-induct: ZERO uses.
- **8/35 re-branch inside arms** (nested induct/case-on; deepest =
  utf8_lead_le with 10 branch nodes, case-on chains over if-guard
  conditions `(le n 0)`, `(le 128 c0)`, `(le c0 191)`). Arms must
  re-enter the FULL space including branching — at their own roots.
- **Every arm body is LS4-i material** — unfold/reduce/simp/rewrite
  chains closed by refl or arith — PLUS the hyp vocabulary:
  `(rewrite (hyp k) …)` (the IH or the case equation), and
  have-cuts whose equation is an INSTANCE of the IH or of an
  unpremised lemma, discharged by one rewrite/rewrite-with + refl,
  then cited by the arith closer. word's wrap goals' rewrite-with
  mod_lo/mod_hi cascades are the exact shape LS4-i's
  div-facts+farkas absorbed natively in the made-bound family.
- **The checker already exposes the arm seam non-recursively**
  (kernel/checker.shard): `induct_case_subgoal` /
  `caseon_case_subgoal` build ONE arm's sub-sequent (IH attached as
  hyps by `build_ihs`), out of the check_sequent SCC — callable
  natively, the LS4-i pattern (the checker's own functions, no
  reflection). Setup is reproducible: find_param → type_head →
  lookup_typedef (zero-ctor refusal stands) → zip_pairs;
  tc_scrut_targs + inst_ctor_fields for case-on.
- **The IH's shape decides the citation tier** (`build_ih`): with no
  surviving ∀-params it is `(Goal Nil Nil eq)` — plain-rewrite
  citable; with surviving params (take_le's n) it is a GENERALIZED
  Goal needing rewrite-with instantiation — the folded LS4-ii
  machinery's real content.
- **farkas_solve reads PREMISES only** (1–6, meta/proofgen) — hyps
  are invisible to the closer. The ladder's own bridge is the
  have-cut: `(have <ih-instance> (… (rewrite (hyp k) …)) refl)`
  puts the instance in the premise slot arith cites.

*The architecture (fork LS4-C's lean): branching ABOVE the drive,
not inside it.* A pf_branch orchestrator wraps pf_drive: try the
linear drive first (LS4-i behavior, unchanged); on refusal,
enumerate branch candidates at the ROOT — induct over ctor-headed
goal params, case-on over guard conditions harvested from the goal
equation's 1-step-unfolded call heads (+ comparisons already in the
goal) — build arm sub-sequents via the checker's own builders, and
recurse per arm (branch budget bounds total branch nodes). Each arm
is a FRESH drive invocation: hyps are constant within it, so the
exact-state-factoring key law holds per invocation with hyps
excluded from keys, exactly as today. AND semantics: an arm's
refusal kills the candidate; the goal's verdict = best over
candidates (conservative: PfBudget unless all candidates drained).
Budgets stay in APPLIES, shared: arms consume the goal's one
apply_cap sequentially. Assembly: per-arm PfFound proofs →
`(Case cname names pf)` → `(Induct var cases)` / `(CaseOn scrut ty
cases)`; acceptance = check_sequent on the ORIGINAL sequent — the
certificate law unchanged.

*The linear drive grows the arm vocabulary (the fold's landing):*
- **M1 hyp-rewrite**: `(Rewrite (Hyp k) dir side)` over UNPREMISED
  no-param hyps — the mirror of the premise-rewrite family.
- **M2 hyp-bridge**: a have move injecting an unpremised hyp's
  equation as a premise (discharge = one hyp-rewrite + refl, built
  and verified at move-generation time) — farkas then sees the IH.
  A new history segment joins the key bytes, the PfDiv mechanism.
- **M3 instance-cut**: synthesize IH/lemma INSTANCES by unifying the
  cited equation against goal subterms (take_le's
  `(take (- n 1) c1)` picks the instantiation); have + rewrite-with
  (hyp) / rewrite (lemma) discharge; the instance lands as a
  premise. M2 = M3 at the identity instance — kept as the cheap
  tier.
All three generate ZERO moves when hyps = Nil, so base-goal drives
(every LS4-i path, the pin) are byte-identical in behavior.

*Fork LS4-C — where branching lives (RULED 2026-08-08: (a) ADOPTED).*
- *(a) Root-alternative orchestrator.* Branching tried only at the
  root of each (sub-)search; arms are fresh recursive drives.
  Preserves the frozen frontier/key machinery; matches 35/35
  measured proofs (and the ladder itself only roots inductions —
  parity is root-only regardless). Cost: cannot find
  steps-then-induct proofs — a shape with zero measured demand.
- *(b) Branching as frontier moves (AND/OR in one frontier).*
  Strictly more general. Cost: hyps join the search state — the
  exact-state key law and dominance machinery need a redesign; a
  large build against zero measured demand.

*Fork LS4-D — how much folded citation machinery in build 1 (RULED
2026-08-08: (a) ADOPTED; (b) later on measured refusal).*
- *(a) Instance tier only* (M1+M2+M3). Covers every citation shape
  appearing in the 35-goal population; word's premised mod_lo/mod_hi
  shapes are already absorbed natively (LS4-i measured).
- *(b) Also premised-LEMMA rewrite-with enumeration* (the original
  LS4-ii headliner: theory-wide premised citations with searched
  discharges). Cost lands exactly where LS4-i measured pool-width
  pain, for ≤2 reachable goals (B's of_list_id/get_set_get — outside
  the D population the gate hinges on). Later on measured refusal
  (LS-law 2's pattern) unless ruled otherwise.

*Measurement:* the same 20-file benchmark, cheap-first, same
census-grade budgets; tags grow branch-node counts
(engine-bB-dK-pN-aM). Gate read vs the LS4-i record: ladder needs
164/182 (22 more; ≥17 must come from D), far-tier leg re-confirmed.

**LS4-iii BUILD (landed 2026-08-08; probe-validated, the benchmark
fires next).** The design held; the probes reshaped three pieces of
it, each a measured lesson:

- **The machinery, as designed:** prove_drive grew the hyp tier —
  `(Rewrite (Hyp k) …)` moves over unpremised hyps and PfHyp (a Have
  cut of a hyp INSTANCE, identity or matched, discharged by ONE
  hyp-rewrite + refl, self-verified at apply time via tc_check_cut +
  discharge replay; a new key-history segment at separator 38, the
  PfDiv mechanism). prove_branch is the orchestrator: one
  fuel-measured task machine (PbTryGoal / PbTryCands / PbTryArms —
  no mutual SCC), arm sub-sequents built by the CHECKER'S OWN
  induct_case_subgoal / caseon_case_subgoal, arms as fresh drive
  invocations, candidates = induct over ctor-headed params then
  case-on over Bool guards harvested from one-step unfolds. Field
  names are gen_fresh AT BUILD and STORED in the Case — replay
  rebuilds subgoals from the stored names (Nil names would re-fresh
  at replay and break field-mentioning cuts). PbLin returns the root
  linear verdict VERBATIM when no branching applies — unbranched
  goals report exactly as LS4-i.
- **Probe lesson 1 — THE IH-COVERAGE EXCLUSION (p30/p31, take_le):**
  an arm's induct candidates on a RECURSIVE FIELD of an enclosing
  induct re-derive the IH's own statement with strictly less to work
  with; the wasted subtree measured >150k applies on take_le.
  Measured license: 0/35 population proofs re-induct an IH-covered
  field (every nested induct targets a different variable).
  Exclusions accumulate down arms by name.
- **Probe lesson 2 — THE LEMMA-INSTANCE FARKAS ASSIST (p31, the
  take_le True leaf):** after full simp the leaf goal is
  `(le 0 (+ 1 (len c1)))` — closable ONLY by a len_nonneg instance
  the goal does not contain as a subterm; NO move in the linear
  alphabet can express it at any budget (a real vocabulary gap, the
  census's utf8 have-lemma pattern). Fix on the CLOSER side, the
  div-facts philosophy: when plain farkas fails inside an arm (hyps
  non-Nil — base drives never pay, LS4-i parity), synthesize
  instances of le/lt-headed unpremised theory lemmas by the hyp
  tier's subterm matching and retry farkas with ONE instance added;
  the winner assembles as a self-discharging Have. The frontier
  never grows.
- **Probe lesson 3 — arm budgets:** the False (IH-citing) leaf's
  find measured 31.4k applies — a 25k arm cap LOSES it; the refusal
  tax that motivated a smaller cap was cured at the source by
  lessons 1+2, not by squeezing. Final budgets: depth 6 root / 8
  arm, pops 4000, linear cap 50k root AND arm, divs 1, haves 2 per
  chain, fuel 400, pool 600k per goal (the deepest measured tree,
  utf8_lead_le's 10 branch nodes, ≈ 9 internal 50k refusals + leaf
  finds).
- **Probe record (p30, local capped scopes):** auto_demo
  add_n_zero b1-a68 / add_comm b1-a20479 / beq_sym b3-a943 (nested)
  / max_idem b1-a225 (case-on via unfold harvest); std/list
  take_zero b1-a50524 / append_assoc b1-a53448 / take_le
  b2-a139261 (induct + nested case-on + M3 instance + assist). All
  branched closes passed the root check_sequent; zero witness
  fails.
- **The benchmark's three-way split:** with the bigger per-goal pool
  the 20-file run cannot fit one 24h job — bench_engine (12
  light-theory files, where D concentrates) + bench_engine2
  (bytes/mem/sha256) + bench_engine_tail (stream/word/reader/
  wasm_rev/kit), each under the pairing rule. Tags: engine-bB-aM
  for branched closes, OPEN-branch-aM for exhausted candidates
  (never a refutation); OPEN-drained now additionally requires ZERO
  branch candidates.
- **CI 313+314+315 lost to a HOST CRASH (the sequential-fire ruling
  REVERSED same day — user).** The first three-way fire died at ~3h:
  node c-srv3-k8s went NotReady and all three pods were killed the
  same second. My memory-contention conjecture did NOT survive the
  facts: the host carries 2.7TB of RAM (user), and concurrent
  storage work crashed the VM — the fires were collateral, not
  cause. Parallel engine-run fires are GREEN-LIT (user 2026-08-08);
  refires = CI 317 + 320 + 321. The durable lesson is narrower: a
  POD-LEVEL KILL LANDS NO ARTIFACTS — only the internal watchdog
  exit uploads partials — so long fires remain exposed to
  infrastructure loss, and per-file streaming only pays off if the
  pod dies through the watchdog path.
- **INTERIM RESULTS (2026-08-09; official record pends the heavy
  refires).** A push auto-canceled still-QUEUED 321 (running fires
  survive pushes); the tail refired as 323.
  - **CI 317 (fire 1, the 12 light files; 20h watchdog cut before
    file 12): +20 closes over LS4-i on its slice, ZERO regressions.
    18 of the 20 are ladder members and ALL 18 are D-bucket — the
    gate's ≥17-from-D leg is MET by this fire alone.** Running
    ladder tally 142+18 = 160/182 (gate 164). Wins include the
    whole std/list induction family (take_le b2-a139261, drop_le
    b2-a134996, append_assoc b1-a53448, rev_rev b1-a66953 …), all
    seven auto_demo induction goals, adq13_leaf b2-a294,
    int_of_nat_nonneg b1-a1160, ilen_nonneg b1-a424. Base-drive
    parity HELD at benchmark scale: every goal the fold can't touch
    reports a byte-identical tag to LS4-i. Still open: utf8_lead_le
    / utf8_cont_le / take_len / drop_len drain the full 600k pool
    (OPEN-branch-a600k); str_valid + the div pair refuse exactly as
    before (linear-shaped).
  - **CI 331 (fire 1b, imp_mix make-up, bench_engine1b, 2h):
    identical closes to LS4-i (2/3, tags byte-identical);
    searched_mix_wasm now branches and drains (OPEN-branch-a600416,
    was OPEN-budget).**
  - **CI 320 (bytes/mem/sha256, 20h): bytes.shard only — 0 gain / 0
    loss (7 branch attempts died candidates-exhausted at 100k-600k;
    list_of_cat hit the pool). The remaining ~18h sat inside mem (21
    opens) without completing it. CI 323 (tail, 20h): ZERO bytes —
    the full run sat inside sha256.stream (19 opens, the heaviest
    theory), RSS flat at 67.8GB.** Post-mortem: the three-way split
    was calibrated on LS4-i per-goal costs; LS4-iii's pool is up to
    12x the work per open goal (measured ~2h for ONE 600k drain on
    imp_mix's theory), and reach_target_engine's per-FILE write
    threw away every finished goal on both cuts.
  - **The refire topology (50f7a7c, harness only — engine semantics
    and budgets untouched; smoke = adq13 lines byte-identical to CI
    317):** reach_goals_engine now STREAMS each REACH line as its
    goal resolves (a watchdog cut loses only the goal in flight);
    rch_drop_goals + reach_target_engine_skip + reach_all_engine_skip
    = continuation fires (skip = REACH lines already landed for the
    file). Fire configs: bench_mem (mem alone) + bench_stream
    (stream alone) + bench_rest (sha256/reader/wasm_rev/word/kit,
    cheap-first); bench_engine2 + bench_engine_tail DELETED
    (superseded). SECS 79200 (22h — the 20h+4h-slack pairing was
    exactly what mem missed by; ~1.5h pod-setup margin stays under
    the 24h ceiling). First refire round (CI 333/334/335) was
    auto-canceled by the ledger push while `waiting_for_resource` —
    **the auto-cancel law is BROADER than "queued": a push kills
    every not-yet-RUNNING pipeline on the ref. Operational order:
    finish ALL pushes, then fire; hold subsequent pushes until the
    fires reach running.** Refires = CI 337 (mem) + 338 (stream) +
    339 (rest).
  - **CI 337 (mem, clean exit): all 45 goals measured, 0 gain / 0
    loss — parity on all 24 closes (far pair get_set + gs_i
    re-confirmed). Most of mem's 21 opens now die
    candidates-exhausted (OPEN-branch 94k-250k): the fold finds
    branch candidates but the arms need div/mod reasoning through
    the opaque mem surface, not structural induction.**
  - **CI 338 (stream): +6, 0 lost — the whole shc_ chunk-list
    family (slen_nn, slen_sapp, sapp_nil, sapp_assoc, sdrop0,
    stake0) closes engine-b1, ALL SIX LADDER D MEMBERS. Running
    ladder 160+6 = 166 ≥ 164 — THE GATE LINE IS CROSSED, pending
    the official full-corpus record (CI 339 + the stream tail).
    The far tier is now fully seen across combined logs: 6/13
    closed, far leg re-confirmed.** The fire ended not by watchdog
    but `rt trap: heap exhausted` after 24 of 30 goals. Streaming
    banked all 24; the tail refired fresh-heap as CI 349
    (bench_stream2, the skip machinery's first use — skip 24
    verified against goal order).
  - **CI 349 (stream tail, fresh heap): trapped `rt trap: heap
    exhausted` after 6.4h with ZERO lines — the cross-goal
    retention diagnosis is REFUTED: shc_blocks_split ALONE
    exhausts the runtime heap at census budgets (CI 338's trap was
    mid-goal-25 on a warm heap, 349's on a cold one; also the
    likely mechanic behind CI 323's flat 67.8GB RSS). CI 353
    (bench_rest2) continues wasm_rev skip 14 + word + kit; CI 35x
    (bench_stream3, skip 25) probes whether the remaining five
    correspondence goals are measurable or the family blows the
    same way. If the family is unmeasurable at census budgets, the
    record types those goals ENGINE-HEAP refusals (they were all
    OPEN-budget in LS4-i — no ladder close at stake; the gate line
    stands at 166 without them).**
  - **CI 356 (skip 25): trapped identically — `rt trap: heap
    exhausted`, 5.9h, zero lines. THE FAMILY IS TYPED: two members
    (shc_blocks_split, shc_blocks_prefix) independently exhaust the
    runtime heap at census budgets on fresh heaps; the remaining
    four (step_abs, fin_abs, stream_gen, sha_stream_corresponds)
    are the same correspondence shape, heavier. No further stream
    fires — sha256.stream enters the record 24/30 measured (+6
    ladder D, 0 lost) + 6 ENGINE-HEAP refusals. The heap ceiling on
    a single 600k-pool branch search over the machine-tier theory
    is an ENGINE limitation, not a search-policy refutation — a
    future rung (heap growth, per-arm eviction, or process-per-goal
    harness isolation) owns it.**

**THE LS4-iii RECORD (official, 2026-08-11) — LS4-iii CLOSED, THE
LS4 REACH GATE IS MET.**

- **Ladder: 172/182 — gate 164 CLEARED with +8 margin** (LS4-i
  baseline 142; +30 ladder closes, ZERO losses anywhere). All 182
  ladder members measured. **Far leg: 6/13 closed (needs ≥1) — MET**
  (the same six as LS4-i: blen_is_len, cat_len, get_set, gs_i,
  lookup_insert_eq, str_len_nonneg). **D leg: 31 of the 37
  census-open induction-shaped goals closed — the ≥17 requirement
  nearly doubled.**
- **Corpus totals: 297 goals across 20 files** (kit measured for the
  first time, +1 linear close fpow2_le0). Tag census: 163 floor / 38
  linear-engine / 33 branched-engine closes; 43 OPEN-branch / 20
  other refusals; six stream correspondence goals = ENGINE-HEAP
  refusals (non-ladder; twin fresh-heap witnesses CI 349+356).
  **Every branched close in the corpus is a NEW close over LS4-i —
  the fold contributed exactly its design target and nothing
  regressed: base-drive parity held byte-identical on all ~200
  untouched goals.**
- **34 gains total (30 ladder + drop_shortens, rev_rev, fpow2_le0,
  size_sexpr_list_nonneg): the std/list family (take_le b2-a139261 =
  the assist's flagship, drop_le b2, append_assoc, rev_rev …), all
  seven auto_demo inductions, the stream shc_ chunk-list family (six
  b1), the word wrap family (i8/i16/i32 hi+lo, six b1 at 61k-78k —
  precisely what LS4-i's 50k lin cap refused), adq13_leaf,
  int_of_nat_nonneg, ilen_nonneg, reader's size_sexpr_list_nonneg.**
- **What still refuses, typed:** mem's opaque-surface family
  (OPEN-branch 94k-250k — arms need div/mod reasoning through the
  mem abstraction, not structural induction); the deep utf8 pair +
  take_len/drop_len (full 600k pool drains — deeper composite
  shapes); bytes' seven candidates-exhausted; str_valid + div pair
  (linear-shaped as ever); the ENGINE-HEAP six.
- **Fire ledger:** CI 317 (light, +20) / 331 (imp_mix, parity) / 320
  (bytes, 0/0) / 337 (mem, 0/0 clean) / 338 (stream 24/30, +6) / 339
  (sha256+reader+wasm_rev partial, +1) / 353 (wasm_rev tail + word
  86/86 + kit cut, +7) / 349+356 (heap-trap witnesses). Everything
  under census-grade budgets, identical across fires.
- **Surfaced for the slice boundary (user decision, per the interim
  ruling): the gate is met — LS-law 3's prove-on-engine rewrite
  decision is now live.**
- **2026-08-22 (the reset): RE-SCOPED as #27** — no standalone rewrite
  of tools/prove; the engine becomes the coverage arc's (#23) proof
  automation, its reach measured on the generated obligation families.
  The arc merged to main 8bf4b2d (+ fixup 5031db8) the same day.
