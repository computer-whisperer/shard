#!/usr/bin/env bash
# wasm_diff.sh — the ISA slice-4 reality check, end to end: the model-side
# plan emitter (wasm_diff_run.shard: real .wasm bytes + model-computed
# expectations) piped to the engine-side replayer (wasm_diff.mjs, node/V8).
# Dev-side only — this exercises the "engine conforms to model" trust leaf;
# nothing here is in-logic. Run from the repo root. Exit 0 = full agreement.
set -eu
command -v node >/dev/null || { echo "SKIPPED: no node on PATH"; exit 0; }
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
if [ -x bin/shard_eval ]; then EMIT=(bin/shard_eval); else EMIT=(./rust_bootstrap/target/release/eval); fi
"${EMIT[@]}" run examples/wasm_diff_run.shard > "$TMP/plan.txt"
node examples/wasm_diff.mjs "$TMP/plan.txt"
