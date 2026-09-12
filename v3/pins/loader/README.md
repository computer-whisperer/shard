# v3/pins/loader — the loader's corpus (phase 2, slice 3)

Each directory is a **package root**; `CASE/main.shard` is the file the
loader is given (`v3/kernel/load.shard --root v3/pins/loader/CASE …`),
its imports resolve under that root, and `(import Init NAME)` streams
the 3000-line export fixture (`v3/kernel/test/fixtures/`). The header
line of `main.shard` states the load's expected outcome — the first
record that is not a module, an Init load or an acceptance:

```
;; expect: ok                          every record accepted (at least one; parameters and the
;;                                     implementation's own declarations count)
;; expect: NAME refused REASON         K refused NAME for REASON
;; expect: NAME policy AXIOM…          NAME's axiom closure reaches AXIOM… outside the file's policy (§9)
;; expect: NAME pending sorry          read, admits nothing (a theorem's proof, or a fulfills)
;; expect: NAME later KIND             an E form (fn …), reported and skipped until slice 5
;; expect: NAME impl-error REASON      the implementation check (§6.6): impl_missing_type, impl_missing_fn,
;;                                     impl_missing_fulfills, impl_type_arity, impl_signature, fulfills_unknown
;; expect: read-error REASON           the reader refused (the file's reading ends there)
;; expect: load-error REASON           the loader refused: import_cycle, missing_file, missing_interface,
;;                                     import_outside_root, init_name_not_found, export_pin_mismatch,
;;                                     req_scope, view_form, sig_outside_view, …
;; roots: FILE…                        what the loader is given, in order (default main.shard);
;;                                     a directory (lib) is its view and then its implementation checked
```

`v3/kernel/test/loader_pins_test.shard` replays every case in its list;
`loader_test.shard` holds the cases a header cannot state (two root
files, the wrong-pin fixture, the records' text). `v3/test.sh` runs both.
