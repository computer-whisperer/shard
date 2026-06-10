# The temporary compiler chain (lower + codegen)

Purpose: escape the interpretation tax (direct engine ~minutes per gate run,
tower ~100x that) while the core language is still moving. This chain is a
DEV ACCELERATOR, not an authority: anything soundness-relevant (sidecar
replay, ledger, corpus verdicts) remains the interpreted kernel's word. A
compiler bug may cost a wrong dev signal, never a wrong proof.

## Architecture

Two written-in-shard apps, sharing the kernel front end as a library
(resolve_closure + build_module_d -> Module of native Expr). Parsing,
scoping, and binding are therefore ALWAYS the kernel's — the compiler never
re-reads source text.

1. `tools/lower/lower.shard` — shard -> RS-shard (a restricted subset of
   shard, still legal shard). Reads the app's closure through the kernel
   front end, rewrites every fn body to RS form, emits ONE self-contained
   lowered file (type decls + extern decls + lowered fns).
2. `tools/codegen/codegen.shard` — RS-shard -> C. Mechanical transcription;
   all semantic work already happened in lower.

Because RS-shard is legal shard, each stage gates independently against the
engines we already trust:

- LOWERING GATE: run original vs lowered app on the direct engine over the
  corpus/app targets — byte-identical stdout + exit codes required.
- CODEGEN GATE: run the lowered app on the direct engine vs the compiled
  binary — byte-identical again.

## The anti-split-brain contract

- The front end is the kernel's, by library import. No second parser, no
  second scope discipline.
- REFUSE, DON'T GUESS (the shardfmt contract): any form, prim, or pattern
  the lowerer does not positively recognize is a loud per-fn refusal and a
  nonzero exit — never an approximation. New kernel features land first;
  the compiler catches up when a target actually needs them.
- The compiled chain is never the soundness authority; the tower remains
  the dogfood/self-hosting proof, run occasionally.

## RS-shard (the restricted subset)

Still parsed by the kernel reader; the grammar below is what lowered BODIES
are guaranteed to look like (ANF):

  atom  ::= VAR | INTLIT | 'SYM
  rhs   ::= atom
          | (CTOR atom…)            ; allocation
          | (fn-or-prim atom…)      ; saturated call
  expr  ::= rhs
          | (let X rhs expr)        ; straight-line binding
          | (if atom expr expr)
          | (match VAR (ARM expr)…) ; FLAT arms only:
                                    ;   (C x1 … xn) | INTLIT | 'SYM | x
  tail calls are syntactically evident (a Call in expr tail position).

Nested patterns are compiled away by the lowerer (standard column-wise
match expansion into nested flat matches). Temp names are deterministic
(t__N per fn), so lowered output is byte-reproducible.

Top-level RS file = the closure's `type` and `extern` decls verbatim + the
lowered `fn`s. Check-only forms (claim/use/sig/requirement/…) are dropped —
the lowered artifact is a RUN artifact.

## Codegen v0 representation (chosen for days-not-weeks)

- C backend (cc does register allocation and calling conventions; the
  backend is swappable later without touching lower).
- Uniform value: one machine word. Low-bit-tagged 63-bit ints; pointers to
  heap cells [tag | arity | fields…] for ctor values; symbols = pointers
  into a static interned table emitted at compile time.
- Bump allocator, NEVER frees. These are batch processes that exit; the
  live tower process peaks at ~6 MB, the box has 125 GB. A GC is pure
  schedule risk.
- INT POLICY: i63 with trap-on-overflow (loud abort naming the operation).
  The interpreted engines are BigInt-exact; a trap means "this input needs
  the slow engine", never a wrong answer. A bignum runtime is a later,
  optional slice.
- Word ops: widths <= 64 implemented on uint64_t; width-128 refused in v0.
- gen_fresh: global counter, same contract as the hosts.
- Externs (all 7: get_args/read_file/write/write_file/write_line/exit/
  read_key): thin libc shims; World is threaded as a unit value at runtime
  (effect order is already explicit in ANF).
- Tail calls: self-tail-recursion compiled to loops (covers the list-walk
  hot paths); other recursion uses the C stack with a large stack rlimit
  set in main. If a real target blows the stack, escalate to clang
  musttail with a uniformized calling convention.

## Known, accepted gaps (v0)

- BigInt-exact programs trap at i63 overflow -> rerun on direct engine.
- Word width 128 refused.
- No reclamation: peak memory = total allocation; acceptable for batch
  tools, revisit only if a target proves otherwise.

First targets, in order: check.shard closure (the gate/corpus driver),
eval.shard (native engine, self-hosting optics), prove.shard.
