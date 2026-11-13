# The port manifest (phase 0 draft, FOUNDATION §12.2)

Every file gets one label before any agent touches it. Labels: **PORT**
(the file's statements and proofs migrate under §10.2's classes; its
proofs become I), **ARCHIVE** (stays in history; not ported; small
regression fixtures retained), **REGENERATE** (generated certificate
text; replaced by validator proofs and tactic-emitted I). Toolchain
families are **new code** (§10.1) — their theorems and tests PORT, their
implementations do not. Counts are `.shard` files / lines at `28809ec`
(2026-09-06). **Labels marked (proposed) are the user's call per
family; the rest follow the contract.**

| family | files | lines | label | note |
|---|---|---|---|---|
| `kernel/` | 39 | 35,224 | **DELETED 2026-11-13, slice 5.0** (`LANGUAGE.md` §8.7; the tag `v2-kernel-last` holds it — 37 files, 26,914 lines by then); was: new code; `facts.shard`'s 15 axioms PORT as theorems (phase 3); `proof.shard`'s Step roster is the I vocabulary's source | §3, §7 |
| `meta/` | 38 | 17,704 | new code onto the goal graph (§7.3); theorems PORT. **`meta/format` (3 files, 2,046 lines) migrated with `tools/shardfmt` at slice 5.1 and deleted** — the formatter's core is `v3/tools/shardfmt/print`, in the tool's directory until a second consumer exists (the old one, `tools/prove`, is unchecked text) | §6–§7 |
| `std/list`, `order`, `nat`, `div`, `bits`, `arith`, `map` | 18 | 3,390 | PORT (phase 3, named) | the naming law; `Nat` sizes; `order`, `nat`, `div`, `arith` are records in `v3/std/README.md` and no file (slice 3.27 landing 2, 2026-10-10); `list`, `bits` are `v3/std/list.shard` and `v3/std/bits.shard` (landing 3, 2026-10-10); `map` open |
| `std/str`, `std/bytes`, `std/word` | 15 | 3,475 | PORT — typed representation change onto `String`/`ByteArray`/`UInt*` (INVENTORY.md) | §10.3 |
| `std/mem` | 7 | 2,198 | PORT | the counted heap's substrate |
| `std/rat`, `std/f32`, `std/f64`, `std/float` | 16 | 38,005 | PORT — floats as their own line (phase 5) | `FLOATS.md` kept; #39 (f64 = f32 substituted) resolved by the port, not copied |
| `std/rng/rng.shard` + req | 3 | ~100 | PORT | |
| `std/rng/rng.wasm.shard` | 1 | 11,115 | REGENERATE | a lowered twin |
| `std/sha256/sha256.shard`, `mod.req`, `.imp` | 3 | ~30,000 | PORT (the spec and its imp refinement) | `.imp` is hand-authored lowering-form text: PORT its statements, REGENERATE its certificates |
| `std/sha256/impgen_wasm_out`, `impgen_x86_out` | 2 | 188,625 | REGENERATE | Theorem A's computed value replaces impgen output (#23) |
| `std/sha256/*weld*`, `shani*`, `xchain` | 8 | ~45,000 | ARCHIVE (proposed) | Arc B's hand-authored fast path and dispatch; the flagship binary is re-derived through the generic path or re-authored later; its measured result stays in `STREAM.md` |
| `models/imp/imp.shard` + core | 4 | 4,697 | PORT | the neutral dialect; the call tier's closure of `tb_len` at slice 4.2 (`v3/models/imp`, the evaluator one function over a task type), the rest phase 5 |
| `models/imp/probes/vx86_acc_probe`, `fra_kit`, `rth_kit`, `rth_inst` | 4 | ~495,000 | REGENERATE | Theorem A and C2b's laws: the **statements** PORT (records/COVERAGE.md), the certificate kits are tactic-emitted I |
| `models/imp/probes/tb_kit`, `tb_micro`, `tbh_kit` | 3 | 16,753 | PORT | phase 4's `tb_len` rung inputs — slice 4.2: the definitions ported at landing 0, the laws rewritten by need at landing 1 (`v3/models/imp/{word,cell,step}.shard`, the run lemmas in `len.shard`: 55 theorems against the probes' 155 claims — 51 since 2026-11-10, four run lemmas uncited once the others run the evaluator, `docs/records/REDUCE-2026-11-10.md`) and landing 2 (`ghost.shard` 49, the word kit's parity and bound, the store law; `rtlaw.shard` the runtime's six laws as parameters standing for `rth_kit`'s 278 claims until phase 7; `app.shard` the allocating member, 489 lines against the old 1,148, no twin); the rest of the probes stay with phase 5) |
| `models/imp/probes/*` (rest) | ~16 | ~8,000 | PORT (smoke and de-risk probes) | |
| `models/x86` | 34 | 40,469 | PORT | the flagship target |
| `models/linux` | 2 | 306 | PORT | the syscall boundary |
| `models/wasm` | 34 | 12,445 | ARCHIVE (proposed) | slimming-census candidate; V8 replay retained as a fixture only if a consumer returns |
| `models/riscv` | 6 | 3,370 | ARCHIVE (proposed) | consumerless third target |
| `models/pio` | 6 | 5,592 | ARCHIVE (proposed) | consumerless |
| `tools/wasmgen`, `tools/x86gen` | 3 | 12,585 | ARCHIVE (proposed) | FROZEN generators; superseded by the generic path |
| `tools/impgen` (`blueprints/`, `fixtures/`) | 28 | 65,892 | ARCHIVE (proposed) | the frozen oracles; a handful of blueprints retained as PORT fixtures for the generic compiler's tests |
| `tools/impc`, `lowbuild`, `lowcheck`, `bytetie`, `build`, `low`, `reach`, `invoke`, `image` | ~80 | ~30,000 | new code where toolchain — phase 7, with the per-function generator (amended 2026-10-16, `v3/LANGUAGE.md` §13 item 69), its first generation compiled through slice 4.4's bridge; their theorems and fixtures PORT | the lowering-side toolchain; certificates become I |
| `tools/lower`, `tools/codegen` | 2 | 2,013 | **DELETED 2026-11-13, slice 5.0** — replaced by V3's own back end in their design (`v3/kernel/lower.shard`, `v3/kernel/cgen.shard`; `rt.h` moved to `v3/kernel/`), unproven; the proven lowering replaces that at phase 7 (law §12.4 as amended 2026-11-13; was: carried through the flip) | the temporary chain |
| `tools/search` | 117 | 40,569 | new code onto §7.3–7.4; the LS-law fixtures PORT | the engine |
| `tools/prove`, `tools/explain`, `tools/canon`, `tools/digest` | 14 | 10,685 | new code (I producer; goal-state renderer; the S rule set; the map instrument) | |
| `tools/shardfmt` | 3 | 989 | **MIGRATED 2026-11-13, slice 5.1 landing 1** (`LANGUAGE.md` §8.7) to `v3/tools/shardfmt` — the program, its binary and its test; the old files deleted (the tag `v2-kernel-last` holds them). Carried as fixtures: the nine scenarios. Not yet carried: the contract as theorems (`fmt_gate`, `fmt_main_contract` — the slice's landing 2); the scenarios as theorems and the printer's termination (they wait on mutual recursion compiled into L). Was: PORT | the goal's flagship (#23) |
| `pins/` | 118 | 6,052 | PORT | corpus law; `pins/trust` seeds T0's hostile battery |
| `examples/calc` | — | 3,130 | PORT — the program half (lexer, parser, evaluator, show, the app's step; 51 fns, 9 types, ~1,000 lines) **PORTED 2026-09-13 (slice 6) to `v3/examples/calc/`, one S file per old file, byte-identical to the old tree over the differential's input set**; the 100 claims (~2,100 lines) at phase 3 with tactics (ruled 2026-09-07) | `examples/calc/calc_differential.shard` is the old side's pure driver module; **slice 4.3 landing 1 (2026-10-16): the first `bin`, `calc_bin.shard` — three observables, nine bolts, two requirements proven (`main_output` through the model and 3.21's claim, `main_reads`); landing 2: its trace tied to the model's, the bolts on a model host; route 1's binary blocked at the chain (phase 5)** |
| `examples/sha256sum` (the bin, `mod.req`, stream) | ~5 | ~10,000 | PORT | |
| `examples/sha256sum/*_dispatch_x86`, `*_shani_x86` | 2 | 58,208 | ARCHIVE with the weld family (proposed) | |
| `examples/snake_game_3`, `addw`, `add`, `io`, `modules_demo`, `req_*`, `ledger_dep`, root demos | ~30 | ~11,000 | PORT | the teaching corpus; T9's material |
| `examples/snake_game` (v1) | — | 676 | ARCHIVE (proposed) | superseded by `snake_game_3` |
| `rust_bootstrap/` | — | (Rust) | carried — the enduring bootstrap facility (§9.1); its parser profile is fixed by §9.2 | |
| `bin/`, `run_corpus.sh`, `.gitlab-ci.yml`, `fails-base.txt` | — | — | the old kernel's harness (`run_corpus.sh`, `fails-base.txt`, `gate_sweep.sh`, `bin/check`, `bin/rebuild.sh`, `bin/engine_stamp.sh`) and its CI jobs **deleted 2026-11-13, slice 5.0**; `bin/ci_docs_only.sh` and the V3 jobs stay | §10.5 |

**Since 2026-11-13 (law §12.4 amended; `LANGUAGE.md` §8.7):** the old kernel is deleted and every family below that is still in the tree is unchecked text — nothing loads it, nothing gates it — until it is taken up; a family's label is decided then, and its old files are deleted when its migration is green. The labels above are the phase-0 proposals.

Totals at drafting: ≈1.22M `.shard` lines; the REGENERATE rows alone
are ≈700k of them and the proposed ARCHIVE rows ≈200k, so the PORT
load is ≈300k lines, of which ≈40k are `models/x86` and ≈38k floats.
