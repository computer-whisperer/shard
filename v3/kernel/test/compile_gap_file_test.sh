#!/usr/bin/env bash
# v3/kernel/test/compile_gap_file_test.sh — the file three functions deep run
# (v3/compile/x86/gap_file.shard, gap_main.shard): a start-up, the ten instructions of `dot` as
# an author wrote them, then two modules the compile system's pass compiled each on its own:
# `sqdiff`'s, which calls them, and `gap`'s, which calls `sqdiff`.
# Held here: the evaluator's status of `gap` — the function over two lists — on the buffers the
# start-up pushes, the model's run of the file and the processor's run of it are one status,
# 54. That the file's run exits with it is a theorem, and compile_modules_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_gap_file_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

"${LOAD[@]}" --run compile.x86.gap_main.main $C/x86/gap_main.shard -- "$OUT/gap" > "$OUT/gaprun.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 54 ] && [ -s "$OUT/gap" ] || { bad "the file was not written, or the evaluator's status of gap is not 54 (exit $rc)"; tail -3 "$OUT/gaprun.txt" | cut -c1-200; }
m=$("${RUN[@]}" run 4000 "$OUT/gap" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 54" ] || bad "the model's run of the file is not exit 54: $m"
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/gap")" = "exit 54" ] || bad "the processor's run of the file is $(chip "$OUT/gap"), the model's exit 54"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
