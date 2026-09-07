#!/usr/bin/env bash
# v3/build.sh — compiled K: v3/kernel/t0.shard through the chain (tools/lower →
# tools/codegen → cc) into v3/bin/t0. FOUNDATION §9.1 route 1 — K compiled by
# shard's own lowering — as an engine; its proof is still to come, so the Rust
# interpreter (route 3) stays the authority and v3/t0_full.sh byte-ties the two
# on a prefix before every full replay.
#
# The tools are named REPO-RELATIVE from the repo root: kernel/resolve.shard
# grants `core` identity only to the exact path kernel/stdlib, so a tool named
# by an absolute path imports a non-core prelude and its every Pair pattern
# fails ("match fell through … scrutinee head: Pair") — #41.
#
# Boot engine: bin/shard_eval when present (seconds), else the Rust
# interpreter (minutes). v3/bin/ is derived and untracked.
set -euo pipefail
cd "$(dirname "$0")/.."
SRC=${1:-v3/kernel/t0.shard}
OUT=${2:-v3/bin/t0}
RUST_EVAL=./rust_bootstrap/target/release/eval
BOOT=bin/shard_eval
if [ ! -x "$BOOT" ]; then
  [ -x "$RUST_EVAL" ] || { echo "missing $RUST_EVAL (cargo build --release in rust_bootstrap/)"; exit 1; }
  echo "== boot engine: the Rust interpreter (bin/shard_eval absent; minutes, not seconds)"
  BOOT=$RUST_EVAL
fi
mkdir -p "$(dirname "$OUT")"
LOW=$OUT.low.shard
echo "== lower   $SRC"
"$BOOT" run tools/lower/lower.shard "$SRC" "$LOW"
[ -s "$LOW" ] || { echo "FAIL: $LOW is empty"; exit 1; }
echo "== codegen $LOW"
"$BOOT" run tools/codegen/codegen.shard "$LOW" "$LOW.c"
[ -s "$LOW.c" ] || { echo "FAIL: $LOW.c is empty"; exit 1; }
echo "== cc      -> $OUT"
cc -O2 -o "$OUT" "$LOW.c" -I tools/codegen
echo "OK: $OUT"
