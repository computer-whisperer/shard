#!/usr/bin/env bash
# v3/kernel/test/tactic_lib_test.sh — the tactic library (v3/LANGUAGE.md §8.4 slice 3.29 rules
# 2, 4, 7, 8 and 9; law §12.5's T10 row) over v3/examples/tactic: bool_split.shard, an ordinary
# E library whose `solve` reads a goal as text and answers a block of I, and use_split.shard,
# six theorems delegated `(auto (tactic examples.tactic.bool_split.solve))` with their committed
# sidecar. (1) The replay: the build accepts every theorem from the sidecar, loads the library
# as any E module and runs no producer. (2) The regeneration: a scratch copy under v3 (the
# loader reads repository-relative paths; the delegation renamed to the copy's module) without
# its sidecar, prove.shard running the library — the sidecar it writes is byte-identical to the
# committed one (the statements name Init's constants only, so the fingerprints are the same).
# (3) Rule 8, one solution checked identically: the engine solves the same statements in a
# second copy delegated `auto`; the two sidecars differ, both replay to the same ACCEPT records
# but for the axiom closure (the proof's: the engine's arith rests on propext and Quot.sound,
# the library's exact term on nothing), both are stored and verified by K alone. (4) Rule 4: the copy's sidecar without its schema
# form replays as schema 1, reported, never refused. (5) Rule 7: the delegation with a hint
# added replays unchanged. (6) --measure runs both producers on every statement.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "tactic_lib_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
OUT=$(mktemp -d)
A=$(mktemp -d -p v3 tactic_scratch_XXXXXX)   # the library's copy
B=$(mktemp -d -p v3 tactic_scratch_XXXXXX)   # the engine's copy
trap 'rm -rf "$OUT" "$A" "$B"' EXIT
fail=0; checks=0
bad() { echo "tactic_lib_test: $*"; fail=$((fail+1)); }
load() { "$EVAL" direct v3/kernel/load.shard --root v3 $INIT "$@"; }
prove() { "$EVAL" direct v3/kernel/prove.shard --root v3 $INIT "$@"; }
verify() { "$EVAL" direct v3/kernel/verify_release.shard "$1" "$EXPORT" "$INDEX"; }
NAMES="and_comm and_assoc de_morgan excluded le_self eq_self"
MA=$(basename "$A"); MB=$(basename "$B")

# ---- (1) the replay
load v3/examples/tactic/use_split.shard > "$OUT/replay.txt" 2>&1
checks=$((checks+1)); grep -q 'accepted 17 .*pending 0 .*errors 0$' "$OUT/replay.txt" || { bad "the example does not load clean"; grep -E 'ERROR|REFUSE|PENDING|^LOAD:' "$OUT/replay.txt" | head -4 | cut -c1-300; }
for t in $NAMES; do checks=$((checks+1)); grep -q "^ACCEPT examples.tactic.use_split.$t " "$OUT/replay.txt" || bad "$t not accepted from the sidecar"; done
checks=$((checks+1)); grep -q '^PROVED\|^TACTIC\|^ENGINE' "$OUT/replay.txt" && bad "the build ran a producer (a PROVED, TACTIC or ENGINE record under load.shard)"
checks=$((checks+1)); grep -q '^SIDECAR' "$OUT/replay.txt" && bad "the committed sidecar is not at the current schema"
checks=$((checks+1)); grep -q '^RUNNABLE fn examples.tactic.bool_split.solve ' "$OUT/replay.txt" || bad "the library is not loaded as an ordinary E module"
echo "tactic_lib_test: the build replays 6 theorems from the sidecar with the library loaded and never run"

# ---- (2) the regeneration through the library
lib() { sed 's#"\.\./\.\./kernel/sexpr\.shard"#"../kernel/sexpr.shard"#' v3/examples/tactic/bool_split.shard > "$1/bool_split.shard"; }   # the copy is one level under v3, the example two
lib "$A"
sed "s/examples\.tactic\.bool_split\.solve/$MA.bool_split.solve/" v3/examples/tactic/use_split.shard > "$A/use_split.shard"
prove "$A/use_split.shard" > "$OUT/proveA.txt" 2>&1
checks=$((checks+1)); grep -q 'proved 6  unsolved 0  sidecars written 1$' "$OUT/proveA.txt" || { bad "prove did not solve 6 of 6 through the library"; grep -E 'PROVE:|PENDING|ERROR' "$OUT/proveA.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); cmp -s "$A/use_split.auto.shard" v3/examples/tactic/use_split.auto.shard || { bad "the regenerated sidecar differs from the committed one"; diff "$A/use_split.auto.shard" v3/examples/tactic/use_split.auto.shard | head -6 | cut -c1-300; }
checks=$((checks+1)); cmp -s <(sed 's#"\.\./kernel/sexpr\.shard"#"../../kernel/sexpr.shard"#' "$A/bool_split.shard") v3/examples/tactic/bool_split.shard || bad "prove wrote into the library's source"
echo "tactic_lib_test: prove regenerates the sidecar through the library, byte-identical"

# ---- (3) rule 8: the engine's solution for the same statements, checked identically
lib "$B"
sed 's/(auto (tactic [^)]*))/auto/' v3/examples/tactic/use_split.shard > "$B/use_split.shard"
prove "$B/use_split.shard" > "$OUT/proveB.txt" 2>&1
checks=$((checks+1)); grep -q 'proved 6  unsolved 0  sidecars written 1$' "$OUT/proveB.txt" || { bad "the engine did not solve 6 of 6"; grep -E 'PROVE:|PENDING|ERROR' "$OUT/proveB.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); cmp -s "$A/use_split.auto.shard" "$B/use_split.auto.shard" && bad "the engine's sidecar equals the library's (two producers, one solution each)"
load "$A/use_split.shard" > "$OUT/replayA.txt" 2>&1
load "$B/use_split.shard" > "$OUT/replayB.txt" 2>&1
acc() { grep "^ACCEPT $2.use_split\." "$1" | sed "s/$2/M/g" | sed 's/ axioms=[^ ]* / axioms=- /'; }   # the axiom closure is the proof's, not the statement's: the engine's arith rests on propext and Quot.sound, the library's exact on nothing
checks=$((checks+1)); [ "$(acc "$OUT/replayA.txt" "$MA" | wc -l)" = 6 ] || { bad "the library's copy does not replay 6 theorems"; grep -E 'PENDING|ERROR|^LOAD:' "$OUT/replayA.txt" | head -3 | cut -c1-300; }
checks=$((checks+1)); cmp -s <(acc "$OUT/replayA.txt" "$MA") <(acc "$OUT/replayB.txt" "$MB") || { bad "the two sidecars do not replay to the same ACCEPT records (but for the axiom closure)"; diff <(acc "$OUT/replayA.txt" "$MA") <(acc "$OUT/replayB.txt" "$MB") | head -6 | cut -c1-300; }
checks=$((checks+1)); grep -q "^ACCEPT $MB.use_split.le_self .* axioms=Quot.sound,propext " "$OUT/replayB.txt" && grep -q "^ACCEPT $MA.use_split.le_self .* axioms= " "$OUT/replayA.txt" || bad "le_self's axiom closures are not the proofs' (arith: propext and Quot.sound; exact: none)"
checks=$((checks+1)); grep -q '^PROVED\|^TACTIC' "$OUT/replayA.txt" "$OUT/replayB.txt" && bad "a replay ran a producer"
mkdir -p "$OUT/storeA" "$OUT/storeB"
load --store "$OUT/storeA" "$A/use_split.shard" > "$OUT/storeA.txt" 2>&1
load --store "$OUT/storeB" "$B/use_split.shard" > "$OUT/storeB.txt" 2>&1
checks=$((checks+1)); grep -q '^STORE: ' "$OUT/storeA.txt" && grep -q '^STORE: ' "$OUT/storeB.txt" || bad "a copy did not store"
verify "$OUT/storeA" > "$OUT/vA.txt" 2>&1; rcA=$?
verify "$OUT/storeB" > "$OUT/vB.txt" 2>&1; rcB=$?
checks=$((checks+1)); [ "$rcA" = 0 ] || { bad "verify_release failed on the library's store (exit $rcA)"; grep -v '^ACCEPT' "$OUT/vA.txt" | head -4 | cut -c1-300; }
checks=$((checks+1)); [ "$rcB" = 0 ] || { bad "verify_release failed on the engine's store (exit $rcB)"; grep -v '^ACCEPT' "$OUT/vB.txt" | head -4 | cut -c1-300; }
checks=$((checks+1)); grep -q '^RELEASE: declarations [0-9]*  accepted [0-9]* .* malformed 0  mistied 0  missing 0  taken 0 ' "$OUT/vA.txt" && grep -q '^RELEASE: declarations [0-9]*  accepted [0-9]* .* malformed 0  mistied 0  missing 0  taken 0 ' "$OUT/vB.txt" || bad "a release line is not clean"
checks=$((checks+1)); [ "$(grep -c '^ACCEPT' "$OUT/vA.txt")" = "$(grep -c '^ACCEPT' "$OUT/vB.txt")" ] || bad "the two stores do not verify the same number of declarations"
echo "tactic_lib_test: the engine's and the library's sidecars differ, replay to the same ACCEPT records, and both stores verify by K alone"

# ---- (4) rule 4: the schema-1 copy reported
sed -i '/^(sidecar 2)$/d' "$A/use_split.auto.shard"
load "$A/use_split.shard" > "$OUT/replayA1.txt" 2>&1
checks=$((checks+1)); grep -q "^SIDECAR .*use_split.shard schema=1 current=2$" "$OUT/replayA1.txt" || bad "the schema-1 sidecar is not reported"
checks=$((checks+1)); [ "$(acc "$OUT/replayA1.txt" "$MA" | wc -l)" = 6 ] || bad "the schema-1 sidecar does not replay 6 theorems"
checks=$((checks+1)); grep -q 'pending 0 .*errors 0$' "$OUT/replayA1.txt" || bad "the schema-1 sidecar is refused"
# ---- (5) rule 7: the delegation with a hint added replays unchanged
sed -i "s/(auto (tactic \([^)]*\)))/(auto (tactic \1) (budget 1))/" "$A/use_split.shard"
load "$A/use_split.shard" > "$OUT/replayA2.txt" 2>&1
checks=$((checks+1)); [ "$(acc "$OUT/replayA2.txt" "$MA" | wc -l)" = 6 ] || { bad "the hints-changed copy does not replay 6 theorems"; grep -E 'PENDING|ERROR' "$OUT/replayA2.txt" | head -3 | cut -c1-300; }
echo "tactic_lib_test: an older schema is reported and replays; changed hints replay unchanged"

# ---- (6) --measure: both producers on every statement
prove --measure v3/examples/tactic/use_split.shard > "$OUT/measure.txt" 2>&1
checks=$((checks+1)); grep -q '^ENGINE: solved 6 of 6$' "$OUT/measure.txt" || bad "the engine does not close 6 of 6 under --measure"
checks=$((checks+1)); [ "$(grep -c '^TACTIC examples.tactic.use_split.[a-z_]* examples.tactic.bool_split.solve solved$' "$OUT/measure.txt")" = 6 ] || bad "the tactic does not close 6 of 6 under --measure"
echo "tactic_lib_test: $checks checks, $fail failed"
exit $fail
