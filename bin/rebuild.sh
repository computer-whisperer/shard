#!/usr/bin/env bash
# Rebuild the native engine from kernel sources via the temporary compiler
# chain: tools/lower (shard -> RS-shard) -> tools/codegen (RS -> C) -> cc.
#
# ENGINE SELECTION (2026-07-10): the chain runs on the PREVIOUS compiled
# engine when one exists — gen-N builds gen-N+1, minutes instead of ~1h —
# with the Rust interpreter kept as the bootstrap authority:
#   TRUSTED=1 bin/rebuild.sh   forces the full Rust-interpreter build
#   (and a missing bin/shard_eval falls back to Rust automatically)
# The trusting-trust worry ("a wrong binary propagates itself") is answered
# by GATES, not by the builder:
#   - the chain is DETERMINISTIC, and every fast build byte-ties a fixture's
#     lower+codegen output against the Rust interpreter before building —
#     a self-reproducing miscompile dies here, at its first generation;
#   - the corpus FAIL-set diff gates every rebuild behaviorally;
#   - soundness-authority runs use EVAL=rust explicitly (run_corpus.sh);
#     the compiled chain is never the soundness authority.
# The boot engine is SNAPSHOTTED before building so the chain never runs on
# a binary it is itself replacing, and a STALE previous engine is fine by
# construction: it is gen-N building gen-N+1's sources; if the new sources
# outgrow it, the loud-stuckness guard / empty-output checks / cc refuse.
#
# The generated artifacts (SRC.low.shard, SRC.low.shard.c, the binary) are
# derived files and stay untracked; this script is the only build path.
set -euo pipefail
cd "$(dirname "$0")/.."
RUST_EVAL=./rust_bootstrap/target/release/eval
[ -x "$RUST_EVAL" ] || { echo "missing $RUST_EVAL (cargo build --release in rust_bootstrap/)"; exit 1; }

BOOT="$RUST_EVAL"
MODE=trusted
if [ "${TRUSTED:-}" != 1 ] && [ -x bin/shard_eval ]; then
  cp bin/shard_eval bin/.shard_eval.boot
  BOOT=bin/.shard_eval.boot
  MODE="fast (boot engine: previous bin/shard_eval, stamp $(cat bin/shard_eval.stamp 2>/dev/null | cut -c1-12))"
fi
trap 'rm -f bin/.shard_eval.boot' EXIT
echo "== build mode: $MODE"

# Refuse to build (and stamp) from non-canonical sources: the stamp hashes
# raw bytes, so a later formatting-only pass over a stamp input would flip
# the stamp without changing engine behavior, permanently flagging the fresh
# binary STALE (seen 2026-07-06: post-rebuild shardfmt of codegen.shard).
# shardfmt --check: exit 0 = already canonical, 1 = a reformat would change
# bytes. rt.h is a stamp input too but not a .shard file; it has no
# canonical form to drift from.
stamp_inputs() {
  ls kernel/*.shard | grep -v '\.low\.shard$'
  echo tools/lower/lower.shard
  echo tools/codegen/codegen.shard
}
echo "== fmt gate: stamp inputs canonical?"
drift=$(stamp_inputs | xargs -P 8 -I{} sh -c \
  '"$1" run tools/shardfmt/shardfmt.shard --check "{}" >/dev/null 2>&1 || echo "{}"' \
  _ "$BOOT")
if [ -n "$drift" ]; then
  echo "REFUSE: stamp inputs are not shardfmt-canonical; format these first:" >&2
  echo "$drift" >&2
  exit 1
fi

# The BYTE-TIE (fast mode only): lower+codegen one small World app on BOTH
# engines and require byte-identical artifacts. Determinism makes equality
# the expected outcome; any divergence is a miscompile in the boot engine
# and the build refuses before it can propagate.
TIE_SRC=examples/tie_probe.shard
if [ "$MODE" != trusted ]; then
  echo "== byte-tie: $TIE_SRC on boot engine vs Rust interpreter"
  TIE=$(mktemp -d)
  trap 'rm -f bin/.shard_eval.boot; rm -rf "$TIE"' EXIT
  "$BOOT"      run tools/lower/lower.shard "$TIE_SRC" "$TIE/a.low" >/dev/null
  "$RUST_EVAL" run tools/lower/lower.shard "$TIE_SRC" "$TIE/b.low" >/dev/null
  cmp -s "$TIE/a.low" "$TIE/b.low" || { echo "REFUSE: lower byte-tie FAILED — boot engine diverges from Rust; rerun with TRUSTED=1"; exit 1; }
  "$BOOT"      run tools/codegen/codegen.shard "$TIE/a.low" "$TIE/a.c" >/dev/null
  "$RUST_EVAL" run tools/codegen/codegen.shard "$TIE/a.low" "$TIE/b.c" >/dev/null
  cmp -s "$TIE/a.c" "$TIE/b.c" || { echo "REFUSE: codegen byte-tie FAILED — boot engine diverges from Rust; rerun with TRUSTED=1"; exit 1; }
  echo "   byte-tie OK (lower + codegen identical)"
fi

build() {
  local src=$1 out=$2
  local low=$src.low.shard
  echo "== lower   $src"
  "$BOOT" run tools/lower/lower.shard "$src" "$low"
  [ -s "$low" ] || { echo "FAIL: $low is empty"; exit 1; }
  echo "== codegen $low"
  "$BOOT" run tools/codegen/codegen.shard "$low" "$low.c"
  [ -s "$low.c" ] || { echo "FAIL: $low.c is empty"; exit 1; }
  echo "== cc      -> $out"
  cc -O2 -o "$out" "$low.c" -I tools/codegen
}

build kernel/eval.shard bin/shard_eval
bin/engine_stamp.sh > bin/shard_eval.stamp
echo "OK: bin/shard_eval (stamp $(cat bin/shard_eval.stamp | cut -c1-12))"

# `bin/rebuild.sh check` additionally builds the DIRECT-compiled checker
# (gate_sweep's fastest engine: ~0.2s/155MB vs ~minutes/40GB interpreting).
if [ "${1:-}" = check ]; then
  build kernel/check.shard bin/shard_check
  bin/engine_stamp.sh > bin/shard_check.stamp
  echo "OK: bin/shard_check (stamp $(cat bin/shard_check.stamp | cut -c1-12))"
fi

# Live guard: run-mode stuckness and malformed extern args must die LOUDLY
# (exit 4 + the offending head), never exit 0 — see examples/run_stuckctl.sh.
./examples/run_stuckctl.sh || { echo "REFUSE: stuckness guard failed on the fresh engine"; exit 1; }
