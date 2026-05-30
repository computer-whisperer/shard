# Snake Game — a convention experiment in requirement isolation

This directory is a deliberate experiment, not a finished pattern. The goal
is to find an *ergonomic* way to isolate **requirements/specs** from
**implementations/proofs** — by running a convention by hand first and only
installing language features once the convention visibly earns them. Expect
this document, and the layout it describes, to change as we build. Nothing
here is stone.

## What we are reacting against

The traceability discipline of safety standards (ISO 26262 and friends) has
the right *skeleton* — requirements decompose, evidence links to claims — but
in practice it rots into a paper machine. Two specific failure modes we will
not reproduce:

1. **Reverse-engineered specs.** A spec written by walking the code and
   describing what each function already does asserts nothing. It is vacuous
   by construction. Here the spec **stands alone and comes first**: it is
   frozen at project start and the implementation is built *to* it, so it
   cannot be back-fitted to whatever the code happens to do.
2. **Per-function granularity.** Hundreds of assertions, each that one
   function matches a sentence in a Word doc, carry no information — just
   ceremony. Here the contract lives at the **module surface**, where a small
   number of lemmas actually constrain behaviour, not at every function.

## The intended pattern

**A module is a directory (or file) that advertises an upper surface.** The
surface is its public types, its public function signatures, and — the part
that makes this a proof system and not just a build tool — **its lemmas**.
The contract is *behavioural*, not merely type-shaped.

Three consequences we are trying to exploit:

- **Churn underneath the surface is cheap.** Dependents rely only on the
  surface. The implementation below it may be rewritten freely as long as the
  surface lemmas still hold.
- **Traceability falls out of the import graph.** If dependents cite *only*
  surface lemmas, then "what depends on this requirement" is just "who imports
  this surface." We get the traceability artifact for free, at the module
  boundary where it carries information — with none of the subsystem-walking.
- **The surface *is* the requirement.** Fulfillment is proving the surface
  lemmas. There is no separate requirements bureaucracy parallel to the code.

**Fulfillment reuses the calc pattern.** The implementation discharges the
contract by proving it: a reference-spec anchor (`step = spec_step`, exactly
as `calc_app_spec.sexp` proves `step = step_spec` from `run = spec_run`) plus
any named invariant that is awkward to phrase as an equivalence (e.g. "score
equals food eaten", "the snake never occupies a wall cell"). Snake, unlike the
memoryless calculator, carries state across events — so at least one real
invariant proof is part of the point.

## File / naming convention (provisional)

- `*.req.sexp` — the **contract**: public types, public signatures, the
  promised surface lemmas, and informal requirements as prose right alongside
  the formal ones. **Frozen by observation, not by mechanism** — integrity is
  just git. The history of a `.req.sexp` file should read "created, then
  stable"; an edit without a recorded reason is the smell.
- plain `*.sexp` — the **fulfillment**: implementation functions and the
  proofs that discharge the contract. Free to churn.
- `*.app.sexp` — the `(app …)` entrypoint driven by `check app`.

One module may be fulfilled by **more than one implementation** (a naive one,
an efficient one): each implementation cites the same contract and carries its
own proof. The contract never names its fulfillers — that is what keeps it
implementation-agnostic and lets several coexist.

## Deliberately NOT built yet (earmarked, to be pulled out by friction)

These are the language/tooling features we expect to want, but will only add
when the convention concretely hurts — so their shape is decided by practice,
not guessed in advance:

- A machine-checked **`requirement` / `fulfills` edge** (state a surface lemma
  in the contract, discharge it with a named proof elsewhere, have the tool
  verify the goals match). Today the link is by convention and the eye.
- A **`check project`** command that walks the tree and reports met / unmet
  surface lemmas — the traceability report in concrete form.
- A **surface lint**: flag any dependent that cites a non-surface (internal)
  name.
- A **trust ledger**: surface the axioms / externs / Action-interpreter
  boundaries each fulfillment rests on, so "meets the contract modulo 3
  axioms" is visibly weaker than "meets it outright" (see
  `../../docs/BOUNDARIES.md`).
- The **informal → formal refinement** as an explicit, reviewable link rather
  than adjacent prose. (This edge is human-asserted and never machine-closed —
  the machine checks the formal leaves; the argument that they cover the
  informal intent stays a human artifact.)

## Status

Empty scaffold. Next: lay down the first `.req.sexp` contract and a fulfillment
against it, and let the rough edges tell us which earmarked feature to build
first.
