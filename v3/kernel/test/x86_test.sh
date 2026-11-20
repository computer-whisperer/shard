#!/usr/bin/env bash
# v3/kernel/test/x86_test.sh — the x86-64 model (v3/models/x86), loaded, and held to the
# processor: the model, its Linux boundary, the file writer and the runner load with every
# function a definition and nothing refused or pending; `exit42_run` and `exit42_exits` —
# the model's run of the first program's file ends the process with status 42, from any
# registers and flags — are accepted on no axiom but `propext`; the file is written by the
# model's own writer; the runner's line for it is `exit 42`; and on an x86-64 Linux machine the
# file is run and the processor's exit status is the model's. A file the model does not load is
# `stuck` with its reason. On another machine the processor's run is skipped, and said so.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "x86_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
# the scratch directory holds a file this script executes: beside the index, never under /tmp
mkdir -p v3/.cache
OUT=$(mktemp -d v3/.cache/x86_test.XXXXXX)
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "x86_test: $*"; fail=$((fail+1)); }
LOAD=("$EVAL" direct v3/kernel/load.shard --root v3 --init "$EXPORT" --init-index "$INDEX")
M=v3/models/x86

# ---- the load ---------------------------------------------------------------------------------
"${LOAD[@]}" $M/exit42.shard $M/run_main.shard > "$OUT/load.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the model did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/load.txt" || { bad "the summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/load.txt")" = 0 ] || { bad "a function is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/load.txt" | head -5 | cut -c1-200; }
for f in x86.reg_get x86.reg_set x86.mapped x86.fetch x86.decode x86.exec x86.step linux.elf_load linux.run linux.run_file elf.elf_exec; do
  checks=$((checks+1)); grep -q "^DEFINE models.x86.$f equations=." "$OUT/load.txt" || bad "models.x86.$f is not a definition with its equations"
done
# the theorems, on `propext` at most
for t in exit42_run exit42_exits; do
  checks=$((checks+1)); grep -q "^ACCEPT models.x86.exit42.$t " "$OUT/load.txt" || bad "models.x86.exit42.$t is not accepted"
  axs=$(grep "^ACCEPT models.x86.exit42.$t " "$OUT/load.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  checks=$((checks+1)); case "$axs" in ""|propext) ;; *) bad "models.x86.exit42.$t rests on $axs" ;; esac
done

# ---- the file, the model's run of it, the processor's ------------------------------------------
"${LOAD[@]}" --run models.x86.exit42.main $M/exit42.shard -- "$OUT/exit42" > "$OUT/write.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$(stat -c %s "$OUT/exit42" 2>/dev/null)" = 132 ] || { bad "the writer did not write the 132 bytes (exit $rc)"; head -3 "$OUT/write.txt" | cut -c1-200; }
model() { "${LOAD[@]}" --run models.x86.run_main.main $M/run_main.shard -- "$1" "$2" 2>&1 | tail -1; }
m=$(model "$OUT/exit42" 100)
checks=$((checks+1)); [ "$m" = "exit 42" ] || bad "the model's run of the file is not exit 42: $m"
# out of steps: two steps do not reach the system call
m=$(model "$OUT/exit42" 2)
checks=$((checks+1)); [ "$m" = "out of steps at 4194434" ] || bad "two steps of the model do not stop at the system call's address: $m"
# a file the model does not load
printf '\177ELF' > "$OUT/junk"
m=$(model "$OUT/junk" 100)
checks=$((checks+1)); [ "$m" = "stuck: not a file the model loads" ] || bad "a file of four bytes is not refused by the model: $m"
if [ "$(uname -s)-$(uname -m)" = "Linux-x86_64" ]; then
  chmod +x "$OUT/exit42"; "$OUT/exit42"; rc=$?
  checks=$((checks+1)); [ "$rc" = 42 ] || bad "the processor's exit status is $rc, the model's 42"
else
  echo "x86_test: not an x86-64 Linux machine: the processor's run is skipped"
fi

echo "x86_test: $checks checks, $fail failed"
exit $fail
