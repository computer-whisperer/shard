# modules_demo — bodyless module interfaces (`.req.shard` as `.h`/`mod.rs`)

A throwaway, self-contained demo for nailing down the **exact setup** of the
module system before we touch core (the loader, the gate, or std). Nothing here
is imported by anything real.

## The idea

A module is a directory. Its `mod.req.shard` is the **interface** — the public,
bodyless `.h`: exported signatures and promised lemmas, *no* bodies, *no* proofs.
The dir's other `*.shard` files are the private **impl**: real bodies + the
proofs that discharge the interface.

The interface is a single source of truth read **two ways**, by context:

| interface form (`mod.req.shard`) | impl provides (`bump.shard`)      | a CONSUMER sees                    |
|----------------------------------|-----------------------------------|------------------------------------|
| `(sig fn bump (x) Int)`          | `(fn bump (x) Int …)` — the body  | opaque `(sig fn bump …)` (stuck)   |
| `(requirement bump_grows …)`     | `(fulfills bump_grows …)` — proof | `(axiom bump_grows …)` — granted   |
| `(type …)`                       | (uses it)                         | the type (transparent, for now)    |

That symmetry is the whole design: a module promises signatures + lemmas; the
impl fulfills them; a consumer reasons against the promises **without** the
bodies. It reuses `requirement` / `fulfills` (already in the language) plus a new
bodyless signature form `(sig fn …)`; the opacity falls out of a bodyless symbol
being stuck-in-proofs — the same mechanism the io contracts use at the world
boundary, turned inward.

`(sig fn NAME (params) RET)` (added to `reader.shard`) loads as a bodyless entry:
stuck in proofs like an `extern`, but distinct from one — the impl supplies a
real body (which shadows the sig when the module is checked/run), whereas an
extern has no body anywhere and the runtime dispatches it. That distinction is
why it's `sig fn`, not `extern`: it matters at run/link time, not at proof time.

## Files

```
bump/
  mod.req.shard   INTERFACE  — sig fn bump + requirement bump_grows
  bump.shard      IMPL       — fn bump (= x+3) + fulfills bump_grows
consumer.shard    TARGET consumer — (import "bump"); reasons about bump opaquely
views/            FLATTENED views that prove the SEMANTICS work TODAY:
  module_view.shard    the module-self-check, interface-first      → PASS
  consumer_view.shard  the consumer's-eye interface-only view       → PASS
  necessity.shard      same, but lemma removed                      → FAIL (intended)
```

## What already works (zero new proof machinery)

Run the three `views/` files through `check` today:

- `module_view` **passes** — `fulfills` discharges `requirement`, and the impl's
  `fn` body wins over the interface `sig fn` so `unfold bump` works.
- `consumer_view` **passes** — an opaque `sig fn bump` + the lemma as an `axiom`
  lets the consumer prove its goal through `bump_grows`, no body in sight.
- `necessity` **fails** — drop the lemma and the opaque `bump` is unreasonable
  (`ord` can't see the body). The lemma is load-bearing: opacity is real, not
  cosmetic. (This is the demo's `lia_rejects`/`module_gate_rejects` analogue.)

So the proof-time semantics are settled. The missing piece is purely the
**loader**.

## What the loader must do (the spec this demo pins down)

`(import "bump")` and `check bump` must produce the two views above from the one
set of files. Concretely, three things — each discovered by building this demo:

1. **Selective loading.** Checking a *consumer* loads bump's **interface only**
   (sig + lemmas), never `bump.shard`'s bodies. Checking the *module itself*
   loads interface + impl together.
2. **`requirement` is context-dependent.** For the module's own check it's an
   *obligation* (must be `fulfills`-ed). For a consumer it's a granted *axiom*
   (the module already proved it; the consumer doesn't re-run the proof).
   `consumer.shard` is broken today precisely because the current loader re-runs
   bump's `fulfills` in the consumer's run instead of granting the lemma.
3. **Interface loads FIRST.** The current dir-module rule puts `mod.req.shard`
   *last* (so old-style public *claims* could cite dir-mate privates). With
   bodyless interfaces that inverts: `requirement`s must precede the impl's
   `fulfills`, so the interface loads **first**. (`check bump` fails today with
   "no requirement … in scope" for exactly this reason.)

Per the slice-94 correction, this loader belongs in **shard** (`loader.shard`),
which also closes the directory-resolution self-hosting gap — it needs a new
directory-listing extern (the loader only has `read_file`).

## Decided

- **`(sig fn …)` for fn signatures** (not `extern`). Distinguishes "opaque for
  reasoning" from "no body anywhere / runtime-dispatched." Implemented in
  `reader.shard` (loads as a bodyless entry).

## Open forks (deliberately unresolved)

- **Transparent vs opaque types.** Types are transparent for now (consumers see
  the ctors). Abstract types (name only, ctors hidden) — perhaps a `(sig type …)`
  companion — is a later step.
