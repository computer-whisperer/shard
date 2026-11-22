#!/usr/bin/env bash
# v3/kernel/test/compile_dot_test.sh — one function through every layer of the compile system
# (v3/compile; docs/COMPILE.md §5, §8): `dot`, a sum of products over two buffers of machine
# words. Held here: its low form is a definition and computes its meaning (dot_mean.shard
# `dot_lo_means`); its intermediate code, written by hand, answers with the low form's value
# (dot_ir.shard `dot_prog_tie`); the code the pass answers for it realizes that answer
# (x86/dot_run.shard `dot_code_realizes`, by the module's theorem); a file with a function that
# realizes its answer exits as the answer says (x86/link.shard `link_exits`); and so the file —
# a start-up that pushes two buffers of three words, then the compiled code — exits with what
# `dot` computes of them (`dot_file_exits`), all on no axiom but Lean's three. The file's run
# by the model and by the processor is compile_dot_file_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_dot_test; . "$(dirname "$0")/x86_common.sh"
C=v3/compile

"${LOAD[@]}" $C/x86/dot_run.shard > "$OUT/dot.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "dot's chain did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/dot.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/dot.txt" || { bad "the summary of dot's chain is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/dot.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/dot.txt")" = 0 ] || { bad "a function of dot's chain is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/dot.txt" | head -5 | cut -c1-200; }
for f in dot_lo.dot_lo_go x86.startup.label_at; do
  checks=$((checks+1)); grep -q "^DEFINE compile.$f equations=." "$OUT/dot.txt" || bad "compile.$f is not a definition with its equations"
done
for t in lo.rd_word dot_lo.dot_lo_go_turn dot_lo.dot_lo_go_done dot_ir.dot_go_tie dot_ir.dot_prog_tie dot_mean.dot_lo_means dot_mean.dot_answer x86.seam_laws.kept_code x86.link.link_exits x86.module.module_realizes x86.dot_file.dot_compiled x86.dot_file.dot_laid x86.dot_file.dot_placed x86.dot_run.dot_code_realizes x86.dot_run.dot_file_exits; do
  checks=$((checks+1)); grep -q "^ACCEPT compile.$t " "$OUT/dot.txt" || bad "compile.$t is not accepted"
  axs=$(grep "^ACCEPT compile.$t " "$OUT/dot.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "compile.$t rests on $other"
done

echo "$NAME: $checks checks, $fail failed"
exit $fail
