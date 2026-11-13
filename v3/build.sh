#!/usr/bin/env bash
# v3/build.sh — route 1 (FOUNDATION §9.1): a toolchain closure as the V3 loader accepted
# it, lowered and written as C by V3's own back end inside the loader's process (`load.shard
# --emit`: kernel/lower.shard, kernel/cgen.shard, over kernel/rt.h), then `cc` into a native
# binary under v3/bin/ (LANGUAGE.md §8.7 slice 5.0 rules 2 and 4; §8.6 slice 4.1 rules 1, 2
# and 8). The binaries are ENGINES: route 1's proof is still to come, so the Rust interpreter
# (route 3) stays the authority and the byte-ties (v3/t0_full.sh;
# kernel/test/native_tie_test.sh) tie the two. Nothing of the old tree is read: until slice
# 5.0 the closure was printed in the old dialect and compiled by the old tree's two tools.
#
#   v3/build.sh [SRC [OUT]] [--root DIR] [EXTRA…]   one root: SRC's closure (default
#                        v3/kernel/t0.shard) into OUT (default v3/bin/STEM); --root the loader's
#                        root (default v3); EXTRA… further root files for the load (K's directory)
#   v3/build.sh --all    the inventory below, V3_BUILD_JOBS roots at a time (default 8); a binary
#                        whose stamp is current is kept; exit = the roots that failed
#   v3/build.sh --stamp NAME   the stamp v3/bin/NAME would carry now (empty: never emitted)
#   v3/build.sh --list   the inventory: NAME SRC
#   v3/build.sh --shim   the engine shim v3/bin/eval (rule 4; --all writes it last): the bootstrap's
#                        command line over the binaries — `direct FILE` runs v3/bin/STEM when its
#                        stamp is current and fails loudly otherwise; `dump` the bootstrap's
#
# THE INVENTORY (rule 8): the five drivers, calc_harness, the public harnesses of
# kernel/test/ and those of k/test/ — parity's list (kernel/test/parity_test.sh).
# A root outside K's directory whose closure consumes K's view is loaded with v3/kernel/k as a
# second root, so the directory's own check links the implementation into the program (as
# parity and k_clients_test do; the bootstrap's dump shows env_empty when K is in the closure).
# Every shell entrypoint's invocation of an engine has its disposition here:
#   eval direct v3/kernel/X.shard ARGS…        -> v3/bin/X ARGS…      (the drivers and the harnesses: this inventory)
#   eval direct load.shard … --run M.FN F…     -> v3/bin/load …       (the loader's own operation: k_clients_test, wire_test, define_test, calc_main_test, route2_test)
#   eval direct load.shard … --dump F          -> v3/bin/load …       (the loader's own operation: template_test, define_test, parity's V3 side)
#   eval dump F                                -> the bootstrap's     (parity's second reader; never compiled)
# The shim v3/bin/eval is these dispositions as a program: v3/test.sh runs every entrypoint on it
# under V3_ENGINE=native (rule 4), and kernel/test/suite_tie.sh compares that run with the bootstrap's.
#
# THE EMIT'S ENGINE: the bootstrap for every root. The loader's own closure — 9,003
# declarations — loads, lowers and generates in 34 s on it (measured at slice 5.0), so no binary
# builds another and a build never trusts one.
#
# THE STAMP (rule 8): sha256 over the root's closure sources (the emit's `sources` list),
# the toolchain's sources — v3/kernel/*.shard, the back end among them, k/*.shard, k's view —,
# the runtime (v3/kernel/rt.h) and the compiler's flags — v3/bin/NAME.stamp beside the binary;
# the tie and the shim refuse a stale one. The `sources` list names a view by its mod.req.shard
# alone, never the implementation behind it, so a root built with a view's directory as an
# extra root has that directory's .shard files in its stamp too (emit/NAME/extra lists the
# directories; K's own is in every stamp already). Found at slice 5.1 landing 3: an edit of
# the formatter's printer, which sits behind a view, left its binary's stamp unchanged. Generated files live under v3/bin/ only (emit/NAME/
# with main.c, its parts, map and sources; NAME, NAME.stamp, NAME.build.log); an authored file
# is never written. v3/bin/ is derived and untracked.
set -uo pipefail
cd "$(dirname "$0")/.."
RUST_EVAL=${RUST_EVAL:-./rust_bootstrap/target/release/eval}
CFLAGS="-O2"
[ -x "$RUST_EVAL" ] || { echo "missing $RUST_EVAL (cargo build --release in rust_bootstrap/)"; exit 1; }

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
    if [ -s "v3/bin/emit/$1/extra" ]; then   # the implementations behind the root's views
      while IFS= read -r d; do find "$d" -name '*.shard' | LC_ALL=C sort | xargs cat 2>/dev/null; done < "v3/bin/emit/$1/extra"
    fi
    cat v3/kernel/*.shard v3/kernel/k/*.shard v3/kernel/k/mod.req.shard
    cat v3/kernel/rt.h
    echo "cc $CFLAGS"; } | sha256sum | cut -c1-16
}
# K's directory as a second root when the closure consumes its view from outside it
extra_roots() {
  case $1 in v3/kernel/k/*) return;; esac
  # grep reads to the end: under pipefail a `-q` that closes the pipe early would fail the dump
  if "$RUST_EVAL" dump "$1" 2>/dev/null | grep '^fn env_empty ' > /dev/null; then echo v3/kernel/k; fi
}
secs() { echo $(( $(date +%s) - $1 )); }
# the engine shim (rule 4): a generated file under v3/bin/, rewritten by every --all
write_shim() {
  mkdir -p v3/bin
  cat > v3/bin/eval <<'SHIM'
#!/usr/bin/env bash
# v3/bin/eval — the engine shim (v3/LANGUAGE.md §8.6 slice 4.1 rule 4), GENERATED by
# v3/build.sh --all (or --shim); never edited. The bootstrap's command line over the binaries
# route 1 built:
#   direct FILE ARGS…   v3/bin/NAME ARGS… (NAME = FILE's stem) when that binary exists and its
#                       stamp is current (v3/build.sh --stamp NAME); otherwise exit 70 with a
#                       message — never a silent fallback to the bootstrap (R84)
#   dump …              the bootstrap, unchanged (parity's second reader)
# The command line's paths stay the caller's: every entrypoint runs from the repo root.
set -u
root=$(cd "$(dirname "$0")/../.." && pwd)
case ${1:-} in
  direct)
    [ $# -ge 2 ] || { echo "v3/bin/eval: direct FILE ARGS…" >&2; exit 2; }
    name=$(basename "$2" .shard); bin=$root/v3/bin/$name; shift 2
    [ -x "$bin" ] || { echo "v3/bin/eval: no native binary for $name (v3/build.sh --all)" >&2; exit 70; }
    now=$("$root/v3/build.sh" --stamp "$name")
    [ -n "$now" ] && [ "$now" = "$(cat "$bin.stamp" 2>/dev/null)" ] || { echo "v3/bin/eval: v3/bin/$name is stale (v3/build.sh --all)" >&2; exit 70; }
    exec "$bin" "$@";;
  dump) exec "${RUST_EVAL:-$root/rust_bootstrap/target/release/eval}" "$@";;
  *) echo "v3/bin/eval: direct FILE ARGS… | dump … (the engine shim; v3/build.sh --all)" >&2; exit 2;;
esac
SHIM
  chmod +x v3/bin/eval
}
# build_one NAME SRC OUT ROOT EXTRA…: emit (the load, the lowering, the C text), cc; the stamp last
build_one() {
  local name=$1 src=$2 out=$3 root=$4; shift 4
  local emit=v3/bin/emit/$name log=v3/bin/$name.build.log s t_emit t_cc entry rc
  mkdir -p v3/bin; rm -rf "$emit"; mkdir -p "$emit"; : > "$log"
  # Init on demand for a root under the naming law (slice 4.3 landing 2): the pinned export and
  # its index when the fixtures hold them and the index is current — never built here (CI builds
  # the driver before the index: on pipeline 629 the lookup built it on the bootstrap, ~1,950 s,
  # in each heavy job); the kernel's roots import no Init and are unchanged
  local init=""; read -r EXPORT INDEX < <(v3/kernel/test/init_index.sh --current 2>/dev/null) && [ -n "${INDEX:-}" ] && init="--init $EXPORT --init-index $INDEX"
  s=$(date +%s)
  "$RUST_EVAL" direct v3/kernel/load.shard --root "$root" $init --emit "$emit" "$src" "$@" > "$log" 2>&1; rc=$?
  if [ "$rc" != 0 ]; then
    echo "FAIL: $name: the emit (exit $rc); $(grep -m1 '^EMIT: \|^LOAD: ' "$log" | cut -c1-200)"; grep -E '^(REFUSE|ERROR|EMIT)' "$log" | head -3 | cut -c1-300; return 1
  fi
  t_emit=$(secs $s); echo "$root" > "$emit/root"
  for x in "$@"; do case $x in v3/kernel/k|v3/kernel/k/) ;; *) [ -d "$x" ] && echo "$x";; esac; done > "$emit/extra"
  entry=$(sed -n 's/^EMIT: .* entry=\([^ ]*\).*/\1/p' "$log" | tail -1)
  [ -n "$entry" ] && [ -s "$emit/$entry" ] || { echo "FAIL: $name: no entry file after the emit"; tail -2 "$log" | cut -c1-200; return 1; }
  s=$(date +%s)
  cc $CFLAGS -o "$out" "$emit/$entry" -I v3/kernel >> "$log" 2>&1 || { echo "FAIL: $name: cc"; tail -3 "$log" | cut -c1-300; return 1; }
  t_cc=$(secs $s)
  stamp_of "$name" > "$out.stamp"
  echo "OK: $out ($(sed -n 's/^EMIT: [^ ]* entry=[^ ]* //p' "$log" | tail -1); emit $t_emit s, cc $t_cc s)"
}
export -f stamp_of extra_roots secs build_one inventory
export RUST_EVAL CFLAGS

case ${1:-} in
  --list) inventory; exit 0;;
  --shim) write_shim; exit 0;;
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
    write_shim
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
