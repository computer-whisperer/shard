#!/usr/bin/env bash
# v3/kernel/test/messages_test.sh — what a refusal says. A loader pin states the reason its
# load must end in (v3/pins/loader/README.md); the text after the reason is no part of a pin's
# header. This test loads a set of pins, and sources of its own, and requires of each
# record the words an author acts on: the counts a `cases` or `induction` arm is told, what
# to do with an `(import Init)` line, the file a refusal inside an implementation names,
# the row that makes later rows unreachable, the let a nested recursion cites, and what to
# write where Lean's `mutual`, `instance`, a bare `true` or a head shadowed by Init's was
# written.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "messages_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
OUT=$(mktemp -d)
SCRATCH=$(mktemp -d -p v3 messages_scratch_XXXXXX)
trap 'rm -rf "$OUT" "$SCRATCH"' EXIT
fail=0; checks=0
bad() { echo "messages_test: $*"; fail=$((fail+1)); }
P=v3/pins/loader
S=$(basename "$SCRATCH")
# a record of the load that matches the extended regular expression; on a miss, the records
# that name the subject are shown
has() {   # WHAT SUBJECT REGEX
  checks=$((checks+1))
  grep -Eq -- "$3" "$OUT/load.txt" || { bad "$1"; grep -F -- "$2" "$OUT/load.txt" | grep -v '^ACCEPT\|^MODULE' | head -3 | cut -c1-400; }
}
lacks() {   # WHAT REGEX
  checks=$((checks+1))
  if grep -Eq -- "$2" "$OUT/load.txt"; then bad "$1"; grep -E -- "$2" "$OUT/load.txt" | head -2 | cut -c1-300; fi
}

# the test's own sources. step.shard and latefile.shard are the first roots of the load: what
# each needs of Init — the lemmas a step builds on, a lemma cited after a file import that
# follows a declaration — is fetched for it, nothing a root after them fetches being in the
# environment yet. named.shard writes the line that says a file sees Init
printf '%s\n' '(theorem t ((a Nat) (b Nat) (h (Eq a b))) (Eq a b) (by (rw h) rfl))' > "$SCRATCH/step.shard"
printf '%s\n' '(fn twice ((n Nat)) Nat (+ n n))' > "$SCRATCH/far.shard"
printf '%s\n' '(theorem a () (Eq 1 1) (exact (Eq.refl 1)))' '(import "far.shard")' '(theorem t ((n Nat) (h (Not (Eq n 0)))) (Nat.lt (Nat.sub n 1) n) (exact (Nat.sub_one_lt h)))' > "$SCRATCH/latefile.shard"
printf '%s\n' '(import Init)' '(theorem t () (Eq 1 1) (exact (Eq.refl 1)))' > "$SCRATCH/named.shard"
printf '%s\n' '(fn f ((n Nat)) Nat (+ 1 (g n)))' '(fn g ((n Nat)) Nat (Nat.add n "one"))' > "$SCRATCH/parked.shard"
printf '%s\n' '(class Foo () Nat 1)' > "$SCRATCH/class.shard"

"$EVAL" direct v3/kernel/load.shard --root v3 $INIT "$SCRATCH/step.shard" "$SCRATCH/latefile.shard" \
  "$SCRATCH/named.shard" \
  $P/by_cases_arm_fields/main.shard $P/by_cases_fields/main.shard \
  $P/impl_read_error/lib $P/impl_load_error/lib \
  $P/define_nested/main.shard \
  $P/match_dead_row/main.shard $P/match_dead_row_def/main.shard $P/ev_dead_row/kernel/main.shard \
  $P/lean_mutual/main.shard $P/reserved_instance/main.shard "$SCRATCH/class.shard" $P/bool_bare/main.shard \
  "$SCRATCH/parked.shard" > "$OUT/load.txt" 2>&1 &
# a case whose `use` names its own module is loaded under its own root
"$EVAL" direct v3/kernel/load.shard --root $P/shadowed_head $INIT $P/shadowed_head/main.shard > "$OUT/shadowed.txt" 2>&1 &
wait
checks=$((checks+1)); grep -q '^LOAD: ' "$OUT/load.txt" || { bad "the load did not end"; tail -3 "$OUT/load.txt" | cut -c1-300; }

# ---- the arm's names against its case
has "cases: an arm naming one of two fields is not told both counts" "by_cases_arm_fields" \
  "cases_fields: step 1: pins\.loader\.by_cases_arm_fields\.main\.Exp\.Add's arm names 1 binder\(s\) and its case has 2: name the constructor's 2 field\(s\) — \(a : "
has "induction: an arm naming the fields and no hypothesis is not told both counts" "by_cases_fields" \
  "induction_fields: step 1: List\.cons's arm names 2 binder\(s\) and its case has 3: name the constructor's 2 field\(s\), then 1 induction hypothesis name\(s\), one for each recursive field — \(x : Nat\) \(r : "

# ---- Init: the line that is no form, what a first root is given
has "(import Init): not refused with what to do" "$S/named.shard" \
  "^LOAD-ERROR $S/named\.shard bad_import: \(import Init\): every file sees Init, and no line says so — delete the line$"
has "a step's lemmas: a theorem by rw in a first root is not accepted" "$S.step" "^ACCEPT $S\.step\.t "
has "a late file import: a lemma cited after it is not found" "$S.latefile" "^ACCEPT $S\.latefile\.t "
lacks "an INIT record" "^INIT "
lacks "an Init position on a record" " init="
lacks "a refusal that speaks of a horizon" "horizon"

# ---- the file a refusal inside an implementation names
has "an implementation's refused form: the record does not name the implementation's file" "impl_read_error" \
  "^IMPL READ-ERROR pins/loader/impl_read_error/lib/lib\.shard rfl_failed: "
has "an implementation's failed import: the record does not name the implementation's file" "impl_load_error" \
  "^IMPL LOAD-ERROR pins/loader/impl_load_error/lib/lib\.shard import_failed: pins/loader/impl_load_error/lib/absent\.shard$"
lacks "a record of an implementation's check that names the view's file" "^IMPL (READ|LOAD)-ERROR [^ ]*mod\.req\.shard "

# ---- a row that matches everything
has "an unresolved constructor's row: the refusal does not name the row and why it is a variable" "match_dead_row.main.code" \
  "^REFUSE pins\.loader\.match_dead_row\.main\.code unreachable_arm: the row Rde binds a variable and matches every value, so the 2 row\(s\) after it can never be taken: no constructor of pins\.loader\.match_dead_row\.main\.Color named Rde is in scope"
has "a row after _: not refused as unreachable" "match_dead_row_def" \
  "^READ-ERROR pins/loader/match_dead_row_def/main\.shard unreachable_arm: the row _ matches every value, so the 1 row\(s\) after it can never be taken"
has "an unresolved constructor's row in a program-only file: not refused" "ev_dead_row" \
  "^REFUSE pins\.loader\.ev_dead_row\.kernel\.main\.code unreachable_arm: the row Rde binds a variable and matches every value, so the 2 row\(s\) after it can never be taken: no constructor named Rde is in scope"
has "a capitalised variable in a last row is not a variable any more" "ev_dead_row" \
  "^RUNNABLE fn pins\.loader\.ev_dead_row\.kernel\.main\.first_or "

# ---- nested recursion
has "nested recursion: the reason does not name the let the argument cites" "define_nested" \
  "^RUNNABLE fn pins\.loader\.define_nested\.main\.f why=nested_recursion route=typed: .* it cites r, a let bound to the result of a recursive call"
lacks "a definition K refused for a free variable in an obligation" "fvar_in_type"

# ---- what to write
has "mutual: the refusal does not say that the functions are written one after another" "lean_mutual" \
  "^READ-ERROR pins/loader/lean_mutual/main\.shard lean_form: mutual — there is no mutual block: write each fn on its own, with its \(measure …\)"
has "instance: the refusal does not say what stands in its place" "reserved_instance" \
  "^READ-ERROR pins/loader/reserved_instance/main\.shard reserved_form: instance: there are no user classes and no instance declarations: .*\(derive TYPE CAPABILITY…\)$"
has "class: the refusal does not say what stands in its place" "$S/class.shard" \
  "^READ-ERROR $S/class\.shard reserved_form: class: there are no user classes and no instance declarations: "
lacks "a refusal that answers with a phase or a deferral" "(: phase [0-9]|is deferred)"
has "a bare true: the refusal does not say to write Bool.true or (use Init.Bool)" "bool_bare" \
  "^REFUSE pins\.loader\.bool_bare\.main\.is_zero unknown_constant: true: Init's constructor is Bool\.true — write Bool\.true, or open the namespace with \(use Init\.Bool\)$"
has "a function that calls a refused one: not refused with the callee named" "$S.parked" \
  "^REFUSE $S\.parked\.f callee_undeclared: g: a function of this file that has no declaration — it was refused, so this one does not read$"
lacks "a parked function labelled mutual_recursion" "why=mutual_recursion"
checks=$((checks+1)); grep -Eq "^READ-ERROR main\.shard function_expected: the head resolved to Add — Init's declaration; .* applied to 1 more arguments than its type has: " "$OUT/shadowed.txt" || { bad "a head shadowed by Init's: the refusal does not name the head it resolved"; grep -E 'ERROR|REFUSE' "$OUT/shadowed.txt" | head -3 | cut -c1-400; }

echo "messages_test: $checks checks, $fail failed"
exit $fail
