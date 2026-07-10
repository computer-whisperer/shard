# TCB.md — the bootstrap trusted computing base

**Status: DRAFT for ratification (trust-residue fork, 2026-07-10).**
The standing discipline for what a shard verdict rests on, what is
explicitly NOT authority, and how trust is re-established after an
engine rebuild. This writes down rules that have governed the repo
since the compiler-chain arc (2026-06-10); nothing here is new policy.

## The principle

**The compiled chain is never the soundness authority.** The native
engines (`bin/shard_eval`, `bin/shard_check`) exist to escape the
interpretation tax in the dev loop. They are built by a temporary,
unproven compiler chain, and no acceptance-grade verdict may rest on
them alone. Every fast-engine convenience below is paired with the
differential gate that keeps it honest; when the two disagree, the
authority side wins by definition.

## The trust roster

A shard acceptance verdict ultimately rests on, in order:

1. **The Rust bootstrap interpreter**
   (`rust_bootstrap/target/release/eval`). The execution authority for
   engine verdicts. Soundness-authority runs — the pre-commit corpus
   diff, sidecar replay, ledger acceptance — invoke it explicitly
   (`EVAL=./rust_bootstrap/target/release/eval`); the test scripts'
   engine ladders say so in their headers.
2. **The checker sources** (`kernel/*.shard`). The checking LOGIC is
   written in shard and reviewed as source; the interpreter only
   executes it. The self-hosted tower (eval.shard → check.shard) means
   the logic is one artifact regardless of which engine runs it.
3. **The reviewed core-math axiom set** (`kernel/facts.shard`, 15
   axioms, ledger kind `'kernel`). The trust floor for proofs: prim
   recurrences and euclidean/ring characterizations, each tagged
   `(kind operational)`. The axiom-scope gate (driver.shard) refuses
   axioms authored anywhere in std/, meta/, models/; the kind gate
   refuses any authored axiom without a `(kind …)` tag.
4. **Per-artifact trust scopes.** App/bin axioms (I/O bolts, bridges),
   kind-tagged and reported per-bin by the LEDGER block: own axioms
   with kinds, upstream/granted split, extern reachability, dead-trust
   flags. Trust granted here is visible, per-artifact, and additive —
   it never widens the floor.
5. **The prim tables, tied by conformance.** Exactly two prim
   implementations exist: `prim.rs::try_apply` (native) and
   `reduce.shard::try_step_prim` (object reducer; eval.shard reuses
   it). The `prim_conformance_*` cargo tests in `rust_bootstrap`
   sweep them against each other over a value matrix; the spec lists
   (`SHARED_INT2` / `OBJECT_ONLY_*`) in lib.rs are the prim-set source
   of truth.

The C compiler, shardfmt, and the test scripts are NOT on the roster:
cc's output is differentially gated (below), shardfmt is a gate that
refuses rather than a transform that is trusted (its R-GATE theorem
says fmt output re-parses structurally equal), and the scripts only
orchestrate verdicts the engines produce.

## Explicitly not authority

`bin/shard_eval`, `bin/shard_check`, `tools/lower`, `tools/codegen`,
`rt.h`, and any binary they produce. These are the DEV LOOP. A fast
engine may propose any verdict; it may not RATIFY one. Concretely:

- Pre-commit corpus verdicts, sidecar replay, and ledger acceptance
  run on the Rust interpreter.
- A **stale `bin/shard_check` is never used** — it is direct-compiled
  from check.shard's whole closure, so stale means silently wrong
  logic. The scripts fall back to `bin/shard_eval` and print a NOTE.
- A **stale `bin/shard_eval` only warns.** It interprets
  `kernel/check.shard` fresh at runtime, so edits to checker-side
  sources (driver, checker, types, admit, check) are picked up with
  zero rebuild. Only its BAKED closure — loader, term, module, reduce,
  reader, evm (eval.shard's imports) — requires `bin/rebuild.sh`.

## The stamps

`bin/engine_stamp.sh` = one sha256 over everything engine behavior
depends on: `kernel/*.shard` (minus derived `*.low.shard`),
`tools/lower/lower.shard`, `tools/codegen/codegen.shard`,
`tools/codegen/rt.h`. `bin/rebuild.sh` records it next to each binary;
the scripts compare before trusting one.

- **Stamp match = "this binary was compiled from these bytes."** It is
  a freshness claim, not a correctness claim — correctness comes from
  the differential gates.
- **Over-flagging is by design.** The stamp hashes all kernel sources,
  a superset of each binary's true closure, so a driver-only edit
  flags shard_eval STALE even though its baked closure is unchanged.
  Stale-but-correct is tolerable; fresh-but-wrong is not.
- **The mid-build hole (written in blood, 2026-06-18):** the stamp is
  computed at build END but sources are read at build START. Editing
  or committing a kernel source while `bin/rebuild.sh` runs produces a
  FALSE-FRESH binary (old code, new stamp). Do not edit kernel sources
  during a rebuild; the hole self-corrects on the next kernel edit.

## Rebuild discipline — how trust is re-established

`bin/rebuild.sh` (~6 min; `bin/rebuild.sh check` ~1 h for the direct
checker) re-establishes a fresh engine by construction and by gate:

1. **The chain always runs on the Rust interpreter** — never on a
   previous native binary. A stale or wrong engine cannot propagate
   itself into its successor.
2. **The fmt gate refuses non-canonical stamp inputs** (a later
   formatting-only pass would flip the stamp of a byte-identical
   engine, permanently flagging it stale).
3. **The stuckness guard runs on the fresh binary**
   (`examples/run_stuckctl.sh`): run-mode stuckness and poisoned
   extern writes must die LOUDLY (exit 4 + the offending head), never
   exit 0. A fresh engine that fails the guard is REFUSED.
4. **The corpus is the acceptance check.** After a rebuild (or any
   kernel edit), gate by the FAIL-set diff against the pinned
   baseline — `run_corpus.sh` is a DIFF tool and exits 0 with red
   targets:

   ```
   awk '/^=== /{t=$2} /^FAIL/{print t, $0}' out.txt | sort | diff fails-base.txt -
   ```

   Engine-suspicious changes additionally re-run the corpus on the
   Rust authority and require byte-identical verdicts across engines
   (the discipline that caught the parallel-let miscompile and
   validated the GC and bignum landings).

## Lifetime

The chain is TEMPORARY (compiler-chain arc). The long-term engine is
the proven lowering pipeline (LOWERING.md, X86.md): shard→shard
refinement with certificates, where the emitted artifact's trust is a
theorem, not a differential. This document governs until that pipeline
replaces the bootstrap chain; the principle — the executing engine is
never the soundness authority, the reviewed sources and the authority
interpreter are — survives the replacement unchanged.
