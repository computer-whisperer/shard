# CANON.md — RECORDS (moved out of docs/CANON.md on 2026-09-02)

> **STATUS: RECORD.** Dated rung/slice records split out of [../CANON.md](../CANON.md) so the LAW ledger stays readable in one sitting. Section numbers are exactly as they were in docs/CANON.md; a citation `CANON.md §N` for a section listed here resolves to this file. Nothing here is normative unless the law ledger says so; any "NEXT" pointer is history.

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
  the reducer's own prim table itself — a ground prim redex is exactly a
  core-pathed Call whose literal args make that table step. (Since
  2026-07-25 the kernel recognizer calls `try_step_prim`, which *is* the
  core gate; `tools/canon`'s pre-resolution walk has only a bare symbol,
  so it establishes core-ness itself via `cw_core_free` and then calls
  the ungated table `prim_apply` directly.) Excluded
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

