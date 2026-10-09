#!/usr/bin/env bash
# calc's loop closed against the host (v3/LANGUAGE.md §8.4 slice 3.18 rule 6;
# §12.6's "calc's app step"): v3/examples/calc/calc_main.shard, an S program
# over v3/std/host.shard and v3/std/bytes.shard, run by the driver with one
# line of the differential's inputs per argument. Its output must be the
# model world's — `run_world`, the same loop against a world datum, which
# calc_test.sh ties to the old tree — byte for byte: the harness prints the
# model as `* run_world: (CalcWorld (Cons LINE …))`, each LINE a list of
# codes, decoded here to the lines it stands for.
# The libraries import Init through UInt8.toNat (line 263,515 of the export):
# the prefix is two chunks, the pins' and its continuation.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "calc_main_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index (slice 3.26)
RCPT=${INIT_RECEIPT:+--init-receipt $INIT_RECEIPT}   # the Init receipt (slice 3.23), set by v3/test.sh when one exists
INPUTS=v3/kernel/test/fixtures/calc_inputs.txt
got=$(mktemp); want=$(mktemp); model=$(mktemp); trap 'rm -f "$got" "$want" "$model"' EXIT

lines=()
while IFS= read -r line || [ -n "$line" ]; do lines+=("$line"); done < "$INPUTS"

s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT $RCPT --run examples.calc.calc_main.main v3/examples/calc/calc_main.shard -- "${lines[@]}" > "$got" 2>&1; rc=$?
e=$(date +%s)
[ "$rc" -eq 0 ] || { echo "calc_main_test: the program failed (exit $rc)"; head -5 "$got"; exit 1; }
echo "calc_main_test: the program on the host in $((e-s)) s, $(wc -l < "$got") lines"

"$EVAL" direct v3/kernel/test/calc_harness.shard "$EXPORT" "$INDEX" "$INPUTS" > "$model" 2>&1 || { echo "calc_main_test: the harness failed"; head -5 "$model"; exit 1; }
# the model world's lines: after CalcWorld, an outer Cons opens a line, an inner Cons is
# followed by a code, an inner Nil ends the line, the outer Nil ends the list
grep '^\* run_world: ' "$model" | tr '()' '  ' | awk '
  { st = 0
    for (i = 4; i <= NF; i++) {
      if (st == 0) { if ($i == "Cons") st = 1; else if ($i == "Nil") break }
      else if ($i == "Cons") { i++; printf "%c", $i + 0 }
      else if ($i == "Nil") { printf "\n"; st = 0 } } }' > "$want"
[ -s "$want" ] || { echo "calc_main_test: the model world is empty"; exit 1; }
[ "$(wc -l < "$want")" -eq "${#lines[@]}" ] || { echo "calc_main_test: the model has $(wc -l < "$want") lines for ${#lines[@]} inputs"; exit 1; }
cmp "$got" "$want" || { echo "calc_main_test: the host's output is not the model world's"; diff "$got" "$want" | head -10; exit 1; }
echo "calc_main_test: byte-identical to the model world over ${#lines[@]} inputs"
