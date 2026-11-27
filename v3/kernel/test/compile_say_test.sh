#!/usr/bin/env bash
# v3/kernel/test/compile_say_test.sh — a COMPILED program that writes IN A LOOP
# (v3/compile/say_src.shard): `say`, a function of its command-line arguments on a World
# (v3/std/world.shard) that writes each argument after the name on a line of its own. Its
# intermediate code is four functions — a string's length, one line, the loop over the
# arguments, the entry — that call the platform's `write_all` as an outside function and go
# through the pass; its file (x86/say_file.shard) is proven to end as `say` says on the World
# its process starts with, for every stack, EVERY list of arguments the kernel leaves and
# EVERY list of the kernel's answers (x86/say_run.shard `say_file_ends`).
# Held here: the theorems of the chain accepted on Lean's axioms and on no parameter — the
# rules of a run with what is outside for a branch, a tail call and a call of a function of the
# program, the rules of a tie (v3/compile/tie.shard), the ties of the four functions and the
# file's theorem; for cases of answers and arguments — every byte taken at once, a byte or a
# few a call, more than was offered, no argument, no name, an empty argument, a failure before
# any byte, inside an argument, at a line's end and in a later turn of the loop, the process
# ended by the kernel — the theorem's right side, evaluated, and the model's run of the file
# are one line; and the processor's runs: the lines and status 0, for a few arguments and for
# two hundred, the same as the model's run; nothing and status 0 with no argument, even with
# the output closed; with the output closed, and to a full device, the status of a failed
# write; to a pipe nobody reads, the process ended by the kernel — and the status of a failed
# write where that signal is ignored; and a SHORT write: an argument of 100,000 bytes to a pipe
# that does not block is taken in part, the next call fails, and the file exits with the
# status of a failed write having written exactly the bytes taken.
# Exit code = the number of checks that failed.
NAME=compile_say_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

"${LOAD[@]}" $C/x86/say_run.shard $C/x86/say_main.shard > "$OUT/load.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the files did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/load.txt" || { bad "the summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
for f in std.world.write_all compile.put.put_ext compile.put.put_wx compile.say_src.say_line compile.say_src.say_each compile.say_src.say compile.say_ir.count_on compile.say_ir.say_res models.x86.world.world_res; do
  checks=$((checks+1)); grep -q "^DEFINE $f equations=." "$OUT/load.txt" || bad "$f is not a definition with its equations"
done
for t in compile.ir_laws.wends_if_true compile.ir_laws.wends_if_false compile.ir_laws.wends_tail compile.ir_laws.wends_call compile.ir_laws.wends_call_end models.x86.world.tied_exit std.world.write_all_stopped models.x86.args.str_bytes compile.tie.tie_fn compile.tie.tie_ret compile.tie.tie_let compile.tie.tie_if_true compile.tie.tie_if_false compile.tie.tie_tail compile.tie.tie_call_same compile.tie.tied_end compile.tie.tie_call compile.tie.tie_ext compile.put.put_tied compile.put.tie_put compile.put.put_wx_is compile.x86.put.put_realizes compile.x86.put.put_wx_realizes compile.x86.module.wmodule_realizes compile.x86.module.one_out_realizes compile.x86.link.start_ends_top compile.say_src.say_status compile.say_src.say_line_stopped compile.say_src.say_each_stopped compile.say_src.sample_say compile.say_ir.count_on_val compile.say_ir.say_tie0 compile.say_ir.say_tie1 compile.say_ir.say_tie2 compile.say_ir.say_tie3 compile.say_ir.say_prog_tie compile.x86.say_run.say_code_realizes compile.x86.say_run.say_answer compile.x86.say_run.say_file_ends compile.x86.say_run.say_file_says; do
  checks=$((checks+1)); grep -q "^ACCEPT $t " "$OUT/load.txt" || bad "$t is not accepted"
  axs=$(grep "^ACCEPT $t " "$OUT/load.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "$t rests on $other"
  checks=$((checks+1)); grep "^ACCEPT $t " "$OUT/load.txt" | grep -q ' params= ' || bad "$t rests on a parameter"
done

# ---- the file ---------------------------------------------------------------------------------------
SM=("${LOAD[@]}" --run compile.x86.say_main.main $C/x86/say_main.shard --)
"${SM[@]}" write "$OUT/say" > "$OUT/write.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$(stat -c %s "$OUT/say" 2>/dev/null)" = 819 ] || { bad "the writer did not write the 819 bytes (exit $rc)"; tail -3 "$OUT/write.txt" | cut -c1-200; }
# what a process run with these arguments writes where every byte is taken: each argument after
# the first and a line's end, as the runner prints bytes
wrote() { shift; local o; o=$( { for a in "$@"; do printf '%s\n' "$a"; done; } | od -An -tx1 -v | tr -s ' \n' ' ' | sed 's/ $//'); [ $# = 0 ] && echo "exit 0" || echo "exit 0, wrote$o"; }
AZ=abcdefghijklmnopqrstuvwxyz
# cases — the kernel's answers, then the arguments, the first the name — and what each ends in:
# all at once; one byte a call; more than was offered; counts that split an argument; twelve
# arguments; no argument; no name; an empty argument, and a failure at the line's end after it
# (an empty argument offers nothing and takes no answer); a failure at the first call, inside
# an argument, at a line's end, in the loop's third turn; the process ended at the first call,
# at a line's end, in the third turn
CASES=(); WANT=()
add() { local want=$1; shift; [ ${#CASES[@]} = 0 ] || CASES+=(/); CASES+=("$@"); WANT+=("$want"); }
add "$(wrote say hi yo)" 0 say hi yo
add "$(wrote say hi yo)" 1,1,1,1,1,1 say hi yo
add "$(wrote say hi yo)" 100,100,100,100 say hi yo
add "$(wrote say $AZ 0123456789)" 0 say $AZ 0123456789
add "$(wrote say $AZ)" 7,7,7,7,7,7 say $AZ
add "$(wrote say 1 2 3 4 5 6 7 8 9 10 11 12)" 0 say 1 2 3 4 5 6 7 8 9 10 11 12
add "exit 0" 0 say
add "exit 0" 0
add "$(wrote say "" x)" 0 say "" x
add "exit 74" e1 say ""
add "exit 74" e5 say hi
add "exit 74, wrote 68" 1,e9 say hi yo
add "exit 74, wrote 68 69" 2,e9 say hi yo
add "exit 74, wrote 61 0a 62" 0,0,0,e4095 say a b c
add "killed" k say hi
add "killed, wrote 68 69" 3,k say hi yo
add "killed, wrote 61 0a 62" 0,0,0,k say a b c
"${SM[@]}" says "${CASES[@]}" 2>&1 | tail -${#WANT[@]} > "$OUT/says.txt"
"${RUN[@]}" cases 20000 "$OUT/say" "${CASES[@]}" 2>&1 | tail -${#WANT[@]} > "$OUT/runs.txt"
for i in "${!WANT[@]}"; do
  s=$(sed -n "$((i+1))p" "$OUT/says.txt"); r=$(sed -n "$((i+1))p" "$OUT/runs.txt")
  checks=$((checks+1)); [ "$s" = "${WANT[$i]}" ] || bad "in case $((i+1)) the theorem says: $s — expected: ${WANT[$i]}"
  checks=$((checks+1)); [ "$r" = "$s" ] || bad "in case $((i+1)) the model's run is: $r — the theorem says: $s"
done

# ---- the processor ----------------------------------------------------------------------------------
if [ $onchip = 1 ]; then
  chmod +x "$OUT/say"
  # a file's run by the processor with arguments, as the runner prints a run
  ran() { local rc; "$OUT/say" "$@" > "$OUT/chip.out" 2>/dev/null; rc=$?; local o; o=$(od -An -tx1 -v "$OUT/chip.out" | tr -s ' \n' ' ' | sed 's/ $//'); [ -s "$OUT/chip.out" ] && echo "exit $rc, wrote$o" || echo "exit $rc"; }
  got=$(ran hello "big world" "" x)
  checks=$((checks+1)); [ "$got" = "$(wrote say hello "big world" "" x)" ] || bad "the processor's run with four arguments is: $got"
  m=$("${RUN[@]}" cases 20000 "$OUT/say" 0 "$OUT/say" hello "big world" "" x 2>&1 | tail -1)
  checks=$((checks+1)); [ "$m" = "$got" ] || bad "the model's run with the four arguments is: $m — the processor's: $got"
  got=$(ran)
  checks=$((checks+1)); [ "$got" = "exit 0" ] || bad "the processor's run with no argument is: $got"
  got=$(ran $(seq 1 200))
  checks=$((checks+1)); [ "$got" = "$(wrote say $(seq 1 200))" ] || bad "the processor's run with two hundred arguments is not their two hundred lines and status 0"
  "$OUT/say" >&- 2>/dev/null; rc=$?
  checks=$((checks+1)); [ "$rc" = 0 ] || bad "with its output closed and no argument the file's status is $rc: it writes nothing, and the theorem says 0"
  "$OUT/say" hi >&- 2>/dev/null; rc=$?
  checks=$((checks+1)); [ "$rc" = 74 ] || bad "with its output closed the file's status is $rc, the model's on a failed write 74"
  if [ -c /dev/full ]; then
    "$OUT/say" hi yo > /dev/full 2>/dev/null; rc=$?
    checks=$((checks+1)); [ "$rc" = 74 ] || bad "writing to a full device the file's status is $rc, the model's on a failed write 74"
  fi
  # a pipe with no reader: opened for both, then for writing, and the first closed
  rc=$( bash -c 'mkfifo "$1"; exec 3<>"$1"; exec 4>"$1"; exec 3>&-; "$0" hi yo >&4; echo $?' "$OUT/say" "$OUT/pipe" 2>/dev/null ); rm -f "$OUT/pipe"
  checks=$((checks+1)); [ "$rc" = 141 ] || bad "writing to a pipe nobody reads the shell reports $rc, not the signal's 141: the model's answer that ends the process"
  rc=$( bash -c 'trap "" PIPE; mkfifo "$1"; exec 3<>"$1"; exec 4>"$1"; exec 3>&-; "$0" hi yo >&4; echo $?' "$OUT/say" "$OUT/pipe" 2>/dev/null ); rm -f "$OUT/pipe"
  checks=$((checks+1)); [ "$rc" = 74 ] || bad "writing to a pipe nobody reads with the signal ignored the file's status is $rc, the model's on a failed write 74"
  # A SHORT WRITE. A pipe set not to block (`dd` sets the flag on the pipe it is given as its
  # output) takes of 100,000 bytes what it has room for; the call after it fails, the pipe full.
  # The file's status is a failed write's and the pipe holds exactly the bytes taken: the
  # model's answers `N,e11`, N the count the kernel took.
  head -c 100000 /dev/zero | tr '\0' 'q' > "$OUT/big"
  res=$( bash -c 'mkfifo "$1"; exec 3<>"$1"; exec 4<"$1"; dd oflag=nonblock count=0 >&3 2>/dev/null; "$0" "$(cat "$2")" >&3; rc=$?; exec 3>&-; cat <&4 > "$3"; echo $rc' "$OUT/say" "$OUT/pipe" "$OUT/big" "$OUT/taken" 2>/dev/null ); rm -f "$OUT/pipe"
  n=$(stat -c %s "$OUT/taken" 2>/dev/null || echo 0)
  checks=$((checks+1)); [ "$res" = 74 ] || bad "after a short write and a failed one the file's status is $res, the model's on a failed write 74"
  checks=$((checks+1)); [ "$n" -gt 0 ] && [ "$n" -lt 100000 ] || bad "the pipe took $n of the 100000 bytes: not a short write"
  checks=$((checks+1)); head -c "$n" "$OUT/big" | cmp -s - "$OUT/taken" || bad "the pipe does not hold the first $n bytes of the argument"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
