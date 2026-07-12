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

**Second-round ruling (USER, 2026-07-11): mod.build.shard is the
configuration home, not just the metaprogram slot.** mod.req.shard
establishes the req-tied targets and durable goals; mod.build.shard
is the natural place for profiles, hook registrations, and the
explicit binding of pinned implementation files (`.x86.shard` etc.)
to owned exports of the mod.req surface — bins, libs, opaque types
and functions. The driving motive is **filesystem-structure
independence**: a build entry imports `meta/build` (or similar) and
emits `BuildProfile` values from ordinary functions
(`bin_helloworld_wasm()`-style) — no kernel-backed syntax, boring
shard with a standard symbol and setup, easy for an aftermarket
build chain to co-opt, and droppable entirely when a consumer only
interprets or manipulates shard programmatically. Consequences
threaded through §3, §4, §5, §7; D1/D4/D5 updated in §10.


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
derived twins, derived layout from the platform model.

**Residence (RESOLVED, second-round ruling):** profiles are ordinary
shard values defined in the module's **mod.build.shard** — imported
from `meta/build`'s vocabulary and returned by standard symbols —
or constructed directly by a host program. `BuildProfile` values are
the single interface the driver consumes (§7); mod.build.shard, the
host program, and the default deriver are three producers of the
same values, one consumer path. Two laws guard the common case:
**absence is inviolable** — no mod.build.shard means tier 0, all
defaults, and the boring-library vision never regresses into a
mandatory third file; and **build entries are outside the check
closure** — they import models and targets, are loaded only by the
driver (the §6o precedent made law), and never burden proof
checking.


## 4. Stage artifacts and the authorship trichotomy

Between a product and its final bytes lies a fixed set of **stage
artifacts** — the twin (model-vocabulary fns + certs), glue, image —
each with a canonical file identity (naming: D4). Every stage
artifact has exactly three possible authors, and **the gates do not
care which** (the safety inversion makes authorship ceremony-free):

- **PIN** — a hand-written artifact, **bound by declaration, not
  discovered by path**: the module's build entry imports the twin
  file (ordinary `(import "mem.x86.shard")`) and binds its values by
  qname to owned exports of the mod.req surface — bins, libs, opaque
  types and functions alike. The driver's only filesystem coupling is
  the import resolver the module system already owns; file naming
  (`.x86.shard`) is human convention, never load-bearing. The driver
  verifies pinned artifacts through the same gates it applies to its
  own output. Pins are not a fallback tier — for mem-arc-shaped
  modules they are the genuinely cleaner form (user ruling). Binding
  opaque-type impls this way is the conformance-graph path selection
  of §2 stated as data — the multi-impl consumer in full generality.
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
  know *what for*") and **registered in its mod.build.shard** (the
  configuration home), receiving a **BuildCtx** — target, profile
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
- **The CLI is a thin skin over a library API whose sole input
  vocabulary is `BuildProfile` values** (meta/build). The CLI path
  loads build entries and calls their standard symbols; a jit-like
  host constructs the same values directly and calls the same
  library entry — the embedding consumer (the README's IR ambition)
  served with zero special-casing, and the drop-entirely property
  free: a purely programmatic consumer never touches mod.build,
  meta/build's vocabulary, or a filesystem layout at all. Compile
  latency is a first-class criterion on this path (the
  evaluator-promotion discipline applies).
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

- The **bodies** of `std/mem/mod.build.shard` and
  `std/str/mod.build.shard` (plan assembly, Target-branching) and
  their per-module `lowbuild.sh`. The *slot* is re-founded, not
  buried: mod.build.shard survives with the new charter —
  configuration home (profiles, hook registrations, pin bindings)
  per the second-round ruling. std/mem's binding of its hand twins
  becomes the first build entry of the new era.
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
2. **The mod.build re-founding:** std/mem and std/str migrate to the
   new-charter build entries — pin bindings replacing plan-assembly
   bodies; per-module scripts deleted; meta/build's BuildProfile
   vocabulary and the standard-symbol shape (D1 residual) proven out
   on real cases here.
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

- **D1 — profile residence: RESOLVED (second-round ruling) —
  mod.build.shard**, as ordinary values over meta/build's vocabulary;
  hosts construct the same values directly. Residuals: the
  standard-symbol shape (one standard entry returning the product
  list — the lean, since name-pattern scanning is a stringly
  convention of the kind this ledger deletes — vs named per-product
  functions, the ruling's `bin_helloworld_wasm()` sketch); and
  product discovery at repo scope (how the driver enumerates build
  entries). Both at rung 2 with real cases.
- **D2 — pin grain.** Lean: the twin file (fns + certs together) is
  the pin unit; per-fn pinning inside a derived twin is a later
  refinement if demanded.
- **D3 — vector pins.** RESOLVED by prior record: synthesized by
  default, hand vectors as the curated fallback (LOWERING.md §6ag).
- **D4 — variant naming: DEMOTED.** Binding is explicit data in the
  build entry, so file naming is human convention and never
  load-bearing; settle the convention informally at rung 2
  (`mem.wasm2.shard` stops being a warning sign once nothing parses
  names).
- **D5 — hookable stages.** Declaration site RESOLVED (second-round
  ruling): hooks register in mod.build.shard; profile provides
  context only. Which stages are hookable (spec-source synthesis,
  twin, glue?) — rung 4.
- **D6 — cert story for synthesized code.** Schemas / prove / emit,
  per §6. Rung 4.
- **D7 — BuildCtx contents and the metaprogram return surface.**
  Implementation-refined by explicit user license; any direction.
- **D8 — driver naming and residence.** `tools/build` vs graduation
  into meta/ once the library API stabilizes (the hygiene-pass rule:
  graduate when a second consumer exists).


## 11. Slice records

**Rung 1, slice 1 — the driver skeleton, wasm lib path (2026-07-11).**
What landed:

- **meta/build** — the BuildProfile vocabulary (transparent record,
  meta/plan Target precedent; law family machine-proved, 29/0). v1
  fields: kind / target / src / out — exactly what the generic
  scripts took as arguments.
- **examples/build_products.shard** — the first products entry: the
  standard symbol `build_products : () -> (List BuildProfile)`,
  carrying the two wasm lib products (purelib, std/rng).
- **tools/build/build.shard** — the driver, TWO-PHASE because no
  subprocess extern exists (by design; plumbing moves bytes):
  `plan PRODUCTS CAPDIR EVAL CHECK...` loads the products closure at
  runtime (meta/invoke), decodes whole-or-nothing, and emits a
  deterministic `RUN <capture> <argv...>` order list; `verify
  PRODUCTS CAPDIR` reads captures + exit codes back and judges the
  six-gate ladder in-process — byte-identical regen, schema rc,
  kernel " 0 failed" tails + the LIB acceptance line, accepts rc,
  MOD=TIE hex equality + manifest, engine rc. This is §6ag's
  expected resolution ("gates move into the driver") realized for
  the wasm lib ladder: every judgment the retired script made with
  diff/grep/cut now lives in shard.
- **tools/build/build.sh** — the ONE executor: run argv, capture
  stdout+stderr, record the exit code. Zero build knowledge. Keeps
  the capture dir on failure (the debugging evidence); surfaces
  plan-phase refusals (gotcha: the first version swallowed them into
  the orders file).
- **Retired:** examples/lowbuild_lib.sh (no remaining callers);
  lowbuild_all.sh's two generic-lib entries collapse into one driver
  entry.

Gates: driver build green end-to-end (2 products, 34s serial);
negative probe (mismatched src/out product) fails regen + accepts +
bytetie with exit 1; lowbuild_all.sh fully green (26 builds);
corpus grows two targets (build_products, the driver), FAIL set
unchanged.

Gotchas for the next slices: loader imports are FILE-RELATIVE — a
products file outside the repo cannot import meta/build by relative
path (hosts construct BuildProfile values programmatically instead;
that is the intended path, not a bug). Products run serially inside
one driver invocation — cross-product parallelism (order groups are
independent) is a deliberate later refinement; lowbuild_all's outer
concurrency absorbs the cost today.

Next slices: x86 lib + ELF ladders into orders_bp/v_bp (slice 2),
the bin ladder (slice 3) — each retiring its generic script the same
way.

**Rung 1, slice 2 — the x86 lib + ELF ladders (2026-07-11).** The
products file now carries all three lib targets over purelib
(examples/build_products.shard: wasm ×2, x86, x86elf); the driver
grew `orders_lib_x86` / `orders_lib_x86elf` and their verify ladders,
gate-for-gate the retired scripts:

- **x86 lib** (six gates): x86gen regen; schema; kernel ×2; accepts
  (width-ordered coverage rides the same tool); byte-tie as XMOD/TIE
  *set equality* plus the EFF percolation line (a pure lib asserts an
  empty effect surface) plus manifest against models/x86; engine =
  `grep -v ^ARTIFACT` as a plain RUN order (the plan sieve needs no
  executor smarts), `cc` on the differential harness, and the real
  CPU replaying the plan.
- **x86elf** (three gates): regen; IMGTIE (embedded IMG ==
  cert-assembled TIEIMG, + EFF); the on-silicon run — where the
  driver's new third mode earns its keep: **`hexbin IN PREFIX OUT`
  makes the driver its own world-command toolbox** (find the hex
  line, decode, `write_file` the raw ELF), so the xxd/cut shell
  pipeline died instead of being ported. The executor gained nothing;
  the ELF is materialized by a driver self-invocation order, chmod'd
  and executed as plain RUN orders, and verify compares captured
  silicon stdout against the plan's EXPOUT bytes in-process.
- **Structure lesson (from a real defect):** the first cut of the
  x86 verify gates inlined judgment into World-threaded match towers
  and shipped a paren-imbalance; the fix was the right shape anyway —
  *pure judgment helpers over slurped text* (xtie_judge,
  imgtie_judge, elfrun_judge, eff_judge) with World-threading only in
  thin rc/slurp shells. Follow that pattern for the bin ladder.
- **Retired:** examples/lowbuild_lib_x86.sh,
  examples/lowbuild_lib_x86_elf.sh; lowbuild_all.sh entries collapse
  3→1 (24 entries remain).

Gates: driver green end-to-end (4 products, 41s serial, silicon leg
included); lowbuild_all.sh fully green; corpus FAIL set unchanged.
Duplicate-work note for a later refinement: the x86 and x86elf
products over the same SRC/OUT re-run regen and kernel checks that
sibling products already ran — per-(stage,inputs) dedup by content
address is the natural fix and rides D-acct-style digests, not new
order vocabulary.

**Rung 1, slice 3 — the bin ladder (2026-07-11).** The seven x86 bin
products join the products file (11 products total); the 212-line
lowbuild_bin_x86.sh is retired. Two architectural pieces landed with
it:

- **The plan fixpoint.** A bin's engine vectors (BNOARG/BVEC/BVEC2
  lines with expected exit + stdout) are DERIVED at build time by
  lowbuild binelf, so their orders cannot be known statically. The
  driver's plan phase now emits only orders whose capture file is
  missing, and the wrapper loops plan→execute until the plan is
  empty: round 1 runs the static ladder (which lands be.txt), round 2
  reads be.txt and emits one RUN per vector, round 3 is empty →
  verify. The same mechanism is capture-level incrementality for
  free: re-running a failed build inside a kept capture dir re-plans
  only what's missing.
- **@file argv tokens.** Engine vectors carry arbitrary-byte
  arguments (the 300-char MAXLEN pool entry); argv rides
  space-separated order lines. The executor gained exactly one
  substitution: a token `@FILE` becomes FILE's contents as ONE
  argument — byte-moving, content-blind (the driver writes the arg
  bytes at plan time via write_file). NUL is excluded by execve
  itself.

The bin verify ladder: regen; schema; kernel ×2 (the BIN acceptance
line — g_klog generalized to a wanted-prefix parameter); byte tie as
XMOD/TIE set equality + EFF + manifest (bytetie invoked with SRC for
the declared effect surface); the SURFACE gate (accepts rc + the
"(glue-covered)" marker); plan-engine (sieve/cc/CPU); binelf (IMGTIE
over be.txt — g_imgtie generalized to a capture-tag parameter — plus
hexbin/chmod); and the engine proper: be.txt re-parsed in verify,
each vector's captured exit code and stdout compared against
EXIT/OUT expectations in-process, plus the §49 pool-coverage fence
(WORLD bins: no-arg leg carries OUT, BVEC2 pool ≥ 5).

Gates: driver green end-to-end (11 products, 2m54 serial — silicon
vector runs included); negative probe (bin src against wrong OUT)
fails regen/bytetie/surface/imgtie/hexbin with exit 1;
lowbuild_all.sh fully green at 17 entries; corpus FAIL set unchanged.

**Promoted to next: cross-product parallelism.** The aggregate's wall
time doubled (1m27→2m58) because the driver entry serializes what
were previously concurrent script entries. Orders are product-indexed
and independent by construction; the wrapper can execute order groups
(by capture prefix) concurrently without any driver change. That —
plus the sibling-product dedup note above — is the natural slice 4,
ahead of rung 2's mod.build re-founding.

**Rung 1, slice 4 — the parallel executor (2026-07-11).** Scoped by a
user ruling: near-term speedup is welcome, but a TRUE parallelism
scheme for shard is coming within weeks — so wrapper concurrency is
admitted only because it adds no design surface, and it is
**scaffolding with a named dissolution path**: when shard's own
parallelism arc lands, the executor's concurrency dissolves into it.
What changed: the wrapper partitions each round's orders by product
(the `<i>.` capture prefix) and runs the groups concurrently —
order preserved within a group (in-product dependencies), a barrier
per round (the fixpoint needs this round's captures). Order grammar,
driver, and verify untouched; ~15 lines of shell. The
sibling-product stage dedup half of the original slice sketch IS
design surface (capture-naming semantics) and is DEFERRED to the
content-addressed incrementality story. Gates: 11 products
2m54→1m03; the aggregate 2m58→1m17 (under the pre-driver 1m27
baseline); negative probe through the parallel path fails both
concurrent products with exit 1; corpus untouched by construction
(no corpus-target file changed).

**Rung 2, slice 5 — driver absorption of the mod.build-era builds
(2026-07-11).** std/mem and std/str build under the driver; both
per-module lowbuild.sh scripts are deleted; the corpus's build-pin
loop swaps the two script entries for ONE driver entry over the full
products file — which also corpus-gates the lib/bin products for the
first time. Three small compositional product kinds carry the
transition, with no vocabulary growth:

- **'check** — a file's kernel obligations are green (std/str's
  aggregate rep certs). Generally useful, permanent.
- **'regen** — generator output is byte-identical to the committed
  artifact. Permanent, and see the contract change below.
- **'modbuild** — TRANSITIONAL: the entry still renders its own Plan
  (schema/kernel/tie/manifest/engine absorbed by the driver; the tie
  generalized to every-MOD-covered-by-some-TIE, hex membership —
  stronger than std/mem's script, which checked one module of one).
  This kind dies at the re-founding proper, when pin bindings + the
  pinlib deriver replace plan-assembly bodies.

**The driver found a real latent defect on first contact.** The
retired std/str script's regen gate was silently toothless: its
`diff -q … && echo "REGEN OK"` line cannot fail under `set -e`
(a failure on the LEFT of `&&` does not exit a bash script), so the
gate has been swallowing a genuine drift — wasmgen's regenerated
output no longer matched the committed str.wasm.shard. Root cause of
the drift itself: the canon std sweep rewrote the committed twin
(a C8 scrutinee-rebuild respelling, `(int_of_nat (S k2))` →
`(int_of_nat k)`), and wasmgen does not emit C8-canonical spellings.
Verified: `canon(shardfmt(wasmgen(src)))` equals the committed file
byte-for-byte.

**Contract decision (pending user ratification): regen =
determinism up to canonicalization.** The 'regen ladder gained a
tools/canon leg (via a transient sibling probe file, since canon
resolves relative imports); the gate compares the CANONICALIZED
regeneration against the committed artifact. This is the honest
contract while committed artifacts live in the canonical tree and
generators predate the canon rules. The alternative — generators
emitting canonical spellings directly — is the SEARCH.md
"three speakers" question (kernel recognizer / tools/canon fixer /
generators as a third speaker of the C-rules) and belongs to that
coordination, not this arc. The lib/bin ladders' regen gates do not
yet carry the canon leg (their outputs are canonical today); lifting
them to the same contract is the uniformity follow-up.

Gates: driver green (15 products, 1m05); aggregate green at 15
entries (1m07); corpus FAIL set unchanged with the driver entry now
INSIDE the corpus's build-pin loop. mod.build.shard files survive
this slice AS-IS (bodies die at the re-founding); the examples/
fragment-era scripts (lowbuild.sh, _mem, _loop, _call, the seven
x86 fragment builds) remain for a later mechanical migration.

**Rung 2, slice 6 — the re-founding proper (2026-07-11).** Both
mod.build.shard entries are rewritten to the new charter, and the
transitional 'modbuild kind is renamed out of existence ('pinlib):

- **meta/build grows PinMod** — name, spec-side encoded bytes (the
  meta/plan pre-encoded law: the driver never imports target
  encoders), the pinned twin file, claim names in module-index
  order, curated vectors. meta/build now imports meta/plan (config
  vocabulary referencing plan vocabulary — the layering is
  deliberate: entries RETURN plan-adjacent content).
- **tools/lowbuild grows `pinlib ENTRY`** — loads the entry, invokes
  the standard symbol, and derives every ARTIFACT binding
  POSITIONALLY from the claims list (the full-prefix convention:
  claim K pins fn index K). No raw cert parsing in the deriver: the
  manifest gate already verifies every derived index against the
  cert file read RAW, so a wrong derivation fails the build — PCC
  discipline, reused instead of duplicated.
- **The entries shed exactly their packaging.** What remains is
  content: the module value + spec-side encoding, the pin binding,
  the curated vectors (mem: 5; str: 6, spec-side sc_copy/sc_eq).
  What died: the art builders, the PM/MkPlan assembly, every
  hand-maintained cert/certfile/model/index cross-reference — the
  drift-prone bookkeeping the entries can no longer misstate.
  Verified: mem's derived plan is BYTE-IDENTICAL to the old hand
  wire; str's differs only where uniform derivation FIXED the old
  entry's inconsistency (a spurious `callees=none` dropped).

**D1 residual RESOLVED on real cases: role-named standard symbols.**
`build_products : () -> (List BuildProfile)` for product lists;
`build_mods : Target -> (List PinMod)` for a module's pinned
artifacts. Named, role-specific entry points won over one omnibus
`build()` — the omnibus form is what made the original mod.build
verbose, and the ruling's `bin_helloworld_wasm()` sketch survives as
free-form helper fns behind the standard symbols. Repo-scope product
discovery (the other D1 residual) stays open: examples/
build_products.shard remains the single aggregator until a consumer
demands per-module product declarations.

Gates: entries check green (123/140 claims incl. closures); pinlib
plans verified against the old wire; driver 15 products green 1m07;
aggregate + corpus pending at record time, confirmed in the commit.

**Rung 2, slice 7 — the wasm fragment migration (2026-07-11).** The
four fragment-era wasm scripts (lowbuild.sh, _mem, _loop, _call) die;
their four .build.shard entries are re-founded to `build_mods`; each
fragment becomes two driver products — a 'regen (generator
determinism up to canonicalization) plus a 'pinlib (the remaining
four gates: schema, kernel, tie+manifest, engine):

- **PinMod grows the callee-prefix shape** — `pre` (leading module
  fns pinned by ANOTHER product's certs; claims pin indices pre+K)
  and `callees` (the prefix's module note). A linked mem-fragment
  module [mget, mset, self] declares pre 2, callees "std/mem";
  leaves declare 0/"". The claim-kind strip generalizes to
  first-underscore (lowered_/linked_ both derive the source fn
  name), and `lowbuild pinlib ENTRY [x86]` now selects the ISA model
  by target (the x86 arm lands with the x86 fragment slice).
- **BuildProfile grows `aux`** — the auxiliary artifact path ("" =
  none) for two-file cert sets: regen's aux = the second generator
  output; pinlib's aux = the portable companion of the linked
  primary (own schema leg + manifest seat; the linked OUT stays
  primary — kernel checks its import closure, verified: LINK's
  check = PORT's obligations + its own; ties cover its module
  literals). Kinds that have no aux shape refuse a nonempty aux at
  plan time.
- **Two-output regen writes REAL BASENAMES** in a per-product
  capture subdir (mkdir leg). Found by the driver's gates on the
  first run: the generator derives the linked file's import line and
  module name from its output FILENAMES, so capture-named outputs
  produced `(import "15.raw")` — the retired script never hit this
  because its tmp files carried the committed basenames.
- **Second canon-tree catch:** the committed loop cert carried
  pre-canon `(int_of_nat (S k2))` spellings (the same C8 class as
  slice 5's str finding, opposite direction — the canon std sweep
  never touched examples/). The committed file is respelled into the
  canonical tree (machine inductions re-check green, 71/0; the
  entry's derived plan is byte-stable — pins reference values, not
  spellings). The plain fragment was already canonical; mem/call had
  no drift.

Verified: plain and mem derived plans BYTE-IDENTICAL to the old
wires; call byte-identical; loop differs only by dropping the old
entry's spurious `callees=none` (the slice-6 str precedent, now
uniform). Products 15→23, all green 1m13 — the same wall time as 15
products (group parallelism absorbed the growth). Aggregate at 10
entries 1m22. Remaining fragment-era scripts: the nine x86 builds
(next slice — entries are Opus-delegated per the working split).

**Rung 2, slice 8 — the x86 fragment migration; the script era ends
(2026-07-11).** The nine x86 fragment scripts die (lowbuild_x86.sh
+ loop/mem/call/chain/loopcall/intloop/div/itoa), their entries are
re-founded to `build_mods`, and with the last script entries gone
lowbuild_all.sh itself DISSOLVES — the aggregate build is now
exactly one command, `tools/build/build.sh
examples/build_products.shard`, and the corpus lowering loop runs
that single entry. 18 scripts at the arc's opening; zero remain.

- All nine were the same single-file shape (SRC/OUT/entry, one
  x86gen output, XMOD/TIE set gate, CPU engine) — no aux pairs.
- Driver: 'regen x86 (x86gen + the canon ladder) and 'pinlib x86
  (the wasm ladder's shape over the x86 model + the CPU engine
  legs). Fragment ties carry no EFF line, so the x86 byte-tie
  refactored into `xtie_sets` (pure set equality) with the EFF
  percolation check as the lib/bin tail — the pinlib judge is the
  set gate alone, exactly the retired scripts' gate. lowbuild
  renders pinlib x86 plans through xplan_lines (`pinlib ENTRY x86`).
- The nine committed x86 certs are ALREADY canonical — the canon
  legs came back green with no loop-style respell. The canon-drift
  exposure was a wasm-era artifact.
- Entry rewrites Opus-delegated (the standing low-level split); all
  nine derived plans BYTE-IDENTICAL to the old wires, no
  callees=none drops needed (the x86 entries always passed "").
- Flagged in passing, PRE-EXISTING in the committed entries (not
  corpus targets, not driver-gated — entries load in run mode):
  five entries' local `xnat` helper fails the totality gate
  (measure site 0), and x86div/x86intloop reference `eapp` without
  a use-clause (run-mode glob resolution covers it; check-mode
  flags it). Queue: fix with the old-era lowbuild mode retirement.

41 products all green 1m56. Corpus lowering section = the one
driver line. With the scripts gone, tools/lowbuild's old-era
default mode (`lowbuild ENTRY [x86]` over a `build` fn) has ZERO
callers — retiring it (drive/run_isa + the Plan decoders that only
it uses) is a follow-up residual.

**Rung 2, slice 9 — the old-era mode retired; the flagged entry
defects fixed (2026-07-11).** The delete-old-code half of the
re-founding:

- **tools/lowbuild sheds the old-era mode.** `drive`/`run_isa` and
  the six Plan decoders only they used (dc_art/dc_arts/dc_mod_lib/
  dc_mod/dc_mods/dc_plan) are DELETED; the 1-arg and 2-arg-ISA main
  arms die with them (the 2-arg form is now `pinlib ENTRY` alone);
  header + usage respelled as the PLAN DERIVER with its four live
  modes (pinlib/lib/elf/binelf). Entries that render their own Plan
  no longer exist anywhere, so the machinery that consumed them no
  longer exists either. dc_vec/dc_strs, target_val, ctor_qn, and
  both wire renderers stay — the live modes share them.
- **The xnat totality FAIL (five x86 entries): fixed by adopting
  the proven idiom.** The bare `(measure i)` self-call never
  carried the witnesses the measure gate verifies; the repo's own
  fnat_go (models/wasm) is the same fuel-from-Int loop with
  verify-don't-search `by arith` witnesses on an accumulator shape.
  xnat is now that pair (xnat_go + wrapper), value-identical —
  every derived plan byte-stable.
- **The eapp use-clause (x86div/x86intloop): added.** Check-mode
  resolution now matches what run-mode glob resolution always did.

All nine x86 entries + the four wasm entries + std/mem + std/str
now check 0 failed — the build-entry population is kernel-clean,
not just driver-green. Driver 41 products green 1m42; corpus
FAIL-set identical. Rung 2 closes here: entries are content-only,
the deriver speaks only the new charter, and no drift-prone
packaging survives anywhere in the tree.

**Rung 2 coda, slice 10 — the two residuals (2026-07-11).**

- **Regen canon-leg uniformity.** The lib (wasm/x86/x86elf) and bin
  ladders adopt the 'regen products' canon pipeline (gen -> fmt ->
  probe -> canon -> rm, judged canon-vs-OUT); the byte-identical
  fmt-vs-OUT gate (g_regen) dies. One contract, every regen surface:
  determinism up to canonicalization, committed artifacts in the
  canonical tree. Third canon-tree catch on the first run: the addw
  bin cert carried `(S^ 20 Z)` where canon spells the packed ground
  literal `(S^ 20 0)` — respelled in (354/0 re-check green).
- **D1 residual #2 RESOLVED: repo-scope discovery = COMPOSITION,
  NOT DISCOVERY.** A products file may declare a second standard
  symbol, `build_entries : () -> (List (List Int))` — paths of
  further build entries. The driver loads each as its OWN closure
  (meta/invoke's find_fn refuses ambiguous local names, so
  importing entries into one closure was never on the table) and
  appends its build_products in listed order. std/mem and std/str
  now own their product lists in their entries (the configuration
  home doing its job); examples/build_products.shard keeps the
  examples products and lists the two entries. No filesystem
  scanning, no name patterns — adding a module to the build is one
  path in one list. Guards: build_entries is OPTIONAL (absent =
  none) but ambiguous refuses; nesting refuses loudly at ONE level
  (an included entry declaring build_entries fails the plan —
  negative-probed).

Driver 41 products green (composition adds ~0 wall); corpus
FAIL-set identical. The arc's rung-2 queue is empty; next is
rung 3 — profiles v1, opened by design exchange (MEMORY.md D1's
class-assignment surface lands there).

## 12. Non-goals, stated once

No config dialect — profiles are shard values. No distinguished
trusted linker — ISA.md §4's ruling stands; linking is user-writable
script logic and all outputs are checked. No privileged generators —
derive is a slot, not a status. No recipe language — the conformance
graph plus derivation is the whole vocabulary. And the driver never
becomes the soundness authority — authoritative replay stays on the
standard pipeline, exactly as the compiled chain's standing rule.
