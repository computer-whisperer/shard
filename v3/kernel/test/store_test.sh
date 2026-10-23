#!/usr/bin/env bash
# v3/kernel/test/store_test.sh — the store and the release gate (v3/LANGUAGE.md §8.4 slice
# 3.22, landing 3; law §7.5). (1) v3/examples/auto under `load.shard --store`: 23 declarations
# written as the records K ingests, DIR/order in admission order; a second load of the same
# module writes nothing new and lists nothing twice; verify_release.shard — K, the host and
# the JSON reader, no elaborator — replays the store after the closure of Init it cites and accepts every
# declaration. Then the gate refuses what it must: a theorem whose value is replaced by its
# statement is REJECTED, an expression record removed is MALFORMED, a file removed is MISSING,
# a name record removed is MISTIED (the declaration lands under another name), each with a
# non-zero exit; then slice 4.0's tampers (R79, R80): a second rejected record and a second
# accepted one in a file, the theorem replaced by an axiom, a proof resting on an undeclared
# axiom, a target removed from order, a discharge removed from the manifest — each a non-zero
# exit —, and a discharged store verifying with its DISCHARGE lines. (2) calc: the five leaves of tactic_test.sh stored in sequence into
# v3/release/calc (the CI artifact; shared modules written once) and replayed by K — every
# declaration accepted, the count printed. Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "store_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index (slice 3.26)
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "store_test: $*"; fail=$((fail+1)); }
load_store() { "$EVAL" direct v3/kernel/load.shard --root v3 $INIT --store "$1" "$2"; }
verify() { "$EVAL" direct v3/kernel/verify_release.shard "$1" "$EXPORT" "$INDEX"; }

# ---- (1) the example
mkdir -p "$OUT/auto"
load_store "$OUT/auto" v3/examples/auto/auto.shard > "$OUT/load1.txt" 2>&1
checks=$((checks+1)); grep -q '^STORE: .* new=23 shared=0$' "$OUT/load1.txt" || { bad "the example's store is not 23 new declarations"; tail -2 "$OUT/load1.txt" | cut -c1-300; }
checks=$((checks+1)); [ "$(wc -l < "$OUT/auto/order")" = 23 ] || bad "order does not list 23 declarations"
checks=$((checks+1)); [ "$(ls "$OUT/auto" | grep -c '\.ndjson$')" = 23 ] || bad "the store does not hold 23 files"
load_store "$OUT/auto" v3/examples/auto/auto.shard > "$OUT/load2.txt" 2>&1
checks=$((checks+1)); grep -q '^STORE: .* new=0 shared=23$' "$OUT/load2.txt" || { bad "the second load did not share every declaration"; tail -1 "$OUT/load2.txt" | cut -c1-300; }
checks=$((checks+1)); [ "$(wc -l < "$OUT/auto/order")" = 23 ] || bad "the second load changed order"
verify "$OUT/auto" > "$OUT/v1.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "verify_release failed on the example (exit $rc)"; grep -v '^ACCEPT' "$OUT/v1.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^RELEASE: declarations 23  accepted 23 .* malformed 0  mistied 0  missing 0  taken 0  unlisted 0  miskind 0  cardinality 0  undeclared 0  conditional 0$' "$OUT/v1.txt" || { bad "the release line is not 23 of 23"; tail -1 "$OUT/v1.txt" | cut -c1-300; }
checks=$((checks+1)); [ "$(head -1 "$OUT/auto/manifest")" = "manifest 1" ] && [ "$(grep -c '^decl ' "$OUT/auto/manifest")" = 23 ] || bad "the manifest does not list 23 declarations under its header"
echo "store_test: the example's 23 declarations stored once and accepted by K alone"

# the gate refuses: a theorem's value replaced by its statement (REJECT), a record removed (MALFORMED), a file removed (MISSING)
id=$(grep ' examples.auto.auto.le_step$' "$OUT/auto/order" | cut -d' ' -f1)
cp -r "$OUT/auto" "$OUT/t1"; sed -i -E 's/"type":([0-9]+),"value":[0-9]+/"type":\1,"value":\1/' "$OUT/t1/$id.ndjson"
verify "$OUT/t1" > "$OUT/t1.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^REJECT .* examples.auto.auto.le_step$' "$OUT/t1.txt" && grep -q '^RELEASE: declarations 23  accepted 22  rejected 1 ' "$OUT/t1.txt" || { bad "a theorem proved by its own statement was not rejected (exit $rc)"; tail -2 "$OUT/t1.txt" | cut -c1-300; }
cp -r "$OUT/auto" "$OUT/t2"; sed -i '/"ie":1,/d' "$OUT/t2/$id.ndjson"
verify "$OUT/t2" > "$OUT/t2.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^MALFORMED' "$OUT/t2.txt" && grep -q 'malformed [1-9][0-9]*  mistied 0  missing 0  taken 0 ' "$OUT/t2.txt" || { bad "an expression record removed was not malformed (exit $rc)"; tail -2 "$OUT/t2.txt" | cut -c1-300; }
cp -r "$OUT/auto" "$OUT/t3"; rm "$OUT/t3/$id.ndjson"
verify "$OUT/t3" > "$OUT/t3.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q "^MISSING $id examples.auto.auto.le_step$" "$OUT/t3.txt" && grep -q 'missing 1  taken 0 ' "$OUT/t3.txt" || { bad "a file removed was not missing (exit $rc)"; tail -2 "$OUT/t3.txt" | cut -c1-300; }
# a name record removed: the import reads the missing index as anonymous and K accepts the
# theorem under a shorter name — the gate holds each declaration to its order line's name
cp -r "$OUT/auto" "$OUT/t4"; sed -i '/"in":3,/d' "$OUT/t4/$id.ndjson"
verify "$OUT/t4" > "$OUT/t4.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^MISTIED examples.auto.auto.le_step accepted as ' "$OUT/t4.txt" && grep -q 'mistied 1  missing 0  taken 0 ' "$OUT/t4.txt" || { bad "a name record removed was not mistied (exit $rc)"; tail -2 "$OUT/t4.txt" | cut -c1-300; }
echo "store_test: the gate rejects a forged proof, a broken record, a missing file and a renamed declaration"

# ---- the reconstruction contract (slice 4.5 rule 2; law §7.2, §7.5): the example's P reconstructed
# the same, a differing retained P an evidence change, another reconstruction version named on
# every line and never written into; the gate reads the recon line past
reconstruct() { "$EVAL" direct v3/kernel/load.shard --root v3 $INIT --reconstruct "$1" "$2"; }
checks=$((checks+1)); [ "$(sed -n 2p "$OUT/auto/manifest")" = "recon 1" ] || bad "the manifest's second line is not the reconstruction version (recon 1)"
nthm=$(grep -c '^decl [0-9-]* thm ' "$OUT/auto/manifest")
reconstruct "$OUT/auto" v3/examples/auto/auto.shard > "$OUT/r1.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$(grep -c '^RECONSTRUCT examples.auto.auto.* same recon=1$' "$OUT/r1.txt")" = "$nthm" ] && grep -q "^RECONSTRUCT: theorems $nthm  same $nthm  changed 0  not-loaded 0 recon=1$" "$OUT/r1.txt" || { bad "the example does not reconstruct the same P for its $nthm theorems (exit $rc)"; grep -E '^(RECONSTRUCT|EVIDENCE)' "$OUT/r1.txt" | tail -3 | cut -c1-200; }
reconstruct "$OUT/t1" v3/examples/auto/auto.shard > "$OUT/r2.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 1 ] && grep -q '^EVIDENCE-CHANGE examples.auto.auto.le_step recon=1$' "$OUT/r2.txt" && grep -q "changed 1  not-loaded 0 recon=1$" "$OUT/r2.txt" || { bad "a retained P that differs is not an evidence change (exit $rc)"; grep -E '^(EVIDENCE|RECONSTRUCT:)' "$OUT/r2.txt" | head -3 | cut -c1-200; }
cp -r "$OUT/auto" "$OUT/r0"; sed -i 's/^recon 1$/recon 0/' "$OUT/r0/manifest"
reconstruct "$OUT/r0" v3/examples/auto/auto.shard > "$OUT/r3.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$(grep -c '^RECONSTRUCT examples.auto.auto.* same recon=0→1$' "$OUT/r3.txt")" = "$nthm" ] || { bad "a store at another reconstruction version does not name the versions (exit $rc)"; grep -E '^(RECONSTRUCT|EVIDENCE)' "$OUT/r3.txt" | tail -2 | cut -c1-200; }
load_store "$OUT/r0" v3/examples/auto/auto.shard > "$OUT/r4.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 2 ] && grep -q "^STORE: refused .*recon 0 is not this loader's 1" "$OUT/r4.txt" || { bad "a store at another reconstruction version accepted a write (exit $rc)"; tail -1 "$OUT/r4.txt" | cut -c1-200; }
verify "$OUT/r0" > "$OUT/r5.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the gate refuses a manifest with a recon line (exit $rc)"; grep -v '^ACCEPT' "$OUT/r5.txt" | head -3 | cut -c1-200; }
echo "store_test: the reconstruction contract — the example reconstructs the same P, a differing P is an evidence change, another version is named on every line and never written into"

# ---- the release gate's exit and target (slice 4.0 rules 2–3; R79, R80)
F="$OUT/auto/$id.ndjson"; decl=$(tail -1 "$F")
kin=$(grep -o '"in":[0-9]*' "$F" | tail -1 | tr -dc 0-9); kie=$(grep -o '"ie":[0-9]*' "$F" | tail -1 | tr -dc 0-9)
nrec=$(grep -o '{"in":[0-9]*,"str":{"pre":[0-9]*,"str":"le_step"}}' "$F"); nidx=$(echo "$nrec" | grep -o '"in":[0-9]*' | tr -dc 0-9); pre=$(echo "$nrec" | grep -o '"pre":[0-9]*' | tr -dc 0-9)
# R80: a second, rejected record in le_step's file (its value its statement) — one accepted, one rejected, exit nonzero
cp -r "$OUT/auto" "$OUT/t5"; echo "$decl" | sed -E 's/"type":([0-9]+),"value":[0-9]+/"type":\1,"value":\1/' >> "$OUT/t5/$id.ndjson"
verify "$OUT/t5" > "$OUT/t5.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^REJECT' "$OUT/t5.txt" && grep -q 'accepted 23  rejected 1 ' "$OUT/t5.txt" || { bad "a rejected second record did not fail the gate (exit $rc)"; grep -v '^ACCEPT' "$OUT/t5.txt" | tail -3 | cut -c1-300; }
# R80: a second record K accepts under another name — accepted by K, MISTIED against the manifest, exit nonzero (before: n − accepted + mistied = 0)
cp -r "$OUT/auto" "$OUT/t6"; sed -i "\$i {\"in\":$((kin+1)),\"str\":{\"pre\":$pre,\"str\":\"le_step_copy\"}}" "$OUT/t6/$id.ndjson"
echo "$decl" | sed "s/\"name\":$nidx,/\"name\":$((kin+1)),/" >> "$OUT/t6/$id.ndjson"
verify "$OUT/t6" > "$OUT/t6.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^MISTIED examples.auto.auto.le_step accepted as examples.auto.auto.le_step_copy' "$OUT/t6.txt" && grep -q 'accepted 24 .* mistied 1 ' "$OUT/t6.txt" || { bad "a second record accepted under another name did not fail the gate (exit $rc)"; grep -v '^ACCEPT' "$OUT/t6.txt" | tail -3 | cut -c1-300; }
# R79: the theorem replaced by a same-name same-type axiom — MISKIND, UNDECLARED, exit nonzero
cp -r "$OUT/auto" "$OUT/t8"; sed -i -E 's/\{"thm":\{"all":\[[0-9,]*\],/{"axiom":{"isUnsafe":false,/; s/,"value":[0-9]+\}\}$/}}/' "$OUT/t8/$id.ndjson"
verify "$OUT/t8" > "$OUT/t8.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^MISKIND examples.auto.auto.le_step: the manifest says thm, the record is axiom' "$OUT/t8.txt" && grep -q 'miskind 1 ' "$OUT/t8.txt" || { bad "an axiom in the theorem's place was not MISKIND (exit $rc)"; grep -v '^ACCEPT' "$OUT/t8.txt" | tail -3 | cut -c1-300; }
# R79: a proof resting on an axiom the manifest does not declare — the axiom's file added before le_step's (UNLISTED: no manifest line), le_step's value the axiom — CONDITIONAL, exit nonzero
cp -r "$OUT/auto" "$OUT/t9"; cp "$F" "$OUT/t9/ax.ndjson"
sed -i -E 's/"str":"le_step"\}\}$/"str":"le_step_ax"}}/; s/\{"thm":\{"all":\[[0-9,]*\],/{"axiom":{"isUnsafe":false,/; s/,"value":[0-9]+\}\}$/}}/' "$OUT/t9/ax.ndjson"
sed -i "/ examples.auto.auto.le_step\$/i ax examples.auto.auto.le_step_ax" "$OUT/t9/order"
sed -i "\$i {\"in\":$((kin+1)),\"str\":{\"pre\":$pre,\"str\":\"le_step_ax\"}}" "$OUT/t9/$id.ndjson"
sed -i "\$i {\"ie\":$((kie+1)),\"const\":{\"name\":$((kin+1)),\"us\":[]}}" "$OUT/t9/$id.ndjson"
sed -i -E "\$s/,\"value\":[0-9]+\}\}$/,\"value\":$((kie+1))}}/" "$OUT/t9/$id.ndjson"
verify "$OUT/t9" > "$OUT/t9.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^UNLISTED ax examples.auto.auto.le_step_ax' "$OUT/t9.txt" && grep -q '^UNDECLARED examples.auto.auto.le_step_ax: an axiom the manifest declares neither' "$OUT/t9.txt" && grep -q '^CONDITIONAL examples.auto.auto.le_step: rests on examples.auto.auto.le_step_ax' "$OUT/t9.txt" && grep -q 'unlisted 1 .* undeclared 1  conditional 1$' "$OUT/t9.txt" || { bad "a proof resting on an undeclared axiom was not CONDITIONAL (exit $rc)"; grep -v '^ACCEPT' "$OUT/t9.txt" | tail -4 | cut -c1-300; }
# R79: a manifest target removed from order with its file — MISSING, exit nonzero
cp -r "$OUT/auto" "$OUT/t10"; sed -i "/ examples.auto.auto.le_step\$/d" "$OUT/t10/order"; rm "$OUT/t10/$id.ndjson"
verify "$OUT/t10" > "$OUT/t10.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q "^MISSING $id examples.auto.auto.le_step (a manifest declaration without an order line)" "$OUT/t10.txt" && grep -q 'missing 1 ' "$OUT/t10.txt" || { bad "a target removed from order was not MISSING (exit $rc)"; grep -v '^ACCEPT' "$OUT/t10.txt" | tail -3 | cut -c1-300; }
# R79: a discharged obligation — the pin discharge_measure stored: its three obligations DISCHARGE lines, no parameter, exit 0; the discharge of count.dec_1 removed from the manifest: the function and its theorem rest on the obligation, CONDITIONAL, exit nonzero
mkdir -p "$OUT/dm"
"$EVAL" direct v3/kernel/load.shard --root v3/pins/loader/discharge_measure $INIT --store "$OUT/dm" v3/pins/loader/discharge_measure/main.shard > "$OUT/dm_load.txt" 2>&1
checks=$((checks+1)); grep -q '^STORE: ' "$OUT/dm_load.txt" && [ "$(grep -c '^discharge ' "$OUT/dm/manifest")" = 3 ] && ! grep -q '^param ' "$OUT/dm/manifest" || { bad "discharge_measure did not store with three discharge lines and no parameter"; grep -E 'ERROR|REFUSE|PENDING|^LOAD:|^STORE:' "$OUT/dm_load.txt" | head -3 | cut -c1-300; cat "$OUT/dm/manifest" 2>/dev/null | grep -v '^decl' | head -5; }
verify "$OUT/dm" > "$OUT/dm1.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$(grep -c '^DISCHARGE ' "$OUT/dm1.txt")" = 3 ] && grep -q 'undeclared 0  conditional 0$' "$OUT/dm1.txt" || { bad "the discharged store did not verify with three DISCHARGE lines (exit $rc)"; grep -v '^ACCEPT' "$OUT/dm1.txt" | tail -4 | cut -c1-300; }
cp -r "$OUT/dm" "$OUT/dm2"; sed -i '/^discharge main.count.dec_1 /d' "$OUT/dm2/manifest"
verify "$OUT/dm2" > "$OUT/dm2.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^CONDITIONAL main.count: rests on main.count.dec_1' "$OUT/dm2.txt" && grep -q 'undeclared 1  conditional [1-9]' "$OUT/dm2.txt" || { bad "a discharge removed from the manifest did not make its function CONDITIONAL (exit $rc)"; grep -v '^ACCEPT' "$OUT/dm2.txt" | tail -4 | cut -c1-300; }
echo "store_test: the gate's exit and target hold against the memo's tampers"

# ---- (2) calc: the five leaves into the release store, K replays it
REL=v3/release/calc
rm -rf "$REL"; mkdir -p "$REL"
s=$(date +%s)
for f in calc_app_world calc_show_run calc_ndigit calc_spec_tests calc_reconcile_tests; do
  load_store "$REL" "v3/examples/calc/$f.shard" > "$OUT/c_$f.txt" 2>&1
  checks=$((checks+1)); grep -q '^STORE: ' "$OUT/c_$f.txt" || { bad "$f did not store"; grep -E 'ERROR|REFUSE|PENDING|^LOAD:|^STORE:' "$OUT/c_$f.txt" | head -3 | cut -c1-300; }
done
e=$(date +%s)
n=$(wc -l < "$REL/order")
echo "store_test: calc's five leaves stored in $((e-s)) s: $n declarations, $(du -sk "$REL" | cut -f1) kB"
s=$(date +%s)
verify "$REL" > "$OUT/vc.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "verify_release failed on calc (exit $rc)"; grep -v '^ACCEPT' "$OUT/vc.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q "^RELEASE: declarations $n  accepted $n " "$OUT/vc.txt" || { bad "K did not accept every one of calc's $n declarations"; tail -1 "$OUT/vc.txt" | cut -c1-300; }
echo "store_test: K alone accepts calc's $n declarations after Init's closure in $((e-s)) s"
# R80: an inductive's exported metadata altered after its acceptance — MISMATCH counted, exit nonzero
cp -r "$REL" "$OUT/t7"; f=$(grep -l '"isRec":false' "$OUT/t7"/*.ndjson | head -1)
checks=$((checks+1)); [ -n "$f" ] || bad "calc's store holds no non-recursive inductive to tamper"
[ -n "$f" ] && sed -i '0,/"isRec":false/s//"isRec":true/' "$f"
verify "$OUT/t7" > "$OUT/t7.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] && grep -q '^MISMATCH isRec ' "$OUT/t7.txt" && grep -q 'mismatched 1 ' "$OUT/t7.txt" || { bad "an inductive's metadata mismatch did not fail the gate (exit $rc)"; grep -v '^ACCEPT' "$OUT/t7.txt" | tail -3 | cut -c1-300; }
echo "store_test: $checks checks, $fail failed"
exit $fail
