#!/usr/bin/env bash
# v3/kernel/test/compile_sqdiff_hand_file_test.sh — the file of `sqdiff` with the hand-written
# `dot` run (v3/compile/x86/sqdiff_hand_file.shard, sqdiff_hand_main.shard):
# a start-up, the ten instructions of `dot` as an author wrote them, then the instructions
# the compile system's pass answers for `sqdiff`'s intermediate code, which calls them.
# Held here: the evaluator's status of `sqdiff` — the function over two lists — on the buffers
# the start-up pushes, the model's run of the file and the processor's run of it are one
# status, 27. That the file's run exits with it is a theorem, and
# compile_modules_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_sqdiff_hand_file_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

"${LOAD[@]}" --run compile.x86.sqdiff_hand_main.main $C/x86/sqdiff_hand_main.shard -- "$OUT/sq" > "$OUT/sqrun.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 27 ] && [ -s "$OUT/sq" ] || { bad "the file was not written, or the evaluator's status of sqdiff is not 27 (exit $rc)"; tail -3 "$OUT/sqrun.txt" | cut -c1-200; }
m=$("${RUN[@]}" run 2000 "$OUT/sq" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 27" ] || bad "the model's run of the file is not exit 27: $m"
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/sq")" = "exit 27" ] || bad "the processor's run of the file is $(chip "$OUT/sq"), the model's exit 27"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
