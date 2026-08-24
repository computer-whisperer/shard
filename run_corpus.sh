#!/usr/bin/env bash
# Run the checker over a fixed corpus of entrypoints; print per-target output.
# Used to diff a kernel change against a baseline (same target set both runs).
# Targets run in parallel (JOBS env, default nproc); per-target output is
# buffered and emitted in list order, so output stays byte-diffable with any
# serial run.
set -u
# Engine selection, fastest fresh option first (see gate_sweep.sh):
# direct-compiled bin/shard_check (stamp-fresh only) > bin/shard_eval
# interpreting kernel/check.shard > Rust interpreter. EVAL env overrides with
# the interpreter command shape. The native chain is the DEV loop only --
# soundness-authority runs (pre-commit corpus diff, ledger, sidecar replay)
# use EVAL=./rust_bootstrap/target/release/eval explicitly.
CHECK_CMD=()
if [ -z "${EVAL:-}" ]; then
  if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
    CHECK_CMD=(bin/shard_check)
  elif [ -x bin/shard_eval ]; then
    [ -x bin/shard_check ] && echo "NOTE: bin/shard_check is STALE -- bin/rebuild.sh check (~1h) re-enables the fast sweep" >&2
    EVAL=bin/shard_eval
    if [ "$(bin/engine_stamp.sh)" != "$(cat bin/shard_eval.stamp 2>/dev/null)" ]; then
      echo "WARNING: bin/shard_eval is STALE vs kernel/compiler sources -- run bin/rebuild.sh" >&2
    fi
  else
    EVAL=./rust_bootstrap/target/release/eval
  fi
fi
[ ${#CHECK_CMD[@]} -eq 0 ] && CHECK_CMD=("$EVAL" run kernel/check.shard)

# ── run-mode pin gating (prune arc P4/P6, 2026-08-22): the search drivers
# spell failure as "SUPERPOSED-SEARCH FAIL …" / "AUDIT FAIL …", which the
# CI FAIL-set awk (^FAIL |^TYPE! ) never matched — so a failed search pin
# was invisible to the gate. pin_run turns a nonzero driver exit into a
# FAIL row the gate sees. NAME = the task (or driver) basename. SCOPE: every
# run-mode pin below (60 lines). The 17 task-driver pins (typed_superpose /
# typed_expr / imp_expr) had verified exit codes from the start; the other
# 43 probes were first recorded informationally (PIN-EXIT rows) and promoted
# once the CORPUS_LONG=1 fire (pipeline 391) showed all 43 exit 0.
pin_run() { # NAME CMD...
  local name=$1; shift
  if "$@"; then :; else echo "FAIL pin-$name (exit $?)"; fi
}

# ── S2b image-differential leg (docs/STORAGE.md §8a): SHARD_IMAGES=1
# routes every check in this run through the loader's image front door
# (a +images token ahead of the target; check.shard strips it anywhere
# in argv). Used by the CI corpus-images job (cold populate + warm hit,
# both FAIL-projections diffed against the text baseline). NEVER the
# default: the default corpus stays from-text — cache, never trust.
if [ -n "${SHARD_IMAGES:-}" ]; then
  mkdir -p .shard-cache/images
  CHECK_CMD+=(+images)
fi
JOBS="${JOBS:-$(nproc)}"
# ── wall-time instrumentation (prune arc P4, 2026-08-22): the parallel
# target phase hides its long pole, so every target records its own
# seconds and the log ends with a sorted top-20 plus phase totals. Lines
# never start with FAIL/TYPE!/SKIP, so the CI FAIL-set diff ignores them.
T_SCRIPT0=${EPOCHSECONDS:-$(date +%s)}
TARGETS=(
  examples/add_nat_zero.shard
  pins/lang/nat_prim.shard
  pins/lang/nat_absurd_rejects.shard
  pins/trust/axiom_kind_rejects.shard
  pins/trust/axiom_untagged_rejects.shard
  kernel/facts.shard
  # THE KERNEL ITSELF. Checking a target checks its whole IMPORT CLOSURE's
  # claims plus the type/totality gates over the merged module, so these two
  # entry points gate every kernel/*.shard between them: check.shard covers the
  # checking kernel (admit arith canon checker desugar driver expr_size loader
  # module proof proof_reader proof_size reader reduce stdlib term trace types,
  # ~58s) and eval.shard adds the evaluator pair (eval evm, ~13s). Before this
  # the corpus reached only canon/evm/expr_size/loader/module/reader/reduce/term
  # -- incidentally, via other targets' imports -- so checker, types, driver,
  # trace and arith were gated by NOTHING, and kernel/types' unverified tc_infer
  # measure obligation sat unnoticed. Per-file kernel targets would be pure
  # redundancy against these closures; add one only to sharpen a diagnosis.
  kernel/check.shard
  kernel/eval.shard
  std/bits/bits.shard
  std/rat/mod.req/gcd.shard
  std/rat/rat.shard
  examples/rat_demo.shard
  std/float/mod.req/float.shard
  std/float/mod.req/kit.shard
  std/float/mod.req/pack.shard
  std/float/mod.req/grs.shard
  std/float/mod.req/ops2.shard
  std/float/mod.req/wf.shard
  std/float/mod.req/dec.shard
  std/float/mod.req/hex.shard
  examples/float_val_compute.shard
  std/f32/f32.shard
  std/f64/f64.shard
  examples/float_surface_demo.shard
  examples/bits_demo.shard
  std/axiom_scope_rejects.shard
  examples/ledger_dep/ledger_dep.shard
  pins/proof/auto_demo.shard
  pins/proof/auto_missing_rejects.shard
  pins/proof/admit_demo.shard
  examples/bytes_bridge.shard
  pins/trust/contract_demo.shard
  pins/lang/decl_rejects.shard
  pins/proof/div_pairs.shard
  pins/lang/double_claims.shard
  pins/lang/use_demo.shard
  pins/lang/double_lib.shard
  pins/lang/shadow_rejects.shard
  pins/lang/finsplit_shadow_rejects.shard
  pins/lang/prim_shadow_rejects.shard
  pins/proof/finsplit_test.shard
  pins/proof/have_test.shard
  pins/proof/inject_basic.shard
  pins/proof/inject_rejects.shard
  pins/proof/rewrite_at.shard
  pins/proof/rewrite_at_rejects.shard
  pins/proof/sort_gate.shard
  pins/proof/sort_gate_rejects.shard
  pins/proof/conv_probe.shard
  pins/proof/conv_rejects.shard
  pins/proof/cert_rows.shard
  pins/proof/cert_rows_rejects.shard
  pins/proof/rewrite_with_occ.shard
  pins/proof/rewrite_with_occ_rejects.shard
  pins/proof/prove_cond_mine.shard
  pins/lang/homonym_dispatch.shard
  pins/lang/runhom_run.shard
  pins/proof/inspect_demo.shard
  pins/proof/cite_resolves.shard
  pins/proof/cite_rejects.shard
  examples/req_dir_demo/consumer.shard
  examples/req_dir_demo/demo/demo.shard
  examples/req_gate_rejects/mod.req.shard
  pins/proof/lia_basics.shard
  pins/proof/lia_rejects.shard
  pins/proof/list_named_hyp.shard
  pins/proof/named_haves.shard
  pins/proof/named_case_hyps.shard
  pins/proof/reflect_demo.shard
  pins/proof/reflect_rejects.shard
  pins/lang/refine_basic.shard
  pins/lang/refine_rejects.shard
  pins/lang/refine_return.shard
  pins/lang/refine_return_rejects.shard
  pins/lang/refine_circular_rejects.shard
  pins/lang/refine_try.shard
  pins/lang/refine_try_rejects.shard
  pins/lang/refine_shadow_rejects.shard
  pins/lang/utf8_compute.shard
  examples/str_demo.shard
  pins/lang/module_gate_rejects.shard
  pins/lang/parse_rejects.shard
  examples/pending_demo.shard
  examples/reverse_proof.shard
  pins/proof/rewrite_arms_test.shard
  pins/proof/rewrite_with_demo.shard
  pins/proof/tracer_demo.shard
  pins/proof/unfold_scrutinee.shard
  pins/trust/trust_ledger.shard
  pins/lang/types_gate.shard
  pins/lang/types_gate_cite.shard
  pins/lang/occurs_rejects.shard
  pins/proof/wf_induct_demo.shard
  pins/proof/measure_clause.shard
  pins/proof/measure_import_synth.shard
  pins/proof/measure_lex_demo.shard
  examples/io/cat_loop.shard
  tools/invoke/invoke_fixture.shard
  tools/invoke/invoke_probe.shard
  pins/proof/measure_tree_demo.shard
  pins/proof/nested_measure.shard
  examples/mem_reverse.shard
  examples/mem_copy.shard
  pins/proof/mutual_toy.shard
  pins/lang/record_proto.shard
  pins/lang/record_basic.shard
  models/imp/imp.shard
  models/imp/probes/imp_probe.shard
  models/imp/probes/ipcall_probe.shard
  models/imp/rt.shard
  models/imp/probes/rt_run.shard
  tools/impc/impc.shard
  tools/impc/fixtures/micro.shard
  tools/impc/fixtures/micro_ipc_out.shard
  tools/impc/fixtures/micro_run.shard
  tools/impc/fixtures/micro_x86_run.shard
  tools/impgen/fixtures/imp_scalar.shard
  models/imp/to_wasm.shard
  models/imp/probes/imp_wasm_bridge.shard
  models/imp/to_x86.shard
  models/imp/probes/imp_x86_bridge.shard
  models/imp/probes/fra_kit.shard
  models/imp/probes/fra_micro.shard
  models/imp/probes/rth_kit.shard
  models/imp/probes/rth_run.shard
  tools/impgen/fixtures/imp_loop.shard
  models/imp/probes/imp_wasm_loop_bridge.shard
  models/imp/probes/imp_x86_loop_bridge.shard
  models/imp/probes/vx86_acc_probe.shard
  models/imp/probes/vx86_oracle_probe.shard
  models/imp/probes/ipatch_probe.shard
  models/imp/probes/ilinv_probe.shard
  tools/impgen/fixtures/impgen_wasm_out.shard
  tools/impgen/fixtures/impgen_x86_out.shard
  tools/impgen/fixtures/impgen_wasm_loop_out.shard
  tools/impgen/fixtures/impgen_x86_loop_out.shard
  tools/impgen/fixtures/imp_mixed.shard
  tools/impgen/fixtures/impgen_wasm_mixed_out.shard
  tools/impgen/fixtures/impgen_x86_mixed_out.shard
  tools/impgen/blueprints/iwg_probe.shard
  tools/impgen/blueprints/sqw_probe.shard
  tools/impgen/blueprints/sqx_probe.shard
  tools/impgen/blueprints/sqmw_probe.shard
  tools/impgen/blueprints/sqm2_probe.shard
  tools/impgen/blueprints/sqmc_probe.shard
  tools/impgen/blueprints/sqxc_probe.shard
  examples/weld_probe.shard
  tools/impgen/blueprints/sqbw_probe.shard
  tools/impgen/blueprints/sqbx_probe.shard
  tools/impgen/blueprints/sqblw_probe.shard
  tools/impgen/blueprints/sqblx_probe.shard
  tools/impgen/blueprints/sqbsx_probe.shard
  tools/impgen/fixtures/imp_if.shard
  tools/impgen/fixtures/impgen_wasm_if_out.shard
  tools/impgen/fixtures/impgen_x86_if_out.shard
  tools/impgen/fixtures/imp_ifl.shard
  tools/impgen/fixtures/impgen_wasm_ifl_out.shard
  tools/impgen/fixtures/impgen_x86_ifl_out.shard
  models/wasm/wasm.shard
  models/wasm/encode.shard
  models/wasm/probes/wasm_smoke.shard
  models/wasm/probes/wasm_pieces.shard
  models/wasm/probes/wasm_weld.shard
  models/wasm/probes/wasm_weld_out.shard
  models/wasm/diff/wasm_diff_run.shard
  models/wasm/probes/wasm_rev.shard
  models/wasm/probes/wasm_copy.shard
  models/wasm/probes/lowered_form.shard
  pins/lang/w64_probe.shard
  models/x86/x86.shard
  models/x86/encode.shard
  models/x86/probes/x86_pieces.shard
  std/mem/mem.x86.shard
  models/x86/probes/xmemcall_probe.shard
  models/x86/probes/x86_window_law.shard
  models/x86/probes/xsibcall_probe.shard
  models/x86/probes/xchain_probe.shard
  models/x86/probes/xloopcall_probe.shard
  models/x86/probes/xintloop_probe.shard
  models/x86/probes/xcopyloop_probe.shard
  models/x86/probes/xtransform_probe.shard
  models/x86/probes/xfoldloop_probe.shard
  models/x86/probes/xdiv_probe.shard
  models/linux/probes/lxkernel_probe.shard
  models/x86/probes/xworld_probe.shard
  models/x86/probes/xadequacy_probe.shard
  models/x86/probes/xweff_probe.shard
  models/x86/probes/xvector_probe.shard
  models/x86/probes/stdin_count_probe.shard
  examples/addw/addw_src.shard
  examples/sha256sum/sha256sum_src.shard
  examples/sha256sum/sha256sum_x86_out.shard
  examples/sha256sum/sha256sum_elf.shard
  models/x86/probes/stdin_echo_probe.shard
  examples/addw/addw_x86_out.shard
  models/x86/probes/xitoa_probe.shard
  tools/lowbuild/fixtures/x86div_src.shard
  tools/lowbuild/fixtures/x86div_out.shard
  tools/lowbuild/fixtures/x86itoa_src.shard
  tools/lowbuild/fixtures/x86itoa_out.shard
  models/x86/probes/xbinadd_probe.shard
  examples/add/add_src.shard
  examples/add/add_x86_out.shard
  models/x86/probes/xbinsum_probe.shard
  models/x86/probes/xid_probe.shard
  tools/lowbuild/fixtures/bytesum_src.shard
  tools/lowbuild/fixtures/bytesum_x86_out.shard
  models/wasm/probes/libmod_probe.shard
  tools/lowcheck/fixtures/lib_form.shard
  tools/lowcheck/fixtures/lib_form_rejects.shard
  tools/lowbuild/fixtures/purelib_src.shard
  tools/lowbuild/fixtures/purelib_out.shard
  tools/build/build_products.shard
  tools/build/build.shard
  tools/lowbuild/fixtures/purelib_x86_out.shard
  tools/lowbuild/fixtures/arglen_src.shard
  tools/lowbuild/fixtures/arglen_x86_out.shard
  tools/lowbuild/fixtures/echoarg_src.shard
  tools/lowbuild/fixtures/echoarg_x86_out.shard
  models/riscv/riscv.shard
  models/riscv/probes/riscv_smoke.shard
  models/riscv/encode.shard
  models/riscv/diff/riscv_diff_run.shard
  models/riscv/loopkit.shard
  models/riscv/probes/riscv_pieces.shard
  tools/lowbuild/fixtures/upcase_src.shard
  tools/lowbuild/fixtures/upcase_x86_out.shard
  tools/lowbuild/fixtures/parse_src.shard
  tools/lowbuild/fixtures/parse_x86_out.shard
  pins/trust/bin_entry_rejects.shard
  models/x86/diff/x86_diff_run.shard
  models/wasm/probes/rep_probe.shard
  models/wasm/probes/lowfrag_probe.shard
  models/wasm/probes/divfrag_probe.shard
  models/wasm/probes/bitfrag_probe.shard
  models/wasm/probes/wordfrag_probe.shard
  tools/lowcheck/fixtures/lowcheck_rejects.shard
  pins/lang/record_rejects.shard
  pins/lang/record_sugar_rejects.shard
  pins/lang/statement_sugar.shard
  pins/lang/statement_sugar_rejects.shard
  pins/proof/chain_sugar.shard
  pins/proof/chain_sugar_rejects.shard
  pins/proof/named_cite_rejects.shard
  pins/proof/compute_stop.shard
  pins/proof/simp_stop.shard
  pins/proof/subterm_induct.shard
  pins/proof/subterm_induct_rejects.shard
  pins/proof/below_resolve_rejects.shard
  pins/proof/subterm_ctorless_rejects.shard
  pins/proof/rewrite_arm_capture_rejects.shard
  pins/proof/struct_clause.shard
  pins/proof/struct_mutual_list.shard
  pins/proof/adq13_probe.shard
  pins/proof/natview_pin.shard
  examples/natview_pin2.shard
  examples/natview_rejects.shard
  pins/proof/zerocase_rejects.shard
  pins/lang/canon_pin.shard
  pins/lang/canon_rejects.shard
  tools/canon/rewrite.shard
  tools/canon/canon.shard
  tools/canon/census.shard
  tools/canon/hash.shard
  pins/proof/hash_pin.shard
  pins/proof/parlet_pin.shard
  examples/render_model.shard
  examples/modules_demo/consumer.shard
  examples/modules_demo/views/module_view.shard
  examples/modules_demo/views/consumer_view.shard
  examples/modules_demo/views/necessity.shard
  examples/calc/calc.shard
  examples/calc/calc_spec.shard
  examples/calc/calc_proof.shard
  examples/calc/calc_spec_tests.shard
  examples/calc/calc_reconcile_tests.shard
  examples/calc/calc_show.shard
  examples/calc/calc_show_run.shard
  examples/calc/calc_ndigit.shard
  examples/calc/calc_equiv.shard
  examples/calc/calc_app.shard
  examples/calc/calc_app_spec.shard
  examples/calc/calc_app_trace.shard
  examples/calc/calc_app_world.shard
  examples/snake_game/snake_game.req.shard
  examples/snake_game/snake.shard
  examples/snake_game/snake_view.shard
  examples/snake_game/snake_app.shard
  examples/snake_game_3/game/game.shard
  examples/snake_game_3/render/render.shard
  examples/snake_game_3/snake.shard
  std/mem/mem.shard
  std/list.shard
  std/map.shard
  std/arith.shard
  std/div.shard
  std/nat.shard
  std/order.shard
  std/rng/rng.shard
  std/rng/rng.wasm.shard
  std/bytes/bytes.shard
  std/str/utf8.shard
  std/str/str.shard
  std/list/list.shard
  std/arith/arith.shard
  std/div/div.shard
  std/map/map.shard
  std/word/word.shard
  std/nat/nat.shard
  std/sha256/sha256.shard
  std/sha256/sha256.imp.shard
  std/sha256/sha256.patch.shard
  std/sha256/sha256.xpatch.shard
  std/sha256/sha256.xconv.shard
  std/sha256/sha256.xchain.shard
  std/sha256/sha256.xcomp.shard
  std/sha256/sha256.stream.shard
  examples/sha256sum/sha256sum_stream_src.shard
  std/sha256/impgen_wasm_out.shard
  std/sha256/impgen_x86_out.shard
  std/sha256/sha256.weld.shard
  std/sha256/sha256.sweld.shard
  std/sha256/sha256.shani.shard
  models/x86/diff/shani_diff_run.shard
  examples/sha256sum/sha256sum_stream_x86.shard
  std/sha256/sha256.shaniw.shard
  std/sha256/sha256.snweld.shard
  examples/sha256sum/sha256sum_shani_x86.shard
  examples/sha256sum/sha256sum_stream_elf.shard
  examples/sha256sum/sha256sum_shani_elf.shard
  models/x86/link.shard
  examples/sha256sum/sha256sum_dispatch_x86.shard
  examples/sha256sum/sha256sum_dispatch_elf.shard
  pins/proof/sketch_pin.shard
  meta/sketch/mod.req.shard
  meta/invoke/prepared.shard
  meta/census/mod.req.shard
  meta/rewrite/mod.req.shard
  meta/search/mod.req.shard
  tools/search/rev_obj.shard
  tools/search/tasks/pure_program_obj.shard
  tools/search/rev.shard
  tools/search/search.shard
  tools/search/census.shard
  tools/search/catalog.shard
  tools/search/catalog_pressure.shard
  tools/search/sym.shard
  tools/search/frontier.shard
  examples/spell_pin.shard
  tools/search/render_gate.shard
  tools/search/gen/rev_synth.shard
  tools/search/gen/cat_bracket.shard
  tools/search/superpose.shard
  tools/search/subsume.shard
  tools/search/imp_expr.shard
  tools/search/typed_grammar.shard
  tools/search/typed_expr.shard
  tools/search/typed_superpose.shard
  tools/search/transition_mine.shard
  tools/search/protocol_probe.shard
  tools/search/arena_probe.shard
  tools/search/theorem_scope.shard
  tools/search/profile_census.shard
  tools/search/rewrite_probe.shard
  tools/search/constraint_probe.shard
  tools/search/nonlinear_constraint_probe.shard
  tools/search/region_probe.shard
  tools/search/nonlinear_symbolic_probe.shard
  tools/search/constraint_superpose_probe.shard
  tools/search/typed_rule_probe.shard
  tools/search/tasks/imp_add1.shard
  tools/search/tasks/imp_mix.shard
  tools/search/tasks/typed_imp_add1.shard
  tools/search/tasks/typed_imp_mix.shard
  tools/search/tasks/typed_shard_call.shard
  tools/search/tasks/typed_wasm_add1.shard
  tools/search/tasks/typed_x86_calculator.shard
  tools/search/tasks/typed_x86_calculator4.shard
  tools/search/tasks/typed_append_value.shard
  tools/search/tasks/observer_model.shard
  tools/search/tasks/typed_observer_value.shard
  tools/search/tasks/typed_expect_empty.shard
  tools/search/tasks/typed_list_first.shard
  tools/search/tasks/typed_observer_conjunctive_first.shard
  tools/search/tasks/typed_observer_conjunctive.shard
  tools/search/gen/imp_add1_refinement.shard
  tools/search/gen/imp_mix_refinement.shard
  tools/search/gen/x86_calculator_refinement.shard
  tools/search/gen/x86_calculator4_refinement.shard
  models/pio/pio.shard
  models/pio/encode.shard
  models/pio/probes/pio_smoke.shard
  models/pio/diff/pio_vecrun.shard
  models/pio/diff/pio_vecgate.shard
  tools/search/tasks/typed_pio_square.shard
  tools/search/gen/pio_square_refinement.shard
  tools/search/tasks/typed_pio_dme.shard
  tools/search/gen/pio_dme_refinement.shard
  tools/search/tasks/pio_transition_mining.shard
  tools/search/tasks/pio_transition_window.shard
  tools/search/tasks/pio_dme_model.shard
  tools/search/tasks/pio_dme_free.shard
  tools/search/tasks/swap_route_model.shard
  tools/search/tasks/swap_route.shard
  tools/search/tasks/swap_route5_model.shard
  tools/search/tasks/swap_route5.shard
  tools/search/tasks/pcb_route_model.shard
  tools/search/pcb_route_probe.shard
  tools/search/pcb_detour_probe.shard
  tools/search/tasks/pcb_time_model.shard
  tools/search/pcb_time_probe.shard
  tools/search/tasks/pcb_cost_model.shard
  tools/search/pcb_cost_probe.shard
  tools/search/pcb_cap_probe.shard
  tools/search/pcb_lns_probe.shard
)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

T_TARGETS0=${EPOCHSECONDS:-$(date +%s)}
for i in "${!TARGETS[@]}"; do
  while (( $(jobs -rp | wc -l) >= JOBS )); do wait -n; done
  {
    echo "=== ${TARGETS[$i]} ==="
    t0=${EPOCHREALTIME:-$(date +%s)}
    "${CHECK_CMD[@]}" "${TARGETS[$i]}" 2>&1
    awk -v a="$t0" -v b="${EPOCHREALTIME:-$(date +%s)}" -v f="${TARGETS[$i]}" \
      'BEGIN { printf "%.1f %s\n", b - a, f }' > "$TMP/$i.secs"
  } > "$TMP/$i" &
done
wait

for i in "${!TARGETS[@]}"; do cat "$TMP/$i"; done
T_TARGETS=$(( ${EPOCHSECONDS:-$(date +%s)} - T_TARGETS0 ))
T_REST0=${EPOCHSECONDS:-$(date +%s)}

echo "=== corpus: per-target wall seconds, top 20 (target phase ${T_TARGETS}s at JOBS=$JOBS) ==="
cat "$TMP"/*.secs | sort -rn | head -20

# Scope-mode pin: the resolver-decision report (use-line classification +
# rebind verdicts) for the snake v3 impl — the item-vs-alias and rebind
# machinery's regression surface.
echo "=== scope: snake_game_3/game ==="
"${CHECK_CMD[@]}" scope examples/snake_game_3/game/game.shard 2>&1

# Invocation-shape pin: an ABSOLUTE target must be refused (exit 2) — module
# identity is only sound for repo-root-relative paths; a silent acceptance
# here would strip kernel/stdlib's core identity and quietly disable the
# theory backends (the false "purity bug" of 2026-06-10). See loader.shard
# visit / reader.shard path_escapes.
# Weld-regen pin: the committed weld certificate must be byte-identical to
# what the compile script emits from the CURRENT pieces (drift in either
# direction fails; the certificate's citations already fail loudly on
# structural drift, this catches the silent kind).
echo "=== weld: regen wasm_weld_out ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run models/wasm/probes/wasm_weld.shard > "$TMP/weld.raw" 2>/dev/null
  bin/shard_eval run tools/shardfmt/shardfmt.shard "$TMP/weld.raw" > "$TMP/weld.fmt" 2>/dev/null
  if diff -q "$TMP/weld.fmt" models/wasm/probes/wasm_weld_out.shard >/dev/null; then
    echo "REGEN OK (byte-identical)"
  else
    echo "REGEN DRIFT: emitted certificate differs from models/wasm/probes/wasm_weld_out.shard"
  fi
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# PIO vector-data regen pin: the committed generated data file must be
# byte-identical to gen_vectors.py's output re-formatted (docs/PIO.md §6 —
# the gate claim already fails loudly on semantic drift; this catches
# silent drift of the data itself).
echo "=== pio: regen pio_vectors_data ==="
if [ -x bin/shard_eval ] && command -v python3 >/dev/null; then
  python3 tools/piovec/gen_vectors.py > "$TMP/piovec.raw" 2>/dev/null
  bin/shard_eval run tools/shardfmt/shardfmt.shard "$TMP/piovec.raw" > "$TMP/piovec.fmt" 2>/dev/null
  if diff -q "$TMP/piovec.fmt" models/pio/diff/pio_vectors_data.shard >/dev/null; then
    echo "REGEN OK (byte-identical)"
  else
    echo "REGEN DRIFT: generated vector data differs from models/pio/diff/pio_vectors_data.shard"
  fi
else
  echo "SKIPPED (no bin/shard_eval or python3)"
fi

# Prove-regen pin (prove tool gate, lockstep-arc slice 0): tools/prove
# re-run FROM SCRATCH on two fixture files must reproduce their committed
# sidecars byte-identically.  This is the only place the corpus EXECUTES
# prove, so kernel-seam signature drift (shard#21's class — the 7-arg
# apply_rewrite_with_env drift crashed prove for 13 days unseen), ladder
# regressions, and sidecar fixed-point drift all land HERE instead of in a
# downstream session.  prove_cond_mine exercises the conditional-citation
# + premise-mining rungs; auto_demo spans compute/induction/case-on/
# lemma-citation/farkas/discovery — both solve in seconds, cheap enough
# for the default tier.  Committed sidecars are restored unconditionally —
# the tree stays clean even when the gate fails.
echo "=== prove: regen fixture sidecars (run-mode tool gate) ==="
if [ -x bin/shard_eval ]; then
  for pf in pins/proof/prove_cond_mine pins/proof/auto_demo; do
    cp "$pf.auto.shard" "$TMP/prove_gate.bak"
    rm "$pf.auto.shard"
    if bin/shard_eval run tools/prove/prove.shard "$pf.shard" \
         > "$TMP/prove_gate.log" 2>&1 \
       && diff -q "$pf.auto.shard" "$TMP/prove_gate.bak" >/dev/null 2>&1; then
      echo "REGEN OK (byte-identical) $pf"
    else
      echo "FAIL prove-regen ($pf)"
      tail -5 "$TMP/prove_gate.log"
    fi
    cp "$TMP/prove_gate.bak" "$pf.auto.shard"
  done
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# The reach moves-floor pin (lock-step arc LS2, SEARCH.md LS-law 3): the
# benchmark harness executes end-to-end and the policy-free floor on the
# two prove fixtures stays where it was measured (0/4 cond_mine, 5/16
# auto_demo). A deliberate move/closer change updates these totals with
# the change (regen-canon contract); anything else drifting them is a bug
# in the oracle kit or the harness.
echo "=== reach: moves-floor pin (LS2 harness gate) ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run tools/reach/reach.shard \
      pins/proof/prove_cond_mine.shard pins/proof/auto_demo.shard \
      > "$TMP/reach_pin.log" 2>&1
  if grep -q "^TOTAL pins/proof/prove_cond_mine.shard 0/4$" "$TMP/reach_pin.log" \
     && grep -q "^TOTAL pins/proof/auto_demo.shard 5/16$" "$TMP/reach_pin.log"; then
    echo "REACH PIN OK (floor 0/4 + 5/16)"
  else
    echo "FAIL reach-pin (moves floor drifted)"
    tail -8 "$TMP/reach_pin.log"
  fi
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Nat-former RUN pin: ground construction/packing, view matching, deep
# patterns under the RUN engine (ev). Output must be engine-independent.
echo "=== nat: run probe ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run pins/lang/nat_run_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Kernel-facts ground differential: every axiom in kernel/facts.shard is
# checked as a ground equation against the LIVE prims over a value grid
# (the reviewed core-math set rides executable evidence). Self-checking:
# one OK/FAIL line per section, exit 0 iff all OK. NOTE: a stuck run-mode
# program prints nothing and exits 0, so the pin is the OK LINES, which a
# stuck run would drop from the diff.
echo "=== facts: ground differential ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run pins/proof/facts_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# 64-bit range pin (the x86_64 arc's probe 0a): ground facts at and past
# 2^64 through the compiled chain's bignum runtime. Same OK-line pin
# discipline as facts_probe; the CHECK side rides the kernel TARGETS list
# (wrap64_id's farkas certificate carries 2^64 coefficients).
echo "=== facts: 64-bit range probe ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run pins/lang/w64_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Dynamic-invocation pin: meta/invoke loads a fixture's closure AT RUNTIME
# (kernel-as-a-module), marshals values across the meta-level boundary, and
# calls fns by local name — the mod.build driver's foundation. Self-checking
# app: one OK/FAIL line per case, exit 0 iff all OK.
echo "=== invoke: dynamic-invocation probe ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run tools/invoke/invoke_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# ---- long-tier engine pins (CORPUS_LONG=1) ---------------------------------
# Every section from here to the matching `fi` is a search/PIO ENGINE RUN
# (bin/shard_eval executing censuses, hunters, superposition and mining
# tasks) — seconds-to-minutes each, ~45-90 min of serial wall clock in
# total (measured 2026-07-17: the block grew the CI corpus from ~35 min
# to 80+). The default corpus skips them; CORPUS_LONG=1 includes them
# (the corpus-long CI job, or an occasional local run). New search-family
# run pins belong INSIDE this guard; their proof artifacts stay in
# TARGETS above as ordinary check targets regardless.
if [ "${CORPUS_LONG:-0}" = "1" ]; then

# Typed model-fragment pin: a task supplies only imp signature/grammar data,
# a combined wf+observation probe, a target vector, and one certified witness.
# The generic engine builds the grammar, rank/unrank-checks all 52 members,
# keeps an exact full-vector census, canonically spells the lowest solution,
# and requires the certified witness to occur in the solution set.  Its G4
# spec-to-wasm proof is a check target above.
echo "=== search: typed imp expression pin ==="
if [ -x bin/shard_eval ]; then
  pin_run imp_add1 bin/shard_eval run tools/search/imp_expr.shard tools/search/tasks/imp_add1.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# General typed-grammar pins: first exercise an ISA-free Let template whose
# body hole receives a new BVar, then drive the same reflected scope engine
# over imp expressions and parametric Wasm instruction lists.  typed_expr and
# typed_superpose are ALSO check targets since 42902ae closed the
# tc_infer/tc_arms measure gap that used to fail in their kernel/types
# closure; the executable pins below remain the behavioral half.
# Task-protocol scan pin: the hook table + structural refusal matrix
# (unknown search_* fn, witness multiplicity, solo spine hook, flat
# vocabulary under search_environment) — every rule pinned on synthetic
# modules; every real task passes through the same scan via te_config.
# Also the claim-ladder pins: every mode x witness cell of te_claim
# (census / absence / first) and both te_screen_legal directions, refusal
# messages byte-exact — the legality matrix te_load_task enforces for
# every engine consuming the TeTask record.
echo "=== search: task-protocol scan pin ==="
if [ -x bin/shard_eval ]; then
  pin_run protocol_probe bin/shard_eval run tools/search/protocol_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: graduated meta rewrite profile pin ==="
if [ -x bin/shard_eval ]; then
  pin_run rewrite_probe bin/shard_eval run tools/search/rewrite_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: theorem-scoped canon license pin ==="
if [ -x bin/shard_eval ]; then
  # Checked append licenses generate four separable formation clauses. The
  # generic prefix census audits every depth-2 profile (243 -> 31) and counts
  # the harder depth-3 prefixes (59295 -> 94), exhaustively auditing the selected
  # endpoint. The identical generic profile also rewrites symbolic neutrals and
  # re-enters on nested calls produced by a theorem RHS.
  pin_run theorem_scope_probe bin/shard_eval run tools/search/theorem_scope_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: theorem-filtered dynamic superposition pin ==="
if [ -x bin/shard_eval ]; then
  # Four checked append requirements quotient 210,066,388,900 raw depth-5
  # expressions to 63 normal forms before narrowing; the audit enumerates only
  # that filtered rank space and agrees on its unique target.
  pin_run typed_append_value bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_append_value.shard audit
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: checked observer-root reduction pin ==="
if [ -x bin/shard_eval ]; then
  # The checked law equates observations rather than candidate values. It
  # removes Leaf(Noise 1) only at the candidate root: Wrap(Leaf(Noise 1))
  # remains in the three-member quotient because congruence was not proved.
  pin_run typed_observer_value bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_observer_value.shard audit
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: residual theorem-constraint pin ==="
if [ -x bin/shard_eval ]; then
  # A coupled two-child observer law cannot be weakened into independent
  # formation exclusions. The graduated constraint layer classifies open,
  # redex, clean, and root-scoped regions; then both search engines agree that
  # exactly one two-member subtree of the eight generic ADT candidates is
  # theorem-redundant. Repeated-variable probes separately pin structural
  # equality across concrete, partial-grammar, prepared, and symbolic values.
  # region_probe's second line pins the RELATION-AWARE region algebra (R1):
  # the diagonal counts as ONE region (EQ-3/NE-6), the 3-clique closes by
  # inclusion-exclusion (6), deep domains diagonalize (4/12) and tuple-ne
  # descends (6), eq fixes its partner, a last closed-template ne pair
  # becomes a forbid, split partitions 9=3+6, release restores by policy,
  # and the v1 boundaries (iso/liveness/closure/cap/rank) refuse loudly.
  # region_probe also pins the R3 entailment decision (sk_rels_decide):
  # transitive eq reachability, singleton ne lifted across eq classes,
  # vector ne entails nothing pointwise, unrelated stays Unknown.
  # nonlinear_constraint_probe pins the PAIR-CARRYING verdicts (R2) and
  # RELATION-AWARE evaluation (R3): the both-open equality reports
  # MsBlockedPair 0/1 (direct and prepared), an eq/ne relational region
  # DECIDES it (Redex/Clear — the split-child progress property), the exact
  # equality partition consumes it TERMINALLY (EQ3+NE6 in 1+1 regions,
  # 1 relational split, 0 ground forks), the non-isomorphic-domain door
  # refusal falls back to ground (1+3/F2), a two-pair conjunction degrades
  # its second pair and splits under each diagonal assignment (9+72/F4S3),
  # a distinct guard over two open holes pairs through the conditioned
  # verdict and is decided by eq/ne relation events, and the PREPARED plan
  # partition now SPLITS the pair relationally (3+6 in 1+1 regions, S1;
  # was the R2 degradation pin 3+3/F4) with the non-isomorphic plan case
  # falling back to ground (1+3 in 1+2, F2S0).
  # constraint_superpose_probe's DIAG-SPLIT line pins the DRIVE-level R3
  # split: a nonlinear diagonal rule before a constant-pass query settles
  # as FOUND 6 (the relational ne child, counted exactly) + KILLED 3 (the
  # cited eq child) in 2 regions / 1 boundary, and the relational passing
  # region yields an in-region first-walk representative.
  # region_probe's third line pins the SCHEMA-side relational algebra (R4):
  # relate/split doors over occurrence addresses (iso = schema-id equality),
  # cross-schema refusal, and the graduated language-parametric first walk
  # (eq propagates to Pair(A0,A0); ne separates to Pair(A0,A1); a deep
  # fixed pair descends to the child-occurrence tuple and the backtracker
  # resolves it).  constraint_superpose_probe's SCHEMA-DIAG-SPLIT line pins
  # the same diagonal drive over a regular schema: the single-alternative
  # root is transparent, the occurrence pair splits at the empty region
  # (1 boundary), and the relational passing region's representative comes
  # from the graduated walk.
  pin_run constraint_probe bin/shard_eval run tools/search/constraint_probe.shard
  pin_run nonlinear_constraint_probe bin/shard_eval run tools/search/nonlinear_constraint_probe.shard
  pin_run region_probe bin/shard_eval run tools/search/region_probe.shard
  pin_run nonlinear_symbolic_probe bin/shard_eval run tools/search/nonlinear_symbolic_probe.shard
  # Contextual sequence pressure keeps its checked traversal, conditional
  # guard, prepared-cache, and mined-schema contraction boundaries generic.
  pin_run spine_probe bin/shard_eval run tools/search/spine_probe.shard
  pin_run guard_probe bin/shard_eval run tools/search/guard_probe.shard
  pin_run affine_guard_probe bin/shard_eval run tools/search/affine_guard_probe.shard
  pin_run int_order_guard_probe bin/shard_eval run tools/search/int_order_guard_probe.shard
  pin_run antiunify_probe bin/shard_eval run tools/search/antiunify_probe.shard
  # Empirical affine families are inferred from orbit support and each is
  # replayed independently; a two-support coincidence must fail on the full
  # transition corpus before it can reach theorem classification.
  pin_run transition_affine_probe bin/shard_eval run tools/search/transition_affine_probe.shard
  pin_run constraint_superpose_probe bin/shard_eval run tools/search/constraint_superpose_probe.shard
  # The minimal ROUTING-SHAPED consumer: graph coloring as one nonlinear
  # diagonal rule per interference edge over per-variable register holes,
  # constraint-dominated so every pair survives to the partition.  Pins:
  # P4/3 census 24+57 in 4 regions / 3 relational splits; K3/3 census
  # 6+21 in 4/3 (the drive twin of region_probe's CLIQUE-6); K6/6 FIND
  # succeeds past the exact-counting cap (15 ne edges, one component) in
  # 16 regions / 15 splits via count-free inhabitance screening, with the
  # representative verified in-region; K4/3 FIND terminates EMPTY through
  # the walk-refusal ground fallback.
  pin_run coloring_probe bin/shard_eval run tools/search/coloring_probe.shard
  pin_run typed_observer_conjunctive bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_observer_conjunctive.shard audit
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: general typed-rule binder pin ==="
if [ -x bin/shard_eval ]; then
  pin_run typed_rule_probe bin/shard_eval run tools/search/typed_rule_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: general typed imp scope pin ==="
if [ -x bin/shard_eval ]; then
  # The audit proves the narrowing regions agree with all 114 enumerated
  # members while keeping enumeration visibly separate from the search path.
  pin_run typed_imp_add1 bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_imp_add1.shard audit
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: general typed Wasm scope pin ==="
if [ -x bin/shard_eval ]; then
  pin_run typed_wasm_add1 bin/shard_eval run tools/search/typed_expr.shard tools/search/tasks/typed_wasm_add1.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: native Shard call/if scope pin ==="
if [ -x bin/shard_eval ]; then
  pin_run typed_shard_call bin/shard_eval run tools/search/typed_expr.shard tools/search/tasks/typed_shard_call.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Portability/flex pins: the reflected engine searches the current x86 model
# over the old mlx86 calculator battery.  The first task retains the checked-in
# addition-only sample site; the second restores the source's complete
# add/sub/mul/div switch and uses the generic routed task environment.  Both
# are searched by demanded-hole superposition against the complete probe.
echo "=== search: exhaustive-absence pin (typed_expect_empty) ==="
if [ -x bin/shard_eval ]; then
  # No witness exists over the two-atom language; completing the census IS
  # the witness (WITNESS EXPECTED EMPTY, AUDIT … SOLUTIONS 0). Orphaned since
  # aae17c7 (2026-07-16); registered by the 2026-08 prune arc.
  pin_run typed_expect_empty bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_expect_empty.shard audit
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: regular-schema first-result pin (typed_list_first) ==="
if [ -x bin/shard_eval ]; then
  # Bit lists of length <= 2 interned as a lazy regular schema; the first
  # result is the (0 1) witness — SEARCH.md's formation-pressure /
  # occurrence-independence pin. Orphaned since 0539572; registered 2026-08-22.
  pin_run typed_list_first bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_list_first.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: conjunctive observer first-result pin (typed_observer_conjunctive_first) ==="
if [ -x bin/shard_eval ]; then
  # The checked observer theorem couples two ctor positions (nonseparable);
  # first-result search partitions it against lazy occurrences — SEARCH.md's
  # residual-theorem counterpart of typed_list_first. Registered 2026-08-22.
  pin_run typed_observer_conjunctive_first bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_observer_conjunctive_first.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: typed x86 calculator pin ==="
if [ -x bin/shard_eval ]; then
  pin_run typed_x86_calculator bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_x86_calculator.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

echo "=== search: typed x86 four-operation calculator pin ==="
if [ -x bin/shard_eval ]; then
  pin_run typed_x86_calculator4 bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_x86_calculator4.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Checked multi-instruction transition pressure: three observer-spine laws
# remove 264 of 820 XOR programs, including guarded cancellation under a
# nonempty prefix.  The task opts into generic theorem-first region partition;
# audit exhaustively compares the resulting lazy narrowing with enumeration.
echo "=== search: checked x86 transition-window narrowing ==="
if [ -x bin/shard_eval ]; then
  pin_run x86_transition_window bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/x86_transition_window.shard audit
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# The PIO square-wave objective (docs/PIO.md P3): the same engine over the
# PIO model's typed instruction scope.  Expected census: TOTAL 400, FOUND 2
# (the datasheet wave at rank 61 = the witness, plus its set-pindirs gauge
# twin), BEST = WITNESS 61.
echo "=== search: typed pio square-wave pin ==="
if [ -x bin/shard_eval ]; then
  pin_run typed_pio_square bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_pio_square.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# The PIO DME reproduction (docs/PIO.md P4): mlx-pio's locked benchmark over
# the fixed-role dme_spec_ref skeleton with timing/wiring holes.  Expected
# census: TOTAL 4608, FOUND 2 — the jmp-6[0] re-drive gauge twin at BEST 834,
# the transplanted reference at WITNESS 854.  (~2m40s single-thread.)
echo "=== search: typed pio dme pin ==="
if [ -x bin/shard_eval ]; then
  pin_run typed_pio_dme bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/typed_pio_dme.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Checked PIO transition-window narrowing (docs/PIO.md P5b): the mining
# scope searched under four proven spine laws (re-drive merge + the
# drive-absorb trio, nonlinear MovOp metavariable) over the task-local
# straight-line projection.  Expected: SPINE RULES 4; RAW 1111; AUDIT
# ENUMERATIVE AGREEMENT OK ACCEPTED 1067 CONSTRAINED 44.
echo "=== search: checked pio transition-window narrowing ==="
if [ -x bin/shard_eval ]; then
  pin_run pio_transition_window bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/pio_transition_window.shard audit
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Swap-network routing (R5): theorem-first FIND-mode synthesis under two
# proven spine laws (nonlinear self-inverse cancel + disjoint-swap
# ordering) — the live consumer of the find path's theorem-first entry,
# the separated SPLITS counter, AND the skl_iso structural extension
# (cross-depth leaf schema states relate; the first task with live
# relational splits).  Expected: SPINE RULES 2; a green six-swap
# canonical route (KILLED 59 CONSTRAINT 78 FORKS 174 SPLITS 6 STEPS 4542
# at depth 7, measured 2026-07-28).
echo "=== search: swap-network routing synthesis ==="
if [ -x bin/shard_eval ]; then
  pin_run swap_route bin/shard_eval run tools/search/typed_superpose.shard tools/search/tasks/swap_route.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# PCB routing demo: the heuristic-tier driver, scaled to MULTI-PIN
# nets on a parametric 12x12 board — each net's first pin seeds its
# tree, later pins are TAPS routed to the goal set (any tree cell)
# by su_find_query_schema_steps with the self-avoiding bounded-walk
# model, nearest-goal move ordering, parity-aware iterative deepening
# (+1 for mixed-parity trees), give-up ramp 32000*2^level EVALUATOR
# STEPS, deepen-only-on-EMPTY, and the REFUTATION CENSUS convicting
# the rip victim by blocking weight (an occupied pin short-circuits
# to the same conviction).  Heuristic EXISTENCE only (tap-by-tap
# replay certificate re-deriving all cells); never a census pin.
# CONGESTED instance (measured CI pipeline 215, 2026-07-31, ~3.5 min
# job / ~177k steps total): 9 keepouts (2x4 component
# block + corridor plug), 7 nets, rips that COMPOUND.  T's tap is
# sandwiched (component W, net C's col-5 wall E, plug N): four EMPTY
# levels, d=9 census discriminates TWO REAL blockers — C w=3 over the
# S strap w=1 — RIP 29; T re-routes d=5 through the freed corridor;
# then E greedily squats C's freed seed 29, C's retry hits the
# occupied-pin short-circuit, RIP 17 w=1, both relocate (C col 6 len
# 8, E west len 4).  Expected ending:
# PCB-ROUTE 12x12 KEEP-9 NETS-7 RIPS-2 WIRE-29 CERT-DISJOINT-OK.
echo "=== search: PCB routing demo (rip-up/re-route) ==="
if [ -x bin/shard_eval ]; then
  pin_run pcb_route_probe bin/shard_eval run tools/search/pcb_route_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# PCB SPACE-TIME demo (the time-axis rung): occupancy is time-indexed
# (st = t*W*H + c), packets vanish on arrival, the move alphabet gains
# a HOLD.  8x8 full wall with one gap, two nets — SPATIALLY UNROUTABLE
# by construction (both wires would need the gap cell), routed in time:
# A's unique d=5 route fixes the conflict schedule, B's ladder refutes
# d=4/5 on geometry and d=6 on the time conflict (census convicts A's
# TRANSIT space-time cells b@90/b@155), then finds d=7 with EXACTLY
# ONE hold (parity-provable), convoying one cell behind A and crossing
# A's goal cell after it vanishes.  The astar arm re-routes both nets
# in one query each on (cell,tick) dominance keys (pure duplicate
# detection in space-time).  Measured CI pipeline 228, 2026-08-01,
# ~170s job: ladder B 110,616 steps / 80 forks over four levels, astar
# B 89,489 / 15 forks / 17 dominated.  Expected ending, BOTH arms:
# PCB-TIME 8x8 KEEP-7 NETS-2 MAKESPAN-7 TICKS-12 HOLDS-1
# CERT-DISJOINT-OK (astar line prefixed PCB-TIME-ASTAR).
echo "=== search: PCB space-time demo (holds + convoy) ==="
if [ -x bin/shard_eval ]; then
  pin_run pcb_time_probe bin/shard_eval run tools/search/pcb_time_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# PCB COST demo (rung B): weighted moves (cmove=2, chold=5) on the
# space-time model and MIN-COST CLAIMS.  Two-gap wall, objectives
# DISAGREE: B's fastest route holds once behind A through the near gap
# (makespan 7, cost 17), its cheapest takes 8 moves (cost 16, makespan
# 8) — the Pareto pair.  Min-cost bounds are single EMPTYs at C-1
# (cost bounds are monotone): A's refuses at the entry floor with ZERO
# forks; B's runs on BOTH the exact ladder (no dominance, 34 forks)
# and the checked-dominance astar entry (9 forks, 14 dominated — the
# entry's first live EMPTY), cross-checked in-driver.  The cost
# bound's census re-convicts A's transit (b@90/b@155), tick-resolved.
# Measured CI pipeline 231, 2026-08-01, ~236s job.  Expected ending:
# PCB-COST 8x8 KEEP-6 NETS-2 FAST-MAKESPAN-7 FAST-COST-17
# CHEAP-COST-16 CHEAP-MAKESPAN-8 MINCOST-A-10 MINCOST-B-16
# DOM-CHECKED-AGREE CERT-DISJOINT-OK.
echo "=== search: PCB cost demo (weighted moves + min-cost claims) ==="
if [ -x bin/shard_eval ]; then
  pin_run pcb_cost_probe bin/shard_eval run tools/search/pcb_cost_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# PCB CAPACITY demo: negotiated congestion (PathFinder-style) pricing
# the toll seam.  Nets route INDEPENDENTLY (no priority, no hard
# occupancy, no victim selection) against the current toll list;
# space-time conflicts inflate the conflicted CELLS' tolls (+2 = the
# meeting-net count), pure history between rounds.  On rung B's
# two-gap board both nets take near-gap optima and B rides A's
# corridor at the same ticks (SHARE n=4 cells 27 28 29 30); one
# pricing round later A stays (18 vs 24 — the goal toll is
# unavoidable) and B diverts to the untolled far gap (16 < 18):
# converged in 2 rounds, and the no-priority outcome lands exactly on
# the per-priority minima rung B certified (driver asserts 10/16 =
# bound+1; certificate replays with tolls STRIPPED).  Round forks
# 6/0, 7/2-dom, 19/32-dom, 21/24-dom; ~316k engine steps.  Measured
# CI pipeline 234, 2026-08-01, ~192s job.  Expected ending:
# PCB-CAP 8x8 KEEP-6 NETS-2 ITERS-2 TOLLED-4 COST-A-10 COST-B-16
# MAKESPAN-8 CERT-DISJOINT-OK.
echo "=== search: PCB capacity demo (negotiated congestion) ==="
if [ -x bin/shard_eval ]; then
  pin_run pcb_cap_probe bin/shard_eval run tools/search/pcb_cap_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# PCB LNS demo: global-objective destroy-and-repair over insertion
# orders — the first driver where the TOTAL is the acceptance
# criterion.  Zero-keepout cascade instance (A vertical crosses B at
# 28@t3 and C at 36@t4, consecutive, so ONE +2 delay by A clears
# both): A-before-B orders total 40 (two yields), B-before-A orders
# 36 (one yield) = the paper joint optimum.  Greedy strict descent
# from (A,B,C): one accepted swap to (B,A,C), re-swap and tie both
# rejected, LOCAL-OPT.  All 12 inner A* queries pop-predicted EXACTLY
# (forks/dominated); re-swap counters byte-identical to INIT (the
# determinism self-check).  Driver asserts INIT=40/BEST=36; n-net
# cross-replay certificate.  Measured CI pipeline 239, 2026-08-01,
# ~371s job, ~599k engine steps.  Expected ending:
# PCB-LNS 8x8 KEEP-0 NETS-3 INIT-40 BEST-36 ACCEPTED-1 REJECTED-2
# LOCAL-OPT CERT-DISJOINT-OK.
echo "=== search: PCB LNS demo (global-objective order descent) ==="
if [ -x bin/shard_eval ]; then
  pin_run pcb_lns_probe bin/shard_eval run tools/search/pcb_lns_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Pure Shard function-body pins. These retain the playground's full grammars
# and exact solution floors while SUPERPOSE settles them by demanded holes.
# The executable imports kernel/types for an independent representative gate,
# so (like typed_superpose above) it is not a checker target until the known
# tc_infer/tc_arms mutual-recursion measure gap is closed.
echo "=== search: pure Shard function-body benchmarks ==="
if [ -x bin/shard_eval ]; then
  pin_run pure_bench bin/shard_eval run tools/search/pure_bench.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Checked theorem formation over a supplied pure-Shard grammar.  The bounded
# depth-3 run pins the 10^17-member quotient and an exact partial census without
# making the much longer complete interpreter run a routine corpus cost.
echo "=== search: pure Shard checked-formation depth-3 probe ==="
if [ -x bin/shard_eval ]; then
  pin_run pure_deep bin/shard_eval run tools/search/pure_deep.shard probe 3 5000
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Match-arm definitional facts and a checked polymorphic order involution are
# compiled generically into ordinary exact Grammars.  The first pin audits
# match context against the old hand-written rev dialect; the next two compose
# those mechanisms with sort and pin both exact quotients cheaply.
echo "=== search: generic match-context formation ==="
if [ -x bin/shard_eval ]; then
  pin_run context_formation_probe bin/shard_eval run tools/search/context_formation_probe.shard
  pin_run pure_deep bin/shard_eval run tools/search/pure_deep.shard context-probe 2 1
  pin_run pure_deep bin/shard_eval run tools/search/pure_deep.shard order-probe 2 1
  pin_run pure_deep bin/shard_eval run tools/search/pure_deep.shard nonlinear-probe 2 1
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Canonical rev is the scale pin for quotient-first formation composed with
# demanded-hole superposition.  Both runs start from the full grammar and
# derive the append quotient from four checked requirements.  The d4 fork
# count (639) exactly matches the playground across both the 2.25-trillion
# append quotient and the 1.14-trillion contextual quotient; this gate guards
# the categorical decision structure, not host-language throughput.
echo "=== search: checked append-canonical rev d3-d4 ==="
if [ -x bin/shard_eval ]; then
  pin_run rev_deep bin/shard_eval run tools/search/rev_deep.shard 3
  pin_run rev_deep bin/shard_eval run tools/search/rev_deep.shard 4
  pin_run rev_deep bin/shard_eval run tools/search/rev_deep.shard context 4
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Ground-search pin (docs/SEARCH.md slice 1): the rev accumulator space,
# rank-addressed by meta/sketch and settled through the real machine
# (meta/invoke -> evm_call_pure). Counts and solution sets must match the
# playground's published measurement record EXACTLY: 108 candidates / 1
# solution at depth 1, 7788 / 13 at depth 2. Every COUNT / SOL / SOLUTIONS
# line is diffed; a grammar or addressing change moves them and fails the
# corpus diff (re-pin deliberately, with the change).
echo "=== search: ground rev pin ==="
if [ -x bin/shard_eval ]; then
  pin_run search bin/shard_eval run tools/search/search.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Canonicality census pin (docs/SEARCH.md S9, G1+G2): the dialect rev
# grammar's candidate set must equal the cn_e-clean subset of the full
# grammar EXACTLY, censused term-by-term through rank/unrank round-trips
# (FULL 108 DIALECT 56 CLEAN 56 at d1; 7788/1736/1736 at d2; the 13 full
# solutions collapse to exactly 1 dialect solution). This is the
# three-speakers drift alarm: a kernel C-rule change or a generator edit
# moves these lines and fails the diff — re-pin deliberately, with the
# change.
echo "=== search: canonicality census (G1+G2) ==="
if [ -x bin/shard_eval ]; then
  pin_run census bin/shard_eval run tools/search/census.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Catalog census pin (docs/SEARCH.md S7-lite, G5): the structural list
# fragment enumerated at rungs 1-2, post-filtered through cn_e, and
# battery-bucketed by behavior. GEN/CLEAN/BEHAVIORS, SAMPLE-GAUGE
# (excess spellings / collided buckets / collided members / max bucket),
# flagged content families, and rev/id spelling counts are pinned against the
# REAL dialect. Rung 1: 20/17/13, gauge 4/3/7/3 — the playground's certified
# "exactly 13". Rung 2: 3395/2345/1068, gauge 1277/596/1873/18 — behaviors
# match the playground's 1068, rev = 2 spellings. A canon-rule, grammar, or
# sample-pattern change moves these lines and fails the diff; re-pin
# deliberately with the change.
echo "=== search: catalog census (G5) ==="
if [ -x bin/shard_eval ]; then
  pin_run catalog bin/shard_eval run tools/search/catalog.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Playground-transfer pin: ablate the catalog's mined R1 generation policy,
# then compare the production, C8-normal, and fully canonical behavior-key
# sets exactly. Rung 2: 9435/3395/2356/2345 forms; every terminating layer
# retains the same 1068 sampled behaviors. In particular 2356/1068 reproduces
# the playground's post-R1 endpoint, including rev/id spellings 2/6, against
# the newer kernel ledger (whose full-canon rev/id endpoint is 2/4).
echo "=== search: catalog pressure (playground R1 transfer) ==="
if [ -x bin/shard_eval ]; then
  pin_run catalog_pressure bin/shard_eval run tools/search/catalog_pressure.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Laws-oracle pin (docs/SEARCH.md S4a+S5, G3): the symbolic evaluator +
# requirements-as-oracle. The first line pins that the append four came from
# rev_obj's CHECKED item scope as a generic profile (no NRAppend fallback).
# SELF: std/list's own rev and len must
# symbolically PROVE their own interface laws (reduction + congruence +
# the append canon). G3: over the catalog's cn_e-clean candidates,
# law verdicts against the ground battery — rung 1: 17 clean, 0 proven,
# 17 refuted, 0 undecided; rung 2: 2345 clean, 2 proven (exactly the
# two rev spellings), 2343 refuted, 0 undecided. Any Proven non-passer
# or Refuted passer exits 1 inside the tool (G3 violations are hard
# failures, not statistics). laws.shard rides kernel/driver for goal
# parsing, so like tools/prove it is pinned by RUN output, not checked
# as a corpus target (the known kernel/types tc_infer measure gap).
# TRACE lines (slice 5 component 2) pin the proof SKELETONS a Proven
# verdict leaves behind: std rev/len and rev_c62 join by REFL
# (compute + refl at render time); rev_c347 needs exactly the case its
# own body introduced — (SPLIT 0 (Nil REFL) (Cons REFL)).
# SYNTH REGEN (component 3) re-renders the WHOLE artifact — fns AND
# claims with proofs rendered from the traces (leaf lemma tails
# decided by the check-mode replay twin) — and requires byte-identity
# with the committed gen/rev_synth.shard; `laws emit` re-pins. G4 is
# continuous: the artifact is a check TARGET, so its rendered proofs
# replay through bin/shard_check in the sweep above.
# BRACKET REGEN (component 4 — the arc's EXIT CRITERION) does the same
# for gen/cat_bracket.shard: the CERTIFIED rung-1 bracket — all 17
# clean candidates as fns over the local bx_append twin (bridged to
# std/list append by one induct claim), the kernel-computed FLOOR
# (13 representatives' vectors pairwise distinct) and the four CEILING
# equivalence claims (three via the D5 catalog license, rendered as
# induct with (hyp ih) citations). 17 clean = EXACTLY 13 functions,
# kernel-checked on every sweep; `laws bracket` re-pins.
echo "=== search: laws oracle (S4a+S5, G3) ==="
if [ -x bin/shard_eval ]; then
  pin_run laws bin/shard_eval run tools/search/laws.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Behavior-collision proof census (the pre-mining queue): every clean
# catalog member is compared with its bucket's minimum-rank representative.
# Rung 1 closes all 4 observational edges.  Rung 2 proposes 1,277 edges;
# structural induction + append normalization proves 1,242, refutes none,
# and leaves 35 explicit auxiliary-lemma candidates; retained symbolic
# residuals classify all 35 as append-spine permutations (other/missing 0).
# Exact alpha-stable grouping then collapses them to five residual signatures
# with support 7 each; relative-order analysis finds one smaller commutation
# basis with support 35. `range` materializes that basis as a typed law for a
# representative and pins the honest next boundary: still Undecided with one
# permutation basis (derived-relation induction is not yet licensed).
# This also pins partially-decided strict-subterm provenance: losing it drops
# hundreds of these proofs while leaving ordinary G3 tests green.
echo "=== search: behavior-collision proof census ==="
if [ -x bin/shard_eval ]; then
  pin_run laws bin/shard_eval run tools/search/laws.shard mine 1
  pin_run laws bin/shard_eval run tools/search/laws.shard mine 2
  pin_run laws bin/shard_eval run tools/search/laws.shard range 2 295
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Render RELOAD pin (docs/SEARCH.md D11, slice 5): the committed
# artifact, loaded through the real reader/resolver, yields bodies
# expr_eq to the unranked candidates (rank 62/347, re-verified by
# cn_e+battery). The REGEN half lives in the laws suite above (SYNTH
# REGEN — the full artifact including rendered proofs).
echo "=== search: render round-trip (D11) ==="
if [ -x bin/shard_eval ]; then
  pin_run render_gate bin/shard_eval run tools/search/render_gate.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Superposed-executor pin (docs/SEARCH.md S4b, built AS RATIFIED —
# named superpose; "narrow" is the bootstrap dialect's name): the
# choices-map machine settles the rev spaces EXACTLY — d1: 108
# candidates in 26 regions (8 forks), 1 found; d2: 7,788 in 443
# regions (133 forks), 13 found. The rev suite rides the GENERIC
# query drive (slice-0 retirement of the legacy per-test battery;
# settlement identical, STEPS re-pinned 623/12,651 -> 489/7,777 —
# one interned battery query per drive replaces per-test input
# re-thunking). STEPS still pins the consulted-choice-set memo's
# leverage. AGREE extends G3 three ways: found coverage == the
# enumerative solution count, every enumerative solution lies in a
# found region, and every region representative passes the
# kernel/evm battery. Any drift exits 1 inside the tool.
echo "=== search: superposed executor (S4b) ==="
if [ -x bin/shard_eval ]; then
  pin_run superpose bin/shard_eval run tools/search/superpose.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Arena memo-scoping pin (shard#19 ask 4, slice 0c): the release lever's
# contract on the substrate directly — a hit replays the SAME node id at
# zero steps; sa_forget_holes spares rows over untouched holes and forces
# fresh-id re-evaluation (same ground value, row re-recorded) for ripped
# ones; sa_forget_mm empties the table while region-independent
# indirections keep replaying for free. The memo is agreement-keyed, so
# every forget is verdict-neutral by construction; this pins it.
echo "=== search: arena memo-scoping (forget) ==="
if [ -x bin/shard_eval ]; then
  pin_run arena_probe bin/shard_eval run tools/search/arena_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# False-equivalence-proof hunter pin (docs/SEARCH.md standing-use #2):
# the ground battery + the S4a comparator pointed at the std tree's
# OWN claimed theorems — 13 interface files' requirements + 14
# in-closure impl files' claims, typed ground enumeration with
# premise filtering, evaluated by the kernel's own reducer in the
# open run closure. Pinned at cut: 291 laws — 262 PASS, 0 REFUTED,
# 0 SYMREFUTED, 0 VACUOUS, 22 STUCK (the 12 word shift laws stick
# exactly on negative shift amounts — partial-domain prims, 10/10
# reducible vectors pass; the sha256 class is fuel-bounded, its
# ground pins already replay as compute claims), 7 SKIP (refined Str,
# over-cap batteries, the 8-field H8). Sym cross-check: 117 proven,
# 4 SYMERR (the S4a comparator's ctor-vs-atom refusal on bytes/mem
# length laws — a recorded S4a question). A ground counterexample
# against a symbolic proof exits 1 inside the tool (G3); REFUTED
# lines are FINDINGS (tool exits 0) — any new one changes this
# output and shows in the corpus diff: investigate before re-pinning.
echo "=== search: false-equivalence hunter ==="
if [ -x bin/shard_eval ]; then
  pin_run hunt bin/shard_eval run tools/search/hunt.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Canon-subsumption census pin (docs/SEARCH.md standing-use #1): rule
# subsumption as absence proofs by exhaustion — every candidate of the
# rev full space (d1/d2) and the catalog rungs (1/2) judged by cn_e,
# flag sets deduplicated and tallied. Pinned at cut: CLEAN counts
# match the census/catalog pins exactly (56/1736/17/2345); every rule
# that fires has UNIQUE witnesses at every rung (no LOCALLY REDUNDANT
# line, no PAIR ... COVERS line) — the kernel ledger carries no
# internal redundancy on these fragments; slice 3's C8⊃R1 was
# kernel-over-playground, not intra-kernel. The instrument re-measures
# on every sweep: a future rule whose UNIQUE hits 0 across fragments
# (or a COVERS pair) changes these lines and shows in the corpus diff
# — evidence for the canon arc, which owns the ledger.
echo "=== search: canon-subsumption census ==="
if [ -x bin/shard_eval ]; then
  pin_run subsume bin/shard_eval run tools/search/subsume.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

else
  echo "=== long-tier engine pins: SKIPPED (CORPUS_LONG=0) ==="
fi
# ---- end long-tier engine pins ----------------------------------------------

# Run-mode qualified-dispatch pin (the enc_instr bug): two co-loaded modules
# define the same-named internal helper; each pick must call its OWN. Also
# pins the run loop's World-identity threading (the app defines its own
# World and destructures the token write hands back — a mis-tagged token
# sticks loudly, exit 4). Self-checking: OK lines, exit 0 iff both hold.
echo "=== run-mode: qualified dispatch (runhom) ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run pins/lang/runhom_run.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Engine-differential pin: encode the composite + const probe to real .wasm
# and replay the model-computed vectors under node/V8 (the slice-4 reality
# check — the "engine conforms to model" trust leaf). FULL output flows
# through: the replayer prints per-mismatch `FAIL <line> [detail]` at
# column 0, which is what CI's ^(FAIL|TYPE!) projection gates on (the old
# `| tail -1` kept only the always-printed summary line, so disagreements
# were INVISIBLE to the gate — fixed 2026-07-29, all three differential
# legs). The `||` arm turns a nonzero exit (crash or disagreement count)
# into a synthetic FAIL row, so a differ that DIES is as loud as one that
# differs.
echo "=== wasm: engine differential ==="
if command -v node >/dev/null && [ -x bin/shard_eval ]; then
  bash models/wasm/diff/wasm_diff.sh 2>&1 || echo "FAIL wasm-differential (exit $?)"
else
  echo "SKIPPED (needs node + bin/shard_eval)"
fi

# Silicon differential (the x86_64 arc's Probe B): flatten each XFunc to real
# machine code, mmap it executable, and CALL it on this CPU — comparing the
# hardware result + memory (and the trap leg: model None == hardware fault)
# against the model. The "CPU conforms to the model" trust leaf. Full output;
# per-mismatch FAIL lines gate via the CI projection, nonzero exit adds a
# synthetic FAIL row (see the wasm leg's note).
echo "=== imp: the runtime (run-mode probe, COVERAGE.md C2a) ==="
pin_run imp_rt_run bin/shard_eval run models/imp/probes/rt_run.shard
pin_run imp_rth_run bin/shard_eval run models/imp/probes/rth_run.shard

# impc (COVERAGE.md C3a): regenerate the micro-flagship's product and byte-compare
# it against the committed file (the regen contract), then the run-mode
# differential of every wrapper (spec vs compiled-through-iprun)
echo "=== impc: the micro-flagship (regen tie + differential, COVERAGE.md C3a) ==="
pin_run impc_micro_regen bash -c 'bin/shard_eval run tools/impc/impc.shard tools/impc/fixtures/micro.shard tools/impc/fixtures/micro_ipc_out.raw && bin/shard_eval run tools/shardfmt/shardfmt.shard tools/impc/fixtures/micro_ipc_out.raw | cmp - tools/impc/fixtures/micro_ipc_out.shard'
pin_run impc_micro_run bin/shard_eval run tools/impc/fixtures/micro_run.shard
# the frame tier (C1c): the same wrappers through the x86 MODEL on ixf_prog's translation
pin_run impc_micro_x86_run bin/shard_eval run tools/impc/fixtures/micro_x86_run.shard
# the silicon leg (C1c-2): one static ELF per wrapper through enc_winelf, exit status vs the spec mod 256
pin_run impc_micro_silicon tools/impc/fixtures/micro_silicon.sh

echo "=== x86: silicon differential ==="
if command -v cc >/dev/null && [ -x bin/shard_eval ]; then
  bash models/x86/diff/x86_diff.sh 2>&1 || echo "FAIL x86-differential (exit $?)"
else
  echo "SKIPPED (needs cc + bin/shard_eval)"
fi

# SHA-NI BLOCK-GRAIN silicon differential (STREAM.md §8.4, B5/E2c): the same
# trust leaf as the leg above, one rung up — the sha256.shani article's HAND
# BODY (shn_body, 207 straight-line XVec instructions) emitted as real machine
# code and executed, with the stored state compared against the article's
# VALUE tier (shn_block_b, itself ground-pinned to the spec at E2a) and the
# whole XMM file against the vector-tier walk. Carries its own un-blinding:
# five perturbed twins that must NOT reproduce the reference (XVNCASE rows,
# tallied per tooth). Rows SKIP loudly on a CPU without sha_ni — the CI runner
# is such a box, so this leg is adjudicated on SHA-capable silicon (X86.md §6's
# capability posture). Per-mismatch FAIL lines gate via the CI projection;
# nonzero exit adds a synthetic FAIL row (see the wasm leg's note).
echo "=== x86: sha-ni block differential ==="
if command -v cc >/dev/null && [ -x bin/shard_eval ]; then
  bash models/x86/diff/shani_diff.sh 2>&1 || echo "FAIL shani-block-differential (exit $?)"
else
  echo "SKIPPED (needs cc + bin/shard_eval)"
fi

# RISC-V engine differential (the RISC-V arc's G2): encode each RvFunc to real
# RV32I/RV64I bytes, map them executable, and CALL them under qemu-user at BOTH
# widths — comparing the emulated core's result + memory (and the trap leg:
# model None == core fault) against the model. The "core conforms to the
# model" trust leaf; qemu-user plays V8's role (there is no native silicon leg
# on this box). Full output; per-mismatch FAIL lines gate via the CI
# projection, nonzero exit adds a synthetic FAIL row (see the wasm leg's note).
# riscv_diff.sh self-guards (SKIP exit 0) when clang/qemu-user/rust-lld absent.
echo "=== riscv: engine differential ==="
if command -v clang >/dev/null && command -v qemu-riscv64 >/dev/null && [ -x bin/shard_eval ]; then
  bash models/riscv/diff/riscv_diff.sh 2>&1 || echo "FAIL riscv-differential (exit $?)"
else
  echo "SKIPPED (needs clang + qemu-user + bin/shard_eval)"
fi

# sha256sum SCALAR + ONE-SHOT bin-level silicon differential (STREAM.md §7.9 M5
# slice 4): emit BOTH of those proven sha256sum ELFs fresh via the run-only
# write glue, assert caplessness (getcap empty, RW window at 0x10000 = stock
# mmap_min_addr, no vaddr-0 PT_LOAD), then drive the emitted binaries against
# coreutils — the streaming bin PIPE-FED (short reads are the thing under test;
# the §51 file-redirect discipline is retired), the one-shot by file redirect
# with its 64887/64888 cap boundary pinned on both sides. The "hardware conforms
# to the model" trust leaf at whole-program grain, where the x86 leg above is
# the same leaf per XFunc. Per-row OK/FAIL at column 0; nonzero exit adds a
# synthetic FAIL row (see the wasm leg's note). The script self-fails (no
# SKIP) on missing coreutils/getcap/readelf — its assertions must never
# silently not happen.
# ⚠ RENAMED 2026-08-10 (B6 slice D): this leg was `sha256sum-silicon` and its
# one-shot product sat at the unqualified path examples/sha256sum/sha256sum.
# Under ruling R2 both belong to the dispatch bin (third block below), so the
# leg is `sha256sum-scalar-oneshot-silicon` and the product is
# examples/sha256sum/sha256sum_oneshot.
echo "=== sha256sum scalar+one-shot: capless silicon differential ==="
if [ -x bin/shard_eval ]; then
  bash examples/sha256sum/sha256sum_silicon_diff.sh 2>&1 \
    || echo "FAIL sha256sum-scalar-oneshot-silicon (exit $?)"
else
  echo "SKIPPED (needs bin/shard_eval)"
fi

# sha256sum SHA-NI variant bin-level silicon differential (STREAM.md §8.4 E3
# slice D): the same leaf as the leg above, for the variant that computes the
# digest through the CPU's sha256rnds2/sha256msg1/sha256msg2 units. Emits the
# proven SHA-NI ELF fresh via its run-only write glue, asserts caplessness, and
# drives it PIPE-FED against a DOUBLE ORACLE — coreutils sha256sum AND openssl
# dgst — with oracle-vs-oracle disagreement given its own FAIL text. Running the
# product needs the sha_ni CPU flag, so the script CPUID-GATES ITSELF and prints
# one loud SKIP line (exit 0) on a runner without it; the CI runner is such a
# box, so the skip line is the EXPECTED output there and this leg is adjudicated
# on SHA-capable silicon. That gate is its only skip: missing
# coreutils/openssl/getcap/readelf is still a FAIL. Per-row OK/FAIL at column 0;
# nonzero exit adds a synthetic FAIL row (see the wasm leg's note).
echo "=== sha256sum shani: capless silicon differential ==="
if [ -x bin/shard_eval ]; then
  bash examples/sha256sum/sha256sum_shani_diff.sh 2>&1 || echo "FAIL sha256sum-shani-silicon (exit $?)"
else
  echo "SKIPPED (needs bin/shard_eval)"
fi

# sha256sum CPUID-DISPATCH bin-level silicon differential (STREAM.md §9.2, rung
# B6 slice D): the same leaf again, for the R2 plain-name artifact — the ONE
# binary that carries both proven variants (scalar fns 0-14, SHA-NI fns 15-29)
# and picks between them with its own two-step CPUID stub at fn 30. Emits the
# proven merged ELF fresh via its run-only write glue, asserts caplessness,
# asserts by DISASSEMBLY that both families are in the image on every box
# (sha256rnds2 x32 / sha256msg1 x12 / sha256msg2 x12 / cpuid x2, count-exact),
# then drives it PIPE-FED against the DOUBLE ORACLE — coreutils sha256sum AND
# openssl dgst — with oracle-vs-oracle disagreement given its own FAIL text.
# ⚠ THIS LEG RUNS EVERYWHERE AND HAS NO SKIP, unlike the SHA-NI leg above: the
# product adjudicates on any chip because the stub calls the half that chip can
# execute (a runner without sha_ni exercises the scalar arm = the proofs'
# dsm/dsx `_lo`-and-bit-clear case; SHA-capable silicon exercises the SHA-NI
# arm), and the digests are double-oracled either way. The cpuid probe inside
# the script only REPORTS which arm this box took. Missing
# coreutils/openssl/getcap/readelf/objdump is a FAIL, as everywhere else.
# Per-row OK/FAIL at column 0; nonzero exit adds a synthetic FAIL row (see the
# wasm leg's note).
echo "=== sha256sum dispatch: capless silicon differential (runs everywhere) ==="
if [ -x bin/shard_eval ]; then
  bash examples/sha256sum/sha256sum_dispatch_diff.sh 2>&1 \
    || echo "FAIL sha256sum-dispatch-silicon (exit $?)"
else
  echo "SKIPPED (needs bin/shard_eval)"
fi

echo "=== guard: absolute path ==="
out=$("${CHECK_CMD[@]}" "$PWD/pins/proof/auto_demo.shard" 2>&1); code=$?
if [ "$code" -eq 2 ] && grep -q "escapes the repo root" <<<"$out"; then
  echo "REFUSED (exit 2)"
else
  echo "GUARD FAILED: exit $code"
  printf '%s\n' "$out" | head -3
fi

# Lowering-build pins (ratified 2026-07-04, docs/LOWERING.md): the
# gated artifact builds — REGEN (producer determinism) / SCHEMA (lowcheck)
# / KERNEL / BYTETIE (cert↔binary) / ENGINE (V8; the x86 build's engine
# is the CPU itself) — run end to end. Summary
# line only; any gate failure changes it and fails the corpus diff.
for LB in "tools/build/build.sh tools/build/build_products.shard"; do
  echo "=== lowering: $LB ==="
  if [ -n "${SKIP_LOWERING:-}" ]; then
    echo "SKIPPED (SKIP_LOWERING set -- the driver ran separately this cycle)"
  elif [ -x bin/shard_eval ]; then
    if bash $LB > "$TMP/lb.out" 2>&1; then
      tail -1 "$TMP/lb.out"
    else
      echo "BUILD FAILED"
      tail -20 "$TMP/lb.out"
    fi
  else
    echo "SKIPPED (no bin/shard_eval)"
  fi
done

# Schema-recognizer negative pin: lowcheck_rejects.shard is kernel-TRUE yet
# schema-REFUSED (truth ≠ composability) — the recognizer must reject it.
echo "=== lowering: lowcheck negative fixture ==="
if [ -x bin/shard_eval ]; then
  if bin/shard_eval run tools/lowcheck/lowcheck.shard tools/lowcheck/fixtures/lowcheck_rejects.shard > "$TMP/lc.out" 2>&1; then
    echo "GATE FAILED: nonconforming fixture ACCEPTED"
    tail -5 "$TMP/lc.out"
  else
    tail -1 "$TMP/lc.out"
  fi
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Manifest-gate negative pin: ARTIFACT lines that MISBIND (wrong export
# index / cert name not lowered_<name>-shaped / a certfile the build never
# gated) must be REFUSED — before this gate the manifest's cert/export
# fields were checked by nothing (docs/LOWERING.md §6ad).
echo "=== lowering: manifest negative fixture ==="
if [ -x bin/shard_eval ]; then
  if bin/shard_eval run tools/lowcheck/manifest.shard tools/lowcheck/fixtures/manifest_rejects.txt models/wasm/wasm.shard tools/lowbuild/fixtures/wasmgen_call_link.shard > "$TMP/mf.out" 2>&1; then
    echo "GATE FAILED: misbound manifest ACCEPTED"
    tail -5 "$TMP/mf.out"
  else
    tail -1 "$TMP/mf.out"
  fi
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Percolation-twin negative pin (docs/X86.md §49): a smuggled XSyscall in a
# module with an empty declared effect surface must be REFUSED by bytetie
# — before this gate a hidden effect-point in a "pure" body was checked by
# nothing at the byte level.
echo "=== lowering: percolation negative fixture ==="
if [ -x bin/shard_eval ]; then
  if bin/shard_eval run tools/bytetie/bytetie.shard pins/trust/percolation_rejects.shard > "$TMP/pc.out" 2>&1; then
    echo "GATE FAILED: hidden effect-point ACCEPTED"
    tail -5 "$TMP/pc.out"
  else
    tail -1 "$TMP/pc.out"
  fi
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Pattern binder-capture negative pin (friction item 1 residue): a bare
# CAPITALIZED symbol in pattern position naming no visible ctor must be
# REFUSED, not silently bound (the ctor-capture footgun). Runs the SOURCE
# checker through the tower — the compiled engine accepts this file until
# its next rebuild, the source must refuse it TODAY.
echo "=== reader: pattern binder-capture negative fixture ==="
if [ -x bin/shard_eval ]; then
  if bin/shard_eval run kernel/check.shard pins/lang/pat_binder_rejects.shard > "$TMP/pb.out" 2>&1; then
    echo "GATE FAILED: capitalized binder-capture pattern ACCEPTED"
    tail -5 "$TMP/pb.out"
  else
    tail -2 "$TMP/pb.out"
  fi
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Canon stage-1 advisory pins (CANON.md §5/§9): canon_pin must produce ZERO
# CANON lines (canonical bodies + the goal-position exemption); canon_rejects
# must still EXIT 0 (stage 1 is advisory, never a failure) while reporting
# every invariant C1-C6. Runs the SOURCE checker through the tower so the
# pins bind TODAY, before the next engine rebuild. Failures emit FAIL lines
# so the corpus FAIL-set diff catches them.
echo "=== canon: stage-1 advisory pins ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run kernel/check.shard pins/lang/canon_pin.shard > "$TMP/cnp.out" 2>&1
  cnp_rc=$?
  if [ $cnp_rc -ne 0 ]; then
    echo "FAIL canon_pin (exit $cnp_rc)"
  elif grep -q '^CANON ' "$TMP/cnp.out"; then
    echo "FAIL canon_pin (canonical bodies flagged)"
    grep '^CANON ' "$TMP/cnp.out"
  else
    echo "CANON PIN OK"
  fi
  bin/shard_eval run kernel/check.shard pins/lang/canon_rejects.shard > "$TMP/cnr.out" 2>&1
  cnr_rc=$?
  if [ $cnr_rc -ne 0 ]; then
    echo "FAIL canon_rejects (advisory changed the exit code: $cnr_rc)"
  else
    cnr_missing=""
    for code in "C1 +" "C2 if" "C2 match" "C3 dead" "C3 order" "C3 merge" "C4 CRB" "C4 lit" "C5 unreachable" "C6 S" "C6 Z" "C7 nil_left" "C7 cons" "C7 assoc" "C7 nil_right" "C8 respell" "C8 rebuild" "C9 bool" "C10 vacif" "C10 match" "C11 resplit"; do
      grep -q "^CANON cr_.*: $code" "$TMP/cnr.out" || cnr_missing="$cnr_missing [$code]"
    done
    if [ -n "$cnr_missing" ]; then
      echo "FAIL canon_rejects (missing:$cnr_missing)"
    else
      echo "CANON REJECTS OK ($(grep -c '^CANON ' "$TMP/cnr.out") lines)"
    fi
  fi
  # The REWRITER roundtrip pin (CANON.md slices 2/3): tools/canon on the
  # pin file is the IDENTITY (already canonical + fmt-canonical), and on
  # the rejects file it machine-fixes everything except the refusal tier —
  # C3 (let hygiene, not in v1) and C7 (theory redexes: recognizer-only in
  # v1; the tool never applies theory rules) — so the rewritten output
  # re-checks with EXACTLY those 7 advisory lines and no others. (C6 left
  # the refusal tier with the tc_nat_lit_view return-position fix; C11
  # joined the FIXABLE tier in §13 slice 3 — the tool fires shape pins.)
  bin/shard_eval run tools/canon/canon.shard pins/lang/canon_pin.shard > "$TMP/cnt_pin.out" 2>/dev/null
  if cmp -s "$TMP/cnt_pin.out" pins/lang/canon_pin.shard; then
    echo "CANON TOOL PIN-IDENTITY OK"
  else
    echo "FAIL canon_tool (pin file not a fixed point of the rewriter)"
  fi
  # the rewritten file must sit BESIDE the pin (pins/lang/) so its relative imports resolve
  bin/shard_eval run tools/canon/canon.shard pins/lang/canon_rejects.shard > pins/lang/.cnt_rej_tmp.shard 2>/dev/null
  cnt_rc=$?
  if [ $cnt_rc -ne 0 ]; then
    echo "FAIL canon_tool (rejects rewrite exited $cnt_rc)"
    rm -f pins/lang/.cnt_rej_tmp.shard
  else
    bin/shard_eval run kernel/check.shard pins/lang/.cnt_rej_tmp.shard > "$TMP/cnt_rej.out" 2>&1
    rm -f pins/lang/.cnt_rej_tmp.shard
    cnt_lines=$(grep -c '^CANON ' "$TMP/cnt_rej.out")
    cnt_bad=$(grep '^CANON ' "$TMP/cnt_rej.out" | grep -vc 'C3 \|C7 ')
    if [ "$cnt_lines" = "7" ] && [ "$cnt_bad" = "0" ]; then
      echo "CANON TOOL ROUNDTRIP OK (7 refusal lines)"
    else
      echo "FAIL canon_tool (roundtrip: $cnt_lines lines, $cnt_bad outside the refusal tier)"
      grep '^CANON ' "$TMP/cnt_rej.out"
    fi
  fi
  # The EXACTNESS CENSUS (CANON.md §9 / slice 2b): recognizer image must
  # equal the rewriter's fixpoint image over the enumerated tier.
  if bin/shard_eval run tools/canon/census.shard > "$TMP/census.out" 2>&1; then
    tail -1 "$TMP/census.out"
  else
    echo "FAIL canon_census"
    cat "$TMP/census.out"
  fi
  # STAGE-2 PIN (CANON.md §3 ratchet / slice 4): the std/ tree is CANONICAL —
  # every std source (impl files, wasm/x86/rep siblings, mod.build plans,
  # mod.req interfaces; .auto proof sidecars and derived .low files excluded)
  # produces ZERO CANON advisory lines. New std code that regresses the
  # canonical form fails the corpus here.
  #
  # The sweep needs only the CANON advisories, and the checker emits those
  # at LOAD: files whose proofs the corpus already checked as targets run
  # in focus mode on a claim name that never exists (full load + canon
  # pass, zero proof re-checking; measured 2026-07-16: the serial full
  # sweep was 1368s of a 2343s corpus, wf.shard alone 119s full vs 0.4s
  # load-only). Files OUTSIDE the target list keep the full check — the
  # sweep is their only proof coverage. Parallel with buffered output
  # (the target-phase idiom); green state is empty either way.
  std_sweep=()
  while IFS= read -r f; do std_sweep+=("$f"); done < <(ls std/*/*.shard std/*/mod.req/*.shard 2>/dev/null | grep -v '\.auto\.shard$\|\.low\.shard$')
  for i in "${!std_sweep[@]}"; do
    while (( $(jobs -rp | wc -l) >= JOBS )); do wait -n; done
    {
      f="${std_sweep[$i]}"
      case " ${TARGETS[*]} " in
        *" $f "*) "${CHECK_CMD[@]}" "$f" __canon_sweep_load_only__ 2>/dev/null ;;
        *) "${CHECK_CMD[@]}" "$f" 2>/dev/null ;;
      esac | grep '^CANON ' | sed "s|^|$f |"
    } > "$TMP/canon.$i" &
  done
  wait
  std_canon=$(cat "$TMP"/canon.* 2>/dev/null)
  if [ -z "$std_canon" ]; then
    echo "CANON STD STAGE-2 OK (tree at zero)"
  else
    echo "FAIL canon_std_stage2 (non-canonical std source):"
    echo "$std_canon"
  fi
  # The CONTENT-ADDRESS pin (CANON.md §7 / slice 5): digest-stable
  # properties only — alpha-twins hash equal, Merkle callers of equal
  # referents hash equal, distinct definitions hash apart, deterministic —
  # so the digest swap (FNV-1a-128 -> std/sha256) touches no goldens.
  bin/shard_eval run tools/canon/hash.shard pins/proof/hash_pin.shard > "$TMP/hx1.out" 2>&1
  bin/shard_eval run tools/canon/hash.shard pins/proof/hash_pin.shard > "$TMP/hx2.out" 2>&1
  hx_f() { grep "hash_pin.$1\$" "$TMP/hx1.out" | cut -d' ' -f1; }
  if ! cmp -s "$TMP/hx1.out" "$TMP/hx2.out"; then
    echo "FAIL canon_hash (nondeterministic)"
  elif [ -z "$(hx_f hp_twin_a)" ]; then
    echo "FAIL canon_hash (no output)"
    cat "$TMP/hx1.out"
  elif [ "$(hx_f hp_twin_a)" != "$(hx_f hp_twin_b)" ]; then
    echo "FAIL canon_hash (alpha twins differ)"
  elif [ "$(hx_f hp_calls_a)" != "$(hx_f hp_calls_b)" ]; then
    echo "FAIL canon_hash (Merkle callers differ)"
  elif [ "$(hx_f hp_twin_a)" = "$(hx_f hp_other)" ] || [ "$(hx_f hp_calls_a)" = "$(hx_f hp_calls_other)" ]; then
    echo "FAIL canon_hash (distinct definitions collide)"
  else
    echo "CANON HASH PIN OK (alpha + merkle + distinctness, deterministic)"
  fi
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Rust-conformance pin: the bootstrap evaluator's cargo suite (prim
# conformance vs the object table, Word/Bytes revocation guards, kernel
# loading, induct/match plumbing). It was manual-only and rotted silently
# for weeks (repaired 2026-07-03); this pin keeps it in every corpus run.
# Summary line only — a failure changes it and fails the corpus diff.
echo "=== rust_bootstrap: cargo test ==="
if command -v cargo >/dev/null; then
  if cargo test --release --manifest-path rust_bootstrap/Cargo.toml -q > "$TMP/cargo.out" 2>&1; then
    echo "CARGO OK"
  else
    echo "CARGO FAILED"
    tail -30 "$TMP/cargo.out"
  fi
else
  echo "SKIPPED (no cargo)"
fi

echo "=== corpus: phase wall seconds ==="
T_NOW=${EPOCHSECONDS:-$(date +%s)}
echo "targets ${T_TARGETS}s  rest $(( T_NOW - T_REST0 ))s  total $(( T_NOW - T_SCRIPT0 ))s"
