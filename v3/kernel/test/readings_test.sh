#!/usr/bin/env bash
# v3/kernel/test/readings_test.sh — the two readings of a program (docs/LAW.md rule 24).
# A fn's program is the erasure of its elaborated body (the typed route); the classifier reads
# a program from the source and the types its binders state, with no elaborated term. Under
# `load.shard --both` every fn the typed route gives a program is read by the classifier too
# and the two programs are compared: `BOTH NAME same`, `differs` with both texts, or `unread`
# with the classifier's refusal. Over v3/pins/readings/main.shard:
# (1) the sixteen fns of its first part, and `report`, are `same`: the operators by the type
#     expected and by the operands, a comparison as a Bool, Init's operations written by name;
# (2) the controls: a numeral pattern is `differs` (a chain of tests on the typed route, a
#     match here), a body that passes a lambda and one that converts a String are `unread`;
#     no fn has another verdict, and a load without the flag has no `BOTH` record;
# (3) the part the classifier alone reads runs to five lines: `-` at Nat stops at zero, `/`
#     and `%` at Int are the Euclidean pair, Int.natAbs is a Nat, a comparison is Init's Bool.
# Runs on either engine (EVAL); under v3/test.sh the two runs are compared byte for byte.
# Exit code = the number of failed checks.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "readings_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
OUT=$(mktemp -d); trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "readings_test: $*"; fail=$((fail+1)); }
F=v3/pins/readings/main.shard
M='pins\.readings\.main'

"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --both $F > "$OUT/both.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the load under --both failed (exit $rc)"; grep -E '^(READ-ERROR|LOAD-ERROR|REFUSE|LOAD:)' "$OUT/both.txt" | head -5 | cut -c1-300; }

# ---- (1) one program by both readings
for f in sub_nat sub_int numeral_right numerals_at_int numerals_at_nat div_both mod_both under_int under_list cmp_value cmp_cond abs_pred connectives succ_zero by_name self_call report; do
  checks=$((checks+1))
  grep -Eq "^BOTH $M\.$f same\$" "$OUT/both.txt" || { bad "$f is not read the same both ways"; grep -E "^BOTH $M\.$f " "$OUT/both.txt" | cut -c1-400; }
done

# ---- (2) the controls
checks=$((checks+1)); grep -Eq "^BOTH $M\.lit_pattern differs: typed: fn lit_pattern 0 \(Nat\) Nat \(if \(= #0 0\) 1 0\) \| plain: fn lit_pattern 0 \(Nat\) Nat \(match #0 \(0 1\) \(_ 0\)\)\$" "$OUT/both.txt" || { bad "the numeral pattern is not reported with its two programs"; grep -E "^BOTH $M\.lit_pattern " "$OUT/both.txt" | cut -c1-300; }
checks=$((checks+1)); grep -Eq "^BOTH $M\.add_two unread: function_value: " "$OUT/both.txt" || { bad "the lambda is not reported unread"; grep -E "^BOTH $M\.add_two " "$OUT/both.txt" | cut -c1-300; }
checks=$((checks+1)); grep -Eq "^BOTH $M\.say unread: unknown_head: String\.toByteArray\$" "$OUT/both.txt" || { bad "the String conversion is not reported unread"; grep -E "^BOTH $M\.say " "$OUT/both.txt" | cut -c1-300; }
n=$(grep -c '^BOTH ' "$OUT/both.txt")
checks=$((checks+1)); [ "$n" = 20 ] || { bad "$n BOTH records, not the twenty of the fixture"; grep '^BOTH ' "$OUT/both.txt" | cut -c1-120; }
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT $F > "$OUT/plain.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || bad "the load without the flag failed (exit $rc)"
checks=$((checks+1)); if grep -q '^BOTH ' "$OUT/plain.txt"; then bad "a load without --both has BOTH records"; fi

# ---- (3) the classifier's program, run
printf '%s\n' 'Nat.sub: ok' 'Int.ediv: ok' 'Int.emod: ok' 'natAbs then Nat.sub: ok' 'a comparison is a Bool: ok' > "$OUT/want.txt"
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --run pins.readings.main.main $F > "$OUT/run.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || bad "the run failed (exit $rc)"
checks=$((checks+1)); cmp -s "$OUT/run.txt" "$OUT/want.txt" || { bad "the run's lines are not the five expected"; cut -c1-200 "$OUT/run.txt" | head -8; }

echo "readings_test: $checks checks, $fail failed"
exit $fail
