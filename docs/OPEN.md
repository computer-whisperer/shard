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
| The loader's program reader does not know the parameter and result types of a function declared later in the same file (it records name, kind and arity before the bodies; the Rust reader has every signature). What depends on them is read as if untyped: `(- (late n) 5)` with `late` returning a `Nat` is integer subtraction, −2 at 3; a subtraction of two numerals passed to such a function is `Nat.sub` (`(- 0 1)` is 0 where the Rust reader gives −1); a comparison passed to one is not made a `Bool`; `(/ (late x) 2)` bound by a `let` is the table's bare `/`, which truncates and is stuck at a zero divisor, where the Rust reader gives `Int.ediv` — the bare `/` and `mod` entries are reached by no other source. With the callee declared first each is read right. Nothing false is proved — such a function has no meaning in the logic — and `parity_test.sh` fails on any toolchain function the two readers read apart; the toolchain writes negative numerals as literals for this reason. | reproduced: `(fn early ((n Nat)) Nat (let ((k (band 1 1)) (d (- (late n) 5))) d))` before `(fn late ((n Nat)) Nat n)`; five functions of the loader's closure before their sites were rewritten |
| A lambda whose body takes a value apart by constructor rows is refused: the function it is lifted to names the body's matcher, which is admitted after it — `REFUSE F.lam_1 unknown_constant: F.match_1`. A lambda with numeral rows or an `if` reads. | reproduced: `(fn f ((d Int) (xs (List (Option Int)))) (List Int) (List.map (fun ((o (Option Int))) (match o (none d) ((some v) (+ v d)))) xs))` over `std/list.shard` |
| The two readings of a lambda differ where a proof or a type argument in its body names a local that its program does not: the typed route captures the local as a parameter the program never reads, the classifier passes proofs over unread and does not, and the instance's name differs with the captured count. | reproduced: `proof_captures` in `v3/pins/readings/main.shard` (`load --both`) |
| A wrong name of an Init lemma gets no suggestion: `unknown_constant: step 1: Nat.div_lt_slf`. | reproduced |
| `v3/build.sh` names its log and its emit directory under `v3/bin/` by the output path's last component, wherever the output is: two builds of one name overwrite each other's. | reproduced |
| A `match` on an indexed family: a constructor row is refused `pattern_constructor: C is not a constructor of T`, where C is one; a bare field-less constructor row (`Vec.nil`) is read as a variable. | reproduced |
| After a load error among an implementation's directives its check goes on: the parameter is reported `IMPL-ERROR … impl_missing_fn`, then `IMPL DEFINE` and `DISCHARGE … defined` follow for the same function. The exit status is right; the later records mislead. | reproduced: pin `impl_load_error` |
| The program readers do not read a `let` binding at the type it states: `(let ((b Bool (< x y))) …)` is the bare comparison in the loader's program-first reading and in the Rust reader's, and the decision's value, `(if (< x y) true false)`, where the typed route reads the function (`classify.shard` `read_ebinds`, `settle_let`; `plain/read.rs` `read_elet`; a program's `let` carries no types). The two programs compute the same cell and are not one text. No toolchain function is written so: `toolchain_check.sh` fails on one that is. | found by `toolchain_check.sh` on `derive.shard` `dv_eq`, which now binds `(if (< 1 (len cs)) true false)` |
| The formatter's own lexer still makes a quote node of `'x` (`tools/shardfmt/print/cst.shard` `TQuote`, `CQuote`, `FQuote`, and two cases of `print/layout.shard` with their proofs): the reader refuses such a source before any layout, so no accepted text reaches the node. | read from the code; scenario `refuse_quote` |
| The back end builds `main`'s `World` cell with one field whatever the constructor's field count (`cgen.shard`, `cg_main`), where the evaluator builds one zero per field; a `World` of two fields is not refused at the emit. | read from the code |
| Two host shims of the runtime (`rt_read_range`, `rt_exit` in `rt.h`) untag their integer arguments without checking that they are small integers: an integer past 62 bits would be read as a pointer's bits. | read from the code |
| A supplied `realize` that is no template and whose body is the first to take a template's instance may be refused "cited before its realization is attached" (`realize.shard`, `realize_order`); no pin has such a `realize`. | read from the code |
| K's `check_level` does not refuse a level metavariable when it is given no list of level parameters. Admission always gives the list; the `infer` that K's view offers to callers does not. | read from the code |
| The Rust bootstrap runs a closure in which a function has no reading — an identifier that names nothing, an undeclared type: `eval dump` names the function and exits 3, `eval direct` says nothing and stops only if the function is reached. A source of the toolchain with such a name fails `parity_test.sh` and is refused when the V3 loader reads it. | reproduced on a scratch file; none in the toolchain |
| Two texts of a refusal can never print because an earlier check refuses first: `match_indexed` and the back end's string-pattern refusal. | read from the code; no run printed them |
| Messages that say less than the case: five `LOAD-ERROR` records of the loader carry no detail; `no_l_meaning: F: an extern has no L meaning` is printed for a supplied `realize` body that calls an extern, where an extern is an opaque constant; `unknown_in_body` says "a fn without a definition" for any program-side name. | read from the code |
| A function the checker reads that cites a function declared later in its file waits on a list, and the list is gone through again after every later function; and a checked function is read again once for each program-only callee its body names (`loader.shard` `load_fn_with`). The checker reads most of the toolchain's functions, so its sources pay both: the loader's own closure loads checked in 63 s on its binary, where the reading of programs alone takes 8. The 4,800 rows of `k/accel_pins.shard` are generated in an order where none cites a later one (`k/refgen.shard`); in the order they had, the same load was 299 s. Of the toolchain's 8,600 functions outside its tests, 3,500 cite a later function of their file. | measured (`DECISIONS.md`, Measured) |
| A toolchain symbol costs at run time what the engines' symbol did not: two are compared byte by byte (`sym.shard` `sym_eq`), and a `(sym "x")` or `(utf8 "x")` of a literal builds its list of bytes at every evaluation, where a quoted symbol was one cell made once. In the loader pins' harness these are 575 million of 3,231 million calls, and the harness takes half as long again on both engines; on CI the native tie, which waits for it, is 758 s for 485, and a pipeline 38 minutes for 31. Nothing is built against it: a back end that makes a literal's bytes one constant cell, or a symbol that carries its hash, are the two known ways. | measured (`DECISIONS.md`, Measured) |
| The record of a `type` that exists in programs only carries a reason that is none: `RUNNABLE type T why=bad_shape` or `why=bad_binder` is the extern reader's complaint about a form that is not an extern (`loader.shard`, `load_e_form`). | read from the code; the records of every toolchain root |
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
| a meaning in the logic for the rest of the toolchain's sources | the sources stand on Init's types and are read by the rules every file is read by; of the 8,834 functions of the loader's closure the checker holds 2,438 as definitions and reads the types of 4,963 more, which are programs because a function they call is one (`callee_runnable`, 4,841) or for their recursion — an integer measure, a group of functions that call each other (about 120). The other 1,433 it does not read: 1,388 have a signature over a type that is a program only (`e_only_type`) — a `type` used above its declaration in its file, and types that name each other, the three representations of a program among them (`prog.shard` `ETerm`, `ev.shard` `LTerm`, `lower.shard` `LwE`, each with its arm type) and the loader's state; 41 use a bit primitive with no meaning in the logic. 95 types are programs only. No load checks the types of those 1,433, nor the measure any of the programs states (§4) |
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
| a build's reading that passes a written proof over | under `load --unchecked` a `fn` that writes a proof is refused (`unchecked_proof`): nothing reads the proof there, though the checked load of the same file does. A toolchain function that writes one would load checked and fail its build | the first toolchain function that writes a proof; none does |
| a constructor of more than 125 fields in a binary | refused at the emit (the allocator's size classes end there); the evaluator runs it | nobody yet |
| a constructor named `World` standing second in its type | refused at the emit: it carries the World's tag, and an `if` reads the place from the tag; the evaluator runs it | nobody yet |
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
- The Rust evaluator reads every closure as the classifier does (`KERNEL.md`, The two
  engines): `parity_test.sh` holds it to the loader's text on every toolchain root, and
  `plain_test.sh` on six ordinary closures. Over the 59 roots of the examples, the library, the
  formatter and the model nothing holds it; measured there (`DECISIONS.md`, Measured), 51 read
  whole and no line differs. What it does not read, each named on standard error and left out
  of the program: a call of a template — so no lifted lambda and no instance; a `derive` and
  what calls a derived function, `=` at a derived equality among them; a `(realize NAME
  (view))`. It refuses nothing the loader refuses, and links a view to whatever implementation
  file stands in its directory without comparing the two.
- A function the typed route does not read is not type-checked: the program reader takes the
  declared types to choose an operator's entry, and compares no argument with its parameter.
  In the toolchain those are the functions whose signature names a program-only type, 1,433
  of the loader's 8,834. Their string literals are converted like every other
  (`(utf8 "…")`), and nothing holds them to it: an unconverted one would run, a `String`
  being the list of its bytes.
- The World's cell goes by a name in a native binary: the back end gives the World's tag to
  a constructor named `World`, in whatever type declares it, and refuses one that stands
  second in its type; the Rust evaluator takes the constructor of the type `main`'s parameter
  is typed at. A program whose World is spelled otherwise builds and runs (`(type W (MkW
  Nat))`, tried once); no test holds the two rules together.
- A program's `let` holds a list of bindings (`prog.shard` `ELet`, the Rust `Tm::Let`, the
  dump's `(let (E…) E)`), and both readers write one binding to a `let`: the engines' code
  for several is run by no source.
- The table of Init's facts the bootstrap reads the toolchain by (`v3/kernel/init.facts`) is
  believed. `parity_test.sh` compares it with what the loader prints from K — and that
  loader is run by the bootstrap, reading by the table: a table wrong in a way that made the
  loader print the same wrong table would pass. The loader's native binary, built from the
  loader's own reading, prints it too (the suite on the binaries).
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
