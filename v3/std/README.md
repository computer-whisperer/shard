# `v3/std` — the library under the naming law, and the records of the old modules

Init is the library (`v3/LANGUAGE.md` §8.4 slice 3.27, rule 1): a statement
Init has is cited by Init's name; this directory declares what Init lacks, in
Init's namespaces under the naming law (law §5.3). An old module of the
lineage's tree (`std/…` at the repository root) every declaration of which is
Init's or `arith`'s becomes a record below and no file. A record is law
§10.1's per-interface record: every declaration of the old module's public
interface (its `mod.req.shard`), its V3 spelling or Init's name, the
connecting evidence and its class under law §10.2 — *name* (a rename that
preserves resolution), *typed* (a representation change with a
correspondence: `Int` lengths and bit patterns to `Nat`, a Bool-valued
comparison `(= (le a b) True)` to the proposition `(<= a b)`, the former
`0 ≤` premises to the type), *behavior* (a defined value where the old
operation was stuck). The evidence is one of: a theorem of `facts.shard`
(the former axioms) or `migration.shard` (law §10.3's rows, and the
records' entailments), named; Init's theorem, cited by name and checked
against the pinned export's index; `arith`, slice 3.21's Farkas certificate
over the goal and the premises, which `migration.shard` then states as a
theorem so the claim is checked rather than asserted.

## The files

| file | what it holds | gate |
|---|---|---|
| `facts.shard` | the fifteen former axioms of the old `kernel/facts.shard` as theorems: twelve by Init's names (landing 2); the three bitwise recurrences `band_rec`, `bor_rec`, `bxor_rec` (landing 3) — each the Euclidean split of the operation by 2 (Init's `and_div_two`, `or_div_two`, `xor_div_two`) joined by `arith` to its low bit, which `land_low`, `lor_low`, `lxor_low` decide from the operands' low bits through Init's `*_mod_two_eq_one` iffs (four cases each) | `std_test.sh` | `kernel/test/std_test.sh` |
| `migration.shard` | one theorem per row of law §10.3 — the row's V3 spelling and the exceptional behavior its third column names, on this pin — and the records' evidence theorems for `order`, `nat`, `div` and `arith` | `std_test.sh` |
| `bytes.shard`, `derive.shard`, `fresh.shard`, `host.shard` | slices 3.17–3.19: bytes and text, the derived views, the fresh-name supply, the host's externs | `define_test.sh`, `derive_test.sh`, `calc_main_test.sh`, `std_test.sh` |
| `list.shard` | landing 3: Init's list functions realized by supplied bodies with their equations (`List.length`, `append`, `reverseAux`, `reverse`, `take`, `drop`, `get?Internal`, `getD`); slice 3.28 landing 2: `List.map`, `filter`, `foldl`, `foldr`, `any`, `all` as templates — a function-typed binder a static parameter, the equations proven once over it, the programs the instances later bodies take (`filter`'s rows by `List.filter_cons`), `Option.getD` through its matcher's own definition (the derived view), `List.get` under the dependent match with its nil row refuted; the old `std/list`'s requirements as theorems citing Init (`list_*`); what Init lacks — `sum_append` at `Nat`, and the ghost refinement: `insert`, `mem_insert`, `insert_sorted`, `Sorted` = `Subtype (List.Pairwise Nat.le)`, `sorted_insert` | `std_test.sh`; calc's files run `List.append` and `List.length` through it |
| `bits.shard` | landing 3: the old `std/bits`'s requirements as theorems citing Init at `Nat`, and the width material Init lacks — the powers `pow2_8`, `pow2_32`, `pow2_64` by `Eq.refl`, the instances at 32 and 64 bits (`bor_le32`, `bxor_le32`, `bxor_lt32`, `bxor_lt64`), the masks at 8, 32 and 64, the literal shifts | `std_test.sh` |
| `map.shard` | slice 4.2 landing 0: the old `std/map`'s finite map as an association list over `Nat` keys — `map_empty`, `map_lookup`, `map_insert` — and the interface's four requirements as theorems; the substrate `mem.shard` stands on | `imp_test.sh` (through the model root); `std_test.sh` |
| `mem.shard` | slice 4.2 landing 0: the old `std/mem`'s byte memory — `Mem` a record over the map; `mem_empty`, `mem_get`, `mem_set`; `load_le`, `store_le` on a `Nat` measure with their descents discharged — the imp machine's memory; the framing laws arrive with their consumers | `imp_test.sh`; `std_test.sh` |

`kernel/test/std_test.sh` (the 39th entrypoint) loads every file of this
directory in one process — Init on demand, shared across the files — stores
every declaration and replays the store with K alone (slice 3.22's gate),
holding the records' theorems by name. Slice 3.13's first library file,
`list.shard` with V3's own `List.sum`, was retired at landing 2: `List.sum` is
Init's under the instance table (slice 3.27 rule 3; the `instance_table` pin);
landing 3's `list.shard` is the library's. Calc's `examples/calc/list.shard`
(its own `append` and `len` "until `v3/std` exists") retired onto it at landing
3: calc's claims re-spelled `List.append`/`List.length`, its proofs reducing by
the realizations' equations as they did by its own functions'.

## `std/order` — 22 claims about `Int`'s order (no file)

The old module was pure lemmas over `le`, `lt` and `int_eq` at `Int`, each
decided by the old backends; one claim swapped the branches of an `if`. Every
one is `arith`'s where it is linear, Init's theorem where the name exists;
`migration.shard` states each (`order_*`).

| old | V3 | evidence | class |
|---|---|---|---|
| `le_refl` `(le a a) = True` | `(<= a a)` | `Int.le_refl a`; `order_le_refl` | typed |
| `lt_refl_false` `(lt a a) = False` | `(not (< a a))` | `Int.lt_irrefl a`; `order_lt_irrefl` | typed |
| `int_eq_refl` `(int_eq a a) = True` | `(= a a)` | `Eq.refl a`; `order_eq_refl` | typed |
| `lt_succ_from_lt` `p < i → p < i + 1` | the same, propositions | `arith`; `order_lt_succ_from_lt` | typed |
| `le_implies_not_lt_swap` `a ≤ b → (lt b a) = False` | `(-> (<= a b) (not (< b a)))` | `Int.not_lt`; `order_le_implies_not_lt` | typed |
| `not_le_implies_lt_swap` `(le a b) = False → b < a` | `(-> (not (<= a b)) (< b a))` | `Int.not_le`; `order_not_le_implies_lt` | typed |
| `if_le_lt_swap` `(if (le a b) x y) = (if (lt b a) y x)` | `ite` over `Int.decLe`, `Int.decLt` | `Int.not_le` with `cases` on the decision; no V3 consumer — stated when one appears | typed |
| `le_trans` | the same | `Int.le_trans`; `order_le_trans` | typed |
| `lt_implies_neq` `a < b → (int_eq a b) = False` | `(-> (< a b) (not (= a b)))` | `Int.ne_of_lt`; `order_lt_implies_ne` | typed |
| `lt_trans_to_neq` `p < i → i < j → p ≠ j` | the same | `arith` then `Int.ne_of_lt`; `order_lt_trans_to_ne` | typed |
| `lt_pred_from_lt` `j < p → j - 1 < p` | the same | `arith`; `order_lt_pred_from_lt` | typed |
| `lt_implies_neq_flip` `a < b → (int_eq b a) = False` | `(-> (< a b) (not (= b a)))` | `Int.ne_of_gt`; `order_lt_implies_ne_flip` | typed |
| `lt_trans_to_neq_flip` `i < j → j < p → p ≠ i` | the same | `arith` then `Int.ne_of_gt`; `order_lt_trans_to_ne_flip` | typed |
| `le_succ_from_lt` `i < p → i + 1 ≤ p` | the same | `Int.add_one_le_of_lt` (`a < b` is `a + 1 ≤ b` at `Int` by definition); `order_le_succ_from_lt` | typed |
| `le_pred_from_lt` `p < j → p ≤ j - 1` | the same | `arith` (Init: `Int.le_sub_one_of_lt`); `order_le_pred_from_lt` | typed |
| `lt_self_succ` `x < x + 1` | the same | `arith` (Init: `Int.lt_add_one_of_le`); `order_lt_self_succ` | typed |
| `le0_succ` `0 ≤ x → 0 ≤ 1 + x` | the same | `arith`; `order_le0_succ` | typed |
| `le_succ_r` `i ≤ b → i ≤ b + 1` | the same | `arith`; `order_le_succ_r` | typed |
| `le_succ_l` `i ≤ b → i ≤ 1 + b` | the same | `arith`; `order_le_succ_l` | typed |
| `lt_from_le_succ` `a ≤ b → a < 1 + b` | the same | `arith`; `order_lt_from_le_succ` | typed |
| `le_succ_shift` `a + (1 + j) ≤ b → (a + 1) + j ≤ b` | the same | `arith`; `order_le_succ_shift` | typed |
| `le_succ_shift_r` `a + (1 + j) ≤ b → (a + 1) + j ≤ b + 1` | the same | `arith`; `order_le_succ_shift_r` | typed |

## `std/nat` — three opaque functions and four lemmas (no file)

The old module supplied arithmetic over the kernel's `Nat` behind opaque
signatures. The functions are Init's; the lemmas are definitional or
`arith`'s.

| old | V3 | evidence | class |
|---|---|---|---|
| `add_nat a b` (`sig fn`) | `(+ a b)` at `Nat`, `Nat.add` | — | name |
| `int_of_nat n` (`sig fn`) | `(Int.ofNat n)`, the coercion `Nat.cast` | — | name |
| `half_nat n` (`sig fn`) | `(/ n 2)` at `Nat`, `Nat.div n 2` | — | name |
| `int_of_nat_nonneg` `0 ≤ int_of_nat n` | `(<= 0 (Int.ofNat n))` | `Int.natCast_nonneg n`; `nat_int_of_nat_nonneg` | typed |
| `int_of_nat_succ` `int_of_nat (S k) = 1 + int_of_nat k` | `(= (Int.ofNat (+ k 1)) (+ (Int.ofNat k) 1))` | `Eq.refl` (Init: `Int.natCast_succ`); `nat_int_of_nat_succ` | name |
| `int_of_nat_zero` `int_of_nat Z = 0` | `(= (Int.ofNat 0) 0)` | `Eq.refl`; `nat_int_of_nat_zero` | name |
| `half_bound` `int_of_nat m - 1 ≤ 2 · int_of_nat (half_nat m)` | `(<= m (+ (* 2 (/ m 2)) 1))` at `Nat` — the same bound with the saturating subtraction moved to the other side | `arith` over `Nat.div_add_mod m 2` and `Nat.mod_lt m Nat.zero_lt_two`; `nat_half_bound` | typed |

## `std/div` — five requirements over the literal divisor 10 and two symbolic facts (no file)

The old module's facts at the literal divisor were proven by the old
`div-facts` step; the symbolic ones rode the former axioms. `/` and `%` at
`Int` are Euclidean (law §10.3 row 3; `migration.shard`'s `ediv_zero`,
`emod_zero` state the totalization at zero, which no row here reaches).

| old | V3 | evidence | class |
|---|---|---|---|
| `ediv_mod_10_id` `n = 10 · (ediv n 10) + (mod n 10)` | `(= n (+ (* 10 (/ n 10)) (% n 10)))` | `arith` (slice 3.21 rule 4: the quotient and remainder rows at a literal divisor); `div_ediv_mod_10_id` | behavior (row 3) |
| `mod_10_lo` `0 ≤ mod n 10` | `(<= 0 (% n 10))` | `arith` (Init: `Int.emod_nonneg`); `div_mod_10_lo` | behavior |
| `mod_10_hi` `mod n 10 ≤ 9` | `(<= (% n 10) 9)` | `arith` (Init: `Int.emod_lt_of_pos`); `div_mod_10_hi` | behavior |
| `div_lt` `0 < n → ediv n 10 < n` | `(-> (< 0 n) (< (/ n 10) n))` | `arith`; `div_div_lt` | behavior |
| `div_nonneg` `0 ≤ n → 0 ≤ ediv n 10` | `(-> (<= 0 n) (<= 0 (/ n 10)))` | `arith` (Init: `Int.ediv_nonneg`); `div_div_nonneg` | behavior |
| `ediv_nonneg` `0 ≤ n → 0 < d → 0 ≤ ediv n d` | the same | `Int.ediv_nonneg h1 (Int.le_of_lt h2)`; `div_ediv_nonneg` | behavior |
| `mod_small` `0 ≤ x → x < d → mod x d = x` | the same | `Int.emod_eq_of_lt h1 h2`; `div_mod_small` | behavior |
| `mod_lo`, `mod_hi` (cited from `kernel/facts.shard`) | `facts.shard` | `Int.emod_nonneg`, `Int.emod_lt_of_pos` | behavior |

## `std/arith` — seven index identities (no file)

Linear identities over `Int`, each `arith`'s; `migration.shard` states each
(`arith_*`). Class: name (law §10.3 row 1).

| old | V3 | evidence |
|---|---|---|
| `idx_inner_simp` `((i + 1) + (j - 1)) - p = (i + j) - p` | the same | `arith_idx_inner_simp` |
| `sub_zero` `x - 0 = x` | the same | `arith_sub_zero` (Init: `Int.sub_zero`) |
| `sub_sub_one` `(x - 1) - y = x - (1 + y)` | the same | `arith_sub_sub_one` |
| `reassoc_succ` `(b + (1 + y)) - 1 = ((b + 1) + y) - 1` | the same | `arith_reassoc_succ` |
| `idx_cancel` `(((b + 1) + y) - 1) - y = b` | the same | `arith_idx_cancel` |
| `cap_idx` `(0 + (x - 1)) - 0 = (0 + x) - 1` | the same | `arith_cap_idx` |
| `idx_pred` `s - (base + 1) = (s - base) - 1` | the same | `arith_idx_pred` |

## `std/list` — five opaque functions and 24 requirements (`list.shard`, landing 3)

Slice 3.28 landing 2 (§8.4 slice 3.28 rule 6) adds Init's six higher-order list functions as templates: `List.map`, `filter`, `foldl`, `foldr`, `any`, `all`, each realized by a supplied body whose function binder is a static parameter — `TEMPLATE` records, no program of their own, their equations proven once over the arbitrary function (five by `Eq.refl` per leaf; `filter`'s cons row is a match on the test in Init and an `if` here, equal by `List.filter_cons` through the `equations` clause) —; a body that calls one at a name, a named partial application, a lambda or a package's field takes an instance (law §5.3's `add_offset` is written as the law writes it: `v3/examples/template/`).

The old module's operations were opaque `sig fn`s over `(List T)` with
`Int`-valued lengths and counts; its requirements were their defining
equations and algebra. The functions are Init's, realized by supplied
bodies in `list.shard` (rule 6) — `take` and `drop` recurse on the list where
Init recurses on the count first, their equations by cases on the count —;
the requirements are Init's theorems, each cited by name in a `list_*`
theorem of `list.shard` (so the record is checked, not asserted), the counts
and lengths at `Nat`. Class: typed throughout (`Int` → `Nat`, Bool → Prop).
Beyond the old module: `List.get?Internal` and `List.getD` realized,
`Option.getD` through its matcher's own definition, and `List.get` under the
dependent match (rule 7) — its nil row `(absurd (Fin.isLt i) (Nat.not_lt_zero
(Fin.val i)))`, the index re-bound at `Fin (List.length List.nil)`; a
realize's binders are E types, so the `Fin` index is written `Fin`, the
registry's row. What Init lacks: `sum_append` at `Nat` (Init's `sum_append`
asks for `Std.LawfulLeftIdentity`, which the instance table does not hold)
and the ghost refinement phase 3 owes — `Sorted` is `Subtype (List.Pairwise
Nat.le)` (named by a `def`, since a `fn` signature reads `@Subtype` at the
wrong universe today), `sorted_insert` keeps the proof by `insert_sorted`,
proved by induction with the hypothesis as an implication (`cases` and
`induction` generalize the target only).

| old | V3 | evidence |
|---|---|---|
| `append xs ys` | `List.append` | realized (`list.shard`) |
| `len xs` (`Int`) | `List.length` (`Nat`) | realized (`list.shard`) |
| `rev xs` | `List.reverse` (`List.reverseAux`) | realized (`list.shard`) |
| `take n xs` (`n : Int`) | `List.take` (`Nat`) | realized, the equations by `cases` on the count (`take_cons_eq`, `drop_cons_eq`) |
| `drop n xs` (`n : Int`) | `List.drop` (`Nat`) | realized, the equations by `cases` on the count (`take_cons_eq`, `drop_cons_eq`) |
| `append_nil_left` `append Nil ys = ys` | `List.nil_append` | cited |
| `append_cons` `append (Cons h t) ys = Cons h (append t ys)` | `List.cons_append` | cited |
| `rev_nil` | `List.reverse_nil` | cited |
| `rev_cons` `rev (Cons h t) = append (rev t) (Cons h Nil)` | `List.reverse_cons` | cited |
| `len_nil` | `List.length_nil` | cited |
| `len_cons` `len (Cons h t) = 1 + len t` | `List.length_cons` (`as.length + 1`) | cited |
| `take_nil`, `drop_nil` | `List.take_nil`, `List.drop_nil` | cited |
| `take_cons` `take n (Cons h t) = if n ≤ 0 then Nil else Cons h (take (n - 1) t)` | `List.take_zero`, `List.take_succ_cons` — the guard is the count's two cases at `Nat` | cited |
| `drop_cons` `drop n (Cons h t) = if n ≤ 0 then Cons h t else drop (n - 1) t` | `List.drop_zero`, `List.drop_succ_cons` | cited |
| `take_zero` `n ≤ 0 → take n xs = Nil` | `List.take_zero` (`n = 0` at `Nat`) | cited |
| `drop_zero` `n ≤ 0 → drop n xs = xs` | `List.drop_zero` | cited |
| `len_nonneg` `0 ≤ len xs` | `Nat.zero_le` — the type | `list_len_nonneg` |
| `append_nil_right` | `List.append_nil` | cited |
| `append_assoc` | `List.append_assoc` | cited |
| `len_append` | `List.length_append` | cited |
| `len_rev` | `List.length_reverse` | cited |
| `rev_append` `rev (append xs ys) = append (rev ys) (rev xs)` | `List.reverse_append` | cited |
| `rev_rev` | `List.reverse_reverse` | cited |
| `drop_le` `len (drop n xs) ≤ len xs` | `List.length_drop` with `Nat.sub_le` | `list_drop_le` |
| `drop_shortens` `1 ≤ n → 1 ≤ len xs → len (drop n xs) + 1 ≤ len xs` | `List.length_drop` with `Nat.sub_lt` | `list_drop_shortens` |
| `take_le` `len (take n xs) ≤ len xs` | `List.length_take_le'` | cited |
| `take_len` `0 ≤ n → n ≤ len xs → len (take n xs) = n` | `List.length_take_of_le` | `list_take_len` |
| `drop_len` `0 ≤ n → n ≤ len xs → len (drop n xs) = len xs - n` | `List.length_drop` (unconditional at `Nat`) | cited |

## `std/bits` — one function and 37 requirements (`bits.shard`, landing 3)

The old module (2,135 lines of proofs from the former recurrences) is Init's
bitwise library at `Nat`: the former `0 ≤` premises are the type, `pow2 k`
is `(Nat.pow 2 k)` — one spelling with Init's `2 ^ k` since landing 3 —,
every row below a theorem of `bits.shard` under the old name, and the width
instances at 8, 32 and 64 each one application of the general theorem to a
literal (through a `Nat.pow` hypothesis: a literal does not determine the
exponent, so `Nat.or_lt_two_pow ha hb` reads `a < Nat.pow 2 32`, and the
literal form is `exact`'s conversion or `arith`'s). Class: typed throughout.

| old | V3 | evidence |
|---|---|---|
| `pow2 k` (`fn`, `1` below zero) | `(Nat.pow 2 k)` | — |
| `pow2_z` `pow2 0 = 1` | `Nat.pow_zero 2` | cited |
| `pow2_s` `pow2 (k + 1) = 2 · pow2 k` | `Nat.pow_succ` (`2 ^ k * 2`) | cited |
| `pow2_pos` `0 < pow2 k` | `Nat.two_pow_pos k` | cited |
| `pow2_8`, `pow2_32`, `pow2_64` | `Eq.refl` (the literal computes) | — |
| `band_zero_l` `band 0 b = 0` | `Nat.zero_and` | cited (the `one_spelling` pin applies it) |
| `bor_zero_l` `bor 0 b = b` | `Nat.zero_or` | cited |
| `bxor_zero_l` `bxor 0 b = b` | `Nat.zero_xor` | cited |
| `bxor_self` `bxor a a = 0` | `Nat.xor_self` | cited |
| `bxor_cancel_right` `bxor (bxor a b) b = a` | `Nat.xor_assoc`, `Nat.xor_self`, `Nat.xor_zero` | cited |
| `band_zero_r` `band a 0 = 0` | `Nat.and_zero` | cited |
| `band_lo`, `bor_lo`, `bxor_lo` `0 ≤ …` | `Nat.zero_le` — the type | cited |
| `band_le_l` `band a b ≤ a` | `Nat.and_le_left` | cited |
| `band_le_r` `band a b ≤ b` | `Nat.and_le_right` | cited; `migration.shard`'s `land_nat` states it |
| `bor_le_pow2` `a ≤ pow2 k - 1 → b ≤ pow2 k - 1 → bor a b ≤ pow2 k - 1` | `Nat.or_lt_two_pow` through `Nat.le_sub_one_iff_lt (Nat.two_pow_pos k)` | cited |
| `bor_le32` (the instance at 32) | `bits.shard` | — |
| `bxor_le_pow2` | `Nat.xor_lt_two_pow`, the same bridge | cited |
| `bxor_le32`, `bxor_lt32`, `bxor_lt64` | `bits.shard` | — |
| `shl1_mul` `bshl a 1 = 2 · a` | `Eq.refl` (`Nat.shiftLeft` computes one step) | — |
| `shl8_mul`, `shl5_mul`, `shl13_mul` `bshl a k = 2^k · a` | `Nat.shiftLeft_eq a k` with `Nat.mul_comm`, the power a literal | cited |
| `shr1_div` `bshr a 1 = ediv a 2` | `Eq.refl` (`Nat.shiftRight` computes one step) | — |
| `shr8_div`, `shr17_div` `bshr a k = ediv a 2^k` | `Nat.shiftRight_eq_div_pow a k` | cited |
| `shl_pow2` `bshl a k = a · pow2 k` | `Nat.shiftLeft_eq` | cited |
| `shr_pow2` `bshr a k = ediv a (pow2 k)` | `Nat.shiftRight_eq_div_pow` | cited |
| `shl_nonneg`, `shr_nonneg` | `Nat.zero_le` — the type | cited |
| `mask_pow2` `band a (pow2 k - 1) = mod a (pow2 k)` | `Nat.and_two_pow_sub_one_eq_mod a k` | cited |
| `mask_byte`, `mask_word32`, `mask_word64` | `Nat.and_two_pow_sub_one_eq_mod` at 8, 32, 64 | cited |

The three recurrences the old proofs started from — `band_rec`, `bor_rec`,
`bxor_rec` of `kernel/facts.shard` — are `facts.shard`'s (rule 9's last row),
stated at `Nat` and subtraction-free, the product moved to the left: `lor a b
+ (a%2)·(b%2) = 2·lor (a/2) (b/2) + a%2 + b%2` and `xor a b + 2·(a%2)·(b%2) =
2·xor (a/2) (b/2) + a%2 + b%2`, since `arith` reads `Nat.sub` as an atom.

## `std/map` — three opaque functions and five requirements (`map.shard`, slice 4.2 landing 0)

The old module hid an association list behind `(sig type (Map V))` with
`Int` keys and stated its algebra extensionally, under `lookup` at a probe
key. The keys are `Nat` here (an address is a size: typed), the type and its
two constructors are the file's, and the names carry the module's prefix so
that `std/list`'s `insert` and this one never share a short name in one load.

| old | V3 | evidence | class |
|---|---|---|---|
| `empty`, `lookup k m`, `insert k v m` | `map_empty`, `map_lookup`, `map_insert` | realized (`map.shard`) | name; the key `Nat` (typed) |
| `int_eq_refl` `(int_eq k k) = True` | `(= k k)` | `Eq.refl` — the enabling lemma is the proposition itself | typed |
| `lookup_empty` `lookup k empty = None` | `(= (map_lookup k map_empty) Option.none)` | `map_lookup_empty`: `unfold`, `rfl` | name |
| `lookup_insert_eq` `lookup k (insert k v m) = Some v` | the same | `map_lookup_insert_eq`: `unfold`, `rw (if_pos (Eq.refl k))` | name |
| `lookup_insert_neq` `(int_eq j k) = False → lookup j (insert k v m) = lookup j m` | `(-> (Not (= j k)) …)` | `map_lookup_insert_neq`: `unfold`, `rw (if_neg h)` | typed |
| `insert_shadow` `lookup j (insert k v2 (insert k v1 m)) = lookup j (insert k v2 m)` | the same | `map_insert_shadow`: `cases (Nat.decEq j k)`, `if_pos`/`if_neg` | name |

## `std/mem` — one opaque type, six functions and 27 requirements (`mem.shard`, slice 4.2 landing 0: the operations)

The old module (1,534 lines with its lowered twins) is the flat, unbounded,
default-zero byte memory every byte-addressed model consumes: a read is a
byte, a store truncates on read, no bounds and no traps. Addresses, bytes and
words are `Nat` (typed); `Mem` is a record over the map; the width of a word
operation is a `Nat` measure — `(if (= w 0) …)` with the descent by
`Nat.sub_one_lt` — where the old file recursed on `S`/`Z`. The operations
are the rung's; the 27 requirements — `get_empty`, `get_lo`, `get_hi`,
`get_set`, `get_set_byte`, `get_set_get`, `get_set_other`, the `load_le` and
`store_le` steps and framing (`load_le_z`, `load_le_s`, `store_le_z`,
`store_le_s`, `store_le_get_below`, `store_le_get_above`, `ls1_id`,
`ls2_id`, `ls4_id`, `ls8_id`, `load_le_store_le_below`,
`load_le_store_le_above`, `load_le_set_below`, `load_le_set_above`,
`store_le_get_congr`) and the four `mem_read` laws — are theorems to state
with their consumers (slice 4.2 landing 1 by need; the rest phase 5), and
`mem_read` with them.

| old | V3 | evidence | class |
|---|---|---|---|
| `Mem` (`sig type`) | `(record Mem (rep (Map Nat)))` | realized (`mem.shard`) | typed (bytes and addresses `Nat`) |
| `mem_empty`, `mem_get m a`, `mem_set m a v` | the same names | realized | name; `Nat` (typed) |
| `load_le w m a`, `store_le w m a v` (`w : Nat` structural) | the same, `w` a measure | realized; `load_le.dec_1`, `store_le.dec_1` discharged | typed (the width's recursion) |
| `mem_read w m a` | not ported | with its consumers | — |
| the 27 requirements | not yet stated | with their consumers (landing 1; phase 5) | — |
