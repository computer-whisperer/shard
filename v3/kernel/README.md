# v3/kernel — K: the rule inventory (phase 0) and the checker (phase 1)

**Status: phase 1 closed against the law (T0 holds on routes 1 and 3
with axiom closures; `v3/README.md` for the evidence); phase 2 opened
2026-09-07 — the reader, loader, views, classifier and `ev` are built
to `v3/LANGUAGE.md`, and `prog.shard` is its first file.**
The rule inventory of phase 0 stands as the comments beside the
declarations (user ruling 2026-09-06: declarations, not a prose
restatement, "because the declarations are phase 1's first file anyway
and a prose restatement would drift from them"); the `fn`s beneath them
are the procedure, file for file after the pinned sources. The
representation is annotated nodes with identity (ruled 2026-09-07;
`expr.shard`'s header is the law: every node's first field is its
cached hash, loose-bvar range, flags and id; `St` mints ids and holds
the pin's memo tables; a `CheckedEnv` carries the node-id watermark).
The declarations are the narrow-compatible E profile
(FOUNDATION §9.2): today's `(type …)` forms with the stdlib names the
Rust loader has built in (`Nil Cons True False Some None Z S`), because
the loader reads these files exactly as it reads `kernel/*.shard`. The
naming law (§5.3) governs V3 **source**; the toolchain's own sources are
the bootstrap floor and keep this profile, which the V3 reader carries
as a profile at the flip. Indices and counts are `Int`, nonnegative by
construction and validated at raw admission (§3.5), today's idiom.

**Reconciled against** (the pin, `v3/README.md`): Carneiro, *The Type
Theory of Lean* (2019) for the declarative rules; Lean4Lean
(`8223d22`) for what is and is not proven and the additions since the
thesis; and the pinned kernel sources at `v4.33.1` (`819816b`) —
`src/kernel/{expr.h, level.cpp, declaration.h, type_checker.cpp,
inductive.cpp, inductive.h, quot.cpp, environment.cpp}` — for the
**procedure**, which is what K implements exactly (§3.1). Line numbers
in the comments are into those files at that commit.

## Files

| file | declares | rules carried as comments |
|---|---|---|
| `prelude.shard` | `List Option Bool Pair Nat` — the stdlib copy the loader needs; its identities are the wire's and the decisions' (§6.7) | — |
| `name.shard` | `Name` | hierarchical names; `check_name` (no redeclaration); reserved suffixes; `hash_name`/`name_key` (the environment's and the E tables' key; moved from expr.shard at slice 5 so ev.shard needs no K) |
| `level.shard` | `Level` | `normalize`, `is_equivalent`, `is_geq`, `is_not_zero`, `normalizes_to_zero`; `check_level` (declared parameters only) |
| `expr.shard` | `BinderInfo Literal D Expr Memo St` | the typing rules per constructor (`infer_*`), `whnf_core`/`whnf`, definitional equality (`is_def_eq_core` in its exact order), literal rules, projection rules; the representation (node data, identity, the state); `ingest_expr` — a raw term's node data is a claim, rebuilt at K's raw entry `check` (add.shard; §3.5, GPT-6 R43, 2026-09-12) |
| `decl.shard` | `ReducibilityHints DefinitionSafety ConstantVal RecursorRule QuotKind ConstantInfo InductiveType Constructor Declaration` | admission per declaration kind (`environment::add_*`), the inductive admission checks, recursor generation, the quotient axioms |
| `env.shard` | `Env` (raw and checked), `Outcome` | §3.3's outcomes; raw versus checked (§3.5); the fixed-identity `Nat` accelerators |
| `accel_pins.shard` | `pin_candidates accel_ref accel_ref_closure` | **GENERATED** by `gen_pins.sh` from the pinned export: FOUNDATION §3.2's fixed identities as reference declarations — each accelerated or literal-bearing candidate's identity closure (`add.shard` `ref_edges`), spelled with the anonymous constructors; `add.shard` `pin_if_matches` compares an admitted closure against them structurally (2026-09-12, replacing a 61-bit hash table — GPT-6 R42, records §4.7) |
| `refgen.shard` | — | the generator behind `t0.shard --pins`: prints the reference table as `REF` lines; generation-only, outside K's checking path |
| `sexpr.shard` | `SExpr SRes RAll` | **phase 2, slice 2 (2026-09-12):** the s-expression reader (`v3/LANGUAGE.md` §2) — `read_expr`/`read_all` over UTF-8 bytes; the universe suffix `NAME.{…}` kept on the symbol; string escapes to bytes; `'X` = `(quote X)`; the profile's `-7` as the `profile` flag; `show_sexpr` for the tests and the parity dump (§10) |
| `reader.shard` | `Scope RDecl Cx` | **phase 2, slice 2:** the Stage-0 reader, S forms → K's `Declaration` (§4, §5.1–5.3), term for term; bound names to BVar indices, unbound names resolved through the scope (the module's own declarations, the opened prefixes in order, the bare name; two hits = ambiguous); `.{…}` universe arguments checked against the arity, level 0 filled by rule in E-type positions only; `structure` → the inductive plus abbrev projections over `proj`; `theorem` proofs `(exact TERM)` or `sorry` (pending, admits nothing); reserved forms refused with their phase; `import use trusts` and the E forms routed to their owners; slice 4: `sig type`/`sig fn`/`requirement` read as view parameters (`RDParam`), `read_signature` for a `fn`'s head, `read_fulfills` against a requirement's statement. **Slice 6:** in an E-type position (a `fn`'s binders and result, a `type`'s fields) a citation that resolves to both a type and its same-named constructor takes the type (`resolve_type_pick`; §13 item 28) — v2's `(type World (World Int))` idiom |
| `loader.shard` | `Mod InitSt Rec Load Fx Form ForkPt Im` | **phase 2, slices 3–4 (2026-09-12):** the loader (`v3/LANGUAGE.md` §3, §3.1–3.3, §6.6, §9) — a package root and its files as modules; `import` once, cycles refused, the imported closure visible and nothing opened; `(import Init NAME)` streaming the pinned export through NAME against the pin's meta line, nested prefixes loaded once; `use` and selective `use`; `trusts` as the file's policy over each admitted declaration's axiom closure; `sorry` pending; directory modules: the view (`mod.req.shard`, the dir form) with `sig type`/`sig fn`/`requirement` as view parameters, the req-scope gate, and the implementation `DIR/BASE.shard` checked in a fork from the view's fork point (replay with substitution, `fulfills`, `DISCHARGE` per parameter); the acceptance records (`Rec`). **Slice 5b:** `load_realize` — the equations admitted through the same path as any declaration, one refused and nothing attached; the `REALIZE` record and the `realized` count; a `fulfills` outside an implementation check refused (`fulfills_outside_impl`); each S form scanned for Init's E-eligible inductives before classification. **Slice 6:** a file whose loading failed after its heads were pre-registered is recorded (`Mod`'s loaded flag), so a later import of it reports `import_failed` instead of re-reading it into `duplicate_name` |
| `load.shard` | — | the loader's driver: `eval direct v3/kernel/load.shard --root DIR [--init CHUNK]… [--route N] [--engine S] FILE…`; prints the records and the counts; exit = refused + errors. **Slice 5:** `--run MODULE.FN [--fuel N] FILE… [-- ARG…]` runs the program a clean load yields from FN on the World (ev.shard's `run_prog`), the program's externs performed; a load failure exits 2 with the records, out of fuel 3, stuck 4, a link failure 5. **Slice 6:** `--dump FILE…` prints the program a clean load yields as dump.shard's canonical text |
| `dump.shard` | — | **slice 6 (2026-09-13; `v3/LANGUAGE.md` §10 item 1):** a `Prog` as the canonical text of frontend parity — one line per declaration, sorted bytewise; names by their last component, primitives in full, type variables `?i` by first occurrence, terms at the sequential indices, strings and list sugar as the constructor chains both loaders build, the measure clause outside the text — the same text `rust_bootstrap/src/dump.rs` prints for the bootstrap's Module (`eval dump FILE`); `test/parity_test.sh` compares the two over every toolchain closure |
| `etable.shard` | `EInfo EEnt ESt EHit` | **slice 5b:** the E table split out of the classifier — every E identity by every suffix, pre-registered per file; resolution = the visible hits, in S those among the scope's candidates; an entry registered under `Init` is visible where the Init prefix is |
| `erase.shard` | `W BRole TBind Tele TeleRes` | **slice 5b (`v3/LANGUAGE.md` §7.5):** roles and levels over K's environment — a constant's level parameters solved so its Sort binders are `Type`, its telescope walked with K's locals and each binder classified (type parameter, erased, runtime data with its E type; a Pi over propositions erases to its codomain); Init's inductives E-eligible (no indices, not in Prop, parameters types or propositions, fields E types or propositions; `Nat`/`Int` never) and registered with their constructors on first citation — each S form is scanned for its names before it is classified |
| `realize.shard` | `Tr EV NPat Col Row Leaf Field RzRes …` | **slice 5b (§7.5):** `realize` — the supplied form: the E signature against the L type's erasure, the body as a `fn`'s, descent (a self-call passes a variable bound under the structural parameter) and order (a callee realized earlier, or a sig fn), the leaves of the body's leading matches compiled column by column (a column's variable replaced by the constructor term everywhere, a variable row bound to it), each leaf an equation `NAME.realize_N` — the body translated into L with every constant's telescope filled by first-order matching, a `let` by inference, a non-variable `match` and an `if` as the recursor with a constant motive — proven by `Eq.refl` or the `equations` clause; the derived view: the value's lambdas the binders, `casesOn`/`rec` (no hypothesis used) a `match`, `ite`/`dite` an `if`, a projection a `match`, constructors and realized constants by their roles, `brecOn` and choice refused by name |
| `classify.shard` | `EInfo EEnt ESt EHit PreRes TyRes Rd TRes PRes CRes …` | **phase 2, slice 5 (2026-09-12):** the fragment classifier (`v3/LANGUAGE.md` §6.3, §6.7) — the E table (every identity by every suffix, pre-registered per file before any body; a fn over a sig fn and a type over a sig type override, the fork's substitution); resolution = the visible hits, in S those among the scope's candidates, the profile flat; `fn`/`extern`/`type`/`sig` into prog.shard's data with heads saturated and of one kind, the escape rule, exhaustiveness by the pattern matrix, the private-equality leak by static types; the profile's dialect (auto-bound type variables, `Int`/`Symbol` built in, strings, `-7`, `(quote X)`, `(list …)`) and S's refusals by name. **Slice 5b:** S numerals type as `Nat`; `Nat.succ`/`Int.ofNat` refused with the pointer (`nat_constructor`); `realize` heads pre-registered once the file's directives made their targets visible; `read_fn_rest` shared with the realized body |
| `ev.shard` | `IVal LPat LTerm LArm LFn Wire Linked Kont Halt RunRes` (private) | **phase 2, slice 5:** `ev : Prog → Name → List Val → Int → EvRes` (§6.2) as a machine with an explicit continuation over a linked representation (constructor tags with the second-constructor bit, function indices, operation codes, literals built once) — two tail-recursive steps, fuel on function entry, stuck reasons naming the function, the machine stopping at an extern; `run_prog` performs the host's six externs by short name over the prelude's wire cells and resumes (§6.7). **Slice 5b:** K's Nat set (codes 28–38) total where Lean's are, returning Init's `Bool` and `Decidable` cells, interned with the wire's |
| `prog.shard` | `EType ELit EPat ETerm EArm ERec ECtorDef EDecl Prog Val EvRes` | **phase 2, slice 1 (2026-09-07):** E programs as data — what `ev` runs (`v3/LANGUAGE.md` §6): the bootstrap's AST with resolved identities and classified heads; `ev`'s rule per node; the recursion structure the correspondence is stated over (§4.4) |

## The gate for these files

The toolchain profile is what the Rust loader reads flat: the check is
`./rust_bootstrap/target/release/eval direct <probe.shard>` on an
entrypoint that imports `env.shard` and defines `main` — the closure
must load and run (done 2026-09-06, exit 0). `bin/check` is **not** the
instrument: the old checker resolves imported type names through
`(use (:: <path-derived module> *))` lines, and the package root gives
these files a different module identity by design (LAYOUT.md). `use`
lines are added in phase 2 under the V3 reader's own resolution.

The same flat profile is what the compiler chain's RUN-mode front end
reads, so `v3/build.sh` compiles `t0.shard` to a native binary through
`tools/lower` → `tools/codegen` → cc (FOUNDATION §9.1 route 1, unproven:
the interpreter stays the authority and `v3/t0_full.sh` byte-ties the
two on a prefix before every full-export replay; ruled 2026-09-07). Name
the tools repo-relative from the repo root — #41.

Two more gates since slice 6 (2026-09-13; `v3/LANGUAGE.md` §10),
both run by `v3/test.sh`: **frontend parity** — `test/parity_test.sh`
prints every toolchain closure (the driver, `t0`, the calc harness,
every test) as one canonical text from both readers, `eval dump` and
`load.shard --dump`, and compares them byte for byte (18 closures,
61,080 declaration lines); this is what retired TCB bring-up item 2 —
and **calc's differential** — `test/calc_test.sh` runs the S port
under `ev` (`test/calc_harness.shard`, the Init prefix through `Int`
from `test/fixtures/init_prefix_int.ndjson`) and the old tree's tower
over `examples/calc/calc_differential.shard` on the one input set
`test/fixtures/calc_inputs.txt`, 34 cases per line and 13 folds, and
compares the outputs byte for byte (727 lines a side).

T0's other half (§3.6, "identical verdicts *and axiom closures*"):
`axioms.shard` computes each admitted constant's closure — the relation
of `src/Lean/Util/CollectAxioms.lean` at the pin, as a least fixpoint
(an inductive block is resolved as a unit; the memo lives outside the
CheckedEnv) — and the driver's `-a` prints it (`AX name: …`); the
oracle is `v3/axioms.lean` over Lean's own environment (the same
relation as a Kleene fixpoint, not `collectAxioms`'s order-dependent
memo; records §8), joined by `v3/t0_axioms_cmp.sh`. The fixture test
asserts it on the committed prefix against
`test/fixtures/init_prefix_3000.axioms`; `v3/t0_full.sh` on the whole
export.

## The reconciliation ledger — what the pinned kernel has beyond the thesis

Each item is a rule K implements because the pin does; Lean4Lean's
status is noted where known. None is a departure.

1. **Nested inductives** (`inductive.cpp` `elim_nested_inductive_fn`, `restore_nested`, `check_uniform_ind_occs`): nested occurrences `I (J … T …)` are translated to auxiliary mutual types, admitted, and the recursors translated back; every occurrence must be applied to the declaration's own levels and parameters uniformly. Lean4Lean: modeled, partly proven.
2. **Structure eta** (`try_eta_struct_core`, `to_cnstr_when_structure`): `t ≡ mk t.1 … t.n` for non-recursive single-constructor types; also in recursor reduction on a non-constructor major premise.
3. **Unit-like types** (`is_def_eq_unit_like`): any two terms of a non-recursive structure with zero fields are definitionally equal.
4. **Proof irrelevance** (`is_def_eq_proof_irrel`): two proofs of definitionally equal propositions are equal; `is_prop` is up to level normalization (`imax 1 0`).
5. **Nat literals** (`infer_lit`, `reduce_nat`, `is_def_eq_offset`, `nat_lit_to_constructor`): GMP-backed literals of type `Nat`; `Nat.zero` and literal `0` interchangeable; `succ n` versus literal by offset; binary accelerators on literal arguments only for `Nat.add sub mul pow gcd mod div beq ble land lor xor shiftLeft shiftRight (and the unary Nat.succ)` with **size limits** (`LEAN_NAT_MAX_SIZE`; `Nat.pow`/`shiftLeft` count must fit 32 bits) — an `Exhausted`, not a rule. K binds each to a fixed admitted identity (FOUNDATION §3.2).
6. **String literals** (`string_lit_to_constructor`, `try_string_lit_expansion`): a string literal has type `String` and unfolds to **`String.ofList (List.cons Char (Char.ofNat c₁) (… List.nil Char))`** over the UTF-8-decoded scalar values — at `v4.33.1` the head is `String.ofList`, not `String.mk` (the logical `String` is `ofByteArray`; `v3/INVENTORY.md`). The expansion fires in recursor reduction, projection reduction and definitional equality against a `String.ofList` application. The expansion is enabled only under the pinned identities it names — `String`, `String.ofList`, `List`, `Char`, `Char.ofNat` (§3.2; `string_lit_expansion`, Unsupported `string_literal_unpinned_expansion` otherwise).
7. **K-like reduction** (`to_cnstr_when_K`, `init_K_target`): for a single-constructor, zero-field inductive predicate, the major premise is replaced by the constructor when its type is definitionally the expected one.
8. **Lazy delta with hints** (`lazy_delta_reduction_step`, `ReducibilityHints`): unfold the side with the greater height; `abbrev` first; equal regular heights try argument-wise equality before unfolding; a projection application on one side is unfolded in preference. **Theorems unfold**: `constant_info::has_value` is `is_theorem() || is_definition()` — a theorem's body is delta-reducible in the kernel (FOUNDATION §3.2 said "opaque for unfolding"; corrected 2026-09-06). `opaque` never unfolds.
9. **Native reduction** (`reduce_native`, `Lean.reduceBool`/`Lean.reduceNat`): runs compiled code; sound only under the axiom `Lean.ofReduceBool`, outside the standard profile. K refuses these applications as `Unsupported` and the axiom as outside policy — not a departure, since no standard-profile declaration depends on them.
10. **`eagerReduce`** (`is_eager_reduce`, `m_eager_reduce`): an application argument of the form `eagerReduce _ _` switches the checker to eager `Nat`/native reduction even under free variables. K implements the flag exactly.
11. **Bool-by-reflection shortcut** (`is_def_eq_core`, the `Bool.true` case): when `s` is `Bool.true` and `t` is closed, `t` is fully normalized first (the `decide` path). Order-sensitive; implemented in place.
12. **Projections** (`infer_proj`, `reduce_proj_core`): on single-constructor inductives applied to all parameters and indices; a projection out of a proposition must itself be a proposition; `t.i =?= s.i` tries `t =?= s` under lazy delta before reducing either side (`lazy_delta_proj_reduction`).
13. **Recursor reduction** (`inductive_reduce_rec`): major premise index = `nparams + nmotives + nminors + nindices`; the rule's `rhs` is instantiated with the recursor's levels, applied to params, motives, minors, then the constructor's fields (counted from the end of the major's arguments, so nested-inductive parameter counts need no special case), then the extra arguments.
14. **Elimination level** (`elim_only_at_universe_zero`): a type that can be `Prop` eliminates only into `Prop` unless it is a single-constructor type whose non-`Prop` fields all occur in the result type, or has no constructors; mutual predicates eliminate into `Prop` only.
15. **Universe checks for constructors**: every field's sort is `≤` the inductive's level unless the level normalizes to zero; `is_geq` is on normalized levels.
16. **Positivity and occurrence** (`check_positivity`, `is_valid_ind_app`): strictly positive occurrences only; an occurrence may not appear inside its own indices (lean4#2125); reflexive occurrences (a function returning the type) allowed and flagged `isReflexive`.
17. **Safety** (`DefinitionSafety`): `unsafe` and `partial` declarations may not be used by `safe` ones; mutual definitions are `unsafe`/`partial` only (safe mutual recursion is compiled to recursors/`WellFounded.fix` before the kernel). K's standard profile admits `safe` only; `unsafe`/`partial` are `Unsupported` at raw admission and never exported by default.
18. **Metavariables and free variables** in inputs are refused (`check_no_metavar_no_fvar`); `mdata` is transparent to every rule and is stripped at import (the exporter drops it by default).
19. **Resource limits** (`max_heartbeat`, `max_rec_depth`, `check_system`): every procedure is bounded; K reports `Exhausted(resource, site)` (§3.3, §9.4). Fuel monotonicity (§9.4) holds and is tested (hostile battery 12b/12c): a result reached under a budget is the same under any larger one — exhaustion is never cached and never mutates the environment.
20. **Quotients** (`quot.cpp`): `Quot`, `Quot.mk`, `Quot.lift`, `Quot.ind` are added as `QuotInfo` constants with exactly the types built there, after `Eq` is checked to have the expected shape; `Quot.lift f h (Quot.mk r a) ≡ f a` is the computation rule.

## The procedure (FOUNDATION §3.3)

`check(env, decl, limits)` follows `environment::add` exactly:
`check_name` (no redeclaration, including the reserved `.rec` name),
`check_duplicated_univ_params`, no metavariables or free variables,
the type checks and is a sort; then per kind — axiom: nothing more;
definition/opaque: the value checks and its type is definitionally the
declared type; theorem: additionally the declared type is a
proposition; mutual: header checks, add, then bodies (unsafe/partial
only); inductive: the checks of `decl.shard`; quot: the shape check of
`Eq` and the four constants. Every negative names its subject; every
`kernel_exception` of the pin maps to `Rejected(RuleViolation | Malformed
| Unsupported)` and every resource throw to `Exhausted` — the mapping
table is `env.shard`'s comment and is validated by T0's hostile battery.
