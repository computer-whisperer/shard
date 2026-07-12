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
  examples/axiom_untagged_rejects.shard
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
  models/imp/imp.shard
  examples/imp_probe.shard
  examples/imp_scalar.shard
  examples/imp_loop.shard
  models/imp/to_wasm.shard
  examples/imp_wasm_bridge.shard
  examples/imp_wasm_loop_bridge.shard
  models/imp/to_x86.shard
  examples/imp_x86_bridge.shard
  examples/imp_x86_loop_bridge.shard
  std/sha256/sha256.imp.shard
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
  examples/x86_window_law.shard
  examples/xsibcall_probe.shard
  examples/xchain_probe.shard
  examples/xloopcall_probe.shard
  examples/xintloop_probe.shard
  examples/xcopyloop_probe.shard
  examples/xtransform_probe.shard
  examples/xfoldloop_probe.shard
  examples/xdiv_probe.shard
  examples/lxkernel_probe.shard
  examples/xworld_probe.shard
  examples/xadequacy_probe.shard
  examples/xweff_probe.shard
  examples/addw_src.shard
  examples/addw_x86_out.shard
  examples/xitoa_probe.shard
  examples/x86div_src.shard
  examples/x86div_out.shard
  examples/x86itoa_src.shard
  examples/x86itoa_out.shard
  examples/xbinadd_probe.shard
  examples/add_src.shard
  examples/add_x86_out.shard
  examples/xbinsum_probe.shard
  examples/xid_probe.shard
  examples/bytesum_src.shard
  examples/bytesum_x86_out.shard
  examples/libmod_probe.shard
  examples/lib_form.shard
  examples/lib_form_rejects.shard
  examples/purelib_src.shard
  examples/purelib_out.shard
  examples/build_products.shard
  tools/build/build.shard
  examples/purelib_x86_out.shard
  examples/arglen_src.shard
  examples/arglen_x86_out.shard
  examples/echoarg_src.shard
  examples/echoarg_x86_out.shard
  examples/upcase_src.shard
  examples/upcase_x86_out.shard
  examples/parse_src.shard
  examples/parse_x86_out.shard
  examples/bin_entry_rejects.shard
  examples/x86_diff_run.shard
  examples/rep_probe.shard
  examples/lowfrag_probe.shard
  examples/divfrag_probe.shard
  examples/bitfrag_probe.shard
  examples/wordfrag_probe.shard
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
  examples/struct_mutual_list.shard
  examples/adq13_probe.shard
  examples/natview_pin.shard
  examples/natview_pin2.shard
  examples/natview_rejects.shard
  examples/zerocase_rejects.shard
  examples/canon_pin.shard
  examples/canon_rejects.shard
  tools/canon/rewrite.shard
  tools/canon/canon.shard
  tools/canon/census.shard
  tools/canon/hash.shard
  examples/hash_pin.shard
  examples/parlet_pin.shard
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
  examples/sketch_pin.shard
  tools/search/rev_obj.shard
  tools/search/rev.shard
  tools/search/search.shard
  tools/search/census.shard
  tools/search/catalog.shard
  tools/search/sym.shard
  examples/spell_pin.shard
  tools/search/render_gate.shard
  tools/search/gen/rev_synth.shard
  tools/search/gen/cat_bracket.shard
  tools/search/superpose.shard
  tools/search/subsume.shard
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

# Ground-search pin (docs/SEARCH.md slice 1): the rev accumulator space,
# rank-addressed by meta/sketch and settled through the real machine
# (meta/invoke -> evm_call_pure). Counts and solution sets must match the
# playground's published measurement record EXACTLY: 108 candidates / 1
# solution at depth 1, 7788 / 13 at depth 2. Every COUNT / SOL / SOLUTIONS
# line is diffed; a grammar or addressing change moves them and fails the
# corpus diff (re-pin deliberately, with the change).
echo "=== search: ground rev pin ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run tools/search/search.shard
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
  bin/shard_eval run tools/search/census.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Catalog census pin (docs/SEARCH.md S7-lite, G5): the structural list
# fragment enumerated at rungs 1-2, post-filtered through cn_e, and
# battery-bucketed by behavior. GEN/CLEAN/BEHAVIORS + the flagged
# content-family tallies + rev/id spelling counts are the arc's first
# spellings-per-behavior numbers against the REAL dialect (rung 1:
# 20/17/13 — the playground's certified "exactly 13"; rung 2:
# 3395/2345/1068 — behaviors match the playground's 1068, rev = 2
# spellings). A canon-rule or grammar change moves these lines and
# fails the diff — re-pin deliberately, with the change.
echo "=== search: catalog census (G5) ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run tools/search/catalog.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Laws-oracle pin (docs/SEARCH.md S4a+S5, G3): the symbolic evaluator +
# requirements-as-oracle. SELF: std/list's own rev and len must
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
  bin/shard_eval run tools/search/laws.shard
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
  bin/shard_eval run tools/search/render_gate.shard
else
  echo "SKIPPED (no bin/shard_eval)"
fi

# Superposed-executor pin (docs/SEARCH.md S4b, built AS RATIFIED —
# named superpose; "narrow" is the bootstrap dialect's name): the
# choices-map machine settles the rev spaces EXACTLY — d1: 108
# candidates in 26 regions (8 forks), 1 found; d2: 7,788 in 443
# regions (133 forks), 13 found. STEPS pins the consulted-choice-set
# memo's leverage (pre-memo baselines: 896 / 29,008 — the memo halves
# re-evaluation at d2 and compounds with depth). AGREE extends G3
# three ways: found coverage == the enumerative solution count, every
# enumerative solution lies in a found region, and every region
# representative passes the kernel/evm battery. Any drift exits 1
# inside the tool.
echo "=== search: superposed executor (S4b) ==="
if [ -x bin/shard_eval ]; then
  bin/shard_eval run tools/search/superpose.shard
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
  bin/shard_eval run tools/search/hunt.shard
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
  bin/shard_eval run tools/search/subsume.shard
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
for LB in "tools/build/build.sh examples/build_products.shard"; do
  echo "=== lowering: $LB ==="
  if [ -x bin/shard_eval ]; then
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

# Percolation-twin negative pin (docs/X86.md §49): a smuggled XSyscall in a
# module with an empty declared effect surface must be REFUSED by bytetie
# — before this gate a hidden effect-point in a "pure" body was checked by
# nothing at the byte level.
echo "=== lowering: percolation negative fixture ==="
if [ -x bin/shard_eval ]; then
  if bin/shard_eval run tools/bytetie/bytetie.shard examples/percolation_rejects.shard > "$TMP/pc.out" 2>&1; then
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
  if bin/shard_eval run kernel/check.shard examples/pat_binder_rejects.shard > "$TMP/pb.out" 2>&1; then
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
  bin/shard_eval run kernel/check.shard examples/canon_pin.shard > "$TMP/cnp.out" 2>&1
  cnp_rc=$?
  if [ $cnp_rc -ne 0 ]; then
    echo "FAIL canon_pin (exit $cnp_rc)"
  elif grep -q '^CANON ' "$TMP/cnp.out"; then
    echo "FAIL canon_pin (canonical bodies flagged)"
    grep '^CANON ' "$TMP/cnp.out"
  else
    echo "CANON PIN OK"
  fi
  bin/shard_eval run kernel/check.shard examples/canon_rejects.shard > "$TMP/cnr.out" 2>&1
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
  bin/shard_eval run tools/canon/canon.shard examples/canon_pin.shard > "$TMP/cnt_pin.out" 2>/dev/null
  if cmp -s "$TMP/cnt_pin.out" examples/canon_pin.shard; then
    echo "CANON TOOL PIN-IDENTITY OK"
  else
    echo "FAIL canon_tool (pin file not a fixed point of the rewriter)"
  fi
  # the rewritten file must sit in examples/ so its relative imports resolve
  bin/shard_eval run tools/canon/canon.shard examples/canon_rejects.shard > examples/.cnt_rej_tmp.shard 2>/dev/null
  cnt_rc=$?
  if [ $cnt_rc -ne 0 ]; then
    echo "FAIL canon_tool (rejects rewrite exited $cnt_rc)"
    rm -f examples/.cnt_rej_tmp.shard
  else
    bin/shard_eval run kernel/check.shard examples/.cnt_rej_tmp.shard > "$TMP/cnt_rej.out" 2>&1
    rm -f examples/.cnt_rej_tmp.shard
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
  std_canon=$(ls std/*/*.shard | grep -v '\.auto\.shard$\|\.low\.shard$' | while read -r f; do
    "${CHECK_CMD[@]}" "$f" 2>/dev/null | grep '^CANON ' | sed "s|^|$f |"
  done)
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
  bin/shard_eval run tools/canon/hash.shard examples/hash_pin.shard > "$TMP/hx1.out" 2>&1
  bin/shard_eval run tools/canon/hash.shard examples/hash_pin.shard > "$TMP/hx2.out" 2>&1
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
