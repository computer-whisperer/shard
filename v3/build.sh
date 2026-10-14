#!/usr/bin/env bash
# v3/build.sh — route 1 (FOUNDATION §9.1): a toolchain closure as the V3 loader accepted
# it, projected by `load.shard --emit` (kernel/emit.shard) into the dialect the old tree's
# chain reads, then tools/lower → tools/codegen → cc into a native binary under v3/bin/
# (LANGUAGE.md §8.6 slice 4.1 rules 1, 2 and 8). The binaries are ENGINES: route 1's proof
# is still to come, so the Rust interpreter (route 3) stays the authority and the byte-ties
# (v3/t0_full.sh; kernel/test/native_tie_test.sh) tie the two.
#
#   v3/build.sh [SRC [OUT]] [--root DIR] [EXTRA…]   one root: SRC's closure (default
#                        v3/kernel/t0.shard) into OUT (default v3/bin/STEM); --root the loader's
#                        root (default v3); EXTRA… further root files for the load (K's directory)
#   v3/build.sh --all    the inventory below, V3_BUILD_JOBS roots at a time (default 8) with at most
#                        V3_HEAVY_JOBS of them in the chain's two heavy steps at once (default 2: on a
#                        driver-sized closure the lowerer holds 12 GB and the generator 20–30 GB);
#                        a binary whose stamp is current is kept; exit = the roots that failed
#   v3/build.sh --stamp NAME   the stamp v3/bin/NAME would carry now (empty: never emitted)
#   v3/build.sh --list   the inventory: NAME SRC
#
# THE INVENTORY (rule 8): the five drivers, calc_harness, the 18 public harnesses of
# kernel/test/ and the 5 of k/test/ — 29 roots, parity's list (kernel/test/parity_test.sh).
# A root outside K's directory whose closure consumes K's view is loaded with v3/kernel/k as a
# second root, so the directory's own check links the implementation into the program (as
# parity and k_clients_test do; the bootstrap's dump shows env_empty when K is in the closure).
# Every shell entrypoint's invocation of an engine has its disposition here:
#   eval direct v3/kernel/X.shard ARGS…        -> v3/bin/X ARGS…      (the drivers and the harnesses: this inventory)
#   eval direct load.shard … --run M.FN F…     -> v3/bin/load …       (the loader's own operation: k_clients_test, wire_test, define_test, calc_main_test, route2_test)
#   eval direct load.shard … --dump F          -> v3/bin/load …       (the loader's own operation: template_test, define_test, parity's V3 side)
#   eval dump F                                -> the bootstrap's     (parity's second reader; never compiled)
#   BOOT run tools/lower|codegen …             -> bin/shard_eval or the bootstrap (the old tree's chain; its boot)
#
# THE STAMP (rule 8): sha256 over the root's closure sources (the emit's `sources` list),
# the emitter's closure (v3/kernel/*.shard, k/*.shard, k's view), the chain (tools/lower/
# lower.shard, tools/codegen/codegen.shard, tools/codegen/rt.h) and the compiler's flags —
# v3/bin/NAME.stamp beside the binary; the tie and the shim refuse a stale one. Generated
# files live under v3/bin/ only (emit/NAME/ with map and sources, NAME.low.shard,
# NAME.low.shard.c, NAME, NAME.stamp, NAME.build.log); an authored file is never written.
#
# The tools are named REPO-RELATIVE from the repo root: kernel/resolve.shard grants `core`
# identity only to the exact path kernel/stdlib, so a tool named by an absolute path imports
# a non-core prelude and its every Pair pattern fails (#41). Boot engine: bin/shard_eval
# when present (seconds), else the Rust interpreter (minutes). v3/bin/ is derived and untracked.
set -uo pipefail
cd "$(dirname "$0")/.."
RUST_EVAL=${RUST_EVAL:-./rust_bootstrap/target/release/eval}
BOOT=bin/shard_eval
CFLAGS="-O2"
CODEGEN_JOBS=${V3_HEAVY_JOBS:-${V3_CODEGEN_JOBS:-2}}
[ -x "$RUST_EVAL" ] || { echo "missing $RUST_EVAL (cargo build --release in rust_bootstrap/)"; exit 1; }
if [ ! -x "$BOOT" ]; then
  echo "== boot engine: the Rust interpreter (bin/shard_eval absent; minutes, not seconds)"
  BOOT=$RUST_EVAL
fi

inventory() {   # NAME SRC per root, parity's list
  for f in v3/kernel/load.shard v3/kernel/prove.shard v3/kernel/verify_release.shard v3/kernel/t0.shard v3/kernel/initindex.shard \
           v3/kernel/test/calc_harness.shard v3/kernel/test/*_test.shard v3/kernel/k/test/*_test.shard; do
    echo "$(basename "$f" .shard) $f"
  done
}
# the stamp of NAME from its emitted sources list; empty when never emitted
stamp_of() {
  local srcs=v3/bin/emit/$1/sources root
  [ -s "$srcs" ] || { echo ""; return; }
  root=$(cat "v3/bin/emit/$1/root" 2>/dev/null || echo v3)
  { awk -v r="$root" '{print r "/" $2}' "$srcs" | sort | xargs cat 2>/dev/null
    cat v3/kernel/*.shard v3/kernel/k/*.shard v3/kernel/k/mod.req.shard
    cat tools/lower/lower.shard tools/codegen/codegen.shard tools/codegen/rt.h
    echo "cc $CFLAGS"; } | sha256sum | cut -c1-16
}
# K's directory as a second root when the closure consumes its view from outside it
extra_roots() {
  case $1 in v3/kernel/k/*) return;; esac
  # grep reads to the end: under pipefail a `-q` that closes the pipe early would fail the dump
  if "$RUST_EVAL" dump "$1" 2>/dev/null | grep '^fn env_empty ' > /dev/null; then echo v3/kernel/k; fi
}
secs() { echo $(( $(date +%s) - $1 )); }
# the heavy steps' slots: at most CODEGEN_JOBS lowerings and code generations at once (their
# resident sets are the build's memory); the same slots for both steps
codegen_acquire() {
  local i fd
  while :; do
    for i in $(seq 1 "$CODEGEN_JOBS"); do
      exec {fd}> "v3/bin/.codegen.$i.lock"
      if flock -n "$fd"; then CODEGEN_FD=$fd; return 0; fi
      exec {fd}>&-
    done
    sleep 5
  done
}
codegen_release() { flock -u "$CODEGEN_FD"; exec {CODEGEN_FD}>&-; }
# the generator under /usr/bin/time when present: its peak resident set into the log
run_codegen() {
  if [ -x /usr/bin/time ]; then /usr/bin/time -f 'codegen peak RSS %M kB' -a -o "$1" "$BOOT" run tools/codegen/codegen.shard "$2" "$3"
  else "$BOOT" run tools/codegen/codegen.shard "$2" "$3"; fi
}
# build_one NAME SRC OUT ROOT EXTRA…: emit, lower, codegen, cc; the stamp last
build_one() {
  local name=$1 src=$2 out=$3 root=$4; shift 4
  local emit=v3/bin/emit/$name log=v3/bin/$name.build.log low=v3/bin/$name.low.shard s t_emit t_lower t_codegen t_cc entry rc
  mkdir -p v3/bin; rm -rf "$emit"; mkdir -p "$emit"; : > "$log"
  s=$(date +%s)
  "$RUST_EVAL" direct v3/kernel/load.shard --root "$root" --emit "$emit" "$src" "$@" > "$log" 2>&1; rc=$?
  if [ "$rc" != 0 ]; then
    echo "FAIL: $name: the emit (exit $rc); $(grep -m1 '^EMIT: \|^LOAD: ' "$log" | cut -c1-200)"; grep -E '^(REFUSE|ERROR|EMIT)' "$log" | head -3 | cut -c1-300; return 1
  fi
  t_emit=$(secs $s); echo "$root" > "$emit/root"
  entry=$(sed -n 's/^EMIT: .* entry=\([^ ]*\).*/\1/p' "$log" | tail -1)
  [ -n "$entry" ] && [ -s "$emit/$entry" ] || { echo "FAIL: $name: no entry file after the emit"; tail -2 "$log" | cut -c1-200; return 1; }
  codegen_acquire; s=$(date +%s)
  "$BOOT" run tools/lower/lower.shard "$emit/$entry" "$low" >> "$log" 2>&1 && [ -s "$low" ] || { codegen_release; echo "FAIL: $name: the lowerer"; tail -3 "$log" | cut -c1-300; return 1; }
  t_lower=$(secs $s); codegen_release
  codegen_acquire; s=$(date +%s)
  run_codegen "$log" "$low" "$low.c" >> "$log" 2>&1 && [ -s "$low.c" ] || { codegen_release; echo "FAIL: $name: the codegen"; tail -3 "$log" | cut -c1-300; return 1; }
  t_codegen=$(secs $s); codegen_release; s=$(date +%s)
  cc $CFLAGS -o "$out" "$low.c" -I tools/codegen >> "$log" 2>&1 || { echo "FAIL: $name: cc"; tail -3 "$log" | cut -c1-300; return 1; }
  t_cc=$(secs $s)
  stamp_of "$name" > "$out.stamp"
  echo "OK: $out ($(sed -n 's/^EMIT: [^ ]* entry=[^ ]* //p' "$log" | tail -1); emit $t_emit s, lower $t_lower s, codegen $t_codegen s$(sed -n 's/^codegen peak RSS \([0-9]*\) kB/ at \1 kB/p' "$log" | tail -1), cc $t_cc s)"
}
export -f stamp_of extra_roots secs build_one inventory codegen_acquire codegen_release run_codegen
export RUST_EVAL BOOT CFLAGS CODEGEN_JOBS

case ${1:-} in
  --list) inventory; exit 0;;
  --stamp) stamp_of "$2"; exit 0;;
  --one)   # NAME SRC (the --all worker): current → kept, else built
    name=$2; src=$3
    if [ -x "v3/bin/$name" ] && [ -n "$(stamp_of "$name")" ] && [ "$(cat "v3/bin/$name.stamp" 2>/dev/null)" = "$(stamp_of "$name")" ]; then
      echo "current: v3/bin/$name"; exit 0
    fi
    build_one "$name" "$src" "v3/bin/$name" v3 $(extra_roots "$src");;
  --all)
    JOBS=${V3_BUILD_JOBS:-8}
    out=$(mktemp -d); trap 'rm -rf "$out"' EXIT
    s=$(date +%s)
    inventory | awk '{print NR, $1, $2}' | xargs -P "$JOBS" -L 1 bash -c 'v3/build.sh --one "$1" "$2" > "'"$out"'/$0.txt" 2>&1; echo $? > "'"$out"'/$0.rc"'
    n=0; built=0; kept=0; failed=0
    while read -r i name src; do
      n=$((n+1)); cat "$out/$i.txt"
      if [ "$(cat "$out/$i.rc" 2>/dev/null)" != 0 ]; then failed=$((failed+1)); elif grep -q '^current:' "$out/$i.txt"; then kept=$((kept+1)); else built=$((built+1)); fi
    done < <(inventory | awk '{print NR, $1, $2}')
    echo "v3/build.sh: $n roots, $built built, $kept current, $failed failed, in $(secs $s) s"
    exit $failed;;
  *)
    SRC=${1:-v3/kernel/t0.shard}; [ $# -gt 0 ] && shift
    OUT=""; ROOT=v3; EXTRA=()
    if [ $# -gt 0 ] && [ "${1#--}" = "$1" ]; then OUT=$1; shift; fi
    while [ $# -gt 0 ]; do
      case $1 in --root) ROOT=$2; shift 2;; *) EXTRA+=("$1"); shift;; esac
    done
    [ -n "$OUT" ] || OUT=v3/bin/$(basename "$SRC" .shard)
    [ "$ROOT" = v3 ] && [ ${#EXTRA[@]} -eq 0 ] && EXTRA=($(extra_roots "$SRC"))
    build_one "$(basename "$OUT")" "$SRC" "$OUT" "$ROOT" "${EXTRA[@]}";;
esac
