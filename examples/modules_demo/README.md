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

So the proof-time semantics are settled with zero new proof machinery.

## The selective loader (BUILT — self-hosted path)

The whole thing now works end-to-end through the self-hosted checker:

```
check run kernel/check.shard -- examples/modules_demo/consumer.shard   → consumer_grows PASS (bump opaque)
check run kernel/check.shard -- examples/modules_demo/bump/bump.shard  → bump_grows PASS (fulfilled)
```

`loader.shard`'s `resolve_closure` (the shard import resolver, per slice-94)
implements three things the demo pinned down:

1. **Bare-name imports resolve to the interface.** `(import "bump")` (no
   `.shard`) → `bump/mod.req.shard`, tagged **granted**; `(import "x.shard")` →
   that file, ungranted. So a consumer loads bump's **interface only** — never
   `bump.shard`'s bodies. (No directory-listing was needed: the interface sits
   at a deterministic path.)
2. **`requirement` is context-dependent** (`collect_decls`, reader.shard). In a
   granted source it is admitted as an **axiom** (the module already proved it;
   the consumer doesn't re-run the proof). In an ungranted source — the module's
   own check — it stays an **obligation** that `fulfills` must discharge.
3. **Interface loads FIRST**, without a special rule: the impl imports its own
   interface (`(import "mod.req.shard")`), making the interface a dependency, so
   deps-first ordering puts the requirement before the impl's `fulfills`.

Threaded as a per-source `granted` tag from `resolve_closure` → `check_production_src`
→ `run_srcs` → `parse_decls` → `collect_decls`. **Bonus:** because bare-name
imports now resolve (the shard loader used to skip them), this also closed the
self-hosting directory-resolution gap — e.g. `std/list.shard` went 6/7 → 28/28
self-hosted.

The native `check` path keeps loading whole modules (no selective loading) — it
is transitional per slice-94; it still passes the demo, just without enforcing
opacity.

## Decided

- **`(sig fn …)` for fn signatures** (not `extern`). Distinguishes "opaque for
  reasoning" from "no body anywhere / runtime-dispatched." Implemented in
  `reader.shard` (loads as a bodyless entry).

## Open forks (deliberately unresolved)

- **Transparent vs opaque types.** Types are transparent for now (consumers see
  the ctors). Abstract types (name only, ctors hidden) — perhaps a `(sig type …)`
  companion — is a later step.
