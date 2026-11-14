#!/usr/bin/env bash
# v3/kernel/test/route2_test.sh — K under `ev` against K run directly: the replay driver's
# closure (kernel/t0.shard) loaded by the V3 loader from the package root and run under `ev`
# on the export's first 3,000 lines, its output and exit code byte-identical to the
# engine's direct run of the driver.
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
DIR=${V3_EXPORT_DIR:-.shard-cache/v3-export}
[ -s "$DIR/init.ndjson" ] || { echo "route2_test: no export at $DIR/init.ndjson (v3/export.sh)"; exit 1; }
r3=$(mktemp); r2=$(mktemp); FIX=$(mktemp); trap 'rm -f "$r3" "$r2" "$FIX"' EXIT
head -n 3000 "$DIR/init.ndjson" > "$FIX"
"$EVAL" direct v3/kernel/t0.shard -a "$FIX" > "$r3" 2>&1; rc3=$?
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 --run kernel.t0.main v3/kernel/t0.shard v3/kernel/k -- -a "$FIX" > "$r2" 2>&1; rc2=$?
e=$(date +%s)
echo "route2_test: the direct run exit $rc3, the run under ev exit $rc2 in $((e-s)) s, $(wc -l < "$r2") lines"
[ "$rc3" -eq 0 ] || { echo "route2_test: the direct run failed"; tail -5 "$r3"; exit 1; }
[ "$rc2" -eq 0 ] || { echo "route2_test: the run under ev failed"; tail -5 "$r2"; exit 1; }
cmp "$r3" "$r2" || { echo "route2_test: outputs differ"; diff "$r3" "$r2" | head -20; exit 1; }
grep -q 'rejected 0  exhausted 0  mismatched 0  unsupported 0' "$r2" || exit 1
echo "route2_test: byte-identical"
