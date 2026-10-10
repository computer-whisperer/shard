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
| `facts.shard` | the fifteen former axioms of the old `kernel/facts.shard` as theorems: twelve by Init's names (landing 2); the three bitwise recurrences `band_rec`, `bor_rec`, `bxor_rec` with `bits.shard` (landing 3) | `kernel/test/std_test.sh` |
| `migration.shard` | one theorem per row of law §10.3 — the row's V3 spelling and the exceptional behavior its third column names, on this pin — and the records' evidence theorems for `order`, `nat`, `div` and `arith` | `std_test.sh` |
| `bytes.shard`, `derive.shard`, `fresh.shard`, `host.shard` | slices 3.17–3.19: bytes and text, the derived views, the fresh-name supply, the host's externs | `define_test.sh`, `derive_test.sh`, `calc_main_test.sh`, `std_test.sh` |
| `list.shard`, `bits.shard` | landing 3, not yet in the tree: the realizations of Init's list functions, `List.get` under the dependent match, `List.Pairwise`'s ghost refinement; the three recurrences and the width material | `std_test.sh` |

`kernel/test/std_test.sh` (the 39th entrypoint) loads every file of this
directory in one process — Init on demand, shared across the files — stores
every declaration and replays the store with K alone (slice 3.22's gate),
holding the records' theorems by name. Slice 3.13's first library file,
`list.shard` with V3's own `List.sum`, is retired: `List.sum` is Init's under
the instance table (slice 3.27 rule 3; the `instance_table` pin).

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

## `std/list` — five opaque functions and 24 requirements (`list.shard` at landing 3)

The old module's operations were opaque `sig fn`s over `(List T)` with
`Int`-valued lengths and counts; its requirements were their defining
equations and algebra. The functions are Init's, realized by supplied
bodies at landing 3 (rule 6); the requirements are Init's theorems, cited by
name (each checked against the export's index), the counts and lengths at
`Nat`. Class: typed throughout (`Int` → `Nat`, Bool → Prop).

| old | V3 | evidence |
|---|---|---|
| `append xs ys` | `List.append` | realized, landing 3 |
| `len xs` (`Int`) | `List.length` (`Nat`) | realized, landing 3 |
| `rev xs` | `List.reverse` (`List.reverseAux`) | realized, landing 3 |
| `take n xs` (`n : Int`) | `List.take` (`Nat`) | realized by `cases` on the count, landing 3 |
| `drop n xs` (`n : Int`) | `List.drop` (`Nat`) | realized by `cases` on the count, landing 3 |
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
| `len_nonneg` `0 ≤ len xs` | `Nat.zero_le` — the type | cited |
| `append_nil_right` | `List.append_nil` | cited |
| `append_assoc` | `List.append_assoc` | cited |
| `len_append` | `List.length_append` | cited |
| `len_rev` | `List.length_reverse` | cited |
| `rev_append` `rev (append xs ys) = append (rev ys) (rev xs)` | `List.reverse_append` | cited |
| `rev_rev` | `List.reverse_reverse` | cited |
| `drop_le` `len (drop n xs) ≤ len xs` | `List.length_drop` with `Nat.sub_le` | cited |
| `drop_shortens` `1 ≤ n → 1 ≤ len xs → len (drop n xs) + 1 ≤ len xs` | `List.length_drop` with `Nat.sub_lt` | cited |
| `take_le` `len (take n xs) ≤ len xs` | `List.length_take_le'` | cited |
| `take_len` `0 ≤ n → n ≤ len xs → len (take n xs) = n` | `List.length_take_of_le` | cited |
| `drop_len` `0 ≤ n → n ≤ len xs → len (drop n xs) = len xs - n` | `List.length_drop` (unconditional at `Nat`) | cited |

## `std/bits` — one function and 37 requirements (`bits.shard` at landing 3)

The old module (2,135 lines of proofs from the former recurrences) is Init's
bitwise library at `Nat`: the former `0 ≤` premises are the type, `pow2 k`
is `2 ^ k` (`Nat.pow 2 k`), and the width instances at 8, 32 and 64 are
`bits.shard`'s at landing 3, each one application of the general theorem to
a literal. Class: typed throughout.

| old | V3 | evidence |
|---|---|---|
| `pow2 k` (`fn`, `1` below zero) | `(Nat.pow 2 k)` | — |
| `pow2_z` `pow2 0 = 1` | `Nat.pow_zero 2` | cited |
| `pow2_s` `pow2 (k + 1) = 2 · pow2 k` | `Nat.pow_succ` (`2 ^ k * 2`) | cited |
| `pow2_pos` `0 < pow2 k` | `Nat.two_pow_pos k` | cited |
| `pow2_32`, `pow2_64` | `Eq.refl` (the literal computes) | — |
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
| `bor_le32` (the instance at 32) | `bits.shard`, landing 3 | — |
| `bxor_le_pow2` | `Nat.xor_lt_two_pow`, the same bridge | cited |
| `bxor_le32`, `bxor_lt32`, `bxor_lt64` | `bits.shard`, landing 3 | — |
| `shl1_mul` `bshl a 1 = 2 · a` | `Eq.refl` (`Nat.shiftLeft` computes one step) | — |
| `shl8_mul`, `shl5_mul`, `shl13_mul` `bshl a k = 2^k · a` | `Nat.shiftLeft_eq a k` with `Nat.mul_comm`, the power a literal | cited |
| `shr1_div` `bshr a 1 = ediv a 2` | `Nat.shiftRight_succ_inside a 0` with `Nat.shiftRight_zero`; `facts.shard`'s `bshr_s` | cited |
| `shr8_div`, `shr17_div` `bshr a k = ediv a 2^k` | `Nat.shiftRight_eq_div_pow a k` | cited |
| `shl_pow2` `bshl a k = a · pow2 k` | `Nat.shiftLeft_eq` | cited |
| `shr_pow2` `bshr a k = ediv a (pow2 k)` | `Nat.shiftRight_eq_div_pow` | cited |
| `shl_nonneg`, `shr_nonneg` | `Nat.zero_le` — the type | cited |
| `mask_pow2` `band a (pow2 k - 1) = mod a (pow2 k)` | `Nat.and_two_pow_sub_one_eq_mod a k` | cited |
| `mask_byte`, `mask_word32`, `mask_word64` | `bits.shard`, landing 3 (`Nat.and_two_pow_sub_one_eq_mod` at 8, 32, 64) | — |

The three recurrences the old proofs started from — `band_rec`, `bor_rec`,
`bxor_rec` of `kernel/facts.shard` — are `facts.shard`'s at landing 3 (rule
9's last row).
