#!/usr/bin/env bash
# v3/test.sh — run every V3 test entrypoint through the Rust bootstrap
# (route 3, FOUNDATION §9.1): `eval direct` loads each test's flat import
# closure and runs its main; the exit code is its failure count. The
# bootstrap resolves flat and enforces no seal — the V3 loader is the gate
# for that, through parity (parity_test.sh) and, for K's tests, through
# k_clients_test.sh, which loads and runs them under the V3 loader.
#
# The entrypoints are independent processes over read-only sources (each
# shell test keeps its scratch in its own mktemp directory), so they run
# V3_TEST_JOBS at a time (default 8; 1 is the old sequential run). Each one's
# output is held and printed in the list's order with its seconds, so the log
# reads as the sequential run's did.
set -u
cd "$(dirname "$0")/.."
export EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
# the Init index once (kernel/test/init_index.sh; slice 3.26): the drivers below load
# Init on demand from the pinned export through it. A receipt (slice 3.23) is a speed
# lever a T0 run over the export writes; set INIT_RECEIPT to use one
read _ _ < <(v3/kernel/test/init_index.sh) || { echo "v3/test.sh: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
export INIT_RECEIPT=${INIT_RECEIPT:-}
JOBS=${V3_TEST_JOBS:-8}
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
run_one() {   # ENTRYPOINT INDEX: its output, exit code and seconds under $OUT
  local t=$1 i=$2 s rc
  s=$(date +%s)
  case $t in
    */loader_pins_test.shard) "$EVAL" direct "$t" ${INIT_RECEIPT:+--init-receipt "$INIT_RECEIPT"} > "$OUT/$i.out" 2>&1; rc=$? ;;
    *.shard) "$EVAL" direct "$t" > "$OUT/$i.out" 2>&1; rc=$? ;;
    *)       "$t" > "$OUT/$i.out" 2>&1; rc=$? ;;
  esac
  echo "$rc $(( $(date +%s) - s ))" > "$OUT/$i.rc"
}
export -f run_one
export OUT
tests=(v3/kernel/test/*_test.shard v3/kernel/k/test/*_test.shard v3/kernel/test/*_test.sh)
for i in "${!tests[@]}"; do printf '%s %s\n' "${tests[$i]}" "$i"; done | xargs -P "$JOBS" -L 1 bash -c 'run_one "$0" "$1"'
total=0; failed=0
for i in "${!tests[@]}"; do
  t=${tests[$i]}; total=$((total+1))
  if [ -s "$OUT/$i.rc" ]; then read -r rc secs < "$OUT/$i.rc"; else rc=255; secs=0; fi
  case $t in
    *.shard) if [ "$rc" -ne 0 ]; then failed=$((failed+1)); echo "== $t: exit $rc (${secs} s)"; grep -v '^PASS' "$OUT/$i.out" | tail -20; fi ;;
    *)       cat "$OUT/$i.out"; if [ "$rc" -ne 0 ]; then failed=$((failed+1)); echo "== $t FAILED (${secs} s)"; fi ;;
  esac
  [ "$secs" -ge 30 ] && echo "   ($t: ${secs} s)"
done
echo "v3 tests: $total entrypoints, $failed failed"
exit $failed
