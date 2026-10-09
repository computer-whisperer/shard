#!/usr/bin/env bash
# v3/kernel/test/index_test.sh — the Init index (LANGUAGE.md §8.4 slice 3.26 rule 1)
# over the export's first 20,000 lines: built by initindex.shard, read back by its
# --verify (every record at its row with its kind and first ids, every name at its
# ordinal and by its name id, the name table sorted without a duplicate, the sizes the
# header's), the record and name counts, a declaration's row, an inductive block's type
# and recursor, and four tampered inputs refused: a name row's ordinal changed, a record
# row's length changed, a declaration row's ordinal changed, the export a byte longer
# than the header says.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
DIR=${V3_EXPORT_DIR:-.shard-cache/v3-export}
[ -s "$DIR/init.ndjson" ] || { echo "index_test: no export at $DIR/init.ndjson (v3/export.sh)"; exit 1; }
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
head -n 20000 "$DIR/init.ndjson" > "$T/export.ndjson"
fail=0
check() { if [ "$1" -eq 0 ]; then echo "PASS $2"; else echo "FAIL $2"; fail=1; fi; }
s=$(date +%s)
out=$("$EVAL" direct v3/kernel/initindex.shard "$T/export.ndjson" "$T/init.index" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-80) ($(( $(date +%s) - s )) s)"
check $rc "the index builds"
hdr=$(head -c 86 "$T/init.index")
[ "$hdr" = "#INDEX records=00000519 bytes=000001143143 meta=00000173 names=00000691 decls=00003153" ]; check $? "the header: 519 records, 691 names, 3153 name ids over 1,143,143 bytes"
R=519; N=691
names() { tail -c +$((128 + 62*R + 1)) "$T/init.index" | head -c $((218*N)); }
names | grep -q "^Nat\.add  *00000[0-9]*\$"; check $? "Nat.add has its name row"
names | grep -q "^Nat  *00000[0-9]*\$"; check $? "Nat, an inductive block's type, has its name row"
names | grep -q "^Nat\.rec  *00000[0-9]*\$"; check $? "Nat.rec, the block's recursor, has its name row"
s=$(date +%s)
out=$("$EVAL" direct v3/kernel/initindex.shard --verify "$T/export.ndjson" "$T/init.index" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-100) ($(( $(date +%s) - s )) s)"
check $rc "the tables verify against the export"
tamper() { cp "$T/init.index" "$T/bad.index"; printf '%s' "$1" | dd of="$T/bad.index" bs=1 seek="$2" conv=notrunc 2>/dev/null; }
# a name row's ordinal changed (row 1's ordinal field: bytes 209..216 of the row)
tamper 00009999 $((128 + 62*R + 218 + 209))
out=$("$EVAL" direct v3/kernel/initindex.shard --verify "$T/export.ndjson" "$T/bad.index" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-100)"
[ $rc -ne 0 ] && echo "$out" | grep -q "is at ordinal 9999, the export says"; check $? "a name row's ordinal changed: refused by name"
# a record row's length changed (row 0's length field: bytes 24..33 of the row)
tamper 0000003454 $((128 + 24))
out=$("$EVAL" direct v3/kernel/initindex.shard --verify "$T/export.ndjson" "$T/bad.index" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-100)"
[ $rc -ne 0 ] && echo "$out" | grep -q "row differs at ordinal 0"; check $? "a record row's length changed: refused with both rows"
# a declaration row's ordinal changed: Subtype is name id 1, the first inductive block's type
tamper 00000077 $((128 + 62*R + 218*N + 9))
out=$("$EVAL" direct v3/kernel/initindex.shard --verify "$T/export.ndjson" "$T/bad.index" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-100)"
[ $rc -ne 0 ] && echo "$out" | grep -q "Subtype declares ordinal 77 by its name id, the export says 0"; check $? "a declaration row's ordinal changed: refused by name and id"
# the export a byte longer
cp "$T/export.ndjson" "$T/bad.ndjson"; printf '\n' >> "$T/bad.ndjson"
out=$("$EVAL" direct v3/kernel/initindex.shard --verify "$T/bad.ndjson" "$T/init.index" 2>&1); rc=$?
echo "  $(echo "$out" | cut -c1-100)"
[ $rc -ne 0 ] && echo "$out" | grep -q "the export is not 1143143 bytes"; check $? "another export: refused by its byte count"
exit $fail
