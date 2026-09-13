#!/usr/bin/env bash
# Route 2's byte-tie (v3/LANGUAGE.md §6.7, §10 item 2; FOUNDATION §9.1): the
# T0 driver's closure loaded from the package root under the toolchain
# profile and run under `ev` — K interpreted by ev, hosted on the bootstrap —
# on the committed export prefix, its output and exit code byte-identical to
# route 3's (the bootstrap interpreting K directly).
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
FIX=v3/kernel/test/fixtures/init_prefix_3000.ndjson
r3=$(mktemp); r2=$(mktemp); trap 'rm -f "$r3" "$r2"' EXIT
"$EVAL" direct v3/kernel/t0.shard -a "$FIX" > "$r3" 2>&1; rc3=$?
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 --run kernel.t0.main v3/kernel/t0.shard -- -a "$FIX" > "$r2" 2>&1; rc2=$?
e=$(date +%s)
echo "route2_test: route 3 exit $rc3, route 2 exit $rc2 in $((e-s)) s, $(wc -l < "$r2") lines"
[ "$rc3" -eq 0 ] || { echo "route 3 failed"; tail -5 "$r3"; exit 1; }
[ "$rc2" -eq 0 ] || { echo "route 2 failed"; tail -5 "$r2"; exit 1; }
cmp "$r3" "$r2" || { echo "route2_test: outputs differ"; diff "$r3" "$r2" | head -20; exit 1; }
grep -q 'rejected 0  exhausted 0  mismatched 0  unsupported 0' "$r2" || exit 1
echo "route2_test: byte-identical"
