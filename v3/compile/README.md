# v3/compile

The first pieces of the compile system (`docs/COMPILE.md` §5): an intermediate machine with its
laws, the statement that crosses a module's seam, a pass from intermediate code to x86-64
instructions with its theorem, a link theorem, and three programs taken through every layer to
a file the processor runs — six functions over words with calls, a test and a loop; `dot`, a
sum of products over two buffers; and `sqdiff`, a compiled caller of `dot` across a seam, in
one file with the `dot` an author wrote in instructions and in another with the compiled one.
The machine's model is `v3/models/x86` (its README).

Nothing here is a build: no tool reads a `bin` form, and nothing turns a shard definition into
intermediate code. What is proven is proven for the files below.

## The chain, for one function

| level | the first program | `dot` |
|---|---|---|
| meaning | `calls_lo.shard` `calls_f0` … `calls_f5`: functions over words. An integer that fits a word has one representation, so the meaning is the low form | `v3/models/x86/buffers.shard` `dot`: a function over two lists of words; no address |
| low form | the same six functions | `dot_lo.shard` `dot_lo`: a function over words and a memory; each read carries the evidence that it is in the ranges given (`lo.shard` `rd64`) |
| intermediate code | `calls_ir.shard` `calls_prog`, by hand | `dot_ir.shard` `dot_prog`, by hand |
| instructions | `x86/calls_file.shard` `calls_listing` | `x86/dot_file.shard` `dot_listing`: what the pass answers for the code (`compile`) |
| file | `x86/calls_file.shard` `calls_image`; `x86/calls_main.shard` writes it | `x86/dot_file.shard` `dot_image`: a start-up, then the instructions, assembled; `x86/dot_main.shard` writes it |

and the facts between them:

- `dot_mean.shard` `dot_lo_means`: where memory holds the two lists, the low form's value is the
  meaning's. The first program has no such fact to prove.
- `calls_ir.shard` `calls_tie0` … `calls_tie5`, `dot_ir.shard` `dot_prog_tie`: the code's answer
  is the function's value. By hand: a rule of the machine for each statement; at a test a case;
  for a loop induction on the function's own measure; and at a call that returns the callee's
  tie, as it is stated and nothing else about the callee (`iends_call`). A function is tied in
  any program that has its callees where its code calls them.
- `x86/calls_run.shard` `calls_realizes`, `x86/dot_run.shard` `dot_code_realizes`: the
  instructions realize the code's answer — an instance of the pass's theorem, every hypothesis
  closed by computation.
- `x86/calls_run.shard` `calls_file_exits`: the file, started as the model's runner starts it,
  exits with 22, the low byte of what `calls_f5` computes of the words the start-up hands over —
  by the link theorem. `x86/dot_run.shard` `dot_file_exits`: `dot`'s file exits with 68, the low
  byte of what `dot` computes of the buffers 3 5 7 and 2 4 6 that the start-up pushes.

## Across a seam: one caller, two `dot`s

`sqdiff` is the difference of two buffers' sums of squares, each a call of `dot`
(`sqdiff_lo.shard`: its meaning over two lists, its low form over words and a memory, the first
fact). Its module holds no code of `dot`: its intermediate code calls an outside function
(`sqdiff_ir.shard`), and of that function it has an answer and nothing else.

- The answer is `dot`'s low form, as a function of the argument words and the memory
  (`dot_lo.shard` `dot_ext`, and `dot_ans`, the same as a relation): the low form's value where
  the evidence about the two buffers exists, and no answer elsewhere (`lo.shard` `ans_on`). No
  program computes it; it is what both sides are proven against.
- The caller's side, once: `sqdiff_tie` — the code answers `sqdiff_lo`, with any outside
  functions whose function 0 answers as `dot_ext` (a rule of the machine for a call of an
  outside function, `iends_call_ext`).
- `dot`'s side, twice. The ten instructions an author wrote and proved on the model
  (`v3/models/x86/buffers.shard`, `buffers_facts.shard` `dot_contract`, over two lists held in
  memory) realize `dot_ans` with no stack of their own, in any file whose table has their rows
  (`x86/dot_hand.shard` `dot_hand_realizes`). The module the pass compiled from `dot`'s
  intermediate code realizes its run's answer, and so `dot_ans`, which that run gives by `dot`'s
  own tie (`dot_ir.shard` `dot_ans_tie`, `x86/seam_laws.shard` `realizes_sub`).
- Two files: a start-up, the hand-written lines, `sqdiff`'s module compiled after them
  (`x86/sqdiff_hand_file.shard`); and a start-up, `dot`'s compiled module, `sqdiff`'s module
  compiled after it (`x86/sqdiff_comp_file.shard`). In each, `sqdiff`'s code realizes its
  answer by the module's theorem, whose hypothesis about the outside function is that file's
  fact about `dot`; and each exits with 27, the low byte of what `sqdiff` computes of the
  buffers the start-up pushes (`x86/sqdiff_hand_run.shard` `sqdiff_hand_exits`,
  `x86/sqdiff_comp_run.shard` `sqdiff_comp_exits`).

The two files differ in what the pass was told of `dot` — an entry and a stack need — and in
nothing else on the caller's side.

## Files

| file | what it holds |
|---|---|
| `ir.shard` | the intermediate machine: expressions (a constant, a local, an operation of two, a load), statements in tail form (a local set, a store, a call, a two-way branch, and at each leaf a return or a tail call — no loop: a loop is a function's tail call of itself), a function (`IFn`: parameters, further locals, a body), the run by a fuel (`irun`), a function's answer (`fn_ans`). No proof: the pass reads this file |
| `ir_laws.shard` | the machine's laws: what a run of a statement says of its parts (`irun_let_inv` and its like), a run of a statement from its parts, that a run given more fuel answers the same (`irun_more`), and a rule of `iends` for each statement form a proof above the pass uses: a return, a local set, a branch each way, a tail call, a call that returns (`iends_call`), whose two runs — the callee's and what follows — are brought to one fuel, and a call of an outside function (`iends_call_ext`), by that function's answer |
| `lo.shard` | what a low form is written with: a read with its evidence (`rd64`), a buffer in the ranges (`buf_ok`), the evidence at an index (`at8_ok`), a read's word against a list held in memory (`rd_word`); and an answer given where evidence exists (`ans_on`), by which a low form is the answer of a function called from outside |
| `calls_lo.shard`, `calls_ir.shard` | the first program: its six functions with the loop's two cases, and their intermediate code with a tie for each |
| `dot_lo.shard`, `dot_ir.shard`, `dot_mean.shard` | `dot`'s low form with its two cases, and as the answer of a function called from outside (`dot_ext`, `dot_ans`); its intermediate code with the tie, and that the code's run answers whatever `dot_ans` says; the low form against the meaning |
| `sqdiff_lo.shard`, `sqdiff_ir.shard` | a caller of `dot`: its meaning, its low form and the first fact; its intermediate code, which calls `dot` as an outside function, and the tie |
| `x86/seam.shard` | the statement at a seam: `(realizes tbl clo chi entry k rs ans)` — the code at `entry`, called by the convention (arguments in rdi rsi rdx rcx r8 r9, the result in rax, rbx rbp r12–r15 kept) with room for `k` words of stack, returns what the answer `ans` says on any memory the machine's agrees with on the data ranges `rs`, leaves those ranges as the answer's memory has them and every other byte outside the `k` words as it was |
| `x86/seam_laws.shard` | laws of the statement that name no pass: memories that agree on a range or on the data ranges; a table still in place after code that kept memory has returned (`kept_code`); code that realizes an answer realizes every answer that says no more (`realizes_sub`), in every table that has its rows (`realizes_part`) |
| `x86/dot_hand.shard` | the hand-written `dot` at the seam: `dot`'s low form reads the ranges alone, so its value is the same on two memories that agree on them; and from the author's contract, `dot_hand_realizes` |
| `x86/pass.shard` | the pass, `(compile xs p base)`: a module's functions to instructions from the address `base`, given each outside callee's entry and stack need; it answers the listing and each function's entry and need. A local lives in one of the six registers a callee keeps; an expression is evaluated on a stack of nine temporary registers; a tail call is a jump |
| `x86/lists.shard` `code.shard` `mem.shard` `regs.shard` `inv.shard` | what the proof stands on: lists of words; a listing in place in a table (`placed`, and `placedb`, the same as a decision) and each instruction's rule for a table; memories and a frame's description across a callee's return and across a store; the registers of locals and temporaries; what the pass's answer for a statement says of its parts |
| `x86/expr.shard` `entry.shard` `stmt.shard` `stmt_store.shard` `call.shard` `stmt_call.shard` `stmt_if.shard` `tail.shard` | the proof by cases: an expression (`cexp_all`), into a function and out of it, a local set and a return, a store, the law of a call and a call as a statement, a branch, a tail call |
| `x86/function.shard` `module.shard` | the statement for every statement (`sim_all`), a compiled function (`cfn_ok`), and the module's theorem (`module_realizes`) |
| `x86/startup.shard` | a file's start-up of twenty instructions, as a listing and as a table — two buffers of three words pushed, their addresses and the count in the argument registers, a call, exit with the low byte of rax —, what it hands the function it calls, and a compiled listing as lines with a label at one entry. No proof: a file's writer reads this file |
| `x86/link.shard` | `link_exits`: for the start-up followed by any lines, where the function called realizes an answer, the file exits as the answer says |
| `x86/calls_file.shard` `calls_run.shard` `calls_main.shard` | the first program compiled and its file; the module's theorem met by it and the theorem that the file exits with 22; the file written |
| `x86/calls_cases_main.shard` | the same module run one call at a time: a file of six first instructions for a function of two parameters on two given words, and the word the intermediate machine's run returns, for a test to compare |
| `x86/dot_file.shard` `dot_run.shard` `dot_main.shard` | `dot` compiled and its file; the two theorems above; the file written |
| `x86/sqdiff_hand_file.shard` `sqdiff_hand_run.shard` `sqdiff_hand_main.shard` | `sqdiff` compiled after the hand-written `dot`, and that file (names `sqh_`); the outside function's fact, the module's theorem met and the theorem that the file exits with 27; the file written |
| `x86/sqdiff_comp_file.shard` `sqdiff_comp_run.shard` `sqdiff_comp_main.shard` | the same with `dot`'s compiled module in the hand-written lines' place (names `sqc_`) |

## The theorems proven once

`irun_more` and `iends_call` (`ir_laws.shard`): a run that answers answers the same on any fuel
above; so where a callee's body answers `w` and leaves `m1`, and what follows the call answers
with the local set to `w` on `m1`, the call statement answers — the callee's fact used as
`fn_ans` states it, the same statement the pass's theorem realizes. `iends_call_ext`: the same
for a function outside the module, by its answer.

`realizes_sub` and `realizes_part` (`x86/seam_laws.shard`): code that realizes an answer
realizes every answer that says no more, and realizes it in every table that has the code's
rows at their addresses. By the first, code proven against one form of its answer — a run of
intermediate code, a contract over lists — stands for a caller that runs with another; by the
second, a proof over a function's own rows holds in any file that has them.

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
at these addresses — are closed computations (`rfl`, `decide`): twelve in `x86/dot_file.shard`,
the same twelve in `x86/calls_file.shard`, and in each of `sqdiff`'s two files those and one
more: that the hand-written rows are rows of the file's table, or that the second module's
listing stands in it.

## Not here

- the step from a definition to intermediate code, and the proofs above the pass written by a
  tool: every function's intermediate code and its tie are by hand;
- a rule of `iends` for a store and for a tail call of an outside function: no proof here uses
  one (the run's own step for a store is in `ir_laws.shard`);
- the trial's other two hand-written functions at the seam (`v3/models/x86/buffers_facts.shard`):
  `addto`, which writes into the buffer it is given, and `both`, which has a frame and calls;
  their contracts are in their own form, over the trial's one table;
- hand-written code at another address: `dot`'s contract is over its own rows, at the addresses
  it was written for, so a file has it at 4194489 or not at all;
- an outside function's answer that a program computes: `ans_on` is decided by no program, so
  the evaluator does not run a caller's intermediate code against it;
- a start-up other than the one of `x86/startup.shard`: the function a file enters takes its
  three words, so the first program's entry takes two addresses it does not use; a file whose
  data is not pushed by its start-up, a host call, a declared failure;
- a compiled store run by the processor: the pass's proof covers a store, no file here has one;
- a file's facts stated once for any module: each of the four files states its own.

## Tests

Eleven entrypoints of the suite, in `v3/kernel/test/`: `compile_pass_test.sh` (the machine's
laws, the pass and its theorem accepted), `compile_calls_test.sh`, `compile_dot_test.sh`,
`compile_sqdiff_hand_test.sh` and `compile_sqdiff_comp_test.sh` (each file's chain accepted, to
the theorem of its exit status), `compile_calls_file_test.sh`, `compile_dot_file_test.sh`,
`compile_sqdiff_hand_file_test.sh` and `compile_sqdiff_comp_file_test.sh` (each file run by the
model and the processor), `compile_calls_cases_test.sh` (eight calls of the first program's
functions compared between the intermediate machine, the model and the processor). Natively
each takes under a minute:

```sh
EVAL=v3/bin/eval v3/kernel/test/compile_calls_test.sh
```

A fault planted in the pass is refused by its proof, one in a function, its code or its file by
the fact at that joint, a hand-written line changed in a file by the file's layout fact, and
one changed together with its rows by its author's proof; none of this is a test in the suite.
