#!/usr/bin/env bash
# Live guard for the loud-stuckness hardening (kernel/evm.shard, X86.md era
# QoL item 1): run-mode stuckness and malformed extern arguments must FAIL
# LOUDLY (exit 4 + an error naming the offending head), never exit 0.
# Before the hardening both controls exited 0 — the stuck one silently, the
# poison one after TRUNCATING its output at the stuck byte (measured
# 2026-07-09: stdout was "H" where "H?\n" was offered).
set -u
cd "$(dirname "$0")/.."
fail=0

out=$(bin/shard_eval run examples/stuckctl_exit.shard 2>&1); rc=$?
if [ "$rc" -ne 4 ] || ! echo "$out" | grep -q "match fell through"; then
  echo "STUCKCTL FAIL: stuck exit code did not die loudly (rc=$rc)"; fail=1
else
  echo "STUCKCTL OK: stuck exit dies loudly (rc=4)"
fi

out=$(bin/shard_eval run examples/stuckctl_write.shard 2>&1); rc=$?
if [ "$rc" -ne 4 ] || ! echo "$out" | grep -q "error:"; then
  echo "STUCKCTL FAIL: poisoned write did not die loudly (rc=$rc)"; fail=1
else
  echo "STUCKCTL OK: poisoned write dies loudly (rc=4)"
fi

exit $fail
