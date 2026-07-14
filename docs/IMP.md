shard imp — IMP.md
==================

STATUS: RATIFIED (2026-07-12; drafted 2026-07-11; §2a typed-machine
amendment ratified 2026-07-14) — the scope ledger
for **models/imp**, the
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
- **The container entry point (2026-07-12).** imp is the natural
  home for Vec-class primitives — containers that rely on heap
  behaviors and surface EXPLICIT allocation-fail results. Lowering
  high-level List onto imp-level Vec is a standard, convenient path
  for both pins and auto-lowering (§4a).
- **Decision-point resolutions (2026-07-12):** DI1 — the existing
  fragment taxonomy verbatim until otherwise needed. DI2 — RESOLVED
  BY DISSOLUTION: no differential vectors at imp as a build gate;
  the kernel gate is the gate (§3). DI3 — discovered as the pieces
  are fit together; stays open. DI4 — as leaned (theorems with the
  model, generators in tools/).
- **The typed-machine re-adjudication (2026-07-14).** The refined
  premise: manually-written imp twins are FIRST-CLASS refinement
  inputs — a custom imp refinement of a high-level module must be
  refinable to every accepted target with 100% coverage, exactly
  like compiler-emitted imp. Consequence: v1's unbounded-Int
  scalar story is superseded by crystallized scalar kinds (§2a) —
  explicit kind tags on op nodes plus the well-kinded gate; U8
  restricted to load/store/compare/convert in v1; addresses = U32
  indexes with a declared memsize, realized per target as
  base+offset; the CAPABILITY-SET doctrine replaces
  width-parametricity; the landed tiers migrate (the sha sibling
  included, not frozen).

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
  runtime. [SUPERSEDED 2026-07-14 — the v2 re-adjudication
  crystallizes scalar kinds at imp; see §2a. Kept as the story the
  v1 rung records were built against.]
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


## 2a. The machine v2 — crystallized scalar kinds (ratified 2026-07-14)

User-initiated re-adjudication. The refined premise: hand-written
imp twins are first-class refinement inputs — whoever writes a
custom imp refinement of a high-level module must get generated
imp ⊑ ISA legs with 100% coverage, exactly like compiler-emitted
imp. The v1 scalar story fails that premise at the root: unbounded
Int locals are the SOLE source of program-dependence in the
machine-vs-imp alignment relation. Wherever a twin uses exact ops,
"machine local = imp local" holds only under range facts; range
facts across loops are per-program invariants; and impgen's
recognizer tiers existed to synthesize those invariants from
memorized body shapes — a closed-family mechanism that can never
be total. The evidence was already in this file: the IWrap bridge
is UNPREMISED (imp_w_add1w — imp's explicit wrap and the machine's
inherent wrap are the same mod), while the unwrapped I1 bridges
carry the same premise families PER TARGET in each machine's
modulus (I1c: "2^64 where the wasm leg said 2^32") — the width
decision paid N times downstream of the layer whose purpose is to
force lowering decisions once. Type crystallization belongs at imp
alongside layout; v1 commonized the layout half and left the type
half out.

- **The kind set.** A closed value-level vocabulary: U8, U32, U64
  — unsigned first, matching the machines' unsigned story; signed
  variants are named growth behind a consumer. Kinds are ordinary
  ctor tags consumed by eval and the translators as VALUES, never
  type parameters — the Word-former lesson and the
  value-parametric ruling (FLOATS.md §3a) are complied with, not
  revisited.
- **Attachment and the gate.** Every IFn signature declares each
  local's kind; every op node carries its kind explicitly —
  (IBin U32 IAdd a b) — because refl-grade syntactic alignment is
  what pays (the band-spelling lesson). A structural WELL-KINDED
  predicate (wk_fn) joins the 'imp product gate: operand kinds
  agree, constants sit in band, conversions are explicit, shift
  counts are below width.
- **Op semantics.** Every arithmetic op wraps to its kind; the
  machine invariant is that every local holds an in-band value of
  its declared kind. IWrap DISSOLVES (every op is a wrap point);
  IRotr's width parameter becomes its kind; the band-mask idiom
  (the sha sibling's m32 spelling) becomes redundant. Conversions
  are explicit nodes — IExt (zero-extend up) and ITrunc
  (mask-narrow down) — the crystallization points, proof-visible.
  U8 is load/store/compare/convert only in v1 (no U8 arithmetic;
  convert to U32 first). Shift counts at or above width are
  EXCLUDED (well-kinded for constants, guarded for symbolic):
  wasm masks counts mod 32 and x86 mod 64, so an out-of-range
  count is precisely the class of per-target quirk imp exists to
  exclude rather than parameterize. Wrapping is the total
  default; checked variants (trap/Fail on overflow) arrive with
  MEMORY.md D8's reasoned Fail value — this rework does not
  entangle with D8.
- **Memory and addresses.** Byte grain stays the primitive (ILoad
  yields U8); word-grain accessors remain the named perf rung and
  take LE per the mem-arc precedent when they land. Addresses
  never enter imp as machine pointers: an imp address is an INDEX
  into the model's own memory, held in an ordinary U32 local,
  with memsize a DECLARED parameter of the twin (the hardcoded
  65536 retires). The target leg realizes index → address as
  base + zero-extended offset — wasm linear memory literally is
  this, and the x86 leg gains the base-register convention.
- **The capability doctrine (replaces v1 width-parametricity).**
  imp kind semantics are target-independent; what varies per
  target is which kinds it supports, never what an op means. Each
  target model declares a CAPABILITY SET (native kinds,
  addressable region bound, op residue such as division); the
  acceptance gate is per (twin, target): accept iff the twin's
  declared needs fit the target's capabilities, refusals loud and
  naming the missing capability. U8/U32 plus U32 indexes form the
  portable core — native or free on every real platform (wasm
  i32; x86_64's 32-bit operand forms zero-extend results, so U32
  is maskless-native there). U64 is native on BOTH current
  targets (wasm has i64; the local model grows the vocabulary). A
  genuinely 32-bit future target refuses U64 twins in v1; a
  proven pair-arithmetic emulation rung is additive growth behind
  a consumer. A new target ships as: ISA model + per-kind op
  lowerings + capability declaration; existing twins that fit run
  unchanged. This is what makes the future target set safely
  unbounded — the kind lattice is the negotiation interface, and
  set inclusion at the gate replaces semantic parameters inside
  the machine.
- **What v2 buys.** The alignment relation "machine local = imp
  local" is exact and program-independent BY TYPE. The
  bridge-side width-residue apparatus — wrap32/wrap64 haves,
  per-modulus range premise families, the k-scaled accumulator
  invariants that forced impgen into shape recognition —
  dissolves; generated proofs reduce to the structural walk plus
  guards plus fuel, total over well-kinded imp by construction,
  hand-written twins included. The fit obligations relocate to
  spec ⊑ imp, stated ONCE (MEMORY.md rung 1's refined scalars are
  the source-side supply line), instead of once per target.
  imp ⊑ ISA becomes width-story-free the way it was already
  memory-story-free (§1).
- **Consumer growth and migration.** The wasm model grows the i64
  op vocabulary; the x86 model grows non-REX.W 32-bit forms —
  encodings and differential vectors Opus-delegated per the
  standing split; ix_home 6 → 12 is orthogonal and still wanted.
  Migration is delete-first, file by file, corpus green
  throughout: machine + wk gate + re-validated probe grid →
  translators + the scalar tier re-landed → the loop tier → the
  sha sibling (masks dissolve into kinds; the continuation-phase
  machinery survives untouched — it is fuel/spine structure,
  width-free) → impgen rebuilt as the structural walk over
  well-kinded imp (the recognizer tiers retire, their generated
  files regenerated under the rebuilt tool). The coverage arc
  opens on the typed model.
- **Named-later growth, all consumer-gated:** signed kinds, U16,
  wide-mul high halves, word-grain accessors, U64 indexes for
  huge-memory targets, 32-bit-target U64 emulation. The
  uniform-rep compiler's default kind for unrefined source Int
  (U64 + D8 checked ops vs the heap tier) is a coverage-arc
  opening pin, not resolved here.


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
  on-CPU runner gate the ISA models against reality. imp adds NO
  differential surface at all — and deliberately has no engine gate
  (DI2 resolution, USER 2026-07-12). The ISA engine legs differ the
  models against EXTERNAL reality; imp has no external reality — its
  semantics ARE its shard definitions, so once spec ⊑ imp checks,
  vectors re-prove nothing, and real vectors already run end to end
  through the ISA engine legs downstream of any imp-derived
  artifact. The one legitimate vector use is DEVELOPMENT-TIME: a
  probe grid validating the machine means what we intended before
  proofs rest on it (the facts_probe / FLOATS.md toy-format idiom),
  built once at I0 and corpus-pinned — never a per-product gate.


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


## 4a. The container layer: Vec as the entry point

(USER ruling, 2026-07-12.) imp is the natural entry point for
Vec-class primitives: containers that RELY on heap behaviors —
growth, reallocation — and surface EXPLICIT allocation-fail results.
This is MEMORY.md §7's tier-1 story made concrete as a value: no
ambient OOM premise, no cert conclusion growing an OOM leg — the
fail leg is in the result type, observed at exactly the call that
allocated.

- **What Vec is here**: a unique-owned growable region (ptr/len/cap
  in imp terms) with a readback law — the Vec denotes exactly the
  List read back from its initialized prefix — and ops whose exact
  results are the List ops (push/pop/index/iterate), except that
  allocating ops carry the explicit fail leg. Growth policy
  (doubling) is an imp-level implementation with an amortized-cost
  statement, not a hidden runtime service.
- **The standard dynamic-data path**: List → Vec is THE default
  lowering for dynamically-sized sequence data, in both authoring
  modes — a hand pin writes imp Vec ops directly; auto-lowering maps
  List-typed spec values onto Vec when the class assignment says so.
  The rep-swap that founded the refinement-lowering vision (linked
  list → linear memory) becomes a REUSABLE library citizen instead
  of a per-module construction.
- **Residence and timing**: the container layer is a library OVER
  the machine (the analogue of std/mem over bytes), not machine
  primitives — §2's surface does not grow. Unique-owned Vec enters
  at the owned-mutation/region rung, no counting needed; SHARED
  containers wait for the counted-heap rung. Ladder position: I2.5
  (§6).
- **Beyond Vec**, the same shape serves the obvious family (string
  builders; hash tables as the §4-hybrid at region granularity) —
  each is a readback law plus explicit-fail allocating ops; none is
  scoped until a consumer names it.


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
  schema, kernel, and tie against the entry's declared machine
  values. PROOF GATES ONLY: no engine leg exists at imp (§3, the
  DI2 resolution). Details land driver-side at rung I0; expected to
  be a small slice on the slice-7/8 pattern.
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
  control, Mem regions, Int scalars with wrap ops) + the ONE-TIME
  machine-validation probe grid (§3 — a development instrument,
  corpus-pinned, not a product gate) + imp twins of the existing
  straight-line and loop fragments. Gate: spec ⊑ imp certs for
  those twins check green; corpus diff-clean.
- **I1 — the ISA legs.** imp ⊑ wasm and imp ⊑ x86 lowering families
  for the I0 fragment classes, generate-and-check, six gates, wasm
  first (width-ordered coverage precedent). Gate: the SAME imp twin
  lands green artifacts on both targets.
- **I2 — the first cancellation flagship.** MEMORY.md rung 2 stated
  at imp: **std/sha256 zero-heap** — frame class + one region,
  region cancellation proven at imp, `./sha256sum` on silicon and
  under V8 from one twin. This is where "proven once, landed twice"
  is demonstrated on a real module.
- **I2.5 — the container layer.** Unique-owned Vec over imp regions
  (§4a): the readback law, explicit-fail allocating ops, and the
  List→Vec default path wired into the class-assignment story.
  Flagship: a List-consuming module re-repped onto Vec with zero
  spec change.
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

**REDIRECTION (2026-07-12, user-ratified re-adjudication).** The
"later, unscheduled" line above is superseded: the COVERAGE ARC is the
spine. After I2d/I2e close the flagship, the next arc is the
UNIFORM-REPRESENTATION COMPILER over imp — the generic, TOTAL
spec → imp translation for arbitrary first-order shard (ctor = counted
cell alloc, match = tag dispatch + field loads, call = icall, tail
recursion = IWhile, non-tail = real calls on the models' structured
call mechanisms), with MEMORY.md rung 4's counted heap pulled forward
as its runtime and EVERY cert family generated from day one: per-TYPE
readback lemmas from the type declaration (the records-arc precedent),
per-CONSTRUCT heap-invariant lemmas proven once, per-FN certs = one
induction along the fn's own totality measure — the measure clause
every shard fn already carries is the induction skeleton, precomputed
by the language itself. Consequences, all ratified:

- **I2d (impgen) is promoted to standing infrastructure** and is the
  next slice: the imp ⊑ ISA legs are generated for the closed imp
  construct set, absorbing the fragment ladder's proof kit (WrapK,
  fuel algebra, clobber sets, the window kit). The standing rule
  extends the no-oneoff-generators ruling: NO PROOF FAMILY GETS A
  THIRD HAND INSTANCE — probe twice, then the generator ships with
  the family. The sha256 sibling (11k hand lines) is the measurement
  that priced this.
- **I2.5 (containers) and I3 (profiles) leave the critical path** —
  they land with or after the coverage arc, consumer-driven.
- **wasmgen/x86gen freeze at their landed extent** (kept green; the
  §5 absorption license is exercised now rather than after fragment
  parity — new coverage arrives only through imp, and the direct
  spec→ISA path stops growing).
- **The full-gap pathfinder milestone: examples/calc** as a proven
  (bin …) on silicon through the generic path — strings, lists, ADTs,
  recursion crossing the whole gap in one artifact.
- **The controlled-failure surface (MEMORY.md D8)** — overflow/OOM/
  stack as a Done-or-Fail cert conclusion plus a requirements-level
  `except` clause — is OPEN and resolves early in the coverage arc;
  imp's machine grows a reasoned Fail value (distinct from ITrap and
  fuel None) when it lands.

**RE-SEQUENCING (2026-07-14, with the §2a re-adjudication).** I2d-3
as previously scoped (recognizer + phase-machinery growth to absorb
the sha bodies) is CANCELLED: shape recognition synthesizes loop
invariants from memorized body shapes and can never be total over
hand-written imp — and §2a removes the need, since the invariants
existed only because v1 scalars could leave band. The next work is
the v2 migration ladder (§2a), then impgen rebuilt as the
structural walk over well-kinded imp; the sha ISA legs — the
original I2d-3 deliverable — then fall out of the rebuilt generator
run over the migrated sibling, with no recognizers anywhere. I2e
(the bins) follows the migration; the coverage arc opens on the
typed model. The landed impgen tiers stay green until their
migration slice retires them.


## 6a. Rung records

**I0 — the machine (2026-07-12).** models/imp/imp.shard landed:
IExp/IStmt/IFn/IProg over std/mem's Mem, exact Int ops with the
premised IWrap, the istmt/iwhile/istmts mutual fuel SCC (one unit
per entry, the wasm discipline verbatim; iwhile a NAMED member for
loop workers), icall/icall_mem denotations. All failure modes are
honest (traps ITrap, fuel None, arity/index None).

- **The probe** (examples/imp_probe.shard, corpus-pinned): 14
  ground claims — arithmetic, both if arms, loop, memory round-trip
  and framing, wrap, div, and the four honesty corners (div0, OOB,
  fuel-out, arity) all None. Memory probes close with std/mem's
  surface laws; everything else is (compute both).
- **The twins.** Straight-line (examples/imp_scalar.shard): six
  representative members incl. branching (case-on) and the divisor
  premise — all EXACT AND UNPREMISED except division, because imp
  scalars are unbounded Int; the wasm certs' range premises simply
  do not exist at imp. One IWrap demonstration pins the premise
  discipline (premises appear exactly where wrap is used). Loop
  (examples/imp_loop.shard): lp_fill (store direction) and lp_sum
  (read direction) with induction workers — the collapse is DIRECT
  because Mem is the shared substrate (the imp run constructs
  literally the spec's mem_set/mem_get terms; no byte reasoning, no
  wrap32 haves — the wasm workers' whole width-residue apparatus
  has no imp analogue). Fuel rides loopkit's lg_fuel tower inside a
  constant S^ reservoir; the reservoir is restored by the tower's S
  each iteration, so the re-entry fuel is exactly the IH's shape.
- **models/wasm/loopkit reuse**: the Int/Nat lemma kit (lg_fuel,
  lg_adv, lg_ne, lg_sub, lg_lo1, lg_shift) is wasm-free and served
  the imp workers verbatim; ONE new lemma (im_lt, the machine's
  strict store/load guard shape) was needed. imp is now the kit's
  SECOND SPEAKER — the meta/ graduation law fires; queued, not done
  mid-rung (moving the file touches wasm cert imports).
- **Gates**: 4 corpus targets added; 4 'check products (target
  'imp — proof gates only, per DI2); driver 45 products green;
  corpus FAIL-set unchanged.
- Proof-idiom notes for I1: workers must `(unfold iwhile lhs)` once
  before the stopped compute (the eval_loop precedent), and claims
  must state loop bodies via `(inline …)` so goals carry the
  literal statement list execution produces.

**I1a — the imp → wasm translator, straight-line class
(2026-07-12).** The first ISA leg opens hand-proven (the
pieces-before-wasmgen precedent: the proof shape stabilizes before
the generator mechanizes it):

- **models/imp/to_wasm.shard** — the bridge file, the one place
  both models import: expression trees compile postorder to stack
  code; result-expression-only fns become MkFunc values.
  Untranslatable forms (statements, ILoad, non-32 widths) refuse
  with None — no artifact rather than a wrong one.
- **The ties**: the translator's output is BYTE-IDENTICAL to the
  instruction sequences wasmgen produced directly for the same
  source fns (ground equalities against the pinned literals).
  Factoring the pipeline changed zero bytes.
- **The bridges (examples/imp_wasm_bridge.shard)**: call_fn over
  the translated func = icall over the imp twin. The width story
  landed exactly where §1 said it would: unwrapped exact ops carry
  range premises HERE (once per target), division carries only its
  divisor guard (neither side wraps), and the IWrap twin bridges
  UNPREMISED — imp's explicit wrap and wasm's inherent wrap are the
  same mod, so dropping the IWrap at translation is unconditionally
  faithful. That one claim (imp_w_add1w) is the neutrality thesis
  in a single unpremised equation.
- **The composition (wcomp_lg_add1)**: spec ⊑ imp (I0) chains with
  imp ⊑ wasm (this slice) into spec ⊑ wasm in two rewrites — the
  statement the direct generator's cert makes, reached through the
  neutral machine. The factored tower closes end to end.
- Gates: 2 corpus targets, 2 'check products; driver 47 green;
  corpus FAIL-set unchanged. Remaining in I1: statements + loops
  (I1b — the IIf/IWhile translation onto Block/Loop/Br and the
  loop-worker bridge inductions), then the x86 leg (I1c), then
  mechanization into a cert-emitting generator.

**I1b — statements + loops, the wasm leg (2026-07-12).** The
translator covers the full v1 statement tier and the loop bridge
inductions land the M×N kill:

- **The statement tier (to_wasm.shard)**: iw_stmts — one structural
  walk (the kernel's (struct …) measure accepts nested-match
  descent, probed before building). ISet→LocalSet,
  IStore→I32Store8, ILoad→I32Load8U, and the two SELF-CONTAINED
  label encodings: IIf → Block(Block(⟦c⟧ BrIf 0 ⟦els⟧ Br 1) ⟦thn⟧),
  IWhile → Block(Loop(⟦c⟧ I32Eqz BrIf 1 ⟦body⟧ Br 0)). Every emitted
  branch targets the encoding's own blocks, so nesting composes with
  no depth adjustment. imp2w_fn now takes any v1 body (statements ++
  result expression); iw_out is the loop-exit alignment adapter
  (INorm ↔ OBr 0, ITrap ↔ OTrap).
- **The ties**: tie_sel (imp_wasm_bridge.shard) — the IIf encoding
  is byte-identical to wasmgen's gate literal; tie_fill/tie_sum
  (imp_wasm_loop_bridge.shard) — (imp2w_fn (il_*_fn)) = (Some
  (lp_*_func)), stated against the generated cert file's own named
  funcs. The IWhile encoding reproduces wasmgen's loop shape byte
  for byte. (wasmgen's clamp2 rides a temp-local let template, so
  imp_w_clamp2 instead demonstrates the other face: the imp path
  certifying, unpremised, an artifact the direct generator never
  produced.)
- **THE LOOP WORKER BRIDGES (iww_fill/iww_sum)**: eval_loop over
  the translated body = (iw_out st (iwhile … over the imp body)),
  by induction on the counter — and the induction NEVER MENTIONS
  THE SPEC. The wasm side's whole width-residue apparatus (the
  wrap32 haves: counter, pointer, accumulator) discharges against
  the imp machine's exact ops inside the one induction; loopkit +
  im_lt served verbatim, zero new lemmas. This is the M×N kill
  made concrete: a new loop program needs its imp worker (spec ⊑
  imp, target-free) and this bridge (imp ⊑ wasm, spec-free) — no
  per-program spec ⊑ wasm worker ever again. Proof-idiom note: the
  workers share ONE slack tail c across both machines' towers so
  the IH binds fully from the left side; (inline …) resolves
  against the file's own nullary fns only, so bridge files carry
  local body copies pinned by tie_*body claims.
- **The denotation bridges + compositions**: imp_w_fill/imp_w_sum
  (call_fn_mem = icall_mem, independent slack tails restored, proof
  = worker rewrite + imp-side worker rewrite both sides);
  wcomp_lp_fill/wcomp_lp_sum — lowered_lp_*'s exact statements,
  reached through the neutral machine in two rewrites.
- Gates: 1 corpus target, 1 'check product (driver 48); corpus
  FAIL-set unchanged at 57. Remaining in I1: the x86 leg (I1c),
  then mechanization into a cert-emitting generator once the proof
  shape has a second target's confirmation.

**I1c — the x86 leg (2026-07-12).** The second target confirms the
proof shape; rung I1 is complete:

- **The translator (models/imp/to_x86.shard)**: the register machine
  gets a PATTERN tier, not to_wasm's postorder scheme — imp locals
  map to their SysV homes (args then extras on rdi rsi rdx rcx r8
  r9; xargs zeroes the extras' homes, matching imp's zeroed extras;
  >6 refuses), ISet compiles to the in-place / mov+bin / RAX-load-
  scratch shapes tools/x86gen emits (mov+bin fenced against the
  right operand reading the dst), conditions FUSE (CEq/CLtU/CLeU —
  no comparison materialization), results compile left-spine into
  RAX with one R10 right-compound level. Same self-contained label
  encodings, with the fused guard (XBrIf (CEqz home) 1 — wasm's
  I32Eqz+BrIf pair is one instruction here).
- **The width story, second target**: (IWrap 64 e) absorbs
  unpremised (imp_x_add1w64 — x86's native width), IWrap 32 refuses;
  the mirror image of the wasm leg. The unwrapped bridges carry the
  SAME premise shapes in THIS machine's modulus — 2^64 where the
  wasm leg said 2^32, from the same imp twin. Division refuses in
  v1 (XDivU's rdx:rax preamble is named growth).
- **The ties**: add1/add/mix reproduce x86gen's literals byte for
  byte; the sum loop ties EXACTLY against the generated artifact
  ((imp2x_fn (il_sum_fn)) = (Some (xi_bsum_func)), epilogue
  included); the fill loop is the generated loop body (cited by
  name) plus imp's honest XMovRI RAX 0 result — x86gen's Mem-output
  template leaves rax accidental. sel/clamp2 don't tie (x86gen's
  templates deposit through the scratch pool; imp spells the extra
  local) — they ride their own bridges, the wasm-leg clamp2
  precedent.
- **The loop worker bridges (ixw_fill/ixw_bsum)**: the same
  spec-free inductions, against the register machine. The one new
  ingredient x86 adds — the RAX load-scratch residue in the exit
  register file — rides the ix_out adapter's ra argument (0 for the
  storing loop; the loopkit's xlg_last shape for the reading loop,
  quantified at entry exactly as the generated worker quantifies
  it, so the IH binds the re-entry scratch by matching).
  Proof-idiom note: xlg_last must sit in the stopped computes' stop
  set — left free, the compute unfolds it into a stuck match on the
  induction variable and the IH's folded spelling never matches.
- **Bridges + compositions**: xcall_fn_mem = icall_mem
  (imp_x_fill/imp_x_bsum); xcomp_lp_fill/xcomp_lp_sum and
  xcomp_lg_add1 end at THE SAME spec fns (lp_fill, lp_sum, lg_add1)
  the wasm compositions end at — one spec, one imp twin, one imp
  pin, two silicon-bound targets, and only the imp ⊑ ISA legs were
  written twice. The M×N kill, demonstrated across the full matrix.
- Gates: 3 corpus targets, 3 'check products (driver 51); corpus
  FAIL-set unchanged at 57. Remaining in I1: mechanization into a
  cert-emitting generator (both targets' proof shapes now
  stabilized) — scheduled after the flagship rungs exercise the
  hand era further.

**I2 opening rulings (2026-07-12).** The sha256 zero-heap flagship's
scoping pass, ratified: **(1)** shift AND rotate opcodes enter the
vocabulary (the wasm model's own "wait for a consumer" gate fired);
**(2)** "one region" at I2 means STATICALLY CARVED WINDOWS — the
region cancellation is a cert shape (scratch quantified arbitrary,
output windows framed disjoint), NOT machine syntax; dynamic region
alloc/death ops defer to MEMORY.md rung 3+ (deliberate deviation
from the I0-era machine-header note, now corrected); **(3)**
byte-spelled big-endian word access in v1 (wide loads are a later
perf rung). Slicing: I2a vocabulary → I2b word kit → I2c twin +
cancellation cert → I2d ISA legs → I2e ./sha256sum + V8.

**I2a — the shift/rotate vocabulary (2026-07-12).** Three layers,
one slice, both differentials green:

- **imp**: IShl/IShr (exact kernel-prim semantics; negative counts
  trap — the honest corner) and (IRotr W a k), width-parametric on
  the IWrap precedent but BAND-spelled: rotation is a bit op, so
  its containment is the bit-mask idiom — which is also the
  sha-class specs' own m32 spelling, so spec ⊑ imp rotation aligns
  syntactically. wrap/mod stays the arithmetic idiom.
- **wasm**: BShl/BShrU/BRotr (counts mod 32; BRotr band-spelled to
  match), encodings 0x74/0x76/0x78; 24 new differential vectors
  (count-masking boundary 0/31/32/33, high-bit wrap, all-ones) —
  V8: 173 agree, 0 disagree.
- **x86**: XShlI/XShrI (immediate-count only — the CL register-count
  quirk stays out until a consumer; counts mod 64 = the hardware's
  6-bit mask), REX.W C1 /4|/5 ib encodings + 48 silicon vectors
  including the count-65≡count-1 masking witness (Opus-delegated
  per the standing split; conclusions verified by re-running the
  differential first-hand) — silicon: 82 agree, 0 disagree. No
  32-bit-view rotate instruction yet: to_x86 lowers (IRotr 32 …)
  as the mov/shr/shl/or/and composition through the R11 scratch;
  the single-instruction ror rung is named growth for the perf
  pass.
- **The pins**: shift bridges UNPREMISED on both targets; the
  rotation bridge UNPREMISED on wasm (band meets band — the
  machine-width family is free on its native target) and carrying
  exactly ONE wrap64_id premise on x86 (the left-shift leg of the
  composition). The width mirror's third data point.
- **Found in passing (Opus, verified first-hand)**: the x86 silicon
  differential had been silently degraded since b3954ab (2026-07-09)
  — the XCASE parser moved to a 4-slot wire but the emitter stayed
  at 3, so all 27 XCASE lines scored unparseable. Fixed emitter-side
  in x86_diff_run.shard; the differential's scored set went from 7
  to 82 lines.
- Gates: no new files (all edits to existing corpus targets); driver
  51 green; corpus FAIL-set unchanged at 57.

**I2b — the sha word kit (2026-07-12).** std/sha256 grows its imp
spelling — the manual-spelling-target vision made concrete:

- **std/sha256/sha256.imp.shard** — the module's imp dialect SIBLING
  (the .wasm.shard/.x86.shard convention; in-module residence
  resolves the privacy question: the impl-file import plus the
  module use-clause gives a sibling full private visibility, so the
  public surface stays two fns). Contents: IExp BUILDERS, one per
  spec word fn (ie_rotr32/ie_m32/ie_ch/ie_maj/ie_bsig0/ie_bsig1/
  ie_ssig0/ie_ssig1), parametric over source local indices — the
  vocabulary the round twin composes from — plus per-fn ALIGNMENT
  PINS. Every pin is (compute both)/refl: IRotr's band-spelled
  containment IS the spec's m32 idiom, so builder and spec compute
  to the same tree. Spelling drift now fails at the kit, not deep
  inside the round induction.
- **The general-ISet fallback (to_x86)**: the direct patterns moved
  into ix_set; on pattern refusal ix_stmts compiles the expression
  into RAX via ix_res and deposits into the home. Statement-position
  trees up to one R10 level (ch/maj shapes) are now expressible;
  every existing tie stayed byte-identical (patterns fire first).
  Pinned end-to-end by the ch-shaped it_chset_fn: the x86 tie's
  8-instruction literal and an UNPREMISED bridge — the sha word
  class is pure bitwise, exact on every machine.
- **Build**: std/sha256/mod.build.shard opens (products-only — the
  module's 'check imp gate; pinlib/artifact products arrive with
  I2d/I2e), third entry in build_entries.
- Gates: driver 52 products green; corpus FAIL-set unchanged at 57.
- I2c note (recorded at scoping): the round/schedule/block twins and
  the cancellation cert live in this same sibling file (or a second
  one beside it); the layout question to resolve at I2c's opening is
  the round loop's LOCAL BUDGET on x86 — state(8) + counter + temps
  + running pointers lands at 12–13 against 12 available homes
  (15 GPRs minus the RAX/R10/R11 scratch trio); the outs are
  accumulate-t1-in-h's-slot, two-pointer fusion, or a 13th home by
  reducing scratch pressure.

**I2c-1 — the round tier (2026-07-12).** The compression loop's imp
twin lands in the sibling file, with the recorded budget question
resolved at its opening:

- **The layout, at exactly 12 = 12.** Locals 0–7 the working state,
  8/9 the K/W running pointers, 10 the counter, 11 ONE shuttle temp.
  Two observations close the budget without W/K fusion or a 13th
  home: T1 accumulates IN H'S SLOT (old h dies into the T1 sum
  first, so h's slot IS the temp), and the state rotation is ordered
  so every original is read through its not-yet-overwritten slot.
  Every statement shape was verified against to_x86's CURRENT
  pattern tier before the twin was frozen (in-place bin/mask ops,
  add-with-load, the rotation set pattern, one-R10 general trees):
  the x86 leg's only growth at I2d is ix_home 6 → 12.
- **The round body** (isha_round_body, 51 statements): forward
  Horner word loads (t := t*256 + m[p], p += 1 — in-pattern on both
  targets), Ch as the pinned kit tree, Σ trees built right-nested in
  t so the xor spines land the spec's association exactly, masks
  wherever the spec masks, the 8-cycle rotation broken by t (t ends
  holding a').
- **The single-pass lemma** (isha_round_pass): one body run on
  symbolic state = one sha_round on (wget m kp)/(wget m wp), both
  pointers +4, counter −1, a' in the shuttle, MEMORY UNTOUCHED. The
  proof walks the body discharging the 16 load guards, then aligns
  by exactly four linear identities: Horner → word_be per word
  (ish_be) and the two sum re-associations (ish_sum5/ish_sum3) —
  everything else is refl-grade because the band-spelled rotation
  trees ARE the spec's trees (the I2b thesis paying off at scale).
- **The rounds worker** (isha_rounds_w): induction on k lands
  iwhile = sha_rounds over (wlist m kp k)/(wlist m wp k) — the
  window-content reading is Nat-indexed (wlist) so the induction
  unfolds it structurally, pointers advance by lg_advk stride 4, and
  the shuttle's k-dependent final value rides the rt_last selector
  with a two-case shift lemma (ish_rt_shift). Memory returns
  UNTOUCHED for any k — the compression loop is a pure reader: the
  first half of the region-cancellation story, already in theorem
  form.
- **Proof-system findings** (recorded for the arc): plain
  (rewrite … true ()) rewrites ALL occurrences but rewrite-with
  rewrites the FIRST only — big-tree rewrites repeat per occurrence
  (the T1 tree occurs three times: a'-slot, e'-slot, shuttle); a
  cited lemma with a premise-only binder needs an explicit (inst …)
  (the type gate names it a dangling pivot); and the stuck-match
  unfolding gotcha (xlg_last precedent) resolves cleanly as ONE-STEP
  UNFOLDING LEMMAS (wlist_s/sha_rounds_s/lg_advk_s/rt_last_s, proven
  by unfold+reduce/refl) cited through the all-occurrence rewrite
  form, with the kit fns themselves in every stop set.
- Gates: fast-engine 93/0 on the sibling; driver 52 products green;
  corpus FAIL-set unchanged at 57.
- REMAINING I2c: I2c-2 the schedule tier (words16 copy + sched_ext
  extension loop — the write direction: byte-store/word-readback
  roundtrip + window framing enter here), I2c-3 block walk + padding
  + the digest-readback theorem over arbitrary scratch (K-window
  content as a PREMISE, discharged at I2d/I2e by target data
  segments).

**I2c-2a — the write-direction kit (2026-07-12).** The schedule
tier's law families, landed ahead of its loops (the round tier only
read; the extension loop writes at the frontier and re-reads below
it):

- **wput** — the big-endian word store, spelled as EXACTLY the
  mem_set chain the twin's store block will compute: last byte first
  at p+3, the value shuttled up by nested >>8 steps. The nesting is
  deliberate — it keeps the whole roundtrip inside the literal
  vocabulary (shr8_div/mask_byte), so pow2 never appears on any
  surface.
- **ish_be_recomp** — the arithmetic core: the big-endian byte sum
  of the three-level euclidean quotient chain rebuilds any word
  below 2^32. The top byte is the third quotient ITSELF (mod_unique
  at quotient 0) rather than a fourth mod level — that shape keeps
  every farkas certificate rationally sound (a fourth level forces
  integer tightening mid-sum, which the certificate calculus
  correctly rejects). Facts materialize as lemma-fed haves citing
  the kernel's euclidean characterization (ediv_mod_id/mod_lo/
  mod_hi), closed by one paired certificate.
- **isha_wput_get** — the roundtrip: reading back a just-stored
  in-range word is the identity. get_set_byte/get_set_other resolve
  the four reads through the four-store chain (six distinct-offset
  lemmas feed the framing premises); shr8_div and mask_byte convert
  the byte trees into euclidean vocabulary; ish_be_recomp closes.
- **ish_wget_set_other / ish_wlist_set_above** — window framing: a
  store at or above a window's end is invisible to the window's
  words (per-word from get_set_other ×4; per-window by induction
  with the stride-4 premise steps).
- New finding for the arc: the checker's farkas rejection prints the
  FULL SLOT TABLE (every fact in premise order with its normalized
  linear form) — certificates are now derivable directly against
  the table rather than reconstructed from the claim text.
- Gates: fast-engine 113/0 on the sibling; driver 52 products green;
  corpus FAIL-set unchanged at 57.
- REMAINING I2c-2: (b) the words16 copy loop + its worker, (c) the
  extension loop twin + sched_mem (the memory-level schedule
  recursion) + its pass/worker, (d) the list bridge (sched_mem's
  readback = sha_sched's srev_acc/sched_ext shape — the reversal
  algebra, pure list work).

**I2c-2b — the word-copy loop (2026-07-12).** words16's machine half,
end to end — and the first slice where the stabilized machinery just
composed (every claim closed on the first checker run):

- **Word grain, not byte grain.** The copy loop moves one WORD per
  iteration: a Horner load (the round tier's proven shape) and a
  down-walk store that computes EXACTLY wput's mem_set chain. So the
  pass lemma's memory effect is literally (wput m wp (wget m p2)) —
  after the guard ladder and the flatten haves, two all-occurrence
  RL-rewrites through defining-equation lemmas (wget_be, wput_sets)
  FOLD the machine's computed trees back into the kit's vocabulary,
  and the worker aligns against the word-grain denotation copy_wmem
  with no byte-level reasoning anywhere.
- **The zero-in/zero-out invariant.** The body re-zeroes its three
  scratch slots at each iteration's end, and the worker states the
  initial scratch as literal zeros — so the final locals are UNIFORM
  in k and no iteration-dependent residue selectors exist (the
  rt_last lesson, resolved by construction; three statements of
  honest cost, the perf pass's business later).
- **The readback theorem** (ish_copy_read): reading the copied
  window gives the source words, premise = source entirely below
  destination. Rides the 2a kit exactly as designed: head =
  copy-below framing + the wput/wget ROUNDTRIP (fed by the new
  wget range lemmas); tail = IH + window-through-wput framing.
- Gates: fast-engine 142/0 on the sibling; driver 52 products green;
  corpus FAIL-set unchanged at 57.
- REMAINING I2c-2: (c) the extension loop twin + sched_mem + its
  pass/worker, (d) the list bridge to sha_sched (the reversal
  algebra).
- I2c-2c design note (settled; the idiom pinned by ish_phase_pin):
  the extension body's σ-trees wrap each loaded word several times
  across the four store positions, so a monolithic pass lemma would
  pay ~48 occurrence-chased conversion rewrites — and the walk
  offers no stick point between a word's last load and its σ
  statements, so mid-walk folding cannot intervene. The resolution
  is the CONTINUATION-PHASE IDIOM: split the body at fold points
  into phase lemmas of the shape istmts(S^N fl, ⟨literal spine ·
  tail⟩, lc, m) = istmts(S^(N−k) fl, tail, lc′, m′) — the spine
  written literally with a quantified tail (istmts sticks at the
  symbolic tail on both sides; fuel steps by exactly the spine
  length; pinned closing by compute/refl). Each phase folds its own
  word at single occurrence. This is also the block tier's
  composition machinery (three sequenced loops = three phase
  rewrites).

**I2c-2c — the extension loop (2026-07-12).** The schedule-extension
twin, built exactly to the design note — the continuation-phase idiom's
first full outing, and it composed perfectly (the pass and worker both
closed on their first checker runs):

- **The composition probe** (ish_phase_comp): before building, two
  chained pin-rewrites validated the one mechanism the pin alone had
  not — a phase citation matching the S^ residue fuel a previous phase
  rewrite leaves behind. The rewrite result lands already
  tower-normalized, so phases chain with no fuel restatement.
- **Denotations**: sched_w = sched_ext's word recurrence read through
  the memory window ending at p (σ1(W[t−2]) + (W[t−7] + (σ0(W[t−15]) +
  W[t−16])), masked); sched_mem = the stride-4 store recursion. The
  fold lemma sched_w_fold states the unfolded band-spelled tree (σ and
  wget spellings kept folded via compute-with-stops), cited
  right-to-left once at the pass's end.
- **The body** (isha_ext_body, 79 statements): pointer drop to
  wp−64; four Horner loads (W[t−16] into the accumulator, the rest
  through the shuttle — W[t−15] adjacent for free, jumps +28/+16 to
  W[t−7]/W[t−2]); σ trees right-nested in u; THE ACCUMULATOR BUILDS
  RIGHT-TO-LEFT, so the finished tree IS the spec's association (zero
  re-association — the round tier needed four linear identities, the
  extension loop needs none beyond Horner→BE per word); the down-walk
  store at the frontier; advance/re-zero/count. Raw IRotr ctors, no
  builder fns — the spine is pure constructors, so no fold/unfold
  ambiguity exists at phase-match time.
- **Eight phase lemmas** (fuel k+5 → 5, the slack window slides):
  ptr / load-into-acc / load-into-shuttle (REUSED ×3 at three offsets)
  / σ0+sum+jump / sum+jump / σ1+sum+mask / store / fin. Load phases
  carry generic bounds premises discharged at the pass level by
  offset-tree lemmas (ish_xlo64…ish_xhi8, all cert (1 1)); σ phases
  state (ssig0 t)/(ssig1 t) folded and close by compute both — the
  band-spelling thesis again, iexp's IRotr tree = the spec's rotr32
  unfolded, refl-grade.
- **The pass** (isha_ext_pass): ten phase rewrites chained on the
  folded istmts application, three pointer flattens between loads,
  one sched_w fold at the end. Memory effect = (wput m wp (sched_w m
  wp)) — reads before the write, from the original memory, exactly
  sched_mem's step.
- **The worker** (isha_ext_w): the loop IS sched_mem, by the copy
  worker's induction verbatim (frontier premise 64 ≤ wp stepped by
  ish_ext_wstep; write premise by ish_w4shift; fuel 88 = pass 84 + 4).
- **NEW PROOF GOTCHA (the slice's one debug):** rewrite-with does NOT
  rewrite under match-branch binders; plain rewrite does. A phase
  stuck at its symbolic tail is an OPEN match, so in-phase folds
  (ish_be, wget_be, wput_sets) must be plain all-occurrence rewrites —
  premise-free lemmas bind by matching there just fine. Guard
  discharges are unaffected (guards stick in scrutinee position, where
  rewrite-with works). At the PASS level the goal stays a folded
  istmts application, so both forms work between phases.
- Gates: fast-engine 166/0 on the sibling; driver 52 products green;
  corpus FAIL-set unchanged at 57.
- REMAINING I2c-2: (d) the list bridge to sha_sched (sched_mem's
  readback = the srev_acc/sched_ext shape — the reversal algebra,
  pure list work).
**I2c-2d — the list bridge (2026-07-12).** sched_mem's readback IS
sched_ext — the reversal algebra closed, and the bridge itself plus
every stage-B lemma landed on their first checker runs:

- **The downward reader.** wrev m p k = Cons (wget m (- p 4)) (wrev m
  (- p 4) k2): the reversed window's head is the newest word, so one
  sched_mem step is one Cons — exactly srev_acc's algebra. THE BRIDGE
  (isha_sched_bridge, PREMISE-FREE — the denotation level has no
  bounds story; the machine worker carries the guards):
  wrev (sched_mem m p k) (lg_advk p 4 k) (wn_add k W) =
  sched_ext (wrev m p W) (int_of_nat k), with W the window count
  spelled LITERALLY as sixteen nested S around a quantified j — wrev
  provably unfolds 16 deep while the tail stays abstract, and the
  17-S tree needed by the induction (window grows) is SYNTACTICALLY
  both (S W16j) and W16(S j), so the IH binds j := (S j) by matching
  with zero count algebra beyond one wn_add succ-commute rewrite.
- **The reversed-index mirror**: sched_ext's snth 1/6/14/15 of the
  downward window resolve to wget at p−8/−28/−60/−64 — sched_w
  verbatim (ish_sw_snth; a 15-have flatten cascade normalizes the
  nested (- (- p 4k) 4) trees).
- **The cons step** (ish_wrev_put_s): head = the isha_wput_get
  roundtrip (value bounds only), tail = downward framing
  (ish_wrev_wput_above, the ish_wlist_wput_above mirror).
- **The stored-value bounds** (ish_sw_lo/ish_sw_hi, premise-free):
  band_lo/band_le_r need the masked sum nonnegative, which walks the
  σ trees down to per-count shift nonnegativity. pow2 is opaque
  outside std/bits, so the generic shr_pow2/shl_pow2 gateway is
  unusable at ground counts from a consumer — instead the kernel
  recurrences (bshr_s/bshl_s + _z) build LADDERS: ish_shr1..19_lo
  and ish_shl1..25_lo, each rung ~8 lines citing the rung below,
  with ONE divisor-2 euclidean lemma (ish_ediv2_lo) feeding every
  shr step. No std/bits surface growth needed.
- Counts ride a transparent in-sibling wn_add (std/nat is opaque —
  add_nat exports no lemmas). sched_ext steps by a premised
  one-step unfolding lemma (sched_ext_s; guard resolved via
  int_of_nat_succ + ish_le_ne, fuel residue via lg_sub).
- Gates: fast-engine 235/0 on the sibling; driver 52 products green;
  corpus FAIL-set unchanged at 57; V8 173/0; silicon 82/0.
- I2c-2 COMPLETE. REMAINING I2c: I2c-3 block walk + padding + the
  digest-readback theorem — needs one more list glue (the upward
  reader wlist as srev of the downward wrev, for the rounds worker's
  window ↔ sha_sched's final srev_acc), then the three-loop
  composition via the continuation-phase machinery.

**I2c-3 opening — the window glue (2026-07-12).** The weld vocabulary
between the workers' windows and the spec's reversal algebra, every
lemma first-try:

- **wlist_acc** (the upward reader with an explicit tail; the plain
  reader is its Nil instance) + the SNOC lemma (one more count moves
  the LAST word into the tail, its address spelled (lg_advk p 4 k)).
- **THE GLUE (ish_wrev_srev)**: srev_acc of the downward window read
  IS the upward read — stated with the frontier spelled (lg_advk b 4
  k) over the BASE as primary variable, which makes the IH bind
  directly and reduces the pointer algebra to one step-down lemma
  (ish_advk_m4); the inverse-lemma route the design note anticipated
  is unnecessary in this orientation.
- **ish_advk_split**: pointer advance splits over the count sum
  (lg_advk z s (wn_add a b) = advance twice) — the block tier keeps
  every frontier in lg_advk vocabulary (copy's exit = ext's entry =
  (lg_advk b 4 16); the 64-window frontier = the split), so ground
  instantiation never flattens +4 towers. Z-form unfold lemmas
  (wn_add_z, ish_advk_z) added — stopped fns don't reduce their own
  Z-redexes.
- Weld analysis (recorded for the block tier): at ground counts the
  mirror direction (wrev = srev of wlist) needs NO new machinery —
  cite the glue instance as a have, rewrite it backwards under
  srev_acc, and the double-srev computes away (ground counts walk
  structurally). The weld theorem is proof-steps-only from the landed
  kit: wlacc_nil + glue + bridge + copy-readback + double-srev, with
  int_of_nat at ground counts aligned by succ/zero lemma rewrites
  (opaque fns do not ground-compute).
- Gates: fast-engine 244/0; driver 52 green; corpus FAIL-set
  unchanged at 57; V8 173/0; silicon 82/0.
- **THE SCHEDULE-WINDOW WELD (isha_sched_window), landed same day**:
  copy 16 words from the source + extend 48 at the frontier, and the
  64-word window reads back as EXACTLY the spec's schedule shape —
  srev_acc (sched_ext (srev_acc (wlist m src 16) Nil) 48) Nil — over
  the source words, premise (le (+ src 64) b) only. Proof exactly as
  analyzed (glue → count/frontier alignment haves → bridge →
  int_of_nat alignment → the double-srev mirror → copy readback).
  NEW GOTCHA: ground Nat REPRESENTATIONS can diverge — (S^ 64 Z)
  normalizes to the raw S-tower while wn_add's evaluation PACKS its
  ground result; a one-sided compute leaves TOWER = 64 and refl
  refuses. Ground-Nat equality haves must (compute both) so both
  sides normalize through the same path.
- Gates: fast-engine 247/0; driver 52 green; corpus FAIL-set
  unchanged at 57; V8 173/0; silicon 82/0.

**I2c-3 — the byte crossing (2026-07-12).** words16 over the memory's
byte readback IS the word-window read (ish_words16_read): the message
block enters the schedule story as bytes (std/mem's mem_read), and
the copy loop's Horner words are exactly words16's word_be groups.

- The byte count rides a transparent quadrupler (wn_q: four S per
  word), so mem_read_s peels four bytes per induction step; the
  premised one-step unfolding words16_s mirrors sched_ext_s.
- sdrop4_cons: the spec's sdrop checks the list match BEFORE the
  count guard, so (sdrop 4 spine·TAIL) sticks OPEN on a folded tail —
  a two-ctor-case lemma (induct, both cases compute) folds it; the
  same shape will recur for any stake/sdrop/snth-guard-behind-match
  spec fn meeting a symbolic tail.
- **REWRITER FINDING (flagged for review): ground packed Nat and Int
  literals are the SAME ATOM to the rewriter.** A premise rewrite of
  Int 16 (fuel position) also rewrote a packed Nat 16 inside a
  Nat-sorted argument, producing the ill-sorted (wn_q (int_of_nat
  …)) — which the checker then computed without complaint. At
  minimum a proof-authoring hazard (ground-literal rewrites can cross
  sorts); possibly a type-gate gap on rewrite results. The ground-16
  corollary was cut pending a rep-story pin at the block tier;
  the generic crossing lemma is unaffected (no ground literals).
- Also confirmed: (S^ k Z) normalizes to the raw S-tower on some
  paths and to the packed literal on ceval paths — ground-Nat
  spelling alignment needs compute-both (the weld's gotcha, now seen
  from both sides).
- Canon catch: wn_q's Z-arm body spelled the ground Nat as the ctor
  (C6 requires the literal 0) — the canon_std gate flagged it, and
  shardfmt does NOT (format ≠ canon); fixed to (Z 0). The corpus
  FAIL-set diff is the only gate that sees this class.
- Gates: fast-engine 251/0; driver 52 green; corpus FAIL-set
  restored to the 57 baseline after the C6 fix; V8 173/0;
  silicon 82/0.

**I2c-3 — the sequencing pin (2026-07-12).** Machine-level loop
sequencing validated end to end on a mini-block (ish_blk_seq): two
copy-loop worker citations + straight-line resets in ONE statement
spine — the first composition of loop workers inside istmts. The two
block-tier pins, now settled:

- **PIN A — fuel chaining.** The block runs on ONE literal tower
  (S^ N d). A loop worker consumes NOTHING from the spine's fuel
  (istmts hands the same f2 to head and tail), so each loop head
  needs only its residual tower RE-SPELLED as the worker's
  (S^ 40 (lg_fuel k slack)) shape — a have proven by compute both
  (lg_fuel ground-collapses; the slack tower absorbs the excess).
  The first loop's re-spelling happens UP FRONT on the initial fuel,
  where it occurs exactly once, so that loop head lands on the
  worker's shape by construction; later loops re-spell mid-proof at
  spellings known by construction from the previous have. Nested
  continuation-fuel spellings CANNOT work — the tail inherits the
  loop's unconsumed reservoir, so the next loop's lg_fuel node would
  sit under leftover literal S's and never match.
- **PIN B — ground rep: PACKED EVERYWHERE.** ceval normalizes the
  args of STOPPED applications — the probe's first run showed
  (lg_fuel (S (S Z)) …) packing to (lg_fuel 2 …) inside the stopped
  node while the locals alignment had introduced the tower spelling;
  the worker's k then unified inconsistently (packed from fuel,
  tower from locals) and the citation failed. Resolution: ground
  counts are SPELLED packed claim-side (C6's source form); towers
  appear only where structural matching needs them (induction
  windows). The rep bridge (= (int_of_nat N) (int_of_nat (S^ N Z)))
  holds by compute both — the arg packs — and feeds the succ-ladder
  (ish_i1/ish_i2, packed statements; ish_i16/ish_i48 stay
  tower-spelled for the weld).
- Count alignment rewrites the FULL 12-local list — a compound
  pattern with exactly one occurrence — never the bare literal (the
  byte-crossing rewriter finding stands: packed Nat and Int ground
  literals are one rewrite atom).
- Farkas discharge certs must cover ALL premise slots including
  accumulated cut haves (they join the premise rows in order);
  non-linear rows (fuel/list equations) take multiplier 0.
- Worker range premises discharge by ish_iN rewrite + compute
  (grounding lg_advk inside the subgoal, where nothing is stopped) +
  premise-rewrite or arith.

Remaining I2c-3 ladder: the block body fn (copy 16 → ext 48 →
H-load → rounds 64 → h8-add/store finish; bases symbolic with range
premises, K-window content as a wlist premise discharged at I2e by
the data segment) → the block walk theorem welding the three
workers via this pin + finish phase lemmas → padding + the digest
readback.

**I2c-3 — THE BLOCK WALK (2026-07-12).** isha_block_walk: the
329-statement block body — three loops and every phase between —
runs end to end at the machine and lands on spec shapes. From
locals src/wb over any memory (premises: 0 ≤ src, src+64 ≤ wb,
wb+544 ≤ 65536), the body computes Some(INorm ⟨the rounds output
through the eight H8 projections, scratch zeroed⟩, (h8add_mem m2
(+ wb 512) rounds_out)) where m2 = the copy+ext memory and
rounds_out = sha_rounds over the K/W windows of m2 with the H
window's words as initial state.

- The relative layout is the flagship's: W at wb, K at wb+256, H at
  wb+512, message at src below W. wb parks in local 3 (pass-through
  for copy and ext); every mid-body pointer derives from it by
  ground arithmetic, and after the rounds loop the advanced kp
  local IS the H base — the finish walks it with zero re-derivation.
- The body: setup (park + count) → copy 16 → ext frame resets →
  ext 48 → pre-rounds phase (wp/kp/H-pointer from the park) →
  H-load (8 Horner lanes via pointer 10, ish_blk_ld0-7) → counters
  → rounds 64 → finish (8 lanes: ish_ext_ph_ldt gather reused
  verbatim + ish_blk_fs0-7 add/wrap/scatter) → 4 zero-resets.
- h8add_mem: the finish's memory effect as one spec-side fn —
  eight sequential read-add-wrap-write lanes with the machine's
  CUMULATIVE address trees, so the walk's two sides meet at one
  normal form with no flattening.
- The stuck-record trick: the rounds worker returns locals through
  h8_locals of a STUCK sha_rounds record — the finish phases need a
  literal list. The whole proof wraps in a single-ctor case-on of
  the rounds output; hyp 0 rewrites both sides, h8_locals/rt_last
  and the RHS projections compute over mk_h8, and the case binders
  carry the finish.
- The sequencing pin at scale: one S^400 literal tower; per-loop
  reshape haves (fuel = 40+16+340 / 88+48+256 / 60+64+174), an
  lg_fuel ground-collapse have after each loop, phase citations
  chain by pure tower matching, computes stop istmts only when the
  next citation is a phase (loops want the peeled iwhile; phases
  want the folded application).
- 57 farkas discharges, generated positionally: slots = G + the
  three goal premises + one row per accumulated cut have (the
  case-on hypothesis never joins the rows); every discharge is one
  of two shapes (nonneg-side G+M0+M1, range-side G+M2).
- (inline …) does NOT nest (documented; reader.shard §expansion) —
  the block body pastes the three loop bodies literally; the
  extraction must strip ;;-comments (parenthesized comment
  fragments read as statements — "missing 3 ')'" from an
  apostrophe was the tell).
- The one non-cert failure: the walk's computes evaluated ie_ch/
  ie_rotr32 IExp-builder calls inside the round body's AST while
  the worker's pattern keeps them folded — the builders join every
  walk stop list (the workers' own proofs already did this).

**I2d-1 — impgen: the scalar/branch tier mechanized (2026-07-12).**
tools/impgen lands as standing infrastructure (the redirection's next
slice): for every nullary IFn pin in a target file, the generator
emits the per-target TIE and BRIDGE certs mechanically — the shapes
the I1 hand era stabilized, now stamped by code.

- THE FACTORING: the translators stay the single source of
  translation truth — impgen LOADS models/imp/to_wasm / to_x86
  through meta/invoke (the kernel as a loading library) and RUNS
  them on the pin values; the tie literal is the spelled result
  (meta/spell's sp_e over a qname table harvested from the value
  itself). Nothing about translation is re-derived in the tool.
- The generator owns the CERT TEXT: the premise discipline (wrap
  pairs per exact-op node in evaluation order; operands without a
  free variable compute away and are priced free; divisor guards;
  §6j condition-relative premises via cond_wrap), the fuel towers
  (depth bounds computed structurally — all three machines hand the
  same reservoir to head and tail, so cost is a max — plus slack;
  surplus fuel rides the symbolic tail), and the proof skeletons
  (compute both; wrap-event haves citing wrap32_id/wrap64_id;
  case-on trees mirroring the pin's IIf structure with the hand
  files' arm template and p/w have naming).
- Validation: examples/impgen_wasm_out.shard (11 ties + 11 bridges —
  including shl4 and chset legs the hand era never priced) and
  examples/impgen_x86_out.shard (9 + 9; divq/add1w honestly refused
  by the translator; imp_x_mix and the wrap64-shl-leg imp_x_rotr7
  are new coverage). Every generated claim passed the kernel
  FIRST-TRY; the generated imp_w_sel is the hand claim up to binder
  naming and fuel padding. Regen is byte-identical on both targets;
  both files are corpus targets.
- V1 scope fences (a decline leaves the tie standing plus an honest
  note): IWhile/ILoad/IStore (the loop/mem tier), comparisons in
  value position, symbolic shift/rotate counts, conditional divisor
  guards, premise-carrying ops under nested branches.
- NEXT: I2d-2 the loop tier (mechanize the iww_*/ixw_*
  worker-induction shapes), I2d-3 the sha256 legs (ix_home 6→12 +
  the K-window data segments), build DERIVE integration.

**I2d-2 — impgen: the loop tier mechanized (2026-07-12).** The
counting-loop WORKER (the iww_*/ixw_* induction template — the M×N
kill's per-program machine-vs-machine leg) and the DENOTATION BRIDGE
(call_fn_mem/xcall_fn_mem = icall_mem) are now generated. Recognition
is EXACT and honest: a pin whose body is one IWhile over (ILoc c)
with one of the two hand-validated statement sequences — fill
`[mem[p] := v; p += 1; c -= 1]`, sum `[a += mem[p]; p += 1; c -= 1]`
— with distinct param locals, no extras, and an IConst/ILoc result;
everything else declines to tie-plus-note (the scalar tier's fence
discipline).

- The worker synthesizes: the premise family from the shape (pointer
  lo/hi bounds; the sum shape adds the k-scaled byte-accumulator
  invariant at the target modulus), the machine module/loop-body
  literals from the RUN translation (sp_e-spelled, zero re-derivation),
  the x86 register file as the MkRegs literal over the SysV homes
  (counter spelled through `(int_of_nat k)`), the read-loop scratch
  residue as the loopkit's `xlg_last` with a quantified `ra`, and the
  hand S-case chain verbatim with indices spliced — fuel unfolds,
  lg_ne/im_lt guard deciders, the hsub counter collapse, the
  hwk/hw0(/hwa) wrap-identity haves, and the IH with per-premise
  preservation discharges. Every discharge cites the target loopkit
  GENERICALLY (lg_*/xlg_* + im_lt); no new lemma is generated, ever.
- The bridge's joints are DISCOVERED, none re-derived: the
  IFn→IProg wrapper by signature scan (this also generalized the
  scalar tier's hardcoded `it_prog`), the imp-side meeting lemma
  `imw_<stem>` by a textual scan of the source file (the claim
  roster is CHECK-ONLY — run-mode parse skips claims; a false
  positive fails loudly at kernel time), and the wrapper memsize
  checked = 65536 (the loopkit's constants). Missing joints leave
  the worker standing with an honest note. CONTRACT: the meeting
  lemma is stated at `(S^ (gcost body + 5) (lg_fuel k c))` with
  premises `[le 0 ptr; le (+ ptr (int_of_nat k)) 65536]` in that
  order — both hand imw_* already are.
- Fuel: the machine-side reservoir is tcost(loop body)+4 and needs
  only to be sufficient (the per-iteration burn is exactly the entry
  unit, restored by the lg_fuel tower's S, so the IH shape returns
  at ANY constant ≥ the body depth); the imp side must match the
  meeting lemma's spelling exactly, hence the contract above. Bridge
  entries peel exactly 4 (machine: seq+instr(Block)+seq+instr(Loop))
  and 2 (imp: istmts+istmt).
- Validation: examples/impgen_wasm_loop_out.shard and
  examples/impgen_x86_loop_out.shard — per target 2 ties + 2 workers
  + 2 denotation bridges over il_fill_fn/il_sum_fn, ALL TWELVE
  kernel-green FIRST-TRY; the generated claims are the hand
  iww_*/ixw_*/imp_w_*/imp_x_* up to binder naming and fuel padding.
  Regen byte-identical both targets; the committed scalar outputs
  regenerate byte-identical under the extended tool; both loop files
  are corpus targets.
- NEXT: I2d-3 the sha256 legs (ix_home 6→12 + the K-window data
  segments; the sha loop bodies exceed the v1 loop family — the
  recognizer and the phase/fold machinery grow there), build DERIVE
  integration.

**I2d-2b — build DERIVE integration: impgen enters the derive slot
(2026-07-13).** The four generated cert files are driver products —
the small slice §5 predicted, pulled ahead of I2d-3 because it closes
the regen-is-canon contract on the whole impgen surface cheaply. The
driver grew kind **'impgen** (targets 'wasm/'x86): the standard regen
ladder (determinism up to canonicalization) with tools/impgen as the
generator. A new KIND rather than a new target because 'regen selects
its generator BY target and impgen shares both targets with
wasmgen/x86gen — a second generator per target forces the kind axis
(derive holds no privilege; a new generator is just a new kind over
the same ladder). One mechanical constraint shaped the orders: the
generated header stamps the regen command (raw path included), so the
raw MUST be generated at OUT's `.shard`→`.raw` sibling spelling —
capture-dir naming fails the byte compare by construction — and a
gated rm leg cleans it after the fmt leg reads it. Each output also
gates as a 'check product (the imp-tower norm: the aggregate
kernel-gates what it regen-gates). Products 52→60 in
examples/build_products.shard as 'impgen+'check pairs, all green;
corpus FAIL-set unchanged at 57. Remaining in I2d: I2d-3 per the
record above.

## 7. Non-goals, stated once

- imp as a shipped target or public surface — it is an intermediate;
  terminal targets remain the ISA models.
- imp as kernel syntax — it is a library; the reader never learns it.
- Registers/flags/modes/encodings at imp level (forever).
- A general optimizer at imp level — rep choices are declared and
  proven, never discovered by a hidden pass (MEMORY.md §10's
  no-hidden-liveness rule applies to imp verbatim).
- Per-width imp twin families — a twin fixes its kinds (§2a) and
  runs wherever capability gates accept it; the Word residue
  (type-parameter families) is not re-created — kinds are a closed
  value-level tag set. [v1 wording — "width is value-parametric
  with premised wrap ops" — superseded 2026-07-14 by §2a.]


## 8. Decision points

- **DI1 — fragment grammar granularity: RESOLVED (2026-07-12).** The
  existing fragment taxonomy (straight-line / loop / mem /
  calls-in-loops) verbatim, until otherwise needed.
- **DI2 — imp twin vectors: RESOLVED BY DISSOLUTION (2026-07-12).**
  The question was curated-vs-spec-derived; the user's counter —
  what would they prove? — dissolves it: no differential vectors at
  imp as a build gate at all. The kernel gate is the gate; imp has
  no external reality to differ against, and real vectors run end
  to end at the ISA engine legs. The development-time
  machine-validation probe (§3) is the surviving remnant.
- **DI3 — how much frame convention is imp-level: OPEN,
  discovery-mode (2026-07-12 ruling).** To be discovered as the
  pieces are fit together at I0–I2. Standing lean: extent/ownership
  structure at imp; byte-exact packing with the ISA docs
  (MEMORY.md D6).
- **DI4 — residence of the lowering families: RESOLVED
  (2026-07-12).** As leaned: theorems live with the model,
  generators in tools/, matching the wasm/x86 split today.
