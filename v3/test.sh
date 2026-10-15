#!/usr/bin/env bash
# v3/test.sh — run every V3 test entrypoint on an engine. The Rust bootstrap
# (route 3, FOUNDATION §9.1: `eval direct` loads each test's flat import
# closure and runs its main; the exit code is its failure count) is the
# authority — V3_ENGINE=bootstrap, the default. V3_ENGINE=native runs the same
# entrypoints on the binaries route 1 built: the shim v3/bin/eval answers
# `direct` with v3/bin/NAME when its stamp is current and fails loudly
# otherwise, never a silent fallback (LANGUAGE.md §8.6 slice 4.1 rule 4). The
# bootstrap resolves flat and enforces no seal — the V3 loader is the gate for
# that, through parity (parity_test.sh) and, for K's tests, through
# k_clients_test.sh, which loads and runs them under the V3 loader.
#
# The entrypoints are independent processes over read-only sources (each
# shell test keeps its scratch in its own mktemp directory), so they run
# V3_TEST_JOBS at a time (default 8; 1 is the old sequential run). Each one's
# output is held and printed in the list's order with its seconds, so the log
# reads as the sequential run's did. V3_TEST_OUT=DIR keeps every entrypoint's
# output, exit status and seconds (DIR/NAME.out, .rc, .secs; DIR/engine,
# DIR/wall) for kernel/test/suite_tie.sh, which compares a bootstrap run and a
# native run entrypoint by entrypoint — CI's suite_tie job over the v3 and
# native jobs' runs; the native run's time is the measurement (rule 7).
#
# The native toolchain's byte-tie gate (kernel/test/native_tie_test.sh; rule 3)
# is the last entrypoint under V3_NATIVE=1, after `v3/build.sh --all`: it needs
# every binary of the inventory present and current and fails otherwise.
# Without V3_NATIVE it is not run and the last line says so; CI's `native`
# job runs it beside this suite.
set -u
cd "$(dirname "$0")/.."
ENGINE=${V3_ENGINE:-bootstrap}
case $ENGINE in
  bootstrap) export EVAL=${EVAL:-./rust_bootstrap/target/release/eval};;
  native) [ -x v3/bin/eval ] || { echo "v3/test.sh: V3_ENGINE=native, but v3/bin/eval is missing (v3/build.sh --all)"; exit 1; }; export EVAL=v3/bin/eval;;
  *) echo "v3/test.sh: V3_ENGINE=$ENGINE; bootstrap (the default) or native"; exit 1;;
esac
# the Init index once (kernel/test/init_index.sh; slice 3.26): the drivers below load
# Init on demand from the pinned export through it
read _ _ < <(v3/kernel/test/init_index.sh) || { echo "v3/test.sh: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
JOBS=${V3_TEST_JOBS:-8}
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
KEEP=${V3_TEST_OUT:-}
if [ -n "$KEEP" ]; then
  mkdir -p "$KEEP"; find "$KEEP" -maxdepth 1 -type f \( -name '*.out' -o -name '*.rc' -o -name '*.secs' -o -name engine -o -name wall \) -delete
  echo "$ENGINE" > "$KEEP/engine"
fi
s0=$(date +%s)
run_one() {   # ENTRYPOINT INDEX: its output, exit code and seconds under $OUT
  local t=$1 i=$2 s rc
  s=$(date +%s)
  case $t in
    *.shard) "$EVAL" direct "$t" > "$OUT/$i.out" 2>&1; rc=$? ;;
    *)       "$t" > "$OUT/$i.out" 2>&1; rc=$? ;;
  esac
  echo "$rc $(( $(date +%s) - s ))" > "$OUT/$i.rc"
}
export -f run_one
export OUT
tests=(v3/kernel/test/*_test.shard v3/kernel/k/test/*_test.shard $(ls v3/kernel/test/*_test.sh | grep -v native_tie_test.sh))
[ "${V3_NATIVE:-0}" = 1 ] && tests+=(v3/kernel/test/native_tie_test.sh)
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
  if [ -n "$KEEP" ]; then
    b=$(basename "$t"); b=${b%.*}
    [ ! -e "$KEEP/$b.out" ] || { echo "v3/test.sh: two entrypoints keep the name $b"; failed=$((failed+1)); }
    cp "$OUT/$i.out" "$KEEP/$b.out" 2>/dev/null || : > "$KEEP/$b.out"
    echo "$rc" > "$KEEP/$b.rc"; echo "$secs" > "$KEEP/$b.secs"
  fi
done
[ -z "$KEEP" ] || echo $(( $(date +%s) - s0 )) > "$KEEP/wall"
if [ "${V3_NATIVE:-0}" = 1 ]; then echo "v3 tests on the $ENGINE: $total entrypoints, $failed failed (the native tie among them)"
else echo "v3 tests on the $ENGINE: $total entrypoints, $failed failed; the native tie not run (V3_NATIVE=1 after v3/build.sh --all; CI's native job)"; fi
exit $failed
