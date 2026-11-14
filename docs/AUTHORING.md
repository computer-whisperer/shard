# Writing shard

An ordinary program is a `.shard` file under a package root (`v3` here) that imports Init — Lean's
base library, read from a pinned export — and is written over Init's types. Its functions run, and
most of them are also definitions in the logic, so theorems can be stated about them. The forms
below are copied from files of the tree; where a table names no file, the form was run as written.
The system's own code under `v3/kernel` is another kind: written over its own prelude, it has no
meaning in the logic; section 8 shows how to call it.

## 1. A file

A file is the module named by its path from the root: `v3/examples/calc/calc_show.shard` is
`examples.calc.calc_show` and declares `examples.calc.calc_show.valI`. Directives act on the rest
of the file, in order. Abridged from that file:

```
(import "calc.shard")                 ; a file, relative to this one
(import "../../std/list.shard")
(use examples.calc.calc)              ; open a module's names
(use std.list)
(use Init.List cons nil)              ; open two names of Init's List

(fn valI_go ((ds (List Int)) (acc Int)) Int
  (measure (struct ds))
  (match ds
    (nil acc)
    ((cons d rest) (valI_go rest (+ (* 10 acc) d)))))
(fn valI ((ds (List Int))) Int (valI_go ds 0))
(theorem valI_snoc ((ds (List Int)) (d Int))
  (= (valI (List.append ds (cons d nil))) (+ (* 10 (valI ds)) d))
  (by (unfold valI) (exact (valI_go_snoc ds d 0))))
```

A theorem's proof is `(exact TERM)`, `(by STEP…)`, `auto` or `sorry`. A function may call one
declared later in the file; a theorem cites only what stands above it.

## 2. Imports and the Init horizon

| form | effect |
|---|---|
| `(import "FILE")` | loads the file once and makes its declarations citable by their full names; opens nothing |
| `(import "DIR")` | the same for a directory that has a view (section 8) |
| `(import Init NAME)` | the module sees the export's declarations up to and including `NAME` |

The export is ordered, and a module sees it up to its *horizon*: the farthest `NAME` among its own
`(import Init …)` lines and those of the files it imports. A file may have several such lines
(`v3/models/imp/word.shard` has ten); the farthest one sets the horizon, whatever their order. A
name past the horizon is refused with the line to add:

```
READ-ERROR FILE above_horizon: NAME is declaration N of the export, at or above the module's
horizon H (import Init K): write (import Init NAME)
```

`K` there is the file's last `(import Init …)` line, which need not be the one that set `H`. A
lemma a proof cites may lie anywhere in the export, far past every row below (`Nat.sub_one_lt` is
declaration 6,037, `Nat.succ_sub_one` 48,720): cite it, and add the line the refusal names.
Horizons that matter, in the export's order — each row covers the rows above it:

| import through | declaration | what it brings | example |
|---|---|---|---|
| `Int.decEq` | 835 | `Int` and `Nat` arithmetic and their decisions | `v3/examples/calc/calc.shard` |
| `Lean.Omega.Int.ofNat_lt_of_lt` | 1,508 | what `arith` builds its proofs from | `v3/examples/calc/calc_proof.shard` |
| `WellFounded.fix_eq` | 1,647 | a measured function as a definition (from `InvImage.wf`, 1,581) and its equations | `v3/pins/loader/by_wf/main.shard` |
| `UInt8.toNat` | 3,399 | the projections of `UInt8` and `ByteArray`, which the bytes library is written over | `v3/std/bytes.shard` |
| `Int.lt_mul_ediv_self_add` | 3,522 | `arith` over a product with a literal and over a quotient | `v3/examples/calc/natabs.shard` |
| `Nat.instMax` | 3,958 | `max` at `Nat` | — |

A file that imports `v3/std/bytes.shard` inherits its horizon (`UInt8.toNat`); one that imports
`v3/std/list.shard` inherits `List.take_succ_cons` (14,110). Section 15 shows how to find a
lemma's name and its position.

## 3. Opening names

| form | opens |
|---|---|
| `(use std.bytes)` | every name of module `std.bytes`: `Bytes.ofList` for `std.bytes.Bytes.ofList` |
| `(use Init.List)` | Init's `List` namespace: `cons`, `nil` |
| `(use Init.List cons nil)` | those two names only |
| `(use Init.Bool)` | `true` and `false`; without it they are written `Bool.true`, `Bool.false` |
| `(use examples.calc.calc_app.Action)` | the constructors of a type declared in another file |

A `type` opens its own constructors for the rest of its own file. `(use MODULE)` opens a module's
functions and types, not the constructors of its types: those take the last row's form. A bare
name two opened prefixes declare is refused `ambiguous_name`, and one Init also has may be read as
Init's: write the full name (`Exp.Add` in `v3/examples/calc/calc_spec.shard`, where `Add` alone is
a class of Init).

## 4. Types

| form | example | gives |
|---|---|---|
| `type` | `(type Token (TNum Int) (TPlus) (TMinus) (TBad))` (`v3/examples/calc/calc.shard`) | constructors with positional fields; `(type (Step S A) (Step S A))` takes parameters |
| `record` | `(record Mem (rep (Map Nat)))` (`v3/std/mem.shard`) | the constructor `MkMem`, an accessor `Mem.rep`, an updater `Mem.with_rep`; `(make Mem (rep …))` builds, `(with Mem m (rep …))` updates |
| `structure` | `(structure Monoid ((a Type)) (op (-> a a a)) (unit a) (op_assoc (forall …)) …)` (`v3/examples/template/package.shard`) | fields that may be functions or proofs; `Monoid.mk`, `Monoid.op` |

Values that run include `Nat`, `Int`, `Bool`, `List`, `Option`, `Prod`, `String`, `ByteArray`,
`UInt8`, `Fin`, `Subtype` and the program's own types. A string literal is a `String`; its bytes are
`(Bytes.ofString s)` (`v3/std/bytes.shard`; section 14 lists the library's functions). A proof
field or argument is erased: `(Fin.mk i h)` runs as `i` (`v3/examples/path/path.shard`).

## 5. Functions, and what makes one a definition

`(fn NAME (BINDERS) RESULT BODY)`. The body is built from calls, `match`, `if`, `let`, numerals and
`(dif h COND THEN ELSE)`, an `if` whose branches may use the proof `h`. A type parameter is a binder
`(T Type)` that calls do not write. A function of no parameters, `(fn sorted_nil () Sorted …)`
(`v3/std/list.shard`), is called by its name alone, `sorted_nil`; `(sorted_nil)` is accepted too.

The operators `+ * = < <=` work at `Nat` and `Int`; `-` at `Nat` stops at zero; `/` and `mod` are
total (`(/ 7 0)` is 0, `(mod 7 0)` is 7); `and`, `or`, `not` are for `Bool`. `max` at `Nat` needs
`(import Init Nat.instMax)`. A comparison is a `Bool` where the result is one (`is_ws` in
`v3/examples/calc/calc.shard`); the other spelling in the tree is `(if (= n 0) true false)`
(`is_zero` in `v3/tools/shardfmt/print/cst.shard`), where the bare `true` and `false` need
`(use Init.Bool)`. Test a `Bool` or a comparison with `if` and take any other value apart with
`match`. An `if` on a value of another two-constructor type (`Option`, `List`, a type of the
program) is accepted too, and takes the then-branch on the second constructor (`some`, `cons`),
under `--run` and in a native binary alike. A `Nat` is taken apart by numeral rows,
`(match n (0 A) (1 B) (_ C))`, or by `if`: the patterns `Nat.zero` and `(Nat.succ k)` are refused
in a `fn`.

The loader records what each function became:

| record | meaning |
|---|---|
| `DEFINE F equations=F.eq_1,…` | a definition in the logic, with its equations as theorems |
| `DEFINE F … pending=F`, `PENDING F measure` | defined, resting on obligations not yet proven (section 6) |
| `TEMPLATE F static=f equations=…` | a function with a function-typed binder: defined once, compiled per use |
| `RUNNABLE fn F why=REASON` | a program with no definition; a theorem that cites it is refused `unknown_constant` |
| `REFUSE F REASON: MESSAGE` | it should be a definition and is not; the load fails |

A function is a definition when its file imports Init, every type and callee it names has a meaning
in the logic, and it either does not call itself or says how it recurses. A call to an `extern` is
no obstacle. The equations are what proofs use: one per case of the leading `match` on a parameter
(`perform.eq_1` to `perform.eq_3` in `v3/examples/calc/calc_bin.shard`), otherwise one for the body
(`main.eq_1`). A lambda may appear only as the function argument of a template:
`(List.map (fun ((x Int)) (+ x n)) xs)` (`v3/examples/template/add_offset.shard`).

## 6. Recursion and measures

| the function | write | owes |
|---|---|---|
| calls itself on a piece of a parameter | `(measure (struct xs))` | nothing |
| calls itself on something smaller by a count | `(measure N)`, `N` a `Nat` expression over the parameters | one fact per self-call |
| is one of several that call each other | `(measure N)` on every one | one fact per call between them |

Each owed fact is an obligation `F.dec_K`. It says the callee's measure at its arguments is below
the caller's at its parameters, under everything known at the call. The numbers follow the loader's
walk of the body, which is not always the source's order: in `em_list` of the file below, `dec_1`
is the call in the last `else` and `dec_2` the call in the first `then`. Until it is proven the
records say `PARAM F.dec_K obligation` and `PENDING F measure`; a proof is
`(fulfills F.dec_K PROOF)` and gives `DISCHARGE F.dec_K proved`. The simplest case counts down
(`v3/std/mem.shard`):

```
(fn load_le ((w Nat) (m Mem) (a Nat)) Nat
  (measure w)
  (if (= w 0) 0 (+ (mem_get m a) (* 256 (load_le (- w 1) m (+ a 1))))))
(fulfills load_le.dec_1 (by (intro p h) (exact (Nat.sub_one_lt h))))
```

`Nat.sub_one_lt` lies past every horizon of section 2's table: that file has
`(import Init Nat.sub_one_lt)`. A count-down by another step, or by division, is closed by the two
`apply` rows of section 10; `(arith)` alone does not close them.

**Functions that call each other** are found by the loader; there is no form that declares them.
They are settled when the last one is read, so their `fulfills` lines, their callers' theorems and
anything citing their equations stand below it. The recipe, on the formatter's width probe
(`v3/tools/shardfmt/print/layout.shard`):

```
(fn flat_w ((n CNode) (b Nat)) (Option Nat)
  (measure (CNode.size n))
  (match n
    …
    ((CQuote m)
     (match (fit b 1)
       (none none)
       ((some b1) (flat_w m b1))))
    …))
(fn flat_items_w ((ns (List CNode)) (b Nat)) (Option Nat) (measure (List_CNode.size ns)) …)
(fulfills flat_w.dec_1 (by (intro n b m h b1 h2) (rw h) (rw CNode.size.eq_3) (arith)))
```

1. Give each function a measure. Where a function hands its argument on unchanged, add a rank:
   `em_if` calls `em_head` on the same list, so their measures are three times the list's size plus
   2 and plus 1. Write the product as a sum, `(+ s (+ s s))`.
2. Read the obligation: `(fulfills F.dec_K (by sorry))` prints it and lets the load go on —
   `PENDING F.dec_K.proof sorry goals=1 [⊢ (forall ((n Nat)) (-> (Not (< n 2)) (< (Nat.sub n 2)
   n)))]`. A step that fails prints its goal too (`step N: MESSAGE — the goal: GOAL`), so
   `(by (arith))` shows the same statement, but as a `READ-ERROR`, which ends the file's reading.
3. Introduce its variables in order: the function's parameters; then, for each `match` arm the
   call stands under, the arm's variables and the equation `SCRUTINEE = PATTERN` (`m` and `h`
   above, then `b1` and `h2`); for each `if`, the branch's condition (`c` in `flat_w.dec_2`); for
   a wildcard arm, one variable and its equation. A `let` takes no name.
4. `(rw h)` turns the parameter into the pattern, `(rw T.size.eq_K)` opens the size at the K-th
   constructor, and `(arith)` closes the comparison, which is then linear in the sizes.
5. Where the argument is a list a helper returned, prove once that the helper returns no more than
   it was given and pass the fact: `(arith (drop_blanks_le rest) (em_keep_rest … h))` (`em_head`).

A function that only calls itself states its obligations over its parameter when it has one, and
over one tuple `p` when it has several (`load_le` above).

## 7. Deriving a size

`(derive CNode size)` after the type declares `CNode.size` and, because `CNode` holds a
`(List CNode)`, `List_CNode.size` (`v3/tools/shardfmt/print/cst.shard`). A constructor's size is 1
plus the sizes of its recursive fields; the equations are `T.size.eq_K` in constructor order,
`List_T.size.eq_1` (nil is 1) and `List_T.size.eq_2` (cons is `1 + (size h + size t)`). The sizes
have no program; a measure may cite them, and the function still runs. This is the way to recurse
from a value into children it holds in a `List`, where structural recursion does not reach. A
function that only walks the list by its tail still says `(measure (struct ns))` (`em_keep`).

## 8. Walling off code that has no meaning in the logic

A directory `D/` with a *view* `D/mod.req.shard` shows its consumers only signatures, such as
`(sig fn reads_same ((a ByteArray) (b ByteArray)) Bool)` (`v3/tools/shardfmt/reads/mod.req.shard`).
The implementation is `D/D.shard`; it may import kernel files and write their types in full
(`kernel.prelude.List.Cons` beside Init's `List` in `v3/tools/shardfmt/reads/reads.shard`). A
consumer writes `(import "reads") (use tools.shardfmt.reads)` and sees `reads_same` as an opaque
function: its own functions are definitions, and its theorems hold for any implementation. Files
inside `D/` are private; importing one from outside is refused `private_module`. Give the directory
to the loader and to `v3/build.sh` after the root file, so that the implementation is checked and
linked: the record is `DISCHARGE D.f fn` for a linked function and `DISCHARGE D.f defined` where
the implementation is itself a definition (`tools.shardfmt.print.print`). A view may also state
`(sig type T)` and `(requirement NAME (BINDERS) STATEMENT)`, proven in the implementation by
`(fulfills NAME …)` (`v3/pins/loader/view_basic`).

## 9. A binary and its contract

The program is a `main` that takes the `World` last and returns it, over the host calls of
`v3/std/host.shard`; each call's result `World` goes into the next call. Where a proof is to
follow, write the calls nested, `(write_line B (write A w))`. A `let` chain,
`(let ((w1 (write A w))) (write_line B w1))`, runs the same, but the rewriting steps do not reach
under a `let`, and the proof must first restate the goal with the calls nested (section 10).

The contract stands in a file of its own beside the program. `v3/build.sh` is given the program's
file, the one that declares `main`; it never reads the `bin` form, and given the contract's file
it fails with `EMIT: no fn MODULE.main to start from`. That the binary is the program the contract
speaks of rests on building it from the file the contract imports. The contract
(`v3/examples/calc/calc_bin.shard`; abridged here from `v3/tools/shardfmt/shardfmt_bin.shard`):

```
(trusts write_reads write_output exit_exit …)
(opaque w_output ((w World)) (List ByteArray) nil)            ; an observable: never unfolded
(axiom write_output ((bs ByteArray) (w World))                ; what one host call does to it
  (= (w_output (write bs w)) (List.append (w_output w) (cons bs nil))))
(axiom read_file_output ((p ByteArray) (w World) (r (Option ByteArray)) (w1 World))
  (-> (= (read_file p w) (Prod.mk r w1)) (= (w_output w1) (w_output w))))
(fn run_ok ((reads …) (outs (List ByteArray)) (ex (Option Int))) Bool …)
(theorem fmt_main_contract ((w World))
  (-> (= (w_reads w) nil)
    (-> (= (w_output w) nil)
      (= (run_ok (w_reads (main w)) (w_output (main w)) (w_exit (main w))) true)))
  (by …))
(bin shardfmt (entry main) (externs get_args read_file write exit)
  (trusts …) (requires fmt_gate fmt_main_contract))
```

- **Observables** are opaque constants over the `World`: what was written, read, and the exit.
- **Axioms about host calls**, one per call and observable the proof uses. A call that returns a
  pair takes the premise form of `read_file_output`. Every axiom is named in the file's `(trusts …)`
  and in the bin's. `v3/examples/calc/calc_bin_model.shard` proves calc's nine on a model host.
- **Requirements** speak of `main`'s observables only. The formatter states its contract as a
  function into `Bool` over the three observables and proves it `true`: every case of a run has an
  answer, and the proof walks the program's cases once, a lemma per function. Calc states equations.
- The loader checks that every function uses each `World` at most once (`THREAD F ok`), that
  `externs` lists every host call `main` reaches, and that the requirements rest on no axiom outside
  `trusts` and Lean's own. A clean load prints `BIN tools.shardfmt.shardfmt_bin.shardfmt entry
  tools.shardfmt.shardfmt.main externs 4 trusts 9 requires 2/2`.

## 10. Proof steps that work

| to | write | in |
|---|---|---|
| open a function | `(rw main.eq_1)` or `(unfold valI)` | `calc_bin.shard`, `calc_show.shard` |
| compute both sides | `(reduce) rfl`; one side `(reduce lhs)` | `calc_reconcile_tests.shard` |
| split on a call's result and keep the equation | `(cases (get_args w) hg ((Prod.mk (lines w1) STEP…)))`, then `(reduce)` fires the match; where the call also stands under the match, `(reduce) (rw hr) (reduce)` | `calc_bin.shard`; `shardfmt_bin.shard` `fmt_gate` |
| split an `if` on a comparison | `(cases (Nat.decEq k 0) ((isFalse (h) (rw (if_neg h)) …) (isTrue (h) (rw (if_pos h)) …)))` | `layout.shard` `em_keep_le` |
| induct with another variable free | put `(forall ((acc Int)) …)` in the statement; `(induction ds ((nil () (intro acc) …) (cons (x r ih) (intro acc) … (rw (ih …)) …)))` | `calc_show.shard` |
| induct on a measure | `(wf n (Int.natAbs n) ih STEP…)` | `calc_show.shard` `show_correct` |
| close linear arithmetic | `(arith)`; with facts `(arith (strip_le a))` | `pins/loader/derive_size_plain` |
| use a lemma with premises | `(apply (natAbs_lt a b) (arith) (arith))` | `calc_show.shard` |
| rewrite by several rules | `(simp_only emitted_eq_spec next_state_eq_spec ih)` | `calc_app_trace.shard` |
| reason about a `main` written as a `let` chain | restate the goal with the calls nested, `(show (= (w_output (write_line B (write A w))) …))`, then `(simp_only write_line_output write_output)`. After `(rw main.eq_1)` the `let`s remain and no rule rewrites under them (`simp_no_progress`); `(reduce)` removes them but also computes every closed argument | — |
| close `n - k < n` in a decrease proof, `k` a positive numeral | `(apply Nat.sub_lt (arith) (decide))`: `(arith)` proves `0 < n` from the branch's condition, `(decide)` proves `0 < k` | — |
| close `n / k < n`, `k` a numeral above 1 | `(apply Nat.div_lt_self (arith) (decide))`: `0 < n`, then `1 < k` | — |
| use a truncated subtraction in arithmetic, where `(- n 2)` is an atom to `arith` | `(have hs (= (+ (- n 2) 2) n) (apply Nat.sub_add_cancel (arith)))`, then `(arith FACT… hs)` | — |
| find a lemma of Init and read its statement | section 15 | |
| give a witness; open one | `(apply (Exists.intro W) ((exact h)))`; `(cases h ((intro (x hx) STEP…)))` | `models/imp/app.shard` |
| restate the goal, then rewrite backwards | `(show G) (rw (<- h))` | `layout.shard` `opt_trail_rest` |
| run a function over given facts | `(reduce lhs (using ht len_ipfn.eq_1 (if_pos hd)))` | `models/imp/len.shard` |
| leave a goal open, or hand it to the search | `sorry`: the theorem is `PENDING NAME sorry` and admits nothing; as a step, `(by … sorry)` also prints the goal it stands at. `auto`: `prove.shard` writes the steps into `FILE.auto.shard`; it works for a `theorem`, not for a `fulfills` (`OPEN.md` §1) | `pins/loader/by_sorry`; `examples/auto` |

## 11. What goes wrong

| the loader or the build says | because | do |
|---|---|---|
| `RUNNABLE fn F why=group_measure` | functions call each other and one has no `(measure N)`; a `struct` clause does not count | a `Nat` measure on every one |
| `RUNNABLE fn F why=group_shape` | such a function has a type parameter, or a result type over its parameters | make it specific |
| `RUNNABLE fn F why=measure_type` | the measure is an `Int` | `(measure (Int.natAbs n))` (`calc_show.shard`) |
| `RUNNABLE fn F why=wf_unreached`; its `fulfills` is then `LOAD-ERROR FILE fulfills_outside_impl` | `WellFounded.fix`, `InvImage.wf` or `Nat.lt_wfRel` is past the horizon: importing through `arith`'s lemma alone does not reach them | `(import Init WellFounded.fix_eq)` |
| `RUNNABLE fn F why=callee_runnable` | it calls a function that is a program only | repair the callee, or put it behind a view |
| `REFUSE F realize_recursion: F calls itself without a measure` | no recursion clause | add `(measure …)` |
| `REFUSE F equation_form: T: a nested inductive's recursor has no Stage-0 translation` | structural recursion into a type nested through `List` | `(derive T size)` and a `Nat` measure |
| `DEFINE F equations=` with no name after it; then `READ-ERROR FILE unknown_constant: F.eq_1`, and `(unfold F)` is `unfold_stuck` | a structurally recursive body that starts with `if` has no equation | let the `match` on the recursion parameter lead (`em_keep`) |
| `REFUSE F definition_refused: F1.mutual: K refused the group's fixpoint (fvar_in_type)`, on every member of the group | a `let` bound to a call of a member, with another such call in its scope: an obligation is stated over the bindings in scope and cannot cite the recursion | write the call where it is used (`em_kids`, `em_if_deep`) |
| `REFUSE F unknown_head: Nat.succ is not a constructor — …` | a `fn` that matches a `Nat` by `Nat.zero` and `(Nat.succ k)` | numeral rows closed by `_` or a name, or an `if` |
| `REFUSE F unbound_name: true — read E-first after the typed route's unknown_constant: true` | bare `true` or `false` | `(use Init.Bool)`, or `Bool.true` |
| `REFUSE F unknown_head: max — … no_realization: a package constant whose value is not its constructor applied`; in a theorem `REFUSE T unknown_constant` or `decide_failed` | `max` at `Nat` below its instance; no message names the import | `(import Init Nat.instMax)` |
| `READ-ERROR FILE function_expected: applied to 1 more arguments than its type has: (Sort (succ ?u2))` | a bare constructor (`Add`) whose name Init also has, in a file that opens the type's module and not the type | `Exp.Add`, or `(use MODULE.Exp)` |
| `REFUSE F type_mismatch: expected (List.{0} UInt8), found (List.{0} Nat)` | `Bytes.ofList` given numbers | `Bytes.ofNats` (section 14) |
| `READ-ERROR FILE unknown_constant: step N: NAME — the goal: …` | a lemma's name is wrong, or is not Init's; no name is suggested | search the names (section 15) |
| `READ-ERROR FILE arith_failed: … — rows: [0 <= …] …` | the goal is not linear in the atoms the rows show: a truncated subtraction `(Nat.sub n 2)`, a quotient, a call | give the missing fact, or use the `apply` rows of section 10 |
| `READ-ERROR FILE cases_fields: … C's arm names N binder(s) and its case has M: …`; `induction_fields` likewise | an arm's name list has the wrong length; the two counts printed are not reliable (`OPEN.md` §1) | name every field of the constructor; under `induction`, then one hypothesis per recursive field: `(Add (a b iha ihb) STEP…)` |
| `READ-ERROR FILE arith_unreached: a product with a literal needs Lean.Omega.LinearCombo.mul_eval_of_const_right, past the Init prefix in scope: …` | the horizon is before `arith`'s product lemma | write a sum, or `(import Init Int.lt_mul_ediv_self_add)` |
| `REFUSE F function_value: …` | a lambda outside a template's function argument, or a function passed as a value | name it as a `fn`, or pass it to a template |
| `REFUSE F lean_symbol`, `REFUSE F infix_form` | `==`, `&&`, `\|\|`, or `(n + 1)` | `(= a b)`, `(and a b)`, `(+ n 1)` |
| `POLICY T outside the policy: A` | `T` rests on an axiom this file does not trust | `(trusts A)` in every file whose theorems rest on `A` |
| `UNTHREADED F world_twice: parameter 2 used twice: std.host.write_line, std.host.write_line` | one `World` passed to two calls | pass each call's result on |
| `BAD-BIN B extern_outside: X is called in F and is not in the roster`; `BAD-BIN B untrusted: T rests on A, which the bin does not trust`; `UNMET T` | the bin's `externs` or `trusts` misses a name; a requirement's proof is `sorry` | add the name; prove the requirement |
| `LOAD-ERROR FILE private_module: PATH is inside the sealed directory DIR — import the directory` | a file behind a view imported from outside | `(import "DIR")` |
| `v3/build.sh` prints `FAIL: NAME: the emit (exit 2); EMIT: no fn MODULE.main to start from` | the file given declares no `main` (the contract's file, say) | give the program's file |
| `v3/build.sh` prints `FAIL: NAME: the emit (exit 2); EMIT: MODULE.main takes N parameters: a binary's main takes the World alone (a main with parameters before the World runs under the loader's --run only)` | a `main` with parameters before the `World` is an entry for `--run` only | take the arguments through `get_args` |
| `v3/build.sh` prints `FAIL: NAME: the emit (exit 2); EMIT: the constructor C is named as a cell of the runtime and must not be the second constructor of its type` (or `must be the second`) | a constructor named `True`, `False`, `Nil`, `Cons`, `None`, `Some`, `Pair` or `World` stands at another place in its type than the runtime's cell of that name (`True`, `Cons`, `Some` are second constructors; the others are not) | rename the constructor, or move it |
| `v3/build.sh` prints `FAIL: NAME: the emit (exit 2); EMIT: the constructor C has N fields: a cell of the runtime holds at most 125` | a constructor of more than 125 fields | nest the fields in further types |
| `v3/build.sh` prints `FAIL: NAME: cc: FILE:LINE: error: …` and the path of the log | the C compiler refused the generated program; the line shown is its first error | read the rest in `v3/bin/NAME.build.log`: it is a defect of the back end |
| `v3/build.sh` prints `FAIL: NAME: changed during the build: FILE…` | a source of the program, or of the toolchain, was edited while the build ran; no stamp is written | build again |

## 12. The formatting gate

Every tracked `.shard` file under `v3/` that an author writes is kept in one canonical layout, and
the formatter's test fails on a file that is not. The layout law is `docs/CANON.md`: a line is 100
columns, and only the closing parentheses of enclosing forms, a comment, or an atom or string too
long by itself may run past it.

| command | does |
|---|---|
| `v3/fmt.sh --fix FILE…` | rewrites each file in place: `rewritten: FILE`, then `v3/fmt.sh: N files, K rewritten, 0 refused` |
| `v3/fmt.sh FILE…` | checks and writes nothing: `not canonical: FILE`, then `v3/fmt.sh: N files, K not canonical, 0 refused`; exit 1 when a file is not canonical |
| `v3/fmt.sh`, `v3/fmt.sh --fix` | the same over every gated file |

The gate sees a file once git tracks it; a file not yet tracked is checked by naming it. The proof
sidecars (`*.auto.shard`) and `v3/kernel/k/accel_pins.shard` are written by tools and are not
gated. Comments are kept as written. Nothing checks the order of a file's directives.

## 13. Build and run

```
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)        # the export and its index
E=./rust_bootstrap/target/release/eval; I="--init $EXPORT --init-index $INDEX"
$E direct v3/kernel/load.shard --root v3 $I FILE [DIR…]                        # check; prints the records
$E direct v3/kernel/load.shard --root v3 $I --run MODULE.main FILE [DIR…] -- ARG…   # run main
$E direct v3/kernel/load.shard --root v3 $I --store OUT FILE                    # keep the checked terms
$E direct v3/kernel/verify_release.shard OUT $EXPORT $INDEX                    # the checker alone on them
$E direct v3/kernel/prove.shard --root v3 $I FILE                              # write FILE's sidecar
v3/build.sh FILE OUT [--root v3] [DIR…]                                        # a native binary at OUT
```

`v3/bin/load ARG…` and `v3/bin/prove ARG…` are the same two programs compiled
(`v3/build.sh --all` builds them): the same arguments and records, faster. The compiled loader
differs from the bootstrap at one edge, an integer past 2^3840 (`OPEN.md` §1). `DIR…` are the
view directories whose implementations the program links
(`v3/tools/shardfmt/print v3/tools/shardfmt/reads`). A load's summary line is
`LOAD: modules N … runnable R  refused X  pending P … errors E`; a clean file has zeros for
refused, pending and errors. Files are named by paths under the working directory: a scratch file
goes inside `v3/`.

**The entry.** Under `--run` a `main` may take checked parameters before the `World`
(`(fn main ((i Nat) (w World)) World …)` in `v3/examples/path/path_main.shard`), read from `ARG…`.
A native binary's `main` takes the `World` alone and reads its arguments through `get_args`. The
build of a `main` with parameters is refused at the emit (`EMIT: MODULE.main takes N parameters:
a binary's main takes the World alone …`), and no binary is made.

**What a build leaves.** `v3/build.sh` is given the file that declares `main`. It writes the
binary at OUT and its stamp at `OUT.stamp`. Wherever OUT is, it also writes, under the name NAME
that is OUT's last component, the log `v3/bin/NAME.build.log` and the directory
`v3/bin/emit/NAME/` (the generated C, `map`, `sources`). Git ignores `v3/bin/`, so `git status`
does not show them, and two builds whose OUT ends in the same NAME overwrite each other's. When
the C compiler fails the console shows `FAIL: NAME: cc:` with the compiler's first error line and
the path of the log. A source edited while its build runs fails the build, and no stamp is
written: build again.

## 14. The library

`v3/std/README.md` describes every file of `v3/std`. What a program calls most, each with the
import and `use` that reach it; `a` and `b` are type parameters, which a call does not write.

**`v3/std/host.shard`**, `(use std.host)`: the `World` and the six host calls.

| name | binders, result | does |
|---|---|---|
| `get_args` | `((w World)) (Prod (List ByteArray) World)` | the command-line arguments |
| `read_file` | `((path ByteArray) (w World)) (Prod (Option ByteArray) World)` | a file's bytes; `none` when it cannot be read |
| `write` | `((bytes ByteArray) (w World)) World` | the bytes to standard output |
| `write_line` | `((bytes ByteArray) (w World)) World` | the bytes and a newline |
| `write_file` | `((path ByteArray) (bytes ByteArray) (w World)) (Prod Bool World)` | a file written whole; whether it succeeded |
| `exit` | `((code Int) (w World)) World` | the process ends with the code |

**`v3/std/bytes.shard`**, `(use std.bytes)`: bytes and text. A byte is a `UInt8`, a byte array a
`ByteArray`; both run as lists of small numbers.

| name | binders, result | does |
|---|---|---|
| `Bytes.ofString` | `((s String)) ByteArray` | a text's UTF-8 bytes; the way from a literal to something `write` takes |
| `Bytes.ofNats` | `((ns (List Nat))) ByteArray` | a byte array from numbers, each taken mod 256 |
| `Bytes.toNats` | `((b ByteArray)) (List Nat)` | its bytes as numbers |
| `Bytes.ofList` | `((l (List UInt8))) ByteArray` | a byte array from bytes |
| `Bytes.toList` | `((b ByteArray)) (List UInt8)` | its bytes |
| `Byte.ofNat` | `((n Nat)) UInt8` | `n mod 256` as a byte |
| `UInt8.toNat` | `((b UInt8)) Nat` | a byte's number (Init's function, given its program here) |

No function turns bytes into a `String`: a program's text is its literals, and its input stays
bytes.

**`v3/std/derive.shard`**, `(use std.derive)`: numbers as text.

| name | binders, result | does |
|---|---|---|
| `Nat.renderOnto` | `((n Nat) (rest (List Nat))) (List Nat)` | the decimal digits of `n`, as character codes, before `rest`: `(Bytes.ofNats (Nat.renderOnto n nil))` is `n` as text |
| `Int.renderOnto` | `((n Int) (rest (List Nat))) (List Nat)` | the same with a leading `-` for a negative number |

**`v3/std/list.shard`**: the programs of Init's list functions. They keep Init's names, so they
need the import and no `use`; without the import a `fn` that calls one is refused: `REFUSE F
unknown_head: List.length — read E-first after the typed route's no_realization: List.length: no
realization attached — realize it first`.

| name | binders, result | does |
|---|---|---|
| `List.length` | `((xs (List a))) Nat` | the number of elements |
| `List.append` | `((xs (List a)) (ys (List a))) (List a)` | `xs` followed by `ys` |
| `List.reverse` | `((xs (List a))) (List a)` | the elements in reverse order |
| `List.take`, `List.drop` | `((n Nat) (xs (List a))) (List a)` | the first `n` elements; the rest after them |
| `List.get?Internal` | `((xs (List a)) (i Nat)) (Option a)` | the element at index `i`, or `none` |
| `List.getD` | `((xs (List a)) (i Nat) (d a)) a` | the element at index `i`, or `d` |
| `Option.getD` | `((o (Option a)) (d a)) a` | the value inside, or `d` |
| `List.get` | `((xs (List a)) (i (Fin (List.length xs)))) a` | the element at an index proven in range |
| `List.map` | `((f (-> a b)) (xs (List a))) (List b)` | `f` over every element |
| `List.filter` | `((p (-> a Bool)) (xs (List a))) (List a)` | the elements `p` accepts |
| `List.foldl` | `((f (-> a b a)) (init a) (xs (List b))) a` | a fold from the left |
| `List.foldr` | `((f (-> a b b)) (init b) (xs (List a))) b` | a fold from the right |
| `List.any`, `List.all` | `((xs (List a)) (p (-> a Bool))) Bool` | whether some element, or every element, passes `p` |

With `(use std.list)` the file's own names are bare: twenty-four lemmas `list_*` that give Init's
list facts short names (`list_len_append`, `list_rev_rev`, …; the table is in
`v3/std/README.md`), `sum_append`, and a sorted list — `insert ((x Nat) (xs (List Nat))) (List
Nat)`, the type `Sorted`, `sorted_insert ((x Nat) (s Sorted)) Sorted`.

## 15. Finding a lemma of Init

A fact that is not linear arithmetic is usually a lemma of Init, cited under Lean's name
(`Nat.sub_lt`, `List.length_append`; `LEAN.md` list 1 gives the grammar of the names). No tool
looks a name up or prints a statement, and a wrong name gets no suggestion
(`unknown_constant: step 1: Nat.div_lt_slf`). Two things work.

**Names.** The Init index holds every name of the export with the position of its declaration.
Search it by a part of the name:

```
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)
grep -a 'Nat\.div_lt' $INDEX | grep -v '^_private' | tr -s ' '
```
```
Nat.div_lt_div_left 00045615
Nat.div_lt_div_of_lt_of_dvd 00037192
Nat.div_lt_div_right 00026975
Nat.div_lt_iff_lt_mul 00001053
Nat.div_lt_of_lt 00019897
Nat.div_lt_of_lt_mul 00008627
Nat.div_lt_one_iff 00038273
Nat.div_lt_self 00000580
```

The number is the `N` of `above_horizon`'s "declaration N": a file cites the lemma when it imports
Init through that lemma or a later one.

**Statements.** Cite the lemma where it cannot fit, and read its type off the refusal. A scratch
file under `v3/`, loaded as in section 13:

```
(import Init UInt64.ofBitVec_shiftLeft)   ; the export's last declaration: every lemma is in reach

(theorem probe () False (by (cases @Nat.div_lt_self ())))
```
```
READ-ERROR FILE cases_type: step 1: not an inductive type: (forall ((n Nat)) (forall ((k Nat))
(-> (LT.lt Nat instLTNat (OfNat.ofNat Nat 0 (instOfNatNat 0)) n) (-> (LT.lt Nat instLTNat
(OfNat.ofNat Nat 1 (instOfNatNat 1)) k) (LT.lt Nat instLTNat (HDiv.hDiv Nat Nat Nat (instHDiv Nat
Nat.instDiv) n k) n))))) — the goal: ⊢ False
```

Read it as: for `n` and `k`, from `0 < n` and `1 < k`, `n / k < n`. Each `forall` is an argument
and each `->` a premise, in the order a citation and `apply` take them. Lean's class spellings
stand for the operators: `(LT.lt Nat instLTNat a b)` is `(< a b)`, `(OfNat.ofNat Nat 0 …)` is `0`,
`(HDiv.hDiv Nat Nat Nat … a b)` is `(/ a b)`; a lemma stated in them matches a goal written with
the operators. Two limits:

- The print does not say which arguments are implicit. `(theorem probe () False (exact NAME))`,
  with no `@`, prints the type from the first argument a citation writes, the implicit ones before
  it as `?m1`, `?m2` (`type_mismatch: expected False, found …`, in the checker's notation, where
  `#K` is the K-th binder outward).
- A lemma with no argument is not a `forall`, and `cases` answers something else. The `exact`
  form prints any lemma's type.
