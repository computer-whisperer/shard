# `v3/models/x86` — the x86-64 processor under Linux, as far as a consumer has needed it

A model of the machine that the compile system's last step targets (`docs/COMPILE.md` §4). The
model is a program: K states theorems about it, it runs from the command line on a file, and the
same file run by the real processor is what it is held to. A program is bytes. Bytes the model
does not decode, a system call it has no rule for and a file it does not load are *stuck*, with
the reason: such a run has no outcome, and nothing is claimed of it.

## The files

| file | what it holds |
|---|---|
| `word.shard` | the machine word, Init's `(BitVec 64)`: `w64` (a number mod 2^64 as a word, Init's `BitVec.ofNat 64`) and `w64_val` (its number) |
| `x86.shard` | the registers `Regs`, the flags `Flags`, a mapped range `Seg`, the state `Cpu`; `fetch`; the instructions `Instr` and `decode`; `exec` and `step`; why a step is stuck, `Why` |
| `linux.shard` | an executable file read (`elf_load`), a process's first state (`boot`), system calls (`on_syscall`), a run (`run`, `run_file`), and the machine's system `exits` |
| `elf.shard` | an executable file written: `elf_exec code`, the smallest file Linux runs |
| `exit42.shard` | the first program, its file, and the theorems `exit42_run` and `exit42_exits`; a `main` that writes the file |
| `run_main.shard` | the model run on a file from the command line |

## The machine

**State** (`Cpu`): sixteen 64-bit registers in the order of their numbers in an encoding, the
program counter, four flags (carry, zero, sign, overflow), a memory of bytes (`v3/std/mem.shard`)
and the ranges mapped, each with its permissions (4 read, 2 write, 1 execute).

**A step** fetches up to fifteen bytes at the program counter — as many as are mapped for
execution — decodes one instruction and executes it.

| instruction | bytes | does |
|---|---|---|
| `mov r32, imm32` | `B8+r id` | the register is the immediate, its high half zero |
| `syscall` | `0F 05` | control passes to the operating system |

**Linux.** `elf_load` reads a 64-bit little-endian executable for x86-64 at fixed addresses (type
2) whose program headers are all loadable segments: each segment's bytes are copied and its range
mapped with its permissions. It refuses a segment that reaches past the file, that is longer in
the file than in memory, whose address and place in the file differ within a page, or that lies
below 65536. System call 60 (`exit`) ends the process with the low half of `rdi`; any other is
stuck.

**The system** (`docs/COMPILE.md` §2): `(exits file regs flags status)` — there is a count of
steps at which the run of the file, from the registers and flags the kernel leaves, has exited
with the status. No statement names the count.

## What is proven, and what is believed

`exit42_run`: the run of the first program's file — 132 bytes, written by `elf_exec` — exits with
42 after three steps, from any registers and flags. K evaluates the model to accept it; it rests
on `propext`. `exit42_exits` states the same of the system.

Believed: that the model is the processor and the kernel, where it has a rule. The evidence is
`kernel/test/x86_test.sh`, which runs the file on the machine it is run on and compares the
exit status with the model's.

Not here yet: any instruction but the two, memory access, a stack, a system call that returns;
the assembler from instructions to bytes with its fact; the link to a shard program.

## From the command line

```
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)
L="v3/bin/load --root v3 --init $EXPORT --init-index $INDEX"
$L --run models.x86.exit42.main v3/models/x86/exit42.shard -- /path/exit42      # write the file
$L --run models.x86.run_main.main v3/models/x86/run_main.shard -- /path/exit42 100
```

The second prints `exit 42`; `stuck: no instruction of the model at ADDRESS: BYTES…`, `stuck: no
rule for system call N`, `stuck: not a file the model loads`; or `out of steps at ADDRESS`.
