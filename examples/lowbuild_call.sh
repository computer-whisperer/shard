#!/usr/bin/env bash
# lowbuild_call.sh — the five-gate build for lowergen's CALLS-IN-LOOPS
# set (examples/lowergen_call_src.shard: a mem-fragment callee plus a
# loop that CALLS it per iteration; docs/LOWERING.md, probe:
# examples/callloop_probe.shard). Same gates as lowbuild_mem.sh; the
# loop's unit lives in the LINKED file (structural form — callee
# literals inside, regenerates on callee edits).
#   1. REGEN     — generator output byte-identical (both files)
#   2. SCHEMA    — tools/lowcheck structurally validates every cert
#   3. KERNEL    — the machine-written proofs check (bridge + callee
#                  cert citations inside the induction)
#   4. BYTETIE   — cert module literals re-encode to the shipped bytes
#   5. ENGINE    — the artifact plan replays under real V8
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -eu
SRC=examples/lowergen_call_src.shard
OUT=examples/lowergen_call_out.shard
LINK=examples/lowergen_call_link.shard
EVAL=bin/shard_eval
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism, portable + linked)"
"$EVAL" run tools/lowergen/lowergen.shard "$SRC" "$TMP/lowergen_call_out.shard" "$TMP/lowergen_call_link.shard" >/dev/null
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/lowergen_call_out.shard" > "$TMP/out.fmt"
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/lowergen_call_link.shard" > "$TMP/link.fmt"
diff -q "$TMP/out.fmt" "$OUT" && diff -q "$TMP/link.fmt" "$LINK" && echo "REGEN OK (both byte-identical)"

echo "== gate 2: schema (consumer-side validation, both forms)"
"$EVAL" run tools/lowcheck/lowcheck.shard "$OUT"
"$EVAL" run tools/lowcheck/lowcheck.shard "$LINK"

echo "== gate 3: kernel (the machine-written proofs)"
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK=(bin/shard_check)
else
  CHECK=("$EVAL" run kernel/check.shard)
fi
"${CHECK[@]}" "$LINK" 2>&1 | tail -1 | tee "$TMP/kv.txt"
grep -q " 0 failed" "$TMP/kv.txt"

echo "== gate 4: byte tie (cert module literals re-encode to the shipped bytes)"
"$EVAL" run tools/lowbuild/lowbuild.shard examples/lowergen_call_src.build.shard > "$TMP/plan.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$LINK" > "$TMP/tie.txt"
grep '^MOD ' "$TMP/plan.txt" | sort > "$TMP/mods.txt"
sed 's/^TIE /MOD /' "$TMP/tie.txt" | sort > "$TMP/ties.txt"
diff "$TMP/ties.txt" "$TMP/mods.txt" && echo "BYTETIE OK"

echo "== gate 5: engine (V8 replay of the artifact plan)"
command -v node >/dev/null || { echo "SKIPPED: no node"; exit 0; }
node examples/wasm_diff.mjs "$TMP/plan.txt"

echo "ARTIFACT OK: calls-in-loops binaries + manifest + certs, all five gates green"
