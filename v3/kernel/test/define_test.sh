#!/usr/bin/env bash
# v3/kernel/test/define_test.sh — Stage 1's `fn` = `def` + `realize` on real
# files (LANGUAGE.md §8.4, slice 3.13): the V3 loader loads v3/std/list.shard
# and calc's spec file with the export through Int.decEq and must define the
# named functions (DEFINE with their equations) and accept the theorems that
# cite the equations. Exit code = the number of files that failed.
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
  'ACCEPT examples.calc.calc_spec.eval_add '
echo "define_test: 2 files, $fail failed"
exit "$fail"
