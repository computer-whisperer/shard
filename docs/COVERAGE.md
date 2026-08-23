# COVERAGE.md — the coverage arc: the generic path, shardfmt as the flagship

> **STATUS (2026-08-22): RATIFIED — ARC OPEN (C0 done, C1 in progress).** User ratification 2026-08-22 ("Read through the doc and your report — looks good to me. Call it ratified and let's begin."). Issue #23 is the goal; this ledger is its law. Two rulings already taken by the user on 2026-08-22 and written in as law below: **≈0 hand-lowering is a GATE, not an aspiration (§3)**, and **per-fn inductions are the certificate-shape lean (§4 P7)**. Every other pin carries a lean and its rejected alternative and is ratified with this ledger. Nothing emits before ratification (CERT.md §10). The backlog is the GitHub issue tracker (labels `arc:coverage` / `parked` / `debt`).

Charter sources: docs/OVERVIEW.md §10 (the 2026-08-22 reset ruling),
docs/IMP.md §6 (the 2026-07-12 redirection — the uniform-representation
compiler) and §9 (the 2026-08-22 amendment), docs/MEMORY.md §9 (rung 4
pulled forward) and D8, docs/CERT.md §4/§8/§10 (validators, the ratified
dialect, the generator-freeze laws), docs/X86.md §3–§4/§10/§32 (the call
mechanism, the stack split, the platform-extern law). Issues consumed:
#23 (goal), #25 (regalloc rung), #26 (except clause), #27 (engine as
proof automation), #9 (reclamation + packed buffer — the counted heap is
its answer for the compiled path), #17 (refined-opaque composition —
off the path, see §8).

## 1. Mission

Ship `tools/shardfmt` as a proven native Linux binary through the
GENERIC path — spec → models/imp by a compiler → x86 by tools/impgen —
and switch `bin/rebuild.sh`'s fmt gate from `shard_eval run` to that
binary. `examples/calc` is rung 1.

The deliverable is the PATH, not the formatter: the compiler, its
generated certificate families, the counted-heap runtime, calls and a
data stack at both machines, and the generic bin tail that turns a
`(bin …)` line into an ELF with an artifact verdict. shardfmt is the
consumer that forces every one of those honest, because its oracle
already exists and cannot be fooled: byte-identical output over the
whole corpus (~945k lines of shard), the fmt gate's own contract.

What this arc tests is the sentence the ledgers have carried since
July without ever taking literally. IMP.md §6 promises "the generic,
TOTAL spec → imp translation for arbitrary first-order shard"; CERT.md
§4 ends "the floor for a default-path program is spec + bin
declaration"; OVERVIEW.md §10 records that the generic path "has never
seen a constructor, a `match`, a real call, or a heap cell." Closing
that gap is the whole arc. Performance is measured and recorded, not
gated (§6).

## 2. The consumer, measured (2026-08-22)

**The closure.** A token-level call-graph walk from shardfmt's `main`
(over-approximating slightly — any identifier matching a fn name
counts) reaches **173 fns, ~2.2k body lines**: 54 in meta/format, 3 in
tools/shardfmt, **107 in kernel/reader** and 9 in kernel/term. The
reader enters twice: the gate check re-parses fmt's own output with
the production `read_all`, and the error branch renders diagnostics
through `parse_diag`, which pulls in the elaborator's error rendering.
(#23's "~700 lines over meta/format" undercounts by ~3×; the census is
re-run by the compiler's own refusal list at C5.)

**The feature census** — what the path must be total over, all of it
present in this one closure:

- ADTs, including polymorphic ones: `List`/`Option`/`Pair`, `CNode`
  (7 ctors, recursive through `(List CNode)`), `SExpr`, `Tok`, `SR`,
  `PFrame`, `EmK`; nested patterns (`(Cons TPlus (Cons (TNum n) rest))`
  in calc; the reader's form matchers).
- Recursion of every admitted shape: structural (`(struct xs)`);
  wf-measures on derived list measures (`lz`, `cz`), on Int with
  arith certs (`wrap_q`, `sp`), and the printer's weighted
  dispatch-rank measures (`em_head`: `(+ (* 10 (lz t)) 2)`); a
  mutual-recursion SCC (`em_kids`/`em_head`/`em_if`/`em_list`/`em_pp`/
  `em_chain`/`em_if_deep`); non-tail recursion (`cat`, `flat_w`, the
  `em_*` family); self-tail recursion (`lex_go`, `rev_n`).
- Signed integers in earnest: `flat_w` runs a width budget negative
  and aborts on it; calc's `show_ascii` prints negatives.
- String literals (`"shardfmt: cannot read file\n"`, the reader's
  diagnostic vocabulary) — ground constructor data.
- I/O through four externs over a bin-local `World` with 12 bolt
  axioms (tools/shardfmt/mod.req): `get_args`, `read_file` (whole
  file → `(Option (List Int))`), `write`, `exit`.
- Nothing higher-order (LANGUAGE.md: first-order, no lambdas).

**Why hand-lowering is not a fallback.** The sha256 spec is 411 lines;
its hand imp sibling (std/sha256/sha256.imp.shard) is 29,045 — 71×. At
that ratio shardfmt's closure is ~150k hand lines. The aim in §3 is
not generous; it is the only plan that exists at this size.

**Performance expectation, recorded so it is not re-argued at C6.**
Strings are cons lists of bytes (≈16 B/char plus count traffic), every
temp round-trips memory (the 6.1× no-regalloc tax, #25), every call
spills. The first native shardfmt will be orders of magnitude faster
than the interpreter (reader.shard takes ~1 min interpreted) and well
short of a C formatter. The gate is functional; the numbers go on
record; the cancellations (MEMORY.md rung 5, #25, packed strings) are
per-decl upgrades AFTER the oracle passes.

## 3. The author-residue gate (user ruling 2026-08-22)

"Approximately zero hand-lowering" made measurable. For the flagship
(and for every future default-path bin), the author-side residue is
exactly:

- **One line**: `(bin shardfmt (entry main) (externs …) (except
  overflow oom stack))` — the except clause is #26's spelling (P9).
- **The measures already written** — they are the induction skeletons
  (P7); nothing is added for the lowering's sake.
- **Once per platform, never per bin**: the extern realization (P6) —
  syscall-grain shims in models/x86/linux.shard plus the proven
  adapter from a memory window to `(List Int)`, in a library.
- **One mechanical switch**: tools/shardfmt/mod.req imports the
  standard World module (P6) instead of declaring its own World,
  externs, and bolts. Its 11 requirements and their proofs are
  unchanged in meaning.

Banned, by name, because each is exactly what the hand path cost:
an imp twin; per-fn hand certs; a per-bin machine article (Arc B's
`sha256sum_stream_x86.shard` is 5.5k hand lines — the generic path
cannot have one, §6 gate 2); per-bin ELF/write glue files (a generic
build step instead); per-fn annotations of kinds, classes, or reps
for the goal's sake.

**The law that makes the gate enforceable:** a compiler refusal on
the flagship's closure is a compiler bug, never author work. The
standing "no proof family gets a third hand instance" (IMP.md §6)
sharpens, for the generic path, to **no FIRST hand instance**. When
impc refuses a fn, impc grows. A hand-filled generated obligation is
hand-lowering by another name and counts against the gate.

Measured form, at C6: the diff under tools/shardfmt is the bin line
plus `use` lines (≤ ~20 lines), zero hand-authored .shard files are
added there, and every generated product regenerates byte-identically
under BUILD.md's regen contract.

## 4. Opening pins — the paper debts closed

Each pin: what it fixes, the LEAN, the REJECTED alternative with its
cost, where it lands. IMP.md §9 named four debts (calls/stack, signed
kinds, address policy, heap patch/framing algebra) plus the
cons/match/free micro-flagship; they are P5, P3, P4, P4, and C3.

**P1 — The tool: `tools/impc`, spec → imp.** A second generator
beside tools/impgen (imp → ISA), consuming the loaded kernel `Module`
(FnDef bodies over `Expr`, the same loader/meta-invoke path impgen
already uses) and producing an `IProg` value plus certs. Two tools,
two products, one seam: hand-written imp twins keep entering at imp
as first-class refinement inputs (IMP.md §2a's premise), and the M×N
factoring stays visible. REJECTED — one fused spec → ISA tool: hides
the seam the whole IMP ledger exists to expose, and the x86 leg's
existing walk, ties, and bridges would be re-homed for nothing.
Name: `impc` pairs with `impgen` (into imp / out of imp) and is what
an author guessing from the tree finds; `tools/lower` was rejected
for colliding with LOWERING.md's (frozen) lowering form.

**P2 — The representation: boxed constructors, immediate scalars,
tag bit.** Uniform representation over the imp window: every
constructor value is a headered cell (MEMORY.md §6 — count, tag,
size, then 8-byte slots); every slot holds either an immediate Int
or a reference. LEAN: references are window indexes with the low bit
0 (cells 8-byte aligned), immediates carry low bit 1 (i63, two's
complement) — the OCaml/Lisp scheme, and already the in-house
convention of the compiled chain (bin/shard_check's i63). Count ops
test the bit, so ONE body of code and ONE cert serve `(List Int)`
and `(List CNode)` alike — no monomorphization of code or of
certificates. REJECTED — box every Int: every column counter in the
printer allocates. REJECTED — per-instantiation layout maps
(monomorphized code): code and cert families multiply by
instantiation count, and the per-fn theorem is no longer one theorem
per source fn. This resolves MEMORY.md D6 (header/tag scheme) and D2
(accounting in bytes: the header's size word) at the imp grain, where
IMP.md §1 says layout crystallizes. Headers are written once while
shared (§5's write-once law); the header word layout is fixed at C2
and recorded there.

**P3 — Unrefined `Int` and signedness: no signed kind at imp.**
Spec `Int` lowers to an i63 immediate in a U64 slot. Arithmetic is
the U64 wrapping ops the machine already has, followed by a band
check (result in [−2^62, 2^62)) whose failing leg is
`Fail overflow`. Signed comparison is the bias respelling —
`a <s b ⟺ (a xor 2^63) <u (b xor 2^63)` — over the existing
`CLtU`/`CLeU`, so the x86 model's `Cond` vocabulary does not grow.
`ediv`/`mod` compile to a small library fn in the runtime (P4's
module) proven against the kernel's own Euclidean definition,
including its behaviour at divisor 0 — whatever ceval computes is
what the artifact computes, no new family. REJECTED — a signed kind
family at imp (IMP.md §2a named-later growth): a second band
discipline through every translator, wk gate, and bridge for what
one respelling and one check express; the V2-6 steer (IMP.md §6a:
respell over premise apparatus) applies. Refined
register-class scalars (MEMORY.md rung 1) remain the per-decl
dissolution path (D8's DISSOLVE tier) and are not required for the
goal.

**P4 — The heap: a checked allocator and precise counting, as imp
code, proven once.** MEMORY.md rung 4 restated at this arc's grain:
a bump allocator with size-classed free lists over a fixed region of
the declared window; `inc` at reference duplication and `dec` at
syntactic last use, emitted by impc; cascading free through a
worklist; allocation failure is `Fail oom`. The runtime is a LIBRARY
of imp fns (P5's ICall reaches it like any callee), hand-written at
imp and hand-proven ONCE — allowed by §3 because it is per-platform,
not per-bin. Its once-proven theorems are the arc's hardest proofs:
the managed-graph invariant (MEMORY.md §4: purity ⇒ the counted graph
is acyclic, count zero ⟺ unreachable from the roots) and the
alloc/store/free framing lemmas — stated in the base+patch
vocabulary (CERT.md §5, MEMORY.md's closing note) from day one: a
heap step is a patch footprint, readback of every cell outside the
footprint is preserved by a once-per-model law. That IS the
"heap patch/framing algebra" debt. Address policy: the window is the
product's declared `[ibase, imemsize)` (IMP.md §2a as amended), with
the heap region, the data-stack region (P5), and the static-data
region (P8) carved from it at fixed offsets recorded in the IProg;
indexes are U32; the ELF maps the window as one RW PT_LOAD with
p_filesz covering static data and the kernel zero-filling the rest
(X86.md §32's binelf precedent). v1 window is large and fixed — OOM is
exhaustion of a region the loader already granted, so no resource
axiom is needed (P9). REJECTED — mmap/brk growth in v1: the machine
window becomes a set of windows (X86.md §32 names it as growth) and
every framing statement gains a case; it waits for a consumer whose
live data does not fit a fixed window. REJECTED — writing the
allocator in spec shard and compiling it with impc: elegant, and the
named lowsrc-absorption door, but it makes impc's first consumer its
own runtime.

**P5 — Calls and the data stack.** imp grows a call — AMENDED at C1a
(2026-08-22): a STATEMENT `(IpCall i k args)` in a mirror statement
tier over the unchanged expression language, not an expression; see
IMP.md §2b and the C1a record — with the frame semantics `icall`
already has: a fresh locals frame per call. The
x86 leg rides the structured `XCall`/`XRet` the model already owns
(X86.md §3.3: the control stack is private, never addressable) and
the §10 callee-cert form (clobber sets in the cert, `xcall_bridge`
composition). What is new is the DATA stack X86.md §4.3 deferred
"until a consumer forces it": this consumer does. LEAN: a shadow
stack in the window — a stack-pointer cell plus a region; the
generated imp spells every spill and reload as ordinary word stores
and loads, so neither machine gains a stack primitive and the
stack's proof story is the window's (LLVM→wasm's protected-call-stack
+ shadow-stack split, the precedent §4.3 cites). v1 calling
discipline: caller saves everything live across a call to the shadow
stack; args in the SysV homes as today; the regalloc rung (#25)
refines this on measured output. The stack FAMILY (D8's open
sub-question): a depth check at each call site against the region
bound — "depth counter = fuel made real" — whose failing leg is
`Fail stack`; the frame class's discharge-by-construction is the
later dissolution for bounded programs. Self-tail-recursion lowers to
`IWhile` (the accumulator shapes `lex_go`/`rev_n`/`valI_go`); every
other call, mutual tail calls included, is a real call in v1.
REJECTED — stack primitives in the imp machine: machine growth for
what window stores already say, and a second framing vocabulary.
AMENDED at A-0 (2026-08-22): the stack FAMILY's depth check is the
MODEL's, not generated code's — `IpProg` carries the budget
`ipdepth`, `ipcall` fails `FStack` at it, and the frame tier mirrors
the count in R14; impc's v0 counter (the `[rb+16]` cell, "depth
counter = fuel made real") is retired, and the frame region's
sufficiency (`ipstack + ipdepth · maxframe ≤ ipmemsize`) is decided
once by `ixf_prog`'s carve gate. Why: a generic machine theorem must
bound the dynamic call depth to know the frames fit, and imp's fuel
cannot (it also bounds loop iterations and list lengths) — see §11.

**P6 — The standard World and the extern realization.** A standard
module — LEAN name `std/world` — declares once what every bin today
re-declares: the `World` type, the observables (`w_args`, `w_reads`,
`w_output`, `w_exit`), the spec-level externs `get_args`/`read_file`/
`write`/`exit`, and the bolt axioms relating them. Its MACHINE
realization, once per platform: models/x86/linux.shard grows
`open`(2) and `close`(2) shims beside read/write/exit (two-instruction
bodies, the §32 theorem pedigree; encoding is the same `0F 05`), and a
library fn `list_of_window` — compiled code, not shim logic — turns
the read loop's filled window into a `(List Int)`, so that
`read_file` = open, read until EOF into the window, close, adapt.
`get_args` reads the `_start` entry-stack contract (X86.md §32's
binelf probe: `[rsp]=argc`, argv pointers, NUL-terminated) through the
same adapter. The composition theorem — the machine-grain trace
(`LxRead`/`LxWrote`/`LxExited`) observes as the spec-level
`w_reads`/`w_output`/`w_exit` — is proven ONCE for the module and
instantiated per bin by the build (§6 gate 2); the artifact's trust
ledger is then the kernel model alone. REJECTED — bin-local externs
(Arc B's pattern, the right one for a hand-optimized leg): each bin
re-declares 12 axioms and re-proves the composition — precisely the
hand article §3 bans. Sub-question CD3: the bin-boundary law (IMP.md
compendium: a checked closure's declared externs EQUAL the bin's list
both directions) means a bin lists std/world's whole set even if it
uses three of four; lean: accept that — an unused extern costs
nothing — rather than split the module per extern.

**P7 — Certificates: per-fn inductions in the ratified dialect (user
lean 2026-08-22).** For every source fn `f` with measure `μ`, impc
emits ONE theorem: for every heap satisfying the managed-graph
invariant and every argument tuple read back as `a_i`, the imp call
returns `Done (h', r)` with `readback h' r = (f a_1 … a_n)` and `h'`
extending `h` outside the call's footprint — or `Fail family` for a
declared family. Its proof is an induction along `μ` — `induct`/
`subterm-induct` for structural measures, `wf-induct` for Int
measures (the TOTALITY.md §6 mutual extension gives an SCC one
induction on the shared measure, so the printer's seven-fn SCC is one
generated theorem) — whose body is the conversion dialect
(`change`/`exact-conv`, CERT.md §3) citing the once-proven
per-construct laws: P4's alloc/store/free framing, the match-as-tag-
dispatch readback law (per type, derived from the type declaration —
the records-arc precedent), P5's call composition. The imp → x86 leg
is impgen's existing structural walk grown by the new arms (ICall via
the §10 callee-cert form; word-grain loads/stores), emitting the
A1-era validator citation where the expression tier already has one
(`vxg_valid`) and walk-tree bridges elsewhere — the same dialect Arc
B's generated files speak. Pricing: the CDR's per-program estimate
under this dialect is 3–6k lines; at 173 fns that is ~10–20k
generated lines for shardfmt against 188k for sha's replay-era outs.
REJECTED (on record, with the lean) — ONE verified-compiler theorem
over a reflected interpreter: correctness proven once over eval.shard
and impc as data, per-program proof = nothing. It needs the
reflection theorem "kernel reduction of `f` = eval.shard's
interpretation of `f`'s definition", an unpriced research item with
no corpus precedent; per-fn inductions price linearly in the closure
and need no reflection. It remains the named door if per-fn sizes
read superlinear (§6). Closing the proofs: impc emits the complete
skeleton; where a step needs search (measure-decrease arithmetic,
readback equations under a fresh cell), the lock-step engine is the
closer (#27 — its reach is MEASURED at C3 before anything depends on
it) and tools/prove the fallback; by §3, a generated obligation is
never hand-filled.

**P8 — Static data.** String literals and every ground constructor
term in the source become pre-built cells in a static region of the
window, count set to the sticky "immortal" value (dec never frees
it), with their readback proven by compute once per literal. REJECTED
— building literals at startup: an allocation and a code path per
literal, and the reader's diagnostic vocabulary has dozens.

**P9 — The except clause and the artifact claim (#26).** Grammar
growth on the `(bin …)` form through the canon-owned reader: a sixth
clause `(except FAMILY …)` over the closed v1 family set
{overflow, oom, stack}; the driver's verdict line prints
`MET (artifact: except: …)`; the accepts-ratchet twin holds the
generated certs' fail families EQUAL the clause, both directions.
The imp machine gains the reasoned Fail value — `IFail Family` on
`IOut`, distinct from `ITrap` and from fuel `None` — and the x86 leg
realizes it as the v1 fallback signature (stderr diagnostic, reserved
nonzero exit; D8's disjointness pin). The prefix theorem rides the
fact that imp effects happen only at extern calls (P6), so a Fail run's
trace is a prefix of the spec's by construction. The CONDITIONAL form
(resource axioms in `trusts`) does NOT ship in v1: P4's fixed window
is granted at load time, so "the machine grants ≥ W bytes" is not a
run-time condition — its failure is no run at all. It stays #26's
named door for the first consumer whose grant is dynamic.

**P10 — Strings and containers: cons lists, in this arc.** `(List
Int)` is the string representation the flagship already uses and is
what the path compiles. std/str (packed, refined-opaque — and carrying
#17's composition bug) and the Vec container layer (IMP.md §4a, I2.5)
stay off the critical path as per-decl representation upgrades after
C6. REJECTED — packed strings first: a rep bridge and a second readback
family before the first generic artifact exists.

**P11 — The target set.** x86 only. impgen's wasm leg stays frozen at
its landed extent; the new arms (ICall, word-grain memory, IFail) are
added to the x86 leg. The goal is a Linux binary; the wasm leg of the
generic path is a named door (CD5), opened by a consumer.

## 5. Rung ladder

Each rung: ratified scope first, corpus pins, CI green behind it,
one commit per slice, a RECORD appended to §10. Byte-emit and
on-silicon runner files are Opus-delegated per the standing split.

- **C0 — this ledger ratified.** The pins above are law; §3 is a
  gate. Record the ratification date here.

- **C1 — the machine grows.** models/imp: `ICall`, word-grain
  `ILoadW`/`IStoreW` (the named accessor rung, now forced), `IFail`
  on `IOut`; `wk_fn` extended; the probe grid (IMP.md §3) re-validated
  with the honesty corners for each new arm. models/x86: 64-bit
  load/store forms (encoder arms + silicon pins Opus-delegated); the
  open/close shims in linux.shard. tools/impgen: the walk's new arms in
  the ratified dialect. Gate: corpus diff-clean; the frozen oracle
  outs (std/sha256/impgen_*_out.shard) regenerate byte-identically
  — CERT.md §10's freeze is about the DIALECT, and no existing pin
  uses a new arm.

- **C2 — the runtime.** The P4 allocator/counting library and the P8
  static-data convention as imp code under models/imp (lean:
  `models/imp/rt.shard`), with the managed-graph invariant and the
  framing laws proven in the base+patch vocabulary; P3's `ediv`/`mod`
  fns. The header word layout is fixed and recorded here. Gate: the
  once-proven theorems check; tiny-heap differential vectors force
  frees and OOM deterministically (MEMORY.md §3).

- **C3 — impc v0: the cons/match/free micro-flagship.** A handful of
  fns chosen to cross the whole gap once — list length and append, a
  match over a three-constructor ADT, one non-tail recursion, one
  Int-measure recursion — compiled to imp, certified (P7's first
  generated family), lowered by impgen, run on silicon against the
  model through the W-rung differential layer. MEASURED here, before
  anything depends on them: per-fn cert size vs body size (the
  superlinearity trigger, §6), checker peak-live per generated file
  (CERT.md §9(c)), and the engine's reach on the generated
  obligations (#27's wake condition).

- **C4 — calc on silicon.** examples/calc gains a spec-level bin
  source (a World main: read stdin until EOF, evaluate each line,
  print — the stream_main statement schema at spec grain, with the
  TOTALITY.md §8 unbounded/fueled split) and the line
  `(bin calc (entry main) (externs …) (except overflow oom stack))`;
  impc/impgen/the generic bin tail produce the ELF; the verdict prints
  the except form. calc exercises the overflow family honestly
  (unbounded user arithmetic), non-tail recursion (`eval`, `show_nat`),
  nested patterns, `ediv`. MEASURED: generated lines, check time/RSS,
  runtime vs `shard_eval`. **#25 (register allocation) is priced on
  these numbers and positioned here or after C6 on the evidence.**

- **C5 — closure totality.** impc's refusal list over shardfmt's
  reachable set (§2) driven to empty: polymorphic cells, the weighted
  SCC measures, signed budgets, the reader's diagnostic rendering.
  Every refusal closed by growing impc, never by touching the source
  (§3). Gate: impc accepts the closure with zero refusals; the
  generated certs check.

- **C6 — shardfmt ships.** tools/shardfmt/mod.req switches to
  std/world (the one mechanical edit); the bin line lands; products
  are generated as siblings; the ELF at tools/shardfmt/shardfmt is
  gitignored like the sha binaries. The differential: the WHOLE corpus
  formatted by the binary, byte-compared against `shard_eval run
  tools/shardfmt/shardfmt.shard` — the oracle — in CI (tier and cost:
  CD6, #37). bin/rebuild.sh's fmt gate runs the binary. **The arc
  closes at two numbers: corpus-wide diff count (must be 0) and the
  fmt gate's wall-clock vs the interpreter**, with §3's residue
  measurement on record.

## 6. Gates and falsification

Measured gates, per rung, are listed in §5. The arc-level gates:

1. **The residue gate (§3)** at C6, in lines: bin line + `use` lines
   under tools/shardfmt, zero hand .shard files added, zero hand-filled
   generated obligations.
2. **The generic composition gate.** No per-bin machine article
   exists for calc or shardfmt; the artifact theorem is an
   instantiation of std/world's once-proven composition (P6) by the
   build. If the first instantiation needs a hand weld, the arc stops
   and P6 is redesigned before C5.
3. **CERT.md §9(c)** — peak live terms per generated cert file —
   measured at C3 and C4; the named remedy is the module-layer image
   (#34), pulled into the arc if the gate fails.
4. **The oracle**: corpus-wide byte identity at C6.

The architecture is materially DOWNGRADED — stop and redesign before
the next rung — if:

- per-fn generated certs grow superlinearly in body size, or average
  past ~300 formatted lines per fn at C3/C4 (the reflected-interpreter
  door of P7 opens as a priced candidate);
- the managed-graph invariant does not close in the base+patch
  vocabulary (then the framing algebra is redesigned before C3 emits);
- the first generic artifact is slower than `shard_eval` on its own
  input, or slower than a C-class implementation for reasons inherent
  to the proof-facing IR rather than the known backend immaturity
  (OVERVIEW.md §10's falsification, inherited from Arc B);
- the engine plus tools/prove close less than the generated
  obligations impc's skeleton leaves open, and the gap cannot be
  closed by growing the skeleton — then §3 holds and the generator
  grows, never the author's file.

## 7. Sequencing rules

- **Nothing emits before C0 ratifies** (CERT.md §10). C1's
  byte-level parts (encoder arms, silicon pins, runner files) go to
  an Opus subagent per the standing split; Fable does the machine,
  the runtime, impc, and the docs.
- **Serial on main**, one commit per slice, corpus gate = CI (never
  the full corpus locally). The corpus-wide differential at C6 is a
  long-tier job until its cost is known (CD6).
- **One design track at a time.** #25 waits for C4's numbers. #9 is
  answered for the compiled path by P4 and closes when C6 lands; #17
  and std/str stay parked (P10). #27's engine work is measurement
  only until C3's numbers exist.
- **Generated files are never hand-merged or hand-patched**
  (regen = the canon contract; the thread-division boundaries).
- **Signature changes to shared models** (IExp/IStmt/IOut, XInstr)
  are announced in the rung record and walked STRUCTURALLY across
  every consumer in one commit (the XCpuid precedent).

## 8. Non-goals and named doors

- **Tracing GC** — a non-goal by MEMORY.md law. The counted heap is
  the reclamation story (#9's first item, for the compiled path).
- **Higher-order functions** — not in the language; no closure
  representation is designed here.
- **The performance cancellations** — borrow, unique/in-place reuse,
  frame class, register-class scalars (MEMORY.md rung 5 and rung 1):
  per-decl upgrades after C6, each priced against the C6 baseline.
- **Register allocation** — #25, a rung inside the arc, opened on
  C4's evidence; not a v1 prerequisite.
- **Packed strings / Vec / a second container rep** — P10, after C6.
- **Heap growth (mmap/brk)** — P4's named door; the conditional
  artifact form rides with it (#26).
- **Signed kinds at imp** — P3's respelling makes them unnecessary;
  the named-later growth in IMP.md §2a stays named-later.
- **Mutual tail-call elimination** — real calls in v1.
- **The wasm leg of the generic path** — CD5.
- **Many-legal-targets validator generality (clause 1)** — #32,
  untouched; impgen's output is canonical and recompile-checked.
- **A streaming shardfmt** — slurp is the program's own shape
  (`read_file`); not a lowering concern.
- **Replacing the interpreter's shardfmt as the oracle** — the
  interpreted run stays the reference forever; the binary is what the
  gate RUNS.

## 9. Decision points (open; resolved in-rung, recorded in §10)

- **CD1 — reference identification: tag bit (P2 lean) vs per-type
  layout maps.** Ratified at C0 as the tag bit; revisited at C3 only
  if the readback laws under the bit read worse than expected.
- **CD2 — data-stack region size and the depth check's placement**
  (every call site vs SCC entry). C1/C4.
- **CD3 — std/world's extern set vs the bin-boundary law** (P6's
  sub-question). C4, with the first generic bin.
- **CD4 — product naming and residence**: `impc_out.shard` sibling
  beside the source (the impgen convention) vs a per-module products
  directory. C3; LAYOUT.md amended when decided.
- **CD5 — the wasm leg.** Door; opened by a consumer.
- **CD6 — the corpus-wide differential's CI tier and cost.** C6,
  with #37's numbers.
- **CD7 — the header word layout and the immortal count value.** C2.

## 11. The certificate phase — design note (2026-08-22; ORDER RATIFIED the same day: A → C2b → B)

With C1a–C3a landed the generic path RUNS end to end at the model
(spec → imp → x86, three-way differential 15/15) and nothing is yet
PROVEN about it beyond those differentials. The arc's remaining
substance is two simulation theorems and the library they stand on.
This note fixes their shapes so the decision is about order and cost,
not about what is being built.

**Theorem A — imp ⊑ x86 at the frame tier (C1c-3).** The frame tier
has ONE alignment relation for every statement, which is exactly what
makes CERT.md §4's validator form stateable at statement grain (the
"mem-capable validator statement tier" STREAM.md §6 named as a door —
this is its demand): acceptance is recompile-equality,
`valid_frame p xm := (ixf_prog p) = (Some xm)`, witness = unit (A1's
arity placeholder), and the soundness theorem, proven ONCE over every
well-kinded IpProg, says: for every fn k, arguments, and memory M whose
stack region is disjoint from every address the program reads or
writes outside its frames, the x86 run from a register file with
R15 = fp and the arguments pre-written into the frame at fp returns
RAX = v and a memory agreeing with imp's outside the stack region
whenever `iprun` returns `IpRv v`, traps whenever imp traps, and
reaches the exit shim with RDI = 70+family whenever imp fails. The
state relation is `rs.R15 = fp ∧ frame(XM, fp, nl) = lc ∧ XM ≡ M off
the stack region`; the proof is an induction over the call tier's
fuel SCC with one lemma per IpStmt constructor and one per IExp
constructor, each a framing argument over std/mem's word view. What
is NEW relative to A1's expression validator: memory (word-level
framing at width 8 — `ls8_id` and the store/load disjointness laws are
std/mem growth, owed since C2a), calls (the callee's frame sits above
the caller's by `own`, so the caller's frame is preserved by the
callee's stores — the frame-disjointness lemma), and the fuel algebra
(A1 rode `lg_fuel` towers over instruction counts; the frame tier's
per-statement instruction counts are static but path-dependent, so
the theorem is stated at "sufficient fuel" with a cost function
`ixf_cost` per statement — the place Runs/RunsWithin (#35) was designed
for, and the first thing to re-open if the towers get heavy).
Per-program cost after this theorem: ONE `exact-conv` of the soundness
theorem at `p`, plus one compute of `ixf_prog p` — tools/impgen emits
nothing for this leg; the product's x86 module is a computed value.
This replaces "impgen's walk grown by the new arms" (P7's text) for
the frame tier. REJECTED-because: per-program generated walk certs
(impgen's current family) re-derive the same simulation per statement
instance; with a uniform relation there is nothing program-specific
left to walk, and the replay-era volume (188k lines for sha) is the
measured cost of that shape.

**Theorem B — spec ⊑ imp (C3b, the P7 per-fn inductions) over the
runtime laws (C2b).** Per type T a READBACK `rb_T : Mem → word → Option
T` derived from the TypeDef (immediates decode; a reference reads its
header's tag and recurses into the slots), fuel-bounded on memory
(acyclicity of the managed graph is what makes a bound exist — the
C2b invariant). The heap invariant `hinv M` := every live cell's slots
hold valid words, counts ≥ 1 on reachable cells, the free lists and
the bump pointer well-formed, the static region immortal. The once-
proven laws (C2b): `rt_alloc` returns a fresh cell disjoint from every
live one and preserves every readback (patch-footprint framing in the
base+patch vocabulary, CERT.md §5); filling a fresh cell's slots then
reading it back is the constructor; `rt_inc` preserves every readback;
`rt_dec` on an owned reference preserves every readback reachable from
anything else (the release theorem — the hard one: the iterative
worklist must be shown to free exactly the unreachable cells); the
depth cell is ghost to readbacks. Then the per-fn theorem impc emits
for each source fn f: `hinv M ∧ rb_args M args = Some as ⟹ iprun fuel P
f_ix args M = Some (IpRv v M') ∧ hinv M' ∧ rb_T M' v = Some (f as) ∧
every readback of a borrowed argument survives into M' ∨ IpRfailed
fam` — an induction along f's own measure (the TOTALITY obligations
supply the decrease), with the body's statements discharged by the
per-construct laws and the ownership discipline (borrowed in, owned
out) as the invariant threaded through calls. The engine (#27) is
measured as the closer of the skeleton's leaves here.

**Order — RATIFIED 2026-08-22 (user: "A -> C2b -> B sounds good").**
Lean, as adopted: **A first, then C2b, then B.** A is
self-contained at the machine layer (framing only, no heap semantics),
has A1 as its template, and closes C3's "certified at the machine"
gate; its library growth (width-8 framing in std/mem, region
disjointness) is exactly what B's heap laws reuse. Cost guess: A ≈ the
A1 expression tier's effort doubled (statements + calls + memory); C2b
≈ A again (the release theorem dominates); B's per-fn family is
generated text whose size the C3 gates measure. Alternative: B first
(the arc's novel claim, the thing no one has shown) — rejected-for-now
because its foundation (C2b) is the deepest proof of the arc and the
machine leg would sit unproven meanwhile; revisit if A's fuel algebra
stalls. Either way, nothing here changes a pin: P7's lean stands for B,
and A is the validator form CERT.md §4 already ratified.


### 11.1 Theorem A as stated at A-0 (2026-08-22) — what the draft did not predict

Stating the theorem precisely, against the landed code, moved three
things out of the draft's premises and into the MODEL, and fixed the
proof's two vocabularies. Recorded here because each is a decision
(with its corpse), and the slices below are cut along them.

**(1) Disjointness is the window's, for free.** The draft premised "a
memory whose stack region is disjoint from every address the program
reads or writes". No footprint instrument is needed: `IpProg` now
carries `ipstack`, and `iprun` evaluates at the window
`[ipbase, ipstack)` — the frame region `[ipstack, ipmemsize)` is
INVISIBLE to imp (a word op there is an imp trap), so a run that
returns a value never touched it, and the x86 module (window
`[ipbase, ipmemsize)`) keeps its frames there. Consequence: imp TRAPS
are outside the theorem (imp traps where x86 would not); the clauses
are `IpRv` and `IpRfailed` only — exactly the outcomes Theorem B
produces, so nothing is lost at the composition.

**(2) The depth budget is the model's.** The theorem must know the
frames FIT the region, i.e. bound the dynamic call depth; imp's fuel
cannot (it also bounds loop iterations and statement-list length, so
"fp + fuel · maxframe ≤ hi" is unusable for any real run), and a
generic theorem cannot see a counter impc happens to emit. So:
`ipdepth` in `IpProg`, the SCC threads (dmax, d), `ipcall` fails
`FStack` at the budget and runs the body at d + 1; the frame tier
mirrors d in R14 (prologue: fail unless R14 < dmax, then R14 += 1;
epilogue R14 −= 1); `ixf_prog` gates the carve once
(`ipstack + ipdepth · ixf_maxown ≤ ipmemsize`). impc's own counter is
deleted (29 prologues gone from the micro product, 1922 → 1669
lines). THE FINDING THAT FORCED IT: impc v0's bound was 100000 while
the ELF's frame region is 512 KiB ≈ 2600–3500 frames — a
4000-deep recursion would have faulted the process (SIGSEGV) instead
of exiting 72; the proof design exposed a real bug before a line of
proof was written. The `t_deep` wrapper (sumto 5000 at budget 1000)
now pins the fail leg end to end: imp −13, x86 model trap, silicon
exit status 72. REJECTED-because: the unbounded-window layering
(prove functional correctness in a model with xmemhi = ∞, "the stack
fits" as a separate resource theorem) — it moves the same unproven
gap to the bin, where it is least visible.

**(3) The x86 memory needs a WITNESS, so the proof has a twin.** The
conclusion "x86 memory agrees with imp's off the stack region and
holds the caller's locals" is an existential over the frame contents;
equations have no ∃. Skolemize it: an imp-side TWIN evaluator
`ipt_*` (proof-facing, A2's status) returns the imp outcome AND the
list of frame patches the frame tier writes (locals sets, spills,
arguments, the callee's zeroing). With A2's base+patch vocabulary
(CERT.md §5): x86 memory at every point = `xp_mem mem0 psx`
STRUCTURALLY — every XMStore64 / XStore8 is one patch prepended,
refl-grade — imp memory = `xp_mem mem0 (fbelow ipstack psx)` (the same
stores in the same order, the frame ones filtered), and the locals
relation is a computable read-through on the patch list at
`[fp + 8i]`. Register garbage (RAX/RDX/RDI/R10/R11) is projected
away by `xo_fr`, A1's projection idiom. REJECTED-because: per-address
observational conclusions (∀a) cannot be carried as the induction's
premise; a patch-level twin of the x86 machine would be a second
semantic authority.

**(4) Fuel: towers over `kf K f`, "for all heights", no monotonicity.**
x86 burns fuel per instruction position and per nesting level, imp per
statement position — a per-program ratio K (`ixf_kok K p`: every
statement's emission cost ≤ K, computed). x86 fuel is
`lg_fuel c (kf K f)` with `kf K (S f2) = lg_fuel K (kf K f2)`: S-headed
whenever imp's fuel is, so the induction on imp fuel peels both. Every
lemma is stated for ALL heights c ≥ (its cost), so a nested point
re-enters the IH at the height that remains — linear side conditions
only, no Nat inequalities, no fuel-monotonicity lemma. The seam is the
GENERAL one (no purity premise: the same fuel on both sides is an
exact equality). The multiplicative K is the re-opening point for
Runs/RunsWithin (#35) if the towers get heavy.

**(5) One induction.** A request type `IptReq` (stmt / while / stmts /
call) and a dispatcher give the four engines ONE claim and one IH;
each engine's case is a lemma cited from it.

**The slices (each a corpus row, CI behind it):**
- **A-0 — the model amendments** (this record; landed): `ipstack`,
  `ipdepth`, R14, the carve gate, impc without the counter, `t_deep`.
- **A-1 — the kit**: the frame-patch grammar (`FPatch`: word/byte at
  width 8 → `store_le (xw8)` / `mem_set`), apply / read-through /
  footprint laws (`ls8_id` and the width-8 below/above framing in
  std/mem — owed since C2a), `lg_fuel` peel + additivity, the general
  seam with its continuation adapter, `xo_fr`, `kf`, `ixf_kok`.
- **A-2 — the expression lemma**: `ixf_exp` sound — RAX := the value,
  spills = patches above the locals, R14/R15 preserved; induction on
  e over the seven constructors and the op table.
- **A-3 — straight-line statements**: IpSet / IpStore / IpLoadW /
  IpStoreW / IpFail / IpUnreach against the twin.
- **A-4 — control**: IpIf / IpWhile and the dispatcher induction.
- **A-5 — calls**: arguments as patches into the callee's frame, the
  frame-disjointness lemma (the callee writes only at ≥ fp + own and
  below ipstack), the depth mirror, the fn-level lemma.
- **A-6 — the theorem**: `valid_frame p xm := (ixf_prog p) = (Some xm)`
  and its soundness at `iprun` / `xrun_fn`; the micro-flagship's
  instance by citation (no replay) as the corpus row; the world-tier
  clause (RDI = 70 + family at the exit shim → process status) noted
  as C6's composition.

## 10. Rung records

(Appended per rung: what landed, commits, measured numbers, what was
found that the pins did not predict.)

**C0 — RATIFIED 2026-08-22** (user, on the drafted ledger d997209 and
its report, without amendment). §3 is a gate; P1–P11 are law; C1 opens.

**C1a — the call tier lands (2026-08-22).** models/imp/imp.shard
+383 lines (145/0): `IFam`, `IpStmt` (the four base forms mirrored +
`IpCall` / `IpLoadW` / `IpStoreW` / `IpFail`), `IpFn` / `IpProg`, the
SCC `ipstmt` / `ipwhile` / `ipstmts` / `ipcall` + `iprun`, the lift
`ip_of_*`, the gate `ipwk_prog`; probe
models/imp/probes/ipcall_probe.shard (34 claims, corpus-registered).
IMP.md §2b is the machine record. **The finding that shaped it:** the
plan was to extend `IExp` / `IStmt` / `IOut`; the consumer census
showed the base constructor sets are load-bearing — `case-on … IOut`
trees in impgen's emitted certs and the blueprints, exhaustive
`match`es and inductions over `IExp` / `IStmt` in the 247k-line
vx86_acc_probe (emitted by scratchpad tooling that no longer exists)
— so any new constructor would have meant hand-patching emitted
proofs. REJECTED-because recorded here: extension = a structural walk
through ~500k certificate lines with no regenerator. The mirror tier
costs ~130 lines of duplicated evaluator and one lift law (owed at
C3); everything existing is untouched, and the C1 gate "frozen oracle
outs regenerate byte-identically" holds trivially. Consequences: P5's
call is a statement (expressions stay pure and fuel-free); word loads
are statements too (`IpLoadW i addr`), which is what x86 emits anyway
(mov r64, [addr]). Gotcha for generated proofs: `compute` packs ground
Nats, so a claim that must rewrite through `load_le_s` fences `iw8`
out of the compute and opens it by unfold + reduce. Owed: `ls8_id` in
std/mem (C2); the lift law (C3). Re-slotted within C1: the open/close
shims move to C5/C6 prep (they are consumed only by shardfmt's
`read_file`; calc needs stdin/stdout/exit, which exist).

**C2a — the runtime's code lands (2026-08-22; C1b runs in parallel on
the x86 side).** models/imp/rt.shard: the spelling kit (rk/rl/ra/radd/…
/rldw/rldo/rstw/rsto, risref, rcount/rarity, rfree_at — shared with
impc), the table `rt_fns rb` = rt_init / rt_alloc / rt_inc / rt_dec as
IpFn values parameterized by the runtime base, `rt_prog` (table ++
program). Representation and layout fixed as the file header records
(CD7 resolved): header = count + tag·2^32 + arity·2^48; immediates
odd, references even U32 indexes; immortal = count ≥ 2^31; a dead
cell's count field carries the free worklist link; free lists per
arity < 16 at [rb+24+8k], arity ≥ 16 bump-only (a leak, never a wrong
answer); rt_dec releases children through an ITERATIVE worklist, no
recursion. Probe: models/imp/probes/rt_run.shard — a RUN-MODE driver
(17 tests: placement, header, reuse, inc/dec, the three-cell cascade
and its LIFO reuse order, a shared child surviving its parent, oom,
immortal, immediates, the large class, the gate) pinned with
`pin_run imp_rt_run`; run mode because std/mem's word view is opaque
in check mode and the runtime's control flow depends on loaded values
— the theorems are C2b's. **Invariant surfaced by the probe** (two
tests first failed on it): every slot of a live cell holds a valid
word — the runtime never null-checks, so an uninitialized 0 reads as
a reference to index 0. impc's constructor emission fills every slot
before the cell can be observed; the C2b invariant states it.

**C1b — the x86 memory sub-vocabulary lands (2026-08-22; Opus-delegated
byte parts, b216dfc).** `(XMem XMInstr)` with `XMLoad64 Reg Addr` /
`XMStore64 Addr Reg` (register source only: `mov r/m64, imm32`
sign-extends, no honest 64-bit immediate twin), semantics in the scalar
tier (`xminstr_leaf`, XLoad8's shape at an 8-byte span, `xw8` = imp's
`iw8` tower), the world/vector tiers delegating through their
catch-alls. Structural walk: link.shard's 12 XInstr inductions (two
needed a species split — `xlk_extx`/`xlk_shx` compute the window guard
in place, so the flat mirror's rewrites could not fire under the
binder), vx86_acc_probe 11 case-on pads + 11 dispatch-table arms, the
transition-window pad, bytetie's readback species. Encoder REX.W 8B/89
through the existing `enc_mem`; **silicon differential 250/0** (was
230/0), with two rows worth keeping: an address inside the window whose
8-byte span is not (model traps, CPU faults at the page boundary —
agreed) and a byte-order pin (store 0x8877…11, read back 11…88). Open
(not a gate): nothing pins the tier DELEGATION itself in a probe — the
world/vector catch-alls carry it. Finding: `bin/check` does not flag a
non-exhaustive `match` (a missing arm goes stuck), so a green check is
not evidence a ctor census is complete — the census was done by reading.

**C3a — impc v0 + THE FIRST ORACLE (2026-08-22).** tools/impc/impc.shard
(159/0): spec → call-tier imp over the runtime. The scheme as landed
(the file header is the record): immediates = two's-complement words
(i63 ints 2n+1, nullary ctors 2·tag+1, Bool 1/3); compound ctors =
runtime cells filled slot by slot; locals = one U64 per binder and per
sub-expression (no allocation, #25); ownership = borrowed params and
pattern binders, owned temporaries and let-binders, with an IMMEDIATE
state so literals/arithmetic/nullary ctors generate no count traffic
(own-position consumers inc a borrowed reference; borrow-position
consumers dec an owned temporary after use); `+ − *` with the band check
→ `IpFail FOverflow` (add/sub by the sign-xor test on the doubled words,
mul through unsigned magnitudes with the divide-back wrap check and the
2^62 bound); `lt`/`le` by the xor-2^63 bias, `int_eq` direct; match =
per-arm ok flag through nested tag/field tests (fields loaded into
fresh locals = borrowed binders), no arm left → `IpUnreach`; calls =
IpCall into the table (runtime fns 0–3, then the module's own fns in
definition order); every fn's prologue bumps the depth cell `[rb+16]`
and fails stack at the bound (100000), the epilogue restores it. Product
= a shard file (NAME_ipfn values, NAME_ix indexes, SRC_iprog = rt_prog
over the table), `impc SRC OUT.raw` then shardfmt, the impgen
convention. **The micro-flagship** tools/impc/fixtures/micro.shard (13
fns: len/app/wsum/perim/second/sumto/rev_go/pick/find_neg/mod_free/
shapes/perims + id; 15 wrappers) compiles to a 1922-line product (36
runtime count calls) and **the run-mode differential micro_run.shard
agrees on all 15 wrappers + the gate (16/16)** — spec evaluated by the
engine vs iprun over the product — the first oracle of the generic
path, before any certificate exists; corpus rows: check × 4, the regen
byte-tie `impc_micro_regen`, the differential `impc_micro_run`.
Findings: (i) `(use (:: kernel term chars_of_sym))` aliases a reducer
PRIM to a nonexistent definition and every call through it goes stuck
(term.shard's own warning; cost an hour) — spell prims bare; (ii)
negative literals must be emitted as WORDS or the wk gate rejects the
product (caught by the gate row, not by the behavioral rows — the
machine's U64 ops wrap either way); (iii) run-mode "stuck" surfaces
only at the extern boundary as a malformed byte list — stage the writes
to bisect. NOT YET (C3's second half): the generated per-fn certificates
(P7) — nothing is proven about the product beyond the differential; the
x86 leg of the product (C1c: to_x86 over IpStmt with the caller-save
convention, impgen's arms) is also still owed; v0 refuses ediv/mod/bit
ops, symbols, externs, and packs no static data (P8).

**C1c-1 — THE FRAME TIER: the x86 leg of the call tier, at the model
(2026-08-22).** models/imp/to_x86.shard grows an additive section
(`ixf_*`, 448/0): locals live in MEMORY — R15 is the frame pointer, local
i is the word at [FP+8i], expression temporaries spill to slots past the
locals, RAX/R10/R11/RDX are the only scratch; a call writes the
arguments into the callee's frame, bumps FP by the caller's own size,
XCalls, drops FP back (the callee owns the whole scratch file, so
nothing is saved across calls — P5's caller-save convention
degenerates to "nothing live in registers"); the callee's prologue
zeroes its extras; `xparams` is 0 everywhere (arguments never travel in
registers — a driver pre-writes the entry frame); Fail = RDI := 70 +
family, XCall the exit shim appended as the last image fn; Unreach = a
word load no window reaches. Comparisons materialize 0/1 through the
fused-branch blocks; `IDiv`/`IRem` through RDX:RAX; shifts only at
constant counts (impc emits nothing else). `ixf_prog : IpProg → Option
XModule` gates on `ipwk_prog` first. REJECTED-because (recorded): reuse
the base tier's register-home statement tier — it admits ≤12 locals and
impc's products exceed that (len: 16), and every call would need
save/restore of the live home file anyway; the memory-locals tier has
ONE alignment relation for every statement and is what #25 later
prices against. Cost recorded: every local access is three instructions
because `Addr` carries no displacement — `(ADisp Reg Int)` is the named
door. **The three-way differential** tools/impc/fixtures/micro_x86_run.shard
(spec natively / iprun on the product / the x86 model on ixf_prog's
translation) **agrees on all 15 wrappers** (`impc_micro_x86_run`), first
run. impc's runtime base moved to 65536 (stock vm.mmap_min_addr) so one
product serves the model and the ELF; the window is [65536, 65536+2^20)
with the heap at +152 and frames from +2^19 (driver-chosen, not yet a
product parameter — P4's address policy leaves the carve to the bin).
Owed: C1c-2 the silicon leg (ELF + runner, Opus-delegated), C1c-3 the
imp ⊑ x86 certificates for this tier (impgen's arms in the ratified
dialect — with one uniform relation the validator shape of CERT.md §4 is
the candidate).

**C1c-2 — ON SILICON (2026-08-22; Opus-delegated).** tools/impc/fixtures/
micro_x86_write.shard (462/0) packages `ixf_prog` of the product plus a
trampoline appended AFTER the exit shim (indexes untouched): R15 := the
stack base, rt_init's two arguments pre-written into the entry frame,
`call rt_init`, `call K`, `shr rax,1` → RDI (one logical shift IS the
i63 decode for the exit byte: −1 → 255, −4 → 252), `call exit-shim`;
`enc_image` entry-first, `enc_winelf` at the module's own bounds (base
65536, size 2^20). tools/impc/fixtures/micro_silicon.sh builds one ELF
per wrapper, runs it, compares the process exit status with the spec's
value mod 256 — **15/15 on the 5900X, first run, no encoder refusal, no
fault**; a 16th row pins the emitter's refusal of a non-wrapper index.
The 15 ELFs (74559 B each) differ in exactly two bytes — the `call K`
rel32 — a free structural check that the wrapper index is the only
selector. Honesty note: the exit status is one byte, so this oracle
distinguishes values only mod 256 — the model-level differential
(C1c-1) carries the full Ints; the silicon leg carries the bytes, the
encoder, and the loader. Registered as `impc_micro_silicon`.
**Finding (CI):** pipeline 398 went red on a summary line: the run-mode
drivers printed `FAIL rows: 0`, and the CI FAIL-set awk matches any line
beginning `FAIL ` — the probe itself was 17/17. Renamed to `rows failed:
N` in every driver; a driver's summary must never start with FAIL/TYPE!.

**A-0 — THE CERTIFICATE PHASE OPENS: the model amendments (2026-08-22;
order A → C2b → B ratified by the user the same day).** Stating
Theorem A against the landed code (§11.1) moved three things into the
MODEL before any proof: (i) `IpProg.ipstack` — `iprun` now evaluates at
the window `[ipbase, ipstack)`, so the frame region is invisible to imp
and the disjointness premise is free; (ii) `IpProg.ipdepth` — the
call-depth budget is the model's: the SCC threads (dmax, d), `ipcall`
fails `FStack` at the budget and runs the body at d + 1, the frame tier
mirrors the count in R14 (`ixf_enter` / `ixf_leave`), and `ixf_prog`
gates the carve once (`ixf_carve_ok`: base ≤ stack, 0 ≤ depth,
stack + depth · `ixf_maxown` ≤ memsize); impc's own counter and its
`[rb+16]` traffic are DELETED (`ctx_depth`, `depth_bound` gone; the
product's `_iprog` takes `(base memsize stack depth)`; the micro
product regenerates at 1669 lines, was 1922); (iii) nothing else — the
twin, the fuel towers and the projection are proof-facing vocabulary
for A-1. Signature change walked (the thread-division law): the four
SCC members, `MkIpProg` (5 fields), `rt_prog`, the probe's 11 program
spellings, the three drivers, the ELF emitter. Evidence: ipcall_probe
+3 claims (fact 5 at budget 6 = 120, at budget 5 = `IpRfailed FStack`,
budget 0 refuses the entry call) 235/0; rt_run 17/17; **the new
`t_deep` wrapper (sumto 5000 at budget 1000): imp −13, x86 model trap,
silicon EXIT STATUS 72 — the stack family pinned end to end** (the
fail leg had never been exercised past imp before); the other 15
wrappers unchanged on all three legs (micro_run 17/17, micro_x86_run
16/16, micro_silicon 17/17 incl. the refusal row). **THE BUG THE
DESIGN EXPOSED:** impc v0's depth bound was 100000 against a 512 KiB
frame region (≈ 2600–3500 frames of the micro product's sizes) — a
few-thousand-deep recursion would have faulted the process (SIGSEGV)
instead of exiting 72; no differential could have seen it (nothing
recursed that deep). Observation (Opus, from the disassembly):
`enc_winelf`'s `_start` glue already zeroes r12–r15, so the trampoline's
explicit `R14 := 0` is belt-and-braces today — kept, because the frame
tier's convention makes R14 = 0 the driver's obligation, not the
loader's. Owed, unchanged: the lift law, `ls8_id`, the world-tier
exit-code clause (C6).

**A-1 — THE KIT (2026-08-22).** models/imp/probes/fra_kit.shard (1505
lines formatted, 513/0) + std/mem growth (89/0): the vocabulary of
§11.1 with its once-proven laws. std/mem: `ls8_id` (the 64-bit round
trip — owed since C2a; ls4_id's ladder at eight bytes through `mbid7`
and `neq_lo4…7`), the four word-grain framing laws
`load_le_store_le_below/above` and `load_le_set_below/above` (generic
in both widths, induct on the read width), and `store_le_get_congr`
(a store's byte depends only on the base's byte — the law that
compares patch lists below a cut without byte-index arithmetic); the
two byte framing fulfills now cite internal twins `sgb`/`sga`
(std/bytes' precedent). The kit: the Int-height tower `xt c f` with
`xt_peel`/`xt_stop` and `kf K f` (`kf_s`); **the GENERAL SEAM
`xseq_app`** — `xeval_seq (xt c f) (a ++ b) = xcont (xt (c − |a|) f) b
(xeval_seq (xt c f) a)` for every prefix (no purity premise: the same
fuel flows through both sides; A1's `vxg_seam` needed `vx_regis`
because it let the prefix run at a DIFFERENT tower); the projection
`xo_fr`; the patch grammar `FPatch` (word/byte), `fp_mem` (apply,
oldest first), `fp_wordv` (the newest word at a slot), `fbelow` (the
program's patches), the discipline `fp_disc`, and the laws: **`fp_read`**
(a frame read under the discipline is the newest word at its slot;
every other patch is a disjoint skip — `al_lt`, the slot lemma, needs
an INTEGER CUT: 8·(qa − qb) ≥ 1 ⟹ qa − qb ≥ 1, taken as a `have`
whose tight negation the Farkas engine refutes at multiplier 8 — a
one-step combination cannot), **`fp_below_b` / `fp_below_lw`** (below
the cut, bytes and words at any width see only the program's patches),
the `fbelow` and `fp_disc` cons laws. Iteration log (the pricing
datum): every proof closed on its FIRST structural attempt; the
iterations were four Farkas certificates (the engine's slot table in
the trace names every row — read it, never guess), two dangling pivots
(`(inst slo slo) (inst v v)` on IH cites whose conclusion does not
mention them), one rewrite side, two paren counts. Nothing here is
program-specific; A-2 states the expression lemma in this vocabulary.

**A-2 — THE EXPRESSION LEMMA (2026-08-22).** models/imp/probes/fra_kit.shard
(652/0) + models/imp/probes/gen_fra.py. `fe_sound`: for every
expression `ixf_exp` accepts, at every tower at or above `ixf_ecost e`,
from any register file with R15 = fp and R14 = dep over a patch list
under the discipline with the locals read-through (`fe_ctx`), the
emission's run equals `fe_out v RS RUN M'` — RAX = imp's value, the
non-scratch registers carried, the scratch (RDX R10 R11) spelled as
projections of the run itself (no witness), and the memory
`fp_mem mem0 (fp_app (fe_tr … e …) psx)`: the spill-trace twin's patches
on top of the entry list. Structure: one STEP LEMMA per constructor
shape (25: const, loc, ext, trunc×3, shl×2, shr, rotr32, load, six
64-bit ops, three 32-bit ops, div, rem, eq/lt/le) with the sub-runs'
induction hypotheses as explicit premises — each checkable alone — and
the induction `fe_sound` that case-splits the emission exactly as
`ixf_exp` does (refused shapes close by `None ≠ Some`), derives the
sub-run hypotheses from the IHs (a at the entry file, b at the
post-spill file over `psx2 = FWord addr va :: Ta ++ psx`, with
`fe_ctx` rebuilt at d + 1 through `ctx_intro`), and cites the step
lemma. Supporting laws landed on the way: `fe_len` (emission length =
`ixf_elen`), `fe_band` (every imp value in [0, 2^64) under banded
locals + constants-in-band — the spilled word's `ls8_id` needs it), the
twin's laws `fe_tr_disc/locs/below/min`, the seven straight-line block
lemmas, `ixf_dep/elen/oplen_nonneg`, the product-order kit
(`mul_nonneg`/`le_mul_r` ported; `ediv_nonneg`/`ediv_le_self` from the
kernel's euclidean axioms), `pow2_32/64` in std/bits, the 15 register
accessor laws + `fe_out_of_norm` (the closing never EVALUATES an
accessor: evaluation opens it on the opaque sub-run file).
**Two translator findings:** the frame tier now REFUSES U8 binary ops
(a 64-bit add of bytes ≠ imp's byte wrap) and the U32 BITWISE trio (the
32-bit forms mask, imp's band/bor/bxor at U32 do not — agreement would
need a per-kind operand band) — neither is emitted by impc or the
runtime (products are U64; U32 = the address ITrunc), so the generic
path loses nothing; differential 16/16 unchanged. **The pricing datum**
(this is what #27's engine would replace): fra_kit.shard = 36k lines,
of which ~24k are generated by gen_fra.py (in-tree, banner-marked,
`splice`-regenerable; the 2026-07-12 ruling's prescribed form; #18
commented). Every generated claim closed on its FIRST structural
check; the iterations were certificate multipliers (read off the
checker's slot table), paren counts in the generator, `(inst …)` pins
for pivots, and the compute stop-set discipline. Hand-written: the
kit's laws, the block lemmas, four step lemmas (the templates). Gotchas
recorded in memory: compute stops are one form `(stop a b c)`; compute
does not enter a stuck `if`'s branches; `unfold` is single-occurrence
outermost-first (re-spell a nested target as an equation have instead);
`rewrite` is all-occurrence and side-restricted, rewrite-with one
occurrence; a premised lemma needs rewrite-with; `div-facts` needs a
literal divisor; an induction binder must not shadow a claim variable.
Owed to A-3: the statement tier over this (`IpSet`'s store of RAX into
a local = `fr_locs` growth; `IpStore`/`IpLoadW`/`IpStoreW` = below-the-cut
patches; `IpFail`/`IpUnreach` = the trap clauses).

**A-3 — STRAIGHT-LINE STATEMENTS, parts 1–2 (2026-08-22; commits
7133737, fb80053).** fra_kit.shard 664/0. The statement vocabulary —
`fs_out` (every scratch register incl. RAX as the run's own), `ips_tr`
(the straight-line statement twin: the slot word on top of the
expression spills), `fl_set`/`fr_locs_set` (the locals relation under a
set), `ilset_some/lo/hi`, `int_eq_eq` — and three step lemmas generated
by gen_fra.py's `stmt` mode: `fs_step_set` (`fe_sound` → `fe_st_run` →
`fs_out`), `fs_step_fail` (RDI := 70 + family, XCall the exit shim — Some
XTrap in the pure tier), `fs_step_unreach` (the word load no window
reaches traps either way; case-split on the low guard). Part 3 —
`IpStore` / `IpLoadW` / `IpStoreW` — is OWED: the inline attempt fought
the `unfold xminstr_leaf cannot reach a match-arm body` gate; they want
once-proven word-load/store block lemmas (`fe_ldw_run`/`fe_stw_run`, the
`fe_st_run` shape) first.

**A-4 — CONTROL LANDS (2026-08-23; commits 4372773 + this).**
models/imp/probes/fra_kit.shard 726/0 (102743 lines formatted, of which
39610 are gen_fra4.py's) + models/imp/probes/gen_fra4.py (the A-4 emitter,
importing gen_fra.py). THE THEOREM OF THE SLICE, `ipt_sound` — §11.1(5)'s
ONE induction: a request `r` (a statement, a statement list, or a while
request — the loop body under `xeval_loop`, since the engine re-enters on
exactly that list) that the frame tier accepts (`ixt_emit r = Some is`),
at imp fuel `f` and at EVERY x86 tower `xt c (kf K f)` with `c` at or
above the request's own cost and `K` bounding every statement cost in
the tree (`ixf_skok K (ixt_body r)`, `1 ≤ K`), from a context file
(`fe_ctx`, R14 = dep, R15 = fp, constants in band, the spill depth within
the window), runs to `ixt_expect r out RS RUN MEM'`: a NORMAL imp outcome
is `fs_out`/`fw_out` over the twin's memory (`ipt_tr`: the statement
patches newest-first, outcomes stay `ipstmt`'s own), a FAILED outcome is
`Some XTrap` (the exit shim), a TRAPPED outcome is the run itself (imp
traps are outside the theorem, §11.1(1)). Under the A-4 FENCE `ixf_a4`:
no call (A-5), no byte store / word op (A-3 part 3) — the induction's
arms for those are refusals, lifted by regenerating.

Structure (all generated, every claim closed on its first structural
check; the iterations were Farkas slot counts, binder pins, paren
counts, and the two DSL facts below): the side-predicate EXTRACTIONS
(25: `a4/scb/skok` × tail/head/if_t/if_e/while_b + the expression parts +
`skok_cost`, `sdep_*` as `imax2` inequalities); THE TWIN LAWS `ipt_mem`
(imp's memory after a normal run = the base under the twin's patches
below `slo`) and `ipt_ctx` (`fe_ctx` preserved at the post-state) — the
first two instances of the dispatcher induction, imp-side only; nine
x86 BLOCK LEMMAS over an explicit S-tower (if then/else × norm/trap,
loop-rest exit/norm/trap, loop-statement brk/trap — the block/loop
choreography proven once, outcome-specific so no wrapper function has
to mirror the interpreter's match shape); nine control STEP LEMMAS
(`fs_step_nil/cons/cons_fail/if_t/if_e/while`, `fw_step_exit/iter/
iter_fail`) generic in the imp outcome through `ixt_expect`, the
sub-runs' simulations as premises — the A-2 recipe at statement
granularity; `slen_cost` (emission length ≤ statement cost); and the
induction itself (673 generated lines before formatting): each arm
decomposes the engine's run, derives the sub-requests' premises (sides
by extraction, the post-state by the twin laws, heights by arithmetic),
re-enters the IH at `xt (c + K) (kf K f2)` through `kf_s` + `xt_add`,
and cites the step lemma.

**Three findings that moved into the TRANSLATOR** (to_x86.shard,
output-identical: differential 16/16, silicon 17/17, micro_elf
byte-identical): (1) THE BLOCK DISCIPLINE — a nested statement list (a
loop body, an else branch) enters through its own `XBlock`. The encoder
emits nothing for a block, but `xeval_seq` hands one fuel unit per
POSITION, so a spliced list would put the instruction after it at
"position + the list's length", a quantity bounded only by the imp fuel
tower — exactly the Nat-inequality swamp §11.1(4) rejected; a block body
runs at its block's own fuel and the instruction after the block at the
block's position, so every statement's emission length is LOCAL (a
block counts one), the general seam is applied at statement granularity
only, and the per-program ratio `K` is a maximum over statement costs
(`ixf_scost`: ecost + 4 for a set, + 3 for an if, + 4 for a while, 8 for
a fail). The then branch needs no block (it is already the tail of the
cons after the inner block). (2) `ixf_stmts` became the mutual pair
`ixf_stmt`/`ixf_stmts` — "a list runs its head's emission then its
tail's" is now the definition, not a nine-arm lemma. (3) `ixf_sdep`
became `ixf_sdep`/`ixf_sdeps`; likewise every list predicate of the
proof tier is a mutual pair with the per-statement part NAMED, so an
extraction unfolds one layer and never spells a match term. Also found:
`fs_step_unreach` is not a clause of the theorem (IpUnreach is an imp
trap — outside); it stays as the machine-behavior record.

**Proof-DSL facts (canonical, new this slice):** `case-on` on a VARIABLE
does not substitute — the fact is `hyp 0` and every re-spelling of a
premise must rewrite the captured constructor equation (and an
outcome variable must be folded BACK, `rl`, before the premise whose
right side names it); every `have` and every `inject` product along the
path is a Farkas slot, in chain order — certs come from a tracker, never
by hand; `reduce` does not enter the branches of a stuck `if` (fold an
inner literal `if` by `if_f`/`if_ff`); claim premises are cited by
index, haves by name; a lemma binder the goal pattern leaves dangling
needs an `inst`; `(int_eq 1 0) = False` negates to an EQUATION and is
refuted with G = −1; `wf-induct`'s IH carries the ordering premises
after the claim's own; `shardfmt` must follow every splice (the
generators emit one-liners — 102k lines formatted from ~64k emitted).
Owed: A-3 part 3 (lifts the store/word-op fence), A-5 (calls: args as
patches at fp + own + 8j, frame disjointness, the R14 mirror, `ixf_kok`
over fn bodies), A-6 (`valid_frame` + `iprun`/`xrun_fn`, the promotion of
fra_kit out of probes/).

**A-3 — PART 3 LANDS: THE MEMORY STATEMENTS (2026-08-23).**
fra_kit.shard 741/0 (116769 lines formatted; 726/0 at A-4). `IpStore` / `IpLoadW` /
`IpStoreW` are lifted from the fence — `ixf_a4s` now refuses only the
call (A-5). Three hand BLOCK LEMMAS over an explicit S-tower, the
`fe_st_run` shape: `fe_ldw_run` (RAX := [RAX]), `fe_stw_run` ([R10] :=
RAX, the word), `fe_stb_run` ([R10] := RAX, the byte) — `xt_peel2`, one
`compute` with the two window guards rewritten, `compute` again. The
`unfold xminstr_leaf cannot reach a match-arm body` gate part 2 fought is
not on this path at all: with the fuel a constructor tower, `compute`
runs the interpreter THROUGH `xminstr_leaf` with no unfold, exactly as
`fe_st_run` / `fe_rl_run` already did for the frame slot. Step lemmas by
`gen_fra.py stmt` (the block now carries its banner and is spliced like
the others; `splice` prepends a banner an emitter omits — the twin-laws
banner had been lost that way once): `fs_step_loadw` (fe_sound →
`fe_ldw_run` → `fp_below_lw` through the twin: the word below the cut
read from the machine's memory IS imp's → `fe_st_run`), `fs_step_store`
/ `fs_step_storew` (ONE generator: the binary choreography of A-2 with
`fe_sound` cited for both operands, the context at d = 1 rebuilt by
`ctx_intro` as fe_sound's own IBin arm does, the reload's `fp_read`, then
ONE patch below the cut — `fp_mem_cons_b` / `fp_mem_cons_w`). The twin
needed nothing new: `ips_tr` spelled the three arms at part 1, and the
patch laws (`fbelow_b_lo` / `fbelow_w_lo`, `fp_disc_b` / `fp_disc_w_lo`,
`fr_locs_b` / `fr_locs_skip_lo`, `fp_below_lw`, `ldw_lo` / `ldw_hi`) were
all in the kit since A-1. `gen_fra4.py`: eight extractions
(`scb_{store,storew}_{a,v}`, `scb_loadw_a`, `sdep_{store,storew,loadw}`),
the engine decompositions `arm_store` / `arm_loadw` shared by the three
dispatcher inductions, T1/T2 leaves (T2's set leaf generalized to
`set_like` — the word load is a slot store of the loaded word, in band
by `ldw_lo/hi`), the `ipt_sound` arms, the `slen_cost` arms. Every claim
closed on its first structural check except the two store lemmas, which
failed once on an unresolved citation: `add_zero_r` is std/mem's
INTERNAL lemma (not in its mod.req), so the kit has its own `add0` — the
spill slot is spelled `(+ nl 0)` by `ixf_spill nl 0` and `nl` by
`ips_tr`, reconciled by one rewrite on the machine side. No translator
change. A finding worth recording: the machine's memory guard is WEAKER
than imp's (the x86 fence is the module window [xmemlo, xmemhi), imp's
the program window [mlo, slo)); the theorem never sees the gap because
an imp trap is outside it (§11.1 (1)), and on the normal leg imp's
guards imply the machine's — the window facts of `fe_ctx` are exactly
what the step lemma spends. Gotchas: a `(rewrite (lemma X) …)` citation
resolves against the file and the `use`d modules' mod.req SURFACES, not
their internals; `unfold fp_app rhs` is unsafe when the RHS's register
file carries a sub-run whose memory contains an earlier `fp_app`
(outermost-first picks THAT one and leaves a stuck match) — state the
unfolding as an equation have over the patch list alone; `(compute
both)` closes `(xw8) = (iw8)`. Owed: A-5 (calls), A-6 (the theorem).

**A-5 — THE CALL: DESIGN (2026-08-23; written before the first edit, the
ground truth for the slice).** Theorem A's last statement form. The
statement `(IpCall i k args)` compiles to `iargs ++ [R15 += own; XCall k;
R15 −= own] ++ ixf_st i`, the callee to `enter ++ zero ++ [XBlock ib] ++
ir ++ leave` (see the translator changes). The proof follows §11.1's
design: the call engine is the FOURTH request `(IqCall k)` — its imp side
is `ipcall f … k lc mem` (the request's locals ARE the argument values,
so `fe_ctx`'s `fr_locs fp lc psx` says "the args sit in the callee's
frame at fp"), its machine side `xeval_call`, its expected outcome
`fe_out v` (RAX = the result, R14/R15 carried, the scratch as the run's
own), its cost 0 and body `Nil` (every per-fn fact comes from the TABLE
premises instead). At a call statement at imp fuel S f2 the callee runs
`ipcall f2`, the IqCall IH at f2; inside the callee at S f3 the body runs
`ipstmts f3`, the IqStmts IH at f3 — the one induction absorbs both
levels with no off-by-one, which is why the call is a request and not
an inline decomposition. The caller instantiates the IqCall IH at
fp := fp + own, lc := the arg values, nl := their count, own := 0 (the
callee's frame facts are the callee's business, derived from the table).

TRANSLATOR (models/imp/to_x86.shard; bytes identical for every program
the corpus compiles): (i) `ixf_fn` puts the body in its own `XBlock` —
the block discipline at fn level, for the same reason as A-4's: the
result expression after the body needs LOCAL fuel, and a spliced body
would leave it "position + |body|"; (ii) `ixf_fn` REFUSES a fn with a
non-U64 parameter (`ixf_p64`): `ipcall` bands the args to their kinds on
entry (`iband_args`, `mod v (ikmod k)`) while the machine stores the raw
word — they agree exactly at U64 (the identity on a banded value).
Every compiled program has U64 params (rt.shard's four fns, impc's
`u64s`); the only non-U64 params live in ipcall_probe (imp-only). The
named door: kind soundness (a well-kinded arg's value lies in its kind's
band) would lift the restriction and is a separate induction. (iii) THE
CARVE GATE RESERVES ONE MORE FRAME: `ipstack + (ipdepth + 1) · maxown ≤
ipmemsize`. THE BUG THE PROOF FOUND: the args are stored into the
callee's frame BEFORE the depth check in the callee's prologue, so a
call at depth = ipdepth writes `8·nargs` bytes above the last budgeted
frame; with the old gate those stores can fall outside the window, and
the x86 model TRAPS where imp fails `FStack` (the silicon: a fault
instead of exit 72). One frame of slack closes it (the micro drivers'
512 KiB / depth 1000 have room).

THE TWIN grows `own` (the SCC `ipt_stmt/ipt_while/ipt_stmts/ipt_tr` take
it after `nl`: the arg patches live at fp + own + 8j) and a fourth member
`ipt_call` (zero patches `fz_tr`, the body's twin at the callee's frame,
the result's spill trace — newest first; `Nil` off the normal leg); the
statement twin's IpCall arm is `(FWord (fp + 8i) v) :: ipt_call … ++
ipt_args …` with `ipt_args` the structural arg twin (each arg's spills
then its slot word). New patch vocabulary: `fp_wmin slo lo ps` (every
word patch is a program patch or sits at ≥ lo — THE FRAME-DISJOINTNESS
predicate), `fp_away ps a` (no word patch at a), `fe_tr_max` (the fifth
generated twin law: the spills lie below fp + 8(nl + d + dep e)) — the
arg slot lemma needs both a min and a max bound because the later args'
spills sit BELOW the earlier slots while their stores sit above.

THE PREMISES (appended; indices 0–11 unchanged, the fence premise 9 goes
vacuous this slice and is deleted in the follow-up): `ixt_fr r nl own
(ixf_maxown fs)` (the frame bundle: 8(nl + 1 + sdep body) ≤ own, own ≡ 0
mod 8, own ≤ maxown — True for a call request), THE ROOM INVARIANT
`fp + own + (dmax + 1 − dep) · maxown ≤ xmemhi` (uniform across requests
because the call request's own is 0: the callee's frame is the
(dep+1)-th), `0 ≤ dep`, `dep ≤ dmax`, `dmax < 2^64` (the unsigned CLtU
and the R14 increment), the TABLE `ixf_fns dmax fail_ix fs = Some xfs`
and `xfuncs_of m = xfs ++ [shim]`, and the per-fn predicates `ixf_fnsok
fs` (U64 params, constants in band, the result's band) and `ixf_fnskok
K fs` (fn cost `ixf_fcost` ≤ K, the body's statement costs ≤ K). A FOURTH
dispatcher instance `ipt_min` (imp-side): the twin's patches satisfy
`fp_wmin slo fp` — the callee writes only at ≥ fp + own, so the caller's
locals survive (`fr_locs_app_wmin`). Order: ipt_mem → ipt_min → ipt_ctx
→ ipt_sound.

THE LEMMAS (hand, explicit S-towers where straight-line): the call-site
blocks (`fc_addfp/subfp/call/argst_run`), the callee blocks (`fc_enter_ok/
enter_fail/zero1/leave/body_blk_run`), the ZERO lemma `fz_sound` (wf-induct
on nl − from) with the twin's laws, the ARG lemma `fa_sound` (induction
on args: `fe_sound` at each arg, then the slot store, the context
re-established via `fr_locs_skip` + `al_shift_own`) with `ipt_args`'s
laws (`fa_below/disc/locs/wmin/slots`), the table laws (`ixf_tbl_at`,
`ixf_maxown_at`, `fnsok_at`), `iband_id`, and the STEP LEMMAS: call site
`fs_step_call` / `fs_step_call_fail`, callee `fc_step_norm` /
`fc_step_fail` / `fc_step_stack` — every sub-run's simulation an
explicit premise, the twins abstracted as patch-list variables so the
step lemmas never spell `ipt_*`.

**A-5 — THE CALL: LANDS (2026-08-23).** fra_kit.shard **856/0** (741 at
A-3 part 3; +115), to_x86.shard 470/0, the impc micro differential
fixture rows failed: 0, and the splice → shardfmt cycle is a byte-level
fixpoint for every generated block (gen_fra.py: fe_len twin trmax
fe_band unary binary sound stmt; gen_fra4.py: extract t1 t15 t2 ctl
slen t3 — t15 is new). Departures from the design record, none
structural: (i) THE FENCE DIED THIS SLICE, not in a follow-up —
`ixf_a4`/`ixf_a4s` are deleted, `ipt_sound`'s premise list is the
design's 20 with no vacuous slot; there is no fence anywhere in the
file. (ii) `ixt_lc` became **`ixt_post`**: the call request's outcome
carries the RESULT, not a locals frame, so the context law's post-state
is per-request — for `IqCall` it exports `fe_ctx … Nil ps ∧ fe_inband
(ihd lc2)` (the band is what the caller's `fr_locs_set` needs for the
write-back), for the others `fe_ctx … lc2 ps` as before; extraction
lemmas `post_call_ctx` / `post_call_band`. (iii) The caller cites the
IqCall IH at own := 0 in `ipt_sound` but at own := own in `ipt_ctx`
(the twin term under `ixt_post` mentions the caller's own); `ixt_fr
(IqCall _) = (le 0 own)` discharges either way. The LAST BUG was not a
proof gap but a lemma's: `fs_step_call_fail` carried a vestigial
`ilset` premise copied from the normal-leg shape — undischargeable on
the fail path (no write-back happens) and unused by its own proof.
Removing a premise is SURGERY: every flat Farkas list in the body
carries one slot per scope item, so the dead premise's index (11: goal
negation at 0, premise j at j+1) had to be deleted from all 17 lists,
not merely left zero. What else the slice taught the DSL user: lemma
citations resolve in FILE ORDER (`fe_tr_max` needed its own generated
block after the laws it cites); two-sided eq certs are `(list LE GE)`
with G multiplying L−R−1 on the LE side; a wf-induct IH's measure
binder is fresh and unnamed — cite `(hyp 1)` with NO insts and spell
the index so the measure appears in the conclusion (`(- nl n)`), with
the ordering discharges after the premises; `(int_eq X Y) = False`
goals take a negated row X = Y; a case-on binder must not shadow a
claim variable (`r` → `rt`). Named doors unchanged: KIND SOUNDNESS
(a well-kinded arg's value lies in its kind's band) lifts `ixf_p64`'s
U64-params refusal; A-6 (the theorem: `ipt_sound` at the program's
entry, the table premises established once from the carve) is next.

**A-6 — THE THEOREM: DESIGN (2026-08-23; written before the first
edit, the ground truth for the slice).** The program tier: `ipt_sound`
cited ONCE at the program's entry — the request `(IqCall k)` at dep 0,
own 0, fp = slo = ipstack, lc = the entry argument values — with every
premise established from `(ixf_prog p) = (Some xm)` (valid_frame IS
this equation; no wrapper predicate), the imp run, and the computable
side premises. Prefix `fra_` = the program tier's lemmas in fra_kit.

TRANSLATOR (one change): `ixf_carve_ok` grows the three SCALAR WINDOW
BOUNDS — `0 ≤ ipbase`, `ipmemsize ≤ 2^32` (fe_ctx's addressing bound),
`ipdepth + 1 < 2^64` (premise 15, the unsigned CLtU) — the gate owns
every scalar window/depth fact (A-0's precedent; without the third the
derivation is NONLINEAR: mul_nonneg through the entry fn's own ≥ 8).
Output-identical for every `ixf_prog` consumer in the tree (micro:
base 65536, memsize 1114112, depth+1 = 1001 — all pass; fra_kit never
mentions the gate today, so no proof breaks).

THE PREMISE LEDGER (ipt_sound's 20 at the entry): 0/3/5/11 compute
(IqCall emits Nil, body Nil, `ixt_fr` call = le 0 0); 1 = the iprun
premise re-spelled (iprun unfolds to ipcall at dep 0, `ipt_run
(IqCall k)` = ipout_of_ret ∘ ipcall, the entry patches' fbelow
vanishes); 2 = fe_ctx (scalar clauses from the gate; fr_locs/fp_disc
from the arg-patch laws below); 4 from the arity (extracted from the
run) + `ixf_own g ≥ 8·np` + `ixf_maxown_at` + the carve; 6/18/19 stay
THE THEOREM'S computable premises (`1 ≤ K`, `ixf_fnsok`, `ixf_fnskok
K` — the KIND-SOUNDNESS door subsumes 18 later; the ratio is
genuinely per-application data); 7/8 from `0 ≤ c`; 9 = the shim — NEW
induction `ixf_shim_at`: under the table premise, `xfunc_at (xf_app
xfs tl) (ixf_count fs) = xfunc_at tl 0`; 10/13/14/15 from the gate;
12 = THE ROOM INVARIANT IS THE CARVE GATE VERBATIM at fp = slo,
own = 0, dep = 0 (the design's checksum); 16/17 by inject on
ixf_prog's match tree.

THE STATEMENT — general form, then the entry corollary. General:
premises valid_frame, the run at `(fp_mem mem0 (fbelow slo psx))`,
`fe_ctx xm (ipbase_of p) slo slo (ilen args) 0 args psx`, and the
computable four; conclusion at `xrun_fn (xt c (kf K f)) xm k rs0
(fp_mem mem0 psx)` with rs0 = MkRegs over 13 scratch binders, R14 0,
R15 slo. Proof: cite ipt_sound at c := c + 1 (xt_peel, an add-sub
have), then bridge `xeval_call (S g)` → `xrun_fn g` — NEW lemmas
`xrf_norm`/`xrf_trap` (case xeval_seq's five outcomes; both need
`xfunc_at = Some`, extracted from the run via ipfn_at + `ixf_tbl_at`
+ a NEW xf_some lemma). The clauses: `fra_sound_v` (RAX = v, machine
memory = `fp_mem mem0 (fp_app TW psx)`, TW the entry twin — the
witness spelled, §11.1(3)); `fra_sound_mem` (imp's memf = the fbelow
view — ipt_mem cited); `fra_sound_ctx` (`fe_ctx … Nil (fp_app TW
psx)` — ipt_ctx + post_call_ctx: THE CHAINING CLAUSE, the next
call's premise 2); `fra_sound_fail` (xeval_call = Some XTrap) and
`fra_sound_fail_none` (xrun_fn = None — the pure tier cannot tell
the shim from a trap, A-0's record). Entry corollary at psx :=
`fa_words slo args` (NEW value-level arg patches; laws: fr_locs
given the band, fp_disc by 8-alignment, fbelow vanishes → the run
premise at mem0 itself) — the bin boundary's exact shape
(micro_x86_run's init_mem is this term). Entry extractions carry no
∃: opt-defaulting predicates (A-1(4)'s idiom) shared across the five
clauses — the run forces S-fuel, `ipfn_at` Some, the arity.

THE INSTANCE (the citation demo): micro, INSIDE fra_kit (importing
micro.shard + micro_ipc_out.shard — probes importing tools fixtures
is precedented by imp_x86_bridge). Cross-file CLAIM citation is NOT
available (a citation resolves against the file + used modules'
mod.req; models have no mod.req) — PROMOTING the theorem to a
citable surface is a NAMED DOOR the composition phase (C2b, B) will
need. Shape: the PREMISED instance — xm/v/memf universally bound,
pinned by the ixf_prog/iprun premises; every COMPUTABLE premise
discharged by compute against the real product (the gate, fnsok,
fnskok at a literal K, the entry args' band); run values stay the
differential row's business (no run literals in certs). Optionally
one chained wrapper through fra_sound_ctx.

**A-6 — THE THEOREM: LANDS (2026-08-23). THEOREM A IS COMPLETE
(A-0 … A-6 all landed).** fra_kit.shard **905/0** (856 at A-5; +49),
**fra_micro.shard NEW** (the corpus row: 908/0 over its closure),
to_x86.shard 470/0 with the widened carve gate, the three-way micro
differential 16/16, and splice → shardfmt still a byte-level fixpoint
with the A-6 hand section appended (its `;; ====` banner is the
generators' block terminator). Departures from the design record:
(i) CROSS-FILE CLAIM CITATION EXISTS for plain file imports — fra_kit
already cites imp.shard's `ilen_nonneg`; only directory-module imports
resolve through a mod.req. So the instance lives in its own probe
(models/imp/probes/fra_micro.shard imports fra_kit.shard and
micro_ipc_out.shard and cites the theorems directly), and the
PROMOTION door the design flagged is not needed: the composition
phase (C2b, B) can import fra_kit the same way. (ii) The statement
layer is wider than the design's five clauses: the GENERAL theorems
`fra_sound_v/mem/ctx/band/fail/fail_none` (any psx under fe_ctx —
`fra_sound_ctx` is THE CHAINING CLAUSE) plus the ENTRY corollaries
`fra_entry_ctx/base/v/fail_none/mem/ctxout/band` at the `fae_words`
patch shape (micro_x86_run's init_mem term), plus `ctx_renl` (fe_ctx
at a different locals bound — nl appears only in the arity clause) so
a chained call re-enters at its own arity. (iii) The instance is
PREMISED and STRONGER than designed: `fra_micro_wrap` covers ANY
wrapper called after init (k2 universally bound — one claim for all
fifteen value rows), `fra_micro_deep` the fail shape (machine None,
C6's exit-72 clause pending), `fra_micro_v` the entry call; the
computable premises (ixf_fnsok, ixf_fnskok at K = 1000, the gate, the
args' band) compute against the real product in ~1 s of check mode.
(iv) The no-∃ machinery is the A-1(4) idiom at Option: extractors
xfns_or/ipfn_or/xf_or with ok-predicates and `_get` lemmas — every
witness is an extractor value, and the main theorems contain NO
case-on at all (each refutation lives inside its own small
extraction lemma: fra_wk/carve, the six scalar gate facts, fra_mod/
funcs/lo/hi/shim/fns_ok, fra_iprun/at_ok/arity/p4/xat). (v) fra_p4's
nonlinear step went as designed: mul_comm + mul_dist respell the
(dmax+1)·maxown atom, mul_nonneg supplies the dmax·maxown ≥ 0 row.
PROOF-DSL FACTS (new): a chain must not carry a bare `refl` after a
rewrite-with that has its own terminal — one closer per chain; bare
`(compute lhs)` folds ground prims that `reduce` leaves stuck
((le 0 0), (mod 0 8), (int_eq 0 0)); a two-sided `(int_eq X X) = True`
goal takes `(list (G …) (G …))`; a citation's match binds ONLY the
conclusion-side variables of the cited side — every other binder,
even one spelled identically in the consumer, is a dangling pivot to
pin with an inst. CORPUS COST: fra_micro's row re-checks fra_kit's
closure — the existing pattern for probes over models (the bridge
probes re-check to_x86 the same way); #37 owns the strategy if this
compounds. NEXT = C2b (runtime theorems: the managed-graph invariant
and framing in the base+patch vocabulary) per the ratified order
A → C2b → B.
