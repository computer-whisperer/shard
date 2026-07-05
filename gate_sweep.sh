#!/usr/bin/env bash
# Type-gate sweep: run `check` over every kernel file + the core tools, in
# parallel (JOBS env, default nproc). Each gate run re-checks the file's whole
# import closure (~11min for the big kernel closures), so the serial sweep is
# hours; parallel it's bounded by the slowest single file. Output is buffered
# per file and emitted in list order (byte-diffable across runs).
# A file is CLEAN iff its run prints nothing but the pass/fail tally and the
# tally has 0 failed; the summary line at the bottom collects non-clean files.
set -u
# Engine selection, fastest fresh option first:
#   1. bin/shard_check (check.shard compiled DIRECTLY; ~0.2s/155MB on the
#      biggest gate) when present AND stamp-fresh -- build: bin/rebuild.sh check
#   2. bin/shard_eval interpreting kernel/check.shard (stale shard_check only
#      warns; a stale DIRECT checker is silently wrong logic, so it is never
#      used past its stamp)
#   3. the Rust interpreter
# EVAL env overrides with the interpreter command shape. The native chain is
# the DEV loop only -- soundness-authority runs use
# EVAL=./rust_bootstrap/target/release/eval.
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
# bin/shard_eval interpreting check.shard bump-allocates and NEVER frees: a
# big kernel-closure gate peaks ~40GB RSS. Cap that mode's concurrency so the
# sweep can't swap-storm the box (3 x 40GB on 125GB). shard_check (~155MB) and
# the Rust interpreter (~MBs) run at nproc.
if [ "${EVAL:-}" = bin/shard_eval ]; then
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
  kernel/arith.shard
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
  tools/lowergen/lowergen.shard
  tools/lowcheck/lowcheck.shard
  tools/bytetie/bytetie.shard
  meta/invoke/invoke.shard
  meta/plan/plan.shard
  tools/lowbuild/lowbuild.shard
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
