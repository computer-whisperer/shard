#!/usr/bin/env bash
# v3/kernel/test/loader_pins_list_test.sh — the loader pins the harness runs are the loader pins
# there are. v3/kernel/test/loader_pins_test.shard replays the cases its `cases` list names;
# this test compares that list with the directories of v3/pins/loader/: a directory the list
# lacks is a case nothing runs, a name without a directory is a case that cannot be read, and
# a name listed twice runs twice. Exit code = the number of names out of place.
set -u
cd "$(dirname "$0")/../../.."
HARNESS=v3/kernel/test/loader_pins_test.shard
PINS=v3/pins/loader
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
fail=0
bad() { echo "loader_pins_list_test: $*"; fail=$((fail+1)); }

# the strings of the `cases` function: from its head to the next top-level form
awk '/^\(fn cases \(\)/{on=1; print; next} on && /^\(/{on=0} on' "$HARNESS" | sed 's/;.*//' | grep -o '"[^"]*"' | tr -d '"' > "$OUT/listed.txt"
LC_ALL=C sort "$OUT/listed.txt" > "$OUT/listed.sorted"
for d in "$PINS"/*/; do basename "$d"; done | LC_ALL=C sort > "$OUT/dirs.sorted"

[ -s "$OUT/listed.txt" ] || bad "no case read from the cases list of $HARNESS"
[ -s "$OUT/dirs.sorted" ] || bad "no directory under $PINS"
while read -r c; do bad "$PINS/$c is not in the cases list of $HARNESS: nothing runs it"; done < <(LC_ALL=C comm -13 <(uniq "$OUT/listed.sorted") "$OUT/dirs.sorted")
while read -r c; do bad "the cases list names $c, and $PINS/$c is no directory"; done < <(LC_ALL=C comm -23 <(uniq "$OUT/listed.sorted") "$OUT/dirs.sorted")
while read -r c; do bad "the cases list names $c twice"; done < <(uniq -d "$OUT/listed.sorted")
while read -r c; do [ -f "$PINS/$c/main.shard" ] || bad "$PINS/$c has no main.shard"; done < "$OUT/dirs.sorted"

echo "loader_pins_list_test: $(wc -l < "$OUT/dirs.sorted") directories, $(wc -l < "$OUT/listed.txt") listed, $fail out of place"
exit $fail
