#!/usr/bin/env bash
# The first bin (v3/LANGUAGE.md §8.6 slice 4.3 landing 1, rules 4–5; law §4.7, §9.3):
# v3/examples/calc/calc_bin.shard declares calc's main as a bin over calc_main.shard —
# three observables as opaque constants, nine bolt axioms, two requirements proven through
# the model world (main_output, main_reads). The load records the bin (BIN … requires 2/2)
# and every function of its closure THREAD ok; the store's manifest carries the bin line
# and the release gate verifies it (BIN … ok); the requirements' closures are Init's two
# and the bolts they use; the program runs through the driver with the bin file as the
# root file, byte-identical to the model world as calc_main_test.sh ties it; and two
# negatives — the bin with a requirement left `sorry` fails the load with UNMET, and a
# program with a write duplicated fails it with UNTHREADED — each a nonzero exit.
# Runs on either engine (EVAL); under v3/test.sh the two runs are compared byte for byte.
# Exit code = the number of failed checks.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "bin_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index (slice 3.26)
INPUTS=v3/kernel/test/fixtures/calc_inputs.txt
OUT=$(mktemp -d); ROOT=$(mktemp -d -p v3 bin_scratch_XXXXXX); trap 'rm -rf "$OUT" "$ROOT"' EXIT   # the scratch root inside the tree: the loader takes paths relative to the working directory
mkdir -p "$OUT/store"
fail=0; checks=0
bad() { echo "bin_test: $*"; fail=$((fail+1)); }
BIN=examples.calc.calc_bin.calc

# ---- (1) the load: the record, the closure's verdicts, the requirements' closures
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --store "$OUT/store" v3/examples/calc/calc_bin.shard > "$OUT/load.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the bin's load failed (exit $rc)"; grep -E '^(READ-ERROR|LOAD-ERROR|REFUSE|UNMET|UNTHREADED|BAD-BIN|LOAD:)' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -qx "BIN $BIN entry examples.calc.calc_main.main externs 3 trusts 9 requires 2/2" "$OUT/load.txt" || { bad "the bin's record is not BIN … externs 3 trusts 9 requires 2/2"; grep '^BIN' "$OUT/load.txt" | cut -c1-300; }
checks=$((checks+1)); [ "$(grep -c '^\(UNMET\|UNTHREADED\|BAD-BIN\) ' "$OUT/load.txt")" = 0 ] || { bad "the bin has failure lines"; grep '^\(UNMET\|UNTHREADED\|BAD-BIN\) ' "$OUT/load.txt" | head -3 | cut -c1-300; }
for f in perform drive_host main; do
  checks=$((checks+1)); grep -qx "THREAD examples.calc.calc_main.$f ok" "$OUT/load.txt" || bad "calc_main.$f is not THREAD ok"
done
checks=$((checks+1)); [ "$(grep -c '^PARAM examples.calc.calc_bin.[a-z_]* axiom$' "$OUT/load.txt")" = 9 ] || bad "the nine bolts are not nine PARAM … axiom records"
for x in get_args write_line exit; do
  checks=$((checks+1)); grep -q "^ACCEPT std.host.$x " "$OUT/load.txt" || bad "the extern std.host.$x has no L constant (ACCEPT)"
done
# the requirements' closures: Init's two (propext, Quot.sound) and the bolts each uses, nothing else
closure() { sed -nE "s/^ACCEPT $1 .* axioms=([^ ]*) .*/\1/p" "$OUT/load.txt" | tr ',' '\n' | LC_ALL=C sort | tr '\n' ' '; }
want_output="Quot.sound examples.calc.calc_bin.exit_output examples.calc.calc_bin.get_args_output examples.calc.calc_bin.write_line_output propext "
want_reads="Quot.sound examples.calc.calc_bin.exit_reads examples.calc.calc_bin.get_args_reads examples.calc.calc_bin.write_line_reads propext "
checks=$((checks+1)); [ "$(closure examples.calc.calc_bin.main_output)" = "$want_output" ] || { bad "main_output's closure is not Init's two and its three bolts"; closure examples.calc.calc_bin.main_output; }
checks=$((checks+1)); [ "$(closure examples.calc.calc_bin.main_reads)" = "$want_reads" ] || { bad "main_reads's closure is not Init's two and its three bolts"; closure examples.calc.calc_bin.main_reads; }
n=$(sed -nE 's/^LOAD: modules [0-9]+  accepted ([0-9]+) .*/\1/p' "$OUT/load.txt")
echo "bin_test: calc's bin loaded and stored in $((e-s)) s — $n declarations accepted, BIN $BIN requires 2/2"

# ---- (2) the store's manifest and the release gate
checks=$((checks+1)); grep -qx "bin $BIN entry examples.calc.calc_main.main externs std.host.get_args std.host.write_line std.host.exit requires examples.calc.calc_bin.main_output examples.calc.calc_bin.main_reads" "$OUT/store/manifest" || { bad "the manifest lacks the bin line"; grep '^bin' "$OUT/store/manifest" | cut -c1-300; }
stored=$(sed -nE 's/^STORE: .* new=([0-9]+) .*/\1/p' "$OUT/load.txt" | head -1)
checks=$((checks+1)); [ -n "$stored" ] || { bad "no store was written"; stored=0; }
s=$(date +%s)
"$EVAL" direct v3/kernel/verify_release.shard "$OUT/store" "$EXPORT" "$INDEX" > "$OUT/verify.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "verify_release failed on the bin's store (exit $rc)"; grep -v '^ACCEPT' "$OUT/verify.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -qx "BIN $BIN ok" "$OUT/verify.txt" || { bad "the gate does not say BIN $BIN ok"; grep '^\(BIN\|BAD-BIN\)' "$OUT/verify.txt" | cut -c1-300; }
checks=$((checks+1)); grep -q "^RELEASE: declarations $stored  accepted $stored " "$OUT/verify.txt" || { bad "K did not accept every one of the store's $stored declarations"; tail -1 "$OUT/verify.txt" | cut -c1-300; }
echo "bin_test: K alone accepts the store's $stored declarations and the gate says BIN $BIN ok in $((e-s)) s"

# a tampered manifest: the bin's requirement replaced by a definition's name is BAD-BIN
cp -r "$OUT/store" "$OUT/t1"; sed -i 's/requires examples.calc.calc_bin.main_output examples.calc.calc_bin.main_reads$/requires examples.calc.calc_bin.main_output examples.calc.calc_bin.line_bytes/' "$OUT/t1/manifest"
"$EVAL" direct v3/kernel/verify_release.shard "$OUT/t1" "$EXPORT" "$INDEX" > "$OUT/verify_t1.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 1 ] || bad "the gate accepts a bin line naming a definition as a requirement (exit $rc)"
checks=$((checks+1)); grep -q "^BAD-BIN $BIN: examples.calc.calc_bin.line_bytes is not an accepted theorem of the store" "$OUT/verify_t1.txt" || { bad "the tampered bin line is not BAD-BIN"; grep '^\(BIN\|BAD-BIN\)' "$OUT/verify_t1.txt" | cut -c1-300; }

# ---- (3) the program through the driver, the bin file the root file, tied to the model world
lines=()
while IFS= read -r line || [ -n "$line" ]; do lines+=("$line"); done < "$INPUTS"
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --run examples.calc.calc_main.main --trace "$OUT/trace.txt" v3/examples/calc/calc_bin.shard -- "${lines[@]}" > "$OUT/got" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the program failed through the bin (exit $rc)"; head -3 "$OUT/got" | cut -c1-300; }
"$EVAL" direct v3/kernel/test/calc_harness.shard "$EXPORT" "$INDEX" "$INPUTS" trace > "$OUT/model" 2>&1 || { bad "the harness failed"; head -3 "$OUT/model"; }
grep '^\* run_world: ' "$OUT/model" | tr '()' '  ' | awk '
  { st = 0
    for (i = 4; i <= NF; i++) {
      if (st == 0) { if ($i == "Cons") st = 1; else if ($i == "Nil") break }
      else if ($i == "Cons") { i++; printf "%c", $i + 0 }
      else if ($i == "Nil") { printf "\n"; st = 0 } } }' > "$OUT/want"
checks=$((checks+1)); [ "$(wc -l < "$OUT/want")" -eq "${#lines[@]}" ] || bad "the model has $(wc -l < "$OUT/want") lines for ${#lines[@]} inputs"
checks=$((checks+1)); cmp -s "$OUT/got" "$OUT/want" || { bad "the program's output through the bin is not the model world's"; diff "$OUT/got" "$OUT/want" | head -6; }
echo "bin_test: the program through the bin in $((e-s)) s, byte-identical to the model world over ${#lines[@]} inputs"

# ---- (3b) the effect trace (slice 4.3 landing 2; law §4.7 (iii)): the driver's --trace FILE —
# one host call per line, the arguments read then one write_line per input — against the
# model's trace (calc_app_world.shard trace_of, rendered by the harness as `* trace:` lines
# with the driver's own renderer): the call lists identical, byte for byte
checks=$((checks+1)); [ -s "$OUT/trace.txt" ] || bad "the driver wrote no trace"
sed -n 's/^\* trace: //p' "$OUT/model" > "$OUT/model_trace.txt"
checks=$((checks+1)); [ "$(wc -l < "$OUT/trace.txt")" -eq $((${#lines[@]} + 1)) ] || bad "the trace has $(wc -l < "$OUT/trace.txt") calls for ${#lines[@]} inputs (one get_args and one write_line each)"
checks=$((checks+1)); head -1 "$OUT/trace.txt" | grep -q '^get_args "' || { bad "the trace does not open with get_args"; head -1 "$OUT/trace.txt" | cut -c1-200; }
checks=$((checks+1)); cmp -s "$OUT/trace.txt" "$OUT/model_trace.txt" || { bad "the driver's trace is not the model's"; diff "$OUT/trace.txt" "$OUT/model_trace.txt" | head -6 | cut -c1-200; }
echo "bin_test: the trace — $(wc -l < "$OUT/trace.txt") host calls — is the model's, byte for byte"

# ---- (3c) the bolt bundle on the model host (rule 6 (e); law §4.7 (ii)): the nine axioms the
# bin trusts, instantiated at calc_bin_model.shard's Host and proven, and a ground run computed
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/examples/calc/calc_bin_model.shard > "$OUT/model_load.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the model host's file failed to load (exit $rc)"; grep -E '^(READ-ERROR|LOAD-ERROR|REFUSE|LOAD:)' "$OUT/model_load.txt" | head -3 | cut -c1-300; }
for t in get_args_output get_args_reads get_args_exit write_line_output write_line_reads write_line_exit exit_output exit_reads exit_exit ground; do
  checks=$((checks+1)); grep -q "^ACCEPT examples.calc.calc_bin_model.m_$t .* axioms= " "$OUT/model_load.txt" || bad "the model host's m_$t is not accepted on no axioms"
done
echo "bin_test: the nine bolts hold on the model host, with the ground run, in $((e-s)) s"

# ---- (4) the negatives, over a scratch root that sees the library through a link
mkdir -p "$ROOT/examples"; ln -s ../std "$ROOT/std"; cp -r v3/examples/calc "$ROOT/examples/calc"
# a requirement left sorry: UNMET, requires 1/2, the load fails
python3 -I - v3/examples/calc/calc_bin.shard "$ROOT/examples/calc/calc_bin.shard" <<'PY'
import sys
s = open(sys.argv[1]).read()
i = s.find('(theorem main_reads '); j = s.find('\n(theorem', i + 1); k = s.find('\n;; ----', i + 1)
end = min(x for x in (j, k) if x > 0)
assert i > 0 and end > i
head = s[i:s.find('\n', i)]            # the statement line, its proof on the lines after it
assert head.endswith('(w_reads w))'), head
open(sys.argv[2], 'w').write(s[:i] + head + ' sorry)' + s[end:])
PY
grep -q '^(theorem main_reads .* sorry)$' "$ROOT/examples/calc/calc_bin.shard" || bad "the sorry variant was not written"
"$EVAL" direct v3/kernel/load.shard --root "$ROOT" $INIT "$ROOT/examples/calc/calc_bin.shard" > "$OUT/unmet.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] || bad "a bin with a requirement left sorry loads clean"
checks=$((checks+1)); grep -qx "BIN $BIN entry examples.calc.calc_main.main externs 3 trusts 9 requires 1/2" "$OUT/unmet.txt" || { bad "the sorry variant's record is not requires 1/2"; grep -E '^(BIN|READ-ERROR|LOAD-ERROR|REFUSE)' "$OUT/unmet.txt" | head -3 | cut -c1-300; }
checks=$((checks+1)); grep -qx "UNMET examples.calc.calc_bin.main_reads" "$OUT/unmet.txt" || { bad "the sorry variant is not UNMET main_reads"; grep '^UNMET' "$OUT/unmet.txt" | cut -c1-200; }
# a write duplicated in the program: UNTHREADED, the load fails (the requirements over it left
# sorry — the point is the threading verdict on the bin's closure)
sed 's/^    ((Print cs) (write_line (Bytes.ofList (bytes_of cs)) w))$/    ((Print cs) (match (write_line (Bytes.ofList (bytes_of cs)) w) (_ (write_line (Bytes.ofList (bytes_of cs)) w))))/' v3/examples/calc/calc_main.shard > "$ROOT/examples/calc/calc_main.shard"
grep -q '(_ (write_line (Bytes.ofList (bytes_of cs)) w))))' "$ROOT/examples/calc/calc_main.shard" || bad "the duplicated-write variant was not written"
cat > "$ROOT/examples/calc/calc_bin.shard" <<'EOF'
(import "calc_main.shard")
(use examples.calc.calc_main)
(use std.host)
(theorem main_total ((w World)) (= (main w) (main w)) (exact (Eq.refl (main w))))
(bin calc (entry main) (externs get_args write_line exit) (trusts) (requires main_total))
EOF
"$EVAL" direct v3/kernel/load.shard --root "$ROOT" $INIT "$ROOT/examples/calc/calc_bin.shard" > "$OUT/dup.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] || bad "a bin over a program with a write duplicated loads clean"
checks=$((checks+1)); grep -qx "UNTHREADED examples.calc.calc_main.perform world_twice: parameter 2 used twice: std.host.write_line, std.host.write_line" "$OUT/dup.txt" || { bad "the duplicated write is not UNTHREADED perform world_twice"; grep -E '^(UNTHREADED|BIN|THREAD examples.calc.calc_main.perform|READ-ERROR|LOAD-ERROR)' "$OUT/dup.txt" | head -3 | cut -c1-300; }

echo "bin_test: $checks checks, $fail failed"
exit $fail
