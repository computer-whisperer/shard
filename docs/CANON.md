shard canonicalization — CANON.md
=================================

STATUS: RATIFIED (user review completed 2026-07-10) — §1–§10 stand as
the arc's scope ledger; §11 items remain OPEN questions, each needing
its own ruling before any code assumes an answer. The v1 cut is C1–C9
at enforcement stages 1–2. Also ratified: (a) canonicalization is the
next core-shaping arc; (b) content addressing is a direction shard
should adopt and exploit — it constrains the canonical form now even
though its consumers land later; (c) the D7 ruling recorded at C1;
(d) the census-driven sharpening of 2026-07-10 (playground commits
f76845a + d9c7dbf): C8 joins the v1 cut, §6 gains the certificate
taxonomy and the escape-rule criterion, §8/§9 gain the census's
boundary examples and verification lesson — all ratified before
stage 2 so the whole cut rides ONE std migration.

The evidence base is ~/workspace/playground/shard_search_playground
(read as data, never touched): needed narrowing over shard terms, three
engines at equal work currency, and the canonicalization experiments
its README records. Numbers cited below are from that README.


## 1. Why: the residual region

Programming languages were designed under a filter — humans must be
able to bear writing and reading them. That filter silently rejected
whole regions of the design space: languages that demand totality
proofs, languages with one spelling per program, languages where the
source form is a normal form. shard already lives in the rejected
region on the first axis (mandatory totality, mandatory proofs); this
arc explores the second: **invariant-forced canonicalization** — the
language admits one spelling per meaning, up to a stated, proven,
growing quotient.

The prototype's export is that the degrees of freedom in base shard
are not neutral: they are HAZARDS to program search, and (the same
fact seen from other angles) to proof automation, to caching, and to
any consumer that must decide term equality. Measured, on rev-from-
examples with the append theory as the quotient:

- Ground search, depth 4: the full grammar (1.09e15 candidates) was
  UNFINISHABLE — 67% settled at 4m21s, aborted at 25GB; the fork tree
  tracks extensional DUPLICATES, not candidates. The lemma-quotiented
  grammar settled its entire remaining space (2.25e12) in **4,095
  evaluator steps**. The space shrank 485x; the work fell from
  unfinishable to seconds. The duplicates were the whole cost.
- Law-directed search (proofs instead of tests), depth 2: undecided
  candidates fell 9,318 → 1,040 with canonical joins, → 138 with the
  quotiented grammar composed in; the lemma-hard bucket flipped to
  proven (2 → 6 certified against the interface). Depth 3:
  quotient+join runs at 16,093 steps where join-alone needs 51.2M.
- The quotient was verified EXACT at depths 1–3 (87 / 2,787 /
  2,597,487 canonical candidates, censused term-by-term against the
  full grammar's normal forms): nothing lost, nothing added.

The rule set that did this is four std/list requirements the kernel
had already checked. The quotient was sitting in the tree, proven,
waiting to be applied at the right boundary.

## 2. The five precedents already in the tree

shard has converged on invariant-forced canonicalization piecemeal,
five times, without naming it:

1. **shardfmt** — byte layout. Gate-not-printer; the repo is
   formatted; bin/rebuild.sh REFUSES to stamp non-canonical inputs.
2. **prove regen** — proof text. Sidecars must regenerate
   byte-identically (the regen gate IS an is-canonical check).
3. **Nat packing** — ground values. A nonneg IntLit IS the packed
   ground-Nat normal form; packing lives in step+ceval only.
4. **Qualified identity** — names. Every reference resolves to one
   QName; bare-name freedom is gone from the core.
5. **Citation desugaring** — proof references. Named `(hyp ih)` is
   REWRITTEN to positional `(Hyp k)` before parse; the AST has no
   name ctor at all (kernel/desugar.shard).

Each has the same shape: a canonical core, a presentation surface,
a deterministic bridge, and a gate. Each was adopted because a
machine author plus a gate made bearable what humans wouldn't
tolerate. This arc generalizes the pattern to TERM STRUCTURE — the
one layer still uncanonicalized.

The cost of its absence is not hypothetical: friction #6 (2026-07-10,
X86.md §45 rider) was two spellings of the same ground Nat — source
tower vs packed literal — leaking across the reducer/type-gate
boundary and demanding a typing view to bridge. Canonicalization is
the systematic fix for that defect class: fewer forms, fewer bridges.

## 3. Architecture: recognizer, rewriter, ratchet

The load-bearing safety property: **an admission gate is semantics-
neutral and trust-neutral.** `is_canonical` admits or rejects source;
it never changes what a term means, never changes what the kernel
accepts as proof. A wrong recognizer can only be too strict or too
lax about ADMISSION — it cannot prove false things. v1 can therefore
be iterated on aggressively, and enforcement can ratchet.

Three pieces:

- **The recognizer (kernel).** `is_canonical` over the core AST —
  one pattern-matching pass, no rewriting, no search. Syntactic
  invariants (§5 C1–C6) are decidable at read; theory invariants
  (§5 C7) need types and gate at check, after the type gate. The
  recognizer NEVER needs termination or confluence — "no rule
  matches anywhere" is pure matching.
- **The rewriter (tools/canon, untrusted).** shardfmt's contract:
  gate-not-printer. The tool rewrites a file into canonical form;
  the kernel recognizer is the only authority on whether it
  succeeded. Every tier-2 step the rewriter takes cites a proven
  requirement — replayable as `rewrite (lemma …)` if we ever want
  certificates for the rewrite itself (we don't, for admission; the
  gate re-judges from scratch).
- **The ratchet (enforcement stages, each a separate decision):**
  1. tool exists; recognizer reports advisory CANON line in check output
  2. std/ tree canonicalized; corpus gate pins it (fmt's arc, replayed)
  3. bin/rebuild.sh stamp refusal (fmt precedent, exactly)
  4. canonical form required for lowering inputs and search corpora
  5. read-time refusal for the syntactic tier (the "deny parse" end
     state — only ever reachable for C1–C6; C7 is check-time by nature)

  v1 proposes stages 1–2 only. Nothing about later stages needs
  deciding now; the point of the ratchet is that it CAN stop anywhere.

## 4. The degrees-of-freedom census (against the grammar)

The core grammar is small enough to census exhaustively. Expr has
NINE formers (kernel/term.shard:102): FVar, BVar, Ctor, Call,
Match(scrut, arms), Let(rhss, body) — multi-binding, parallel scope —
If(c,t,e) — no binders, reduces by True/False — IntLit, SymLit.
Pat has FOUR: PVar (nameless — binds the next BVar), PCtor, PInt,
PSym. Binder NAMES do not exist in the core AST at all; they live
only in surface text and die at elaboration. Match is first-match
with a three-valued matcher. The Bool prim comparison surface is
int_eq / le / lt (no gt/ge, no not/and/or prims).

Per degree of freedom: what varies, whether it carries meaning, the
hazard, and the disposition.

- **D1 byte layout** — no meaning. DONE (shardfmt). Tier 0.
- **D2 proof text** — no meaning given the goal. DONE (regen). Tier 0.
- **D3 binder names** — no meaning (already erased at elaboration;
  BVar/PVar are nameless). Presentation only. PROPOSAL: outside the
  canonical form, unconstrained — names are the corpus-alignment
  surface, like whitespace. Consequence: the content hash (§7) is
  alpha-invariant for free, because it hashes the core AST.
- **D4 match arm order** — MEANINGFUL under first-match when arms
  overlap; meaningless permutation freedom when ctor heads are
  distinct. Hazard: search must enumerate orderings; readers must
  simulate first-match to know if order matters. PROPOSAL C4 below:
  decl-order ctor arms + at-most-one trailing catch-all.
- **D5 dead arms** — an arm subsumed by an earlier arm can never
  fire. Pure noise; hides bugs. PROPOSAL: refused (C5).
- **D6 let structure** — three freedoms: (a) dead bindings (never
  referenced by the body); (b) nested single-binding lets vs one
  parallel multi-binding Let when bindings are independent (both
  spellings exist in the corpus today — std/rng nests, the surface
  form is parallel); (c) binding order within a parallel Let.
  PROPOSAL C3 below. Full let-normal form (ANF — every non-atomic
  subterm named) is a much bigger hammer: named as an OPEN question
  (§11), not proposed for v1; it changes every proof goal's shape.
- **D7 ground redexes** — `(+ 1 2)`, `(le 1 2)`, `(if True a b)`:
  closed prim redexes and decided ifs spell a value the long way.
  Hazard: infinitely many spellings of every literal — the single
  worst DOF for search (the depth-4 wallpaper was mostly this class
  composed with D11). PROPOSAL C1/C2 below. Bounded deliberately:
  prim-op redexes and literal-condition ifs only — NOT general
  closed-call normalization (`(fact 20)` stays; normalizing it is
  running the program, with a fuel story we refuse to open here).
- **D8 ground Nat spelling** — tower `(S (S Z))` vs literal `2`
  (with the S^ sugar in between). The kernel already HAS a canonical
  form: the packed literal (precedent 3; friction #6 resolved the
  typing side). PROPOSAL C6: one source spelling for ground Nats —
  the literal. `(S x)` with x non-ground is untouched.
- **D9 algebraic spelling** — `(+ a b)` vs `(+ b a)`; association;
  literal placement. True ring normalization would quotient hardest
  for arithmetic-heavy search BUT it rewrites the SHAPE of every
  arithmetic goal in every existing proof, and the proof kit already
  dissolves comm/assoc via `by arith`. DISPOSITION: named, NOT
  proposed. Revisit when a consumer (search over arithmetic sketches)
  demands it; expected to enter as a tier-2 rule set drawn from
  kernel/facts' ring laws, not as new machinery.
- **D10 comparison orientation** — `(if (lt a b) X Y)` vs
  `(if (le b a) Y X)`: branch-swap freedom via negation. Real but
  minor (three prims, no `not`). DISPOSITION: census-noted, not
  proposed; a search dialect can quotient it grammar-side.
- **D11 theory-equal spellings** — `(Cons h (append acc Nil))` vs
  `(Cons h acc)`: distinct terms the proven theory identifies. THE
  measured hazard (§1). PROPOSAL C7: theory quotients as data,
  append family first.
- **D12 sugar** — string/list literals, S^, records make/with, `'X`.
  Expansion is deterministic reader-level; the core never sees sugar.
  PROPOSAL: canonical form is defined on the CORE (sugar-transparent);
  the rewriter EMITS maximally sugared surface (readability, corpus
  alignment). Same split as D3: core canonical, surface free — except
  where a sugar and its expansion both appear in SOURCE and D7/D8
  already collapse them.
- **D13 module-level order** — decl order beyond dependency
  requirements, import order. Loader semantics constrain some of it.
  DISPOSITION: deferred to a later slice; census first (what order
  freedom actually exists per decl kind), then propose. fmt already
  pins intra-decl layout, so the blast radius is small.
- **D14 duplicate definitions** — extensionally-equal fns defined
  twice. Not a form question; a CORPUS question. Becomes detectable
  ~for free under §7 (same canonical hash). Consumer of the arc, not
  an invariant in it.
- **D16 basis minimization (the lt pilot)** — when a primitive family
  is closed under a cheap syntactic involution (argument swap, branch
  swap), keep ONE orbit representative: the freedom never EXISTS
  rather than being rewritten away. shard already half-lives by this
  (no gt/ge, no and/or/not). The playground is testing the comparison
  set collapsed to lt: branch-switching absorbs le in if position;
  the VALUE position needs the canonical not-idiom
  `(if (lt b a) False True)` plus one emitter recognizer unit (setge)
  — perf parity holds, the machine keeps its condition codes. The
  BOUNDARY RULE: drop a primitive only when its elimination is
  absorbed by syntax for FREE; keep it when elimination computes (the
  division quartet stays — sign corrections are value-dependent) or
  hides a machine instruction (int_eq stays). Payoff compounds
  through the emitters: a smaller basis = a permanently smaller
  proven-fragment surface per target. RULED 2026-07-10: lt AND le
  both KEPT for now, pending the playground's results; the control-
  layer instance (match on Bool) ruled IN as C9. Symmetric-operand
  order (int_eq/sym_eq) sequences behind §7's hash order.
  MEASURED (playground, later 2026-07-10): the lt-only vocabulary is
  IN and satisfies the boundary rule — dialect cond sets collapsed
  12 → 2 (lt, both argument orders), sort d3 → 6.76×10¹⁵ candidates,
  merge d4 → 1.96×10³⁶, with solution counts UNCHANGED (the tie pair
  re-spells through the branch swap, confirming absorption is free).
  The lt/le KEEP ruling stands for v1 enforcement regardless: a
  le-ban now would create fresh std migration (std sits at zero
  violations) and the value-position price (not-idiom + one setge
  emitter recognizer) touches the emitters mid-x86-arc. Basis
  contraction sequences as its own post-v1 slice, riding the queued
  x86gen simplification. Boundary note: the sound involution is
  le↔lt-negation WITH branch swap; the cond-MIRROR double swap is
  not an equivalence (see §8).
- **D15 arm-local scrutinee respelling** — inside a match arm, the
  scrutinee variable and the arm's pattern name the same value by the
  arm's own equation: `ys` ≡ `Nil` in the Nil arm; `(Cons h t)` — the
  exact rebuild of what was just destructured — ≡ `ys` in the Cons
  arm. First filed under §8 as context-sensitive equality; the
  playground's solution-set census (commit d9c7dbf) showed the
  arm-LOCAL cases are LEXICAL, not flow — the sketch builder cut them
  statically, and they priced at a factor of 4 of insertion sort's 16
  residual true-sort spellings (the census's 2×2 of "nil-arm
  respelling" × "cons-arm rebuild"; quotient depth-stable across
  eight orders of space). PROPOSAL C8 below. The rebuild direction
  also SHARES where the spelling re-allocates — a small performance-
  parity bonus.
- **D17 vacuous control, and the placement taxonomy (playground,
  2026-07-10)** — `(if c X X)` computes X while spelling a branch:
  the census's first equality-shaped SIBLING constraint (a Bool case
  tautology, not a redex — neither C1 nor C2 sees it). Measured on
  whole-body calculator synthesis as LIVE FORK WEIGHT, not output
  noise: excluding the family cut solution regions 1,210 → 14 and
  steps 290,626 → 39,565. The same generation priced WHERE
  sibling-content constraints live: generative content-pair grammars
  bought exactness at memo fragmentation (steps ×7, wall ×6);
  opportunistic join-side pruning bought a strictly larger quotient
  at a seventh of baseline cost. The taxonomy: sharing-preserving
  quotients (leaf filters, index ordering, point exclusions) are
  nearly free generation-side; sibling-CONTENT constraints (equality-
  or order-shaped) belong at the join/recognizer tier — which is
  where §3 already put shard's canon. PROPOSAL C10 for the equality
  case in if position; the order-shaped case (AC operand order) is
  D9/C7 tier-2 territory; the match analog (all arms binder-less and
  identical) is priced, not bought, for v1.
  MEASURED (playground d3bbf0b, the stack machine): at the VM-synthesis
  frontier the §8 context-sensitive-equality classes were a 50-MINUTE
  wall-clock tail (92% of a 10²⁵ space settled in seconds; the rest was
  the engine enumerating spellings of identity in one arm), and the
  pinned-list normal form — C8 COMPOSED through nested destructuring,
  plus the vacuous-match rules — turned the same search into 4.6s.
  Both bought (2026-07-10): C8's rebuild check composes through
  ground-pinned binders, and C10 gained the 'match tier.
- **D18 match-commutation order (playground d3bbf0b, census-only)** —
  independent nested matches commute (the mirror nesting exists with
  rearranged arm bodies); the playground canonicalized inner-scrutinee
  index > outer's and cut its VM space 60×. Carries the first honest
  DEPTH-BUDGET caveat: commuting can push an arm body one level
  deeper, so under a fixed depth budget a solution whose only
  in-budget spelling is non-canonical would be lost (search-side
  concern; source code has no depth budget). Named and priced, NOT
  proposed for v1 — the let-order analog (C3) has a clear first-use
  metric, while match order needs an independence analysis the
  recognizer doesn't yet own. Revisit with the stricter-modes bundle
  (§11).

## 5. The invariant set (proposed v1 cut)

Tier 1 — syntactic, theory-free, recognizer runs at read:

- **C1 no ground prim redexes.** No Call of a prim-table op with all
  args literal. `(+ 1 2)` is refused; write `3`. SCOPE: executable
  positions (fn bodies) ONLY — goals of claims/requirements are
  exactly where distinct spellings are RELATED (`(= (shl 1 12)
  4096)`, the ground differential probes), so goal positions are
  exempt from C1/C2 by construction, not by carve-out.
  RULING (user, 2026-07-10): refuse-at-gate stands; fold-at-read was
  weighed and declined (it re-opens surface multiplicity where the
  corpus lives). The intent-carrying spelling idiom (C's `1<<12`)
  MIGRATES to the claim layer rather than being lost: a named
  constant fn plus a defining claim — `(fn page_size () Int 4096)` +
  `(claim page_size_pow2 … (= (page_size) (shl 1 12)))` — carries
  intent at every USE site through the name, is citable in proofs,
  and cannot drift. What `1<<12` really provided was a derivation
  the compiler checked; shard's checked home for derivations is a
  claim. std/bits already converged on this shape unforced.
- **C2 no decided control.** No `(if True …)`/`(if False …)`; no
  Match whose scrutinee is a literal/ground-ctor value that decides
  an arm. (The residual ground-ctor scrutinee cases ride the same
  matcher the kernel already has; MStuck never arises on ground.)
- **C3 let hygiene.** Every binding referenced by the body (no dead
  bindings); independent adjacent lets merged into one parallel Let
  (maximal merge — nesting is the spelling of DEPENDENCE); bindings
  within a Let ordered by first use in the body.
- **C4 match arm discipline.** Ctor-headed arms appear in ctor
  DECLARATION order; at most one catch-all arm, and only in last
  position; literal arms (PInt/PSym) sorted ascending before the
  catch-all. Arm order thereby stops carrying meaning everywhere a
  reader can see. (Full disjointness — banning the catch-all — would
  outlaw the corpus's pervasive `(_ default)` idiom and explode
  13-ctor matches; named as the stricter mode in §11, not proposed.)
- **C5 no dead arms.** No arm subsumed by earlier arms (a ctor arm
  after a catch-all, a duplicate ctor head, a literal repeated).
- **C6 ground Nats are literals.** The packed form is the source
  form: `2`, never `(S (S Z))`; S^ only with a symbolic base or a
  symbolic count.
- **C8 arm-local scrutinee discipline** (census-driven, ratified
  2026-07-10; applies only when the scrutinee is a VARIABLE — a
  computed scrutinee has no other spelling). Two directions:
  (a) in an arm whose pattern binds nothing (a ground ctor, an Int or
  Sym literal), the scrutinee variable must not occur in the arm
  body — the arm's equation has pinned it; write the value.
  (b) no arm body may contain the EXACT rebuild of the arm's own
  pattern over the arm's own binders — the scrutinee variable already
  names that value (and shares it). Partial rebuilds — any component
  changed — are untouched.
  COMPOSED (2026-07-10, the stack machine's pinned-list normal form):
  the rebuild check carries GROUND PINS through nested destructuring —
  inside `(match xs ((Cons h t) (match t (Nil …))))` the inner Nil arm
  pins t = Nil, so `(Cons h Nil)` there IS the rebuild of xs and is
  refused; every matched value keeps exactly one spelling. Still
  lexical (each equation is an enclosing arm's own — no calls,
  conditions, or paths), so the §8 flow boundary is untouched. The
  nested-Cons side needs no new rule: the inner rebuild is flagged
  one level down and the rewriter's fixpoint composes outward.
- **C9 no match on Bool** (ratified 2026-07-10). `(match b (True X)
  (_ Y))` and `(if b X Y)` are two spellings of the same branch; the
  if is canonical (shorter, binder-free). Detection is syntactic —
  any arm whose pattern head is the core Bool ctor. The first
  application of D16's basis-minimization principle at the control
  layer: shard already has no and/or/not prims (Bool combination IS
  if-spellings); this closes the match-side duplicate.
- **C10 no vacuous control** (ratified 2026-07-10, D17's equality
  case; the 'match tier added after the stack machine measured it).
  Three shapes of control that decides nothing:
  (if) the two branches of an `if` must differ structurally —
  `(if c X X)` is X dressed as control (one expr_eq; If binds
  nothing, so structural equality is alpha-equality);
  (match-constant) a COVERED match whose every arm body ignores its
  binders and agrees is that body;
  (match-identity) a COVERED match whose every arm body respells the
  scrutinee (the shifted scrut var, the arm's ground value, or a
  bare-PVar alias) is its scrutinee.
  Coverage is load-bearing for the match tier: a PARTIAL match is a
  runtime filter, not a vacuous one — only a catch-all or the full
  ctor set makes it decorative (pinned by cp_c10p).

Tier 2 — theory quotients, recognizer runs at check (needs types):

- **C7 no theory redexes, per ratified rule set.** v1 ships exactly
  ONE rule set, the playground-validated append family (all four are
  proven requirements in std/list/mod.req.shard):

      append_nil_left    (append Nil ys)         -> ys
      append_cons        (append (Cons h t) ys)  -> (Cons h (append t ys))
      append_assoc       (append (append a b) c) -> (append a (append b c))
      append_nil_right   (append xs Nil)         -> xs

  A term is canonical iff no rule's LHS matches any subterm AT ITS
  REQUIRED TYPE. The typed side-condition is load-bearing: the
  playground's recorded bug — append_nil_right applied to an
  Int-typed stuck operand certified crashing candidates — is the
  pinned counterexample. Rules fire only where the requirement's
  binder types are satisfied.

What C1–C7 buy, concretely: every consumer that keys on term equality
(the prove solver's candidate spaces, future search executors, the §7
hash, the emitters' shape dispatch) gets "syntactically equal iff
equal up to the instituted quotient" — and the quotient is a LIST OF
CITED THEOREMS, not a compiler's private opinion.

## 6. Rule sets as data (the tier-2 format)

A tier-2 rule set is a declared, ratified object, not code:

    (canon-rules list-append
      (rule append_nil_left)
      (rule append_cons)
      (rule append_assoc)
      (rule append_nil_right))

Each `rule` names a REQUIREMENT in scope; the kernel derives the
oriented rewrite (goal equation read left-to-right) and the typed
side-conditions (the goal's binder types) directly from the checked
requirement. There is no separate rule syntax to trust — a canon rule
IS a citation. Orientation is part of ratification: the declaration
is accepted into the tree by the same review that accepts an axiom
placement (human-gated, tiny surface).

Obligations, split honestly:

- **Recognizer: none.** Matching LHS shapes needs neither
  termination nor confluence.
- **Rewriter: termination.** The tool must halt. v1: hand-ratified
  per rule set (the append four are the textbook completion example).
  v2 (named growth): a meta tool checking a termination order +
  critical-pair convergence, so rule-set ratification becomes
  machine-assisted.
- **Metatheory: confluence = uniqueness.** Without it "canonical" is
  still a well-defined predicate (no rule applies), just not a unique
  representative per class. Search needs only representative
  EXISTENCE (termination + rules-are-proven-equalities). Uniqueness
  is what §7 needs — hash-equality should mean theory-equality — so
  confluence is REQUIRED for any rule set admitted while content
  addressing is live. The append four are confluent.

Growth law: one rule set per proven lemma family, each bought
exactly when a consumer demands it. The boundary stays legible — the
playground measured this directly: raising the case-split budget
moved ZERO verdicts once the append theory was quotiented; what
remained needed match-context reasoning no equational lemma states.
Each family buys its exact quotient, no more.

**The certificate taxonomy (from the playground's second theory,
commit f76845a).** Three kernel-replayable license categories for
canon steps, and rule sets may draw on any of them:

- *definitional* — constant folding IS computation (C1's category);
- *lemma-cited* — each rewrite names a checked requirement (the
  append four; the `canon-rules` form above);
- *decision-procedure-backed* — each rewrite is a tautology of a
  kernel decision procedure (the lia arithmetic normalizer: fold
  constants, drop zeros, order operands, right-nest — every step a
  `by lia` the kernel would accept). std/arith self-describes as
  "lia tautologies used to reconcile term shapes the reducer doesn't
  canonicalize" — the scar tissue of the missing canonicalizer, and
  the standing measure of what this category is owed.

**The escape-rule criterion (the typed-lemma lesson, generalized on
first contact with a second theory).** A rule that REBUILDS a spine
keeps every operand under its original forcing context and is
fail-consistent for any operand; a rule that lets an operand ESCAPE
its context (`append_nil_right`'s bare `xs`, zero-elimination's bare
operand) requires the typed side-condition — the operand must be
known to inhabit the binder's type. When ratifying a rule set, sort
its rules by this criterion first: spine-preserving rules are safe by
shape; escape rules carry the type obligation.

## 7. Content addressing (ratified direction)

Ratified 2026-07-10: shard should adopt and exploit content-addressed
definitions. The Unison precedent probed this region and found humans
could barely bear it; nothing in OUR authorship model objects.

What it is here: a definition's identity is the hash of its CANONICAL
core term, binder-nameless (the core AST already is — D3), with every
Call/Ctor QName reference replaced by the referent's own hash
(Merkle). Two definitions have equal hashes iff their canonical forms
are structurally identical up to naming — and, because hashing sits
downstream of C1–C7, up to the instituted quotient. Canonicalization
is what makes the hash MEANINGFUL; content addressing is what makes
canonicalization PAY compositionally:

- **Cert and proof reuse.** Certificates keyed by hash survive
  renames, module moves, and duplicate definitions. The cert economy
  (lowering certs, callee certs, CK chains) becomes keyed by what a
  function IS, not where it lives or what it's called.
- **Duplicate census.** D14 for free: same hash, same function —
  reported, not refused.
- **Search memo keys.** The playground's "the memo is the whole
  game": hash-consing is content addressing at executor scale; a
  corpus-level hash gives cross-RUN memoization a sound key.
- **Incremental checking.** A definition whose hash is unchanged has
  an unchanged meaning; recheck nothing beneath it. (The checker's
  future scaling story, named here, built later.)

v1 scope, deliberately thin: DEFINE the hash (spec below, ratified
2026-07-10 — §11.5 resolved) and ship a tool that computes and lists
it. KEY NOTHING ON IT YET. The first real consumer should be chosen
the way all our consumers are — by demand; the cert cache is the
likely candidate. What v1 must get right is only the constraint that
already binds: the hash is over the canonical nameless core, so the
canonical form must be stable before any hash is stored anywhere.

### The hash spec (v1, ratified 2026-07-10)

**The serialization is the stable contract; the digest is a
replaceable parameter.**

SERIALIZATION. A definition's identity bytes are a tagged,
constructive encoding of its elaborated canonical nameless core —
signature and body for a fn; ctor field types for a typedef. One tag
byte per former; integers as sign + decimal digits (bignum-safe);
child lists count-prefixed. References fall in three classes:

- **core-pathed names** (prims and (core)-pathed formers — a closed
  vocabulary per docs/TCB.md) serialize as themselves: terminal
  symbols, never hashed;
- **out-of-SCC references** are replaced by the referent's DIGEST
  (Merkle) — so identity is compositional: a definition's hash pins
  the full meaning of everything it reaches;
- **within-SCC references** (self/mutual recursion) serialize as the
  referent's index in the SCC's canonical member order.

The reference graph is ONE graph over fns and typedefs (fn→fn calls,
fn→type signature/ctor refs, type→type field refs); SCCs are hashed
bottom-up along the condensation. The canonical member order inside
an SCC: sort members by their serialization with all same-SCC refs
as a FIXED PLACEHOLDER tag; a member's hash is then
digest(member-tag ++ scc-digest ++ member-index). Residual: members
whose placeholder serializations are byte-identical (structurally
identical mutual twins) tie-break by declaration order — the one
place decl order can reach a hash, documented and vanishingly rare.
Type variables in signatures serialize by first-occurrence index
(alpha-invariant); ctor references serialize as (typedef-ref, ctor
declaration index) — ctor NAMES are presentation, like binder names.

DIGEST. std/sha256 over the serialization (SWAPPED IN 2026-07-10,
same day as the spec: the sha256-std side-arc landed FIPS 180-4
SHA-256 with a proven 32-byte digest-length contract and NIST-vector
compute pins, and hx_digest now folds its 32 bytes into the digest
Int). v1 briefly shipped FNV-1a-128 — dispersive but trivially
invertible, licensed for dedup only — and the swap invalidated no
stored state because v1 stored none: exactly the replaceability the
spec promised. The digest remains a parameter behind hx_digest.

TOOL. tools/canon/hash.shard prints one `<digest-hex>  <qname>` line
per fn and typedef of the target module (production loader; whole
closure hashed so Merkle refs resolve). Definitions the recognizer
flags are marked `!` — the hash is only MEANINGFUL on canonical form
(stage 2 guarantees it for std). The corpus pins the DIGEST-STABLE
properties only (determinism; alpha-twins hash equal; distinct fns
hash apart), so the digest swap touches no goldens.

## 8. Named exclusions (tier 3 — the honest boundary)

Excluded by decision, not accident:

- **Context-sensitive equality — the FLOW-dependent kind.** The
  census sharpened this boundary (see D15): equalities pinned by the
  ENCLOSING arm's own equation are lexical and moved IN as C8; what
  stays out is equality that needs reasoning across calls, through
  conditions, or along paths — anything the arm's pattern does not
  spell directly. OUT.
- **Extensional variation.** The census's cleanest specimen: `le` vs
  `lt` as insertion sort's tie-break — the SAME function computed
  differently on equal keys. No spelling quotient can touch it; it
  is laws/oracle territory (behavioral fingerprinting, requirement
  proofs), permanently. OUT.
- **The cond-mirror family** (playground-pinned 2026-07-10).
  `(if (lt a b) X Y)` vs `(if (lt b a) Y X)` looks like an
  involution orbit but is NOT an equivalence: on ties both conds are
  false, so the first computes Y and the second computes X. The
  sound D16 involution negates the comparison AND swaps branches
  (`le a b` ⟺ ¬`(lt b a)`); the double swap of the SAME comparison's
  arguments plus branches differs exactly on ties — the tie pair
  above is the extensional floor, and this is its spelling-side
  shadow. OUT, permanently.
- **Recursion shape.** Accumulator vs direct recursion, fold vs
  explicit structural descent — extensionally equal, structurally
  incomparable. Program equivalence, not spelling. OUT.
- **Helper decomposition / inlining freedom.** Where one cuts a
  function into two is a design act, not a spelling. OUT. (Content
  addressing makes the DUPLICATE case visible — that much and no
  more.)
- **General closed-call normalization.** `(fact 20)` → literal is
  running the program at read time; fuel and cost stories we refuse
  to import into a recognizer. Prim redexes only (C1).

## 9. Verification

- **The exactness census (the playground's `--canon-verify`,
  graduated).** Enumerate all terms to a size bound over a small
  signature; check the recognizer's image equals the rewriter's
  normal-form image term-by-term, and that every equivalence class
  keeps exactly one representative. This is the gate that catches a
  recognizer/rewriter drift — the pair must define the SAME form.
- **Corpus gates.** A canonical-sweep target (recognizer over the
  std tree once stage 2 lands); negative fixtures per invariant
  (canon_rejects.shard: one violation of each of C1–C7, each must be
  refused with a named message — the percolation_rejects pattern).
- **Cross-implementation agreement.** The recognizer is kernel shard;
  the rewriter is a tool. No Rust twin to drift against; the
  playground remains a differentially-gated accelerator on its own
  corpus subset, never an authority.
- **Proof-neutrality probes.** Canonicalizing a module must leave its
  claims checkable — pin with one proof-bearing module rewritten by
  the tool and rechecked green before any tree-wide sweep (the
  records-arc proof-neutrality pilot, replayed).
- **Two-level fingerprinting (the census's lesson, commit d9c7dbf).**
  Any census over "equivalent" artifacts needs fingerprints at TWO
  levels of the artifact (the playground: the sort entry AND the bare
  insert) — the first battery lacked the one input that separates
  true inserts from impostors, and the two levels DISAGREEING is what
  exposed it. Silence is not success; a single fingerprint cannot
  audit its own battery. Corollary for oracle strength: the 32
  surviving impostors were the TEST SET's weakness, not duplication —
  requirement proofs (the laws oracle) are the standing fix.

## 10. Migration

- **Order: std first** (smallest, most cited, owns the append
  requirements), then examples, then kernel sources last — kernel
  files are the densest proof-site real estate.
- **Proof-site churn is the real cost.** Rewriting a measured fn's
  body shifts sidecar sites (the known gotcha); the prove regen
  machinery absorbs sidecar entries, but inline measure proofs and
  hand proofs over reshaped goals need eyes. Price per tree, not per
  arc; the ratchet exists so this never has to happen in one sweep.
- **Transition multiplicity is friction-#6 territory by
  construction.** While canonical and legacy spellings coexist,
  bridges (typing views, defining-equation lemmas) carry the load.
  Keep the window short per tree: canonicalize, recheck, pin, move on.

## 11. Open questions (for ratification, none blocking §4–§5 review)

1. **ANF depth (D6).** Does canonical shard eventually name every
   non-atomic subterm? Massive goal-shape churn; real sharing and
   emitter benefits. Not in v1; wants its own evidence pass.
2. **Strict match mode (D4).** A no-catch-all, full-disjointness mode
   (match as ctor map) — as a per-type or per-module opt-in ratchet
   beyond C4? The 13-ctor corpus idiom argues against forcing it.
3. **Ring normalization (D9).** Enters as a tier-2 rule set from
   kernel/facts when a consumer demands it — or never; `by arith`
   already dissolves what it would pin.
4. **Module-level order (D13).** Census first; likely a later slice
   of the same recognizer.
5. **Hash algorithm + spec (§7).** Settle after C1–C7 stabilize; the
   spec lands in this file.
6. **Tool naming/home.** tools/canon vs growing shardfmt (fmt = the
   byte layer, canon = the term layer; they compose but are distinct
   passes with distinct authorities). Lean: separate tool, shared
   gate discipline.


## 12. Implementation record

**Slice 1 (2026-07-10, commit 389d877): the C1–C6 recognizer + the
stage-1 advisory.** kernel/canon.shard walks fn BODIES of the target
file's module (goal positions exempt by construction — never
consulted); the driver appends one count-free COCanon outcome (the
COLedger shape) rendering `CANON <fn>: C<k> <subject>` per violation,
via canon_note at run_srcs' target entry, so a violation is stated
once, at its home module. Nothing fails; no exit code changes.

Facts the slice pinned:

- **C1 is drift-proof by construction**: the recognizer consults
  try_step_prim itself — a ground prim redex is exactly a core-pathed
  Call whose literal args make the reducer's own table step. Excluded
  from flagging: gen_fresh (effectful — folding it would be WRONG, not
  merely non-canonical) and refine_val (a typing coercion).
- **The Let binder convention** (needed by C3's first-use walk):
  the elaborator assigns BVar indices innermost-first — source binding
  i = BVar n-1-i in the body, the same convention as parameters
  ("BVar 0 = the LAST parameter"). Pinned empirically by
  examples/canon_pin.shard's cp_pair. The Let node STORES bs in SOURCE
  order (reader.shard elab_let); open_many's bindings argument is
  index-ordered, so every opener must REVERSE first, as apply_fn does
  for call args. (An earlier version of this note claimed the stored
  list was index-ordered — false, and the proof-mode reducer's Let
  openings shared the error: they opened unreversed, silently swapping
  parallel-let bindings under compute. Found by the SHA-256 build,
  fixed 2026-07-10, pinned by examples/parlet_pin.shard.)
- **Self-application is honest**: checking kernel/canon.shard reports
  its own C4 arm-order violations (Call-before-Ctor in cn_e, Some-
  before-None). Advisory, correct, and burns down when the kernel
  tree migrates (§10 puts kernel last).
- Pins: canon_pin (canonical bodies, ZERO lines; the goal-exemption
  claim `(= (+ 1 2) 3)` passes unflagged while the same shape in a
  body flags) and canon_rejects (one fn per invariant, 13 lines,
  exit 0). run_corpus gained both targets plus a tower-run gate that
  emits FAIL lines on drift, so the FAIL-set diff enforces the pins.

**Slice 1b (2026-07-10, commit faa4617): C8 joins the recognizer.**
The census-ratified arm-local scrutinee discipline, both directions,
variable-scrutinee only. The pattern-binder convention needed its own
empirical pin (the Let lesson repeating at the pattern layer): a
pattern's k-th DFS PVar = BVar nb-1-k at arm-body top — innermost-
first, as parameters and lets — arbitrated by cr_c8b's two-binder
rebuild matching on the first try. Non-flags pinned: partial rebuilds
(any component changed) and whole-scrutinee reuse in a binding arm,
which is the spelling C8 steers toward. Slice-1 validation on the
first rebuilt engines: corpus FAIL-set identical, both canon gates
green, sweep 25/25, and the corpus-wide advisory footprint measured
at 255 CANON lines under C1–C6.

**Slice 1c (2026-07-10, commit 6f01d20, validated post-merge on stamp
7562f67c543c): C9 joins the recognizer.** Match-on-Bool detection is
one arm-scan for a core Bool ctor head, flagged once per match. The
measured organic footprint corpus-wide: 6 sites (plus the 2 deliberate
rejects lines) — the collapse was already the corpus's idiom. Full
final measurement under C1–C9 (231 targets): C6 198 (the emitter
towers, unchanged), C4 28, C8 22, C3 10, C1 9, C9 8, C5 6, C2 2.
Validation battery: corpus FAIL-set = the justified post-#16 baseline
(the canon fixtures contributed exactly their deliberate lines), canon
gates green (17 rejects lines), sweep 26/26 (the WORD-fragment build
joined the roster).

**Slice 1d (2026-07-10): C10 joins the recognizer.** The playground's
opportunistic-pruning generation (D17) landed three doc amendments —
the D17 census entry with the generation-vs-join placement taxonomy,
the D16 lt-only measurement (boundary rule satisfied; KEEP ruling
stands for v1, basis contraction sequences post-v1), and the §8
cond-mirror negative pin — plus the C10 check itself: one expr_eq of
an If's two branches (If binds nothing, so structural equality IS
alpha-equality). cn_code_chars grew two-digit rendering (code 10
previously rendered as `C:`). Pins: cp_c10 (deep near-miss branches,
unflagged) and cr_c10 (18 rejects lines; gate roster + `C10 vacif`).
Measured: std/ at ZERO violations under C1–C10 (tower scan, all 12
std targets) — stage 2 stays pin-only.

**Slice 2a (2026-07-10): the tools/canon REWRITER.** Two files:
tools/canon/rewrite.shard (the pure CST→CST core) + canon.shard (the
CLI: kernel-loader closure resolution, facts scan, fmt-gated print).
Architecture as ratified in §3, with these implementation rulings:

- **Substrate = meta/format's CST** (comments/blanks/raw spellings
  survive; output pipes through fmt, so it is simultaneously
  canon-rewritten and shardfmt-canonical). Patterns, quotes, and goal
  positions are never entered — same scope as the recognizer.
- **v1 rewrite set**: C1 (folds through try_step_prim + the
  recognizer's own cn_prim_flaggable — parity by construction), C2
  (both forms; nat-view matches refused), C4 (decl-order sort via a
  closure-wide typedef scan; unique-owner or refuse), C5, C8 both
  directions, C9, C10. **Refusal tier: C3** (let hygiene — slice 2b)
  **and C6** — the type gate's tc_nat_lit_view fires at argument
  positions but NOT at the body-vs-declared site, so a folded literal
  in return position fails to type (found by the rejects roundtrip:
  `(fn f () Nat 2)` refused Int-vs-Nat). The C6 fold unlocks when the
  view covers every Nat-expected position — QUEUED kernel companion
  fix, canon-owned.
- **Conservative-refusal catalogue**: shadowed names (closure scan;
  kernel files' typedefs ARE the core vocabulary, never shadows),
  comment-bearing deletions, inter-arm layout on reorders, binder
  rebinding under C8 substitution, unknown pattern atoms.
- **THE SELF-APPLICATION LESSON (a §9 argument written in downtime):**
  the tool's first self-canonicalization CORRUPTED its own source —
  the facts scanner missed parametric typedefs (`(type (List T) …)`),
  so `Nil` patterns classified as NAMED CATCH-ALLS and C5 "pruned"
  the live `(_ …)` arms after them, in eight functions. The checker
  stayed GREEN (52/0) — arm deletion is invisible to the type gate —
  and the corruption surfaced only as run-mode stuckness on inputs
  that forced the damaged paths. Two fixes: parametric names scanned,
  and unknown pattern atoms now classify as UNKNOWN (refusing all
  match-level rewrites) rather than as binders. The episode is the
  census's thesis made concrete: a rewriter bug ships silently unless
  something re-judges the pair — slice 2b's exactness census is not
  optional polish.
- Validated: rejects-file roundtrip re-checks with EXACTLY the 6
  refusal lines (3×C3 + 3×C6); pin file is a byte-identical fixed
  point; the tool is idempotent on and self-canonical over its own
  two files (ZERO advisory lines); both files check 52/0. run_corpus
  gains both targets + two gate pins (pin-identity, roundtrip).

**Slice 2b (2026-07-10): the exactness census
(tools/canon/census.shard).** §9's `--canon-verify`, graduated, with
one structural upgrade over the playground: no mini-elaborator — the
census synthesizes ONE module holding every enumerated candidate and
judges it through the PRODUCTION pipeline on both sides (recognizer =
kernel/canon's cn_e over a build_module_r-built module on the real
stdlib closure; rewriter = the tools/canon core on the file's CST),
so the census has no translation layer to drift in. Enumerated
domain: 102 typed fn bodies over `b:Bool, n:Int, xs:(List Int)`
covering the fixed tier — every if over 5 conditions × 4×4 bodies
(C2 decided conds incl. the C1→C2 fold cascade, C10 equal branches),
Bool matches both orders (C9×C4), int-literal arm orders + dups +
after-catch (C4/C5), ground-scrutinee matches (C2), standalone prim
redexes (C1), and List-ctor matches with respell/rebuild/reuse arms
(C8 both directions, C4 ctor order, C5). Excluded by design: the
refusal tier (C3/C6) and partial matches (runtime-stuck programs;
refusal is correct there). Asserted: A1 rewriter image is
recognizer-clean; A2 admitted ⟺ fixpoint, per term; A3 file-wide
idempotence. Result: **OK — 102 terms, 74 flagged→fixed, 28 admitted
unchanged** (the flagged count prints in the OK line so a vacuous
census is visible at a glance). Wired into run_corpus's canon gate;
census.shard is itself self-canonicalized and checks 52/0.

**Slice 3 (2026-07-10): the C7 tier + the tc_nat_lit_view companion
fix.** Two kernel edits, one rebuild:

- **tc_nat_lit_view at the RETURN position** (kernel/types.shard):
  tc_check_fn now applies the same literal view per-argument checking
  always had, so `(fn f () Nat 2)` types — the canonical C6 spelling
  is legal everywhere the tower was. Value-aware only: Int VARIABLES
  still never coerce (natview_rejects unchanged); pinned by
  examples/natview_pin2.shard (incl. the goal-position idiom: goals
  spell Nat towers freely — they are C6-exempt and the goal type gate
  wants Nat=Nat). NOT a bidirectional-checking change: a literal at a
  branch/arm JOIN still synthesizes Int and fails a Nat return — the
  view sees the literal only where the literal IS the checked term.
- **C6 leaves the tool's refusal tier, POSITIONALLY**: the rewriter
  folds ground towers only at argument positions and the whole fn
  body (an `ap` flag threaded through the walker) — exactly where the
  view guarantees the folded literal types. Towers under if branches,
  match arms, or lets stay advisory; folding them would trade a
  well-typed tower for an Int-vs-Nat refusal.
- **C7 joins the recognizer** (kernel/canon.shard cn_c7): the four
  append shapes, keyed by QNAME to std/list's own append (resolution
  already happened at load) — nil_left / cons / assoc / nil_right,
  each line naming the proven std/list requirement being refused. The
  §6 typed side-condition is discharged by placement: by the time a
  CHECKED module's advisory matters, the type gate has vetted the
  operands; and C7 stays check-time forever (never joins a read-time
  refusal stage). The REWRITER does not apply theory rules in v1 —
  C7 replaces C6 in the roundtrip's refusal tier (3×C3 + 4×C7 = 7).
- Validated: rejects grows cr_c7a–d (22 advisory lines), pin grows
  cp_c7 (a rule-free append stays clean) and remains a rewriter fixed
  point; census grows the Nat whole-body family — 108 terms, 78
  flagged→fixed, biconditional green with the C6 fold live.

**Slice 3b (2026-07-10): C8 composed + C10 'match — the stack
machine's rules, bought.** Playground d3bbf0b measured the §8
context-sensitive-equality classes as a 50-minute wall-clock tail and
its pinned-list normal form as the cure; both halves that are LEXICAL
moved in:

- **C8 composes through ground pins** (recognizer: cn_rbp/cn_rbs
  thread a pin context; tool: cw_subst_pin, a pin-aware substitution)
  — `(Cons h Nil)` under a nested Nil arm is the rebuild of xs and is
  refused/rewritten. The nested-Cons side was already covered: the
  inner rebuild flags one level down and the rewriter's fixpoint
  composes outward (validated: cr_c8c collapses to `xs` through
  composed-rebuild → constant → identity in one canon run).
- **C10 'match** (cn_c10m; cw_rule_c10m): constant and identity
  matches, coverage-guarded (catch-all or full ctor set — cp_c10p
  pins that a PARTIAL identity-shaped match is a filter, not
  vacuous). Two prior pin fns were exposed as genuinely vacuous by
  the new rule and were repaired — the recognizer catching its own
  fixture file is the system working.
- **Organic footprint: ZERO** — std (all 12 targets) and the tool's
  own three files show no new violations under the composed rules;
  stage 2 stays pin-only. D18 (match-commutation order) entered the
  census as named-not-proposed, with its depth-budget caveat.
- Fixtures: rejects 25 lines (cr_c8c, cr_c10m, cr_c10i); pin grows
  the composed near-miss (literal head ≠ binder) + cp_c10m/cp_c10p;
  census 115 terms, 84 flagged→fixed, incl. the full-cascade term
  and the constant/identity near-misses.

**Slice 4 (2026-07-10): STAGE 2 — the std/ tree canonical, pinned.**
The ratchet's second notch. Three parts:

- **The join views complete friction #6's family** (kernel/types.shard):
  tc_nat_lit_view now fires at the IF-branch join and the MATCH-arm
  join — the latter via a two-pass tc_arms (elaborate all arms, join
  non-literal arms first, then literal arms unify-or-view), because
  rty is a fresh meta and C4's canonical order puts literal arms
  FIRST: a one-pass join would pin the meta to Int at `(Nil 0)` and
  refuse the `(S …)` arm that follows, exactly backwards. Covered
  positions now: arguments, returns, if branches, match arms —
  `(match xs (Nil 0) ((Cons _ t) (S …)))` types in either arm order
  (natview_pin2 pins all four join cases; Int VARIABLES still never
  coerce). The tool's C6 fold widened to the join positions; only
  LET positions remain advisory (a binder's type is inferred from
  its RHS, and a let body is opaque to the view).
- **The last organic residue fell to the tool**: the full-tree scan
  found three violations outside the corpus targets — two `Z`-in-arm
  sites in the mod.build plans (untypeable as `0` until the join view
  landed) and one C8 rebuild in std/str/str.wasm.shard. Each fixed by
  a one-line tools/canon rewrite; **str.wasm.shard is cert-bearing
  and its 51 claims recheck green — the §9 PROOF-NEUTRALITY PROBE,
  passed on a real lowering-cert module.**
- **The stage-2 pin** (run_corpus canon gate): every std source —
  impl files, wasm/x86/rep siblings, mod.build plans, mod.req
  interfaces (.auto sidecars and derived .low files excluded) — must
  produce ZERO CANON advisory lines. Measured at zero across all 26
  files; regressions fail the corpus.

Census: 117 terms, 86 flagged→fixed (join-position towers added).
Enforcement stages 1–2 are now both LANDED; the v1 ratchet's stated
scope is complete.

**Slice 5 (2026-07-10): the content address — spec + tool + pin.**
§11.5 resolved; the spec is in §7. tools/canon/hash.shard computes it:
production loader (run-mode closure), one reference graph over fns and
typedefs, Kosaraju SCCs, fixpoint Merkle hashing bottom-up along the
condensation, the digest behind the single hx_digest swap point
(FNV-1a-128 at landing; std/sha256 swapped in later the same day —
the pins moved without edits, as designed). examples/hash_pin.shard +
the corpus pin exercise exactly the digest-stable properties:
alpha-twins hash EQUAL (names and binder names are presentation),
distinct definitions hash apart, and the Merkle showpiece —
hp_calls_a/hp_calls_b call DIFFERENT twins yet hash EQUAL, because
Merkle substitution replaces both references by the referents' equal
digests: identity is what a definition MEANS. Implementation findings:
a directory impl's public fns rebind to the INTERFACE's module path
(std list, one segment above the impl file's own std/list/list), so
the tool's target filter is prefix-based; stdlib's typedefs are
core-pathed and serialize terminal like prims.

**The v1 arc is COMPLETE**: C1–C10 recognized, machine-rewritten
(C3/C7 refusal-tier), censused, std at stage 2, and content-addressed
— all under the corpus gates. Post-v1 queue (each its own decision):
the §13 contextual-normality proposal (drafted 2026-07-10, the
catalog measurement); D16 basis contraction riding the x86gen
simplification; C3 rewriting (2b's deferral); D18 and the §11
stricter modes as evidence arrives. (The digest swap to std/sha256
landed 2026-07-10, same day as the spec.)

## 13. Contextual normality — C11/C12 (post-v1 tier)

STATUS: RATIFIED 2026-07-11 (user: "agreed on this scope proposal") at
the draft's positions on D19–D22. Slice records and the
implementation-discovered amendments at the end of this section.

PROVENANCE (the playground's catalog arc, measured 2026-07-10). The
canonical-program census enumerates EVERY canonical program of the
structural list fragment per depth and brackets the distinct
functions among them: d1 = 19 programs / exactly 13 functions,
d2 = 7,790 / [1,068–1,160], d3 = 653,491,008 / ≥ 4.45M. Spellings
per behavior: 1 → 1.5 → 7 → 147 — the redundancy the v1 rules do
not name, compounding per rung. Classifying the full spelling sets
of the named d2 buckets: **85% of the post-dialect space is
contextually-provable respelling** in exactly two families, and the
two rules below would take spellings-per-behavior from ~7 to ~1.09.
The residue (different algorithms, one function — `(append xs xs)`
vs its single-pass twin) is genuine semantics: the induction prover
closes those, a syntactic canon never should.

### C11 contextual partial evaluation (proposed)

Inside a match arm, the arm's equation — scrutinee ≡ pattern
instance — is a hypothesis, and C8's pins already carry the GROUND
tier of it. C11 generalizes the pins from ground values to pattern
SHAPES (ctor spines with symbolic binder fields) and the judgment
from "is the rebuild" to "can take an evaluation step":

    A body subterm that can take a gated evaluation step UNDER THE
    ARM-HYPOTHESIS PINS is a contextual redex. Canonical form is
    contextually normal.

The three step kinds, in the license's own terms:
- **decided control under pins**: a match/if whose scrutinee/cond is
  pinned to a deciding shape fires (C2 becomes the empty-pin-set
  special case);
- **ground folds under pins**: a prim/packed-Nat redex whose args
  are pinned ground folds (C8a-respell becomes the corollary: the
  pinned occurrence IS the fold's argument — C1/C6 at empty pins);
- **gated unfolds under pins**: a user-fn call unfolds ONLY when the
  unfolded body immediately head-fires under the pins — exactly
  step_smart's δ gate, evaluated with pins. This is the family the
  census measured as the bulk: `(f t)` in an arm where t ≡ Nil is
  `(f Nil)`, a decided call; spelling the call is spelling a redex.

LICENSE: definitional — every C11 step is a step of the kernel's own
reduction relation instantiated at the arm hypothesis, the same
license as C1/C2 (and the recognizer consults the kernel's step
machinery, not a private table). Composition with C8b is the
already-landed pattern: C11 reduces `(f t)` → Nil under t ≡ Nil,
then C8b's composed-pin fold takes `(Cons h Nil)` → xs; the pinned
normal form stays "every matched value has exactly one spelling",
now closed under evaluation as well as rebuilding.

TERMINATION: steps consume pin ctor-layers (finite, lexical) or fold
ground redexes (measure-decreasing); δ is gated on immediate
head-fire, the same discipline that keeps step_smart from chasing
recursion. The rewriter's obligation is a contextual normal form per
arm; the recognizer's is one boolean per body (does ANY subterm
step).

COST (open, measure at census): the recognizer runs on every checked
body. Step-with-pins is bounded by the same gate that bounds simp;
the census slice must record wall-time delta on the std tree before
stage-2 is even discussed.

### C12 no needless case splits (proposed, rides C11)

A COVERED match whose arms, after C11-normalization under each arm's
own pins, are IDENTICAL up to the arm equation (each body is the
same expression with the arm's specialization applied) is that
expression: the split decided nothing the body used. v1 proposes
only the mechanically-safe tier: all arm bodies contextually
normalize to the SAME term once the scrutinee variable and binder
occurrences are folded through the pins — the C10-match-constant
rule lifted from syntactic equality to contextual-normal equality.
Full anti-specialization (reconstructing E from its specializations)
is named and NOT proposed.

### Slice ladder (each gated as v1's were)

1. **Recognizer** (kernel/canon.shard): shape pins + step-with-pins,
   advisory CANON lines; pins + rejects fixtures; corpus FAIL-diff
   at the 57-line baseline.
2. **Census** (§9 extension): generated contextual-redex terms both
   judged and rewritten; assert image-clean/fixpoint/idempotence;
   record recognizer wall-time delta on the std tree.
3. **Tool tier** (tools/canon/rewrite.shard): cw shape pins +
   fixpoint normalization; self-canonical; roundtrip gate.
4. **std scan → stage-2 re-pin** only after the census says the tree
   is near-zero and the proof-neutrality probe (a cert-bearing file
   whose claims recheck green after rewriting) passes.

### Decision points (for ratification)

D19. δ scope: full gated-unfold tier in v1 of C11, or land decided
     control + ground folds first and add δ after the census prices
     it? (The census says δ is the bulk; the draft proposes full.)
D20. Ledger shape: C11 as the general rule with C2/C8a annotated as
     its empty-pin/ground corollaries (kept, implementation shared),
     or restated-and-absorbed? (Draft proposes: kept as corollaries
     — the tight-general-rule taste, no re-litigating landed pins.)
D21. C12 in the same slice group or queued behind C11's census?
     (Draft proposes: same group, it is one comparison once C11
     normalization exists.)
D22. Goal positions stay exempt (unchanged from v1 — goals are where
     spellings are RELATED). Draft treats this as settled, listed
     only for the record.

### §13 slice 1 (2026-07-11): the 'resplit recognizer — LANDED

The decided-control tier, implemented as SHAPE PINS: kernel/canon's
main walk now threads two pin environments (FVar-keyed for fn-param
scrutinees, BVar-keyed for binder scrutinees — cn_ep/cn_lp/cn_abp;
cn_e is the unchanged external wrapper). Every ctor/literal arm over
a variable scrutinee pins it to the pattern's shape (cn_pat_shape:
k-th DFS PVar = arm binder BVar nb-1-k, the parameter convention);
values shift with depth (cn_pins_shift now shifts values — identity
on C8's ground pins). The check (cn_c11): a match whose scrutinee is
pinned, where the BARE variable does not fire but the PIN-EXPANDED
shape does (cn_px: one substitution pass per pin bounds the
depth-ordered chase), flags `C11 resplit`. Firing rides the kernel's
own three-valued try_match_arms — an arm the shape leaves UNDECIDED
is a runtime filter, no flag (pin cp_c11b); a match the bare
variable already fires is C5/C10m territory, not a re-split.
Fixtures: cr_c11a (direct re-split), cr_c11b (fires only through the
COMPOSED shape (Cons h (Cons a b)) — deep-pattern composition),
cp_c11a (re-matching the arm's own binder is honest), cp_c11b (the
MStuck filter). Roundtrip refusal tier grows to C3/C7/C11 (9 lines)
until slice 3 teaches the tool shape pins.

### Implementation-discovered amendments (recorded at slice 1)

- **The 'if tier is DROPPED**: single-step, an if-condition can only
  become True/False through a Bool pin, and Bool pins arise only from
  match-on-Bool — C9-flagged spelling. In C9-clean code the case is
  unreachable; multi-step decidings route through the fold tier.
- **D19a (OPEN — the fold tier decomposes; needs a ruling).** Working
  the rule revealed the census's fold family splits three ways in
  shard's ledger: (i) pin-derived ground folds are C8a's REWRITE
  IMAGE — the occurrence is already flagged at the variable, and
  folding is what the C8a rewrite produces — no new rule needed;
  (ii) SOURCE-ground user-call folds (`(len Nil)` spelled in a body)
  collide head-on with the C1 ruling's named-constant idiom (`(fn
  page_size () Int 4096)` + `(page_size)` at use sites is canonical
  BY DESIGN) — folding ground calls needs a C1-style user ruling on
  where names stop and redexes start, plus an evaluation-at-gate cost
  story; (iii) shape-pin δ (unfold when the body head-fires on a
  partially-symbolic shape) canonicalizes toward INLINED CALLEE
  BODIES — `(sum xs)` inside xs's own Cons arm would rewrite to sum's
  arm body — duplication the playground's evidence does not support
  (its measured family was ground-decided calls only). Proposed
  resolution: (i) closed by C8a, (iii) rejected, (ii) = D19a, its own
  decision. The census slice (slice 2) should measure how often
  source-ground user calls actually occur before D19a is argued.

The catalog's raw-grammar twin measured that canon rules are
DIRECTED: a behavior's canonical spelling can sit one rung deeper
than its shortest raw spelling — and exactly one (raw-depth-d
behaviors all reachable canonically at d+1; 0 lost at both measured
rungs, at 8,930× fewer d2 forms and ~10¹⁴× at d3). Two standing
consequences: **"size of the canonical form" is the size metric**
for any budgeted tooling over canonical shard (search executors,
catalog consumers, §7-hash-keyed stores), and a canonical-only
dialect must budget the extra rung. This generalizes the §4
match-commutation depth caveat from edge case to norm.
