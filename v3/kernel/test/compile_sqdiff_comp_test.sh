#!/usr/bin/env bash
# v3/kernel/test/compile_sqdiff_comp_test.sh — two compiled modules in one file, one calling
# the other across the seam (v3/compile; docs/COMPILE.md §5, §8). `sqdiff` is the difference of
# two buffers' sums of squares, each a call of `dot`; in this file `dot` is the module the pass
# compiled from `dot`'s intermediate code, and `sqdiff`'s module is compiled on its own, with
# `dot` as an outside function. Held here: whatever `dot_ans` says — the answer `sqdiff`'s code
# runs with — the run of `dot`'s intermediate code answers (dot_ir.shard `dot_ans_tie`); code
# that realizes an answer realizes every answer that says no more (`realizes_sub`), so `dot`'s
# compiled code realizes `dot_ans` in this file's table (`sqc_dot_realizes`); the code the pass
# answers for `sqdiff` then realizes its answer (`sqc_code_realizes`, by the module's theorem
# and the same tie of `sqdiff` that the file with the hand-written `dot` uses); and so the
# file — a start-up, then the two modules — exits with what `sqdiff` computes of the buffers
# the start-up pushes (`sqdiff_comp_exits`), all on no axiom but Lean's three. The file's run
# by the model and by the processor is compile_sqdiff_comp_file_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_sqdiff_comp_test; . "$(dirname "$0")/x86_common.sh"
C=v3/compile

"${LOAD[@]}" $C/x86/sqdiff_comp_run.shard > "$OUT/sq.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the chain did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/sq.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/sq.txt" || { bad "the chain's summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/sq.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/sq.txt")" = 0 ] || { bad "a function of the chain is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/sq.txt" | head -5 | cut -c1-200; }
for t in compile.dot_ir.dot_prog_tie compile.dot_ir.dot_ans_tie compile.sqdiff_ir.sqdiff_tie compile.sqdiff_ir.sqdiff_prog_tie compile.x86.seam_laws.realizes_sub compile.x86.module.module_realizes compile.x86.link.link_exits compile.x86.sqdiff_comp_file.sqc_compiled compile.x86.sqdiff_comp_file.sqc_laid compile.x86.sqdiff_comp_file.sqc_dot_placed compile.x86.sqdiff_comp_file.sqc_placed compile.x86.sqdiff_comp_run.sqc_dot_code_realizes compile.x86.sqdiff_comp_run.sqc_dot_realizes compile.x86.sqdiff_comp_run.sqc_code_realizes compile.x86.sqdiff_comp_run.sqdiff_comp_exits; do
  checks=$((checks+1)); grep -q "^ACCEPT $t " "$OUT/sq.txt" || bad "$t is not accepted"
  axs=$(grep "^ACCEPT $t " "$OUT/sq.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "$t rests on $other"
done

echo "$NAME: $checks checks, $fail failed"
exit $fail
