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
  examples/nat_prim.shard
  examples/axiom_kind_rejects.shard
  kernel/facts.shard
  std/bits/bits.shard
  examples/bits_demo.shard
  std/axiom_scope_rejects.shard
  examples/ledger_dep/ledger_dep.shard
  examples/auto_demo.shard
  examples/auto_missing_rejects.shard
  examples/admit_demo.shard
  examples/bytes_bridge.shard
  examples/contract_demo.shard
  examples/decl_rejects.shard
  examples/div_pairs.shard
  examples/double_claims.shard
  examples/use_demo.shard
  examples/double_lib.shard
  examples/shadow_rejects.shard
  examples/finsplit_test.shard
  examples/have_test.shard
  examples/inject_basic.shard
  examples/inject_rejects.shard
  examples/rewrite_at.shard
  examples/rewrite_at_rejects.shard
  examples/prove_cond_mine.shard
  examples/homonym_dispatch.shard
  examples/runhom_run.shard
  examples/inspect_demo.shard
  examples/cite_resolves.shard
  examples/cite_rejects.shard
  examples/req_dir_demo/consumer.shard
  examples/req_dir_demo/demo/demo.shard
  examples/req_gate_rejects/mod.req.shard
  examples/lia_basics.shard
  examples/lia_rejects.shard
  examples/list_named_hyp.shard
  examples/named_haves.shard
  examples/named_case_hyps.shard
  examples/reflect_demo.shard
  examples/reflect_rejects.shard
  examples/refine_basic.shard
  examples/refine_rejects.shard
  examples/refine_return.shard
  examples/refine_return_rejects.shard
  examples/refine_circular_rejects.shard
  examples/refine_try.shard
  examples/refine_try_rejects.shard
  examples/utf8_compute.shard
  examples/str_demo.shard
  examples/module_gate_rejects.shard
  examples/parse_rejects.shard
  examples/pending_demo.shard
  examples/reverse_proof.shard
  examples/rewrite_arms_test.shard
  examples/rewrite_with_demo.shard
  examples/tracer_demo.shard
  examples/unfold_scrutinee.shard
  examples/trust_ledger.shard
  examples/types_gate.shard
  examples/types_gate_cite.shard
  examples/wf_induct_demo.shard
  examples/measure_clause.shard
  examples/measure_import_synth.shard
  examples/measure_lex_demo.shard
  examples/io/cat_loop.shard
  examples/invoke_fixture.shard
  examples/invoke_probe.shard
  examples/measure_tree_demo.shard
  examples/nested_measure.shard
  examples/mem_reverse.shard
  examples/mem_copy.shard
  examples/mutual_toy.shard
  examples/record_proto.shard
  examples/record_basic.shard
  models/wasm/wasm.shard
  models/wasm/encode.shard
  examples/wasm_smoke.shard
  examples/wasm_pieces.shard
  examples/wasm_weld.shard
  examples/wasm_weld_out.shard
  examples/wasm_diff_run.shard
  examples/wasm_rev.shard
  examples/wasm_copy.shard
  examples/lowered_form.shard
  examples/w64_probe.shard
  models/x86/x86.shard
  models/x86/encode.shard
  examples/x86_pieces.shard
  std/mem/mem.x86.shard
  examples/xmemcall_probe.shard
  examples/xsibcall_probe.shard
  examples/xchain_probe.shard
  examples/xloopcall_probe.shard
  examples/libmod_probe.shard
  examples/lib_form.shard
  examples/lib_form_rejects.shard
  examples/purelib_src.shard
  examples/purelib_out.shard
  examples/purelib_x86_out.shard
  examples/x86_diff_run.shard
  examples/rep_probe.shard
  examples/lowfrag_probe.shard
  examples/divfrag_probe.shard
  examples/bitfrag_probe.shard
  examples/lowcheck_rejects.shard
  examples/record_rejects.shard
  examples/record_sugar_rejects.shard
  examples/statement_sugar.shard
  examples/statement_sugar_rejects.shard
  examples/chain_sugar.shard
  examples/chain_sugar_rejects.shard
  examples/named_cite_rejects.shard
  examples/compute_stop.shard
  examples/simp_stop.shard
  examples/subterm_induct.shard
  examples/subterm_induct_rejects.shard
  examples/struct_clause.shard
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
  std/bytes/bytes.shard
  std/str/utf8.shard
  std/str/str.shard
  std/list/list.shard
  std/arith/arith.shard
  std/div/div.shard
  std/map/map.shard
  std/word/word.shard
  std/nat/nat.shard
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
  bin/shard_eval run examples/wasm_weld.shard > "$TMP/weld.raw" 2>/dev/null
  bin/shard_eval run tools/shardfmt/shardfmt.shard "$TMP/weld.raw" > "$TMP/weld.fmt" 2>/dev/null
  if diff -q "$TMP/weld.fmt" examples/wasm_weld_out.shard >/dev/null; then
    echo "REGEN OK (byte-identical)"
  else
    echo "REGEN DRIFT: emitted certificate differs from examples/wasm_weld_out.shard"
  fi
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Nat-former RUN pin: ground construction/packing, view matching, deep
# patterns under the RUN engine (ev). Output must be engine-independent.
echo "=== nat: run probe ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run examples/nat_run_probe.shard
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
  bin/shard_eval run examples/facts_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# 64-bit range pin (the x86_64 arc's probe 0a): ground facts at and past
# 2^64 through the compiled chain's bignum runtime. Same OK-line pin
# discipline as facts_probe; the CHECK side rides the kernel TARGETS list
# (wrap64_id's farkas certificate carries 2^64 coefficients).
echo "=== facts: 64-bit range probe ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run examples/w64_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Dynamic-invocation pin: meta/invoke loads a fixture's closure AT RUNTIME
# (kernel-as-a-module), marshals values across the meta-level boundary, and
# calls fns by local name — the mod.build driver's foundation. Self-checking
# app: one OK/FAIL line per case, exit 0 iff all OK.
echo "=== invoke: dynamic-invocation probe ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run examples/invoke_probe.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Run-mode qualified-dispatch pin (the enc_instr bug): two co-loaded modules
# define the same-named internal helper; each pick must call its OWN. Also
# pins the run loop's World-identity threading (the app defines its own
# World and destructures the token write hands back — a mis-tagged token
# sticks loudly, exit 4). Self-checking: OK lines, exit 0 iff both hold.
echo "=== run-mode: qualified dispatch (runhom) ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run examples/runhom_run.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Engine-differential pin: encode the composite + const probe to real .wasm
# and replay the model-computed vectors under node/V8 (the slice-4 reality
# check — the "engine conforms to model" trust leaf). Summary line only;
# disagreements change it and fail the diff.
echo "=== wasm: engine differential ==="
if command -v node >/dev/null && [ -x bin/shard_eval ]; then
  bash examples/wasm_diff.sh 2>&1 | tail -1
else
  echo "SKIPPED (needs node + bin/shard_eval)"
fi

# Silicon differential (the x86_64 arc's Probe B): flatten each XFunc to real
# machine code, mmap it executable, and CALL it on this CPU — comparing the
# hardware result + memory (and the trap leg: model None == hardware fault)
# against the model. The "CPU conforms to the model" trust leaf. Summary line
# only; disagreements change it and fail the diff.
echo "=== x86: silicon differential ==="
if command -v cc >/dev/null && [ -x bin/shard_eval ]; then
  bash examples/x86_diff.sh 2>&1 | tail -1
else
  echo "SKIPPED (needs cc + bin/shard_eval)"
fi

echo "=== guard: absolute path ==="
out=$("${CHECK_CMD[@]}" "$PWD/examples/auto_demo.shard" 2>&1); code=$?
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
for LB in examples/lowbuild.sh examples/lowbuild_mem.sh examples/lowbuild_loop.sh examples/lowbuild_call.sh std/mem/lowbuild.sh std/str/lowbuild.sh examples/lowbuild_x86.sh examples/lowbuild_x86loop.sh examples/lowbuild_x86mem.sh examples/lowbuild_x86call.sh examples/lowbuild_x86chain.sh examples/lowbuild_x86loopcall.sh; do
  echo "=== lowering: $LB ==="
  if [ -x bin/shard_eval ]; then
    if bash "$LB" > "$TMP/lb.out" 2>&1; then
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
  if bin/shard_eval run tools/lowcheck/lowcheck.shard examples/lowcheck_rejects.shard > "$TMP/lc.out" 2>&1; then
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
  if bin/shard_eval run tools/lowcheck/manifest.shard examples/manifest_rejects.txt models/wasm/wasm.shard examples/wasmgen_call_link.shard > "$TMP/mf.out" 2>&1; then
    echo "GATE FAILED: misbound manifest ACCEPTED"
    tail -5 "$TMP/mf.out"
  else
    tail -1 "$TMP/mf.out"
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
