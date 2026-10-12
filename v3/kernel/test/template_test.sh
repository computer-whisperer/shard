#!/usr/bin/env bash
# v3/kernel/test/template_test.sh — T2 and T3 as a test (v3/LANGUAGE.md §8.4 slice 3.28 rule 7;
# law §12.5). Over v3/examples/template: the four files load in one process — std/list's six
# templates (TEMPLATE records, no program of their own), the law's `add_offset` taking `List.map`
# at a lifted lambda with its captured value leading, a named fn and a named partial application
# as static arguments, a package at Int with its generic theorems proven once over the package
# parameter and applied at the constants, a package's operation passed on to another template;
# the theorems over the instances' callers hold by Eq.refl through the unfolding; T3's large
# finite workload — 64 instances at 64 lifted lambdas in one file, within the budget (landing 3).
# The entry runs under --run and prints the instances' results as one line of digits. Then the
# ARTIFACT check:
# the programs dumped (--dump) hold no lambda, no function type and no partial application —
# the dump's grammar has none (calls saturated by construction); the check is that no `fun` and
# no `->` appears in any program line while the source files hold the lambdas — T2's risk column
# checked on the artifact, not read off the source. Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "template_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "template_test: $*"; fail=$((fail+1)); }
load() { "$EVAL" direct v3/kernel/load.shard --root v3 $INIT "$@"; }
EX=v3/examples/template
FILES=("$EX/add_offset.shard" "$EX/named.shard" "$EX/package.shard" "$EX/workload.shard" "$EX/template_main.shard")
ENTRY=examples.template.template_main.main
secs() { echo $(( $(date +%s) - $1 )); }

# ---- the load -------------------------------------------------------------------------------
s=$(date +%s); load "${FILES[@]}" > "$OUT/load.txt" 2>&1; rc=$?; t_load=$(secs $s)
checks=$((checks+1)); [ "$rc" = 0 ] && grep -q "^LOAD: modules 8 .* refused 0  pending 0 .* errors 0$" "$OUT/load.txt" || { bad "the examples do not load clean (exit $rc)"; grep -E 'ERROR|REFUSE|PENDING|^LOAD:' "$OUT/load.txt" | head -5 | cut -c1-300; }
# the six templates of std/list, the package template, and no program of their own
for t in "List.map static=f" "List.filter static=p" "List.foldl static=f" "List.foldr static=f" "List.any static=p" "List.all static=p" "examples.template.package.fold_m static=m" "examples.template.package.scale_all static=m"; do
  checks=$((checks+1)); grep -q "^TEMPLATE $t " "$OUT/load.txt" || bad "missing record: TEMPLATE $t"
done
# the instances: a lifted lambda with its captured value, a name, a partial application, the registry's Int.add, a package constant, a package's field
for i in "List_map_Int_Int.of.examples_template_add_offset_add_offset_lam_1_1 of=List.map shapes=examples.template.add_offset.add_offset.lam_1/1" \
         "List_map_Int_Int.of.examples_template_named_inc_0 of=List.map shapes=examples.template.named.inc/0" \
         "List_map_Int_Int.of.examples_template_named_addk_1 of=List.map shapes=examples.template.named.addk/1" \
         "List_foldl_Int_Int.of.Int_add_0 of=List.foldl shapes=Int.add/0" \
         "List_foldr_Int_Int.of.Int_add_0 of=List.foldr shapes=Int.add/0" \
         "List_filter_Int.of.examples_template_named_big_0 of=List.filter shapes=examples.template.named.big/0" \
         "List_any_Int.of.examples_template_named_big_0 of=List.any shapes=examples.template.named.big/0" \
         "List_all_Int.of.examples_template_named_big_0 of=List.all shapes=examples.template.named.big/0" \
         "examples_template_package_fold_m_Int.of.examples_template_package_IntAdd_0 of=examples.template.package.fold_m shapes=examples.template.package.IntAdd/0" \
         "examples_template_package_fold_m_Int.of.examples_template_package_IntMul_0 of=examples.template.package.fold_m shapes=examples.template.package.IntMul/0" \
         "examples_template_package_scale_all_Int.of.examples_template_package_IntAdd_0 of=examples.template.package.scale_all shapes=examples.template.package.IntAdd/0" \
         "List_map_Int_Int.of.Int_add_1 of=List.map shapes=Int.add/1"; do
  checks=$((checks+1)); grep -q "^INSTANCE [a-z_.]*$i " "$OUT/load.txt" || bad "missing record: INSTANCE …$i"
done
# the lifted lambda defined, the theorems over the callers accepted (Eq.refl through the unfolding), the generic theorems applied at the constants
for d in "DEFINE examples.template.add_offset.add_offset.lam_1 " "ACCEPT examples.template.add_offset.add_offset_cons " "ACCEPT examples.template.named.incs_cons " "ACCEPT examples.template.named.shift_cons " "ACCEPT examples.template.named.total_r_cons " \
         "ACCEPT examples.template.package.fold_m_nil " "ACCEPT examples.template.package.sum_int_nil " "ACCEPT examples.template.package.sum_int_cons " "ACCEPT examples.template.package.prod_int_nil " "ACCEPT examples.template.package.int_add_unit "; do
  checks=$((checks+1)); grep -q "^$d" "$OUT/load.txt" || bad "missing record: $d"
done
# the workload (T3): its 64 instances, each at its own lifted lambda, all built in this load
n_wl=$(grep -c "^INSTANCE examples.template.workload.List_[a-z]*_Int[_A-Za-z]*.of.examples_template_workload_[mfal][0-9]*_lam_1_0 " "$OUT/load.txt")
checks=$((checks+1)); [ "$n_wl" = 64 ] || bad "the workload took $n_wl instances, not 64"
checks=$((checks+1)); [ "$(grep -c '^DEFINE examples.template.workload.[mfal][0-9]*.lam_1 ' "$OUT/load.txt")" = 64 ] || bad "the workload's lambdas are not 64 lifted fns"
# the E-first instance (rule 3's last sentence): main takes List.map at digit with no equations
checks=$((checks+1)); grep -q "^INSTANCE examples.template.template_main.List_map_Int_UInt8.of.examples_template_template_main_digit_0 of=List.map shapes=examples.template.template_main.digit/0 equations=$" "$OUT/load.txt" || bad "missing record: the E-first INSTANCE at digit with no equations"
checks=$((checks+1)); grep -q "^RUNNABLE fn examples.template.template_main.main" "$OUT/load.txt" || bad "the entry is not RUNNABLE"
# a package type is never an E type: the structure has no E twin, no program mentions it
checks=$((checks+1)); grep -q "^RUNNABLE examples.template.package.Monoid" "$OUT/load.txt" && bad "the package type Monoid was taken as an E type"

# ---- the run --------------------------------------------------------------------------------
s=$(date +%s)
checks=$((checks+1)); [ "$(load --run $ENTRY "${FILES[@]}" 2>&1 | tail -1)" = "9662014509" ] || { bad "the entry did not print 9662014509"; load --run $ENTRY "${FILES[@]}" 2>&1 | tail -3 | cut -c1-300; }
t_run=$(secs $s)

# ---- the artifact (T2's risk column, on the programs) ----------------------------------------
s=$(date +%s); load --dump "${FILES[@]}" > "$OUT/dump.txt" 2>&1; rc=$?; t_dump=$(secs $s)
checks=$((checks+1)); [ "$rc" = 0 ] && grep -q "^fn " "$OUT/dump.txt" || bad "the programs did not dump (exit $rc)"
n_src=$(cat "${FILES[@]}" v3/std/list.shard | grep -c "(fun \|(-> ")
checks=$((checks+1)); [ "$n_src" -ge 68 ] || bad "the sources hold fewer lambdas and function types than expected ($n_src)"
n_art=$(grep -c "(fun \|->" "$OUT/dump.txt")
checks=$((checks+1)); [ "$n_art" = 0 ] || { bad "a program holds a lambda or a function type ($n_art lines)"; grep "(fun \|->" "$OUT/dump.txt" | head -3 | cut -c1-200; }
# the instances and the lifted lambda are programs of the artifact; the templates are not
checks=$((checks+1)); grep -q "^fn [A-Za-z_.]*List_map_Int_Int.of.[A-Za-z_.]*add_offset_lam_1_1 " "$OUT/dump.txt" || bad "the lambda's instance is not a program of the artifact"
checks=$((checks+1)); grep -q "^fn [A-Za-z_.]*add_offset.lam_1 " "$OUT/dump.txt" || bad "the lifted lambda is not a program of the artifact"
checks=$((checks+1)); grep -q "^fn [A-Za-z_.]*fold_m_Int.of.[A-Za-z_.]*IntAdd_0 " "$OUT/dump.txt" || bad "the package instance is not a program of the artifact"
checks=$((checks+1)); grep -q "^fn [A-Za-z_.]*List_map_Int_UInt8.of.[A-Za-z_.]*digit_0 " "$OUT/dump.txt" || bad "the E-first instance is not a program of the artifact"
checks=$((checks+1)); grep -q "^fn List.map \|^fn [A-Za-z_.]*fold_m [0-9]" "$OUT/dump.txt" && bad "a template has a program of its own"
n_inst=$(grep -c "^fn [A-Za-z0-9_.]*\.of\." "$OUT/dump.txt")
checks=$((checks+1)); [ "$n_inst" -ge 80 ] || bad "fewer than 80 instances in the artifact ($n_inst)"
echo "template_test: $n_inst instances in the artifact, $n_src lambdas and function types in the sources, none in the programs"
echo "template_test: load $t_load s, run $t_run s, dump $t_dump s"
echo "template_test: $checks checks, $fail failed"
exit $fail
