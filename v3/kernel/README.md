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
The declarations are E — the one E of `v3/LANGUAGE.md` §8 (ruled and
built 2026-09-14; FOUNDATION §9.2 as amended): `(type …)`, `fn` and
`extern` forms with explicit `(T Type)` binders and `use` lines like
any file's, over the prelude's `List Option Bool Pair`
(`prelude.shard`, the wire's cells) and the built-in `Int`, `Nat` and
`Symbol`. The Rust loader reads them as E's executor and the V3 reader
as its gate (frontend parity). The naming law (§5.3) governs V3
**source**; these files see no `Init` by nature, so their `type`s over
`Int` and `Symbol` are E only and their `fn`s enter L at the flip.
Indices and counts are `Int`, nonnegative by construction and
validated at raw admission (§3.5), today's idiom.

**Reconciled against** (the pin, `v3/README.md`): Carneiro, *The Type
Theory of Lean* (2019) for the declarative rules; Lean4Lean
(`8223d22`) for what is and is not proven and the additions since the
thesis; and the pinned kernel sources at `v4.33.1` (`819816b`) —
`src/kernel/{expr.h, level.cpp, declaration.h, type_checker.cpp,
inductive.cpp, inductive.h, quot.cpp, environment.cpp}` — for the
**procedure**, which is what K implements exactly (§3.1). Line numbers
in the comments are into those files at that commit.

## Files

**The sealed directory `k/` (slice 3.8, 2026-09-16; `v3/LANGUAGE.md`
§6.6, §13 item 26):** the nine trust files — `env tc add inductive
nested import axioms accel_pins refgen` — live under `kernel/k/`,
private to it behind its view `k/mod.req.shard` (landing 2: seven sig
types, sixty sig fns — the one way in; `k/k.shard` the implementation's
import list; the nine files one module `kernel.k`, §13 item 39; the
internals' tests at `k/test/`); the data vocabulary stays public at
`kernel/`: `name level expr decl intmap json` and `verdict.shard`. A
consumer writes `(import "k")` and `(use kernel.k)`; `load.shard --root
v3 v3/kernel/k` checks the implementation against the view. The rows
below keep their file names; a moved file's path is `k/NAME.shard`.
Landing 3 (2026-09-17): the hostile battery a client of the view
(`test/hostile_test.shard`), its matcher cases `k/test/accel_test.shard`;
the client fixture `test/k_client_test.shard`; `ConstantVal`'s accessors
(`cv_name cv_lparams cv_type cval_eq`) in `decl.shard`.

| file | declares | rules carried as comments |
|---|---|---|
| `prelude.shard` | `List Option Bool Pair Nat` — the stdlib copy the loader needs; its identities are the wire's and the decisions' (§6.7) | — |
| `name.shard` | `Name` | hierarchical names; `check_name` (no redeclaration); reserved suffixes; `hash_name`/`name_key` (the environment's and the E tables' key; moved from expr.shard at slice 5 so ev.shard needs no K) |
| `level.shard` | `Level` | `normalize`, `is_equivalent`, `is_geq`, `is_not_zero`, `normalizes_to_zero`; `check_level` (declared parameters only) |
| `expr.shard` | `BinderInfo Literal D Expr Memo St` | the typing rules per constructor (`infer_*`), `whnf_core`/`whnf`, definitional equality (`is_def_eq_core` in its exact order), literal rules, projection rules; the representation (node data, identity, the state); `ingest_expr` — a raw term's node data is a claim, rebuilt at K's raw entry `check` (add.shard; §3.5, GPT-6 R43, 2026-09-12) |
| `decl.shard` | `ReducibilityHints DefinitionSafety ConstantVal RecursorRule QuotKind ConstantInfo InductiveType Constructor Declaration` | admission per declaration kind (`environment::add_*`), the inductive admission checks, recursor generation, the quotient axioms |
| `verdict.shard` | `Resource Reason (Outcome E) Verdict Failure (KRes A)` | **slice 3.8 (the K seal):** the vocabulary of K's answers, public data beside the sealed checker — `Outcome` and `KRes` take the environment / value as a parameter, so a client matches on them without seeing `CheckedEnv`'s shape; the rules and the mapping of the pin's throws moved here from `env.shard` |
| `k/env.shard` | `CheckedEnv` (`RawEnv` deleted at the seal; `Outcome`, `Reason`, `Resource` moved to `verdict.shard`) | §3.3's outcomes; raw versus checked (§3.5); the fixed-identity `Nat` accelerators |
| `k/accel_pins.shard` | `pin_candidates accel_ref accel_ref_closure` | **GENERATED** by `gen_pins.sh` from the pinned export: FOUNDATION §3.2's fixed identities as reference declarations — each accelerated or literal-bearing candidate's identity closure (`add.shard` `ref_edges`), spelled with the anonymous constructors; `add.shard` `pin_if_matches` compares an admitted closure against them structurally (2026-09-12, replacing a 61-bit hash table — GPT-6 R42, records §4.7) |
| `k/refgen.shard` | — | the generator behind `t0.shard --pins`: prints the reference table as `REF` lines; generation-only, outside K's checking path |
| `sexpr.shard` | `SExpr SRes RAll` | **phase 2, slice 2 (2026-09-12):** the s-expression reader (`v3/LANGUAGE.md` §2) — `read_expr`/`read_all` over UTF-8 bytes; the universe suffix `NAME.{…}` kept on the symbol; string escapes to bytes; `'X` = `(quote X)`; `-7` a numeral in every file (§8.1 rule 2; the `profile` flag deleted at slice 3.4); `show_sexpr` for the tests and the parity dump (§10) |
| `reader.shard` | `Scope RDecl Cx` | **phase 2, slice 2:** the Stage-0 reader, S forms → K's `Declaration` (§4, §5.1–5.3), term for term; bound names to BVar indices, unbound names resolved through the scope (the module's own declarations, the opened prefixes in order, the bare name; two hits = ambiguous); `.{…}` universe arguments checked against the arity, level 0 filled by rule in E-type positions only; `structure` → the inductive plus abbrev projections over `proj`; `theorem` proofs `(exact TERM)` or `sorry` (pending, admits nothing); reserved forms refused with their phase; `import use trusts` and the E forms routed to their owners; slice 4: `sig type`/`sig fn`/`requirement` read as view parameters (`RDParam`), `read_signature` for a `fn`'s head, `read_fulfills` against a requirement's statement. **Slice 6:** in an E-type position (a `fn`'s binders and result, a `type`'s fields) a citation that resolves to both a type and its same-named constructor takes the type (`resolve_type_pick`; §13 item 28) — v2's `(type World (World Int))` idiom |
| `loader.shard` | `Mod InitSt Rec Load Fx Form ForkPt Im` | **phase 2, slices 3–4 (2026-09-12):** the loader (`v3/LANGUAGE.md` §3, §3.1–3.3, §6.6, §9) — a package root and its files as modules; `import` once, cycles refused, the imported closure visible and nothing opened; `(import Init NAME)` streaming the pinned export through NAME against the pin's meta line, nested prefixes loaded once; `use` and selective `use`; `trusts` as the file's policy over each admitted declaration's axiom closure; `sorry` pending; directory modules: the view (`mod.req.shard`, the dir form) with `sig type`/`sig fn`/`requirement` as view parameters, the req-scope gate, and the implementation `DIR/BASE.shard` checked in a fork from the view's fork point (replay with substitution, `fulfills`, `DISCHARGE` per parameter); the acceptance records (`Rec`). **Slice 5b:** `load_realize` — the equations admitted through the same path as any declaration, one refused and nothing attached; the `REALIZE` record and the `realized` count; a `fulfills` outside an implementation check refused (`fulfills_outside_impl`); each S form scanned for Init's E-eligible inductives before classification. **Slice 6:** a file whose loading failed after its heads were pre-registered is recorded (`Mod`'s loaded flag), so a later import of it reports `import_failed` instead of re-reading it into `duplicate_name`. **Slice 10:** `Load` carries each realization's pending roots — a pending measure recorded once (`PENDING`) and carried by every realization whose body reaches it (`REALIZE … pending=`, `realize_roots`; R58). **Slice 3.4 (the one E):** the profile flag, `profile_form` and the layout rule gone; a `type` over E's built-ins alone is E only (`type_is_e_only`, §8.1 rule 1), every `type` opens its namespace at pre-registration (`open_types`, §13 item 7). **Slice 3.7:** the `UNUSED` record — a file's `(use P)` lines no symbol token names a declaration under, judged against K and the E table once the file is loaded (`unused_uses`; the E table's suffix index, the native K declarations bucketed by last component, the K scan as the fallback); a lint the prune tool acts on, never a refusal |
| `load.shard` | — | the loader's driver: `eval direct v3/kernel/load.shard --root DIR [--init CHUNK]… [--route N] [--engine S] FILE…`; prints the records and the counts; exit = refused + errors. **Slice 5:** `--run MODULE.FN [--fuel N] FILE… [-- ARG…]` runs the program a clean load yields from FN on the World (ev.shard's `run_prog`), the program's externs performed; a load failure exits 2 with the records, out of fuel 3, stuck 4, a link failure 5. **Slice 6:** `--dump FILE…` prints the program a clean load yields as dump.shard's canonical text; **slice 7:** `--run MODULE.FN FILE… -- ARG…` validates ARG… against the entry's parameters before the World (§9) and refuses a malformed one as `RUN: argument N REASON: TEXT`, exit 6 |
| `dump.shard` | — | **slice 6 (2026-09-13; `v3/LANGUAGE.md` §10 item 1):** a `Prog` as the canonical text of frontend parity — one line per declaration, sorted bytewise; names by their last component, primitives in full, type variables `?i` by first occurrence, terms at the sequential indices, strings and list sugar as the constructor chains both loaders build, the measure clause outside the text — the same text `rust_bootstrap/src/dump.rs` prints for the bootstrap's Module (`eval dump FILE`); `test/parity_test.sh` compares the two over every toolchain closure |
| `etable.shard` | `EInfo EEnt ESt EHit` | **slice 5b:** the E table split out of the classifier — every E identity by every suffix, pre-registered per file; resolution = the visible hits among the scope's candidates, for every file (slice 3.4), decided structurally on the hit's identity (`cited_in_scope`, `name_above`; slice 3.6); an entry registered under `Init` is visible where the Init prefix is |
| `erase.shard` | `W BRole TBind Tele TeleRes` | **slice 5b (`v3/LANGUAGE.md` §7.5):** roles and levels over K's environment — a constant's level parameters solved so its Sort binders are `Type`, its telescope walked with K's locals and each binder classified (type parameter, erased, runtime data with its E type; a Pi over propositions erases to its codomain); Init's inductives E-eligible (no indices, not in Prop, parameters types or propositions, fields E types or propositions; `Nat`/`Int` never) and registered with their constructors on first citation — each S form is scanned for its names before it is classified |
| `realize.shard` | `Tr EV NPat Col Row Leaf Field RzRes …` | **slice 5b (§7.5):** `realize` — the supplied form: the E signature against the L type's erasure, the body as a `fn`'s, descent (a self-call passes a variable bound under the structural parameter) and order (a callee realized earlier, or a sig fn), the leaves of the body's leading matches compiled column by column (a column's variable replaced by the constructor term everywhere, a variable row bound to it), each leaf an equation `NAME.realize_N` — the body translated into L with every constant's telescope filled by first-order matching, a `let` by inference, a non-variable `match` and an `if` as the recursor with a constant motive — proven by `Eq.refl` or the `equations` clause; the derived view: the value's lambdas the binders, `casesOn`/`rec` (no hypothesis used) a `match`, `ite`/`dite` an `if`, a projection a `match`, constructors and realized constants by their roles, `brecOn` and choice refused by name |
| `classify.shard` | `EInfo EEnt ESt EHit PreRes TyRes Rd TRes PRes CRes …` | **phase 2, slice 5 (2026-09-12):** the fragment classifier (`v3/LANGUAGE.md` §6.3, §6.7) — the E table (every identity by every suffix, pre-registered per file before any body; a fn over a sig fn and a type over a sig type override, the fork's substitution); resolution = the visible hits, in S those among the scope's candidates, the profile flat; `fn`/`extern`/`type`/`sig` into prog.shard's data with heads saturated and of one kind, the escape rule, exhaustiveness by the pattern matrix, the private-equality leak by static types; the profile's dialect (auto-bound type variables, `Int`/`Symbol` built in, strings, `-7`, `(quote X)`, `(list …)`) and S's refusals by name. **Slice 5b:** S numerals type as `Nat`; `Nat.succ`/`Int.ofNat` refused with the pointer (`nat_constructor`); `realize` heads pre-registered once the file's directives made their targets visible; `read_fn_rest` shared with the realized body. **Slice 10 (R56):** an `if`'s condition is typed where the declarations fix it — a `sig type` is `private_if`, an inductive of other than two constructors or `Int`/`Nat`/`Symbol` is `if_type` (`if_bad`). **Slice 3.4 (the one E):** the `profile` flag gone — one scope rule, `(T Type)` binders, the literal rules for every file (`"…"` and `(list …)` over the `List` in scope, `list_in_scope`), `type_ident` = the built-ins, then K's constants, then the E-only types |
| `ev.shard` | `IVal LPat LTerm LArm LFn Wire Linked Kont Halt RunRes` (private) | **phase 2, slice 5:** `ev : Prog → Name → List Val → Int → EvRes` (§6.2) as a machine with an explicit continuation over a linked representation (constructor tags with the second-constructor bit, function indices, operation codes, literals built once) — two tail-recursive steps, fuel on function entry, stuck reasons naming the function, the machine stopping at an extern; `run_prog` performs the host's six externs by short name over the prelude's wire cells and resumes (§6.7). **Slice 5b:** K's Nat set (codes 28–38) total where Lean's are, returning Init's `Bool` and `Decidable` cells, interned with the wire's; **slice 7:** `run_prog`'s checked entry (§9) — the parameters before the World validated from the driver's arguments (`Int`, `Nat`, a two-constructor list type), `RunArg` by position and text; `Nat.pow x 0` and `sym_of_chars` on invalid UTF-8 no longer reach the host's fatal refusal, `Nat.sub` under the non-negative guard; **slice 9 (R51):** a primitive's outcome is `PrimRes` — a value, the guard failed, a resource exhausted (`nat_size`, `nat_count`) — decided before the work in K's order, `HExhausted`/`EvExhausted`/`RunExhausted` through to the driver's exit 3; **slice 10 (R59):** the entry's byte-list codecs by identity and element type (`list_ctors`: the prelude's `(List Int)`, Init's `(List Nat)` and `(List Int)`), never by constructor shape; **slice 3.4:** a string literal links as the cells of its own `List` (`LStr`'s constructors, interned with the program's) |
| `prog.shard` | `EType ELit EPat ETerm EArm ERec ECtorDef EDecl Prog Val EvRes` | **phase 2, slice 1 (2026-09-07):** E programs as data — what `ev` runs (`v3/LANGUAGE.md` §6): the bootstrap's AST with resolved identities and classified heads; `ev`'s rule per node; the recursion structure the correspondence is stated over (§4.4); **slice 3.4:** `LStr` names the `List` in scope's nil and cons beside its bytes, the literal's kind is its sign |

## The gate for these files

These files are what the Rust loader reads (flat resolution, `use`
ignored, dotted citations canonicalized — `LANGUAGE.md` §8.1 rule 8):
the check is `./rust_bootstrap/target/release/eval direct <probe.shard>` on an
entrypoint that imports `env.shard` and defines `main` — the closure
must load and run (done 2026-09-06, exit 0). `bin/check` is **not** the
instrument: the old checker resolves imported type names through
`(use (:: <path-derived module> *))` lines, and the package root gives
these files a different module identity by design (LAYOUT.md). The
`use` lines arrived with phase 3's slice 3.3 (2026-09-14) under the V3
reader's own resolution.

The same flat reading is what the compiler chain's RUN-mode front end
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
61,080 declaration lines at slice 6; 20 closures, 67,976 lines since
slice 7); this is what retired TCB bring-up item 2 —
and **calc's differential** — `test/calc_test.sh` runs the S port
under `ev` (`test/calc_harness.shard`, the Init prefix through `Int`
from `test/fixtures/init_prefix_int.ndjson`) and the old tree's tower
over `examples/calc/calc_differential.shard` on the one input set
`test/fixtures/calc_inputs.txt`, 34 cases per line and 13 folds, and
compares the outputs byte for byte (727 lines a side).

Two more since slice 7 (2026-09-13), both `v3/test.sh` entrypoints:
**the primitive suite** — `test/prims_test.shard` runs every entry of
`prog.shard`'s table under `ev` with a positive, a negative and a
boundary case fixed by hand (39 entries, 120 cases; the guards, the
totalizations, the cells, the symbol round trip, arithmetic past 64
bits) — and **the checked entry** — `test/entry_test.shard` runs
`run_prog` on the `entry` and `entry_s` pins: the driver's arguments
validated against the entry's parameters before the World, a malformed
one refused by position and text (`v3/LANGUAGE.md` §9). The loader
tests share `test/loader_kit.shard` (a load and its records, the
program it yields, predicates over the records).

Since slice 9 (2026-09-13; GPT-6's checkpoint memo, records §4.8):
the parity harness checks each closure's projection injective before
comparing (one declaration per short name among the heads, the types
and the constructors; R53); `test/t0_gate_test.sh` drives
`v3/t0_full.sh` with stub engines and requires the byte-tie to fail
when either engine fails or a log lacks its verdict line (R54); the
primitive suite has 127 cases, the resource outcomes among them
(R51); the hostile battery's 7e refuses an exempt kind under a
candidate's name (R49).

Since slice 10 (2026-09-14; GPT-6's ratification memo, records §4.9):
the loader pins hold the `if` condition's typing (`if_private`,
`if_type`, `if_one`, `if_ok`; R56), the propagated pending measure
(`realize_pending_via`, with `loader_test` checking the `pending=`
lines; R58), the entry codecs by identity (`entry_shape`, run by
`entry_test`: a `Tree` with the list arities is `bad_entry`; R59) and
the qualified-name collision in E and in L (`qualified_collision`,
`qualified_collision_l`; R61); `loader_test` checks the implementation
check's three `DISCHARGE` kinds (R57).

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
