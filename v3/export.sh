#!/usr/bin/env bash
# v3/export.sh — the T0 oracle: Lean v4.33.1's `Init` (kernel pin
# 819816b2e0a3bf405af45ae5c7af2491d8f5bee6, v3/README.md) exported by
# lean4export at 15f6055 with its toolchain forced to v4.33.1 (the head pins a
# later toolchain that cannot export the pinned kernel — records §8), into
# $V3_EXPORT_DIR (default .shard-cache/v3-export: untracked; on CI built
# fresh every run, 77 s — caching the 3.5 GB tree cost more than that).
# Idempotent: an export carrying the pin marker is reused.
# elan is installed into $ELAN_HOME (default $V3_EXPORT_DIR/elan) when absent,
# without touching shell profiles.
set -euo pipefail
cd "$(dirname "$0")/.."
LEAN_TOOLCHAIN=leanprover/lean4:v4.33.1
EXPORT_COMMIT=15f6055
DIR=${V3_EXPORT_DIR:-.shard-cache/v3-export}
mkdir -p "$DIR"
DIR=$(cd "$DIR" && pwd)
PIN="$LEAN_TOOLCHAIN $EXPORT_COMMIT"
if [ -s "$DIR/init.ndjson" ] && [ "$(cat "$DIR/init.ndjson.pin" 2>/dev/null)" = "$PIN" ]; then
  echo "export present: $DIR/init.ndjson ($(wc -l < "$DIR/init.ndjson") lines; $PIN)"
  exit 0
fi
export ELAN_HOME=${ELAN_HOME:-$DIR/elan}
export PATH="$ELAN_HOME/bin:$PATH"
if [ ! -x "$ELAN_HOME/bin/elan" ]; then
  echo "== $(date '+%H:%M:%S') installing elan into $ELAN_HOME"
  curl https://elan.lean-lang.org/elan-init.sh -sSf | sh -s -- -y --no-modify-path --default-toolchain "$LEAN_TOOLCHAIN"
fi
elan toolchain install "$LEAN_TOOLCHAIN"
if [ ! -d "$DIR/lean4export/.git" ]; then
  echo "== $(date '+%H:%M:%S') cloning lean4export"
  git clone -q https://github.com/leanprover/lean4export.git "$DIR/lean4export"
fi
cd "$DIR/lean4export"
git checkout -q "$EXPORT_COMMIT" 2>/dev/null || { git fetch -q; git checkout -q "$EXPORT_COMMIT"; }
echo "$LEAN_TOOLCHAIN" > lean-toolchain
echo "== $(date '+%H:%M:%S') lake build"
lake build
echo "== $(date '+%H:%M:%S') exporting Init"
lake env .lake/build/bin/lean4export Init > "$DIR/init.ndjson.tmp" 2> "$DIR/init.export.err"
mv "$DIR/init.ndjson.tmp" "$DIR/init.ndjson"
echo "$PIN" > "$DIR/init.ndjson.pin"
rm -rf "$DIR/chunks"
echo "== $(date '+%H:%M:%S') export built: $(wc -l < "$DIR/init.ndjson") lines"
