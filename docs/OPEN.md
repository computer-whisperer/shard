# Open

What is known wrong, what is missing, and what the rules ask for that nothing checks. One line
each, with who needs it. An item leaves this file when it is done; the commit says how.

"Read from the code" means the item comes from reading the source and has not been reproduced.

## 1. Known defects

| what | evidence |
|---|---|
| A native binary takes the else-branch of every `if` whose condition is a value of a two-constructor type other than `Bool`: a type of the program (`(type Dec (No) (Yes))`), `Option`, `List`. The loader accepts such an `if` in a `fn` and defines the function, and the evaluator takes the then-branch on the second constructor (`Yes`, `some`, `cons`), as the language says. Programs that test only `Bool` and comparisons are unaffected. | reproduced: one program prints `then` for `Dec.Yes`, `some 1`, `cons 1 nil` and `true` under the evaluator on both engines, and for `true` alone as a binary (`cgen.shard` emits a test for the `true` tag only) |
| A `main` that takes parameters before the `World` cannot be built: the emit writes a call on the `World` alone and refuses nothing, and the C compiler rejects it (`FAIL: NAME: cc`, `too few arguments to function`). No binary is made; such a `main` runs under `--run` only. | reproduced on `v3/examples/path/path_main.shard` |
| A binary and the evaluator differ at edges: `Nat.shiftLeft` and `Nat.shiftRight` by a count of 64 or more trap in a binary, and the evaluator computes them (run at 64); a binary's integers stop at 2^3840; a stuck program exits 2 on the Rust bootstrap run directly, 4 under `load --run` and 3 as a binary; a constructor of 126 or 127 fields ends a binary in a segmentation fault. | reproduced |
| The compiled loader has the binary's integer limit: `v3/bin/load` ends with `rt trap: bignum too large (exceeds 2^3840)`, exit 3, on a theorem K evaluates past that size and on a `--run` that reaches it. The loader on the bootstrap accepts both. | reproduced: `(= (Nat.pow 2 4000) (* (Nat.pow 2 2000) (Nat.pow 2 2000)))` by `decide` |
| `v3/build.sh` writes a binary's stamp after compiling, from the sources as they are then: a source edited during the build is marked as built. | reproduced |
| `(fulfills NAME auto)` does not work end to end: `prove` writes the entry under the name `proof`, where a load looks for `F.dec_N`; and a load reads the sidecar only when the file also has an `auto` theorem. The obligation stays `PENDING F.dec_N.proof auto_missing`, and each `prove` searches again. | reproduced; with both mended by hand the load prints `DISCHARGE F.dec_N proved` |
| A stored proof's key includes every parameter the load has admitted so far, so one file can be reported stale (`auto_stale policy`) when it is loaded after another root that states an obligation. | reproduced |
| A directory behind a view is sealed against file imports only; a directory module nested inside it can be imported from outside. | reproduced; no pin |
| The reader answers the keyword `mutual` with "mutual recursion is deferred … one measured fn at a time", and the loader still labels a parked function `mutual_recursion`, though groups of functions are defined. | reproduced; `reader.shard`, `loader.shard` |
| The `cases_fields` and `induction_fields` messages print wrong counts: an arm that names one of a constructor's two fields is told `names 1 binder(s) and its case has 1`; an `induction` arm that names the two fields and not their two hypotheses, `names 2 binder(s) and its case has 2`. | reproduced |
| With several `(import Init …)` lines the `INIT` record and the `above_horizon` message name the last one, not the one that set the horizon: `horizon 1648 (import Init Lean.Omega.Int.ofNat_lt_of_lt)`, where that import alone gives 1509. After a late `(import Init …)`, which fetches nothing, the message contradicts itself: `Int is declaration 470 of the export, at or above the module's horizon 836`. | reproduced |
| A `let` bound to a call of a group member, with another such call in its scope, gets only `definition_refused: F1.mutual: K refused the group's fixpoint (fvar_in_type)`, on every member; nothing points at the `let`. | reproduced |
| `max` at `Nat` below `Nat.instMax` fails without naming the import it needs: `REFUSE NAME unknown_constant`, `decide_failed` or `unknown_head: max — … no_realization`, never `above_horizon`. | reproduced |
| A wrong name of an Init lemma gets no suggestion: `unknown_constant: step 1: Nat.div_lt_slf`. | reproduced |
| When the C compiler fails, `v3/build.sh` shows `FAIL: NAME: cc` and the log's last three lines; the compiler's error stands earlier, in `v3/bin/NAME.build.log` alone. | reproduced |
| `v3/build.sh` names its log and its emit directory under `v3/bin/` by the output path's last component, wherever the output is: two builds of one name overwrite each other's. | reproduced |
| About 180 refusal messages and the comments of 411 source files cite sections, slices and dates of documents that no longer exist. | counted |
| Dead code not yet removed: eight functions in K and `expr.shard`, three verdict constructors nothing produces, the `nat_operator` refusal in `prog.shard`, `v3/tools/rename_cmp.py`, demo functions in `std/list.shard`. | read from the code |

## 2. On the way to the trusted compile system

These are what the compile system has to replace or supply (`SHAPE.md` §6).

| what is missing | today |
|---|---|
| a proven compiler | lowering to C, unproven; compared with the evaluator on the tests |
| a proven step from a definition to its program | the erasure, trusted by review |
| proofs of the runtime's laws | six axioms in `v3/models/imp/rtlaw.shard`; nothing stated about `init` and `dec` |
| a compiler as a function, and a theorem over all programs | two compiled forms entered by hand |
| a machine model | none in this tree; the previous generation's is at a tag (`V2.md` §6) |
| a large computed result as a theorem | K's evaluation through a measured function gives out at depth 40 to 200; the rewriting step stops at 64 unfoldings |
| packed byte buffers and machine words | bytes and strings are lists; every number is an arbitrary-size integer |
| a binary's trace tied to its contract | a binary is compared on output and exit status only; the runtime writes no trace |
| a meaning in the logic for the toolchain's own sources | under `v3/kernel/` a string literal is a byte list and a symbol has no logical identity; no load checks that those functions terminate |
| a fixed identity for the `World` | an entry's last parameter is taken as the World whatever its type |

## 3. Missing, waiting for someone to need it

| what | today | who needs it |
|---|---|---|
| instance search | instances come from a fixed table; anything else is `instance_needed` | nobody yet |
| derive for more types | no types with parameters, no nested or mutual types | nobody yet |
| injectivity of constructors for a type with parameters | exists for types without parameters only | nobody yet |
| structural recursion for a group, or over a nested type | `equation_form: a nested inductive's recursor has no Stage-0 translation`; a measure is used instead | nobody yet |
| a group with type parameters or a dependent result | stays a program (`group_shape`) | nobody yet |
| two implementations of one view | one implementation per view directory | nobody yet |
| rewriting in a hypothesis; case splits and cuts in arithmetic | not built | nobody yet |
| a witness through `exact` | `(exact (Exists.intro w h))` is `unify_stuck`; `apply` with `show` works | nobody yet |
| goal states kept between steps; shared holes | every query replays its block | a search engine working on goals |
| source positions on diagnostics | a failure names its step and goal, not a span | incremental goal states |
| budgets and reclamation for a long-lived session | not built | nobody yet |
| a proof that the evaluator is right, and results proven by running it | not built | the compile system, possibly (§2) |
| a checked decoder from bytes to text | text enters a program as literals only | nobody yet |
| host calls to list a directory or read a key | the runtime has them; nothing declares them | nobody yet |
| a recogniser for the canonical layout | the gate compares the formatter's output | nobody yet |
| one canonical spelling of terms | only "a number is a numeral" is enforced; the older rules are not in force (`CANON.md` §4) | nobody yet |
| a search of Init's names, and a print of a lemma's statement | no tool: `grep` over the index's name table, and a refusal made on purpose that prints the lemma's type (`AUTHORING.md` §15) | every author |

## 4. Rules that nothing checks

Each is a rule of `LAW.md`, or follows from one, with no test behind it.

- The toolchain's own functions are meant to be total. Nothing checks their measures.
- "Could not show these equal" is never read as "these differ": no test.
- A positive node id in K is written only by five constructors: no test. The test of K's state
  discipline believes the types the sources declare.
- A function either compiles or is refused: one pin shows a program the evaluator runs and the
  compiler refuses (a numeral pattern past 2^62). A `main` with parameters is neither (§1).
- No run-time value from a non-constructive choice: the refusal exists, no pin exercises it.
- A user function or host call spelled like an operator: no pin. A `fn` named `lt` is defined and
  cannot be called (`renamed_primitive`).
- The list of loader pins the test runs is kept by hand; nothing compares it with the directory.
- An unfinished proof fails neither a load nor the release check, so a stored release can verify
  with a theorem absent. Only a binary's contract insists.
- Which engine and route produced an acceptance is not recorded.
- Identity under a move of the tree: no test.
- "An agent with only the documents can write a program and prove something": no test. It is
  checked by hand, by giving an agent the documents and a task.
