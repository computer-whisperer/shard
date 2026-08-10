#!/usr/bin/env bash
# examples/sha256sum/sha256sum_shani_diff.sh — THE SILICON LEG for the SHA-NI
# streaming sha256sum bin (docs/STREAM.md §8.4, rung B5/E3 slice D). The proofs
# say what the machine module does; this says the CPU agrees, by running the
# emitted ELF against TWO independent oracles on real input. The "hardware
# conforms to the model" trust leaf for a whole program, where
# models/x86/diff/shani_diff.sh is the same leaf per SHA-NI block and
# examples/sha256sum/sha256sum_silicon_diff.sh is the same leaf for the scalar
# twin. Dev-and-CI side; nothing here is in-logic.
#
# RUN FROM THE REPO ROOT (run_corpus.sh's differential legs all do):
#   bash examples/sha256sum/sha256sum_shani_diff.sh
#
# OUTPUT CONTRACT (inherited verbatim from the scalar twin). One `OK <row>` or
# `FAIL <row>` per row, AT COLUMN 0, and exit nonzero iff any row failed. CI
# gates on the ^(FAIL|TYPE!) projection, so nothing here may summarize, tail, or
# swallow a row — the wasm leg's `| tail -1` bug (fixed 2026-07-29) is the
# cautionary tale. Two consequences shape the code:
#   * `set -e` is DELIBERATELY OFF. A dying command must not take
#     already-printed FAIL rows with it.
#   * the EXIT trap prints its own FAIL row if control never reaches the end,
#     so a crash is as loud as a disagreement (never a silent green).
# A missing tool is a FAIL, not a SKIP: this leg exists to make assertions, and
# an assertion that quietly did not happen is the failure mode being guarded.
#
# THE ONE EXCEPTION — THE CPUID GATE, AND IT COMES FIRST. The product issues
# sha256rnds2/sha256msg1/sha256msg2, so on a CPU without the sha_ni flag it
# cannot be executed at all (it would SIGILL, which is not evidence about the
# encoder). That leg legitimately cannot run there, so this script prints ONE
# loud line and exits 0 before making any assertion, in shani_diff.sh's wording:
# a run that adjudicated NOTHING must never read as agreement. The CI runner is
# such a box — the skip line is the expected output there, and this leg is
# adjudicated on SHA-capable silicon. SHANI_DIFF_FORCE_NO_SHA=1 exercises that
# path on a capable box. Everything else missing (coreutils, openssl, getcap,
# readelf, dd, the engine) is still a FAIL/die, never a skip.
#
# THE DOUBLE ORACLE. This leg's ratified scope is "differential vs coreutils AND
# openssl", so every digest row computes the expectation TWICE — coreutils
# `sha256sum` first field, and `openssl dgst -sha256 -r` first field — and
# passes only when bin == coreutils == openssl. An ORACLE DISAGREEMENT (the two
# references differ from each other) gets its own FAIL text: it is a different
# event from the bin disagreeing, it says nothing about our encoder, and it must
# never be read as one. Note that openssl itself may be using the same silicon
# SHA extensions, so the second oracle is independent SOFTWARE, not independent
# hardware — the hardware leaf is still coreutils' scalar path.
#
# EXPECTATIONS NEVER COME FROM PINNED HEX, so the leg cannot drift into agreeing
# with itself. Every input is materialized as a FILE first and both expectations
# are taken from that file; the pipe writers replay it. (An earlier scratch
# version of the twin computed the expectation by re-running the writer, which
# silently hashed DIFFERENT random bytes for the two arms. Materialize once,
# read twice.)
#
# WHAT IT COVERS.
#   CAPLESS — the binary must load with no privilege: getcap EMPTY, RW PT_LOAD
#     at 0x10000 = stock vm.mmap_min_addr, and no PT_LOAD at vaddr 0. This is
#     the property that lets the leg run unprivileged at all.
#   THE STREAMING BIN — PIPE-FED, always, because short reads are the thing
#     under test: it loops read(≤61440) until EOF. Gold vectors, the 55..128
#     block strata, the 61440 read-cap crossings, a sub-cap sweep, byte-at-a-time
#     and small-chunk writers, a prefix-stop refutation, bulk, a seeded random
#     sweep, and both read-error legs. There is no one-shot bin in this variant.
set -uo pipefail

# --------------------------------------------------------------- CPUID gate --
# FIRST, before the trap and before any assertion: the only skip in this script.
if [ -n "${SHANI_DIFF_FORCE_NO_SHA:-}" ] || ! grep -qw sha_ni /proc/cpuinfo 2>/dev/null; then
  echo "sha256sum-shani silicon differential: SKIPPED, nothing adjudicated (no sha_ni on this cpu)"
  exit 0
fi

rc=0
finished=0
TMP=""

ok()  { printf 'OK %s\n' "$1"; }
bad() { printf 'FAIL %s\n' "$1"; rc=1; }

cleanup() {
  local code=$?
  [ -n "$TMP" ] && rm -rf "$TMP"
  if [ "$finished" -eq 0 ]; then
    printf 'FAIL sha256sum-shani-silicon: aborted before completion (exit %s) — rows above stand\n' "$code"
    exit 1
  fi
  exit "$rc"
}
trap cleanup EXIT

die() { bad "$1"; finished=1; exit 1; }

# ---------------------------------------------------------------- preflight --
[ -f examples/sha256sum/sha256sum_shani_write.shard ] \
  || die "preflight: not at the repo root (examples/sha256sum/sha256sum_shani_write.shard absent)"
command -v sha256sum >/dev/null 2>&1 \
  || die "preflight: coreutils sha256sum absent — it is oracle #1 of this leg's double oracle"
command -v openssl >/dev/null 2>&1 \
  || die "preflight: openssl absent — it is oracle #2 of this leg's double oracle"
command -v getcap >/dev/null 2>&1 \
  || die "preflight: getcap absent (libcap) — the capless assertion cannot be made"
command -v readelf >/dev/null 2>&1 \
  || die "preflight: readelf absent (binutils) — the vaddr-0 assertion cannot be made"
command -v dd >/dev/null 2>&1 || die "preflight: dd absent"
if [ -x bin/shard_eval ]; then EMIT=(bin/shard_eval)
elif [ -x ./rust_bootstrap/target/release/eval ]; then EMIT=(./rust_bootstrap/target/release/eval)
else die "preflight: no engine (bin/shard_eval or rust_bootstrap eval)"; fi
ok "preflight: repo root, sha_ni cpu, coreutils sha256sum, openssl, getcap, readelf, engine"

TMP=$(mktemp -d) || die "preflight: mktemp failed"

SHANI=examples/sha256sum/sha256sum_shani

# ------------------------------------------------------------- the oracles ---
# oracles FILE — sets EXP_CU / EXP_SSL. `openssl dgst -sha256 -r` prints the
# coreutils-ish "HEX *stdin" form, so field 1 is bare hex; the preflight row
# below asserts that shape rather than assuming it, because a format change
# would otherwise turn every row into a silent bin-disagreement.
EXP_CU=""
EXP_SSL=""
oracles() {
  EXP_CU=$(sha256sum < "$1" | cut -d' ' -f1)
  EXP_SSL=$(openssl dgst -sha256 -r < "$1" | cut -d' ' -f1)
}
printf 'abc' > "$TMP/abc"
oracles "$TMP/abc"
if [[ "$EXP_SSL" =~ ^[0-9a-f]{64}$ ]] && [ "$EXP_SSL" = "$EXP_CU" ]; then
  ok "preflight: openssl -r first field is bare 64-char lowercase hex and agrees with coreutils"
else
  die "preflight: openssl -r first field is not usable as an expectation — coreutils [$EXP_CU] openssl [$EXP_SSL]"
fi

# ------------------------------------------------------------------- emit ----
emit() { # $1 = write script, $2 = product, $3 = row label
  rm -f "$2"
  if "${EMIT[@]}" run "$1" >"$TMP/emit.log" 2>&1 && [ -s "$2" ]; then
    chmod +x "$2"
    ok "$3 ($(wc -c < "$2") bytes)"
  else
    bad "$3 — writer failed or produced no bytes; see below"
    sed 's/^/    /' "$TMP/emit.log" >&2
  fi
}
emit examples/sha256sum/sha256sum_shani_write.shard "$SHANI" "shani-emit-streaming-elf"

# ---------------------------------------------------------------- capless ----
# Each assertion is its own row: a binary that needs privilege is a different
# failure from a binary that hashes wrong, and CI should be able to tell them
# apart from the row name alone.
capless() { # $1 = binary, $2 = name
  local caps
  if [ ! -x "$1" ]; then bad "shani-capless-$2: no binary to inspect"; return; fi
  caps=$(getcap "$1" 2>/dev/null)
  if [ -z "$caps" ]; then ok "shani-capless-$2-getcap-empty"
  else bad "shani-capless-$2-getcap-empty — carries [$caps]"; fi
  if readelf -l "$1" 2>/dev/null | grep -q '0x0000000000010000 0x0000000000010000'; then
    ok "shani-capless-$2-rw-window-at-0x10000"
  else bad "shani-capless-$2-rw-window-at-0x10000 — RW PT_LOAD is elsewhere"; fi
  if readelf -l "$1" 2>/dev/null \
     | grep -qE 'LOAD.*0x0000000000000000 0x0000000000000000 0x0000000000000000'; then
    bad "shani-capless-$2-no-vaddr0-load — a PT_LOAD sits at vaddr 0 (the page-0 container is back)"
  else ok "shani-capless-$2-no-vaddr0-load"; fi
}
capless "$SHANI" "streaming"

# ------------------------------------------------------------- row helpers ---
# spipe FILE WRITER LABEL — WRITER writes FILE's bytes to stdout and is piped
# into the SHA-NI bin; both expectations are read from FILE itself, so the
# writer is never run twice and need not be deterministic.
spipe() {
  local f="$1" writer="$2" lbl="$3" got code
  got=$(eval "$writer" | "$SHANI"); code=$?
  oracles "$f"
  if [ "$EXP_CU" != "$EXP_SSL" ]; then
    bad "$lbl — ORACLE DISAGREEMENT: coreutils $EXP_CU vs openssl $EXP_SSL (this says nothing about the bin, which said [$got])"
    return
  fi
  if [ "$code" -ne 0 ]; then bad "$lbl — exit $code (expected 0)"; return; fi
  if [ "${#got}" -ne 64 ]; then bad "$lbl — wrote ${#got} chars, expected 64: [$got]"; return; fi
  if [ "$got" != "$EXP_CU" ]; then bad "$lbl — got $got expected $EXP_CU (both oracles agree)"; return; fi
  ok "$lbl"
}
# scat FILE LABEL — the plain `cat` pipe row
scat() { spipe "$1" "cat '$1'" "$2"; }

# ------------------------------------------- streaming: gold + NIST + strata --
printf ''    > "$TMP/empty"
printf 'abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq' > "$TMP/nist56"
scat "$TMP/empty"  "shani-stream-gold-empty"
scat "$TMP/abc"    "shani-stream-gold-abc"
scat "$TMP/nist56" "shani-stream-gold-nist56"
for n in 55 56 63 64 65 127 128; do
  head -c "$n" /dev/urandom > "$TMP/s$n"
  scat "$TMP/s$n" "shani-stream-stratum-${n}B"
done

# ------------------------------------------------ streaming: cap crossings ---
# 61440 is the model's per-read cap; these straddle it and its double.
for n in 61439 61440 61441 122880 122881; do
  head -c "$n" /dev/urandom > "$TMP/c$n"
  scat "$TMP/c$n" "shani-stream-capcross-${n}B"
done
# The OLD (4096) proving-rung cap's boundaries, kept as plain sub-cap rows:
# every one of them now fits in a single read, so they exercise the
# one-read-then-EOF shape rather than a crossing. c4097 / c8193 are also the
# inputs the short-read rows below replay.
for n in 4095 4096 4097 8192 8193; do
  head -c "$n" /dev/urandom > "$TMP/c$n"
  scat "$TMP/c$n" "shani-stream-subcap-${n}B"
done

# ------------------------------------------------- streaming: short reads ----
# Every pipe read that returns fewer than the requested 61440 bytes is a short
# read; these rows force many per run. (A pipe's own 64 KiB buffer means the
# plain `cat` rows above are already short-read rows at this cap.)
head -c 300 /dev/urandom > "$TMP/b300"
spipe "$TMP/b300"   "dd if='$TMP/b300' bs=1 2>/dev/null"      "shani-stream-shortread-dd-bs1-300B"
spipe "$TMP/c4097"  "dd if='$TMP/c4097' bs=1 2>/dev/null"     "shani-stream-shortread-dd-bs1-4097B"
spipe "$TMP/c8193"  "dd if='$TMP/c8193' bs=7 2>/dev/null"     "shani-stream-shortread-dd-bs7-8193B"
spipe "$TMP/c8193"  "dd if='$TMP/c8193' bs=4095 2>/dev/null"  "shani-stream-shortread-dd-bs4095-8193B"
spipe "$TMP/c61441" "dd if='$TMP/c61441' bs=4095 2>/dev/null" "shani-stream-shortread-dd-bs4095-across-cap-61441B"

# PREFIX-STOP REFUTATION — the behavioural proof that the loop keeps reading
# after a short read, needing no tracer (this box has no strace and `perf trace`
# wants root). 40 bytes dripped with real gaps cannot be satisfied by one read
# of the 61440-byte request, so a bin that stopped at the first short read would
# digest a PREFIX. Asserted both ways: equals the whole input, matches no
# proper prefix.
printf 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX' > "$TMP/drip40"
dripped=$( { for i in $(seq 1 40); do printf 'X'; sleep 0.01; done; } | "$SHANI" )
oracles "$TMP/drip40"
whole="$EXP_CU"
if [ "$EXP_CU" != "$EXP_SSL" ]; then
  bad "shani-stream-prefix-stop-refuted-40x1B-10ms-gaps — ORACLE DISAGREEMENT: coreutils $EXP_CU vs openssl $EXP_SSL"
else
  pfx=""
  for k in $(seq 1 39); do
    head -c "$k" "$TMP/drip40" > "$TMP/pfx"
    [ "$dripped" = "$(sha256sum < "$TMP/pfx" | cut -d' ' -f1)" ] && pfx="$k"
  done
  if [ "$dripped" = "$whole" ] && [ -z "$pfx" ]; then
    ok "shani-stream-prefix-stop-refuted-40x1B-10ms-gaps"
  else
    bad "shani-stream-prefix-stop-refuted-40x1B-10ms-gaps — got $dripped whole $whole prefix-match-at ${pfx:-none}"
  fi
fi

# ------------------------------------------------------- streaming: bulk -----
head -c 1048576 /dev/urandom > "$TMP/m1"
scat  "$TMP/m1" "shani-stream-bulk-1MiB"
spipe "$TMP/m1" "dd if='$TMP/m1' bs=997 2>/dev/null" "shani-stream-bulk-1MiB-in-997B-chunks"

# ------------------------------------------------ streaming: random sweep ----
# Shapes are seeded so a failing row is REPRODUCIBLE (same size/chunk next run);
# the content is fresh /dev/urandom every run, which is what varies the digest.
# Size and chunk are in the row name so a FAIL is diagnosable without a re-run.
# The size bound is ~3x the read cap so the sweep keeps straddling it; the chunk
# bound stays sub-cap, which is what makes every write a short read.
RANDOM=20260809
for i in $(seq 1 20); do
  sz=$(( (RANDOM * 32768 + RANDOM) % 200000 ))
  bs=$(( RANDOM % 4200 + 1 ))
  head -c "$sz" /dev/urandom > "$TMP/r"
  spipe "$TMP/r" "dd if='$TMP/r' bs=$bs 2>/dev/null" "shani-stream-random size=$sz chunk=$bs"
done

# -------------------------------------------------- streaming: error legs ----
# A read error must fail-stop: exit 1, nothing on stdout (§7.2). `.` is the repo
# root, a directory, so read(2) answers EISDIR; `<&-` leaves fd 0 closed, EBADF.
out=$("$SHANI" < . 2>/dev/null); code=$?
if [ "$code" -eq 1 ] && [ -z "$out" ]; then ok "shani-stream-errorleg-EISDIR-exit1-empty"
else bad "shani-stream-errorleg-EISDIR-exit1-empty — exit $code stdout=[$out]"; fi
out=$("$SHANI" <&- 2>/dev/null); code=$?
if [ "$code" -eq 1 ] && [ -z "$out" ]; then ok "shani-stream-errorleg-EBADF-exit1-empty"
else bad "shani-stream-errorleg-EBADF-exit1-empty — exit $code stdout=[$out]"; fi

finished=1
