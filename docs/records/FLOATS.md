# FLOATS.md — RECORDS (moved out of docs/FLOATS.md on 2026-09-02)

> **STATUS: RECORD.** Dated rung/slice records split out of [../FLOATS.md](../FLOATS.md) so the LAW ledger stays readable in one sitting. Section numbers are exactly as they were in docs/FLOATS.md; a citation `FLOATS.md §N` for a section listed here resolves to this file. Nothing here is normative unless the law ledger says so; any "NEXT" pointer is history.

## 10. Rungs and gates

The exhaustive-toy-format idiom (symfpu's validation trick) is the
arc's principal derisking device: on a toy format every claim is
checked by ground compute over ALL inputs (2^16 pairs per binary op)
before the parametric proofs are attempted, and again at L2 against
the bit patterns. Cheap, mechanical, catches spec bugs where they are
cheapest.

- **R0 — std/rat + kit.** §2. Gate: corpus diff-clean; kit demo
  consumer. **LANDED 2026-07-12** (through 26b70b4 + the interface
  lift): gcd + full theorem kit (divides pair, homogeneity, cofactor
  coprimality, universal property, Bezout-free Euclid), the opaque
  Rat (refined record pair, rat_make's canonicality a discharged
  refined-return obligation), full op set routed through the
  constructor, canonical-representation UNIQUENESS lifted to the
  interface as cross-multiplication completeness (rat_cross_num/
  rat_cross_den), consumer demo examples/rat_demo.shard. Corpus
  57-stable throughout.
- **R1 — L1 parametric model + named instances.** rnd laws
  (monotonicity, idempotence on representables, tie behavior),
  exhaustive toy-format gates, probe grids on F32/F64 samples. The
  `2^k` algebra kit (§3a tier 1) starts here. Gate: exhaustive toy
  pass + corpus diff-clean.
- **R2 — L2 bit-level model.** pack/unpack bijection (up to NaN
  class), computable ops ⊑ L1, toy-format exhaustive at the bit
  level. This is the arc's heavy proof rung (the Binary.v analogue).
- **R3a — the thin per-format surface modules** (§3a) — a HARD
  dependency of R4, not a preference: the lowering fragments match
  the surface symbols, and R4's six-gate consumer decl is written
  against them. `to_bits` canonicalization per §5 lands here.
  [Rationale AMENDED 2026-07-14 — under §4a the surfaces' defining
  equations are the spec ⊑ imp entry instead; still sequenced
  before the lowering rung. See the close-out block below.]
  **LANDED 2026-07-16** (fork `floats-r3`, three slices; formats
  f32 + f64 per user ruling, bf16 on first consumer):
  - *R3a-a — the core enters req-scope.* The surface interfaces
    must STATE defining equations in core vocabulary, and the
    req-scope import gate forbids interfaces importing impl-space
    files — so the five core files relocated to
    `std/float/mod.req/` (the std/rat/mod.req/gcd.shard precedent
    at scale; user-ruled). qname-neutral: `(:: std float float …)`
    and all sibling imports unchanged.
  - *R3a-b — the wf/roundtrip theorem layer*
    (std/float/mod.req/wf.shard): op wf-closure PARAMETRIC (§3a
    tier 1) — fadd_wf/fsub_wf/fmul_wf/fdiv_wf (premises fmt_wf +
    hnan; the hnan premise covers the NaN-creating table arms) +
    fabs_wf + helpers (fwf_zero, fval_den_pos_any, fwf_m_lo,
    fval_num_pos_probe); decode-side wf fmt-ground (tier 2):
    funpack_wf_f32/_f64 — funpack is wf on ANY Int, so from_bits
    is total with zero premises; combined roundtrip/range
    assemblies funpack_fpack_fXX + fpack_range_all_fXX over the
    R2.2 pieces.
  - *R3a-c — the surfaces.* std/f32 + std/f64: opaque
    `(refine FVal fXX_ok)` modules (str-shape: predicate exposed,
    invariant a FULFILLED requirement), ops = one-line defining
    equations exported as requirements (the spec ⊑ imp entry),
    IEEE le/lt/eq derived from fle_val (NaN-False, ±0-equal),
    is_nan classifier, bits doors with §5 canonicalization
    (from_bits total; to∘from = id off the NaN class,
    canonicalizing on it — 0x7FC00000 / 0x7FF8000000000000), bits
    range, host-IEEE-verified ground pins (incl. 0.1f+0.2f,
    sNaN-collapse, 1/-0 = -Inf, ±0 comparisons), consumer demo
    examples/float_surface_demo.shard (citation-only, the
    rat_demo shape). Gate: corpus FAIL-set == 65-line baseline,
    driver green.
- **R3b — literals + hex printing.** No downstream dependency;
  floats on consumer pain (interim: gates and probe sources
  construct values via from_bits). Expected pull-in: alongside R4's
  consumer demos, the moment hand-written bit constants get old.
  **LANDED 2026-07-16** (fork `floats-r3`, three slices; user
  rulings: pure std, NO syntax — literal constructors are ordinary
  fns and any token-level literal is a later canon/main-thread
  rung; raw integral hex; (m, e) pairs; nan/inf/neg_inf constants):
  - *R3b-a — the decimal door* (std/float/mod.req/dec.shard).
    `fdec f m e` = RNE of the exact rational m × 10^e via frnd
    (e >= 0 scales the numerator, e < 0 the denominator) — §9's
    exact-decimal pipeline with zero new rounding code and no
    host-float laundering anywhere. fpow10 (fpow2's decimal twin) +
    fpow10_pos; `fdec_wf` premises fmt_wf ONLY (frnd_wf's 0 < d
    discharges as the ground 1 or via fpow10_pos), so the
    surfaces' from_dec refine obligation is one cite. Differential
    probe vs host IEEE: 14 (m,e) points across both formats —
    normals, inexact rounding, overflow→±Inf, subnormal round-up,
    underflow→+0, big-int rounding — all bit-exact.
  - *R3b-b — the hex-exact observer* (std/float/mod.req/hex.shard).
    Raw integral-significand form `[-]0x<hex m>p<+/-><dec |e-mw|>`
    ("0xc00000p-23" is 1.5f32): the string denotes the value under
    the standard hex-float reading, the printed power adjusting the
    stored leading-bit exponent by mw (fhex takes the format for
    exactly this). Specials "nan"/"+inf"/"-inf"; zeros print their
    true exponent ("0x0p-149" at f32, sign visible on -0). §9's
    "roundtrip-trivial" substantiated at the digit layer:
    LSD-first digit-value lists (no append, no accumulator
    generalization — the calc_show representation minus std/list,
    which req-scope forbids) with PROVEN roundtrips fhexd_val /
    fdecd_val (wf-induct; div-facts as a CHAIN ITEM plants the
    euclidean rows for the measure facts and the final fold).
    ASCII assembles by right-to-left consing (prepend-render
    reverses LSD to MSD for free).
  - *R3b-c — the surfaces.* Both formats: `fXX_from_dec` (refined
    return via one fdec_wf cite), `fXX_hex`, constants
    fXX_nan/fXX_inf/fXX_neg_inf (ground-compute obligations); all
    exported as defining-equation requirements + fulfills; ground
    pins host-IEEE-verified (1.5/0.1 bits, the 0.1+0.2 classic
    literal-spelled, sign-on-m, overflow, subnormal ulp,
    underflow, hex strings incl. -0, constant bits); consumer demo
    extended (citation-only). Gate: hand-run per the ratified fork
    policy — every touched file standalone-green, fmt + canon
    clean, driver green.
- **R4 — wasm lowering.** f32/f64 fragments, tier-1 quotient
  theorems against the wasm model, tier-2 V8 gates (six-gate
  discipline per the lib arc). wasm first, per width-ordered
  coverage precedent. Conversion instructions get heavyweight edge
  vectors — historically the buggy engine surface (the basic five
  are single hardware instructions under any JIT and can't
  realistically diverge; trunc_sat can). [SUPERSEDED as spelled
  2026-07-14 — re-homed as the IMP.md I4 wasm leg; see §4a. Kept
  as the obligation inventory: the edge-vector emphasis and the V8
  gates carry over verbatim.]
- **R5 — x86 lowering.** SSE2 scalar fragments + min/max/convert
  fixups, tier-1 theorems against the x86 model, tier-2 TestFloat +
  on-CPU differential gates. [SUPERSEDED as spelled 2026-07-14 —
  re-homed as the IMP.md I4 x86 leg; see §4a. The fixup content
  and tier-2 instruments carry over verbatim.]
- **R6 — fma.** In-model from R1; this rung is the FMA3 fragment +
  wasm software path parity gate.
- **R7 — narrow formats.** Certified wide-compute per §7 + packed
  tensor demo in std/mem (the ML guard made concrete).

Flagship candidate for the arc: a dot-product / small-GEMM kernel in
BF16-in, F32-accumulate — exercises §6 packing, §7 certification,
and both targets from one decl. (Now shared: IMP.md §6 names it as
rung I4's layout flagship.)

**CLOSE-OUT SCHEDULING (RATIFIED 2026-07-14, with the §4a
amendment).** The floats fork merges at R2-complete
plus this amendment: the fork's content sits entirely below the
lowering seam, nothing on main touches std/float, and nothing
remaining is fork-shaped. The rest of the ladder re-sequences onto
main, post-merge:

- **R3a, R3b — post-merge std work on main.** Pure std/float +
  surface-module rungs with no imp interaction. R3a keeps its
  place before the lowering rung — surfaces are the consumer
  interface (static format distinction), their defining equations
  are the spec-side entry to spec ⊑ imp, and `to_bits`
  canonicalization lands there.
- **R4/R5 — restated as the IMP.md I4 joint**: float capability in
  the machine + the imp ⊑ ISA float legs, sequenced behind imp's
  v2 migration and the impgen rebuild per IMP.md's 2026-07-14
  re-sequencing. wasm still first (width-ordered coverage
  precedent); tier-2 gating unchanged.
- **R6, R7 — unchanged in order**, entering through the same imp
  seam when their turns come.

Merging also closes a dangling reference: main's ratified IMP.md
cites FLOATS.md §3a/§5/§8, which exist only on this fork until the
merge.


