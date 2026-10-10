#!/usr/bin/env bash
# v3/kernel/test/std_test.sh — the library's gate (v3/LANGUAGE.md §8.4 slice 3.27, landing
# 2; law §7.5): the std root — every file under v3/std — loaded in ONE process, Init on
# demand shared across the files (slices 3.25–3.26), every declaration written to a store,
# and the store replayed by K alone (verify_release.shard, slice 3.22's gate) after the
# closure of Init it cites. The records' theorems are held by name, each under no axiom but
# Lean's own (propext, Quot.sound, Classical.choice — what the cited Init theorems carry):
# std/facts.shard's fifteen former axioms (twelve at landing 2, the three bitwise recurrences
# with their low-bit lemmas at landing 3), std/migration.shard's one per row of law §10.3 and the
# records' evidence, std/bits.shard's record (landing 3), std/list.shard's realizations' equations,
# its record's evidence and the ghost refinement (landing 3). The run of the sorted list's demo
# through the realizations — List.get's refuted row, Subtype's row — is checked by its value.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "std_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index (slice 3.26)
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "std_test: $*"; fail=$((fail+1)); }
held() {   # NAME: accepted, and its axioms Lean's own and no other
  local line ax a
  line=$(grep "^ACCEPT $1 " "$OUT/load.txt") || return 1
  ax=$(echo "$line" | sed -E 's/.* axioms=([^ ]*).*/\1/')
  for a in ${ax//,/ }; do case $a in propext|Quot.sound|Classical.choice) ;; *) return 1;; esac; done
  return 0
}
FILES=(v3/std/*.shard)
n=${#FILES[@]}
mkdir -p "$OUT/std"
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --store "$OUT/std" "${FILES[@]}" > "$OUT/load.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the std root did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|ERROR|POLICY|PENDING' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q "^LOAD: modules $n .* refused 0  pending 0 .* errors 0$" "$OUT/load.txt" || { bad "not every one of the $n files loaded clean"; grep '^LOAD:' "$OUT/load.txt"; }
# the former axioms of kernel/facts.shard, each a theorem under its old name
for t in mod_lo mod_hi ediv_mod_id div_unique mod_unique mul_comm mul_assoc mul_dist bshl_z bshl_s bshr_z bshr_s land_low lor_low lxor_low band_rec bor_rec bxor_rec; do
  checks=$((checks+1)); held std.facts.$t || bad "std/facts.shard: $t not accepted under Lean's axioms alone"
done
# law §10.3's rows, one theorem each
for t in int_ops tdiv_zero tmod_zero ediv_zero emod_zero le_decides beq_decides and_bridge bool_literals ctor_names list_names land_nat nat_sub_saturates refine_subtype size_nat word_wraps bytes_view str_buffer record_projections char_of_nat; do
  checks=$((checks+1)); held std.migration.$t || bad "std/migration.shard: $t not accepted under Lean's axioms alone"
done
# the records' evidence: std/order's 21 stated claims, std/nat's 4, std/div's 7, std/arith's 7
for t in order_le_refl order_lt_irrefl order_eq_refl order_lt_succ_from_lt order_le_implies_not_lt order_not_le_implies_lt order_le_trans order_lt_implies_ne order_lt_trans_to_ne order_lt_pred_from_lt order_lt_implies_ne_flip order_lt_trans_to_ne_flip order_le_succ_from_lt order_le_pred_from_lt order_lt_self_succ order_le0_succ order_le_succ_r order_le_succ_l order_lt_from_le_succ order_le_succ_shift order_le_succ_shift_r \
         nat_int_of_nat_nonneg nat_int_of_nat_succ nat_int_of_nat_zero nat_half_bound \
         div_ediv_mod_10_id div_mod_10_lo div_mod_10_hi div_div_lt div_div_nonneg div_ediv_nonneg div_mod_small \
         arith_idx_inner_simp arith_sub_zero arith_sub_sub_one arith_reassoc_succ arith_idx_cancel arith_cap_idx arith_idx_pred; do
  checks=$((checks+1)); held std.migration.$t || bad "std/migration.shard: the record's $t not accepted under Lean's axioms alone"
done
# std/bits.shard: the bits record's requirements as theorems (landing 3)
for t in pow2_z pow2_s pow2_pos pow2_8 pow2_32 pow2_64 band_zero_l bor_zero_l bxor_zero_l bxor_self bxor_cancel_right band_zero_r band_lo bor_lo bxor_lo shl_nonneg shr_nonneg band_le_l band_le_r bor_le_pow2 bxor_le_pow2 bor_le32 bxor_le32 bxor_lt32 bxor_lt64 shl1_mul shl_pow2 shl5_mul shl8_mul shl13_mul shr1_div shr_pow2 shr8_div shr17_div mask_pow2 mask_byte mask_word32 mask_word64; do
  checks=$((checks+1)); held std.bits.$t || bad "std/bits.shard: $t not accepted under Lean's axioms alone"
done
# std/list.shard: the realizations' equations, the list record's evidence, sum_append, the refinement (landing 3)
for t in List.length.realize_1 List.length.realize_2 List.append.realize_1 List.append.realize_2 List.reverseAux.realize_1 List.reverseAux.realize_2 List.reverse.realize_1 List.take.realize_1 List.take.realize_2 List.drop.realize_1 List.drop.realize_2 List.get?Internal.realize_1 List.get?Internal.realize_2 List.getD.realize_1 List.get.realize_1 List.get.realize_2 \
         take_nil_eq take_cons_eq drop_nil_eq drop_cons_eq get_opt_nil get_opt_cons get_bound get_nil_eq get_cons_eq list_append_nil_left list_append_cons list_rev_nil list_rev_cons list_len_nil list_len_cons list_take_nil list_drop_nil list_take_zero list_take_succ_cons list_drop_zero list_drop_succ_cons list_len_nonneg list_append_nil_right list_append_assoc list_len_append list_len_rev list_rev_append list_rev_rev list_drop_len list_drop_le list_drop_shortens list_take_le list_take_len sum_append insert mem_insert insert_sorted Sorted sorted_insert sorted_insert_val sorted_nil sorted_demo one_lt_two get_demo; do
  checks=$((checks+1)); held std.list.$t || bad "std/list.shard: $t not accepted under Lean's axioms alone"
done
checks=$((checks+1)); grep -q '^REALIZE Option.getD view ' "$OUT/load.txt" || bad "Option.getD is not realized through its matcher's definition (the view)"
decls=$(wc -l < "$OUT/std/order" 2>/dev/null || echo 0)
checks=$((checks+1)); grep -q "^STORE: .* new=$decls shared=0$" "$OUT/load.txt" || { bad "the store's count is not the order's $decls"; grep '^STORE:' "$OUT/load.txt"; }
echo "std_test: the std root — $n files, $decls declarations — loaded and stored in one process in $((e-s)) s"
s=$(date +%s)
"$EVAL" direct v3/kernel/verify_release.shard "$OUT/std" "$EXPORT" "$INDEX" > "$OUT/verify.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "verify_release failed on the std root (exit $rc)"; grep -v '^ACCEPT' "$OUT/verify.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q "^RELEASE: declarations $decls  accepted $decls " "$OUT/verify.txt" || { bad "K did not accept every one of the $decls declarations"; tail -1 "$OUT/verify.txt" | cut -c1-300; }
echo "std_test: K alone accepts the root's $decls declarations after Init's closure in $((e-s)) s"
echo "std_test: $checks checks, $fail failed"
exit $fail
