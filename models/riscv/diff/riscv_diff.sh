#!/usr/bin/env bash
# riscv_diff.sh — the RISC-V engine differential (docs/RISCV.md G2), end to
# end: the model-side plan emitter (riscv_diff_run.shard: real RV32I/RV64I
# machine-code bytes + model-computed expectations) executed by the engine-side
# replayer (riscv_diff.c) — a freestanding harness compiled for BOTH widths and
# run under qemu-user, which maps each module's bytes executable and CALLS
# them. Dev-side only; nothing here is in-logic. Run from the repo root. Exit 0
# = full agreement at both widths (the emulated core conforms to the model).
#
# There is no RISC-V libc/gcc/binutils on this box: clang cross-compiles, and
# the linker is rust-lld invoked through a symlink NAMED ld.lld (it flavors by
# argv[0]). Missing clang / qemu-user / rust-lld -> SKIP (exit 0), the
# node-guard / cc-guard discipline.
set -euo pipefail

RUSTLLD=/home/christian/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld

miss=""
command -v clang >/dev/null 2>&1 || miss="$miss clang"
command -v qemu-riscv64 >/dev/null 2>&1 || miss="$miss qemu-riscv64"
command -v qemu-riscv32 >/dev/null 2>&1 || miss="$miss qemu-riscv32"
LD_LLD="$(command -v ld.lld 2>/dev/null || true)"
if [ -z "$LD_LLD" ] && [ ! -x "$RUSTLLD" ]; then miss="$miss ld.lld/rust-lld"; fi
if [ -n "$miss" ]; then echo "SKIP riscv_diff (missing:$miss)"; exit 0; fi

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

# rust-lld flavors by argv[0]; give it the name ld.lld and pass the full path.
if [ -z "$LD_LLD" ]; then
  ln -sf "$RUSTLLD" "$TMP/ld.lld"
  LD_LLD="$TMP/ld.lld"
fi

if [ -x bin/shard_eval ]; then EMIT=(bin/shard_eval); else EMIT=(./rust_bootstrap/target/release/eval); fi
"${EMIT[@]}" run models/riscv/diff/riscv_diff_run.shard > "$TMP/plan.txt"

CFLAGS="-nostdlib -static -ffreestanding -fno-builtin -fno-stack-protector -Os -fuse-ld=$LD_LLD"
clang --target=riscv64-unknown-linux-gnu -march=rv64im $CFLAGS models/riscv/diff/riscv_diff.c -o "$TMP/rv64"
clang --target=riscv32-unknown-linux-gnu -march=rv32im $CFLAGS models/riscv/diff/riscv_diff.c -o "$TMP/rv32"

r64=0; r32=0
qemu-riscv64 "$TMP/rv64" "$TMP/plan.txt" 64 || r64=$?
qemu-riscv32 "$TMP/rv32" "$TMP/plan.txt" 32 || r32=$?
total=$(( r64 + r32 ))
echo "riscv engine differential: $total disagreement(s) [rv64 $r64, rv32 $r32]"
exit $total
