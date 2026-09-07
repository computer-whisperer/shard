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
| `kernel/` | 39 | 35,224 | new code; `facts.shard`'s 15 axioms PORT as theorems (phase 3); `proof.shard`'s Step roster is the I vocabulary's source | §3, §7 |
| `meta/` | 38 | 17,704 | new code onto the goal graph (§7.3); theorems PORT | §6–§7 |
| `std/list`, `order`, `nat`, `div`, `bits`, `arith`, `map` | 18 | 3,390 | PORT (phase 3, named) | the naming law; `Nat` sizes |
| `std/str`, `std/bytes`, `std/word` | 15 | 3,475 | PORT — typed representation change onto `String`/`ByteArray`/`UInt*` (INVENTORY.md) | §10.3 |
| `std/mem` | 7 | 2,198 | PORT | the counted heap's substrate |
| `std/rat`, `std/f32`, `std/f64`, `std/float` | 16 | 38,005 | PORT — floats as their own line (phase 5) | `FLOATS.md` kept; #39 (f64 = f32 substituted) resolved by the port, not copied |
| `std/rng/rng.shard` + req | 3 | ~100 | PORT | |
| `std/rng/rng.wasm.shard` | 1 | 11,115 | REGENERATE | a lowered twin |
| `std/sha256/sha256.shard`, `mod.req`, `.imp` | 3 | ~30,000 | PORT (the spec and its imp refinement) | `.imp` is hand-authored lowering-form text: PORT its statements, REGENERATE its certificates |
| `std/sha256/impgen_wasm_out`, `impgen_x86_out` | 2 | 188,625 | REGENERATE | Theorem A's computed value replaces impgen output (#23) |
| `std/sha256/*weld*`, `shani*`, `xchain` | 8 | ~45,000 | ARCHIVE (proposed) | Arc B's hand-authored fast path and dispatch; the flagship binary is re-derived through the generic path or re-authored later; its measured result stays in `STREAM.md` |
| `models/imp/imp.shard` + core | 4 | 4,697 | PORT | the neutral dialect |
| `models/imp/probes/vx86_acc_probe`, `fra_kit`, `rth_kit`, `rth_inst` | 4 | ~495,000 | REGENERATE | Theorem A and C2b's laws: the **statements** PORT (records/COVERAGE.md), the certificate kits are tactic-emitted I |
| `models/imp/probes/tb_kit`, `tb_micro`, `tbh_kit` | 3 | 16,753 | PORT | phase 4's `tb_len` rung inputs |
| `models/imp/probes/*` (rest) | ~16 | ~8,000 | PORT (smoke and de-risk probes) | |
| `models/x86` | 34 | 40,469 | PORT | the flagship target |
| `models/linux` | 2 | 306 | PORT | the syscall boundary |
| `models/wasm` | 34 | 12,445 | ARCHIVE (proposed) | slimming-census candidate; V8 replay retained as a fixture only if a consumer returns |
| `models/riscv` | 6 | 3,370 | ARCHIVE (proposed) | consumerless third target |
| `models/pio` | 6 | 5,592 | ARCHIVE (proposed) | consumerless |
| `tools/wasmgen`, `tools/x86gen` | 3 | 12,585 | ARCHIVE (proposed) | FROZEN generators; superseded by the generic path |
| `tools/impgen` (`blueprints/`, `fixtures/`) | 28 | 65,892 | ARCHIVE (proposed) | the frozen oracles; a handful of blueprints retained as PORT fixtures for the generic compiler's tests |
| `tools/impc`, `lowbuild`, `lowcheck`, `bytetie`, `build`, `low`, `reach`, `invoke`, `image` | ~80 | ~30,000 | new code where toolchain; their theorems and fixtures PORT | the lowering-side toolchain; certificates become I |
| `tools/lower`, `tools/codegen` | 2 | 2,013 | ARCHIVE at the flip | the temporary chain; route 1 replaces it (§9.1) |
| `tools/search` | 117 | 40,569 | new code onto §7.3–7.4; the LS-law fixtures PORT | the engine |
| `tools/prove`, `tools/explain`, `tools/canon`, `tools/digest` | 14 | 10,685 | new code (I producer; goal-state renderer; the S rule set; the map instrument) | |
| `tools/shardfmt` | 3 | 989 | PORT | the goal's flagship (#23) |
| `pins/` | 118 | 6,052 | PORT | corpus law; `pins/trust` seeds T0's hostile battery |
| `examples/calc` | — | 3,130 | PORT (phase 2, named) | |
| `examples/sha256sum` (the bin, `mod.req`, stream) | ~5 | ~10,000 | PORT | |
| `examples/sha256sum/*_dispatch_x86`, `*_shani_x86` | 2 | 58,208 | ARCHIVE with the weld family (proposed) | |
| `examples/snake_game_3`, `addw`, `add`, `io`, `modules_demo`, `req_*`, `ledger_dep`, root demos | ~30 | ~11,000 | PORT | the teaching corpus; T9's material |
| `examples/snake_game` (v1) | — | 676 | ARCHIVE (proposed) | superseded by `snake_game_3` |
| `rust_bootstrap/` | — | (Rust) | carried — the enduring bootstrap facility (§9.1); its parser profile is fixed by §9.2 | |
| `bin/`, `run_corpus.sh`, `.gitlab-ci.yml`, `fails-base.txt` | — | — | carried; a V3 job per gate from phase 1 | §10.5 |

Totals at drafting: ≈1.22M `.shard` lines; the REGENERATE rows alone
are ≈700k of them and the proposed ARCHIVE rows ≈200k, so the PORT
load is ≈300k lines, of which ≈40k are `models/x86` and ≈38k floats.
