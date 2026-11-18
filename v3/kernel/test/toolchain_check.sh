#!/usr/bin/env bash
# v3/kernel/test/toolchain_check.sh — the check of the toolchain's own sources. Every other
# load of a toolchain root reads it --unchecked (v3/build.sh, THE READING; the tests that run
# or dump one): this is the load without the flag. For every root of the inventory
# (v3/build.sh --list) the loader's BINARY loads the closure checked — each `fn` on the typed
# route first: its types checked, its definition K's where it has one — and
#   (1) the load refuses nothing and has no error (exit 0);
#   (2) the program it yields (`--dump`) is the bootstrap reader's text (`eval dump`), byte
#       for byte: the reading that elaborates gives the program the bootstrap runs (LAW 24).
# The checked load of the toolchain is too slow for the bootstrap (LAW 21: the loader
# interpreted, running the checker over the sources it is written in), so the two engines
# are compared on a part:
#   (3) for the roots whose closure is under V3_CHECK_SMALL declarations (default 400: eight
#       of them, each some seconds on the binary and a minute or two on the bootstrap) the
#       loader on the bootstrap exits the same and prints the same text.
# parity_test.sh is the same comparison for the reading of programs alone, on both engines.
# Needs v3/bin/load current (v3/build.sh --all): a missing or stale binary fails here, and
# nothing falls back to the bootstrap. V3_CHECK_JOBS roots at a time (default 8).
# Exit = the roots that failed.
set -u
cd "$(dirname "$0")/../../.."
RUST_EVAL=${RUST_EVAL:-./rust_bootstrap/target/release/eval}
[ -x "$RUST_EVAL" ] || { echo "toolchain_check: missing $RUST_EVAL (cargo build --release in rust_bootstrap/)"; exit 1; }
st=$(v3/build.sh --stamp load)
[ -x v3/bin/load ] && [ -n "$st" ] && [ "$(cat v3/bin/load.stamp 2>/dev/null)" = "$st" ] || { echo "toolchain_check: v3/bin/load is missing or stale (v3/build.sh --all)"; exit 1; }
read -r EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "toolchain_check: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
SMALL=${V3_CHECK_SMALL:-400}
JOBS=${V3_CHECK_JOBS:-8}
d=$(mktemp -d); trap 'rm -rf "$d"' EXIT
s=$(date +%s)
# one I NAME SRC: $d/I.res holds `ok DECLS BOTH` (BOTH 1 where the bootstrap loaded it too) or
# the failure's lines
one() {
  local i=$1 name=$2 src=$3 a=$d/$1.a b=$d/$1.b c=$d/$1.c kroot="" rc both=0
  "$RUST_EVAL" dump "$src" > "$a" 2>&1 || { { echo "toolchain_check: $name: eval dump failed"; head -3 "$a"; } > "$d/$i.res"; return; }
  # K's directory as a second root for a consumer of K's view outside it (parity_test.sh)
  if grep -q '^fn env_empty ' "$a" && [ "${src#v3/kernel/k/}" = "$src" ]; then kroot=v3/kernel/k; fi
  v3/bin/load --root v3 $INIT --dump "$src" $kroot > "$b" 2>&1; rc=$?
  if [ "$rc" != 0 ]; then
    { echo "toolchain_check: $name: the checked load exits $rc"; grep -m3 -E '^(REFUSE|ERROR|READ-ERROR|LOAD-ERROR|IMPL REFUSE|IMPL-ERROR) ' "$b" | cut -c1-300; } > "$d/$i.res"; return
  fi
  if ! cmp -s "$a" "$b"; then { echo "toolchain_check: $name: the checked load's program is not the bootstrap reader's"; diff "$a" "$b" | head -6 | cut -c1-200; } > "$d/$i.res"; return; fi
  if [ "$(wc -l < "$a")" -lt "$SMALL" ]; then
    both=1
    "$RUST_EVAL" direct v3/kernel/load.shard --root v3 $INIT --dump "$src" $kroot > "$c" 2>&1; rc=$?
    if [ "$rc" != 0 ] || ! cmp -s "$b" "$c"; then { echo "toolchain_check: $name: the loader on the bootstrap exits $rc, and its text is$(cmp -s "$b" "$c" && echo "" || echo " not") the binary's"; diff "$b" "$c" | head -6 | cut -c1-200; } > "$d/$i.res"; return; fi
  fi
  echo "ok $(wc -l < "$a") $both" > "$d/$i.res"
  rm -f "$a" "$b" "$c"
}
export -f one; export RUST_EVAL INIT d SMALL
v3/build.sh --list | awk '{print NR, $1, $2}' > "$d/list"
xargs -P "$JOBS" -L 1 bash -c 'one "$0" "$1" "$2"' < "$d/list"
n=0; decls=0; both=0; failed=0
while read -r i name src; do
  n=$((n+1))
  if read -r ok k b < "$d/$i.res" && [ "$ok" = ok ]; then decls=$((decls + k)); both=$((both + b))
  else cat "$d/$i.res" 2>/dev/null || echo "toolchain_check: $name: no result"; failed=$((failed+1)); fi
done < "$d/list"
echo "toolchain_check: $n roots loaded checked by the loader's binary, $decls declarations, $failed failed, in $(( $(date +%s) - s )) s"
[ "$failed" -eq 0 ] && echo "toolchain_check: nothing refused; every root's program is the bootstrap reader's text; $both roots under $SMALL declarations the same on the bootstrap"
exit $failed
