#!/usr/bin/env bash
# v3/kernel/test/store_test.sh — the store and the release gate (v3/LANGUAGE.md §8.4 slice
# 3.22, landing 3; law §7.5). (1) v3/examples/auto under `load.shard --store`: 23 declarations
# written as the records K ingests, DIR/order in admission order; a second load of the same
# module writes nothing new and lists nothing twice; verify_release.shard — K, the host and
# the JSON reader, no elaborator — replays the store after the closure of Init it cites and accepts every
# declaration. Then the gate refuses what it must: a theorem whose value is replaced by its
# statement is REJECTED, an expression record removed is MALFORMED, a file removed is MISSING,
# a name record removed is MISTIED (the declaration lands under another name), each with a
# non-zero exit. (2) calc: the five leaves of tactic_test.sh stored in sequence into
# v3/release/calc (the CI artifact; shared modules written once) and replayed by K — every
# declaration accepted, the count printed. Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "store_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index (slice 3.26)
RCPT=${INIT_RECEIPT:+--init-receipt $INIT_RECEIPT}   # the Init receipt (slice 3.23), set by v3/test.sh when one exists
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "store_test: $*"; fail=$((fail+1)); }
load_store() { "$EVAL" direct v3/kernel/load.shard --root v3 $INIT $RCPT --store "$1" "$2"; }
verify() { "$EVAL" direct v3/kernel/verify_release.shard "$1" "$EXPORT" "$INDEX"; }

# ---- (1) the example
mkdir -p "$OUT/auto"
load_store "$OUT/auto" v3/examples/auto/auto.shard > "$OUT/load1.txt" 2>&1
checks=$((checks+1)); grep -q '^STORE: .* new=23 shared=0$' "$OUT/load1.txt" || { bad "the example's store is not 23 new declarations"; tail -2 "$OUT/load1.txt" | cut -c1-300; }
checks=$((checks+1)); [ "$(wc -l < "$OUT/auto/order")" = 23 ] || bad "order does not list 23 declarations"
checks=$((checks+1)); [ "$(ls "$OUT/auto" | grep -c '\.ndjson$')" = 23 ] || bad "the store does not hold 23 files"
load_store "$OUT/auto" v3/examples/auto/auto.shard > "$OUT/load2.txt" 2>&1
checks=$((checks+1)); grep -q '^STORE: .* new=0 shared=23$' "$OUT/load2.txt" || { bad "the second load did not share every declaration"; tail -1 "$OUT/load2.txt" | cut -c1-300; }
checks=$((checks+1)); [ "$(wc -l < "$OUT/auto/order")" = 23 ] || bad "the second load changed order"
verify "$OUT/auto" > "$OUT/v1.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "verify_release failed on the example (exit $rc)"; grep -v '^ACCEPT' "$OUT/v1.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^RELEASE: declarations 23  accepted 23 .* malformed 0  mistied 0  missing 0  taken 0$' "$OUT/v1.txt" || { bad "the release line is not 23 of 23"; tail -1 "$OUT/v1.txt" | cut -c1-300; }
echo "store_test: the example's 23 declarations stored once and accepted by K alone"

# the gate refuses: a theorem's value replaced by its statement (REJECT), a record removed (MALFORMED), a file removed (MISSING)
id=$(grep ' examples.auto.auto.le_step$' "$OUT/auto/order" | cut -d' ' -f1)
cp -r "$OUT/auto" "$OUT/t1"; sed -i -E 's/"type":([0-9]+),"value":[0-9]+/"type":\1,"value":\1/' "$OUT/t1/$id.ndjson"
verify "$OUT/t1" > "$OUT/t1.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^REJECT .* examples.auto.auto.le_step$' "$OUT/t1.txt" && grep -q '^RELEASE: declarations 23  accepted 22  rejected 1 ' "$OUT/t1.txt" || { bad "a theorem proved by its own statement was not rejected (exit $rc)"; tail -2 "$OUT/t1.txt" | cut -c1-300; }
cp -r "$OUT/auto" "$OUT/t2"; sed -i '/"ie":1,/d' "$OUT/t2/$id.ndjson"
verify "$OUT/t2" > "$OUT/t2.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^MALFORMED' "$OUT/t2.txt" && grep -q 'malformed [1-9][0-9]*  mistied 0  missing 0  taken 0$' "$OUT/t2.txt" || { bad "an expression record removed was not malformed (exit $rc)"; tail -2 "$OUT/t2.txt" | cut -c1-300; }
cp -r "$OUT/auto" "$OUT/t3"; rm "$OUT/t3/$id.ndjson"
verify "$OUT/t3" > "$OUT/t3.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q "^MISSING $id examples.auto.auto.le_step$" "$OUT/t3.txt" && grep -q 'missing 1  taken 0$' "$OUT/t3.txt" || { bad "a file removed was not missing (exit $rc)"; tail -2 "$OUT/t3.txt" | cut -c1-300; }
# a name record removed: the import reads the missing index as anonymous and K accepts the
# theorem under a shorter name — the gate holds each declaration to its order line's name
cp -r "$OUT/auto" "$OUT/t4"; sed -i '/"in":3,/d' "$OUT/t4/$id.ndjson"
verify "$OUT/t4" > "$OUT/t4.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^MISTIED examples.auto.auto.le_step accepted as ' "$OUT/t4.txt" && grep -q 'mistied 1  missing 0  taken 0$' "$OUT/t4.txt" || { bad "a name record removed was not mistied (exit $rc)"; tail -2 "$OUT/t4.txt" | cut -c1-300; }
echo "store_test: the gate rejects a forged proof, a broken record, a missing file and a renamed declaration"

# ---- (2) calc: the five leaves into the release store, K replays it
REL=v3/release/calc
rm -rf "$REL"; mkdir -p "$REL"
s=$(date +%s)
for f in calc_app_world calc_show_run calc_ndigit calc_spec_tests calc_reconcile_tests; do
  load_store "$REL" "v3/examples/calc/$f.shard" > "$OUT/c_$f.txt" 2>&1
  checks=$((checks+1)); grep -q '^STORE: ' "$OUT/c_$f.txt" || { bad "$f did not store"; grep -E 'ERROR|REFUSE|PENDING|^LOAD:|^STORE:' "$OUT/c_$f.txt" | head -3 | cut -c1-300; }
done
e=$(date +%s)
n=$(wc -l < "$REL/order")
echo "store_test: calc's five leaves stored in $((e-s)) s: $n declarations, $(du -sk "$REL" | cut -f1) kB"
s=$(date +%s)
verify "$REL" > "$OUT/vc.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "verify_release failed on calc (exit $rc)"; grep -v '^ACCEPT' "$OUT/vc.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q "^RELEASE: declarations $n  accepted $n " "$OUT/vc.txt" || { bad "K did not accept every one of calc's $n declarations"; tail -1 "$OUT/vc.txt" | cut -c1-300; }
echo "store_test: K alone accepts calc's $n declarations after Init's closure in $((e-s)) s"
echo "store_test: $checks checks, $fail failed"
exit $fail
