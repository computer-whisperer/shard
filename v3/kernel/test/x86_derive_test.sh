#!/usr/bin/env bash
# v3/kernel/test/x86_derive_test.sh — the tactic library of the x86-64 model
# (v3/models/x86/derive.shard, buffers_tactic.shard), which derives the proof of a run of
# instructions from a listing's table. Nine of the trial's lemmas — the runs between a function's
# branch points — are proven by it: their blocks stand in buffers_facts.auto.shard, which a load
# of the trial replays (x86_trial_test.sh). Held here: the library loads, every function of it a
# definition; and `prove`, run on a copy of the model without that file, writes it again byte for
# byte, so the library still derives each of the nine. The copy is a root of its own with the
# standard library and the toolchain linked into it: the modules keep their names, and the
# entries their keys.
# Exit code = the number of checks that failed.
NAME=x86_derive_test; . "$(dirname "$0")/x86_common.sh"

"${LOAD[@]}" $M/buffers_tactic.shard > "$OUT/tac.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the tactic library did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR' "$OUT/tac.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn models.x86.\(derive\|buffers_tactic\)\.' "$OUT/tac.txt")" = 0 ] || bad "a function of the tactic library is a program without a definition"
for f in derive.derive_block derive.walk derive.rule_of buffers_tactic.block; do
  checks=$((checks+1)); grep -q "^DEFINE models.x86.$f equations=." "$OUT/tac.txt" || bad "models.x86.$f is not a definition with its equations"
done

R="$OUT/root"; mkdir -p "$R/models/x86"; ln -s "$PWD/v3/std" "$R/std"; ln -s "$PWD/v3/kernel" "$R/kernel"
for f in $M/*.shard; do case "$f" in *.auto.shard) ;; *) cp "$f" "$R/models/x86/" ;; esac; done
"$EVAL" direct v3/kernel/prove.shard --root "$R" --init "$EXPORT" --init-index "$INDEX" "$R/models/x86/buffers_tactic.shard" "$R/models/x86/buffers_facts.shard" > "$OUT/prove.txt" 2>&1
checks=$((checks+1)); grep -q 'proved 9  unsolved 0  sidecars written 1$' "$OUT/prove.txt" || { bad "prove did not derive the 9 blocks through the library"; grep -E 'PROVE:|PENDING|ERROR' "$OUT/prove.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); cmp -s "$R/models/x86/buffers_facts.auto.shard" $M/buffers_facts.auto.shard || bad "the blocks the library derives are not the committed ones (buffers_facts.auto.shard)"

echo "$NAME: $checks checks, $fail failed"
exit $fail
