#!/usr/bin/env bash
# v3/kernel/test/define_test.sh — Stage 1's `fn` = `def` + `realize` on real
# files (LANGUAGE.md §8.4, slice 3.13): the V3 loader loads v3/std/list.shard
# and calc's spec file with the export through Int.decEq and must define the
# named functions (DEFINE with their equations; parse_rest by course-of-values,
# slice 3.14) and accept the theorems that cite the equations. Exit code = the number of files that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
FIX=v3/kernel/test/fixtures/init_prefix_int.ndjson
fail=0
check() {
  local f=$1; shift
  local out rc
  out=$("$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" "$f" 2>&1); rc=$?
  if [ "$rc" -ne 0 ]; then
    echo "define_test: $f exit $rc"; echo "$out" | grep -E 'REFUSE|ERROR|POLICY' | head -5; fail=$((fail+1)); return
  fi
  for pat in "$@"; do
    if ! echo "$out" | grep -q -- "$pat"; then
      echo "define_test: $f lacks '$pat'"; echo "$out" | grep -E 'DEFINE|RUNNABLE|REFUSE' | head -8; fail=$((fail+1))
    fi
  done
}
check v3/std/list.shard \
  'DEFINE std.list.List.sum equations=std.list.List.sum.eq_1,std.list.List.sum.eq_2' \
  'ACCEPT std.list.List.sum_cons '
check v3/examples/calc/calc_spec.shard \
  'DEFINE examples.calc.calc.eval equations=examples.calc.calc.eval.eq_1,examples.calc.calc.eval.eq_2,examples.calc.calc.eval.eq_3' \
  'DEFINE examples.calc.calc.parse_rest equations=' \
  'ACCEPT examples.calc.calc_spec.eval_add '
# calc's spec file whole (slice 3.16's gate): every function defined, the measured one pending
out=$("$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" v3/examples/calc/calc_spec.shard 2>&1)
if ! echo "$out" | grep -q 'runnable 0  refused 0  pending 1  realized 0  defined 25  errors 0'; then
  echo "define_test: calc_spec is not 25 of 25 defined"; echo "$out" | grep -E '^(RUNNABLE|REFUSE|LOAD:)' | head -8; fail=$((fail+1))
fi
# G8 (GPT-6 R70): a RUNNABLE callee added to a body changes the call and nothing else in the program
dump=$("$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --dump v3/pins/loader/route_callee/main.shard 2>&1)
a=$(echo "$dump" | sed -n 's/^fn a //p'); b=$(echo "$dump" | sed -n 's/^fn b //p' | sed 's/(down #1)/#1/')
if [ -z "$a" ] || [ "$a" != "$b" ]; then echo "define_test: route_callee: b's program is not a's with the call"; echo "$dump" | grep '^fn ' | head -4; fail=$((fail+1)); fi
echo "define_test: 4 checks, $fail failed"
exit "$fail"
