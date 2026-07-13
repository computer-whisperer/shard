# M3, two ways: v1 vs v2

Both versions of the project proved the **same** milestone — M3, the
in-place two-pointer memory reverse equals functional `rev`, for
universal `n`, fully kernel-checked with no admitted arithmetic. So this
is not a soundness comparison; it's an **author-effort** comparison: how
much work the proof took, and *where the complexity lives*.

- **v1** = `../proving_bootstrap_test`. Kernel in Rust; M3 proofs
  authored as Rust functions that build object-language proof terms.
- **v2** = this repo. Self-hosting kernel written in narrow (Rust only
  evaluates it); M3 proofs written directly as s-expression proof
  scripts.

## The numbers

| | **v1** | **v2** |
|---|---|---|
| Proof authored in | Rust proof-term builders | s-expressions (the object language) |
| Kernel | Rust (`src/proof/check.rs`) | narrow, self-hosting |
| M3 proof code | ~3,094 lines Rust | 1,356 lines sexp (894 NCNB; ~34% comments) |
| Total M3 claims | ~65–70 | 54 |
| **Conditional arithmetic** | **1,373 lines / 41 hand-proved lemmas** | **~35 one-line certificates** |
| …how it's proved | hand double/triple induction + ex-falso + `RewriteWith`-of-IH | a list of integer multipliers, **checked** by the kernel |
| Arith automation | untrusted search; unconditional leaves only — *provably can't* do the conditionals | `farkas` decision procedure (156 NCNB, written once, reusable) |
| Loop invariant | 1 induction; S-case fans into **5 hand-enumerated index regions** | `rev_loop_mirror` (positional, 3-way + center) + reusable `untouched_below/above` |
| Heavy proofs | 4 are `#[ignore]`d (rebuild a search theory, **~45 s each**) | all check in the normal run (<1 s) |
| Admitted arithmetic axioms | none | none |

## The dominant story: conditional linear arithmetic

Both proofs need the same family of side conditions: `i<j ⟹ i<j+1`,
`i ≤ p ∧ p ≤ j`, the mirror identity `i+j−p`, disequalities `p≠i`, the
`n/2` bound, etc.

- **v1 hand-proves each one as an induction.** `arith.rs` is **1,373
  lines — 44% of all of M3.** Every conditional fact is a uniform but
  manual ritual (the walkthrough's words): *"induct the variable that
  turns a stuck premise into a refutable constructor clash, then close
  that boundary with `Absurd` (ex-falso) and apply the conditional IH
  with `RewriteWith`."* The untrusted search **structurally cannot**
  help — it has no move that emits `RewriteWith`.

- **v2 discharges each with a Farkas certificate.** The *entire* proof of
  `lt_succ_from_lt` (`p<i ⊢ p<i+1`) is:

  ```lisp
  (ByTheory 'farkas (Cert 'farkas (list 1 1)))
  ```

  The author computes integer multipliers (untrusted); the kernel checks
  that the linear combination cancels to a negative constant. The
  1,373-line hand-arithmetic block collapses to ~35 one-liners backed by
  a 156-line checker.

That single capability — moving conditional linear arithmetic from
"hand-proved by induction" to "cert-checked decision procedure" — is the
biggest user-facing win in v2.

## Where v2 took on complexity v1 *avoided* (the honest part)

It is **not** "v2 easier everywhere." Two places v2 made more work for
itself, both downstream of a *more faithful model*:

1. **Int addresses → fuel counter → `half_bound` → a kernel change.**
   v1 made indices `Nat` and recursed `rev_loop` *structurally on the
   right pointer `j`*, so termination is free and "the loop runs enough
   times" is automatic — **there is no `n/2` arithmetic at all.** v2
   chose `Int` addresses (a more honest memory model), which breaks
   structural recursion, so `rev_loop` carries a `Nat` fuel counter
   `k = half(n)`. That forced the `(j−i) ≤ 2·⌊n/2⌋` bound (`half_bound`),
   which `half_nat`'s two-at-a-time recursion made unprovable by
   single-step induction — which is *why* `Induct2` (two-step induction)
   had to be added to the kernel. v1 sidestepped that entire subtree.

2. **The list↔memory bridge "representation tax."** v1's
   association-list memory made framing nearly free (`read(swap …)`
   falls out of `simp`). v2's bridge (`dump∘load = id`, the `rev`/`rdump`
   front-back flip, the mirror traversal) cost a real grind in `Nat`/
   `Int` index reconciliation — several `lia` lemmas (`sub_sub_one`,
   `reassoc_succ`, `idx_cancel`, `idx_pred`, `cap_idx`) exist *only* to
   massage term shapes. This is TRANSFER.md's "death by a thousand
   lemmas," and v2 felt it more than v1 here.

## The TCB trade (a deliberate project choice)

- **v1** keeps *zero* arithmetic decision procedure in its trusted base —
  all arithmetic is user-space hand-proof + untrusted search. Smaller
  trusted surface, but the **user pays 1,373 lines**.
- **v2** put `farkas` (156 NCNB) + `ord`/`lia`/`eqdec` and `Induct2`
  (~90 NCNB) into the trusted kernel (now 1,823 NCNB total). Larger
  trusted surface, but the **user pays one-liners**, and each addition is
  reviewed once and reused forever.

Both keep proof-*finding* untrusted and proof-*checking* trusted; both
are fully kernel-checked. v2 simply moved the leverage point: a bit more
audited kernel in exchange for dramatically less per-proof labor.

## Verdict

For the M3 author, v2 is **substantially easier on the part that
dominated v1** (conditional arithmetic: ~1,400 lines → ~35 one-liners)
and a **wash-to-slightly-harder** on the parts v2's design choices
created (the fuel-counter / `half_bound` / `Induct2` subtree, and the
index-reconciliation tax of an honest `Int`-addressed bridge). Net, v2's
M3 is roughly **half the proof code** with the painful 44% arithmetic
block essentially eliminated.

The substrate investments — Farkas certificates, the `ord`/`eqdec`
reflection backends, and now `Induct2` — earned their keep, at the cost
of a larger but reusable trusted kernel. The remaining v2 friction is
concentrated in **representation-alignment plumbing** (the index-shape
`lia` lemmas) and **proof reuse** (small lemmas re-proven per file for
standalone checkability) — which point at the next round of
quality-of-life work.
