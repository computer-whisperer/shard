# ISA — machine computations as proven data

> Status legend: **[BUILT]** in the kernel/tools and exercised by the corpus ·
> **[DECIDED]** ratified, not yet built · **[FUTURE]** anticipated, deliberately
> deferred. Keep these honest — this doc is the record of *why* the arc has
> the shape it does, so a later change starts from intent, not re-derivation.

See also: `OVERVIEW.md` (the trust model this instantiates), `REVISIT.md`
§Architecture/Trust (the dated ratification entry), `docs/archive/TRANSFER.md`
(the v1 pilot: `wasm ⊑ loop ⊑ rev`, the founding demonstration), issue #14
(refinement lowering — this doc is that issue's ground-up re-derivation and
supersedes parts of the 2026-06-18 multi-impl/linker design discussion).

Ratified 2026-07-02. Nothing in this arc is built yet; the demonstrator (§6)
is the first slice.

---

## 1. The root needs

Strip away every mechanism proposed so far and three needs remain. Everything
in this doc cascades from them, constrained by the standing project
invariants: tight kernel with a closed assumable base, generate-and-check
economics (untrusted searchers, checked artifacts), verify-don't-search in the
kernel, the static-lowering principle (no first-class functions, ever), and
the module surface discipline.

- **Need A — representability.** A computation expressed in some concrete ISA
  must be representable in shard — statable, reasoned about, related to a
  high-level specification by ordinary proof — **without the ISA model being
  part of the language.** The kernel must never learn what wasm is.

- **Need B — composition.** Proofs about individual ISA pieces must compose
  mechanically into proofs about welded assemblies. The scenario to serve: a
  module offers a wasm implementation of some function together with its
  correctness theorem; a linker process welds it to other such
  implementations into a composite artifact; the composite's correctness
  proof is assembled *from the piece theorems*, never by re-verifying piece
  internals.

- **Need C — the toolchain is object-language code.** Compilation, lowering,
  and linking are shard programs run at compile time: shard functions
  manipulating programs *as data structures*, using the kernel as a library.
  This is promoted from implementation convenience to a root need because
  the trust economics depend on it — a toolchain written in the object
  language under generate-and-check needs **no trusted tools at all**
  (§4).

## 2. Cascade from Need A — the ISA model is a library

**The model is an ordinary shard module.** [DECIDED] Instruction and module
datatypes, machine-state types (a natural `(record …)` consumer), and an
interpreter function — plain definitions, zero axioms, zero kernel or loader
involvement. "This wasm code computes `f`" is then an ordinary claim about an
ordinary function applied to ordinary data. This is exactly the v1 pilot's M4
posture, re-instantiated on the v2 substrate.

Consequences that fall out for free:

- **Model multiplicity.** Because a model is just a module, several can
  coexist as peers (wasm now; RISC-V or an interim C-chain machine later).
  Nothing anywhere privileges one. Corollary: composition (Need B) is scoped
  *within one model*; cross-model welds are the adapter problem, deferred
  (§5).
- **The trust leaf is named, external, and singular.** The only unproven step
  is "the real engine/hardware conforms to the modeled semantics" — an
  external-pedigree question exactly like an axiom's pedigree under the
  closed-base rule, and it goes in the trust ledger like extern reachability
  does today. Everything between the shard spec and the modeled execution is
  proof.

**Which ISA first: wasm, for structural reasons, not just precedent.**
[DECIDED] Wasm's structured control flow (blocks/loops, no arbitrary jumps)
is what makes a *big-step denotation* definable at all — and §3 shows the
whole composition story rests on that. An unstructured ISA forces CFG
reconstruction or continuation-style reasoning, killing the equational
approach. Additionally: real engines exist for differential reality-checks,
and wasm's module instances have private linear memories, which §3 leans on.

**wasm32 fragment only in v1.** [DECIDED] The compiled chain i63-traps on
u64; putting i64 in the interpreter puts that debt on the critical path.
Fence it out until it's needed.

## 3. Cascade from Need B — composition is citation, not a calculus

**The load-bearing design decision of the whole arc:** the model's primary
semantic object is a *big-step function-call denotation* —

```
call_fn : Module -> FuncIdx -> (List Val) -> Option (List Val)
```

— with the fuel-indexed small-step machine internal to the model library
(related by proven lemmas, fuel-monotonicity in the law kit). A **piece
theorem** is then an ordinary equation:

```
∀ x,  (call_fn m f (encode x)) = (Some (encode (spec x)))
```

And a **weld proof is ordinary equational reasoning with the kernel we
already have**: unfold the composite's execution to a call site, cite the
piece theorem as a rewrite, continue. Composition = `rewrite-with` + cuts +
the model's lemma kit. No new kernel steps. No program logic. No bespoke
certificate format — **the end-to-end certificate is a shard file** whose
claims cite piece theorems, checked like any other file.

**Rejected alternative: a Hoare-style spec calculus.** [DECIDED] Canonical
pre/post specifications over machine configurations (argument locations,
memory framing clauses, fuel bounds) were the obvious import from the
verified-compilation literature. Rejected because the burden lands on every
artifact author (all theorems must speak the calculus), the calculus must
anticipate every weld shape in advance, and — decisively — the equational
form obtains the same welds from existing kernel machinery. The design work
moves from "invent a logic" to "factor the model so its call boundary is a
clean equation," which is the same kind of work as std/bytes' law family.

**Framing is dissolved for v1, not solved.** [DECIDED] The composition
boundary is the wasm module-instance boundary: instances have *private*
memories, cross-instance calls pass values. Pieces cannot smash each other's
state *by the model's structure*, so piece theorems carry no framing clauses
— memory effects are internal to each piece's denotation. What remains at
the boundary is **encode/decode round-trip proofs** per crossing value type
(finite and concrete for the i32-ish types of v1). The costs accepted with
this: a copying tax at boundaries (priced in by the earlier multi-impl
analysis; irrelevant at demonstrator scale), and the interface-adequacy
dragon is *deferred to rich boundary types*, not slain.

**What this pushes out of the arc entirely:** the in-place/`Mem` story.
Shared memory, regions, and framing are about *per-piece performance*, not
about composition — the pilot already demonstrated in-place once (TRANSFER
M3), while composition has never been demonstrated. `Mem` becomes the
*second* arc, entered only after composition stands. [FUTURE]

## 4. Cascade from Need C — compile scripts, quotation without eval

**The trust-critical property: quotation without internalized evaluation.**
[DECIDED] Programs — wasm code now, lowered shard later — are ordinary data
values. A compile script constructs, transforms, and welds them; "running"
quoted code is just applying an interpreter function to data. There is **no
axiom connecting a representation to the function it denotes** — no
`quote`/`eval` in the logic, ever. The connection is generate-and-check: the
script *emits* actual decls and claims, and the kernel checks the emitted
things. This keeps full reflection (a known soundness minefield and a kernel
fattener) permanently out, and it means the compile script itself never
needs verification.

- **A "suspension" is a data pairing**, not an operational thunk: an artifact
  value plus the qname of its correctness theorem, packaged behind a module
  surface. Forcing = interpretation. No first-class functions sneak in; the
  static-lowering principle stands untouched.
- **The authority model is prove's, generalized.** tools/prove already calls
  `check_sequent` in-process to search, then emits winners the driver
  re-verifies. A compile script does the same for artifacts: in-process
  kernel calls are for iteration speed and search; **trust attaches only to
  emitted, replayable artifacts re-checked by the standard pipeline** (the
  Rust-interpreted kernel remains the sole soundness authority). Certificates
  over sessions — which engine ran the script never matters.
- **Kernel as a library** therefore means: a sanctioned, stable module
  surface over what check.shard already exposes internally (`check_sequent`
  and friends). prove is the existence proof that this works; the new work
  is making the surface deliberate instead of tool-internal.
- **Linking is user-writable script logic.** There is no distinguished
  linker tool, trusted or otherwise: a linker is a shard function from
  (artifact, theorem) pairs to a composite artifact plus stitched claims.
  Anyone can write one; none is special; all outputs are checked.

## 5. What this supersedes

Torn up deliberately (from the 2026-06-18 multi-impl/linker design
discussion and earlier framings of #14):

- **The distinguished proof-carrying linker** as an artifact of the system →
  dissolved into user-writable compile scripts (§4).
- **"One end-to-end ISA certificate"** as a format → there is no format; the
  certificate is a shard file (§3).
- **Loader-level "expansion joint" machinery** (module = interface + N
  selectable implementations wired by the loader) → the module system stays
  exactly as it is (interfaces, `fulfills`, opaque types); implementation
  *selection and assembly* is untrusted compile-script logic. The loader
  stays dumb.
- **`Mem` on the composition critical path** → second arc (§3).

Kept from that discussion, unchanged: reasoning is representation-agnostic
behind the module interface; keep a shard reference implementation as the
lemma factory; proven adapters at shard altitude are the eventual boundary-
tax answer; interface adequacy is the structural precondition for
compositional refinement (deferred with rich boundary types, and the surface
discipline is what buys it when it arrives).

## 6. What to build now — the demonstrator [DECIDED]

Two functions, not one: one function re-proves representability, which the
pilot already proved; **only two proves composition**, which is the arc's
novel claim. Slices, demonstrator-first per house method:

1. **The model library.** Wasm32 fragment sized to the demonstrator and no
   larger: i32 const/arith/compare, locals get/set, block/loop/br_if,
   call/return. **No linear memory in v1** — pieces compute on locals; memory
   enters with the `Mem` arc. Big-step `call_fn` as the primary object;
   fuel machinery internal; a minimal law kit (call unfolding at a weld
   boundary, fuel monotonicity).
2. **Two hand-written pieces.** Small arithmetic-flavored functions as code
   values, each with its equational piece theorem `∀ x, call_fn … = Some …`.
   This slice carries **the measured question of the arc** (§7).
3. **The weld.** A compile script that builds the composite module value
   (B calls A), emits it plus the stitched correctness claims as an ordinary
   shard file, and the standard pipeline checks it. First exercise of
   kernel-as-library outside prove.
4. **The reality check.** An encoder from the module value to real `.wasm`
   bytes (`Bytes` exists for exactly this) and a differential run under a
   real engine — so the "engine conforms to model" trust leaf is exercised,
   not hypothetical. Dev-side script; nothing in-logic.

Hard fences for all four slices: **zero kernel changes, zero loader
changes.** If a slice appears to need one, the design is wrong — stop and
re-derive.

## 7. The measured question: symbolic reduction burden

The logic above is clean; the empirical risk is not. Proving
`call_fn code (encode x) = …` for symbolic `x` means the kernel's
reduce/rewrite machinery grinding through an interpreter — match-heavy,
fuel-recursive — with symbolic values in the state. Everything the snake arc
taught (simp exploding match-headed fns on ctor args, unfold refusing match
scrutinees, stuck-if arms) says this is where "effective vs. burdensome"
gets decided, and only the demonstrator answers it.

Slice 2 therefore measures: proof lines per instruction, wall time per
theorem, and which tactic vocabulary carries the weight. **Go/no-go:** if
per-instruction burden is heroic (a snake-slice of tactic archaeology per
ten-instruction function), the response is to re-factor the model's
denotation toward computation-friendliness and/or extend the prove solver —
*before* scaling to real workloads. The model's factoring is untrusted
vocabulary; iterating on it is cheap. Do not push through heroically.

## 8. Non-goals for v1

Linear memory and in-place algorithms (`Mem`, second arc) · shared-memory
composition and framing · i64, floats, tables, growth · cross-model welds
and adapters · generated refinement passes (#11 rides later) · surface sugar
for code values · packaging/build-system concerns (`bin` remains the
artifact gate; how compile scripts are invoked stays ad-hoc run-mode for
now) · any change to kernel or loader.

## 9. Open questions

- **Reduction burden** — empirical, owned by the demonstrator (§7).
- **The sanctioned kernel-library surface** — which check.shard entry points,
  and in what module shape, compile scripts may call. Shaped by slice 3.
- **Encode/decode for rich types** — the adapter story's re-entry point when
  boundaries carry more than machine ints. [FUTURE]
- **Where the interim C chain meets this** — it stays the execution path for
  shard code itself; refined artifacts run under real wasm engines; whether
  the chain's own output ever becomes a modeled-ISA artifact is deliberately
  unforced. [FUTURE]
