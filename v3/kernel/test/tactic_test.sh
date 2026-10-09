#!/usr/bin/env bash
# v3/kernel/test/tactic_test.sh — I on calc's claims (v3/LANGUAGE.md §8.4 slices 3.20 and
# 3.21; law §7): the old tree's 100 claims are theorems by `(by STEP…)`, file for file.
# Slice 3.20's are the ground tests (reduce, then Eq.refl) and the lexer's structural
# lemmas (induction, cases with the equation hypothesis, rw, unfold, decide); slice 3.21's
# are the ones that need arithmetic (arith: a Farkas certificate over the goal's rows,
# reconstructed where the node gives none) or a chain of rewrites (simp_only), the
# well-founded inductions over `show`, and the capstone run cs = spec_run cs. The three
# measured functions (parse_tail, show, show_nat) have their descent obligations
# discharged, so nothing a claim cites is pending.
# Five loads cover the fourteen files (a leaf of the import graph loads its closure):
# each loads with no error and nothing pending, every claim of the old tree (the list in
# calc_claims.txt, shared with engine_test.sh) is accepted under its name, and the descents
# are DISCHARGE records.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
FIX=v3/kernel/test/fixtures/init_prefix_int.ndjson
RCPT=${INIT_RECEIPT:+--init-receipt $INIT_RECEIPT}   # the Init receipt (init_receipt.sh), set by v3/test.sh
TAIL=v3/kernel/test/fixtures/init_prefix_str_tail.ndjson
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0; claims=0
bad() { echo "tactic_test: $*"; fail=$((fail+1)); }
load() {   # LEAF: its closure loads clean, nothing pending
  local f=$1
  "$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --init "$TAIL" $RCPT "v3/examples/calc/$f.shard" > "$OUT/$f.txt" 2>&1
  checks=$((checks+1))
  if ! grep -q 'pending 0 .*errors 0$' "$OUT/$f.txt"; then bad "$f does not load clean"; grep -E 'ERROR|REFUSE|PENDING|^LOAD:' "$OUT/$f.txt" | head -4 | cut -c1-400; fi
}
names() {   # LEAF MODULE THEOREM…: each accepted in the leaf's load
  local f=$1 m=$2; shift 2
  for t in "$@"; do
    checks=$((checks+1)); claims=$((claims+1))
    grep -q "^ACCEPT examples.calc.$m.$t " "$OUT/$f.txt" || bad "$m: $t not accepted"
  done
}
discharged() {   # LEAF NAME
  checks=$((checks+1))
  grep -q "^DISCHARGE examples.calc.$2 proved" "$OUT/$1.txt" || bad "$2 not discharged"
}
load calc_app_world & load calc_show_run & load calc_ndigit & load calc_spec_tests & load calc_reconcile_tests & wait
checks=5
for f in calc_app_world calc_show_run calc_ndigit calc_spec_tests calc_reconcile_tests; do
  grep -q 'pending 0 .*errors 0$' "$OUT/$f.txt" || fail=$((fail+1))
done
while read -r leaf mod name; do
  case $leaf in \#*|'') continue;; esac
  names "$leaf" "$mod" "$name"
done < v3/kernel/test/calc_claims.txt
discharged calc_app_world calc_spec.parse_tail.dec_1
discharged calc_app_world calc_spec.parse_tail.dec_2
discharged calc_app_world calc_app.show_nat.dec_1
discharged calc_show_run calc_show.show.dec_1
checks=$((checks+1)); [ "$claims" = 100 ] || bad "the test names $claims claims, the old tree has 100"
echo "tactic_test: $checks checks ($claims claims), $fail failed"
exit $fail
