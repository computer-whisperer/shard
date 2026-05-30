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
reader and module parser (`tools/reader.shard`) are validated byte-for-byte
against the Rust loader, and an environment-machine evaluator makes them fast
enough to use. `eval` now exists as a standalone shard CLI app
(`examples/cli/eval_app.shard`, driven by `check cli`) that reads files and
evaluates them, with Rust only ferrying bytes. The remaining cord-cutter is the
shard→machine compiler.


## 7. Why now: the generate / check asymmetry

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


## 8. Known hard parts (dragons)

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
- `BOUNDARIES.md` — modeling external systems (extern + axiom; the effect-as-
  data mechanism the `check cli` loop realizes).
- `REVISIT.md` — the design-decision ledger: every choice and when to revisit.
- `archive/TRANSFER.md` — the v1→v2 handoff: premise, lessons, what changed.
