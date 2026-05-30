# proving-bootstrap-v2

A new way to build software, in which **requirements — formal and
informal — cascade through layers of proven refinement into a running
application.** The end state is the kind of program you'd otherwise
write in Rust, except its structure and behavior are *formally
guaranteed* to meet the requirements it was derived from, top to
bottom. This is a development methodology, **not** a math/logic proof
library — the proof machinery below is the means, not the point.

The mechanism is a single, transitive **refinement** relation
`spec ⊑ … ⊑ code`: start from a high-level requirement, refine it into
a clear (probably inefficient) implementation, then into an efficient
one (eventually machine-code-like), with each link a *separately
proven* artifact rather than a tested one. Requirements→design,
design→code, and code→machine-code become the **same operation at
different altitudes**. Verified compilers (CompCert, CakeML) do the
bottom half this way; the goal here is general-purpose software, top to
bottom. See [TRANSFER.md](docs/archive/TRANSFER.md) for the full premise — including
the economic inversion (code is cheap; *coherent, proven requirements*
are scarce) that makes this timely now.

The substrate that makes each link *checkable* is a tiny trusted Rust
kernel loading a self-hosting proof checker written in a deliberately
narrow object language ("narrow"); the checker reasons equationally
about pure first-order programs in the same narrow form (same data
type, same evaluator, same reduction rules). The v1 pilot validated the
architecture end to end (naive `rev` ⊑ accumulator-passing `fast` ⊑
in-place memory reverse ⊑ wasm) at toy scale; v2 rebuilds the substrate
with the lessons applied — see TRANSFER.md for the changes from v1.

The product asymmetry, restated:
- **Generation is cheap and untrusted.** An LLM (or, later, an SMT
  solver) proposes the refinements and their proofs.
- **Checking is small and trusted.** A few hundred lines of audited
  Rust evaluator running a small kernel written in narrow.

## Quick start

```sh
cargo run --bin check -- std/*.shard          # the standard library
```

Expected output:

```
PASS  append_nil_right
…
PASS  mem_reverses

72 passed, 0 failed
```

`std/mem.shard` `(import …)`s the rest of `std/`, so checking any one file
pulls its dependencies transitively; checking `std/*.shard` checks the
whole library (each file loaded once). The demos live in `examples/`
(`cargo run --bin check -- examples/lia_basics.shard …`);
`examples/lia_rejects.shard` is a deliberate negative test (it FAILs).

The `check` binary loads the bundled kernel, then walks each `.shard`
file. A file may mix code, dependencies, and proofs:

```
(claim NAME GOAL PROOF)   ; check a theorem; cite later via (Lemma NAME)
(import "path/file.shard") ; load another file's code AND proven claims,
                          ;   transitively, de-duplicated (use-module = alias)
(type …) (fn …) (extern …); object-level definitions the proofs reason about
```

Run the Rust test suite (loader, evaluator, kernel-from-rust mirrors):

```sh
cargo test --release
```

109 tests as of slice 50.

## Repository layout

```
docs/
  LANGUAGE.md          ; narrow object language reference (syntax, semantics)
  BOUNDARIES.md        ; modeling external systems (extern + axiom, modellable
                       ;   externs, audit ledger pattern)
  REVISIT.md           ; design-decision ledger — every choice + when to
                       ;   revisit. The "why" lives here.
  archive/
    TRANSFER.md        ; v1→v2 handoff (archived): premise, lessons, mandate.
                       ;   Bootstrapped v2 from the v1 pilot; kept for rationale.

kernel/                ; the trusted kernel, written in narrow (1,823 NCNB)
  stdlib.shard          ;   List / Option / Pair / Bool
  term.shard            ;   Expr / Pat / shift / subst / open_many / close_many
  reduce.shard          ;   step / step_iota / step_smart (gated δ) / memo
  proof.shard           ;   Equation / Goal / Step / Proof / Theory / Cert
  module.shard          ;   Module / FnDef / TypeDef / ExternDef
  check.shard           ;   check_sequent — the entry point
  lia.shard             ;   LIA decision procedure (ByTheory backend)
  eqdec.shard           ;   equality-reflection backend (int_eq/sym_eq = True)
  ord.shard             ;   order-reflection backend (lt/le = True via LIA diff)
  farkas.shard          ;   linear-integer entailment (premises ⊢ lt/le, cert-checked)

src/                   ; the trusted-by-review Rust component
  ast.rs               ;   Expr / Pat / Type / Module ADTs the loader produces
  load.rs              ;   sexp → ast::Module; (ty …) and (tv T) sugars
  eval.rs              ;   CBV evaluator; stuck-and-intercept for primitives
  prim.rs              ;   primitive table (+ - * mod, int_eq, gen_fresh, …)
  nval.rs              ;   narrow-value builders for tests
  lib.rs               ;   loader entrypoint + Rust test suite (109 tests)
  bin/check.rs         ;   the `check` proof-script driver binary

std/                   ; the standard library — reusable code + theorems,
                       ;   each a topic file (code + its lemmas, co-located),
                       ;   wired together with (import …) (slice 51)
  arith.shard           ;   pure lia index identities (sub_zero, idx_cancel, …)
  order.shard           ;   Int order / disequality entailment (ord + farkas):
                       ;     lt_succ_from_lt, le_trans, lt_implies_neq,
                       ;     eq_from_le_both, lt_trans_to_neq, … (19 lemmas)
  nat.shard             ;   Nat + add_nat / int_of_nat / half_nat + nonneg /
                       ;     half_bound (the Induct2 showcase). imports order.
  list.shard            ;   (List T) append/rev/fast + the reverse tower
  map.shard             ;   (Map V) lookup/insert + extensional lemmas + int_eq_refl
  mem.shard             ;   M3 linear memory = (Map Int): read/write/swap/rev_loop/
                       ;     load/dump/rdump + framing + rev_loop_mirror + bridge
                       ;     + mem_reverses (the PROVEN capstone). imports the rest.

examples/              ; demonstrations (not the library)
  lia_basics.shard      ;   LIA examples incl. the Insts demo (slice 32)
  rewrite_with_demo /  ;   RewriteWith + Induct demos (import std/nat)
  add_nat_zero.shard    ;
  double_claims.shard   ;   Simp-unfold of a user fn (double_lib.shard)
  lia_rejects.shard     ;   NEGATIVE test — the kernel correctly REJECTs it

tools/
  zed-narrow/          ;   Zed editor syntax-highlighting extension
                       ;     for .shard files
```

## Current state

The reverse-refinement headline from v1's M2 is reproduced in v2 and
extended to polymorphism + proof reuse:

```
∀ xs : (List T). (fast xs Nil) = (rev xs)      ;; once, polymorphic
∀ xs : (List Int).    (fast xs Nil) = (rev xs)  ;; one Rewrite citation
∀ xs : (List Symbol). (fast xs Nil) = (rev xs)  ;; one Rewrite citation
```

Feature checklist (✓ = shipped in v2; → = next):

| Feature                                 | State    | Slice  |
|-----------------------------------------|----------|--------|
| Kernel structure + loader               | ✓        | 1–7    |
| Reducer (`step`, `step_iota`, `simp`)   | ✓        | 7c–14  |
| `Unfold` / `Reduce` / `Simp` steps      | ✓        | 10–14  |
| `Rewrite` with cited equations          | ✓        | 12, 17 |
| `Induct`, `CaseOn`                      | ✓        | 11, 13 |
| Polymorphic-type `Induct` over typedefs | ✓        | 16     |
| Pattern-variable `Rewrite` (∀-capture)  | ✓        | 20     |
| `Absurd` (closing by contradiction)     | ✓        | 9      |
| `ByTheory` + LIA decision procedure     | ✓        | 22     |
| `eqdec` theory (`int_eq`/`sym_eq` = True) | ✓      | 33     |
| CLI driver (`check` binary)             | ✓        | 23     |
| `(use-module …)` loader                 | ✓        | 24     |
| Surface sugars (`'foo`, `(list …)`, `(ty …)`) | ✓  | 25, 28 |
| `RewriteWith` (conditional citations)   | ✓        | 27     |
| Reverse-tower capstone in v2            | ✓        | 29     |
| Simp guarding (gated δ + list-memo)     | ✓        | 30     |
| Polymorphism in fn sigs + `(tv T)`      | ✓        | 31     |
| Insts pre-instantiation                 | ✓        | 32     |
| Finite maps (Int keys) + extensional lemmas | ✓    | 33     |
| Cross-module composition (`use-module` deps) | ✓   | 34     |
| M3 linear-memory model + array framing  | ✓        | 34     |
| `ord` theory (`lt`/`le` = True via LIA diff) | ✓   | 35     |
| `farkas` theory (entailment ≤/</≠/=, +plain eq) | ✓ | 37-42 |
| M3 loop invariant — untouched (below + above) | ✓     | 39,40  |
| M3 loop invariant — mirror (`rev_loop` reverses) | ✓  | 44     |
| M3 capstone (`rev_loop ⊑ rev`: full list↔mem refinement) | ✓ | 45-50 |
| Two-step induction (`Induct2`, Nat-shaped)   | ✓  | 50     |
| `(import …)` — transitive, deduped deps + `std/` library | ✓ | 51 |
| Polymorphic-key maps `(Map K V)`        | →        |        |
| Defunctionalized higher-order           | →        |        |
| Measure / well-founded recursion        | →        |        |
| Mutual recursion + mutual induction     | →        |        |
| `let` in term language (sub-term sharing) | →      |        |
| Module-tree loader (`(module …)`)       | →        |        |
| Audit ledger tool                       | →        |        |
| Self-hosting kernel tests in sexp       | →        |        |

Each `→` row is also captured in [docs/REVISIT.md](docs/REVISIT.md)
under its corresponding "Revisit when:" hook — the README is the
roadmap, REVISIT is the rationale.

## Roadmap

Ordered by leverage on the v2 mandate (see TRANSFER.md §"Change these
premises"). Each item links to its REVISIT entry if one exists.

### Big-ticket mandate items

1. **Finite maps / collections** — TRANSFER mandate #2. *Slice 33
   shipped the first cut:* `(Map V)` over **Int keys** (assoc list,
   prepend-insert, first-match-lookup) with an extensional lemma
   library (`lookup_insert_eq`, `lookup_insert_neq`, `insert_shadow`),
   enabled by the `eqdec` backend deciding `int_eq k k = True`. Map
   facts are stated EXTENSIONALLY — quantified over a probe key under
   `lookup` — because prepend-insert leaves structurally-distinct but
   observationally-equal maps. *Remaining:* polymorphic keys
   `(Map K V)` (needs a key-equality mechanism — couples to the
   defunctionalized-HOF item below); a richer lemma library
   (`remove`, `keys`, domain reasoning); the gateway to declarative
   specs like `perm` for sorting; and possibly a Rust-side hashmap
   primitive for performance.

2. **Defunctionalized higher-order** — TRANSFER mandate #3. Pass
   function *names*, not closures. ACL2's `apply$` is the model.
   This is the prerequisite for effect-as-data per
   [docs/BOUNDARIES.md](docs/BOUNDARIES.md), and for recovering
   `map`/`fold`/`filter` without reintroducing binders.

3. **Measure / well-founded recursion** — TRANSFER mandate #5.
   Real algorithms (divide-and-conquer, graph traversal) aren't
   structurally recursive on a subterm. Costs the
   syntactic-totality-for-free property — termination becomes a
   discharged proof obligation.

4. **Mutual recursion + mutual induction** — TRANSFER mandate #6.
   Needed for any mutually-inductive AST (expr/stmt, block/instr).

5. **`let` in the term language** — TRANSFER gotcha #1; the v1
   `simp` blowup root cause. Slice 30 partially mitigates with
   the outer-loop memo, but proper sharing in the term
   representation is the long-term fix.

### Smaller kernel / loader gaps

- **Module-tree loader** (`(module …)`) — slice 23 reserved the
  syntax but rejects it. The current kernel loader is a hardcoded
  flat path list in `src/lib.rs`. See REVISIT, *Proof-file module
  syntax*.
- **Bridging-axiom tag** — distinguish "the extern matches the
  model" from "the extern has these direct properties" at the
  `Axiom` entry. See BOUNDARIES.
- **More theory backends** — LIA shipped slice 22; bitvector,
  arrays, congruence closure are the natural next ones. The
  `(ByTheory NAME Cert)` slot is theory-pluggable.
- **`Insts` validation hardening** — duplicate-Inst names are
  silently first-match-wins (slice 32). Could tighten to reject
  duplicates if it becomes a footgun.
- **Sub-tree memo in `simp_expr`** — slice 30's memo is at the
  outer fixed-point loop only. Inner `step_smart` recursion does
  not thread the memo (narrow has no monadic bind). Both this and
  hash-cons/sharing of Expr are flagged `TODO[v3]` in
  `kernel/reduce.shard:493`.

### Tooling

- **Audit ledger tool** — walk a proof DAG, collect every axiom and
  extern. Easy once the data shapes are stable; just hasn't been
  written. See BOUNDARIES.
- **Self-hosting kernel tests in sexp** — many Rust tests in
  `src/lib.rs` are mirrors of what a sexp claim could state. Once
  the loader is rich enough to express those tests as claims,
  shrink the Rust test count and grow the sexp claim count. Right
  now Rust mirrors guard kernel-side behavior; sexp claims exercise
  end-to-end including loader paths.
- **Module-tree loader for the kernel itself** — once `(module …)`
  works, migrate `kernel/*.shard` to `kernel/mod.shard`-style tree.
  Cosmetic but consistency-tightening.

## Architecture in two paragraphs

The kernel is written in narrow. Narrow is a small total-pure
first-order language whose grammar fits on one page (see
[docs/LANGUAGE.md](docs/LANGUAGE.md)). The Rust runtime loads
narrow source into runtime values and walks them with a
straightforward CBV evaluator; primitive symbols (`+`, `int_eq`, …)
are stuck-and-intercept — the narrow reducer treats them as
unknown calls, and the Rust runtime recognizes them and applies
the primitive table.

Proofs are `(claim NAME (Goal …) (Proof …))` — the Goal is a
`(List Param) (List Equation) Equation`-shaped value, the Proof
is a `(Steps …)`/`(Induct …)`/`(Rewrite …)`/`(ByTheory …)` tree.
`check_sequent` (defined in narrow) dispatches the Proof against
the Goal. Successful claims are consed onto a running Theory,
citable by name from later claims. The Theory is content-stored
as `(Proven NAME GOAL)` or `(Axiom NAME GOAL)` — the latter making
the audit boundary visible at the kernel layer.

## Conventions

- **Slice = one logical change set committed atomically.** Each
  commit message starts `slice N: …` and includes test/claim counts
  before and after. Read `git log --oneline` for the slice history.
- **Trusted core touch is called out explicitly.** Changes to
  `kernel/*.shard` or `src/*.rs` mean the audited surface grew or
  shifted. Changes to `examples/*.shard` or `tests` do not.
- **REVISIT entries are first-class.** Every design decision under
  uncertainty has an entry with the "what was chosen", "why now",
  and "revisit when" triad. The README's roadmap section is a
  view-by-priority over REVISIT's "revisit when" hooks.

## Status

- **Substrate:** v2 kernel + loader + driver. Feature-complete for
  the v1 reverse-tower headline; extended with polymorphism, Simp
  guarding, ByTheory (LIA + eqdec + ord + farkas), Insts, finite maps (Int
  keys), and the M3 linear-memory model + array framing.
- **Trusted core size:**
  - Kernel narrow code: **1,823 NCNB** across 10 `kernel/*.shard`.
  - Rust trusted-by-review: **1,136 NCNB** across
    `ast.rs` + `eval.rs` + `load.rs` + `prim.rs` + `bin/check.rs`.
  - (Plus ~2,000 NCNB of Rust tests + builders in `lib.rs` + `nval.rs`
    that are not part of the trusted surface.)
- **Next:** see Roadmap above. The "Big-ticket mandate items" list
  is the gating set for getting v2 to the TRANSFER north-star bar
  (schedule-refinement proofs over a partitioned compute graph).
