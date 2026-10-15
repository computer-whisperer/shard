# `v3/models/imp` — the neutral imperative machine's call tier, ported for the tb_len rung

The old tree's `models/imp` (docs/IMP.md, docs/COVERAGE.md) under the naming law, as far
as `tb_len`'s closure reaches (`v3/LANGUAGE.md` §8.6 slice 4.2, rule 2; the port manifest's
rows for `models/imp/imp.shard` and the probes `tb_kit`, `tb_micro`, `tbh_kit`): the
definitions a per-function certificate is stated over, and nothing of the old files' laws —
those are landing 1's, written by need. The rest of the model — the base tier, the
well-kinded gates, the lifts, the length-preservation family, the ISA legs — is phase 5's.

| file | what it holds |
|---|---|
| `imp.shard` | the kinds, the operators, the expressions, the call-tier statements (`IpStmt`, a nested inductive: `IpIf` and `IpWhile` carry statement lists), `IpFn` and `IpProg`, the outcomes, and the evaluator `ipeval` — one function over a task (a statement, a statement list, a loop, a call), measured on the fuel, where the old file had four mutually recursive ones; `ipstmt`, `ipstmts`, `ipwhile`, `ipcall` and `iprun` are abbreviations of it |
| `rt.shard` | the counted-heap runtime: the spelling kit the compiler and the laws share, the four runtime functions as statement lists (init, alloc, inc, dec), the table `rt_fns` a program's functions follow |
| `heap.shard` | the ghost heap (`HCell`, `HRaw`, `GHeap`), its representation in memory (`heap_rep`) and its invariant at the roots (`hinv`), with the vocabulary over cells and extents; the specifications are `def`s into `Prop`, the recursive ones with a recursion clause |
| `readback.shard` | the word kit (`imm`, `in63`, …), the readback relations (`rb_int`, `rb_cell`, `rb_list`), the post-predicate `tbh` of a per-function certificate, `ldok` |
| `len.shard` | the rung's object: the source `len`, the old compiler's product `len_ipfn` constructor for constructor, and the theorem `tb_len` — pending on `sorry` at landing 0, proven at landing 1 |

`kernel/test/imp_test.sh` loads the root in one process (Init on demand) and asserts the
definition gate: every function and every specification `DEFINE` with its equations, nothing
`RUNNABLE`, nothing refused, the evaluator's nine descents discharged, the theorem alone
pending. The library files the rung added, `v3/std/map.shard` and `v3/std/mem.shard`, load
with it. The classes applied and every re-spelling are recorded in `LANGUAGE.md` §8.6 "as
built, landing 0".
