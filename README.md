# Shard
<p align="center">
  <img src="assets/shard.png" alt="shard" width="128" height="128">
</p>

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

The language is **shard**, and its defining choice is that **programs
are data**: functions, data structures, whole applications are
in-memory compute structures the tooling can inspect, transform, and
prove about. That is what makes each refinement link *checkable* — the
proof checker, itself written *in shard*, reasons equationally about
other shard programs (same data type, same evaluator, same reduction
rules). Crucially this is a **build-time** power, not a runtime one. A
serious shard application is **compiled** to a bare binary — no
interpreter, no GC, no reflection, no kernel sidecar — so the snake
demo reduces to a standalone x86 executable that is just its `step`
function plus IO. The language is kept compilable to the metal by
design: features that would smuggle in a runtime (first-class closures,
runtime `eval`) are admitted only if they fully compile away.

Underneath, a deliberately tiny **Rust bootstrap** understands only
*narrow* shard — a minimal subset — and hosts the rest until shard can
compile itself. The engine, kernel, checker, and parser are all written
in narrow shard; new language features grow in *full* shard, in the
shard engine, and are pushed down into the Rust narrow backend only when
genuinely needed. There is no full→narrow lowering — narrow is just the
bootstrap floor, and the compile story is full shard straight to a
machine target. When that compiler exists, `rust_bootstrap/` is deleted
and shard stands on its own.

The v1 pilot validated the whole arc end to end at toy scale: a
hand-written **wasm** reverse, run on a wasm interpreter *written in
shard*, proven equal to functional `rev` for all inputs as the composed
chain `wasm ⊑ rev_loop ⊑ rev`. Verification reached all the way to the
machine code, because the machine is modeled in the same provable
language. v2 rebuilds the substrate with the lessons applied — see
[docs/OVERVIEW.md](docs/OVERVIEW.md) for the full design intent and
TRANSFER.md for the v1→v2 changes.

The product asymmetry, restated:
- **Generation is cheap and untrusted.** An LLM (or, later, an SMT
  solver) proposes the refinements and their proofs.
- **Checking is small and trusted.** A small kernel written in shard,
  run today on a disposable Rust bootstrap that will be compiled away.

## Quick start

The Rust bootstrap lives in `rust_bootstrap/`; build it once. Everything runs
through the `eval` binary — it executes a World-threading shard program, and
the proof checker IS such a program (`kernel/check.shard`), so checking a
file means running the self-hosted checker on it:

```sh
cargo build --release --manifest-path rust_bootstrap/Cargo.toml
rust_bootstrap/target/release/eval run kernel/check.shard std/mem.shard
```

Expected output (tail):

```
PASS  …
PASS  mem_reverses

53 passed, 0 failed, 29 axiom(s) admitted without proof
```

`std/mem.shard` `(import …)`s the rest of `std/`, so this one file checks the
whole library (dependencies load transitively, de-duplicated). An optional
trailing argument focuses a single claim and traces it
(`… kernel/check.shard std/mem.shard mem_reverses`). The demos live in
`examples/` (`… eval run kernel/check.shard examples/lia_basics.shard`);
`examples/lia_rejects.shard` is a deliberate negative test (it FAILs).
Sources are kept in canonical form by the proven formatter:
`rust_bootstrap/target/release/eval run tools/shardfmt/shardfmt.shard FILE`.

The checker loads the target's import closure, then walks each `.shard`
file. A file may mix code, dependencies, and proofs:

```
(claim NAME GOAL PROOF)   ; check a theorem; cite later via (Lemma NAME)
(import "path/file.shard") ; load another file's code AND proven claims,
                          ;   transitively, de-duplicated (use-module = alias)
(type …) (fn …) (extern …); object-level definitions the proofs reason about
```

The Rust test suite (`cargo test --release --manifest-path
rust_bootstrap/Cargo.toml`, 32 tests) covers what Rust owns: the loader,
the evaluator, and the primitives. Kernel behavior is regression-tested by
the self-hosted corpus above, not by Rust mirrors (the legacy `check_seq_*`
mirror suite was deleted once it went stale against the kernel's evolving
data shapes — the corpus had long superseded it).

## Repository layout

```
docs/
  OVERVIEW.md          ; the v2 design intent — the why behind the architecture
  LANGUAGE.md          ; narrow object language reference (syntax, semantics)
  BOUNDARIES.md        ; modeling external systems (extern + axiom, modellable
                       ;   externs, audit ledger pattern)
  REVISIT.md           ; design-decision ledger — every choice + when to
                       ;   revisit. The "why" lives here.
  M3-V1-VS-V2.md       ; the M3 memory capstone, v1 vs v2 compared
  archive/
    TRANSFER.md        ; v1→v2 handoff (archived): premise, lessons, mandate.
                       ;   Bootstrapped v2 from the v1 pilot; kept for rationale.

kernel/                ; the kernel + its self-hosted toolchain, all narrow
                       ;   (5,582 NCNB total; breakdown under Status)
  ;; — the checking core —
  stdlib.shard          ;   List / Option / Pair / Bool
  term.shard            ;   Expr / Pat / shift / subst / open_many / close_many
  reduce.shard          ;   step / step_iota / step_smart (gated δ) / memo
  proof.shard           ;   Equation / Goal / Step / Proof / Theory / Cert
  module.shard          ;   Module / FnDef / TypeDef / ExternDef + FnTrie dispatch
  checker.shard         ;   check_sequent + the step interpreters — the kernel proper
  check.shard           ;   the checker ENTRYPOINT, itself a World app (main)
  lia.shard             ;   LIA decision procedure (ByTheory backend)
  eqdec.shard           ;   equality-reflection backend (int_eq/sym_eq = True)
  ord.shard             ;   order-reflection backend (lt/le = True via LIA diff)
  farkas.shard          ;   linear-integer entailment (premises ⊢ lt/le/≠/=, cert-checked)
  ;; — the self-hosted front-end + engine —
  reader.shard          ;   s-expr reader + module parser, validated byte-for-byte
                       ;     vs load.rs; located parse-failure diagnostics
  proof_reader.shard    ;   the proof DSL parsed DIRECTLY to native step structures
  desugar.shard         ;   named-hypothesis desugaring (labels → positional Hyp k)
  loader.shard          ;   the World I/O externs + import-closure resolver
                       ;     (mode-aware module resolution: check vs run)
  driver.shard          ;   the production pipeline: build modules, run claims,
                       ;     requirement obligations, the (bin …) met/unmet report
  eval.shard            ;   the engine: env-machine `ev` (CBV, Rc-shared host
                       ;     values), FnTrie call dispatch, TCO, inlined externs
  trace.shard           ;   UNTRUSTED diagnostics: the branch-aware FAIL tracer
  reader_corpus.txt    ;   parse-check differential corpus

rust_bootstrap/        ; the DISPOSABLE Rust bootstrap — host + parser + eval
  Cargo.toml           ;   until shard self-compiles; then this whole dir goes
  src/                 ;   the trusted-by-review Rust component
    ast.rs             ;     Expr / Pat / Type / Module ADTs the loader produces
    load.rs            ;     surface → ast::Module; (ty …) and (tv T) sugars
    eval.rs            ;     environment-machine evaluator; Rc-shared values
    prim.rs            ;     primitive table (+ - * mod, int_eq, sym_of_chars, …)
    lib.rs             ;     loader entrypoint + the Rust-owned test suite
                       ;       (loader/evaluator/prims — kernel behavior is
                       ;        regression-tested by the self-hosted corpus)
    bin/eval.rs        ;     the `eval` driver: runs World programs (the
                       ;       checker, apps, tools) + the World extern handlers

std/                   ; the standard library — DIRECTORY MODULES: each topic
                       ;   is a dir whose mod.req.shard is the reviewed public
                       ;   interface (opaque sig fns + requirement lemmas) and
                       ;   whose impl file carries bodies + fulfills proofs.
                       ;   Mode-aware: proofs see the opaque interface, running
                       ;   code gets the bodies. Top-level X.shard files are
                       ;   back-compat shims forwarding to the modules.
  arith/                ;   pure lia index identities (sub_zero, idx_cancel, …)
  div/                  ;   Euclidean div/mod foundation (WfInduct substrate)
  order/                ;   Int order / disequality entailment (ord + farkas)
  nat/                  ;   Nat + add_nat / int_of_nat / half_nat (+ Induct2)
  list/                 ;   (List T) append/len/rev algebra behind an opaque surface
  map/                  ;   (Map V) — an OPAQUE TYPE (private ctors) + extensional lemmas
  mem.shard             ;   M3 linear memory = (Map Int): read/write/swap/rev_loop
                       ;     + framing + mem_reverses (the PROVEN capstone);
                       ;     imports the rest — checking it checks the library

examples/              ; demonstrations (not the library)
  calc/                ;   the stage-0 calculator: spec-first (calc_spec) +
                       ;     run=spec equivalence + the MVU app + World/trace theorems
  io/                  ;   direct-style World I/O programs (filecat, calc_repl,
                       ;     echo_world, cat_lazy / cat_loop + clock theorems)
  snake_game/          ;   first requirement-isolation probe (pure step, R1–R4)
  snake_game_2/        ;   the full bin pipeline: mod.req.shard contract, arena
                       ;     spec vocab, PROVEN interactive play loop (parametric
                       ;     board, renderer faithfulness, no fin-split)
  modules_demo/        ;   directory-module mechanics + surface-discipline views
  lia_basics.shard      ;   LIA + Insts demos; wf_induct_demo / have_test /
                       ;     finsplit_test / list_named_hyp / rewrite_arms_test /
                       ;     rewrite_with_demo — one demo file per proof feature
  reverse_proof.shard   ;   the polymorphic reverse tower on the proof-DSL surface
  contract_demo.shard   ;   axiom + requirement/fulfills surface demo
  lia_rejects.shard     ;   NEGATIVE tests — each must FAIL for the right reason:
  module_gate_rejects.shard  ;   (kernel rejects a false LIA claim / a module-
  parse_rejects.shard   ;     surface violation / an unparseable file — the
                       ;     last exercises the located parse diagnostics)

tools/
  shardfmt/            ;   the canonical formatter — itself a requirement-
                       ;     contracted bin (the gate: it REFUSES output that
                       ;     parses differently). The whole tree is formatted.
  zed-narrow/          ;   Zed editor syntax-highlighting extension for .shard
```

## Current state

The reverse-refinement headline from v1's M2 is reproduced in v2 and
extended to polymorphism + proof reuse:

```
∀ xs : (List T). (fast xs Nil) = (rev xs)      ;; once, polymorphic
∀ xs : (List Int).    (fast xs Nil) = (rev xs)  ;; one Rewrite citation
∀ xs : (List Symbol). (fast xs Nil) = (rev xs)  ;; one Rewrite citation
```

Since then the substrate has moved from "a kernel with demos" to "a
toolchain that carries contracts": the whole front-end and checker are
self-hosted (the Rust bootstrap only evaluates), the standard library
lives behind reviewed module interfaces, and executables ship as
`(bin …)` artifacts whose acceptance contract is a `requires` list of
proven requirements over their I/O boundary. The flagship demos are
**snake_game_2** — a playable, interactive binary whose renderer
faithfulness, input response, and run discipline are proven against its
`mod.req.shard` contract at a parametric board size — and **shardfmt**,
the canonical formatter, contracted to refuse output that changes what
a file parses to (the whole tree is formatted with it).

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
| Char↔symbol primitives (`sym_of_chars`/`chars_of_sym`) | ✓ | self-host |
| S-expr reader + module parser **in shard** (`kernel/reader.shard`) | ✓ | self-host |
| Environment-machine evaluator (Rc values; ~700× faster) | ✓ | self-host |
| `check run` World/extern I/O — direct-style, effects axiom-stubbed | ✓ | self-host |
| extern dispatch + clock-discipline theorems (incl. oracle-driven loop) | ✓ | self-host |
| `eval` as a standalone shard World program (`examples/io/`) | ✓ | self-host |
| `.shard` rename + `rust_bootstrap/` split | ✓ | self-host |
| Module-elaborator (file→Module) **in shard** (`build_module`) | ✓ | self-host |
| Whole checker as a World app: the eval.rs→eval.shard→check.shard tower | ✓ | self-host |
| Engine perf: env-machine `ev` + TCO + FnTrie dispatch (mem 10-15min → 43s) | ✓ | self-host |
| `wf-induct` — well-founded recursion/induction on Int measures | ✓ | proofs |
| `have` (cut), `fin-split` (bounded-Int enumeration) | ✓ | proofs |
| Named hypotheses + load-time proof validation + branch-aware FAIL tracer | ✓ | proofs |
| `rewrite` descends into match arms (BVar-free patterns) | ✓ | proofs |
| Directory modules: `mod.req.shard` interfaces, mode-aware resolution | ✓ | modules |
| Opaque types (`(sig type …)` — private ctors, parse-enforced) | ✓ | modules |
| `requirement`/`fulfills` + the `(bin …)` artifact (met/unmet report) | ✓ | contracts |
| Proven interactive app — snake_game_2 (parametric board, 10 reqs met) | ✓ | apps |
| Located parse-failure diagnostics (5 failure modes) | ✓ | tools |
| `shardfmt` — gate-contracted formatter; whole tree canonical | ✓ | tools |
| Defunctionalized higher-order (compiles away) | →     |        |
| Mutual recursion + mutual induction     | →        |        |
| Audit ledger tool (the `(bin …)` trusts/requires report is the first cut) | → | |
| **shard → wasm/x86 compiler** (retires `rust_bootstrap/`) | → |   |

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
   observationally-equal maps. *Since then:* `(Map V)` became an
   OPAQUE TYPE behind `std/map`'s module interface (private ctors,
   parse-enforced), so consumers can only reason extensionally.
   *Remaining:* polymorphic keys
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

3. **Mutual recursion + mutual induction** — TRANSFER mandate #6.
   Needed for any mutually-inductive AST (expr/stmt, block/instr).

(TRANSFER mandate #5 — measure/well-founded recursion — SHIPPED as
`wf-induct`: induction on any Int measure expression, termination as
discharged farkas/ord obligations. `let` exists in the term language;
what remains of TRANSFER gotcha #1 is sharing-aware reduction — see
the memo/hash-cons bullet below.)

### Smaller kernel / loader gaps

- **Bridging-axiom tag** — distinguish "the extern matches the
  model" from "the extern has these direct properties" at the
  `Axiom` entry. See BOUNDARIES.
- **More theory backends** — LIA shipped slice 22; bitvector,
  arrays, congruence closure are the natural next ones. The
  `(ByTheory NAME Cert)` slot is theory-pluggable.
- **`Insts` validation hardening** — duplicate-Inst names are
  silently first-match-wins (slice 32). Could tighten to reject
  duplicates if it becomes a footgun.
- **Sub-tree memo in `simp_expr` / Expr sharing** — slice 30's memo
  is at the outer fixed-point loop only; hash-cons/sharing of Expr
  is the long-term fix (`TODO[v3]` in `kernel/reduce.shard`).
- **Checker theory threading is O(N²)** — the residual perf lever
  after the FnTrie work: each claim re-threads the growing Theory.
- **Parse-path recursion was O(input bytes) deep — FIXED** — the
  `(Cons h (recurse t))` list-builders cost one host-frame group per
  element; on file-sized lists that overflowed the stack (`check`
  topped out near ~150 KiB sources, shardfmt ~120 KiB). The hot pair
  was the extern-boundary conversions (`intlist_to_expr` /
  `expr_to_intlist` on read_file/write payloads); those plus the
  reader's builders (`take_atom`/`read_items`/`read_all`/`str_sugar`/
  `list_sugar`) are now accumulator-style — a 186 KiB file checks in
  64 MiB of stack. Residual (accepted): a single ~50 KiB atom or a
  single multi-thousand-element literal still recurses with its OWN
  size (structural walks over deep object-list values are inherent);
  `SHARD_STACK_MIB` overrides the 4 GiB default reserve for probing.

### Tooling

- **Audit ledger tool** — walk a proof DAG, collect every axiom and
  extern. The `(bin …)` trusts/requires met-unmet report is the
  first cut; the full-DAG walk remains. See BOUNDARIES.
- **Self-hosting kernel tests in sexp** — DONE by deletion: the
  legacy `check_seq_*` Rust mirror suite went stale against the
  kernel's evolving data shapes (pre-FnTrie `Module`) and the
  self-hosted corpus had long superseded it, so it was removed.
  Rust tests now cover only what Rust owns (loader/evaluator/prims).
- **Proof-authoring QoL backlog** — farkas `auto` certificates
  (loader-side solving), dev-mode subset checking, sharper
  rewrite-with failure diagnostics, filenames (not closure ordinals)
  in parse/check diagnostics, a `write_file` extern so shardfmt
  gains `--write`/`--check` in-place modes.
- **Kernel as a directory module** — `kernel/*.shard` is still a
  flat import list; migrating it to the `mod.req.shard` convention
  the rest of the tree uses is cosmetic but consistency-tightening.

## Architecture in three paragraphs

Everything is written in narrow — a small total-pure first-order
language whose grammar fits on one page (see
[docs/LANGUAGE.md](docs/LANGUAGE.md)) — and runs as a tower. At the
bottom, the Rust `eval` binary is a plain CBV environment-machine
evaluator plus the primitive table (`+`, `int_eq`, `sym_of_chars`, …;
stuck-and-intercept) and the World extern handlers (file I/O, args,
stdin keys, exit). It evaluates `kernel/eval.shard` — the self-hosted
engine: its own environment machine `ev` (Rc-sharing host values),
FnTrie call dispatch, tail-call optimization, and inlined externs.
That engine runs any World-threading shard program; the proof checker
is simply one such program.

Checking a file means running `kernel/check.shard`: the loader resolves
the import closure through the `read_file` extern (module imports
resolve MODE-AWARE — proof checking sees a directory module's
`mod.req.shard` interface, running code gets the impl bodies); the
self-hosted reader parses sources to `Module` values (with located
diagnostics on failure); the proof DSL is parsed directly to native
step structures and validated at load time; and the driver walks the
claims through `check_sequent`. Successful claims are consed onto a
running Theory, citable by name from later claims. The Theory is
content-stored as `(Proven NAME GOAL)` or `(Axiom NAME GOAL)` — the
latter making the audit boundary visible at the kernel layer. Failed
claims are explained by an UNTRUSTED tracer that replays the proof
spine branch-aware.

Above the proof layer sits the contract layer: a `(requirement NAME
GOAL)` declares an obligation (typically in a reviewed `mod.req.shard`),
a `(fulfills NAME PROOF)` in the implementation discharges it, and a
`(bin NAME (entry …) (externs …) (trusts …) (requires …))` artifact
declares an executable whose acceptance contract is its requires list —
`check` reports each requirement MET or UNMET, and the `trusts` list
names the extern bolt axioms that are that binary's trust surface.
Review attention concentrates on the `mod.req.shard` files and the
reviewed spec vocabulary they import; implementations and proofs are
fungible behind those surfaces.

## Conventions

- **One logical change set per commit, topic-prefixed.** The kernel
  build-out used `slice N: …` numbering (145 commits — read
  `git log --oneline` for that history); current work prefixes the
  subsystem instead (`snake_game_2: …`, `shardfmt: …`, `kernel: …`)
  and states what is proven/checked before and after.
- **Trusted core touch is called out explicitly.** Changes to
  `kernel/*.shard`, `src/*.rs`, or any `mod.req.shard` / reviewed
  spec-vocab file mean the audited surface grew or shifted. Changes
  to impls, proofs, or `examples/` do not.
- **Sources are canonical-format.** Run shardfmt on touched files;
  the formatter's gate guarantees it cannot change what a file
  parses to (`tools/shardfmt/mod.req.shard` is the contract).
- **REVISIT entries are first-class.** Every design decision under
  uncertainty has an entry with the "what was chosen", "why now",
  and "revisit when" triad. The README's roadmap section is a
  view-by-priority over REVISIT's "revisit when" hooks.

## Status

- **Substrate:** the self-hosted tower (eval.rs → eval.shard →
  check.shard) running a proof system with structural + two-step +
  well-founded induction, cut, bounded enumeration, four theory
  backends (LIA, eqdec, ord, farkas), a mode-aware module system with
  opaque interfaces/types, and the requirement/fulfills/bin contract
  layer. Proven artifacts beyond the M3 capstone: snake_game_2
  (interactive binary, 10 boundary requirements met) and shardfmt
  (the formatter the tree is canonicalized with).
- **Code size (NCNB):**
  - `kernel/*.shard`: **5,582** across 18 files — the checking core
    (term/reduce/proof/module/checker/check + the four theories +
    stdlib) is **2,582**; the self-hosted front-end + engine
    (reader/proof_reader/desugar/loader/driver/eval) is **2,478**;
    untrusted failure diagnostics (trace) is **522**.
  - Rust trusted-by-review: **1,013** across
    `ast.rs` + `eval.rs` + `load.rs` + `prim.rs` + `bin/eval.rs`
    (check.rs deleted — the checker is self-hosted, `eval` just runs it).
  - (Plus ~450 NCNB of Rust tests in `lib.rs` that are not part of the
    trusted surface — the legacy kernel-mirror suite and its builders
    were deleted in favour of the self-hosted corpus.)
- **Next:** see Roadmap above. The "Big-ticket mandate items" list
  is the gating set for getting v2 to the TRANSFER north-star bar
  (schedule-refinement proofs over a partitioned compute graph).
