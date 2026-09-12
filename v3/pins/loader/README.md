# v3/pins/loader — the loader's corpus (phase 2, slice 3)

Each directory is a **package root**; `CASE/main.shard` is the file the
loader is given (`v3/kernel/load.shard --root v3/pins/loader/CASE …`),
its imports resolve under that root, and `(import Init NAME)` streams
the 3000-line export fixture (`v3/kernel/test/fixtures/`). The header
line of `main.shard` states the load's expected outcome — the first
record that is not a module, an Init load or an acceptance:

```
;; expect: ok                          every record accepted (at least one)
;; expect: NAME refused REASON         K refused NAME for REASON
;; expect: NAME policy AXIOM…          NAME's axiom closure reaches AXIOM… outside the file's policy (§9)
;; expect: NAME pending sorry          read, admits nothing
;; expect: NAME later KIND             an E form (fn …), reported and skipped until slice 5
;; expect: read-error REASON           the reader refused (the file's reading ends there)
;; expect: load-error REASON           the loader refused: import_cycle, missing_file, directory_module,
;;                                     import_outside_root, init_name_not_found, export_pin_mismatch, …
```

`v3/kernel/test/loader_pins_test.shard` replays every case in its list;
`loader_test.shard` holds the cases a header cannot state (two root
files, the wrong-pin fixture, the records' text). `v3/test.sh` runs both.
