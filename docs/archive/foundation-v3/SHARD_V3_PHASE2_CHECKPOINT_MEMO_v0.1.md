# Shard V3 — Phase-2 Checkpoint Memo
## Admission, canonicalization, and execution boundaries

**Version:** 0.1 — feedback for Fable, not a new foundation proposal  
**Baseline:** `computer-whisperer/shard` at **`b45cc22ab2e5d2001141d9e022525450ddeabc4d`**  
**Checkpoint:** Phase 2 close-out, including `v3/LANGUAGE.md` and the new `v3/CANON.md`  
**Continues:** R42–R48 in `SHARD_V3_IMPLEMENTATION_REVIEW_v0.1.md`; this memo adds **R49–R54**  
**Requested response:** Accept / amend / defer for each item, with the changed contract or implementation, regression, and completion gate.

## 0. Scope, evidence, and recommendation

**Continue the implementation plan. Do not reopen the foundation.** The reader, loader, views, evaluator, and explicit realization machinery now exercise the architecture as a language engine, not only as a checker for exported declarations. The calculator differential and toolchain frontend comparisons are useful integration evidence. The checkpoint record also distinguishes the Stage-0 `fn` gap, the unproved compiled execution route, and the still-unsealed K boundary from completed work. [S1]

This memo converts the latest checkpoint review into bounded implementation requests. It preserves the agreed mathematical foundation, the separation of L and E, compiled deployment, the Shard-written toolchain, the maintained Rust execution path, and the I/P distinction. It does not require certifying Rust or the temporary compiled chain before development can proceed.

### Evidence limits

The findings below are based on source inspection at the pinned commit and the preceding review. The Shard test suites, T0, CI pipelines, and proposed new regressions were **not executed independently for this memo**. Repository-reported test results remain repository-reported results.

In particular:

- **R49 is a source-traced authorization defect.** The theorem-kind path is visible in the code. The proposed admission regression and an end-to-end false-theorem exploit have not been executed here.
- **R50 is a contract-level problem in the proposed V3 canonicalization rules.** The V3 recognizer and enforcement are not yet implemented; this is not a report of an executed faulty rewrite.
- **R51 identifies unbudgeted primitive work and conflated outcomes in the current evaluator.** The deliberately huge shift example has not been run.
- **R52–R54 concern a recorded remaining boundary and the demonstrated scope of the comparison and harness code.** They are not claims that the existing positive results are fabricated or useless.

All code-like examples below are schematic unless explicitly labeled as an excerpt from the repository. Sources are pinned permalinks in §10; later `main` changes should be evaluated as fixes against this baseline, not silently substituted into it.

### Priority map

| ID | Request | Priority / completion point |
|---|---|---|
| **R49** | Check the expected declaration kind before accelerator exemptions | **Immediate correctness fix**, followed by hostile and full-export regressions |
| **R50** | Give computation-discarding canonicalization rules explicit execution preconditions | **Before ratifying those rules**; tests before enforcement or search pruning |
| **R51** | Bound primitive work and distinguish resource exhaustion from guard failure | Evaluator hardening before broad embedding or generated-program search |
| **R52** | Seal K at the Phase-3 opener, preserving explicit evidence status | Before the first general `meta/` client depends on K |
| **R53** | State what the frontend comparison preserves and strengthen identity-sensitive coverage | Existing frontend-conformance gate |
| **R54** | Require successful completion independently of matching prefix output | Small harness fix in the existing T0 gate |

## 1. Preserve the progress and the useful existing decisions

The original R42 hash collision is addressed by reference-declaration and dependency-closure comparison. The hostile battery now includes the colliding `Nat.add`, checks that it is not pinned, and distinguishes its real result from the arithmetic shortcut's result. R49 is a different hole in that replacement mechanism, not a request to revert it. [S2, S3]

The raw declaration entry reconstructs node metadata, closing the earlier route in which caller-supplied IDs or flags could reach equality and traversal shortcuts. Keep that distinction between external ingestion and validated internal construction. [S2, S3]

Sequential `let` in both L and E is the right resolved direction. The bootstrap's parallel representation can be normalized for a declared comparison profile without making ordinary V3 source change meaning between `def` and `fn`. [S4, S7]

The explicit `RUNNABLE` versus `REALIZE` distinction, equation checking through K, and candid phase-status records are useful foundations for Phase 3. A completed Stage-0 phase under recorded deferrals is not a claim that the final executable-admission guarantee already exists. [S1]

---

## 2. R49 — Accelerator authorization must validate kind before exemptions

**Priority:** immediate.  
**Primary files:** `v3/kernel/add.shard`, the generated reference machinery, `v3/kernel/test/hostile_test.shard`.  
**Related:** R42; FOUNDATION's fixed-identity primitive contract; T0.

### Source observation

`pin_if_matches` starts a reference-closure walk at any candidate name. `ref_matches` dispatches on the *actual* admitted declaration kind and returns `True` immediately for several kinds, including `ThmInfo`. Only other branches inspect `accel_ref c` and compare the expected declaration. `ref_edges` returns no edges for theorem declarations. The root candidate uses this same path. [S2]

Relevant excerpt:

```text
ref_matches(env, c, info):
    ThmInfo ...  => True
    QuotInfo ... => True
    CtorInfo ... => True
    RecInfo ...  => True
    DefnInfo ... => compare against accel_ref(c)
    ...
```

The intended exemption for proof bodies has therefore become an exemption from checking whether the candidate is the expected kind of declaration at all.

### Source-derived counterexample to the authorization contract

Start with an admitted minimal environment containing the needed Nat and equality declarations, but **not** `Nat.add`. Submit:

```text
theorem Nat.add : (0 : Nat) = 0 := rfl
```

This is a theorem under an unusual free name, not a definition of addition. Tracing the current code:

1. Ordinary theorem admission checks its proposition and evidence.
2. The resulting declaration name is considered as a pin candidate.
3. `ref_matches` returns `True` on `ThmInfo` without consulting the expected `DefnDecl` for addition.
4. The theorem contributes no closure edges.
5. The name receives accelerator authorization.

This establishes a source-level mismatch between the authorization contract and its implementation. It does **not** establish, without further testing, that every later typing/reduction path will permit an invalid theorem to be admitted.

The fix should not be to ban this theorem name as a workaround. Where the namespace policy otherwise permits it, the theorem can be admitted normally; it must remain unaccelerated.

### Requested change

Dispatch jointly on the expected reference role and the actual declaration kind. Validate the root's expected kind, universe-parameter interface, type, body where required, and relevant dependency identities **before** applying any proof-body or generated-declaration exemption.

For the current `Nat.add` candidate, the expected reference is a definition. An actual theorem, constructor, recursor, quotient constant, opaque, or axiom does not satisfy that expectation merely because its branch is exempt elsewhere.

For genuine proof dependencies, distinguishing a proposition from its particular proof remains useful. But permission not to compare a proof body must first be justified for the expected proof role and proposition. Any remaining proof-dependent reduction and assumption obligations must remain covered; do not extend the exemption based on a kind name alone.

Constructors and recursors may continue to be justified through their checked generating inductive declarations. Their use as dependencies does not make them valid substitutes for an unrelated root primitive.

The generated table and matcher should agree on these roles. Continue generating reference data through the normal generator; do not hand-patch generated files.

### Proposed contract text

> Accelerator authorization validates the expected reference declaration before applying any exemption for dependency bodies. An exemption from comparing a proof body is not an exemption from checking the declaration's kind, interface, or expected role. Root candidates must match their prescribed kinds and signatures. Generated declarations are justified through their validated generating declarations. Failure or exhaustion of reference validation enables no shortcut.

### Regressions and closure criteria

**A. Wrong-kind root.** In the minimal environment described above, the theorem named `Nat.add` is admitted as a theorem, but `env_pinned` for that name remains false. Construct the input through the raw declaration API; a full `Init` prefix that already contains `Nat.add` would test name duplication instead of this defect.

**B. Genuine root.** Replay the genuine reference closure and confirm that its addition definition is pinned.

**C. Existing protections.** Retain the deliberately colliding constant-function regression and changed-dependency regression from R42. Passing a new kind check must not weaken either.

**D. Exemption coverage.** Add small matcher-level cases for the other unconditional branches so a later edit cannot accidentally reintroduce the same root-role bypass. Where a declaration kind cannot legitimately occupy a candidate name through normal admission, label that limitation rather than inventing an impossible end-to-end fixture.

**E. Full gate.** Run the existing full replay, axiom-closure comparison, and all-twenty-candidates pin check after the fix. Record separately whether any further executable exploit was reproduced. Do not label a source-level authorization reproducer as a completed false-theorem exploit.

---

## 3. R50 — Canonicalization must preserve required evaluation

**Priority:** clarify before ratifying the affected CANON rules; implement tests before enforcement.  
**Primary documents:** `v3/CANON.md` §§1, 4–5, 8–9; execution and effect contracts in FOUNDATION/LANGUAGE.  
**Related:** C3, C10, and every derived rule that discards, duplicates, or moves evaluation; T7 and the planned canon battery.

### Source observation

C3 rejects an unused binding. C10 rejects identical-branch control and certain vacuous matches. The term tier ranges over E bodies, including supplied realizations. The new document carries the recognizer's no-proof-authority argument from the old law. [S5]

The current E evaluator is strict: right-hand sides, arguments, conditions, and scrutinees are evaluated before their consumers proceed. Stage-0 `fn` bodies do not yet have the final totality guarantee. Effects have an explicit execution path. [S1, S4]

### Why the preconditions matter

Under strict execution:

```text
let unused = loop()
in 0
```

does not have the same bounded execution outcome as `0`. The former exhausts its budget when the recursive computation fails to complete.

Likewise:

```text
let ignored = perform_effect(w)
in 0
```

still performs the effect even though its result is unused. This example is schematic: the concrete fixture should use a permitted, well-threaded host operation and return the entry's required result form. It must not rely on an already-invalid duplicate use of World.

And:

```text
if condition()
then 0
else 0
```

still evaluates `condition()`. Identical branch results do not alone justify removing that computation.

The issue is not that a canon recognizer can prove a false proposition. It is that a claimed semantics-neutral normal form can remove a required behavior, reject its available spelling, or support unsound representative pruning. K will not discover a valid solution that a search engine has already discarded.

### Requested change

Separate **sequential-let flattening**, which preserves the sequence with capture-safe rebinding, from **discarding an evaluated right-hand side**. The first does not justify the second.

For a computation-discarding rule, state the observation relation and conditions under which discarding is authorized. The initial implementation can restrict such rules to admitted pure, total computations under a value-oriented contract. If relevant failure, effect, or resource behavior must also be preserved, establish the corresponding relationship or retain the sequencing.

Do not require identical fuel consumption from equivalent implementations unless the contract explicitly does. Mathematical result equivalence under adequate resources, preservation of a failure family, and equality of a resource trace are different claims. The rule must say which one it supports.

Audit C2, C8, C11, and C12 for the same issue wherever their concrete implementation would skip, repeat, or move evaluation. This is a side-condition audit, not a claim that every one of those rules is defective.

### Proposed contract text

> Canonicalization preserves the observations specified by its profile. A rule that discards or duplicates an evaluation establishes that the change is permitted under that profile; unused values and equal branch results are not sufficient evidence. Stage-0 runnable code carries no implicit totality proof. When a rule's execution preconditions are unavailable, the rule does not apply: the recognizer and rewriter preserve a supported sequencing form rather than treating the expression as redundant. Any search representative-replacement claim carries the same scope and conditions.

A conservative diagnostic such as “rule not applicable under this execution profile” is preferable to silently demanding a rewrite whose semantic premise is absent.

### Regressions and closure criteria

| Fixture | Expected property |
|---|---|
| Unused pure terminating calculation, value-oriented profile | A justified elimination remains available |
| Unused Stage-0 recursive computation | No unconditional replacement with an immediately returning body |
| Unused result of a permitted effect | The effect remains in the trace |
| Equal branches with an effectful or nonterminating condition | Required condition evaluation remains |
| Flattened sequential lets with shadowed display names | Same resolved bindings, evaluation order, and result |
| Resource-sensitive contract | The accepted rewrite proves the selected resource/failure relation or is refused |

The recognizer, rewriter, and search profile must agree on the applicability conditions. Merely proving that the rewritten program has *some* valid type is not the required correspondence test.

---

## 4. R51 — Primitive work needs budgets and distinct resource outcomes

**Priority:** evaluator hardening before broad embedding or generated-program search.  
**Primary files:** `v3/kernel/ev.shard`, primitive/result interfaces and tests; `v3/CANON.md` C1.  
**Related:** FOUNDATION resource-outcome contract; existing primitive and evaluator gates.

### Source observation

The continuation machine spends program fuel at function entry. The primitive branch invokes `prim_apply` without a corresponding primitive-work budget. A returned `None` becomes `HStuck guard`. Some size and exponent-count refusals also take that path. [S4]

`nat_shl` repeatedly shifts by 62 and decreases its count by 62; unlike `nat_shr`, it has no zero shortcut. [S4]

```text
nat_shl(n, k):
    if k < 62 then n << k
    else nat_shl(n << 62, k - 62)
```

Thus `Nat.shiftLeft 0 1000000000000` has result zero but entails approximately 16.1 billion helper iterations under this algorithm. This is an arithmetic inference from the recurrence, not an executed benchmark.

### Requested change

Add the zero shortcut, but treat it as the first regression, not the whole resource fix.

Bound primitive work, output size, and relevant allocation independently of interpreted function-entry fuel. For potentially large host operations, inspect input sizes and operation counts before committing to the work. Where feasible, propagate a consumable work budget or cancellation check through the helper. A size cap checked only after constructing a huge result is not an allocation guard.

Do not change the mathematical operation merely to simplify budgeting. An implementation may return an explicit resource outcome without claiming that the input violates the mathematical domain.

Use a result distinction equivalent to:

```text
PrimitiveValue(value)
PrimitiveGuardFailure(reason)
PrimitiveResourceLimit(resource, site)
```

The precise constructors are an implementation choice. The distinction must survive through `ev`, the driver, diagnostics, caches, and consumers. An oversized valid Nat operation is not the same as a negative value supplied where Nat data is required. A host timeout is not proof that the candidate has no valid result.

C1 must use a bounded normalization interface too. A tiny ground primitive expression may denote an enormous output. Failure to normalize it within limits is not proof that it is already canonical, and must not force an unbounded check. The canonicalization profile should expose that limitation or explicitly exempt the operation under a stated size policy.

### Proposed contract text

> Function-entry fuel does not bound primitive work. Primitive execution enforces explicit work and allocation limits, including preflight guards for expensive host operations. Guard violations and resource exhaustion remain distinct outcomes. Resource failure is never cached or interpreted as mathematical invalidity. Tools that invoke primitive evaluation, including canonical-form checking, inherit its bounded outcome contract.

### Regressions and closure criteria

Use modest injected limits or instrumentation; do not put a billion-iteration test into ordinary CI.

**Zero shift case:** the large-count zero input returns zero through the shortcut within a small agreed work bound.

**Large nonzero result:** a small configured allocation/work limit causes a resource outcome before constructing the oversized result.

**Distinct negatives:** invalid domain data yields a guard/input refusal; valid data exceeding operational limits yields a resource outcome. The output and cache behavior distinguish them.

**Retry:** a computation exhausted under a smaller budget can be retried under a larger one without a cached negative or mutation of accepted state.

**C1:** canonical checking of an expensive ground primitive returns a bounded, intelligible result. It neither silently runs arbitrary work nor marks the operation mathematically invalid.

---

## 5. R52 — Make the sealed K boundary the actual Phase-3 opener

**Priority:** before the first general `meta/` client.  
**Primary surfaces:** K's package view, raw syntax ingress, environment/session ownership, loader restrictions.  
**Related:** R43, R45; existing T0/T5/T6 boundaries.

### Source observation

Raw declaration ingestion now reconstructs metadata. The checkpoint nevertheless explicitly states that the toolchain profile exposes the `CheckedEnv` constructor and defers sealing the whole K package to the Phase-3 opener. Its stated rationale is that the first non-K consumer will be `meta/`; sealing only `kernel/env` would expose other construction paths through its consumers. [S1, S2]

That is a coherent sequencing decision. It must not become another temporary public interface that subsequent metaprograms come to depend on.

### Requested change

The first general `meta/` consumer should use the sealed interface. It should not directly import the raw environment constructor, pin-setting operations, internal admission path, or arbitrary memo/session state while those are promised to be hidden later.

Preserve structural introspection and raw syntax construction: clients still need to inspect and manipulate Shard programs. What they must not manufacture is a claim that their syntax, metadata, or environment has already been validated.

A public checked-construction API can preserve sharing efficiently; it need not rebuild the entire environment on every call. The ownership and context rules must explain which cached fields and identities are trusted, and when foreign or raw nodes are revalidated.

Keep the evidence statuses separate. `RUNNABLE` is not proof of logical admission or totality. A particular realization's checked equations, remaining progress obligations, execution trust, and acceptance conditions must not collapse into one undifferentiated success flag.

### Proposed contract text

> Before a general metaprogramming client is admitted, K is consumed through a sealed boundary. External clients may construct raw syntax and inspect supported semantic structure, but cannot construct a checked environment or authorize accelerators, cached judgments, or internal node identities directly. The ingress operation validates or reconstructs the information on which checking relies. Execution status and established logical or realization guarantees remain separately represented.

### Regressions and closure criteria

An ordinary client outside the trusted implementation boundary cannot import or construct authority-bearing internals, including through transitive imports or a profile shortcut. The existing forged-ID, forged-range/flag, stale-hash, and cross-snapshot fixtures continue to exercise the *public* ingress route.

A legitimate client can construct a small raw declaration, obtain its checked result, inspect the supported information, and perform a subsequent check without reaching around the API.

A runnable self-recursive Stage-0 body remains executable only under the stated operational limits and cannot discharge a totality or realization obligation merely because a different input terminated.

Completion is an actual first `meta/` consumer behind the boundary, not merely a new view file whose consumers still import the implementation.

---

## 6. R53 — Frontend parity must state and test its information-preserving scope

**Priority:** strengthen the existing conformance gate; no new public IR required.  
**Primary files:** `v3/kernel/dump.shard`, `rust_bootstrap/src/dump.rs`, the parity harness, and its scope documentation.  
**Related:** R44/R47; frontend parity, module identity, T5.

### Source observation

The common dump deliberately uses final name components for most declarations and references. Constructor, function, and extern applications share an application-shaped rendering. Measure clauses are excluded, and Nat/Int literals share a textual form. These choices are explicit in the dump implementation and its header. [S7]

This can be useful normalization for a specific comparison. It is not an injective encoding of all module-aware program distinctions.

For example:

```text
ECall(a.f, args)    → (f ...)
ECall(b.f, args)    → (f ...)
```

Therefore byte equality of dumps establishes equality of a projection of the loaded programs. It establishes broader semantic agreement only under conditions making the omitted distinctions harmless.

### Requested change

Keep the fast existing comparison, but give it an explicit coverage profile. Either validate that its name normalization and omitted tags are unambiguous for each tested flat toolchain closure, or extend the compared representation where they are not.

Add an identity-sensitive comparison for module resolution. It can normalize both frontends through a validated identity mapping rather than require identical host-internal names. Preserve declaration/reference identity, head kind, binding structure, and any other distinction claimed by that test.

Excluded measure clauses are acceptable in a runtime-only comparison whose coverage says so. They are not evidence of agreement about recursion obligations. Likewise, merging numeric literal spellings can be legitimate for a shared runtime representation without validating source-level type agreement.

Do not turn this into a requirement to use the comparison dump as canonical P, a content-store format, or the public embedding representation. These formats have different jobs.

### Proposed contract text

> Frontend parity records its compared representation and the distinctions that normalization intentionally omits. Byte agreement is evidence for that declared scope, not a blanket identity or typing theorem. Omitted name and head-kind information is either reconstructed unambiguously under validated profile conditions or retained in an identity-sensitive comparison. Changes to resolution that alter a called declaration are detected even when display suffixes coincide.

### Regressions and closure criteria

Construct a small module fixture containing `a.f` and `b.f` with different behavior. Deliberately change one resolved reference while preserving its final display component; the identity-sensitive comparison must fail.

Construct a case that changes a call's resolved head kind where that distinction is part of the comparator's claim; it must fail or be explicitly outside that comparison profile.

Retain the positive parallel-to-sequential bootstrap normalization tests. Add the chosen uniqueness/profile check to the existing toolchain-closure gate, and state separately what it proves about runtime structure and what remains covered by other tests.

---

## 7. R54 — Positive conformance requires successful completion, not only equal output

**Priority:** small harness correction alongside R49.  
**Primary file:** `v3/t0_full.sh`; reuse the helper in other positive comparisons where appropriate.  
**Related:** T0 fixture/prefix byte-ties; existing full-replay checks.

### Source observation

The fixture and prefix commands each end with `|| true` before the logs are compared. Their exit statuses are discarded. Later stages check pin output and axiom closures, and the full replay separately checks its exit status and expected verdict. Thus this is a weakness in the positive prefix checks, not a claim that the entire full gate consists only of comparing two arbitrary logs. [S8]

Two executions can agree on a failure. A positive conformance gate must establish that both completed the intended successful workload as well as that their outputs agree.

### Requested change

Capture both statuses, preserve their diagnostics, and require success independently before comparing output. Require the expected completion record for the particular fixture or prefix. Keep separate expectations for intentional negative tests rather than imposing zero exit on every test indiscriminately.

A minimal shell pattern is:

```bash
interp_status=0
native_status=0
"$RUST_EVAL" direct v3/kernel/t0.shard -v -a -p "$FIX" \
  > "$LOG.tie_interp" 2>&1 || interp_status=$?
"$K" -v -a -p "$FIX" \
  > "$LOG.tie_native" 2>&1 || native_status=$?

if [ "$interp_status" -ne 0 ] || [ "$native_status" -ne 0 ]; then
  printf 'positive byte-tie failed: interpreter=%s native=%s\n' \
    "$interp_status" "$native_status"
  tail -20 "$LOG.tie_interp"
  tail -20 "$LOG.tie_native"
  exit 1
fi
cmp "$LOG.tie_interp" "$LOG.tie_native"
# Then validate this subset's expected completion/verdict and pin/closure records.
```

This is illustrative code for the fixture portion, not a complete replacement script. Apply the same outcome discipline to the prefix path using its actual argument list and expected records.

### Regressions and closure criteria

Use stub commands in a harness test. Two commands that emit identical logs but exit nonzero must fail the positive gate. One success and one failure must fail. Two truncated logs without the completion record must fail even if byte-identical. Two successful complete matching outputs pass.

Keep the existing full-export verdict, axiom-closure comparison, and twenty-candidate authorization checks. This change supplements them.

---

## 8. Positions on the five open CANON decisions

These are positions on the existing questions, not additional review IDs or new phases. The document's six previously ruled choices remain intact; R50 supplies necessary execution scope rather than reversing sequential let or generalized tag-based `if`. [S5, S6]

| CANON §9 item | Recommended disposition | Condition / test |
|---|---|---|
| **7 — Normal-form universe levels** | Accept as canonical source output if the tooling supplies it | Keep raw K input acceptance and canonical P identity distinct. Unsupported or exhausted normalization is reported, not interpreted as invalid mathematics. |
| **8 — `_` for unused binders** | Accept for genuinely unused bindings | Count dependencies in later types, propositions, and proof terms, not just runtime occurrences. Preserve scope and de Bruijn meaning. |
| **9 — Sorted header block** | Accept only for the demonstrated order-independent fragment | Permute imports and `use` lines in fixtures and compare resolved identities, ambiguity outcomes, accepted declarations, and effective policy. Do not treat order independence as established by a comment alone. |
| **10 — Reserved `canon-rules` form** | Reserve the word now; specify the form with I | Include rule direction, conditions, dependency identity, and the relation permitting representative replacement. No new privileged inference rule. |
| **11 — Declaration order** | Keep deferred until the census | Preserve required dependency and realization order; do not introduce broad source reordering without measured need. |

For the header tests, compare the semantic outcomes the policy claims are order-independent rather than demanding identical diagnostic ordering or identical host memory consumption.

## 9. Disposition and execution order

The requested sequence fits the existing phases:

| Existing activity | Fold in |
|---|---|
| Immediate K maintenance and T0 | R49 kind/role validation and regressions; R54 positive-gate completion checks |
| CANON ratification | R50 execution preconditions; the five decisions in §8 |
| Phase-3 opener | R52 sealed K interface before general `meta/` use |
| Evaluator/conformance maintenance | R51 primitive budgets/outcomes; R53 comparison-profile and identity fixtures |
| Planned canon recognizer and enforcement | Exercise R50's positive and negative cases before using its quotient for pruning or mandatory admission |
| Connected implementation path | Import → realization → caller → I → retained P → prepared invocation, with established and pending guarantees identified at each step |

Retain the large positive corpus and the calculator differential. The purpose of the new negatives is to test the associations that a valid-input corpus does not: expected kind versus actual kind, syntax versus required evaluation, primitive work versus declared limits, and display normalization versus semantic identity.

Please respond by **R49–R54** with accept / amend / defer, implementation or contract location, regression, and completion gate. Record a concrete replacement condition for any amended request. Keep R42's collision fix credited as complete while tracking R49 separately, and keep the remaining R43 sealing obligation visible until its actual client boundary is enforced.

**Bottom line:** Phase 2 demonstrates that the architecture is working. The next work should close these bounded gaps and continue into Phase 3, not reopen the mathematical foundation or add another system-wide redesign.

## 10. Pinned sources

Every repository link below refers to **`b45cc22ab2e5d2001141d9e022525450ddeabc4d`**, not moving `main`. Section and function names are the primary locators; line anchors are convenience pointers within this snapshot. The older architecture drafts are historical context, not the normative specification of this checkpoint.

- **[S1] Progress, evidence, and remaining obligations:** [`v3/README.md`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/README.md), especially the Phase-2 slice records and “Open obligations”; [close-out and deferrals](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/README.md#L285-L397). Numerical test results cited there are repository records, not reruns by this memo's author.
- **[S2] Pin authorization and raw ingestion:** [`v3/kernel/add.shard`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/kernel/add.shard#L225-L377): `pin_if_matches`, `ref_closure_ok`, `ref_edges`, `ref_matches`, `check_decl_pinning_lim`, `decl_pin_candidates`, and `check_with`.
- **[S3] Existing hostile regressions:** [`v3/kernel/test/hostile_test.shard`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/kernel/test/hostile_test.shard), cases 7b–7d and 16. These specify the earlier collision, dependency, and raw-metadata tests; their existence is distinct from independently rerunning them.
- **[S4] Strict execution and primitive limits:** [`v3/kernel/ev.shard`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/kernel/ev.shard#L305-L507): `ev_term`, `ev_ret`, `ev_apply`, `prim_apply`, `nat_prim`, `nat_sized`, `nat_shl`, and `nat_shr`.
- **[S5] Proposed canonicalization rules:** [`v3/CANON.md`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/CANON.md), §§1–5 and §8. Its status distinguishes the written rule set from the later recognizer/enforcement implementation.
- **[S6] Open canonical-form decisions:** [`v3/CANON.md` §9](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/CANON.md#L280-L332).
- **[S7] Frontend comparison projection:** [`v3/kernel/dump.shard`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/kernel/dump.shard#L1-L130): header contract, `dm_term`, `dm_lit`, and `dm_short`. The paired producer is [`rust_bootstrap/src/dump.rs`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/rust_bootstrap/src/dump.rs); the targeted observation in R53 comes from the Shard dump's explicit projection.
- **[S8] Positive T0 comparisons:** [`v3/t0_full.sh`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/t0_full.sh): fixture and prefix `|| true` paths, followed by the separate full-replay status/verdict, axiom-closure, and pin checks.
- **[S9] Governing terminology and phase decisions:** [`docs/FOUNDATION.md`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/docs/FOUNDATION.md) and [`v3/LANGUAGE.md`](https://github.com/computer-whisperer/shard/blob/b45cc22ab2e5d2001141d9e022525450ddeabc4d/v3/LANGUAGE.md), especially Stage-0 status, runtime semantics, views, realization, conformance, and ratification items.

No repository files were changed in preparing this memo. The proposed fixes and test outcomes above remain requests until Fable implements and records them.
