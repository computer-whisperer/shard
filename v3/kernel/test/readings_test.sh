#!/usr/bin/env bash
# v3/kernel/test/readings_test.sh — the two readings of a program (docs/LAW.md rule 24).
# A fn's program is the erasure of its elaborated body (the typed route); the classifier reads
# a program from the source and the types its binders state, with no elaborated term. Under
# `load.shard --both` every fn the typed route gives a program is read by the classifier too
# and the two programs are compared: `BOTH NAME same`, `differs` with both texts, or `unread`
# with the classifier's refusal. Over v3/pins/readings/main.shard:
# (1) the fns of its first three parts, and `say` and `report`, are `same`: the operators by
#     the type expected and by the operands, a comparison as a Bool, Init's operations written
#     by name; then what is written and erased — a type, a proof, a hypothesis, a value in a
#     type — a row's constructor, projection and pattern, a projection at its field's type, a
#     decision as a value, a condition and two cells, `=` at Bool and at a derived equality,
#     `@` with every binder written; then a function as an argument — a lambda closed, over
#     two locals, in a lambda, with its result's type from the type expected of the call, in
#     a call that is itself a template's; a name, a name applied, a constant of the registry
#     bare and applied, a package — each lifted lambda `same` under its own name; numeral
#     rows and a let of several bindings; and a supplied realize, under its constant's name;
# (2) the controls: a call that leaves a runtime argument for the elaborator to fill in is
#     `unread`, and so is a package's field read off its constant; a lambda whose proof names
#     a local its program does not is `differs`, in the fn and in the lambda — the typed
#     route captures the local, the classifier never reads the proof; no fn has another
#     verdict, and a load without the flag has no `BOTH` record;
# (3) the part the classifier alone reads runs to ten lines: `-` at Nat stops at zero, `/`
#     and `%` at Int are the Euclidean pair, Int.natAbs is a Nat, a comparison is Init's Bool,
#     `=` at a derived equality is its decision, a lambda lifted there and a registry
#     constant passed run, a package's instance runs, numeral rows are tests in row order, a
#     row built there is its field;
# (4) v3/pins/readings/unchecked.shard: where the classifier's is the only reading a written
#     proof and a refuted row are refused — nothing checks them — and the same row without a
#     proof is a program;
# (5) the reading of a build, `load.shard --unchecked`, over v3/pins/readings/build.shard: a
#     load without the flag defines `total` and refuses `mistyped`, a String where a list is
#     declared, and `main` after it; under the flag each of the five fns is a program
#     (`why=unchecked route=e_first`), nothing is defined or refused, and the program runs to
#     its two lines — the type error is compiled, not refused. A proof written inside a fn is
#     not passed over: build_proof.shard's `first`, a definition without the flag, is refused
#     under it. A `theorem` and a `fulfills` are: build_theorem.shard's descent is proven and
#     its theorem accepted without the flag, and under it the load accepts and proves nothing.
# Runs on either engine (EVAL); under v3/test.sh the two runs are compared byte for byte.
# Exit code = the number of failed checks.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "readings_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
OUT=$(mktemp -d); trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "readings_test: $*"; fail=$((fail+1)); }
F=v3/pins/readings/main.shard
M='pins\.readings\.main'

"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --both $F > "$OUT/both.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the load under --both failed (exit $rc)"; grep -E '^(READ-ERROR|LOAD-ERROR|REFUSE|LOAD:)' "$OUT/both.txt" | head -5 | cut -c1-300; }

# ---- (1) one program by both readings
for f in sub_nat sub_int numeral_right numerals_at_int numerals_at_nat div_both mod_both under_int under_list cmp_value cmp_cond abs_pred connectives succ_zero by_name bits_nat bits_int bits_expected self_call \
         Coin.ctorIdx Coin.decEq row_make row_field row_pattern value_in_type proof_binder proof_call alias_type hypothesis decision_value decision_cond decision_cells eq_bool eq_derived every_binder width_written row_typed \
         inc scale lam_closed lam_closed.lam_1 lam_captures lam_captures.lam_1 lam_nested lam_nested.lam_1 lam_nested.lam_2 lam_expected lam_expected.lam_1 lam_inner lam_inner.lam_1 lam_inner.lam_2 \
         named named_partial by_registry registry_partial package rows_local rows_bound let_run \
         say report; do
  checks=$((checks+1))
  grep -Eq "^BOTH $M\.$f same\$" "$OUT/both.txt" || { bad "$f is not read the same both ways"; grep -E "^BOTH $M\.$f " "$OUT/both.txt" | cut -c1-400; }
done

checks=$((checks+1)); grep -Eq '^BOTH List\.length same$' "$OUT/both.txt" || { bad "the supplied realize is not read the same both ways"; grep -E '^BOTH List\.length ' "$OUT/both.txt" | cut -c1-400; }

# ---- (2) the controls
checks=$((checks+1)); grep -Eq "^BOTH $M\.width_inferred unread: unwritten_argument: BitVec\.toNat: " "$OUT/both.txt" || { bad "the runtime argument left unwritten is not reported unread"; grep -E "^BOTH $M\.width_inferred " "$OUT/both.txt" | cut -c1-300; }
checks=$((checks+1)); grep -Eq "^BOTH $M\.field_at_constant unread: function_value: $M\.Monoid\.op: " "$OUT/both.txt" || { bad "the package's field at its constant is not reported unread"; grep -E "^BOTH $M\.field_at_constant " "$OUT/both.txt" | cut -c1-300; }
checks=$((checks+1)); grep -Eq "^BOTH $M\.proof_captures differs: typed: fn proof_captures 0 \(Nat Nat\) Nat \([a-z_]*proof_captures_lam_1_1 #1 #0\) \| plain: fn proof_captures 0 \(Nat Nat\) Nat \([a-z_]*proof_captures_lam_1_0 #0\)\$" "$OUT/both.txt" || { bad "the local a proof alone names is not reported with the two programs"; grep -E "^BOTH $M\.proof_captures " "$OUT/both.txt" | cut -c1-300; }
checks=$((checks+1)); grep -Eq "^BOTH $M\.proof_captures\.lam_1 differs: typed: fn lam_1 0 \(Nat Nat\) Nat 0 \| plain: fn lam_1 0 \(Nat\) Nat 0\$" "$OUT/both.txt" || { bad "the lifted lambda is not reported with its two programs"; grep -E "^BOTH $M\.proof_captures\.lam_1 " "$OUT/both.txt" | cut -c1-300; }
n=$(grep -c '^BOTH ' "$OUT/both.txt")
checks=$((checks+1)); [ "$n" = 66 ] || { bad "$n BOTH records, not the sixty-six of the fixture"; grep '^BOTH ' "$OUT/both.txt" | cut -c1-120; }
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT $F > "$OUT/plain.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || bad "the load without the flag failed (exit $rc)"
checks=$((checks+1)); if grep -q '^BOTH ' "$OUT/plain.txt"; then bad "a load without --both has BOTH records"; fi

# ---- (3) the classifier's program, run
printf '%s\n' 'Nat.sub: ok' 'Int.ediv: ok' 'Int.emod: ok' 'natAbs then Nat.sub: ok' 'a comparison is a Bool: ok' 'a derived equality: ok' 'a lifted lambda: ok' 'a package: ok' 'numeral rows: ok' 'ok' > "$OUT/want.txt"
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --run pins.readings.main.main $F > "$OUT/run.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || bad "the run failed (exit $rc)"
checks=$((checks+1)); cmp -s "$OUT/run.txt" "$OUT/want.txt" || { bad "the run's lines are not the ten expected"; cut -c1-200 "$OUT/run.txt" | head -10; }

# ---- (4) a written proof has one reader
U='pins\.readings\.unchecked'
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/pins/readings/unchecked.shard > "$OUT/unchecked.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" != 0 ] || bad "the load of the unchecked proofs succeeded"
checks=$((checks+1)); grep -Eq "^REFUSE $U\.false_claim unchecked_proof: Fin\.mk: " "$OUT/unchecked.txt" || { bad "a proof with no reader is not refused"; grep -E "$U\.false_claim" "$OUT/unchecked.txt" | cut -c1-300; }
checks=$((checks+1)); grep -Eq "^REFUSE $U\.refuted absurd_e_first: " "$OUT/unchecked.txt" || { bad "a refuted row with no reader is not refused"; grep -E "$U\.refuted" "$OUT/unchecked.txt" | cut -c1-300; }
checks=$((checks+1)); grep -Eq "^RUNNABLE fn $U\.no_proof " "$OUT/unchecked.txt" || { bad "the row without a proof is not a program"; grep -E "$U\.no_proof" "$OUT/unchecked.txt" | cut -c1-300; }

# ---- (5) the reading of a build
B='pins\.readings\.build'
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/pins/readings/build.shard > "$OUT/b_checked.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 2 ] || bad "the checked load of the build fixture exits $rc, not 2 (mistyped and main refused)"
checks=$((checks+1)); grep -Eq "^DEFINE $B\.total " "$OUT/b_checked.txt" || { bad "total is not a definition in the checked load"; grep -E "$B\.total" "$OUT/b_checked.txt" | cut -c1-300; }
checks=$((checks+1)); grep -Eq "^REFUSE $B\.mistyped type_mismatch: expected \(List\.\{0\} Nat\), found String\$" "$OUT/b_checked.txt" || { bad "the type error is not refused in the checked load"; grep -E "$B\.mistyped" "$OUT/b_checked.txt" | cut -c1-300; }
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --unchecked v3/pins/readings/build.shard > "$OUT/b_unchecked.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the load under --unchecked failed (exit $rc)"; grep -E '^(READ-ERROR|LOAD-ERROR|REFUSE|LOAD:)' "$OUT/b_unchecked.txt" | head -5 | cut -c1-300; }
for f in say total mistyped report main; do
  checks=$((checks+1)); grep -Eq "^RUNNABLE fn $B\.$f why=unchecked route=e_first\$" "$OUT/b_unchecked.txt" || { bad "$f is not a program alone under --unchecked"; grep -E "$B\.$f( |\$)" "$OUT/b_unchecked.txt" | cut -c1-300; }
done
checks=$((checks+1)); if grep -Eq '^(DEFINE|REFUSE) ' "$OUT/b_unchecked.txt"; then bad "a load under --unchecked defines or refuses"; grep -E '^(DEFINE|REFUSE) ' "$OUT/b_unchecked.txt" | head -3 | cut -c1-300; fi
printf '%s\n' 'total: ok' 'a String where a list is declared: its bytes' > "$OUT/b_want.txt"
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --unchecked --run pins.readings.build.main v3/pins/readings/build.shard > "$OUT/b_run.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || bad "the run under --unchecked failed (exit $rc)"
checks=$((checks+1)); cmp -s "$OUT/b_run.txt" "$OUT/b_want.txt" || { bad "the run under --unchecked does not print the two lines expected"; cut -c1-200 "$OUT/b_run.txt" | head -5; }
P='pins\.readings\.build_proof'
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/pins/readings/build_proof.shard > "$OUT/p_checked.txt" 2>&1; rc=$?
checks=$((checks+1)); { [ "$rc" = 0 ] && grep -Eq "^DEFINE $P\.first " "$OUT/p_checked.txt"; } || { bad "the written proof is not checked into a definition without the flag (exit $rc)"; grep -E "$P\.first" "$OUT/p_checked.txt" | cut -c1-300; }
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --unchecked v3/pins/readings/build_proof.shard > "$OUT/p_unchecked.txt" 2>&1; rc=$?
checks=$((checks+1)); { [ "$rc" != 0 ] && grep -Eq "^REFUSE $P\.first unchecked_proof: Fin\.mk: " "$OUT/p_unchecked.txt"; } || { bad "a written proof is not refused under --unchecked (exit $rc)"; grep -E "$P\.first" "$OUT/p_unchecked.txt" | cut -c1-300; }
T='pins\.readings\.build_theorem'
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT v3/pins/readings/build_theorem.shard > "$OUT/t_checked.txt" 2>&1; rc=$?
checks=$((checks+1)); { [ "$rc" = 0 ] && grep -Eq "^DISCHARGE $T\.count\.dec_1 proved\$" "$OUT/t_checked.txt" && grep -Eq "^ACCEPT $T\.count_two " "$OUT/t_checked.txt"; } || { bad "the descent is not proven or the theorem not accepted without the flag (exit $rc)"; grep -E "^(DISCHARGE|READ-ERROR|LOAD-ERROR|REFUSE) " "$OUT/t_checked.txt" | head -3 | cut -c1-300; }
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --unchecked v3/pins/readings/build_theorem.shard > "$OUT/t_unchecked.txt" 2>&1; rc=$?
checks=$((checks+1)); { [ "$rc" = 0 ] && grep -Eq "^RUNNABLE fn $T\.count why=unchecked route=e_first\$" "$OUT/t_unchecked.txt"; } || { bad "the load of a theorem and a fulfills under --unchecked failed (exit $rc)"; grep -E '^(READ-ERROR|LOAD-ERROR|REFUSE|LOAD:)' "$OUT/t_unchecked.txt" | head -3 | cut -c1-300; }
checks=$((checks+1)); if grep -Eq '^(ACCEPT|DEFINE|DISCHARGE|PARAM|PENDING) ' "$OUT/t_unchecked.txt"; then bad "a load under --unchecked reads a theorem or a fulfills"; grep -E '^(ACCEPT|DEFINE|DISCHARGE|PARAM|PENDING) ' "$OUT/t_unchecked.txt" | head -3 | cut -c1-300; fi

echo "readings_test: $checks checks, $fail failed"
exit $fail
