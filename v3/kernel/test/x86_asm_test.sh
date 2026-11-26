#!/usr/bin/env bash
# v3/kernel/test/x86_asm_test.sh — the x86-64 model's assembler (v3/models/x86/asm.shard) and the
# demonstration (demo.shard). The load: the assembler's functions definitions; its fact and the
# lemmas that open it accepted, a listing with a missing label and one with an immediate too
# wide refused and three listings assembled, all on no axiom but `propext`. The assembler's
# bytes for three listings are the bytes assembled by hand (test/*.hex, which x86_test.sh
# runs). The demonstration's file is written, and run by the model and, on an x86-64 Linux
# machine, by the processor; the theorem of its exit status is x86_trial_test.sh's.
# Exit code = the number of checks that failed.
NAME=x86_asm_test; . "$(dirname "$0")/x86_common.sh"
no_chip

# ---- the load ---------------------------------------------------------------------------------
"${LOAD[@]}" $M/test/listings.shard $M/demo.shard > "$OUT/load.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the assembler and the demonstration did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/load.txt" || { bad "the summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/load.txt")" = 0 ] || { bad "a function is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/load.txt" | head -5 | cut -c1-200; }
for f in asm.encode asm.resolve asm.decodes asm.assemble; do
  checks=$((checks+1)); grep -q "^DEFINE models.x86.$f equations=." "$OUT/load.txt" || bad "models.x86.$f is not a definition with its equations"
done
# the assembler's fact, the lemmas that open it, and three listings decided by evaluation: on no
# axiom but `propext`
for t in asm.checked_some asm.assemble_decodes asm.same_decode_true asm.decodes_cons test.listings.refuses_missing_label test.listings.refuses_wide_immediate test.listings.assembles_three; do
  checks=$((checks+1)); grep -q "^ACCEPT models.x86.$t " "$OUT/load.txt" || bad "models.x86.$t is not accepted"
  axs=$(grep "^ACCEPT models.x86.$t " "$OUT/load.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  checks=$((checks+1)); case "$axs" in ""|propext) ;; *) bad "models.x86.$t rests on $axs" ;; esac
done

# ---- the assembler against the hand ------------------------------------------------------------
# Three listings of test/listings.shard are the programs of stack.hex, high.hex and add.hex (at
# KK = F8): the assembler's bytes for each are the bytes assembled by hand.
for n in stack high; do hex2bin $M/test/$n.hex -- > "$OUT/$n.--.bin"; done; hex2bin $M/test/add.hex F8 > "$OUT/add.F8.bin"
"${LOAD[@]}" --run models.x86.test.listings.main $M/test/listings.shard -- stack "$OUT/stack.asm" high "$OUT/high.asm" add "$OUT/add.asm" > "$OUT/asm.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the listings were not assembled (exit $rc)"; head -3 "$OUT/asm.txt" | cut -c1-200; }
for n in stack high add; do
  k=--; [ $n = add ] && k=F8
  checks=$((checks+1)); cmp -s "$OUT/$n.asm" "$OUT/$n.$k.bin" || bad "$n: the assembler's bytes are not the bytes assembled by hand"
done

# ---- the demonstration (demo.shard) ---------------------------------------------------------------
# Three shard functions over machine words, a listing compiled by hand, the listing's file: the
# file is written, and the model's run of it and the processor's exit with 20, the low byte of
# what the function computes. That it does so from every start is x86_trial_test.sh's
# (demo_run.shard `demo_exits`).
"${LOAD[@]}" --run models.x86.demo.main $M/demo.shard -- "$OUT/demo" > "$OUT/demo.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ -s "$OUT/demo" ] || { bad "the demonstration's file was not written (exit $rc)"; head -3 "$OUT/demo.txt" | cut -c1-200; }
m=$("${RUN[@]}" run 100 "$OUT/demo" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 20" ] || bad "the model's run of the demonstration is not exit 20: $m"
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/demo")" = "exit 20" ] || bad "the processor's run of the demonstration is $(chip "$OUT/demo"), the model's exit 20"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
