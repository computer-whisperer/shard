# The canonical form of source

A shard source file (`*.shard`) has one canonical text: its layout. This document states the layout
rules, the formatter that produces the layout, and the gate that holds every source file to it.

The layout is not the dump text (`load.shard --dump`), which encodes a loaded program to compare
two readers.

## 1. Layout

The layout follows the structure of the source; the width only decides when a form that could stay
on one line must break. A *form* is a parenthesised list, its *head* the first item when that is an
atom, its *arguments* the items after the head. A line is 100 columns, counted in bytes;
indentation is spaces. The last column names the worked examples that pin a rule: `NAME.in` and
its expected text `NAME.out` in `v3/tools/shardfmt/test/scenarios/`.

| | rule | scenario |
|---|---|---|
| 1 | A form that fits in what is left of its line prints on one line. | `fit`, `collapse` |
| 2 | A `match` with an arm never prints on one line: the scrutinee stays with `match`, each arm takes a line. | `match` |
| 3 | An `if` whose else-branch is an `if` never prints on one line: each link of the chain takes a line, at the first `if`'s column. | `chain`, `ifchain` |
| 4 | A form that breaks keeps arguments on its head line while each fits there whole. From the first that does not, every argument takes its own line, two columns in. | `keep` |
| 5 | A head listed in §1.1 keeps at most its count. So when the form breaks, a declaration's body, a `let`'s body and the steps of a `by` each start a line. | `break`, `measure`, `theorem` |
| 6 | A form made of atoms only fills: as many to a line as fit, the following lines two columns in. | `header` |
| 7 | A list that starts with a list (binders, `let` bindings, an arm with a constructor pattern, the cases of `induction`) puts its items one to a line, aligned under the first. | `theorem` |
| 8 | A broken `if` whose condition fits its line: the then-branch stays on that line if it fits, and the else-branch follows at the `if`'s own column. Otherwise both branches go two columns in, except an else-`if`, which stays at the `if`'s column. | `chain` |
| 9 | Closing parentheses pack onto the form's last line. A `)` has a line of its own only after a comment. | `inner` |
| 10 | Blank lines: at most two between top-level forms and one inside a form; none first or last in a form, none right after a head line. | `blanks`, `inner` |
| 11 | A comment keeps its bytes and is never rewrapped. On its own line it is indented like the items around it; after code it stays on that line, two spaces after the code. A form that holds a comment or a blank line breaks. | `comments`, `inner` |
| 12 | Atoms and strings print with the bytes they were written with: escapes, numerals, `'x` and `(quote x)` stay as written. A universe list `NAME.{u v}` prints on one line. | `primes`, `sugar`, `levels` |

Rules 4 and 5 (`keep.out`): `fn` keeps its name, binders and result; `append_all` has no row and
keeps three arguments; the fourth does not fit, so it and the fifth take a line each.
```
(fn message ((why (List Nat)) (at Nat)) (List Nat)
  (append_all (prefix_of_the_message why) (separator) (show_the_number_in_decimal at)
    (final_newline)
    (list 1 2 3)))
```

Rules 3 and 1 (`chain.out`): a chain, then an `if` that is no chain and fits.
```
(fn pick ((c Nat)) Nat
  (if (= c 1) 10
  (if (= c 2) 20
  30)))
(fn one ((c Nat)) Nat (if (= c 1) 10 30))
```

Rule 8, a then-branch that does not fit around one that does (`v3/tools/shardfmt/shardfmt.shard`):
```
           (if check
             (if (same_bytes (Bytes.toList out) (Bytes.toList src)) (exit 0 (write out w2))
             (exit 1 (write out w2)))
             (exit 0 (write out w2))))
```

Rules 5, 7 and 9 (`theorem.out`): the proposition and the proof each start a line; `by` keeps
nothing; the two cases align; the closers pack.
```
(theorem out_bytes_append ((xs (List (List Int))))
  (forall ((ys (List (List Int))))
    (= (out_bytes (List.append xs ys)) (List.append (out_bytes xs) (out_bytes ys))))
  (by
    (induction xs
      ((nil () (intro ys) (rw List.nil_append) (rw out_bytes.eq_1) (rw List.nil_append) rfl)
       (cons (x r ih) (intro ys) (rw List.cons_append) (rw out_bytes.eq_2) (rw (ih ys)) rfl)))))
```

### 1.1 The head table

The most arguments a head keeps on its line when its form breaks.

| head | keeps | on the head line |
|---|---|---|
| `fn`, `extern` | 3 | name, binders, result |
| `sig` | 4 | `fn`, name, binders, result |
| `def`, `abbrev`, `opaque`, `axiom`, `inductive` | 3 | name, binders, type |
| `theorem` | 3 | name, binders, proposition |
| `realize` | 3 | name, binders, result |
| `structure`, `requirement` | 2 | name, binders |
| `have` | 2 | name, statement |
| `fulfills`, `type`, `bin` | 1 | name |
| `match`, `if`, `let` | 1 | the scrutinee, the condition, the bindings |
| `measure`, `exact` | 1 | the term |
| `fun`, `forall` | 1 | the binders |
| `by` | 0 | nothing: each step takes a line |
| any other head | all | every argument that fits: `import`, `use`, `trusts`, `derive`, a call, a proof step (`rw`, `cases`, `induction`, …) |

### 1.2 The width and the tokens

A form stays on a line when its own text ends by column 100. Three things may run past it: the
closing parentheses of enclosing forms, which pack after it; a comment; an atom, a string or a
universe list too long by itself, which prints as it is. Non-ASCII text counts a column per byte.

The formatter splits text as the loader's reader does. A token ends at whitespace, a parenthesis,
`"`, `;` or a brace. `'` starts a quoted form only at the start of a token; elsewhere it is a name
character (`f'`: `primes.out`). An atom ending in `.` and followed by `{` opens a universe list,
closed by `}`; a comment inside one leaves the source without a layout (§2, exit 2).

## 2. The formatter

`v3/bin/shardfmt [--check] FILE` writes FILE's canonical text to standard output and never writes
a file. It is not trusted to be right: it prints, then asks the loader's own reader
(`v3/kernel/sexpr.shard`) whether the output reads as the input did, and emits it only if it does.
A printer defect can cost a file its looks or get it refused; it cannot change what the file
means. `v3/tools/shardfmt/README.md` describes the program and what is proven about it.

| exit | meaning |
|---|---|
| 0 | the text was written; with `--check`, FILE already is that text, byte for byte |
| 1 | `--check` only: the text was written and differs from FILE |
| 2 | no text: the usage; `shardfmt: cannot read file`; `shardfmt: the source does not read: REASON at byte N`; `shardfmt: the printer has no reading of this source` |
| 3 | no text: `shardfmt: the reformatted text reads differently; refusing to emit` — a printer defect to report |

## 3. Enforcement

**The gate.** `v3/tools/shardfmt/test/shardfmt_test.sh`, one of the tests `v3/test.sh` runs, formats
every gated source and fails when one is not its own canonical text, naming it (`not canonical:
FILE`). It also fails when the formatter refuses a file with exit 3, and when a source other than
the two pins broken on purpose (`v3/pins/loader/read_error/main.shard`,
`v3/pins/reader/refuse_unterminated.shard`) does not read.

**What is gated.** Every `.shard` file under `v3/` that git tracks, except the files a tool writes:
the proof sidecars `*.auto.shard` (written by `prove`) and `v3/kernel/k/accel_pins.shard` (its
generated section written by `v3/kernel/gen_pins.sh`). `v3/fmt.sh --list` prints the gated files. A
new file is gated once it is added to git.

**The command.** `v3/fmt.sh` builds `v3/bin/shardfmt` when it is missing or stale. A file the
formatter refuses is reported (`refused (exit N): FILE: …`) and left as it is.
```
v3/fmt.sh [FILE…]         # list the files that are not canonical; exit 1 if there is one or a file is refused
v3/fmt.sh --fix [FILE…]   # rewrite them in place; exit 1 if a file is refused
```
With no FILE, both forms take every gated file.

**After a rewrite.** The file reads as it did, so no declaration or proof changes. A declaration's
fingerprint (`hash=` on its `ACCEPT` record) is computed from the terms the checker holds, never
from source text, so it does not change either. The file's bytes did change, and a binary's stamp
hashes the bytes of its sources (`v3/build.sh`): binaries built from a rewritten file are stale.

## 4. Spelling of terms

One rule about how a term is spelled is enforced, by the loader: a number in a program body is a
numeral. `(Nat.succ Nat.zero)` in a `fn` body is refused, `REFUSE NAME nat_constructor`; write `1`.

Nothing else about spelling is in force. A set of rules for one canonical spelling of terms (no
control flow on a known value, one flat `let`, arms in declaration order, `if` in place of a
two-arm `match`, arrows and sorts in their short forms) was written for the previous layout law. No
tool checks them and the tree is not written to them. They are at the tag `v2-last`
(`v3/CANON.md` §4 and §5) for whoever builds a recogniser.

## Why

| chosen | rejected | because |
|---|---|---|
| a layout judged by three things: a small edit touches few lines, reading costs few lines, the rules are simple enough to predict | a layout judged by how it reads in an editor | the readers and writers of this tree are agents, and their tools read by the line |
| 100 columns | 80; a wider page alone | the tree is written at 100, and at 80 a quarter of the kernel's lines break; a wider page saves lines only by packing nested control onto long ones |
| a broken form keeps arguments on its head line | every argument of a broken call on its own line | measured on the tree: 78% more lines against 41% |
| a `match` and an else-`if` chain never on one line | a `match` on one line when it fits (about 7% fewer lines) | a nested decision on one line is the worst shape to edit |
| the reader judges the formatter's output | a printer trusted to be right | no printer defect can then change a file's meaning, and the proof is about a comparison, not a printer |
| the printer reads the text itself (`print/cst.shard`) | printing the loader's reader's trees | that reader drops comments, blank lines and the bytes of atoms; it stays the judge |
| `--fix` is a script over the binary | a mode of the binary that writes files | the binary's proven contract is one read and one write to standard output |
| tool-written files are not gated | formatting them too | the tools' tests compare a regenerated file with the committed one byte for byte, and a reformat fails them |
