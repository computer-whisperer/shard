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
  **SCOPE RATIFIED (user, 2026-08-02) — the FULL redirection.**
  Scoping found the out file carries THREE replay chains, not one:
  shblock (13 boundaries, the priced ~40.6k) plus shpad and shfpad
  (single-boundary, ~2.7k each), whose walk claims weld/sweld cite
  — the B1 record never mentioned the pads. Ruled mechanism:
  sha256.xchain.shard gains conversion-dialect twins for the two
  pad chains (seam + walk each; the pads share the 12-slot window,
  ilv_inv12 applies); weld/sweld/xconv/xpatch migrate citations to
  the twins; then tools/impgen's mixed x86 tier drops the boundary
  ladder emission UNCONDITIONALLY — no marker, no mode, the
  deprecated form stops being emitted for every pin present and
  future. REJECTED-because: the shblock-only alternative (input
  marker, zero new proofs) kept two replay chains committed and
  taught the generator a per-pin convention — the landing sentence
  above stayed false under it. Staging around the thread-B anneal
  window (impgen in its scope): the pad twins are a pure xchain
  addition and land FIRST; the generator flip + regeneration +
  citation swaps land in one slice at the window's closing
  announcement.
  **SLICE 1 LANDED (2026-08-02).** The inversion law grew into a
  family — ilv_inv5/ilv_inv6 join ilinv_probe (same template as
  ilv_inv12 at the pad arities; probe closure 101/0) — and the
  four twins landed in xchain: xch_shpad_b1/xch_shfpad_b1 (seam
  statements byte-identical to the cmp_x originals, guard-tree
  proofs verbatim — the b13 precedent held: the pad seams carry
  no exposure, the guard tree is the semantic floor) and
  xch_shpad/xch_shfpad (walk statements over local program
  copies, the xch_gx device; the 5/6-deep Cons exposures replaced
  by ONE ilv_invN citation minting the reads, closing on the seam
  twin by the generated leaf's own citation form). xchain closure
  951/0 on the FIRST check — the conversion-dialect kit
  transferred to new fns with zero iteration. What remains at the
  window close: the flip, four regens, five citation swaps, the
  D-number, THE RECORD.

  **B4b RECORD (2026-08-02; slice 2 lands — the rung CLOSES).**
  Sequenced behind the thread-B anneal merge (6d830de brings
  e22f179's gated impgen), so the flip regenerated with the
  repaired tool. The flip: mxx_lems threads one structural Bool —
  ec, "emit the boundary-chain claims" — false exactly when the
  pin has boundaries (nlg ≥ 2). Suppressed: the cmp_x_ ladder
  claims and their b=0 walks. Still emitted: the sqs_x_ seg
  sub-lemmas (the terms the conversion seams cite), rest/wid/
  worker vocabulary, ties, and the single walk of any 0-boundary
  pin (shhex keeps imp_x_shhex — a walk without a chain is a
  bridge, not the deprecated form). Scope proof: regenerated
  under the flipped tool, impgen_wasm_out and the scalar/loop x86
  fixtures are BYTE-IDENTICAL; only the three mixed-content x86
  fixtures and the sha out change. The deletion: impgen_x86_out
  100,016 → 49,173 lines (18 claims: the three chains + walks);
  fixtures mixed 4,440→1,778, if 7,471→2,184, ifl 4,350→2,039
  (21 claims, zero external citations — verified by repo scan);
  net −61k committed generated lines. Five citation swaps, all
  statement-neutral (the xchain local program copies verified
  byte-identical to the generated defs before swapping): weld →
  xch_shblock + xch_shpad, sweld → xch_shfpad, xconv/xpatch →
  xch_b13. Gates: 11-closure check battery ALL GREEN first run
  (out 898/0, xchain 933/0, xconv 938/0, xpatch 936/0, weld
  1103/0, sweld 1192/0, stream articles 1313/0 + 1316/0 — the
  byte-tie holds over the shrunk out file, fixtures 442/432/423);
  the FULL build driver: 80 products, all gates green. THE
  D-NUMBER (idle-core stash pair, SHARD_STATS, whole closures):
  impgen_x86_out 1,130.2M → 761.5M calls (−32.6%), live peak
  430.3 → 241.0 MB (−44.0%), maxrss 1.72 → 1.16 GB (−32.6%);
  xchain −28.7% / −36.2% / −31.4%; weld and sweld — which now
  IMPORT xchain and check the conversion twins instead — still
  net −5.8%/−5.5% calls and −13.4%/−13.5% live peak. The B1
  pricing (−19.3% calls) is EXCEEDED on the direct closures. The
  landing sentence is now literal: sha256.xchain.shard is the
  closure's only block-chain dialect, and the generator can no
  longer emit the replay form anywhere.

- **B5 — the expert leg: SHA-NI/SIMD hand-pinned variant vs
  OpenSSL.** This is the FIRST real non-canonical consumer — the
  named trigger for clause 1 of the validator's clause architecture
  (CERT.md §4): decide build-clause-1 vs hand-prove-the-leg when
  the artifact exists. Rung-design forks: SHA-NI vs AVX2 tier
  first; the dispatch-visible variant contract.
  **CLOSED 2026-08-09** — E1–E3 landed (§8.4: THE B5 NUMBER =
  1.13× coreutils-as-shipped; verdict `sha256sum_shani: MET
  (artifact: unconditional)`); E4 DISSOLVED by the clause-1
  re-founding (CERT.md §4: the adjudication was mis-posed — the
  hand leg was never awaiting admission).

- **B6 — proven feature dispatch.** CPUID/feature detection selects
  only certified variants; the dispatch proof composes the B4/B5
  artifacts under one bin contract. **OPEN 2026-08-10 — §9 = the
  ratified rung design** (R1 chip answer rides the oracle, R2 the
  dispatch bin takes the plain name, R3 linking meta-theorems, R4
  the two-step CPUID probe).

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
*[B6 R2 rename, 2026-08-10: the streaming scalar bin is now `(bin
sha256sum_scalar)`; the plain name belongs to the B6 dispatching
artifact. This verdict stands unchanged under the new name.]*

**The verdict line (§7.5 form — a bare MET is never an artifact
verdict):**

    sha256sum_scalar: MET (artifact: unconditional)
    [stood as `sha256sum: MET (artifact: unconditional)` from
     2026-08-01 until the B6 R2 rename, 2026-08-10]

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
    [the shard row's bin = today's sha256sum_scalar — B6 R2
     rename, 2026-08-10]
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

## 8. B5 — the expert-leg design note (RATIFIED 2026-08-02)

**RATIFICATION (2026-08-02, user ruling, all four forks to their
leans):** Fork A = SHA-NI first (AVX2/YMM = named door); Fork B =
four dword lanes (this WRITES X86.md §6's deferred lane ruling —
recorded there); Fork C = the parallel extended tier (widening
Regs/XOut REJECTED on the survey's numbers — the corpse stands in
8.5); Fork D = per-variant verdicts (cross-module equivalence
REJECTED — welds the variants). The §8 body is law; the E-ladder
executes in order, E1 next.

**Mission.** The hand-pinned SHA-NI variant of the streaming bin,
benchmarked vs OpenSSL/coreutils-as-shipped — the FIRST real
non-canonical consumer (CERT.md §4's named clause-1 trigger) and
the artifact half of the ratified parity fork (b): B6's dispatch
composes this variant with the scalar bin to chase the 17.8x
as-shipped bar (0.598 s/GiB on the quiet box). The rung also
answers §5's second downgrade gate in the affirmative or negative:
can an expert SHA-NI schedule be expressed WITHOUT target
semantics leaking into the high-level requirement?

### 8.1 Ground (survey 2026-08-02)

- models/x86 has ZERO vector state: Regs = 15 Int GPRs (no rsp),
  XInstr = 19 constructors, xeval_instr = one exhaustive match.
  X86.md §6 reserved the growth point ("an XMM/YMM file
  absent-or-empty until used") and pinned the hazard: the op
  vocabulary is FIVE tables wide (semantics / encoder / cert
  readback / render / emitter conditions) — extend in step.
- Widening Regs is a REPO-WIDE ARITY BREAK: 1033 MkRegs sites,
  nearly all full 15-argument positional patterns in claim goals,
  plus generated certs (vx86_acc_probe, 246k lines) and hard-coded
  MkRegs string templates inside tools/x86gen. Measured, closed.
- The world layer (weval_instr, xeff_i) carries (other …)
  fallbacks — new PURE species need zero world-layer edits at the
  scalar tier; models/imp/to_x86.shard's exhaustive ix_dwl never
  sees vector arms (imp never emits them) — no edit, confirmed by
  E1's structural scan. New CONSTRUCTORS are not the 965e349
  arity hazard (existing arms untouched, no regen owed).
- An instruction-grain silicon differential harness already
  exists: models/x86/diff/ (spec-side vectors + C replayer).
- The box has SHA-NI (Ryzen 9 5900X; the 0.598 s coreutils number
  IS this path). Read-syscall floor at cap 61440 ≈ 0.13 s/GiB;
  compression dominates.

### 8.2 The vector tier (the architecture)

XInstr GROWS the vector species (additive arms; scalar-tier
xeval_instr arms for them = XTrap, the loud-refusal discipline —
old programs' semantics untouched, no signature changes anywhere).
A NEW FILE models/x86/vector.shard holds the tier:

- **State**: (record Xmm (x0..x3 Int)) — four dword lanes, each in
  [0, 2^32) — and (record Xmms (xmm0..xmm15 Xmm)), the
  architectural sixteen. See fork B for the lane ruling.
- **Evaluator**: a thin twin SCC xveval_instr/call/loop/seq
  threading (Regs, Xmms, Mem) with new outcome types XVOut/WVOut;
  scalar LEAF arms delegate to xeval_instr and carry Xmms
  unchanged; compound forms recurse in-tier; a world twin mirrors
  world.shard's effect arms. Existing XOut/WOut/xrun_*/weval_*
  signatures are UNTOUCHED — every landed cert re-runs byte-
  identical (the standing additivity proof).
- **THE EMBEDDING THEOREM** (the reuse keystone, proven once per
  tier): a body containing no vector arms steps identically under
  xveval with Xmms unchanged. Scalar per-instruction walks lift by
  ONE citation; the M-skeleton's composition equations
  RE-INSTANTIATE over the B5 module at their xcall seams (the
  _run species' free restfs tail) with the vector fold's walk
  cited where the scalar fold's was — re-compose, not re-prove.
- **Species** (≈10, at the generality the schedule exercises —
  XRorI32 precedent; wider forms fence loudly): sha256rnds2,
  sha256msg1, sha256msg2 + support: 16-byte unaligned load/store
  (mem_read/mem_set at dword assembly), dword shuffle (pshufd
  imm), the byte-swap shuffle (per-lane bswap32 — general
  byte-grain pshufb is NOT modeled), dword add (paddd), align/
  concat (palignr imm), qword unpacks (punpckl/hqdq).
- **Entry state**: silicon guarantees nothing about XMM at _start
  — the bin's top statement universally quantifies initial Xmms
  (the M3 junk-regs precedent); the schedule writes before it
  reads.
- **Trust locus** (platform-externs law): the tier's semantics are
  differential-gated at instruction grain — random Regs/Xmms/Mem
  states, each species run on silicon vs the model (the
  models/x86/diff harness grows vector rows; vector rows compare
  against the EXTENDED tier). The SDM is the drafting reference;
  the differential is the authority.

### 8.3 The schedule and its proof

The canonical SHA-NI block loop keeps state in the PERMUTED
ABEF/CDGH two-register form; sha256rnds2 performs two spec rounds
per issue (wk implicit in xmm0), msg1/msg2 compute the message-
schedule recurrences. The article (std/sha256/sha256.shani.shard)
proves: pack/unpack laws for the permutation, rnds2 = two rounds
of the spec's compression at the permuted layout, msg1/msg2 = the
schedule recurrence, and THE BLOCK WALK: the hand body folded k
blocks = sha_blocks over mem_read — the same conclusion shape the
scalar fold's walk lands, which is what makes the E3 swap a seam
replacement.

### 8.4 Rung ladder

- **E1 — the vector tier lands.** Fable: x86.shard species arms +
  trap fence + vector.shard (state, twin SCC, world twin,
  embedding theorems). Opus-delegated: encode.shard arms, bytetie
  readback keys, diff replayer + vector rows. Gates: instruction
  differential green on silicon; existing cert battery re-runs
  untouched; boundary-3 announcement in the thread log BEFORE
  landing (XInstr arm growth + new files; no existing type gains
  a field).
- **E1 RECORD (LANDED 2026-08-02).** Gates met: silicon differential
  **223/223 agree** (123 scalar unchanged + 100 new vector rows);
  battery re-runs at its recorded numbers exactly (898 / 933 / 938 /
  936 / 1103 / 1192 / 1313 / 1316) = the additivity proof; boundary-3
  announced before landing. New: models/x86/vector.shard (638),
  models/x86/vworld.shard (644), models/x86/probes/xvector_probe.shard
  (676, corpus-registered) + the Opus half (encode.shard, bytetie,
  the diff replayer's XMM trampoline). THREE CORRECTIONS to §8.2's
  draft, all on measured evidence — the full record is X86.md §6:
  (i) **the species enter through ONE wrapper ctor `(XVec XVInstr)`,
  not twelve flat arms** — flat growth breaks every exhaustive
  `case-on … XInstr` and takes the imp pricing cert's quadratic
  dispatch table 19×19 → 31×31, and the ratified AVX2 door would levy
  it again; (ii) **`xvec_i` answers True on XCall** (world.shard's
  xfn_eff answers False and leans on an image-level walk, which leaves
  a latent trap the B5 skeleton would walk straight into) so
  transitivity is dynamic — every nested seam re-dispatches;
  (iii) **`sha256rnds2`'s operands were exchanged in the drafted
  semantics** and the differential caught it on the first run (9 rows,
  all that instruction) — the SDM puts ABEF in the SOURCE and CDGH in
  the DEST. §8.2's claim that no consumer needed editing was WRONG in
  one place: the pricing cert took +1 case at each of 11 case-on sites
  and thread B's x86_transition_window took a 3-line mechanical pad.
- **E2 — the hand-pinned block article** (probe-first tradition).
  Gates: article green; block-grain silicon differential (the
  block fn on CPU vs spec vectors, gold + random rows, teeth via
  perturbation).
- **E2a RECORD (LANDED 2026-08-02) — the value tier of the article.**
  std/sha256/sha256.shani.shard (742/0, corpus-registered), every
  claim first-check green after probe-first development (e2_probe
  .shard, untracked root): pack/unpack shuffle laws SYMBOLIC
  (pblendw-free — palignr 8 + punpcklqdq route); **shn_rnds2 = THE
  TWO-ROUNDS LAW** (sha256rnds2 at the packed layout = two spec
  rounds, component-nonneg premises only) + shn_slide (new CDGH =
  old ABEF verbatim, pure compute); **shn_sched_group = THE
  SCHEDULE LAW** (one msg1/palignr/paddd/msg2 step = the next four
  sched_ext words, in-instruction chaining matching the spec's
  recursion); shn_block_b = the reference dataflow, ground-pinned
  to sha_block on three vectors incl. chained feedback. Proof
  architecture: tree-identity alignment family (one compute each)
  reduces the whole article to ONE semantic seam — the wk mask —
  crossed by shn_madd32 (unconditional mod collapse) + shn_absorb4;
  a nonneg range ladder (pow2 literal rungs 1..30, ground-shift
  rotate bounds, helper bounds) feeds the mask/mod bridge.
  REMAINING at E2a's landing (see the E2b record below): the hand
  body + THE BLOCK WALK at machine grain (E2b, ties to
  shn_block_b), the block-grain silicon differential w/
  perturbation teeth (E2c, Opus-delegated).
- **E2b RECORD (VALUE HALF LANDED 2026-08-02) — THE WORD-GRAIN
  BLOCK THEOREM.** shn_blockw (sha256.shani.shard, article now
  782/0 in ~3 s): `shn_block st g0..g3 = h8_add st (sha_rounds st
  (sha_k) (shn_w64 g0..g3))` under state-los + input-group xlos
  premises — one SHA-NI block dataflow IS the spec's 64-round walk
  over the schedule-word projections (shn_w64 = lane projections of
  the named shn_g4..g15 chain, everything folded); the fact E3's
  fold swap cites. Probe-first (e2b_probe.shard, untracked root,
  superseded by the article). THE PERFORMANCE LESSON: the 16-step
  unrolled rewrite-the-qround-law walk was >14 min INTRACTABLE
  (each step restates the growing tower — O(n²) explicit nests; an
  abstract-16-var refactor measured WORSE, free vars keep stuck
  projections alive). The fix = the scalar ish_srounds_los pattern
  one level up: **shn_qwalk** (qround folded over k/msg group lists
  in lockstep) + **shn_qwalk_thm** by ONE induction (step =
  shn_qround_law_v — the qround law at VARIABLE k group, its case
  split + blnth triples proven once; nonneg invariants ride
  slo/gso list folds with hd/tl inversion lemmas) — >14 min → 3 s.
  The article's 16-issue nest bridges to the fold by ONE stopped
  compute; the ground k side collapses to (sha_k) by pure compute.
  Checker gotchas on record in the arc memory: compute unfolds a
  variable-applied projection into a stuck match (always stop
  x*_of; collapse ctor-applied projections by targeted
  unfold+reduce haves); rewrite-with rewrites the FIRST occurrence
  only (the feedback's two arms = two citations).
  REMAINING for E2: the hand body + THE BLOCK WALK at xveval
  grain (machine half of E2b); E2c differential (Opus-delegated).
- **E2b MACHINE HALF, SLICE A (2026-08-03) — THE HAND BODY, ground-
  validated.** e2bm_probe.shard (untracked scratch, teach precedent):
  eb_body = 208 XVec/scalar instructions, one SHA-NI block start to
  finish — the state pack shuffles (shnp_pack_abef/cdgh's trees), 16
  qround issues each `XMM0 <- W_j; paddd XMM0,K; rnds2 CDGH,ABEF;
  pshufd XMM0,0x0E; rnds2 ABEF,CDGH`, the twelve just-in-time schedule
  steps (shn_sstep's tree), the paddd feedback and the unpack + store.
  REGISTER ALLOCATION FROZEN (the probe header is the record): XMM0 wk,
  XMM1 ABEF / XMM2 CDGH (roles never swap — the slide law), XMM3..XMM6
  the rotating message window (W_j in XMM(3 + j mod 4)), XMM7 schedule
  temp, XMM8 the byte-flip mask resident, XMM9/XMM10 the entry state
  saved for the feedback, XMM11 the K load + shuffle temp; RDI = src,
  R10 home scratch, R11 the K pointer.
  GROUND VALIDATION = e2bm_run.shard, a RUN-MODE differential (check
  mode cannot pin a machine run: std/mem's Mem is opaque, the M1 law),
  machine leg vs shn_block_b: **8/8 rows PASS**, first semantic try —
  abc, empty-pad, ascending bytes, distinct bytes at a computed state,
  all-0xFF, all-0xFF at the all-max state (wrap stress), the NIST
  two-block vector CHAINED through the memory state home, and a framing
  row (mask, block bytes and an unwritten cell untouched). Teeth: K base
  +4 and a swapped rnds2 operand pair each collapse it to 1/8; a
  palignr immediate 4 -> 8 leaves 5/8 — the survivors are the
  degenerate blocks, which is why the distinctive-byte rows exist.
  ONE correction, in the harness not the body: **xveval_seq spends fuel
  PER INSTRUCTION**, so a 208-instruction body needs ~210 (the harness
  carries 512), and a differential's failure fallback must poison its
  compare window — reading back the entry state made an under-fueled
  run look like a body that stored its input.
  MEASURED FOR SLICE B: expanding shn_block symbolically with the
  primitives stopped CORE-DUMPS at ~168 s. The schedule chain's term
  grows tetranacci (|g_n| ~ 1.93^n) with no sharing, so the walk cannot
  be one compute — every schedule group and every round state must be
  folded behind the article's own functions, never expanded. So the body
  is now written AS STANZA FUNCTIONS — a prologue, sixteen qrounds and
  an epilogue, each taking the rest of the instruction stream and
  consing its own instructions on front (207 instructions, K addresses
  ground per stanza so no pointer state is threaded; the differential
  re-ran 8/8 on the restructure).
- **E2b MACHINE HALF, SLICE B part 1 (2026-08-03) — THE STANZA LAWS.**
  Sixteen walk laws, one per qround, ALL first-check green (798/0, 3 s):
  each walks its stanza's twelve (or seven, for the schedule-free last
  four) instructions at a FREE fuel tail and a FREE instruction tail, so
  the laws chain by rewriting and the walk is never run past a stanza's
  end. Every value a stanza produces stays FOLDED behind the article's
  own functions — the round state as shn_qround's two projections, the
  schedule step as shn_sstep — which is exactly what keeps the
  tetranacci growth out of the terms. TWO MECHANICS, both load-bearing:
  the fuel tower is one deeper than the instruction count (xveval_instr
  destructures one more level than xveval_seq spends, so a
  twelve-instruction stanza is stated S^13 f -> S f); and BOTH records
  must be spelled OPEN (MkRegs / MkXmms over variables) — over an
  abstract record every read is a stuck projection and the address gates
  never resolve, the same law the value half learned on Xmm.
  (The walk's remaining pieces landed same day — next bullet.)
- **E2b MACHINE HALF, SLICE B part 2 (2026-08-03) — THE BLOCK WALK
  (eb_walk, summit first-check; probe 806/0 in ~2.5 s; differential
  re-run 8/8).** The hand body's 207 instructions, walked at xveval
  grain from ANY entry register/XMM file, compute EXACTLY the article's
  shn_block on the loaded state and the four byte-flipped message
  groups, concluded at the eb_hmem store shape with the full final
  file spelled (the clobber map). Premises = THE ONE SEAM: the mask
  home read, the two state-home reads as MkXmm literals, the two src
  window bounds, the sixteen K-group reads. Pieces: the epilogue law
  (feedback paddds folded, the unpack shuffles COMPUTED THROUGH to
  MkXmm-of-projections — vsel cracks a shuffle over any argument, so
  the store trees meet shn_unpack's computed shape by syntax); the
  prologue law (state reads as FREE Xmm values, pack chains kept
  FOLDED; three mod_unique q=0 wrap collapses + eight farkas gate
  discharges in an alternating stall/rewrite walk); a ShnRs eta law
  by case-on (the plan's "Xmm eta" — the actual seam is MkShnRs of
  the previous qround's two projections, collapsed 15×); eb_s16/
  eb_f1/eb_f2 naming the article's nest + feedback registers (def
  lemmas cited rl fold the walked towers into them — the summit
  stays readable); eb_block_split (shn_block = shn_unpack of the two
  names). The composition is ~70 pure rewrites and two computes:
  pack laws fire AT COMPOSITION TIME on the literal-instantiated
  folded chains, each stanza rewrite is chased by its g_def rl fold
  + K-premise + eta. TWO corrections to part 1's plan, both
  load-bearing: (1) COMPUTE CANNOT RE-DESCEND INTO A STUCK CONTEXT —
  a premise rewrite that plants a redex inside the frozen tail
  (match on the free instruction tail) leaves it unreduced forever;
  everything downstream of the last stall must stay a folded
  APPLICATION and be adjusted by REWRITING only. Corollary: content
  BEFORE the last stall re-normalizes when the walk resumes (the
  mask register unfolds to its literal), so the boundary sits at the
  last stall, not at the walk's start. (2) The byte tie to
  shn_block_b is NOT the walk's business: eb_walk concludes over
  (vbswap (vload mem src+16i)) groups; the group->bytes tie is the
  differential's ground rows today and E3's seam symbolically.
  (Transplanted to the article — next bullet.)
- **E2b TRANSPLANT (2026-08-08) — the machine tranche lands in the
  article.** The hand body (shn_pro/shn_q0..15/shn_epi/shn_body),
  shn_m, shn_hmem, and every law through THE BLOCK WALK summit
  (now shn_walk) moved from the probe into
  std/sha256/sha256.shani.shard as the "E2b MACHINE TIER" tranche —
  a PURE PREFIX SWAP (s/eb_/shn_/, s/ebx_/shn_/; zero collisions),
  so the probe's green transfers verbatim. The article gained the
  std/mem and models/x86/x86.shard imports (Mem, XInstr/Regs/fuel;
  vector.shard alone was the value tier's surface). Gates: article
  806/0 in ~2.4 s; shardfmt canonical (the tranche needed one
  reformat pass — the probe was never fmt'd); the C3 canon stage-2
  sweep's per-file form (focus-mode load + grep CANON) at ZERO
  advisories. e2bm_run.shard is now SELF-CONTAINED against the
  article (harness fns inlined, pointed at shn_m/shn_hmem; probe
  import dropped) and re-ran 8/8 — the differential exercises the
  article's copy. e2bm_probe.shard = FROZEN HISTORY (header note;
  nothing imports it). (E2c landed same day — next bullet.)
- **E2c LANDED (2026-08-08) — THE BLOCK-GRAIN SILICON DIFFERENTIAL
  (Opus-delegated; 57/57 positive rows first semantic run, five teeth
  all biting, E1 leg unregressed 223/0).** The article's shn_body
  emitted as real machine code and EXECUTED (models/x86/diff/
  shani_diff_run.shard = the plan emitter, shani_diff.sh = the
  end-to-end leg, x86_diff.c = the same E1 replayer +67 lines):
  stored state compared against the VALUE tier's shn_block_b, the
  whole XMM file against the vector-tier walk — model==silicon at
  block grain, the loop E2a/E2b left open. Rows: 8 gold + 48 random
  (state x block x entry XMM file, deterministic LCG — plan
  byte-identical run to run) + the NIST two-block CHAINED row
  (feedback through the memory home; the chained module composes the
  article's own stanzas twice, add rdi,16 between — the prologue
  advances RDI by 48). Memory is compared over [0,2000) of the
  pre-zeroed page, so framing (mask home, block bytes, unwritten
  cells) is implicit in every row. UN-BLINDING IS NOW A GATE, not a
  measurement: new plan row kind XVNCASE (XVCASE inverted — PASS =
  silicon disagrees) carries five perturbed twins re-scored EVERY
  run, replayer tallies per tooth, every tooth must bite >= 1 row:
  K-base+4 8/8, qround-0 rnds2 swap 8/8, palignr imm 4->8 4/8 (the
  four survivors are exactly slice A's degenerate uniform-lane
  blocks), byte-grain opcode CB->CD 8/8, byte-grain REX.R 8/8 — the
  SDM operand-order convention (E1's rnds2 correction) confirmed by
  silicon at block grain. THE RELOCATION LAW (E3-relevant):
  vm.mmap_min_addr=65536 makes the body's ground homes unmappable, so
  the plan rewrites every XMovRI immediate by +DATA_BASE and pins the
  rewrite with an enc-identity first line (enc(sr_reloc 0 body) ==
  enc(body) byte-for-byte; FAIL path tested) — E3's image places the
  homes at real addresses by regeneration, and the mechanical-reloc
  seam is now exercised and known-good. Harness self-tests (poisoned
  expectations, reverted): both legs proven load-bearing. CI: corpus
  leg after the E1 leg (run_corpus.sh), CPUID gate -> loud SKIP +
  exit 0 (#289's posture), SKIPs COLLAPSED to one line per module so
  the corpus FAIL/SKIP projection can't be flooded blind, verdict
  line unconditional. Gates: plan file 806/0, fmt + canon clean.
  Checker gotchas from the leg: ediv-descent and counting-up measures
  from the scratch runner's helpers are NOT corpus-clean (flat dec4 +
  countdown recursion instead); canon C8(a) — a nullary-ctor match
  arm must not use the scrutinee variable in its body. E2 (a+b+c) IS
  NOW CLOSED; the rung's remaining ladder = E3 (variant bin) then E4.
- **E3 — the variant bin** (lean name: sha256sum_shani; naming of
  the eventual dispatch bin = B6's fork). Skeleton re-composition
  + fold swap, emitted ELF (enc_winelf), byte-tie, capless silicon
  pipe differential vs coreutils AND openssl, corpus registration
  + run_corpus rows. Gates: THE B5 NUMBER on record (§5's gate);
  the §7.5-form verdict for the variant (fork D's contract); the
  expressibility gate answers here.
- **E3 SLICE LADDER (ratified in-session 2026-08-08, leans adopted):**
  A = the rebase probe (the machine tranche regenerated at the
  streaming layout); B = the fold leaf (fused call-free loop fn +
  THE FOLD WALK, incl. the symbolic group->bytes tie E2b deferred
  here); C = the skeleton twin at wveval grain (sha256sum_shani_x86,
  embedding lifts + the fold seam via wvcall_bridge); D = ELF +
  byte-tie + pipe differential vs coreutils AND openssl; E = THE B5
  NUMBER (B4's method) + the fork-D verdict + records. Micro-forks
  ruled to their leans: fn 0 dead-kept (index stability); fn 1 = the
  fused SHA-NI fold leaf (vworld's ruled shape); mask home @130496
  (the dead W base — SHA-NI keeps the schedule in registers), K/H at
  the skeleton's own homes 130752/131008 (byte-identical content
  layout, so the absorber/finalizer readback equations hold
  unchanged); init wrapper fn 13 extended to write the mask; the
  variant bin = (bin sha256sum_shani ...) over the same spec entry
  sst_main (fork D's per-variant contract).
- **E3 SLICE A LANDED (2026-08-08) — THE REBASE PROBE.** The
  article's whole machine tranche (shn_pro..shn_walk, 2739 lines)
  regenerated at THE STREAMING LAYOUT under shnw_ names
  (e3a_probe.shard, untracked root): **830/0 FIRST CHECK in ~2.6 s
  with ZERO proof-step changes** — three ordered token maps only
  (48 names shn_X -> shnw_X word-bounded; the address map 256->
  130496, 512+16i -> 130752+16i, 768/784/800 -> 131008/131024/
  131040, 65536->131072; the base fixes (le 0 ..) -> (le 65536 ..)
  on the five src lower-bound sites + the module former's base).
  WHY it could be pure regeneration: every farkas gate discharge in
  the tranche is an index-and-multiplier row citation — (rows (goal
  1) (N 1)), no ground constants in any cert row — so the discharges
  are invariant under the address map; the mod_unique wrap collapses
  keep q=0 (addresses stay tiny vs 2^64). Ground validation
  e3a_run.shard (e2bm_run mapped the same way; fuel 512 and mod-256
  byte generators deliberately NOT mapped): **8/8 first run**;
  K-home tooth (group 0 displaced +4) collapses to 1/8 with only
  framing surviving — e2bm's exact tooth signature — and reverts to
  8/8. MEASURED for slice C: vworld's purity dispatch spends budget
  PER LIST ELEMENT (xwv_l), so xwv_budget=96 exhausts on the ~220-
  instruction leaf and answers True = MIRROR — the designed delegate
  route (wvcall_bridge) needs the budget raised past the leaf's
  length; one-literal vworld.shard edit, gated by the E1 probe
  re-check + a thread-log announcement, deferred to slice C.
- **E3 SLICE B-i LANDED (2026-08-08) — THE FOLD LEAF, ground-
  validated.** e3b_probe.shard (untracked root): snw_wrap = the
  variant fn 1's body (arity 2, sst_xfold_run's exact boundary):
  wrapper [RDX := RSI (count); RSI := 0], XBlock[XLoop[brif (CEqz
  RDX) 1; the rebased hand body STANZA-COMPOSED AT THE LIVE TAIL
  (add RDI 16 / sub RDX 1 / br 0 — shnw_body is the same nesting
  closed at Nil, so the free-tail stanza laws re-compose against
  it)]], exit ladder [R12 := RDI (the advanced cursor); RDI := 0;
  R10 := 130512; SIXTEEN XVLoads zero the WHOLE XMM file from the
  dead-zero cell (no pxor species — a never-written cell in the
  dead W frame is the zero source, ONE vload-zero premise at the
  seam); R10/R11 := 0]. RDX survives the body (its scalar footprint
  is RDI/R10/R11 only) so the loop needs NO spills — the scalar
  fold's spill cells stay dead. Exit contract = every GPR zero
  except R12 = src+64k, XMM file ground-zero (the M3 re-zero
  discipline extended — downstream twin walks carry a ground file).
  Differential e3b_run.shard **7/7 FIRST SEMANTIC RUN**: k=0/1/2/3
  (NIST two-block chained BY THE FOLD; 3-block stride) + the exit
  register row + the XMM-zero row (junk RDX=77/R12=55/XMM=9s
  entries prove the contract bites) + framing (mask, block bytes,
  unwritten cell, dead spill cells, zero-source). Teeth on the
  fresh glue: stride 16→8 collapses to 4/7 (k=0/1 survive, exactly
  as predicted); dropping XMM15's zero-load fails r6 alone (6/7).
  THE TIE ARCHITECTURE (measured during design, B-iii's map):
  sha_block = h8_add + sha_rounds over sha_sched; shn_blockw lands
  the same form over shn_w64; so the tie = shn_w64(bswap'd loads)
  = sha_sched(mem_read 64) — 16 seed lanes (word_be vs
  vbswap-of-LE-assembly, per-lane) + the extension via 12
  shn_sched_group citations + srev_acc plumbing; xlos premises from
  vload/vbswap byte-assembly bounds; the walk-side per-iteration
  state premise needs shn_los-of-sha_block (m32 range). The value
  leg's shape: vfold = shb_fold's byte-list mirror over shn_block_b
  (stake/sdrop chaining) — the differential's spec side. REMAINING:
  B-ii the per-iteration walk (the summit chain at the live tail) +
  the loop walk (shb_xloop's mirror at xveval_loop grain, XVBrk 0
  exit shape, lgv fuel tower) + the run form (sst_xfold_run's twin
  at xvrun_regs); B-iii the tie.
- **E3 SLICE B-ii LANDED (2026-08-08) — THE FOLD WALK, machine leg
  complete.** e3b_probe.shard extended to **877/0 (~2.5 s)**; the
  differential re-runs 7/7. Five tranches, in landing order: (1) the
  loop tail AMENDED — R10/R11 re-zeroed PER ITERATION (two movs; the
  M3 discipline applied inside the loop so the invariant register
  file is uniform across k=0 and k>=1 — without this the loop
  conclusion forks on k). (2) THE MODULE-GENERAL STANZA LAWS:
  e3a_probe's 18 laws regenerated as snw_law_pro/q0..q15/epi over a
  QUANTIFIED function list (MkXModule fs 65536 131072) — the walk
  touches the module only through xmemlo_of/xmemhi_of, which reduce
  on the constructor with fs free, so all 18 proofs landed VERBATIM
  first check; ONE law block now serves the probe module, the loop
  walk, and slice C's variant module. Plus snw_law_gate (the
  not-taken brif as a free-tail law: both sides reduce to the same
  stuck match on the abstract tail). (3) THE SEPARATION KIT (none
  existed at the vector tier): snw_llb (load_le4 strictly below a
  store_le4 — FOLDED-FORM statement, proof peels to bytes via
  load_le_s/z and frames with store_le_get_below, so no refold is
  ever needed), snw_vlb (vload below vstore, 16 crossings; gotcha:
  unfold vstore FOUR times — unfold hits one occurrence per call),
  snw_hb2 (below the H-store pair), snw_fmem_frame (below the whole
  fold, induction on k) — statements stay folded exactly like the
  walk's stop-set discipline, so patterns always match. (4) THE
  FOLD DESCRIPTORS, shx_mem's mirrors: snw_hst (H readback as
  x?_of-projections of the folded vloads), snw_blk (shn_block of
  the readback over the four bswap'd groups), snw_fmem / snw_fxs /
  snw_xstep (+ _z/_s defining claims; snw_xstep = the summit's
  final file transcribed as a function of (xs, m, src)), and
  snw_xs_eta (Xmms eta, case-on). (5) **snw_xloop + snw_xrun**, the
  slice's summits. THE ETA MOVE (the design discovery that made the
  walk small): the ha..hh state premises are GONE — the chain
  instantiates law_pro's sA/sC as MkXmm-ETA forms of the FOLDED H
  reads ((x0_of (vload m 131008)) ...), so the read obligations
  close refl-by-compute, the chain's state terms equal snw_hst's
  spelling BY SYNTAX (the descriptor close is refl), the IH needs
  no giant insts, and NO read-back lemma is needed anywhere —
  ls4_id + the m32 range ladder defer wholly to B-iii. snw_xloop
  (fuel S^230(lg_fuel k c), premises mask/lo/hi + 16 K): Z computes
  after int_of_nat_zero (int_of_nat is OPAQUE — rewrite BEFORE any
  compute or the packed literal blocks the law); S case = gate law,
  the summit's 62-rewrite stanza chain with K premises shifted -2,
  one compute-with-stops through the tail (wrap64 haves mirror
  shb_xloop: hsub/hwk/hw0a/hw0), s16/f1/f2 rl folds, IH (19 side
  goals: 17 below-reads x two snw_vlb strips with LITERAL-arith
  obligations that close by compute, 2 bounds by farkas), close =
  advk_s/fmem_s/fxs_s + block_split + symmetric computes. Fuel law:
  the loop recursion re-enters at the LOOP PEEL's own f2, so the
  base constant is slack-insensitive (any depth >= the body's ~215)
  and lg_fuel's +1/iteration IS the loop peel. snw_xrun
  (S^236(lg_fuel k c); zero-cell read = premise 19): stated over the
  PARTIALLY-QUANTIFIED module (Cons f0 (Cons (MkXFunc 2 (snw_wrap))
  restfs)) — slice C instantiates the variant bin's module directly;
  proof = spine compute, the loop lemma (19 premise-verbatim
  obligations), exit-ladder compute, then the XMM-zero collapse:
  snw_xs_eta rl (explicit inst) -> COMPUTE (the vset match chains
  dispatch on the literal) -> snw_fmem_frame (7-arg, all 16 sites)
  -> premise 19 -> compute, refl. ⚠ TWO fuel/walk laws worth blood:
  (a) spine depth counts ONE PEEL PER CONS BEFORE the instr eval
  (the run spine is 6, not 5 — movrr, movri, block-Cons, block
  peel, loop-Cons, loop peel); (b) rewrite CANNOT reach sites
  inside stuck match ARM BODIES (bound vars) — eta-then-COMPUTE
  first, so the sites surface into constructor position. REMAINING:
  B-iii the group->bytes tie (per the recorded map).
- **E3 SLICE B-iii LANDED (2026-08-08) — THE GROUP->BYTES TIE, slice B
  complete.** e3b_probe.shard extended to **1810/0 (~16 s; the probe
  now imports imp/weld/sweld — whole-closure checks)**; differential
  re-runs 7/7. THE SUMMIT: snw_tie — `snw_hst (snw_fmem m src k) =
  shb_fold (snw_hst m) m src k` with ONE premise (src + 64k <= 130496)
  — the machine fold's H readback IS the spec fold; and snw_blocks_read
  = snw_tie + **ssr_fold_read CITED VERBATIM** (the sweld's
  fold-input bridge composes unchanged — the B-i design gate MET).
  Six tranches: (1) the DISJOINT-OR facts std/bits never had
  (snw_bor0r, snw_borsh at symbolic k via wf-induct on the bits.shard
  skeletons, literal instances bor8/16/24, the pow2 literal ladder
  snw_pw1..24); (2) THE BYTE-SWAP CORE snw_bsw (vbsw32 of a LE
  4-byte assembly = word_be reversed) via div_unique/mod_unique
  uniqueness-as-computation + the disjoint-or collapses, lifted to
  snw_wgb (vbsw32 . load_le4 = wget) and snw_vbs (vbswap . vload =
  MkXmm of wgets); (3) THE SCHEDULE WALK: snw_wni (the sched_ext
  step's m32 spelling IS shn_wnext — premise-free, ssig=vss are
  def-identical), snw_ext4 (free-tail 4-step sched_ext law), snw_gstep
  (the eta'd group law over ABSTRACT Xmms) + 4 per-lane recompression
  laws, then snw_sched16: shn_w64 over the four wget-MkXmm groups =
  sha_sched (mem_read 64 m src), PREMISE-FREE — 12 rounds of
  ext4-then-lane-recompress keep every atom a 7-node x?_of(shn_gN …)
  projection, so the exponential wnext nest NEVER materializes; (4)
  the slo (elementwise-nonneg) pipeline over mem_read/words16/
  sched_ext/srev_acc/sha_sched riding imp's slo kit + ish_srounds_los
  — the m32/los range ladder the map predicted; (5) snw_blockm — THE
  PER-BLOCK TIE: shn_block st (4 bswap'd vloads) = sha_block st
  (mem_read 64 m src) with only the state-los premise — plus snw_l32m
  (every lane of a sha_block result is in [0,2^32)); (6) the probe
  kit: snw_vls (vload∘vstore round trip via ls4_id + 6 ordered
  snw_llb frames), snw_hrt (the H-home round trip through
  shnw_hmem), snw_fold_frame (shb_fold ignores the H stores), and
  the summit induction (IH -> blkw -> hrt -> fold_frame ->
  shb_fold_s). ⚠ Laws worth blood: (a) plain `rewrite … true ()` is
  ALL-SITES but 6-arg rewrite-with is FIRST-SITE — the 7-arg
  `rewrite-with L dir side true (insts) (obls)` spelling is the
  all-sites form (the lane laws NEED it: raw wnext atoms sit both at
  spine slots and nested inside later atoms); (b) occurrence
  selectors are (at K), K 0-BASED, in the 4th slot; (c) `compute`
  PACKS ground Nats (breaks (S …)-spelled patterns — load_le/ls4_id):
  unfold-then-REDUCE instead (ι never packs); (d) equality-shaped
  arith goals need the two-direction cert (list (rows …) (rows …));
  (e) `pow2` is opaque outside bits.shard (pow2_z/pow2_s chains or
  the snw_pw ladder); (f) chain items are right-folded continuations
  — a chain may not END in a non-continuation form and 1-item chains
  are malformed. **THE ENDIANNESS FINDING (slice-C ratification
  fork, measured this session):** the skeleton's K/H windows are
  BIG-ENDIAN (ikh_wstmts writes MSB at the low address; the sweld
  invariant reads via wget = word_be) but the leaf's vload/vstore
  are LE — byte-identical K/H content does NOT satisfy the leaf as
  built. Lean recorded in the arc memory: variant init writes K LE
  (fn 13 is already being amended for the mask) + the WRAPPER
  converts H at entry/exit (vload+vbswap+vstore, outside the loop)
  so B-ii's loop walk and B-iii's tie survive untouched and only
  snw_xrun re-derives; awaiting the user's ruling before amending
  the leaf. REMAINING: slice C per the ladder (+ the endianness
  ruling).
- **E3 SLICE C OPENED (2026-08-08) — endianness RATIFIED + EXECUTED
  at the leaf; THE FOLD SEAM STACK; the budget dial.** The user
  ratified the recorded lean ("proceed with the lean"). Landed, in
  order: (1) THE H CONVERSIONS — snw_wrap amended (e3b_probe,
  untracked): entry [load mask into XMM2 from 130496, load both H
  groups 131008/131024 into XMM0/XMM1, XVBswap32 each, store back,
  re-zero R10/R11] before the loop, the mirror conversion after the
  loop before the XMM-zero ladder; the H window stays BIG-endian at
  every boundary the absorber/finalizer see, the loop body sees LE.
  fn snw_hbsw = the conversion's memory descriptor (load-both-then-
  store-both, so both loads read the entry memory). (2) THE SEQ-GRAIN
  REFACTOR + SEAM STACK, all module-quantified over (Cons f0 (Cons
  (MkXFunc 2 (snw_wrap)) restfs)): snw_xseq = the whole wrapper walk
  at xveval_seq grain, fuel S^248(lg_fuel k c) (18 pre-loop peels +
  the loop's 230; conclusion mem = snw_hbsw(snw_fmem(snw_hbsw m) src
  k)); snw_xrun / snw_xcall = the xvrun_regs / xveval_call
  projections (xcall at S(S^248 ...): the xvec_fn mirror dispatch
  discharged by a ground-scan have); **snw_wcall = THE FOLD SEAM**:
  wveval_call completes as the pure walk with the world untouched and
  the trace empty, via wvcall_bridge (premise 1 = the ground budget
  scan, premise 2 = snw_xcall; f/rs2/xs2/mem2 are dangling pivots —
  explicit insts). Slice C's absorber/finalizer twins cite snw_wcall
  at their XCall-1 sites. Probe 1819/0 (~16 s); differential
  e3b_run 7/7 with the harness flipped to the REAL boundary (H
  written BE via eb_hbe, mwin un-bswaps on read; K stays LE) and a
  convention tooth (LE-written H fails exactly the 4 H rows).
  (3) **xwv_budget 96 -> 512 LANDED 1f17224** (models/x86/vworld
  .shard, the one-literal delegate dial; xwv_l spends one unit per
  list element so 96 exhausted on the ~250-instr leaf and answered
  True = MIRROR, silently defeating wvcall_bridge's premise; ground-
  checked False at 512; gates vworld 644/0 + xvector_probe 683/0 +
  fmt + canon-count unchanged; thread-log announced; xvec_budget and
  weff_budget stay 96 — their conservative-True direction is correct
  for their dispatches). Proof-engine findings: the premise/eta
  rewrite DOES reach if-branch copies (the arm-body fence is about
  BINDER-carrying match arms) — both bswap fences of a conversion
  clear in ONE rewrite round; but content frozen behind a stall does
  NOT re-normalize when later exposed, so the exit conversion needs
  the eta at ALL SITES (7-arg true) — first-site eta leaves the
  frozen lane nests on raw snw_fxs. REMAINING (slice C): the variant
  module former + fn 13 extension (mask bytes + K->LE conversion +
  XMM-file ground-zero for loop-head uniformity) + init ground
  differential; the wveval-grain skeleton twin tower (absorber/
  finalizer/main/invariant/main_eof) instantiating snw_wcall; the
  article transplant; the bin form.
- **E3 SLICE C: VARIANT MODULE + THE INIT-WALK TWIN (2026-08-08,
  e3c_probe 1867/0; differential e3c_run 7/7).** Two chunks. (1) THE
  VARIANT MODULE (e3c_probe/e3c_run, untracked root): vfm_m = the
  skeleton's fifteen slots with fn 1 = (MkXFunc 2 (snw_wrap)) and
  fn 13 = vfm_init (XCall 14, the 16 mask store-immediates at 130496,
  K->LE via 16x vload/XVBswap32/vstore, the XMM-file ground-zero from
  the dead cell 130512, RDI := 0); differential 7/7 FIRST SEMANTIC
  RUN incl. i7 = the init->fold composition row (init, "abc" block,
  fold leaf k=1, un-bswapped H = the chained value leg); teeth:
  K-conversion no-op'd fails exactly i2+i7, a corrupted mask byte
  trips the XVBswap32 fence loud. (2) THE INIT-WALK TWIN — fn 13's
  cert at wveval grain. **Both dispatch budgets -> 1024 (afce8bd)**:
  gb_shinit_x is 577 flat instructions and the call ladder needs both
  scans to COMPLETE on it (xwv_i answers True on XCall by design, so
  fn 13 MIRRORS at world grain and its XCall-14 site delegates
  through the bridges; the skeleton's 1-instr-wrapper trick cannot
  help at this tier). The ladder: vin_x14 (xrun_regs at the 14-slot-
  quantified module = ONE shi_xinit cite, inst d Z) -> vin_xv14
  (xvcall_bridge) -> vin_wv14 (wvcall_bridge) — a landed scalar cert
  welds into a world-vector caller through two citations, §8.2's
  reuse claim EXERCISED. THE WALK vin_wseq (S^586 c, 57 steps):
  descriptors vim_msk (the raw mask nest) + vim_kn n i (the K
  conversion through group i-1; each level references level i-1
  TWICE, so the naive walk term is 2^16 — the walk STOPS vxeq so
  every stanza sticks at its mask fence, and each stick refolds the
  raw store level into the compact descriptor via the ground fold
  equations vim_kf1..16, keeping the term LINEAR); exit needs NO eta
  (a vset chain over a LITERAL MkXmms ctor self-collapses under
  compute — e3b's eta pain was symbolic-base-specific); conclusion =
  WVNorm all-zero regs, all-zero XMM file, vki_mem m, w, Nil, one
  premise (vload m 130512 = MkXmm 0 0 0 0). vin_wcall (S^587 c) = the
  form the main-loop twin cites. CONSUMERS over vki_mem: vim_mthru
  (mask), vim_zthru (zero cell), vim_kread0..15 (vload = shn_kg i:
  descend via kf-rl + snw_vlb/vim_vsta hops, snw_vbs to wget grain,
  content by vim_nth extraction from the landed ikh_k_read — no K
  constants respelled). New kit: vim_vsa/vim_lla/vim_vsta (the
  above-direction separation lemmas; store_le_get_above existed
  unused), vim_gikh (ssw_get_ikh transplant), vim_mload, vim_zfin.
  ENGINE FINDINGS (each cost a cycle): compute DOES normalize args
  under a stopped head (refines the 3h law — the vim_kf alignment
  haves were unnecessary); int_eq-False Farkas certs take G = -1
  (the equation slot's SIGN picks the refutation direction — the
  slot table says so explicitly); int_of_nat in an arith obligation
  must be rewritten away via int_of_nat_succ/zero BEFORE the cert
  (compute PACKS the Nat and strands the opaque app); a vector-over-
  vector separation lemma needs 16 byte-grain hops (4 lanes x 4
  layers — snw_vlb precedent held). REMAINING (slice C): the
  wveval-grain skeleton twin tower (absorber/finalizer/main/
  invariant/main_eof citing snw_wcall at XCall-1 sites and vin_wcall
  at XCall-13; the zero-cell premise rides the invariant); the
  article transplant; the bin form.
- **E3 SLICE C: THE TWIN TOWER, HALF 1 — CALLEE LADDERS + THE
  ABSORBER/FINALIZER TWINS (2026-08-09, e3c_probe 1894/0).** The
  wveval-grain twins of the skeleton's two big callees, both walks
  PASSING FIRST CHECK on the transplant design. (1) THE CALLEE
  LADDERS (vin_x14/xv14/wv14 pattern, one 3-lemma ladder per scalar
  callee): vtr (totr, S^42), vtw (totw, S^50), vcp (copy, S^35 over
  lg_fuel k), vfp (fpad, S^90), vhx (hex, S^80) — each = xrun_regs
  over the variant module citing the landed sweld/weld walk cert
  (sst_xtotr0/sst_xtotw0/ssc_xcopy/ssp_xfpad/shh_xhex), lifted
  through xvcall_bridge then wvcall_bridge with the XMM file riding
  (STREAM.md §8.2's two-bridge reuse claim, now exercised five more
  times). e3c_probe grew imports: loopkit (lg_fuel lives there — use
  is NOT transitive) + sweld. (2) SEE-THROUGH KIT: vsm_gvs/ghmem/
  ghbsw/gfmem (byte reads below the fold's H-home stores — the
  variant's post-fold spill reloads see through snw_hbsw∘snw_fmem∘
  snw_hbsw where the skeleton saw through shx_mem), vsm_vhbsw +
  vsm_vibtw + vsm_vipf (vload-grain: below the H conversions, above
  ibtw_mem, above ipf_mem — the vim_vsa unfold-reassembly template
  with store_le_get_below/ish_fpad_hi hops). (3) THE ABSORBER TWIN
  vsm_abs_wseq/wcall: sst_xabsorb's 30 value haves TRANSPLANTED
  VERBATIM (they are tier-independent — hw0..hcd3), hg8/hg10/hg9
  re-derived over the sandwich, 18 new fold-site premise haves
  (mask/zero/16 K over the spilled+ibtw memory via vsm_vibtw + 3x
  vim_vsa); the mirror walk freezes each XCall behind stopped
  wveval_call and resolves sites by ONE ladder rewrite each; the
  fold site cites snw_wcall (e3b's seam) DIRECTLY — premises
  discharged from the entry-memory premises; fuel = S^283(lg_fuel kb
  (S^49 (lg_fuel krem c))), copy site aligned by sst_push50 +
  sst_lgf_comm + NEW vsm_push249 rl (the push-out/commute/push-in
  kit at the variant's constants). Premises = sst_xabsorb's 4 + the
  18 vector-cell equations at entry m; exit = scalar exit regs +
  GROUND-ZERO XMM file + icp∘(snw_hbsw∘snw_fmem∘snw_hbsw)∘ibtw
  ∘spills memory + trace Nil. (4) THE FINALIZER TWIN vsm_fin_wseq/
  wcall: sst_xfinal's 12 haves transplanted, fold premises over
  ipf_mem via vsm_vipf (the (65664-pn)+pn+72 obligation cancels to a
  goal-only farkas row), hex bounds ground; fuel = S^266(lg_fuel kf
  c) — NO mid-pad: the only lg_fuel-shaped cert is the fold, the
  ground-tower hex cert absorbs the tail as its free c. Both
  packagings = scan-True have + delegate to the walk (vin_wcall
  pattern; ⚠ lg_fuel MUST be in the packaging computes' stop lists
  or the tower opens into match trees and the walk cite misses).
  REMAINING (slice C): the loop tier — ground-fuel piece packagings
  (vsm_absg/fing_wcall via the ssm_lg_pad slab at 284+49+960+63+16 =
  S^1372 and 267+2+16 = S^285), shim mirrors at wveval, iteration
  pieces/walks/entry equations mirroring the article's ssm_ tier
  (article importable for the SsmIt/take/fill/wnat/lg_pad value
  substrate), the vector-premise preservation lemma (one generic
  vload see-through over vsm_step's layers), the weld twins, the
  article transplant, the bin form.
- **E3 SLICE C: THE TWIN TOWER, HALF 2 — THE MACHINE TIER COMPLETE
  (2026-08-09, e3c_probe 2028/0, differential e3c_run 7/7).** The
  variant bin's whole machine-tier contract now stands at xvrun_w
  fn 9: vsm_main_eof/err/dry are the skeleton's three entry
  equations at the world-vector grain, with the digest write's
  payload = lx_read of the finalizer transform over
  (vsm_mem (vki_mem m0) its). The road (every stage first-check or
  one mechanical fix): (1) ground-fuel packagings vsm_absg_wcall
  (S^1372 g) / vsm_fing_wcall (S^285 g) — ssm_lg_pad slab at the
  variant constants (284+49+960+63+16; 267+2+16), bound-derivation
  haves transplanted from the article's ssm_abs_wcall (hshr respelled
  via sst_shr6 — shr_pow2 is a std/bits name the probe doesn't
  import); (2) shim mirrors vsm_read/readeof/write/exit_wcall at
  (S^ 4 g) — syscalls preserve the vector file (wvlift), the XMM
  lanes ride every shim; ⚠ lx_take must NOT be stopped in the
  readeof mirror (the zero-byte take computes); (3) vsm_pre (ssm_pre
  needs a WVOut twin), vsm_vfill/vsm_vicp (vload above lx_fill/
  icp_mem — ssw_fill_above + the get_set hop with an
  int_of_nat_nonneg farkas row), vsm_step/vsm_mem (the variant
  transform at wnat-exhibited ghosts), and vsm_vstep_thru — ONE
  generic preservation lemma (130496<=p, p+16<=131008) carrying all
  18 vector-cell equations across an iteration's seven write layers;
  (4) iteration pieces vsm_iter_data/err/eof at head S^1385 (the
  absorber site lands at S^1372 exactly; ssm_krem/kb/kf_coh are
  tier-independent and REUSED from the imported article; the fill
  discharge composes ssw_msk_len — NOT ssw_slen_msk, which is
  slen-typed); (5) walks vsm_walk_eof/err/dry — the article's
  induction shape with 18 preservation haves per Cons case
  discharging the IH's vector premises at (vsm_step m0 it); (6)
  entry equations at S^1390 citing vin_wcall at the XCall-13 site —
  the ONLY pristine-memory premise is the zero cell at 130512 (ELF
  zero-fill), and the walks' vector premises land via the LANDED
  vim_mthru/vim_zthru/vim_kread0..15 consumers over vki_mem, closing
  the loop the init-walk twin opened. ⚠ ENGINE LAW (cost one cycle):
  the module term must spell the LOOP BODY EXPANDED everywhere — a
  folded (vfm_loop) app inside slot 9 gets opened by any walk
  compute and then no cert's folded-module pattern matches (the
  article's inline-everything convention exists for this reason);
  other slots' folded body apps are fine because every compute stops
  them. REMAINING (slice C): the weld twins (vsw_step/inv/main_eof
  citing snw_blocks_read where the skeleton cited ssr_fold_read —
  seals Done = bytes_hex (sha256 (scat chunks))), the article
  transplant, the bin form.
- **E3 SLICE C: THE WELD TIER — THE SEALED DONE LEG (2026-08-09,
  e3c_probe 2057/0 full acceptance ~18 s, differential 7/7).** The
  variant bin's value half is complete: vsw_main_eof states that from
  the ELF stub's all-zero registers, ANY initial XMM file, a zero
  total cell, and the zero cell at 130512, for EVERY oracle behavior
  at every sufficient fuel the bin reads until EOF, writes
  bytes_hex (sha256 (scat (ssw_chunks its))), and exits 0 — the same
  Done-leg form as the skeleton's ssw_main_eof, over the SHA-NI
  module. Design (the chunk's three ideas): (1) THE PACKAGE PLUG —
  vsw_obs reuses SswOb/ssw_spec/ssw_pk_* with the K slot filled by
  the CONSTANT (sha_k) on the observation side too: the variant's K
  window is LE (the BE observation is FALSE) and nothing at the value
  tier reads it — the vector fold bakes its K constants and the
  machine tier's 18-cell invariant already fed the XMM file — so the
  K premise is DROPPED from every readback twin (the sweld rb pack's
  index 10; old 11 renumbers to 10, flat positional farkas certs lose
  slot 7). (2) THE H OBSERVATION over the sandwich
  snw_hbsw∘snw_fmem∘snw_hbsw (which replaces shx_mem): vsw_wbg =
  vbsw32(wget) = load_le (the CONVERSE of e3b's snw_wgb, same
  snw_bsw skeleton with the byte roles transposed) makes the double
  swap collapse WITHOUT an involution lemma — vsw_vlhb0/1 (vload
  over hbsw = vbswap of the base vload, via snw_vlb + snw_vls with
  snw_wg_lo/vsw_wg_hi lane bounds), then vsw_hobs (wlist of the
  swapped window = shw_hlist (snw_hst mf)) and vsw_hent (snw_hst
  over hbsw = the H8 the wlist premise names, vim_nth peels + case-on
  mk_h8). vsw_hread = hobs + snw_blocks_read + hent + read-below:
  the e3b summit composes into the article's exact rb_h shape;
  vsw_out_read = shw_hex_read over it (the weld's hex seal is
  GENERAL — shx_out_read's shape transplants with the K row deleted).
  (3) TRANSPLANTS carry the rest: vsw_buf_split/rb_tot/rb_p/rb_pend
  (sweld texts; the only proof-hop swaps are ssr_ibtv_shx →
  vsw_ibtv_sw and ssr_read_shx_below → vsw_read_sw_below over the
  vsm_ghbsw/gfmem byte kit), vsw_step/vsw_inv (article texts minus
  the four K-side haves; hr0's positional cert loses one slot),
  vsw_fin_read (ssr_fin_read with the shx_out_read+ssr_fold_read
  pair collapsed to ONE vsw_out_read cite), vsw_main_eof (ssw_main_eof
  with the machine cite → vsm_main_eof + the zero-cell row, hbase
  over vki_mem via vsw_ibtv_vki + vsw_wl_vki — the mask/K-ladder
  see-through at byte and wlist grain, vim_kf16..1 rl + vsm_gvs/
  vsw_wlvs hops, closing at ikh_h_read). Every claim passed first
  check or with one mechanical fix; the two real cycles: reduce does
  NOT open x?_of projections without explicit unfolds (snw_vbs's own
  proof pattern), and dropped premises shift POSITIONAL flat certs
  (slot tables diagnose; keyed rows ride free). New imports:
  models/wasm/loopkit precedent extended — e3c now also imports
  sha256.stream (shc_ closure) + the selective (use (:: std sha256
  sha256)) beside the glob (the B3 shadowing lesson, again).
  REMAINING (slice C): the article transplant (examples/sha256sum
  sibling, imports re-rooted, sfm-tie-style carrier ties, corpus
  registration) and the (bin sha256sum_shani …) form; then slices
  D/E per §8.4.
- **E3 SLICE C LANDED (2026-08-09, e9d543c) — THE PROBE CHAIN AS
  THREE ARTICLES + THE BIN FORM. SLICE C COMPLETE.** The landing
  split (user-ratified: three siblings mirroring the probe chain,
  each surface independently consumable): (1)
  std/sha256/sha256.shaniw.shard = the rebased machine tranche
  (e3a's shnw_ content, 830/0) — B6's dispatch rung can cite the
  streaming-layout block walk without the weld; (2)
  std/sha256/sha256.snweld.shard = the SHA-NI streaming weld (e3b's
  snw_ content: leaf, stanza laws, walks, tie, the wveval fold seam
  snw_wcall; 1819/0) — sweld's analog one register file up; (3)
  examples/sha256sum/sha256sum_shani_x86.shard = the variant bin's
  machine+weld article (e3c's vfm_/vin_/vim_/vsm_/vsw_ content;
  2064/0 = the probe's 2057 + SEVEN vfm_*_tie carrier ties pinning
  the body copies to the skeleton article's sfm_/ssm_ originals —
  cross-file drift fails AT THE TIE, sfm-tie precedent). Sibling
  imports by basename, cross-tree by ../../ (sweld's style); use
  paths are path-derived ((:: std sha256 sha256.shaniw) etc.).
  (bin sha256sum_shani (entry sst_main) …) lands in
  sha256sum_stream_src.shard beside the skeleton's bin over the SAME
  spec entry (fork D's per-variant contract as ratified — two bins,
  one spec, verdicts per variant; the checker accepts the pair).
  Corpus rows: the three files registered after the skeleton article
  in run_corpus.sh (dependency order tranche -> weld -> bin article);
  all three shardfmt-canonical; local canon stage-2 sweep CLEAN. The
  e3a/e3b/e3c probes at the repo root remain untracked scratch and
  are STALE as of this landing (m1r precedent — the articles are the
  source of truth). NEXT: slice D (ELF emission via enc_winelf,
  byte-tie, capless silicon pipe differential vs coreutils AND
  openssl — byte-emit legs Opus-delegated) then slice E (THE B5
  NUMBER + the fork-D verdict + records); E4 after (user rules).
- **E3 SLICE D LANDED (2026-08-09) — ELF + BYTE-TIE + THE SILICON
  PIPE DIFFERENTIAL.** The variant is now a SHIPPED CAPLESS BINARY
  adjudicated on silicon. (1) THE TIE ARTICLE examples/sha256sum/
  sha256sum_shani_elf.shard (corpus-registered, closure 2067/0 =
  the article's 2064 + 3 ties, FIRST CHECK): vsx_xfuncs = the entry
  claims' OWN module literal, extracted mechanically from the
  article's goals (loop body already expanded per the engine law,
  carriers folded), so each tie — vsx_main_eof/err/dry, the whole
  D8 disjunction at world-vector grain over (MkXModule (vsx_xfuncs)
  65536 131072) — closes by ONE unfold + the article citation, no
  compute at all (the scalar twin needed its stop-list compute
  because its ssx_xfuncs respelled carriers; extraction is the
  cheaper tie). Teeth measured: shim-slot 10/11 swap and leaf arity
  2->3 each turn vsx_main_eof RED. Bytes = enc_image_ord/
  img_offs_ord/enc_winelf at [65536,131072), entry slot 9. The
  loader's zero-fill now realizes TWO premises (total cell 65536
  AND the zero source cell 130512); the XMM file is quantified
  ARBITRARY (fn 13 ground-zeroes it in-image), so enc_winelf's
  glue is unchanged — no vector zeroing exists anywhere. (2) The
  run-only write glue sha256sum_shani_write.shard (check-green
  2081/0, NEVER a target — separation-of-duty as the twin). (3)
  THE DIFFERENTIAL sha256sum_shani_diff.sh (Opus-delegated per the
  standing split): CPUID-gates FIRST with shani_diff.sh's
  loud-SKIP wording (exit 0, "nothing adjudicated"; the CI runner
  has no sha_ni so the SKIP line is CI's expected output;
  SHANI_DIFF_FORCE_NO_SHA=1 exercises the path) — the ONLY skip;
  missing coreutils/openssl/getcap/readelf still FAILs. DOUBLE
  ORACLE per the ratified slice text: every digest row passes only
  when bin == coreutils == openssl, oracle-vs-oracle disagreement
  gets its OWN FAIL text (and the header records the caveat:
  openssl may dispatch to the same SHA silicon, so oracle 2 is
  independent software, not hardware — the hardware leaf stays
  coreutils' path). RESULT on this box (AMD 5900X, sha_ni): **56
  OK / 0 FAIL, exit 0**, twice; ELF = 10467 bytes; the product
  carries sha256rnds2 x32 + sha256msg1/msg2 x12 each (the scalar
  twin has ZERO of all three — the leg cannot vacuously test
  scalar code); abc cross-check: shani == scalar twin == both
  oracles, both bins write exactly 64 bytes, no newline.
  run_corpus.sh gained the differential block after the scalar
  leg. NEXT: slice E — THE B5 NUMBER (B4's method) + the fork-D
  §7.5 verdict + records; then E4 (user rules).
- **E3 SLICE E (2026-08-09) — THE B5 NUMBER + THE FORK-D VERDICT.
  E3 CLOSES.** Method = B4's exactly (median-of-5, core-pinned,
  warm page cache, 1 GiB, quiet box), all five contenders
  re-measured fresh (B4's baselines reproduce within noise):

      shard sha256sum_shani           0.676 s  (~1.5 GiB/s)
      coreutils as shipped (SHA-NI)   0.600 s  → gap 1.13×
      openssl dgst -sha256            0.646 s  → gap 1.05×
      coreutils, SHA-NI masked        1.738 s  → 2.6× in OUR favor
      shard sha256sum (scalar twin)  10.485 s  → 15.5× internal

  **THE B5 NUMBER: 1.13× coreutils-as-shipped, 1.05× openssl** —
  parity-class with the box's own expert SHA-NI paths, and the
  proven bin BEATS libcrypto's expert scalar path by 2.6×. B4's
  17.8×-vs-shipped gap closes to 1.13×. ATTRIBUTION (B4's gate
  protocol, perf on 64 MiB + 1 GiB): ours 3.35 instr/byte at IPC
  1.47; coreutils 2.69 at IPC 1.31 — BOTH latency-bound on the
  sha256rnds2 dependency chain (contrast the scalar bins' IPC
  4.0+), so our +25% instruction volume (the scalar glue: absorber
  bookkeeping, the spill idiom) costs only +9% cycles — it hides
  in the rnds2 latency shadow. The fold leaf IS the budget: 208
  instructions per 64-byte block = 3.25 of the 3.35 instr/byte;
  the byte-copy loop touches only sub-block tails. The cross-check
  closes exactly: 1 GiB wall 0.693 s = 0.558 s user (2.24
  cycles/byte at 4.31 GHz effective) + 0.131 s sys, which is B4's
  priced read floor (0.127 s/GiB at 64 KiB; our cap 61440).
  Nothing unexplained. Backend-debt reading for E4's pricing: the
  scalar 6.1× tax was instruction VOLUME at core-peak IPC; at the
  vector tier the same lowering idiom is 25% volume that vanishes
  into instruction-latency shadow — hand-pinning evidence that
  regalloc's win concentrates where IPC is high.
  **THE VERDICT (fork D(a)'s contract):**

      sha256sum_shani: MET (artifact: unconditional)

  Same claim form as the skeleton's (§7.5: no given, no except —
  the D8 disjunction covers every oracle behavior at every fuel,
  over an ARBITRARY initial XMM file), stated over the EMITTED
  module (slice D's byte-tie), corpus-gated on silicon behind the
  loud CPUID gate. TWO bins over ONE spec entry sst_main; the
  requirement never mentions the target — **§5's expressibility
  gate is answered by construction**, and with B4's IR gate both
  §5 silicon downgrades now read NO. B6's dispatch proof will
  case-split on the feature test and cite each variant's verdict.
  §5's B5 wall-clock-on-record obligation: met (this table).
  X86.md §53 is the x86-side record. E3 CLOSED (slices A-E all
  landed); B5's remaining ladder = E4 alone (user rules).
- **E4 CLOSED BY DISSOLUTION (user ruling 2026-08-09) — B5
  CLOSES.** As drafted, E4 was the clause-1 adjudication: with the
  artifact in hand, rule "grow the validator's disjunctive
  acceptance (clause 1 = dst freedom w/ witness tag + soundness
  leg; validator libraries under models/imp/probes)" vs "the hand
  theorem standing as the boundary insertion (CERT.md §4's R-a-b
  path)." THE RULING: the question was MIS-POSED — it treated hand
  proof as awaiting admission into "the system," when hand
  refinement proofs are the ground layer the methodology started
  on and the validator is the economy wing for the generated
  corner; validation is a proof STRATEGY (valid_P computed
  in-proof + valid_P_sound cited), never a second trust mechanism.
  B5 itself is the demonstration: the variant bin landed through
  the uniform (bin …) contract — no dispensation, no waiting — and
  the facets already interleave at fn granularity (the hand module
  carries generated gb_* bodies; the hand walks cite generated
  walk certs through the callee ladders). CERT.md §4 carries the
  law amendment: **clause 1 RE-FOUNDED as a NAMED ECONOMY DOOR,
  opened by a SECOND expert consumer** (a witness grammar from a
  sample of one = the vx_sa corpse's shape), not by this leg's
  existence. The rung's evidence obligations are BANKED in the
  slice E record: the witness grammar a clause-1 acceptance would
  need (register roles, schedule interleave, layout facts — frozen
  in the articles), the second-consumer answer (none concrete: B6
  composes proven pieces, AVX2 is a consumerless named door), and
  the backend pricing (regalloc's win concentrates at the
  high-IPC scalar tier; the expert vector leg needed NOTHING from
  the backend — the x86gen freeze never strained). **B5 CLOSED**
  (E1 vector tier, E2 block article, E3 variant bin, E4
  dissolved). Arc B's remaining ladder = B6 (§3): proven feature
  dispatch, whose §8.6 opening move is CPUID/feature modeling.

### 8.5 Forks (RULED 2026-08-02, all four to their leans — texts
### kept with the rejected options' costs in place)

- **Fork A — which tier first: SHA-NI or AVX2?** (a) SHA-NI: the
  comparison target's own fast path on this silicon (the 0.598 s
  bar IS SHA-NI), the smallest model surface (~10 XMM species,
  dword-native semantics tying directly to the 32-bit spec).
  (b) AVX2: no SHA instructions — a vectorized scalar schedule;
  single-stream gains are modest (AVX2 SHA wins by multi-buffer
  interleaving, but one pipe has one stream), VEX + YMM roughly
  double the model surface, and it cannot approach the bar this
  box sets. LEAN: (a); AVX2/YMM stays a named door — the tier
  mechanism (separate register file, twin SCC, embedding)
  generalizes when a consumer arrives.
- **Fork B — lane representation** (X86.md §6 explicitly deferred
  this; the ruling lands there). (a) Xmm = four dword lanes:
  every SHA-NI instruction's semantics is dword-native; proofs
  land on the spec's 32-bit functions with no lane surgery;
  byte-grain ops are simply not modeled until a consumer forces
  the ruling wider. (b) Xmm = one Int in [0, 2^128): purer "a
  register is one value", but every semantic arm then opens
  div/mod 2^32 lane extraction — proof noise on every step; only
  byte-grain ops get cheaper. LEAN: (a).
- **Fork C — where vector state lives.** (a) The parallel extended
  tier (§8.2): zero existing signatures touched, additivity
  proven by re-running the battery, cost = the embedding theorem
  + one thin twin SCC. (b) Widen Regs/XOut in place: dead on the
  survey's numbers (1033 positional sites + generated certs +
  x86gen templates = a repo-wide break for zero semantic gain).
  LEAN: (a) — (b) is recorded as the corpse.
- **Fork D — the dispatch-visible variant contract.** (a)
  Per-variant verdicts: the SHA-NI bin carries its OWN §7.5-form
  claim (same conclusion shape: writes sha256(everything)); B6's
  dispatch proof case-splits on the feature test and cites each
  variant's verdict; the requirement never mentions the target —
  §5's expressibility gate is answered by construction. (b)
  Cross-module equivalence: prove the SHA-NI module
  observationally equal to the scalar module and inherit its
  verdict — the hardest available theorem, and it WELDS the
  variants (a scalar regen invalidates the SHA-NI proof).
  LEAN: (a).

### 8.6 Non-goals and fences

AVX2/YMM/multi-buffer (named door); general byte-grain pshufb
(fence, loud); CPUID/feature modeling (B6's opening move — B5's
bin runs unconditionally on SHA-NI silicon, CI's runner permitting,
else the row degrades loudly like the capless legs); x86gen
emission of vector code (the freeze stands — this is the HAND leg;
x86gen learns nothing); other hash families (§6 standing).
Delegation: byte-emit, encodings, replayer C, and schedule byte
transcription are Opus-delegated per the standing split; Fable
owns semantics, laws, articles, docs.

## 9. B6 — proven feature dispatch (rung design RATIFIED 2026-08-10)

One bin whose entry asks the chip and jumps: CPUID selects between
the scalar streaming artifact (B1–B4's) and the SHA-NI artifact (B5's)
inside ONE module, under the SAME `(bin … (entry sst_main) …)`
contract the variants already share. The requirement never mentions
chips or targets; the dispatch proof case-splits on the feature
answer and cites each family's §7.5 verdict — fork D(a)'s machinery,
landed. Arc B closes at this rung's number.

### 9.1 Rulings (2026-08-10, all four to their leans — rejected
### options' costs kept in place)

- **R1 — the chip answer rides the existing oracle.** The new XCpuid
  world-layer arm consumes FOUR ints from lxans (the eax/ebx/ecx/edx
  results, 32-bit masked — the syscall arm's rcx/r11 junk-draw
  discipline extended to the four output registers), emits NO trace
  event; theorems quantify over the draws and case-split on the
  feature bits; the variant verdicts instantiate at the rest-of-list
  world; the D8 dry leg covers exhaustion (lx_step's convention:
  short answers refuse). Zero signature changes. REJECTED — a chip
  field on LxWorld: cleaner-looking separation at the cost of every
  MkLxWorld literal in the tree (measured: 136 occurrences across 19
  files — every world-grain article breaks) plus a shared-surface
  signature walk; the soundness story (quantified environment
  answer) is identical either way.
- **R2 — the dispatch bin takes the plain name.** The B6 artifact
  ships as `sha256sum`; the scalar streaming bin renames to
  `sha256sum_scalar`, with the rename recorded at every verdict that
  names it (the B4 RECORD, §7.5, X86.md §52, CI rows). The
  user-facing artifact carries the tool's name — it is what the
  parity claim compares against. REJECTED — a qualified name
  (sha256sum_fat / _dispatch): zero record churn, but the flagship's
  best artifact wears a qualifier forever while the plain name sits
  on the second-best bin.
- **R3 — variant theorems transfer by linking meta-theorems.** Two
  generic theorems over the eval tower, proven once: EXTENSION —
  appending fns to a module preserves every walk that never calls
  past the old length (the unshifted family transfers) — and SHIFT —
  uniformly mapping (XCall k) → (XCall (k+d)) over a family placed
  at offset d preserves walks (the other family transfers). Every
  future multi-variant bin reuses them. The scalar family's entry
  equations live at xrun_w; how they cross into the world-vector
  tier the merged run lives at is slice B's embedding check.
  REJECTED — re-proving one family over the merged literal: the E2b
  transplant class of work, re-paid at every future composition,
  with the theorems welded to one layout.
- **R4 — the stub does the real dispatcher's two-step.** CPUID leaf
  0 first (max basic leaf ≥ 7), then leaf 7 subleaf 0, EBX bit 29 —
  what coreutils and openssl both do, because pre-2013 silicon
  answers out-of-range leaves with the highest basic leaf's data and
  bit 29 is then garbage: a leaf-7-only artifact could jump to SHA
  instructions on a chip without them. The theorem is proven either
  way (it quantifies over all draws); the two-step makes the
  ARTIFACT honest on old silicon. Cost: one more XCpuid, one
  compare, one case leg (maxleaf < 7 → scalar).

### 9.2 The slice ladder

- **A — CPUID enters the model.** XCpuid arm on XInstr: the BASE
  tier refuses loudly (Some XTrap — the XSyscall/XVec precedent; the
  pure VECTOR tier inherits the trap through its base delegation),
  the world scalar tier draws four and writes eax/ebx/ecx/edx
  (wcpuid, wsyscall's sibling), the world vector tier mirrors
  through wvlift with the XMM file untouched, and BOTH effect
  predicates (xeff_i, xwv_i) answer True — the never-under-report
  invariant. Structural walk over every XInstr consumer +
  thread-B announcement (the case-on pad in
  tools/search/tasks/x86_transition_window.shard is built for
  exactly this growth). Encoder arm (0F A2) + silicon pinning =
  Opus-delegated. Battery additivity re-run.
- **B — the linking theorems** (R3's objects) + the world-grain
  embedding check for the scalar family's crossing.
- **C — the dispatch article.** Merged module literal (one family
  unchanged, one shifted by the R3 layout, dispatch main last); the
  R4 stub body — CPUID leaf 0, CPUID leaf 7, re-zero the four
  clobbered registers (both variant theorems enter from all-zero
  registers), XCall the chosen entry; the exit-propagation call
  lemma (WExit passes straight through the call arm — neither
  variant main returns); entry equations = the D8 disjunction whose
  proof case-splits on the draws and cites each family's transferred
  verdict; the third (bin …) form over sst_main, with R2's renames.
- **D — bytes + differential.** Tie file by mechanical extraction
  (the slice-D generator pattern), write file (Opus), and the
  dispatch differential: runs EVERYWHERE, no SKIP — the CI runner
  exercises the scalar path, a sha_ni box the SHA-NI path, digests
  double-oracled on both.
- **E — the B6 number + records.** Expected ≈ B5's 0.676 s/GiB (the
  stub is nanoseconds); the headline = the ratified parity fork (b)
  like-for-like at last: our dispatching artifact vs
  coreutils-as-shipped. Verdict + the record here + the X86.md
  section. ARC B CLOSES.

Slice-level items deliberately left open: the exact merged layout
(which family shifts — cost-symmetric under R3); how the bin
requirement's oracle bookkeeping spells the chip prefix ahead of the
read answers; and the emitted-file collision — examples/sha256sum/
sha256sum is today the ONE-SHOT's emitted path, so slice D re-homes
that emission (to match its bin name, sha256sum_oneshot) before the
dispatch artifact claims the plain path.

### 9.3 Slice records

- **Slice A LANDED 2026-08-10 — CPUID enters the model.** `(XCpuid)`
  sits between XSyscall and XVec; the base tier traps (the pure
  vector tier inherits through its base delegation, no arm needed);
  `wcpuid` (world.shard, wsyscall's sibling) draws four →
  eax/ebx/ecx/edx each `mod 2^32` (silicon zero-extends CPUID's
  dwords), NO trace event, short answer list refuses; vworld mirrors
  through wvlift; xeff_i AND xwv_i answer True. THE STRUCTURAL WALK,
  measured: outside the model files exactly TWO consumers paid —
  models/imp/probes/vx86_acc_probe.shard (11 `(XCpuid False)` arms:
  vx_regi + the pair table enumerate ctors with NO catch-all — the
  alphabet stays a conscious decision; plus 12 case-on pads) and
  tools/search/tasks/x86_transition_window.shard (1 pad + a use
  line: the file imports ctors ONE BY ONE — a new ctor needs its
  `(use …)` before any case can name it). THE PAD RULE: copy the
  local `(case XSyscall …)` donor and rename the token — XCpuid is
  nullary like XSyscall and every predicate the pads reduce treats
  both alike. Encoder (Opus): ONE arm `(XCpuid (list 15 162))` —
  xsz_instr/enc_instr reach nullary ctors through enc_simple
  catch-alls, so size cannot disagree with emission. Silicon pin
  (Opus): six CPUID plan rows (leaves 0, 1, 7/0, 7/1, 8000_0000,
  8000_0001; leaf setup = the model's own XMovRR32 instructions) +
  the `cpuidno` inverted tooth in the UNGATED leg — replayer
  trampoline (cpuid clobbers callee-saved rbx) vs
  __get_cpuid_count; x86_diff 230/0 (was 223/0, delta exactly the
  new rows); negative control (opcode flipped to 0F A3): six loud
  hardware-fault FAILs, reverted; sha-ni leg untouched, 62/0.
  Gates: x86/world/vworld/vector + probes + encode 303/0 + diff_run
  638/0 green; deep smoke std/sha256/sha256.snweld.shard 1819/0
  (the heaviest wvcall_bridge consumer — landed world-grain certs
  replay untouched); all touched files shardfmt-canonical.
- **Slice B LANDED 2026-08-10 — the linking layer:
  `models/x86/link.shard` (674/0, corpus-registered).** ONE article,
  three theorem families over a query-sum encoding of the four-fn
  eval SCC (`XQ`: instr / call / loop-walk / seq — one claim per
  theorem, induct fuel, every other binder generalized):
  - **The embedding tower** — `xlk_embx` (base→vector), `xlk_embp`
    (vector→world-vector), `xlk_adeq` (base→world, static-scan
    premised — forced by the 96/1024 budget disagreement), and
    `xlk_embw` (world→world-vector, THE CROSSING for the scalar
    family's entry equations). Premise = the SEMANTIC ok
    discriminator of the lower walk (None rides, only the trap is
    excluded) — no static vector-freedom scan needed.
  - **EXTENSION** — `xlk_extx/extv/extwv`: appending functions to a
    module preserves every walk whose code and reachable family
    RESOLVE in the base family. The calls-below predicate is spelled
    as call-RESOLUTION (`xlk_cb*`: XCall k resolves ⟺ `xfunc_at`
    answers Some) — resolution IS the in-range fact, so the lemma
    layer is arithmetic-free; the only farkas lemmas are the
    prefix-skip index fact (`xlk_at_skip`) and resolution⇒nonneg
    (`xlk_at_nonneg`).
  - **SHIFT** — `xlk_shx/shv/shwv`: uniformly mapping (XCall k) →
    (XCall (k+d)) over a family placed at offset d = len pre (m2 =
    pre ++ shifted family ++ post, same window) preserves walks:
    `xlq_T fuel m2 (xlk_shq d q) = xlq_T fuel m1 q`. The dispatch
    scans ignore the shift (`xlk_scv_sh`/`xlk_scw_sh` + fn-level
    corollaries — both scans judge XCall by CONSTRUCTOR, never by
    index).
  Both families stratify downward exactly like the embedding tower:
  each upper tier cites the tier its call arm delegates to, at
  (S f2) — extwv→extv→extx, shwv→shv→shx. Denotation corollaries
  `xlk_extrun`/`xlk_shrun` transfer both families to `xvrun_w`, the
  wv-tier bin boundary slice C's dispatch article composes at (the
  shifted entry runs at index k+d). The run-grain form of the
  EMBEDDING crossing (xrun_w equations → xvrun_w) stays a slice-C
  item as ratified (WExit inverts through the wrapper; the None/dry
  leg needs a seq-grain fact — decide there).
  Mechanics of record: mutual STRUCTURAL recursion over
  XInstr/List XInstr (the encoder's xsz precedent) carries both the
  resolve predicate and the shifter; the six main claims are
  generator-emitted (session scratchpad, gen_ext.py/gen_shift.py —
  the probes are the source, b6b4/b6b6/b6b7/b6b8 untracked at
  root, now STALE relative to the landed article); EXTENSION's
  non-call arms close by α-equality after scrutinee alignment
  (rewrite the continuation-equality have into the shared
  scrutinee, fork-first only where continuations sit under match
  binders: loop re-entry, seq tail); SHIFT's call arm is the
  four-lemma lookup chain at_nonneg → at_skip → at_app → at_sh.
  Full check 0.3 s; corpus row after sha256sum_shani_elf.
- **Slice C LANDED 2026-08-10 — the dispatch article:
  `examples/sha256sum/sha256sum_dispatch_x86.shard` (2124/0,
  corpus-registered) + the third `(bin sha256sum …)` over sst_main +
  the R2 renames.** The open slice-level items, decided: the SHA-NI
  family SHIFTS (scalar keeps 0-14 and its landed indices; SHA-NI
  moves to 15-29, entry 24 = 9+15; dispatch main = fn 30), and the
  chip answers ride as a QUANTIFIED Cons prefix ahead of the variant
  answer list, with each D8 leg split into TWO claims by the max-leaf
  gate — `_lo` (4 draws, premise `(lt (mod c0 2^32) 7) = True`,
  scalar always: old silicon consumes ONE cpuid) and `_hi` (8 draws,
  premise False, the proof case-splitting on
  `(int_eq (band (bshr (mod c5 2^32) 29) 1) 0)` — EBX bit 29 of the
  leaf-7 draw-set). One claim per maxleaf case is forced, not
  stylistic: the second draw-set's cells sit BETWEEN the first and
  the variant answers, so no single world spelling makes both
  consumption depths an instance of `ssm_ws its TAIL`. Six claims
  total, `dsm_main_{eof,err,dry}_{lo,hi}`, every branch delivering
  the SAME canonical RHS (eof: `WVExit 0` with
  `bytes_hex (sha256 (scat (ssw_chunks its)))`; err: `WVExit 1`
  after LxReadErr; dry: None) — the parity design's whole point at
  the bin boundary. Chip-short answer lists refuse through wcpuid
  exactly as mid-read exhaustion refuses through lx_step (B3's D8
  closure convention; noted in the article header).
  THE MECHANICS OF RECORD:
  - **The exit-propagation call lemma DISSOLVED.** Restating the six
    variant verdicts at BODY grain (weval_seq/wveval_seq over the
    original modules — sdb_*/vdb_*) turned out to be MINUS-ONE-UNFOLD
    REPLAYS of the landed proofs: the run wrapper maps WExit to
    itself and None to None, so every RHS and every proof step after
    the wrapper unfold carries over VERBATIM (all eight restatements
    passed first shot). With body-grain facts in hand the call arm
    just computes — no wrapper inversion anywhere, and the dry leg
    (None does NOT invert through the wrapper: body-walk None, WVBrk,
    WVTrap all map to it) needs no special treatment. This also
    settled the deferred run-grain embedding seam: the crossing
    happens at (XQS body) via xlk_embw (None rides), never at the
    wrapper.
  - **The transfer ladder**: sdw_* cross the scalar body facts
    world→wv with the XMM file quantified; sdt_*/vdt_* land both
    families on the merged module via xlk_extwv / xlk_shwv (the
    shifted query respelled by one xlk_shq unfold). Every linking
    premise is refl-grade or computes on the ground lists — the
    slice-B consumer API held exactly as recorded.
  - **Fuel**: N = 1610 calibrated (deliberately-failing probe claims
    with ground draws; read the stalled call's decimal fuel from the
    trace — but measure the CONCRETE MkXFunc body-walk term, NOT the
    pending-arm `(xbody_of @0)` towers, which cost one wrong
    recalibration). Scalar legs arrive at exactly
    S^1601 (ssm_fuel its f); the SHA-NI leg sits at S^1593 and its
    verdict wants S^1390 — the 203-unit slack rides through the
    verdict's quantified f as `(dsm_pad (S^203 Z) f)` with
    dsm_fuel_pad commuting the pad through ssm_fuel (std/nat's
    add_nat is OPAQUE by module discipline — the article defines its
    own transparent pad).
  - **Proof-law reruns**: L9 struck twice (xfuncs_of and xbody_of
    both stall on ctor-with-stuck-field in guard position — window
    haves / unfold-the-accessor both times); the wvlift value
    delivery must close by unfold+reduce, NOT compute (compute chews
    into ssm_tr/sha256's symbolic normal form past the folded RHS
    spelling).
  - **R2 executed**: `(bin sha256sum_scalar)` renamed in
    sha256sum_stream_src.shard; rename annotations at the B3 RECORD
    verdict line, the B4 RECORD benchmark row, X86.md §52. CI-row
    renames ride slice D with the emission re-homing, per the ladder.
  Remaining for slice D: tie + write files (Opus), the emitted-path
  collision (one-shot emission re-homes to sha256sum_oneshot before
  the dispatch artifact claims examples/sha256sum/sha256sum), the
  everywhere-runs differential.
- **Slice D LANDED 2026-08-10 — BYTES + THE EVERYWHERE-DIFFERENTIAL.**
  The dispatch bin is a SHIPPED CAPLESS BINARY that adjudicates on
  EVERY box. (0) THE RE-HOMING, first per the ladder: the one-shot
  emission moves examples/sha256sum/sha256sum →
  sha256sum_oneshot (write-glue path + elf header +
  silicon_diff.sh's ONESHOT + labels); the scalar/one-shot CI leg is
  renamed `sha256sum-scalar-oneshot-silicon` with a dated
  annotation; no other repo file pointed the plain path at the
  one-shot, so the plain path now belongs to the dispatch artifact
  exactly as R2 rules. (1) THE TIE ARTICLE examples/sha256sum/
  sha256sum_dispatch_elf.shard (corpus-registered, closure 2130/0 =
  the article's 2124 + 6 ties, FIRST CHECK): dsx_xfuncs = the entry
  claims' OWN module term — the COMPOSED spelling
  (xlk_app SCALAR15 (xlk_app (xlk_shfns 15 SHANI15) (Cons STUB
  Nil))), extracted verbatim by a single-pass paren-matcher from
  dsm_main_eof_lo and verified byte-identical across all six claims,
  NOT flattened: xlk_app/xlk_shfns are ordinary computable fns the
  encoder evaluates at emission time. Each tie dsx_main_{eof,err,
  dry}_{lo,hi} closes by ONE unfold + the article citation (empty
  inst list, premises discharged by index) — the extraction-is-the-
  cheaper-tie law holds at six claims as it did at three. Bytes =
  enc_image_ord/img_offs_ord/enc_winelf at [65536,131072), ENTRY
  SLOT 30; the loader zero-fill realizes the SAME two premise cells
  as the shani twin (the dispatch claims carry the UNION of both
  variants' premises; identical window, enc_winelf glue unchanged;
  XMM file quantified arbitrary, no vector zeroing exists). TEETH
  measured: swapping the stub's adjacent re-zero pair (XMovRI RBX 0)
  /(XMovRI RCX 0) and the shifted-entry off-by-one (XCall 24)→
  (XCall 23) each turn ALL SIX ties RED (2124/6); reverted, green.
  (2) The run-only write glue sha256sum_dispatch_write.shard
  (check-green 2144/0, NEVER a target) writes the plain path. (3)
  THE DIFFERENTIAL sha256sum_dispatch_diff.sh (Opus-delegated per
  the standing split): RUNS EVERYWHERE, NO SKIP — missing
  coreutils/openssl/getcap/readelf/objdump/dd all FAIL; the cpuid
  probe only REPORTS which arm this box's silicon takes; the
  BOTH-FAMILIES DISASSEMBLY TEETH assert on every box that the
  image carries sha256rnds2 ×32 + sha256msg1 ×12 + sha256msg2 ×12 +
  cpuid EXACTLY ×2 (count-exact; disasm starts at the entry file
  offset computed from the headers, not a pinned constant) — the
  leg can never vacuously pass on scalar-only silicon; DOUBLE
  ORACLE per row with oracle-vs-oracle disagreement its own FAIL;
  cross-rows against the freshly-emitted scalar twin
  (unconditional) and shani twin (sha_ni-gated, labelled extra
  evidence never a gate — the shani twin would SIGILL without the
  flag). RESULT on this box (5900X, sha_ni): **65 OK / 0 FAIL,
  exit 0**, three runs; ELF = 18974 bytes; path line "sha-ni".
  WHICH-HALF-RAN evidence (out-of-band, deliberately not a script
  row): 256 MiB timed through all three bins — dispatch 0.169 s ≈
  shani twin 0.167 s, 16.7× the scalar twin's 2.818 s: the stub
  reaches fn 24, not merely the right digest. Regressions: the
  re-homed scalar/one-shot leg 67/0, the shani leg 56/0; CI's
  runner (no sha_ni) adjudicates the scalar arm of the SAME
  binary — the ratified everywhere contract. NEXT: slice E — THE
  B6 NUMBER (expect ≈ B5's 0.676 s/GiB; the stub is nanoseconds) +
  the like-for-like headline (our dispatching artifact vs
  coreutils-as-shipped) + verdict + records + the X86.md section.
  ARC B CLOSES.
