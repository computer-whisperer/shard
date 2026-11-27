# v3/compile

The first pieces of the compile system (`docs/COMPILE.md` §5): an intermediate machine with its
laws, the statement that crosses a module's seam, a pass from intermediate code to x86-64
instructions with its theorem, a link theorem, and eight programs taken through every layer to
a file the processor runs — six functions over words with calls, a test and a loop; `argsum`,
a program of its command-line arguments, proven for every list of them; `greet`, a program
that writes, proven for every list of the kernel's answers; `say`, a program of its arguments
that writes in a loop, proven for every list of both; `dot`, a
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
  (`stack_ok`) and whatever the kernel would answer a call (`exits`, `v3/models/x86/linux.shard`:
  it writes nothing), exits with 22, the low byte of what `calls_f5` computes of the words the start-up hands over —
  by the link theorem. `x86/dot_run.shard` `dot_file_exits`: `dot`'s file exits with 68, the low
  byte of what `dot` computes of the buffers 3 5 7 and 2 4 6 that the start-up pushes.

## A program of its arguments

`argsum` is the sum of the decimal numbers given after the name a program is run by
(`argsum_src.shard`): a function over a list of lists of bytes, in word arithmetic. Its file's
theorem (`x86/argsum_run.shard` `argsum_file_exits`) is for every start `stack_ok` allows and
every list of arguments the start's memory holds at the stack pointer (`args_at`,
`v3/models/x86/linux.shard`): the file exits with the low byte of `argsum` of them.

- The start-up pushes nothing and hands the function it calls one word, the address of word 0
  of no data: the stack pointer as the kernel left it.
- The intermediate code (`argsum_ir.shard`) is three functions: the entry loads the count and
  passes over the first address; a loop over the addresses loads each and calls the third; and
  that one reads a string a byte at a time (`ELoad8`) to the 0 after it. It reads through one
  range, from the stack pointer to the top of the stack's range (`args_rs`).
- The tie is from the meaning straight to the code, by induction on the lists
  (`argsum_tie0`, `argsum_tie1`, `argsum_tie2`): there is no low form. What the code may read
  — each address in the range, each string inside it — is what `args_at` states.
- The link theorem for such a file is `start_exits_top`: the function's data range reaches
  from the data's first word to the top of the stack's range, and its answer is asked on a
  memory that is the start's from the stack pointer up, which therefore holds the same
  arguments (`v3/models/x86/args.shard` `args_at_same`).

## A program that writes

`greet` writes "hello, world" and a line's end, in two pieces (`greet_src.shard`). Its meaning
is a function on a WORLD (`v3/std/world.shard`): one caller's conversation with the outside —
the answers it will still be given to its offers of bytes, the bytes it has written, and
whether its process still runs. The platform's `write_all` is an ordinary function on a World,
and no axiom: every byte where the answers take them all, however few at a time; the bytes
taken before a failure, and the process stopped; those taken before the outside ended it; and
nothing more in a process that has stopped. `greet`'s value is a word, the status its process
exits with, and the World after.

- The intermediate machine has a second run, with what is outside a process (`ir.shard`
  `wrun`): the kernel's answers still to come and the bytes written (`models/x86/linux.shard`
  `Os`). An outside function's answer may change it, or end the process, and a run in which a
  callee ends the process has ended (`Outcome`). No local holds the World.
- `write_all` outside a module is such a function (`put.shard` `put_ext`): on an address and a
  count whose bytes the data ranges map to read, the bytes go as the answers decide; it
  returns 1, or the process exits with a failed write's status 74, or the kernel ends it.
  That answer is the platform's function on the World of what is outside (`put_all`,
  `put_fail`, `put_kill`), because the model's loop is (`models/x86/world.shard` `send_put`).
- Its code is two moves and the loop an author proved on the model (`models/x86/write.shard`):
  it realizes the answer wherever its sixteen instructions stand, with no word of stack
  (`x86/put.shard` `put_realizes`).
- The tie (`greet_ir.shard` `greet_tie`) is by the rules of a tie (`tie.shard`) and one of
  them for a call that writes, used twice (`put.shard` `tie_put`): a call that writes, then
  what follows where the process still runs — and where the write stopped the process
  nothing, the rest of the program writing nothing in a process that has stopped.
- The file (`x86/greet_file.shard`): a start-up that pushes the text as two words and hands
  over the address of the first, `write_all`'s lines, and `greet`'s module compiled after them
  with `write_all` as its one outside function. The text is known byte for byte in a memory
  that holds the two words (`models/x86/bytes.shard` `held_bytes`).
- The theorem (`x86/greet_run.shard` `greet_file_ends`): started with its stack anywhere and
  on EVERY list of the kernel's answers, the file ends as `greet`'s value on the World a
  process starts with says (`models/x86/world.shard` `world_res`) — the text written and
  status 0; status 74 and the bytes taken before a failed write; or ended by the kernel.

### A loop that writes

`say` writes each of its arguments after the name on a line of its own (`say_src.shard`): a
recursion over the arguments on a World, each written by `write_all` and a line's end after
it. Where a write stops the process the arguments after it write nothing, because `write_all`
writes nothing in a process that has stopped (`say_each_stopped`).

- The code is four functions (`say_ir.shard`): a string's length, a loop over its bytes; one
  line, which calls the first and then `write_all` twice; the loop over the arguments'
  addresses, which calls the second for each and then itself; and the entry, which reads the
  count the kernel left.
- A tie is stated with one relation (`tie.shard` `ties`, `fn_ties`): the code's outcome is
  the value's — it returned the word, left the memory said and left outside what the World is
  of; or the process ended in it as a World that has stopped says (`models/x86/world.shard`
  `tied`). There is a rule for each statement form a tie uses: a return, a local set, a
  branch each way, a tail call, and a call of a function (`tie_call`) or an outside function
  (`tie_ext`) that is itself tied.
  The last two ask one thing more than a call that returns: where the process stopped in the
  callee, the caller's World is the callee's.
- The loop's tie (`say_tie2`) is by induction on the arguments, for whatever is outside: the
  line's tie; then, where the process still runs, the function's own tail call on what the
  line left outside; and where the line stopped the process, the code's run has ended, and in
  the meaning the arguments after it write nothing. A function that calls nothing outside has
  the same statement, its World the World of what is outside as it was (`say_tie0`), and is
  called by `tie_call_same`.
- The file (`x86/say_file.shard`): a start-up that pushes one word, the byte of a line's end,
  and hands over the address the kernel left the stack pointer at and the address of that
  word; `write_all`'s lines; and the four functions compiled after them. The code reads
  through one range, from the pushed word to the top of the stack's range.
- The theorem (`x86/say_run.shard` `say_file_ends`): started with its stack anywhere, with
  ANY arguments the kernel leaves at the stack pointer and on EVERY list of the kernel's
  answers, the file ends as `say`'s value on the World a process starts with says.

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
| `ir.shard` | the intermediate machine: expressions (a constant, a local, an operation of two, a load of a word, a load of a byte), statements in tail form (a local set, a store, a call, a two-way branch, and at each leaf a return or a tail call — no loop: a loop is a function's tail call of itself), a function (`IFn`: parameters, further locals, a body), ONE run by a fuel, with what is outside a process (`wrun`, `wends`, `wfn_ans`), in which an external's answer is an outcome; and, for code that names nothing outside, its answer as a word and a memory for every outside (`iends`, `fn_ans`: definitions over that run). No proof: the pass reads this file |
| `tie.shard` | a function that writes tied to its code: the statement of a statement's run and of a function's (`ties`, `fn_ties`: the outcome is the value's, a word and a World, with the memory left), and a rule for each statement form a tie uses — `tie_ret`, `tie_let`, `tie_if_true`, `tie_if_false`, `tie_tail`, and for a call of what is itself tied `tie_call`, `tie_ext` and `tie_call_same` (a callee that leaves the outside as it was) |
| `put.shard` | the platform's `write_all` as a function outside a module: its answer on what is outside the process (`put_out`, `put_ext`), that answer as the platform's function on a World (`put_all`, `put_fail`, `put_kill`), that the answer's outcome is tied to it (`put_tied`), the rule for a call of it (`tie_put`, by `tie.shard`'s rule for a call of what is tied), and the outside functions of a module whose one outside function it is (`put_wx`) |
| `greet_src.shard`, `greet_ir.shard` | a program that writes: its meaning on a World, and its intermediate code — one function that calls `write_all` twice — with its tie and how its file ends by the kernel's answers (`greet_res`) |
| `say_src.shard`, `say_ir.shard` | a program of its arguments that writes in a loop: its meaning on a World, with what it writes in a process that has stopped (`say_each_stopped`); and its intermediate code — four functions — with a tie for each (`say_tie0` … `say_tie3`, `say_prog_tie`) and how its file ends by the arguments and the kernel's answers (`say_res`) |
| `ir_laws.shard` | the machine's laws: an expression's word; what a run of a statement says of its parts (`wrun_let_inv` and its like) and a run of a statement from its parts; that a run with an outcome has it on any fuel above (`wrun_more`); a rule of `wends` for each statement form — a return, a local set, a store, a branch each way, a tail call, and a call, of an outside function or of a function of the program, that returns (the two runs it joins brought to one fuel) and one in which the process ends —; and from those the rules of `iends` for code that names nothing outside (`iends_let`, `iends_call`, `iends_call_ext` and the rest), each its rule of `wends` for every outside |
| `lo.shard` | what a low form is written with: a read with its evidence (`rd64`), a write with its evidence (`wr64`), a buffer in the ranges (`buf_ok`), the evidence at an index (`at8_ok`), a read's word against a list held in memory (`rd_word`); and an answer given where evidence exists (`ans_on`), by which a low form is the answer of a function called from outside |
| `calls_lo.shard`, `calls_ir.shard` | the first program: its six functions with the loop's two cases, and their intermediate code with a tie for each |
| `argsum_src.shard`, `argsum_ir.shard` | a program of its command-line arguments: its meaning over a list of lists of bytes — a string's digits, the sum over the strings, the first argument passed over —, and its intermediate code, three functions, with a tie for each from the meaning straight to the code where a memory holds the arguments (`args_at`) |
| `wide_ir.shard` | intermediate code of five functions of more than six locals each, tied to no meaning: what a compiled file is run on to compare the pass's code for locals in the frame with the intermediate machine |
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
| `x86/seam.shard` | the statement at a seam: `(realizes tbl clo chi entry k rs ans)` — the code at `entry`, called by the convention (arguments in rdi rsi rdx rcx r8 r9, the result in rax, rbx rbp r12–r15 kept) with room for `k` words of stack, returns what the answer `ans` says on any memory the machine's agrees with on the data ranges `rs`, leaves those ranges as the answer's memory has them and every other byte outside the `k` words as it was, and what is outside the process as it was. `(wrealizes …)` is the same of an answer given on the outside as well: the code arrives at the answer's outcome (`arrives`), a return with the outside as the code left it or the process's end. Both say where the run comes to (`models/x86/laws.shard` `comes`); an answer that names nothing outside is one that does (`lift_ans`, `realizes_lift`) |
| `x86/seam_laws.shard` | laws of the statement that name no pass: memories that agree on a range or on the data ranges, and a list that stands in one standing in the other (`holds_agree`); a table still in place after code that kept memory has returned (`kept_code`); code that realizes an answer realizes every answer that says no more (`realizes_sub`), in every table that has its rows (`realizes_part`) |
| `x86/dot_hand.shard` | the hand-written `dot` at the seam, a consumer of `dot`'s view: from the author's contract, given the lists read out of the machine's memory, and the view's first fact on the answer's memory, `dot_hand_realizes` |
| `x86/pass.shard` | the pass, `(compile xs p base)`: a module's functions to instructions from the address `base`, given each outside callee's entry and stack need; it answers the listing and each function's entry and need. Each of a function's first six locals lives in a register a callee keeps, a local past the sixth in a word of the function's frame (`ld_loc`, `st_loc`; made on entry by `slots_in`, given back by `slots_out`); a function takes at most six arguments; an expression is evaluated on a stack of nine temporary registers; a tail call is a jump |
| `x86/lists.shard` `code.shard` `mem.shard` `regs.shard` `inv.shard` | what the proof stands on: lists of words; a listing in place in a table (`placed`, and `placedb`, the same as a decision) and each instruction's rule for a table; memories and a frame's description across a callee's return and across a store; the registers of locals and temporaries, and a function's locals in a state (`locs`: the first six in their registers, the rest words of the frame); what the pass's answer for a statement says of its parts |
| `x86/expr.shard` `entry.shard` `stmt.shard` `stmt_store.shard` `call.shard` `stmt_call.shard` `stmt_if.shard` `tail.shard` | the proof by cases: an expression (`cexp_all`), into a function and out of it (the six pushes, the frame's locals made and given back, the six pops), a local set from rax in its register or its word of the frame (`set_loc`) and a return, a store, the law of a call and a call as a statement, a branch, a tail call |
| `x86/function.shard` `module.shard` | the statement for every statement (`sim_all`), a compiled function (`cfn_ok`), the module's theorem for runs with the outside (`wmodule_realizes`) and from it the one for answers that name nothing outside (`module_realizes`, by `lift_ext` and `unlift_fn`), what a list of one outside function, and of two, has at an index (`pair_at_one`, `pair_at_two`), and that code which realizes the answer of a module's one outside function meets the theorem's hypothesis about outside functions (`one_out_realizes`) |
| `x86/put.shard` | `write_all`'s code at the seam: two moves and the model's loop, and that it realizes the function's answer in any table that has its sixteen instructions (`put_realizes`) — the answer a module's run is given for its one outside function (`put_wx_realizes`) |
| `x86/startup.shard` | a file's start-up as data: the words `data` it pushes, the last first, and at most six arguments (`Arg`), each a word or the address of one of the data's words, taken from the stack pointer when the start-up runs; then a call, and exit with the low byte of rax. The listing (`start_listing`) and the table its instructions lay out to (`start_table`), what the start-up hands the function it calls (`start_base`, `start_rs`, `arg_words`), the sample the tree's files share — two buffers of three words and their count —, and a compiled listing as lines with a label at one entry. No proof: a file's writer reads this file |
| `x86/link.shard` | `start_exits_top`: for any data and any arguments, the start-up followed by any lines, where the function called realizes an answer over a range from the data's first word to any address of the stack's range, the file exits as the answer says, the answer asked on a memory that is the start's from the stack pointer up (`first_above`: the image lies below the stack; the start-up pushes below the stack pointer); `start_exits`, its corollary for the data's range alone. By induction on the data (`pushes_run`) and on the arguments (`args_run`), over a table known from an address on (`tail_at`). `holds_app`, `holds_mid`: a memory that holds the data holds each stretch of it. For every start `stack_ok` allows: the stack's range and the stack pointer are unknowns. `link_exits`: the theorem for the sample. `start_ends_top` and `start_ends` are the theorems for a function whose answer is an outcome — the file ends as the outcome says —, proven first; the three above are their corollaries for an answer that names nothing outside |
| `x86/calls_file.shard` `calls_run.shard` `calls_main.shard` | the first program compiled and its file; the module's theorem met by it and the theorem that the file exits with 22; the file written |
| `x86/argsum_file.shard` `argsum_run.shard` `argsum_main.shard` | `argsum` compiled after a start-up that pushes nothing and hands over the stack pointer's address, and its file; the module's theorem met by it and the theorem that the file exits with `argsum` of any arguments; the file written, and the evaluator's status for lists of arguments |
| `x86/greet_file.shard` `greet_run.shard` `greet_main.shard` | `greet` compiled after `write_all`'s lines, and its file; `write_all`'s theorem and the module's met by it, the text known in the memory the start-up leaves, and the theorem that the file ends as `greet` says for every list of answers; the file written, and the theorem's right side printed for lists of answers |
| `x86/say_file.shard` `say_run.shard` `say_main.shard` | `say` compiled after `write_all`'s lines, and its file; the module's theorem met by it, the line's end known in the memory the start-up leaves, and the theorem that the file ends as `say` says for every list of arguments and of answers; the file written, and the theorem's right side printed for cases of answers and arguments |
| `x86/calls_cases_main.shard` | a module run one call at a time — the first program's, or `wide_ir.shard`'s —: a file of six first instructions for a function of two parameters on two given words, and the word the intermediate machine's run returns, for a test to compare |
| `x86/dot_file.shard` `dot_run.shard` `dot_main.shard` | `dot` compiled and its file; the two theorems above; the file written |
| `x86/sqdiff_hand_file.shard` `sqdiff_hand_run.shard` `sqdiff_hand_main.shard` | `sqdiff` compiled after the hand-written `dot`, and that file (names `sqh_`); the outside function's fact, the module's theorem met and the theorem that the file exits with 27; the file written |
| `x86/sqdiff_comp_file.shard` `sqdiff_comp_run.shard` `sqdiff_comp_main.shard` | the same with `dot`'s compiled module in the hand-written lines' place (names `sqc_`) |
| `x86/gap_file.shard` `gap_run.shard` `gap_main.shard` | three functions deep: the hand-written `dot`, `sqdiff`'s module and `gap`'s compiled after it (names `gp_`); a statement at each seam, the module's theorem met twice and the theorem that the file exits with 54; the file written |
| `x86/total_file.shard` `total_run.shard` `total_main.shard` | the function that writes and its two callers: the hand-written `dot`, then the modules of `addto`, `both` and `total` compiled after it (names `tt_`); a statement at each seam, the module's theorem met three times and the theorem that the file exits with 75; the file written |

## The theorems proven once

`wrun_more` and `wends_call` (`ir_laws.shard`): a run with an outcome has it on any fuel
above; so where a callee's body answers `w` and leaves `m1`, and what follows the call answers
with the local set to `w` on `m1`, the call statement answers — the callee's fact used as
`wfn_ans` states it, the same statement the pass's theorem realizes. `wends_call_ext`: the
same for a function outside the module, by its answer. The rules of `iends`, for code that
names nothing outside, are these for every outside (`iends_call`, `iends_call_ext`).

`realizes_sub` and `realizes_part` (`x86/seam_laws.shard`): code that realizes an answer
realizes every answer that says no more, and realizes it in every table that has the code's
rows at their addresses. By the first, code proven against one form of its answer — a run of
intermediate code, a contract over lists — stands for a caller that runs with another; by the
second, a proof over a function's own rows holds in any file that has them.

`wmodule_realizes` (`x86/module.shard`): if `(compile xs p base)` answers a listing and that
listing stands in a table (`placed`), and each outside callee realizes its answer at the entry
and with the need `xs` gives, then every function of `p` realizes its own answer — `wfn_ans`,
the run of its intermediate code with what is outside the process — at the entry and with the
need the pass answered. The proof is one, over `wrun`: the law of a call is by the callee's
outcome, a return or the process's end. `module_realizes` is its corollary for code that names
nothing outside (`fn_ans`: the same run, for every outside, the outside left as it was), and
its statement is as it was. It covers every
form the pass accepts and nothing else: the pass refuses what it does not compile (a seventh
parameter, an expression deeper than the temporaries, a comparison other than unsigned below and not
below, a call's argument that is not a constant or a local, a call of itself that is not a tail
call).

`start_ends_top` (`x86/link.shard`) is the link for a function whose answer is an outcome: the
file ends as the outcome says (`models/x86/linux.shard` `exit_of`) — the function returned,
and the start-up exits with its word's low byte, the process having written what it then had;
or the process ended in the function. `start_ends` is that for the data's range alone, and
`start_exits_top` its corollary for an answer that names nothing outside:

`start_exits_top`: for the listing of any start-up — any data, any arguments —
with any lines after it, assembled, where the function the start-up calls realizes an answer
over the range from the data's first word to an address `top` of the stack's range, and that
answer has a value on the arguments' words in any memory that holds the data and is the start's
from the stack pointer up, the file exits with the value's low byte. `start_exits` is that
theorem with `top` the data's end, the answer asked of nothing above the data: what a file
that takes no arguments from its kernel is linked by. `link_exits` is that theorem for the sample: two buffers of
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
`x86/total_file.shard` those for three and the rows; in `x86/greet_file.shard` and
`x86/say_file.shard` those for the module, that `write_all`'s code stands in the table, and
that the table ends below 2^47.

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
- a file of the tree with data other than the sample's: the function each of seven files
  enters takes the sample's three words, so the first program's entry takes two addresses it
  does not use; a file whose data is not pushed by its start-up;
- of what is outside a process: a function that reads, a descriptor other than the standard
  output, a declared failure other than a failed write's, a call of the evaluator's host
  (`v3/std/host.shard`); a module behind a view whose answer is an outcome — the views state
  answers that name nothing outside; a rule of a tie for a store and for a tail call of an
  outside function — no tie here uses one; a program that writes what it computed: `say`
  writes bytes the kernel left, `greet` bytes its file holds, and no code here fills a buffer
  and writes it; and a tool that writes a tie: `greet`'s and `say`'s intermediate code and
  their ties are by hand, and the World a meaning takes is a parameter no code holds, which
  nothing in the language marks;
- a file's facts stated once for any module: each of the nine files states its own;
- one statement of the answer of a low form called from outside, for any signature: `two.shard`
  and `upd.shard` state it for a signature each;
- the first program as a module behind a view: its six functions are plain files.

## Tests

Fifteen entrypoints of the suite, in `v3/kernel/test/`: `compile_greet_test.sh` (the program
that writes: the chain accepted to the theorem for every list of the kernel's answers, that
theorem's right side against the model's run on twelve lists, and the processor's runs — the
text, a failed write's status, the process ended at a pipe nobody reads);
`compile_say_test.sh` (the program that writes in a loop: the chain accepted to the theorem
for every list of arguments and of answers, that theorem's right side against the model's run
in seventeen cases, and the processor's runs — the lines for four arguments and for two
hundred, nothing with no argument, a failed write's status, the process ended at a pipe nobody
reads, and a short write: 100,000 bytes to a pipe that does not block);
`compile_argsum_test.sh` (the program
of its arguments: the chain accepted to the theorem for every list of them) and
`compile_argsum_file_test.sh` (its file run with seven lists of arguments by the model and the
processor); `compile_pass_test.sh` (the machine's
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
each takes under two minutes, `compile_greet_test.sh` about three and `compile_say_test.sh`
about four:

```sh
EVAL=v3/bin/eval v3/kernel/test/compile_modules_test.sh
```

A fault planted in the pass is refused by its proof, one in a function, its code or its file by
the fact at that joint, a hand-written line changed in a file by the file's layout fact, one
changed together with its rows by its author's proof, a caller's proof that opens the body of
what it calls by the view, a low form that writes a buffer it has no evidence for and a caller
that passes one buffer twice where they are written, and a view that claims less written than
its function writes by the proof of what it leaves alone; none of this is a test in the suite.
