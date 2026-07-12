shard floats — FLOATS.md
========================

STATUS: DRAFT (2026-07-11) — the scope ledger for floating point in
shard: how IEEE-754 arithmetic enters the language honestly — exact
deterministic semantics, no pretense of reals — and how it lowers to
bare hardware float instructions on every target. This file owns the
float story end to end: the spec domain (std/rat), the core float
model, the per-arch ISA float ops, and the bridge between them.
docs/ISA.md listed floats as a v1 non-goal; this ledger takes
ownership. docs/LANGUAGE.md's "No floats" line describes the narrow
form and stands until the surface rung lands.

User rulings already on record from the design discussions of
2026-07-11 (do not relitigate silently):

- **std/rat is wanted independently.** Rationals are the natural
  stand-in for algorithms that express math without mandating extent,
  the same role unbounded Int plays. It is a std construction over
  Int (the Word/Bytes precedent) — the kernel is untouched.
- **One core deterministic float model, ours.** The alternative —
  skip a core model and use only per-arch float ops during lowering,
  keeping high-level work in rationals — was considered and REJECTED:
  it duplicates the core float machinery per target and fights the
  premise set with Word. We declare a core deterministic float model,
  model the per-arch quirks honestly in their ISA models, and rely on
  lowering to bridge the gaps.
- **Parametric formats from day one.** The whisper-tensor lesson:
  fully parametric floats are quite useful, especially once bf16 is
  involved. The core model is parameterized by format descriptor, not
  a set of named variants.
- **The packing law.** Float values are represented at their natural
  width and pack densely — machine-learning workloads are a planned
  shard target and depend on dense narrow-format tensors. "Opaque"
  in the NaN-quotient sense means bit-preservation and
  no-branching-on-payload, NEVER width (guard against misreading;
  see §6).
- **Honest-caveat framing carries over from MEMORY.md**: hardware
  constraints enter through models with 1:1 high/low behavior;
  lowering attaches honest caveats. For floats the caveat surface is
  the NaN observation quotient (§5) — nothing else about float
  behavior is target-dependent.


## 1. Purpose and stance

Floats are what programs compute; Int and std/rat are what specs
mean. The two-type structure is load-bearing:

- **std/rat** is the spec domain — requirements and exact statements
  live there. It is NOT a program type for numeric inner loops:
  denominators grow without bound under iteration, and a rat-valued
  loop is a bignum interpreter in disguise (against C-class
  identity). Rat as a program type stays legitimate for genuinely
  exact work (geometry predicates, currency) where it lowers as an
  ordinary record of Ints.
- **Floats** are the program type: fixed extent, hardware-speed,
  exact deterministic semantics defined by the core model. The
  connection back to rat/Int is by theorem (rounding laws, error
  bounds), never by pretense.

The honesty bar: every float operation in shard has ONE meaning,
defined by a written-in-shard model, identical on every target, with
the hardware's deviations from that meaning either proven harmless
(the NaN quotient, §5) or bridged by explicit lowering fixups (§4,
§8). No "implementation-defined", no fast-math, no implicit
contraction, ever.

Foundational fact the whole ledger rests on: IEEE-754 defines the
basic operations as round(exact result), and for add/sub/mul/div/sqrt
the rounding decision is decidable over integers — every finite float
is m·2^e; add/sub/mul are exact in dyadics; div and sqrt rounds are
decided by integer division/isqrt WITH REMAINDER (the sticky bit). No
real numbers are needed anywhere in this arc. Precedent that this
works at scale: ACL2's RTL library (the AMD/Intel/Arm hardware
verification corpus, reals-free logic), Flocq's computable Binary.v
ops (Coq/CompCert), Lean 4.33's kernel-reducible Float.Model, symfpu
(the SMT solvers' pure bit-vector semantics, validated by exhaustive
checking on tiny formats — an idiom this arc adopts wholesale, §10).


## 2. The spec domain: std/rat (rung R0)

An opaque refined pair over Int — `(num, den)`, `den > 0`,
gcd-reduced — the std/str construction pattern. Zero axioms, zero
kernel changes.

- Ops: add/sub/mul/div/neg/abs/compare, Int embedding, floor/ceil.
- The theorem kit: cross-multiplication lemmas turning rat
  (in)equalities into Int (in)equalities under positive denominators,
  so farkas fires on the lowered goals (the std/bits kit pattern —
  the solver never learns the type exists).
- Ground-compute performance: in-language gcd under shard_eval is
  sufficient for spec-side work. If a future consumer needs fast
  ground rat (the interval-automation arc), the Nat-former precedent
  applies: packed ground literals in step+ceval, performance only,
  zero semantic axioms. Not built until demanded.
- The dyadic fragment (m·2^e — closed under add/sub/mul, no gcd,
  normalization is shifting) starts as std/float's internal working
  representation; graduation to its own module is deferred to the
  interval arc (§13).

std/rat lands FIRST — the float spec layer states its meaning
through `val : Fin → Rat`.


## 3. The tower

Five layers; each lower layer is bridged to the one above by theorem
or by named pin. Only the silicon boundary is a pin.

**L0 — std/rat.** §2.

**L1 — the core float model (std/float, parametric).** A format is a
descriptor of four parameters (adopted from whisper-tensor's
FloatType, proven against the ONNX format zoo):

    (fmt EW MW HAS-INF HAS-NAN)
    EW = exponent bits ≥ 1, MW = mantissa bits ≥ 1
    bias = 2^(EW−1) − 1   (always; non-standard-bias formats excluded)

Value view: `(Fin sign m e) | (Inf sign) | NaN`, with the
normalization refinement, and `val : Fin → Rat` the exact dyadic
value. Encodings partition the max-exponent row as a function of the
two flags:

    HAS-INF  HAS-NAN   mant=0      0<mant<max    mant=max
    yes      yes       ±Inf        NaN           NaN        (IEEE)
    yes      no        ±Inf        normal        normal
    no       yes       normal      normal        NaN        (FN)
    no       no        normal      normal        normal     (MX)

Named instances: F64 (11,52), F32 (8,23), F16 (5,10), BF16 (8,7),
F8E5M2 (5,2), F8E4M3FN (4,3,no-inf) — plus toy formats for the
exhaustive gates (§10). FNUZ formats (non-standard bias, −0-as-NaN)
and signless E8M0 do not fit the decomposition and are named
exclusions until a consumer forces separate arms (§12).

Every operation is DEFINED as `rnd ∘ exact-op` over the value view,
with `rnd` = round-to-nearest-even (ties-to-even, overflow→Inf per
IEEE, gradual underflow through subnormals). Ops whose exact result
is not rational (sqrt) are specified by the integer bracket that
decides the round (isqrt with remainder). The model is ordinary shard
code — executable in check mode, so ground differentials and probe
grids work from day one.

**L2 — the bit-level model.** Pack/unpack between the value view and
the Word-width bit pattern (proven bijective up to the NaN class),
and computable ops working directly on (m, e) with guard/round/sticky
bits — never materializing a rat. Proven ⊑ the L1 specs. This is the
shard analogue of Flocq's Binary.v, narrowed: radix 2, RNE only.

**L3 — the per-arch ISA float ops, modeled honestly.** The x86 model
grows SSE2 scalar ops with the true Intel SDM semantics: the both-NaN
rule (result = first source operand, quieted), the MINSD/MAXSD quirk
(second operand returned when either input is NaN — not IEEE minimum),
the CVTTSD2SI out-of-range sentinel, MXCSR as modeled machine state.
The wasm model grows f32/f64 numerics per the wasm 3.0 core spec:
correctly rounded RNE basic five, IEEE-style min/max (−0 < +0),
nondeterministic NaN payloads acknowledged as the spec's honest
content. Quirks are never smoothed over in L3 — L3 is where the
hardware's truth lives.

**L4 — lowering fragments.** Model calls are replaced by single
instructions (or short fixup sequences where L3 quirks demand:
min/max on x86, float→int fixups). The bridge obligations are §4's
two tiers.


## 4. The bridge: two tiers, one pin

**Tier 1 — DERIVE (proven, per arch).** `core_op ⊑ arch_op` up to
the NaN-class quotient (§5): for all inputs,
`decode(arch_op(a, b)) ≡ core_op(decode a, decode b)` where `≡` is
equality except that all NaNs are identified (payload AND sign). Both
sides are written-in-shard models, so this is a theorem, not trust.
Fixup sequences (x86 min/max, conversions) are proven at this tier to
implement the core spec exactly.

**Tier 2 — PIN (differential, at silicon).** The arch model vs the
actual hardware/engine: TestFloat vectors and probe grids through the
on-CPU runner for x86, V8 runs for wasm (the engine-of-record). This
is the platform-externs trust pattern (X86.md §32) and the ONLY
empirical trust in the tower. Note this is strictly stronger than the
syscall story, where the kernel model is pure pin — for floats,
everything above the silicon boundary is theorem.

Because V8-on-x86 emits x86-flavored NaN payloads (the wasm
deterministic profile is not what engines implement), tier-2 gating
compares under the same NaN-class quotient — which is exactly what
makes the gate valid on any conforming engine regardless of payload
behavior.


## 5. Determinism: the NaN observation quotient

The one place hardware disagrees observably: NaN payloads. x86
propagates the first operand's payload; RISC-V always emits the
canonical NaN; wasm's spec makes payloads nondeterministic. Since the
lib paradigm demands SAME decl → same answers on every target,
payloads must be unobservable. The ruling:

- The core model has a SINGLE NaN value: positive, quiet, zero
  payload (the RISC-V / wasm-deterministic-profile choice). No
  signaling NaNs in the model at all.
- The surface API's only bits-observer (`to_bits`) CANONICALIZES NaN
  (payload and sign) on the way out. `from_bits` maps every NaN
  encoding to the model's NaN.
- Lowered code moves float values bit-preserving (registers, calling
  convention, memory round-trips — raw bits are fine and free) but
  never BRANCHES on NaN payload bits; only model-sanctioned ops
  inspect representations. This is a small permanent invariant on the
  lowering fragments, the float analogue of Word's
  semantics-via-Int-images discipline.

Under this quotient, cross-target determinism is a theorem, not a
hope — and nothing pretends the hardware divergence doesn't exist;
the exported bridge is quotient-typed (§4).

Consequences stated once: IEEE `==` is a Bool-valued function, not
equality (`NaN ≠ NaN` under it; `+0 == −0` while the values are
distinct); IEEE-2019 `totalOrder` is provided separately for contexts
needing a real order; sign-bit ops (neg/abs/copysign) act on the
model's single NaN as identity-on-NaN, consistent under the quotient.


## 6. Representation and the packing law

Every format's memory representation is its natural width: F64 = 8
bytes, F32 = 4, F16/BF16 = 2, F8 = 1 — packed densely in aggregates
and std/mem byte regions (LE, per the mem arc), slotting into
MEMORY.md's rep families as ordinary fixed-width scalar fields.
Machine-learning workloads are a planned shard target; dense
narrow-format tensors are a day-one representation law, not a
retrofit. Scalar values may ride wider registers in flight (XMM,
wasm locals) — layout law governs memory, not registers.

The core model's decode/encode is the packing spec: pack/unpack per
format descriptor, proven bijective up to the NaN class (§3 L2).


## 7. Narrow formats: certified wide-compute

f32/f64 get direct hardware lowering. BF16/F16/F8 ops lower as
convert → compute in a wide hardware format → convert back, certified
by the classical double-rounding condition (round-to-wide then
round-to-narrow equals direct round when p_wide ≥ 2·p_narrow + 2 —
covers bf16 and f16 via f32 in the normal range). The condition
genuinely thins in the subnormal tails; the exhaustive small-format
gates (§10) find the exact boundary mechanically, and the L4
fragments carry explicit fixups where the theorem fails rather than
accepting drift. Native narrow-format hardware (AVX512-BF16 class —
which flushes subnormals and is not IEEE) is a LATER, separately
differentially-gated fast path; the certified software route is the
trustworthy bf16 story and lands first.


## 8. Machine-state law

- **RNE only, statically.** No dynamic rounding mode exists in the
  language. (Directed rounding may later enter the MODEL as a
  parameter for the interval arc — §11; a hardware mode register
  never enters the semantics.)
- **MXCSR is pinned at defaults** (RNE, DAZ/FTZ off, exceptions
  masked) by the `_start` stub, modeled honestly in the x86 model,
  and never written by generated code. ZERO-C means nothing else in
  the process can flip it — the bin invariant is total.
- **No IEEE status flags in v1.** Pure value semantics. Flags are
  MXCSR sticky state on x86 and absent on wasm, so flag-based
  algorithms are cross-target-dead regardless; a future World-effect
  treatment is named in §11.
- **x87 is never used.** SSE2 scalar only on x86.
- **Full subnormals, always.** The model never flushes; lowering
  never enables flushing.
- **No implicit contraction.** `a*b + c` is two roundings unless the
  program writes `fma` explicitly. fma is in the core model from the
  start; it lowers to FMA3 on x86 and to the L2 software path on wasm
  (no scalar fma instruction exists there) — slower, identical
  semantics.


## 9. Surface: literals, printing, conversions

- **Literals** are defined by the model: exact decimal → rat →
  `rnd`, evaluated at load time (build-time compute is free). No
  host-float laundering anywhere in the pipeline — the bootstrap
  reader's lexpr lossy-f64 path must be bypassed for float literals
  exactly as it is loud-rejected for big ints today; the self-hosted
  reader implements the exact pipeline.
- **Hex-exact printing** (sign, hex mantissa, binary exponent) is the
  rung-1 observer — exact, cheap, roundtrip-trivial.
- **Shortest-roundtrip decimal printing** (Ryū-class) is a named
  later rung and is near-unverified territory globally — a shard
  proof here would be novel work, priced accordingly.
- **Conversions**: format↔format (exact where widening; rnd where
  narrowing), Int→float (rnd), float→Int (truncation). Out-of-range
  float→Int LEANS saturating (the wasm trunc_sat / RISC-V precedent;
  x86's sentinel needs a fixup branch either way) — ratification
  decides (§13).


## 10. Rungs and gates

The exhaustive-toy-format idiom (symfpu's validation trick) is the
arc's principal derisking device: on a toy format every claim is
checked by ground compute over ALL inputs (2^16 pairs per binary op)
before the parametric proofs are attempted, and again at L2 against
the bit patterns. Cheap, mechanical, catches spec bugs where they are
cheapest.

- **R0 — std/rat + kit.** §2. Gate: corpus diff-clean; kit demo
  consumer.
- **R1 — L1 parametric model + named instances.** rnd laws
  (monotonicity, idempotence on representables, tie behavior),
  exhaustive toy-format gates, probe grids on F32/F64 samples. Gate:
  exhaustive toy pass + corpus diff-clean.
- **R2 — L2 bit-level model.** pack/unpack bijection (up to NaN
  class), computable ops ⊑ L1, toy-format exhaustive at the bit
  level. This is the arc's heavy proof rung (the Binary.v analogue).
- **R3 — surface + literals + hex printing.** Opaque surface types
  over the L2 representations; exact literal pipeline; `to_bits`
  canonicalization per §5.
- **R4 — wasm lowering.** f32/f64 fragments, tier-1 quotient
  theorems against the wasm model, tier-2 V8 gates (six-gate
  discipline per the lib arc). wasm first, per width-ordered
  coverage precedent.
- **R5 — x86 lowering.** SSE2 scalar fragments + min/max/convert
  fixups, tier-1 theorems against the x86 model, tier-2 TestFloat +
  on-CPU differential gates.
- **R6 — fma.** In-model from R1; this rung is the FMA3 fragment +
  wasm software path parity gate.
- **R7 — narrow formats.** Certified wide-compute per §7 + packed
  tensor demo in std/mem (the ML guard made concrete).

Flagship candidate for the arc: a dot-product / small-GEMM kernel in
BF16-in, F32-accumulate — exercises §6 packing, §7 certification,
and both targets from one decl.


## 11. Future named arcs (not this ledger's scope)

- **Error-bound automation** — the Gappa precedent: interval
  arithmetic over dyadic endpoints with directed rounding as a model
  parameter; the standard-model theorem (`fl(x∘y) = (x∘y)(1+δ)`,
  `|δ| ≤ 2^−(MW+1)`, no-overflow premise) proven once in std/float
  as the manual bridge until then.
- **Transcendentals** — RLIBM-class: correctly-rounded f32 unary
  functions are exhaustively certifiable in-shard (finite domain);
  correctly-rounded f64 rests on table-maker's-dilemma worst-case
  searches, which would enter as external-pedigree PINs or be
  sidestepped with faithful-rounding specs. Decide when a consumer
  exists.
- **Shortest decimal printing/parsing** (§9).
- **IEEE status flags as a World-style effect** (§8).
- **Constructive reals over std/rat** if a spec domain beyond ℚ is
  ever genuinely needed — zero-axiom std path, kernel untouched.


## 12. Non-goals, stated once

x87 (forever) · dynamic rounding modes · signaling NaN semantics ·
observable NaN payloads · fast-math or any value-changing
optimization · implicit fma contraction · decimal IEEE formats ·
binary80/binary128 (until a consumer) · FNUZ and E8M0 descriptor
arms (until a consumer) · flush-to-zero anywhere · IEEE exception
traps.


## 13. Open decision points

1. **Surface former shape**: type-level parametric former à la
   `(Word W S)` — `(Float EW MW …)` — vs dir-module instances
   (std/f32, std/f64, …) over one parametric core. The Word
   precedent cuts both ways post-revocation; decide at R3 with the
   generics/BuildCtx story in view.
2. **float→Int out-of-range**: saturating (lean) vs checked/partial.
   Ratification decides.
3. **Dyadic fragment home**: std/float-internal now; std/dyadic
   graduation decided at the interval arc.
4. **wasm engine-of-record set**: V8 only now; whether wasmtime joins
   tier-2 later.
5. **Rung order flexibility**: R3 (surface/literals) may swap after
   R4 if the first consumers are lowering-side; consumer-driven.
