#!/usr/bin/env bash
# v3/kernel/test/lower_test.sh — the lowering's own gate: the restricted form RS is a Prog,
# so `ev` runs it. Each program below is run twice by
# the loader's driver — as loaded, and under `--lowered`, its restricted form (kernel/lower.shard)
# in its place — and the two runs must agree on stdout, stderr and the exit status, byte for
# byte. The generator (kernel/cgen.shard) is not in this test: a difference here is the
# lowering's; a difference between a binary and the interpreter that this test does not show is
# the generator's or the runtime's (native_tie_test.sh).
#
# THE PROGRAMS: two ordinary cases of v3/pins/emit — a string literal over Init's List,
# Init's primitives at their edges —; calc's main over the first inputs of the
# differential; two of the toolchain's harnesses; K's replay of the export's first 100 lines.
# Small on purpose: each is run by `ev` on the engine under test, twice. Exit code = the number
# of failed checks.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "lower_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
OUT=$(mktemp -d); trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "lower_test: $*"; fail=$((fail+1)); }
# both LABEL VERDICT ARGS…: the driver over ARGS as loaded and under --lowered; the same streams
# and status, and VERDICT (a pattern) on stdout — a run that stops before its program proves nothing
both() {
  local l=$1 v=$2; shift 2
  checks=$((checks+1))
  "$EVAL" direct v3/kernel/load.shard "$@" > "$OUT/a.out" 2> "$OUT/a.err"; local ra=$?
  "$EVAL" direct v3/kernel/load.shard --lowered "$@" > "$OUT/b.out" 2> "$OUT/b.err"; local rb=$?
  grep -q "$v" "$OUT/a.out" || { bad "$l: the program's own run has no line matching $v (exit $ra)"; head -2 "$OUT/a.out" | cut -c1-200; return; }
  [ "$ra" = "$rb" ] || { bad "$l: exit $ra as loaded, $rb lowered"; head -2 "$OUT/b.out" | cut -c1-200; return; }
  cmp -s "$OUT/a.out" "$OUT/b.out" || { bad "$l: stdout differs"; diff "$OUT/a.out" "$OUT/b.out" | head -4 | cut -c1-200; return; }
  cmp -s "$OUT/a.err" "$OUT/b.err" || { bad "$l: stderr differs"; return; }
  echo "   same $l: exit $ra, $(wc -l < "$OUT/a.out") lines"
}

for c in init_list init_prim; do
  both "pin $c" '.' --root "v3/pins/emit/$c" $INIT --run main.main "v3/pins/emit/$c/main.shard"
done
lines=(); while IFS= read -r line || [ -n "$line" ]; do lines+=("$line"); done < v3/kernel/test/fixtures/calc_inputs.txt
both "calc's main" '.' --root v3 $INIT --run examples.calc.calc_main.main v3/examples/calc/calc_main.shard -- "${lines[@]:0:4}"
for h in json_test level_test; do
  both "harness $h" "^$h: failures = 0" --root v3 --run "kernel.test.$h.main" "v3/kernel/test/$h.shard"
done
head -n 100 "$EXPORT" > "$OUT/fix100.ndjson"
both "t0 over 100 lines" '^T0: ' --root v3 --run kernel.t0.main v3/kernel/t0.shard v3/kernel/k -- -a "$OUT/fix100.ndjson"
# a lowering refused is the driver's refusal, not a run: the pin with a string pattern has none
# under the loader's reader, so the refusal path is the generator's (native_tie_test.sh's
# refused_* cases); here only that --lowered without --run changes nothing
checks=$((checks+1))
"$EVAL" direct v3/kernel/load.shard --root v3/pins/loader/basic $INIT v3/pins/loader/basic/main.shard > "$OUT/r.a" 2>&1; ra=$?
"$EVAL" direct v3/kernel/load.shard --lowered --root v3/pins/loader/basic $INIT v3/pins/loader/basic/main.shard > "$OUT/r.b" 2>&1; rb=$?
[ "$ra" = "$rb" ] && cmp -s "$OUT/r.a" "$OUT/r.b" || bad "--lowered changed a load's records"

echo "lower_test: $checks checks, $fail failed"
exit $fail
