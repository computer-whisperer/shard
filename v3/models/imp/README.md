# `v3/models/imp` — the neutral imperative machine's call tier, ported for the tb_len rung

The old tree's `models/imp` (docs/IMP.md, docs/COVERAGE.md) under the naming law, as far
as `tb_len`'s closure reaches (`v3/LANGUAGE.md` §8.6 slice 4.2, rule 2; the port manifest's
rows for `models/imp/imp.shard` and the probes `tb_kit`, `tb_micro`, `tbh_kit`): the
definitions a per-function certificate is stated over (landing 0), and the laws the
certificate's proof needed, written by need (landing 1). The rest of the model — the base
tier, the well-kinded gates, the lifts, the length-preservation family, the ISA legs — is
phase 5's.

| file | what it holds |
|---|---|
| `imp.shard` | the kinds, the operators, the expressions, the call-tier statements (`IpStmt`, a nested inductive: `IpIf` and `IpWhile` carry statement lists), `IpFn` and `IpProg`, the outcomes, and the evaluator `ipeval` — one function over a task (a statement, a statement list, a loop, a call), measured on the fuel, where the old file had four mutually recursive ones; `ipstmt`, `ipstmts`, `ipwhile`, `ipcall` and `iprun` are abbreviations of it |
| `rt.shard` | the counted-heap runtime: the spelling kit the compiler and the laws share, the four runtime functions as statement lists (init, alloc, inc, dec), the table `rt_fns` a program's functions follow |
| `heap.shard` | the ghost heap (`HCell`, `HRaw`, `GHeap`), its representation in memory (`heap_rep`) and its invariant at the roots (`hinv`), with the vocabulary over cells and extents; the specifications are `def`s into `Prop`, the recursive ones with a recursion clause |
| `readback.shard` | the word kit (`imm`, `in63`, …), the readback relations (`rb_int`, `rb_cell`, `rb_list`), the post-predicate `tbh` of a per-function certificate, `ldok` |
| `word.shard` | the word kit's laws: the immediate of a small natural, `in63` of a cast |
| `cell.shard` | the cell kit's laws: a live cell's window in the heap, its header word's tag, the loads of its header and slots through `hcells_rep`, the 32-bit band |
| `step.shard` | the evaluator's step laws: one per equation of `ipeval` over the structural fuel (`stmts_nil`, `stmt_set`, `call_ret`, …), and the same over a Nat fuel through `fuel_of_succ` (`stmts_nil_n`, …) — the proof's vocabulary, never `reduce` over the evaluator |
| `len.shard` | the rung's object: the source `len`, the old compiler's product `len_ipfn` constructor for constructor, the run lemmas (the product's statements over their locals at a fuel base, each a K check of its own), and the theorem `tb_len` — proven at landing 1 through its general form `tb_len_aux` (induction on the list) |

`kernel/test/imp_test.sh` loads the root in one process (Init on demand) and asserts the
gate: every function and every specification `DEFINE` with its equations, nothing
`RUNNABLE`, nothing refused, nothing pending, the evaluator one function over the task type
on the structural fuel, `tb_len` accepted on Init's three axioms with the run lemmas, the
step laws and the kits; then the root stored and the store verified by K alone. The library
files the rung added, `v3/std/map.shard` and `v3/std/mem.shard`, load with it. The classes
applied, every re-spelling, and what the proof found in the kernel are recorded in
`LANGUAGE.md` §8.6 "as built, landing 0" and "as built, landing 1".
