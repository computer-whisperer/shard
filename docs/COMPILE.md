# The compile system: the statement

Status: a draft for the owner's ratification. Nothing in the tree implements it. It fixes one
statement and the two ends of the first chain, and leaves the middle open on purpose (§7).

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

A machine is a library. For each piece of code it supplies a system: the inputs (the outside's
answers among them, so that a run has at most one outcome), the outcomes with their events, and a
rule for linking — code that calls a function may be joined with anything the callee is refined
by. K knows no machine.

| machine | its code | today |
|---|---|---|
| the logic | a function | a `fn`'s definition |
| shard run | a program as data (`kernel/prog.shard`) under `ev.shard`, each host call answered from the input | `ev`'s machine is not yet a definition (`OPEN.md` §2); the loop over host calls is not written |
| x86-64 under Linux | the bytes of an executable file | not in this tree; the previous generation's model is at a tag (`V2.md` §6) |
| others | a shader language, another processor | when a consumer comes; its floor may be a stated model above the bytes |

Between the second and the third stands low-level code: machine words, a memory, loops.
**A conjecture, to be tested before anything is built on it:** that code is a subset of shard —
programs over words and a memory value used once — and not a second language. The evaluator, K and
every tool would then work at every level, and hand-written fast code would be ordinary shard.

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

Open on purpose: the form of low-level code (§4's conjecture); how integers of any size,
constructors and reclaimed memory are represented; the calling convention and the linking rule's
exact form; events and their prefixes at the Linux boundary; registers; programs that do not end;
floats; a second target; how results are stored and reused between builds; names and directories.

## 8. For the owner to rule

1. §2's fact is the one statement every transformation and every compile step yields.
2. An engine's run may stand where K cannot compute a proven pass's result, named in the record
   (§3, §6).
3. The first trust removed is the back end; the erasure stays believed until a later step.
4. A binary may stop early with a declared failure, and no requirement is restated for it (§5).
