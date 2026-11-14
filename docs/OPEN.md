# Open

What is known wrong, what is missing, and what the rules ask for that nothing checks. One line
each, with who needs it. An item leaves this file when it is done; the commit says how.

"Read from the code" means the item comes from reading the source and has not been reproduced.

## 1. Known defects

| what | evidence |
|---|---|
| A binary and the evaluator differ at edges: a binary's integers stop at 2^3840, so an operation whose result reaches that — a `Nat.shiftLeft` among them — ends in `rt trap: bignum too large (exceeds 2^3840)` where the evaluator computes it; a stuck program exits 2 on the Rust bootstrap run directly, 4 under `load --run` and 3 as a binary. | reproduced |
| The compiled loader has the binary's integer limit: `v3/bin/load` ends with `rt trap: bignum too large (exceeds 2^3840)`, exit 3, on a theorem K evaluates past that size and on a `--run` that reaches it. The loader on the bootstrap accepts both. | reproduced: `(= (Nat.pow 2 4000) (* (Nat.pow 2 2000) (Nat.pow 2 2000)))` by `decide` |
| What the toolchain builds on by name is not held to a module's horizon, so a file's records can depend on the other roots of the load. A lemma a proof step builds on that lies past the file's horizon (`rw` on `Eq.mpr`) is refused when the file is loaded alone, with the import to write, and used when another root fetched it. A file that sees no Init gets its types' `noConfusion` and its functions' equations when another root fetched `Eq`, and neither when it is loaded alone. | reproduced: `(import Init Eq)` with a theorem by `(rw h)`; `(type Color (Red) (Green)) (fn pick ((c Color)) Color c)` with no import — each alone, and after a root that imports through `Int.decEq` |
| The Init names an implementation file cites are not fetched for it: the implementation's check starts from what was fetched for its view's forms, so a name below the horizon that only the implementation cites is `IMPL READ-ERROR DIR/DIR.shard unknown_constant: NAME` — and is found when another root, or a private file of the directory, fetched it. | reproduced: a view `(import Init UInt8.toNat) (sig fn pick ((n Nat)) Nat)` and an implementation with the same import citing `BitVec.instAndOp` (declaration 3,214 of the export): refused alone, accepted after a plain file that cites it; the same theorem in a plain file is accepted |
| A file import after a declaration that brings a farther horizon leaves the names the file cites between the two horizons unfetched: such a name is `above_horizon` when the file is loaded alone and found when another root fetched it. An implementation that imports a private file of a farther horizon than its view's is the same case. | reproduced: `(import Init Eq)`, a theorem, `(import "far.shard")` with `far.shard` importing through `Nat.sub_one_lt`, a theorem citing `Nat.sub_one_lt` — refused alone, accepted after a root that cites the lemma |
| In a function that is only a program, `-` on parameters declared `Nat` is integer subtraction: a body `(- n k)` gives a negative number for 3 and 5, under the evaluator on both engines and in a binary. The same body in a defined function is `Nat.sub` and gives 0. Nothing false is proved — such a function has no meaning in the logic — but a value declared `Nat` is negative at run time. | reproduced: `(fn prog ((s Symbol) (n Nat) (k Nat)) Nat (- n k))` |
| A wrong name of an Init lemma gets no suggestion: `unknown_constant: step 1: Nat.div_lt_slf`. | reproduced |
| `v3/build.sh` names its log and its emit directory under `v3/bin/` by the output path's last component, wherever the output is: two builds of one name overwrite each other's. | reproduced |
| A `match` on an indexed family: a constructor row is refused `pattern_constructor: C is not a constructor of T`, where C is one; a bare field-less constructor row (`Vec.nil`) is read as a variable. | reproduced |
| After a load error among an implementation's directives its check goes on: the parameter is reported `IMPL-ERROR … impl_missing_fn`, then `IMPL DEFINE` and `DISCHARGE … defined` follow for the same function. The exit status is right; the later records mislead. | reproduced: pins `impl_load_error`, `impl_import_late` |
| The back end builds `main`'s `World` cell with one field whatever the constructor's field count (`cgen.shard`, `cg_main`), where the evaluator builds one zero per field; a `World` of two fields is not refused at the emit. | read from the code |
| Two host shims of the runtime (`rt_read_range`, `rt_exit` in `rt.h`) untag their integer arguments without checking that they are small integers: an integer past 62 bits would be read as a pointer's bits. | read from the code |
| A supplied `realize` that is no template and whose body is the first to take a template's instance may be refused "cited before its realization is attached" (`realize.shard`, `realize_order`); no pin has such a `realize`. | read from the code |
| K's `check_level` does not refuse a level metavariable when it is given no list of level parameters. Admission always gives the list; the `infer` that K's view offers to callers does not. | read from the code |
| The Rust bootstrap loads an identifier that names nothing as a free variable, and `eval dump` prints an undeclared bare type name as a type variable; a source of the toolchain with such a name is refused only when the V3 loader reads it. | reproduced on ordinary files; none in the toolchain |
| The text that says why a function got no definition is written and then dropped: where the loader prints `RUNNABLE fn NAME why=REASON` for a single function or for a group (`group_shape`, `group_measure`, `measure_type`, `callee_runnable`), the sentence `define.shard` built for that reason is not printed; it prints only for a recursive `def` and for a template. Three more texts can never print because an earlier check refuses first: `match_indexed`, the back end's string-pattern refusal, `name_taken` for a declaration built as data. | read from the code; no run printed them |
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
| budgets and reclamation for a long-lived session | not built | nobody yet |
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
- "Could not show these equal" is never read as "these differ": no test.
- A positive node id in K is written only by five constructors: no test. The test of K's state
  discipline believes the types the sources declare.
- A function either compiles or is refused: four pins show a program the evaluator runs and the
  compiler refuses (`v3/pins/emit/refused_*`); no test covers the rule itself.
- No run-time value from a non-constructive choice: the refusal exists, no pin exercises it.
- A user function or host call spelled like an operator: no pin. A `fn` named `lt` is defined and
  cannot be called (`renamed_primitive`).
- An unfinished proof fails neither a load nor the release check, so a stored release can verify
  with a theorem absent. Only a binary's contract insists.
- Which engine and route produced an acceptance is not recorded.
- Identity under a move of the tree: no test.
- "An agent with only the documents can write a program and prove something": no test. It is
  checked by hand, by giving an agent the documents and a task.
