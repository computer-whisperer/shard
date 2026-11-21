#!/usr/bin/env bash
# v3/kernel/test/x86_trial_test.sh — the trial of low-level code as shard on the x86-64 model
# (v3/models/x86/buffers.shard): its functions definitions; the laws of the model (laws.shard)
# and of a function's stack frame (frame.shard), the contracts of the three functions' code
# (buffers_facts.shard), a listing's code in place in its loaded file (loaded.shard) and the
# theorem that the trial's file exits with 19 (buffers_run.shard) accepted on Lean's axioms; and
# its file's run by the model and by the processor exiting with what the evaluator computes of
# the functions. Nine of the contracts' lemmas are blocks a tactic library derived, replayed by
# the load; that the library still derives them is x86_derive_test.sh's.
# Exit code = the number of checks that failed.
NAME=x86_trial_test; . "$(dirname "$0")/x86_common.sh"
no_chip

# ---- low-level code as shard: the trial (buffers.shard, laws.shard, frame.shard, -------------------
# ---- buffers_facts.shard, loaded.shard, buffers_run.shard) ------------------------------------------
# Three functions over buffers of machine words — each taking, beside its buffers and their count,
# the evidence that each buffer holds that many words, and an access the evidence that its index
# is in range — and a listing for them compiled by hand. Held here: every function is a definition;
# the laws of the model and the contracts of the code of `dot`, of `addto` and of `both` — which
# calls the two and keeps a frame on the stack, by the laws of a frame — are accepted; so is the
# fact that the code of any listing the assembler answers for is in place once its file is loaded
# (`listing_loaded`), and through it and `both`'s contract that the trial's file exits with 19
# (`buffers_exits`); all on
# no axiom but Lean's three (a stored word read back cites Init's lemmas on division, which rest
# on `Classical.choice`); the evaluator's run of the functions, the model's run of the listing's
# file and the processor's give that one status, 19.
"${LOAD[@]}" $M/buffers_run.shard > "$OUT/buf.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the trial did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/buf.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/buf.txt" || { bad "the trial's summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/buf.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/buf.txt")" = 0 ] || { bad "a function of the trial is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/buf.txt" | head -5 | cut -c1-200; }
for f in words.word_at words.put_at buffers.dot_go buffers.dot buffers.addto_next buffers.addto_go buffers.addto buffers.both buffers.buffers_table laws.steps loaded.layout; do
  checks=$((checks+1)); grep -q "^DEFINE models.x86.$f equations=." "$OUT/buf.txt" || bad "models.x86.$f is not a definition with its equations"
done
for t in laws.reaches_step laws.reaches_trans laws.step_at laws.do_alu_rr laws.do_cmp_rr laws.do_alu_ri laws.do_mov_rr laws.do_jcc_not laws.do_jcc_taken laws.do_jmp laws.do_load_sib laws.do_imul_sib laws.do_add_m_sib laws.do_push laws.do_pop laws.do_call laws.do_ret laws.mem_put64_other laws.le64_put64_word laws.le64_same laws.fetch_same laws.code_in_same frame.holds_upd frame.holds_same buffers_facts.dot_loop buffers_facts.dot_contract buffers_facts.dot_contract_same buffers_facts.add_loop buffers_facts.addto_contract word.w64_val_sub word.w64_val_add laws.same_outside2_around laws.ret_addr_pushed laws.code_in_after laws.code_in_of buffers.addto_length buffers.addto_room frame.holds_head frame.holds_tail frame.framed_enter frame.framed_push_ok frame.framed_push frame.framed_top_ok frame.framed_top frame.framed_up frame.framed_pop frame.framed_back_at frame.framed_back frame.framed_back_same frame.framed_returned buffers_facts.both_enter buffers_facts.both_mid buffers_facts.both_leave buffers_facts.both_in buffers_facts.both_first buffers_facts.both_second buffers_facts.both_out buffers_facts.both_contract laws.do_mov_ri32 laws.mapped_part laws.mapped_words_of laws.same_outside2_hull laws.run_of_steps laws.run_exit frame.holds_cons loaded.put_bytes_at loaded.elf_load_exec loaded.fetch_loaded loaded.layout_in loaded.listing_loaded loaded.run_file_exec buffers_run.buffers_assembled buffers_run.buffers_laid buffers_run.buffers_code_first buffers_run.buffers_startup buffers_run.su_code buffers_run.buffers_end buffers_run.buffers_exits buffers_facts.dot_turn buffers_facts.dot_leave buffers_facts.dot_enter buffers_facts.add_turn buffers_facts.add_leave buffers_facts.add_enter; do
  checks=$((checks+1)); grep -q "^ACCEPT models.x86.$t " "$OUT/buf.txt" || bad "models.x86.$t is not accepted"
  axs=$(grep "^ACCEPT models.x86.$t " "$OUT/buf.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "models.x86.$t rests on $other"
done
"${LOAD[@]}" --run models.x86.buffers.main $M/buffers.shard -- "$OUT/buffers" > "$OUT/bufrun.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 19 ] && [ -s "$OUT/buffers" ] || { bad "the trial's file was not written, or the evaluator's status of the functions is not 19 (exit $rc)"; tail -3 "$OUT/bufrun.txt" | cut -c1-200; }
m=$("${RUN[@]}" run 1000 "$OUT/buffers" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 19" ] || bad "the model's run of the trial's file is not exit 19: $m"
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/buffers")" = "exit 19" ] || bad "the processor's run of the trial's file is $(chip "$OUT/buffers"), the model's exit 19"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
