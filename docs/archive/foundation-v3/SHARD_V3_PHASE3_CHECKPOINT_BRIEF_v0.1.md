<!-- The brief given to `codex exec` on 2026-10-13 for the phase-3 checkpoint memo (SHARD_V3_PHASE3_CHECKPOINT_MEMO_v0.1.md); written by Claude Fable 5.1 in the phase-2 memo's shape. -->
# Shard V3 — phase-3 checkpoint review (request)

You are reviewing the repository at commit `bea43451a653e46f0e7dba86b567a58723dca23c` (branch main of this checkout; do not check out anything else). You are the outside reviewer in the role GPT-6 has held for this project since its foundation: previous memos answered in `docs/records/FOUNDATION.md` §4 (R1–R77; the last checkpoint memo, R49–R54, is `docs/archive/foundation-v3/SHARD_V3_PHASE2_CHECKPOINT_MEMO_v0.1.md` — read it first for the shape and the standard of evidence expected). This is the phase-3 close-out checkpoint. Continue the numbering: your items are **R78 onward**.

## Read first, in this order

1. `docs/FOUNDATION.md` — the law (§12.4 the phases; §9.1 the routes; §5.4 the authoring gate; §10.5 the doc disposition).
2. `v3/LANGUAGE.md` — the design as phase 3 built it (the status block; §8.4 the per-facility current statements; §8.5; §11 deferred by phase; §12 the v2 ledger; §13 the ratified decisions, items 1–64).
3. `docs/records/FOUNDATION.md` §9 — the build chronology, the phase-3 close-out ledger (search "Phase 3 close-out ledger"), and §9.1 the verbatim slice texts of slices 3.13–3.29 that §8.4's current statements were compressed from at slice 3.30 landing 3.
4. `docs/LEAN.md` (the author-facing rows), `docs/records/T9-2026-10-13.md` (the second authoring-gate run, the author's report verbatim), `v3/README.md` ("Phase 3 status" and "Phase 4 status").
5. The kernel: `v3/kernel/*.shard` (the K seal is `v3/kernel/k/`, one module behind `k/mod.req.shard`), the pins under `v3/pins/`, the suite `v3/test.sh` and `v3/kernel/test/*.sh`.

## What is asked

Produce a memo in the shape of the phase-2 checkpoint memo: version 0.1; baseline the commit above; evidence limits stated (what you traced in source vs what you executed); a priority map; then items R78… each with the observation, the evidence as `path:line` permalinks at the commit, the requested change (contract or implementation), the regression that would pin it, and the completion gate. Mark each item with its kind: **defect** (source-traced), **contract gap**, **drift** (a document and the code or the record disagree), **risk**.

Focus, in priority order:

A. **Soundness and authorization in the kernel as it stands.** Anything on the admission path (K's checks, the seal, the loader's fork/merge for views, `fulfills`, `realize`, the store's `verify_release`) where a wrong or forged input could be accepted. Trace it in source; say what you did not run.

B. **The consolidation (slice 3.30 landing 3).** `v3/LANGUAGE.md` §8.4's current statements are new prose, a compression written from the slice texts now at records §9.1. Where a current statement and the record disagree, or a current statement states a rule the kernel does not enforce (or enforces differently), report it as drift with both locations. Sample at least three facilities fully (the Definitions section 3.13–3.16, the I section 3.20–3.21b, and one more of your choice) against §9.1 and against the kernel.

C. **The phase-3 close-out ledger against law §12.4 item 3 and §12.5.** Is every item genuinely covered by the cited evidence? Is any "covered as ruled" row a reading you would not accept?

D. **The proposed phase-4 opener, before its design is written.** The plan: extend route 1 (the v2 lowering chain `tools/lower` → `tools/codegen` → cc, booted by `bin/shard_eval`, which today compiles `v3/kernel/t0.shard` and `v3/kernel/initindex.shard`) over the kernel's five drivers and nineteen test harnesses, through a V3 pre-pass that emits plain-E copies of the kernel sources (records expanded to constructor applications and accessor functions under V3's names, capitalised locals renamed) and a byte-tie gate comparing each suite entrypoint's output under the Rust bootstrap with the native binary's. The Rust bootstrap stays the authority; `Symbol`'s `String` flip stays carried (its ruling ties it to "route 1's chain is V3's own", which this is not). What would you want in the design, what would you refuse, and what regression would you demand before the suite runs on the binaries?

E. **Trajectory.** With phases 4–7 as the law states them, what in the current tree will cost the most later if not changed now? Be specific and bounded; this is not an invitation to reopen the foundation.

## Constraints on your work

- Read-only. Do not modify files, do not create branches or commits.
- Do not run `v3/test.sh`, `run_corpus.sh`, `v3/t0_full.sh`, `v3/export.sh` or `v3/build.sh`; they take 20 minutes to hours and are CI's. You may run `./rust_bootstrap/target/release/eval direct v3/kernel/load.shard --root v3 --init "$EXPORT" --init-index "$INDEX" FILE` on single small files (a pin's `main.shard`, a file under `v3/examples/`) with `read EXPORT INDEX < <(v3/kernel/test/init_index.sh)` first; each such run takes 20–40 s. Say in the memo which runs you made.
- Every observation about source carries a `path:line` reference at the commit. Distinguish what you executed from what you read.
- Length: as long as the evidence needs and no longer. The phase-2 memo is the model.

Write the memo as your final message, as Markdown, starting with the title line `# Shard V3 — Phase-3 Checkpoint Memo`.
