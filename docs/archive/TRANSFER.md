# Transfer Notes → Next Iteration

## The core premise (read this first)

Software is written twice: once as *what it must do* (requirements/spec) and
again as *how it does it* (code, then machine code). Today the link between those
is trust and testing — nothing *proves* the implementation satisfies the spec, or
that the compiler preserved it. This project's bet is to make that link a single,
transitive **refinement** relation `spec ⊑ … ⊑ code`, where every step is a
separately *proven* artifact. Requirements→design, design→code, and
code→machine-code then become the **same operation at different altitudes**.
Verified compilers (CompCert, CakeML) already do the bottom half this way; the
goal is to bring it to **general-purpose software, top to bottom** — a pipeline
whose output is code *proven*, not tested, to meet the original requirements.

What makes this timely is an economic inversion. Code is now cheap to produce;
*coherent, proven requirements* are the scarce resource. An LLM is good at
*proposing* an implementation or a proof — but a proposal is untrustworthy on its
own, and a small checker is exactly what converts untrusted output into a
guarantee. **That generate-and-check asymmetry is the whole product:** low-trust
automation (LLMs, and SMT solvers as certificate-emitters) *proposes* each
refinement; a tiny trusted kernel *proves* the requirement is met at every link.
The refinement-derivation systems of the 80s–2000s (Specware/Kestrel) stalled
because a *human* had to write every refinement by hand — LLMs are what invert
that economics, and the reason to build this now.

Everything below is the engineering residue of a pilot that validated this shape
at small scale (an in-place array reverse, then hand-written wasm, both proven
equal to a functional spec). Your job is to rebuild the substrate so it can carry
the premise to *real* software.

---

*Final report from the throwaway pilot (M0–M4). The next iteration rebuilds from
scratch on updated premises. **This doc is the handoff — read it instead of the
old code.***

**Why "instead of":** past handoffs of this shape failed because the next build
reflexively copied chunks of the old one. Don't. The old code encodes premises
v2 explicitly discards — monomorphic types, structural-recursion-only, a
hand-built arithmetic toolkit, fuel as an existential-substitute. Copying it
imports dead assumptions. Keep the *conclusions* below; re-derive the *code*.

## What we built — the refinement trace (why this is viable)

The same operation — **list reverse** — realized at four descending levels of
abstraction, each step a machine-checked sameness proof, bottoming out in
executable bytecode:

1. **Spec — naive reverse.** `rev` over lists, the obvious O(n²) definition
   (`rev(x:xs) = rev(xs) ++ [x]`). This is the requirement.
2. **Optimized — linear reverse.** `fast`, an accumulator-passing O(n) list
   reverse. Proven `fast = rev` for *all* lists. *(Algorithmic refinement: same
   data type, better cost.)*
3. **Linear memory — in-place reverse.** A loop over an address-indexed mutable
   memory (`Mem`) that swaps ends inward. Proven that reading the memory back out
   after the loop equals `rev` of the input, for **universal n**. *(Data
   refinement: list ↔ linear memory; needed the array-framing + `mirror`
   machinery.)*
4. **Machine code — wasm.** A hand-written wasm reverse, executed by a
   structured-wasm interpreter written in the object language. Proven
   `wasm ⊑ rev_loop ⊑ rev` — a transitively composed chain — for arbitrary memory
   and length.

The punchline: this is `spec ⊑ … ⊑ code` carried from a one-line functional
requirement all the way down to running bytecode, every link **proven rather than
tested**. Toy scale — but the whole thesis, executed end to end.

## Writing requirements "backwards" (declarative specs)

The trace above used a *functional* spec (`impl = rev`) — the easy sub-case,
where the "requirement" is itself an algorithm. The real ambition is requirements
written **backwards**: an acceptance predicate that *checks* an output rather than
computing it — `∀x, accept(x, impl(x)) = true`, e.g. `sorted(out) ∧ perm(xs, out)`
for a sort. The framework supports this natively (it *is* the `t : Inputs → Bool`
shape), with three things to internalize:

- **No new kernel feature, and the declarative obligation sits at exactly one
  link — the top.** Prove `accept(x, impl₁(x)) = true` once for a first clear
  implementation; every lower refinement is then ordinary functional equality
  `impl₂ = impl₁`, and transitivity carries acceptance to the bottom. The lowering
  tower is unchanged — a declarative top just adds one harder link *above* it.
- **What's harder at that link:** the proof flips from "match two computations" to
  "maintain an invariant that *implies* the predicate" (Hoare-style; M3's loop
  invariant is the same shape, so it's reachable, not new). And **spec adequacy
  stops being free** — `impl = rev` pins the exact output, but `sorted(out)` alone
  is satisfied by `[]`; you must conjoin `perm`, and the completeness of that
  conjunction is a human/LLM responsibility. The weak-spec dragon, made concrete.
- **This is the use case that makes the v2 mandate non-optional:** `perm` needs
  multiset/`count` reasoning (→ finite maps/collections); `sorted` needs order
  lemmas (→ the SMT layer). Declarative specs are what turn those from
  nice-to-have into load-bearing.

The alignment worth keeping in mind: a no-existentials logic expresses **exactly
the decidable predicates** — specs you can *check* on a candidate output. That is
the definition of an acceptance criterion, and "checkable but hard to produce" is
the generate-and-check asymmetry the whole product rests on. The constraint and
the vision are the same shape.

## What the pilot proved (don't re-litigate — keep as principles, not source)

The architecture works. Validated:

- **Two languages, not one.** Object language = total/pure/first-order programs
  *as data*; a separate trusted checker reasons about them. ACL2/LCF lineage,
  deliberately **not** Curry-Howard / dependent types.
- **Kernel/search split is the product.** Tiny trusted checker; proof *search* is
  untrusted and swappable — the LLM, and later an SMT solver, live there.
- **One shared reduction engine.** The checker's reduction must *be* the
  interpreter's, not a re-implementation. This is the sharpest soundness risk; a
  single source of truth is non-negotiable.
- **Logic as computable Bool.** and/or/not are ordinary object functions. The
  meta-logic needs only: equality, ∀-equations, and conditional premises
  (`premises ⊢ lhs = rhs`).
- **Refinement (`⊑`) is the spine.** `spec ⊑ … ⊑ code`, transitive. Demonstrated
  `wasm ⊑ loop ⊑ rev` end to end.
- **A small proof-step kit sufficed through M4:** guarded reduce (`simp`) as the
  workhorse, `unfold`/`reduce` as escape hatches, `induct`, `case_on`,
  `rewrite`/`rewrite_with` (conditional), `absurd`. **M4 added zero new proof
  primitives.** The equational core + conditional rewriting + ∀-instantiation is
  more complete than expected. Start there; spend v2's budget on the *language*
  and *automation*, not on new proof rules.

## The one meta-lesson

**Object-language ergonomics and kernel complexity are the same problem.** Every
feature that makes the language pleasant to program in — polymorphism,
higher-order, maps, non-structural recursion — lands as a proof obligation or new
matching machinery in the trusted core. You cannot make the language nice "in the
frontend." Budget for the kernel cost up front.

## Change these premises (the v2 mandate)

The pilot's simplifications are now walls. Minimal expansion to build
*significant* software:

1. **Parametric polymorphism — in the LOGIC, not just monomorphized.** The point
   is proof *reuse*: prove `append_nil` once over `List<T>`. Monomorphize-then-prove
   throws that away and re-explodes the lemma library per element type.
2. **First-class finite maps / collections**, with their lemma library. Real
   programs are indexed collections of things. *Bonus:* the theory of
   arrays/maps is **decidable** — the data structure you need and the automation
   you want (below) are one investment. (The pilot's single `Mem` array is the
   degenerate seed.)
3. **Defunctionalized higher-order** — pass function *names*, not closures.
   Recovers map/fold/filter without reintroducing binders/capture (the thing
   first-order-only bought us). ACL2's `apply$` is the model — and note it was
   hard and late even there.
4. **`let` / sub-term sharing** — terms get huge; this is also a *performance* fix
   (see the simp gotcha).
5. **Measure / well-founded recursion** — real algorithms (divide-and-conquer,
   graph traversal over "remaining work") are not structurally recursive on a
   subterm. This is the one relaxation that costs the *syntactic-totality-for-free*
   property: termination becomes a discharged proof obligation. Decide deliberately.
6. **Mutual recursion + mutual induction** — needed for any mutually-inductive AST
   (expr/stmt, block/instr). The pilot's VM was contorted into a flat instruction
   list + explicit control stacks *specifically* to dodge this. Don't inherit that
   contortion.

**Type system:** Rust-inspired *surface* (nominal ADTs, generics, traits) with
ML-style **decidable** inference. But traits must **carry laws** (proof
obligations) — that's the one place Rust is insufficient; you want Coq/Lean
typeclass / Isabelle locale semantics, not Rust's lawless traits. **Drop**
ownership/lifetimes (purity removes the aliasing problem they solve), dynamic
dispatch, and dependent types.

**Automation — design for SMT-as-certificate from day one.** A large fraction of
pilot hand-proofs (arithmetic, order, congruence, the memory/array framing) are
**decidable theories**. Put the SMT solver in the *untrusted proposer box* and
have it emit certificates a *small per-theory kernel checker* validates. This
keeps the small-TCB thesis intact — do **not** trust Z3's bare "unsat." Structure
goals so the decidable fragments are syntactically separable from the inductive
part: **induction stays in the kernel; decidable leaves go to the solver.** Don't
hand-build an arithmetic toolkit again.

**Concurrency — model as data, don't enact.** The bar (below) is proving a
parallel compute-graph schedule matches its sequential spec. Do NOT add
threads/effects to the language. Represent the schedule (jobs + barriers) as a
*value* and execution as a *pure fold*. Single-assignment dataflow +
barrier-ordered consumers = determinism by construction ⇒ equational refinement,
**not** separation logic. "Independent jobs commute" = "they write disjoint
state" = the M3 framing lemmas, reused.

## Gotchas found the hard way

- **`simp` blew up exponentially.** The reducer simplified a call's arguments,
  then re-simplified them inside the substituted body, so reducing `step^n` over a
  stuck guard was 2ⁿ. Fixed by memoizing (simp is a pure function of the term —
  memo by structural identity is sound and a pure speedup). **Lesson:** any
  reducer that re-traverses substituted subterms needs sharing/memoization *from
  the start*. Also an argument for `let` in the term representation.
- **No existentials → fuel as a closed-form exact count.** "Halts in *some* fuel"
  is unsayable, so we computed the exact step count. It works but couples proofs
  to exact counts (brittle), and it is a *symptom* of structural-recursion-only —
  with well-founded recursion you may not need fuel at all. Decide before
  inheriting it.
- **Representation alignment is death by a thousand lemmas.** `add(i,1)` vs `S i`,
  `le` vs `lt` guards — a pile of tiny order lemmas, all hand-proved. This is
  *exactly* the brittle work an SMT/LIA decision procedure erases. It is the
  strongest concrete argument for the automation layer.
- **Brute-force search hit a wall.** It found only unconditional leaves, couldn't
  do conditional rewriting, and degraded as the theory grew (we had to cache the
  searched theory). Don't lean on it; the LLM-proposer + SMT layer is the real
  automation story.

## Proof points worth remembering (don't re-derive)

- **In-place array reverse = functional `rev`** (M3): a per-position loop
  invariant, induction over the counter, a `mirror(i,j,p)` function relating the
  two swap positions — all resting on the array framing `read(write(m,a,v),b)`
  cased on `a=b`.
- **VM execution = functional spec** (M4): per-iteration step lemmas
  (continue/exit, gated on the loop guard) → induction over the loop counter →
  compose. Fuel = exact closed-form count.
- **Where the data-refinement dragon actually lives:** representation *change*
  (tensors→atoms, atoms→buffers). Schedule/partition refinement is the *easy*
  corner — identity coupling (same value at the same id), provable equationally.
  Don't conflate the two.

## North star for v2

Concrete bar (from the owner's whisper-tensor system): express a compute-graph →
parallel-partition (phases + barriers + per-lane spans) **in the object language
as data**, then prove **schedule refinement** — the partitioned plan computes the
same values as the sequential graph. That system already states its own
correctness as "same value at matching atom IDs across the partition" and tests
it with an A/B reference harness; v2's job is to *prove* what that harness checks.
Confirm whether the atom space is write-once (SSA): if so, the proof collapses
from M3-style overwrite reasoning to pure schedule-independence.
