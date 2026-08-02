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
full-arc review COMPLETE 2026-07-26)

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

**A1 VERDICT (2026-07-19; run as a validation spike per the user
ruling of the same date — the full-arc review remains a later user
decision point).** `vxg_valid` proven: `valid_imp_x86 = True`
entails unpremised total machine/imp equality over every
straight-line declaration that is both compiler-accepted and
fence-passing (recompile design; 12 rejection paths — two emission
matches, 9 guards, the witness sum;
models/imp/probes/vx86_acc_probe.shard).
All 14 straight-line impgen pins re-derived by citation with ZERO
execution replay, a pass-constant ~8-line skeleton per pin
(vx86_oracle_probe.shard); a 10..400-statement size ladder proves
the same statements by compute-both replay AND by citation.
Measured: gate (a) CONFIRMED — the generic theorem is a linear
citation ladder, per-program proofs are pass-constant. Gate (b) is
UNDISCRIMINATED on this family: at ≤400 straight-line statements
both dialects check inside the ~3.3s file-load floor; the
discriminating measurement lives on the block leg (A2) or needs
reduction-count/RSS instrumentation the engine does not yet expose
(a §9 instruments line item). Coverage fences on record: single-fn,
ISet-only, mem-free, banded programs; branchy pins (sel/selq/clamp2)
excluded; witness v0 consistency-checked only (segment-local check +
segf factorization deferred, as priced). **A2 OPENED 2026-07-19
(user ruling: proceed).**

**A2 VERDICT (2026-07-19; run in the same spike spirit — measured
against the landed certs, replacing nothing).** The patch algebra
landed as an ordinary library: `IPatch` (PLoc/PByte/PWord), ONE
newest-first list per model, apply/view/footprint functions,
unpremised read-through laws, one generic framing law per
observation, and the istmts collapse theorems — proven once in
models/imp/probes/ipatch_probe.shard — then exercised on BOTH legs
of the block worst case. Hand leg (std/sha256/sha256.patch.shard):
the observation×writer pair-lemma PRODUCT becomes a SUM — one
patch twin + footprint pair per writer, one frame law per
observation grain (byte and window); the family's bespoke
inductions (30–500 lines each) re-derive as ~12–15-line citation
chains, and `blk_ps` names the block walk's repeated writer chain
once, with collapse = patch-append plus an occurrence-targeted
rewrite. Generated leg (std/sha256/sha256.xpatch.shard): the
cmp_x_shblock seam family — ~2200 lines of 12-deep state exposure
per seam, ~26k lines of exposure ≈ 30% of the 92k x86 out file
(CORRECTED 2026-07-26: the family on disk is ~40.6k lines — the two
loop seams, 9.4k + 2.9k, and the 1.3k replay tail sit on top of the
counted exposure portions) — derives at
view states by pure citation: the per-segment step cert
instantiates at the canonical view reads, xm_scont's register
rebuild COMPUTES through the literal-of-reads state, and the
suffix cert cites at the post-segment reads; zero case-ons, ~250
formatted lines per seam of which ~150 are the per-segment
instruction split the generator already owns. Segment interiors:
8 verbatim generated statements collapse to ONE named patch term
with the segment's root bounds as the only premises, every proof
leaf first-check. The §9 falsification item — "base+patch cannot
avoid materializing full state at most composition seams" — is
answered NO on every seam exercised: hand-leg phase seams are
patch-append plus footprint arithmetic; the generated-leg suffix
seam is citation at view reads. Full state is never respelled.

Structural findings for the coverage-compiler design: (1) the
read-through laws carry ONLY symbolic-patch seams — on concrete
patch lists the view materializes by computation, so segment
interiors stay compute-driven exactly like today's certs; the
dialects split cleanly at the seam boundary. (2) VALUES TELESCOPE
— base+patch does not by itself tame value-term growth; a
generator must emit value-naming ladders (the gb_ discipline it
already practices), or the sharing arrives at A3's representation
layer instead. This is the arc's live coupling: A2's residual cost
is exactly the term-sharing problem A3 prices. Gate (b) remains
structurally measured only: text volume (better than 10x on the
seam family at the demonstrated shape) and per-statement
constancy; wall clock stays inside the load floor (the 92k-file
closure 3.9s → 4.4s with the exercise file), and reduction-count/
RSS instrumentation is still the §9 instruments line item.
Coverage fences on record: straight-line ISet/IStore segments and
the sha writer family only; loop bodies as computed patch lists
(the round_deltas shape) designed but unexercised; the sched value
characterization and the full block-walk re-derivation deferred
with named interfaces (the absorption seed xhg_wget_hit landed);
the A1 validator and the patch dialect have not been composed —
independent spikes. **A2 CLOSED 2026-07-19. A3 (conversion forms,
the only kernel-touching variant) is the next decision point,
gated on its own numbers per the protocol.**

**A3 VERDICT (2026-07-26; rungs (a)–(d) landed 2026-07-19/20, rung
(e) ruled on the pricing memo).** The conversion forms landed at the
smallest kernel commitment in the redirection: (a) SHARD_STATS
instruments (calls/allocs/live-peak/RSS + per-fn counters; the first
instrumented build surfaced and fixed the GC stack-base soundness
bug); (b) the literal-sort rewrite gate — the packed-Nat/Int atom
hazard now refused, zero existing proofs broken; (c) naive
`change`/`exact-conv` — reduction-based, zero new axioms, explicit
occurrence + stop-set spelling so implicit search stays shut out
(accept pins 9/0 first try, reject pins 5/5 with exact diagnostics);
(d) the conversion leg on A2(d)'s objects — the suffix seam closes by
ONE fully-instantiated exact-conv, per-seam text 2695 (replay) → 243
(patch) → 106 (conversion), marginal checker cost +6.7M calls vs the
patch leg's +24.2M ≈ 3.6x, reproduced exactly on post-kernel-survey
main. The instruments' structural finding: the closure's bill is
parse (~31%) plus env/name/type traffic (~40%); proof-step machinery
sits at the bottom of the per-fn table — source-text shrink IS
checker-work shrink, and an evaluation memo cannot reach the dominant
costs. **Rung (e) DESCOPED TO §7 (user ruling 2026-07-26): the
late-fold recompute is bounded by the whole conversion marginal
(≈0.65% of the closure) at exercised scale; the hash-consed arena is
§7's slice and lands there once.** Owed forward: the
replacement-basis measurement — the block chain re-derived in the
conversion dialect WITHOUT the cmp_ replay family in the closure —
falls out of the next block-chain touch (or the coverage compiler's
first family) and doubles as DC3's gate evidence. Gate (b) status:
text 10–25x on the exercised family, marginal calls 3.6x below
patch, replacement basis owed. Gate (d): explicitly deferred to §7.
Coverage fences: one segment sampled (8 of ~23 statements) + one
suffix seam; the full 13-seam chain re-derivation deferred with
named interfaces. **A3 CLOSED 2026-07-26 (task #74). Arc A's rungs
are complete; the full-arc review — the A1 spike ruling, DC2 final
adjudication, the generator-freeze dialect ratification, Arc B/C/D
re-adjudication — is the next user decision point.**

**FULL-ARC REVIEW COMPLETE (2026-07-26; four independent
fresh-context reviews; synthesis + evidence:
docs/archive/ARC-A-REVIEW-2026-07-26.md; NO UNSOUNDNESS found in Arc
A's kernel surface — all five survey lenses clean, probe-driven).**
Rulings, all ratified: **R1** — the A1 spike ruling resolves to
gates-dissolved RATIFIED + the honest rename + the
clause-architecture growth door (recorded in §4; witness v0 = arity
placeholder; DC1 moot at this tier). **R2** — DC2 FROZEN with the
recorded boundaries (§3). **R3** — the generator dialect RATIFIED
PROVISIONALLY: the 12-point spec of the review's §2d (value-naming
ladders; named boundary states per segment; ∀-bound seam boundaries
+ exactly one exact-conv per seam; patch terms at SYMBOLIC seams
only; ONE state representation per claim, values shared by name;
generator-emitted stop sets), with the LOOP FENCE explicit — no
conversion-dialect loop exercise exists; final ratification rides
the replacement-basis measurement. **R4** — the post-Arc-A sequence:
ARC B OPENS NEXT with the replacement-basis measurement as rung 1
(scope: review §2f — the 13 cmp_bN seams + the weld-facing walk
region restated in conversion dialect, one new once-per-model
12-slot list-inversion law, the cmp_ family dropped from the
closure, SHARD_STATS both ways; the two loop seams are the
make-or-break and double as DC3's gate evidence) and the ~120-line
A1×A2 composition exercise riding along; §7's design opens when the
D-number lands; Arc C's paper half may run alongside; the coverage
arc unfreezes after B's dialect exercise; Arc D last. **R5** — the
corrections batch applied: this file (§3, §4, §7, §8, §10, §11), the
pricing memo (erratum), kernel/checker.shard's compound-exemption
comment. Known fence on record from the review: the literal-sort
gate's LAny fence is reachable through polymorphic ctor fields from
both the compound path and `change` (ratified scope, probe-verified,
not escalatable to 0=1); if the fence ever tightens, poly-ctor field
positions go first.

Prediction on record (review consensus): conversion + DAG storage
gives the quickest 10-50x representation win and kills most weld
glue; base+patch prevents the next program from recreating
quadratic symbolic states; validators are the change that collapses
per-program proof structure to one checked pass boundary.

**FINAL DIALECT RATIFICATION (2026-07-28; user ruling on the B1
record — STREAM.md §3 holds the full measurement).** The
replacement-basis measurement landed green: the 13-seam chain plus
the weld-facing walk restated at statements byte-identical to the
replay originals, 43,451 → 7,969 lines, D-number −19.3% calls /
−34% live peak, both loop seams closed, and the A1×A2 composition
exercise first-check. R3's provisional spec is RATIFIED FINAL with
one amendment from contact: **chain interiors are UNPREMISED
seams**. The seam statement is the replay statement; the proof
case-forks on the shared segment term (fail forks close by compute
because the adapters mirror failure), derives the locals arity by a
change-fold into the length law (il_slen; il_wlen at loops), mints
the slot reads with ONE once-per-model list-inversion citation
(ilv_inv12 — per-arity; the generator emits it once per model), and
closes with exactly one fully-instantiated exact-conv of the next
seam. The ∀-bound-boundary premised shape of the §2d spec remains
the LEAF/LIBRARY form only — it cannot chain, because successor
premises are undischargeable at symbolic state. The loop fence is
CLOSED: loop seams keep their approach paths (fuel reshapes, wrap
collapses, the sqxw citation) and end at the same case-fork;
guard-fork trees are the semantic floor and survive in every
dialect. Fences carried forward, named and open: machine-side
segment steps still cite replay-dialect sqs_ certs (a
pure-conversion generator owes the conversion form of the segment
step); branchy code and multi-fn remain unexercised; the committed
block closure still contains the cmp_ family until a migration
touch — the measurement's variant was a scratch artifact by
ratified scope (SLOTTED 2026-08-01: STREAM.md rung B4b — the
"next block-chain touch" trigger misfired at the M5 relocation,
which was such a touch, so the implicit trigger is retired for an
explicit rung. LANDED 2026-08-02, the B4b RECORD: the full
three-chain redirection — the replay ladder left the committed
closure AND the generator, xchain is the only block-chain
dialect, −61k generated lines, the −19.3% pricing exceeded at
−32.6% calls on the direct closure). Same-ruling consequences: DC3 CLOSED-DORMANT on the
loop-seam evidence (§11); §7's design formally OPEN under its
gated-slice protocol; the coverage arc UNFREEZES (§10's B1
condition met).

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
  large design decision and gets the bandwidth).

## 11. Decision points

- **DC1 — witness grammar granularity per pass**: MOOT at the
  canonical-lowering tier (full-arc review 2026-07-26): witness v0
  is an arity placeholder (§4); delete-or-grow is decided when
  clause 1 of the §4 clause architecture arrives with a real
  non-canonical consumer.
- **DC2 — `change`/`exact-conv` surface spelling**: DECIDED — FROZEN
  at the full-arc review 2026-07-26 with the §3 boundaries recorded
  (gensym-IH citation, gated-Occ counting, per-site stop sets). Step: `(change SIDE OCC TERM)` /
  `(change SIDE OCC TERM (stop F …))`, OCC in the rewrite spelling
  (`true` all / `false` first / `(at K)`); `change` FOLDS — it
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
