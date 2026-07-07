#!/usr/bin/env bash
# lowbuild_x86.sh — the x86_64 straight-line artifact build (docs/X86.md
# §11): the mod.build convention end to end for examples/x86gen_src.shard,
# through the SECOND back end (tools/x86gen). Five gates:
#   1. REGEN     — the generator's cert file is byte-identical to the
#                  committed one (weld discipline; producer honesty)
#   2. SCHEMA    — tools/lowcheck structurally validates every cert
#                  (the x86 structural form; PCC discipline)
#   3. KERNEL    — the cert proofs actually check
#   4. BYTETIE   — tools/bytetie re-encodes each cert's function literal
#                  at restfs := Nil (the x86 reflector + flattening
#                  encoder) and diffs against the plan's XMOD bytes, and
#                  the manifest's ARTIFACT lines bind name -> cert ->
#                  pinned export index (tools/lowcheck/manifest.shard)
#   5. ENGINE    — the artifact plan replays on the REAL CPU
#                  (examples/x86_diff.c: mmap + SysV call); ARTIFACT
#                  lines are filtered out of the replayer's copy
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -euo pipefail
SRC=examples/x86gen_src.shard
OUT=examples/x86gen_out.shard
BUILD=examples/x86gen_src.build.shard
EVAL=bin/shard_eval
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism)"
"$EVAL" run tools/x86gen/x86gen.shard "$SRC" "$TMP/certs.raw" >/dev/null
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

echo "== gate 4: byte tie (cert function literals re-encode to the shipped bytes)"
"$EVAL" run tools/lowbuild/lowbuild.shard "$BUILD" x86 > "$TMP/plan.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$OUT" > "$TMP/tie.txt"
grep '^XMOD ' "$TMP/plan.txt" | sort > "$TMP/mods.txt"
sed 's/^TIE /XMOD /' "$TMP/tie.txt" | sort > "$TMP/ties.txt"
diff "$TMP/ties.txt" "$TMP/mods.txt" && echo "BYTETIE OK"
"$EVAL" run tools/lowcheck/manifest.shard "$TMP/plan.txt" models/x86/x86.shard "$OUT"

echo "== gate 5: engine (the CPU replays the artifact plan)"
command -v cc >/dev/null || { echo "REFUSED: no cc — the ENGINE gate cannot run"; exit 1; }
grep -v '^ARTIFACT ' "$TMP/plan.txt" > "$TMP/cpu_plan.txt"
cc -O2 -o "$TMP/x86_diff" examples/x86_diff.c
"$TMP/x86_diff" "$TMP/cpu_plan.txt"

echo "ARTIFACT OK: binaries + manifest + certs, all five gates green"
