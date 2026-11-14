#!/usr/bin/env bash
# calc's differential: the program half of calc (v3/examples/calc/), run under `ev` by
# kernel/test/calc_harness.shard on one input set (fixtures/calc_inputs.txt), one line per
# input and case, compared byte for byte with the recorded reference,
# fixtures/calc_old_tree.txt: the output of the previous generation's evaluator on the same
# inputs (the tag v2-kernel-last; 21 inputs, 727 lines). A disagreement is a finding, never
# a rendering to patch around; a new input needs its reference lines from the tag.
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "calc_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INPUTS=v3/kernel/test/fixtures/calc_inputs.txt
OLD=v3/kernel/test/fixtures/calc_old_tree.txt
v3out=$(mktemp); trap 'rm -f "$v3out"' EXIT

s=$(date +%s)
"$EVAL" direct v3/kernel/test/calc_harness.shard "$EXPORT" "$INDEX" "$INPUTS" > "$v3out" 2>&1; rc=$?
e=$(date +%s)
[ "$rc" -eq 0 ] || { echo "calc_test: the harness failed (exit $rc)"; head -20 "$v3out"; exit 1; }
echo "calc_test: calc under ev in $((e-s)) s, $(wc -l < "$v3out") lines"

i=$(grep -c '' "$INPUTS")
[ -s "$OLD" ] || { echo "calc_test: no reference ($OLD)"; exit 1; }
echo "calc_test: the recorded reference answers, $(wc -l < "$OLD") lines"
cmp "$v3out" "$OLD" || { echo "calc_test: outputs differ"; diff "$v3out" "$OLD" | head -20; exit 1; }
echo "calc_test: byte-identical over $i inputs"
