# The shared-type inventory (phase 0, FOUNDATION §4.4)

The imported identity and **actual fields** of every shared
mathematical type, read from the pinned sources
(`Init/Prelude.lean` and `Init/Data/*` at Lean v4.33.1, commit
`819816b`). Shapes are the pin's, never restated from memory; T0's
export validates this table declaration-for-declaration. Views and
realizations are recorded only where a first consumer needs them.

| type | pinned declaration (file:line) | constructors / fields at the pin | mathematical view | E realization (proposed) | first consumer |
|---|---|---|---|---|---|
| `Bool` | Prelude.lean:107 | `false`, `true` | — | tag | everything |
| `Nat` | Prelude.lean:1239 | `zero`, `succ n` | — | unbounded integer (GMP literals per §3.2) | everything |
| `Int` | Data/Int/Basic.lean:46 | `ofNat : Nat → Int`, `negSucc : Nat → Int` | — | unbounded integer | everything |
| `Prod α β` | Prelude.lean:563 | `mk (fst : α) (snd : β)` | — | structure | everything |
| `Subtype p` | Prelude.lean:664 | `val : α`, `property : p val` | — | the carrier; `property` erased (§4.1) | ghost refinements |
| `Option α` | Prelude.lean:2924 | `none`, `some (val : α)` | — | inductive | everything |
| `List α` | Prelude.lean:2978 | `nil`, `cons (head : α) (tail : List α)` | — | inductive (today's cells) | everything |
| `Fin n` | Prelude.lean:2324 | `mk (val : Nat) (isLt : val < n)` | — | `Nat`; `isLt` erased; the bound is a static parameter | indices; `UInt*` |
| `BitVec w` | Prelude.lean:2376 | `ofFin (toFin : Fin (2 ^ w))` | — | a word of width `w` where `w ≤ 64` is static; L-only otherwise | `UInt*`; `std/bits` |
| `UInt8` … `UInt64`, `USize` | Prelude.lean:2439 ff. | `ofBitVec (toBitVec : BitVec 8)` etc. | — | machine word, wrapping (§10.4) | `std/word`, `ByteArray` |
| `Char` | Prelude.lean:2856 | `val : UInt32`, `valid : val.isValidChar` | — | `UInt32`; `valid` erased | `String` |
| `Array α` | Prelude.lean:3198 | `mk (toList : List α)` | `List α` (definitional through `toList`) | contiguous buffer under the representation simulation | the counted heap's first library case |
| `ByteArray` | Prelude.lean:3417 | `mk (data : Array UInt8)` | `List UInt8` via `data.toList` | packed byte buffer | `String`; today's `Bytes` |
| `String` | Prelude.lean:3537 | `ofByteArray (toByteArray : ByteArray) (isValidUTF8 : ByteArray.IsValidUTF8 toByteArray)` | `List Char` via `String.toList`/`String.ofList` (a view, not the definition) | today's validated-UTF-8 buffer (`std/str`) — the realization of the imported `String`, `isValidUTF8` erased after checking at the boundary (§9.3) | `std/str`, the toolchain's own diagnostics |
| `Float` | Data/Float/Float.lean:37 | `ofModel (toModel : Float.Model)` | the 4.33 kernel-reducible model | `FLOATS.md`'s proven formats stay ours; Lean's model is the comparison reference (§10.4) | `std/float` (phase 5, its own line) |
| `Decidable p` | Prelude.lean (class inductive) | `isFalse (h : ¬p)`, `isTrue (h : p)` | — | tag kept, payload erased (§4.1) | every `if` |

Notes recorded at drafting:

- v0.7 of the contract described `String` as `List Char` in L; the pin
  says otherwise (above), which is why this table exists (R35).
- `BitVec`'s `Fin (2 ^ w)` makes every `UInt*` a two-level wrapper over
  `Nat` with an erased bound; the realization collapses both levels to
  one machine word — one simulation, stated once for `UInt8` and
  instantiated per width.
- `Array`'s `toList` is the definition, so `List` theorems reach
  `Array` through `Array.toList`/`Array.mk` lemmas, an explicit view
  conversion (§4.4), never by name.
- Nothing here is executable yet; the realizations are proposals for
  phase 2–3 and become law when T1's fixtures pass.
- **As built at slice 3.18 (2026-10-02; `LANGUAGE.md` §8.4):** `Fin`,
  `BitVec`, `UInt8`, `Array`, `ByteArray` and `String` are each
  represented by their one runtime field — rows of the realization
  registry. A `UInt8` is the unbounded integer, below 256 by its
  erased proof (not yet a machine word); an `Array` and a `ByteArray`
  are `Init`'s `List` cells (not yet a buffer); a `String` is its
  UTF-8 bytes, `isValidUTF8` erased and checked nowhere at run time —
  an S program builds a `String` from a literal only, the boundary's
  decoder not being built. The layouts in the table's realization
  column stay the lowering's, behind the same types. `Char` and the
  wider `UInt`s have no row.
- **As built at slice 3.19 (2026-10-02; `LANGUAGE.md` §8.4):**
  `UInt8`, `ByteArray` and `String` have decidable equality and an
  ordering in `v3/std/derive.shard` — hand-written over the one
  runtime field (a byte by its number, a byte array by its list, a
  text by its bytes: on valid UTF-8 the order of code points), each
  tied to the type's identity by a term over Init's own lemmas —
  registered in the derivation table; `UInt8` and `String` have a
  rendering. Init's `String.decEq` (line 1,218,988 of the export) is
  not imported.

- **As built at slice 3.21 (2026-10-06; `LANGUAGE.md` §8.4):** `arith`
  reads `Int` and `Nat` comparisons in Init's spelling and the source's
  as one (`LE.le Int _ a b` is `Int.le a b`), casts a `Nat` row into
  `Int` under `Int.ofNat`, and takes `Int.ediv`/`Int.emod` by a positive
  literal as linear; its proofs are over `Lean.Omega.LinearCombo` and
  `Lean.Omega.Constraint` (the export through line 91,131; line 274,616
  with a product or a quotient). A function under a measure states its
  descent obligations under each enclosing match's equation; `show`
  and `show_nat` take the measure `(Int.natAbs n)`, discharged.
- **As built at slice 3.22 (2026-10-07/08; `LANGUAGE.md` §8.4):**
  a `theorem`'s proof may be `auto` — the build replays the sidecar's
  block (`FILE.auto.shard`) and never searches; `kernel/prove.shard`
  runs the engine (`kernel/engine.shard`, an E library over
  `tactic.shard`'s API) and rewrites the sidecar; `v3/examples/auto/` is
  the example, its sidecar committed and regenerated byte-identical by
  `kernel/test/engine_test.sh`, which also counts the calc claims the
  engine closes unaided (`kernel/test/engine_calc.txt`). The store:
  `load.shard --store DIR` writes every constant a clean load admitted
  as the records K ingests (`kernel/store.shard`), and
  `kernel/verify_release.shard` is K alone replaying it after the Init
  export; `kernel/test/store_test.sh` writes calc's into `v3/release/`
  (the CI artifact, ignored by git) and replays it.
- **As built at slice 3.23 (2026-10-08; `LANGUAGE.md` §8.4):** the Init cache — `t0.shard --receipt` writes the chunks a clean T0 run accepted by path and byte count; `load.shard --init-receipt` (and `prove.shard`, the loader pins test) streams a listed chunk in admit mode (`k/add.shard` `admit_decl_pinning`: axioms, definitions, theorems and opaques inserted without the typing judgments, inductive blocks and the quotient checked), refusing a stale one; `kernel/test/init_receipt.sh` keeps `v3/.cache/init.receipt` current against K's sources, `v3/test.sh` exports it. The fixture's check 47 s → 17 s; `verify_release` takes no receipt.
- **As built at slice 3.20 (2026-10-06; `LANGUAGE.md` §8.4):** a
  `theorem`'s proof may be `(by STEP…)` — I's opener, `kernel/tactic.shard`
  — and a measured `fn` has its equations by `WellFounded.fix_eq`
  (`define.shard`); 44 of calc's 100 claims are theorems by blocks in
  `v3/examples/calc/` (the ground tests, the lexer's structural lemmas,
  the digit type's).
