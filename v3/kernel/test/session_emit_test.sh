#!/usr/bin/env bash
# v3/kernel/test/session_emit_test.sh — declarations built as data, lowered under the names
# they were admitted, prepared and invoked under (LANGUAGE.md §8.6 slice 4.6 rule 2; GPT-6 R61;
# the T6 row's "construct, prepare once, invoke repeatedly, transform, check, lower under one
# identity"). session_test.shard's `emit` mode introduces v3/pins/session/data.shard as a
# revision, builds `show3 : Nat → World → World` and `main : World → World := show3 2` as terms
# of K, admits them through the raw entry, takes their derived views, writes the revision's
# closure as route 1's chain reads it and runs `main` through a prepared handle. This test
# lowers that closure (tools/lower → tools/codegen → cc) and runs the binary: its output must
# be the handle's, byte for byte. Exit code = the number of failed checks.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
BOOT=bin/shard_eval; [ -x "$BOOT" ] || BOOT=${RUST_EVAL:-./rust_bootstrap/target/release/eval}   # the chain's engine, as v3/build.sh takes it
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
# the two are in the emitted module under the names they were admitted under, and not in the source
checks=$((checks+1)); grep -q '^(fn main ((v0 World)) World' "$OUT/emit/$entry" || { bad "the emitted module has no fn main over the World"; grep -n 'main' "$OUT/emit/$entry" | head -3 | cut -c1-200; }
checks=$((checks+1)); grep -q '^(fn show3 ((v0 Int) (v1 World)) World' "$OUT/emit/$entry" || { bad "the emitted module has no fn show3 over (Int, World)"; grep -n 'show3' "$OUT/emit/$entry" | head -3 | cut -c1-200; }
checks=$((checks+1)); ! grep -qE '\(fn (main|show3)\b' v3/pins/session/data.shard || bad "the pin's source declares main or show3: they would not be the ones built as data"

if "$BOOT" run tools/lower/lower.shard "$OUT/emit/$entry" "$OUT/low.shard" > "$OUT/lower.txt" 2>&1 && [ -s "$OUT/low.shard" ] \
   && "$BOOT" run tools/codegen/codegen.shard "$OUT/low.shard" "$OUT/low.c" > "$OUT/codegen.txt" 2>&1 && [ -s "$OUT/low.c" ] \
   && cc -O2 -o "$OUT/main" "$OUT/low.c" -I tools/codegen > "$OUT/cc.txt" 2>&1; then
  got=$("$OUT/main" 2>&1); rc=$?
  checks=$((checks+1)); [ "$rc" = 0 ] && [ "$got" = "$handle" ] || bad "the binary printed '$(echo "$got" | head -1 | cut -c1-80)' (exit $rc), the handle '$handle'"
else
  checks=$((checks+1)); bad "the chain did not lower the emitted closure"; tail -2 "$OUT/lower.txt" "$OUT/codegen.txt" "$OUT/cc.txt" 2>/dev/null | cut -c1-300
fi
echo "session_emit_test: $checks checks, $fail failed"
exit $fail
