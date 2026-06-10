#!/usr/bin/env bash
# Type-gate sweep: run `check` over every kernel file + the core tools, in
# parallel (JOBS env, default nproc). Each gate run re-checks the file's whole
# import closure (~11min for the big kernel closures), so the serial sweep is
# hours; parallel it's bounded by the slowest single file. Output is buffered
# per file and emitted in list order (byte-diffable across runs).
# A file is CLEAN iff its run prints nothing but the pass/fail tally and the
# tally has 0 failed; the summary line at the bottom collects non-clean files.
set -u
# Engine selection: native binary (bin/rebuild.sh) when present, else the Rust
# interpreter. EVAL env overrides. The native engine is the DEV loop only --
# soundness-authority runs use EVAL=./rust_bootstrap/target/release/eval.
if [ -z "${EVAL:-}" ]; then
  if [ -x bin/shard_eval ]; then
    EVAL=bin/shard_eval
    if [ "$(bin/engine_stamp.sh)" != "$(cat bin/shard_eval.stamp 2>/dev/null)" ]; then
      echo "WARNING: bin/shard_eval is STALE vs kernel/compiler sources -- run bin/rebuild.sh" >&2
    fi
  else
    EVAL=./rust_bootstrap/target/release/eval
  fi
fi
# The native engine bump-allocates and NEVER frees: a big kernel-closure gate
# peaks ~40GB RSS. Cap native concurrency so the sweep can't swap-storm the
# box (3 x 40GB on 125GB); the Rust interpreter stays at nproc (~MBs each).
if [ "$EVAL" = bin/shard_eval ]; then
  JOBS="${JOBS:-3}"
else
  JOBS="${JOBS:-$(nproc)}"
fi
TARGETS=(
  kernel/stdlib.shard
  kernel/term.shard
  kernel/module.shard
  kernel/proof.shard
  kernel/reduce.shard
  kernel/types.shard
  kernel/farkas.shard
  kernel/lia.shard
  kernel/ord.shard
  kernel/eqdec.shard
  kernel/checker.shard
  kernel/desugar.shard
  kernel/proof_reader.shard
  kernel/reader.shard
  kernel/loader.shard
  kernel/trace.shard
  kernel/driver.shard
  kernel/check.shard
  kernel/eval.shard
  tools/prove/prove.shard
  tools/shardfmt/shardfmt.shard
)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

for i in "${!TARGETS[@]}"; do
  while (( $(jobs -rp | wc -l) >= JOBS )); do wait -n; done
  {
    echo "=== ${TARGETS[$i]} ==="
    $EVAL run kernel/check.shard "${TARGETS[$i]}" 2>&1
  } > "$TMP/$i" &
done
wait

bad=()
for i in "${!TARGETS[@]}"; do
  cat "$TMP/$i"
  grep -q ", 0 failed" "$TMP/$i" || bad+=("${TARGETS[$i]}")
done
echo
if (( ${#bad[@]} )); then
  echo "GATE: ${#bad[@]} file(s) NOT clean: ${bad[*]}"
  exit 1
else
  echo "GATE: all ${#TARGETS[@]} files clean"
fi
