#!/usr/bin/env bash
# v3/kernel/test/compile_total_file_test.sh — the file with a function that writes memory run
# (v3/compile/x86/total_file.shard, total_main.shard): a start-up, the ten instructions of `dot`
# as an author wrote them, then three modules the compile system's pass compiled each on its
# own: `addto`'s, whose loop stores into the first buffer; `both`'s, which calls it and `dot`;
# `total`'s, which calls `both` and `dot`.
# Held here: the evaluator's status of `total` — the function over two lists — on the buffers
# the start-up pushes, the model's run of the file and the processor's run of it are one status,
# 75; it would be 139 were the first buffer not written. That the file's run exits with it is a
# theorem, and compile_writer_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_total_file_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

"${LOAD[@]}" --run compile.x86.total_main.main $C/x86/total_main.shard -- "$OUT/total" > "$OUT/totalrun.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 75 ] && [ -s "$OUT/total" ] || { bad "the file was not written, or the evaluator's status of total is not 75 (exit $rc)"; tail -3 "$OUT/totalrun.txt" | cut -c1-200; }
m=$("${RUN[@]}" run 4000 "$OUT/total" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 75" ] || bad "the model's run of the file is not exit 75: $m"
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/total")" = "exit 75" ] || bad "the processor's run of the file is $(chip "$OUT/total"), the model's exit 75"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
