#!/usr/bin/env bash
# v3/kernel/test/compile_greet_test.sh — a COMPILED program that writes (v3/compile/greet_src.shard):
# `greet`, a function on a World (v3/std/world.shard), whose intermediate code calls the
# platform's `write_all` as an outside function (v3/compile/put.shard, x86/put.shard) and goes
# through the pass; its file (x86/greet_file.shard) is proven to end as `greet` says on the
# World its process starts with, for every stack and EVERY list of the kernel's answers
# (x86/greet_run.shard `greet_file_ends`).
# Held here: the theorems of the chain accepted on Lean's axioms and on no parameter — the run
# of intermediate code with what is outside a process, the pass's theorem and the link over
# it, `write_all`'s code at a seam, the rules that tie a function that writes (v3/compile/tie.shard,
# put.shard), the tie and the file's theorem; for lists of answers — every byte taken at once, one byte a call, more than
# was offered, a failure before any byte, inside a piece and between the pieces, the process
# ended by the kernel — the theorem's right side, evaluated, and the model's run of the file
# are one line; and the processor's runs: the text and status 0; with the output closed, and
# to a full device, the status of a failed write; to a pipe nobody reads, the process ended by
# the kernel — and the status of a failed write where that signal is ignored.
# Exit code = the number of checks that failed.
NAME=compile_greet_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

"${LOAD[@]}" $C/x86/greet_run.shard $C/x86/greet_main.shard > "$OUT/load.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the files did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/load.txt" || { bad "the summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
for f in std.world.took std.world.put std.world.after_put std.world.write_all compile.put.put_out compile.put.put_ext compile.put.put_wx compile.greet_src.greet compile.greet_ir.greet_res models.x86.world.world_res models.x86.linux.exit_of; do
  checks=$((checks+1)); grep -q "^DEFINE $f equations=." "$OUT/load.txt" || bad "$f is not a definition with its equations"
done
# the run with the outside is a definition by its recursion: accepted, with its equations
checks=$((checks+1)); grep -q "^ACCEPT compile.ir.wrun " "$OUT/load.txt" && grep -q "^TEMPLATE compile.ir.wrun .*equations=." "$OUT/load.txt" || bad "compile.ir.wrun is not accepted with its equations"
for t in compile.ir_laws.wlift_is compile.ir_laws.wrun_more compile.ir_laws.wends_call_ext compile.ir_laws.wends_call_ext_end compile.x86.seam.arrives_comes compile.x86.seam.realizes_lift compile.x86.call.call_law compile.x86.tail.tail_law compile.x86.function.sim_all compile.x86.function.cfn_ok compile.x86.module.wmodule_realizes compile.x86.module.lift_ext compile.x86.module.unlift_fn compile.x86.module.module_realizes compile.x86.link.exit_ends compile.x86.link.start_ends_top compile.x86.link.start_exits_top compile.x86.link.start_ends models.x86.world.send_put models.x86.world.tied_exit std.world.write_all_stopped models.x86.bytes.le64_bytes models.x86.bytes.held_bytes compile.put.put_ext_inv compile.put.put_all compile.put.put_fail compile.put.put_kill compile.put.put_tied compile.put.tie_put compile.tie.tie_fn compile.tie.tie_ret compile.tie.tie_let compile.tie.tie_ext compile.x86.put.put_arrives compile.x86.put.put_realizes compile.x86.put.put_wx_realizes compile.x86.module.one_out_realizes compile.greet_ir.greet_tie compile.x86.greet_run.greet_code_realizes compile.x86.greet_run.greet_answer compile.x86.greet_run.greet_file_ends compile.x86.greet_run.greet_file_writes; do
  checks=$((checks+1)); grep -q "^ACCEPT $t " "$OUT/load.txt" || bad "$t is not accepted"
  axs=$(grep "^ACCEPT $t " "$OUT/load.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "$t rests on $other"
  checks=$((checks+1)); grep "^ACCEPT $t " "$OUT/load.txt" | grep -q ' params= ' || bad "$t rests on a parameter"
done

# ---- the file ---------------------------------------------------------------------------------------
GM=("${LOAD[@]}" --run compile.x86.greet_main.main $C/x86/greet_main.shard --)
"${GM[@]}" write "$OUT/greet" > "$OUT/write.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$(stat -c %s "$OUT/greet" 2>/dev/null)" = 357 ] || { bad "the writer did not write the 357 bytes (exit $rc)"; tail -3 "$OUT/write.txt" | cut -c1-200; }
TEXT="68 65 6c 6c 6f 2c 20 77 6f 72 6c 64 0a"
# lists of answers: all at once; one byte a call; counts that split the first piece; more than
# was offered; a failure at the first call, inside the first piece, at the second piece's
# first call, inside the second; the process ended at the first call, between the pieces,
# inside each
SPECS=(0 1,1,1,1,1,1,1,1,1,1,1,1,1 3,1,1,2,1 100,100 e5 2,e9 8,e1 8,3,e4095 k 8,k 8,3,k 2,2,k)
WANT=("exit 0, wrote $TEXT" "exit 0, wrote $TEXT" "exit 0, wrote $TEXT" "exit 0, wrote $TEXT" "exit 74" "exit 74, wrote 68 65" "exit 74, wrote 68 65 6c 6c 6f 2c 20 77" "exit 74, wrote 68 65 6c 6c 6f 2c 20 77 6f 72 6c" "killed" "killed, wrote 68 65 6c 6c 6f 2c 20 77" "killed, wrote 68 65 6c 6c 6f 2c 20 77 6f 72 6c" "killed, wrote 68 65 6c 6c")
"${GM[@]}" says "${SPECS[@]}" 2>&1 | tail -${#SPECS[@]} > "$OUT/says.txt"
"${RUN[@]}" answers 2000 "$OUT/greet" "${SPECS[@]}" 2>&1 | tail -${#SPECS[@]} > "$OUT/runs.txt"
for i in "${!SPECS[@]}"; do
  s=$(sed -n "$((i+1))p" "$OUT/says.txt"); r=$(sed -n "$((i+1))p" "$OUT/runs.txt")
  checks=$((checks+1)); [ "$s" = "${WANT[$i]}" ] || bad "on the answers ${SPECS[$i]} the theorem says: $s — expected: ${WANT[$i]}"
  checks=$((checks+1)); [ "$r" = "$s" ] || bad "on the answers ${SPECS[$i]} the model's run is: $r — the theorem says: $s"
done
# with no answers given the kernel takes all it is offered: the same run as `run` makes
m=$("${RUN[@]}" run 2000 "$OUT/greet" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 0, wrote $TEXT" ] || bad "the model's run of the file is: $m"

# ---- the processor ----------------------------------------------------------------------------------
if [ $onchip = 1 ]; then
  chmod +x "$OUT/greet"
  "$OUT/greet" > "$OUT/chip.out" 2>/dev/null; rc=$?
  got="exit $rc, wrote$(od -An -tx1 -v "$OUT/chip.out" | tr -s ' \n' ' ' | sed 's/ $//')"
  checks=$((checks+1)); [ "$got" = "exit 0, wrote $TEXT" ] || bad "the processor's run of the file is: $got"
  "$OUT/greet" >&- 2>/dev/null; rc=$?
  checks=$((checks+1)); [ "$rc" = 74 ] || bad "with its output closed the file's status is $rc, the model's on a failed write 74"
  if [ -c /dev/full ]; then
    "$OUT/greet" > /dev/full 2>/dev/null; rc=$?
    checks=$((checks+1)); [ "$rc" = 74 ] || bad "writing to a full device the file's status is $rc, the model's on a failed write 74"
  fi
  # a pipe with no reader: opened for both, then for writing, and the first closed
  rc=$( bash -c 'mkfifo "$1"; exec 3<>"$1"; exec 4>"$1"; exec 3>&-; "$0" >&4; echo $?' "$OUT/greet" "$OUT/pipe" 2>/dev/null ); rm -f "$OUT/pipe"
  checks=$((checks+1)); [ "$rc" = 141 ] || bad "writing to a pipe nobody reads the shell reports $rc, not the signal's 141: the model's answer that ends the process"
  rc=$( bash -c 'trap "" PIPE; mkfifo "$1"; exec 3<>"$1"; exec 4>"$1"; exec 3>&-; "$0" >&4; echo $?' "$OUT/greet" "$OUT/pipe" 2>/dev/null ); rm -f "$OUT/pipe"
  checks=$((checks+1)); [ "$rc" = 74 ] || bad "writing to a pipe nobody reads with the signal ignored the file's status is $rc, the model's on a failed write 74"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
