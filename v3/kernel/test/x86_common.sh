# v3/kernel/test/x86_common.sh — what the tests of the x86-64 model (v3/models/x86) share:
# x86_test.sh, x86_asm_test.sh, x86_trial_test.sh and x86_derive_test.sh each set NAME and source
# it. The engine, the Init index, a scratch directory, the count of checks, the loader's command
# line and the model runner's, a file's run by the processor, a .hex file's bytes. Not an
# entrypoint: each of the four is one, so that the suite runs them side by side — on the
# bootstrap the four together are the longest of the suite by far.
set -u
cd "$(dirname "${BASH_SOURCE[0]}")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "$NAME: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
# the scratch directory holds files these scripts execute: beside the index, never under /tmp
mkdir -p v3/.cache
OUT=$(mktemp -d "v3/.cache/$NAME.XXXXXX")
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "$NAME: $*"; fail=$((fail+1)); }
LOAD=("$EVAL" direct v3/kernel/load.shard --root v3 --init "$EXPORT" --init-index "$INDEX")
M=v3/models/x86
# the model's runner: `run STEPS FILE…` prints a line for each file, `wrap BYTES FILE…` writes files
RUN=("${LOAD[@]}" --run models.x86.run_main.main $M/run_main.shard --)
onchip=0; [ "$(uname -s)-$(uname -m)" = "Linux-x86_64" ] && onchip=1
no_chip() { [ $onchip = 1 ] || echo "$NAME: not an x86-64 Linux machine: the processor's runs are skipped"; }
# a file run by the processor: `exit N`, or `killed` where the shell reports a signal
chip() { local err rc; chmod +x "$1"; err=$( { bash -c '"$0"; exit $?' "$1"; } 2>&1 >/dev/null ); rc=$?; if [ -n "$err" ]; then echo killed; else echo "exit $rc"; fi; }
# the bytes of a .hex file (hexadecimal bytes with `;` comments), its `KK` read as $2
hex2bin() { local h; for h in $(sed 's/;.*//' "$1" | tr -s ' \n' ' '); do printf "\\x${h/KK/$2}"; done; }
