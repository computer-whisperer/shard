# CERT.md — the certificate architecture: conversion, sharing, validators

> **STATUS (reset 2026-08-22): LAW.** certificate representation — conversion forms, validators, storage; Arc A CLOSED 2026-07-26. §10's post-Arc-A ORDERING is RETIRED by the reset; its two standing laws (no new replay-cert families; generators born speaking the ratified dialect) STAND. The backlog is the GitHub issue tracker (labels `arc:coverage` / `parked` / `debt`; the goal = #23, the prune arc = #24) — any "next arc/rung" pointer below is history unless it names an issue.

STATUS: direction RATIFIED 2026-07-18 at the three-way design review
(Christian + Fable + codex; the full correspondence is archived at
docs/archive/DESIGN-REVIEW-2026-07-18.md — this ledger is its
distilled law and takes precedence for anything it states). The
pathfinder arc (Arc A, §8) is OPEN and runs SERIAL ON MAIN (user
ruling 2026-07-18: careful design iteration over parallel speed).
Planning beyond Arc A is deliberately HELD until the pathfinders
prove or falsify the architecture (user ruling, same date) — no Arc
B/C/D content is law yet; the archive records the candidates.

## 1. The measured problem

The replay-certificate dialect is the binding constraint on the
project. Two independent instruments agree: the review's inside-view
analysis, and a git-timeline audit run blind to it. The numbers:
~45% of the tracked repo is generated certificates; the sha256 chain
alone is ~255k lines (34k hand + 221k generated); sha256's three
cert files are 244k of std's 312k lines. Almost none of that text is
information — it is representation overhead with four identifiable
sinks:

1. **State-spelling repetition** (dominant): every chain/segment
   lemma restates the full machine state at every boundary, both
   sides of every equation — quadratic-shaped text for linear
   content.
2. **Instruction/tie literals**: translated instruction lists pinned
   as full ctor literals and re-threaded through claims, because
   citation needs syntactic matching against literals.
3. **Fuel-tower bookkeeping**: exact S^N constants, reshape haves,
   burn laws — an arithmetic shadow-economy serving syntactic tower
   matching.
4. **Spelling bridges (the weld tax)**: byte-copied spellings +
   compute-both bridges + stop-set choreography wherever
   independently-produced artifacts meet.

All four share one cause: **the rewriter/citation matcher is purely
syntactic**. States must be spelled because a named state fn would
not match; ties must be literals because a translator application
would not match; towers must be exact because slack shapes would not
match; spellings must be byte-copied because convertible-but-not-
identical terms do not match. The pure-syntactic matcher was the
right conservative v1 choice; 255k lines is its measured bill.

## 2. The architecture: untrusted transformations, three mechanisms

The ratified stance, jointly held by all three reviewers:
transformations stay UNTRUSTED, always — no compiler executable ever
joins the TCB. Repetitive semantic reasoning moves from proof replay
into once-proved, composable machinery. Three mechanisms, all
needed:

- **Conversion is the naming mechanism** (§3): a cheap way to say
  "use this compact name for a definitionally equal large term."
- **DAGs are the sharing mechanism** (§7): hash-consed terms and
  content-addressed certificates so repeated subterms cost once.
- **Validators are the amortization mechanism** (§4): the large
  semantic proof is proven once per pass and cited per program.

The goal state: the 200k-line result disappears from BOTH the
repository (source form) and the checker's working set (actual
checking work). Syntax compression without checking compression
fails the second half.

## 3. Explicit conversion: `change` / `exact-conv`

The kernel gains EXPLICIT conversion forms, not implicit
conversion-aware search:

```
change SIDE OCCURRENCE COMPACT_TERM (stop ...)
exact-conv CITATION (inst ...) (proof ...) (stop ...)
```

`change` replaces a selected term with a compact term only after the
kernel normalizes both under the stop set and verifies equality;
subsequent rewriting is again syntactic. `exact-conv` closes a whole
equation from an explicitly instantiated cited theorem when the two
equations are convertible. The author/generator says WHERE conversion
is wanted; the expensive operation is visible, cacheable, and has a
specific failure boundary.

**REJECTED-because — implicit normalize-on-failed-match:** it is
matching/unification modulo conversion, operationally heavier (stop
sets, fuel exhaustion, occurrence order, capture, candidate-subterm
choice), and an implicit fallback can turn a cheap syntactic walk
into nodes × normalization-cost with unpredictable proof
performance. Add matching modulo conversion only if the pathfinder
proves the two explicit forms cannot express the important cases
compactly.

Kernel-pressure accounting: zero new axioms, zero new logic — the
reduction relation is already the kernel's own; this changes WHERE
it is applied (matching), not WHAT is true. Two riders come with it:
memoization by hash-consed terms (§7 synergy), and SORT-AWARE
MATCHING (the known Int/Nat ground-literal rewrite-atom hazard —
needed anyway as a soundness hardening; it gates this feature). The
three-valued matcher and opaque module boundaries carry over
unchanged: sealed fns do not reduce; stop sets already model that.

Known boundaries, recorded at the DC2 freeze (2026-07-26): (1)
`exact-conv` cannot cite WfInduct/SubtermInduct induction hypotheses
— their binders are checker gensyms an author cannot name in
`(inst …)`; the refusal is loud. If a deep run needs
conversion-closure against a strong IH, that is a named
expressiveness rung, not a bug. (2) On gated bare-literal patterns,
occurrence counting (`false`/`(at K)`) counts only sort-compatible
sites — refused sites are skipped, not counted — which differs from
compound patterns' counting. (3) Stop sets are per-site; compute
fences and change stop sets are hand-kept in sync today (an ambient
per-steps-block fence is compatible future QoL, no syntax change).

What conversion buys, sink by sink: sink 1 — boundaries become named
state fns (st_17, defined once), citations fire through definitions,
a 300-line lemma becomes ~5; sink 2 — claims state (imp2x_fn prog)
directly and consumers cite through the application, instruction
literals survive in exactly one place (the final byte-tie, content-
addressable); sink 3 — convertible tower spellings stop needing
bridge haves (the exact-fuel remainder dies via §6); sink 4 — dies
almost entirely, compute-both bridges ARE manual conversion checking.

## 4. Validators: certify a relation, not a replay

For each regular pass P:

```
valid_P : Src -> Dst -> Witness -> Bool

valid_P_sound :
  valid_P src dst witness = True
  -> ObsDst(dst) refines ObsSrc(src)
```

The untrusted compiler emits `dst` and `witness`; the per-program
proof computes `valid_P` and cites `valid_P_sound`. A witness
carries block correspondence, chosen registers, loop invariants,
layout facts, schedule choices — DATA in a small pass-specific
grammar, never a generic proof language. Properties preserved (the
house non-negotiables, verified in review):

- The compiler has no authority: wrong output or witness is
  rejected.
- A hand theorem `R a b` can be inserted at any boundary, feeding
  `b` to the next validator — hand and automatic paths compose by
  the same refinement-transitivity theorem.
- A compiler may choose among many legal targets; it is not forced
  to be one canonical verified lowering.
- Validators and their soundness theorems are ordinary shard
  libraries — the kernel never learns a compiler pass.

**A1 adjudication (full-arc review 2026-07-26; evidence:
docs/archive/ARC-A-REVIEW-2026-07-26.md §2a).** The landed A1 tier
is a RECOMPILE design — acceptance pins dst uniquely, so the
many-legal-targets bullet above is DEFERRED, not delivered; the
honest name for this tier is "verified canonical lowering with an
extensional acceptance check" (every trust property above holds; the
compiler stays out of the TCB). Gates-dissolved is RATIFIED for
SHAPE gates: acceptance rides the compiler's own success set plus
the SEMANTIC fences (bands, constant shift counts in range) the
compiler does not check — hand shape grammars are banned (the vx_sa
corpse: it silently under-approximated the compiler by 24 live
leaves, commit 42daf67). The growth door is the CLAUSE ARCHITECTURE:
recompile-equality = clause 0 of a disjunctive acceptance; dst
freedom arrives as new clauses, each with its own soundness leg and
witness tag, triggered by the FIRST real non-canonical consumer
(expected: Arc B's hand-optimized/SHA-NI leg). Witness v0 is an
ARITY PLACEHOLDER — one guard reads it (a sum check), it carries no
independent information, and soundness never touches it; kept to
hold the (src, dst, witness) shape open. DC1 is MOOT at this tier;
delete-or-grow is decided when clause 1 arrives.

**Clause-1 re-founding (user ruling 2026-08-09; the named consumer
in hand).** The clause-architecture sentence above framed dst
freedom as "triggered by the FIRST real non-canonical consumer,"
with the hand theorem as a boundary insertion awaiting adjudication
(STREAM.md's E4). That framing had the dependency INVERTED, and the
consumer's arrival demonstrated it: Arc B's SHA-NI variant bin
landed through the uniform `(bin …)` contract — kernel-checked
theorems, no dispensation, verdict `sha256sum_shani: MET (artifact:
unconditional)` — before any adjudication happened, because hand
refinement proofs are the GROUND LAYER this methodology started on
and never left. The validator is the economy wing built so the
GENERATED corner is affordable (§1's measured problem), and
validation is a proof STRATEGY — `valid_P` computed inside a
kernel-checked proof citing `valid_P_sound` — never a second trust
mechanism beside proof. One system, two facets; the house
properties above always said so ("hand and automatic paths compose
by the same refinement-transitivity theorem"), and the facets
already interleave at function granularity (the hand SHA-NI module
carries generated `gb_*` bodies; its hand walks cite generated walk
certs through the callee ladders). CONSEQUENCE: **clause 1 is
RE-FOUNDED as a NAMED ECONOMY DOOR, not an admission mechanism** —
it opens when a SECOND expert consumer makes the witness grammar's
generality real; a grammar designed from a sample of one is the
vx_sa corpse's shape (silent under-approximation). Until then the
hand-article path is the priced, repeatable route for expert legs
(STREAM.md §8.4 slices A–E measured it). REJECTED-because, on
record: "build clause 1 now on the SHA-NI witness evidence" died on
(i) the sample-of-one grammar risk, (ii) no second expert consumer
in sight (B6 composes already-proven pieces; AVX2 is a consumerless
named door), (iii) the hand path's measured, falling cost.

The historical blocker to generic simulation arguments was the v1
untyped machine; the v2 crystallized kinds made alignment
program-independent BY TYPE, which is what makes the generic
induction tractable now. impgen is already an executable
specification of much of the relation — it is PRESERVED as the
oracle/regression source while stable cert families are replaced by
validator clauses, one at a time.

Under this architecture the per-program PROOF for ordinary
imp-to-ISA passes is tens-to-hundreds of lines plus witness data;
sha256's honest per-program floor (spec, imp spelling, one invariant
per genuine loop, ground pins, one byte literal per artifact) is
~3-6k lines. The floor for a default-path program is spec + bin
declaration. What does NOT compress: genuine loop invariants and the
hand-optimized clever-spelling proofs an author chooses to write.

## 5. Base+patch proof-facing states

Proof-facing machine states become base + ordered patch sets; the
models' eval stays the semantic authority, the patch view is an
OBSERVATION layer with a collapse theorem (patch-apply = eval
effect) proven once per model — the same shape as std/mem's law
family and the floats NaN quotient. The hand-built framing families
(below/above/point-grain/wlist-grain) are ad-hoc patch-composition
laws discovered one shape at a time; base+patch internalizes the
family. This is also the representation the heap era needs (heap
framing and parallel disjointness are patch-footprint statements) —
the separation library gets built IN this vocabulary from day one
(see docs/MEMORY.md note), pending Arc A's verdict.

## 6. Runs/RunsWithin: the proof-facing cost interface

Exact fuel becomes interpreter-internal. The proof-facing relation
is Runs/RunsWithin with a cost algebra: composition by cost
addition, monotonicity absorbing surplus (proven once per machine),
machine cost models refining abstract cost separately. Ordinary
library, no kernel rule. Sink 3's shadow-economy dies here;
impgen's tcost/gcost empirics become theorems. The same relation is
the natural home for observation-refinement (MEMORY.md D8) and,
later, parallel-tier cost claims.

## 7. Storage: hash-consed DAGs, content-addressed certificates

Hash-consed term representation, content-addressed cert sidecars,
binary serialization with source rendered on demand, memoized
conversion checking. Multiplies with everything above. CORRECTED at
the full-arc review (2026-07-26): the serialization and cache faces
are engineering, but the arena threads the term representation
through reader/types/checker — the deepest commitment in the
redirection (the pricing memo's finding) — so this slice gets a
GATED-SLICE protocol when it opens, not an engineering-slice
treatment. Task #62 (per-module check cache over shared
import closures) is this layer's incremental-checking face and
lands with it. Canon's content-addressing work (CANON.md) is the
existing house precedent.

**This slice now carries Arc A's rung (e) sharing mandate (user
ruling 2026-07-26; docs/archive/A3E-PRICING-2026-07-26.md = the
evidence trail).** Term sharing — the hash-consed arena and any
conversion-checking memo — arrives ONCE, here, priced with binary
serialization + content addressing + gate (d) incremental behavior
in a single design, on the replacement-basis number the next
block-chain touch owes (see the A3 verdict, §8).

**TRIGGER FIRED (2026-07-28): the D-number landed (STREAM.md §3 B1
RECORD) and the B1 ratification formally OPENS this slice's design
under the gated-slice protocol** — parallel to Arc B's B2+ rungs,
not part of that arc.

**DRAFTING SCHEDULED (2026-08-01, user ruling at the B3-close
progress review): the design note drafts IN PARALLEL WITH Arc B's
B4 rung** — paper phase only under the gated-slice protocol (no
kernel or representation code moves on the draft alone). Its charge
is the redirection's one unmeasured gate — §9 (d) local-edit
incremental behavior — plus task #62 and DC4; the standing exhibit
is B1b's composition closure (2.38B calls / 4.17GB RSS to check a
154-line file, STREAM.md B1b RECORD).

**RATIFIED (2026-08-02, user — the three forks ruled at their leans): docs/STORAGE.md is law; S1 (the per-module skip) is the OPEN slice.**
Three layers on one identity (CANON.md §7's content hash): L1 the
per-module check certificate (kernel-free, gate (d)'s first
number), L2 binary module images resolving DC4 into the canon
serialization schema (kills the measured load floor), L3 the
hash-consed arena carrying rung (e)'s mandate — deliberately last,
possibly CLOSED-DORMANT if S1+S2 drain its pool. Fork D's number
(the pricing memo's one missing measurement) landed via the B4b
record: deletion credit −32.6% calls on the direct closure,
swamping the extrapolated conversion cost. Note: "task #62" above
is github issue #7 (per-module check certificates) — the number
was stale at first writing.

## 8. Arc A — the pathfinder protocol (CLOSED: verdicts recorded,

> **Moved to [records/CERT.md](records/CERT.md) (2026-09-02, the ledger split: LAW stays here, dated RECORDS live under docs/records/ with their section numbers unchanged).** Cited as `CERT.md §…` everywhere; open records/CERT.md for 8. Arc A — the pathfinder protocol (CLOSED: verdicts recorded,.

## 9. Gates and falsification

The decision question is NOT "did 92k lines become 2k?". Required,
measured: (a) pass-constant proof structure; (b) checker work
linear in unique source+target nodes; (c) bounded peak live terms;
(d) local-edit incremental behavior — a local change must not
recheck unrelated blocks. Instruments: DAG node counts, peak live
nodes, reduction counts, RSS, incremental-recheck timing.

The architecture is materially DOWNGRADED if any of these occur:

- the generic straight-line validator theorem is itself heroic or
  checks superlinearly;
- base+patch cannot avoid materializing full state at most
  composition seams;
- approximate/large artifacts still recheck mostly-whole after
  content addressing on a local edit;
- conversion-form proof performance is unpredictable in practice
  (the implicit-search failure mode arriving by the explicit door).

On failure: STOP and redesign before any coverage compiler or
further generator learns the old dialect. That is the arc's whole
point.

## 10. Standing consequences while Arc A runs

> **RETIRED 2026-08-22 (the reset).** The post-Arc-A ORDERING below (B → §7 → C-paper → coverage → D) is no longer the authority: Arc B closed 2026-08-11, §7's S1/S2 landed 2026-08-02, and the user retired the design review's sequencing in favour of one goal — the coverage arc as #23 (shardfmt through the generic path, examples/calc = rung 1), opening after the 2026-08 prune arc (#24). The two LAWS in this section stand: no new replay-cert families anywhere; coverage emission born speaking the ratified dialect (final ratification 2026-07-28, §8). The coverage arc's opener ledger is docs/COVERAGE.md (2026-08-22); its P7 fixes the generated certificate shape (per-fn inductions in the conversion dialect) under these laws.

- **No new replay-cert families anywhere.** impgen/wasmgen/x86gen
  are FROZEN as oracle/regression sources; existing outs regenerate
  byte-identically but no new family is taught the old dialect.
- **The coverage arc does not emit.** Its design frontier
  (calls/stack, signed kinds, address policy, heap patch/framing
  algebra, the cons/match/free micro-flagship) proceeds on paper;
  its first emitted certs wait for FINAL dialect ratification
  (provisional 2026-07-26, §8 R3; final rides Arc B's
  replacement-basis measurement). Generators must be born speaking
  the ratified dialect.
- **Post-Arc-A candidates: RE-ADJUDICATED 2026-07-26 (§8 R4).** Arc
  B opens next (rung 1 = the replacement-basis measurement); Arc C's
  paper half may run alongside, its runtime half queues behind I4
  behind the emission machinery; the coverage arc unfreezes after
  B's dialect exercise; Arc D last; PARALLEL.md drafts during the
  coverage arc. AMENDED 2026-08-01 (B3-close progress review): one
  parallel design track at a time — §7's design note drafts
  alongside B4; C-paper and the coverage paper debts stay QUEUED
  until B5 is underway (B5's clause-1 adjudication is the next
  large design decision and gets the bandwidth). NOTE 2026-08-09:
  B5 CLOSED (its clause-1 question dissolved — §4's re-founding),
  so the queue condition is spent; C-paper and the coverage paper
  debts are eligible again, still one design track at a time.

## 11. Decision points

- **DC1 — witness grammar granularity per pass**: MOOT at the
  canonical-lowering tier (full-arc review 2026-07-26): witness v0
  is an arity placeholder (§4). AMENDED 2026-08-09: clause 1 was
  RE-FOUNDED as a named economy door (§4's re-founding — B5's hand
  leg landed through the uniform contract and needed no clause);
  delete-or-grow is decided if/when a SECOND expert consumer opens
  that door.
- **DC2 — `change`/`exact-conv` surface spelling**: DECIDED — FROZEN
  at the full-arc review 2026-07-26 with the §3 boundaries recorded
  (gensym-IH citation, gated-Occ counting, per-site stop sets). Step: `(change SIDE OCC TERM)` /
  `(change SIDE OCC TERM (stop F …))`, OCC in the rewrite spelling
  (`true` all / `false` first = `(at 0)`, one walk / `(at K)`); `change` FOLDS — it
  replaces occurrences of the term's normal form (under the stop set)
  by the term itself, so matching stays syntactic against the NF and a
  bare-literal NF rides the sort-gated literal walk. Closing form:
  `(exact-conv REF (INST …) (PROOF …))` /
  `(exact-conv REF (INST …) (PROOF …) (stop F …))` — full
  instantiation mandatory, premise sub-proofs discharge like
  rewrite-with obligations, both equations normalized under the one
  stop set and compared per side. Pins:
  pins/proof/conv_probe.shard / conv_rejects.shard.
- **DC3 — checkpointed-walk proof form** (the seal discipline
  promoted to kernel tactic): CLOSED-DORMANT (2026-07-28, on the
  named measured leg's evidence — Arc B rung 1): both loop seams
  (the 9.4k-line rounds loop, the 2.9k schedule loop) closed in the
  conversion dialect with NO checkpointed-walk form — the replay
  exposures died via the inversion law, and the residual bulk is
  semantic guard forks no walk form removes. The door reopens only
  on a future measured need (a loop family whose converted walks
  are still too big); nothing builds until then.
- **DC4 — cert binary serialization format**: OPEN; engineering,
  decided inside §7's slice.
