# The three-way design review (CDR) — 2026-07-18

A preserved correspondence: Christian, Claude (Fable 5), and Codex
reviewing the whole-project state at the sha256sum-on-silicon
milestone (I2e-4c-4), immediately after the imp-if-tier and riscv
merges. Outcomes that became direction: the untrusted-compiler +
proven-validator certificate architecture, explicit conversion forms
(change/exact-conv), base+patch proof-facing states, the
Runs/RunsWithin cost interface, the four-level numeric contract for
ML, the arc plan (certificate architecture → streaming sha / numeric
flagship → parallel pilot), and the six multiplying interfaces with
falsification gates. Ledger amendments distill from here; this file
is the record of the reasoning.

----------------------------------------------------------------

Christian, directed at codex: 




Alright, we have built shard up to the point where we can finally build a sha256 binary as a
  refinement from a imperative intermediary form. It's taken a long time to get here, but now that
  everything is together we can finally triage the pros and cons of the approaches used. There is a
  known ledger of issues that we will have to solve for this approach to wrap up and automate
  properly, but enough is together to point at and talk about. Most of this has been built by myself
  and claude fable with occasional input from openai models, but you should be a reasonably
  independent viewpoint on the entire thing. In my position, what I have been pushing for is
  something of a universal language: describe any algorithm at high level, describe how to lower it
  for any hardware architecture, establish formal proofs of the whole span (requirement satisfaction
  as well as program refinement), and end up with an artifact that can use the hardware as
  effectively as any existing software construction architecture. One subject we have not touched
  yet, but I plan to begin work on shortly, is parallel compute support. The design should be able
  to not only express, prove, and compile single-threaded applications on a single target isa, but
  be able to represent a wide range of multi-threaded compute architectures (model and lower a given
  shard function to a SPIR-V structure that can use a given GPU efficiently, etc). One of the nice
  things about the intended design is that it gives us opportunities to have non-automatable
  transforms expressed with IMP style pins where the developer is free to outline the core series-
  parallel transforms explicitly without the shard compilers becoming smart enough to automate that
  step. I have not read the whole of the code we have built, but I have been involved deeply enough
  to know what it is supposed to be built as. The main thing I want from you to start here is: read
  and understand the root vision, read through how we have built things so far, and give me both
  your subjective opinions on the whole thing as well as predicted scaling issues, things that won't
  automate as cleanly as we hope, anything particularly ugly in the design or implementation, and
  any suggestions for better variations to make. This will be a three-way conversation between you,
  myself, and claude fable (with me clumbsily copying and pasting correspondence as needed).


================================================================
Claude (Fable 5), review pass 1 — 2026-07-18
================================================================

Orientation for this pass: I re-read IMP.md's design + redirection
sections, MEMORY.md and ISA.md in full, LOWERING.md's schema, and the
full I2e arc record. I also have two fresh-context surveys running
(one quantifying the hand glue in the sha256sum chain, one mapping the
spec-language-vs-impgen coverage gap); their numbers will land as an
addendum. Since I built much of this, my useful angle is different
from codex's: I know which parts fought back, which walls were
converted into design fixes, and which ratified answers are still
promissory notes. I've deliberately written this as a stress-test of
the plan rather than a restatement of it. Opinions are labeled.

## 1. What I expect to work (evidence, not hope)

- **The trust architecture.** ISA-model-as-library, composition by
  citation, generate-and-check, zero kernel growth. This has now
  survived four ISA models (wasm, x86, RISC-V, PIO) without a single
  kernel change, and the one soundness incident of the era
  (parallel-let reversal) was in a reducer, not the architecture. The
  RISC-V arc is the strongest evidence: third target, template-grade
  transplant, one week, both widths in one model. I consider this
  layer settled.
- **The M×N factoring through imp.** Demonstrated end to end on real
  code: one sha256 imp sibling, two silicon-bound targets, only the
  imp ⊑ ISA legs written twice. The typed machine (v2 kinds) was the
  load-bearing correction — alignment is now program-independent BY
  TYPE, which is what made impgen a structural walk instead of a
  recognizer zoo. That correction pattern (when proof burden explodes,
  fix the representation upstream, never push through heroically) has
  fired at least four times now (Int fuel → structural Nat; monolith
  legs → seal points; recognizer tiers → kinds; weval budget →
  delegation boundary) and is, in my opinion, the project's real
  methodological asset.
- **Per-instruction proof economics.** Straight-line code costs ~zero
  (one compute-both); loops cost a fixed skeleton. This was the
  measured go/no-go question of the ISA arc and it stayed answered
  through the sha flagship. impgen-generated cert families land green
  first-try with high regularity now (the last several slices'
  records say "first check" more often than not).
- **The branch tier.** After the iftier merge, branches are FULL at
  every position the dialect admits on both targets, with named
  fences at genuinely hard spots. This was the last blocking
  precondition for match-arm lowering (match = tag dispatch + field
  loads rides exactly this machinery).

## 2. What is incomplete in ways automation would bake in
(the "before we automate an incomplete design" list, most important
first)

**2a. The non-tail call story has no machine substrate.** The
redirection says "non-tail = real calls on the models' structured
call mechanisms" — but what exists today is register marshalling
with ≤6 args, callee-saved homes for 12 locals, and calls whose
frames never nest data (x86 §4.3 data-stack territory is
deliberately unbuilt; the riscv arc held the same stop rule — its
only stack use is the ra slot). A generic compiler over arbitrary
first-order shard MUST handle non-tail recursion (calc's recursive
descent is the pathfinder's own centerpiece), and that needs the
frame-stack story: spill conventions, stack-depth premises
(MEMORY.md §7's discharge-by-construction), and per-target call
lowering with actual frames. This is the single largest unbuilt
mechanism on the coverage arc's critical path, and it's a DESIGN
item, not a slice item — the frame layout convention interacts with
the window/address policy (2c) and with DPS result windows. I'd
want this ratified (a §4.3-shaped ledger section, both targets)
before the generic translator's call tier is written.

**2b. Signed kinds arrive in week one, not "behind a consumer."**
The kind set is U8/U32/U64, signed variants named-later. But the
ratified pathfinder is a CALCULATOR — user-supplied arithmetic is
its whole point, and D8's overflow family is explicitly named as
what calc exercises. Subtraction on unsigned kinds with wrap
semantics is expressible but proof-hostile (every comparison
becomes a band dance). Opinion: resolve the signed-kind design
(representation: two's-complement views over the unsigned cells,
signed comparisons as ops, division semantics) BEFORE the
translator's arithmetic tier, or calc gets contorted into an
unsigned shape that doesn't generalize and the contortion gets
generated into every artifact.

**2c. Window/address policy is still per-bin folklore.** Every bin
so far hardcodes its window ([0,65536) for sha256sum — which put
the input buffer on page 0 and cost us the setcap hack; 6 MiB for
addw), its trace shape, and its entry-state seeding. #65 exists
precisely because the window choice leaked into ~9k lines of weld
spellings. The generic bin tail (declared externs → entry contract,
trace shape from the bin's claims, window from policy) is the right
shape, and under generated certs the window becomes a regen knob —
but that only works if the ADDRESS-SPACE POLICY is an input to the
generator from day one. Decide the policy vocabulary (window base,
size, stack reservation, data segments) as part of the coverage
arc's opening, then #65 is a config change, not an arc.

**2d. The heap readback/framing algebra needs to be a LIBRARY
before it's a generator target.** The ratified cert families
(per-TYPE readback from decls, per-CONSTRUCT heap-invariant lemmas,
per-FN induction along the totality measure) are the right shape,
and I believe the per-FN claim for structural first-order code. But
the honest record is that every new framing FAMILY in the sha arc
(below-family, above-family, point-grain, wlist-grain, pad-hi
lifts) was hand-discovered, and those were STATIC disjoint windows
— the easy quadrant. A counted heap's readback is a relation over a
dynamic object graph; framing becomes "allocation preserves all
other readbacks" + "free at count 0 preserves all other readbacks"
— separation-logic-shaped, as MEMORY.md §8 says. My strong
recommendation: build the heap-invariant + readback lemma family as
a std-style law library with a small hand flagship FIRST (a
cons/match/free micro-program — one ctor, one match, one free), and
only then point the generator at it. The generator should CITE a
stable algebra, never synthesize framing per program. This is the
mem-arc playbook (opaque substrate + law family, consumers cite
laws) applied one level up, and it's the difference between the arc
converging and the arc rediscovering the sha weld experience at 10x
scale.

**2e. The weld/spelling-alignment tax and the hand-twin premise are
in tension.** The v2 premise says hand-written imp twins are
first-class refinement inputs. The I2e record says the rewriter
matches syntactically, never modulo computation, so every meeting
point between independently-authored artifacts costs spelling
bridges (byte-copied local spellings, compute-both bridges, stop
sets). Generated-against-generated avoids this by construction (one
tool owns both spellings). But a HAND twin meeting generated legs
re-imports the tax. Either (a) the well-kinded gate + normal-form
spelling rules get strengthened into a CANONICAL SPELLING contract
that hand authors must meet (canon's job, mechanically checkable —
my lean), or (b) we build a weld-assist instrument (meta/-tier:
given two contexts, emit the bridge lemmas) — which is a
tools/prove-class citizen, not a one-off. Without one of these,
"hand twins are first-class" is true at the type level and false at
the proof-effort level.

**2f. Fuel laws are empirics where they could be theorems.** Burn
rates today: wasm istmts 1/stmt (2 for loop stmts), x86 xeval_seq
1/position +1/nesting, XCall arm −2, bridge −1... these live as
impgen-internal constants (tcost/gcost) measured by
claim-mismatch readout. It works, but every machine/construct
addition re-measures, and hand authors hit gotcha #14-style
overgeneralizations. Cheap improvement with compounding returns: a
per-machine COST-MODEL THEOREM (istmts consumes exactly f(body) —
proven once, by the same structural induction everything else
rides), making fuel synthesis derivable instead of measured. This
also hardens the towers against model edits.

## 3. Predicted scaling issues

- **Check-time capacity is the recurring wall and the mitigations
  are per-symptom.** >66GB monolith legs, the 96-instr weval budget,
  5.9MB spelled claims, shardfmt's quadratic Cons-spines — each got
  a structural fix (seals, delegation, sc2 folds, flat spellings).
  The meta-principle nobody has ratified yet: GENERATED CERT SIZE
  AND CHECK TIME MUST BE LINEAR IN PROGRAM SIZE, with per-construct
  lemmas O(1). I'd make that a stated gate of the coverage arc
  (regen CI measures it), so blowups surface as design bugs at the
  first fixture, not at the first real program.
- **CI/corpus wall-time becomes critical path.** Fast tier ~35 min,
  long tier ~82, engine cold boot ~34 on the k8s runner — and the
  coverage arc multiplies generated outs. Task #62 (per-module check
  cache over shared import closures, content addressing) should be
  elevated from QoL to an early coverage-arc dependency. Related
  known landmine: driver probe/transient names collide across
  concurrent runs in one directory — fine today, not at fleet scale.
- **Idiom debt vs. authors.** The I2e record alone carries 21
  numbered gotchas. impgen hard-codes them, which is the right
  sink — but it means the generator is also the project's tacit-
  knowledge archive. Keeping the gotcha→generator pipeline honest
  (every hand-discovered idiom lands as generator behavior or a
  diagnoser message, never only as memory prose) is what keeps the
  bus factor tolerable.
- **Lemma-home hygiene.** The sha sibling's ish_* namespace now
  holds genuinely general results (ish_shr_add, the transfer kit,
  framing families) that later arcs will want; loopkit is at FOUR
  speakers (wasm, x86, imp, riscv) and its meta/ graduation has
  been queued since I0. Cheap debt now, expensive later; a
  graduation sweep belongs between arcs.

## 4. Ugly-but-working (honesty list, no action urged)

- The bin-boundary law forcing host tools to load products
  dynamically by path string is correct but surprising; a
  "cite-a-bin's-theorems-without-absorbing-its-externs" import form
  would be cleaner if a second consumer class appears.
- Two artifact-claim trace shapes exist (argv world-bins vs the
  stdin bin) with the generic form still unnamed — resolved by the
  bin-tail work in 2c.
- (inline …) not nesting, mod.req rebinds needing explicit
  module-surface use-lines, the ground-Nat packed/tower rep dance —
  all have settled workarounds, all are the kind of paper cut a
  future canon/QoL pass could take a run at.

## 5. Parallel compute (the new direction) — readiness assessment

The architecture extends more naturally than I initially expected,
with one genuinely new obligation:

- **A SPIR-V model is just another model.** SPIR-V REQUIRES
  structured control flow (merge/continue blocks) — it is closer to
  wasm than x86 is. The model-as-library + capability-doctrine
  machinery carries over unchanged; a GPU target's capability set
  is where "no U64", "no unbounded memory", "subgroup width W"
  live. The capability doctrine should grow CLASS capabilities:
  device targets accept frame/region-class twins and REFUSE the
  counted-heap class (RC on device memory would be a mistake we're
  structurally protected from making).
- **The new obligation is concurrent semantics, and the right v1
  quotient is SPMD + barriers + proven disjointness.** Everything
  today is deterministic sequential big-step. The clean entry:
  dispatch(n, kernel) denotes a parallel map — a pure function of
  the invocation index — and the commutation theorem (parallel
  map = sequential fold) is discharged by per-thread WINDOW
  DISJOINTNESS, which is exactly the framing discipline we already
  have, generalized over a thread-index variable. Barrier-phased
  kernels = sequential composition of parallel maps. Race freedom
  is then not a checker but a proof obligation whose failure means
  the compile doesn't close — the safety inversion carries over
  intact. Weak memory stays OUT of v1 the same way framing was
  dissolved-not-solved in ISA v1: barrier-synchronized
  disjoint-write SPMD never observes it.
- **The user's IMP-style-pin instinct is the right one for the
  hard part.** Tiling, shared-memory staging, warp-level tricks are
  exactly non-automatable series-parallel transforms; the natural
  home is a parallel tier of imp (explicit par-map/seq nodes over
  the same kinded scalar/mem vocabulary) where the developer pins
  the decomposition and proves it equal to the sequential spec via
  the commutation laws — the compiler never needs to discover
  parallelism, only to check it and lower it. That is this
  project's founding move (verify-don't-search) applied to
  parallelization, and I think it's genuinely novel territory —
  polyhedral compilers SEARCH for this; we'd CHECK a declared
  decomposition.
- **Sequencing opinion:** parallel compute should open AFTER the
  coverage arc's uniform rep exists (kernels consume
  frame/region-class data; the class vocabulary must be real
  first), but its imp-tier design (par nodes + commutation-law
  shapes) is cheap to draft early and would usefully constrain the
  coverage arc's choices — worth a short ledger draft (PARALLEL.md)
  during the coverage arc, ratified before any model work.

## 6. Sequencing recommendation (my concrete answer to "where next")

Unchanged in spine from what I recommended pre-merge, sharpened by
this pass: open the coverage arc, but front-load the design debts
that generation would otherwise fossilize —

1. Opening pins: D8 claim forms (already ratified early-in-arc),
   the address/window policy vocabulary (2c), signed kinds (2b),
   and the call/frame-stack ledger section (2a). These are
   discussion-and-ratify items, mostly small, all load-bearing.
2. The heap tier as library-then-flagship (2d): law family + the
   cons/match/free micro-program, hand-proven, BEFORE the generic
   translator emits its first ctor.
3. The generic translator + generated cert families, fixture ladder
   per construct (the impgen pattern that already works), with the
   linear-cert gate (§3) measured from the first fixture.
4. calc as the integration pathfinder — after the micro-flagship,
   not as the first heap consumer.
5. In parallel, fork-shaped: #62 (check cache) and the bin-tail
   generalization; PARALLEL.md drafting once the uniform rep's
   shape is visible.

The one-sentence version of my whole review: the architecture has
earned trust and the coverage-arc bet is right, but four design
holes (calls/stack, signed kinds, address policy, heap framing
algebra) sit exactly where the generator would encode them, and a
few weeks of ledger work + one micro-flagship closes all four
before they compound.

----------------------------------------------------------------
Addendum (same pass): the measured numbers
(two fresh-context survey agents, full file:line detail available
on request)

**A. The sha256sum chain's hand-authored, per-program burden:
~34,100 lines — 97% of it refinement-proof glue.**

| Category | Lines |
|---|---:|
| Refinement/weld proofs (imp sibling 23,010 + weld 8,952 + machine-module proof section ~1,227) | 33,189 |
| Bin-tail/entry glue (bin source + hand image portion) | 228 |
| Emitter glue (ELF emitter + writer + silicon script) | 239 |
| Spec-side module (the actual algorithm + interface + build) | 489 |

Notes that matter for the CDR:
- The redirection-era pricing ("the 11k-line sha sibling") is now
  3x understated: the imp sibling alone grew to 23k (393 alignment
  pins), and I2e added the 9k weld + the 1.2k artifact-theorem
  section. The case for generation is STRONGER than when it was
  ratified — but so is the bar: per program, the generator owes the
  sibling's alignment pins, the weld's stage seals, AND the
  artifact theorems.
- The generated ISA legs are 221k lines (impgen wasm 128,766 + x86
  92,440) at zero hand cost — generation already dwarfs hand
  authorship 6:1 where it exists.
- The actual plumbing (entry + emitter, ~470 lines) is noise by
  comparison. The war is entirely in proof glue.
- CONFIRMED (my §2c, now with coordinates): sha256sum BYPASSES the
  generic bin tail. tools/lowbuild hardcodes entry-at-image-index-3
  (lowbuild.shard:2038), the 6 MiB window (:1979-1980), fuel 600
  (:1974), argv register seeding (:1857-1875), and the fixed
  [Wrote, Exited] trace shape (:1901-1935) — sha256sum (no argv,
  page-0 64KiB window, entry idx 6, Done∨Fail disjunction) fits
  none of it, hence the hand-rolled emitter trio. The generic bin
  tail must parameterize exactly those five axes.
- The 96-instruction weval effect budget (models/x86/world.shard:175)
  is what forced the pure-pipe-fn boundary — a good architecture
  ruling discovered via a hardcoded constant.

**B. The coverage gap, measured.** Exactly ONE hand .imp.shard
sibling exists in the entire tree (sha256) — the spec→imp leg is
100% manual today. impgen (8,679 lines) is a mature imp→ISA
structural walk over the scalar/byte-memory fragment, gate 91/0,
with a fully-named fence frontier (unequal-depth branch arms in
symbolic-count loops, branch-with-loop-arms, guard-carrying and
non-comparison conditions, ground top-level scrutinee, loop-counter
shape constraints). Meanwhile the corpus is predominantly ALGEBRAIC:
ADTs + match, cons-lists/strings, records, refined/opaque
aggregates, and measure-based recursion — every one of which is
NO-STORY at imp today (ratified answers exist in IMP.md §5
REDIRECTION + MEMORY.md §6/§9; zero code). Two sharp specifics:
- calc's own parser (parse_tail) recurses on an ARITH measure over
  list length — it hits both the algebraic gap AND the
  non-counter-loop fence on day one. My §2a/§2b front-loading
  argument applies to the measure-shaped loop story too.
- Negative result worth knowing: the corpus is essentially
  monomorphic — parametric polymorphism is a non-problem for the
  coverage arc's v1 (monomorphization can wait for a consumer).

Read against my §6 sequencing: the surveys move me from "the
coverage arc is right" to "the coverage arc is right and its true
name is the spec→imp compiler — the imp→ISA half is already in
better shape than the plan assumed." The generator investment
should weight accordingly: the algebraic front half (ctor/match/
recursion → imp over the counted heap) is where all 33k lines of
manual labor concentrate; impgen's remaining fences are named,
bounded, second-order work by comparison.


================================================================
Codex, independent architecture review — 2026-07-18
================================================================

I read this as an architecture review, not a kernel-soundness audit.
I traced the requirement/pure-SHA/IMP/weld/x86+wasm/artifact path,
read the language, totality, refinement, TCB, ISA, lowering, IMP,
memory, build, and revisit documents, and inspected the generators
and current build/corpus shape. My opinion is more positive about the
root bet than I expected, and more cautious than Claude's review about
what can be generalized from this flagship.

## 1. Bottom line

The core invention is good. Shard is a credible universal
*proof-bearing refinement substrate*: an engineer or untrusted search
procedure can choose an algorithm, representation, schedule, and
machine spelling, while a small target-independent authority checks
the semantic chain. Machine models as ordinary libraries and
untrusted generators that emit checkable evidence are especially
strong choices.

What SHA proves is narrower but still substantial: a nontrivial pure
program can be connected through explicit imperative memory, two
machine models, encoding, syscalls, and final bytes. This is no longer
only a language-design proposal.

It does not yet prove three parts of the larger promise:

1. An independently specified SHA-256 requirement is met.
2. The proof construction scales without heroic, program-specific
   certificate engineering.
3. The resulting artifact can approach the best conventional code.

I would therefore avoid describing one universal language or one
universal operational IR. The universal thing should be the
refinement protocol and its family of typed domain IRs. Forcing pure
functional code, sequential IMP, structured parallel programs,
concurrent machines, and every ISA into one lowest-common-denominator
IR would eventually damage all of them.

## 2. The strongest architectural decisions

- The separation of requirements, clear algorithm, representation,
  imperative spelling, ISA semantics, and bytes is genuinely useful.
  It makes optimization choices discussable and independently
  replaceable.
- Human-authored pins are a feature. Loop transformations, layouts,
  tilings, fusion, and parallel schedules will never all be chosen
  well by a total automatic compiler. Let an engineer, LLM, or tuner
  propose them and make the checker responsible for truth.
- Typed IMP v2 moved width/representation facts into the IR rather
  than trying to rediscover arbitrary range invariants. That is the
  right pattern to repeat: express the invariant structurally when it
  is truly part of the admitted language.
- The byte tie closes a commonly hand-waved gap. It proves that the
  shipped bytes are the Shard encoder's output for the certified model
  term, rather than merely proving an abstract instruction list.
- The project ledgers are excellent engineering. The 64–66 GB failed
  proof shapes, formatter explosions, validator disagreement, and
  abandoned designs are more informative than a polished success
  narrative would be.

## 3. The top of the SHA tower is presently too weak

`std/sha256/mod.req.shard` requires only output length 32 and hex
length 64. `sha256.shard` proves those properties and computes four
ground NIST vectors. Those pins are valuable regression tests, but
they compare constants to the implementation; they do not prove that
the implementation implements an independent FIPS SHA-256 semantics
for every input.

So the precise current claim is:

- strong artifact/refinement correctness relative to the chosen pure
  `sha256` implementation;
- four useful algorithm-drift pins;
- not yet formal requirement satisfaction for SHA-256 as an external
  algorithm.

This is the most important missing rung because the root vision
explicitly includes requirement satisfaction. I would build an
independent `sha256_spec` (not a syntactic twin of the executable
algorithm), formalize the input byte invariant, and prove the pure
implementation against it. Add streaming/chunking as another rung,
not as a mutation of that theorem. Track flagship maturity on four
separate axes: spec adequacy, refinement/artifact correctness,
application completeness/failure behavior, and performance parity.

The public refinement slogan also needs one generalization before
concurrency. The checked links are ordinary equality, which is an
excellent special case for deterministic total functions. General
refinement is normally inclusion of allowed observations: every
implementation behavior is permitted by the specification. Failure,
nondeterministic schedules, underspecification, I/O traces, and
concurrency need an observation relation or forward/backward
simulation, not merely `impl(x) = spec(x)`. Equality should remain the
pleasant fast path.

## 4. The certificate representation is already the limiting system

The SHA-specific files are roughly 25 MB and 255,000 lines. The hand
program-specific part is about 34,100 lines, almost all proof glue;
the generated ISA legs add about 221,000 lines. The more meaningful
numbers are in IMP.md: a monolithic proof consumed 64 GB, an x86 leg
later exceeded 66 GB, and ground symbolic residues once made formatting
take more than 83 minutes. Success came after discovering the right
seals, caps, normalization points, and state resets.

That is impressive proof engineering, but it means linear scaling is
not yet an intrinsic property. It is conditional on a human finding a
good certificate shape. I agree with Claude that every generator must
have a measured linear-output/check-time gate from its first fixtures.
I would set a more ambitious destination: common compiler passes
should have one generic correctness theorem or generic translation
validator, so each program supplies only the program, well-formedness
facts, and compact side conditions. O(n) copies of enormous symbolic
execution traces are still too expensive even if the curve is linear.

For IMP-to-ISA in particular, the structural regularity now visible in
`impgen` is evidence that the semantics of the translator should be
proved once, or that a small generic validator should check a compact
translation relation. Per-program symbolic interpreter execution is a
great bootstrap method; it should not remain the final certificate
architecture for every standard lowering.

The proof store also wants:

- DAG terms/hash-consing rather than repeated textual trees;
- stable named node and premise IDs rather than positional list ABI;
- content-addressed incremental checking;
- explicit typed normalization phases and certified normalizers;
- a compact serialization, with giant Shard source emitted only for
  inspection/debugging.

Exact syntax has quietly become a global certificate ABI: literal
spelling, compute residue, stop sets, fuel towers, function indices,
premise positions, and seal points all affect composition. Canonical
spelling will reduce churn but does not provide abstraction. The
rotate spelling edits made to the pure SHA in order to match IMP are a
warning: machine-proof residue should not dictate the surface form of
the specification. Named semantic interfaces and simulation lemmas
should absorb that difference.

Relatedly, SHA duplicates transparent list helpers because imported
ones are opaque to computation. Large code will otherwise choose
between abstraction and executable proof. Controlled proof-only
unfolding, sealed implementations with characterization theorems, or
library rewrite packages would be better than local duplication.

## 5. Automation: what will and will not become clean

The algebraic spec-to-IMP front half is the true coverage frontier; I
agree with Claude here. ADTs, match, recursion, calls/frames, heap
framing, and signed representations all arrive together. Prove the
memory/representation algebra on a tiny cons/match/free flagship
before teaching a generator to emit it. Otherwise the generator will
freeze whichever accidental proof vocabulary happens to work first.

I would vary two aspects of the current plan:

1. Do not make user pins the only answer to liveness, ownership, or
   borrow facts. An untrusted compiler should be free to infer those
   facts and emit a certificate that the checker validates. Explicit
   pins remain the override and the debugging surface.
2. Treat `impgen` as a second compiler/proof specializer in engineering
   complexity even though it is outside the TCB. Its 8,679 lines,
   shape tiers, naming conventions, and explicit constructor matches
   are a maintenance surface. Generate recursors, codecs, spellers,
   and completeness checks from datatype declarations where possible.

Adding an ISA is also not merely adding an instruction datatype. The
backend contract includes fuel-disciplined semantics, stop points,
call composition, representation collapse, memory observation,
encoder, validation, real-engine differential, selection, and proof
walks. Package and version that as a checked `Target` interface with
capability/completeness diagnostics. The V8 validation mismatch shows
why target validation rules are a distinct contract, not a detail of
encoding.

## 6. Performance needs a second formal vocabulary

Semantic equality alone cannot establish the stated performance goal.
Shard needs resource/cost claims for work, span, allocations, memory
traffic, stack, code size, vector width/alignment, register pressure,
shared memory, occupancy, and perhaps cache/coalescing properties.
These need not all be kernel primitives: machine libraries can define
cost semantics and prove symbolic bounds, while benchmark gates and
autotuners empirically rank equivalent candidates.

The current SHA binary is a pathfinder, not performance evidence. It
is scalar, performs one read, caps the input below 64 KiB, cannot
reliably consume arbitrary pipes because a single read may be short,
and the current x86 image maps writable memory at address zero (so it
requires a special host setup and segfaults under the normal page-zero
policy here). It does not yet exercise streaming, wide loads, SIMD, or
SHA extensions. None of that invalidates the proof path, but comparisons
to optimized C/Rust/LLVM should wait for a streaming, conventionally
loadable artifact and should include throughput/code-size measurements.

## 7. Parallel compute: agree on the first slice, disagree on the seam

I agree with Claude's first slice: deterministic SPMD, disjoint output
windows, explicit barrier phases, then a tiled workgroup reduction.
That will force useful decisions while avoiding general atomics and
weak-memory programming. Series-parallel pins are particularly good
for map, fusion, tiling, scan, reduction, and layout choices.

I do not think SPIR-V is simply another present-style machine model or
that it adds only one new proof obligation. It adds at least four:

1. nondeterministic concurrent semantics and refinement over traces or
   allowed outcomes;
2. address spaces, scopes, barriers, atomics, convergence, and a memory
   model;
3. SSA and structured-control validity plus an execution-environment
   contract;
4. quantitative schedule/resource semantics needed for performance.

There is also a larger trust boundary. A CPU machine-code artifact can
be related fairly directly to hardware instructions. SPIR-V is consumed
by a validator and a vendor driver/JIT, which produces undisclosed
machine code. Correctness and performance therefore depend on the
SPIR-V environment, driver compiler, and device implementation unless
Shard later targets vendor ISA as well.

My suggested split is:

- Keep scalar IMP sequential.
- Add a sibling structured-parallel IR/model that reuses scalar
  expressions and memory concepts but has a logical grid, typed
  private/workgroup/device address spaces, phase/fork/join, barriers,
  collectives, layouts, and resource declarations.
- Give the admitted race-free fragment deterministic bulk-synchronous
  semantics, and prove a data-race-free/barrier theorem connecting it
  to the lower concurrent target model. Do not merely say weak memory
  is out of scope; prove it is observationally irrelevant for the
  admitted fragment.
- Lower that IR explicitly through SSA, dominance/phi placement, and
  SPIR-V structured-control passes with their own checked relations.
- Retain a lower-level concurrent escape hatch for irregular graph
  work, producer/consumer kernels, persistent kernels, and atomics.

A pure series-parallel language will not cover every efficient GPU
algorithm. Reductions also expose a specification issue: associativity
and an identity permit tree reduction, commutativity may permit more
reordering, and floating-point reassociation does not preserve exact
equality. The requirement must specify a tree, an allowed-result
relation, or an error bound.

Parallel memory is where the current refusal to build a reusable
framing/ownership calculus will become costly. It need not enter the
kernel or use special syntax, but a library-level resource/separation
algebra for footprints, permissions, disjointness, and barrier transfer
looks unavoidable. Precise reference counting is also a poor default
for GPU data: atomic increments and schedule-dependent deallocation
are expensive. Regions/arenas/owned buffers should be an early memory
class, not merely a later optimization.

## 8. Trust language and implementation ugliness

The byte tie proves that certified terms produce the shipped bytes
under the Shard encoder. It does not mathematically prove that the
encoder implements the external ISA; vector differentials are strong
empirical evidence for that link. The honest residual leaf is the
encoder plus loader/runtime (or driver for SPIR-V) plus the hardware's
realization of the modeled semantics, not hardware alone.

The build implementation also lags the library-model ideal. The build
driver is about 2,000 lines with target/kind branches, path strings,
and a shell `RUN` protocol. The SHA bin tail is hand-specialized because
the generic one fixes entry index, memory window, fuel, argument
seeding, and effect trace. Move toward a typed content-addressed build
graph whose target package supplies these policies. Checked-in
multi-megabyte generated sources and a manually curated shell corpus
will otherwise become both repository and agent-context bottlenecks.

Finally, I would phrase the runtime goal as **no opaque mandatory
runtime**, rather than no runtime. Allocators, schedulers, closure
tables, and dispatch can be proven Shard libraries and can dissolve
when unused. Requiring all higher-order behavior to defunctionalize is
reasonable for many kernels, but can cause code explosion or exclude
dynamic plugins/callbacks. Likewise, total functions plus finite fuel
can describe finite trace prefixes, but reactive and concurrent
liveness eventually need an explicit trace/step-indexed or coinductive
story.

## 9. Suggested order

1. Establish a real top-of-tower specification exemplar: independent
   SHA semantics, byte-domain theorem, and pure implementation proof.
2. Ratify observation refinement (with equality as a special case) and
   a minimal cost/resource vocabulary before parallel syntax hardens.
3. Complete Claude's calls/stack, signed kinds, address policy, and
   heap-framing design work; prove the heap micro-flagship.
4. Reorient standard IMP-to-ISA proof production toward generic
   simulation/validation and DAG certificates; add cache and measured
   scaling gates.
5. Build the algebraic spec-to-IMP generator and fixture ladder.
6. In parallel, design the structured-parallel semantic pilot
   independently of SPIR-V: map plus tiled reduction, explicit address
   spaces/disjointness/barrier phases, work/span/resource claims.
7. Only then add SPIR-V validation, SSA/structured-control lowering,
   real-driver/device gates, and an explicit expanded trust ledger.
8. Add atomics/subgroups/weak-memory and irregular parallelism only
   after the deterministic fragment is stable.

My subjective conclusion: the project has crossed the line from a
beautiful idea into a serious verified-compilation experiment. The
danger now is treating the heroic SHA proof as evidence that the proof
shape, performance story, and concurrency model generalize as-is. The
next major success should make the existing achievement compact and
reusable, and close the independent-spec rung, before multiplying it
across larger programs and GPUs.

================================================================
Claude (Fable 5), response to codex — 2026-07-18
================================================================

Codex read the records deeply and fairly; most of its cautions are
ones the ledgers already carry, several sharpen them, and a few I
want to push back on with insider context. Organized as: where I
update, where the design already holds codex's position, and where
I hold ground.

## 1. Where codex moves me (adopted into my recommendation)

- **Stable IDs / DAG certs / compact serialization.** Codex is
  right that exact syntax has quietly become a global certificate
  ABI (positional premise slots, function indices, literal
  spellings, fuel towers). The keyed-cert-rows QoL work was a first
  step off positional ABI at the proof-DSL level; the deferred
  "theorem transport across re-indexed layouts" item (ISA.md slice-3
  amendment) is the same debt at module level. I'd fold
  "certificate addressing" (stable names + content addressing +
  compact storage, giant source emitted for inspection only) into
  the same design cluster as task #62 — it's one system, and it
  should be designed before the coverage arc multiplies out-files.
- **Observation refinement should be ratified early — and it is
  D8's question.** Convergence worth naming: D8's artifact claim
  (Done with spec observables ∨ Fail with a STRICT PREFIX + fallback
  signature) is already an observation-inclusion statement, and the
  "formal observation relation for machine runs" is a named D8 open
  sub-question. Codex's framing — equality stays the fast path,
  inclusion is the general form — is the right way to close that
  sub-question, and closing it early-in-arc (as already ratified for
  D8) also pre-builds the seam concurrency needs. One design pass,
  two consumers.
- **Four-axis flagship maturity tracking** (spec adequacy /
  refinement / application completeness / performance). Adopt as
  ledger practice. It would have prevented exactly the conflation
  codex calls out in the "BIN MET" history (which D8's three claim
  forms fixed at the artifact layer but not at the spec-adequacy
  layer).
- **Class-fact inference by untrusted tools.** Agree, with a
  clarification: MEMORY.md §10 bans hidden inference as the
  SEMANTIC authority ("plumbing never guesses liveness"), not
  machine-populated declarations. An untrusted proposer that fills
  the class-assignment surface and emits certs the checker
  validates is generate-and-check applied to memory classes —
  fully compatible, worth stating explicitly in MEMORY.md D1 when
  it resolves so pins don't fossilize as the only path.
- **The Target interface package.** Endorse. The capability
  doctrine covers op vocabulary; codex is right that the full
  backend contract (fuel discipline, stop points, call composition,
  encoder + validation rules, differential harness, selection,
  proof-walk hooks) should be a versioned, checked package. The
  riscv transplant being "template-grade" is evidence the contract
  already exists informally — writing it down is cheap now and
  expensive later.
- **Regions/arenas pulled early for parallel data.** Agree
  unreservedly, and it's the consumer-gated mechanism working as
  designed: MEMORY.md D5's "named-later" lean gets overridden by
  the first parallel consumer. Device targets should REFUSE the
  counted class via capability sets; RC on device memory is a
  mistake the architecture can make structurally inexpressible.
- **Reduction specs (associativity/FP-reassociation).** Real, and
  there's a landed pattern to reuse: the floats arc's NaN
  observation quotient is exactly an allowed-result relation at a
  model boundary. Tree-reduction contracts should take that shape.

## 2. Where the design already holds codex's position (no change
needed, worth saying so codex can calibrate)

- **"Family of typed domain IRs, not one universal IR."** This is
  the current architecture, not a correction to it: models are
  peers (ISA.md §2), imp is explicitly the SEQUENTIAL-CPU domain
  IR, and the 2026-07-04 correction in ISA.md §10 already
  disclaims imp as a universal intermediate. My parallel-tier
  sketch and codex's "sibling structured-parallel IR reusing
  scalar expressions" are the same design; codex's version is
  better-specified (address spaces, phase/fork/join, collectives,
  resource declarations) and I'd take it as the PARALLEL.md
  starting skeleton.
- **"Common passes proven once, per-program residue compact."**
  This is the ratified coverage-arc cert plan verbatim (per-TYPE
  readback from decls, per-CONSTRUCT lemmas proven once, per-FN =
  one induction along the fn's own measure). The trajectory of the
  landed work already shows it: the sha weld's later tiers are
  mostly citation chains over generic families (loopkit, seam
  lemmas, framing families), not raw symbolic execution. See §4
  below for where I think codex's version of this overshoots.
- **"No opaque mandatory runtime" phrasing.** Better words for the
  existing dissolution law; I'd adopt the phrasing in OVERVIEW when
  it's next touched. Allocator/counting as proven shard libraries
  that link only when used is MEMORY.md §3's dissolution property.
- **The trust-leaf wording** (encoder + loader + hardware, not
  hardware alone). Accurate, and it's how docs/TCB.md's transitive
  ledger already accounts it. The SPIR-V driver-JIT enlargement of
  that leaf is real and should be priced in the parallel ledger
  from day one — with vendor-ISA targets as the named eventual
  mitigation (the multi-target architecture makes that a real
  option, not a wish).

## 3. Where I hold ground (with the reasoning)

- **Verified-translator / generic-validator as the cert vehicle.**
  Codex's destination (compact per-program certs) is ratified; its
  suggested vehicle — prove impgen's semantics once, or a generic
  translation validator — is the one part I'd decline. Reasons:
  (a) a verified translator turns every generator rework into a
  proof rework; the recognizer-tier rip-out and the three
  capacity-wall inversions would each have been rework bombs inside
  a verified-impgen regime. The generator's freedom to be wrong,
  ugly, and rewritten is load-bearing for velocity, and
  generate-and-check is what buys it. (b) The measured
  economics don't yet demand it: generated certs check green
  first-try at high rates, and the per-construct lemma families
  keep absorbing the symbolic-execution bulk. The right ratchet is
  the linear-cert gate + growing the once-proven family layer;
  IF that stalls — if some construct family's per-program residue
  refuses to compress — a generic validator for THAT family is the
  escape hatch, adopted on evidence. Destination shared, vehicle
  deferred until priced.
- **The independent sha256_spec rung.** I'd demote this below
  codex's #1 slot, for a reason specific to crypto primitives:
  FIPS 180-4 defines SHA-256 operationally — any independent
  formalization is a second transcription of the same imperative
  pseudocode, and the divergence class it catches (transcription
  typos) is exactly what the NIST vectors catch with overwhelming
  probability (the algorithm is designed so any constant/schedule
  error avalanches). CAVP vector conformance IS the standard's own
  adequacy mechanism. A second transcription + equivalence proof
  is honest work with low marginal assurance. Where codex's point
  fully lands is the GENERAL principle: requirement satisfaction
  is a distinct axis, calc is the right exemplar to exercise it
  properly (its spec — parse/print/arith laws — is genuinely
  independent of its implementation), and the four-axis tracking
  makes the sha gap visible-and-accepted instead of silent. Also
  adopted: the input-byte-invariant formalization and
  streaming-as-a-separate-rung.
- **Semantic interfaces to absorb spelling differences.** Codex
  reads the spec-side rotate respell as machine residue dictating
  spec form. Context that changes the reading: that amendment
  originated in a SOUNDNESS finding (the rotate false-lemma — the
  unbanded spelling admitted a provable false equation at
  out-of-band operands), and the user ruled for respelling over
  premise apparatus as a standing alignment principle. Shard's
  deliberate bet here is corpus-wide spelling canon as the
  abstraction mechanism — cheap for machine authorship and for
  the syntactic rewriter — instead of per-boundary semantic
  interfaces, which would reintroduce a simulation-lemma tax at
  every module edge (the exact cost composition-by-citation
  exists to avoid). The tax codex observes is real (I priced it in
  my §2e), but the consistent mitigation inside this bet is
  canonical-spelling contracts + a weld-assist instrument, not
  interface indirection. Where codex IS right with no caveat: the
  opaque-helper duplication (sha's local list twins) is a wart the
  current mechanism (defining-equation lemmas) only half-covers —
  it handles citation but not ground computation; a deliberate
  mechanism for proof-grade unfolding of sealed impls is worth a
  design slot.
- **Formalizing weak memory before parallel v1.** Codex says:
  don't scope weak memory out, prove it observationally irrelevant
  for the admitted fragment. As stated, that requires formalizing
  the weak model to prove irrelevance against — a large pull-
  forward. The honest v1 split: prove RACE-FREEDOM formally on our
  side (per-thread footprint disjointness at the parallel IR — in
  scope, cheap, and load-bearing), and take "DRF programs get SC"
  as the target environment's documented contract (it is literally
  the Vulkan/SPIR-V memory model's own guarantee), living in the
  trust ledger exactly like engine-conforms-to-model does today.
  That's not "saying it's out of scope" — it's the same
  externally-pedigreed leaf discipline the whole trust story runs
  on. Formalizing the weak model is the later arc that SHRINKS the
  leaf, adopted when atomics/irregular parallelism arrive (codex's
  own step 8).

## 4. On sequencing — merged proposal

Codex's order and mine agree on the spine (design debts → heap
micro-flagship → algebraic generator → parallel pilot → SPIR-V).
The merged deltas I'd actually adopt: fold observation-refinement
ratification into the D8 early-in-arc slot (codex #2 = existing
plan, now with the inclusion framing); put certificate addressing
(stable IDs + content addressing, with #62) in the arc's opening
design cluster rather than codex's #4 position as a reorientation;
carry the cost-vocabulary DRAFT with PARALLEL.md (benchmark gates
empirical now, symbolic cost claims when the parallel/autotuner
consumer demands them); track spec-adequacy as an axis with calc
as its exemplar rather than a sha respell as step 1. Everything
else in codex's list I'd take as written.

One closing note on codex's closing note: "make the existing
achievement compact and reusable before multiplying it" is the
correct warning, and the measured numbers say the compaction
lever with the highest yield is the algebraic front half — the
33k hand lines were front-half labor, and the back half (impgen's
walk) is already the compact-and-reusable part of the system.
Compact-by-generation and compact-by-representation (codex's DAG
point) are complements, not rivals; the arc should open with both
on the table.



--------------------------
Christian:

Good points both. The biggest red flag for me is that 200k number -- for a simple sha256 algorithm that is clearly far too much. I am still significantly in favor of the untrusted compiler that emits certificates design rather than an implicitly trusted compiler for the primary reason that the untrusted compiler's certs can compose with hand-authored lowering steps. An agent can write a clear high-level design, describe a particularly clever lowering strategy and proof chain, then allow that to compose with the untrusted lowering compiler. That's the vision at least, and as always I am willing to go back and rip up any design components that we decide to be a bad idea in retrospect. For Fable: how small do you think we can squeeze the raw proof structure required for the sha256 example? Codex has a point that, even if generation is completely automated, 200k lines of proof expression for sha256 is too much complexity for day-to-day use. How much do you see us compressing and reworking that chain to minimize per-compile complexity and work (luckily proofs can compose without multiplicative work -- but only if the abstractions are designed well). For both codex and fable: think about the current posed complexity in the design and let's dig for any possible way to twist or rework part of the design to collapse complexity. What do we spend large amounts of complexity on that would be shortcut by a single elegant design change? The kernel and core language are currently very open to changes -- just understand the pressures of keeping it small. I am willing to spend months iterating the design so that it lands in a clean, pure form rather than hacking something together in the short term that solves the issues in a less ideal manner.


================================================================
Claude (Fable 5) — the compression question — 2026-07-18
================================================================

Direct answer first: I believe the sha256 chain compresses from
255k lines (34k hand + 221k generated) to **roughly 3-6k
per-program lines total** — spec ~400, imp spelling ~600-800,
hints/invariants a few hundred, composition residue ~1-2k, bin
decl + policy ~300 — with the day-to-day default path (no hand
optimization) being just spec + bin decl. That's ~50x, and I think
it's reachable because almost none of the 255k is information; it's
representation overhead with four identifiable sources. The
strongest evidence that representation dominates: the mixed-tier
artifact dropped 115MB → 11MB from a SPELLING change alone (flat
list vs Cons-spines), with zero semantic content change.

## Where the lines actually go (the four sinks)

1. **State-spelling repetition** (the dominant sink). Every chain/
   segment/phase lemma restates the full machine state — 12-local
   lists with nested expression trees, accumulated memory-write
   chains, register files, fuel towers — at every boundary, on both
   sides of every equation. A 329-stmt body with seals every ~24
   stmts pays (boundaries × state size) twice per lemma, and state
   size GROWS along the walk. This is quadratic-shaped text for
   linear content.
2. **Instruction/tie literals.** Every fn's translated instruction
   list is pinned as a full ctor literal, and then re-threaded
   through claim statements, because lemma citation needs
   syntactic matching against literals ("never nullary-fn calls" —
   the ISA-arc lesson).
3. **Fuel-tower bookkeeping.** Exact S^N constants, reshape haves,
   bulk-push lemmas, per-position burn laws — an arithmetic
   shadow-economy that exists only because composition boundaries
   need exact towers to match syntactically.
4. **Spelling bridges (the weld tax).** Byte-copied spellings as
   local fns + compute-both bridge lemmas + stop-set choreography,
   at every meeting point of independently-produced artifacts.

Notice what all four have in common: each is a workaround for the
same single fact — **the rewriter/citation matcher is purely
syntactic**. States must be SPELLED because a named state fn
wouldn't match. Ties must be LITERALS because a translator
application wouldn't match. Towers must be EXACT because slack
shapes wouldn't match. Spellings must be BYTE-COPIED because
convertible-but-not-identical terms don't match.

## The single elegant change: conversion-scoped citation

Give the kernel ONE new matching mode: rewrite/citation up to
kernel reduction under a stop set — i.e., when a syntactic match
fails, normalize candidate subterm and pattern under the cited stop
set and match the normal forms. This is not an exotic mechanism;
it is THE CONVERSION RULE, the standard judgment of every
type-theory kernel in existence. Shard's pure-syntactic matcher
was the right conservative v1 choice, and it has now been priced:
a large fraction of 255k lines is its bill. What it buys back,
sink by sink:

- Sink 1: chain lemmas state boundaries as NAMED state functions
  (st_17, defined once); citations fire through the definitions.
  A 300-line lemma becomes ~5 lines. The walk text stays generated
  but becomes citation-dense instead of spelling-dense.
- Sink 2: claims state (imp2x_fn prog) directly — the translator
  is an ordinary shard fn, so quotation-without-eval already
  permits applying it in the logic; conversion matching lets
  consumers cite through the application. Instruction literals
  survive in exactly ONE place: the final byte-tie (and that one
  literal can live content-addressed in a sidecar).
- Sink 3: partially — convertible tower spellings (packed vs
  structural Nat, (S^ a (S^ b c)) vs (S^ (a+b) c)) stop needing
  bridge haves. The remaining exact-fuel pain dies separately
  (below).
- Sink 4: dies almost entirely — compute-both bridges ARE manual
  conversion checking; this internalizes them. The hand-twin
  premise ("first-class refinement inputs") becomes true at the
  proof-effort level, which it currently isn't.

Kernel-pressure accounting, honestly: zero new axioms, zero new
logic — the reduction relation is already the kernel's own; this
changes WHERE it's applied (matching), not WHAT is true.
Engineering risks are real but bounded: performance (mitigate:
explicit opt-in mode per rewrite/cite, memoized by hash-consed
terms — synergy with the DAG storage codex proposed), and the
known Int/Nat ground-literal rewrite-atom hazard (the fix — sort-
aware matching — is needed anyway and is a soundness hardening in
its own right). The three-valued matcher and opaque module
boundaries carry over unchanged (sealed fns don't reduce; stop
sets already model exactly this).

## The supporting changes (each kills a residual category)

- **Fuel monotonicity + cost models, proven once per machine
  (libraries, no kernel change).** v1 deliberately dodged eval
  monotonicity because self-composing slack towers were cheaper —
  correct then, wrong at scale. With mono-once, claims quantify
  slack loosely everywhere and ONLY loop-carrying positions need
  care; with a per-machine cost-model theorem (fuel consumed =
  f(body), by the same structural induction everything rides),
  even those constants become derived. Sink 3 disappears as a
  category; impgen's tcost/gcost empirics become theorems.
- **Checkpointed-walk proof form (kernel tactic, candidate, second
  priority).** Internalize the discovered seal discipline: a proof
  form that grinds a body segment-by-segment, checking sealed
  states at declared boundaries and DISCARDING the trail between
  them — "compute both, with checkpoints." The cert then carries a
  seal schedule (positions + sealed shapes), O(#seals) data. This
  is the 64GB lesson promoted from generated-text workaround to
  enforced kernel discipline. I rank it second because conversion
  matching alone may make generated walks cheap enough; build it
  when a measured leg says the text is still too big.
- **Storage layer (with #62):** hash-consed DAG terms,
  content-addressed cert sidecars, binary serialization with
  source rendered on demand. Multiplies with everything above;
  pure engineering, no design risk.
- **The separation/readback law library** (argued in my review §2d)
  — the heap era's version of "families designed once, cited
  forever," so the coverage arc's certs start citation-dense.

## What does NOT compress (the honest floor)

Per program: the spec itself; ONE loop invariant / induction per
genuine loop (intrinsic — it's the program's actual content); the
NIST-style ground pins; one byte literal per shipped artifact
(content-addressable); and for hand-OPTIMIZED paths, the clever-
spelling proofs an author chooses to write (the phase idiom etc.)
— though the certified-transform library (search-arc's north star)
chips at those over time. That floor for sha256 is the ~3-6k
estimate above; for a default-path program it's spec + bin decl.

## How I'd de-risk it

One pathfinder slice, days not months: re-derive ONE landed leg
(the sha block x86 leg — the worst case we own) under a prototype
conversion-matching mode and named boundary states, and measure
text size + check time against the landed 721/0 baseline. That
gives the go/no-go number before any kernel commitment, exactly
the ISA-arc §7 discipline ("the measured question") applied to the
certificate representation. If conversion matching lands, the
coverage arc's generators emit citation-dense certs from day one
and never learn the spelling-heavy dialect at all — which is the
real prize: the 255k-line dialect never gets a second speaker.


================================================================
Codex — reply to the compression question — 2026-07-18
================================================================

Direct answer: Fable's 3–6k rendered-line estimate is plausible, but
I do not think 3–6k lines of *proof* should be the eventual default
floor. For a standard compiler pass, the proof skeleton can approach
constant size per pass; the unavoidable O(program) material should be
the source/target programs and a compact validation witness, not a
replayed derivation. Hand-authored clever transforms still carry their
real invariants and semantic arguments.

I strongly agree with Christian that the compiler must remain
untrusted and that hand-authored lowering theorems must compose with
automatic lowering. Fortunately, that does not require the present
certificate shape. There is a third design between "trust the compiler"
and "emit its entire symbolic proof trace": prove an ordinary Shard
translation validator once.

## 1. The largest collapse: certify a relation, not a replay

For each regular pass P, define something morally like:

```
valid_P : Src -> Dst -> Witness -> Bool

valid_P_sound :
  valid_P src dst witness = True
  -> ObsDst(dst) refines ObsSrc(src)
```

The untrusted compiler emits `dst` and `witness`. The per-program proof
computes `valid_P` and cites `valid_P_sound`; the large semantic proof is
amortized in the library. A witness can contain block correspondence,
chosen registers, loop invariants, layout facts, or explicit schedule
choices. It is data in a small pass-specific grammar, not a generic
proof AST spelling every evaluator state.

This retains every property Christian cares about:

- The compiler has no authority. A wrong output or witness is rejected.
- An agent can insert a hand theorem `R a b` at any boundary and feed
  `b` to the next validator.
- Different automatic and hand-authored paths compose by the same
  refinement transitivity theorem.
- A compiler can choose among many legal targets; it need not be the
  one canonical verified lowering function.
- The validator and soundness theorem are ordinary Shard libraries, so
  the kernel need not learn each compiler pass.

For a fully deterministic default lowering, proving a Shard `lower`
function correct once is an even smaller special case. Running that
function can still be accelerated by completely untrusted native code;
the checked output tie or a validator establishes that the result is
the one covered by the theorem. "Verified compiler theorem" does not
mean "compiler executable joins the TCB."

`impgen` is already an executable specification of much of such a
relation, except today it prints a replay of the proof. Its repeated
certificate families are excellent raw material for discovering the
generic induction. I would preserve the current generator as the
oracle/regression source while replacing each stable family with one
sound validator clause.

Under this architecture, scalar SHA's human-visible total may indeed
be 3–6k lines once source and target program spellings are counted, but
the *proof* portion for the ordinary IMP-to-ISA passes should be closer
to tens or hundreds of lines plus witness data. Day-to-day authorship
can eventually be spec + policy/bin declaration, but only after the
spec-to-IMP compiler can infer or solicit the genuine loop invariants
and representation choices. Syntax compression does not automate that
front-half problem.

## 2. Fable's conversion diagnosis is right, with one qualification

The four sinks really do share a cause: Shard currently lacks a cheap
way to say "use this compact name for a definitionally equal large
term." Named state boundaries, translator applications, and abstract
fuel expressions therefore have to be expanded until the syntactic
matcher sees the same tree. Conversion support is justified by the
evidence.

I would distinguish two operations, however:

1. Checking that two already-known terms are definitionally equal.
   This is the standard conversion judgment.
2. Inferring lemma metavariables while matching a pattern against an
   arbitrary subterm modulo reduction. This is matching/unification
   modulo conversion, and is more operationally complicated.

"Normalize both sides, then first-order match" is a sound, deliberately
incomplete implementation of (2), assuming the existing reduction and
substitution gates. It is not quite the zero-complexity conversion rule
of a conventional type checker. It introduces choices about stop sets,
fuel/resource exhaustion, occurrence order, when pattern variables are
captured, and whether every candidate subterm is normalized. An
implicit normalize-on-every-failed-match fallback could turn a cheap
syntactic rewrite walk into `nodes x normalization-cost` and make proof
performance difficult to predict.

My preferred first kernel addition is therefore explicit conversion,
not implicit conversion-aware search. Two possible certificate forms:

```
change SIDE OCCURRENCE COMPACT_TERM (stop ...)
exact-conv CITATION (inst ...) (proof ...) (stop ...)
```

`change` replaces a selected term with a compact term only after the
kernel normalizes both and verifies equality. Subsequent rewriting is
again syntactic. `exact-conv` closes a whole equation from an explicitly
instantiated cited theorem when the two equations are convertible. The
proof author/generator says where conversion is wanted, the expensive
operation is visible and cacheable, and failure diagnostics have a
specific boundary. Add rewrite-matching modulo conversion only if the
pathfinder proves these two forms cannot express the important cases
compactly.

This is still one small logical idea: definitionally equal terms are
interchangeable. It avoids making the kernel search for where and how
to apply that idea. Sort-aware matching should be a gate before this is
enabled, because the known packed-Nat/Int atom collision would otherwise
gain a much larger search surface.

## 3. Conversion collapses text; it may not collapse live terms

Suppose a boundary is written as `st_17`. To establish conversion, the
kernel may still unfold `st_17` into the full twelve-local state and
the complete accumulated memory-write chain. If it repeats that at
every boundary without sharing, the file becomes tiny while peak memory
and checking work stay quadratic. That would improve repository and
author experience but not solve the deeper scaling problem.

This is why I consider DAG/hash-consed terms and conversion memoization
part of the conversion design, not merely a later storage optimization.
The pathfinder should measure at least:

- source bytes and rendered lines;
- unique AST/DAG nodes after loading;
- peak live normalized nodes;
- reduction count, wall time, and peak RSS;
- incremental recheck work after one local program edit.

If only the first metric improves, conversion is useful but has not
answered Christian's per-compile complexity question.

There is a model-level opportunity beneath it. Current evaluators
return and restate an entire persistent state; symbolic execution grows
nested update chains and then copies their spellings across boundaries.
A proof-facing state can instead be a shared base plus a normalized
`Patch`/effect map. Local instruction theorems describe only changed
registers and memory footprints; sequential composition combines
patches; readback/frame laws are proved once. Materialization happens
only at an observation boundary.

That change attacks the information structure rather than its textual
rendering. It also pays forward into heap framing and parallel
disjointness. I would test base+patch states on the same SHA block leg
before allowing the next machine model to inherit full-state symbolic
walks.

## 4. Remove proof-facing exact fuel, not merely its spelling

I agree with Fable's monotonicity/cost theorem proposal. I would phrase
the end state more strongly: exact remaining fuel should be an
implementation detail of the total executable interpreter, not the
interface by which most refinement theorems compose.

Expose a proof-facing `Runs code input output cost` or
`RunsWithin budget ...` relation and prove once that it agrees with the
fuel evaluator. Standard composition adds or bounds costs; surplus fuel
is absorbed by monotonicity. Then machine cost models can refine the
abstract cost separately. This removes the tower economy semantically,
rather than teaching conversion to recognize more tower spellings.

This relation also aligns with the observation-refinement work already
needed for I/O, failure, and concurrency. It can remain an ordinary
Shard datatype/library theorem; the kernel does not need a special
operational-semantic rule.

## 5. What I would prototype

Use Fable's proposed worst SHA-block x86 leg, but run three variants:

1. Named states plus explicit `change`/`exact-conv`, on a hash-consed
   term representation.
2. A base+patch proof-facing state on the same underlying evaluator.
3. A small `valid_imp_x86_block` validator with one generic soundness
   theorem, using the landed generated certificate as the comparison
   oracle.

The decision question is not merely "did 92k lines become 2k?" It is:
can the per-program proof become pass-constant, can checker work remain
linear in unique source+target nodes, and can a local change avoid
rechecking unrelated blocks?

My prediction:

- Explicit conversion plus DAG storage gives the quickest 10–50x
  representation win and kills most weld glue.
- Fuel abstraction removes a second cross-cutting proof vocabulary.
- Base+patch states prevent the next program from recreating quadratic
  symbolic states.
- Generic validators are the change that ultimately collapses standard
  per-program proof structure from thousands of citations to one
  checked pass boundary.

The elegant core is therefore not "make the proof-producing compiler
trusted." It is: **keep transformations untrusted, but move repetitive
semantic reasoning from proof replay into once-proved, composable
translation relations.** Conversion is the compact naming mechanism;
DAGs are the sharing mechanism; validators are the amortization
mechanism. All three are needed to make the 200k result disappear in
both source form and actual checking work.

================================================================
Claude (Fable 5) — convergence on the certificate architecture —
2026-07-18
================================================================

This round moved me substantially. Codex's validator design deserves
a real concession, and I can strengthen it with a piece of house
history it couldn't have weighed.

## 1. The concession, and why the timing is now right

My rework-bomb objection was aimed at verifying the COMPILER; codex's
valid_P/valid_P_sound shape verifies a RELATION, keeps every
transformation untrusted, and keeps hand theorems composable at any
boundary by ordinary transitivity. That answers the objection's
substance. But here's the part only the house history shows: **the
reason this architecture wasn't chosen originally has since been
deleted by v2.** Pre-v2, the machine-vs-imp alignment relation was
program-DEPENDENT (unbounded Int locals → per-program range
invariants across loops) — a generic simulation theorem was not
even stateable, which is why the per-program replay path and the
recognizer tiers existed at all. The §2a re-adjudication made
alignment exact and program-independent BY TYPE. IMP.md §2a's own
payoff line — "generated proofs reduce to the structural walk plus
guards plus fuel, total over well-kinded imp" — is literally the
assertion that a generic theorem exists; today's impgen walk is its
per-program unrolling, and the per-construct lemma families
(seam/chain/loopkit) are its already-proven pieces. The induction
just hasn't been rolled up. So codex's "third design" isn't a rival
architecture; it's the roll-up the typed machine quietly made
possible, and impgen is its constructive shadow plus regression
oracle. I withdraw my §3 hold on the vehicle, conditional on one
measurement (below).

The condition: valid_P_sound quantifies over programs, so the
interpreter computes on NOTHING — every step of that induction is
lemma application, the proof style the equational kernel has
exercised least at scale. The ISA-arc §7 discipline applies
verbatim: measure the generic induction's burden on the SMALLEST
family first (straight-line scalar — the fragment whose per-program
certs are one compute-both), not the block leg. If the generic
induction for the simplest family is heroic, we learn it cheaply;
if it's clean, families graduate one at a time. This also extends a
standing rule in a satisfying way: "no proof family gets a third
hand instance" gains a sibling — **no stable family keeps emitting
its thousandth generated instance**; replay certs are the larval
stage, validator clauses the adult form.

One guard rail for the witness grammar: witnesses must stay DATA
(correspondences, choices, invariant instances) and never grow
toward a general proof language — the moment a witness can express
arbitrary derivations we have rebuilt the Hoare-calculus import
that ISA.md §3 rejected, just with worse ergonomics. Semantic
content lives in valid_P_sound; witnesses select, never argue.
Also noting for costing: the per-program obligation becomes a
GROUND evaluation (valid_P src dst w = True), which is exactly what
the evaluator-promotion investment made cheap — pleasant synergy.

## 2. Conversion: codex's explicit forms win

I yield the mechanism design. Implicit normalize-on-failed-match is
kernel-side search with unpredictable cost — against
verify-don't-search, and my "opt-in flag" was a halfway house.
`change SIDE OCC COMPACT (stop…)` + `exact-conv` are the right
primitives, and `change` captures the actual compression better
than my version: today's compute-both bridge must SPELL BOTH
spellings (that's precisely where the lines go); `change` points at
an occurrence and states only the compact replacement. The big term
is already in the goal — stop restating it. Sort-aware matching as
a gating prerequisite: agreed, and it's a soundness hardening
independently owed (the packed-Nat/Int atom finding).

## 3. Base+patch proof-facing states: endorse, with the house frame

This is the round's second-best idea and it slots into an existing
pattern: the models' eval stays the semantic authority, the patch
view is an OBSERVATION layer with a collapse theorem (patch-apply =
eval effect, proven once per model) — the same shape as std/mem's
law family and the floats NaN quotient. What convinces me it's
right: the hand-built framing families (below/above/point-grain/
wlist-grain) are exactly ad-hoc patch-composition laws discovered
one shape at a time; base+patch internalizes the whole family. And
it's the representation the heap era NEEDS anyway — heap framing
and parallel disjointness are patch-footprint statements. Building
the separation library (my review §2d) IN this vocabulary from day
one collapses two of my recommendations into one design.

## 4. Runs/RunsWithin: endorse as stated

Exact fuel becomes interpreter-internal; the proof-facing relation
composes by cost algebra; monotonicity absorbs surplus; machine
cost models refine abstract cost separately — and the same relation
is the natural home for the observation-refinement/D8 closure and,
later, cost claims for the parallel tier. Ordinary library, no
kernel rule. This is the correct end state of my mono+cost-theorem
item.

## 5. The prototype, refined

Codex's three variants and metric set are right. Two adjustments:

- Variant 3 (validator) runs on the STRAIGHT-LINE family first, per
  §1, with the block leg as its second data point only if the first
  is clean. Variants 1-2 run on the block leg as proposed (it's the
  worst case we own and they don't carry the generic-induction
  risk).
- Kernel-growth accounting per variant: variant 1 needs `change`/
  `exact-conv` (kernel, reduction-based, no new axioms); variants
  2-3 are pure library/model work. So the kernel commitment is
  gated on variant 1's numbers alone, and variants 2-3 can start
  immediately without touching canon-owned files.

Measured gates as codex lists them (DAG nodes, peak live nodes,
reduction count, RSS, incremental recheck), with the decision
question phrased codex's way: pass-constant proof, checker work
linear in unique nodes, local edits don't recheck the world.

## 6. Consequence for sequencing

This reorders my earlier recommendation in one place: the
certificate-representation track (conversion forms + patch states +
the validator pilot + DAG storage with #62) now runs BEFORE the
coverage arc's generators are written, not alongside. The
generators should emit the citation-dense dialect from birth —
teaching them the replay dialect first and migrating later is
double work we can see coming. The front half (spec→imp: ADTs,
match, calls/stack, signed kinds, address policy, heap law library)
remains the coverage frontier and its design work proceeds in
parallel on paper — but its first emitted certs wait for the
representation verdict. Christian's "months for a clean landing"
license is exactly what this spends, and the pathfinder produces
its go/no-go numbers in the first days of it.

Points of agreement I'll simply restate as joint: untrusted
transformations always; conversion = naming, DAGs = sharing,
validators = amortization; all three needed; the 200k result should
disappear from both the repository and the checker's working set.


-------------------------------------
Christian:

I agree the validator design is definitely worth exploring. One more redirect point: pressure on codex's "not universal performance point". The original concept was allowing for some performance concession in favor of formal reviewability and conceptual composability, but over time I think we can actually achieve something like "c-like performance on x86, high performance hand-written shader performance on spir-v, with virtually unlimited headroom for compute complexity". That's what excites me about this project -- we have a few major problems to solve, but even if it takes months, if the core concept makes it to the end without too many concessions we will have something that, to me, looks like a genuinely superior programming architecture to basically anything. It's been awhile since we reviewed this point, so i'll bring it up again: what have other prior-art projects done for these needs, where did they fail, and where can we learn from them? My honest reaction internally is "that's a nice concept, but nobody has succeeded in making an actual universal language without catastrophic concessions somewhere -- something *has* to break down about the concept". I have been having us push this as far as we can with the expectation that we will run into a critical limitation somewhere -- but so far (to my surprise), we haven't run into anything that looks like a genuine architectural block where we have no hope of solving it. So that's what I want both codex and fable to review next: what do we expect to be the limits of this approach, given (optimistically) that we solve the near-term issues we see here. Given the core thesis of "if we can prove refinement of a target-specific approach-specific implementation of x, then it can fit in our ecosystem", where does this break down? Are there any core tricks used in x86 programs, or compute-accelerator software that is fundamentally impractical to prove refinement on? Specifically, any tricks that are load-bearing for shard to become relevant in that domain?


================================================================
Codex — prior art and the actual universality boundary — 2026-07-18
================================================================

Direct answer: I do not see a fundamental x86 or accelerator
*implementation trick* that invalidates the core thesis. The strongest
prior art has already verified hand-scheduled SIMD assembly, code that
beats conventional compiler output, relaxed-memory concurrent code,
garbage collectors, and even JIT/self-modifying systems. The apparent
wall is not expressibility.

The wall is a conjunction of promises. I think Shard can plausibly be
universal in this sense:

> Any mathematically specified source behavior, target semantics, and
> refinement argument can coexist in one composable checked graph.

I do not think any system can be universal in this stronger sense:

> One source abstraction automatically lowers to every architecture,
> always reaches near-best performance, keeps proofs small and human
> obvious, and proves facts about undocumented physical behavior.

Shard's explicit pins, domain IRs, untrusted search, and validators are
exactly how it can give up the second claim without giving up the first.
That is not a catastrophic concession. It means the universal object is
the ecosystem/refinement protocol, while automation and performance
coverage are deliberately plural.

## 1. What the nearest prior art actually established

This is the useful comparison, compressed:

| Lineage | What succeeded | Where it paid or stopped | Lesson for Shard |
|---|---|---|---|
| [CompCert](https://compcert.org/compcert-C.html) | General C-like semantic preservation through an optimizing compiler to several real ISAs | Its official performance characterization is about 90% of GCC '-O1' on ARMv8, not parity with the strongest conventional optimization pipelines | Broad verified compilation is possible; peak optimization breadth is a very large engineering program, not a logical impossibility |
| [CakeML](https://cakeml.org/) | Eight IRs, six machine targets, bootstrapping, I/O, arbitrary precision, dynamic compilation, and a verified GC/runtime | It optimizes a substantial ML rather than serving as a peak HPC/kernel system | GC, runtime components, higher-level language features, and end-to-end bootstrapping are not architectural blockers |
| [Cogent](https://arxiv.org/abs/1601.05520) | A self-certifying compiler using language-level proofs plus per-program translation validation; functional reasoning connected to efficient C | Linear types, no required GC, and a domain suited to systems code with limited sharing | This is very close to the validator direction; strong structural restrictions buy generic automation |
| [Vale](https://www.microsoft.com/en-us/research/publication/vale-verifying-high-performance-cryptographic-assembly-code/), [Jasmin](https://arxiv.org/abs/1904.04606), and Fiat/HACL | Verified hand-tuned assembly, vector instructions, constant-time properties, and competitive or superior crypto kernels | Specialized languages, domain lemmas, SMT/proof-assistant infrastructure, and considerable expert input | Flags, carry chains, SIMD, aliasing, and instruction scheduling are all provable; narrow domains make the proof economics excellent |
| [CryptOpt](https://arxiv.org/abs/2211.10665) | Randomized untrusted assembly search, empirical benchmarking on target CPUs, and a verified equivalence checker; it reported new fastest-known field-arithmetic implementations | A deliberately modest instruction/algorithm domain | This is almost a direct existence proof for Shard's proposed optimizer architecture: search and benchmarking outside authority, equivalence validation inside |
| [Halide](https://arxiv.org/abs/2210.15740) and [Exo](https://people.csail.mit.edu/yuka/pdf/exo_pldi2022_full.pdf) | Algorithm/schedule separation and user-controlled composable rewrites yielding state-of-the-art x86 and accelerator kernels | Domain-specific computation models and restricted schedule languages; formalizing Halide's schedule soundness exposed real language flaws | Christian's explicit lowering-pin intuition is well precedented, and schedule validity is subtle enough that checked semantics adds real value |
| [Futhark](https://futhark.readthedocs.io/en/v0.25.11/performance.html) | High-performance functional GPU compilation with regular nested data parallelism and uniqueness-based in-place arrays | Irregular nested parallelism can become sequential or be refused; recursion, dynamic allocation, and function pointers are deliberately constrained on device | A single pleasant high-level GPU subset inevitably has a performance envelope; Shard should descend to a lower concurrent dialect outside it rather than pretend the subset is universal |
| [Alive2](https://web.ist.utl.pt/nuno.lopes/pubs.php?id=alive2-pldi21) | Practical translation validation for LLVM transformations | Bounded reasoning and the formidable complexity of LLVM's undefined/poison/memory semantics | Validators scale practical trust better than replay, but the semantic contract must be designed for proof rather than inherited accidentally |
| [GPUVerify](https://nchong.github.io/papers/oopsla12.pdf), [CompCertTSO](https://doi.org/10.1145/2487241.2487248), and [Iris weak-memory work](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ECOOP.2017.17) | Mechanical proofs of race/barrier properties and nontrivial relaxed-memory concurrency | Restricted fragments or sophisticated simulation/separation logics rather than simple functional equality | Weak memory is possible, but it is a separate proof ecology and should not contaminate the deterministic fast path |

Two less obvious lessons matter here.

First, prior projects usually chose one side of a triangle: broad
language and verified compiler (CompCert/CakeML), peak hand-controlled
performance (Vale/Jasmin/Exo), or highly automated domain synthesis
(Fiat/Futhark). Shard's novelty would be making those modes nodes in one
refinement graph rather than demanding that one compiler embody all
three.

Second, proof-engine asymptotics have sunk otherwise good designs.
Fiat's reflective rewriting work reports roughly a 1000x compiler-speed
improvement after redesigning the proof engine rather than the
cryptographic algorithms ([primary paper](https://arxiv.org/abs/2205.00862)).
That is close enough to the current Shard certificate discussion to be
a warning: a mathematically elegant proof rule can still be the wrong
computational representation.

## 2. x86: what is actually load-bearing

For ordinary C-like performance, none of these are fundamental proof
barriers:

- precise stack frames, calls, spills, ABI boundaries, and tail calls;
- graph-coloring or linear-scan register allocation;
- instruction selection and scheduling;
- flags, carry/borrow chains, bit tricks, conditional moves, and
  branchless code;
- SIMD, masked vectors, shuffle networks, AES/SHA/BMI instructions;
- unaligned access, overlapping input/output, in-place mutation, and
  carefully specified aliasing;
- function multiversioning and runtime dispatch on CPUID/features;
- custom allocators, arenas, reference counting, or verified GC.

Vale, Jasmin, Fiat, CryptOpt, CompCert, and CakeML collectively cover
most of that list. Shard's obstacle is implementing enough reusable
relations, solvers, and cost models—not finding a new foundation.

The difficult but still feasible x86 tier is:

- relaxed atomics and lock-free structures, especially safe memory
  reclamation and linearizability;
- signal handlers, interrupts, context switches, asynchronous
  exceptions, stack unwinding, and interaction with foreign runtimes;
- dynamic linking/loading, JITs, inline caches, and self-modifying code;
- randomized/probabilistic code and hardware entropy sources;
- constant-time, cache leakage, speculation, and other side-channel
  observations.

There is already mechanized work on self-modifying code and verified
JIT native generation, so even that is not forbidden
([certified self-modifying code](https://flint.cs.yale.edu/certikos/publications/smc.html),
[verified effectful JIT backend](https://arxiv.org/abs/2212.03129)). It
requires code memory and generated programs to become part of the
semantics instead of assuming a fixed artifact. The proof is simply no
longer the current pure-function/equality story.

The genuinely bad inputs are programs whose performance or correctness
depends on *not having a stable contract*: undefined C behavior,
intentional data races outside a specified memory model, undocumented
instructions, guessed cache behavior, kernel accidents, or silicon
errata. Shard can reproduce the code, but it cannot honestly prove its
meaning without choosing a meaning. Replacing UB with explicit
preconditions/refinements is therefore important for performance:
optimizers need facts such as no alias, alignment, non-overflow, or
unreachable cases, but those facts should be proof obligations rather
than semantic landmines.

My performance forecast for x86 is consequently fairly optimistic:

- **C-like performance across broad code:** plausible with a serious
  conventional backend, verified/validated pass library, and proven
  runtime components.
- **Best-known performance on important kernels:** plausible through
  hand/agent schedules and untrusted search plus validation; CryptOpt
  demonstrates the shape.
- **Always matching LLVM/GCC on every program automatically:** not a
  realistic success criterion. Those compilers also miss optimizations,
  and automatic optimal code generation is itself an intractable search
  problem.

## 3. Accelerators: the same answer, with one extra opaque layer

These high-performance GPU/accelerator techniques are conceptually
straightforward refinement targets once their semantics are explicit:

- workgroup/subgroup mapping, tiling, fusion, and loop reordering;
- global/shared/private layouts, coalescing, bank-aware padding, and
  staged copies;
- race-free barrier phases and double-buffered pipelines;
- vector operations, shuffle/ballot collectives, and tensor/matrix
  instructions;
- specialization constants, shape-specialized variants, and runtime
  dispatch;
- persistent buffers and explicit host/device transfer schedules.

Exo and Halide show that explicit scheduling can reach strong
performance without a universally intelligent optimizer. Shard can add
the missing end-to-end semantic validation.

The hard accelerator tier consists of irregular work queues, persistent
kernels, dynamic parallelism, warp specialization, atomics, lock-free
communication, subgroup-width-dependent algorithms, and nondeterministic
reductions. Again, none is unprovable. They require target-specific
concurrent observations, ownership transfer, progress/fairness
assumptions, and sometimes a result relation rather than equality.
Floating-point reduction deserves special emphasis: a different tree
usually changes bits. RealCake demonstrates the right broad move—relate
optimized floating computation to a flexible real/error semantics
rather than pretending fast-math preserves strict IEEE results
([RealCake](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ECOOP.2022.1)).

The extra accelerator wall is SPIR-V's position in the stack. Khronos
states explicitly that the rules for consuming a module are the
combination of SPIR-V plus the chosen client API/environment
([SPIR-V specification](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html)).
Then a proprietary driver compiles it again. A Shard theorem can prove
that a SPIR-V module has the intended allowed behavior, and benchmarks
can show that a particular driver/device produces excellent code. It
cannot prove the undisclosed final ISA or guarantee performance across
future drivers from SPIR-V semantics alone.

That produces a clean product distinction:

- **Functional artifact guarantee:** source refines valid SPIR-V under
  an explicit Vulkan/OpenCL environment contract.
- **Observed performance qualification:** benchmark records for named
  device/driver/configuration tuples.
- **Full final-code guarantee:** requires targeting and modeling the
  vendor ISA, or trusting/validating the driver compiler.

High-performance hand-written shader parity is still plausible in the
first two senses. It is not a theorem about all devices merely because
the SPIR-V schedule looks right.

## 4. Where universality really breaks

I see six hard limits. None requires abandoning Shard, but the project
should name them rather than discover them as disappointments.

### A. Refinement discovery is undecidable

Equivalence/refinement of arbitrary programs cannot be completely
automated. Loop invariants, representation relations, termination,
linearization points, numerical error bounds, and algorithmic insights
can contain arbitrarily difficult mathematics. Agents improve search;
they do not remove the boundary. Shard's answer is correct: explicit
pins and hand theorems, with automation consuming the regular regions
between them.

### B. Proof complexity is not bounded by code or runtime complexity

A trillion-iteration loop can have a five-line inductive proof, while a
twenty-line concurrent algorithm can require a research-grade invariant.
So "unlimited compute complexity" is plausible if it means runtime size
does not force unrolling. It is false if it means arbitrary algorithms
have uniformly small specifications and proofs. The mathematical novelty
of the algorithm, not its cycle count, is often the cost.

### C. Formal checkability and human reviewability diverge

A one-line citation of a huge generic validator theorem is excellent
for repeated checking, but a human cannot re-audit the theorem every
compile. Reviewability becomes architectural: small named relations,
stable interfaces, legible witnesses, and audits of reusable libraries.
Shard can be much more reviewable than 200k replay lines, but "small
certificate" should not be confused with "obvious proof."

### D. The environment eventually becomes empirical

ISA models omit microarchitecture; SPIR-V omits the driver's final code;
operating systems, firmware, devices, clocks, caches, thermal throttling,
and hardware faults sit outside pure program semantics. Functional
conformance can be modeled to a chosen boundary. Peak performance and
physical side channels require measurement and possibly a much larger,
less stable model.

### E. Performance portability has no single optimum

The best schedule depends on microarchitecture, problem sizes, data
distribution, contention, driver version, and neighboring workload.
There is no universally best lowering. The right architecture is a
family of equivalent variants plus proven dispatch conditions and
empirical autotuning. Shard's refinement graph is unusually well suited
to that, provided the build/artifact model admits variant sets rather
than insisting on one final implementation.

### F. Semantics can become the real project

Weak memory, speculation leakage, GPU convergence, distributed failure,
probability, liveness, and real-time deadlines are all formalizable, but
each needs a substantial model and proof algebra. "Machines are ordinary
libraries" keeps the kernel small; it does not make those libraries
small. At some point the cost is maintaining a federation of semantic
domains, not maintaining the language core.

## 5. The concession I would make explicit

Shard should promise **universal admission with graduated automation**:

1. A clear pure/spec tier for mathematical intent.
2. Domain IRs where common transforms are structural and validators are
   generic.
3. Explicit schedule/refinement pins for expert choices.
4. A fully general low-level/concurrent escape tier for tricks outside
   the pleasant subset.
5. Target models and artifact ties to the strongest documented boundary.
6. Empirical cost/benchmark evidence alongside, but not confused with,
   semantic proofs.

This resembles MLIR's successful recognition that heterogeneous
compilation needs several abstraction levels/dialects, not one flattened
IR ([MLIR language reference](https://mlir.llvm.org/docs/LangRef/)).
Shard's differentiator would be that the edges are refinement theorems
and that untrusted agents can introduce new nodes without acquiring
authority.

The cost is that some programs descend early. A regular image pipeline
may remain pure through scheduled parallel combinators. A lock-free GPU
queue may enter the concurrent target dialect almost immediately and
carry a difficult manual proof. A JIT may refine to a code-generating
transition system rather than a static binary theorem. They still live
in one ecosystem, but they do not all enjoy the same automation.

## 6. My updated subjective verdict

I am now less skeptical of the performance half than my first review
may have sounded. The crypto and scheduling literature demonstrates
all the necessary ingredients separately:

- high-level specifications;
- explicit expert schedules;
- untrusted stochastic search and empirical tuning;
- verified equivalence checkers;
- real SIMD/assembly output matching or beating conventional code.

Shard's proposed combination is credible. I see no theorem saying it
must collapse before C-like x86 or strong SPIR-V kernel performance.

The likely failure mode is organizational complexity, not a semantic
dead end: too many target models, validator theorems, cost models,
runtime libraries, and domain-specific proof algebras for a small team
to keep polished. The defense is exactly the architecture now emerging:
family-level validators, shared patch/resource algebra, compact
conversion, content-addressed proofs, and a strict refusal to make the
kernel absorb domain complexity.

So the honest bet is stronger than "formal correctness with a modest
performance concession," but slightly weaker than a universal optimizing
language: **a universal refinement environment in which specialists and
untrusted optimizers can express essentially any documented performance
technique, prove it once at the right abstraction boundary, and reuse it
without trusting the tool that chose it.** Prior art says every piece is
possible. What remains unproven—and genuinely novel—is whether Shard can
make all the pieces compose economically.

================================================================
Claude (Fable 5) — the limits question, from inside the machine —
2026-07-18
================================================================

Codex's prior-art survey is accurate and I won't rehash it. My turn
is the complement: check each claimed limit against shard's SPECIFIC
commitments (equational kernel, first-order source, fuel semantics,
composition by citation), answer Christian's question directly, and
name the one variable the prior-art comparison cannot price.

## 1. The direct answer: the load-bearing-trick inventory

"Is there a core trick, load-bearing for relevance, that is
fundamentally impractical to prove refinement on?" Going down the
list of what actually makes x86 and accelerator code fast, checked
against what shard's kernel can state today:

- SIMD, shuffle networks, branchless tricks, carry chains, ISA
  extension instructions: equational, deterministic, precedented
  (Vale/Jasmin/CryptOpt). Fits the existing claim form unchanged.
- Register allocation, scheduling, spills, calling conventions:
  deterministic rewrites of deterministic programs — validator
  territory. No new kernel semantics.
- Arenas, custom allocators, RC, in-place mutation: this is
  MEMORY.md's own spine; the collapse-proof trick (pre/post states
  as pure objects) is shard's founding move, not an accommodation.
- Runtime dispatch / multiversioning (CPUID, subgroup width):
  fits the value-parametric discipline unusually well — width and
  feature flags are already values consumed at proof time; variant
  sets behind one mod.req are already BUILD.md's variant-selection
  design. Dispatch conditions become premises; each variant carries
  its own cert. Designed-for, not merely possible.
- Tiling, staged copies, barrier pipelines, collectives: the
  structured-parallel IR + footprint disjointness story from
  earlier rounds. Hard work, no impossibility.
- Constant-time (crypto flagship-relevant): NOT a functional
  refinement — it's a two-run hyperproperty. But it is stateable
  first-order over a leakage-INSTRUMENTED model variant (the model
  additionally emits the address/branch observation trace;
  constant-time = trace equality across secret variation). Jasmin
  proves exactly this shape. Cost: one instrumented sibling per
  model, a known multiplier on the federation cost, no new logic.

The one place the equality fast path definitively ENDS inside a
load-bearing technique: **nondeterministic-order floating-point
accumulation** — atomic float adds / scatter-add, pervasive in ML
backward passes. FP non-associativity + unordered atomics = the
RESULT is bit-nondeterministic; there is no equation to prove. The
honest menu: (a) spec as an allowed-result set or error bound
(RealCake direction; the floats arc's NaN quotient is our landed
precedent for observation quotients), or (b) restrict to
deterministic tree reductions — which costs some throughput on some
workloads but is the direction the ML industry itself is moving for
reproducibility. Shard should pick (b) as default with (a) as the
escape, and say so in PARALLEL.md from day one. This is a boundary
to DECLARE, not a wall we hit.

Below that: speculation/microarchitectural side channels and
undocumented-contract behavior — codex's limit D, agreed, boundary
honesty. Nothing in this inventory threatens the x86-batch or
GPU-kernel relevance thesis.

## 2. Two shard-specific limits codex's list understates

**(i) The proof-style monoculture has a liveness ceiling.** Every
shard proof is equation + induction; fuel makes all temporal
properties bounded ("within f steps, X"). Safety and
bounded-response cover batch bins and compute kernels — the entire
near-term thesis — but UNCONDITIONAL liveness of a reactive system
(a server that always eventually responds, fairness under
scheduling) has no home without coinduction or step-indexing,
which would be genuine kernel growth of the high-bar kind. I flag
it as the one place I can point at where "the kernel never learns
X" might eventually bend. It is NOT load-bearing for the
performance thesis; it becomes load-bearing if shard's ambitions
extend to long-running services as proven artifacts.

**(ii) Runtime code generation inverts generate-and-check.** Our
whole trust economics assumes the checker runs at build time. A
JIT emits code when no checker is present. Two honest options:
verify the generator itself (the one place the verified-compiler
architecture is forced), or — the elegant consequence of this
week's convergence — ship the VALIDATOR in the artifact: valid_P
is an ordinary total shard function, so a proven-correct-once
validator can run at runtime, gating each emitted fragment before
execution, with the reject path a D8-style controlled failure.
Generate-and-check becomes a runtime discipline. Far future, but
it means even self-modifying-code territory has a shard-shaped
story that needs no new trust primitive.

## 3. The variable prior art cannot price

Every project in codex's table paid its cost in EXPERT-HUMAN proof
labor, and that is what killed breadth every time: CompCert's
backend breadth, Fiat's engine rewrite, Vale's per-kernel expert
input. Shard's wager — visible in this repo's own history — is
that the labor economics changed. The sha chain's 34k hand lines
were authored in days of agent time, not months of researcher
time; the RISC-V third target was a one-week transplant; the
language is deliberately DESIGNED for machine authorship (corpus
alignment, guessable names, error hardening). The federation cost
codex correctly fears — many models, validators, cost algebras,
instrumented siblings — is a cost in exactly the currency whose
price collapsed. What did NOT collapse: design-decision bandwidth
(Christian's), and checker compute (addressable engineering —
evaluator promotion, #62, the DAG work). So my forecast for limit
F is more optimistic than codex's phrasing: the federation is
affordable HERE specifically, provided the design-decision
bottleneck is respected — which is what this discussion's
ledger-first culture already does.

The second under-priced asset: COMPOSITION UNIFORMITY. Prior art's
triangle (broad compiler / peak hand kernels / domain synthesis)
was never unified because no project had ONE claim form all three
modes discharge. Vale artifacts don't weld into CakeML programs.
Shard's LOWERING.md schema + module system is precisely that weld
layer — hand-tuned, default-lowered, and searched artifacts are
interchangeable BY STATEMENT SHAPE. That is the actual novel bet,
and it is why "universal refinement environment" (codex's phrase,
correct) is more than a federation of existing results.

## 4. Sharpenings on the accelerator trust wall

Codex's SPIR-V three-tier product distinction is right; one
strategic addition: the driver wall is VENDOR-SHAPED, not uniform.
AMD's stack is open source with a publicly documented ISA (RDNA);
Intel's compiler is open; Tenstorrent — already on our radar via
the RISC-V arc's service cores — is open-stack with documented
cores. On those stacks the "full final-code guarantee" tier is
reachable TODAY by modeling the vendor ISA and differentially
gating the driver, the exact V8-versus-wasm-model pattern at the
next layer down. NVIDIA is the closed corner and gets tiers 1-2
only. Consequence: pick shard's first accelerator targets by
DOCUMENTATION OPENNESS, not market share — the trust story is a
product feature there, and the differential harness is a
transplant of machinery we already run.

## 5. On "something has to break"

Christian's prior is sound — every "universal language" broke
somewhere. Reading this discussion's full ledger, the concessions
shard has made so far are real but share one property: each is
EXPLICIT and CHOSEN. First-order source (defunctionalize; the
static-lowering principle). Spelling canon as the abstraction
mechanism (authoring discipline instead of interface indirection).
Fuel-bounded temporal claims (no unconditional liveness). Trust to
the strongest DOCUMENTED boundary (no microarchitectural claims).
Determinism or a declared result relation (no bit-ambiguous
outcomes). Graduated automation (some programs descend early —
codex's §5). The catastrophic failures in prior art were mostly
HIDDEN concessions — semantics inherited by accident (LLVM's
UB/poison swamp is codex's own example), performance ceilings
discovered after commitment, proof engines that didn't scale and
were load-bearing. Shard's method — ledgers, priced walls, named
fences, corpus pins — is structurally a machine for refusing
hidden concessions. That doesn't guarantee no wall exists; it
guarantees a wall will be found early, named, and priced, which
is the strongest position a project can be in relative to its own
unknowns.

To keep the performance half falsifiable rather than
inspirational, I'd name the next perf milestone now: a STREAMING
sha256sum (the named read-until-EOF rung) benchmarked against
coreutils on this box, then a SIMD rung against openssl's. First
numbers, however humbling, convert the C-parity claim from thesis
to trendline — and the honest current state is that we have zero
performance evidence either way; everything so far was
correctness-shaped by design.

-----
Christian

Good points on both sides, and I will agree that floats are currently an under-developed component that will prove critical. One of the major roles I have in mind for shard is as a core component of a future machine learning project. We built one before (~/workspace/whisper-tensor) and the meta-observations while building it were one of the strongest input motivations to building shard. I had developed an impression of "why am I building this again -- this shape seems like it should be something we should build once well, but why has nobody done it?" in particular with the iterated IR necessary to efficiently take a representation of a SOTA-scale language model, transform, optimize, and plan evaluation of it, then make it happen on a wide range of target ISAs. Shard will need to be able to deal with imprecision in a given compute platform's floating point model, and I am not certain how to approach that problem yet. One upside though that both Fable and Codex missed (it's a blind spot in virtually all language models) -- I want both of you to take a look at the actual timestamps in the git history. Notice how much we have built in how fast, and let me know if that changes either of your estimates or outlooks. We will be wrapping up here shortly and getting back to development, so I want to also hear any final remarks or pressures on what we should do in the next few arcs to prove out or rework the most fundamental things we will need to change.

================================================================
Codex — velocity audit, ML numerics, and closing recommendations —
2026-07-18
================================================================

I did the timestamp audit, including the earlier whisper-tensor tree.
It changes my forecast materially.

## 1. The timestamps are not a small correction

The first commit in this Shard history is 2026-05-28 01:19. The
current tip I inspected is 2026-07-18 10:41:

- 975 commits in about 51 days;
- commits on 45 distinct dates;
- 696 files changed, 613,463 insertions, 1,850 deletions from the
  first commit to tip;
- 573,575 tracked lines of Shard today;
- 287 commits in ISO week 28 alone.

Generated certificates inflate the line total, so I do not treat
613k insertions as 613k lines of independent design. The milestone
timestamps cannot be explained away that way:

- wasm model began July 2;
- x86 model began July 6;
- pure SHA began July 10;
- IMP.md began July 11 at 23:48;
- the typed IMP model landed July 12 at 00:35;
- the SHA IMP sibling began July 12 at 10:24;
- the SHA binary ran on silicon July 18 at 08:45;
- the PIO model went from its first model commit to merge in roughly
  34 hours;
- the RISC-V model went from first groundwork to merged third-target
  groundwork in roughly 19 hours.

The float history is just as striking. FLOATS.md was drafted late
July 11. std/rat went from its first implementation slice to R0
complete in about ten hours. The parametric float model started the
next morning, and most of the R1/R2 construction landed over the next
day or two.

I also found whisper-tensor under
whisper-tensor/whisper-tensor.main. It has 971 commits beginning
2025-04-04. Its current architecture is exactly the motivating shape
Christian described: ONNX/symbolic graph -> milli-op graph -> several
backends, with a correctness reference, Vulkan execution, broad dtype
handling, 142 ONNX ops, and more than 2,000 backend conformance tests.
Shard has accumulated roughly the same commit count in seven weeks
that whisper-tensor accumulated across its longer history.

I therefore revise two judgments:

1. I reduce my estimate of implementation/federation labor by a large
   factor. Fable is right that agent-authored model, theorem, and
   transplant work is being purchased in a different currency than
   the prior-art projects paid.
2. I increase my estimate of architecture-propagation risk. A weak
   abstraction can now acquire three targets and hundreds of fixtures
   before a human could traditionally have finished reviewing its
   first implementation.

The scarce resources are not typing code or even producing first-pass
proofs. They are Christian's design-decision bandwidth, checker CPU and
memory, specification adequacy, and the quality of the gates that tell
the team when fast output is wrong. The ledger/ratification culture is
not process overhead in this environment; it is the flow-control
mechanism that keeps extraordinary implementation velocity from
amplifying a mistaken premise.

So yes: the timestamps make me substantially more optimistic that a
broad semantic federation is affordable here. They do not make
normalization asymptotics, undocumented hardware, or hard mathematical
invariants cheaper. They make it more urgent to settle those structural
issues before pointing the production machinery at a new dialect.

## 2. ML changes the float requirement

FLOATS.md is much further along than “under-developed” suggests if the
question is exact IEEE-style arithmetic. It already has:

- an exact rational specification domain;
- a parametric format descriptor;
- exact deterministic rounding semantics;
- F32/F64 surfaces and BF16/F16/F8 descriptors;
- a bit-level GRS implementation;
- NaN observation quotienting;
- a planned BF16-input/F32-accumulate GEMM flagship.

That is an excellent foundation. It is deliberately exact, bans
implicit contraction and fast-math, and currently says that target
differences beyond the NaN quotient are fixed up by lowering. That
stance is right for the **exact float core** and wrong as the only
numeric refinement interface for ML.

High-performance ML routinely depends on value-changing choices:

- BF16/F16/F8 storage with F32 or narrower accumulation;
- FMA contraction and tensor-core tile operations;
- different reduction trees;
- flush-to-zero or reduced intermediate precision;
- reciprocal, rsqrt, exp, tanh, and other approximate instructions;
- quantization/dequantization and saturation;
- stochastic rounding;
- atomic scatter-add and other nondeterministic accumulation.

Trying to call all of those equal to one deterministic float program
would either reject important hardware or quietly weaken “equality.”
The clean answer is to keep std/float exact and add a separate
policy-indexed numerical refinement layer.

## 3. A four-level numeric contract

I would give ML computations four distinct contracts.

### Level A — exact machine arithmetic

This is today's std/float and per-ISA model:

- exact format bits;
- exact operation order;
- exact rounding;
- explicit NaN quotient only where observations intentionally erase
  payload/sign;
- target theorem or differential at the documented boundary.

It answers: “What bit-level computation did this kernel perform?”

### Level B — an allowed numeric policy

Define an ordinary Shard value describing the implementation envelope,
roughly:

- storage format;
- multiply/input format;
- accumulator format;
- rounding mode;
- subnormal/FTZ behavior;
- whether FMA contraction is allowed;
- reduction-order class: fixed tree, any association, or atomic order;
- per-primitive approximation law for reciprocal/exp/etc.;
- exceptional-value policy.

Then define a relation such as:

    allowed(policy, op_or_graph, inputs, output)

Exact deterministic execution is the singleton special case. A
deterministic tree reduction has one permitted result. An unordered
atomic reduction admits a set. A target kernel proves that its result is
a member of the policy's allowed set.

This is where the observation-refinement generalization becomes
load-bearing: the implementation's outcomes must be a subset of the
specification's permitted outcomes. No kernel primitive is needed; the
policy, result relation, and composition theorems are ordinary Shard
libraries.

### Level C — numerical error against mathematical intent

Relate the allowed float results to an ideal Rat/real/tensor expression
under a named metric and budget:

    approximates(metric, budget, ideal, concrete)

There should not be one universal epsilon algebra. Useful contracts
include:

- absolute/relative/ULP bounds;
- interval enclosures;
- vector norms;
- componentwise mixed bounds;
- argmax preservation under a proven margin;
- probability/distribution distance for stochastic operations.

Libraries prove reusable propagation rules. A schedule or precision
choice consumes a budget. An optimizer may reassociate, choose a tensor
instruction, or reduce precision only when the validator can show the
selected policy stays within the requirement's budget.

The existing future “Gappa-style interval automation” is one engine for
this level, but naive whole-network intervals will usually explode into
useless bounds. ML needs block-level stability lemmas, norm bounds,
range refinements, and task-aware observations rather than multiplying
worst cases through billions of scalar operations.

### Level D — task adequacy

“This implementation stays within a local numerical envelope” is not
the same theorem as “the language model remains useful.” At model level,
the meaningful requirement may be:

- same top-1 token while the logit margin exceeds a bound;
- KL/logit distance below a threshold;
- image/audio metric within a bound;
- accuracy on a named validation corpus;
- training convergence under statistical assumptions.

Some of these can be formal conditional theorems. Dataset accuracy and
real deployment quality will often remain empirical gates. Shard should
record that evidence beside the formal artifact without laundering it
into a proof.

This decomposition lets Shard make a stronger claim than most ML
stacks: exact kernel semantics, proven membership in a declared
imprecision policy, and a compositional numerical bound where one is
useful—while still being honest about empirical model quality.

## 4. Specific pressures from whisper-tensor

The earlier project suggests several ML IR facts should be first-class
from the beginning:

- Tensor **storage dtype** and **accumulator dtype** are different
  fields, not one type tag.
- Layout, tiling, padding, quantization scale/zero point, and address
  space are refinement data.
- A model graph, a primitive tensor graph, a scheduled tensor graph,
  a parallel kernel graph, and an ISA module are different dialects.
  Their commonality is the refinement edge, not a shared node enum.
- Shape-symbolic validation must not require expanding model weights or
  unrolling dimensions.
- Variant sets and dispatch are normal: shape, dtype, device feature,
  subgroup width, and workspace budget choose among proven kernels.
- Reference fallback is useful during coverage growth, but the artifact
  theorem must make each fallback boundary and its cost visible.
- Training is a separate later semantic tier. Inference should be the
  first flagship; gradients, optimizer state, atomic updates, and
  stochasticity combine every hard issue at once.

The first numeric consumer should remain small: BF16 inputs with F32
accumulation in a dot product or small GEMM. Give it two schedules—a
fixed deterministic tree and a reassociated/tiled implementation—and
prove both against the appropriate policy. Follow with softmax or
LayerNorm, which forces reduction plus an approximate transcendental.
That pair will reveal much more about the eventual ML architecture than
adding many isolated float operations.

## 5. The next arcs I would actually run

I would treat the next phase as two orthogonal flagships joined by one
foundation track.

### Arc A — certificate architecture, before new generators

Run the agreed pathfinders:

1. generic validator induction on the smallest straight-line IMP family;
2. block leg only after that theorem is clean;
3. explicit change/exact-conv rather than implicit conversion search;
4. hash-consed/content-addressed terms and conversion memoization;
5. base+patch proof-facing states;
6. Runs/RunsWithin as the proof-facing cost/fuel interface.

The gate is not only source shrinkage. Require pass-constant proof
structure, checker work linear in unique source+target nodes, bounded
peak live terms, and local-edit incremental behavior. If this track
fails, stop and redesign before the coverage compiler or GPU proof
generator learns the old dialect.

### Arc B — streaming SHA as the x86 truth serum

Turn the current demonstrator into a conventional program:

- relocate away from page zero;
- read until EOF and handle short reads;
- remove the 64 KiB application cap;
- maintain incremental SHA state;
- benchmark the scalar version against coreutils;
- add a SHA-extension/SIMD hand-pinned variant and compare against
  OpenSSL;
- prove CPUID/feature dispatch selects only certified variants.

This tests application completeness, World/I/O composition, compact
certificates, target-specific expert lowering, and the performance
claim in one existing domain. It is a much better next SHA milestone
than adding another proof-only rung.

### Arc C — ML numeric semantics before broad parallel syntax

Amend the float roadmap without weakening std/float:

1. ratify the policy/allowed-result/approximation layering;
2. implement the BF16-in/F32-accumulate dot flagship;
3. fixed tree first, reassociation/error-bound second;
4. make storage, compute, and accumulator formats explicit;
5. add one target-approximate primitive only when softmax/LayerNorm
   demands it;
6. measure both numerical envelope and runtime.

This is the smallest experiment that tells us whether Shard's
equational core can host approximate refinement cleanly through ordinary
relations.

### Arc D — parallel pilot reuses Arc C

Lower the same dot/small-GEMM declaration through the structured
parallel IR:

- disjoint map first;
- tiled workgroup reduction with barriers second;
- deterministic reduction default;
- allowed-result relation for nondeterministic order only as an explicit
  alternate policy;
- first target chosen for open documentation/toolchain, as Fable
  recommends;
- compare with a hand-written shader on the same device.

Using one numeric flagship across scalar x86, structured parallel, and
SPIR-V prevents three separate demo semantics from drifting apart.

### Coverage arc — design now, emit after Arc A

Calls/stack, signed kinds, address policy, heap patch/framing algebra,
and the cons/match/free micro-flagship remain the right spec-to-IMP
frontier. Continue their design while Arc A is measured. Do not let the
coverage generator emit production certificates until the new
certificate dialect and patch algebra are decided.

## 6. What would change my mind

The project should now choose falsification gates, because raw feature
velocity will otherwise make almost any direction look successful.

I would materially downgrade the architecture if any of these occur:

- the generic straight-line validator theorem is itself heroic or
  checks superlinearly;
- base+patch cannot avoid materializing full state at most composition
  seams;
- the streaming scalar SHA artifact is dramatically slower for reasons
  inherent to the proof-facing IR rather than an immature backend;
- an expert SHA-NI or tiled-GEMM schedule cannot be expressed without
  target semantics leaking back into the high-level requirement;
- approximate float refinement requires kernel changes rather than
  ordinary result relations and error algebras;
- SPIR-V validation/environment conditions cannot be modularized as a
  target package;
- a local edit still invalidates/rechecks most of a large artifact after
  content addressing.

Passing those gates would be far stronger evidence than another hundred
thousand generated lines.

## 7. Closing outlook

The history changes my subjective probability, not just my schedule
estimate. Before inspecting it, “a small team maintains a federation of
verified models and validators” looked like the most likely practical
failure. The repo shows that this team can build and transplant semantic
systems at a rate prior-art staffing assumptions do not predict. The
federation now looks plausible.

The remaining danger is that implementation velocity masks a bad
semantic seam until it is everywhere. The right response is not to slow
ordinary construction; it is to spend deliberate time on the few
interfaces that multiply across all later work:

- observational refinement;
- validator/certificate representation;
- patch/resource composition;
- proof-facing cost semantics;
- numeric approximation policy;
- target/environment boundary.

If those six interfaces land cleanly, I am comfortable with a much
stronger forecast: C-class x86 performance, competitive expert-scheduled
accelerator kernels, and very large computations are all credible
outcomes rather than aspirational slogans. The residual universal
concession is not performance. It is that automation depth varies by
domain, and the hardest algorithms still require hard mathematics.

My final pressure is therefore to resist two tempting celebrations:
do not answer the SHA milestone by immediately broadening coverage, and
do not answer the ML ambition by immediately building a large tensor
compiler. First make SHA compact and fast; then make one mixed-precision
dot product numerically honest and parallel. Those two flagships will
price almost every foundational claim we discussed.

================================================================
Claude (Fable 5) — the timeline audit + closing pressures —
2026-07-18
================================================================

Per Christian's instruction I ran my git survey BEFORE reading his
request or codex's reply, so the observations below are
independent; where they converge with codex's audit, that's two
instruments agreeing.

## 1. What my audit found (and what codex's didn't)

The shared facts: 975 commits, 52 days, five clean eras
(bootstrap slices + snake → kernel deepening → formers/totality →
ISA era → imp + fork era), essentially zero legacy accumulation.
Three observations I'll add to codex's numbers:

- **Churn ratio as a health instrument.** Per-era
  insertions/deletions: the bootstrap era deleted HALF of what it
  wrote (56k ins / 27k del); later eras run 10-20% deletion with
  named retire commits in every single era (Induct2, snake_game_2,
  the script era, the model-local Peano, ix_rot, the recognizer
  tiers, pres/pmem…). The delete-first discipline isn't aspiration
  — it's measured, continuous, and it means the repo has NEVER
  carried a legacy layer for even a week.
- **The rate-limiter has migrated three times, and the git data
  alone says where it is now.** June's limiter was kernel
  capability (quiet eras align with kernel arcs). Early July's was
  proof idioms (the x86 era's grind). Mid-July's is orchestration:
  13 of the repo's 14 merges landed in the last 9 days, and the
  quiet days sit immediately after ratification bursts (Jul 13
  after the Jul 12 redirection day) — throughput visibly gates on
  decision bandwidth, not generation. And the NEXT limiter is
  already visible in the raw insertions: the last era added 500k
  lines, and **45% of the tracked repo is now generated
  certificates** (sha256's three cert files are 244k of std's
  312k). I found this before reading a word of this round — the
  git history independently nominates the certificate
  representation as the binding constraint, which is exactly what
  the discussion had already concluded from the inside.
- **Velocity cuts BOTH ways, and that refines codex's propagation
  risk.** Codex worries a weak abstraction can acquire three
  targets before traditional review would finish. True — but the
  log also shows the counter-mechanism operating at the same
  speed: the v2 scalar migration deleted the whole v1 consumer
  tree in ~2 days; the recognizer tiers, a substantial system,
  were retired without ceremony. Cheap construction implies cheap
  RETRACTION — for code. What is NOT cheap to retract is a
  ratified ruling or a canon rule, because those consume the one
  currency that didn't inflate (Christian's bandwidth) twice —
  once to make, once to unmake. So I'd state the flow-control
  principle more precisely than "gate the velocity": **keep the
  rulings queue short and explicit, and let code move at machine
  speed behind it.** Which is, to codex's point, exactly what the
  ledger culture already implements — the timestamps prove it
  works rather than suggesting we need it.

Does the audit change my outlook? My labor-economics argument last
round was made before measuring, so for me this is confirmation
with numbers rather than revision — but two things do update: my
"months of iterating the design" language compresses to
days-per-pathfinder with ratification as the scheduler; and the
six-interface program codex lists stops looking like a quarter of
work and starts looking like a few weeks, serialized through
design sessions rather than construction.

## 2. ML numerics: the four-level contract, grounded in what's landed

Codex's A/B/C/D decomposition is right and I endorse it as the
PARALLEL.md-adjacent numeric ledger's skeleton. Three groundings
from the house side:

- **The approach Christian says he's unsure of already has a
  landed precedent at small scale.** Platform FP divergence was
  hit once before — NaN payloads — and the resolution (FLOATS.md
  §5's observation quotient at the model boundary) is Level B in
  miniature: machine semantics stay EXACT, the quotient lives in
  the relation. Generalizing quotient → policy-indexed allowed-set
  relation is design work, not a new kind of thing.
- **One rule I'd pin now: ISA/accelerator models stay bit-exact
  per platform, always.** Imprecision lives in RELATIONS between
  spec and target, never inside machine semantics — a "fuzzy"
  model would destroy the differential-gate methodology (you
  can't diff a vague model against silicon). For instructions
  vendors specify only by error bound (rsqrt et al.), the model's
  denotation is honestly bounded-nondeterministic — result-in-
  interval — and the differential checks membership. Level A
  stays the ground truth everywhere.
- **whisper-tensor is not just the motivation — it's an importable
  asset.** It carries a reference-correctness implementation, 142
  ONNX ops with real usage frequencies, and 2,000+ backend
  conformance tests. That's a ready-made external differential
  oracle and a frequency-ordered coverage checklist for shard's ML
  flagships — the V8 pattern at tensor grain. Arc C's dot/GEMM
  flagship should differentially gate against wt's reference from
  day one.

Also worth answering Christian's "why has nobody built this once,
well": each layer of the iterated-IR tower historically belonged
to a different expert community (compiler, numerics, formal
methods, GPU), and the composition tax ACROSS communities was
fatal — nobody could afford all the experts at once. The
refinement-edge architecture plus agent authorship is the first
configuration where one small team can hold the whole tower.
That's the same labor-economics argument, but it's specifically an
answer to why the gap Christian noticed exists.

## 3. Closing pressures (convergent with codex's arcs, three deltas)

I co-sign Arc A → B/C → D and the falsification-gate list — it is
the discussion's synthesis and I have no structural quarrel. My
deltas:

1. **Arc B is secretly also the D8 closer — schedule it as such.**
   Streaming = the World-loop species + short reads + EOF = the
   observation-relation and except-grammar work with a real
   consumer forcing every decision. Plus #65 and the generic
   bin-tail (window policy, entry contract, trace shapes) land
   inside it naturally. Frame Arc B's deliverable as "the bin
   story generalized and made fast," not "sha again."
2. **Sequencing discipline between A and B/C:** B and C should
   CONSUME Arc A's ratified forms, not run on the replay dialect
   (another 200k-line artifact pile would be the one genuinely
   embarrassing outcome of this review). Their design work — 
   streaming World shape, numeric policy ledger — runs on paper in
   parallel, the measured house pattern (bursts behind
   ratification). Arc A's pathfinders are days-scale; nothing
   starves.
3. **Every one of the six interfaces gets the same treatment:**
   a ledger section, a days-scale measured pathfinder, and a
   falsification gate — and their ratifications serialize through
   Christian one at a time. That's not process for its own sake;
   per §1, rulings are the only expensive artifact left, so the
   design queue IS the schedule.

Final remark. This discussion produced real direction changes —
the validator architecture, explicit conversion forms, base+patch
states, the Runs interface, the numeric contract levels, the arc
ordering. Those currently live only in this untracked file. The
first act of "getting back to development" should be distilling
them into the ledgers they amend (IMP.md, FLOATS.md, MEMORY.md, a
new certificate-architecture ledger, the PARALLEL.md skeleton) as
ratified-direction blocks — so the next session's agent reads them
as law rather than as correspondence. I'm glad to draft those
amendments for review when we wrap here.
