#!/usr/bin/env bash
# v3/kernel/test/index_test.sh — the Init index (LANGUAGE.md §8.4 slice 3.26 rule 1)
# over the two fixtures: built by initindex.shard, read back by its --verify (every
# record at its row with its kind and first ids, every name at its ordinal, the name
# table sorted without a duplicate), the record count K's stream admits (3523), and
# three tampered inputs refused: a name row's ordinal changed, a record row's length
# changed, the export a byte longer than the header says.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
cat v3/kernel/test/fixtures/init_prefix_int.ndjson v3/kernel/test/fixtures/init_prefix_str_tail.ndjson > "$T/export.ndjson"
fail=0
check() { if [ "$1" -eq 0 ]; then echo "PASS $2"; else echo "FAIL $2"; fail=1; fi; }
s=$(date +%s)
out=$("$EVAL" direct v3/kernel/initindex.shard "$T/export.ndjson" "$T/init.index" "$T/init.names" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-60) ($(( $(date +%s) - s )) s)"
check $rc "the index builds"
[ "$(head -c 23 "$T/init.index")" = "#INDEX records=00003523" ]; check $? "3523 records, the count K's stream admits"
[ "$(head -c 20 "$T/init.names")" = "#NAMES rows=00003985" ]; check $? "3985 declaration names"
grep -q "^Nat\.add  *00000[0-9]*\$" "$T/init.names"; check $? "Nat.add has its name row"
grep -q "^Nat  *00000[0-9]*\$" "$T/init.names"; check $? "Nat, an inductive block's type, has its name row"
grep -q "^Nat\.rec  *00000[0-9]*\$" "$T/init.names"; check $? "Nat.rec, the block's recursor, has its name row"
s=$(date +%s)
out=$("$EVAL" direct v3/kernel/initindex.shard --verify "$T/export.ndjson" "$T/init.index" "$T/init.names" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-100) ($(( $(date +%s) - s )) s)"
check $rc "the tables verify against the export"
# a name row's ordinal changed (row 1's ordinal field: bytes 209..216 of the row)
cp "$T/init.names" "$T/bad.names"; printf '00009999' | dd of="$T/bad.names" bs=1 seek=$((218 + 209)) conv=notrunc 2>/dev/null
out=$("$EVAL" direct v3/kernel/initindex.shard --verify "$T/export.ndjson" "$T/init.index" "$T/bad.names" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-100)"
[ $rc -ne 0 ] && echo "$out" | grep -q "is at ordinal 9999, the export says"; check $? "a name row's ordinal changed: refused by name"
# a record row's length changed (row 0's length field: bytes 24..33 of the row)
cp "$T/init.index" "$T/bad.index"; printf '0000003454' | dd of="$T/bad.index" bs=1 seek=$((62 + 24)) conv=notrunc 2>/dev/null
out=$("$EVAL" direct v3/kernel/initindex.shard --verify "$T/export.ndjson" "$T/bad.index" "$T/init.names" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-100)"
[ $rc -ne 0 ] && echo "$out" | grep -q "row differs at ordinal 0"; check $? "a record row's length changed: refused with both rows"
# the export a byte longer
cp "$T/export.ndjson" "$T/bad.ndjson"; printf '\n' >> "$T/bad.ndjson"
out=$("$EVAL" direct v3/kernel/initindex.shard --verify "$T/bad.ndjson" "$T/init.index" "$T/init.names" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-100)"
[ $rc -ne 0 ] && echo "$out" | grep -q "the export is not 14328703 bytes"; check $? "another export: refused by its byte count"
exit $fail
