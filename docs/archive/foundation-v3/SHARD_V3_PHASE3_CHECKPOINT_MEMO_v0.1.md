<!-- Produced 2026-10-13 through `codex exec` (model gpt-6-astra, reasoning xhigh, read-only sandbox, 991 s) over the tree at bea4345, from the brief SHARD_V3_PHASE3_CHECKPOINT_BRIEF_v0.1.md beside this file, on the user's steer ("feel free to use codex with cli options for an outside perspective"). Archived verbatim; answered in docs/records/FOUNDATION.md §4.12. -->
# Shard V3 — Phase-3 Checkpoint Memo

**Version:** 0.1  
**Baseline:** [`bea43451a653e46f0e7dba86b567a58723dca23c`](https://github.com/computer-whisperer/shard/commit/bea43451a653e46f0e7dba86b567a58723dca23c)  
**Reviewer:** GPT-6  
**Date:** 2026-10-13  
**Items:** R78–R85

## Scope and evidence limits

I read the phase-2 checkpoint memo first, then the requested law, design, records, authoring report and implementation material. The consolidation comparison covered every rule in Definitions 3.13–3.16, I 3.20–3.21b, and Producers/store 3.22 against their recorded designs and as-built amendments. Source tracing concentrated on admission, policy, discharge, realization, implementation forks, proof replay and release verification. This was not an exhaustive verification of K’s typing and inductive-checking algorithms.

**Executed:** repository and text-inspection commands only. I ran no evaluator, compiler, test suite, export command or Init-index helper. All acceptance failures below are source-traced deductions, not reproduced runtime results. Historical test results remain the records’ reports.

The checkout’s HEAD changed during the review. Subsequent inspection used Git objects at the requested baseline; later changes are outside this memo. I made no filesystem changes.

**Recommendation:** accept the recorded functional progress, but do not treat the K seal or release gate as closed at this SHA. R78–R80 expose admission failures. The native opener is useful, provided its design preserves the bootstrap’s authority and makes its coverage and comparison boundary explicit.

| Priority | Item | Kind | Required disposition |
|---|---|---|---|
| P0 | R78 — Public unchecked admission and unconditional pinning | **defect** | Close before expanding reliance on K’s sealed API |
| P0 | R79 — Release verification does not enforce a fixed target and policy | **defect** | Close before treating stored output as authorized release evidence |
| P0 | R80 — Release failures can produce exit zero | **defect** | Close before using the verifier as an automated gate |
| P1 | R81 — Consolidation retained superseded rules | **drift** | Correct the current statements and close-out claims |
| P1 | R82 — Requirement and reconstruction identity are incomplete | **contract gap** | Settle before persistent caching, prepared handles and bulk migration |
| P1 | R83 — Close-out ledger needs narrower coverage statements | **drift** | Amend the ledger without reopening explicit carries |
| P1 | R84 — Native opener needs a complete transformation and parity contract | **risk** | Ratify the contract and pass discriminating regressions before suite migration |
| P2 | R85 — Occurrence provenance will become expensive to retrofit | **risk** | Establish the representation before incremental authoring work spreads |

## R78 — The public K view exposes ways to manufacture an unchecked `CheckedEnv`

**Kind: defect — source-traced.**

The seal hides constructors but exports operations that defeat the invariant those constructors protect.

The public view exposes `empty_run(pins_mode, ax_mode)`, `process_record`, `run_env`, `run_with_env` and `run_admitting`. A client needs no private import to select either privileged mode. [v3/kernel/k/mod.req.shard:58–96](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/mod.req.shard#L58-L96)

There are two independent paths:

1. **Unchecked declarations.** `run_admitting` sets a caller-controlled Boolean. `check_into` then calls `admit_decl_pinning` instead of checking. That path inserts axioms, definitions, theorems and opaque declarations directly; the theorem case performs no typing check at all. Processing an ill-typed theorem record therefore produces `VAccept` and an environment retrievable through `run_env`. [v3/kernel/k/import.shard:458–480](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/import.shard#L458-L480), [v3/kernel/k/add.shard:355–382](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/add.shard#L355-L382)

2. **Unauthorized accelerators.** `empty_run True …` enables unconditional pinning after declaration admission. `pin_names_unconditionally` checks only whether the name is a pin candidate. It bypasses the exact identity-closure comparison used by ordinary admission. A well-typed counterfeit candidate can consequently receive accelerator authority. [v3/kernel/k/import.shard:474–477](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/import.shard#L474-L477), [v3/kernel/k/import.shard:607–615](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/import.shard#L607-L615), [v3/kernel/k/add.shard:238–264](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/add.shard#L238-L264)

The intended unchecked-replay caller is understandable: `od_admit` transfers records already checked in the Init run into the loader’s environment. But the API accepts ordinary records and a Boolean, not evidence that binds those records to that check and the destination environment. The intended restriction exists in comments and caller convention. [v3/kernel/initload.shard:484–504](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/initload.shard#L484-L504)

**Requested change — implementation.** Remove these powers from the ordinary sealed client interface. Recheck transferred declarations, or introduce a K-owned checked-transfer object whose construction and application enforce record identity, dependency compatibility and destination-name constraints. Generator-only pinning must not yield an environment usable as an ordinarily checked environment. Merely moving an unrestricted wrapper outside K would preserve the hole.

**Regression.** Add clients outside `k/`, loaded through the V3 loader, attempting:

- unchecked admission of a theorem whose proposed proof is its statement;
- unchecked replacement of an existing declaration;
- unconditional pinning of a same-named, differently defined accelerator candidate;
- application of a checked transfer to an incompatible environment.

The existing seal harness is the right execution boundary: it deliberately exercises clients through the V3 loader because bootstrap flat loading does not enforce the seal. [v3/kernel/test/k_clients_test.sh:2–19](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/test/k_clients_test.sh#L2-L19)

**Completion gate.** Every public operation producing a `CheckedEnv` preserves its checked-environment invariant. The negative clients fail through the sealed interface; legitimate Init transfer and pin generation still work through explicitly constrained paths.

## R79 — `verify_release` accepts an unauthorized axiom in place of a retained theorem

**Kind: defect — source-traced.**

The verifier checks whether stored declarations are valid K inputs. It does not establish that they prove the expected release claims under the expected policy.

A concrete substitution requires no hash collision:

1. Keep a stored theorem’s name, universe parameters and type.
2. Replace its declaration record with an axiom record.
3. Leave its order entry and fingerprint unchanged.

The importer accepts that axiom representation, and ordinary K admission correctly permits a well-formed axiom. `identity_hash` hashes only the type for both theorem and axiom entries, so the fingerprint remains identical. The verifier compares name and fingerprint, with no declaration-kind or assumption-policy check. [v3/kernel/k/import.shard:233–255](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/import.shard#L233-L255), [v3/kernel/k/add.shard:164–189](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/add.shard#L164-L189), [v3/kernel/k/add.shard:217–229](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/add.shard#L217-L229), [v3/kernel/verify_release.shard:51–75](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/verify_release.shard#L51-L75)

This is an authorization failure, not a claim that K should forbid axioms. The verifier’s entire invocation supplies only the store, export and index; it receives no independently fixed target or policy. Its target inventory comes from the store’s own `order`. [v3/kernel/verify_release.shard:167–193](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/verify_release.shard#L167-L193)

The discharge representation makes the missing policy boundary consequential even for honestly produced stores. The loader settles obligations by rewriting its accounting records; storage selects `ACCEPT` and `PARAM` constants from the environment and does not retain the `DISCHARGE` relation as such. Thus replay must distinguish declared parameters and discharged obligations from newly introduced assumptions—it cannot infer that distinction from K acceptance alone. [v3/kernel/loader.shard:693–741](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/loader.shard#L693-L741), [v3/kernel/load.shard:123–142](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/load.shard#L123-L142)

The law explicitly requires fixed-target and policy checking without the elaborator. [docs/FOUNDATION.md:767–784](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/FOUNDATION.md#L767-L784)

**Requested change — implementation and artifact contract.** Give verification an independently fixed release manifest: required claims, declaration kinds, dependency identities and allowed assumptions. Preserve sufficient evidence to account for discharged obligations and intentionally abstract parameters. Bind retained P to that manifest by an actual evidence identity; the report fingerprint is explicitly unsuitable as authority.

**Regression.** Starting from a valid bundle:

- replace a theorem with a same-name, same-type axiom;
- replace its proof with one using a prohibited axiom;
- remove a required target from `order`;
- remove discharge evidence while retaining the obligation axiom and its consumers.

Each must fail the relevant release contract. An explicitly conditional bundle should remain verifiable as conditional, with its parameters reported.

**Completion gate.** Cold verification establishes the specified claims and their permitted assumption closure. Acceptance of arbitrary well-typed declarations is no longer sufficient.

## R80 — Release failures can cancel out of the exit status

**Kind: defect — source-traced.**

`vr_store` increments `n` once per file listed in `order`, while `accepted` counts declaration records processed inside those files. The parser permits multiple declaration records per file. The exit calculation is:

```text
n − accepted + malformed + mistied + missing + taken + Init failures
```

Rejected, exhausted, mismatched and unsupported store counts are printed but are not independently included. [v3/kernel/verify_release.shard:122–139](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/verify_release.shard#L122-L139), [v3/kernel/verify_release.shard:184–191](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/verify_release.shard#L184-L191)

Two source-level counterexamples follow:

| One listed file contains | Relevant counts | Exit contribution |
|---|---|---|
| Expected accepted declaration, followed by an extra rejected declaration | `n=1, accepted=1, rejected=1` | `0` |
| Expected accepted declaration, followed by an accepted declaration under another name | `n=1, accepted=2, mistied=1` | `0` |

`vr_lines` continues after both rejection and mistie. [v3/kernel/verify_release.shard:62–73](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/verify_release.shard#L62-L73)

There is also a direct mismatch case: an inductive declaration is counted accepted before its exported metadata is validated; metadata disagreement adds a mismatch count while retaining the acceptance count. The release formula does not independently reject that mismatch. [v3/kernel/k/import.shard:490–501](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/k/import.shard#L490-L501)

The existing tamper tests replace or remove records within otherwise single-declaration files. They do not cover this cardinality mismatch. [v3/kernel/test/store_test.sh:39–54](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/test/store_test.sh#L39-L54)

**Requested change — implementation.** Make success an explicit conjunction: valid order entry, expected declaration cardinality, correct binding, and zero failures of every verdict class. Return a fixed nonzero status on failure; retain detailed counts in the report. Do not encode failure as subtraction between quantities with different units.

**Regression.** Pin both examples above, an inductive metadata mismatch, an unsupported extra declaration, and zero/multiple declarations in one file. Assert both the diagnostic and nonzero process status.

**Completion gate.** No failed verdict or malformed store structure can produce exit zero, regardless of how many other declarations are accepted.

## R81 — The consolidation does not consistently state the current rules

**Kind: drift.**

The consolidation promises a current statement for each rule, with superseded rules identified. Several Definitions paragraphs instead preserve obsolete implementation descriptions beside their replacements. [v3/LANGUAGE.md:1825–1831](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L1825-L1831)

Four corrections are needed:

- **Nat operators.** The 3.13 statement says `-`, `/` and `mod` at `Nat` are refused. The record explicitly reverses that refusal for the typed route while retaining the E-first restriction. Current elaboration resolves these operators to `Nat.sub`, `Nat.div` and `Nat.mod`. [v3/LANGUAGE.md:1854–1864](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L1854-L1864), [docs/records/FOUNDATION.md:3690–3698](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3690-L3698), [v3/kernel/elab.shard:900–905](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/elab.shard#L900-L905)

- **Structural recursion.** The 3.14 statement retains positional `below` lookup and a surviving `recursion_depth` obstacle. The record says the obstacle is gone and lookup proceeds by type; `pj_search` implements that latter account. [v3/LANGUAGE.md:1874–1886](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L1874-L1886), [docs/records/FOUNDATION.md:4026–4036](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L4026-L4036), [v3/kernel/define.shard:228–265](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/define.shard#L228-L265)

- **Constructor support.** “`T.casesOn` always” drops eligibility conditions present in the record. The implementation can return no bundle for indices, nested/mutual structure, an unsuitable recursor or generation failure. [v3/LANGUAGE.md:1929–1935](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L1929-L1935), [docs/records/FOUNDATION.md:3468–3484](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3468-L3484), [v3/kernel/define.shard:1094–1120](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/define.shard#L1094-L1120)

- **Obligation discharge.** The current statement says K checks the proof “in the environment without the obligation.” The implementation checks in `Load.env l`, then rejects a closure reaching the obligation. That matches the recorded as-built account, not the compressed sentence. This distinction identifies where the circularity defense actually lives. [v3/LANGUAGE.md:2027–2036](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L2027-L2036), [docs/records/FOUNDATION.md:4096–4107](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L4096-L4107), [v3/kernel/loader.shard:693–716](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/loader.shard#L693-L716)

The §11 disposition also says `Name` literals and symbols landed at 3.18, contradicting item 51’s explicit deferral and §8.4’s current symbol limitation. [v3/LANGUAGE.md:3458–3462](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L3458-L3462), [v3/LANGUAGE.md:4140–4145](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L4140-L4145), [v3/LANGUAGE.md:2166–2170](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L2166-L2170)

The other two complete prose samples had different outcomes:

| Facility | Comparison result |
|---|---|
| **I, 3.20–3.21b** | The closed-goal representation, sequential replay, pending `sorry`, arithmetic certificate/reconstruction distinction and 3.21b corrections substantially preserve the recorded account. The implementation’s block runner also keeps `sorry` pending through finalization. The unresolved issue is reconstruction identity, R82—not evidence of unchecked proof acceptance. [v3/LANGUAGE.md:2268–2457](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L2268-L2457), [v3/kernel/tactic.shard:1796–1853](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/tactic.shard#L1796-L1853) |
| **Producers/store, 3.22** | The compression largely preserves the record, including the weak fingerprint-based store tie. Consequently R79 is principally an implementation-versus-law gap inherited by the consolidation. The current verifier invocation should also name `DIR EXPORT INDEX`, replacing the retained `DIR CHUNK…` spelling. [v3/LANGUAGE.md:2466–2524](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L2466-L2524), [docs/records/FOUNDATION.md:5806–5846](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L5806-L5846), [v3/kernel/verify_release.shard:171–193](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/verify_release.shard#L171-L193) |

**Requested change — contract documentation.** Rewrite the current statements around final behavior, retaining explicit supersession pointers where slice numbering requires historical entries. Leave the verbatim record unchanged.

**Regression.** Use typed Nat arithmetic, recursion beneath a `let` or argument match, an ineligible constructor bundle, and direct/indirect discharge cycles as executable examples attached to the corrected rules.

**Completion gate.** A reader following §8.4 alone reaches one answer for each behavior. Search the current-document set for the superseded claims before declaring consolidation complete.

## R82 — A statement fingerprint is not the resolved requirement; a schema number is not a reconstruction identity

**Kind: contract gap.**

The producers section calls `hash_expr` of the closed statement “the resolved requirement.” The law’s requirement also includes referenced declarations, selected instances and coercions, policy and environment dependencies. [v3/LANGUAGE.md:2474–2477](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L2474-L2477), [docs/FOUNDATION.md:525–538](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/FOUNDATION.md#L525-L538)

The implementation compares only that statement fingerprint when choosing an existing sidecar entry. Constant-expression hashes include names and universe arguments, not declaration revisions. Consequently changing a referenced definition’s body while retaining its name can leave the sidecar key unchanged; an explicit policy change also does not enter this comparison. [v3/kernel/reader.shard:232–261](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/reader.shard#L232-L261), [v3/kernel/reader.shard:320–330](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/reader.shard#L320-L330), [v3/kernel/expr.shard:329–350](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/expr.shard#L329-L350)

Replay still elaborates the block and K checks its resulting proof. This finding alone does **not** establish acceptance of an invalid theorem. It establishes that the promised detection of a changed task is absent.

The version story has the same compression. The current account identifies I’s version with `(sidecar N)`; the loader accepts older schemas and replays their blocks through the current implementation. No reconstruction-version dispatch or retained-P comparison appears on that path. The law separately requires reconstruction changes yielding different valid P to be reported and migrated. [v3/LANGUAGE.md:2877–2885](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L2877-L2885), [v3/kernel/loader.shard:293–324](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/loader.shard#L293-L324), [docs/FOUNDATION.md:720–735](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/FOUNDATION.md#L720-L735)

Nor should deterministic NDJSON be silently promoted to the law’s final canonical-P identity: current structural sharing deliberately compares binder display names, whereas the law separates display names from proof identity. [v3/kernel/store.shard:256–269](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/store.shard#L256-L269), [docs/FOUNDATION.md:775–781](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/FOUNDATION.md#L775-L781)

**Requested change — contract, then implementation.** Define three distinct persisted identities: resolved requirement, reconstruction contract and retained P. Include semantic dependency revisions and policy in the first; identify the bounded reconstruction semantics in the second; make the third satisfy the canonical encoding contract. Keep producer strategy and origin metadata separate. Until implemented, name the existing key a statement fingerprint and record the remaining obligations explicitly.

**Regression.** Keep the printed proposition unchanged while changing a referenced body or policy: report a changed requirement. Change producer hints only: reuse remains valid. Change reconstruction semantics while producing different valid P: report an evidence change. Change only binder display names or origins: preserve canonical P identity.

**Completion gate.** These distinctions are settled before phase-4 caches and prepared handles adopt persistent keys, and implemented before bulk-port tools use those keys to police frozen requirements. Otherwise the eventual correction reaches sidecars, stores, handles and migration records simultaneously.

## R83 — The close-out ledger is defensible only with explicit limits on “covered”

**Kind: drift.**

I would retain the functional milestones and explicit rulings, but amend the following interpretations of the ledger.

| Ledger area | Review disposition |
|---|---|
| Whole E, Stage 1, arithmetic, Init, measured pipelines | Retain as recorded milestones, subject to the admission defects above. This review did not rerun their pipelines. [docs/records/FOUNDATION.md:3200–3204](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3200-L3204), [docs/records/FOUNDATION.md:3212](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3212) |
| Stage 2 and goal graph | Accept the stated sequential scope and the explicit phase-4 carry for shared holes and incremental states. Do not shorten this later to “incremental goal graph complete.” [docs/records/FOUNDATION.md:3202](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3202) |
| Library modules, former axioms, migration table, ghost refinement | “Covered as ruled” is acceptable for replacing wrapper modules with migration records. The harness checks named theorem evidence and assumption closures, rather than merely file presence. [docs/records/FOUNDATION.md:3205–3208](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3205-L3208), [v3/kernel/test/std_test.sh:23–69](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/test/std_test.sh#L23-L69) |
| Static package; T2/T3 | Accept the documented artifact boundary: specialized E programs and refusal/budget behavior. The report explicitly excludes native compilation of those user programs. [docs/records/T2-T3-2026-10-11.md:11–23](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/T2-T3-2026-10-11.md#L11-L23), [docs/records/T2-T3-2026-10-11.md:49–65](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/T2-T3-2026-10-11.md#L49-L65) |
| First release bundle | Retained declaration records exist; the stronger law §7.5 release guarantee is not covered. Amend this row pending R79–R80. [docs/records/FOUNDATION.md:3211](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3211) |
| T9 small and T9 again | Accept the agreed local protocol. The second run expressly retains route 2 and defines success for the diagnostics by goal/call identification. It does not establish the full source-origin requirement or the compiler-class authoring experiment. The route departure was ratified. [docs/records/T9-2026-10-13.md:3–30](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/T9-2026-10-13.md#L3-L30), [v3/LANGUAGE.md:4460–4469](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L4460-L4469) |
| T10 | Accept the producer/replay and syntax-schema experiments. Do not treat them as reconstruction-version migration evidence: the test removes a schema header and verifies successful replay, which is a narrower experiment. [docs/records/T10-2026-10-12.md:73–90](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/T10-2026-10-12.md#L73-L90), [v3/kernel/test/tactic_lib_test.sh:79–88](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/test/tactic_lib_test.sh#L79-L88) |
| Stage 3; branch-local proof; imported-theorem fixture; explicit carries | Retain the bounded statuses. In particular, `realize_imported` genuinely cites the imported theorem before and after attachment. The checked-instance/two-implementation carry remains phase 5; the Symbol flip remains deferred. [docs/records/FOUNDATION.md:3217–3221](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3217-L3221), [v3/pins/loader/realize_imported/main.shard:8–13](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/pins/loader/realize_imported/main.shard#L8-L13) |
| LEAN, document disposition, R60, consolidation, ratification | Retain the authoring/document/ratification milestones. Correct the R60 landed claim and consolidation completion under R81. Document disposition is not evidence that every sentence in the replacement document is accurate. [docs/records/FOUNDATION.md:3210](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3210), [docs/records/FOUNDATION.md:3222–3225](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/FOUNDATION.md#L3222-L3225) |

**Requested change — contract documentation.** Distinguish “artifact exists,” “recorded experiment passed,” “covered under a ratified restriction,” and “full law contract established.” Carry unresolved acceptance clauses by name and phase. Preserve the author’s verbatim report.

**Regression.** Each amended row should point to a discriminating check, not merely a positive example. The release tampering cases, dependency/version changes and occurrence-origin cases in this memo supply the missing checks.

**Completion gate.** A reader can reconstruct what phase 3 established and what remains without resolving contradictions between the ledger, §11 and the underlying reports. I would not reject the library or route-2 readings that were explicitly ruled; I would reject interpreting them as evidence of broader guarantees.

## R84 — The native opener needs a complete coverage and transformation contract

**Kind: risk.**

The proposed direction is appropriate as an execution improvement. The existing build already states that route 1 is unproved and the Rust interpreter remains authoritative. Extending its reach should preserve that distinction. [v3/build.sh:2–6](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/build.sh#L2-L6)

The scope needs correction or clarification. The baseline suite has **23 Shard test entrypoints plus 18 shell entrypoints**. The parity inventory includes five drivers, `calc_harness`, eighteen public Shard tests and five tests inside K: **29 program roots**, before its additional fixtures. If “nineteen harnesses” means the public tests plus `calc_harness`, it excludes the five private K tests. [v3/test.sh:23–46](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/test.sh#L23-L46), [v3/kernel/test/parity_test.sh:32–59](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/test/parity_test.sh#L32-L59)

I would require the design to specify:

1. **One V3 interpretation before projection.** Reuse the V3 reader, resolution and record expansion, or demonstrate equivalence against them. Record sugar already defines constructor/accessor names and update behavior; a separate approximate expander would create another language boundary. [v3/kernel/record.shard:4–27](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/record.shard#L4-L27)

2. **Capture-free, injective naming.** Capitalized-local renaming must follow binder scope through patterns, sequential lets and shadowing. Quoted symbols, strings, constructor identities and persisted names must retain their meanings. Any projection to flat names must preserve or replace the existing collision gate. [v3/kernel/test/parity_test.sh:35–53](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/test/parity_test.sh#L35-L53)

3. **A complete engine-dispatch inventory.** Cover direct test roots and every nested driver invocation in shell tests. State explicitly which control operations remain bootstrap operations. Missing native binaries must fail loudly rather than silently satisfying native coverage through fallback. The K-client harness, for example, invokes `load.shard --run` rather than directly executing each test. [v3/kernel/test/k_clients_test.sh:14–19](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/test/k_clients_test.sh#L14-L19)

4. **A precise comparison boundary.** Compare stdout, stderr, exit status and generated evidence artifacts where relevant. Whole suite logs are not deterministic bytes: the runner prints elapsed time, store tests report timings and sizes, and tactic tests create randomly named source roots. Use paired deterministic workspaces or an explicitly limited normalization of observational metadata. Do not normalize verdicts, assumption closures, fingerprints or evidence. [v3/test.sh:23–44](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/test.sh#L23-L44), [v3/kernel/test/store_test.sh:57–73](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/test/store_test.sh#L57-L73), [v3/kernel/test/tactic_lib_test.sh:23–33](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/test/tactic_lib_test.sh#L23-L33)

5. **Artifact freshness and source immutability.** Bind binaries to their complete source closure, pre-pass, lowerer, code generator and relevant compilation configuration. Emit generated files into an isolated derived tree. Preserve the repository-relative identity behavior already required by the build. [v3/build.sh:8–35](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/build.sh#L8-L35)

I would refuse a regex-based binder rewrite, hand-maintained generated kernel copies, success-only parity, or a claim that byte agreement proves the compiler. I would also refuse counting this opener as the Symbol flip: the current rule ties that change to route 1’s chain becoming V3’s own. [v3/LANGUAGE.md:2149–2155](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/LANGUAGE.md#L2149-L2155)

**Requested change — design contract.** Write those boundaries before implementation, including the complete root inventory and a disposition for every shell invocation.

**Regression before running the suite on binaries.** A small differential gate should cover record construction/update, capitalized locals colliding with constructors, nested shadowing, quoted capitalized symbols, dotted-name collisions, a sealed client, normal and rejected K admission, malformed input, exhaustion, nonzero program exit, sidecar output and store verification. Deliberately alter a native verdict and remove a native binary: the gate must detect both.

**Completion gate.** The small gate passes, authored files remain byte-identical, every intended invocation demonstrably selects the native binary, and the bootstrap suite remains an independently executed authority beside the native suite. Native agreement with the defects in R78–R80 would demonstrate agreement, not correctness.

## R85 — Preserve occurrence provenance before incremental authoring and the bulk port

**Kind: risk.**

After admission and persistent identity, occurrence provenance is the current representation choice most likely to impose widespread later work.

The reader returns unlocated `SExpr` values after successful parsing; byte offsets exist for parse errors. The loader then expands those values and carries a file-level context. Tactic failures add a step index and rendered goal, but no source occurrence. [v3/kernel/sexpr.shard:19–31](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/sexpr.shard#L19-L31), [v3/kernel/loader.shard:392–409](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/loader.shard#L392-L409), [v3/kernel/loader.shard:1600–1615](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/loader.shard#L1600-L1615), [v3/kernel/tactic.shard:1796–1798](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/v3/kernel/tactic.shard#L1796-L1798)

The T9 report demonstrates useful goal-level diagnostics. It also makes their scope clear: the second recursive call is identified through `bad.dec_2` and its proposition, rather than a retained origin pointing to the expression that generated it. [docs/records/T9-2026-10-13.md:450–507](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/records/T9-2026-10-13.md#L450-L507)

The law already specifies the required separation: semantic nodes may have many occurrences; origins include source revision, generated-parent links and API/artifact origins, and do not affect canonical P identity. This needs implementation, not a new foundation decision. [docs/FOUNDATION.md:487–500](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/FOUNDATION.md#L487-L500)

**Requested change — bounded architectural preparation.** Before expanding incremental goal APIs, define occurrence identifiers and an external origin map spanning parsing, record expansion, generated obligations and I nodes. Keep K terms location-free. The native pre-pass should preserve this mapping rather than making generated-source positions the only available origins.

**Regression.** Use identical shared terms at two source sites, two similar recursive calls producing different obligations, and an API-built declaration without source text. Diagnostics must select the correct occurrence or report an artifact origin. Whitespace edits and improved origin maps must leave evidence identity unchanged.

**Completion gate.** One end-to-end authoring fixture demonstrates those cases before phase-4 incremental interfaces become established. Complete coverage can then grow with consumers.

For phases 4–7, I would keep the bounded sequence: close admission first; settle persistent requirement/evidence identities before caching and prepared handles; preserve origins before incremental authoring spreads. The explicitly deferred checked module instance can remain in phase 5, and the compiler proof, World-use tests and reflection work remain phase 4’s obligations. Faster execution should make those experiments cheaper without being counted as their completion. [docs/FOUNDATION.md:1183–1202](https://github.com/computer-whisperer/shard/blob/bea43451a653e46f0e7dba86b567a58723dca23c/docs/FOUNDATION.md#L1183-L1202)