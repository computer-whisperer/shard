#!/usr/bin/env bash
# v3/kernel/test/compile_calls_test.sh — the first program through every layer of the compile
# system (v3/compile; docs/COMPILE.md §5, §8): six functions over machine words with calls, a
# test and a loop. Held here: the functions are definitions (calls_lo.shard); a run of the
# intermediate machine answers the same given more fuel (ir_laws.shard `wrun_more`), whence the
# rule of a call that returns (`iends_call`); the intermediate code of each function, written
# by hand, answers with the function's value, a call by its callee's tie (calls_ir.shard
# `calls_prog_tie`); the code the pass answers for the module realizes each function's answer
# (x86/calls_run.shard `calls_realizes`, by the module's theorem); a file with a function that
# realizes its answer exits as the answer says (x86/link.shard `link_exits`); and so the file —
# a start-up, then the compiled module — exits with what the function the start-up calls
# computes: 22 (`calls_file_exits`), all on no axiom but Lean's three. The file's run by the
# model and by the processor is compile_calls_file_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_calls_test; . "$(dirname "$0")/x86_common.sh"
C=v3/compile

"${LOAD[@]}" $C/x86/calls_run.shard > "$OUT/calls.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the program's chain did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/calls.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/calls.txt" || { bad "the summary of the program's chain is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/calls.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/calls.txt")" = 0 ] || { bad "a function of the program's chain is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/calls.txt" | head -5 | cut -c1-200; }
for f in calls_lo.calls_f2 calls_lo.calls_f3 calls_lo.calls_f5 ir_laws.fadd; do
  checks=$((checks+1)); grep -q "^DEFINE compile.$f equations=." "$OUT/calls.txt" || bad "compile.$f is not a definition with its equations"
done
for t in calls_lo.calls_f3_turn calls_lo.calls_f3_done calls_lo.sample_f5 ir_laws.wrun_more ir_laws.iends_call calls_ir.calls_tie2 calls_ir.calls_tie3 calls_ir.calls_tie5 calls_ir.calls_prog_tie x86.link.link_exits x86.module.module_realizes x86.calls_file.calls_compiled x86.calls_file.calls_laid x86.calls_file.calls_placed x86.calls_run.calls_realizes x86.calls_run.calls_code_realizes x86.calls_run.calls_file_exits; do
  checks=$((checks+1)); grep -q "^ACCEPT compile.$t " "$OUT/calls.txt" || bad "compile.$t is not accepted"
  axs=$(grep "^ACCEPT compile.$t " "$OUT/calls.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "compile.$t rests on $other"
done

echo "$NAME: $checks checks, $fail failed"
exit $fail
