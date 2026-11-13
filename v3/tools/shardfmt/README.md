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
| `print/` | the printer behind a view (`mod.req.shard`: one opaque function). `cst.shard` reads a source into a tree that keeps comments, blank lines, the `'x` sugar and raw bytes; `layout.shard` prints it — the old printer carried function for function |
| `reads/` | the reader behind a view: `reads_same` and `read_error`, implemented by the loader's reader itself |
| `test/shardfmt_test.sh` | the gate of this tool: the scenarios, the command line, the program under `ev` against the binary, and every source of the tree |
| `test/scenarios/` | the layout's worked examples, `NAME.in` and `NAME.out`; the first nine are the old contract's, their texts unchanged |

Both views exist for the same reason: the formatter's statements are
about `fmt` and `main`, and those are definitions only if what they
call has a meaning in the logic. The reader is toolchain code, which
has none yet; the printer's renderers call one another in a cycle,
which V3 does not yet compile into a definition. Behind a view each is
an opaque parameter, and the promise above never needed either body.

## What is not here yet

- **The contract as theorems** (the old `fmt_gate` and
  `fmt_main_contract`): a `bin` with its observables, bolts and
  requirements, as `examples/calc/calc_bin.shard` has. Nothing in V3
  is missing for it; it is the slice's second landing.
- **The scenarios as theorems, and the printer's totality.** Both
  wait on V3 compiling mutual recursion into a definition. Until then
  the scenarios are run against the binary and the printer's functions
  are programs without a definition.
- **The gate on the tree.** `v3/test.sh` does not yet refuse a file
  that is not canonical; the test prints how many are. That is the
  slice's third landing, and it rewrites the layout of the V3 sources.
