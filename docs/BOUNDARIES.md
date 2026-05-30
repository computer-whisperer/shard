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

### (C) Effect-as-data without continuations: the update loop (MVU)

Mechanism (A) returns the *whole* effect tree from one pure run, with
the rest-of-program embedded as a defunctionalized continuation
(`Symbol` + `apply$`). That needs HOF — hence deferred to the full
language.

There is a narrow-compatible variant that drops the continuation. Split
the program into a **pure step** plus an **external loop**:

```
(type CalcState (CalcState (Option Int)))
(type Action (Print (List Int)) (Exit Int) (Nop))   ; ONE action, no continuation
(type (Step S A) (Step S A))                         ; next-state + action

(fn step ((s CalcState) (line (List Int))) (Step CalcState Action) …)

(app (state CalcState) (init (CalcState None)) (update step))
```

`step : State -> Event -> (Step State Action)` is an ordinary pure,
total function — no HOF, accepted by the narrow kernel as-is. The
**continuation is externalized into the driver's loop** instead of
reified in the Action: the runtime holds the current state, reads an
event, calls `step`, performs the one returned Action, and recurses
with the new state. Where (A) does one pure run that yields the entire
effect tree, (C) does one pure call per event.

The event loop and the Action interpreter live in the untrusted driver
(`check app`, `bin/check.rs::run_app`) — the same trust status as
`eval::eval`, which is already the substrate that runs the narrow
kernel. Non-termination is the loop's, never the object program's.

The **trust boundary is the Action interpreter**: the fixed, enumerable
set of actions the driver knows how to perform (today `Print` / `Exit`
/ `Nop`). Adding an action expands this boundary by exactly one arm —
declared, audited, minimized, per the principle above. Everything the
program decides remains a pure value; the driver only translates
`(Print cs)` into bytes on a descriptor.

Proofs reason about *what `step` produces*: invariants
(`∀ s e. inv s ⟹ inv (state_of (step s e))`), safety
(`∀ s e. action_of (step s e) ≠ (Exit 1)`), and spec-equivalence
(an implementation `step` agrees with a spec `step` — the stateful
sequel to `run = spec_run`). This is the first non-oneshot application
shape that ships in narrow; the continuation-carrying (A) form still
waits on `apply$`. The worked example is `examples/calc/calc_app.shard`.

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

## Audit ledger

For any verified artifact, we want to enumerate its *trust dependencies*:

- the axioms it transitively cites
- the externs the artifact's code (and those axioms) reference
- the bridging axioms it relies on, separately from operational ones

This is a small tool, not part of the kernel: walk the proof DAG,
collect every `Axiom`-tagged theory entry and every `ExternDef`
referenced. The output is the trust ledger for a build.

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
  language. (The continuation-free MVU variant, mechanism (C), ships
  now — see `check app` / `examples/calc/calc_app.shard`.)
- **Runtime linkage** between extern names and native Rust
  functions. Out of the kernel; a deployment-time concern.
- **Audit ledger tool.** Easy once the data shapes are stable; just
  hasn't been written.
- **Bridging-axiom distinction.** Tag on `Axiom` entries; not needed
  for v2.

## Open questions surfaced by writing the code

These are worth deciding deliberately when the relevant work lands
rather than being defaulted into:

- **Strictness of the "all foreign contact through effect-as-data"
  rule.** Direct extern calls work but are not proof-friendly. Should
  the type system forbid them in proof-relevant code, or is it left
  to convention?
- **Where axioms live.** Currently inline in the theory file. Could
  move them to live with their corresponding `extern` declaration in
  the module for locality, or fold them into trait-with-laws when
  traits land.
- **Whether the runtime linkage step is data (a config file mapping
  extern names to Rust function paths) or code (Rust registration
  calls).** Affects how a deployment "links" against a verified
  artifact.
