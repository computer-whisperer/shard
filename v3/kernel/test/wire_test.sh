#!/usr/bin/env bash
# The wire by declared type (v3/LANGUAGE.md §6.7, §8.4 slice 3.18 rule 4): an S
# program whose six externs are declared under the naming law — ByteArray,
# Init's List, Option, Prod and Bool (v3/pins/loader/wire_s) — run by the
# driver against the real host, its output and the file it writes compared
# byte for byte; and programs whose externs are declared over a type the wire
# has no codec for (wire_bad) or with a result the extern does not return
# (wire_bad_result), which must be stuck before the host is asked.
# The toolchain's own convention, the prelude's cells, is entry_test's.
# Exit code = the number of failed checks.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "wire_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index (slice 3.26)
P=v3/pins/loader
d=$(mktemp -d); trap 'rm -rf "$d"' EXIT
fail=0; n=0
run() { local c=$1 entry=$2; shift 2; "$EVAL" direct v3/kernel/load.shard --root $P/$c $INIT --run "$entry" $P/$c/main.shard -- "$@"; }
check() { n=$((n+1)); if ! cmp -s "$2" "$3"; then echo "wire_test: $1: output differs"; diff "$2" "$3" | head -6; fail=$((fail+1)); fi; }
code() { n=$((n+1)); if [ "$2" -ne "$3" ]; then echo "wire_test: $1: exit $2, expected $3"; fail=$((fail+1)); fi; }

# get_args and write_line: a literal (its UTF-8 bytes), then each argument on its own line — an empty one, one with a space
run wire_s main.main a "" "b c" "é" > "$d/out" 2>&1; code main $? 0
printf 'wire: héllo\na\n\nb c\né\n' > "$d/want"; check main "$d/out" "$d/want"

# read_file, write_file, write, exit, and the checked entry's ByteArray parameters: arbitrary bytes, not text
printf 'line one\n\377\000\200 raw\n' > "$d/src"
run wire_s main.copy "$d/src" "$d/dst" > "$d/out" 2>&1; code copy $? 7
printf 'copied' > "$d/want"; check copy "$d/out" "$d/want"
check copy-file "$d/src" "$d/dst"
run wire_s main.copy "$d/absent" "$d/dst2" > "$d/out" 2>&1; code copy-missing $? 9
printf 'no such file\n' > "$d/want"; check copy-missing "$d/out" "$d/want"
n=$((n+1)); [ ! -e "$d/dst2" ] || { echo "wire_test: copy-missing wrote a file"; fail=$((fail+1)); }
run wire_s main.copy "$d/src" "$d/no/such/dir/dst" > "$d/out" 2>&1; code copy-unwritable $? 8

# a declared type the wire has no codec for: stuck, nothing printed by the program
run wire_bad main.main > "$d/out" 2>&1; code bad $? 4
printf 'RUN: stuck extern in main.write_line\n' > "$d/want"; check bad "$d/out" "$d/want"

# a declared result the extern does not return: stuck before the host is asked, nothing written
for entry in out args; do
  run wire_bad_result main.$entry > "$d/out" 2>&1; code bad-result-$entry $? 4
  if [ "$entry" = out ]; then printf 'RUN: stuck extern in main.write_line\n' > "$d/want"; else printf 'RUN: stuck extern in main.get_args\n' > "$d/want"; fi
  check bad-result-$entry "$d/out" "$d/want"
done

echo "wire_test: $n checks, $fail failed"
exit $fail
