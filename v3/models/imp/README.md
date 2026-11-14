# `v3/models/imp` — an imperative intermediate language, and two functions proven against it

The model defines a small imperative language with calls, its evaluator, a reference-counting
runtime written in that language, and the relation between the runtime's memory and the lists a
source program computes with. Two source functions, `len` and `app`, each stand beside a lowered
form written out in the language, and a theorem says the lowered form computes what the source
computes. There is no compiler here: the lowered forms are literal values.

## The files

| file | what it holds |
|---|---|
| `imp.shard` | the kinds, operators, expressions and statements; `IpFn` and `IpProg`; the outcomes; the evaluator `ipeval` and the abbreviations `ipstmt`, `ipstmts`, `ipwhile`, `ipcall`, `iprun` |
| `rt.shard` | the spelling kit for runtime code; the four runtime functions `rt_init_fn`, `rt_alloc_fn`, `rt_inc_fn`, `rt_dec_fn` as statement lists; the table `rt_fns` |
| `heap.shard` | the ghost heap (`HCell`, `HRaw`, `GHeap`); `heap_rep`, memory represents it; `hinv`, it is well formed at the roots; the ghost operations `gh_inc`, `gh_fill`, `gh_seal`; `rinc`, `rdec` |
| `readback.shard` | the word kit (`imm`, `in63`, …); the readbacks `rb_int`, `rb_cell`, `rb_list`; the post-predicate `tbh`; `ldok` |
| `rtlaw.shard` | the six laws assumed of the runtime, as axioms |
| `word.shard` | lemmas on immediates: `imm_ofNat`, `imm_small`, `in63_ofNat`, `in63_ofNat_lt`, `imm_word`, `imm_odd`, `imm_lt` |
| `cell.shard` | lemmas on a live cell: its window in the heap (`cell_window`), the loads of its header and slots (`rd_hdr`, `rd_slot`), its header word's tag (`hword_tag`), the 32-bit band (`band32`) |
| `ghost.shard` | lemmas on the ghost operations: what the increment, the fill and the seal do to the cells, the end and the raw cell; readbacks kept by the increment and the seal (`rbl_inc`, `rbl_seal`); the raw cell fresh and inside the window (`raw_fresh`, `raw_window`); the roots' algebra; the table's entries (`rt_at_alloc`, `rt_at_inc`) |
| `step.shard` | three lemmas on the evaluator: `call_depth` (a call at the depth bound fails), `fuel_of_succ`, `ret_of_some` |
| `len.shard` | the source `len`, the lowered form `len_ipfn`, the run lemmas `len_call_one`, `len_call_fail`, `len_call_ov`, `len_call_ret`, the general form `tb_len_aux` and the theorem `tb_len` |
| `app.shard` | the source `app`, the lowered form `app_ipfn`, the run lemmas `app_call_nil`, `app_call_nil_fail`, `app_call_fail_app`, `app_call_fail_alloc`, `app_call_ret`, the lemmas `app_ghost` and `app_cons_ok`, the general form `tb_app_aux` and the theorem `tb_app` |

## The language (`imp.shard`)

A value — a word, an address, a local's index, a count — is a `Nat`.

| part | definition |
|---|---|
| kind `IKind` | `U8`, `U32`, `U64`; `ikw` its width, `ikmod` its modulus |
| operator `IOp` | `IAdd ISub IMul IDiv IRem IAnd IOr IXor IShl IShr IEq ILt ILe`; `iop_val` wraps add, subtract, multiply and shift-left to the kind, has no value for a division by zero or a shift at or past the width, and yields 1 or 0 for a comparison |
| expression `IExp` | `IConst`, `ILoc`, `IBin`, `IRotr`, `IExt`, `ITrunc`, `ILoad` (one byte); `iexp` evaluates one without fuel; a load outside the window `[mlo, msz)` has no value |
| statement `IpStmt` | `IpSet` (a local), `IpStore` (a byte), `IpIf`, `IpWhile`, `IpCall i k args` (local `i` := function `k`), `IpLoadW` and `IpStoreW` (a little-endian 64-bit word, its eight bytes inside the window), `IpFail` (a reasoned failure: `FOverflow`, `FOom`, `FStack`), `IpUnreach` |
| function `IpFn` | parameter kinds, extra-local kinds, body, result expression, result kind |
| outcome | of statements, `IpOut`: `IpNorm locals mem`, `IpTrap`, `IpFailed fam`; of a call, `IpRet`: `IpRv value mem`, `IpRtrap`, `IpRfailed fam` |

`ipeval fuel fs mlo msz dmax d task` is one function over a task — a statement, a statement list,
a loop, a call — recursing structurally on the fuel (`Fuel`: `FZ`, `FS`). One unit of fuel is spent
per statement, per loop test, per list step and per call; at `FZ` the result is `none`. A call
reads entry `k` of the table `fs`, traps on a wrong arity, fails with `FStack` when the depth `d`
has reached `dmax`, and otherwise runs the body on the arguments reduced to their kinds followed by
zeroed extra locals, then reads the result expression. `ipcall fuel …` is `ipeval` at `fuel_of
fuel` for a `Nat` fuel. Memory is `v3/std/mem.shard`'s `Mem`.

## The runtime and the heap (`rt.shard`, `heap.shard`, `readback.shard`)

`rt_fns rb` is the list `init`, `alloc`, `inc`, `dec` for a heap whose bookkeeping starts at address
`rb`: the heap's top at `rb`, its end at `rb + 8`, sixteen free-list heads from `rb + 24`. A
program's own functions follow in the table, so its first function is entry 4. A cell is a header
word — the reference count in the low 32 bits, the tag at bit 32, the arity at bit 48 — followed
by one word per slot. A count of 2^31 or more marks an immortal cell.

The ghost heap `GHeap` is the model's account of the heap: the live cells, sixteen free lists, the
window `[glo, gtop)` of `[glo, gend)`, and at most one raw cell under construction.

| predicate | says |
|---|---|
| `heap_rep m rb g` | the memory `m` holds the ghost `g` at base `rb`: the top, the end, the free lists, every cell's header and slots, the raw cell |
| `hinv rb g r` | `g` is well formed at the roots `r`: the bounds and alignment, the cells' extents disjoint and inside the window, every count, tag and arity in its band, every slot an immediate or the address of an older cell, every count the number of references to its cell, every root a cell |
| `rb_int w v` | the word `w` is the immediate of the integer `v`: `v` fits 63 bits and `w = imm v = (2v + 1) mod 2^64` |
| `rb_list cs w v` | `w` reads back as the list `v` in the cells `cs`: `1` for nil; for a cons, an even word that is the address of a live cell of tag 1 and arity 2 whose slots read back as the head and the tail |
| `tbh r rb msz g w rs ok` | the post-predicate of a call's result `r`. Returned `IpRv w2 m2`: `w2 = w`, `m2` represents `g`, `g` is well formed at the roots with `w` added if it is an address, no raw cell, the end within `msz`, and `ok`. A reasoned failure: true. A trap or no result: false |

## What is assumed (`rtlaw.shard`)

Six axioms under the file's `(trusts …)`, each recorded `PARAM NAME axiom`:

| axiom | says |
|---|---|
| `rt_alloc_run` | with fuel at least 12, a memory representing a well-formed ghost with no raw cell, a tag and an arity below 65,536: the call of entry 1 on the tag and the arity returns; it fails with `FOom`, or it returns an address `p` over a memory representing a well-formed ghost with the same cells and end and the raw cell `p` of that tag and arity, empty |
| `hr_fill` | storing a word below 2^64 into the raw cell's next slot gives a memory that represents `gh_fill g w` |
| `hinv_fill` | the fill keeps the invariant, the stored reference leaving the roots |
| `hr_seal` | the memory already represents `gh_seal g` |
| `hinv_seal` | sealing a full raw cell keeps the invariant, the new cell a root |
| `rt_inc_run` | with fuel at least 12: the call of entry 2 on an immediate or a live address returns 0 over a memory representing `gh_inc g v`, well formed with `v` added to the roots |

## What is proven

| theorem | hypotheses | conclusion | rests on |
|---|---|---|---|
| `tb_len` | `m` represents `g`; `hinv rb g r`; no raw cell; `rb_list (gcells g) a av`; entry 4 of `rt_fns rb ++ fs` is `len_ipfn`; `mlo ≤ rb`; `gend g ≤ msz`; fuel at least `24 + 12·|av|` | `tbh (ipcall fuel … 4 [a] m) rb msz g (imm (len av)) r (rb_int (imm (len av)) (len av))` | `propext`, `Quot.sound`, `Classical.choice` |
| `tb_app` | the same for `a`, `av` and `b`, `bv`; entry 5 is `app_ipfn`; fuel at least `25 + 12·|av|` | there are `g2` and `w` with `tbh (ipcall fuel … 5 [a, b] m) rb msz g2 w r (rb_list (gcells g2) w (app av bv))` | those three and the six axioms above |

In words: a call of the lowered form never traps and never runs out of the stated fuel; when it
returns, the word it returns reads back as the source function's value, over a memory that still
represents a well-formed heap. Each proof is an induction on the first list. Each case runs the
evaluator over one whole call with the facts of that call (a run lemma per outcome), using the
cell lemmas for the loads and, in `app`, the six axioms for the allocation, the two slot stores,
the seal and the increment.

## What is not proven

- The six axioms. `rt_alloc_fn` and `rt_inc_fn` are defined in `rt.shard`; no theorem shows that
  running them satisfies `rt_alloc_run` and `rt_inc_run`. The fill and seal laws are statements
  about `store_le`, `heap_rep`, `hinv`, `gh_fill` and `gh_seal`, also unproven.
- Anything about `rt_init_fn` and `rt_dec_fn`. That the memory represents a well-formed heap before
  the call is a hypothesis of both theorems; nothing shows that `init` establishes it.
- A compiler. `len_ipfn` and `app_ipfn` are written out in `len.shard` and `app.shard`. No function
  takes a source function to a lowered form, and no theorem ranges over source functions.
- Anything below the language: there is no machine-code model and no relation between `ipeval` and
  native code.
- Success. A reasoned failure satisfies `tbh`: an overflow of the 63-bit immediate in `len`, an
  exhausted heap in `app`, the depth bound in both.
- The laws of `load_le` and `store_le` as a library. `cell.shard` proves the loads it needs from
  `heap_rep`.

## The test

`kernel/test/imp_test.sh` loads `len.shard` and `app.shard` in one process and asserts: 14 modules,
nothing `RUNNABLE`, refused or pending; every function `DEFINE` with its equations (`ipeval` with at
least 17 and no descent obligation; `fuel_of` with its descent discharged); the recursive
specifications of `heap.shard` and `readback.shard` defined with two equations each; every lemma of
the kits and every run lemma accepted; `tb_len` accepted on Lean's three axioms and `tb_app` on
those and the six; exactly six `PARAM … axiom` records. The load is then stored and
`verify_release.shard` has the checker K alone accept every stored declaration, with six `PARAM` lines, three
`DISCHARGE` lines (`fuel_of`, `load_le`, `store_le`) and no `CONDITIONAL` or `UNDECLARED` line. The
test does not run the machine.
