# Shard
<p align="center">
  <img src="assets/shard.png" alt="shard" width="128" height="128">
</p>

> **State of the tree (2026-11-13).** The kernel this README describes — `kernel/`, the v2 lineage — was
> deleted on 2026-11-13 ([`v3/LANGUAGE.md`](v3/LANGUAGE.md) §8.7, slice 5.0). What runs on this branch is
> **V3**, under [`v3/`](v3/README.md): a kernel implementing Lean 4's rules, its loader, and its own
> compiler back end. The rest of the old tree (`std/`, `meta/`, `models/`, `tools/`, `examples/`, `pins/`)
> is **unchecked text** until each family migrates to V3 — nothing on this branch loads or gates it. The
> proven binaries, the search engine and the corpus described below are the old tree's: they run at the
> tag **`v2-kernel-last`**, the last commit that holds the old kernel, where the commands of the old
> quick start work as written. **Migrated so far:** the formatter — `tools/shardfmt` and `meta/format`
> are [`v3/tools/shardfmt`](v3/tools/shardfmt/README.md) since 2026-11-13 (slice 5.1), their old files
> deleted.

Every mainstream programming language asks one notation to do three
jobs at once: **state what the software must do**, **express how it is
computed**, and **control what the hardware actually executes**. All
three suffer for it — requirements live in tickets and comments,
algorithms are tangled with their optimizations, low-level control is
delegated to a monolithic optimizing compiler you can only trust — and
the result is welded to a single ecosystem. **shard** splits the three
jobs into separate, machine-checked artifacts in one tiny total
language: requirements (`requirement`/`fulfills` contracts), the pure
algorithm (plain functions), and the low-level spelling (proven
refinements, down to machine code), connected by one transitive
**refinement** relation `spec ⊑ … ⊑ code`. Each layer is authored,
audited, swapped, and reused without touching the others. The proof
engine is the key that lets a language this small do all three jobs
at once.

The output is real software, not a verification exercise. A `(bin …)`
declaration builds a standalone Linux x86-64 ELF — no interpreter, no
GC, no libc, and no C in the artifact chain (syscalls are made
directly, against a kernel-interface model written in shard) — whose
bytes are tied by kernel-checked certificates to the program the
proofs are about. The honest residue of hardware is an **explicit
premise list** ("under 2^64", "as long as live data fits"), never a
trusted compiler. [docs/TCB.md](docs/TCB.md) states exactly what is
trusted and why; the current pins run on real silicon.

And because **programs are data** — functions, types, whole
applications are inspectable in-memory values the tooling reasons
about — shard is built to be *embedded* as well as invoked: compiler
infrastructure as a component, not a monolith. If your project reduces
to "an IR for some compute domain plus a sufficiently fast
compiler/evaluator for it" (most interesting projects eventually do),
shard terms are that IR — portable, introspectable, provable when you
want proofs, compilable when you want speed.

## Why shard exists

Five observations, each a reaction to the state of practice:

1. **The trichotomy.** Requirements, algorithm, and low-level spelling
   are different concerns with different owners and lifetimes, yet
   every language forces them through one notation and one compiler.
   The formal split between "here is my pure algorithm" and "here is
   how hardware replicates it efficiently" is strangely rare; shard
   makes it the spine of the whole system.
2. **Build time is practically free; runtime is the
   performance-critical dimension.** Most languages under-exploit
   this: Rust's UOM library encodes SI units in the type system to
   catch dimension errors — a papercut that was always a *build-time
   problem* — and the borrow checker moves runtime hazards into
   pre-run verification, but both are fixed, conservative mechanisms
   baked into the language. shard's proof engine is the graduated
   version: arbitrary per-program facts, checked once at build time,
   with nothing left to police at runtime. One consequence is an
   inversion worth naming: safety comes from the refinement bar
   itself, so memory layout, representation tricks, and hand
   optimization compete only on efficiency and proof effort — an
   aggressive spelling can fail to compile, but it cannot ship a
   hazard ([docs/MEMORY.md](docs/MEMORY.md) §1).
3. **A language should be usable as an IR.** No common language works
   as an embeddable, semantics-clean intermediate representation for
   other systems (wasm comes closest and still misses domains). shard
   is partly a reaction to that: the toolchain — parser, evaluator,
   prover, compilers — is meant to be exploitable *inside* other
   projects, including jit-like systems, not just from a shell.
4. **Verified compilation should not stop at the compiler.** CompCert
   and CakeML prove the bottom half of the tower; shard runs the same
   discipline top to bottom, for general-purpose software, with the
   machine models themselves written in the provable language.
5. **Models write the code now.** shard is LLM-first: designed for
   what a 2026 agentic model can utilize, not for what a human learns
   fastest — a small, regular, corpus-aligned surface with guessable
   names and hardened errors, because the author's context window is
   the manual. The requirement chain is what makes model-written
   software *adoptable*, and it is the trichotomy paying off: you do
   not review ten thousand generated lines; you review the interface
   they provably meet.

The history, honestly: shard began as a bootstrapped-language
experiment about the refinement chain. The deliberately tiny core —
kept simple under threat of making proofs intractable — plus a proof
DSL turned out to be far more powerful than first hoped. The founding
premise and v1→v2 history are archived in
[docs/archive/TRANSFER.md](docs/archive/TRANSFER.md); what this file
describes is what turned out to matter.

## What shard turned out to be

The domain shard landed in was not among the original premises, and it
is the part most worth understanding before reading anything else:
**shard proper is a theoretical language** — unbounded integers, a
free unchecked heap, pure, total. Hardware never enters the language;
it enters as an **emulated model**. `models/wasm` is a pile of
ordinary shard functions that read like a very verbose wasm assembler;
`models/x86` and `models/linux` do the same for the CPU and the
syscall boundary — and the kernel neither knows nor cares that they
describe machines. Compiling is then a theorem, proven in two moves:
refine the algorithm until it is spelled inside the model's
vocabulary, and transliterate that spelling 1:1 into real bytes (with
the transliteration tied back to the certificates, byte for byte).
The machine's limitations surface as honest, explicit premises —
"under 2^64", "while live data fits" — rather than silent truncation
or a trusted optimizer's discretion.

The move this unlocks — one we know few systems attempt — is the
proven **representation swap**: the algorithm you reason about walks
high-level linked lists in the theoretical domain; the artifact you
ship mutates linear memory; and the swap between the two is generated
automatically and certified by the kernel — a theorem, not a compiler
pass. Nothing about the theoretical language is compromised to get
there; [docs/MEMORY.md](docs/MEMORY.md) is the design ledger carrying
this to its full depth.

## How it works

> **FOUNDATION (2026-09-06):** the V3 foundation — a kernel implementing Lean 4's rules exactly, written in first-order shard, with a proof IR and the naming law — is **ratified** ([docs/FOUNDATION.md](docs/FOUNDATION.md)) and is being built under [`v3/`](v3/README.md) beside this tree. This section describes the tree as it runs today; the Rust bootstrap is now the *enduring* bootstrap facility (FOUNDATION §9.1), not a disposable one.

**The tower (the old tree, as the tag `v2-kernel-last` holds it).** A deliberately small **Rust bootstrap**
(`rust_bootstrap/`, ~4k lines, trusted by review)
evaluates *narrow* shard — a minimal subset. Everything else is shard:
the engine (`kernel/eval.shard`, an environment machine with TCO), the
reader, the elaborator, and the proof checker
(`kernel/check.shard`), which is just another shard program the engine
runs. For day-to-day speed the checker and engine are also compiled to
native binaries (`bin/shard_check`, `bin/shard_eval`) by a temporary
native chain — stamp-guarded against source drift, and **never the
soundness authority**: authoritative runs go through the Rust
interpreter path explicitly. When a proven compiled kernel
replaces that chain, the Rust host leaves the trust roster (it is
never deleted by fiat — FOUNDATION §9.1).

**The proof layer.** `(claim NAME GOAL PROOF)` checks a theorem
against a small kernel: equational reduction, structural /
well-founded / subterm induction, cut, bounded enumeration, checked
Farkas arithmetic certificates, and a totality regime in which every
recursive function carries a verified measure
([docs/TOTALITY.md](docs/TOTALITY.md)). Proofs are authored in a DSL
parsed directly to kernel structures; `tools/prove` machine-solves
the mechanical tiers and regenerates sidecars. Structural invariants
ride `(refine BASE PRED)` types ([docs/REFINEMENT.md](docs/REFINEMENT.md)).

**The contract layer.** A directory module's `mod.req.shard` is its
reviewed public interface — opaque types and signatures plus
requirement lemmas; proofs never pierce another module's surface.
Executables declare `(bin …)` artifacts whose acceptance contract is a
`requires` list over their I/O boundary, reported MET/UNMET, plus a
transitive **trust ledger**: every cited axiom (kind-tagged
`operational`/`bridging` — untagged axioms are refused), every
granted fact, every reachable extern. Review attention concentrates
on `mod.req.shard` files; implementations and proofs are fungible
behind them.

**The lowering layer.** Machine targets are ordinary shard libraries
(`models/wasm`, `models/x86`, `models/linux`), so "compiled correctly"
is a theorem, not a compiler property. A `(lib …)`/`(bin …)`
declaration is lowered by untrusted generators (`tools/wasmgen`,
`tools/x86gen`) into a *lowered twin* plus per-function certificates;
gates make the generators irrelevant to trust: regenerated output must
be byte-identical, the certs must check in the kernel, the emitted
image must re-assemble *from the certs* (the byte-tie), and the result
must replay on the real engine — V8 for wasm, the bare CPU for x86.
The same declaration builds both targets. Ratified design ledgers:
[docs/ISA.md](docs/ISA.md), [docs/LOWERING.md](docs/LOWERING.md),
[docs/X86.md](docs/X86.md), [docs/CANON.md](docs/CANON.md) (the whole
tree is written in one canonical, machine-recognized dialect), and
[docs/MEMORY.md](docs/MEMORY.md) (representation and memory
management — draft).

**The product asymmetry**, restated: generation is cheap and
untrusted — an LLM (or a search procedure) proposes definitions,
refinements, lowerings, and their proofs; checking is small and
trusted — one kernel, written in shard, run today on a bootstrap that
will be compiled away.

## What's proven today

- **Native x86-64 binaries from `(bin …)` declarations** — plainly
  executable ELFs (`./addw` and siblings), zero C, direct syscalls
  through the shard-written Linux model, glue that only moves bytes,
  six gates from regeneration to an on-silicon run
  (docs/X86.md §19–§49).
- **The wasm target** — the same lib declarations lowered to wasm,
  differentially replayed in V8; the founding `wasm ⊑ rev_loop ⊑ rev`
  chain and the linear-memory capstone (`examples/mem_reverse.shard`)
  are corpus pins.
- **snake_game_3** — an interactive game binary whose I/O contract
  (two-tier requirement pyramid over an opaque `GameState`) is fully
  met: playable, seeded, proven.
- **The self-hosted checker tower** — eval.rs → eval.shard →
  check.shard; the reader, elaborator, driver, and tracer are all
  shard; the kernel's axiom surface is a reviewed 15-axiom core-math
  set plus tagged extern bolts.
- **A proven std** — arithmetic, division, order, lists, maps, bits,
  bytes, strings, words, rng, sha256 (NIST-pinned), and the mem byte
  substrate, all behind reviewed opaque interfaces with
  corpus-verified implementation proofs.
- **Canonical form as a theorem** — `tools/shardfmt` is contracted to
  refuse output that parses differently; `kernel/canon.shard`
  recognizes the dialect; content addressing rides std/sha256.

## Quick start

Build the Rust evaluator once, then run V3's suite on it and build V3's
native binaries:

```sh
cargo build --release --manifest-path rust_bootstrap/Cargo.toml
v3/export.sh                 # the pinned Lean `Init` export (needs Lean; see v3/README.md)
bash v3/test.sh              # every V3 test entrypoint on the interpreter
v3/build.sh --all            # V3's own back end: each root lowered, written as C, compiled
V3_ENGINE=native bash v3/test.sh   # the same suite on the binaries
```

A single file is loaded, checked and reported by the V3 driver:

```sh
rust_bootstrap/target/release/eval direct v3/kernel/load.shard --root v3 \
  --init .shard-cache/v3-export/init.ndjson --init-index v3/.cache/init.index \
  v3/examples/calc/calc_spec.shard
```

**The old tree's quick start** — `eval run kernel/check.shard FILE`,
`bin/rebuild.sh`, `./run_corpus.sh`, the formatter and the digest tool
on `bin/shard_eval` — needs the old kernel: check out the tag
`v2-kernel-last` and read this section there.

## Repository layout

```
kernel/          ; DELETED 2026-11-13 (the tag v2-kernel-last holds it); was
                 ;   the trust floor: term/reduce/proof/checker + the
                 ;   self-hosted reader, elaborator, driver, engine (ev),
                 ;   canon recognizer, and the reviewed facts.shard axioms
std/             ; the standard library — directory modules with reviewed
                 ;   mod.req.shard interfaces (arith, div, order, nat, list,
                 ;   map, bits, bytes, str, word, mem, rng, sha256)
meta/            ; the meta stdlib: shared machinery for programs-about-
                 ;   programs (plan, invoke, format, shape, proof, sketch …)
models/          ; machine + OS models as ordinary proven libraries:
                 ;   wasm, x86, riscv, pio, linux (the syscall boundary);
                 ;   each with probes/ (validation articles) and diff/
                 ;   (engine differentials)
tools/           ; untrusted toolchain: prove (auto-prover), shardfmt,
                 ;   digest (the stratified read surface), canon,
                 ;   wasmgen/x86gen/impgen/wordgen (generators),
                 ;   lowcheck/bytetie/lowbuild (the gates), build (the
                 ;   product driver + build_products.shard), bench (the
                 ;   temporary native chain lower/codegen: deleted 2026-11-13);
                 ;   tools own their fixtures/ and blueprints/
examples/        ; genuine programs and demonstrations: the bin
                 ;   families (add, addw, sha256sum), feature demos, the
                 ;   stateful apps; *_rejects.shard files are deliberate
                 ;   negative demos (see docs/LAYOUT.md for the layout law)
pins/            ; corpus-law pins: proof/ (proof-layer regressions),
                 ;   lang/ (language + loader semantics), trust/ (trust-
                 ;   boundary refusals) — kernel behavior may not drift
                 ;   from these silently
bin/             ; bin/ci_docs_only.sh is CI's question whether a push
                 ;   changed documentation only (.gitlab-ci.yml's header)
docs/            ; the documentation map below
rust_bootstrap/  ; the Rust host (evaluate-only): the enduring bootstrap
                 ;   facility, never architecture (FOUNDATION §9.1)
shard-viewer/    ; a graphical navigator for shard source (Rust; its own
                 ;   README)
```

## The documentation map

Ratified **scope ledgers** carry the arcs — each records its user
rulings, decision points, slice history, and gotchas, and is the
authority for its area:

| doc | area |
|---|---|
| [FOUNDATION.md](docs/FOUNDATION.md) | **the V3 foundation (RATIFIED 2026-09-06)**: Lean-rule-exact kernel in first-order shard, E, the proof IR, the naming law, the arc; history in [records/FOUNDATION.md](docs/records/FOUNDATION.md), GPT-6's reviews in [archive/foundation-v3/](docs/archive/foundation-v3/) |
| [v3/README.md](v3/README.md) | the V3 tree: package root, the Lean pin, phase status, the port manifest and the shared-type inventory |
| [OVERVIEW.md](docs/OVERVIEW.md) | design intent — the why, in full (§11: the foundation ruling) |
| [LAYOUT.md](docs/LAYOUT.md) | repository layout: the placement rule + the move map |
| [LANGUAGE.md](docs/LANGUAGE.md) | the narrow object language (syntax, semantics) |
| [LEAN.md](docs/LEAN.md) | V3 for a reader who knows Lean 4: the same, the refused with its pointer, the shard-only (law §5.4) |
| [TOTALITY.md](docs/TOTALITY.md) | measures, admission, mutual recursion |
| [REFINEMENT.md](docs/REFINEMENT.md) | `(refine …)` — invariants as types |
| [BOUNDARIES.md](docs/BOUNDARIES.md) | modeling external systems; extern + axiom |
| [ISA.md](docs/ISA.md) | machine models as libraries; target architecture |
| [LOWERING.md](docs/LOWERING.md) | the lowering-form paradigm: statements, certs, gates |
| [CERT.md](docs/CERT.md) | the certificate architecture: conversion, sharing, validators |
| [STORAGE.md](docs/STORAGE.md) | the storage + incremental slice: cert cache, module images, arena |
| [STREAM.md](docs/STREAM.md) | Arc B: the streaming bin — `(bin sha256sum)` made conventional and fast (closed) |
| [COVERAGE.md](docs/COVERAGE.md) | the coverage arc: the generic path spec → imp → x86, shardfmt as the flagship (ratified 2026-08-22; Theorem A + runtime laws C2b-1..4 landed) |
| [IMP.md](docs/IMP.md) | the neutral imperative dialect: spec ⊑ imp ⊑ wasm/x86 |
| [X86.md](docs/X86.md) | the x86-64 target: emitter → bin ladder → World on silicon |
| [CANON.md](docs/CANON.md) | the canonical dialect: rules, census, content addressing |
| [TCB.md](docs/TCB.md) | the trust story: exactly what is trusted, and why |
| [MEMORY.md](docs/MEMORY.md) | representation + memory management (draft) |
| [BUILD.md](docs/BUILD.md) | the build layer: products, profiles, PIN/DERIVE/SYNTHESIZE |
| [SEARCH.md](docs/SEARCH.md) | program search + the lock-step proof engine (arcs complete, on main) |
| [FLOATS.md](docs/FLOATS.md) | rationals + parametric floats: the value-level numeric core |
| [PIO.md](docs/PIO.md) | the RP2350 PIO ISA model + program search |
| [RISCV.md](docs/RISCV.md) | the RISC-V third target: one model, both widths |
| [REVISIT.md](docs/REVISIT.md) | the design-decision ledger: choice + revisit-when |
| [records/](docs/records/) | the dated rung/slice RECORDS split out of the law ledgers (2026-09-02); `X.md §N` for a moved section resolves to `records/X.md` |
| [archive/M3-V1-VS-V2.md](docs/archive/M3-V1-VS-V2.md) | v1 vs v2 proof-effort comparison (history) |
| [archive/TRANSFER.md](docs/archive/TRANSFER.md) | the founding premise and v1→v2 mandate |
| [archive/DESIGN-REVIEW-2026-07-18.md](docs/archive/DESIGN-REVIEW-2026-07-18.md) | the three-way design review at the sha-on-silicon milestone |

## Live arcs

- **THE FOUNDATION ARC (V3) — RATIFIED 2026-09-06; phases 0–3 closed
  (phase 3 on 2026-10-13); phase 4 closed short by ruling 2026-11-13,
  slices 4.0–4.6; phase 5 open — the old kernel removed 2026-11-13 (slice
  5.0), the port by need
  (status and evidence in [`v3/README.md`](v3/README.md))**:
  [docs/FOUNDATION.md](docs/FOUNDATION.md) is the law; the tree is built
  under [`v3/`](v3/README.md) (Lean v4.33.1 pinned; `v3/LANGUAGE.md` the
  V3 surface). Phases 0–7 in FOUNDATION §12.4; the coverage arc below is
  **parked** until phase 7.
- **The prune arc (2026-08, issue #24) — CLOSED 2026-08-22**: tree and
  branches, the issue tracker as the single backlog, ledger status
  banners, corpus cost instrumentation (the long pole is debt #37).
- **The coverage arc (issue #23) — PARKED 2026-09-05 at B-1b, resumes at FOUNDATION phase 7**: `tools/shardfmt` shipped
  as a proven native binary through the GENERIC path (spec → imp by the
  uniform-representation compiler → x86 as Theorem A's computed
  `ixf_prog` value; tools/impgen emits nothing for that leg), with the
  repo's own fmt gate running that binary; `examples/calc` is rung 1.
  **Its ledger, ratified 2026-08-22 — [docs/COVERAGE.md](docs/COVERAGE.md)**
  (pins, rung ladder C0–C6, the author-residue gate).
  [docs/IMP.md](docs/IMP.md) §6 is its charter and
  [docs/OVERVIEW.md](docs/OVERVIEW.md) §10 the ruling.
- **Closed, for orientation**: Arc A — the certificate architecture
  ([CERT.md](docs/CERT.md)); Arc B — the streaming bin
  ([STREAM.md](docs/STREAM.md): `(bin sha256sum)` at 1.08× coreutils-
  as-shipped with proven CPUID dispatch); the lock-step search arc
  ([SEARCH.md](docs/SEARCH.md), merged to main). Everything banked from
  them is an issue: labels `arc:coverage` / `parked` / `debt`.

## Conventions

- **Soundness is the foundation, not a feature.** A derivation of
  `true = false` would be as catastrophic here as it would be for
  Lean — every contract, ledger, and shipped binary rests on the
  kernel's word. Hence the tiny kernel, mandatory axiom kinds with a
  driver-enforced veto, and the negative-test corpus: every
  historical soundness bug (primitive-name shadowing, zero-case
  induction, parallel-let reversal) is not just fixed but pinned by a
  rejects test. A soundness suspicion outranks all other work.
- **One logical change set per commit, topic-prefixed** (`kernel: …`,
  `x86gen: …`, `docs/MEMORY.md: …`), stating what is proven/checked
  before and after. The kernel build-out's `slice N:` history is in
  `git log`.
- **Trusted-core touch is called out explicitly.** Changes to
  `kernel/*.shard`, `rust_bootstrap/src/*.rs`, or any `mod.req.shard`
  grow or shift the audited surface; impls, proofs, and examples do
  not. Under `v3/`, `v3/kernel/*.shard` is the audited surface.
- **Suite discipline.** Every change is gated by V3's suite on both
  engines, their byte-tie and the full `Init` replay (`v3/test.sh`,
  `v3/build.sh`, `v3/t0_full.sh`; CI's `v3`, `native`, `suite_tie`); new
  features land with pins (positive and negative) under `v3/pins/`.
- **Sources are written toward the canonical format** (`v3/CANON.md`;
  the formatter is `v3/tools/shardfmt`). No gate refuses a drifting
  file yet: the old tree's stamp check went with its kernel, and V3's
  gate is slice 5.1's third landing.
- **Design decisions live in ledgers.** Scope ledgers (docs above)
  for the arcs; REVISIT.md entries ("what was chosen / why now /
  revisit when") for everything smaller.

## Status

≈1.22M lines of tracked shard (kernel: ~35k; well over half of the
total is generated certificate text and validation articles — see
`v3/MANIFEST.md`) over a ~3,970-line Rust bootstrap. The proof corpus, the std
interfaces, the wasm / x86 / RISC-V models and lowering pipelines, the
trust-ledger machinery, the proof-search engine, and the streaming-bin
flagship are live and CI-gated; the prune arc closed 2026-08-22, the coverage arc is parked at B-1b,
and the current front is the V3 foundation arc (see Live arcs).
