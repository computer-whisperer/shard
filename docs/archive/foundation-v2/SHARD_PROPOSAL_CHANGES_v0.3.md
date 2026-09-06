# Shard proposal revision v0.3 — review map

**Date:** September 5, 2026.  
**Status:** Draft documentation update. No repository implementation, proof migration, benchmark result, or completed metatheoretic proof is claimed.

## Documents

- [Foundation proposal v0.3](SHARD_FOUNDATION_PROPOSAL_v0.3.md) — canonical engine architecture, the proposed dependent foundation, native contextual holes, evidence, and acceptance gates.
- [Bootstrap addendum v0.3](SHARD_BOOTSTRAP_ADDENDUM_v0.3.md) — direct Rust execution of those services, embedding interfaces, conformance, and trust accounting.

These replace the corresponding v0.2 drafts. Earlier versions remain unchanged in the conversation. Existing decisions D01–D08 and B06–B12 retain their identifiers; this revision adds D09–D13 and B13–B16.

## Central change

Treat a partial program or proof as a native contextual object of the shared engine, rather than a special function call understood only by a search library.

Schematic form:

```text
?h : [u : Nat ⊢ Nat]
left  := fun x => ?h[u ↦ x]
right := fun y => ?h[u ↦ y]
```

One assignment `h := u + 1` supplies one template to both occurrences with their explicit scope mappings. Two separate hole IDs remain separate choices even when their types and candidate grammars match. A hole declared without a local parameter cannot capture that parameter merely because it appears at an occurrence site.

The engine can validate an open construction without knowing its final filling. That does not prove a filling exists, and it does not make a proof hole evidence of its expected proposition. Final admission validates completed assignments and rechecks the fully instantiated evidence closure. A complete theorem about *quoted open-program data* is legitimate; an unresolved native hole in that theorem's proof is not.

## What is substantively different

**Embedding is a primary deliverable.** The public engine includes construction, inspection, prepared execution, open validation, closed checking, and realization. The logical authority stays small within that library. E0 becomes an internal execution/lowering view tied to shared declarations, not another public programming world.

**`meta/` becomes a migration driver.** Prepared invocation, sketch/search, authentic theorem capture, anti-unification, relation-aware rewriting, and structured proof construction become early pathfinders. The rewrite may replace their representations; it must preserve the workflows they enable.

**Contextual holes are specified beyond syntax.** Foundation Sections 4.8 and 13 cover telescopes, occurrence substitutions, dependencies through types/universes, assignment validation, subholes, blocked constraints, persistent branches, and closure. The substitution/closure argument is an explicit foundational obligation, not something assumed from a green test suite.

**Partial proof reuse becomes an architectural objective.** A fully proved statement about every valid filling of a template can justify many later transformations. The statement keeps its guards, observation relation, and context. Speculative partial checks remain distinct from such completed proofs.

**Search claims are separated.** Choosing a candidate, preserving a candidate set, proving a region empty, retaining equivalent representatives, and proving cost optimality are different operations. Exact rank/count remains a promise of selected grammar/constraint libraries—not a consequence of native holes. Timeout or failure of a unifier does not establish unsatisfiability.

**The Rust decision is preserved and propagated.** The main foundation draft now agrees with the bootstrap addendum: a reviewed, uncertified Rust executor may run the Shard checker for complete certification. Rust runs the engine's contextual logic; it does not acquire an independent proof or hole-solving authority.

**Runtime metaprogramming is explicitly supported.** Applications may link the engine and `meta/` to compile or inspect code after deployment. Other artifacts need not retain them. Bulk data uses explicit host/resource interfaces rather than necessarily becoming large expression literals.

## Recommended review route

Start with foundation Section 3 for the public engine boundary. Read Section 4.8 and all of Section 13 together for the native-hole proposal. Section 10.4 maps the current `meta/` components to the replacement. Foundation gates G7–G9 test contextual holes, embedding, and search fidelity. Bootstrap decisions B13–B16 and gates G7–G10 cover execution and isolation of those services.

## First implementation slice proposed

Implement one canonical contextual-hole representation, typed substitutions, a persistent workspace, basic open validation, and a closed-admission recheck. Exercise a first-order exact search case and a dependent hole case immediately. Add a public retained-invocation interface alongside them, running directly on Rust.

Do not require complete higher-order unification, arbitrary multilevel modal reflection, globally exact constrained counting, or a new kernel algorithm for each search domain. The first deliverable is a small native open-term discipline that the libraries can reliably build on.

## Questions worth challenging

Is the shared term/workspace boundary small enough to own and verify? Does the proposed assignment discipline accommodate the actual search correlations without forcing context hacks? Can partial results be reused without importing the whole solver into the trusted core? Does the embedding path avoid both interpreter nesting and repeated preparation? Are the first exact-search claims small enough to validate against independent ground truth?

These remain design questions. The revision establishes concrete proposed answers and falsification gates, not a claim that the design is already sound or faster.
