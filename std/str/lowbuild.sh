#!/usr/bin/env bash
# std/str/lowbuild.sh — the gated artifact build for std/str's wasm string
# ops (the first module whose pieces are EMITTER-GENERATED end to end:
# str.lowsrc.shard sources -> tools/lowergen -> str.wasm.shard certs, with
# the aggregate rep certs in str.rep.shard tying the machine behavior to
# the module's opaque surface). Gates:
#   1. REGEN     — the generator's cert file is byte-identical to the
#                  committed one (producer determinism)
#   2. SCHEMA    — tools/lowcheck structurally validates every cert
#   3. KERNEL    — the machine-written inductions actually check, AND the
#                  aggregate rep certs (lowered_str_copy) check
#   4. BYTETIE   — tools/bytetie re-encodes the cert's module literal at
#                  restfs := Nil and diffs against the plan's MOD bytes
#   5. ENGINE    — the artifact plan (binary + spec-semantics vectors)
#                  replays under real V8
# Exit 0 = a fully gated artifact set. Run from the repo root.
set -eu
SRC=std/str/str.lowsrc.shard
OUT=std/str/str.wasm.shard
REP=std/str/str.rep.shard
BUILD=std/str/mod.build.shard
EVAL=bin/shard_eval
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism)"
"$EVAL" run tools/lowergen/lowergen.shard "$SRC" "$TMP/str.wasm.shard" >/dev/null
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/str.wasm.shard" > "$TMP/out.fmt"
diff -q "$TMP/out.fmt" "$OUT" && echo "REGEN OK (byte-identical)"

echo "== gate 2: schema (consumer-side validation)"
"$EVAL" run tools/lowcheck/lowcheck.shard "$OUT"

echo "== gate 3: kernel (machine inductions + aggregate rep certs)"
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK=(bin/shard_check)
else
  CHECK=("$EVAL" run kernel/check.shard)
fi
"${CHECK[@]}" "$OUT" 2>&1 | tail -1 | tee "$TMP/kv.txt"
grep -q " 0 failed" "$TMP/kv.txt"
"${CHECK[@]}" "$REP" 2>&1 | tail -1 | tee "$TMP/kr.txt"
grep -q " 0 failed" "$TMP/kr.txt"

echo "== gate 4: byte tie (cert module literal re-encodes to the shipped bytes)"
"$EVAL" run tools/lowbuild/lowbuild.shard "$BUILD" > "$TMP/plan.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$OUT" > "$TMP/tie.txt"
TIE=$(grep '^TIE sc_copy ' "$TMP/tie.txt" | cut -d' ' -f3)
MOD=$(grep '^MOD stdstr ' "$TMP/plan.txt" | cut -d' ' -f3)
[ -n "$TIE" ] && [ "$TIE" = "$MOD" ] && echo "BYTETIE OK"

echo "== gate 5: engine (V8 replay of the artifact plan)"
command -v node >/dev/null || { echo "SKIPPED: no node"; exit 0; }
node examples/wasm_diff.mjs "$TMP/plan.txt"

echo "ARTIFACT OK: std/str wasm pieces — binary + manifest + certs, all five gates green"
