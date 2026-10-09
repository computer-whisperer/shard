# v3/pins/loader — the loader's corpus (phase 2, slices 3–7)

Each directory is a **package root**; `CASE/main.shard` is the file the
loader is given (`v3/kernel/load.shard --root v3/pins/loader/CASE …`),
its imports resolve under that root, and `(import Init NAME)` reaches
into the export fixtures (`v3/kernel/test/fixtures/`), which the test
streams once for every case (slice 3.25) — a module sees Init below its
own horizon, whatever the process has streamed. The header
line of `main.shard` states the load's expected outcome — the first
record that is not a module, an Init load or an acceptance:

```
;; expect: ok                          every record accepted (at least one; parameters, RUNNABLE declarations
;;                                     and the implementation's own declarations count)
;; expect: NAME refused REASON         K refused NAME for REASON
;; expect: NAME policy AXIOM…          NAME's axiom closure reaches AXIOM… outside the file's policy (§9)
;; expect: NAME pending sorry          read, admits nothing (a theorem's proof, or a fulfills)
;; expect: NAME refused REASON         also the classifier's refusal (slice 5): unsaturated, function_value,
;;                                     nonexhaustive, private_match, unknown_head, ambiguous_head, ambiguous_type, unbound_name,
;;                                     unknown_type,
;;                                     pattern_type, pattern_name, nat_constructor, name_taken, private_if, if_type (slice 10), no_list, …; a realize's (§7.5): unknown_constant,
;;                                     realize_kind, realize_primitive, realize_signature, realize_descent, realize_recursion, realize_order, no_l_meaning,
;;                                     no_l_identity, untyped_subterm, equation_form, equations_count, recursion_structure,
;;                                     no_realization, erased_in_runtime, noncomputable; an equation K refused is
;;                                     NAME.realize_N refused conversion
;; expect: NAME pending measure        a realization attached under a measure other than struct (§7.2): the obligation reported
;; expect: NAME runnable REASON       every record accepted, and the fn NAME is RUNNABLE with that obstacle to L meaning
;; expect: NAME refuses REASON         some record refuses NAME for REASON, not necessarily the first bad one (slice 3.16: a refused discharge after a PENDING measure)
;;                                     named (§8.4 rule 1, slice 3.13): e_only_type, no_l_meaning, no_l_identity, unknown_constant,
;;                                     measure_pending, numeral_pattern, recursion_depth
;; expect: NAME impl-error REASON      the implementation check (§6.6): impl_missing_type, impl_missing_fn,
;;                                     impl_missing_fulfills, impl_type_arity, impl_signature, fulfills_unknown
;; expect: read-error REASON           the reader refused (the file's reading ends there)
;; expect: load-error REASON           the loader refused: import_cycle, missing_file, missing_interface,
;;                                     import_outside_root, init_name_not_found, export_pin_mismatch,
;;                                     req_scope, view_form, sig_outside_view, duplicate_name,
;;                                     module_collision, fulfills_outside_impl, …
;; roots: FILE…                        what the loader is given, in order (default main.shard);
;;                                     a directory (lib) is its view and then its implementation checked
```

A `REALIZE` record (a realization attached, §7.5) counts as accepted, as `RUNNABLE` does,
and so does `DEFINE` (a fn defined with its equations, §8.4). The cases stream the export
through `InvImage.wf` (`fixtures/init_prefix_int.ndjson`, 100,851 lines; slice 3.13 cut it at
`Int.decEq`, 3.14 extended it for `WellFounded.fix`'s well-foundedness) — the operator
identities, the decisions and the well-founded constants live past the 3000-line prefix;
`init_not_found` asks for `Int.tdiv`, past it. The slice-3.13 refusals a header may state: `nat_operator`,
`recursion_depth` (as an obstacle), `define_matrix`, `realize_recursion` and
`realize_descent` for a fn.

Slice 3.19's cases (deriving, `LANGUAGE.md` §8.4): `derive_eq` and `derive_containers`
(equality generated and used by an `if`; Init's `List`, `Option`, `Prod` at closed
arguments), the refusals `derive_field`, `derive_nested` (`derive_shape`), `derive_params`
(`derive_needs`), `derive_policy`, `derive_duplicate`, `derive_by`, `derive_needs`,
`derive_sig` (`derive_type`: a view's sig type in a consumer, after the decision its view
exports is registered and used) — each stated as
`main.derive refused REASON` — and `derive_e_first` (`prim_type`: an integer comparison at
an inductive in an E-first body); `match_dependent` (a match generalizes a local
scrutinee in its expected type), `if_rec_call` and `measure_if_call` (an `if` whose
condition is a self-call; one obligation per statement), `bool_ops` (`and`, `not` on
`Bool`s). From the second reader's findings: `registry_native` (a registry row never
speaks for a native declaration bearing its name — under this test's one load of the whole
export the declaration is the refusal `Bool.or refused name_taken`, Init's names being
Init's, slice 3.25 rule 2; `define_test.sh`'s lazy load reads the finding's dump), `decidable_cells` (a `Decidable` value
is the type's cells off an `if`'s condition; `define_test.sh` runs it), `derive_name_taken`
(`NAME refused name_taken`), `derive_refusals` (what has no derivation; `define_test.sh`
reads its five records), `derive_visible` (an entry is visible where its deriving module
is; two roots). Orderings and renderings need Init past this fixture and the library:
`kernel/test/derive_test.sh`.

Slice 3.20's cases (I's opener, `LANGUAGE.md` §8.4): `by_core` (every form on a small type
and two list functions: the plain forms, `apply` with a premise block and with the middle
term given through the first premise, `cases` with its arms in one list, `unfold` and
`reduce`, `induction` with the hypothesis rewritten in, `decide`, `cases` on a term, a
reversed `rw` and an occurrence selector, a conditional lemma's premise as a block),
`by_wf` (a measured function's equation by `WellFounded.fix_eq`, the theorem by `wf` with
the hypothesis at `n - 1`, the descent obligation discharged by a block — the export's
second fixture, which holds `fix_eq`), `by_sorry` (`NAME pending sorry goals=2`), and the
refusals, each `read-error REASON`: `by_goal_open`, `by_goal_closed`, `by_unknown_step`,
`by_intro_no_pi`, `by_rfl_failed`, `by_apply_goals`, `by_witness` (`by_apply_unsolved` until
slice 3.22: the witness goal is built, the pin expects `ok`), `by_cases_rows`, `by_cases_fields` (`induction_fields`), `by_rw_no_match`
(syntactic matching: `a + b` finds no `b + a`), `by_rw_occ` (`(occ K)` past the count),
`by_unfold_stuck`, `by_decide_failed`, `by_show_mismatch`, `by_reduce` (`reduce_stuck`: an
unfolding equation is not a computation rule). The consumer is `kernel/test/tactic_test.sh`
over calc's files.

Slice 3.21's cases (`arith` and `simp_only`, `LANGUAGE.md` §8.4): `arith_core` (`Int` and
`Nat` rows, a certificate given and one reconstructed, an equation by one normal form and by
`≤` both ways, a goal `¬ P` and `False`, a fact as a term, Init's spelling), `arith_div` (a
product with a literal, the quotient and the remainder by one: the second fixture to its
end), and the refusals `arith_failed` (the rows shown in normal form), `arith_certificate`,
`arith_goal`, `arith_fact`, `arith_unreached` (the product's lemma past the prefix);
`simp_core` (list order, premises by assumption, a hypothesis and its reverse as rules),
`simp_no_progress`, `simp_budget`. What 3.20's second reader found, pinned: 
`by_cases_local_eq`, `by_induction_eq` (`induction_eq`), `by_rw_occ_spine`. What calc's
claims needed: `by_case_step` (a matcher at a constructor whose row is an open `if`; a
comparison left as it is by `reduce`), `define_match_eq` (a measured function's matcher
carries its equation: the descent obligation under `tail2 xs = cons x r`, discharged, and a
ground run by `reduce`), `type_ctor_theorem` (a type and its constructor of one name in a
theorem's binder). `by_sorry`'s record carries the sorried goals. What the slice's own second
reader found: `simp_self` (a rule whose right side holds its left never reaches a fixpoint),
`by_ne_level` (`Ne` at a universe other than 1), and `define_match_eq`'s second function (a
descent obligation as the comparison at `Nat` under the source's names).

Slice 3.21b (GPT-6's trajectory review, findings 1–3): `op_expected` (an arithmetic
operator under an expected `Nat` or `Int` takes that type's identity — `(- 1 2)` at `Int` is
`Int.sub 1 2`, -1, where it was `Int.ofNat (Nat.sub 1 2)`, 0 — and a comparison's operands
decide by the first that is not numeric-closed), `init_horizon` (a module sees Init below
its own horizon whatever an earlier root admitted: the review's failing load order),
`init_horizon_beyond` (a declaration past the horizon, admitted by another module, is
`unknown_constant`); `by_reduce` expects `ok` (reduce leaves a target with nothing to
reduce as it is).

Slice 3.22, landing 1 (the producers' ground): `arith_only` (`(arith only FACT… (farkas K…))`:
the rows are the goal and the facts named — the review's probe, an unrelated `have` before an
explicit certificate, and a context hypothesis that is no row until named), `by_witness`
(`apply Eq.trans` with its middle term assigned by the first premise's `exact`) and
`by_witness_open` (both premises sorried: `witness_open` names the argument), `auto_core` (a
theorem whose proof is `auto` or `(auto HINT…)` replays the block of `main.auto.shard`'s
entry for it), `auto_stale` (an entry made for another statement: `pending auto_stale
fp=…`, the fingerprint an entry needs), `auto_missing` (no sidecar: `pending auto_missing
fp=…`), `auto_malformed` (a form of another shape in the sidecar: the file's
`sidecar_malformed`, the theorem pending). The build never searches.

`v3/kernel/test/loader_pins_test.shard` replays every case in its list;
`loader_test.shard` holds the cases a header cannot state (two root
files, the wrong-pin fixture, the records' text, a loaded program run
under `ev`). `v3/test.sh` runs both. The `ev_*` cases are the
classifier's (slice 5); `ev_profile` has its files under `kernel/` — a
file without `Init`, E only, formerly the toolchain profile (§8, slice
3.4); the `realize_*` cases are slice 5b's (§7.5). Slice
7's: `entry` (a kernel-like file: a checked entry with `Int`, a byte list and
the World, run by `entry_test`) and `entry_s` (S: `Nat`, Init's `(List
Nat)`), `same_spelled` (T5: Init's `Bool` beside `main.Bool`, the bare
name ambiguous, `Init.Bool` the explicit citation) and
`realize_theorem` (T5: a theorem about `List.append` stated before its
realization, cited after). The ratification pass's (§13 item 35,
2026-09-15): `type_e_only` (no `Init`: `(type Foo (MkFoo Nat))` is E
only by its field and a `def` citing it is `unknown_constant`) and
`type_flip` (the same type with `(import Init Nat.add)` enters K, the
`def` accepted) — the same text, decided by its file's scope. The K
seal's (slice 3.8): `private_module` (a direct import of a file inside
a directory that has a view, from outside it, refused) and
`private_inside` (the implementation importing its private helper from
inside, the consumer importing the directory); landing 2's:
`sealed_module` (one module across the directory: the view's type and
fn implemented in two private files, the implementation file only
importing them), `impl_e_only` (an E-only implementation type stands
for a sig type as an E parameter), `view_req_scope` now imports the
view's own implementation file (a plain file outside the directory is
allowed). `ev_private_match` and `ev_launder` are retired: the seal
closes the case — outside, the import is refused first; inside, the
type is concrete. Landing 3's `k_client_reach`: a client of K's view
naming `CheckedEnv`'s constructor, refused `unknown_head` — the first
case under the package root, by the `;; root:` header (the case's
files then sit where a client of `kernel/k` must). Slice 3.9's records:
`record_basic` (a plain, a `(ctor …)` and a parametric record; `make`
in any order; `with` chained; the closure also tied by parity) and
`record_make` (a `make` missing a field: `record_make`). Slice 3.13's
(`fn` = `def` + `realize`, §8.4): `define_basic` (a non-recursive fn
and a structurally recursive one defined with `eq_N`, theorems citing
the equations, an accumulator through the abstracted motive, `+` and
`lt` by operand type, a `let` and an `if` in tail position),
`define_depth` (a self-call two constructors down: `RUNNABLE …
why=recursion_depth`), `define_nat_operator` (`-` at `Nat` refused),
`define_runnable` (the obstacles named: a `Symbol` binder, a call to
such a fn, an extern, a measure, a numeral pattern — beside a fn that
is defined), `define_refused` (an eligible fn refused: a self-call
without a measure), `def_match` (a `def` in the E forms, rule 5), and
§8.5's `view_eq_hidden` (a consumer citing `lib.step.eq_1`:
`unknown_constant`) and `view_rfl` (a consumer's `Eq.refl` through
the implementation's body: K's `conversion` refusal) — both with the
directory as a second root, where the implementation's own check
shows `DEFINE lib.step` and `DISCHARGE lib.step defined`.
Slice 3.14's (course-of-values, measures, §8.4): `define_depth` now
defined by `brecOn`, its third equation cited; `define_below` (a native
`Tree` gets `Tree.below` and `Tree.brecOn` generated, two `ACCEPT`
lines, then an immediate and a deep recursion over it, equations
cited), `define_measure` (a measured fn: `PARAM main.count.dec_1
obligation`, `DEFINE … pending=main.count`, `PENDING main.count
measure`, a theorem citing it), `define_measure_int` (`RUNNABLE …
why=measure_type`) and `define_mutual` (`why=mutual_recursion`).

Slice 3.15's (the elaborator, §5.1, §8.4): every pin rewritten to
Stage 1's spelling — an implicit argument never written (`(Eq a b)`,
`(Eq.refl a)`, `(List.cons x t)`; `(len xs)` and `(Pair2.mk a b)` in
a theorem as in a body, a `type`'s parameters and a `fn`'s type
parameters being implicit now), a universe not written inferred,
`@NAME` the explicit application; `refused` and `view_rfl` are
`read-error type_mismatch` (the elaborator refuses a false theorem's
proof, and a consumer's `Eq.refl` through a view parameter, before K).
The new cases: `elab_implicit` (`List.append`, `List.cons`, `Eq` bare;
`Eq.refl`'s and `Or.inl`'s implicits from the expected type; `List
Type` at level 1; `@Eq.{1}`; `exists`), `elab_numeral` (`5` and `-3`
at `Int`, a `Nat` coerced by `Int.ofNat` where an `Int` is expected,
`(+ n 0)` at `Nat`, `(- n n)` at `Nat` as `Nat.sub` in a statement,
`(+ a b)` at `Int`, `if` as `ite` with `Nat.decLt`, `(< a b)` the
proposition), `elab_unsolved` (`(= List.nil List.nil)`: the element
type undetermined, `unsolved_implicit`), `elab_instance` (`ite True`:
`instance_needed`), `elab_no_decision` (`(if True …)`: `no_decision`),
`elab_mismatch` (an `Int` where a `Nat` is expected: `type_mismatch`,
no `Int.toNat`).
`define_forward` (a fn calling one defined later in the file: parked,
defined once the callee is, its equation cited) and `elab_confusion`
(the constructor-structure bundle of two native types cited: `Color.
noConfusion` at `False`, `Tree.Node.inj`, `Tree.casesOn` with the
motive from the expected type) close slice 3.15.
Slice 3.16 landing 1 (matchers; §8.4 rule 1, 5): `match_def` (a `match`
in a `def` as a generated matcher admitted before its owner; nested
patterns with a fall-through row; the theorems `Eq.refl` through it),
`match_statement` (the same form in a statement, the theorem owning
the matcher), `match_poly` (the matcher abstracts the type parameter
its scrutinee's type mentions), `match_imposter` (a hand-written
matcher and an imposter of the same type, for
`kernel/test/matcher_test.shard`), `match_not_exhaustive`,
`match_literal` (`literal_pattern`), `match_motive` (the type not
determined at the position), `ctor_expected` (`Add` at an expected
`Exp` beside Init's class). Landing 2: `project_bool` (definitions
erased by the derived view and run by `kernel/test/project_test.shard`:
both values of a `decide` returned, matched, stored, used as a
condition; a matcher's chosen arm only) and `elab_bridge` (`decide`,
`if` on a `Bool` and on a two-constructor type, an operator past a
numeral operand, `(list …)`, an untyped `let`; the theorems `Eq.refl`). Landing 3 (the flip): `route_callee` (G8: a `RUNNABLE` callee as a
callee-local — `callee_runnable route=typed`; `define_test.sh` compares
the two programs' dumps), `one_meaning` (G1: one expression as a `fn`'s
body and a `def`'s value, equal by `Eq.refl`; `-` at `Int` with a
`Nat` operand cast at the leaf since slice 3.21b), `discharge_measure` (G4: three obligations with their branch
proof discharged by `Nat.sub_lt`, the account read in
`define_test.sh`), `discharge_self_cycle` and `discharge_two_cycle` (G7:
`fulfills_cycle`), `match_in_impl` (G9: a matcher generated and read
back inside an implementation check), `define_arg_match` (a self-call
under a `match` in an argument and a `let`; no equation where K cannot
decide it). Moved by the flip, each with its cause in its header:
`ev_unsaturated` and `ev_fn_value` (`type_mismatch`), `ev_nonexhaustive`
(`match_not_exhaustive`), `ev_ambiguous` (`ambiguous_name`),
`ev_pattern_type` (`pattern_constructor`), `k_client_reach`
(`function_expected`), `define_nat_operator` (`ok`: rule 4),
`define_mutual` (`callee_runnable`), `realize_order`
(`no_realization`), `ev_no_l_meaning` and `realize_fn` (a new obstacle:
the old one, a measure without a self-call, is now a plain
definition).

Slice 3.17's (§8.4, the porting facilities): `literal_pattern` (numeral
rows at `Int` and `Nat` on the typed route, in a `fn` and a `def`, the
rows computing in K), `literal_open` (`match_not_exhaustive`: no last
row), `literal_type` (`pattern_type`: a numeral row at a `Bool`),
`literal_rows` (a row after the catch-all), `literal_nested` (a numeral
under a constructor: E-first, `no_l_identity`); `match_literal` is
narrowed to that nested case in a `def`. `rename_ok` (`< <= =` on both
routes), `rename_refused` and `rename_refused_e` (`renamed_primitive`
from the elaborator and from the classifier: `lt le int_eq` outside the
toolchain's own sources). `record_laws` (v2's law family as `Eq.refl`),
`struct_make` (`make` and `with` over a structure, the dependent field
supplied), `dependent_update` (it is not: a read error naming the field),
`struct_params` (a structure with parameters: `record_make`).
`measure_natabs` (a measure through `Int.natAbs`, erased by the
registry's expression row; `pending measure`). `route_sig_callee` (a
view's `sig fn` with no L reading as a callee: `no_l_meaning`). A header
may now also state `renamed_primitive`, `literal_rows`, `pattern_type`
(the elaborator's) and `read-error dependent_update`. Slice 3.18's:
`row_types` (the registry's type rows: a constructor its argument, a
projection its subject, a constructor pattern binding the field),
`row_e_first` and `row_e_match` (an E-first body cannot cite a row's
constructor, `unknown_head`, nor take its value apart with the
representation's, `pattern_type`), `struct_proj` (a projection function is its
projection), `ev_string` rewritten (a literal is a `String`, its
program its bytes), `str_e_first` (the same literal in an E-first
body), `str_no_string` and `str_bad_utf8` (`no_string`; `read-error
bad_string`), `wire_s`, `wire_bad` and `wire_bad_result` (the externs under
the naming law; one declared over a type the wire has no codec for;
two with a result the extern does not return; run by
`kernel/test/wire_test.sh`). `kernel/test/rows_test.shard` runs
`ev_string`, `row_types` and `struct_proj` under `ev` and compares
the values. The cases stay on the first chunk of the prefix; the
libraries' tests add `fixtures/init_prefix_str_tail.ndjson` (the
export's lines 100,852 to 274,616: through `UInt8.toNat`, line 263,515, for the libraries,
and on through `Int.lt_mul_ediv_self_add` for `arith`'s product and quotient).
