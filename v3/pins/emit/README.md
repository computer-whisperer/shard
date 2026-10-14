# v3/pins/emit — the native toolchain's discriminating cases

Each case is a self-contained toolchain-profile program (`main.shard`, a `main` over the
World, printing to its streams) that `v3/kernel/test/native_tie_test.sh` builds through
route 1 (`v3/build.sh`: `load.shard --emit`, kernel/emit.shard, then the old tree's chain)
and runs under both engines — the Rust bootstrap's `eval direct` and the binary — tying
stdout, stderr and the exit status byte for byte (LANGUAGE.md §8.6 slice 4.1 rule 3; R84,
records §4.12). A case is what the emitter must get right where the two dialects differ,
or what the chain's runtime must agree with the bootstrap on; no case carries an expected
text — the bootstrap is the authority, the comparison is the verdict — except a `refused_*`
case, whose first line `;; emit refused: MESSAGE` is the emitter's refusal the tie expects
in place of a binary (the bootstrap runs such a program; the chain never receives it).

| case | discriminates |
|---|---|
| `record_with` | a record built with `make`, read through its accessors, updated with `with`: the loader's expansion reaches the chain, never the record form |
| `cap_local` | capitalised locals — a parameter, a let binding, a match variable — beside a type whose constructors share their spelling: every local printed by its depth |
| `nested_shadow` | a let rebinding one name three times, a match variable shadowing a parameter, a let inside an arm: the sequential let (R44) and the de Bruijn depths |
| `quoted_cap` | a capitalised symbol literal in a term and a pattern, a symbol the old reader cannot lex as one atom (`t'`), operator-spelled symbols, the symbol primitives |
| `dotted_last` | two records' accessors sharing a last component (`A.x`, `B.x`) and a plain fn spelled `A_x`: the mangled heads distinct or the emit refused |
| `sealed_client` | a consumer of a directory module's view, the implementation linked by the directory's check (`lib` as a second root): the linked program is what the chain receives |
| `program_exit` | a nonzero exit after output: the status and the streams tied, never the output alone |
| `string_escapes` | a quote, a backslash, a newline, a tab, a carriage return, UTF-8 bytes, an empty string, a string spelled like a form: re-escaped for the old reader |
| `refused_symbol_pattern` | a symbol the old reader cannot lex, in a pattern: the emit refused with its message (the first line's `;; emit refused:`), nothing built — the refusal path of rule 1 |
| `refused_mangle_collision` | a record's accessor `A.x` beside a plain fn `A_x`: the mangling's collision refused, nothing built |
| `prims` | the profile's integer and symbol primitives at their edges (negative operands of `mod`/`tmod`/`ediv`, truncating `/`, the bitwise five, a product near 2^62): the chain's runtime against the bootstrap's |

The loader's own cases — a malformed input, a refused declaration, a run out of fuel, a
sidecar written by `prove`, a store written and verified — are the tie script's workloads
over the drivers, not programs here.
