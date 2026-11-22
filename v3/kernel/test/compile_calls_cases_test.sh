#!/usr/bin/env bash
# v3/kernel/test/compile_calls_cases_test.sh — the first program's module run one call at a
# time (v3/compile/x86/calls_cases_main.shard): its functions of two parameters — word
# arithmetic, an `if` with a call that returns each way, a loop as a tail call of itself, a
# tail call of another function — put through the compile system's pass and written as files
# of six first instructions and the module. Held here: for eight calls the word the
# intermediate machine's run returns, the model's run of the compiled file and the
# processor's run of it agree in the low byte. That the pass's theorem holds of this module
# and what its code computes are compile_calls_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_calls_cases_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

cases=("2 3 5" "2 9 5" "3 3 0" "3 10 0" "4 3 5" "3 12 7" "1 9 4" "0 7 8")
args=(); files=(); n=0; for c in "${cases[@]}"; do args+=("$OUT/calls$n" $c); files+=("$OUT/calls$n"); n=$((n+1)); done
"${LOAD[@]}" --run compile.x86.calls_cases_main.main $C/x86/calls_cases_main.shard -- "${args[@]}" > "$OUT/callsrun.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the files of the eight calls were not written (exit $rc)"; tail -3 "$OUT/callsrun.txt" | cut -c1-200; }
"${RUN[@]}" run 2000 "${files[@]}" 2>&1 | tail -${#cases[@]} > "$OUT/callsmodel.txt"
n=0
for c in "${cases[@]}"; do
  v=$(grep "^$c = " "$OUT/callsrun.txt" | sed 's/.* = //')
  checks=$((checks+1)); [ -n "$v" ] || { bad "the intermediate machine's run of $c printed no word"; v=0; }
  want="exit $((v % 256))"
  m=$(sed -n "$((n+1))p" "$OUT/callsmodel.txt")
  checks=$((checks+1)); [ "$m" = "$want" ] || bad "function and arguments $c: the model's run is $m, the intermediate machine's word gives $want"
  if [ $onchip = 1 ]; then
    checks=$((checks+1)); [ "$(chip "$OUT/calls$n")" = "$want" ] || bad "function and arguments $c: the processor's run is $(chip "$OUT/calls$n"), the model's $want"
  fi
  n=$((n+1))
done
echo "$NAME: $checks checks, $fail failed"
exit $fail
