#!/usr/bin/env bash
# v3/kernel/test/compile_up_test.sh — a COMPILED program whose code STORES
# (v3/compile/up_src.shard): `up`, a function of its command-line arguments on a World
# (v3/std/world.shard) that writes each argument after the name with every character replaced
# by the one after it (`up HAL` writes IBM), one write a character, a line's end after each.
# The character to write is computed, so its code keeps it somewhere: four functions
# (v3/compile/up_ir.shard), the first of which stores the character's word in a scratch word
# of the file's data and writes one byte from there. Each is tied in the one form of a tie
# (v3/compile/tie.shard): a word, a World, and a CONDITION on the memory left — here that it is
# the memory given but for the scratch word —, by one rule a statement and the lemmas of the
# values' representations (v3/compile/rep.shard): a value that outlives a store is carried
# over by its `_keep` lemma, on the evidence that its region is apart from the scratch word's.
# The file (x86/up_file.shard) is proven to end as `up` says on the World its process starts
# with, for every stack, EVERY list of arguments the kernel leaves and EVERY list of the
# kernel's answers (x86/up_run.shard `up_file_ends`).
# Held here: the theorems of the chain accepted on Lean's axioms and on no parameter — the
# rule of a run for a store, the rules of a tie with a condition, the representations' lemmas
# the ties cite, the four ties and the file's theorem; for cases of answers and arguments —
# every write taken, more than was offered, no argument, no name, an empty argument, the
# character 255 (the next is 0), a failure at the first write, inside an argument, at a line's
# end and in a later argument, the process ended by the kernel — the theorem's right side,
# evaluated, and the model's run of the file are one line; and the processor's runs: `up HAL`
# writes IBM; the lines and status 0 for a few arguments and for two hundred, the same as the
# model's run; nothing and status 0 with no argument, even with the output closed; with the
# output closed, and to a full device, the status of a failed write; to a pipe nobody reads,
# the process ended by the kernel — and the status of a failed write where that signal is
# ignored; and a pipe that does not block and fills: the file exits with the status of a
# failed write having written exactly the characters taken.
# Exit code = the number of checks that failed.
NAME=compile_up_test; . "$(dirname "$0")/x86_common.sh"
no_chip
C=v3/compile

"${LOAD[@]}" $C/x86/up_run.shard $C/x86/up_main.shard > "$OUT/load.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the files did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/load.txt" || { bad "the summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
for f in std.world.write_all compile.put.put_ext compile.put.put_wx compile.up_src.up_char compile.up_src.up_each compile.up_src.up_line compile.up_src.up_all compile.up_src.up compile.up_ir.up_res models.x86.world.world_res; do
  checks=$((checks+1)); grep -q "^DEFINE $f equations=." "$OUT/load.txt" || bad "$f is not a definition with its equations"
done
for t in compile.ir_laws.wends_store models.x86.world.tied_weaken models.x86.world.tied_exit compile.tie.tie_fn compile.tie.tie_weaken compile.tie.tie_ret compile.tie.tie_let compile.tie.tie_store compile.tie.tie_if_true compile.tie.tie_if_false compile.tie.tie_tail compile.tie.tie_call compile.tie.tie_ext compile.put.put_tied compile.put.tie_put compile.rep.readable_in compile.rep.writable_in compile.rep.args_rs_readable compile.rep.args_rs_writable compile.rep.chr_add_low compile.rep.str_keep compile.rep.str_load compile.rep.str_nil_test compile.rep.str_cons_test compile.rep.str_cons_chr compile.rep.str_cons_tail compile.rep.bytes_keep compile.rep.bytes_put compile.rep.cell_store compile.rep.cell_frame compile.rep.cell_apart compile.rep.cell_put1 compile.rep.argv_keep compile.rep.argv_nil_test compile.rep.argv_cons_test compile.rep.argv_cons_load compile.rep.argv_cons_head compile.rep.argv_cons_tail compile.rep.args_load compile.rep.args_nil_test compile.rep.args_cons_test compile.rep.args_cons_tail compile.x86.put.put_wx_realizes compile.x86.module.wmodule_realizes compile.x86.module.one_out_realizes compile.x86.link.start_ends_top compile.up_src.up_status compile.up_src.up_each_stopped compile.up_src.up_line_stopped compile.up_src.up_all_stopped compile.up_src.sample_up compile.up_ir.up_tie0 compile.up_ir.up_tie1 compile.up_ir.up_tie2 compile.up_ir.up_tie3 compile.up_ir.up_prog_tie compile.x86.up_run.up_code_realizes compile.x86.up_run.up_answer compile.x86.up_run.up_file_ends compile.x86.up_run.up_file_says; do
  checks=$((checks+1)); grep -q "^ACCEPT $t " "$OUT/load.txt" || bad "$t is not accepted"
  axs=$(grep "^ACCEPT $t " "$OUT/load.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "$t rests on $other"
  checks=$((checks+1)); grep "^ACCEPT $t " "$OUT/load.txt" | grep -q ' params= ' || bad "$t rests on a parameter"
done

# ---- the file ---------------------------------------------------------------------------------------
SM=("${LOAD[@]}" --run compile.x86.up_main.main $C/x86/up_main.shard --)
"${SM[@]}" write "$OUT/up" > "$OUT/write.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$(stat -c %s "$OUT/up" 2>/dev/null)" = 856 ] || { bad "the writer did not write the 856 bytes (exit $rc)"; tail -3 "$OUT/write.txt" | cut -c1-200; }
# what a process run with these arguments writes where every write is taken: for each argument
# after the first its bytes, each one more (255 gives 0), and a line's end — as the runner
# prints bytes
wrote() { shift; local o="" a b; for a in "$@"; do for b in $(printf '%s' "$a" | od -An -tu1 -v); do o="$o $(printf '%02x' $(( (b + 1) % 256 )))"; done; o="$o 0a"; done; [ $# = 0 ] && echo "exit 0" || echo "exit 0, wrote$o"; }
AZ=abcdefghijklmnopqrstuvwxyz
# cases — the kernel's answers, then the arguments, the first the name — and what each ends in:
# every write taken; a count for each; more than was offered; a long argument; twelve
# arguments; no argument; no name; an empty argument; the character 255; a failure at the
# first write (of a character, and of the line's end of an empty argument), inside an argument,
# at a line's end, in the second argument; the process ended at the first write, at a line's
# end
CASES=(); WANT=()
add() { local want=$1; shift; [ ${#CASES[@]} = 0 ] || CASES+=(/); CASES+=("$@"); WANT+=("$want"); }
add "exit 0, wrote 49 42 4d 0a" 0 up HAL
add "$(wrote up HAL)" 1,1,1,1 up HAL
add "$(wrote up HAL az)" 100,100,100,100,100,100,100 up HAL az
add "$(wrote up $AZ 0123456789)" 0 up $AZ 0123456789
add "$(wrote up 1 2 3 4 5 6 7 8 9 10 11 12)" 0 up 1 2 3 4 5 6 7 8 9 10 11 12
add "exit 0" 0 up
add "exit 0" 0
add "$(wrote up "" x)" 0 up "" x
add "exit 0, wrote 00 0a" 0 up $'\xff'
add "exit 74" e5 up hi
add "exit 74" e1 up ""
add "exit 74, wrote 69" 0,e9 up hi yo
add "exit 74, wrote 69 6a" 0,0,e9 up hi yo
add "exit 74, wrote 69 6a 0a 7a" 0,0,0,0,e4095 up hi yo
add "killed" k up hi
add "killed, wrote 69 6a" 0,0,k up hi yo
"${SM[@]}" says "${CASES[@]}" 2>&1 | tail -${#WANT[@]} > "$OUT/says.txt"
"${RUN[@]}" cases 40000 "$OUT/up" "${CASES[@]}" 2>&1 | tail -${#WANT[@]} > "$OUT/runs.txt"
for i in "${!WANT[@]}"; do
  s=$(sed -n "$((i+1))p" "$OUT/says.txt"); r=$(sed -n "$((i+1))p" "$OUT/runs.txt")
  checks=$((checks+1)); [ "$s" = "${WANT[$i]}" ] || bad "in case $((i+1)) the theorem says: $s — expected: ${WANT[$i]}"
  checks=$((checks+1)); [ "$r" = "$s" ] || bad "in case $((i+1)) the model's run is: $r — the theorem says: $s"
done

# ---- the processor ----------------------------------------------------------------------------------
if [ $onchip = 1 ]; then
  chmod +x "$OUT/up"
  # a file's run by the processor with arguments, as the runner prints a run
  ran() { local rc; "$OUT/up" "$@" > "$OUT/chip.out" 2>/dev/null; rc=$?; local o; o=$(od -An -tx1 -v "$OUT/chip.out" | tr -s ' \n' ' ' | sed 's/ $//'); [ -s "$OUT/chip.out" ] && echo "exit $rc, wrote$o" || echo "exit $rc"; }
  got=$("$OUT/up" HAL 2>/dev/null)
  checks=$((checks+1)); [ "$got" = "IBM" ] || bad "the processor's run of \`up HAL\` wrote: $got"
  got=$(ran hello "big world" "" x)
  checks=$((checks+1)); [ "$got" = "$(wrote up hello "big world" "" x)" ] || bad "the processor's run with four arguments is: $got"
  m=$("${RUN[@]}" cases 40000 "$OUT/up" 0 "$OUT/up" hello "big world" "" x 2>&1 | tail -1)
  checks=$((checks+1)); [ "$m" = "$got" ] || bad "the model's run with the four arguments is: $m — the processor's: $got"
  got=$(ran $'\xff\xfe')
  checks=$((checks+1)); [ "$got" = "exit 0, wrote 00 ff 0a" ] || bad "the processor's run with the characters 255 and 254 is: $got"
  got=$(ran)
  checks=$((checks+1)); [ "$got" = "exit 0" ] || bad "the processor's run with no argument is: $got"
  got=$(ran $(seq 1 200))
  checks=$((checks+1)); [ "$got" = "$(wrote up $(seq 1 200))" ] || bad "the processor's run with two hundred arguments is not their two hundred lines and status 0"
  "$OUT/up" >&- 2>/dev/null; rc=$?
  checks=$((checks+1)); [ "$rc" = 0 ] || bad "with its output closed and no argument the file's status is $rc: it writes nothing, and the theorem says 0"
  "$OUT/up" hi >&- 2>/dev/null; rc=$?
  checks=$((checks+1)); [ "$rc" = 74 ] || bad "with its output closed the file's status is $rc, the model's on a failed write 74"
  if [ -c /dev/full ]; then
    "$OUT/up" hi yo > /dev/full 2>/dev/null; rc=$?
    checks=$((checks+1)); [ "$rc" = 74 ] || bad "writing to a full device the file's status is $rc, the model's on a failed write 74"
  fi
  # a pipe with no reader: opened for both, then for writing, and the first closed
  rc=$( bash -c 'mkfifo "$1"; exec 3<>"$1"; exec 4>"$1"; exec 3>&-; "$0" hi yo >&4; echo $?' "$OUT/up" "$OUT/pipe" 2>/dev/null ); rm -f "$OUT/pipe"
  checks=$((checks+1)); [ "$rc" = 141 ] || bad "writing to a pipe nobody reads the shell reports $rc, not the signal's 141: the model's answer that ends the process"
  rc=$( bash -c 'trap "" PIPE; mkfifo "$1"; exec 3<>"$1"; exec 4>"$1"; exec 3>&-; "$0" hi yo >&4; echo $?' "$OUT/up" "$OUT/pipe" 2>/dev/null ); rm -f "$OUT/pipe"
  checks=$((checks+1)); [ "$rc" = 74 ] || bad "writing to a pipe nobody reads with the signal ignored the file's status is $rc, the model's on a failed write 74"
  # A PIPE THAT FILLS. A pipe set not to block (`dd` sets the flag on the pipe it is given as
  # its output) takes the characters of a 100,000-byte argument one write each until it has no
  # room; the write after that fails. The file's status is a failed write's and the pipe holds
  # exactly the characters taken, each the one after the argument's.
  head -c 100000 /dev/zero | tr '\0' 'q' > "$OUT/big"
  res=$( bash -c 'mkfifo "$1"; exec 3<>"$1"; exec 4<"$1"; dd oflag=nonblock count=0 >&3 2>/dev/null; "$0" "$(cat "$2")" >&3; rc=$?; exec 3>&-; cat <&4 > "$3"; echo $rc' "$OUT/up" "$OUT/pipe" "$OUT/big" "$OUT/taken" 2>/dev/null ); rm -f "$OUT/pipe"
  n=$(stat -c %s "$OUT/taken" 2>/dev/null || echo 0)
  checks=$((checks+1)); [ "$res" = 74 ] || bad "after the pipe filled the file's status is $res, the model's on a failed write 74"
  checks=$((checks+1)); [ "$n" -gt 0 ] && [ "$n" -lt 100000 ] || bad "the pipe took $n of the 100000 characters: it did not fill"
  checks=$((checks+1)); head -c "$n" /dev/zero | tr '\0' 'r' | cmp -s - "$OUT/taken" || bad "the pipe does not hold $n times the character after the argument's"
fi

echo "$NAME: $checks checks, $fail failed"
exit $fail
