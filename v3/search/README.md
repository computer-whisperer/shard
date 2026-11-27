# v3/search — search over programs with unknowns

`find` answers one question: *for which values, and which function bodies, does this goal
answer yes?* The goal is an ordinary shard function that returns a `Bool` (or a decision). What
is unknown is stated with what the language already has:

- **an unknown value** is a parameter of the goal: `(fn rev_is ((x (List Nat))) Bool (list_eq
  (rev x) (list 3 1 2)))` asks for the list that reverses to `3 1 2`;
- **an unknown part of a function** is a hole, `(hole NAME)`, written where the part belongs
  (`docs/LANGUAGE.md`, "Terms"). A sketch is a function written as far as its author knows
  it. Nothing is declared for a hole: the load gives it its type and the names in scope there;
- **an unknown function** is a function the program names and does not define — an `extern`,
  or a view's `sig fn` with no implementation among the roots —, named by `--hole`.

A goal can also be a **property** (`--spec`): a function that must answer yes on *every*
argument, tried on all the small ones. Nothing of it is unknown; what is searched for is the
holes and hole functions that make it hold. That is how a whole function is asked for in one
line — the inverse of `rev` is `(fn unrev_spec ((xs (List Nat))) Bool (list_eq (unrev (rev
xs)) xs))` over `(fn unrev ((xs (List Nat))) (List Nat) (hole))`.

Nobody writes a grammar. The ways to settle an unknown are read off the types the program
states: an unknown of an inductive type is one of its constructors over new unknowns; an
unknown `Int` or `Nat` is one of a list of integers (`--ints`); a hole function's body is a
parameter of its result type, a constructor of that type, an integer, a call of a function
of the vocabulary (`--use`) whose result type fits, or a `match` — on a parameter, or on what
a function of the vocabulary answers (an `if`, where that is a `Bool`) — each over new hole
functions for its arguments or its rows. A body is in normal form: the matches come first,
each row ends in an expression without one. A function being found calls itself only on a
proper part of what it was given (a variable a `match` bound under a parameter): the
language's own structural rule, so what is found is printed with its `(measure (struct x))`
and loads as a definition.

A hole says more than a signature does, and the search uses it. Its filling is read off the
same types — the names in scope are its parameters — and the function it stands in, its
*owner*, is the function being found: from the owner's own body the search knows which of
those names are the owner's parameters, which are proper parts of one (bound by a match the
author wrote) and which the author has already taken apart. So a filling calls the owner only
on a proper part, and matches no name a second time.

```
v3/build.sh v3/search/find.shard v3/bin/find --native        # ~20 s
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)
F="v3/bin/find --root v3 --init $EXPORT --init-index $INDEX"
T=search.test.tasks

$F --goal $T.inv.rev_is --ints 0,1,2,3 v3/search/test/tasks/inv.shard
#  FOUND size=7
#    ARG 0 (List Nat) = (List.cons 2 (List.cons 1 (List.cons 3 List.nil)))
#  FIND: found size=7 work=163 …

# a sketch: (fn rev_acc ((xs (List Nat)) (acc (List Nat))) (List Nat)
#             (match xs (nil (hole at_nil)) ((cons x r) (hole at_cons))))
$F --goal $T.holes.rev_goal --use $T.holes.rev_acc v3/search/test/tasks/holes.shard
#  FOUND size=5
#    HOLE ….rev_acc.at_nil = acc
#    HOLE ….rev_acc.at_cons = (….rev_acc r (List.cons x acc))
#    MEASURE ….rev_acc (struct xs)

# the case analysis written, the answers left out — (match ys (nil (hole empty))
#   ((cons y r) (if (leq x y) (hole before) (hole after)))) — and a property:
$F --spec $T.holes.insert_spec --use $T.holes.insert --ints 0,1,2 v3/search/test/tasks/holes.shard
#    HOLE ….insert.empty = (List.cons x ys)
#    HOLE ….insert.before = (List.cons x ys)
#    HOLE ….insert.after = (List.cons y (….insert x r))
#    MEASURE ….insert (struct ys)

# the same with nothing in scope stated: the parts declared as externs over what they read
$F --goal $T.sketch.rev_goal --hole $T.sketch.at_nil --hole $T.sketch.at_cons \
   --use $T.sketch.rev_acc v3/search/test/tasks/sketch.shard
#  FOUND size=6
#    FN (fn at_nil ((x0 (List Nat))) (List Nat) x0)
#    FN (fn at_cons ((x0 Nat) (x1 (List Nat)) (x2 (List Nat))) (List Nat) (….rev_acc x1 (List.cons x0 x2)))

W=$T.whole     # whole functions: "x in another form", "the inverse of z"
$F --spec $W.rev_onto_spec --hole $W.rev_onto --use $W.rev_onto v3/search/test/tasks/whole.shard
#  FOUND size=6
#    FN (fn rev_onto ((x0 (List Nat)) (x1 (List Nat))) (List Nat) (measure (struct x0))
#         (match x0 (List.nil x1) ((List.cons x2 x3) (….rev_onto x3 (List.cons x2 x1)))))
$F --spec $W.unmirror_spec --hole $W.unmirror --use $W.unmirror v3/search/test/tasks/whole.shard
#    FN (fn unmirror ((x0 ….Tree)) ….Tree (measure (struct x0))
#         (match x0 (….Leaf x0) ((….Fork x1 x2 x3) (….Fork (….unmirror x3) x2 (….unmirror x1)))))
# from what it must keep — sorted stays sorted, one more x, every other as often as before:
$F --spec $W.insert_spec --hole $W.insert --use $W.insert --use $W.le --ints 0,1,2 v3/search/test/tasks/whole.shard
#    FN (fn insert ((x0 Nat) (x1 (List Nat))) (List Nat) (measure (struct x1))
#         (match x1 (List.nil (List.cons x0 x1))
#           ((List.cons x2 x3) (match (….le x0 x2) (Bool.false (List.cons x2 (….insert x0 x3)))
#                                                  (Bool.true (List.cons x0 x1))))))
```

A `HOLE` line is the expression for the hole's place, in the names of its scope (a variable
the filling binds itself is `xK`); `MEASURE F (struct x)` follows where the fillings make `F`
call itself — the recursion clause `F` then takes. An `FN` line is source text too: it stands
in the place of the hole function's `extern`, or in the implementation of its view. The
options are in `find.shard`'s header, the parts of a problem and the records in
`finder.shard`'s.

## Where what is found goes: the sidecar

```
$F --goal $T.holes.rev_goal --use $T.holes.rev_acc --write v3/search/test/tasks/holes.shard
#  …
#  WROTE v3/search/test/tasks/holes.auto.shard fill-for rev_acc holes=at_nil,at_cons
```

With `--write` the first answer's code for the holes of functions' bodies goes to the sidecar
of each function's file, `FILE.auto.shard`, the file `prove` writes proofs to
(`docs/LANGUAGE.md`, "A function's code in the sidecar"):

```
(fill-for rev_acc (fn 1631611479201210237) ((at_nil acc) (at_cons (….rev_acc r (List.cons x acc)))) (measure (struct xs)))
```

One entry a function: its name, the fingerprint of its form as the source has it, each hole's
code, and the measure where the code makes it call itself. The next load reads the function
with the code where its holes stand (`FILLED ….rev_acc holes=at_nil,at_cons (measure (struct
xs))`): it is defined, built and run as a function written that way, a theorem about it is
proven of it as filled, and `find` over the file meets no hole of it. Once the function's text
changes the entry is not its any more — `PENDING ….rev_acc fill_stale` — and its holes are
holes until `find --write` runs again. No source file is written, and the file's proofs in the
same sidecar are kept. What is not written, with its line: a hole without a name
(`NOT-WRITTEN F: …` — an entry stands under its hole's name); a function named by `--hole`
(its `FN` line is for its author to place).

## The search as a library

`finder.shard` is the search without a command line. A program that has a load asks it the same
question `find` does and gets a value back:

| call | gives |
|---|---|
| `(ground_of l)` | the load's program linked once, with the holes its functions hold — many problems are asked of one |
| `(ask_for GOAL SPEC)` | a problem with every part at its default: the goal's name as text, whether it is a property; `ask_option` and `ask_flag` set the other parts by the command line's words |
| `(find_in g ask)` | `FindNo WHY`, or `FindOk` with the verdict, the tally and the answers: for each, the unknown values as text, each hole's code and the function it stands in, each function's measure |
| `(found_lines r)` | the records `find` prints |
| `(write_fills l an w)` | an answer's code written to the sidecars; the `WROTE` and `NOT-WRITTEN` lines, and whether every function was written |

`find.shard` is that program: arguments, a load, these five calls, an exit code. No second
caller exists yet.

## How it works

| file | what |
|---|---|
| `narrow.shard` | the **narrowing machine**: `ev`'s machine (`v3/kernel/ev.shard` — the same linked program, the same primitives) over values that may be references to cells of a heap: an unknown, a suspended call of a hole function, or a settled value. It passes references around unread and stops, with the cell and the rest of the run in hand, where the program reads one that is not settled. Heap and continuation are values, so a stopped run is resumed in as many ways as there are ways to settle the cell, and all of them share what ran before |
| `task.shard` | the problem, and what makes a stopped search's state an answer: what is still undecided is filled with the first thing of its type, the hole functions are written into the program, and the goal is **replayed under `ev`**. Only an answer `ev` gives the same yes to is reported |
| `drive.shard` | the ways to settle a cell, and the walks. For a goal: depth first under a size bound raised one at a time. For a property: its tests, made from the parameter types by size, run in order, a branch ending at the first no; the bound is a depth (how deep a row's expression nests, and one less, how deep matches do); and a failure carries its *conflict* — the hole functions whose bodies the failing run entered — so a level whose hole function is no part of it is not tried another way: a function whose rows are reached by different tests is found row by row. A round's fuel is set by what the costliest answer so far took. Also the comparison walk, `--enum`: every candidate decided whole before it is run |
| `finder.shard` | the search as a library: the problem as asked (`Ask`), the holes of the load (`placed_holes`: each hole a run of the goal reaches, with what its owner's body says of the names it is handed), the answer as a value (`find_in`), the records (`found_lines`), the code into the sidecars (`write_fills`) |
| `find.shard` | the command over it: arguments, the load, `--batch`, `--closed`, `--write`, the exit code |
| `test/` | `find_test.sh` (an entrypoint of `v3/test.sh`): the problems of `problems.txt` over the task files, against `expected.txt`. `tasks/holes.shard`: sketches with holes in place; `tasks/sketch.shard`: the same with externs; `tasks/whole.shard`: whole functions from properties |

## What a verdict claims

| verdict | claim |
|---|---|
| `found` | the goal answers yes on what is printed — under `ev`, replayed. For a property: on every test, and on nothing more — a function found is consistent with the tests, and too few tests are satisfied by wrong ones (the test's problems show one); its law is still to be proven. The first answer found is among the smallest in what the runs needed decided |
| `none` | every branch ended, none cut by the size bound or by fuel: no answer over this vocabulary and these integers, at any size |
| `bounded` | no answer of the last size bound or less; `out=` counts the branches that ran out of fuel, which the claim does not cover |
| `gave_up` | the work budget ended first: nothing is claimed |

Work is counted in function entries, the machine's fuel, so a count means the same on every
engine.

## What is measured (2026-11-26, the test's problems; work in function entries)

| problem | lazy | generate and test |
|---|---|---|
| a list of four values by its reversal | 163 | 963 |
| a stack program by its result | 1,119 | 3,012 |
| the inverse of `rev`, whole | 1,832 | 2,777 |
| the inverse of a map, whole | 1,393 | 3,502 |
| the inverse of a tree's mirror, whole | 3,578 | 10,573 |
| reverse with an accumulator, whole, against the plain one | 12,672 | 26,966 |
| a tree's flattening with an accumulator, whole | 22,499 | 29,664 |
| insertion into a sorted list, whole, from three things it must keep | 4,437,150 (2 s) | not within 20 million |
| a sketch of reverse, two holes in place, from two examples | 923 | 3,412 |
| a sketch of append, one hole in place, from two examples | 360 | 542 |
| the inverse of a tree's mirror, the whole body one hole | 4,104 | 12,170 |
| insertion into a sorted list, the case analysis written and three holes | 14,847 | 13,339,528 |
| the sketch of reverse with its parts declared as externs | 17,497 | 1,881 |
| the sketch of append with its part declared as an extern | 3,704 | 833 |

Every search but the insertion is well under a second on the binary; a process is 2.3 s,
nearly all of it the load. Running a goal on something half decided pays where the unknown
is data and where a function is found whole: the rows of a match are decided by the tests
that reach them, and a failure revisits only what its run entered. The insertion shows it:
a match on a parameter, under it a match on a comparison, three rows — generate and test
does not reach it. Most of its four million entries go to the rounds before its own (deep
expressions under one match, which no body of that shape satisfies). With the case analysis
written and the three answers left as holes it is found in fifteen thousand: what the author
knows of the shape is the costly part of the search.

Where a sketch's parts are holes in place, the search knows the owner and calls it on proper
parts alone. Where they are declared as externs it knows a signature and no more: a hole
function calls the sketch's own function with nothing saying on what, a half-decided body
can loop without asking for anything more, and each such branch is run to the round's fuel —
the last two rows, the same two sketches as the first two of the four above them.

## On a machine model (2026-11-26, by hand: `test/tasks/span.shard`)

The unknown is a `(List Instr)` of the x86-64 model (`v3/models/x86`), folded through the
model's `exec` from three starting states; the goal reads `rax` at the end. Registers are
numbers in `Instr`, so `--ints 0,1,6,7` names `rax`, `rsi`, `rdi` and the immediate 1.

| wanted | lazy | generate and test |
|---|---|---|
| `rax := rdi` (one instruction) | found, 10,885 entries | found, 7,445 |
| `rax := rdi + rsi` (two) | found, 14.0 million, 9.7 s | found, 35.3 million, 17.1 s |
| `rax := 2*rdi + 1` (three) | not within 50 million | not within 50 million |

Work grows about 3.3 times a unit of size. With every example input under 256 a byte move
(`Movzx8`) passes as an answer: the examples must tell the wanted function from its near
neighbours. So `find` runs a real machine model as it is, and is of use there for spans of
one or two instructions; past that it needs what it does not have — two instruction lists
that leave every test state the same are one candidate, not two. The file is not among the
suite's problems: a load of the model is too long for the bootstrap.

## What is not here yet

- A function calls itself structurally or not at all; recursion on a number is not made.
- A found function is as good as its tests. They are every small argument in generation
  order, more of them where the property has more parameters; nothing looks for the argument
  that tells a found body from the wanted one, and nothing proves the property.
- A primitive (`+`, `<`) is in a body only through a function of the vocabulary that wraps it.
- An unknown integer is tried at the listed values only; nothing solves for one.
- A hole is filled where its function has no type parameter and the place gives it a type
  (`docs/LANGUAGE.md`); a filling binds no `let` and uses a function-typed parameter not at all.
- Code is stored for a named hole of a function in a file of the load's root; not for a bare
  `(hole)`, and not for a function inside a sealed directory. A function with stored code is
  searched again only once its text changes or its sidecar's entry is removed.
- The goal is a program the author writes. Nothing yet turns a `requirement` or a `theorem`
  into one, and `find` does not prove the property of what it found: with the code in the
  sidecar that is `prove`'s part, over a `theorem … auto` the author states. A property's tests
  are all the small arguments in generation order; a found function that fails a larger one
  is not caught here.
- The functions of this directory are programs without definitions, a handful of helpers
  aside: nothing is proven about the machine, and nothing rests on it — an answer rests on its
  replay.
