#!/usr/bin/env bash
# v3/t0_full.sh [LOG] — T0 over the WHOLE export on compiled K (v3/build.sh),
# gated twice: (1) the interpreter byte-tie — `eval direct` and the binary
# print byte-identical verbose per-declaration output on the committed fixture
# and on the export's first 100,000 lines (the authority confirming the
# engine, FOUNDATION §9.1); (2) the verdict line of the full replay equals the
# pinned line in v3/t0_expected.txt, and (3) every admitted constant's axiom
# closure (the driver's -a lines) is identical to the oracle's (init.axioms,
# v3/axioms.lean; v3/t0_axioms_cmp.sh), and (4) every accelerator candidate
# present is pinned (the driver's -p line; all 20 on the full export). The replay is one process over the
# 20,000-line chunks; its peak resident set is reported (the export tables hold
# every referenced node: 30 GB at 2026-09-07's export, 745 s) and capped by
# V3_T0_RSS_CAP_KB (default 400 GB). V3_T0_SKIP_FULL=1 runs the byte-ties only
# (the local smoke; the dev box does not replay the whole export routinely).
set -euo pipefail
cd "$(dirname "$0")/.."
DIR=${V3_EXPORT_DIR:-.shard-cache/v3-export}
K=${K:-v3/bin/t0}
RUST_EVAL=${RUST_EVAL:-./rust_bootstrap/target/release/eval}
LOG=${1:-v3_t0_full.log}
CAP_KB=${V3_T0_RSS_CAP_KB:-$((400 * 1024 * 1024))}
[ -x "$K" ] || { echo "missing $K (v3/build.sh)"; exit 1; }
[ -x "$RUST_EVAL" ] || { echo "missing $RUST_EVAL (cargo build --release in rust_bootstrap/)"; exit 1; }
[ -s "$DIR/init.ndjson" ] || { echo "missing $DIR/init.ndjson (v3/export.sh)"; exit 1; }
[ -s "$DIR/init.axioms" ] || { echo "missing $DIR/init.axioms (v3/export.sh)"; exit 1; }
DIR=$(cd "$DIR" && pwd)
K=$(cd "$(dirname "$K")" && pwd)/$(basename "$K")
CH=$DIR/chunks
if [ ! -d "$CH" ] || [ -z "$(ls "$CH" 2>/dev/null)" ]; then
  mkdir -p "$CH"
  (cd "$CH" && split -l 20000 -d -a 4 "$DIR/init.ndjson" probe_)
fi
FIX=v3/kernel/test/fixtures/init_prefix_3000.ndjson

# the driver's -p line: every candidate present in the environment pinned (its
# identity closure matched accel_pins.shard's reference rows — FOUNDATION §3.2,
# GPT-6 R42); with a count, that many pinned
pins_ok() {
  local line; line=$(grep '^PINS ' "$1" | tail -1)
  [ -n "$line" ] || { echo "NO PINS LINE in $1"; return 1; }
  echo "   $line"
  case "$line" in *"| unpinned:") ;; *) echo "UNPINNED ACCELERATOR CANDIDATES: $line"; return 1;; esac
  if [ -n "${2:-}" ]; then
    local n; n=$(echo "$line" | sed 's/ | unpinned:.*//; s/^PINS pinned://' | wc -w)
    [ "$n" -eq "$2" ] || { echo "EXPECTED $2 PINNED CANDIDATES, GOT $n: $line"; return 1; }
  fi
}

echo "== byte-tie: interpreter vs compiled K, verbose + closures, on the fixture"
"$RUST_EVAL" direct v3/kernel/t0.shard -v -a -p "$FIX" > "$LOG.tie_interp" 2>&1 || true
"$K" -v -a -p "$FIX" > "$LOG.tie_native" 2>&1 || true
cmp "$LOG.tie_interp" "$LOG.tie_native" || { echo "BYTE-TIE FAILED (fixture)"; diff "$LOG.tie_interp" "$LOG.tie_native" | head -20; exit 1; }
echo "   fixture: identical ($(wc -l < "$LOG.tie_native") lines)"
echo "== byte-tie: chunks 0-4 (100,000 lines)"
PRE=$(ls "$CH"/probe_000[0-4])
"$RUST_EVAL" direct v3/kernel/t0.shard -v -a -p $PRE > "$LOG.tie_interp" 2>&1 || true
"$K" -v -a -p $PRE > "$LOG.tie_native" 2>&1 || true
cmp "$LOG.tie_interp" "$LOG.tie_native" || { echo "BYTE-TIE FAILED (chunks 0-4)"; diff "$LOG.tie_interp" "$LOG.tie_native" | head -20; exit 1; }
echo "   chunks 0-4: identical ($(wc -l < "$LOG.tie_native") lines)"
tail -1 "$LOG.tie_native"
pins_ok "$LOG.tie_native" || exit 1
v3/t0_axioms_cmp.sh "$LOG.tie_native" "$DIR/init.axioms" || { echo "AXIOM CLOSURES DIFFER FROM THE ORACLE (chunks 0-4)"; exit 1; }
if [ "${V3_T0_SKIP_FULL:-0}" = 1 ]; then
  echo "V3_T0_SKIP_FULL=1: byte-ties only, the full replay skipped"
  exit 0
fi

echo "== full export: $(ls "$CH" | wc -l) chunks, compiled K"
start=$(date +%s)
"$K" -a -p "$CH"/probe_* > "$LOG" 2>&1 &
ENG=$!
PEAK=0
while kill -0 "$ENG" 2>/dev/null; do
  RSS=$(awk '/VmRSS/ {print $2}' /proc/$ENG/status 2>/dev/null)
  [ "${RSS:-0}" -gt "$PEAK" ] && PEAK=$RSS
  if [ "${RSS:-0}" -gt "$CAP_KB" ]; then
    echo "WATCHDOG: RSS ${RSS} kB over the ${CAP_KB} kB cap -- killing the replay"
    kill -9 "$ENG"
    break
  fi
  sleep 5
done
wait "$ENG" && STATUS=0 || STATUS=$?
echo "   replay exit $STATUS, $(( $(date +%s) - start )) s, peak RSS ${PEAK} kB"
[ "$STATUS" -eq 0 ] || { tail -20 "$LOG"; exit 1; }
tail -1 "$LOG"
grep -qxF "$(cat v3/t0_expected.txt)" "$LOG" || { echo "VERDICT LINE DIFFERS FROM v3/t0_expected.txt: $(cat v3/t0_expected.txt)"; exit 1; }
v3/t0_axioms_cmp.sh "$LOG" "$DIR/init.axioms" || { echo "AXIOM CLOSURES DIFFER FROM THE ORACLE"; exit 1; }
pins_ok "$LOG" 20 || exit 1
echo "T0 FULL == PINNED, closures identical, all 20 accelerator candidates pinned"
