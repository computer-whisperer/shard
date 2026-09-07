#!/usr/bin/env bash
# the T0 driver on the committed export prefix: every declaration accepted, none mismatched
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
out=$("$EVAL" direct v3/kernel/t0.shard v3/kernel/test/fixtures/init_prefix_3000.ndjson 2>&1); rc=$?
echo "$out" | tail -1
[ "$rc" -eq 0 ] || { echo "$out" | grep -v '^ACCEPT' | tail -20; exit 1; }
echo "$out" | grep -q 'rejected 0  exhausted 0  mismatched 0  unsupported 0' || exit 1
