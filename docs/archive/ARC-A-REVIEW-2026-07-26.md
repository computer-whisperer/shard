# Arc A full-arc review — evidence synthesis and decision agenda

Status: DRAFT decision material (Fable, 2026-07-26). Four independent
fresh-context reviews ran against main @ 5b77761: (R-VAL) the A1
validator architecture, code-verified; (R-KER) adversarial kernel
soundness of the A3 forms, probe-driven; (R-DIA) dialect composition +
generator-ratification evidence; (R-ARC) Arc B/C/D re-adjudication.
Every claim below was verified by a reviewer against code or archive
with file:line evidence; probes ran in the session sandbox, never in
the repo. Rulings belong to the user; recommendations are marked.

## 1. Bottom line

- **No unsoundness found anywhere in Arc A's kernel surface.** change
  and exact-conv reduce to already-trusted components (the ONE
  compute_eval normalizer, the rewrite walks, tc_check_site,
  check_premise_proofs) with gates wired in the right order; the
  new-ctor exhaustiveness sweep is clean across every Step/Proof
  case-on; the literal-sort gate holds symmetrically through change
  (probe-confirmed both directions).
- **The A1/A2/A3 verdicts substantively hold**, with a handful of
  honest-naming and bookkeeping corrections (§3) — none of which
  reverses a measured conclusion; one (the cmp_ undercount)
  strengthens the descope ruling's deletion credit.
- **The reviews converge on an ordering**: rulings first, then Arc B
  opened with the replacement-basis measurement as rung 1, §7 opening
  when that number lands, Arc C's paper half alongside, coverage
  unfreeze after B's dialect exercise, Arc D last.

## 2. Findings by decision point

### 2a. The A1 spike ruling (gates-dissolved; witness v0)

Code-verified facts (R-VAL; models/imp/probes/vx86_acc_probe.shard —
validator at 238938, theorem at 238971, witness at 232664):

- **What was built is the recompile design, verbatim**: acceptance =
  the compiler's two emission successes + 9 if-guards + the witness
  sum; G1 (vx_xleq against the recompilation) + G2 pin dst UNIQUELY —
  acceptance accepts exactly one dst per src. CERT.md §4's
  many-legal-targets property is NOT delivered; the honest name for
  this tier is "verified canonical lowering with an extensional
  acceptance check." The trust story is intact — the compiler is out
  of the TCB; soundness flows from the checked predicate through the
  generic simulation ladder.
- **Witness v0 contributes nothing**: one guard reads it (a bare Int
  sum against the recompiled statement-leg length — no per-statement
  correspondence; `(list -5 (+ len 5))` would pass); the soundness
  conclusion never mentions w; every oracle pin self-derives the
  witness from src via the validator's own function. Soundness cannot
  break on a bad witness (verified); a bad witness only rejects. It
  is a schema placeholder that LOOKS load-bearing.
- **The shape alphabet did not fully dissolve**: vx_xieq is
  deliberately reflexivity-incomplete (False-on-self for excluded
  ctors), so G1 doubles as an implicit alphabet gate on dst. And the
  validator is strictly NARROWER than compiler acceptance — the
  semantic fences (constant shift counts in [0,width), bands) are
  validator-side truths the compiler does not check; the simulation
  genuinely needs them. The adjudication concerns SHAPE gates only.
- **segf exists nowhere** (prose only). Absence costs nothing
  measurable today (pins are pass-constant regardless; whole oracle
  closure checks in 5.8s). It is the only door to dst freedom and to
  §9 gate (d) segment-local incrementality at coverage scale;
  vxg_seam is already the composition lemma factorization would need.

Options (R-VAL), with the evidence each needs:
1. **Ratify gates-dissolved; rename the tier honestly** ("verified
   canonical lowering w/ extensional acceptance"); record §4's
   relational property as deferred-not-delivered; witness v0 either
   DELETED or explicitly documented as an arity placeholder; DC1 moot
   at this tier. Evidence FOR is already on record: the deleted hand
   gate vx_sa under-approximated the compiler's real domain by 24
   live leaves (commit 42daf67) — hand grammars drift, the compiler's
   success set cannot.
2. **Build the real relational validator via segf** (per-statement
   emission fn + append law; G1 relaxes to per-segment acceptance;
   wsegs finally locates boundaries). Gate this on ONE real consumer
   demanding a non-canonical dst; nothing produces one today. Cost ≈
   a second vxg_walk-sized effort.
3. **Clause-architecture hybrid**: recompile-equality = clause 0 of a
   disjunctive acceptance; dst freedom arrives as clauses with their
   own soundness legs; witness gains a clause tag. Cheapest hedge;
   same trigger as option 2 but commitment deferred.

RECOMMENDATION (Fable): option 1 now, with option 3's door named in
CERT.md §4 as the growth path, triggered by Arc B's first
non-canonical artifact (the SHA-NI/hand-optimized leg is the natural
first consumer). Under all options the semantic fences survive.

### 2b. DC2 — the conversion-form surface spelling

R-KER verdict: **freeze-worthy; no soundness reservation.** Verified
link by link: change uses the SAME normalizer as Compute (one
definition, reduce.shard:1796); the fold direction needs no
confluence assumption (the common reduct is explicit); the type gate
runs before normalization and refuses unbound vars loudly; capture is
blocked (literal walk inserts the replacement verbatim, env always
Empty; generic walk carries the full no_bvars/no_binders guard);
exact-conv enforces full instantiation including premise-only binders
(probe: refused with the missing binder named), sort-crossing via
instantiation is blocked by tc_check_site (probes: Int expr and
negative literal at a Nat binder both refused; nonneg literal — a
genuine Nat value — correctly admitted), premise discharge follows
the RewriteWith discipline (openings substituted before discharge).
Diagnostics measured strong (failed folds render the NF; exact-conv
names missing/mis-typed binders and renders both NFs on a miss).

Four warts to RECORD at the freeze (none blocks it):
1. **Stop-set duplication**: compute fences and change stop sets are
   hand-synced per site (xconv repeats an 8-name fence 4x). An
   ambient per-steps-block fence is future QoL; the frozen syntax
   does not preclude it.
2. **exact-conv cannot cite WfInduct/SubtermInduct IHs** — their
   binders are gensyms an author cannot spell in (inst …); always
   refused, loudly. A real expressiveness boundary if a deep run
   wants conversion-closure against a strong IH. One line in §3.
3. **Occurrence counting on gated literal patterns counts only
   sort-compatible sites** (skip-not-fail) — subtly different from
   compound patterns. One sentence in the Occ doc pins it.
4. The claim-level equation join does not apply the nat-literal view
   (pre-existing, consistent with the argument-position-only rule).

### 2c. The literal-sort gate — one confirmed crossing, in ratified scope

R-KER built a working probe: a compound-pattern rewrite through a
polymorphic ctor (Pair's TVar fields = LAny) lands an Int-typed var
in packed-Nat data — structurally the banned sg_cross_both smuggle,
wrapped in Pair, and it PASSES. Assessment: this is the explicitly
ratified LAny fence (sg_unknown_ok pin) reached from the compound
side — not an inconsistency, and NOT escalatable to 0=1 (every
downstream consumer re-gates; true equations preserve values). Also
reachable via change (second probe). Two follow-ups:
- **The code comment justifying the compound exemption is WRONG**
  (checker.shard:1103-1104 claims "the head fixes every literal
  subposition's sort" — false for polymorphic heads). Reword so a
  future hardening pass doesn't trust the stronger claim.
- If the fence ever tightens, poly-ctor field positions are the
  place to start; the fence census should note rung (c)'s surface.

### 2d. Generator dialect ratification

R-DIA drafted the full 12-point spec — every element with a landed
file:line precedent (value-naming ladders; compute-to-guard→fold
walks with stop-set-as-sharing-structure; named boundary states per
segment; ∀-bound boundary values + ONE premise per seam; exactly one
fully-instantiated exact-conv per seam; patch terms at SYMBOLIC seams
only, since concrete views materialize by computation; gb_-style
code-name ladders; byte literals only at the tie; validator citation
where a family is stable). Safe composition rule, from the exercised
files: **one state representation per claim; values shared by name;
stop sets generator-emitted per site** (two stop-set-shaped collision
surfaces identified; coexistence proven at file granularity,
unproven at claim granularity).

NOT settled — must be fences in any ratification: **loops** (3 of the
block's 13 segments are loops incl. the 9.4k-line rounds seam; zero
conversion-dialect loop exercise exists), branchy code, multi-fn,
and the machine-side segment steps (the exercised seam still cites a
replay-dialect sqs_ cert; a pure-conversion generator needs a
conversion segment step on the machine side too).

RECOMMENDATION: ratify the spec PROVISIONALLY with the loop fence
explicit, final ratification riding the replacement-basis measurement
(which is exactly the loop exercise).

### 2e. A1×A2 composition

No structural obstacle at the theorem's grain: vxg_valid's conclusion
is a call-level Option-Int observable with ∀-bound Int args and
args-independent ground premises — instantiating args with patch-view
reads is legal with zero new machinery. A ~90-140-line one-session
exercise (a vxo_ pin at xq8-boundary view reads) would demonstrate it.
**The real wall is coverage**: the validator family is mem-free by
construction; every block segment loads/stores; vxg_valid cannot
replace any block-chain link until a mem-capable statement tier
lands. RECOMMENDATION: run the small exercise (cheap, closes the
recorded composition gap honestly); record the mem-tier as the
validator family's next growth rung, not an Arc A defect.

### 2f. The replacement-basis measurement, scoped (feeds B rung 1 + DC3)

R-DIA measured the cmp_ family on disk: **~40.6k lines, not the
ledger's ~26k** (the ten uniform seams ≈2.7k each PLUS the two loop
seams 9.4k/2.9k and the 1.3k tail). Deletion credit is larger than
the pricing memo assumed; so is the loop-risk share. Scope: restate
the 13 cmp_bN + the seam-consuming region of imp_x_shblock; sqs_/
sqxw_/gb_ stay; ONE new once-per-model 12-slot list-inversion law
(~500 lines) because the weld cite forces the top level UNPREMISED
(seams must survive the None fork — the ∀-y premise shape was
exercised only as a leaf). Estimate: ~3-4.5k lines replacing ~40.6k,
proof-authoring days on owned patterns, SHARD_STATS both ways. The
two loop seams are the unpriced make-or-break — which is precisely
why this measurement doubles as DC3's gate evidence.

### 2g. Ordering (R-ARC; full brief in the agent report, evidence cited)

**Arc B is the keystone**: the only candidate that composes all three
Arc A products on a real program, produces the number §7's ratified
pricing waits on, retires the deferred consumer debts (D8 observation
relation + except grammar; Runs/RunsWithin — confirmed ZERO
implementation anywhere; page-0/#65; the generic bin tail), and
tests two recorded falsification gates on silicon. §7 does NOT run
first — the law's own sequencing (§7 priced on the D-number) plus the
fact that the parse floor is a calls-axis cost the wall clock doesn't
feel; but §7 must not drift (gate (d) and #62 live there). Arc C's
paper half (policy ledger, spec-level envelope vs the wt oracle) has
zero blocked prerequisites and can run alongside now; its runtime
half queues behind I4 behind the ratified-dialect emission machinery.
Coverage unfreezes after B's dialect exercise, its four design debts
(calls/stack, signed kinds, address policy, heap framing) closing on
paper during B. Arc D last; PARALLEL.md drafts during the coverage
arc.

ORDERING RECOMMENDATION: rulings session → B (rung 1 = the
measurement) → §7 design opens when the D-number lands → C-paper
alongside → coverage unfreeze after B → D.

## 3. Corrections queue (apply on ruling; none reverses a conclusion)

1. CERT.md §8 (A2/A3 verdicts) + the pricing memo: the cmp_ family
   figure ~26k → **~40.6k on disk** (loop seams + tail were
   uncounted). Strengthens the descope's deletion credit; enlarges
   the loop-risk share; the memo's ~95M parse-credit line understates.
2. CERT.md §8 (A1 verdict): "9-guard chain" → 12 rejection paths
   (2 matches + 9 ifs + witness sum); "every straight-line
   declaration the compiler accepts" → "…that is both
   compiler-accepted and fence-passing" (the validator is strictly
   narrower; the fences sentence already says so — tighten the lead).
3. CERT.md §7 text conflict: "pure engineering, no design risk"
   vs the memo's "deepest commitment in the redirection" (ratified
   into §7's own new paragraph). Resolve in §7's text; R-ARC
   recommends the slice gets a gated-slice protocol when it opens.
4. kernel/checker.shard:1103-1104: reword the compound-pattern
   exemption comment (the "head fixes every subposition" claim is
   false for polymorphic heads; the true justification is
   coincidence with the LAny fence).
5. CERT.md §3: one line each — exact-conv vs gensym IHs; gated-Occ
   skip-not-fail counting. §4: witness v0's true status + the
   option-3 growth door (per the 2a ruling).

## 4. The rulings this session needs (in dependency order)

- **R1 (A1 spike ruling)**: option 1 / 2 / 3 of §2a. [REC: 1, with
  3's door recorded; witness v0 delete-or-document.]
- **R2 (DC2)**: freeze the spelling as-is with §2b's four warts
  recorded. [REC: freeze.]
- **R3 (generator dialect)**: ratify the §2d spec provisionally with
  the loop fence explicit, final ratification riding the
  measurement. [REC: yes.]
- **R4 (ordering)**: §2g's sequence; B opens with the measurement as
  rung 1; the composition mini-exercise (§2e) rides along. [REC: yes.]
- **R5 (corrections)**: apply §3 as one batch. [REC: yes.]
