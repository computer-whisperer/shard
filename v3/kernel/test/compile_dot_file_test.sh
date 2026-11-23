#!/usr/bin/env bash
# v3/kernel/test/compile_dot_file_test.sh — the file of the compiled `dot` run
# (v3/compile/x86/dot_file.shard, dot_main.shard): a start-up that pushes two buffers of three
# words, then the instructions the compile system's pass answers for `dot`'s intermediate code.
# Held here: the evaluator's status of `dot` — the function over two lists — on those buffers,
# the model's run of the file and the processor's run of it are one status, 68. That the
# file's run exits with it is a theorem, and compile_modules_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_dot_file_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

"${LOAD[@]}" --run compile.x86.dot_main.main $C/x86/dot_main.shard -- "$OUT/dot" > "$OUT/dotrun.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 68 ] && [ -s "$OUT/dot" ] || { bad "dot's file was not written, or the evaluator's status of dot is not 68 (exit $rc)"; tail -3 "$OUT/dotrun.txt" | cut -c1-200; }
m=$("${RUN[@]}" run 2000 "$OUT/dot" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 68" ] || bad "the model's run of dot's file is not exit 68: $m"
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/dot")" = "exit 68" ] || bad "the processor's run of dot's file is $(chip "$OUT/dot"), the model's exit 68"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
