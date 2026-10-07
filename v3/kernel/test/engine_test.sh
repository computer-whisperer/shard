#!/usr/bin/env bash
# v3/kernel/test/engine_test.sh — the producers (v3/LANGUAGE.md §8.4 slice 3.22, landing 2;
# law §7.5). Two parts. (1) v3/examples/auto: every theorem delegates with `auto`; the build
# replays the committed sidecar (auto.auto.shard) — 11 theorems accepted by name, `beyond`
# pending auto_missing, nothing searched —, and prove.shard regenerates the sidecar into a
# scratch copy of the example under v3/ (the loader reads repository-relative paths), which
# must be byte-identical to the committed one but for the fingerprints (the copy is another
# module, and a statement's fingerprint carries the module's name): the engine's output is reproducible and the
# sidecar is machine-owned. (2) The engine's reach over calc's 100 claims (the slice's third
# ruling): prove.shard --measure over the five leaves of tactic_test.sh runs the engine on
# every hand-written theorem's statement unaided; the names it closes must be exactly
# engine_calc.txt's (a change either way is a report, not a silent drift) and the count is
# printed. Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
FIX=v3/kernel/test/fixtures/init_prefix_int.ndjson
TAIL=v3/kernel/test/fixtures/init_prefix_str_tail.ndjson
OUT=$(mktemp -d)
SCRATCH=$(mktemp -d -p v3 engine_scratch_XXXXXX)
trap 'rm -rf "$OUT" "$SCRATCH"' EXIT
fail=0; checks=0
bad() { echo "engine_test: $*"; fail=$((fail+1)); }
check() { checks=$((checks+1)); "$@" || bad "check $checks: $*"; }

# ---- (1) the example: replay, then regeneration
"$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --init "$TAIL" v3/examples/auto/auto.shard > "$OUT/load.txt" 2>&1
checks=$((checks+1))
grep -q 'accepted 23 .*pending 1 .*errors 0$' "$OUT/load.txt" || { bad "the example does not load as expected"; grep -E 'ERROR|REFUSE|PENDING|^LOAD:' "$OUT/load.txt" | head -5 | cut -c1-300; }
for t in two_three len_two lt_dec le_step le_chain nat_cast implication app_nil len_app len_rev_onto app_assoc; do
  checks=$((checks+1)); grep -q "^ACCEPT examples.auto.auto.$t " "$OUT/load.txt" || bad "$t not accepted from the sidecar"
done
checks=$((checks+1)); grep -q "^PENDING examples.auto.auto.beyond auto_missing fp=" "$OUT/load.txt" || bad "beyond is not pending auto_missing"
checks=$((checks+1)); grep -q "^PROVED" "$OUT/load.txt" && bad "the build searched (a PROVED record under load.shard)"
# the regeneration: the example copied without its sidecar, prove run on the copy
cp v3/examples/auto/auto.shard "$SCRATCH/auto.shard"
"$EVAL" direct v3/kernel/prove.shard --root v3 --init "$FIX" --init "$TAIL" "$SCRATCH/auto.shard" > "$OUT/prove.txt" 2>&1
checks=$((checks+1)); grep -q 'proved 11  unsolved 1  sidecars written 1$' "$OUT/prove.txt" || { bad "prove did not solve 11 of 12"; grep -E 'PROVE:|PENDING|ERROR' "$OUT/prove.txt" | head -5 | cut -c1-300; }
# the fingerprints masked: a statement's fingerprint carries the module's name (the copy's is
# another module), and the committed one's fingerprints are checked by the replay above
mask() { sed -E 's/^\(proof-for ([^ ]+) [0-9]+ /(proof-for \1 FP /' "$1"; }
checks=$((checks+1)); cmp -s <(mask "$SCRATCH/auto.auto.shard") <(mask v3/examples/auto/auto.auto.shard) || { bad "the regenerated sidecar differs from the committed one"; diff <(mask "$SCRATCH/auto.auto.shard") <(mask v3/examples/auto/auto.auto.shard) | head -6 | cut -c1-300; }
checks=$((checks+1)); cmp -s "$SCRATCH/auto.shard" v3/examples/auto/auto.shard || bad "prove wrote into the source"
echo "engine_test: the example replays 11 theorems from its sidecar and prove regenerates it byte-identical"

# ---- (2) the count over calc
measure() { "$EVAL" direct v3/kernel/prove.shard --root v3 --init "$FIX" --init "$TAIL" --measure "v3/examples/calc/$1.shard" > "$OUT/m_$1.txt" 2>&1; }
measure calc_app_world & measure calc_show_run & measure calc_ndigit & measure calc_spec_tests & measure calc_reconcile_tests & wait
for f in calc_app_world calc_show_run calc_ndigit calc_spec_tests calc_reconcile_tests; do
  checks=$((checks+1)); grep -q 'pending 0 .*errors 0$' "$OUT/m_$f.txt" || bad "$f does not load clean under --measure"
done
: > "$OUT/solved.txt"; total=0
while read -r leaf mod name; do
  case $leaf in \#*|'') continue;; esac
  total=$((total+1))
  grep -q "^ENGINE examples.calc.$mod.$name solved$" "$OUT/m_$leaf.txt" && echo "$mod.$name" >> "$OUT/solved.txt"
done < v3/kernel/test/calc_claims.txt
solved=$(wc -l < "$OUT/solved.txt")
checks=$((checks+1)); [ "$total" = 100 ] || bad "the claim list has $total names, not 100"
checks=$((checks+1)); grep -v '^#' v3/kernel/test/engine_calc.txt | diff - "$OUT/solved.txt" > "$OUT/diff.txt" || { bad "the engine's reach over calc changed (engine_calc.txt against this run)"; cat "$OUT/diff.txt"; }
echo "engine_test: the engine closes $solved of calc's $total claims unaided"
echo "engine_test: $checks checks, $fail failed"
exit $fail
