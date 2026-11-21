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
   ├──► ev, run                                         the evaluator: what "run" means
   └──► lower, cgen, emit, rt.h  ──► cc                 the compiler to native binaries (unproven)
```

`loader.shard` drives all of it for one load; `load.shard` is its command line.

## The files

Paths are under `v3/kernel/` unless they start elsewhere. "Rests on" is what a file imports;
"called by" is who uses it.

### Source text, modules, the loader

| File | What it does | Rests on | Called by |
|---|---|---|---|
| `sexpr.shard` | The s-expression reader: bytes → `SExpr` (numeral, symbol, symbol with universe list, string, list); `read_all` for a whole file with the error's byte offset; `show_sexpr` printer; `size_sexpr`. No name has a meaning here. Every function of it is a definition: the reader of a string descends by the bytes left, the four that call each other by a fuel. | `util.shard` | `scope.shard`, `record.shard` (imports); `loader.shard` and `reader.shard` (`read_all`), `derive.shard`, `engine.shard`, `prove.shard`; `v3/tools/shardfmt/reads` (the formatter's judge) |
| `descent.shard` | The descent facts of the toolchain's integer measures, theorems alone: `natAbs_lt`; `natAbs_pred`, a counter stepping down past a test that it is at least 1; `natAbs_div10`, a number losing a decimal digit; `natAbs_shr2_le` and `natAbs_shr2_lt`, an integer losing its two low bits. A function that recurses on an `Int` states `(measure (Int.natAbs n))` and each `fulfills` beside it cites one (133 functions in 33 files, K's directory among them; the forms: `LANGUAGE.md`, "Toolchain files and ordinary files"). No program is made of this file, and a build passes it over. | — | `util.shard`, `intmap.shard` (imports); every file with such a function (`(use kernel.descent)`) |
| `util.shard` | List, byte and decimal utilities: `len append rev nth take drop`, `halves` (a list dealt into two piles, what the merge sorts of `dump`, `index` and `initload` halve by, with the fact that each pile of a list of two or more is shorter), `is_ws is_digit`, `starts_with_lit` (with the fact that what it leaves is shorter where the literal has a byte), `show_nat show_int`, `hex_val`, `utf8_ok`; `utf8`, a literal's bytes (text here is `(List Int)`, a literal a `String`), and `utf8_all` for a list of them. The lowest file a program is made of. | `descent.shard` | `sym.shard`; nearly every kernel file (`sexpr`, `json`, `record`, `index`, `initload`, …) |
| `base.shard` | The base: the definitions every file sees beside Init's, as text the loader reads as the module `shard.int` when a file's forms first use one of its operators — `land`, `lor`, `xor` on `Int`, by cases on an integer's two constructors over Init's `Nat.land`, `Nat.lor`, `Nat.xor`. `base_decl` (a declaration of the base: the one native declaration a row of the registry speaks for), `uses_base`, `base_reserved` (no file's module is `shard`). | `util.shard`, `sym.shard`, `name.shard`, `sexpr.shard` | `elab.shard` (the operators at `Int`), `prog.shard` (the rows), `erasure.shard`, `loader.shard` |
| `sym.shard` | `Symbol`, an ordinary type: the bytes of a text (`MkSym`). `sym` (the symbol of a literal), `sym_eq`, `chars_of_sym`, `sym_of_chars`: ordinary functions; no reader knows the name and no engine stores a symbol in a form of its own. | `util.shard` | `name.shard`, `sexpr.shard`; every file that names a symbol |
| `name.shard` | Hierarchical names (`Anon`/`Str`/`Num`, Lean's `Name`) and the structural hash (`mix`, `hash_name`, `name_key`) every table is keyed by; a name's hash under a prefix from the prefix's hash and the name's components (`comp_hashes`, `hash_under`, `key_under`); `name_eq name_lt names_eq mem_name show_name`, `n1`, `n2`. | `sym.shard`, `util.shard` | `level.shard`, `expr.shard`, `verdict.shard`, `index.shard`, `initload.shard`, … — the base of everything that names |
| `intmap.shard` | Persistent map keyed by a non-negative Int (4-ary trie): `im_find im_insert im_has`, each a definition: the lookup and the insert recurse on the map, and `im_split`, which parts two keys, under a measure over both with its descent proven. | `descent.shard` | `expr.shard`, `prog.shard`, `ev.shard`, `index.shard`, `initload.shard`; through them `scope`, `loader`, K's tables |
| `scope.shard` | What a citation means in a file: `Scope` (declaration prefix, module tag, opened prefixes, visible set; each prefix held with its hash, built by `scope_of`, `open_of`, `open_some`), the elaboration context `Cx`, `resolve` / `scope_candidates` / `resolve_ctor`, `scope_keyed` (the candidates of a citation, each with its key in the environment: the citation's bytes are hashed once), the lookups at a key the caller holds (`tbl_find_at`, `visible_at`, `native_decl_at`, `ords_find_at`, `ords_add_at`; `keyed_all` gives names their keys), `visible`, `init_cited` (`Init.NAME`), the renamed-primitive table, `type_head`. | `sexpr.shard`, K's view (`k`: `CheckedEnv`, `env_find`, `env_find_at`), `expr`, `intmap`, `name`, `util` | `etable.shard`, `matcher.shard`, `elab.shard` (imports); used by `reader`, `classify`, `erase`, `predef`, `loader` |
| `spelling.shard` | Two fixed tables over Init's names: `sp_canon` (a class method under Init's instance read as its function, e.g. `HAdd.hAdd Nat …` → `Nat.add`), `sp_instance` (the instance term for an instance-implicit binder at Nat/Int/Bool/List), and `sp_kit` (every constant the tables can write, for the Init demand). | `kw.shard` (K's view, `expr`, `level`, `name`) | `unify.shard` (import); `elab`, `tactic`, `arith`, `engine`; `loader.shard` (`sp_kit` in `init_kit`) |
| `reader.shard` | The reader of the logic's forms: one S form → K `Declaration`s (`def abbrev opaque theorem axiom inductive type structure`; the `type` forms of types that name each other → one block, `read_types`), view parameters (`sig`, `requirement`), `fulfills` and discharge proofs, the proof forms (`exact`, `by`, `auto`, `sorry`) with the sidecar key (`Req`, `req_key`, `auto_fit`) and the memo of its walk (`ReqMemo`: a declaration's part of every key that reaches it, `req_node`); routes `fn extern realize fulfills bin` and the directives to the loader; refuses reserved and Lean keywords with a pointer. | `elab.shard`, `engine.shard`, `ev.shard`, `etable.shard`, `inhabit.shard`; `tactic`, `scope`, `sexpr` | `loader.shard`, `predef.shard`; tests through `test/reader_kit.shard` |
| `loader.shard` | The loader: package root, paths → module paths, imports (file, directory, Init) with cycle/collision/seal checks, `use`/`trusts`, the Init demand per file, per-form dispatch (K admission, policy check, `fn`/`extern`/`realize`/`derive`/`bin`/`fulfills` pipelines), views and the implementation check in a fork, the sidecar, the record type `Rec` with `show_rec`, the counts and the exit code. | `host`, `reader`, K's view, `realize`, `define`, `template`, `record`, `derive`, `initload`, `elab`, `tactic`, `arith`, `thread` | `load.shard`, `prove.shard`, `run.shard`, `test/calc_harness.shard`, `test/loader_kit.shard` |
| `load.shard` | The loader's driver (`main`): parses the command line, loads the files, prints the records and the counts; the modes `--dump`, `--emit`, `--run` (with `--fuel`, `--trace`, `--lowered`); `--facts OUT` (`write_facts`); `--unchecked`, the reading of a build. | `loader`, `ev`, `dump`, `facts`, `emit`, `lower`, `run` | nothing imports it; run as `eval direct v3/kernel/load.shard` or `v3/bin/load` by every test script and `v3/build.sh` |
| `initload.shard` | Init on demand: opens the export by its index (meta line against the pin `init_pin_*`, index header, export size), looks names up (`od_lookup`, `od_names_under`, `od_has_under`), walks the closure of a set of records by range reads and feeds it to K in export order (`od_load`). | `host`, `json`, `intmap`, `name`, `util`, `index`, K's view | `loader.shard` |
| `index.shard` | The Init index's format, builder (`ix_build`) and verifier (`ix_verify`): header row, record table (ordinal, kind, offset, length, first ids), name table (sorted, name → ordinal), declaration table (name id → ordinal); all read by range. | `host`, `json`, `intmap`, `name`, `util` | `initindex.shard`, `initload.shard` |
| `initindex.shard` | Driver for the index: `initindex.shard EXPORT INDEX` builds, `--verify EXPORT INDEX` reads back; one line of output, exit 0 or 1 (2 on usage). | `index.shard` | nothing imports it; run by `test/init_index.sh` and `test/index_test.sh` |
| `verdict.shard` | The public vocabulary of K's answers beside the sealed checker: `Resource`, `Reason`, `(Outcome E)`, `Verdict`, `Failure`, `(KRes A)`. Data only. | `expr.shard`, `name.shard` | K (`k/env.shard`, `k/mod.req.shard` and the files using `kernel.verdict`), `kw.shard`, `loader.shard` (`Outcome`, `Resource`), `initload.shard`, `test/decls_kit.shard` |
| `dump.shard` | A loaded program as canonical text, one sorted line per declaration (`type`/`fn`/`extern`/`sig`/`sigtype`), for comparison with the bootstrap's `eval dump`. A projection: names print as their last component. | `prog.shard`, `etable.shard` | `load.shard` (`--dump`); compared in `test/parity_test.sh` |
| `json.shard` | A JSON reader over UTF-8 bytes (`json_parse`, `jget`, `jstr`); numbers kept as digit strings, key order kept. Every function a definition: a value, an array and an object, which go on from the rest a call of their own returned, are bounded by a fuel handed down beside the input — the input itself, less a byte at each call made after one was read. Proven beside them, branch by branch: a parsed value leaves less input than it was given (`json_parse_lt`), and the parse is the same at every fuel as long as the input or longer (`json_parse_fuel`). | `util.shard` | `index.shard`, `initload.shard`, `k/import.shard`, K's view (imported there as public vocabulary), `test/json_test.shard` |

### Declarations, definitions, programs

| file | lines | what it does | rests on | called by |
|---|---|---|---|---|
| `decl.shard` | 186 | The data of K's declarations: `Declaration` (raw input: axiom, definition, theorem, opaque, inductive block, quotient) and `ConstantInfo` (a checked environment entry), reducibility hints, safety. Shapes follow Lean's export format field for field. | `expr.shard` | K (`k/`), `reader`, `elab`, `define`, `realize`, `derive`, `loader` — everything that builds or inspects a declaration |
| `prog.shard` | 489 | The program representation: `EType`, `ELit`, `EPat`, `ETerm` (ten forms of a program, and the lambda of a body as read), `ERec` (recursion clause), `EDecl` (inductive, function, extern, sig fn, sig type), `Prog`, values `Val`, results `EvRes`; `ETerm.size`, a term's derived size, with its arm's and their lists', by which the walkers `calls_acc`, `calls_all` and `calls_arms` are measured, their descent proven beside them; the primitive table (identity → operation code, arity); the registry both readers write by (`realization_of`: an operation of Init → its entry; `realization_expr`, `realization_type`: an expression row and its type); the identities of the runtime's cells. | `util`, `intmap`, `name` | `etable`, `classify`, `erase`, `erasure`, `thread`, `ev`, `lower`, `dump`, `loader` |
| `etable.shard` | 334 | The program table `ESt`: every program-side identity findable by any suffix of its spelling, registered per file before any body is read (forward references); a re-registered identity overrides (how a view's signature is linked to its implementation). `EInfo` kinds incl. `EITemplate`. | `scope`, `prog` | `erase`, `classify`, `erasure`, `realize`, `define`, `template`, `thread`, `loader` |
| `erase.shard` | 838 | Roles over the checked environment: which inductives may appear in programs (`ind_eligible`), an L type as a program type (`l_to_etype`), the role of each binder of a constant (`BType`, `BErased`, `BData`, `BStatic`), level solving, the registry's one-field structures (`rep_row`), package types. A type is read into the types it names to a depth the caller hands down (`budget_depth`); every function is a definition. | `etable`, `kw` (the walk over K) | `classify`, `erasure`, `realize`, `define`, `template`, `loader` |
| `classify.shard` | 3994 | The program-first reader (`route=e_first`): `fn`, `extern`, `type`, `sig` forms read straight into `prog.shard` data with every head resolved and saturated; a call's written arguments sorted by the head's declared signature (`written_data`: runtime data kept, a type or a proof passed over, a proof only where the typed route read the form — `classify_checked`); Init's constants with a fixed program (`init_row`), the registry's rows, projections, `dite` and decisions (`k_head`, `read_cond`); binders that are proofs, types that name values (`read_params`, `type_args`); numeral rows as tests and a `let` of several bindings as lets (`read_ematch_lit`, `let_chain`); the operators' entries from the types the binders state (`settle_ops`: the type expected, else the operands'; `=` at Bool and at a derived equality; a template's call in the elaborator's order, `settle_tm`), a row's expression written last (`expand_rows`); exhaustiveness, private-match and comparison-type checks; pre-registration of a file's heads; static types of program terms (`c_type`); the instances a body takes and the lambdas it lifts (`take_instances`, `take_lambda`: a lambda, a name, a registry constant, a package as shapes); a supplied `realize` read as the fn it is (`classify_realize`). | `etable`, `erase` | `loader` (`load_e_first`, twins of `type`/`sig`), `realize`, `thread`, `template` |
| `elab.shard` | 2596 | The elaborator: source terms → K's `Expr`, bidirectional, a definition by a depth its callers hand down (`elab` tests it and hands on one less; a term nested deeper is refused `term_depth`), with metavariables for implicit arguments and levels solved by first-order unification; operators by operand type; `if`/`dif`; `match` (generates and admits a matcher); numerals, strings, `list`, `let`, binders; the three coercions; instance table lookup. | `scope`, `unify`, `matcher`, `spelling`, `kw` | `reader` (every L form), `predef`, `realize`, `derive`, `tactic`, `loader` |
| `predef.shard` | 227 | Reads a `fn` form once into a *pre-definition*: declared type, binder locals, a local for the function's own name, the elaborated body, the recursion clause, callee-locals for program-only callees. `PdSkip` = signature has no L reading; `PdErr` = refusal. | `reader`, `elab` | `erasure` (`erase_predef`), `define`, `template`, `loader` (`load_fn_with`, `probe_fn`, `read_group`, `load_ldef`) |
| `erasure.shard` | 2685 | The erasure of an L term to a program term by `erase.shard`'s roles; the walk state `Tr` and its K procedures; `erase_predef` (a `fn`'s program, or the verdict "template"); template calls → instance demands, lambda lifting (`CALLER.lam_N`), shapes and instance names. | `predef`, `erase`, `prog`, `matcher` | `realize`, `define`, `template`, `loader` |
| `realize.shard` | 1174 | `(realize NAME …)`: the supplied form (signature against the erased type, body elaborated at the constant's telescope, descent and callee-order checks) and the derived view; the equation generator `pj_equations` (leaves of the leading matchers, `eq_N` / `realize_N`, `fix_eq` proofs) shared with `define`. | `erasure`, `classify` | `define`, `template`, `loader` (`load_realize`) |
| `define.shard` | 2977 | The K projection of a pre-definition: non-recursive value; structural recursion by `T.brecOn` (`pj_struct`, generating `T.below`/`T.brecOn` for a program's type); measure by `WellFounded.fix` with obligations `f.dec_N`, each stated over the function's type parameters and every local in scope at the call (`pj_wf`, `pj_self`) — the data parameters as one tuple, a pair of it `PSigma` over its first component where a later parameter's type names that component and `PProd` where none does (`wf_tuple_type`, `wf_tuple`, `wf_projs`) —, in the order of the measure's type — `Nat.lt`, or Init's `Prod.Lex` for a measure of several parts (`measure_order`); a group as one fixpoint over a generated `f.args` type (`define_group`); `casesOn`, `noConfusion`, `inj` bundle for a new inductive, each type of a block its own (`rec_app_dep`: the type's motive and minors at its place in the block, `block_place`, the other types' constant at `PUnit`, `block_tail`). Verdicts `DfOk`/`DfSkip` (RUNNABLE)/`DfErr` (REFUSE)/`DfAux`. | `realize`, `erasure`, `predef`, `matcher`, `kw` | `loader` (`load_fn_with`, `load_ldef`, `load_template`, `load_group`, `bundle_after`) |
| `template.shard` | 787 | Templates (a `fn` or supplied `realize` with a function-typed binder): checks the declaration at placeholder shapes (`check_template`), builds an instance — program, L definition (the template applied), equations — for a demand (`build_instance`), or the program alone for a program-first taker (`build_instance_e`). | `erasure`, `realize`, `classify` | `loader` (`load_template`, `build_demands`, `build_e_demands`, `register_template`) |
| `derive.shard` | 1355 | `(derive TYPE CAPABILITY…)`: reads the type's constructors from K's environment and writes source forms — `ctorIdx`, `decEq`, `compare`, `renderOnto`/`render` as `fn`s; `size` as `def`s over the recursor with `eq_K` theorems, one per type of a block and per auxiliary type — which the loader then loads as the file's own; checks a `(CAP by NAME)` procedure's type. | `elab`, `matcher`, `unify`, `scope` (`derived_for`) | `loader` (`load_derive`) |
| `record.shard` | 447 | `(record …)` expanded at the s-expression level before anything is read: the `type`, accessor and updater `fn`s; `(make …)` and `(with …)` rewritten in every form of the file, also over a parameterless `structure`. | `sexpr`, `util` | `loader` (`expand_records`, per file) |
| `inhabit.shard` | 129 | Builds a default value of a K type structurally (a lambda for a function type; the first constructor whose fields all close) — the value an `extern`'s opaque constant is declared with. | `kw` | `reader` (`read_extern`) |
| `thread.shard` | 1368 | The use check over programs: World types and carriers, abstract tokens and boxes, the walk (`th_term`), a function's verdict (`th_fn`: `ok`, `world_twice`, `world_unsupported`, `world_forged`); second half: the same walk over the checker's own state, on request (`--state`). | `classify` (static types), `etable`, `prog` | `loader` (`rec_threads` at each file's end, `bin_threads`, `rec_states`) |
| `loader.shard` (declaration parts) | 5507 | `load_decl_go` dispatches each form; `load_e_form`/`load_fn_with` (the `fn` pipeline: pre-definition → erasure → definition; parking on a forward reference, read again once the callee has a declaration: `retry_each`, `callee_awaited`), `load_unheld_type` (a `type` naming types K does not hold: those below it gathered with their forms, `gather_types`, and admitted where it stands, block by block, `admit_gathered`/`admit_types`; the ones admitted ahead of their forms, `Load.tahead`, get their program's declaration there; else a program with its reason), `settle_groups`/`find_group`/`load_group` (groups; the search keeps how the readings of the waiting functions ended, `Reads`, and forgets one when what it rested on changes: `reading`, `reads_settled`, `ld_forget`, `ld_forget_all`), `load_ldef` (a recursive `def`), `load_extern`, `load_realize`, `load_derive`, `load_discharge` (obligations), `load_template`/`build_demands` (instances), `admit_core`/`policy_pass` (K admission and the axiom policy, each type of a block a subject: `decls_subjects`), `bundle_after` (the bundle of every type admitted), `check_bin`, `rec_threads`, `show_rec` (the printed records). | `reader`, `k`, `realize`, `define`, `template`, `record`, `derive`, `elab`, `tactic`, `thread`, `initload` | `load.shard` (the driver), `run.shard`, `prove.shard`, tests |

Related files outside the list that the area depends on: `reader.shard` (the L declaration forms
`def`, `abbrev`, `opaque`, `theorem`, `axiom`, `inductive`, `type`, `structure`, `sig`,
`requirement`, `extern`'s L constant; the refused Lean keywords), `matcher.shard` (generated
matchers and their read-back), `spelling.shard` (the instance table; class methods read as their
functions), `scope.shard` (name resolution, `renamed_prim`, the derivation table's lookup),
`ev.shard` (running a program; entry argument codec; effect trace).

### Proofs

| File | What it does | Rests on | Called by |
|---|---|---|---|
| `tactic.shard` (3,405) | The step elaborator. Reads a `(by STEP…)` block into `Node` data (`read_block`, `read_node`), runs each step on a goal (`tac_step`, one `step_*` function per form), threads the block depth-first (`tac_block`) and returns the closed proof term, the sorried goals, or a refusal (`tac_prove`). Also renders goals (`goal_of`, `goal_sx`), lists the steps a goal admits (`applicable`), and declares `reconstruction_version` (2). | `elab.shard` (terms, metavariables), `arith.shard`, `unify.shard` (`assign`, levels), `matcher.shard` (`mt_is_matcher`, for case steps in `reduce`), `spelling.shard` (`sp_canon`), `kw.shard`, K through its view. | `reader.shard` (`read_proof`, `read_block_proof`, `run_tactic`), `engine.shard`, `loader.shard` (`reconstruction_version`, `tc_kit`), `prove.shard`. |
| `arith.shard` (1,133) | `arith`'s back end. Reads a statement as a comparison (`ar_rel`), linearizes terms over atoms (`ar_lin`), builds the rows, finds weights by Fourier–Motzkin elimination (`fm_find`) or checks the given ones, and builds the proof from `Lean.Omega`'s lemmas (`ar_prove`). | `elab.shard` (locals, visibility of the kit), `spelling.shard`, `kw.shard`. | `tactic.shard` (`step_arith`, `applicable`), `engine.shard` (`eng_arith`), `loader.shard` (`ar_all_kit`). |
| `engine.shard` (558) | The default producer for `auto`: a fixed ladder of steps, each confirmed through `tac_step`, under a step budget and hints; renders `Node` data back to source text for the sidecar (`nodes_sx`). Declares `engine_version` (1). | `tactic.shard`, `arith.shard`, `elab.shard`. | `reader.shard` (`read_auto`, `read_measured`), `prove.shard` (the version, in the header and the sidecar comment). |
| `prove.shard` (325) | The `prove` program: loads the files under mode `solve` or `measure`, prints the records and counts, rewrites the sidecars of the files that gained a block — an implementation's beside the implementation file — or were read at an older schema or reconstruction version. | `loader.shard` (the whole load), `reader.shard` (`Auto`, `Req`), `engine.shard`, `tactic.shard`. | A root program: `eval direct v3/kernel/prove.shard`, `v3/bin/prove`; tests `engine_test.sh`, `tactic_lib_test.sh` (`parity_test.sh` lists it among the drivers). |
| `unify.shard` (909) | Metavariables and first-order unification: the metavariable context (`MCtx`), assignment with occurs and scope checks (`assign`), `unify` with four outcomes (yes, no, stuck, exhausted), level unification, instantiation (`inst_mvars`). A closed mismatch is decided by K's `is_def_eq`. The twelve functions of `unify_go` call each other and are one definition, measured as K's checker is: the depth, a rank, a list's length (47 descents at the end of the group). | `kw.shard`, `spelling.shard`, K's view. | `elab.shard` (every term), `tactic.shard` (`tac_match`, `rfl`, `apply`), `engine.shard`, `derive.shard`. |
| `matcher.shard` (646) | Generates the auxiliary definition a `match` elaborates to (`mt_generate`) and recognizes one by regenerating it from its type (`mt_is_matcher`). | `kw.shard`, `scope.shard`, K's view. | `elab.shard` (`elab_match`), `tactic.shard` (the case step of `reduce`), `define.shard`, `realize.shard`, `erasure.shard`, `derive.shard`, `template.shard`. |
| `kw.shard` (187) | The walk over K's terms from outside K: an environment, a local context and checker state (`W`), with K's `whnf`, `infer`, `is_def_eq`, `is_prop`, local creation and binder closing wrapped so that a failure is a value. | K's view only. | Every elaborating file: `unify`, `elab`, `tactic`, `arith`, `spelling`, `matcher`, `reader`, `loader`, `define`, `realize`, `derive`, `erase`, `erasure`, `template`, `classify`, `inhabit`, `run`, `predef`. |
| `elab.shard` (2,596), proof-related parts | The term elaborator the steps call for every TERM, PROP, LEMMA and FACT (`elab`, `elab_type`); the elaborator state `El`; `el_finish` and `unsolved_msg` (the refusals for undetermined arguments); `decision_of` and `no_decision` (what `decide` and `if` can decide); `el_sees` (can the module cite this constant); the `pending_obligation` refusal on citing a pending name. | `unify.shard`, `matcher.shard`, `scope.shard`, `spelling.shard`, `kw.shard`. | `reader.shard`, `tactic.shard`, `arith.shard`, `engine.shard`, `derive.shard`, `loader.shard`. |
| `loader.shard` (5,507), proof-related parts | Reads the sidecar once per file, and once for the implementation file of a view where its check begins (`read_sidecar`, `sidecar_split`, `sidecar_entries`; `sidecar_schema` is 3); computes the file's parts of the key (`req_ctx`) and gives each accepted theorem its key (`accept_key`), the walk's memo kept in one field with K's memo of axiom closures (`Memos`); turns the reader's answers into records (`RPending`, `RProved`, `REngine`, `RTactic`, `RSidecar`, `RRecon`; `rd_plain`); keeps the pending names (`Load.pending`); handles `(fulfills f.dec_N PROOF)` (`load_discharge`, `settle_discharge`, `fulfills_cycle`) and a view's `fulfills` (`fulfil`); checks a `bin`'s `requires` against pending theorems (`bin_requires`); prints every record (`show_rec`) and the counts. | `reader.shard`, `tactic.shard`, `elab.shard`, K. | `load.shard`, `prove.shard`, `run.shard`, the loader tests. |

#### Other files on the path of a proof

| File | Why it matters here |
|---|---|
| `reader.shard` (1,128) | The entry for every proof: `read_proof` and `read_proof_go` choose among `(exact …)`, `(by …)`, `sorry`, `auto` (the `bad_proof` refusal is here); `read_auto` computes the key (`req_key_alone`: `req_key` from no memo, `req_walk`), fits the sidecar entry (`auto_fit`) and, under mode `solve`, calls the engine or a named tactic (`run_tactic`); `read_fulfills` and `read_discharge` do the same for `fulfills`. |
| `spelling.shard` | `sp_canon`: Lean's class-method spellings read as this language's function spellings in goals, lemmas and `arith` rows; `sp_instance`: the fixed instance table behind `instance_needed`. |
| `load.shard` (380) | The `load` program: the records printed, the exit code (refusals plus errors). |
| `k/add.shard` | `identity_hash`, the content fingerprint. |

### K, the Init import, the replay gate

#### `v3/kernel/k/` — the sealed directory (one module, `kernel.k`)

- `mod.req.shard` (108) — the view: 8 `sig type`s (CheckedEnv AxMemo Tabs Run Ctx LDecl Loc Limits)
  and 64 `sig fn`s; the only way in. Rests on the public vocabulary (host name level expr
  decl intmap json verdict). Imported (`(import "k")`) by kw, loader, scope, initload, t0;
  tests hostile_test, k_client_test; pins k_client_{reach,admit,transfer}.
- `k.shard` (13) — the implementation's import list; declares nothing. The view's signatures are
  matched against what these imports declare (`load.shard --root v3 v3/kernel/k`).
- `env.shard` (39) — `CheckedEnv` (constants by 24-bit name key with buckets, quotInitialized,
  pinned names, node-id watermark) and `env_empty`; states the one-environment invariant.
  Rests on decl, intmap, verdict. Used by every other K file.
- `tc.shard` (2,761) — the type checker: `infer` (checking and inference-only modes), `whnf_core`,
  `whnf`, `is_def_eq` in Lean's order (quick check, proof irrelevance, lazy delta, eta, structure
  eta, string literal expansion, unit-like); memo tables; `guard` (depth and heartbeat budgets);
  literal typing gated on pins; `reduce_nat` / `nat_apply` (the 14 accelerators); `reduce_native`
  (Unsupported). Its 60 functions that call each other, `infer` to `defeq_step7`, are one
  definition of the logic, each measured by the depth, then its rank among the calls that stay
  at one depth, then what a walk or a loop of its own descends by; the 189 descents are proven
  at the end of the file. The depth steps at the three guard sites (`infer`, `whnf_core_c`,
  `is_def_eq_core`) and where `reduce_nat` normalizes an argument; a loop (`whnf_loop`, the two
  lazy-delta loops) goes round only when the heartbeat count advanced (`loop_advanced`), and
  is out of heartbeats otherwise; the walk to a projection's field ends at its index. Also `Ctx`, `LDecl`, `Loc`, `env_find`, `env_add`, `st_for`, `st_seed`.
  Rests on env, expr, level, name, intmap, util, verdict. Called by inductive, nested, add,
  axioms; through the view by kw (hence elaborator, unifier, tactics, definer, erasure).
- `inductive.shard` (1,366) — `add_inductive_core`: check the types, declare them, check and
  declare constructors (positivity, universes), elimination level, K-target, recursor types and
  rules, then `check_recursors` (each rule type-preserving). Budgets `budget_depth` 8192,
  `budget_ticks` 100,000,000. Its nine walks down a telescope (`cit_binders`, `is_rec_argument`,
  `check_positivity`, `cc_binders`, `eoz_single`, `mri_open_type`, `open_ctor`, `open_pis`,
  `open_ctor_plain`) go on with a body they substituted into, or with what `whnf` made of it,
  so no term measures them: each spends a heartbeat at every binder (`a_step`, the checker's
  guard) and calls itself only when the count advanced below the limit since it was entered
  (`a_advanced`), out of heartbeats otherwise; the heartbeats left are the measure, and the 16
  descents are proven beside the walks. The fresh universe name is looked for among one more
  candidate than there are names taken. Rests on tc. Called by nested.
- `nested.shard` (1,050) — nested inductives: uniform-occurrence check, translation to auxiliary
  mutual types (`_nested` prefix reserved), admission through inductive.shard, translation back;
  `add_inductive` is the entry. The worklist of auxiliary types and the search for an unused
  auxiliary name stop at 100,000 (`nested_max`), out of heartbeats: the pin's two loops have no
  bound. Rests on inductive. Called by add.
- `add.shard` (519) — admission: `check_decl_core` per kind, `add_quot` (checks `Eq`'s shape,
  builds the four constants), `identity_hash` (a report fingerprint), the pin match
  (`pin_if_matches`, `ref_closure_ok`, `ref_matches`), the raw entry `check` / `check_with`
  (ingest, then check, then pin), `Limits`. Rests on nested, accel_pins. Called by import
  (`check_decl_pinning`, no ingestion) and, through the view's `check`, by loader, kw (`w_admit`),
  tests.
- `axioms.shard` (277) — `axioms_of`: a constant's axiom closure as a least fixpoint over
  "cites in type or value", inductive blocks resolved as a unit, memo (`AxMemo`) kept outside the
  environment; `ci_used`, `induct_ctors`, `names_union`. Rests on tc. Called by import (`-a`),
  loader (`policy_pass`, `check_bin`), reader (`ci_used` for requirement keys).
- `import.shard` (854) — the lean4export NDJSON import: table records (names, levels, terms,
  each compound term minted from the run's state), declaration records → `Declaration` →
  `check_decl_pinning`; an inductive record's exported types, constructors and recursors compared
  field by field with what K generated (`MISMATCH`); `Run` (tables, environment, state, five
  counts, axiom mode); `empty_run`, `run_on`, `run_resume`, `process_record`, `process_lines`,
  `show_verdict`, `show_counts`, `record_constants`. Rests on add, axioms, json, host.
  Called by t0, initload (`od_feed`), hostile_test.
- `accel_pins.shard` (5,136, generated below its marker) — `pin_candidates` (20 names),
  `is_pin_candidate`, then 206 reference declarations `rd_H` over shared term nodes `re_K` and
  names `rn_H`, the lookup `accel_ref` and `accel_ref_closure`. Produced by `gen_pins.sh`;
  checked by `t0_full.sh` (all 20 pinned on the export) and `k/test/accel_test.shard`.
  Rests on decl. Called by add (the match), t0 (`pin_candidates`), loader (`k_kit`).
- `refgen.shard` (665) — `show_refs`: prints each candidate's identity closure as `REF ` lines
  (the generated section of accel_pins), what a row cites above it: the names, the term nodes,
  the rows, the table's buckets and the table, the closures. Generation only. Rests on add, host. Called by t0 under
  `--pins`.
- Six of these files (`tc`, `add`, `inductive`, `nested`, `axioms`, `refgen`) hold the proofs that their counters'
  recursion ends, beside the functions (`fulfills`), over the facts of `../descent.shard`, which
  reaches the directory through `../intmap.shard`. They are no part of K's program: a build
  passes them over, and `toolchain_check.sh` checks them.
- `test/tc_test.shard` (101 cases), `test/inductive_test.shard` (29), `test/nested_test.shard`
  (11), `test/add_test.shard` (321, with the one-counter census), `test/accel_test.shard` (98) —
  tests inside the seal; run by `v3/test.sh` on the bootstrap and by `k_clients_test.sh` through
  the V3 loader.

#### The vocabulary K is written over (`v3/kernel/`, public)

- `expr.shard` (1,256) — `Expr` (11 node kinds, each with node data `D`: id, hash, loose-bvar
  range, three flags), `Literal`, `BinderInfo`, `Memo`, `St` (fvar counter, heartbeats, node
  counter, memo); `mk_*` (anonymous) and `new_*` (identified) constructors; `ingest_expr` (the
  raw entry's rebuild); `Expr.size`, a term's derived size, by which `expr_eq` and
  `expr_eq_core`, which call each other, are measured, their descent proven beside them;
  the application spine (`get_app_fn`, `get_app_args`) with the facts a walk by the spine
  descends by — the head and the arguments together weigh what the term does
  (`exprs_weight`, `get_app_fn_lt`, `get_app_args_lt`, `app_arg_lt`);
  `same_node`, `used_consts`/`expr_consts`; instantiate, abstract,
  lift/lower, level instantiation, beta; `show_expr`. The header comments carry the typing,
  reduction and definitional-equality rules. Rests on level, intmap, name. Used by everything
  that builds or reads terms.
- `level.shard` (653) — `Level` (zero succ max imax param, and the elaborator's `LMVar`),
  `normalize`, `is_equivalent`, `is_geq`, `undef_param`; every function a definition, `normalize`
  and the order it sorts by through two facts about their helpers (`to_offset_le`,
  `push_max_args_weight`). Rests on util. Used by expr, tc, elab.
- `decl.shard` (186) — `Declaration` (raw) and `ConstantInfo` (checked entries), hints, safety,
  recursor rules; the admission rules as comments. `verdict.shard` (66) — `Outcome`, `Reason`,
  `Resource`, `Verdict`, `Failure`, `KRes`. `name.shard` (178) — `Name`, `hash_name`, `name_key`,
  `mix`, `hash_under`. Also `intmap` (96), `util` (170), `json` (217), `host` (21); and `descent` (43), theorems that `util` and `intmap` cite.
- `kw.shard` (187) — the walk over K from outside the seal: `W` = environment + local context +
  state; `w_infer`, `w_whnf`, `w_is_def_eq` (and `_r` forms that keep K's failure), `w_local`,
  `w_pis`/`w_lams` (K's `mk_binding`), `w_admit` (K's `check` into the walk's environment),
  `reason_of`. Rests on the view. Called by elab, unify, tactic, arith, define, derive, erase,
  erasure, realize, matcher, reader, loader.

#### The Init side

- `t0.shard` (76) — the replay driver: `[-v] [-a] [-p] [--pins] CHUNK…`; reads each chunk,
  `process_lines`, prints non-ACCEPT verdicts and counts; `-a` axiom closures, `-p` the pins
  line, `--pins` the reference table; exit = rejected + exhausted + mismatched. Rests on the
  view, host. Called by `t0_full.sh`, `gen_pins.sh`, t0_fixture_test, route2_test, native_tie.
- `index.shard` (966) — the Init index: header, record table (62-byte rows), sorted name table
  (218-byte rows), declaration table (9-byte rows); `ix_build` (one pass), range readers
  (`ix_read_row`, `ix_lookup`, `ix_names_under`, `ix_decl_of`), `ix_verify`. Rests on host, json,
  name, util. Called by initindex, initload.
- `initindex.shard` (60) — driver: build an index, or `--verify` one. Called by
  `kernel/test/init_index.sh` (keeps `v3/.cache/init.index` current) and index_test.
- `initload.shard` (854) — the load on demand: `od_open` (meta line against the pin, index
  header, export size), `od_lookup` (name → ordinal), `od_walk` (the closure of the demanded
  records by range reads), `od_feed` (lines through K's `process_record` on `run_on env`),
  `od_load`. Rests on index, the view, json, host. Called by loader (`init_through`,
  `init_demand`, `init_load`).

#### Scripts and fixed files

- `v3/export.sh` (61) — builds `init.ndjson` (lean4export `15f6055`, toolchain v4.33.1) and
  `init.axioms` under `$V3_EXPORT_DIR` (default `.shard-cache/v3-export`, untracked); writes the
  pin marker `init.ndjson.pin`.
- `v3/axioms.lean` (79) — the oracle: Lean computes each `Init` constant's axiom closure as a
  least fixpoint; one `AX name: axioms` line each.
- `v3/t0_full.sh` (102) — the replay gate. `v3/t0_expected.txt` (1) — the pinned verdict line.
  `v3/t0_axioms_cmp.sh` (26) — K's `AX` lines against the oracle's.
- `v3/kernel/gen_pins.sh` (25) — regenerates accel_pins.shard below its marker.
- Tests: `state_test.sh` (the state discipline + `v3/pins/state/main.shard`, 28 controls),
  `t0_gate_test.sh` (the gate on stub engines, 5 scenarios), `t0_fixture_test.sh` (3,000 lines,
  closures vs oracle, one altered record a `MISMATCH`), `route2_test.sh` (K under `ev` vs K direct, byte-identical),
  `k_clients_test.sh` (K's tests through the V3 loader), `index_test.sh` (build, verify, four
  tampered inputs), `init_index.sh` (not a test: keeps the index current).

### Running, the back end, the build

#### Kernel sources

| file | lines | what it does | rests on | called by |
|---|---|---|---|---|
| `prog.shard` | 489 | Programs as data: `EType`, `ELit`, `EPat`, `ETerm`, `ERec`, `EDecl`, `Prog`, `Val`, `EvRes`; the primitive table (36 entries, code and arity by identity); the identities of the runtime's cells. | `name`, `util`, `intmap` | the classifier and loader (build a `Prog`), `ev`, `lower`, `cgen`, `emit`, `dump`, `run` |
| `ev.shard` | 1,870 | `ev`, the definition of "run": links a `Prog` into a private form (constructors as tags, functions as indices) and runs it as a machine with an explicit continuation, fuel spent per function entry. Also `prepare`/`invoke`/`invoke_run` (the handle and its two doors), the argument validation, the text codec for command-line arguments, `run_prog`, the extern wire by declared type (six host calls), the effect trace. | `host`, `prog`, `intmap`, `util` | `run.shard` (the door on a load), `load.shard` (through `run.shard`), `reader.shard`/`loader.shard` (a tactic library's run), tests `ev_test`, `prims_test`, `entry_test`, `handle_test`, `rows_test`, `project_test`, `loader_test`, `calc_harness` |
| `run.shard` | 183 | A load's program run from an entry, the command line's `--run`: `run_load`/`run_load_as` prepare the entry at `ev`'s checked door with effects, refuse it `preconditioned` for what the load's environment shows of it that the program does not state (an evidence binder, a proposition field of a parameter's constructor), read the arguments by the text codec and invoke once. | `loader`, `ev`, K's view (`kernel.k`), `kw`, `etable`, `prog` | `load.shard` (`--run`, `--lowered`), `test/entry_test.shard` |
| `lower.shard` | 951 | Back end stage 1, `lw_prog : Prog → Prog`: to a named form, the seven expression rows for Init's primitives whose runtime operation differs (`Nat.sub div mod`, `Int.tdiv tmod ediv emod`), lifting of `if`/`match` in operand position, nested patterns to flat chains, every operand an atom, back to indices. `lw_role` gives the eight wire roles. Refuses unlinked `sig`s, primitives with no runtime operation, string patterns. | `prog`, `name`, `util` | `emit.shard`, `cgen.shard` (`lw_role`), `load.shard` (`--lowered`) |
| `cgen.shard` | 853 | Back end stage 2, `cg_prog`: the restricted form transcribed to C over `rt.h` — name mangling, tags (900–908 by role, others from 1000, odd for a type's second constructor), per-function text, the files (`prog.h` with what every unit reads, `main.c` with the tables and `main`, parts of 400 functions), each `.c` a translation unit of its own. Refuses anything outside the restricted form, a numeral pattern past the small integers, an extern with no shim, a missing `main` or one with parameters before the World, a constructor of more than 125 fields, a constructor named as a cell of the runtime at another place in its type. | `lower`, `prog`, `intmap`, `name`, `util` | `emit.shard` |
| `emit.shard` | 105 | The back end's driver `emit_prog`: lowering, generation, the `map` lines (C name, kind, identity, source file), the module count. | `prog`, `etable`, `lower`, `cgen` | `load.shard` (`--emit`) |
| `rt.h` | 966 (C) | The runtime under the generated C, in two halves. Every unit's: tagged small integers, cells, the primitives' inline paths, declarations of the rest. `main.c`'s alone (`RT_MAIN`), once in a program: the allocator, the conservative mark–sweep collector, bignum arithmetic, seven extern shims, traps, `SHARD_STATS` counters, `rt_init`. | libc, GNU C extensions, Linux `mmap`/`setrlimit` | generated C (`cc … -I v3/kernel`: `v3/build.sh`) |
| `host.shard` | 21 | The toolchain's World type and seven extern declarations, text as `(List Int)` (`get_args read_file read_range write write_line write_file exit`). | — | every toolchain entrypoint; `ev.shard` (its driver performs six of them through these) |
| `load.shard` | 380 | The loader's command line. In this area: `--run MODULE.FN … -- ARG…` with `--fuel`, `--trace`, `--lowered` (`main_run`, the exit codes 2–6), `--emit DIR` (`main_emit`: writes the C parts, `map`, `sources`; the `EMIT:` line), `--dump` (`main_dump`), `--facts OUT` (`write_facts`). | `loader`, `ev`, `dump`, `facts`, `emit`, `lower`, `run` | `eval direct v3/kernel/load.shard …`, `v3/bin/load …`; `v3/build.sh` (the emit); most shell tests |
| `dump.shard` | 290 | A `Prog` as the canonical text that the bootstrap's `eval dump` also prints: one line per declaration, sorted; a stated projection (last name components). | `prog`, `etable` | `load.shard --dump`; `test/parity_test.sh`, `test/plain_test.sh`, `test/template_test.sh` |
| `facts.shard` | 253 | What the program-first reading asks K about Init's constants, as a table (`facts_lines`): for each constant a token of the closure's files can resolve to — with an inductive's constructors, a constructor's inductive, the types its fields are at and a projection function's structure — its kind, its binders' flags (written or not; runtime data, a type, a proof), an inductive's parameters, constructors and E view, a projection function's structure and field, whether its type ends in Prop. The driver prints the load's table joined with that of each implementation it linked, whose fork fetched what the main line did not (`facts_join`). The Rust bootstrap reads what a closure cites of Init by it. | `classify`, `erase`, `dump` | `load.shard --facts OUT`; `test/plain_test.sh` |
| `base.shard` | 108 | The base every file sees beside Init: the text of the module `shard.int` (`land`, `lor`, `xor` on `Int`), the test for its declarations and for a file that uses its operators. | `util`, `sym`, `name`, `sexpr` | `elab`, `prog`, `erasure`, `loader`, `load` |
| `intmap.shard` | 96 | A persistent map keyed by non-negative integers (a 4-ary trie). | `descent` | `ev` (function and tag tables), `cgen` (tags), K and the loader |

The other three roots of the build's inventory are drivers of other areas: `t0.shard` (K over the
export's chunks), `prove.shard` (the producers), `initindex.shard` (the Init index).

#### The Rust bootstrap (`rust_bootstrap/src/`, 6,047 lines; no README)

| file | lines | what it does |
|---|---|---|
| `bin/eval.rs` | 287 | The `eval` executable: `direct [--root DIR] [--facts F] FILE ARGS…` and `dump [--root DIR] [--facts F] FILE…` (DIR is `v3` where none is named); runs on a thread with a 4 GiB stack (`SHARD_STACK_MIB`); the host handler for seven extern names, answering an extern declared over Init's types in Init's cells and any other in the runtime's. |
| `plain/mod.rs` | 1008 | The reader's data and tables: programs as `prog.shard` has them, names as identities, a file's scope (`use`, its own types, the visible modules), the E table and its resolution (`etable.shard`), resolution over K's constants (`scope.shard`), the closure's files as modules (`loader.shard`: paths, imports, views, the seal's prefix), every view linked to its implementation (`link_views`), Init's inductives registered on citation. |
| `plain/read.rs` | 1,446 | `classify.shard`'s reading, function by function: types, terms, patterns, the written arguments that run, decisions, rows, numeral rows, `let` chains, signatures and bodies. It refuses nothing; a form with no reading is recorded and left out (a template's call, a derivation that writes functions, a derived view). A derivation of `size` alone writes definitions with no program: it is passed over. |
| `plain/settle.rs` | 410 | `classify.shard`'s `c_type`, `settle_ops`, `expand_rows`: static types, the operators by their operands' types, the rows written out. |
| `plain/record.rs` | 246 | The host parser's entry and `record` expansion (`record.shard`'s twin). |
| `plain/facts.rs` | 216 | The table `load.shard --facts` prints, parsed: what the reader has in K's place. |
| `plain/out.rs` | 279 | The canonical text for a closure read (`dump.shard`'s format), the evaluator's module — the runtime's cells under the runtime's names —, the entry of a run. |
| `eval.rs` | 1,139 | The evaluator: an environment machine over one value stack, reference-counted values; `if` by the constructor's position; `SHARD_PROF` call counts. |
| `prim.rs` | 182 | The primitives over arbitrary-precision integers: the eighteen bare-named ones and Init's twenty-one (`Int.tdiv` … `Nat.decLe`). |
| `ast.rs` | 81 | The AST. |
| `lib.rs` | 755 | The crate root, the allocator (mimalloc), 28 unit tests. |

#### Scripts

| script | what it does | called by |
|---|---|---|
| `v3/build.sh` | the emit — on the bootstrap or by the loader's binary —, then `cc` on the text's units side by side and the link; the inventory, the stamp, the shim `v3/bin/eval`; the emit tie (`--tie`) | CI, `v3/fmt.sh`, the shim (`--stamp`), `native_tie_test.sh`, `bin_test.sh`, `build_test.sh`, `shardfmt_test.sh` |
| `v3/test.sh` | every entrypoint on one engine, in parallel; keeps outputs for the suite tie | CI (`v3`, `native`), authors |
| `v3/fmt.sh` | lists or rewrites sources that are not canonical; builds `v3/bin/shardfmt` when stale | authors; `shardfmt_test.sh` (`--list`) |
| `v3/t0_full.sh` | the engines' tie on 100,000 lines, then the full Init replay against its pins | CI (`v3`), `test/t0_gate_test.sh` (with stub engines) |
| `v3/t0_axioms_cmp.sh` | K's axiom closures against Lean's oracle file | `v3/t0_full.sh`, `test/t0_fixture_test.sh` |
| `v3/export.sh` | installs Lean (elan), builds lean4export, exports `Init` and the axiom oracle into `.shard-cache/v3-export` | CI, authors (once) |
| `v3/kernel/test/init_index.sh` | keeps `v3/.cache/init.index` current and prints `EXPORT INDEX`; `--current` never builds | `v3/test.sh`, `v3/build.sh`, most shell tests |
| `v3/kernel/test/suite_tie.sh` | compares two kept suite runs | CI (`suite_tie`), `native_tie_test.sh` (its self-test) |
| `bin/ci_docs_only.sh` | exit 0 when a push changed only `.md` files; `--self-test` | CI (every gate job's first line; `host` runs the self-test) |
| `.gitlab-ci.yml` | the five jobs `host`, `v3`, `replay`, `native`, `suite_tie` | GitLab |

## K


K is the checker of L. It takes a checked environment and one declaration and answers. It has no
elaboration, unification, tactics or global state, and reads no file. The data it is written over
is public and outside its directory: `name`, `level`, `expr` (terms, node data, the state), `decl`,
`verdict`.

| file (`v3/kernel/k/`) | lines | holds |
|---|---|---|
| `env.shard` | 39 | `CheckedEnv`: constants by name hash, whether `Quot` is declared, the pinned names, the node-id watermark |
| `tc.shard` | 2,761 | type inference, weak-head normal form, definitional equality; memo tables; budgets; literal rules; the `Nat` accelerators; the descent of its 60 functions |
| `inductive.shard` | 1,366 | inductive blocks: the checks, the constructors, the recursors and their rules; the descent of its walks |
| `nested.shard` | 1,050 | nested inductives through auxiliary mutual types |
| `add.shard` | 519 | `check`: admission per kind, the `Quot` constants, the pin match, the raw entry |
| `axioms.shard` | 277 | axiom closures |
| `import.shard` | 854 | export records → tables → declarations → checks; generated constants against exported ones |
| `accel_pins.shard`, `refgen.shard` | 5,136, 665 | the generated reference rows for the pins; their generator, on no checking path |
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

`mod.req.shard` is the only way in: eight sealed types and 64 signatures.

    (sig type CheckedEnv)
    (sig fn check ((env CheckedEnv) (st St) (d Declaration)) (Outcome CheckedEnv))
                                                    (v3/kernel/k/mod.req.shard)

The signatures are admission (`env_empty`, `check`, `check_with`, `limits`), readers of an
environment (`env_find`, `env_find_at`, `ci_*`, `identity_hash`, `env_pinned`), axiom closures
(`axioms_of`), the record import (`empty_run`, `run_on`, `process_record`, `process_lines`, …)
and the checker as a service (`ctx_new`, `st_for`, `infer`, `whnf`, `is_def_eq`, `is_prop`,
`mk_local`, `mk_binding`). A client cannot build a `CheckedEnv`: `env_empty` is the empty one,
and only `check` and the record import extend it. A client that names the constructor gets
`REFUSE NAME function_expected` (v3/pins/loader/k_client_reach). No signature takes two
environments. The service signatures answer about terms a client built; an answer extends nothing.

The view's importers are `kw.shard` (the walk the elaborator, the tactics, the definer and the
erasure build terms with), `loader.shard`, `scope.shard`, `initload.shard` and `t0.shard`.
`load.shard --root v3 v3/kernel/k` checks the implementation against the view;
`k_clients_test.sh` runs K's tests through the V3 loader, where the seal holds.

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
to records), and every module sees every declaration of the export
(`v3/pins/loader/init/main.shard`).
Before a file's first declaration the loader takes the names the file cites and a fixed kit of
names the toolchain itself builds (`init_kit` in `loader.shard`), walks their dependencies through
the index (`initload.shard`) and feeds those lines to K in export order, into the load's own
environment. Every record is checked; none is admitted on trust. A name the toolchain builds from
a type it meets, outside both, is fetched when a declaration's reading is refused for lack of it,
and the declaration is read again (`settle_missed`). What a file is given does not depend on what
the load fetched for another file. The loader pins are loaded a second time in the reverse order and each
case's records compared with the first (`loader_pins_test.shard`). Only `Init` is exported; no
other Lean library can be imported.

| refusal | when |
|---|---|
| `export_pin_mismatch` | the export's first line names another Lean commit, exporter or format |
| `init_index_stale` | no index, or one built over another byte count |
| `init_unavailable` | the load was given no export: no file loads |
| `bad_import` | `(import Init)`: no line says that a file sees Init; the message says to delete it |
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
its verdict line, fail it); `t0_fixture_test.sh` and `route2_test.sh` replay 3,000 lines, and the
first replays them again with one inductive's exported `isRec` altered: `MISMATCH isRec Subtype`,
`mismatched 1`, exit 1.

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
   (5,845 lines) and the generated table `accel_pins.shard`; the data they are written over,
   `v3/kernel/`: `name level expr decl verdict intmap util`, and `json` for the record
   import. K is reviewed source, written function for function after Lean's kernel. It is not
   proved. `toolchain_check.sh` checks the declared types of its functions, and of the
   directory's own 630 the logic holds 460 as definitions; nothing is stated about them but
   that their recursion ends, and the termination clauses of the other 170 nothing checks.
   Evidence: the replay gate; the hostile battery (`v3/kernel/test/hostile_test.shard`,
   76 cases of declarations that must be refused); the tests in `k/test/`.
   Two beliefs inside this item are easy to miss:
   - *Node identity.* Two terms with the same positive node id and the same hash are taken as
     equal without being compared. This is right only while an id names one node within an
     environment: the state discipline ("The state discipline" above), held by `state_test.sh` to
     the limits stated there.
   - *Accelerated primitives.* K computes fourteen `Nat` operations on literals, types literals
     and expands string literals itself. One believes each computes what the pinned definition
     defines (`k/test/tc_test.shard`: results, large operands, a wrong result for each).
3. **The engine that runs K.** The Rust bootstrap, `rust_bootstrap/src/`
   (5,891 lines, 730 of them its unit tests; the crates `lexpr`, `num-bigint`, `num-traits`,
   `mimalloc`; the Rust compiler): its reader (`plain/`, 3,564 lines) reads K's sources — every
   name to an identity through the file's scope, K's view linked to its implementation — and the
   evaluator runs them and supplies integer arithmetic and the host calls. What the reader reads
   is compared with the V3 loader's program on every toolchain root (`parity_test.sh`). It reads
   them by a table of facts about Init's constants kept in the tree (`v3/kernel/init.facts`,
   39 lines: which arguments of `List.cons` or `Prod.mk` exist at run time, and the like). The
   table is believed; the same test has the loader print it again from K. A checker
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
   an empty `params=` is unconditional only if that code is right.
6. **What the statement says.** K checks the term the elaborator built. Which proposition a
   source statement denotes is decided by `reader.shard` and `elab.shard`; a mistake there gives a
   true theorem about something other than what the author wrote.

### (b) For a compiled binary to behave as its contract says

The contract of a binary is its `bin` form: the requirements are theorems about the entry
function's definition and about observables of the World. Believe (a) for each requirement, and:

| component | files | what is believed | evidence |
|---|---|---|---|
| host-call axioms | the axioms the `bin` trusts | they are consistent together and true of the host | none |
| program = definition | `erase erasure predef realize template`; the registry that runs `Nat` and `Int` as integers, and its rows for the bit-level five (`base.shard`, `prog.shard`) | the program derived from a `fn` computes what its definition denotes | tests; no proof |
| the World used once | `thread.shard` | each World value is consumed at most once, so the axioms' order of effects is the run's | pins `thread_*`; `UNTHREADED` fails the load |
| the roster | `loader.shard` (`check_bin`) | the entry's closure calls no extern outside `externs` | pin `bin_extern_outside` |
| the run, interpreted | `ev.shard` on the bootstrap | it computes the program | the suite |
| the run, compiled | `lower.shard`, `cgen.shard`, `emit.shard`, `rt.h`, `cc -O2`, libc, the OS | the binary computes the program | `lower_test.sh`, `native_tie_test.sh`, `bin_test.sh` |
| host calls | `rust_bootstrap/src/bin/eval.rs`; `rt.h` (`rt_get_args` … `rt_exit`) | each does what its axioms state | none |
| the entry | `ev.shard`, `run.shard` | command-line text becomes values of the entry's types | `entry_test.shard` |

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
| driver | `emit.shard`, `emit_prog` | writes `DIR/main.c`, `DIR/prog.h`, the functions in `DIR/pNNNN.c` (400 a file), `DIR/map` (each declaration: C name, kind, identity, source file) and `DIR/sources` (each module's file and hash); prints `EMIT: DIR entry=main.c modules=N declarations=M` |
| compiler | `cc -O2 -c` on each `.c` of DIR with `-I v3/kernel`, then `cc -O2` over the objects | a translation unit a file, `V3_CC_JOBS` of them compiled side by side (`v3/build.sh`, THE COMPILE); `rt.h` uses GNU C extensions and Linux `mmap` |

**The restricted form.** An atom is a variable, a numeral or a symbol. A right-hand side is an atom
or a constructor, call, primitive or extern over atoms. A body is a right-hand side, a `let` of
one right-hand side, an `if` on an atom, or a `match` on an atom whose arms are flat (a
constructor over variables, a numeral, a symbol, a variable). The lowering gets there by lifting
an `if` or `match` in operand position to a new function over its free variables (named `f.7`: a
numeric component no source name has), compiling a nested pattern to a chain of flat matches, and
binding every operand. The restricted form is still a `Prog`, so `ev` runs it (`--lowered`).

**Primitives.** The fifteen bare-named ones (`= + - < <= band bshr * mod / tmod ediv bor
bxor bshl`) and `Nat.land lor xor shiftLeft shiftRight beq ble` map to
runtime operations. The bit-level five are the engines' side of definitions — Init's
`Int.shiftLeft` and `Int.shiftRight`, the base's `land`, `lor`, `xor` on `Int`
(`base.shard`) — by rows of the registry (`prog.shard` `realization_of`): two's complement
on integers of any size, a shift by any count that is a natural number. `tmod` and `ediv`
are the lowering's own: no source writes them (`renamed_primitive`). `Nat.sub`, `Nat.div`, `Nat.mod`, `Int.tdiv`, `Int.tmod`, `Int.ediv`,
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
| tags | odd for the second constructor of a type, even for every other. 900–908 for the constructors the runtime itself builds and reads (false 900, true 901, nil 902, cons 903, none 904, some 905, pair 906, the World 908) — Init's `Bool`, `List`, `Option` and `Prod`, `Decidable`'s two as `Bool`'s, and a constructor so named in a file's own type; every other constructor two tags from 1000 in program order, of which it carries the odd one where it is its type's second |
| a list; `ByteArray`, `String`, `Array`; a string literal | cons cells ending in nil, one per element; a byte is a small integer in a 32-byte cons cell. A literal is rebuilt each time its code runs |
| a symbol | an ordinary constructor cell, `MkSym` over the list of its bytes (`kernel/sym.shard`): the runtime has no form of its own for it, and equality compares the bytes |
| a closure | none exists: every call is saturated and names a top-level function |
| the World | one cell; the host operations return the cell they were given |

**Memory.** Cells are bump-allocated in a 64 GB reserved region and reclaimed by a conservative,
non-moving mark–sweep collector that runs only inside an allocation. Its roots are the C stack,
the callee-saved registers and the five cells the runtime keeps (the generated text holds no
value outside its functions); a per-slot bitmap of cell starts rejects words
that only look like pointers. Freed cells go to free lists by size. A collection follows every
64 MiB to 2 GiB of fresh allocation, in proportion to what the last one found live.

| limit | what happens |
|---|---|
| a `match` no arm takes; a refuted branch reached | `rt trap: inexhaustive match in F`, exit 3 |
| the bare `/` and `mod` by zero, where a function nothing type-checks leaves them bare; Init's `Nat.div`, `Int.ediv` and their kin are total | `rt trap: div by zero (stuck on the interpreted engines)` and its like, exit 3 (under `ev`: exit 4) |
| `bshl` or `bshr` by a negative count, which only a function nothing type-checks can pass | `rt trap: bshl shift out of range (stuck on engines)`, `bshr` likewise, exit 3. A shift takes a count of any size at `Nat` and at `Int`, as under `ev`: `(bshl 1 64)` is 18446744073709551616, `(bshr -5 64)` is −1 (`v3/pins/emit/int_bits`, `init_prim`) |
| an `if` on a number | `rt trap: if on a value that is no constructor (stuck on the interpreted engines)`, exit 3 |
| an integer of 2^3840 or more | `rt trap: bignum too large (exceeds 2^3840)`, exit 3: a sum, a product, a `Nat.shiftLeft` whose result is that large (any count past 3840 on a nonzero number), where `ev` computes it. The toolchain's own binaries too: `v3/bin/load` ends so on a `--run` that reaches such a number and on a theorem K evaluates past it, where the loader on the bootstrap accepts both |
| the heap past 64 GB | `rt trap: heap exhausted` |
| recursion depth | a call that is not a self tail call uses the C stack; the limit is raised to 2 GB at start, best effort |

A program stuck at a primitive ends differently on each engine: the Rust bootstrap run directly
(`eval direct`) exits 2 with `error running entrypoint: …`; `load --run`, on either engine, exits
4 with `RUN: stuck guard in F`; a binary exits 3 with `rt trap: …`.

**Refused at the emit** (`EMIT: MESSAGE`, exit 2, nothing written; `v3/build.sh` prints
`FAIL: NAME: the emit (exit 2); EMIT: …`):

| message | cause |
|---|---|
| `unlinked sig fn NAME: the implementation's directory was not loaded (give it as a root)` | a view's implementation is not among the roots (also `unlinked sig type`) |
| `the primitive NAME has no operation in a native binary (those that have one: the eighteen bare-named primitives; Nat.sub div mod land lor xor shiftLeft shiftRight beq ble; Int.tdiv tmod ediv emod) in F` | a primitive with no runtime operation |
| `a string pattern has no arm in a native binary (match its cons and nil cells) in F` | a string literal as a pattern; a source file does not get this far, the loader refuses the row first (`pattern_literal`) |
| `a numeral pattern outside the runtime's small integers in F` | a numeral pattern beyond ±2^62; one stands under a constructor, a numeral that is a whole row being read as a test (`v3/pins/emit/refused_numeral_pattern`) |
| `the extern NAME has no shim in the runtime in F` | an extern other than `get_args read_file read_range write write_line write_file exit` |
| `no fn MODULE.main to start from` | the root module has no `main` |
| `MODULE.main takes N parameters: a binary's main takes the World alone (a main with parameters before the World runs under the loader's --run only)` | a `main` with parameters before the World, which `--run` reads from the arguments (`v3/pins/emit/refused_main_params`); `takes no parameter` for a `main` of none |
| `the constructor NAME has N fields: a cell of the runtime holds at most 125` | a constructor of more than 125 fields: the allocator's size classes end at a cell of 1 KiB (`v3/pins/emit/refused_wide_ctor`; `wide_ctor` runs 125). Both evaluators run such a program |
| `the constructor NAME is the World's cell and must not be the second constructor of its type` | a constructor named `World` carries the World's tag, whose low bit is fixed (`v3/pins/emit/refused_world_place`). Both evaluators run such a program |

**What is proven, and what is not.** Nothing about the back end is proven — not the lowering, the
generator, the runtime or the C compiler. A theorem about a function, or a `bin`'s requirement, is
about the program's meaning in the logic, not about the binary. What holds a binary to that
meaning is agreement by test: the lowering against the program under `ev` (`lower_test.sh`), and
the binaries against the Rust evaluator (the ties below). Where a binary's answer is used as a
verdict — the full Init replay runs on compiled K — it rests on those tests.

## The two engines

The toolchain — the loader, the checker K, the provers, the tests — is itself a set of shard
programs under `v3/kernel/`. Two engines run it.

**The Rust bootstrap** (`rust_bootstrap/`, about 6,000 lines; the binary is
`rust_bootstrap/target/release/eval`) is a reader and an evaluator and nothing else, with two
commands. `eval direct [--root DIR] [--facts F] FILE ARGS…` reads FILE and the files it imports
and runs FILE's `main` on `(World 0)` with ARGS as the arguments; the exit code is the program's,
or 2 when the closure does not load or the run fails. `eval dump [--root DIR] [--facts F] FILE…`
prints the closure read as a canonical text and runs nothing; its exit code is 3 when a form was
left unread. DIR is the package root, `v3` where none is named, as for `v3/build.sh`. It performs
the host calls itself and is trusted by review; its 28 unit tests run in CI.

*Its reader* (`plain/`) reads the toolchain's own sources and an ordinary program alike. A file
is a module named by its path under DIR, a name an identity resolved through the file's scope,
and a body is read as the loader's program-first reader reads it (`classify.shard`), from the
source and the types its binders state. The Rust side has no K: what that reading asks K about a
constant of Init it takes from the table F, which the loader prints for the closure (`load.shard
--facts F`, `facts.shard`). For the toolchain's own closure the table is a file of the tree,
`v3/kernel/init.facts` — `eval` takes it where no `--facts` is named — and it is believed: it is
how the authority reads K. `parity_test.sh` has the loader print it again from K over every
toolchain root and fails on a line that differs. A directory
import brings the view, and every view loaded is then linked to its implementation, whose `fn`s
and `type`s take the places of the view's `sig`s — the loader links a directory it is given as
a root, and checks it there. The reader refuses nothing — no seal, no view's privacy, no
mismatch between a view and its implementation: the loader is where a closure is refused — and
names on standard error each form it has no reading for, leaving its program out: a call of a
template (so a lifted lambda and an instance), a `derive` that writes functions, a `(realize NAME
(view))`. A `(derive T size)` writes definitions with no program, and the reader passes it over
as it passes a `theorem`.

*The runtime's cells.* A comparison answers `True` or `False`, the symbol bridge and the host
speak `Cons`, `Nil`, `Some`, `None` and `Pair` cells. Those seven are the constructors of
Init's `Bool`, `List`, `Option` and `Prod`, by identity: the reader hands them to the evaluator
under the cells' names, a native binary gives them the cells' tags (`lower.shard`, the wire
roles), and the loader's own evaluator `ev` builds them by those identities and an extern's
values by its declared types (`LANGUAGE.md`, Host calls). Every other constructor goes by its
identity; a native binary also takes the constructor named `World` of a program's own type as
the World's cell, by that name.

`parity_test.sh` compares the reader's text with the loader's over every toolchain root, and
`plain_test.sh` over ordinary closures, where it also runs calc's `main` on what was read.

**The native binaries** (`v3/bin/NAME`) are the same sources compiled by the back end above. To
build one, the loader is run over the root's closure with `--emit`: it loads the closure, lowers
it and writes C, and `cc` compiles that. The loader's own binary is built with the loader
running on the bootstrap; every other root of `v3/build.sh --all` is emitted by that binary,
seconds where the bootstrap takes minutes. A binary the loader's binary emitted rests on that
binary being a right compile of the loader, and `v3/build.sh --tie` is the check: each root's C
text produced again by the other engine and compared byte for byte. CI builds every root on the
bootstrap and runs the tie on every pipeline. A build loads a toolchain root `--unchecked`:
each `fn` is read as a program alone, and the check of the toolchain's sources is a load
without the flag. The binaries are faster and unproven, and must agree with the bootstrap: a
difference fails a gate, one of the two is wrong, and what a program computes is what the
evaluator written in shard computes (`ev.shard`; `LAW.md` rule 21).

`eval direct` is the one spelling for "run this toolchain program". The test scripts call
`$EVAL direct FILE ARGS…`; `EVAL` is the bootstrap by default, or the shim `v3/bin/eval`, which
answers `direct FILE ARGS…` by running `v3/bin/STEM ARGS…` and exits 70 with a message when that
binary is missing or stale. It never falls back to the bootstrap; `dump` always goes to it. The
loader does not detect its engine, and its records do not name one.

| use | engine |
|---|---|
| the suite by default; every emit; the reference side of every tie; `eval dump` | bootstrap |
| the full Init replay (`v3/bin/t0`); the index build when `v3/bin/initindex` is newer than its sources; the formatter (`v3/bin/shardfmt`); the suite's second run | native |

| check of their agreement | compares |
|---|---|
| `parity_test.sh` | the bootstrap's reader and the V3 loader over every toolchain root: the two programs printed as one canonical text, byte for byte. The loader reads each root `--unchecked`, the reading a build compiles and a test runs |
| `toolchain_check.sh` | the same roots loaded checked by the loader's binary: nothing refused, every proof in them accepted, and the program again the bootstrap reader's text; the smallest roots, under 400 declarations (seven), loaded so on the bootstrap too, with the same text |
| `plain_test.sh` | the same over ordinary closures: byte for byte where the reader reads the whole closure, line for line where it does not; calc's `main` run on what it read against the loader's run |
| `route2_test.sh` | K run directly by the engine against K loaded by the V3 loader and run under `ev` (`k_clients_test.sh` runs K's tests the second way) |
| `lower_test.sh` | a program under `ev` as loaded and in its restricted form |
| `native_tie_test.sh` | fixed workloads under the bootstrap and the binaries: stdout, stderr, exit status and every file written, byte for byte, nothing normalized. The workloads are independent and run `V3_TIE_JOBS` at a time (default 8), each under both engines in turn; the log's order is fixed |
| `suite_tie.sh A B` | two whole runs of the suite, one per engine, entrypoint by entrypoint; only counts of seconds, sizes in kB and scratch names are normalized |
| `v3/t0_full.sh` | K's verbose output on the export's first 100,000 lines under both engines, before the full replay |

A difference is a defect of the back end; the fix goes there and the build is rerun.

## Build and gates

### `v3/build.sh`

| command | does |
|---|---|
| `v3/build.sh [SRC [OUT]] [--native] [--root DIR] [EXTRA…]` | builds SRC's closure (default `v3/kernel/t0.shard`) into OUT (default `v3/bin/STEM`); `--root` is the loader's root (default `v3`); EXTRA are further root directories and can only follow OUT. The emit runs on the bootstrap, or under `--native` by `v3/bin/load`, which must be current. Exit 1 on failure |
| `v3/build.sh --all [--bootstrap]` | builds the inventory: the loader on the bootstrap, then the other roots by its binary, `V3_BUILD_JOBS` at a time (default 8); under `--bootstrap` every root on the bootstrap. Keeps a binary whose stamp is current, writes the shim; exit = the roots that failed |
| `v3/build.sh --tie` | after `--all`: every root's C text produced again by the engine that did not build it (`v3/bin/emit/NAME/engine`) and compared with the build's, every file byte for byte; a control at the end, one altered byte that must be reported; exit = the roots not the same |
| `v3/build.sh --list`, `--stamp NAME`, `--shim` | the inventory, `NAME SRC` per line; the stamp `v3/bin/NAME` would carry now (empty: never emitted); writes `v3/bin/eval` |

*The inventory* is 29 roots: the four drivers (`load`, `prove`, `t0`, `initindex`),
`test/calc_harness`, and every `v3/kernel/test/*_test.shard` and `v3/kernel/k/test/*_test.shard`.
The formatter is built on demand by `v3/fmt.sh`. A root of the inventory is loaded
`--unchecked`; any other SRC is loaded checked, as every load is.

*The engine of an emit.* The loader's own root is emitted on the bootstrap and never by a
binary, so no binary builds its successor. A missing or stale `v3/bin/load` fails a build that
names it (`--native`, or a root of `--all` after the loader's); nothing falls back to the
bootstrap. On the dev box `--all` builds the 29 binaries in 267 s at 8 roots at a time: the
loader's emit 137 s and its compile 8 s, then the other 28, whose compiles fill the machine.
The tie of that build, the bootstrap emitting 28 roots again 8 at a time, is 314 s.

*The readings of a toolchain root.* A build compiles the reading of programs alone, and so
does every test that runs or dumps a toolchain root: `--unchecked`. `parity_test.sh` holds
that reading to the bootstrap reader's text (`eval dump`) on every root, under both engines.
The load without the flag is the check of the toolchain's sources, and it runs on the
loader's binary: `kernel/test/toolchain_check.sh` loads every root checked, requires that
nothing is refused and compares the program with the same text. On the bootstrap that load
is the loader interpreted, running the checker over the sources it is written in; the script
runs it there for the seven roots under 400 declarations and compares the two engines
(`LAW.md` rule 21).

*Extra roots.* A root that uses a directory module through its view needs the directory as a
further root, or the emit refuses `unlinked sig fn`. K's directory `v3/kernel/k` is added
when the root is `v3`, no EXTRA is given, SRC lies outside that directory and its closure
consumes K's view (under `--all`, for every such root); others are named, as `v3/fmt.sh` names
the formatter's `print` and `reads` directories.

*The stamp* (`v3/bin/NAME.stamp`) is a hash over the root's closure sources, the `.shard` files of
its extra directories, every `v3/kernel/*.shard` and `v3/kernel/k/*.shard`, `rt.h` and the compiler
flags. Any edit of a kernel source makes every binary stale. It does not cover the Rust evaluator
or the C compiler. A build hashes every file that can enter its stamp before the emit reads any,
and compares the stamp's files with those hashes after the compile: when one was edited, created
or removed while the build ran, the build fails with `FAIL: NAME: changed during the build:
FILE…` and leaves no stamp beside the binary. A file outside the stamp, or one touched with its
content unchanged, fails nothing (`build_test.sh`).

*What a build writes.* By default everything is under `v3/bin/` (untracked): `NAME`,
`NAME.stamp`, `NAME.build.log`, `emit/NAME/` (`main.c`, `prog.h`, the parts, `map`, `sources`,
`root`, `extra`, `engine`). The objects of the compile are scratch and removed after the link. NAME is OUT's last component. With an OUT elsewhere the binary and `OUT.stamp` stand
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
| `entry_test.shard` | command-line arguments validated against an entry's parameters before it runs; an entry with an evidence binder or a proposition field refused by a load's run |
| `ev_test.shard` | `ev` on hand-built programs: each rule, guards, fuel, externs, deep recursion |
| `expr_test.shard` | K's term utilities and a node's hash, range and flags |
| `handle_test.shard` | `prepare` and `invoke`: refusals by name, the two doors, one handle invoked many times |
| `hostile_test.shard` | declarations K must refuse, each for its stated reason, through K's view |
| `json_test.shard` | the JSON reader |
| `k_client_test.shard` | a client of K's view builds a raw declaration, checks it, reads the result |
| `level_test.shard` | universe levels |
| `loader_pins_test.shard` | every `v3/pins/loader/CASE` loaded and compared with its `;; expect:` line |
| `loader_test.shard` | the loader beyond the pins: records, visibility, a loaded program run, a load with no export |
| `matcher_test.shard` | a generated matcher is recognized from its type; an imposter is not |
| `prims_test.shard` | every primitive under `ev`: a positive, a negative and a boundary case |
| `prog_test.shard` | a `Prog` built by hand and looked up |
| `project_test.shard` | definitions erased to programs and run: decisions as conditions, a matcher |
| `reader_pins_test.shard` | every `v3/pins/reader/*.shard` read and compared with its `;; expect:` line |
| `reader_test.shard` | the reader: every form, scope resolution, the refusals with their reasons |
| `rows_test.shard` | `String`, `ByteArray`, `UInt8`, `Fin` and structure projections as run-time values |
| `sexpr_test.shard` | the s-expression reader and every one of its errors |
| `unify_test.shard` | the unifier's outcomes: exhausted, blocked, assigned, refused |
| `k/test/accel_test.shard` | the accelerated primitives' reference matcher, inside K's directory |
| `k/test/add_test.shard` | admission to the environment, quotients, the one node counter |
| `k/test/inductive_test.shard` | inductive types admitted, their recursors generated |
| `k/test/nested_test.shard` | nested inductives |
| `k/test/tc_test.shard` | the type checker on a hand-built environment |
| `bin_test.sh` | calc's `bin`: record, requirements, trace, native binary, two negatives |
| `build_test.sh` | `v3/build.sh` on one small program through a stand-in C compiler: the first error of a failed compile shown, a source edited during the build refused a stamp |
| `calc_main_test.sh` | calc's `main` under `--run` prints the model's output, byte for byte |
| `calc_test.sh` | calc's functions under `ev` against recorded reference answers |
| `define_test.sh` | calc's functions get definitions with equations; theorems citing them accepted |
| `derive_test.sh` | `derive`: the library's procedures register; a program over derived equality runs |
| `engine_test.sh` | the proof engine: sidecars replay and regenerate identically; an obligation delegated with `auto` is proved, entered under its name and discharged; `auto` inside an implementation is proved into the sidecar beside the implementation file and replayed; its reach over calc |
| `handle_cost_test.sh` | only `prepare` links; no other file names a handle's internals |
| `imp_test.sh` | the imp machine model loads and its theorems are accepted |
| `index_test.sh` | the Init index built and read back; four tampered inputs refused |
| `k_clients_test.sh` | K's tests loaded by the V3 loader (`--unchecked`) and run under `ev`, so K's view is enforced |
| `loader_pins_list_test.sh` | the cases `loader_pins_test.shard` lists are the directories of `v3/pins/loader/`: none unlisted, none missing, none twice |
| `lower_test.sh` | seven programs as loaded and under `--lowered`: same streams, same exit |
| `messages_test.sh` | what a refusal says after its reason: the counts of an arm's names, what to do with an `(import Init)` line, the file a refusal inside an implementation names, the row that makes rows unreachable, what to write for `mutual`, `instance`, a bare `true` |
| `parity_test.sh` | the Rust reader and the loader agree on every toolchain root, and the table of Init's facts in the tree is what the loader prints from K over those roots (above) |
| `plain_test.sh` | the Rust reader reads five closures to the loader's program, byte for byte: calc's, two with records, one with a directory module, and the formatter's sources with their two directory modules (140 declarations; their derivation of a size passed over); on the two readings' fixture every line it prints is the loader's (61 of 105, 20 forms named unread); calc's `main` run on it prints what the loader's run prints |
| `path_test.sh` | one example end to end — load, run — and three breaks that must fail |
| `readings_test.sh` | a `fn`, a lambda it lifts and a supplied `realize` read as programs alone are the programs their elaborated bodies erase to, on one fixture; three controls reported; that reading run; a written proof refused where that reading is the only one; a build's reading over a theorem and a fulfills, passed over |
| `route2_test.sh` | K under `ev` prints what K run directly prints, on 3,000 lines of the export |
| `state_test.sh` | K's sources never advance one version of the checker's state twice |
| `std_test.sh` | every file of `v3/std` loaded in one process, its theorems held by name on Lean's axioms alone |
| `t0_fixture_test.sh` | K on 3,000 lines of the export: all accepted, axiom closures equal the oracle's; one inductive record altered is `MISMATCH`, exit 1 |
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
there is one, or when the formatter refuses a file), `v3/fmt.sh --fix [FILE…]` rewrites them (exit
1 when a file is refused), `v3/fmt.sh --list` names the gated files; exit 2 when the formatter
did not build.
Not gated: `*.auto.shard` and `v3/kernel/k/accel_pins.shard`, which tools write.

**The tree-unchanged check** is the native tie's last step: after its workloads, `git status`
must show no tracked file under `v3/` modified and no new file under `v3/kernel`, `v3/pins`,
`v3/examples`, `v3/std`, `v3/tools` or `v3/models`. Tools and tests write only to untracked
places: `v3/bin/`, `v3/.cache/`, `.shard-cache/` and scratch directories.

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
| `v3` | gate | the export; the Init index; the suite on the bootstrap, 24 entrypoints at a time (outputs kept) |
| `replay` | gate | the export; the Init index; `v3/build.sh`; `v3/t0_full.sh` |
| `native` | gate | the export; the Init index; `v3/build.sh --all --bootstrap`, 28 roots and 28 compiles at a time; `v3/build.sh --tie`; `toolchain_check.sh`, 24 roots at a time; the native tie, 24 workloads at a time; the suite on the binaries, 24 entrypoints at a time (outputs kept) |
| `suite_tie` | tie | `suite_tie.sh` over the two kept runs |

The three gate jobs run side by side on one machine, each held to 32 of its 64 cores by the
runner; the work inside a job is processes of one thread each, so a job's time is its longest
single process — in `v3`, one load of the imp model under the bootstrap.

*Documents-only pushes.* When every file a push changed ends in `.md`, `v3`, `replay`, `native`
and `suite_tie` print `DOCUMENTATION-ONLY PUSH` and end green in seconds
(`bin/ci_docs_only.sh`). Such a green pipeline says nothing about the tree: cite the pipeline of
the push that carried the code.
The whole push is examined, so code pushed with its documents is gated, and any doubt (a new
branch, a manual run) runs the gates. `glab ci run -b main` forces a full run.

*Durations:* the loader's own closure (9,174 declarations) emits in about 128 s on the
bootstrap as a build reads it, and in 10 s by the loader's binary, on the dev box, and its C text — 32 units — compiles in 8 s side by side; the checked load of that closure, the toolchain's check, is 116 s on the binary; the export takes 77 s and its axiom oracle 15 s; the full
replay 420 s on the dev box and 1,260 s on the CI runner, at 29 GB; the index about 1,950 s on
the bootstrap on the CI runner, 14 times the binary's. The runner takes 2 to 3 times the dev
box's time for a step (`DECISIONS.md`, Measured, has a pipeline's minutes). CI's timeouts
are 30 min (`host`), 3 h (`v3`, `replay`, `native`) and 20 min (`suite_tie`).
