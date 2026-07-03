#!/usr/bin/env bash
# tools/bench/nested_bench.sh — the evaluator-promotion yardstick.
#
# Times bench_work.shard at nesting levels L0/L1/L2:
#   L0: the native engine runs the workload directly
#   L1: the native engine runs eval.shard, which runs the workload
#   L2: eval.shard runs eval.shard, which runs the workload
# Each level runs twice: a TRIVIAL point (R=0 — fixed load cost: closure
# resolution + parse at that level's speed) and a WORK point. The marginal
# per-unit cost is (work - trivial) / (R*N); the per-level interpretation
# multiplier is the ratio of adjacent levels' marginal costs. Work sizes
# shrink ~100x per level to keep wall time sane; every run must print the
# same-formula sum (R * N*(N+1)/2) — a differential check rides along.
#
# Usage: tools/bench/nested_bench.sh [max_level]   (default 2)
set -u
cd "$(dirname "$0")/../.."
EV=bin/shard_eval
WORK=tools/bench/bench_work.shard
EVSH=kernel/eval.shard

# per-level work points: R N (unit count = R*N)
R0=2000; N0=1000
R1=100;  N1=1000
R2=10;   N2=100

max=${1:-2}

run_timed() {  # label, then the command
  local label="$1"; shift
  local t0 t1 out
  t0=$(date +%s.%N)
  out=$("$@")
  t1=$(date +%s.%N)
  printf '%-14s %10.2fs   -> %s\n' "$label" "$(echo "$t1 - $t0" | bc)" "$out"
}

echo "engine: $EV ($(cat bin/shard_eval.stamp 2>/dev/null | cut -c1-8))"
run_timed "L0 trivial"  $EV run $WORK 0 0
run_timed "L0 ${R0}x${N0}" $EV run $WORK $R0 $N0
if [ "$max" -ge 1 ]; then
  run_timed "L1 trivial"  $EV run $EVSH run $WORK 0 0
  run_timed "L1 ${R1}x${N1}" $EV run $EVSH run $WORK $R1 $N1
fi
if [ "$max" -ge 2 ]; then
  run_timed "L2 trivial"  $EV run $EVSH run $EVSH run $WORK 0 0
  run_timed "L2 ${R2}x${N2}" $EV run $EVSH run $EVSH run $WORK $R2 $N2
fi
