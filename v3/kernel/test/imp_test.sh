#!/usr/bin/env bash
# v3/kernel/test/imp_test.sh — the tb_len rung's definition gate (LANGUAGE.md §8.6 slice 4.2
# landing 0, rule 3): the model root v3/models/imp — the call-tier imp machine, the counted-heap
# runtime's table, the ghost heap and its invariant, the readbacks and the post-predicate, the
# source `len`, its compiled product and the theorem — loaded by the V3 loader with Init on
# demand: every function and every recursive specification DEFINE with its equations, no
# RUNNABLE, no refusal; the evaluator's nine descents discharged; the theorem pending on its
# `sorry` and nothing else pending (landing 1 proves it). The two library files the rung added,
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
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 --init "$EXPORT" --init-index "$INDEX" v3/models/imp/len.shard > "$OUT/load.txt" 2>&1; rc=$?
e=$(date +%s)
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the model root did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
# the summary: the eight modules (map, mem, list; imp, rt, heap, readback, len), nothing runnable,
# nothing refused, one pending — the theorem on its sorry
checks=$((checks+1)); grep -q '^LOAD: modules 8 .* runnable 0  refused 0  pending 1 .* errors 0$' "$OUT/load.txt" || { bad "the summary is not 8 modules, 0 runnable, 0 refused, 1 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
checks=$((checks+1)); grep -q '^PENDING models.imp.len.tb_len sorry$' "$OUT/load.txt" || bad "tb_len is not the pending theorem (sorry)"
checks=$((checks+1)); [ "$(grep -c '^PENDING ' "$OUT/load.txt")" = 1 ] || { bad "something besides tb_len is pending"; grep '^PENDING ' "$OUT/load.txt" | head -5; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE ' "$OUT/load.txt")" = 0 ] || { bad "a function is RUNNABLE without a definition"; grep '^RUNNABLE ' "$OUT/load.txt" | head -5 | cut -c1-200; }
# the evaluator: one function over the task type, defined with its equation, its nine descents discharged
defined() {   # NAME [N]: DEFINE with at least N equations and no pending
  local line n=${2:-1}
  line=$(grep "^DEFINE $1 " "$OUT/load.txt") || return 1
  echo "$line" | grep -q ' pending=' && return 1
  [ "$(echo "$line" | sed -E 's/.*equations=([^ ]*).*/\1/' | tr ',' '\n' | grep -c .)" -ge "$n" ]
}
checks=$((checks+1)); defined models.imp.imp.ipeval 1 || bad "ipeval is not defined with its equation and nothing pending"
checks=$((checks+1)); [ "$(grep -c '^DISCHARGE models.imp.imp.ipeval.dec_[1-9] proved$' "$OUT/load.txt")" = 9 ] || { bad "the evaluator's nine descents are not all discharged"; grep 'ipeval.dec_' "$OUT/load.txt" | head -9 | cut -c1-120; }
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
  checks=$((checks+1)); grep -q "^ACCEPT models.imp.$d " "$OUT/load.txt" || bad "models.imp.$d is not accepted"
done
# the two library files
checks=$((checks+1)); grep -q '^ACCEPT std.map.map_insert_shadow ' "$OUT/load.txt" || bad "std/map's four requirements are not theorems (map_insert_shadow)"
checks=$((checks+1)); [ "$(grep -c '^DISCHARGE std.mem.\(load_le\|store_le\).dec_1 proved$' "$OUT/load.txt")" = 2 ] || bad "std/mem's two word operations' descents are not discharged"
n=$(grep -c '^ACCEPT ' "$OUT/load.txt"); d=$(grep -c '^DEFINE ' "$OUT/load.txt")
echo "imp_test: the model root — 8 modules, $n declarations accepted, $d defined — loaded in $((e-s)) s; tb_len pending (sorry)"
echo "imp_test: $checks checks, $fail failed"
exit $fail
