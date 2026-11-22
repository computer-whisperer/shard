#!/usr/bin/env bash
# v3/kernel/test/compile_calls_file_test.sh — the first program's file run
# (v3/compile/x86/calls_file.shard, calls_main.shard): a start-up, then the instructions the
# compile system's pass answers for the intermediate code of six functions over words — word
# arithmetic, an `if`, calls that return, a loop, tail calls. Held here: the evaluator's status
# of the function the start-up calls, on the words the start-up hands it, the model's run of
# the file and the processor's run of it are one status, 22. That the file's run exits with it
# is a theorem, and compile_calls_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_calls_file_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

"${LOAD[@]}" --run compile.x86.calls_main.main $C/x86/calls_main.shard -- "$OUT/calls" > "$OUT/callsrun.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 22 ] && [ -s "$OUT/calls" ] || { bad "the program's file was not written, or the evaluator's status of its function is not 22 (exit $rc)"; tail -3 "$OUT/callsrun.txt" | cut -c1-200; }
m=$("${RUN[@]}" run 2000 "$OUT/calls" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 22" ] || bad "the model's run of the program's file is not exit 22: $m"
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/calls")" = "exit 22" ] || bad "the processor's run of the program's file is $(chip "$OUT/calls"), the model's exit 22"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
