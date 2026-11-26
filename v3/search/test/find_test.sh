#!/usr/bin/env bash
# v3/search/test/find_test.sh — the search's gate: `find` (v3/search/find.shard) over the
# problems of problems.txt, one run and one load of the three task files, on the engine under
# test; its records are expected.txt byte for byte. What that holds:
#   (1) UNKNOWN VALUES: a function inverted at a point (a list by its reversal; a stack
#       machine's program by its result, and with its start), every answer of a size, a goal
#       with no answer under a size bound (`bounded`) and one with none at all (`none`).
#   (2) UNKNOWN CODE: a sketch's hole functions found from examples (reverse with an
#       accumulator, append, a constant), as source text.
#   (3) THE TWO WALKS: the same problems by generate and test (`--enum`) give the same
#       answers; the work of both is in the records, so a change of either walk shows here.
#   (4) THE CLAIMS: every answer printed was replayed under `ev` (task.shard); a budget that
#       ends first is `gave_up`; a problem that does not read is one FIND-ERROR line and the
#       run's exit is 2.
#   (5) THE MACHINE IS `ev` WHERE NOTHING IS UNKNOWN (`--closed`): ten closed functions —
#       recursion, `let`, rows, a tree, `Nat.sub`, a negative integer, a run out of fuel, an
#       extern — end alike under both: ten SAME lines, no DIFFER.
# After a change that moves the records on purpose: V3_FIND_WRITE=1 writes expected.txt anew
# (review the difference). Exit = the checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read -r EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "find_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
T=v3/search/test
out=$(mktemp); trap 'rm -f "$out"' EXIT
fail=0; checks=0
bad() { echo "find_test: $*"; fail=$((fail+1)); }
s=$(date +%s)
"$EVAL" direct v3/search/find.shard --root v3 --init "$EXPORT" --init-index "$INDEX" \
  --batch $T/problems.txt $T/tasks/inv.shard $T/tasks/sketch.shard $T/tasks/closed.shard > "$out" 2>&1; rc=$?
[ "${V3_FIND_WRITE:-0}" = 1 ] && { cp "$out" $T/expected.txt; echo "find_test: $T/expected.txt written"; }
checks=$((checks+1)); [ "$rc" = 2 ] || bad "the run exits $rc, not 2 (the batch ends in problems that do not read)"
checks=$((checks+1)); cmp -s "$out" $T/expected.txt || { bad "the records are not $T/expected.txt:"; diff $T/expected.txt "$out" | head -12 | cut -c1-200; }
checks=$((checks+1)); n=$(grep -c '^SAME ' "$out"); [ "$n" = 10 ] || bad "$n SAME lines, not 10"
checks=$((checks+1)); ! grep -q '^DIFFER ' "$out" || bad "the narrowing machine and ev differ: $(grep -m1 '^DIFFER ' "$out" | cut -c1-200)"
checks=$((checks+1)); n=$(grep -c '^FIND-ERROR ' "$out"); [ "$n" = 4 ] || bad "$n FIND-ERROR lines, not 4"
checks=$((checks+1)); n=$(grep -c '^FIND: found ' "$out"); [ "$n" = 11 ] || bad "$n problems found, not 11"
# the two walks answer alike: each --enum problem's FOUND block is its lazy twin's
checks=$((checks+1))
blocks() { awk -v want="$1" '/^FIND goal=/ { on = (($0 ~ / enum$/) == want); next } /^FIND: / { on = 0 } on' "$out"; }
[ -z "$(blocks 1 | sort -u | comm -23 - <(blocks 0 | sort -u))" ] || bad "generate and test printed an answer the lazy walk did not"
echo "find_test: $checks checks, $fail failed, in $(( $(date +%s) - s )) s"
exit $fail
