#!/usr/bin/env bash
# lowbuild.sh — the P4b ARTIFACT CAPSTONE: the mod.build convention end to
# end for examples/lowergen_src.shard. Five gates:
#   1. REGEN     — the default generator's cert file is byte-identical to
#                  the committed one (weld discipline; producer honesty)
#   2. SCHEMA    — tools/lowcheck structurally validates every cert
#                  (consumer-side; PCC discipline — never trust the producer)
#   3. KERNEL    — the cert proofs actually check
#   4. BYTETIE   — tools/bytetie re-encodes each cert's module literal at
#                  restfs := Nil and diffs against the plan's MOD bytes,
#                  and the manifest's ARTIFACT lines bind name -> cert ->
#                  pinned export index (tools/lowcheck/manifest.shard)
#   5. ENGINE    — the build file's artifact plan (binaries + manifest +
#                  source-semantics vectors) replays under real V8
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -euo pipefail
SRC=examples/lowergen_src.shard
OUT=examples/lowergen_out.shard
EVAL=bin/shard_eval
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism)"
"$EVAL" run tools/lowergen/lowergen.shard "$SRC" "$TMP/certs.raw" >/dev/null
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/certs.raw" > "$TMP/certs.fmt"
diff -q "$TMP/certs.fmt" "$OUT" && echo "REGEN OK (byte-identical)"

echo "== gate 2: schema (consumer-side validation)"
"$EVAL" run tools/lowcheck/lowcheck.shard "$OUT"

echo "== gate 3: kernel (the proofs)"
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK=(bin/shard_check)
else
  CHECK=("$EVAL" run kernel/check.shard)
fi
"${CHECK[@]}" "$OUT" 2>&1 | tail -1 | tee "$TMP/kv.txt"
grep -q " 0 failed" "$TMP/kv.txt"

echo "== gate 4: byte tie (cert module literals re-encode to the shipped bytes)"
"$EVAL" run tools/lowbuild/lowbuild.shard examples/lowergen_src.build.shard > "$TMP/plan.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$OUT" > "$TMP/tie.txt"
grep '^MOD ' "$TMP/plan.txt" | sort > "$TMP/mods.txt"
sed 's/^TIE /MOD /' "$TMP/tie.txt" | sort > "$TMP/ties.txt"
diff "$TMP/ties.txt" "$TMP/mods.txt" && echo "BYTETIE OK"
"$EVAL" run tools/lowcheck/manifest.shard "$TMP/plan.txt" models/wasm/wasm.shard "$OUT"

echo "== gate 5: engine (V8 replay of the artifact plan)"
command -v node >/dev/null || { echo "REFUSED: no node — the ENGINE gate cannot run"; exit 1; }
node examples/wasm_diff.mjs "$TMP/plan.txt"

echo "ARTIFACT OK: binaries + manifest + certs, all five gates green"
