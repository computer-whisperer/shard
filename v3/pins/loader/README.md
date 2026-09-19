# v3/pins/loader — the loader's corpus (phase 2, slices 3–7)

Each directory is a **package root**; `CASE/main.shard` is the file the
loader is given (`v3/kernel/load.shard --root v3/pins/loader/CASE …`),
its imports resolve under that root, and `(import Init NAME)` streams
the 3000-line export fixture (`v3/kernel/test/fixtures/`). The header
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
body and a `def`'s value, equal by `Eq.refl`; `-` at `Nat` and at
`Int`), `discharge_measure` (G4: three obligations with their branch
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
