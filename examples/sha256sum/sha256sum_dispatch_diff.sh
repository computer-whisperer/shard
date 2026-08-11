#!/usr/bin/env bash
# examples/sha256sum/sha256sum_dispatch_diff.sh — THE SILICON LEG for the
# CPUID-DISPATCH sha256sum bin (docs/STREAM.md §9.2, rung B6 slice D). The
# proofs say what the merged machine module does; this says the CPU agrees, by
# running the emitted ELF against TWO independent oracles on real input. The
# "hardware conforms to the model" trust leaf for a whole program, where
# models/x86/diff/x86_diff.sh is the same leaf per XFunc,
# examples/sha256sum/sha256sum_silicon_diff.sh is that leaf for the scalar and
# one-shot bins, and examples/sha256sum/sha256sum_shani_diff.sh is that leaf for
# the SHA-NI variant. Dev-and-CI side; nothing here is in-logic.
#
# RUN FROM THE REPO ROOT (run_corpus.sh's differential legs all do):
#   bash examples/sha256sum/sha256sum_dispatch_diff.sh
#
# ⚠ THIS LEG RUNS EVERYWHERE — IT HAS NO SKIP, and that is the whole point of
# the rung. The SHA-NI twin's leg cannot run on a chip without the sha_ni flag:
# its product issues sha256rnds2 unconditionally and would SIGILL, so that
# script CPUID-gates itself and prints one loud SKIP line. THIS product asks
# the chip itself (the R4 two-step CPUID stub at fn 30) and calls the half the
# chip can execute, so there is no box where it cannot be adjudicated:
#   * on a box WITHOUT sha_ni the scalar half (fns 0-14, entry 9) runs;
#   * on a box WITH sha_ni the SHA-NI half (fns 15-29, entry 24) runs;
# and every digest row is double-oracled either way. The cpuid probe below is
# kept only to REPORT which half this box exercises — nothing is gated on it,
# and there is no force-no-sha override, because there is nothing to force.
# The proofs cover both arms (dsx_main_*_lo, dsx_main_*_hi), so a green run on
# either kind of box is evidence about a real arm; running it on both kinds is
# how the pair gets covered.
#
# OUTPUT CONTRACT (inherited verbatim from the twins). One `OK <row>` or
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
# THE DOUBLE ORACLE. Every digest row computes the expectation TWICE —
# coreutils `sha256sum` first field, and `openssl dgst -sha256 -r` first field —
# and passes only when bin == coreutils == openssl. An ORACLE DISAGREEMENT (the
# two references differ from each other) gets its own FAIL text: it is a
# different event from the bin disagreeing, it says nothing about our encoder,
# and it must never be read as one. Note that openssl itself may be using the
# same silicon SHA extensions, so the second oracle is independent SOFTWARE,
# not independent hardware — the hardware leaf is still coreutils' scalar path.
#
# EXPECTATIONS NEVER COME FROM PINNED HEX, so the leg cannot drift into agreeing
# with itself. Every input is materialized as a FILE first and both expectations
# are taken from that file; the pipe writers replay it. (An earlier scratch
# version of the scalar twin computed the expectation by re-running the writer,
# which silently hashed DIFFERENT random bytes for the two arms. Materialize
# once, read twice.)
#
# WHAT IT COVERS.
#   CAPLESS — the binary must load with no privilege: getcap EMPTY, RW PT_LOAD
#     at 0x10000 = stock vm.mmap_min_addr, and no PT_LOAD at vaddr 0. This is
#     the property that lets the leg run unprivileged at all.
#   BOTH FAMILIES ARE IN THE IMAGE — STATIC teeth, by disassembly, asserted on
#     every box regardless of which half this chip will run: exactly 32
#     sha256rnds2 / 12 sha256msg1 / 12 sha256msg2 (the SHA-NI half, fns 15-29)
#     and exactly 2 cpuid (the two-step stub's leaf-0 and leaf-7 probes). Count-
#     exact, so a half that silently vanished from the merge — the way this leg
#     could otherwise pass VACUOUSLY on a scalar-only box — is a FAIL.
#   THE STREAMING BIN — PIPE-FED, always, because short reads are the thing
#     under test: it loops read(≤61440) until EOF. Gold vectors, the 55..128
#     block strata, the 61440 read-cap crossings, a sub-cap sweep, byte-at-a-time
#     and small-chunk writers, a prefix-stop refutation, bulk, a seeded random
#     sweep, and both read-error legs. There is no one-shot bin in this variant.
#   CROSS-CHECK against the separately-proven twins' own binaries: the scalar
#     streaming bin everywhere, the SHA-NI bin additionally where the chip can
#     run it. Extra rows, never a gate — the double oracle above is the gate.
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
    printf 'FAIL sha256sum-dispatch-silicon: aborted before completion (exit %s) — rows above stand\n' "$code"
    exit 1
  fi
  exit "$rc"
}
trap cleanup EXIT

die() { bad "$1"; finished=1; exit 1; }

# ---------------------------------------------------------------- preflight --
[ -f examples/sha256sum/sha256sum_dispatch_write.shard ] \
  || die "preflight: not at the repo root (examples/sha256sum/sha256sum_dispatch_write.shard absent)"
command -v sha256sum >/dev/null 2>&1 \
  || die "preflight: coreutils sha256sum absent — it is oracle #1 of this leg's double oracle"
command -v openssl >/dev/null 2>&1 \
  || die "preflight: openssl absent — it is oracle #2 of this leg's double oracle"
command -v getcap >/dev/null 2>&1 \
  || die "preflight: getcap absent (libcap) — the capless assertion cannot be made"
command -v readelf >/dev/null 2>&1 \
  || die "preflight: readelf absent (binutils) — the vaddr-0 assertion cannot be made"
command -v objdump >/dev/null 2>&1 \
  || die "preflight: objdump absent (binutils) — the both-families teeth cannot be made"
command -v dd >/dev/null 2>&1 || die "preflight: dd absent"
if [ -x bin/shard_eval ]; then EMIT=(bin/shard_eval)
elif [ -x ./rust_bootstrap/target/release/eval ]; then EMIT=(./rust_bootstrap/target/release/eval)
else die "preflight: no engine (bin/shard_eval or rust_bootstrap eval)"; fi
ok "preflight: repo root, coreutils sha256sum, openssl, getcap, readelf, objdump, engine"

# ------------------------------------------------------------ path report ----
# REPORT ONLY, gating NOTHING (see the header): which half of the merged module
# this box's own CPUID answer will select, so a reader of the log knows which
# arm of the proof pair this run adjudicated.
if grep -qw sha_ni /proc/cpuinfo 2>/dev/null; then BOXPATH=sha-ni; else BOXPATH=scalar; fi
ok "dispatch-path-report: dispatch path on this box: $BOXPATH (report only — no row is gated on it)"

TMP=$(mktemp -d) || die "preflight: mktemp failed"

DISPATCH=examples/sha256sum/sha256sum

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
emit examples/sha256sum/sha256sum_dispatch_write.shard "$DISPATCH" "dispatch-emit-elf"

# ---------------------------------------------------------------- capless ----
# Each assertion is its own row: a binary that needs privilege is a different
# failure from a binary that hashes wrong, and CI should be able to tell them
# apart from the row name alone.
capless() { # $1 = binary, $2 = name
  local caps
  if [ ! -x "$1" ]; then bad "dispatch-capless-$2: no binary to inspect"; return; fi
  caps=$(getcap "$1" 2>/dev/null)
  if [ -z "$caps" ]; then ok "dispatch-capless-$2-getcap-empty"
  else bad "dispatch-capless-$2-getcap-empty — carries [$caps]"; fi
  if readelf -l "$1" 2>/dev/null | grep -q '0x0000000000010000 0x0000000000010000'; then
    ok "dispatch-capless-$2-rw-window-at-0x10000"
  else bad "dispatch-capless-$2-rw-window-at-0x10000 — RW PT_LOAD is elsewhere"; fi
  if readelf -l "$1" 2>/dev/null \
     | grep -qE 'LOAD.*0x0000000000000000 0x0000000000000000 0x0000000000000000'; then
    bad "dispatch-capless-$2-no-vaddr0-load — a PT_LOAD sits at vaddr 0 (the page-0 container is back)"
  else ok "dispatch-capless-$2-no-vaddr0-load"; fi
}
capless "$DISPATCH" "merged"

# --------------------------------------------------- both-families teeth -----
# The image must carry BOTH halves on EVERY box, and the stub must carry BOTH
# cpuid steps. Disassembly starts at the entry's file offset, computed from the
# headers rather than pinned: the R E segment maps file offset 0 at 0x400000, so
# entry-file-offset = entry-vaddr - 0x400000 (this skips the ELF header bytes,
# which are inside that segment and would otherwise be decoded as instructions).
teeth() { # $1 = binary
  local ent off dis n
  if [ ! -x "$1" ]; then bad "dispatch-teeth: no binary to disassemble"; return; fi
  ent=$(readelf -h "$1" 2>/dev/null | sed -n 's/.*Entry point address: *//p')
  if [ -z "$ent" ]; then bad "dispatch-teeth: could not read the entry point"; return; fi
  off=$(( ent - 0x400000 ))
  if [ "$off" -le 0 ]; then bad "dispatch-teeth: entry $ent is not inside the 0x400000 text segment"; return; fi
  dis="$TMP/dis.txt"
  objdump -D -b binary -m i386:x86-64 --start-address="$off" "$1" > "$dis" 2>/dev/null \
    || { bad "dispatch-teeth: objdump failed"; return; }
  tooth() { # $1 = mnemonic, $2 = exact count, $3 = what it witnesses
    local n
    n=$(grep -cwE "$1" "$dis")
    if [ "$n" -eq "$2" ]; then ok "dispatch-teeth-$1-x$2 ($3)"
    else bad "dispatch-teeth-$1-x$2 — found $n ($3)"; fi
  }
  tooth sha256rnds2 32 "the SHA-NI half is in the image on every box"
  tooth sha256msg1  12 "the SHA-NI half is in the image on every box"
  tooth sha256msg2  12 "the SHA-NI half is in the image on every box"
  tooth cpuid        2 "the R4 two-step probe: leaf 0 then leaf 7"
}
teeth "$DISPATCH"

# ------------------------------------------------------------- row helpers ---
# spipe FILE WRITER LABEL — WRITER writes FILE's bytes to stdout and is piped
# into the dispatch bin; both expectations are read from FILE itself, so the
# writer is never run twice and need not be deterministic.
spipe() {
  local f="$1" writer="$2" lbl="$3" got code
  got=$(eval "$writer" | "$DISPATCH"); code=$?
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
scat "$TMP/empty"  "dispatch-gold-empty"
scat "$TMP/abc"    "dispatch-gold-abc"
scat "$TMP/nist56" "dispatch-gold-nist56"
for n in 55 56 63 64 65 127 128; do
  head -c "$n" /dev/urandom > "$TMP/s$n"
  scat "$TMP/s$n" "dispatch-stratum-${n}B"
done

# ------------------------------------------------ streaming: cap crossings ---
# 61440 is the model's per-read cap; these straddle it and its double.
for n in 61439 61440 61441 122880 122881; do
  head -c "$n" /dev/urandom > "$TMP/c$n"
  scat "$TMP/c$n" "dispatch-capcross-${n}B"
done
# The OLD (4096) proving-rung cap's boundaries, kept as plain sub-cap rows:
# every one of them now fits in a single read, so they exercise the
# one-read-then-EOF shape rather than a crossing. c4097 / c8193 are also the
# inputs the short-read rows below replay.
for n in 4095 4096 4097 8192 8193; do
  head -c "$n" /dev/urandom > "$TMP/c$n"
  scat "$TMP/c$n" "dispatch-subcap-${n}B"
done

# ------------------------------------------------- streaming: short reads ----
# Every pipe read that returns fewer than the requested 61440 bytes is a short
# read; these rows force many per run. (A pipe's own 64 KiB buffer means the
# plain `cat` rows above are already short-read rows at this cap.)
head -c 300 /dev/urandom > "$TMP/b300"
spipe "$TMP/b300"   "dd if='$TMP/b300' bs=1 2>/dev/null"      "dispatch-shortread-dd-bs1-300B"
spipe "$TMP/c4097"  "dd if='$TMP/c4097' bs=1 2>/dev/null"     "dispatch-shortread-dd-bs1-4097B"
spipe "$TMP/c8193"  "dd if='$TMP/c8193' bs=7 2>/dev/null"     "dispatch-shortread-dd-bs7-8193B"
spipe "$TMP/c8193"  "dd if='$TMP/c8193' bs=4095 2>/dev/null"  "dispatch-shortread-dd-bs4095-8193B"
spipe "$TMP/c61441" "dd if='$TMP/c61441' bs=4095 2>/dev/null" "dispatch-shortread-dd-bs4095-across-cap-61441B"

# PREFIX-STOP REFUTATION — the behavioural proof that the loop keeps reading
# after a short read, needing no tracer (this box has no strace and `perf trace`
# wants root). 40 bytes dripped with real gaps cannot be satisfied by one read
# of the 61440-byte request, so a bin that stopped at the first short read would
# digest a PREFIX. Asserted both ways: equals the whole input, matches no
# proper prefix.
printf 'XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX' > "$TMP/drip40"
dripped=$( { for i in $(seq 1 40); do printf 'X'; sleep 0.01; done; } | "$DISPATCH" )
oracles "$TMP/drip40"
whole="$EXP_CU"
if [ "$EXP_CU" != "$EXP_SSL" ]; then
  bad "dispatch-prefix-stop-refuted-40x1B-10ms-gaps — ORACLE DISAGREEMENT: coreutils $EXP_CU vs openssl $EXP_SSL"
else
  pfx=""
  for k in $(seq 1 39); do
    head -c "$k" "$TMP/drip40" > "$TMP/pfx"
    [ "$dripped" = "$(sha256sum < "$TMP/pfx" | cut -d' ' -f1)" ] && pfx="$k"
  done
  if [ "$dripped" = "$whole" ] && [ -z "$pfx" ]; then
    ok "dispatch-prefix-stop-refuted-40x1B-10ms-gaps"
  else
    bad "dispatch-prefix-stop-refuted-40x1B-10ms-gaps — got $dripped whole $whole prefix-match-at ${pfx:-none}"
  fi
fi

# ------------------------------------------------------- streaming: bulk -----
head -c 1048576 /dev/urandom > "$TMP/m1"
scat  "$TMP/m1" "dispatch-bulk-1MiB"
spipe "$TMP/m1" "dd if='$TMP/m1' bs=997 2>/dev/null" "dispatch-bulk-1MiB-in-997B-chunks"

# ------------------------------------------------ streaming: random sweep ----
# Shapes are seeded so a failing row is REPRODUCIBLE (same size/chunk next run);
# the content is fresh /dev/urandom every run, which is what varies the digest.
# Size and chunk are in the row name so a FAIL is diagnosable without a re-run.
# The size bound is ~3x the read cap so the sweep keeps straddling it; the chunk
# bound stays sub-cap, which is what makes every write a short read.
RANDOM=20260810
for i in $(seq 1 20); do
  sz=$(( (RANDOM * 32768 + RANDOM) % 200000 ))
  bs=$(( RANDOM % 4200 + 1 ))
  head -c "$sz" /dev/urandom > "$TMP/r"
  spipe "$TMP/r" "dd if='$TMP/r' bs=$bs 2>/dev/null" "dispatch-random size=$sz chunk=$bs"
done

# -------------------------------------------------- streaming: error legs ----
# A read error must fail-stop: exit 1, nothing on stdout (§7.2). `.` is the repo
# root, a directory, so read(2) answers EISDIR; `<&-` leaves fd 0 closed, EBADF.
out=$("$DISPATCH" < . 2>/dev/null); code=$?
if [ "$code" -eq 1 ] && [ -z "$out" ]; then ok "dispatch-errorleg-EISDIR-exit1-empty"
else bad "dispatch-errorleg-EISDIR-exit1-empty — exit $code stdout=[$out]"; fi
out=$("$DISPATCH" <&- 2>/dev/null); code=$?
if [ "$code" -eq 1 ] && [ -z "$out" ]; then ok "dispatch-errorleg-EBADF-exit1-empty"
else bad "dispatch-errorleg-EBADF-exit1-empty — exit $code stdout=[$out]"; fi

# ------------------------------------------------------------ cross-check ----
# The dispatch bin against the twins' OWN binaries, emitted fresh here so the
# row never depends on another leg having run first. Extra evidence, not the
# gate: the double-oracle rows above are the gate. The scalar twin runs on any
# box; the SHA-NI twin issues sha256rnds2 unconditionally, so its row is made
# only where the chip can execute it (this box: $BOXPATH).
cross() { # $1 = other binary, $2 = row label
  local got_o got_d
  if [ ! -x "$1" ]; then bad "$2 — no twin binary to compare against"; return; fi
  got_o=$(printf 'abc' | "$1"); got_d=$(printf 'abc' | "$DISPATCH")
  if [ "$got_o" = "$got_d" ] && [ "${#got_d}" -eq 64 ]; then ok "$2"
  else bad "$2 — twin said [$got_o], dispatch said [$got_d]"; fi
}
emit examples/sha256sum/sha256sum_stream_write.shard \
     examples/sha256sum/sha256sum_stream "dispatch-cross-emit-scalar-twin"
cross examples/sha256sum/sha256sum_stream "dispatch-cross-scalar-twin-abc"
if [ "$BOXPATH" = "sha-ni" ]; then
  emit examples/sha256sum/sha256sum_shani_write.shard \
       examples/sha256sum/sha256sum_shani "dispatch-cross-emit-shani-twin"
  cross examples/sha256sum/sha256sum_shani "dispatch-cross-shani-twin-abc"
fi

finished=1
