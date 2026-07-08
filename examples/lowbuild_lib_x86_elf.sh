#!/usr/bin/env bash
# examples/lowbuild_lib_x86_elf.sh SRC OUT — the ENGINE-AS-A-PROCESS gate
# (docs/X86.md §20, the lib-build arc): a (lib …) source lowered by x86gen
# LIB mode is packaged into a from-scratch static ELF64 executable (no libc,
# no C harness) that RUNS ON REAL LINUX, calling each proven export on the
# synthesized vectors and sys_writing its results. The gate:
#   1. REGEN   — x86gen LIB mode output is byte-identical to OUT (producer
#                determinism; the full schema/kernel/accepts cert gates are
#                examples/lowbuild_lib_x86.sh, over the same OUT)
#   2. IMGTIE  — the image the ELF embeds (LNAME_elf_image) equals the
#                index-order image ASSEMBLED FROM THE CERTS (bytetie TIEIMG):
#                the ELF runs bytes tied to the proofs, not producer output
#   3. ENGINE  — the ELF, executed as a process, writes exactly the plan's
#                expected results (spec fns invoked in the OUT closure): the
#                translation validation reality-check, on silicon, in-process
# Exit 0 = the proven lib runs correctly as a native Linux binary. Run from
# the repo root.
set -euo pipefail
[ $# -eq 2 ] || { echo "usage: lowbuild_lib_x86_elf.sh SRC OUT"; exit 2; }
SRC=$1
OUT=$2
EVAL=${EVAL:-bin/shard_eval}
command -v xxd >/dev/null || { echo "REFUSED: no xxd — the ENGINE gate cannot run"; exit 1; }
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT

echo "== gate 1: regen (producer determinism)"
"$EVAL" run tools/x86gen/x86gen.shard "$SRC" "$TMP/out.raw" >/dev/null
"$EVAL" run tools/shardfmt/shardfmt.shard "$TMP/out.raw" > "$TMP/out.fmt"
diff -q "$TMP/out.fmt" "$OUT"
echo "REGEN OK (byte-identical)"

echo "== gate 2: image cert-tie (the ELF runs bytes assembled from the certs)"
"$EVAL" run tools/lowbuild/lowbuild.shard elf "$SRC" "$OUT" > "$TMP/elf.txt"
"$EVAL" run tools/bytetie/bytetie.shard "$OUT" > "$TMP/tie.txt"
IMG=$(grep '^IMG ' "$TMP/elf.txt" | cut -d' ' -f2)
TIEIMG=$(grep '^TIEIMG ' "$TMP/tie.txt" | cut -d' ' -f2)
[ -n "$IMG" ]
[ -n "$TIEIMG" ]
[ "$IMG" = "$TIEIMG" ]
echo "IMGTIE OK (embedded image == cert-assembled image)"

echo "== gate 3: engine (the ELF runs on real Linux, stdout == spec results)"
grep '^ELF ' "$TMP/elf.txt" | cut -d' ' -f2 | xxd -r -p > "$TMP/a.elf"
chmod +x "$TMP/a.elf"
GOT=$("$TMP/a.elf" | xxd -p | tr -d '\n')
EXP=$(grep '^EXPOUT ' "$TMP/elf.txt" | cut -d' ' -f2)
if [ "$GOT" != "$EXP" ]; then
  echo "ENGINE MISMATCH:"
  echo "  got: $GOT"
  echo "  exp: $EXP"
  grep '^EXP ' "$TMP/elf.txt"
  exit 1
fi
NV=$(grep -c '^EXP ' "$TMP/elf.txt")
echo "ELF ENGINE OK ($NV vectors agree on silicon)"

echo "ARTIFACT OK: $(basename "$SRC" .shard) — a native Linux binary, proven exports, three gates green"
