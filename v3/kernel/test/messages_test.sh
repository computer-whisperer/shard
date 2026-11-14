#!/usr/bin/env bash
# v3/kernel/test/messages_test.sh — what a refusal says. A loader pin states the reason its
# load must end in (v3/pins/loader/README.md); the text after the reason is no part of a pin's
# header. This test loads a set of pins, and five sources of its own, and requires of each
# record the words an author acts on: the counts a `cases` or `induction` arm is told, the
# import a horizon is attributed to, the import to write for a name past the horizon, where an
# import that raises the horizon belongs, the file a refusal inside an implementation names,
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
# each is refused for is a constant the environment lacks, and nothing a root after them
# fetches is in the environment yet. latefile.shard imports far.shard after a declaration, and
# with it a horizon past the lemma it cites
printf '%s\n' '(import Init Eq)' '(theorem t ((a Nat) (b Nat) (h (Eq a b))) (Eq b a) (by (rw h) rfl))' > "$SCRATCH/step.shard"
printf '%s\n' '(import Init Nat.sub_one_lt)' '(fn twice ((n Nat)) Nat (+ n n))' > "$SCRATCH/far.shard"
printf '%s\n' '(import Init Eq)' '(theorem a () (Eq 1 1) (exact (Eq.refl 1)))' '(import "far.shard")' '(theorem t ((n Nat) (h (Not (Eq n 0)))) (Nat.lt (Nat.sub n 1) n) (exact (Nat.sub_one_lt h)))' > "$SCRATCH/latefile.shard"
printf '%s\n' '(import Init Int.decEq)' '(fn f ((n Nat)) Nat (+ 1 (g n)))' '(fn g ((n Nat)) Nat (if (= n 0) 0 (g (- n 1))))' > "$SCRATCH/parked.shard"
printf '%s\n' '(import Init Int.decEq)' '(class Foo () Nat 1)' > "$SCRATCH/class.shard"

"$EVAL" direct v3/kernel/load.shard --root v3 $INIT "$SCRATCH/step.shard" "$SCRATCH/latefile.shard" \
  $P/by_cases_arm_fields/main.shard $P/by_cases_fields/main.shard \
  $P/init_two_imports/main.shard $P/init_horizon_inherited/main.shard \
  $P/init_import_late/main.shard $P/init_import_late_none/main.shard \
  $P/impl_read_error/lib $P/impl_load_error/lib $P/impl_import_late/lib \
  $P/define_nested/main.shard \
  $P/init_above_instance/main.shard $P/init_above_instance_fn/main.shard \
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

# ---- the import that set the horizon
has "two Init imports: the INIT record does not name the one that set the horizon" "init_two_imports" \
  "^INIT WellFounded\.fix_eq: horizon [0-9]+, [0-9]+ declarations admitted$"
has "two Init imports: above_horizon does not name the import that set the horizon" "init_two_imports" \
  "^READ-ERROR pins/loader/init_two_imports/main\.shard above_horizon: Nat\.sub_one_lt is declaration [0-9]+ of the export, at or above the module's horizon [0-9]+ \(import Init WellFounded\.fix_eq\): write \(import Init Nat\.sub_one_lt\)$"
has "an inherited horizon: above_horizon does not name the imported file" "init_horizon_inherited" \
  "^READ-ERROR pins/loader/init_horizon_inherited/main\.shard above_horizon: Nat\.sub_one_lt is declaration [0-9]+ of the export, at or above the module's horizon [0-9]+ \(inherited through the import of pins/loader/init_horizon_inherited/base\.shard\): write "
has "an inherited horizon: no INIT record with - for its import" "INIT -" "^INIT -: horizon [0-9]+, "

# ---- an import that raises the horizon after the fetch
has "a late Init import that raises the horizon: not refused with the import and where it belongs" "init_import_late/" \
  "^LOAD-ERROR pins/loader/init_import_late/main\.shard late_init_import: \(import Init Int\.decEq\) raises the module's horizon from [0-9]+ \(import Init Eq\) to [0-9]+ after the file's Init declarations were fetched: an import of Init that raises the horizon stands before the file's first declaration — move it there"
has "a late Init import in a file read without Init: not refused" "init_import_late_none" \
  "^LOAD-ERROR pins/loader/init_import_late_none/main\.shard late_init_import: \(import Init Int\.decEq\) gives the module a horizon, [0-9]+, after its first declaration was read without one: "
has "an implementation's Init import past its view's: not refused under the implementation's file" "impl_import_late" \
  "^IMPL LOAD-ERROR pins/loader/impl_import_late/lib/lib\.shard late_init_import: \(import Init Int\.decEq\) raises the module's horizon from [0-9]+ \(import Init Eq\) to [0-9]+ after the file's Init declarations were fetched: .*\(an implementation's: into its view\)$"
has "a late file import that raises the horizon: the refusal does not give the horizon of the fetch and the import to move" "$S/latefile.shard" \
  "^READ-ERROR $S/latefile\.shard above_horizon: Nat\.sub_one_lt is declaration [0-9]+ of the export, at or above the horizon [0-9]+ \(import Init Eq\) the file's Init declarations were fetched at; the horizon was raised to [0-9]+ \(inherited through the import of $S/far\.shard\) after that, by a file import that follows a declaration, and the names this file cites were not fetched again: move that import before the first declaration"

# ---- the file a refusal inside an implementation names
has "an implementation's refused form: the record does not name the implementation's file" "impl_read_error" \
  "^IMPL READ-ERROR pins/loader/impl_read_error/lib/lib\.shard rfl_failed: "
has "an implementation's failed import: the record does not name the implementation's file" "impl_load_error" \
  "^IMPL LOAD-ERROR pins/loader/impl_load_error/lib/lib\.shard import_failed: pins/loader/impl_load_error/lib/absent\.shard$"
lacks "a record of an implementation's check that names the view's file" "^IMPL (READ|LOAD)-ERROR [^ ]*mod\.req\.shard "

# ---- a name past the horizon that the source does not spell
has "an instance past the horizon: a theorem is not refused with the import to write" "init_above_instance/" \
  "^READ-ERROR pins/loader/init_above_instance/main\.shard above_horizon: Nat\.instMax is declaration [0-9]+ of the export, .*: write \(import Init Nat\.instMax\)$"
has "an instance past the horizon: a fn is not refused with the import to write" "init_above_instance_fn" \
  "^REFUSE pins\.loader\.init_above_instance_fn\.main\.m above_horizon: Nat\.instMax is declaration [0-9]+ of the export, .*: write \(import Init Nat\.instMax\)$"
has "a step's lemma past the horizon: K's refusal is not named with the import that reaches it" "$S.step" \
  "^REFUSE $S\.step\.t above_horizon: Eq\.mpr is declaration [0-9]+ of the export, .*: write \(import Init Eq\.mpr\)$"

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
has "nested recursion: the refusal does not name the let the argument cites" "define_nested" \
  "^REFUSE pins\.loader\.define_nested\.main\.f nested_recursion: .* it cites r, a let bound to the result of a recursive call"
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
has "a function waiting on a refused one: not labelled callee_undeclared" "$S.parked" \
  "^RUNNABLE fn $S\.parked\.f why=callee_undeclared route=e_first$"
lacks "a parked function labelled mutual_recursion" "why=mutual_recursion"
checks=$((checks+1)); grep -Eq "^READ-ERROR main\.shard function_expected: the head resolved to Add — Init's declaration; .* applied to 1 more arguments than its type has: " "$OUT/shadowed.txt" || { bad "a head shadowed by Init's: the refusal does not name the head it resolved"; grep -E 'ERROR|REFUSE' "$OUT/shadowed.txt" | head -3 | cut -c1-400; }

echo "messages_test: $checks checks, $fail failed"
exit $fail
