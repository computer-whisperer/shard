# v3/ — the V3 tree (FOUNDATION.md, RATIFIED 2026-09-06)

This directory is the **logical package root** of the V3 system
(`docs/FOUNDATION.md` §8.3, §12.1; `docs/LAYOUT.md` "The V3 sibling
tree"). The qualified name of `v3/A/B.shard` is `A.B` — never
`v3.A.B` — so the flip (phase 6) relocates the tree and changes no
identity. Nothing under `v3/` imports the old tree, and nothing in the
old tree imports `v3/`; the old tree is the **oracle** for ported
modules by differential runs.

## Phase 0 deliverables (FOUNDATION §12.4)

| deliverable | where | status |
|---|---|---|
| the contract ratified | `docs/FOUNDATION.md` banner | DONE 2026-09-06 |
| the Lean release pinned | below | DONE 2026-09-06 |
| the package root declared | this file; `docs/LAYOUT.md` | DONE 2026-09-06 |
| the relevance rules, the I reconstruction contract, the core-library identity policy | FOUNDATION §4.1, §7.2, §4.4 (ratified text) | DONE |
| the shared-type inventory | `INVENTORY.md` | DRAFTED 2026-09-06 (from the pinned sources; validated by T0's export) |
| the port manifest | `MANIFEST.md` | DRAFTED 2026-09-06 — labels proposed per family, the user rules |
| the translations that remain trusted during bring-up | `docs/TCB.md` "V3 bring-up" | DONE 2026-09-06 |
| K's rule inventory and procedure | `kernel/README.md` + `kernel/{prelude,name,level,expr,decl,env}.shard` | DONE 2026-09-06 — declarations with the rules as comments (user ruling); reconciled against the pinned `src/kernel`; loads under the Rust bootstrap |
| `LAYOUT.md` gains `v3/` | `docs/LAYOUT.md` | DONE 2026-09-06 |

## The pin (2026-09-06)

The pinned Lean release and the oracle tools. Every rule, name, shape
and verdict in V3 is reconciled against these revisions; a change of
pin is a dated decision recorded in `docs/records/FOUNDATION.md`.

| component | revision | role |
|---|---|---|
| Lean 4 | **v4.33.1**, commit `819816b2e0a3bf405af45ae5c7af2491d8f5bee6` (released 2026-08-21; the latest stable at pinning; installed locally under elan) | the kernel rules (K implements them exactly); `Init` (the shared mathematical types, §4.4); the naming law's source |
| `leanprover/lean4export` | `411dce7db58a` (2026-08-25) | the export producer for T0 (`Init` declaration-for-declaration) |
| `digama0/lean4lean` | `8223d223ed98` (2026-08-29) | the reconciled rule reference (additions since Carneiro 2019; what is and is not proven) |
| `ammkrn/nanoda_lib` | `05055695879d` (2026-08-25) | independent checker for differential verdicts |
| `leanprover/lean4checker` | `91a7f0e8e9df` (2026-03-25) | replay harness through Lean's own kernel (not an independent kernel) |

The tool revisions are the heads at pinning; phase 1 (T0) validates
each against v4.33.1 and records any bump here.

## Layout (grows by phase)

```
v3/README.md      this file: root, pins, phase status
v3/MANIFEST.md    the port manifest — PORT / ARCHIVE / REGENERATE per family
v3/INVENTORY.md   the shared-type inventory — imported identity, fields, view, realization
v3/kernel/        the rule inventory as declarations (phase 0); phase 1: K's fns, then ev and the loader tower
v3/meta/          phase 3: the elaborators, I, the goal graph, tactics
v3/std/           phase 3: the first library under the naming law
v3/pins/          the corpus law of the new tree (T0's hostile battery first)
```
