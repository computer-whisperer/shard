# STORAGE.md — the storage & incremental slice: arena, images, and the per-module skip

Status: RATIFIED 2026-08-02 (user; all three §6 forks ruled at
their leans same-day: F1 = the S1→S2→S3 ladder, F2 = uncommitted
local cache, F3 = module-hash + mod.req-interface v1). This is
CERT.md §7's design note under the gated-slice protocol. S1 is the
OPEN slice; S2/S3 open only on the prior slice's gate numbers. Charge
(CERT.md §7, DRAFTING SCHEDULED 2026-08-01): the redirection's one
unmeasured gate — §9 (d) local-edit incremental behavior — plus the
per-module check cache (github #7; CERT.md cites it as "task #62",
a stale number — the tracker item is issue #7) and DC4 (cert binary
serialization format), carrying Arc A rung (e)'s sharing mandate:
term sharing arrives ONCE, here, priced with serialization +
content addressing + incremental behavior in a single design.
Evidence trail: docs/archive/A3E-PRICING-2026-07-26.md; CANON.md §7
(the ratified content-addressing spec + shipped hash tool); the B4b
RECORD (STREAM.md), which resolved the pricing memo's fork D.

## 1. The measured problem

Three facts, all instrumented (SHARD_STATS, whole closures,
bin/shard_check):

**(i) Checking a small file costs its closure, not its content.**
The standing exhibit (STREAM.md B1b RECORD): sha256.xcomp.shard —
154 formatted lines, one claim family — costs 2.38B calls / 1.22GB
live / 4.17GB RSS, because it imports both the x86 out file and the
oracle probe and the checker RELOADS AND RE-ELABORATES the entire
closure text on every check. Nothing about the 154 lines is
expensive; the closure is.

**(ii) The bill is the load floor, not proof checking.** The
pricing memo's cost census on the out-file closure: reader
(is_ws+skip_ws) 317M calls ≈ 31.5%; env_lookup pair 238M ≈ 23.6%;
type-gate substitution 58M; name resolution ~85M. The proof-step
machinery — expr_match, rewrite_all, ceval_list — sits at the
BOTTOM of the top-40 at ~2–3M calls per fn. Parse + names + env
traffic is linear in closure TEXT and is paid in full on every
check of every importer.

**(iii) Deleting text is worth more than memoizing walks — fork D
is now a number.** B4b replaced the replay block chains with the
conversion dialect and deleted 61k generated lines: the direct
closure fell 1,130.2M → 761.5M calls (−32.6%), live peak −44.0%,
maxrss −32.6%. The pricing memo's extrapolated conversion-walk
cost (170–400M) was swamped by the deletion credit, exactly as
fork C predicted. Consequence: the memo's addressable pool for a
checker-level compute memo shrank again — the residual 761.5M on
the out closure is still dominated by the load floor, which no
memo reaches. The next order of magnitude is storage's, not the
normalizer's.

Corpus-scale consequences (github #7): every check re-verifies its
whole import closure from scratch; kernel-touching edits drop the
corpus to the interpreted engine (~40GB RSS) until a ~1h native
rebuild; the full corpus is a ~40-minute CI job. Check cost grows
as closure size × edit frequency — super-linear in project scale.

## 2. The design: three layers, one identity

The unit of identity everywhere is CANON.md §7's ratified
content hash (sha256 over the canonical nameless core, Merkle over
references, shipped as tools/canon/hash.shard). The three layers
consume it at three granularities; none of them enters the trust
ledger.

**L1 — the per-module check certificate (the skip).** A driver-side
cache: `(module content hash, transitive interface hashes, kernel
hash, engine stamp) → checked-clean`. On a check run, modules whose
certificate matches are SKIPPED as claim-check targets; anything
downstream of a changed interface re-checks. mod.req interface
hashing does the invalidation work: impl-only edits inside a module
do not invalidate consumers (their proofs only see the interface).
Both halves exist today — engine stamps (bin/engine_stamp.sh) and
the canon hash tool — they have never been combined into a skip.
NO kernel contact: this is check-driver logic over existing text
files. What it does NOT fix: the load floor — a skipped module's
text is still parsed and elaborated to build the importer's
environment. What it DOES fix: re-CHECKING unrelated claims, and it
produces the first real measurement of §9 (d) at module
granularity.

**L2 — binary module images (DC4).** A module's ELABORATED core —
post-reader, post-canon, post-type-gate — serialized to a binary
image keyed by the module's content hash; the loader maps an image
instead of re-parsing text when the key matches; source is rendered
on demand (shardfmt from the core) rather than stored twice. This
is the layer that kills the load floor: parse (31.5%) and the
elaboration share of env/name traffic are paid ONCE per content
hash, not once per check of every importer. The serialization
schema is CANON.md §7's identity bytes extended from per-definition
to per-module segments — the format already ratified as the STABLE
CONTRACT, with the digest a replaceable parameter behind hx_digest.
Images are caches: re-derivable from text at any time, deleted
freely, never committed, never trusted — a mismatch or absent image
means "load from text". The kernel checker's semantics are
untouched; the loader grows a second front door that must produce
IDENTICAL in-memory results (a differential gate, image-load vs
text-load, over the corpus).

**L3 — the hash-consed arena + conversion memo (rung (e)'s
mandate).** The live term representation becomes an interning
arena: structurally equal terms share one node id; equality on the
hot paths becomes id comparison; the conversion memo keys on node
id pairs and becomes cheap by construction. This is the deepest
commitment in the redirection — the representation threads
reader/types/checker, all canon-owned — and it is DELIBERATELY
LAST: its addressable pool is what remains after L1 stops
re-checking and L2 stops re-parsing, and the honest projection from
the pricing memo plus B4b is that this residual is the SMALLEST of
the three. L2's image format is designed arena-shaped (nodes +
references) so L3 changes the live representation without changing
the on-disk contract.

## 3. Sequencing and gates (the slice ladder)

- **S1 = L1**, first, because it is kernel-free and produces gate
  (d)'s first number. Gate: on the standing benchmark (edit one
  impl line of a leaf module; edit one impl line of sha256.imp;
  edit a mod.req interface), the re-checked set must be exactly the
  dependency-forced set, measured by the driver's own log. DOWNGRADE
  if interface hashing cannot separate impl edits from interface
  edits in practice.
- **S2 = L2**, second, priced on S1's residual (the load floor is
  then the whole remaining bill by construction). Gate: warm-image
  check of the B1b exhibit (154-line file over the big closure)
  drops from 2.38B calls to within a small multiple of the file's
  OWN cost; the image-vs-text differential gate is corpus-green.
  DOWNGRADE if image invalidation churns so much in normal work
  that warm hits are rare (measure hit rate over a real session).
- **S3 = L3**, last, priced on S2's residual with its own
  gated-slice review (this is the canon-owned representation
  change). Gate: §9 (b) — checker work linear in unique nodes on a
  sharing-heavy closure (the weld/sweld composition files are the
  benchmark), plus the conversion-memo pool actually measured
  before the memo is built (the A3 lesson: do not build the memo
  for a pool the naming discipline already drained).

Each slice lands with instruments in the driver so the next
slice's pricing is a by-product — the same discipline that let B4b
resolve fork D for free.

## 4. Trust posture (unchanged, stated once)

Certificates and images are CACHES for the dev loop. The
full-closure, from-text, Rust-authority corpus run remains the
soundness gate (CI's corpus job; the pre-commit discipline).
Nothing keyed by hash ever feeds the trust ledger: a cache hit
asserts "you already checked this content with this engine", never
"this is true". The canon hash's stability constraint (hash only
canonical forms; the serialization is the contract) is already law
via CANON.md §7. Poisoned-cache posture: deleting the cache
directory is always safe and always available; the differential
gate (S2) is the only new verification obligation this design
creates.

## 5. DC4 resolution (proposed)

DC4 ("cert binary serialization format") resolves INTO S2: the
format is CANON.md §7's tagged constructive encoding, extended
with a module-segment header (definition table, interface slice,
content-hash key) — one schema serving definition identity (canon,
shipped), module images (S2), and, if S3 wants persistent arenas
later, arena segments. No second format is ever designed. The
digest stays a parameter behind hx_digest.

## 6. Forks (RATIFIED 2026-08-02 at the leans, user ruling)

- **F1 — slice order.** (a) S1→S2→S3 as above (LEAN: each slice
  prices the next, kernel-free first, the deepest commitment last
  and possibly never — if S1+S2 satisfy gate (d) and the residual
  proof-step pool stays at the measured ~2–3M/fn floor, S3 can be
  CLOSED-DORMANT the way DC3 was). (b) Arena-first (the pricing
  memo's fork B shape): REJECTED-because the memo already priced it
  — building the deepest commitment first, on marginals that don't
  demand it, ratifies §7's core by side effect and inverts the
  evidence ladder.
- **F2 — cache home and lifetime.** LEAN: an uncommitted local
  cache directory (.shard-cache/ at repo root, gitignored), never
  in the tree, never in CI artifacts at first — unlike proof
  sidecars, these caches are machine-local and engine-keyed.
  Alternative (committed, CI-shared) only if S1's measured win is
  large enough that CI wants it too; that lands as its own slice
  with its own poisoning review.
- **F3 — invalidation granularity.** v1 = module content hash +
  mod.req interface hashes (github #7's sketch, buildable today).
  The refinement — per-definition Merkle reachability via the canon
  hash, so a consumer re-checks only claims whose reached
  definitions changed — is named as S1's growth rung, NOT built
  first (it wants the canon hash meaningful over the whole corpus,
  which stage-2 canonicalization currently guarantees for std
  only).

## 7. What this note does not do

No code moved. No schema frozen (S2's schema freezes at S2's
ratification, on S1's numbers). No kernel or canon-owned file is
touched before S3's own gated review. The note's ratification
opens S1 only.

## 8. S1 RECORD (2026-08-02; landed same-day as ratification)

Mechanism (bin/check): key = engine stamp + sha256 over the
target's transitive import closure — file imports, directory-module
members recursively, and every member's `.auto.shard` sidecar
(sidecars are check inputs; the prove-regen flow must invalidate).
Only the stamp-fresh shard_check engine caches; EVAL-ladder runs
never do. Success-only writes (`0 failed` verified in the output);
unresolvable imports fail OPEN to a full check; certs live in
`.shard-cache/check/` (gitignored, F2 as ruled); corpus/CI never
consult the cache.

Numbers. The standing exhibit (sha256.xcomp, the 2.38B-call
154-line file): 11.29s cold → 0.103s warm — 109×, and the warm
cost is pure closure hashing (~60 files). Invalidation on the
three-edit benchmark is exact: an edit OUTSIDE the closure
(models/riscv) leaves the cert valid; an impl edit INSIDE the
closure (sha256.imp) forces the re-check; a SIDECAR edit
(sha256.stream.auto.shard) forces the re-check. §9 (d) at module
granularity: the re-checked set is exactly the dependency-forced
set. What S1 does not touch, measured: the cold check still pays
11.3s for 154 lines — the load floor, S2's charge, now the whole
residual by construction.

**F3 AMENDMENT (discovered building v1, surfaced to the user).**
The ruled F3 lean said "module content hash + mod.req interface
hashes" — github #7's premise that impl edits cannot affect
consumers. That premise is FALSE in shard today: consumer proofs
COMPUTE imported fn bodies (every fuel tower runs nat/mem ops;
compute reaches through the surface), so impl bytes are genuine
check inputs and interface-only keying would produce FALSE CACHE
HITS. v1 therefore keys on whole content — over-invalidation only,
never a wrong hit. The mod.req-granular refinement remains F3's
growth rung with a new entry condition: it may key a consumer on an
import's interface ONLY for imports whose members provably cannot
enter the consumer's compute (opaque-surface modules), or after a
mode-aware-resolution mechanism actually hides impl bodies. This is
a scope finding, not a §3 DOWNGRADE: the skip delivers gate (d)
regardless; interface granularity was a hit-rate refinement.
