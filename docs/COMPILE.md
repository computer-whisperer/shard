# The compile system: the statement

Status: the owner agreed to it as the statement to build to (`LAW.md` §6, 18). Of it the tree
holds a machine's model and a trial of low-level code (§4, `v3/models/x86`); no compiler. It fixes
one statement and the two ends of the first chain, and leaves the middle open on purpose (§7).

## 1. What it is for

A shard program is to be something programs transform: differentiated, fused, given another
representation, lowered to a machine. K checks each result, so a transformation may be written
quickly, by anyone, and be wrong without consequence. A native binary is the last such step on one
target, not the purpose. The owner: "the verified compute project is wanted much less for 'here is
an x86 program that we have verified has no malicious behavior or buffer overflows', but for the
greater vision of what it means to have a program language that can be manipulated and compiled in
a symbolic fashion."

## 2. The fact

Every transformation, a compile step included, yields one kind of theorem.

A **system** is a relation between inputs and outcomes: started on input `i` it may end with
outcome `o`. A function is a system (on `x` it ends with `f x`). So is a program under an
evaluator, and so is machine code on a machine. A run that never ends, or that stops where the
evaluator has no rule, has no outcome.

```
(def refined_by (p q ri ro) Prop           ; p the specification, q the implementation
  (forall (i j o) (-> (ri i j) (p i o) (exists (o2) (and (q j o2) (ro o o2))))))
```

In words: whenever the specification ends, the implementation, started on any related input, also
ends, and with a related outcome. `ri` relates inputs and `ro` outcomes. They carry everything
particular to a step:

- **representation**: how a value of the specification sits in the implementation's state — a list
  as a chain of cells, or as a flat buffer;
- **preconditions**: an input the step does not cover is related to nothing;
- **events**: what the outside observes (host calls, system calls), as part of an outcome;
- **failures**: an implementation may end early in a failure it declares — memory, stack, an
  integer too large.

What the form settles: the implementation ends (there is no "if it returns, then"); where the
specification has no outcome nothing is claimed; no step count appears in a statement — an
evaluator's fuel is quantified away inside its system.

Three laws, each a theorem:

- **Composition.** `p` refined by `q` and `q` refined by `s` give `p` refined by `s`, under the
  two pairs of relations one after the other.
- **Failures.** Two steps that may each stop early compose to one that may stop early, with the
  failures the last step declares: an earlier step's failure arrives as one of those.
- **Functions.** Between two functions the fact is `∀ x x', ri x x' → ro (f x) (g x')`, the usual
  statement of a change of representation.

K accepts the definitions and the three laws, their binders written out, on no axiom. That was
checked once by hand; no test in the tree holds it.

## 3. Two ways to a fact, one record

- **A pass proven once.** A function `t` from code to code, with a theorem: whenever `t c` is `d`,
  the run of `c` is refined by the run of `d`. For one program the fact then needs `t c = d`. K
  states that where it can compute it. Where it cannot — a large program — an engine runs `t`, and
  the record names the run.
- **A certificate.** Any tool, trusted or not, produces `d` and a proof that `c` is refined by `d`,
  and K checks the proof. This is the route for a transformation whose author proves nothing in
  general. It is cheap where the program is small and its data large: K reads the program, never
  the data.

Both give the same theorem, so both stand in one chain.

An **artifact** is its bytes or its program, the fact about them, and a **record**: each step's two
ends and relations, how its fact was obtained, and what the whole rests on — axioms, models
believed, engine runs (`LAW.md` rule 20, for artifacts).

## 4. Machines

A machine is a library, and K knows none. Its model is a program: one text that theorems are
stated about, that runs — natively, for tests and for search — and that is held to the real
machine by running both on the same input. Machine code is data in the language, so the tools that
search and transform programs reach it as they reach any value.

**The floor is what the processor does.** For x86-64: the registers, the stack pointer and the
program counter among them, the flags, and a memory of bytes. One step reads the bytes at the
program counter, decodes one instruction and executes it. A program is bytes, so every program has
a representation. The model covers the instructions a consumer has needed and no others. Bytes it
does not decode, and an access outside what is mapped, have no rule: a run that reaches one has no
outcome, and no fact can be stated of it. A partial model is sound for that reason, and it grows
by an instruction when a consumer needs one. A machine word is one of Init's bit vectors, so its
range is its type and Init's lemmas apply to it.

**Notations stand above the floor: several, none universal.** A list of instructions with labels,
a structured form, low-level shard. Each is tied to the bytes by a step with its fact; for the
first, an assembler: labels to offsets, instructions to bytes, and the decoding of an encoding
giving back the instruction. Code that fits no notation is given as bytes, with a proof of its own.

**K reasons about a model in general; engines run it in particular.** The route to a fact about
machine code is §3's, a pass proven once or a certificate. Stepping the model through one program
inside K is for a fragment of a few instructions, a system call's stub.

For each piece of code a machine supplies a system (§2): the inputs — the outside's answers among
them, so that a run has at most one outcome — and the outcomes with their events; and a rule for
linking: code that calls a function may be joined with anything the callee is refined by.

| machine | its code | today |
|---|---|---|
| the logic | a function | a `fn`'s definition |
| shard run | a program as data (`kernel/prog.shard`) under `ev.shard`, each host call answered from the input | `ev`'s machine is not yet a definition (`OPEN.md` §2); the loop over host calls is not written |
| x86-64 under Linux | the bytes of an executable file | `v3/models/x86`, begun: fifteen instructions, laws for them, three contracts, one file proven by them to exit as its functions compute |
| others | a shader language, another processor | when a consumer comes; its floor may be a stated model above the bytes |

No general form of a machine is designed before a second, different machine exists.

**Low-level code** — machine words, buffers, loops — is a subset of shard and not a second
language (the owner's ruling after a trial, `LAW.md` §6, 19). The evaluator, K and every tool work
at every level, and hand-written fast code is ordinary shard. The owner's requirement of it: the
specificity and the performance of C or Rust, no runtime and no evaluator in what runs, and facts
about a piece of machine code — what it does not write — stated and proven. Its form, as the trial
has it (`v3/models/x86/buffers.shard`):

- A function over machine words is the processor's own arithmetic: the model executes an
  instruction by the function the source calls.
- A buffer is a value. In the logic it is its contents, a list of words, and no address appears
  in what a function means. A read or a write takes the evidence that its index is in range — a
  proof, erased like every proof — so the function is total and its code checks no index.
- A function that returns a changed buffer may be compiled to code that writes into the buffer
  it was given. Nothing in the language says a buffer is used once: the code's contract is
  proven or it is not, and where the old contents are still needed it is not.
- What is proven of a piece of code is its *contract*: from a state that holds the function's
  arguments the processor comes to a state that holds its value — in a register, or in the
  buffer's memory — with what the code has left as it was: memory outside a stated range, the
  registers a caller keeps. A contract is §2's fact, its specification "end where you began" (K
  accepts both directions; checked once by hand, held by no test).

Proven there, by hand, from laws of the model stated once (`laws.shard`): the contracts of a sum
of products over two buffers, which writes nothing; of a sum into the first of two buffers,
which changes no byte outside that buffer's words; and of a function that calls the two and
saves two registers on the stack, which changes no byte outside that buffer's words and three
words of its stack. The caller's proof takes each callee's contract as it is stated. A contract
assumes its code in place; that is proven once, of the first state of the file of any listing
the assembler answers for (`loaded.shard`), and with it the system's statement of the trial's
file: started as the model's runner starts it, the file exits with the low byte of what the
functions compute (`buffers_run.shard` `buffers_exits`) — every hypothesis of the caller's
contract met in the state the file's first instructions reach. The proof of a run of
instructions between two branch points is derived from the listing by a tactic library
(`derive.shard`), a certificate in §3's sense; everything else is written by hand. Not shown:
a compiler that derives a contract; a frame that holds variables of its own; a start whose
stack is elsewhere; and anything for
code not of this kind — lists, constructors, integers of any size.

## 5. The first instance: a binary

For `(bin NAME (entry main) …)` and the bytes `b` of an executable file:

> The run of `main`'s program is refined by the run of `b` on x86-64 under Linux. On the same
> arguments and the same answers from outside, `b` makes the system calls that realize `main`'s
> host calls, in order, and exits with its code; or it stops early with a declared failure and a
> reserved exit code, having made a prefix of them.

A `bin`'s requirements are theorems about `main`. None is restated for the binary: each holds of
what the binary did, up to the point of a declared failure.

Only the two ends are fixed. The chain between them is assembled from facts: definition → program
(the erasure, believed today) → passes → instructions → bytes.

## 6. Trust, before and after

| | today | for a binary with this fact |
|---|---|---|
| K, Lean's theory, the three axioms | believed | the same |
| definition → program (the erasure) | believed | the same; later a fact per function |
| the lowering, the C generator, `rt.h`, `cc`, libc | believed, compared by test | gone |
| the machine's model and the Linux boundary | none | believed; tested against the processor and the kernel |
| an engine's run of a proven pass | none | believed where K did not compute it, named in the record, repeatable under `ev.shard`; the engines are today's, C-built ones included, until the compiler compiles itself |
| host-call axioms | believed, per `bin` | the same at first; the aim is to derive them from the boundary's model |

## 7. The first build, and what stays open

Two programs go through every layer before any layer is widened:

1. integers that fit a machine word, calls, `if`, an exit code computed;
2. a sum of products over two flat buffers, stated first over lists: one change of representation
   with its fact, then the same chain.

Done means: each binary runs on the owner's machine; K accepts its fact, and the record lists what
it rests on; the build is a shard program that reads the `bin` form; and a fault planted at each
joint — a wrong byte, a wrong instruction, a pass that drops a branch, an undeclared failure — is
refused. After that the chain widens by what `calc`, `shardfmt` and the loader refuse; a refusal
is a defect of the compiler.

Open on purpose: how integers of any size, constructors and reclaimed memory are represented
(§4's form of low-level code does not reach them); the calling convention and the linking rule's
exact form; events and their prefixes at the Linux boundary; registers; floats; a second target;
how results are stored and reused between builds; names and directories.

Also open: a program that does not run to an end — a server, a desktop application. The fact does
not ask that a program be refined whole. Its parts carry their own facts, and the loop around them
may be built by other means (the owner's words: `LAW.md` §6, 18).

## 8. What the owner agreed to

1. §2's fact is the one statement every transformation and every compile step yields.
2. An engine's run may stand where K cannot compute a proven pass's result, named in the record
   (§3, §6).
3. The first trust removed is the back end; the erasure stays believed until a later step.
4. A binary may stop early with a declared failure, and no requirement is restated for it (§5).
5. Low-level code is ordinary shard, in §4's form: a buffer a value, evidence at each access, a
   piece of code's contract with what it leaves unwritten (`LAW.md` §6, 19).
