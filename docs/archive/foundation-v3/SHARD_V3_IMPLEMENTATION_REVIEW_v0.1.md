# Shard V3: implementation review after the first Init replay

## Harden admission; keep Phase 2 moving

**Status:** REVIEW MEMO v0.1 — requested fixes and decisions, not amended law or completed validation.  
**Date:** September 12, 2026.  
**To:** Christian Balcom and Claude Fable.  
**From:** GPT-6 Astra Pro.  
**Reviewed baseline:** `computer-whisperer/shard` at `d6f1fd66aa13f40250e005aed1d1422eb8b29ffa`, the revision examined in the preceding progress review. Its latest commit in that review was dated September 8, 2026. This memo is pinned to that revision; it does not assert that subsequent revisions retain these issues.  
**Relationship:** Continues the foundation reviews R1–R41 with implementation items **R42–R48**. The ratified K/L/E architecture is not reopened.  
**Evidence boundary:** Source observations below come from the pinned repository. Replay counts and performance figures are the repository's recorded results, not benchmarks independently rerun here. The accelerator-collision arithmetic was independently recomputed in Python, including while preparing this memo. No end-to-end false-theorem acceptance was executed, and this is not a complete soundness audit. Proposed interfaces, counterexamples, and regression tests are labeled as such. The earlier uploaded foundation drafts are historical context, not the implementation baseline.

---

## 0. Recommendation and scope

Proceed with Phase 2, but fix **hash-only accelerator authorization immediately** and seal **annotated terms as well as checked environments** before exposing the general embedding surface. Resolve the proposed L/E `let` difference before the new reader and corpus depend on it.

Keep the pragmatic execution strategy. An explicitly trusted, uncertified Rust executor can run the Shard checker, and an explicitly identified unverified compilation route can support development and differential testing. Neither formally verifying Rust nor replacing the foundation is a prerequisite for these corrections.

The reported milestone is substantial: the pinned `Init` export has 57,977 top-level declarations accepted with no reported rejection, exhaustion, mismatch, or unsupported result. The record also reports matching axiom closures for 59,433 admitted constants and a whole-export interpreter/compiled log comparison. These are distinct counting units and strong coverage evidence, not a soundness proof. [S1]

The task now is to turn that coverage into a robust reusable engine, without making temporary bring-up semantics permanent public language behavior.

### Decisions and work requested

| ID | Item | Priority / timing | Existing work it belongs to |
|---|---|---|---|
| **R42** | Replace hash-only accelerator authorization with collision-resolving validation | **Immediate; correctness-critical** | K admission, generated accelerator references, hostile battery |
| **R43** | Seal or reconstruct expression metadata, node identities, checked environments, and checking sessions | Before general API/loader exposure | Phase-2 loader/module surface; T0 and later T4/T6 |
| **R44** | Give the ordinary V3 `let` form one meaning in both L and E | Before Stage-0 language ratification / reader commitment | `v3/LANGUAGE.md` §5.4 and §13 item 5; frontend conformance |
| **R45** | Make runnable Stage-0 programs distinct from admitted definitions and checked realizations | With the first Stage-0 execution/acceptance interface | Language §0, classifier, views, `realize`, T1/T5/T8 |
| **R46** | Preserve execution-route trust labels and measure memory/outlier costs separately | Continue during implementation; no new certification prerequisite | Existing test/build/full-replay scripts and performance records |
| **R47** | Keep prefix imports and compatibility omissions bounded, explicit bring-up mechanisms | Loader design and compatibility-ledger ratification | Language §§3, 12–13; T5/T6 |
| **R48** | Separate ratified rules, demonstrated coverage, and remaining implementation obligations | Documentation cleanup now | Foundation banner, V3 status records, issue/gate tracking |

**Scope control:** R42 is a concrete source-level correctness issue. R43 is a boundary gap already acknowledged by the project. R44 is a proposed language choice to change. R45–R48 are implementation and lifecycle disciplines, not four new architectural subsystems.

---

## 1. R42 — A structural hash must not authorize a logical accelerator

### Source observation

`expr.shard` uses a structural hash modulo `M = 2^61 - 1`. Its normal structural equality procedure uses hash agreement as a filter and then compares structure, except for the positive-node-ID shortcut governed by R43. In contrast, `add.shard::pin_if_matches` enables acceleration after comparing the candidate's `identity_hash` with the integer returned by `accel_pin`. No collision-resolving structural check follows that match in the inspected function. `accel_pins.shard` contains the resulting expected integer hashes. [S2], [S3], [S4]

The relevant decision is effectively:

```text
hash(candidate declaration) == committed pin
    => authorize accelerator for candidate name
```

For an accelerator, this is not merely an indexing decision. It permits a particular shortcut to participate in logical computation. A different admitted definition under the same name must not acquire that shortcut.

### Independently checked arithmetic counterexample

The source defines:

```text
M = 2305843009213693951
mix(a, b) = (1000003 * a + b + 1) mod M
hash(LitNat n) = mix(61, n)
```

Consequently, literals `n` and `n + M` collide. More directly, applying the source's constructor hashes to the signature `Nat -> Nat -> Nat` and this constant-function body:

```text
Nat.add := fun (a : Nat) (b : Nat) => 1781693077243641776
```

produces:

```text
signature hash       = 1170759705707887636
body hash            = 1506445342912286541
identity hash        =  606620205132656612
committed Nat.add pin =  606620205132656612
```

The values match exactly. Appendix A contains the calculation. It does not forge cached metadata: it computes the hashes that ordinary constructors assign to this different body. [S2], [S3], [S4]

**What has and has not been established:** the hash collision and the hash-only decision in the inspected source are established. Admission of the candidate followed by a false-theorem verdict has not been executed here. The end-to-end kernel regression is required, not claimed complete.

The inferred risk is direct: if the candidate is admitted and pinned as this code indicates, unfolding its body and invoking the authorized addition shortcut describe different computations. This is a logical-identity problem, not a reason to expand the project's supply-chain threat model.

### Requested fix

Keep the fast structural hash for buckets, memoization, and early mismatch detection. After a potential pin match, validate the declaration against the expected reference using an exact, explicitly specified comparison of the relevant declaration structure and dependency identities.

That comparison should cover the declaration kind, universe-parameter treatment, type, body or inductive metadata, and the identities of constants on which the shortcut depends. Display binder names may be ignored under a specified equivalence; arbitrary logical computation must not be needed merely to recognize the expected definition. A same-shaped reference to a different dependency is not the same trusted primitive.

Generate the reference material from the pinned export using the existing generation workflow; do not hand-maintain a parallel primitive theory. Hashes can identify where to look, but must not be the final authority. A collision-resolving comparison must itself avoid trusting caller-supplied node IDs or metadata (R43).

A stronger persistent digest can be useful for storage. Merely increasing the width of this affine hash, salting it, or adding another similar modular hash does not address the underlying authorization contract. For this small fixed collection, exact validation is the preferred rule.

**Temporary containment:** where an expected definition cannot yet be validated adequately, leave that shortcut disabled or refuse the unsupported input/profile. Do not compensate by accepting the pin on weaker evidence.

### Regression and closure criteria

Use an environment with the expected `Nat`/equality setup but without the genuine `Nat.add`, following the pattern of the existing hostile test's custom-addition case. [S8]

1. Construct the candidate constant function using ordinary expression constructors; confirm its legacy hash equals the committed pin.
2. Submit it through the normal admission-and-pinning path. If it is a legal custom definition under the namespace policy, it may be admitted, but must **not** be accelerated as core addition. Alternatively, an explicit reserved-core policy may reject it. Rejection by an unrelated malformed fixture does not exercise the bug.
3. In the admitted-custom-definition case, `Nat.add 2 3 = 1781693077243641776` must retain the custom body's meaning; `Nat.add 2 3 = 5` must not become provable by the core addition shortcut.
4. Confirm that the actual pinned `Nat.add` still activates acceleration and that T0's normal cases remain unchanged.
5. Exercise a mismatched dependency and the analogous literal-bearing pins, not just the operation's own body.

A green positive `Init` replay does not substitute for this negative regression: its input contains the expected definitions.

### Small related identity cleanup

`v3/LANGUAGE.md` §3 also calls the same structural hash the content hash for declaration revisions. Do not carry the hash-only equality assumption into durable evidence or revision identity. Either keep collision-resolving canonical content available, or explicitly separate the chosen persistent identity mechanism from this fast in-memory hash. The complete persistence design can remain in its scheduled phase; documenting the distinction should happen now. [S5]

---

## 2. R43 — Protect annotated terms and sessions, not only CheckedEnv

### Source observation

The expression representation caches a structural hash, loose-bound-variable range, free-variable and universe-parameter flags, and an integer node ID. `expr_eq` returns true immediately for matching positive IDs. Traversals use the cached flags and ranges to skip work. `CheckedEnv` carries admitted entries, primitive pins, and a node-ID watermark. The project explicitly records that sealing the checked environment awaits the Phase-2 module surface. [S1], [S2], [S6]

These optimizations are valuable. They also make metadata and allocation provenance part of the checking implementation's invariants. Hiding the `CheckedEnv` constructor alone is insufficient if a client can still submit arbitrary annotated expressions or reuse a checking state incorrectly.

### Requested boundary

External clients supply raw syntax, or handles created by the engine's supported constructors. The ingestion boundary validates structure and reconstructs or verifies cached metadata; it assigns identities in an owned lineage/session. Public callers must not need to choose a correct `St` counter or manufacture a `D` record to use the canonical checker safely.

Distinguish at least conceptually:

```text
raw syntax
    -> validated structural representation / engine-owned nodes
    -> type checking and declaration admission
    -> checked environment
```

These stages need not require redundant full copies or a new universal framework. Re-annotation can preserve sharing with a memoized traversal. Nor does a structurally validated expression automatically have a valid type: metadata validity and logical validity are different properties.

The current flat toolchain profile may temporarily expose internal constructors while callers remain reviewed implementation code. Record that limitation rather than describe the general library boundary as sealed before it is.

### Identity and branch discipline

A positive ID is usable as an equality shortcut only within the domain in which uniqueness has been established. A numeric watermark alone does not prove that two independently created branches or environments cannot reuse the same number.

Use an owned allocation domain, lineage-aware handles, validated re-interning at crossings, or another explicit mechanism. Immutable ancestor nodes may be shared safely; newly allocated nodes from sibling branches cannot be equated solely by their local counters. This is a boundary requirement, not a claim that a current fork exploit was executed.

### Regression and closure criteria

Test raw/API submissions containing conflicting positive IDs, false `has_fvar` or universe flags, incorrect loose-variable ranges, stale cached hashes, foreign-environment nodes, and a fabricated checked-environment/pin record. The public path must reject or reconstruct the untrusted metadata before shortcuts consume it.

Also test supported construction positively: independently built structurally identical anonymous terms compare appropriately; snapshot forks do not collide; failed checks do not pollute later checks; ordinary imported DAGs retain the measured sharing benefit.

**Close R43 when the public admission path enforces these invariants, not when comments merely state them.** Broad external embedding and parallel-search callers should follow that path.

---

## 3. R44 — One user-facing let semantics for L and E

### Source observation

The Stage-0 language draft proposes sequential L `let` and parallel E `let`, leaving the rule for a `fn`'s L interpretation to Stage 1. `prog.shard::ELet` explicitly stores parallel right-hand sides. [S5], [S7]

### Why this is consequential

With an outer binding `x = 1`, consider the same schematic source:

```text
let x = 2
    y = x
in y
```

Sequential binding returns `2`; parallel binding returns `1`. Both meanings are coherent. Selecting between them because a declaration is a `def` rather than a `fn` is inconsistent with the intended common authoring language and complicates the eventual defining-equation bridge.

### Requested decision

Give ordinary V3 S one binding rule across executable bodies and logical terms. My preference is sequential binding, lowered to nested one-binding lets when useful. A uniform parallel rule is also defensible. An explicitly different parallel form is an optional future convenience, not necessary scope now.

The bootstrap/toolchain profile may retain legacy parallel syntax under its identified compatibility profile. That does not require ordinary V3 source to inherit it. The typed migration tool must preserve old behavior by introducing fresh names or otherwise translating dependencies, not by silently changing the meaning of existing groups.

Internal E representation need not change immediately: a sequential public form can lower to nested single-binding `ELet` nodes. The decision is about source meaning, not mandating an AST redesign.

### Regression and closure criteria

Run the shadowing example and dependent right-hand-side examples through the L reader and E reader. Require one chosen meaning in ordinary S, including after future `fn` elaboration to L. Retain a legacy-profile fixture and verify its explicit migration preserves its old result. Once effectful execution is enabled, add a fixture preserving the specified evaluation order.

Resolve this before ratifying §13 item 5 or porting source that depends on it.

---

## 4. R45 — Keep Stage-0 execution separate from logical admission and realization

### Source observation

The Phase-2 draft explicitly permits runnable `fn` bodies with no admitted L meaning, no totality check, and no theorem that cites them as logical functions. The calculator's program half is scheduled before its proofs. This is an intentional staging decision, also listed as temporarily at risk in the compatibility ledger. [S1], [S5]

### Recommendation

Keep that decision. It solves the bring-up dependency without provisional axioms. Make its limits structural in the APIs and visible in output.

A runnable program, an admitted L declaration, and a checked executable realization are different objects. A successful execution result is not a receipt that the eventual `fn` contract holds. A parsed measure is recorded syntax until the required termination evidence and logical admission exist.

Schematically, public status should distinguish:

```text
RunnableStage0(program)
AdmittedLogical(declaration)
CheckedRealization(declaration, program, evidence)
```

These names are illustrative, not proposed final syntax. No new user-facing keyword is required merely to implement the distinction.

Do not allow runtime linking against an E signature to discharge a logical view requirement by itself. Likewise, `fulfills`, accepted artifact claims, or removal of proof-justified checks must not silently promote a runnable Stage-0 body into admitted mathematical evidence. Explicit assumptions remain assumptions under policy until discharged.

This does not prevent proving facts about program syntax under a separately established operational model. The prohibition is citing a provisional body *as though it were already an admitted total logical definition*.

### The host/subject distinction remains important

The checker implementation may itself execute under reviewed, uncertified infrastructure while its own verification is incomplete. That implementation trust does not entitle the checker to accept incomplete declarations submitted as its subject. Do not turn this staging fix into a new requirement to certify Rust before K may check explicit L proofs.

### Regression and closure criteria

A self-recursive Stage-0 candidate runs only within the evaluator's stated resource behavior, can exhaust, and gains no logical equations or realization receipt. A terminating candidate returning a value still cannot satisfy an unrelated proof requirement by execution alone. An explicit L theorem can be checked independently, and a genuine checked realization remains distinguishable from a merely linked function.

When Stage 1 arrives, promotion occurs by checked admission and correspondence, not by changing a phase flag on old records. This is the exit condition for the temporary guarantee gap.

---

## 5. R46 — Keep the fast route; characterize its costs and assurance precisely

### Source observation

The repository records the following whole-export comparison: [S1]

| Route | Recorded elapsed time | Recorded peak resident memory |
|---|---:|---:|
| Direct Rust execution of K | 4,591 s | 8.0 GB |
| Compiled K | 745 s | 30.2 GB |

The record also reports a roughly 25-minute interpreted outlier in `WellFounded.partialExtrinsicFix₃_eq_partialExtrinsicFix`. These figures were not independently rerun here. The compiled route is explicitly unproved; interpreted execution remains the reference, and the harness compares the two on a fixture/prefix before the full native replay. A full log tie is also recorded. [S1], [S9]

### Recommendation

Continue using compiled K for the expensive gate. Treat its output agreement as measured execution conformance, not as a compiler-correctness theorem. Keep the source revision, executable identity, export pin, execution route, and remaining execution trust visible in the result record.

Before deciding whether to redesign memory management, measure retained bytes or counts by category: export reference tables, admitted declarations, transient expression construction, memo tables, and host representation overhead. Preserve a small prefix workload and the costly theorem/closure as repeatable performance fixtures.

Measure cold load, warm checking, peak memory, and memory retained after contexts are released separately. The whole `Init` import is a large coverage workload; it is not the memory contract for a small embedding client.

For harness hardening, assert successful execution and expected coverage for each required comparison, in addition to comparing output. Keep expected name/count coverage explicit when comparing a prefix against a larger oracle. This is a requested validation discipline, not a claim that a particular failed CI run was observed.

### Completion criterion

R46 does not require a new performance target or formally verified bootstrap. It requires reproducible route labels and measurements, with an identified cause or scoped follow-up for the largest retained-memory component and the outlier. Optimize from those observations while keeping admission semantics unchanged.

---

## 6. R47 — Keep prefix imports and compatibility gaps from defining the long-term API

### Source observation

The draft imports `Init` as a dependency-ordered export prefix ending at a named declaration. Prefixes are nested and a combined load checks the larger one. The draft also introduces a detailed compatibility ledger with explicit deferred, dropped, and at-risk features. [S5]

Both are useful bring-up mechanisms. Neither requires building a complete package manager before the first application runs.

### Requested guardrails

Treat prefix choice as a loading/scope mechanism, not as the ultimate semantic identity of every declaration it happens to contain. For the same pinned export, enlarging a prefix must not turn an unchanged earlier declaration into different mathematics. The environment revision and loaded scope can change; the referenced declaration and its actual dependencies remain distinguishable from those changes.

This is a proposed identity guardrail following the review's warning about prefix coupling. The current draft describes the import identity using the pin and prefix name; it needs to say which uses of that identity concern the load and which concern declarations.

Keep prefix-based loading behind the importer so a future indexed dependency-closure implementation can replace it without rewriting ordinary program and theorem identities. Do not require every eventual embedding application to know the incidental final declaration of a convenient export prefix.

For each at-risk compatibility row, name an owner or owning phase, a concrete consumer, a small regression, and its intended disposition. Prioritize operations needed by the first connected path: modules/views, executable status, ordinary data construction, and the extern byte boundary. Preserve an old proof-step spelling only when it provides useful capability, not merely because it existed.

### Regression and closure criteria

Load two nested prefixes of the same pin and confirm that their shared declarations retain the intended identities. An incompatible pin must fail the declared compatibility policy rather than alias by spelling. A small application should not need the entire export. At-risk rows exercised by the first consumer must either work or produce a named, intentional refusal.

The import index itself can remain deferred; the identity rule cannot be left to accidental cache behavior.

---

## 7. R48 — Report demonstrated progress without overstating completion

### Source observation

The foundation banner still says Phase 0 is open and that nothing is implemented, while `v3/README.md` records the completed full replay and Phase-2 work. The same README explicitly keeps sealing the checked environment open. [S1], [S10]

### Requested change

Keep the normative document authoritative about the rules. Make the V3 status record authoritative about implementation progress, with dated evidence and open obligations. Replace obsolete blanket status text with a pointer rather than duplicate a volatile phase summary across documents.

Distinguish at least:

- positive-library replay and generated-declaration agreement;
- axiom-closure comparison;
- hostile-input coverage, including R42's collision regression;
- whether the public raw-to-checked boundary is enforced;
- whether an executable route is tested, trusted, or proved;
- the temporary Stage-0 absence of logical `fn` admission.

Do not retract valid historical replay results because a new negative case was found. Equally, do not use the positive milestone to mark the admission surface generally complete. A tracking item closes on its stated regression or gate, or is explicitly superseded by a named owner—not merely because a newer phase began.

The already acknowledged checked-environment gap and R42 should appear beside the Phase-1 coverage result until resolved. That is ordinary engineering status, not a request for an entirely new assurance taxonomy.

---

## 8. Implementation order and response requested

**Immediate:** reproduce R42 through K, fix accelerator authorization, regenerate the reference material, and run the hostile plus normal replay checks. Do not hand-patch generated pin data.

**With the current loader/module slice:** enforce R43 and R45, choose the uniform binding rule in R44, and settle R47's identity distinction. Keep direct execution available throughout bring-up.

**Alongside that work:** retain the compiled route, add R46's targeted measurements, and update R48's status records.

Then complete the already planned connected path: imported logical declaration, explicit checked realization, a caller, a claim, and independently checkable evidence. Phase-0 execution of the program half is useful progress, but only mark each later guarantee complete when its evidence exists.

Please answer **accept / amend / defer** for R42–R48. For implementation fixes, give the regression location and resulting commit or test evidence. For deferred work, give its owning phase and the restriction that keeps the unfinished boundary from being relied on meanwhile. R44 requires an actual language choice, not only acknowledgment of the ambiguity.

**No new foundation, general closure runtime, collector, or certified-Rust prerequisite is requested. The goal is to preserve the implementation momentum while removing the places where a fast approximation or temporary representation could acquire more authority than it deserves.**

---

## Appendix A — Reproducing the accelerator hash collision

This dependency-free Python calculation reproduces the relevant formulas from the pinned `expr.shard`, the `identity_hash` composition from `add.shard`, and the committed `Nat.add` pin. It does not run the Shard checker or prove that a particular submitted theorem is accepted. Binder display names and binder information do not participate in this structural hash. [S2], [S3], [S4]

```python
# Python 3. No third-party dependencies.
M = 2**61 - 1
BASE = 1_000_003
CANDIDATE_VALUE = 1_781_693_077_243_641_776
EXPECTED_ADD_PIN = 606_620_205_132_656_612


def mix(a: int, b: int) -> int:
    return (BASE * a + b + 1) % M


def mix3(a: int, b: int, c: int) -> int:
    return mix(mix(a, b), c)


def hash_bytes(data: bytes, initial: int = 5) -> int:
    result = initial
    for byte in data:
        result = mix(result, byte)
    return result


# Name = Str(Anon, "Nat"); hash(Anon) = 7.
nat_name = mix(7, hash_bytes(b"Nat"))
# Const Nat []: hash_levels([], 1) = 1.
nat_type = mix3(41, nat_name, 1)
# Pi a Nat (Pi b Nat Nat); Pi's tag is 53.
signature = mix3(53, nat_type, mix3(53, nat_type, nat_type))
# Lam a Nat (Lam b Nat (LitNat CANDIDATE_VALUE)); Lam's tag is 47.
body = mix3(
    47, nat_type,
    mix3(47, nat_type, mix(61, CANDIDATE_VALUE)),
)
identity = mix(signature, body)

assert mix(61, 0) == mix(61, M)  # A simpler literal collision.
assert identity == EXPECTED_ADD_PIN
print(f"signature={signature}")
print(f"body={body}")
print(f"candidate_identity={identity}")
print(f"committed_pin={EXPECTED_ADD_PIN}")
```

Output independently obtained for this memo:

```text
signature=1170759705707887636
body=1506445342912286541
candidate_identity=606620205132656612
committed_pin=606620205132656612
```

Port this arithmetic fixture into the hostile battery using the engine's normal raw or anonymous constructors. The end-to-end regression must reach the actual pinning decision. The Python agreement alone closes neither the kernel test nor the fix.

---

## Source references

All repository references below are fixed to `d6f1fd66aa13f40250e005aed1d1422eb8b29ffa`. They support source observations and recorded results. Proposed fixes and inferred failure modes remain the review's analysis.

[S1]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/v3/README.md
[S2]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/v3/kernel/expr.shard
[S3]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/v3/kernel/add.shard
[S4]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/v3/kernel/accel_pins.shard
[S5]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/v3/LANGUAGE.md
[S6]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/v3/kernel/env.shard
[S7]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/v3/kernel/prog.shard
[S8]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/v3/kernel/test/hostile_test.shard
[S9]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/v3/t0_full.sh
[S10]: https://github.com/computer-whisperer/shard/blob/d6f1fd66aa13f40250e005aed1d1422eb8b29ffa/docs/FOUNDATION.md

| Ref | Source and inspected subject |
|---|---|
| [S1] | `v3/README.md`: phase records, full replay, axiom comparison, execution measurements, unresolved checked-environment boundary |
| [S2] | `expr.shard`: representation invariants; `mix`, `hash_lit`, constructors, `expr_eq`, cached flags and ranges |
| [S3] | `add.shard`: `identity_hash`, `pin_if_matches`, `check_decl_pinning`, environment extension |
| [S4] | `accel_pins.shard`: generated reference hash table, especially `Nat.add` |
| [S5] | `v3/LANGUAGE.md`: Stage 0, identities and prefix imports, binding rules, compatibility ledger, decisions for ratification |
| [S6] | `env.shard`: raw/checked representations, pin list, watermark, outcome definitions |
| [S7] | `prog.shard`: E program data, parallel `ELet`, recursion metadata and execution outcomes |
| [S8] | `hostile_test.shard`: existing custom `Nat.add` positive/negative case and admission fixtures |
| [S9] | `t0_full.sh`: fixture/prefix execution comparison and full-replay gate |
| [S10] | `docs/FOUNDATION.md`: ratified contract and stale implementation-status wording in its banner |
