shard imp — IMP.md
==================

STATUS: RATIFIED (2026-07-12; drafted 2026-07-11) — the scope ledger
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
