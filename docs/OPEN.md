# Open

What is known wrong, what is missing, and what the rules ask for that nothing checks. One line
each, with who needs it. An item leaves this file when it is done; the commit says how.

"Read from the code" means the item comes from reading the source and has not been reproduced.

## 1. Known defects

| what | evidence |
|---|---|
| A binary and the evaluator differ at edges: a binary's integers stop at 2^3840, so an operation whose result reaches that — a `Nat.shiftLeft` among them — ends in `rt trap: bignum too large (exceeds 2^3840)` where the evaluator computes it; a stuck program exits 2 on the Rust bootstrap run directly, 4 under `load --run` and 3 as a binary. | reproduced |
| The compiled loader has the binary's integer limit: `v3/bin/load` ends with `rt trap: bignum too large (exceeds 2^3840)`, exit 3, on a theorem K evaluates past that size and on a `--run` that reaches it. The loader on the bootstrap accepts both. | reproduced: `(= (Nat.pow 2 4000) (* (Nat.pow 2 2000) (Nat.pow 2 2000)))` by `decide` |
| Two files that each give a program to the same function of Init cannot be roots of one load: the second is `LOAD-ERROR … duplicate_name: List.length`, and what imports it `import_failed`. Each loads alone. | reproduced: `load --root v3 v3/std/list.shard v3/examples/path/path_main.shard` (`examples/path/path.shard` and `std/list.shard` both realize `List.length`) |
| In a function that is only a program, an operator whose type is decided by a call of a function declared later in the file is left as written: `(- (late n) 5)` with `late` returning a `Nat` is integer subtraction, −2 at 3, where no `Nat` or `Int` is expected of it. With `late` declared first it is `Nat.sub` and gives 0. Nothing false is proved — such a function has no meaning in the logic — but a value declared `Nat` is negative at run time. | reproduced: `(fn early ((s Symbol) (n Nat)) Nat (let ((d (- (late n) 5))) d))` before `(fn late ((n Nat)) Nat n)` |
| A lambda whose body takes a value apart by constructor rows is refused: the function it is lifted to names the body's matcher, which is admitted after it — `REFUSE F.lam_1 unknown_constant: F.match_1`. A lambda with numeral rows or an `if` reads. | reproduced: `(fn f ((d Int) (xs (List (Option Int)))) (List Int) (List.map (fun ((o (Option Int))) (match o (none d) ((some v) (+ v d)))) xs))` over `std/list.shard` |
| The two readings of a lambda differ where a proof or a type argument in its body names a local that its program does not: the typed route captures the local as a parameter the program never reads, the classifier passes proofs over unread and does not, and the instance's name differs with the captured count. | reproduced: `proof_captures` in `v3/pins/readings/main.shard` (`load --both`) |
| A wrong name of an Init lemma gets no suggestion: `unknown_constant: step 1: Nat.div_lt_slf`. | reproduced |
| `v3/build.sh` names its log and its emit directory under `v3/bin/` by the output path's last component, wherever the output is: two builds of one name overwrite each other's. | reproduced |
| A `match` on an indexed family: a constructor row is refused `pattern_constructor: C is not a constructor of T`, where C is one; a bare field-less constructor row (`Vec.nil`) is read as a variable. | reproduced |
| After a load error among an implementation's directives its check goes on: the parameter is reported `IMPL-ERROR … impl_missing_fn`, then `IMPL DEFINE` and `DISCHARGE … defined` follow for the same function. The exit status is right; the later records mislead. | reproduced: pins `impl_load_error`, `impl_import_late` |
| The back end builds `main`'s `World` cell with one field whatever the constructor's field count (`cgen.shard`, `cg_main`), where the evaluator builds one zero per field; a `World` of two fields is not refused at the emit. | read from the code |
| Two host shims of the runtime (`rt_read_range`, `rt_exit` in `rt.h`) untag their integer arguments without checking that they are small integers: an integer past 62 bits would be read as a pointer's bits. | read from the code |
| A supplied `realize` that is no template and whose body is the first to take a template's instance may be refused "cited before its realization is attached" (`realize.shard`, `realize_order`); no pin has such a `realize`. | read from the code |
| K's `check_level` does not refuse a level metavariable when it is given no list of level parameters. Admission always gives the list; the `infer` that K's view offers to callers does not. | read from the code |
| The Rust bootstrap loads an identifier that names nothing as a free variable, and `eval dump` prints an undeclared bare type name as a type variable; a source of the toolchain with such a name is refused only when the V3 loader reads it. | reproduced on ordinary files; none in the toolchain |
| The text that says why a function got no definition is written and then dropped: where the loader prints `RUNNABLE fn NAME why=REASON` for a single function or for a group (`group_shape`, `group_measure`, `measure_type`, `callee_runnable`), the sentence `define.shard` built for that reason is not printed; it prints only for a recursive `def` and for a template. Two more texts can never print because an earlier check refuses first: `match_indexed` and the back end's string-pattern refusal. | read from the code; no run printed them |
| Messages that say less than the case: five `LOAD-ERROR` records of the loader carry no detail; `no_l_meaning: F: an extern has no L meaning` is printed for a supplied `realize` body that calls an extern, where an extern is an opaque constant; `unknown_in_body` says "a fn without a definition" for any program-side name. | read from the code |
| Small: the classifier offers a parameterized head `(NAME T…)` that it then refuses; `t0.shard` with `-p` and `--pins` prints only the `PINS` line; `define_test.sh` prints a constant `20 checks`; `v3/t0_full.sh` writes its log to the repository root, where nothing ignores it; `wf_proj` and `wf_proj_rest` in `define.shard` are one function written twice. | read from the code |
| Code nothing uses, still in place: in the formatter `drop_lead_blanks` only renames `drop_blanks`, and two proofs cite its equation; the reader's `empty_atom` error cannot occur. | read from the code |

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
| structural recursion for a group, or over a nested type | `equation_form: T: a nested inductive's recursor has a motive per auxiliary type, and structural recursion takes one — …`; a measure is used instead | nobody yet |
| a group with type parameters or a dependent result | stays a program (`group_shape`) | nobody yet |
| two implementations of one view | one implementation per view directory | nobody yet |
| rewriting in a hypothesis; case splits and cuts in arithmetic | not built | nobody yet |
| a witness through `exact` | `(exact (Exists.intro w h))` is `unify_stuck`; `apply` with `show` works | nobody yet |
| goal states kept between steps; shared holes | every query replays its block | a search engine working on goals |
| source positions on diagnostics | a failure names its step and goal, not a span | incremental goal states |
| a proof that the evaluator is right, and results proven by running it | not built | the compile system, possibly (§2) |
| a checked decoder from bytes to text | text enters a program as literals only | nobody yet |
| host calls to list a directory or read a key | not built | nobody yet |
| a constructor of more than 125 fields in a binary | refused at the emit (the allocator's size classes end there); the evaluator runs it | nobody yet |
| a constructor named `True`, `False`, `Nil`, `Cons`, `None`, `Some`, `Pair` or `World` at another place in its type than the runtime's cell of that name | refused at the emit: it carries the cell's tag, and an `if` reads the place from the tag; the evaluator runs it | nobody yet |
| a recogniser for the canonical layout | the gate compares the formatter's output | nobody yet |
| one canonical spelling of terms | only "a number is a numeral" is enforced; the older rules are not in force (`CANON.md` §4) | nobody yet |
| a search of Init's names, and a print of a lemma's statement | no tool: `grep` over the index's name table, and a refusal made on purpose that prints the lemma's type (`AUTHORING.md` §15) | every author |

## 4. Rules that nothing checks

Each is a rule of `LAW.md`, or follows from one, with no test behind it.

- The toolchain's own functions are meant to be total. Nothing checks their measures.
- What runs is written (rule 24): no load refuses a function whose two readings differ, nor one
  that leaves a runtime argument to the elaborator — the classifier refuses that call
  (`unwritten_argument`), the typed route reads it. The comparison runs only under
  `load --both`, over what a file writes — a `fn`, a lambda it lifts, a supplied `realize` —
  and `readings_test.sh` holds one fixture to it. Over the tree every such program reads the
  same both ways (`DECISIONS.md`, Measured). An instance is outside it: the classifier names
  the instance a call takes and does not read one — its program is the erasure of the
  template's elaborated body on both routes, so a template's source is read as a program by
  nothing. What the typed route reads and the classifier refuses, each reproduced: a
  package's field applied at its constant outside a template (`(Monoid.op IntAdd x y)` — the
  typed route unfolds the constant; one function of the pins); a lambda of fewer binders than
  its parameter's arity (`(fun ((a Int)) (Int.add a))` for a function of two); a type
  argument that only the type expected of the call fixes (`(pick inc List.nil)` at
  `(List Int)`).
- "Could not show these equal" is never read as "these differ": no test.
- A positive node id in K is written only by five constructors: no test. The test of K's state
  discipline believes the types the sources declare.
- A function either compiles or is refused: four pins show a program the evaluator runs and the
  compiler refuses (`v3/pins/emit/refused_*`); no test covers the rule itself.
- No run-time value from a non-constructive choice: the refusal exists, no pin exercises it.
- A user function or host call spelled like an operator: no pin. A `fn` named `lt` is defined and
  cannot be called (`renamed_primitive`).
- An unfinished proof does not fail a load, so a load can end clean with a theorem absent. Only a
  binary's contract insists.
- The layers above K are untrusted and K checks what they produce: no test hands K a wrong term
  from the elaborator. That those layers reach the environment only through K's view is tested.
- Which engine and route produced an acceptance is not recorded.
- Identity under a move of the tree: no test.
- "An agent with only the documents can write a program and prove something": no test. It is
  checked by hand, by giving an agent the documents and a task.
