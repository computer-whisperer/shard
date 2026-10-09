#!/usr/bin/env bash
# v3/kernel/test/path_test.sh — law §12.4's first connected path, assembled and broken
# (LANGUAGE.md §8.4 slice 3.24; §13 item 58; GPT-6's R76). The path, in order, on
# v3/examples/path: the program loads (two imported Init functions realized with their
# equations proved, the caller `at` with its branch-local proof, the two claims through I);
# the entry runs on a raw argument; the load's constants are stored; K alone replays the
# store after the closure of Init it cites. Then the law's five breaks, each required to fail by name:
# a wrong executable body in a realize (the equation refused), missing bound evidence (the
# hypothesis cited outside its branch, or dropped from the Fin.mk), a mismatched revision
# (the store's order line naming another declaration — MISTIED), a tampered result (a stored
# proof replaced by its statement — REJECT), an invalid raw argument (refused at the entry,
# exit 6). The path's cost is printed: load, run, store and verify in seconds. Exit code =
# the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "path_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index (slice 3.26)
RCPT=${INIT_RECEIPT:+--init-receipt $INIT_RECEIPT}   # the Init receipt (slice 3.23), set by v3/test.sh when one exists
OUT=$(mktemp -d)
SCRATCH=$(mktemp -d -p v3 path_scratch_XXXXXX)
STORE=$(mktemp -d -p v3 path_store_XXXXXX)
trap 'rm -rf "$OUT" "$SCRATCH" "$STORE"' EXIT
fail=0; checks=0
bad() { echo "path_test: $*"; fail=$((fail+1)); }
load() { "$EVAL" direct v3/kernel/load.shard --root v3 $INIT $RCPT "$@"; }
MAIN=v3/examples/path/path_main.shard
ENTRY=examples.path.path_main.main
secs() { echo $(( $(date +%s) - $1 )); }

# ---- the path, in order ----------------------------------------------------------------------
s=$(date +%s); load "$MAIN" > "$OUT/load.txt" 2>&1; rc=$?; t_load=$(secs $s)
checks=$((checks+1)); [ "$rc" = 0 ] && grep -q 'refused 0 .*errors 0$' "$OUT/load.txt" || { bad "the path does not load clean (exit $rc)"; grep -E 'ERROR|REFUSE|PENDING|^LOAD:' "$OUT/load.txt" | head -5 | cut -c1-300; }
for d in "REALIZE List.length supplied" "REALIZE List.get?Internal supplied" "DEFINE examples.path.path.at " "DEFINE examples.path.path.pick " "ACCEPT examples.path.path.at_some " "ACCEPT examples.path.path.at_none "; do
  checks=$((checks+1)); grep -q "^$d" "$OUT/load.txt" || bad "missing record: $d"
done
s=$(date +%s)
checks=$((checks+1)); [ "$(load --run $ENTRY "$MAIN" -- 2 2>&1 | tail -1)" = "4" ] || bad "read at index 2 is not 4"
t_run=$(secs $s)
checks=$((checks+1)); [ "$(load --run $ENTRY "$MAIN" -- 7 2>&1 | tail -1)" = "6" ] || bad "read at index 7 is not 6"
checks=$((checks+1)); [ "$(load --run $ENTRY "$MAIN" -- 8 2>&1 | tail -1)" = "-" ] || bad "read past the end is not -"
s=$(date +%s); load --store "$STORE" "$MAIN" > "$OUT/store.txt" 2>&1; t_store=$(secs $s)
checks=$((checks+1)); grep -q "^STORE: $STORE new=56 shared=0$" "$OUT/store.txt" || { bad "the store did not take 56 declarations"; grep '^STORE' "$OUT/store.txt"; }
s=$(date +%s); "$EVAL" direct v3/kernel/verify_release.shard "$STORE" "$EXPORT" "$INDEX" > "$OUT/verify.txt" 2>&1; rc=$?; t_verify=$(secs $s)
checks=$((checks+1)); [ "$rc" = 0 ] && grep -q '^RELEASE: declarations 56  accepted 56 ' "$OUT/verify.txt" || { bad "K alone did not accept the 56 declarations (exit $rc)"; grep -v '^ACCEPT' "$OUT/verify.txt" | tail -3 | cut -c1-300; }
echo "path_test: the path composes — load $t_load s, run $t_run s, store $t_store s, verify $t_verify s"

# ---- the breaks ------------------------------------------------------------------------------
# 1. a wrong executable body: List.length's realization counts two per cell
sed 's/(+ (List.length t) 1)/(+ (List.length t) 2)/' v3/examples/path/path.shard > "$SCRATCH/wrong_body.shard"
load "$SCRATCH/wrong_body.shard" > "$OUT/b1.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q "List.length.realize_2 " "$OUT/b1.txt" && grep -q "^REFUSE" "$OUT/b1.txt" || { bad "a wrong body was not refused at its equation (exit $rc)"; grep -E 'REFUSE|REALIZE' "$OUT/b1.txt" | head -3 | cut -c1-300; }
# 2. missing bound evidence: the hypothesis cited outside its branch, and dropped from the Fin.mk
sed 's/(dif h (< i (List.length xs))/(if (< i (List.length xs))/' v3/examples/path/path.shard > "$SCRATCH/no_branch.shard"
load "$SCRATCH/no_branch.shard" > "$OUT/b2a.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q "^REFUSE .*\.at .*unknown_constant: h" "$OUT/b2a.txt" || { bad "the hypothesis outside its branch was not refused (exit $rc)"; grep -E 'REFUSE' "$OUT/b2a.txt" | head -3 | cut -c1-300; }
sed 's/(Fin.mk i h)/(Fin.mk i)/' v3/examples/path/path.shard > "$SCRATCH/no_proof.shard"
load "$SCRATCH/no_proof.shard" > "$OUT/b2b.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q "^REFUSE .*\.at " "$OUT/b2b.txt" || { bad "the Fin.mk without its proof was not refused (exit $rc)"; grep -E 'REFUSE' "$OUT/b2b.txt" | head -3 | cut -c1-300; }
# 3. a mismatched revision: the order line of at_some names another declaration
cp -r "$STORE" "$SCRATCH/mistied"
sed -i 's/ examples\.path\.path\.at_some$/ examples.path.path.at_other/' "$SCRATCH/mistied/order"
"$EVAL" direct v3/kernel/verify_release.shard "$SCRATCH/mistied" "$EXPORT" "$INDEX" > "$OUT/b3.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^MISTIED' "$OUT/b3.txt" && grep -q 'mistied 1 ' "$OUT/b3.txt" || { bad "the mismatched order line was not MISTIED (exit $rc)"; grep -v '^ACCEPT' "$OUT/b3.txt" | tail -3 | cut -c1-300; }
# 4. a tampered result: at_some's proof replaced by its statement
cp -r "$STORE" "$SCRATCH/tampered"
f=$(grep ' examples\.path\.path\.at_some$' "$STORE/order" | cut -d' ' -f1)
sed -i -E 's/"type":([0-9]+),"value":[0-9]+/"type":\1,"value":\1/' "$SCRATCH/tampered/$f.ndjson"
"$EVAL" direct v3/kernel/verify_release.shard "$SCRATCH/tampered" "$EXPORT" "$INDEX" > "$OUT/b4.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^REJECT' "$OUT/b4.txt" && grep -q 'rejected 1 ' "$OUT/b4.txt" || { bad "the tampered proof was not REJECTED (exit $rc)"; grep -v '^ACCEPT' "$OUT/b4.txt" | tail -3 | cut -c1-300; }
# 5. an invalid raw argument
load --run $ENTRY "$MAIN" -- x > "$OUT/b5.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 6 ] && grep -q '^RUN: argument 1 not_a_nat: x' "$OUT/b5.txt" || { bad "a non-number was not refused at the entry (exit $rc)"; tail -2 "$OUT/b5.txt" | cut -c1-300; }

echo "path_test: $checks checks, $fail failed"
exit $fail
