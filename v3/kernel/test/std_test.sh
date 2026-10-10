#!/usr/bin/env bash
# v3/kernel/test/std_test.sh — the library's gate (v3/LANGUAGE.md §8.4 slice 3.27, landing
# 2; law §7.5): the std root — every file under v3/std — loaded in ONE process, Init on
# demand shared across the files (slices 3.25–3.26), every declaration written to a store,
# and the store replayed by K alone (verify_release.shard, slice 3.22's gate) after the
# closure of Init it cites. The records' theorems are held by name, each under no axiom but
# Lean's own (propext, Quot.sound, Classical.choice — what the cited Init theorems carry):
# std/facts.shard's former axioms (twelve at landing 2, fifteen at landing 3),
# std/migration.shard's one per row of law §10.3 and the records' evidence. Exit code = the
# number of checks that failed.
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
for t in mod_lo mod_hi ediv_mod_id div_unique mod_unique mul_comm mul_assoc mul_dist bshl_z bshl_s bshr_z bshr_s; do
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
