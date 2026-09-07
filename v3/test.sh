#!/usr/bin/env bash
# v3/test.sh — run every V3 test entrypoint through the Rust bootstrap
# (route 3, FOUNDATION §9.1): `eval direct` loads each test's flat import
# closure and runs its main; the exit code is its failure count. This is
# the gate for the toolchain profile until V3's own checker exists (phase 2).
set -u
cd "$(dirname "$0")/.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
total=0; failed=0
for t in v3/kernel/test/*_test.shard; do
  out=$("$EVAL" direct "$t" 2>&1); rc=$?
  total=$((total+1))
  if [ "$rc" -ne 0 ]; then failed=$((failed+1)); echo "== $t: exit $rc"; echo "$out" | grep -v '^PASS' | tail -20; fi
done
echo "v3 tests: $total entrypoints, $failed failed"
exit $failed
