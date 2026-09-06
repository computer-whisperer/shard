# Follow-up memo for Fable

## Borrow Lean's mathematics; choose Shard's engineering contracts

**Status:** REVIEW DRAFT v0.1 — proposed amendments, not ratified policy.  
**Date:** September 6, 2026.  
**To:** Christian Balcom and Claude Fable.  
**From:** GPT-6 Astra Pro.  
**Reviewed baseline:** `docs/FOUNDATION.md`, Fable DRAFT v0.2, at commit `d6b25f10a401d72e8be476e277cbe44a08eed818`; blob `1885a7cffb271e9c8c44c9a892149e882f1609ee`. [F]  
**Relationship:** Continues `SHARD_FOUNDATION_FEEDBACK_v0.1.md`. R1–R8 are substantially addressed in the reviewed revision. This memo introduces R9–R16 and focuses on the next iteration of §§5.4, 6–7, and especially §13. It is not another foundation replacement proposal.  
**Evidence boundary:** Project observations refer to the pinned revision, not the earlier uploaded v0.1. External technical checks are identified separately and listed at the end. Examples, contracts, and tests below are proposals, not implemented Shard syntax, completed proofs, or measured performance results. The pinned revision is the review target; this memo does not assert that it remains the latest repository state.

---

## 0. Recommendation

Proceed with the agreed architecture. Preserve K as an E program, L as the mathematical language represented in the toolchain's data, the reference-exact initial closed-proof fragment, direct Rust bootstrap execution, compiled applications, and the sibling `v2/` implementation plan. The source-level lambda profile already agreed in §5.3 is a good compromise. No interpreted-only `fn` category is requested here. [F]

The next revision should distinguish three policies:

1. **Borrow:** the selected logical foundation, its useful mathematical abstractions, proof terms, and existing techniques that fit Shard.
2. **Refuse as Shard defaults:** silent changes to a requirement's meaning, unaccounted implementation substitutions, proof acceptance without assumption policy, and speculative search operations whose state or logical significance is unclear.
3. **Experiment without blocking v1:** evidence-directed conversion, expected-type-directed checking, and eventually alternative presentations of the foundation with fewer implicit equalities.

A design can be appropriate for Lean and inappropriate for Shard's workloads. Describe those tradeoffs precisely instead of categorizing every difference as a Lean soundness bug. Several statements in the current §13 should also be updated against current documentation.

**Two correctness-boundary edits deserve agreement before ratification:** specify how a particular native evaluation supplies checkable evidence, and stop identifying the checking algorithm with the declarative logic. The remaining items refine interfaces and tests within the existing phases; they do not require another architecture cycle.

### Decisions requested

| ID | Amendment | Main locations in FOUNDATION.md | Timing |
|---|---|---|---|
| **R9** | Bind each evaluated result to replayable evidence; a general evaluator theorem is not the missing instance proof | §§5.4, 13.2; T1/T8 | Specify now; gate the evaluation feature when enabled |
| **R10** | Separate declarative validity, the pinned checker, and its operational outcomes | §§3.1–3.4, 13.4; T0/T7 | Before ratification |
| **R11** | Freeze resolved requirements; distinguish reconstruction from consequential selection during inference | §§6.1–6.3, 13.1 | First elaborator contract |
| **R12** | Make assumption policy and distinct semantic/evidence identities intrinsic to artifact acceptance | §§4.1, 7–8, 13.2 | First persistent environment and replay path |
| **R13** | Give speculative metaprogramming transactional state and precise failure meanings | §7; T4/T7 | First open-workspace implementation |
| **R14** | Separate mathematical totalizations from application-facing error policies | §§5, 12–13 | Primitive crosswalk and first library |
| **R15** | Permit evidence-backed replacements under explicit relations; preserve compiled deployment without banning useful source abstractions | §§5.3–5.7, 13.1–13.2 | Existing realization and lambda gates |
| **R16** | Instrument conversion and retain a bounded research path without changing v1 rules | §§3.5, 10, 13.3–13.4 | Measure first; optional experiments later |

---

## 1. R9 — A proved evaluator still needs evidence for a particular result

**Source observation.** Section 5.4 proposes running `ev` natively and citing its correctness theorem; §13.2 says that this adds no trust. The general idea is useful, but the description omits the evidence that this particular invocation produced this particular value. [F]

Suppose the relevant theorem is schematically:

```text
realizes_f :
    for all arguments and results,
    Runs_E(program_f, arguments, result)
        -> ResultRelation(result, logical_f(arguments))
```

Receiving a native value `v` does not itself construct a term of type:

```text
Runs_E(program_f, arguments, v)
```

Changing the premise to `ev(program_f, arguments) = v` moves the same obligation; it does not discharge it. The theorem about the evaluator and evidence for an individual execution are different inputs to the argument.

### Recommended initial implementation

Prefer **proof-producing evaluation**: compute the result while constructing shared L evidence from admitted equations. K checks the resulting evidence. A compact execution witness plus a proved witness checker is another legitimate design. That route must also explain how successful witness checking is established without repeating the same raw-native-result shortcut one level down.

A specially accelerated implementation of a K checking operation is possible, but it has an explicit validation contract and execution-trust treatment. Do not silently install a new result oracle while describing the closed checker as reference-exact.

This does **not** require proof traces for every machine operation used to execute K. Rust remains an explicitly trusted executor. The distinction is between executing the agreed checking algorithm and extending what that algorithm accepts on the strength of a tactic's unchecked output.

Bind evaluation evidence to the resolved program body, arguments, result encoding, environment, and relevant realization equations. A correct equation named `f.eq_1` is insufficient if a stale executable view now computes a different function.

**Replacement for the final bullet of §5.4:**

> A theorem relating E execution to L meaning supports proof-producing or certificate-checked evaluation. Each invocation supplies independently checkable evidence that the specified computation yields the returned value. The general soundness theorem then connects that evidence to the requested L proposition. A native value alone is not a proof. The execution route used to run K remains recorded separately.

**Technical comparison.** Current Lean documentation describes `cbv` as proof-producing computation and `decide_cbv` as capable of handling well-founded definitions without trusting the code generator. These are useful references, not a Shard dependency or a correctness proof for the proposed E evaluator. Do not frame Shard's opportunity as fixing a feature Lean categorically lacks. [X3]

**Tests:** tamper with the returned result; swap the executable body while preserving the old equations; and replay the generated evidence cold without rerunning the original tactic. The first two must fail, and an untampered supported case must succeed. Keep progress/termination obligations distinct from successful-result correctness: `f x = f x` does not justify a looping implementation.

---

## 2. R10 — Keep the mathematics separate from the procedure that checks it

**Source observation.** Section 13.4 currently resolves incomplete conversion by making the algorithm the rule. That should be replaced, not carried into the normative foundation. [F]

Keep two specifications:

```text
Declarative:       Environment; Context |- term : type

Operational:       check(environment, declaration, limits) -> outcome
```

The central implementation obligation is that successful checking implies the declared judgment. The exact theorem needs the actual well-formedness and environment assumptions. This memo does not claim that obligation has already been proved for Shard.

Compatibility tests can pin an algorithmic profile and compare the supported inputs. They do not turn resource limits, search ordering, or a failed heuristic into mathematical definitions. Lean4Lean distinguishes the declarative account from its typechecking and conversion procedures, including limitations of negative conversion results. [X4]

A failed attempt to establish conversion is not automatically evidence that two terms can never be equal. Likewise, failing to synthesize a proof of P is not evidence for not-P. An implementation may provide a definitive rejection for a specified malformed input; it must not give every operational failure that stronger meaning.

The public API may retain `Rejected` for an unacceptable submitted certificate. Its diagnostic must distinguish malformed evidence, unsupported input, unresolved obligations, conversion not established, and exhaustion where relevant. A search consumer may prune on an actual justified incompatibility, not on the spelling of a Boolean return value.

**Replacement for §13.4:**

> Shard specifies the declarative judgments and, separately, the bounded checking procedure used by its initial compatibility profile. Successful checking must conform to those judgments. Algorithmic incompleteness, unsupported inputs, and exhausted resources remain explicit limitations. Neither a timeout nor failure of a conversion attempt is a general theorem of inequality or unsatisfiability. No completeness theorem is assumed from agreement with another implementation.

This separation also provides the stable specification against which a faster checker can later be justified. No change to v1's selected conversion rules is requested by R10.

Reference-exactness applies to the named supported foundation and initialization contract, not to forged primitive declarations or upstream bugs outside that domain. For accelerated Nat operations, validate the designated declaration identity, signature, and defining equations, or bind the shortcut to fixed admitted declarations. Test an unrelated same-spelled definition and an altered primitive body. Lean4Lean explicitly discusses validating primitive definitions before using numeric accelerators; Shard should retain this safeguard rather than inherit a name-based shortcut by imitation. [X4]

---

## 3. R11 — Inference may reconstruct meaning, not silently select a different task

The draft already rejects automatic creation of parameters from unknown identifiers. Retain that decision. Extend it into an explicit **resolved-requirement contract**, rather than relying only on surface binder discipline. [F] (§6.2)

Lean documents automatic implicit parameters and selection among competing instances using priority and declaration order. These are deliberate conveniences. Shard should choose which conveniences it wants rather than inheriting their defaults accidentally. [X1, X2]

Distinguish:

**Reconstruction:** infer information determined by the declared inputs and expected result, such as a list element type.

**Selection:** choose among semantically consequential alternatives, such as an ordering, numeric interpretation, implementation package, or coercion route.

Permit generous reconstruction. For consequential selections, use explicit scopes or parameters, and record the resolved choice. Do not require proof that all alternative instances are equivalent, and do not promise to decide global inference uniqueness. A practical initial design resolves under a declared scope, exposes the chosen identities, and refuses material ambiguity when the contract requires a choice.

A resolved requirement should include its proposition, bound parameters, referenced declarations, selected computational instances/coercions, and relevant policy/environment dependencies. A source edit or import change that alters these creates a different requirement and must be visible as such.

The proof-solving agent does not get to redefine the target while solving it. This does not prohibit synthesis: a task can deliberately contain unknown implementations. Freezing that task freezes the holes' scopes, expected types, admissible domains, and success relation; filling those declared holes remains the job.

**Proposed addition to §6.2:**

> Concise source and fully resolved meaning are separate views. Inference may reconstruct omitted details, but unknown identifiers do not become new parameters by accident, and ambient imports do not silently retarget a frozen requirement. Consequential instance and coercion selections are scoped, inspectable, and part of the resolved task. Explicitly declared synthesis holes remain fillable without changing that task.

Do not overcorrect into maximum verbosity. Typeclasses, local inference, and broad `simp`-style discovery remain useful. A successful search should leave checked evidence and actual dependencies; deterministic replay should not require rediscovering the same proof with the same tactics.

**Tests:** a misspelled requirement identifier fails; an import that changes a selected ordering is reported as a requirement change; changing only a proof strategy leaves the resolved requirement unchanged; an explicitly declared synthesis hole remains solvable.

---

## 4. R12 — Make acceptance a complete answer about the requested artifact

The question is not only whether K accepted a term. Artifact acceptance must also establish that the term proves the requested proposition under the permitted assumptions and concerns the intended executable realization.

Retain separate records for:

```text
requirement and semantic dependencies
public interface
implementation and executable view
particular evidence and logical assumption closure
compiled realization and target observations
checking engine, version, and execution dependencies
```

These need not become a new package-management framework. They can begin as fields and immutable references in the existing environment and artifact manifests.

Axiom-policy checking is mandatory at the relevant acceptance boundary. Open construction and imported hypotheses remain useful, but a pending proof must not become an accepted fact merely because a dependency file exists. Final closure examines reachable obligations, including those hidden through types, substitutions, or draft references.

Two proofs of one proposition may have different assumption sets. Proof irrelevance in L is not permission to discard that distinction from the evidence record. Conversely, changing only a proof body need not invalidate every abstract client. It may require rechecking acceptance policy without rebuilding unrelated machine code.

A private implementation change can leave abstract client proofs valid while invalidating an inlined executable. Avoid both extremes: trusting stale realizations and rebuilding the entire system after every evidence-only change.

**Technical comparison.** Lean already stores proofs in `.olean` files and documents transitive axiom inspection. Its successful editor status alone is not a complete check of dependency assumptions. Shard's improvement should be integrated artifact acceptance and finer dependency accounting, not the claim that Lean has no proof objects or caching. [X5]

**Tests:** prove the same proposition once under permitted assumptions and once using a prohibited axiom. Mathematical statement identity may remain the same, but the second artifact must fail policy. Change an implementation without changing its interface and check that abstract proofs and prepared/inlined execution invalidate differently and correctly.

---

## 5. R13 — Speculative operations need transactional state and honest conclusions

The native-hole contract in the current §7 is a substantial improvement. Add one operational invariant before multiple search tools depend on it:

> An unsuccessful speculative operation leaves the caller's semantic workspace unchanged unless it returns an explicit state update that the caller elects to commit.

An operation may return a useful partial assignment with blocked obligations. That is not forbidden; it must be explicit, scoped, and committed deliberately. A thrown failure or timeout must not leave a hidden assignment behind.

A schematic interface is:

```text
attempt(snapshot, request, limits)
    -> Completed(patch, evidence)
     | Blocked(obligations, proposed_patch)
     | Invalid(reason)
     | Exhausted(resource)

commit(snapshot, checked_patch) -> new_snapshot
```

The exact API is open. Persistent values and rollback journals are both possible implementations. The invariant applies to semantic workspace state; it does not magically roll back arbitrary filesystem or network effects. Keep those outside speculative logic, or explicitly account for them at the host boundary.

Context-sensitive caches bind at least the applicable environment, local telescope, universe assignment, metavariable assignments, and policy. Sharing immutable syntax is different from sharing an assignment. A proof or incompatibility result obtained in one branch cannot silently constrain another.

For search, maintain separate claims for candidate correctness, exact enumeration, region emptiness, representative replacement, and optimality under a cost model. A theorem about all valid fillings is valuable; it must state its constraints and does not establish that a filling exists.

**Tests added to T4/T7:** force failure after a tentative assignment; exhaust a comparison after partial progress; fork branches with different assignments to the same originating hole; and reuse a cache entry under a changed context. No hidden branch leakage is permitted. A failed or exhausted heuristic never counts as UNSAT.

This is canonical engine behavior implemented in E/meta. It does not put search policy, enumeration, or general unification into K's final closed-proof authority.

---

## 6. R14 — Mathematical totalization is not automatically an application's error policy

Lean's Nat subtraction saturates at zero, and Nat division defines a result at a zero divisor. These are coherent total mathematical operations. They should remain unchanged when imported or used to establish Lean parity. [X7]

They are not necessarily the best implicit API for every E application. A buffer-size calculation that saturates can conceal a failed bound; a totalized division can conceal a violated denominator invariant. Perfectly proving that code implements the mathematical convention does not establish that the convention was the product requirement.

Provide deliberately distinct interfaces where the distinction matters:

```text
div_total(x, y)              -- specified total mathematical convention
checked_div(x, y)            -- explicit success/failure data
div_nonzero(x, y, evidence)  -- a checked nonzero obligation
```

All can be total functions. This does not relax E's totality rule. Nor does it require every arithmetic operation to return an error object: proved preconditions may remove checks through an established realization.

**Proposed addition to §12's primitive crosswalk:**

> Record exceptional-input behavior as part of each operation's meaning. Preserve imported totalizations exactly. Application-facing checked or preconditioned wrappers are separately named operations with explicit bridge theorems; they do not silently reinterpret imported notation or migrated statements.

Apply the same principle to failed lookups, finite-width arithmetic, approximation, and resource exhaustion. The initial deliverable is a small set of conventions and boundary tests, not a second mathematical library.

---

## 7. R15 — Reject unaccounted replacement, not implementation freedom

Replace §13.1's prohibition on a `fn` being implemented elsewhere with a prohibition on **unjustified replacement of its computation**. Shard should encourage alternative implementations when their relation to the original is established.

A transformation should carry a source identity, destination identity, relevant preconditions, preserved relation, and evidence. The relation may be exact equality, a representation simulation, an error bound, or preservation of selected effects and resource behavior. These are not interchangeable Boolean forms of correctness.

For example, two approximate passes do not automatically retain the same error allowance as one pass. Their errors need a composition argument, including amplification and domain conditions where applicable. Likewise, `O(p) = O(q)` does not license replacement inside every surrounding context. Keep the root/spine/congruence distinctions already present in Shard's search work.

**Technical comparison.** Lean's `implemented_by` can substitute a runtime implementation without establishing equality to the logical one; its documentation distinguishes this execution risk from logical inconsistency. But `@[csimp]` is associated with equality theorems used in compilation and should not be categorized as the same unchecked mechanism. A theorem-directed rewrite is useful precedent; the missing end-to-end compiler and byte relationship remains separate. [X8, X9]

### Preserve the agreed lambda profile

Keep closed lambdas, named partial applications, and non-escaping capture as forms eliminated into first-order E. Captured runtime values remain data parameters rather than specialization keys. Preserve strict evaluation points and establish the applicable transformation relation; use propositional evidence where definitional equality is insufficient.

No interpreted-only `fn`, general closure ABI, or new HOF feature gate is requested. The eventual dynamic-tier wake condition should be a named workload with a demonstrated cost or compositional disadvantage from manual sealed variants—not a requirement to prove defunctionalization impossible. Possible encoding and worthwhile encoding are different questions.

Explicitly linking compiled E implementations of K, `ev`, or `meta/` remains valid application behavior. Silently inserting an interpreter because requested lowering failed remains invalid. This distinction preserves the embedding/ML use case without compromising the ordinary compiled-deployment promise.

---

## 8. R16 — Improve conversion deliberately, without moving the first foundation's goalposts

Retain the agreed v1 closed-rule inventory. Do not make cumulativity, removal of proof irrelevance, or a different quotient theory prerequisites for implementation.

At the same time, do not declare the inherited conversion design permanently optimal. Small explicit proofs can conceal expensive computational comparisons. The relevant optimization target is total proposal, checking, storage, and repair cost—not proof text length alone.

### First experiments: unchanged judgments

**Conversion plans.** Let a producer supply selected unfoldings, explicit instantiations, and shared intermediate results. Validate them with existing rules. A theorem of propositional equality is not permission to change definitional equality. A plan may increase evidence size; measure that cost.

**Expected-type-directed checking.** Prototype using known expected types to guide checking of explicit evidence where justified. Lean4Lean describes an inference-forward kernel design; that does not require Shard to copy the same implementation algorithm. Any alternative still needs the same acceptance justification and must be evaluated against the compatibility profile. [X4]

**Measurement.** Use one representative compiler certificate, one mathematical theorem, and one contextual-search operation. Count unique evidence nodes, conversion work, cold/warm costs, and repair after a small change. No performance gain is assumed.

### Later research: changed presentation of the theory

A separate experiment could ask whether selected implicit equalities are better represented by explicit transports or certificates. Lean4Less provides a research example of replacing some definitional equalities by explicit casts in a translation. It does not establish a full Shard design or a cheap translation of every future Lean development. [X10]

A departure changes the validation route; it does not necessarily destroy all external comparison. The options include direct replay for an unchanged fragment and translated replay where a faithful translation exists. Price the translation, assumptions, proof expansion, and validation work explicitly.

No such departure is ratified here. Keep it out of the initial implementation's critical path. The distinction between declarative rules and checking algorithms in R10 is what makes this experiment discussable without confusing a performance patch with a logical change.

---

## 9. Correct the comparison with Lean before using it as design rationale

The following are narrowly sourced corrections, not reasons to copy Lean wholesale. Live documentation can describe features absent from the release eventually pinned at phase 0; reconcile that separately.

| Current framing to remove or qualify | Replacement |
|---|---|
| Well-founded computation is necessarily stuck unless native execution is trusted | Current `cbv`/`decide_cbv` provide proof-producing alternatives. Study them while designing E-specific evidence. [X3] |
| Lean retains only tactic scripts | Compiled environments contain proofs that can be replayed. Shard's opportunity is coherent, fine-grained requirement/evidence/realization identity. [X5] |
| Current Lean privacy is only name mangling | Current opt-in modules have public/private scopes and separately controlled body exposure. Shard should still specify its own well-formed view and matching contracts. [X6] |
| `partial`, `unsafe`, `implemented_by`, and `csimp` are equivalent logical holes | They have different roles and safeguards. Keep Shard's chosen E discipline, but distinguish logical validity from runtime replacement and theorem-directed transformation. [X8, X9] |
| Deterministic elaboration eliminates timeout-sensitive behavior | Version, inputs, limits, and execution conditions matter. Structured exhaustion and replayable evidence are the guarantees; fast completion is not. |
| A Shard-written checker is automatically more trustworthy than a C++ checker | E ownership provides integration, dogfooding, and an eventual certified execution route. Language choice alone is not a correctness proof. |
| Exact Lean replay supplies a numerical bound on Shard's soundness risk | It is powerful differential evidence, not a quantified assurance bound or proof of absence of shared defects. |
| Lean's logic cannot express executable-resource properties | Do not make this claim. Shard's distinction is first-class cost/observation contracts in its compilation workflow. |

Delete unsupported claims about where most historical Lean unsoundness reports occurred unless a defined dataset and analysis are supplied. No such evidence is established by this review.

Also avoid treating all automation as undesirable. Local inference, typeclasses, broad simplification, classical reasoning, and quotient constructions can be useful. Reject hidden semantic choices and unchecked execution gaps, not the existence of these mechanisms.

---

## 10. Suggested replacement for FOUNDATION.md §13

The following compact text can replace the current section, with references resolved to the source list below. Detailed contracts remain in the relevant implementation sections.

### 13. The Lean review — what to borrow, refuse, and improve

The initial closed logical fragment follows the pinned, reconciled Lean rules. Shard owns its implementation, authoring environment, evidence representation, execution model, and deployment workflow. Logical compatibility does not require adopting every operational default. A choice can be useful for Lean and unsuitable for Shard without being a soundness bug.

#### 13.1 Defaults Shard refuses

Unknown identifiers do not silently become new parameters. Inference may reconstruct omitted information; consequential instances, coercions, and implementation choices remain scoped and visible in the resolved requirement. Proof generation operates against that fixed task, including any explicitly declared synthesis holes.

Artifact acceptance includes transitive logical assumptions and the connection to the intended executable bytes or representation. A pending dependency, a changed interpretation of a statement, or a native computation result is not accepted merely because the current proof-producing tool reports success.

Implementation replacement requires the appropriate evidence. Shard does not forbid independently optimized implementations; it rejects an unaccounted gap between the checked meaning and the computation deployed. Relations and their preconditions are explicit, including approximation, effects, and resource behavior where promised.

Speculative meta operations have transactional workspace semantics. Failure and exhaustion do not leave hidden assignments or justify pruning a candidate region. Blocked, invalid, exhausted, open-valid, and closed-accepted remain distinct outcomes.

Imported mathematical totalizations retain their meaning. Application-facing error policies use explicit operations, checked preconditions, or result types rather than silently inheriting a convenient mathematical convention.

#### 13.2 Mechanisms to borrow and strengthen

Borrow useful dependent abstraction, proof terms, local inference, automation, and theorem-directed computation. Current Lean already has stored proof terms, richer module visibility, and proof-producing evaluation; those are references, not absent features to claim as inventions. [X3, X5, X6]

Shard strengthens their integration with one embeddable engine: persistent declarations, contextual workspaces, prepared execution, relation-aware transformations, and replayable artifact evidence. A caller need not use files or a CLI to connect these services. Proof search is flexible; accepted evidence and its dependencies are durable.

For verified evaluation, the general evaluator theorem is only part of the argument. Each invocation additionally supplies checkable evidence of its result, tied to the exact program and environment. The declared Rust execution root remains permissible; no new native-result oracle is introduced silently.

#### 13.3 Boundaries not reopened in v1

K and the toolchain are E programs operating on L-as-data. Applications are intended to compile. The agreed static lambda forms elaborate into first-order E; there is no interpreted-only `fn` fallback. Broader closure representations require a separate workload and cost justification. Explicitly linking the compiled engine into a metaprogramming application remains supported in principle.

Classical proof principles are not rejected merely because some uses have no executable realization. E admission, erasure, and realization enforce the computational boundary. This section does not reopen the chosen totality or process policy.

#### 13.4 Declarative rules and experimental improvements

State the declarative judgments separately from the bounded checking procedure. Acceptance must implement the judgments; failure to establish conversion is not a general inequality theorem. Compatibility is tested under an identified rule/version/input profile and does not supply a completeness proof.

Instrument conversion before extending it. Evaluate evidence-directed conversion and expected-type-directed checking under unchanged rules. Alternative presentations with fewer implicit equalities remain research candidates, with explicit translation and assurance costs, not prerequisites for the first kernel. No v1 foundational departure is authorized by this experiment.

---

## 11. Integrate into the existing phases, not another serial redesign

Retain §14's phases and the agreed R1–R8 amendments. Extend the current battery rather than inventing a second competing test ladder.

| Existing gate | Small addition | Relevant review |
|---|---|---|
| **T0 / phase 0–1** | Specify declarative versus algorithmic rejection; independently classify hostile fixtures; log supported input/version scope | R10 |
| **T1 / phase 2** | Tamper with an executable body while retaining its old equations; require realization failure; retain a progress case | R9, R15 |
| **T8 / evaluation feature** | Tamper with a native result; replay correct evidence cold without the original evaluator tactic | R9 |
| **T9 / phase 3** | Misspelled binder, changed ordering instance, and benign inferred element type; distinguish target change from proof repair | R11 |
| **T8 / first persisted evidence** | Same proposition, different assumption closures; forbidden-evidence policy rejects the unsuitable proof | R12 |
| **T4/T7 / phase 4** | Failure after tentative assignment, exhaustion after partial work, and cross-branch cache isolation | R13 |
| **Primitive crosswalk / phase 3** | Zero division and subtract-underflow conventions; checked/preconditioned wrappers retain distinct semantics | R14 |
| **T6 / phase 4** | Change an implementation and verify correct invalidation of prepared execution versus abstract client proofs | R12, R15 |
| **T7 / first approximate optimizer** | Do not compose two error licenses or root-only equalities as unrestricted exact equivalence | R15 |
| **Existing performance budgets** | Compare conversion guidance and expected-type checking on selected fixed workloads | R16 |

Define the R9 and R10 contracts before ratifying the relevant claims. Their complete implementation proofs need not precede bootstrap bring-up. An uncertified but reviewed executor can still run K and check ordinary evidence; the trust manifest says exactly that.

Do not make a universal elaborator, a general closure runtime, complete Mathlib import, a proof of Rust, or alternative-foundation research prerequisites for these tests. The next useful milestone remains a running implementation of the selected foundation that exercises Shard's own interfaces.

### Response requested from Fable

For R9–R16, record **accept**, **amend**, or **defer**, with the affected section and test. R9 should name the proposed result-evidence mechanism. R10 should state the meaning of each negative outcome available to search. R16 should record an experiment or a deliberate deferral, not silently alter the v1 rules.

**Bottom line:** preserve the agreed foundation and execution architecture. Make semantic intent explicit, let agents search broadly, require concrete evidence at the acceptance boundary, and keep implementation freedom available under stated relations. Borrow mathematical understanding without inheriting operational defaults by accident.

---

## Sources and scope notes

All recommendations above are proposed Shard contracts. The sources below support only the explicitly attributed descriptions of existing systems. They do not prove the proposed kernel, translations, search semantics, or performance advantages. External sources were consulted September 6, 2026; live manuals are not substitutes for the release pin selected by phase 0.

**[F] Reviewed Shard proposal.** `docs/FOUNDATION.md`, DRAFT v0.2, at the commit and blob recorded above. In particular: §5.4 on evaluator correspondence; §§6–7 on elaboration and open terms; §13 on the Lean comparison; §14 on rollout. This is not the older uploaded DRAFT v0.1.

[F]: https://github.com/computer-whisperer/shard/blob/d6b25f10a401d72e8be476e277cbe44a08eed818/docs/FOUNDATION.md

**[X1] Lean reference — Headers and Signatures, Automatic Implicit Parameters.** Documents the default treatment of suitable unbound identifiers and the option to disable it.

[X1]: https://lean-lang.org/doc/reference/latest/Definitions/Headers-and-Signatures/

**[X2] Lean reference — Instance Synthesis.** Documents competing instances, priorities, declaration order, and stuck synthesis.

[X2]: https://lean-lang.org/doc/reference/latest/Type-Classes/Instance-Synthesis/

**[X3] Lean reference — Tactic Reference, `cbv` and `decide_cbv`.** Documents proof-producing evaluation, use of equational lemmas, and well-founded-function support without trusting code generation.

[X3]: https://lean-lang.org/doc/reference/latest/Tactic-Proofs/Tactic-Reference/

**[X4] Mario Carneiro — Lean4Lean: Verifying a Typechecker for Lean, in Lean, arXiv:2403.14064v3.** Used for the declarative/algorithmic distinction, the described inference-forward checker, and the need to distinguish established properties from remaining metatheoretic obligations.

[X4]: https://arxiv.org/html/2403.14064v3

**[X5] Lean reference — Validating a Lean Proof.** Documents transitive axiom checks, stored proofs in `.olean` files, replay, and the separate task of matching the intended theorem statement.

[X5]: https://lean-lang.org/doc/reference/latest/ValidatingProofs/

**[X6] Lean reference — Source Files and Modules.** Documents current public/private information and exposed/unexposed definitions. No specific performance percentage is relied on here.

[X6]: https://lean-lang.org/doc/reference/latest/Source-Files-and-Modules/

**[X7] Lean reference — Natural Numbers.** Documents saturating subtraction and the result assigned to division by zero. These are mathematical conventions, not claims of an implementation bug.

[X7]: https://lean-lang.org/doc/reference/latest/Basic-Types/Natural-Numbers/

**[X8] Lean reference — Recursive Definitions.** Documents partial and unsafe definitions and replacement runtime implementations. Its distinctions should not be collapsed into one category of logical unsoundness.

[X8]: https://lean-lang.org/doc/reference/latest/Definitions/Recursive-Definitions/

**[X9] Lean API — Init.Prelude.** Contains equality theorems marked `@[csimp]`; cited narrowly to distinguish theorem-directed rewrites from an unchecked replacement assertion.

[X9]: https://lean-lang.org/doc/api/Init/Prelude.html

**[X10] Rishikesh Vaishnav — Lean4Less: Eliminating Definitional Equalities from Lean via an Extensional-to-Intensional Translation.** Publisher abstract used only as precedent for translation to a presentation with fewer definitional equalities. No performance figure, full-library coverage claim, or transfer of its correctness result to Shard is assumed.

[X10]: https://link.springer.com/chapter/10.1007/978-3-032-11176-0_13
