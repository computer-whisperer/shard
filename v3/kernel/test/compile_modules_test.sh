#!/usr/bin/env bash
# v3/kernel/test/compile_modules_test.sh — functions as modules behind views, through every layer
# of the compile system and three functions deep (v3/compile; docs/COMPILE.md §5, §8): `dot`, a
# sum of products over two buffers; `sqdiff`, which calls `dot`; `gap`, which calls `sqdiff`.
# Each is a directory whose view states its low form's signature and its two facts as laws, and
# whose implementation holds the low form's body and the proofs. One load, with the link
# (`load.shard --link`, docs/LANGUAGE.md "The link"), of the four files' theorems — `dot` compiled
# alone; `sqdiff` with the hand-written `dot` and with the compiled one; `gap` over the compiled
# `sqdiff` over the hand-written `dot` — and of the three directories. Held here:
#   - every law of every view is discharged by its implementation, at the view's own statement;
#   - what a module's proofs rest on is the view of the function it calls and nothing below it:
#     `gap`'s on `sqdiff`'s signature and first fact, `sqdiff`'s on `dot`'s, the hand-written
#     instructions' proof on `dot`'s signature and first fact — K's own account (`params=`);
#   - each file's theorem — the file exits with what its function computes of the two buffers
#     the start-up pushes — is accepted by K in the linked environment, where each parameter is
#     its implementation, on no parameter and no axiom but Lean's three.
# The files' runs by the model and by the processor are the compile_*_file_test.sh's.
# Exit code = the number of checks that failed.
NAME=compile_modules_test; . "$(dirname "$0")/x86_common.sh"
C=v3/compile

"${LOAD[@]}" --link $C/x86/gap_run.shard $C/x86/dot_run.shard $C/x86/sqdiff_hand_run.shard $C/x86/sqdiff_comp_run.shard $C/gap $C/sqdiff $C/dot > "$OUT/m.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the modules did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/m.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/m.txt" || { bad "the summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/m.txt"; }
checks=$((checks+1)); [ "$(grep -c 'RUNNABLE fn ' "$OUT/m.txt")" = 0 ] || { bad "a function of the modules is a program without a definition"; grep 'RUNNABLE fn ' "$OUT/m.txt" | head -5 | cut -c1-200; }

# the definitions: the meanings, the code as data, the loop behind dot's view
for f in compile.sqdiff_src.sqdiff compile.gap_src.gap compile.x86.startup.label_at models.x86.part.table_part models.x86.frame.read_words models.x86.buffers.dot_rows; do
  checks=$((checks+1)); grep -q "^DEFINE $f equations=." "$OUT/m.txt" || bad "$f is not a definition with its equations"
done
# what rests on no view: the laws used, each file's computed facts, the samples — accepted on no
# axiom but Lean's three
for t in models.x86.part.code_in_part models.x86.buffers_facts.dot_rows_in models.x86.buffers_facts.dot_contract compile.lo.rd_word compile.lo.ans_on_pos compile.lo.ans_on_inv compile.two.two_ext_is compile.two.two_ans_inv compile.two.two_ans_tie compile.ir_laws.iends_call_ext compile.x86.seam_laws.kept_code compile.x86.seam_laws.holds_agree compile.x86.seam_laws.realizes_sub compile.x86.seam_laws.realizes_part compile.x86.link.link_exits compile.x86.module.module_realizes compile.x86.module.pair_at_one compile.sample.sample_dot compile.sqdiff_src.sample_sqdiff compile.gap_src.sample_gap compile.x86.dot_file.dot_compiled compile.x86.dot_file.dot_laid compile.x86.dot_file.dot_placed compile.x86.sqdiff_hand_file.sqh_compiled compile.x86.sqdiff_hand_file.sqh_laid compile.x86.sqdiff_hand_file.sqh_placed compile.x86.sqdiff_hand_file.sqh_dot_rows compile.x86.sqdiff_comp_file.sqc_compiled compile.x86.sqdiff_comp_file.sqc_laid compile.x86.sqdiff_comp_file.sqc_dot_placed compile.x86.sqdiff_comp_file.sqc_placed compile.x86.gap_file.gp_sq_compiled compile.x86.gap_file.gp_compiled compile.x86.gap_file.gp_laid compile.x86.gap_file.gp_sq_placed compile.x86.gap_file.gp_placed compile.x86.gap_file.gp_dot_rows; do
  checks=$((checks+1)); grep -q "^ACCEPT $t " "$OUT/m.txt" || bad "$t is not accepted"
  axs=$(grep "^ACCEPT $t " "$OUT/m.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "$t rests on $other"
done
# the proofs behind dot's view that rest on no other module: the loop's cases, its first fact, its tie
for t in dot_lo_go_turn dot_lo_go_done dot_lo_go_means dot_go_tie dot_tie sample_dot_lo; do
  checks=$((checks+1)); grep -q "^IMPL ACCEPT compile.dot.$t " "$OUT/m.txt" || bad "compile.dot.$t is not accepted in dot's implementation"
  axs=$(grep "^IMPL ACCEPT compile.dot.$t " "$OUT/m.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "compile.dot.$t rests on $other"
done

# each view's parameters are parameters to its consumers, and each is met by the implementation
for p in dot.dot_lo:fn sqdiff.sqdiff_lo:fn gap.gap_lo:fn dot.dot_lo_means:requirement dot.dot_prog_tie:requirement sqdiff.sqdiff_lo_means:requirement sqdiff.sqdiff_tie:requirement gap.gap_lo_means:requirement gap.gap_tie:requirement; do
  n=${p%%:*}; k=${p##*:}
  checks=$((checks+1)); grep -q "^PARAM compile.$n $k\$" "$OUT/m.txt" || bad "compile.$n is not a parameter ($k) of its view"
  want=proved; [ "$k" = fn ] && want=defined
  checks=$((checks+1)); grep -q "^DISCHARGE compile.$n $want\$" "$OUT/m.txt" || bad "compile.$n is not discharged ($want) by its implementation"
done
checks=$((checks+1)); [ "$(grep -c '^IMPL-ERROR' "$OUT/m.txt")" = 0 ] || { bad "an implementation fails its view"; grep '^IMPL-ERROR' "$OUT/m.txt" | head -3 | cut -c1-200; }

# what a declaration rests on, of the views: the parameters K lists for it
params() { grep -E "^(IMPL )?ACCEPT compile.$1 " "$OUT/m.txt" | head -1 | sed -E 's/.* params=([^ ]*).*/\1/'; }
# $1 rests on parameters, all of them of the module $2
rests_on() {
  local ps; ps=$(params "$1")
  checks=$((checks+1)); [ -n "$ps" ] || { bad "compile.$1 is not accepted on parameters of a view"; return; }
  checks=$((checks+1)); [ -z "$(echo "$ps" | tr ',' '\n' | grep -v "^compile\.$2\.")" ] || bad "compile.$1 rests on $ps: more than the view of $2"
}
rests_on gap.gap_lo sqdiff;          rests_on gap.gap_lo_means sqdiff;        rests_on gap.gap_tie sqdiff
rests_on sqdiff.sqdiff_lo dot;       rests_on sqdiff.sqdiff_lo_means dot;     rests_on sqdiff.sqdiff_tie dot
rests_on x86.dot_hand.dot_hand_realizes dot
checks=$((checks+1)); [ "$(params x86.dot_hand.dot_hand_realizes)" = "compile.dot.dot_lo_means,compile.dot.dot_lo" ] || bad "the hand-written dot's proof rests on $(params x86.dot_hand.dot_hand_realizes), not on dot's signature and first fact alone"
# the low form's body stays behind its view: the loop is the implementation's own
checks=$((checks+1)); grep -q '^IMPL DEFINE compile.dot.dot_lo_go equations=.' "$OUT/m.txt" || bad "compile.dot.dot_lo_go is not a definition of dot's implementation"
checks=$((checks+1)); [ "$(grep -c '^DEFINE compile.dot.dot_lo_go ' "$OUT/m.txt")" = 0 ] || bad "compile.dot.dot_lo_go is defined in the main line: a consumer holds the low form's body"

# the files' theorems: conditional on parameters against the views, closed in the linked environment
for t in gap_run.gap_file_exits dot_run.dot_file_exits sqdiff_hand_run.sqdiff_hand_exits sqdiff_comp_run.sqdiff_comp_exits; do
  checks=$((checks+1)); [ -n "$(params x86.$t)" ] || bad "compile.x86.$t is not accepted on the views' parameters"
  l=$(grep "^LINK compile.x86.$t " "$OUT/m.txt")
  checks=$((checks+1)); [ -n "$l" ] || { bad "compile.x86.$t has no record of the link"; continue; }
  checks=$((checks+1)); [ -z "$(echo "$l" | sed -E 's/.* params=//')" ] || bad "compile.x86.$t rests on parameters in the linked environment: $l"
  other=$(echo "$l" | sed -E 's/.* axioms=([^ ]*) .*/\1/' | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "compile.x86.$t rests on $other in the linked environment"
done
checks=$((checks+1)); [ "$(grep -c '^LINK-ERROR' "$OUT/m.txt")" = 0 ] || { bad "a declaration did not enter the linked environment"; grep '^LINK-ERROR' "$OUT/m.txt" | head -3 | cut -c1-240; }
# every declaration that rests on a parameter entered it, and none rests on a parameter there
on=$(grep -c -E '^ACCEPT .* params=[^ ]' "$OUT/m.txt")
checks=$((checks+1)); [ "$on" -gt 0 ] && [ "$(grep -c '^LINK ' "$OUT/m.txt")" = "$on" ] || bad "$on declarations rest on a parameter, $(grep -c '^LINK ' "$OUT/m.txt") were linked"
checks=$((checks+1)); [ "$(grep '^LINK ' "$OUT/m.txt" | grep -v -c ' params=$')" = 0 ] || { bad "a linked declaration rests on a parameter"; grep '^LINK ' "$OUT/m.txt" | grep -v ' params=$' | head -3 | cut -c1-240; }

echo "$NAME: $checks checks, $fail failed"
exit $fail
