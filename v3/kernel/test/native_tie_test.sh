#!/usr/bin/env bash
# v3/kernel/test/native_tie_test.sh — the byte-tie gate of the native toolchain
# (v3/LANGUAGE.md §8.6 slice 4.1 rule 3; R84, records §4.12): a fixed set of the suite's
# workloads run under both engines — the Rust bootstrap (`eval direct`, route 3, the
# authority) and the binaries route 1 built (v3/build.sh, kernel/emit.shard) — and compared
# on four things: stdout, stderr, the exit status and every artifact written (a sidecar, a
# store's files, an index), byte for byte. Both runs use the same scratch paths (the first
# engine's artifacts are set aside before the second runs), so a path printed is the same
# text in both logs. A divergence is a miscompile or a semantic gap between the bootstrap
# and the chain and fails the gate: the binary is never patched — the emitter or the chain
# is fixed and the build rerun.
#
# THE NORMALIZATION, in full: none. No workload below prints a time or a size; a
# scratch root is the same path for both engines. A verdict, an assumption closure, a
# fingerprint, a record are never normalized.
#
# THE WORKLOADS: (0) the gate's own test — an altered verdict, a missing binary, each
# detected; (1) every binary of the inventory present with a current stamp (a missing or
# stale binary fails: never a silent fallback to the bootstrap); (2) the discriminating
# cases v3/pins/emit/* built fresh and tied, the refused_* cases refused with their first
# line's message; (3) the loader over pins' cases (a clean load, a refusal, a read error, a
# view), a dump, an example, a program run under ev with and without fuel, a store written;
# (4) prove regenerating two examples' sidecars and a measure run; (5) verify_release over
# the tie's store and a store without a manifest; (6) t0 over the export's first 3,000
# lines with -v -a -p, the usage line, the wrong-pin fixture; (7) initindex building and
# verifying an index over the same lines; (8) one harness, loader_pins_test, over every
# loader pin; (9) the tree's authored files byte-identical before and after. Exit = the
# checks that failed. Run after `v3/build.sh --all`; v3/test.sh runs it under V3_NATIVE=1.
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "native_tie_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
DIR=${V3_EXPORT_DIR:-.shard-cache/v3-export}
OUT=$(mktemp -d)
SCRATCH=$(mktemp -d -p v3 native_tie_XXXXXX)
trap 'rm -rf "$OUT" "$SCRATCH"' EXIT
fail=0; checks=0; s0=$(date +%s)
bad() { echo "native_tie_test: $*"; fail=$((fail+1)); }
# the authored tree: every tracked file under v3, tools and kernel unmodified, and no new file under
# the authored directories (the suite's other entrypoints keep their scratch roots directly under v3/)
tree_state() { { git status --porcelain --untracked-files=no -- v3 tools kernel; git status --porcelain --untracked-files=all -- v3/kernel v3/pins v3/examples v3/std tools kernel; } 2>/dev/null | grep -v "native_tie_" | sort -u; }
before=$(tree_state)

# ---- the comparison: two result directories (out, err, rc, artifacts/…) byte for byte
compare() {   # LABEL DIR_A DIR_B -> 0 identical
  local l=$1 a=$2 b=$3 d
  for f in rc out err; do [ -f "$a/$f" ] && [ -f "$b/$f" ] || { echo "   $l: no $f on one side"; return 1; }; done
  [ "$(cat "$a/rc")" = "$(cat "$b/rc")" ] || { echo "   $l: exit $(cat "$a/rc") against $(cat "$b/rc")"; return 1; }
  cmp -s "$a/out" "$b/out" || { echo "   $l: stdout differs"; diff "$a/out" "$b/out" | head -4 | cut -c1-200; return 1; }
  cmp -s "$a/err" "$b/err" || { echo "   $l: stderr differs"; diff "$a/err" "$b/err" | head -4 | cut -c1-200; return 1; }
  if [ -d "$a/artifacts" ] || [ -d "$b/artifacts" ]; then
    d=$(diff -r "$a/artifacts" "$b/artifacts" 2>&1 | head -3) || true
    [ -z "$d" ] || { echo "   $l: artifacts differ"; echo "$d" | cut -c1-200; return 1; }
  fi
  return 0
}
# set_aside DIR: the artifact ARTIFACT names moved under DIR/artifacts and its place left as the
# run found it (a directory artifact, a store, recreated empty for the next engine)
set_aside() {
  [ -n "${ARTIFACT:-}" ] || return 0
  mkdir -p "$1/artifacts"; cp -r "$ARTIFACT" "$1/artifacts/" 2>/dev/null
  if [ -d "$ARTIFACT" ]; then rm -rf "$ARTIFACT"; mkdir -p "$ARTIFACT"; else rm -f "$ARTIFACT"; fi
}
# run_into DIR CMD…: the command's streams and status under DIR
run_into() { local d=$1; shift; mkdir -p "$d"; "$@" > "$d/out" 2> "$d/err"; echo $? > "$d/rc"; }
# tie LABEL NAME ARGS…: v3/kernel/NAME.shard under the bootstrap and v3/bin/$(basename NAME)
# over ARGS, compared; a verdict line required on both sides when VERDICT is set
tie() {
  local l=$1 n=$2 bin; shift 2
  bin=v3/bin/$(basename "$n")
  checks=$((checks+1))
  run_into "$OUT/$l/a" "$EVAL" direct "v3/kernel/$n.shard" "$@"
  set_aside "$OUT/$l/a"
  [ -x "$bin" ] || { bad "$l: $bin is missing (v3/build.sh --all)"; return; }
  run_into "$OUT/$l/b" "$bin" "$@"
  set_aside "$OUT/$l/b"
  if [ -n "${VERDICT:-}" ]; then
    for side in a b; do grep -q "$VERDICT" "$OUT/$l/$side/out" || { bad "$l: no verdict line ($VERDICT) on side $side"; return; }; done
  fi
  compare "$l" "$OUT/$l/a" "$OUT/$l/b" && echo "   tie $l: exit $(cat "$OUT/$l/b/rc"), $(wc -l < "$OUT/$l/b/out") lines" || bad "$l differs"
}

# ---- (0) the gate's own test (t0_gate_test.sh's shape): the compare must detect an altered
# verdict and a binary that is not there
mkdir -p "$OUT/self/a" "$OUT/self/b"
printf 'T0: accepted 1  rejected 0\n' > "$OUT/self/a/out"; : > "$OUT/self/a/err"; echo 0 > "$OUT/self/a/rc"; cp "$OUT/self/a/"* "$OUT/self/b/"
checks=$((checks+1)); compare self-identical "$OUT/self/a" "$OUT/self/b" > /dev/null || bad "the gate rejects identical results"
printf 'T0: accepted 1  rejected 1\n' > "$OUT/self/b/out"
checks=$((checks+1)); compare self-altered "$OUT/self/a" "$OUT/self/b" > /dev/null && bad "the gate accepts an altered verdict"
cp "$OUT/self/a/out" "$OUT/self/b/out"; echo 1 > "$OUT/self/b/rc"
checks=$((checks+1)); compare self-exit "$OUT/self/a" "$OUT/self/b" > /dev/null && bad "the gate accepts a differing exit status"
mkdir -p "$OUT/self/a/artifacts" "$OUT/self/b/artifacts"; echo x > "$OUT/self/a/artifacts/f"; echo y > "$OUT/self/b/artifacts/f"; echo 0 > "$OUT/self/b/rc"
checks=$((checks+1)); compare self-artifact "$OUT/self/a" "$OUT/self/b" > /dev/null && bad "the gate accepts a differing artifact"
checks=$((checks+1)); f0=$fail; ( fail=0; tie self-missing no_such_binary_for_the_tie > /dev/null 2>&1; exit $fail ) && bad "a missing binary passes the tie"
echo "native_tie_test: the gate detects an altered verdict, a differing exit, a differing artifact and a missing binary"

# ---- (1) every binary of the inventory present and current
n_inv=0
while read -r name src; do
  n_inv=$((n_inv+1)); checks=$((checks+1))
  st=$(v3/build.sh --stamp "$name")
  if [ ! -x "v3/bin/$name" ]; then bad "v3/bin/$name is missing (v3/build.sh --all)"
  elif [ -z "$st" ] || [ "$(cat "v3/bin/$name.stamp" 2>/dev/null)" != "$st" ]; then bad "v3/bin/$name is stale (v3/build.sh --all)"; fi
done < <(v3/build.sh --list)
echo "native_tie_test: $n_inv binaries of the inventory present and current"

# ---- (2) the discriminating cases: each built fresh through route 1, run under both engines
for d in v3/pins/emit/*/; do
  c=$(basename "$d"); extra=""; [ -d "$d/kernel/lib" ] && extra="$d/kernel/lib"
  checks=$((checks+1))
  if v3/build.sh "$d/kernel/main.shard" "$OUT/bin_$c" --root "$d" $extra > "$OUT/build_$c.txt" 2>&1; then
    case $c in refused_*) bad "$c: built, but the case expects the emit refused"; continue;; esac
    run_into "$OUT/pin_$c/a" "$EVAL" direct "$d/kernel/main.shard"
    run_into "$OUT/pin_$c/b" "$OUT/bin_$c"
    compare "pin $c" "$OUT/pin_$c/a" "$OUT/pin_$c/b" || bad "pin $c differs"
  else
    case $c in
      refused_*) msg=$(sed -n '1s/^;; emit refused: //p' "$d/kernel/main.shard"); grep -qF "EMIT: $msg" "$OUT/build_$c.txt" || { bad "$c: not refused with its message"; grep -E 'EMIT|FAIL' "$OUT/build_$c.txt" | head -2 | cut -c1-200; };;
      *) bad "$c: the build failed"; grep -E 'EMIT|FAIL|REFUSE' "$OUT/build_$c.txt" | head -2 | cut -c1-200;;
    esac
  fi
done
echo "native_tie_test: the discriminating cases of v3/pins/emit tied, the refused ones refused"

# ---- (3) the loader
for c in basic refused read_error; do tie "load pin $c" load --root v3/pins/loader/$c $INIT v3/pins/loader/$c/main.shard; done
tie "load view" load --root v3/pins/loader/view_basic $INIT v3/pins/loader/view_basic/main.shard v3/pins/loader/view_basic/lib
tie "load dump" load --root v3 $INIT --dump v3/examples/auto/auto.shard
tie "load calc_spec" load --root v3 $INIT v3/examples/calc/calc_spec.shard
head -n 300 "$DIR/init.ndjson" > "$SCRATCH/fix300.ndjson"
VERDICT='^T0: ' tie "load run t0" load --root v3 --run kernel.t0.main v3/kernel/t0.shard v3/kernel/k -- -a "$SCRATCH/fix300.ndjson"
VERDICT='^RUN: out of fuel' tie "load run fuel" load --root v3 --fuel 20000 --run kernel.t0.main v3/kernel/t0.shard v3/kernel/k -- -a "$SCRATCH/fix300.ndjson"
mkdir -p "$SCRATCH/store"
ARTIFACT=$SCRATCH/store VERDICT='^STORE: ' tie "load store" load --root v3 $INIT --store "$SCRATCH/store" v3/examples/auto/auto.shard
echo "native_tie_test: the loader tied over four pins, a dump, an example, a run with and without fuel, a store"

# ---- (4) prove: the examples copied without their sidecars, the sidecars regenerated
for ex in auto tactic; do
  mkdir -p "$SCRATCH/$ex"; for f in v3/examples/$ex/*.shard; do case $f in *.auto.shard) ;; *) cp "$f" "$SCRATCH/$ex/";; esac; done
done
ARTIFACT=$SCRATCH/auto/auto.auto.shard VERDICT='^PROVE: ' tie "prove auto" prove --root v3 $INIT "$SCRATCH/auto/auto.shard"
ARTIFACT=$SCRATCH/tactic/use_split.auto.shard VERDICT='^PROVE: ' tie "prove use_split" prove --root v3 $INIT "$SCRATCH/tactic/use_split.shard"
VERDICT='^PROVE: \|^LOAD: ' tie "prove measure" prove --root v3 $INIT --measure v3/examples/calc/calc_ndigit.shard
echo "native_tie_test: prove tied over two sidecars regenerated and a measure run"

# ---- (5) verify_release over the tie's store (the bootstrap's copy) and a store without a manifest
cp -r "$OUT/load store/a/artifacts/store" "$SCRATCH/vstore"
VERDICT='^RELEASE: ' tie "verify store" verify_release "$SCRATCH/vstore" "$EXPORT" "$INDEX"
mkdir -p "$SCRATCH/nomanifest"; cp "$SCRATCH/vstore/order" "$SCRATCH/nomanifest/"
tie "verify no manifest" verify_release "$SCRATCH/nomanifest" "$EXPORT" "$INDEX"
tie "verify usage" verify_release
echo "native_tie_test: verify_release tied over the store, a store without a manifest, the usage line"

# ---- (6) t0 over the export's first 3,000 lines; (7) initindex over the same
head -n 3000 "$DIR/init.ndjson" > "$SCRATCH/fix.ndjson"
VERDICT='^T0: ' tie "t0 fixture" t0 -v -a -p "$SCRATCH/fix.ndjson"
tie "t0 usage" t0
VERDICT='^T0: ' tie "t0 wrong pin" t0 -a v3/kernel/test/fixtures/init_wrong_pin.ndjson
ARTIFACT=$SCRATCH/fix.index VERDICT='^index: ' tie "initindex build" initindex "$SCRATCH/fix.ndjson" "$SCRATCH/fix.index"
cp "$OUT/initindex build/a/artifacts/fix.index" "$SCRATCH/fix.index"
VERDICT='^index: ' tie "initindex verify" initindex --verify "$SCRATCH/fix.ndjson" "$SCRATCH/fix.index"
tie "initindex usage" initindex
echo "native_tie_test: t0 and initindex tied over the export's first 3,000 lines and their usage lines"

# ---- (8) one harness: loader_pins_test over every loader pin
VERDICT='^loader_pins_test: failures = ' tie "harness loader_pins_test" test/loader_pins_test
echo "native_tie_test: the loader pins' harness tied"

# ---- (9) the authored tree untouched
checks=$((checks+1)); [ "$(tree_state)" = "$before" ] || { bad "the tree's authored files changed under the tie"; diff <(echo "$before") <(tree_state) | head -5; }

echo "native_tie_test: $checks checks, $fail failed, in $(( $(date +%s) - s0 )) s"
[ "$fail" -eq 0 ] && echo "native_tie_test: byte-identical under both engines on every workload"
exit $fail
