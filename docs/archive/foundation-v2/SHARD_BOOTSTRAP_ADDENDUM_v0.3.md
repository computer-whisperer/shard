# Shard bootstrap and execution architecture

## A maintained Rust backend for the embeddable Shard engine and its contextual workspaces

**Status:** DRAFT v0.3 — aligned with the embeddable-engine and native-hole proposal; detailed interfaces remain proposed, not implemented or formally validated.  
**Date:** September 5, 2026.  
**Prepared for:** Christian Balcom and discussion with Claude Fable.  
**Evidence baselines:** execution and `meta/` review at `82c00e9b4e2e35eda689c632d3240a81be2429bd`; prepared invocation and sketch interfaces rechecked at `5abc60074f260f882a92a563f5c9d8fcd891199a`. No execution tests are claimed.  
**Relationship:** Supersedes `SHARD_BOOTSTRAP_ADDENDUM_v0.2.md` and retains its maintained-Rust/acceptance policy. Complements [Foundation proposal v0.3](SHARD_FOUNDATION_PROPOSAL_v0.3.md), which owns the logical rules, canonical engine services, and contextual-hole semantics. This document owns direct execution, embedding runtime interfaces, conformance, and provenance.  
**Scope:** Architecture and proposed policy. No implementation changes, runtime benchmarks, conformance results, or formal correctness proofs are claimed here. New APIs and commands are schematic.

## 0. Recommendation

Promote the Rust bootstrap from disposable narrow-language scaffolding to a maintained implementation of Shard's executable semantics. Expand it wherever doing so removes nested interpretation, permits ordinary modular engine source, or improves the development loop. Execute the Shard-written engine directly: not only the proof checker, but also inspection, native open-workspace services, prepared invocation, `meta/` tools, and compilation libraries. Permit the reviewed Rust-hosted checker to perform normal, acceptance-grade certification under an explicitly stated trusted execution base; do not require formal certification of the host before useful software can be certified.

Preserve a single logical rule implementation in Shard. Rust implements the execution of that program, not a second proof system. A self-hosted compiled backend later executes the same source or an explicitly related implementation. A common semantic specification, shared interfaces, differential testing, negative tests, and coverage accounting prevent the execution backends from becoming different languages.

**The invariant is semantic agreement, not minimal Rust source size.** The embedding client sees one Shard declaration system and public service API. It does not reconstruct its own type checker, theorem namespace, hole representation, or interpreter merely to use the optimized execution path.

### 0.1 Changes in v0.3

The public product is the canonical embeddable engine; K is its smaller logical authority. Native contextual holes are engine-managed data with Shard-defined validation semantics. Rust executes that implementation without creating a second unifier or theorem authority. Retained invocation and workspace operations are normal public services. Reference calls, candidate execution, proof checking, and compiled realization share identities but have distinct result contracts.

The first integration gates now include in-process `meta/`, correlated/contextual-hole behavior, branch/cache isolation, and an ML-shaped host workload. The v0.2 rule permitting complete certification on a reviewed but uncertified Rust executor is unchanged.

## 1. What changes from v0.1

| Previous direction | Revised direction |
|---|---|
| Run the new checker as an application of the existing self-hosted interpreter | Run its loaded executable code directly on the improved Rust backend; retain the interpreter tower as a diagnostic route |
| Avoid expanding the external seed by staging translators first | Expand the Rust backend when it is the practical way to support the intended language; staging remains an option, not a ritual |
| Treat a tiny, mostly frozen seed as the long-term execution constraint | Maintain a source-buildable Rust execution path, with versioned semantics and an explicitly supported feature profile |
| Make certified image promotion the main route to normal execution | Accept ordinary certification on a reviewed, uncertified execution backend; certify the host/image as additional assurance when useful |
| Protect the kernel through a special narrow source discipline | Use ordinary module source and one executable contract across supported backends |

Retained: pending claims are not proved facts; executable behavior must not depend on fabricated evidence; exact artifact/statement binding matters; logical rule changes are reviewed separately from runtime changes; self-checking is not a consistency proof.

The user's declared priorities are the premise for this revision: nested interpretation has been a serious productivity cost, vertical integration is valuable, and minimizing trusted supply-chain dependencies is not the current optimization target. This is not a new empirical claim about the software industry.

## 2. Execution backend, public engine, and logical authority

Use distinct terminology:

- **R:** the Rust execution backend: generic value operations, calls, allocation, loading/decoding, numeric operations, and explicit host services.
- **Engine:** the embeddable Shard library: canonical declarations, inspection/construction, contextual workspaces, execution, checking, and realization interfaces. `meta/` supplies reusable higher-level mechanisms against these surfaces.
- **K:** the engine's smaller logical authority, implemented in Shard. It establishes closed judgments under the foundation specification and exposes the native open-judgment boundary described in the foundation proposal.
- **S:** the executable-semantics contract that R and other execution paths implement. It does not dictate their internal value layout or performance.

Eventually **N** is a self-hosted native execution route. It may be compiled engine/library code rather than an interpreter. Its public semantics and identities must agree with the selected profile.

```text
Host application / CLI
    -> Shard engine and meta/ modules executed directly by R
    -> shared committed environment + branch-local open workspace
    -> check / inspect / transform / prepare / invoke / compile
```

Avoid the routine path in which R interprets a Shard evaluator that interprets each engine function. If a metaprogram intentionally evaluates a represented candidate, that is a separate explicit operation; prepared or compiled execution may realize it. Neither the logical comparison of proof terms nor intentional evaluation of code-as-data justifies an extra interpretation layer around all metaprogram execution.

### B13 — Embedding and retained invocation are first-class

Schematic public operations:

```text
open_workspace(checked_environment) -> Workspace
prepare(environment, entry, execution_profile) -> PreparedExecutable
invoke(prepared, runtime_arguments) -> ExecutionResult
validate_open(workspace, contextual_candidate, limits) -> OpenResult
commit(workspace_snapshot, declaration, limits) -> AdmissionResult
```

The foundation document owns the judgments behind these operations. This list describes the execution/embedding contract, not final API syntax.

A prepared handle binds the declaration, environment/realization version, runtime profile, and reusable preparation. It hides indexes and evaluator tables while preserving semantic inspection through independent views. Loading another module or changing a draft cannot silently retarget an existing handle.

The CLI is an adapter over these operations. Hosts and Shard `meta/` libraries need not print source files, invoke a subprocess, or reparse proof strings to call one another. Standalone Rust hosts may use a binding layer; its decoding, identity, lifetime, and failure semantics are part of the declared execution boundary.

---

## 3. Current implementation evidence

At the inspected baseline, `rust_bootstrap/src/bin/eval.rs` already provides `eval direct`: it loads an application's closure and executes its main function on the Rust evaluator rather than through `eval.shard`. Its comment identifies the restrictions as narrow syntax and flat imports, and explicitly discusses the cost of the additional interpretation layer. This revision extends that existing architectural direction; it does not claim the required module parity has already been implemented. [R1]

`rust_bootstrap/src/lib.rs` describes the Rust crate as an execution host, with proof-shaped work implemented in Shard. That is the separation to retain while replacing the narrow execution boundary. [R2]

The project also already has primitive-table conformance tests comparing Rust operations with their Shard counterparts. That is a useful starting pattern, but primitive tests alone do not establish whole-program or module-loading conformance. [R3]

Prepared invocation currently lives beside the main invocation interface because its EVM table dependencies cause an import-cycle problem for checker-hosted consumers. Sketch holes are specially named expression calls, with binder context maintained by library discipline. Those are concrete boundaries to improve, not features this revision assumes are already native. [R4, R5]

## 4. Certification on an uncertified backend

### B06 — Reviewed execution is a legitimate authority root

An uncertified implementation is not necessarily an unusable implementation. The current acceptance claim should identify:

1. the Shard rule specification and the Shard checker source;
2. the Rust backend, any bootstrap transformation, and relevant build/runtime dependencies used to execute that source;
3. the checked proposition, evidence, definitions, and allowed assumptions;
4. the target/environment premises and the connection to the actual output bytes.

Assuming the listed execution path faithfully runs the checker, successful checking establishes the requested judgment under K's rules. The project's confidence that K implements sound rules is a separate foundational/implementation obligation. A proof that R implements S is not a prerequisite for carrying out this check.

Do not call the host untrusted while relying on its result: it is **trusted, not yet formally verified**. Its existence need not appear as a new mathematical axiom in every theorem; it belongs in the execution provenance and TCB description. The logical assumption closure and the execution-trust manifest are different records.

### 4.1 What can be completed now

K running on R may check the complete compiler/refinement evidence for a target program, including the byte tie. That program is certified relative to the declared foundation, environment assumptions, and trusted checking execution. Its certificate need not be marked logically pending merely because R lacks its own correctness proof.

Lean's public TCB discussion likewise distinguishes checking mathematical evidence from additional implementation trust involved in running generated native code. The general lesson is to state the actual trust boundary, not to make every dependency prove itself before any theorem can be used. [P2]

### 4.2 What this does not claim

A Rust bug that changes K's behavior can invalidate an observed acceptance. A reference-versus-R differential is evidence, not a universal proof of equivalence. Replay on a later independently executed checker can improve confidence, but cannot retroactively turn an uncertified bootstrap into a proved one.

A compiled K' can be certified against K using K running on R. This is a useful assurance improvement and need not be a circular derivation: the original R/K execution remains the stated root. A checker accepting its own correctness statement does not establish global self-soundness.

The near-term threat model does not demand supply-chain minimality or a binary-free bootstrap. Accidental semantic disagreement and silent false acceptance still deserve strong testing because they directly undermine the product.

## 5. Rust execution scope

### B07 — Mirror executable capability, not logical authority

Rust may implement or optimize:

- explicit call frames, tail calls, recursion, and efficient value passing;
- constructors, pattern matching, immutable sharing, and any supported runtime closures;
- exact integers and specified word operations;
- arrays, buffers, regions, memory reclamation, and other representations under a defined interface;
- module/image loading and the host services needed by Shard tools;
- generic graph storage, stable handles, and snapshot transport used by native contextual workspaces;
- prepared execution and bulk-data interfaces used by embedding clients;
- a bytecode compiler or native execution path if measurements justify it;
- generic execution mechanisms useful to the checker, compiler, and ordinary programs alike.

Rust should not independently define:

- whether a theorem is valid;
- universe constraints or inductive admission;
- the logical conversion relation;
- what pending totality/refinement claims may be assumed;
- which axioms an artifact is permitted to use.

An optimized native implementation of a logical operation is not an ordinary runtime optimization merely because it is fast. It needs a separately justified equivalence contract and an explicit trust treatment. The default is to compile/execute K's implementation, not replace it by a Rust function keyed on a convenient source name.

There is no requirement that Rust execute arbitrary noncomputable mathematical objects. It must execute the supported implementation language in which K represents those objects as data. Runtime language features and subject-logic features therefore need not grow in lockstep.

### B14 — Native hole semantics stay in Shard

R can store and execute operations on the engine's term graphs, telescope records, contextual substitutions, assignments, and constraints. It does not independently decide which scope a hole may capture, which assignment is type-correct, or whether residual proof obligations have been discharged.

Those decisions are made by the Shard implementation of the native open-term services and K. A Rust acceleration of such a decision procedure would need an explicit correspondence/trust review, just as a second logical conversion implementation would.

Likewise, supporting native contextual holes does not require making an arbitrary unknown a runtime value of its expected type. Unresolved native holes are represented open syntax. Normal execution of a completed program rejects unresolved runtime dependencies. A separately requested partial-evaluation service can return residual syntax and blockers. Quoted syntax containing hole descriptions remains ordinary data and can be processed by a completed search program.

No Rust-only theorem status, hole solver, or compiler registry should be necessary for a `meta/` program to work. The maintained backend should make the shared Shard services practical, not replace them with a parallel language engine.

---

## 6. Module support without two language definitions

### B08 — Share resolved interfaces where economical; permit explicit mirrored parsing where needed

Preferred steady-state boundary:

```text
ordinary modular source
    -> Shard frontend running directly on R
    -> resolved module graph / execution image
    -> R loads the executable representation and calls its entry point
```

Resolve names, imports, visibility, interface/implementation binding, and source-to-declaration identity once. R need not reinterpret the frontend's policy at every call. The first frontend can be supplied as a generated bootstrap image or via an existing supported source path.

This is a preference, not a prohibition on expanding Rust's parser or resolver. A direct Rust implementation may be the fastest useful first slice. If so, make it a tested implementation of the same frontend contract: explicit qualified identity, duplicate handling, shadowing, opaque-interface behavior, and rejection behavior. Compare resolved artifacts independently from execution results. Track remaining duplication rather than silently retaining two subtly different name systems.

Do not force a permanently narrow kernel source dialect to avoid implementing normal module support. Do not build a new universal packaging system as a prerequisite to removing the present flat-import restriction.

The resolved artifact is a view of the canonical declaration graph, not a new public namespace. Frontend-to-runtime marshalling is explicit and paid during loading/preparation where possible, not repeated for every invocation. Raw code inspection and optimized execution must resolve the same declaration identity.

## 7. Pending inline claims and totality

### B09 — Development execution and theorem admission stay separate

R may run supported executable bodies before their source-level termination proofs or contracts have been checked. These are host implementation programs, not automatically admitted total functions in the subject logic.

K's committed environment contains only declarations admitted under K's rules and explicitly permitted assumptions. A workspace may contain pending definitions, native holes, and partial judgments, but cannot export them as checked facts. Running the checker implementation does not install its own intended correctness or termination claims in its subject environment.

Distinguish `OpenValid`, blocked constraints, ordinary execution results, and `Accepted(CheckedDeclaration)` in the API and serialized records. A proof hole of type P is an obligation, not a proof of P. Final admission expands the completed assignment and checks all reachable type/proof/declaration dependencies before runtime proof erasure. Explicitly generalizing holes into parameters creates a different theorem and cannot silently satisfy the original closed requirement.

Once K and its libraries are available, it can check the inline proofs attached to its source. That closes ordinary implementation obligations. Initial confidence in K still comes from the reviewed implementation and the stated execution trust, not solely its self-acceptance.

R must not silently treat pending proof annotations as authority for deleting checks, accepting invalid logical terms, inventing witnesses, or choosing supposedly unreachable behavior. For early code use conservative executable behavior or clearly delimited, reviewed erasure rules. Unverified bootstrap translation algorithms are permitted in the declared TCB; assuming missing user proofs is not the same thing.

Reference algorithms may use explicit budgets and return `Accepted`, `Rejected`, or `Exhausted`. Budgeting remains useful for robustness and to make later termination proofs tractable. It is not a reason to impose expensive interpreter towers or elaborate termination proofs before development execution can occur.

## 8. The non-divergence contract

### B10 — One specification, explicit support, no silent dialects

Every executable feature needs:

- a semantic contract and version;
- a frontend elaboration/representation rule;
- Rust support status;
- reference-model/test coverage;
- compiled-backend support status, when that backend exists;
- representative positive, negative, and boundary tests.

R may lead the compiled backend in coverage. This does not define a different language if the feature already has common semantics and the unsupported backend refuses it explicitly. Requiring a complete certified lowering before R can support a feature would recreate the original bootstrap blockage.

An experiment lacking agreed semantics remains an explicitly experimental capability, not part of the supported release profile.

### 8.1 What must agree

For the declared common profile and controlled environment, compare returned values, semantic failures, effect ordering, and other contract-relevant observations. For proof checking compare proposition/declaration identity, acceptance or rejection, and assumptions using the same K source and evidence. Compare successful runs at sufficient resources.

Concrete heap layouts, host pointer addresses, internal sharing decisions, wall-clock times, and total allocations need not match unless deliberately exposed by the relevant contract. R may use tracing collection while the native target uses counting. A word operation's overflow policy, however, is semantic. Exact Int computation must not silently turn into wrapping arithmetic.

A specified semantic budget is an input whose meaning must agree. An external timeout or an engine's physical OOM is a resource event, not a mathematical verdict. If one target allows a declared resource-failure family and another succeeds, record that difference rather than treating all such outcomes as identical or as false proof rejection.

### 8.2 Test both halves of the boundary

Use separate suites for:

1. **Frontend parity:** source to resolved modules and declarations.
2. **Execution parity:** identical resolved programs to values/traces/failures.
3. **Checker parity:** identical K and evidence on different execution paths.
4. **Independent semantic pins:** fixed expected results not generated solely by either engine under comparison.
5. **Embedding and workspace parity:** identical public operations over the same environment and assignments yield corresponding open judgments, committed declarations, and observable execution behavior; runtime-private node addresses are not compared.

Tests include large and negative integers, signed division and zero cases, nested patterns, simultaneous bindings, qualified names, shadowing, mutation/sharing rules where applicable, malformed runtime images, effect ordering, and rejection propagation. Checker inputs include valid and deliberately invalid certificates, holes, circular declarations, and malformed binders.

Native-hole cases additionally cover explicit substitutions under renamed binders, dependency changes in expected types, shared versus independent hole IDs, cycles through assignments/types, rollback and merge, incorrect use of a blocked result as a negative, and runtime code that legitimately manipulates quoted holes. Both engines must reject a native unresolved proof dependency at commit.

Use generated supported programs, counterexample shrinking, and bug-derived regression pins. Passing the suites is evidence of conformance, not proof that divergence is impossible. Shared frontend or primitive implementations create correlated blind spots; fixed independent cases and an appropriately separate reference path mitigate them.

### 8.3 No silent fallback

A normal command should disclose its selected execution engine and semantic version. If a fast path cannot handle a program, either reject clearly or use an explicitly selected fallback. Do not silently start a much slower evaluator tower and make ordinary CI latency unpredictable.

### B15 — Branch isolation and prepared caches are part of conformance

Share immutable environments and term storage, not mutable proof assignments. A workspace branch binds its own assignment/constraint snapshot. Merge is a checked proposal; equal hole display names or reused storage offsets cannot establish identity. New IDs are unique within their lineage or explicitly remapped.

Cache keys include the declaration/environment and execution profile for prepared code; contextual operations also include the applicable telescope, occurrence substitution, hole/universe assignments, and constraints. Coarse revision-keyed invalidation is a legitimate starting design. Optimizing it to dependency-sensitive invalidation is separate work with adversarial tests.

No partial result is stored as a closed theorem receipt. A pending inline proof and an unchanged runtime body may share prepared execution, but cannot share an old acceptance receipt whose logical dependencies changed. Commit reads a frozen snapshot even when other agents continue editing.

These invariants matter on one workstation without a hostile supply chain: a stale successful check or leaked branch assignment is already enough to report an incorrect result.

---

## 9. CI should guard semantics without reinstating the interpretation tax

### B11 — Fast authority by default; reference checks targeted to their purpose

Normal development and proof CI may run K directly on the supported Rust backend. This is an acceptance-grade route under the stated TCB, not a permanently second-class development convenience.

Proposed scheduling:

- Every relevant change: fast proof corpus, runtime conformance, negative fixtures, and targeted performance budgets.
- Changes to execution semantics, primitive dispatch, frontend identity, or runtime representations: broader differential tests and the affected reference cases before merge.
- Periodic/release validation: broader reference replay, engine comparisons where supported, and cold bootstrap from source.

The exact cadence should follow measured cost and bug risk. No universal wall-clock target is asserted here.

Measure cold load/elaboration, warm checking, small-edit feedback, peak memory, and time spent executing K versus interpreting an interpreter. Also count environment preparations per repeated invocation, proof-text round trips in search, and invalidated workspace nodes after a small assignment. Retain the full tower for regression investigation and selected reference testing, not as a mandatory per-commit toll for every certificate.

## 10. Build and rollout

### B12 — Make Rust-hosted certification useful before certifying Rust

1. Generalize the existing direct execution path enough for the new checker to use ordinary modules.
2. Establish the execution-semantic profile and conformance harness at the same time.
3. Implement the Shard checker and minimal canonical open-workspace services; run their tests on R without requiring the successor foundation to justify its own source beforehand. Port one native-hole sketch/search consumer while those interfaces can still change.
4. Adopt the reviewed K/R snapshot as the explicitly described proof-checking authority; recheck the new libraries, compiler evidence, and required kernel-source obligations.
5. Complete useful target artifact certificates on this path, including an embedding client that prepares, searches/transforms, checks, compiles, and repeatedly invokes through the shared library interfaces.
6. Add native/self-hosted execution and certified checker-image realization when coverage and economics justify them. Keep R as a cold-build and comparison backend.

Certification of R itself is optional additional assurance, not a prerequisite or a promised final project. The long-term maintenance target can narrow to the cold-build closure if maintaining full application-execution parity ceases to be useful. That decision is separate from shrinking R during current development.

A source-buildable implementation in another language is a valuable product choice, not a logical necessity for every self-hosted language: Rust normally bootstraps from an earlier compiler binary. The objective here is a practical fresh-machine route from Rust/Cargo and repository sources, without a chain of historical Shard binaries. [P1]

The project need not recreate historical stages once R can directly execute current toolchain sources or their mechanically regenerated images.

### B16 — Runtime linking of the engine is intentional, optional, and bounded

An ML runtime or other application may link the Shard engine and `meta/` services to compile newly supplied programs after deployment. This is not an implicit runtime requirement on every Shard artifact. Generated code may omit the engine entirely.

A host adapter supplies explicitly named capabilities for file access, buffer access, code installation, or other effects. Bulk arrays/tensors are not automatically expanded into expression-literal trees. State buffer ownership, lifetime, mutation, shape, and any numeric interpretation at that interface. Logical checking must not observe mutable host data as a constant without an appropriate frozen representation or explicit assumption.

The canonical service contract is the same whether the engine is invoked from a CLI, another Shard module, or a host-language binding. Rust-backed execution remains permitted, with the same trust accounting; crossing an embedding boundary does not change a speculative judgment into a theorem or add permission for a hidden evaluator tower.

---

## 11. Provenance without unnecessary supply-chain machinery

Record a compact manifest sufficient to reproduce and debug a verdict:

```text
foundation/rule version
checker source and loaded-module identity
execution semantics/profile version
execution backend build identity
required source/evidence/statement identities
logical assumption closure
artifact bytes and target/environment profile, where applicable
```

Open-workspace records additionally carry the base environment, hole/context identities, assignment lineage, constraint/policy identities, and whether they are merely drafts or validated partial judgments. Open-state serialization is not a theorem certificate.

Engine caches must include the relevant runtime semantics and checker identity. Changing a pending proof, runtime implementation, or referenced definition must invalidate the appropriate entries. Do not recompute a build stamp from newly edited source and attach it to a binary built from an older snapshot; use an immutable input snapshot or detect input changes during the build.

Full supply-chain attestations and a minimal assembly bootstrap are outside this proposal. Statement identity and faithful runtime behavior remain in scope because they are necessary for the product to mean what it reports, regardless of adversarial threat model.

## 12. First acceptance gates

The following are proposed experiments, not completed tests:

**G0 — Direct modular checker:** The new checker uses ordinary modules and executes directly on R, with no Shard evaluator interpreting each checker call. Source identity and interface binding match the common contract.

**G1 — Primitive and frontend parity:** Extend the existing primitive conformance pattern to whole resolved modules; include known binding/name hazards and independent expected results.

**G2 — Evidence isolation:** False inline claims may coexist with explicitly provisional execution but never become accepted facts. Fully supplied artifact evidence can be checked to completion on R without a proof of R's own implementation.

**G3 — Invalid input parity:** The same bad certificates fail on both supported execution paths. Engine crashes and exhaustion yield no successful verdict.

**G4 — Useful artifact:** Build one byte-tied compiled program and complete its required proof chain using Rust-hosted K. The result records its execution TCB rather than being labeled incomplete merely because R is uncertified.

**G5 — Cost:** Compare cold/warm checker execution and small-edit feedback against the current tower and direct path. Do not infer a speedup from the architecture alone.

**G6 — Fresh build:** Produce the operational checker from a clean checkout and declared Rust toolchain without requiring an earlier Shard executable. Generated images, when used, have a regeneration route.

**G7 — Public prepared embedding:** Construct a declaration in memory, prepare once, invoke repeatedly, inspect its original identity, and request a checked realization without source files, CLI processes, or evaluator-private tables. A bulk-data adapter is explicit. Record actual preparation counts and data-conversion cost.

**G8 — Native contextual workspace parity:** Validate and fill one hole under two explicitly related binder contexts. Preserve a shared choice and keep same-typed distinct holes independent. Reject scope escape, indirect cycles, stale expected types, and committing an unresolved proof. Compare semantic results across supported execution paths, not private heap addresses.

**G9 — Branch and judgment isolation:** Fork conflicting assignments, roll back, merge only by validated reconciliation, and verify that caches do not leak outcomes across branches. `Blocked` never becomes `UNSAT`; `OpenValid` never becomes `Accepted`; a quoted hole remains legal program data.

**G10 — In-process search/compilation:** A `meta/` consumer builds candidates, obtains theorem licenses, constructs evidence, checks it, and compiles a selected candidate through the canonical environment. Candidate generation remains outside K, and certification of one candidate does not imply exhaustive search or optimality.

## 13. Decisions for review with Fable

| ID | Proposed decision |
|---|---|
| B06 | A reviewed but uncertified Rust host may perform normal authoritative checking of Shard evidence |
| B07 | Expand executable capability in Rust; keep the logical checker and its rules in Shard |
| B08 | Share resolved frontend artifacts where economical; test any duplicated parsing/resolution explicitly |
| B09 | Allow provisional implementation execution without admitting pending claims or recursive equations |
| B10 | Govern parity through semantic versions, feature coverage, and multi-layer conformance tests |
| B11 | Make fast direct execution the CI default; retain costly reference paths for targeted assurance |
| B12 | Maintain a source-buildable Rust route; certify target artifacts before making host certification a priority |
| B13 | Provide the complete embeddable engine and public retained-execution interfaces |
| B14 | Execute native contextual-hole services in Shard; do not create Rust-specific assignment authority |
| B15 | Include workspace isolation, semantic cache keys, and immutable commit snapshots in conformance |
| B16 | Support explicit runtime linking of engine/meta services and explicit bulk-data/host boundaries |

Open implementation questions: which immediate restriction blocks direct modular K; which current execution representation best supports a one-time module handoff; whether a bytecode evaluator beats the existing Rust evaluator for K; and which minimal independent reference cases catch the most likely runtime divergences. These are measured implementation choices, not reasons to delay agreeing the overall trust and execution contract. Also decide the smallest public prepared handle that removes the current import-cycle pressure, and the minimal native-hole operations required by the first `meta/sketch` migration. Their logical contracts are owned by foundation Sections 4.8 and 13.

**Bottom line:** keep one Shard-written logical authority inside a canonical embeddable engine, with shared contextual syntax and reusable `meta/` services. Allow multiple faithful ways to execute those services. Rust is an ordinary, efficient, explicitly trusted execution route for completed certification now; its flexibility must not create a second theorem system, a second hole semantics, or a hidden interpreter tower.

## References

Repository descriptions are source observations, not independently reproduced performance results. Primary external references are engineering precedents, not proofs of this proposal.

- **[R1] Direct execution and current restrictions:**  
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/rust_bootstrap/src/bin/eval.rs>
- **[R2] Bootstrap execution contract and current test loader:**  
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/rust_bootstrap/src/lib.rs>
- **[R3] Primitive conformance and current trust accounting:**  
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/docs/TCB.md>  
  <https://github.com/computer-whisperer/shard/blob/82c00e9b4e2e35eda689c632d3240a81be2429bd/rust_bootstrap/src/prim.rs>
- **[P1] Rust compiler development guide, stages of bootstrapping:**  
  <https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html>
- **[P2] Lean FAQ, trusted code base and native execution:**  
  <https://lean-lang.org/faq/>
- **[R4] Retained pure invocation and public-interface dependency pressure:**
  <https://github.com/computer-whisperer/shard/blob/5abc60074f260f882a92a563f5c9d8fcd891199a/meta/invoke/prepared.shard>
- **[R5] Existing sketch-hole encoding and binder-context contract:**
  <https://github.com/computer-whisperer/shard/blob/5abc60074f260f882a92a563f5c9d8fcd891199a/meta/sketch/mod.req.shard>
- **Related architecture:** [Foundation proposal v0.3](SHARD_FOUNDATION_PROPOSAL_v0.3.md), especially Sections 3, 4.8, 9.4, and 13.
- **Historical discussion artifacts:** `SHARD_BOOTSTRAP_ADDENDUM_v0.1.md` and `SHARD_BOOTSTRAP_ADDENDUM_v0.2.md`, superseded by this revision. External P1–P2 references are retained from the earlier draft, not newly reverified in v0.3.
