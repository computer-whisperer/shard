#!/usr/bin/env bash
# Rebuild the native engine from kernel sources via the temporary compiler
# chain: tools/lower (shard -> RS-shard) -> tools/codegen (RS -> C) -> cc.
# The chain always runs on the trusted Rust interpreter -- never on a previous
# native binary -- so a stale or wrong binary cannot propagate itself.
# ~6 min total: lower ~2.5 min, codegen ~4 min, cc ~2 s.
#
# The generated artifacts (SRC.low.shard, SRC.low.shard.c, the binary) are
# derived files and stay untracked; this script is the only build path.
set -euo pipefail
cd "$(dirname "$0")/.."
RUST_EVAL=./rust_bootstrap/target/release/eval
[ -x "$RUST_EVAL" ] || { echo "missing $RUST_EVAL (cargo build --release in rust_bootstrap/)"; exit 1; }

build() {
  local src=$1 out=$2
  local low=$src.low.shard
  echo "== lower   $src"
  "$RUST_EVAL" run tools/lower/lower.shard "$src" "$low"
  [ -s "$low" ] || { echo "FAIL: $low is empty"; exit 1; }
  echo "== codegen $low"
  "$RUST_EVAL" run tools/codegen/codegen.shard "$low" "$low.c"
  [ -s "$low.c" ] || { echo "FAIL: $low.c is empty"; exit 1; }
  echo "== cc      -> $out"
  cc -O2 -o "$out" "$low.c" -I tools/codegen
}

build kernel/eval.shard bin/shard_eval
bin/engine_stamp.sh > bin/shard_eval.stamp
echo "OK: bin/shard_eval (stamp $(cat bin/shard_eval.stamp | cut -c1-12))"
