#!/usr/bin/env bash
# v3/kernel/test/imp_test.sh — the tb_len rung (LANGUAGE.md §8.6 slice 4.2, rules 3 and 4): the
# model root v3/models/imp — the call-tier imp machine, the counted-heap runtime's table, the
# ghost heap and its invariant, the readbacks and the post-predicate, the word and cell kits, the
# evaluator's step laws, the source `len`, its compiled product, the run lemmas and the theorem —
# loaded by the V3 loader with Init on demand: every function and every recursive specification
# DEFINE with its equations, no RUNNABLE, no refusal, nothing pending; the evaluator one function
# over the task type on the structural fuel, its fuel's descent discharged; `tb_len` ACCEPTED
# through I (landing 1), the run lemmas and the step laws with it. The root is then stored and the
# store verified by K alone (verify_release, rule 4). The two library files the rung added,
# std/map and std/mem, load with it. The load's seconds are printed on the engine that runs it.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "imp_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "imp_test: $*"; fail=$((fail+1)); }
mkdir -p "$OUT/imp"
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 --init "$EXPORT" --init-index "$INDEX" --store "$OUT/imp" v3/models/imp/len.shard > "$OUT/load.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the model root did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
# the summary: the eleven modules (map, mem, list; imp, rt, heap, readback, word, cell, step, len),
# nothing runnable, nothing refused, nothing pending
checks=$((checks+1)); grep -q '^LOAD: modules 11 .* runnable 0  refused 0  pending 0 .* errors 0$' "$OUT/load.txt" || { bad "the summary is not 11 modules, 0 runnable, 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
checks=$((checks+1)); [ "$(grep -c '^PENDING ' "$OUT/load.txt")" = 0 ] || { bad "something is pending"; grep '^PENDING ' "$OUT/load.txt" | head -5 | cut -c1-200; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE ' "$OUT/load.txt")" = 0 ] || { bad "a function is RUNNABLE without a definition"; grep '^RUNNABLE ' "$OUT/load.txt" | head -5 | cut -c1-200; }
# the theorem, its general form and the run lemmas: accepted, on Init's three axioms and nothing else
accepted() { grep -q "^ACCEPT $1 " "$OUT/load.txt"; }
for t in tb_len tb_len_aux len_ofNat len_S1 len_S2 len_S2_one len_E3a len_E3b len_T9_fail len_T9_ov len_T9_ret len_body_ret len_body_fail len_body_one; do
  checks=$((checks+1)); accepted models.imp.len.$t || bad "models.imp.len.$t is not accepted"
done
checks=$((checks+1)); grep '^ACCEPT models.imp.len.tb_len ' "$OUT/load.txt" | grep -q 'axioms=\(Classical.choice\|Quot.sound\|propext\),\(Classical.choice\|Quot.sound\|propext\),\(Classical.choice\|Quot.sound\|propext\) ' || { bad "tb_len does not rest on Init's three axioms alone"; grep '^ACCEPT models.imp.len.tb_len ' "$OUT/load.txt" | cut -c1-200; }
# the step laws (the evaluator's equations as laws over the structural fuel and over a Nat fuel),
# the cell kit and the word kit
for t in stmts_nil stmts_cons stmts_cons_fail stmt_set stmt_if_z stmt_if_nz stmt_loadw stmt_call_ret stmt_call_fail stmt_fail call_ret call_fail call_depth fuel_of_succ \
         stmts_nil_n stmts_cons_n stmts_cons_fail_n stmt_set_n stmt_if_z_n stmt_if_nz_n stmt_loadw_n stmt_call_ret_n stmt_call_fail_n stmt_fail_n call_ret_n call_fail_n call_depth_n; do
  checks=$((checks+1)); accepted models.imp.step.$t || bad "the step law models.imp.step.$t is not accepted"
done
for t in hfget_head hfget_tail haddrs_mem rd_hdr rd_slots rd_slot cells_ok_at e_brk_app_l e_brk_hexts cell_window band32 hword_tag; do
  checks=$((checks+1)); accepted models.imp.cell.$t || bad "the cell law models.imp.cell.$t is not accepted"
done
for t in imm_ofNat imm_small in63_ofNat in63_ofNat_lt; do
  checks=$((checks+1)); accepted models.imp.word.$t || bad "the word law models.imp.word.$t is not accepted"
done
# the evaluator: one function over the task type on the structural fuel, one equation per task and
# statement kind, nothing pending; the Nat fuel's conversion with its descent discharged
defined() {   # NAME [N]: DEFINE with at least N equations and no pending
  local line n=${2:-1}
  line=$(grep "^DEFINE $1 " "$OUT/load.txt") || return 1
  echo "$line" | grep -q ' pending=' && return 1
  [ "$(echo "$line" | sed -E 's/.*equations=([^ ]*).*/\1/' | tr ',' '\n' | grep -c .)" -ge "$n" ]
}
checks=$((checks+1)); defined models.imp.imp.ipeval 17 || bad "ipeval is not defined with its seventeen equations and nothing pending"
checks=$((checks+1)); [ "$(grep -c '^DISCHARGE models.imp.imp.ipeval.dec_' "$OUT/load.txt")" = 0 ] || bad "ipeval carries a descent obligation (the fuel is structural)"
checks=$((checks+1)); defined models.imp.imp.fuel_of 1 || bad "fuel_of is not defined with its equation"
checks=$((checks+1)); grep -q '^DISCHARGE models.imp.imp.fuel_of.dec_1 proved$' "$OUT/load.txt" || bad "fuel_of's descent is not discharged"
for f in ipstmt ipstmts ipwhile ipcall iprun; do
  checks=$((checks+1)); defined models.imp.imp.$f 1 || bad "$f (the old name, an abbreviation of ipeval) is not defined"
done
# the machine, the runtime, the heap and the readbacks: every function with its equations, one per arm
for spec in imp.iexp:7 imp.iop_val:13 imp.ilget:2 imp.ilset:2 imp.izeros_of:2 imp.iband_args:3 imp.ipexps:2 imp.ipfn_at:2 \
            rt.rt_init_fn:1 rt.rt_alloc_fn:1 rt.rt_inc_fn:1 rt.rt_dec_fn:1 rt.rt_fns:1 rt.rt_zero_heads:2 \
            heap.hword:1 heap.haddrs:2 heap.count_in:2 heap.inbound_cells:2 heap.hfind:2 heap.hfget:1 heap.rinc:1 heap.hexts:2 heap.fexts:2 heap.ext_all:1 \
            readback.ilsetf:2 readback.tb_mem:4 readback.tb_fam:4 readback.tb_w:4 len.len:2 len.len_ipfn:1; do
  checks=$((checks+1)); defined models.imp.${spec%%:*} "${spec##*:}" || bad "models.imp.${spec%%:*} is not defined with ${spec##*:} equations"
done
# the recursive specifications into Prop: defs with a recursion clause, defined with their two equations
for d in heap.hslots_rep heap.hcells_rep heap.hchain_rep heap.hfree_rep heap.slots_ok heap.cells_ok heap.counts_ok heap.roots_ok \
         heap.e_disj1 heap.e_disj heap.e_brk heap.anodup heap.emem heap.eall readback.rb_list; do
  checks=$((checks+1)); defined models.imp.$d 2 || bad "the specification models.imp.$d is not defined with its two equations"
done
# the non-recursive specifications and the statement's vocabulary: accepted as L definitions
for d in heap.heap_rep heap.hinv heap.hcell_rep heap.hraw_rep heap.slot_ok heap.raw_ok heap.cbands heap.live_ok heap.e_disj2 \
         readback.imm readback.dbl readback.hi readback.lo readback.in63 readback.rb_int readback.rb_cell readback.rb_slot readback.graw_none readback.tbh readback.tb_okr readback.tb_rv readback.ldok; do
  checks=$((checks+1)); accepted models.imp.$d || bad "models.imp.$d is not accepted"
done
# the two library files
checks=$((checks+1)); accepted std.map.map_insert_shadow || bad "std/map's four requirements are not theorems (map_insert_shadow)"
checks=$((checks+1)); [ "$(grep -c '^DISCHARGE std.mem.\(load_le\|store_le\).dec_1 proved$' "$OUT/load.txt")" = 2 ] || bad "std/mem's two word operations' descents are not discharged"
n=$(grep -c '^ACCEPT ' "$OUT/load.txt"); d=$(grep -c '^DEFINE ' "$OUT/load.txt")
echo "imp_test: the model root — 11 modules, $n declarations accepted, $d defined — loaded and stored in $((e-s)) s; tb_len accepted"
# the store, replayed by K alone (rule 4): every declaration the load admitted, accepted again
stored=$(sed -nE 's/^STORE: .* new=([0-9]+) .*/\1/p' "$OUT/load.txt" | head -1)
checks=$((checks+1)); [ -n "$stored" ] || { bad "no store was written"; grep '^STORE' "$OUT/load.txt" | head -2; stored=0; }
s=$(date +%s)
"$EVAL" direct v3/kernel/verify_release.shard "$OUT/imp" "$EXPORT" "$INDEX" > "$OUT/verify.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "verify_release failed on the model root's store (exit $rc)"; grep -v '^ACCEPT' "$OUT/verify.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q "^RELEASE: declarations $stored  accepted $stored " "$OUT/verify.txt" || { bad "K did not accept every one of the store's $stored declarations"; tail -1 "$OUT/verify.txt" | cut -c1-300; }
echo "imp_test: K alone accepts the root's $stored declarations from the store in $((e-s)) s"
echo "imp_test: $checks checks, $fail failed"
exit $fail
