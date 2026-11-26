# v3/compile

The first pieces of the compile system (`docs/COMPILE.md` §5): an intermediate machine with its
laws, the statement that crosses a module's seam, a pass from intermediate code to x86-64
instructions with its theorem, a link theorem, and five programs taken through every layer to
a file the processor runs — six functions over words with calls, a test and a loop; `dot`, a
sum of products over two buffers; `sqdiff`, a compiled caller of `dot` across a seam, in one
file with the `dot` an author wrote in instructions and in another with the compiled one;
`gap`, a caller of `sqdiff`, three functions deep; and `total`, a caller of `both`, which calls
`addto`, a function that writes into the buffer it is given. Each function but the first
program's is a module behind a view. The machine's model is `v3/models/x86` (its README).

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
- `x86/calls_run.shard` `calls_file_exits`: the file, started with its stack anywhere
  (`stack_ok`), exits with 22, the low byte of what `calls_f5` computes of the words the start-up hands over —
  by the link theorem. `x86/dot_run.shard` `dot_file_exits`: `dot`'s file exits with 68, the low
  byte of what `dot` computes of the buffers 3 5 7 and 2 4 6 that the start-up pushes.

## A function as a module

`dot`, `sqdiff`, `gap`, `addto`, `both` and `total` are directories with a view
(`docs/LANGUAGE.md`, "Views"). For a function `F`:

| | where | who sees it |
|---|---|---|
| the meaning `F` | `F_src.shard` (`dot`'s, `addto`'s and `both`'s: the model's `buffers.shard`) | everyone: it is what a file's theorem is stated with, and what the evaluator runs |
| the intermediate code `F_prog` | `F_obj.shard` | everyone: the pass reads it and a file's writer runs it |
| the low form's signature `F_lo`, with the evidence it asks for | the view `F/mod.req.shard` | everyone |
| the first fact `F_lo_means` and the tie (`F_prog_tie`, `F_tie`), as laws; for a function that writes, what it leaves alone (`F_lo_keeps`) | the view | everyone, as statements |
| the low form's body, and the proofs of the laws | `F/F.shard` and the directory's other files | the directory alone |

A consumer is checked against the view: K holds `F_lo` and the two laws as constants with a
type and no value, so no proof step of a caller, of a file, or of the author of hand-written
instructions reaches the low form's body, and what such a proof rests on is listed with it
(`params=`). What each rests on here:

| proof | rests on, of the views |
|---|---|
| `gap`'s low form and its two facts (`gap/gap.shard`) | `sqdiff_lo`, `sqdiff_lo_means` |
| `sqdiff`'s low form and its two facts (`sqdiff/sqdiff.shard`) | `dot_lo`, `dot_lo_means` |
| the hand-written `dot` realizes its answer (`x86/dot_hand.shard`) | `dot_lo`, `dot_lo_means` |
| `total`'s low form and its three laws (`total/total.shard`) | `both_lo`, `both_lo_means`, `both_lo_keeps`, `dot_lo`, `dot_lo_means` |
| `both`'s low form and its three laws (`both/both.shard`) | `addto_lo`, `addto_lo_means`, `addto_lo_keeps`, `dot_lo`, `dot_lo_means` |
| a file exits with its function's value (`x86/*_run.shard`) | the laws of every module in the file |

Nothing of `dot` is in what `gap`'s proofs rest on, and nothing of `addto` in what `total`'s
do. The implementations are checked each in its own fork, against the views of what it calls.
The link (`load.shard --link`) then has K check each file's theorem once more in an
environment where every parameter is its implementation's constant: there `gap_file_exits`,
`total_file_exits` and the three others rest on no parameter and on no axiom but Lean's three.

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

## A function that writes

`addto` adds the second buffer's words into the first. Its meaning returns a new list
(`v3/models/x86/buffers.shard`); its low form writes into the first buffer. What its view
states (`addto/mod.req.shard`), and each caller's after it:

- the signature: the low form's value is a pair, the result word and the memory after
  (`upd.shard` `Upd2`). `addto` has no value of its own and returns the word 0;
- the evidence it asks for: the first buffer in the ranges to read and to write, the second to
  read, and that the two lie apart (`upd.shard` `apart`). The evidence is about the ranges and
  not about the memory, so a caller hands the same evidence on after a write;
- the first fact, `addto_lo_means`: where the two lists stand at the two addresses, the memory
  after holds at the first address the list the meaning returns. A caller with a value has
  that too (`both_lo_means`: the word is the meaning's);
- what it leaves alone, `addto_lo_keeps`: every byte outside the first buffer's words is as it
  was (`same_outside`, the model's);
- the tie, `addto_prog_tie`: the run of the code answers the low form's word and memory.

The answer of such a function called from outside its module is stated once (`upd.shard`
`upd_ext`, `upd_ans`), as `two.shard`'s is for a function that only reads.

`both` calls `addto` and then `dot` on the first buffer in the memory `addto` leaves; `total`
calls `both` and then `dot` on the second buffer, which `both` did not write
(`total_src.shard`). What passes up through the views above a writer, besides what passes
above a reader:

- the evidence the writer asks for. `total`'s signature asks that the first buffer may be
  written and that the two lie apart, and hands both on to `both`, which hands them to `addto`;
- what is left alone. `both_lo_keeps` and `total_lo_keeps` are `addto`'s law, each proven from
  the law of the view below.

What a reading function gave of a buffer before a call that writes elsewhere, it gives after
it. `total` reads the second buffer after the call: by `both_lo_keeps` and the evidence that
the buffers lie apart, the memory before and the memory after agree on the second buffer's
words (`upd.shard` `kept_apart`); and `dot`'s value is one in any two memories that agree on
its buffers' words (`two.shard` `reads_on`). That is no law of `dot`'s view. It follows from
the first fact of any function that only reads — its value is the meaning's of whatever lists
stand at its addresses, so of the words read out of either memory — and is proven once, for
the signature: `sqdiff` and `gap` have it as they stand.

One file has the four functions (`x86/total_file.shard`): the hand-written `dot`, then the
modules of `addto`, `both` and `total`, each compiled on its own; it exits with 75, the low
byte of what `total` computes of the sample buffers, and with 139 if the first buffer is not
written (`x86/total_run.shard` `total_file_exits`).

The form of a writer's view — the pair, apartness as evidence in the signature, the law of what
is left alone, the word 0 — is recorded in `docs/DECISIONS.md` as chosen here and not ruled by
the owner.

## Files

| file | what it holds |
|---|---|
| `ir.shard` | the intermediate machine: expressions (a constant, a local, an operation of two, a load), statements in tail form (a local set, a store, a call, a two-way branch, and at each leaf a return or a tail call — no loop: a loop is a function's tail call of itself), a function (`IFn`: parameters, further locals, a body), the run by a fuel (`irun`), a function's answer (`fn_ans`). No proof: the pass reads this file |
| `ir_laws.shard` | the machine's laws: what a run of a statement says of its parts (`irun_let_inv` and its like), a run of a statement from its parts, that a run given more fuel answers the same (`irun_more`), and a rule of `iends` for each statement form a proof above the pass uses: a return, a local set, a branch each way, a tail call, a store (`iends_store`), a call that returns (`iends_call`), whose two runs — the callee's and what follows — are brought to one fuel, and a call of an outside function (`iends_call_ext`), by that function's answer |
| `lo.shard` | what a low form is written with: a read with its evidence (`rd64`), a write with its evidence (`wr64`), a buffer in the ranges (`buf_ok`), the evidence at an index (`at8_ok`), a read's word against a list held in memory (`rd_word`); and an answer given where evidence exists (`ans_on`), by which a low form is the answer of a function called from outside |
| `calls_lo.shard`, `calls_ir.shard` | the first program: its six functions with the loop's two cases, and their intermediate code with a tie for each |
| `two.shard` | functions of two buffers of one count that only read: the type of such a low form (`Reads2`), and such a function as the answer of a function called from outside — `two_ext`, `two_ans`, on words that have the evidence (`two_ext_is`), whatever it answers (`two_ans_inv`), and that code whose run answers the function's value answers what the answer says (`two_ans_tie`); and what such a function reads, from its first fact: its value is one in two memories that agree on the two buffers' words (`Means2`, `reads_on`) |
| `upd.shard` | functions of two buffers of one count that write the first: that two buffers lie apart (`apart`), the type of such a low form (`Upd2`), and such a function as the answer of a function called from outside — `upd_ext`, `upd_ans`, `upd_ext_is`, `upd_ans_inv`, `upd_ans_tie`, as `two.shard`'s —; and across a call that writes: a read clear of what was written reads the same (`rd_same`), and the memories before and after agree on a buffer apart from the written one (`kept_apart`) |
| `stretch.shard` | a stretch of a range of words from ANY address b (a file's data: where a kernel puts it is not the file's to know): the range (`data_rs`), and the evidence a low form asks about a stretch of it, by its offset — mapped (`start_buf`), apart from another (`start_apart`), held at the address the start-up computes (`start_addr`, `start_holds`). All that is asked of b: the range ends at or below 2^47 |
| `sample.shard` | the two buffers the sample's start-up pushes, as a function's arguments see them, the data at any address b: the data range, the argument words, the evidence about each buffer; and for evaluation a memory that holds them at one address |
| `dot_obj.shard`, `sqdiff_obj.shard`, `gap_obj.shard`, `addto_obj.shard`, `both_obj.shard`, `total_obj.shard` | each module's intermediate code, by hand: the code as data. `sqdiff`'s calls `dot`, `gap`'s `sqdiff`, `both`'s `addto` and `dot`, and `total`'s `both` and `dot`, each as a function outside its module. `addto`'s loop has a store |
| `sqdiff_src.shard`, `gap_src.shard`, `total_src.shard` | the meanings of `sqdiff`, `gap` and `total`, functions over two lists, and their values on the sample |
| `dot/` | `dot`'s module. `mod.req.shard`, the view: the signature `dot_lo`, the laws `dot_lo_means` and `dot_prog_tie`, and from them that the code's run answers whatever the low form's answer says (`dot_ans_tie`) and answers the meaning's value (`dot_answer`). Behind it: `low.shard` the loop with its two cases, `mean.shard` the loop against the meaning, `tie.shard` the loop's code against the loop, `dot.shard` the low form and the two laws' proofs |
| `sqdiff/`, `gap/` | the modules of the two callers. Each view: the signature `F_lo`, the laws `F_lo_means` and `F_tie` — the tie with any outside functions whose function 0 answers as the callee's low form —, the module's outside functions by their answers (`F_ext`), and `F_prog_tie`, `F_ans_tie`, `F_answer` from the laws. Each implementation (`F/F.shard`) has the callee's view and no more of the callee |
| `addto/` | the module of the function that writes. `mod.req.shard`, the view: the signature `addto_lo`, the laws `addto_lo_means`, `addto_lo_keeps` and `addto_prog_tie`, and `addto_ans_tie` from the last. Behind it: `low.shard` the loop with its two cases, `mean.shard` the loop against the meaning and what the loop leaves alone, `tie.shard` the loop's code against the loop, `addto.shard` the low form and the three laws' proofs |
| `both/`, `total/` | the modules of the two callers above it. Each view: the signature `F_lo` with the evidence `addto` asks for, the laws `F_lo_means`, `F_lo_keeps` and `F_tie` — the tie with any outside functions whose function 0 answers as the writing callee's low form and function 1 as `dot`'s —, and `F_ext`, `F_prog_tie`, `F_ans_tie` from them; `total`'s also `total_answer` and the sample |
| `x86/seam.shard` | the statement at a seam: `(realizes tbl clo chi entry k rs ans)` — the code at `entry`, called by the convention (arguments in rdi rsi rdx rcx r8 r9, the result in rax, rbx rbp r12–r15 kept) with room for `k` words of stack, returns what the answer `ans` says on any memory the machine's agrees with on the data ranges `rs`, leaves those ranges as the answer's memory has them and every other byte outside the `k` words as it was |
| `x86/seam_laws.shard` | laws of the statement that name no pass: memories that agree on a range or on the data ranges, and a list that stands in one standing in the other (`holds_agree`); a table still in place after code that kept memory has returned (`kept_code`); code that realizes an answer realizes every answer that says no more (`realizes_sub`), in every table that has its rows (`realizes_part`) |
| `x86/dot_hand.shard` | the hand-written `dot` at the seam, a consumer of `dot`'s view: from the author's contract, given the lists read out of the machine's memory, and the view's first fact on the answer's memory, `dot_hand_realizes` |
| `x86/pass.shard` | the pass, `(compile xs p base)`: a module's functions to instructions from the address `base`, given each outside callee's entry and stack need; it answers the listing and each function's entry and need. A local lives in one of the six registers a callee keeps; an expression is evaluated on a stack of nine temporary registers; a tail call is a jump |
| `x86/lists.shard` `code.shard` `mem.shard` `regs.shard` `inv.shard` | what the proof stands on: lists of words; a listing in place in a table (`placed`, and `placedb`, the same as a decision) and each instruction's rule for a table; memories and a frame's description across a callee's return and across a store; the registers of locals and temporaries; what the pass's answer for a statement says of its parts |
| `x86/expr.shard` `entry.shard` `stmt.shard` `stmt_store.shard` `call.shard` `stmt_call.shard` `stmt_if.shard` `tail.shard` | the proof by cases: an expression (`cexp_all`), into a function and out of it, a local set and a return, a store, the law of a call and a call as a statement, a branch, a tail call |
| `x86/function.shard` `module.shard` | the statement for every statement (`sim_all`), a compiled function (`cfn_ok`), the module's theorem (`module_realizes`), and what a list of one outside function, and of two, has at an index (`pair_at_one`, `pair_at_two`) |
| `x86/startup.shard` | a file's start-up as data: the words `data` it pushes, the last first, and at most six arguments (`Arg`), each a word or the address of one of the data's words, taken from the stack pointer when the start-up runs; then a call, and exit with the low byte of rax. The listing (`start_listing`) and the table its instructions lay out to (`start_table`), what the start-up hands the function it calls (`start_base`, `start_rs`, `arg_words`), the sample the tree's files share — two buffers of three words and their count —, and a compiled listing as lines with a label at one entry. No proof: a file's writer reads this file |
| `x86/link.shard` | `start_exits`: for any data and any arguments, the start-up followed by any lines, where the function called realizes an answer, the file exits as the answer says. By induction on the data (`pushes_run`) and on the arguments (`args_run`), over a table known from an address on (`tail_at`). `holds_app`, `holds_mid`: a memory that holds the data holds each stretch of it. For every start `stack_ok` allows: the stack's range and the stack pointer are unknowns. `link_exits`: the theorem for the sample |
| `x86/calls_file.shard` `calls_run.shard` `calls_main.shard` | the first program compiled and its file; the module's theorem met by it and the theorem that the file exits with 22; the file written |
| `x86/calls_cases_main.shard` | the same module run one call at a time: a file of six first instructions for a function of two parameters on two given words, and the word the intermediate machine's run returns, for a test to compare |
| `x86/dot_file.shard` `dot_run.shard` `dot_main.shard` | `dot` compiled and its file; the two theorems above; the file written |
| `x86/sqdiff_hand_file.shard` `sqdiff_hand_run.shard` `sqdiff_hand_main.shard` | `sqdiff` compiled after the hand-written `dot`, and that file (names `sqh_`); the outside function's fact, the module's theorem met and the theorem that the file exits with 27; the file written |
| `x86/sqdiff_comp_file.shard` `sqdiff_comp_run.shard` `sqdiff_comp_main.shard` | the same with `dot`'s compiled module in the hand-written lines' place (names `sqc_`) |
| `x86/gap_file.shard` `gap_run.shard` `gap_main.shard` | three functions deep: the hand-written `dot`, `sqdiff`'s module and `gap`'s compiled after it (names `gp_`); a statement at each seam, the module's theorem met twice and the theorem that the file exits with 54; the file written |
| `x86/total_file.shard` `total_run.shard` `total_main.shard` | the function that writes and its two callers: the hand-written `dot`, then the modules of `addto`, `both` and `total` compiled after it (names `tt_`); a statement at each seam, the module's theorem met three times and the theorem that the file exits with 75; the file written |

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

`start_exits` (`x86/link.shard`): for the listing of any start-up — any data, any arguments —
with any lines after it, assembled, where the function the start-up calls realizes an answer
and that answer has a value on the arguments' words in any memory that holds the data, the file
exits with the value's low byte. `link_exits` is that theorem for the sample: two buffers of
three words and their count. The theorem is for every start `(start_at lo sp hi r f m)` that
`stack_ok` allows (`v3/models/x86/linux.shard`): the stack's range begins above the table's end,
the words the file needs — its data's, the function's stack need, the return address — stand
under the stack pointer, and nothing else is known of where the stack is. So no address of a
buffer is a number: the evidence about a buffer is that of a stretch of the data from any
address (`stretch.shard`: `start_buf`, `start_apart`, `start_holds`), and the start-up's
instructions take the stack pointer as they find it.

A file's own facts — the pass answers this listing, the assembler these bytes, the listing lies
at these addresses, the code ends at or below the table's end — are closed computations (`rfl`,
`decide`): ten in `x86/dot_file.shard`, the same ten in `x86/calls_file.shard`, in each of `sqdiff`'s two files those and one more —
that the hand-written rows are rows of the file's table, or that the second module's listing
stands in it —, in `x86/gap_file.shard` those for two compiled modules and the rows, and in
`x86/total_file.shard` those for three and the rows.

## Not here

- the step from a definition to intermediate code, and the proofs above the pass written by a
  tool: every function's low form, its intermediate code and its two facts are by hand;
- a rule of `iends` for a tail call of an outside function: no proof here uses one;
- hand-written code at the seam that writes memory or keeps a frame. The trial's hand-written
  `addto` and `both` (`v3/models/x86/buffers_facts.shard`) are not at the seam: their contracts
  are in their own form, over the trial's one table, and `addto`'s leaves the result register
  as the loop left it, where the low form's word is 0;
- a second implementation of a writer's view, proven from the view alone;
- two functions in one chain that write different buffers, and a function that writes what its
  arguments do not name;
- what a function that writes reads, as a fact: `reads_on` is for a function that only reads;
- a low form that follows pointers, whose evidence names values no code holds;
- hand-written code at another address: `dot`'s contract is over its own rows, at the addresses
  it was written for, so a file has it at 4194547 — where the lines after the sample's
  start-up stand — or not at all;
- an outside function's answer that a program computes: `ans_on` is decided by no program, so
  the evaluator does not run a caller's intermediate code against it;
- a file of the tree with a start-up other than the sample's: the function each of the seven
  enters takes the sample's three words, so the first program's entry takes two addresses it
  does not use; a file whose data is not pushed by its start-up, a host call, a declared
  failure;
- a file's facts stated once for any module: each of the six files states its own;
- one statement of the answer of a low form called from outside, for any signature: `two.shard`
  and `upd.shard` state it for a signature each;
- the first program as a module behind a view: its six functions are plain files.

## Tests

Eleven entrypoints of the suite, in `v3/kernel/test/`: `compile_pass_test.sh` (the machine's
laws, the pass and its theorem accepted), `compile_calls_test.sh` (the first program's chain
accepted, to the theorem of its exit status), `compile_modules_test.sh` (the three modules and
the four files over them in one load with the link: each view's laws discharged, each proof
resting on the view of what it calls and nothing below, each file's theorem accepted in the
linked environment on Lean's three axioms and no parameter), `compile_writer_test.sh` (the
same for the function that writes, its two callers and their file), `compile_calls_file_test.sh`,
`compile_dot_file_test.sh`, `compile_sqdiff_hand_file_test.sh`,
`compile_sqdiff_comp_file_test.sh`, `compile_gap_file_test.sh` and
`compile_total_file_test.sh` (each file run by the model and the processor), `compile_calls_cases_test.sh` (eight calls of the first program's
functions compared between the intermediate machine, the model and the processor). Natively
each takes under two minutes:

```sh
EVAL=v3/bin/eval v3/kernel/test/compile_modules_test.sh
```

A fault planted in the pass is refused by its proof, one in a function, its code or its file by
the fact at that joint, a hand-written line changed in a file by the file's layout fact, one
changed together with its rows by its author's proof, a caller's proof that opens the body of
what it calls by the view, a low form that writes a buffer it has no evidence for and a caller
that passes one buffer twice where they are written, and a view that claims less written than
its function writes by the proof of what it leaves alone; none of this is a test in the suite.
