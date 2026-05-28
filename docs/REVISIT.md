# v2 → v3 Revisit Ledger

Decisions made during v2 design under uncertainty. Each entry is: what
was chosen, why now, and what would push us to revisit in v3 rather
than retrofit during v2.

v2 is a prototype. If a choice here proves painful in practice, this
is where to start when planning v3.

## Architecture / Trust

### Locally-nameless term representation
- **Chose:** hybrid `FVar Symbol` + `BVar Int` (de Bruijn). See
  `kernel/term.sexp`.
- **Why now:** `subst` (the hot path) stays simple; free vars keep
  names in intermediate goals (matters for LLM-in-the-loop authoring);
  capture-free by construction for the common substitution operation.
- **Revisit if:** the kernel ends up dominated by off-by-one bugs in
  `shift`/`open_many`, or the open/close discipline causes recurring
  confusion. Alternatives: full de Bruijn (uniform but unreadable raw),
  names + α-rename (heavy fresh-name plumbing in a first-order pure
  setting).

### Trusted-by-review Rust component
- **Chose:** Rust evaluator audited rather than proven correct.
- **Why now:** proving the bootstrap evaluator is its own project,
  and at this scale code review is tractable.
- **Revisit if:** TCB anxiety becomes load-bearing for users, or if
  we want to bootstrap a proof of the Rust evaluator.

### No termination check in narrow
- **Chose:** narrow trusts authors; nonterminating code just hangs.
  Termination admissibility lives in the full-language checker.
- **Why now:** narrow is a small audited substrate; a syntactic check
  tangles with mutual recursion and adds trusted Rust code.
- **Revisit if:** we burn debugging time on bootstrap-kernel loops a
  cheap syntactic check would have caught.

## Language Surface

### sexp file format
- **Chose:** s-expressions for all source and artifact files
  (placeholder extension `.sexp`).
- **Why now:** off-the-shelf parsers, hand-writable, LLM-fluent,
  zero parser TCB beyond the sexp library itself.
- **Revisit if:** paren density makes large kernel files unreadable.
  Next step would be a Rust-like surface syntax with a parser written
  in the object language, lowering to canonical sexp.

### `if` as a core form
- **Chose:** primitive form, not match-on-Bool sugar.
- **Why now:** match-on-Bool was painful even in 50 lines of
  experimentation; `if` is universal and trivial in the evaluator.
- **Revisit if:** ever — hard to imagine being wrong.

### Wildcard `_` in patterns and ignored binder positions
- **Chose:** allowed in both positions, same character.
- **Why now:** load-bearing for non-trivial pattern code; conventional.

### Parallel `let` only
- **Chose:** all RHSs see outer env; sequential expressible as
  nested `let`.
- **Why now:** cleaner for equational reasoning (no order dependency).
- **Revisit if:** common idioms force ugly nesting; could add `let*`
  surface sugar.

### Pattern binding order: innermost-first
- **Chose:** `match_pat` accumulates bindings such that
  `bindings[k]` corresponds to `BVar k` — the rightmost (last
  encountered) PVar binds the innermost (lowest) index.
- **Why now:** consistent with `open_many` lookup by BVar index;
  standard convention.
- **Revisit if:** the inversion produces awkward arithmetic when
  writing the surface→core elaborator.

### Erased polymorphism in narrow
- **Chose:** type variables permitted syntactically; no narrow-level
  parametricity check.
- **Why now:** lets the kernel use generic data structures cleanly;
  the full-language checker enforces parametricity for theorems.
- **Revisit if:** subtle parametricity violations creep into the
  bootstrap kernel and create real bugs.

## Primitives

### Native `Int` (bignum) + `Symbol`, no fixed-width
- **Chose:** arbitrary-precision `Int` and interned `Symbol` as the
  only primitive value types. Modular / fixed-width arithmetic is a
  library wrapper around `Int` (`mod`, bitwise ops as primitives).
- **Why now:** simpler semantics, fewer reasoning rules, decidable
  BitVec theory available via SMT later.
- **Revisit if:** SMT integration is cleaner with BitVec primitives,
  or modular-heavy targets push library performance unacceptably.

### Primitive comparisons return user `Bool`
- **Chose:** `int_eq` / `sym_eq` return the user-defined `Bool` ADT.
  Rust evaluator learns the `True` / `False` ctor names via a module
  header directive.
- **Why now:** keeps call sites natural (`(if (int_eq a b) ... ...)`);
  avoids 0/1-Int wrapping ceremony.
- **Revisit if:** multiple incompatible `Bool` definitions need to
  coexist, or the coupling pinches when bootstrapping variants.

### Primitives reachable from the kernel's reducer
- **Chose:** the kernel's `step` (and through it `simp_expr`) handles
  primitive calls via a `try_step_prim` helper. When `lookup_fn`
  fails on a Call value's symbol, `try_step_prim` pattern-matches
  the args against the primitive shapes (two IntLit / two SymLit /
  …) and invokes the primitive *in narrow code* (e.g. literal
  `(+ a b)` inside `try_step_prim`'s body). The Rust evaluator's
  stuck-and-intercept then dispatches it to `prim::try_apply`.
- **Why now:** stuck-and-intercept on the OUTER level handles Calls
  the Rust eval is asked to execute as code. But when the kernel's
  step is reducing an Expr VALUE (data) that happens to be a Call
  to a primitive, the Rust eval never sees a `+` to execute — it
  sees a Ctor value with "+" in it. The kernel has to do its own
  primitive dispatch on that data. Discovered at slice 8 when the
  first user-fn proof fell over because `(double 5)` correctly
  unfolded to `(+ 5 5)` but then stuck (no kernel handling of `+`).
- **Cost:** the kernel duplicates the primitive table (names and
  argument-shape patterns). Each primitive added in `src/prim.rs`
  also needs a clause in `try_step_prim`.
- **Revisit if:** the primitive set grows beyond a dozen or two,
  or if we want primitives to be discoverable rather than
  hardcoded. The cleaner long-term fix is for the kernel to call
  out to an extern (`try_prim_step`) that the Rust runtime
  intercepts, putting the primitive table on one side only. That
  introduces a runtime hook the kernel's reducer doesn't currently
  need; weighed against the table-duplication cost, deferred.

### Primitive call protocol: stuck-and-intercept
- **Chose:** the narrow reducer treats Call'd symbols with no `FnDef`
  as stuck (returns `None`). The Rust runtime, driving the reducer,
  is responsible for recognizing primitive call sites in the stuck
  expression and applying them itself.
- **Why now:** keeps narrow free of any primitive-name list (no
  coupling to the Rust primitive set); the narrow code we wrote does
  not need to know what primitives exist or do.
- **Revisit if:** Rust's "scan for primitive call sites" pass becomes
  awkward, or if we want the narrow reducer to be runnable
  standalone (e.g., in a self-test). Alternatives: emit a `PrimApply`
  marker form the reducer can construct, or carry a primitive-name
  list on `Module` so the reducer can route calls explicitly.

### Library maps, not primitive
- **Chose:** maps as a library type (balanced tree of pairs).
- **Why now:** keeps Rust TCB smaller; matches "primitives only when
  semantics demand it."
- **Revisit if:** kernel memo / environment access measures as a real
  bottleneck. Promotion to a Rust-backed primitive is the expected
  path.

## Proof Language

### `Theory` as flat ordered cons-list
- **Chose:** `Theory` is `(TheoryEmpty)` / `(TheoryCons name claim rest)` —
  later theorems cite earlier ones by name; order prevents circularity.
- **Why now:** matches v1's working approach; the kernel only needs
  citation-by-name and ordering at this stage.
- **Revisit when:** refinement composition and cross-artifact reuse
  start mattering. The natural upgrade is a content-addressed
  artifact store with dependency tracking — a separate concern from
  the kernel ADT, but it would replace `Theory`.

### `Reduce` and `Simp` are now split (was: collapsed)
- **History:** between slices 7c and 13, `Reduce` was wired as full
  δ+ι (driving `simp_expr` = `step` to fixed point). `Simp` was
  stubbed. Slice 13b surfaced the gap: IH-consuming inductive
  proofs need a reducer that fires Matches/Ifs but STOPS at
  recursive sub-calls, so a Rewrite-via-IH can match the exposed
  subterm. Slice 14 split them.
- **Chose (slice 14):**
  - `Reduce side` is ι-only: drives `simp_iota_expr`, which uses
    `step_iota` — Match firing, If dispatch, Let opening, descend
    into Ctor/Call args. NEVER unfolds a Call (user fn or primitive).
  - `Simp side` is full δ+ι: drives `simp_expr` (= the previous
    Reduce semantics).
- **Why now:** the split is what enables the canonical
  `Induct → case → Unfold → Reduce → Rewrite IH → Refl` pattern.
- **Revisit when:** the v1 "blowup" problem reappears. The classical
  fix is to make `Simp` *guarded* — memoized, with cycle detection
  or size bounds. `Reduce` (ι-only) is naturally bounded since
  every step strictly decreases reducible-match count, but `Simp`
  is not. v1's lesson is non-negotiable here; the unguarded `Simp`
  is a known liability inherited from this slice.

### `ByTheory` cert payload under-specified per theory
- **Chose:** `(Cert Symbol Expr)` — theory name plus a payload
  encoded as an `Expr`. Concrete payload shapes (e.g. LRAT-style
  trace for LIA, bitblasting transcript for BV) are pinned only
  when each theory's checker is implemented.
- **Why now:** reserves the proof leaf so the kernel grammar is
  stable; defers cert encoding per theory.
- **Revisit if:** `Expr` is too restrictive for some theory's
  certificate format (e.g., one wants opaque binary blobs); promote
  the payload to a more general carrier.

### Rewrite pattern-matching descends into `Match` arm bodies
- **Chose:** v2's rewriter descends under binders (v1 refused to,
  to dodge variable capture). Locally-nameless makes this capture-
  safe: pattern variables are tracked out-of-band, and BVars in the
  target match structurally.
- **Why now:** unlocks rewrites v1 couldn't express; depth-tracking
  machinery already exists in `shift` / `open_many`.
- **Revisit if:** matching-under-binders produces surprising
  performance or correctness issues. Falling back to v1's stance
  is a small edit.

### Implicit type instantiation by default in full language
- **Chose:** ML-style inference; explicit instantiation available
  when needed.
- **Why now:** matches the TRANSFER mandate; explicit annotation is
  the rare case.
- **Revisit if:** inference ambiguities accumulate in practice.

## Foreign Boundaries

### `ExternDef` as a separate `Module` field
- **Chose:** extended `Module` from `(types fns)` to `(types fns externs)`;
  `ExternDef` is its own ADT with signature only (no body).
- **Why now:** keeps `FnDef` cleanly "fully-defined function with a
  body" and makes the audit query "what externs does this module
  declare?" a one-liner. Alternative was `FnDef` with `(Option Expr)`
  body where `None` = extern.
- **Revisit if:** we want to attach per-extern metadata (linkage
  info, model bindings, axioms) and the separate-list shape starts
  fighting that. The `Option`-body variant would let a richer `FnDef`
  carry all of it uniformly.

### `TheoryEntry` tagged `Proven` vs `Axiom`
- **Chose:** `(type TheoryEntry (Proven Symbol Goal) (Axiom Symbol Goal))`
  rather than untagged `(Symbol Goal)` with a parallel set of axiom
  names.
- **Why now:** makes the audit boundary visible at every theory
  access — every code path that looks at a theory entry has to
  acknowledge whether it's looking at a proven theorem or an axiom.
- **Revisit if:** we add more entry kinds (bridging axiom, deferred
  obligation, etc.) and the binary sum gets awkward. Then a richer
  enum or a record with kind tags is the upgrade.

### Bridging-axiom distinction not in v2
- **Chose:** all axioms are `Axiom` entries; no separate tag for
  "this axiom is the model-mirrors-reality kind."
- **Why now:** the distinction is auditing UX, not kernel semantics;
  not worth the additional ADT shape until the audit tool exists.
- **Revisit when:** the audit ledger tool lands and bridging axioms
  start outnumbering operational ones in the modellable-extern
  pattern. Add a `BridgingAxiom` constructor or a tag on `Axiom`.

## Implementation Conventions

### Manual Option-bind in narrow (no do-notation)
- **Status:** narrow has no monadic bind / do-notation, so chained
  `(Option …) → (Option …)` pipelines collapse into pyramids of
  `(match … ((Some x) …) (None None))`. `do_induct` has an 8-level
  nested destructure that would be ~5 lines with `bind_opt`.
- **Why now:** narrow is deliberately minimal; bind/do is a full-
  language concern (it wants HOF, i.e. `apply$`).
- **Revisit when:** the kernel grows enough Option-chaining that the
  pyramids become a maintenance problem. The fix is in the full
  language: define `bind_opt` and rewrite the kernel using it once
  the kernel is dogfooded onto the full evaluator.

### Fresh-symbol generation as an effectful primitive
- **Chose:** `(gen_fresh)` is a runtime-provided primitive that returns
  a unique `Symbol` each call. Used by the kernel when opening binders
  (Induct / CaseOn) to mint fresh FVar names.
- **Why now:** the pure alternative — threading a counter through
  every binder-introducing operation — is exactly the `Pair`-cascade
  ugliness from option 1 of the capture-avoidance comparison, and
  it'd pollute the entire kernel's signatures. A single effectful
  primitive is a small, localized exception that keeps the rest of
  the kernel pure-shaped.
- **Revisit if:** we ever want to formally reason about the kernel
  inside the system itself — at that point the effectful primitive
  becomes a soundness obstacle, and we'd switch to explicit counter
  threading (probably with `let` to soften the boilerplate, or with
  the full language's HOF to abstract the threading).

### Single-pass `subst`; env values not re-substituted
- **Chose:** `subst` replaces FVars once; values in env taken verbatim.
- **Why now:** standard proof-assistant convention; iterated subst
  available by repeated calls.
- **Revisit if:** iterated substitution becomes the common case.

### Language and project not yet named
- **Chose:** placeholder `.sexp` extension; project remains
  "proving_bootstrap_test_v2".
- **Revisit:** once the kernel takes shape and a name suggests itself.

## CLI / Tooling

### Proof-file surface syntax: light sugar on value-construction sexp
- **Chose (slice 23, sugared slice 25):** `(claim NAME GOAL PROOF)`
  where GOAL and PROOF are narrow expressions parsed against the
  kernel's ctor set and evaluated. Surface sugar:
    - `'foo` → `(quote foo)` → `SymLit foo` (lexpr's reader handles
      `'` automatically).
    - `(list a b c)` → `(Cons a (Cons b (Cons c Nil)))` (new
      special form in `load_expr`, slice 25; reserves `list` as
      a name).
- **Why now:** reuses `load::expr_from_value` plus the existing
  evaluator. The kernel's ctor application IS the value-construction
  syntax. The two sugars cut a typical claim by ~50% of LOC and
  bring authoring close to a "math content" / "syntactic noise"
  ratio that's acceptable for hand-writing.
- **What's still verbose:** `(TCon 'Int (list))` for base types
  (the `(list)` for empty type args is unavoidable without further
  sugar); the `Cons` / `Nil` / `Some` / `None` Pair chain in
  more-elaborate proofs.
- **Revisit when:** real proof authoring surfaces specific pain
  points. Cheap next steps if needed: a unary type ctor shorthand
  (e.g., `Int` as a bare symbol meaning `(TCon 'Int (list))`), or
  a `(pair a b)` form. Heavier: a separate proof-script surface
  syntax that lowers to canonical claim sexp.

### Proof-file module syntax: parse-but-error
- **Chose (slice 23):** `(module NAME)` is recognized as a top-level
  form but rejected with "not yet implemented in v1".
- **Why now:** locks in the syntax so v1 proof files won't need
  rewriting when the directory-tree loader lands. Avoids implementing
  the loader (with name resolution, cycle detection, path
  resolution) before the kernel's scale demands it.
- **Revisit when:** a proof artifact wants to span multiple files
  with cross-references. Likely concurrent with introducing
  hierarchical names (which v1 supports via Symbols already, but
  nothing produces or relies on hierarchical Symbol contents yet).

### `(use-module "path")` accumulates, does not replace
- **Chose (slice 24):** each `(use-module "path/to/file.sexp")`
  directive loads that file as a Rust `ast::Module` and *merges*
  its types / fns / externs into a running user-module accumulator.
  All subsequent claims (until end-of-run) see the merged module
  as the `m` arg to `check_sequent`. The accumulator persists across
  files in a single binary invocation (no per-file reset).
- **Why now:** matches Rust's `mod` semantics in spirit (declarations
  bring items into scope, additively). Lets a proof file pull in
  multiple lib files cumulatively without manual concatenation.
  Last-replaces would force any claim file using lib A and lib B
  to manually concat them upstream.
- **Cost:** name conflicts across modules are silent — first
  declaration wins on lookup_fn / lookup_typedef. A real scaling
  story wants explicit imports / namespacing; for v2 this likely
  unifies with `(module …)` and hierarchical names.
- **Revisit when:** any of: real conflicts arise; per-file isolation
  becomes desirable (e.g., for parallel checking); or the (module …)
  loader lands and subsumes use-module's role.

### ast::Module → runtime-value conversion in the binary
- **Chose (slice 24):** the mechanical Rust-AST → runtime-Ctor-value
  conversion (Expr/Pat/Type/CtorDef/FnDef/ExternDef/TypeDef/Module)
  lives in `src/bin/check.rs`. ~110 LOC, no public API change.
- **Why now:** the binary is the only caller; promoting to lib /
  ast.rs would grow the trusted core for a single consumer.
- **Revisit when:** a second caller appears (e.g., tests that want
  to load a user module from disk rather than building with nval
  helpers, or a future REPL). At that point promote to a
  `pub module_value::to_value` API.

### Kernel loader is a flat path list
- **Chose:** the kernel's seven `.sexp` files are loaded by a
  hardcoded list in `lib.rs::load_kernel_from`. Tests and the
  `check` binary share that list.
- **Why now:** the kernel itself doesn't yet use the (module …)
  directive (and `module` isn't implemented anyway). Migrating the
  kernel to a module-tree layout is a separable slice.
- **Revisit when:** the directory-tree loader lands. Migrating the
  kernel to `kernel/mod.sexp` becomes a consistency cleanup.
