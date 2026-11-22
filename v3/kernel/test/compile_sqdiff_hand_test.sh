#!/usr/bin/env bash
# v3/kernel/test/compile_sqdiff_hand_test.sh — hand-written code at a module's seam, and a
# compiled caller of it through every layer (v3/compile; docs/COMPILE.md §5, §8). `sqdiff` is
# the difference of two buffers' sums of squares, each a call of `dot`; in this file `dot` is
# the ten instructions an author wrote and proved on the model (v3/models/x86/buffers.shard,
# buffers_facts.shard). Held here: `dot`'s contract is stated over its own rows and holds in
# any table that has them (`code_in_part`); those rows realize `dot_ans` — the low form's value
# where the evidence about the two buffers exists, the answer a caller outside runs with
# (x86/dot_hand.shard `dot_hand_realizes`); `sqdiff`'s intermediate code, written by hand,
# answers with its low form's value, each call of `dot` by that answer and a rule of the
# machine for a call of an outside function (sqdiff_ir.shard `sqdiff_tie`, ir_laws.shard
# `iends_call_ext`); the code the pass answers for `sqdiff` realizes its answer
# (`sqh_code_realizes`, by the module's theorem); and so the file — a start-up, `dot`'s
# hand-written lines, the compiled code — exits with what `sqdiff` computes of the buffers
# the start-up pushes (`sqdiff_hand_exits`), all on no axiom but Lean's three. The file's run
# by the model and by the processor is compile_sqdiff_hand_file_test.sh's; the same caller with
# the compiled `dot` is compile_sqdiff_comp_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_sqdiff_hand_test; . "$(dirname "$0")/x86_common.sh"
C=v3/compile

"${LOAD[@]}" $C/x86/sqdiff_hand_run.shard > "$OUT/sq.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the chain did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/sq.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/sq.txt" || { bad "the chain's summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/sq.txt"; }
for f in compile.sqdiff_lo.sqdiff compile.sqdiff_lo.sqdiff_lo models.x86.part.table_part models.x86.frame.read_words models.x86.buffers.dot_rows; do
  checks=$((checks+1)); grep -q "^DEFINE $f equations=." "$OUT/sq.txt" || bad "$f is not a definition with its equations"
done
for t in models.x86.part.code_in_part models.x86.buffers_facts.dot_rows_in models.x86.buffers_facts.dot_contract models.x86.buffers_facts.both_contract compile.lo.ans_on_pos compile.lo.ans_on_inv compile.dot_lo.dot_ext_is compile.dot_lo.dot_ans_inv compile.ir_laws.iends_call_ext compile.sqdiff_lo.sqdiff_lo_means compile.sqdiff_ir.sqdiff_tie compile.sqdiff_ir.sqdiff_answer compile.x86.seam_laws.realizes_part compile.x86.dot_hand.dot_lo_agree compile.x86.dot_hand.dot_hand_realizes compile.x86.module.pair_at_one compile.x86.sqdiff_hand_file.sqh_compiled compile.x86.sqdiff_hand_file.sqh_laid compile.x86.sqdiff_hand_file.sqh_placed compile.x86.sqdiff_hand_file.sqh_dot_rows compile.x86.sqdiff_hand_run.sqh_dot_realizes compile.x86.sqdiff_hand_run.sqh_code_realizes compile.x86.sqdiff_hand_run.sqdiff_hand_exits; do
  checks=$((checks+1)); grep -q "^ACCEPT $t " "$OUT/sq.txt" || bad "$t is not accepted"
  axs=$(grep "^ACCEPT $t " "$OUT/sq.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "$t rests on $other"
done

echo "$NAME: $checks checks, $fail failed"
exit $fail
