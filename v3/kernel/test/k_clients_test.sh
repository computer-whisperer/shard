#!/usr/bin/env bash
# K's tests through the V3 loader (phase 3, the third of the four follow-ups after the K
# seal; v3/LANGUAGE.md §6.6, §13 item 26): every other entrypoint runs on the Rust
# bootstrap, which resolves flat and enforces no seal, so a test that reached around K's
# view would still pass there. Here each K-facing test is loaded by the V3 loader and run
# under `ev` (`load.shard --run`), the clients outside the directory with `v3/kernel/k` as
# a second root (the directory's own check links the implementation), the tests inside
# with the implementation in their closure — so the sealed-directory rule, the view and
# the E-side visibility are load-bearing at test time. Exit = the number of tests that
# did not load or did not pass.
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
failed=0; n=0; s=$(date +%s)
for t in v3/kernel/test/hostile_test.shard v3/kernel/test/k_client_test.shard v3/kernel/k/test/*_test.shard; do
  n=$((n+1)); name=$(basename "$t" .shard)
  case "$t" in v3/kernel/k/*) main="kernel.k.test.$name.main"; extra="";; *) main="kernel.test.$name.main"; extra="v3/kernel/k";; esac
  out=$("$EVAL" direct v3/kernel/load.shard --root v3 --run "$main" "$t" $extra 2>&1); rc=$?
  if [ "$rc" -ne 0 ] || ! echo "$out" | grep -q "^$name: failures = 0$"; then
    failed=$((failed+1)); echo "k_clients_test: $t: exit $rc"; echo "$out" | grep -vE '^(PASS|RUNNABLE|ACCEPT|MODULE|INIT|PARAM|DISCHARGE|IMPL)' | tail -8
  fi
done
e=$(date +%s)
echo "k_clients_test: $n tests through the V3 loader, $failed failed, in $((e-s)) s"
exit $failed
