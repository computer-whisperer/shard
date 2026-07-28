# STREAM.md — Arc B: the streaming bin (the bin story generalized)

STATUS: RATIFIED 2026-07-26 (user ruling; drafted by Fable the same
day). Arc B is the ratified successor to the certificate pathfinder
arc (CERT.md §8 R4, §10: "Arc B opens next, rung 1 = the
replacement-basis measurement"). Charter sources: the design review's
Arc B section and the D8-closer delta
(docs/archive/DESIGN-REVIEW-2026-07-18.md), the full-arc review's
scoping (docs/archive/ARC-A-REVIEW-2026-07-26.md §2e–§2g). Rung B1's
scope was pre-ratified at the full-arc review (R4); the full ladder
was ratified with this ledger. Results that amend certificate law
(the D-number, DC3 evidence, final dialect ratification) are recorded
in CERT.md; this ledger tracks the arc.

## 1. Mission

Turn examples/sha256sum from a demonstrator into a conventional
program — and in doing so generalize the bin story. The deliverable
is NOT "sha again": it is the World/I-O composition surface every
future bin rides (window policy, entry contract, trace shapes,
artifact claims), forced honest by one real consumer, plus the
performance claim tested on silicon.

Demonstrator ground truth today (X86.md §32 tail): issues exactly ONE
read at cap 64888 (file-redirect only — a pipe's short read would
diverge); data window = a PT_LOAD at vaddr 0, loadable only via
`setcap cap_sys_rawio+ep` (page-0 hack, task #65); 64 KiB application
cap; scratch ELF emitter (examples/sha256sum/sha256sum_elf.shard),
models/x86/elf.shard unmodified. The conventional program: streams
until EOF, handles short reads, unbounded input via incremental SHA
state, loads with no capability (silicon leg joins CI), benchmarked
against coreutils, with a hand-pinned SHA-NI/SIMD variant against
OpenSSL and proven feature dispatch.

The arc is the "x86 truth serum" (review's phrase): it tests
application completeness, World/I-O composition, the compact
certificate dialect at scale, target-specific expert lowering, and
the performance claim, all in one owned domain.

## 2. The consumer debts this arc retires

Arc B was scheduled as the closer for every deferred interface that
needs a real consumer forcing its decisions:

- **D8 — the controlled-failure surface + ARTIFACT CLAIM forms**
  (docs/MEMORY.md D8, direction ratified 2026-07-12). Streaming =
  short reads + EOF = the observation relation for machine runs
  (syscall trace → contract observables with the Fail projection)
  with a live consumer. The `except` clause extends the `(bin …)`
  grammar — kernel growth through canon-owned files, a gated
  high-bar slice when its rung arrives. D8's open sub-questions
  (imp's reasoned Fail value, effect atomicity for the prefix
  theorem — the write shim's retry loop is the named case, stack
  family v1 mechanism, resource-axiom vocabulary) resolve in-arc
  with generated cert shapes in hand.
- **Runs/RunsWithin** (CERT.md §6) — the proof-facing cost/fuel
  interface. ZERO implementation exists anywhere (review-verified);
  this arc is its first landing. Exact fuel becomes
  interpreter-internal; sink 3's tower bookkeeping dies here; the D8
  observation relation gets its natural home.
- **Task #65 — window relocation**: bins' data segments above page
  0, so the ELF loads with no capability and the silicon
  differential leg can join CI. Until then every emitter reading
  the vaddr-0 layout is provisional by construction.
- **The generic bin tail**: window policy, entry contract, trace
  shapes — thin World mains + PURE artifacts (IMP.md's composition
  ruling). What sha256sum forces here must land as the DEFAULT tail
  every future bin gets, not sha-specific glue.
- **Certificate-arc residue, as rung B1** (ratified, CERT.md §8 R4):
  the replacement-basis measurement (the D-number §7's design waits
  on; DC3's gate evidence; the generator dialect's FINAL
  ratification) and the A1×A2 composition exercise.

## 3. Rung ladder

- **B0 — this ledger ratified.**

- **B1 — the replacement-basis measurement** (scope RATIFIED at the
  full-arc review, §2f/R4). Restate the 13 cmp_x_shblock_bN seams +
  the seam-consuming (weld-facing) region of imp_x_shblock in the
  conversion dialect; sqs_/sqxw_/gb_ stay. ONE new once-per-model
  12-slot list-inversion law (~500 lines) — the weld cite forces the
  top level UNPREMISED, so seams must survive the None fork (the
  ∀-y premise shape was exercised only as a leaf). Drop the cmp_
  family (~40.6k lines on disk) from the closure; SHARD_STATS both
  ways. Estimate: ~3–4.5k lines replacing ~40.6k, proof-authoring
  days on owned patterns. The two loop seams (b5 ≈ 9.4k lines, b1 ≈
  2.9k) are the unpriced make-or-break and double as DC3's gate
  evidence. Outputs, recorded in CERT.md: the D-number (§7's design
  opens), DC3 verdict evidence, final dialect ratification (or its
  falsification — see §5). Rung-design forks: where the restatement
  lives (extend std/sha256/sha256.xconv.shard is the default) and
  how cmp_ leaves the closure given impgen is FROZEN (a variant
  regeneration as measurement artifact, never a new emission mode).
  - **B1b (rides along)**: the A1×A2 composition exercise — a vxo_
    pin instantiated at xq8-boundary view reads (~90–140 lines),
    closing the recorded composition gap. The mem-capable statement
    tier stays a named door (§6), not this rung.

  **B1 RECORD (2026-07-26; measurement complete, B1b pending).**
  Landed: models/imp/probes/ilinv_probe.shard (ilv_inv12, e965ec2) +
  std/sha256/sha256.xchain.shard (37de92a, 71f0065, c700ea9, 2a7f586)
  — every statement byte-identical to its cmp_x_shblock_bN /
  imp_x_shblock original, corpus-registered, ZERO cmp_ citations in
  the chain. The design that survived contact (amends the R3
  provisional dialect — final ratification is the user's): chain
  interiors are UNPREMISED seams — ist_seam + the kept sqs_cN put
  the same segment term on both sides, the proof case-forks on it,
  fail forks close by compute (xm_scont/ist_cont/ix_sout mirror
  failure), and the Some-INorm fork derives arity by a change-fold
  into il_slen (il_wlen at loops), mints the twelve reads with ONE
  ilv_inv12 citation, and closes with ONE exact-conv of the next
  seam; the ∀-y premised shape remains the leaf/library form only.
  Loop seams closed WITHOUT any checkpointed-walk form (DC3's
  evidence): approach paths kept verbatim (the guard-fork trees are
  the semantic floor), the 12-deep exposures died. Per-seam
  formatted lines (replay → conversion): ten uniform seams 2,695 →
  172–173 (≈15.6x); b1 loop 2,922 → 212; b5 loop 9,401 → 3,031 (the
  kept guard tree dominates); b13 tail 1,330 → 885 (stop-strip only
  — no exposure existed); imp_x walk 2,884 → 272; support = 577
  lines of local instr copies/constants + 1,209 once-per-model
  inversion law. Text total: 43,451 → 7,969 on disk (5.5x; ≈9.5x
  excluding the two semantic-floor trees). THE D-NUMBER (idle box,
  bin/shard_check, whole closures; variant = scratch
  impgen_x86_out minus the 14 claims + import-swapped xchain,
  872/0 green): calls 1,008.4M → 814.2M (−19.3%); live peak 407.2MB
  → 267.0MB (−34%); maxrss 1.64GB → 1.22GB (−26%); wall 4.31s →
  3.54s; reader pair 317.1M → 205.6M; env_lookup pair 237.9M →
  194.4M. Decisions this record opens (CERT.md §8 R3, §11 DC3, §7):
  final dialect ratification, DC3 disposition, §7 design opening —
  all the user's.

  **B1b RECORD (2026-07-27; landed).** std/sha256/sha256.xcomp.shard
  (154 fmt lines, corpus-registered): vxq8_add = vxo_add's oracle
  skeleton (vx86_oracle_probe.shard) with both ∀-bound Int args
  instantiated at A2's xq8-boundary view reads — x0 = xp_loc of the
  segment's accumulator local (slot 11) through the patch view, x1 =
  xp_byte of the byte the segment just wrote (address xv7) through
  the mem view. FIRST CHECK, 957/0. The recorded composition gap is
  closed as the review's §2e predicted: the A1 soundness theorem
  (vxg_valid) and the A2 patch dialect compose in ONE claim with
  zero new machinery — neither execution is replayed, neither view
  is materialized; the read terms ride the citation as opaque
  values. The mem-tier fence stands as recorded: the validator
  family is mem-free by construction, and the mem-capable statement
  tier is its next growth rung, not an Arc A/B defect. Closure note:
  the file is the first to import BOTH the x86 out file and the acc
  probe — 2.38B calls / 1.22GB live / 4.17GB RSS per check, the
  incremental-checking debt's (task #62) loudest exhibit to date.
  With B1b landed, B1's WORK is complete; the rung closes on the
  three user rulings above.

  **B1 CLOSED (2026-07-28, user ratification of the B1 record):
  final dialect RATIFIED with the unpremised-interior amendment
  (CERT.md §8 FINAL DIALECT RATIFICATION — fences named there);
  DC3 CLOSED-DORMANT on the loop-seam evidence (CERT.md §11); §7's
  design formally OPEN under its gated-slice protocol (CERT.md §7);
  the coverage arc UNFREEZES per §4. Next rung: B2.**

- **B2 — the streaming World design note** (paper before build, the
  house pattern). The read-until-EOF World shape: the read shim's
  short-read/EOF contract (X86.md §32 discipline — syscall-direct,
  theorem-pedigree shims); the observation relation for machine
  runs; Runs/RunsWithin's first concrete interface; which D8 claim
  form the streaming bin lands on (`given` vs `except` — if
  `except`, the (bin …) grammar slice gets its own gated rung);
  the incremental-SHA state shape and the entry contract. Ratified
  before B3 emits anything.

- **B3 — the conventional program on silicon.** #65 relocation
  (elf.shard proper, two-PT_LOAD with data above page 0, no
  capability; silicon leg joins CI); the read loop until EOF with
  short-read handling; the 64 KiB cap removed; incremental SHA
  state across reads; the generic bin tail extracted. The artifact
  claim lands in one of D8's three forms; a bare MET is never an
  artifact verdict. Cert story consumes the B1-ratified dialect —
  the streaming loop is exactly the shape the loop seams priced.

- **B4 — benchmark: scalar vs coreutils sha256sum.** The C-class
  identity is the success criterion (parity-class, not "close for a
  verified toolchain"). Measured on the CI-joinable binary.

- **B5 — the expert leg: SHA-NI/SIMD hand-pinned variant vs
  OpenSSL.** This is the FIRST real non-canonical consumer — the
  named trigger for clause 1 of the validator's clause architecture
  (CERT.md §4): decide build-clause-1 vs hand-prove-the-leg when
  the artifact exists. Rung-design forks: SHA-NI vs AVX2 tier
  first; the dispatch-visible variant contract.

- **B6 — proven feature dispatch.** CPUID/feature detection selects
  only certified variants; the dispatch proof composes the B4/B5
  artifacts under one bin contract.

## 4. Sequencing rules

- **B consumes Arc A's ratified forms ONLY.** No new replay-dialect
  certs anywhere in this arc (CERT.md §10 standing consequence —
  another 200k-line artifact pile is the one genuinely embarrassing
  outcome on record).
- **B1 runs first** and everything that emits waits for it: final
  dialect ratification rides the measurement, and B3's streaming
  loop cert needs exactly the loop-seam result B1 prices.
- **§7 (storage/DAG) design opens when B1's D-number lands** —
  parallel to B2+, gated-slice protocol, not part of this arc.
- **Arc C's paper half may run alongside; the coverage arc
  unfreezes after B1's dialect exercise; Arc D last** (CERT.md §10).
- Emit-layer changes (elf.shard, shims, SHA-NI emission) follow
  X86.md §32's platform-extern law: syscall-direct, zero C,
  differential-gated.

## 5. Gates and falsification

Measured gates, per rung: B1 — text (est. 3–4.5k replacing ~40.6k)
and SHARD_STATS calls/RSS both ways, per-seam constancy on the
uniform seams, and the loop seams land or DC3 opens with their
numbers as evidence. B3 — the differential leg green in CI without
capabilities; the artifact verdict carries its claim form. B4/B5 —
wall-clock numbers on record vs coreutils/OpenSSL.

The architecture is materially DOWNGRADED if (design-review §6, the
two silicon gates this arc owns):

- the streaming scalar SHA artifact is dramatically slower for
  reasons inherent to the proof-facing IR rather than an immature
  backend;
- an expert SHA-NI schedule cannot be expressed without target
  semantics leaking back into the high-level requirement.

And from the certificate side: if B1's loop seams show generated
walks still too big after conversion, DC3 (checkpointed-walk form)
opens as a priced candidate rather than the dialect ratifying final.
On any downgrade: stop and redesign before the coverage compiler
learns the dialect — same law as Arc A's.

## 6. Non-goals and named doors

- **Tracing GC** stays a non-goal (docs/MEMORY.md law).
- **Coverage emission** stays frozen until after B1 (CERT.md §10);
  its four design debts close on paper during B.
- **The mem-capable validator statement tier** (validator family
  covering loads/stores — required before vxg_valid can replace any
  block-chain link) is a named door for the validator family's next
  growth rung, opened by demand, not in this arc's scope.
- **Many-legal-targets validator generality** beyond clause 1's
  decision stays deferred (CERT.md §4).
- **The 64-bit/length-extension SHA variants, other hash families**:
  out. One program, made conventional and fast.
