#!/usr/bin/env bash
# v3/kernel/test/handle_cost_test.sh — the prepared handle's two standing checks beside
# handle_test.shard (LANGUAGE.md §8.6 slice 4.6 rules 1 and 6):
#   1. hidden preparation (the T6 row's named failure): linking happens in `prepare` alone —
#      ev.shard calls `link` in one place — and the linked representation stays ev.shard's:
#      no other toolchain source names Linked, IVal, the linker's results or the handle's
#      own fields, and none makes or restates a handle (the record's constructor MkHandle and
#      its updaters Handle.with_FIELD are ordinary declarations any file could call: this
#      check is what holds the convention, no view seals ev.shard).
#      The same for the session (rule 2): a client holds a handle's ordinal, and nothing outside
#      session.shard takes the prepared value out of a slot or restates the tables.
#   2. the link-once ratio, printed: N invocations of one entry through `ev` — N links of a
#      program of D functions — against one `prepare` and N `invoke`s; the two sums of the
#      answers must agree and be the sum the entry's definition gives.
# The times are printed, never judged. Exit code = the checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
fail=0; checks=0
bad() { echo "handle_cost_test: $*"; fail=$((fail+1)); }
ms() { echo $(( $(date +%s%N) / 1000000 )); }

n=$(grep -c '(link p)' v3/kernel/ev.shard)
checks=$((checks+1)); [ "$n" = 1 ] || bad "ev.shard links in $n places (prepare alone links)"
leak=$(grep -lE '\b(Linked|IVal|LinkRes|LkOk|LkErr|MkHandle|Handle\.(lk|id|token|inds)|Handle\.with_[a-z]+)\b' \
         v3/kernel/*.shard v3/kernel/k/*.shard v3/kernel/test/*.shard v3/kernel/k/test/*.shard v3/std/*.shard 2>/dev/null \
       | grep -v '^v3/kernel/ev\.shard$' | tr '\n' ' ')
checks=$((checks+1)); [ -z "$leak" ] || bad "the linked representation or the handle's making is named outside ev.shard: $leak"
# the session's client holds an ordinal, never the prepared value (rule 2): what hands one out —
# the slot's contents, ses_got — is session.shard's own, and so is making or restating its tables
held=$(grep -lE '\b(ses_got|GotHandle|SlotLive|Held\.handle|MkSession|MkHeld|MkRev|(Session|Held|Rev)\.with_[a-z]+)\b' \
         v3/kernel/*.shard v3/kernel/k/*.shard v3/kernel/test/*.shard v3/kernel/k/test/*.shard v3/std/*.shard 2>/dev/null \
       | grep -v '^v3/kernel/session\.shard$' | tr '\n' ' ')
checks=$((checks+1)); [ -z "$held" ] || bad "a prepared value is taken out of the session's table, or its tables restated, outside session.shard: $held"

N=${HANDLE_COST_N:-100}; D=${HANDLE_COST_D:-1000}
want=$(( N * (N + 1) / 2 + 6 * N ))   # total n [1 2 3] = n + 6, for n = 1..N
s=$(ms); each=$("$EVAL" direct v3/kernel/test/handle_test.shard each "$N" "$D" 2>&1 | tail -1); t_each=$(( $(ms) - s ))
s=$(ms); once=$("$EVAL" direct v3/kernel/test/handle_test.shard once "$N" "$D" 2>&1 | tail -1); t_once=$(( $(ms) - s ))
s=$(ms); "$EVAL" direct v3/kernel/test/handle_test.shard once 0 "$D" > /dev/null 2>&1; t_zero=$(( $(ms) - s ))
checks=$((checks+1)); [ "$each" = "each $want" ] || bad "through ev the answers sum to '$each', not $want"
checks=$((checks+1)); [ "$once" = "once $want" ] || bad "on one handle the answers sum to '$once', not $want"
echo "handle_cost_test: $N invocations of one entry, a program of $D functions: $t_each ms through ev (a link each), $t_once ms on one handle; the process with one preparation and no invocation $t_zero ms"
echo "handle_cost_test: $checks checks, $fail failed"
exit $fail
