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
JOBS="${JOBS:-$(nproc)}"
TARGETS=(
  examples/add_nat_zero.shard
  examples/axiom_kind_rejects.shard
  examples/auto_demo.shard
  examples/auto_missing_rejects.shard
  examples/contract_demo.shard
  examples/decl_rejects.shard
  examples/div_pairs.shard
  examples/double_claims.shard
  examples/use_demo.shard
  examples/double_lib.shard
  examples/shadow_rejects.shard
  examples/finsplit_test.shard
  examples/have_test.shard
  examples/homonym_dispatch.shard
  examples/cite_resolves.shard
  examples/cite_rejects.shard
  examples/lia_basics.shard
  examples/lia_rejects.shard
  examples/list_named_hyp.shard
  examples/module_gate_rejects.shard
  examples/parse_rejects.shard
  examples/reverse_proof.shard
  examples/rewrite_arms_test.shard
  examples/rewrite_with_demo.shard
  examples/tracer_demo.shard
  examples/trust_ledger.shard
  examples/types_gate.shard
  examples/types_gate_cite.shard
  examples/types_gate_word.shard
  examples/utf8_classify.shard
  examples/wf_induct_demo.shard
  examples/word_facts.shard
  examples/word_facts_signed.shard
  examples/modules_demo/consumer.shard
  examples/calc/calc_proof.shard
  examples/calc/calc_spec_tests.shard
  examples/calc/calc_reconcile_tests.shard
  examples/snake_game/snake.shard
  examples/snake_game_2/arena.shard
  std/mem.shard
  std/list.shard
  std/map.shard
  std/arith.shard
  std/div.shard
  std/nat.shard
  std/order.shard
)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

for i in "${!TARGETS[@]}"; do
  while (( $(jobs -rp | wc -l) >= JOBS )); do wait -n; done
  {
    echo "=== ${TARGETS[$i]} ==="
    "${CHECK_CMD[@]}" "${TARGETS[$i]}" 2>&1
  } > "$TMP/$i" &
done
wait

for i in "${!TARGETS[@]}"; do cat "$TMP/$i"; done
