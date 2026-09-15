# The canonical form of S — CANON's rule set rewritten for V3 (phase 2, slice 8)

> **STATUS (2026-09-13): the rule set, written; the gate is phase 6.**
> This document is law §10.5's CANON row at phase 2: `docs/CANON.md`'s
> rule set rewritten for S, its §7 content addressing superseded by law
> §8.3, and the law itself carried — one canonical form, a gate that
> refuses rather than a printer that is trusted (FOUNDATION §5.1 "One
> canonical S"). It supersedes `docs/CANON.md` for the `v3/` tree; the
> old document keeps describing the old tree until the flip. The fmt
> gate on V3 — the recognizer and the formatter over these rules — is
> phase 6's (`v3/LANGUAGE.md` §12.5); until then this is the form the
> tree is written *toward*, measured by the sweep in §7, not refused by.
> Decisions this rewrite makes beyond the old law's text are collected
> in §9; the seven the user ruled on 2026-09-13 are marked RULED, the
> rest are for ratification with `v3/LANGUAGE.md` §13. GPT-6's
> checkpoint memo (R50, 2026-09-13) added §1's execution profile and
> its positions on the open items, recorded in §9; its ratification
> memo (R56, 2026-09-14; slice 10) bounded C9's domain to the types
> `if` may observe.

The surface this document canonicalizes is `v3/LANGUAGE.md`'s: S, its
explicit-L forms and its E forms at Stage 0. It never restates a
grammar; it cites LANGUAGE.md's sections and names one spelling where
that grammar admits several. What the old law established and this
rewrite keeps unchanged is cited by section of `docs/CANON.md`, never
redrawn.

## 1. What carries from the old law

- **The thesis** (old §1): the language admits one spelling per
  meaning, up to a stated, proven, growing quotient; the degrees of
  freedom in the surface are hazards to search, proof automation and
  caching, not neutral.
- **The architecture** (old §3): the recognizer is an admission gate,
  semantics-neutral and trust-neutral — it can only be too strict or
  too lax about admission, never prove anything; the rewriter is an
  untrusted tool under shardfmt's gate-not-printer contract; enforcement
  is a ratchet whose stages are separate decisions (§8 below).
- **The goal-position exemption** (old C1 scope ruling): positions
  that *relate* spellings are exempt by construction. In V3 these are
  the L forms — a `theorem`'s statement, a `requirement`, a `realize`'s
  equations — where two spellings of one value is the whole point. The
  term tier (§4) ranges over E bodies only.
- **The exclusions** (old §8): flow-dependent equality, extensional
  variation, the cond-mirror family, recursion shape, helper
  decomposition, general closed-call normalization — out, for the same
  reasons.
- **The certificate taxonomy and the escape-rule criterion** (old §6):
  a canon rule is definitional, lemma-cited or decision-procedure
  backed; a rule that lets an operand escape its forcing context needs
  the typed side-condition.
- **The verification discipline** (old §9): the exactness census, one
  negative fixture per rule, cross-implementation agreement between the
  recognizer and the rewriter, the proof-neutrality probe, two-level
  fingerprinting.
- **The depth price** (old §13's last note): a behavior's canonical
  spelling can sit one rung deeper than its shortest raw spelling; the
  size metric for budgeted tooling is the size of the canonical form.
- **The execution profile** (new; GPT-6 R50, 2026-09-13; §9 item 12).
  The term tier's quotient is stated over one observation relation:
  the value a body returns under adequate fuel and the World trace its
  externs produce. Fuel consumption is not observed — law §9.4's
  monotonicity is the contract, a result under a budget is the result
  under any larger one — and neither is host time or allocation. The
  rules that discard or move an evaluation — C2, C3's dead binding,
  C10, C11, C12 — are equalities under that relation only when the
  discarded computation is total and effect-free, which is what E is
  once Stage 1 has discharged every `fn`'s measure obligation (phase
  3) and the well-threadedness check refuses a World used twice
  (phase 4): a dead binding's extern would leave its World unused and
  the old one used again. Under strict evaluation an unused
  right-hand side still runs, so at Stage 0 a dead binding may loop
  and a discarded condition may perform an effect. Hence the
  ratchet's order (§8): the advisory recognizer REPORTS under Stage 0
  with the caveat named — "not applicable under Stage 0" for a rule
  whose precondition is absent, never a demand for a rewrite whose
  premise is missing — and the gate is phase 6's, after both. The
  same scope binds any use of the quotient by search: a representative
  replaces a spelling only under this relation and these conditions.

## 2. The three layers of S

The old law had one term structure to canonicalize, the core `Expr`.
S has three layers, and the rules split by layer:

| layer | ranges over | the rules |
|---|---|---|
| **layout** | the s-expression text (`LANGUAGE.md` §2) | §3: shardfmt's law carried, the V3 heads added, the reader's lexical rules |
| **the term tier** | E bodies — `fn` and a supplied `realize` — as `kernel/prog.shard`'s `ETerm` (nine formers: `EBVar ELit ECtor ECall EPrim EExt EMatch ELet EIf`; patterns `PVar PCtor PLit`) | §4: C1–C12 recast |
| **the L layer** | explicit L (`LANGUAGE.md` §5.1) and the file's header block (§3.1) | §5: new rules L1–L8 |

The canonical text of frontend parity (`LANGUAGE.md` §10 item 1,
`kernel/dump.shard`) is **not** this form: it is a comparison encoding
of an E program after reading — nameless, sorted, de-Bruijn — used to
tie two readers. The canonical form of S is a form of the *source*.

## 3. Layout — tier 0, D1

**Carried, the whole of shardfmt's law** (`tools/shardfmt`,
`meta/format`, `docs/CANON.md` §2 precedent 1): 80 columns; two-space
indent; fit-or-break; break after the head with a per-head count of
arguments that stay on the head line, greedy while they fit; all-atom
lists fill; a `match` is never flat; a broken `if` cascade holds one
column, a trailing comment on the condition or the then-branch rides
its line; closers pack; blank runs cap at two between top-level forms
and one inside; comments, the `'x` sugar and the raw bytes of atoms and
strings are preserved. The gate is the contract: the formatter refuses
to emit output that reads differently from its input.

**The V3 heads join the keep table.** Today's table (`meta/format`'s
`keep_n`) knows `fn sig extern claim requirement axiom fulfills goal
match if let type case-on case wf-induct induct have bin`. The V3 forms
of `LANGUAGE.md` §4 take these counts — the arguments that name the
declaration and its signature stay on the head line, the body breaks:

| head | keep | on the head line |
|---|---|---|
| `def`, `abbrev`, `opaque` | 3 | name, binders, type |
| `theorem` | 3 | name, binders, proposition |
| `axiom` | 3 | name, binders, type |
| `inductive` | 3 | name, binders, type |
| `structure` | 2 | name, binders |
| `realize` | 3 | name, binders, result (`(realize NAME (view))`: 2) |
| `requirement` | 2 | name, binders |
| `import`, `use`, `trusts` | all | one line each, atoms fill |
| `measure` | 1 | the measure term |
| `fun`, `forall` | 1 | the binders |
| `exact` | 1 | the term |

**The reader's lexical rules are the formatter's** (`LANGUAGE.md` §2):
the universe suffix `List.{0}` is one symbol opening a level list
closed by `}`; `'` is an identifier character and `'x` the quote macro
only at a token's start; `-7`, `"…"`, `(quote X)` and `(list …)` read
as E's literal rule says in every file (`LANGUAGE.md` §8.1 rule 2,
2026-09-14). The old formatter's reader
knows none of this — `Eq.{1}` is "unreadable source" to it (§7) — so the
V3 formatter is a printer over `kernel/sexpr.shard`'s s-expressions,
which already lex S.

**Sugar is presentation** (old D12, carried): the canonical form is
defined on what the reader builds; the rewriter emits the maximally
sugared surface. In E a byte string is written `"…"`, never
its `Cons` chain; a list literal `(list a b c)`, never its chain; a
symbol literal `'x`, never `(quote x)` — the quote macro is lexical
(`LANGUAGE.md` §2), so the two read alike in every position.

## 4. The term tier over E — C1–C12 recast

Each rule of `docs/CANON.md` §5 and §13, restated on E's formers. A
rule marked *carried* keeps its old statement with the names changed;
the three marked *recast* change under V3's semantics.

| rule | in S | status |
|---|---|---|
| **C1** no ground primitive redexes | no `EPrim` whose arguments are all literals, for any entry of the primitive table (`LANGUAGE.md` §6.4) whose guard the literals pass — a value spelled the long way; the intent-carrying spelling migrates to a named `fn` plus a `theorem` about it (old D7 ruling). The recognizer consults `ev`'s own table, as the old one consulted the reducer's, and inherits its bounded outcome (R51): an application the table leaves stuck or exhausted on those literals (`EvStuck guard`, `EvExhausted nat_size`) is not a redex — reported as such, never as canonical and never as invalid | carried |
| **C2** no decided control | no `EIf` whose condition is a constructor cell, no `EMatch` whose scrutinee is a ground constructor or literal; C11's empty-pin case | carried |
| **C3** let hygiene | **the flat sequential let is canonical**: a `let` whose body is a `let` merges into one binding list; bindings keep their written order, which is now meaning (sequential, R44); a binding the body and the later bindings never reference is refused. The old rule's "independent bindings merge into one parallel let, nesting spells dependence" is void — nesting spells nothing | **recast** (RULED 2026-09-13) |
| **C4** match arm discipline | constructor arms in the constructors' declaration order; literal arms ascending before the catch-all; at most one catch-all, last. The classifier's pattern matrix (`LANGUAGE.md` §6.7) already holds the declaration order | carried |
| **C5** no dead arms | an arm the matrix shows subsumed by the arms before it is refused — the same matrix that decides exhaustiveness | carried |
| **C6** ground naturals are literals | **by construction**: numerals are K's `LitNat`; `Nat.succ` and `Nat.zero` in an E body are refused by the reader with the pointer to numerals (`nat_constructor`, `LANGUAGE.md` §12.1). Nothing is left for a recognizer | carried, as a reader rule |
| **C7** no theory redexes, per ratified rule set | a rule set cites **theorems** (the old law's requirements): `(canon-rules NAME (rule THEOREM)…)` is the reserved form, the oriented rewrite read off the theorem's statement, the typed side-conditions its binder types; the escape-rule criterion sorts a set's rules before ratification; confluence is required of any set admitted while the store keys on the form. Waits for phase 3's I, when a theorem's statement is citable as a rule | carried, phase 3 |
| **C8** arm-local scrutinee discipline | carried whole: a variable scrutinee; in an arm whose pattern binds nothing the scrutinee variable does not occur in the body; no body contains the exact rebuild of its own arm's pattern over its own binders; ground pins compose through nested destructuring | carried |
| **C9** no match on a decision | **generalized**: `if` branches on the second constructor of any two-constructor type (`LANGUAGE.md` §6.2's tag rule), so a `match` over a two-constructor type whose arms bind nothing is a second spelling of an `if` — over `Bool`, Init's `Bool`, `Decidable`, and an `Option` matched with `_` in its `some` arm alike. The `if` is canonical, the second constructor's arm its then-branch. **Domain (GPT-6 R56, slice 10):** exactly the conditions `LANGUAGE.md` §6.2 admits — a transparent two-constructor type whose static type the declarations fix; a `sig type` scrutinee is neither matchable (`private_match`) nor a condition (`private_if`), so C9 never reaches an opaque type, and a type parameter's scrutinee is left as written at Stage 0. Dropping a payload from a pattern is not licence to drop the computation that produced it: §1's execution profile governs, as for C2 and C3 | **recast** (RULED 2026-09-13); domain bounded 2026-09-14 |
| **C10** no vacuous control | carried: an `if` whose branches are structurally equal; a covered match whose arms ignore their binders and agree; a covered match whose arms respell the scrutinee. Coverage is the classifier's exhaustiveness | carried |
| **C11** contextual partial evaluation | carried: a body subterm that can take one of `ev`'s steps under the arm's pins is a contextual redex; the `if` tier stays dropped, and the fold tier stays at the D19a ruling — no fold of applied ground user calls, the C7 extension the mechanism of record if written code ever accumulates them | carried |
| **C12** no needless case split | carried as emergent: C10's constant-match rule after C11 in the rewriter's fixpoint | carried |

Three notes on scope. E's symbols (`'x`, `sym_eq`) and its negative
numerals are E values like any other under these rules, in every file
since the one-E ruling (`LANGUAGE.md` §8); `(list …)` is sugar (§3). A `realize`'s supplied body is
an E body: C1–C12 apply to it; its equations are L and exempt. And the
rules that discard an evaluation — C2, C3's dead binding, C10, C11,
C12 — hold under §1's execution profile only (R50): what the
recognizer reports while a precondition is absent is stated there.

## 5. The L layer — new rules

Explicit L at Stage 0 (`LANGUAGE.md` §5.1) is written term for term
into K's `Expr`, so its spelling freedom is exactly where two S
spellings elaborate to one `Expr`. Each rule below names one.

- **L1 arrows.** A non-dependent Pi is written `(-> A B)`, right-nested
  for a chain; `(forall ((x A)) B)` only when `x` occurs in `B`. One
  `Expr` either way; the arrow is the form that says the binder is
  unused.
- **L2 sorts.** `Prop`, `Type` and `(Type u)` are written, never
  `(Sort 0)`, `(Sort 1)`, `(Sort (succ u))`; `(Sort L)` only for a level
  neither form spells (`(Sort (max u v))`, `(Sort (imax u v))`).
- **L3 application is flat.** `(f a b)`, never `((f a) b)`: the reader
  left-nests either into the same `Expr`.
- **L4 universe arguments.** Every polymorphic constant cited in L
  carries its full suffix (`List.{0}`; a bare citation is a Stage-0
  error already); in an E-type position the suffix is never written,
  since the position instantiates at level 0 by rule (`LANGUAGE.md`
  §4). A level is written in K's normal form (`level.shard`'s
  `normalize`): `u`, never `(max u 0)` or `(imax u u)`. *For
  ratification (§9 item 5): the normal-form rule asks the author for
  a computation K performs anyway.*
- **L5 let.** As C3: one flat binding list, `(let ((x TYPE TERM)…)
  TERM)`, a let whose body is a let merged, a binding nothing after it
  references refused. The reader makes one L `Let` per binding either
  way.
- **L6 proofs.** At Stage 0 a `theorem`'s proof is `(exact TERM)` or
  `sorry`; no other form exists, so nothing to choose. `sorry` is
  reported, never canonical-form business.
- **L7 binders.** `((x TYPE))` with the info marker only when it is not
  the default (`implicit`, `strict`, `inst`); binder names are outside
  the form (old D3, carried — K's `Expr` keeps them for display only).
  *For ratification (§9 item 6): an unused pattern or binder variable
  is spelled `_`.*
- **L8 the header block.** A file's forms before its first declaration
  are, in this order: at most one `(import Init NAME)`; the file and
  directory imports, sorted bytewise by their path; the `use` lines,
  sorted bytewise by prefix, each member list sorted; `(trusts …)`, its
  names sorted. The loader's semantics depend on none of these orders
  (visibility is the closure, `LANGUAGE.md` §3.3), so the order is
  presentation and gets one spelling. **Declaration order** stays the
  old D13: census first. The loader fixes only what it must — a
  `realize` after the realizations it calls, an L citation after its
  constant — and the rest is open until measured.

The old C6, ground naturals, has an L cousin already in force: a
numeral is `LitNat`, and `(Nat.succ (Nat.succ Nat.zero))` in L is a
different `Expr` from `2` — two *values* K identifies by its literal
rule, not two spellings of one term. No rule here; the term tier's C1
does not reach L.

## 6. Identity and content addressing — §7 superseded

`docs/CANON.md` §7 defined a definition's identity as a Merkle hash of
its canonical nameless core: a tagged serialization, out-of-component
references by digest, within-component references by a canonical
member index, SHA-256 behind a swap point. Law §8.3 supersedes it: a
declaration's identity is its qualified name within the logical package
root plus the content hash of its **L** revision, and identity hashes
are over L and P, **never over S text** (FOUNDATION §5.1). At phase 2
that hash is `expr.shard`'s structural hash — a fingerprint for records
and fixtures, never an authority (GPT-6 R42; `LANGUAGE.md` §3); the
collision-resistant identity the store needs is chosen with the store
at phase 3.

What this rewrite keeps from §7, as a note for the store's designer
rather than a rule: the within-component ordering lesson — sort a
strongly connected component's members by their serialization with the
same-component references replaced by a fixed placeholder, so that
declaration order reaches an identity only for byte-identical mutual
twins, documented and vanishingly rare.

The canonical form's role under the new identity: the term tier and
the layout never reach L at all — E is a separate pipeline and layout
is bytes — and the L rules L1–L5 are exactly the S spellings that
elaborate to one `Expr`. So canonical S is injective into L up to the
instituted quotient: two canonical files that read to the same
declaration are the same file up to binder names. That is what makes
"key nothing on S" free of cost — nothing about identity is lost by
keying on L.

## 7. Where the tree stands — the migration baseline (2026-09-13)

Measured before the rules were written, with the old tree's
`shardfmt --check` over every V3 source (210 files; `bin/shard_eval`):

| area | canonical | drifting | unreadable |
|---|---|---|---|
| `v3/kernel` (30 files) | 1 | 29 | 0 |
| `v3/kernel/test` (20) | 0 | 20 | 0 |
| `v3/examples/calc` (14) | 7 | 7 | 0 |
| `v3/pins` (146) | 101 | 42 | 3 |

The drift is layout: 3,103 of `v3/kernel`'s 17,745 lines exceed 80
columns, and the rest is comment alignment — the tree was written
without a gate. The three unreadable files are two pins broken on
purpose (`read_error`, `refuse_unterminated`) and one S pin the old
reader cannot lex: `Eq.{1}`, the universe suffix. The term tier's
baseline is smaller: the toolchain's 6,304 functions hold 163 `let`s,
27 with several bindings and 10 whose body is a `let` (C3's recast
touches those ten); C1, C2, C4, C5, C8–C10 have no V3 recognizer yet
to count with, so their baseline is the stage-1 slice's first output.
Migration order carries from the old §10 — the smallest tree first, the
kernel sources last, and the proof-neutrality probe before any sweep —
and lands at phase 6.

## 8. Enforcement — the ratchet on V3

The old §3 ratchet, restated for the new tree; each stage is its own
decision:

0. **The rule set** — this document (slice 8, 2026-09-13).
1. **The recognizer, advisory.** The E tier (§4) as one `CANON NAME:
   C<k> subject` record per violation from the classifier, which
   already holds every input the rules need (the pattern matrix, the
   static types, the primitive table); `v3/pins/canon/` with one
   negative fixture per rule; the census of the toolchain's own
   sources as the first output. The first step of the phase-6 fmt-gate
   slice, or earlier on demand. Its reports for C2, C3, C10–C12 carry
   §1's caveat under Stage 0, and its fixtures include R50's six: an
   unused terminating computation, an unused Stage-0 recursion, an
   unused extern result, equal branches under a looping or effectful
   condition, flattened `let`s under shadowed names, a
   resource-sensitive body — each with the report expected.
2. **The tree canonical, pinned.** `v3/kernel` reformatted under the V3
   formatter — a printer over `kernel/sexpr.shard` — and its E bodies at
   zero advisory lines; the loader pins gain the canon cases.
3. **The gate.** `v3/test.sh` refuses a drifting file, as
   `bin/rebuild.sh` refuses to stamp one today — the fmt gate on V3
   (`LANGUAGE.md` §12.5, phase 6).
4. **Read-time refusal for the syntactic tier** — the "deny parse" end
   state; a later decision, reachable only for C1–C6 and the L rules,
   never for C7 and C11, which are check-time by nature.

The tools follow the manifest: `tools/shardfmt` ports (`v3/MANIFEST.md`,
the goal's flagship), `tools/canon` is new code over these rules; both
keep the gate-not-printer contract, and the recognizer stays in the
kernel tree, next to the classifier.

## 9. For ratification — decisions made here beyond the old law

Seven were put to the user on 2026-09-13 and ruled; the rest are open.
GPT-6's positions on items 7–11 (the checkpoint memo, 2026-09-13,
records §4.8) are recorded under each for the ratification pass.

1. **RULED — home.** A file of its own beside `LANGUAGE.md`, superseding
   `docs/CANON.md` for the V3 tree with a banner each way. Alternative:
   a section of `LANGUAGE.md`; declined because the two documents have
   different readers — the surface's author and the gate's builder.
2. **RULED — scope.** The rule set alone at slice 8; the E recognizer
   (§8 stage 1) named as the phase-6 slice's first step. Alternative:
   build the recognizer now; declined as the old law's own precedent —
   ratify the document, then land its first slice.
3. **RULED — C3's recast** (§4). Alternative: keep nesting as a
   spelling of dependence; void under sequential let.
4. **RULED — C9's generalization** (§4). Alternative: `Bool`-shaped
   types only; declined because `if`'s tag rule already reaches every
   two-constructor type.
5. **RULED — no reformat now.** The 98 drifting files wait for the V3
   formatter at phase 6. Alternative: the old tool now; it cannot lex
   the universe suffix and has no counts for the V3 heads.
6. **RULED — §7 superseded** by law §8.3 with the store (§6), the
   ordering lesson kept as a note. Alternative: carry the Merkle spec
   into V3; declined because the law keys identity on L, never S.
7. **OPEN — L4's normal-form levels.** Written in K's normal form, or
   as the author spells them with the store keying on the normalized
   level. Lean: normal form, since one spelling per level is the thesis
   and `normalize` is one call. GPT-6: accept, keeping raw K input
   acceptance distinct from canonical P identity; an unsupported or
   exhausted normalization is reported, never read as invalid.
8. **OPEN — `_` for an unused variable** (L7). A syntactic rule the
   old law did not have; cheap for a recognizer, useful to a reader.
   Lean: adopt, in the term tier and L alike. GPT-6: accept for a
   genuinely unused binder — one whose occurrences are counted in the
   later types, propositions and proof terms too, not only at runtime
   — preserving scope and index meaning.
9. **OPEN — L8's header order.** Bytewise sorting of imports and `use`
   lines, or written order preserved. Lean: sorted, since the loader
   reads no meaning into the order. GPT-6: accept for the fragment
   shown order-independent by a fixture that permutes the imports and
   `use` lines and compares the resolved identities, the ambiguity
   outcomes, the accepted declarations and the effective policy — not
   by the comment alone; identical diagnostic order is not required.
10. **OPEN — the reserved `canon-rules` form** (C7) as the phase-3
    spelling of a rule set. Lean: reserve the word now, decide the form
    with I. GPT-6: the same; the form carries a rule's direction, its
    conditions, its dependency identity and the relation that permits
    replacement, and adds no inference rule.
11. **OPEN — declaration order** (old D13, L8): census at stage 1
    before any rule. GPT-6: keep deferred; the required dependency and
    realization order is preserved and no broad reordering lands
    without a measured need.
12. **RULED — the execution profile** (§1; GPT-6 R50, 2026-09-13; the
    user's go-ahead on the slice-9 dispositions). The term tier's
    quotient is the value-and-trace relation and the discarding rules
    apply only where E is total and World-threaded — reported, not
    gated, under Stage 0. Alternative: an unconditional quotient ("an
    unused value is unobservable"); rejected because under strict
    evaluation an unused right-hand side still runs, and at Stage 0 it
    may loop or perform an effect.

## 10. Related

`docs/CANON.md` (the old tree's law, kept until the flip; its §12
record at `docs/records/CANON.md`); `v3/LANGUAGE.md` §2 (lexical
syntax), §4 (forms), §5.1 (explicit L), §5.4 (E terms), §6.2 (`ev`'s
tag rule), §6.4 (the primitive table), §6.7 (the classifier's matrix),
§10 item 1 (the parity text, which this form is not), §11 and §12.5
(the fmt gate at phase 6), §13 (ratification); `docs/FOUNDATION.md`
§5.1 "One canonical S", §8.3 identity, §10.5 the disposition row;
`tools/shardfmt` and `meta/format` (the layout law's implementation);
`kernel/canon.shard` and `tools/canon/` (the old recognizer and
rewriter, the reference for stage 1).
