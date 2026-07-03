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

Ratified 2026-07-02. All four demonstrator slices (§6) are **[BUILT]**
(model + encoder at `models/wasm/`, pieces at `examples/wasm_pieces.shard`,
weld script + emitted certificate at `examples/wasm_weld{,_out}.shard`,
engine differential at `examples/wasm_diff*`); the measured question (§7)
is answered — see the dated addendum there. A post-demonstrator review
(§10, 2026-07-02) reframed wasm's role and sequenced the next arcs: the
Nat former (kernel arc, precondition for fuel at scale) first, then `Mem`.

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
(related by proven lemmas). A **piece theorem** is then an ordinary
equation:

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
   fuel machinery internal. *(Slice-2 amendment: the anticipated
   fuel-monotonicity law was never needed — see the fuel-form addendum in
   §7. The law kit that materialized instead: `wrap32_id` (in-range mod is
   the identity) and per-piece fuel-tower shift lemmas.)*
2. **Two hand-written pieces.** Small arithmetic-flavored functions as code
   values, each with its equational piece theorem `∀ x, call_fn … = Some …`.
   This slice carries **the measured question of the arc** (§7).
3. **The weld.** A compile script that builds the composite module value
   (B calls A), emits it plus the stitched correctness claims as an ordinary
   shard file, and the standard pipeline checks it. First exercise of
   kernel-as-library outside prove.

   *(Slice-3 amendment, 2026-07-02 — built, with two shape findings. (i) The
   **weldable theorem form**: a piece's theorem pins ONLY the module slots it
   touches — its own entry and its callees' — as ctor literals at their ABI
   indices; every other slot is an opaque `Func` binder the index lookup
   walks past without inspecting, and the module list's tail is an opaque
   `(restfs (List Func))`. Composite theorems are then pure instantiations:
   the weld script computes each piece's filler/tail bindings from the
   layout and emits one `rewrite-with` per claim. Three pieces
   (add=0, sum=1, triple=2, triple calling add across the opaque slot 1)
   exercise filler, tail, and cross-slot cases. (ii) The script needed **no
   kernel-as-library beyond the model itself**: pieces are manipulated as
   ordinary `Func`/`WModule` values and rendered to source text; the checker
   re-checks the emitted file, so the script stays fully untrusted. In-process
   checking for iteration remains available later; it wasn't needed here. The
   corpus carries a regen pin: the committed certificate must be
   byte-identical to what the script emits from the current pieces. v1
   composition scope: the ABI layout is fixed at authoring time — theorems
   are not transportable across re-indexed layouts; that needs symbolic-index
   or theorem-transport machinery, deliberately deferred.)*
4. **The reality check.** An encoder from the module value to real `.wasm`
   bytes (`Bytes` exists for exactly this) and a differential run under a
   real engine — so the "engine conforms to model" trust leaf is exercised,
   not hypothetical. Dev-side script; nothing in-logic.
   *(Built 2026-07-02. `models/wasm/encode.shard` = the encoder (untrusted,
   corpus-gated for totality only): type/function/export/code sections, one
   functype per function, every function exported as `f<i>`, and all LEB128
   as bounded if-ladders — u32/s32 fit in ≤5 seven-bit chunks, so minimal-
   width encoding needs no `Int` recursion and no div-based measure proofs.
   Two commitments where model meets format: Block/Loop get the VOID
   blocktype (the model threads the whole stack through blocks; a piece that
   branches carrying operands fails engine VALIDATION — the differential's
   honest failure mode, not a silent divergence), and `(I32Const c)` encodes
   `s32(wrap32 c)`, mirroring the model's wrap-at-push. The differential:
   `examples/wasm_diff_run.shard` (RUN-mode) emits the welded composite +
   a 15-function const-width probe as hex plus vectors whose expectations
   are computed by `call_fn` — not hardcoded; `examples/wasm_diff.{sh,mjs}`
   replays them under node/V8. Result: **25/25 agreement** (wraparound adds,
   loop sum to 5050, cross-slot triple, every sleb width boundary at both
   signs), and the harness was verified to fail loudly on a corrupted module
   and on a wrong expectation. `run_corpus.sh` carries the differential as a
   node-guarded pin. The output stays `(List Int)` rather than `Bytes` —
   the artifact is dev-side and `std/bytes` is opaque in check mode; wrap it
   when something in-logic ever consumes encodings.)*

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

**Addendum 2026-07-02 — answered.** Slice 2 measured it, and the go/no-go
clause fired exactly as intended: the first denotation (Int fuel, `(lt fuel
1)` guard) made every symbolic-fuel proof pay a case-on + farkas pair per
fuel decrement (~14 per loop iteration) because the guard never reduces on
symbolic fuel. Two model re-factors dissolved the burden instead of pushing
through it:

- **Fuel is structural** (`(type Fuel (FZ) (FS Fuel))`), not Int. A concrete
  FS tower over an opaque tail ι-reduces through the interpreter and sticks
  exactly where the proof takes over. Piece theorems quantify an *additive
  slack tail* `(c Fuel)` — `∀ c, call_fn (TOWER c) m k args = Some …` — which
  **self-composes**: a weld instantiates `c` with whatever budget remains at
  the call site. No fuel-monotonicity lemma, no `fuel ≥ bound` premises.
  Loop pieces use a per-piece tower fn (`pf_sum n c` = FS^n over the tail,
  one unit per iteration) plus one shift lemma re-spelling the same tower
  with the per-pass budget at the head.
- **The loop engine is a named SCC member** (`eval_loop`), not an inline
  `Loop`-arm blob, because ι-reduction consumes an inline arm before a
  worker lemma can cite it; the named call stays folded under `reduce` and
  is the exact unit loop inductions recurse on.

Measured burden after the re-factors: a straight-line piece theorem
(symbolic values, slack fuel) closes with ONE `(compute both)` — zero
per-instruction cost. The loop piece (`sum` over `Call add`, ∀ n in i32
range) costs a fixed ~10-step skeleton per loop — one wf-induct worker whose
inductive arm is: fire the exit test, one compute per iteration boundary,
one `wrap32_id` cut for the wrapped counter, cite the IH via a
have-materialized instance (so LHS-matching binds the slack binder that a
RHS-side rewrite would dangle), and a final compute both — the two sides
collide as identical stuck terms. Per-instruction cost inside an iteration:
still zero (compute grinds the 12-instruction body + the cross-function
`Call` in one step). Wall time: the whole pieces file checks in ~25 ms under
`shard_check`. Spelling discipline that makes it work: specs must mirror the
interpreter's residue exactly (kernel compute is inert on symbolic
arithmetic — `((x+1)+2)` ≠ `(+ x 3)` syntactically), and claim statements
spell module/body values as ctor literals, never as nullary-fn calls, since
CBV evaluates call arguments before sticking.

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

## 10. Post-demonstrator review (2026-07-02) [DECIDED]

A failure-modes review with the demonstrator complete produced two
corrections to this doc's framing and a sequencing decision.

**Fuel representation.** Structural `Fuel` (§7's finding) keeps its proof
economics — kernel cost tracks static body size, never run length, because
fuel bounds depth/iterations and induction handles one symbolic iteration at
a time. But *ground* fuel is unary: a real workload's model run at fuel 10⁸
is 10⁸ cells before the interpreter takes a step. Materializing large
linked-list naturals is a non-starter, and a dev-side Int-fueled twin was
rejected (patches one site, leaves an unproven artifact in the loop). The
fix is a **Nat former**: values are
machine integers with a nonneg invariant, with constructor *views*
(a literal `n ≥ 1` matches `(S m)` binding `m = n-1` as a literal; symbolic
`(S c)` spellings reduce and stick syntactically, unchanged). Every tower,
shift lemma, and additive-slack theorem from §7 survives verbatim; ground
fuel becomes a numeral. The `(Word W S)` former is the historical playbook,
with the caveat that Word was later revoked to std in the trusted-core
contraction — so the design's first question is the *minimal* kernel delta
(views are necessarily kernel reduction; everything else may not be).
Rejected alternatives: refine-only Nat (an opaque
eliminator kills the free ι-step — every match needs a cited refine-fact)
and Int fuel with guard-discharge automation (a farkas certificate per
decrement where an ι-step used to be — worse *checking* cost, not just
authoring). This was sequenced as a kernel arc outside this arc's fence,
subsuming the pending Nat refine retrofit (#39).

**BUILT 2026-07-03** (commits 9b1b3ac → 3fcfc4e): the kernel `Nat` —
`(type Nat (Z) (S Nat))` in `kernel/stdlib.shard`, ground values packed
to nonneg Int literals, `Z`/`S` views in every engine's matcher; see
`docs/LANGUAGE.md` §3 "Nat". The model's local `Fuel` type is deleted;
`models/wasm` uses the kernel `Nat` directly (`FZ`/`FS` became `Z`/`S`
by mechanical rename, every theorem verbatim, weld certificate
regenerated byte-identical). The fuel-materialization failure mode is
measured closed: `sum(50000)` at fuel 10⁶ runs sub-second in ~4MB where
the unary fuel value alone was 258MB. One placement rule emerged, now
corpus-pinned (`examples/nat_prim.shard`): packing fires only in
evaluation-grade reduction (`step`/`ceval`), never in the proof-facing
normalizers — a packed goal stops matching `Z`/`S`-spelled IHs and
lemma statements.

**Wasm's role, reframed.** Wasm is the *simplest lowering target a proof can
be traced to* — the training ground for composition-is-citation and the
certificate shape — not the terminal proof ISA. The ambitious end state is a
compiler chain whose per-target contract is: *here is a binary, here is the
ISA model used, here is the proof that the binary-in-model meets the
bin-level requirements*. An x86 story terminates in an x86 binary with that
same contract. Crucially the chain **factors**: a certifying wasm→x86
translation emits per-unit cross-model refinement theorems, and the x86-level
requirement proof is *composed* from the wasm-level one plus translation
refinement — the wasm intermediary stays load-bearing as interior lemmas
inside the x86 certificate, never re-proved from scratch. Certifying
compilation (per-unit proof emission, standard pipeline checks) is this
doc's generate-and-check commitment applied to passes; the slice-3 weld
script is its seed. Dragons, in dependency order: `Mem` (data representation
in linear memory is the precondition for everything richer, including x86's
flat memory and calling conventions) → cross-model correspondence vocabulary
(the adapter thread coming due) → pass-proof generation economics (solver-
manufactured simulation proofs; same measured-question discipline as §7).

**Sequencing decided:** Nat former first (self-contained, fence-respecting
kernel work that removes a known wall before it is hit), then the `Mem` arc.
