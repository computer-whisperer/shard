# The shape of shard

What the system is today, what is solid in it, and what is weak or missing. Read this first; the
rules are in `LAW.md`, the reference in `LANGUAGE.md` and `KERNEL.md`.

## 1. In one page

shard is a language for programs and for proofs about them, and the toolchain that checks both.
A source file declares types, functions and theorems. Every declaration is turned into a term of
Lean 4's type theory and checked by a small checker, K. A function is at once a definition in that
logic, which theorems can talk about, and a program, which runs.

```
 source text (.shard, s-expressions)
    │  reader, elaborator, proof steps                     untrusted
    ▼
 declarations ─────────► K, the checker ─────────► accepted, with what it rests on
    │                       ▲
    │                       └─ Lean's Init library, read from Lean's own export
    │  erasure: drop proofs and types                      trusted, for binaries
    ▼
 programs ───► the evaluator                               defines what "run" means
          └──► lowering → C → the C compiler ──► native binaries       unproven, compared with the evaluator
```

Three things carry the design:

- **The logic is Lean's, exactly.** K re-implements Lean 4's kernel and is tested against all of
  Lean's base library. shard gets Lean's library and Lean's standing as an oracle, and adds no
  axioms of its own.
- **Only K is trusted to judge.** Everything that builds terms — the reader, the proof steps, the
  automation — may be wrong; K refuses what does not check.
- **Programs are definitions.** There is no second language for specifications. A function's
  meaning in the logic and the program that runs are derived from one source text.

The toolchain is written in shard and runs two ways: on a small Rust evaluator (the bootstrap) and
as native binaries it compiles from itself. What a program computes is defined by a third, the
evaluator the toolchain contains (`ev.shard`); the two engines stand in for it and must agree
with it and with each other.

## 2. The parts

**K, the checker** (`v3/kernel/k/`, about 5,800 lines). It takes an environment and one declaration
and answers accepted, rejected with a reason, or exhausted (a budget ran out; neither a yes nor a
no). It sits behind one interface of 64 operations, so no other code can build an environment
except by having K check each declaration. Evidence that it is right: it accepts all 57,977
declarations of Lean's `Init` export with Lean's own axiom lists, and passes a battery of 76 cases
built to break it. It is not proven.

**Init, Lean's library as the base.** `v3/export.sh` builds Lean v4.33.1 and exports its `Init`
library as text. Every source file sees all of it, and a load gives K
only the part of Init the file cites, each declaration checked again. `v3/std/` holds only
what Init lacks, and the programs Init's own functions run as.

**The loader** (`v3/kernel/loader.shard`, driven by `load.shard`). It reads a root file and what it
imports, one module per file, and prints one record per declaration: `ACCEPT` for a checked
declaration, `DEFINE` for a function that became a definition, `RUNNABLE` for one that is only a
program, `REFUSE`, `PENDING`, and so on. The records are the system's output; the tests compare
them.

**From a function to a definition.** `(fn NAME …)` yields a definition, its equations as theorems,
and a program. Recursion on a smaller piece of an argument is accepted as written. Any other
recursion states a measure, a number that must get smaller, and owes one proof per recursive call
(`NAME.dec_N`). Functions that call each other become one definition the same way. A function that
cannot be given a meaning stays a program, with the reason on its record.

**Proofs.** A proof is `(by STEP…)` with sixteen steps (introduce, apply, rewrite, case-split,
induction, arithmetic, …). Replaying a proof searches nothing. `auto` replays a proof that a
separate tool found and stored in a file beside the source. Arithmetic is linear arithmetic with a
checked certificate.

**Views.** A directory with an interface file (`mod.req.shard`) is a module whose consumers see
only signatures and stated requirements. The implementation is checked against the interface
separately. K itself sits behind a view; so do the two halves of the formatter.

**Effects and contracts.** A program touches the outside through a `World` value it may use at
most once, and through host calls (`extern`) that are opaque to the logic. A `bin` declaration
names a program's entry point, its host calls, the axioms about those calls it trusts, and the
theorems that make up its contract. The load fails if a contract theorem is missing.

**Running.** The evaluator runs a program inside the loader (`--run`). `v3/build.sh` compiles one to
a native binary through C. The compiler is unproven: its output is compared with the evaluator's on
every test, and nothing more. Numbers are big integers (a native binary stops at 2^3840); bytes
and strings are lists.

**Two kinds of source.** The toolchain's own files (under `v3/kernel/`) are ordinary files on
Init's types, and the checker reads them: of the 8,996 functions the loader is made of, 8,988 are
definitions in the logic — 4,792 of them the rows of one generated table, 4,196 written, all of
K's among them —, and the other 8
are type-checked and run as programs: the run of a program with effects, which calls itself
with no measure, and the seven functions above it. What is proven about a toolchain function is that its recursion ends, for 747 of
them (1,866 proofs
beside them, over the facts of `kernel/descent.shard`, a term's derived size and a list's length), and of the JSON parser that a parsed value leaves less input than it was given and that its answer does not turn on the fuel it is defined by (`kernel/json.shard`); nothing else is stated. One check loads the
toolchain's sources this way and checks those proofs (`toolchain_check.sh`); a build and every
test read them as programs alone and pass the proofs over. The files elsewhere carry the theorems. This split is the largest irregularity in the
system.

**Build and gates.** 59 test scripts run on both engines and their outputs are compared byte for
byte. With the full replay of Init and the layout check, the whole gate takes about 75 minutes
in CI on an idle runner, its jobs side by side: the time of the longest, `native`, of which the
tie of the two engines' outputs is 42. It is not run whole on the development machine.

## 3. What is in the tree

| where | size | what |
|---|---|---|
| `v3/kernel/` | 51,000 lines | the toolchain: K (`k/`), reader, elaborator, proof steps, loader, evaluator, compiler |
| `v3/kernel/test/` | 66 files | the test scripts and drivers |
| `v3/pins/` | 488 files | small sources, each stating the outcome it expects; 328 cases for the loader |
| `v3/std/` | 1,000 lines | the library beyond Init: lists, bytes, a map, a byte memory, facts on division and bits, host calls |
| `v3/models/x86/` | 6,500 lines | a model of the x86-64 processor under Linux, begun: a program is bytes, decoded and executed a step at a time; an assembler checked by the model's decoder; the model held to the processor on programs assembled by hand; laws of the model, one rule for each instruction among them, and of a function's stack frame; one demonstration from shard functions to a file that runs, and a trial of low-level code as shard: functions over buffers of machine words, a listing compiled by hand, for each of them the contract of its code, and by those the theorem that the listing's file exits with what the functions compute (§6) |
| `v3/compile/` | 10,300 lines | the compile system's first pieces (§6): an intermediate machine, the statement that crosses a module's seam, a pass from intermediate code to x86-64 instructions with its theorem, a link theorem, and one function proven from its definition to the exit status of its file |
| `v3/examples/` | 2,200 lines | `calc`, a calculator with 100 proven claims and a proven contract for its binary; small worked examples of each feature |
| `v3/tools/shardfmt/` | 1,300 lines | the source formatter, with a proven contract and a gate over every source file |
| `rust_bootstrap/` | 5,900 lines of Rust | the bootstrap evaluator: its reader of shard sources, the toolchain's and any program's (3,600), the engine (1,600), its tests (700) |
| `tools/zed-shard/` | | an editor grammar |

Two programs are carried all the way: `calc` and `shardfmt` each have a native binary and a
contract proven about their entry point. In both, the contract rests on nine stated axioms about
host calls.

## 4. Where it stands

### Solid

- **Parity with Lean is measured.** All of `Init` checks, with Lean's axioms, on every gate run.
- **Three axioms.** Everything else a result rests on is listed on its record.
- **The untrusted layers really are untrusted.** K rebuilds every term it is handed; the proof steps
  and the automation cannot make a false theorem accepted. Each soundness hole found so far has a
  test that fails without its fix.
- **Proofs do not rot silently.** A stored proof is keyed by the statement and everything it cites;
  when any of those changes it is reported stale, not reused.
- **Views hold.** A consumer cannot see through an interface, by construction and not by a check.
- **The toolchain builds itself** to 29 native binaries in about four minutes, and the binaries
  agree with the bootstrap on every test.

### Weak or missing

*Trust.*
- The compiler to native code is unproven. Agreement with the evaluator on the tests is the only
  evidence, and it has been wrong outside them: a binary took the wrong branch of every `if` whose
  condition was not a `Bool` until a program outside the tests showed it (`CHECKS.md`, soundness
  lessons). What is known to differ today is in `OPEN.md` §1.
- The step from a function's definition to its program (the erasure) is trusted by review.
- The host-call axioms a contract trusts are checked against nothing.
- K is reviewed and tested, not proven. The export of Lean's library is fetched and built, and
  nothing in the repository fixes its bytes.

*The language.*
- Most of the toolchain's own code is a program with no definition in the logic, and nothing is
  proven of the rest (§2, "Two kinds of source"), so a tool's contract does not reach into the
  reader or the loader. The formatter's contract stops at "the reader is some function".
- Bytes and strings are lists. There are no packed buffers and no machine-word types that run as
  machine words.
- No instance search and no user notation: an operation at a type outside a fixed table must be
  written out in full.
- Each recursive call of a measured function needs a proof of decrease, written by hand wherever
  the proof search (`(fulfills NAME auto)`) does not find one — it does not find `n - 1 < n` from
  `¬ n < 2`. The formatter's printer needed 43.

*Proofs.*
- K's own evaluation is slow through measured functions: a closed call gives out at a depth between
  40 and 200. A test at program size cannot be stated as a theorem; it is run against the binary.
- Automation is thin: linear arithmetic, rewriting by named lemmas, a search tool that closes about
  half of calc's 100 claims unaided.
- An unfinished proof never fails a load by itself. Only a binary's contract does.

*Cost.*
- Checking is slow where proofs are long: the x86 trial's proofs load in about 20 seconds
  natively (`v3/models/x86/buffers_run.shard`, 1,680 declarations with what they rest on); the
  full replay of Init takes 7 minutes natively and 76 on the bootstrap.
- The full gate is 75 minutes in CI. A change to
  the toolchain is pushed after the checks it can move, minutes on the development machine, and
  the pipeline's verdict on the whole follows it.

## 5. Decisions most likely to be proposed again

Each of these was considered and turned down, with a reason that still holds. The full list, by
area, is `DECISIONS.md`.

- **Finding proofs of decrease automatically while loading.** A build never searches (`LAW.md`
  rule 14), so a later change to the search cannot change what loads. Decrease proofs are written out.
- **Lean's `mutual` blocks.** Functions that call each other are found by the loader when the last
  one is read; a `mutual` form would state the same fact a second time.
- **Structural recursion across a group of functions.** Not built: it refuses a call that passes
  its argument on unchanged, which most of the formatter's renderers do.
- **Type classes and instance search.** Refused. Instances come from a fixed table, and an interface
  with laws is a structure passed explicitly.
- **`sorry` as an axiom.** A theorem with `sorry` is not admitted at all, so nothing can depend on it.
- **`simp` with a lemma database, `omega`.** Rewriting takes its lemmas by name, and arithmetic is a
  certificate, so that a proof replays the same way forever.
- **Proving a test by evaluation inside K.** Too slow (see above). Tests run against the binary.
- **A cache that lets a load skip re-checking Init.** Measured: it saves a tenth to a fifth, and only
  a full replay could write it.
- **Trusting a hash.** A hash is a fingerprint for reports. Identity is by structure or by a number
  the system minted.
- **Packed byte buffers before the compiler exists.** Their representation belongs to the compiler;
  designed earlier they would be designed twice.
- **Writing a tool in the toolchain's own dialect.** Quicker, but nothing could then be stated about
  it. The formatter is an ordinary program for that reason.

## 6. Toward the trusted compile system

The goal (`LAW.md` §1) is a compiler from programs to machine code that comes with a proof. This is
what exists towards it and what does not. The design is not written; the statement it is to
be built to is `COMPILE.md`.

**Exists** (`v3/models/x86/`, its README):
- the machine's model: a program is bytes, a step decodes one instruction at the program counter
  and executes it; an assembler whose bytes the model's decoder reads back; the model held to the
  processor;
- laws proven once of the model (`laws.shard`): for each instruction a rule — where the processor
  decodes it at the program counter and the state after it comes to a state, the state before
  does —, what a store leaves as it was, a stored word read back, code unmoved by a store
  elsewhere; and of a function's stack frame (`frame.shard`): the words that stand from the stack
  pointer up to the return address, and what a push, a pop, a callee's return and the function's
  own return make of them;
- a trial of low-level code as ordinary shard (`buffers.shard`, `COMPILE.md` §4): functions over
  buffers of machine words that take the evidence of each access's range, a listing for them
  compiled by hand, and the contract of each one's code, proven against the functions'
  definitions: a sum of products, which writes nothing; a sum written into the buffer it was
  given, which changes no byte outside that buffer's words; and a caller of the two, which
  changes no byte outside that buffer's words and three words of its own stack;
- the step from a listing to the processor (`loaded.shard`): the code of any listing the
  assembler answers for is in place when its file is loaded, proven once from the assembler's
  check; and by it and the caller's contract, that the trial's file exits with the low byte of
  what its functions compute (`buffers_exits`), for the start the model's runner gives;
- a tactic library (`derive.shard`) that writes the proof of a run of a listing's instructions
  from the listing's table; nine of the trial's lemmas are proven by it.

**Exists** (`v3/compile/`, its README; the form is `COMPILE.md` §5):
- an intermediate machine a little above x86 (`ir.shard`): local words, word operations, loads
  and stores, `if`, calls, and a return or a tail call at the end of every path;
- the one statement about a function's code that crosses a module's seam (`x86/seam.shard`
  `realizes`), the same for compiled and for hand-written code;
- a pass, a function from a module's intermediate code to instructions of the model
  (`x86/pass.shard`), and its theorem, proven once (`x86/module.shard` `module_realizes`): where
  each function the module calls from outside meets the seam's statement, each function of the
  compiled module meets it. Its fragment: six locals in registers, expressions of word
  operations and loads, stores, `if` on an unsigned comparison, calls, tail calls;
- a link theorem (`x86/link.shard` `link_exits`): a file of a fixed start-up and any lines
  exits as the answer of the function it calls says, where that function meets the statement;
- one function through every layer: `dot`, a sum of products over two buffers, from its
  definition over lists to the exit status of an executable file (`x86/dot_run.shard`
  `dot_file_exits`), with its low form, its intermediate code and the two proofs above the
  pass written by hand.

**Does not exist:**
- the step from a definition to intermediate code: no tool reads a definition, and the proofs
  above the pass are written by hand, a function at a time;
- a build: nothing reads a `bin` form and assembles a file's facts; what a compiled file's
  start-up does is fixed, and its data is what the start-up pushes;
- in the pass: more than six locals, a function's call of itself that is not a tail call, a
  host call, a declared failure;
- hand-written code met through the seam's statement: the trial's contracts are in their own
  form;
- any link between the machine's model and what the present compiler emits;
- a way to turn a large computed result (megabytes of compiled output) into a theorem;
- the link from a binary's system calls to the `World` its contract speaks of.

The previous generation finished two parts of its compile proof in its own logic (the theorem from
its intermediate machine to x86, and its runtime's laws) and left its plan and its machine model
at a tag: `V2.md` §5 and §6.
