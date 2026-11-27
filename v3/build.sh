#!/usr/bin/env bash
# v3/build.sh — a native binary from a shard program: the root's closure as the V3 loader
# accepted it, lowered and written as C by V3's own back end inside the loader's process
# (`load.shard --emit`: kernel/lower.shard, kernel/cgen.shard, over kernel/rt.h), then `cc` on
# each file of that text side by side, into v3/bin/ by default. The back end is unproven: a binary
# stands in for the evaluator written in shard as the Rust bootstrap does (LAW.md rule 21), and
# the byte-ties (v3/t0_full.sh; kernel/test/native_tie_test.sh) hold the two engines together.
#
#   v3/build.sh [SRC [OUT]] [--native] [--root DIR] [EXTRA…]   one root: SRC's closure (default
#                        v3/kernel/t0.shard) into OUT (default v3/bin/STEM); --root the loader's
#                        root (default v3); EXTRA… (after OUT) further roots for the load: the
#                        directories of the views the program links (none named: K's, when the
#                        closure consumes its view); --native: the emit by the loader's binary
#                        (THE EMIT'S ENGINE, below)
#   v3/build.sh --all [--bootstrap]   the inventory below: the loader first, on the bootstrap,
#                        then the other roots by its binary, V3_BUILD_JOBS at a time (default
#                        8); under --bootstrap every root on the bootstrap, side by side; a
#                        binary whose stamp is current is kept; exit = the roots that failed
#   v3/build.sh --tie    after --all: the C text of every root of the inventory produced again
#                        by the engine that did not build it and compared with the build's, byte
#                        for byte, V3_BUILD_JOBS roots at a time; exit = the roots not the same
#   v3/build.sh --stamp NAME   the stamp v3/bin/NAME would carry now (empty: never emitted)
#   v3/build.sh --list   the inventory: NAME SRC
#   v3/build.sh --shim   the engine shim v3/bin/eval (--all writes it last): the bootstrap's
#                        command line over the binaries — `direct FILE` runs v3/bin/STEM when its
#                        stamp is current and fails loudly otherwise; `dump` the bootstrap's
#
# THE INVENTORY: the four drivers, the search's driver (v3/search/find.shard), calc_harness,
# every *_test.shard of kernel/test/ and of k/test/ — parity's list (kernel/test/parity_test.sh).
# A root outside K's directory whose closure consumes K's view is loaded with v3/kernel/k as a
# second root, so the directory's own check links the implementation into the program (as
# parity and k_clients_test do; the bootstrap's dump shows env_empty when K is in the closure).
# What a shell entrypoint's invocation of an engine becomes on the binaries:
#   eval direct v3/kernel/X.shard ARGS…    -> v3/bin/X ARGS…  (a driver or a test: this inventory)
#   eval direct load.shard … --run M.FN F… -> v3/bin/load …   (the loader's own operation)
#   eval direct load.shard … --dump F      -> v3/bin/load …   (the loader's own operation)
#   eval dump F                            -> the bootstrap's (its reader's text for parity; no binary)
# The shim v3/bin/eval is these dispositions as a program: v3/test.sh runs every entrypoint on it
# under V3_ENGINE=native, and kernel/test/suite_tie.sh compares that run with the bootstrap's.
#
# THE EMIT'S ENGINE. An emit is a run of the loader, and the loader runs on the bootstrap
# (`eval direct v3/kernel/load.shard …`) or as its own binary (`v3/bin/load …`). `--all` builds
# the loader's binary on the bootstrap and every other root by that binary: on the dev box 29
# binaries in 238 s, at 8 roots at a time as at 24 — the loader's emit 128 s and its compile
# 8 s, then the other 28, whose compiles fill the machine. One root is emitted on the
# bootstrap unless --native names the binary. The binary must then be current: a missing or
# stale v3/bin/load fails the build, and nothing falls back to the bootstrap. The loader's own
# root is built on the bootstrap alone, so the chain has one link and no binary builds its
# successor. v3/bin/emit/NAME/engine names the engine that emitted NAME.
# A root the binary emitted rests on that binary being a right compile of the loader. `--tie`
# is the check: each root's text is produced again by the other engine — the bootstrap for a
# root the binary emitted, the binary for a root the bootstrap emitted, the loader's own among
# them — and compared with the build's, every file of it. It ends with a control: one byte of
# one text altered must be reported. CI builds every root on the bootstrap (`--all
# --bootstrap`) and ties each to the binary's emit, seconds a root, on every pipeline.
#
# THE COMPILE. The text is one translation unit a file (kernel/cgen.shard: main.c and the
# parts, each over prog.h): `cc -O2 -c` runs on V3_CC_JOBS of them side by side (default: the
# machine's processors), then one `cc` links the objects. Under --all the count is the whole
# build's, not a root's: the roots in flight take their compiles' places from one fifo of
# V3_CC_JOBS tokens (V3_CC_SEM). The objects are scratch, removed after the link.
#
# THE READING. A root of the inventory is loaded --unchecked (kernel/load.shard): each `fn` is
# read as a program alone, nothing checks its types and it gets no definition. The check of the
# toolchain's sources is a load without the flag: kernel/test/toolchain_check.sh loads every
# root so, on the loader's binary. Any other SRC — an ordinary program, a tool — is loaded as
# every load is, checked. The program a build compiles is then the reading of programs alone:
# kernel/test/parity_test.sh ties it to the text of the bootstrap's reader (`eval dump`) on
# every root, under both engines, and toolchain_check.sh ties the checked load's program to
# the same text.
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
# Generated files live under v3/bin/ (emit/NAME/ with main.c, prog.h, the parts, map and sources;
# NAME, NAME.stamp, NAME.build.log; NAME is OUT's last component); only a binary and its stamp
# go elsewhere, when OUT names another place. An authored file is never written. v3/bin/ is
# derived and untracked.
set -uo pipefail
cd "$(dirname "$0")/.."
RUST_EVAL=${RUST_EVAL:-./rust_bootstrap/target/release/eval}
CFLAGS="-O2"
[ -x "$RUST_EVAL" ] || { echo "missing $RUST_EVAL (cargo build --release in rust_bootstrap/)"; exit 1; }

inventory() {   # NAME SRC per root, parity's list
  for f in v3/kernel/load.shard v3/kernel/prove.shard v3/kernel/t0.shard v3/kernel/initindex.shard \
           v3/search/find.shard \
           v3/kernel/test/calc_harness.shard v3/kernel/test/*_test.shard v3/kernel/k/test/*_test.shard; do
    echo "$(basename "$f" .shard) $f"
  done
}
# the files NAME's stamp is made of, in the stamp's order: the root's closure sources (the emit's
# `sources` list), the implementations behind its views, the toolchain's sources, the table of
# Init's facts the bootstrap reads those by, the runtime
stamp_files() {
  local root
  root=$(cat "v3/bin/emit/$1/root" 2>/dev/null || echo v3)
  awk -v r="$root" '{print r "/" $2}' "v3/bin/emit/$1/sources" | sort
  if [ -s "v3/bin/emit/$1/extra" ]; then
    while IFS= read -r d; do find "$d" -name '*.shard' | LC_ALL=C sort; done < "v3/bin/emit/$1/extra"
  fi
  printf '%s\n' v3/kernel/*.shard v3/kernel/k/*.shard v3/kernel/init.facts v3/kernel/rt.h
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
snapshot() { { find "$@" v3/kernel -name '*.shard' -type f 2>/dev/null; echo v3/kernel/init.facts; echo v3/kernel/rt.h; } | sums; }
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
# is SRC a root of the inventory (THE READING)
toolchain_root() { inventory | awk '{print $2}' | grep -qxF "$1"; }
# the loader's binary present and a build of the sources as they stand
loader_current() {
  local st; st=$(stamp_of load)
  [ -x v3/bin/load ] && [ -n "$st" ] && [ "$(cat v3/bin/load.stamp 2>/dev/null)" = "$st" ]
}
# emit_text ENGINE DIR ROOT SRC EXTRA…: the load, the lowering and the C text of SRC's closure
# into DIR by the loader on ENGINE; the loader's lines on stdout, its exit code returned
emit_text() {
  local engine=$1 dir=$2 root=$3 src=$4; shift 4
  # Init on demand: the pinned export and its index. Every root of the toolchain needs them —
  # its sources stand on Init's types — and so does an ordinary root; ensure_index (below) has
  # made the index current before the first emit
  local init="" flag=""
  read -r EXPORT INDEX < <(v3/kernel/test/init_index.sh --current 2>/dev/null) && [ -n "${INDEX:-}" ] && init="--init $EXPORT --init-index $INDEX"
  toolchain_root "$src" && flag=--unchecked
  case $engine in
    native) v3/bin/load --root "$root" $init $flag --emit "$dir" "$src" "$@";;
    *)      "$RUST_EVAL" direct v3/kernel/load.shard --root "$root" $init $flag --emit "$dir" "$src" "$@";;
  esac
}
# same_text A B: the two emits hold the same files with the same bytes (what the build itself
# writes beside the loader's files — root, extra, engine — is not the loader's text); prints
# the first files that differ
same_text() {
  local a=$1 b=$2 la lb f bad=0
  la=$(cd "$a" && find . -type f ! -name root ! -name extra ! -name engine | LC_ALL=C sort)
  lb=$(cd "$b" && find . -type f ! -name root ! -name extra ! -name engine | LC_ALL=C sort)
  [ -n "$la" ] && [ "$la" = "$lb" ] || { echo "the two emits do not hold the same files"; return 1; }
  while IFS= read -r f; do
    cmp -s "$a/$f" "$b/$f" || { bad=$((bad+1)); [ "$bad" -le 3 ] && echo "${f#./} differs"; }
  done <<< "$la"
  [ "$bad" = 0 ]
}
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
#   dump …              the bootstrap, unchanged (its reader's text, for parity)
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
# cc_unit EMIT OBJ UNIT: one unit of the text to its object, the compiler's output beside it;
# under --all a token of V3_CC_SEM is held while the compiler runs (THE COMPILE)
cc_unit() {
  local rc tok
  if [ -n "${V3_CC_SEM:-}" ]; then exec 9<>"$V3_CC_SEM"; read -r -N 1 -u 9 tok || return 1; fi
  cc $CFLAGS -c -o "$2/${3%.c}.o" "$1/$3" -I v3/kernel > "$2/$3.out" 2>&1; rc=$?
  [ -z "${V3_CC_SEM:-}" ] || printf . >&9
  return $rc
}
# compile EMIT OUT LOG: every unit of the text in EMIT to an object, V3_CC_JOBS of them side by
# side, and the objects linked into OUT; the compiler's output is added to LOG in the units' order
compile() {
  local emit=$1 out=$2 log=$3 obj rc=0 u
  obj=$(mktemp -d)
  (cd "$emit" && ls -- *.c) | xargs -P "${V3_CC_JOBS:-$(nproc)}" -I{} bash -c 'cc_unit "$1" "$2" "$3"' _ "$emit" "$obj" {} || rc=1
  for u in "$obj"/*.out; do cat "$u"; done >> "$log" 2>/dev/null
  if [ "$rc" = 0 ]; then cc $CFLAGS -o "$out" "$obj"/*.o >> "$log" 2>&1 || rc=1; fi
  rm -rf "$obj"
  return $rc
}
# build_one ENGINE NAME SRC OUT ROOT EXTRA…: emit (the load, the lowering, the C text) by the
# loader on ENGINE (bootstrap, native), the compile; the stamp last
build_one() {
  local engine=$1 name=$2 src=$3 out=$4 root=$5; shift 5
  local emit=v3/bin/emit/$name log=v3/bin/$name.build.log s t_emit t_cc entry rc snap err changed
  if [ "$engine" = native ]; then
    [ "$src" != v3/kernel/load.shard ] || { echo "FAIL: $name: the loader's own root is built on the bootstrap, never by a binary"; return 1; }
    loader_current || { echo "FAIL: $name: v3/bin/load is missing or stale, and nothing falls back to the bootstrap (v3/build.sh v3/kernel/load.shard, or --all)"; return 1; }
  fi
  mkdir -p v3/bin; rm -rf "$emit"; mkdir -p "$emit"; : > "$log"
  snap=$(snapshot "$root" "$@")
  s=$(date +%s)
  emit_text "$engine" "$emit" "$root" "$src" "$@" > "$log" 2>&1; rc=$?
  if [ "$rc" != 0 ]; then
    echo "FAIL: $name: the emit (exit $rc); $(grep -m1 '^EMIT: \|^LOAD: ' "$log" | cut -c1-200)"; grep -E '^(REFUSE|ERROR|EMIT)' "$log" | head -3 | cut -c1-300; return 1
  fi
  t_emit=$(secs $s); echo "$root" > "$emit/root"; echo "$engine" > "$emit/engine"
  for x in "$@"; do case $x in v3/kernel/k|v3/kernel/k/) ;; *) [ -d "$x" ] && echo "$x";; esac; done > "$emit/extra"
  entry=$(sed -n 's/^EMIT: .* entry=\([^ ]*\).*/\1/p' "$log" | tail -1)
  [ -n "$entry" ] && [ -s "$emit/$entry" ] || { echo "FAIL: $name: no entry file after the emit"; tail -2 "$log" | cut -c1-200; return 1; }
  s=$(date +%s)
  if ! compile "$emit" "$out" "$log"; then
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
# The Init index current before any emit. Where the index's own driver has no current binary
# — a fresh tree, or after a change of kernel/index.shard or kernel/initindex.shard — the
# bootstrap builds it, about half an hour over the export: the driver is a toolchain program,
# and the loader needs the index to load it. Without an export nothing is built here, and a
# root fails at its load, which names the export.
ensure_index() {
  v3/kernel/test/init_index.sh --current > /dev/null 2>&1 && return 0
  [ -s "${V3_EXPORT_DIR:-.shard-cache/v3-export}/init.ndjson" ] || return 0
  echo "v3/build.sh: the Init index is not current: building it (kernel/test/init_index.sh)" >&2
  v3/kernel/test/init_index.sh > /dev/null || { echo "v3/build.sh: the Init index was not built" >&2; return 1; }
}
# tie_one NAME SRC: NAME's C text produced again by the engine that did not build it, into a
# scratch directory, and compared with the build's
tie_one() {
  local name=$1 src=$2 built=v3/bin/emit/$1 st engine other d rc s extra
  st=$(stamp_of "$name")
  [ -x "v3/bin/$name" ] && [ -n "$st" ] && [ "$(cat "v3/bin/$name.stamp" 2>/dev/null)" = "$st" ] || { echo "FAIL: $name: v3/bin/$name is missing or stale (v3/build.sh --all)"; return 1; }
  loader_current || { echo "FAIL: $name: v3/bin/load is missing or stale (v3/build.sh --all)"; return 1; }
  engine=$(cat "$built/engine" 2>/dev/null)
  case $engine in
    bootstrap) other=native;;
    native) other=bootstrap;;
    *) echo "FAIL: $name: $built/engine does not name the engine that built it (v3/build.sh --all)"; return 1;;
  esac
  d=$(mktemp -d); s=$(date +%s); extra=$(extra_roots "$src")
  emit_text "$other" "$d" v3 "$src" $extra > "$d.log" 2>&1; rc=$?
  if [ "$rc" != 0 ]; then
    echo "FAIL: $name: the emit on the $other engine (exit $rc); $(grep -m1 '^EMIT: \|^LOAD: ' "$d.log" | cut -c1-200)"; rm -rf "$d" "$d.log"; return 1
  fi
  if ! same_text "$built" "$d" > "$d.diff"; then
    echo "DIFFER: $name: built by the $engine engine, emitted again by the $other: $(tr '\n' ' ' < "$d.diff")"
    rm -rf "$d" "$d.log" "$d.diff"; return 1
  fi
  echo "same: $name ($(find "$d" -type f | wc -l) files; built by the $engine engine, emitted again by the $other in $(secs $s) s)"
  rm -rf "$d" "$d.log" "$d.diff"; return 0
}
export -f stamp_files stamp_of sums snapshot changed_since extra_roots secs build_one inventory
export -f toolchain_root loader_current emit_text same_text tie_one cc_unit compile
export RUST_EVAL CFLAGS

case ${1:-} in
  --list) inventory; exit 0;;
  --shim) write_shim; exit 0;;
  --stamp) stamp_of "$2"; exit 0;;
  --one)   # NAME SRC (the --all worker): current → kept, else built — the loader on the
           # bootstrap, any other root by the loader's binary (on the bootstrap too under
           # --all --bootstrap, which says so through V3_ALL_ENGINE)
    name=$2; src=$3
    if [ -x "v3/bin/$name" ] && [ -n "$(stamp_of "$name")" ] && [ "$(cat "v3/bin/$name.stamp" 2>/dev/null)" = "$(stamp_of "$name")" ]; then
      echo "current: v3/bin/$name"; exit 0
    fi
    engine=${V3_ALL_ENGINE:-native}; [ "$src" = v3/kernel/load.shard ] && engine=bootstrap
    build_one "$engine" "$name" "$src" "v3/bin/$name" v3 $(extra_roots "$src");;
  --tie-one) tie_one "$2" "$3";;   # NAME SRC (the --tie worker)
  --tie)
    ensure_index || exit 1
    JOBS=${V3_BUILD_JOBS:-8}
    out=$(mktemp -d); trap 'rm -rf "$out"' EXIT
    s=$(date +%s)
    inventory | awk '{print NR, $1, $2}' | xargs -P "$JOBS" -L 1 bash -c 'v3/build.sh --tie-one "$1" "$2" > "'"$out"'/$0.txt" 2>&1; echo $? > "'"$out"'/$0.rc"'
    n=0; same=0; bad=0
    while read -r i name src; do
      n=$((n+1)); cat "$out/$i.txt"
      if [ "$(cat "$out/$i.rc" 2>/dev/null)" = 0 ]; then same=$((same+1)); else bad=$((bad+1)); fi
    done < <(inventory | awk '{print NR, $1, $2}')
    # the control: a comparison that cannot fail says nothing, so one byte of the loader's text
    # is altered in a copy and the comparison must report its file
    cp -r v3/bin/emit/load "$out/altered" 2>/dev/null && echo >> "$out/altered/main.c"
    if [ "$(same_text v3/bin/emit/load "$out/altered" 2>/dev/null)" = "main.c differs" ]; then echo "control: an altered byte is reported"
    else echo "FAIL: the control: an altered byte of v3/bin/emit/load/main.c was not reported"; bad=$((bad+1)); fi
    echo "v3/build.sh --tie: $n roots, $same the same under the other engine, $bad not, in $(secs $s) s"
    exit $bad;;
  --all)
    case ${2:-} in ""|--bootstrap) ;; *) echo "v3/build.sh: --all [--bootstrap]" >&2; exit 2;; esac
    ensure_index || exit 1
    JOBS=${V3_BUILD_JOBS:-8}
    out=$(mktemp -d); trap 'rm -rf "$out"' EXIT
    # one count of compiles over all the roots in flight (THE COMPILE)
    export V3_CC_SEM=$out/cc.sem
    mkfifo "$V3_CC_SEM"; exec 9<>"$V3_CC_SEM"; head -c "${V3_CC_JOBS:-$(nproc)}" /dev/zero | tr '\0' . >&9
    s=$(date +%s)
    if [ "${2:-}" = --bootstrap ]; then
      # every root on the bootstrap, side by side
      export V3_ALL_ENGINE=bootstrap
      inventory | awk '{print NR, $1, $2}' | xargs -P "$JOBS" -L 1 bash -c 'v3/build.sh --one "$1" "$2" > "'"$out"'/$0.txt" 2>&1; echo $? > "'"$out"'/$0.rc"'
    else
      # the loader first, alone: every other root is emitted by its binary
      inventory | awk '$1 == "load" {print NR, $1, $2}' | xargs -L 1 bash -c 'v3/build.sh --one "$1" "$2" > "'"$out"'/$0.txt" 2>&1; echo $? > "'"$out"'/$0.rc"'
      inventory | awk '$1 != "load" {print NR, $1, $2}' | xargs -P "$JOBS" -L 1 bash -c 'v3/build.sh --one "$1" "$2" > "'"$out"'/$0.txt" 2>&1; echo $? > "'"$out"'/$0.rc"'
    fi
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
    OUT=""; ROOT=v3; EXTRA=(); ENGINE=bootstrap
    if [ $# -gt 0 ] && [ "${1#--}" = "$1" ]; then OUT=$1; shift; fi
    while [ $# -gt 0 ]; do
      case $1 in --root) ROOT=$2; shift 2;; --native) ENGINE=native; shift;; *) EXTRA+=("$1"); shift;; esac
    done
    [ -n "$OUT" ] || OUT=v3/bin/$(basename "$SRC" .shard)
    [ "$ROOT" = v3 ] && [ ${#EXTRA[@]} -eq 0 ] && EXTRA=($(extra_roots "$SRC"))
    ensure_index || exit 1
    build_one "$ENGINE" "$(basename "$OUT")" "$SRC" "$OUT" "$ROOT" "${EXTRA[@]}";;
esac
