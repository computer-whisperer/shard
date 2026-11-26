# The law

This file holds the rules the system and the work are held to. It is short on purpose: the owner
reads it and ratifies it. Every other document describes what exists; this one prescribes. A change
here is the owner's decision, recorded in §6 in their words.

## 1. What shard is for, and the bar

shard is a system for writing programs whose behaviour is proven, down to the code that runs.

**The bar for the logic is parity with Lean.** The checker, K, implements the rules of Lean 4's
kernel exactly (the pinned version is v4.33.1). Parity is tested, not claimed: every declaration of
Lean's `Init` library, as Lean exports it, checks in K with Lean's verdict and Lean's axioms. A
departure from Lean's rules is a decision that names what it gives up: the comparison with Lean,
Lean's library, and what authors already know.

**The goal now is the trusted compile system**: a compiler from programs to machine code that comes
with a proof. Today's compiler is unproven and is to be replaced by it. The statement it is built
to is `COMPILE.md`.

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
24. What runs is written. A program is fixed by the source text and the types its binders state:
    elaboration fills in types and proofs, never an argument that exists at run time. A program
    can therefore be read without elaboration, and the reading that elaborates gives the same
    program.
25. A function that reads and whose types check has a program. Where it cannot be given a
    definition — its recursion is not justified, a function it calls has none, an operation it
    uses has no meaning in the logic — it is a program alone and carries the reason. A function
    that does not read, or whose types do not check, is refused.
26. No type is special in the language by its name. An engine may store a type in a form of its
    own; that is a shortcut: the type is an ordinary declaration with ordinary operations, and a
    program run with the shortcut gives what it gives run as declared.

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
    consumers. What a consumer proved against a view and what an implementation proved in its
    own check are joined by K and by nothing else: the link checks the consumer's declarations
    in an environment where each parameter is the implementation's constant.
19. Identity is the qualified name together with the content. A hash is a fingerprint for reports
    and never an authority. Types never merge because they look or are spelled alike.
20. The assumption policy is checked at every acceptance. Each accepted result lists what it rests
    on: axioms, view parameters, open obligations.

**Engines and gates**

21. The evaluator written in shard (`v3/kernel/ev.shard`, rule 11's) is the canonical authority
    on what a program computes. The Rust bootstrap evaluator and a binary from the C compile
    chain stand in for it in almost every case, and development relies on them for their speed.
    All three agree: a difference between any two is a defect (the known ones: `OPEN.md` §1),
    and the written evaluator's answer is the one that stands. It is itself a shard program, and
    runs on one of the other two. The gates compare the Rust evaluator and the binaries byte for
    byte on every test, and the written evaluator with both on the programs the tests run under
    it. Two checks are too slow for the bootstrap: the full replay of Lean's library (76 minutes
    against 7), and the checked load of the toolchain's own sources, where the bootstrap
    interprets the loader checking the sources it is written in. In each the two are compared on
    a part and the binary runs the rest alone.
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
- The engine that runs K: the Rust bootstrap (about 5,200 lines and the Rust compiler; 3,600
  of them are the reader, which reads K's sources as it reads any program's), and the table of
  facts about Init's constants it reads them by (`v3/kernel/init.facts`, 39 lines, printed by
  the loader from K and compared with K's by a test). When K runs
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
  (the erasure: reviewed, tested, not proven) — and in it, that an engine's operation computes
  the definition a row of the registry runs it for (`kernel/prog.shard`): Init's arithmetic on
  `Nat` and `Int`, its two shifts on `Int`, and the `and`, `or`, `xor` on `Int` that the
  system defines itself (`kernel/base.shard`). A wrong row makes a program compute what its
  definition does not; it proves nothing, K reading the definition alone;
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
- **One line of work on `main`.** Before a push the dev box runs the checks the change can
  move; the full gate runs on the pipeline behind every push, and what it finds is mended next
  (ruling 17). Generated files are never patched by hand.

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
9. *This file and `SHAPE.md`.* "I read through and can ratify that those match what we built so far.
   There are some details there that I think are worth reviewing and potentially reworking now".
   (Ratified as a description of what exists; the details stay open to rework.)
10. *One language.* "The main incentive I am following here is the desire to have the full v3
    shard source code be itself first-class shard. A shard application should be able to import K
    as a module and use it natively, with the whole system compiled in the same fashion. Given how
    small K itself is, the extra complexity of promoting the rust bootstrap to be capable of the
    same things seems worthwhile if it means K itself can be standard." (Rule 24 is the means: a
    program the Rust evaluator can read without the elaborator.)
11. *No precedence among names.* "collisions should be refused overall." (Rule 13: a citation
    that two visible names answer is refused, a file's own name against Init's included.)
12. *No type special by its name.* "I want to avoid types that gain special treatment in the
    language overall by their name" … "The kernel's builtin evaluator taking an optimization
    shortcut on Symbol that preserves all behavior an unprivileged version would seems fair to
    me." (Rule 26. `Symbol` is an ordinary declaration, `v3/kernel/sym.shard`, and no engine
    stores it in a form of its own: `DECISIONS.md`.)
13. *A program before its proofs.* "I lean towards keeping an unaccepted function as a program
    with a reason. I am thinking of what prototyping or rough sketch workflows may want here --
    where we may want to run or test shard programs before the full due diligence for proof
    assembly is paid for. Some minimum bar is reasonable, but it shouldn't take a whole research
    program to throw together some small mvp with almost no proof structures." And the bar:
    "Type errors should result in refused programs." (Rule 25.)

14. *A binary may build a binary.* "My main objective here is speeding up development. We
    still have lots of ways to run slow integrity checks in CI where things are interpreted to
    reduce complexities when evaluating trust, but for day to day change tests we can get away
    with a lot less." … "I am willing to completely remove the standing 'no binary builds
    another' guideline so long as we have an alternate trust path that can be run on CI
    occasionally, and we otherwise trust our compile and rust eval processes to be at least
    roughly correct." (The build: `DECISIONS.md`, Build and gates.)

15. *The canonical evaluator.* "I would say that the written-in-shard evaluator is the canonical
    authority here. Alternatively, the rust evaluator and the result of running the c compile
    chain should also be acceptable in almost all cases, and we are speeding up development by
    relying on them. All three paths should agree, but the written-in-shard kernel evaluator
    should be considered the canonical focus." (Rule 21, which named the Rust bootstrap
    evaluator the authority until this ruling.)

16. *Operators beyond Init.* Of the bit-level primitives the engines run and the logic has
    no form for: "The main point is that these bitwise operations will be things we want to
    use in normal shard code, beyond what types and functions Lean's Init happens to have. I
    don't mean that we should cheat and axiomize everything, but we will need to expand our
    operator and logical proof base over time beyond what Lean happened to implement for
    us." (An operator's meaning is a definition K checks and its facts are proven from it;
    what is believed is that an engine's operation computes that definition: §4. Built for
    the bit-level five: `DECISIONS.md`, Programs and the back end.)
17. *The dev box checks what a change can move; the pipeline checks all of it.* "let's try to
    speed things up by limiting local gate sweeps to areas relevant with the changes made. CI
    can run async and tell us when low-rate mistakes are made." (§5. What is run for which
    change: `DECISIONS.md`, Build and gates.)

18. *The compile system's statement.* Of `COMPILE.md`, the first four points of its §9 among
    them: "I
    read through it, and I agree with it." And of programs that do not run once to an end: "I
    do hope for us to use shard in open-ended applications that aren't one-shot (http server,
    desktop app, etc), but the beauty of this approach is that we aren't locking all consumers
    into one particular paradigm. You could verify that all the encode and decode operations
    in your server meet formal requirements, while the overall loop that cannot be proven
    refined as easily can be constructed with other means." What it is for, in §1 there: the
    owner's words. How a machine is represented (its §4) was left open in the same reply — "I
    have less confidence that we have the right approach" — and settled on the next: the
    processor's own state and bytes as the floor, notations above it, K reasoning about a
    model in general and engines running it. "That matches my thinking, rewrite §4 and start
    there".
19. *Low-level code: as C or Rust, and its machine code's facts provable.* Of the form low-level
    code is to have (`COMPILE.md` §4), against a proposal that gave it its meaning through the
    evaluator: "One root requirement here is that programs written in shard should reach the
    specificity and performance of those written in c or rust. A permanent runtime or evaluator
    is out of the question to begin with, and facts like 'this chunk of x86 machine code, under
    these circumstances, does not write memory outside of these addresses' should ideally be
    statable and provable." The form then put to the owner — a buffer a value, evidence at each
    access, the contract of a piece of code — went to a trial first: "I think we were on the
    right page, but let's get another thought cycle to review the problem and any predicted
    consequences of the approach. If everything looks good, then we can proceed with the test
    you proposed." On the trial's report — two functions' contracts proven, that of a third,
    which calls them, not: "I would call the frame ruled".
20. *A function at a module's seam.* On what stands between a program and its machine code:
    "IMP was intended to be the imperative intermediate point that let us compile to something
    a bit higher level than x86 in v2, and I still expect something to take up that role in v3.
    Past that, I would be optimizing for a system of components that can be used in different
    scenarios. We do want a completely automated pipeline for basic projects, but we also need
    clean ways to escalate overrides and custom lowering approaches for a given project." …
    "we need to avoid locking ourselves into a fixed gcc-like paradigm." … "The challenge will
    be making the spectrum from custom to auto functional, and particularly with composing
    different levels". On the unit: "I think that modules being their own compilation units
    with almost no inlining across modules could help this." … "Between modules we can have
    the same interface regardless of what degree of automation they use for their systems."
    Asking for a review of the form before any of it was tried: "I think this may be one of the
    most important paradigms to set in shard, so it is worth getting it right." On the report
    of its trial, of the form in `COMPILE.md` §5 — three levels of shard for a function a
    module exports, two facts, one statement at the seam whatever produced the code, the
    intermediate inside the second fact: "Good -- this looks good to be the paradigm."
21. *What a tool writes.* On a compile step that writes lemmas: "I think we should answer that
    carefully. I do want metaprogramming to be a first-class entity in shard, but don't have a
    clear vision for how to enable that yet. In any case, the compilation process emitting some
    amount of shard (in-memory IR or AST only, or as a file -- up in the air which) is an
    expected outcome. We don't have a clear story for how the build process works overall, but
    as before I would lean towards a pile of flexible tools rather than a fixed process. What
    we have so far are sidecar files -- intended to take up autogenerated shard rather than
    requiring the kernel or build process to search for proofs." Of the rule then put to them
    — a source file states everything that is claimed, and nothing an author writes names
    anything in a file a tool wrote beside it — as the intent of those files: "yes, this
    matches my intent." (Rule 16 has the file beside the source; the rule is stated in
    `COMPILE.md` §5 and is in no rule's text here. What else a tool may write there, and
    whether it is kept or made again by each build, is not ruled.)
22. *The answer at a seam.* On what a caller's proof is given of a function outside its
    module, first what it must not become: "my main thought is how this will scale with
    program complexity. The hope has been for modules to establish their own proofs on their
    own, relying on published facts of modules they call, rather than having to rederive
    detailed consequences of the exact implementations used downstream. I see that some of
    this is unavoidable (the execution of the callee is part of the execution of the caller),
    but we need to do whatever we can to stop this from propogating (do we then need to
    investigate the callee's callee?) and limit the complexity of cross-module reasoning."
    Then, asking "What exactly am I ratifying?", of the sentence put to them — when a compiled
    function calls a function outside its module, the caller's proof is given this about the
    call: it returns the value of the callee's low form, and leaves memory as the low form
    says, whenever the arguments meet what the low form demands; whether they meet it is a
    question in the logic, not a check any program runs —: "Ratify it". (Stated in
    `COMPILE.md` §5. Not settled by it: how it extends to a low form whose demands name values
    no code holds.)
23. *A function as a module, and the link.* On the report of a trial of the three functions
    behind views — a caller's proof rests on the view of what it calls and on nothing below,
    and the views fell short in two places: a signature could carry no evidence, and nothing
    in K joined a consumer's theorem to the implementations' discharges —: "I would lean
    towards extending the loader in both cases now. The main worktree has not made much
    progress there yet. Take the time to design the changes properly based on what is needed."
    On the design then put to them — a signature's proof parameters read as a function's; a
    discharge checked by K to be of the view's own type, a defect found on the way (an
    implementation recorded as proving a law of which it had proved another reading); and
    after the roots K's check, once more, of what rests on a parameter, in an environment where
    each parameter is its implementation —: "Proceed with building it as designed, including
    the link." (Rule 18; `LANGUAGE.md`, "Views"; `v3/compile/README.md`, "A function as a
    module". That the link is asked for by a flag is the builder's, for the cost to the
    toolchain's own loads.)
24. *Where memory comes from, and a program's entry.* On four questions, each put with a
    lean — where a function gets memory that no argument of it names; what the compiler does
    where the licence to write in place does not close; what happens when memory runs out; and
    the order of the work on a program's entry —: "1: agreed with 1, 2: 1, 3: that sounds fine,
    4: your lean sounds reasonable." The leans as put: (1) such a function is given the memory
    by its caller — a region is an argument like any buffer and the interface states the need;
    the entry receives the regions from the start-up; a heap, later, is a library over a
    region; (2) the compiler copies, and runs the in-place code on the copy — a separate
    destination for a writer is a later optimisation of one function; (3) the need is a formula
    in the arguments' sizes and a proof obligation, with no check inside a function: one
    comparison at the entry when the sizes are known, and a reserved exit code; (4) the
    start-up as data, then command-line arguments, then output and files, the limit of six
    words a function lifted beside the first. (Each a short assent to a stated lean. Built of
    it in the tree: the start-up as data, `DECISIONS.md`, "Programs and the back end".)
25. *The model does not promise where a process's stack is.* On a finding — a start-up that
    passed its buffers' addresses as constants, computed from the one stack pointer the
    model's start fixed, had its theorem accepted by K and its file run by the model, and the
    processor killed the file — the question was put: "should the model stop promising where
    the stack pointer is?", with three options: yes, as part of the command-line-arguments
    stage; "Yes, now, before the six words."; no, relying on the processor's runs. The owner,
    answering this question and one other in one message: "Agreed with 2 for both; proceed
    with the six words" — "2" being here the option "Yes, now, before the six words." (A bare
    assent to a stated option: a checkpoint. Built of it in the tree: `DECISIONS.md`,
    "Programs and the back end", the stack's place not promised.)
26. *A function's locals past the sixth live in its frame; the first six stay in registers.*
    The question was put with a sizing: "where do locals beyond six live?", with two options —
    every local in a stack slot (the smaller proof; every existing function's code changes,
    each read or write of a local a memory access); "The first six stay in registers, the rest
    go to stack slots" (the stated lean: the bar is no regrets against C, and the first would
    later be undone). The owner, in the message of ruling 25: "Agreed with 2 for both; proceed
    with the six words" — "2" being here the second option. (A bare assent to a stated lean: a
    checkpoint. What was asked and built is the limit on a function's LOCALS; a function still
    takes at most six argument words, which was not put to the owner. Built of it in the tree:
    `DECISIONS.md`, "Programs and the back end", a function's locals.)

The text of rules 25 and 26, written from rulings 12 and 13: "LAW 25 and 26 look good to ratify."

Rule 21's text is written from ruling 15, which the owner gave as "One minor adjustment to Rule
21" on reading the rule; its second slow check is written from their go-ahead for the ordinary
`Symbol`, whose first step, as put to them, moved the toolchain's check onto the binaries: "Good.
Proceed with the symbol change (full 1-4 scope)". The text as adjusted: "That matches my intent,
and we can consider it ratified."

Design choices the owner approved by bare assent, and the alternatives they were chosen over, are in
`DECISIONS.md`.
