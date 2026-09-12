#!/usr/bin/env bash
# v3/kernel/gen_pins.sh CHUNK… — regenerate accel_pins.shard's generated section
# from the pinned export's chunks through the last candidate's declaration:
# chunks 0–35 at the 2026-09-07 export (Nat, String, Char, List and most Nat
# operations in chunks 0–1; Char.ofNat and String.ofList in 3; Nat.lor and
# Nat.shiftLeft in 9; Nat.xor at line 702,093, chunk 35). Runs on the
# interpreter — the authority produces the table: t0.shard --pins replays the
# chunks, then refgen.shard prints every candidate's identity closure as the
# L declarations K admitted (FOUNDATION §3.2's fixed identities; add.shard
# `pin_if_matches` is the gate that compares against them). The hand-written
# header above the marker is kept; everything below it is replaced.
set -eu
cd "$(dirname "$0")/../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
out=$("$EVAL" direct v3/kernel/t0.shard --pins "$@")
echo "$out" | grep -v '^REF' | tail -1 >&2
if echo "$out" | grep -q '^REF-ERROR'; then echo "$out" | grep '^REF-ERROR' >&2; exit 1; fi
{
  names=$(for f in "$@"; do basename "$f"; done | tr '\n' ' ')
  sed -n '1,/^;; ---- GENERATED BELOW/p' v3/kernel/accel_pins.shard | sed "s|^;;; STATUS: .*|;;; STATUS: generated $(date +%F) from ${names}|"
  echo "$out" | sed -n 's/^REF //p'
} > v3/kernel/accel_pins.shard.new
mv v3/kernel/accel_pins.shard.new v3/kernel/accel_pins.shard
echo "accel_pins.shard: $(grep -c '^(fn rd_' v3/kernel/accel_pins.shard) reference rows, $(grep -c '^(fn rn_' v3/kernel/accel_pins.shard) names"
