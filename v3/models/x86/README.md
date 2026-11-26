# `v3/models/x86` — the x86-64 processor under Linux, as far as a consumer has needed it

A model of the machine that the compile system's last step targets (`docs/COMPILE.md` §4). The
model is a program: K states theorems about it, it runs from the command line on a file, and the
same file run by the real processor is what it is held to. A program is bytes. Bytes the model
does not decode, an access outside what is mapped, a system call it has no rule for and a file it
does not load are *stuck*, with the reason: such a run has no outcome, and nothing is claimed of
it.

## The files

| file | what it holds |
|---|---|
| `word.shard` | the machine word, Init's `(BitVec 64)`: `w64` (a number mod 2^64 as a word), `w64_val` (its number), `w64_add`, `w64_sub`, `w64_mul`, `w64_and`, `w64_or`, `w64_xor`, `w64_msb`, the sign extensions `sx8`, `sx32`, and facts of a word's number |
| `words.shard` | a buffer of machine words as a value, a list: the word at an index (`nth`), the list with one replaced (`upd`), an access that takes the evidence that its index is in range (`word_at`, `put_at`) |
| `x86.shard` | the registers `Regs`, the flags `Flags`, a mapped range `Seg`, the state `Cpu`; memory access (`load64`, `store64`, and `load8`, a byte as a word); `fetch`; the instructions `Instr` and `decode`; the flags of each operation; `exec` and `step`; why a step is stuck, `Why` |
| `linux.shard` | an executable file read (`elf_load`), what the kernel leaves at a process's start (`Start`; `start_at`, a start with the stack's range and the stack pointer given; `stack_ok`, what the model takes a kernel to promise a file of them; `runner_start`, the one start the runner makes), what stands at the stack pointer then — a process's arguments (`args_at`) —, its first state (`boot`), what is outside a process (`Os`: the kernel's answers to come, each an `Answer`, and the bytes written to descriptor 1), system calls (`on_syscall`, `write_to`), a run (`run`, `run_file`), the machine's system (`ends`; `exits`, of a file that writes nothing), what a loop that writes all of some bytes comes to under the answers (`send`, `sends_end`), and what a call of a function comes to — it returns, or the process ends in it (`Outcome`) — with how a file ends by it where its start-up exits with the word returned (`exit_of`) |
| `asm.shard` | the assembler: `encode`, an instruction's bytes; a listing with labels, `Asm`, and instructions as its lines (`lines`); `assemble`, a listing's bytes, checked by the model's decoder; the fact `assemble_decodes` |
| `elf.shard` | an executable file written: `elf_exec code`, the smallest file Linux runs |
| `write.shard` | writing bytes out: the rules of the system call `write` by the kernel's next answer (`write_took`, `write_err`, `write_kill`), and `write_all` — fourteen instructions by hand that offer a range's bytes to descriptor 1 until all are taken — with its contract for any address, any range and every list of answers (`write_all_ok`); where it returns, rax holds 1 |
| `world.shard` | a program's World (`v3/std/world.shard`) against what is outside a process: the World of an `Os` (`os_world`) and of a process's start (`world_of`), with of each answer how the call went and nothing of the registers; that the model's loop is the platform's `write_all` (`send_put`); how a file ends by a program's value, a word and a World (`world_res`); and when the outcome of code is such a value, with the memory left (`tied`, `tied_exit`) |
| `bytes.shard` | the bytes of a word that stands in memory: the eight bytes from an address are the eight low bytes of the number read there (`le64_bytes`), which is below 2^64 (`le64_lt`); so where a number's word is read, its bytes are known (`held_bytes`) |
| `hello.shard` | a file that writes: a listing by hand that writes two pieces of text with `write_all` and exits with 0; what its process ends as, on a list of answers (`hello_res`) |
| `hello_run.shard` | that file from end to end: started with its stack anywhere and on any list of answers, it ends as `hello_res` says (`hello_ends`) |
| `hello_main.shard` | the file written; what the theorem says of lists of answers, printed |
| `exit42.shard` | the first program, its file, and the theorems `exit42_run` and `exit42_exits`; a `main` that writes the file |
| `demo.shard` | a demonstration: three shard functions over machine words, a listing for them compiled by hand, and the listing's file; a `main` that writes the file |
| `args.shard` | laws of a process's arguments: a memory that is the start's from the stack pointer up holds the same arguments (`args_at_same`); a string's bytes are the bytes of memory at its address (`str_bytes`); the range a function reads them through, from the stack pointer to the top of the stack's range (`args_rs`) |
| `run_main.shard` | the model from the command line: files run, a file run with arguments (`start_args`, the arguments laid out as Linux lays them), a file run on lists of the kernel's answers, a file run once for each case of answers and arguments (`cases`, the cases between arguments `/`), code wrapped as an executable file |
| `laws.shard` | what is proven once of the model: a run of steps (`reaches`), a rule for each of twenty-four instruction forms, what a store leaves as it was, a stored word read back, code unmoved by a store elsewhere, what a function's return keeps, what is mapped, a run to its end |
| `frame.shard` | words in memory and a function's stack frame: a list of words standing from an address (`holds`), and the words a memory has from an address read out as a list (`read_words`); room for a frame (`room`); a state inside a function described by the words that stand from its stack pointer up to the return address (`framed`), and what a push, a pop, a callee's return and the function's own return make of the description |
| `loaded.shard` | a listing's file, loaded: what `elf_load` makes of the file `elf_exec` writes, and the listing's code in place in the process's first state (`listing_loaded`) |
| `process.shard` | a loaded file's process with its stack anywhere: an access inside the stack's range is mapped (`stack_access`, `stack_words`); a state of the process told by where its stack pointer stands (`proc_at`) — the first state (`proc_first`), across a push (`proc_pushed`), across instructions that move registers (`same_stack`, `proc_same`), across a call that has returned (`proc_returned`) —; and the room a function is entered with (`proc_room`) |
| `part.shard` | a part of a table of instructions is in place where the whole is: `table_part`, a computation, and `code_in_part` |
| `derive.shard` | a tactic library: `derive_block`, the proof of a run of a listing's instructions by the laws' rules, read off the listing's table |
| `buffers.shard` | a trial of low-level code as shard (`docs/COMPILE.md` §4): three functions over buffers of machine words, a listing for them compiled by hand, the listing's file, its instructions by address (`buffers_table`); `dot`'s own lines and rows of them (`dot_asm`, `dot_rows`) |
| `buffers_tactic.shard` | the tactics the trial's lemmas name: `derive_block` over the trial's table (`block`), and over `dot`'s own rows (`block_dot`) |
| `buffers_facts.shard` | the contracts of the code of those three functions, proven from the laws |
| `buffers_run.shard` | that listing's file from end to end: started with its stack anywhere `stack_ok` allows, it exits with 19 (`buffers_exits`), by `both`'s contract |
| `demo_run.shard` | the demonstration's file from end to end: the contracts of `triple`'s code and of `pick`'s (`triple_contract`, `pick_contract`), and the theorem that the file, started with its stack anywhere, exits with the low byte of what `entry` computes (`demo_exits`) |
| `test/*.hex` | programs assembled by hand, each stating what the model's run of it is |
| `test/listings.shard` | three of those programs as listings for the assembler; listings it refuses |

## The machine

**State** (`Cpu`): sixteen 64-bit registers in the order of their numbers in an encoding, the
program counter, four flags — carry, zero, sign, overflow — a memory of bytes
(`v3/std/mem.shard`) and the ranges mapped, each with its permissions (4 read, 2 write, 1
execute). A flag is known or not: an instruction that leaves a flag undefined on the processor,
or of whose flags the model says nothing, leaves it unknown, and a jump that reads an unknown
flag is stuck. The parity and the auxiliary carry are not kept.

**A step** fetches up to fifteen bytes at the program counter — as many as are mapped for
execution — decodes one instruction and executes it. A REX prefix (`40`–`4F`) is read; no other
prefix has a rule.

| instruction | bytes | does |
|---|---|---|
| `mov r32, imm32` | `B8+r id` | the register is the immediate, its high half zero |
| `mov r64, imm64` | `REX.W B8+r io` | the register is the immediate |
| `mov r/m64, r64`; `mov r64, r/m64` | `REX.W 89 /r`; `REX.W 8B /r` | a word moved |
| `add`, `or`, `and`, `sub`, `xor`, `cmp` `r/m64, r64` | `REX.W 01 09 21 29 31 39 /r` | the operation, its flags; `cmp` writes none but the flags |
| the same, `r64, r/m64` | `REX.W 03 0B 23 2B 33 3B /r` | |
| the same, `r/m64, imm` | `REX.W 81 /n id`, `REX.W 83 /n ib` | the immediate sign-extended |
| `imul r64, r/m64` | `REX.W 0F AF /r` | the low word of the product; every flag left unknown (the processor defines the carry and the overflow) |
| `movzx r64, r/m8` | `REX.W 0F B6 /r` | the register is one byte as a word — a register's low byte, or the byte in memory —, the bytes above it zero; no flag changes |
| `jcc rel32` | `0F 80+cc cd` | a jump where the condition holds: `o no b ae e ne be a s ns l ge le g`; the two on parity are stuck |
| `jmp rel32`; `call rel32`; `ret` | `E9 cd`; `E8 cd`; `C3` | a jump; the return address pushed; popped |
| `push r64`; `pop r64` | `50+r`; `58+r` | a word to and from the stack |
| `syscall` | `0F 05` | control passes to the operating system |

A register-or-memory operand is a register; the eight bytes at a base register plus a
displacement of none, one or four bytes; or those at a base register plus an index register
times 1, 2, 4 or 8 plus such a displacement. `rsp` and `r12` as a base take a SIB byte, and so
does every operand with an index; `rsp` is no index. An address from the program counter, an
index with no base and an absolute address have no rule. A read needs eight bytes inside one range mapped to read — a byte's read the one byte —, a write eight
inside one mapped to write.

**Linux.** `elf_load` reads a 64-bit little-endian executable for x86-64 at fixed addresses (type
2) whose program headers are all loadable segments: each segment's bytes are copied and its range
mapped with its permissions. It refuses a segment that reaches past the file, that is longer in
the file than in memory, whose address and place in the file differ within a page, that lies
below 65536 or ends above 2^47. A process starts from the image and a `Start`: the registers, the
flags, the stack's range and the memory under the image — whatever the kernel leaves. System call
60 (`exit`) ends the process with the low byte of `rdi`; 1 is `write`; any other is stuck.

**What is outside a process.** A run takes, beside the processor's state, an `Os`: the answers
the kernel will give the process's calls, in order, and the bytes the process has written to
descriptor 1. The answers are an input of a run, as the arguments are: the model does not say
what a kernel answers, and a statement about a file is for every list of them. An answer says
how a call that offers bytes went (`How`, the platform's type: `v3/std/world.shard`) — `(HTook a)`: the kernel took a of them, at least one and at
most what was offered, all for a = 0; `(HErr e)`: it failed, and the call returns the negative
of an error number from 1 to 4095; `HKill`: it ended the process there, as it does by a signal
at a write to a pipe nobody reads — and what the call leaves in `rcx` and `r11`, which Linux
destroys at every system call. When the list is used up the kernel takes everything offered:
the empty list is a kernel that never takes less and never fails. No flag is known after a call
that returns. `write` has a rule for descriptor 1 and for bytes that are all mapped to read: the
bytes the answer takes, from the first, are written and their count returned. A write to
another descriptor, or from bytes not mapped, is stuck — Linux answers the second with an error;
the model does not say so, and a proven program does not ask. Nothing is read: no `read`, no
`open`, no descriptor but 1.

**A process's arguments.** `(args_at m sp hi args)` is what the model takes the kernel to leave
at the stack pointer `sp` of a start whose memory is `m` and whose stack's range ends at `hi`,
for the arguments `args`, a list of lists of bytes: the word at `sp` is their count; the words
after it are addresses, one for each argument; at each address stand that argument's bytes, none
of them 0, and a 0 after them; and the count, the addresses and the strings lie from `sp` up,
inside the stack's range. The first argument is by convention the name the program was run by;
the model does not say there is one. Nothing is said of the word after the last address, of the
environment, of the auxiliary vector, or of the stack pointer's alignment. A theorem about a file
that reads its arguments is for every `args` and every start that holds them
(`v3/compile/x86/argsum_run.shard`). A file's first instructions push below the stack pointer,
so the memory a function is entered with holds the same arguments (`args.shard`
`args_at_same`).

**The system** (`docs/COMPILE.md` §2): `(ends file start answers r)` — there is a count of steps
at which the run of the file from that start, on those answers, has ended with `r`: `(RExit
status out)`, the process exited with the status having written `out`, or `(RKill out)`, the
kernel ended it. No statement names the count. `(exits file start status)` is the statement of
a file that writes nothing: for EVERY list of answers it ends with `(RExit status nil)`.

**Writing all of some bytes.** `(send answers bytes)` is what a loop comes to that offers the
bytes until all are taken, each call taking one answer: `(SAll left)`, every byte taken and
these answers left; `(SFail written)`, an answer failed; `(SKill written)`, the kernel ended
the process. `(sends_end answers before pieces status)` is how a process ends that writes
pieces one after another so and then exits with the status: with the status and everything
written; with `status_io`, 74, where a call failed; or ended by the kernel — each with what was
taken before. A theorem about a file that writes is an equation in it (`hello_ends`).

**A program's World.** A program does not have an `Os`: it has a World (`v3/std/world.shard`),
its own conversation with the outside — of each answer how the call went, and nothing of
registers —, and the platform's `write_all` is a function on it. `world.shard` joins the two:
`send` on the kernel's answers is `write_all`'s loop on the World's (`send_put`), and the
outcome of code that was called — a return with what is outside as the code left it, or the
process's end (`Outcome`) — is a program's value, a word and a World, when it is `tied` to it.
A file whose function's outcome is tied to its program's value ends as `world_res` says of
that value. The compile system's files that write are stated so (`v3/compile/README.md`, "A
program that writes").

## The assembler

`encode` gives an instruction its bytes: a 64-bit operation always with its prefix, an immediate
in one byte where it is the sign extension of one, a base register's displacement in none, one or
four bytes, an index register in a SIB byte. A listing (`Asm`) is instructions, labels, and jumps and calls to labels; a jump to a
label is always the form with four bytes of displacement, so every line's length is known before
any label's place. `assemble` resolves the labels, encodes, and then has the model's `decode`
read the bytes back: it answers only with bytes that decode, instruction by instruction and with
nothing left over, to the listing, and each of which is a byte (below 256). An instruction with no encoding here — an immediate too wide —
or a label no line declares makes it answer nothing.

Nothing is proven of `encode`. The fact is made for each listing (`docs/COMPILE.md` §3, a
certificate): `assemble_decodes` — where `assemble p` is the bytes `bs`, the listing's
instructions decode from `bs` (`decodes`) and `bs` is bytes — and `decodes_cons` opens that one instruction at a
time: the model decodes the first from the first fifteen bytes, with its encoding's length, and
the rest stand after those bytes.

## What is proven, and what is believed

`exit42_run`: the run of the first program's file — 132 bytes, written by `elf_exec` — exits with
42 after three steps, from every `Start`. K evaluates the model to accept it; it rests on
`propext`. `exit42_exits` states the same of the system. `w64`, `w64_add`, `w64_sub` and `w64_mul` are
proven to be Init's `BitVec.ofNat 64`, `BitVec.add`, `BitVec.sub` and `BitVec.mul`. The assembler's fact and its
lemmas (above) are accepted; a listing with a missing label and one with an immediate too wide are
proven refused, by evaluation.

`demo_exits` (`demo_run.shard`): three functions written as ordinary shard over machine words —
a sum, a comparison under an `if`, calls — and a listing for them compiled by hand
(`demo.shard`); the listing's file, started with its stack anywhere `stack_ok` allows, exits
with the low byte of what the function computes in the logic. The model is not run on the file:
the code of `triple` and the code of `pick` each have a contract, for any arguments — entered
with room for a frame, the code returns with the function's value in rax; `pick`'s by the two
ways its comparison can go —, the first instructions call the two in states that meet the
contracts, and what the contracts leave in rax, less one, is `entry` as it is written. It rests
on Lean's three axioms. It is one program: there is no compiler.

**The laws** (`laws.shard`). `(reaches c c2)`: from the state `c` the processor comes to `c2` in
some count of steps, none a system call or stuck. A rule for an instruction form says: where the
program counter is at `pc`, the processor decodes that instruction there (`code_at`), and an
operand in memory is mapped, the state before the instruction reaches whatever the state after it
reaches; the state after is written out, with its program counter a numeral. There are twenty-four.
Fifteen are for the forms the trial's listing uses: an operation between registers, `cmp`, an
operation with an immediate, `mov` between registers, `mov` of an immediate, a conditional jump
taken and not taken, `jmp`, a load and an `imul` through an indexed operand, `add` into an indexed
operand, `push`, `pop`, `call`, `ret`. Seven are for forms the compile system's pass emits
(`v3/compile/x86/pass.shard`): `mov` of a 64-bit immediate, a load and a store through a base
register, the two through a base register and a displacement, a byte's load through a base
register, an `imul` of two registers. Two are for the demonstration's listing: an operation of a
register with the word at a base register, and `cmp` so. Of memory: bytes stored leave every other address as it was
(`mem_put64_other`); a stored word is the word read back (`le64_put64_word`); two memories that
agree outside a range (`same_outside`) give the same word, the same fetched bytes and the same
decoded instructions clear of the range (`le64_same`, `fetch_same`, `code_in_same`); two that
agree inside a range (`same_on`) give the same word inside it (`le64_on`). For code that
calls: the word at the stack pointer after a push is the word pushed (`ret_addr_pushed`); what a
callee's `returned` says, one part at a time (`returned_at`, `kept_sp`, `kept_rbx`, …); code in
place in any later state with the same mapping whose memory agrees outside a range clear of it
(`code_in_after`); agreement outside two ranges, a buffer and a frame (`same_outside2`), composed
of a caller's writes to its frame around a callee's to a buffer (`same_outside2_around`). Of what
is mapped: a range mapped whole is mapped word by word (`mapped_words_of`). Of a run's end: a run
from a state the processor comes to is a run from where it came, what is outside unchanged
(`run_of_steps`: no step of the processor touches it), and the system call `exit` ends the
process with the low byte of `rdi` and what it has written (`run_exit`). `(comes c o c2 o2)`:
the run from `c` with `o` outside goes on as the run from `c2` with `o2` — system calls that
return may lie between; it follows from `reaches` (`comes_of_reaches`) and from one step of a
run (`comes_of_move`), and is transitive. `(ends_at c o r)`: a run from there ends with `r`. Of
a range's bytes (`mem_bytes`): how many, the first k and the rest, and that a memory which
differs only in a range they lie outside of has the same (`mem_bytes_outside`).

**Writing** (`write.shard`). At a system call whose registers ask for a write to descriptor 1
of bytes mapped to read, the run's step is the kernel's next answer (`write_call`); the three
kinds each have their rule. `write_all` is fourteen instructions: while bytes are left it
offers them all, moves on by the count returned, and exits with 74 where the result is above
what it offered — a count never is, an error always. Its contract `write_all_ok`, for the code
at any address, any range and every list of answers, by induction on the answers: what the
code comes to is what `send` says — every byte written and the code returned in a state with
the same memory, the stack pointer above the return address and the registers a callee keeps;
or the process ended, the bytes taken before that written. It is used twice in `hello_ends`:
the answers the first call leaves are the second's.

**A function's frame** (`frame.shard`). `(holds m p ws)`: the words `ws` stand in the memory
`m` from the address `p`, eight bytes each — a buffer a function was handed, or the words a
function has pushed. `(room c clo chi f k)`, of the state a function is entered in: the stack
pointer stands `k` words above an address `f`; the `k` words from `f` are mapped to write, and
they and the return address above them to read; and they are clear of the code, which lies in
`[clo, chi)`. It is what a contract asks of its caller's stack, in one hypothesis.
`(framed c0 tbl clo chi f k m0 c j ws)`, of a state `c` inside a function entered in `c0` with
that room: the mapping is the entry's, the stack pointer stands `j` words above `f`, the words
`ws` stand from there up to the return address — the last of them — the table's code is in
place, and memory outside the `k` words is as in `m0`: the entry's memory, or the memory a
callee returned in. Each law takes one description to the next:

- `framed_enter`: the entry state is described, the one word at its stack pointer its return
  address;
- `framed_push`, `framed_push_ok`: after a push the word pushed stands first, a word down; and
  the word a push stores at is mapped — what the rule of a `push` or a `call` asks;
- `framed_pop`, `framed_top`, `framed_top_ok`: after a pop the rest stand, a word up; the word
  at the stack pointer is the first of the words — the word a `pop` gives, the address a `ret`
  jumps to — and is mapped to read;
- `framed_back`, `framed_back_same`, `framed_back_at`: a callee entered in a described state
  has returned: the state it returned in is described, a word up, where what it wrote lies in a
  range clear of the words and of the code, or where it wrote nothing; and the program counter
  is at the first of the words, the address the `call` pushed;
- `framed_returned`: where the stack pointer stands again at the return address and the six
  registers a callee keeps are the entry's, the state after `ret` is the function's code
  `returned`.

So the stack pointer's arithmetic, each saved word read back, the code still in place after a
push and the mapping of each word are proven once, and a function's proof names its words.
The laws are used by one function, `both`: a callee that keeps a frame of its own (its
footprint two ranges) and a frame that holds variables have no law yet.

**A listing's file, loaded** (`loaded.shard`). `elf_load_exec`: the file `elf_exec` writes for
any code that ends below the top of a process's addresses loads to one range, the file's bytes
from 400000 (hexadecimal), mapped to read and execute. `listing_loaded`: where `assemble`
answers `code` for a listing, in the first state of the process of that code's file the
processor decodes each instruction of the listing, its labels resolved, at its address
(`code_in` of `layout`, the instructions by address) — provided the byte after the image is not
the stack's to execute. It is proven once, from the assembler's own check: no instruction of a
file is decoded again to know it. `run_file_exec`: a run of the file is a run from that state.

**The trial** (`buffers.shard`, `words.shard`, `buffers_facts.shard`; `docs/COMPILE.md` §4). Three functions
written as ordinary shard: `dot`, the sum of the products of two buffers' words; `addto`, the
first buffer with the second's words added to it — in the logic a new list; and `both`, which
calls the two, `dot` on one buffer as both operands. A buffer is a list of words; a read
(`word_at`) and a write (`put_at`) take the evidence that the index is below the list's length,
and each function takes, beside its two buffers and their count, the evidence that each buffer
holds that many words: `both` hands `dot` the evidence that `addto`'s sum is as long as its
first buffer was (`addto_room`).
The listing is compiled by hand, an instruction or two for each operation of the source, with no
check of an index; `addto`'s loop writes into the buffer it was given. Proven:

- `dot_contract`: from a state at `dot`'s first instruction whose registers hold the addresses
  of two buffers and a count, with the buffers' words in memory and mapped to read, the processor
  comes to a state at the return address with `dot`'s value in `rax`, the stack pointer above the
  address, the callee's registers kept, and memory as it was. `dot_contract_same`: the same where
  the two addresses are one. It is stated over `dot`'s own ten rows (`dot_rows`), not over the
  trial's table, so it holds in any file whose table has those rows at those addresses
  (`part.shard` `code_in_part`); `both` uses it by that (`dot_rows_in`). From it the compile
  system proves that these instructions meet its statement at a module's seam, and links a
  compiled caller against them (`v3/compile/x86/dot_hand.shard`, `v3/compile/README.md`).
- `addto_contract`: from such a state, the first buffer mapped to write and its counted words
  clear of the code, of the second buffer and of the return address, the processor comes to a
  state at the return address in which the first buffer's memory holds the list `addto` returns,
  and every byte outside that buffer's counted words is as it was.
- `both_contract`: from a state at `both`'s first instruction with the two buffers as for
  `addto` and room for a frame of three words above an address `f` (`room`), the frame and the
  return address clear of both buffers, the processor comes to a state at the return address
  with `both`'s value in `rax`, the first buffer's memory holding the list `both` returns,
  `rbx` and `r12` as the caller had them, and every byte outside the first buffer's counted
  words and the 24 bytes from `f` as it was. Its proof takes `addto_contract` and
  `dot_contract_same` as they are stated, each in the state the code calls from: what a callee
  is said to keep — the stack pointer, the mapping, six registers — and what it is said to
  leave unwritten is all the caller's proof has of the state the callee returns in. Its frame
  is the laws of `frame.shard`: the proof names the words that stand from the stack pointer at
  each state — the return address; under it `rbx`; under that `r12`; under that a call's
  return address — and each push, pop and return is one law. It is five theorems beside its
  three runs of instructions, and not one theorem: K's recursion is bounded in depth
  (`OPEN.md` §3), so each part is a lemma whose statement lists its hypotheses again.

- `buffers_exits` (`buffers_run.shard`): the listing's file, started with its stack anywhere
  `stack_ok` allows — the stack's range beginning at or above 4194661, past the table and the
  fifteen bytes the processor reads from an instruction's address, with ten words under the
  stack pointer; whatever the registers but the stack pointer, the flags and the memory then
  hold — exits with 19, the low byte of `both` of the buffers 3 5 7 and 2 4 6. The proof does
  not run the model: the code is in place by `listing_loaded`; the seventeen instructions
  before `both` are taken each by its rule, in five stages — the first buffer's three pushes,
  its address kept, the second's pushes, the arguments, the call —, each a theorem about a
  state of the process (`process.shard` `proc_at`) that names the state after the stage and
  does not open the one before; the state they end in meets each of `both_contract`'s nineteen
  hypotheses by where its ten words lie in the stack's range, no address a number; the
  contract gives the state at the return, and from any state it allows three instructions more
  end the process. That the functions' value is 19 is the logic's own computation of `both`.
  Between its first instructions and the functions the listing has 58 bytes that are never
  run: the functions then stand from 4194547, where the lines after the start-up of the compile
  system's sample files stand (`v3/compile/x86/startup.shard`), so that such a file has `dot`'s
  rows as they are.

Each rests on Lean's three axioms and on nothing else. A contract assumes that the processor
decodes the listing's instructions at their addresses (`code_in`); `listing_loaded` proves that
of the first state, and `buffers_exits` is the one statement here that assumes nothing of the
code. It is for a stack anywhere, as `demo_exits` is and as the compile system's file theorems
are (`v3/compile/x86/link.shard`): the statement is made with `start_at` and `stack_ok`
(`linux.shard`) — the stack's range and the stack pointer unknown, the range above a bound the
statement gives, room under the pointer for the words the file needs. K is slow where a proof
step has it compare, as wholes, the states of two instructions far apart in a run — seconds a
step at twenty instructions' distance —; a stage's theorem is therefore about ANY state before
it, and a later proof rewrites by the stage's facts. The evaluator's
run of `both`, the model's run of the listing's file and the processor's give that one status,
19. No compiler made the listing. The proofs are written by hand, but for nine: the runs of
instructions between a function's branch points, which a tactic derives.

**Derived proofs** (`derive.shard`; `docs/LANGUAGE.md`, "A tactic library"). A statement that
the processor comes from one state to another through a run of the listing's instructions —
`(reaches A B)`, A at a known program counter — is proven by one rule of `laws.shard` for each
instruction, in order. `derive_block` writes that proof: it walks the listing's table from A's
program counter, and for each instruction answers the rule with its numbers, the table's row,
and what else the rule asks — an operand mapped, the outcome of the comparison before a `jae`,
the code still in place after a store — taken from the statement's hypotheses by their shape.
It follows a jump, and ends where B's program counter is reached, after a `ret`, after a `call`,
or after a number of instructions a hint gives. A function of the listing's own hands it the
table (`buffers_tactic.shard` `block`; `block_dot` hands it `dot`'s own rows), and a theorem
names that function: `(auto (tactic models.x86.buffers_tactic.block))`. Nine theorems of the trial are proven so — `dot_turn`,
`dot_leave`, `dot_enter`, `add_turn`, `add_leave`, `add_enter`, `both_enter`, `both_mid`,
`both_leave` — their blocks in `buffers_facts.auto.shard`, which `prove` writes and a load
replays. No file of the
trial imports the library, so a load does not read it; `prove` is given the tactic's file first:

```
v3/bin/prove --root v3 --init $EXPORT --init-index $INDEX \
  v3/models/x86/buffers_tactic.shard v3/models/x86/buffers_facts.shard
```

`x86_derive_test.sh` holds that the library still derives the nine: it runs `prove` on a copy of
the model without the blocks, and the file written is the committed one byte for byte.

Nothing of the library is believed: K checks each block it answers, and a goal it does not
understand it leaves unanswered. It writes a proof and no statement: the states a run passes
through and the lemma for each run are still written by hand, and so is everything about a
loop and a call's hypotheses. What a run asks of the stack — each push's word mapped, the code
in place after it — is a hypothesis of the run's lemma, which the frame's description gives.

Believed: that the model is the processor and the kernel, where it has a rule. The evidence is
`kernel/test/x86_test.sh` (the assembler and the demonstration are `x86_asm_test.sh`'s, the trial
`x86_trial_test.sh`'s, a file that writes `x86_write_test.sh`'s). It runs the first program's file and the programs of `test/*.hex` —
sums and differences with their carries, a product, the fourteen conditions over the four flags,
sign extension at its edge, memory through each form of operand, the stack, a call — under the model
and, on an x86-64 Linux machine, on the processor: where the model's run exits, the processor's
exits with the same status. The status is one byte, so a program that checks a word reads it back
a byte at a time, as eight programs. Twenty faults planted one at a time each failed one of the
model's four tests at least: thirteen in the model's rules (each fails `x86_test.sh`, the
comparison with the processor), two in how a file is laid out and loaded, one in the assembler,
two in the trial (a function, its listing's table), one in the tactic library (only
`x86_derive_test.sh` fails) and one in a stored proof. A change to the model leaves every stored
proof stale, which alone fails the trial's test, so the stored proofs were derived again under
each fault. That was checked with a tool outside the tree.
The assembler's bytes for three of the programs, written as listings, are the bytes assembled by
hand.

Of what is outside a process the evidence is `x86_write_test.sh`, on the file `hello.shard`
describes, 236 bytes: for twelve lists of answers — everything taken at once, one byte a call,
more than was offered, a failure before any byte and between the pieces, the process ended —
the right side of the file's theorem, evaluated, and the model's run are one line; and the
processor's runs are what three of the model's kinds of answer say: the text and status 0; the
status 74 with the output closed, to `/dev/full`, and to a pipe nobody reads where the signal
is ignored; the process ended by the kernel at such a pipe otherwise. That a kernel takes fewer
bytes than it was offered is not made to happen on the processor: only the model's runs and
the theorem cover it. Two faults planted, each refused by K: the loop moving its address back
by the count instead of on (at the turn's statement), and the file asking for five bytes of
its second piece (at the file's table). The same runs are made of a file of COMPILED code
that writes, by `compile_greet_test.sh`.

Not here: any other instruction (a `nop` is stuck); operands of 8, 16 and 32 bits but the one
`mov` and a byte's read; a system call but `exit` and `write`, a descriptor but 1, anything
read; a line of the assembler for bytes that are no instruction (the text a file writes stands
in it as the immediate numbers of instructions it never comes to); the bitwise three proven to
be Init's; a
theorem about `encode` for every instruction; what stands above a process's arguments when it
starts (the environment, the auxiliary vector), and the stack pointer's alignment; a rule for
every instruction form; a law for the return of a callee that keeps a frame of its own; a
derivation of
a whole contract (a loop's invariant and a callee's hypotheses are by hand, and the frame's
laws are applied by hand); a compiler from a shard function to a listing and its contract.

## From the command line

```
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)
X="v3/bin/load --root v3 --init $EXPORT --init-index $INDEX --run models.x86.run_main.main v3/models/x86/run_main.shard --"
$X wrap CODE FILE            # the bytes of CODE as the executable file FILE that runs them
$X run 10000 FILE…           # each file run for at most 10000 steps: a line each
$X args 10000 FILE NAME ARG… # the file run with the arguments NAME ARG…, the first its name
$X answers 10000 FILE SPEC…  # the file run once on each list SPEC of the kernel's answers
$X cases 10000 FILE SPEC NAME ARG… / SPEC NAME ARG… …  # once for each case: answers, then arguments
```

A list of answers is items between commas: `N`, the kernel takes N bytes (all for 0); `eN`, it
fails with the error N; `k`, it ends the process. `run` and `args` give no answers: the kernel
takes all it is offered.

A line is `exit N`; `killed`, where the kernel ended the process — either of the two followed by
`, wrote` and the bytes written, in hexadecimal, where there are any —; `stuck: no instruction of
the model at ADDRESS: BYTES…`, `stuck: not mapped to
read at ADDRESS`, `stuck: not mapped to write at ADDRESS`, `stuck: no rule for a jump on condition
N`, `stuck: no rule for system call N`, `stuck: not a file the model loads`; or `out of steps at
ADDRESS`. The runner starts a process from zeroed registers, cleared flags and eight megabytes of
zeroed stack under `7FFFFFFFF000`. With `args` the arguments stand in that stack as Linux lays
them: the strings at the top, under them — the stack pointer at a multiple of sixteen — the
count, the strings' addresses, and two words of 0, the end of the arguments and of an empty
environment.

A test program is a file of hexadecimal bytes with `;` comments (`test/stack.hex`). `KK` in it
stands for each of `F8`…`FF` in turn, eight programs. Its comments state what the model's run is:
`; statuses: N…`, one for each program, or `; model: LINE`; and may state the processor's:
`; processor: exit N`, `; processor: killed`.
