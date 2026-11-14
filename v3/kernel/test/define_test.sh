#!/usr/bin/env bash
# v3/kernel/test/define_test.sh — a `fn` is a definition with equations, on real
# files: the V3 loader loads calc's spec file and must define the named functions
# (DEFINE with their equations; parse_rest by course-of-values) and accept the
# theorems that cite the equations; then calc's app file, std/fresh, std/bytes and
# std/host, and pins of the definer (a RUNNABLE callee, a discharged measure, a
# native name under a registry row, Decidable's cells, a group, derive's refusals).
# The library's other files are std_test.sh's. Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "define_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index
fail=0
check() {
  local f=$1; shift
  local out rc
  out=$("$EVAL" direct v3/kernel/load.shard --root v3 $INIT "$f" 2>&1); rc=$?
  if [ "$rc" -ne 0 ]; then
    echo "define_test: $f exit $rc"; echo "$out" | grep -E 'REFUSE|ERROR|POLICY' | head -5; fail=$((fail+1)); return
  fi
  for pat in "$@"; do
    if ! echo "$out" | grep -q -- "$pat"; then
      echo "define_test: $f lacks '$pat'"; echo "$out" | grep -E 'DEFINE|RUNNABLE|REFUSE' | head -8; fail=$((fail+1))
    fi
  done
}
check v3/examples/calc/calc_spec.shard \
  'DEFINE examples.calc.calc.eval equations=examples.calc.calc.eval.eq_1,examples.calc.calc.eval.eq_2,examples.calc.calc.eval.eq_3' \
  'DEFINE examples.calc.calc.parse_rest equations=' \
  'ACCEPT examples.calc.calc_spec.eval_add '
# calc's spec file whole: every function defined; the measured one's two descents
# discharged (each stated under its matchers' equations), so nothing pending.
# calc runs Init's List.append and List.length through std/list: its ten realizations and
# nine definitions (three fns, six templates) load beside calc's 23 (realized 10, defined 32)
out=$("$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/examples/calc/calc_spec.shard 2>&1)
if ! echo "$out" | grep -q 'runnable 0  refused 0  pending 0  realized 10  defined 32  errors 0' \
   || [ "$(echo "$out" | grep -c '^DISCHARGE examples.calc.calc_spec.parse_tail.dec_[12] proved')" -ne 2 ]; then
  echo "define_test: calc_spec is not 32 of 32 defined (calc's 23, std/list's 9 — three fns and six templates) with parse_tail's descents discharged"; echo "$out" | grep -E '^(RUNNABLE|REFUSE|PENDING|READ-ERROR|LOAD:)' | head -8; fail=$((fail+1))
fi
# a RUNNABLE callee added to a body changes the call and nothing else in the program
dump=$("$EVAL" direct v3/kernel/load.shard --root v3 $INIT --dump v3/pins/loader/route_callee/main.shard 2>&1)
a=$(echo "$dump" | sed -n 's/^fn a //p'); b=$(echo "$dump" | sed -n 's/^fn b //p' | sed 's/(down #1)/#1/')
if [ -z "$a" ] || [ "$a" != "$b" ]; then echo "define_test: route_callee: b's program is not a's with the call"; echo "$dump" | grep '^fn ' | head -4; fail=$((fail+1)); fi
# the discharged measure's account — three DISCHARGE records, nothing PENDING,
# and the theorem that rested on count.dec_1 rests on nothing after
out=$("$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/pins/loader/discharge_measure/main.shard 2>&1)
if [ "$(echo "$out" | grep -c '^DISCHARGE .* proved')" -ne 3 ] || echo "$out" | grep -q '^PENDING' || ! echo "$out" | grep -q '^ACCEPT .*main.count_self .* params= req='; then
  echo "define_test: discharge_measure: the account is not empty after the discharges"; echo "$out" | grep -E '^(DISCHARGE|PENDING|REFUSE|READ-ERROR)|count_self' | head -8; fail=$((fail+1))
fi
# calc's app file: show_nat defined by its Nat measure through Int.natAbs — the
# registry's expression row erases the measure —, its descent discharged by arith over the
# quotient's rows and its equation by WellFounded.fix_eq; nothing RUNNABLE, nothing pending
out=$("$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/examples/calc/calc_app.shard 2>&1)
if ! echo "$out" | grep -q '^DEFINE examples.calc.calc_app.show_nat equations=examples.calc.calc_app.show_nat.eq_1$' \
   || ! echo "$out" | grep -q '^DISCHARGE examples.calc.calc_app.show_nat.dec_1 proved' \
   || ! echo "$out" | grep -q 'runnable 0  refused 0  pending 0  realized 10  defined 24  errors 0'; then
  echo "define_test: calc_app is not 24 of 24 defined (calc's 15, std/list's 9 — three fns and six templates) with show_nat's descent discharged"; echo "$out" | grep -E '^(RUNNABLE|REFUSE|PENDING|READ-ERROR|DEFINE .*show_nat|LOAD:)' | head -8; fail=$((fail+1))
fi
# the fresh-name supply: every function defined, two successive names differ
check v3/std/fresh.shard \
  'DEFINE std.fresh.Fresh.next equations=std.fresh.Fresh.next.eq_1' \
  'ACCEPT std.fresh.Fresh.next_ne .* axioms= ' \
  'runnable 0  refused 0  pending 0  realized 0  defined 14  errors 0'
# bytes and text: Init's UInt8.toNat by its derived view, the S operations defined,
# Byte.ofNat tied to Init's UInt8.ofNat and the two round trips, each Eq.refl with no axiom
out=$("$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/std/bytes.shard 2>&1)
for pat in '^REALIZE UInt8.toNat view' \
           '^DEFINE std.bytes.Byte.ofNat equations=std.bytes.Byte.ofNat.eq_1' \
           '^ACCEPT std.bytes.Byte.ofNat_eq .* axioms= ' \
           '^ACCEPT std.bytes.Bytes.ofList_toList .* axioms= ' \
           '^ACCEPT std.bytes.Bytes.toList_ofList .* axioms= ' \
           'runnable 0  refused 0  pending 0  realized 2  defined 8  errors 0'; do
  if ! echo "$out" | grep -q -- "$pat"; then echo "define_test: v3/std/bytes.shard lacks '$pat'"; echo "$out" | grep -E '^(REALIZE|DEFINE|RUNNABLE|REFUSE|READ-ERROR|LOAD:)' | head -8; fail=$((fail+1)); fi
done
# the host under the naming law: six externs, nothing refused
check v3/std/host.shard \
  'RUNNABLE extern std.host.get_args' \
  'runnable 6  refused 0  pending 0  realized 0  defined 0  errors 0'
# a registry row never speaks for a native declaration that bears its name: the driver
# refuses the native `Bool.or` by name wherever it loads, as the release gate does
# (`name_taken` on the record); a Decidable value is the type's cells off an `if`'s condition
# (the program agrees with K's theorems on all three entries); the refusals of what has no
# derivation, each with its reason
R=v3/pins/loader/registry_native
out=$("$EVAL" direct v3/kernel/load.shard --root $R $INIT $R/main.shard 2>&1)
echo "$out" | grep -q '^REFUSE Bool.or name_taken' || { echo "define_test: registry_native: the native Bool.or is not refused by name"; echo "$out" | grep -E '^(REFUSE|ACCEPT|RUNNABLE)' | head -3; fail=$((fail+1)); }
R=v3/pins/loader/decidable_cells
for e in main_bool:10 main_nat:10 main_nat_eq:11; do
  "$EVAL" direct v3/kernel/load.shard --root $R $INIT --run main.${e%%:*} $R/main.shard > /dev/null 2>&1; rc=$?
  [ "$rc" -eq "${e##*:}" ] || { echo "define_test: decidable_cells: ${e%%:*} exits $rc, K proves ${e##*:}"; fail=$((fail+1)); }
done
# the members of a group are one definition in K and run as written — the
# entry's exit code is the one K proves of `ping 5` (the pin's theorem `caller_five`)
R=v3/pins/loader/mutual_measure
out=$("$EVAL" direct v3/kernel/load.shard --root $R $INIT $R/main.shard 2>&1)
for pat in '^ACCEPT main.ping.args ' '^ACCEPT main.ping.mutual ' '^ACCEPT main.ping.mutual.eq ' '^DEFINE main.ping equations=main.ping.eq_1$' '^DEFINE main.pong equations=main.pong.eq_1$' '^DISCHARGE main.pong.dec_1 proved' 'pending 0  realized 0  defined 4  errors 0'; do
  echo "$out" | grep -q -- "$pat" || { echo "define_test: mutual_measure lacks '$pat'"; echo "$out" | grep -E '^(DEFINE|RUNNABLE|REFUSE|PENDING|READ-ERROR|LOAD:)' | head -6; fail=$((fail+1)); }
done
"$EVAL" direct v3/kernel/load.shard --root $R $INIT --run main.main $R/main.shard > /dev/null 2>&1; rc=$?
[ "$rc" -eq 3 ] || { echo "define_test: mutual_measure: main exits $rc, K proves 3"; fail=$((fail+1)); }
R=v3/pins/loader/derive_refusals
out=$("$EVAL" direct v3/kernel/load.shard --root $R $INIT $R/main.shard 2>&1)
for pat in 'derive_shape: main.Void' 'derive_type: True is a proposition' 'derive_type: Nat is the integer' 'derive_type: (Fin 3): an argument that is a value' 'derive_type: Nope is not a type here'; do
  echo "$out" | grep -q -- "$pat" || { echo "define_test: derive_refusals lacks '$pat'"; fail=$((fail+1)); }
done
echo "define_test: 20 checks, $fail failed"
exit "$fail"
