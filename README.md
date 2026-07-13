# Shard
<p align="center">
  <img src="assets/shard.png" alt="shard" width="128" height="128">
</p>

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

**The tower.** A deliberately small **Rust bootstrap**
(`rust_bootstrap/`, ~3k lines, trusted by review, disposable)
evaluates *narrow* shard — a minimal subset. Everything else is shard:
the engine (`kernel/eval.shard`, an environment machine with TCO), the
reader, the elaborator, and the proof checker
(`kernel/check.shard`), which is just another shard program the engine
runs. For day-to-day speed the checker and engine are also compiled to
native binaries (`bin/shard_check`, `bin/shard_eval`) by a temporary
native chain — stamp-guarded against source drift, and **never the
soundness authority**: authoritative runs go through the Rust
interpreter path explicitly. When the certifying compiler below
replaces that chain, `rust_bootstrap/` is deleted and shard stands on
its own.

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

Build the bootstrap once, then check a file — checking means running
the self-hosted checker on it:

```sh
cargo build --release --manifest-path rust_bootstrap/Cargo.toml
rust_bootstrap/target/release/eval run kernel/check.shard examples/mem_reverse.shard
```

Expected output (tail):

```
51 passed, 0 failed, 55 axiom(s) admitted without proof
```

(The "axioms" are imported modules' opaque interfaces as seen from the
consumer side.) For the fast dev loop, compile the checker itself:

```sh
bin/rebuild.sh check                        # gen-N builds gen-N+1, byte-tie guarded
bin/shard_check examples/mem_reverse.shard  # same output, much faster
```

`./run_corpus.sh` runs the whole pinned corpus in parallel. **It is a
diff tool**: it exits 0 even with failing targets — gate changes by
diffing its FAIL set against the current baseline, never by exit code.
Sources are kept canonical with the proven formatter
(`bin/shard_eval run tools/shardfmt/shardfmt.shard FILE`); the
formatter's gate guarantees it cannot change what a file parses to.

## Repository layout

```
kernel/          ; the trust floor: term/reduce/proof/checker + the
                 ;   self-hosted reader, elaborator, driver, engine (ev),
                 ;   canon recognizer, and the reviewed facts.shard axioms
std/             ; the standard library — directory modules with reviewed
                 ;   mod.req.shard interfaces (arith, div, order, nat, list,
                 ;   map, bits, bytes, str, word, mem, rng, sha256)
meta/            ; the meta stdlib: shared machinery for programs-about-
                 ;   programs (plan, invoke, format, shape, proof, sketch …)
models/          ; machine + OS models as ordinary proven libraries:
                 ;   wasm, x86, linux (the syscall boundary)
tools/           ; untrusted toolchain: prove (auto-prover), shardfmt,
                 ;   canon, wasmgen/x86gen/wordgen (lowering generators),
                 ;   lowcheck/bytetie/lowbuild (the gates), bench,
                 ;   plus the temporary native chain (lower/codegen/low)
examples/        ; demonstrations and corpus pins — every feature lands
                 ;   with a pinned example; *_rejects.shard files are
                 ;   deliberate negative tests; lowbuild_*.sh are the
                 ;   pinned lowering builds
bin/             ; local fast engines (shard_check / shard_eval) built by
                 ;   bin/rebuild.sh; stamp files guard source drift
docs/            ; the documentation map below
rust_bootstrap/  ; the disposable Rust host (evaluate-only); deleted the
                 ;   day shard self-compiles
shard-viewer/    ; a graphical navigator for shard source (Rust; its own
                 ;   README)
```

## The documentation map

Ratified **scope ledgers** carry the arcs — each records its user
rulings, decision points, slice history, and gotchas, and is the
authority for its area:

| doc | area |
|---|---|
| [OVERVIEW.md](docs/OVERVIEW.md) | design intent — the why, in full |
| [LANGUAGE.md](docs/LANGUAGE.md) | the narrow object language (syntax, semantics) |
| [TOTALITY.md](docs/TOTALITY.md) | measures, admission, mutual recursion |
| [REFINEMENT.md](docs/REFINEMENT.md) | `(refine …)` — invariants as types |
| [BOUNDARIES.md](docs/BOUNDARIES.md) | modeling external systems; extern + axiom |
| [ISA.md](docs/ISA.md) | machine models as libraries; target architecture |
| [LOWERING.md](docs/LOWERING.md) | the lowering-form paradigm: statements, certs, gates |
| [IMP.md](docs/IMP.md) | the neutral imperative dialect: spec ⊑ imp ⊑ wasm/x86 |
| [X86.md](docs/X86.md) | the x86-64 target: emitter → bin ladder → World on silicon |
| [CANON.md](docs/CANON.md) | the canonical dialect: rules, census, content addressing |
| [TCB.md](docs/TCB.md) | the trust story: exactly what is trusted, and why |
| [MEMORY.md](docs/MEMORY.md) | representation + memory management (draft) |
| [BUILD.md](docs/BUILD.md) | the build layer: products, profiles, PIN/DERIVE/SYNTHESIZE |
| [SEARCH.md](docs/SEARCH.md) | program search over shard terms (arc complete, merged) |
| [REVISIT.md](docs/REVISIT.md) | the design-decision ledger: choice + revisit-when |
| [archive/M3-V1-VS-V2.md](docs/archive/M3-V1-VS-V2.md) | v1 vs v2 proof-effort comparison (history) |
| [archive/TRANSFER.md](docs/archive/TRANSFER.md) | the founding premise and v1→v2 mandate |

## Live arcs

- **The compile story** — how anyone actually compiles shard: one
  declarative surface over a pin-or-derive tower, from "one decl to
  ELF, zero configuration" to "every stage hand-spelled, same gates."
  The next ledger to be drafted.
- **Memory and representation** — [docs/MEMORY.md](docs/MEMORY.md):
  the tower of cancellation theorems; flagships are sha256→x86 with
  zero heap and the evaluator's aggregate half.
- **The flagship lowering** — compile `kernel/eval.shard` itself and
  retire the temporary native chain (LOWERING.md §7).
- **Program search** — [docs/SEARCH.md](docs/SEARCH.md): sketches,
  dialect grammars, and proof-rendered synthesis, developing in a
  parallel worktree.
- **The canon flywheel** — census → rule → re-measure
  ([docs/CANON.md](docs/CANON.md) §13 onward).

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
  not.
- **Corpus discipline.** Every change is gated by the FAIL-set diff of
  `./run_corpus.sh` against the pinned baseline; new features land
  with corpus pins (positive and negative).
- **Sources are canonical-format**, enforced by shardfmt's gate and
  the rebuild stamp check.
- **Design decisions live in ledgers.** Scope ledgers (docs above)
  for the arcs; REVISIT.md entries ("what was chosen / why now /
  revisit when") for everything smaller.

## Status

~141k lines of tracked shard (kernel: ~26k) over a ~3k-line disposable
Rust bootstrap. The proof corpus, the std interfaces, the wasm and x86
lowering pipelines, and the trust ledger machinery are live and gated;
the current work fronts are the compile story, memory/representation,
and the eval.shard flagship (see Live arcs).
