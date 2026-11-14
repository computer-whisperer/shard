# v3/tools/shardfmt — the formatter

`shardfmt [--check] FILE` prints a source file in the one canonical
layout (`v3/CANON.md` §3). It is the old tree's `tools/shardfmt` and
`meta/format`, migrated at slice 5.1 (`v3/LANGUAGE.md` §8.7) as an
ordinary V3 program: Init's types, the host's World, built by
`v3/build.sh` like any other.

```
v3/build.sh v3/tools/shardfmt/shardfmt.shard v3/bin/shardfmt v3/tools/shardfmt/print v3/tools/shardfmt/reads
v3/bin/shardfmt FILE > OUT          # the canonical text on standard output; FILE is never written
v3/bin/shardfmt --check FILE        # the same text; exit 0 when FILE already is it
```

| exit | meaning |
|---|---|
| 0 | the text was written and reads as the file does; under `--check`, the file is that text byte for byte |
| 1 | `--check` only: the text was written and differs from the file |
| 2 | no text: the usage, a file that cannot be read, a source the reader refuses (its reason and byte offset are printed) or one the printer has no reading of |
| 3 | no text: the printer's output reads differently from the file — the gate's refusal, a printer defect to report |

The arguments are exactly `FILE` or `--check FILE`. The old program
read `FILE --check` as a plain format and exited 0; this one answers
with the usage.

## The one promise

The formatter is not trusted to be right. It prints, then asks the
loader's own reader (`kernel/sexpr.shard`) whether the output reads as
the input did, and emits the output only if it does (`fmt.shard`). A
printer defect can cost a file its looks or get it refused; it cannot
change what the file means.

## The files

| file | what |
|---|---|
| `shardfmt.shard` | the program: the arguments, the one read, the one write, the exit |
| `fmt.shard` | `fmt`: print, then the gate |
| `shardfmt_bin.shard` | the contract: the binary declared with its two requirements proven (`fmt_gate`, `fmt_main_contract`), the observables and the nine bolts they rest on |
| `print/` | the printer behind a view (`mod.req.shard`: one opaque function). `cst.shard` reads a source into a tree that keeps comments, blank lines, the `'x` sugar and raw bytes; `layout.shard` prints it — the old printer carried function for function, the layout rules as ruled 2026-11-13; every function a definition, the renderers' three groups with their measures and descent proofs |
| `reads/` | the reader behind a view: `reads_same` and `read_error`, implemented by the loader's reader itself |
| `test/shardfmt_test.sh` | the gate of this tool: the scenarios, the command line, the program under `ev` against the binary, and every source of the tree |
| `test/scenarios/` | the layout's worked examples, `NAME.in` and `NAME.out`; nine sources are the old contract's, `keep` and `chain` pin the rules ruled 2026-11-13 |

Both views were made for the same reason: the formatter's statements
are about `fmt` and `main`, and those are definitions only if what
they call has a meaning in the logic. The reader is toolchain code,
which has none yet. The printer's renderers call one another in a
cycle, which V3 did not compile into a definition when the formatter
was migrated; since slice 5.2 it does, and the printer is one (below).
Its view stays: behind a view each is an opaque parameter, the
promise above never needed either body, and the contract is stronger
for holding of any printer.

## What is proven

`shardfmt_bin.shard` loads as `BIN … externs 4 trusts 9 requires 2/2`
(slice 5.1 landing 2):

- **`fmt_gate`** — whatever `fmt` returns for a source, the reader
  reads as it read the source.
- **`fmt_main_contract`** — from a world with nothing read and nothing
  written, the run ends in an exit, and exit 0 means exactly one file
  read, `fmt` accepting its bytes, and exactly one chunk written:
  `fmt`'s text. A nonzero exit promises nothing more.

Both hold for any printer and any reader: their assumptions are the
views' parameters and nine axioms about the four host calls, one per
call and observable the proof uses. Read those nine and the two
statements; nothing else in this directory needs trust.

**The printer is a definition** (slice 5.2 landing 4, `v3/LANGUAGE.md`
§8.7). Every function of `print/` has a meaning in the logic: the
loader records `DISCHARGE tools.shardfmt.print.print defined` — the
view's function discharged by the implementation, where it was linked
with nothing shown about it. So the printer the binary runs returns,
and is the function the contract's `print` stands for. Its renderers
call one another, so they are three groups, each one definition by
well-founded recursion: every function states a measure — the size of
the node or list it is given, with a rank where one hands its
argument on unchanged — and the 43 calls between them each owe a
proof that the measure got smaller (`print/layout.shard`). The
reader's half is still linked: it is toolchain code, which has no
meaning in the logic yet.

## What is not here yet

- **The scenarios as theorems.** They need more than a definition:
  evaluation of a closed call at a scenario's size, which the kernel's
  own evaluation does not reach through a measured definition
  (measured 2026-11-13; a deferred row of `v3/LANGUAGE.md` §11). Until
  then the scenarios are run against the binary.

## The gate, and the command

Every source of `v3/` that an author writes is kept in its canonical
layout: the formatter's test — an entrypoint of `v3/test.sh` — fails on
one that is not and names it (`v3/CANON.md` §8 stage 3, on since slice
5.1 landing 3).

```
v3/fmt.sh                 # list the sources that are not canonical
v3/fmt.sh --fix [FILE…]   # rewrite them in place
```

`v3/fmt.sh` builds `v3/bin/shardfmt` when it is stale. The rewrite is
the script's, not the binary's: the text replaces the file only when the
binary exits 0. Files a tool writes — the proof sidecars
(`*.auto.shard`) and `kernel/k/accel_pins.shard` — are not gated.

The layout rules are `v3/CANON.md` §3: 100 columns as the backstop; a
form that breaks keeps arguments on its head line while they fit, the
rest one a line; a `match` and an `if` chain never print on one line.
