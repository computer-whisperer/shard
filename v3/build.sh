#!/usr/bin/env bash
# v3/build.sh — a native binary from a shard program: the root's closure as the V3 loader
# accepted it, lowered and written as C by V3's own back end inside the loader's process
# (`load.shard --emit`: kernel/lower.shard, kernel/cgen.shard, over kernel/rt.h), then `cc`,
# into v3/bin/ by default. The back end is unproven, so the Rust bootstrap stays the authority
# and the byte-ties (v3/t0_full.sh; kernel/test/native_tie_test.sh) tie the two.
#
#   v3/build.sh [SRC [OUT]] [--root DIR] [EXTRA…]   one root: SRC's closure (default
#                        v3/kernel/t0.shard) into OUT (default v3/bin/STEM); --root the loader's
#                        root (default v3); EXTRA… (after OUT) further roots for the load: the
#                        directories of the views the program links (none named: K's, when the
#                        closure consumes its view)
#   v3/build.sh --all    the inventory below, V3_BUILD_JOBS roots at a time (default 8); a binary
#                        whose stamp is current is kept; exit = the roots that failed
#   v3/build.sh --stamp NAME   the stamp v3/bin/NAME would carry now (empty: never emitted)
#   v3/build.sh --list   the inventory: NAME SRC
#   v3/build.sh --shim   the engine shim v3/bin/eval (--all writes it last): the bootstrap's
#                        command line over the binaries — `direct FILE` runs v3/bin/STEM when its
#                        stamp is current and fails loudly otherwise; `dump` the bootstrap's
#
# THE INVENTORY: the five drivers, calc_harness, every *_test.shard of kernel/test/ and of
# k/test/ — parity's list (kernel/test/parity_test.sh).
# A root outside K's directory whose closure consumes K's view is loaded with v3/kernel/k as a
# second root, so the directory's own check links the implementation into the program (as
# parity and k_clients_test do; the bootstrap's dump shows env_empty when K is in the closure).
# What a shell entrypoint's invocation of an engine becomes on the binaries:
#   eval direct v3/kernel/X.shard ARGS…    -> v3/bin/X ARGS…  (a driver or a test: this inventory)
#   eval direct load.shard … --run M.FN F… -> v3/bin/load …   (the loader's own operation)
#   eval direct load.shard … --dump F      -> v3/bin/load …   (the loader's own operation)
#   eval dump F                            -> the bootstrap's (parity's second reader; no binary)
# The shim v3/bin/eval is these dispositions as a program: v3/test.sh runs every entrypoint on it
# under V3_ENGINE=native, and kernel/test/suite_tie.sh compares that run with the bootstrap's.
#
# THE EMIT'S ENGINE: the bootstrap for every root. The loader's own closure — 9,003
# declarations — loads, lowers and generates in 34 s on it, so no binary builds another and a
# build never trusts one.
#
# THE STAMP: sha256 over the root's closure sources (the emit's `sources` list),
# the toolchain's sources — v3/kernel/*.shard, the back end among them, k/*.shard, k's view —,
# the runtime (v3/kernel/rt.h) and the compiler's flags — v3/bin/NAME.stamp beside the binary;
# the tie and the shim refuse a stale one. A stamp covers every file that can enter the binary:
# the `sources` list names a view by its mod.req.shard alone, never the implementation behind
# it, so a root built with a view's directory as an extra root has that directory's .shard
# files in its stamp too (emit/NAME/extra lists the directories; K's own is in every stamp
# already), and an edit behind a view makes the binary stale.
# A stamp says which sources the binary is a build of, so a build hashes every file that can
# enter its stamp before the emit reads any and compares the stamp's files with those hashes
# after the compile: one that was edited, created or removed while the build ran fails the
# build (`FAIL: NAME: changed during the build: FILE…`) and no stamp is left beside the binary.
# A file outside the stamp, or a file touched with its content unchanged, fails nothing.
# Generated files live under v3/bin/ (emit/NAME/ with main.c, its parts, map and sources;
# NAME, NAME.stamp, NAME.build.log; NAME is OUT's last component); only a binary and its stamp
# go elsewhere, when OUT names another place. An authored file is never written. v3/bin/ is
# derived and untracked.
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
# the files NAME's stamp is made of, in the stamp's order: the root's closure sources (the emit's
# `sources` list), the implementations behind its views, the toolchain's sources, the runtime
stamp_files() {
  local root
  root=$(cat "v3/bin/emit/$1/root" 2>/dev/null || echo v3)
  awk -v r="$root" '{print r "/" $2}' "v3/bin/emit/$1/sources" | sort
  if [ -s "v3/bin/emit/$1/extra" ]; then
    while IFS= read -r d; do find "$d" -name '*.shard' | LC_ALL=C sort; done < "v3/bin/emit/$1/extra"
  fi
  printf '%s\n' v3/kernel/*.shard v3/kernel/k/*.shard v3/kernel/rt.h
}
# the stamp of NAME from its emitted sources list; empty when never emitted
stamp_of() {
  [ -s "v3/bin/emit/$1/sources" ] || { echo ""; return; }
  { stamp_files "$1" | xargs cat 2>/dev/null; echo "cc $CFLAGS"; } | sha256sum | cut -c1-16
}
# sums: the paths on the input, each once and spelled one way, with the hash of its content now
sums() { sed 's://*:/:g' | LC_ALL=C sort -u | xargs -r sha256sum 2>/dev/null; }
# snapshot ROOT EXTRA…: every file that can enter a stamp — the .shard files under the loader's
# root and the extra roots, the toolchain's sources, the runtime — hashed before the emit reads any
snapshot() { { find "$@" v3/kernel -name '*.shard' -type f 2>/dev/null; echo v3/kernel/rt.h; } | sums; }
# changed_since NAME SNAPSHOT: the files of NAME's stamp whose content is not the snapshot's — a
# source edited, created or removed while the build ran. A file outside the stamp is not looked at
changed_since() {
  local then now
  now=$(stamp_files "$1" | sums)
  then=$(awk 'NR == FNR { f[$2]; next } ($2 in f)' <(stamp_files "$1" | sed 's://*:/:g; s:^:x  :') <(echo "$2"))
  [ "$now" = "$then" ] || diff <(echo "$then") <(echo "$now") | sed -n 's/^[<>] [0-9a-f]*  //p' | sort -u
}
# K's directory as a second root when the closure consumes its view from outside it
extra_roots() {
  case $1 in v3/kernel/k/*) return;; esac
  # grep reads to the end: under pipefail a `-q` that closes the pipe early would fail the dump
  if "$RUST_EVAL" dump "$1" 2>/dev/null | grep '^fn env_empty ' > /dev/null; then echo v3/kernel/k; fi
}
secs() { echo $(( $(date +%s) - $1 )); }
# the engine shim: a generated file under v3/bin/, rewritten by every --all
write_shim() {
  mkdir -p v3/bin
  cat > v3/bin/eval <<'SHIM'
#!/usr/bin/env bash
# v3/bin/eval — the engine shim, GENERATED by v3/build.sh --all (or --shim); never edited.
# The bootstrap's command line over the native binaries:
#   direct FILE ARGS…   v3/bin/NAME ARGS… (NAME = FILE's stem) when that binary exists and its
#                       stamp is current (v3/build.sh --stamp NAME); otherwise exit 70 with a
#                       message — never a silent fallback to the bootstrap
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
  local emit=v3/bin/emit/$name log=v3/bin/$name.build.log s t_emit t_cc entry rc snap err changed
  mkdir -p v3/bin; rm -rf "$emit"; mkdir -p "$emit"; : > "$log"
  snap=$(snapshot "$root" "$@")
  # Init on demand for a root under the naming law: the pinned export and its index when the
  # index is current — never built here (a build by this lookup would run on the bootstrap,
  # ~1,950 s: CI builds the index's driver before the index); the kernel's roots import no Init
  # and need neither
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
  if ! cc $CFLAGS -o "$out" "$emit/$entry" -I v3/kernel >> "$log" 2>&1; then
    # the compiler's first error, wherever in its output it stands; the whole output is the log
    err=$(grep -m1 'error:' "$log" | cut -c1-300); [ -n "$err" ] || err=$(tail -1 "$log" | cut -c1-300)
    echo "FAIL: $name: cc: $err"; echo "   the compiler's output: $log"; return 1
  fi
  t_cc=$(secs $s)
  # a stamp says which sources the binary is a build of: one edited after the emit read it would be
  # stamped as built, so the sources are compared with what they were when the build began
  changed=$(changed_since "$name" "$snap")
  if [ -n "$changed" ]; then
    rm -f "$out.stamp"
    echo "FAIL: $name: changed during the build: $(echo "$changed" | head -3 | tr '\n' ' ')— no stamp is written; build again"; return 1
  fi
  stamp_of "$name" > "$out.stamp"
  echo "OK: $out ($(sed -n 's/^EMIT: [^ ]* entry=[^ ]* //p' "$log" | tail -1); emit $t_emit s, cc $t_cc s)"
}
export -f stamp_files stamp_of sums snapshot changed_since extra_roots secs build_one inventory
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
