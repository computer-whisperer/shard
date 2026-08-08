#!/usr/bin/env bash
# shani_diff.sh — THE BLOCK-GRAIN SILICON DIFFERENTIAL (docs/STREAM.md §8.4,
# rung B5/E2c), end to end: the model-side plan emitter
# (shani_diff_run.shard: the article's hand body as real machine-code bytes,
# plus expectations — the stored state from the VALUE tier's shn_block_b, the
# whole XMM file from the vector-tier walk) executed by the engine-side
# replayer (x86_diff.c), which is the CPU itself. Dev-side only; nothing here
# is in-logic. Run from the repo root. Exit 0 = full agreement AND every
# un-blinding tooth bit at least one row.
#
# Row set (fixed, ~8 s to emit, milliseconds to replay — no CI/local split):
#   57 positive rows  8 gold single-block + 48 random state x block x entry
#                     XMM file + 1 two-block CHAINED (feedback through memory)
#   40 tooth rows     5 perturbations x the 8 gold rows, scored INVERTED
#
# On a CPU without sha_ni every row SKIPs loudly and the exit stays 0 (the
# x86_diff.c CPUID gate, pipeline #289's repair) — the CI runner is such a box,
# so this leg is adjudicated on SHA-capable silicon. X86_DIFF_FORCE_NO_SHA=1
# exercises that path here.
set -uo pipefail
command -v cc >/dev/null || { echo "REFUSED: no cc on PATH — the differential cannot run"; exit 1; }
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
if [ -x bin/shard_eval ]; then EMIT=(bin/shard_eval); else EMIT=(./rust_bootstrap/target/release/eval); fi
"${EMIT[@]}" run models/x86/diff/shani_diff_run.shard > "$TMP/plan.txt" || {
  echo "FAIL shani-plan-emit (exit $?)"; echo "sha-ni block differential: PLAN EMIT FAILED"; exit 1; }
cc -O2 -o "$TMP/x86_diff" models/x86/diff/x86_diff.c || {
  echo "FAIL shani-replayer-build"; echo "sha-ni block differential: REPLAYER BUILD FAILED"; exit 1; }

# the plan's own pin (the relocation identity) rides at the head of the plan as
# a comment, or as a ^FAIL line the CI projection catches
grep -m1 '^# shani' "$TMP/plan.txt" || true
pinbad=0
if grep '^FAIL ' "$TMP/plan.txt"; then pinbad=1; fi

"$TMP/x86_diff" "$TMP/plan.txt" > "$TMP/out.txt"
rc=$?
# Per-row SKIPs are COLLAPSED to one line per module before they reach stdout.
# On a runner without sha_ni this leg skips 97 rows, and the CI trace's
# `grep '^FAIL \|^TYPE! \|^SKIP ' | head -80` projection would then be nothing
# but this file's SKIPs — hiding every FAIL that follows it, which is #289's
# blindness rebuilt. The collapse keeps the information (module + row count) at
# the position of the module's first skipped row and leaves every other line,
# including SKIP TOOTH and every FAIL, verbatim. x86_diff.c's own per-row
# printing is untouched, so the E1 leg's output shape does not move.
awk 'NR==FNR { if (/^SKIP XVN?CASE /) c[$3]++; next }
     /^SKIP XVN?CASE / { if (!seen[$3]++)
         printf "SKIP XVCASE %s: %d rows [no sha_ni on this cpu]\n", $3, c[$3];
       next }
     { print }' "$TMP/out.txt" "$TMP/out.txt"
summary=$(grep '^x86 silicon differential:' "$TMP/out.txt" | tail -1)

# The verdict, ALWAYS printed and always last: pipeline #289's lesson is that a
# gate whose answer is not on stdout at the end is a gate nobody reads. The
# no-sha_ni case gets its OWN word — a run that adjudicated nothing must never
# read as agreement.
if [ "$rc" -ne 0 ] || [ "$pinbad" -ne 0 ]; then
  echo "sha-ni block differential: DISAGREE (replayer exit $rc, plan pin bad=$pinbad) -- $summary"
  [ "$rc" -eq 0 ] && rc=1
elif printf '%s' "$summary" | grep -q '^x86 silicon differential: 0 agree, 0 disagree, [0-9]* skipped'; then
  echo "sha-ni block differential: SKIPPED, nothing adjudicated (no sha_ni on this cpu) -- $summary"
else
  echo "sha-ni block differential: AGREE, the emitted hand body matches shn_block_b on silicon -- $summary"
fi
exit "$rc"
