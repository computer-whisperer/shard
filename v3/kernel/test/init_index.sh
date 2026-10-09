#!/usr/bin/env bash
# v3/kernel/test/init_index.sh — the Init index current, its paths printed
# (LANGUAGE.md §8.4 slice 3.26 rule 1): v3/.cache/init.index and init.names, the
# record table and the name table over the export (V3_EXPORT_DIR/init.ndjson,
# v3/export.sh's), rebuilt by initindex.shard when the export's pin or size or the
# index's sources changed since they were written (the stamp beside them), never
# committed (v3/.cache/ is ignored). The index is a property of the export's text,
# not of K: a kernel edit does not stale it; another export does, and the loader
# refuses an index whose header names another byte count.
#   read INDEX NAMES < <(v3/kernel/test/init_index.sh)
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
DIR=${V3_EXPORT_DIR:-.shard-cache/v3-export}
EXPORT=$DIR/init.ndjson
[ -s "$EXPORT" ] || { echo "init_index.sh: no export at $EXPORT (v3/export.sh)" >&2; exit 1; }
IDX=v3/.cache/init.index; NAMES=v3/.cache/init.names; STAMP=v3/.cache/init.index.stamp
x=$( { cat "$DIR/init.ndjson.pin" 2>/dev/null; stat -c %s "$EXPORT"; cat v3/kernel/index.shard v3/kernel/initindex.shard; } | sha256sum | cut -c1-16)
if [ -s "$IDX" ] && [ -s "$NAMES" ] && [ "$(cat "$STAMP" 2>/dev/null)" = "$x" ]; then echo "$IDX $NAMES"; exit 0; fi
mkdir -p v3/.cache
tmp=$(mktemp "v3/.cache/init.index.XXXXXX")
# the driver compiled by route 1 (v3/build.sh v3/kernel/initindex.shard v3/bin/initindex; 68 s over
# the export against 952 s on the bootstrap, the tables byte-identical) when it is newer than the
# index's sources; the bootstrap otherwise
BIN=v3/bin/initindex
if [ -x "$BIN" ] && [ "$BIN" -nt v3/kernel/index.shard ] && [ "$BIN" -nt v3/kernel/initindex.shard ]; then
  RUN=("$BIN")
else
  RUN=("$EVAL" direct v3/kernel/initindex.shard)
fi
if "${RUN[@]}" "$EXPORT" "$tmp" "$tmp.names" > "$tmp.log" 2>&1 && [ -s "$tmp" ] && [ -s "$tmp.names" ]; then
  mv "$tmp" "$IDX" && mv "$tmp.names" "$NAMES" && echo "$x" > "$STAMP"; rm -f "$tmp.log"
  echo "$IDX $NAMES"
else
  echo "init_index.sh: the index was not built" >&2; tail -3 "$tmp.log" >&2; rm -f "$tmp" "$tmp.names" "$tmp.log"; exit 1
fi
