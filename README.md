# shard

shard is a language for programs and for proofs about them, and the toolchain that checks both.
Every declaration becomes a term of Lean 4's type theory and is checked by a small checker, K, that
implements Lean's kernel rules exactly. A function is at once a definition theorems can talk about
and a program that runs. The goal is programs whose behaviour is proven down to the code that runs.

## Where to read

| document | what it is |
|---|---|
| [`docs/SHAPE.md`](docs/SHAPE.md) | the system as it stands: its parts, what is solid, what is weak. Start here. |
| [`docs/LAW.md`](docs/LAW.md) | the rules the system and the work are held to; the trust base; the owner's rulings |
| [`docs/OPEN.md`](docs/OPEN.md) | known defects, what is missing, and who needs it |
| [`docs/AUTHORING.md`](docs/AUTHORING.md) | how to write a program and prove something about it |
| [`docs/LANGUAGE.md`](docs/LANGUAGE.md) | reference: every form, every proof step, every record the loader prints |
| [`docs/KERNEL.md`](docs/KERNEL.md) | reference: the toolchain's files, the checker, the trust base in detail, build and gates |
| [`docs/LEAN.md`](docs/LEAN.md) | for a reader who knows Lean 4: what is the same, what is refused and what to write instead |
| [`docs/CANON.md`](docs/CANON.md) | the layout every source file is kept in |
| [`docs/CHECKS.md`](docs/CHECKS.md) | each rule in detail with the test that holds it; the soundness mistakes made so far and the tests that pin them |
| [`docs/DECISIONS.md`](docs/DECISIONS.md) | design choices with the alternative each was chosen over |
| [`docs/V2.md`](docs/V2.md) | the previous generation: what it was, what it reached, what to fetch from its tags |

The documents describe the present. History is the commit log and two tags: `v2-last` (the previous
generation's tree and every document written before the cleanup) and `v2-kernel-last` (its kernel).

## The tree

| where | what |
|---|---|
| `v3/kernel/` | the toolchain, written in shard: the checker (`k/`), reader, elaborator, proof steps, loader, evaluator, compiler; its tests in `test/` |
| `v3/std/` | the library beyond Lean's `Init` |
| `v3/models/imp/` | a model of a low-level language and runtime, with two functions proven against their compiled forms |
| `v3/examples/` | `calc` and small worked examples of each feature |
| `v3/tools/shardfmt/` | the source formatter |
| `v3/pins/` | small sources, each stating the outcome it expects |
| `rust_bootstrap/` | the Rust evaluator the toolchain runs on |
| `tools/zed-shard/` | an editor grammar |

## Build, run, gate

Needs: Rust (`cargo`), a C compiler (`cc`, GCC), `git`, `curl`, `python3`, Linux, network access for
the first export, and about 30 GB of memory for the full replay only.

```sh
cargo build --release --manifest-path rust_bootstrap/Cargo.toml   # the Rust evaluator
v3/export.sh                       # Lean's Init, exported once into .shard-cache/ (installs Lean)
v3/build.sh v3/kernel/initindex.shard v3/bin/initindex            # the index builder, compiled
v3/kernel/test/init_index.sh       # the Init index, into v3/.cache/
V3_TEST_OUT=v3/bin/suite/bootstrap bash v3/test.sh   # the suite on the Rust evaluator
v3/build.sh --all                  # the 31 native binaries and the shim, into v3/bin/
V3_ENGINE=native V3_NATIVE=1 V3_TEST_OUT=v3/bin/suite/native bash v3/test.sh    # and on them
v3/kernel/test/suite_tie.sh v3/bin/suite/bootstrap v3/bin/suite/native   # the two runs compared
V3_T0_SKIP_FULL=1 v3/t0_full.sh    # K against Init: the tie; without the variable, all of it
# load and check one file; then run a program
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)
v3/bin/load --root v3 --init $EXPORT --init-index $INDEX v3/examples/calc/calc_spec.shard
v3/bin/load --root v3 --init $EXPORT --init-index $INDEX \
  --run examples.calc.calc_main.main v3/examples/calc/calc_main.shard -- "1+2" "10 - 3"
```

After editing a `.shard` file under `v3/`: `v3/fmt.sh --fix FILE`, then the tests that cover it;
before a push, both suite runs. A push of `.md` files alone is not gated by CI.
