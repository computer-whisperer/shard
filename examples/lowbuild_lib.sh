#!/usr/bin/env bash
# examples/lowbuild_lib.sh SRC OUT — THE GENERIC LIB BUILD (the lib-build
# arc, docs/LOWERING.md §6ag): one script for every (lib …) source; no
# per-module mod.build, no per-module gate script. The plan is DERIVED —
# arts from the declaration, bytes/exports evaluated in the OUT closure,
# vectors SYNTHESIZED inside the certs' premise domain with spec-side
# expected values. Gates:
#   1. REGEN    — wasmgen LIB mode output is byte-identical to OUT
#   2. SCHEMA   — tools/lowcheck validates every cert (lib form included)
#   3. KERNEL   — OUT's machine proofs check; SRC checks with its (lib …)
#                 acceptance (exports implemented, accepts names exports)
#   4. ACCEPTS  — the PREMISE-PERCOLATION gate: every export's cert
#                 premise surface EQUALS its declared accepts entry
#   5. BYTETIE  — the module assembled FROM THE CERTS + the reflected
#                 export table re-encodes to the plan's MOD bytes; the
#                 manifest binds name -> cert -> pinned index
#   6. ENGINE   — the derived plan replays under real V8, exports by name
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -euo pipefail
[ $# -eq 2 ] || { echo "usage: lowbuild_lib.sh SRC OUT"; exit 2; }
SRC=$1
OUT=$2
EVAL=bin/shard_eval
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism)"
"$EVAL" run tools/wasmgen/wasmgen.shard "$SRC" "$TMP/out.raw" >/dev/null
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/out.raw" > "$TMP/out.fmt"
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
grep -q "^LIB   " "$TMP/ks.txt"
tail -1 "$TMP/ks.txt"
tail -1 "$TMP/ks.txt" | grep -q " 0 failed"

echo "== gate 4: accepts (the premise-percolation surface)"
"$EVAL" run tools/lowcheck/accepts.shard "$SRC" "$OUT"

echo "== gate 5: byte tie (certs -> assembled module -> enc_lib = shipped bytes)"
"$EVAL" run tools/lowbuild/lowbuild.shard lib "$SRC" "$OUT" > "$TMP/plan.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$OUT" > "$TMP/tie.txt"
LNAME=$(grep '^MOD ' "$TMP/plan.txt" | head -1 | cut -d' ' -f2)
MOD=$(grep "^MOD $LNAME " "$TMP/plan.txt" | cut -d' ' -f3)
TIE=$(grep "^TIE $LNAME " "$TMP/tie.txt" | cut -d' ' -f3)
[ -n "$LNAME" ]
[ -n "$TIE" ]
[ "$TIE" = "$MOD" ]
echo "BYTETIE OK ($LNAME)"
"$EVAL" run tools/lowcheck/manifest.shard "$TMP/plan.txt" models/wasm/wasm.shard "$OUT"

echo "== gate 6: engine (V8 replay, exports by name)"
command -v node >/dev/null || { echo "REFUSED: no node — the ENGINE gate cannot run"; exit 1; }
node examples/wasm_diff.mjs "$TMP/plan.txt"

echo "ARTIFACT OK: lib $LNAME — derived plan, six gates green"
