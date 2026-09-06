# Shard foundation redesign

## An embeddable mathematical engine with native contextual holes and certified realizations

**Status:** DRAFT v0.3 — integrates the agreed embedding/metaprogramming direction and proposes native contextual holes. Detailed rules, APIs, and acceptance gates remain proposals, not implemented or formally validated.  
**Date:** September 5, 2026.  
**Prepared for:** Christian Balcom and review with Claude Fable.  
**Evidence baselines:** foundational source review at `e682c81233beeacb6d1c2f296727f5a74c098e33`; embedding and `meta/` review at `82c00e9b4e2e35eda689c632d3240a81be2429bd`; `meta/sketch` and prepared invocation rechecked at `5abc60074f260f882a92a563f5c9d8fcd891199a`. References identify their own pins.  
**Historical comparison baseline from v0.1:** Lean 4.33.1, source `819816b2e0a3bf405af45ae5c7af2491d8f5bee6`. This is not a build dependency, normative specification, or mandatory compatibility target. Its release metadata is carried forward, not independently reverified for this revision.  
**Scope:** Foundational architecture, boundaries, acceptance criteria, and migration. Not an implemented kernel, a complete formal calculus specification, or a soundness audit. All example interfaces below are schematic rather than tested Shard syntax. This revision changes the proposal documents, not the repository.  
**Companion:** [Bootstrap and execution architecture v0.3](SHARD_BOOTSTRAP_ADDENDUM_v0.3.md). This document owns logical and public-engine contracts; the companion owns Rust execution, backend conformance, and bootstrap provenance. The two supersede their v0.2 predecessors.

---

## 0. Recommendation

Replace Shard's specialized equational proof authority within a **canonical embeddable language engine**, not with a stand-alone proof assistant around which execution must be rebuilt. The engine provides shared declarations, program inspection/construction, native open terms, prepared execution, checking, and certified realization. Its logical authority is a general, explicitly specified dependent-type-theoretic kernel implemented in Shard from the first prototype. Shard owns the rules and integration boundaries. Lean and other established systems are mathematical references, not required executables or authorities for a Shard verdict.

The long-term mathematical-coverage target is that mathematics supported by Lean's stated foundation and declared assumptions can be faithfully expressed and justified in Shard. Unmodified Lean source, identical proof terms, exact kernel behavior, and automatic Mathlib import are separate goals and are not requirements of this redesign.

**One foundation; different representations and execution disciplines.** Mathematical definitions, specifications, algorithm correctness, compiler transformations, machine models, and artifact claims should ultimately produce judgments in the same foundation. They need not all be executable, share one IR, use one proof strategy, or depend on one axiom package.

The critical separation is:

> Partial-construction validity, mathematical validity, executable realization, and artifact acceptance are different judgments over a shared semantic environment. Connect them by evidence; do not conflate them.

Native holes are contextual unknowns in the engine's open syntax and judgments, not forged function calls and not inhabitants supplied by an axiom. Search can inspect, share, constrain, and propose assignments to them. Final admission checks the fully instantiated evidence closure. Solvers and search policy remain outside the authority for closed theorems.

Run this Shard-written engine directly on an expanded, maintained Rust execution backend. A reviewed, uncertified backend is permitted for complete acceptance-grade certification under a stated execution TCB; its formal verification is not a prerequisite. Preserve the reference interpreter as a selected validation path, not a mandatory layer around every engine call.

This proposal favors a Shard-native, Lean-informed dependent foundation over a new HOL-shaped extension because a willingness to rewrite removes much of the migration advantage of preserving the current first-order proof shape. Indexed invariants, generic structures with laws, typed program representations, and mathematical interoperability are sufficiently central to Shard's future that dependent abstraction is worth paying for once.

HOL remains a credible alternative, not an inadequate logic. The proposed preference is an engineering judgment about this product, not a claim that only dependent type theory can express these constructions.

Do **not** migrate the entire proof corpus merely because this document is attractive. First validate the proposed foundation against a small set of representative high-level and machine-level pathfinders, including checking cost. Existing proof text is disposable; theorem meaning and trust boundaries are not.

### 0.1 What changes in v0.3

| Decision | New requirement | Main location |
|---|---|---|
| D09 | A small logical authority inside a complete, composable embedding library | Section 3 |
| D10 | One declaration/identity system; execution IRs are linked views, not separate public semantic worlds | Sections 3 and 7 |
| D11 | Native contextual metavariables, open judgments, checked substitutions, and an explicit closure gate | Sections 4.8 and 13 |
| D12 | Reusable proofs about partial constructions and search transformations, without claiming all search is decidable or exact | Sections 10.4 and 13 |
| D13 | Explicit runtime linking of the engine and `meta/` is supported; generated artifacts need not retain them | Section 9.4 |

`meta/` is a design consumer and migration pathfinder, not a late port. Prepared invocation, contextual candidate construction, theorem capture, relation-aware rewriting, and in-process evidence construction must survive the rewrite. This revision also reconciles the main proposal with the maintained-Rust/acceptance policy already adopted in the bootstrap discussion.

The detailed contextual rules below are a proposed conservative open-term discipline around the closed foundation. The required substitution/closure arguments remain work for G0/G7; citing related type theories does not prove this particular combination sound.

### 0.2 Historical changes from v0.1 to v0.2

Christian's explicit design input is that owning the implementation and integration boundaries has repeatedly been more valuable than adopting an almost-suitable package. This proposal accepts that as a product constraint; it does not assert an independently measured industry-wide productivity result.

- **Withdrawn:** starting with Lean as the prototype's proof authority.
- **Withdrawn:** literal compatibility with one pinned Lean kernel as the governing architecture.
- **Required:** the first foundational checker is a Shard program, initially executable on the existing bootstrap path with that path's trust assumptions stated.
- **Retained:** a general dependent foundation, a single theorem authority, separate executable realizations, explicit evidence and assumptions, and reference to established metatheory.
- **Clarified:** external mathematics is a coverage target; proof translation is useful infrastructure; upstream implementation fidelity is optional.
- **Added:** a versioned Shard rule specification, semantic-change review, and a bounded experiment on explicit conversion evidence.

The correction is not merely to rewrite Lean's checker in another language. It is to make the entire foundation a Shard-owned subsystem whose design is selected for Shard's workloads.

---

## 1. Product objectives and invariants

### 1.1 Objectives

Shard should make it inexpensive to propose implementations, specifications, proofs, optimizations, and reusable abstractions, while making it difficult to confuse an accepted artifact with one justified under a different statement or assumption set.

The system should support ordinary software without demanding application-level proofs everywhere; increasingly rich evidence should be attachable without rewriting the program into a special proof-facing dialect. It should also support advanced mathematics when that mathematics enables better algorithms, stronger specifications, numerical error bounds, or resource guarantees.

Build-time resources are available, but not free. Optimize for the combined costs of proposal, checking, repair, and deployment, especially over repeated edits.

### 1.2 Preserve

- Machine and operating-environment models remain libraries, not kernel primitives.
- Implementations may be replaced by independently certified realizations of the same specification.
- Search, elaboration, compilation, linking, and optimization have no authority to assert unproved facts.
- Authoritative artifacts retain explicit assumptions and a connection to the exact deployed bytes.
- Programs remain inspectable data through the public engine, including in applications that explicitly link metaprogramming capabilities.
- `meta/` uses the same declaration, term, scope, and evidence services as the compiler and checker; no parallel theorem namespace or per-tool hole encoding.
- Native contextual holes support partial programs, types, and proofs without granting missing witnesses.
- Prepared invocation and retained checking contexts are public library operations, not access to interpreter internals.
- Hardware constraints and exceptional behavior are explicit, never silently identified with ideal mathematics.
- The kernel is usable as a component without importing the complete build workflow.
- The kernel is implemented in Shard; normal proof acceptance does not require a Lean executable, Lean service, or foreign library verdict.
- Shard owns its rule specification and may make justified departures from other systems without treating upstream compatibility as a veto.

### 1.3 Deliberately relax

The first-order nature of the current proof language, the conditional-equation shape of all claims, universal executability of mathematical definitions, source-format compatibility, and the current placement of specialized proof rules are not invariants.

Likewise, the same logical syntax need not be the compiler's main optimization IR. A mathematical function is not necessarily a runtime closure, and a theorem about a process need not imply that the process has a predetermined finite lifetime.

### 1.4 Do not promise

This is not a promise that all future mathematics fits one axiom-free universe, that imported Lean libraries work unchanged immediately, that every valid program is automatically verified against every useful property, or that richer type theory makes proof search cheap.

The proposal also does not claim that self-hosting proves the consistency of the foundation.

---

## 2. The foundational choice

### D01 — Own the foundation; use established mathematics as its starting point

**Recommendation:** Design and implement a Shard-native dependent foundation. Begin from a coherent, well-understood family of rules rather than inventing every logical principle anew. Lean is the leading reference because its dependent abstraction and mathematical scope fit the intended uses. Its implementation choices are not automatically Shard's requirements.

The normative artifact is a versioned Shard declarative specification: typing judgments, conversion, universe constraints, declaration admission, and assumption policy. A small initial fragment is legitimate. Later additions and deliberate divergences must identify which rules change and what justifies the combined system.

A comparison with a specific Lean release must name that release and the supported translation. This is useful for reproducible evidence, not for handing upstream control of Shard's design. The historical release pin in this document serves only that comparison role. [L0, L1]

**Borrowing established logic and owning its implementation are compatible choices.** Metatheoretic results apply only when their hypotheses match Shard's actual rules; naming a source system does not transfer a soundness proof. Conversely, rebuilding implementation machinery does not by itself require inventing a new axiom or weakening the mathematical target.

### 2.1 Alternatives considered

| Alternative | Principal attraction | Principal cost for Shard | Disposition |
|---|---|---|---|
| Extend the present equational system | Least conceptual migration for current proofs | Repeatedly adding binding, abstraction, relational definitions, and transport around a narrow statement representation | Reject as the long-term authority |
| HOL-style foundation | Small, mature logical basis; excellent machine-verification pedigree | More of the desired indexed programming vocabulary would be represented through predicates, packages, and tooling | Retain as the strongest fallback |
| Shard-native, Lean-informed dependent foundation | Common representation for mathematical abstraction and indexed specifications; full ownership of integration and execution | Conversion, inductive admission, and universe handling require a precise Shard specification | Recommend |
| Literal Lean-compatible reimplementation or Lean-hosted prototype | Direct reference behavior and potentially easier proof replay | Constrains design around external representations and behavior; postpones the intended dogfooding | Not the governing architecture |
| Rocq/PCUIC-compatible foundation | Extensive metatheory and verified-checker/erasure work | Different engineering and interoperability choices; not interchangeable with Lean's rules | Credible alternative if verification infrastructure wins the comparison |
| A universal logical framework hosting several equal-status foundations | Broad proof exchange and experimentation | Moves complexity into encodings, trusted rule admission, and theorem transfer | Do not make this the first product architecture |

Candle provides strong evidence for the HOL alternative. MetaRocq provides relevant evidence for a verified dependent checker and erasure path. Neither makes Shard's new implementation automatically verified. MetaRocq's published safe-checker description explicitly names a normalization assumption; its exact proof obligations should be read rather than summarized as unconditional self-verification. [L8, L9]

### 2.2 Why dependent types are worth considering here

The strongest reason is not that agents can tolerate difficult syntax. It is that one abstraction mechanism can carry many recurring obligations:

- A buffer whose length is part of its interface.
- A value paired with evidence that it satisfies an invariant.
- A mathematical structure carrying operations and their laws.
- A typed expression indexed by its context and result type.
- A transformation indexed by the contract it preserves.

These could be represented without dependent types. The proposal is that using the same general mechanism can reduce the number of bespoke subsystems and transport conventions.

The counter-risk is real: excessive indexing creates casts, conversion work, and fragile elaboration. Use dependent types where they improve composition, not as a mandate to make every internal data structure intrinsically typed.


### 2.3 Vertical integration is a requirement, not a final cleanup task

The first checker is written in Shard. The current first-order language can represent the richer logical syntax as data and implement its checking algorithms. Begin with explicit terms and a small declaration language; do not make a complete elaborator or external importer a prerequisite.

Run the checker and other engine services directly on an expanded Rust execution backend, not through an avoidable evaluator tower. Rust executes Shard code; it does not independently decide the new logic. A reviewed checker/runtime snapshot can issue complete accepted results under its stated TCB before either implementation is formally verified. Do not require the old logic to establish unrestricted semantic soundness of a stronger successor before the prototype is useful. The companion bootstrap document specifies this execution contract.

Ownership allows the term arena, incremental environment, diagnostics, certificate layout, and checking strategy to be co-designed with Shard's proof-generation and compilation workloads. Dogfooding should expose deficiencies in those interfaces early, rather than defer them until a foreign-hosted proof development is ported.

External checkers can help test overlapping fragments or translated results. They are optional independent instruments outside the normal acceptance path, not the initial authority and not required services at build time.

### 2.4 Freedom to change does not mean ambiguous theorem identity

Distinguish three kinds of change:

1. **Implementation changes:** indexing, sharing, evaluation order within justified bounds, caching, and diagnostics. Preserve the declared judgments and test/prove the revised implementation against them.
2. **Evidence and representation changes:** new certificate forms, explicit instantiations, conversion witnesses, and serialization. Supply a validation or interpretation into the declared judgments; do not silently promote a representation shortcut into a new rule.
3. **Logical changes:** conversion laws, universe rules, elimination restrictions, inductive admission, or additional principles. Update the versioned foundation specification and justify the change as a derived rule, conservative extension, supported interpretation, or explicit strengthening with stated assumptions. None of those statuses is established merely by a green regression suite.

All three kinds of change are allowed. The distinction is about what evidence and revalidation they need, not permission to innovate. Each accepted artifact binds the foundation version and assumption policy used to justify it. A cosmetic version label without the associated rules and evidence is insufficient.

This keeps freedom to reshape the system while preventing an old theorem receipt from silently acquiring a different meaning.

---

## 3. Architecture: a small authority inside an embeddable engine

### D09 — The public product is the language engine, not just `check_proof`

The word *kernel* currently denotes both the trusted inference mechanisms and a reusable engine for loading, manipulating, and executing Shard. Keep the distinction explicit without forcing a directory rename. The public engine may be broad and selectively importable while the authority for accepting a theorem stays small.

```text
Host application / CLI / Shard metaprogram
                    |
          canonical engine library
                    |
    shared declarations, identities, term structure
          /            |             \
 open workspace    closed checker K   prepared execution
     |                  |             / realization
     +--------- evidence relations ---------+
                    |
          meta/ reusable mechanisms
     sketch, rewrite, search, antiunify, proofgen,
            shape, proof, plan, invoke
                    |
      application policies and target libraries
```

The last two rows are clients of the services above; they are not dependencies that K must import. Mathematical and executable-semantics libraries are checked against K. Neither loading the whole mathematics library nor loading every compiler target is a prerequisite for structural program inspection.

### D10 — One semantic environment, several validated views

A resolved declaration has one logical identity and an explicit environment version. Inspection, execution, theorem lookup, search, and compilation refer to that identity. Implementations, internal execution IRs, and machine artifacts have separate identities linked by explicit realization records. This is not a demand for one physical AST or one heap layout.

Separate the immutable admitted environment `E` from an editable workspace `W` based on `E`. A workspace may contain drafts, contextual holes, assignments, and pending constraints. It is not a second namespace of proved facts. A branch can shadow a draft by creating a new draft identity, never mutate an admitted declaration in place.

Public services should support the following workflow without temporary files or subprocesses:

| Service | Input and result | Authority boundary |
|---|---|---|
| Inspect/build | Declaration views, contexts, syntax constructors | Building a term does not prove it well-typed |
| Elaborate/refine | Source or structured candidates into `W` and obligations | May search; residual work stays explicit |
| Validate open | Contextual typing/constraint evidence | Establishes only the stated partial judgment |
| Admit | Fully instantiated declarations and evidence | K checks the closed result and assumption closure |
| Prepare/invoke | Entry identity plus runtime policy and data arguments | Execution is not automatically a logical conversion proof |
| Transform/realize | Candidate replacement and evidence for a named relation | A generated proposal is checked before acceptance |
| Query evidence | Resolved proposition, dependencies, assumptions, evidence status | Never reconstruct trust from a theorem's display name |

### 3.1 Logical authority and open-term support

K owns the closed typing, conversion, universe, and admission rules. The engine also natively represents contextual metavariables and validates open judgments using a specified extension of those rules. Native support means shared syntax, binding, typed occurrence substitution, and explicit partial results—not a tactic-local encoding of holes.

The initial final-admission path expands assignments and rechecks under the closed rules. Thus a solver bug or a mistaken claim of partial progress cannot mint a theorem. If later incremental receipts replace that recheck, their validation and substitution/closure argument join the acceptance-critical implementation.

Unification strategies, candidate enumeration, rewrite orientation, cost functions, and constraint-solving policy remain in elaboration and `meta/`. A tool may propose a unifier; the engine validates its typed substitution before relying on it.

### 3.2 Prepared execution and inspection are coequal services

A public prepared-execution context should retain expensive module preparation once and bind it to the exact declaration/environment and execution profile. Its implementation can use an interpreter, bytecode, or compiled code; clients should not depend on `FnTrie`, effect-table layout, or evaluator-private tables.

Current `meta/invoke/prepared.shard` exists because repeated invocation otherwise rebuilds preparation, but its table types create an import-cycle problem for the main public interface. The replacement must make the fast retained path a supported surface, not an opt-in dependency on machine internals. [R6]

Program structure remains inspectable through views. Execution indexes and mutable runtime storage remain private. No handle can silently refer to a declaration from a different environment or stale implementation snapshot.

### 3.3 The authoring layer and `meta/`

Source parsing, inference, instance search, tactics, and specializations produce canonical objects through engine APIs. `meta/proofgen` should construct evidence directly; printing and parsing it is an edge operation, not an obligatory in-process round trip. The existing distinction between reusable mechanisms and caller policy is retained. [R7]

The CLI is a client of these libraries. The engine's data-level services must not require filesystem access, process-global proof state, or the full command-line driver. Host services enter through explicit adapters.

### 3.4 Execution and artifact acceptance

Ordinary Shard declarations in the supported executable fragment have a prepared execution route and an automatic baseline lowering contract. Arbitrary mathematical definitions need not execute. E0 is an internal execution/lowering representation linked to the common declaration system, not a competing public language with a separate theorem store.

Artifact acceptance additionally binds the intended contract, actual bytes, target, environment assumptions, and resource behavior. A valid theorem about a different program is not artifact acceptance. This binding is still acceptance-critical even when implemented outside the small inference kernel.

### 3.5 Backend freedom without semantic fragmentation

The same public services are implemented in Shard and run directly through Rust now and compiled Shard later. Rust may accelerate generic execution and maintain prepared runtime values. A separate Rust implementation of theorem admission or hole-solving authority is not implied. Performance paths must report their execution profile and cannot silently fall back to an interpreter tower.

---

## 4. Candidate logical kernel

### D02 — Specify a coherent Shard rule package

Choose and specify one coherent Shard rule package, starting from an established dependent theory. The candidate below deliberately resembles Lean in several places, but each choice is owned by Shard and remains open to justified revision. Do not combine features from several systems and assume that their separate soundness arguments automatically justify the combination.

This section fixes candidate defaults, not a complete calculus. Gate G0 must produce the precise Shard rule inventory and identify its relationship to an established metatheoretic account. The schematic descriptions here are not sufficient to implement a sound checker from scratch.

### 4.1 Universes

Candidate baseline: universe-polymorphic constants, a predicative hierarchy of data universes, and an impredicative proposition sort. Start with noncumulative universes, as in Lean: `Type u` is not silently a subtype of `Type (u+1)`. Explicit lifting handles cases that need it. This is a concrete starting choice, not an upstream-compatibility obligation.

Universe parameters are explicit in checked declarations and constant applications. Universe inference is an elaborator task; validation of the resulting levels is K's responsibility. Open workspaces may contain explicitly declared universe metavariables and level constraints. Final admission rejects `Type : Type`, unresolved universe metavariables, and invalid level assignments; it may retain properly bound universe parameters. Cumulativity could be reconsidered with a coherent rule specification and cost/translation analysis; it is not excluded as unsound. [L2]

### 4.2 Dependent functions and explicit terms

The essential term vocabulary includes bound variables, typed lambda abstraction, application, dependent function types, let bindings, universe levels, and references to admitted constants. Inductive constructors and recursors are declarations available through those references.

A schematic carrier is:

```text
Level = zero | parameter | successor | max | imax

Term = bound_variable
     | constant(DeclarationId, universe_arguments)
     | sort(Level)
     | pi(domain, codomain)
     | lambda(domain, body)
     | apply(function, argument)
     | let(type, value, body)
     | specified literal / projection encodings
     | meta(HoleId, ContextSubstitution)   -- open-workspace form only
```

This is not a final interchange grammar. Literal and projection encodings must have an exact expansion or fixed reference semantics; they are not a permission for plugins to add term-level magic.

The closed authority is approximately `Environment; Context ⊢ term : type`. Proofs of propositions are terms of the corresponding proposition types. A proof-producing tool can choose any supported construction. The shared open syntax adds contextual metavariables, with a distinct judgment and result type (Sections 4.8 and 13). A meta node is not a logical constant and cannot survive unresolved into an admitted declaration. [L1]

### 4.3 Propositions and proof irrelevance

Candidate baseline: a proof-irrelevant proposition sort with precisely specified typing and conversion behavior. Universal quantification and implication use dependent function types; conjunction, disjunction, existential quantification, equality, and related forms use ordinary logical definitions and inductives.

Distinguish an existential proposition from executable witness data:

```text
Exists (y : Y), R x y       -- a proposition
{ y : Y // R x y }          -- a subtype: witness data paired with a proof
```

A constructive implementation may return the second. The first alone does not authorize extracting an arbitrary witness into runtime code.

Elimination from propositions into data must follow Shard's exact, justified restrictions. Lean's treatment is the starting reference, not an informal specification by example. Do not implement the misleading rule that all propositions can eliminate into data, or the equally misleading rule that none can: equality transport and designated subsingleton cases require careful treatment. [L3]

### 4.4 Inductive families and admission

Support strictly positive inductive families with checked constructor types, universe conditions, and recursors. Ordinary datatypes, indexed invariants, typing judgments, and finite derivation relations use this common mechanism.

Mutual and nested declarations should initially be supported only through a precisely specified Shard admission fragment. More elaborate front-end declarations may elaborate into that subset. Matching another checker’s generated recursor representation is not a requirement.

K must validate recursor types and computation rules. It cannot trust the elaborator to provide arbitrary recursors, because those rules establish new reasoning principles.

Empty inductive types are legitimate. They must not be confused with opaque inhabited types. Generic code must request an inhabitant when it needs one, rather than relying on a global convention that every declared type is inhabited. [L4]

### 4.5 Recursion

The logical core does not gain an unrestricted recursive-definition axiom. Structural recursion elaborates to admitted recursors. Well-founded recursion is built through accessibility and reusable well-foundedness results. Integer measures become one convenient instance rather than the only permanent admission mechanism.

Surface recursion tooling proposes the elaborated definition and any needed evidence. A declaration is not available for circular use while its own admission obligations are being established.

This retains Shard's important separation between finding a termination argument and validating it, but moves more of the argument into ordinary mathematics. [R4, L5]

### 4.6 Quotients and extensionality

Include a justified quotient mechanism in the target design rather than postpone the question until libraries have worked around its absence. A Lean-style construction is the baseline candidate. Its realization as primitive machinery, a conservative construction, or a translated library interface is Shard's decision; each route must provide the needed laws and account for computation behavior.

For the Lean-style candidate, distinguish the quotient formation/lifting machinery from the additional quotient-soundness principle, and record use of that principle in the assumption closure. Function extensionality is then available through its standard derivation; propositional extensionality is an explicit named assumption, not an automatic conversion shortcut. A different realization must establish the corresponding laws rather than merely reuse their names. [L6, L7]

No part of this permits program representations with equal denotations to become syntactically interchangeable. See Section 8.

### 4.7 Classical reasoning and additional assumptions

The default mathematical library profile may permit the standard named classical principles needed by its chosen library base, including choice and propositional extensionality. A restricted constructive profile can disallow dependencies on them. This is a policy over dependency closures, not a second checker.

Axiom names alone are not enough: record their exact types, identities, and transitive dependencies. A package of abstract operations and laws should normally be a parameterized structure, not a list of globally installed axioms.

Choice used to construct runtime-relevant data has no automatic executable realization. Classical reasoning used to prove a property of an independently executable implementation can be permitted without adding runtime machinery. [L7]

The proof-irrelevant intensional foundation is not an attempt to support every alternative foundation natively. In particular, do not casually append incompatible higher-equality or univalence principles. Such systems can be modeled as objects or handled through an explicit, separately reviewed foundational extension.

### 4.8 Native contextual holes: a disciplined extension of the judgment

Declare a hole in a well-formed meta-context `Δ` by:

```text
?h : [Ψ ⊢ A]
```

`Ψ` is the local telescope the solution may use; `A` is its expected type in that telescope. Both may refer to previously declared metavariables according to an explicit acyclic dependency discipline. The hole is an obligation to supply a term, not a declaration that A has an inhabitant.

An occurrence is `?h[σ]`, where `σ` supplies appropriately typed terms for the variables of `Ψ` in the occurrence context `Γ`. The native open rule is schematically:

```text
Δ(h) = [Ψ ⊢ A]       E; Δ; Γ ⊢ σ : Ψ
------------------------------------
        E; Δ; Γ ⊢open ?h[σ] : A[σ]
```

Here `Γ ⊢ σ : Ψ` means that the telescope entries of Ψ are instantiated by terms well-typed in Γ, with dependent entries checked after earlier substitutions. Assigning `h := t` requires checking `t : A` in Ψ, validating dependencies and occurs checks, and recording residual obligations rather than assuming them solved. A hole occurrence then instantiates to `t[σ]`.

These are schematic rules to specify and justify, not a complete calculus. Contextual type theory and explicit-substitution work provide relevant precedents for recording local dependencies and distinguishing ordinary variables from metavariables. They do not establish soundness of adding this discipline to Shard's proposed universe, proof-irrelevance, and inductive rules. [L13, L14]

The required adequacy property is that a validated open derivation, instantiated by a well-typed solution of all its obligations, yields the corresponding closed derivation. It does not assert that such a solution exists. Holes of an empty type are representable search obligations, never automatic witnesses.

The minimal kernel growth is this explicit open-term/context/substitution boundary. General constraint languages, higher-order search, and grammar enumeration do not become primitive logical rules. Section 13 defines their layering and closure conditions.

---

## 5. Conversion and checking cost

### D03 — Fix conversion; keep theorem search outside it

Specify Shard's conversion relation independently of a particular implementation algorithm. Record exactly which beta, delta, iota, zeta, eta, projection, quotient, and proof-irrelevance rules are admitted, with their typed premises. Lean is a reference for this analysis, not an acceptance oracle. Do not summarize the contract as unrestricted normalization of every term.

An upstream convenience may instead become an explicit propositional theorem or elaborator-generated transport when that provides better checking economics. Such a change is not automatically conservative, proof-preserving, or cheap: dependent terms, elimination, and translation obligations must be evaluated together. No specific conversion-rule departure is ratified by this paragraph.

In particular, do not add **equality reflection**: a proof of `a = b` is not permission to make all future type comparisons treat `a` and `b` as automatically convertible. Use explicit transport or a proof-producing rewriting tool.

Also prohibit arbitrary user-installed kernel rewrite equations, SMT calls during foundational conversion, and plugins that declare expressions equal because a heuristic succeeded. These are places where a seemingly convenient feature changes the trusted theory.

### 5.1 Explicit resource outcomes

Lean4Lean discusses nontermination of Lean-style normalization and separates a declarative account from the implemented checker. This is a warning against assuming that adopting familiar dependent features automatically provides global strong normalization or reproducing an implementation's behavior without analysis. For Shard's actual chosen package, termination and completeness are explicit proof obligations, not inherited promises. [L10]

Implement checking with explicit resource limits:

```text
check(environment, declaration, limits)
    -> Accepted(receipt)
     | Rejected(diagnostic)
     | Exhausted(resource, location)
```

`Exhausted` is not evidence of invalidity and never counts as acceptance. A caller can provide more resources or propose a different proof.

This makes the checker implementation executable as a total, fuel-bounded function where desired. It does not impose that fuel as the lifetime of applications the logic describes.

Proof complexity is not eliminated by better agents. Large generated proofs require sharing, small intermediate lemmas, opaque theorem boundaries, and specialized certificates.

### 5.2 Two equalities, two APIs

Expose computational conversion and propositional equality as distinct operations. Agent-facing tools may offer aggressive, budgeted assistance for propositional goals, but K only checks the resulting evidence.

Do not advertise conversion as a canonical-normal-form service for arbitrary mathematics. Do not assume a checker-specific equality implementation is a clean, context-free equivalence relation simply because its API is named `isDefEq`. The declarative Shard rules and typed contexts must govern which uses are justified.

### 5.3 Opacity and ordinary proof reuse

Theorem bodies are checked when admitted, then normally opaque to clients. Concrete implementation bodies are unfolded only where their declared visibility and the selected proof require it.

An abstract module interface is best expressed using parameters and law-bearing records, with a checked concrete instance. Do not recreate the current problem in which co-loading an implementation changes what a consumer is allowed to prove about its opaque interface.

### 5.4 A Shard-specific experiment: explicit conversion evidence

Price an optional evidence form in which a proof producer supplies a typed conversion plan: selected unfoldings, beta/iota steps, shared intermediate terms, and explicit instantiations. K verifies the plan using only the same admitted conversion rules. Automatic bounded conversion remains available as a convenience.

This may improve predictability and expose reusable computation that otherwise occurs implicitly during type checking. It may also create excessive certificates. Measure both costs on compiler proofs before making it a public format. The aim is not to spell every reduction step as text; named results, sharing, and sound domain validators must remain available.

A propositional equality that is not conversion uses ordinary equality elimination/transport; the plan does not license equality reflection. A plugin's claim to have computed the answer is not conversion evidence. This experiment is a reason to own the checker/evidence boundary, not a promise that the design has already been validated.

### 5.5 Open conversion is not unification by assertion

The closed conversion API does not instantiate holes. The open API may establish a conversion from the existing assignments, report the unresolved constraints/blockers, establish a stable incompatibility for its supported fragment, or exhaust its budget. A solver separately proposes assignments in a workspace transaction.

`Blocked` is not `False`: solving a metavariable may make the comparison succeed. Failure of a higher-order unification heuristic is not evidence that no solution exists. A cached negative or branch-pruning conclusion must either be justified for every completion of the stated workspace region or remain a revisitable heuristic result.

No hidden metavariable assignment is allowed in final admission. Complete and validate the assignment graph first, then check the resulting declaration. A change to an assignment, its dependency context, or a relevant universe constraint invalidates affected speculative results.

---

## 6. Declarations, evidence, and the trusted boundary

### 6.1 Immutable checked environments

A checked environment contains admitted declaration identities, types, universe parameters, bodies where applicable, and dependency information. Admission yields a new environment version rather than mutating the meaning of an existing identifier.

New definitions, opaque constants with checked bodies, inductive blocks, theorems, and assumptions have different admission paths. There is no core `Admit` constructor returning success.

Native holes live in an engine-managed open workspace and can participate in explicitly validated partial judgments. They cannot create an authoritative theorem handle. A conditional theorem is an ordinary checked theorem with explicit parameters or hypotheses, not a successful proof with hidden obligations. Final admission instantiates assignments, validates every reachable obligation and declaration, and checks the hole-free result before adding anything atomically to the admitted environment. See Section 13 for the distinction between logical parameters and unsolved metas.

### 6.2 What the receipt means

A closed theorem receipt is a different type from an open-validation result or speculative search status. It should identify at least:

```text
logical foundation/version
checked environment root
declaration identity and type
proof/evidence identity
transitive assumption closure
logical dependencies and conversion dependencies
resource-accounting information
```

A receipt is not valid because a file says it is valid. Within a process, the checker owns the validated state. Across processes, the receiving checker rechecks the required evidence or applies a separately specified authenticated-validation policy. A signature records who checked something; it does not create a mathematical derivation. An in-process opaque receipt prevents ordinary API misuse, but serializing its type tag does not make the receipt authentic. Its exposed evidence status must retain whether a fact is proved, an explicit local hypothesis, or an allowed foundational/environment assumption; theorem-query clients should not need to reconstruct this distinction from source-driver conventions.

### 6.3 Provenance survives proof erasure

Proof irrelevance must not erase the evidence trail. Compute assumption dependencies from the checked declaration/evidence graph, including referenced types and any bodies used during conversion. It is acceptable to conservatively overapproximate the set; it is not acceptable to omit an assumption because the proof term later disappears from executable code.

Different valid proofs of the same proposition can have different assumption sets. The proposition's identity and a particular evidence package's identity are therefore different objects.

### 6.4 The full acceptance TCB

The artifact acceptance story includes more than inference rules: decoding, declaration identity, binding, primitive semantics, resource-limit handling, and the comparison between the reviewed requirement and the checked claim all matter.

A parser or elaborator bug may cause a proof of the wrong statement even while K behaves perfectly. Acceptance must compare against an independently fixed requirement identity and its semantic dependency closure, not merely the printed theorem name. Independent validation guidance for Lean makes the same broad distinction between checking a proof and protecting the intended statement against misleading submissions. [L11]

---

## 7. Executability and certified realization

### D04 — Separate execution from general mathematics without splitting the public system

Keep an E0-like executable representation as an internal prepared/lowering view of canonical Shard declarations. Give it total mathematical semantics for bounded computations and explicit operational relations for processes. Its constructors belong to executable-semantics libraries, not K's proof rules.

A client must not re-enter a program independently into a second environment to execute or compile it. For supported declarations the engine derives the executable view and binds the correspondence to the original declaration. Domain IRs may remain ordinary Shard data with their own semantics, connected through explicit refinement relations; one physical graph representation is not mandatory.

Initial coverage includes ordinary data, pure calls, supported recursion, and explicit environment boundaries. Canonical declaration construction is useful without writing dependent proof terms at every call site. Search and compilation share identities, context services, and evidence rather than each creating a separate semantic namespace.

### 7.1 Three author-facing categories

A surface declaration can be:

**Mathematical:** a definition or proposition in the full supported logic. No automatic code promise.

**Executable:** a definition in the declared supported compilation fragment. Automatic certified baseline lowering is part of the product contract.

**Realized specification:** a mathematical specification paired with a separately supplied executable implementation and a checked relation between them.

These are elaboration/build classifications, not three separate foundations. Unsupported mathematical constructs should never be silently compiled by pretending they have runtime behavior.

### 7.2 Erasure is a theorem-bearing pass

Dependent types and proof arguments need not appear in runtime data. But deleting them safely requires a precise erasure/specialization relation and its correctness argument.

A dimension used only to type a value may erase. A dimension used to drive a loop remains as data unless specialization removes it. Proof fields can erase under the logic's elimination discipline. Choice that supplies runtime data cannot be erased into an invented value.

Initially certify a deliberately restricted lowering, then enlarge it. Do not claim that importing a general mathematical kernel automatically gives a verified compiler for its entire term language. MetaRocq's erasure work is a precedent to study, not a proof of Shard's proposed pass. [L9]

### 7.3 Dependency-indexed correctness example

A useful logical interface might be:

```text
Buffer n
index : (n : Nat) -> Buffer n -> Fin n -> Byte
```

The executable representation could be a pointer and length, a fixed-size inline region, or a host-owned slice. The type-level API does not force that physical layout. A realization theorem connects the selected representation to the logical one.

For most application code, ordinary syntax and inferred obligations should hide this machinery. Explicit dependent terms remain available for library authors and specialized proof tasks.

### 7.4 Compilation chain

The intended artifact argument is:

```text
mathematical requirement
    <- application correctness proof
logical algorithm
    <- realization / supported erasure theorem
E0 executable view bound to the same declaration
    <- compiler/validator evidence
imperative representation
    <- target refinement evidence
machine program and encoded artifact
    <- byte identity and environment contract
actual accepted artifact
```

Each arrow denotes a stated relation, not necessarily equality. No universal relation silently conflates exact results, approximate results, nondeterministic behaviors, or resource failure.

The current ISA-as-library and validator architecture are assets to carry forward. Port their definitions and statements before deciding that the corresponding mathematics needs replacement. [R2, R3]

---

## 8. Code, denotation, and observation are separate objects

### D05 — Preserve intensional program identity

Use distinct representations for:

- **Logical terms:** expressions checked by K.
- **Executable program data:** objects manipulated by compilers and search tools.
- **Denotations:** mathematical meanings assigned by a semantics.
- **Observation models:** outputs, traces, costs, failures, or other contract-relevant behavior.

Do not identify two programs because their result functions are extensionally equal. They may differ in allocation, instruction selection, latency, or secret-dependent behavior.

A relation such as

```text
same_result p q
```

does not automatically imply

```text
same_trace p q
same_cost p q
same_security_observations p q
```

A compiler pass must state which relation it preserves.

Quotation is an operation on syntax/declared code objects. There is no unrestricted inverse that assigns every mathematical function a unique program. Typed quotation, capture-free substitution, and semantics-preserving code construction are library/tool capabilities with explicit validation.

Logical higher-order functions also do not mandate runtime closures. Static specialization or defunctionalization can provide many executable cases; later closure support can be another certified representation when a consumer justifies it.

A Shard program may manipulate *data representing* open syntax, including its holes. A closed theorem can quantify over such syntax and its completions. That is different from an unresolved native metavariable occurring in the proof of the theorem. Final closure checks native unknowns and pending references; they do not ban a normal datatype constructor named `Hole` inside quoted program data. No unrestricted `eval`/truth reflection is introduced by exposing contextual syntax.

---

## 9. Effects and processes

### 9.1 No effectful oracle in logical reduction

Logical evaluation is pure. Environment-dependent operations must appear through an explicit model or operational semantics, not by invoking live I/O while deciding a proof.

For a process, a starting interface can be:

```text
step : State -> Event -> State × Commands
```

together with relations on finite and infinite observations. The step function is ordinary total mathematics. The implementation may run indefinitely.

Safety over finite prefixes, eventual progress under fairness, termination, and resource failures are separate propositions. In particular, finite-prefix inclusion alone must not allow an implementation that stops immediately to pass a contract requiring continued service.

### 9.2 A concrete environment model before derived laws

State, histories, event streams, and capabilities should have a coherent model. Derive effect laws from it where possible. An integer sequencing clock can be an observable or runtime representation, but is not assumed to uniquely identify every possible logical history.

Foreign implementation behavior remains an explicit boundary assumption or a theorem about a more detailed model. Defining a mathematical model establishes internal coherence, not that a device or operating system follows it.

### 9.3 Profiles do not alter mathematical truth

Hosted, freestanding, embedded, and latency-constrained profiles select available realizations and obligations. They do not change K's proof rules. The final contract states which environmental behavior and resource conditions it assumes.

### 9.4 Explicit runtime metaprogramming is a supported capability

**D13:** an application may explicitly link the engine and `meta/` to inspect, search, evaluate, or compile newly supplied Shard programs at runtime. No application carries these services implicitly. A generated numerical kernel can remain a standalone artifact without including the compiler that produced it.

Build-time is relative to the artifact being produced: a deployed ML service may compile a model after startup. The no-ambient-runtime goal must not prohibit this intended use. Logical reduction remains pure; loading/execution/code-installation effects use explicit host adapters.

Bulk tensor or buffer payloads use explicit data/resource interfaces with lifetime, mutation, shape, and observation contracts. They need not become large expression-literal trees. Preparing a compute graph and transferring its input data are separate operations.

---

## 10. Proof automation without domain-specific authority

### 10.1 Old tactics become evidence producers

The current forms `rewrite`, `have`, `case-on`, `induct`, arithmetic closure, and explicit conversion can survive as authoring tools. Their products become ordinary proof terms or applications of validated library theorems.

Illustrative replacements:

| Present mechanism | Proposed authority |
|---|---|
| Conditional equation goal | A proposition built from equality, dependent products, and hypotheses |
| Rewrite step | Equality elimination/congruence plus ordinary proof construction |
| Structural induction | Application of an admitted inductive recursor |
| Numeric well-founded induction | A library well-foundedness theorem and recursor |
| Arithmetic-specific leaf rule | A validated arithmetic certificate and its soundness theorem |
| Refined-type fact | Projection from a subtype/law-bearing interface |
| Constructor injection | Derived no-confusion/injectivity theorem |
| `Admit` plus reporting discipline | An unresolved authoring goal, never an admitted theorem |

This is not a requirement to render every proof as lambda calculus in source. The stable checked representation and the convenient authoring representation serve different purposes. [R1]

### 10.2 Certified reflection

A domain package can expose:

```text
check_D : EncodedProblem -> Witness -> Bool
sound_D : check_D problem witness = true -> Meaning_D problem
```

An untrusted solver supplies the witness. A checked computation establishes the premise, and ordinary theorem application establishes the conclusion.

There are two independent connections to validate: the certificate checker is sound, and the encoded problem means the actual requested proposition. Correct certificates for the wrong encoding prove nothing about the original goal.

Domains such as polynomial normalization, bit-vector reasoning, interval arithmetic, finite maps, and SAT should primarily use this route. A verified checker written in Shard is not automatically granted an oracle primitive in K.

### 10.3 Fast computation policy

The first implementation may require a small, explicitly reviewed set of numeric reductions for practical checking. Such reductions must preserve the declared arithmetic semantics and be listed as implementation-critical code. Tests against a reference help; they are not a proof of equivalence.

Longer term, use proved specialized evaluators, compact computation witnesses, and certified compilation of the checker. Preserve a straightforward reference checker.

Do not grant a foreign solver an unrecorded oracle shortcut. The source-to-execution binding for a specialized checker must be justified or included in the declared execution TCB. In the near-term policy, K itself and its ordinary Shard certificate-checking computations may run on the reviewed, uncertified Rust backend and produce complete accepted results. This is explicit execution trust, not an extra mathematical axiom or a requirement to certify Rust first. A separately substituted native decision procedure needs its own correspondence/trust treatment. [L12; companion bootstrap Sections 4–5]

### 10.4 Preserve `meta/` as reusable mechanisms, not a second authority

**D12:** the rewrite ports are judged by the workflows they preserve, not by preserving their old proof strings or constructor layouts.

| Existing component | Requirement for the new architecture |
|---|---|
| `meta/invoke` / prepared invocation | Reusable, environment-bound preparation and direct invocation behind a public execution interface |
| `meta/sketch` | Native contextual holes supply identity/scope/typing; grammar structure, enumeration, and rank operations remain library policy |
| `meta/search` | Preserve correlated decisions and explicit residual constraints; distinguish heuristic failure from established impossibility |
| `meta/rewrite` / theorem capture | Obtain authentic facts through canonical scope/evidence queries; preserve congruence, observer-root, and restricted-context licenses |
| `meta/antiunify` | Produce typed conjectures and candidate guards; observations never become proof licenses automatically |
| `meta/proofgen` / `meta/proof` | Build evidence in memory and call the actual checker; textual formats remain optional renderings |
| `meta/shape` / `meta/plan` | Share target-independent descriptions and structured artifact plans, not tool-private internals |

These properties are visible in the current sources. Some theorem-authenticity machinery still lives in `tools/search`; a reusable successor should expose the semantics without forcing all clients to import that application's policy. [R6–R13]

A rewrite result carries the relation it preserves, its preconditions, allowed contexts, and evidence. Exact semantic equality does not imply identical source syntax or cost; observer equality does not imply congruence in all contexts. Approximate relations need an explicit error-composition law rather than being treated as ordinary transitive equality.

A proof about all valid fillings of a partial expression can justify reuse across search branches. It must retain its hole telescope, guards, and observation relation. Confluence, termination, completeness, and objective preservation of a rewrite/search policy are separate claims, not automatic consequences of each individual rewrite being correct.

---

## 11. Storage, sharing, and incremental composition

### D06 — Proof graphs are infrastructure, not a later compression feature

Store explicit terms as shared structures, with checked declaration boundaries and a canonical external encoding. Text is an inspection and interchange surface, not the only representation available to tools.

Separate at least these identities:

```text
logical declaration identity
public interface identity
implementation identity
evidence package identity
compiled artifact identity
```

A different proof of the same proposition need not invalidate all users. A changed implementation need not invalidate abstract client proofs, but it can invalidate code that inlined it. A change in assumption closure can invalidate acceptance even when the statement text is unchanged.

Hash-based addressing is not a license to equate distinct nominal types. Preserve nominal/generative identity deliberately. Hashes bind immutable content; they do not decide every question of type identity or trust.

A cache key for open terms must include the binding context, occurrence substitution, environment view, and the relevant metavariable/universe assignments and constraints. A coarse workspace revision is a safe initial key; dependency-sensitive keys can reduce invalidation later. Domain/grammar policy and observation relations are also part of keys for search conclusions. A result blocked by unknown assignments must not be cached as a universal negative. Node equality in one arena is not automatically semantic equality in another.

Persistent workspaces share immutable nodes but isolate assignments and obligations across branches. New hole identities are globally unique within their workspace lineage or remapped explicitly on merge; matching display labels or node hashes never merges decisions. Commit validates a closed snapshot, not the mutable state of a concurrently running search. Prepared execution contexts depend on committed code/realizations and explicit execution inputs, not on mutable proof-search assignments.

Serialized graphs are untrusted input. Validate bounds, acyclicity where required, declaration references, and scope before using them. Resource-exhausted or partially checked nodes must never be stored as successfully checked.

If correctness relies on cryptographic identity rather than exact-content comparison, state that assumption. Do not treat hashing as a metatheorem.

---

## 12. Mathematical coverage and optional Lean interoperability

### D07 — Make mathematical reach the target, not implementation fidelity

Keep three different goals separate:

1. **Mathematical coverage:** relevant definitions, statements, and arguments available under Lean's stated theory and declared assumptions can be faithfully expressed and justified in Shard. This is the long-term design target.
2. **Evidence transport:** a tool translates or reconstructs selected external developments into evidence accepted by Shard. This is valuable but is not a prerequisite for native development.
3. **Source or kernel-behavior compatibility:** unchanged Lean source/proof objects and matching upstream conversion or binary-module behavior. This is not a requirement.

The intended theorem-level target is schematic:

```text
if ΓL ⊢Lean p : P,
then translated assumptions/definitions τ(ΓL) support
     a Shard-checked proof q : τ(P).
```

This is a goal for a faithful interpretation or reconstruction process, not an implemented translator or a theorem proved by this proposal. The supported Lean theory must be named. The interpretation must preserve the mathematical meaning of the statement and its assumptions; merely embedding the source string or proving that an external checker returns true is insufficient. Nor may a translator map every proposition to a trivial truth or add the desired conclusion as a new axiom.

An external development's additional assumptions remain explicit assumptions or are discharged in Shard. No promise is made to turn all user-authored Lean axioms into unconditional truths. Likewise, a bug-induced acceptance by a particular implementation is not part of the mathematical-coverage target.

### 12.1 Reconstruction belongs at the edge

An optional adapter consumes external declarations and evidence, translates the chosen dependency closure, and emits Shard declarations and proofs. K checks only its native forms. Representation mismatches, generated casts, recursor adaptation, and library namespace mappings belong to the adapter unless they expose a genuinely general foundational need.

A change from upstream implicit conversion to Shard explicit equality may require proof reconstruction; it is not just renaming syntax. Begin with a small supported fragment and price the translation cost. When it cannot be justified, report the unsupported case rather than admit a bridge axiom silently.

Imported definitions with the same display name are not automatically identical to native ones. Establish their relationship before using imported theorems to authorize native optimizations. [L1, L7, L11]

### 12.2 Coverage measurements

Use native projects plus selected mathematical case studies: dependent algebraic structures, quotient-based constructions, real-valued error bounds, and well-founded or process arguments. For each, ask whether the mathematics remains natural, the proof composes with an executable realization, and the checking cost is tolerable.

External export or differential checking can add independent evidence for a shared fragment. It is an optional audit route outside native acceptance, not a gate that allows Lean to veto justified Shard-only improvements. The translation and its limitations must be stated; agreement between tools is not by itself a soundness proof.

---

## 13. Native contextual holes, partial proofs, and search

### D11 — Open programs are native engine objects

The current sketch library encodes a hole as a specially named `Call` in ordinary `Expr`; its own contract says the kernel never checks a sketch and that filling assumes the correct binder context. Shared IDs encode correlated choices. [R8]

The replacement makes contextual holes native to shared syntax and open judgments. The engine checks scope and expected types before a complete program exists. Elaborators, proof tools, synthesis, and partial evaluators use this one representation. This is more than adding `Hole Int` to an AST: it requires a well-defined context, substitution, obligation, and closure discipline.

Native support does **not** add a constant of arbitrary type, let a hole count as a proof, or make general search a privileged operation. K's final theorem authority still accepts only completed evidence under explicitly stated parameters and assumptions.

### 13.1 Four objects that must remain distinct

| Object | Meaning |
|---|---|
| Ordinary bound parameter `x : A` | The current argument ranges over values of A; it is not a missing implementation |
| Contextual hole `?h : [Ψ ⊢ A]` | One not-yet-chosen term of type A, allowed to depend on Ψ |
| Quoted open program | Ordinary data describing syntax, holes, and contexts; it can be inspected without supplying the holes |
| Checked theorem | A completed derivation, possibly universally quantified or explicitly conditional, with no unresolved workspace evidence |

An existential proposition saying an acceptable program exists is also distinct from an actual candidate and from an algorithm that finds one. The proposal makes candidates easier to represent; it does not assume existence or decidability.

A compiled search application can contain data describing a hole. What cannot remain in its accepted certificate is an unresolved metavariable standing in for a missing proof. This distinction is essential for metaprogramming the engine itself.

### 13.2 A hole record and occurrence

Schematic logical record:

```text
HoleDecl {
    id
    base_environment
    telescope Ψ
    expected_type A
    earlier_hole_dependencies
    source_origin
}
Occurrence = Meta(id, typed_substitution σ)
```

A workspace separately holds assignments, constraints, and search metadata. Grammar membership, candidate costs, enumeration order, domain selectors, and strategy preferences are not primitive components of the type theory. They attach through library records keyed by stable hole/region identities.

A hole's type can itself depend on earlier holes. Term, type, and proof holes use the same typed contextual mechanism; universe holes use their own level sort and constraints. Initial declarations have an explicit acyclic ordering. Later assignments may introduce fresh subholes, but the resulting dependency graph—including dependencies through types, contexts, and substitutions—must remain acyclic.

An assignment supplies one expression in the declaration telescope, not arbitrary text in whichever scope currently happens to be displayed. Each occurrence's checked substitution transports that expression to the occurrence context.

### 13.3 Sharing across binders: example

```text
?h : [u : Nat ⊢ Nat]

left  := fun x : Nat => ?h[u ↦ x]
right := fun y : Nat => ?h[u ↦ y]
```

Assigning `h := u + 1` yields `left = fun x => x + 1` and `right = fun y => y + 1`. There is one chosen template and two typed instantiations, not two independent choices and not necessarily equal runtime results.

If `h` is declared in the empty telescope, assigning it the local `x` is scope escape and is refused. Reusing a hole under unrelated binders requires explicit appropriate substitutions; equal printed variable names are insufficient.

For dependent data:

```text
?n : [· ⊢ Nat]
?v : [· ⊢ Vector Byte ?n]
?ok : [· ⊢ Good ?n ?v]
```

Assigning `n` refines the expected type of `v` and proposition for `ok`. It does not clone the holes or prove `Good`. Dependent contexts and occurrence substitutions must be revalidated or updated through a justified incremental rule.

Assignment performs transitive occurs/dependency checks, validates scope, checks the proposed term in the declared telescope, and records any remaining subgoals. An assignment that still contains well-formed subholes is a refinement, not a completed solution.

### 13.4 Partial judgments and result types

Use separate APIs and result types for partial construction and closed admission:

```text
validate_open(E, W, Γ, term, expected, limits)
    -> OpenValid(derivation, dependencies)
     | Pending(obligations, blockers)
     | Incompatible(reason_or_certificate)
     | Exhausted(resource)

admit_closed(E, candidate_package, completed_assignments, limits)
    -> Accepted(CheckedDeclaration)
     | Rejected(diagnostic)
     | Exhausted(resource)
```

`OpenValid` means that the particular open judgment has been established under its declared hole context. It can still contain unsolved holes. It does not say the workspace has a completion. `Pending` means further facts must be checked before even that judgment is established. `Incompatible` must name its scope: a proposed assignment can be rejected without proving that the entire region has no completion.

Unification is an untrusted proposal mechanism, not an interpretation of `Accepted`. An algorithm that finds no solution reports that limitation. It must not automatically produce `Unsatisfiable`. No API silently guesses a fill because an unknown occurs where a proof is expected.

### 13.5 The partial-derivation contract

Let `Comp(W)` denote the completions that satisfy W's typed hole declarations, assignments, and explicit constraints. The intended adequacy statement is schematically:

```text
OpenDerivation(E, W, Γ, t, A)
  and θ ∈ Comp(W)
    imply Derivation(E, θ(Γ), θ(t), θ(A)).
```

This is a metatheoretic obligation for the native open rules. It is not a theorem already proved by this document and it is not a primitive truth-reflection rule. `Comp(W)` may be empty. A validated open proof of P with a pending `?p : P` does not establish P and does not establish the existence of a completion.

For final admission, validate a particular complete assignment, expand assignments through the term and its type/context/declaration closure, discharge every reachable obligation, and check the resulting closed evidence under the declared foundation. No unresolved proof may be concealed by proof irrelevance, runtime erasure, a dead branch, or an opaque constant lacking admission evidence.

Only dependencies of the committed result must be closed; unrelated scratch holes in another workspace branch do not block admission. Conversely, dependency closure includes types, universe levels, instantiations, and referenced draft declarations—not just the visible runtime body.

A legitimate alternative is explicit generalization into a new theorem with parameters/hypotheses, which is then checked normally. That creates a different statement; it cannot fulfill a requirement for the original closed theorem without supplying the missing evidence.

### 13.6 Reasoning about candidate spaces is ordinary mathematics

The foundation must also support *closed* theorems about *data describing* open programs. Schematic library relations include:

```text
Completion(sketch, filling)
WellTypedFilling(environment, sketch, filling)
Instantiate(sketch, filling)
Preserves(contract, source, destination)
```

A useful theorem can establish, for all typed fillings θ satisfying guards C(θ), that applying a transformation to a template preserves relation R after instantiation. Its proof is complete even though it quantifies over unknown future fillings; those are ordinary bound parameters, not unresolved proof holes.

Distinguish searches over syntax from searches over values or denotations. Two fillings that compute the same value may be distinct programs with different cost or observations. Proof irrelevance in Prop and extensional equality of denotations must not collapse the search engine's program identities or hole decisions implicitly.

The correspondence between library-level encoded open syntax and native workspace operations must be stated and tested, and proved where a certified search claim relies on it. A metaprogram calling `validate_open` does not thereby acquire unrestricted reflection from represented acceptance to arbitrary truth.

### 13.7 Constraints: validate proposals, do not install a universal solver

The engine natively validates contextual substitutions and basic typed constraints. Libraries may represent grammar membership, affine arithmetic, relational guards, target capabilities, or cost conditions, supplying certificates where a correctness claim relies on them. One small constraint protocol should not become a kernel constructor for every domain.

Initial automation may cover explicit assignments, a restricted pattern-unification fragment, and replay of supplied certificates. Higher-order unification and dependent search need not be complete. Unsupported cases remain pending or fall back to candidate enumeration outside K.

A workspace refinement must distinguish at least:

- **Selection:** deliberately choosing a subset of candidates, such as assigning a hole. It need not preserve the full search space.
- **Equivalent simplification:** representing the same completions in a cheaper form; requires a correspondence if used in a certified search result.
- **Unsatisfiability pruning:** establishing that a region has no admissible completion.
- **Representative/dominance pruning:** retaining substitutes for removed candidates under a specified observation or objective relation. The removed region may be inhabited.

For observational pruning, validity preservation is not enough to claim the cheapest candidate was retained. Objective preservation needs its own bound or dominance statement. A heuristic optimizer may still return a fully certified individual program without certifying exhaustive coverage or optimality. These are different product claims.

### 13.8 Exactness and correlation remain library obligations

The current `meta/sketch` has exact rank/count operations for its stratified grammar fragment and restricts sharing to preserve those counts. `meta/search` deliberately separates exact constraints from unsupported approximations. Do not erase those distinctions merely because holes become native. [R8, R9]

A grammar's candidate count, the number satisfying a logical constraint, and the number of equivalence classes under a rewrite profile are different quantities. General contextual holes imply none of these is cheaply computable. Exact rank/count is promised only for a declared grammar/constraint fragment; an unsupported constrained count returns `Unknown`/`Unsupported`, never the unconstrained product mislabeled as exact.

The same `HoleId` denotes one choice per branch. Two holes with the same type or grammar remain independent unless explicitly related. Constraints may correlate different holes. A pruning transform cannot clone a shared hole into independent ones, merge equal-looking independent holes, or split a conjunction into independent exclusions without a justified change of representation.

With occurrence substitutions, a single template choice may produce different expressions in different contexts. Count assignments to declared holes under the stated grammar policy; do not silently count occurrences or normalized results instead.

### 13.9 Partial evaluation and proof-domain-aware rewriting

Open evaluation returns a residual expression and explicit dependencies/blockers. It does not make up a value for a hole, treat a blocked pattern as a mismatch, or execute host effects as logical reduction. Reductions independent of a hole can proceed. Rules requiring facts about a hole must wait, branch, or retain those facts as explicit conditions.

Preserve the current distinctions among unrestricted congruence profiles, observer-root profiles, and restricted-context/spine profiles. A root observational theorem is not a rewrite license under an arbitrary binder or constructor. Partial rewriting must preserve guards and the occurrence substitutions on which the license depends. [R10]

### 13.10 Persistent workspaces and parallel agents

An immutable checked environment may be shared by many workspaces. `fork` shares structure and creates a distinct assignment/constraint history. `refine` returns a new version with a dependency record. `merge` validates a proposed joint assignment; it cannot identify holes by display name or silently overwrite a conflicting solution. `commit` closes and checks a frozen candidate.

Canonical caches depend on context, substitutions, assignments, universe state, environment, and relevant policy. Initial full-revision invalidation is acceptable. Dependency-sensitive invalidation is an optimization with its own validation tests. A blocked result may be revisited; it is not a permanent negative.

A proposer cannot weaken the goal, widen permitted axioms, redefine a referenced declaration, or change the observer and still claim progress on the same acceptance target. Human and agent interfaces operate over the same native objects and obligation state.

### 13.11 Recommended implementation scope

First implement native hole declarations/occurrences, typed explicit substitutions, simple open checking, branch isolation, assignment validation, and a full closed-admission recheck. Port one exact first-order sketch/search workflow and one dependent hole example immediately.

Then add reusable partial-derivation evidence and certified constraints where measured use justifies them. Do not require a general modal type system, arbitrary levels of metavariables, a complete higher-order solver, or native execution of unfinished effects as the first milestone. The representation should preserve scope/level distinctions needed for later staging without introducing an unreviewed tower of logical reflection.

The substitution/closure theorem, not a larger trusted search algorithm, is the key foundational deliverable.

---

## 14. Kernel implementation and assurance

### 14.1 A narrow, inspectable API

The trusted core needs declaration admission, term checking, conversion, and carefully delimited environment operations. It should not require filesystem access or live network/I/O handlers to judge a declaration already supplied as data.

The first implementation is in Shard; the Rust executor may grow to run it and the surrounding engine directly with normal modules. The richer subject logic and contextual workspace structures are ordinary runtime data. A second Rust proof authority or Lean-hosted authority is not part of this plan. The reviewed Shard checker on the explicitly trusted Rust runtime may complete certification now. Independent tools may assist testing; certifying the runtime itself is additional assurance rather than an acceptance prerequisite.

Do not make removing every bootstrap dependency the first acceptance milestone.

### 14.2 Two implementations and explicit metatheory

Use a straightforward Shard reference implementation first; introduce a separately structured optimized Shard implementation when measurements justify it. They share a declarative specification, not a requirement to mirror internal algorithms. A shared bootstrap can still create correlated failures, so two implementations are evidence rather than complete independence. Optional export of a supported fragment to an external checker adds a different validation route without making it a build dependency.

The assurance program should establish:

1. A precise declarative typing/admission specification.
2. Soundness of successful checking relative to that specification.
3. Correctness of serialization, environment construction, and relevant accelerators.
4. An explicit trusted execution contract for the Rust-hosted checker, with conformance tests now and optional certified compiler/runtime realization later.
5. An explicit account of semantic-model assumptions for the logical theory.
6. Substitution, context transport, workspace isolation, and closure correctness for native partial judgments.

These are different deliverables. Lean4Lean and Candle are precedents for parts of this program, not evidence that a new Shard checker already satisfies them. [L8, L10]

### 14.3 No unrestricted self-soundness shortcut

Do not add a rule saying that K's acceptance of a represented theorem directly makes that theorem true. Specialized reflection works because its syntax, semantics, and soundness theorem are explicit.

A full semantic soundness proof may require a stronger metatheory or extra model assumptions. Compilation correctness for K does not remove that issue. The architecture should remain useful without claiming unconditional self-justification.

---

## 15. Migration strategy

### D08 — Rewrite proof text aggressively; migrate meanings conservatively

The current equational statements have an obvious initial target shape:

```text
forall parameters, premise_1 -> ... -> premise_k -> lhs = rhs
```

But translating this syntax is not enough. Preserve the meaning of primitive operations, type identities, opacity, totality obligations, and imported assumptions. A rewritten proof of an accidentally changed theorem is not a successful migration.

Create a migration manifest for each interface recording its old statement, new statement, changed definitions, and any intentional strengthening or weakening. Where an exact semantic correspondence cannot be established immediately, mark the item for review rather than manufacturing a legacy axiom.

### 15.1 Retain versus replace

| Asset | Treatment |
|---|---|
| Requirement intent and useful interface laws | Preserve and review |
| Algorithms, machine-model definitions, environment models | Port and compare semantics |
| Current proof strings | Rewrite freely |
| `meta/` workflows | Preserve as first-class acceptance tests; representations may change |
| Reserved-call sketch holes | Migrate to contextual hole declarations/occurrences; preserve correlations and explicit domains |
| Prepared invocation | Promote to a public environment-bound service, without evaluator-private table dependencies |
| Textual proof generation | Retain rendering; make structured evidence the in-process path |
| Specialized kernel tactics | Reimplement primarily as proof-producing tools/libraries |
| Existing binary encoders and byte-tie discipline | Preserve unless a concrete defect is found |
| Representation and memory-management mathematics | Reuse where valid; do not discard because the proof syntax changes |
| Current proof-acceptance status | Historical evidence only; not automatic admission to K |
| Performance fixtures and negative tests | Port early and strengthen |

### 15.2 Do not start by rewriting everything

Choose a small foundation prototype and a bounded set of pathfinders including metaprogramming and search. Gate the native-hole/context interface before porting every AST walker. Retain a first-order search adapter where useful, but do not make every client translate between independent hole or theorem namespaces. Once the interfaces and rule inventory are fixed, parallel migration becomes much more meaningful.

The expensive unknown is not how many strings agents can rewrite. It is whether the new foundation provides correct, compositional semantics and acceptable checking behavior across the workloads the product actually needs.

---

## 16. Pathfinders and acceptance gates

No runtime measurements or prototype proof-checking results are claimed in this document. The following are proposed tests.

### G0 — Foundation contract

Freeze the first Shard theory version: declarative judgments, supported declarations, conversion behavior, axiom policy, and serialization rules. Identify the established metatheory used as a baseline and the precise deviations, if any. An external comparison must separately pin its theory/version and translation. Produce a precise enough rule document that two implementations can disagree meaningfully; no Lean-hosted prototype is required.

A mismatch must be classified as unsupported representation, a translation error, a Shard implementation bug, an external implementation problem, or an intentional and separately justified theory difference. No silent semantic extensions. A native Shard verdict follows Shard's declared rules, not a majority vote among executables.

### G1 — Core correctness and hostile-input suite

Positive cases: polymorphic identity, dependent function application, equality transport, inductive elimination, a subtype, a well-founded definition, and the supported quotient rules.

Negative cases must cover at least:

- Universe collapse and bad level instantiation.
- Scope capture, malformed binders, and wrong-type theorem instantiation.
- Negative inductive occurrences and forged recursors.
- Illicit existential/disjunction elimination into runtime data.
- Cyclic definition admission and circular proof dependencies.
- Substitution of a theorem with a different assumption closure.
- A forged core identity or stale imported declaration.
- Resource exhaustion treated as success or logical rejection.
- Corrupted proof graphs, forged cache receipts, and altered requirement identities.

Use both deterministic fixtures and adversarial generation. Passing them is evidence, not a proof of soundness.

### G2 — Reusable abstraction

Define a law-bearing algebraic interface and prove a fold/composition result once. Instantiate it for multiple carriers and operations with genuinely different representations. The proof should transport by application, not be separately rewritten per instance.

Also pass a refined opaque value through a consumer module with its implementation both absent and present in the build. Its public typing and proof obligations must not change accidentally.

### G3 — Compiler and machine pathfinder

Port one nontrivial memory/call/representation argument from the current compiler work. Include an actual emitted artifact and byte tie. Compare warm/cold checking cost, peak memory, unique proof nodes, and author intervention with the old path.

The current formatter-through-generic-compilation goal remains a valuable product test. Do not require it to complete before testing the new foundation, but do not abandon the roughly-zero-hand-lowering criterion during cutover. [R5]

### G4 — Mathematics that changes an implementation

Connect rational interval computations to a real-valued specification, for example a square-root approximation under stated input bounds. Prove an error property, then connect it to an executable implementation. Include the full mathematical dependency closure and exact assumptions. It may be developed natively; successful import is not a prerequisite.

This tests the seam between abstract mathematics and executable realization, not merely whether the new checker accepts a theorem about reals.

### G5 — Process and observation discipline

Model a reactive state machine without an application-level fixed lifetime. Prove a finite-prefix safety property and a separate progress statement under explicit environment conditions. Demonstrate that an implementation that prematurely stops is rejected when the contract requires progress.

### G6 — Independent artifact replay and cutover

A clean environment rechecks the requirement, evidence, assumptions, and bytes using the reviewed Shard engine. The maintained, uncertified Rust backend is an allowed execution root with recorded provenance; no certificate for Rust is required. No warm-cache assertion, legacy proof verdict, or unrecorded foreign solver oracle substitutes for checking the requested evidence.

Only after G1–G4 and the basic G7 open/closure path are convincing should bulk proof migration dominate. G5/G6 protect process and artifact semantics; G8/G9 ensure that a batch proof checker does not replace the intended embedding/search product. G7's minimal slice should be co-developed with G1, not postponed until all mathematical libraries are ported. External mathematical replay may proceed independently and must not replace the Shard-written checker.

### G7 — Native holes, dependent contexts, and closure

Positive cases: one shared hole under renamed binders with explicit substitutions; independent holes with the same expected type; a type depending on a value hole; a proof hole depending on that value; partial beta reduction; refinement introducing well-formed subholes; explicit generalization to a different parameterized theorem; a closed theorem about quoted open syntax.

Negative cases: local-variable escape; reuse under an incompatible dependent telescope; direct and indirect assignment cycles through types or substitutions; undeclared universe holes; merging branch assignments by display name; a native proof hole hidden behind an opaque draft or erasure; `?p : False` accepted as a proof; automatic generalization silently weakening a closed requirement; a blocked comparison treated as incompatibility; stale partial evidence reused after assignments change.

Demonstrate that final admission refuses every reachable unresolved obligation, even when the runtime body is unchanged. The open-derivation substitution/closure claim needs a formal account; tests do not prove it.

### G8 — Embeddable engine and in-process `meta/`

A library client creates declarations through canonical APIs, prepares and repeatedly invokes an entry, forks a partial workspace, constructs evidence in memory, commits a candidate, and requests a byte-tied realization. No source-file round trip, CLI process, or second declaration environment is mandatory. A selected Rust execution profile runs the Shard services directly.

Include an ML-shaped compute example with shape-specialization and an explicit exact/approximate contract. Buffer data arrives through a separately specified resource interface, not a giant syntax literal. Count module preparations, environment builds, and marshaling costs rather than assuming an in-process API is automatically fast.

### G9 — Search-space fidelity and reusable partial evidence

Port a small existing exact grammar workload. Compare native-hole enumeration with a hand-enumerated ground truth for correlated holes and scoped occurrences. State whether the count addresses syntactic assignments, accepted candidates, or quotient representatives.

Include a dependent constraint where Cartesian multiplication gives the wrong count, a guard blocked until assignment, an observer-root rewrite invalid in a nested context, a nonempty branch removed by a licensed representative replacement, and a heuristic timeout that must not count as UNSAT.

Establish one complete theorem about all valid fillings of a template and reuse it across several assignments. Separately demonstrate that certifying one found candidate does not assert search completeness or optimality. The kernel must not acquire grammar enumeration or candidate-order policy to pass this gate.

### 16.1 Performance decision rule

Do not invent a universal speedup promise. Record a baseline and agree concrete budgets before the relevant prototype is run. A richer foundation that makes basic compilation impractical must be optimized, narrowed at the executable boundary, or reconsidered.

Distinguish logically necessary work from reparsing, repeated elaboration, serialization inflation, and repeated normalization. Fixing the latter should not require weakening the logic.

---

## 17. Risks and deliberately deferred work

**Conversion complexity:** A dependent kernel may be larger and its checking behavior less predictable than the old equation checker. Mitigation: explicit terms, controlled opacity, resource outcomes, shared data, and a justified fast fragment under Shard's rules. This is a cost, not something the proposal hides.

**Coverage mistaken for compatibility:** The long-term Lean-mathematics target may tempt indiscriminate imports, false promises of easy proof transport, or accidental identification of similarly named definitions. Mitigation: native development first, small translated closures, explicit assumption mappings, and bridge theorems.

**Dependent-type overuse:** Indexed data can create excessive transports. Mitigation: permit both intrinsically typed and extrinsically validated representations, with interfaces chosen for the consumer.

**Certificate expansion:** Proof terms can still explode. Mitigation: proof sharing, theorem boundaries, certified reflection, and measurement on unique nodes rather than only source text.

**Specification drift:** Agents can prove a nearby but weaker proposition. Mitigation: immutable acceptance targets and independent comparison of definitions and assumptions.

**Bootstrap circularity:** A checker compiled by an unverified chain is not justified solely because its source is elegant. Mitigation: explicit execution authority and independent replay.

**Foundational drift:** Small local modifications can invalidate a metatheoretic argument or change old receipts' meaning. Mitigation: version the Shard rule package, distinguish implementation changes from logical changes, and supply the appropriate argument and revalidation. Do not freeze the architecture around a foreign implementation merely to avoid making this distinction.

**Vertical-integration scope:** Ownership does not require rebuilding a whole external ecosystem before Shard is useful. Mitigation: a small Shard checker, public engine interfaces, `meta/` pathfinders, and a narrow useful mathematical library; optional interoperability remains outside the critical path.

**Open-state authority leakage:** Partial typing may be mistaken for a witness or theorem. Mitigation: native contextual judgments, separate result types, explicit solution obligations, and a closed-admission recheck.

**Constraint/correlation explosion:** Native holes do not make dependent search or constrained counting tractable. Mitigation: declared exact fragments, explicit `Unknown`/blocked outcomes, persistent structure, and separate candidate-validity versus search-completeness claims.

**Engine fragmentation:** E0, host embeddings, or `meta/` might recreate their own namespaces and runtimes. Mitigation: shared identities/views, public prepared services, structured evidence, and G8 before large-scale integration.

Defer universal proof-system interoperability, a universal pass scheduler, cubical/higher-inductive extensions, kernel-level effect handlers, general native recursion in logical conversion, and a requirement that all mathematical definitions compile. These are not needed to validate the proposal.

---

## 18. Questions for the next review with Fable

Use the decision IDs to keep the discussion stable.

**D01 — Native foundation:** Given the explicit Shard-native requirement, does the dependent baseline remove enough recurring interface/transport machinery to justify its cost? Does a Shard-native HOL alternative win on actual workloads? No proposal may make a Lean executable the prototype's acceptance authority.

**D02 — Rule package:** Which precise dependent rules belong in Shard theory version 1? Which established account justifies the combination? What should differ from Lean for Shard's workloads, and what proof, interpretation, or explicit assumption would justify that difference?

**D03 — Computation:** Which concrete compilation certificate stresses conversion most? Does optional explicit conversion evidence reduce total checking cost? What numeric support does the native reference checker need, and what is its exact trust treatment?

**D04 — Executable boundary:** What internal execution view best serves the shared declaration system? What realization theorem is needed, and can an embedding client inspect/execute/compile the same declaration without restating it?

**D05 — Program identity:** Where can equality of denotations currently be mistaken for equality of program representations, cost, or traces? Fix those interfaces before introducing more extensional reasoning.

**D06 — Evidence graph:** What is the smallest persistent checked-environment interface that avoids re-elaborating entire closures without trusting arbitrary cached verdicts?

**D07 — Mathematical reach:** Which mathematics would improve a Shard implementation but would be awkward or impossible in the proposed native fragment? Develop that case natively first or through a small justified translation; do not conflate mathematical reach with source compatibility.

**D08 — Migration:** Which bounded set of modules exposes both logical and `meta/` risks? For each, who freezes the meaning and who checks the proposed migration?

**D09 — Embedding:** Which public services remove the prepared-invocation import cycle without exposing evaluator tables? Can syntax-only clients avoid importing the full checker?

**D10 — Common identity:** Which declarations, assumptions, and realization references must be shared across checking, search, and invocation? Which physical representations should remain independent?

**D11 — Native holes:** Specify the first telescope/substitution/assignment rules. How are dependencies through types and universes checked? What is the smallest open-typing adequacy theorem and closed-admission gate?

**D12 — Search evidence:** Which existing workload demonstrates useful pruning over partial candidates? What exactly is its coverage/count/observer claim, and which facts are merely heuristic?

**D13 — Runtime metaprogramming:** Which engine services must be explicitly linked for a deployed compiler or ML runtime, and what bulk-data/host capability interface avoids implicit global services?

A useful next review should amend the architecture, identify a better foundation, or select pathfinders that could falsify it. It should not yet assign agents to mechanically port every proof.

---

## 19. Decision summary

| ID | Proposed decision | Confidence |
|---|---|---|
| D01 | One Shard-owned foundation, implemented in Shard from the first prototype; dependent baseline preferred | Native ownership is an explicit user requirement; logical selection remains a design judgment |
| D02 | A versioned, coherent Shard rule package informed by established metatheory; justified divergence is allowed | High |
| D03 | Bounded checking with explicit exhaustion; fixed conversion, no equality reflection | High |
| D04 | Separate mathematical validity from executable realization; certify erasure/lowering | High |
| D05 | Separate program syntax, denotation, and observation models | High |
| D06 | Shared explicit evidence and immutable checked environments from the beginning | High |
| D07 | Target faithful mathematical reach; keep proof transport optional and compatibility at the edge | High on separation; full coverage is an unproven long-term goal |
| D08 | Migrate proof text freely but freeze and compare theorem meaning | High |
| D09 | Preserve the canonical embeddable engine; keep only logical authority minimal | User-agreed direction |
| D10 | Shared declaration/identity system with prepared execution and linked realization views | High; concrete API needs pathfinders |
| D11 | Native contextual holes and partial judgments, separate from closed theorem acceptance | User-requested direction; precise rules/adequacy remain to prove |
| D12 | Reusable partial-program/search evidence; keep enumeration and solving outside K | High on boundary; exact fragments remain consumer-driven |
| D13 | Explicit runtime linking of engine and `meta/`, not an implicit universal runtime | High |

**Bottom line:** the rewrite should strengthen the system that `meta/` already makes possible: import Shard, construct partial programs and proofs, refine them through shared logical resources, execute and inspect them, and obtain justified artifacts. Own the foundation and implement it in Shard; run it efficiently on Rust now. Make holes native without making them evidence. One semantic environment and explicit authority boundaries should reduce, not multiply, the concerns that each consumer must understand.

---

## Sources and evidence notes

The architectural recommendations are proposals. The references below support descriptions of existing systems or repository observations, not the correctness or performance of an unimplemented Shard redesign.

### Shard repository sources

R1–R5 retain the earlier foundational source baseline. R6–R13 identify the embedding/search review baseline separately; sketch and prepared invocation were also rechecked at the latest inspected commit. No repository runtime or conformance tests were executed for this documentation revision.

- **[R1] Current proof representation.** `kernel/proof.shard`: conditional equation goals and the specialized certificate constructors.  
  <https://github.com/computer-whisperer/shard/blob/e682c81233beeacb6d1c2f296727f5a74c098e33/kernel/proof.shard>
- **[R2] Certificate architecture.** `docs/CERT.md`: explicit conversion, sharing, and validators.  
  <https://github.com/computer-whisperer/shard/blob/e682c81233beeacb6d1c2f296727f5a74c098e33/docs/CERT.md>
- **[R3] Machine models as libraries.** `docs/ISA.md`: model and composition boundaries.  
  <https://github.com/computer-whisperer/shard/blob/e682c81233beeacb6d1c2f296727f5a74c098e33/docs/ISA.md>
- **[R4] Current admission and totality intent.** `docs/OVERVIEW.md`, especially the definition-admission discussion.  
  <https://github.com/computer-whisperer/shard/blob/e682c81233beeacb6d1c2f296727f5a74c098e33/docs/OVERVIEW.md>
- **[R5] Current generic-compilation goal and author-residue gate.** `docs/COVERAGE.md`.  
  <https://github.com/computer-whisperer/shard/blob/e682c81233beeacb6d1c2f296727f5a74c098e33/docs/COVERAGE.md>

### Embedding and metaprogramming source observations

- **[R6] Invocation and retained preparation.**
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/meta/invoke/mod.req.shard>
  <https://github.com/computer-whisperer/shard/blob/5abc60074f260f882a92a563f5c9d8fcd891199a/meta/invoke/prepared.shard>
- **[R7] In-process proof candidates and proof-generation templates.**
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/meta/proofgen/mod.req.shard>
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/meta/proof/mod.req.shard>
- **[R8] Sketch holes, context assumptions, and exact grammar addressing.**
  <https://github.com/computer-whisperer/shard/blob/5abc60074f260f882a92a563f5c9d8fcd891199a/meta/sketch/mod.req.shard>
- **[R9] Correlated partial search and prepared constraints.**
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/meta/search/mod.req.shard>
- **[R10] Rewrite domains and authentic theorem capture.**
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/meta/rewrite/mod.req.shard>
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/tools/search/theorem_scope.shard>
- **[R11] Anti-unification and observational guards.**
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/meta/antiunify/mod.req.shard>
- **[R12] Target-independent shape and build-plan libraries.**
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/meta/shape/mod.req.shard>
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/meta/plan/mod.req.shard>
- **[R13] Signature-driven grammar generation.**
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/tools/search/typed_grammar.shard>

### External primary references

The original reference set was assembled for v0.1 on September 5, 2026. For v0.2, the live type-system and universe references, Axioms and Computation, Lean4Lean paper, and proof-validation guidance were reread. For v0.3 the new contextual-type references L13–L14 were consulted; the earlier external reference set and release metadata are carried forward, not reported as freshly reverified. Live pages can change. None of these sources defines Shard acceptance by reference; G0 must state the native rules precisely.

- **[L0] Historical Lean comparison release and source pin from v0.1, not an implementation dependency.**  
  <https://github.com/leanprover/lean4/releases/tag/v4.33.1>  
  <https://github.com/leanprover/lean4/tree/819816b2e0a3bf405af45ae5c7af2491d8f5bee6>
- **[L1] Lean Language Reference, The Type System.** Explicit terms, proof checking, and conversion. This is live documentation; do not treat it as the specification of the historical comparison release or of Shard.  
  <https://lean-lang.org/doc/reference/latest/The-Type-System/>
- **[L2] Lean Language Reference, Universes.** Predicativity, impredicative propositions, noncumulativity, and level expressions.  
  <https://lean-lang.org/doc/reference/latest/The-Type-System/Universes/>
- **[L3] Theorem Proving in Lean, Inductive Types.** Inductive propositions and elimination principles.  
  <https://lean-lang.org/theorem_proving_in_lean4/Inductive-Types/>
- **[L4] Lean Language Reference, Inductive Types.** Positivity, universe checks, recursors, and indexed families.  
  <https://lean-lang.org/doc/reference/latest/The-Type-System/Inductive-Types/>
- **[L5] Lean Language Reference, Recursive Definitions.** Structural and well-founded elaboration.  
  <https://lean-lang.org/doc/reference/latest/Definitions/Recursive-Definitions/>
- **[L6] Lean Language Reference, Quotients.** Quotient constructors, lifting, and respectful operations.  
  <https://lean-lang.org/doc/reference/latest/The-Type-System/Quotients/>
- **[L7] Theorem Proving in Lean, Axioms and Computation.** Quotient soundness, extensionality, classical choice, erasure, and noncomputable definitions.  
  <https://lean-lang.org/theorem_proving_in_lean4/Axioms-and-Computation/>
- **[L8] Candle project.** A verified HOL Light implementation and specialized computation.  
  <https://cakeml.org/candle/>  
  <https://cakeml.org/projects.html>
- **[L9] MetaRocq project.** PCUIC checking, metatheory, erasure, and stated normalization assumptions.  
  <https://metarocq.github.io/>
- **[L10] Mario Carneiro, Lean4Lean: Towards a Verified Typechecker for Lean, in Lean.** Independent checking, declarative correspondence, and the normalization/fuel discussion.  
  <https://arxiv.org/html/2403.14064v2>
- **[L11] Lean Language Reference, Validating a Lean Proof.** Independent checking and protection of the intended statement.  
  <https://lean-lang.org/doc/reference/latest/ValidatingProofs/>
- **[L12] Lean FAQ, trusted computing base.** Native execution and `native_decide` add execution dependencies beyond mathematical proof checking.  
  <https://lean-lang.org/faq/>

- **[L13] Andreas Abel and Brigitte Pientka, Explicit Substitutions for Contextual Type Theory (2010).** A primary reference on separating ordinary and meta substitutions and checking contextual syntax. Consulted for v0.3; not a proof of Shard's proposed open-term extension.
  <https://arxiv.org/abs/1009.2789>
- **[L14] Mathieu Boespflug and Brigitte Pientka, Multi-level Contextual Type Theory (2011).** Contextual dependencies, incomplete proof objects, and distinctions among object/meta levels. Consulted for v0.3; adopting the paper's full multilevel calculus is not proposed here.
  <https://arxiv.org/html/1111.0087v1>
