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
