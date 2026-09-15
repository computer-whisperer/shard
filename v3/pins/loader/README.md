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

A `REALIZE` record (a realization attached, §7.5) counts as accepted, as `RUNNABLE` does.

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
`def` accepted) — the same text, decided by its file's scope.
