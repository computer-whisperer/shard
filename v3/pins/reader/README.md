# v3/pins/reader — the Stage-0 reader's corpus (phase 2, slice 2)

S files read by `v3/kernel/reader.shard` into K from an empty
environment, no import, no Rust loader: the direct S → L → K path. Each
file is self-contained and carries its expectation in a header line:

```
;; expect: ok                          every form admitted
;; expect: NAME refused REASON         K refused NAME for REASON (its declared reason symbol)
;; expect: NAME pending sorry          read, admits nothing
;; expect: read-error REASON           the reader (or the s-expression reader) refused, REASON
```

`v3/kernel/test/reader_pins_test.shard` replays every file named in its
list and compares the outcome with the header; `v3/test.sh` runs it.
The old tree's `pins/trust` seeded this directory's shape: the
declarative reason is the pin, never the message text.

Since slice 3.15 (LANGUAGE.md §5.1, §8.4) the files are written in
Stage 1's spelling: an implicit argument is never written (`(Eq a a)`,
`(Eq.refl two)`, `(Pair2.mk two Color.green)`), a universe not written
is inferred, `@NAME` is the explicit application. Two expectations
moved with it — `refuse_false_theorem` and `refuse_universe_collapse`
are `read-error type_mismatch`, the elaborator refusing before K (K's
own refusals of the raw declarations stay pinned in the hostile
battery, cases 1 and the false theorem) — and `refuse_universe_args`
became `refuse_unsolved_universe`: a bare polymorphic constant is no
longer an error, a universe the inputs do not determine is.
