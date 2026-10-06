#!/usr/bin/env bash
# v3/kernel/test/tactic_test.sh — I's opener (v3/LANGUAGE.md §8.4 slice 3.20; law §7)
# on calc's claims: the files of the old tree's proof graph whose claims need no
# arithmetic hold theorems by `(by STEP…)` now — the ground tests (reduce, then
# Eq.refl; the measured parse_tail by its WellFounded.fix_eq equation), the lexer's
# structural lemmas (induction, cases with the equation hypothesis, rw by if_pos/
# if_neg, unfold), the digit type's lemmas (a ten-way split closed by decide).
# Each file loads clean through the V3 loader with the export through
# WellFounded.fix_eq, and the count of theorems accepted is the count written.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
FIX=v3/kernel/test/fixtures/init_prefix_int.ndjson
TAIL=v3/kernel/test/fixtures/init_prefix_str_tail.ndjson
fail=0; checks=0
load() { "$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --init "$TAIL" "$@" 2>&1; }
check() {   # FILE MODULE THEOREMS…
  local f=$1 m=$2; shift 2
  local out; out=$(load "v3/examples/calc/$f")
  checks=$((checks+1))
  if ! echo "$out" | grep -q 'errors 0$'; then echo "tactic_test: $f does not load clean"; echo "$out" | grep -E 'ERROR|REFUSE' | head -4 | cut -c1-300; fail=$((fail+1)); fi
  for t in "$@"; do
    checks=$((checks+1))
    echo "$out" | grep -q "^ACCEPT examples.calc.$m.$t " || { echo "tactic_test: $f: $t not accepted"; fail=$((fail+1)); }
  done
}
check calc_spec_tests.shard calc_spec_tests t_1plus2 t_ws t_sub t_chain t_garbage t_leadop t_empty t_trailop t_twonum
check calc_reconcile_tests.shard calc_reconcile_tests r_1plus2 r_ws r_sub r_chain r_garbage r_garbage2 r_leadop r_empty r_trailop r_twonum r_tabnl
check calc_show_run.shard calc_show_run lex_nil lex_plus codes_cons append_int_cons codes_append
check calc_equiv.shard calc_equiv numr_eta lex_num lex_num_check lex_digit_head is_digit_false_lo lex_plus_head lex_minus_head lex_bad_head skip_ws_idem head_skipws_false skipws_head_nonws parse_tail_nil_test
check calc_ndigit.shard calc_ndigit codes_cons append_int_cons value_go_cons value_cons is_digit_code lex_plus
echo "tactic_test: $checks checks, $fail failed"
exit $fail
