#!/usr/bin/env bash
# examples/binelf_probe.sh — the hand-driven BIN-rung de-risk probe (docs/X86.md
# §21). Emits the two-segment ELF from examples/binelf_probe.shard, then on real
# Linux:
#   CHECK 1  run with the fixed arg "hello": stdout (8-byte LE result) == EXPOUT
#   CHECK 2  run with NO argument: process exits with code 1 (the argc guard)
# Loud PASS/FAIL per check; nonzero exit on any failure. No `diff && echo`
# patterns that could swallow a red result. Run from the repo root.
set -uo pipefail
EVAL=${EVAL:-bin/shard_eval}
ARG=hello  # must match tstr in examples/binelf_probe.shard
command -v xxd >/dev/null || { echo "REFUSED: no xxd — the probe cannot un-hex"; exit 1; }
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== emit: build the two-segment ELF + the expected stdout"
"$EVAL" run examples/binelf_probe.shard > "$TMP/probe.txt" || {
  echo "FAIL: emitter did not run"; cat "$TMP/probe.txt"; exit 1; }
grep '^ELF ' "$TMP/probe.txt" | cut -d' ' -f2 | xxd -r -p > "$TMP/a.out"
EXP=$(grep '^EXPOUT ' "$TMP/probe.txt" | cut -d' ' -f2)
if [ ! -s "$TMP/a.out" ] || [ -z "$EXP" ]; then
  echo "FAIL: emitter produced no ELF/EXPOUT"; cat "$TMP/probe.txt"; exit 1
fi
chmod +x "$TMP/a.out"
echo "  ELF bytes: $(wc -c < "$TMP/a.out"); EXPOUT: $EXP"

rc=0

echo "== check 1: run with arg \"$ARG\" — stdout == EXPOUT"
GOT=$("$TMP/a.out" "$ARG" | xxd -p | tr -d '\n')
if [ "$GOT" = "$EXP" ]; then
  echo "PASS check 1 (stdout $GOT == EXPOUT)"
else
  echo "FAIL check 1:"
  echo "  got: $GOT"
  echo "  exp: $EXP"
  rc=1
fi

echo "== check 2: run with NO argument — exit code 1 (argc guard)"
set +e
"$TMP/a.out" >/dev/null 2>&1
code=$?
set -e 2>/dev/null || true
if [ "$code" -eq 1 ]; then
  echo "PASS check 2 (no-arg exit code $code)"
else
  echo "FAIL check 2 (no-arg exit code $code, expected 1)"
  rc=1
fi

if [ "$rc" -eq 0 ]; then
  echo "BINELF PROBE OK: two PT_LOAD + argv contract + glue copy + image read, on silicon"
else
  echo "BINELF PROBE FAILED"
fi
exit $rc
