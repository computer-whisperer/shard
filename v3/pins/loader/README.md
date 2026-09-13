# v3/pins/loader — the loader's corpus (phase 2, slices 3–5)

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
;;                                     nonexhaustive, private_match, unknown_head, ambiguous_head, unbound_name,
;;                                     string_literal, list_sugar, symbol_literal, unknown_type,
;;                                     pattern_type, pattern_name, …
;; expect: NAME later KIND             a realize form, reported and skipped until slice 5b
;; expect: NAME impl-error REASON      the implementation check (§6.6): impl_missing_type, impl_missing_fn,
;;                                     impl_missing_fulfills, impl_type_arity, impl_signature, fulfills_unknown
;; expect: read-error REASON           the reader refused (the file's reading ends there)
;; expect: load-error REASON           the loader refused: import_cycle, missing_file, missing_interface,
;;                                     import_outside_root, init_name_not_found, export_pin_mismatch,
;;                                     req_scope, view_form, sig_outside_view, duplicate_name, profile_form,
;;                                     module_collision, …
;; roots: FILE…                        what the loader is given, in order (default main.shard);
;;                                     a directory (lib) is its view and then its implementation checked
```

`v3/kernel/test/loader_pins_test.shard` replays every case in its list;
`loader_test.shard` holds the cases a header cannot state (two root
files, the wrong-pin fixture, the records' text, a loaded program run
under `ev`). `v3/test.sh` runs both. The `ev_*` cases are the
classifier's (slice 5); `ev_profile` has its files under `kernel/`, the
toolchain profile.
