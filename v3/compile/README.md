# v3/compile

The first pieces of the compile system (`docs/COMPILE.md` §5): an intermediate machine, the
statement that crosses a module's seam, a pass from intermediate code to x86-64 instructions with
its theorem, a link theorem, and one function, `dot`, taken through every layer to a file the
processor runs. The machine's model is `v3/models/x86` (its README).

Nothing here is a build: no tool reads a `bin` form, and nothing turns a shard definition into
intermediate code. What is proven is proven for the files below.

## The chain, for one function

`dot` is a sum of products over two buffers of machine words.

| level | where | what |
|---|---|---|
| meaning | `v3/models/x86/buffers.shard` `dot` | a function over two lists of words; no address |
| low form | `dot_lo.shard` `dot_lo` | a function over words and a memory; each read carries the evidence that it is in the ranges given (`lo.shard` `rd64`) |
| intermediate code | `dot_ir.shard` `dot_prog` | written by hand from the low form |
| instructions | `x86/dot_file.shard` `dot_listing` | what the pass answers for the code (`compile`) |
| file | `x86/dot_file.shard` `dot_image` | a start-up, then the instructions, assembled; `x86/dot_main.shard` writes it |

and the facts between them:

- `dot_mean.shard` `dot_lo_means`: where memory holds the two lists, the low form's value is the
  meaning's.
- `dot_ir.shard` `dot_prog_tie`: the code's answer is the low form's value. By hand: induction on
  the low form's own measure, a case at the test, one rule of the machine for each statement.
- `x86/dot_run.shard` `dot_code_realizes`: the instructions realize the code's answer — an
  instance of the pass's theorem, every hypothesis closed by computation.
- `x86/dot_run.shard` `dot_file_exits`: the file, started as the model's runner starts it, exits
  with 68, the low byte of what `dot` computes of the buffers 3 5 7 and 2 4 6 that the start-up
  pushes — by the link theorem.

## Files

| file | what it holds |
|---|---|
| `ir.shard` | the intermediate machine: expressions (a constant, a local, an operation of two, a load), statements in tail form (a local set, a store, a call, a two-way branch, and at each leaf a return or a tail call — no loop: a loop is a function's tail call of itself), a function (`IFn`: parameters, further locals, a body), the run by a fuel (`irun`), a function's answer (`fn_ans`), and a rule of the run for each statement form a proof here uses |
| `lo.shard` | what a low form is written with: a read with its evidence (`rd64`), a buffer in the ranges (`buf_ok`), the evidence at an index (`at8_ok`), a read's word against a list held in memory (`rd_word`) |
| `dot_lo.shard`, `dot_ir.shard`, `dot_mean.shard` | `dot`'s low form with its two cases, its intermediate code with the tie, the low form against the meaning |
| `x86/seam.shard` | the statement at a seam: `(realizes tbl clo chi entry k rs ans)` — the code at `entry`, called by the convention (arguments in rdi rsi rdx rcx r8 r9, the result in rax, rbx rbp r12–r15 kept) with room for `k` words of stack, returns what the answer `ans` says on any memory the machine's agrees with on the data ranges `rs`, leaves those ranges as the answer's memory has them and every other byte outside the `k` words as it was |
| `x86/seam_laws.shard` | laws of the statement that name no pass: a table still in place after code that kept memory has returned (`kept_code`), memories that agree on a range |
| `x86/pass.shard` | the pass, `(compile xs p base)`: a module's functions to instructions from the address `base`, given each outside callee's entry and stack need; it answers the listing and each function's entry and need. A local lives in one of the six registers a callee keeps; an expression is evaluated on a stack of nine temporary registers; a tail call is a jump |
| `x86/lists.shard` `code.shard` `mem.shard` `regs.shard` `inv.shard` | what the proof stands on: lists of words; a listing in place in a table (`placed`, and `placedb`, the same as a decision) and each instruction's rule for a table; memories and a frame's description across a callee's return and across a store; the registers of locals and temporaries; what the pass's answer for a statement says of its parts |
| `x86/expr.shard` `entry.shard` `stmt.shard` `stmt_store.shard` `call.shard` `stmt_call.shard` `stmt_if.shard` `tail.shard` | the proof by cases: an expression (`cexp_all`), into a function and out of it, a local set and a return, a store, the law of a call and a call as a statement, a branch, a tail call |
| `x86/function.shard` `module.shard` | the statement for every statement (`sim_all`), a compiled function (`cfn_ok`), and the module's theorem (`module_realizes`) |
| `x86/link.shard` | a start-up of twenty instructions — two buffers of three words pushed, their addresses and the count in the argument registers, a call, exit with the low byte of rax — followed by any lines, and `link_exits`: where the function called realizes an answer, the file exits as the answer says |
| `x86/calls.shard` `calls_facts.shard` `calls_main.shard` | a module of five functions with calls, an `if` and a loop; the theorem's hypotheses met by it; its file for the processor |
| `x86/dot_file.shard` `dot_run.shard` `dot_main.shard` | `dot` compiled and its file; the two theorems above; the file written |

## The two theorems proven once

`module_realizes` (`x86/module.shard`): if `(compile xs p base)` answers a listing and that
listing stands in a table (`placed`), and each outside callee realizes its answer at the entry
and with the need `xs` gives, then every function of `p` realizes its own answer — `fn_ans`, the
run of its intermediate code — at the entry and with the need the pass answered. It covers every
form the pass accepts and nothing else: the pass refuses what it does not compile (a seventh
local, an expression deeper than the temporaries, a comparison other than unsigned below and not
below, a call's argument that is not a constant or a local, a call of itself that is not a tail
call).

`link_exits` (`x86/link.shard`): for the start-up's listing with any lines after it, assembled,
where the function the start-up calls realizes an answer and that answer has a value on the two
buffers, the file exits with the value's low byte.

A file's own facts — the pass answers this listing, the assembler these bytes, the listing lies
at these addresses — are closed computations (`rfl`, `decide`): twelve in `x86/dot_file.shard`.

## Not here

- the step from a definition to intermediate code, and the proofs above the pass written by a
  tool: `dot`'s low form, its code and both proofs are by hand;
- a rule of the machine's run for a call that returns (a tail call has one), so no function that
  calls and returns has its code tied to a low form; `calls.shard`'s module is run, not tied;
- hand-written code joined through the seam's statement: the trial's `dot`
  (`v3/models/x86/buffers_facts.shard`) is proven in its own contract's form;
- a file whose data is not pushed by its start-up, a host call, a declared failure;
- a compiled store run by the processor: the pass's proof covers a store, no file here has one.

## Tests

Four entrypoints of the suite, in `v3/kernel/test/`: `compile_pass_test.sh` (the pass and its
theorem accepted; its hypotheses met by the module of calls), `compile_calls_test.sh` (that
module's file: eight calls compared between the intermediate machine, the model and the
processor), `compile_dot_test.sh` (`dot`'s chain accepted) and `compile_dot_file_test.sh`
(`dot`'s file run by the model and the processor). Natively each takes under a minute:

```sh
EVAL=v3/bin/eval v3/kernel/test/compile_dot_test.sh
```

A fault planted in the pass is refused by its proof, one in `dot`'s low form, code or file by the
fact at that joint; none of this is a test in the suite.
