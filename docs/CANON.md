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
- **C9 no match on Bool** (ratified 2026-07-10). `(match b (True X)
  (_ Y))` and `(if b X Y)` are two spellings of the same branch; the
  if is canonical (shorter, binder-free). Detection is syntactic —
  any arm whose pattern head is the core Bool ctor. The first
  application of D16's basis-minimization principle at the control
  layer: shard already has no and/or/not prims (Bool combination IS
  if-spellings); this closes the match-side duplicate.
- **C10 no vacuous if** (ratified 2026-07-10, D17's equality case).
  The two branches of an `if` must differ structurally: `(if c X X)`
  is X — the Bool case tautology — dressed as control. Nameless
  syntax makes the check exact alpha-equality for free (If binds
  nothing; one expr_eq of the two branches). In authored source a
  vacuous if is always dead control or a bug; in search spaces the
  family measured as live fork weight (D17). The match analog stays
  census-priced, not bought, for v1.

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

v1 scope, deliberately thin: DEFINE the hash (spec in this file once
the invariant set settles) and ship a tool that computes and lists
it. KEY NOTHING ON IT YET. The first real consumer should be chosen
the way all our consumers are — by demand; the cert cache is the
likely candidate. What v1 must get right is only the constraint that
already binds: the hash is over the canonical nameless core, so the
canonical form must be stable before any hash is stored anywhere.

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
  examples/canon_pin.shard's cp_pair; term.shard's open_many comment
  ("BVar k = bindings[k]") is about the STORED list, which is
  index-ordered, i.e. reversed from source.
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

Next slices, in dependency order: stage-2 enforcement — std/
canonicalized + the corpus sweep pin (slice 4); the §7 hash spec +
compute tool (slice 5, after the form stabilizes).
