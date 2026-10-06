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
TAIL=v3/kernel/test/fixtures/init_prefix_str_tail.ndjson   # WellFounded.fix_eq: a measured definition's equations (slice 3.20)
fail=0
check() {
  local f=$1; shift
  local out rc
  out=$("$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --init "$TAIL" "$f" 2>&1); rc=$?
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
# calc's spec file whole (slice 3.16's gate): every function defined; the measured one's two
# descents discharged (slice 3.21: each stated under its matchers' equations), so nothing pending
out=$("$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --init "$TAIL" v3/examples/calc/calc_spec.shard 2>&1)
if ! echo "$out" | grep -q 'runnable 0  refused 0  pending 0  realized 0  defined 25  errors 0' \
   || [ "$(echo "$out" | grep -c '^DISCHARGE examples.calc.calc_spec.parse_tail.dec_[12] proved')" -ne 2 ]; then
  echo "define_test: calc_spec is not 25 of 25 defined with parse_tail's descents discharged"; echo "$out" | grep -E '^(RUNNABLE|REFUSE|PENDING|READ-ERROR|LOAD:)' | head -8; fail=$((fail+1))
fi
# G8 (GPT-6 R70): a RUNNABLE callee added to a body changes the call and nothing else in the program
dump=$("$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --dump v3/pins/loader/route_callee/main.shard 2>&1)
a=$(echo "$dump" | sed -n 's/^fn a //p'); b=$(echo "$dump" | sed -n 's/^fn b //p' | sed 's/(down #1)/#1/')
if [ -z "$a" ] || [ "$a" != "$b" ]; then echo "define_test: route_callee: b's program is not a's with the call"; echo "$dump" | grep '^fn ' | head -4; fail=$((fail+1)); fi
# G4 (GPT-6 R65, R69): the discharged measure's account — three DISCHARGE records, nothing PENDING,
# and the theorem that rested on count.dec_1 rests on nothing after
out=$("$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" v3/pins/loader/discharge_measure/main.shard 2>&1)
if [ "$(echo "$out" | grep -c '^DISCHARGE .* proved')" -ne 3 ] || echo "$out" | grep -q '^PENDING' || ! echo "$out" | grep -q '^ACCEPT .*main.count_self .* params=$'; then
  echo "define_test: discharge_measure: the account is not empty after the discharges"; echo "$out" | grep -E '^(DISCHARGE|PENDING|REFUSE|READ-ERROR)|count_self' | head -8; fail=$((fail+1))
fi
# calc's app file (slice 3.17 rule 7): show_nat defined by its Nat measure through Int.natAbs — the
# registry's expression row erases the measure —, its descent discharged by arith over the
# quotient's rows (slice 3.21) and its equation by WellFounded.fix_eq; nothing RUNNABLE, nothing pending
out=$("$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --init "$TAIL" v3/examples/calc/calc_app.shard 2>&1)
if ! echo "$out" | grep -q '^DEFINE examples.calc.calc_app.show_nat equations=examples.calc.calc_app.show_nat.eq_1$' \
   || ! echo "$out" | grep -q '^DISCHARGE examples.calc.calc_app.show_nat.dec_1 proved' \
   || ! echo "$out" | grep -q 'runnable 0  refused 0  pending 0  realized 0  defined 17  errors 0'; then
  echo "define_test: calc_app is not 17 of 17 defined with show_nat's descent discharged"; echo "$out" | grep -E '^(RUNNABLE|REFUSE|PENDING|READ-ERROR|DEFINE .*show_nat|LOAD:)' | head -8; fail=$((fail+1))
fi
# the fresh-name supply (slice 3.17 rule 6): every function defined, two successive names differ
check v3/std/fresh.shard \
  'DEFINE std.fresh.Fresh.next equations=std.fresh.Fresh.next.eq_1' \
  'ACCEPT std.fresh.Fresh.next_ne .* axioms= ' \
  'runnable 0  refused 0  pending 0  realized 0  defined 14  errors 0'
# bytes and text (slice 3.18 rule 6): Init's UInt8.toNat by its derived view, the S operations defined,
# Byte.ofNat tied to Init's UInt8.ofNat and the two round trips, each Eq.refl with no axiom. The
# import reaches line 263,515 of the export: the pins' prefix and its continuation
out=$("$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --init v3/kernel/test/fixtures/init_prefix_str_tail.ndjson v3/std/bytes.shard 2>&1)
for pat in '^REALIZE UInt8.toNat view' \
           '^DEFINE std.bytes.Byte.ofNat equations=std.bytes.Byte.ofNat.eq_1' \
           '^ACCEPT std.bytes.Byte.ofNat_eq .* axioms= ' \
           '^ACCEPT std.bytes.Bytes.ofList_toList .* axioms= ' \
           '^ACCEPT std.bytes.Bytes.toList_ofList .* axioms= ' \
           'runnable 0  refused 0  pending 0  realized 2  defined 8  errors 0'; do
  if ! echo "$out" | grep -q -- "$pat"; then echo "define_test: v3/std/bytes.shard lacks '$pat'"; echo "$out" | grep -E '^(REALIZE|DEFINE|RUNNABLE|REFUSE|READ-ERROR|LOAD:)' | head -8; fail=$((fail+1)); fi
done
# the host under the naming law (rule 4): six externs, nothing refused
check v3/std/host.shard \
  'RUNNABLE extern std.host.get_args' \
  'runnable 6  refused 0  pending 0  realized 0  defined 0  errors 0'
# slice 3.19, the second reader's findings: a registry row never speaks for a native
# declaration that bears its name; a Decidable value is the type's cells off an `if`'s
# condition (the program agrees with K's theorems on all three entries); the refusals of
# what has no derivation, each with its reason
R=v3/pins/loader/registry_native
dump=$("$EVAL" direct v3/kernel/load.shard --root $R --init "$FIX" --dump $R/main.shard 2>&1)
echo "$dump" | grep -q '^fn f 0 (Bool Bool) Bool (or #1 #0)$' || { echo "define_test: registry_native: f is not the call of the native or"; echo "$dump" | grep '^fn ' | head -3; fail=$((fail+1)); }
R=v3/pins/loader/decidable_cells
for e in main_bool:10 main_nat:10 main_nat_eq:11; do
  "$EVAL" direct v3/kernel/load.shard --root $R --init "$FIX" --run main.${e%%:*} $R/main.shard > /dev/null 2>&1; rc=$?
  [ "$rc" -eq "${e##*:}" ] || { echo "define_test: decidable_cells: ${e%%:*} exits $rc, K proves ${e##*:}"; fail=$((fail+1)); }
done
R=v3/pins/loader/derive_refusals
out=$("$EVAL" direct v3/kernel/load.shard --root $R --init "$FIX" $R/main.shard 2>&1)
for pat in 'derive_shape: main.Void' 'derive_type: True is a proposition' 'derive_type: Nat is the integer' 'derive_type: (Fin 3): an argument that is a value' 'derive_type: Nope is not a type here'; do
  echo "$out" | grep -q -- "$pat" || { echo "define_test: derive_refusals lacks '$pat'"; fail=$((fail+1)); }
done
echo "define_test: 12 checks, $fail failed"
exit "$fail"
