#!/usr/bin/env bash
# x86_diff.sh — the Probe B reality check, end to end: the model-side plan
# emitter (x86_diff_run.shard: real machine-code bytes + model-computed
# expectations) executed by the engine-side replayer (x86_diff.c), which is
# the CPU itself — it mmaps each module's bytes into an executable page and
# CALLS them. Dev-side only; nothing here is in-logic. Run from the repo
# root. Exit 0 = full agreement (the CPU conforms to the model).
set -euo pipefail
command -v cc >/dev/null || { echo "REFUSED: no cc on PATH — the differential cannot run"; exit 1; }
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
if [ -x bin/shard_eval ]; then EMIT=(bin/shard_eval); else EMIT=(./rust_bootstrap/target/release/eval); fi
"${EMIT[@]}" run examples/x86_diff_run.shard > "$TMP/plan.txt"
cc -O2 -o "$TMP/x86_diff" examples/x86_diff.c
"$TMP/x86_diff" "$TMP/plan.txt"
