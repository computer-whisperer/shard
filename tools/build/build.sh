#!/usr/bin/env bash
# tools/build/build.sh PRODUCTS — THE build wrapper (docs/BUILD.md §7,
# rung 1): the driver plans RUN orders, this loop executes them (byte-
# moving only: run argv, capture stdout+stderr and the exit code), then
# the driver verifies the captures. ALL build knowledge lives in
# tools/build/build.shard; nothing here inspects an output.
set -euo pipefail
cd "$(dirname "$0")/../.."
PRODUCTS=${1:?usage: build.sh PRODUCTS.shard}
EVAL=${EVAL:-bin/shard_eval}
command -v node >/dev/null || { echo "REFUSED: no node — the ENGINE gate cannot run"; exit 1; }
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK="bin/shard_check"
else
  CHECK="$EVAL run kernel/check.shard"
fi
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT
"$EVAL" run tools/build/build.shard plan "$PRODUCTS" "$TMP" "$EVAL" $CHECK > "$TMP/orders.txt" \
  || { echo "build.sh: plan refused:" >&2; cat "$TMP/orders.txt" >&2; exit 1; }
while read -r tag cap args; do
  [ "$tag" = RUN ] || { echo "build.sh: bad order line: $tag $cap"; exit 2; }
  rc=0
  $args > "$cap" 2>&1 || rc=$?
  echo "$rc" > "$cap.rc"
done < "$TMP/orders.txt"
vrc=0
"$EVAL" run tools/build/build.shard verify "$PRODUCTS" "$TMP" || vrc=$?
if [ "$vrc" != 0 ]; then
  trap - EXIT
  echo "build.sh: captures kept at $TMP" >&2
fi
exit "$vrc"
