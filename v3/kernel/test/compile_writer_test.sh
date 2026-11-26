#!/usr/bin/env bash
# v3/kernel/test/compile_writer_test.sh — a function that writes memory, as a module behind a
# view, with two callers above it (v3/compile; docs/COMPILE.md §5, §8): `addto`, which adds the
# second buffer's words into the first; `both`, which calls `addto` and then `dot` on what
# `addto` wrote; `total`, which calls `both` and then `dot` on the buffer `both` did not write.
# A writer's view states its low form's signature — the value is a word and the memory after,
# and the evidence asked is that the first buffer may be read and written, the second read, and
# that the two lie apart — and three laws: what the written buffer holds after, that every byte
# outside it is as it was, and that the code's run answers the low form. One load, with the link
# (`load.shard --link`, docs/LANGUAGE.md "The link"), of the file's theorem and of the four
# directories. Held here:
#   - every law of every view is discharged by its implementation, at the view's own statement;
#   - what a module's proofs rest on is the views of the functions it calls and nothing below
#     them: `total`'s on `both`'s and `dot`'s — nothing of `addto` —, `both`'s on `addto`'s and
#     `dot`'s; K's own account (`params=`);
#   - what `total` knows of the second buffer after the call it has from `both`'s law of what it
#     leaves alone;
#   - the file's theorem — the file exits with what `total` computes of the two buffers the
#     start-up pushes, a value that is another if the write is not made — is accepted by K in
#     the linked environment, where each parameter is its implementation, on no parameter and no
#     axiom but Lean's three.
# The file's run by the model and by the processor is compile_total_file_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_writer_test; . "$(dirname "$0")/x86_common.sh"
C=v3/compile

"${LOAD[@]}" --link $C/x86/total_run.shard $C/total $C/both $C/addto $C/dot > "$OUT/m.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the modules did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/m.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/m.txt" || { bad "the summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/m.txt"; }
checks=$((checks+1)); [ "$(grep -c 'RUNNABLE fn ' "$OUT/m.txt")" = 0 ] || { bad "a function of the modules is a program without a definition"; grep 'RUNNABLE fn ' "$OUT/m.txt" | head -5 | cut -c1-200; }

# the definitions: the meanings, the write, the code as data
for f in models.x86.buffers.addto models.x86.buffers.both compile.total_src.total compile.lo.wr64 compile.addto_obj.addto_go_code compile.addto_obj.addto_code compile.both_obj.both_code compile.total_obj.total_code; do
  checks=$((checks+1)); grep -q "^DEFINE $f equations=." "$OUT/m.txt" || bad "$f is not a definition with its equations"
done
# what rests on no view: the laws used, the file's computed facts, the sample — accepted on no
# axiom but Lean's three
for t in compile.ir_laws.iends_store compile.upd.upd_ext_is compile.upd.upd_ans_inv compile.upd.upd_ans_tie compile.upd.rd_same compile.upd.kept_apart compile.two.reads_on models.x86.frame.holds_read_on compile.x86.module.module_realizes compile.x86.module.pair_at_two compile.x86.link.link_exits compile.total_src.sample_total compile.sample.sample_ew compile.total.sample_ea compile.stretch.start_buf compile.stretch.start_apart compile.x86.total_file.tt_ad_compiled compile.x86.total_file.tt_bo_compiled compile.x86.total_file.tt_compiled compile.x86.total_file.tt_laid compile.x86.total_file.tt_ad_placed compile.x86.total_file.tt_bo_placed compile.x86.total_file.tt_placed compile.x86.total_file.tt_dot_rows; do
  checks=$((checks+1)); grep -q "^ACCEPT $t " "$OUT/m.txt" || bad "$t is not accepted"
  axs=$(grep "^ACCEPT $t " "$OUT/m.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "$t rests on $other"
done
# the proofs behind addto's view rest on no other module: the loop's cases, its two facts of the
# memory, its tie
for t in addto_lo_go_turn addto_lo_go_done addto_lo_go_means addto_lo_go_keeps addto_go_tie addto_tie; do
  checks=$((checks+1)); grep -q "^IMPL ACCEPT compile.addto.$t .* params= " "$OUT/m.txt" || bad "compile.addto.$t is not accepted in addto's implementation on no parameter"
  axs=$(grep "^IMPL ACCEPT compile.addto.$t " "$OUT/m.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "compile.addto.$t rests on $other"
done

# each view's parameters are parameters to its consumers, and each is met by the implementation
for p in dot.dot_lo:fn addto.addto_lo:fn both.both_lo:fn total.total_lo:fn dot.dot_lo_means:requirement dot.dot_prog_tie:requirement addto.addto_lo_means:requirement addto.addto_lo_keeps:requirement addto.addto_prog_tie:requirement both.both_lo_means:requirement both.both_lo_keeps:requirement both.both_tie:requirement total.total_lo_means:requirement total.total_lo_keeps:requirement total.total_tie:requirement; do
  n=${p%%:*}; k=${p##*:}
  checks=$((checks+1)); grep -q "^PARAM compile.$n $k\$" "$OUT/m.txt" || bad "compile.$n is not a parameter ($k) of its view"
  want=proved; [ "$k" = fn ] && want=defined
  checks=$((checks+1)); grep -q "^DISCHARGE compile.$n $want\$" "$OUT/m.txt" || bad "compile.$n is not discharged ($want) by its implementation"
done
checks=$((checks+1)); [ "$(grep -c '^IMPL-ERROR' "$OUT/m.txt")" = 0 ] || { bad "an implementation fails its view"; grep '^IMPL-ERROR' "$OUT/m.txt" | head -3 | cut -c1-200; }

# what a declaration rests on, of the views: the parameters K lists for it
params() { grep -E "^(IMPL )?ACCEPT compile.$1 " "$OUT/m.txt" | head -1 | sed -E 's/.* params=([^ ]*).*/\1/'; }
# $1 rests on parameters, all of them of the modules $2 and $3
rests_on() {
  local ps; ps=$(params "$1")
  checks=$((checks+1)); [ -n "$ps" ] || { bad "compile.$1 is not accepted on parameters of a view"; return; }
  checks=$((checks+1)); [ -z "$(echo "$ps" | tr ',' '\n' | grep -v -e "^compile\.$2\." -e "^compile\.$3\.")" ] || bad "compile.$1 rests on $ps: more than the views of $2 and $3"
}
rests_on total.total_lo both dot;    rests_on total.total_lo_means both dot;  rests_on total.total_lo_keeps both dot;  rests_on total.total_tie both dot
rests_on both.both_lo addto dot;     rests_on both.both_lo_means addto dot;   rests_on both.both_lo_keeps addto dot;   rests_on both.both_tie addto dot
# what is kept across the call is known from the callee's law of what it leaves alone
for t in total.total_lo_means:both.both_lo_keeps total.total_lo_keeps:both.both_lo_keeps both.both_lo_keeps:addto.addto_lo_keeps; do
  checks=$((checks+1)); echo ",$(params "${t%%:*}")," | grep -q ",compile\.${t##*:}," || bad "compile.${t%%:*} does not rest on compile.${t##*:}: $(params "${t%%:*}")"
done
# the low form's body stays behind its view: the loop is the implementation's own
checks=$((checks+1)); grep -q '^IMPL DEFINE compile.addto.addto_lo_go equations=.' "$OUT/m.txt" || bad "compile.addto.addto_lo_go is not a definition of addto's implementation"
checks=$((checks+1)); [ "$(grep -c '^DEFINE compile.addto.addto_lo_go ' "$OUT/m.txt")" = 0 ] || bad "compile.addto.addto_lo_go is defined in the main line: a consumer holds the low form's body"

# the file's theorem: conditional on parameters against the views, closed in the linked environment
t=total_run.total_file_exits
checks=$((checks+1)); [ -n "$(params x86.$t)" ] || bad "compile.x86.$t is not accepted on the views' parameters"
l=$(grep "^LINK compile.x86.$t " "$OUT/m.txt")
checks=$((checks+1)); [ -n "$l" ] || bad "compile.x86.$t has no record of the link"
checks=$((checks+1)); [ -n "$l" ] && [ -z "$(echo "$l" | sed -E 's/.* params=//')" ] || bad "compile.x86.$t rests on parameters in the linked environment: $l"
other=$(echo "$l" | sed -E 's/.* axioms=([^ ]*) .*/\1/' | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
checks=$((checks+1)); [ -z "${other// /}" ] || bad "compile.x86.$t rests on $other in the linked environment"
checks=$((checks+1)); [ "$(grep -c '^LINK-ERROR' "$OUT/m.txt")" = 0 ] || { bad "a declaration did not enter the linked environment"; grep '^LINK-ERROR' "$OUT/m.txt" | head -3 | cut -c1-240; }
# every declaration that rests on a parameter entered it, and none rests on a parameter there
on=$(grep -c -E '^ACCEPT .* params=[^ ]' "$OUT/m.txt")
checks=$((checks+1)); [ "$on" -gt 0 ] && [ "$(grep -c '^LINK ' "$OUT/m.txt")" = "$on" ] || bad "$on declarations rest on a parameter, $(grep -c '^LINK ' "$OUT/m.txt") were linked"
checks=$((checks+1)); [ "$(grep '^LINK ' "$OUT/m.txt" | grep -v -c ' params=$')" = 0 ] || { bad "a linked declaration rests on a parameter"; grep '^LINK ' "$OUT/m.txt" | grep -v ' params=$' | head -3 | cut -c1-240; }

echo "$NAME: $checks checks, $fail failed"
exit $fail
