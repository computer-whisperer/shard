#!/usr/bin/env bash
# v3/kernel/test/derive_test.sh — deriving under a declared policy (v3/LANGUAGE.md
# §8.4 slice 3.19; law §5.1) on real files, with the export through UInt8.toNat
# (the pins' prefix and its continuation: Ordering and the bytes' structures lie
# past the pins'):
#   1. v3/std/derive.shard loads clean and registers the library's fourteen
#      procedures — Bool's derived, UInt8's, ByteArray's and String's by hand,
#      Nat's and Int's renderers;
#   2. v3/examples/derive/derive_main.shard runs on the host and prints its
#      values as source would write them: derived equality in an `if`, the
#      derived ordering as a sort key, the renderings, and every row of
#      Init's Bool connectives and of Bool's equality (K decides the same list);
#   3. v3/examples/derive/convention.shard loads clean — the declaration order
#      is the ordering's convention, decided by K — and the record of a type
#      over a hand-written ordering names the procedure it selected.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "derive_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"   # Init on demand from the pinned export and its index (slice 3.26)
RCPT=${INIT_RECEIPT:+--init-receipt $INIT_RECEIPT}   # the Init receipt (slice 3.23), set by v3/test.sh when one exists
fail=0; checks=0
load() { "$EVAL" direct v3/kernel/load.shard --root v3 $INIT $RCPT "$@" 2>&1; }
want() {   # NAME TEXT PATTERN
  checks=$((checks+1))
  if ! echo "$2" | grep -q -- "$3"; then echo "derive_test: $1 lacks '$3'"; echo "$2" | grep -E 'REFUSE|ERROR|DERIVE' | head -6; fail=$((fail+1)); fi
}

out=$(load v3/std/derive.shard)
want library "$out" 'runnable 0  refused 0  pending 0  realized 2  defined 26  errors 0'
checks=$((checks+1))
n=$(echo "$out" | grep '^DERIVE ' | grep -vc ' cap=idx ')   # a constructor index is its own record
[ "$n" -eq 14 ] || { echo "derive_test: the library registers $n procedures, not 14"; fail=$((fail+1)); }
want library "$out" 'DERIVE std.derive.List_UInt8.decEq type=(List UInt8) cap=eq policy=structural fields=std.derive.Byte.decEq'
want library "$out" 'DERIVE std.derive.Text.decEq type=String cap=eq policy=by fields='
want library "$out" 'DISCHARGE std.derive.Nat.renderOnto.dec_1 proved'

got=$(mktemp); exp=$(mktemp); trap 'rm -f "$got" "$exp"' EXIT
cat > "$exp" <<'LINES'
(Hand "north" (cons (Card 10 Hearts) (cons Joker (cons (Card 2 Spades) (cons (Card 10 Clubs) nil)))) (some (Card 1 Diamonds)) true -3)
(Hand "south" nil none false 7)
(cons (Card 2 Spades) (cons (Card 10 Clubs) (cons (Card 10 Hearts) (cons Joker nil))))
1
same
different
lt
"a\"b\\c"
true
(cons false (cons false (cons false (cons true (cons false (cons true (cons true (cons true (cons true (cons false (cons true (cons false (cons false (cons true nil))))))))))))))
LINES
checks=$((checks+1))
load --run examples.derive.derive_main.main v3/examples/derive/derive_main.shard > "$got"; rc=$?
if [ "$rc" -ne 0 ] || ! cmp -s "$got" "$exp"; then
  echo "derive_test: the example's output is not the expected lines (exit $rc)"; diff "$got" "$exp" | head -8; fail=$((fail+1))
fi
out=$(load v3/examples/derive/derive_main.shard)
want example "$out" 'refused 0  pending 0  realized 2 '
want example "$out" 'DERIVE examples.derive.derive_main.Hand.decEq type=examples.derive.derive_main.Hand cap=eq policy=structural fields=std.derive.Text.decEq,examples.derive.derive_main.List_Card.decEq,examples.derive.derive_main.Option_Card.decEq,instDecidableEqBool,Int.decEq'
want example "$out" 'ACCEPT examples.derive.derive_main.sorted '
want example "$out" 'ACCEPT examples.derive.derive_main.truth_is '

out=$(load v3/examples/derive/convention.shard)
want convention "$out" 'runnable 0  refused 0  pending 0  realized 0  defined 8  errors 0'
want convention "$out" 'DERIVE examples.derive.convention.Urgency.first type=examples.derive.convention.Urgency cap=ord policy=by fields='
want convention "$out" 'DERIVE examples.derive.convention.Job.compare type=examples.derive.convention.Job cap=ord policy=structural fields=examples.derive.convention.Urgency.first,Nat.lt'
want convention "$out" 'ACCEPT examples.derive.convention.urgent_first '

echo "derive_test: $checks checks, $fail failed"
exit $fail
