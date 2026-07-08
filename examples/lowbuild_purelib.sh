#!/usr/bin/env bash
# examples/lowbuild_purelib.sh — the gated artifact build for the LIB-BUILD
# pilot (the boring library): examples/purelib_src.shard carries a (lib …)
# declaration; tools/wasmgen LIB mode lays the whole file into ONE wasm
# module with NAMED exports. Gates:
#   1. REGEN    — the generator's cert file is byte-identical to the
#                 committed one (producer determinism)
#   2. SCHEMA   — tools/lowcheck structurally validates every cert (the
#                 position-general lib form included)
#   3. KERNEL   — the machine-written proofs check, AND the source file's
#                 (lib …) acceptance holds (exports all implemented)
#   4. BYTETIE  — tools/bytetie assembles the module FROM THE CERTS (own
#                 literal at each pinned index) + the reflected export
#                 table, re-encodes via enc_lib, and the result equals the
#                 plan's MOD bytes; the manifest gate binds each ARTIFACT
#                 line name -> cert -> pinned export index
#   5. ENGINE   — the artifact plan replays under real V8, exports invoked
#                 BY NAME
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -euo pipefail
SRC=examples/purelib_src.shard
OUT=examples/purelib_out.shard
BUILD=examples/purelib_src.build.shard
EVAL=bin/shard_eval
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism)"
"$EVAL" run tools/wasmgen/wasmgen.shard "$SRC" "$TMP/purelib_out.shard" >/dev/null
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/purelib_out.shard" > "$TMP/out.fmt"
diff -q "$TMP/out.fmt" "$OUT"
echo "REGEN OK (byte-identical)"

echo "== gate 2: schema (consumer-side validation)"
"$EVAL" run tools/lowcheck/lowcheck.shard "$OUT"

echo "== gate 3: kernel (machine proofs + the lib acceptance)"
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK=(bin/shard_check)
else
  CHECK=("$EVAL" run kernel/check.shard)
fi
"${CHECK[@]}" "$OUT" 2>&1 | tail -1 | tee "$TMP/kv.txt"
grep -q " 0 failed" "$TMP/kv.txt"
"${CHECK[@]}" "$SRC" > "$TMP/ks.txt" 2>&1
grep -q "LIB   purelib: 3/3 exports implemented" "$TMP/ks.txt"
tail -1 "$TMP/ks.txt"
tail -1 "$TMP/ks.txt" | grep -q " 0 failed"

echo "== gate 4: byte tie (certs -> assembled module -> enc_lib = shipped bytes)"
"$EVAL" run tools/lowbuild/lowbuild.shard "$BUILD" wasm > "$TMP/plan.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$OUT" > "$TMP/tie.txt"
TIE=$(grep '^TIE purelib ' "$TMP/tie.txt" | cut -d' ' -f3)
MOD=$(grep '^MOD purelib ' "$TMP/plan.txt" | cut -d' ' -f3)
[ -n "$TIE" ]
[ "$TIE" = "$MOD" ]
echo "BYTETIE OK"
"$EVAL" run tools/lowcheck/manifest.shard "$TMP/plan.txt" models/wasm/wasm.shard "$OUT"

echo "== gate 5: engine (V8 replay, exports by name)"
command -v node >/dev/null || { echo "REFUSED: no node — the ENGINE gate cannot run"; exit 1; }
node examples/wasm_diff.mjs "$TMP/plan.txt"

echo "ARTIFACT OK: the purelib LIB set — one module, named exports, all five gates green"
