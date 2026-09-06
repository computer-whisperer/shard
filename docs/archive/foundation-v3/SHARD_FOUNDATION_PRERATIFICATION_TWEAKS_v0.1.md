# Shard V3: pre-ratification refinements

## Pinned types, explicit defaults, provenance, and tool boundaries

**Status:** REVIEW MEMO v0.1 — proposed edits for discussion, not ratified policy.  
**Date:** September 6, 2026.  
**To:** Christian Balcom and Claude Fable.  
**From:** GPT-6 Astra Pro.  
**Reviewed baseline:** `docs/FOUNDATION.md`, DRAFT v0.7, at commit `68c0e08daef9b9d1025e7098322f8668881202a8`; blob `de7831a6a10527d63d9ac695fe4090e45e026482`. [F]  
**Relationship:** Continues `SHARD_FOUNDATION_FINAL_CLARIFICATIONS_v0.1.md`. R29–R34 are treated as incorporated. R35–R41 below concern the additions reviewed in v0.7, not another foundation redesign. The project now calls this rewrite **V3**; earlier memos used V2 for the same work.  
**Evidence boundary:** Project observations refer to the pinned repository revision. External documentation checks are marked separately as [X1–X5]; they inform the proposed corrections but do not replace the phase-0 release pin. Examples are deductions or proposed tests, not executed Shard examples. No implementation, performance result, or formal validation is claimed.

---

## 0. Recommendation

Keep the v0.7 additions: type realizations, source provenance from the first elaborator, Stage-1 deriving, a specified numeric-literal boundary, fixed source syntax for the initial release, and generated proof artifacts outside authored source. Keep the documentation-disposition table and the existing connected implementation path. [F, §§4.4–5.3, 7.5, 10.5, 12.4]

The remaining edits should make these additions obey the contracts already chosen: exact imported identities, explicit consequential choices, shared term graphs, an embeddable engine, and verification without hidden authoring or search. They do not require a different calculus, another runtime, or a new implementation phase.

Three points deserve particular attention before ratification: the imported `String` description must follow the selected release; derived ordering is a defined policy rather than information inferred uniquely from a type; and source provenance must describe occurrences and source-free inputs, not assume one span per shared semantic node. The other items delimit existing mechanisms and add small tests.

### Requested dispositions

| ID | Amendment | FOUNDATION.md locations | Existing gate / timing |
|---|---|---|---|
| **R35** | Separate the exact imported type, its mathematical views, and its runtime realization | §§4.4, 10.3, 12.4 | Phase 0 pin; T1/T5 when types are realized |
| **R36** | Qualify quotient erasure by supported carriers, operations, and checked respect evidence | §4.6 | T1 when executable quotient support is enabled |
| **R37** | Derive operations under an explicit policy and supported field capabilities | §§5.1–5.2 | Stage 1; T1/T9 |
| **R38** | Use occurrence-aware provenance, including generated and source-free origins | §§5.1, 7.5, 12.5 | First elaborator/I schema; T0/T8/T9 |
| **R39** | Specify defaulting and coercion placement; decide and document bounded numeral behavior | §§5.2, 5.4 | First numeric elaboration; T1/T9 |
| **R40** | Scope the syntax-extension ban without closing the structured metaprogramming API | §5.3 | Wording now; existing T10 |
| **R41** | Preserve source ownership while giving explicit authoring/migration a coherent route | §§7.5, 12.3 | Wording now; T8 and migration tooling |

The replacement passages below are proposals for Fable to integrate, amend, or defer with a stated reason. They are not statements that the user has already ratified their exact wording.

---

## 1. R35 — Imported types, mathematical views, and runtime representations

**Source observation.** The new type-realization paragraph describes `String` as `List Char` in L and `ByteArray` as being over `List UInt8`; the migration table repeats those descriptions. The same document requires identity with the actual pinned `Init` declarations. [F, §§4.4, 10.3]

**External check.** The currently retrieved Lean reference describes logical `String` as a structure containing a `ByteArray` and evidence of valid UTF-8. It identifies the list-of-characters representation as the earlier model, accessible through `String.ofList` and `String.toList`. The documented `ByteArray` has an `Array UInt8` field. [X1, X2] This does not determine which release Shard must select; it means the contract must not substitute a remembered representation for the selected export.

### Proposed replacement for the concrete type examples in §4.4

> Type realizations distinguish three objects: the exact admitted declaration from the pinned core library, a mathematical view connected to it by functions and theorems, and the runtime representation related to it by a simulation. The selected `Init` export determines the logical constructor and field shapes. `Array`, `ByteArray`, `String`, and fixed-width types retain those identities; lists or character sequences are views where appropriate, not replacement definitions. Shard's existing validated-UTF-8 representation becomes an executable realization of the imported `String`, with the required correspondence, rather than a second mathematical string type.

A small inventory produced at phase 0 should record each shared type's imported identity and actual fields. Only add the view and realization information needed by the first consumers; this is not a new registry project.

For the migration table, describe `Str → String` as “pinned logical `String`; validated-UTF-8 E realization.” Describe `Bytes → ByteArray` by the pinned declaration and packed realization. Put any list-view description in a separately labeled view column or note.

**Tests:** attach a realization without redefining the imported type; reuse a theorem through an explicit view conversion; reject an invalid raw byte sequence where a checked string entry requires valid UTF-8. The test targets the selected pin, not whatever a live documentation page says later. Existing T1, T5, and boundary checks provide the homes.

---

## 2. R36 — Quotient erasure is conditional execution support

**Source observation.** The new erasure row says `Quot` uses its carrier, `Quot.mk` is a runtime identity, and the respect proof of `Quot.lift` erases. [F, §4.6]

That is a useful strategy for a supported executable carrier and lifted operation. It is not a claim that every quotient has an executable carrier, a canonical representative, or decidable equality. The logical quotient type remains distinct from its representation.

**External check.** `Quot.lift` requires evidence that its underlying function respects the relation; its computation on a constructed quotient returns that function's result. [X3] The proposed erasure can remove the evidence after checking it, not remove the obligation to establish it.

### Proposed replacement row

> **`Quot`:** for a supported executable carrier and supported quotient operations, retain the carrier's representation; `Quot.mk` adds no runtime wrapper. A realized `Quot.lift` uses the realized underlying function and erases its K-checked respect evidence. Runtime inspection or comparison of representatives is not automatically an operation on the quotient. Other quotient constructions require their own supported realization; erasure does not supply noncomputable witnesses.

This adds no logical rule and requires no general normalization of representatives.

**Test:** let `r(a,b)` mean `a % 2 = b % 2`. The representative operation `a ↦ a % 2` respects `r`; the operation `a ↦ a` does not. The former should lift and produce the same answer on representatives 0 and 2. An attempted lift of the latter must not pass with missing or invalid respect evidence. Raw representative equality must not be installed as quotient equality. These are proposed T1 fixtures, not a claim of a current implementation failure.

---

## 3. R37 — Deriving is convenient generation under a declared policy

**Source observation.** Section 5.1 places decidable equality, a canonical ordering, and diagnostic rendering together under structural reconstruction rather than consequential selection. Section 5.2 separately identifies ordering selection as consequential. [F]

These can be reconciled without making authors manually implement routine operations.

For `Color = red | green | blue`, constructor-order comparison is a possible derived ordering. The type's mathematical existence does not uniquely select it. For a record, lexicographic comparison likewise chooses a field order and the comparisons used for those fields. A default can be fixed and deterministic while still being a policy with semantic dependencies.

### Proposed replacement for the deriving paragraph

> Deriving is available at Stage 1 for supported `type` and `structure` declarations. It generates operations and the evidence required by their advertised interfaces under a declared derivation policy. Structural equality requires suitable field equality procedures and laws; ordering records its constructor/field convention and selected field orderings. Rendering has an explicit diagnostic policy. Generated declarations and policy dependencies participate in ordinary resolution and requirement identity. Unsupported deriving reports the missing capability or refuses that derivation; it does not manufacture an instance or invalidate the otherwise admissible type.

A project-default derivation policy is sufficient initially. It should be visible in the generated declaration's dependencies, not rediscovered at every use. No global instance-selection framework is required before the first structural examples.

An arbitrary function-valued field, for example, does not come with an executable extensional equality procedure merely because it occurs inside a structure. A quotient realization cannot inherit logical equality by comparing representatives. A diagnostic renderer may use a declared opaque placeholder instead of claiming to inspect every mathematical value.

**Tests:** derive operations for an ordinary algebraic type; change an ordering convention and observe a changed generated operation/affected resolved requirement; request equality on a field lacking the necessary executable capability and obtain a precise refusal. Confirm that the underlying type still admits. Use T1/T9, not a new gate.

---

## 4. R38 — Provenance belongs to occurrences and construction history

**Source observation.** The new source-span rule maps every L and I node to its S origin and requires a span on every diagnostic. K remains span-free. [F, §5.1]

Keep source provenance early and structural, but not as a mandatory single span on each shared term.

One shared node can represent two occurrences, such as the literal 42 in `f(42)` and `g(42)`. One source expression can also generate many logical nodes and proof obligations. An embedding client may construct a declaration without any S source file. Imported P and generated candidates have the same issue. These are consequences of the shared, programmatically constructible engine already specified, not exceptional input routes.

### Proposed replacement for the source-span paragraph

> K's logical terms remain free of source locations. Above K, occurrence-aware provenance records source spans, generated origins with parent references, and artifact/API origins. A shared semantic node may have multiple occurrences and origins; an operation's diagnostic follows the occurrence and construction context that produced it. Every diagnostic identifies its subject and carries the best available origin. When source text does not exist or its origin is unavailable, the diagnostic says so rather than inventing a span. Reserve provenance support in the first I schema, but exclude nonsemantic locations from logical declaration and canonical P identity.

An illustrative origin vocabulary is:

```text
SourceOrigin(source_revision, range)
GeneratedOrigin(operation, parent_origins)
ArtifactOrApiOrigin(artifact_or_request_id, node_path)
UnavailableOrigin(reason)
```

This is a sketch, not a mandated datatype. Origin metadata can have its own storage identity. Editing whitespace, relocating a file, or improving an origin map must not by itself change mathematical evidence identity. A stored source range must identify the source revision it describes.

**Tests:** identical shared terms at two source sites produce the correct site-specific diagnostic; a generated obligation points to its originating expression; a malformed API-built declaration gets an artifact/request location without a fabricated file span; a source-only relocation leaves P identity unchanged. Fold these into T0, T8, and T9.

---

## 5. R39 — Make literal construction and coercion placement predictable

**Source observation.** The new policy uses expected numeric types, defaults unconstrained numerals to `Nat`, inserts only `Nat → Int` automatically, and leaves other conversions explicit. [F, §5.2]

Keep that policy. Specify when expected types propagate, when unresolved literal types default, and where the inserted conversion sits in the resolved expression. A coercion is not permission to reinterpret an operation already resolved at another type.

Natural subtraction truncates at zero. [X5] Consequently these are different computations, expressed here in Lean-like illustrative notation:

```text
Int.ofNat ((1 : Nat) - (2 : Nat)) = 0
(1 : Int) - (2 : Int)             = -1
```

Moving the conversion across subtraction changes the program. `omega` can reason about a resolved expression; it cannot decide what expression the author intended.

### Proposed addition to the numeric paragraph

> Expected-type propagation and numeric defaulting have a specified order. Available type constraints are processed before an otherwise unconstrained numeral defaults to `Nat`; explicit type boundaries are respected. The resolved form records the selected numeric operations, literal constructions, and coercion sites. An inserted `Nat → Int` conversion never retroactively changes an operation already resolved on `Nat`. Numeric construction is not assumed to preserve the written magnitude when the selected instance specifies reduction or wrapping.

### A decision to make explicit: `Fin` numerals

**External check.** Lean's documented numeral instance for `Fin n` requires a nonzero bound and reduces oversized numerals modulo that bound: `(5 : Fin 3)` has value 2. It does not prove `5 < 3`. In contrast, `Fin.mk` takes the intended underlying value and an explicit bound proof. [X4]

**Recommendation, not an already ratified exception:** preserve the chosen imported `Fin` numeral semantics, state the behavior in `LEAN.md`, and point users to checked construction or `Fin.mk` when the original value must be preserved. A different source-level policy would need an explicit decision, not a hidden reinterpretation of the imported instance.

A zero or unresolved bound cannot be assumed positive simply to make a numeral elaborate. A blocked nonzero obligation is reported and must be discharged before final admission. This does not add automatic `Fin → Nat` or `Int → Nat` coercions; Shard's chosen explicit-conversion boundary remains.

**Tests:** expected-`Int` arithmetic; an explicit `Nat` subtraction followed by conversion; an unconstrained numeral's documented default; an oversized `Fin` numeral under the chosen policy; a zero bound; and nonwrapping construction with a supplied bound proof. Show the relevant resolved operation or construction in a diagnostic. Use T1/T9.

---

## 6. R40 — Fixed source syntax must preserve library-level metaprogramming

**Source observation.** v0.7 prohibits user-defined notation, macros, and source syntax extensions in the initial release, directing users toward functions, definitions, or tactics. [F, §5.3]

Accept this scope control. It should not prohibit the existing extension model: ordinary E programs constructing L or I, implementing proof search, defining domain-specific data, or transforming programs through canonical engine APIs.

### Proposed addition to departure 6

> This restriction concerns additions to the accepted source grammar and source-elaboration hooks. It does not restrict ordinary libraries that construct declarations, L terms, I derivations, or domain data through the published APIs, nor tactics invoked through the fixed supported syntax. Generated declarations and evidence pass the same admission and realization checks. Extending a reasoning tool does not authorize extending K's rules or bypassing the frozen target.

Keep the future notation door as already stated. No syntax plugin system is requested now.

**Test:** an ordinary E library contributes an I-producing tactic or generator, invoked through the fixed interface, while a proposed source-grammar extension receives the documented refusal. The generated evidence is checked through the same path as built-in tooling. Existing T10 can cover this distinction.

---

## 7. R41 — Source ownership and explicit migration need compatible rules

**Source observation.** Section 7.5 prohibits tools from writing authored source and locates generated I in sidecars/the pin store. Section 12.3 also assigns typed migration to a tool. [F] The intended protections are compatible, but the blanket wording needs an explicit authoring boundary.

Keep the important rule: ordinary proof search, builds, reconstruction, and verification do not edit authored source. A stale or absent `auto` pin is a pending obligation at verification, not permission to run search or alter the target. A source tactic or delegation changes only through authoring.

### Proposed replacement for the opening rule

> Search, build, reconstruction, and verification never modify authored source as a side effect. The source carries author-maintained tactics or explicit `auto` delegation; generated I and P remain in their designated sidecars and store. Source changes occur through explicit authoring or migration actions, not as a consequence of proof success or failure. Verification reports missing or stale evidence without silently re-searching.

**Strict-rule-compatible implementation:** the migration/refactoring tool emits a patch or separate staged output; the implementing agent applies it as an explicit authoring change. This preserves the existing preference that tools do not directly patch maintained source.

An explicit authoring command could instead apply an approved patch, but that is a separate policy choice; this memo does not silently authorize it. In either arrangement, semantic migration records and frozen requirement checks still apply. Labeling an operation “authoring” must not let a proof-solving task change its target unnoticed.

**Tests:** search and verification leave authored source byte-identical; a missing/stale pin reports pending without launching search; migration emits a labeled patch or staged output without altering the input tree; explicit application is a distinct action, and a requirement-changing edit is reported. These extend T8 and the existing migration-tool checks.

---

## 8. Documentation disposition and ratification

Keep §10.5's disposition table. Its purpose is to prevent contradictory current instructions while the old and new trees coexist. Clarify that ratifying a replacement contract is not the same as implementing and passing its gate.

An older issue can be superseded by a foundation decision, but its unresolved capability should remain attached to the named replacement gate. Similarly, a banner saying an old document is superseded must identify which tree the old behavior still describes during migration. This is tracking and documentation hygiene within the existing plan, not an additional review ID or phase.

### Integrate the tests without expanding the arc

| Existing area | Additions from this memo |
|---|---|
| **Phase 0 / T0–T5** | Pin actual imported declaration shapes; distinguish logical views from representations; retain source-free input diagnostics |
| **T1 / first realizations** | Quotient-respect fixture; imported string realization; supported deriving; explicit numeric/coercion cases |
| **T8 / evidence and workflow** | Origin-only changes preserve semantic identity; normal tools preserve source bytes; stale pins remain pending; migration produces explicit output |
| **T9 / authoring** | Occurrence-correct and generated diagnostics; missing deriving capabilities; bounded numerals and coercion pointers |
| **T10 / metaprogramming** | Library-provided I production under unchanged grammar and admission rules |

No complete deriving framework, general quotient compiler, notation system, or new source editor is a prerequisite for the first checker. Agree the rules now, and test each mechanism when its existing phase enables it.

## 9. Requested response from Fable

For R35–R41, record **accept / amend / defer**, the resulting normative section, and the existing gate that owns the test. Where the exact choice depends on the Lean release pin, state that dependency instead of hard-coding a live-documentation fact as the permanent rule. Record the `Fin` literal choice and the authoring/migration action boundary explicitly.

After these edits, the recommendation is to ratify the design and implement the first connected path already in §12.4. The next substantial review should use working code, failed fixtures, or measured behavior. These amendments are intended to close the last wording and default-policy gaps, not perpetuate the design cycle.

---

## References and evidence boundaries

Project observations cite [F] by section. External sources were consulted on September 6, 2026 for the specific facts described above. Their live contents are not substitutes for the chosen release and exported declarations. Proposed policies, examples, and regression tests are this memo's recommendations.

**[F] Reviewed project contract.** `docs/FOUNDATION.md`, DRAFT v0.7, commit `68c0e08daef9b9d1025e7098322f8668881202a8`.

<https://github.com/computer-whisperer/shard/blob/68c0e08daef9b9d1025e7098322f8668881202a8/docs/FOUNDATION.md>

**[X1] Lean Language Reference, “Strings,” Logical Model and Backwards Compatibility.** Supports the currently documented byte-array/UTF-8 logical model and the distinction from the older character-list model.

<https://lean-lang.org/doc/reference/latest/Basic-Types/Strings/>

**[X2] Lean Language Reference, “Byte Arrays,” `ByteArray` structure.** Supports the documented `Array UInt8` field; not a claim that all useful byte-list views are definitional identities.

<https://lean-lang.org/doc/reference/latest/Basic-Types/Byte-Arrays/>

**[X3] Theorem Proving in Lean 4, “Axioms and Computation,” Quotients.** Supports the respect obligation and computation rule for `Quot.lift`; it does not prove Shard's proposed erasure implementation correct.

<https://lean-lang.org/theorem_proving_in_lean4/Axioms-and-Computation/>

**[X4] Lean Language Reference, “Finite Natural Numbers,” Coercions and Literals.** Supports the nonzero-bound requirement, modulo behavior of numerals, and the distinction from proof-bearing construction.

<https://lean-lang.org/doc/reference/latest/Basic-Types/Finite-Natural-Numbers/>

**[X5] Lean Language Reference, “Natural Numbers,” `Nat.sub`.** Supports natural subtraction truncated at zero; the coercion-placement example is an elementary consequence, not a reported compiler defect.

<https://lean-lang.org/doc/reference/latest/Basic-Types/Natural-Numbers/>
