# Modeling and Connecting to Foreign Systems

The language is pure and total: every reduction is deterministic,
every function returns a value, nothing in an expression is allowed
to "do" anything in the world. The proof story rests on that —
`simp`/reduction is soundness-critical, and effects break determinism.

Real software has to read files, talk to networks, drive hardware.
So we need a discipline for crossing the boundary without breaking
the purity above it.

## The principle

Wherever the verified world meets the unverified world there is a
trust boundary. The discipline is:

- **Proofs reason above the line.** Programs are pure functions
  producing values; theorems are about what they produce.
- **The runtime translates below the line.** A Rust-side driver
  interprets the values the program produced into real effects.
- **Crossing the line is declared, audited, and minimized.** Every
  boundary crossing has a name, a signature, and (where possible) an
  explicit model — never an ambient effect leaking into pure code.

This is the same shape TRANSFER commits us to for concurrency
("represent the schedule as a value and execution as a pure fold"),
generalized to all foreign contact.

## Two mechanisms

> Historical spelling note (2026-07-12): the examples below predate the
> `Bytes` former revocation (issue #15) — read `Bytes` as `std/bytes`'
> opaque type (byte payloads at the wire are `(List Int)`). The
> mechanisms themselves are unchanged.

### (A) Effect-as-data

A program that "does I/O" actually returns a value of some `Action`
type describing what to do. The program is a pure function from
inputs to an Action tree:

```
(type Action
  (Done)
  (ReadFile  Path Symbol)        ; result handed to a continuation
  (WriteFile Path Bytes Symbol)
  (Print     String Symbol)
  (Fail      String))

(fn main ((args (List String))) Action ...)
```

The Rust runtime walks the tree: `(ReadFile p k)` reads `p`, then
calls `(apply$ k contents)` to get the next Action, and recurses.

Same input → same Action tree, always. Proofs reason about *what
tree the program produces*; the runtime is the only piece that varies
with the world.

The `Symbol` in each constructor is a defunctionalized continuation
(see the full-language `apply$` design). Effect-as-data therefore
needs HOF and is a **full-language feature** — it lands when the full
evaluator does. **In narrow we cannot write programs in this style.**

### (B) Extern + axiomatic theory

For operations that don't fit "build a tree, return it" — a single
MMIO register read, a third-party library function, a syscall to be
called directly — declare the function with a signature but no body
and add axioms describing its behavior:

```
(extern read_bytes  ((p Path))            (Option Bytes))
(extern write_bytes ((p Path) (b Bytes))  Unit)

(axiom read_after_write
  (forall ((fs FS) (p Path) (b Bytes))
    (= (read_bytes (with_write fs p b) p)
       (Some b))))
```

`extern` is a function declaration with no body. The reducer treats
it as opaque — calls remain stuck, same protocol as native primitives
(see REVISIT, *Primitive call protocol*). The Rust runtime intercepts
stuck calls to extern symbols and dispatches them to a linked native
implementation.

`axiom` is a `Goal` admitted into the `Theory` *without* a `Proof`.
The kernel marks the entry as an axiom so audit can enumerate it.
Proofs cite axioms exactly like proven lemmas — they are equational
facts in the theory, just admitted rather than derived.

This works in narrow today and is what v2 ships.

### (C) Effect-as-data without continuations: direct-style World threading

Mechanism (A) returns the *whole* effect tree from one pure run, with
the rest-of-program embedded as a defunctionalized continuation
(`Symbol` + `apply$`). That needs HOF — hence deferred to the full
language.

The narrow-compatible mechanism that **ships** is *direct-style World
threading*, built on (B). A program is an ordinary pure function

```
(fn main ((w World)) World …)
```

where `World` is a sequencing token (a clock). Each effect is an
`extern` that threads the World:

```
(extern read_file ((p (List Int)) (w World)) (Pair (Option (List Int)) World))
(extern write     ((bytes (List Int)) (w World)) World)
(extern read_key  ((w World)) (Pair (Option Int) World))
```

The data dependency on the threaded World *orders* the effects, so the
program calls them in **direct style** wherever it needs them — no state
machine, no request/response bouncing.

In a PROOF the externs are uninterpreted (mechanism B): a call is left
stuck and its behaviour is given by axioms (the bridging axioms — e.g.
"read advances the clock by 1"). Because the World threads through, the
sequencing discipline is itself a theorem — e.g. `clock(main w) ≥ clock w`
(monotonic ⇒ no effect reuses a clock), proven by ordinary induction even
for an oracle-driven loop (`examples/io/cat_loop.shard`).

At RUN time the Rust handler (the World effect handler in
`rust_bootstrap/src/bin/eval.rs`) intercepts each stuck extern call and
performs the real I/O. The proof layer never interprets externs — inside
a proof they are permanently stuck symbols, so the **trust boundary is
exactly the extern axioms** (the contract the handler must satisfy),
explicit and auditable. (The checker itself is a World program and reads
files through these same externs; that is harness I/O, not proof
semantics.) Adding an operation is a declared extern + its axiom: the
boundary grows by exactly one arm.

The I/O vocabulary today: `get_args` / `read_line` / `read_key` /
`read_file` (input), `write` / `write_line` (output), `exit`. Worked
examples: the self-hosted evaluator (same World/extern shape) is the
kernel entrypoint `kernel/eval.shard`, run via the `eval` binary;
`examples/io/cat_loop` (the oracle-driven loop and its clock-discipline
theorem); and the snake bin (`examples/snake_game_3/snake.shard`,
interactive key-driven play under a proven formal contract).

Proofs reason about *what the program produces*: invariants and
spec-equivalence of the pure core (`examples/calc/calc_app.shard`'s
`step`, the snake game core), plus the clock/sequencing discipline of the
World loop itself.

*Earlier form, now retired.* (C) first shipped as an **external loop** — a
pure `step : State -> Event -> (Step State Action)` driven by a Rust MVU
loop (`(app …)`) or a request/response loop (`(cli …)`), with the
continuation externalized into the driver and the trust boundary an
enumerable Action set (`Print`/`GetArgs`/`ReadFile`/`Write`/`Exit`).
Direct-style World threading subsumes it — you call effects in place
instead of bouncing Actions/Events through a driver — so `run_app` /
`run_cli` and the `(app …)` / `(cli …)` entrypoints have been removed in
favour of direct-style World programs run by `eval`.

## The bolt-axiom pattern (what shipped contracts actually use)

The worked form of (B)+(C), established by `examples/snake_game_2` and
`tools/shardfmt`: declare the run's **observables** as opaque `sig fn`s
over the World (`w_output` — chunks written so far; `w_input`/`w_reads`
— what the input effects yielded; `w_exit` — the exit code), then admit
one dumb one-line **bolt axiom per (effect × observable) pair** — the
effect's action on its own observable (`write` APPENDS to `w_output`),
and PRESERVATION for every other (`write` leaves `w_reads` alone). The
bolts are the binary's entire trust surface: each is auditable by
inspection, and the `(bin …)` artifact names them in its `trusts` list
so nothing is implicit. Requirements are then stated over `main`'s
observables only — never an internal function or state field — and
proven by symbolic execution through the bolts. See
`examples/snake_game_2/mod.req/mod.req.shard` and
`tools/shardfmt/mod.req/mod.req.shard` for the two reference contracts.

## Modellable externs: the good pattern

Bare extern+axiom is the operational story — "we trust these symbols
behave these ways." It works, but the axiom is doing all the work.

The stronger pattern: alongside the extern, ship a **pure model** in
the language, and admit one *bridging axiom* tying the extern to the
model:

```
(type FS (List (Pair Path Bytes)))

(fn model_read  ((fs FS) (p Path))                (Option Bytes) ...)
(fn model_write ((fs FS) (p Path) (b Bytes))      FS             ...)

(extern current_fs ()         FS)               ; the live FS
(extern read_bytes ((p Path)) (Option Bytes))

(axiom read_bytes_matches_model                 ; <- the only axiom
  (forall ((p Path))
    (= (read_bytes p)
       (model_read (current_fs) p))))
```

Every proof about `read_bytes` reduces (via the bridging axiom) to
a proof about `model_read` — an ordinary function over an ordinary
data structure, with full equational power. The trust burden shrinks
from "all axioms about `read_bytes` hold" to "the model matches
reality" — usually a smaller, more legible claim.

This is the same shape as the pilot's `Mem` ADT (an in-language data
model of memory). It generalizes to file systems, networks, hardware
state, anything pinnable as data.

## Where axioms live (resolved 2026-07-05)

Axioms are authored in exactly two places; the driver's axiom-scope
gate (`kernel/driver.shard::run_srcs`) enforces it, with
`std/axiom_scope_rejects.shard` as the pinned negative fixture:

1. **`kernel/facts.shard` — the reviewed core-math set.** Facts about
   kernel prims that have no derivation route inside the proof theory
   (the euclidean characterization at a symbolic divisor — range,
   identity, and the uniqueness pair; the multiplication ring laws;
   the bitwise/shift defining recurrences). These are the trust floor —
   the same standing as the arith backend and the `div-facts` step:
   exempt from `(bin trusts)` listing, rendered on their own ledger
   line ("kernel axioms (reviewed core)"), and each rides executable
   evidence (`examples/facts_probe.shard` checks every statement
   against the live prims — the axiom file's engine differential).
   Growing this file is a kernel change; review it so.
2. **App/bin trust scopes** — the bolt axioms and bridging axioms this
   document describes, granted per-artifact and named in the bin's
   `trusts` list.

The library trees — `std/`, `meta/`, `models/` — are **theorem-only**:
libraries never grow the trust surface. (std/div's old `mod_lo`/`mod_hi`
axioms moved to `kernel/facts.shard`; an ISA model's trust comes from
its engine differential, never from an axiom.)

## Audit ledger

For any verified artifact, we want to enumerate its *trust dependencies*:

- the axioms it transitively cites
- the externs the artifact's code (and those axioms) reference
- the bridging axioms it relies on, separately from operational ones

This is a small tool, not part of the kernel: walk the proof DAG,
collect every `Axiom`-tagged theory entry and every `ExternDef`
referenced. The output is the trust ledger for a build. The `(bin …)`
artifact's report is the first cut: its `trusts` list names the bolt
axioms and its `requires` list is checked MET/UNMET per build; the
full transitive DAG walk remains to be written.

Distinguishing **bridging axioms** ("the extern matches the model")
from **operational axioms** ("the extern has these direct properties")
in the ledger would make the audit story sharper — the bridging ones
are the load-bearing assumptions in the modellable-extern pattern.
For v2 axioms are uniform; the distinction can be added later as a
tag on the axiom entry.

## What v2 lands

- **`ExternDef`** as a new module-level declaration alongside `FnDef`.
- **`TheoryEntry` tagged as `Proven` or `Axiom`** in the `Theory`,
  so the audit boundary is visible at the kernel layer.
- **Reducer unchanged** — externs are stuck calls, same protocol as
  native primitives.
- **No `Action` type, no `apply$` runtime driver.** Effect-as-data
  needs HOF; it ships with the full language.

## What v2 defers

- **Continuation-carrying effect-as-data (mechanism A).** The
  `(ReadFile p k)` tree form needs `apply$`; lands with the full
  language. (The continuation-free form, mechanism (C), ships now as
  direct-style World threading — see `eval run` / `examples/io/`.)
- **Runtime linkage** between extern names and native Rust functions —
  *now ships* (out of the kernel: the World effect handler in
  `bin/eval.rs`; proofs never interpret externs).
- **Audit ledger tool.** LANDED (trust-hardening arc): the per-bin
  trust LEDGER block — kind-tagged axioms, upstream-granted split,
  dead-trust flags. See `TCB.md`.
- **Bridging-axiom distinction.** Tag on `Axiom` entries; not needed
  for v2.

## Open questions surfaced by writing the code

These are worth deciding deliberately when the relevant work lands
rather than being defaulted into:

- **Strictness of the "all foreign contact through effect-as-data"
  rule.** Direct extern calls work but are not proof-friendly. Should
  the type system forbid them in proof-relevant code, or is it left
  to convention?
- ~~**Where axioms live.**~~ RESOLVED 2026-07-05 — see "Where axioms
  live" above: kernel/facts.shard for reviewed core math, app/bin
  trust scopes for boundary bolts, library trees theorem-only
  (driver-enforced).
- **Whether the runtime linkage step is data (a config file mapping
  extern names to Rust function paths) or code (Rust registration
  calls).** Affects how a deployment "links" against a verified
  artifact.
