#!/usr/bin/env bash
# v3/kernel/test/compile_argsum_test.sh — a program of its command-line arguments through every
# layer of the compile system (v3/compile/argsum_src.shard, argsum_ir.shard, x86/argsum_*.shard;
# docs/COMPILE.md §5, §8): `argsum`, the sum of the decimal numbers given after the program's
# name. Held here: what the model takes the kernel to leave at the stack pointer carries to any
# memory that is the start's from there up (v3/models/x86/args.shard `args_at_same`); the
# model's rule for a byte read as a word and the pass's proof for a byte load are accepted; the
# intermediate code, written by hand, answers `argsum` where a memory holds the arguments
# (`argsum_prog_tie`); a file whose function realizes its answer over a range that reaches above
# its data exits as the answer says (x86/link.shard `start_exits_top`); and so the file exits
# with the low byte of `argsum` of ANY arguments (`argsum_file_exits`), on no axiom but Lean's
# three and on no parameter. The file's run by the model and by the processor is
# compile_argsum_file_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_argsum_test; . "$(dirname "$0")/x86_common.sh"
C=v3/compile

"${LOAD[@]}" $C/x86/argsum_run.shard > "$OUT/load.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the program's chain did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/load.txt" || { bad "the summary of the program's chain is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/load.txt")" = 0 ] || { bad "a function of the program's chain is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/load.txt" | head -5 | cut -c1-200; }
for f in compile.argsum_src.digits compile.argsum_src.sum_strs compile.argsum_src.argsum models.x86.linux.str_at models.x86.linux.argv_at; do
  checks=$((checks+1)); grep -q "^DEFINE $f equations=." "$OUT/load.txt" || bad "$f is not a definition with its equations"
done
for t in models.x86.args.args_at_same models.x86.laws.do_load8_b compile.x86.expr.load8_case compile.argsum_ir.argsum_tie0 compile.argsum_ir.argsum_tie1 compile.argsum_ir.argsum_tie2 compile.argsum_ir.argsum_prog_tie compile.x86.link.start_exits_top compile.x86.link.start_exits compile.x86.argsum_file.argsum_laid compile.x86.argsum_run.argsum_realizes compile.x86.argsum_run.argsum_file_exits; do
  checks=$((checks+1)); grep -q "^ACCEPT $t " "$OUT/load.txt" || bad "$t is not accepted"
  axs=$(grep "^ACCEPT $t " "$OUT/load.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "$t rests on $other"
done
checks=$((checks+1)); grep -q "^ACCEPT compile.x86.argsum_run.argsum_file_exits .* params= " "$OUT/load.txt" || bad "the file's theorem rests on a parameter"

echo "$NAME: $checks checks, $fail failed"
exit $fail
