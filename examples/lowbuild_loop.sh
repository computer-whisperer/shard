#!/usr/bin/env bash
# lowbuild_loop.sh — the four-gate build for lowergen's LOOP fragment
# (examples/lowergen_loop_src.shard: Nat-counted tail recursions lowered
# to counter-as-local wasm loops with MACHINE-WRITTEN induction workers;
# docs/LOWERING.md §6h). Same gates as lowbuild.sh:
#   1. REGEN     — the generator's cert file is byte-identical to the
#                  committed one (producer determinism)
#   2. SCHEMA    — tools/lowcheck structurally validates every cert
#   3. KERNEL    — the machine-written inductions actually check
#   4. ENGINE    — the artifact plan (binaries + spec-semantics vectors)
#                  replays under real V8
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -eu
SRC=examples/lowergen_loop_src.shard
OUT=examples/lowergen_loop_out.shard
EVAL=bin/shard_eval
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism)"
"$EVAL" run tools/lowergen/lowergen.shard "$SRC" "$TMP/lowergen_loop_out.shard" >/dev/null
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/lowergen_loop_out.shard" > "$TMP/out.fmt"
diff -q "$TMP/out.fmt" "$OUT" && echo "REGEN OK (byte-identical)"

echo "== gate 2: schema (consumer-side validation)"
"$EVAL" run tools/lowcheck/lowcheck.shard "$OUT"

echo "== gate 3: kernel (the machine-written inductions)"
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK=(bin/shard_check)
else
  CHECK=("$EVAL" run kernel/check.shard)
fi
"${CHECK[@]}" "$OUT" 2>&1 | tail -1 | tee "$TMP/kv.txt"
grep -q " 0 failed" "$TMP/kv.txt"

echo "== gate 4: engine (V8 replay of the artifact plan)"
command -v node >/dev/null || { echo "SKIPPED: no node"; exit 0; }
"$EVAL" run examples/lowergen_loop_src.build.shard > "$TMP/plan.txt"
node examples/wasm_diff.mjs "$TMP/plan.txt"

echo "ARTIFACT OK: loop-fragment binaries + manifest + certs, all four gates green"
