#!/usr/bin/env bash
# v3/kernel/test/x86_write_test.sh — output at the x86-64 model's boundary (v3/models/x86): the
# kernel's answers to a process's calls as an input of a run and the bytes it has written as
# part of how the run ends (linux.shard); the rules of the system call `write` and the contract
# of `write_all`, instructions that offer a range's bytes until the kernel has taken them all
# (write.shard); and a file that writes two pieces of text with it (hello.shard), proven to end
# as its theorem's right side says for every stack and EVERY list of answers (hello_run.shard).
# Held here: those theorems accepted on Lean's axioms; for lists of answers — every byte taken
# at once, one byte a call, more than was offered, a failure before any byte and between the
# pieces, the process ended by the kernel — the theorem's right side, evaluated, and the
# model's run of the file are one line; and the processor's runs: the text and status 0; with
# the output closed, and to a full device, the status of a failed write; to a pipe nobody
# reads, the process ended by the kernel — and the status of a failed write where that signal
# is ignored.
# Exit code = the number of checks that failed.
NAME=x86_write_test; . "$(dirname "$0")/x86_common.sh"
no_chip

"${LOAD[@]}" $M/hello_run.shard $M/hello_main.shard > "$OUT/load.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the files did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/load.txt" || { bad "the summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
for f in linux.took linux.mem_bytes linux.write_to linux.on_write linux.on_syscall linux.move linux.run linux.send linux.sends_end write.write_all hello.hello_listing hello.hello_res; do
  checks=$((checks+1)); grep -q "^DEFINE models.x86.$f equations=." "$OUT/load.txt" || bad "models.x86.$f is not a definition with its equations"
done
for t in laws.run_of_steps laws.comes_trans laws.comes_of_reaches laws.ends_of_comes laws.comes_of_move laws.ends_of_move laws.run_exit laws.mem_bytes_length laws.mem_bytes_take laws.mem_bytes_drop laws.mem_bytes_split laws.mem_bytes_outside loaded.loaded_byte loaded.loaded_bytes write.took_le write.took_pos write.err_word_big write.write_call write.write_took write.write_err write.write_kill write.wa_code_same write.wa_head write.wa_done write.wa_more write.wa_loop write.wa_fail write.wa_turn write.wa_turn_err write.wa_turn_kill write.returned_frame write.wa_post_step write.wa_zero write.send_took write.send_err write.send_kill write.write_all_ok hello_run.hello_assembled hello_run.hello_laid hello_run.hello_one_in hello_run.hello_two_in hello_run.hello_wa hello_run.piece_of_wa hello_run.hello_piece hello_run.hello_exit hello_run.hello_second hello_run.hello_both hello_run.hello_ends hello_run.hello_exits; do
  checks=$((checks+1)); grep -q "^ACCEPT models.x86.$t " "$OUT/load.txt" || bad "models.x86.$t is not accepted"
  axs=$(grep "^ACCEPT models.x86.$t " "$OUT/load.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "models.x86.$t rests on $other"
  checks=$((checks+1)); grep "^ACCEPT models.x86.$t " "$OUT/load.txt" | grep -q ' params= ' || bad "models.x86.$t rests on a parameter"
done

# ---- the file ---------------------------------------------------------------------------------------
HM=("${LOAD[@]}" --run models.x86.hello_main.main $M/hello_main.shard --)
"${HM[@]}" write "$OUT/hello" > "$OUT/write.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$(stat -c %s "$OUT/hello" 2>/dev/null)" = 236 ] || { bad "the writer did not write the 236 bytes (exit $rc)"; tail -3 "$OUT/write.txt" | cut -c1-200; }
TEXT="68 65 6c 6c 6f 2c 20 77 6f 72 6c 64 0a"
# lists of answers: all at once; one byte a call; counts that split the pieces; more than was
# offered; a failure at the first call, inside the first piece, at the second piece's first
# call, inside the second; the process ended at the first call, between the pieces, inside one
SPECS=(0 1,1,1,1,1,1,1,1,1,1,1,1,1 3,1,1,2 100,100 e5 2,e9 7,e1 7,3,e4095 k 7,k 7,3,k 2,2,k)
WANT=("exit 0, wrote $TEXT" "exit 0, wrote $TEXT" "exit 0, wrote $TEXT" "exit 0, wrote $TEXT" "exit 74" "exit 74, wrote 68 65" "exit 74, wrote 68 65 6c 6c 6f 2c 20" "exit 74, wrote 68 65 6c 6c 6f 2c 20 77 6f 72" "killed" "killed, wrote 68 65 6c 6c 6f 2c 20" "killed, wrote 68 65 6c 6c 6f 2c 20 77 6f 72" "killed, wrote 68 65 6c 6c")
"${HM[@]}" says "${SPECS[@]}" 2>&1 | tail -${#SPECS[@]} > "$OUT/says.txt"
"${RUN[@]}" answers 400 "$OUT/hello" "${SPECS[@]}" 2>&1 | tail -${#SPECS[@]} > "$OUT/runs.txt"
for i in "${!SPECS[@]}"; do
  s=$(sed -n "$((i+1))p" "$OUT/says.txt"); r=$(sed -n "$((i+1))p" "$OUT/runs.txt")
  checks=$((checks+1)); [ "$s" = "${WANT[$i]}" ] || bad "on the answers ${SPECS[$i]} the theorem says: $s — expected: ${WANT[$i]}"
  checks=$((checks+1)); [ "$r" = "$s" ] || bad "on the answers ${SPECS[$i]} the model's run is: $r — the theorem says: $s"
done
# with no answers given the kernel takes all it is offered: the same run as `run` makes
m=$("${RUN[@]}" run 400 "$OUT/hello" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 0, wrote $TEXT" ] || bad "the model's run of the file is: $m"

# ---- the processor ----------------------------------------------------------------------------------
if [ $onchip = 1 ]; then
  chmod +x "$OUT/hello"
  "$OUT/hello" > "$OUT/chip.out" 2>/dev/null; rc=$?
  got="exit $rc, wrote$(od -An -tx1 -v "$OUT/chip.out" | tr -s ' \n' ' ' | sed 's/ $//')"
  checks=$((checks+1)); [ "$got" = "exit 0, wrote $TEXT" ] || bad "the processor's run of the file is: $got"
  "$OUT/hello" >&- 2>/dev/null; rc=$?
  checks=$((checks+1)); [ "$rc" = 74 ] || bad "with its output closed the file's status is $rc, the model's on a failed write 74"
  if [ -c /dev/full ]; then
    "$OUT/hello" > /dev/full 2>/dev/null; rc=$?
    checks=$((checks+1)); [ "$rc" = 74 ] || bad "writing to a full device the file's status is $rc, the model's on a failed write 74"
  fi
  # a pipe with no reader: opened for both, then for writing, and the first closed
  rc=$( bash -c 'mkfifo "$1"; exec 3<>"$1"; exec 4>"$1"; exec 3>&-; "$0" >&4; echo $?' "$OUT/hello" "$OUT/pipe" 2>/dev/null ); rm -f "$OUT/pipe"
  checks=$((checks+1)); [ "$rc" = 141 ] || bad "writing to a pipe nobody reads the shell reports $rc, not the signal's 141: the model's answer that ends the process"
  rc=$( bash -c 'trap "" PIPE; mkfifo "$1"; exec 3<>"$1"; exec 4>"$1"; exec 3>&-; "$0" >&4; echo $?' "$OUT/hello" "$OUT/pipe" 2>/dev/null ); rm -f "$OUT/pipe"
  checks=$((checks+1)); [ "$rc" = 74 ] || bad "writing to a pipe nobody reads with the signal ignored the file's status is $rc, the model's on a failed write 74"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
