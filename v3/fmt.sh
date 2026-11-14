#!/usr/bin/env bash
# v3/fmt.sh — the formatter over sources of the tree (v3/tools/shardfmt; the layout law is
# docs/CANON.md). The gate itself is the formatter's test
# (tools/shardfmt/test/shardfmt_test.sh, an entrypoint of v3/test.sh): it refuses a tracked
# source that is not canonical. This script is the command that answers it.
#
#   v3/fmt.sh [FILE…]         check: the FILEs — the gated sources when none is named — that
#                             are not their canonical text, one per line; exit 0 when there
#                             is none and none is refused, 1 otherwise
#   v3/fmt.sh --fix [FILE…]   the same files rewritten in place with their canonical text;
#                             exit 1 when one is refused
#   v3/fmt.sh --list          the gated sources: every tracked .shard file under v3/ that an
#                             author writes
#
# NOT GATED: a file a tool writes is that tool's text — its regeneration is what keeps it
# stable, and the tools' tests compare a regenerated file with the committed one byte for
# byte. These are the proof sidecars (*.auto.shard: kernel/prove.shard writes them) and
# kernel/k/accel_pins.shard (kernel/gen_pins.sh writes its generated section).
#
# The binary never writes a file (its contract is one read and one write to standard output:
# tools/shardfmt/shardfmt_bin.shard), so the rewrite is this script's: `shardfmt --check`
# prints the text into a temporary file beside the source, which replaces the source only on
# exit 1 — the binary printed a text that differs from the source and that the reader reads
# as it reads the source. On exit 0 the source already is that text. A source the binary
# refuses (exit 2: it does not read; exit 3: the gate's refusal) is reported and left as it
# is; the two pins broken on purpose are such sources and are not listed when the whole tree
# is the argument.
#
# v3/bin/shardfmt is built here when it is missing or its stamp is stale (v3/build.sh; the
# emit is ~30 s on the bootstrap).
set -u
cd "$(dirname "$0")/.."
T=v3/tools/shardfmt
BIN=v3/bin/shardfmt
gated() { git ls-files 'v3/*.shard' | grep -v -e '\.auto\.shard$' -e '^v3/kernel/k/accel_pins\.shard$' | LC_ALL=C sort; }
[ "${1:-}" = "--list" ] && { gated; exit 0; }
fix=0; [ "${1:-}" = "--fix" ] && { fix=1; shift; }
now=$(v3/build.sh --stamp shardfmt)
if [ ! -x "$BIN" ] || [ -z "$now" ] || [ "$now" != "$(cat "$BIN.stamp" 2>/dev/null)" ]; then
  v3/build.sh $T/shardfmt.shard "$BIN" $T/print $T/reads > /dev/null || { echo "v3/fmt.sh: the formatter did not build (v3/bin/shardfmt.build.log)" >&2; exit 2; }
fi
whole=0
if [ $# -eq 0 ]; then whole=1; mapfile -t files < <(gated); else files=("$@"); fi
drift=0; refused=0
for f in "${files[@]}"; do
  tmp=$(mktemp "$f.fmt.XXXXXX") || exit 2
  "$BIN" --check "$f" > "$tmp" 2>/dev/null; rc=$?
  case $rc in
    0) ;;
    1) drift=$((drift+1))
       if [ $fix = 1 ]; then chmod --reference="$f" "$tmp" && mv "$tmp" "$f"; tmp=""; echo "rewritten: $f"; else echo "not canonical: $f"; fi ;;
    *) case "$whole:$f" in
         1:v3/pins/loader/read_error/main.shard|1:v3/pins/reader/refuse_unterminated.shard) ;;
         *) refused=$((refused+1)); echo "refused (exit $rc): $f: $(head -1 "$tmp" | cut -c1-160)" ;;
       esac ;;
  esac
  [ -z "$tmp" ] || rm -f "$tmp"
done
if [ $fix = 1 ]; then echo "v3/fmt.sh: ${#files[@]} files, $drift rewritten, $refused refused"
else echo "v3/fmt.sh: ${#files[@]} files, $drift not canonical, $refused refused$([ $drift -gt 0 ] && echo " — v3/fmt.sh --fix")"; fi
if [ $fix = 1 ]; then [ $refused -eq 0 ]; else [ $drift -eq 0 ] && [ $refused -eq 0 ]; fi
