# `v3/pins/loader` — the loader's cases

Each directory is a small package. `CASE/main.shard` is the file the loader is given, its imports
resolve under `CASE/`, and its first line states the outcome the load must have. A case reads
from the pinned export of Init, on demand, what it needs.

`v3/kernel/test/loader_pins_test.shard` replays every case in its list and compares the outcome with
the header; `v3/test.sh` runs it, and `loader_pins_list_test.sh` requires that the list and this
directory name the same cases. `v3/kernel/test/loader_test.shard` holds the cases a header cannot
state (two root files in order, a wrong export, a load with no export, the records' text, a
loaded program run). Some shell tests read the records of a case: `define_test.sh`
(`route_callee`, `discharge_measure`, `registry_native`, `decidable_cells`, `mutual_measure`,
`derive_refusals`), `derive_test.sh` (`derive_size`, `derive_size_plain`, `derive_size_group`),
`wire_test.sh`
(`wire_*`), `template_test.sh` (`template_e_first`), `messages_test.sh` (the text after the reason:
`by_cases_arm_fields`, `by_cases_fields`,
`impl_read_error`, `impl_load_error`, `match_dead_row`, `match_dead_row_def`, `ev_dead_row`,
`define_nested`, `lean_mutual`, `reserved_instance`, `bool_bare`, `shadowed_head`; a type's record:
`type_group`, `type_group_arity`, `type_e_only`),
`parity_test.sh` (`view_basic`, `record_basic`), `native_tie_test.sh`
and `lower_test.sh` (`basic`, `refused`, `read_error`, `view_basic`). Some harnesses run a case's
program and compare values: `entry_test.shard` (`entry`, `entry_s`, `entry_shape`, `handle_door`),
`handle_test.shard` (`handle_door`, `entry`, `entry_s`, `row_types`), `rows_test.shard`
(`ev_string`, `row_types`, `struct_proj`), `project_test.shard` (`project_bool`),
`matcher_test.shard` (`match_def`, `match_imposter`, `match_poly`).

## The header

| line | the load's outcome |
|---|---|
| `;; expect: ok` | every record is an acceptance (a declaration accepted, a parameter, a function defined or left a program, a realization, a template, an instance); at least one |
| `;; expect: NAME refused REASON` | the first record that is not an acceptance refuses `NAME` for `REASON` |
| `;; expect: NAME refuses REASON` | some record refuses `NAME` for `REASON`, not necessarily the first bad one |
| `;; expect: NAME policy AXIOM…` | `NAME`'s assumptions reach `AXIOM…`, outside the file's policy |
| `;; expect: NAME pending REASON…` | `NAME` is pending: `sorry` (with `goals=N` and the goals), `measure`, `auto_missing fp=…`, `auto_stale PART fp=…` |
| `;; expect: NAME runnable REASON` | every record accepted, and the function `NAME` is a program without a definition for `REASON` |
| `;; expect: NAME param KIND` | every record accepted, and `NAME` is a declared parameter of `KIND` |
| `;; expect: NAME threaded` | every record accepted, and `NAME`'s use of the `World` is judged `ok` |
| `;; expect: NAME unthreaded REASON` | every record accepted, and the verdict is `world_twice`, `world_unsupported` or `world_forged` |
| `;; expect: NAME bin ok` | every record accepted, and the bin `NAME` has every requirement met |
| `;; expect: NAME bad-bin REASON` | some record is the bin's failure: `unmet`, `unthreaded`, `bad_entry`, `extern_outside`, `untrusted` |
| `;; expect: NAME impl-error REASON` | a view's implementation fails its check |
| `;; expect: read-error REASON` | a file's reading ends in `REASON` |
| `;; expect: load-error REASON` | the loader refuses; `load-error-after REASON` when another bad record stands before it |
| `;; roots: FILE…` | what the loader is given, in order (default `main.shard`); a directory is its view, then its implementation checked |
| `;; root: DIR` | the package root (default the case's directory) |

## The cases

A case expects `ok` unless it is listed as refused. A reason in parentheses belongs to the cases
named before it, back to the previous `;` or `)`. A refused case with no reason shown is refused for
the reason its name spells, less a leading `by_`.

### Modules, names and Init

| cases | what they pin |
|---|---|
| `basic`; refused: `no_open`, `selective` (`unknown_constant`) | an import makes names citable in full and opens nothing; `(use a)` opens a prefix, `(use a Nat)` opens it for one name |
| refused: `ambiguous` (`ambiguous_name`); `same_spelled` (`ambiguous_type`) | a bare name two opened prefixes declare is never chosen silently; Init's `Bool` beside the file's own |
| `visible` | a file sees what its own imports reach, whatever else the load has admitted |
| refused: `cycle` (`import_cycle`), `missing` (`missing_file`), `outside` (`import_outside_root`), `collision` (`module_collision`) | the import errors; a file `lib.shard` beside a directory `lib/` |
| refused: `ev_duplicate`, `qualified_collision` (`duplicate_name`); `qualified_collision_l` (`already_declared`) | one name declared twice: in a file, by two modules that build the same name, and in K |
| `type_opens`, `type_ctor_name`, `type_ctor_theorem` | a `type` opens its constructors for its file; a type whose constructor bears its name |
| refused: `read_error` (`unterminated_list`); `retry` (`bad_constructor`) | a file that does not read; a file whose reading failed is loaded once |
| `init`, `init_nested`; refused: `init_import_line` (`bad_import`) | Init's names are bare; two files of one load that cite Init; `(import Init)` is no form, and the refusal says to delete the line |
| `init_step_lemma`, `impl_init_demand`, `init_late_file_import`, `init_use_late`, `init_fetch_second` | what a file is given of Init: the lemmas a proof step builds on, an implementation's own citations, a name cited after a late file import or under a late `use`, a constant the toolchain builds by name from a type (fetched for a second reading) |
| refused: `init_name_taken_l`, `registry_native`, `def_takes_fn`, `realize_name_taken`, `derive_name_taken` (`name_taken`) | a declaration may not take the name of one of Init's, of a function that is a program only, or of a generated declaration |
| `type_in_logic`, `type_forward`, `type_group`, `type_group_records`, `type_chain`, `type_group_params`; `type_group_arity` (`runnable block_parameters`); refused: `type_e_only` (`unknown_constant`) | a type whose fields are types of the logic is a logical type, and so is one that names types declared below it, with them: a type named above its form, two types that name each other, a group of records with their accessors, a type naming such a pair, a group with a parameter; types that name each other at different parameters are runtime-only types, and so is one that names a type below a `use`, which a `def` cannot cite |
| `sorry` (`pending sorry`); refused: `refused` (`type_mismatch`) | a `sorry` admits nothing; a false theorem is refused |
| `policy_axiom` (`policy`), `policy_trusts` (`param axiom`), `policy_cross` (`policy`) | an axiom needs `(trusts …)`; the policy is per file |

### Views and sealed directories

| cases | what they pin |
|---|---|
| `directory`, `view_basic`, `view_dir_form`, `view_helper` | a directory's view, its implementation checked as a second root, the `mod.req/mod.req.shard` form, a private helper type |
| `view_dir_form_private`; refused: `impl_read_error` (`rfl_failed`), `impl_load_error` (`missing_file`) | an implementation's forms are read under its own file: an import resolves from its directory where the view is in the directory form, and a refusal names the implementation's file |
| `view_fulfills_obligation`; `view_pending` (`pending sorry`) | a measured function inside an implementation discharging its own obligation; a requirement fulfilled by `sorry` |
| `view_missing_type`, `view_missing_fn`, `view_missing_fulfills`, `view_signature`, `view_arity`, `view_fulfills_unknown` (each an `impl-error`) | each way an implementation can fail its view |
| refused: `view_form`; `view_no_interface` (`missing_interface`), `view_sig_outside` (`sig_outside_view`), `view_req_scope` (`req_scope`) | the view file's own errors |
| refused: `view_eq_hidden` (`unknown_constant`), `view_rfl` (`type_mismatch`) | a consumer cannot cite an implementation's equation or compute through its body |
| `private_inside`, `sealed_module`; refused: `private_module`, `private_dir` (`private_module`) | files behind a view are private to their directory, which is one module across its files; so is a directory module nested behind it |
| `impl_e_only`, `match_in_impl`; `route_sig_callee` (`runnable no_l_meaning`); refused: `if_private` (`private_if`), `derive_sig` (`derive_type`) | a runtime-only implementation type; a matcher inside an implementation; a view function with no logical reading as a callee; an `if` and a derivation on a view's type |
| refused: `k_client_reach` (`function_expected`), `k_client_admit`, `k_client_transfer` (`unknown_head`) | a client of the checker's view cannot name what the view does not export |

### Programs: the forms and their refusals

| cases | what they pin |
|---|---|
| `ev_basic`, `runnable_fn`, `ev_type_param`, `ev_list_sugar`, `ev_nested_exhaustive`, `ev_string`, `str_e_first` | the forms of a program: a type, `match`, `let`, `if`, the primitives, a type parameter, `(list …)`, nested patterns, a string literal |
| refused: `ev_unknown_head` (`unknown_head`), `ev_unsaturated`, `ev_fn_value` (`type_mismatch`), `ev_ambiguous` (`ambiguous_name`), `ev_nonexhaustive` (`match_not_exhaustive`), `ev_pattern_type` (`pattern_constructor`), `ev_no_l_meaning` (`unknown_constant`) | the refusals of a body |
| refused: `match_dead_row`, `match_dead_row_def`, `ev_dead_row` (`unreachable_arm`) | a row that matches every value — `_`, or a name no constructor bears, such as a misspelt one — with rows after it: in a `fn`, in a `def`, in a function that is a program only; a capitalised variable in a last row stays a variable |
| refused: `bool_bare` (`unknown_constant`), `shadowed_head` (`function_expected`), `lean_mutual` (`lean_form`), `reserved_instance` (`reserved_form`) | refusals that say what to write: a bare `true`, a bare constructor that is Init's class of the same name, `mutual`, `instance` |
| `if_ok`; refused: `if_one`, `if_type` (`if_type`) | an `if` takes a `Bool`, a two-constructor type or a comparison; not a one- or three-constructor type, not a `Nat` |
| `dif_ok`; refused: `dif_type`; `dif_scope` (`unchecked_proof`) | the dependent `if` and its hypothesis's scope |
| `bool_ops`, `op_expected`, `op_int_div`, `define_nat_operator`, `rename_ok` | `and`, `or`, `not` at `Bool`; an operator takes the expected type's meaning; `/` and `%`; `-` at `Nat`; `< <= =` |
| refused: `rename_refused`, `rename_refused_e`, `rename_refused_kernel`, `rename_refused_ediv` (`renamed_primitive`) | `lt`, `le`, `int_eq`, and the back end's `ediv` and `tmod`, are refused with the spelling to write |
| `bit_ops`; refused: `bit_shift_count` (`type_mismatch`), `reserved_module` (load error) | the bit-level operators by their operand's type, definitions K computes; a shift's count a `Nat`; the base's module no file's |
| `literal_pattern`; `literal_nested` (`runnable no_l_identity`); refused: `literal_rows`; `literal_open` (`match_not_exhaustive`), `literal_type` (`pattern_type`), `match_literal` (`literal_pattern`) | numeral rows in a `match` |
| refused: `str_bad_utf8` (`bad_string`) | a string literal is valid UTF-8 |
| `row_types`, `struct_proj`, `refined_row`; `row_e_first` (`runnable no_l_identity`); refused: `row_e_match` (`pattern_type`) | Init's types that run as their one field (`Fin`, `UInt8`, `ByteArray`, `String`, `Subtype`); a projection; a row built in a body with no L reading |
| refused: `lean_form`, `lean_notation` (`lean_form`), `lean_symbol`, `lean_by_term` (`lean_symbol`), `infix_form`; `inhabited_default`, `inhabited_default_by` (`inhabited_default`), `absurd_e_first` (`absurd_e_first`) | Lean spellings refused with a pointer: `partial`, `notation`, `==`, a `by` term, `(n + 1)`, `List.get!` |
| refused: `quote_refused` (`no_quote`) | there is no quote form: `(quote x)` in a body is refused with the spelling to write, `(sym "x")` over the toolchain's ordinary `Symbol` (the mark `'x` is the reader's refusal: `sexpr_test.shard`) |
| `zero_arity_bare` | a function of no parameters written bare is its call |

### Records, structures, elaboration and matchers

| cases | what they pin |
|---|---|
| `record_basic`, `record_laws`; refused: `record_make` | `record`, `make`, `with`, the accessors and updaters; get-after-set by `Eq.refl`; a `make` missing a field |
| `struct_make`; refused: `dependent_update`; `struct_params` (`record_make`) | `make` and `with` on a structure; one with parameters; an update that would need a cast |
| `elab_implicit`, `elab_numeral`, `elab_bridge`, `elab_confusion`, `ctor_expected`, `one_meaning`, `one_spelling`, `instance_table` | implicit arguments, numerals, a proposition where a `Bool` is expected, `noConfusion` and injectivity, a constructor at an expected type, one expression as a `fn` body and a `def`, class methods read as their functions |
| refused: `elab_unsolved` (`unsolved_implicit`), `elab_instance` (`instance_needed`), `elab_no_decision` (`no_decision`), `elab_mismatch` (`type_mismatch`) | the elaborator's refusals |
| `match_def`, `match_statement`, `match_poly`, `match_imposter`, `match_dependent`, `dependent_match`, `nested_match`, `def_match`, `matcher_view`, `project_bool`, `decidable_cells` | a `match` as a generated matcher: in a `def`, in a statement, over a type parameter, recognized by regeneration, with dependent locals, on a nested type; a decision as a value |
| refused: `match_not_exhaustive`, `match_motive` | the matcher's refusals |

### A function as a definition

| cases | what they pin |
|---|---|
| `define_basic`, `define_below`, `define_depth`, `define_forward`, `define_arg_match`, `if_rec_call`, `def_struct` | a function defined with its equations: non-recursive, structural, two constructors deep, calling one declared later, a self-call under a `match` or in an `if`'s condition; a `def` with a recursion clause |
| `define_runnable`; `route_callee` (`runnable callee_runnable`); `define_refused` (`runnable realize_recursion`), `define_descent` (`runnable realize_descent`); refused: `callee_refused` (`callee_undeclared`) | a function left a program with its reason; a caller of one; a self-call without a measure and a structural clause that is not one, each a program and no definition; the callers of a refused function, refused in turn |
| `fn_named_below` | functions named `below`, `brecOn`, `fix`, `casesOn` and their caller are defined: only the generated constants of an inductive type and Init's fixpoints are read as such |
| `define_measure`, `measure_natabs`, `measure_if_call` (`pending measure`); `define_measure_int` (`runnable measure_type`) | a measured function and its obligation; a measure through `Int.natAbs`; an `Int` measure |
| `measure_parts`, `measure_parts_group`; `measure_parts_uneven` (`runnable measure_type`) | a measure of several parts, compared part by part: a function whose call may refill its list when its counter falls, with the pair written out too; a group measured by a depth, a rank and a length; a group whose members state different numbers of parts |
| `define_let_call`, `mutual_let_call`; `define_nested` (`runnable nested_recursion`) | a `let` bound to a recursive call with another call in its scope, alone and in a group: the later obligation is stated without the let; a call whose argument cites such a let |
| `discharge_measure`, `define_match_eq`; refused: `discharge_self_cycle`, `discharge_two_cycle` (`fulfills_cycle`) | obligations discharged; an obligation stated under its match's equation; a discharge that rests on itself |
| `mutual_measure`, `mutual_forward`, `mutual_three`; `mutual_pending` (`pending measure`); `mutual_no_measure`, `define_mutual` (`runnable group_measure`), `mutual_shape` (`runnable group_shape`) | functions that call each other as one definition; what leaves them programs |
| `mutual_forward_after`, `mutual_callee_settles`, `mutual_type_between`, `mutual_use_between` | a group is found whatever is read between its members and after them: a function a member names after its call of the other, read and defined later; a function a member took as a variable while it waited, defined since; a type a member builds a value of, declared after both; a namespace opened after both. The search for a group keeps how a reading ended, and each of these ends what it kept |
| `fulfills_refused`, `fulfills_not_obligation`, `fulfills_outside` (each a load error) | a `fulfills` of a refused function's obligation, of a declaration, of nothing |
| `realize_supplied`, `realize_native`, `realize_view`, `realize_equations`, `realize_param_use`, `realize_theorem`, `realize_imported` | a program attached to a declaration of Init: by a supplied body, by Init's own definition, with named equation theorems; theorems cited before and after |
| `realize_pending`, `realize_pending_via` (`pending measure`) | a realization under a measure not yet discharged, and a caller of one |
| refused: `realize_recursion`, `realize_primitive`, `realize_signature`; `realize_wrong` (`conversion`), `realize_loop`, `realize_measure_loop` (`realize_descent`), `realize_brecon` (`recursion_structure`), `realize_order` (`no_realization`), `realize_fn` (`no_l_meaning`), `realize_prop` (`realize_signature`), `realize_unknown` (`unknown_constant`), `realize_dup` (`duplicate_name`) | each way a realization is refused |

### Deriving and templates

| cases | what they pin |
|---|---|
| `derive_eq`, `derive_containers` | equality derived and used by an `if`; Init's `List`, `Option`, `Prod` at closed arguments |
| refused: `derive_field`, `derive_needs`, `derive_policy`, `derive_duplicate`, `derive_by`; `derive_nested`, `derive_refusals`, `derive_group_eq` (`derive_shape`), `derive_params` (`derive_needs`), `derive_e_first` (`prim_type`), `derive_visible` (`no_decision`) | the derivation's refusals — a type nested through another and types that name each other have a size and no other derivation; a derivation is visible where its module is |
| `derive_size`, `derive_size_plain`, `derive_size_group`; refused: `derive_size_mixed` (`bad_shape`), `derive_size_twice` (`name_taken`) | a size derived for a type nested through `List`, for a plain type and for types that name each other (a size each, whichever the form names), used as measures |
| `template_lift`, `template_named`, `template_partial`, `template_passed`, `template_package` | a function-typed binder: a lambda lifted, a named function, a partial application, a parameter passed on, a structure of functions |
| `template_e_first` (`runnable no_l_identity`); refused: `template_escape`, `template_transform` (`function_value`), `template_growth` (`realize_descent`), `template_budget` (`specialization_budget`) | a lambda elsewhere; a template calling itself at a growing type; the instance budget; an instance taken by a body with no logical reading |

### Proof steps and sidecars

| cases | what they pin |
|---|---|
| `by_core`, `by_wf`, `by_witness`, `by_case_step`, `by_cases_local_eq`, `by_cases_dependent`, `by_sibling_names`, `by_exact_external`, `by_rw_occ_spine`, `by_ne_level`, `by_reduce` | every step on small goals; well-founded induction; a witness given by a premise; `cases` on a term and with dependent hypotheses; an external term through `exact` |
| `by_sorry` (`pending sorry goals=2 …`) | the open goals are shown in the record |
| refused: `by_goal_open`, `by_goal_closed`, `by_unknown_step`, `by_intro_no_pi`, `by_rfl_failed`, `by_apply_goals`, `by_witness_open`, `by_cases_rows`, `by_induction_eq`, `by_rw_no_match`, `by_unfold_stuck`, `by_decide_failed`, `by_show_mismatch`; `by_cases_fields` (`induction_fields`), `by_cases_arm_fields` (`cases_fields`), `by_rw_occ` (`rw_no_match`), `by_rw_occ_guard` (`rw_occ_changed`) | each step's refusal, a read error |
| `arith_core`, `arith_div`, `arith_only`, `arith_eq_bare`; refused: `arith_failed`, `arith_certificate`, `arith_goal`, `arith_fact` | linear arithmetic and its refusals |
| `simp_core`; refused: `simp_no_progress`, `simp_budget`, `simp_self` | `simp_only` and its refusals |
| `reduce_using`, `reduce_dependent`, `reduce_let`, `reduce_keep`; refused: `reduce_rule_unused`, `reduce_rule_open`, `reduce_keep_unused`; `reduce_rule_fn` (`rw_not_eq`), `rw_dependent` (`rw_motive`) | `reduce` with facts; a rewrite at an occurrence beside a proof that states it; a `let` opened where the case analysis under it waits on an open condition; a kept function's calls left as written, and a kept name that held none |
| `auto_core`, `auto_tactic`, `auto_hints_changed`, `auto_strategy_free` | a theorem proven by `auto` replays its sidecar's entry; hints and binder names are outside the key |
| `auto_fulfills`, `auto_other_root` | an obligation delegated with `auto` is discharged from the entry under its name, in a file with no `auto` theorem; an entry made for a file loaded alone fits when another root has admitted a parameter the file does not see |
| `auto_impl` | `auto` inside the implementation of a view — a requirement's `fulfills`, a theorem, an obligation — replays the sidecar beside the implementation file, `lib/lib.auto.shard` |
| `auto_missing`, `auto_stale`, `auto_stale_deps`, `auto_stale_policy`, `auto_stale_env` (`pending`) | no entry; an entry made for another statement, dependency, policy or export of Init |
| `auto_schema_old`, `auto_schema_2`, `auto_recon_old`; refused: `auto_schema_future` (`sidecar_schema`), `auto_malformed` (`sidecar_malformed`) | older sidecar formats replay and are reported; a newer or malformed one is a load error |

### Entries, host calls and binaries

| cases | what they pin |
|---|---|
| `entry`, `entry_s`, `entry_shape`, `handle_door` | functions run from outside with checked arguments; what the check does not cover on a program alone (an evidence binder, a proposition field), which a load's run refuses |
| `wire_s`, `wire_bad`, `wire_bad_result` | host calls over Init's types; one over a type with no encoding; two with a wrong result |
| `thread_branches`, `thread_helper_ok`, `thread_unused`, `thread_proof_ref` (`threaded`) | a `World` used once per branch, through a helper, left unused, named only in an erased proof |
| `thread_sequential`, `thread_pair_twice`, `thread_helper_twice` (`world_twice`); `thread_match_world`, `thread_poly` (`world_unsupported`); `thread_forged` (`world_forged`) | a `World` used twice, taken apart, passed to a polymorphic function, or built by a program |
| `bin_ok` (`bin ok`); `bin_unmet`, `bin_unthreaded`, `bin_bad_entry`, `bin_extern_outside`, `bin_untrusted` (`bad-bin`) | the `bin` form and each way it fails |
