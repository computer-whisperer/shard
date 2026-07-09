#!/usr/bin/env bash
# Run every pinned five-gate build CONCURRENTLY (they are independent —
# separate tmp dirs, disjoint outputs); buffer per-build output and emit in
# list order so the result stays byte-diffable with a serial run. Wall time
# = the slowest single build instead of the sum. Exit 1 if any build fails.
set -u
cd "$(dirname "$0")/.."
BUILDS=(
  examples/lowbuild.sh
  examples/lowbuild_mem.sh
  examples/lowbuild_loop.sh
  examples/lowbuild_call.sh
  std/mem/lowbuild.sh
  std/str/lowbuild.sh
  examples/lowbuild_x86.sh
  examples/lowbuild_x86loop.sh
  examples/lowbuild_x86mem.sh
  examples/lowbuild_x86call.sh
  examples/lowbuild_x86chain.sh
  examples/lowbuild_x86loopcall.sh
  examples/lowbuild_x86intloop.sh
  "examples/lowbuild_lib.sh examples/purelib_src.shard examples/purelib_out.shard"
  "examples/lowbuild_lib_x86.sh examples/purelib_src.shard examples/purelib_x86_out.shard"
  "examples/lowbuild_lib_x86_elf.sh examples/purelib_src.shard examples/purelib_x86_out.shard"
  "examples/lowbuild_bin_x86.sh examples/arglen_src.shard examples/arglen_x86_out.shard"
  "examples/lowbuild_bin_x86.sh examples/bytesum_src.shard examples/bytesum_x86_out.shard"
  "examples/lowbuild_bin_x86.sh examples/echoarg_src.shard examples/echoarg_x86_out.shard"
  "examples/lowbuild_bin_x86.sh examples/upcase_src.shard examples/upcase_x86_out.shard"
  "examples/lowbuild_bin_x86.sh examples/parse_src.shard examples/parse_x86_out.shard"
)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

for i in "${!BUILDS[@]}"; do
  # unquoted: an entry may carry arguments (the generic lib build)
  { ${BUILDS[$i]} > "$TMP/$i.out" 2>&1; echo $? > "$TMP/$i.rc"; } &
done
wait

fail=0
for i in "${!BUILDS[@]}"; do
  echo "=== ${BUILDS[$i]}"
  tail -1 "$TMP/$i.out"
  if [ "$(cat "$TMP/$i.rc")" != 0 ]; then
    echo "FAILED (${BUILDS[$i]}) — full output:"
    cat "$TMP/$i.out"
    fail=1
  fi
done
exit $fail
