# Shard V3 — Single-Frontend and Construction-Layer Review

**To:** Christian and Claude Fable  
**From:** ChatGPT  
**Date:** 2026-09-18 (America/New_York)  
**Version:** v0.1 — review recommendations, not a ratification decision  
**Review baseline:** `computer-whisperer/shard` at `3062f23a92e1dd2b1ded2b83f722e3275fefe182`  
**Primary target:** `v3/LANGUAGE.md` §8.4, slice 3.16, “one front end: the pre-definition and its two projections,” with §8.5 and the implemented 3.13–3.15 machinery.  
**Review IDs:** R63–R71, continuing after the LANGUAGE.md ratification memo’s R55–R62.

## Executive recommendation

**Proceed with the 3.16 single-frontend refactor. Do not replace the logical foundation again. Strengthen the construction, erasure, and partial-result contracts while the refactor is still small.**

The recent friction is well explained by Stage-0 infrastructure becoming a second frontend: E’s reader and limited type reconstruction determine one interpretation of a body, while L’s elaborator determines another. The new design identifies that problem and correctly stops trying to repair every symptom independently. [S1]

The intended invariant should be:

> Resolve and type the intended computation once; preserve the structure needed to justify it and execute it; derive the mathematical and executable representations from that common construction.

The principal amendments are to make the **pre-definition a complete contextual construction object**, make **matcher and realization descriptions explicit inputs to executable correspondence**, and preserve **partial knowledge and outstanding obligations** through the APIs that elaboration, I, and search will share.

This memo does not reopen K’s closed logical rules, the Shard-written E implementation, the maintained Rust bootstrap, the static-lowering objective, the agreed static-lambda direction, or the sealed K boundary. It does not require dynamic closures, a second user-facing language, or verification of Rust before useful checking continues.

### Basis and limits

This document packages the preceding architectural review against the fixed revision above. **Slice 3.16 is a design at this baseline, not an implemented new frontend.** The implementation observations concern the existing 3.13–3.15 code. The checkpoint’s test and performance results are repository-reported; no current suite, proposed counterexample, or end-to-end exploit was executed for this memo. [S1], [S2]

Each request separates **source observation**, **assessment**, **proposed contract**, and **completion tests**. Suggested schemas and code fragments are schematic, not a claim that the corresponding public syntax already exists. A risk in an unimplemented projection is not reported as an observed runtime defect. An incomplete unifier is not automatically a soundness failure in K.

The earlier FOUNDATION v0.1 is historical context only. Its original ambition—broad mathematical capability while retaining static lowering—remains relevant; its superseded architectural details are not the specification being reviewed here.

## 1. Checkpoint assessment

The recent increments validate important parts of the architecture. The directory-selected toolchain profile has been retired. K is behind a module view, and client tests exercise that view through the V3 loader. The elaborator sits above K. Functions can now acquire logical definitions and equation theorems, with implemented structural, course-of-values, and measured-definition paths. [S2]

The specific frontend failure is also well localized. The slice-3.16 design reports that calc’s spec file has twelve functions defined and thirteen left `RUNNABLE`, with three roots: comparisons interpreted differently across Bool and Prop, missing operator typing around `(- c 48)`, and a constructor name resolving differently in `fn` and `def`. The probe also found that expanding nested patterns into a finished case tree duplicated a fall-through alternative eight times. [S1]

Those observations support changing the common construction layer, not changing the mathematical foundation. The proposed arrangement is the right direction:

```text
S source, or an equivalent programmatic construction request
                         |
                  one elaboration
                         |
           contextual pre-definition object
                /                    \
       logical projection       executable projection
       recursion justification  erasure and realization selection
       declarations + equations  structured E program
                |                    |
                K              execution / lowering
```

Neither output must be reconstructed from the other after information has been discarded. The common input is valuable, but it is not itself a proof that the two projections correspond.

## 2. Request summary

| ID | Request | Earliest point where it matters |
|---|---|---|
| **R63** | Make the pre-definition a contextual construction object with two explicit projection contracts | Before re-pointing `define.shard` |
| **R64** | Bind matcher descriptions to declarations and preserve conditional forcing | Matcher landing and E projection |
| **R65** | Carry branch facts into obligations; complete one measured-recursion proof | During the 3.16 integration, before relying on I |
| **R66** | Use realization-directed erasure, including an explicit Bool/Decidable representation bridge | E projection landing |
| **R67** | Preserve blocked, rejected, and exhausted outcomes; identify tentative assignments | Shared wrappers and workspace, before general I/search clients |
| **R68** | Preserve contextual holes across binder closure | Workspace interface before I hardens |
| **R69** | Share contextual obligations and explicit dependency/discharge bookkeeping | Obligation integration; scheduling improvements can follow |
| **R70** | Prevent bootstrap fallback from reinterpreting source | Frontend flip |
| **R71** | Validate semantic correspondence without permanently freezing old elaboration artifacts | Migration gate and follow-on features |

These are changes within the existing sequence, not nine new phases.

## 3. Review requests

### R63 — Make the pre-definition a complete contextual construction object

**Priority:** Agree before the refactor redirects the definition machinery.  
**Affected surfaces:** `LANGUAGE.md` §8.4’s 3.16 shape and rules 1–2, 6–7; `elab.shard`, `define.shard`, and their public construction results.

**Source observation.** The proposed pre-definition is an ordinary `Expr` over K’s locals, with a self-local and applications of generated matcher constants. Matcher descriptions live in a side table. The existing translation state already carries multiple pieces of related context: locals, E declarations, scope, recursion information, tables, and obligations. [S1], [S3], [S4]

**Assessment.** Reusing `Expr` is good. Treating the bare body as the whole reusable object would leave its essential invariants distributed across ambient state. A metaprogram should not need to recreate the original elaborator call stack to inspect, modify, or project a construction.

In particular, a body that is well-typed **assuming a local for its own recursive function** is not yet an admitted recursive definition. That provisional status must survive handoff to another library.

**Proposed contract**

> A pre-definition is a contextual construction object whose body uses the shared `Expr` vocabulary. It retains the declared type, local telescope, recursive references under construction, matcher descriptions, unresolved constraints and proof obligations, and resolved dependency identities needed by its projections. An executable projection and a logical projection identify the construction revision they consume. Neither an open self-local nor a missing obligation is promoted to an admitted definition by wrapping the body in this object.

A conceptual shape is sufficient; the initial record can reference existing structures rather than duplicate them:

```text
PreDefinition
    declared identity, type, and contextual body
    recursive-reference context
    matcher descriptions
    constraints and contextual proof obligations
    resolved environment/dependency revision
    origins and reconstruction information
```

This is not a request for a new calculus or a giant universal compiler IR. It is a request to name a state that already exists and make its boundary usable.

**Projection correspondence.** Retain an explicit obligation that, under discharged construction obligations and the declared execution assumptions, the executable projection realizes the admitted logical definition. A reviewed translator may remain a stated bring-up trust dependency. A shared input or matching provenance does not independently establish semantic correspondence.

**Completion tests**

- A small construction can be handed from the frontend to a separate library operation that inspects and projects it without rerunning source resolution.
- A self-local remains visibly provisional until recursion admission; an open construction cannot be passed as an admitted declaration.
- Both projection results identify the same construction and dependencies; stale metadata from another revision is not silently accepted.
- A deliberately changed E projection does not inherit the old construction’s completed correspondence status merely because it uses the same owner name.

**Completion means:** one explicit construction boundary with documented invariants, not certification of every translator before development proceeds.

### R64 — Treat matcher descriptions as semantic lowering inputs and preserve branch forcing

**Priority:** Before the matcher and E-projection landings are considered complete.  
**Affected surfaces:** Slice 3.16 rule 1, matcher side tables, erasure, fork/merge behavior, later artifact persistence.

**Source observation.** Each generated matcher is to be a definition K checks, while a side table retains its pattern matrix so erasure can recover `EMatch` with the original alternatives. The design calls out side-table survival across a view fork as a risk. Alternatives without pattern variables have no `Unit` thunk; erasure is expected to take them directly as branches. [S1]

**Assessment.** A matcher’s admission to K establishes the validity of that declaration. It does not establish that an arbitrary side-table entry describes the same computation. The table is therefore part of the executable correspondence, not merely a printing hint.

For example, swapping two alternatives in stale lowering metadata can change E execution while leaving the logical matcher declaration perfectly valid. Recognizing a generated-looking name such as `Owner.match_3` is not an authorization check.

**Proposed contract: identity and structure**

> A matcher description is associated with the exact matcher declaration revision and its relevant dependencies. It specifies the discriminants, alternative ordering, parameter telescopes, and the correspondence used by each projection. A missing or mismatched description is reconstructed through a specified path or produces unsupported lowering; it does not authorize name-based reconstruction. Forks and persisted constructions retain descriptions with their owning declarations and revision relationships.

The matcher declaration and description can be generated together by reviewed code during bring-up. That trust should be stated. A later checked translation or validator can strengthen the guarantee without changing the public concept. Exact declaration binding is necessary but is not, by itself, proof that a semantically wrong descriptor is correct.

**Proposed contract: forcing**

> The executable matcher evaluates its discriminants according to the specified order and then evaluates only the selected alternative. Alternative bodies are not ordinary eager argument computations. Erasure must preserve this behavior or refuse the unsupported case; it must not silently lower a matcher application as an ordinary strict function call.

For example:

```text
if n = 0 then 0 else f(n - 1)
```

must not evaluate the recursive alternative before selecting the branch. The same issue appears with an expensive or failing unselected alternative and with repeated evaluation of a computed scrutinee.

This does not require runtime closures. Preserve alternatives as control-flow structure, statically specialized helpers, or another justified representation. The logical form and executable forcing contract need not use identical physical structure.

**Completion tests**

- A stale or deliberately swapped matcher description cannot produce a completed realization for the original matcher.
- An unselected recursive or failing development branch is not evaluated. A computed discriminant is evaluated with the specified multiplicity and order.
- Nested patterns preserve first-match priority and the intended fall-through behavior without requiring the expanded case tree as the public construction representation.
- A construction remains projectable after its supported fork/merge or persistence round trip; loss of its descriptor produces an explicit failure, not generic eager-call lowering.

### R65 — Preserve branch facts in the common context and discharge one real descent obligation

**Priority:** During 3.16, before general tactics become a dependency of validating this boundary.  
**Affected surfaces:** Matcher alternatives, conditional elaboration, `define.shard`, contextual obligation generation, the measured-recursion fixture.

**Source observation.** The current `define_measure` fixture checks that `count` has a pending measure and that a dependent theorem inherits that assumption. `tr_self_wf` forms each obligation from the scope recorded at the self-call. The leading `df_branches` route pushes constructor fields into that scope, including erased decision evidence. The generic `tr_minors` route does not perform the same scope update. This is a source-level asymmetry; no failing nested-call reproducer was executed for this review. [S4], [S7], [S8]

**Assessment.** The unified frontend should eliminate this distinction between syntactic positions. A recursive call inside a branch needs the branch’s facts whether the branch is top-level, inside a `let`, or inside another expression.

The existing example is enough to expose the required contract:

```text
count(n, acc) =
    if n = 0 then acc
    else count(Nat.sub n 1, acc + 1)
```

The useful obligation is:

```text
n : Nat, acc : Nat, h : n ≠ 0
    ⊢ Nat.sub n 1 < n
```

The unguarded inequality for every `n` would fail at zero. Merely adding a pending label to that stronger, wrong obligation would not establish that the elaborator has produced a usable proof task.

**Proposed contract**

> A branch’s contextual construction includes its introduced variables and the evidence or discriminant equalities justified by reaching that branch. Generated typing, refinement, and termination obligations are closed over that context, including its dependencies. Those facts do not depend on whether the branch appears in a leading case tree or in a nested expression. Sibling branches do not inherit one another’s facts.

A proof-bearing conditional representation, an appropriate `dite` construction, or explicit branch-context evidence can implement this. A plain `ite` application is not sufficient merely because its arguments appear visually under then/else. Full dependent motives can remain staged; a constant runtime result type does not eliminate the need for branch facts.

**Completion tests**

- Print or inspect the generated obligation for the fixture and verify that its necessary nonzero hypothesis is present.
- Supply a completed, independently K-checked proof of that obligation. A handwritten proof using the available library is sufficient; do not wait for a general `omega` or I implementation.
- After discharge, the function’s pending set and its callers’ relevant pending sets are empty through the documented closure operation—not by suppressing the warning.
- Move an analogous recursive call into another supported expression position. The obligation retains the necessary branch context.
- For a match on a computed value, retain the equality needed to relate the matched constructor to the original scrutinee when an obligation depends on it.

**Completion means:** at least one nontrivial measured function is justified end to end. An unconditional reflexivity theorem about a function whose descent assumption remains open is not this test.

### R66 — Use realization-directed erasure, including the Bool/Decidable bridge

**Priority:** Before the new E projection becomes the default for typed functions.  
**Affected surfaces:** Slice 3.16 rules 3–4; primitive mapping, `decide` erasure, runtime representations, and projection tests.

**Source observation.** The design proposes erasing `decide p inst` to the erasure of `inst`. In the implemented evaluator, `init_bool_cell` and `dec_cell` select different wire entries: Bool constructors and Decidable constructors have distinct runtime tags. The shared second-constructor bit lets both drive an `if`, but does not make their complete values identical. The design also describes an inverse operator table and retains an E refusal for natural subtraction/division/modulo even though named `Nat.sub`, `Nat.div`, and `Nat.mod` entries already exist. [S1], [S9], [S10]

**Assessment.** A bridge that works only as an `if` condition can still fail when a Boolean is returned, pattern-matched, stored, or passed through another operation. The proposed simplification needs a representation argument, not just agreement about truth.

**Proposed contract**

> Erasure selects a supported realization of the resolved logical operation at its instantiated types and representations. A conversion may be erased as an identity only when the participating runtime representations and their permitted observations justify that identity. Otherwise erasure emits a conversion or reports unsupported lowering. Source spelling does not select a second meaning after elaboration has resolved the logical declaration.

For `decide`, there are two acceptable initial designs:

| Design | Obligation |
|---|---|
| **Explicit decision-to-Bool conversion** | Map the selected decision representation to the expected Bool representation, preserving the result and boundary contract. |
| **Shared representation** | Establish a common two-valued runtime representation and consistently use it for the affected constructors, matches, primitive results, and boundaries. |

The explicit conversion is the smaller initial change. Sharing can be an optimization later. Returning the Decidable tag unchanged before either contract exists is not the proposed acceptance criterion.

**Operator selection.** Formulate the current table as the first implementation of a realization registry, not a presumed one-to-one inverse of source syntax. A minimal entry records:

```text
resolved logical operation and instantiation
required argument and result representations
executable operation
applicability conditions and correspondence evidence or stated trust
```

This can remain a small static table. It need not become a plugin system or a new execution layer.

For the new frontend, once `-` at `Nat` resolves to `Nat.sub`, select its saturating implementation. Likewise use the existing total natural division and modulo implementations when those are the resolved operations. The former obstacle arose from preserving the legacy E-first spelling’s different behavior; it should not automatically survive the direction change. Keep legacy bootstrap execution faithful to its stated semantics rather than silently changing old sources.

**Completion tests**

- A `decide` result works when returned, put in a Boolean field, matched against Bool constructors, passed to a Boolean operation, and used as a condition.
- Test both truth values; a shared condition bit alone cannot satisfy the test.
- On the new frontend, natural subtraction at `1 - 2` produces the logical `Nat.sub` result, and natural division/modulo at a zero divisor use their selected total operations.
- Int subtraction remains distinct; operator choice does not depend on whether the expression was written in a `fn`, `def`, or theorem context once its expected types and scope are fixed.
- A representation or primitive mapping change invalidates the corresponding realization binding rather than reusing evidence by spelling.

### R67 — Preserve partial outcomes and distinguish tentative from validated assignments

**Priority:** Establish the shared contract before I and search take dependencies on it.  
**Affected surfaces:** `kw.shard`, `unify.shard`, the elaborator’s results, future goal-graph APIs.

**Source observation.** `kw.shard` converts every failure of K’s `whnf`, `infer`, `is_prop`, and `is_def_eq` wrappers into `None`, including exhaustion and refusal of open terms. `unify.shard` exposes `UYes`/`UNo`, and its depth limit returns the same negative result as other unsuccessful attempts. In `assign_checked`, a failed type inference can still install an assignment and return `UYes`; the comment explicitly relies on K checking the completed term later. [S5], [S6]

**Assessment.** That is permissible speculation inside an untrusted producer. It is not a validated-partial-judgment contract. A future search operation must know whether it obtained a checked assignment, a tentative assignment with a remaining typing constraint, a blocked comparison, an unsupported attempt, or an exhausted computation.

Final K checking prevents a bad completed proof from being accepted under its normal rules. It does not repair branches a search engine incorrectly discarded before producing any proof.

**Proposed contract**

> Shared construction operations preserve the distinction among successful validation, unresolved constraints, an unsuccessful or rejected attempt, and resource exhaustion. A speculative assignment is retained with the typing and scope constraints still owed. It is not reported as a validated assignment until those constraints have been checked. No caller may interpret an unsupported, blocked, or exhausted operation as mathematical inequality or an empty solution region.

A schematic result vocabulary is:

```text
Solved(result, validated_patch)
Deferred(candidate_patch, constraints, blockers)
Rejected(subject, reason)
Exhausted(resource, site)
```

An unsupported operation can have its own variant or be an explicit reason; do not erase that distinction into a generic type mismatch. `Rejected` identifies what failed—the requested elaboration step or candidate assignment, for example. It does not automatically mean that every alternative proof or filling is impossible.

**Transactions.** Keep the existing rollback intent. A failed operation leaves the prior workspace unchanged unless it returns a patch the caller explicitly accepts. A deferred patch preserves its unresolved constraints, rather than presenting tentative work as a checked update. K’s failure reason must survive the first wrapper above K.

**Completion tests**

- A low-budget operation reports exhaustion; increasing the budget can make the same operation succeed without contradicting a stored rejection.
- An operation blocked on a hole identifies that dependency and can be retried after assignment.
- An assignment whose type cannot yet be validated remains tentative with its constraint. It either validates later or fails without contaminating the accepted state.
- A failed speculative operation leaves the original assignments and constraints unchanged.
- Finalization refuses outstanding constraints and metavariables even when a tentative assignment made intermediate traversal possible.

**Scope control.** Do not replace every private `Option` helper indiscriminately. Repair the common API boundaries and any local helper whose information loss makes the public result impossible to reconstruct. The initial implementation can still be deliberately incomplete.

### R68 — Preserve a hole’s creation context across binder closure

**Priority:** Before the metavariable API becomes the permanent substrate for I and search.  
**Affected surfaces:** `MDecl`, `MCtx`, `mc_drop_fvars`, instantiation and binder-closing operations.

**Source observation.** `MDecl` stores an expected type and a list of allowed local IDs. `mc_drop_fvars` removes closing locals from each unresolved metavariable’s permitted context; its comment identifies this as refusing cases that would need delayed assignment. [S5]

**Assessment.** This is a defensible prototype restriction, but not the contextual-hole capability the architecture intends. The lifetime of an elaborator traversal is not the lifetime of a hole’s declared context.

For example, a tool should be able to construct:

```text
?h : [x : Nat ⊢ Nat]
left  := fun x => ?h[x ↦ x]
right := fun y => ?h[x ↦ y]
```

leave those binders, solve another constraint, and then assign the one shared template `h := x + 1` in its declared context. Both occurrences instantiate it consistently. A hole declared with no access to `x` must still be unable to capture `x`.

**Proposed contract**

> A contextual hole has a stable creation telescope and expected type. Each occurrence is interpreted through an explicit or equivalently represented instantiation of that telescope. Closing a syntactic traversal does not remove legitimate dependencies from the hole. Assignments are checked in the hole’s declared context and instantiated capture-avoidantly at occurrences. Scope restriction, when useful, is an explicit justified operation rather than a side effect of leaving a binder.

Possible implementations include contextual substitutions, raising metavariables over their permitted locals, or delayed abstraction/assignment. Reuse the existing `Expr` and metavariable representation where practical; no new trusted inference rule is required. K continues to check completed declarations with unresolved holes rejected.

The context must retain the dependency information needed to interpret its types, not just reusable integer labels. Branch identity and fresh-name lineage must prevent unrelated contexts from being confused. The common workspace should support this once, rather than requiring separate repairs in elaboration, termination, and proof search.

**Completion tests**

- Leave a binder with a contextual hole unresolved, then fill it using its permitted local. The closed result checks.
- Reuse one contextual template under alpha-renamed binders and confirm correlated filling at both occurrences.
- Fill a hole with a dependent expected type, such as a proof obligation `[n : Nat ⊢ n = n]`, after leaving the traversal that introduced `n`.
- Refuse a genuinely out-of-context capture and an assignment cycle.
- Fork a workspace, assign a hole differently in each branch, and ensure neither assignment silently changes the other branch’s construction.

**Completion means:** the minimal shared contextual contract is exercised before I depends on it, not that a production synthesis engine or arbitrary higher-order unification is complete.

### R69 — Make obligations contextual first, with acyclic discharge and explicit dependencies

**Priority:** Alongside the common workspace and the first completed measure obligation.  
**Affected surfaces:** Generated `f.dec_N`, the fourth policy class, future `fulfills` processing, forward-reference retries, module instantiation.

**Source observation.** Measured definitions use generated axiom-kind declarations as a fourth policy class, separate from permitted axioms and view parameters. Their names are retained in pending/dependency records; the document says they are discharged by proof, never by policy. Forward references are parked and retried after definitions to a fixed point. The intended module-instance path substitutes into P and rechecks, rather than re-elaborating consumers against hidden implementation bodies. [S4], [S10], [S11]

**Assessment.** These are useful mechanisms. They should be encodings and operations over one contextual construction system, not the beginnings of three independent systems for elaborator holes, termination obligations, and I proof holes.

**Proposed contract**

> A generated proof obligation is primarily a contextual goal owned by a construction. Its kind determines its discharge policy. A parameterized or axiom-kind declaration may encode it for provisional checking, but does not turn it into an allowed ambient axiom. Discharge records checked evidence and its transitive dependencies, and finalization verifies that no unresolved obligation is justified through a circular chain of assumptions. Pending dependencies remain visible through callers and checked module instances.

Keep the existing distinctions: an implicit type metavariable, a termination proof obligation, a module law, and a user-permitted axiom do not become interchangeable because they share bookkeeping. One substrate should preserve their different rules.

**Acyclicity.** Evidence for `f.dec_1` must not discharge it when the evidence depends on `f.dec_1`, directly or through another obligation, theorem, or definition. Similarly, `h1` depending on `h2` and `h2` depending on `h1` leaves both unresolved. K’s check under an assumption-bearing environment does not by itself establish that a higher-level assumption-discharge process is acyclic.

Use an explicit substitution/finalization route for the resulting proof objects. Preserve the module stance: the consumer’s reasoning environment remains the interface, and instance construction operates on checked evidence, not a rerun of tactics against hidden bodies. Any permitted extra assumptions used by the supplied evidence remain in the resulting acceptance account.

**Dependency-directed scheduling.** Record whether work is waiting for a declaration, an assignment, a proof, an auxiliary matcher, or executable support. The existing retry loop can remain while workloads are small. As it grows, use those dependencies to wake affected tasks rather than repeatedly attempting every parked definition. A true recursive component is not merely a forward reference that has been retried many times.

**Completion tests**

- Direct and two-obligation circular discharge attempts do not close a pending set.
- A valid independent proof closes its obligation through the documented construction, and dependent artifacts reflect the change without losing other assumptions.
- A caller inherits the pending obligations it actually needs; unrelated work does not acquire them solely through sharing a loader session.
- Reordering independent declarations preserves resolved results; waiting on a declaration and waiting on a proof remain inspectable as different causes.
- A module-instantiated proof retains the assumptions of its supplied evidence and does not gain access to private defining equations during proof production.

### R70 — A bootstrap fallback may weaken guarantees, not reinterpret the source

**Priority:** Before the frontend flip.  
**Affected surfaces:** Slice 3.16 rule 6, `DfSkip`/obstacle handling, build/run status, retained E-first execution.

**Source observation.** The new plan retains the E-first path for toolchain sources whose signatures lack L identities. It also permits a body-level obstacle to send an otherwise eligible whole function back through that path. The design’s own motivation is that the paths currently differ in type reconstruction, operator handling, and name resolution. [S1], [S3]

**Assessment.** Retaining bootstrap execution is practical and consistent with the project’s earlier decisions. Automatically rerunning a partially elaborated body through another interpretation is the risky part. An unrelated missing realization must not change which constructor an ambiguous name denotes or which arithmetic operation an already-resolved subexpression uses.

**Proposed contract**

> A fallback may retain an explicitly identified runnable construction when its stronger logical or realization guarantee is unavailable. It preserves the semantic choices already resolved for that construction. If a meaning-preserving fallback is unavailable, the system reports the obstacle rather than reinterpreting the source through another resolver or operator policy. A genuine type error or K rejection does not become successful runnable-only admission through fallback.

Where possible, retain the resolved partial construction and report which projection is unavailable. Where the bootstrap subset still requires a separate source path, state the selected execution capability in the build inputs and result. This is not a request to reinstate the retired directory-selected profile or a permanent alternate dialect.

A build that requires an admitted definition or completed realization must refuse a merely runnable result. Development can deliberately accept the weaker status and execute it under the appropriate budget and trust account. The status must be machine-readable, not only a warning embedded in a log.

**Completion tests**

- Introducing an unrelated unsupported callee does not change the meaning of already-resolved operators, literals, or constructor citations elsewhere in the body.
- A typed-route error remains a refusal, not an E-first success.
- The legacy toolchain route remains usable, with its coverage and selected path explicit and its existing conformance tests retained.
- A request for a completed realization rejects a runnable-only result; a deliberate development run can accept the weaker status without promoting its guarantees.

**Deletion criterion.** Keep the E-first implementation only for a named set of remaining consumers. Port them and delete the redundant route when its coverage obligation is met. Avoid replacing the removed profile flag with scattered per-feature semantic switches.

### R71 — Gate the refactor on semantic connections, not permanently frozen elaboration hashes

**Priority:** Revise the 3.16 gate before using it to judge the new architecture.  
**Affected surfaces:** The three planned landings, calc and std fixtures, artifact revisions, later R60/I consumers.

**Source observation.** The design calls for byte agreement between the E projection and the existing classifier program, unchanged equation names and logical-value hashes for currently defined functions, all twenty-five calc spec functions defined, and the existing full gates remaining green. [S1]

**Assessment.** Exact comparisons are valuable migration alarms. They should not permanently constrain how a correct elaborator expresses recursion, shares matchers, or constructs proof terms. A justified refactor may alter a generated body’s content while preserving the intended operation and establishing the necessary relationship.

**Proposed contract**

> Use exact agreement as the first migration check where the intended representation is unchanged. Investigate every mismatch. An intentional representation or elaboration change is accepted only with its stated semantic comparison or checked correspondence, updated revision identity, and reviewed migration of affected references. Imported declarations pinned by the foundation retain their exact identities. Generated artifacts are not falsely declared unchanged merely to preserve a hash or a green test.

This does not authorize changing expected outputs until tests pass. Nor does it require changing all existing hashes. Preserve them wherever that comes naturally; permit justified differences where the new architecture is genuinely different.

**Semantic coverage.** The next three consumers should become simpler to add through the same interfaces: a branch-local refinement, a dependent-safe record update, and a contextual proof/search operation. The first landing need not implement all three. Their required context and projection capabilities should not force three new special representations.

**Completion tests** are the integrated battery in §4. The existing calculator differential, K seal tests, frontend parity within its stated domain, route comparisons, and T0 replay remain regression infrastructure. They supplement rather than replace the new boundary tests.

## 4. Integrated acceptance battery

These are proposed fixtures, not measurements or executed reproductions. Map them to existing suites; no separate test framework is needed.

| Fixture | Minimum result | Requests |
|---|---|---|
| **G1 — One source meaning** | The same scoped expression in a `fn`, `def`, and theorem context resolves the same declarations and types, including numerals, constructor ambiguity, and nested matches. Exact internal syntax may differ only through a specified elaboration relationship. | R63, R66, R70 |
| **G2 — Matcher identity and forcing** | Tampered metadata fails correspondence; a missing descriptor cannot cause generic eager lowering; only the chosen alternative runs, with specified scrutinee evaluation. | R64 |
| **G3 — Bool beyond `if`** | Both values of `decide` survive return, storage, Bool pattern matching, calls, and conditional use. | R66 |
| **G4 — A completed measured definition** | The generated descent goal contains its branch hypotheses; a real proof closes it and its dependent pending account. Test another supported syntactic position too. | R65, R69 |
| **G5 — Contextual delayed filling** | Fill a shared hole after leaving its binder, including a dependent expected type; reject foreign capture and preserve branch isolation. | R68 |
| **G6 — Partial outcomes and rollback** | Exhaustion, blocking, tentative assignment, and rejection are distinguishable. Later evidence or a larger budget can resume work; failure leaves the prior workspace intact. | R67 |
| **G7 — Acyclic obligation closure** | Direct and indirect circular discharge fail; independent discharge succeeds and retains all other assumptions. | R69 |
| **G8 — No reinterpretation on fallback** | An unavailable projection changes the result status, not the meaning of resolved subexpressions; strict admission requests reject weaker outputs. | R70 |
| **G9 — Retained construction and views** | A pre-definition survives handoff and supported fork/merge with its descriptors; consumer proof production still sees only the interface; a revision mismatch is explicit. | R63, R64, R69, R71 |

For forcing tests, distinguish object-language behavior from an artificial test budget. A low-budget or deliberately failing development candidate is a useful probe of which branch ran; it is not automatically a total admitted program. For semantic equivalence of accepted programs, state the observation and resource assumptions actually promised.

## 5. Integration into the existing slice plan

### Before landing 1: settle the shared contracts

Agree on the minimal pre-definition record, matcher description binding, and the workspace’s outcome/context contract. This should be a small amount of named data and explicit invariants, not a new general framework. Record the two projection obligations and their current execution/translation trust.

### Landing 1: matchers and elaboration

Build matchers and `match` in L terms as planned. Add descriptor identity and branch-context tests now, while the data structures are new. Preserve K’s failure information in the shared wrappers before new clients depend on the collapsed `None`/`UNo` interface. Begin the minimal contextual-hole test rather than freezing the binder-dropping behavior as the I API.

### Landing 2: executable projection

Implement realization selection and an explicit Bool/Decidable conversion or its justified shared representation. Retain the proposed E comparison against current calc/std functions. Add forcing and returned-Boolean tests, which byte agreement on the current easy examples would not necessarily cover. Permit the already-supported natural operations once the new frontend has resolved their identities.

### Landing 3: frontend flip

Re-point the recursion machinery, delete the upward `tr` route for the migrated functions, and constrain the retained E-first route to its explicit consumers. Complete one measured-recursion proof using existing explicit evidence facilities rather than waiting for the general tactics. Keep provisional definitions and realizations distinguishable from finalized artifacts.

### Slices 3.17–3.18 and I from 3.19

Use the same construction/context interfaces for the scheduled porting facilities, text/byte realizations, deriving, and I. Do not create a second hole model for I or a separate proof-obligation context for record updates. Keep the view-substitution stance and the planned two-instance gate. Optimize retry scheduling and storage only after the first workloads make their cost visible.

### Work explicitly not requested here

No new logical foundation; no dynamic closure runtime; no change to the chosen module opacity; no complete higher-order unifier; no full search engine before I; no certification of the Rust bootstrap as a prerequisite; and no unconditional ban on temporary runnable code. The changes are meant to reduce future special cases, not expand the trusted kernel or require every later feature immediately.

## 6. Suggested normative replacement passages

These consolidate the requests into a few paragraphs suitable for the next iteration of `LANGUAGE.md`. They are **proposed wording**, not quotations of the current contract.

### 6.1 The pre-definition and its projections

> S is elaborated once into a contextual pre-definition whose body uses `Expr` and whose associated data records its telescope, recursive references, matcher descriptions, constraints, obligations, and resolved dependencies. The logical projection justifies recursion and produces declarations for K; the executable projection erases and selects supported realizations. Both identify the construction revision they consume. Their common input does not substitute for the correspondence connecting execution to the admitted logical meaning.

### 6.2 Matchers and branch evidence

> Matcher descriptions are bound to their exact logical declarations and provide the supported executable reconstruction. They preserve first-match order, discriminant evaluation, alternative forcing, and the contextual evidence available in each branch. A missing or mismatched description is unsupported until reconstructed or validated. Branch-local proof and termination obligations are generated from the same contextual representation in every syntactic position.

### 6.3 Partial construction

> Construction APIs preserve solved, deferred, rejected, and exhausted outcomes. Tentative assignments carry their remaining validation constraints. A hole retains its declared context across traversal boundaries and is instantiated capture-avoidantly at each occurrence. Failure or incompleteness of a construction procedure is not evidence that a proposition is false or a search region empty. Finalization discharges constraints and proof obligations without circular dependence, and K checks the resulting completed declarations.

### 6.4 Realization selection and fallback

> The executable projection selects representations and operations from the resolved logical identities and their applicability contracts, not from an inverse of surface spellings. Bool/Decidable conversion is retained unless representation equivalence justifies removing it. A bootstrap fallback may preserve a runnable candidate with weaker guarantees; it may not reinterpret resolved source choices or turn a typed failure into successful admission. Requests for stronger guarantees inspect the candidate’s unresolved obligations and available projection evidence.

## 7. Requested response from Fable

For **R63–R71**, mark **accept / amend / defer**, identifying the affected contract paragraph, implementation landing, and fixture that will establish completion. A deferral should name the narrower supported guarantee and prevent callers from depending on the deferred capability.

The key decisions are whether to adopt the contextual pre-definition boundary now; how to bind and validate matcher descriptions; how to implement the Bool/Decidable representation bridge; and how far the current `MCtx` contract is strengthened before I becomes its consumer. The exact record and constructor names remain implementation choices.

**Overall verdict:** the 3.16 direction is the right correction. Judge it not only by getting all twenty-five calc spec functions defined, but by making the next refinement, record, and proof-search features share one account of meaning and partial knowledge.

## Source map

All repository references below are pinned to `3062f23a92e1dd2b1ded2b83f722e3275fefe182`. They identify source evidence; the recommendations and counterexamples are the review’s analysis. Function names are included because the memo remains useful when later edits move line numbers.

| Ref | Source and relevant content |
|---|---|
| [S1] | `v3/LANGUAGE.md` §8.4, slice 3.16 design, its motivation, eight rules, landings, gates, and named risks. |
| [S2] | `v3/README.md`, Phase 3 status: the one-E migration, K seal, implemented 3.13–3.15 work, reported gates, and 3.16 design-only status. |
| [S3] | `v3/kernel/define.shard`, `DfRes`, `df_obstacle`, `define_fn`, `define_equations`, and the existing definition/recursion path. |
| [S4] | `v3/kernel/realize.shard`, `Tr`, `DefRec`, `tr_self_wf`, `tr_minors`, `tr_restore`, and the current translation/obligation machinery. |
| [S5] | `v3/kernel/unify.shard`, `MDecl`, `MCtx`, `mc_drop_fvars`, `URes`, `unify_go`, `assign_checked`, `infer_light`, and fallback behavior. |
| [S6] | `v3/kernel/kw.shard`, the K-wrapper contract and `w_whnf`, `w_infer`, `w_is_prop`, `w_is_def_eq`. |
| [S7] | `v3/kernel/define.shard`, `df_term`, `df_if`, `df_branches`, and obligation scopes through the leading definition path. |
| [S8] | `v3/pins/loader/define_measure/main.shard`, the pending measure example and conditional dependent theorem. |
| [S9] | `v3/kernel/ev.shard`, separate Bool/Decidable wire entries, `init_bool_cell`, `dec_cell`, and primitive execution. |
| [S10] | `v3/LANGUAGE.md` §§6.4, 7.5 and 8.4’s earlier slice rules: primitive identities, pending realizations, measured definitions, and forward-reference processing. |
| [S11] | `v3/LANGUAGE.md` §8.5 and §9: consumer view opacity, checked instance construction on P, and the fourth obligation class. |

[S1]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/LANGUAGE.md#L2013-L2179
[S2]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/README.md
[S3]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/kernel/define.shard#L1-L185
[S4]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/kernel/realize.shard
[S5]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/kernel/unify.shard
[S6]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/kernel/kw.shard#L1-L95
[S7]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/kernel/define.shard#L665-L786
[S8]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/pins/loader/define_measure/main.shard
[S9]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/kernel/ev.shard
[S10]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/LANGUAGE.md
[S11]: https://github.com/computer-whisperer/shard/blob/3062f23a92e1dd2b1ded2b83f722e3275fefe182/v3/LANGUAGE.md#L2180-L2260
