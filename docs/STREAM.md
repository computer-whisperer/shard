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
  before B3 emits anything. **RATIFIED 2026-07-29 — design = §7 of
  this ledger, all three §7.8 forks ruled to their leans. B2 is
  CLOSED; B3 is the active rung.**

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
  verified toolchain"). Measured on the CI-joinable binary. Includes
  Fork F's deferred half: the read cap retunes from the proving-rung
  4096 as a REGENERATION (the M5 relocation's every-literal-shifts
  mechanism, priced once, reused). The falsification gate needs
  ATTRIBUTION, not just a number: "dramatically slower for reasons
  inherent to the proof-facing IR" vs an immature backend — so the
  record carries cycles-per-byte and what coreutils actually
  dispatches to on the box, not wall clock alone.

- **B4b — the cmp_ migration touch** (SLOTTED 2026-08-01, user
  ruling at the B3-close progress review). Drop the cmp_ replay
  family from the COMMITTED block closure — the variant
  regeneration B1 already priced (a measurement artifact mechanism,
  never a new emission mode), landing sha256.xchain.shard as the
  closure's only block-chain dialect. R4 had tied this to "the next
  block-chain touch"; the M5 base-65536 shift WAS such a touch and
  the migration did not ride it, so the implicit trigger is RETIRED
  in favor of this explicit rung. Claims on the table: the measured
  −19.3% calls / −34% live peak / −26% maxrss on every closure
  check, and ~40k generated lines leaving the committed repo.

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
  AMENDED 2026-08-01 (B3-close progress review, user ruling): ONE
  parallel design track at a time — §7's storage design note drafts
  alongside B4; C-paper and the coverage arc's paper debts stay
  QUEUED until B5 is underway (the decision-bandwidth rule: B5's
  clause-1 adjudication wants that bandwidth).
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

## 7. B2 — the streaming World design note (RATIFIED 2026-07-29)

**RATIFICATION (2026-07-29, user ruling: "agreed on all of those
— ratify it and continue").** The §7 body is law: stream_main as
the one spec-level combinator (7.1), the read contract with the
fail-stop error leg (7.2), the per-bin contracted-event projection
as D8's observation relation (7.3), Runs/RunsWithin v1 in the
monotone threshold form (7.4), the UNCONDITIONAL claim form with
the except-grammar slice staying deferred (7.5), and the
list-grain incremental correspondence (7.6). All three §7.8 forks
ruled to their leans — the rulings are recorded inline there, with
the rejected options' costs kept as the record of why. B3 builds
to §7.7's list.

Ground truth this note stands on: the read rung landed (X86.md §50
— LxRead delivered-bytes events, LxGoM, lx_fill/lx_take, the
two-instruction read shim, pins green) with the v1 fence "effect
points in straight-line positions only; read-until-EOF loops are
the streaming rung's named growth". This note designs that growth
plus the interfaces B2 owes (D8 observation relation,
Runs/RunsWithin, claim form, state shape, generic tail).

**7.1 The generic bin tail = ONE combinator.** The streaming main
is written once, as a spec-level World combinator (working name
`stream_main`): given a pure incremental step `st × chunk → st`
and a finalizer `st → bytes`, the main is THE read-until-EOF loop
— read into the window at a ground cap, step on delivered bytes,
repeat until ret 0, write the finalized output, exit 0.
sha256sum's main becomes `stream_main sha_step sha_fin` and every
future filter-shaped bin instantiates the same combinator (thin
World mains + PURE artifacts — IMP.md's composition ruling). The
window policy is the combinator's parameter pair (base, cap);
task #65 relocates base above page 0 before B3's ELF.

**7.2 The read contract.** Per iteration: ret ∈ [0, cap]; ret = 0
is EOF and ONLY EOF; short reads are legal at any time (the pipe
case the one-shot bin had to exclude by file-redirect discipline —
the streaming loop makes pipes first-class). ONE pulled-in named
growth, needed for honesty: the ERROR LEG. Today's kernel model
normalizes a negative return draw to 0 — under a loop that reads
as EOF, so a real read error would silently produce a digest of a
prefix. A conventional program must fail-stop instead: the model's
read arm grows an error outcome (negative draw ⇒ error event, not
EOF), and stream_main's loop exits nonzero on it. Without this leg
the B4 coreutils comparison would be dishonest on error paths.
The write side stays v1 offered-bytes (single write, no retry
loop); effect atomicity stays one-syscall-grain.

**7.3 The observation relation (D8's first sub-question,
resolved here).** Defined ONCE at the kernel-model event grammar:
each bin DECLARES its contracted-event projection (which of
LxRead/LxWrote/LxExited… its requirements observe; undeclared
events are ghost). For sha256sum the projection is total: the
LxRead sequence DEFINES the input (delivered bytes concatenate to
the message), LxWrote carries the digest, LxExited the verdict.
The artifact-satisfaction statement is D8's disjunction over
PROJECTED sequences: machine run = Done (projection equals spec
main's) ∨ Fail(declared family) with the projection a strict
prefix of spec's followed by the fallback signature (disjoint from
contracted observables except exit code). The syscall-trace side
of the relation is the W-rung differential layer's existing
vocabulary — no new trust surface.

**7.4 Runs/RunsWithin v1 (first landing, CERT.md §6).** No
existential machinery: `RunsWithin P args C v` is the MONOTONE
THRESHOLD form — "∀ f ≥ C: eval f args = Some v" — with the
fuel-monotonicity lemma proven once per machine and composition by
cost addition. Exact fuel becomes interpreter-internal; the bin
equations shed their S^N towers (B1's per-seam fuel constants
G_N = 24N + c are exactly these cost functions in embryo, so the
cost algebra has measured ground truth from day one). For the
streaming bin the cost is input-length-linear: C(n) = a·⌈n/cap⌉ +
b. First consumers: the B3 bin equation's machine leg, then sink
3's tower bookkeeping dies by attrition, never by rewrite.

**7.5 The artifact claim form (D8 ladder, worked).** Per
condition: memory — fixed window, no allocator, oom DISSOLVED by
construction (the landed embedded tier's shape); stack — no
recursion, DISSOLVED; overflow — the only counter is the SHA-256
length field, and FIPS 180-4 itself bounds messages < 2^64 bits,
so the bound lives in the REQUIREMENT's domain (spec vocabulary,
not failure vocabulary). Consequence: the streaming sha256sum
lands **UNCONDITIONAL — claim form 1, no given, no except** — and
the `(bin …)` except-grammar kernel slice stays deferred to the
coverage arc (the calc pathfinder remains its honest forcing
consumer). The one open question here — what the program does on
an input exceeding SHA-256's own domain — is Fork A in §7.8.

**7.6 Incremental state + the correspondence theorem.** State =
(H-vector of 8 words, pending tail < 64 bytes, total length);
`sha_step` folds whole 64-byte blocks from window bytes into H and
carries the remaining tail; `sha_fin` pads and produces the 32-byte
digest. The spec-side theorem, proven once at list grain with no
memory vocabulary: folding chunks then finalizing = one-shot
sha256 of the concatenation (chunk-association law). The machine
leg then refines exactly this via the B1-ratified dialect — the
streaming loop is the shape the loop seams priced.

**7.7 What B3 builds (consequences, in order).** #65 window
relocation in elf.shard proper (capless load; the silicon
differential leg joins CI); the model's read-error outcome + the
stream_main combinator + its trace theorem; the incremental spec
correspondence; the sha instantiation replacing the one-shot cap
bin; the bin verdict line carrying `MET (artifact: unconditional)`
— a bare MET is never an artifact verdict (D8 law).

**7.8 Open forks — the three decisions this note needs ruled.**
Each is stated self-contained: the question, the options, what
each costs, and the lean. **ALL THREE RULED 2026-07-29: the lean
adopted in each case** (rulings marked inline; the non-adopted
option's cost text stands as the REJECTED-because record).

**Fork A — what does the program do on an input too large for
SHA-256 itself?** RULED: no runtime check. SHA-256 is only
defined for messages shorter
than 2^64 bits — 2^61 bytes, about 2.3 exabytes. That bound is
FIPS 180-4's, not ours; a streaming bin could in principle be fed
more.
- *No runtime check (lean).* The requirement's domain is "the
  messages SHA-256 defines"; larger inputs are documented
  non-contract. Cost: none. At 100 GB/s a stream needs roughly
  nine months to cross the bound — it is physically unreachable,
  and a check that can never fire is theater.
- *Checked counter with fail-stop.* The bin gains a length counter
  and a declared failure mode. Cost: the claim stops being
  unconditional — a declared failure mode is exactly what the
  `(bin …)` except clause expresses, so this option drags in the
  kernel grammar slice the design otherwise avoids entirely
  (deferred to the coverage arc, above).

**Fork B — how is "a read failed" spelled in the event
vocabulary?** RULED: a new payload-free event kind. The machine
model records a run as a list of events:
LxRead (these bytes were delivered), LxWrote (these bytes were
written), LxExited (this exit code). §7.2 adds a new observable
outcome — a read that errors rather than delivering bytes or
signaling EOF. Two spellings:
- *A new payload-free event kind (lean).* Precedent: LxGoM is
  already a payload-free event. Purely additive — no existing
  event constructor changes type, so no existing proof, pin, or
  differential check is touched; bins that don't declare the new
  event never see it.
- *An error payload on LxRead itself.* LxRead's constructor
  changes type, so everything that pattern-matches it must be
  revisited. The error carries no data we contract on, so the
  churn buys no expressiveness.

**Fork C — what type is a "chunk" in stream_main's spec-level
signature?** RULED: a plain byte list. The combinator takes a
step function
`state × chunk → state`. The question is what the spec-level
program means by `chunk`:
- *A plain byte list (lean).* The spec side never mentions memory.
  The correspondence theorem (§7.6: folding chunks then finalizing
  equals hashing the concatenation) is a pure list statement, and
  the fixed memory window appears only in the machine leg — the
  same spec-pure/machine-view split the B1 dialect already prices.
- *A memory-view chunk (bytes-in-window).* Spec statements acquire
  memory vocabulary, every consumer of the combinator couples to
  the window representation, and the correspondence theorem stops
  being memory-free.

**7.9 The machine leg, laddered (B3 execution plan — RATIFIED
2026-07-29, user: "agreed on both"; forks D and F ruled to their
leans, rejected options' costs kept as the record; spec-side
§7.6/§7.7 pieces already landed: sha_stream_corresponds + the
sst_trace ghost theorem).**

What already exists and is reused as-is: the FROZEN compression
block artifact and its Nat-counted fold loop (both targets; the
x86 spill idiom at [65504,65536)); the padding and hex tiers; the
one-shot window layout W@64960 K@65216 H@65472 — H already has a
persistent memory home; the hand-built thin-main pattern
(I2e-4c-3: the main is ~11 instructions delegated whole into the
bridge, NO tool growth); enc_winelf at base 65536 (#65 groundwork,
landed); the B1-ratified conversion dialect for every new cert.

The ladder:

- **M0 — the effect-loop walk probe.** The genuinely NEW proof
  ground is a BACKWARD BRANCH around an effect point (every landed
  effect sits in a straight-line position — §50's fence, lifted
  here). De-risk it minimal first, in the §20/§21 probe tradition:
  a byte-counter bin (read-until-EOF at a ground cap, count
  delivered bytes, print the count) — the machine sst_fuel analog:
  a fueled walk quantified over per-iteration oracle draws, the
  three-way branch (error/EOF/data), the loop-carried state at a
  ground address. Everything sha adds later is pure artifacts this
  probe doesn't need.
- **M1 — the streaming absorber artifact (pure, imp grain).**
  sha_step at machine grain: merge the pending tail with the
  delivered bytes, fold every whole block via the frozen block
  loop, keep the remainder pending, bump the count — H updated in
  place at its landed home. Artifact theorem reads back as
  `sha_step` of the window state.
- **M2 — the streaming finalizer artifact (pure, imp grain).**
  sha_fin at machine grain: pad the pending tail with the TOTAL
  count (the landed padding species with its length source
  changed), fold the last block(s), hex to [0,64). Reads back as
  `bytes_hex (sha_fin st)`.
- **M3 — the streaming main (hand, thin).** The read-until-EOF
  loop instantiating M0's proven walk shape with M1/M2 at the two
  call boundaries. The composition ruling holds: compute never
  lives in the I/O proof.
- **M4 — the weld.** The loop invariant (the window realizes the
  ShaSt) composed with sst_trace's ghost form: the machine run's
  projection = "read everything, wrote sha256_hex(everything)" ∨
  the declared Fail legs — D8's disjunction, closed.
- **M5 — emission and gates.** The image regenerated at base
  65536 (every ground address +65536 — regeneration, not
  re-proof), enc_winelf ELF, corpus registration, the differential
  leg fed by PIPES (short reads are now first-class — the §51
  file-redirect discipline retires), capless silicon joining CI,
  and the verdict line: `MET (artifact: unconditional)`.

**Fork D — where does the pending tail live across iterations?**
RULED: fixed read address + copy-the-tail-home.
The read effect's destination address is the question: the walk
stack prices GROUND addresses everywhere (shim certs, frame
lemmas, the differential's trace grammar all speak fixed
addresses).
- *Fixed read address + copy-the-tail-home (lean).* The read
  always lands at one ground buffer address; after folding whole
  blocks the ≤63 remaining bytes are copied to a fixed tail home;
  the absorber consumes tail-then-buffer. Cost: a ≤63-byte copy
  loop artifact and its (routine, landed-species) proof per the
  absorber. Benefit: every effect point in the program keeps a
  ground address — zero new machinery in the shim/walk layer.
- *In-place rolling offset.* The read lands at buffer + pending
  length, so no copy — but the read's buf argument varies per
  iteration, forcing symbolic-address read certs: new machinery in
  the one layer (the effect/walk stack) where everything landed is
  ground. Saves a 63-byte copy; costs a new cert species.

**Fork E — absorber call grain?** Settled by precedent, recorded
for completeness: ONE pure XCall'd absorber per iteration. The
4c-3 pipe-boundary finding (weval's effect budget forces
delegating pure work whole behind a call) plus the thin-main law
leave no second option worth pricing.

**Fork F — the read cap now?** RULED: 4096 for the proving rung.
The spec article pinned one page
(4096) provisionally; the machine window fits caps to ~56 KiB
below the W frame.
- *Keep 4096 for the proving rung (lean).* Smallest surface while
  the novel walk lands; B4's benchmark retunes the cap as a
  REGENERATION (the same every-literal-shifts mechanism as the
  #65 relocation — priced once, reused).
- *Pick the benchmark cap now (~56 KiB).* Saves one regeneration
  later; costs tuning discussion before the walk shape even
  exists, on a number B4 will re-measure anyway.

**Fork F retune LANDED (B4, 2026-08-01).** Cap = 61440 (15 pages),
the largest 4096-multiple below the W frame — the measured free
span is buffer@65664 → W@130496 = 64832 bytes, so the "~56 KiB"
estimate above ran low. Exactly as priced: a pure regeneration,
ZERO proof-step changes (the tight kb-cert's farkas multipliers
are cap-invariant — the sum telescopes to −1 at any cap ≡ 0 mod
64; every other site was translation-uniform slack), absorber-path
fuel heads +896 (960 vs 64 blocks/iteration), entry claims S^705 →
S^1601. One site outside the priced file list: the cap premise
physically lives in std/sha256/sha256.sweld.shard's seven
absorber/readback statements (`le r 4096` → `le r 61440`) — a
caller cannot widen a premise, so the sweld article moves with the
cap by construction. Silicon leg 61 → 67 rows (cap-cross rows
re-bracketed at 61439/61440/61441/122880/122881; the old 4095–8193
sizes stay as sub-cap short-read rows). All four closures
re-verified green: sweld 1175/0, stream_src 43/0, stream_x86
1296/0, the byte-tie 1299/0; check wall time unchanged despite the
2.3× taller ground fuel towers.

**B3 RECORD (2026-08-01; the machine leg lands — M0–M5 complete,
the rung closes).** The ladder ran exactly as §7.9 laid it out:
M0 80cdeba (the backward-branch-around-an-effect probe; its
findings — fuel = depth, state below the read buffer, the
structural ≥1-byte argument — carried M3/M4 as predicted), M1
7423473 (the absorber + its readback), M2 2e0642e (the finalizer +
readback), M3 9125b6c (the thin main; differential 25/25 first
run), M4 2a261f0 (THE WELD — the SswOb package invariant through
the loop induction; §7.3's D8 disjunction CLOSED: EOF ⇒ wrote
`bytes_hex (sha256 (scat chunks))` ∨ read-error fail-stop ∨
oracle-dry no-verdict), and M5 in five commits: a2a89df + ae1de80 +
a1e85d7 the base-65536 relocation (window equality FORCED at every
tier by shared bridge guards — the window became declared source;
laws in the arc memory item 3o), 34fc196 emission + the CROSS-FILE
BYTE-TIE (ssx_main_eof/err/dry restate all three D8 legs over the
emitted module, teeth measured), b8ddc7a the stale one-shot emitter
repaired, 9a5a7f9 the capless silicon leg joining the corpus (61
rows, both bins vs coreutils; the streaming rows PIPE-FED — X86.md
§51's file-redirect discipline retires, short reads are under
test). The name ruling (2026-08-01): `(bin sha256sum)` = the
streaming program; the capped demonstrator = `(bin
sha256sum_oneshot)`. X86.md §52 is the x86-side record.

**The verdict line (§7.5 form — a bare MET is never an artifact
verdict):**

    sha256sum: MET (artifact: unconditional)

The artifact claims are UNCONDITIONAL in §7.5's sense — no except
grammar, no cap condition: the disjunction covers every oracle
behavior at every fuel. They are stated over the EMITTED module
(the byte-tie), and the emitted binary is corpus-gated in CI on
real silicon against coreutils. The one-shot's §50-era verdict (its
cap leg a declared controlled failure) stands unchanged under its
new name.

**B4 RECORD (2026-08-01; benchmark + attribution land, the rung
closes).** Method: median-of-5, core-pinned, warm page cache,
1 GiB input, quiet box (shard run spread 10.640–10.924 s; every
other contender under 2%). This box's coreutils links libcrypto
and dispatches to SHA-NI, so the honest scalar-class bar is
coreutils under `OPENSSL_ia32cap=":~0x20000000"` (clears leaf-7
EBX bit 29 — SHA-NI off, libcrypto's expert AVX/SSSE3 scalar
path).

    shard sha256sum (cap 61440)    10.649 s  (~96 MiB/s)
    coreutils, SHA-NI masked        1.752 s  → gap 6.1x
    coreutils as shipped (SHA-NI)   0.598 s  → gap 17.8x
    openssl dgst -sha256            0.631 s  → gap 16.9x

ATTRIBUTION (the gate's real requirement, measured with perf on
64 MiB, cap-4096 build — the retune changes syscall count only):
ours retires 195 instructions/byte at IPC 4.40 — core peak — vs
28 instr/byte at IPC 4.05 for masked coreutils. Cross-check:
195/4.40 cycles/byte at the box's 4.5 GHz predicts 9.85 ns/byte;
the 1 GiB run measured 9.92. So the gap is ~100% backend
instruction VOLUME (7.0× the instructions — the lowering's spill
idiom, every IMP temp round-tripping through memory) and ~0%
stall: nothing about the proof-facing IR's shape slows the
machine down; the emitted program simply does seven times the
work, fast. The syscall axis was priced separately: read floors
measured 0.256 s/GiB at 4096-byte reads vs 0.127 at 64 KiB; the
cap retune's wall effect was correspondingly small (0.686 →
0.663 s best-of-3 on 64 MiB) because compute dominates at 6× —
consistent with the volume attribution.

**The §5 falsification gate reads NO.** "Dramatically slower for
reasons inherent to the proof-facing IR" is refuted by the IPC
evidence: this is the classic no-register-allocation tax of an
immature backend, not an IR property.

**The parity fork (RATIFIED (b), user, 2026-08-01).** The
C-class identity demands parity-class, and 6.1× scalar is not
it. Plain question: where does parity come from?

- *(a) A register-allocation rung on the x86 backend.* Attacks
  the 6.1× directly; every future artifact inherits the win.
  Costs: the largest certified-lowering surface proposed since
  the imp dialect — x86gen is FROZEN (IMP.md ruling), so this
  reopens a ratified freeze mid-arc and builds a new proof
  surface before B5.
- *(b) Parity via B5/B6 — the hand-pinned SHA-NI leg + proven
  dispatch.* The comparison target's own fast path on this
  silicon IS SHA-NI; nobody ships scalar where SHA-NI exists.
  The parity target becomes the B6 dispatch artifact vs
  coreutils-as-shipped; the 6.1× stays on record as a priced,
  attributed backend debt. Costs: every scalar-tier consumer
  keeps paying it until a backend arc, priced then on B5's
  hand-pinning evidence.

RATIFIED: (b) — parity comes from B5/B6; the register-allocation
arc is real future work, REJECTED-for-now because it reopens the
x86gen freeze mid-arc and should be priced by B5's evidence of
what expert code needs from the backend, not opened on one
number ("we can get to register allocations properly later" —
user, 2026-08-01). The 6.1× scalar gap stands on record as a
priced, attributed backend debt.
