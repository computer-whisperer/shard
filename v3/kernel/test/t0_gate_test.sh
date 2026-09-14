#!/usr/bin/env bash
# v3/kernel/test/t0_gate_test.sh — the positive byte-tie of v3/t0_full.sh under
# stub engines (GPT-6 R54): the gate must require each engine to complete, not
# only the two logs to agree. Both engines are shell stubs printing one canned
# log (a verdict line, an axiom line, a pins line) and exiting as told; the
# export directory is a throwaway with a one-line init.ndjson and the matching
# one-line oracle; the full replay is skipped (V3_T0_SKIP_FULL=1).
#   1. both exit 0, complete, identical            -> the gate passes
#   2. both exit 1 with identical logs             -> fails (agreement on a failure)
#   3. one exits 1                                 -> fails
#   4. both exit 0, identical, no verdict line     -> fails (a truncated log)
cd "$(dirname "$0")/../../.."
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/export"
echo '{"stub":1}' > "$tmp/export/init.ndjson"
printf 'AX stub.c:\tstub.ax\n' > "$tmp/export/init.axioms"
for side in interp native; do
  cat > "$tmp/$side" <<'STUB'
#!/usr/bin/env bash
# STUB_LOG: the canned log; STUB_EXIT_<SIDE>: the exit status
cat "$STUB_LOG"
eval "exit \${STUB_EXIT_$(basename "$0" | tr a-z A-Z):-0}"
STUB
  chmod +x "$tmp/$side"
done
printf 'DECL stub.c ok\nAX stub.c:\tstub.ax\nT0: accepted 1  rejected 0  exhausted 0  mismatched 0  unsupported 0\nPINS pinned: | unpinned:\n' > "$tmp/complete.log"
grep -v '^T0: ' "$tmp/complete.log" > "$tmp/truncated.log"
run() {  # run LABEL EXPECTED_STATUS LOG EXIT_INTERP EXIT_NATIVE
  local out rc
  out=$(STUB_LOG=$3 STUB_EXIT_INTERP=$4 STUB_EXIT_NATIVE=$5 V3_EXPORT_DIR=$tmp/export K=$tmp/native RUST_EVAL=$tmp/interp V3_T0_SKIP_FULL=1 \
        v3/t0_full.sh "$tmp/t0.log" 2>&1); rc=$?
  if [ "$rc" -eq "$2" ]; then echo "PASS $1 (exit $rc)"; else echo "FAIL $1: exit $rc, expected $2"; echo "$out" | tail -8; failed=$((failed+1)); fi
}
failed=0
run "both complete and agree: the gate passes" 0 "$tmp/complete.log" 0 0
run "both fail identically: the gate fails" 1 "$tmp/complete.log" 1 1
run "the interpreter fails: the gate fails" 1 "$tmp/complete.log" 1 0
run "the native engine fails: the gate fails" 1 "$tmp/complete.log" 0 1
run "identical truncated logs without the verdict line: the gate fails" 1 "$tmp/truncated.log" 0 0
echo "t0_gate_test: failures = $failed"
exit $failed
