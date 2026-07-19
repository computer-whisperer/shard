# CERT.md — the certificate architecture: conversion, sharing, validators

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
conversion checking. Pure engineering, no design risk, multiplies
with everything above. Task #62 (per-module check cache over shared
import closures) is this layer's incremental-checking face and
lands with it. Canon's content-addressing work (CANON.md) is the
existing house precedent.

## 8. Arc A — the pathfinder protocol (OPEN)

Three measured variants, serial on main, in this order:

- **A1 — the validator pilot** (library-only; no kernel, no
  canon-owned files). `valid_imp_x86` on the SMALLEST STRAIGHT-LINE
  imp family first; one generic soundness theorem; the landed
  impgen certs as comparison oracle. The block leg is the second
  data point ONLY after the straight-line theorem is clean.
- **A2 — base+patch** (library-only). The observation layer +
  collapse theorem on the same evaluator, exercised on the sha
  block leg (the worst case we own).
- **A3 — conversion forms** (the only kernel-touching variant).
  `change`/`exact-conv` on hash-consed terms, on the block leg.
  The kernel commitment is gated on A3's OWN numbers; A1/A2 carry
  no kernel risk and their verdicts stand independently.

Prediction on record (review consensus): conversion + DAG storage
gives the quickest 10-50x representation win and kills most weld
glue; base+patch prevents the next program from recreating
quadratic symbolic states; validators are the change that collapses
per-program proof structure to one checked pass boundary.

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

- **No new replay-cert families anywhere.** impgen/wasmgen/x86gen
  are FROZEN as oracle/regression sources; existing outs regenerate
  byte-identically but no new family is taught the old dialect.
- **The coverage arc does not emit.** Its design frontier
  (calls/stack, signed kinds, address policy, heap patch/framing
  algebra, the cons/match/free micro-flagship) proceeds on paper;
  its first emitted certs wait for the representation verdict.
  Generators must be born speaking the ratified dialect.
- **Post-Arc-A candidates live in the archive, not in law**: the
  streaming-sha flagship, ML numeric contract, parallel pilot
  (archive §"The next arcs I would actually run" and the closing
  turns). They are re-adjudicated when Arc A reports.

## 11. Decision points

- **DC1 — witness grammar granularity per pass**: OPEN; discovered
  by A1 on the straight-line family (start minimal: block
  correspondence + register choice; grow only on demand).
- **DC2 — `change`/`exact-conv` surface spelling**: OPEN until A3;
  the two-form split itself is ratified (§3).
- **DC3 — checkpointed-walk proof form** (the seal discipline
  promoted to kernel tactic): CANDIDATE, deliberately second
  priority — build only if a measured leg says generated walks are
  still too big after conversion+DAG.
- **DC4 — cert binary serialization format**: OPEN; engineering,
  decided inside §7's slice.
