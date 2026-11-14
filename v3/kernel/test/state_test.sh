#!/usr/bin/env bash
# The state's check as a gate (docs/KERNEL.md "The state discipline";
# kernel/thread.shard): K's node ids are unique because inside a check no version of the
# checker's state is advanced twice, and this is what holds K's own sources to it.
# (1) K's implementation — kernel/t0.shard with the directory kernel/k — loaded under
#     `--state`: every function of `kernel.k` and `kernel.expr` that takes or returns the
#     state, opens or builds one, or calls a source of one is `ok` or a `primitive`; none is
#     `reused`, none `unsupported`; the primitives (the functions that open the state or
#     build one) are exactly the twelve listed here; the resumptions (`run_resume`,
#     k/import.shard) exactly check_into's three and process_record's one; the sources (a
#     state answered where none was taken) are called by `empty_run` and `run_on`, once
#     each, and by no one else. A new primitive, resumption or source is an edit of this file.
# (2) The controls, v3/pins/state/main.shard: each spelling that uses a version twice is
#     `reused`, each linear spelling beside it `ok`; what the walk cannot follow is
#     `unsupported`; a source is counted; and every one of them is judged. One limit is
#     pinned as it stands (`lim_lie_out`): a declared type is believed.
# What this does not hold K to (the same section, its limits): the declared types
# themselves, the memo tables as values, a client's use of a run, the reading of a primitive.
# Runs on either engine (EVAL); under v3/test.sh the two runs are compared byte for byte.
# Exit code = the number of failed checks.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
OUT=$(mktemp -d); trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "state_test: $*"; fail=$((fail+1)); }

# ---- (1) K's implementation
"$EVAL" direct v3/kernel/load.shard --root v3 --state v3/kernel/t0.shard v3/kernel/k > "$OUT/k.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the load of K under --state failed (exit $rc)"; grep -E '^(READ-ERROR|LOAD-ERROR|REFUSE|IMPL REFUSE|IMPL-ERROR|LOAD:)' "$OUT/k.txt" | head -5 | cut -c1-300; }
sed -n 's/^IMPL STATE //p; s/^STATE //p' "$OUT/k.txt" | grep -E '^kernel\.(k|expr)\.' > "$OUT/k.state"
n=$(grep -c . "$OUT/k.state")
OKRE=' ok(: (resumes [0-9]+|sources [0-9]+|resumes [0-9]+, sources [0-9]+))?$'
nok=$(grep -cE "$OKRE" "$OUT/k.state")
nprim=$(grep -c ' primitive$' "$OUT/k.state")
checks=$((checks+1)); [ "$n" -ge 300 ] || bad "only $n functions of K were judged (the check is not running over K)"
checks=$((checks+1)); [ "$((nok+nprim))" = "$n" ] || { bad "$((n-nok-nprim)) function(s) of K neither ok nor primitive:"; grep -vE "$OKRE| primitive\$" "$OUT/k.state" | head -10 | cut -c1-300; }
want_prims="kernel.expr.st_anon kernel.expr.st_fresh_node kernel.expr.st_init kernel.expr.st_node kernel.k.guard kernel.k.mk_local kernel.k.st_clear_memo kernel.k.st_for kernel.k.st_memo kernel.k.st_seed kernel.k.st_set_memo kernel.k.st_ticks"
got_prims=$(sed -n 's/ primitive$//p' "$OUT/k.state" | LC_ALL=C sort | tr '\n' ' ' | sed 's/ $//')
checks=$((checks+1)); [ "$got_prims" = "$want_prims" ] || { bad "the primitives are not the twelve listed"; echo "   got: $got_prims"; }
want_res="kernel.k.check_into ok: resumes 3
kernel.k.process_record ok: resumes 1"
got_res=$(grep ' resumes ' "$OUT/k.state" | LC_ALL=C sort)
checks=$((checks+1)); [ "$got_res" = "$want_res" ] || { bad "the resumptions are not check_into's three and process_record's one"; echo "$got_res" | head -5 | sed 's/^/   got: /'; }
want_src="kernel.k.empty_run ok: sources 1
kernel.k.run_on ok: sources 1"
got_src=$(grep ' sources ' "$OUT/k.state" | LC_ALL=C sort)
checks=$((checks+1)); [ "$got_src" = "$want_src" ] || { bad "a state is made from nothing elsewhere than in empty_run and run_on, once each"; echo "$got_src" | head -5 | sed 's/^/   got: /'; }
echo "state_test: K — $n functions judged: $nok ok, $nprim primitive, $((n-nok-nprim)) other"

# ---- (2) the controls
"$EVAL" direct v3/kernel/load.shard --root v3 --state v3/pins/state/main.shard > "$OUT/c.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the controls' load failed (exit $rc)"; grep -E '^(READ-ERROR|LOAD-ERROR|REFUSE|LOAD:)' "$OUT/c.txt" | head -5 | cut -c1-300; }
sed -n 's/^STATE pins\.state\.main\.//p' "$OUT/c.txt" > "$OUT/c.state"
for f in box_st box_set_st box_app ok_chain ok_drop ok_record ok_beside ok_beside_out ok_join; do
  checks=$((checks+1)); grep -qx "$f ok" "$OUT/c.state" || { bad "the control $f is not ok"; grep "^$f " "$OUT/c.state" | cut -c1-200; }
done
for f in re_fork re_rewind re_drop_caller re_record_fork re_record_rewind re_branch_rewind re_two_minters re_over_written re_launder_if re_launder_match re_nosig; do
  checks=$((checks+1)); grep -q "^$f reused: " "$OUT/c.state" || { bad "the control $f is not reused"; grep "^$f " "$OUT/c.state" | cut -c1-200; }
done
for f in un_launder_ctor un_payload un_prim_hand un_lie_in; do
  checks=$((checks+1)); grep -q "^$f unsupported: " "$OUT/c.state" || { bad "the control $f is not unsupported"; grep "^$f " "$OUT/c.state" | cut -c1-200; }
done
for l in "src_two ok: sources 2" "src_reset ok: sources 1" "pr_nested_open primitive" "lim_lie_out ok"; do
  checks=$((checks+1)); grep -qx "$l" "$OUT/c.state" || { bad "the control's line is not \"$l\""; grep "^${l%% *} " "$OUT/c.state" | cut -c1-200; }
done
checks=$((checks+1)); [ "$(grep -c . "$OUT/c.state")" = 28 ] || bad "the controls judged are $(grep -c . "$OUT/c.state"), not 28"
sed 's/^/   /' "$OUT/c.state" | cut -c1-160

echo "state_test: $checks checks, $fail failed"
exit $fail
