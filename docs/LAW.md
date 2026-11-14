# The law

This file holds the rules the system and the work are held to. It is short on purpose: the owner
reads it and ratifies it. Every other document describes what exists; this one prescribes. A change
here is the owner's decision, recorded in §6 in their words.

Status: **draft for ratification.**

## 1. What shard is for, and the bar

shard is a system for writing programs whose behaviour is proven, down to the code that runs.

**The bar for the logic is parity with Lean.** The checker, K, implements the rules of Lean 4's
kernel exactly (the pinned version is v4.33.1). Parity is tested, not claimed: every declaration of
Lean's `Init` library, as Lean exports it, checks in K with Lean's verdict and Lean's axioms. A
departure from Lean's rules is a decision that names what it gives up: the comparison with Lean,
Lean's library, and what authors already know.

**The goal now is the trusted compile system**: a compiler from programs to machine code that comes
with a proof. Today's compiler is unproven and is to be replaced by it.

## 2. The layers

| layer | what it is | who checks it |
|---|---|---|
| L, the logic | Lean's type theory: terms, types, propositions, proofs | K |
| E, programs | the definitions that are programs: what a `fn` is and what runs | K as definitions; the loader's checks on the program |
| I, proof scripts | the steps written in `(by …)`, turned into proof terms | nothing: the proof term they produce is checked by K |
| S, the surface | the s-expression source text | nothing: what it elaborates to is checked by K |

## 3. Rules of the system

**The checker**

1. K implements Lean's kernel rules and nothing else. It holds no elaboration, no unification, no
   proof steps, no arithmetic procedure, no file access and no global state.
2. The standard axioms are Lean's three: `propext`, `Quot.sound`, `Classical.choice`. Any other
   assumption is named on every result that rests on it.
3. A checked environment is made only by K admitting declarations one at a time. K rebuilds
   whatever a caller hands it and believes none of the caller's annotations. No operation of K
   takes two environments.
4. A shortcut inside K (computing on number and string literals) is enabled only for a declaration
   that equals its pinned reference by structure, never by name.
5. K answers accepted, rejected with a reason, or exhausted. Exhausted is neither of the others,
   and "could not show these equal" is never read as "these differ".

**Programs**

6. One name, one object. A `fn` is one definition in the logic and one program, and the bridge
   between them is equations K checks.
7. A function with a meaning in the logic is total. Its recursion is structural, or justified by a
   measure with a proof for each recursive call that the measure decreases. An open proof of
   decrease is reported on everything that depends on it.
8. A running program holds no function values. Higher-order code is specialised before it runs,
   within a budget; past the budget is a refusal.
9. Proofs and types are erased. Nothing that exists only by a non-constructive choice becomes a
   run-time value.
10. Effects go through the World. Each World value is used at most once on every path. What a host
    call does is stated as axioms, and a binary's contract names the axioms it trusts.
11. The evaluator defines what "run" means. Every other way of running is compared with it.

**Elaboration and proofs**

12. Everything above K is untrusted: the reader, the elaborator, the code that turns functions into
    definitions, the proof steps, the automation. What they produce is accepted only as K checks it.
13. Elaboration fills in what its inputs determine and never chooses. What is left undetermined is
    refused, with a pointer to what to write.
14. A proof in source is a certificate: replaying it searches nothing. Search happens in a tool, its
    result is stored beside the source, and a build only replays it. A stored proof that is missing
    or out of date is reported pending, never searched for during a build.
15. `sorry` admits nothing. It is reported, and citing a result that rests on it is refused.
16. No tool edits a source file an author wrote, except the layout fix an author runs on it. A
    proof a tool finds goes to a file beside the source.
17. The surface is fixed: no user notation, macros or syntax extensions. One spelling per concept,
    and Lean's name wherever Lean has one.

**Modules and acceptance**

18. A consumer knows a module only through its view: signatures and requirements. No definition and
    no defining equation crosses a view, so a module may change inside without touching its
    consumers.
19. Identity is the qualified name together with the content. A hash is a fingerprint for reports
    and never an authority. Types never merge because they look or are spelled alike.
20. The assumption policy is checked at every acceptance. Each accepted result lists what it rests
    on: axioms, view parameters, open obligations.

**Engines and gates**

21. The Rust bootstrap evaluator is the authority. A compiled engine is confirmed against it by
    comparing the two byte for byte on every test. One check is too slow for the bootstrap, the full
    replay of Lean's library (76 minutes against 7): there the two are compared on the first part
    and the binary runs the rest alone.
22. A missing or stale binary fails loudly. It never falls back silently to another engine.
23. Every source file an author writes is kept in the canonical layout by a gate that refuses and
    does not rewrite. The layout serves agents reading and editing, not a human in an editor.

## 4. The trust base

What has to be believed, because nothing inside the system checks it. Tests are evidence, not proof.
The detail, file by file, is in `KERNEL.md`.

**For an accepted theorem to be true:**

- Lean 4's type theory, and the consistency of the axioms the result lists.
- K (`v3/kernel/k/`, about 5,800 lines, and the data types it is written over). It is reviewed
  source, not proven. The evidence is the replay of all of `Init` against Lean's own verdicts and a
  battery of hostile declarations that must be refused.
- The engine that runs K: the Rust bootstrap (about 3,500 lines and the Rust compiler). When K runs
  as a native binary, also the unproven compiler, its runtime and the C compiler; the binaries are
  compared with the bootstrap, not proven equal to it.
- That the imported library is Lean's genuine `Init`. The export is fetched and built, not stored in
  the repository; the replay compares it with Lean's verdicts and axioms.
- The loader's bookkeeping above K: which axioms a file permits, and the replacement of a discharged
  obligation by what its proof rests on.
- That the statement means what its author intended. K checks the term the elaborator built; a
  mistake in the reader or the elaborator yields a true theorem about something else.

**For a compiled binary to behave as its contract says**, all of the above for each requirement, and:

- the axioms about host calls that the contract trusts, and the host calls themselves;
- that the program derived from a function computes what the function's definition denotes
  (the erasure: reviewed, tested, not proven);
- the check that the World is used once;
- the compiler, its runtime, the C compiler, libc and the operating system.

The contract says nothing about memory or time.

**Not trusted**: the reader, the elaborator, the unifier, the proof steps, the automation and its
stored proofs, and the code that turns functions into definitions. A mistake there can refuse good
input, fail to prove, or prove a different statement than intended; it cannot get a declaration
admitted that the logic does not derive.

## 5. How the work is done

- **Soundness first.** A suspicion that something false can be accepted outranks all other work.
  Every such bug is fixed and pinned by a test that fails without the fix.
- **No design without a consumer.** A capability is designed when something in the tree needs it,
  not because a plan lists it next. An open item names who needs it, or says nobody does.
- **Delete first, rebuild after.** The old implementation goes before its replacement is built.
- **Explicit over automatic.** Where a later update could silently change what an automatic step
  finds, the choice is written down and only verified: measures are written in source, found proofs
  are stored and replayed.
- **Documents describe the present.** History is the commit log and the tags. No dated narratives,
  no records of gate runs in documents.
- **The owner decides** the law, the trust base, and choices no test would catch. Such a choice is
  put to the owner in plain language with its options and a recommendation. A ruling is recorded in
  §6 in the owner's words. One given as bare assent is a checkpoint and cheap to reopen; one given
  with reasons stands until the owner reopens it.
- **One line of work on `main`**, the full gate behind each change to the system, generated files
  never patched by hand.

## 6. The owner's rulings

1. *The bar.* "the existing kernel has some properties that reflect the early MVP nature" … "parity
   with Lean … in a few years we won't find our system fundamentally incapable of the kinds of
   reasoning LM agents may wish to exploit". (Rule 1 and §1.)
2. *A view's requirements are the boundary.* "the requirements holding the opaque boundary of the
   module is an important boundary to keep so that internal mutations that preserve requirements
   are safe without rippling to all consumers". (Rule 18.)
3. *No design without a consumer.* "I think the biggest risk here is instilling bad architectural
   details. I am only able to provide scattered attention here in the evenings, and have been
   probably leaning too much on the roadmap." (§5.)
4. *Delete first.* "As always, I am in favor of ripping out the old first and rebuilding the new
   once we know what we need". (§5.)
5. *The compiler.* "The compiler should be the same idea as the v2 one, with proven lowering
   replacing it later." (§1, and the trust base's last line.)
6. *Layout is for agents.* "The exact layout rules are up for mutation, since humans looking at it
   in an ide is not a critical measure anymore." (Rule 23.)
7. *The critical path.* "In my current expectation, the critical path in general is getting the
   trusted compile system working, and I am tempted to just wipe the slate clean of everything v2
   and just focus on that." (§1.)
8. *What is carried forward.* "All the milestones behind what we have have limited value now, since
   they are probably an order of magnitude greater than your full context window. v3 is a chance
   for us to do things over cleanly, and IMO the best information to carry forward is the basic
   shape we ended up with and what pros and cons remained at that state." (§5, "Documents describe
   the present".)

Design choices the owner approved by bare assent, and the alternatives they were chosen over, are in
`DECISIONS.md`.
