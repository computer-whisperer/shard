#!/usr/bin/env bash
# micro_silicon.sh — THE SILICON LEG of the frame tier (docs/COVERAGE.md
# C1c-2): the generic path's first compiled program on a real CPU.
#
# For every t_* wrapper of tools/impc/fixtures/micro.shard it builds ONE static
# Linux ELF (tools/impc/fixtures/micro_x86_write.shard — the frame tier's
# translation of impc's product plus a trampoline that pre-writes the entry
# frame and exits with the wrapper's Int in the low byte of RDI), RUNS it, and
# compares the PROCESS EXIT STATUS against the `EXPECT` line the emitter prints
# — which is the spec wrapper's own value, evaluated natively by the engine,
# reduced mod 256. Exit status is a byte, so this is the wrapper's Int mod 256:
# t_second_short = -1 arrives as 255, t_find = -4 as 252.
#
# One row per wrapper plus one REFUSAL row (index 29 = `id`, a real fn of the
# module but not a wrapper — the emitter must decline it with exit 2). Exit 0 =
# every row PASS, 1 = any row FAIL.
#
# The ELFs land in tools/impc/fixtures/micro_elf/*.bin — gitignored by the
# repo's `*.bin` rule; nothing here is tracked.
#
# Dev-side only, nothing in-logic. Run from the repo root:
#   tools/impc/fixtures/micro_silicon.sh
set -uo pipefail

if [ -x bin/shard_eval ]; then EVAL=(bin/shard_eval); else EVAL=(./rust_bootstrap/target/release/eval); fi
SRC=tools/impc/fixtures/micro_x86_write.shard
OUTDIR=tools/impc/fixtures/micro_elf
mkdir -p "$OUTDIR"

fails=0

# name index  — the table indexes are micro_ipc_out.shard's NAME_ix values
ROWS=(
  "len 17"
  "len0 18"
  "app 19"
  "perim 20"
  "second 21"
  "second_short 22"
  "sumto 23"
  "rev 24"
  "pick 25"
  "find 26"
  "find_none 27"
  "shapes 28"
  "arith 30"
  "cmp 31"
  "let 32"
)

for row in "${ROWS[@]}"; do
  set -- $row
  name=$1; ix=$2
  elf="$OUTDIR/t_$name.bin"
  rm -f "$elf"
  out=$("${EVAL[@]}" run "$SRC" "$ix" "$elf" 2>&1)
  rc=$?
  if [ $rc -ne 0 ]; then
    echo "FAIL silicon $name: emitter exit $rc ($out)"
    fails=$((fails + 1))
    continue
  fi
  want=$(printf '%s\n' "$out" | sed -n 's/^EXPECT \([0-9]*\)$/\1/p')
  if [ -z "$want" ]; then
    echo "FAIL silicon $name: no EXPECT line ($out)"
    fails=$((fails + 1))
    continue
  fi
  chmod +x "$elf"
  "./$elf"
  got=$?
  if [ "$got" = "$want" ]; then
    echo "PASS silicon $name: exit $got want $want"
  else
    echo "FAIL silicon $name: exit $got want $want"
    fails=$((fails + 1))
  fi
done

# the refusal leg: index 29 is `id`, a compiled fn of the module but not a
# wrapper, so the emitter has no spec value for it and must decline
out=$("${EVAL[@]}" run "$SRC" 29 "$OUTDIR/refused.bin" 2>&1)
rc=$?
if [ $rc -eq 2 ] && [ ! -e "$OUTDIR/refused.bin" ]; then
  echo "PASS silicon refuse-29: emitter exit $rc want 2, no ELF written"
else
  echo "FAIL silicon refuse-29: emitter exit $rc want 2 ($out)"
  fails=$((fails + 1))
fi

echo "rows failed: $fails"
[ "$fails" -eq 0 ]
