# v3/compile

The first pieces of the compile system (`docs/COMPILE.md` §5): an intermediate machine with its
laws, the statement that crosses a module's seam, a pass from intermediate code to x86-64
instructions with its theorem, a link theorem, and four programs taken through every layer to
a file the processor runs — six functions over words with calls, a test and a loop; `dot`, a
sum of products over two buffers; `sqdiff`, a compiled caller of `dot` across a seam, in one
file with the `dot` an author wrote in instructions and in another with the compiled one; and
`gap`, a caller of `sqdiff`, three functions deep. `dot`, `sqdiff` and `gap` are each a module
behind a view. The machine's model is `v3/models/x86` (its README).

Nothing here is a build: no tool reads a `bin` form, and nothing turns a shard definition into
intermediate code. What is proven is proven for the files below.

## The chain, for one function

| level | the first program | `dot` |
|---|---|---|
| meaning | `calls_lo.shard` `calls_f0` … `calls_f5`: functions over words. An integer that fits a word has one representation, so the meaning is the low form | `v3/models/x86/buffers.shard` `dot`: a function over two lists of words; no address |
| low form | the same six functions | the directory `dot`: a function over words and a memory, each read with the evidence that it is in the ranges given (`lo.shard` `rd64`). Its view has the signature, `dot_lo`; its body is the directory's own |
| intermediate code | `calls_ir.shard` `calls_prog`, by hand | `dot_obj.shard` `dot_prog`, by hand: the code as data |
| instructions | `x86/calls_file.shard` `calls_listing` | `x86/dot_file.shard` `dot_listing`: what the pass answers for the code (`compile`) |
| file | `x86/calls_file.shard` `calls_image`; `x86/calls_main.shard` writes it | `x86/dot_file.shard` `dot_image`: a start-up, then the instructions, assembled; `x86/dot_main.shard` writes it |

and the facts between them:

- `dot_lo_means`, a law of `dot`'s view: where memory holds the two lists, the low form's value
  is the meaning's. The first program has no such fact to prove.
- `calls_ir.shard` `calls_tie0` … `calls_tie5`; `dot_prog_tie`, the other law of `dot`'s view:
  the code's answer is the function's value. By hand: a rule of the machine for each statement;
  at a test a case; for a loop induction on the function's own measure; and at a call that
  returns the callee's tie, as it is stated and nothing else about the callee (`iends_call`). A
  function is tied in any program that has its callees where its code calls them.
- `x86/calls_run.shard` `calls_realizes`, `x86/dot_run.shard` `dot_code_realizes`: the
  instructions realize the code's answer — an instance of the pass's theorem, every hypothesis
  closed by computation.
- `x86/calls_run.shard` `calls_file_exits`: the file, started as the model's runner starts it,
  exits with 22, the low byte of what `calls_f5` computes of the words the start-up hands over —
  by the link theorem. `x86/dot_run.shard` `dot_file_exits`: `dot`'s file exits with 68, the low
  byte of what `dot` computes of the buffers 3 5 7 and 2 4 6 that the start-up pushes.

## A function as a module

`dot`, `sqdiff` and `gap` are directories with a view (`docs/LANGUAGE.md`, "Views"). For a
function `F`:

| | where | who sees it |
|---|---|---|
| the meaning `F` | `F_src.shard` (`dot`'s: the model's `buffers.shard`) | everyone: it is what a file's theorem is stated with, and what the evaluator runs |
| the intermediate code `F_prog` | `F_obj.shard` | everyone: the pass reads it and a file's writer runs it |
| the low form's signature `F_lo`, with the evidence it asks for | the view `F/mod.req.shard` | everyone |
| the first fact `F_lo_means` and the tie (`dot_prog_tie`, `F_tie`), as laws | the view | everyone, as statements |
| the low form's body, and the proofs of the two laws | `F/F.shard` and the directory's other files | the directory alone |

A consumer is checked against the view: K holds `F_lo` and the two laws as constants with a
type and no value, so no proof step of a caller, of a file, or of the author of hand-written
instructions reaches the low form's body, and what such a proof rests on is listed with it
(`params=`). What each rests on here:

| proof | rests on, of the views |
|---|---|
| `gap`'s low form and its two facts (`gap/gap.shard`) | `sqdiff_lo`, `sqdiff_lo_means` |
| `sqdiff`'s low form and its two facts (`sqdiff/sqdiff.shard`) | `dot_lo`, `dot_lo_means` |
| the hand-written `dot` realizes its answer (`x86/dot_hand.shard`) | `dot_lo`, `dot_lo_means` |
| a file exits with its function's value (`x86/*_run.shard`) | the laws of every module in the file |

Nothing of `dot` is in what `gap`'s proofs rest on. The implementations are checked each in
its own fork, against the views of what it calls. The link (`load.shard --link`) then has K
check each file's theorem once more in an environment where every parameter is its
implementation's constant: there `gap_file_exits` and the three others rest on no parameter and
on no axiom but Lean's three.

## Across a seam: one caller, two `dot`s, and a caller of the caller

`sqdiff` is the difference of two buffers' sums of squares, each a call of `dot`. Its module
holds no code of `dot`: its intermediate code calls an outside function (`sqdiff_obj.shard`),
and of that function it has an answer and nothing else.

- The answer is `dot`'s low form, as a function of the argument words and the memory
  (`two.shard` `two_ext dot_lo`, and `two_ans`, the same as a relation): the low form's value
  where the evidence about the two buffers exists, and no answer elsewhere (`lo.shard`
  `ans_on`). No program computes it; it is what both sides are proven against. It is stated
  once, for any function of two buffers that only reads.
- The caller's side, once: `sqdiff_tie`, a law of `sqdiff`'s view — the code answers
  `sqdiff_lo`, with any outside functions whose function 0 answers as `two_ext dot_lo` (a rule
  of the machine for a call of an outside function, `iends_call_ext`).
- `dot`'s side, twice. The ten instructions an author wrote and proved on the model
  (`v3/models/x86/buffers.shard`, `buffers_facts.shard` `dot_contract`, over two lists held in
  memory) realize that answer with no stack of their own, in any file whose table has their
  rows (`x86/dot_hand.shard` `dot_hand_realizes`): the lists the contract is given are those
  read out of the machine's memory, they stand in the answer's memory too, and there the low
  form's value is their meaning's by the view's law. The module the pass compiled from `dot`'s
  intermediate code realizes its run's answer, and so the same answer, which that run gives by
  `dot`'s own tie (`dot_ans_tie`, `x86/seam_laws.shard` `realizes_sub`).
- Two files: a start-up, the hand-written lines, `sqdiff`'s module compiled after them
  (`x86/sqdiff_hand_file.shard`); and a start-up, `dot`'s compiled module, `sqdiff`'s module
  compiled after it (`x86/sqdiff_comp_file.shard`). In each, `sqdiff`'s code realizes its
  answer by the module's theorem, whose hypothesis about the outside function is that file's
  fact about `dot`; and each exits with 27, the low byte of what `sqdiff` computes of the
  buffers the start-up pushes (`x86/sqdiff_hand_run.shard` `sqdiff_hand_exits`,
  `x86/sqdiff_comp_run.shard` `sqdiff_comp_exits`).

The two files differ in what the pass was told of `dot` — an entry and a stack need — and in
nothing else on the caller's side.

`gap` is `sqdiff` of two buffers less `sqdiff` of the same two the other way round: 54 on the
sample. Its module calls `sqdiff` as `sqdiff`'s calls `dot`. One file has the three
(`x86/gap_file.shard`): the hand-written `dot`, `sqdiff`'s module compiled after it, `gap`'s
after that; one statement crosses each seam, and the file exits with 54
(`x86/gap_run.shard` `gap_file_exits`).

## Files

| file | what it holds |
|---|---|
| `ir.shard` | the intermediate machine: expressions (a constant, a local, an operation of two, a load), statements in tail form (a local set, a store, a call, a two-way branch, and at each leaf a return or a tail call — no loop: a loop is a function's tail call of itself), a function (`IFn`: parameters, further locals, a body), the run by a fuel (`irun`), a function's answer (`fn_ans`). No proof: the pass reads this file |
| `ir_laws.shard` | the machine's laws: what a run of a statement says of its parts (`irun_let_inv` and its like), a run of a statement from its parts, that a run given more fuel answers the same (`irun_more`), and a rule of `iends` for each statement form a proof above the pass uses: a return, a local set, a branch each way, a tail call, a call that returns (`iends_call`), whose two runs — the callee's and what follows — are brought to one fuel, and a call of an outside function (`iends_call_ext`), by that function's answer |
| `lo.shard` | what a low form is written with: a read with its evidence (`rd64`), a buffer in the ranges (`buf_ok`), the evidence at an index (`at8_ok`), a read's word against a list held in memory (`rd_word`); and an answer given where evidence exists (`ans_on`), by which a low form is the answer of a function called from outside |
| `calls_lo.shard`, `calls_ir.shard` | the first program: its six functions with the loop's two cases, and their intermediate code with a tie for each |
| `two.shard` | functions of two buffers of one count that only read: the type of such a low form (`Reads2`), and such a function as the answer of a function called from outside — `two_ext`, `two_ans`, on words that have the evidence (`two_ext_is`), whatever it answers (`two_ans_inv`), and that code whose run answers the function's value answers what the answer says (`two_ans_tie`) |
| `sample.shard` | the two buffers the fixed start-up pushes, as a function's arguments see them: the data range, a memory that holds them, the evidence about each |
| `dot_obj.shard`, `sqdiff_obj.shard`, `gap_obj.shard` | each module's intermediate code, by hand: the code as data. `sqdiff`'s calls `dot`, and `gap`'s `sqdiff`, as a function outside its module |
| `sqdiff_src.shard`, `gap_src.shard` | the meanings of `sqdiff` and `gap`, functions over two lists, and their values on the sample |
| `dot/` | `dot`'s module. `mod.req.shard`, the view: the signature `dot_lo`, the laws `dot_lo_means` and `dot_prog_tie`, and from them that the code's run answers whatever the low form's answer says (`dot_ans_tie`) and answers the meaning's value (`dot_answer`). Behind it: `low.shard` the loop with its two cases, `mean.shard` the loop against the meaning, `tie.shard` the loop's code against the loop, `dot.shard` the low form and the two laws' proofs |
| `sqdiff/`, `gap/` | the modules of the two callers. Each view: the signature `F_lo`, the laws `F_lo_means` and `F_tie` — the tie with any outside functions whose function 0 answers as the callee's low form —, the module's outside functions by their answers (`F_ext`), and `F_prog_tie`, `F_ans_tie`, `F_answer` from the laws. Each implementation (`F/F.shard`) has the callee's view and no more of the callee |
| `x86/seam.shard` | the statement at a seam: `(realizes tbl clo chi entry k rs ans)` — the code at `entry`, called by the convention (arguments in rdi rsi rdx rcx r8 r9, the result in rax, rbx rbp r12–r15 kept) with room for `k` words of stack, returns what the answer `ans` says on any memory the machine's agrees with on the data ranges `rs`, leaves those ranges as the answer's memory has them and every other byte outside the `k` words as it was |
| `x86/seam_laws.shard` | laws of the statement that name no pass: memories that agree on a range or on the data ranges, and a list that stands in one standing in the other (`holds_agree`); a table still in place after code that kept memory has returned (`kept_code`); code that realizes an answer realizes every answer that says no more (`realizes_sub`), in every table that has its rows (`realizes_part`) |
| `x86/dot_hand.shard` | the hand-written `dot` at the seam, a consumer of `dot`'s view: from the author's contract, given the lists read out of the machine's memory, and the view's first fact on the answer's memory, `dot_hand_realizes` |
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
| `x86/gap_file.shard` `gap_run.shard` `gap_main.shard` | three functions deep: the hand-written `dot`, `sqdiff`'s module and `gap`'s compiled after it (names `gp_`); a statement at each seam, the module's theorem met twice and the theorem that the file exits with 54; the file written |

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
the same twelve in `x86/calls_file.shard`, in each of `sqdiff`'s two files those and one more —
that the hand-written rows are rows of the file's table, or that the second module's listing
stands in it — and in `x86/gap_file.shard` those for two compiled modules and the rows.

## Not here

- the step from a definition to intermediate code, and the proofs above the pass written by a
  tool: every function's low form, its intermediate code and its two facts are by hand;
- a rule of `iends` for a store and for a tail call of an outside function: no proof here uses
  one (the run's own step for a store is in `ir_laws.shard`);
- a function at a seam that writes memory: `two.shard`'s answer is a reader's, which leaves
  the memory as it was. The trial's other two hand-written functions
  (`v3/models/x86/buffers_facts.shard`) are not at the seam: `addto`, which writes into the
  buffer it is given, and `both`, which has a frame and calls; their contracts are in their own
  form, over the trial's one table;
- a low form that follows pointers, whose evidence names values no code holds;
- hand-written code at another address: `dot`'s contract is over its own rows, at the addresses
  it was written for, so a file has it at 4194489 or not at all;
- an outside function's answer that a program computes: `ans_on` is decided by no program, so
  the evaluator does not run a caller's intermediate code against it;
- a start-up other than the one of `x86/startup.shard`: the function a file enters takes its
  three words, so the first program's entry takes two addresses it does not use; a file whose
  data is not pushed by its start-up, a host call, a declared failure;
- a compiled store run by the processor: the pass's proof covers a store, no file here has one;
- a file's facts stated once for any module: each of the five files states its own;
- the first program as a module behind a view: its six functions are plain files.

## Tests

Nine entrypoints of the suite, in `v3/kernel/test/`: `compile_pass_test.sh` (the machine's
laws, the pass and its theorem accepted), `compile_calls_test.sh` (the first program's chain
accepted, to the theorem of its exit status), `compile_modules_test.sh` (the three modules and
the four files over them in one load with the link: each view's laws discharged, each proof
resting on the view of what it calls and nothing below, each file's theorem accepted in the
linked environment on Lean's three axioms and no parameter), `compile_calls_file_test.sh`,
`compile_dot_file_test.sh`, `compile_sqdiff_hand_file_test.sh`,
`compile_sqdiff_comp_file_test.sh` and `compile_gap_file_test.sh` (each file run by the model
and the processor), `compile_calls_cases_test.sh` (eight calls of the first program's
functions compared between the intermediate machine, the model and the processor). Natively
each takes under two minutes:

```sh
EVAL=v3/bin/eval v3/kernel/test/compile_modules_test.sh
```

A fault planted in the pass is refused by its proof, one in a function, its code or its file by
the fact at that joint, a hand-written line changed in a file by the file's layout fact, one
changed together with its rows by its author's proof, and a caller's proof that opens the body
of what it calls by the view; none of this is a test in the suite.
