#!/usr/bin/env bash
# tools/build/build.sh PRODUCTS — THE build wrapper (docs/BUILD.md §7,
# rung 1): the driver plans RUN orders, this loop executes them (byte-
# moving only: run argv, capture stdout+stderr and the exit code), then
# the driver verifies the captures. ALL build knowledge lives in
# tools/build/build.shard; nothing here inspects an output.
#
# The plan phase is a FIXPOINT: the driver emits only orders whose
# capture file is missing, so a round that derives new data (a bin's
# be.txt vector list) makes the next round emit the runs that depend on
# it; an empty order list ends the loop. An argv token `@FILE` is
# replaced by FILE's contents as ONE argument (byte-moving — arbitrary
# bytes except NUL, which execve forbids in arguments anyway).
set -euo pipefail
cd "$(dirname "$0")/../.."
PRODUCTS=${1:?usage: build.sh PRODUCTS.shard}
EVAL=${EVAL:-bin/shard_eval}
command -v node >/dev/null || { echo "REFUSED: no node — the wasm ENGINE gate cannot run"; exit 1; }
command -v cc >/dev/null || { echo "REFUSED: no cc — the x86 ENGINE gate cannot run"; exit 1; }
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK="bin/shard_check"
else
  CHECK="$EVAL run kernel/check.shard"
fi
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT
ROUND=0
while :; do
  ROUND=$((ROUND+1))
  [ "$ROUND" -le 6 ] || { echo "build.sh: plan did not reach a fixpoint"; exit 2; }
  "$EVAL" run tools/build/build.shard plan "$PRODUCTS" "$TMP" "$EVAL" $CHECK > "$TMP/orders.txt" \
    || { echo "build.sh: plan refused:" >&2; cat "$TMP/orders.txt" >&2; exit 1; }
  [ -s "$TMP/orders.txt" ] || break
  while read -r tag cap args; do
    [ "$tag" = RUN ] || { echo "build.sh: bad order line: $tag $cap"; exit 2; }
    newargs=()
    for tok in $args; do
      case "$tok" in
        @*) f=${tok#@}; a=$(cat "$f"; printf x); newargs+=("${a%x}");;
        *)  newargs+=("$tok");;
      esac
    done
    rc=0
    "${newargs[@]}" > "$cap" 2>&1 || rc=$?
    echo "$rc" > "$cap.rc"
  done < "$TMP/orders.txt"
done
vrc=0
"$EVAL" run tools/build/build.shard verify "$PRODUCTS" "$TMP" || vrc=$?
if [ "$vrc" != 0 ]; then
  trap - EXIT
  echo "build.sh: captures kept at $TMP" >&2
fi
exit "$vrc"
