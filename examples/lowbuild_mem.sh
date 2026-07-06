#!/usr/bin/env bash
# lowbuild_mem.sh — the four-gate build for lowergen's MEM fragment
# (PORTABLE certs = the primary, callee bytes appear nowhere; LINKED
# derivations = the artifact form, derived by citation — regenerates
# alone when std/mem's implementation changes)
# (examples/lowergen_mem_src.shard: bodies that CALL std/mem's shipped
# wasm artifacts). Same gates as lowbuild.sh:
#   1. REGEN     — the generator's cert file is byte-identical to the
#                  committed one (producer determinism)
#   2. SCHEMA    — tools/lowcheck structurally validates every cert
#   3. KERNEL    — the cert proofs actually check (bridge citations of
#                  lowered_mem_get/lowered_mem_set through call_bridge)
#   4. BYTETIE   — tools/bytetie re-encodes each linked cert's module
#                  literal at restfs := Nil, diffed against the MOD bytes,
#                  and the manifest's ARTIFACT lines bind name -> cert ->
#                  pinned export index (tools/lowcheck/manifest.shard)
#   5. ENGINE    — the artifact plan (binaries + spec-semantics vectors)
#                  replays under real V8
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -euo pipefail
SRC=examples/lowergen_mem_src.shard
PORT=examples/lowergen_mem_port.shard
LINK=examples/lowergen_mem_link.shard
EVAL=bin/shard_eval
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism, portable + linked)"
"$EVAL" run tools/lowergen/lowergen.shard "$SRC" "$TMP/lowergen_mem_port.shard" "$TMP/lowergen_mem_link.shard" >/dev/null
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/lowergen_mem_port.shard" > "$TMP/port.fmt"
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/lowergen_mem_link.shard" > "$TMP/link.fmt"
diff -q "$TMP/port.fmt" "$PORT" && diff -q "$TMP/link.fmt" "$LINK" && echo "REGEN OK (both byte-identical)"

echo "== gate 2: schema (consumer-side validation, both forms)"
"$EVAL" run tools/lowcheck/lowcheck.shard "$PORT"
"$EVAL" run tools/lowcheck/lowcheck.shard "$LINK"

echo "== gate 3: kernel (the proofs; the linked file imports the portable one)"
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK=(bin/shard_check)
else
  CHECK=("$EVAL" run kernel/check.shard)
fi
"${CHECK[@]}" "$LINK" 2>&1 | tail -1 | tee "$TMP/kv.txt"
grep -q " 0 failed" "$TMP/kv.txt"

echo "== gate 4: byte tie (cert module literals re-encode to the shipped bytes)"
"$EVAL" run tools/lowbuild/lowbuild.shard examples/lowergen_mem_src.build.shard > "$TMP/plan.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$LINK" > "$TMP/tie.txt"
grep '^MOD ' "$TMP/plan.txt" | sort > "$TMP/mods.txt"
sed 's/^TIE /MOD /' "$TMP/tie.txt" | sort > "$TMP/ties.txt"
diff "$TMP/ties.txt" "$TMP/mods.txt" && echo "BYTETIE OK"
"$EVAL" run tools/lowcheck/manifest.shard "$TMP/plan.txt" models/wasm/wasm.shard "$PORT" "$LINK"

echo "== gate 5: engine (V8 replay of the artifact plan)"
command -v node >/dev/null || { echo "REFUSED: no node — the ENGINE gate cannot run"; exit 1; }
node examples/wasm_diff.mjs "$TMP/plan.txt"

echo "ARTIFACT OK: mem-fragment binaries + manifest + certs, all five gates green"
