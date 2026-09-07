# v3/kernel — K's rule inventory (phase 0) and, from phase 1, K itself

**Status: phase 0 deliverable, 2026-09-06 — the rule inventory and
procedure, written as the shard declarations K will use, with every
rule as a comment beside the constructor it governs** (user ruling
2026-09-06: declarations, not a prose restatement, "because the
declarations are phase 1's first file anyway and a prose restatement
would drift from them"). Nothing here executes yet; the `fn`s come in
phase 1. The declarations are the narrow-compatible E profile
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
| `prelude.shard` | `List Option Bool Pair Nat` — the stdlib copy the loader needs | — |
| `name.shard` | `Name` | hierarchical names; `check_name` (no redeclaration); reserved suffixes |
| `level.shard` | `Level` | `normalize`, `is_equivalent`, `is_geq`, `is_not_zero`, `normalizes_to_zero`; `check_level` (declared parameters only) |
| `expr.shard` | `BinderInfo Literal Expr` | the typing rules per constructor (`infer_*`), `whnf_core`/`whnf`, definitional equality (`is_def_eq_core` in its exact order), literal rules, projection rules |
| `decl.shard` | `ReducibilityHints DefinitionSafety ConstantVal RecursorRule QuotKind ConstantInfo InductiveType Constructor Declaration` | admission per declaration kind (`environment::add_*`), the inductive admission checks, recursor generation, the quotient axioms |
| `env.shard` | `Env` (raw and checked), `Outcome` | §3.3's outcomes; raw versus checked (§3.5); the fixed-identity `Nat` accelerators |

## The gate for these files

The toolchain profile is what the Rust loader reads flat: the check is
`./rust_bootstrap/target/release/eval direct <probe.shard>` on an
entrypoint that imports `env.shard` and defines `main` — the closure
must load and run (done 2026-09-06, exit 0). `bin/check` is **not** the
instrument: the old checker resolves imported type names through
`(use (:: <path-derived module> *))` lines, and the package root gives
these files a different module identity by design (LAYOUT.md). `use`
lines are added in phase 2 under the V3 reader's own resolution.

## The reconciliation ledger — what the pinned kernel has beyond the thesis

Each item is a rule K implements because the pin does; Lean4Lean's
status is noted where known. None is a departure.

1. **Nested inductives** (`inductive.cpp` `elim_nested_inductive_fn`, `restore_nested`, `check_uniform_ind_occs`): nested occurrences `I (J … T …)` are translated to auxiliary mutual types, admitted, and the recursors translated back; every occurrence must be applied to the declaration's own levels and parameters uniformly. Lean4Lean: modeled, partly proven.
2. **Structure eta** (`try_eta_struct_core`, `to_cnstr_when_structure`): `t ≡ mk t.1 … t.n` for non-recursive single-constructor types; also in recursor reduction on a non-constructor major premise.
3. **Unit-like types** (`is_def_eq_unit_like`): any two terms of a non-recursive structure with zero fields are definitionally equal.
4. **Proof irrelevance** (`is_def_eq_proof_irrel`): two proofs of definitionally equal propositions are equal; `is_prop` is up to level normalization (`imax 1 0`).
5. **Nat literals** (`infer_lit`, `reduce_nat`, `is_def_eq_offset`, `nat_lit_to_constructor`): GMP-backed literals of type `Nat`; `Nat.zero` and literal `0` interchangeable; `succ n` versus literal by offset; binary accelerators on literal arguments only for `Nat.add sub mul div mod gcd beq ble land lor xor shiftLeft shiftRight pow log2` with **size limits** (`LEAN_NAT_MAX_SIZE`; `Nat.pow`/`shiftLeft` count must fit 32 bits) — an `Exhausted`, not a rule. K binds each to a fixed admitted identity (FOUNDATION §3.2).
6. **String literals** (`string_lit_to_constructor`, `try_string_lit_expansion`): a string literal has type `String` and unfolds to **`String.ofList (List.cons Char (Char.ofNat c₁) (… List.nil Char))`** over the UTF-8-decoded scalar values — at `v4.33.1` the head is `String.ofList`, not `String.mk` (the logical `String` is `ofByteArray`; `v3/INVENTORY.md`). The expansion fires in recursor reduction, projection reduction and definitional equality against a `String.ofList` application.
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
19. **Resource limits** (`max_heartbeat`, `max_rec_depth`, `check_system`): every procedure is bounded; K reports `Exhausted(resource, site)` (§3.3, §9.4).
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
