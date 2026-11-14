# v3/pins/emit — the native toolchain's discriminating cases

Each case is a self-contained program (`main.shard`, a `main` over the World, printing to its
streams) that `v3/kernel/test/native_tie_test.sh` builds into a binary (`v3/build.sh`:
`load.shard --emit` — `kernel/lower.shard`, `kernel/cgen.shard` — then `cc`) and runs under both
engines — the Rust bootstrap's `eval direct` and the binary — tying standard output, standard
error and the exit status byte for byte. A case is what the back end must get right, or what its
runtime must agree with the bootstrap on. No case carries an expected text: the bootstrap is the
authority and the comparison is the verdict.

A case is one of two kinds by where its file lives, and a name marks a refusal:

- **A toolchain case** (`CASE/kernel/main.shard`): a file written as the system's own sources
  are — no Init, its own `List`, `Bool` and `World`. Its reference is the bootstrap's
  `eval direct` on the file.
- **An ordinary case** (`CASE/main.shard`, no `kernel/` directory): a program that imports Init.
  The tie loads it with Init and takes the V3 driver's run (`load.shard --run main.main`) as its
  reference, never the bootstrap's flat read, which dispatches by bare name — an unknown head an
  effect, `-` at `Nat` the integer's.
- **A `refused_*` case** states the back end's refusal in its first line,
  `;; emit refused: MESSAGE`. The tie expects the build to fail with `EMIT: MESSAGE` in place of
  a binary; nothing is run.

| case | discriminates |
|---|---|
| `record_with` | a record built with `make`, read through its accessors, updated with `with`: the loader's expansion reaches the back end, never the record form |
| `cap_local` | capitalised locals — a parameter, a let binding, a match variable — beside a type whose constructors share their spelling: every local printed by its depth |
| `nested_shadow` | a let rebinding one name three times, a match variable shadowing a parameter, a let inside an arm: the sequential let and the de Bruijn depths |
| `quoted_cap` | a capitalised symbol literal in a term and a pattern, a symbol with a quote character in it (`t'`) compared with `sym_eq`, operator-spelled symbols in patterns, the symbol primitives |
| `dotted_last` | two records' accessors and updaters sharing a last component (`A.x`, `B.x`): distinct identities, distinct C names |
| `sealed_client` | a consumer of a directory module's view, the implementation linked by the directory's check (`lib` as a second root): the linked program is what the back end receives |
| `program_exit` | a nonzero exit after output: the status and the streams tied, never the output alone |
| `string_escapes` | a quote, a backslash, a newline, a tab, a carriage return, UTF-8 bytes, an empty string, a string spelled like a form: each literal's bytes the same in the binary |
| `symbol_pattern` | a symbol with a quote character in it, in a pattern: a static of the generated text, compared by identity |
| `mangle_injective` | a record's accessor `A.x` beside a plain fn `A_x`: two identities, two C names |
| `refused_numeral_pattern` | a numeral pattern past 2^62: no C case label for it, the emit refused with its message, nothing built — the refusal path |
| `refused_main_params` | a `main` with a parameter before the World: a binary's start has the World alone to call it on, so the emit is refused, where the generated call would fail in the C compiler |
| `refused_wide_ctor` | a constructor of 126 fields, one more than a cell of the runtime holds: refused, where the binary would end in a segmentation fault |
| `wide_ctor` | a constructor of 125 fields, the most a cell holds, built 100,000 times and read back: the cells pass through a collection and are reused from the allocator's last size class |
| `refused_role_place` | a constructor named `Pair`, a first constructor among the runtime's cells, declared second in a type of the program: it carries the cell's tag, whose low bit an `if` reads as the place, so the emit is refused |
| `init_list` | an ordinary case: its own World, an extern over `ByteArray`, a string literal. Init's `Bool`, `List`, `Option` and `Prod` are the cells the runtime's externs build and read, so the literal needs no rewrite |
| `init_prim` | an ordinary case: Init's arithmetic at the values where the runtime's own operations differ — `Nat.sub` at an underflow, `Nat.div`/`Nat.mod` and the four `Int` divisions by zero and on negative operands, the bitwise five, `Nat.beq`/`ble`; `Nat.shiftLeft` and `Nat.shiftRight` by counts of 63, 64 and more, on small and large numbers, up to the last power of two a binary's integers hold, where the runtime's bare-named shifts stop at 63. Each row is silent when Init's result holds and prints its name otherwise; the last line is always printed |
| `if_second` | an ordinary case: an `if` on a value of a program's own type (without fields, with fields, with the fields on the first constructor, each declared after a type of three constructors), of Init's `Option`, `List` and `Bool`, and on a decision — each on its first constructor, then its second, printing the branch taken: the then-branch on the second constructor, read in a binary from the tag's low bit |
| `prims` | the toolchain's integer primitives at their edges (negative operands of `mod`/`tmod`/`ediv`, truncating `/`, the bitwise five, a product near 2^62): the binary's runtime against the bootstrap's |

`v3/kernel/test/lower_test.sh` also runs the ordinary cases `init_list` and `init_prim`, each as
loaded and in its lowered form, and requires the same output.

The loader's own cases — a malformed input, a refused declaration, a run out of fuel, a sidecar
written by `prove`, a store written and verified — are the tie script's workloads over the
drivers, not programs here.
