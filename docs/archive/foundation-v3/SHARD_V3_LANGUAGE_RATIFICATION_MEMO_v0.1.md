# Shard V3 — LANGUAGE.md Ratification Memo

**To:** Christian and Claude Fable  
**From:** ChatGPT  
**Date:** 2026-09-14  
**Version:** v0.1 — review recommendations, not a ratification decision  
**Review baseline:** `computer-whisperer/shard` at `dabfa42b7246593cbc48f5980544089176db5c36`  
**Primary source:** `v3/LANGUAGE.md`, “The V3 language — S, L and E at Stage 0,” including all 33 decisions in §13.  
**Review IDs:** R55–R62, continuing after the Phase-2 checkpoint memo’s R49–R54.

## Executive recommendation

**Ratify most of the language direction, amend the contracts identified below, and distinguish the current Stage-0 profile from permanent language rules. Continue the existing implementation plan.**

The K/E/R division is settled: K and the executable toolchain are written in E; they represent and reason about L as data; Rust provides an explicitly trusted bootstrap execution path. This memo does not reopen the logical foundation, the compiled-deployment objective, the static-lambda direction, or the practical decision to keep Rust useful during bring-up.

The main risk at this checkpoint is ratifying a temporary implementation mechanism as the permanent meaning of Shard. In particular:

- A bootstrap-compatible source profile must not become a permanent special language for `kernel/` and `meta/`.
- A physical constructor tag must not become an observation available on an otherwise opaque type.
- Matching a module’s signatures and names must not be confused with establishing its logical instance.
- Valid equations must not stand in for an unresolved progress argument.
- Constructor arities alone must not establish that decoded host inputs have the promised type.

The source already acknowledges many of the relevant stage limitations. The requested work is to make their consequences explicit in the contract and the APIs, not to replace the architecture. [S1], [S4], [S5], [S6]

### Basis and limits of this memo

This document packages the preceding LANGUAGE.md review against the fixed revision above. **Source observations** below report what that revision says. **Proposed wording** is a recommendation for its next iteration, not a quotation of existing law. **Counterexamples and regressions** are reasoning examples or requested tests; they were not executed against the implementation for this memo. No new CI run, end-to-end exploit, or formal validation is claimed.

The earlier uploaded FOUNDATION v0.1 is historical context, not the specification being ratified here. Section references in this memo mean `v3/LANGUAGE.md` at the review baseline unless explicitly qualified. Later commits may supersede these observations; they should be reconciled by revision rather than silently substituted.

## 1. Ratification vocabulary and scope

Use three dispositions for §13:

| Disposition | Meaning |
|---|---|
| **Ratify** | Accept the rule or commitment, with any stated clarification. This is not a claim that every implementation obligation is already proved. |
| **Profile** | Accept a deliberately bounded Stage-0, bootstrap, or test-harness mechanism. Name its scope and successor condition; do not turn its current coverage into a permanent limitation. |
| **Amend** | Retain the direction, but change the contract before ratifying that item. Implementation may proceed under an explicitly narrower guarantee where appropriate. |

A temporary mechanism can be useful and acceptance-grade for the work it actually supports. “Profile” does not mean that every theorem checked on Rust is provisional. It means that the source frontend, executable realization coverage, or comparison format has a stated domain.

The contract should also separate **deciding the rule**, **implementing it**, and **establishing its evidence**. Agreeing that module instances require justified substitution does not assert that the substitution mechanism already exists. Conversely, an already working Stage-0 evaluator need not be discarded while stronger admission is built.

## 2. Review requests

### R55 — Make the bootstrap profile explicit and prevent directory-based authority

**Priority:** Decide before ratification; integrate with the Phase-3 opener.  
**Affected text:** §§3.3, 6.6–6.7, 8, 11; §13 items 8, 16 and 26.

**Source observation.** A root-relative path under `kernel/` or `meta/` selects the toolchain profile. That profile has flat suffix-based resolution, auto-bound type variables, integer numerals, byte-list string literals, and no L declarations. In it, `type` contributes only E data; `def`, `theorem`, and `inductive` are refused. The draft also commits to admitting the toolchain’s own sources to L later. [S1], [S4], [S7]

**Concern.** This is coherent as compatibility infrastructure, but not as the lasting status of the kernel and metaprogramming libraries. Those libraries need executable algorithms and mathematical claims about them in the same ordinary module system. Leaving them permanently in a profile that rejects such declarations reintroduces the original “kernel as oddball” problem.

This is a source-authoring and bootstrap issue, not an argument that E must execute arbitrary higher-order mathematical objects. K’s computational implementation remains E.

**Proposed wording**

> The bootstrap profile is an explicitly identified and versioned source-compatibility profile. Its selection is recorded among a package’s build inputs. The current directory convention selects it during bring-up, but directory placement is not a permanent semantic privilege. Kernel and metaprogramming sources may migrate to ordinary S as the frontend supports their constructs, while their executable implementations remain in E. Profile selection neither grants access to K’s private representation nor waives logical admission or realization obligations.

A manifest or explicit loader configuration is sufficient. A new source marker is not required now. Preserve the working bootstrap sources and their execution route while the migration is performed.

**Requested tests and completion criteria**

- A package’s selected profile is inspectable and is an input to source processing; a cached frontend result cannot be silently reused under a different profile.
- The first general `meta/` client uses K’s sealed public interface. Its directory does not grant `CheckedEnv`, pin-list, or memo-state construction privileges.
- A small ordinary-S library contains an executable operation and a theorem about it. This demonstrates the destination for metaprogramming source without requiring an immediate whole-kernel port.
- Any remaining legacy-only source features have a documented migration or compatibility rule. Moving a file does not silently change its literal or binding semantics without a profile transition being recognized; the separately specified module-name change still applies.

**Completion means:** the profile is accepted as a named compatibility mechanism, K’s seal remains mandatory at its first general client, and ordinary source is the destination—not that every bootstrap file must be rewritten before Phase 3 starts.

### R56 — Give generalized `if` a typed, representation-independent discriminator

**Priority:** Amend before ratifying the ordinary source rule.  
**Affected text:** §§5.4, 6.2, 6.7; §13 item 9; the corresponding C9 rule in `v3/CANON.md`.

**Source observation.** The written execution rule selects the then-branch when the condition’s cell is its type’s second constructor. The linked evaluator uses a tag bit. The module contract separately prohibits inspecting constructors of an opaque `sig type`. [S3], [S4]

**Concern.** A runtime-cell test must not bypass the public observation boundary of an abstract type. For a consumer of an interface exporting `sig type Handle`, this expression must not acquire meaning solely from the implementation’s physical layout:

```text
if handle then a else b
```

By contrast, testing a transparent `Option A` for its declared second constructor is a legitimate observation under the proposed generalized binary rule. The distinction is whether the type’s public semantics supplies that discriminator—not whether a machine value happens to contain a tag.

**Proposed wording**

> An E conditional consumes a statically established decision representation. The initial supported cases are `Bool`, `Decidable P`, and explicitly known eligible two-constructor types under the declared constructor-discrimination rule. An opaque or unresolved type does not supply this observation merely because its runtime representation has a tag. Lowering preserves the logical discriminator; physical tag layout does not define it. Unsupported condition types are refused or left as explicit elaboration obligations, never assigned incidental truthiness.

For transparent two-constructor types, the constructor-order policy can remain: the second constructor selects then, the first selects else. Keep the source-to-runtime mapping explicit so an alternative representation can implement that observation using a tag, null test, or another justified operation.

At Stage 1, a propositional condition is a different authoring case: elaboration supplies a selected executable decision procedure. The proposition itself is not runtime data. Its choice of procedure and relevant declaration identities belong to the resolved program.

**Requested tests and completion criteria**

- `Bool`, an established `Decidable` result, and a transparent eligible binary datatype branch according to their specified observations.
- A consumer cannot branch on an opaque `sig type` without an exported discriminator or other authorized interface operation.
- One- and three-constructor types do not silently inherit the binary rule.
- An unresolved generic condition remains an obligation or is refused at the appropriate stage; it is not accepted based on the runtime cell.
- A representation change preserves branch selection without preserving its physical tag encoding.
- C9 applies only within this established domain. Dropping a payload from pattern use is not permission to drop the computation that produced it; the earlier forcing/effect conditions remain applicable.

No dynamic truthiness protocol or new logical primitive is requested. A conservative eligibility check is enough for the initial profile.

### R57 — Distinguish module matching, operational linkage, and logical instantiation

**Priority:** Amend the contract before ratification; establish the connected instance path in Phase 3.  
**Affected text:** §§6.5–6.7, 9 and 11; §13 items 12, 13, 14 and 18; T5.

**Source observation.** Views introduce axiom-kind parameters to K. Consumers are checked against those parameters. An implementation is checked in a fork; type and function forms are matched, requirements are handled by `fulfills`, and `DISCHARGE` records track parameters. View theorems are not rechecked in the fork. At Stage 0, a `sig fn` is matched to an E `fn` by signature even though the latter has no admitted L meaning. [S4]

**Keep:** one environment per role, abstract consumer checking, and static substitution instead of mandatory runtime lookup. A new public link declaration is unnecessary.

**Concern.** A dependency closure identifies assumptions; it does not supply their witnesses. Matching a same-named form and deleting a parameter from an assumption report is not the logical substitution argument.

Schematically, a checked consumer result has this shape:

```text
Given p : Operation and h : RequiredLaw(p), establish ConsumerProperty(p).
```

For an implementation `a`, reuse requires a justified substitution for `p`, checked evidence of `RequiredLaw(a)`, and the executable relationship required by the consumer. It need not rediscover the consumer proof, but the instantiated result must remain justified.

If the implementation’s evidence uses an additional axiom `A`, the final result still depends on `A`. Discharging the abstract law does not erase the assumptions used to discharge it.

**Proposed wording**

> A checked module instance records the substitution from view parameters to admitted implementation declarations and evidence. Instantiation preserves the checked judgment, propagates the assumptions of the supplied evidence, and binds the selected executable realizations and their obligations. Signature matching, operational linking, and logical instance establishment are distinct statuses. A parameter is logically discharged only when the corresponding instantiation construction is justified, not merely when an implementation form with the expected name or signature is found.

Because the current parameters are represented as constants, specify the actual mechanism: abstraction into logical parameters, transported proof terms checked in the target environment, or another justified environment-substitution construction. Do not require a particular implementation before evaluating the first example, but do not leave the connection as an implication of the word “closure.”

A no-recheck optimization for consumer proofs needs that construction as its warrant. A checked instance record is compatible with the existing decision that substitution is the link.

**Requested tests and completion criteria**

- A Stage-0 signature match can establish runnable linkage, but cannot be reported as the completed logical instance of an unadmitted function.
- One consumer proof is used with two implementations of the same interface. The records distinguish the two substitutions and their realizations without cloning the authored consumer proof.
- An implementation establishes a required law using an extra permitted axiom. The instantiated consumer retains that assumption; a stricter acceptance policy refuses the resulting artifact.
- A wrong or pending `fulfills` leaves the required law unresolved. Runtime linkage cannot upgrade its status.
- View and implementation revisions cannot be mixed by matching spellings alone. A checked substitution is bound to the actual revisions and dependent types it instantiates.

Keep `DIR/BASE.shard` as the initial layout. Retain the existing Phase-3 two-instance gate before bulk library migration; directory layout must not become the only possible identity for an instance.

### R58 — Separate realization evidence, progress, applicability, and execution trust

**Priority:** Amend before ratification; carry statuses into Stage-1 admission.  
**Affected text:** §§3.2, 6.4, 7.1–7.5 and 11; §13 items 20–24 and 26.

**Source observation.** Supplied realizations generate equations checked by K. Structural recursion receives a syntactic descent check. A nonstructural `(measure E)` is reported as pending while the realization is attached. The same paragraph concludes that self-recursion is structural. Derived views are called exact by construction; the primitive table is described as having K’s rule as its realization. [S5]

**Concern 1: equation correctness is not progress.** Consider this specification-level example:

```text
Admitted logical definition:
    f(n) = 0

Candidate executable body:
    f(n) = f(n + 1)

Proposed measure:
    n
```

The equation `f(n) = f(n + 1)` is true of the logical constant function. The executable recursion does not return, and the proposed measure increases. This is a contract counterexample, not a reported execution through the current checker.

The candidate can be retained for development. Its missing guarantee must remain attached through callers and module instances rather than existing only as a warning beside an otherwise completed realization handle.

**Proposed wording**

> A realization records separately its correspondence evidence, progress obligations, applicability conditions, remaining assumptions, and execution-assurance basis. A candidate with pending obligations may be retained and explicitly executed for development, but is not eligible where a caller requires the missing guarantee. Pending obligations propagate through selected callees and module instances. Checked equations establish their stated logical facts; they do not by themselves establish that the attached executable body terminates or meets every required observation contract.

The public representation need not use five separate object types. It must prevent a consumer from mistaking an attachment with pending progress for a completed total realization. A true equation may still be an independently usable theorem about the admitted logical definition; the missing progress concerns the executable attachment.

**Concern 2: erasure strategy is not automatically verified erasure.** “Exact by construction” can describe the intended derived translation. It should not imply that the correctness of the implemented translation has already been established merely because no per-function equations are generated.

**Additional proposed wording**

> The derived-view translation follows the specified erasure rules. Until its implementation correspondence is established, the relevant translation remains an explicitly identified bring-up trust dependency. K’s logical reduction rules fix the intended meaning of core primitives; the executor’s implementations are separately tied to those meanings by the declared conformance and, when available, realization evidence. A fixed primitive table is not an unaccounted exemption from execution correspondence.

This preserves acceptance-grade checking on the trusted Rust executor. It does not require certifying Rust or the entire compiler before useful work proceeds.

**Requested tests and completion criteria**

- The increasing-recursion example never receives a completed total-realization guarantee, even when its logical equation checks.
- A caller selecting that candidate retains the unresolved obligation or is refused when it requires total execution.
- A fully justified structural realization remains usable without inheriting unrelated pending candidates.
- A refused equation attaches no partial successful realization; the existing transactional behavior is retained.
- Derived and primitive realizations expose their current assurance basis accurately. Reports distinguish completed mathematical evidence from the trusted execution and translation path used with it.
- Update the contradictory structural-only sentence to distinguish completed structural realizations from retained nonstructural candidates.

### R59 — Require validated entry codecs and an explicit World/handler contract

**Priority:** Amend item 29 before ratification; extend the existing checked-entry fixture.  
**Affected text:** §§6.7, 9, 12.1 and 12.6; §13 items 17, 27 and 29; T1.

**Source observation.** The written checked-entry criterion accepts a datatype with a nullary first constructor and binary second constructor as an argument-byte list. It identifies the final World argument using constructor shape. The bootstrap wire uses prelude `List`, `Option`, `Pair`, and `Bool` cells; the public S byte convention remains an open compatibility item. [S6], [S7]

**Concern.** These shapes do not establish the decoded value’s type. For example:

```text
Tree = empty | branch(Tree, Tree)
```

has the stated constructor arities. Supplying a byte integer as the first field of `branch` does not construct a `Tree`. This is a counterexample to the written criterion, not a claim that this particular input was executed.

Likewise, the ability to construct a datatype’s nullary constructor is not sufficient to establish that it is the selected handler’s World capability.

**Proposed wording**

> An entry codec is associated with a specific type and runtime representation contract. Successful decoding establishes that contract. Constructor arities alone neither supply a byte-list codec nor authorize World construction. A checked structural list codec validates its constructor identities, supported head representation, recursive tail type, and instantiated parameters. The World argument is supplied under the selected handler’s declared representation and entry contract. Unsupported entry types are refused before user computation begins.

The minimal implementation may register only the existing supported integer and byte-list cases, or fully validate their structural schema. It does not need a general FFI or deriving mechanism. During bring-up, reviewed codec implementations may be part of the stated execution trust; the contract still needs to say what they establish.

**Public wire recommendation — proposed, not an existing decision.** Prefer `ByteArray` for raw binary transport and `String` for text convenience APIs, with explicit encoding/decoding. Keep the old byte-list cells as internal bootstrap adapters where useful. Choose the public boundary with the first ordinary-S host library so applications need not import a competing prelude to perform I/O.

**Requested tests and completion criteria**

- Supported integer and byte-list entries still decode correctly, with accurate argument origins on failure.
- The `Tree` example is not accepted as a byte-list codec; a list-like type with an unsupported head field is also refused.
- Successful byte decoding establishes the selected byte range and representation requirements.
- An arbitrary final nullary datatype is not silently treated as the handler’s World.
- A small S application invokes the intended host operation through the public adapter, without depending on hidden prelude constructor spellings.

### R60 — Turn the small authoring gaps into positive Stage-1 commitments

**Priority:** Decide scope now; implement before broad source/proof migration.  
**Affected text:** §§11, 12.1 and 12.6; the first ordinary-S library and I consumers.

**Source observation.** The ledger explicitly identifies missing list literals, convenient negative numerals, record construction/update, names/symbols, and the public byte boundary. It names existing consumers and keeps several items “AT RISK.” Wider word realizations and the `(lib …)` form are also deferred. [S7]

**Recommendation.** Keep the staged implementation, but commit to the small facilities that prevent ordinary source from remaining harder to use than the bootstrap profile. These are fixed language/elaboration facilities, not user-defined grammar extensions.

| Facility | Recommended commitment | Required boundary |
|---|---|---|
| List literals | Add at Stage 1, including expected-type-driven empty lists. | The same resolved type gives the same logical and executable construction. Unresolved element types remain ordinary obligations. |
| Negative integer literals | Provide a specified Stage-1 rule without waiting for the full general typeclass system. | No silent `Int → Nat` conversion; the selected numeric interpretation is visible after elaboration. |
| Named-field construction and update | Add before migrating record-heavy code. | Produce a complete well-typed construction; changing an earlier dependent field may require new later fields or proofs. |
| Name values | Choose a convenient literal/construction route for represented names. | Name data is not authority to forge checked declaration, node, or environment identities. |
| Fresh-name supply | Replace effectful `gen_fresh` with a reusable scoped supply in ordinary code. | Counter scope, fork/merge, and rebasing prevent accidental collisions across workspaces. |
| Public binary/text adapters | Decide with the first host-facing S library. | Raw bytes and decoded text have distinct contracts; legacy wire cells remain an adapter detail. |

**Dependent update example.** A record with a length field and a value/proof depending on that length cannot retain those dependent fields blindly when the length changes. The elaborator may preserve unaffected fields, reconstruct justified ones, or create explicit obligations. An unresolved replacement proof must not be hidden by convenient update syntax.

**Scope limits.** Static widths above 64 may remain unsupported by the first executable realizations, but that is not a restriction on what the mathematical `BitVec` family can express. The exact `(lib …)` form may wait; compiling and embedding the engine and selected `meta/` libraries remains a product requirement independent of that spelling. No dynamic closure tier, broad notation facility, or new logical primitive is requested here.

**Completion means:** the Stage-1 plan names these facilities and their first examples, then validates them before the broad port pays the cost of their absence. The plan need not freeze every surface spelling in this memo.

### R61 — Keep identities and validation reusable beyond the current layout

**Priority:** Clarify during ratification; test with the loader, store, and two-instance work.  
**Affected text:** §§3–3.3, 6.7, 7.4 and 10; §13 items 1, 4, 6, 10, 14, 15, 25, 28, 30 and 33.

**Source observation.** Native K names combine module paths and dotted declared names; imported declarations retain their exported names with an `Init.NAME` citation route. The draft distinguishes a reporting fingerprint from an authoritative identity and says prefix growth does not change imported declarations. Equation names use `NAME.realize_N`. Reading and classification are fused, and the frontend dump is an explicitly limited comparison projection. [S2], [S5], [S8]

**Keep these directions.** The needed clarifications are about scope, not replacing the identity architecture.

First, the naming construction is not intrinsically collision-free:

```text
a.shard declares b.c       → a.b.c
a/b.shard declares c       → a.b.c
```

Deterministic duplicate-name refusal is a valid initial policy. Describe it as collision detection rather than claiming that two modules can never collide. K names are scoped to an environment; durable records must bind the intended package/import and content revisions.

Second, `NAME.realize_N` is a useful initial naming convention, not a reason to make only one realization of a logical constant possible. Persistent evidence must identify its particular realization. The current convention can remain while the two-instance/realization work decides how multiple alternatives are named and selected.

Third, transitive visibility follows the selected interface graph. It does not make private implementation declarations public through an indirect import or profile shortcut.

Fourth, fusing reading and classification is an implementation optimization. A client constructing `Prog` or declarations as data must be able to obtain equivalent validation without serializing the object into source and parsing it again. The current dump remains a conformance format, not that public representation.

**Proposed wording**

> Resolved names are unique within a checked environment; conflicting constructions of the same name are refused explicitly. Persistent identities bind the relevant declaration and package/import revisions. Prefix boundaries record a load, not a new identity for shared declarations. Multiple realizations and module instances remain distinguishable from the logical declarations they implement. Validation is available to both source frontends and programmatic construction clients; implementation fusion does not require a text round trip.

**Requested tests and completion criteria**

- The two native declarations above produce a deterministic, informative collision rather than conflation.
- A larger pinned import prefix preserves the identities of declarations already present.
- Imported and same-spelled native types remain independently citable; type-position disambiguation does not choose between two genuinely different type candidates.
- The two-instance test retains separate substitutions and realizations under the shared interface.
- An API client constructs a small program as data, validates it, and executes or checks it through the appropriate public services without a source round trip.
- The existing frontend projection is used only within its checked domain; it is not promoted to a persistent identity or proof format.

### R62 — Consolidate LANGUAGE.md into one current normative account

**Priority:** Complete before marking the document ratified.  
**Affected text:** the status preamble; §§3.3, 6.2–6.7, 7.1–7.5 and 8; §13.

**Source observation.** Earlier specification paragraphs are followed by “as built” sections that revise their answers, while some still say the earlier sections remain the specification. The following conflicts or misleading summaries remain in the reviewed text. [S1], [S3], [S4], [S5]

| Earlier wording | Required consolidated answer |
|---|---|
| `CheckedEnv` is sealed through `kernel/env`’s view. | K as a whole is the authority boundary; preserve item 26’s Phase-3 completion criterion. |
| One realization equation is generated per outermost match arm. | One equation per case-tree leaf under the supported translation. |
| Profile files gain `use` lines when V3 reads them. | The current profile retains flat scope; any later migration is explicitly selected under R55. |
| `gen_fresh` is listed in the primitive table. | It was dropped; use the ordinary scoped-supply direction. |
| Stage 0 “has no types.” | It has declared E types and partial static reconstruction, but not the complete Stage-1 typing/admission guarantees. |
| Attached nonstructural measures can be pending, yet all self-recursion is structural. | Distinguish completed structural realizations from retained candidates with unresolved progress. |
| `ev` is described as the old frontier-search/direct-recursion mechanism. | Separate its public operational semantics from the implemented continuation machine; do not describe an obsolete execution mechanism as the current implementation. |

**Proposed editorial rule**

> Each semantic rule has one current normative statement. Stage-specific limits are stated beside it. Superseded alternatives, implementation chronology, and benchmark narratives live in the phase record or are clearly identified as history. A later “as built” paragraph does not silently override an earlier normative rule.

Replace the long status preamble with a short declaration of revision, stage, normative parent, implementation scope, and links to the record. Preserve the evidence and historical measurements in their appropriate home; do not delete them or upgrade them into proofs.

**Completion means:** an author or agent can answer what a form means and what a successful result guarantees without reconciling several dates of prose. Update the original paragraphs, not merely the decision table.

## 3. Proposed disposition of all 33 LANGUAGE.md §13 items

The numbers in this table are **LANGUAGE.md’s own decision numbers**, not review IDs. This is a recommendation to the ratifier, not a claim that these decisions have been made.

| Item | Subject | Disposition | Recommended ruling |
|---:|---|---|---|
| 1 | Native/imported K names | Ratify with clarification | Keep the mapping; detect qualified-name collisions and bind durable identity to revisions. R61. |
| 2 | Universe suffix `.{…}` | Ratify | Keep the explicit form; Stage-1 inference may omit recoverable arguments. |
| 3 | `axiom`, `opaque`, `abbrev` | Ratify | Preserve their distinctions and the separate assumption policy. |
| 4 | `NAME.realize_N` | Profile | Accept current names; evidence identifies the particular realization, and future alternatives are not prohibited. R61. |
| 5 | Sequential `let` | Ratify | One ordinary meaning across L and E. |
| 6 | Prefix-based `Init` import | Profile | Practical loading mechanism, not permanent user dependency syntax or declaration identity. R61. |
| 7 | Constructor scope and `type` opening | Ratify | Preserve deterministic resolution and ambiguity errors; allow later expected-type reconstruction. |
| 8 | Layout selects the bootstrap profile | Amend | Make profile selection explicit/versioned; directory placement grants no authority. R55. |
| 9 | Second-constructor `if` | Amend | Establish the permitted logical discriminator from the public type contract. R56. |
| 10 | Transitive visibility | Ratify with clarification | Follow the selected interface/import graph without exposing private implementation declarations. R61. |
| 11 | Continue after declaration refusals | Ratify | Continue collecting diagnostics, not granting acceptance to failed targets. |
| 12 | Forked environments by role | Amend | Keep forks; specify checked instance substitution and separate matching/linking/admission. R57. |
| 13 | Reuse view theorems without rechecking | Amend | Require a justified instantiation construction and assumption propagation. R57. |
| 14 | `DIR/BASE.shard` as implementation | Profile | Accept the initial layout; preserve the Phase-3 two-instance gate. R57/R61. |
| 15 | Reading performs classification | Ratify as implementation choice | Fusion is fine; programmatic clients have equivalent validation without text round trips. R61. |
| 16 | Flat E-only `kernel/` and `meta/` | Amend | Compatibility profile, not the permanent authoring restriction for these libraries. R55. |
| 17 | Continuation machine and current wire | Ratify / Profile | Accept the machine; scope prelude cells and six-name dispatch to the current host adapter. R59. |
| 18 | Substitution is the link | Amend | No new link syntax required, but the checked instance/substitution record is explicit. R57. |
| 19 | Drop `gen_fresh` | Ratify | Use a reusable scoped supply in ordinary code. R60. |
| 20 | First-order reconstruction for equations | Profile | Accept the Stage-0 mechanism, not a ceiling on future elaboration. |
| 21 | Register eligible inductives on citation | Ratify / Profile | Keep lazy registration; current eligibility is a supported subset, not the permanent expressive limit. |
| 22 | Fixed primitive set | Amend | Keep identities and the initial roster; account explicitly for executor correspondence and trust. R58. |
| 23 | One equation per case-tree leaf | Ratify with amendment | Keep the construction; progress and applicability remain independent obligations. R58. |
| 24 | Remove `LATER`; restrict `fulfills` | Ratify | Keep the context restriction without losing pending guarantees when the record name disappears. R58. |
| 25 | Normalized frontend dump | Profile | Use within its declared comparison domain, never as the universal identity/evidence format. R61. |
| 26 | Seal K at Phase-3 opener | Ratify | Enforce the first general `meta/` client’s public-boundary criterion. R55. |
| 27 | External calculator differential drivers | Profile | Keep the test arrangement; ordinary applications still receive a coherent public host interface. R59. |
| 28 | Type/constructor disambiguation by position | Ratify | Resolve the supported idiom; refuse multiple genuinely distinct type candidates. R61. |
| 29 | Shape-based checked entries | Amend | Require validated codecs and an explicit World/handler contract. R59. |
| 30 | Explicit `Init.NAME` citation | Ratify | Preserve access to imported identities under native-name ambiguity. R61. |
| 31 | Expected-kind accelerator authorization | Ratify | Accept the authorization contract; this memo does not reopen the closed R49 implementation issue. |
| 32 | Three primitive outcomes | Ratify | Preserve value, domain failure, and resource exhaustion through public APIs. |
| 33 | Checked injectivity of parity projection | Profile | Accept within the stated domain and omitted distinctions; not a persistent representation. R61. |

## 4. Integration into the existing work, not a new phase sequence

| Existing point in the plan | Requested addition or clarification | Completion evidence |
|---|---|---|
| LANGUAGE ratification | Decide R55–R59’s semantics and consolidate R62. | Revised normative paragraphs plus explicit dispositions; remaining implementation limitations named beside the rules. |
| Phase-3 opener / first `meta/` client | Seal K and record profile selection. | Public raw-construction/check/inspect/recheck client; prohibited internal access refused. |
| Stage-1 elaboration and `fn` admission | Carry realization obligations; typed discriminators; small authoring conveniences. | Pending progress cannot become completed admission; list/negative/record/name examples elaborate coherently. |
| T1 branch-local proof and entries | Validate codecs and use a real branch-local proof under the intended erasure. | Wrong-shape host input refused; supported value and proof-bearing operation compose. |
| T5 module views and two instances | Instantiate one consumer proof twice, retaining assumptions and realization choices. | Two non-confusable checked instances; extra implementation assumption remains visible. |
| T6 embedding/prepared operation | Preserve public validation for constructed data and selected realization status. | Retained in-process workflow without a source round trip or private K access. |
| Store/evidence work and T8 | Bind revisions, profiles where relevant to source processing, obligations, and instance evidence. | Prefix growth preserves shared declaration identities; incompatible revisions or statuses cannot reuse an acceptance record. |
| Broad migration | Land the small source conveniences and dependent-update checks first. | Representative library and tool code is authored in ordinary S without relying on the bootstrap-only conveniences. |

These are acceptance criteria for the already-planned consumers. They do not require finishing general higher-order compilation, all mathematical libraries, or a certified Rust executor before useful work can continue.

## 5. Requested response from Fable

For **R55–R62**, mark **accept / amend / defer**, name the affected normative paragraphs, and identify the existing gate that will carry each implementation obligation. For a deferral, state the narrower guarantee that remains available and how its status is represented.

For **LANGUAGE.md §13 items 1–33**, record the ratifier’s disposition separately from implementation completion. A mechanism may be accepted for the Stage-0 profile without becoming a permanent source rule. Resolve conflicting prose in the original sections rather than adding another layer of exceptions at the end.

The desired next step is the current Phase-3 program: a sealed engine interface, productive Stage-1 elaboration, and the first connected consumer of logical definitions, realizations, and reusable evidence. **The recommendation is to make the existing design’s guarantees precise—not to restart the foundation design.**

## Sources

All repository links below are pinned to the review baseline. Section references identify the source of observations; proposed replacement paragraphs and test cases are this memo’s recommendations.

**Primary document:** [v3/LANGUAGE.md at dabfa42][S1]. The labels below refer to different portions of that same pinned file, not to independent corroborating sources.

| Label | Relevant source sections |
|---|---|
| S1 | Status, §0, §§6.7–8: Stage 0 and the toolchain profile. |
| S2 | §§3–3.3: names, imports, identities, records and raw ingestion. |
| S3 | §§5.4, 6.1–6.4 and 6.7: E terms, discriminator and evaluator. |
| S4 | §§6.5–6.7: views, implementation forks, evidence binding and sealing K. |
| S5 | §§7.1–7.5: erasure, equations, pending measures and primitives. |
| S6 | §9 and §13 item 29: assumptions, entries and host decoding. |
| S7 | §§11–12.6: deferred capabilities, migration and AT RISK owners. |
| S8 | §10 and §13: conformance scope and all 33 decisions. |

[S1]: https://github.com/computer-whisperer/shard/blob/dabfa42b7246593cbc48f5980544089176db5c36/v3/LANGUAGE.md "LANGUAGE.md — status, §0, §§6.7–8: Stage 0 and toolchain profile"
[S2]: https://github.com/computer-whisperer/shard/blob/dabfa42b7246593cbc48f5980544089176db5c36/v3/LANGUAGE.md "LANGUAGE.md — §§3–3.3: names, imports, identities, records and raw ingestion"
[S3]: https://github.com/computer-whisperer/shard/blob/dabfa42b7246593cbc48f5980544089176db5c36/v3/LANGUAGE.md "LANGUAGE.md — §§5.4, 6.1–6.4 and 6.7: E terms, discriminator and evaluator"
[S4]: https://github.com/computer-whisperer/shard/blob/dabfa42b7246593cbc48f5980544089176db5c36/v3/LANGUAGE.md "LANGUAGE.md — §§6.5–6.7: view validity, implementation forks, evidence binding and K seal"
[S5]: https://github.com/computer-whisperer/shard/blob/dabfa42b7246593cbc48f5980544089176db5c36/v3/LANGUAGE.md "LANGUAGE.md — §§7.1–7.5: erasure, supplied equations, pending measures and primitive correspondence"
[S6]: https://github.com/computer-whisperer/shard/blob/dabfa42b7246593cbc48f5980544089176db5c36/v3/LANGUAGE.md "LANGUAGE.md — §9 and §13 item 29: assumptions, checked entries and host decoding"
[S7]: https://github.com/computer-whisperer/shard/blob/dabfa42b7246593cbc48f5980544089176db5c36/v3/LANGUAGE.md "LANGUAGE.md — §§11–12.6: deferred capabilities, migration and AT RISK owners"
[S8]: https://github.com/computer-whisperer/shard/blob/dabfa42b7246593cbc48f5980544089176db5c36/v3/LANGUAGE.md "LANGUAGE.md — §10 and §13: conformance scope and all 33 ratification decisions"

---

*This memo proposes document and test changes. It does not modify the repository, ratify decisions on the user’s behalf, or assert that the proposed regressions have been run.*
