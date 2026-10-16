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
| `heap.shard` | the ghost heap (`HCell`, `HRaw`, `GHeap`), its representation in memory (`heap_rep`) and its invariant at the roots (`hinv`), with the vocabulary over cells and extents, and the runtime protocol's ghost operations (`gh_inc`, `gh_fill`, `gh_seal`, `rdec`; landing 2); the specifications are `def`s into `Prop`, the recursive ones with a recursion clause |
| `readback.shard` | the word kit (`imm`, `in63`, …), the readback relations (`rb_int`, `rb_cell`, `rb_list`), the post-predicate `tbh` of a per-function certificate, `ldok` |
| `word.shard` | the word kit's laws: the immediate of a small natural, `in63` of a cast; the immediate of any value odd and below 2^64 (landing 2) |
| `cell.shard` | the cell kit's laws: a live cell's window in the heap, its header word's tag, the loads of its header and slots through `hcells_rep`, the 32-bit band |
| `step.shard` | the evaluator's step laws: one per equation of `ipeval` over the structural fuel (`stmts_nil`, `stmt_set`, `stmt_storew`, `call_ret`, …), and the same over a Nat fuel through `fuel_of_succ` (`stmts_nil_n`, …) — the proof's vocabulary, never `reduce` over the evaluator; `ret_of_some` bridges a call's `Option` result to the result record |
| `len.shard` | the rung's object: the source `len`, the old compiler's product `len_ipfn` constructor for constructor, the run lemmas (the product's statements over their locals at a fuel base, each a K check of its own), and the theorem `tb_len` — proven at landing 1 through its general form `tb_len_aux` (induction on the list) |
| `rtlaw.shard` | the runtime's laws as declared parameters (landing 2; C2b-3 and C2b-4): the allocation call, the fill's representation and invariant, the seal's representation and invariant, the increment call — six `axiom` forms under `trusts`, each `PARAM NAME axiom`, discharged at phase 7; a run law's memory existential, the fill and seal laws one to one |
| `ghost.shard` | the ghost kit (landing 2): what the increment, the fill and the seal do to the cells, the end and the raw cell; the readbacks preserved by the increment and the seal; the raw cell fresh and inside the window under the invariant; the roots' algebra (`rinc`, `rdec`); the runtime table's entries; a rewriting kit of constructor forms |
| `app.shard` | the allocating member (landing 2): the source `app`, the product `app_ipfn`, the twelve run lemmas, the ghost chain `app_ghost`, the success leaf `app_cons_ok`, the general form `tb_app_aux` and the theorem `tb_app` — the output ghost and word existential, no twin; resting on Init's three axioms and the six parameters |

`kernel/test/imp_test.sh` loads the root's two roots, `len.shard` and `app.shard`, in one
process (Init on demand) and asserts the gate: every function and every specification
`DEFINE` with its equations, nothing `RUNNABLE`, nothing refused, nothing pending, the
evaluator one function over the task type on the structural fuel, `tb_len` accepted on
Init's three axioms and `tb_app` on those and the six parameters (each `PARAM NAME axiom`),
with the run lemmas, the step laws and the kits; then the root stored and the store verified
by K alone, the release's `PARAM` lines the six laws and the three measured descents. The
library files the rung added, `v3/std/map.shard` and `v3/std/mem.shard`, load with it. The
classes applied, every re-spelling, and what the proofs found in the kernel are recorded in
`LANGUAGE.md` §8.6 "as built, landing 0", "landing 1" and "landing 2".
