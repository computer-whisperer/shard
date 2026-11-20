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
| `x86.shard` | the registers `Regs`, the flags `Flags`, a mapped range `Seg`, the state `Cpu`; memory access (`load64`, `store64`); `fetch`; the instructions `Instr` and `decode`; the flags of each operation; `exec` and `step`; why a step is stuck, `Why` |
| `linux.shard` | an executable file read (`elf_load`), what the kernel leaves at a process's start (`Start`), its first state (`boot`), system calls (`on_syscall`), a run (`run`, `run_file`), and the machine's system `exits` |
| `asm.shard` | the assembler: `encode`, an instruction's bytes; a listing with labels, `Asm`; `assemble`, a listing's bytes, checked by the model's decoder; the fact `assemble_decodes` |
| `elf.shard` | an executable file written: `elf_exec code`, the smallest file Linux runs |
| `exit42.shard` | the first program, its file, and the theorems `exit42_run` and `exit42_exits`; a `main` that writes the file |
| `demo.shard` | a demonstration: three shard functions over machine words, a listing for them compiled by hand, and the theorem that the listing's file exits with the low byte of what the function computes |
| `run_main.shard` | the model from the command line: files run, code wrapped as an executable file |
| `laws.shard` | what is proven once of the model: a run of steps (`reaches`), a rule for each of fourteen instruction forms, what a store leaves as it was, a stored word read back, code unmoved by a store elsewhere, what a function's return keeps |
| `buffers.shard` | a trial of low-level code as shard (`docs/COMPILE.md` §4): three functions over buffers of machine words, a listing for them compiled by hand, the listing's file |
| `buffers_facts.shard` | the contracts of the code of two of those functions, proven from the laws |
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
| `jcc rel32` | `0F 80+cc cd` | a jump where the condition holds: `o no b ae e ne be a s ns l ge le g`; the two on parity are stuck |
| `jmp rel32`; `call rel32`; `ret` | `E9 cd`; `E8 cd`; `C3` | a jump; the return address pushed; popped |
| `push r64`; `pop r64` | `50+r`; `58+r` | a word to and from the stack |
| `syscall` | `0F 05` | control passes to the operating system |

A register-or-memory operand is a register; the eight bytes at a base register plus a
displacement of none, one or four bytes; or those at a base register plus an index register
times 1, 2, 4 or 8 plus such a displacement. `rsp` and `r12` as a base take a SIB byte, and so
does every operand with an index; `rsp` is no index. An address from the program counter, an
index with no base and an absolute address have no rule. A read needs eight bytes inside one range mapped to read, a write eight
inside one mapped to write.

**Linux.** `elf_load` reads a 64-bit little-endian executable for x86-64 at fixed addresses (type
2) whose program headers are all loadable segments: each segment's bytes are copied and its range
mapped with its permissions. It refuses a segment that reaches past the file, that is longer in
the file than in memory, whose address and place in the file differ within a page, that lies
below 65536 or ends above 2^47. A process starts from the image and a `Start`: the registers, the
flags, the stack's range and the memory under the image — whatever the kernel leaves. System call
60 (`exit`) ends the process with the low byte of `rdi`; any other is stuck.

**The system** (`docs/COMPILE.md` §2): `(exits file start status)` — there is a count of steps at
which the run of the file from that start has exited with the status. No statement names the
count.

## The assembler

`encode` gives an instruction its bytes: a 64-bit operation always with its prefix, an immediate
in one byte where it is the sign extension of one, a base register's displacement in none, one or
four bytes, an index register in a SIB byte. A listing (`Asm`) is instructions, labels, and jumps and calls to labels; a jump to a
label is always the form with four bytes of displacement, so every line's length is known before
any label's place. `assemble` resolves the labels, encodes, and then has the model's `decode`
read the bytes back: it answers only with bytes that decode, instruction by instruction and with
nothing left over, to the listing. An instruction with no encoding here — an immediate too wide —
or a label no line declares makes it answer nothing.

Nothing is proven of `encode`. The fact is made for each listing (`docs/COMPILE.md` §3, a
certificate): `assemble_decodes` — where `assemble p` is the bytes `bs`, the listing's
instructions decode from `bs` (`decodes`) — and `decodes_cons` opens that one instruction at a
time: the model decodes the first from the first fifteen bytes, with its encoding's length, and
the rest stand after those bytes.

## What is proven, and what is believed

`exit42_run`: the run of the first program's file — 132 bytes, written by `elf_exec` — exits with
42 after three steps, from every `Start`. K evaluates the model to accept it; it rests on
`propext`. `exit42_exits` states the same of the system. `w64`, `w64_add`, `w64_sub` and `w64_mul` are
proven to be Init's `BitVec.ofNat 64`, `BitVec.add`, `BitVec.sub` and `BitVec.mul`. The assembler's fact and its
lemmas (above) are accepted; a listing with a missing label and one with an immediate too wide are
proven refused, by evaluation.

`demo_run` (`demo.shard`): three functions written as ordinary shard over machine words — a sum,
a comparison under an `if`, calls — and a listing for them compiled by hand; the model's run of
the listing's file exits with the low byte of what the function computes in the logic. K
evaluates both sides. It is one program: there is no compiler, and the stack's place is fixed in
the statement.

**The laws** (`laws.shard`). `(reaches c c2)`: from the state `c` the processor comes to `c2` in
some count of steps, none a system call or stuck. A rule for an instruction form says: where the
program counter is at `pc`, the processor decodes that instruction there (`code_at`), and an
operand in memory is mapped, the state before the instruction reaches whatever the state after
it reaches; the state after is written out, with its program counter a numeral. There are
fourteen, for the forms the trial's listing uses: an operation between registers, `cmp`, an
operation with an immediate, `mov` between registers, a conditional jump taken and not taken,
`jmp`, a load and an `imul` through an indexed operand, `add` into an indexed operand, `push`,
`pop`, `call`, `ret`. Of memory: bytes stored leave every other address as it was
(`mem_put64_other`); a stored word is the word read back (`le64_put64_word`); two memories that
agree outside a range (`same_outside`) give the same word, the same fetched bytes and the same
decoded instructions clear of the range (`le64_same`, `fetch_same`, `code_in_same`).

**The trial** (`buffers.shard`, `buffers_facts.shard`; `docs/COMPILE.md` §4). Three functions
written as ordinary shard: `dot`, the sum of the products of two buffers' words; `addto`, the
first buffer with the second's words added to it — in the logic a new list; and `both`, which
calls the two, `dot` on one buffer as both operands. A buffer is a list of words; a read
(`word_at`) and a write (`put_at`) take the evidence that the index is below the list's length.
The listing is compiled by hand, an instruction or two for each operation of the source, with no
check of an index; `addto`'s loop writes into the buffer it was given. Proven:

- `dot_contract`: from a state at `dot`'s first instruction whose registers hold the addresses
  of two buffers and a count, with the buffers' words in memory and mapped to read, the processor
  comes to a state at the return address with `dot`'s value in `rax`, the stack pointer above the
  address, the callee's registers kept, and memory as it was. `dot_contract_same`: the same where
  the two addresses are one.
- `addto_contract`: from such a state, the first buffer mapped to write and its counted words
  clear of the code, of the second buffer and of the return address, the processor comes to a
  state at the return address in which the first buffer's memory holds the list `addto` returns,
  and every byte outside that buffer's counted words is as it was.

Each rests on Lean's three axioms and on nothing else; each assumes that the processor decodes
the listing's instructions at their addresses in the file (`code_in`), which is proven of no file.
The evaluator's run of `both` on the buffers 3 5 7 and 2 4 6, the model's run of the listing's
file and the processor's give one status, 19. Of `both`'s code nothing is proven: it calls, and
keeps two registers on the stack; the rules for `push`, `pop`, `call` and `mov` are used by no
contract yet. No compiler made the listing and no tool the proofs.

Believed: that the model is the processor and the kernel, where it has a rule. The evidence is
`kernel/test/x86_test.sh`. It runs the first program's file and the programs of `test/*.hex` —
sums and differences with their carries, a product, the fourteen conditions over the four flags,
sign extension at its edge, memory through each form of operand, the stack, a call — under the model
and, on an x86-64 Linux machine, on the processor: where the model's run exits, the processor's
exits with the same status. The status is one byte, so a program that checks a word reads it back
a byte at a time, as eight programs. Fourteen faults planted in the model one at a time each failed
the test as it stood before the product, the indexed operand and the unknown flags were added;
that was checked by hand, with a tool outside the tree, found two programs missing, and has not
been done again since.
The assembler's bytes for three of the programs, written as listings, are the bytes assembled by
hand.

Not here: any other instruction (a `nop` is stuck); operands of 8, 16 and 32 bits but the one
`mov`; a system call that returns; the bitwise three proven to be Init's; a theorem about
`encode` for every instruction; the step from an assembled listing in a loaded file to what the
processor then executes; a rule for every instruction form; the contract of code that calls; a
compiler from a shard function to a listing and its contract.

## From the command line

```
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)
X="v3/bin/load --root v3 --init $EXPORT --init-index $INDEX --run models.x86.run_main.main v3/models/x86/run_main.shard --"
$X wrap CODE FILE            # the bytes of CODE as the executable file FILE that runs them
$X run 10000 FILE…           # each file run for at most 10000 steps: a line each
```

A line is `exit N`; `stuck: no instruction of the model at ADDRESS: BYTES…`, `stuck: not mapped to
read at ADDRESS`, `stuck: not mapped to write at ADDRESS`, `stuck: no rule for a jump on condition
N`, `stuck: no rule for system call N`, `stuck: not a file the model loads`; or `out of steps at
ADDRESS`. The runner starts a process from zeroed registers, cleared flags and eight megabytes of
zeroed stack under `7FFFFFFFF000`.

A test program is a file of hexadecimal bytes with `;` comments (`test/stack.hex`). `KK` in it
stands for each of `F8`…`FF` in turn, eight programs. Its comments state what the model's run is:
`; statuses: N…`, one for each program, or `; model: LINE`; and may state the processor's:
`; processor: exit N`, `; processor: killed`.
