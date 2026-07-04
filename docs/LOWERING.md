# Lowered-conformance certificates — the standard form

**STATUS: DRAFT — discussion artifact, not ratified.** Companion test
articles: `examples/lowered_form.shard` (kernel-checked, 2 claims). This
document proposes the one formal object the arch-specific build paradigm
hangs from; everything else (the wasm lowerer, mod.build conventions, the
CLI runner, welds/linking) is engineering behind it.

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
- *PRE caveat (v1)*: arm range premises quantify over the whole
  contract — `(if (lt 0 x) (- x 1) 0)` would demand `0 ≤ x-1` globally.
  Arm expressions must be in-range on the full domain;
  condition-relative premises (dischargeable from the case hyp via
  generator-emitted farkas) are future work.

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
  shapes — read-value bounds stated as PREs, derivable via
  `get_lo`/`get_hi`, not yet auto-discharged).
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

Fragment fence (unchanged from §6f): Int-accumulator returns, stride
≠ 1, multiple stores per iteration, and calls in loop bodies
(structural-form-only when they come) are future extensions.

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

## 7. Open questions (the back-and-forth queue)

1. ~~The statement-generator enforcement mechanism~~ RESOLVED by P4a:
   both — regen (producer determinism) AND structural recognition
   (tools/lowcheck, consumer-side). ~~The cert↔binary byte tie~~
   RESOLVED by §6i (tools/bytetie, the fifth gate). Remaining tail:
   wiring lowcheck into the `bin` gate.
2. PRE-slot conventions for the uniform rep: the heap invariant's exact
   statement (bump-pointer validity + allocated-cells-never-rewritten).
3. ~~The footprint/framing lemma shape~~ RESOLVED by P1 (§3: framed form +
   observational companions). Remaining tail: the above-footprint
   pointwise twin, and whether the emitter generates the companions
   mechanically (it should — same schema machinery).
4. Trap-conditional variant: wanted at all, or do premises + interpreted
   fallback cover the roadmap?
5. Cross-rep conversion glue (v2): certified adapter pieces between the
   uniform rep and hand-rolled reps.
6. Multi-result / memory-only functions: `call_fn_mem` returns one scalar
   + memory; is that enough surface for the RS fragment? (RS fns return
   one value; structured results live in memory under the uniform rep —
   likely yes.)
