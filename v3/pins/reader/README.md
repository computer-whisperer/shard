# v3/pins/reader — the reader's corpus

Source files read by `v3/kernel/reader.shard` into the checker K from an empty environment: no
import, no loader. Each file is self-contained — it declares its own `Nat` and `Eq` — and carries
its expectation in a header line, the first line that starts with `;; expect:`:

```
;; expect: ok                          every form admitted
;; expect: NAME refused REASON         K refused NAME for REASON (its declared reason symbol)
;; expect: NAME pending sorry          read, admits nothing
;; expect: read-error REASON           the reader (or the s-expression reader) refused, REASON
```

`v3/kernel/test/reader_pins_test.shard` replays every file named in its list, form by form up to
the first one that is not admitted, and compares the outcome with the header; `v3/test.sh` runs
it. The declared reason is the pin, never the message text.

| file | expects | the case |
|---|---|---|
| `world.shard` | `ok` | the accepted world: `inductive`, `def`, `theorem`, `structure`, `type`, `abbrev`, `opaque` and `axiom`, a numeral, `fun`, `forall`, `let`; the proofs checked by K |
| `accept_by.shard` | `ok` | a proof written as `(by STEP…)`: `rfl` closes the equation by `Eq.refl` |
| `pending_sorry.shard` | `t3 pending sorry` | `sorry` is read and reported, and admits nothing |
| `refuse_false_theorem.shard` | `read-error type_mismatch` | a false theorem: `(Eq.refl two)` is no proof of `(Eq two Nat.zero)` |
| `refuse_universe_collapse.shard` | `read-error type_mismatch` | `(def bad () Prop Nat)`: `Nat` is a `Type`, never a `Prop` |
| `refuse_nonpositive.shard` | `Bad refused non_positive_occurrence` | an inductive type with a non-positive occurrence of itself: K's refusal |
| `refuse_unsolved_universe.shard` | `read-error unsolved_universe` | a polymorphic constant cited bare, whose universe nothing determines: write `NAME.{…}` |
| `refuse_universe_arity.shard` | `read-error universe_arity` | `Eq.{1 2}`: two levels for one universe parameter |
| `refuse_negative_numeral.shard` | `read-error negative_numeral` | `-7` at `Nat`: a negative numeral is a literal at `Int` only |
| `refuse_unknown_constant.shard` | `read-error unknown_constant` | a name nothing declares |
| `refuse_unknown_form.shard` | `read-error unknown_form` | a top-level form with an unknown keyword |
| `refuse_unterminated.shard` | `read-error unterminated_list` | a list that is never closed (one of the two sources the formatter's gate expects not to read) |

The files are written as any source is: an implicit argument is not written (`(Eq a a)`,
`(Eq.refl two)`, `(Pair2.mk two Color.green)`), a universe that is not written is inferred, and
`@NAME` is the explicit application. Two consequences for the expectations:

- `refuse_false_theorem` and `refuse_universe_collapse` are `read-error type_mismatch`: the
  elaborator refuses before K sees the declaration. K's own refusals of raw declarations are
  pinned in the hostile battery (`v3/kernel/test/hostile_test.shard`; the collapse is its case 1).
- A polymorphic constant cited bare is no error; a universe the inputs do not determine is
  (`refuse_unsolved_universe`).
