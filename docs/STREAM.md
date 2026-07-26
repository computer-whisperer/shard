# STREAM.md — Arc B: the streaming bin (the bin story generalized)

STATUS: DRAFT for ratification (Fable, 2026-07-26). Arc B is the
ratified successor to the certificate pathfinder arc (CERT.md §8 R4,
§10: "Arc B opens next, rung 1 = the replacement-basis measurement").
Charter sources: the design review's Arc B section and the D8-closer
delta (docs/archive/DESIGN-REVIEW-2026-07-18.md), the full-arc
review's scoping (docs/archive/ARC-A-REVIEW-2026-07-26.md §2e–§2g).
Rung B1's scope is ALREADY RATIFIED (review R4); everything else
below is draft until ruled on. Results that amend certificate law
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
