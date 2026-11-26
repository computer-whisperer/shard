#!/usr/bin/env bash
# v3/kernel/test/compile_argsum_file_test.sh — the file of a program of its command-line
# arguments, run (v3/compile/x86/argsum_file.shard, argsum_main.shard): a start-up that hands
# over the address the kernel left the stack pointer at, then the instructions the pass answers
# for `argsum`'s intermediate code — a byte read at a time, two loops, a call. Held here, for
# seven lists of arguments — none but the name, numbers, a sum past a byte, a number past a
# word, a byte that is no digit, an argument of no bytes —: the evaluator's status of `argsum`
# of the list, the model's run of the file started with those arguments
# (v3/models/x86/run_main.shard `args`) and the processor's run of it are one status. That the
# file's run exits with it for every list is a theorem, and compile_argsum_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_argsum_file_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

# each case: the status stated here (or - for none stated), then the arguments, the first the
# name the program is run by; `E` stands for an argument of no bytes. The writer writes the file
# and prints the evaluator's status of `argsum` for each list, the lists separated by `/`.
cases=("0 argsum" "42 argsum 12 30" "55 x 1 2 3 4 5 6 7 8 9 10" "0 prog 255 1" "6 p 99999999999999999999 7" "107 name 4x2 9" "5 e E 5")
all=()
for cs in "${cases[@]}"; do
  [ ${#all[@]} = 0 ] || all+=("/")
  set -- $cs; shift
  for a in "$@"; do [ "$a" = E ] && a=""; all+=("$a"); done
done
"${LOAD[@]}" --run compile.x86.argsum_main.main $C/x86/argsum_main.shard -- "$OUT/argsum" "${all[@]}" > "$OUT/write.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ -s "$OUT/argsum" ] || { bad "the program's file was not written (exit $rc)"; tail -3 "$OUT/write.txt" | cut -c1-200; }
tail -n ${#cases[@]} "$OUT/write.txt" > "$OUT/ev.txt"
i=0
for cs in "${cases[@]}"; do
  i=$((i+1)); set -- $cs; want=$1; shift
  args=(); for a in "$@"; do [ "$a" = E ] && a=""; args+=("$a"); done
  ev=$(sed -n "${i}p" "$OUT/ev.txt")
  checks=$((checks+1)); [ "$ev" = "$want" ] || bad "the evaluator's status of argsum on '$*' is '$ev', not $want"
  m=$("${RUN[@]}" args 100000 "$OUT/argsum" "${args[@]}" 2>&1 | tail -1)
  checks=$((checks+1)); [ "$m" = "exit $ev" ] || bad "on '$*' the model's run of the file is '$m', the evaluator's status $ev"
  if [ $onchip = 1 ]; then
    c=$(chip_args "$OUT/argsum" "${args[@]}")
    checks=$((checks+1)); [ "$c" = "exit $ev" ] || bad "on '$*' the processor's run of the file is '$c', the evaluator's status $ev"
  fi
done

echo "$NAME: $checks checks, $fail failed"
exit $fail
