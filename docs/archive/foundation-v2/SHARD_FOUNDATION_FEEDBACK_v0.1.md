# Feedback on the Shard V2 foundation

## Preserve the K/E/R architecture; make the semantic boundaries explicit

**Status:** REVIEW DRAFT v0.1 — feedback for discussion with Claude Fable, not a replacement foundation specification or a ratification.  
**Date:** September 6, 2026.  
**Prepared by:** GPT-6 Pro, for Christian Balcom and Claude Fable.  
**Reviewed source:** `docs/FOUNDATION.md`, DRAFT v0.1, at commit `5d95b8117e6cd03044916c64d8a786c25f5176ff`; file blob `4ca4a9e21c34eb437e7a4d6d65e155387a494063`. [F]  
**Version boundary:** This review addresses the repository revision containing “The toolchain is E; the logic is data” and the four execution routes. It does not treat the older uploaded copy’s resolved-image wording as the current proposal.  
**Additional design input:** Christian’s subsequent clarification permits reconsidering the categorical ban on executable lambdas, including a possible early interpretation path, provided the design does not compromise Shard’s intended compiled deployment. It does not approve unrestricted runtime closures or an interpreter-dependent application model.  
**Scope:** Source review, proposed amendments, and implementation tests. No new checker, erasure pass, lambda lowering, theorem, or performance result has been implemented or validated for this review. Examples are mathematical or pseudocode sketches, not tested Shard syntax.

Source observations below cite [F] by section or the specific repository references [M1–M3]. External technical checks are separately identified by [X1–X7]. Everything labeled “recommendation,” “proposed,” or “test” is a suggested change, not a claim about existing Shard behavior.

---

## 0. Recommendation and decisions requested

Use Fable’s document as the working basis. Keep the dependent logical foundation, its initial reference-exactness objective, the E implementation discipline, the Shard-owned toolchain, and Rust’s role as an enduring bootstrap executor. Do not reopen those boundaries merely to accommodate richer mathematics.

The remaining work is to make the relationships among those components precise and to test Shard’s own compiler, search, and embedding workloads early. A logical checker can be correct while a source-to-execution association is wrong; a native metavariable constructor can exist without a useful partial-program interface; an abstract module can hide information needed to type its exported declarations. Those are distinct engineering obligations.

The proposed amendments are:

| Review ID | Requested amendment | FOUNDATION.md locations | Urgency |
|---|---|---|---|
| **R1** | Give each executable declaration a specified relationship between its L meaning and executable structure; replace “trivial erasure” and literal `K.whnf ≡ ev` with the appropriate obligations | §§2, 4.1–4.4, 5.2, 5.4–5.6, D04 | Resolve the interface before substantial compiler migration |
| **R2** | Treat first-order execution as the initial profile, not a permanent ban on lambda syntax; stage static lowering and make interpreted-only coverage explicit | §§2, 5.1–5.3, Q5 | Agree policy now; implement incrementally |
| **R3** | Specify canonical contextual-hole services, substitution, blocked outcomes, and final closure, while keeping search policy out of K | §7, D11/D12, B15 | Minimal pathfinder alongside early K work |
| **R4** | Specify well-formed module views, implementation matching, and evidence provenance; distinguish opacity from tactic transparency | §4.1, §4.2, F4 | Before sealing the environment API |
| **R5** | Exercise embedding, prepared invocation, and structured evidence as first-class engine operations | D09/D10, B13, §§6–7 | A small early client, not a full SDK project |
| **R6** | Pin the actual logical reference, correct supporting claims, and distinguish an external comparison tool from the source of Shard’s authority | §§1.3–1.4, 3, 4.3, Q1/Q7 | At F0 |
| **R7** | Bring a small compiler/search/embedding slice forward; do not put broad legacy replay or Mathlib replay ahead of every Shard-specific test | §10, Q8 | Before scheduling the rewrite |
| **R8** | Preserve theorem meaning through an explicit crosswalk, rather than promising unchanged declarations solely because equations remain propositions | §12 | Before parallel bulk migration |

**The guiding distinction:** preserve a small logical authority and a deliberately compilable implementation language, but do not make those choices permanently impoverish Shard’s authoring, metaprogramming, or realization interfaces.

---

## 1. Settled architecture: K is an E program; L is its subject language

### 1.1 Accept the clarification without qualification about the host language

The revised §2 is the correct starting point. L terms, universe levels, environments, and declarations are ordinary data. K, the evaluator, the elaborator, and tactics are implemented as E programs manipulating that data. [F, §2]

| Name | Role in this review |
|---|---|
| **L** | Mathematical terms, types, propositions, and proofs checked according to the selected logical foundation |
| **E** | Shard’s executable definitions and their supported computational forms |
| **P** | Proof terms in L; not a second runtime language |
| **S** | The authoring surface and its elaboration conventions |
| **K** | The Shard/E implementation of declaration and proof checking |
| **Rust** | An executor of the E implementation during bootstrap, not an independent implementation of L’s proof rules |

A constructor representing an L lambda in K’s input is not a runtime closure in K. Implementing dependent-function checking does not require the host to support dependent types natively. K’s executable bodies can remain first-order while their own correctness arguments are expressed in L.

Do not infer from this that every possible future implementation of K must be hand-written in today’s smallest E syntax. The initial K can deliberately remain in a stable first-order profile. Later source conveniences can be admitted when their route to that profile is established. This is an evolution of executable coverage, not a requirement to implement L in Rust.

### 1.2 Accept the four routes, with route-specific execution trust

Retain §8’s routes: compiled K; K under compiled `ev`; K directly under Rust; and the full interpreter tower as an optional differential route. Direct Rust-hosted K may issue normal checked results under the stated implementation trust. Formal certification of Rust is not a prerequisite. [F, §8]

Two wording changes would remove ambiguity:

- Replace “Rust never gains semantics” with **“Rust implements E’s operational semantics and does not define a competing source language or L proof system.”**
- Replace “the trust roster does not change” with **“the logical rule authority does not change; the execution dependencies and assurance differ by route and are recorded.”**

The initial frontend/loader still needs a concrete clean-checkout path. State how its first runnable E representation is obtained and how resolved identities reach the executor. This is a bounded bootstrap deliverable, not a demand for a new Rust language frontend or a new public E0 language.

K’s own pending source proofs must not be installed in the subject environment merely because the host executes K. Later checking those proofs establishes their stated properties under the adopted foundation and execution assumptions; it is not unconditional self-justification.

### 1.3 What this review is not proposing

This review does not propose a Lean-hosted acceptance authority, a second Rust proof checker, unrestricted reflection from a checker’s Boolean result into truth, mandatory runtime closures, or a return to nested interpretation as the default development route. It also does not reopen every application/process decision in the draft. The feedback is concentrated on boundaries that the current rewrite is about to establish.

---

## 2. R1 — Specify the relationship between an E program and its admitted L meaning

### 2.1 The unresolved question is about what gets compiled

K being written in E is settled. A separate question remains: **which executable structure realizes the L definition accepted under a particular declaration identity?**

The draft permits subtypes, proof-typed arguments, `decide`, structural recursion, and measure-based recursion through `WellFounded.fix`. It also states that erasure is trivial because proofs never enter E. These statements need a common account. [F, §§5.1–5.5, §9/D04]

In particular, clarify whether E is recognized in a structured executable definition before its justification is elaborated, in the fully elaborated L term, or through a recorded relationship between both. A surface function can be first-order even when its L justification contains lambda-bound motives, recursors, and proof arguments.

**External check:** Lean’s recursive-definition documentation distinguishes a provisional elaborated recursive definition used for compilation from the later construction presented to the kernel. It also states that well-founded recursive computations need not reduce definitionally to their results. This is a precedent for explicitly relating representations, not a proof of Shard’s proposed translation. [X1]

### 2.2 Retain one declaration identity with explicit views

Recommendation: an executable declaration exposes both its logical meaning and the structure useful to execution, search, and lowering. The representations may be identical on a simple fragment; they need not be identical universally.

Schematic internal record:

```text
ExecutableDeclaration {
    identity
    logical_type_and_body
    executable_structure
    logical_and_execution_dependencies
    realization_evidence_or_recorded_bootstrap_status
    execution_profile_and_lowering_status
}
```

This is not a requirement to store duplicate large trees. Sharing, projections, and generated views are implementation choices. It is also not another source language: the author supplies one declaration and the tools derive the relevant views.

The invariant is that a compiler or `meta/` pass cannot silently change the computation associated with an admitted logical constant. The source, resolved identities, views, and evidence must refer to the same snapshot. A shared display name or content hash without a specified correspondence is insufficient.

### 2.3 State the correspondence at the right strength

For a pure function, a schematic successful-result property is:

```text
ValidInput(a) and RunE(exec_d, encode(a)) ⇓ v
    imply ResultRelation(v, logical_d(a)).
```

Where the contract requires a result, also state progress under the relevant resource assumptions:

```text
ValidInput(a) and AdequateResources(exec_d, a)
    imply existence of a permitted completed execution.
```

A one-sided successful-result theorem alone permits an implementation that never returns. Conversely, a resource-bounded evaluator can legitimately exhaust its budget without falsifying the total mathematical function. Make both facts explicit rather than hiding them inside an equality of evaluator names.

Effectful operations need the selected environment/trace relation; they cannot be justified by having K perform live I/O. Declared resource failures need their own permitted behavior. These are instances of the same realization architecture, not new logical primitives.

The evidence can come from a general checked translation, per-declaration certificates, or established compiler lemmas instantiated mechanically. During bootstrap, an uncertified transformation may remain in the recorded execution trust. A final certified application chain must not present an unproved translation link as if it were supplied by K’s type checking.

### 2.4 Replace literal evaluator equality

`K.whnf`, `ev`, and Rust are not three functions with the same result contract. Weak-head normalization may stop at an outer constructor; an evaluator can fully compute a runtime value; the native representation can encode that value differently. Budgets also need not measure identical work. [F, §5.4]

Recommended contract:

> `ev` specifies E’s execution. Each admitted executable definition has a stated relationship to its L meaning. Computational conversion can discharge that relationship on suitable fragments; checked computation equations or realization theorems cover the others. Rust and other execution routes implement the same E semantics on their supported profiles. Conformance compares corresponding results and observations, with resource conditions explicit.

Direct conversion agreement remains a useful regression test. It should not be promoted into a universal theorem that the chosen recursion and representation mechanisms do not support.

### 2.5 Restricted erasure still has an obligation

A small erasure design can be much simpler than compiling all dependent mathematics. It still needs to specify at least:

| Construct | Required account |
|---|---|
| Proof arguments and proof fields | What is removed, and why executable behavior cannot depend on the removed content |
| Subtypes | How the carrier is represented and how introduction obligations remain checked |
| `decide` | Which executable decision procedure is selected; a Boolean result type does not establish that its implementation is in E |
| Dependent casts | Which casts erase, which alter representation, and how the selected case is justified |
| Impossible branches | Which checked evidence permits removing them; a pending claim is not enough |
| Structural and well-founded recursion | How the executable recursion implements the admitted definition and its equations |

Do not describe all proof use as computationally forbidden and then accidentally exclude legitimate transport or recursor scaffolding. Specify what the supported translation does with those cases. Equally, do not infer that an arbitrary proof-valued subterm can be dropped without understanding the surrounding elimination.

**First test:** one structural function, one numeric-measure function, one subtype-producing function, and one concrete `decide` case. Check their L meanings, retain their executable views, and establish the chosen relation before migrating a large compiler closure.

---

## 3. R2 — Relax the lambda prohibition without relaxing compiled deployment

### 3.1 Incorporate Christian’s new direction accurately

The current document says E never has closures and only named, fully applied functions may be supplied to templates. Christian’s subsequent direction removes the presumption that lambda syntax itself is disqualifying. It does not remove the requirement that Shard applications are intended to compile. [F, §§5.1–5.3; additional user direction]

Recommended policy:

> The initial supported E profile is first-order after static elaboration and specialization. Lambda syntax and partial application are not categorically forbidden. They may enter a supported executable profile when their execution semantics, termination discipline, and lowering obligations are specified. Acceptance of a compiled artifact requires an actual supported realization satisfying its declared runtime and resource contract. Successful interpretation alone is not such a realization.

There is no honest promise of zero implementation cost. The relevant question is whether the feature preserves the intended logical and deployment guarantees at an acceptable engineering cost. A new lambda-shaped source construct need not require a new L axiom or K inference rule; it can require substantial work in elaboration, E evaluation, and lowering.

### 3.2 Separate the four questions

| Question | What an affirmative answer establishes |
|---|---|
| Is the L expression valid mathematics? | K accepts its logical typing and evidence |
| Does it have supported executable behavior? | The E execution semantics and implementation cover it |
| Can the selected compiler lower this declaration closure? | A particular realization and its required evidence are available |
| Does the resulting artifact meet the deployment profile? | Its actual code, memory/call machinery, failures, and assumptions satisfy the profile |

Do not answer all four merely by observing a lambda in the source. Do not answer all four merely by successfully evaluating it either.

The word `fn` can remain executable/compilation intent. An experimental function that currently runs only under interpretation must have an explicit “lowering pending/unsupported for this profile” status. It must not silently enter the stable guarantee that ordinary supported programs compile.

### 3.3 Begin with lambdas that disappear into ordinary first-order code

**Closed lambda.** A statically supplied `fun x => x + 1` can be represented by a named helper and specialization of its consumer. No runtime function value is needed in the output.

**Captured immutable data.** Consider this pure example:

```text
add_offset(k, xs) = map (fun x => x + k) xs
```

A possible first-order realization is:

```text
map_add(k, [])      = []
map_add(k, x :: xs) = (x + k) :: map_add(k, xs)
```

Its intended theorem is:

```text
forall k xs, map_add(k, xs) = map (fun x => x + k) xs.
```

The code shape is specialized once; the captured integer `k` remains an ordinary runtime parameter. Do not generate a fresh code specialization for every runtime value of `k`.

**Partial application.** Passing a named function partially applied to known lexical arguments can use the same transformation: lift captured values into helper parameters and specialize the consumer. This is a useful first extension of Q5 without approving arbitrary escaping closures.

These examples are proposed transformations, not an implemented general algorithm. Retain a shared generic theorem where possible and instantiate it with evidence instead of regenerating an unrelated proof for every helper.

### 3.4 Preserve evaluation and capture semantics

A transformation must preserve more than substitution into a pretty-printed body. For example:

```text
let k = expensive_or_effectful_computation()
map (fun x => use(k, x)) xs
```

must not become one evaluation of that computation per element, or delay it past a specified effect/failure. In a pure total setting the result equation may tolerate some reorderings, but resource, trace, or exception contracts may not.

The first source profile can make the problem smaller: capture evaluated immutable values, use statically identifiable consumers, and preserve evaluation points through explicit bindings. Generalizing beyond that is an additional semantic and realization obligation.

If captured resources include borrowed buffers or regions, escape and retention behavior must remain justified. Do not smuggle a general lifetime problem in under the label “syntactic sugar.”

### 3.5 Defunctionalization is compilation, not proof that runtime machinery vanished

For a bounded set of function alternatives, a compiler can represent the choice using a tag and captured data, then dispatch through a first-order function. That gives a possible path to machine code without an evaluator for source syntax. It does not automatically eliminate the tag, environment storage, branch, or allocation.

Whether such residual machinery is allowed is a deployment-profile choice. The first strict profile can require direct statically identified calls after specialization. A later profile could allow finite dispatch and explicitly represented environments when justified and priced. This document does not silently approve generic heap closures or a closure ABI.

General closure conversion is a further option, not the recommended first milestone. Established functional-language compilers reaching machine code, such as CakeML, provide evidence that functional source and compiled deployment are compatible; they do not establish Shard’s desired cost model or no-residual-closure property. [X7]

The acceptance test must inspect the actual runtime obligations. Renaming a closure environment as an ordinary record must not let it evade a profile that prohibits the corresponding allocation or retention behavior.

### 3.6 Static specialization needs its own termination and size discipline

Runtime totality does not imply finite naïve monomorphization. A schematic counterexample is:

```text
f[A](n, x) =
    if n = 0 then 0
    else f[List A](n - 1, [x])
```

For every concrete call, the runtime measure decreases. Specializing the function closure by every encountered type can nevertheless produce:

```text
f[A], f[List A], f[List[List A]], ...
```

The compiler needs a finite-specialization criterion, a representation-sharing fallback, or a clear bounded refusal. A budget-exhausted specialization search is not proof that the program is intrinsically unlowerable.

Likewise, propagating arbitrary captured values or increasingly composed function expressions into specialization keys can create unbounded or impractically large code families. Keys should distinguish static code identity from dynamic environment values. Measure generated code and evidence size as well as compile time.

A terminating template consumer can also invoke its function argument in ways relevant to the caller’s recursion. Higher-order arguments must not become a loophole in the totality argument. Use admitted logical definitions, known terminating consumers, or appropriate explicit obligations; “the callback is a named `fn`” is not by itself a proof about an entire recursive composition.

### 3.7 An early interpreted experiment is acceptable, with an exit condition

An interpreter for a restricted computational lambda fragment can be an ordinary E program using data such as a body and lexical environment plus an apply routine. Its own implementation can remain first-order and compilable. This is intentional interpretation of program data, not a richer logical language inside Rust.

Such an experiment can help validate semantics and improve metaprogramming before all lowering is available. Require:

1. Explicit feature/profile status; no silent interpreter fallback when a compiled artifact was requested.
2. A companion compiled pathfinder for at least the closed-lambda and captured-value cases.
3. A defined boundary for unsupported escapes, foreign callbacks, dynamic code, and noncomputable L objects.
4. Agreement with the intended L/E relationship, including capture, effects, failures, and resource behavior where relevant.

Do not equate this with indiscriminately evaluating any L term that happens to typecheck. L deliberately describes mathematical objects that need not have a computational realization.

### 3.8 Keep the bootstrap implementation independent of optional HOF coverage

The first K and frontend can remain in the initial first-order E profile. Adding source lambdas for ordinary applications need not immediately rewrite K or demand a new native closure value in Rust.

Where an extension is statically eliminated, the established tools can lower it to the supported profile before execution. Where the experiment is an E-written interpreter, Rust executes that interpreter as an E program. A future extension to E’s directly executed value forms is possible but requires an explicit implementation and conformance change; it is not a consequence of L already containing lambdas.

**Suggested sequence:** preserve the present baseline; add closed lambdas and partial applications; add non-escaping value capture; evaluate finite dispatch; consider escaping closure representations only when a real consumer justifies their costs. This is a roadmap, not a demand that all stages gate the foundation rewrite.

---

## 4. R3 — Native holes need an open-construction contract, not just an AST node

### 4.1 Preserve closed K, add canonical open services

Accept that final K admission refuses unresolved metavariables. Search order, ranking, enumerators, and branch heuristics stay outside its closed-proof authority. But §7’s promise of native metavariables does not by itself establish the services needed by `meta/sketch` and search. [F, §7, D11/D12]

The existing sketch interface uses reserved calls for holes and requires compatible binder contexts for shared occurrences; filling is verbatim substitution. These are concrete constraints a native design should improve, not simply reproduce under another constructor. [M1]

Recommendation: specify contextual-hole declaration, occurrence, substitution, assignment, open validation, and final closure as canonical engine services. Their implementation may use K with explicit parameters, a checked open calculus, or a justified combination. Selecting that implementation is open; the user-facing semantic contract should not be deferred.

### 4.2 The minimum useful contract

A hole declaration is schematically:

```text
?h : [Psi |- A]
```

`Psi` is the telescope of local data and hypotheses that a solution may use. `A` is well-formed in that telescope and the declared earlier dependencies. An occurrence `?h[sigma]` supplies a typed substitution from the occurrence context into that telescope.

Example:

```text
?h : [u : Nat |- Nat]
left  = fun x => ?h[u := x]
right = fun y => ?h[u := y]
```

Assigning the single template `h := u + 1` fills both occurrences consistently. Two occurrences are not independent merely because the surrounding binder names differ. A hole declared in an empty context cannot capture a later local variable.

The assignment mechanism must check scope, dependent expected types, universe constraints, and direct/indirect cycles through terms, types, and substitutions. Assignments may introduce fresh subholes only under a well-defined dependency discipline. A reference design for contextual variables does not automatically justify Shard’s exact combined rules. Contextual type-theory research is relevant precedent for explicit dependencies and substitutions, not a ready-made soundness proof for this proposal. [X6]

### 4.3 Do not merge shape validity, satisfiability, and completed proof

The engine should distinguish:

| Result | Meaning |
|---|---|
| Open construction validated | The represented construction has the stated conditional shape under its declared holes/obligations |
| Blocked on obligations | A particular comparison or validation cannot yet be settled |
| Invalid construction | A specified scope/type/structural condition has failed |
| Resource exhausted | The procedure did not finish within its budget |
| Closed evidence accepted | All reachable obligations required for admission have been discharged and the final declaration has been checked |

The names are schematic; the distinctions are required. Open validation is not a proof that a filling exists. A hole of type `False` is an unsolved obligation, not an inhabitant of `False`. Generalizing it into a hypothesis changes the task and must never silently satisfy the original closed requirement.

Likewise, a quoted program containing hole syntax is ordinary data and may occur in a completed theorem. It is not the same thing as an unresolved metavariable being used as that theorem’s missing proof.

### 4.4 Require a closure argument, not a claim that Lean has already supplied it

The desired property is approximately: a validated open derivation, together with a well-typed simultaneous filling satisfying its residual obligations, instantiates to a valid closed judgment under the mapped context. The exact statement depends on the chosen environment, universe, and substitution representation.

Final closure can recheck the fully instantiated declaration; it need not trust cached open results. That is an attractive initial design. It still leaves open-service correctness important for search-space claims: an unsound pruning result can discard all solutions without ever producing an invalid final theorem.

K rejecting metavariables and an external checker accepting the final exported proof do not test that pruning or contextual assignment worked correctly.

### 4.5 Reuse proofs about families without granting search completeness

Establish at least one completed theorem about a template under stated constraints on its fillings. Instantiate it on several candidates. An ordinary quantified L theorem may be the simplest representation; a reified syntax-and-substitution theorem may be better where the property concerns program structure. Do not require new privileged modal rules before evaluating those routes.

Search APIs must separately identify candidate correctness, exact finite enumeration, unsatisfiability, equivalent-representative pruning, and optimality under a specified cost model. None follows merely from native hole syntax. Shared holes and dependent domains invalidate naïve Cartesian counting. Proof irrelevance also does not automatically decide whether the counted objects are evidence terms, programs, or semantic equivalence classes.

The first pathfinder needs only shared contextual holes, one dependent constraint, a blocked comparison, closure, and a reusable template theorem. Full persistent storage and arbitrary branch merging can follow later. Even the first implementation needs snapshot/branch isolation and cache keys that prevent an assignment or theorem from one context being reused as if it belonged to another.

---

## 5. R4 — Module views must remain well-formed under dependency hiding

### 5.1 Body removal is not a complete argument

The module crosswalk describes the consumer environment as a weakening of the implementation environment: same identities, fewer definitional equations, and interface theorems retained as checked constants. That is a plausible direction, but the construction needs conditions. [F, §4.1]

Schematic example:

```text
def Carrier : Type := Nat
def z : Carrier := 0
def acceptsNat : Nat -> Prop := fun _ => True

theorem ok : acceptsNat z := ...
```

After making `Carrier` abstract, the exported proposition `acceptsNat z` is not generally well-typed in that abstract view: `z` has type `Carrier`, while the function expects `Nat`. The concrete environment used an equality that the consumer no longer has.

The issue is not that hiding definitions is impossible. It is that exported types and statements must themselves be valid using only the exported information.

### 5.2 Require three properties

**View validity.** All exported declarations, types, and theorem statements are well-formed in the public view. Private implementation equalities do not leak accidentally into their types.

**Implementation matching.** A concrete implementation satisfies the public signature and its laws. Its realization can be substituted into a consumer under the specified identity/parameter mapping.

**Evidence binding.** A theorem exposed through the interface remains bound to the checked implementation or to explicit parameter assumptions. Hiding its body does not erase its dependencies or turn conditional evidence into an unconditional fact.

A law-bearing L structure and a checked concrete instance may implement these properties cleanly. That does not require allocating a runtime record of function pointers: an E client can instantiate the operations statically. A restricted environment-view construction is another option. Choose and justify one rather than assuming arbitrary deletion of bodies is sufficient.

**First test:** check a consumer with only the public view, then with the implementation linked. Its exported typing and obligations must remain stable. Add a negative case whose proposition secretly depended on a private type equality.

### 5.3 Separate three meanings of “do not unfold”

The draft maps stop fences and explicit-conversion controls to simplifier sets or irreducibility marks. These serve different purposes. [F, §4.2]

| Boundary | Responsibility |
|---|---|
| Interface opacity | Limits what the client can rely on |
| Tactic transparency policy | Controls what the proof-producing tool attempts to unfold |
| Kernel conversion | Determines whether the submitted explicit term is valid |

**External check:** Lean’s documented `decide +kernel` uses kernel reduction and ignores elaborator transparency settings. Thus an irreducibility hint is not automatically an enforced abstraction boundary or a bound on all checking work. [X2]

Keep the intended reference-exact closed conversion rules. Implement robust interface boundaries through admitted declarations/views, and manage expensive proof construction through explicit evidence and tooling policy. Do not silently modify conversion to emulate a tactic fence while continuing to claim unchanged kernel rules.

---

## 6. R5 — The canonical engine and meta/ are early consumers, not a later wrapper

### 6.1 K’s small API is not the entire product API

Accept K’s environment-as-value design and absence of filesystem policy. The public Shard engine must also expose inspection, construction, preparation, invocation, open workspaces, evidence construction, and compilation through coherent library interfaces. Those operations need not all be part of K’s trusted inference code. [F, D09/D10, B13]

There is already a concrete pressure point: `meta/invoke/prepared.shard` retains name indexing, effect analysis, and translated function tables, but sits outside the main interface because exposing its internal EVM table types creates an import cycle. [M2]

An opaque prepared-context interface is a good fix. A small in-process consumer should test that fix early; designing the handle without exercising its ownership, environment binding, and repeated-call behavior is insufficient.

Schematic operations:

```text
inspect(environment, declaration)
prepare(environment, entry, execution_profile)
invoke(prepared_handle, arguments)
new_workspace(checked_environment)
validate_open(workspace, candidate, expected_type)
close_and_admit(workspace_snapshot, declaration)
compile(environment, entry, target_profile)
```

Do not require this exact API. Require that the workflow does not print source, spawn a CLI, rebuild the whole module, or reparse proof strings between libraries that already share a process.

### 6.2 One identity system; different views and different rights

Prepared execution binds an immutable declaration/environment view and a realization. A later workspace edit cannot silently change what an existing handle invokes. Runtime data handles must not be mistaken for logical terms, and draft declarations must not acquire checked status merely because they share a name with committed declarations.

The theorem-query API should retain statement, local parameters, provenance, and assumptions. Today `theorem_scope.shard` joins scope-bound names to the checked theory and consults source declarations to distinguish granted interface requirements from other axioms. The replacement can represent that distinction directly rather than making every consumer reconstruct it. [M3]

“Native” should mean the canonical representation and service are available to all relevant clients. It need not mean everything is a privileged constructor or that a client must know arena or evaluator-table internals.

### 6.3 Preserve relation-aware optimization

The existing theorem-capture code distinguishes ordinary candidate equalities from observer equalities whose use is limited to a particular root observation. Preserve that discipline. [M3]

From `O(p) = O(q)` one cannot infer unrestricted replacement of `p` by `q` under arbitrary contexts. A transformation API should carry its relation, context permissions, preconditions, and evidence. Exact, approximate, trace, and resource properties must not be collapsed into one Boolean “equivalent” flag.

This matters both to executable lambda transformations and to ML optimizers. Equality in L’s extensional function model does not assert identical program syntax, allocation behavior, or observation traces. Preserve intensional program representations and state what a transformation actually guarantees.

### 6.4 Explicit runtime use of the engine is compatible with compiled applications

A compiled ML runtime or compiler service may explicitly link compiled E implementations of K, `ev`, and selected `meta/` modules. Its generated compute kernels need not retain those components. The application is compiled even though part of its intended behavior is to manipulate or evaluate code represented as data.

Do not silently add an interpreter to an ordinary application because a compiler pass failed. Do not forbid an application from deliberately implementing an interpreter as its business logic either. These are different product claims.

Large ML input buffers should use an explicit resource/representation interface, not compulsory expansion into literal term trees. Lifetimes, mutation, and decoding are boundary obligations; tensors need not become K primitives.

**First test:** an in-process client constructs a small compute declaration, prepares and invokes it repeatedly, transforms it with `meta/`, checks a related claim, and requests a compiled realization. Track preparation count, marshaling, and environment invalidation. A fully featured foreign-language binding or production ML benchmark is not required for this test.

---

## 7. R6 — Keep the external reference, correct what it establishes

### 7.1 Exactness is a valuable initial constraint, not an implementation oracle without a contract

Accept zero intentional departures in the first supported closed L fragment. Pin the reference release, export format, universe and declaration rules, conversion behavior, and the primitive reductions actually supported. Unsupported declarations and exhausted checking must remain distinguishable from logical rejection. [F, §§1.3, 3]

A valid mutation of a valid declaration is not necessarily invalid. Negative fixtures must have an independently specified reason for rejection; “it was mutated” is not such a reason. When Shard and another checker disagree, investigate the rule, input mapping, and resource conditions rather than selecting a winner solely by tool name.

Native contextual workspace services, E execution, and compiler lowering are not automatically validated by successful closed Lean proof replay. They need their own tests and stated correspondence.

### 7.2 Factual corrections to the rationale

The following are corrections to supporting claims, not objections to the dependent foundation:

| Draft claim | Recommended correction | Basis |
|---|---|---|
| The 2019 thesis alone supplies the exact rule account for the proposed modern Lean 4 feature set | Reconcile the thesis, later accounts, and the pinned kernel, particularly nested inductives and structure eta | Lean4Lean explicitly identifies these additions as changes to the theory; it also discusses remaining metatheoretic obligations [X3] |
| `lean4checker` is an independent small kernel whose size helps estimate K | It replays declarations through Lean’s own kernel. Use it for replay and comparison, not as an independent kernel-size example | Its README says this explicitly and notes its integration as `leanchecker` [X4] |
| HOL is fundamentally unable to express the underlying mathematics of dependent examples | Native dependent syntax and convenient transport are genuine advantages; encodings using predicates and packages are a different convenience tradeoff | The absence of `Vector A n` as a native type does not prevent specifying a list of length n. This is a logical representation distinction, not a benchmark claim |
| Export makes mathematical-library checking free | Export supplies declarations and dependencies; importing, validating, replaying, and connecting definitions to E still cost work | `lean4export` describes declaration and dependency export, not a zero-cost integration theorem [X5] |
| The trust floor is three things because there are three standard axioms | Keep logical assumptions distinct from correctness-critical code and execution dependencies | K still implements binding, universes, conversion, inductive admission, identity, decoding, and numeric acceleration [F, §§3–4] |

Lean4Lean’s bounded-checking discussion supports explicit exhaustion rather than assuming global normalization. Its account is valuable precisely because it distinguishes theory, checker implementation, and what has been verified. Do not imply that reproducing a compatible rule inventory transfers a completed proof to Shard. [X3]

### 7.3 Primitive acceleration and standard assumptions

For each accelerated Nat operation, record its logical declaration identity, definition/equations, accepted argument shape, literal behavior, and correspondence test or proof. Do not assume the entire proposed acceleration list is a native reduction inventory of every Lean release. Reconcile it with the chosen pin.

The standard mathematical profile may use the stated principles. Per-artifact environment assumptions and abstract interface hypotheses remain separate. Compare exact dependency statements, not only axiom names. Changes to proof evidence can change its assumption closure even when the theorem statement is unchanged.

A claim that an artifact is certified is allowed on the reviewed Rust execution route. A claim that the Rust executor itself is verified requires separate evidence. Neither should be silently substituted for the other.

### 7.4 Size and assurance budgets

Keep the 6–10k-line figure as an unmeasured estimate. Do not make matching it a gate that encourages hiding admission logic in supposedly untrusted helpers. Measure the complete acceptance path and the actual compiler/search workloads.

The useful gate is a coherent specified kernel that accepts valid evidence, rejects invalid evidence, exposes assumptions and exhaustion correctly, and has tolerable cost. The route used to run it is reported. A short source file and a large accepted corpus are evidence of different things; neither proves soundness.

---

## 8. R7 — Test Shard’s boundaries before broad corpus migration

### 8.1 Keep the existing ladder vocabulary; add small early slices

F1 already requires direct execution of K, while F2 establishes that route. Treat direct execution as part of F1’s bring-up rather than an implicit prerequisite accomplished later. K’s source may initially run with pending own-source claims under the explicitly reviewed host; those claims are then checked without becoming assumptions by fiat. [F, §§8–10]

Suggested adjustment:

| Existing rung | Proposed treatment |
|---|---|
| **F0** | Pin closed-rule/reference scope and answer the core interface questions. Include a concrete cold bootstrap route and the minimum L/E/open-service contracts. Do not require all their future proofs to be complete before implementation. |
| **F1 + F2** | Bring up K directly as E on Rust; pass a small explicit dependent core and hostile fixtures. Expand exported `Init` coverage as admission features land. Measure direct execution and loading here. |
| **F3** | Establish the first L/E correspondence cases and one retained invocation. Keep conversion checks as applicable tests, not a universal equality contract. |
| **F4, minimal slice** | Implement only enough elaboration and tactics for the first library and compiler examples; test module-view stability and contextual holes alongside this work. |
| **F8a, pulled forward** | Port a small real compiler argument and its dependencies, preferably the planned `tb_len` case or a clearly identified preparatory subcase. Verify the executable/mathematical relationship, not just a shorter proof. |
| **F5** | Grow certified arithmetic as demanded by the examples. Particular arithmetic work may precede F8a; all arithmetic migration need not. |
| **F7** | Keep small relevant exports early. Broader Mathlib replay remains an important scaling target but does not gate every Shard-specific test. |
| **F6** | Treat the old proof-DSL compatibility layer as an optional migration accelerator. Compare its maintenance cost with direct agent rewriting before making it a mandatory phase. |
| **F9** | Start small authoring trials as soon as the interface is usable; repeat the full gate later. Do not wait until all library work has encoded workaround conventions. |

This is not a proposal to unpark every old arc. It is a small set of experiments on the boundaries the rewrite is choosing.

### 8.2 Minimal combined acceptance battery

| Test ID | Experiment | What failure would reveal |
|---|---|---|
| **T1 — realization** | Structural recursion, numeric-measure recursion, subtype output, and an executable decision procedure each connect E execution to the admitted L definition | Missing or incorrect erasure/recursion/representation bridge |
| **T2 — static lambda** | Compile a closed lambda, a named partial application, and the captured-value `map_add` example to the declared first-order profile | Source restriction mistaken for artifact restriction; incorrect capture or specialization |
| **T3 — bounded specialization** | The type-growing recursion example and a large finite specialization workload fail or fall back explicitly within agreed limits | Runtime totality mistaken for compiler termination or acceptable code size |
| **T4 — contextual holes** | Shared hole under renamed binders; dependent expected type; blocked comparison; final closure; one theorem reused across fillings | A native node without a coherent open-construction discipline |
| **T5 — abstraction** | Consumer checks with public view and with implementation linked; a private-equality leak is rejected | Invalid environment weakening or unstable typing |
| **T6 — embedding** | Construct, prepare once, repeatedly invoke, transform, prove, and compile in process under one declaration identity | CLI dependence, duplicate environments, or preparation overhead hidden by the API |
| **T7 — search fidelity** | Small ground-truth candidate space with correlated holes and a root-only rewrite invalid in a nested position | Incorrect pruning, counting, or reuse of observational equalities |
| **T8 — replay and trust** | Cold replay of terms, assumptions, declaration identity, and artifact bytes on reviewed Rust-hosted K; exhaustion never yields a receipt | Pending evidence, stale caches, or an unstated execution trust transition |

Before interpreting results, agree what is counted: source lines, unique proof nodes, warm checking work, cold loading, peak memory, emitted code, or developer interventions. Do not use only one number to represent all costs.

For T7, distinguish certifying a found candidate from certifying an exhaustive search or optimum. A heuristic timeout is not UNSAT, and a representative-replacement theorem is not proof that the removed region was empty.

---

## 9. R8 — Migrate meanings deliberately and proof syntax freely

The equation-shaped statements have a straightforward target syntax in L. That does not establish that changed integer definitions, recursion elaboration, subtypes, module views, and executable bindings retain the same meaning automatically. Replace “statements verbatim; definitions untouched” with an explicit semantic-preservation goal. [F, §12]

A small per-interface migration record should identify the old declaration, the new declaration, changed definitions, assumption changes, and the evidence or review establishing their relationship. Intentional strengthening or weakening is allowed, but it is a separate decision from rewriting the proof.

Parallel agents can rewrite proof terms aggressively once those interfaces are fixed. Do not let the agent responsible for solving a proof quietly weaken its requirement, expose a private definition, add an axiom, change the observation model, or alter a primitive to make it pass.

Keep valuable negative fixtures and performance fixtures. The precise old proof text is not an asset to preserve at the expense of the new design. Equally, generating a valid new theorem is not evidence that the old product obligation survived unless their meanings have been connected.

The same principle applies to lambda support: an interpreted experiment is useful evidence about a proposed semantics, not a migration of the compiled application guarantee by itself.

---

## 10. Answers to Q1–Q8 under the clarified architecture

| Question | Proposed answer |
|---|---|
| **Q1 — Exactness** | No intentional departures in the first supported closed L fragment. Pin and reconcile the actual rules and numeric reductions. Resource outcomes and Shard’s open-engine services are specified separately; do not reproduce an upstream bug as a requirement. |
| **Q2 — Sidecars** | Source tactics plus checked proof-term DAGs for replay. Keep the statement, evidence, and assumption identities distinct. Search traces are useful development data, not an acceptance prerequisite. |
| **Q3 — Int and words** | Follow the selected mathematical construction, with a semantic crosswalk for existing operations. Validate division, remainder, shifts, widths, and boundary behavior. Runtime big integers and words may have efficient representations; shared names do not prove their correspondence. |
| **Q4 — Indexed E-types** | Full compilation support can be deferred. Test at least one indexed L interface realized through an ordinary E representation early. This protects future use without forcing an intrinsically indexed execution IR today. |
| **Q5 — Templates and lambdas** | Replace the permanent named-function-only prohibition with staged supported profiles. Start with closed lambdas, statically resolvable partial application, and non-escaping captured values when their lowering is available. An earlier interpreted experiment is permitted with explicit pending-lowering status. No default approval of escaping runtime closures. |
| **Q6 — Second syntax** | No Lean concrete-syntax frontend in v1. An explicit-term importer and Shard’s S-expression/structured interfaces are enough. Evaluate syntax conveniences through authoring tasks rather than unsupported claims about agent adaptation. |
| **Q7 — Mathlib** | Use small exports as a valuable early independent input source. Price larger replay and definition bridging; export is not free integration. Broad library availability must not displace the first Shard compiler/search/embedding tests. |
| **Q8 — Coverage ordering** | Pull a bounded F8a forward after the necessary F3/F4/F5 slices, not after all legacy compatibility and Mathlib replay. Keep the full coverage arc parked until the new interfaces actually pass those tests. |

### 10.1 Reconciliation with the earlier D/B decisions

**D01/D02:** accept the reference-exact starting policy while keeping the implementation Shard-owned. No new competing foundation is proposed here.

**D03:** preserve the fixed conversion relation and explicit resource outcomes. Tactic transparency is not a substitute for the relation’s specification or for module opacity.

**D04:** accept E and withdraw any requirement for a separately branded E0. Retain a specified, evidenced relationship between executable structure and L meaning.

**D05/D06/D08:** preserve intensional program identity, shared proof structures, and meaning-preserving migration. Clarify evidence and assumption identities rather than requiring every representation to have one identical hash.

**D07:** use external replay as a strong validation instrument and useful library route, without describing its cost or logical transfer as automatic.

**D09–D12/B13/B15:** accept a small closed K and keep search policy outside it. Bring the minimum engine/open-context/prepared-invocation contract forward. Defer sophisticated branch merging, not the basic distinction between branch state and checked evidence.

**D13/B16:** support explicit linking of E implementations of the engine and `meta/` as application functionality. Do not impose that machinery on generated artifacts or use it as an implicit fallback from failed compilation.

**B06–B12/B14:** accept the clarified K/E/R role and normal checking on an uncertified reviewed executor. Do not revive a broad Rust proof/frontend implementation as this review’s recommendation. E execution conformance and a concrete bootstrap path still need to be stated; Rust’s lack of L rules does not mean it has no E behavior to get right.

---

## 11. Suggested edits for Fable’s next revision

The following are proposed replacement passages, not already accepted policy. They are intentionally small enough to integrate into the existing document.

### 11.1 Add after “The toolchain is E; the logic is data” in §2

> This statement concerns the executable implementation of the toolchain, not the permitted mathematical content of its inputs or attached proofs. The initial toolchain remains in a first-order E profile. Source-level lambda conveniences may later lower into that profile without adding runtime closures to K or L rules to Rust. One declaration identity can expose both its admitted L meaning and its executable structure; their correspondence is an explicit part of admission/realization, not implied by sharing a name.

### 11.2 Replace “E does not have closures, ever” and refine §5.3

> Shard applications are intended to compile. The initial supported E lowering profile is first-order and does not require residual function values or generic closure machinery. Lambda syntax and partial application are not categorically excluded: admit supported forms through specified static transformations, with evidence and finite-specialization discipline. Captured runtime values remain data, not an unbounded source of code specializations. More general closure representations require a separately justified profile; successful interpretation never silently substitutes for requested compilation.

### 11.3 Replace the universal agreement claim in §5.4

> `ev` defines E execution. Each admitted executable declaration has a stated relationship between that execution and its L meaning. Definitional conversion is one way to establish the relationship, not the only one. Computation equations and realization theorems account for well-founded recursion, erasure, and representation changes where required. Execution-route conformance compares related results and observations under explicit resources and environment assumptions; weak-head normalization is not identified with full runtime evaluation.

### 11.4 Replace the “trivial erasure” entry in §9/D04

> E is a deliberately restricted executable discipline. Its erasure and executable-view construction are simpler than general dependent-language extraction but remain specified transformations with correspondence obligations. The first implementation may use explicitly recorded bootstrap trust; a certified artifact claim must not silently omit the relevant realization link.

### 11.5 Expand §7 without adding search policy to K

> Native metavariables have declared local telescopes, dependent expected types, explicit occurrence substitutions, and checked assignments. The engine exposes open validation and blocked obligations without claiming a solution exists. Final K admission rejects reachable unresolved obligations. A completed theorem may quantify over fillings or reason about quoted open syntax. The substitution/closure account is specified for Shard’s actual representation; Lean-style metavariables are a reference, not proof that this obligation has already been discharged. Minimal contextual-hole and family-proof tests land alongside the early checker.

### 11.6 Amend the environment-view row in §4.1

> A public view must be well-formed using only its exposed declarations and equalities. A concrete implementation must satisfy that view, and exported evidence retains its checked dependencies and assumptions. These properties justify client reuse; arbitrary removal of bodies from an implementation environment is not by itself a proof of valid weakening. Tactic transparency controls are separate from enforced interface opacity.

### 11.7 Adjust §8’s terminology, preserving its architecture

> The four execution routes share the same intended checking logic; their execution dependencies are recorded separately. Rust implements E as the enduring bootstrap executor and does not independently define L, source resolution, or proof policy. The first runnable frontend/loader has a concrete source-to-execution route. Reviewed Rust-hosted K can issue acceptance-grade judgments before Rust is formally certified.

### 11.8 Amend §§10 and 12

> Introduce a small L/E realization, contextual-hole, retained-invocation, and compiler-proof slice before broad compatibility-layer or Mathlib migration becomes the critical path. Preserve existing requirement meanings through an explicit declaration/primitive/assumption crosswalk; rewrite proof syntax freely. Do not measure successful migration solely by acceptance of a nearby new theorem.

---

## 12. What should be settled next

The next exchange should decide the following, rather than assigning bulk proof rewrites immediately:

**Executable view:** Where is executable structure recorded relative to L elaboration, and what proves or explicitly accounts for that connection in the first version?

**Lambda profile:** Is the first extension closed lambdas and named partial application, followed by the captured-value example? Which artifact restrictions are truly required, and which were proxies for avoiding interpreter dependence?

**Open engine:** What is the smallest contextual-hole interface that passes T4 and permits a theorem about families of candidates while keeping closed K unchanged?

**Module view:** Is the first implementation based on explicit parameters/law-bearing structures, a restricted environment-view mechanism, or a carefully specified combination?

**Early gate:** Which concrete compiler proof and one in-process consumer will test these choices before the environment and executable representation become entrenched?

No proposal in this review requires abandoning the reference-exact starting logic, the E-written kernel, or compiled deployment. The requested change is to stop treating their connecting obligations as automatic and to stop treating the initial convenient source subset as a permanent mathematical or compiler limitation.

**Bottom line:** keep Fable’s concrete foundation and the clarified implementation split. Make L/E correspondence, contextual construction, and embedding real from the first useful slices. Admit lambda syntax where a justified implementation permits it, but preserve the hard distinction between “this expression can be interpreted” and “this application has the compiled realization it promises.”

---

## References and evidence boundaries

Repository sources are the requested basis of the review. Section references to FOUNDATION.md refer to the pinned repository revision below, not the earlier uploaded copy. External primary sources were consulted on September 6, 2026 to check the specifically attributed technical points. They do not validate an unimplemented Shard architecture, prove the proposed transformations correct, or guarantee the projected development cost.

### Reviewed project sources

**[F]** Fable, `docs/FOUNDATION.md`, DRAFT v0.1, revision containing the E-implementation law and four execution routes. Commit `5d95b8117e6cd03044916c64d8a786c25f5176ff`.

<https://github.com/computer-whisperer/shard/blob/5d95b8117e6cd03044916c64d8a786c25f5176ff/docs/FOUNDATION.md>

**[M1]** `meta/sketch/mod.req.shard`, especially the hole encoding, context requirement, shared-choice semantics, and grammar/counting contract.

<https://github.com/computer-whisperer/shard/blob/5d95b8117e6cd03044916c64d8a786c25f5176ff/meta/sketch/mod.req.shard>

**[M2]** `meta/invoke/prepared.shard`, retained preparation and the documented public-interface import-cycle issue.

<https://github.com/computer-whisperer/shard/blob/5d95b8117e6cd03044916c64d8a786c25f5176ff/meta/invoke/prepared.shard>

**[M3]** `tools/search/theorem_scope.shard`, scope-bound theorem capture, granted-requirement provenance, and distinct candidate/observer rewrite permissions.

<https://github.com/computer-whisperer/shard/blob/5d95b8117e6cd03044916c64d8a786c25f5176ff/tools/search/theorem_scope.shard>

### External technical checks

**[X1]** Lean Language Reference, “Recursive Definitions.” Used for the two-stage treatment of recursive definitions and the distinction between well-founded computation equations and definitional reduction. Live documentation is not a substitute for the release pin that F0 must select.

<https://lean-lang.org/doc/reference/latest/Definitions/Recursive-Definitions/>

**[X2]** Lean Language Reference, “Tactic Reference,” `decide +kernel`. Used for the distinction between kernel reduction and elaborator transparency settings.

<https://lean-lang.org/doc/reference/latest/Tactic-Proofs/Tactic-Reference/>

**[X3]** Mario Carneiro, *Lean4Lean: Verifying a Typechecker for Lean, in Lean*, arXiv:2403.14064, version 3. Used for the changed treatment of nested inductives/structure eta, the relationship to the 2019 thesis, and bounded checking. The paper distinguishes implemented checking from unfinished metatheoretic work; this review does not claim it supplies a complete proof for Shard.

<https://arxiv.org/html/2403.14064v3>

**[X4]** `leanprover/lean4checker`, README. States that the tool replays through Lean’s own kernel rather than supplying an independent kernel, and describes its successor `leanchecker`.

<https://github.com/leanprover/lean4checker>

**[X5]** `leanprover/lean4export`, README. Plain-text export of declarations and dependencies; evidence for an interoperability route, not for zero-cost replay or definition equivalence.

<https://github.com/leanprover/lean4export>

**[X6]** Mathieu Boespflug and Brigitte Pientka, *Multi-level Contextual Type Theory*, arXiv:1111.0087, version 1. Background for contextual variables and substitutions. No claim is made that Shard should adopt the entire theory or that its results automatically apply to Shard’s proposed combination.

<https://arxiv.org/html/1111.0087v1>

**[X7]** CakeML project overview. Evidence that a functional source language can have a machine-code compilation path with a formal correctness argument. Not evidence that Shard’s proposed lambda lowering is implemented, cost-free, or certified.

<https://cakeml.org/>
