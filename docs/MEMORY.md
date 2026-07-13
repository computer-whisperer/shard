shard memory — MEMORY.md
========================

STATUS: DRAFT (2026-07-11; application-story revision same day — the
tower now lands at models/imp, see the application ruling in the
rulings block and docs/IMP.md) — the scope ledger for object
representation and memory management under lowering: how shard values
become bytes, and when bytes that represented one value may be reused
for another.
This file takes ownership of LOWERING.md §7's uniform-rep questions
(items 2, 5, 6) and states the heap/stack lifecycle story for every
native target — the story must survive compiling ANY shard program,
not just the eval.shard flagship (USER RULING 2026-07-10).

User rulings already on record from the design discussions of
2026-07-10/11 (do not relitigate silently):

- **Recovery is mandatory from v1.** Bump-only allocation is a
  non-starter — the temporary C compile path began bump-only and basic
  shard problems became intractable; it had to grow a collector.
- **The honest-caveat framing.** shard is a purely theoretical
  language (unbounded ints, free unchecked heap). Hardware constraints
  enter through models with 1:1 high/low behavior; lowering attaches
  honest caveats ("under 2^64", "as long as live data fits"). Resource
  exhaustion is visible and explicitly handled — never a global
  "unless OOM" acceptance clause.
- **Owned mutation is required.** Entire algorithm classes need it for
  C/Rust parity, and representation change into internally-mutable
  linear-memory models (tracking pre/post states as separate pure
  objects, with a collapse proof onto real mutation) is one of shard's
  core tricks, not an exception to accommodate.
- **The user's lean:** the Rust arc-pointer family over the tracing-GC
  family — held loosely; the governing criterion is the cleanest
  formulation. §1–§3's analysis converges on precise counting;
  ratification of this file decides it.
- **The embedded bar.** Heap allocation is completely optional for
  many programs (the embedded-Rust lesson); well-controlled stack
  management is often all you need, and shard must meet that bar
  where programs admit it.
- **The safety inversion** (first read-through, 2026-07-11): the
  trust story — every shipped binary is a proven refinement of the
  shard application with an explicit list of hardware-necessitated
  premises — means memory design here is never "where do we put the
  data so it's safe"; safety crossed the bar with the refinement.
  The ledger's real question is "how efficient a memory layout can
  we build and still land the proof that closes the compile" (§1).
  Relatedly, the kernel is NOT immutable — the bar for core
  provisions is high but usefulness has bought core syntax before
  (§10).
- Standing context: the C/Rust performance gap is a to-be-closed
  success criterion, never an accepted constant; mimicry is the named
  hazard — most plausible-sounding memory designs import an industry
  groove instead of deriving from what shard actually needs.
- **The application ruling (2026-07-11): the tower lands at
  models/imp.** A neutral imperative dialect (docs/IMP.md — the
  ledger for the machine itself) keeps the memory-allocation story
  minus the ISA-specific quirks and is the natural manual spelling
  target before the full `.wasm.shard`/`.x86.shard` dialects. This
  ledger's theorems are stated ONCE against imp (spec ⊑ imp is
  where cancellations attach); the per-ISA legs (imp ⊑ wasm/x86)
  are memory-story-free instruction selection. A memory class IS a
  choice of imp spelling, and the class-assignment surface (D1)
  steers the spec → imp step from the build profile. The float
  arc's core model joins imp by citation at fork-merge time
  (IMP.md rulings block).
- **The coverage inversion (2026-07-12, re-adjudication ruling).**
  Rung 4 — the counted heap plus the GENERIC spec → imp translator
  (the uniform-representation compiler) — is pulled forward as the
  COVERAGE SPINE: it is what closes "basic function across the
  entire language," and every other class degrades INTO it (§1's
  graceful-degradation argument applied to the schedule). Rungs
  2/3/5 become per-decl performance upgrades layered on the total
  story; the frame flagship (rung 2, sha256) completes but no longer
  gates coverage. Cert families are GENERATED from day one — the
  sha256 sibling (11k hand-proof lines for one module) is the
  measurement that hand certs do not scale past the flagship. The
  lowering-induced-failure surface is D8, opened the same day.


## 1. The root problem

A pure shard computation is term rewriting over mathematical values;
the machine is a finite byte array. Intermediate values come into and
go out of existence, so byte regions must be reused across the
computation's lifetime. The single obligation the lowering must
discharge is **reuse safety**: bytes that represented one value may be
reused for another only if no still-needed value is corrupted. "Still
needed" is liveness — and every memory-management mechanism industry
ever built is a strategy for locating the knowledge of liveness.
Classify by where that knowledge lives (static proof vs runtime
tracking) and at what granularity (per-cell vs bulk):

|  | per-cell | bulk |
|---|---|---|
| **static** | proven uniqueness → exact free/reuse points compiled in (Rust ownership, derived) | stack extents; regions-by-proof; whole-program preallocation |
| **dynamic** | reference counting | tracing GC |

Stack and heap are not primitives of the problem; they are two entries
in this table. The map makes the anti-mimic argument visible: tracing
GC is the dynamic+bulk corner — the "refuse to know anything" strategy
a language adopts when it cannot know liveness statically and will not
track it locally. shard can always know more: proofs are its native
asset. **The dynamic+bulk quadrant stays empty** (§10). The design
gradient everywhere else: push liveness knowledge as static as the
program's proofs allow; where it must be dynamic, keep it local and
deterministic.

**The safety inversion (USER RULING, 2026-07-11).** In every
mainstream language the memory design IS the safety mechanism — where
the data lives decides whether the program is sound, so the design
space is policed by fear, and Rust ships a deliberately conservative
borrow checker to police it mechanically, because it must run without
proofs. shard's trust story inverts this. An x86 binary ships only as
a proven refinement of the shard application, with an explicit premise
list naming exactly what the hardware made necessary; safety is
enforced by THAT bar, not by the memory manager — anything that
provably refines down has already crossed it, and we never build,
tune, or debug a borrow checker. So the question this ledger optimizes
is not "where do we put the data so it's safe" but **"how efficient a
memory layout can we build and still land the proof that closes the
compile."** Three consequences run through everything below:

- Every mechanism in this file competes on exactly two axes —
  layout efficiency and proof-landability. Safety is never a
  tiebreaker; the open decision points (§11) are decided on those
  two axes alone.
- The tower degrades gracefully: failing to prove a cancellation
  theorem never costs soundness — copy instead of mutating in place,
  count instead of cancelling, box instead of framing — it only
  costs speed. Uniqueness and borrow facts are optimization licenses,
  not safety obligations.
- Layout experimentation is safe by construction. An aggressive rep
  cannot ship a hazard; it can only fail to close the compile, at
  which point you retreat one rung. There is no `unsafe` escape hatch
  to audit, because there is no checker to escape. This is the
  biggest novelty the arc gets to capitalize on.


## 2. What shard brings that cancels things

Five properties, each of which deletes machinery that other languages
are forced to carry:

1. **Proofs instead of inference.** Exact per-decl theorems — size
   bounds, uniqueness, non-retention — replace conservative global
   analyses. Rust bakes a fixed borrow checker into the language
   because it cannot prove per-program facts; MLKit's region inference
   was famously brittle for the same reason. shard states the fact and
   proves it. The entire linear-types language apparatus cancels into
   ordinary theorems about lowered forms (the refinement-lowering
   position: ZERO kernel features).
2. **Sharing is observationally invisible.** Purity makes copy-vs-share
   a free per-site choice — any mix is sound; only cost differs. No
   aliasing analysis is needed for correctness, only for pricing.
3. **Certs are denotational at boundaries.** Values cross call
   boundaries as spec terms; addresses are extent-internal plumbing.
   Frees and representation games between or within extents cannot
   disturb composition — the frame-condition machinery already in the
   cert shape carries readback stability for everything not owned.
4. **The runtime is shard.** Allocator, count discipline, any recovery
   code: proven shard, lowered by the same pipeline, differentially
   gated like everything else. No trusted C anywhere.
5. **Totality machinery.** Measures bound recursion depth; sizes and
   consumption are provable, which is exactly what turns
   dynamically-sized data into statically-reservable data.


## 3. The tower of cancellation theorems (the spine)

What shard actually needs is: **one checked allocator, one counted-cell
discipline, and a ladder of theorems, each of which discharges a
runtime mechanism.** Memory class is a property of the CHOSEN
REPRESENTATION, declared at the lowering-form layer and verified by
gates — never inferred by a hidden compiler analysis (§10). A
program sits wherever its proofs reach, per-decl.

Where the classes get their operational semantics (the application
ruling, rulings block): each class below is a family of **imp
spellings** — frame slots and destination windows for `frame`,
explicit region allocation/death for regions, headered cells and
count ops for `shared` — and each cancellation theorem is a spec ⊑
imp obligation, proven once, target-free. The ISA models never see a
memory class; by imp, every allocation decision is already explicit
bytes-and-windows.

**Base class — `shared` (counted heap).** Unbounded, genuinely shared
data. Precise reference counting over headered cells:

- Purity makes the managed-reference graph acyclic (§4), so
  count-zero ⟺ dead. Counting is a PRECISE liveness tracker here, not
  the leaky approximation it is in impure languages — no cycle leaks,
  no weak references, ever.
- The lowering emits increments at reference-duplication sites and
  decrements at syntactic last-use points. Frees are deterministic,
  local, and bit-reproducible in the model: no root discovery, no
  stack maps, no collection points, no ambient service. The 1:1
  model/machine story extends to heap traces; tiny-heap differential
  vectors force frees and OOM deterministically.
- Prior art as evidence (not authority): Lean 4 and Koka compile pure
  languages by precise counting plus reuse (Perceus) to C-class
  performance. Notably that design was DERIVED from pure-language
  structure — it is the non-mimic lineage.

**The cancellation theorems**, each proven per-decl or per-site, each
deleting runtime work:

- **+ non-retention (`borrow`)** — a parameter mode: the callee
  provably retains no managed reference beyond the call, so count
  traffic on that edge cancels. Perceus infers this heuristically;
  shard proves it and the accepts machinery percolates it like width
  premises today.
- **+ uniqueness (count ≡ 1)** — increments/decrements cancel
  entirely and OWNED MUTATION is licensed (§5): in-place reuse at
  cell granularity, linear-memory models at region granularity. This
  is Rust's ownership model derived as a theorem instead of imposed
  as a type system.
- **+ size/depth bounds** — the heap cancels into the **`frame`**
  class: statically-sized flat representations in caller-provided
  stack space (§6, §7). A proven capacity refinement, e.g.
  `(refine (List U8) (λ l. (le (len l) N)))` represented as a flat
  N-slot buffer plus a length word, is heapless-Vec with the runtime
  check proven away.
- **+ word-sized** — the **`register`** class: the already-ratified
  refined-scalar plan (LOWERING.md §7 item 9, scalar half; refined
  u8/u32/u64 on `(refine BASE PRED)`).
- Degenerate corner, still legitimate per-program: **+ total-allocation
  bound** — a batch bin with a proven whole-run allocation bound may
  preallocate and never free (everything cancels, including recovery).
  Rejected as the GLOBAL story (the bump-only ruling); available as a
  per-decl choice where the theorem exists.

**The dissolution property.** Machinery links per-program: an
all-frame/register program links zero allocator and zero counts — an
embedded-C-shaped binary of .text + stack. A shared-class program
links the allocator and the count discipline and nothing else. This is
the harness-dissolution identity applied to memory: nothing ambient,
nothing that cannot dissolve.


## 4. The managed graph and the acyclicity theorem

Two layers of pointer structure, sharply separated. Getting this
distinction right is what makes counting precise AND mutation legal.

**Managed references** — the fields the allocator and count discipline
know about. Discipline: set at construction to pre-existing cells
(or rewritten under a uniqueness license, §5, carrying the same
obligation). Acyclicity of this graph is NOT an asserted side
condition: a cyclic managed graph has no finite readback, so the
rep-relation obligation "these cells read back to this pure value" is
simply unprovable for it — the pure term it would denote is infinite.
**Finite-readback certs enforce the DAG**; no separate invariant is
trusted. (Corrected derivation, 2026-07-11: acyclicity is a property
of the managed-rep discipline enforced by its certs — NOT of "shard
is pure," which would be false one layer down.)

**Region-internal structure** — indices and offsets inside a single
managed object: a std/mem window, a linear-memory model, a flat
buffer. Cycles are welcome here: doubly-linked lists, union-find
parent loops, adjacency structures. The manager sees ONE object; the
internal structure is the program's proven business, invisible to
counting, costing zero per-node overhead. This is where the C-parity
algorithm classes live (in-place sort, hash tables, graph
algorithms). Idiomatic Rust does the same thing with indices into an
owned Vec by convention; shard does it by collapse proof.

**The hybrid** — a mutable region whose slots hold managed references
(e.g. a mutable array of pointers to shared values) — needs count
discipline on slot writes: decrement the old referent, increment the
new. Local and deterministic like everything else, and the
finite-readback argument still excludes cycles threaded through
slots (the denoted pure state would again be infinite). Pure-index
regions — the common case for performance-critical structures — pay
nothing.


## 5. Owned mutation (the uniqueness rung)

What licenses collapsing a pre-state and a post-state onto the same
bytes is always the same fact: **the pre-state is provably dead** —
the state thread is linear. One theorem shape, two granularities:

- **Cell granularity — reuse (functional-but-in-place).** A cell whose
  count is provably 1 at a site that consumes it and constructs a
  same-shape result is updated in place. Pure `map` over uniquely-held
  structure compiles to an in-place loop with zero allocation.
  Statically-proven uniqueness needs no runtime check; a dynamic
  count==1 test enables opportunistic reuse as the fallback (in-scope
  question D4).
- **Region granularity — linear-memory models.** The representation
  change from a persistent structure to a mutable byte region:
  high-level shard tracks previous and future states as separate pure
  objects; the collapse proof (spec ⊑ mutating model) rests on the
  linearity of the state thread. Landed precedent: the mem arc —
  std/mem's mask-on-read substrate, wasm_rev / wasm_copy proven
  spec⊑wasm and V8-green. The native rungs extend this, they do not
  invent it.

The write-once invariant refines accordingly: **write-once while
shared; mutable while proven unique.** Mid-extent mutation is
invisible to composition because certs are denotational at boundaries
(§2.3); the frame conditions carry everything not owned.


## 6. Layout: two rep families and a proven bridge

- **Frame representations: flat, unboxed, headerless.** Sizes from
  types and proven bounds; byte-window readback. The memory-WINDOW law
  generalizes directly — a frame is a window based at the stack
  pointer; result space is carved from the caller's frame
  (destination-passing, §7); disjointness by construction. This
  answers LOWERING.md §7 item 6: structured results live in memory
  via a destination window; one scalar + memory IS enough return
  surface.
- **Heap cells: headered.** A header carries count, tag, and size;
  payloads hold scalars and managed references, distinguishable
  (per-type layout maps or tag bits — the exact scheme is an
  emit-level decision made at rung time in the target docs, D6).
  Payloads are write-once while shared (§5).
- **The bridge.** Frame value → heap cell (boxing at the frontier) is
  a proven serialize, a readback-level identity. This is LOWERING.md
  §7 item 5's cross-rep conversion glue, scoped: the two families are
  kept honest separately and converted explicitly — never merged into
  one compromise layout.
- **The mixing rule.** Frame values may hold managed references
  (these are exactly the explicit roots, and their frame-pop
  decrements are the Drop points). Heap cells never hold frame
  pointers — statically-sized frame values are COPIED in, and purity
  makes the copy semantically invisible.


## 7. Exhaustion: the two finite resources

Both get the honest-caveat treatment: visible, explicitly handled at
a named boundary, no global acceptance clause.

**Heap.** Allocation is checked; OOM is a value at exactly one
observation point, not a trap and not an ambient premise.

- *Tier 1, mid-graph:* certs carry per-extent slack premises ("heap
  remaining ≥ B", with B affine in the extent's inputs where the
  fragment class supports it) plus deterministic consumption
  postconditions, composed caller-ward exactly like fuel today. Under
  counting, frees return space at exact points, so accounting stays
  deterministic and premises stay additive within extents. No cert
  conclusion grows an OOM leg.
- *Tier 2, boundary:* one check where the artifact meets the world —
  the bin/lib glue contract (the glue_fams growth point), like exit-4
  stuckness. With recovery in place the caveat states the TRUE
  condition: "live data fits the window." Bump-only's caveat was the
  history-dependent "total allocations ever ≤ W" — semantically wrong,
  which is WHY it made basic problems intractable and why it was
  ruled out.
- *Interpreter-class programs* (allocation not affine in inputs): the
  **budgeted twin** — a certified shard→shard transform adding
  explicit fuel and heap clocks, with the theorem
  `twin(budgets, args) = Done v ⟹ v = pure(args)`. The pure function
  remains the spec; the twin is the total, resource-honest artifact.
  (Timing: D7.)

**Stack — the sibling of OOM, and it discharges better.**

- Frame sizes are per-fn static constants; required depth D is a sum
  over call paths — static for non-recursive graphs, and TCO turns
  accumulator recursion into O(1)-stack loops.
- The x86 model's fuel is already depth-shaped (X86.md §24), so the
  stack-bytes premise rides the existing fuel discipline
  (frame-size-weighted), not new machinery.
- Discharge by construction: we own the whole binary (ZERO C), so a
  bin whose cert demands D bytes RESERVES D in the artifact layout.
  Stack overflow is impossible-by-construction for the frame class —
  the premise is discharged by the artifact itself, not checked at
  runtime.
- Scope rule: frame class ⇔ statically bounded everything. Non-tail
  recursion over unbounded data was never frame class — the data
  forces heap; measure-affine depth programs either TCO or take the
  heap/twin path.


## 8. Recovery and residuals (the honesty ledger)

Recovery = counting's frees: deterministic, exact, local, mandatory
from v1. The known residuals and their named mitigations:

- **Fragmentation** — the residual disease of any non-moving scheme,
  and this design's weakest point. Mitigations, in order: size-classed
  free lists (bounded internal fragmentation, provable); the optional
  region/arena class for bulk-death allocation patterns (D5); an
  explicit compaction primitive — copy live data from DECLARED roots,
  with a readback-preservation theorem — available as an accessory
  for long-running programs. An accessory, never the spine. If
  measured fragmentation on real workloads defeats all three, §10's
  tracing-GC verdict is the thing to revisit — with data.
- **Cascading frees** — dropping the last reference to a large
  structure triggers recursive frees. Resolution: thread the free
  worklist through the cells being freed (freed cells ARE spare
  memory); O(1) auxiliary space, deterministic, no stack growth.
- **Count traffic on hot shared data** — the borrow theorem cancels
  most of it; what remains is the price of genuine sharing, and
  per-decl escape hatches exist (regions; the preallocate corner for
  proven-bounded batch runs).
- **The count word weakens write-once** — payloads stay immutable
  while shared; the header count is the one mutated word. Contained,
  headered, and stated in the heap invariant rather than worked
  around.
- **The heap invariant is real proof work** — every cell's count
  equals the number of managed references to it from live roots and
  cells; each construct/inc/dec preserves it; the free theorem
  (count 0 ⟹ removal preserves all other readbacks) is proven once.
  The work is LOCAL and separation-logic-shaped. The contrast that
  justifies it: a tracing collector would force whole-heap
  quantification through every cert at every allocation site (§10).
- This file supersedes LOWERING.md §7 item 2's provisional invariant
  sketch ("bump-pointer validity + allocated-cells-never-rewritten"):
  the invariant is now the count invariant plus write-once-while-
  shared, and the PRE-slot conventions land with slice 4.


## 9. Rungs and flagships

The uniform-rep backlog restructures as this ladder. House discipline
per rung: ratified scope first, per-slice user check-ins, corpus pins,
model-vs-silicon differential where a native artifact exists, byte-tie
where a cert names bytes. Low-level emit specifics (header encodings,
frame conventions in machine bytes) land in the target docs at rung
time — this file stays at the architecture level.

Rung residence after the application ruling: rung 1 (scalars) sits
below imp and is unchanged. Rungs 2–5 are stated and proven AT imp
(docs/IMP.md's ladder interleaves: its I0/I1 build the machine and
the ISA legs; its I2 is this file's rung-2 flagship; the counted
heap arrives at imp when rung 4 opens). Each flagship is proven once
at imp and landed per target by the imp ⊑ ISA families.

SCHEDULE REVISION (2026-07-12, the coverage inversion — rulings
block): the ladder below remains the CLASS structure, but the build
order inverts — rung 4 opens as the next arc after IMP.md's I2d/I2e
(the uniform-rep compiler over imp, certs generated from day one;
IMP.md's redirection block is the arc record), with rung 1's scalar
policy folded into its opening pins and rungs 3/5 landing afterward
as measured per-decl upgrades. The full-gap pathfinder milestone:
examples/calc as a proven (bin …) on silicon through the generic
path (strings, lists, ADTs, recursion — the key types crossing the
entire gap).

1. **Scalars (`register`).** The ratified LOWERING.md §7 item 9 plan:
   kernel-inner refined u8/u32/u64; fit obligations discharged from
   source invariants at construction sites. Shared prerequisite of
   every rung below.
2. **Frame class.** Destination-passing calling convention,
   frame-carved windows, flat aggregate layouts, artifact-reserved
   stack. **Flagship: std/sha256 → native x86, zero heap** —
   `./sha256sum` over argv on silicon; hash state is eight u32s, the
   schedule a fixed 64-slot buffer, and the module is already
   NIST-pinned with length proofs. Demonstrates the embedded bar the
   way `./addw` demonstrated the World rung.
3. **Owned mutation over buffers (region granularity).** The mem-arc
   collapse proofs extended to the native path: an in-place algorithm
   on a frame- or unique-held buffer (wasm_rev's native sibling, or
   an in-place sort). No heap machinery yet — uniqueness at region
   granularity rides the frame class.
4. **Counted heap.** Headered cells; the checked allocator (proven
   shard, size-classed free lists); inc/dec emission at duplication
   and last-use; the free theorem; cascading-free worklist; tier-1
   premises + tier-2 boundary check; tiny-heap differential vectors.
   **Flagship direction: the eval.shard aggregate half** — the env
   machine's explicit state tuple makes its sharing structure (shared
   environments) and its roots explicit; the evaluator is the
   canonical genuinely-shared workload.
5. **The performance cancellations.** Proven-borrow edges,
   proven-unique in-place reuse (and dynamic count==1 reuse if D4
   rules it in) — measured against rung-4 baselines, closing on the
   C/Rust criterion.
6. **The budgeted twin** — when the eval flagship demands a total,
   resource-honest artifact (D7).


## 10. Non-goals, stated once

- **Tracing GC.** The dynamic+bulk quadrant. It costs shard everything
  it prizes: an ambient service that never dissolves; root discovery
  (stack maps, register pointer classification); rep relations
  quotiented by address renaming through every cert; whole-heap
  invariants at every allocation site; and a byte-interpreting scanner
  in what should be byte-moving plumbing. Under precise counting all
  of that machinery cancels. Recorded concession for honesty: purity
  would hand a tracing design free generational precision (old cells
  cannot reference young ones — no write barriers, no remembered
  sets), but that optimizes a mechanism we do not want. Revisit only
  on measured fragmentation evidence per §8.
- **A memory type system.** No linear types, no ownership annotations,
  no lifetime language: uniqueness, borrow, and bound facts are
  theorems about lowered forms, and nothing in this ledger needs core
  syntax — `(refine …)` exists; everything here is ordinary
  definitions plus certs. Stated precisely (USER, 2026-07-11): the
  kernel is NOT immutable — shard is a whole package whose usefulness
  has bought core provisions before when the case cleared the high
  bar (refine, have, fin-split, subterm-induct, the Nat former). The
  non-goal is kernel growth as a SAFETY MECHANISM, which the safety
  inversion (§1) makes pointless. If a declaration surface (D1, D3)
  someday earns core QoL on ergonomic evidence, that is an ordinary
  user-decided kernel-growth case, not a violation of this file.
- **Inferred escape analysis or any hidden liveness heuristic.** Class
  assignment is declared and gate-verified. Plumbing moves bytes; it
  never guesses liveness.
- **A single compromise layout.** Frame reps stay flat and headerless;
  heap cells stay headered; the bridge is proven, not avoided.
- **Playground-style leak-and-exit as a default.** The preallocate
  corner exists only behind a proven whole-run bound.


## 11. Open decision points

- **D1 — class-assignment surface.** The principle is ratified
  (declared, proof-backed, gate-verified), and the application
  ruling fixes WHAT is assigned: a class assignment selects the imp
  spelling of the spec → imp step (rulings block; docs/IMP.md §4).
  The remaining surface question — how assignments are written in
  the profile (per-type in the decl, per-binding, per-decl with
  percolation) — resolves at BUILD.md rung 3 jointly with IMP.md
  I3, with the frame flagship in hand.
- **D2 — accounting units.** Tier-1 premises in bytes-via-sizeof
  (lean: regen absorbs repricing when layouts change) vs abstract
  cells. Decide with slice 4's first premise.
- **D3 — borrow/unique declaration surface.** Where the non-retention
  and uniqueness theorems attach (per-param modes on the decl, in the
  accepts family?) and how they percolate. Slice 5 scope.
- **D4 — dynamic reuse.** Does rung 5 include the runtime count==1
  opportunistic-reuse path, or proven-unique-only first? Lean:
  proven-only first — it keeps rung 5 free of new runtime branches;
  the dynamic path is an additive follow-on.
- **D5 — the region/arena class.** In v1 or named-later? Lean:
  named-later — frame covers bulk-death for bounded data, counting
  covers the rest; regions enter if fragmentation or count-traffic
  measurements demand a bulk tier.
- **D6 — header and tag scheme.** Emit-level; decided at rung 4 time
  in the target docs, not here.
- **D7 — budgeted-twin timing.** PARTIALLY RESOLVED by the coverage
  inversion (2026-07-12): the twin's THEOREM SHAPE
  (`Done v ⟹ v = pure(args)`) is promoted into D8's Done-or-Fail
  default cert conclusion for the checked coverage tier — realized as
  checks in GENERATED imp, not a source-level transform. A separate
  source-level twin artifact stays deferred as before.
- **D8 — the controlled-failure surface (OPENED 2026-07-12; direction
  under design, resolves early in the coverage arc).**
  Lowering-induced partiality (word overflow, OOM, stack depth) —
  conditions that do not exist in high-level shard and develop as a
  consequence of implementing it on hardware — becomes CONTROLLED
  FAILURE by default: generated imp threads an explicit fail leg
  (checked ops + the checked allocator), the cert conclusion is
  Done-or-Fail — `run = Done (SPEC args) ∨ Fail(family)` with the
  fail families honest — and the requirement surface grows an
  `except` clause at the artifact boundary (the bin contract states
  the observable fallback: v1 = abort-with-diagnostic, with emitted
  World effects a PREFIX of the spec's). Enforcement is the accepts
  ratchet's twin: an artifact's fail families must be covered by its
  declared except clause in both directions, or the build fails —
  the proof system makes the escape hatch mandatory (the disjunction
  cannot be eliminated without either a premise or the clause). The
  three-tier resolution ladder per condition: DISSOLVE (explicit
  U32/refined types in source — no check, no clause), GUARANTEE
  (accepts premise — check elided, caller obliged), HANDLE (except
  clause — check emitted, fallback certified). Checks are the
  degradation default and proofs DELETE them — the cancellation-tower
  pattern applied to partiality. Non-mimic precedent for the theorem
  shape: CakeML's compiler theorem (behaves-as-source OR terminates
  early with a resource error). Open sub-questions: the except
  clause's grammar and family granularity; the machine-level Fail
  value (imp needs a reasoned fail distinct from ITrap and from fuel
  None); the stack family's v1 mechanism (depth counter = fuel made
  real, vs the frame class's discharge-by-construction); app-defined
  fallback handlers (deliberately OUT for v1 — a handler runs in a
  resource-compromised state; apps wanting specific behavior take the
  dissolve tier).
