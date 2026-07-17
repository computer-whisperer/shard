# RISC-V — the third target: scope and design ledger

Status: **DRAFT — groundwork built on the `riscv` fork 2026-07-17,
awaiting ratification.** The model instantiates the target-model
contract of docs/LOWERING.md §8, exactly as docs/X86.md did for the
second target; every abstraction below is classified under X86.md §2's
three-bin discipline (permanent / training wheels with a named path /
commit-now). Section 8 records what is landed vs pending. Positions
inherited from the x86 ledger are cited rather than re-argued; the
genuinely new decisions (the width parameter, the doubled
encoder-owned register set, the sign-extension dragon) are argued in
full and are the items ratification should weigh.

## 1. Why RISC-V, and why both widths

RISC-V is where the C-class identity (shard binaries are the same kind
of object as C binaries) meets the widest real-hardware spread: 64-bit
application cores on one end, and on the other the 32-bit embedded and
accelerator world — MCUs, and exotic parts like Tenstorrent's devices,
whose on-chip service cores are RV32. Unlike x86 (where the 32-bit
model was ruled out forever — X86.md §1 — because x86_64 subsumed it
on every axis), for RISC-V **both widths are first-class targets**:
RV32 is not a legacy rung, it is the profile the embedded hardware
actually runs.

Structurally, RISC-V is also the cheapest possible test that the
lowering architecture is target-generic rather than
two-targets-generic: a third model speaking the §8 contract, with a
register file that is neither wasm's stack nor x86's 15-GPR
two-operand file.

## 2. Profile and non-goals

The modeled profile is the **RV32I / RV64I base integer ISA** plus the
M extension's `MUL` (in the op table now; the rest of M — `mulh*`,
`div/rem`, which are datasheet-true NON-trapping — is named growth
with its first consumer, the XDivU precedent). Explicitly out of v1,
each with its disposition:

- **RVC (compressed)** — named growth, encoder-only: v1 keeps every
  unit 4 bytes so the flattening size arithmetic is trivial (x86
  earned this with fixed-width branch forms; RISC-V gives it away).
  RVC changes byte math, never semantics.
- **F/D floating point** — arrives through the floats arc's
  value-parametric core when a consumer exists; separate register
  file, separate arc.
- **CSRs / interrupts / privileged modes** — never in the pure model
  (the world layer owns machine state when platform work arrives; the
  models/x86/world.shard precedent).
- **V (vector)** — far future; the extension point is the same
  register-class note as x86 §5's SIMD item.
- **RV32E (16-register embedded profile)** — named growth: a profile
  is a register-subset fence over the same model, not a new model.
- **A (atomics) / Zicsr / fences** — with concurrency, i.e. not before
  the memory-model arc exists anywhere in the tree.

## 3. The abstraction ledger

**Permanent, perf-neutral** (inherited, X86.md §3 arguments apply
verbatim):

1. **Structured reducible control** — RvBlock/RvLoop/RvBr/RvBrIf, the
   third target speaking the proven Block/Loop/depth shape. The
   flattening encoder owns labels and offsets (B-type ±4KiB, J-type
   ±1MiB reaches are encoder legality checks).
2. **Code is not data** — no PC as a value. Consequence: **AUIPC does
   not enter the model, ever.** PC-relative materialization is the
   encoder's business; the model's RvLi is the constant materializer.
3. **Private control stack** — RvCall/RvRet are structured; return
   addresses do not exist as values. This is the x86 no-rsp ruling
   **doubled**: RISC-V links through a register (`jal` writes the
   return address to `ra`) and spills it through `sp` in non-leaf
   functions, so BOTH x1/ra and x2/sp are encoder-owned and absent
   from the model register file. When the data-stack convention
   arrives, its pointer enters as one of the modeled registers
   (x86 §4.3's disposition), not as sp.

**Commit-now** (rework bombs if deferred — the new decisions):

4. **One model, both widths: the width is a VALUE** — see §4. Two
   sibling models would be the rework bomb: every proof template,
   recognizer, and cert form forked at birth.
5. **Three-operand ops day one** — `rd = rs1 OP src2`. The ISA is
   three-operand; a two-operand destructive shape would be an x86-ism
   baked into every cert. (The regalloc win is real: results need not
   clobber operands, so the emitter's temp discipline is freer than
   x86's.)
6. **x0 in the operand type** — reads 0, writes discard, exactly the
   datasheet. This is why the model has NO zero-test condition ctors
   and no store-immediate: beqz/bnez/mv/neg/store-zero are all
   spellings through X0, as in the real ISA.
7. **Fused two-register branches, 1:1 with the datasheet** — RvCond's
   six ctors ARE beq/bne/blt/bge/bltu/bgeu. Where x86's encoder
   synthesizes cmp+jcc from the fused Cond, the RISC-V encoder emits
   ONE instruction per ctor. Branches compare registers only (no
   compare-immediate exists in the ISA — materialize first); both
   signed and unsigned orders are present day one because the ISA has
   both and they cost two rv_sgn calls.
8. **The memory WINDOW [rvmemlo, rvmemhi)** — the x86 §23 soundness
   argument transplanted verbatim (a prefix is a wasm-ism; a process
   never has one; the window makes "never touches unmapped memory" a
   consequence of the cert). Pinned at all legs of both guards in
   examples/riscv_smoke.shard, below-window leg included.
9. **a0-a7 argument homes** — the standard calling convention's
   argument registers, kept as OUR convention (platform-externs: no
   libc, we own the ABI; keeping the standard homes preserves the
   option of conforming interop later). Result = a0. The s-register
   local-home growth (x86's 6→12 precedent) arrives with the lowering
   arc.

**Training wheels with named growth paths:**

10. **Byte-only memory** (LBU/SB). Wide LW/LD/SW/SD and sign-extending
    LB/LH/LW are growth arms; RISC-V's natural cell widths matter for
    parity, and the wasm→x86 mem-fragment history says the byte rung
    carries the proof story first.
11. **Register-indirect-only addressing** (RvAReg). The native
    base+offset form `off(rs1)` is **named growth #1** — it is what
    makes real RISC-V codegen compact — and enters as a new RvAddr
    arm, touching no existing cert.
12. **No W-suffix ops yet** — see the dragon in §5.
13. **Immediate legality unchecked in the model** — RvSImm wraps at
    module width; whether a value fits I-type's 12 signed bits (or
    needs an RvLi expansion) is encoder legality, the x86 SImm=imm32
    precedent. Likewise `sub`-with-immediate has no encoding (addi of
    the negation is the spelling) — the model does not care, the
    encoder refuses or respells.

## 4. The width parameter (the headline design)

`(rvxlen RvXlen)` — `(RV32) | (RV64)` — lives in the module record
beside the memory window, and the interpreter threads it to exactly
the places whose semantics read it:

- `rv_mod` / `rv_wrap` — the wrap modulus (2^32 / 2^64);
- `rv_bits` — the shift-count mask (the datasheet's lower-5/6-bits
  rule, spelled `mod count bits`);
- `rv_half` / `rv_sgn` — the signed view's pivot, feeding sra, slt,
  and the signed branches.

Why a value and not two models or a meta-level parameter: on a
CONCRETE module, compute collapses `(rv_mod (RV32))` to the literal,
so every ground and symbolic-value proof sees exactly the
ground-modulus residues that wasm/x86 proofs see — pinned by
add_sym32/add_sym64 in the smoke file, which close by one
(compute both) each, RV32 and RV64, over the SAME instruction list.
The floats arc's value-parametric core is the precedent; meta/proof's
WrapK (modulus-parametric premise walk) is the machinery that already
speaks per-width moduli on the emitter side.

The representation-collapse lemmas (§8-contract item 4) are the GROUND
per-width forms `rv_wrap32_id` / `rv_wrap64_id` (transplants of
wasm's/x86's, in the model file). Emitters always discharge at a
concrete module's ground width, so no premised width-parametric lemma
is needed; if the common lowering step ever wants one, it is kit
growth, not a model change.

## 5. What is genuinely different from x86 (divergence table)

- **Three-operand, not two-operand destructive** (§3.5).
- **Branches are fused in the SILICON** — the model's Cond discipline
  stops being an abstraction and becomes the datasheet (§3.7).
- **x0** — a constant-zero architectural register; no x86 analog
  (§3.6).
- **TWO encoder-owned registers** (ra, sp) instead of one (rsp)
  (§3.3).
- **All instructions are 4 bytes** (v1, no RVC) — the flattening
  encoder's size arithmetic is uniform where x86 needed fixed-width
  branch-form tricks.
- **THE SIGN-EXTENSION DRAGON (named, deliberately not built):**
  RV64's 32-bit-operand forms (`addw/subw/sllw/...`) SIGN-extend
  their results, where x86_64's 32-bit forms ZERO-extend. imp's U32
  tier rode x86's zero-extension for the maskless-native path
  (IMP.md §2a); the RV64 analog will need masking or the Zba/Zbb
  extensions' unsigned-word ops, and RV32's U32 story is
  native-width (no masking at all — the embedded win). **Do not
  improvise W-forms early**: they enter with the imp-lowering rung
  and a real consumer, where the design is forced honestly, exactly
  as XBin32 entered x86 at the V2 rungs and not at probe A.

## 6. The model

`models/riscv/riscv.shard` — an ordinary library, zero kernel/loader
changes (the ISA.md hard fence). Naming: `Rv`/`rv_` prefixes
throughout (distinct basenames from the wasm/x86 siblings, the house
pattern). The shapes, sibling-for-sibling:

| piece | riscv | x86 sibling |
| --- | --- | --- |
| register file | RvRegs (29 fields, ABI names) | Regs (15) |
| operands | RvSrc (RvSReg/RvSImm), RvAddr (RvAReg) | Src, Addr |
| ALU | RvBin (three-operand), rv_bop (width-threaded) | XBin, xbop_val |
| conditions | RvCond = the six branches | Cond (fused, synthesized) |
| outcomes | RvOut (RvNorm/RvBrk/RvRetO/RvTrap) | XOut |
| SCC | rveval_instr/call/loop/seq (fuel = depth) | xeval_* |
| denotation | rvrun_regs → rvrun_fn (a0) → rvcall_fn_mem/rvcall_fn | xrun_regs → … |
| keystone | rvcall_bridge | xcall_bridge |

The bridge proof transplanted verbatim (it never opens an individual
register, so the wider file rides along) — first check, as did the
whole file.

## 7. Contract instantiation (LOWERING.md §8, item by item)

1. Fuel big-step denotation, additive-slack-ready, fuel = depth — **landed** (G1).
2. Named SCC stop points (rveval_call/rveval_loop/rveval_seq) — **landed** (G1).
3. Call-composition keystone (rvcall_bridge) — **landed** (G1).
4. Representation-collapse lemmas (rv_wrap32_id/rv_wrap64_id) — **landed** (G1); per-op discharge kit grows with the emitter arc.
5. Literal-spelling discipline — **landed** (G1), pinned by the smoke file's symbolic pieces at both widths.
6. Encoder + engine differential — **landed** (G2). One width-blind
   encoder (models/riscv/encode.shard — the width picks shamt masking
   and RvLi legality, never layout) + a freestanding qemu-user
   differential at BOTH widths (examples/riscv_diff_run.shard emits
   model-computed expectations; riscv_diff.c replays them under
   qemu-riscv64/qemu-riscv32): 69 vectors, 0 disagreements. qemu
   plays V8's role from the wasm arc; hardware (an MCU, or a part
   like Tenstorrent's) remains the eventual realest gate. Every
   emitted encoding is byte-exact vs llvm-mc (dev-side check, not a
   committed gate — the byte-tie gate proper is the bytetie arc's
   later business).
7. Memory denotation over std/mem with window guards — **landed** (G1).

## 8. Slices

- **G1 — the groundwork model (LANDED 2026-07-17, this fork):**
  models/riscv/riscv.shard (972/0) + examples/riscv_smoke.shard
  (994/0, 22 claims), both corpus-registered, everything first check
  after one paren-balance fix in the rv_args nest. Findings worth
  keeping: (a) the xcall_bridge proof is register-file-blind — it
  transplanted to a 29-field record with pure renames; (b) the CANON
  advisory profile of the interpreter matches x86's exactly (C4/C6/C8
  on the same SCC members — the explicit ctor-rebuild arms are the
  proof idiom, accepted models/ status quo); (c) sra spelled through
  `ediv` of the signed view keeps every shift primitive on
  nonnegative operands (no bshr-on-negative dependency); (d) the
  width parameter cost symbolic reduction NOTHING — add_sym closes by
  compute at both widths with ground-modulus residues.
- **G2 — encoder + qemu differential (LANDED 2026-07-17, this fork;
  byte-emit/runner files Opus-authored per the standing split):**
  models/riscv/encode.shard (972/0, totality-gated, no claims) +
  examples/riscv_diff_run.shard (972/0) + examples/riscv_diff.{c,sh}
  + corpus registration (2 check targets + a qemu-guarded
  differential pin). Scoreboard: rv64 36 vectors / rv32 33 vectors,
  0 disagreements (~0.3s end to end); non-vacuity demonstrated three
  ways (corrupt code byte → exit 3, corrupt expectation → exit 1
  with a named FAIL line — independently reproduced at review — and
  trap-teeth: widening a vector window in a scratch copy makes the
  still-SIGSEGVing core a scored disagreement). Findings:
  - **The §5 dragon bit at the materializer, exactly once:** the
    planned closed-form RvLi fence ([0,2^31) ∪ top range) is WRONG
    at RV64 — the standard li carry-fix (lo12 sign bit set → hi20+1)
    can round hi20 to 0x80000, which lui SIGN-extends, and on RV64
    the carry propagates into the high half. The shipped fence is
    SELF-CHECKING instead: form the (hi20, lo12) split, SIMULATE
    what `lui; addi` materializes at the module width, and refuse
    unless it equals the target's rv_wrap image. The real RV64
    positive edge is 0x7FFFF7FF, not 2^31−1. RV32 encodes every
    value.
  - **Refusals are reason-coded POISON words** (reason<<16: low bits
    00 = an illegal instruction with C absent → SIGILL, confirmed
    under qemu), one unit wide so flattening arithmetic is
    untouched; 8 named reasons (bad depth / J-reach / B-reach /
    RvCall / sub-imm / mul-imm / I-imm domain / RvLi unencodable) in
    the encoder header.
  - RvCall = poison (the x86 XCall→int3 precedent); in-body RvRet =
    `jalr x0, 0(ra)`, correct precisely BECAUSE the fence keeps ra
    unwritten; enc appends a trailing ret unit.
  - Data page 0x40000000 MAP_FIXED works under qemu-user at both
    widths; below-window and at-hi probes SIGSEGV as the model
    predicts. Trap leg = in-process recovery (freestanding
    rt_sigaction + hand-written setjmp/longjmp), single-invocation
    harness like x86_diff.c.
  - Harness/toolchain facts: one C source both widths (own _start,
    raw ecall syscalls, a trampoline that saves/restores s0-s11 so
    encoded code may clobber ANY model GPR); no riscv libc exists on
    the box — clang -nostdlib -ffreestanding -fno-builtin
    -fno-stack-protector (clang lowers bare loops to memset/memcpy
    even under -nostdlib), linker = rust-lld through a symlink NAMED
    ld.lld (it flavors by argv[0]; -fuse-ld= needs the full path);
    rv32's `unsigned long` is 32-bit, wire values parse as unsigned
    long long. Host clangd lints riscv_diff.c's a0-a7 asm registers
    as errors under the x86 default target — false positives, the
    file only compiles under --target=riscv*.
- **G2b — the call lowering (named, next):** RvCall → `jal ra, off`
  with a multi-function image + per-index offset table (x86
  enc_image's shape) and sp-based ra spill for non-leaf functions;
  the harness trampoline already presents a clean ABI for it.
  Nothing in G2 blocks it.
- **G3 — loopkit + symbolic piece theorems:** the wasm→x86 transplant
  play (guard/collapse/IH-at-fuel−1 templates at rv_wrap's moduli);
  measures whether the third transplant is as mechanical as the
  second (the §8-contract genericity claim, tested).
- **Emitter arc — deliberately unsequenced:** rides the common
  lowering step / imp arc (wasmgen/x86gen are frozen during I2e;
  models/imp is the neutral dialect riscv legs would serve). Not
  started until that sequencing is ruled.

## 9. Open questions for ratification

1. Register-file scope: 29 modeled + x0 + {ra, sp} encoder-owned — is
   reserving gp/tp as ORDINARY model registers acceptable? (They are
   ABI-reserved only for foreign-code interop, a non-goal under
   platform-externs; modeling them costs nothing and E-profile
   fencing is orthogonal.)
2. RvMul in the base op table vs strict-I purity (the M note in §2) —
   kept because most real RV32 targets are IM and the wasm/x86 op
   tables both carry mul; flag if strict-I profile fencing should be
   a named check instead.
3. ~~The G2 execution-differential leg: qemu-user install acceptable,
   or wait for hardware?~~ RESOLVED 2026-07-17: user installed
   qemu; qemu-riscv32/64 are the differential engines (§7.6).
