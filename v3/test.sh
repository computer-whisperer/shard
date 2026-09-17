#!/usr/bin/env bash
# v3/test.sh — run every V3 test entrypoint through the Rust bootstrap
# (route 3, FOUNDATION §9.1): `eval direct` loads each test's flat import
# closure and runs its main; the exit code is its failure count. The
# bootstrap resolves flat and enforces no seal — the V3 loader is the gate
# for that, through parity (parity_test.sh) and, for K's tests, through
# k_clients_test.sh, which loads and runs them under the V3 loader.
set -u
cd "$(dirname "$0")/.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
total=0; failed=0
for t in v3/kernel/test/*_test.shard v3/kernel/k/test/*_test.shard; do
  out=$("$EVAL" direct "$t" 2>&1); rc=$?
  total=$((total+1))
  if [ "$rc" -ne 0 ]; then failed=$((failed+1)); echo "== $t: exit $rc"; echo "$out" | grep -v '^PASS' | tail -20; fi
done
for t in v3/kernel/test/*_test.sh; do
  total=$((total+1))
  if ! "$t"; then failed=$((failed+1)); echo "== $t FAILED"; fi
done
echo "v3 tests: $total entrypoints, $failed failed"
exit $failed
