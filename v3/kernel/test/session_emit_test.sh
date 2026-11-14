#!/usr/bin/env bash
# v3/kernel/test/session_emit_test.sh — declarations built as data, lowered under the names
# they were admitted, prepared and invoked under: construct, prepare once, invoke repeatedly,
# lower, all under one identity. session_test.shard's `emit` mode introduces
# v3/pins/session/data.shard as a
# revision, builds `show3 : Nat → World → World` and `main : World → World := show3 2` as terms
# of K, admits them through the raw entry, takes their derived views, writes the revision's
# closure as the back end's C text (emit.shard: lower.shard, cgen.shard) and
# runs `main` through a prepared handle. This test compiles that text and runs the binary: its
# output must be the handle's, byte for byte. Exit code = the number of failed checks.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
v3/kernel/test/init_index.sh > /dev/null || { echo "session_emit_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
OUT=$(mktemp -d); trap 'rm -rf "$OUT"' EXIT
mkdir -p "$OUT/emit"
fail=0; checks=0
bad() { echo "session_emit_test: $*"; fail=$((fail+1)); }

"$EVAL" direct v3/kernel/test/session_test.shard emit "$OUT/emit" > "$OUT/run.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the emit mode failed (exit $rc)"; head -3 "$OUT/run.txt" | cut -c1-300; }
entry=$(sed -n 's/^EMIT: entry=//p' "$OUT/run.txt")
checks=$((checks+1)); [ -n "$entry" ] && [ -s "$OUT/emit/$entry" ] || { bad "no entry file was emitted"; entry=none; }
handle=$(sed -n '2p' "$OUT/run.txt")
checks=$((checks+1)); [ "$handle" = "6" ] || { bad "the handle's run of main did not print 6"; sed -n '2,3p' "$OUT/run.txt" | cut -c1-200; }
# the two are in the generated text as functions named by the identities they were admitted
# under — main over the World, show3 over a Nat and the World — and not in the source
checks=$((checks+1)); grep -qh '^static value_t s_pins_x2esession_x2edata_x2emain(value_t p_0) {' "$OUT"/emit/p*.c || { bad "the generated text has no function main over the World"; grep -hn 'x2emain(' "$OUT"/emit/*.c | head -3 | cut -c1-200; }
checks=$((checks+1)); grep -qh '^static value_t s_pins_x2esession_x2edata_x2eshow3(value_t p_0, value_t p_1) {' "$OUT"/emit/p*.c || { bad "the generated text has no function show3 over (Nat, World)"; grep -hn 'x2eshow3(' "$OUT"/emit/*.c | head -3 | cut -c1-200; }
checks=$((checks+1)); ! grep -qE '\(fn (main|show3)\b' v3/pins/session/data.shard || bad "the pin's source declares main or show3: they would not be the ones built as data"

if cc -O2 -o "$OUT/main" "$OUT/emit/$entry" -I v3/kernel > "$OUT/cc.txt" 2>&1; then
  got=$("$OUT/main" 2>&1); rc=$?
  checks=$((checks+1)); [ "$rc" = 0 ] && [ "$got" = "$handle" ] || bad "the binary printed '$(echo "$got" | head -1 | cut -c1-80)' (exit $rc), the handle '$handle'"
else
  checks=$((checks+1)); bad "the generated text did not compile"; tail -3 "$OUT/cc.txt" 2>/dev/null | cut -c1-300
fi
echo "session_emit_test: $checks checks, $fail failed"
exit $fail
