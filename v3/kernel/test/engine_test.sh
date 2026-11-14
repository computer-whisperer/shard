#!/usr/bin/env bash
# v3/kernel/test/engine_test.sh — the producers.
# Four parts. (1) v3/examples/auto: every theorem delegates with `auto`; the build
# replays the committed sidecar (auto.auto.shard) — 11 theorems accepted by name, `beyond`
# pending auto_missing, nothing searched —, and prove.shard regenerates the sidecar into a
# scratch copy of the example under v3/ (the loader reads repository-relative paths), which
# must be byte-identical to the committed one but for the fingerprints (the copy is another
# module, and a statement's fingerprint carries the module's name): the engine's output is reproducible and the
# sidecar is machine-owned. (1b) An obligation delegated with `auto` — `(fulfills f.dec_N auto)`
# in a file with no `auto` theorem — is proved by prove.shard, entered in the sidecar under the
# obligation's name and discharged from it by a load; a second prove searches nothing.
# (1c) `auto` inside the implementation of a view — a requirement's `fulfills`, a theorem, an
# obligation: prove.shard given the directory writes one sidecar, beside the implementation
# file; a load of a consumer with the directory replays it; a second prove, given the consumer
# and the directory, searches nothing.
# (2) The engine's reach over calc's 100 claims:
# prove.shard --measure over the five leaves of tactic_test.sh runs the engine on
# every hand-written theorem's statement unaided; the names it closes must be exactly
# engine_calc.txt's (a change either way is a report, not a silent drift) and the count is
# printed. Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "engine_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index
OUT=$(mktemp -d)
SCRATCH=$(mktemp -d -p v3 engine_scratch_XXXXXX)
trap 'rm -rf "$OUT" "$SCRATCH"' EXIT
fail=0; checks=0
bad() { echo "engine_test: $*"; fail=$((fail+1)); }

# ---- (1) the example: replay, then regeneration
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/examples/auto/auto.shard > "$OUT/load.txt" 2>&1
checks=$((checks+1))
grep -q 'accepted 23 .*pending 1 .*errors 0$' "$OUT/load.txt" || { bad "the example does not load as expected"; grep -E 'ERROR|REFUSE|PENDING|^LOAD:' "$OUT/load.txt" | head -5 | cut -c1-300; }
for t in two_three len_two lt_dec le_step le_chain nat_cast implication app_nil len_app len_rev_onto app_assoc; do
  checks=$((checks+1)); grep -q "^ACCEPT examples.auto.auto.$t " "$OUT/load.txt" || bad "$t not accepted from the sidecar"
done
checks=$((checks+1)); grep -q "^PENDING examples.auto.auto.beyond auto_missing fp=" "$OUT/load.txt" || bad "beyond is not pending auto_missing"
checks=$((checks+1)); grep -q "^PROVED" "$OUT/load.txt" && bad "the build searched (a PROVED record under load.shard)"
# the regeneration: the example copied without its sidecar, prove run on the copy
cp v3/examples/auto/auto.shard "$SCRATCH/auto.shard"
"$EVAL" direct v3/kernel/prove.shard --root v3 $INIT "$SCRATCH/auto.shard" > "$OUT/prove.txt" 2>&1
checks=$((checks+1)); grep -q 'proved 11  unsolved 1  sidecars written 1$' "$OUT/prove.txt" || { bad "prove did not solve 11 of 12"; grep -E 'PROVE:|PENDING|ERROR' "$OUT/prove.txt" | head -5 | cut -c1-300; }
# the keys masked: a statement's fingerprint and its dependencies' carry the module's name (the copy's is
# another module), and the committed one's fingerprints are checked by the replay above
mask() { sed -E 's/^\(proof-for ([^ ]+) \(req [0-9 ]+\) /(proof-for \1 KEY /' "$1"; }
checks=$((checks+1)); cmp -s <(mask "$SCRATCH/auto.auto.shard") <(mask v3/examples/auto/auto.auto.shard) || { bad "the regenerated sidecar differs from the committed one"; diff <(mask "$SCRATCH/auto.auto.shard") <(mask v3/examples/auto/auto.auto.shard) | head -6 | cut -c1-300; }
checks=$((checks+1)); cmp -s "$SCRATCH/auto.shard" v3/examples/auto/auto.shard || bad "prove wrote into the source"
echo "engine_test: the example replays 11 theorems from its sidecar and prove regenerates it byte-identical"

# ---- (1b) an obligation delegated with auto: a file whose only `auto` stands in a fulfills.
# prove writes the entry under the obligation's name as the source writes it; then a load
# discharges the obligation from it and a second prove finds nothing to search
printf '%s\n' '(import Init UInt8.toNat)' '(fn dn ((n Nat) (m Nat)) Nat (measure n) (if (< m n) (dn m m) 0))' '(fulfills dn.dec_1 auto)' > "$SCRATCH/fulfills.shard"
M=$(basename "$SCRATCH").fulfills
"$EVAL" direct v3/kernel/prove.shard --root v3 $INIT "$SCRATCH/fulfills.shard" > "$OUT/f_prove.txt" 2>&1
checks=$((checks+1)); grep -q 'proved 1  unsolved 0  sidecars written 1$' "$OUT/f_prove.txt" || { bad "prove did not discharge the delegated obligation"; grep -E 'PROVE:|PENDING|ERROR' "$OUT/f_prove.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^(proof-for dn\.dec_1 (req ' "$SCRATCH/fulfills.auto.shard" 2>/dev/null || { bad "the sidecar's entry does not bear the obligation's name dn.dec_1"; grep '^(proof-for' "$SCRATCH/fulfills.auto.shard" 2>/dev/null | cut -c1-120; }
# (1c)'s sources, and its first prove beside (1b)'s two runs
mkdir "$SCRATCH/lib"
printf '%s\n' '(import Init UInt8.toNat)' '(sig fn pick ((n Nat)) Nat)' '(requirement pick_le ((x Int) (y Int) (z Int) (h1 (<= x y)) (h2 (<= y z))) (<= x z))' > "$SCRATCH/lib/mod.req.shard"
printf '%s\n' '(import Init UInt8.toNat)' '(fn pick ((n Nat)) Nat n)' '(fulfills pick_le auto)' '(theorem extra ((x Int) (y Int) (h1 (<= 0 x)) (h2 (<= x y))) (<= 0 y) auto)' '(fn dn ((n Nat) (m Nat)) Nat (measure n) (if (< m n) (dn m m) 0))' '(fulfills dn.dec_1 auto)' > "$SCRATCH/lib/lib.shard"
L=$(basename "$SCRATCH").lib
printf '%s\n' '(import "lib")' "(fn twice_pick ((n Nat)) Nat ($L.pick ($L.pick n)))" > "$SCRATCH/client.shard"
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT "$SCRATCH/fulfills.shard" > "$OUT/f_load.txt" 2>&1 &
"$EVAL" direct v3/kernel/prove.shard --root v3 $INIT "$SCRATCH/fulfills.shard" > "$OUT/f_prove2.txt" 2>&1 &
"$EVAL" direct v3/kernel/prove.shard --root v3 $INIT "$SCRATCH/lib" > "$OUT/i_prove.txt" 2>&1 &
wait
checks=$((checks+1)); { grep -q "^DISCHARGE $M.dn.dec_1 proved$" "$OUT/f_load.txt" && ! grep -q '^PENDING' "$OUT/f_load.txt" && grep -q 'pending 0 .*errors 0$' "$OUT/f_load.txt"; } || { bad "a load does not discharge the obligation from the sidecar"; grep -E 'PENDING|ERROR|^LOAD:' "$OUT/f_load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q "^PROVED" "$OUT/f_load.txt" && bad "the build searched for the obligation (a PROVED record under load.shard)"
checks=$((checks+1)); grep -q 'proved 0  unsolved 0  sidecars written 0$' "$OUT/f_prove2.txt" || { bad "a second prove searched again"; grep -E 'PROVE:' "$OUT/f_prove2.txt" | cut -c1-300; }
echo "engine_test: an obligation delegated with auto is proved once, entered under its name and discharged from the sidecar"

# ---- (1c) auto inside the implementation of a view: the sidecar stands beside the
# implementation file and holds the three entries under the names the source writes
checks=$((checks+1)); grep -q 'proved 3  unsolved 0  sidecars written 1$' "$OUT/i_prove.txt" || { bad "prove on the directory did not prove the implementation's three delegated proofs into one sidecar"; grep -E 'PROVE:|PENDING|ERROR' "$OUT/i_prove.txt" | head -5 | cut -c1-300; }
for e in pick_le extra 'dn\.dec_1'; do
  checks=$((checks+1)); grep -q "^(proof-for $e (req " "$SCRATCH/lib/lib.auto.shard" 2>/dev/null || { bad "the implementation's sidecar lib/lib.auto.shard has no entry $e"; grep '^(proof-for' "$SCRATCH/lib/lib.auto.shard" 2>/dev/null | cut -c1-120; }
done
checks=$((checks+1)); [ ! -e "$SCRATCH/lib/mod.req.auto.shard" ] || bad "prove wrote a sidecar beside the view for the implementation's proofs"
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT "$SCRATCH/client.shard" "$SCRATCH/lib" > "$OUT/i_load.txt" 2>&1 &
"$EVAL" direct v3/kernel/prove.shard --root v3 $INIT "$SCRATCH/client.shard" "$SCRATCH/lib" > "$OUT/i_prove2.txt" 2>&1 &
wait
checks=$((checks+1)); { grep -q "^DISCHARGE $L.pick_le proved$" "$OUT/i_load.txt" && grep -q "^DISCHARGE $L.dn.dec_1 proved$" "$OUT/i_load.txt" && grep -q "^IMPL ACCEPT $L.extra " "$OUT/i_load.txt" && ! grep -q 'PENDING' "$OUT/i_load.txt" && grep -q 'pending 0 .*errors 0$' "$OUT/i_load.txt"; } || { bad "a load does not replay the implementation's sidecar"; grep -E 'PENDING|ERROR|^LOAD:' "$OUT/i_load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q "PROVED" "$OUT/i_load.txt" && bad "the build searched in the implementation (a PROVED record under load.shard)"
checks=$((checks+1)); grep -q 'proved 0  unsolved 0  sidecars written 0$' "$OUT/i_prove2.txt" || { bad "a second prove, on the consumer and the directory, searched again"; grep -E 'PROVE:' "$OUT/i_prove2.txt" | cut -c1-300; }
echo "engine_test: auto inside an implementation is proved once into the sidecar beside the implementation file and replayed from it"

# ---- (2) the count over calc
measure() { "$EVAL" direct v3/kernel/prove.shard --root v3 $INIT --measure "v3/examples/calc/$1.shard" > "$OUT/m_$1.txt" 2>&1; }
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
