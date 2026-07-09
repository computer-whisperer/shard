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

# Refuse to build (and stamp) from non-canonical sources: the stamp hashes
# raw bytes, so a later formatting-only pass over a stamp input would flip
# the stamp without changing engine behavior, permanently flagging the fresh
# binary STALE (seen 2026-07-06: post-rebuild shardfmt of codegen.shard).
# shardfmt --check: exit 0 = already canonical, 1 = a reformat would change
# bytes. Runs on the trusted interpreter like everything else here, in
# parallel across the stamp inputs (~1-2 min wall; the worst single file is
# ~1 min). rt.h is a stamp input too but not a .shard file; it has no
# canonical form to drift from.
stamp_inputs() {
  ls kernel/*.shard | grep -v '\.low\.shard$'
  echo tools/lower/lower.shard
  echo tools/codegen/codegen.shard
}
echo "== fmt gate: stamp inputs canonical?"
drift=$(stamp_inputs | xargs -P 8 -I{} sh -c \
  '"$1" run tools/shardfmt/shardfmt.shard --check "{}" >/dev/null 2>&1 || echo "{}"' \
  _ "$RUST_EVAL")
if [ -n "$drift" ]; then
  echo "REFUSE: stamp inputs are not shardfmt-canonical; format these first:" >&2
  echo "$drift" >&2
  exit 1
fi

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

# `bin/rebuild.sh check` additionally builds the DIRECT-compiled checker
# (gate_sweep's fastest engine: ~0.2s/155MB vs ~minutes/40GB interpreting).
# check.shard's closure is much bigger than eval.shard's: ~1h total.
if [ "${1:-}" = check ]; then
  build kernel/check.shard bin/shard_check
  bin/engine_stamp.sh > bin/shard_check.stamp
  echo "OK: bin/shard_check (stamp $(cat bin/shard_check.stamp | cut -c1-12))"
fi

# Live guard: run-mode stuckness and malformed extern args must die LOUDLY
# (exit 4 + the offending head), never exit 0 — see examples/run_stuckctl.sh.
./examples/run_stuckctl.sh || { echo "REFUSE: stuckness guard failed on the fresh engine"; exit 1; }
