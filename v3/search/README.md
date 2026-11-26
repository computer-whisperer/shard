# v3/search — search over programs with unknowns

`find` answers one question: *for which values, and which function bodies, does this goal
answer yes?* The goal is an ordinary shard function that returns a `Bool` (or a decision). What
is unknown is stated with what the language already has:

- **an unknown value** is a parameter of the goal: `(fn rev_is ((x (List Nat))) Bool (list_eq
  (rev x) (list 3 1 2)))` asks for the list that reverses to `3 1 2`;
- **an unknown function body** is a function the program names and does not define — an
  `extern`, or a view's `sig fn` with no implementation among the roots. A sketch is a function
  written as far as its author knows it, the rest left as such calls over what they may read.

Nobody writes a grammar. The ways to settle an unknown are read off the types the program
states: an unknown of an inductive type is one of its constructors over new unknowns; an
unknown `Int` or `Nat` is one of a list of integers (`--ints`); a hole function's body is a
parameter of its result type, a constructor of that type, an integer, or a call of a function
of the vocabulary (`--use`) whose result type fits — each over new hole functions for its
arguments.

```
v3/build.sh v3/search/find.shard v3/bin/find --native        # ~20 s
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)
F="v3/bin/find --root v3 --init $EXPORT --init-index $INDEX"
T=search.test.tasks

$F --goal $T.inv.rev_is --ints 0,1,2,3 v3/search/test/tasks/inv.shard
#  FOUND size=7
#    ARG 0 (List Nat) = (List.cons 2 (List.cons 1 (List.cons 3 List.nil)))
#  FIND: found size=7 work=163 …

$F --goal $T.sketch.rev_goal --hole $T.sketch.at_nil --hole $T.sketch.at_cons \
   --use $T.sketch.rev_acc v3/search/test/tasks/sketch.shard
#  FOUND size=6
#    FN (fn at_nil ((x0 (List Nat))) (List Nat) x0)
#    FN (fn at_cons ((x0 Nat) (x1 (List Nat)) (x2 (List Nat))) (List Nat) (….rev_acc x1 (List.cons x0 x2)))
```

An `FN` line is source text: it stands in the place of the hole function's `extern`, or in the
implementation of its view. `find` writes no file. The options and the records are in
`find.shard`'s header.

## How it works

| file | what |
|---|---|
| `narrow.shard` | the **narrowing machine**: `ev`'s machine (`v3/kernel/ev.shard` — the same linked program, the same primitives) over values that may be references to cells of a heap: an unknown, a suspended call of a hole function, or a settled value. It passes references around unread and stops, with the cell and the rest of the run in hand, where the program reads one that is not settled. Heap and continuation are values, so a stopped run is resumed in as many ways as there are ways to settle the cell, and all of them share what ran before |
| `task.shard` | the problem, and what makes a stopped search's state an answer: what is still undecided is filled with the first thing of its type, the hole functions are written into the program, and the goal is **replayed under `ev`**. Only an answer `ev` gives the same yes to is reported |
| `drive.shard` | the ways to settle a cell, and the walk: depth first under a size bound raised one at a time, a round's fuel set by what the costliest answer so far took. Also the comparison walk, `--enum`: every candidate decided whole before it is run |
| `find.shard` | the command line and the records |
| `test/` | `find_test.sh` (an entrypoint of `v3/test.sh`): the problems of `problems.txt` over the task files, against `expected.txt` |

## What a verdict claims

| verdict | claim |
|---|---|
| `found` | the goal answers yes on what is printed — under `ev`, replayed. The first answer found is among the smallest in what the runs needed decided |
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
| reverse's two hole functions from two examples | 3,501 | 1,245 |
| append's hole function from two examples | 1,178 | 593 |

Running a goal on something half decided pays where the unknown is data. On these two small
sketches it costs: a half-decided body can loop without asking for anything more, and each
such branch is run to the round's fuel.

## What is not here yet

- No production makes a `match` or an `if`: a hole function's body is built of parameters,
  constructors, integers and calls. A sketch states the case analysis.
- A primitive (`+`, `<`) is in a body only through a function of the vocabulary that wraps it.
- An unknown integer is tried at the listed values only; nothing solves for one.
- The goal is a program the author writes. Nothing yet turns a `requirement` into one, and
  nothing proves the requirement of what was found: that is `prove`'s part.
- The functions of this directory are programs without definitions, a handful of helpers
  aside: nothing is proven about the machine, and nothing rests on it — an answer rests on its
  replay.
