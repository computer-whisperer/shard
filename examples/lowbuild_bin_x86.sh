#!/usr/bin/env bash
# examples/lowbuild_bin_x86.sh SRC OUT — the GENERIC x86 BIN BUILD (docs/
# X86.md §21/§22, the bin rung): one script for every (bin …) source lowered
# by tools/x86gen BIN mode into a PLAINLY EXECUTABLE Linux ELF. A bin is
# admitted AS a one-export lib, so the schema/kernel/tie/manifest machinery is
# the lib script's, unchanged over the same OUT; the bin-specific legs are the
# glue-contract SURFACE gate and the BINELF packaging + on-silicon ENGINE run.
# Gates:
#   1. REGEN      — x86gen BIN mode output is byte-identical to OUT
#   2. SCHEMA     — tools/lowcheck validates the entry's cert (x86 lib form)
#      KERNEL     — OUT's machine proofs check; SRC's bin acceptance checks
#      TIE        — the image assembled FROM THE CERTS re-encodes to the plan's
#                   XMOD bytes; MANIFEST binds name -> cert -> pinned index
#   3. SURFACE    — the accepts tool's bin arm: the entry's cert-premise surface
#                   is covered by the GLUE CONTRACT (v1: must be empty)
#   4. PLAN-ENGINE — the derived lib plan replays on the REAL CPU (x86_diff.c)
#   5. BINELF     — lowbuild binelf emits the executable; IMGTIE: the embedded
#                   image == bytetie's cert-assembled TIEIMG
#   6. ENGINE     — the ELF is run AS A USER WOULD: the no-arg leg, then one run
#                   per BVEC (hexarg '-' = the empty-string argument, otherwise
#                   the hex decoded to the literal argument); exit codes AND
#                   stdout must equal the derived plan's. Exit-variant lines
#                   carry no OUT field: stdout must be EMPTY (contract 5a writes
#                   nothing). Write-variant lines carry 'OUT <hex|->': stdout
#                   must be exactly those bytes (contract 5b: sys_write(1, BUF,
#                   len), exit 0). WORLD-bin lines (docs/X86.md §47) carry
#                   'EXIT n OUT <hex|->' both no-arg and two-arg: the expected
#                   stdout/exit come from evaluating xrun_w — the bin equation
#                   at ground values. One pool argument is 300 chars, pinning
#                   the MAXLEN=255 truncation clause (5a: expected exit 255;
#                   5b: exactly 255 bytes out).
# Exit 0 = a fully gated, plainly-executable artifact. Run from the repo root.
set -uo pipefail
[ $# -eq 2 ] || { echo "usage: lowbuild_bin_x86.sh SRC OUT"; exit 2; }
SRC=$1
OUT=$2
EVAL=${EVAL:-bin/shard_eval}
command -v xxd >/dev/null || { echo "REFUSED: no xxd — the BINELF/ENGINE gates cannot run"; exit 1; }
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT
rc=0
fail() { echo "FAIL $1"; rc=1; }

echo "== gate 1: regen (producer determinism)"
if "$EVAL" run tools/x86gen/x86gen.shard "$SRC" "$TMP/out.raw" >/dev/null 2>"$TMP/regen.err" \
   && "$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/out.raw" > "$TMP/out.fmt" 2>>"$TMP/regen.err" \
   && diff -q "$TMP/out.fmt" "$OUT" >/dev/null; then
  echo "PASS regen (byte-identical)"
else
  fail "regen (x86gen BIN output drifts from OUT)"; cat "$TMP/regen.err"
fi

echo "== gate 2a: schema (consumer-side cert validation)"
if "$EVAL" run tools/lowcheck/lowcheck.shard "$OUT" > "$TMP/schema.txt" 2>&1; then
  tail -1 "$TMP/schema.txt"; echo "PASS schema"
else
  cat "$TMP/schema.txt"; fail "schema"
fi

echo "== gate 2b: kernel (machine proofs + the bin acceptance)"
if [ -x bin/shard_check ] && [ "$(bin/engine_stamp.sh)" = "$(cat bin/shard_check.stamp 2>/dev/null)" ]; then
  CHECK=(bin/shard_check)
else
  CHECK=("$EVAL" run kernel/check.shard)
fi
"${CHECK[@]}" "$OUT" > "$TMP/kv.txt" 2>&1
if tail -1 "$TMP/kv.txt" | grep -q " 0 failed"; then
  tail -1 "$TMP/kv.txt"; echo "PASS kernel (OUT proofs)"
else
  tail -1 "$TMP/kv.txt"; fail "kernel (OUT has failing obligations)"
fi
"${CHECK[@]}" "$SRC" > "$TMP/ks.txt" 2>&1
if grep -q "^BIN   " "$TMP/ks.txt" && tail -1 "$TMP/ks.txt" | grep -q " 0 failed"; then
  grep "^BIN   " "$TMP/ks.txt"; tail -1 "$TMP/ks.txt"; echo "PASS kernel (SRC bin acceptance)"
else
  grep "^BIN   " "$TMP/ks.txt" || true; tail -1 "$TMP/ks.txt"; fail "kernel (SRC bin acceptance)"
fi

echo "== gate 2c: byte tie (certs -> assembled image -> enc_image = shipped bytes)"
"$EVAL" run tools/lowbuild/lowbuild.shard lib "$SRC" "$OUT" x86 > "$TMP/plan.txt" 2>"$TMP/plan.err" || {
  cat "$TMP/plan.err"; fail "plan derivation"; }
# SRC rides along: bytetie reads the bin form's extern count as the
# declared effect surface for the percolation gate (2e)
"$EVAL" run tools/bytetie/bytetie.shard "$OUT" "$SRC" > "$TMP/tie.txt" 2>&1 || {
  tail -1 "$TMP/tie.txt"; fail "bytetie run"; }
grep '^XMOD ' "$TMP/plan.txt" | sort > "$TMP/mods.txt"
grep '^TIE ' "$TMP/tie.txt" | sed 's/^TIE /XMOD /' | sort > "$TMP/ties.txt"
if diff "$TMP/ties.txt" "$TMP/mods.txt" > "$TMP/tiediff.txt"; then
  echo "PASS byte tie"
else
  cat "$TMP/tiediff.txt"; fail "byte tie (certs != shipped bytes)"
fi

echo "== gate 2d: manifest (name -> cert -> pinned index)"
if "$EVAL" run tools/lowcheck/manifest.shard "$TMP/plan.txt" models/x86/x86.shard "$OUT" > "$TMP/man.txt" 2>&1; then
  tail -1 "$TMP/man.txt"; echo "PASS manifest"
else
  cat "$TMP/man.txt"; fail "manifest"
fi

echo "== gate 2e: percolation (syscall bytes == the declared effect surface)"
# docs/X86.md §49: bytetie refuses on any hidden effect-point; the EFF
# line asserts the check actually ran (anti-drift: a bytetie that stops
# emitting it fails here, not silently)
EFFL=$(grep '^EFF ' "$TMP/tie.txt" || true)
if [ -n "$EFFL" ] && printf '%s\n' "$EFFL" | grep -q ' OK$'; then
  echo "PASS percolation ($EFFL)"
else
  fail "percolation (no EFF OK line from bytetie)"
fi

echo "== gate 3: surface (the glue-contract premise gate, bin arm)"
if "$EVAL" run tools/lowcheck/accepts.shard "$SRC" "$OUT" > "$TMP/acc.txt" 2>&1 \
   && grep -q "(glue-covered)" "$TMP/acc.txt"; then
  grep "BINSURFACE" "$TMP/acc.txt"; echo "PASS surface"
else
  cat "$TMP/acc.txt"; fail "surface (entry's cert surface exceeds the glue contract)"
fi

echo "== gate 4: plan-engine (the CPU replays the derived lib plan)"
if command -v cc >/dev/null; then
  grep -v '^ARTIFACT ' "$TMP/plan.txt" > "$TMP/cpu_plan.txt"
  cc -O2 -o "$TMP/x86_diff" examples/x86_diff.c 2>"$TMP/cc.err" || { cat "$TMP/cc.err"; fail "cc"; }
  if [ -x "$TMP/x86_diff" ] && "$TMP/x86_diff" "$TMP/cpu_plan.txt" > "$TMP/eng.txt" 2>&1; then
    tail -1 "$TMP/eng.txt"; echo "PASS plan-engine"
  else
    cat "$TMP/eng.txt"; fail "plan-engine (CPU differential)"
  fi
else
  echo "REFUSED: no cc — the PLAN-ENGINE gate cannot run"; fail "plan-engine (no cc)"
fi

echo "== gate 5: binelf (the plainly-executable ELF, cert-tied image)"
"$EVAL" run tools/lowbuild/lowbuild.shard binelf "$SRC" "$OUT" > "$TMP/be.txt" 2>"$TMP/be.err" || {
  cat "$TMP/be.err"; fail "binelf derivation"; }
IMG=$(grep '^IMG ' "$TMP/be.txt" | cut -d' ' -f2)
# ^TIEIMG anchored: the '^TIE ' family collides on prefix otherwise
TIEIMG=$(grep '^TIEIMG ' "$TMP/tie.txt" | cut -d' ' -f2)
if [ -n "$IMG" ] && [ "$IMG" = "$TIEIMG" ]; then
  echo "PASS binelf IMGTIE (embedded image == cert-assembled image)"
else
  echo "  IMG=$IMG"; echo "  TIEIMG=$TIEIMG"; fail "binelf IMGTIE"
fi
grep '^ELF ' "$TMP/be.txt" | cut -d' ' -f2 | xxd -r -p > "$TMP/a.bin"
chmod +x "$TMP/a.bin"
if [ -s "$TMP/a.bin" ]; then
  echo "PASS binelf ($(wc -c < "$TMP/a.bin") bytes)"
else
  fail "binelf (empty ELF)"
fi

echo "== gate 6: engine (run the binary as a user would)"
# expected stdout for one line: no OUT field (exit variant) or OUT '-' =
# empty; otherwise the OUT hex decoded to bytes
want_out() { # $1 = OUT hex field ('' or '-' = empty)
  if [ -z "$1" ] || [ "$1" = "-" ]; then : > "$TMP/want.out"; else
    printf '%s' "$1" | xxd -r -p > "$TMP/want.out"; fi
}
# pool-coverage fence (docs/X86.md §49): a WORLD bin's vector pool must
# not shrink — the no-arg leg carries OUT (the at-least-once store) and
# at least five two-arg legs ride BVEC2 (single-digit, multi-digit,
# differing-length, and the MAXLEN truncation pair live in lowbuild's
# pinned pool; this fence refuses a shrunken pool rather than passing)
if grep -q '^BVEC2 .* OUT ' "$TMP/be.txt"; then
  grep -q '^BNOARG EXIT [0-9]* OUT ' "$TMP/be.txt" \
    || fail "pool coverage (a WORLD bin's no-arg leg lacks OUT)"
  N2=$(grep -c '^BVEC2 ' "$TMP/be.txt")
  [ "$N2" -ge 5 ] || fail "pool coverage (BVEC2 pool shrank to $N2 < 5)"
fi
BNOARG=$(grep '^BNOARG EXIT ' "$TMP/be.txt" | awk '{print $3}')
want_out "$(grep '^BNOARG EXIT ' "$TMP/be.txt" | awk '{print $5}')"
"$TMP/a.bin" > "$TMP/got.out"; code=$?
if [ "$code" = "$BNOARG" ] && cmp -s "$TMP/got.out" "$TMP/want.out"; then
  echo "PASS engine no-arg (exit $code == BNOARG $BNOARG, stdout $(wc -c < "$TMP/got.out") byte(s))"
else
  fail "engine no-arg (exit $code expected $BNOARG, stdout $(wc -c < "$TMP/got.out") expected $(wc -c < "$TMP/want.out") byte(s))"
fi
while read -r _bv hexarg _e want _kw outhex; do
  want_out "$outhex"
  if [ "$hexarg" = "-" ]; then
    "$TMP/a.bin" "" > "$TMP/got.out"; code=$?; label="'' (empty string)"
  else
    arg=$(printf '%s' "$hexarg" | xxd -r -p)
    "$TMP/a.bin" "$arg" > "$TMP/got.out"; code=$?; label="<${#arg} byte(s)>"
  fi
  if [ "$code" = "$want" ] && cmp -s "$TMP/got.out" "$TMP/want.out"; then
    echo "PASS engine $label -> exit $code, stdout $(wc -c < "$TMP/got.out") byte(s)"
  else
    fail "engine $label (exit $code expected $want, stdout $(wc -c < "$TMP/got.out") expected $(wc -c < "$TMP/want.out") byte(s))"
  fi
done < <(grep '^BVEC ' "$TMP/be.txt")

# two-arg vectors (BVEC2 hex1 hex2 EXIT n [OUT hex]) — exit-variant lines
# carry no OUT field (stdout must be empty); the WORLD bin's lines carry
# the model-predicted stdout (docs/X86.md §47: xrun_w's WExit trace)
while read -r _bv hex1 hex2 _e want _kw outhex; do
  want_out "$outhex"
  if [ "$hex1" = "-" ]; then a1=""; else a1=$(printf '%s' "$hex1" | xxd -r -p); fi
  if [ "$hex2" = "-" ]; then a2=""; else a2=$(printf '%s' "$hex2" | xxd -r -p); fi
  "$TMP/a.bin" "$a1" "$a2" > "$TMP/got.out"; code=$?
  if [ "$code" = "$want" ] && cmp -s "$TMP/got.out" "$TMP/want.out"; then
    echo "PASS engine <'${#a1}'+'${#a2}' byte args> -> exit $code, stdout $(wc -c < "$TMP/got.out") byte(s)"
  else
    fail "engine two-arg (exit $code expected $want, stdout $(wc -c < "$TMP/got.out") expected $(wc -c < "$TMP/want.out") byte(s))"
  fi
done < <(grep '^BVEC2 ' "$TMP/be.txt")

if [ "$rc" -eq 0 ]; then
  echo "ARTIFACT OK: $(basename "$SRC" .shard) — a plainly-executable Linux ELF, proven entry, six gates green"
else
  echo "BIN BUILD FAILED"
fi
exit $rc
