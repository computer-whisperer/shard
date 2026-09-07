#!/usr/bin/env bash
# v3/kernel/gen_pins.sh CHUNK… — regenerate accel_pins.shard from the pinned
# export's chunks through the last candidate's declaration: chunks 0–35 at
# the 2026-09-07 export (Nat, String, Char, List and most Nat operations in
# chunks 0–1; Char.ofNat and String.ofList in 3; Nat.lor and Nat.shiftLeft in
# 9; Nat.xor at line 702,093, chunk 35). Runs on the interpreter — the
# authority produces the table (4.5 min). The table is the fixed-identity pin
# of FOUNDATION §3.2.
set -eu
cd "$(dirname "$0")/../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
out=$("$EVAL" direct v3/kernel/t0.shard --pins "$@")
echo "$out" | grep -v '^PIN' | tail -1 >&2
{
  names=$(for f in "$@"; do basename "$f"; done | tr '\n' ' ')
  sed -n '1,/^(fn accel_pin/p' v3/kernel/accel_pins.shard | sed '$d' | sed "s|^;;; STATUS: .*|;;; STATUS: generated $(date +%F) from ${names}|"
  echo "(fn accel_pin ((n Name)) (Option Int)"
  echo "$out" | grep '^PIN' | while read -r _ name hash; do
    # name → (Str (Str Anon 'a) 'b) form
    expr="Anon"; IFS=. read -ra parts <<< "$name"; for p in "${parts[@]}"; do expr="(Str $expr (quote $p))"; done
    echo "  (if (name_eq n $expr) (Some $hash)"
  done
  n=$(echo "$out" | grep -c '^PIN')
  printf '  None'; for _ in $(seq 1 "$n"); do printf ')'; done; echo ')'
} > v3/kernel/accel_pins.shard.new
mv v3/kernel/accel_pins.shard.new v3/kernel/accel_pins.shard
echo "accel_pins.shard: $(grep -c 'name_eq n' v3/kernel/accel_pins.shard) rows"
