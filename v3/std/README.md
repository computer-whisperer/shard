# `v3/std` — the library

Init — Lean's base library, read from the pinned export — is the library. A statement Init has is
cited by Init's name. This directory holds two things: what Init lacks, and the programs that Init's
functions run as (a `realize` attaches a program to one of Init's functions and proves the two
equal). Everything here is in Init's vocabulary: a size, an address or a length is a `Nat`; a
comparison is the proposition `(<= a b)`; `/` and `%` at `Int` are the Euclidean pair.

## The files

| file | module | what it holds | tests |
|---|---|---|---|
| `list.shard` | `std.list` | programs for Init's list functions, their equations, list lemmas, a sorted list | `std_test.sh`, `template_test.sh` |
| `bytes.shard` | `std.bytes` | bytes, byte arrays and text | `define_test.sh` |
| `derive.shard` | `std.derive` | equality, ordering and rendering for the basic types | `derive_test.sh` |
| `fresh.shard` | `std.fresh` | a supply of fresh names | `define_test.sh` |
| `host.shard` | `std.host` | the `World` and the host calls | `define_test.sh`, `bin_test.sh` |
| `map.shard` | `std.map` | a finite map over `Nat` keys | `imp_test.sh` |
| `mem.shard` | `std.mem` | a byte memory | `imp_test.sh` |
| `facts.shard` | `std.facts` | division, shift and bitwise facts | `std_test.sh` |
| `bits.shard` | `std.bits` | powers of two, masks, shifts and bounds at `Nat` | `std_test.sh` |
| `migration.shard` | `std.migration` | what each operator and constructor spelling means; small arithmetic lemmas | `std_test.sh` |

`kernel/test/std_test.sh` loads every file of the directory in one process, holds the theorems of
`facts`, `bits`, `migration` and `list` by name — each accepted on no axiom but Lean's `propext`,
`Quot.sound` and `Classical.choice` — stores every declaration and has the checker K alone accept
the store.

## `list.shard`

**Programs for Init's functions.** `List.length`, `List.append`, `List.reverseAux`, `List.reverse`,
`List.take`, `List.drop`, `List.get?Internal`, `List.getD` and `List.get` each have a body here and
the equations `NAME.realize_1`, `NAME.realize_2` as theorems. `take` and `drop` recurse on the list
where Init recurses on the count; their equations are `take_nil_eq`, `take_cons_eq`, `drop_nil_eq`,
`drop_cons_eq`, proven by cases on the count. `get?Internal` tests the index with an `if`
(`get_opt_nil`, `get_opt_cons`). `Option.getD` runs as Init's own definition (`(realize Option.getD
(view))`). `List.get` takes its index as a `Fin`; its nil row is refuted with `absurd` and its cons
row carries the bound into the tail by `get_bound`.

**Functions that take a function.** `List.map`, `List.filter`, `List.foldl`, `List.foldr`,
`List.any` and `List.all` are templates: each is defined once over an arbitrary function, and a
caller that passes a named function, a partial application, a lambda or a structure's field gets a
compiled instance (`v3/examples/template/`). `filter`'s cons equation is by Init's
`List.filter_cons`.

**Lemmas.** Twenty-four `list_*` theorems give Init's list facts under short names; each is one
citation:

| theorem | Init's name |
|---|---|
| `list_append_nil_left`, `list_append_cons`, `list_append_nil_right`, `list_append_assoc` | `List.nil_append`, `List.cons_append`, `List.append_nil`, `List.append_assoc` |
| `list_rev_nil`, `list_rev_cons`, `list_rev_append`, `list_rev_rev` | `Eq.refl`, `List.reverse_cons`, `List.reverse_append`, `List.reverse_reverse` |
| `list_len_nil`, `list_len_cons`, `list_len_nonneg`, `list_len_append`, `list_len_rev` | `Eq.refl`, `Eq.refl`, `Nat.zero_le`, `List.length_append`, `List.length_reverse` |
| `list_take_nil`, `list_take_zero`, `list_take_succ_cons`, `list_take_le`, `list_take_len` | `List.take_nil`, `List.take_zero`, `List.take_succ_cons`, `List.length_take_le'`, `List.length_take_of_le` |
| `list_drop_nil`, `list_drop_zero`, `list_drop_succ_cons`, `list_drop_len`, `list_drop_le`, `list_drop_shortens` | `List.drop_nil`, `List.drop_zero`, `List.drop_succ_cons`, `List.length_drop`, then `Nat.sub_le` and `Nat.sub_lt` |

**What Init lacks.** `sum_append` at `Nat`, by induction: Init's `sum_append` asks for an instance
the loader's instance table does not hold (`Std.LawfulLeftIdentity`). A sorted list: `insert`,
`mem_insert`, `insert_sorted`, and `Sorted`, the type `Subtype (List.Pairwise Nat.le)`, with
`sorted_insert` keeping the proof. `Sorted` is named by a `def` because a `fn` signature reads
`@Subtype` at the wrong universe. `sorted_demo` and `get_demo` are small closed uses.

## `bytes.shard`

`UInt8`, `ByteArray` and `String` are Init's types. A byte runs as an integer below 256, a byte
array as a list, a text as its UTF-8 bytes.

| name | what it is |
|---|---|
| `Byte.ofNat n` | `n mod 256` as a byte; `Byte.ofNat_eq` says it is Init's `UInt8.ofNat` |
| `Bytes.ofList`, `Bytes.toList` | a byte array from and to a list of bytes; the two round trips are theorems |
| `Bytes.ofNats`, `Bytes.toNats` | a byte array from and to a list of numbers |
| `Bytes.ofString` | a text's UTF-8 bytes |

Not here: text from bytes. `String.fromUTF8?` needs a validator and a proof against
`ByteArray.IsValidUTF8`. A program's text is its literals; its input stays bytes.

## `derive.shard`

A program derives equality, an ordering and a rendering for its own types with
`(derive TYPE CAPABILITY…)`; a field's procedure is the one registered for the field's type. `Nat`,
`Int` and `Bool` have built-in equality, `Nat` and `Int` a built-in ordering. This file registers
the rest: `Bool`'s ordering and rendering; `UInt8`, `ByteArray` and `String`'s equality, ordering
and rendering (`Byte.decEq`, `Byte.compare`, `Bytes.decEq`, `Bytes.compare`, `Text.decEq`,
`Text.compare`, …); the renderers `Nat.renderOnto` and `Int.renderOnto`. A file that renders imports
this one. A rendering is the value as source would write it, built onto a rest
(`T.renderOnto x rest`): a number in decimal, a text quoted with the reader's escapes. Numbers order
by `<`; bytes and texts byte by byte, a prefix first.

## `fresh.shard`

There is no effectful primitive. A function that needs fresh names takes a supply `(Fresh P)` and
returns the advanced one with its result: `Fresh.start prefix`, `Fresh.next s`. A name is the
supply's prefix and an index. `Fresh.next_ne` proves that two successive names of one supply differ.

## `host.shard`

`(type World (World Nat))` and the six host calls, each over `ByteArray` and Init's `List`,
`Option`, `Prod`, `Bool`:

| call | signature |
|---|---|
| `get_args` | `((w World)) (Prod (List ByteArray) World)` |
| `read_file` | `((path ByteArray) (w World)) (Prod (Option ByteArray) World)` |
| `write`, `write_line` | `((bytes ByteArray) (w World)) World` |
| `write_file` | `((path ByteArray) (bytes ByteArray) (w World)) (Prod Bool World)` |
| `exit` | `((code Int) (w World)) World` |

A host call has no body in the logic: it is an opaque constant, and what a program's theorems may
assume of it is stated as axioms in the program's contract (`v3/examples/calc/calc_bin.shard`).

## `map.shard` and `mem.shard`

`(Map V)` is an association list over `Nat` keys: `map_empty`, `map_lookup k m`, `map_insert k v m`.
An insert prepends and a lookup takes the first match, so facts about a map are stated through
`map_lookup` at a key: `map_lookup_empty`, `map_lookup_insert_eq`, `map_lookup_insert_neq`,
`map_insert_shadow`.

`Mem` is one unbounded byte memory over the map: `mem_empty`, `mem_get m a` (the last store at `a`
mod 256; 0 where nothing was stored), `mem_set m a v`, and the little-endian words `load_le w m a`
and `store_le w m a v` of `w` bytes. No bounds and no traps. The laws of `load_le` over `store_le`
are not stated here; `v3/models/imp/cell.shard` proves the ones it needs.

## `facts.shard`, `bits.shard`, `migration.shard`

| file | theorems |
|---|---|
| `facts.shard` | at `Int`: `mod_lo`, `mod_hi`, `ediv_mod_id` (under `0 < d`), `div_unique`, `mod_unique`; `mul_comm`, `mul_assoc`, `mul_dist`; at `Nat`: `bshl_z`, `bshl_s`, `bshr_z`, `bshr_s`; the low bit of `land`, `lor`, `xor` (`land_low`, `lor_low`, `lxor_low`) and the halving recurrences `band_rec`, `bor_rec`, `bxor_rec` |
| `bits.shard` | `pow2_z`, `pow2_s`, `pow2_pos`, `pow2_8`, `pow2_32`, `pow2_64`; `band_zero_l`, `band_zero_r`, `bor_zero_l`, `bxor_zero_l`, `bxor_self`, `bxor_cancel_right`; `band_le_l`, `band_le_r`; `bor_le_pow2`, `bxor_le_pow2` and their instances `bor_le32`, `bxor_le32`, `bxor_lt32`, `bxor_lt64`; `shl1_mul`, `shl5_mul`, `shl8_mul`, `shl13_mul`, `shl_pow2`, `shr1_div`, `shr8_div`, `shr17_div`, `shr_pow2`; `mask_pow2`, `mask_byte`, `mask_word32`, `mask_word64`; the lower bounds `band_lo`, `bor_lo`, `bxor_lo`, `shl_nonneg`, `shr_nonneg` |
| `migration.shard` | one theorem per spelling rule (below); `order_*` (21 facts on `Int`'s order), `nat_*` (4 on the cast from `Nat`), `div_*` (7 on division by 10 and by a positive divisor), `arith_*` (7 index identities) |

The recurrences are stated without subtraction — `lor a b + (a%2)·(b%2) = 2·lor (a/2) (b/2) + a%2 +
b%2` — because `arith` reads `Nat.sub` as an opaque term. A power of two is `(Nat.pow 2 k)`. A
literal does not determine an exponent, so a width instance states its bound as a `Nat.pow`
hypothesis first (`bor_le32`).

What `migration.shard` states about the spellings:

| theorem | says |
|---|---|
| `int_ops` | `+ - *` at `Int` are `Int.add`, `Int.sub`, `Int.mul` |
| `ediv_zero`, `emod_zero` | `/` and `%` at `Int` are Euclidean; `x / 0 = 0`, `x % 0 = x` |
| `tdiv_zero`, `tmod_zero` | the truncating pair is `Int.tdiv`, `Int.tmod`, with the same values at zero |
| `le_decides`, `beq_decides` | a comparison is a proposition, decided by the type's decision |
| `and_bridge`, `bool_literals` | `and` at `Bool` operands is `Bool.and`; `Bool.true` and `Bool.false` are values, `True` and `False` propositions |
| `ctor_names`, `list_names` | `Option.some`, `Option.none`, `List.cons`, `List.nil`, `Prod.mk`; `List.length`, `List.get?Internal`, `List.getD` |
| `land_nat`, `nat_sub_saturates` | the bitwise operations are at `Nat`; `0 - n = 0` at `Nat` |
| `refine_subtype`, `size_nat` | a refined value is a `Subtype`; a size is a `Nat` |
| `word_wraps`, `bytes_view`, `str_buffer`, `char_of_nat` | `UInt8.ofNat` wraps mod 256; a `ByteArray` is its list; a `String` is its valid UTF-8 buffer |
| `record_projections` | a record's fields are projections |

Most of the `order_*`, `div_*` and `arith_*` lemmas are one `(by (arith))`; a proof that needs
such a fact usually calls `arith` directly.
