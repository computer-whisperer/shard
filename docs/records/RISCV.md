# RISCV.md — RECORDS (moved out of docs/RISCV.md on 2026-09-02)

> **STATUS: RECORD.** Dated rung/slice records split out of [../RISCV.md](../RISCV.md) so the LAW ledger stays readable in one sitting. Section numbers are exactly as they were in docs/RISCV.md; a citation `RISCV.md §N` for a section listed here resolves to this file. Nothing here is normative unless the law ledger says so; any "NEXT" pointer is history.

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
- **G2b — the call lowering (LANDED 2026-07-17, Opus-authored per
  the split):** rv_enc_image = the entry-first multi-function image
  (per-index offset table, x86 enc_image's shape); RvCall k →
  `jal ra, rel` — RISC-V J-immediates are self-relative, so the
  offset arithmetic has NO instruction-length fixup (cleaner than
  x86's +5). Poison reason 4 repurposed to "RvCall unresolvable"
  (single-function enc keeps its fence — the enc_func/enc_image
  split), reason 9 added: jal out of J-type reach (±1MiB). The
  PRIVATE CONTROL STACK materializes as the fixed ra spill:
  non-leaf functions (any RvCall in the body) get
  `addi sp,sp,-16; s[dw] ra,8(sp)` / restore + ret, sp 16-aligned
  (psABI, both widths); **sd/ld (rv64) vs sw/lw (rv32) = the first
  width-dependent encoding beyond shamt/li** (rv_store_ra/
  rv_load_ra). In-body RvRet in a non-leaf expands to the FULL
  epilogue (12 bytes vs the leaf's bare 4-byte ret — a bare ret
  there would return through clobbered ra); size arithmetic threads
  a non-leaf flag. Scoreboard: rv64 48 / rv32 43 vectors, 0
  disagreements; the G2 wire regenerated ADDITIVE-ONLY (0
  deletions, 34 added lines — independently reproduced at review
  against the committed G2 encoder), leaf functions byte-identical
  in and out of images; teeth on the new machinery (corrupted jal
  offset → faults; corrupted nest expectation → named FAIL, exit 1,
  reproduced at review). New vector shapes: forward + BACKWARD jal
  (`jal ra,-16` byte-exact vs llvm-mc), 3-function nest with
  non-leaf mid, live value parked in s1 across a call, the smoke
  sum-loop-with-call, stores-through-callee with memory readback,
  early-return-through-epilogue. Harness needed ZERO changes
  (entry-first = the blob start it already calls; encoded prologues
  borrow the process sp qemu-user provides, verified live).
  **STOP-RULE OUTCOME: no corners hit** — the only stack use in the
  entire encoder is the single fixed ra slot; locals / callee-saved
  spills / frame pointers / outgoing-arg slots (x86 §4.3 data-stack
  territory, unbuilt there too) remain untouched, as ruled.
  **G3 note recorded:** the model has no ra/sp, so piece theorems
  state over the PURE body — the prologue/epilogue is a separable,
  differentially-checked encoder concern (the same trust split as
  G2); rvcall_bridge's citation story is untouched by this slice.
- **G3 — loopkit + symbolic piece theorems (LANDED 2026-07-17, this
  fork):** models/riscv/loopkit.shard (11/0) +
  examples/riscv_pieces.shard (994/0), both corpus-registered, both
  shardfmt-canonical. **THE MEASUREMENT CAME BACK: the entire pieces
  file was green on the FIRST CHECK — zero debug iterations.** The
  third transplant is template-grade mechanical; the §8-contract
  genericity claim is now tested three-for-three. Contents:
  straight-line register pieces at BOTH widths (add32/add64 + an
  add-mul showcasing the three-operand shape — the sum lands in t0
  without clobbering a0, retiring x86's mov dance), the byte-store
  piece (width-blind: register-indirect addressing does no
  arithmetic), the fill LOOP worker + piece theorem at BOTH widths,
  and the call arc at RV32 (a 29-field regs-general clobber-set cert
  welded into a two-call consumer through rvcall_bridge, a live value
  parked in s1 riding the cert's passthrough, the callee's body never
  computed into). Findings:
  - **The kit graduated at birth.** models/riscv/loopkit.shard is
    window-parametric from day one (the x86 §24 lesson applied
    prospectively) and the pieces file CITES it rather than keeping
    self-contained copies — the probe-copy stage is skippable when
    the design article already exists twice (loopgen_probe →
    x86_pieces).
  - **One kit fact family has NO x86 analog: rvlg_dec32/64.** RISC-V
    has no sub-with-immediate, so the counter decrement is
    `addi rd, rs, -1`; the model wraps the immediate at module width,
    putting the machine's counter residue at
    mod ((1+n) + (2^W−1)) 2^W — one full modulus above n. The
    collapse is mod_unique at the exhibited q = 1 decomposition, one
    fact per width. In exchange x86's xlg_sub + separate wrap-id
    cleanup FUSE into this single rewrite — the §3.13 sub-imm
    respelling reaches the proof layer exactly once, priced at two
    kit facts.
  - **The x0-fused guard is proof-invisible:** rv_cond of
    (RvBeq counter X0) computes to the same (int_eq v 0) spelling
    x86's synthesized CEqz produced, so the guard decider rvlg_ne
    transplants verbatim. The datasheet's branch fusion costs the
    proof layer nothing.
  - **The 32/64 worker proofs are textually identical up to modulus
    literals and _32/_64 lemma names** — the width-as-value design's
    G1 promise (ground-modulus residues) held through a full
    loop-induction proof, both directions.
  - **Fuel towers transplanted as constants:** S^9 at the worker,
    S^13 at the entry (+4 spine: outer seq / RvBlock / inner seq /
    RvLoop) — the same numbers as x86, because the body shape (5
    instructions) and spine depth are the same.
  - The fill piece returns the ADVANCED POINTER: a0 is both the
    pointer argument's home and the result home, so the machine hands
    back rvlg_adv for free (x86's returned a parked 0 in rax).
- **Emitter arc — deliberately unsequenced:** rides the common
  lowering step / imp arc (wasmgen/x86gen are frozen during I2e;
  models/imp is the neutral dialect riscv legs would serve). Not
  started until that sequencing is ruled.

## 9. Ratification record (all resolved 2026-07-17)

1. ~~Register-file scope: gp/tp as ordinary model registers?~~
   RATIFIED as built: 29 modeled + x0 + {ra, sp} encoder-owned; gp/tp
   are ordinary model registers (ABI-reserved only for foreign-code
   interop, a non-goal under platform-externs; E-profile fencing is
   orthogonal).
2. ~~RvMul in the base op table vs strict-I purity?~~ RATIFIED as
   built: RvMul stays in the base table (most real RV32 targets are
   IM; the wasm/x86 op tables both carry mul). A strict-I profile
   fence, if ever wanted, is a named check, not a model change.
3. ~~The G2 execution-differential leg: qemu-user install acceptable,
   or wait for hardware?~~ RESOLVED 2026-07-17: user installed
   qemu; qemu-riscv32/64 are the differential engines (§7.6).
