#!/usr/bin/env bash
# examples/lowbuild_lib_x86.sh SRC OUT — the GENERIC x86 LIB BUILD (docs/
# X86.md §19, the lib-build arc's second target): one script for every
# (lib …) source lowered by tools/x86gen LIB mode; no per-module
# mod.build, no per-module gate script. The plan is DERIVED — per-export
# XMOD blobs (enc_image, export laid first) + ART bindings at the PINNED
# cert indices, vectors SYNTHESIZED inside the certs' premise domain with
# spec-side expected values. Gates:
#   1. REGEN    — x86gen LIB mode output is byte-identical to OUT
#   2. SCHEMA   — tools/lowcheck validates every cert (x86 lib form)
#   3. KERNEL   — OUT's machine proofs check; SRC checks with its (lib …)
#                 acceptance (exports implemented, accepts names exports)
#   4. ACCEPTS  — the PREMISE-PERCOLATION gate under the WIDTH-ORDERED
#                 COVERAGE law: every export's cert premise surface is
#                 covered/witnessed by its declared accepts entry (wrap32
#                 declared covers this target's wrap64 surface)
#   5. BYTETIE  — the image assembled FROM THE CERTS re-encodes per
#                 export to the plan's XMOD bytes; the manifest binds
#                 name -> cert -> pinned index
#   6. ENGINE   — the derived plan replays on the REAL CPU
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -euo pipefail
[ $# -eq 2 ] || { echo "usage: lowbuild_lib_x86.sh SRC OUT"; exit 2; }
SRC=$1
OUT=$2
EVAL=${EVAL:-bin/shard_eval}
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism)"
"$EVAL" run tools/x86gen/x86gen.shard "$SRC" "$TMP/out.raw" >/dev/null
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

echo "== gate 5: byte tie (certs -> assembled image -> enc_image = shipped bytes)"
"$EVAL" run tools/lowbuild/lowbuild.shard lib "$SRC" "$OUT" x86 > "$TMP/plan.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$OUT" > "$TMP/tie.txt"
grep '^XMOD ' "$TMP/plan.txt" | sort > "$TMP/mods.txt"
sed 's/^TIE /XMOD /' "$TMP/tie.txt" | sort > "$TMP/ties.txt"
diff "$TMP/ties.txt" "$TMP/mods.txt"
echo "BYTETIE OK"
"$EVAL" run tools/lowcheck/manifest.shard "$TMP/plan.txt" models/x86/x86.shard "$OUT"

echo "== gate 6: engine (the CPU replays the derived plan)"
command -v cc >/dev/null || { echo "REFUSED: no cc — the ENGINE gate cannot run"; exit 1; }
grep -v '^ARTIFACT ' "$TMP/plan.txt" > "$TMP/cpu_plan.txt"
cc -O2 -o "$TMP/x86_diff" examples/x86_diff.c
"$TMP/x86_diff" "$TMP/cpu_plan.txt"

echo "ARTIFACT OK: x86 lib $(grep -c '^XMOD ' "$TMP/plan.txt") export blob(s) — derived plan, six gates green"
