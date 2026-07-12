shard imp — IMP.md
==================

STATUS: DRAFT (2026-07-11) — the scope ledger for **models/imp**, the
neutral imperative dialect: one target-neutral machine model that
keeps the memory-allocation story and drops the ISA-specific quirks.
imp is the "common lowering step" docs/ISA.md predicted as the major
development to come, the operational home docs/MEMORY.md's class
vocabulary was missing, and the natural manual spelling target for
modules before graduating to full `.wasm.shard` / `.x86.shard`
dialect twins. This file owns the machine's definition, its position
in the proof tower, its authoring/product surface, and its rung
ladder. It does NOT own the memory-management design itself
(docs/MEMORY.md), the ISA models (docs/X86.md, the wasm model), or
the build machinery (docs/BUILD.md) — it is the joint where they
meet.

User rulings already on record from the design discussions of
2026-07-11 (do not relitigate silently):

- **The dialect is wanted.** A neutral imperative dialect that keeps
  most of the memory-allocation story minus the ISA-specific quirks
  is a useful shape to include: a more natural manual spelling
  target for modules before graduating to the full ISA dialects.
- **The name is models/imp** — distinct, on-the-nose, and unlikely
  to alias onto anything else, which matters because `imp` becomes a
  common token if the approach scales.
- **The float arc slots in at merge.** The floats fork (docs/
  FLOATS.md, ratified rulings) builds a value-parametric core float
  model (L1/L2) that is already target-neutral: imp's float
  instructions enter by CITATION of std/float core ops, and
  FLOATS.md's tier-1 bridge theorems (core ⊑ arch up to the NaN
  quotient) are exactly imp→ISA lowering obligations. Neither arc
  blocks the other: imp v1 has no float dependency; float ops join
  imp as a later rung when the fork merges.

Standing constraints inherited whole: the ISA-arc discipline (a
model is an ordinary shard library; composition is citation; ZERO
kernel changes), the C-class dissolution law (imp never ships — it
is a proof-bearing intermediate, and its differential engine is
shard's own evaluator), the Word-lesson/value-parametric ruling
(FLOATS.md §3a: no numeric type parameters; width and format
descriptors are values consumed at proof/build time), and the
safety inversion (MEMORY.md §1: imp competes on layout efficiency
and proof-landability; safety crossed the bar with the refinement).


## 1. Purpose and stance

Today a lowered module has exactly two spellings: high-level shard,
or a per-ISA model dialect (`.wasm.shard` with block/br indices and
i32 wrap; `.x86.shard` with registers and flags). Every memory
decision — where a value gets a frame slot, when a region is carved,
where a copy lands — is entangled with ISA encoding, stated twice,
and proven twice. The neutral dialect factors this:

    spec  ⊑  imp  ⊑  wasm
                  ⊑  x86

- **spec ⊑ imp is where the memory story lives.** MEMORY.md's tower
  of cancellation theorems — frame and region cancellations, borrow
  and uniqueness erasure, later the RC spine — is stated ONCE,
  against the imp machine, where "frame", "region", "cell", and
  "copy point" are first-class. A memory class IS a choice of imp
  spelling (§4).
- **imp ⊑ ISA is memory-story-free.** By the time a program is imp,
  every allocation decision is explicit; the per-ISA obligation is
  instruction selection — per-construct lowering families,
  generate-and-check, the existing six-gate discipline.
- **The M×N cancellation.** Without imp, every memory class × every
  target needs its own theorem family and its own spelling, drifting
  independently (the canon catches of the build arc were exactly
  same-decision-different-spelling drift). With imp: M classes
  proven once at imp, N targets each proven once against imp.

imp is a MODEL, not a syntax: ordinary shard types plus step/eval
functions plus theorems, exactly like models/wasm and models/x86.
`.imp.shard` twins are ordinary shard files that construct imp
machine values, the same way `.wasm.shard` twins construct WModule
values today.


## 2. The machine (v1)

Small by law. imp keeps what the memory story needs and refuses
everything that is one ISA's quirk:

**In:**

- **Locals and frames.** Named slots of scalar or managed-reference
  kind; a frame is the extent-scoped window MEMORY.md §6 describes,
  with destination-passing result windows carved from the caller's
  frame. Extent structure (who owns which window, when it dies) is
  imp-level truth; byte-exact frame layout is the ISA docs' business
  at rung time (MEMORY.md D6 discipline).
- **Structured control.** Loops, branches, calls — structured forms
  only, chosen so both wasm's block/br discipline and x86's jumps
  lower from one shape. No branch indices, no labels-as-offsets.
- **Byte regions over std/mem.** Mem IS the neutral memory — both
  ISA models already consume it (mem arc; mask-on-read, LE settled).
  imp's load/store/window ops are Mem ops; region allocation and
  region death are explicit imp operations so the region
  cancellation theorem has syntax to attach to.
- **Scalars: unbounded Int with explicit wrap points.** Locals hold
  Int in the theoretical-language spirit; width enters as explicit
  wrap/width ops carrying premises — the cert premise discipline the
  fragments already use, `Target.width`-parametric in the WORD
  fragment's Int-binder style (LOWERING.md §6ah). One imp twin
  serves both a 32- and 64-bit target; the descriptor is consumed at
  proof/build time (the pinned-literal-spine precedent), never at
  runtime.
- **(post-float-merge) Float ops by citation.** `(fadd fmt x y)`
  etc. cite std/float's L1/L2 core semantics directly; the NaN
  observation quotient (FLOATS.md §5) is the model-boundary law, and
  imp fragments inherit the bit-preserve-never-branch invariant.

**Out (permanently, at this layer):**

- Registers, register allocation, calling-convention byte details.
- Flags, status words, rounding-mode state, MXCSR — machine-state
  honesty lives in the ISA models (FLOATS.md §8 keeps imp clean
  here: RNE-only pure float ops, state pinned below).
- Block/br indices, encodings, relocation, anything ELF/wasm-binary.
- An imp interpreter that ships. The model's eval exists for proofs
  and differential gates and dissolves like every harness.


## 3. The trust story

Nothing new, by construction:

- **imp's semantics** are its written-in-shard step/eval functions —
  an ordinary library. Theorems cite it (ISA.md: composition is
  citation). Zero kernel growth.
- **spec ⊑ imp** certs are ordinary refinement theorems; this is
  where MEMORY.md's cancellation obligations (finite-readback,
  linearity of the state thread, capacity refinements) attach.
- **imp ⊑ ISA** is a per-construct lowering family per target,
  generate-and-check, gated by the existing ladder (schema, kernel,
  byte-tie, manifest, engine). The ISA models remain where hardware
  truth lives.
- **The only empirical pins stay where they are**: V8 and the
  on-CPU runner gate the ISA models against reality. imp adds NO new
  differential surface against the world — its engine leg runs on
  shard's own evaluator (spec vs imp-model traces on curated
  vectors), which is a proof-side instrument, not a trust boundary.


## 4. The memory-class joint (with MEMORY.md and BUILD.md)

- **A memory class is a choice of imp spelling.** `frame` means the
  value lives in frame slots / a frame-carved window; a region class
  means an explicit region with an explicit death point; `shared`
  (later) means headered cells and count ops. MEMORY.md's D1
  class-assignment surface therefore steers the **spec → imp** step
  and nothing else.
- **The profile is the steering wheel** (BUILD.md rung 3): the
  zero-config end takes default classes → default imp derivation;
  the fully-manual end is a hand-written `.imp.shard` twin; between
  them, profile class assignments parameterize the derivation. This
  is the original spectrum question answered: "compile my program,
  don't make me fuss" and "I spelled it out by hand" are the two
  ends of one surface.
- **Graduation ladder for authors** (human or model): spec → imp
  twin (memory story explicit, ISA-free) → ISA twin only when an
  ISA-specific trick is genuinely wanted (SIMD, syscall shapes). The
  multi-impl backbone already accommodates coexistence: an imp twin
  and a hand ISA twin are conformant impls of one mod.req surface;
  the profile's variant selection picks per target.


## 5. Authoring and products (with BUILD.md)

The build vocabulary absorbs imp without new concepts:

- **PIN**: a hand `.imp.shard` twin, pinned by claims exactly like
  ISA twins (PinMod carries it; the prefix convention is unchanged).
- **DERIVE**: the aspirational default — hand-write (or derive) imp
  once, DERIVE both ISA twins + certs mechanically (the same
  derive-and-verify-raw shape the build arc validated: packaging
  and bindings derived, manifest gate re-checks against certs read
  raw).
- **SYNTHESIZE**: metaprograms emit imp content, never packaging
  (the mod.build charter, verbatim).
- **Products**: an imp twin gates as a product with target 'imp —
  schema, kernel, tie against the entry's declared machine values,
  and the in-shard differential engine leg. PVec is reused for
  vectors. Details land driver-side at rung I0; expected to be a
  small slice on the slice-7/8 pattern.
- **The existing generators are not ripped up preemptively.**
  wasmgen/x86gen keep their direct lowsrc→ISA path; leaf fns may
  keep it forever (a second layer is pure cost for a three-line
  fn). imp absorbs the front end (lowsrc→imp, one front + two
  backs) under the standing rip-up license once it demonstrates
  parity on the fragment corpus.


## 6. Rungs and flagships

House discipline per rung: ratified scope first, per-slice check-ins,
corpus pins, byte-tie where a cert names bytes.

- **I0 — the machine.** models/imp v1 (locals/frames, structured
  control, Mem regions, Int scalars with wrap ops) + the in-shard
  differential harness + imp twins of the existing straight-line and
  loop fragments. Gate: spec ⊑ imp certs for those twins check
  green; differential vectors pass on the model's own eval; corpus
  diff-clean.
- **I1 — the ISA legs.** imp ⊑ wasm and imp ⊑ x86 lowering families
  for the I0 fragment classes, generate-and-check, six gates, wasm
  first (width-ordered coverage precedent). Gate: the SAME imp twin
  lands green artifacts on both targets.
- **I2 — the first cancellation flagship.** MEMORY.md rung 2 stated
  at imp: **std/sha256 zero-heap** — frame class + one region,
  region cancellation proven at imp, `./sha256sum` on silicon and
  under V8 from one twin. This is where "proven once, landed twice"
  is demonstrated on a real module.
- **I3 — profiles consume it.** BUILD.md rung 3 lands class
  assignment as spec→imp steering (MEMORY.md D1 resolves here),
  variant selection chooses imp-derived vs hand ISA twins.
- **I4 (post-float-merge) — float ops + the layout flagship.**
  std/float citations enter the machine; FLOATS.md's GEMM flagship
  (BF16-in, F32-accumulate) exercises §6-packing regions and loops
  at imp — the layout complement to I2's cancellation story.

Later, unscheduled: the counted-heap class at imp (MEMORY.md rung 4
restated), the lowsrc→imp front-end absorption, imp-level reuse
(Perceus-shaped) once counting exists.


## 7. Non-goals, stated once

- imp as a shipped target or public surface — it is an intermediate;
  terminal targets remain the ISA models.
- imp as kernel syntax — it is a library; the reader never learns it.
- Registers/flags/modes/encodings at imp level (forever).
- A general optimizer at imp level — rep choices are declared and
  proven, never discovered by a hidden pass (MEMORY.md §10's
  no-hidden-liveness rule applies to imp verbatim).
- Per-width imp twin families — width is value-parametric with
  premised wrap ops; the Word residue is not re-created here.


## 8. Open decision points

- **DI1 — fragment grammar granularity.** Does I0 adopt the existing
  fragment taxonomy (straight-line / loop / mem / calls-in-loops) as
  imp's fragment classes verbatim, or define imp-native classes and
  map the existing four onto them? Lean: adopt verbatim for I0–I1
  (byte-diffable against the landed corpus), revisit when the
  counted-heap class arrives.
- **DI2 — the imp twin's vector story.** Curated PVec vectors like
  pinlib entries, or spec-derived vectors (the source fn applied,
  slice-7 style)? Lean: spec-derived — imp twins always have a
  runnable spec sibling by construction.
- **DI3 — how much frame convention is imp-level.** Extent/ownership
  structure yes; slot packing and alignment? Lean: imp states sizes
  and disjointness abstractly; byte-exact packing stays with the ISA
  docs (MEMORY.md D6), revisited if the imp⊑ISA proofs want more.
- **DI4 — residence of the lowering families.** models/imp alongside
  the machine, or tools/-side with the generators? Lean: theorems
  with the model, generators in tools/, matching the wasm/x86 split
  today.
