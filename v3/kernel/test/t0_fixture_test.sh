#!/usr/bin/env bash
# the T0 driver on the export's first 3,000 lines: every declaration accepted,
# none mismatched, and every admitted constant's axiom closure identical to
# the oracle's (init.axioms = v3/axioms.lean's lines for the whole export);
# and the same lines with one inductive's exported `isRec` altered: what K
# generated differs from the record, MISMATCH names the field and the type,
# the count is 1 and the exit is not 0
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
DIR=${V3_EXPORT_DIR:-.shard-cache/v3-export}
[ -s "$DIR/init.ndjson" ] || { echo "t0_fixture_test: no export at $DIR/init.ndjson (v3/export.sh)"; exit 1; }
[ -s "$DIR/init.axioms" ] || { echo "t0_fixture_test: no oracle at $DIR/init.axioms (v3/export.sh)"; exit 1; }
FIX=$(mktemp); log=$(mktemp); TAMPER=$(mktemp); trap 'rm -f "$FIX" "$log" "$TAMPER"' EXIT
head -n 3000 "$DIR/init.ndjson" > "$FIX"
out=$("$EVAL" direct v3/kernel/t0.shard -a "$FIX" 2>&1); rc=$?
echo "$out" | tail -1
[ "$rc" -eq 0 ] || { echo "$out" | grep -v '^ACCEPT\|^AX ' | tail -20; exit 1; }
echo "$out" | grep -q 'rejected 0  exhausted 0  mismatched 0  unsupported 0' || exit 1
echo "$out" > "$log"
sed '0,/"isRec":false/s//"isRec":true/' "$FIX" > "$TAMPER"
bad=$("$EVAL" direct v3/kernel/t0.shard "$TAMPER" 2>&1); rc=$?
[ "$rc" -ne 0 ] && echo "$bad" | grep -q '^MISMATCH isRec ' && echo "$bad" | grep -q 'mismatched 1  unsupported 0' || { echo "t0_fixture_test: an inductive record altered in the export is not a MISMATCH (exit $rc)"; echo "$bad" | grep -v '^ACCEPT' | tail -3; exit 1; }
echo "t0_fixture_test: an inductive record altered in the export is $(echo "$bad" | grep '^MISMATCH ' | head -1), exit $rc"
v3/t0_axioms_cmp.sh "$log" "$DIR/init.axioms"
