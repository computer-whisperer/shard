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

**V2-1 — the typed machine (2026-07-14).** The §2a migration ladder's
first slice: models/imp/imp.shard reworked to crystallized scalar
kinds, delete-first.

- **The machine**: IKind (U8/U32/U64 value ctors) + ikw/ikmod;
  IBin/IRotr carry their kind and WRAP RESULTS to it (add/sub/mul by
  euclidean mod — the unsigned two's-complement wrap; div/rem/bit
  ops band-closed on in-band operands); IWrap DELETED (and ipow2
  with it — ikmod is ground literals); explicit IExt (value identity
  at this level) and ITrunc (mask-narrow); shift counts trap outside
  [0, width) — at-width counts now trap where v1 allowed them,
  because wasm masks counts mod 32 and x86 mod 64 (rotate counts
  stay masked: that masking IS uniform across machines). IFn
  signatures carry per-local kinds (ikparams/ikextra lists +
  irkind); icall/icall_mem BAND ARGS to parameter kinds on entry
  (a machine register IS its width — unrepresentability mirrored as
  banding, not error).
- **The gate**: wk_fn — ikchk (constants in band at their use site,
  operand kinds agree with the node, conversions strictly
  directional, U8 admits only compare among the ops, ground
  shift/rotate counts below width) + iksyn (conditions must announce
  a kind — bare-constant conditions refuse) + iwk_stmts (sets at the
  target local's kind, stores U32-address/U8-value). Symbolic
  shift/rotate counts refuse at the v1 gate (named growth behind a
  guard story).
- **The probe grid re-validated** (examples/imp_probe.shard): the
  carried v1 probes keep their expected values verbatim — in-band
  programs mean what they meant (the IWrap probe becomes the ITrunc
  heir ipb_trunc) — plus the v2 additions: U32/U64 add-wrap and
  sub-wrap at each kind's own modulus, entry banding observed
  through a U8 param, IExt identity, at-width shift trap, and
  twelve wk_fn accept/reject pins (one per gate rule, including the
  symbolic-count refusals). 87/0 on the fast engine, every claim
  (compute both) except the two std/mem surface-law chains,
  unchanged in shape from v1.
- **The prune (delete-first, gates honest)**: the v1 consumer
  surface is retired from the tree and the gates until each
  migration slice re-lands it — deleted: to_wasm.shard,
  to_x86.shard, imp_scalar/imp_loop twins, the four bridges, the
  four impgen outputs, tools/impgen (the recognizer-era tool; the
  rebuild is the structural walk). The sha sibling STAYS in-tree as
  migration source, unregistered (std/sha256/mod.build.shard's
  product list emptied with a migration note). Corpus targets 266 →
  253; driver products 60 → 43.
- Gates: machine + probe green on the fast engine (52/0, 87/0);
  driver 43 products all green; corpus FAIL-set unchanged at the
  57-line baseline (deleted targets exit the run, they do not
  redden it).
- NEXT (the §2a ladder): the translators re-land kind-directed
  (to_wasm with the i64 vocabulary growth, to_x86 with non-REX.W
  32-bit forms + ix_home 6→12; encodings/vectors Opus-delegated) +
  the scalar tier re-lands, then the loop tier, then the sha
  sibling, then impgen as the structural walk over wk programs.

**V2-2a — the wasm leg, kind-directed (2026-07-14).** The translator
and the scalar tier re-land on the typed machine, and the §2a thesis
is now a checked artifact: EVERY imp ⊑ wasm bridge in the re-landed
file is UNPREMISED. Every claim in all three files passed first-try.

- **to_wasm.shard rebuilt**: imp2w_fn checks wk_fn FIRST (the
  membership gate made code), then kind-directed selection — U8/U32
  ops onto the wrapping i32 ops (whose wrap IS imp's op-result mod),
  IRotr U32 onto the band-spelled i32.rotr, IExt-below-32 a no-op,
  ITrunc U32→U8 the And-255 emission (imp's ITrunc was amended
  BAND-spelled to match — the bit-op idiom, the sha specs' own mask
  spelling; arithmetic wrap stays mod). U64 anywhere REFUSES pending
  the i64 vocabulary — the capability doctrine's honest fence,
  pinned by tie_addq_refuses = None. Statement encodings verbatim
  (kinds change zero emitted bytes for the U32 class: the v1 tie
  literals re-tie byte-identical).
- **The bridge statement**: call_fn is fed through iband_args — the
  alignment relation made syntax; icall bands identically inside.
  Consequence: imp_w_mix (v1: six range premises, three
  wrap-identity haves) is now `compute both; refl` with ZERO
  premises; imp_w_sel's four conditional premises dissolve into a
  case split on the shared banded scrutinee; even imp_w_divq's
  divisor premise dissolves (both machines guard the banded divisor
  — div0 is None = None, a case split, not a premise).
- **imp_scalar.shard re-landed**: eleven twins (the six v1 members +
  shr3/shl4/rotr7/chset + conversion twins it_tob/it_ext + the U64
  it_addq) with per-kind wrap-identity lemmas
  (iwrap8_id/iwrap32_id/iwrap64_id — the v1 fin-split proof survived
  modulus substitution verbatim at 256 and 2^64) and spec ⊑ imp
  certs carrying the FIT PREMISES — args and exact intermediates in
  band, discharged by the wrap lemmas. This is the v1 bridge-premise
  apparatus RELOCATED to the target-free half, stated once
  (imp_lg_addq states it in 2^64 — same twin vocabulary, its own
  modulus). wcomp_lg_add1 re-closes the tower: unpremised bridge +
  fit-premised imp cert = spec ⊑ wasm in two rewrites.
- Gates: fast engine 84/0 (to_wasm), 68/0 (imp_scalar), 120/0
  (bridge); driver 46 products green; corpus FAIL-set unchanged at
  the 57 baseline (256 targets).
- REMAINING V2-2: (b) the x86 leg — non-REX.W 32-bit forms + ix_home
  6→12 in the x86 model (encodings/vectors Opus-delegated), to_x86
  rebuilt kind-directed, the x86 bridges re-landed; (c) the i64
  capability — the wasm model's i64 vocabulary + the U64 legs.

**V2-2b — the x86 leg, kind-directed (2026-07-14).** The second
target confirms the v2 thesis: the x86 bridges are UNPREMISED by the
same iband_args statement shape, and this target's capability set is
the full {U8, U32, U64} from day one (U64 is native where wasm still
refuses it — the doctrine's first two-target asymmetry, pinned from
both sides).

- **Model growth (x86.shard — semantics Fable-side, encodings +
  silicon vectors Opus-delegated per the standing split)**: three
  ADDITIVE ctors. (XBin32 op d s) — the 32-bit operand forms;
  arithmetic is congruence-honest at mod 2^32 with NO operand
  truncation spelled (the low-half sum IS the full-register sum mod
  2^32), bitwise truncates its result; results zero-extend (the
  x86_64 rule, U32's maskless-native path). (XShlI32 d k) — count
  masked mod 32, result mod 2^32. (XMovRR32 d s) — the
  zero-extending 32-bit mov, band-spelled: the one-instruction
  truncate-to-32.
- **The width-form selection law (to_x86 rebuilt)**: wrapping arith
  picks its form by kind (XBin32 at U32, XBin at U64); the
  band-closed bitwise trio and shr ride the 64-bit forms AT EVERY
  KIND (they are syntactically imp's exact ops — the chset tie is
  byte-identical to its v1 self); IShl picks XShlI32/XShlI; IExt is
  a no-op at every widening pair; ITrunc→U8 is And-255, ITrunc
  U64→U32 is XMovRR32. imp2x_fn gates on wk_fn first, params ≤ 6
  (the SysV boundary), locals ≤ 12 — **ix_home 6→12**: body locals
  6-11 in the callee-saved file (rbx rbp r12-r15), zero at entry in
  the boundary's fresh register file = imp's zero-init (xargs needs
  no change until the loop tier's ix_out). Pinned by the it_wide
  twin (7 locals, local 6 in RBX) — tie + unpremised bridge + imp
  cert.
- **The U32 tie literals are NEW** (XBin32 forms): the
  byte-identity-to-x86gen story ends for the U32 class — the frozen
  direct generator emits 64-bit ops for what are now U32 twins; its
  own artifacts stay green on their own chain. Division still
  refuses (xtie_divq_refuses; the rdx:rax preamble is named
  growth). New conversion twins: it_tow (ITrunc U64→U32; wasm
  refuses it with the U64 tier) rides XMovRR32.
- **The one scoped non-refl**: the IRotr composition's left-shift
  leg wraps at 2^64 mid-tree, which imp's band-spelled IRotr does
  not spell — the rotation twin TIES on x86 but its bridge waits
  for the native 32-bit rotate instruction (named growth; the wasm
  rotation bridge is already refl). Every OTHER bridge:
  compute-both refl or a bare case split on the banded scrutinee.
  xcomp_lg_add1 closes the same spec through the same imp cert on
  the second target — the M×N kill on the typed machine.
- Gates: fast engine 355/0 (to_x86), 70/0 (imp_scalar + tow/wide),
  401/0 (x86 bridge) — every claim first-try; driver 48 products
  green; corpus FAIL-set unchanged at the 57 baseline (258
  targets); silicon differential green over the grown vector set
  (the Opus-delegated encode.shard arms + wrap/truncation/
  count-mask witnesses).
- REMAINING V2-2: (c) the i64 capability — the wasm model's i64
  vocabulary + the U64 legs on wasm. Then the loop tier (V2-3).

**V2-2c — the wasm i64 capability (2026-07-14).** The wasm leg joins
x86 at the full {U8, U32, U64} capability set; V2-2 is complete, and
every capability asymmetry between the two current targets is now
pinned from both sides.

- **Model growth (wasm.shard — wasm encodings are Fable-side per the
  I2a precedent; only x86 byte-emit rides the Opus split)**: four
  ADDITIVE Instr ctors on the same untyped-Int stack. (I64Const c) /
  (I64Bin Bop) with bop64_val mirroring bop_val at the wider width —
  wrap64 arithmetic, guarded division, band-closed bitwise, shift
  counts mod 64, rotr band-spelled at the 64-bit mask; I64ExtendI32U
  (the value identity); I32WrapI64 (low 32, BAND-spelled — the trunc
  idiom, the XMovRR32 precedent). encode.shard grows the i64 opcode
  page and the ten-level sleb64 ladder.
- **A LATENT TOTALITY BUG found and fixed in BOTH models**: the
  kernel shift prims reduce only for counts in [0, 64), and the
  rotate spelling's complement leg (- w (mod c w)) hits exactly 64
  at 64-bit rotate-count ≡ 0 — so imp's own V2-1 IRotr at U64 (and
  the first draft of the wasm i64 rotr) STUCK on rotate-by-0 instead
  of answering. Both spellings now DOUBLE-MOD the complement leg
  ((mod (- w (mod c w)) w) — bshl by 0 is the identity rotation),
  which ground-collapses identically at every nonzero count; the
  probe grid pins the corner (ipb_rotrq0/ipb_rotrq, 87→89). The
  32-bit rotates never trip the fence (complement ≤ 32) and are
  untouched.
- **The differential grew 173 → 280 agree, 0 disagree**: the i64
  tier never crosses the module boundary (functype/locals stay
  all-i32) — operands enter as I64Const immediates, results leave
  through i32.wrap_i64 (low half direct, high half through shr-32),
  comparisons return their i32 directly. Vectors cover the wrap and
  carry witnesses, count-mod-64 boundaries (count 32 must SHIFT —
  the divergence-from-32 witness — and count 64 must mask to 0),
  the 2^63 signed-comparison frontier, equal-low-half inequality,
  zero-divisor traps, the extend/wrap conversions against a live
  param, the count≡0 rotate, and every sleb64 rung including the
  deep negatives.
- **to_wasm rebuilt kind-directed at the expression level**: iw_exp
  translates AT AN EXPECTED KIND (the ikchk discipline — constants
  and op selection are kind-directed from context), so well-kinded
  imp translates to type-correct wasm by construction; the U8/U32
  emissions are byte-identical to their V2-2a selves (every existing
  tie re-tied). The one type seam — comparisons produce i32 at every
  kind — resolves twice: value-position U64 comparisons re-widen
  through i64.extend_i32_u, and comparison-headed CONDITIONS emit
  the comparison directly into BrIf (the fused shape, the x86 leg's
  condition-fusion move; byte-identical for the U32 class).
- **The new twins pin the whole capability matrix**: it_addq's wasm
  leg lands (tie_addq_refuses retires) and it_tow gains its wasm
  side; it_extq pins the one conversion that emits a real
  instruction on wasm (a no-op on x86); it_selq pins fused U64
  conditions on BOTH targets (i64 compare into BrIf; kind-agnostic
  CEq on the full register); it_rotrq is NATIVE on wasm and refuses
  on x86 (the rotate asymmetry — the mirror image of the old addq
  story); it_tobq is one instruction on x86 (And-255 serves any
  source width) and refuses on wasm (no single band-spelled
  emission). Every bridge UNPREMISED, same iband_args statement
  shape, every claim in every file FIRST-TRY.
- **Scoped refusals, all loud, all named growth**: wasm — the direct
  U64→U8 narrowing (tie_tobq_refuses; needs a band-absorption story
  or a dedicated emission) and non-comparison U64 conditions
  (i64.eqz enters with its loop-tier consumer); x86 — the U64
  rotation (xtie_rotrq_refuses). Typed FUNCTION BOUNDARIES at the
  binary encoder (i64 params/results/locals in functype and
  localdecl) are named growth behind the first shipping U64 wasm
  artifact — the kernel-level bridges need no encoding, and the
  differential's vector boundaries are deliberately all-i32.
- Gates: fast engine 32/0 (wasm model), 32/0 (encode), 52/0 (imp),
  89/0 (probe), 84/0 (to_wasm), 74/0 (imp_scalar), 136/0 (wasm
  bridge), 412/0 (x86 bridge), 274/0 (diff plan); V8 280/0; no new
  files, no product-list changes (driver 48, corpus 258); corpus
  FAIL-set unchanged at the 57 baseline.
- NEXT: V2-3 the loop tier — xargs 6→12 (ix_out's consumer), the
  loopkit re-land, the loop twins + worker bridges on the typed
  machine (i64.eqz and the U64 while guards arrive here). Then the
  sha sibling migration, impgen rebuilt as the structural walk.

**V2-3 — the loop tier (2026-07-14).** The loop fragment class
re-lands on the typed machine, and the §2a dissolution is
demonstrated exactly where it was priced: in the loop workers, whose
per-target accumulator apparatus was the original evidence against
v1 scalars.

- **Model growth**: wasm gains I64Eqz (the i32-valued zero test,
  opcode 0x50; Fable-side per the I2a precedent) — the named
  consumer of the V2-2c refusal, and the reason it waited: a WHILE
  guard is a NEGATED test, so i64.eqz IS the U64 guard emission
  (iw_wcond ends every guard in the kind-directed eqz; IF-position
  non-comparison U64 conditions still refuse — the positive
  truthiness test would need a double eqz and has no consumer). The
  V8 differential grew 280 → 287 agree, 0 disagree; the eqz vectors
  include the 2^32 operand — low half zero, so an i32.eqz-of-wrap
  confusion answers 1 where i64.eqz must answer 0 — and a live-param
  variant. x86: xargs grows 6 → 12 (positions 6-11 onto the
  callee-saved rbx rbp r12-r15, the §2a home file) — ix_out's
  consumer, rebuilding a full locals list as the loop-exit register
  file; no encoding surface (silicon differential unchanged, 112/0).
- **Translator growth**: to_wasm's while tier moves the zero test
  into the condition (iw_wcond — U32-class emissions byte-identical,
  every loop tie re-ties); to_x86's ISet byte-load pattern unwraps
  the explicit IExt around ILoad (v2's crystallization point in the
  sum body — a no-op on this machine, XLoad8 zero-extends into the
  full register).
- **examples/imp_loop.shard re-lands kinded**: fill at (U32 address,
  U8 value, U32 counter), sum with the explicit (IExt U8 U32 (ILoad
  …)) read, and the NEW U64 doubling twin — spec lq_dbl is
  BAND-spelled (the sha m32 idiom at 2^64), and its bare-local U64
  guard is the i64.eqz consumer on wasm and the kind-agnostic CEqz
  on x86. THE RELOCATED FIT APPARATUS: the sum worker's accumulator
  premises (nonneg + the k-scaled invariant) and the wrap-collapse
  haves now live at spec ⊑ imp, once — imw_sum carries the v1 WASM
  worker's premise family at 2^32, imw_fill/imw_dblq grow the
  counter/pointer collapses, and the icall pins carry the
  args-in-band premises (entry banding), collapsed through the new
  cap lemmas (il_kM/il_dM from the pointer-range pair; il_kdec the
  U64 counter step-down — the dblq family's one fit premise is the
  counter cap itself, since no pointer premise exists to imply it).
- **THE DISSOLUTION AT THE BRIDGES**: iww_sum and ixw_bsum carry the
  pointer pair ONLY — v1 carried four premises plus the hwa
  accumulator collapse and the lg_sum_*/get_* citations, PER TARGET
  in each machine's modulus; v2's machines wrap the accumulator
  exactly as imp does, so the accumulated value rides the IH's
  quantifier as the shared mod term and the whole apparatus is gone
  from both legs. The remaining premises are the memory story (store/
  load guards + the counter cap), and the IH-shape collapses they
  drive are now the SAME 2^32 haves on both targets (v1's x86 leg
  restated them at 2^64) — one discharge vocabulary,
  iwrap32_id/iwrap64_id from the scalar tier serving both legs.
- **Ties**: the wasm fill/sum twins re-tie BYTE-IDENTICAL against
  wasmgen_loop_out's generated funcs (kinds + the explicit IExt
  change zero emitted bytes); the x86 ties are new XBin32 literals
  (the byte-identity-to-x86gen story ends for the U32 loop class,
  exactly the V2-2b scalar precedent); the dblq twin ties literal on
  both targets — loop artifacts no direct generator ever made, with
  the first i64-bodied IWhile encoding. Denotation bridges feed the
  machines through iband_args (the v2 statement shape); the
  compositions close spec ⊑ wasm and spec ⊑ x86 at the same spec
  fns, the U64 tier included (wcomp_lq_dblq/xcomp_lq_dblq).
- Gates: fast engine 99/0 (imp_loop), 172/0 (wasm loop bridge),
  441/0 (x86 loop bridge), every existing consumer re-checked green
  — EVERY CLAIM FIRST-TRY AGAIN; V8 287/0; silicon 112/0; +3 corpus
  targets (261), +3 'check imp products (driver 51); corpus FAIL-set
  unchanged at the 57 baseline.
- NEXT: the sha sibling migration (masks dissolve into kinds; the
  continuation-phase machinery survives untouched), then impgen
  rebuilt as the structural walk over well-kinded imp.

**V2-4 — the sha sibling migration (2026-07-14).** The 11k-line
flagship twin (std/sha256/sha256.imp.shard) migrates to crystallized
kinds — the ladder's stress test of the §2a claim that hand-written
twins re-land without structural loss, and the largest single v2
slice: 349 claims, every machine-facing tier re-proven.

- **What dissolved, what appeared.** All 26 explicit m32 mask
  statements are GONE (every wrapping op is a mask point now), and
  the byte-store mask+shuttle pairs fused into direct
  (ITrunc U32 U8 …) stores — the conversion node IS the byte mask,
  and the stored trees are wput's chain verbatim. In exchange the
  machine wraps EVERY arithmetic op via mod while the spec masks via
  band only where FIPS masks — the band/mod seam the v1
  band-spelling thesis never had to cross. It is paid ONCE, in a
  conversion kit at the top of the sibling: the mask/mod bridge at
  the 32-bit window (mask_pow2 grounded through a pow2-32 ladder),
  euclidean flatten congruences (mod_unique against exhibited
  decompositions), the in-band collapse, pointer/counter collapse
  shapes at the walks' literal trees, and the wrapped-Horner unwrap
  (ish_bem — premise-free and mem-facing, so every pass cites it
  with one plain all-occurrence rewrite).
- **The claim statements survived; the proofs grew conversions.**
  Pass/worker/phase statements are v1's up to premises: pointer
  walks collapse wraps as they appear (the guard-lemma idiom, one
  tiny premised lemma per tree shape), Horner loads unwrap to v1's
  exact trees, and the masked sums convert by per-shape composites
  (ish_t1c/t2c/e2c for the round tier's T1/T2/e' chains, ish_swc at
  the extension's sched_w fold) — after which the v1 rewrite chains
  (ish_be, ish_sum5/sum3, the wget_be/wput_sets folds) fire
  VERBATIM. The continuation-phase machinery carried over untouched
  as §2a predicted; σ/sum/jump phases state the machine's wrapped
  trees and stay compute-both refl-grade; only spine fuels shifted
  where mask statements left (sg1 10→9, the store phase 20→16, the
  finish lanes 22→17). The block walk's fuel-reshape algebra and all
  57 positional farkas certs survived unchanged — the collapse work
  lives inside the cited lemmas, never at the walk.
- **The relocated fit story, sha-sized.** The round pass/worker now
  carry the in-band premises the v2 doctrine prices at spec ⊑ imp:
  eight state nonnegs plus the counter range, discharged in the
  worker by two computed-round projection bounds (ish_ra_lo/re_lo,
  fed by an extended shift ladder — shr to 25, shl to 30, the six
  Σ-rotation trees the σ-only v1 ladder never needed) and at the
  block walk by wget's premise-free range lemmas. Counter caps
  derive from the pointer-window premises (ish_kle), the V2-3 cap
  pattern verbatim.
- **h8add_mem respells mod-style** (machine-facing, like the fs
  lanes that build it); the future digest tier pays its band
  conversion where the h-values are concrete wget words. The four
  bodies are pinned WELL-KINDED (ish_wk_* in the 12-slot U32 frame)
  — the rebuilt impgen's structural walk consumes them directly.
- **Registration restored**: the corpus target and the module's
  'check imp product return (driver 52); corpus FAIL-set unchanged
  at the 57 baseline.
- NEXT: impgen rebuilt as the structural walk over well-kinded imp
  (the recognizer tiers retire; the sha ISA legs fall out of the
  rebuilt generator over this sibling), then I2e (./sha256sum).

**V2-5 — impgen rebuilt: the structural walk over well-kinded imp
(2026-07-14).** The recognizer era ends. Four commits: the probe, the
translator growth it required, and the two generated tiers.

- **The probe (examples/iwg_probe.shard, kept as the form's pin).**
  The loop tier's statement form, kernel-validated by hand before the
  generator stamped it: the GENERIC Some-conditional worker. NO
  per-program premises — locals fully symbolic, the iteration count a
  GHOST in the fuel shape (imp side `(S^B (lg_fuel k 0))`, tail zero
  so exhaustion is contradictory; machine side keeps the slack tail),
  ONE premise: the imp loop run LANDS (`= (Some o)`). Proof leaves:
  EXIT (guard dies, both machines exit, the premise pins o), TRAP
  (bound tests case-on'd on the shared banded scrutinee; the
  machine's le-(+1) spelling bridged by integer-tight KEYED-ROWS
  arith haves — the QoL pair's first consumer), ABSURD (fuel death
  under the premise, `(absurd …)` on the computed None = Some), STEP
  (the premise TRANSPORTS one iteration through the same case facts;
  the IH fires with `(inst o o)`). Both engines burn exactly one
  tower unit per iteration (same reservoir to head and tail), so
  lockstep holds at matched ghost towers. The v1 workers'
  hwk/hw0/accumulator preservation apparatus has no analogue — with
  no premises there is nothing to preserve.
- **V2-5a — the zero-scratch loop-boundary invariant (to_x86
  growth).** The x86 while tier emits Z(body) — one `XMovRI r 0` per
  scratch register (RAX/R10/R11) the emitted body WRITES — at loop
  entry and before the back-edge: scratch is 0 at every loop
  boundary, the exit register file is exactly `(xargs locals)`, and
  workers state entry/exit scratch as literal 0. The xlg_last residue
  selector (a closed form that exists only for recognized families)
  retires: ixw_bsum drops its ra binder and its unfold dance;
  xtie_bsum pins the new emission; scratch-free bodies (fill/dblq)
  emit nothing and stay byte-stable. Dead-store elimination is a perf
  rung. No encoding surface.
- **V2-5b — the scalar/branch tier.** tools/impgen re-lands at ~1.4k
  lines (was 2.5k): premise events, condition-relative paths, Ev
  dedup, and loop recognition are GONE. The walk mirrors iexp/iop_val
  exactly — params enter BANDED (the statement feeds iband_args), op
  results wrap to their node's kind — and a bridge is the case-on
  tree over shared banded scrutinees (branch conditions, div/rem
  guards; every False guard arm a trap leaf, both denotations None).
  Statements UNPREMISED; the factoring unchanged (translators loaded
  via meta/invoke and RUN; sp_e ties; wrapper by signature, its
  memsize DISCOVERED by running it). x86 rotation bridges fence
  tie-only (the V2-2b scoped non-refl). Outputs regenerate
  byte-identical at the sibling raw path; the generated files are
  RICHER than the v1 outs (U64 tier, ext/trunc/tob/tow legs) with
  zero premises anywhere. Every generated claim green:
  impgen_wasm_out 142/0, impgen_x86_out 410/0.
- **V2-5c — the loop tier.** The probe form, stamped structurally:
  per single-IWhile pin the tool emits body-copy fns, the generic
  worker (B = the loop body's EXACT istmts need, so the base case
  dies at the last statement's entry and only the earlier statements'
  splits fire; hyp citations positional, computed per depth), and the
  Some-CONDITIONAL denotation — a plain `call_fn_mem = icall_mem`
  equality premised only on the imp run landing, proof = case-on the
  banded iwhile term (None arm absurd), the worker citation, case-on
  the outcome, and exit-locals spine exposure to the result local's
  depth (Nil arms close: both machines trap on short lists). fill/
  sum/dblq: every wasm claim green (140/0), every x86 tie + worker
  green (432/0 — the zero-scratch invariant delivering literal-0
  IH re-entry). The four outs are corpus targets and 'impgen+'check
  driver product pairs again (the I2d-2b DERIVE contract unchanged).
- **The named x86 joint: il_wlen.** The x86 loop DENOTATIONS emit
  worker-plus-note for now: ix_out rebuilds the register file via
  `(xargs lc2)`, which needs the exit-locals ARITY to compute, and
  the impossible-length case arms want iwhile LENGTH PRESERVATION —
  an honest imp-side model fact (il_wlen + the istmts/istmt/ilset
  chain + ilen_nonneg), to be proved ONCE in models/imp and cited by
  the generated refutations, never synthesized per program. The wasm
  leg needs nothing (iw_out carries the locals list opaquely).
- **Fences (note-and-tie, all named growth):** loop bodies with
  branches or nested loops (every sha loop body is straight-line),
  comparison while-guards (to_x86 already refuses them), results
  other than IConst/ILoc, stores whose VALUE walk carries splits
  (machine and imp order value-vs-bounds differently), comparisons in
  value position, multi-statement bodies around a loop (the sha
  statement-level tier is the next rung).
- **The contract change, recorded:** generated loop bridges are
  Some-conditional; the I2d-2 meeting-lemma premise contract
  (pointer-window order) is SUPERSEDED — a composition instantiates
  the meeting lemma's slack tail at Z and feeds the Some fact
  directly (ground Nat packing aligns the `lg_fuel k 0` spelling
  under compute). Hand bridge files stand as regression pins.
- NEXT: il_wlen + the x86 denotation flip, then the sha legs — the
  generated statement-level (istmts ≈ eval_seq) bridges over the
  sibling's wk-pinned bodies (the walk machinery is
  statement-generic; the denotation tier is what specializes), then
  I2e (./sha256sum).

**V2-5e — il_wlen landed, the x86 denotation flip (the loop tier is
FULL on both targets).** Two commits: the model fact, then the
consumer.

- **The length-preservation family (models/imp/imp.shard grows its
  first claims).** The statement engines never change the locals
  arity. Stated EXTRACTOR-STYLE: total unwrappers `ollen`/`iolen`
  read a length out of a result Option (defaulting to n on
  None/ITrap), so every lemma is an UNPREMISED equation — no
  constructor inversion at consumers, no premised claims; a consumer
  rewrites its in-scope hyp into the extractor term and computes.
- **The mutual-SCC knot resolves with zero mutual citation.** The
  wrapper equivalences `il_s2s`/`il_w2s` spell one statement (a loop)
  as its singleton list at fuel+2 — the extra units feed exactly the
  peels and the Nil tail (case-on + computes, NO induction). That
  makes `il_slen` (istmts) the ONE induction: subterm-induct on fuel
  (strong IH along the structural subterm order — its first use on a
  Nat), statement dispatch INLINED, the IWhile arm reaching the
  loop's self-recursion by rewriting `(iwhile f4 …)` to its il_w2s
  spelling at `(S (S f4))`, strictly below `fuel = (S (S (S f4)))`.
  `il_s1len`/`il_wlen` then fall out one-directionally.
- **Kernel empirics (probed first, then spent):** subterm-induct
  accepts a Nat subject; `(below)` chases case-on hyp equations to
  depth 2 (vars AND ctor terms) but NOT depth 3 — a deep cite either
  folds the fuel spelling back up before citing (exit/tail arms) or
  rewrites the below-obligation fully syntactic inside the
  rewrite-with DISCHARGE sub-proof (`((steps ((rewrite (hyp F) lr
  lhs true ()) …) (below))`) — the discharge runs in the same
  sequent, so hyp indices carry over.
- **The flip (impgen's x86 leg emits full denotations).** lp_spine
  splits by target: the x86 walker (`lpx_spine`) case-ons the exit
  locals to the twin's FULL ARITY nl2 (ix_out's xargs rebuild needs
  the whole register file — wasm still stops at the result depth).
  Arms: Nil BELOW the result local → the imp result extraction
  computes None, absurd against the Some premise; Nil at/above it →
  refl (xargs zero-fills the missing homes and the result home is
  filled — the vacuous arms are TRUE, not just unreachable); Nil at
  depth nl2 → the true exit (xargs grounds to the register file);
  Cons at depth nl2 → the overlong tail, refuted by il_wlen +
  ilen_nonneg: three transport haves (hbad via the lemma, hlen via
  compute, hg chaining them — the lemma-across-computation dance),
  the spine-expanded length (hs, an unfold/reduce ladder), and a
  farkas absurd whose cert is CONSTANT across programs —
  `(rows (goal 1) (hg -1) (hs 1) (hnn 1))`, since NL2 − nl2 − 1 = −1
  at every arity. `(absurd …)` closes on `(le 0 0) = False` directly
  (absurd simps both sides; ground calls reduce, True/False clash).
- Gates: imp.shard 59/0 + all 15 importers re-checked green;
  the regenerated x86 loop out 442/0 — EVERY generated denotation
  (fill/sum/dblq incl. sum's absurd-flavor arms and the U64 tier)
  first-try after one hand probe; regen deterministic at the sibling
  raw path; wasm + scalar outs BYTE-STABLE under the modified tool;
  driver 60 green; corpus FAIL-set identical.
- NEXT: the sha statement-level legs (istmts ≈ eval_seq bridges over
  the sibling's ish_wk_* pinned bodies, both targets; phase-structured
  emission is the named mitigation if whole-body kernel time bites),
  then I2e (./sha256sum).

**V2-5e-3 — the statement tier: the straight-line sha legs (the trio
chunk).** impgen grows its third bridge tier; the sha round/copy/ext
bodies get generated seq-grain legs. Two commits: the translator
growth, then the tier + the generated outs.

- **The translator growth (probe-diagnosed, additive-only).** Across
  all four pinned sha bodies the v2 x86 pattern tier refused exactly
  TWO shapes: the bare ext-load `(ISet i (IExt U8 _ (ILoad (ILoc
  j))))` (every Horner walk's opening byte) and the ITrunc byte store
  `(IStore (ILoc a) (ITrunc U32 U8 v))`. Growth: the ext-load joins
  ix_set's pattern tier (XLoad8 zero-extends — the ext dissolves);
  a non-src store value takes the general path (ix_res into RAX,
  XStore8 (SReg RAX)) — XStore8 stores the register RAW, so the
  And-mask ix_acc appends MATERIALIZES imp's byte band and the stored
  trees align by construction. All four bodies now translate on both
  targets; every existing generated out is regen-byte-identical (the
  growth touched only previously-None arms).
- **The seq seam.** to_x86 `ix_sstmts` = the SEALED statement-sequence
  entry: translated chunk + zero-scratch seal (`ix_zs` over the
  chunk's own emission), so a straight-line chunk ENTERS and EXITS at
  scratch zero and seq-grain facts compose at `(xargs locals)`
  register files on both ends — the V2-5a loop-boundary invariant in
  sequence position. Adapters: `iw_sout` (INorm ↔ ONorm at the ENTRY
  stack — inline code has no labels; statements are stack-neutral)
  and `ix_sout` (INorm ↔ XNorm∘xargs, NO residue argument — the seal
  resolves scratch by construction, ix_out's `ra` dance has no
  seq-grain analog).
- **The claim form (probe-validated FIRST-TRY on both targets before
  the generator was written — examples/sqw_probe/sqx_probe, kept as
  corpus pins):** UNPREMISED, the v2 thesis at statement grain —
  `(eval_seq (S^A c) (MkWModule restfs MSZ) INSTRS locals st m) =
  (iw_sout st (istmts (S^G c2) MSZ BODY locals m))` (x86: xeval_seq
  at the MkRegs literal entry, ix_sout, MkXModule restfs 0 MSZ).
  Locals fully symbolic, stack/module/fuel tails quantified; fuels
  ground towers (G = gcost+4, A = tcost+4 — both engines are linear
  in a straight-line body, no ghost counts, no fuel-death case). The
  proof is the walk's split spine with compute-both leaves: shared
  low-bound guards case-on directly (both engines stick on the SAME
  banded tree); mem-hi guards case-on the IMP spelling (lt a MSZ) and
  bridge to the machine spelling (le (+ a 1) MSZ) by a keyed-rows
  have-pair AT THE ARM'S POLARITY (the hlt-lift + integer-tight cert,
  both directions (rows (goal 1) (hltN 1))); every False arm is a
  compute-closed trap leaf (both machines trap; the adapters
  conflate). Splits DEDUP by scrutinee (all-occurrence rewrites
  resolve every duplicate at its first split).
- **The tier in impgen.** gen_pin now dispatches by shape: loops →
  the loop tier, memful straight-line → the statement tier (sq_pin),
  memless → the scalar fn-grain tier. The statement tier reuses the
  loop tier's lw_stmts walk VERBATIM (raw symbolic locals, symbolic
  memory) — only the emission differs; its ties pin the seq
  translation itself (`iw_stmts` at the pin's kind env / the sealed
  `ix_sstmts`), not imp2w/x_fn (no call boundary — the SysV param
  limit does not apply, so the 12-slot pins are fine). Pin carrier:
  nullary IFn fns in the sibling (it_shround/shcopy/shext_fn, 12×U32
  params, extras Nil, result unused) + one IProg wrapper announcing
  memsize 65536.
- **The x86 rotation fence stands (V2-2b):** round/ext x86 legs are
  tie-plus-note (the ix_rot shl leg wraps 2^64 mid-tree; imp's band
  IRotr doesn't spell it). The named growth that unlocks them is the
  native 32-bit rotate instruction (model + encoding, byte-emit
  Opus-delegated) — a user decision point, not a momentum fix.
- Outputs: std/sha256/impgen_{wasm,x86}_out.shard (in-module
  residence — the pins are private; OUT must be SRC's sibling). wasm:
  3 ties + 3 FULL bridges (round 16 splits / copy 16 / ext 32; the
  ext spine re-walks a 306-instruction body per arm and checks fine —
  the phase-emission mitigation stays unspent). x86: 3 ties + the
  copy bridge + 2 rotation notes. EVERY generated claim kernel-green
  FIRST TRY (394/0 + 663/0); regen deterministic; the four
  pre-existing outs byte-stable; driver 64 green (4 new products);
  corpus grows 4 targets (2 outs + 2 probes), FAIL-set identical.
- NEXT: the mixed tier (the block body: seq segments chained around
  generic-worker citations at the three embedded IWhiles — PIN-A
  fuel chaining machine-side), the x86 rotation decision (native ror
  = the round/ext x86 unlock), then I2e (./sha256sum).

**V2-5e-4 — the mixed tier: the block leg, COMPOSITIONAL (segments
around counting loops at seq grain).** impgen's fourth bridge tier;
the sha block body gets its generated wasm leg. Two designs were
built; the first is a recorded dead end with a measured wall.

- **The monolithic dead end (measured, then replaced).** The first
  leg emission told the whole block as ONE claim: per-statement
  segment peels, loop-head blocks, exit spines — one 800KB proof,
  237 case-ons on a single spine. It was UNCHECKABLE: the fast
  engine heap-exhausts its 64GB reserve ~50min in. Bisection
  (truncate-the-spine series) measured LINEAR RSS accumulation
  ~90-330MB per case-on level: per-step allocation churn scales
  with the goal (the whole-block machine code + 12 sha-scale locals
  carried through ~2000 steps), and the non-moving GC's high-water
  mark never comes back down. Probe-scale greenness (K=2, ~30-instr
  bodies) said nothing about engine capacity at sha scale (K=64,
  ~1500 instrs). The claim FORM was fine; the PROOF SHAPE was not.
- **The compositional leg (user-ruled option A; landed).** The
  public claim is UNCHANGED — the statement-tier form verbatim over
  the whole body. The proof decomposes at PIECE grain: the body
  slices into flat SEGMENTS and LOOP PIECES (a loop piece = the
  maximal trailing const-set run + its IWhile — the counter rides
  with its loop, so the ground count is restored by a CONCRETE set
  over the previous piece's exposed case-vars). Between pieces the
  states are OPAQUE case variables; goals stay piece-sized. The
  three legs of the decomposition:
  - `ist_seam` (models/imp/to_wasm.shard, proven once): the imp
    append seam — istmts over (isapp A B) at (lg_fuel (istn A)
    (S f)) equals ist_cont of the standalone A-run, flat-A-premised
    (ISet/IStore never touch the reservoir past the peel; loops
    never cross a seam), standalone tail quantified so segment
    lemmas align by slack instantiation. Rides lg_snoc (loopkit):
    lg_fuel k (S c) = S (lg_fuel k c).
  - **Open-spine segment lemmas** (generated, `sqs_<nm>_s<b>`): the
    machine walk of a segment's instrs followed by ANY tail rw =
    `es_scont` (iw_out's sibling at open-tail code grain) of the
    standalone imp run. Machine code as a Cons-spine ending in the
    variable rw — the worker-locals trick at code grain — so NO
    machine append law is needed. Fuel (S^ kW (S f)): eval_instr
    peels its own S; the adapter continues at (S f). Proof = the
    statement-tier split spine (honest mem guards, keyed-rows
    hlt/hle pairs) with es_scont stopped on mid computes, open at
    trap leaves and the final leaf. Segments are CAPPED (~24 stmts)
    — per-split churn grows with the accumulated state, so big
    segments split into adjacent MxPS pieces (the composition
    handles seg-seg boundaries identically).
  - **Per-segment machine chunks by INVOKE**: the emitter evaluates
    (iw_stmts ks segstmts) through the translator module per
    segment — the translator cuts its own chunks; no spine-position
    arithmetic (the q=2 const-run at the sha rounds loop broke the
    positional cut and was the bug that forced this).
  - **The composition CHAIN** (`cmp_<nm>_b<N>`, boundary 0 = the
    public `imp_w_<nm>`): one statement-tier-form lemma per piece
    boundary, emitted deepest-first, each proving ITS piece and
    citing the next boundary's lemma at the exit leaf. (A single
    nested composition claim was built first and checked green,
    but nested ~500 deep — shardfmt is quadratic in nesting depth,
    319s/115MB — so the chain replaced it; nesting stays
    piece-deep.) Chain citations need `(inst c2 c2)` — the cited
    lemma's imp fuel appears only on its RHS, a dangling pivot —
    plus a boundary-stopped compute after the rewrite so iw_sout
    unfolds to the match form the goal carries. Per segment piece —
    isapp respell-have + fuel reshape-have + ist_seam cite (inst g)
    + segment-lemma cite + ONE case-on of the SHARED standalone
    istmts run (None/ITrap conflate through the adapters) +
    il_slen arity exposure of the INorm case-vars; per loop piece —
    concrete peel compute (walkers OPEN, loop machinery + rest fns
    stopped) + the packed int_of_nat ladder + counter respell +
    per-side reshape haves ((S^ (gm−q−2) c2) = (S^ Bw (lg_fuel K
    (S^ si c2))), machine at fm−2q−4 respelled with Aw = wlen+4
    OUTSIDE — the worker's own machine tower, so the cite matches)
    + the counter-tied worker cite + ONE case-on of the SHARED
    iwhile + il_wlen exposure. Machine tails fold behind
    gb_<nm>_r<b>_w rest fns (empty rests spell Nil); the machs
    stream advances only on LOOP pieces (segments consume the msls
    stream). Fuels A/G from per-loop MAX constraints; reshape
    haves absorb any oversupply. Boundary computes stop BOTH
    walkers (istmts, eval_seq) + adapters; loop-peel computes
    leave walkers open — the stop-set split (cstop/pstop/lstop)
    is load-bearing.
  - **The flat list spelling** (`spl_e`): shardfmt indents a Cons
    spine one level per element — quadratic bytes in list length
    (the ~1500-instr block tie alone formatted to 50MB). All
    closed (Nil-terminated) code lists in generated files —
    ties, gb fn bodies, worker bodies, rest fns — now spell as the
    flat `(list …)` sugar (the same term after desugar;
    proof-neutral; canon preserves it). Open spines (the segment
    lemmas' variable-tail trick) keep Cons spelling — they are
    capped at ~30 elements. The block tie: 50MB → 63KB.
- **The counter-tied workers, tie, and wid twin are UNCHANGED from
  the monolithic build** (they were never the problem — all green
  at MB-scale from the start).
- **Measured**: the sha wasm out (tie + wid + 3 workers + 13 capped
  segment lemmas + the 15-lemma composition chain) checks 430/0 in
  6s at 645MB peak — against the monolithic 64GB trap at ~50min.
  The committed artifact is 11MB formatted (1.2MB raw; the residue
  is case-ladder indentation — the cmp exposure ladders ~4.4MB,
  segment lemmas ~2.3MB, the rounds-loop worker ~1.3MB — a fmt
  depth cost, not a spelling one; fmt runs 31s). The dev pin
  (examples/imp_mixed.shard → impgen_wasm_mixed_out) mirrors
  examples/sqmc_probe.shard (the hand-proven blueprint, corpus pin)
  at 120/0 / 4MB / 204ms.
- **Dispatch**: has_while + lp_pieces-fits → loop tier; has_while
  otherwise → mixed; memful straight-line → statement tier; memless
  → scalar. x86 → tie + honest note (the sha block refuses at the
  ext body's IRotr with the V2-2b rotation message; the native
  32-bit rotate = the approved next slice).
- Outputs: std/sha256/impgen_{wasm,x86}_out.shard regenerate
  deterministically; all four pre-existing example outs re-landed
  under the flat list spelling (ties respelled — proof-neutral,
  the same claims); the imp_mixed pins registered as impgen DERIVE
  products + corpus targets; probes sqmw/sqm2/sqmc ride the
  corpus.
- NEXT: the native x86 rotate slice (model growth approved
  2026-07-15 — unlocks the round/ext/block x86 legs), then I2e
  (./sha256sum: weld = hand block walk ∘ imp_w_shblock).

**V2-6 — the native rotate (2026-07-15; two commits: V2-6a the
operand-band amendment, V2-6b the instruction + the fence lift).**
The slice opened on a FALSE-LEMMA FINDING that supersedes V2-2b's
unlock plan: a hardware 32-bit rotate reads only the low 32 bits of
its register, while IRotr banded only its RESULT — on the raw
symbolic locals of the statement-tier claim forms (empty premise
lists, checked at impgen_x86_out's copy bridge), an unpremised
bridge over a truncating native rotate is FALSE, not merely
non-refl (x0 = 2^32, count 2: imp's shr leg drags bit 32 into the
band window where silicon reads 0). Refl-grade tree identity is the
ONLY premise-free mechanism at raw locals — a semantic rewrite
(e.g. discharging the old composition's wrap64 mid-tree by
absorption) needs nonneg obligations and re-admits the premise
apparatus.

- **The amendment (V2-6a, user-ruled including the spec deviation):
  rotation bands its OPERAND at width, both legs, at every tier.**
  models/imp IRotr (complement stays double-modded), models/wasm
  BRotr at both widths (identity on typed values — V8 287/0
  unchanged), std/sha256's private rotr32 (m32 on the operand;
  value-identical on all in-band input, NIST vector pins re-verify,
  zero external symbolic consumers), tools/impgen rot_val (mirror
  hygiene — no committed artifact spells rotation trees; all 8 outs
  regen byte-stable under V2-6a alone). Aligning the SPEC too is
  what collapsed the sibling surgery: with spec ≡ imp ≡ machine
  trees, the pins stay premise-free lockstep-refl and the
  pass/worker/walk premise plumbing (incl. the block walk's 57
  positional farkas certs) re-fired UNTOUCHED. The paid cost: a
  mechanical respell of 117 stated rotation trees to banded-operand
  form + 20 ladder discharges redirected through band_lo in the ten
  leaf bounds lemmas (ish_rotr7/17/18/19_lo, ish_rt2/6/11/13/22/25
  _lo) — sibling 357/0. std/bits grew mask_word32/mask_word64 (the
  mask_byte precedent at word widths — proved inside the module
  where pow2 grounds; the band↔mod seam any rotation consumer
  needs); the imp_scalar rotate twins collapse their operand bands
  through them.
- **The instruction (V2-6b): (XRorI32 Reg Int)** — ror r/m32, imm8,
  immediate-count only (the XShlI32 precedent). Semantics
  Fable-side spell EXACTLY the amended IRotr-at-U32 tree (operand
  band both legs, double-modded complement, result band32) —
  honest to hardware at every register value, so the model change
  and the bridge doctrine stop being in tension. Encoding + vectors
  + silicon Opus-delegated per the standing split: C1 /1 ib (the
  XShlI32 arm at reg field /1), +11 vectors (count boundaries
  0/1/31/32/33, high-bit wraparound, OPERAND-TRUNCATION witnesses
  with bits ≥ 32 set, zero-extension, the REX.B r11 leg) — silicon
  123 agree / 0 disagree, re-run first-hand.
- **to_x86**: IRotr U32 emits the single XRorI32 in both the ix_acc
  and ix_set arms; the five-instruction ix_rot composition through
  R11 RETIRED with the scoped non-refl it carried. xtie_rotr7
  re-ties at two instructions; imp_x_rotr7 = the once-impossible
  bridge, plain refl first-try. U64 rotation still refuses (the
  composition's algebra never scaled to the register width; native
  ror r/m64 is trivial growth behind a consumer).
- **The impgen fence lift**: the two x86 rotation refusals in the
  walkers (se_exp/lw_exp) retire — rotation walks identically on
  both targets. THE SLICE'S ONE DEBUG (the new-ctor stuck-scan
  class): the sha x86 regen refused with "spell: unmapped ctor
  XRorI32" — to_x86's zero-scratch write-scan ix_dwl matches every
  XInstr ctor explicitly and had no XRorI32 arm, so the SEAL
  computation stuck and the reflected translated value carried the
  rotate ctors under a stuck Call node (harvest walks Ctor args
  only; sp_e spells through non-Ctor nodes — hence the unmapped
  miss at spl_e's leaf delegation). Site isolated with a minimal
  statement-tier rotation probe + per-site error tags. RULE
  RECORDED: growing an ISA type means sweeping its EXPLICIT
  matchers (grep the last-added ctor); world.shard's walkers use
  catch-all arms and were safe, encode.shard was the Opus arm.
- **Outcome**: std/sha256/impgen_x86_out.shard goes from tie+note
  (689/0, 162KB) to tie + THREE FULL statement-tier bridges —
  imp_x_shround / imp_x_shcopy / imp_x_shext — 691/0 at 1.66MB
  (the 51-stmt round and 79-stmt ext walks with their guard splits
  check on the first generated attempt); the block pin's note
  flips from the rotation message to the honest remaining gap
  ("x86 mixed-tier emission (named growth)"). examples
  impgen_x86_out gains the generated tie_x_rotr7 + imp_x_rotr7
  (418/0); wasm/loop/mixed outs byte-stable through both phases.
- Gates: driver 69 products green ×2 (per phase); corpus FAIL-set
  identical to baseline-65 ×2; V8 287/0; silicon 112→123/0.
  NEXT: the x86 mixed-tier emission (unlocks the block leg), I2e.

**V2-7 — the x86 mixed tier (2026-07-15; two commits: V2-7a the
foundation, V2-7b the emission): the mixed tier lands on the register
machine; the sha block leg is generated and piece-sound but FENCED on
a measured engine-capacity wall at the finish segment.** The wasm
composition (V2-5e-4) factors segments into standalone lemmas joined
by ist_seam/es_scont; that factoring cannot cross to x86 without the
generator learning machine semantics — a segment lemma's continuation
must RECONSTRUCT the register file, and the three scratch registers
carry concrete trees between pieces that only the checker should
compute. The x86 design (hand-validated first in
examples/sqxc_probe.shard, 386/0, the compositional blueprint over
imp_mixed's program):

- **Loop-exit cuts, inline segment walks.** One chain lemma per
  [flat segments + counting loop]; the lemma walks its segments
  INLINE by the statement-tier split spine (scratch states stay
  concrete all the way to the stuck loop machinery — no x86 append
  seam exists or is needed at this grain), cites the counter-tied
  worker at the loop head, exposes the exit locals by il_wlen, and
  cites the next boundary lemma at the exposure leaf, where the
  continuation arm holds the next rest-fn's FOLDED CALL protected by
  the walker stops. An open-walker compute past a braked call leaves
  match residue no folded statement can cite (measured: the first
  emission walked boundary tails inline and failed exactly there) —
  so tails beyond a loop belong to the NEXT lemma, and boundary
  lemmas state their subjects AS the folded rest-fn calls, opening
  their own proofs with the unfold (the reduce overshoot is harmless
  at a proof's start).
- **Scratch discipline.** ix_mout (to_x86.shard) = the mixed-grain
  loop-exit adapter — ix_out's sibling carrying all three scratch
  residues (ix_out ra = ix_mout ra 0 0; the fn-grain loop tier's
  contract is untouched). Boundary lemmas quantify exactly the FULL
  body's seal-set registers (the terminal ix_zs zeroes them before
  the final XNorm; a non-seal-set register never gets written, so
  quantifying it would state a falsehood — it spells 0). Workers
  quantify the slots their body never touches (pass-through,
  restored by ix_mout) and spell 0 in body-dirtied slots (the V2-5a
  seals). Every unknown tree binds by MATCHING at cite sites — the
  emitter never spells a scratch value it did not put there (the
  probe witnesses a store's band tree riding za THROUGH the
  scratch-clean loop 1 and dying at the next leg's zs).
- **The instruction-grain counter respell.** The wasm chain respells
  the ground counter in the locals list; x86 has a second occurrence
  inside MkRegs whose neighbor slots are unspellable walked trees.
  Resolution: respell BEFORE the walk, at the instruction —
  (XMovRI reg K) / (ISet gi (IConst K)) rewritten to the
  (int_of_nat K) payload at first occurrence (single in the subject
  because deeper legs are folded behind their rest fns) — so the
  walk itself carries the worker's counter spelling into the stuck
  loop term. XMovRI wraps its immediate at 2^64, so the folded
  payload surfaces as (mod (int_of_nat K) 2^64): a per-file
  sqm_wid64 twin (iwrap64_id verbatim) collapses it at the loop
  head.
- **Workers** (sqxw_<nm>_lN): the counter-tied form at x86 — closed
  MkRegs/locals (xargs walks the whole list at the exit adapter, so
  the wasm open-tail rr trick cannot cross), per-iteration store
  splits at the statement-tier polarity dance, decrement collapse
  through the wid twin, IH at (inst c2 c2). aw = machine body
  length + 4; bw = gcost + 5 (the imp side is target-shared).
- **Emitter** (tools/impgen, all Fable-side — no new instructions or
  encodings): mxx_mach (XBlock/XLoop scan), mxx_chunks (per-segment
  ix_stmts by INVOKE), mxx_dirty (scratch scan by INVOKING ix_dwl —
  the model answers, never a generator-side scan), mxx_legs
  (loop-exit grammar), mxx_ag (per-loop MAX fuel constraints),
  per-leg lw_stmts re-walks from fresh params (splits + walked-tree
  scrutinees: the case-on iwhile term carries the walked locals and
  mem_set chain), mxc_exit reused verbatim (Doc-parametric),
  rest fns as Cons spines ending in the next rest fn's call, the
  tie re-pinned as (ix_sstmts (ibody_of pin)) = (Some (inline
  gb_<nm>_x)) so it also binds the gb chain to the translator.
- **THE CAPACITY WALL (measured, then fenced).** Per-lemma churn
  grows with the leg's straight-line span: goal size × steps, with
  the walked mem_set chain and local trees renormalized at every
  split — and unlike the wasm factoring, an x86 chain lemma CANNOT
  reset that state mid-segment (the wasm segment lemmas each start
  from a fresh opaque memory and locals; an x86 mid-segment boundary
  would need scratch-clean cut points that the emitted code does not
  have). Focus-mode measurements on the generated sha block leg:
  all three counter-tied workers 400MB each and green (the round
  worker's 51-stmt body included — per-iteration state resets at the
  IH); the 79-stmt shext statement bridge costs 3.4GB; the FINISH
  leg (cmp_x_shblock_b3: ~150 straight-line stmts, 32 byte-stores of
  word-scale trees) exceeds 66GB against the fast engine's 64GB
  reserve. The fence: sqx_emit declines pins whose max leg span
  exceeds 96 stmts with an honest note ("x86 chain capacity: a
  straight-line span exceeds the per-lemma churn budget
  (state-reset boundaries = named growth)"). UNLOCK OPTIONS (user
  ruling pending): (a) translator seal-points — to_x86 emits ix_zs
  at capped intervals in TOP-LEVEL straight-line runs only (never
  inside loop bodies; for sha ≈ 40 extra movs once per block,
  ~0.1%), giving the x86 chain scratch-clean boundaries and the
  full wasm-style capped segment-lemma factoring; (b) the engine
  churn/reify-sharing arc; (c) reserve bump (rejected class — the
  driver inherits the cost).
- **Outcome**: examples/impgen_x86_mixed_out.shard goes tie+note →
  the FULL inventory (tie + wid + wid64 + 2 counter-tied workers +
  rest fns + cmp_x chain + imp_x_imx), 410/0, regen deterministic —
  the x86 mixed tier is real and generating. The sha block pin:
  tie + the capacity note (round/copy/ext x86 bridges stand FULL
  from V2-6; the block's generated chain is retained nowhere — it
  regenerates identically the day the fence lifts). All other outs
  byte-stable (sha wasm out included); driver 69; corpus FAIL-set
  == baseline-65.
- NEXT: the seal-point ruling (unlocks the block x86 leg), then I2e
  (./sha256sum — weld = hand block walk ∘ imp_w_shblock; the wasm
  block leg carries it meanwhile).

**V2-8 — translator seal-points: the sha block x86 leg goes FULL
(2026-07-15/16; two commits: V2-8a the foundation, V2-8b the
emission).** The user-ruled unlock (option (a) of the V2-7 fence): the
translator gives long straight-line code scratch-clean state-reset
boundaries, and the x86 chain factoring rides them.

- **V2-8a — the chunk grid + the blueprint.** to_x86 grows
  ix_chunks/ix_capq/ix_cjoin/ix_cstmts: a top-level statement list
  chunks into maximal flat runs of at most 24 statements (const-set
  runs never straddle a cut — the mixed tier's loop pieces pop them
  off a chunk tail; loops/branches are singleton chunks; loop bodies
  are NOT chunked — the V2-5a entry/back-edge seals already
  discipline them, and mid-body seals would cost every iteration).
  Between two consecutive FLAT chunks the emission inserts the
  leading chunk's own ix_zs, so straight-line code crosses a
  scratch-clean boundary at least every 24 statements (~0.1% sha
  cost; bodies whose runs stay within the cap emit byte-identically).
  The append seam family (isapp/istn/ist_flat/ist_cont/ist_seam)
  relocated from to_wasm to models/imp — it is target-neutral and
  both legs now cite it. xs_scont = the seq-grain continuation
  adapter. examples/sqxc_probe.shard grew the validated blueprint:
  sqxc_sl1 (the open-tail seal-point sub-lemma — a 26-stmt body
  chunks 24+2, the sub-lemma walks chunk 1 THROUGH ITS SEAL and hands
  ANY tail rw to xs_scont) + sqxc_lscomp (ist_seam splits the imp
  side at the seal point, the sub-lemma cite rewrites the machine
  side, ONE case-on of the SHARED standalone run absorbs every
  outcome, il_slen exposes the segment exit, the tail walks inline).
- **V2-8b — the emission; the V2-7 finding re-measured.** The first
  V2-8b attempt cut the chain at chunk boundaries and let each
  mid-flat lemma WALK its chunk inline, citing the next boundary at
  the walked tail — and failed exactly as the V2-7 record warned: an
  open-walker compute past the folded rest-fn call leaves match
  residue no lemma can cite, and a clean call-grain park exists only
  where a STUCK SCRUTINEE freezes the continuation arm (the stopped
  iwhile at loop exits; the opaque exposed run at seal points). So a
  mid-flat leg never walks its machine side at all: the walk lives in
  the leg's OPEN-TAIL sub-lemma (sqs_x_<nm>_c<B>: the chunk's instrs
  through its seal over ANY tail rw = xm_scont of the standalone imp
  run; unpremised; splits from the chunk's own lw_stmts walk at fresh
  locals), and the chain lemma only composes — machine-side-only
  opener, hsp (the folded rest-fn call = isapp of the closed chunk fn
  and the NEXT folded rest call: no spelled spines anywhere),
  hfg/ist_seam/hfb at exact fuel counts, the sub-lemma cite, the
  shared-run case-on, il_slen exposure (mxc_exit reused verbatim,
  Doc-parametric), and the next boundary cited at the parked leaf,
  where the xeval_seq stop keeps the machine side at the folded call.
- **xm_scont** (to_x86) = xs_scont carrying the three scratch
  residues (ix_mout's seq-grain sibling; xs_scont = xm_scont at zero
  residues): a chunk's seal re-zeroes exactly what the chunk wrote —
  those slots continue at 0 — while scratch the chunk never touches
  PASSES THROUGH and the residue restores it at the continuation's
  register file. Residues are read off the model's own ix_zs answer
  (mxx_zflags scans the invoked seal — the seal IS the dirt set) and
  bind BY MATCHING at cite sites, never by spelling.
- **Leg grammar** (tools/impgen): boundary lemmas at every interior
  seal AND every loop exit; rest fns cut at both. The chunk before a
  loop stays IN the loop's lemma (no seal separates them — its dirt
  rides into the loop construct and ix_mout restores it), so loop
  legs are V2-7 VERBATIM, and the entire V2-7 fuel arithmetic
  survives with seal-aware widths (mxx_segm counts interior seals;
  every formula holds with the new W). mxx_chunks invokes ix_stmts
  AND ix_zs per chunk (the model answers; the slicer's mxc_capq and
  the translator's ix_capq deliberately spell the same cap
  discipline). Single-chunk pins emit byte-identically to V2-7 — the
  imx dev pin is byte-stable.
- **Dev fixture**: it_imxl_fn (examples/imp_mixed.shard) = two
  26-stmt spans crossing the seal grid. Covers: the b=0 scratch-clean
  sub-lemma; the b≥1 sub-lemma entered with a band tree riding za
  (the pre-loop chunk's unsealed store dirt rides the dirt-free loop
  — the pass-through witness at seal grain); the loop leg whose chunk
  is not the span head; the ground leg behind the terminal seal. x86
  out 421/0; wasm out 132/0 (the wasm emitter generates the new pin
  through its own capped factoring, untouched).
- **THE WALL FALLS**: the sha block x86 leg generates FULL — tie +
  wid twins + 3 counter-tied workers + 13 rest fns + 10 seal-point
  sub-lemmas + 14 boundary lemmas (cmp_x_shblock_b13..b1 +
  imp_x_shblock) — and the whole sha x86 out checks 721/0 at ~1.2GB
  peak RSS, against V2-7's measured >66GB for the single-lemma finish
  leg alone. The interior seals also shrink the STATEMENT-tier
  bridges (shext measured 3.4GB in V2-7; the whole regenerated file
  now peaks under half that) — a seal kills the accumulated scratch
  trees mid-walk. The mxx_maxspan fence is DELETED; no tie+note path
  remains in the x86 mixed tier.
- Gates: all other outs byte-stable at canonical raw paths; driver 69
  green; corpus FAIL-set == baseline-65.
- NEXT: I2e (./sha256sum — weld = hand block walk ∘ imp_w_shblock;
  both block legs now stand FULL).

**I2e-1 — THE BLOCK WELD (2026-07-16): one compression block's
MACHINE code lands on spec shapes, both targets.**
std/sha256/sha256.weld.shard composes the generated ISA legs with the
hand walk: `shw_wblock` (eval_seq of the wasm block code from any
stack/locals frame at the walk's layout = ONorm of the named output
value) and `shw_xblock` (xeval_seq of the x86 block code from the
layout realized as the SysV register file = XNorm through xargs' home
mapping), each premised exactly on the walk's three layout facts
(0 ≤ src, src+64 ≤ wb, wb+544 ≤ 65536). File checks 801/0; both weld
claims green on the first structurally-complete draft.

- **The named output value**: shw_sm (the schedule memory =
  sched_mem ∘ copy_wmem), shw_r (the rounds output over H/K/W window
  reads), shw_locals (the eight projections + re-zeroed pointers),
  shw_mem (h8add_mem at the H window) — spelled exactly as
  isha_block_walk's RHS trees and folded into the weld statements by
  defining-equation RL rewrites, so the I2e driver composition speaks
  four calls instead of ~300-line trees.
- **The spelling-bridge measurement (examples/weld_probe.shard,
  corpus-pinned)**: the rewriter matches SYNTACTICALLY, never modulo
  computation — a walk lemma stated over the sibling's SOURCE
  spelling (builder calls) cannot be cited on a goal holding the same
  body's NORMAL FORM (the wasm out's flat spelling) or the x86 out's
  chunk-chain spelling. Since every body position is closed ground
  data, the weld carries byte-copies of all three spellings as local
  nullary fns ((inline …) is file-local — the I1b bridge-file
  precedent) and crosses by compute-both bridge equations that meet
  at the shared normal form under (stop istmts). The probe pins the
  consumer recipe: bridge RL first, then the walk cites.
- **The composition chain** (both targets, symmetric): unfold the
  out's nullary code fn (exposes the spliced-source term the out
  claim's LHS pattern needs) → cite imp_w/imp_x_shblock with
  (inst c2 (S^ 103 d)) — the dangling-pivot fuel alignment,
  (S^ 297 (S^ 103 d)) IS (S^ 400 d) as a tree → spelling bridge RL →
  cite isha_block_walk with the three premises discharged from the
  weld's own → fold the output vocabulary RL → one stopped compute
  reduces the exit adapter (x86: shw_locals stays UNFOLDED there —
  xargs needs the literal list structure to reduce; the projections
  ride folded behind stops) → refl.
- Registration: weld = 'check imp product (driver 70) + corpus
  target; probe = corpus target.
- NEXT: I2e-2 (padding + digest readback + K/H window init — the
  K-window wlist premise discharges by computation once an init twin
  writes sha_k into the K window), then I2e-3/I2e-4 per the
  composition ruling below.

**The I2e composition ruling (2026-07-16, user-ratified).** The
sha256sum bin organizes as NESTED GRAINS with the call boundary as
the composition primitive at every layer — the bin-organization
precedent for shard binaries generally:

- **main = World, THIN** (addw X86.md §48 machinery, two calls → a
  few): read stdin, call the artifacts, write the digest. Compute
  never lives in the I/O proof (the BIN-BOUNDARY LAW and the
  req-fulfills principle both want main's premise surface = exactly
  the glue contract).
- **The blocks loop = a PURE artifact (I2e-3)**: a Nat-counted loop
  XCalling the block per iteration — X86.md §18 / wasm §6ac's
  species with an impgen-generated callee. xcall_bridge's callee
  slot is discharged by exactly shw_xblock's species (xeval_seq of
  the body from the caller's registers), so the lift = wrap the
  block statement list as an XFunc + one adapter lemma. Hand weld,
  probe first — and the measured blueprint for the coverage arc's
  icall tier (the redirection above names call = icall as the
  uniform compiler's primitive; I2e-3 is its hand-built precedent).
- **v1 SLURPS, streaming = named growth**: stdin buffered into the
  model window (cap ≈ 64KiB minus schedule/state overhead); oversize
  input = a controlled-failure leg (MEMORY.md's Done ∨ Fail-prefix
  claim forms), never a silent wrong answer. A World-loop main
  interleaving reads with hash calls (cat_loop's clock-discipline
  species) is the honest streaming form — a future rung, not v1.
- The capacity rationale, once more: boundaries scale, monoliths
  don't (64GB monolith vs 645MB compositional; >66GB wall vs 1.2GB
  seal-points). The loop proven inline in main's World cert would be
  the monolith shape again.

**I2e-2a — THE COLLAPSE (2026-07-16): the weld output lands on
sha_block.** With the K window holding sha_k and the H window holding
the running state's words, the weld's named outputs collapse to spec
vocabulary — the three memory facts the blocks-loop driver chains,
all in std/sha256/sha256.weld.shard (closure 838/0):

- `shw_out_h`: the exit memory's H window reads back as
  `shw_hlist (sha_block (mk_h8 y0…y7) (mem_read 64 m src))` — the
  per-block step in one equation. `shw_out_k`: the K window persists
  (the constants survive every block). `shw_out_lo`: every window
  below wb persists (the message region survives). `shw_r_sha`: the
  rounds output = `sha_rounds` of the state over `sha_k` and
  `sha_sched` of the block's bytes (rides `isha_sched_window` +
  `ish_words16_read`, both pre-existing).
- **The sibling grew three I2e-2 kits** (sha256.imp.shard, 387/0):
  the below-side frame mirrors (`ish_wget/wlist_sched_below`,
  `ish_wlist_copy_below` — the message region reads across the block
  effect); the ROUNDS RANGE KIT (`slo`/`h8_los` predicates,
  `ish_rnd_los`/`ish_srounds_los` preservation citing the V2-6
  tree-identical `ish_ra_lo`/`ish_re_lo`, projection-style
  extractions `ish_h8los_a..h` — list equations extract by `blnth`
  projection, NO Bool-chain inversion); and the FINISH READBACK
  (`ish_h8add_g0..g7` per-lane + `ish_h8add_read` assembly +
  `ish_wget/wlist_h8add_below` — every address obligation is
  `ish_d34` stretched by `ish_lt_b4`/`ish_le_b4`, zero arith rows).
- **The band/mod crossing**: h8add_mem stores mod-spelled sums, the
  spec's h8_add is band-spelled (m32); `ish_mm32` bridges under
  nonnegativity — the state side free from `ish_wget_range_lo`
  (mask-on-read), the rounds side from the range kit. The W-list slo
  fact converts through `shw_w_sched`'s equation to wlist form where
  it is free — no slo-of-sha_sched induction needed.
- Proof-mechanics findings: `int_of_nat` is OPAQUE — ground
  occurrences in citation obligations collapse via `ish_i16p/48p/64p`
  (packed) or `ish_i16/i48` (tower), never by compute; the
  tower-vs-packed ground spelling gap (isha_sched_window spells
  `(S^ 16 Z)`, the weld spells packed `16`) bridges by a compute-both
  have under stops (the I2e-1 spelling-bridge species, ground-Nat
  edition); keyed cert rows `(rows (goal 1) (0 1) (hname 1))` keep
  have-heavy proofs insertion-stable.
- NEXT: I2e-2b (the K/H init artifact — an imp source whose
  generated legs write sha_k + sha_h0 into the windows, discharging
  shw_r_sha's content premises by computation), then padding +
  digest-hex artifacts, then I2e-3 per the ruling above.

**I2e-2b — THE K/H INIT ARTIFACT + THE GROUND-FOLD RESOLUTION
(2026-07-16): the init legs land, and the walker learns normal
forms.** The K/H window init (72 words: sha_k into the K window at
65216, sha_h0 into the H window) is the first GROUND-ADDRESS program
through the statement tier — a running pointer walked by U32
decrements over 577 straight-line statements — and it exposed the
first real scaling failure of the generated-proof layer:

- **The quadratic.** The statement-tier walker tracked locals
  symbolically with NO constant folding, so after k decrements the
  pointer spelled as a k-deep `(mod (- … 1) 4294967296)` tower, and
  every store's bounds case-on spelled the full accumulated address:
  the generated `imp_w_shinit`/`imp_x_shinit` claims came out 5.9MB
  EACH (84% of the leg file, 186,624 `mod (-` layers), shardfmt
  ground 83+ minutes without producing output — and the proofs were
  UNCHECKABLE anyway: ground guards compute through and never stick,
  so the case-on hyp rewrites (must-apply) had no occurrences. The
  walk contract ("emitted terms are the stuck terms compute leaves")
  already ruled the spelling wrong: ground is never stuck.
- **The fix (impgen, the walker):** `sc2` folds ground arith at
  spelling time (both operands literal → compute; the SAME total
  primitives the checker computes, so the fold is identity by
  construction); ground comparisons follow the known arm
  (`se_cond_cmp` → CFollow, the se_cond_val precedent); ground
  in-bounds addresses elide their guards (`lws_addr` — the checks
  compute through); ground out-of-bounds refuses (the program traps —
  there is no refinement claim to state); a ground load through a
  store chain refuses (the lookup computes through; the chain-lookup
  mirror is named growth). Regen is byte-identical outside the shinit
  claims on BOTH targets (the fold never activates on the symbolic
  legs — ground-ground arith could not have existed in green legs).
  Effect: the shinit claim collapsed 5,945,809 → 473 bytes, proof =
  compute-both + refl; the legs format in ~30s each (was
  unfinishable); the weld closure checks 857/0 in 15s.
- **The artifact content:** the sibling grew the init kit
  (`ikh_h0`/`ikh_kh` tables, `ikh_wstmts` — the BE store walk, LSB
  first down from base+3, matching wput's nesting — `ikh_body`,
  `ikh_mem`, the setw/read lemmas, `isha_init_walk`); the weld grew
  the spelling bridges (`shi_src_body`/`shi_nf_body`/`shi_bridge`)
  and the machine init runs (`shi_winit` at eval fuel S^2024,
  `shi_xinit` at xeval fuel S^583, both at istmts fuel S^583 with the
  c2 := (S^ 17 d) alignment). Weld gotcha: the wasm out is imported
  SELECTIVELY — new generated names need their own use lines
  (`gb_shinit_w`, `imp_w_shinit`).
- **THE STANDING SCALING RULES (user-ratified 2026-07-16).** The
  proof-burden-linear-in-code goal survives; every observed quadratic
  came from one root: substitution-style symbolic execution over an
  unbounded straight-line segment. The rules: (i) NORMAL-FORM
  SPELLING — no walker may spell a tracked value by its construction
  history (ground folds now; named intermediates are the symbolic
  analog if ever needed); (ii) SEGMENT BOUNDS — every tier must bound
  proof-text and goal growth in segment length (the translator
  seal-points are the mixed-tier instance, measured >66GB → 1.2GB;
  the statement tier's envelope is short-or-ground bodies until it
  grows its own bound); (iii) DATA-AS-DATA (named growth, decision
  owed) — table inits belong to a loop over the table or a
  module-level data segment (wasm data segments / x86 .rodata), not
  to unrolled store code; the K/H init as 577 unrolled statements is
  the founding cautionary case, and I2e-2d's nibble table is the
  first prospective consumer.
- NEXT: I2e-2c (the padding artifact — 0x80 + zero-fill + BE length,
  mixed tier), I2e-2d (digest-hex), then I2e-3 per the composition
  ruling.

**I2e-2c — THE PADDING ARTIFACT (2026-07-16): sha_pad lands on both
targets through the existing mixed tier, zero generator growth.** The
pad body (std/sha256/sha256.imp.shard `ipd_body`, pin `it_shpad_fn`,
5 locals src/n/t/z/l with l at U64) appends sha_pad's suffix in
place; the design keeps every loop count ground:

- **The always-72 zero fill.** The zero span `(55−n) mod 64` is at
  most 63, so the body unconditionally zero-fills the ground 72-byte
  maximum pad span at src+n and lets the 0x80 mark and the eight
  length bytes overwrite their positions — the loop is a ground-count
  counting loop (const-set counter, band-decrement), exactly the
  mixed tier's existing species; a symbolic-count loop head would
  have been real emitter growth for one consumer. Surplus zeros land
  either under the length field or above the padded region, below
  the caller's bound; the readback never sees them.
- **Vocabulary choices forced by the targets**: `mod 64` respells as
  `band 63` (IRem refuses on the x86 leg; bridged to the spec's mod
  by `ish_zf` via mask_pow2 at k=6); the bit length computes at U64
  (8·n exact under the 64KiB bound, so the stored byte trees align
  with len_be's shifts refl-grade); byte narrowing spells the
  two-step `(ITrunc U32 U8 (ITrunc U64 U32 …))` — the direct U64→U8
  trunc is a scoped wasm refusal, and the double band bridges to the
  spec's single `band 255` by `ish_bb` (mask conversions + one
  mod-of-divisor collapse). New general lemma `ish_shr_lo`
  (shr-nonneg at any count, wf-induct over bshr_s + ish_ediv2_lo)
  replaced what would otherwise have been 37 more ladder rungs.
- **The sibling kit** (+35 claims, 434/0): `pad_zmem` + framing
  (below/hi/window-below/aligned-zeros-read), the zero-fill
  pass+worker (isha_copy_w's mirror at stride 1), `ipd_mem` +
  the `ipd_*` fold vocabulary (defining-equation RL folds keep the
  walk's trees compact — the I2c-2b fold-back idiom), the full-body
  hand walk `isha_pad_walk` at (S^ 120 d), THE READBACK
  `ish_pad_read` — the padded window over `ipd_mem` reads back as
  `sha_pad` of the message window, counts entering as Nats (wn, wz
  tied to the spec's mod-64 spelling by an equation premise) — and
  `ish_pad_hi` (persistence above the span, the composition's
  framing fact).
- **The weld runs** (sha256.weld.shard 909/0): `shp_wpad`/`shp_xpad`
  land the machine legs on `ipd_mem` on both targets. The pin body
  is its own normal form, so the wasm leg needs NO spelling bridge
  (the walk cites directly on the generated body — first time);
  the x86 out spells its body with a rest-fn cut, bridged by
  `shp_xbridge` (compute-both under stopped istmts, the I2e-1
  species).
- Generated legs: wasm `imp_w_shpad` (chain of 2 boundaries + worker
  + 2 segment lemmas), x86 `imp_x_shpad` (loop-exit-cut chain);
  regen byte-identical on both targets including all pre-existing
  content.
- NEXT: I2e-2d (digest-hex — the nibble table, where the DATA-AS-DATA
  decision owed to the user first bites), then I2e-3 per the
  composition ruling.

**I2e-2d — THE HEX TIER (2026-07-16): digest-hex lands on both
targets, BRANCHLESS — the DATA-AS-DATA decision stays unforced.**

- Design: the nibble table was sidestepped entirely. For a nibble x
  the hex character is `(x + 48) + 39*((x + 6) >> 4)` — the shift
  term is 1 exactly when x >= 10, lifting '0'..'9' onto 'a'..'f'
  with add/shift/and/mul vocabulary only. No IIf, no value-position
  comparison (to_x86 refuses those in value position — SETcc is
  named growth), no data table: both targets in-tier with ZERO
  model/emitter growth (`XMul`/`BMul` were already encoded). The
  DATA-AS-DATA ruling stays owed but nothing here forces it.
- Pin `it_shhex_fn` (6 U32 params: out ptr q, digest ptr p, counter,
  byte, char, scratch): a ground-count loop (32 iterations, the
  mixed tier's const-counter species), per pass one byte load and
  two character stores; scratch re-zeroes each iteration (the
  zero-in/zero-out idiom — exit locals uniform, no residue
  selectors) and a pre-loop const-run zeroes it on entry, so the
  walk holds from arbitrary entry locals.
- The machine tree meets the spec's `hex_digit` by ONE fin-split
  lemma (`ish_hexd`, 16 ground cases, everything computes) — the
  first fin-split use in the sha ladder; it replaces the whole
  mod-collapse/branch-analysis apparatus a symbolic proof would
  need. Byte bounds come free from std/mem's `get_lo`/`get_hi`;
  the hi nibble's `<= 15` runs a descending halving ladder
  (`ish_ediv2_hi` + `ish_shrh4..0`).
- NEW GENERAL LEMMA `ish_shr_add`: `bshr (bshr a j) k = bshr a
  (j+k)` by wf-induct along `bshr_s` (the ish_shr_lo pattern). It
  converts word_bytes' literal-count shifts (24/16) into chained
  8-shifts, where `shr8_div` applies per level — no `div_div`, no
  pow2, no per-count ladder.
- THE DIGEST CROSSING (`ish_wb_read` + `ish_digest_read`): a 4-byte
  window reads back as `word_bytes (wget m a)` (div_unique/
  mod_unique pin every quotient/remainder of the byte sum), and the
  32-byte digest window IS `h8_bytes` of the eight words it holds —
  the input-side glue the I2e-4 composition consumes against the
  block weld's wget equations.
- Readback `ish_hex_read`: the 64-byte output window over the hex
  effect is `bytes_hex` of the 32-byte digest window (aligned
  induction; framing via `ihx_get_below`/`ish_hex_hi` +
  mem_read_set_below). The denotation `ihx_mem` stores hex_digit
  values directly — the branchless-tree conversion happens ONCE per
  pass, where the byte's bounds are at hand.
- Weld: `shh_whex`/`shh_xhex` — BOTH legs bridge-free (the program
  ends at its loop, so even the x86 chain has no post-loop rest-fn
  cut; both generated `gb_shhex_i` spellings are the pin body
  verbatim). Bridges at istmts (S^ 68 c2), walk at (S^ 122 d),
  c2 := (S^ 54 d).
- Gates: sibling 456/0 (+22: 21 claims + measured ihx_mem); wasm
  out 542/0, x86 out 832/0, regen byte-identical on all
  pre-existing content (diffs purely additive); weld 942/0.
- NEXT: I2e-3 (the pure blocks-loop artifact) per the composition
  ruling, then I2e-4 (the bin).
**IF-1 — the branch tier (2026-07-16; the imp-if-tier fork; four
commits: IF-1a the foundation, IF-1b the wasm emission, IF-1c-a the
x86 blueprint, IF-1c-b the x86 emission).** impgen's coverage grows
past sha's silhouette to the first branch class: top-level `IIf` with
straight-line comparison-headed branches in a mixed body, generated
FULL on BOTH targets.

- **IF-1a — ist_seam1 + the blueprint.** models/imp grows the
  SINGLETON seam: `istmts` over `(Cons s b)` at tied fuel =
  `ist_cont` of the standalone singleton run, for ANY statement — no
  flatness premise, no induction (the istmts defining equation
  refactored through the adapter; proof = one stopped compute + two
  case-ons). This is the imp-side split a branch boundary composes
  through — `ist_seam`'s flat premise excludes `IIf` and needs no
  relaxing; the tied fuels cost nothing because sub-lemmas quantify
  their own fuel tails and towers align by matching.
  examples/sqbw_probe.shard = the hand-validated blueprint (138/0;
  corpus pin): a mixed body [segment / branch with a store in the
  then arm / counting loop] proven end to end in the V2-5e-4 chain
  form. The branch piece's shapes: the OPEN-TAIL BRANCH SUB-LEMMA is
  the segment-lemma form verbatim (the Block/BrIf/Br encoding is
  self-contained, so from outside a branch chunk completes ONorm at
  the entry stack like inline code) with ONE case-on of the SHARED
  comparison scrutinee — both machines stick on the same raw-operand
  cmp tree (iop_val compares unwrapped; iw_cond emits the fused
  comparison) — and each arm walking concretely by the statement-tier
  split spine; the parked continuation is CLAIM-MINUS-ONE regardless
  of which arm ran (the machine hands one reservoir to head and
  tail), so the chain's fuel bookkeeping sees a branch as a 1-stmt/
  1-instr segment; the else path is the DEEP path (all seven inner
  elements including Br 1); branch exits expose by il_slen — the
  SEGMENT flavor, not il_wlen (a branch preserves arity through
  istmts generically). At the chain, a branch boundary needs NO
  isapp/fuel haves: the pasted `(Cons IIf rest)` and the S-tower
  match ist_seam1 directly; the sub-lemma cite instantiates its
  dangling `g` so both cites present the SAME standalone term, one
  case-on absorbs every outcome, and the next boundary is cited at
  the exposure leaf — cmp cites stay `(inst c2 c2)` with zero
  offsets.
- **IF-1b — the emission.** The piece grammar grows `MxPB` (scrutE,
  flip, per-arm splits, imp fuel bound): `mxc_br` validates a branch
  at slice time — comparison-headed conditions only, guard-free
  operands, arm walks at fresh symbolic locals (lw_stmts's own fences
  keep nested control refused) — everything else declines loud and
  named. The machine-chunk stream now carries branches (the V2-5e-4
  "machs advance on loops / msls on segments" law amends to: msls
  advances on segments AND branches — one invoke of iw_stmts on the
  singleton per branch). Emission: `sqbr_<nm>_s<b>` sub-lemmas
  (mxc_spine reused per arm under the condition case-on),
  `gb_<nm>_s<b>_i` singleton fns, the chain branch case (seam1 +
  sub-cite + shared case-on + il_slen exposure via mxc_exit
  verbatim), fuel bounds by conservative node count (oversupply is
  safe on BOTH sides of a branch sub-lemma: completed standalone runs
  are fuel-independent above need and the parked continuation is
  arm-independent). Dispatch widens: a memful straight-line body with
  a top-level branch routes to the mixed tier (`has_brtop`); memless
  branches keep the scalar tier; branches inside loop bodies keep
  their named fence (rung 2). mx_walk continues through a top-level
  `IIf` at fresh state on wasm (a branch contributes no worker
  event); on x86 both it and the slicer refuse with the branch-leg
  note, so the V2-7/V2-8 x86 machinery never sees a branch piece.
- **Fixture**: examples/imp_if.shard — it_ifm_fn (segment / branch
  with a store in the then arm / counting loop: the mid-chain branch
  piece, cited by the segment boundary and citing the loop boundary)
  + it_ift_fn (memful straight-line + TERMINAL branch through the
  widened dispatch, guards in the else arm — the flip's other
  polarity). Outs: impgen_wasm_if_out 123/0 — EVERY generated claim
  kernel-green FIRST TRY (both sub-lemmas, both chains, both public
  claims); impgen_x86_if_out 406/0 (ties + notes). Regen
  deterministic at the canonical raw siblings; canon-stable; all
  EIGHT pre-existing outs (examples scalar/loop/mixed ×2 targets +
  the two sha outs) regenerate byte-identical under the extended
  tool.
- Gates (IF-1a/1b): driver 74 products green; corpus FAIL-set ==
  baseline-65.
- **IF-1c — the x86 leg (blueprint + emission).** The 1b design note
  ("per-arm polarity-premised sub-lemmas") DISSOLVED on contact with
  V2-7's recorded finding: an x86 sub-lemma's conclusion must SPELL
  its scratch residues, and a branch has NO seal on either side
  (ix_cjoin seals flat-flat neighbors only), so the then arm's store
  dirt — the band tree in RAX — rides out of the branch unspellable
  at any conclusion point. The branch therefore walks INLINE in its
  chain lemma, the loop-leg precedent, and no new adapter species
  exists: ONE case-on of the shared raw-operand comparison scrutinee
  (iop_val compares unwrapped, xcond compares rget — both sides stick
  on the same inner cmp term), walked at the LEG'S IN-FLIGHT state
  (not fresh locals — the x86 chain does not re-bind at a branch),
  and each polarity arm carries its own tail text (arm guards, worker
  cite, il_wlen exposure, next-boundary cite) to the SAME cite
  targets. The measured keys (examples/sqbx_probe.shard, 388/0, the
  hand blueprint): fuel towers are ARM-INDEPENDENT on both sides —
  xeval_seq and istmts both hand continuations the same reservoir
  (depth budgets, not step counts) — so the V2-8 chain arithmetic
  carries over verbatim with the branch counted as 1 machine element
  / 1 imp statement, and imp-side cites keep `(inst c2 c2)` at zero
  offsets; JOINS BIND BY MATCHING — the arms park different RAX trees
  (band tree vs 0) and the next boundary's za binder absorbs both,
  its fuel binder any machine surplus (the user-ruled
  quantified-residue lean, realized without translator changes).
  Emission: `MxPB` carries its source `IIf` (the x86 leg re-walks
  cond + arms at the leg state); the leg grammar GLUES branches
  (mxx_legs: no seal → no cut; a branch leg = [flat?] branch [flat?]
  ending at a loop or the terminal seal); mxx_segm folds the piece
  list; mxx_ag adds conservative node-count headroom for the nested
  arm walks (oversupply safe — machine binds by matching, imp derives
  by subtraction); mxx_bleg emits the leg (prefix guard spine → the
  branch case-on → per-arm suffix spines and tails, arm tails
  identical text modulo walked locals/memory). GUARD DEDUP IS
  PER-REGION (prefix / then-path / else-path, never across the
  branch): an arm's guard occurrences only MATERIALIZE after the
  polarity case-on resolves, so an earlier textually-equal guard's
  rewrite cannot discharge them — found live by it_ift's else-arm
  store sharing guards with the entry store; repeated case-ons are
  sound (an inconsistent duplicate closes by conflated traps); arm
  spine indices offset past the prefix so hlt/hle names never shadow.
  v1 fences (note-grade, mxx_legck): "adjacent branches share an
  unsealed span (named growth)" (>1 branch per leg — 2^K tail text),
  "branch leg ending at a mid-flat seal point (named growth)" (the
  per-arm ist_seam composition, unbuilt). Fixture: it_ifm_fn grew a
  tail segment so its branch leg cites a next boundary PER ARM; outs
  impgen_x86_if_out 414/0 (full legs — the mid-chain pin green FIRST
  TRY, the terminal pin after the dedup-scope fix) and
  impgen_wasm_if_out 125/0; both deterministic + canon-stable; all
  six pre-existing example outs byte-identical (sha's regen legs
  driver-verified).
- NEXT (the fork's ladder): rung 2 branch-in-loop-body (the
  sha-adjacent fence: lw_stmts refuses nested control in loop
  bodies), rung 3 nested branches + the IF-1c fences (adjacent
  branches, mid-leg branch) — assess, fence loudly if not taken.

**I2e-3a — THE BLOCKS LOOP, wasm leg (2026-07-16): a counted loop
CALLS the generated block k times; the composition ruling's icall
blueprint, hand-built.**

- The artifact is PURE at the call boundary (the I2e ruling): a
  2-local loop fn (src, count) whose per-iteration work is `(Call 0)`
  on the callee `shb_wfn = (MkFunc 10 2 (Cons (I32Const 0)
  (gb_shblock_w)))` — the generated block VERBATIM behind one pushed
  `I32Const 0`; the block preserves the stack (shw_wblock quantifies
  it), so the push surfaces as the return value. No append, no
  regeneration.
- `shb_mem m src k`: the k-block memory fold (src advances 64 per
  block, the ground frame wb=64960 reused every pass) — the loop's
  entire memory effect, and I2e-4's composition currency.
- The callee run `shb_wcall` (S^ 1196 c): one-instruction peel
  (`shb_push`, the phase-pin idiom at machine grain — I32Const costs
  1 fuel), then shw_wblock with `(inst d c)` (dangling binder).
- The worker `shb_wloop` (S^ 1212 (lg_fuel k c)): k loop passes from
  the loop head ARE the fold. call_bridge instantiates the FOLDED
  callee (`(inst f (inline shb_wfn))`, popped args reversed, `(inst
  rest st)` for the dangling pivot). Counter bound cert: k2 <= 64896
  < 2^32 via `(rows (goal 1) (hnn 63) (hqs 1) (0 1))`.
- THE LITERAL-MODULE-SPINE idiom (new): the denotation needs
  func_at/body_of to COMPUTE on the loop fn while the worker's cite
  still matches the module — resolved by spelling the loop fn as a
  literal `(MkFunc 2 0 (list (Block (list (Loop (inline
  shb_wbody)))) (I32Const 0)))` IN the module (the callloop_probe
  pattern); callees stay folded, reconciled inside call_bridge.
- Denotation `shb_wblocks` (S^ 1216 (lg_fuel k c)): calling fn index
  1 on (src, k) returns 0 and lands the memory on `shb_mem m src k`,
  premises `0 <= src` and `src + 64k <= 64960`.
- THE X86 FINDING (drives the other leg): shw_xblock's output regs
  are each an h8 word or ZERO — src (r12) and wb (r13) are clobbered.
  Register-as-interface means the x86 loop MUST spill src/count to
  memory across each XCall; [65504, 65536) is free (frame ends at
  65504), spill grain = 4-byte words via the wput/wget chains. The
  spill loop is additive — the frozen block is untouched.
- Gate: weld 948/0 (+6: shb_push/shb_wcall/shb_wloop/shb_wblocks +
  measured shb_mem + shb_mem_z/s).
- NEXT: the x86 spill-loop leg (I2e-3b); the sha_blocks spec
  readback (shb_mem → sha_blocks, needs byte-grain shw_mem framing)
  composes with pad+hex in I2e-4.

**I2e-3b — THE BLOCKS LOOP, x86 leg (2026-07-17): the SPILL LOOP —
register-as-interface's real consequence, paid and proven.**

- The block clobbers everything (r12=src and r13=wb zeroed on
  output), so the loop's induction state lives in MEMORY across each
  call: src at [65504,65506), count at [65506,65508) — above the
  frame end wb+544 = 65504, the one region the block never touches —
  as two-byte LE words (both values < 2^16). Per pass: guard, spill
  through rcx (value) + rbx (address — the model has no immediate
  addressing), wb into r13, XCall 0, reload src (+64) and count
  (−1), re-zero the seven dirtied registers (the zero-in/zero-out
  idiom at the register file), loop. 37 instructions, one PINNED
  head file — the worker induction closes with no residue selectors.
- THE RELOAD READS THROUGH THE BLOCK: post-call the spill cells are
  read back through shw_mem's writes. NEW FRAMING FAMILY (point
  grain, the dual of the wget-grain below-family): `ish_ne_gt` +
  `ish_get_wput_above` + `ish_get_copy_above` + `ish_get_sched_above`
  + `ish_get_h8add_above` (imp file, all-symbolic inductions) compose
  into `ish_get_shw_above` (weld, ground frame: reads at p >= 65504
  see through the whole block effect). These are the first bricks of
  I2e-4's byte-grain window framing (mem_read is a fold of mem_get).
- THE ROUND TRIP `ish_rt2`: the two-byte spill/reload recombination,
  stated on the machine walk's EXACT tree (get_set's mod-spelled
  masked reads, XShl/XAdd's wrap64s) = the identity, by shr8_div +
  the euclidean characterization — div-facts, no bit apparatus.
- The denotation is the HONEST fold `shx_mem` (shw_mem over
  shx_spill, src advancing 64): the spill residue is spelled, not
  hidden. Unification with the wasm leg's shb_mem (residue at 4
  ground cells over the pure fold) is I2e-4's readback business,
  where the digest window reads through mem_set-outside anyway.
- Fuel exactness: xeval_seq burns one per instruction POSITION plus
  one per nesting level; the call at body position 14 fixes the
  worker at S^450 (bridge f2 = S^433 = the callee cert, slack ZERO)
  and the denotation at S^458 (4 preamble movs + block/loop peels).
- `xcall_bridge` fires with f/rs2/mem2 pinned (dangling binders);
  the callee run `shb_xcall` is bridge-free (no marshalling — the
  register file IS the interface; contrast the wasm leg's push-peel).
- GOTCHA (recurring): int_of_nat is OPAQUE — (int_of_nat 48) does
  NOT compute; bridge through the existing ish_iNp packed lemmas
  (ish_i16p/ish_i48p already in the sibling). And a compute that
  must deliver a FOLDED record result needs the projections stopped
  (stop h8a..h8h), or they open into stuck matches.
- Gate: sibling 462/0 (+6), weld 965/0 (+11 claims + measured
  shx_mem). BOTH machine legs of the blocks loop now land.
- NEXT: I2e-4 (the bin): compose write-input + pad + blocks-fold +
  digest readback + hex into sha256sum, with the spec collapse
  (folds → sha_blocks) riding the byte-grain framing.

**I2e-4a — THE SPEC COLLAPSE (2026-07-17): both machine folds land
on sha_blocks; the targets meet at one spec-side fold.**

- Point-grain BELOW family (`ish_get_wput/copy/sched/h8add_below` →
  `ish_get_shw_below` → `ish_read_shw_below`): byte reads below the
  frame see through one block effect — SOURCE SURVIVAL, the mirror
  of I2e-3b's above family.
- `shw_out_hw`: the block step at UNIFORM WLIST GRAIN (K holds
  sha_k, H holds st → H holds sha_block of st and the chunk). The
  8 wget premises of shw_out_h discharge by LANE PROJECTION: apply
  wl_hd/wl_tl^j to both sides of the window equation, then compute
  (case-on st first — and rewrite the case hyp INSIDE each
  obligation, since premises don't see the goal's rewrites).
- `shb_fold` (the memory-chunk spec fold, chunks always read from
  the ORIGINAL memory) + `shb_fold_frame`/`shx_spill_fold_frame`
  (the fold ignores writes above its chunks). THE READBACKS:
  `shb_hread` (wasm) and `shx_hread` (x86) — the loop denotations'
  H windows BOTH equal `shw_hlist (shb_fold st m src k)` under the
  K/H invariants; the x86 spill residue is invisible at this grain,
  so the promised shb_mem/shx_mem unification needed no residue
  surgery at all.
- Stage C `shb_fold_blocks` (UNPREMISED): `shb_fold st m src k =
  sha_blocks st (shb_bytes m src k) (int_of_nat k)` — chunk
  boundaries are exactly stake/sdrop 64 via `ish_read_len` +
  the new `ish_stake64_app`/`ish_sdrop64_app` (premised literal-64
  forms over `ish_stake_app`/`ish_sdrop_app`/`ish_slen_lo`).
- Gotchas: `stake 0 rest` sticks on a symbolic list (case-on the
  second list in Nil cases, rewriting the case hyp in BOTH arms);
  case-on takes a bare type NAME (`List`, not `(List Int)`);
  `ish_i8p` added (the packed bridge family grows on demand).
- Gates: sibling 472/0 (+11), weld 986/0 (+17).
- NEXT (I2e-4b): the K/H init composition (ikh_mem readbacks meet
  the fold's invariants), the pad-tier crossing (memory holds
  sha_pad msg → shb_bytes = the padded list), then digest readback
  + hex over the final fold; then the bin (I2e-4c: thin World main,
  slurp ≤ cap, controlled-failure leg, NIST vectors).

**I2e-4b progress (2026-07-17): the init crossing + width plumbing.**

- Init crossing: `ish_get_setw_below`/`ish_read_ikh_below` (byte
  windows at or below the K window survive the whole init) +
  `ikh_k_read`/`ikh_h_read` (ikh_mem delivers EXACTLY the fold's
  invariants: K window = sha_k, H window = shw_hlist (sha_h0)).
- Width plumbing: `ish_read_app` (a read at a wn_add width is the
  sapp of its halves — the pad kit already had it; a 4b duplicate
  `ish_read_split` was landed then retired), `shb_w` (64k as a Nat
  width) and
  `shb_bytes_read` (the fold's chunk region as ONE mem_read) — the
  pad tier's readback width and the fold's chunks now speak the
  same vocabulary. GOTCHA: canon C6 packs a fn's ground Nat arms
  (`shb_w Z` → literal 0), so structural-pattern rewrites
  (mem_read_z) need a stated-form compute-both have as a bridge.
- Gates: sibling 475/0, weld 992/0.
- THE WIDTH TIE (landed): the pad tier's window and the fold tier's
  chunk region are the same Nat — `shb_pad_w`: under wz's mod-64
  premise and k = ediv (wn+72) 64, `wn_add wn (1 + wz + 8) = shb_w k`.
  Machinery: the sibling's Nat/Int TRANSFER KIT — `ish_nat_inj`
  (injectivity of the opaque embedding, from the surface lemmas
  alone: nonneg pins the successor case, farkas derives (le 1 0) on
  the clashing arms and `absurd` closes them — first use of the
  absurd form in this arc), `ish_iw_add` (the additive hom),
  `ish_i1p`, and `ish_ediv_mul` (ediv (64x) 64 = x, the fuel-tie
  step). Weld side: `shb_w_i` (int image = 64k) and the euclidean
  argument — ediv_mod_id names the mod-64 decomposition of 55−wn,
  div_unique with witness (q = 1−ediv(55−wn,64), r = 63−wz) pins
  ediv (wn+72) 64, and ish_nat_inj carries the resulting Int
  equation back to the Nat widths.
- GOTCHAS: two-sided equality goals need the two-list farkas cert
  `(list (dir1…) (dir2…))` — keyed `(rows …)` builds only one row
  vector (the F2 diagnoser prints the slot table; raw certs cover
  ALL in-scope premises in order, zeros for unused). And the weld
  file did NOT import kernel/facts until now (the euclidean axioms
  resolved in the sibling only); the import adds facts' own claims
  to the checked closure (992 → 996 base), no new axioms (already
  transitive via the sibling).
- Gates: sibling 479/0, weld 998/0.
- REMAINING for 4b: the digest readback + hex chain over the final
  fold (lane projections + ish_digest_read/ish_hex_read +
  hex-over-final framing; output overwrites the dead input region
  at [0,64)).

**I2e-4b complete (2026-07-17): the digest + hex seals — the back
half reads back.**

- `shw_digest_read`: any memory whose wlist-grain H window holds st
  reads the 32-byte digest at 65472 as h8_bytes st — case-on the
  state, then LANE PROJECTION (the shw_out_hw idiom verbatim)
  discharges ish_digest_read's eight wget premises inside the
  citation's obligations (rewrite the case hyp INSIDE each
  obligation, as always).
- `shw_hex_read`: the hex effect over any such memory delivers
  bytes_hex (h8_bytes st) at the output window [0,64) — the dead
  input region, far below the digest window, so ish_hex_read's
  separation premise is ground (ish_i32p bridges the opaque
  int_of_nat 32; a compute-both have respells width 64 as
  wn_add 32 32 to meet the lemma's wn_add wk wk pattern).
- `shb_out_read`/`shx_out_read`: THE BACK-HALF READBACKS — under
  the fold premises, the output window over hex-of-fold-exit is
  the spec's hex digest of shb_fold, both targets (shb_hread/
  shx_hread cited inside shw_hex_read's window obligation; st
  dangles in every one of these patterns — inst it through).
- All four green on the first check. Gates: sibling 478/0 (no
  sibling changes), weld 1001/0.
- NEXT (I2e-4c): the bin — thin World main (slurp ≤ cap,
  controlled-failure leg), the full-pipe pure composition
  (write-input + init + pad + fold + hex through the stage seals;
  shb_pad_w + ish_ediv_mul tie the block count; shb_fold_blocks
  collapses to sha_blocks = the spec), NIST vectors, gates.

**I2e-4c progress (2026-07-17): THE FULL-PIPE COMPOSITION — the
memory-effect pipe IS the spec, both targets.**

- `ish_wget_pad_hi`/`ish_wlist_pad_hi` (sibling): word reads and
  whole windows at or above the pad span's end see through the pad
  effect — the wlist-grain lift of ish_pad_hi, by induction on the
  WINDOW length (the pad effect is composite, so the effect-counter
  induction of the copy/sched lifts doesn't apply; the head wget
  frames via the byte-grain lemma, the IH re-binds p at (+ p 4)).
- `shb_pipe_read`/`shx_pipe_read` (weld): a memory holding the
  message at [0,wn) flows through ikh_mem → ipd_mem → shb/shx_mem
  → ihx_mem and the output window [0,64) reads back as the SPEC's
  `sha256_hex (mem_read wn m 0)`. Premises = the artifact's
  arithmetic only: wz's mod-64 equation, k's ediv equation, and
  wn+72 ≤ 64960. Proof: stage seals end to end — ish_wlist_pad_hi
  carries the init's K/H invariants across the pad; shb_pad_w (rl)
  + ish_pad_read + ish_read_ikh_below deliver the padded window as
  sha_pad of the message; shb_out_read (st := sha_h0) runs the
  back half; shb_fold_blocks collapses the fold; ish_read_len +
  shb_w_i + ish_ediv_mul tie the spec's ediv block count to the
  premised Nat k (the fold bound 64k ≤ 64960 falls out of the ediv
  premise by the euclidean haves + a 5-row farkas).
- GOTCHA: the spec's `sha256`/`sha256_hex` are mod.req sig→def
  rebinds — the file star-use does NOT export them; the weld needed
  the module-surface use lines (`(use (:: std sha256 sha256))` +
  `sha256_hex`), legal to unfold because the weld is home-module.
- Gates: sibling 480/0 (+2), weld 1005/0 (+2).
- REMAINING for 4c: the top-level artifact fn (one imp fn calling
  init/pad/fold/hex; impgen both targets; machine walks land on the
  composed memory, then the pipe lemmas finish), then the bin
  (thin World main, slurp ≤ cap, controlled-failure leg, NIST
  vectors, gates).

**IF-2 — branch-in-loop-body (2026-07-16; the imp-if-tier fork; two
commits: IF-2a the blueprints, IF-2b the emission).** The
sha-adjacent fence falls: counting loops whose BODY holds one
top-level `IIf` (comparison-headed, straight-line arms, stores
allowed anywhere), generated FULL on BOTH targets. No translator or
model changes — both loop-body translators already ride the
IIf-capable statement translators, and `ix_dwl` sees through nested
`XBlock`s.

- **IF-2a — the blueprints** (examples/sqblw_probe.shard 117/0 +
  examples/sqblx_probe.shard 410/0, full pins, assembled from
  flat-twin scaffolds with the branchy loop swapped in). The new
  mechanism is the WORKER: inside the induct step, ONE case-on of
  the shared raw comparison scrutinee (both machines stick on it
  after the entry walk), then each polarity arm rewrites `(hyp 0)`
  into both sides, walks its own region's guard case-ons, rides the
  shared body tail, and cites the IH at its own hyp depth with
  `(inst c2 c2)` — the induct generalizes locals and mem, so the
  per-arm cites bind by matching. On x86 the loop's own `ix_zs` seal
  re-zeroes the union of arm dirt before the back-edge, so the
  header register state is arm-independent in scratch — THE SEAL IS
  THE JOIN. The IF-1c dedup law holds inside the worker: guard
  regions (body prefix / then / else) dedup independently, never
  across the polarity case-on (arm occurrences materialize only
  after the polarity resolves); duplicate case-ons close by
  conflated traps.
- **Fuel.** Towers are depth budgets: one iteration burns exactly
  one `lg_fuel` S at the header on both sides regardless of arm, so
  the V2-5 chain arithmetic carries over. The machine slack must
  price the branch NESTING: `aw = tcost(loopcode) + 2`, which equals
  the old `wlen + 4` on flat bodies (all eight pre-existing outs
  byte-identical) and grows with nesting (the pure loop tier already
  priced by tcost; the mixed tier now does at all six sites —
  mxw/mxxw_emit, mxc/mxx_ag, mxc/mxx_lhead). `bw = gcost(body) + 5`
  was already branch-aware. Chain consequences, blueprint-measured:
  the imp-side boundary towers grow with bw down the chain; the wasm
  machine towers are UNCHANGED (the worker's machine surplus is
  absorbed by its own fuel binder — the reshape have may carry an
  inner `S^k`); the x86 leg's machine claim grows with aw and the
  terminal boundary cite absorbs the surplus by matching (only imp
  `c2` is pinned).
- **IF-2b — the emission.** The loop-body walk factors into
  `lwb_walk` (shared by the slicer's `mxc_loop` and the event
  walker's `mx_walk`): split at the first top-level `IIf`, walk the
  prefix at the loop-head state, take the scrutinee at the
  post-prefix state (mxc_br's constraints verbatim: `is_cmp`-headed,
  guard-free operands), walk each arm ++ post-tail per arm, dedup
  per-REGION, and check the trailing band-decrement on BOTH per-arm
  post-states. The payload rides `(Option LBr)` through
  `MxPL`/`MkMxLp`; `mxw_brcase`/`mxxw_brcase` emit the worker's
  branch case-on (arm spine indices offset past the prefix so
  hlt/hle names stay distinct; arms are separate scopes). The
  generated pin-1 worker is byte-identical to the hand-proven
  blueprint worker (modulo names); the generated chain derives
  tighter machine towers than the twin-based probes (exact `S^1`
  inner), both shapes valid.
- **Fixture** examples/imp_ifl.shard: `it_ifl_fn` = the blueprint
  program verbatim; `it_ifl2_fn` = the full region structure (body
  prefix WITH store guards, banded `IEq` scrutinee, guard-free then
  arm, else arm DUPLICATING the prefix guards — the conflated-trap
  closure exercised inside the worker). Outs:
  impgen_wasm_ifl_out.shard 123/0, impgen_x86_ifl_out.shard 416/0,
  both FULL first-run, deterministic, canon-stable. Registered:
  build_products +5 (driver 74→79), run_corpus +5 (2 probes at
  IF-2a, fixture + 2 outs at IF-2b).
- **Fences that moved inward** (named, loud): lw_stmts's IIf refusal
  now reads "nested or repeated branch (straight-line arms; one
  branch per loop body; named growth)" — it fires for a branch
  inside a branch arm and for a second branch per loop body; the
  walker adds "loop-body branch condition carries guards" and
  "non-comparison loop-body branch condition". [The original seam
  sentence here claimed store-free branchy-loop pins route to the
  pure tier — measured false at IF-2c, superseded 2026-07-16: loop
  pin dispatch keys on BODY SHAPE (lp_pieces: body = one bare
  IWhile), not store-ness. Store-free pins with a const-set counter
  enter the mixed tier and generate FULL; only bare-IWhile
  (symbolic-count) pins go pure.]

**IF-2c — the seam measured; the pure-tier fence named; the store-free
lock (2026-07-16; the imp-if-tier fork; one commit).** No new proof
machinery — a measurement pass over the IF-2 NEXT items, one fence
correction, one fixture lock.

- **The dispatch, measured.** A store-free branchy-loop pin with a
  const-set counter ALREADY generates FULL on both targets — the
  mixed tier never needed mem (empty guard regions walk fine; the
  worker is the polarity case-on alone). Locked by fixture:
  `it_ifl3_fn` (imp_ifl.shard) — outs regenerate strictly
  ADDITIVELY (committed content = byte-identical prefix), wasm
  129/0, x86 422/0, deterministic. Driver product count unchanged.
- **The real seam = the pure loop tier.** Bare-IWhile
  (symbolic-count) pins are the only branchy-loop shape that
  refuses, and the refusal borrowed lw_stmts's "nested or repeated
  branch" message — wrong for a FIRST branch. Now its own named
  fence in lp_gen: "branch in a symbolic-count loop body (the pure
  loop tier; named growth)". Branch support there is a REAL rung:
  the pure tier's worker is Some-conditional (the imp side rides a
  premise, swapped in by `(rewrite (premise 0) rl rhs)` — a
  different integration than the mixed tier's synced walk), so the
  IF-2 case-on mechanism ports but the proof text is new. Queued as
  a named candidate rung, not taken here.
- **Rung-3 fences, probed on both targets** (scratch pins, not
  committed):
  - *Adjacent top-level branches*: the wasm emitter ALREADY
    generates AND certifies the two-branch chain — branch legs
    compose down the boundary chain like any other leg (scratch
    check 126/0). x86 refuses note-grade at `mxx_legck` ("adjacent
    branches share an unsealed span") — the open design choice is
    nested case-ons within one leg (2^n arm paths, no translator
    change) vs a minimal to_x86 seal between adjacent branches
    (shared-ground edit; existing pins have ≤1 branch per span so
    committed bytes hold). RULING OWED before rung 3a.
  - *Nested branch (IIf inside an arm)*: hard fence fires with the
    correct message ("nested or repeated branch …").
  - *Branch-with-loop-arms*: hard fence fires as "nested loop
    (named growth)" — loud and safe, though the message names the
    loop rather than the branch context.
- NEXT (the fork's ladder): rung 3a adjacent branches on x86 (gated
  on the seal-vs-nesting ruling; wasm side needs only a fixture
  lock, which should land WITH the x86 leg so fixtures stay
  both-target); rung 3b nested branches; the pure-tier
  symbolic-count branch rung; branch-with-loop-arms stays fenced.


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
