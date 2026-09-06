# Shard V2: integration review before implementation

## One engine, richer evidence, and boundaries that survive the rewrite

**Status:** REVIEW MEMO v0.1 — proposed amendments for discussion, not ratified rules.  
**Date:** September 6, 2026.  
**To:** Christian Balcom and Claude Fable.  
**From:** GPT-6 Astra Pro.  
**Reviewed baseline:** `docs/FOUNDATION.md`, Fable DRAFT v0.4, at commit `23094d7a3d9b00d33cc10f734d54a8a75f7205df`; blob `8a0f053cd18b0a03842944a8374883a2fe541370`. The branch was checked at the start of this review and pointed to that revision. [F]  
**Relationship:** Continues the R1–R8 feedback and R9–R16 follow-up. R17–R28 below address the new proof IR, naming rule, and interactions exposed by an additional cross-layer review. This memo is not another proposal to replace the agreed foundation.  
**Evidence boundary:** The current GitHub revision is the subject, not the earlier uploaded DRAFT v0.1. Repository descriptions below identify their sources and sections. Mathematical examples, inferred failure modes, suggested interfaces, and tests are proposals or deductions—not reported Shard exploits. No V2 implementation, new proof, performance benchmark, or complete soundness audit was performed. External references check particular technical facts; they do not validate this unimplemented architecture.

---

## 0. Recommendation

Proceed with the rewrite. Keep K written in E, L represented as data, the initially reference-exact closed logic, compiled applications, static lambda elimination, the Rust bootstrap route, and the sibling `v2/` development plan. Keep the naming law and I, the navigable proof IR. They make Shard a more unified system, not merely an old programming language connected to a stronger prover.

The additional review does not reveal a reason to choose a different foundation. It does reveal several interfaces that should be settled before large amounts of code depend on them. The largest opportunities are:

1. **Give executable eligibility a phase/relevance account.** Separate compile-time structure, erased evidence, and runtime data. This accommodates `Decidable`, static operation packages, and useful proof-bearing values without residual closures or a fully dependent runtime.
2. **Attach executable realizations to canonical logical declarations.** Imported and native mathematics should not require duplicate definitions merely because one originally arrived as a `def` rather than a `fn`.
3. **Make I an explicit derivation graph, not an implicitly stateful list of tactic-like instructions.** Retain I for construction and search, and retain P for independent checking of accepted releases.
4. **Make boundary invariants real.** A hash does not prove a translation correct, a ghost invariant is not established by decoding raw bytes, a World token is not automatically linear, and failure of one proof is not failure of the program.

These are primarily elaboration, representation, library, and acceptance contracts. They do not require adding new trusted logical principles to K. Some need a decision now; their implementations belong in the existing phases rather than in twelve new projects.

### 0.1 Decisions requested

| ID | Amendment | Main locations in FOUNDATION.md | When needed |
|---|---|---|---|
| **R17** | Classify static structure, erased evidence, and runtime data; allow arbitrary propositional ghost invariants over supported carriers | §§2, 5.1–5.3, 5.6, 6 | Specify before freezing E; test in phases 2–3 |
| **R18** | Provide a checked executable attachment for an existing logical declaration; define the native/imported library identity policy | §§2, 4.1, 5, 6.4, Q7 | Before the first shared library/import integration |
| **R19** | Make the naming migration typed and relation-aware, not uniformly a rename | §§6.4, 12, 14.3 | Before tier-0 migration touches semantic cases |
| **R20** | Bind the actual executable structure to its logical meaning through evidence, not only matching provenance | §§2, 5.4, 5.6, T1/T8 | Before certified realization is claimed |
| **R21** | Specify I replay completely and keep deterministic reconstruction separate from discovery | §16.1–16.3, Q2 | Before publishing I version 1 |
| **R22** | Give goal dependencies, hole kinds, and refutation scope explicit representations | §§3.3, 7, 16.3–16.4 | Minimal form with the first I/search implementation |
| **R23** | Retain both I and P for accepted releases; separate canonical encodings, versions, and dependency classes | §§4.1, 13.2, 16.5 | First persistent evidence format |
| **R24** | Separate physical paths, nominal interfaces, declaration revisions, and implementation choices | §§4.1, 6.4, 8, 14 | Before `v2/` identities and module views harden |
| **R25** | Audit the pure/effect boundary and World model; explicit threading alone is not a proof of faithful effects | §§4.1, 5.4, 5.7, 6.4; BOUNDARIES.md | Before porting effectful artifact guarantees |
| **R26** | Validate erased invariants at embedding/ABI boundaries; make prepared handles own a precise snapshot and representation contract | D09/D10, T6, §5.6 | First embedding pathfinder |
| **R27** | Budget and measure the whole engine; benchmark reflection and require reclamation for long-lived contexts | §§3.3, 5.4, 10, 14 | Instrument from bring-up; optimize from measurements |
| **R28** | Validate programmatically constructed declarations and checked environments, not just source-generated inputs | §§3–4, T0/T8 | Initial K and admission APIs |

**Priority rule:** agree the meanings and small counterexamples now. Do not require every general theorem, performance optimization, or extension to be finished before the first checker runs.

### 0.2 What is already resolved

The v0.4 separation of declarative rules from the bounded checker is the correct direction. The transactional workspace contract, resolved requirements, assumption-policy boundary, explicit replacement relations, and recorded Rust execution trust should remain. The proposed `rfl` check of a fuelled `ev` run supplies an actual per-invocation evidence route; the older raw-native-result objection is no longer the right objection to that route. Its general correctness, binding, and cost still need work. [F, §§3.1, 5.4, 5.7, 6.2, 7–8]

Do not spend the next iteration restating those agreements. Use the memo to close the newly exposed seams.

---

## 1. R17 — A phase-aware E is a better boundary than a blanket syntactic ban

### 1.1 The new naming rule exposes an existing tension

The draft permits proposition-aware `if` and `decide`, erased proof arguments, subtypes, static templates, and eventually law-bearing typeclasses. It also excludes all function-typed fields, runtime propositions, and indexed inductives, and restricts subtype predicates to E functions. These restrictions are useful approximations, but do not yet state a uniform executable criterion. [F, §§5.1–5.3, 5.6, 6.1]

For example, a decision procedure returns a `Decidable P`, not a proof of P and not simply an untyped Boolean. Its constructor choice is computational data; the constructor contains evidence of P or of its negation. Lean's definition deliberately lives in `Type`, and its dependent conditional exposes branch-local evidence. [X1]

Erasing the entire decision value because it contains proofs would erase the branch decision. Treating its proof payload as a runtime field would retain unnecessary machinery. A precise bridge keeps the tag, erases the proof payload, and validates the evidence supporting the selected branch.

### 1.2 Specify three roles

A supported elaboration should classify an input, field, or intermediate value by its role:

| Role | Examples | Treatment |
|---|---|---|
| **Static structure** | Type arguments, known function identities, selected operation packages | Resolve/specialize where needed before execution; require finite supported specialization |
| **Erased evidence** | A bound proof, a postcondition witness in Prop, an algebraic law | Check it in L; erase it under the specified translation; retain logical provenance |
| **Runtime data** | Integers, buffers, constructor tags, captured values, a decision outcome | Preserve it through an explicit representation; account for resources |

These are roles, not three new mathematical sorts. A value can contain fields with different roles. A type parameter used only for checking may erase rather than cause specialization; one that determines layout may require specialization or an explicit representation parameter. Do not use this table as permission for an unproved erasure pass.

The initial K and toolchain can remain in the simple first-order E implementation profile. This classification primarily improves the source-to-E boundary. It does not make L's function spaces native Rust values.

### 1.3 Add arbitrary propositional ghost invariants

**Recommended language improvement:** permit a supported E carrier A to be refined by an arbitrary well-formed L proposition `P x`, provided the proof is erased and runtime behavior has a valid realization:

```text
{x : A // P x}
```

A decision procedure for `P` is not needed merely to construct such a value using an existing proof or to use its carrier. It is needed when the program promises to decide membership at runtime. The distinction is fundamental, not an optimization. Lean's subtype interface accepts a predicate into Prop and separates the carrier from its proof field. [X2]

A useful Shard example is:

```text
CertifiedProgram(spec) = {p : Program // Realizes(p, spec)}
```

`Program` can be ordinary E data. `Realizes` may quantify over all inputs and have no practical decision procedure. A generator returns concrete program data and a theorem; a consumer executes or lowers that data under the stated contract. Requiring `Realizes` itself to be an E-decided predicate would exclude precisely the proof-bearing values this rewrite should make easy.

This is not unrestricted extraction from `Exists p, Realizes(p, spec)`. The subtype contains an actual supplied carrier. It is also not permission to trust an arbitrary host-constructed `CertifiedProgram`; see R26.

A conservative first implementation can recognize a small erasure class—supported carrier plus Prop-only fields—without implementing general indexed runtime representations. The hypothesis that this remains inexpensive must be tested, not asserted.

### 1.4 Support branch-local evidence without runtime proofs

A small motivating example is:

```text
if h : i < length(xs)
then get(xs, i, h)
else none
```

The program computes a decision once. In the true branch, the proof obligation for access is discharged by h. The runtime representation of h disappears. This should not require an independently computed second bounds check or a general runtime closure for the branch.

Specify short-circuiting and evaluation order for conditional/composite decisions. In particular, retaining one mathematical meaning does not by itself specify when a decision procedure is called, which branch executes, or what work is duplicated.

The current blanket statement that E never eliminates Prop into data should be sharpened: the supported lowering erases proof content while accounting for legitimate logical transports and impossible branches. Eliminating `Decidable P` is elimination from a data type with a tag, not unrestricted extraction from P. Equality transport and `False.elim` need their stated special treatment; a missing proof cannot justify either.

### 1.5 Static law-bearing packages should fit the same model

A generic algebraic package may contain an operation of function type and a proof of associativity. Those are valid L fields even if a runtime E record cannot contain a function value.

For a statically selected package, specialize its operation projections and erase its laws. Keep ordinary value parameters at runtime. This is the package analogue of the accepted lambda/template profile; it needs no dynamic dictionary dispatch.

Without this rule, typeclasses and mathematical structures may work in proofs but fail at the point a generic E algorithm tries to use them. Do not force every library to unpack its laws into an ad hoc parallel interface.

**Small gate:** one generic fold and its theorem, two statically selected operation packages, a captured runtime value, and one arbitrary-Prop refined result. Require a first-order realization, no runtime proof fields, no function-pointer dictionary, and reuse of the generic theorem. Keep unsupported dynamic packages a clear refusal.

### 1.6 Proof as justification is different from proof as data

An L proof used only to justify an invariant may erase. An E value containing the syntax of an I derivation or a P term is ordinary runtime data: K and search tools must be able to inspect it. The fact that the data describes a proposition or proof does not put the data itself in Prop.

Likewise, a compiled generator whose result type carries a ghost correctness proof does not automatically emit a portable proof certificate. To transfer that result across a checking boundary, retain quoted evidence as data or instantiate the generator's correctness theorem together with evidence for the particular execution. These are complementary ways to exploit knowledge, not interchangeable objects.

Inspection applies to explicit syntax/evidence artifacts. Do not introduce an unrestricted L function that takes any proof of P and returns its distinctive syntax; that would conflict with treating proofs as irrelevant in the logic. The ordinary E implementation may freely inspect the represented proof terms supplied to it as data.

---

## 2. R18 — One mathematical vocabulary needs an executable-attachment story

### 2.1 The import boundary is currently underspecified

The draft says `def` never lowers, `fn` expresses executable intent, imported mathematics arrives as L declarations, and the same operation has one name everywhere. It does not yet specify how an imported declaration becomes usable by E without being redefined. [F, §§2, 4.1, 5.3, 6.4, Q7]

For example, importing a logical `List.length` does not import Shard's `fn` annotation or an E body view. Declaring another `List.length` with a conveniently compilable body risks either a duplicate identity or an unjustified identification of two definitions. Matching spellings is not the bridge.

### 2.2 Recommended operation

Allow an existing checked L declaration to acquire a checked executable attachment under its existing logical identity:

```text
register_realization(
    logical_declaration,
    executable_body,
    correspondence_evidence,
    execution_profile
)
```

This is schematic, not proposed surface syntax. The attachment does not modify the logical declaration or declare its body equal to anything new by fiat. It establishes that a particular executable structure realizes it under a particular profile.

There are two normal cases:

- A supported executable view is derived from the existing logical definition and the corresponding relationship is checked.
- A separately supplied E implementation is related to that definition by a theorem.

`fn` can remain the convenient way to author a declaration and request this attachment together. An imported or mathematical `def` remains non-executable by default; it does not silently gain a compiler guarantee. Explicitly registering a realization is not an interpreted-only `fn` and is not a second mathematical name.

This also provides a natural registry for multiple target implementations without redefining the specification.

Keep meaning selection distinct from realization selection. Choosing a different ordering can change the mathematical requirement. Choosing another certified implementation of the same fixed operation may change only execution cost, resource assumptions, or the target realization. Record those choices in the appropriate records instead of declaring every implementation change a new mathematical task. A fixed library decision instance can still have an execution dependency whose version matters.

### 2.3 Decide which standard-library identities are canonical

Before broad import, choose whether the first common library reuses a selected exported declaration closure or defines a native equivalent and maintains an explicit import mapping. Both routes are possible; the mapping work differs.

Require exact handling of universe parameters, declaration dependencies, nominal types, opacity, and theorem assumptions. Two equal-looking ADTs are not interchangeable merely because they serialize similarly. If their definitions differ, use an explicit interpretation or isomorphism rather than name matching.

Imported proof evidence may add another proof of an already represented proposition without replacing its statement identity. Conversely, a theorem with the same name but a different statement is a conflict, not another proof.

### 2.4 Put refusal policies at the appropriate phase

Refusing implicit `Inhabited` fallback in an E API need not prohibit the L structure `Inhabited` or all imported proofs mentioning it. Refusing monadic effect notation in E need not prohibit mathematical definitions of monads in L. Refusing an unchecked foreign implementation does not prohibit the pure theorem describing one.

The short Lean-to-Shard guide should identify whether a refusal concerns source sugar, E eligibility, a deployment profile, or logical/assumption policy. Otherwise an application-facing convention can accidentally defeat the stated mathematical-parity test.

**Small gate:** import or construct one canonical logical list operation, attach an E realization without redeclaration, invoke it under the same identity from a `fn` and a theorem, and reject an inconsistent duplicate. Include one permitted L-only abstraction that is deliberately ineligible for E.

---

## 3. R19 — Make migration typed, and keep genuinely different relations different

### 3.1 Three classes of change

The naming law is useful. The repeated claim that every affected operation is a mechanical rename is too strong. The draft's own exceptional-behavior table records actual semantic changes. [F, §§6.4, 12.1, 14.3]

| Class | Example | Required record |
|---|---|---|
| Name only | `Some` to `some` | Resolution-preserving rename |
| Type/representation | Int length to Nat length; Bool predicate to Prop plus a decision | Typed transformation and a correspondence |
| Deliberate behavior change | Old stuck/trapping zero division to a defined mathematical value | An explicitly approved contract change and affected-use review |

Automation can perform transformations in every class. The label must describe what the transformation actually does.

For example, `remaining = capacity - used` with capacity 3 and used 5 gives a negative integer in the old representation and zero with Nat subtraction. An old `remaining < 0` error branch does not survive blanket replacement. A signed difference is not a size merely because both operands are sizes. The migrated code may need `used > capacity`, a non-underflow proof, a checked operation, or an intentionally signed result.

Likewise, a Boolean-returning function cannot replace `lt x 10` with a bare proposition and keep its return type. It needs the relevant `decide` bridge. A boolean stored in data and a proposition used as an `if` condition have different elaboration contexts.

Keep the deliberate Euclidean/zero-divisor convention. But note that using a totalized primitive does not itself create a nonzero-divisor obligation. The stronger condition must come from a requirement, a preconditioned operation, or an optional warning about the intended use.

### 3.2 Equality deserves an explicit relation table

One name per mathematical object must not become one equality for every purpose. Distinguish at least:

```text
syntactic identity of a term
mathematical equality of its denotation
an executable Boolean comparison
bit-pattern equality
an observational equivalence
an approximate relation
```

A concrete stress test is floating point. IEEE comparison is not reflexive at NaN; logical equality is. Bitwise identity can distinguish signed zeros even when a numerical comparison treats them alike. Approximate closeness with a fixed tolerance need not be transitive and must not silently become a congruence or a hash-table key relation. The distinction between logical and IEEE-style comparison is also explicit in the reference floating-point documentation. [X7]

Lean 4.33 introduced non-opaque Float models and explicitly distinguishes its bit-pattern `DecidableEq` from IEEE `==`. This is a technical comparison, not a recommendation to replace Shard's float model. [X3] Shard's own FLOATS.md already discusses that model while keeping its own format and observation choices; FOUNDATION.md's remaining contrast with an opaque Lean Float should be updated. [FL; F, §12.1]

In particular, imported Float theorems must not be aliased onto Shard's model merely by sharing the name `Float` if the representations or NaN observation rules differ.

### 3.3 The machine port is also affected

Under the selected mathematical division convention, a target instruction that traps on zero cannot be emitted unconditionally for an input domain that includes zero. Either prove the guard, emit the required alternate path, or reject that realization. A signed machine-division overflow case likewise needs an explicit range argument or permitted implementation behavior.

The compiler's old trap behavior is therefore not always an unchanged lower rung. Its mathematics can remain reusable while a particular instruction-selection premise changes.

**Tests:** Nat-underflow-sensitive control flow; a negative stride or offset that must remain Int; a comparator returned as Bool; zero-divisor execution on a target model; NaN and signed-zero comparisons; and a purported approximate-equivalence rewrite that fails transitivity. Record which cases preserve old behavior and which deliberately change it.

---

## 4. R20 — Provenance is not the L/E correspondence theorem

### 4.1 What v0.4 has repaired

The new evaluation-reflection path is meaningful: K checks an equality about the particular fuelled `ev` call, and a correspondence theorem connects that computation to the desired logical result. This is not the earlier proposal to turn a native value directly into a theorem. Keep it as a reference mechanism. [F, §5.4]

The remaining problem is the correspondence theorem's domain and premises. A theorem about executing p must identify why p realizes the logical declaration f. That relationship cannot be supplied solely by a manifest saying that both were generated from the same source hash.

### 4.2 A fresh, internally consistent manifest can still describe a bad translation

Consider an L definition `f x = x + 1`, with a valid equation theorem. A defective executable-view generator associates f with a body returning `x + 2` and writes perfectly fresh hashes for all artifacts. There is no stale file in this example. The generator made a semantic error.

Checking the equation of f and checking the hashes are both useful. Neither proves that the executable body corresponds to that equation. Similarly, co-generating the body and its alleged certificate can produce a consistent mistake unless the certificate's meaning is independently checked.

**Requested contract:** the realization evidence or its validated construction must mention the actual resolved executable structure, not just the source identity from which a tool claims to have derived it.

A schematic successful-run theorem is:

```text
ExecutableBundleRelated(logical_environment, executable_bundle, realization_map)
and InputsRelated(entry, logical_inputs, runtime_inputs)
and ev(executable_bundle, entry, runtime_inputs, fuel) = Some runtime_result
imply OutputsRelated(runtime_result, logical_meaning(entry, logical_inputs)).
```

The bundle relation accounts for the covered equations, types, callees, constructor representations, and recursion. Effects require the appropriate trace relation, not the pure result theorem. Progress under adequate resources remains separate.

This need not be a universal truth predicate for arbitrary L terms. It can be a family of per-declaration realization theorems or a theorem about a restricted typed E semantics with explicit semantic parameters. No general self-soundness oracle is requested.

### 4.3 The executable view may remain derived

I accept keeping the executable view as a derived view rather than a second hand-maintained source or large stored duplicate. The requirement is semantic, not physical.

For straightforward functions, a validated translation rule or a small reflection procedure may establish the relation automatically. For measure recursion, the checked equation family and a well-founded execution argument may do the work. A compiler may reuse a general result rather than emit a trace for every input.

But the schema must cover the actual cases. A true equation `f x = f x` does not establish progress of a looping implementation. A collection of correct equations that omits one executable branch does not justify that branch. Mutually recursive call groups need a joint well-founded construction rather than mutually assumed correctness claims.

### 4.4 Tests that distinguish correctness from freshness

Extend T1/T8 with a deliberately wrong generator output whose hashes are all fresh, an omitted pattern branch, a wrong callee binding, a stale realization attached to a new implementation, and a tautological equation offered for a nonterminating executable recursion.

The first test is intentionally different from the draft's manifest-drift test. A hash check should pass while the semantic acceptance gate fails. The accepted bootstrap may initially trust a named translation component; that status must not be described as a completed application-realization proof.

---

## 5. R21 — Specify I as a reconstruction format, not merely deterministic tactics

### 5.1 Deterministic does not mean discovery-free

I is a good addition. Keep its separation from both search policy and P. However, `simp_only(lemmas, budget)` does not by itself describe every choice a simplifier can make: traversal, side-condition discharge, contextual hypotheses, reduction policy, and lemma orientation can matter. The official Lean simplifier exposes several such controls. This is a warning to specify Shard's own smaller contract, not a requirement to reproduce all those controls. [X4]

Likewise, `apply` may leave instantiation choices; an induction step needs a motive, generalized variables, and a recursor; a positional rewrite needs a well-defined target occurrence. Calling the operation deterministic without pinning those conventions can hide a version-sensitive tactic engine inside I replay.

**Recommended rule:** each I node either carries its consequential choices explicitly or names a versioned, bounded reconstruction algorithm with a fixed input and dependency set. Reconstruction may compute and match. It must not silently consult a changing global instance database, discover new lemmas, or run an unrecorded search procedure.

A closed replay node should fix the relevant goal/context, declaration identities, instantiations, branch/motive information, and local reconstruction configuration. Not every node needs every field. Common conventions can be fixed by the I schema rather than repeated in the file.

### 5.2 Version I independently from the foundation

Distinguish the logical rule version, I schema/reconstruction version, canonical P encoding, and tactic implementation version. A new certificate constructor does not necessarily change mathematical truth. A different tactic need not invalidate a closed I or P artifact.

If replay under a new reconstruction version produces a different valid P, report that as a reconstruction/evidence change. A hash mismatch is not itself a proof that the old or new theorem is false. Do not silently overwrite the old evidence pin, either. Explicit migration can retain the statement, recheck assumptions, and record the new proof.

### 5.3 Use graph structure and stable references

Represent I as a derivation graph with explicit child-goal references, local binding identities, and reconstructed P boundaries. Names such as `ih` are display aids; local references need scope-safe identity. A rewrite location should be a defined occurrence/path in a particular term view, preferably guarded by the expected subterm/context, rather than whichever current pretty-printed occurrence happens to have index 3.

A linear script can remain an authoring view and `goal_of(prefix)` a convenient API. The engine should not require replaying the entire prefix for every search action. Otherwise an n-step interactive proof can accumulate avoidable quadratic reconstruction work before its actual mathematical cost is considered.

The draft's `step` returns subgoals and a term builder. Since the toolchain is E, specify that builder as data—a proof skeleton with scoped references or a defunctionalized construction record—not an implicit captured host-language callback. This keeps the execution model honest while retaining compositional proof building.

### 5.4 Extensibility without a second trusted kernel

Begin with a minimal I kernel of structural operations, evidence references, and checked library applications. Add domain certificates through versioned reconstruction adapters that produce P. An adapter may be imperfect or fail; its successful result must still be checked against the fixed target by K.

The `exact` escape preserves mathematical reach, but it does not guarantee that every proof has a compact or searchable I form. Measure the quality of the intermediate representation on real proofs, and allow an optimizer or importer that already produces P to wrap/reference that proof rather than reverse-engineer a tactic-like derivation.

Do not make the entire old proof roster a completeness checklist for the new IR. Keep the operations that express useful structure; merge redundant historical forms when the new logic makes their distinction unnecessary.

**Tests:** replay after changing ambient simp and instance settings; an induction with an explicitly dependent motive; local names reused in sibling branches; a guarded occurrence whose source term changed; and an external P producer accepted through `exact` without re-running a search tactic.

---

## 6. R22 — Share hole infrastructure without conflating proof terms, recipes, or refutations

### 6.1 A goal state is not simply a type

The engine can share IDs, telescopes, substitutions, transactions, and dependency tracking between L and I. Nevertheless, a missing L term and a missing I derivation have different validation judgments. Both being E data is not the argument that their holes have identical meaning. [F, §§7, 16.4]

Use a common contextual infrastructure with explicit sorts of obligation. An I proof hole can project to an L proof obligation in a declared telescope: a filling is an I fragment whose elaboration supplies the required L term. A hole standing for a whole proof strategy is a different object with its own input/output contract.

For example, `rfl` can close both `0 = 0` and `1 = 1`. Sharing that recipe is not sharing one closed proof term. A single contextual proof template can instead be declared as:

```text
?h : [n : Nat ⊢ n = n]
```

and used at n = 0 and n = 1 through explicit substitutions. These are three distinguishable relationships: identical proof, one contextual proof template, and one goal-polymorphic recipe. Pick the intended relationship explicitly.

### 6.2 Subgoals have dependencies

A witness hole for `exists n, P n` affects the proposition of the accompanying proof hole. A type hole can affect many later term holes. Sharing a proof hole can constrain a program hole and vice versa.

The goal graph must retain those dependencies, assign one owner to each unresolved obligation, and invalidate or reconsider affected checks after a committed assignment. It cannot treat subgoals as independent Boolean tasks. Scoped metavariable substitution and acyclic assignment remain the governing discipline; no generic higher-order solver is required.

This graph is also a useful unit for parallel agents. They can work on independent regions with explicit base snapshots. Merge only validated compatible patches. Reusing a display name or a syntactically similar goal is not permission to combine branches.

Contextual type-theory research provides precedents for explicit local dependencies and substitutions, but Shard's I/L coupling still needs its own specification. [X5]

### 6.3 State exactly what a negative result rules out

If the program is `f n = n` and the goal is `forall n, f n = n`, one malformed proof attempt can fail definitively while another succeeds. Thus:

```text
not ValidPair(program, this_proof)
```

does not imply:

```text
not exists proof, ValidPair(program, proof).
```

A rejection needs a subject: the exact submitted node, a proof branch, a program/proof pair, or all completions of a specified region. `Malformed` for a frozen closed declaration is not automatically a theorem that every filling of a related open template is malformed. `applicable` returning no candidates within its fragment/budget is not a completeness result.

For certified pruning, attach the reason, environment/snapshot, affected region, guards, and the type of claim: empty region, equivalent-representative replacement, or lower-bound/dominance result. Heuristic pruning remains permitted, but cannot support an exhaustive-search or optimality claim unless its coverage loss is separately repaired.

### 6.4 Do not inherit Cartesian counting accidentally

A tiny dependent example suffices: choose `n` from `{0,1,2}`, then `i : Fin n`. There are 0 + 1 + 2 valid pairs, not the product of an independent three-way n choice and an independent two-way i choice.

Similarly, two occurrences of one contextual hole represent one template assignment, not two independent choices. The same recipe at different goals can produce different proof terms. Proof irrelevance does not specify which of those objects the search counts.

A count/rank API should identify whether it addresses syntactic assignments, type-correct candidates, accepted proof/program pairs, or equivalence-class representatives. A larger dependent system can keep a simple exact finite fragment and return unsupported elsewhere. Native holes improve expression; they do not automatically preserve the old grammar's exact-count formulas.

**Tests:** shared contextual proof versus shared recipe; dependent witness/proof goals; failed proof step preserving a valid program candidate; the three-element dependent pair space; and two agents whose assignments to the same originating hole cannot be merged.

---

## 7. R23 — Keep construction evidence and checking evidence durable

### 7.1 Retain I and P for accepted releases

The new sidecar rule stores I plus a P hash and treats P itself as a cache. That is a coherent space/reconstruction tradeoff, but it makes durable verification depend on preserving the appropriate I elaborator and paying its reconstruction cost. A hash detects a changed result; it cannot recover an evicted proof. [F, §16.5]

**Recommendation:** retain I as the navigable construction artifact and retain the complete required P dependency closure for accepted releases, in deduplicated storage. They need not be separate duplicate files or always resident in memory. The active working set may evict loaded P nodes while a durable content store retains the bytes.

Support two operations:

```text
verify_release:   load retained P and dependencies; check the fixed target and policy
reconstruct:      elaborate pinned I with its version; compare/recheck the resulting P
```

Warm verification need not rerun I. Reconstruction is useful for rebuilding missing evidence, checking an elaborator upgrade, debugging, and proving reproducibility. Keeping both paths preserves I's value to search without making an elaborator a permanent operational dependency of every release verifier.

### 7.2 Hash syntax, not mathematical equivalence

Specify a canonical external P encoding independent of arena addresses, hash-table iteration order, allocation history, or parallel schedule. Scope-safe bound-variable encoding and an explicit ordering of graph references make this possible. The format should record universe arguments and immutable declaration references. Source locations and preferred display names can live in a separate diagnostic map.

Do not compute proof identity by normalizing arbitrary terms, by proof irrelevance, or by a user-supplied `BEq`. Those notions can equate objects whose provenance differs, can be expensive or incomplete, and are not substitutes for canonical serialization.

A same-type proof-irrelevance optimization inside K does not authorize treating the evidence objects as identical for assumption policy. The mathematical judgment and the evidence provenance remain different records.

### 7.3 Replace unconditional invalidation slogans with dependency classes

The draft says a proof-only change does not rebuild machine code. That is a good common case, not an unconditional rule in a metaprogramming system that can inspect proofs. [F, §§4.1, 13.2]

If compilation consumed only an established theorem's statement and permitted assumptions, replacing its proof can leave code untouched. If an optimizer deliberately inspected the I derivation, the proof body, or a proof-search result to choose a representation, those are build dependencies. Changing them can change the proposed code.

Track at least logical declaration/type dependencies, body-unfolding dependencies, evidence/assumption dependencies, and executable-build dependencies. An artifact's correctness must still be justified regardless of how its implementation was chosen.

Avoid the opposite mistake too: an implementation-only edit must not invalidate a truly abstract client theorem merely because all data lives under one directory hash. But if a client proved a concrete fact by unfolding that implementation, its proof is not abstract and must be invalidated or revalidated.

**Tests:** different arena allocation orders yield the same canonical P encoding; accepted P replays without the originating tactic or I elaborator; an I upgrade produces a distinct valid proof without silently rewriting the old pin; and a proof-inspecting build dependency invalidates when the inspected evidence changes.

---

## 8. R24 — Make identity stable across abstraction, imports, and the `v2/` flip

### 8.1 Separate physical placement from logical identity

The naming law identifies namespaces with qualified module identities, while the rollout builds under `v2/` and later renames that tree. If physical paths directly determine every declaration ID, the flip can become a corpus-wide semantic-identity change rather than a deployment change. [F, §§4.1, 6.4, 14.1]

Define a logical package root or an explicit source-to-module mapping for V2 before its first exported evidence. The physical `v2/` prefix should not accidentally become part of every long-term theorem identity. This can be a small manifest field; it does not require a registry or elaborate package manager.

Display names, logical namespaces, immutable declaration revisions, and filesystem locations serve different purposes. Preserve explicit resolution and reject ambiguity, but do not make moving a source file indistinguishable from changing a theorem's mathematical definition.

### 8.2 Clarify what survives replacing an implementation

A concrete L definition with a checked body is immutable. Changing that body creates a different revision even if the new body computes the same result. A proof that unfolded the old body cannot automatically be retargeted to the new one.

An abstract interface can remain stable while its implementation changes, but clients must genuinely depend on that interface: explicit parameters and laws, or an equivalent validated view/instantiation construction. Separate an interface slot from the particular implementation filling it.

The draft's three view conditions are the right starting point. Add an explicit account of how bodyless signature constants and required laws are represented in L: as parameters, as permitted assumptions awaiting an instance, or as opaque declarations backed by already checked evidence. Hiding a body does not manufacture a closed proof. The final instance must discharge the relevant obligations or retain them in the permitted assumption closure.

This also resolves a potential conflict between “QName plus content hash” and “implementation changes do not affect client proofs”: the hash of a concrete body and the identity of an abstract slot cannot be one undifferentiated key.

### 8.3 Do not conflate nominal and structural equality

Two packages may each expose a type called `Buffer` with identical-looking constructors and different intended invariants or ABI contracts. They must not collapse because their local spelling or shape matches. Conversely, a deliberate import mapping may identify a selected standard declaration only after its meaning and dependencies have been validated.

The same issue exists for generated names, universe parameters, and local hypotheses. Names must be hygienic, while evidence references remain stable under harmless display renaming.

**Tests:** rename the physical V2 directory without changing logical IDs; instantiate an abstract interface with two independently checked implementations; reject concrete-body proof reuse after a body change; load two same-spelled nominal types without conflation; and preserve an imported declaration through a declared mapping rather than a basename match.

---

## 9. R25 — Keep World syntax, but make the effect boundary a proved or explicitly assumed construction

### 9.1 Explicit state passing is not sufficient by itself

FOUNDATION.md carries the existing World/extern discipline forward. BOUNDARIES.md still states that clock monotonicity implies that no effect reuses a clock. That inference is false. [B; F, §§4.1, 6.4]

For example, schematically:

```text
wA = write("A", w)
wB = write("B", w)
return wB
```

If each write advances the input clock by one, the final clock exceeds the initial clock. Both operations nevertheless reused w. An endpoint inequality does not describe all uses along the execution trace.

This is a design-level counterexample to the stated reasoning, not a demonstrated exploit in the new checker. A richer logic does not repair an inadequate environment model or an invalid inference outside it.

### 9.2 Preserve the pure/effect distinction through optimization

Logical reduction must not perform live I/O. A runtime host call must not be duplicated, eliminated, memoized, or reordered merely because its source spelling resembles a pure application. Explicit World dependencies help, but a transformation must preserve the required uses, effects, and observations.

I would retain direct-style World threading and introduce a small, shared effect-use contract. A conservative first implementation can validate well-threadedness along branches, record effectful call dependencies, and establish an operational trace relationship. It need not add linear types to K, monadic surface syntax, or a general concurrency framework.

For example, mutually exclusive branches may legitimately consume the same incoming token. Two sequential effectful calls using it are a different case. A discarded return value does not necessarily make the underlying host effect dead. The validator or translation theorem must know the difference.

Pure evaluator preparation should refuse reachable live effects. Effectful execution should use an explicitly supplied environment/handler contract and the relevant trace semantics. Mathematical totality of an extern symbol does not imply that an operating-system read returns within finite wall-clock time; progress statements need their actual environment assumptions.

### 9.3 Check coherence of assumption packages before porting them

Prefer a concrete pure model of state, histories, and observations, deriving the local effect laws from it. A runtime sequencing token can represent part of that state without being the entire mathematical identity of every possible history.

Where a law bundle is assumed, record it honestly and test small concrete models or executions. Requiring a witness for an important modeled interface can expose inconsistent or vacuous requirements. This is not a general decision procedure for consistency, nor a claim that a mathematical model proves that hardware implements it.

Similarly, a strong theorem under an impossible precondition is not automatically a useful artifact. Keep representative valid-input witnesses and resource-feasible examples alongside proof gates. Strengthening a precondition to make a proof easy must be an explicit requirement change, not migration progress.

**Tests:** the reused-token example; a discarded effect result; two identical-looking read calls that must not be merged; branch-sensitive legal use; a pure evaluation request with a reachable extern; and a simple concrete model satisfying the retained effect laws. Run these before declaring the effectful application contracts ported.

---

## 10. R26 — Erasure moves obligations to boundaries; it does not remove them

### 10.1 Raw host data does not arrive with an invariant

A compiled function can legitimately erase a proof that `i < length(xs)` or that a buffer satisfies a representation invariant. Its correctness theorem then assumes that its arguments correspond to values with those properties.

An embedding host supplying raw bytes, a pointer, or an integer has not supplied those properties merely by selecting a function with the right name. The library interface needs one of two explicit routes:

```text
checked entry: raw inputs → validation/error → invocation on valid values
preconditioned entry: caller supplies/assumes the stated invariant → invocation
```

The second route is useful for trusted internal callers, but the obligation cannot disappear from the ABI contract. Rust type signatures and an FFI boundary can discharge some structural conditions; they do not automatically establish arbitrary Shard refinements.

A parser should return either a validated representation or an explicit error. Constructing an `Expr`, an `Env`, or an I node as ordinary E data does not itself establish that it is a well-typed object-language term, a checked environment, or an accepted proof.

### 10.2 Prepared handles must bind an actual execution object

The existing retained-invocation code demonstrates the performance need: it preserves name indexing, effect analysis, and translated function tables across calls. It also documents the import-cycle issue created by exposing those tables through the main interface. [MI]

The replacement handle should hide implementation tables while binding the entry, its declaration/environment revision, selected realization, argument/result representation, and execution policy. A workspace edit must not silently retarget an old handle. Reusing a declaration name is not permission to execute a newly linked body through a handle prepared for the old one.

Define failure and ownership behavior for handle release, invalid arguments, cancellation, and reentrant calls. Separate immutable shared code from per-call mutable execution state. This can begin as a small E library interface and a test client, not a fully stable foreign-language SDK.

### 10.3 Large buffers need a resource contract, not syntax expansion

For an ML runtime, do not marshal tensor contents as enormous literal term trees. Keep the program graph as inspectable data and let bulk inputs use explicit buffer views or owned buffers with format, shape, length, alignment, aliasing, and lifetime rules.

A proof about mutable buffer contents is valid for a particular state. It can become stale after mutation even when the pointer is unchanged. Use read-only ownership during the call, a state/version relation, or explicit pre/postconditions. A content hash is useful only under a convention that actually binds it to the bytes in use; it is not a substitute for preventing time-of-check/time-of-use changes.

Boundary validation need not reprove a global mathematical invariant on every call. It can rely on safe constructors, a typed handle with preserved invariants, prior checked evidence, or explicit trusted-caller assumptions. The important point is to identify which route supplies the obligation.

**Tests:** invalid refined scalar at a raw entry; a stale prepared handle after an implementation edit; a buffer shorter than its claimed shape; mutation after validation; repeated invocation without repeated module preparation; and no compulsory copying of the entire tensor payload into L syntax.

---

## 11. R27 — Make the system operationally bounded, not merely structurally fuelled

### 11.1 Compiling K does not eliminate the inner interpreter

The reference reflection route asks K's reducer to evaluate an E-written evaluator operating on program data. Compiling K removes an outer interpreter of K; it does not automatically specialize away the represented evaluator's inner interpretation.

The route may be fast enough. It may also reproduce an expensive pattern the project has already worked to remove. FOUNDATION.md should describe “fast enough once K is compiled” as a measurement question rather than a consequence of native execution. [F, §5.4 and §15]

Compare on the same representative computation:

| Route | Measure |
|---|---|
| Direct E execution | Cost of obtaining the result |
| Reflective `rfl` over `ev` | Cost of establishing the run through K's conversion |
| Equation-based proof-producing evaluation | Construction cost, evidence size, and checking cost |
| A compact domain witness plus a proved checker, where relevant | Witness production and independent validation cost |

The last two are alternatives to investigate, not guaranteed improvements. Lean's documented `cbv`/`decide_cbv` provide a proof-producing reference; they are not evidence that Shard's implementation is already economical. [X6]

Keep the simple reflective path as a reference even if a different mechanism becomes the common fast path. Specialize the evidence mechanism to the workload, not the logical authority to each domain.

### 11.2 Fuel is not a bound on every resource

A finite number of evaluator steps can allocate huge terms or invoke an enormous integer operation. Parsing, decompression, import loading, substitution, graph copying, rendering, and bignum primitives can dominate before a checker-level heartbeat fires.

Give requests an explicit aggregate resource policy covering at least work, live/allocated nodes, term/dependency depth, literal sizes, and external cancellation. A host primitive that cannot be interrupted needs a size guard or another bounded execution arrangement; passing it an enormous input is not made operationally safe by decrementing a counter once.

A logical fuel argument and a host timeout are different records. If the evaluator's fuel semantics support monotonicity of successful results under larger fuel, state and test that property. Do not infer identical fuel accounting across implementation routes unless the chosen semantic interface requires it.

Budget exhaustion must not mutate committed state or be cached as inequality/UNSAT. Request-local exhaustion should not poison a reusable checked environment.

### 11.3 Reclamation is part of the embedding contract

A persistent environment and hash-consed proof DAG can become a memory leak if every abandoned branch, failed proof, and old prepared context stays reachable through a global table.

MEMORY.md already makes recovery a requirement and distinguishes representation choices from the mathematical value model. Preserve that practical requirement for the engine's own workloads. [M]

Specify ownership of terms, checked declarations, branch snapshots, evidence caches, and prepared executables. Releasing a branch must allow its unshared data to become reclaimable. Retaining a durable P archive must not require keeping its whole decoded graph live. Keep a bounded cache policy distinct from authoritative checked state.

No change of collector policy is requested. Counting, arenas with explicit lifetimes, immutable sharing, and generation-tagged handles can be combined under their existing obligations. Measure long-lived behavior rather than declaring a particular memory technique sufficient.

**Tests:** many fork/fail/release cycles with a bounded live working set; a request involving an oversized literal; cancellation during reconstruction; enough-fuel replay; and a compiled/reflected comparison that reports both outer and inner evaluation work. Agree actual budgets before running the pathfinders; do not turn a guessed speedup into a gate.

---

## 12. R28 — The embeddable API makes adversarial construction a normal test case

### 12.1 A checked environment is not just an Env-shaped value

K is deliberately a library. Clients can construct syntax directly rather than using the canonical source elaborator. That is a product feature, and it means K cannot rely on every invariant the frontend normally establishes.

Separate raw declarations/environments from checked ones. Admission must validate the raw inputs and produce an immutable checked environment or a structured failure. Reusing a checked handle may avoid repeated validation; forging a similarly shaped record or deserializing a receipt cannot.

Validate dependent contexts in order, all required type and universe fields, constructor/recursor metadata, references in opaque bodies, cyclic declaration groups, and the assumptions actually consumed. A term's node ID alone does not establish its type independently of its context. Primitive acceleration should use the fixed validated declaration identity already required by the draft, not a convenient name or merely an expected type.

This does not make the frontend trusted simply because it does useful validation too. It gives the checker a clear contract for arbitrary programmatic clients and importers.

### 12.2 Use exported corpora and raw-construction tests for different purposes

A positive exported Lean corpus is an excellent coverage instrument. It is not a complete hostile-input corpus: an exporter or elaborator may reject malformed objects before they reach the checker.

Recent primary release documentation is instructive. Lean 4.33 reports fixes involving free variables in opaque values, malformed arguments to nested inductives, proposition classification after universe normalization, and auxiliary-name collisions. Some cases were filtered by the export path while others survived it. These are regression-test lessons, not claims of unfixed vulnerabilities. [X3]

Construct such classes of input directly through Shard's embedding API, not only by mutating accepted source files. Include same-shaped terms under different contexts, same-spelled declarations in different modules, invalid projections, corrupted DAG references, and draft holes hidden in declaration metadata.

When the reference checker and Shard disagree, keep the current policy: investigate the declarative rule, mapping, and resource conditions. Do not reproduce an upstream bug to meet a verdict-parity score.

### 12.3 Keep the initial trust statement exact

The initial reviewed K running on the reviewed Rust executor remains a legitimate acceptance route. Proving the Rust implementation correct is not a prerequisite. Nor does K's self-check establish its global soundness.

A later compiled K artifact can improve execution assurance when its source-to-bytes relationship is established. Input decoding, environment construction, and the binding between reviewed requirement and checked term remain part of the acceptance story. Do not omit them from the review merely because the central type-checking function is small.

**Tests:** direct API construction bypassing the frontend; a forged checked-environment receipt; malformed inductive/context inputs; cache reuse under a different context; and a supported cold build that exercises normal module resolution without an interpreter interpreting K's every call.

---

## 13. Fold these changes into the current arc, not a replacement arc

### 13.1 The three additions most worth doing in this rewrite

**Phase-aware executable declarations (R17/R18).** This prevents a rich logic from being useful only in theorem files. Static operation packages, erased invariants, and executable attachments let the same mathematical objects participate directly in ordinary programs. Start with a small supported translation; no general dependent runtime is needed.

**A dependency-aware derivation/workspace graph (R21/R22).** This gives I a stable basis for incremental proof construction, program/proof co-search, and independent agents. It is more useful than wrapping a mutable tactic state in a public API after the fact. Keep the first implementation simple and final K checking closed.

**An explicit identity/realization/acceptance interface (R20/R23/R24/R26).** This is the durable connection among the requirement, mathematical definition, executable view, proof, and deployed artifact. It prevents several unrelated hash-and-name conventions from becoming one implicit trust mechanism.

Those are large in leverage, not necessarily in first implementation size. Each has a small pathfinder.

### 13.2 Phase mapping

| Existing phase | Additional decision or test; not a new phase |
|---|---|
| **0 — ledger** | Choose the first phase/relevance rules, logical package-root mapping, I reconstruction contract, and canonical declaration/import policy. State which translations remain trusted during bring-up. Do not demand all realization theorems before K can run. |
| **1 — K** | Validate raw API inputs as well as exported declarations; use immutable checked environments; budget decoding and primitive requests; demonstrate the concrete modular cold-start route. |
| **2 — frontend and ev** | Test the actual executable-to-logical relation, a decision tag with erased proof fields, a branch-local bound proof, and raw/checked argument separation. |
| **3 — elaboration, I, first library** | Add one arbitrary-Prop ghost refinement, one static law-bearing package, executable attachment to an existing L declaration, and I replay under fixed reconstruction semantics. Validate the typed migration cases and retain P for the first accepted release bundle. |
| **4 — entrenchment tests** | Exercise coupled program/proof holes, refutation scope, incremental goal states, repeated invocation, memory reclamation, and the reflective-versus-proof-producing cost comparison. Include the World-use/model test before effectful certificates are treated as ported. |
| **5 — bulk port** | Use migration classes and per-interface semantic records. Do not let name-only tooling silently perform a behavior change. Retain small regression fixtures from archived implementations. |
| **6 — flip** | Move physical paths without unintentionally changing logical IDs; cold replay accepted P and selected I reconstructions; invalidate only the dependencies actually changed. |
| **7 — resume/scale** | Optimize measured hot paths, widen supported realization classes, and extend mathematics/import breadth. Optional logical-kernel experiments remain independent of normal product progress. |

A phase may initially use a conservative implementation with an explicit trust/status record. What it must not do is describe an unimplemented relationship as automatically supplied by the foundation.

### 13.3 A compact end-to-end test that ties the design together

Use one small library client rather than a separate showcase for every rule:

1. Establish a canonical list operation and register its first-order executable realization.
2. Define a static operation package with a law, specialize a fold/map consumer with a captured runtime value, and prove a result once at the abstract interface.
3. Construct a tiny implementation/proof search task with a value hole and a proof hole whose goal depends on it. Include one failed proof recipe for a valid implementation.
4. Return a concrete candidate paired with a Prop-only correctness field; retain I and P separately linked to the same requirement.
5. Prepare it once, invoke it repeatedly through a checked argument boundary, and compile one byte-tied realization.
6. Change a display name, the physical source location, the proof strategy, and then the executable body in separate trials. Confirm that the identities, invalidations, and required rechecks differ appropriately.

A second, tiny effectful fixture tests World reuse and handler observations. A third numerical fixture tests division boundaries and the distinction between logical equality and IEEE-style comparison. These are workload tests, not demands for a production ML benchmark or complete effect framework.

### 13.4 Additional probes mapped to T0–T10

| Existing gate | Additions |
|---|---|
| **T0 — oracle/raw checking** | Direct malformed declaration construction; invalid context/inductive metadata; normalized-universe cases; fixed-identity primitive validation; no majority-vote rule for mismatches. |
| **T1 — realization/migration** | Fresh-but-semantically-wrong body/equation pairing; incomplete equations; arbitrary-Prop subtype; decision-tag erasure; Nat-underflow-sensitive code; zero-divisor target behavior. |
| **T2/T3 — static abstraction** | A static law-bearing record with no runtime dictionary; captured values remain data; finite specialization of the selected program closure. |
| **T4 — holes** | A contextual proof template versus a shared recipe; dependent witness/proof goals; branch-safe assignment and invalidation. |
| **T5 — views/identity** | Same public interface with two validated instances; private equality cannot leak; physical relocation does not silently create a different declaration. |
| **T6 — embedding** | One-time preparation, erased-invariant argument validation, stale-handle rejection, buffer lifetime/mutation rules, and bounded live state after many calls/branches. |
| **T7 — search fidelity** | A bad proof does not eliminate its valid implementation; dependent counting gives three candidates in the n/Fin example; approximate relations are not silently transitive; an empty bounded `applicable` result is not UNSAT. |
| **T8 — acceptance/replay** | Direct P verification without I; reconstruction-version drift; semantic body mismatch with valid hashes; prohibited assumptions remain prohibited after proof erasure. |
| **T9 — authoring** | The agent can use one mathematical name, a ghost invariant, and a branch proof without undocumented E/L spelling workarounds. Include realistic examples and tools; do not optimize only the page count of LEAN.md. |
| **T10 — proof IR** | Fixed replay under changed ambient settings; dependency-aware goal snapshots; scope-stable references; an imported P accepted through `exact`; I schema upgrades separate from foundation changes. |

### 13.5 What not to add now

This review does not request cumulativity, cubical features, unrestricted recursive logical definitions, a new effect calculus, a production closure runtime, a different memory collector, a universal plugin router, a general proof-system importer, or formal verification of Rust. It does not request automatic complete higher-order unification or exact enumeration of arbitrary dependent search spaces.

It also does not request permanent source compatibility with the old kernel, a manually maintained second E implementation, or replacing the current rollout with a broad parallel research program.

The useful standard is narrower: **make the supported paths correct, compositional, inspectable, and cheap enough; keep unsupported paths explicit; avoid representations that make the next useful capability unnecessarily expensive.**

---

## 14. Suggested text for the next FOUNDATION.md revision

These passages are proposed edits, not claims that the decisions have already been accepted.

### 14.1 E relevance and ghost invariants — §§5.1–5.6

> Executable eligibility is checked after separating static structure, erased evidence, and runtime data. A supported runtime carrier may have arbitrary L propositions as erased refinements; deciding such a proposition is a separate capability needed only when runtime branching or membership testing requires it. Decision values retain their computational tag and erase their proof payloads. Statically selected operation packages specialize their operations and erase their laws. The supported translation remains first-order and has an explicit erasure/realization argument; no arbitrary classical witness is extracted into runtime data.

### 14.2 Existing declarations and execution — §§2/4.1

> An existing admitted L declaration may acquire an explicit checked executable realization without being redefined. `fn` requests logical admission and executable realization together; imported or ordinary mathematical declarations do not become executable by default. One canonical mathematical identity may have several implementation identities linked by evidence. An imported name is not identified with a native declaration solely by its spelling.

### 14.3 Migration — §§6.4/12/14.3

> The migration tool distinguishes name-only edits, typed representation changes, and intentional behavior changes. Nat sizes, Boolean-to-propositional interfaces, and exceptional arithmetic behavior require typed migration and an explicit semantic record. The naming law unifies references to the same mathematical object; it does not identify distinct equality, observation, or error conventions.

### 14.4 L/E binding — §5.4

> The executable view may be derived, but its relation to the admitted L definition must be justified for the actual resolved executable structure. Matching source hashes and co-generation establish provenance, not semantic correctness. The realization argument covers cases, callees, representations, and recursion; successful-result correctness and progress are distinct obligations. A fresh but incorrect translation must fail semantic acceptance even when its manifest is internally consistent.

### 14.5 I and P — §16

> I is a versioned derivation representation. Consequential reconstruction choices are explicit or fixed by its schema; replay has no hidden ambient search dependencies. Its goal structure records local identities and dependencies, and term builders are data compatible with E. Accepted releases retain I and the required P dependency closure: I supports explanation and further construction, while P supports direct independent checking. Their linked identities do not create two logical authorities; K checks the fixed target in either replay route.

### 14.6 Holes and refusal scope — §§3.3/7/16.4

> Shared infrastructure does not identify an unknown proof term with an unknown proof recipe. Every contextual hole declares what is shared and which context substitutions are permitted. A negative result identifies its subject and scope: this node, this proof branch, this program/proof pair, or all completions of a specified region. Failure to construct one proof and exhaustion of candidate generation do not establish program invalidity, region emptiness, or optimality.

### 14.7 Effects and embedding — §§5.7/6.4/8

> World threading is the source discipline; faithful execution additionally requires a coherent state/observation model and a well-threaded effect-use or trace argument. Clock monotonicity alone does not imply unique use. At host boundaries, erased input invariants are established by checked decoding, preserved typed handles, or explicit caller assumptions. Prepared execution binds a declaration, realization, environment snapshot, and representation contract; workspace mutation cannot silently retarget it.

### 14.8 Performance and validation — §§3.3/10/14

> Resource limits apply to decoding, allocation, primitives, reconstruction, and checking, not only recursive checker calls. Compiling K does not automatically remove the interpretation performed by reflective `ev`; that path is measured against direct execution and alternative proof-producing mechanisms. Long-lived engine contexts have explicit reclamation rules. Raw programmatic declarations are validated independently of frontend conventions before they enter a checked environment.

---

## 15. A few editorial inconsistencies worth removing while integrating the changes

These do not justify another architecture round, but leaving them in a normative ledger creates avoidable ambiguity.

- The same letter E denotes the executable language and, in some judgment notation, the environment. Use `Sigma` or `Env` for the latter.
- The `List.length_append` example uses an undeclared type variable alpha despite the no-auto-bound-implicit rule. Either bind alpha explicitly or show the enclosing declared section. Similarly, distinguish raw lambda proof syntax in L from lambda conveniences that must elaborate before E execution.
- “No Prop elimination into data” is too blunt next to erased subtypes, decision tags, equality transport, and impossible branches. Use the relevance/translation rule instead.
- “No indices” must identify whether it excludes general indexed inductives, all value-indexed logical interfaces, or only particular runtime layouts. Existing `Fin`, `BitVec`, and proof-indexed access need an explicit supported realization even when a fully dependent runtime IR is deferred.
- A rule restricting partiality, default values, or monadic notation in E must not accidentally forbid the corresponding legitimate L abstractions from the mathematical parity input.
- “Rust never parses or resolves” is inconsistent with the stated initial Rust loader reading narrow-compatible source. Say that it does not define an independently evolving V2 frontend; identify the exact initial loading profile and test it.
- Imported Float is no longer accurately described as opaque in the current external comparison. Preserve Shard's own float design for its actual advantages, not that outdated contrast. [X3; FL]
- “No search,” “carry verbatim,” “mechanical,” and “unchanged” should be used only when their semantic or replay contract really supports the claim. Where the implementation is merely bounded or the port is intended to preserve a relation, say that instead.
- The long dialogue history and previous-ruling tables are useful records. As implementation begins, separate the compact normative contracts from historical arguments. A developer should not need to reconcile several rounds of prose to determine one admission or replay rule. This is an organizational refactor of documentation, not a request to erase the design history.

---

## 16. Requested response from Fable

For R17–R28, mark **accept / amend / defer**, identify the affected FOUNDATION.md section, and name the existing phase/test that will establish the supported behavior. An amendment should state the replacement contract rather than only a preference.

For the three high-leverage additions—phase/relevance, executable attachment, and dependency-aware I—choose the smallest implementation that exercises the contract. For later optimizations, record a workload and a measurement trigger rather than a promise of universal improvement.

The present design is coherent enough to build. This memo is intended to prevent costly boundary mistakes, not to require a proof of the whole system before the first program runs. The initial reviewed Rust-hosted K can remain the practical authority while the stronger artifact relationships are established.

**Bottom line:** keep the foundation and the integrated engine. Let the mathematical language express richer facts, let E erase or specialize what has no runtime role, and make the proof/search/host boundaries explicit enough that those facts can be exploited without changing what they mean. The main gain available in this rewrite is not another logical feature; it is making mathematical knowledge move through the entire system without losing its identity, conditions, or operational meaning.

---

## Sources and review boundaries

Repository sources are pinned below. Section references to [F] always mean v0.4 at the reviewed commit. The old attached v0.1 document was not substituted for it. Previous review memos supplied the R1–R16 history; this memo's current-source findings were checked against the repository.

External sources were consulted on September 6, 2026. Live documentation can change; the implementation's eventual Lean release pin must be used for exact compatibility work. Research precedents motivate mechanisms, not the correctness of this proposal.

### Project sources

**[F]** `docs/FOUNDATION.md`, Fable DRAFT v0.4. Primary review target: §§2–8, 10–17, including the naming law and proof IR.

<https://github.com/computer-whisperer/shard/blob/23094d7a3d9b00d33cc10f734d54a8a75f7205df/docs/FOUNDATION.md>

**[B]** `docs/BOUNDARIES.md`. World/extern semantics, the clock argument, effect-as-data alternatives, and the proposed model/bridge pattern. The clock critique concerns the stated inference, not an executed V2 exploit.

<https://github.com/computer-whisperer/shard/blob/23094d7a3d9b00d33cc10f734d54a8a75f7205df/docs/BOUNDARIES.md>

**[FL]** `docs/FLOATS.md`. Deterministic parametric float semantics, bit/observation distinctions, packing, and the existing reference to a kernel-reducible Lean float model. Its independent design rulings are not silently replaced here.

<https://github.com/computer-whisperer/shard/blob/23094d7a3d9b00d33cc10f734d54a8a75f7205df/docs/FLOATS.md>

**[M]** `docs/MEMORY.md`. Recovery, owned mutation, resource caveats, and representation/lowering responsibilities. Referenced for the engine's own lifetime requirements, not to reopen the collector decision.

<https://github.com/computer-whisperer/shard/blob/23094d7a3d9b00d33cc10f734d54a8a75f7205df/docs/MEMORY.md>

**[MI]** `meta/invoke/prepared.shard`. Existing retained invocation and the documented public-interface import-cycle issue.

<https://github.com/computer-whisperer/shard/blob/23094d7a3d9b00d33cc10f734d54a8a75f7205df/meta/invoke/prepared.shard>

### External primary references

**[X1]** *Theorem Proving in Lean*, “Type Classes,” especially the definitions of operation packages and decidable propositions. Used to distinguish a computational decision tag from its proof payload and to identify the branch-evidence precedent.

<https://lean-lang.org/theorem_proving_in_lean4/Type-Classes/>

**[X2]** *Lean Language Reference*, “Subtypes.” Used for the arbitrary-Prop carrier/proof distinction and the reference erasure behavior. It does not prove Shard's proposed erasure pass.

<https://lean-lang.org/doc/reference/latest/Basic-Types/Subtypes/>

**[X3]** *Lean 4.33.0 release notes*, August 10, 2026. Used for the updated Float model comparison and the named classes of kernel/import hardening fixes. These are already-reported fixes, not claims of current vulnerabilities.

<https://lean-lang.org/doc/reference/latest/releases/v4.33.0/>

**[X4]** *Lean Language Reference*, “Configuring Simplification.” Used for the observation that a lemma list and budget do not alone specify all simplifier behavior.

<https://lean-lang.org/doc/reference/latest/The-Simplifier/Configuring-Simplification/>

**[X5]** Mathieu Boespflug and Brigitte Pientka, *Multi-level Contextual Type Theory*, arXiv:1111.0087v1. Background on contextual variables and explicit substitutions. No claim is made that its calculus is required by Shard or already proves Shard's proposed I/L closure property.

<https://arxiv.org/html/1111.0087v1>

**[X6]** *Lean Language Reference*, “Tactic Reference,” `cbv` and `decide_cbv`. Used as a current proof-producing evaluation precedent, not as a claimed Shard implementation or performance result.

<https://lean-lang.org/doc/reference/latest/Tactic-Proofs/Tactic-Reference/>

**[X7]** *Lean Language Reference*, “Floating-Point Numbers.” Supporting reference for distinguishing mathematical, bit-pattern, and IEEE-style comparisons. Shard must retain the relations required by its own model rather than import them by name alone.

<https://lean-lang.org/doc/reference/latest/Basic-Types/Floating-Point-Numbers/>
