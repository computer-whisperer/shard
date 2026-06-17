# shard — design intent

This is the "why" behind the project. The README is the front door and the
status; `LANGUAGE.md` specifies what runs today; this document records the
shape of the whole idea so the pieces make sense as one thing.

The one-sentence version: **build software the way verified compilers build
machine code — as a chain of separately-proven refinements — but for
general-purpose programs, top to bottom, with the program itself kept as a
data structure the tools can reason about and compile to bare metal.**


## 1. Programs are data

shard's defining choice: a function, a data structure, a whole application is
an **in-memory compute structure** — an algebraic value (the object AST). The
proof checker, the refinement tools, and the compiler all *inspect, transform,
and reason about* shard programs as ordinary data. This is a **deep embedding**
/ two-level reflective system; the closest relatives are ACL2 (a language plus
a separate prover that reasons about its definitions) and the LCF lineage
(only valid results can be constructed). It is deliberately *not* the
dependent-types path (Coq/Lean/Agda, where proofs are programs); we keep two
languages — an ordinary total first-order evaluator, and a separate recursive
proof checker — so neither has to carry the other's complexity.

Why this matters: a pure algorithm expressed as data is **portable,
introspectable, and provable in memory**. You can copy it into a new project,
transform it, and prove things about the transformation, without dragging an
opaque language runtime along. Contrast Rust/C, where reasoning about a program
means reasoning about a large, informal "C virtual machine"; here the
machine model is small and itself written in the same provable language (§4).

**Build-time, not runtime.** "Programs are data" is a power the *tooling* uses
at build time. It is **not** a capability a shipped application gets — see §5.


## 2. Refinement is the spine

The mechanism is one relation, **refinement** (`⊑`, from Dijkstra/Back/Morgan):
`impl ⊑ spec` means "impl satisfies everything spec requires." It is
transitive, so you chain

```
requirement  ⊑  clear implementation  ⊑  efficient implementation  ⊑  … ⊑  machine code
```

and prove **each link** separately. The provable claim for a link is
`∀ x. impl(x) = spec(x)`; a plain unit test is the degenerate case where `spec`
≡ "returns true."

The reframing this buys: **requirements→code and code→machine-code are the same
operation at different altitudes.** Verified compilers (CompCert, CakeML)
already do the lower half this way. The bet here is to do the *whole* tower for
general-purpose software.


## 3. Lowering is a proven artifact, not a smart compiler

Languages like Rust and C **tangle two different things**: *what the algorithm
is* and *how to fulfill it efficiently*. Compilers are hard — and hard to
trust — precisely because there is no 1:1 map between those two; a "sufficiently
smart compiler" has to *guess* the lowering.

shard's alternative: make the lowering an **explicit, separately-proven
artifact**. Refine the algorithm down to a shard form that maps **1:1 onto the
target** — think shard written in a tight subset that corresponds to assembly,
not to C — and then translate that form 1:1 to the actual target. The hard,
creative, untrusted work (choosing an efficient lowering) is done by a human or
an LLM; the *correctness* of each lowering is a proof the kernel checks.

Two consequences worth stating outright:

- **The compiler is outside the trusted core.** It belongs to the same regime
  as `tools/prove` and `tools/shardfmt`: untrusted machinery that makes
  development palatable. Today's chain (`tools/lower` → `tools/codegen` → C)
  is *functionally* inside only because it does not yet emit refinement
  proofs — its correctness is attested differentially (loud-error gates,
  byte-identical corpus runs, and the standing rule that the compiled chain
  is never the soundness authority). The graduation path is
  **differentially-gated accelerator → proof-emitting transformer**: the
  compiler hands back the artifact *plus* a checked refinement tying it to
  the source, and the only residual trust leaf is that the hardware conforms
  to the modeled target semantics (§4).
- **Performant representations need zero kernel or compiler features.**
  A packed string, an in-place buffer algorithm, an arena: each is a
  *lower-level shard program* (over the `Word`/`Bytes`/`Mem` vocabulary —
  see `std/mem.shard` for the in-place seed), proven to refine its
  high-level form, then lowered 1:1. Lining up the refinement that runs
  efficiently on the available machine is **app-level work in the untrusted
  regime** — never a kernel representation, never a smart-compiler guess.
  (Optional surface sugar is the lone exception, and it is sugar only.)


## 4. The machine is modeled in shard

Because programs are data and the evaluator is small, the **target itself can be
modeled in shard**: write a wasm (or x86) interpreter as an ordinary shard
program. Then a piece of emitted wasm is just more object data, and proving
`wasm_program ≡ spec` is the *same equational reasoning* as proving anything
else — you run the wasm on the shard-written interpreter and chain the
equivalence up to the requirement.

This is not hypothetical. The **v1 pilot's M4** result: a hand-written **wasm**
reverse, run on a structured-wasm interpreter written in the object language,
**proven equal to functional `rev` for all inputs** as the composed chain
`wasm ⊑ rev_loop ⊑ rev`. Tellingly, it needed *no new inference rule* — only a
performance fix — because the wasm was just another program to reduce.
Verification reaching the metal falls out of the architecture, rather than
requiring a separate verified-compiler effort.


## 5. Serious applications compile to bare metal

A serious shard application is **compiled, not interpreted**. The output is a
**standalone binary with no runtime, no GC, no reflection, no interpreter, no
kernel sidecar.** The snake demo (`examples/snake_game/`) is the litmus: it
should reduce to a bare x86 executable that is just its `step` function over
in-register/in-memory state, plus IO syscalls — nothing else.

This is the crucial counterweight to §1. "Programs are data" is a *build-time*
power. **We cannot assume a running application can manipulate compute
structures at runtime** — it has no runtime to do so with. The consequence is a
hard constraint on language design:

> A feature is admissible only if it **compiles fully away**.

The cautionary case is **lambdas / first-class closures**: a closure is a heap
environment plus an indirect call — that *is* a runtime. So closures may be
added only if they defunctionalize / inline / monomorphize away completely,
leaving no closure machinery in the binary. Likewise runtime `eval` or runtime
reflection of a program's own code: forbidden in code destined for compilation,
because the bare binary has none of it. Processing an `Expr` *value* (a tagged
tree) is fine and compiles like any data structure — what is forbidden is
**code-as-a-runtime-value**.


## 6. The substrate: shard, narrow, full, and a disposable bootstrap

(See `LANGUAGE.md` §11 for the normative version.)

- **shard** — the language.
- **narrow shard** — the minimal subset the **Rust bootstrap** parses and
  evaluates. The engine, kernel, checker, and parser are written in it. It is
  the bootstrap *floor*, and it grows reluctantly.
- **full shard** — the richer language the **shard engine** implements (the
  engine itself being written in narrow). New features land here first; they
  are pushed down into the Rust narrow backend only when genuinely needed.

There is **no full→narrow lowering** and **no certificate scheme** — narrow is
not a compilation target, it is the floor we bootstrap from. The Rust in
`rust_bootstrap/` is **scaffolding, not the destination**: it hosts shard only
until shard can compile itself, at which point the whole directory is deleted
and shard stands alone. The eventual compile story is full shard straight to a
machine target (§3).

**Self-hosting status.** The front-end has moved into shard: the s-expression
reader and module parser (`kernel/reader.shard`) are validated byte-for-byte
against the Rust loader, and an environment-machine evaluator makes them fast
enough to use. Every driver — `check`, `run`, and `eval` — now parses
user/target code through this shard reader (`build_module` / `parse_expr`); the
Rust loader (`load.rs`) survives only as (a) the **bootstrap floor** — it parses
the kernel and the reader toolchain itself into the VM, since the reader cannot
parse itself — and (b) the **reference oracle** the parse/module/claims
differential harnesses validate the shard reader against. `eval` is now its own
clean entrypoint: the kernel's executable `main` lives in `kernel/eval.shard`
(a direct-style `World -> World` program that reads the referenced files itself
and evaluates them), and the `eval` binary is a pure passthrough — it bootstraps
the toolchain + entrypoint and runs `(main world)`, with no eval logic in Rust.
I/O is done by `extern` calls (uninterpreted in proofs, performed by the host
handler), with Rust only ferrying bytes. The `check` binary is the legacy
orchestrator, kept until proof-checking is likewise a shard app run on this
executor. The remaining cord-cutter is the shard→machine compiler.


## 7. Identity is structural, and soundness depends on it

The kernel is a recursive checker in the LCF lineage (§1): a proof step is
sound only if every name in it denotes what the checker thinks it does. Two
layers read the same program — the **reducer**, which unfolds function calls,
and the **theory backends** (farkas/lia/ord/eqdec), which recognize interpreted
symbols (`le`, `lt`, `int_eq`, `+`, `True`, …) and reason about them as
arithmetic. If those two layers can disagree about what a name means, the
checker is unsound.

They could, while names were bare strings. A user definition named `le`
shadowed the built-in: the reducer dispatched the *user's* function, while a
backend still matched the string `le` and read the call as the linear relation
`≤`. A constant-`True` user `le` makes the premise `(le 5 3) = True`
reducer-true while farkas reads it as the false fact `3 − 5 ≥ 0` — and from a
contradiction, anything follows (`0 = 1`).

The fix is to make a name an **identity, not a string**. Every function / type
/ constructor name is a `QName` = `(module-path, local-name)`, where the
module-path is a hierarchical list (Rust's module tree: `std::list` is
`(std list)`, the built-in crate is `core`) assigned by the **loader** from the
import graph. A source file cannot write a module-path, so a user definition can
never forge a `core` identity. The backends admit an interpreted symbol only at
its `core` identity; a user's same-named definition resolves to its own
module-path and is, to a backend, an opaque atom. The reducer and the backends
can no longer be driven to disagree.

This is **object-language only**. The trusted Rust bootstrap (§6) is untouched —
identity is a property the kernel, itself written in shard, enforces on the
programs it checks.

**Definitions are admitted, not assumed — the totality decision (2026-06-12).**
The same bug class has a second instance: `unfold` treats every `fn` body as a
total function's defining equation, so the `fn` form is the largest axiom
generator in the language — and with no admissibility check, a non-terminating
definition like `(fn liar ((n Int)) Int (+ (liar n) 1))` mints the inconsistent
theorem `liar 0 = liar 0 + 1`, from which farkas derives `0 = 1` (issue #1, a
live exploit until the gate lands). The ratified design:

- **One primitive: admission by nonnegative-Int measure descent.** A recursive
  definition enters the logic only if every recursive call, under its path
  condition, strictly decreases an Int measure that stays ≥ 0. This is the
  same trust floor as admitting `Int` itself was — well-foundedness of
  bounded integer descent has the external pedigree we demand of axioms —
  and unlike a structural-only gate it does not push executable loops onto
  unary Peano fuel.
- **Discover offline, verify at check time** (refined 2026-06-17). The
  descent recognizer is kept *out of the trust path*: `admit` is the offline
  classifier/suggester (the `tools/prove` of totality), and the check-time
  gate only *verifies* an explicit `(measure …)` clause — it never searches
  for a descent. Two forms: structural `(measure (struct ARG))`, where the
  checker verifies the named argument is a strict subterm at each recursive
  call (no proof needed); and numeric `(measure E proofs…)`, where the kernel
  emits the decrease/nonnegativity facts per call site as ordinary claims,
  discharged in the untrusted regime (the prover already enumerates farkas
  certificates for exactly this shape). A recognizer *inside* the gate would
  be TCB — its bugs would be soundness bugs — so discovery stays advisory.
  See `TOTALITY.md` for the full system.
- **No partiality, anywhere.** There is no `partial-fn` caste and no codata.
  Genuinely unbounded processes — the interpreter, reducer fixpoint loops,
  event loops — take an Int fuel/budget parameter and are measure-admitted
  on it; the reference semantics is a *clocked* big-step semantics (CakeML
  precedent), exhaustion is loud refusal (the sound direction for a
  checker), and the unfueled "ideal" meaning is recoverable as ∃-fuel
  theorems. Every defining equation in the system is a theorem.


## 8. Why now: the generate / check asymmetry

The refinement-derivation programs of the 1980s–2000s (Specware/Kestrel)
stalled because a *human* writing every refinement and every proof was brutally
laborious. LLMs invert that economics:

> Code is cheap; coherent, expressive, **proven** requirements are the scarce
> resource. An LLM is good at *proposing* an implementation or a proof; a small
> trusted checker is exactly what makes that untrusted output *trustworthy*.

So the architecture is split on purpose: **generation is cheap and untrusted**
(an LLM, or later an SMT solver, proposes refinements and proofs);
**checking is small and trusted** (the kernel). The kernel/search split is the
product thesis, not just hygiene — proof *search* is swappable, the *checker*
is not.

The boundary is worth drawing precisely. The trusted core's charter is
**exactly three things**: how to parse shard, *one* reference way to execute
the resulting ASTs, and the logic for establishing formal proofs relating
shard expressions to each other. Hardware is not in the charter — the kernel
never cares how a program is executed efficiently, only what it means.
Everything else — the prover, the formatter, the compiler (§3), and the
refinement passes to come — is one untrusted tier, whose outputs are either
kernel-checked or differentially gated.

**The assumable base is a closed list — the contraction decision
(2026-06-12).** What loading a file lets you *assume* is exactly: inductive
datatypes, `Int` with linear arithmetic (the deliberate v2 layer-up from
Peano), nonnegative-Int well-founded descent (§7), and extern `World` axioms
surfaced at the bin ledger. Axioms are reserved for facts with **external
pedigree** — centuries of vetting back integer arithmetic; nothing backs a
hand-written axiom about a data structure we invented last week, and we have
shipped or nearly shipped false ones twice (the Word `/`+`mod` mixed-pair
axiom; std/bytes `of_list_id` unguarded). Consequently the `Word` and `Bytes`
kernel formers are **revoked**: both are conservative constructions over the
base (`Bytes` already carries its byte-list model inside its canonical value)
and are slated for demotion to ordinary `std` modules — opaque `sig type`s
over `Int`/`(List Int)` whose law families are *proven*, turning today's ten
std axioms into theorems. Facts about defined things are auditable inside the
system; facts about opaque primitives are pure trust. The end state: `std`
is axiom-free, layered proof snowballing on the closed base, and `(axiom …)`
outside the base is a corpus-gate violation rather than a convention.


## 9. Known hard parts (dragons)

Flagged so we don't paint ourselves into a corner:

- **Data refinement is the real dragon.** Proving `naive(x) = optimized(x)`
  where both use the *same* representation (the O(n²)→O(n) kind) is tractable.
  Proving a lowering that *changes the data representation* (abstract set →
  sorted array → packed buffer → registers) is where it gets genuinely hard.
- **Efficiency is not a correctness property.** `∀ x. impl(x) = spec(x)` proves
  a lowering *correct*, never *fast*. The framework gatekeeps correctness;
  choosing an efficient lowering is the engineer's/LLM's job. A cost /
  resource-bound layer is future work.
- **Spec adequacy.** "Proven" means "satisfies the stated requirements," not
  "correct" in the intuitive sense — a weak spec admits garbage that still
  passes. The answer is accretion (pile up discriminating requirements, prove
  each), but spec quality stays a human responsibility — doubly so for
  LLM-written code.
- **The spec boundary.** Portability works only if a requirement is statable
  without dragging in the whole world. *What a requirement may mention* (its
  dependency surface) is the axis the long-term vision lives or dies on.


## See also

- `../README.md` — front door, quick start, current feature checklist.
- `LANGUAGE.md` — normative spec of narrow shard (syntax, semantics, the
  narrow/full distinction).
- `BOUNDARIES.md` — modeling external systems (extern + axiom; the direct-style
  World/extern I/O the `eval` runner realizes).
- `REVISIT.md` — the design-decision ledger: every choice and when to revisit.
- `TOTALITY.md` — the measure-descent admissibility system: the verify-don't-search
  architecture, the single-fn and mutual gates, cycle-readiness, the TCB.
- `archive/TRANSFER.md` — the v1→v2 handoff: premise, lessons, what changed.
