# Shard v3 trajectory review for Fable

- Author: Codex (GPT-6), working in `shard/shard.main`
- Date: 2026-10-06
- Reviewed branch: `main`
- Reviewed revision: `7e99261896060f3986507ed6618c388ea603618f`

Status: Review and proposals for discussion. No implementation changes or
changes to ratified decisions are included.

**V3 is moving toward a useful system. Confidence in its logical foundation
has grown faster than confidence in its authoring experience.** The largest
risks now concern predictable elaboration, inexpensive feedback, and
composition across abstraction boundaries. My expectation is strong value
as a verified computational substrate, with broader language usefulness
depending on how those risks are resolved.

The recent trajectory contains several decisions worth keeping:

- Lean-exact checking and shared mathematical identities give future
  reasoning facilities an established foundation.
- Reading a function once and deriving its executable body from that typed
  representation removes a substantial source of disagreement.
- Moving arithmetic and other proof machinery above K lets capabilities
  grow without expanding the trusted checker for each one.
- Using calc as a real consumer paid off. Its proofs exposed termination
  obligations that were false as generated; repairing and discharging them
  provides substantive integration evidence.

The six findings below separate observed behavior from predicted costs and
proposed alternatives. References are relative to the project root; source
line numbers describe the reviewed revision. A self-contained reproduction
script follows the assessment so the observations do not depend on scratch
files from the review session.

**1. Elaboration can establish a meaning the author did not intend**

Observed through the bootstrap loader:

```lisp
(def inferred () Int (- 1 2))
(def explicit () Int (Int.sub 1 2))
```

K accepts `inferred = 0` and `explicit = -1`. It also accepts that `inferred`
equals `Int.ofNat (Nat.sub 1 2)`.

The operator elaborator determines the operation from an operand before
applying the surrounding expected type. The later `Nat → Int` coercion makes
the result type-correct. This behavior sits awkwardly against the foundation's
rule that available constraints precede numeric defaulting. K correctly checks
the elaborated meaning; the problem is the source interpretation.

Evidence: [elab.shard](v3/kernel/elab.shard), `elab_op_at` at line 755 and
`check_expected` at line 212; [FOUNDATION.md](docs/FOUNDATION.md) §5.2,
especially the defaulting order at line 545. Reproduction A below includes
the explicit-operation control and three accepted theorems.

Predicted pain: arithmetic-heavy ports and newly written requirements can
acquire unintended semantics while their proofs check perfectly. Successful
verification can then strengthen confidence in the mistaken interpretation.
This is particularly costly if both a specification and its implementation
inherit the same unintended numeric reading.

Proposed direction: propagate expected types through operator expressions
before choosing numeric operations or defaulting literals. Preserve explicit
numeric boundaries where the author supplies them. Expose the resolved
arithmetic when reviewing a requirement, including operation identities and
coercion sites. That visibility remains useful after this implementation
defect is fixed and follows the existing resolved-requirement contract.

**2. Module meaning currently depends on what else has been loaded**

Observed: two independent roots give different results when loaded in opposite
orders. The small module imports through `Int.decEq`, opens `Init.Bool`, and
uses `decide_eq_true`. The other root only imports through
`Bool.decide_eq_true`.

- Small first, wide second: the theorem is accepted.
- Wide first, small second: the unchanged small module reports
  `ambiguous_name`, naming `Bool.decide_eq_true` and `decide_eq_true`.

The scope has a Boolean flag for Init visibility. Imported declarations
already resident in the environment become visible together; the source
scope does not retain a separate per-module imported prefix restriction.

Evidence: [scope.shard](v3/kernel/scope.shard), `visible` at line 127;
[loader.shard](v3/kernel/loader.shard), `Fx` and `fx_scope` around lines
269–281. Reproduction B below loads the same two files in both orders. The
latest as-built already acknowledges wider Init imports causing ambiguity;
this probe establishes the order dependence directly.

Predicted pain: adding dependencies, combining projects, and changing worker
scheduling produce apparently unrelated failures. Incremental checking and
parallel agents will amplify the cost because those facilities depend on
stable dependency boundaries.

Proposed direction: separate declarations resident in the checked environment
from declarations visible to a module. Shared loading and caching should
remain possible without expanding a module's namespace. A per-module prefix
restriction addresses the immediate problem. Indexed declaration dependencies
could eventually remove the need to import by position in a large export
stream. These are separable steps; replacing the importer is not a prerequisite
for fixing visibility.

**3. Authored proof scripts and certificate IR have different maintenance needs**

The decision to identify authored tactic blocks with I has a clear attraction:
one syntax, fewer translations, and deterministic reconstruction. The current
surface nevertheless exposes operational details that make maintenance
brittle.

Two behaviors were reproduced:

- Adding a second `reduce` after normalization causes `reduce_stuck` on
  `(= 3 3)`.
- An explicit arithmetic certificate stops working after inserting an
  irrelevant local proof of `x = x`, while the theorem's statement remains
  unchanged. The added row shifts the certificate's positional weights.

For the latter, these proof blocks have the same target:

```lisp
(by (intro h) (arith (farkas 1 1)))
(by (have unused (= x x) rfl) (intro h) (arith (farkas 1 1)))
```

The first is accepted. The second fails with rows corresponding to the
negated goal, `0 = 0`, and the useful hypothesis, in that order.

Ordinary `arith` avoids this positional maintenance problem by reconstructing
the certificate. Calc uses that form: repository search found no explicit
`farkas` or `occ` clauses in its examples. The positional issue therefore
primarily concerns explicit certificates and future producers. Default
reconstruction has a different scaling risk: irrelevant context growth can
increase work or encounter the reconstruction budget.

Evidence: [tactic.shard](v3/kernel/tactic.shard), `NRw` and `NArith` at lines
89–91; [arith.shard](v3/kernel/arith.shard), `ar_refute` at line 775;
[LANGUAGE.md](v3/LANGUAGE.md), slice 3.20's source-is-I ruling around line
3502 and slice 3.21's row ordering around line 3844. Reproductions C and D
below preserve the exact small cases.

Proposed direction for discussion: retain one author-facing grammar while
distinguishing an authored node from its resolved certificate. Resolution can
record selected facts, instantiated lemmas, guarded occurrences, and
reconstruction versions in the sidecar. A shared syntax and schema can
support both stages without introducing another user language. This would
reconsider the operational consequences of the recent ruling, rather than
silently treating it as unratified.

Normalization could succeed unchanged when there is nothing to do, with a
separate operation requiring progress for procedures that need that contract.

Retained P already protects independent release verification from I
reconstruction issues. The concern here is maintaining source and regenerating
evidence, not previously accepted evidence becoming unverifiable. That
distinction follows [FOUNDATION.md](docs/FOUNDATION.md) §7.5.

Before stabilizing store and engine interfaces, also exercise dependent
witness goals. Today `apply Eq.trans` cannot leave its middle term to be
determined by subsequent proof work. The implementation explicitly names
this as deferred graph work: [tactic.shard](v3/kernel/tactic.shard),
`premise_goals` at line 630, and the existing
[by_apply_unsolved pin](v3/pins/loader/by_apply_unsolved/main.shard).
The final graph needs to support unknown data and dependent proof obligations,
as the foundation already requires.

**4. Feedback cost is a primary language-design constraint**

A fresh bootstrap load of reproduction A's two definitions and three trivial
proofs took 4.82 seconds. The two scope-order runs took 37.59 and 37.67
seconds. These are individual observations of route 3, not statistical
benchmarks or forecasts of an optimized implementation. The scope probes
ran concurrently with other small probes.

The repository records pipeline 544's v3 job at 6,055 seconds, with full
replay at 32.1 GB peak. Those are useful validation measurements; they do not
establish interactive edit cost. The full suite was not rerun for this review.
Source: [v3/README.md](v3/README.md), slice 3.21's closing gate at line 1017.

An LLM author pays build cost repeatedly while discovering a suitable
formulation. More capable reasoning has limited practical value if each
failed attempt rebuilds substantial context. The README's premise that
build time is practically free needs qualification for this workflow.

There is also a context cost. At this revision `LANGUAGE.md` is 5,124 lines,
and its opening still says I is absent although I is implemented and
documented later in the file. The promised `docs/LEAN.md` authoring map does
not yet exist. These are specific documentation cleanup items for the
authoring work; this review does not edit them.

Proposed sequencing: demonstrate a persistent checked session with structured
goals, occurrence-aware diagnostics, bounded local rechecking, and a compact
description of current behavior before expanding proof production or
migration broadly. Much of this is already in the foundation. The proposed
change is to prioritize demonstrated usability of those commitments.

Measure time to diagnose and repair a change, warm work after one edit, and
retained memory after failed branches. Keep cold release checking as a
separate measurement. The existing T9 authoring gate is an appropriate home
for this evidence: [FOUNDATION.md](docs/FOUNDATION.md) §5.4 and §12.5.

**5. Composition through execution and representation boundaries remains uncertain**

Calc demonstrates useful pure reasoning. Its success does not yet settle
the hardest promises of the eventual executable system.

Current arrays and byte arrays still use lists. Checked module instantiation
is specified as substitution into an existing consumer proof followed by K
rechecking; the current implementation checker combines implementation
records and executable bodies. Effect ownership and trace preservation also
remain substantial implementation work. These are acknowledged commitments
in the design.

Evidence: [LANGUAGE.md](v3/LANGUAGE.md), slice 3.18's representation rows at
line 2950 and checked instances at line 4104;
[loader.shard](v3/kernel/loader.shard), `check_impl` at line 1297;
[FOUNDATION.md](docs/FOUNDATION.md) §4.7.

Predicted pain: the contracts connecting these pieces may demand more author
work than their individual examples suggest. Representation invariants,
resource premises, implementation substitution, and effect observations can
interact. The design states the intended safeguards; implementation and
authoring cost still need evidence from their composition.

Proposed sequencing: exercise a small connected consumer before broad
migration. Validate external data, carry an invariant into a computation,
use a module through its view, retain its proof, and execute its selected
implementation. Then change one implementation or representation without
rewriting the consumer's proof. Include an early compiler-certificate
consumer to test the workload that motivated the foundation work.

The foundation already proposes a first connected path in §12.4. This
recommendation gives that path more weight in deciding what to build next;
it does not claim a missing architectural obligation.

**6. First-order residual execution needs a source-composition workload**

The design promises automatic specialization, closed lambdas, partial
application, and lifting nonescaping captures. Ordinary higher-order
conveniences therefore need not become permanent manual work.

The lasting restriction is escaping function values: callbacks stored in
structures, returned functions, and extensible runtime function dispatch.
Those must use sealed variants. Evidence:
[FOUNDATION.md](docs/FOUNDATION.md) §4.3, beginning at line 269.

Predicted pain: independently authored libraries may compose poorly when
their combination requires extending a shared callback variant or dispatch
function. Parser combinators, event handling, and reusable search machinery
are plausible workloads. This is a hypothesis about extensibility, not a
measured limitation of the current calc consumer.

An alternative worth pricing is a broader source form where whole-program
defunctionalization turns a finite set of function values into generated
variants and direct dispatch. Residual E could remain first-order. This
would deliberately reopen the source escape restriction and add compiler
and proof work, so it should be compared against a concrete workload before
adoption. The existing workload-triggered reopening condition provides a
place for that comparison.

The discriminating experiment is two independently written callback-using
libraries and a third-party composition. Measure author edits, generated
code, specialization growth, and proof reuse when another callback is added.
A `map` example exercises the already planned specialization facilities,
not this disputed boundary.

**Expected usefulness to an LLM author**

Today I could use v3 to write and prove modest pure components, with
substantial reliance on repository examples and implementation knowledge.
I would expect considerable friction around unfamiliar library operations,
dependent constructions, and execution boundaries. This is an authoring
assessment, not a comparative model benchmark.

With the planned trajectory completed, my expectations are:

| Work | Expected value |
|---|---|
| Parsers, codecs, arithmetic components, state machines | High once standard proof patterns and boundary validators are reusable |
| IR transformations, optimizers, generated implementations | Very high potential because many candidates can share a reviewed requirement and reusable correctness machinery |
| General application plumbing and effect-heavy orchestration | Uncertain until ownership, host interaction, and composition are demonstrated together |
| Broad mathematical work | Useful checking foundation; authoring value depends heavily on library access and elaboration ergonomics |

The benefit I expect most is reliable delegation. I can produce a candidate,
obtain precise obligations, repair it, and leave evidence another agent can
check without trusting my explanation. Shared identities, explicit
assumptions, opaque interfaces, and retained proofs support that workflow.

The main advantage need not be fewer keystrokes or a tiny grammar. My
productivity depends more on predictable meanings, discoverable lemmas,
useful failure reports, and cheap retries. The numeric probe illustrates why
the reviewed requirement remains crucial: K can establish the elaborated
proposition perfectly while the author meant something else.

I would keep the core architecture and assess the next stretch by
independently authored changes completed per hour, including repair after
refactoring, under a fixed reviewed requirement. That directly tests the
system's intended advantage for authors like me.

For Fable's review, the distinctions that matter are: findings 1 and 2 have
reproduced behavior and concrete implementation causes; finding 3 asks for a
bounded reconsideration of a recent ratified choice; findings 4 and 5 mostly
change the priority of commitments already in the law; finding 6 asks for a
discriminating workload before any architectural reversal.

**Reproductions**

Run the following Python script from the project root. It uses the existing
release bootstrap and checked-in Init fixtures, creates temporary probe
sources outside the repository, and prints each loader result. Expected
negative cases return a nonzero status. The package root passed to the
loader is relative to the scratch working directory, matching its path
handling at the reviewed revision.

```python
from pathlib import Path
from tempfile import TemporaryDirectory
import subprocess
import time

repo = Path.cwd().resolve()
sources = {
    "numeric_control.shard": """(import Init Int.decEq)
(def inferred () Int (- 1 2))
(def explicit () Int (Int.sub 1 2))
(theorem inferred_resolved () (= inferred (Int.ofNat (Nat.sub 1 2))) (by rfl))
(theorem inferred_zero () (= inferred 0) (by rfl))
(theorem explicit_negative () (= explicit -1) (by rfl))
""",
    "small.shard": """(import Init Int.decEq)
(use Init.Bool)
(theorem t ((p Prop) (d (Decidable p) inst) (h p)) (= (@Decidable.decide p d) true)
  (by (exact (decide_eq_true h))))
""",
    "wide.shard": """(import Init Bool.decide_eq_true)
""",
    "reduce_once.shard": """(import Init Int.decEq)
(theorem t () (= (+ 1 2) 3) (by (reduce) rfl))
""",
    "reduce_twice.shard": """(import Init Int.decEq)
(theorem t () (= (+ 1 2) 3) (by (reduce) (reduce) rfl))
""",
    "arith_base.shard": """(import Init Lean.Omega.Int.ofNat_lt_of_lt)
(theorem t ((x Int)) (-> (<= 0 x) (<= 48 (+ 48 x)))
  (by (intro h) (arith (farkas 1 1))))
""",
    "arith_have.shard": """(import Init Lean.Omega.Int.ofNat_lt_of_lt)
(theorem t ((x Int)) (-> (<= 0 x) (<= 48 (+ 48 x)))
  (by (have unused (= x x) rfl) (intro h) (arith (farkas 1 1))))
""",
}

cases = [
    ("A numeric interpretation", ["numeric_control.shard"], 0),
    ("B small then wide", ["small.shard", "wide.shard"], 0),
    ("B wide then small", ["wide.shard", "small.shard"], 1),
    ("C redundant reduce", ["reduce_once.shard", "reduce_twice.shard"], 1),
    ("D irrelevant arithmetic row", ["arith_base.shard", "arith_have.shard"], 1),
]

with TemporaryDirectory(prefix="shard-v3-review-") as scratch:
    package = Path(scratch) / "case"
    package.mkdir()
    for name, source in sources.items():
        (package / name).write_text(source)

    base = [
        str(repo / "rust_bootstrap/target/release/eval"),
        "direct", str(repo / "v3/kernel/load.shard"),
        "--root", "case",
        "--init", str(repo / "v3/kernel/test/fixtures/init_prefix_int.ndjson"),
    ]
    tail = [
        "--init",
        str(repo / "v3/kernel/test/fixtures/init_prefix_str_tail.ndjson"),
    ]
    for label, files, expected_exit in cases:
        command = base + ([] if label.startswith("A ") else tail)
        command += ["case/" + name for name in files]
        start = time.monotonic()
        result = subprocess.run(command, cwd=scratch, text=True, capture_output=True)
        print(label, flush=True)
        print(result.stdout, end="")
        print(result.stderr, end="")
        print(f"exit={result.returncode}, expected={expected_exit}, "
              f"elapsed={time.monotonic() - start:.2f}s", flush=True)
```

Expected observations at the reviewed revision:

| Probe | Positive control | Observed failure or distinguishing result |
|---|---|---|
| A | Explicit `Int.sub` equals `-1` | Inferred subtraction equals `0` and `Int.ofNat (Nat.sub 1 2)`; all five declarations accepted |
| B | Small then wide accepts `small.t` | Wide then small reports `ambiguous_name` for `decide_eq_true` |
| C | One `reduce` accepts `reduce_once.t` | Second `reduce` reports `reduce_stuck` at step 2 on `(= 3 3)` |
| D | Base certificate accepts `arith_base.t` | Adding the local reflexive fact reports `arith_certificate` at step 3, with rows `[0 <= -1 + -1*x] [0 = 0] [0 <= 0 + 1*x]` |

The review executed these source probes through route 3 using shell
invocations. The Python block consolidates them into a portable runner; its
syntax was checked and all seven embedded sources were compared with the
executed originals. The consolidated runner was not separately executed,
and its sequential timings need not match the original session. The full CI
figures cited above remain the repository's recorded results.
