# Lowered-conformance certificates — the standard form

> **STATUS (reset 2026-08-22): LAW.** the certificate standard form, the five gates, and the model-authoring contract (§8); its arcs are complete. The backlog is the GitHub issue tracker (labels `arc:coverage` / `parked` / `debt`; the goal = #23, the prune arc = #24) — any "next arc/rung" pointer below is history unless it names an issue.

> Path note (2026-07-18): file paths in this ledger are as-landed history; the repo was reorganized — decode old `examples/` paths via [LAYOUT.md](LAYOUT.md).

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
type-gates the four apps (wasmgen, lowcheck, manifest, bytetie) and,
through their closures, the tools/low kit.

> **Naming note (2026-07-07):** the wasm back end `tools/lowergen` was
> renamed **`tools/wasmgen`** (and its build set `examples/lowergen_*` →
> `examples/wasmgen_*`) when the second back end (`tools/x86gen`) made
> the tool-per-target naming the honest one. Dated sections below keep
> the name they landed under; read `lowergen` = today's `wasmgen`.

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

> **Moved to [records/LOWERING.md](records/LOWERING.md) (2026-09-02, the ledger split: LAW stays here, dated RECORDS live under docs/records/ with their section numbers unchanged).** Cited as `LOWERING.md §…` everywhere; open records/LOWERING.md for §6. mod.build.shard — BUILT in miniature (P4, 2026-07-03).

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
4. RE-OPENED 2026-07-12 (the coverage re-adjudication) — the consumer
   appeared: the uniform-rep coverage tier (IMP.md redirection block).
   Direction: a **Done-or-Fail conclusion** — STRONGER than the
   trap-conditional weakening contemplated here: failure is an
   observable value with a certified fallback, the budgeted-twin
   theorem shape made the default for checked artifacts — plus a
   requirements-level `except` clause at the artifact boundary.
   Premises stay as the check-eliding optimization and the embedded
   tier (total-under-premises is unchanged for leaf/fragment certs).
   Design ledgered at MEMORY.md D8.
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
8. LANDED 2026-07-10 (§6ah): the WORD fragment —
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
