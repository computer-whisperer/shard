# LAYOUT.md — repository layout: the placement rule and the move map

Status: RATIFIED 2026-07-18 (the repo reorganization, slices 0–4).
This is the decoder for every path in the tree — and for every
*historical* path in the ledgers, which deliberately keep their
as-landed spellings (see "The path-map policy" below).

## The placement rule

**Every file lives with its owner** — the subsystem that gives it
meaning, not the subsystem that happened to birth it:

- **A model's probes and differential harnesses live with the model.**
  `models/<isa>/probes/` holds the hand-play articles, smoke pins, and
  de-risk probes that validate the model's meaning;
  `models/<isa>/diff/` holds the engine-differential harness (the
  model-side driver plus the untrusted .c/.mjs/.sh plumbing).
- **A tool's fixtures live with the tool.** `tools/<name>/fixtures/`
  holds inputs and their generated outputs — generated files sit
  BESIDE their sources because the generators (impgen, wasmgen,
  x86gen) import the pin source *sibling-by-basename*; splitting src
  from out is a generator contract change, not a file move.
  `tools/impgen/blueprints/` holds the hand-validated articles the
  generator's emission tiers were built against.
- **Kernel-meaning pins live in `pins/`.** `pins/proof/` (proof-layer
  regressions), `pins/lang/` (language/loader semantics),
  `pins/trust/` (trust-boundary refusals). These are corpus law: the
  kernel's behavior may not drift from them silently.
- **`examples/` holds genuine programs and demonstrations** — things a
  reader would open to learn shard, not working parts of some
  subsystem's ladder. Bin programs (add, addw, sha256sum), feature
  demos, the stateful apps, and deliberate `*_rejects.shard` negative
  demos.

Qualified names are PATH-DERIVED (`tools/impgen/fixtures/imp_scalar.shard`
→ `(:: tools impgen fixtures imp_scalar …)`), so a move is a semantic
act: imports, use-lines, harness argv strings, and `fails-base.txt`
keys all move with the file. The mechanics checklist lives in the
session memory (repo-reorg-2026-07); the short version: fix generators
first, regenerate second, and grep every moved basename as a bare
string — argv references hide where import-resolvers cannot see.

## The move map (2026-07-18, slices 0–4)

| old home (examples/) | new home | what |
|---|---|---|
| kernel/proof/trust pin army (102) | `pins/{proof,lang,trust}/` | corpus-law pins |
| `x86_pieces`, `x86_window_law`, `x*_probe` army | `models/x86/probes/` | x86 model hand-plays |
| `x86_diff.{c,sh}`, `x86_diff_run` | `models/x86/diff/` | silicon differential |
| `wasm_smoke/pieces/rev/copy/weld(+out)` | `models/wasm/probes/` | wasm model articles |
| `wasm_diff.{mjs,sh}`, `wasm_diff_run` | `models/wasm/diff/` | V8 differential |
| `riscv_smoke/pieces`, `riscv_diff*` | `models/riscv/{probes,diff}/` | RISC-V twin set |
| `pio_smoke`, `pio_vec*` | `models/pio/{probes,diff}/` | PIO twin set |
| `lxkernel_probe` | `models/linux/probes/` | kernel-model pins |
| `imp_scalar/loop/mixed/if/ifl` + `impgen_*_out` (15) | `tools/impgen/fixtures/` | impgen srcs + legs (siblings) |
| `sq*_probe` (11) + `iwg_probe` | `tools/impgen/blueprints/` | tier blueprints |
| `imp_probe` + `imp_{wasm,x86}[_loop]_bridge` | `models/imp/probes/` | dialect validation articles |
| lib/bin ladder families: `purelib`, `wasmgen*`, `x86gen` + 8 x86 frag families, `arglen/bytesum/echoarg/upcase/parse` (54) | `tools/lowbuild/fixtures/` | build-ladder triplets |
| loop/frag probe army + `lowered_form`, `rep_probe`, `lowfrag_probe`, `repswap/loopgen/portcert/prehyg/callcomp/msetcomp_probe`, `libmod_probe` (23) | `models/wasm/probes/` | LOWERING.md hand-plays |
| `lowcheck_rejects`, `lib_form(_rejects)`, `manifest_rejects.txt` | `tools/lowcheck/fixtures/` | schema-gate negatives |
| `binelf_probe.{shard,sh}`, `stdin_echo_probe` | `models/x86/probes/` | X86.md de-risk probes |
| `invoke_probe`, `invoke_fixture(.auto)` | `tools/invoke/` | meta/invoke end-to-end pair |
| `build_products.shard` | `tools/build/` | the driver's product list |

Deliberately still in `examples/`: `tie_probe` (a minimal example
World app; bin/rebuild.sh's engine-parity fixture), `weld_probe`,
`bytes_bridge`, the demos and pins that ARE demonstrations
(`natview_*`, `spell_pin`, …), the app directories, and the bin
program families (add, addw, sha256sum).

## The path-map policy

Ledgers and archived records keep the paths that were true when their
slices landed — history is not respelled. Reading an old path in
X86.md, IMP.md, LOWERING.md, BUILD.md, or the archive means: consult
the move map above. Live documents (README, this file) and live code
comments track the current tree.
