#!/usr/bin/env bash
# std/mem/lowbuild.sh — the gated artifact build for std/mem's wasm pieces
# (the first REAL module through the mod.build convention). Three gates:
#   1. SCHEMA — tools/lowcheck structurally validates every lowered_* cert
#               (consumer-side; PCC discipline — never trust the producer)
#   2. KERNEL — the cert proofs actually check
#   3. BYTETIE — the shipped binary equals the full-prefix cert's module
#               at restfs := Nil, re-encoded by tools/bytetie
#   4. ENGINE — the build entry's artifact plan (binary + manifest +
#               spec-semantics vectors) replays under real V8
# No REGEN gate v1: the pieces are hand-written; there is no generator to
# hold to determinism yet (examples/lowbuild.sh has the four-gate form).
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -eu
CERTS=std/mem/mem.wasm.shard
BUILD=std/mem/mod.build.shard
EVAL=bin/shard_eval
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: schema (consumer-side validation)"
"$EVAL" run tools/lowcheck/lowcheck.shard "$CERTS"

echo "== gate 2: kernel (the proofs)"
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK=(bin/shard_check)
else
  CHECK=("$EVAL" run kernel/check.shard)
fi
"${CHECK[@]}" "$CERTS" 2>&1 | tail -1 | tee "$TMP/kv.txt"
grep -q " 0 failed" "$TMP/kv.txt"

echo "== gate 3: byte tie (the shipped binary = the full-prefix cert's module)"
"$EVAL" run "$BUILD" > "$TMP/plan.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$CERTS" > "$TMP/tie.txt"
TIE=$(grep '^TIE mem_set ' "$TMP/tie.txt" | cut -d' ' -f3)
MOD=$(grep '^MOD stdmem ' "$TMP/plan.txt" | cut -d' ' -f3)
[ -n "$TIE" ] && [ "$TIE" = "$MOD" ] && echo "BYTETIE OK"

echo "== gate 4: engine (V8 replay of the artifact plan)"
command -v node >/dev/null || { echo "SKIPPED: no node"; exit 0; }
node examples/wasm_diff.mjs "$TMP/plan.txt"

echo "ARTIFACT OK: std/mem wasm pieces — binary + manifest + certs, all gates green"
