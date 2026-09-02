# CERT.md — RECORDS (moved out of docs/CERT.md on 2026-09-02)

> **STATUS: RECORD.** Dated rung/slice records split out of [../CERT.md](../CERT.md) so the LAW ledger stays readable in one sitting. Section numbers are exactly as they were in docs/CERT.md; a citation `CERT.md §N` for a section listed here resolves to this file. Nothing here is normative unless the law ledger says so; any "NEXT" pointer is history.

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

