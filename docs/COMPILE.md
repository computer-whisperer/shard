# The compile system: the statement

Status: the owner agreed to it as the statement to build to (`LAW.md` §6, 18), and to §5 as the
form a function has between the two ends (`LAW.md` §6, 20). Of it the tree holds a machine's model
and a trial of low-level code (§4, `v3/models/x86`); no compiler, and nothing of §5. It fixes one
statement, the two ends of the first chain and what crosses from one module to another, and leaves
the rest of the middle open on purpose (§8).

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
| shard run | a program as data (`kernel/prog.shard`) under `ev.shard`, each host call answered from the input | `ev`'s machine is a definition, by its fuel (`ev_term`); the loop over host calls is not written as one: the toolchain's own (`run_loop`) asks the host and stays a program (`OPEN.md` §2) |
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
words of its stack. The caller's proof takes each callee's contract as it is stated, and its
frame is laws stated once (`frame.shard`): a state inside a function is described by the words
that stand from its stack pointer up to the return address, and a push, a pop and a return
each take one description to the next. A contract
assumes its code in place; that is proven once, of the first state of the file of any listing
the assembler answers for (`loaded.shard`), and with it the system's statement of the trial's
file: started as the model's runner starts it, the file exits with the low byte of what the
functions compute (`buffers_run.shard` `buffers_exits`) — every hypothesis of the caller's
contract met in the state the file's first instructions reach. The proof of a run of
instructions between two branch points is derived from the listing by a tactic library
(`derive.shard`), a certificate in §3's sense; everything else is written by hand. Not shown:
a compiler that derives a contract; a frame that holds variables of its own; a caller of a
function that keeps a frame; a start whose
stack is elsewhere; and anything for
code not of this kind — lists, constructors, integers of any size.

## 5. A function at a module's seam

The owner agreed to this form after it was tried outside the tree (`LAW.md` §6, 20). The tree
holds none of it. What the trial proved and what it cost is in `DECISIONS.md` ("Measured").

A module is the unit that is compiled, and a function is what crosses from one module to another.

**Three levels, all shard.** Each function a module exports has:

1. a *meaning*: the function as its author writes it. A buffer is a list, and no address appears
   (§4);
2. a *low form*: a shard function over machine words and a memory of bytes. Addresses appear, and
   each read and each write carries the evidence that it lies in the ranges the function was given;
3. *code*: bytes on a machine.

**Two facts.** From the meaning to the low form the fact is between two functions, a change of
representation (§2's third law). What is particular to a piece of low-level code is proven here
— a loop's invariant, that two buffers lie apart —, once, whatever code the low form is given.
From the low form to the code the fact is about the machine: the pass's theorem where the code is
compiled, a proof on the machine's model where it is written by hand. Another implementation
proves the second fact again and leaves the first. A property is proven of the meaning or of the
low form, never of code.

**One statement at the seam.** A caller is given one statement about a function's code, the same
whatever produced the code. It is stated with the function's *answer*: a relation from the
argument words and a memory to the result word and the memory after.

> The code at this entry is called by the convention, with room for k words of stack. Take any
> memory that the machine's memory agrees with on the data ranges. Whenever the answer, on the
> arguments and that memory, is a word and a memory: the code returns with that word; the
> machine's memory agrees with the answer's memory on the data ranges; every byte outside them
> and outside the k words is as it was; and the registers a caller keeps are as they were.

This is §4's contract in one form for every function. What is assumed of a function that is
called and what is proven of a function are the same statement, so a function that calls others
is proven from the statement of each callee and from nothing else about it. The answer is taken
on any memory the machine's agrees with, and not on the machine's own: a compiled caller knows
its callee's answer on the memory of its own run, and the machine's memory differs from that one
in the caller's stack frame.

A compiled function's answer is its low form. A hand-written function's answer may be stated
from its meaning, under the way memory holds its arguments; a caller that is compiled, or written
in shard, needs the answer as a function.

**The intermediate is inside the second fact.** Between a low form and its code stands a language
a little above the machine and common to targets: local words, the word operations, loads and
stores, `if`, calls, and at the end of every path a return or a tail call. It has no loop. A loop
is a function that calls itself last, in a low form and in the intermediate alike, so the two
have one shape. A call takes the callee's answer and runs no body, so a callee may be code that
has no intermediate form. By default nobody writes intermediate code and nobody proves on it.
Two steps cross it:

- the *front step*, from a low form to intermediate code, with a certificate for each function
  (§3): the code's answer is the low form's value. The proof follows the code's shape — induction
  on the low form's own measure, a case at each test, a rule for each statement, the low form's
  own descent at the tail call —, so a tool writes it;
- the *pass*, from intermediate code to a listing, proven once (§3) and for a module at a time:
  if each function the module calls from outside meets the seam's statement for its answer, each
  function of the compiled module meets it for its own.

**Linking.** A file is a start-up and listings one after another. Proven once: a file that holds,
among any other lines, a function that meets the seam's statement exits as that function's answer
says. What is particular to one file — that the pass answers this listing, that the listing lies
at these addresses — is closed computation, which K runs. Hand-written code and compiled code
meet through the seam's statement and through nothing else.

**What a tool writes.** A compile step writes shard: intermediate code, the front step's proofs,
the lemmas those proofs need. One rule holds for it (`LAW.md` §6, 21): a source file states
everything that is claimed, and nothing an author writes names anything in a file a tool wrote
beside it. To delete such a file loses proofs and no statement's meaning; what stands in it needs
no trust, K checking each piece, and no lasting name. A statement that an author's text does cite
— a function's equations, the seam's statement of a function — comes from a form in the source
with a fixed shape and name, as a `fn`'s `F.eq_N` and a `derive`'s functions do. Not decided:
whether what a tool writes is kept in a file or made again by each build; the name under which a
seam's statement cites generated code; how a tool is handed a definition to read (§8).

## 6. The first instance: a binary

For `(bin NAME (entry main) …)` and the bytes `b` of an executable file:

> The run of `main`'s program is refined by the run of `b` on x86-64 under Linux. On the same
> arguments and the same answers from outside, `b` makes the system calls that realize `main`'s
> host calls, in order, and exits with its code; or it stops early with a declared failure and a
> reserved exit code, having made a prefix of them.

A `bin`'s requirements are theorems about `main`. None is restated for the binary: each holds of
what the binary did, up to the point of a declared failure.

Only the two ends are fixed. The chain between them is assembled from facts: definition → program
(the erasure, believed today) → passes → instructions → bytes.

§5's chain for one function starts at its definition and passes through no program and no
erasure. It reaches a function that returns; a host call and a declared failure are not in its
form yet. How this section's fact, which names `main`'s program, is assembled from §5's is open
(§8).

## 7. Trust, before and after

| | today | for a binary with this fact |
|---|---|---|
| K, Lean's theory, the three axioms | believed | the same |
| definition → program (the erasure) | believed | the same; later a fact per function |
| the lowering, the C generator, `rt.h`, `cc`, libc | believed, compared by test | gone |
| the machine's model and the Linux boundary | none | believed; tested against the processor and the kernel |
| an engine's run of a proven pass | none | believed where K did not compute it, named in the record, repeatable under `ev.shard`; the engines are today's, C-built ones included, until the compiler compiles itself |
| host-call axioms | believed, per `bin` | the same at first; the aim is to derive them from the boundary's model |

## 8. The first build, and what stays open

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

Open in §5's form, each found by the trial or by the review before it:

- an outcome other than a return: a declared failure, a host call with the World in the state;
- a stack need that depends on the arguments — recursion that is not a tail call;
- what crosses a seam for a type: how memory holds a value of it, its size, the ranges it stands
  in. A low form that follows pointers takes two parameters that no code holds — the list a chain
  of cells stands for, by which its recursion descends, and the ranges it may touch — and nothing
  in the language marks such a parameter;
- placement: hand-written code is proven at the addresses of one file, and a module has no place
  for constants;
- values narrower than a word; the processor's features; the stack's alignment;
- a witness, for each statement at a seam, that its hypotheses can be met;
- which end §6's fact names once a chain is built as §5 has it;
- what a tool writes: kept in a file or made again by each build; the name under which a seam's
  statement cites generated code; a tool's reading of a definition, and its stating of a lemma.
  The owner expects a compile step to write shard, wants metaprogramming a first-class part of
  the language with no form chosen for it yet, and leans to "a pile of flexible tools rather
  than a fixed process" (`LAW.md` §6, 21).

Also open: a program that does not run to an end — a server, a desktop application. The fact does
not ask that a program be refined whole. Its parts carry their own facts, and the loop around them
may be built by other means (the owner's words: `LAW.md` §6, 18).

## 9. What the owner agreed to

1. §2's fact is the one statement every transformation and every compile step yields.
2. An engine's run may stand where K cannot compute a proven pass's result, named in the record
   (§3, §7).
3. The first trust removed is the back end; the erasure stays believed until a later step.
4. A binary may stop early with a declared failure, and no requirement is restated for it (§6).
5. Low-level code is ordinary shard, in §4's form: a buffer a value, evidence at each access, a
   piece of code's contract with what it leaves unwritten (`LAW.md` §6, 19).
6. A function a module exports has three levels, all shard, and two facts; one statement crosses
   the seam, the same for compiled and for hand-written code; the intermediate is inside the
   second fact (§5; `LAW.md` §6, 20).
7. A source file states everything that is claimed: nothing an author writes names anything in a
   file a tool wrote beside it (§5; `LAW.md` §6, 21).
