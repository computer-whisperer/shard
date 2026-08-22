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
