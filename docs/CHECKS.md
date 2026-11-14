# Rules and the tests that hold them

Two tables for anyone changing the toolchain. The first lists what the system must do, in more
detail than `LAW.md` §3, and the test or pin that holds each rule; "no test found" is a gap, and the
gaps are collected in `OPEN.md` §4. The second lists the soundness mistakes made so far, in this
generation and the last, and the test that would catch each kind again. A soundness bug is fixed
and pinned (`LAW.md` §5): a new one adds a row here.

Tests are under `v3/kernel/test/` and pins under `v3/pins/loader/` unless a path is given.

## Rules


### The checker K

| rule | held by |
|---|---|
| K implements the rules of the pinned Lean 4 kernel exactly; a departure is a decision that names what it gives up. | `v3/t0_full.sh` against `v3/t0_expected.txt`; `t0_fixture_test.sh` |
| K's verdict and axiom closure for a declaration are Lean's. | `v3/t0_axioms_cmp.sh`; `t0_fixture_test.sh` |
| K refuses hostile input, each case for a stated reason. | `hostile_test.shard` |
| The standard profile has three axioms: `propext`, `Quot.sound`, `Classical.choice`. | pins `policy_axiom`, `policy_cross`, `policy_trusts`, `init_name_taken_l`; `std_test.sh` |
| K answers Accepted, Rejected with one of five reasons, or Exhausted; Exhausted is neither; every negative names its subject. | `v3/kernel/verdict.shard`; `hostile_test.shard` case 12; `t0_gate_test.sh` |
| A "not established" answer is never read as a disproof and never widened to a region of candidates. | no test found |
| K holds no unification, elaboration, type classes, tactics, arithmetic procedure, measure recogniser, file access or global state. | no test of the absence; clients reach K through its view only (pins `private_module`, `k_client_reach`; `k_clients_test.sh`) |
| A checked environment comes only from admission and cannot be built or forged by a client. | `v3/kernel/k/mod.req.shard`; pins `k_client_reach`, `k_client_admit`, `k_client_transfer` |
| What a client hands K is a claim: K re-derives every node's data before reading it. | `hostile_test.shard` cases 16, 19, 19b, 20, 21 |
| A shortcut for a primitive or a literal is enabled only for an admitted declaration that matches the reference structurally, never by name. | `v3/kernel/k/test/accel_test.shard`; `hostile_test.shard` cases 7, 7b, 7e, 8, 8b |
| Building declarations as data through K's interface is a normal route. | `k_client_test.shard`; `session_test.shard` |
| Acceptance by K implies the judgment holds. | not proven; the evidence is the comparison with Lean |

### Programs (E)

| rule | held by |
|---|---|
| A running program holds no function value: a function-typed expression is a call head or a template argument, nothing else. | pins `template_escape`, `template_transform`, `ev_fn_value`; `template_test.sh` |
| Specialization is finite; exceeding the budget is a refusal. | pins `template_growth`, `template_budget` |
| The toolchain is itself first-order, closure-free programs over the logic as data. | `parity_test.sh`; `v3/build.sh` compiles every root |
| The toolchain's functions are total by verified measure. | no test found, and no load checks it: in a file that sees no Init a measure that grows, a recursion with no measure and a `struct` call on no subterm each load without a record of it (`RUNNABLE fn F why=e_only_type`); in a file that imports Init the same functions state an obligation or are refused (`realize_descent`) |
| A `fn` either lowers or is refused. | no test found; `v3/pins/emit/refused_*` show four programs the interpreter runs and the back end refuses |
| One name for one mathematical object, in a body, a definition and a theorem. | pins `one_meaning`, `one_spelling`, `rename_refused`, `rename_refused_e`, `rename_refused_kernel` |
| A `fn`'s meaning in L is its body's elaboration; its defining equations are theorems K checks. | pins `define_basic`, `define_depth`, `realize_equations`; `define_test.sh` |
| A wrong executable body is refused whatever its provenance. | pin `realize_wrong`; `path_test.sh` |
| Progress is a separate obligation: a looping body is refused, and a group needs a joint well-founded argument. | pins `realize_loop`, `realize_measure_loop`, `define_refused`, `mutual_measure`, `mutual_no_measure` |
| Every `fn` with a meaning in L is total by its admission; an unbounded process takes fuel. | pins `define_measure`, `discharge_measure`, `by_wf`; `ev_test.shard` |
| Proofs and static structure are erased or specialised; runtime data is kept. | pins `realize_param_use`, `realize_prop`, `handle_door`, `template_package` |
| A subtype over any proposition is a runtime value of its base type. | pin `refined_row`; `std_test.sh` |
| A decision keeps its tag and drops its proof; a proof used inside one branch costs nothing at run time. | pins `dif_ok`, `dif_scope`, `project_bool`, `decidable_cells` |
| Erasure never produces a runtime value that exists only by noncomputable choice. | refusal `noncomputable` in `v3/kernel/erasure.shard`; no pin found |
| A definition is not executable until a realization is attached under its existing identity. | pins `realize_supplied`, `realize_view`, `realize_order`, `realize_dup`, `realize_theorem`, `realize_imported` |
| An imported name is never identified with a native declaration by spelling. | pins `same_spelled`, `init_name_taken_l`, `registry_native`, `realize_name_taken` |
| The shared mathematical types are the pinned `Init`'s, imported once and given representations. | pins `row_types`, `refined_row`, `str_bad_utf8`; `std_test.sh` |
| A conditional or approximate realization is selected only with its conditions discharged, and the selection is recorded. | no test found; no mechanism found |
| A quotient's lift runs the underlying function and erases its respect proof. | no test found |
| `ev` defines "run"; routes are compared on results and observations. | `ev_test.shard`, `route2_test.sh`, `lower_test.sh`, `native_tie_test.sh`, `suite_tie.sh` |
| A native result is not a proof; a result's evidence is a computation K checks. | no test found; not built |
| A `bin`'s closure uses each World token at most once on every path, through aggregates, helpers and branches. | pins `thread_branches`, `thread_pair_twice`, `thread_helper_twice`, `thread_forged`, `thread_proof_ref`, `bin_unthreaded` |
| Pure evaluation refuses a reachable extern. | `ev_test.shard` |
| The effect semantics is a trace; assumed effect laws are tested on a concrete model. | `bin_test.sh` (the drivers' trace and output against the model world); a compiled binary is tied on output and exit status only |
| A replacement carries the relation it preserves and its evidence; an interpreter is never inserted because lowering failed. | no test found |
| Division by zero is a defined value; `Nat` subtraction saturates. | `prims_test.shard`; `std_test.sh` (`std/migration.shard`); `v3/pins/emit/init_prim` |

### Elaboration and the surface

| rule | held by |
|---|---|
| The elaborator is untrusted: K checks what it produces. | release verification runs without it (`store_test.sh`) |
| Inference fills in what the inputs determine and never chooses among alternatives; an undetermined argument is a refusal. | pins `elab_unsolved`, `elab_instance`, `elab_no_decision`, `match_motive` |
| An unknown identifier is never turned into a parameter. | `v3/pins/reader/refuse_unknown_constant` |
| A requirement is identified by its statement, the declarations it refers to, the policy and the environment; a changed proof strategy leaves it alone. | pins `auto_stale`, `auto_stale_deps`, `auto_stale_policy`, `auto_stale_env`, `auto_strategy_free` |
| A numeral takes the type its position expects, `Nat` otherwise; the one unwritten coercion between data types is `Nat → Int`; it never changes an operation already resolved. (A proposition and a `Bool` are also bridged: `LANGUAGE.md`, "Terms".) | pins `elab_numeral`, `op_expected`, `elab_mismatch` |
| A `Fin` literal that reaches its bound is refused. | no test found; no mechanism found |
| Lean's names and theorem-naming grammar are the default. | pin `one_spelling`; `std_test.sh` |
| A Lean form that hides failure or choice is refused with a pointer to its replacement. | pins `lean_form`, `lean_symbol`, `inhabited_default`, `infix_form`, `lean_by_term` |
| No user-defined notation, macro or syntax extension. | pin `lean_notation` |
| Deriving is generation under a declared policy; an unsupported derivation is refused and the type stays. | pins `derive_eq`, `derive_policy`, `derive_field`, `derive_refusals`; `derive_test.sh` |
| The surface has one canonical form; the gate refuses and does not rewrite; identity is never over source text. | `v3/tools/shardfmt/test/shardfmt_test.sh`; `reader_test.shard` |
| A diagnostic names its subject and best origin and never invents a span. | `entry_test.shard`; origins per occurrence are not built |
| K refuses a declaration that holds a metavariable. | `hostile_test.shard`; pin `by_witness_open` |
| A failed or exhausted attempt on a goal leaves no hidden assignment. | no test found; no attempt/commit interface |

### The proof language

| rule | held by |
|---|---|
| A tactic block is a certificate: replay elaborates it without search and K checks the term. | `tactic_test.sh`; pins `auto_missing`, `auto_stale` |
| `exact TERM` reaches every proof term. | pin `by_exact_external` |
| A step carries its choices or names a versioned, bounded reconstruction. | pins `arith_only`, `arith_certificate`, `simp_budget`, `auto_recon_old` |
| A rewrite addresses an occurrence together with the subterm expected there. | pins `by_rw_occ`, `by_rw_occ_guard`, `by_rw_occ_spine` |
| Schema and reconstruction versions are apart from the rules; a version change is reported, not overwritten. | pins `auto_schema_old`, `auto_schema_2`, `auto_schema_future`; `store_test.sh` |
| Proof steps are functions over data, and an engine is a program composing them. | `tactic_lib_test.sh`; pin `auto_tactic` (a library exchanges text once) |
| One engine's solution is checked the same way as any other's. | `tactic_lib_test.sh` |
| A failed proof refutes neither the statement nor the program. | no test found |
| `sorry` is reported and never accepted. | pins `sorry`, `by_sorry`, `bin_unmet`; `v3/pins/reader/pending_sorry` |
| Release verification checks stored proof terms against target and policy with no elaborator. | `store_test.sh`, `std_test.sh`, `imp_test.sh`, `path_test.sh` |
| No tool writes into an authored source; a missing or stale delegated proof is pending, never searched for at build. | `engine_test.sh`, `tactic_lib_test.sh`; pins `auto_missing`, `auto_stale` |

### Identity, views and acceptance

| rule | held by |
|---|---|
| Assumption policy is checked at every acceptance; two proofs of one statement keep their own assumption sets. | pins `policy_axiom`, `policy_cross`, `bin_untrusted`; `store_test.sh`; `tactic_lib_test.sh` |
| The checking engine, its version and the route are part of an acceptance. | no test found; no such record found |
| A view is well-formed on its own. | pins `view_req_scope`, `view_form`, `view_sig_outside` |
| An implementation gives each signature a body and each requirement a proof. | pins `view_missing_fn`, `view_missing_type`, `view_missing_fulfills`, `view_signature`, `view_pending` |
| A consumer checks against the view alone, and K cannot unfold a view's constant. | pins `view_eq_hidden`, `view_rfl`, `private_module` |
| Identity is the qualified name under the package root plus content; location is separate. | no relocation test found |
| Types never collapse by shape or spelling. | pins `same_spelled`, `collision`, `qualified_collision`, `entry_shape` |
| A changed body is a new revision; a proof made against the old one is not carried over. | pin `auto_stale_deps`; `session_test.shard` |
| Invalidation follows dependency classes, never a shared directory. | pins `auto_stale*` for stored proofs; nothing else caches |

### Execution and trust

| rule | held by |
|---|---|
| The Rust evaluator executes programs only and defines no rule and no front end of its own. | `parity_test.sh` |
| A compiled engine is confirmed against the interpreter byte for byte. | `native_tie_test.sh`, `suite_tie.sh`, `route2_test.sh`, `t0_gate_test.sh` |
| The toolchain's sources are written in the language every file uses. | `parity_test.sh`; pin `rename_refused_kernel` (string literals under `kernel` remain byte lists) |
| Every entry from outside is checked or preconditioned; a value built as data is not thereby well-typed. | `entry_test.shard`, `handle_test.shard`, `wire_test.sh`; pins `entry`, `entry_shape`, `handle_door` |
| A prepared handle keeps denoting its revision; a newer revision neither retargets nor revokes it. | `session_test.shard`, `handle_cost_test.sh` |
| Bulk buffers cross a boundary through explicit views. | no test found; no packed buffer exists |
| Limits cover decoding, sizes, depth and primitives; exhaustion changes no state and is never cached. | `hostile_test.shard`; `prims_test.shard`; `index_test.sh` |
| A long-lived context owns what it holds, and release makes it reclaimable. | no test found; not built |

## Soundness lessons

| what happened | caught again by |
|---|---|
| A user function named like a built-in ran as written while the arithmetic procedure read the built-in: `0 = 1`. | pins `same_spelled`, `registry_native`, `derive_name_taken`; `hostile_test.shard` case 7 |
| A user `extern` under a primitive's name took the primitive's value: `0 = 1`. | pin `realize_primitive`; `prog_test.shard`; no pin for a user `fn` or `extern` under an operator's spelling |
| A non-terminating definition's equation became a theorem. | pins `define_refused`, `realize_loop`, `realize_measure_loop`, `mutual_no_measure`, `discharge_self_cycle` |
| A descent recogniser inside the gate would have been trusted code. | measures are written and verified; pin `mutual_no_measure` |
| Hand-written axioms shipped false. | three axioms only (`policy_axiom`); the fifteen old axioms are theorems (`std_test.sh`) |
| An induction rule with a missing case ("zero-case induction", named only). | recursors are K's, compared with the export (`v3/t0_full.sh`); one declaration per name (`hostile_test.shard` case 9) |
| A reducer bound a multi-binding `let` in the wrong order. | `let` is sequential in both readers: `parity_test.sh`; `v3/pins/emit/nested_shadow` |
| The gate did not read one failure line shape, so must-fail tests gated nothing. | each pin's header is compared; `loader_pins_list_test.sh` compares the list of cases with the directory |
| A stamp written at the end of a build marked old code as fresh. | `build_test.sh`: a source edited while its build runs fails the build and no stamp is written; a stale stamp is refused by `v3/bin/eval` |
| A fast engine's verdict taken without the authority; agreement on a shared failure. | `native_tie_test.sh`, `suite_tie.sh`, `t0_gate_test.sh` |
| The evaluator's integers wrapped silently at 64 bits. | `prims_test.shard`; `v3/pins/emit/prims`, `refused_numeral_pattern` |
| A shortcut authorised by a hash: a forged `Nat.add` gave `Nat.add 2 3 = 5`. | `hostile_test.shard` case 7b; `k/test/accel_test.shard` |
| A theorem under a primitive's name was authorised (no false theorem). | `hostile_test.shard` case 7e |
| K trusted node data its caller wrote: two literals under one id gave `0 = 1`. | `hostile_test.shard` case 16 |
| Two environments joined through K's interface: `(p : Prop) → p` accepted with no axiom. | `hostile_test.shard` case 18; pins `k_client_transfer`, `k_client_admit`; `state_test.sh` |
| Unnumbered nodes compared without names; a caller's memo table answered a query; one id on two nodes. | `hostile_test.shard` cases 19b, 20, 21 |
| A native axiom named `Classical.choice` passed the policy as Lean's. | pin `init_name_taken_l`; beside it `def_takes_fn` |
| A registry row answered for any constant of its name; a decision had two runtime forms. | pins `registry_native`, `decidable_cells`, `derive_name_taken`, `derive_visible` |
| `(- 1 2)` at `Int` was read as 0, and K proved the statement so read. | pin `op_expected` |
| A module's meaning depended on load order. | pins `init_horizon`, `init_horizon_beyond` |
| An entry decoded by constructor shape let a value of no type in. | pin `entry_shape`; `entry_test.shard` |
| `decide` erased as the identity; a descent obligation nearly stated without its branch fact. | pins `project_bool`, `discharge_measure`, `if_rec_call` |
| A primitive let a negative through a `Nat` operation; a host call died instead of sticking. | `prims_test.shard` |
| The two readers agreed on a text that hid two same-named functions. | `parity_test.sh` checks one declaration per short name |
| The release gate's exit was not the conjunction of its checks; an admit flag was trusted. | `store_test.sh`; pin `k_client_admit` |
| A rewrite's motive was typed and not checked; K refused the certificate. | pins `reduce_dependent`, `rw_dependent` |
| Unique use of the World does not follow from a monotone clock. | pins `thread_sequential`, `thread_pair_twice`, `thread_helper_twice`, `bin_unthreaded` |
| A native binary took the else-branch of every `if` whose condition was not a `Bool` (a program's own two-constructor type, `Option`, `List`): the generated C tested for the `true` tag alone, and every test branched on `Bool`. Wrong output, not a false theorem. | `v3/pins/emit/if_second` (run by `native_tie_test.sh`) |
