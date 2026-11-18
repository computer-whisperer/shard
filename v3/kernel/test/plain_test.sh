#!/usr/bin/env bash
# The Rust bootstrap's reader (rust_bootstrap/src/plain/) against the loader, over ordinary
# closures: one whose files are modules under a root, whose names are identities and whose data
# is Init's, read by the Rust evaluator with no K — from the source and the facts about Init's
# constants the loader prints for it (`load.shard --facts OUT`, kernel/facts.shard) — must be
# the program the loader reads: the two dumps (`load.shard --dump`, `eval dump --root`) are one
# canonical text (kernel/dump.shard's header). parity_test.sh is the same comparison over the
# toolchain's own closures, read by the tree's table (v3/kernel/init.facts).
#
#   complete  a closure the reader reads whole (it exits 0): the two dumps byte for byte
#             — v3/examples/calc/calc_main.shard (calc over the host, std/bytes, std/list)
#             — v3/std/mem.shard and v3/pins/loader/record_basic (records: the type, the
#               accessors and the updaters, `make` and `with` rewritten)
#             — v3/pins/loader/view_basic (a directory module: the view's sigs under the
#               implementation's type and fns; a dotted constructor citation)
#             — v3/tools/shardfmt/fmt.shard with its two directory modules (print, reads),
#               their private files and what those cite of Init; its `(derive CNode size)`
#               writes definitions with no program, and the reader passes it
#   partial   a closure with forms the reader has no reading for (a template's call, a
#             derivation that writes functions, a derived view; it exits 3 and names each on
#             standard error): every line it prints is a line of the loader's dump, and their
#             number is the one stated here
#             — v3/pins/readings/main.shard, the two readings' fixture
#   run       calc's `main` run by the Rust evaluator directly on the closure it read prints
#             what the loader's run of the same program prints, over the differential's inputs
#
# The reader links every view it loads to its implementation; the loader links a directory it
# is given as a root (and checks it there), so a closure's directories are named to it.
#
# The loader runs on $EVAL, the suite's engine; the reader under test is always the Rust
# evaluator's own.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
RUST=./rust_bootstrap/target/release/eval
[ -x "$RUST" ] || { echo "plain_test: no Rust evaluator at $RUST (cargo build --release --manifest-path rust_bootstrap/Cargo.toml)"; exit 1; }
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "plain_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index
d=$(mktemp -d); trap 'rm -rf "$d"' EXIT
failed=0; checks=0
s=$(date +%s)

# ROOT FILE DIR…: the loader's dump in $d/L and its facts in $d/facts, the reader's dump in
# $d/R, what it left unread in $d/E, its exit code in $rc
both() {
  local root=$1 file=$2; shift 2
  "$EVAL" direct v3/kernel/load.shard --root "$root" $INIT --facts "$d/facts" --dump "$file" "$@" > "$d/L" 2>&1 \
    || { echo "plain_test: load --dump failed on $file"; head -3 "$d/L"; return 1; }
  "$RUST" dump --root "$root" --facts "$d/facts" "$file" > "$d/R" 2> "$d/E"; rc=$?
}
complete() {   # ROOT FILE DIR…
  checks=$((checks+1))
  both "$@" || { failed=$((failed+1)); return; }
  if [ "$rc" -ne 0 ]; then echo "plain_test: $2: the reader exits $rc"; head -3 "$d/E"; failed=$((failed+1)); return; fi
  if cmp -s "$d/L" "$d/R"; then echo "plain_test: $2: complete, $(wc -l < "$d/L") declarations byte-identical"
  else echo "plain_test: $2 differs"; diff "$d/L" "$d/R" | head -6 | cut -c1-200; failed=$((failed+1)); fi
}
partial() {   # SAME TOTAL UNREAD ROOT FILE DIR…
  local same=$1 total=$2 unread=$3; shift 3
  checks=$((checks+1))
  both "$@" || { failed=$((failed+1)); return; }
  if [ "$rc" -ne 3 ]; then echo "plain_test: $2: the reader exits $rc, not 3"; head -3 "$d/E"; failed=$((failed+1)); return; fi
  extra=$(comm -13 <(sort "$d/L") <(sort "$d/R") | wc -l)
  got="$(wc -l < "$d/R") of $(wc -l < "$d/L"), $(grep -c '^UNREAD ' "$d/E") unread"
  if [ "$extra" -ne 0 ]; then echo "plain_test: $2: $extra lines that are not the loader's"; comm -13 <(sort "$d/L") <(sort "$d/R") | head -3 | cut -c1-200; failed=$((failed+1)); return; fi
  if [ "$got" = "$same of $total, $unread unread" ]; then echo "plain_test: $2: partial, $got; every line the loader's"
  else echo "plain_test: $2: $got, expected $same of $total, $unread unread"; failed=$((failed+1)); fi
}

complete v3 v3/examples/calc/calc_main.shard
cp "$d/facts" "$d/calc.facts"
complete v3 v3/std/mem.shard
complete v3/pins/loader/record_basic v3/pins/loader/record_basic/main.shard
complete v3/pins/loader/view_basic v3/pins/loader/view_basic/main.shard v3/pins/loader/view_basic/lib
complete v3 v3/tools/shardfmt/fmt.shard v3/tools/shardfmt/print v3/tools/shardfmt/reads
partial 61 105 20 v3 v3/pins/readings/main.shard

# the run: one line of the differential's inputs per argument
checks=$((checks+1))
lines=()
while IFS= read -r line || [ -n "$line" ]; do lines+=("$line"); done < v3/kernel/test/fixtures/calc_inputs.txt
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --run examples.calc.calc_main.main v3/examples/calc/calc_main.shard -- "${lines[@]}" > "$d/want" 2>&1; wrc=$?
"$RUST" direct --root v3 --facts "$d/calc.facts" v3/examples/calc/calc_main.shard "${lines[@]}" > "$d/got" 2>&1; grc=$?
if [ "$wrc" -ne 0 ] || [ "$grc" -ne 0 ]; then echo "plain_test: the run failed (the loader's exit $wrc, the reader's $grc)"; head -3 "$d/got"; failed=$((failed+1))
elif cmp -s "$d/want" "$d/got"; then echo "plain_test: calc's main on the closure the reader read: byte-identical to the loader's run over ${#lines[@]} inputs"
else echo "plain_test: the run differs"; diff "$d/want" "$d/got" | head -6; failed=$((failed+1)); fi

e=$(date +%s)
echo "plain_test: $checks checks, $failed failed, in $((e-s)) s"
exit $failed
