shard builds — BUILD.md
=======================

STATUS: DRAFT (2026-07-11) — the scope ledger for the build story: how
anyone — a person, a model, or a host program — compiles shard, across
the whole spectrum from "one declaration to ELF, zero configuration"
to "every stage hand-spelled, without reinventing the build system."
This is the orchestration layer ABOVE the lowering form; it owns
products, profiles, artifact authorship, synthesis, and the driver.

**Ownership boundaries** (verified against the ledgers, 2026-07-11 —
this file cites, it does not restate):

- **ISA.md §4** owns the compile-script trust architecture: quotation
  without eval, generate-and-check (scripts emit decls + claims; the
  kernel checks the emitted things; the script itself never needs
  verification), kernel-as-a-library, and user-writable linkers with
  no distinguished trusted one. Every synthesis mechanism in this
  ledger inherits that architecture verbatim.
- **LOWERING.md** owns the lowered-conformance form — statements,
  certs, the gates, plan values — and records the lib-build arc
  (§6ag): the target-agnostic `(lib …)` decl, premise percolation and
  the accepts gate, driver-derived plans with synthesized vectors
  (hand vectors as the curated fallback), and the explicit deferral
  this ledger picks up ("how mod.build.shard is structured under the
  multi-target + temp-scripting reality … expected resolution is that
  driver-derivation eats most of mod.build's body, gates move into
  the driver, and the residue is a small per-module override").
- **X86.md §19–§22** owns bin packaging: the bin-as-one-export-lib
  admission, the glue contract, BINELF, the on-silicon engine gate.
- **MEMORY.md** owns representation and memory management; its open
  D1 (the class-assignment surface) LANDS HERE, in profiles (§3).

**Recovered original intent (USER, 2026-07-11 — stated at arc opening;
never previously written down in this form).** mod.build.shard was
always meant as an *optional metaprogram*: it returns shard AST + any
proofs needed + glue; the compiler has a default stand-in approach;
the metaprogram is free to call into compiler libraries itself as it
wants — the black-box interface by which explicitly-described modules
interact with implicitly-compiled ones. ISA.md §4 ratified the trust
shape of such scripts; the build-layer role was the part that never
got recorded, and the surviving mod.build files read as MVP plumbing
because of it. Retrospective rulings attached: full-plan authorship
is far too verbose for common use (this ledger's per-stage trichotomy
is the unbundling); the pinned-twin form (`mem.x86.shard` style) is
genuinely cleaner for many scenarios; the stack definition was
deliberately fuzzy — implementation may clarify it in any direction.

**Further user rulings on record:** rip-up license ("always willing
to rip up and redo if we have a better idea" — §8 uses it); the four
loose ends of §1 are the needs statement; the expectation that the
right answer looks unlike any existing language's build system.


## 1. The needs

The four loose ends named at arc opening, plus the standing demands:

1. **Multi-target management.** The mod.build era hand-branches
   `(fn build ((t Target)) Plan)` per target and hand-maintains a
   variant zoo (std/mem: mem.wasm / mem.wasm2 / mem.x86); the lib era
   manages targets by file suffix and per-shape shell script (18
   `lowbuild_*.sh` variants at count time).
2. **Literal lowered implementations need a first-class home.**
   Hand-writing the wasm/x86 twin is the honest form for some modules
   (the mem arc), but shipping one today requires hand-synthesizing
   the mod.build plumbing around it.
3. **A steering surface for the memory story.** Try-stack vs
   use-heap, copy-vs-share points, rep selection: no clean place to
   push the compiler (MEMORY.md D1). Today's only steering surface is
   the accepts clause's premise families.
4. **mod.req's multiple-impls freedom has no consumer.** The module
   system was built so N implementations can satisfy one requirement
   surface; nothing exercises it.

Plus: **generics** (a generic module's lowering inherently needs
size/stride/class information for its type arguments — from
somewhere); **synthesis from patterns** (metaprogramming that emits
shard); **the embedding consumer** (a host program invoking the
toolchain as a library, jit-like — the README's IR ambition); and
**the zero-config default** (the boring-library vision of LOWERING.md
§6ag, extended to bins: no linking configuration, ever, because the
platform model owns the whole artifact).


## 2. The frame: the build graph is a subgraph of the proof graph

Every other build system describes *recipes* — how to produce
artifacts from files. shard already redefined compiling (OVERVIEW
§3–§4) as proving your way to a model-vocabulary program and
transliterating 1:1. So a shard build is not a recipe: it **names
which proven-refinement path to realize, on which target** — spec
impl → rep-swapped impl → target twin → bytes, every edge a checked
refinement. Derivation machinery exists to *fill in missing edges*,
not to orchestrate steps.

This is where loose end 4 resolves: the clear implementation, the
rep-swapped implementation, and each target twin are all just
**conformant implementations of the same mod.req surface**, tagged by
what they are. Multiple targets stop being a branching problem and
become sibling nodes in the conformance graph. Explicitly-described
and implicitly-compiled modules meet at the same boundary — a
synthesized or hand-written node must fulfill the (possibly
instantiated) requirement surface exactly as a derived one must.
That conformance boundary is the "black-box interface" of the
original intent, realized by the mechanism that was waiting for it.


## 3. Products: decl × profile

The split is meaning vs. machine realization — the trichotomy applied
to the build story itself:

- **The decl** — `(lib …)` / `(bin …)` — stays exactly what §6ag made
  it: the *semantic* contract. Exports, accepts (the percolated
  premise surface — a trust statement, not a build knob), requires.
  Deliberately target-agnostic; the target arrives at build
  invocation, never in the declaration.
- **The profile** — an ordinary shard VALUE, not a config dialect:
  the target set; memory-class and rep assignments per type/decl
  (MEMORY.md D1's home — declared, proof-backed, never inferred);
  variant selection through the conformance graph (which impl chain
  realizes each export); stack budget; link/layout choices at the
  artifact edge. Everything in a profile is *realization*: changing a
  profile can never change what a product means, only how (and
  whether) the proof closes and how fast the artifact runs — the
  safety inversion, applied to configuration.

A **product** is (decl × profile). The **default profile** is what
makes tier 0 real: counted-heap classes where no proofs are offered,
derived twins, derived layout from the platform model. A repo (or a
host program) states its products; everything else is derived or
pinned per §4. Residence and override structure are D1 (§10).


## 4. Stage artifacts and the authorship trichotomy

Between a product and its final bytes lies a fixed set of **stage
artifacts** — the twin (model-vocabulary fns + certs), glue, image —
each with a canonical file identity (naming: D4). Every stage
artifact has exactly three possible authors, and **the gates do not
care which** (the safety inversion makes authorship ceremony-free):

- **PIN** — a literal file, present at its canonical location. The
  driver verifies it through the same gates it would apply to its own
  output. This is the first-class home for hand-written twins (loose
  end 2): write `NAME.x86.shard`, and nothing else. Pins are not a
  fallback tier — for mem-arc-shaped modules they are the genuinely
  cleaner form (user ruling).
- **DERIVE** — the repo generators fill the edge. wasmgen and x86gen
  hold no privileged position in this frame: they are repo-owned
  metaprograms occupying the derive slot, gated like everything else.
- **SYNTHESIZE** — a module-supplied metaprogram produces the
  artifact (§5).

One boundary is absolute: **metaprograms return content — AST, proofs,
glue — never packaging.** Plans, manifests, and vector synthesis are
derived unconditionally (LOWERING.md §6ag landed this; hand vectors
remain the recorded curated-build fallback). Packaging-as-user-
artifact was the mod.build-era mistake, and it is what made the
original metaprogram vision too verbose for common use. A corollary
worth stating: because custom and default call the same compiler
libraries (ISA.md §4's kernel-as-a-library), a metaprogram that wants
to be 95% default simply calls the deriver and edits the result —
custom and default are one code path invoked at different frontiers.


## 5. The metaprogram tier (synthesis)

The recovered original intent, unbundled into its durable form:

- A synthesis hook is an ordinary shard function owned by the module
  it serves ("modules know *how* to specialize themselves; profiles
  know *what for*"), receiving a **BuildCtx** — target, profile
  residue relevant to the module, and for generics the resolved reps
  of the type arguments (§6). Contents of BuildCtx are
  implementation-refined (D7), per the fuzzy-stack license.
- Hooks attach per stage (which stages are hookable: D5), replacing
  exactly one derivation edge; everything around the hook stays
  derived.
- Execution rides the existing build-time substrate — meta/invoke's
  `evm_call_pure` on kernel/evm — so no new engine appears. Purity
  buys reproducible builds outright: content addressing hashes the
  metaprogram with its inputs, and there is no ambient anything.
- Trust is ISA.md §4 verbatim: generation is untrusted regardless of
  author; outputs land as ordinary decls, claims, and artifacts that
  the kernel and the gates check. The product asymmetry covers
  metaprogramming with no new argument needed.
- Dissolution holds: synthesis is build-time power. Nothing about a
  hook survives into the artifact (OVERVIEW §5's admissibility law
  applies without a carve-out).

**The tier ladder** (the verbosity resolution):

- **Tier 0 — common case:** decl + default profile. Everything
  derived; no metaprogramming visible anywhere.
- **Tier 1 — pins:** hand-written twin files; hand vectors.
- **Tier 2 — per-stage hooks:** the generic module's instantiator,
  pattern expanders, custom generators.
- **Tier 3 — full-freedom metaprogram:** returns content wholesale,
  calls compiler libraries at will. Rare by design, legitimate by
  ISA.md §4; packaging still derived.


## 6. Generics

The question "where do size and stride come from" has a closed answer
once profiles exist:

1. The profile assigns representations (class / size / stride) to
   concrete types — MEMORY.md's declared-not-inferred surface.
2. The driver walks a product's closure and discovers the generic
   instantiations it demands (`(Map U32)` in a bin's closure requests
   `Map` at `U32`).
3. Each instantiation is filled by the module's synthesis hook — or
   the repo-owned default monomorphizer, hook-over-derive as
   everywhere — with BuildCtx carrying the resolved reps of the type
   arguments.
4. The synthesized instantiation must fulfill the module's
   requirement surface instantiated at those types. Conformance is
   the interface; the engine composes at surfaces only.

**Certs for synthesized code** (D6) — three mechanisms, likely used
per-stage: (i) the hook emits proofs alongside the AST (proofs are
data; the DSL parses to native structures); (ii) tools/prove solves
the obligations post-hoc (fragment-class certs are machine-solvable
today); (iii) **cert schemas** instantiated with the code — the
records arc is the existence proof (loader-level expansion generating
a law family with machine proofs, proof-neutrality validated).
Expected split: schemas for the regular shapes, prove for the
residue, hand emission as the power tier.


## 7. The driver

One shard program (working name `tools/build`), replacing the script
zoo:

- Walks the declared products; for each stage artifact: verify the
  pin, or call the hook, or derive — then gate. The gates are the
  driver's spine, inherited intact: regen (for derived artifacts),
  schema, kernel, accepts, byte-tie + manifest, engine.
- **The CLI is a thin skin over a library API** — module/decl in,
  bytes + certs out. The library surface is the embedding consumer
  (the README's IR ambition) served by construction; a jit-like host
  calls the same entry the CLI does. Compile latency is a first-class
  criterion on this path (the evaluator-promotion discipline
  applies).
- **Incrementality by content addressing**: stage artifacts carry
  std/sha256 digests over their inputs (the canon arc's content
  addressing; the bin/ stamp discipline generalized). Fresh nodes
  skip; the byte-tie is what makes a cached artifact trustworthy.
- Shell survives only where the world is: the V8 runner, the ELF
  execution leg. Everything else that lives in the 18 scripts —
  sequencing, gating, temp plumbing — moves into the driver.
- The driver is untrusted machinery (ISA.md §4's authority model):
  soundness-authority runs replay through the standard pipeline
  exactly as today.


## 8. What this rips up, what it keeps

Ripped up (under the standing license):

- `std/mem/mod.build.shard`, `std/str/mod.build.shard`, and their
  per-module `lowbuild.sh` — the modules migrate to pinned twins
  under the driver; the variant zoo gets profile-keyed naming (D4).
- The `examples/lowbuild_*.sh` zoo — collapsed into the driver, save
  the world-edge legs.
- Plan-as-user-artifact, everywhere and permanently.

Kept:

- The decls and the accepts gate (§6ag) unchanged.
- The twin files — promoted from era-remnant to first-class pins.
- The six gates, verbatim, now driver-hosted.
- tools/lowbuild's internals — repurposed as the driver's plan stage.
- Hand vectors as the curated fallback (already recorded in
  LOWERING.md §6ag).


## 9. The rung ladder (sketch — sliced at ratification)

1. **Driver skeleton:** products + PIN/DERIVE over the existing lib
   pipeline, both targets; the generic scripts collapse; corpus pins
   move to driver invocations. No new semantics — pure consolidation.
2. **The mod.build funeral:** std/mem and std/str migrate to pinned
   twins under the driver; both mod.build.shard files and per-module
   scripts deleted; variant naming (D4) decided here with real cases.
3. **Profiles v1:** target sets + variant selection + the first
   memory-class surface, coordinated with MEMORY.md's rung ladder
   (D1 resolved here).
4. **Synthesis hooks v1:** BuildCtx + one real hook — either a
   records-style pattern expander or the first generic instantiator;
   the cert-schema mechanism (D6) proves out here.
5. **Generics pilot:** one generic module instantiated at N types
   through the profile-resolved rep path, conformance-gated.
6. **The library API:** the embedding consumer pilot — a host program
   compiling a shard module in-process (module in, bytes + certs
   out).


## 10. Decision points

- **D1 — profile residence.** Sibling form beside the decl, separate
  file, or repo-level defaults with an override chain. Propose at
  rung 3 with real products in hand.
- **D2 — pin grain.** Lean: the twin file (fns + certs together) is
  the pin unit; per-fn pinning inside a derived twin is a later
  refinement if demanded.
- **D3 — vector pins.** RESOLVED by prior record: synthesized by
  default, hand vectors as the curated fallback (LOWERING.md §6ag).
- **D4 — variant naming.** `.wasm.shard`/`.x86.shard` suffixes vs
  profile-keyed names once one module has two same-target variants
  (`mem.wasm2.shard` is the standing warning). Decide at rung 2.
- **D5 — hookable stages and declaration site.** Lean: hooks are
  module-owned functions; profile provides context only. Which stages
  are hookable (spec-source synthesis, twin, glue?) — rung 4.
- **D6 — cert story for synthesized code.** Schemas / prove / emit,
  per §6. Rung 4.
- **D7 — BuildCtx contents and the metaprogram return surface.**
  Implementation-refined by explicit user license; any direction.
- **D8 — driver naming and residence.** `tools/build` vs graduation
  into meta/ once the library API stabilizes (the hygiene-pass rule:
  graduate when a second consumer exists).


## 11. Non-goals, stated once

No config dialect — profiles are shard values. No distinguished
trusted linker — ISA.md §4's ruling stands; linking is user-writable
script logic and all outputs are checked. No privileged generators —
derive is a slot, not a status. No recipe language — the conformance
graph plus derivation is the whole vocabulary. And the driver never
becomes the soundness authority — authoritative replay stays on the
standard pipeline, exactly as the compiled chain's standing rule.
