# Shard V2: final clarifications before implementation

## Close the remaining wording and test gaps; keep the architecture

**Status:** REVIEW MEMO v0.1 — proposed clarifications, not ratified policy.  
**Date:** September 6, 2026.  
**To:** Christian Balcom and Claude Fable.  
**From:** GPT-6 Astra Pro.  
**Reviewed baseline:** `docs/FOUNDATION.md`, DRAFT v0.5, commit `985776b67917fc293ce06d016546df6fffa12075`, blob `c39500771ea70d3755274cefac18b523519fdd68`. [F]  
**Companion source:** `docs/records/FOUNDATION.md` at the same commit. [R]  
**Relationship:** Continues the R17–R28 integration review. Those items are substantively addressed. R29–R34 below capture the last clarifications from the subsequent review; they are not six new workstreams.  
**Evidence boundary:** Source observations refer to the pinned v0.5, not the original uploaded v0.1. Replacement passages, examples, and tests are proposals. Examples are schematic, not tested Shard syntax. No implementation, proof, benchmark, or repository edit is claimed.

## 0. Recommendation

Proceed with the implementation plan. Retain the Shard-written K, L-as-data, relevance-aware E, the agreed static-lambda profile, Rust bootstrap execution, the naming law, executable attachments, contextual holes, I as a derivation graph, durable P, and the sibling `v2/` rollout.

The normative/records split is a useful improvement. Keep implementation obligations in the contract and their historical rationale in the record. Resolve the small inconsistencies below in those existing sections rather than adding another architecture layer.

| ID | Clarification | Contract locations | Existing tests |
|---|---|---|---|
| **R29** | K authorizes logical elimination; E realizes supported valid constructions; make residual restrictions role-aware | §§4.1–4.2, 4.6 | T1, T9 |
| **R30** | A realization is selectable only when its conditions and preserved relation satisfy the caller's contract | §§4.4, 4.8, 8.1 | T1, T8 |
| **R31** | State the `Init` identity decision next to `realize`, and make the example attach rather than redeclare | §§4.4, 5.3, 12.4 | T1, T5 |
| **R32** | Canonical serialization is the evidence-storage boundary, not a required whole-graph operation after every edit | §§7.3–7.5, 9.4 | T8, T10 |
| **R33** | World-use checking follows resource identity through supported aggregates and calls, not variable spellings | §4.7 | T1 and phase 4's World tests |
| **R34** | An older snapshot handle is not automatically invalid; distinguish revision mismatch from release or revocation | §§8.4, 9.3, T6 | T6 |

## 1. R29 — Logical elimination and executable relevance are different judgments

**Source observation.** Section 4.1 correctly preserves the tag of `Decidable P` while erasing its proof payload, but later describes decision tags as elimination from `Prop` and says such elimination happens where the translation permits it. Section 4.2 also retains a blanket prohibition on `Classical.choice`, `Sort`, and `Pi` occurrences despite the preceding static/ghost/runtime distinction. [F, §§4.1–4.2]

Case analysis on `Decidable P` is elimination of computational data: its definition lives in `Type`. It is not unrestricted elimination of a proposition. Logical elimination restrictions belong to K; the executable translation determines which already-valid terms have supported realizations. [X1, X2]

### Replacement for the elimination bullet in §4.1

> Every elimination must first be valid under K's logical rules. The E translation provides specified executable realizations of supported valid constructions; it never grants an additional logical elimination rule. Case analysis on `Decidable P` preserves its data tag and erases its proof payload. Equality transport and impossible branches use the permitted logical eliminators and their justified erasure rules. Missing evidence authorizes none of these operations.

### Qualify the residual restriction in §4.2

> After staging and erasure, runtime E contains no unsupported `Sort`- or `Pi`-typed values and no operation that obtains runtime data solely through noncomputable choice. Static parameters and erased proofs may contain richer L structure, including permitted classical reasoning; their assumptions remain tracked. A logical term is not rejected merely because such structure occurs in a non-runtime role.

This preserves the first-order runtime and the refusal of an unimplemented computational witness. It does not make arbitrary classical definitions executable or authorize arbitrary erasure.

### Repair the branch-proof example

The current successful branch returns an element while the other returns `none`. The intended result is an `Option`. Use an explicitly schematic example such as:

```text
-- xs : List A; i : Nat; result : Option A
if h : i < List.length xs
then some (List.get xs ⟨i, h⟩)
else none
```

Here `⟨i, h⟩` denotes the proof-indexed `Fin` argument. This is mathematical pseudocode, not a proposal for final surface syntax. My earlier review also used an abbreviated example; it should be corrected rather than carried forward as an interface specification.

**Tests within T1/T9:** exercise both branches with the same result type; retain the decision tag but no runtime proof; accept a permitted classical proof used only to justify an erased invariant; refuse a purported executable result supplied solely by noncomputable choice. Check logical validity and executable support separately.

## 2. R30 — Realization selection must discharge applicability, not just find a theorem

**Source observation.** Section 4.4 allows multiple realizations of one mathematical declaration and says their selection changes execution cost and the realization record, not the requirement. Section 4.8 already requires transformations to carry preconditions, preserved relations, and evidence. Apply that same discipline explicitly to selection. [F, §§4.4, 4.8]

For a logical `f : Int → Int`, both of these attachments may have valid theorems:

```text
general:      correct for every Int input
specialized:  correct when 0 ≤ x < 2^32
```

The second is not a valid substitute at an arbitrary call. A proved conditional relationship does not discharge its own condition. Likewise, an error-bounded realization cannot silently satisfy an exact-result requirement.

### Add to §4.4

> A realization is eligible only when its input conditions, representations, preserved observations, resource and failure conditions, and additional assumptions satisfy the caller's declared contract. Selection discharges the necessary conditions, establishes an authorized guard/fallback, or refuses the candidate. Any additional caller obligation is exposed as a requirement or artifact-premise change, never silently inserted. An approximate realization is not selected for an exact contract without the required relationship being established. The selection record binds the chosen implementation, its applicability evidence, and the relevant policy.

A conditional attachment may exist in the registry before it is applicable to a particular call. Registration and selection are distinct operations. Successful selection under an unchanged contract is what permits the draft's statement that the requirement remains unchanged.

This does not require a sophisticated optimizer or registry policy now. A single explicit realization selection can demonstrate the contract.

**Tests within T1/T8:** select the restricted implementation when a bound is available; refuse it when the condition is absent, unless a separately justified fallback is supplied; refuse an approximation for an exact requirement; reject a realization whose additional assumptions violate policy even when its output equation is otherwise appropriate.

## 3. R31 — Make the `Init` identity ruling normative where it is used

**Source observation.** The record explicitly reports the ruling that shared mathematical types come from Lean's exported `Init` declarations with E realizations attached. The phase plan also names this policy. Section 4.4 explains attachments generally, while the surface example still introduces `fn List.length`. [R, §2, v0.5 ruling; F, §§4.4, 5.3, 12.4]

The policy should be stated directly beside `realize`, and the example should make clear whether the declaration already exists.

### Add to §4.4

> The shared mathematical core uses the admitted declarations from the pinned Lean `Init` export under the declared import identity mapping. Import them once; attach E realizations to existing declarations where required. Do not introduce another mathematical declaration merely to give an imported operation an executable spelling. Shard-specific declarations remain native additions. Duplicate or incompatible import revisions are handled explicitly, never unified by name alone.

### Adjust the example in §5.3

Where `List.length` is already imported, show the following workflow rather than a second declaration:

```text
import the pinned Init declaration closure
construct a first-order executable implementation of List.length
realize the existing List.length with that implementation and its evidence
```

Do not invent final `realize` syntax for this memo. The example's obligation is to show one mathematical identity plus a separately identified implementation, not to standardize the parser early. A standalone introductory `fn` example can use a genuinely new Shard declaration instead.

**Tests within T1/T5:** attach an implementation and then use an imported theorem about the original declaration; confirm that the mathematical identity did not change; refuse an attempt to identify a different definition solely by spelling. Retain the existing physical-relocation test.

## 4. R32 — Keep canonical persistence out of the interactive critical path

**Source observation.** Section 7.5 specifies a canonical external P encoding independent of allocation and scheduling, while §7.3 requires incremental goal queries and §9.4 specifies reclamation. These are compatible requirements; an implementation must not satisfy the first by defeating the other two. [F, §§7.3–7.5, 9.4]

### Add to §7.5

> Canonical serialization specifies the persistent evidence representation, not a mandatory whole-graph transformation after every interactive operation. Internal arenas, local identifiers, sharing, and incremental digests may differ from the external encoding. Persistence and loading establish the specified correspondence. Cached admission results remain bound to their validated environments and contexts; internal identity alone is not proof of a judgment.

This permits an implementation to canonicalize or hash incrementally where profitable. It does not require postponing every hash until release, weakening canonical identity, or accepting unchecked cached receipts.

**Tests within T8/T10:** equivalent accepted evidence constructed under different allocation orders serializes identically under the chosen format; a local goal edit does not routinely trigger full-prefix replay or whole-environment serialization. Measure affected nodes and cold/warm work rather than mandating a particular arena or hashing algorithm.

## 5. R33 — World ownership must survive aliases and helper boundaries

**Source observation.** Section 4.7 now requires affine World use, coherent effect models, and trace preservation. This answers the clock-monotonicity concern. Its first implementation should track the resource being used, not merely repeat occurrences of one source variable. [F, §4.7]

A negative test should hide duplication behind ordinary structure:

```text
box = Box(w0)
a = unwrap(box)
b = unwrap(box)
w1 = write("A", a)
w2 = write("B", b)
return w2
```

Both writes consume the same original World despite distinct local names. This is a proposed regression case, not an observed V2 exploit.

### Add to §4.7

> Well-threadedness tracks runtime World ownership through supported aggregate values, projections, helper calls, and mutually exclusive branches. Distinct variable names do not establish distinct tokens. The check follows the staged, erased executable structure; references used only in erased proofs do not consume runtime ownership. Unsupported ownership patterns may be conservatively refused, but cannot bypass the check through a supposedly pure wrapper. Lowering preserves both the established ownership discipline and the specified effect trace.

No linear types in K, new effect notation, or general ownership framework is requested. A conservative local discipline with explicit function summaries is sufficient for the initial supported fragment.

**Existing World gate:** reject the aggregate-alias case and duplication through a helper; accept exclusive branches that each consume the incoming token once; accept erased proof references without treating them as extra runtime uses. Retain the model tests: unique use does not, by itself, establish that an effect-axiom bundle is jointly realizable, and an unused effect result does not authorize deleting the effect.

## 6. R34 — Distinguish an older prepared snapshot from an invalid handle

**Source observation.** Section 9.3 binds a prepared handle to a specific entry, environment revision, realization, representation, and policy. Section 8.4 says an implementation change invalidates prepared execution, and T6 calls for refusing a stale handle after an edit. Those passages need one interpretation. [F, §§8.4, 9.3, T6]

Recommended default: a retained handle for revision A continues to denote A. The existence of revision B does not retroactively change A or invalidate its evidence. Availability, lifetime, handler conditions, and explicit policy still govern whether A may be invoked.

### Clarify §§8.4 and 9.3

> An implementation edit invalidates reuse of an old preparation as the preparation of the new revision; it never retargets an existing handle. A handle bound to revision A may continue invoking A while its retained resources, handler contract, and applicable policy remain valid. An operation requesting revision B refuses an A handle. Release, explicit revocation, or a violated lifetime/environment condition invalidates the handle independently of whether another revision exists. A newer revision alone is not an implicit global revocation.

This supports an embedding application finishing old work while preparing a replacement. It does not permit using stale validated buffer contents or ignoring a policy that explicitly withdraws permission to run the old realization.

### Replace the ambiguous part of T6

> Prepare revision A, then introduce revision B. The retained A handle either continues executing A under its still-valid contract or is refused for an explicitly recorded revocation/lifetime reason; it never executes B. An invocation requiring B rejects A. A new B handle executes B. A released handle is refused. Mutable-input invalidation remains governed by the buffer's own state and lifetime contract.

No automatic hot reload or complex revocation service is required. Specify the simple retained-snapshot behavior first.

## 7. The next review should inspect one connected implementation path

Keep the existing phases. The following is a small integrated path through their existing gates, not a new phase:

```text
one imported logical declaration
    → one checked E realization
    → one caller using a branch-local proof
    → a claim constructed through I
    → retained P verified without the elaborator
    → repeated execution through a prepared handle
```

Introduce it incrementally as the relevant capabilities land. Then break individual connections deliberately: wrong executable body, missing bound evidence, mismatched revision, tampered result, invalid raw argument. Add the separate World-alias fixture before an effectful certificate is considered ported.

The objective is to discover whether the interfaces compose at tolerable cost. It does not require a large library, full search engine, or a certified Rust host before the first useful run.

**Requested response:** mark R29–R34 accept, amend, or defer and fold any accepted wording into the existing sections and tests. Keep the rationale in `records/FOUNDATION.md`. R17–R28 remain substantively accepted; this memo does not reopen the foundation, lambda policy, proof IR, naming law, or implementation sequence.

**Bottom line:** the design is ready for implementation. These clarifications prevent a handful of ambiguous sentences from becoming incompatible implementations; the next useful evidence should be code and measured behavior.

## Sources

**[F]** `docs/FOUNDATION.md`, DRAFT v0.5, pinned review target. Section numbers throughout this memo refer to this revision, not to a later branch state.  
<https://github.com/computer-whisperer/shard/blob/985776b67917fc293ce06d016546df6fffa12075/docs/FOUNDATION.md>

**[R]** `docs/records/FOUNDATION.md`, especially the v0.5 `Init`/durable-P ruling and R17–R28 responses. Historical rationale supports the source observations; the proposed normative text remains in [F].  
<https://github.com/computer-whisperer/shard/blob/985776b67917fc293ce06d016546df6fffa12075/docs/records/FOUNDATION.md>

**[X1]** *Theorem Proving in Lean 4*, “Type Classes,” subsection “Decidable Propositions.” Technical reference for `Decidable P` as data in `Type` with proof-bearing constructors, and for branch-local evidence. Consulted September 6, 2026.  
<https://lean-lang.org/theorem_proving_in_lean4/Type-Classes/>

**[X2]** Lean Language Reference, “Inductive Types.” Technical reference for logical elimination restrictions. Consulted September 6, 2026. The project's pinned rule specification, not live documentation, governs implementation.  
<https://lean-lang.org/doc/reference/latest/The-Type-System/Inductive-Types/>
