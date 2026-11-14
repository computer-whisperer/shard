# The kernel directory

`v3/kernel/` is the system's own code, written in shard. This file is the map: what each source
file is for, how the checker K is built and guarded, what the trust base is file by file, how
programs run and are compiled, and how the whole thing is built and gated. The language itself is
in `LANGUAGE.md`.

How a source file flows through these files:

```
 FILE.shard
   │  sexpr, reader, scope, spelling, elab, unify      the front end: text to declarations (untrusted)
   ▼
 declarations ───────────► k/ (the checker K) ───────► ACCEPT, with the assumptions listed
   │   predef, define, derive, realize, template        a `fn` becomes a definition with equations
   │   tactic, arith, engine, prove                     proof steps become proof terms
   ▼
 erase, erasure, prog, thread                           the program of a `fn`; the World use check
   ├──► ev, session                                     the evaluator: what "run" means
   └──► lower, cgen, emit, rt.h  ──► cc                 the compiler to native binaries (unproven)
```

`loader.shard` drives all of it for one load; `load.shard` is its command line.

## The files

Paths are under `v3/kernel/` unless they start elsewhere. "Rests on" is what a file imports;
"called by" is who uses it.

### Source text, modules, the loader

| File | What it does | Rests on | Called by |
|---|---|---|---|
| `sexpr.shard` | The s-expression reader: bytes → `SExpr` (numeral, symbol, symbol with universe list, string, list); `read_all` for a whole file with the error's byte offset; `show_sexpr` printer; `size_sexpr`. No name has a meaning here. | `util.shard` (and the prelude) | `scope.shard`, `record.shard` (imports); `loader.shard` and `reader.shard` (`read_all`), `derive.shard`, `engine.shard`, `prove.shard`; `v3/tools/shardfmt/reads` (the formatter's judge) |
| `name.shard` | Hierarchical names (`Anon`/`Str`/`Num`, Lean's `Name`) and the structural hash (`mix`, `hash_name`, `name_key`) every table is keyed by. | `prelude.shard` | `util.shard`, `level.shard`, `expr.shard`, `verdict.shard`, `index.shard`, `initload.shard`, … — the base of everything |
| `util.shard` | List, byte, decimal and name utilities: `len append rev nth take drop`, `is_ws is_digit`, `show_nat show_int`, `name_eq name_lt show_name`, `hex_val`, `utf8_ok`. | `name.shard` | nearly every kernel file (`sexpr`, `json`, `record`, `index`, `initload`, …) |
| `intmap.shard` | Persistent map keyed by a non-negative Int (4-ary trie): `im_find im_insert im_has`. | `prelude.shard` | `expr.shard`, `prog.shard`, `ev.shard`, `index.shard`, `initload.shard`; through them `scope`, `loader`, K's tables |
| `scope.shard` | What a citation means in a file: `Scope` (declaration prefix, module tag, opened prefixes, visible set with the Init horizon), the elaboration context `Cx`, `resolve` / `scope_candidates` / `resolve_ctor`, `visible`, `init_cited` (`Init.NAME`), `toolchain_scope`, the renamed-primitive table, `type_head`. | `sexpr.shard`, K's view (`k`: `CheckedEnv`, `env_find`), `expr`, `intmap`, `name`, `util` | `etable.shard`, `matcher.shard`, `elab.shard` (imports); used by `reader`, `classify`, `erase`, `predef`, `loader` |
| `spelling.shard` | Two fixed tables over Init's names: `sp_canon` (a class method under Init's instance read as its function, e.g. `HAdd.hAdd Nat …` → `Nat.add`), `sp_instance` (the instance term for an instance-implicit binder at Nat/Int/Bool/List), and `sp_kit` (every constant the tables can write, for the Init demand). | `kw.shard` (K's view, `expr`, `level`, `name`) | `unify.shard` (import); `elab`, `tactic`, `arith`, `engine`; `loader.shard` (`sp_kit` in `init_kit`) |
| `reader.shard` | The reader of the logic's forms: one S form → K `Declaration`s (`def abbrev opaque theorem axiom inductive type structure`), view parameters (`sig`, `requirement`), `fulfills` and discharge proofs, the proof forms (`exact`, `by`, `auto`, `sorry`) with the sidecar key (`Req`, `req_key`, `auto_fit`); routes `fn extern realize fulfills bin` and the directives to the loader; refuses reserved and Lean keywords with a pointer. | `elab.shard`, `engine.shard`, `ev.shard`, `etable.shard`, `inhabit.shard`; `tactic`, `scope`, `sexpr` | `loader.shard`, `predef.shard`; tests through `test/reader_kit.shard` |
| `loader.shard` | The loader: package root, paths → module paths, imports (file, directory, Init) with cycle/collision/seal checks, `use`/`trusts`, the Init demand per file, per-form dispatch (K admission, policy check, `fn`/`extern`/`realize`/`derive`/`bin`/`fulfills` pipelines), views and the implementation check in a fork, the sidecar, the record type `Rec` with `show_rec`, the counts and the exit code; `load_data` for declarations built as data. | `host`, `reader`, K's view, `realize`, `define`, `template`, `record`, `derive`, `initload`, `elab`, `tactic`, `arith`, `thread` | `load.shard`, `prove.shard`, `session.shard`, `test/calc_harness.shard`, `test/loader_kit.shard` |
| `load.shard` | The loader's driver (`main`): parses the command line, loads the files, prints the records and the counts; the modes `--dump`, `--emit`, `--run` (with `--fuel`, `--trace`, `--lowered`), `--store`, `--reconstruct`. | `loader`, `store`, `ev`, `dump`, `emit`, `lower`, `session` | nothing imports it; run as `eval direct v3/kernel/load.shard` or `v3/bin/load` by every test script and `v3/build.sh` |
| `initload.shard` | Init on demand: opens the export by its index (meta line against the pin `init_pin_*`, index header, export size), looks names up (`od_lookup`, `od_names_under`, `od_has_under`), walks the closure of a set of records by range reads and feeds it to K in export order (`od_load`). | `host`, `json`, `intmap`, `name`, `util`, `index`, K's view | `loader.shard`, `verify_release.shard` |
| `index.shard` | The Init index's format, builder (`ix_build`) and verifier (`ix_verify`): header row, record table (ordinal, kind, offset, length, first ids), name table (sorted, name → ordinal), declaration table (name id → ordinal); all read by range. | `host`, `json`, `intmap`, `name`, `util` | `initindex.shard`, `initload.shard` |
| `initindex.shard` | Driver for the index: `initindex.shard EXPORT INDEX` builds, `--verify EXPORT INDEX` reads back; one line of output, exit 0 or 1 (2 on usage). | `index.shard` | nothing imports it; run by `test/init_index.sh` and `test/index_test.sh` |
| `verdict.shard` | The public vocabulary of K's answers beside the sealed checker: `Resource`, `Reason`, `(Outcome E)`, `Verdict`, `Failure`, `(KRes A)`. Data only. | `expr.shard`, `name.shard` | K (`k/env.shard`, `k/mod.req.shard` and the files using `kernel.verdict`), `kw.shard`, `loader.shard` (`Outcome`, `Resource`), `initload.shard`, `verify_release.shard`, `test/decls_kit.shard` |
| `dump.shard` | A loaded program as canonical text, one sorted line per declaration (`type`/`fn`/`extern`/`sig`/`sigtype`), for comparison with the bootstrap's `eval dump`. A projection: names print as their last component. | `prog.shard`, `etable.shard` | `load.shard` (`--dump`); compared in `test/parity_test.sh` |
| `json.shard` | A JSON reader over UTF-8 bytes (`json_parse`, `jget`, `jstr`); numbers kept as digit strings, key order kept. | `util.shard` | `index.shard`, `initload.shard`, `store.shard`, `verify_release.shard`, `k/import.shard`, K's view (imported there as public vocabulary), `test/json_test.shard` |

### Declarations, definitions, programs

| file | lines | what it does | rests on | called by |
|---|---|---|---|---|
| `decl.shard` | 187 | The data of K's declarations: `Declaration` (raw input: axiom, definition, theorem, opaque, inductive block, quotient) and `ConstantInfo` (a checked environment entry), reducibility hints, safety. Shapes follow Lean's export format field for field. | `expr.shard` | K (`k/`), `reader`, `elab`, `define`, `realize`, `derive`, `loader` — everything that builds or inspects a declaration |
| `prog.shard` | 400 | The program representation: `EType`, `ELit`, `EPat`, `ETerm` (ten forms), `ERec` (recursion clause), `EDecl` (inductive, function, extern, sig fn, sig type), `Prog`, values `Val`, results `EvRes`; the primitive table (identity → operation code, arity); the prelude's identities. | `util`, `intmap`, `name`, `prelude` | `etable`, `classify`, `erase`, `erasure`, `thread`, `ev`, `lower`, `dump`, `loader` |
| `etable.shard` | 332 | The program table `ESt`: every program-side identity findable by any suffix of its spelling, registered per file before any body is read (forward references); a re-registered identity overrides (how a view's signature is linked to its implementation). `EInfo` kinds incl. `EITemplate`. | `scope`, `prog` | `erase`, `classify`, `erasure`, `realize`, `define`, `template`, `thread`, `loader` |
| `erase.shard` | 663 | Roles over the checked environment: which inductives may appear in programs (`ind_eligible`), an L type as a program type (`l_to_etype`), the role of each binder of a constant (`BType`, `BErased`, `BData`, `BStatic`), level solving, the registry's one-field structures (`rep_row`), package types. | `etable`, `kw` (the walk over K) | `classify`, `erasure`, `realize`, `define`, `template`, `loader` |
| `classify.shard` | 1647 | The program-first reader (`route=e_first`): `fn`, `extern`, `type`, `sig` forms read straight into `prog.shard` data with every head resolved and saturated; exhaustiveness, private-match and comparison-type checks; pre-registration of a file's heads; static types of program terms (`c_type`); instances taken by a program-first body. | `etable`, `erase` | `loader` (`load_e_first`, twins of `type`/`sig`), `realize`, `thread`, `template` |
| `elab.shard` | 2082 | The elaborator: source terms → K's `Expr`, bidirectional, with metavariables for implicit arguments and levels solved by first-order unification; operators by operand type; `if`/`dif`; `match` (generates and admits a matcher); numerals, strings, `list`, `let`, binders; the three coercions; instance table lookup. | `scope`, `unify`, `matcher`, `spelling`, `kw` | `reader` (every L form), `predef`, `realize`, `derive`, `tactic`, `loader` |
| `predef.shard` | 236 | Reads a `fn` form once into a *pre-definition*: declared type, binder locals, a local for the function's own name, the elaborated body, the recursion clause, callee-locals for program-only callees. `PdSkip` = signature has no L reading; `PdErr` = refusal. | `reader`, `elab` | `erasure` (`erase_predef`), `define`, `template`, `loader` (`load_fn_with`, `probe_fn`, `read_group`, `load_ldef`) |
| `erasure.shard` | 2150 | The erasure of an L term to a program term by `erase.shard`'s roles; the walk state `Tr` and its K procedures; `erase_predef` (a `fn`'s program, or the verdict "template"); template calls → instance demands, lambda lifting (`CALLER.lam_N`), shapes and instance names. | `predef`, `erase`, `prog`, `matcher` | `realize`, `define`, `template`, `loader` |
| `realize.shard` | 1071 | `(realize NAME …)`: the supplied form (signature against the erased type, body elaborated at the constant's telescope, descent and callee-order checks) and the derived view; the equation generator `pj_equations` (leaves of the leading matchers, `eq_N` / `realize_N`, `fix_eq` proofs) shared with `define`. | `erasure`, `classify` | `define`, `template`, `loader` (`load_realize`, `load_data_view`) |
| `define.shard` | 2651 | The K projection of a pre-definition: non-recursive value; structural recursion by `T.brecOn` (`pj_struct`, generating `T.below`/`T.brecOn` for a program's type); measure by `WellFounded.fix` with obligations `f.dec_N` (`pj_wf`, `pj_self`); a group as one fixpoint over a generated `f.args` type (`define_group`); `casesOn`, `noConfusion`, `inj` bundle for a new inductive. Verdicts `DfOk`/`DfSkip` (RUNNABLE)/`DfErr` (REFUSE)/`DfAux`. | `realize`, `erasure`, `predef`, `matcher`, `kw` | `loader` (`load_fn_with`, `load_ldef`, `load_template`, `load_group`, `bundle_after`) |
| `template.shard` | 753 | Templates (a `fn` or supplied `realize` with a function-typed binder): checks the declaration at placeholder shapes (`check_template`), builds an instance — program, L definition (the template applied), equations — for a demand (`build_instance`), or the program alone for a program-first taker (`build_instance_e`). | `erasure`, `realize`, `classify` | `loader` (`load_template`, `build_demands`, `build_e_demands`, `register_template`) |
| `derive.shard` | 1238 | `(derive TYPE CAPABILITY…)`: reads the type's constructors from K's environment and writes source forms — `ctorIdx`, `decEq`, `compare`, `renderOnto`/`render` as `fn`s; `size` as `def`s over the recursor with `eq_K` theorems — which the loader then loads as the file's own; checks a `(CAP by NAME)` procedure's type. | `elab`, `matcher`, `unify`, `scope` (`derived_for`) | `loader` (`load_derive`) |
| `record.shard` | 436 | `(record …)` expanded at the s-expression level before anything is read: the `type`, accessor and updater `fn`s; `(make …)` and `(with …)` rewritten in every form of the file, also over a parameterless `structure`. | `sexpr`, `util` | `loader` (`expand_records`, per file) |
| `inhabit.shard` | 104 | Builds a default value of a K type structurally (a lambda for a function type; the first constructor whose fields all close) — the value an `extern`'s opaque constant is declared with. | `kw` | `reader` (`read_extern`) |
| `thread.shard` | 1154 | The use check over programs: World types and carriers, abstract tokens and boxes, the walk (`th_term`), a function's verdict (`th_fn`: `ok`, `world_twice`, `world_unsupported`, `world_forged`); second half: the same walk over the checker's own state, on request (`--state`). | `classify` (static types), `etable`, `prog` | `loader` (`rec_threads` at each file's end, `bin_threads`, `rec_states`) |
| `loader.shard` (declaration parts) | 4641 | `load_decl_go` dispatches each form; `load_e_form`/`load_fn_with` (the `fn` pipeline: pre-definition → erasure → definition; parking on a forward reference), `settle_groups`/`find_group`/`load_group` (groups), `load_ldef` (a recursive `def`), `load_extern`, `load_realize`, `load_derive`, `load_discharge` (obligations), `load_template`/`build_demands` (instances), `admit_core`/`policy_pass` (K admission and the axiom policy), `check_bin`, `rec_threads`, `show_rec` (the printed records). | `reader`, `k`, `realize`, `define`, `template`, `record`, `derive`, `elab`, `tactic`, `thread`, `initload` | `load.shard` (the driver), `session.shard`, `prove.shard`, tests |

Related files outside the list that the area depends on: `reader.shard` (the L declaration forms
`def`, `abbrev`, `opaque`, `theorem`, `axiom`, `inductive`, `type`, `structure`, `sig`,
`requirement`, `extern`'s L constant; the refused Lean keywords), `matcher.shard` (generated
matchers and their read-back), `spelling.shard` (the instance table; class methods read as their
functions), `scope.shard` (name resolution, `renamed_prim`, the derivation table's lookup),
`ev.shard` (running a program; entry argument codec; effect trace).

### Proofs

| File | What it does | Rests on | Called by |
|---|---|---|---|
| `tactic.shard` (2,959) | The step elaborator. Reads a `(by STEP…)` block into `Node` data (`read_block`, `read_node`), runs each step on a goal (`tac_step`, one `step_*` function per form), threads the block depth-first (`tac_block`) and returns the closed proof term, the sorried goals, or a refusal (`tac_prove`). Also renders goals (`goal_of`, `goal_sx`), lists the steps a goal admits (`applicable`), and declares `reconstruction_version` (2). | `elab.shard` (terms, metavariables), `arith.shard`, `unify.shard` (`assign`, levels), `matcher.shard` (`mt_is_matcher`, for case steps in `reduce`), `spelling.shard` (`sp_canon`), `kw.shard`, K through its view. | `reader.shard` (`read_proof`, `read_block_proof`, `run_tactic`), `engine.shard`, `loader.shard` (`reconstruction_version`, `tc_kit`), `load.shard`, `prove.shard`. |
| `arith.shard` (1,103) | `arith`'s back end. Reads a statement as a comparison (`ar_rel`), linearizes terms over atoms (`ar_lin`), builds the rows, finds weights by Fourier–Motzkin elimination (`fm_find`) or checks the given ones, and builds the proof from `Lean.Omega`'s lemmas (`ar_prove`). | `elab.shard` (locals, visibility of the kit), `spelling.shard`, `kw.shard`. | `tactic.shard` (`step_arith`, `applicable`), `engine.shard` (`eng_arith`), `loader.shard` (`ar_all_kit`). |
| `engine.shard` (502) | The default producer for `auto`: a fixed ladder of steps, each confirmed through `tac_step`, under a step budget and hints; renders `Node` data back to source text for the sidecar (`nodes_sx`). Declares `engine_version` (1). | `tactic.shard`, `arith.shard`, `elab.shard`. | `reader.shard` (`read_auto`, `read_measured`), `prove.shard` (the version, in the header and the sidecar comment). |
| `prove.shard` (317) | The `prove` program: loads the files under mode `solve` or `measure`, prints the records and counts, rewrites the sidecars of the files that gained a block or were read at an older schema or reconstruction version. | `loader.shard` (the whole load), `reader.shard` (`Auto`, `Req`), `engine.shard`, `tactic.shard`. | A root program: `eval direct v3/kernel/prove.shard`, `v3/bin/prove`; tests `engine_test.sh`, `tactic_lib_test.sh` (`parity_test.sh` lists it among the drivers). |
| `store.shard` (911) | The store writer: renders an admitted constant as lean4export NDJSON records with anonymous binders (`store_rendered`), writes `ID.ndjson`, `ID.names`, `order` and `manifest` (`store_write`), refuses a store at another reconstruction version. | K's view (`env_find`, `identity_hash`), `host.shard` (files), `json.shard`. | `load.shard` (`--store`, `--reconstruct` through `store_records`, `store_path`, `manifest_lines`). |
| `verify_release.shard` (861) | The verifier program: reads a store's manifest, loads the part of Init the store cites, replays each stored file through K, then checks names, kinds, cardinality, parameters, discharges, axiom closures and `bin` lines against the manifest. No elaborator in its closure. | K (`process_record`, `axioms_of`, `identity_hash`), `initload.shard` (the Init index), `json.shard`, `host.shard`. | A root program: `v3/bin/verify_release`; tests `store_test.sh`, `std_test.sh`, `tactic_lib_test.sh`, `path_test.sh`, `bin_test.sh`, `imp_test.sh`. |
| `unify.shard` (729) | Metavariables and first-order unification: the metavariable context (`MCtx`), assignment with occurs and scope checks (`assign`), `unify` with four outcomes (yes, no, stuck, exhausted), level unification, instantiation (`inst_mvars`). A closed mismatch is decided by K's `is_def_eq`. | `kw.shard`, `spelling.shard`, K's view. | `elab.shard` (every term), `tactic.shard` (`tac_match`, `rfl`, `apply`), `engine.shard`, `derive.shard`. |
| `matcher.shard` (570) | Generates the auxiliary definition a `match` elaborates to (`mt_generate`) and recognizes one by regenerating it from its type (`mt_is_matcher`). | `kw.shard`, `scope.shard`, K's view. | `elab.shard` (`elab_match`), `tactic.shard` (the case step of `reduce`), `define.shard`, `realize.shard`, `erasure.shard`, `derive.shard`, `template.shard`. |
| `kw.shard` (186) | The walk over K's terms from outside K: an environment, a local context and checker state (`W`), with K's `whnf`, `infer`, `is_def_eq`, `is_prop`, local creation and binder closing wrapped so that a failure is a value. | K's view only. | Every elaborating file: `unify`, `elab`, `tactic`, `arith`, `spelling`, `matcher`, `reader`, `loader`, `define`, `realize`, `derive`, `erase`, `erasure`, `template`, `classify`, `inhabit`, `session`, `predef`. |
| `elab.shard` (2,082), proof-related parts | The term elaborator the steps call for every TERM, PROP, LEMMA and FACT (`elab`, `elab_type`); the elaborator state `El`; `el_finish` and `unsolved_msg` (the refusals for undetermined arguments); `decision_of` and `no_decision` (what `decide` and `if` can decide); `el_sees` (is an Init constant within the module's horizon); the `pending_obligation` refusal on citing a pending name. | `unify.shard`, `matcher.shard`, `scope.shard`, `spelling.shard`, `kw.shard`. | `reader.shard`, `tactic.shard`, `arith.shard`, `engine.shard`, `derive.shard`, `loader.shard`. |
| `loader.shard` (4,641), proof-related parts | Reads the sidecar once per file (`read_sidecar`, `sidecar_split`, `sidecar_entries`; `sidecar_schema` is 3); computes the file's parts of the key (`req_ctx`); turns the reader's answers into records (`RPending`, `RProved`, `REngine`, `RTactic`, `RSidecar`, `RRecon`; `rd_plain`); keeps the pending names (`Load.pending`); handles `(fulfills f.dec_N PROOF)` (`load_discharge`, `settle_discharge`, `fulfills_cycle`) and a view's `fulfills` (`fulfil`); checks a `bin`'s `requires` against pending theorems (`bin_requires`); prints every record (`show_rec`) and the counts. | `reader.shard`, `tactic.shard`, `elab.shard`, K. | `load.shard`, `prove.shard`, `session.shard`, the loader tests. |

#### Other files on the path of a proof

| File | Why it matters here |
|---|---|
| `reader.shard` (1,010) | The entry for every proof: `read_proof` and `read_proof_go` choose among `(exact …)`, `(by …)`, `sorry`, `auto` (the `bad_proof` refusal is here); `read_auto` computes the key (`req_key`, `req_deps`), fits the sidecar entry (`auto_fit`) and, under mode `solve`, calls the engine or a named tactic (`run_tactic`); `read_fulfills` and `read_discharge` do the same for `fulfills`. |
| `spelling.shard` | `sp_canon`: Lean's class-method spellings read as this language's function spellings in goals, lemmas and `arith` rows; `sp_instance`: the fixed instance table behind `instance_needed`. |
| `load.shard` (613) | The `load` program: `--store` (`store_write`), `--reconstruct` (`main_reconstruct`, `reconstruct_all`), the exit code (refusals plus errors). |
| `k/add.shard` | `identity_hash`, the content fingerprint. |

### K, the Init import, the replay gate

#### `v3/kernel/k/` — the sealed directory (one module, `kernel.k`)

- `mod.req.shard` (109) — the view: 8 `sig type`s (CheckedEnv AxMemo Tabs Run Ctx LDecl Loc Limits)
  and 63 `sig fn`s; the only way in. Rests on the public vocabulary (prelude host name level expr
  decl intmap json verdict). Imported (`(import "k")`) by kw, loader, scope, store, initload,
  verify_release, t0; tests hostile_test, k_client_test; pins k_client_{reach,admit,transfer}.
- `k.shard` (13) — the implementation's import list; declares nothing. The view's signatures are
  matched against what these imports declare (`load.shard --root v3 v3/kernel/k`).
- `env.shard` (43) — `CheckedEnv` (constants by 24-bit name key with buckets, quotInitialized,
  pinned names, node-id watermark) and `env_empty`; states the one-environment invariant.
  Rests on decl, intmap, verdict. Used by every other K file.
- `tc.shard` (1,956) — the type checker: `infer` (checking and inference-only modes), `whnf_core`,
  `whnf`, `is_def_eq` in Lean's order (quick check, proof irrelevance, lazy delta, eta, structure
  eta, string literal expansion, unit-like); memo tables; `guard` (depth and heartbeat budgets);
  literal typing gated on pins; `reduce_nat` / `nat_apply` (the 14 accelerators); `reduce_native`
  (Unsupported). Also `Ctx`, `LDecl`, `Loc`, `env_find`, `env_add`, `st_for`, `st_seed`.
  Rests on env, expr, level, name, intmap, util, verdict. Called by inductive, nested, add,
  axioms; through the view by kw (hence elaborator, unifier, tactics, definer, erasure).
- `inductive.shard` (1,214) — `add_inductive_core`: check the types, declare them, check and
  declare constructors (positivity, universes), elimination level, K-target, recursor types and
  rules, then `check_recursors` (each rule type-preserving). Budgets `budget_depth` 8192,
  `budget_ticks` 100,000,000. Rests on tc. Called by nested.
- `nested.shard` (1,012) — nested inductives: uniform-occurrence check, translation to auxiliary
  mutual types (`_nested` prefix reserved), admission through inductive.shard, translation back;
  `add_inductive` is the entry. Rests on inductive. Called by add.
- `add.shard` (515) — admission: `check_decl_core` per kind, `add_quot` (checks `Eq`'s shape,
  builds the four constants), `identity_hash` (a report fingerprint), the pin match
  (`pin_if_matches`, `ref_closure_ok`, `ref_matches`), the raw entry `check` / `check_with`
  (ingest, then check, then pin), `Limits`. Rests on nested, accel_pins. Called by import
  (`check_decl_pinning`, no ingestion) and, through the view's `check`, by loader, kw (`w_admit`),
  tests.
- `axioms.shard` (241) — `axioms_of`: a constant's axiom closure as a least fixpoint over
  "cites in type or value", inductive blocks resolved as a unit, memo (`AxMemo`) kept outside the
  environment; `ci_used`, `induct_ctors`, `names_union`. Rests on tc. Called by import (`-a`),
  loader (`policy_pass`, `check_bin`), verify_release, reader (`ci_used` for requirement keys).
- `import.shard` (861) — the lean4export NDJSON import: table records (names, levels, terms,
  each compound term minted from the run's state), declaration records → `Declaration` →
  `check_decl_pinning`; an inductive record's exported types, constructors and recursors compared
  field by field with what K generated (`MISMATCH`); `Run` (tables, environment, state, five
  counts, axiom mode); `empty_run`, `run_on`, `run_resume`, `process_record`, `process_lines`,
  `show_verdict`, `show_counts`, `record_constants`. Rests on add, axioms, json, host.
  Called by t0, initload (`od_feed`), verify_release, hostile_test.
- `accel_pins.shard` (5,128, generated below its marker) — `pin_candidates` (20 names),
  `is_pin_candidate`, then 206 reference declarations `rd_H` over shared term nodes `re_K` and
  names `rn_H`, the lookup `accel_ref` and `accel_ref_closure`. Produced by `gen_pins.sh`;
  checked by `t0_full.sh` (all 20 pinned on the export) and `k/test/accel_test.shard`.
  Rests on decl. Called by add (the match), t0 (`pin_candidates`), loader (`k_kit`).
- `refgen.shard` (569) — `show_refs`: prints each candidate's identity closure as `REF ` lines
  (the generated section of accel_pins). Generation only. Rests on add, host. Called by t0 under
  `--pins`.
- `test/tc_test.shard` (101 cases), `test/inductive_test.shard` (29), `test/nested_test.shard`
  (11), `test/add_test.shard` (25, with the one-counter census), `test/accel_test.shard` (9) —
  tests inside the seal; run by `v3/test.sh` on the bootstrap and by `k_clients_test.sh` through
  the V3 loader.

#### The vocabulary K is written over (`v3/kernel/`, public)

- `expr.shard` (1,175) — `Expr` (11 node kinds, each with node data `D`: id, hash, loose-bvar
  range, three flags), `Literal`, `BinderInfo`, `Memo`, `St` (fvar counter, heartbeats, node
  counter, memo); `mk_*` (anonymous) and `new_*` (identified) constructors; `ingest_expr` (the
  raw entry's rebuild); `expr_eq`, `same_node`, `used_consts`/`expr_consts`; instantiate, abstract,
  lift/lower, level instantiation, beta; `show_expr`. The header comments carry the typing,
  reduction and definitional-equality rules. Rests on level, intmap, name. Used by everything
  that builds or reads terms.
- `level.shard` (470) — `Level` (zero succ max imax param, and the elaborator's `LMVar`),
  `normalize`, `is_equivalent`, `is_geq`, `undef_param`. Rests on util. Used by expr, tc, elab.
- `decl.shard` (187) — `Declaration` (raw) and `ConstantInfo` (checked entries), hints, safety,
  recursor rules; the admission rules as comments. `verdict.shard` (70) — `Outcome`, `Reason`,
  `Resource`, `Verdict`, `Failure`, `KRes`. `name.shard` (66) — `Name`, `hash_name`, `name_key`,
  `mix`. Also `intmap` (79), `util` (226), `json` (222), `prelude` (17), `host` (25).
- `kw.shard` (186) — the walk over K from outside the seal: `W` = environment + local context +
  state; `w_infer`, `w_whnf`, `w_is_def_eq` (and `_r` forms that keep K's failure), `w_local`,
  `w_pis`/`w_lams` (K's `mk_binding`), `w_admit` (K's `check` into the walk's environment),
  `reason_of`. Rests on the view. Called by elab, unify, tactic, arith, define, derive, erase,
  erasure, realize, matcher, reader, loader.

#### The Init side

- `t0.shard` (77) — the replay driver: `[-v] [-a] [-p] [--pins] CHUNK…`; reads each chunk,
  `process_lines`, prints non-ACCEPT verdicts and counts; `-a` axiom closures, `-p` the pins
  line, `--pins` the reference table; exit = rejected + exhausted + mismatched. Rests on the
  view, host. Called by `t0_full.sh`, `gen_pins.sh`, t0_fixture_test, route2_test, native_tie.
- `index.shard` (927) — the Init index: header, record table (62-byte rows), sorted name table
  (218-byte rows), declaration table (9-byte rows); `ix_build` (one pass), range readers
  (`ix_read_row`, `ix_lookup`, `ix_names_under`, `ix_decl_of`), `ix_verify`. Rests on host, json,
  name, util. Called by initindex, initload.
- `initindex.shard` (61) — driver: build an index, or `--verify` one. Called by
  `kernel/test/init_index.sh` (keeps `v3/.cache/init.index` current) and index_test.
- `initload.shard` (889) — the load on demand: `od_open` (meta line against the pin, index
  header, export size), `od_lookup` (name → ordinal), `od_walk` (the closure of the demanded
  records by range reads), `od_feed` (lines through K's `process_record` on `run_on env`),
  `od_load`; `od_consts_of_stream` (the release gate's demand). Rests on index, the view, json,
  host. Called by loader (`init_through`, `init_demand`, `init_load`), verify_release.

#### Scripts and fixed files

- `v3/export.sh` (60) — builds `init.ndjson` (lean4export `15f6055`, toolchain v4.33.1) and
  `init.axioms` under `$V3_EXPORT_DIR` (default `.shard-cache/v3-export`, untracked); writes the
  pin marker `init.ndjson.pin`.
- `v3/axioms.lean` (80) — the oracle: Lean computes each `Init` constant's axiom closure as a
  least fixpoint; one `AX name: axioms` line each.
- `v3/t0_full.sh` (102) — the replay gate. `v3/t0_expected.txt` (1) — the pinned verdict line.
  `v3/t0_axioms_cmp.sh` (26) — K's `AX` lines against the oracle's.
- `v3/kernel/gen_pins.sh` (25) — regenerates accel_pins.shard below its marker.
- Tests: `state_test.sh` (the state discipline + `v3/pins/state/main.shard`, 28 controls),
  `t0_gate_test.sh` (the gate on stub engines, 5 scenarios), `t0_fixture_test.sh` (3,000 lines,
  closures vs oracle), `route2_test.sh` (K under `ev` vs K direct, byte-identical),
  `k_clients_test.sh` (K's tests through the V3 loader), `index_test.sh` (build, verify, four
  tampered inputs), `init_index.sh` (not a test: keeps the index current).

### Running, the back end, the build

#### Kernel sources

| file | lines | what it does | rests on | called by |
|---|---|---|---|---|
| `prog.shard` | 400 | Programs as data: `EType`, `ELit`, `EPat`, `ETerm`, `ERec`, `EDecl`, `Prog`, `Val`, `EvRes`; the primitive table (39 entries, code and arity by identity); the prelude's identities. | `name`, `prelude`, `util`, `intmap` | the classifier and loader (build a `Prog`), `ev`, `lower`, `cgen`, `emit`, `dump`, `session` |
| `ev.shard` | 1,578 | `ev`, the definition of "run": links a `Prog` into a private form (constructors as tags, functions as indices) and runs it as a machine with an explicit continuation, fuel spent per function entry. Also `prepare`/`invoke`/`invoke_run` (the handle and its two doors), the argument validation, the text codec for command-line arguments, `run_prog`, the extern wire by declared type (six host calls), the effect trace. | `host`, `prog`, `intmap`, `util` | `session.shard` (the door on a load), `load.shard` (through the session), `reader.shard`/`loader.shard` (a tactic library's run), tests `ev_test`, `prims_test`, `entry_test`, `handle_test`, `rows_test`, `project_test`, `loader_test`, `calc_harness` |
| `session.shard` | 589 | The long-lived value a host holds: revisions by ordinal (`ses_introduce`, `ses_extend`), handles by ordinal (`ses_prepare`, `ses_release`, `ses_revoke`, `ses_withdraw`), invocation (`ses_invoke[_at]`, `ses_run[_at]`), and `prepare_on`: `ev`'s `prepare` plus what the load's environment shows of an entry that the program does not state (evidence binders, proposition fields). `run_load`/`run_load_as` are the command line's `--run`. | `loader`, `ev`, K's view (`kernel.k`), `kw`, `reader`, `index`, `initload` | `load.shard` (`--run`, `--lowered`), `test/session_test.shard` |
| `lower.shard` | 687 | Back end stage 1, `lw_prog : Prog → Prog`: to a named form, the seven expression rows for naming-law primitives, lifting of `if`/`match` in operand position, nested patterns to flat chains, every operand an atom, back to indices. `lw_role` gives the eight wire roles. Refuses unlinked `sig`s, primitives with no row, string patterns. | `prog`, `name`, `util` | `emit.shard`, `cgen.shard` (`lw_role`), `load.shard` (`--lowered`) |
| `cgen.shard` | 791 | Back end stage 2, `cg_prog`: the restricted form transcribed to C over `rt.h` — name mangling, tags (900–908 by role, others from 1000, odd for a type's second constructor), symbol statics, per-function text, `main.c` plus parts of 400 functions, `main`. Refuses anything outside the restricted form, a numeral pattern past the small integers, an extern with no shim, a missing `main` or one with parameters before the World, a constructor of more than 125 fields, a constructor named as a cell of the runtime at another place in its type. | `lower`, `prog`, `intmap`, `name`, `util` | `emit.shard` |
| `emit.shard` | 108 | The back end's driver `emit_prog`: lowering, generation, the `map` lines (C name, kind, identity, source file), the module count. | `prog`, `etable`, `lower`, `cgen` | `load.shard` (`--emit`), `test/session_test.shard` (emit mode) |
| `rt.h` | 1,014 (C) | The runtime every generated `main.c` includes: tagged small integers and bignum cells, cells and the allocator, the conservative mark–sweep collector, the intern table, the primitives, nine extern shims, traps, `SHARD_STATS` counters, `rt_init`. | libc, GNU C extensions, Linux `mmap`/`setrlimit` | generated C (`cc … -I v3/kernel`: `v3/build.sh`, `test/session_emit_test.sh`) |
| `host.shard` | 25 | The toolchain's World type and seven extern declarations over the prelude's types (`get_args read_file read_range write write_line write_file exit`). | `prelude` | every toolchain entrypoint; `ev.shard` (its driver performs six of them through these) |
| `load.shard` | 613 | The loader's command line. In this area: `--run MODULE.FN … -- ARG…` with `--fuel`, `--trace`, `--lowered` (`main_run`, the exit codes 2–6), `--emit DIR` (`main_emit`: writes the C parts, `map`, `sources`; the `EMIT:` line), `--dump` (`main_dump`). | `loader`, `store`, `ev`, `dump`, `emit`, `lower`, `session` | `eval direct v3/kernel/load.shard …`, `v3/bin/load …`; `v3/build.sh` (the emit); most shell tests |
| `dump.shard` | 240 | A `Prog` as the canonical text that the bootstrap's `eval dump` also prints: one line per declaration, sorted; a stated projection (last name components). | `prog`, `etable` | `load.shard --dump`; `test/parity_test.sh`, `test/template_test.sh` |
| `intmap.shard` | 79 | A persistent map keyed by non-negative integers (a 4-ary trie). | `prelude` | `ev` (function and tag tables), `cgen` (tags), `session` (its tables), K and the loader |
| `prelude.shard` | 17 | The toolchain's own `List`, `Option`, `Bool`, `Pair`: the cells the bootstrap, `ev`'s wire and the runtime's roles are built on. | — | every toolchain source |

The other four roots of the build's inventory are drivers of other areas: `t0.shard` (K over the
export's chunks), `prove.shard` (the producers), `verify_release.shard` (K's replay of a store),
`initindex.shard` (the Init index).

#### The Rust bootstrap (`rust_bootstrap/src/`, 3,661 lines; no README)

| file | lines | what it does |
|---|---|---|
| `bin/eval.rs` | 395 | The `eval` executable: `direct FILE ARGS…` and `dump FILE`; resolves the import closure (files; a directory as its view then its implementation); runs on a thread with a 4 GiB stack (`SHARD_STACK_MIB`); the host handler for ten extern names. |
| `load.rs` | 1,108 | The reader: `type`, `fn`, `extern`, `record` expansion, `(T Type)` binders, sequential `let`, flat name resolution with dotted citations canonicalized to the declared name they end in; skips the logic's forms. |
| `eval.rs` | 1,165 | The evaluator: an environment machine over one value stack, reference-counted values; `SHARD_PROF` call counts. |
| `prim.rs` | 165 | The eighteen primitives over arbitrary-precision integers (plus `gen_fresh` and the old spellings `int_eq lt le`). |
| `dump.rs` | 164 | `eval dump`: the canonical text of frontend parity. |
| `ast.rs` | 93 | The AST. |
| `lib.rs` | 571 | The crate root, the allocator (mimalloc), 24 unit tests. |

#### Scripts

| script | what it does | called by |
|---|---|---|
| `v3/build.sh` | emit on the bootstrap, then `cc`; the inventory, the stamp, the shim `v3/bin/eval` | CI, `v3/fmt.sh`, the shim (`--stamp`), `native_tie_test.sh`, `bin_test.sh`, `build_test.sh`, `shardfmt_test.sh` |
| `v3/test.sh` | every entrypoint on one engine, in parallel; keeps outputs for the suite tie | CI (`v3`, `native`), authors |
| `v3/fmt.sh` | lists or rewrites sources that are not canonical; builds `v3/bin/shardfmt` when stale | authors; `shardfmt_test.sh` (`--list`) |
| `v3/t0_full.sh` | the engines' tie on 100,000 lines, then the full Init replay against its pins | CI (`v3`), `test/t0_gate_test.sh` (with stub engines) |
| `v3/t0_axioms_cmp.sh` | K's axiom closures against Lean's oracle file | `v3/t0_full.sh`, `test/t0_fixture_test.sh` |
| `v3/export.sh` | installs Lean (elan), builds lean4export, exports `Init` and the axiom oracle into `.shard-cache/v3-export` | CI, authors (once) |
| `v3/kernel/test/init_index.sh` | keeps `v3/.cache/init.index` current and prints `EXPORT INDEX`; `--current` never builds | `v3/test.sh`, `v3/build.sh`, most shell tests |
| `v3/kernel/test/suite_tie.sh` | compares two kept suite runs | CI (`suite_tie`), `native_tie_test.sh` (its self-test) |
| `bin/ci_docs_only.sh` | exit 0 when a push changed only `.md` files; `--self-test` | CI (every gate job's first line; `host` runs the self-test) |
| `.gitlab-ci.yml` | the four jobs `host`, `v3`, `native`, `suite_tie` | GitLab |

## K


K is the checker of L. It takes a checked environment and one declaration and answers. It has no
elaboration, unification, tactics or global state, and reads no file. The data it is written over
is public and outside its directory: `name`, `level`, `expr` (terms, node data, the state), `decl`,
`verdict`.

| file (`v3/kernel/k/`) | lines | holds |
|---|---|---|
| `env.shard` | 43 | `CheckedEnv`: constants by name hash, whether `Quot` is declared, the pinned names, the node-id watermark |
| `tc.shard` | 1,956 | type inference, weak-head normal form, definitional equality; memo tables; budgets; literal rules; the `Nat` accelerators |
| `inductive.shard` | 1,214 | inductive blocks: the checks, the constructors, the recursors and their rules |
| `nested.shard` | 1,012 | nested inductives through auxiliary mutual types |
| `add.shard` | 515 | `check`: admission per kind, the `Quot` constants, the pin match, the raw entry |
| `axioms.shard` | 241 | axiom closures |
| `import.shard` | 861 | export records → tables → declarations → checks; generated constants against exported ones |
| `accel_pins.shard`, `refgen.shard` | 5,128, 569 | the generated reference rows for the pins; their generator, on no checking path |
| `mod.req.shard`, `k.shard` | 109, 13 | the view; the implementation's import list |

### What `check` does

`check env st decl` answers `Accepted env'`, `Rejected reason subject` or `Exhausted resource
site`. First it rebuilds every term of `decl` from its structure. Then, by kind:

| kind | checked |
|---|---|
| axiom | the name is free; level parameters are distinct; no free or meta variable; the type checks and is a sort |
| definition | the same, and the value checks with a type definitionally equal to the declared one |
| theorem | as a definition, and the type is a proposition |
| opaque | as a definition; the value is never unfolded afterwards |
| inductive block | parameters, universes, strict positivity; K generates the constructors and recursors and checks that each recursor rule preserves types |
| `Quot` | `Eq` has the expected shape; K builds the four constants itself |

| reason | meaning |
|---|---|
| `Malformed` | not a well-formed input: `loose_bvar`, `fvar_in_type`, `mvar_in_value`, … Definitive. |
| `RuleViolation` | a rule refuses it: `already_declared`, `unknown_constant`, `app_type_mismatch`, … Definitive. |
| `Unsupported` | outside the profile: `unsafe_declaration`, `mutual_unsafe_block`, `native_reduction`, an unpinned literal |
| `ConversionNotEstablished` | the procedure did not find two terms equal; not a proof that they differ. Printed `conversion`. |

`Exhausted` is neither acceptance nor rejection, and is never cached. The budgets are a recursion
depth of 8,192 and 100,000,000 steps for each declaration; a numeral is capped at 128 MiB, a `pow`
or `shiftLeft` count at 32 bits. A result reached under a budget is the same under a larger one;
the loader has no option to raise them. Inference, reduction and definitional equality follow
Lean's kernel in its order; memo tables are fresh for each declaration and change cost only.

### The boundary

`mod.req.shard` is the only way in: eight sealed types and 63 signatures.

    (sig type CheckedEnv)
    (sig fn check ((env CheckedEnv) (st St) (d Declaration)) (Outcome CheckedEnv))
                                                    (v3/kernel/k/mod.req.shard)

The signatures are admission (`env_empty`, `check`, `check_with`, `limits`), readers of an
environment (`env_find`, `ci_*`, `identity_hash`, `env_pinned`), axiom closures (`axioms_of`),
the record import (`empty_run`, `run_on`, `process_record`, `process_lines`, …) and the checker
as a service (`ctx_new`, `st_for`, `infer`, `whnf`, `is_def_eq`, `is_prop`, `mk_local`,
`mk_binding`). A client cannot build a `CheckedEnv`: `env_empty` is the empty one, and only
`check` and the record import extend it. A client that names the constructor gets
`REFUSE NAME function_expected` (v3/pins/loader/k_client_reach). No signature takes two
environments. The service signatures answer about terms a client built; an answer extends nothing.

The view's importers are `kw.shard` (the walk the elaborator, the tactics, the definer and the
erasure build terms with), `loader.shard`, `scope.shard`, `store.shard`, `initload.shard`,
`verify_release.shard` and `t0.shard`. `load.shard --root v3 v3/kernel/k` checks the implementation
against the view; `k_clients_test.sh` runs K's tests through the V3 loader, where the seal holds.

### The state discipline

Every term node carries a structural hash and cached flags, and a compound node an id. Ids come
from a counter in the checker state `St`, which every function takes and returns. An environment
records the highest id in it, the *watermark*. Equality takes two nodes with one positive id and
one hash as equal. A change to K must keep these rules ("hostile N" is a case of the battery):

| rule | held by |
|---|---|
| 1. Inside a check, no version of the state is advanced twice. | `state_test.sh` |
| 2. A state is made from nothing only in `empty_run` and `run_on`. | `state_test.sh` |
| 3. A run goes back to an earlier state only through `run_resume`: three uses in `check_into`, one in `process_record`. | `state_test.sh` |
| 4. Twelve named functions open or build a state. A thirteenth is an edit of the test. | `state_test.sh` |
| 5. A check mints above both the environment's watermark and the caller's counter, and writes the new watermark. | `k/test/add_test.shard` (census) |
| 6. Nothing takes two environments or puts one run's tables on another's environment. | hostile 18; pins `k_client_transfer`, `k_client_admit` |
| 7. The raw entry rebuilds every node, mints only what its table lacks, and starts from empty memo tables. | hostile 16, 19, 20 |
| 8. Every reader of an id reads the hash with it. | hostile 21 |
| 9. A positive id is written only by the five `new_*` constructors. | no test |

`state_test.sh` loads K with `load.shard --state`, requires every function that handles the state
(at least 300) to be judged `ok` or `primitive`, and runs the 28 controls of
`v3/pins/state/main.shard`. Its limits: a declared type is believed (control `lim_lie_out`), memo
tables are plain values to it, the primitives are read as given. The layers above K do reuse
states; what they build is admitted only as rule 7 rebuilds it.

### How Init gets in

`v3/export.sh` writes Lean's `Init` as text, one JSON record a line (`init.ndjson`: name, level
and term table lines, then declaration records in dependency order), and Lean's own axiom closures
(`init.axioms`, from `v3/axioms.lean`). K reads the export in two ways. *Whole*, by `t0.shard`,
every line in order: the replay gate. *On demand*, in a load: `initindex.shard` builds an index
over the export once (`index.shard`: a record table, a sorted name table, a table from name ids
to records), and a module writes

    (import Init Int.decEq)                         (v3/pins/loader/init_horizon/small.shard)

Its *horizon* is the record after `Int.decEq`'s: it sees the declarations before its horizon, or
before the farthest horizon among its imports. For each file the loader takes the names the file
cites and a fixed kit of names the toolchain itself builds, keeps those below the horizon, walks
their dependencies through the index (`initload.shard`) and feeds those lines to K in export
order, into the load's own environment. Every record is checked; none is admitted on trust. The
load prints `INIT NAME: horizon H, K declarations admitted`, NAME the import that set the horizon
(`-` when it is inherited through a file import). Only `Init` is exported; no other
Lean library can be imported.

| refusal | when |
|---|---|
| `export_pin_mismatch` | the export's first line names another Lean commit, exporter or format |
| `init_index_stale` | no index, or one built over another byte count |
| `init_name_not_found` | `NAME` is not a declaration of the export |
| `above_horizon` | a cited name lies at or past the horizon — or an instance the elaborator supplies does, or a constant K refused a declaration for; the message names the import that set the horizon and the import to write |
| `init_refused` | K did not accept a record; its verdict follows |

### The replay gate

`v3/t0_full.sh` replays the whole export and passes when all of these hold:

1. On the first 100,000 lines the bootstrap and the compiled K each exit 0, each print a verdict
   line, and their outputs are byte-identical.
2. The compiled K's verdict line over the whole export equals the one line of `v3/t0_expected.txt`:
   `T0: accepted 57977  rejected 0  exhausted 0  mismatched 0  unsupported 0`.
3. The axiom closure K prints for every constant equals Lean's (`v3/t0_axioms_cmp.sh`).
4. All 20 pin candidates are pinned (`PINS pinned: … | unpinned:`).

`mismatched` counts the inductive blocks and `Quot` constants where what K generated differs in
any field from what Lean exported. The driver does not check the export's first line; the pin of
a replayed file is the marker `export.sh` writes beside it. `V3_T0_SKIP_FULL=1` runs condition 1
alone. `t0_gate_test.sh` runs the script on stub engines (two that fail alike, or a log without
its verdict line, fail it); `t0_fixture_test.sh` and `route2_test.sh` replay 3,000 lines.

### The accelerated primitives

K computes directly, instead of unfolding definitions: `Nat.succ` of a literal and the binary
operations `Nat.add sub mul pow gcd mod div beq ble land lor xor shiftLeft shiftRight` on two
literals; the type of a `Nat` or `String` literal; the expansion of a string literal. Each is
enabled for a name only when the declaration admitted under it, and every definition, opaque,
axiom and inductive type it cites transitively, equals a reference row structurally. There are 20
candidates (the fourteen operations, `Nat`, `String`, `Char`, `List`, `String.ofList`,
`Char.ofNat`) and 206 rows. A user's own `Nat.add`, or the genuine one over a respelled
dependency, is an ordinary definition and reaches no shortcut. A literal in an environment whose
`Nat` is not pinned is refused `nat_literal_unpinned_Nat`.

`v3/kernel/gen_pins.sh` regenerates the rows (`t0.shard --pins` replays the export on the
bootstrap, `refgen.shard` prints each closure). They are checked by condition 4 above, by
`k/test/accel_test.shard` (the rows replayed from the empty environment pin) and by hostile 7, 8.

## The trust base, file by file

`LAW.md` §4 states the trust base in short. This is the detail.

To *believe* a component is to rely on it without a check inside the system. Tests are named as
evidence; none of them is a proof.

### (a) For an accepted theorem to be true

The claim: when a load prints `ACCEPT T … axioms=A params=P`, the proposition K holds as the type
of `T` is derivable in Lean 4's type theory from the axioms `A` (the parameters `P` among them).

1. **The logic.** Lean 4's type theory as its kernel at v4.33.1 implements it, and the consistency
   of the axioms in `A` over it. K adds no rule.
2. **K implements the logic.** `v3/kernel/k/`: `env tc add inductive nested import axioms`
   (5,842 lines) and the generated table `accel_pins.shard`; the data they are written over,
   `v3/kernel/`: `name level expr decl verdict intmap util prelude`, and `json` for the record
   import. K is reviewed source, written function for function after Lean's kernel. It is not
   proved; its sources are programs whose declared types and termination clauses nothing checks.
   Evidence: the replay gate; the hostile battery (`v3/kernel/test/hostile_test.shard`,
   72 cases of declarations that must be refused); the tests in `k/test/`.
   Two beliefs inside this item are easy to miss:
   - *Node identity.* Two terms with the same positive node id and the same hash are taken as
     equal without being compared. This is right only while an id names one node within an
     environment: the state discipline ("The state discipline" above), held by `state_test.sh` to
     the limits stated there.
   - *Accelerated primitives.* K computes fourteen `Nat` operations on literals, types literals
     and expands string literals itself. One believes each computes what the pinned definition
     defines (`k/test/tc_test.shard`: results, large operands, a wrong result for each).
3. **The engine that runs K.** The authority is the Rust bootstrap, `rust_bootstrap/src/`
   (3,661 lines; the crates `lexpr`, `num-bigint`, `num-traits`, `mimalloc`; the Rust compiler):
   it parses K's sources, evaluates them, and supplies integer arithmetic and the host calls. Its
   parse is compared with the V3 reader's on every toolchain root (`parity_test.sh`). A checker
   run as a native binary (`v3/bin/load`, `v3/bin/t0`, …) adds the back end (`lower.shard`,
   `cgen.shard`, `emit.shard`), the runtime `rt.h`, `cc -O2`, and the bootstrap run that wrote the
   C text. The binaries are tied to the bootstrap by comparing outputs byte for byte
   (`t0_full.sh`, `native_tie_test.sh`, `suite_tie.sh`), not by a proof.
4. **Init is Lean's Init.** `v3/export.sh` builds the export from the toolchain
   `leanprover/lean4:v4.33.1` and `lean4export` at `15f6055`, both fetched from the network; it is
   not in the repository. A load checks four fields of its first line (the Lean commit, the
   exporter's name and version, the format version) and the byte count of the file its index was
   built over. Nothing fixes its bytes. K checks every declaration it reads, so another file
   cannot get an ill-typed term admitted; but axioms are admitted as written, and the policy takes
   the three standard names found in the file for the standard axioms. So one believes the file
   is the genuine export. Evidence: the replay gate on a fresh export — the count, the closures
   against Lean's own, and 206 declarations equal to rows committed in `accel_pins.shard`.
   In a load on demand, `index.shard`, `initindex.shard` and `initload.shard` choose which lines
   K reads. K still parses and checks each. One wrong choice is not refused: a name or level line
   left out reads as the anonymous name or level zero (`k/import.shard`). An index with one row
   altered, so that a record is read without the line that names it, is not refused either: the
   load prints the same records and exits 0, and the constant is no longer found under its name
   (`unknown_constant: Nat.mul._f`). So the index and the walk are inside the trust base for "a
   constant named as Init's is Lean's". No inconsistency has been built through them, and none is
   shown to be impossible.
5. **The account above K.** To K, view parameters and obligations are axioms. Which axioms are
   permitted, the replacement of a discharged obligation by its proof's closure, the refusal of a
   cyclic discharge and the check of an implementation against its view are the loader's
   (`loader.shard`: `policy_pass`, `permitted`, `discharge_axs`, `load_discharge`). A record with
   an empty `params=` is unconditional only if that code is right. `verify_release.shard` computes
   the same account a second time over a stored bundle, with K, the host and the JSON reader only.
6. **What the statement says.** K checks the term the elaborator built. Which proposition a
   source statement denotes is decided by `reader.shard` and `elab.shard`; a mistake there gives a
   true theorem about something other than what the author wrote.

### (b) For a compiled binary to behave as its contract says

The contract of a binary is its `bin` form: the requirements are theorems about the entry
function's definition and about observables of the World. Believe (a) for each requirement, and:

| component | files | what is believed | evidence |
|---|---|---|---|
| host-call axioms | the axioms the `bin` trusts | they are consistent together and true of the host | none |
| program = definition | `erase erasure predef realize template`; the registry that runs `Nat` and `Int` as integers | the program derived from a `fn` computes what its definition denotes | tests; no proof |
| the World used once | `thread.shard` | each World value is consumed at most once, so the axioms' order of effects is the run's | pins `thread_*`; `UNTHREADED` fails the load |
| the roster | `loader.shard` (`check_bin`) | the entry's closure calls no extern outside `externs` | pin `bin_extern_outside` |
| the run, interpreted | `ev.shard` on the bootstrap | it computes the program | the suite |
| the run, compiled | `lower.shard`, `cgen.shard`, `emit.shard`, `rt.h`, `cc -O2`, libc, the OS | the binary computes the program | `lower_test.sh`, `native_tie_test.sh`, `bin_test.sh` |
| host calls | `rust_bootstrap/src/bin/eval.rs`; `rt.h` (`rt_get_args` … `rt_exit`) | each does what its axioms state | none |
| the entry | `ev.shard`, `session.shard` | command-line text becomes values of the entry's types | `entry_test.shard` |

An extern is an opaque constant in L. Nothing in L says what it does except axioms a file states
and trusts, such as `(axiom exit_exit ((code Int) (w World)) (= (w_exit (exit code w)) (some
code)))` (v3/examples/calc/calc_bin.shard). The compiled route carries no proof (`v3/build.sh`
says so), and the erasure's source calls itself a stated trust dependency. The contract says
nothing about memory, time, or a host call that has no axiom.

### What is not trusted, and why

A checked environment is extended in two ways only: by K's `check`, which rebuilds every node of
the declaration it is handed and reads none of the node data its author wrote, and by K's record
import, which builds its terms from text. The type is sealed ("The boundary" above). So a mistake
in a component below cannot get a declaration admitted that the logic does not derive.

| component | files | what a mistake can still do |
|---|---|---|
| reader, elaborator, unifier | `reader elab unify scope spelling matcher predef` | refuse good input; change which statement is proved (a.6) |
| proof steps | `tactic.shard`, `arith.shard` | fail to prove |
| automation | `engine.shard`, `prove.shard`, sidecar files, tactic libraries | fail; leave a stale stored proof, which is reported `pending auto_stale` and not used |
| the definer | `define derive template record inhabit` | define something other than intended; state a false obligation, which then stays in `params=` |
| the store writer | `store.shard` | write a bundle the release gate refuses |
| clients of K's services | any caller of `infer`, `whnf`, `is_def_eq` | get a wrong answer for forged input; no acceptance follows from an answer |
| formatter, scripts | `v3/tools/shardfmt`, `v3/*.sh` | a gate script that passes wrongly hides a failure |

Not cleanly outside the trust base:

- *The seal.* The V3 loader enforces it; the bootstrap, which runs the toolchain, does not. The
  toolchain's own files are held to it by loads through the V3 loader (`parity_test.sh`,
  `k_clients_test.sh`, `v3/build.sh`). Such a load refuses a breach: a file that imports
  `kernel/k/env.shard` from outside the directory gets `LOAD-ERROR FILE private_module:
  kernel/k/env.shard is inside the sealed directory kernel/k — import the directory`. The
  bootstrap reads the same file without a complaint (`eval dump`, exit 0).
- *The index and the on-demand walk* (a.4) and *the loader's account* (a.5): above K, yet inside
  the trust base for the claims named there.
- *The erasure*: its source calls it untrusted. That holds for (a); for (b) it is trusted.

## The back end

The back end turns the program a load yielded into a native binary. It runs inside the loader's
process (`load.shard --emit DIR FILE…`); no program text is printed and read back.

| stage | file | from → to |
|---|---|---|
| representation | `prog.shard` | a `Prog`: declarations with every name an identity, every call head one of constructor, function, primitive, extern, every variable an index |
| lowering | `lower.shard`, `lw_prog` | `Prog` → `Prog` in the restricted form, or refusals |
| generation | `cgen.shard`, `cg_prog` | restricted form → C text over `rt.h`, or refusals |
| driver | `emit.shard`, `emit_prog` | writes `DIR/main.c`, the functions in `DIR/pNNNN.c` (400 a file, included by `main.c`), `DIR/map` (each declaration: C name, kind, identity, source file) and `DIR/sources` (each module's file and hash); prints `EMIT: DIR entry=main.c modules=N declarations=M` |
| compiler | `cc -O2 -o OUT DIR/main.c -I v3/kernel` | one translation unit; `rt.h` uses GNU C extensions and Linux `mmap` |

**The restricted form.** An atom is a variable, a numeral or a symbol. A right-hand side is an atom
or a constructor, call, primitive or extern over atoms. A body is a right-hand side, a `let` of
one right-hand side, an `if` on an atom, or a `match` on an atom whose arms are flat (a
constructor over variables, a numeral, a symbol, a variable). The lowering gets there by lifting
an `if` or `match` in operand position to a new function over its free variables (named `f.7`: a
numeric component no source name has), compiling a nested pattern to a chain of flat matches, and
binding every operand. The restricted form is still a `Prog`, so `ev` runs it (`--lowered`).

**Primitives.** The eighteen bare-named ones (`= + - < <= sym_eq band bshr * mod / tmod ediv bor
bxor bshl sym_of_chars chars_of_sym`) and `Nat.land lor xor shiftLeft shiftRight beq ble` map to
runtime operations. `Nat.sub`, `Nat.div`, `Nat.mod`, `Int.tdiv`, `Int.tmod`, `Int.ediv`,
`Int.emod` become an expression that tests for the underflow or the zero divisor first. Any other
primitive of the table is refused: `Nat.gcd`, `Nat.pow`, and `Nat.add`, `Nat.mul` and the three
`Nat` decisions under those names (a function with a definition carries them as `+ * = < <=`).

**The generated C.** One `static value_t s_NAME(value_t p_0, …)` per function, the name mangled
injectively from the identity. A self tail call reassigns the parameters and jumps; any other call
is a C call. A `match` is a `switch` on the tag or the integer, or a chain of symbol tests. An
`if` tests the low bit of the condition's tag, which says "second constructor of its type" as the
evaluator's tags do, so a binary takes the branch `ev` takes on a value of any type
(`v3/pins/emit/if_second`).

### The runtime's data (`v3/kernel/rt.h`)

| value | representation |
|---|---|
| every value | one machine word |
| `Nat`, `Int` | the same thing: an odd word holding a 63-bit integer, or beyond that a heap cell of sign and 32-bit limbs; arithmetic is exact up to 2^3840 |
| a constructor | a pointer to a 16-byte-aligned cell: a 32-bit tag, a 32-bit field count, the fields. `true`, `false`, `nil`, `none` are one shared cell each |
| tags | odd for the second constructor of a type, even for every other. 900–908 for the constructors the runtime itself builds and reads (false 900, true 901, nil 902, cons 903, none 904, some 905, pair 906, the World 908) — the prelude's and Init's `Bool`, `List`, `Option`, `Pair`/`Prod`, and `Decidable`'s two as `Bool`'s; every other constructor two tags from 1000 in program order, of which it carries the odd one where it is its type's second |
| a list; `ByteArray`, `String`, `Array`; a string literal | cons cells ending in nil, one per element; a byte is a small integer in a 32-byte cons cell. A literal is rebuilt each time its code runs |
| a symbol | a cell holding an index into an intern table; equality compares indices |
| a closure | none exists: every call is saturated and names a top-level function |
| the World | one cell; the host operations return the cell they were given |

**Memory.** Cells are bump-allocated in a 64 GB reserved region and reclaimed by a conservative,
non-moving mark–sweep collector that runs only inside an allocation. Its roots are the C stack,
the callee-saved registers and the data segment; a per-slot bitmap of cell starts rejects words
that only look like pointers. Freed cells go to free lists by size. A collection follows every
64 MiB to 2 GiB of fresh allocation, in proportion to what the last one found live.

| limit | what happens |
|---|---|
| a `match` no arm takes; a refuted branch reached | `rt trap: inexhaustive match in F`, exit 3 |
| `/ mod tmod ediv` by zero — the toolchain's bare-named primitives; Init's `Nat.div`, `Int.ediv` and their kin are total | `rt trap: div by zero (stuck on the interpreted engines)` and its like, exit 3 (under `ev`: exit 4) |
| `bshl` or `bshr` by a count outside 0–63 — the toolchain's bare-named shifts | `rt trap: bshl shift out of range (stuck on engines)`, `bshr` likewise, exit 3. `Nat.shiftLeft` and `Nat.shiftRight` take a count of any size, as under `ev`: `Nat.shiftLeft 1 64` is 18446744073709551616, `Nat.shiftRight 5 64` is 0 (`v3/pins/emit/init_prim`) |
| an `if` on a number or a symbol | `rt trap: if on a value that is no constructor (stuck on the interpreted engines)`, exit 3 |
| an integer of 2^3840 or more | `rt trap: bignum too large (exceeds 2^3840)`, exit 3: a sum, a product, a `Nat.shiftLeft` whose result is that large (any count past 3840 on a nonzero number), where `ev` computes it. The toolchain's own binaries too: `v3/bin/load` ends so on a `--run` that reaches such a number and on a theorem K evaluates past it, where the loader on the bootstrap accepts both |
| the heap past 64 GB | `rt trap: heap exhausted` |
| `sym_of_chars` over 4,096 bytes | `rt trap: sym_of_chars: name too long` |
| recursion depth | a call that is not a self tail call uses the C stack; the limit is raised to 2 GB at start, best effort |

A program stuck at a primitive ends differently on each engine: the Rust bootstrap run directly
(`eval direct`) exits 2 with `error running entrypoint: …`; `load --run`, on either engine, exits
4 with `RUN: stuck guard in F`; a binary exits 3 with `rt trap: …`.

**Refused at the emit** (`EMIT: MESSAGE`, exit 2, nothing written; `v3/build.sh` prints
`FAIL: NAME: the emit (exit 2); EMIT: …`):

| message | cause |
|---|---|
| `unlinked sig fn NAME: the implementation's directory was not loaded (give it as a root)` | a view's implementation is not among the roots (also `unlinked sig type`) |
| `the primitive NAME has no row in the chain (the profile's eighteen and the bridge's rows only) in F` | a primitive with no runtime operation |
| `a string pattern has no form in the chain in F` | a string literal as a pattern |
| `a numeral pattern outside the runtime's small integers in F` | a numeral pattern beyond ±2^62 (`v3/pins/emit/refused_numeral_pattern`) |
| `the extern NAME has no shim in the runtime in F` | an extern other than `get_args read_file read_range read_dir write write_line write_file exit read_key` |
| `no fn MODULE.main to start from` | the root module has no `main` |
| `MODULE.main takes N parameters: a binary's main takes the World alone (a main with parameters before the World runs under the loader's --run only)` | a `main` with parameters before the World, which `--run` reads from the arguments (`v3/pins/emit/refused_main_params`); `takes no parameter` for a `main` of none |
| `the constructor NAME has N fields: a cell of the runtime holds at most 125` | a constructor of more than 125 fields: the allocator's size classes end at a cell of 1 KiB (`v3/pins/emit/refused_wide_ctor`; `wide_ctor` runs 125). Both evaluators run such a program |
| `the constructor NAME is named as a cell of the runtime and must not be the second constructor of its type` (or `must be the second`) | a constructor named `True`, `False`, `Nil`, `Cons`, `None`, `Some`, `Pair` or `World` carries that cell's tag, whose low bit is fixed: `True`, `Cons` and `Some` are second constructors, the others are not (`v3/pins/emit/refused_role_place`). Both evaluators run such a program |

**What is proven, and what is not.** Nothing about the back end is proven — not the lowering, the
generator, the runtime or the C compiler. A theorem about a function, or a `bin`'s requirement, is
about the program's meaning in the logic, not about the binary. What holds a binary to that
meaning is agreement by test: the lowering against the program under `ev` (`lower_test.sh`), and
the binaries against the Rust evaluator (the ties below). Where a binary's answer is used as a
verdict — the full Init replay runs on compiled K — it rests on those tests.

## The two engines

The toolchain — the loader, the checker K, the provers, the tests — is itself a set of shard
programs under `v3/kernel/`. Two engines run it.

**The Rust bootstrap** (`rust_bootstrap/`, about 3,700 lines; the binary is
`rust_bootstrap/target/release/eval`) is an evaluator and nothing else, with two commands.
`eval direct FILE ARGS…` reads FILE and the files it imports and runs FILE's `main` on
`(World 0)` with ARGS as the arguments; the exit code is the program's, or 2 when the closure does
not load or the run fails. `eval dump FILE` prints the loaded closure as a canonical text and runs
nothing. Its reader takes `type`, `fn`, `extern` and `record`, skips the forms of the logic
(`def`, `theorem`, `sig`, `use`, `bin`, …) and fails on any other. It resolves names flat: by
bare name across the whole closure, with no module paths and no check that a view hides its
implementation. It follows a directory import to the view and then the implementation, and does
not follow `(import Init NAME)`. It knows the eighteen bare-named primitives and performs the
host calls itself. So it runs the toolchain's own sources, which import no Init, and cannot run a
program written over Init's types: that one runs through `load.shard --run` or as a binary. It is
trusted by review; its 24 unit tests run in CI.

**The native binaries** (`v3/bin/NAME`) are the same sources compiled by the back end above. To
build one, the bootstrap runs the loader over the root's closure with `--emit`: the loader loads
the closure under its own rules, lowers it and writes C, and `cc` compiles that. The bootstrap
runs every emit, so no binary builds another. The binaries are faster and unproven, so the
bootstrap is the authority wherever the two differ.

`eval direct` is the one spelling for "run this toolchain program". The test scripts call
`$EVAL direct FILE ARGS…`; `EVAL` is the bootstrap by default, or the shim `v3/bin/eval`, which
answers `direct FILE ARGS…` by running `v3/bin/STEM ARGS…` and exits 70 with a message when that
binary is missing or stale. It never falls back to the bootstrap; `dump` always goes to it. The
loader's first line, `LOAD root=… route=3 engine=bootstrap`, is a label its caller may set
(`--route`, `--engine`); the loader does not detect its engine.

| use | engine |
|---|---|
| the suite by default; every emit; the reference side of every tie; `eval dump` | bootstrap |
| the full Init replay (`v3/bin/t0`); the index build when `v3/bin/initindex` is newer than its sources; the formatter (`v3/bin/shardfmt`); the suite's second run | native |

| check of their agreement | compares |
|---|---|
| `parity_test.sh` | the bootstrap's reader and the V3 loader's reader over every toolchain root: the two programs printed as one canonical text, byte for byte |
| `route2_test.sh` | K run directly by the engine against K loaded by the V3 loader and run under `ev` (`k_clients_test.sh` runs K's tests the second way) |
| `lower_test.sh` | a program under `ev` as loaded and in its restricted form |
| `native_tie_test.sh` | fixed workloads under the bootstrap and the binaries: stdout, stderr, exit status and every file written, byte for byte, nothing normalized |
| `suite_tie.sh A B` | two whole runs of the suite, one per engine, entrypoint by entrypoint; only counts of seconds, sizes in kB and scratch names are normalized |
| `v3/t0_full.sh` | K's verbose output on the export's first 100,000 lines under both engines, before the full replay |

A difference is a defect of the back end; the fix goes there and the build is rerun.

## Build and gates

### `v3/build.sh`

| command | does |
|---|---|
| `v3/build.sh [SRC [OUT]] [--root DIR] [EXTRA…]` | builds SRC's closure (default `v3/kernel/t0.shard`) into OUT (default `v3/bin/STEM`); `--root` is the loader's root (default `v3`); EXTRA are further root directories and can only follow OUT. Exit 1 on failure |
| `v3/build.sh --all` | builds the inventory, `V3_BUILD_JOBS` roots at a time (default 8), keeps a binary whose stamp is current, writes the shim; exit = the roots that failed |
| `v3/build.sh --list`, `--stamp NAME`, `--shim` | the inventory, `NAME SRC` per line; the stamp `v3/bin/NAME` would carry now (empty: never emitted); writes `v3/bin/eval` |

*The inventory* is 31 roots: the five drivers (`load`, `prove`, `verify_release`, `t0`,
`initindex`), `test/calc_harness`, and every `v3/kernel/test/*_test.shard` and
`v3/kernel/k/test/*_test.shard`. The formatter is built on demand by `v3/fmt.sh`.

*Extra roots.* A root that uses a directory module through its view needs the directory as a
further root, or the emit refuses `unlinked sig fn`. K's directory `v3/kernel/k` is added
automatically when no EXTRA is given; others are named, as `v3/fmt.sh` names the formatter's
`print` and `reads` directories.

*The stamp* (`v3/bin/NAME.stamp`) is a hash over the root's closure sources, the `.shard` files of
its extra directories, every `v3/kernel/*.shard` and `v3/kernel/k/*.shard`, `rt.h` and the compiler
flags. Any edit of a kernel source makes every binary stale. It does not cover the Rust evaluator
or the C compiler. A build hashes every file that can enter its stamp before the emit reads any,
and compares the stamp's files with those hashes after the compile: when one was edited, created
or removed while the build ran, the build fails with `FAIL: NAME: changed during the build:
FILE…` and leaves no stamp beside the binary. A file outside the stamp, or one touched with its
content unchanged, fails nothing (`build_test.sh`).

*What a build writes.* By default everything is under `v3/bin/` (untracked): `NAME`,
`NAME.stamp`, `NAME.build.log`, `emit/NAME/` (`main.c` and its parts, `map`, `sources`, `root`,
`extra`). NAME is OUT's last component. With an OUT elsewhere the binary and `OUT.stamp` stand
there, and the log and the emit directory are still written under `v3/bin/` by that name, so two
builds whose outputs share a last component overwrite each other's. A failed C compile prints
`FAIL: NAME: cc:` with the compiler's first `error:` line, then the path of the log, which holds
the compiler's whole output. A root over Init needs a current Init index; the build never builds
one.

### `v3/test.sh`

`bash v3/test.sh` runs every *entrypoint* — a test program or script — `V3_TEST_JOBS` at a time
(default 8) and exits with the number that failed. The entrypoints are found by pattern:
`v3/kernel/test/*_test.shard` and `v3/kernel/k/test/*_test.shard` (run as `$EVAL direct FILE`; the
exit code is the failure count), `v3/kernel/test/*_test.sh` and `v3/tools/*/test/*_test.sh`.
`V3_ENGINE=native` runs them on the binaries through the shim; `V3_TEST_OUT=DIR` keeps each
entrypoint's output, exit status and seconds for `suite_tie.sh`; `V3_NATIVE=1` adds the native tie.
Some shell tests keep a scratch root under `v3/` (`v3/NAME_XXXXXX`) and remove it on exit.

| entrypoint | purpose |
|---|---|
| `entry_test.shard` | command-line arguments validated against an entry's parameters before it runs |
| `ev_test.shard` | `ev` on hand-built programs: each rule, guards, fuel, externs, deep recursion |
| `expr_test.shard` | K's term utilities and a node's hash, range and flags |
| `handle_test.shard` | `prepare` and `invoke`: refusals by name, the two doors, one handle invoked many times |
| `hostile_test.shard` | declarations K must refuse, each for its stated reason, through K's view |
| `json_test.shard` | the JSON reader |
| `k_client_test.shard` | a client of K's view builds a raw declaration, checks it, reads the result |
| `level_test.shard` | universe levels |
| `loader_pins_test.shard` | every `v3/pins/loader/CASE` loaded and compared with its `;; expect:` line |
| `loader_test.shard` | the loader beyond the pins: records, visibility, a loaded program run |
| `matcher_test.shard` | a generated matcher is recognized from its type; an imposter is not |
| `prims_test.shard` | every primitive under `ev`: a positive, a negative and a boundary case |
| `prog_test.shard` | a `Prog` built by hand and looked up |
| `project_test.shard` | definitions erased to programs and run: decisions as conditions, a matcher |
| `reader_pins_test.shard` | every `v3/pins/reader/*.shard` read and compared with its `;; expect:` line |
| `reader_test.shard` | the reader: every form, scope resolution, the refusals with their reasons |
| `rows_test.shard` | `String`, `ByteArray`, `UInt8`, `Fin` and structure projections as run-time values |
| `session_test.shard` | sessions: revisions, handles released and revoked, declarations built as data |
| `sexpr_test.shard` | the s-expression reader and every one of its errors |
| `unify_test.shard` | the unifier's outcomes: exhausted, blocked, assigned, refused |
| `k/test/accel_test.shard` | the accelerated primitives' reference matcher, inside K's directory |
| `k/test/add_test.shard` | admission to the environment, quotients, the one node counter |
| `k/test/inductive_test.shard` | inductive types admitted, their recursors generated |
| `k/test/nested_test.shard` | nested inductives |
| `k/test/tc_test.shard` | the type checker on a hand-built environment |
| `bin_test.sh` | calc's `bin`: record, requirements, store, trace, native binary, two negatives |
| `build_test.sh` | `v3/build.sh` on one small program through a stand-in C compiler: the first error of a failed compile shown, a source edited during the build refused a stamp |
| `calc_main_test.sh` | calc's `main` under `--run` prints the model's output, byte for byte |
| `calc_test.sh` | calc's functions under `ev` against recorded reference answers |
| `define_test.sh` | calc's functions get definitions with equations; theorems citing them accepted |
| `derive_test.sh` | `derive`: the library's procedures register; a program over derived equality runs |
| `engine_test.sh` | the proof engine: sidecars replay and regenerate identically; an obligation delegated with `auto` is proved, entered under its name and discharged; its reach over calc |
| `handle_cost_test.sh` | only `prepare` links; no other file names a handle's or session's internals |
| `imp_test.sh` | the imp machine model loads, its theorems are accepted, stored and replayed |
| `index_test.sh` | the Init index built and read back; four tampered inputs refused |
| `k_clients_test.sh` | K's tests loaded by the V3 loader and run under `ev`, so K's view is enforced |
| `loader_pins_list_test.sh` | the cases `loader_pins_test.shard` lists are the directories of `v3/pins/loader/`: none unlisted, none missing, none twice |
| `lower_test.sh` | seven programs as loaded and under `--lowered`: same streams, same exit |
| `messages_test.sh` | what a refusal says after its reason: the counts of an arm's names, the import a horizon is attributed to, the import to write, the row that makes rows unreachable, what to write for `mutual`, `instance`, a bare `true` |
| `parity_test.sh` | the two readers agree on every toolchain root (above) |
| `path_test.sh` | one example end to end — load, run, store, replay — and five breaks that must fail |
| `route2_test.sh` | K under `ev` prints what K run directly prints, on 3,000 lines of the export |
| `session_emit_test.sh` | declarations built as data compile under their names; the binary matches the handle |
| `state_test.sh` | K's sources never advance one version of the checker's state twice |
| `std_test.sh` | every file of `v3/std` loaded in one process, stored, replayed by K alone |
| `store_test.sh` | the store and the release check: replayed clean; tampered stores refused |
| `t0_fixture_test.sh` | K on 3,000 lines of the export: all accepted, axiom closures equal the oracle's |
| `t0_gate_test.sh` | `t0_full.sh`'s tie under stub engines: agreeing on a failure does not pass |
| `tactic_lib_test.sh` | a tactic library as an ordinary program: replay, regeneration, two producers |
| `tactic_test.sh` | calc's 100 claims as theorems by proof steps, each accepted by name |
| `template_test.sh` | templates: instances within the budget; the dumped programs hold no lambda |
| `wire_test.sh` | externs over `ByteArray` and Init's types against the real host; bad ones stuck |
| `tools/shardfmt/test/shardfmt_test.sh` | the formatter: scenarios, command line, contract, and the format gate |
| `native_tie_test.sh` (`V3_NATIVE=1`) | the native tie (above); needs every binary of the inventory current |

### The other gates

**The format gate.** `shardfmt_test.sh` fails when a tracked `.shard` file under `v3/` that an
author writes is not its own canonical text. `v3/fmt.sh [FILE…]` lists such files (exit 1 when
there is one), `v3/fmt.sh --fix [FILE…]` rewrites them, `v3/fmt.sh --list` names the gated files.
Not gated: `*.auto.shard` and `v3/kernel/k/accel_pins.shard`, which tools write.

**The tree-unchanged check** is the native tie's last step: after its workloads, `git status`
must show no tracked file under `v3/` modified and no new file under `v3/kernel`, `v3/pins`,
`v3/examples` or `v3/std`. Tools and tests write only to untracked places: `v3/bin/`,
`v3/.cache/`, `v3/release/`, `.shard-cache/` and scratch directories.

**The Init replay.** `v3/t0_full.sh [LOG]` needs `v3/bin/t0` (`v3/build.sh`) and the export. It
ties the two engines on the first 100,000 lines, then replays the whole export on the binary and
requires three things: the verdict line of `v3/t0_expected.txt`, which reads
`T0: accepted 57977  rejected 0  exhausted 0  mismatched 0  unsupported 0`; every constant's axiom
closure equal to Lean's own (`v3/t0_axioms_cmp.sh`); and all 20 accelerator candidates pinned.
`V3_T0_SKIP_FULL=1` stops after the tie. The replay holds about 30 GB resident;
`V3_T0_RSS_CAP_KB` (default 400 GB) kills it past the cap.

**CI** (`.gitlab-ci.yml`), one pipeline per push:

| job | stage | runs |
|---|---|---|
| `host` | host | `bin/ci_docs_only.sh --self-test`; the Rust evaluator built; `cargo test` |
| `v3` | gate | the export; the suite on the bootstrap (outputs kept); `v3/build.sh`; `v3/t0_full.sh` |
| `native` | gate | the export; `v3/build.sh --all`; the native tie; the suite on the binaries (outputs kept) |
| `suite_tie` | tie | `suite_tie.sh` over the two kept runs |

*Documents-only pushes.* When every file a push changed ends in `.md`, `v3`, `native` and
`suite_tie` print `DOCUMENTATION-ONLY PUSH` and end green in seconds (`bin/ci_docs_only.sh`). Such
a green pipeline says nothing about the tree: cite the pipeline of the push that carried the code.
The whole push is examined, so code pushed with its documents is gated, and any doubt (a new
branch, a manual run) runs the gates. `glab ci run -b main` forces a full run.

*Durations the scripts record:* the loader's own closure (9,003 declarations) emits in 34 s on
the bootstrap; the export takes 77 s and its axiom oracle 15 s; the full replay 745 s at 30 GB;
the index about 1,950 s on the bootstrap on the CI runner, 14 times the binary's. CI's timeouts
are 30 min (`host`), 3 h (`v3`, `native`) and 20 min (`suite_tie`).
