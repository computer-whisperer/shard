#!/usr/bin/env bash
# examples/sha256sum/sha256sum_silicon_diff.sh — THE SILICON LEG for both
# sha256sum bins (docs/STREAM.md §7.9 M5 slice 4). The proofs say what the
# machine module does; this says the CPU agrees, by running the emitted ELFs
# against coreutils on real input. The "hardware conforms to the model" trust
# leaf for a whole program, where models/x86/diff/x86_diff.sh is the same leaf
# per XFunc. Dev-and-CI side; nothing here is in-logic.
#
# RUN FROM THE REPO ROOT (run_corpus.sh's differential legs all do):
#   bash examples/sha256sum/sha256sum_silicon_diff.sh
#
# OUTPUT CONTRACT. One `OK <row>` or `FAIL <row>` per row, AT COLUMN 0, and
# exit nonzero iff any row failed. CI gates on the ^(FAIL|TYPE!) projection, so
# nothing here may summarize, tail, or swallow a row — the wasm leg's `| tail -1`
# bug (fixed 2026-07-29) is the cautionary tale. Two consequences shape the code:
#   * `set -e` is DELIBERATELY OFF. A dying command must not take
#     already-printed FAIL rows with it.
#   * the EXIT trap prints its own FAIL row if control never reaches the end,
#     so a crash is as loud as a disagreement (never a silent green).
# A missing tool is a FAIL, not a SKIP: this leg exists to make assertions, and
# an assertion that quietly did not happen is the failure mode being guarded.
#
# WHAT IT COVERS.
#   CAPLESS — both binaries must load with no privilege: getcap EMPTY, and no
#     PT_LOAD at vaddr 0. The RW window sits at 0x10000 = stock
#     vm.mmap_min_addr. This is the property that lets the leg run in CI at all;
#     the page-0 era needed setcap or a global sysctl and could not be gated.
#   STREAMING BIN — PIPE-FED, always, because short reads are the thing under
#     test: it loops read(≤4096) until EOF. Gold vectors, the 55..128 block
#     strata, the 4096 read-cap crossings, byte-at-a-time and small-chunk
#     writers, a prefix-stop refutation, bulk, a random sweep, and both
#     read-error legs.
#   ONE-SHOT BIN — FILE REDIRECT, always, because it issues exactly ONE read at
#     cap 64888 and a pipe may short-read it into a truncated digest. Its cap
#     boundary is pinned on both sides: `XBrIf (CEq RSI (SImm 64888))` takes the
#     fail path when the read returns EXACTLY 64888 (a full buffer cannot be
#     told from a truncated one), so ≤64887 hashes and ≥64888 fail-stops.
#
# EXPECTATIONS COME FROM COREUTILS `sha256sum`, first field — never from pinned
# hex, so the leg cannot drift into agreeing with itself. Every input is
# materialized as a FILE first and the expectation is taken from that file;
# the pipe writers replay it. (An earlier scratch version computed the
# expectation by re-running the writer, which silently hashed DIFFERENT random
# bytes for the two arms. Materialize once, read twice.)
set -uo pipefail

rc=0
finished=0
TMP=""

ok()  { printf 'OK %s\n' "$1"; }
bad() { printf 'FAIL %s\n' "$1"; rc=1; }

cleanup() {
  local code=$?
  [ -n "$TMP" ] && rm -rf "$TMP"
  if [ "$finished" -eq 0 ]; then
    printf 'FAIL sha256sum-silicon: aborted before completion (exit %s) — rows above stand\n' "$code"
    exit 1
  fi
  exit "$rc"
}
trap cleanup EXIT

die() { bad "$1"; finished=1; exit 1; }

# ---------------------------------------------------------------- preflight --
[ -f examples/sha256sum/sha256sum_write.shard ] \
  || die "preflight: not at the repo root (examples/sha256sum/sha256sum_write.shard absent)"
command -v sha256sum >/dev/null 2>&1 \
  || die "preflight: coreutils sha256sum absent — every expectation in this leg comes from it"
command -v getcap >/dev/null 2>&1 \
  || die "preflight: getcap absent (libcap) — the capless assertion cannot be made"
command -v readelf >/dev/null 2>&1 \
  || die "preflight: readelf absent (binutils) — the vaddr-0 assertion cannot be made"
command -v dd >/dev/null 2>&1 || die "preflight: dd absent"
if [ -x bin/shard_eval ]; then EMIT=(bin/shard_eval)
elif [ -x ./rust_bootstrap/target/release/eval ]; then EMIT=(./rust_bootstrap/target/release/eval)
else die "preflight: no engine (bin/shard_eval or rust_bootstrap eval)"; fi
ok "preflight: repo root, coreutils sha256sum, getcap, readelf, engine"

TMP=$(mktemp -d) || die "preflight: mktemp failed"

STREAM=examples/sha256sum/sha256sum_stream
ONESHOT=examples/sha256sum/sha256sum

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
emit examples/sha256sum/sha256sum_stream_write.shard "$STREAM"  "emit-streaming-elf"
emit examples/sha256sum/sha256sum_write.shard        "$ONESHOT" "emit-oneshot-elf"

# ---------------------------------------------------------------- capless ----
# Each assertion is its own row: a binary that needs privilege is a different
# failure from a binary that hashes wrong, and CI should be able to tell them
# apart from the row name alone.
capless() { # $1 = binary, $2 = name
  local caps
  if [ ! -x "$1" ]; then bad "capless-$2: no binary to inspect"; return; fi
  caps=$(getcap "$1" 2>/dev/null)
  if [ -z "$caps" ]; then ok "capless-$2-getcap-empty"
  else bad "capless-$2-getcap-empty — carries [$caps]"; fi
  if readelf -l "$1" 2>/dev/null | grep -q '0x0000000000010000 0x0000000000010000'; then
    ok "capless-$2-rw-window-at-0x10000"
  else bad "capless-$2-rw-window-at-0x10000 — RW PT_LOAD is elsewhere"; fi
  if readelf -l "$1" 2>/dev/null \
     | grep -qE 'LOAD.*0x0000000000000000 0x0000000000000000 0x0000000000000000'; then
    bad "capless-$2-no-vaddr0-load — a PT_LOAD sits at vaddr 0 (the page-0 container is back)"
  else ok "capless-$2-no-vaddr0-load"; fi
}
capless "$STREAM"  "streaming"
capless "$ONESHOT" "oneshot"

# ------------------------------------------------------------- row helpers ---
# spipe FILE WRITER LABEL — WRITER writes FILE's bytes to stdout and is piped
# into the streaming bin; the expectation is read from FILE itself, so the
# writer is never run twice and need not be deterministic.
spipe() {
  local f="$1" writer="$2" lbl="$3" got code exp
  got=$(eval "$writer" | "$STREAM"); code=$?
  exp=$(sha256sum < "$f" | cut -d' ' -f1)
  if [ "$code" -ne 0 ]; then bad "$lbl — exit $code (expected 0)"; return; fi
  if [ "${#got}" -ne 64 ]; then bad "$lbl — wrote ${#got} chars, expected 64: [$got]"; return; fi
  if [ "$got" != "$exp" ]; then bad "$lbl — got $got expected $exp"; return; fi
  ok "$lbl"
}
# scat FILE LABEL — the plain `cat` pipe row
scat() { spipe "$1" "cat '$1'" "$2"; }

# oneshot FILE LABEL — file redirect, must hash
osum() {
  local f="$1" lbl="$2" got code exp
  got=$("$ONESHOT" < "$f"); code=$?
  exp=$(sha256sum < "$f" | cut -d' ' -f1)
  if [ "$code" -ne 0 ]; then bad "$lbl — exit $code (expected 0)"; return; fi
  if [ "$got" != "$exp" ]; then bad "$lbl — got $got expected $exp"; return; fi
  ok "$lbl"
}
# oneshot FILE LABEL — file redirect, must fail-stop with empty stdout
orej() {
  local f="$1" lbl="$2" out code
  out=$("$ONESHOT" < "$f"); code=$?
  if [ "$code" -eq 1 ] && [ -z "$out" ]; then ok "$lbl"
  else bad "$lbl — exit $code stdout=[$out] (expected exit 1, empty)"; fi
}

# ------------------------------------------- streaming: gold + NIST + strata --
printf ''    > "$TMP/empty"
printf 'abc' > "$TMP/abc"
printf 'abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq' > "$TMP/nist56"
scat "$TMP/empty"  "stream-gold-empty"
scat "$TMP/abc"    "stream-gold-abc"
scat "$TMP/nist56" "stream-gold-nist56"
for n in 55 56 63 64 65 127 128; do
  head -c "$n" /dev/urandom > "$TMP/s$n"
  scat "$TMP/s$n" "stream-stratum-${n}B"
done

# ------------------------------------------------ streaming: cap crossings ---
# 4096 is the model's per-read cap; these straddle it and its double.
for n in 4095 4096 4097 8192 8193; do
  head -c "$n" /dev/urandom > "$TMP/c$n"
  scat "$TMP/c$n" "stream-capcross-${n}B"
done

# ------------------------------------------------- streaming: short reads ----
# Every pipe read that returns fewer than the requested 4096 bytes is a short
# read; these rows force many per run.
head -c 300 /dev/urandom > "$TMP/b300"
spipe "$TMP/b300"  "dd if='$TMP/b300' bs=1 2>/dev/null"       "stream-shortread-dd-bs1-300B"
spipe "$TMP/c4097" "dd if='$TMP/c4097' bs=1 2>/dev/null"      "stream-shortread-dd-bs1-across-cap-4097B"
spipe "$TMP/c8193" "dd if='$TMP/c8193' bs=7 2>/dev/null"      "stream-shortread-dd-bs7-8193B"
spipe "$TMP/c8193" "dd if='$TMP/c8193' bs=4095 2>/dev/null"   "stream-shortread-dd-bs4095-8193B"

# PREFIX-STOP REFUTATION — the behavioural proof that the loop keeps reading
# after a short read, needing no tracer (this box has no strace and `perf trace`
# wants root). 40 bytes dripped with real gaps cannot be satisfied by one read
# of the 4096-byte request, so a bin that stopped at the first short read would
# digest a PREFIX. Asserted both ways: equals the whole input, matches no
# proper prefix.
printf 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX' > "$TMP/drip40"
dripped=$( { for i in $(seq 1 40); do printf 'X'; sleep 0.01; done; } | "$STREAM" )
whole=$(sha256sum < "$TMP/drip40" | cut -d' ' -f1)
pfx=""
for k in $(seq 1 39); do
  head -c "$k" "$TMP/drip40" > "$TMP/pfx"
  [ "$dripped" = "$(sha256sum < "$TMP/pfx" | cut -d' ' -f1)" ] && pfx="$k"
done
if [ "$dripped" = "$whole" ] && [ -z "$pfx" ]; then
  ok "stream-prefix-stop-refuted-40x1B-10ms-gaps"
else
  bad "stream-prefix-stop-refuted-40x1B-10ms-gaps — got $dripped whole $whole prefix-match-at ${pfx:-none}"
fi

# ------------------------------------------------------- streaming: bulk -----
head -c 1048576 /dev/urandom > "$TMP/m1"
scat  "$TMP/m1" "stream-bulk-1MiB"
spipe "$TMP/m1" "dd if='$TMP/m1' bs=997 2>/dev/null" "stream-bulk-1MiB-in-997B-chunks"

# ------------------------------------------------ streaming: random sweep ----
# Shapes are seeded so a failing row is REPRODUCIBLE (same size/chunk next run);
# the content is fresh /dev/urandom every run, which is what varies the digest.
# Size and chunk are in the row name so a FAIL is diagnosable without a re-run.
RANDOM=20260801
for i in $(seq 1 20); do
  sz=$(( (RANDOM * 32768 + RANDOM) % 20000 ))
  bs=$(( RANDOM % 4200 + 1 ))
  head -c "$sz" /dev/urandom > "$TMP/r"
  spipe "$TMP/r" "dd if='$TMP/r' bs=$bs 2>/dev/null" "stream-random size=$sz chunk=$bs"
done

# -------------------------------------------------- streaming: error legs ----
# A read error must fail-stop: exit 1, nothing on stdout (§7.2). `.` is the repo
# root, a directory, so read(2) answers EISDIR; `<&-` leaves fd 0 closed, EBADF.
out=$("$STREAM" < . 2>/dev/null); code=$?
if [ "$code" -eq 1 ] && [ -z "$out" ]; then ok "stream-errorleg-EISDIR-exit1-empty"
else bad "stream-errorleg-EISDIR-exit1-empty — exit $code stdout=[$out]"; fi
out=$("$STREAM" <&- 2>/dev/null); code=$?
if [ "$code" -eq 1 ] && [ -z "$out" ]; then ok "stream-errorleg-EBADF-exit1-empty"
else bad "stream-errorleg-EBADF-exit1-empty — exit $code stdout=[$out]"; fi

# ------------------------------------------- one-shot: file-redirect rows ----
osum "$TMP/empty"  "oneshot-gold-empty"
osum "$TMP/abc"    "oneshot-gold-abc"
osum "$TMP/nist56" "oneshot-gold-nist56"
osum "$TMP/c4097"  "oneshot-4097B"
head -c 50000 /dev/urandom > "$TMP/o50k"; osum "$TMP/o50k" "oneshot-50000B"
# the cap boundary, both sides
head -c 64887 /dev/urandom > "$TMP/omax"; osum "$TMP/omax" "oneshot-cap-64887B-last-accepted"
head -c 64888 /dev/urandom > "$TMP/ocap"; orej "$TMP/ocap" "oneshot-cap-64888B-first-rejected"
head -c 70000 /dev/urandom > "$TMP/oover"; orej "$TMP/oover" "oneshot-cap-70000B-over"

finished=1
