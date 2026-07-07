# Lowered-conformance certificates — the standard form

**STATUS: RATIFIED 2026-07-04.** What is ratified is the FORM — the
statement schema (§2), the portable/linked split (§6d), the mod.build +
five-gate artifact conventions (§6, §6i), and the tools/low library
architecture (§6g) — as the standard that future work builds on rather
than revises. Fragment coverage, the common lowering step's design, and
target count all stay open by design (§7, §8). Nothing here is set in
stone — reworking on evidence of a better way is expected — but changes
to ratified sections are deliberate acts against the corpus pins, not
drift. This document is the one formal object the arch-specific build
paradigm hangs from; everything else (the wasm lowerer, mod.build
conventions, the CLI runner, welds/linking) is engineering behind it.

Corpus pins (run_corpus.sh): the six gated builds
(`examples/lowbuild{,_mem,_loop,_call}.sh`, `std/mem/lowbuild.sh`,
`std/str/lowbuild.sh`) run end to end, the schema recognizer's and the
manifest gate's negative fixtures must stay refused (§6ad), and the
kernel articles (`lowered_form`, `rep_probe`, `lowfrag_probe`, the
probes riding wasm_diff_run's closure, the generated cert files through
their builds' KERNEL gates) are checked every run. gate_sweep.sh
type-gates the four apps (lowergen, lowcheck, manifest, bytetie) and,
through their closures, the tools/low kit.

## 1. What this is

When a module is compiled for an ISA target, the artifact is:

    binary bytes + ISA-model identity + per-function certificates + glue

The certificates are the load-bearing part, and they only compose — across
modules, across hand-written vs auto-lowered provenance, across build
scripts and JIT-eval consumers — if every certificate has the **same
statement shape**. That shape is the *lowered-conformance form*: it is to
compilation what `fulfills` is to modules. Hand-tuned and default-lowered
code are interchangeable exactly because they discharge the same form.

Shard is first-order, so the form cannot be a parameterized higher-order
predicate. It is a **statement schema**: a shape of claim, instantiated
per function, with ecosystem consensus enforced the weld way — a core
statement-generator that build machinery uses to emit and validate cert
statements (the same discipline that keeps `wasm_weld_out.shard`
regenerable byte-identical).

## 2. The schema

One claim per lowered function. Two denotation entries (both already in
the wasm model): `call_fn` for scalar-observable functions, `call_fn_mem`
for memory-observable ones — the former is the degenerate instance of the
latter.

    (claim lowered_<f>
      (goal ((<args>) (c Nat) (restfs (List Func)) <adapter-free-vars>)
        (<PRE premises>)
        (= (DEC (call_fn_mem (FUELF <args> c)
                             (MkWModule (Cons <func-literal> … restfs) <memsize>)
                             <idx>
                             (ENC-args <args>)
                             (ENC-mem <args>)))
           (SPEC <args>))))

Slots:

| slot | what it is | discipline |
|---|---|---|
| func-literal(s) | the emitted code, PINNED as literals in a Cons-prefix | nullary-fn spellings never match residues; literals or `(inline …)` only |
| `restfs` | free tail of the module's function list | **the linking provision** — the cert holds with anything welded in after |
| FUELF | a fuel *function* of the args, over free slack `c` | shard claims have no ∃; the free-slack tower (`(S^ N (pfc k c))`) IS existential fuel + monotonicity in one move |
| ENC / DEC | ordinary shard fns converting public signature ↔ ISA representation | runnable (the JIT-eval boundary) and citable; identity/`Some` at the scalar end; for MEMORY inputs, ENC is an observation PREMISE over an arbitrary `m0`, not a construction (§3 framing) |
| PRE | premises over args (and encoded memory) | where ALL discipline lives — see §3 |
| SPEC | the source shard function itself | not a wasm-flavored respelling; the cert reaches the fn the consumer knows |

## 3. Design decisions

**Memory is in the form; a heap is NOT.** `call_fn_mem` threads a `Mem`
because the ISA forces one. Nothing in the form assumes an allocator, a
heap well-formedness invariant, or reachability. Both existing hand pieces
(`rev8`, `copy8`) are heap-free — bare address-range premises — and
embedded-style targets need exactly that. The heap discipline of the
*uniform-representation default lowering* (the rt.h analog: tagged cells,
bump allocation) enters as one particular *instantiation* of the PRE/DEC
slots, owned by the future uniform-rep std module — never by the form.

**Fuel: function-slot + free slack, not existentials.** The pieces already
solved this: fuel `(S^ N (pfc k c))` with `c` universally quantified means
the theorem holds at every fuel of that shape — a consumer instantiates
`c` with whatever surplus its own budget leaves. No fuel-monotonicity
metatheorem needed for v1 composition (welds already ride this).

**Total-under-premises, not trap-conditional (v1).** The conclusion
asserts `Some`/a defined observation outright; overflow- and bounds-safety
are PRE premises (the wrap32_id shape). This is the i63 stance made
formal: within the premises, exact correctness; outside them, the artifact
is simply not certified (interpreted path remains). A trap-conditional
variant ("if it returns, it returns right") is a possible later weakening
for default-lowered code — deliberately not in v1.

**Representations are type-owned; per-function adapters live at public
boundaries only.** When lowered `f` calls lowered `g` internally, no
ENC/DEC occurs at the call site — the certs only compose if both sides
agree on representation, so the default lowering owns one uniform rep
keyed by type. A module hand-rolling a custom rep for its public functions
is fine (its adapters say so); cross-rep calls then need certified
conversion glue — **deferred to v2**. V1 rule: custom-rep pieces interact
with default-lowered code at the shard level only (exactly how the hand
pieces coexist with everything today).

**Framing is IN the form for memory pieces — observationally (RESOLVED by
P1, 2026-07-03).** The naive memory schema (initial memory constructed by
ENC from `mem_empty`) cannot compose: it says nothing about running on a
memory some earlier piece produced. The framed shape is the form's memory
variant, and it is the memory analog of `restfs`:

1. the initial memory `m0` is an arbitrary binder;
2. the ENC slot becomes an **observation premise** — "the source range
   dumps to xs" — instead of a memory construction;
3. two standard companion claims ride along: pointwise preservation
   outside the footprint, and the range-level corollary ("any range
   outside the footprint re-dumps unchanged") — the consumer glue that
   lets the NEXT piece's ENC-observation premise survive THIS piece's run.

Two findings from proving it (test articles C/D/E): **no substrate
union/disjoint theory is needed** — std/mem's own discipline (compare
observations, never Mem values) rules out sep-logic-style `mem_union`
statements and the observational spelling turns out to be complete on the
existing surface; and **byte_ok vanished** from the framed premises (it
only ever guarded the unframed capstone's load/dump round trip — the
unframed statement is the `m0 := (load xs 0 (mem_empty))` corollary of the
framed one, not the primitive).

## 4. Evidence: existing statements are already instances

- `add_thm` / `triple_thm` (`wasm_pieces.shard`): scalar schema with slack
  `c` and open `restfs` — missing only the source-fn SPEC (they spell the
  spec wasm-flavored, `wrap32 (+ a b)`).
- `copy8_copies` (`wasm_copy.shard`): the memory schema exactly — ENC =
  `load xs 0 (mem_empty)`, DEC = `dump dd k (omem …)`, inline adapters.
- `rev8_reverses` (`wasm_rev.shard`): same, with SPEC = `(rev xs)`.

The schema is descriptive of practice, not aspirational.

## 5. Test articles (`examples/lowered_form.shard`, 2/2 first try)

**A. `lowered_add1` — the default-lowerer end.** Source fn `add1_src x =
x + 1`; hand-played the cert a certifying lowerer must emit: scalar
denotation, identity adapters, PREs in wrap32_id's range shape, SPEC =
`add1_src` itself. Proof = `compute lhs` + unfold SPEC + one wrap32_id
cite with pass-through premise discharges. This is the per-RS-form schema
instantiation in miniature — mechanical, generator-emittable.

**B. `lowered_copy8` — the hand-piece end.** copy8's capstone restated
with NAMED adapter fns (`enc_bytes`/`dec_range`) in the ENC/DEC slots;
proof = unfold the two adapters + cite `copy8_copies` with pass-through
discharges. The form absorbs the hand piece without weakening.

**C/D/E. The framed form (P1)** — `lowered_copy8_framed` (conformance over
arbitrary `m0` with ENC as a dump-observation premise),
`lowered_copy8_frame_below` (pointwise preservation below the footprint),
`lowered_copy8_frame_dump` (range-level consumer glue, by induction with
head via D and shift via `cp_shift` — the `dump_set_above` template).
All three ride `copy8_thm` directly, which was already fully general in
memory — the frame narrowing only ever lived in the capstone.

**P2 — the emitter probe (`tools/lowergen`, 2026-07-03).** The first
certifying emitter: a ~400-line shard app on the kernel front end
(lower.shard's anti-split-brain contract) that, for every fn in the scalar
straight-line fragment (Int params/return, + - * over params and u32
literals), emits the wasm function literal AND the §2 claim with its full
proof — one wrap32_id cite per param-containing arith node,
innermost-first (= postorder), premises discharged positionally (node j →
premises 2j/2j+1). `examples/lowergen_src.shard` (5 fns) →
`examples/lowergen_out.shard`: **all five machine-written proofs passed on
the first generation attempt** (including the 3-node chain and the
empty-chain identity), the fuel formula `2·instrs + 3` was exact at both
tested sizes, and regeneration is byte-identical after shardfmt (the weld
discipline transfers to cert files). Fragment refusals are loud
(ground-arith subtrees, out-of-range literals — refuse-don't-guess). The
claim-assembly section of lowergen IS the first statement generator; the
consensus/validation mechanism of open question 1 now has a concrete
object to check against.

**P2b — the let fragment (2026-07-03).** lowergen grew to
let/straight-line, de-risked by two probes (`examples/lowfrag_probe.shard`)
before building:

- *Fuel max+slack law*: a Block/BrIf/Br branch diamond with asymmetric
  paths certifies under ONE tower sized for the longer path — fuel is a
  depth bound and `Out` carries no fuel, so a completed run is insensitive
  to leftover slack. No per-branch fuel, no monotonicity lemma. (Pinned
  for the coming `if` fragment; the per-arm proof is a fixed 4-step
  template: compute lhs / unfold rhs / rewrite case-hyp both / compute
  both.)
- *Sharing pattern*: the kernel opens `let` by ζ-substitution, so a
  let-bound value read k times duplicates k-fold on the spec side while
  the wasm side computes its local once. Reconciliation: per unique
  substituted arith node, a named `have` citing wrap32_id ONCE + a plain
  all-occurrences rewrite of the have fact — constant proof cost per node
  regardless of fan-out. (`unfold` does not ζ-open the let it exposes;
  the emitted spine carries one `(reduce rhs)`, a safe no-op on let-free
  bodies.)

The emitter design is TWO WALKS: code from the original let tree (sharing
preserved, one LocalSet per binding, locals allocated after params in
textual order), premises/citations from the substituted tree (its arith
nodes ARE the compute residue's mod-sites; duplicates dedupe by spelling,
an unused binding's nodes vanish). Emitted proofs are now in `chain` form
with named haves — machine certs became human-readable. All 8 proofs
(3 new let fns incl. nested lets and a bare-param alias, 5 re-emitted)
passed on the first generation attempt; four lowbuild gates green, V8
differential 16/16 (the locals section exercised for real).

**P2c — the if fragment (2026-07-03).** Tail-position `if` over
`int_eq`/`lt` conditions whose operands are params/aliases/literals —
*no arith in conditions*, so wraps never enter the case-on spelling and
the split term matches the compute residue on both sides. The emitter
walk became a REGION TREE mirroring the branch structure: per `if`, a
`case-on` with the probe-pinned fixed arm template (rewrite the
ctor-named case hyp into both sides, compute both); each arith node is
discharged in the region where its code executes (pre-branch let
bindings before the case-on, arm-local nodes inside their arm), deduped
against ancestor regions; PREs are globally unique with an index map.
One fuel tower per fn (max path; the slack law), formula `2·instrs+3`
unchanged.

Two findings:
- *The engine gate caught a real ISA fact the model abstracts away*:
  wasm blocks are TYPED, and void blocks must have an empty stack at
  every boundary — a naive diamond that leaves the branch value on the
  stack kernel-checks green but V8 REJECTS the binary ("expected 0
  elements on the stack for fallthru"). The fix is a **result local**
  per if (LocalSet in each arm, LocalGet after the outer block) — the
  same locals-not-stack-across-block-edges discipline the hand-written
  loop pieces already follow, no model or encoder change. This is the
  four-gate architecture doing its job: kernel truth ≠ engine validity,
  and the ENGINE gate is where the difference surfaces.
- *PRE caveat (v1)*: arm range premises quantified over the whole
  contract — `(if (lt 0 x) (- x 1) 0)` would demand `0 ≤ x-1` globally.
  RESOLVED by §6j (PRE hygiene): arm premises are condition-relative,
  and a side derivable from the case hyp is discharged silently by a
  generator-emitted Farkas helper.

All 12 proofs pass (4 new: ground-arm gate, arith arms, nested if,
let-above-if with a zero-node True arm); four gates green, V8 25/25.

**P3 — the adapter-combinator probe (`examples/rep_probe.shard`, 69/0,
2026-07-03).** The type-owned representation cascade demonstrated on the
first non-scalar shape: a pointer-linked `List Int` in 8-byte cells over
std/mem's LE layer. The element combinator (`enc_u32`/`dec_u32` + its law
`u32_round`) and the DERIVED list combinator (`enc_list`/`dec_list` +
`rep_list_id`), where the list round-trip proof literally cites the
element law for the head — swap the element combinator and the derivation
re-instantiates. Two design findings with architectural weight:

1. **Bump direction is a proof-architecture choice.** Encoding
   parent-first (head cell below, tail encoded above) makes every write
   land strictly above finished structure, so the only frame lemma needed
   is "an encoder running above doesn't disturb a word below"
   (`l4_enc_below`, one clean induction). Child-first order would need a
   data-dependent read-set invariant. The uniform-rep allocator should
   allocate-then-fill top-down for this reason.
2. **Fuel-driven decoding removes tags from the adapter.** DEC may take
   the spec-side length as a parameter (it is an observation function,
   like `dump`) — no Nil-word discrimination, no `if`-guard, and the
   round-trip statement is exactly `dump_load_id`-shaped. Tag words
   (rt.h's odd/even immediates) are only needed for POLYMORPHIC slots;
   their proofs (div-facts) are deferred to the uniform-rep arc.

Proof cost: two farkas certs read off the tracer's slot table, one helper
shape, one chain reorder (leftmost-occurrence targeting forces
head-chain-first), plus one genuine QoL discovery: **named cut premises
(`have` names) do not resolve through deep `rewrite-with` continuation
nesting** — introduce the `have` adjacent to its citation site
(positional refs can't reach cut premises at all). Also: `len_cons`
collides with a std/list axiom — homonym hazard for probe-local lemma
names.

**Findings.** (1) Both ends fit one schema; the only variance is slot
contents. (2) `(inline …)` is file-local, so cross-file statement reuse
needs a local nullary twin — fine for generated self-contained cert files,
a papercut for hand reuse. (3) Named adapters cost two `unfold`s per cert;
adapters must be APPLIED calls, never nullary spellings. (4) The framed
articles' only debugging was two `(stop …)` additions: `compute` after a
worker/thm citation must stop every fn whose folded spelling the next
citation matches — including fns in ARGUMENT position (`length_nat`); the
stop-the-fuel-fn law generalizes to "stop everything you intend to cite
against."

## 6. mod.build.shard — BUILT in miniature (P4, 2026-07-03)

The convention, demonstrated end to end on `lowergen_src`:

- **`tools/lowcheck`** (P4a) — the consumer-side schema recognizer. Raw
  s-expr level; validates every `lowered_*` claim's slack binder, restfs
  binder, `(S^ N BASE)` fuel with the slack in BASE, Cons-pinned
  `(MkFunc …)` module prefix *ending in restfs*, literal index. The
  negative fixture (`examples/lowcheck_rejects.shard`) is kernel-TRUE yet
  schema-REFUSED (Nil module tail) — truth and composability are
  different gates, and the consumer checks both (the PCC discipline).
- **`examples/lowergen_src.build.shard`** (P4b) — the first mod.build
  file: assembles the artifact set as a plan — `ARTIFACT` manifest lines
  (cert name ↔ cert file ↔ ISA model ↔ export/externs glue), `MOD` lines
  (real .wasm bytes via the model's encoder, one single-function module
  per fn = the cert's module at restfs := Nil), and `CASE` vectors whose
  expected values are the SOURCE fns applied directly (spec-side
  semantics, not the model).
- **`examples/lowbuild.sh`** — the four-gate build: (1) REGEN
  (producer determinism, byte-identical), (2) SCHEMA (lowcheck), (3)
  KERNEL (the proofs), (4) ENGINE (V8 replays the plan: 10/10 agree).

The **default** mod.build = the core lowering library (lowergen today;
shard → RS-shard → wasm as it grows); a module overrides it to hand-tune,
and both roads discharge identical statements. Variant selection stays
explicit build-script data (no resolver magic).

V1 gaps, on record: (a) ~~the build file's func literals are copied from
the generated certs — the cert↔binary tie should be CHECKED~~ RESOLVED
by the §6i byte-tie gate (2026-07-04); (b)
certs pin their fn at index 0, so combined multi-function modules can't
instantiate them yet — the `triple_thm` filler pattern (pin own index,
opaque fillers before) is the known fix when welding needs it.

### 6b. std/mem — the first REAL module through the convention (2026-07-04)

`std/mem` ships its surface as callable wasm artifacts, exactly as the
call-composition probes rehearsed (no module is special to any compiler
machinery; call-per-byte accepted v1):

- **`std/mem/mem.wasm.shard`** — the artifact pieces: `mget_f`/`mset_f`
  function literals + `lowered_mem_get`/`lowered_mem_set` certs. Memory
  effects are stated DENOTATIONALLY against the module's own opaque
  surface (`mem_set m0 a v`), so consumer frame flow is pure citation:
  re-cite the read cert at the post-write memory (the framed schema's
  arbitrary m0), collapse with `get_set_other`. Certs use the
  **minimal-prefix convention**: each pins the function-table slots up to
  its own index and quantifies the rest — any consumer keeping std/mem's
  fns as the low-index prefix in canonical order can cite them. Not part
  of the std/mem module proper (directory modules load only
  mod.req/impl); consumers import the file by path.
- **`std/mem/mod.build.shard`** — the build entry: the shipped binary is
  the certs' module at `restfs := Nil` (`[mget, mset]`, one 64KiB page);
  MEMCASE vectors' expected values AND memory readbacks are computed
  spec-side (`mem_get`/`mem_set` applied directly).
- **`std/mem/lowbuild.sh`** — three gates (SCHEMA → KERNEL → ENGINE); no
  REGEN gate, the pieces are hand-written v1.
- The composition probes are now CONSUMERS of the shipped certs
  (`sum2`/`setget`/`bump` cite `lowered_mem_*` — the acceptance test that
  a foreign module can compose against the artifact set), and
  `call_bridge` graduated to `models/wasm/wasm.shard`.

Findings recorded on the way: `(inline NAME)` is same-file-only, so a
consumer statement cannot ride a foreign fn's body — consumers carry
byte-identical local literals (drift fails loudly at the bridge citation)
until the link-time generator splices literals; `tools/lowcheck` learned
to resolve same-file `(inline …)` chain elements (the loader's exact
rule — the expanded element must still be a `MkFunc` literal).

### 6c. The emitter's Call fragment (2026-07-04)

`tools/lowergen` grew the MEM fragment: source fns `(m Mem) (x Int) …`
whose bodies read/write bytes lower to wasm **Calls against std/mem's
shipped artifacts** (call-per-byte v1, per the call-composition probes),
and the generated proofs **compose by citation** — `call_bridge` +
`lowered_mem_get`/`lowered_mem_set`, never computing into a callee body.
The generator SPLICES the callee literals into statements (the link-time
answer to the `(inline …)` same-file rule).

- Fragment: `Int` return = read-only body (arith + `mem_get`, reads may
  be addresses); `Mem` return = a **single trailing** `mem_set` (lets
  allowed above; all reads pre-write, so no aliasing premises exist —
  write-then-read stays a hand piece until condition-relative premises).
  Emitted module = `[mget, mset, self]` at index 2 (the minimal-prefix
  convention); PREs per unique (kind, spelling): wrap bounds for arith
  nodes, address bounds for mem-op addresses (the callee certs' premise
  shapes — read-value bounds derivable via `get_lo`/`get_hi` are
  AUTO-DISCHARGED since §6j and never appear as premises).
- **The stage law** (what makes the proofs machine-writable): wrap
  events are collected during the CODE walk and flushed per call site —
  `(compute lhs (stop eval_call))`, then the pending events (everything
  materialized so far — stack, locals, the folded call's arguments — is
  on the lhs and collapses all-occurrences), then the bridge with clean
  spellings. A let-bound value collects once (later reads are LocalGets
  of the collapsed local); a RECOMPUTED spelling collects again and its
  event re-fires for the post-call materialization (`mg_sq` pins this).
- `examples/lowergen_mem_src.shard` (9 fns: reads, let-sharing,
  indirection, re-fire, store, copy, bump): **all 9 machine-written
  proofs passed the kernel on first generation**;
  `examples/lowbuild_mem.sh` = the four gates
  (REGEN → SCHEMA → KERNEL → ENGINE, V8 11/11 incl. store-truncation
  and bump-wrap edges); build file `lowergen_mem_src.build.shard` uses
  spec-side expected values and readbacks throughout.

### 6d. The PORTABLE cert form — RATIFIED and adopted (2026-07-04)

The callee-coupling answer (probed in `examples/portcert_probe.shard`,
adopted as the emitter default the same day). A consumer cert need not
embed its callees' bytes; the emitter now produces TWO files:

- **The portable file** (`lowergen_mem_port.shard`) — the PRIMARY
  certs, `lowered_*`: the module is an abstract binder `(m WModule)`
  with premises (0) the **funcs spine** `(= (funcs_of m) (Cons f0
  (Cons f1 (Cons SELF-LITERAL restm))))` — callee slots abstract, only
  the claim's own function a literal; (1,2) callee **arities**
  (`call_bridge` needs the pop count, never a body); then per-site
  **behavior equations** at each site's exact fuel spelling, and wrap
  bounds, in discovery order. Callee implementation bytes appear
  NOWHERE (grep-checkable) — a std/mem reimplementation leaves this
  file byte-identical. Statement size is O(call sites), not
  O(transitive callee closure).
- **The linked file** (`lowergen_mem_link.shard`) — the DERIVED
  artifact certs, `linked_*`: today's structural statements, proven by
  ONE citation of the portable cert (spine/arity premises by compute,
  behavior premises by citing std/mem's shipped certs, wrap bounds
  passed through). Zero interpreter steps — when a callee's bytes
  change, only this file regenerates and re-derives.

Residual coupling = the **fuel budget**: behavior premises pin site
towers, and at link each must dominate the callee's actual tower (the
slack absorbs the difference by unification). The emitter charges each
call site 2× its callee's tower + 1, so callee growth within 2×
relinks without touching consumers; a budget miss fails the link
derivation loudly and regenerating the one consumer fixes it.

`tools/lowcheck` recognizes BOTH forms, classified by the conclusion's
module slot (`MkWModule` literal = structural — linked artifacts and
leaf certs like std/mem's; the symbol `m` = portable, with the
spine/binder rules above). Build manifests point at the linked certs
(the form tied to the shipped binary). Proof-side gotcha, pinned:
record accessors unfold to matches under compute, so `funcs_of` rides
the first compute's stop set — the spine premise's spelling must
survive to be rewritten.

### 6e. The rep-swap acceptance test — PASSED (2026-07-04)

The claim the portable form was adopted for, demonstrated end to end:
*one interface, two conformant artifact sets, one consumer, zero
consumer proof edits on swap.*

- **`std/mem/mem.wasm2.shard`** — a v2 variant implementation:
  different bytes (scratch-local detours), different towers
  (`S^10`/`S^14` vs v1's `S^6`/`S^10`), same behavior — discharging
  the same statement shapes as v1's certs. Stands in for a hand-tuned
  mod.build override or a representation change behind the surface.
- **`examples/repswap_probe.shard`** — the v2 link derivations for
  `sum2` and `bump`: the portable consumer certs are imported
  UNCHANGED (the file imports v2's certs and *not* v1's), and the
  structural artifact statements re-derive against the v2 module by
  pure citation. Both passed first-try. The v2 towers land inside the
  consumers' 2× fuel budgets, so even a *larger* implementation swaps
  in without touching a consumer — the budget policy earning its keep.
- Engine leg: the v2 module (`rsmod` in the differential plan) runs the
  same consumer binaries over the variant callees, V8-green.

What this pins for the roadmap: an implementation/representation swap
behind a module surface is a **relink** — consumer proof burden zero —
provided consumers cite only behavior-level statements (which the
portable schema enforces by construction: there are no bytes to cite).

### 6f. The loop-generation probe — PASSED (2026-07-04)

The biggest unproven bet of the ratification round: can the emitter
machine-generate a LOOP piece — an induction worker plus its piece
theorem — from a recursive source fn? `examples/loopgen_probe.shard`
hand-plays the generator on a fixed template, every line derived by
mechanical rule; **all four machine-template proofs (two workers, two
theorems) passed with a single template-level fix**, on two instances: a
new `fill_loop` and mem_copy's real `copy_loop` (the machine twin of the
hand-written copy8 piece).

- **The fragment (loop template v1)**: Nat-counted tail recursion —
  `(match k (Z m) ((S k2) (NAME (mem_set …) (+ a 1) … k2)))`, accum
  updates `a` or `(+ a 1)`, one trailing store per iteration, reads
  pre-write. **Counter-as-local** design: the machine carries `k` as an
  i32 local (`ENC` = the uniform type rule `Nat ↦ int_of_nat`), guard
  `LocalGet kk / I32Eqz / BrIf 1`, decrement by `BSub` — a direct
  transliteration of the source recursion. Unlike the hand pieces'
  end-pointer style there is no ghost parameter, so the public signature
  IS the source signature, and every guard/collapse fact is
  template-constant (the price: a fixed 3-step counter-collapse dance —
  `(1+n)-1 → n` by arith, then `wrap32_id`).
- **Generated items**: per file, a fuel fn (`lg_fuel`, one `S` per
  iteration), ONE stride twin (`lg_adv`, shared by every advancing
  accum), and 7 helper certs with **pinned farkas certificates** —
  template constants, mined once by tools/prove while authoring the
  template; the generator never enumerates. Per fn: body/func literals,
  the worker induction (Z = guard-exit compute; S = the staged pass:
  unfold fuel/eval_loop, guard decider, premise-rewrite address guards
  in code order, counter collapse + wrap32 haves, open the spec one step
  on the rhs, cite the IH at the advanced accums), and the `lowered_*`
  theorem (worker + 4 plumbing levels; both pass tools/lowcheck).
- **Fuel law (pinned)**: `eval_loop` re-enters at exactly fuel−1 — fuel
  is a depth bound — so the folded re-entry redex aligns with the IH's
  spelling at ANY sufficient body budget. Charged formula: worker
  `S^(instrs+4)`, theorem `+4`.
- **The one fix — the opacity finding**: `int_of_nat` is a module fn,
  OPAQUE under check-mode compute, and the counter design feeds it into
  MACHINE STATE where the interpreter must evaluate it. The general law:
  any ENC fn riding into machine state must be opened by its **defining
  lemmas**, not compute — rewrite `int_of_nat_zero` (Z case) /
  `int_of_nat_succ` (S case) into the lhs before the first compute. Two
  template steps; the hand pieces never hit this because the end-pointer
  style keeps spec `k` out of the machine.
- **Portability limit, recorded**: loop bodies with CALLS cannot ride
  the §6d portable form — per-site behavior premises are stated at the
  goal's one slack binder, but a call site inside a loop fires at a
  different fuel every iteration (`S^j (lg_fuel k2 c)`). A lemma
  citation CAN instantiate its own slack per site, so calls-in-loops
  work in the STRUCTURAL form (cite the callee cert directly inside the
  S case) at the cost of regenerating the loop cert on callee edits.
  Direct byte-op loop bodies — this probe — are leaf certs with no
  coupling at all.
- Engine leg: `lpmod` in the differential plan (fill + copy), V8 54/0 —
  `Loop`/`Br`/`BrIf`/`I32Eqz`/`BSub` engine-validated, including the
  store-truncation edge and a forward-overlap run beyond the theorem's
  premises.

What remains for the emitter fragment proper (after lowergen
library-ification): source-shape recognition, the walk emitting the
staged S-case chain (a fixed sequence over the mem fragment's existing
event machinery), and PRE generation (a range premise pair per advancing
address accum). Extensions behind the fence: Int-accumulator returns,
stride ≠ 1, multiple stores per iteration, calls in loop bodies
(structural form).

### 6g. lowergen library-ification (2026-07-04)

The emitter's reusable core now lives in **`tools/low/`** — three plain
library files, imported by path, glob-used — with `tools/lowergen`
keeping only what is genuinely the app (the fragment walks, claim
assembly, the module walk, the CLI):

- **`tools/low/doc.shard`** — the output-document rope (`Doc`,
  combinators, one flatten at write time) plus the text/path utilities
  and import/use header-line builders. Pure text; knows nothing about
  wasm or schemas.
- **`tools/low/schema.shard`** — the statement-schema kit: canonical-
  expr spelling (`pr_e`/`spell` — the invariant that premises and inst
  pins must match compute residues exactly), binder environments,
  node-set dedup machinery, type predicates, goal binder/arg/PRE
  printers, the `PItem` portable-premise layout (§6d) with its premise
  renderers, and the RHS/self-literal builders.
- **`tools/low/proof.shard`** — the proof-form templates: ONE unified
  `wrap_event` (the previously duplicated `pr_event`/`m_event` differ
  only in premise-index lookup), the stage-law flush, the `call_bridge`
  citation builders, and the link-derivation discharge templates
  (`link_subs` now takes the shipped cert names as parameters instead
  of hardcoding std/mem's).

Acceptance was mechanical: both four-gate builds pass with the REGEN
gate **byte-identical** — the refactor provably changed no output. The
loop fragment (§6f's emitter work) will be written over this kit; if
lowergen itself grows unwieldy with it, the next split is per-fragment
files over the same kit.

### 6h. The emitter's LOOP fragment (2026-07-04)

The §6f template, mechanized over the §6g kit — `tools/lowergen` now
emits **machine-written inductions**. For every source fn in the loop
shape (`(fn NAME ((m Mem) (a1 Int) … (an Int) (k Nat)) Mem (match k (Z
m) ((S k2) (NAME (mem_set …) U1 … Un k2))))`, updates `ai` or
`(+ ai 1)`, store value an accum / u32 literal / `(mem_get m ADV)`,
addresses ADVANCING accums), it emits the counter-as-local wasm body,
the `lg_*` worker induction, and the `lowered_*` theorem.

- **`models/wasm/loopkit.shard`** — the probe's per-file kit promoted
  to a shared proven article (fuel fn, stride twin, the 7
  template-constant helper certs); generated files import it, the
  generator never emits or enumerates certificates.
- Recognition works on the kernel's own term representation
  (`Match`/`Arm`/`PCtor` over the loader's BVar indexing) — the
  anti-split-brain contract extends to pattern shapes. Refusals are
  loud and specific (non-advancing address, `(+ 1 a)` spelling,
  non-self tail call, …).
- Proof emission is the §6f staged chain: guard decider, per
  address-OCCURRENCE guard stages in code order (a re-used address
  re-fires its stage — the stage law's re-materialization rule),
  counter collapse + wrap haves, spec opened one step on the rhs, the
  IH at the advanced accums. Premise layout: one nonneg + one range
  pair per advancing accum, nonnegs first, param order.
- **`examples/lowergen_loop_src.shard`** (fill / copy / stamp — value
  accum, read value, literal value) → `lowergen_loop_out.shard`: **all
  six machine-written proofs (3 workers + 3 theorems) passed the
  kernel on the first generation attempt**; fuel formula
  `S^(instrs+4)` worker / `+4` theorem held at all three sizes.
- `examples/lowbuild_loop.sh` = the four gates
  (REGEN → SCHEMA → KERNEL → ENGINE, V8 6/6 incl. store-truncation,
  k=0, and forward-overlap vectors); build file
  `lowergen_loop_src.build.shard` computes expected values and
  readbacks spec-side (the Nat counter arg via `fnat`). Loop pieces
  are LEAF certs — module `[self]` at index 0, no callee prefix, no
  link file (the pure/mem REGEN gates stayed byte-identical under the
  shared header change).

Fragment fence (unchanged from §6f): stride ≠ 1, multiple stores per
iteration, and calls in loop bodies (structural-form-only when they
come) are future extensions. Int-accumulator returns landed as §6k.

### 6i. The byte-tie gate — the fifth gate (2026-07-04)

§6 gap (a), closed. Build files hand-copy function literals from the
certs into their MOD lines; nothing tied the shipped bytes to the
modules the theorems are actually about. Now it is CHECKED:

- **`tools/bytetie`** — for every structural `lowered_*`/`linked_*`
  claim in a cert file, re-derives the bytes of the claim's module at
  `restfs := Nil`: the raw module literal is REFLECTED into a `WModule`
  value (`(inline …)` resolved by the same-file rule at any depth —
  generate-and-check, no internalized eval) and run through the model's
  own encoder; prints one `TIE <fn> <hex>` line per claim. Portable
  claims are skipped (no module literal — their artifact form is the
  linked derivation); an uninterpretable literal is a loud exit 2.
- The build scripts diff the TIE lines against the plan's MOD lines
  (`examples/lowbuild{,_mem,_loop}.sh`, exact set match) or, for
  std/mem's single shipped binary, assert `TIE mem_set = MOD stdmem`
  (the full-prefix cert's module IS the binary; the prefix certs are
  tied at the kernel level by citation). A drifted hand-copy now fails
  the build instead of silently shipping a binary the theorems are not
  about (drift-injection verified).
- **`tools/low/certnav.shard`** — the raw-cert navigation machinery
  (form reading, the inline table, call-application location) extracted
  from tools/lowcheck when bytetie became its second consumer; both
  tools now share it.

All five gates green on all four builds (pure/mem/loop/std-mem).

### 6j. PRE hygiene (2026-07-04)

The last ratification-round queue item: derivable premises leave the
contracts, and over-broad premises become condition-relative. Probed by
hand first (`examples/prehyg_probe.shard`, 42/0 first run — in
wasm_diff_run's check closure), then mechanized. Three mechanisms, all
riding `tools/low/lin.shard` — the new generator-side linear-arithmetic
kit (polynomials over spelled atoms, read intervals, and **computed
Farkas multiplier lists** — multipliers are read off coefficients,
never searched; every list rides a generated `(by arith …)` claim the
kernel checks, so a generator bug is a loud gate-3 failure, never an
unsound cert):

- **Read-bound auto-discharge (mem fragment).** An arith node LINEAR in
  byte reads whose interval fits `[0, 2^32)` becomes a `PAuto` item —
  zero premise slots. The proof derives its bounds: a generated
  `wb_<fn>_<k>_lo/hi` helper claim (byte premises per read ⊢ the bound,
  by the interval multiplier cert) cited through a `(have bJ …)` whose
  sub-proofs are `get_lo`/`get_hi` rewrites; the wrap event then cites
  the haves instead of premise indices. `mg_bump`'s two derivable
  premises are GONE from `lowered_mg_bump`; `mg_sum2`, `mg_dbl`
  likewise. A product of reads is NONLINEAR — Farkas cannot certify it,
  so it keeps its premises (`mg_prod` = the pinned negative control);
  nodes touching params keep theirs (genuine contract).
- **Link-side address bounds.** Same classification for behavior
  addresses in the linked derivation: a ground literal's pair
  discharges by `(compute both)` (and an out-of-range literal address
  is now REFUSED at generation — the linked artifact would be vacuous);
  a read-derived address's pair derives via `ab_<fn>_<k>_lo/hi`
  helpers (`linked_mg_ind`'s indirection address, `linked_mg_first`'s
  trivial `(le 0 0)` pair — both premise-free now). Only genuinely
  contractual addresses keep pairs.
- **Condition-relative premises (pure fragment — the P2c caveat,
  resolved).** Arm-local node premises are if-wrapped in the arm's
  condition path — `(= (if COND BOUND True) True)` (`(if COND True
  BOUND)` on a False arm), nested for deeper regions — and STRIPPED
  inside the arm by a `(have pJ …)`: rl-rewrite the premise onto the
  goal's rhs, fire every path hyp (positional `(hyp K)`, innermost = 0
  — reaches same-polarity-shadowed outer hyps), compute the if nest
  away. A side derivable from a path condition alone (the abs pattern)
  emits NO premise: a generated `cb_<fn>_<k>` single-condition helper
  (proportionality solved on the first live atom, verified before
  emission) cited through a silent `(have dJ …)`. `(if (lt 0 x) (- x 1)
  0)` — the caveat's own example — is now `lg_dec` in the source set:
  lower bound silent, upper bound condition-relative. Everything is
  emitted by ONE fused walk (premises, helper claims, proof text
  together), so indices, ordinals, and have names cannot drift apart;
  fns without ifs regenerate byte-identically.

Consumer effect, demonstrated the day it landed: the portable certs'
premise DELETIONS flowed to `repswap_probe`'s v2 derivations as premise
deletions of their own (drop the corresponding sub-proofs, statement
gets stronger) — hygiene propagates through the citation chain. All
five gates green on all four builds; corpus closure 182/0, V8 54/0.
This also stages the future multiple-stores/aliasing loop work: the
disequality premises those need are condition-relative by nature.

### 6k. INT-RETURN loops — the accumulator comes home (2026-07-04)

The first §7.7 fragment-growth slice: loops whose Z arm returns an
ACCUMULATOR instead of the memory. Probe first
(`examples/intloop_probe.shard`, hand-played template on sum-of-bytes,
**44/0 on the first run**), then mechanized; `lp_sum` (byte checksum
over a range) joined the loop source set and **both machine-written
proofs passed on the first generation attempt**; all five gates green,
V8 replays the sum vectors.

The fragment (v1, deliberately minimal): READ-ONLY loops
`(fn NAME ((m Mem) (a1 Int) … (an Int) (k Nat)) Int (match k (Z ar)
((S k2) (NAME m U1 … Un k2))))` — the Mem param passes through
unchanged, the returned accum's update MUST be the sum shape
`(+ ar (mem_get m ADV))`, every other update is `ai` / `(+ ai 1)` as
before. Store+Int-return combinations and `(+ ar 1)` count-returns stay
behind the fence (the scan slice will need the latter and brings its
own helper pair).

Template deltas over the Mem-return loop, each now mechanized in
`tools/lowergen` (`li_*` path):

- **Epilogue** `(LocalGet r)` instead of `(I32Const 0)`; the theorem's
  result is `(Some (Pair (NAME m0 args k) m0))` — the spec call as the
  scalar, memory unchanged.
- **Spec-call spelling in the final locals**: a data-dependent accum
  has no `lg_adv` closed form, so the worker's exit-locals entry for
  the return accum IS the spec call — the IH's result and the
  rhs-opened spec meet at exactly that spelling.
- **The k-scaled wrap invariant**: premise pair `(le 0 ar)` +
  `(le (+ ar (* 256 (int_of_nat k))) 4294967296)` — self-sustaining
  through the induction (the accum grows by a byte while k drops by
  one). Three new template constants in `loopkit`
  (`lg_sum_lo`/`lg_sum_32`/`lg_sum_shift`, farkas certs computed per
  §6j) feed `wrap32_id` at the machine BAdd and step the invariant to
  the IH; byte bounds come from std/mem's `get_lo`/`get_hi`.
- **Machine update order**: the return accum's update reads other
  accums' OLD values (spec updates are simultaneous, machine's are
  sequential), so it is emitted FIRST, before the addresses advance.
- Premise layout generalizes to one nonneg + one range pair per
  PREMISED accum (advancing OR return), nonnegs first, param order;
  `lgw_`/`lowered_` fuel formula `S^(instrs+4)`/`+4` held unchanged
  (17 instrs + 4·extra-advancing).

Gate fallout worth remembering: running the (ratification-pinned but
never-yet-run) gate_sweep targets surfaced two pre-existing holes,
both fixed here — missing strict-scope use-lines (`core_named` in
lowergen, `chars_eq` in tools/low/lin — run-mode dispatch tolerated
them, check-mode scope did not), and a REAL totality gap in
tools/bytetie's reflector (the `(inline …)` table hop isn't structural
descent; a cyclic inline chain in a malformed input would hang the
tool). The reflector now threads reflection FUEL (= the input file's
character count, a bound every legitimate acyclic walk respects) with
`(measure d)` obligations discharged by tools/prove's sidecar
(`tools/bytetie/bytetie.auto.shard`); exhaustion is a loud error.

### 6l. Multi-store + two-pointer loops — in-place reverse (2026-07-04)

The second fragment-growth slice: TWO stores per iteration and a
DECREMENTING address accumulator, driven by the challenge example
`lp_rev` (in-place range reverse — the counter-as-local twin of the
hand-proven `wasm_rev`). Probe first (`examples/revloop_probe.shard`,
**49/0 on the first run**), then mechanized; `lp_rev` joined the loop
source set and **both machine-written proofs passed on the first
generation attempt**; all five gates green, V8 replays the reverse
vectors (12/12).

- **Two stores, zero aliasing lemmas.** The source work generalizes to
  the nest `(mem_set (mem_set m A1 V1) A2 V2)` (inner store executes
  first). The machine pushes the OUTER store's (addr, value) pair
  deepest, then the inner's, then stores twice — every load executes
  before either store on the pure stack (no scratch locals; the hand
  piece's idiom). Because reads happen against the OLD memory on both
  sides, the interpreter's memory residue is byte-for-byte the spec's
  nested spelling — the condition-relative disequality machinery (§6j)
  stays unused for this shape. (It remains staged for write-then-READ
  bodies, where a later load must see through an earlier store.)
- **The stride enum.** Per-accum updates generalize from a Bool to
  static / +1 / −1 (`LStr`). A dec accum rides the premise pair
  count-lower `(le (int_of_nat k) x)` + address-upper
  `(le (+ x 1) 65536)`: every touched address stays ≥ 1 and the final
  local stays ≥ 0, so the last decrement never wraps — the recorded
  fence is that a dec accum can never touch byte 0. Six new template
  constants in `loopkit` (`lg_dec` twin + `lg_dlo`/`lg_dlo1`/`lg_d32`/
  `lg_dshift`/`lg_dhi`, farkas certs computed per §6j). The dec guard
  stage is asymmetric to inc: the address-UPPER premise rewrites
  directly (it IS the model's check spelling) while the lower guard
  cites `lg_dlo`.
- The rework was proven output-neutral before `lp_rev` landed: the
  regenerated cert file for fill/copy/stamp/sum stayed BYTE-IDENTICAL
  under the stride generalization, and the pure/mem REGEN gates stayed
  green.
- V1 fences now explicit in the recognizer: at least one INC accum (the
  counter wrap `lg_kM` rides its premise pair), at most two stores per
  iteration.

### 6m. Conditional loops — early exit and the machine-written twin (2026-07-04)

The third fragment-growth slice, the one flagged as design-risk:
`lp_scan` (bounded byte scan, the strlen shape) — an `if` INSIDE the
loop body with a DATA-DEPENDENT early exit and an Int return. Probe
first (`examples/scanloop_probe.shard`, **49/0 on the first run**),
then mechanized; `lp_scan`'s machine-written worker + theorem (and its
twin) passed on the first generation attempt; five gates green, V8
16/16 including both exit paths.

- **The structure is the hand `wasm_rev` answer key's**: the S case is
  a `case-on` the condition. True arm = exit — the case hypothesis
  collapses the machine's branch flag AND the spec's `if` in one
  positional-hyp rewrite; no wraps, no IH. False arm = the standard
  iteration + IH at the advanced args. Termination stays Nat-counted
  (the budget guard) while correctness follows the condition — the two
  concerns never mix.
- **The twin**: on an early exit the budget counter's local is not a
  closed form of the inputs. The emitter writes a per-fn SPEC TWIN
  (`NAME_kf` — mirroring the source recursion, `wasm_rev`'s `fi`/`fj`
  made machine output) that rides the worker's exit locals; the
  RETURN accum needs no twin because its exit value IS the spec fn.
  This is the first RECURSIVE FUNCTION the emitter writes (spec-side
  only; gate 3 checks it like any article). `int_of_nat` rides into
  the twin's True arm, so that arm opens it with `int_of_nat_succ` on
  the rhs — the §6f opacity law again.
- **No new helper certs**: the scanned pointer is a standard inc accum;
  the whole slice rides the existing loopkit.
- v1 fragment: `(match k (Z ar) ((S k2) (if (int_eq (mem_get m ar) 0)
  ar (NAME m U… k2))))` — condition reads AT the returned accum against
  literal zero (one `I32Eqz`); `Ur = (+ ar 1)`, other accums static.
  Behind the fence: nonzero literals (`I32Const`+`BEq`), conditions at
  non-returned accums (each non-closed-form local costs one more twin),
  accumulating scans.

  UPDATE 2026-07-05: the literal WIDENED to any byte — nonzero tests
  emit `I32Const z` + `I32Bin BEq` (17 instrs, worker S^21/theorem
  S^25), zero keeps the one-instr `I32Eqz` encoding so existing outputs
  stay byte-identical. Both leave the same `(b2i (int_eq BYTE LIT))`
  machine residue, so the case-on template is UNTOUCHED — pure
  condition-vocabulary widening, the §6v precedent (no probe needed;
  `lp_find`, the memchr shape, proved first generation). Literals
  outside [0,256) refuse loudly (reads are bytes — a larger literal is
  a vacuous artifact). Conditions at non-returned accums fell with the
  §6aa flag shape; still fenced here: accumulating scans.

With §6k/§6l/§6m the loop fragment now covers: fill/copy/stamp-style
stores, byte checksums, in-place reverse, and bounded scans — every
piece machine-written end to end, every extension landed with zero
proof-template failures after its probe.

### 6n. Write-then-read — the memory term comes to the walk (2026-07-04)

The fourth fragment-growth slice, and the resolution of §7.7's last
staged item: straight-line mem-fragment bodies may now STORE AND THEN
READ THROUGH THE STORE. Probe first (`examples/wtrmem_probe.shard`,
**36/0 on the first run** — a put-then-get-through-the-store body in
the emitter's exact generated shapes), then mechanized; `mg_putget`
(read back a just-written byte) and `mg_swap` (a real two-byte swap
through a Mem let chain) joined the mem source set — both portable
certs and both link derivations green, five gates, V8 17/17.

- **The design**: the walk threads the CURRENT MEMORY TERM (the spec
  spelling — `m0` growing one `mem_set` per store). Reads, behavior
  premises, and bridge instantiations all fire AT that term, and since
  Mem-typed lets ζ-open by substitution on the spec side (exactly like
  scalar lets), machine and spec spellings stay identical throughout.
  **No collapse lemmas, no disequality premises, anywhere** — the §6j
  condition-relative disequality staging turned out to be needed by
  NONE of the four growth slices; a read that should see through a
  store simply does, on both sides, by spelling. (`get_set_other`-style
  reasoning remains what CONSUMERS of these certs do when they want the
  collapsed value — e.g. the wtrmem probe's statement carries the
  honest `(mem_get (mem_set m0 x 7) x)` spelling.)
- **Mem-typed lets are the linear sequencing form**: `(let ((m2
  (mem_set m x v))) …)` binds the updated memory; the walk enforces
  LINEARITY (using the param or a stale Mem binding after a store
  refuses loudly — the machine destructively updates, so non-linear
  sources are untranslatable). Intermediate mset returns park in
  scratch locals (the no-Drop discipline); the trailing store's return
  is the fn's scalar result, as before.
- **Portable-form generalization**: `PGet`/`PSet` items carry the
  site's memory term; behavior premises spell it in both the input and
  result positions (byte-identical to the old text when the term is
  `m0`). The link derivation discharges an updated-memory behavior by
  citing the same shipped `lowered_mem_get`/`set` — their arbitrary-m0
  binder unifies against the `mem_set` term (the framed schema earning
  its keep at a premise boundary). `lowered_mem_*` needed NO changes.
- Int-return fns stay store-free (the schema pins their memory
  unchanged) — a loud refusal, not a silent wrong statement.
- Fallout fixed en route: `pr_e` learned ternary applications (memory
  terms print as `(mem_set M A V)`); it printed `?` before, which the
  kernel caught as an unbound variable — gate 3 loud, per design.
- Regen stability: pure, mem, and loop outputs all BYTE-IDENTICAL under
  the memory-threading rework before the new fns landed.

### 6o. mod.build v2 — the plan is a VALUE (2026-07-04)

The text-plan-on-stdout interface was a serialization boundary between
two shard programs, and it is retired: a mod.build now exports

    (fn build ((t Target)) Plan)      — PURE; the whole interface

with the vocabulary in **`meta/plan`** (originally `tools/low/plan.shard`,
graduated §6s) (`Target` record:
isa/mem_limit/width — the compile-context channel, grows sizeof/layout
tables for generics without breaking build entries; `Plan` = `PMod`
modules each carrying name, pre-encoded binary bytes, `PArt` manifest
entries, `PVec` vectors). Module bytes ride the plan pre-encoded
(the model's encoder applied spec-side inside build) so the vocabulary
and driver stay target-generic.

Reaching `build` is the **dynamic-invocation pattern**
(**`meta/invoke`**, originally `tools/invoke` — kernel-as-a-module,
graduated §6s): the driver loads
the mod.build's import closure at runtime (`resolve_closure` +
`build_module_r` — QUALIFIED run-mode resolution with glob fallback),
finds fns by LOCAL name over the FnDef list (every file keys at its real
module path — callers shouldn't
know which), marshals values across the meta-level boundary (ctor QNames
from the LOADED closure's own typedefs via `ctor_qn` — the matcher
compares full qualified identities, so hand-spelled paths would be a
silent MNo), and applies via `apply_fn` + `run_expr`. Probe:
`examples/invoke_probe.shard` (4/4, corpus-pinned). Two hard-won laws:
a library consumed this way must NOT import `eval.shard` (its app `main`
collides with the consumer's at the core qname — silent usage exit), and
it lives outside kernel/ because the engine stamp hashes `kernel/*.shard`.

**`tools/lowbuild/lowbuild.shard`** is the ONE generic driver — no
per-module stubs: constructs the v1 Target (wasm/65536/32, argv-selectable
when a second ISA lands), invokes build, decodes the Plan
whole-or-nothing (stuck term or shape mismatch anywhere = loud exit 1,
never a truncated plan), and renders wire formats AT THE BOUNDARY only:
the ARTIFACT/MOD/CASE/MEMCASE lines are the V8 differ's grammar, now a
rendering the driver performs, not the interface modules speak.

`std/mem/mod.build.shard` converted (the rendering half of the file
simply deleted; vectors keep their spec-side computation); its gate
script's plan step became `lowbuild <mod.build path>`. Validation: the
driver's rendering of the converted build is **byte-identical** to the
retired main()-rendered plan, first run; all four std/mem gates green
(V8 5/5). The `Target.mem_limit` field is the first real metadata flow —
the module literal's memory size now arrives from the compile-context.

What this dissolves: hand-kept duplication between build files and
certs (a build entry references the cert file's own Func fns natively),
and shell-grep consumption of structured data. What it deliberately
keeps: cert proofs as FILES (the kernel gate needs source), the V8 wire
format (V8 is outside the shard world; irreducible, but rendered once
in one place), and bytetie for any bytes that PERSIST on disk.

All four build entries speak the form: the three examples plans
(`lowergen_{src,mem,loop}_src.build.shard`) converted next to std/mem —
every one of the four rendered **byte-identical** to its retired
main()-rendered plan on the first driver run, and all four five-gate
builds are green under the new pipeline. The loop build entry references
the GENERATED cert file's own `lp_*_func` literals (no hand copies); the
pure/mem builds keep local literals (the generated files inline their
MkFunc terms — no named Func fns to import yet), with bytetie policing
that tie as before.

### 6p. The aggregate rep-relation probe — PASSED (2026-07-05)

The std/str runway's design question: can a lowered statement speak
about a LIST value living in linear memory? `examples/bytesrep_probe.shard`
— 63/0 on its FIRST check — answers yes, and the aggregate statement
**decomposes** so cleanly that the machine layer never learns about it:

1. **The rep relation is an EQUATION, not a Bool predicate**:
   `(br_read m p n) = bs` — "the readback at [p, p+n) IS bs", with
   `br_read` the abstraction function (memory segment → list). Stating
   it as an equation avoids Bool-invariant inversion entirely; a
   `bytes_at`-style predicate would have needed have/cut chains to
   invert.
2. **The core theorem is pure shard — premise-free, machine-free**:
   `lp_scan m p n = p + br_z (br_read m p n)`, where `br_z` is the
   rep-independent list spec (index of the first 0 — the strlen shape).
   Induction on the budget; both sides case on the same
   `(int_eq (mem_get m p) 0)` scrutinee, so the arms align by spelling —
   the spelling-alignment law extending to the rep boundary. Only two
   local `(by arith (list))` identities needed (`p+0=p`, the
   `(p+1)+x = p+(1+x)` shift).
3. **The aggregate-level cert is composition by citation**:
   `lowered_br_z` = ∀ bs x0 m0 c restfs, under the rep premise + the
   machine cert's bounds premises (k := br_len bs) —
   `DEC(call …(x0, len bs)… m0) = Some (Pair (x0 + br_z bs) m0)`.
   Proof: cite the GENERATED `lowered_lp_scan` (untouched), rewrite with
   the core theorem, rewrite the readback away by the rep premise. Zero
   new machine reasoning, zero kernel features — the refinement-lowering
   decomposition doing exactly what it was ratified to do.

Schema candidate for aggregate-typed module surfaces: rep-premise(s) +
inherited bounds premises + list-spec RHS. Mechanization path: std/str
ops whose loop shapes the fragment already covers, with `br_read`/`br_len`
generalized behind std/bytes' opaque surface. Not yet probed: two-region
ops (`bytes_eq` — two rep premises).

### 6q. The write-side aggregate probe — PASSED (2026-07-05)

`examples/byteswr_probe.shard` (69/0 first check, one name typo aside):
an op that WRITES an aggregate — `lp_fill`, its generated cert untouched.
Same decomposition as §6p, plus the write side's honest new content:

1. **The frame commute is where write-side disequalities live** —
   `mem_get (lp_fill m q v k) x = mem_get m x` under `x < q`: a read
   below the fill window commutes past the WHOLE loop. Induction with a
   premise-carrying IH; discharged by std/mem's `get_set_other` plus two
   farkas one-liners (`x<q → x<q+1`, `x<q → int_eq x q = False`). All at
   the SPEC level — the machine layer never sees a disequality.
2. **The write-side core theorem**:
   `br_read (lp_fill m p v k) p k = br_repl v k` under `0 ≤ v < 256` —
   the written window reads back as the list spec (replicate). Head byte
   = frame + `get_set_byte`; tail = IH. The value-range premises are the
   honest string story: in-range bytes read back exactly; out-of-range
   stores would surface mod-256 in the spec.
3. **The observation form**: since a cert's conclusion is one equation
   about the call, the write-side aggregate statement composes through a
   total extractor — `br_read_call (call_fn_mem …) x0 k = br_repl x1 k`:
   "the observable readback of running the shipped artifact IS the list
   spec". Cites `lowered_lp_fill`, then the core theorem. Zero new
   machine reasoning.

With §6p (read side) and §6q (write side) both passing first-check, the
aggregate schema has two of its three shapes; the remaining probe is
two-region ops (`bytes_eq` — two rep premises, disjointness question),
after which std/str mechanization has its full template set.

### 6r. The two-region aggregate probe — PASSED (2026-07-05)

`examples/bytescp_probe.shard` (76/0, FIRST check, zero fixes): one
region READ while another is WRITTEN — memcpy, via `lp_copy`'s generated
cert untouched. The disjointness question settles cheaply:

- **Disjointness is ONE linear premise** — `s + k ≤ d` (source window
  below dest window) — living at the SPEC level like every write-side
  fact. Read-only two-region ops need none (two readback equations
  cannot conflict); it is the read-while-write shape that pays, and it
  pays exactly two frame lemmas: `bc_swin` (a read WINDOW survives one
  store above it — window-level `get_set_other`) and `bc_frame` (a read
  cell below the dest survives the whole copy — §6q's commute, verbatim
  shape).
- **The memcpy theorem**: `br_read (lp_copy m s d k) d k = br_read m s k`
  under `s + k ≤ d`. Head byte = frame + `get_set_get` (writing a READ
  reads back exactly — NO value-range premises, reads are bytes);
  tail = IH + `bc_swin`.
- **The full two-rep-instance cert** (`lowered_bc_copy`): rep premise at
  the SOURCE (`br_read m0 x0 (br_len bs) = bs`), rep in the CONCLUSION
  at the DEST — the observable readback at x1 of running the shipped
  copy artifact IS `bs`. Cites `lowered_lp_copy` + the memcpy theorem +
  the rep premise.

The aggregate schema's template set is COMPLETE: read (§6p), write
(§6q), two-region read-while-write (§6r) — three probes, three
first-check passes, every generated machine cert untouched, every frame
fact and disequality confined to the spec level. The probes stack
(`bytescp` imports both predecessors' kits: `br_read`/`br_len`/
`br_read_call`/`bw_lt1`/`bw_ne`/`bw_ltp1` all reused) — the shape of the
abstraction library std/str mechanization will extract.

### 6s. The meta stdlib — plan + invoke graduate to modules (2026-07-05)

The first residents of **`meta/`** — the meta stdlib: interfaces and
helpers for programs that manipulate shard under the kernel's own
datastructures. Rationale: build.shard implementations were importing a
tool's internals (`tools/low/plan.shard`) for their own interface
vocabulary — backwards; the vocabulary and the invocation pattern are
library surface, the emitters and gate drivers are tools.

- **`meta/plan`** (from tools/low/plan.shard) — the build-plan
  vocabulary. A PURE-VOCABULARY module: the interface carries everything
  (transparent `Plan`/`PMod`/`PArt`/`PVec` types + the `Target` record
  and its machine-proved law family — producers construct plans, the
  driver destructures them; nothing to hide), and the impl file is just
  the run-mode entry re-importing the interface. First directory module
  exporting a transparent datatype family.
- **`meta/invoke`** (from tools/invoke) — dynamic invocation. The
  interface is stated entirely in KERNEL vocabulary (`Module`/`FnDef`/
  `Expr`/`QName`/`World` — the req-scope gate names the whole kernel/
  crate as the trust floor, so a mod.req may import it); the ten public
  fns are `sig fn`s, and the impl's helpers (`fns_named`,
  `ctor_in_defs`, `inv_budget`) drop OFF the surface — the eval.shard
  main-collision law is now structural: consumers get the interface,
  they cannot drag eval in by accident.

Both modules live outside kernel/ (engine stamp) and outside tools/
(they are libraries, not apps). Consumers rewired: the lowbuild driver,
all four mod.build entries, the invoke probe — bare-module imports +
`(:: meta plan *)`/`(:: meta invoke *)` use lines. Validation: all four
driver-rendered plans **byte-identical** to pre-graduation baselines;
meta/plan 19/0, meta/invoke 72/0, every consumer gate green first run;
invoke probe 4/4; gate_sweep (now pinning both meta impls) + corpus
green.

Deliberately NOT graduated yet: `tools/low/doc.shard` (generic rope +
path utilities — meta-shaped, next candidate when a second consumer
appears), certnav's generic raw-SExpr layer (entangled with cert-shape
logic), and `schema`/`proof`/`lin` (emitter policy, correctly tools).

### 6t. The readback abstraction graduates to std/mem (2026-07-05)

The §6p-§6r probes' shared kit resolved to **option A — std/mem's
surface** (ruling: the stdlib modules are there to be expanded over
time). The composition argument forced *some* single home: the rep
relation must be ONE function with ONE qname, or certs from different
modules state rep facts about definitionally-equal-but-distinct
functions and cannot cite each other.

std/mem's surface grows the aggregate readback view, exactly parallel
to the LE word view:

    (sig fn mem_read ((w Nat) (m Mem) (a Int)) (List Int))
    mem_read_z / mem_read_s          — exported defining equations
    mem_read_set_below / _set_above  — a store OUTSIDE [a, a+w) is
                                       invisible to the readback

The framing laws are the probes' window lemma (bc_swin) made surface
law; the impl proofs are the probe proofs ported (~verbatim — the
above case is bc_swin's induction, the below case reuses std/order's
`lt_implies_neq_flip`). std/mem impl 66/0.

**The retrofit is the adequacy check**: all three probes were weaned
off their local `br_read` onto the opaque surface — every `unfold
br_read` became a `mem_read_z`/`mem_read_s` defining-equation rewrite
(the weaning technique, now exercised across the rep boundary), and
`bc_swin` + its `bc_ne2` one-liner were deleted in favor of citing
`mem_read_set_above`. The probe suite passed **74/0 on the first
check** (76 minus the two graduated claims); wasm_diff_run closure
242/0; std/mem five-gate build + corpus green. This is precisely the
posture std/str's certs will take, so the surface is proven adequate
before mechanization starts.

Still probe-local, by design: `br_len` (list vocabulary — a std/list
question), `br_read_call` (names the machine call's result shape —
graduates toward the wasm model's kit with the std/str slice), and the
per-op frame commutes (bw_fill_frame / bc_frame — they are about the
ops, not the abstraction).

### 6u. std/str ships — the first emitter-generated module artifacts (2026-07-05)

The mechanization the whole runway pointed at: **std/str is the first
module whose wasm pieces are EMITTER-GENERATED end to end** and whose
consumer-facing contract is an aggregate rep cert in the module's own
opaque vocabulary. The file set (the mem.wasm.shard convention —
module-adjacent, consumers import by path):

- **`std/str/str.lowsrc.shard`** — the lowering source: `sc_copy`
  (lp_copy's exact two-region template shape). Copy is deliberately the
  ONLY op v1: it is the one string op the loop fragment covers whose
  spec is honest surface vocabulary ("materialize a string's bytes at
  another address"). NUL-scan is C-string vocabulary, not std/str's;
  `bytes_eq` needs a new machine shape (fragment growth); `bidx` as a
  single-load piece needs an idx↔nth law on std/bytes' surface first.
- **`std/str/str.wasm.shard`** — GENERATED by tools/lowergen (49/0,
  machine proofs first generation; `lowered_sc_copy` = lp_copy's cert
  shape verbatim).
- **`std/str/str.rep.shard`** — the aggregate article (57/0 first
  check): the §6r decomposition made module-grade. `sc_frame` (the
  frame commute), `sc_copy_read` (the memcpy theorem, citing std/mem's
  `mem_read_set_above` — the §6t graduation carrying its weight), and
  **`lowered_str_copy`, the module contract**:

      (mem_read n m0 x0) = (list_of_bytes (bytes_of st))   [rep at src]
      + bounds + x0 + n <= x1                              [disjoint]
      ⊢ mem_read_call (call_fn_mem … (x0, x1, n) … m0) n x1
          = (list_of_bytes (bytes_of st))                  [rep at dst]

  The budget n needs NO length premise: mem_read n returns exactly n
  bytes, so the rep equation itself forces int_of_nat n = str_len st.
- **`std/str/mod.build.shard` + `lowbuild.sh`** — module "stdstr" via
  the v2 build form; five gates green FIRST RUN (regen byte-identical,
  schema, kernel 49/0 + 57/0, bytetie, V8 3/3). Corpus-pinned.

`mem_read_call` graduated to models/wasm/loopkit (the observation
extractor names the machine call's result shape — the wasm model's kit
is its home); both write-side probes weaned onto it.

En route, two emitter bugs any nested module would have hit: `until_dot`
truncated paths at the FIRST dot (mangling dotted basenames like
`str.lowsrc`, which key loader modules — mem.wasm was already one), and
the
generated headers hardcoded depth-1 `../` import prefixes. Fixed in
tools/low/doc.shard (`until_dot` = strip from the LAST dot; new `updots`
= computed "../" chain) and lowergen's header builders; all three
example fragments regen **byte-identical** under the fixed emitter, so
the fix is proven behavior-preserving for everything already shipped.

### 6v. The comparison family — Ring 0 slice A (2026-07-05)

The model's op set grows its unsigned comparison family: `Bop` gains
`BNe`/`BLeU`/`BGtU`/`BGeU` (with encoder bytes 0x47/0x4D/0x4B/0x4F)
alongside the founding `BEq`/`BLtU`. Model values live in `[0, 2^32)`,
so Int `lt`/`le` ARE the unsigned orders — `bop_val`'s new arms are
one-liners and every existing proof is untouched (the whole wasm
article set re-verified green, zero edits). Signed variants wait for a
consumer.

The vocabulary is five tables wide, all extended in step: `bop_val`
(semantics), `enc_bop` (bytes), bytetie's `rbop` (cert readback),
wasm_weld's `r_bop` (rendering), and schema's `cond_op` (the emitter's
condition fragment, now `int_eq`/`lt`/`le` — surface `gt`/`ge`/`ne`
have no prim spelling, so `le` completes what the emitter can meet).
The generated diamond proof is CASE-ON over the SURFACE condition, so
widening `cond_op` needed zero new proof machinery: `lg_le` joined the
lowergen source set and its cert checked first generation, five gates
green (regen byte-identical modulo the new cert, schema 14/14, kernel
47/0, bytetie, V8 replay).

Evidence: `examples/wasm_diff_run.shard` gains `cmpmod` — one function
per comparison op, seven vectors each, straddling the 2^31 SIGNED
boundary both ways so a signed-variant encoding slip (lt_s 0x48 for
lt_u 0x49) flips an expected value. 96 vectors agree under V8.

### 6w. Division and the trap leg — Ring 0 slice B (2026-07-05)

`Bop` gains `BDivU`/`BRemU` (encoder bytes 0x6E/0x70), and with them
the model's first PARTIAL operator: `bop_val` now returns
`(Option Int)`, `None` = the op itself traps. The divisor guard is
spelled `(le 1 b)` — the PRE convention certs will rewrite directly
when the emitter fragment grows division. `do_bin` threads the None
into its existing underflow path, so a zero divisor becomes `OTrap`
with zero new interpreter arms. On the nonneg range `ediv`/`mod` ARE
`div_u`/`rem_u` — machine spelling = spec spelling, no wrap.

The Option-ization is FULLY proof-neutral: for a ground op constructor
the extra Some-match reduces transparently, so every wasm article —
hand pieces, probes, all generated cert files — re-verified green with
zero edits, and all five build pipelines regenerated byte-identical.

The differential's TRAP LEG went live here: wasm_diff.mjs's plan
grammar always allowed `-> None` but the replayer treated every engine
trap as FAIL; now `None`-expected + engine trap = agreement. `divmod`
joins the plan (f0 = div_u, f1 = rem_u, nine vectors each: zero
divisors both shapes, the 2^31 signed boundary where div_s answers
differently, u32 extremes). 114 vectors agree under V8.

NOT in this slice: `ediv`/`mod` in the emitter fragment. A division
node's guard makes `bop_val` stick mid-compute on the symbolic `if`,
so the pure-fragment proof needs the mem-fragment's staged
compute/rewrite interleaving — a fragment-growth slice with its own
probe (§7.7 residue), not a model slice.

### 6x. Bitwise ops close the op set — Ring 0 slices C/D (2026-07-05)

`Bop` gains `BAnd`/`BOr`/`BXor` (encoder bytes 0x71/0x72/0x73). The
arms apply the kernel's `band`/`bor`/`bxor` prims directly — same
qname on the machine and spec side, no wrap: on the nonneg range the
prims ARE the machine ops, and the in-range closure is backed by the
`kernel/facts.shard` defining recurrences. The differential's bitmod
leg (27 vectors: complementary masks, alternating-bit patterns,
identity/annihilator operands, extremes) is thereby also a live
engine differential for the kernel's bitwise prims. 141 vectors agree
under V8. Proof-neutral as with §6v/§6w — every article green, all
builds byte-identical.

Slice D is a spelling, not an op: constant shifts are
`(* x 2^k)` / `(ediv x 2^k)` over existing ops — shift OPCODES (and
dynamic shift counts, which need the mod-32 masking story) wait for a
consumer. With this the Ring 0 op-set gap is CLOSED: the model speaks
add/sub/mul/div_u/rem_u, the full unsigned comparison family, eqz,
and the bitwise trio.

What certs can SAY about bitwise nodes is a separate ledger:
reasoning about `band`-containing spellings needs the std/bits theorem
library over the kernel recurrences — LANDED same day (std/bits:
pow2 + the range/closure family band_le_l / bor_le_pow2+bor_le32 /
bxor twins + literal shifts PROVEN as mul/ediv, all theorems, zero
axioms; consumer demo examples/bits_demo.shard cites the surface
through the granted interface). Two proof-engineering findings there
with reach beyond bits: the arith backend's goal negation is
INTEGER-TIGHT (¬(0≤q) reads q≤−1), which closes quotient bounds and
quotient composition (`ediv (ediv a N) 2 = ediv a 2N`) at literal
divisors by pure farkas; and div-facts — not the kernel mod axioms —
is the integrality gateway at literal divisors (le-spelled remainder
bounds, variable quotient). The mask-mod bridges
(`band x 255 = mod x 256`) need the PARITY of symbolic terms, which
linear reasoning cannot see — the unblock was a euclidean completion
(ediv_mod_id + the uniqueness pair div_unique/mod_unique) plus the
mul ring laws in kernel/facts.shard, reviewed and LANDED same day
(15 axioms total, probe-differentialed); std/bits then proved
mask_pow2/mask_byte and the general symbolic-k shl_pow2/shr_pow2 on
top. Emitter bitwise support = wiring std/bits range facts into
lin.shard's intervals; a later slice.

### 6y. Emitter ediv/mod — the staged pure walk (2026-07-05)

The §6w fence falls: the pure fragment speaks `ediv`/`mod`, lowered to
`BDivU`/`BRemU` behind the divisor-guard PRE `(= (le 1 B) True)`. The
design problem was the proof, not the code: a division's guard makes
`bop_val` reduce to `(if (le 1 B) (Some (ediv A B)) None)`, and with
`B` symbolic the machine fold JAMS at that instruction — the pure
fragment's one-shot `(compute lhs)` no longer runs the whole machine.
The answer is the mem fragment's stage law transplanted (probe:
`examples/divfrag_probe.shard`, eight hand-played instances, all
first-check): per div site in code order, flush the wrap events
materialized so far (their all-occurrence rewrites clean the divisor
spelling), rewrite the guard premise into the stuck `if`, resume
`(compute lhs)`. Three structural facts make the slice small:

- **The div node needs no wrap event.** `bop_val` returns the
  `ediv`/`mod` term unwrapped — machine spelling = spec spelling. Only
  the guard premise is new vocabulary.
- **A literal divisor's guard computes away.** `(le 1 7)` reduces
  mid-compute: no stick, no stage, no premise — and `(ediv x 2^k)`,
  slice D's constant right-shift spelling, is thereby emittable for
  free. A zero-literal divisor refuses at generation (the cert would
  be vacuous — the machine traps).
- **Event collection rides the CODE walk** (`gen_e` now returns the
  event stream), not the substituted tree: only the code walk can tell
  a let-SHARE (one materialization — a re-fire would find no site)
  from a textual RECOMPUTATION (a fresh wrap after any earlier
  all-occurrence fire — a re-fire is required). Wraps batch per
  SEGMENT (between div sticks), dedup within the segment, re-enter
  pending across one; a repeated divisor spelling keeps ONE premise
  and re-fires it per site.

Emitter instances (`lg_div*` in `examples/lowergen_src.shard`, mirroring
the probe matrix): bare div/rem, wrapped operands (flush-before-guard),
literal divisor, div feeding arith, let-shared div, two sites at one
divisor spelling, div-of-div. All eight machine-written proofs passed
FIRST GENERATION; the three existing fragment outputs (pure, mem, loop)
and std/str regenerated byte-identical under the reworked walk before
the source set grew. Fuel formula `2·instrs+3` unchanged (a stage is a
proof event, not a machine step).

v1 fence: div fns are straight-line/let only — the region (if) templates
and the stage law have not been composed yet (a div inside an arm means
staging inside the arm's 4-step template; do it when a consumer wants
it). Behind the same fence: divisors at aggregate readbacks, and
div/mod in mem/loop bodies (the mem walk already has staged machinery —
composing the two guard kinds is mechanical but unproven).

### 6z. Emitter bitwise — transparent ops (2026-07-05)

The pure fragment speaks `band`/`bor`/`bxor`, and the slice is SMALLER
than division because the ops are structurally transparent: `bop_val`
applies the kernel prims directly — total, no wrap, no guard — so a
bitwise node sticks nowhere, contributes NO premise and NO discharge
event, and the machine spelling IS the spec spelling (the folded prim
term just rides the residue). Probe `examples/bitfrag_probe.shard` (six
hand instances, all first-check, corpus-pinned); mechanization =
`top_of` in the op vocabulary, an eventless `gen_e` arm, and one
`nodes_e` change (bitwise heads recurse for children but are never wrap
sites). Because nothing sticks, bitwise composes everywhere the
fragment already goes — lets, SHARED lets, ifs (unlike div: the region
path handles it, no staging), div divisors (`(ediv a (bor b 1))` — the
§6y guard premise lands at the already-clean bor spelling), and arith
over bitwise (the wrap pair fires at a band-containing spelling;
`lg_bifp` exercises the §6j condition-relative machinery over one).
Nine `lg_b*` emitter instances, all machine proofs FIRST GENERATION;
five gates green; V8 replays the bitwise vectors (mask patterns,
complementary halves, u32 extremes).

Bounds on arith-over-bitwise nodes are CONSUMER premises, discharged
through std/bits (`band_le_l`/`bor_le32`/`mask_byte` … — the
`bits_demo` pattern). Auto-discharge (PAuto for bitwise-over-reads in
the mem fragment) is deliberately NOT in this slice: it needs per-atom
intervals in lin.shard's poly kit (atoms are hardcoded byte reads
today), a citation-style helper template alongside the farkas one, and
literal-width corollaries on std/bits' surface (its width facts speak
symbolic `pow2 k`; the emitter wants literal 255/65535 bounds). Do that
wiring when a mem-fragment consumer actually masks bytes; the fragment
substrate no longer blocks it. Also behind the fence: bitwise in
conditions (cond operands stay params/aliases/literals) and bitwise in
mem/loop bodies (same op-classing, unprobed).

### 6aa. The FLAG shape — bytes_eq / two-region comparison loops (2026-07-05)

The loop fragment grows its fourth Int-return template, and two §7.7
residue items fall at once: conditions that compare TWO reads, and
twins for NON-RETURNED advancing accumulators. The shape:

    (fn NAME ((m Mem) (a1 Int) … (an Int) (k Nat)) Int
      (match k
        (Z 1)
        ((S k2) (if (int_eq (mem_get m aA) (mem_get m aB))
                  (NAME m U1 … Un k2)
                  0))))

— walk two regions in lockstep, exit 0 at the first mismatch, return 1
on budget exhaustion (bytes_eq/memcmp). Probe
`examples/beqloop_probe.shard` (worker + theorem hand-played by rule,
first check; rides wasm_diff_run's check closure); mechanization in
lowergen (`lb_*`), `lp_beq` in the source set, all machine proofs FIRST
GENERATION; five gates green, V8 replays equal/mismatch-first/
mismatch-last/zero-budget/self-compare vectors.

Template deltas over the §6m scan:

- **Two-read condition.** The exit test is `I32Bin BEq` over two loads;
  `bop_val BEq`'s residue `(b2i (int_eq a b))` collapses under the case
  hyp exactly like scan's `I32Eqz`. Each read discharges the standard
  address-guard pair, so both accums carry nonneg+range premises
  (nonnegs first, param order) and the read stages fire in CODE order
  (`lp_stage` at A's indices, then B's — re-firing handles A = B).
- **The flag result local.** The source returns LITERAL 1 or 0, so no
  accum local carries the result. The machine materializes it in an
  extra local at index n+1: `I32Const 1, LocalSet` BEFORE the loop
  (locals zero-init and a zero-iteration call must read 1), then set to
  the BEq bit each iteration; the mismatch exit is the flag's `I32Eqz`.
  Its exit value IS the spec fn — scan's returned-accum mechanism, no
  twin. The pre-loop init costs the theorem +2 fuel over scan's +4 (one
  per instruction in the eval_seq spine); the worker formula
  S^(instrs+4) is unchanged. CONSEQUENCE of the encoding: the arm
  literals are PINNED to 1/0 — the flag holds the comparison bit, so a
  source spelling `(Z 7)` refuses. General literal arm values would
  need per-exit-path epilogues; do it when an op wants it.
- **Twins for every advancing accum.** On an early exit NO accum is a
  closed form of the inputs: the emitter writes one spec twin per
  advancing accum (`NAME_t<i>`, exit arm returns the accum's OLD value)
  plus the §6m counter twin (`NAME_kf`, exit arm keeps the
  undecremented count) — machine-written recursive fns, gate-3-checked
  like any article. The worker's exit locals read
  `(t0 … kf spec-call)`.

v1 fences: the two read accums stride +1, every other accum static;
condition operands are exactly the two reads (no literal leg — that
stays the §6m scan; no read-vs-arith); Z arm literal 1, else arm
literal 0. Refusals are loud for all of them.

**The std/str consumer (the slice's point).** std/bytes' surface grew
`bytes_eq ((a (List Int)) (b (List Int))) Bool` — pointwise byte-list
equality as an opaque sig + four defining-equation requirements
(`bytes_eq_nil`/`bytes_eq_nil_cons`/`bytes_eq_cons_nil`/
`bytes_eq_cons`, the bytes_ok pattern; fulfills auto). On top of it
std/str ships `sc_eq` (str.lowsrc → GENERATED lowered_sc_eq in
str.wasm.shard) and str.rep.shard states the aggregate pair:

- `sc_eq_read` — the memcmp theorem: `sc_eq m s d k =
  (if (bytes_eq (mem_read k m s) (mem_read k m d)) 1 0)`. PREMISE-FREE:
  read-only means no frame commute and no disjointness — the two
  windows may overlap arbitrarily (compare §6q/§6r, where every write
  slice paid frame lemmas). Both sides case on the same head-byte
  test; mem_read_s/bytes_eq_cons open the readbacks under the case
  hyp; the IH closes the True arm.
- `lowered_str_eq` — the EQUALITY MODULE CONTRACT: rep premises for
  st's bytes at x0 AND su's bytes at x1, and the shipped artifact
  returns `(if (bytes_eq st-bytes su-bytes) 1 0)`. The ONE budget n
  across both rep premises pins the strings to equal length — the
  honest precondition of a fixed-budget comparison. Proof = cite
  generated lowered_sc_eq + sc_eq_read + both rep premises; zero new
  machine reasoning (the §6r citation discipline, third instance).

mod.build ships the piece as its own module ("stdstreq" — leaf certs
pin slot 0, so one artifact per module), byte-tied by gate 4's second
TIE/MOD pair. str.wasm.shard and lowergen_loop_out.shard both stayed
pure appends; the three sibling fragment outputs regenerated
byte-identical under the emitter rework before the source sets grew.

### 6ab. Stride ≠ 1 — literal strides and generated helper certs (2026-07-05)

Incrementing accums take any positive LITERAL stride: `(+ a C)` with
C ≥ 2 recognizes as `LSIncK C` (stride 1 stays `LSInc`, its outputs
byte-identical). Probe `examples/strideloop_probe.shard` (the CELL-TAG
FILL — stamp the head byte of each 4-byte cell, the uniform-rep
cell-walk shape — hand-played at C=4, first check); `lp_tag` machine
proofs first generation after ONE template fix; five gates green.

The design fact: the loopkit's stride-1 helpers (`lg_a1_ms`/`lg_lo1`/
`lg_32`/`lg_kM`/`lg_shift`) are template constants, and they CANNOT
generalize to a stride binder — `c·n` is nonlinear, farkas has no
multiplier for it. As a generation-time literal, C is just a
COEFFICIENT, so the emitter writes per-(fn, stride) instances
(`lgs_<fn>_<C>_{a1ms,ac,lo,c32,kM,shift}`) into the output file with
farkas lists computed by closed formula:

    a1ms / ac -> (list 1 1 C C)     lo / c32 -> (list 1 1)
    kM        -> (list C 1 C 1)     shift    -> (list 1 1 C)

(mined once at C=4 in the probe; wrong-formula = loud gate-3 fail,
never unsound — the §6j computed-certificate discipline extended to
the loop fragment). The stride-C accum's range premise scales to
`(le (+ x (* C (int_of_nat k))) 65536)`; its exit local is
`(lg_advk x C k)` — the stride rides the new loopkit twin as an
argument. Stage/wrap/IH shapes are the stride-1 template verbatim with
the helper names swapped (`lp_kstage`/`lp_khwk`/`lp_khw`).

The one template fix: the worker's rhs opening unconditionally
unfolded `lg_adv`, which FAILS LOUDLY on a loop with no stride-1 accum
(`lp_tag`'s only mover is the stride-4 pointer) — all three twin
unfolds (`lg_adv`/`lg_dec`/`lg_advk`) are now presence-conditional.

v1 fences: stride ≠ 1 is Mem-return-loop-only (Int-return loops keep
±1 accums — loud refusal; the sum invariant's 256-scaling and a
stride-C read would need a combined k-scaled premise, do it with a
consumer); decrements stay stride 1; scan/flag shapes reject LSIncK
through their existing stride gates.

### 6ac. Calls in loop bodies — the fragments compose (2026-07-06)

The last consumer-backed §7.7 item: a Mem-return loop whose
per-iteration WORK is a CALL to an earlier same-file mem-fragment fn.
Instance: `cw_put` (2-byte cell store, an ordinary §6n two-store fn) +
`cw_fill` (stride-2 loop calling it) — the uniform-rep cell runway.
Probe `examples/callloop_probe.shard` (59/0 FIRST CHECK, all farkas
and fuel guesses held); machine proofs FIRST GENERATION;
`examples/lowbuild_call.sh` five gates green first run (V8 5/5); the
`clmod` differential leg is the first engine validation of
Loop/Br + Call + Drop in one body.

- **Groundwork — the model grew wasm's `Drop` (0x1A)**: the callee's
  return must be discarded before `Br 0` or the re-entry stack does
  not match the IH's. Parking it in a scratch local (the straight-line
  consumers' idiom) would cost a per-fn memory-threading twin for the
  scratch's exit value inside an induction; real wasm has drop, and
  with it the loop body is `Call 2, Drop` — locals untouched across
  the call, IH direct. Vocabulary growth in step (§6v discipline):
  eval arm + encoder byte + bytetie reflector arms; proof-neutral
  (zero article edits); `dropmod` differential leg.
- **STRUCTURAL form, LINK-file residency**: the §6f portability limit
  is real — loop call sites fire at a different fuel every iteration,
  so behavior premises pinned to the goal's one slack binder cannot
  express them. The loop's whole unit (body/func fns, stride kit,
  premise helpers, worker, theorem) is emitted into the LINKED file:
  the callee's literal rides the module spine
  (`[mget, mset, callee@2, self@3 | restfs]`), and the file
  regenerates on callee edits — exactly the linked file's existing
  contract. The portable file stays byte-stable.
- **The bridge stage in the S case**: after the guard, compute stops
  at the folded `eval_call` (every S-case compute carries
  `(stop eval_call eval_loop lg_fuel)`); `call_bridge` fires with the
  callee's LINKED cert discharging the run premise. Fuel composes by
  pure slack unification — the callee's `S^NN c'` tower unifies under
  the iteration fuel with `c' := S^(j-NN) (lg_fuel k2 c)`. Charge
  formula: worker `S^(instrs + 4 + (2·NN_callee + 1))`, theorem +4.
- **Spelling alignment across the call**: the callee cert returns its
  memory effect as the FOLDED spec application (`cw_put m0 x0 x1`) —
  exactly the recursion argument the spec's own rhs unfold produces.
  Zero collapse lemmas at the boundary; the IH fires at the folded
  term.
- **Callee premises at loop-varying args** (the registry): when the
  mem fragment emits a Mem-return fn it records a `CInfo` — name,
  self literal, tower, param count, the linked cert's premises as
  `CPre` items (enumerated in the linked claim's exact premise order —
  drift fails the kernel gate loudly at the citation subproofs), and
  the return-value expr. At the loop site each premise is instantiated
  (callee param ↦ site arg) and classified: exact loop-premise
  spelling → direct rewrite; ground → compute; linear in one
  incrementing accum → a generated `clh_<fn>_<p>` helper claim with
  computed farkas (uppers `(1 1 C C)` from the accum's scaled range
  premise, lowers `(1 1)` from its nonneg; C = the accum's stride),
  cited by a hoisted have before the bridge. A footprint byte at
  offset J needs `J + 1 ≤ C` — the callee must fit the stride
  (loud refusal otherwise).
- v1 fences (loud refusals): work = ONE call, callee = an EARLIER
  same-file mem-fragment Mem-return fn (source order is registration
  order); site args are accums or u32 literals; callee premises may
  touch only incrementing accums; the callee's return value must be a
  param or literal (a read-returning callee like `mg_swap` would need
  a memory-threading twin — wait for a consumer).

### 6ad. The manifest gate + gate hardening (2026-07-06)

The review round after §6ac audited the end-to-end trust story of a
shipped artifact set and found one user-facing seam: the five gates
verified the SET's internal consistency, but the MANIFEST — the first
thing a third party consumes — was checked by nothing. wasm_diff.mjs
replays `MOD`/`CASE`/`MEMCASE` lines and never reads `ARTIFACT` lines,
so a `PA` entry could name the wrong cert or the wrong export index
with all five gates green: the byte tie binds module name ↔ claim ↔
bytes, and every OTHER manifest field was decoration. A consumer
trusting the handed manifest could invoke export `f2` believing it
certified `cw_fill` and get `cw_put`'s behavior.

- **`tools/lowcheck/manifest.shard` = the binding gate** (the third
  certnav consumer; same PCC stance — the plan's producer is never
  trusted). Per `ARTIFACT` line: `cert=` must be
  `lowered_<name>`/`linked_<name>` (the naming rule the byte tie
  already rides), `certfile=` must be one of the build's GATED cert
  files (passed by the calling script — the binding is to what the
  KERNEL/SCHEMA gates actually checked, not to whatever the plan
  says), `model=` must be the gated model, and `export=fN` must equal
  the pinned function-index literal in the named claim's conclusion,
  read from the cert file RAW. A plan with zero `ARTIFACT` lines is
  refused — no pass by vacuity. Rides gate 4 in every lowbuild script
  (the rendered plan is in hand there). Negative fixture
  `examples/manifest_rejects.txt` (wrong index / wrong cert name /
  ungated certfile) is corpus-pinned REFUSED; the app joins
  gate_sweep's type-gated set.
- **Script hardening** from the same review: `set -euo pipefail` in
  all six build scripts (the KERNEL gate's `CHECK | tail -1` pipeline
  used to discard the checker's exit status — the verdict was only
  the grep on the tally line), and a missing `node` now REFUSES
  (exit 1) instead of printing SKIPPED and exiting 0 — the ENGINE
  gate is the one reality tie (§6, P2c), so a box without node must
  fail the build rather than silently record success.
- **Doc alignment**: the ratified-header pin list counts all six
  builds; lowergen's file header dropped the stale P2c PRE caveat
  (resolved by §6j); docs/ISA.md's trust-leaf bullet now carries the
  resource-exhaustion scope note (a real engine may trap on stack
  exhaustion where the model, which has no stack bound, says `Some`).

Recorded from the same review, deliberately NOT this slice: the
rep-swap emitter coupling (lowergen hardcodes std/mem's v1 literals
and tower budgets — a real std/mem rep swap today edits the emitter,
§6d's byte-stability claim covers only the portable file), the
front/back split of lowergen (graduating the recognition vocabulary
LRec/LIRec/RT into a named canonical-form library — the seed of the
common lowering step), and the latent `pr_e` unknown-head `"?"`
spelling (two distinct exprs both spelling `"?"` would dedup to one
premise BEFORE the kernel sees anything; unreachable in the current
fragment, structurally silent if it ever isn't).

### 6ae. The front/back split — tools/low/shape.shard (2026-07-06)

The §6ad review's architectural verdict acted on: fragment growth had
been appending shapes to one 5,100-line app, and the recognition
vocabulary — the part of the emitter a second target consumes UNCHANGED
— had no named home. `tools/low/shape.shard` is now the canonical
source-form library, the FRONT END of the certifying emitter and the
seed of the common lowering step (the §8 contract bounds what a target
model provides; shape.shard is the layer that sits entirely above it).

What graduated (~880 lines; lowergen 5,098 → 4,250):

- **Fragment dispatch scans** — `has_div`/`any_div` (the §6y staged-walk
  dispatch), `dv_has_if`/`any_if` (the div+if fence), `fn_is_loop` (the
  loop-vs-mem-walk dispatch).
- **The branch/region tree vocabulary** — `RT`/`TRes` +
  `rt_nxt`/`rt_count`/`rt_code`/`rt_prepend`. Payload-generic: the tree
  STRUCTURE mirrors the source's branch shape; the code payload is
  whatever the back end fills it with (`Doc` is opaque rope here).
- **The loop-shape recognition layer** — the value/stride/argument
  vocabulary (`LVal`/`LStr`/`LKArg`/`LUpd`) and the shape records
  (`LRec`, `LIRec`) with every recognizer (`lp_*`, `li_*`, `lc_*`,
  `lb_*`): kernel-term inspection only, loud LRErr/LIErr refusals, zero
  Doc construction.
- **The callee registry** — `CPre`/`CInfo` + `reg_find`/`reg_add`/
  `cpre_bound` (the type vocabulary; the back end populates entries
  when it emits a callee and reads them at loop call sites).

The rule that drew the line: a definition graduates iff it reads kernel
terms and produces a target-independent description — it renders
neither instruction text nor proof text. Two consequences worth
pinning: (1) the pure/if fragment has NO separate recognizer to
graduate — `gen_e`/`gen_t` fuse recognition with code emission by
design (the walk refuses out-of-fragment bodies as it compiles), and
their front-end product, the canonical substituted expr, is already
schema.shard vocabulary; (2) `fn_is_mem` turned out to be dead (retired
by the §6o mod.build-v2 rework) and was deleted, not graduated.

Acceptance, §6g's bar: all six five-gate builds green with REGEN
byte-identical — the split provably changed no output. shape.shard
rides lowergen's type-gate closure (the tools/low kit convention).

The survival map this makes structural (the §6ad reviewer's estimate,
now a file boundary): a second target imports shape/schema/lin/doc
unchanged; proof.shard's wrap-event/bridge templates are wasm-shaped
and would fork; lowergen is the wasm back end entirely. NOT graduated,
recorded as the next candidates when a second target arrives: the
calls-in-loops discharge classifier (`lk_lin`/`LKD`/`lk_disch` —
generic classification currently interleaved with `clh_*` helper-cert
emission) and the premise walk over the region tree (`pw_*` — §6j
layout logic fused with wasm proof docs).

## 7. Open questions — triaged at ratification (2026-07-04)

None of these block the ratified form; they are the backlog the next
arcs draw from.

1. ~~The statement-generator enforcement mechanism~~ RESOLVED by P4a
   (regen + tools/lowcheck). ~~The cert↔binary byte tie~~ RESOLVED by
   §6i (tools/bytetie). ~~Corpus pinning~~ RESOLVED at ratification
   (run_corpus.sh build pins + negative fixture + kernel articles).
   The `bin`-gate wiring is RE-SCOPED, deferred: no `bin` artifact
   cites lowered certs today, and every build already passes the
   recognizer through its pinned lowbuild script — driver machinery
   with no consumer would be speculative. Revisit when the first bin
   ships a lowered binary (the wasm CLI runner milestone).
2. OPEN (uniform-rep arc): PRE-slot conventions for the uniform rep —
   the heap invariant's exact statement (bump-pointer validity +
   allocated-cells-never-rewritten).
3. ~~The footprint/framing lemma shape~~ RESOLVED by P1. Remaining
   tail (fragment growth): the above-footprint pointwise twin, and
   emitting the observational companions mechanically.
4. OPEN (decide when a consumer appears): trap-conditional variant, or
   do premises + interpreted fallback cover the roadmap?
5. OPEN (uniform-rep arc, v2): cross-rep conversion glue — certified
   adapter pieces between the uniform rep and hand-rolled reps.
6. OPEN (likely yes, confirm during the RS fragment): is one scalar +
   memory enough return surface? (Structured results live in memory
   under the uniform rep.)
7. Fragment growth — Int-accumulator returns (§6k), multi-store +
   decrementing accums (§6l), conditional early-exit scans (§6m), and
   write-then-read bodies (§6n) LANDED 2026-07-04. Notably, the
   condition-relative DISEQUALITY premises staged by §6j were needed by
   none of them — every slice kept machine and spec on identical
   spellings instead. Two-read conditions + twins at non-returned
   accums LANDED 2026-07-05 (§6aa, the FLAG shape — bytes_eq); nonzero
   scan literals LANDED 2026-07-05 (§6m update — memchr); stride ≠ 1
   LANDED 2026-07-05 (§6ab — literal inc strides, Mem-return loops,
   generated lgs_* helper certs); calls-in-loops LANDED 2026-07-06
   (§6ac — structural form, link-file residency, the fragments
   compose). Still open (all consumer-less, fenced): general literal
   arm values for flag loops (1/0 are pinned by the BEq-bit encoding),
   write-then-read inside LOOP iterations, stride ≠ 1 on
   Int-return/dec accums, read-returning/multi-call loop callees.
8. NAMED SLICE (2026-07-07, no consumer yet): the WORD fragment —
   lowering `std/word`-style modular types. Today both back ends
   accept only Int (+ Mem) and bridge unbounded spec arithmetic to
   wrapping hardware by the per-node premise pair + wrap_id collapse
   (§2, §6j). Word-typed code is the fragment where the machine op IS
   the source semantics (`u32_add a b` = `mod (+ a b) 2^32` = exactly
   `i32.add`), so the slice is PREMISE-FREE: no wrap pairs, no wrap_id
   events — the proof cites the surface defining equations
   (`u32_add_val` …) instead, never piercing the opaque type. The
   natural first consumer is std/rng's xorshift32. Width mismatches
   are the only real content: U32-on-wasm is 1:1; U32-on-x86 wants the
   encoder's 32-bit operand forms (non-REX.W `add` wraps at 2^32
   natively); U8/U16 anywhere = mask-after-op, which the `mod 256`
   spelling matches directly; x86 wants a U64 added to std/word.
9. DIRECTION (user, 2026-07-07): the FLAGSHIP lowering target once the
   x86 pipeline is operational is kernel/eval.shard itself — the
   certifying pipeline replacing the temporary native chain on its own
   interpreter. Kernel code cannot consume std/word's opaque
   rep-switching (layering: kernel is the trust floor), so the scalar
   plan is to grow the kernel's inner stdlib with PRIVATE u8/u32/u64
   built on the invariant refinement structure (`(refine BASE PRED)`):
   range invariants carried by the TYPE discharge the wrap premises at
   the source level, once, instead of riding every artifact cert as
   PREs — the lowering pipeline then picks hardware types for
   refinement-typed bindings directly. Distinct from item 8's modular
   Word semantics (wrap-by-definition, for code that WANTS wrapping):
   refined bounded Ints keep ordinary Int arithmetic + bounds, which is
   what interpreter code (fuel, indices, char codes, lengths) actually
   is. The refinements do not close under arithmetic (u32+u32 can
   exceed 2^32), so ops on refined types carry fit obligations
   discharged from the source invariant at construction sites — the
   measure-clause discipline's shape. Zero new kernel machinery:
   (refine …) exists; the types + op surface + law family are ordinary
   definitions (the std/bits precedent). If eval.shard's scalar
   traffic can be reasonably limited to those types, the scalar half
   of the flagship lowers neatly; the aggregate half (Expr trees,
   tries, allocation) stays with the uniform-rep arc (items 2/5/6).

## 8. The model-authoring contract — what a target ISA model provides

Ratified alongside the form. The lowering architecture is generic in
the target exactly to the extent that a target model supplies the
following; this list is what "add a new ISA" means, distilled from
everything the wasm pilot needed. A model is an ORDINARY shard library
— data types + total functions; zero kernel, loader, or checker
changes. (wasm: `models/wasm/wasm.shard`, encoder in `encode.shard`,
loop article in `loopkit.shard`.)

1. **A fuel big-step denotation** with the additive-slack discipline:
   an entry shaped like `call_fn` / `call_fn_mem` (args in, scalar [+
   memory] out, `Option` for exhaustion), where fuel is a DEPTH bound —
   recursive entry burns exactly one unit, so a loop's induction
   hypothesis respells at ANY sufficient budget — and every cert
   statement quantifies over slack (`(S^ K c)` towers, never exact
   fuels). Over-provisioned towers compose by pure unification; exact
   heights are never computed.
2. **Named SCC stop points**: the evaluator's recursion split into
   named members (wasm: `eval_call`, `eval_loop`, `eval_seq`) so proofs
   can stage — `(compute lhs (stop eval_call))` at call boundaries,
   fuel-fn stops in loop workers. An evaluator written as one opaque
   function cannot be composed against.
3. **A call-composition keystone** (`call_bridge`): folded call-entry =
   the callee's pushed denotation, under premises that all discharge by
   compute at concrete sites plus ONE behavior slot the callee's cert
   fills. Proven once per model, piece-independent. This is what makes
   consumer proofs cite callee certs instead of computing into bodies.
4. **The representation-collapse lemma family** (`wrap32_id`): per
   arithmetic op class, the lemma that collapses the machine's value
   representation back to the spec's, with positional range premises —
   the target of the emitter's discharge events (§6c stage law, §6j
   derived bounds).
5. **Literal-spelling discipline**: the model's value and instruction
   spellings must be stable under check-mode compute — statements match
   compute residues EXACTLY. Folded redexes only under `reduce`;
   literals, not nullary calls, in machine state; any ENC function that
   rides into machine state must be openable by defining lemmas
   (int_of_nat's opacity law, §6f).
6. **An encoder + an engine differential**: model terms → real target
   bytes, and a harness replaying model-computed vectors on a real
   engine (wasm: `encode.shard` + `wasm_diff.mjs`/V8). Kernel truth ≠
   engine validity — the typed-block finding (§6, P2c) was caught ONLY
   by this gate. The encoder also powers the byte tie (§6i).
7. **A memory denotation over the observational substrate** (targets
   with memory): std/mem's discipline — mask-on-read, no Mem equality,
   framed arbitrary-`m0` statements with ENC as an observation premise
   (§3/P1).

The emitter, checker, byte tie, build convention, statement schema, and
proof templates (tools/low) are target-generic against this contract;
what stays per-target is the model library itself and the fragment
walks' instruction selection. The expected common lowering step
(shard→imperative canonical shard, proven shard→shard) sits ABOVE this
contract and narrows what each target's walks must handle; whether its
output form coincides with any one target's shape is deliberately open.
