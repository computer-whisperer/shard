#!/usr/bin/env bash
# v3/kernel/test/compile_pass_test.sh — the compile system's first pass (v3/compile;
# docs/COMPILE.md §5): the intermediate machine (ir.shard), the statement that crosses a module's
# seam on the x86-64 model (x86/seam.shard), the pass from intermediate code to instructions
# (x86/pass.shard) and its theorem, proven once (x86/module.shard `module_realizes`): where each
# function a module calls from outside realizes its answer, each function of the compiled module
# realizes its own. Held here: the files load with nothing refused and nothing pending, the
# intermediate machine and the pass are definitions, and the theorem and the laws it is proven
# from are accepted on no axiom but Lean's three; the theorem's hypotheses are met by a module
# of five functions, and that module's file runs by the model and by the processor as the
# intermediate machine runs its functions.
# Exit code = the number of checks that failed.
NAME=compile_pass_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

# one load: calls_facts.shard, the module of calls with the theorem met, imports the theorem
"${LOAD[@]}" $C/x86/calls_facts.shard > "$OUT/pass.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the pass did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/pass.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/pass.txt" || { bad "the pass's summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/pass.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/pass.txt")" = 0 ] || { bad "a function of the compile system is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/pass.txt" | head -5 | cut -c1-200; }
for f in ir.ieval ir.ievals x86.seam.regs_of x86.pass.cexp x86.pass.cargs x86.pass.cstm x86.pass.cfn x86.pass.cmod x86.pass.compile x86.code.placedb; do
  checks=$((checks+1)); grep -q "^DEFINE compile.$f equations=." "$OUT/pass.txt" || bad "compile.$f is not a definition with its equations"
done
for t in ir.irun ir.iends_ret ir.iends_let ir.iends_if_true ir.iends_if_false ir.iends_tail x86.code.placedb_ok x86.mem.framed_back_kept x86.call.call_law x86.tail.tail_law x86.expr.cexp_all x86.stmt_store.sim_store x86.function.sim_all x86.function.cfn_ok x86.module.module_realizes x86.module.fn_realizes_ext; do
  checks=$((checks+1)); grep -q "^ACCEPT compile.$t " "$OUT/pass.txt" || bad "compile.$t is not accepted"
  axs=$(grep "^ACCEPT compile.$t " "$OUT/pass.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "compile.$t rests on $other"
done

# ---- a module with calls and a loop: the theorem met, and the file run ------------------------------
# calls.shard's five functions — word arithmetic, an `if`, calls of functions before them, a
# loop as a tail call of itself, a tail call of another function. Held here: every hypothesis
# of the module's theorem is closed by computation for it (calls_facts.shard `calls_module`);
# and for eight calls the word the intermediate machine's run returns, the model's run of the
# compiled file and the processor's run of it agree in the low byte.
for t in calls_compiled calls_placed calls_module; do
  checks=$((checks+1)); grep -q "^ACCEPT compile.x86.calls_facts.$t " "$OUT/pass.txt" || bad "compile.x86.calls_facts.$t is not accepted"
done
cases=("2 3 5" "2 9 5" "3 3 0" "3 10 0" "4 3 5" "3 12 7" "1 9 4" "0 7 8")
args=(); files=(); n=0; for c in "${cases[@]}"; do args+=("$OUT/calls$n" $c); files+=("$OUT/calls$n"); n=$((n+1)); done
"${LOAD[@]}" --run compile.x86.calls_main.main $C/x86/calls_main.shard -- "${args[@]}" > "$OUT/callsrun.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the files of the module of calls were not written (exit $rc)"; tail -3 "$OUT/callsrun.txt" | cut -c1-200; }
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
