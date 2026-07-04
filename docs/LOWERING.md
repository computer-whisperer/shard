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

V1 gaps, on record: (a) the build file's func literals are copied from
the generated certs — the cert↔binary tie should be CHECKED (re-encode
the claim's module literal, compare bytes), not maintained by hand; (b)
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
  indirection, re-fire, store, copy, bump) → `lowergen_mem_out.shard`:
  **all 9 machine-written proofs passed the kernel on first
  generation**; `examples/lowbuild_mem.sh` = the four gates
  (REGEN → SCHEMA → KERNEL → ENGINE, V8 11/11 incl. store-truncation
  and bump-wrap edges); build file `lowergen_mem_src.build.shard` uses
  spec-side expected values and readbacks throughout.

## 7. Open questions (the back-and-forth queue)

1. ~~The statement-generator enforcement mechanism~~ RESOLVED by P4a:
   both — regen (producer determinism) AND structural recognition
   (tools/lowcheck, consumer-side). Remaining tail: wiring lowcheck into
   the `bin` gate, and the cert↔binary byte tie (§6 gap a).
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
