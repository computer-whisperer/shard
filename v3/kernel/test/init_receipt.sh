#!/usr/bin/env bash
# v3/kernel/test/init_receipt.sh — the Init receipt current, its path printed
# (LANGUAGE.md §8.4 slice 3.23). The receipt is `t0.shard --receipt`'s: the
# export chunks a T0 run accepted, by path and byte count; under it the loader
# streams Init in ADMIT mode (load.shard --init-receipt). It is a cache of K's
# verdict on the pinned fixture and nothing more: regenerated here whenever K's
# sources or the fixture changed since it was written (the `# k=` header), never
# committed (v3/.cache/ is ignored), never read by verify_release.
#   INIT_RECEIPT=$(v3/kernel/test/init_receipt.sh)   # v3/test.sh does this once
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
FIX=v3/kernel/test/fixtures/init_prefix_int.ndjson
TAIL=v3/kernel/test/fixtures/init_prefix_str_tail.ndjson
R=v3/.cache/init.receipt
# K's identity for the receipt: the sealed module and the kernel files it imports
k=$(cat v3/kernel/k/*.shard v3/kernel/prelude.shard v3/kernel/host.shard v3/kernel/name.shard v3/kernel/level.shard \
       v3/kernel/expr.shard v3/kernel/decl.shard v3/kernel/intmap.shard v3/kernel/json.shard v3/kernel/verdict.shard \
       v3/kernel/util.shard "$FIX" "$TAIL" | sha256sum | cut -c1-16)
if [ -f "$R" ] && [ "$(head -1 "$R")" = "# k=$k" ]; then echo "$R"; exit 0; fi
mkdir -p v3/.cache
tmp=$(mktemp "v3/.cache/init.receipt.XXXXXX")
if "$EVAL" direct v3/kernel/t0.shard --receipt "$tmp" "$FIX" "$TAIL" > "$tmp.log" 2>&1 && [ -s "$tmp" ]; then
  { echo "# k=$k"; cat "$tmp"; } > "$tmp.h" && mv "$tmp.h" "$R"; rm -f "$tmp" "$tmp.log"
  echo "$R"
else
  echo "init_receipt.sh: T0 over the fixture did not accept everything; no receipt" >&2
  tail -3 "$tmp.log" >&2; rm -f "$tmp" "$tmp.log"; exit 1
fi
