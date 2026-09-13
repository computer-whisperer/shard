#!/usr/bin/env bash
# Frontend parity (v3/LANGUAGE.md §10 item 1, slice 6): over every toolchain
# entrypoint — the driver, the T0 driver, the calc harness, every test — the
# V3 reader's Prog (`load.shard --dump`, kernel/dump.shard) and the Rust
# bootstrap's Module over the same closure (`eval dump`, rust_bootstrap/src/
# dump.rs) printed as one canonical text, compared byte for byte. Green, this
# retires TCB bring-up item 2: the Rust loader still parses the toolchain for
# route 3, and the V3 reader's agreement is what keeps it honest.
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
a=$(mktemp); b=$(mktemp); trap 'rm -f "$a" "$b"' EXIT
n=0; decls=0; failed=0
s=$(date +%s)
for entry in v3/kernel/load.shard v3/kernel/t0.shard v3/kernel/test/calc_harness.shard v3/kernel/test/*_test.shard; do
  n=$((n+1))
  "$EVAL" dump "$entry" > "$a" 2>&1 || { echo "parity_test: eval dump failed on $entry"; head -3 "$a"; failed=$((failed+1)); continue; }
  "$EVAL" direct v3/kernel/load.shard --root v3 --dump "$entry" > "$b" 2>&1 || { echo "parity_test: load --dump failed on $entry"; head -3 "$b"; failed=$((failed+1)); continue; }
  if cmp -s "$a" "$b"; then
    decls=$((decls + $(wc -l < "$a")))
  else
    echo "parity_test: $entry differs"; diff "$a" "$b" | head -6; failed=$((failed+1))
  fi
done
e=$(date +%s)
echo "parity_test: $n closures, $decls declarations, $failed differ, in $((e-s)) s"
[ "$failed" -eq 0 ] && echo "parity_test: byte-identical"
exit $failed
