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
# each loads with no error and nothing pending, every claim of the old tree is accepted
# under its name, and the descents are DISCHARGE records.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
FIX=v3/kernel/test/fixtures/init_prefix_int.ndjson
TAIL=v3/kernel/test/fixtures/init_prefix_str_tail.ndjson
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0; claims=0
bad() { echo "tactic_test: $*"; fail=$((fail+1)); }
load() {   # LEAF: its closure loads clean, nothing pending
  local f=$1
  "$EVAL" direct v3/kernel/load.shard --root v3 --init "$FIX" --init "$TAIL" "v3/examples/calc/$f.shard" > "$OUT/$f.txt" 2>&1
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
names calc_app_world calc_spec len_skipws_le len_takedigits_le len_takedigits_lt skipcons_len_le pn_rest_lt ptd
names calc_app_world calc_equiv lex_num le32_le48_false le32_not_43 le32_not_45 le32_not_digit lex_ws_none ws_lex lex_num_check numr_eta lex_digit_head skip_ws_idem is_digit_false_lo digit_lo ge48_not_43 ge48_not_45 digit_not_43 digit_not_45 lex_plus_head lex_minus_head head_skipws_false skipws_head_nonws parse_tail_nil_test lex_bad_head lex_skipnum loop_decrease loop_eq CORE run_eq_spec
names calc_app_world calc_app_spec step_eq_spec
names calc_app_world calc_app_trace next_state_eq_spec emitted_eq_spec run_state_eq_spec run_trace_eq_spec drive_eq_spec
names calc_app_world calc_app_world run_eq_spec_world
names calc_show_run calc_proof digit_val_of_digit digit_ge_lo digit_le_hi is_digit_of_digit lex_xy run_xy_adds
names calc_show_run calc_show lt10f_pos valI_go_snoc valI_snoc show_correct
names calc_show_run calc_show_run codes_cons append_int_cons codes_append mul_comm10 digit_val_id lex_go_digit lt10_le9 show_lt show_ge lex_show_run add_zero lex_go_digit_none lex_show_run_none lex_plus lex_nil lex_two run_show_adds
names calc_ndigit calc_ndigit codes_cons append_int_cons value_go_cons digit_val_code is_digit_code lex_go_digit lex_go_digit0 lex_digit_run value_cons lex_plus lex_ndigit run_ndigit_adds
names calc_spec_tests calc_spec_tests t_1plus2 t_ws t_sub t_chain t_garbage t_leadop t_empty t_trailop t_twonum
names calc_reconcile_tests calc_reconcile_tests r_1plus2 r_ws r_sub r_chain r_garbage r_garbage2 r_leadop r_empty r_trailop r_twonum r_tabnl
discharged calc_app_world calc_spec.parse_tail.dec_1
discharged calc_app_world calc_spec.parse_tail.dec_2
discharged calc_app_world calc_app.show_nat.dec_1
discharged calc_show_run calc_show.show.dec_1
checks=$((checks+1)); [ "$claims" = 100 ] || bad "the test names $claims claims, the old tree has 100"
echo "tactic_test: $checks checks ($claims claims), $fail failed"
exit $fail
