#!/usr/bin/env bash
# calc's differential (v3/LANGUAGE.md §10 item 2, slice 6): the program half of
# examples/calc, ported to S under v3/examples/calc/, run under `ev` by
# kernel/test/calc_harness.shard against the old tree's evaluator — the
# tower's expression mode over examples/calc/calc_differential.shard — on one
# input set (fixtures/calc_inputs.txt), one line per input and case, compared
# byte for byte. The case list below and the harness's are one list in one
# order; a disagreement is a finding, never a rendering to patch around.
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
FIX=v3/kernel/test/fixtures/init_prefix_int.ndjson
INPUTS=v3/kernel/test/fixtures/calc_inputs.txt
OLD=examples/calc/calc_differential.shard
v3out=$(mktemp); oldout=$(mktemp); trap 'rm -f "$v3out" "$oldout"' EXIT

s=$(date +%s)
"$EVAL" direct v3/kernel/test/calc_harness.shard "$FIX" "$INPUTS" > "$v3out" 2>&1; rc=$?
e=$(date +%s)
[ "$rc" -eq 0 ] || { echo "calc_test: the harness failed (exit $rc)"; head -20 "$v3out"; exit 1; }
echo "calc_test: the port under ev in $((e-s)) s, $(wc -l < "$v3out") lines"

# a text line as the old tree's (list BYTE…)
lst() { printf '(list'; printf '%s' "$1" | od -An -v -tu1 | tr -s ' \n' ' ' | sed 's/ \([0-9]\)/ \1/g'; printf ')'; }
# one case: [i] NAME: the tower's rendering of EXPR
case_() { printf '[%s] %s: %s\n' "$1" "$2" "$("$EVAL" "$OLD" "$3" 2>&1)"; }
fold_() { printf '* %s: %s\n' "$1" "$("$EVAL" "$OLD" "$2" 2>&1)"; }

s=$(date +%s)
i=0; P='(list'; ALL='(list'
{
while IFS= read -r line || [ -n "$line" ]; do
  L=$(lst "$line"); Pc="$P)"
  S="(run_state (CalcState None) $Pc)"; S2="(run_state_spec (CalcState None) $Pc)"
  case_ $i lex "(lex $L)"
  case_ $i parse "(parse (lex $L))"
  case_ $i run "(run $L)"
  case_ $i spec_run "(spec_run $L)"
  case_ $i parse_expr "(parse_expr $L)"
  case_ $i skip_ws "(skip_ws $L)"
  case_ $i take_digits "(take_digits $L 0)"
  case_ $i parse_num "(parse_num $L)"
  case_ $i parse_tail "(parse_tail (Num 0) $L)"
  case_ $i strip "(strip (parse_tail (Num 0) $L))"
  case_ $i head_is_ws "(head_is_ws $L)"
  case_ $i pn_some "(pn_some (parse_num $L))"
  case_ $i pn_rest "(pn_rest (parse_num $L))"
  case_ $i numr_val "(numr_val (take_digits $L 0))"
  case_ $i numr_rest "(numr_rest (take_digits $L 0))"
  case_ $i step "(step $S $L)"
  case_ $i step_spec "(step_spec $S2 $L)"
  case_ $i next_state "(next_state $S $L)"
  case_ $i emitted "(emitted $S $L)"
  case_ $i next_state_spec "(next_state_spec $S2 $L)"
  case_ $i emitted_spec "(emitted_spec $S2 $L)"
  case_ $i show_ascii "(h_show_ascii $L)"
  case_ $i show "(h_show $L)"
  case_ $i show_nat "(h_show_nat $L)"
  case_ $i valI "(h_valI $L)"
  case_ $i codes "(h_codes $L)"
  case_ $i digits "(digits_of $L)"
  case_ $i value "(value (digits_of $L))"
  case_ $i value_go "(value_go (digits_of $L) 7)"
  case_ $i dval "(h_dval $L)"
  case_ $i code "(h_code $L)"
  case_ $i is_digit "(h_is_digit $L)"
  case_ $i is_ws "(h_is_ws $L)"
  case_ $i digit_val "(h_digit_val $L)"
  P="$P $L"; ALL="$ALL $L"; i=$((i+1))
done < "$INPUTS"
E="$ALL)"
fold_ run_state "(run_state (CalcState None) $E)"
fold_ run_trace "(run_trace (CalcState None) $E)"
fold_ drive "(drive (CalcState None) $E)"
fold_ run_state_spec "(run_state_spec (CalcState None) $E)"
fold_ run_trace_spec "(run_trace_spec (CalcState None) $E)"
fold_ drive_spec "(drive_spec (CalcState None) $E)"
fold_ run_world "(run_world (CalcWorld Nil) (CalcState None) $E)"
fold_ run_world_spec "(run_world_spec (CalcWorld Nil) (CalcState None) $E)"
fold_ apply_action_print "(apply_action (CalcWorld Nil) (Print (list 49)))"
fold_ apply_action_exit "(apply_action (CalcWorld Nil) (Exit 3))"
fold_ apply_action_nop "(apply_action (CalcWorld Nil) Nop)"
fold_ combine "(combine False (combine True (Num 1) (Num 2)) (Num 3))"
fold_ eval "(eval (Add (Num 1) (Sub (Num 2) (Num 3))))"
} > "$oldout"
e=$(date +%s)
echo "calc_test: the old tree in $((e-s)) s, $(wc -l < "$oldout") lines"
cmp "$v3out" "$oldout" || { echo "calc_test: outputs differ"; diff "$v3out" "$oldout" | head -20; exit 1; }
echo "calc_test: byte-identical over $i inputs"
