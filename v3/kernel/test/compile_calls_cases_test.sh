#!/usr/bin/env bash
# v3/kernel/test/compile_calls_cases_test.sh — two modules run one call at a time
# (v3/compile/x86/calls_cases_main.shard), each function put through the compile system's pass
# and written as a file of six first instructions and the module. The first program's module:
# functions of two parameters — word arithmetic, an `if` with a call that returns each way, a
# loop as a tail call of itself, a tail call of another function. The module of functions with
# more than six locals (v3/compile/wide_ir.shard), whose locals past the sixth live in the
# frame: set and read, one far up the frame, a comparison and calls on them, a loop and a tail
# call out of such a function. Held here: for each call the word the intermediate machine's
# run returns, the model's run of the compiled file and the processor's run of it agree in the
# low byte. That the pass's theorem holds of the first module and what its code computes are
# compile_calls_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_calls_cases_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

# batch MODULE CASE…: the module's cases written, run by the model and by the processor
batch() {
  local mod=$1; shift; local cases=("$@")
  local args=() files=() n=0 c
  for c in "${cases[@]}"; do args+=("$OUT/$mod$n" $c); files+=("$OUT/$mod$n"); n=$((n+1)); done
  "${LOAD[@]}" --run compile.x86.calls_cases_main.main $C/x86/calls_cases_main.shard -- "$mod" "${args[@]}" > "$OUT/${mod}run.txt" 2>&1; rc=$?
  checks=$((checks+1)); [ "$rc" = 0 ] || { bad "$mod: the files of the ${#cases[@]} calls were not written (exit $rc)"; tail -3 "$OUT/${mod}run.txt" | cut -c1-200; }
  "${RUN[@]}" run 2000 "${files[@]}" 2>&1 | tail -${#cases[@]} > "$OUT/${mod}model.txt"
  n=0
  for c in "${cases[@]}"; do
    v=$(grep "^$c = " "$OUT/${mod}run.txt" | sed 's/.* = //')
    checks=$((checks+1)); [ -n "$v" ] || { bad "$mod: the intermediate machine's run of $c printed no word"; v=0; }
    want="exit $((v % 256))"
    m=$(sed -n "$((n+1))p" "$OUT/${mod}model.txt")
    checks=$((checks+1)); [ "$m" = "$want" ] || bad "$mod, function and arguments $c: the model's run is $m, the intermediate machine's word gives $want"
    if [ $onchip = 1 ]; then
      checks=$((checks+1)); [ "$(chip "$OUT/$mod$n")" = "$want" ] || bad "$mod, function and arguments $c: the processor's run is $(chip "$OUT/$mod$n"), the model's $want"
    fi
    n=$((n+1))
  done
}
batch calls "2 3 5" "2 9 5" "3 3 0" "3 10 0" "4 3 5" "3 12 7" "1 9 4" "0 7 8"
batch wide "0 3 5" "0 250 9" "1 7 8" "1 200 77" "2 3 5" "2 9 5" "3 4 0" "3 10 1" "4 6 2"
echo "$NAME: $checks checks, $fail failed"
exit $fail
