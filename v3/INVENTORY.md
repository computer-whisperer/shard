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
