shard imp — IMP.md
==================

> **STATUS (reset 2026-08-22): LAW.** the neutral imperative dialect and impgen; §6's coverage redirection is the charter of the goal (#23); §9's 'Arc A next' pointer is history. The backlog is the GitHub issue tracker (labels `arc:coverage` / `parked` / `debt`; the goal = #23, the prune arc = #24) — any "next arc/rung" pointer below is history unless it names an issue.

> Path note (2026-07-18): file paths in this ledger are as-landed history; the repo was reorganized — decode old `examples/` paths via [LAYOUT.md](LAYOUT.md).

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
  *(Amended 2026-08-01, the M5 base threading — STREAM.md §7.9:
  IProg declares the window as [ibase, imemsize) — both bounds
  are program text, discovered by running the wrapper, never
  assumed — and the evaluator guards every load/store against the
  declared window (iexp/istmt/istmts/iwhile thread mlo msz; the
  generated bridges' guard splits stay UNPREMISED because the
  window is spelled identically on both sides of every split —
  window equality between imp and the machine tiers is forced by
  the v2 unpremised-bridge thesis, a premised low bound would
  re-open every bridge statement). The landed realization is the
  IDENTITY at the declared base: the program's own address
  literals carry the base, the emitted image needs no relocation,
  and the module window equals the mapped region (X86.md §51,
  elf.shard W2). The base-register convention above remains the
  named growth for base-portable programs; it did not land.)*
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


## 2b. The call tier — calls, words, and the fail leg (COVERAGE.md C1, 2026-08-22)

The generic path's programs call. The base machine above never did,
and it cannot grow: its constructor sets (IExp / IStmt / IOut) are
load-bearing for the certificate corpus — hand welds, generated outs,
and the 247k-line vx86_acc_probe induct and case over them, and the
last of those was emitted by tooling that no longer exists. So the
call tier is ADDITIVE, the x86 model's base-tier / world-tier idiom
applied to syntax:

- **`IpStmt`** mirrors the four base statements over the UNCHANGED
  expression language and adds `(IpCall i k args)` — local i := fn k
  applied to pure argument expressions, banded on entry —
  `(IpLoadW i addr)` / `(IpStoreW addr v)` — std/mem's little-endian
  word view at width 8 (`load_le` / `store_le` over the `iw8` tower),
  the whole span guarded inside the declared window — and
  `(IpFail fam)`, the reasoned fail leg over the closed family set
  `IFam = FOverflow | FOom | FStack` (MEMORY.md D8). A call is a
  STATEMENT so that expressions stay pure and fuel-free (`iexp` serves
  both tiers) and memory threads through the statement engine
  unchanged.
- **`IpFn` / `IpProg`** mirror `IFn` / `IProg`; the evaluator is its
  own SCC `ipstmt` / `ipwhile` / `ipstmts` / `ipcall` on the fuel
  measure, with the fn table and the window threaded explicitly (the
  base tier's mlo/msz discipline — claims never project a symbolic
  program), into its own outcomes `IpOut` (norm / trap / failed) and
  `IpRet` (value+memory / trap / failed). `ipcall` burns one unit per
  call, so fuel bounds depth; `iprun` is the program-level denotation
  the generated per-fn theorems are stated at (COVERAGE.md P7).
- **The lift** `ip_of_fn` / `ip_of_prog` embeds base-tier programs
  (the identity on the shared constructors); the lift law — `iprun`
  over a lifted program agrees with `icall` — is owed at the first
  composition (COVERAGE.md C3).
- **The gate** `ipwk_prog`: the base rules plus a call's target local
  at the callee's result kind and its arguments at the callee's
  parameter kinds, word loads into U64 locals from U32 addresses,
  word stores of U64 values, fail always well-kinded.
- **Probe**: models/imp/probes/ipcall_probe.shard (34 claims — calls,
  recursion depth on the fuel, memory through a call, the fail leg
  out of a callee, bad index / arity / fuel honesty, word-op wiring
  and window guards, the lift on imp_probe's program, the gate's
  accept/reject line). The width-8 round-trip law (`ls8_id`) is
  std/mem growth owed at C2.

- **The resource parameters (COVERAGE.md A-0, 2026-08-22).** `IpProg`
  carries `ipstack` — where the imp-visible window ENDS: `iprun`
  evaluates at `[ipbase, ipstack)`, the machine leg keeps its frames
  in `[ipstack, ipmemsize)`, so a run that returns a value never
  touched the frame region — and `ipdepth`, the call-depth budget:
  the SCC threads (dmax, d), `ipcall` fails `FStack` at d = dmax and
  runs the body at d + 1 (the entry call is at 0). The depth is the
  MODEL's so that a generic machine theorem can bound the frames a
  run needs; generated code carries no counter.

What did NOT change: every base-tier definition, every existing
certificate, impgen's emission. Named growth behind consumers: the
lift law (C3), mutual tail calls, the conditional artifact form.


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

> **Moved to [records/IMP.md](records/IMP.md) (2026-09-02, the ledger split: LAW stays here, dated RECORDS live under docs/records/ with their section numbers unchanged).** Cited as `IMP.md §…` everywhere; open records/IMP.md for §6a. Rung records.

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


## 9. The certificate redirection (2026-07-18)

The 2026-07-18 design review (docs/archive/DESIGN-REVIEW-2026-07-18.md)
ratified a redirection that governs this ledger's NEXT pointer:
**docs/CERT.md is now the law for certificate representation**, and
Arc A (its pathfinder protocol, CERT.md §8) is the next arc — before
any new emission tier, coverage family, or lowering rung.

> **AMENDED 2026-08-22 (the reset).** Arc A closed 2026-07-26 and the dialect was ratified FINAL 2026-07-28 (CERT.md §8), so the design-only freeze below is LIFTED: the coverage arc — §6's redirection, the uniform-representation compiler over imp with the counted heap as its runtime, examples/calc as rung 1, shardfmt as the flagship — is THE GOAL (#23), opening after the 2026-08 prune arc (#24). Its paper debts (calls/stack, signed kinds, address policy, heap patch/framing algebra, the cons/match/free micro-flagship) close in the opener ledger, ratified before anything emits. **The opener ledger is docs/COVERAGE.md (drafted 2026-08-22): §4 closes the four debts as pins P3/P5/P4/P4, C3 is the micro-flagship.**

Standing consequences here:

- impgen (V2-5) and its emitted dialect are FROZEN as the
  oracle/regression source (CERT.md §10). Existing outs regenerate
  byte-identically; no new cert family learns the replay dialect.
- The rung ladder past IF-1 (calls/stack, signed kinds, the
  coverage families) continues as DESIGN work only; first emission
  waits for CERT.md's representation verdict.
- The v2 crystallized kinds (§2a) are what make the generic
  validator induction tractable — alignment is program-independent
  by type. The validator pilot (CERT.md §8 A1) runs on this
  machine's smallest straight-line family.
