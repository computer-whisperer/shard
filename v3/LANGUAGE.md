# The V3 language — S, L, E and I as phase 3 built them

> **STATUS (2026-10-16): S, L, E and I as phase 3 built them — phase 3
> closed 2026-10-13 at slice 3.30 landing 4. §13 items 1–69 all ruled:
> 1–38 ratified 2026-09-15; 43, 47 and 49–53 at their slices; 54–64
> 2026-10-08 to 2026-10-13 (57 retired at slice 3.26 landing 3, 59 and
> 60 as amended there, 61–64 as built); 39–42, 44–46 and 48, and item
> 64's as-built notes, at the close. Phase 4 (law §12.4 item 4) is
> open: slices 4.0–4.3 closed 2026-10-14 to 2026-10-16 (§13 items 65–68
> ruled at their slices); 4.4 the bridge designed 2026-10-16 at slice
> 4.3's close with three law changes (§8.6; §13 item 69, ratified).** Normative parent: `docs/FOUNDATION.md`. Scope: the
> surface S, the executable fragment E and `ev` — the reader (§2,
> §4–5), the loader (§3), views (§6.5–6.6, §8.5), the classifier and
> `ev` (§6.2–6.4, §6.7), `realize` (§7) —, the one E and Stage 1 with
> the facilities of phase 3 by their current rules (§8, §8.4: the
> definitions, the porting facilities and bytes, deriving, the proof
> IR **I** with its producers and the tactic library, `Init`, the
> library, templates), entries (§9), conformance (§10). Each semantic
> rule has one current statement here with its as-built note beside
> it (GPT-6 R62, slice 10; slice 3.30 rule 6, 2026-10-13); the build
> history — which slice built what, the design texts, the
> measurements, the findings — is `docs/records/FOUNDATION.md` §9,
> the slice texts of phase 3 its §9.1 verbatim; the review
> correspondence is its §4. What Stages 2–3 add is §11 with its
> phase; §12 is the ledger of changes from v2, the AT RISK rows the
> ones to watch. Decisions this document makes beyond the law's text
> are §13; until ratified they are the implementation's working
> assumptions, not law. This document supersedes `docs/LANGUAGE.md`
> for the `v3/` tree (FOUNDATION §10.5); the old document keeps
> describing the old tree until the flip.

The normative contract is `docs/FOUNDATION.md`; this document
specifies the surface, the executable fragment and the proof IR as
phases 2 and 3 built them and does not restate the law's reasons. K's data — names, levels,
terms, declarations, environments — is defined once, by
`v3/kernel/{name,level,expr,decl,env}.shard`; this document cites
those files and never redraws their constructors.

## 0. The two rulings this draft is built on

- **Stage 0 strictly (ruled 2026-09-07).** At phase 2 a `fn` has an
  executable body that `ev` runs and **no L meaning**: it is not in the
  checked environment, nothing in L can cite it, and no theorem is
  stated about it. Its L value arrives with Stage 1's match compilation
  and structural recursion (phase 3), when `fn` becomes a `def` plus a
  `realize` in one form (law §4.4: "`fn` requests logical admission and
  an executable realization together"). The phase-2 route into K is the
  explicit-L forms: `inductive`, `type`, `structure`, `def`, `theorem`,
  `axiom`, `opaque`, and `realize` for the E side of an L constant.
- **`examples/calc` ports its program half at phase 2 (ruled
  2026-09-07):** the lexer, parser, evaluator, show and the app's step
  function run under `ev`, differential against the old tree; its 100
  claims wait for phase 3's tactics. `v3/MANIFEST.md` says so.

## 1. The languages, restated for the reader

| name | at phase 2 | produced by | checked by |
|---|---|---|---|
| **S** | s-expression text: the forms of §4 and the terms of §5 | authors | nothing (untrusted) |
| **L** | K's `Declaration` and `Expr` values (`decl.shard`, `expr.shard`) | the reader, Stage 0: the explicit-L forms elaborate to L term for term | K (`add.shard`'s `check`) |
| **E** | `kernel/prog.shard`'s `Prog`: E declarations and bodies as data | the reader from `fn`/`type`/`extern`; the classifier from `realize` | the classifier (structural, untrusted); `ev` is its meaning |
| **P** | L terms of `Prop` type | at Stage 0 only `(exact TERM)` | K |

Two pipelines share one reader: **S → L → K** for the logical forms
and **S → E → `ev`** for the executable ones. They meet at `realize`,
which gives an admitted L constant an E body (§7). The toolchain's own
sources — K, `ev`, the reader — are E, read by the Rust bootstrap as
E's executor and by the V3 reader as its gate (§10), under the same
rules as every other file (§8, since 2026-09-14).

## 2. Lexical syntax

The on-disk format is s-expressions.

- **Whitespace** separates tokens; otherwise insignificant.
- **Comments** begin with `;` and run to the end of the line (`;`
  trailing, `;;` line, `;;;` file and section headers — carried).
- **Numerals**: decimal digit strings, `42`; `-7` with a leading `-`
  (§8.1 rule 2). In L a numeral takes the type its position expects
  (§5.3: `Int.ofNat 7`, `Int.negSucc 6` at `Int`; `Nat` otherwise, a
  negative one refused there) — the law's §5.2 rule, since slice 3.15.
- **Symbols**: Lean's identifier characters — Unicode letters, digits,
  `_`, `'`, `?`, `!`, and `.` separating namespace components
  (`List.length`, `Nat.decLt`, `f.eq_1`) — plus the operator symbols
  `+ - * / % < <= > >= = == != && || -> ↑`. A symbol never starts with
  a digit. The naming law (§5.3 of the law) governs which symbols an
  author writes.
- **Universe suffix**: a symbol ending in `.{` opens a **level list**
  closed by `}`: `List.{0}`, `Prod.{u v}`, `Sum.{(max u v) 0}`. The
  reader lexes the list as ordinary tokens (whitespace-separated
  level terms, §5.2) and attaches it to the symbol as its universe
  arguments. This is Lean's `List.{u}` with s-expression level terms.
- **Strings**: `"…"` with the escapes `\n \t \r \\ \" \xHH \u{HHHH}`;
  the value is the UTF-8 byte sequence (K's `LitStr`, `expr.shard`).
- **Lists**: `( … )`. The quote reader macro `'X` ≡ `(quote X)` is
  carried for the toolchain profile; S has no use for it (§5.4).

## 3. Files, modules, identity

**A file is a module.** Its **module path** is its path from the
package root with `/` read as `.` and `.shard` dropped:
`v3/std/list.shard` is `std.list` (`docs/LAYOUT.md`, "The V3 sibling
tree"). A directory module's interface `DIR/mod.req.shard` (or
`DIR/mod.req/mod.req.shard`, the dir form) has the directory's path;
its implementation `DIR/BASE.shard` shares the path (the old tree's
rule, carried exactly — §6.6; other files in the directory are file
modules of their own). The package root is told to the
loader explicitly; nothing under it is named by an absolute path.

**A declaration's identity** (law §8.3) is its **module path plus its
declared name**, and the content hash of its L revision:

- The declared name may itself be dotted — `(fn List.sum …)` in
  `std/list.shard` declares `std.list.List.sum` — so a native
  declaration extends a Lean namespace by name while its identity says
  where it lives. K's environment is keyed on exactly this full name
  (`name.shard`'s `Name`). The construction is not collision-free —
  `a.shard` declaring `b.c` and `a/b.shard` declaring `c` both name
  `a.b.c` — and a collision is **detected**, never conflated: the
  second E declaration is the load error `duplicate_name`, the second
  L declaration K's `already_declared` (pins `qualified_collision`,
  `qualified_collision_l`; GPT-6 R61, slice 10). Names are unique
  within a checked environment; a durable record binds a declaration
  to its package and import revisions (§3.2), never to a spelling.
- **Imported declarations keep their exported names as K names**
  (`List.length`, not `Init.List.length`): their identity is **the
  pin plus the name plus the content** — the `import` form is the
  declared mapping of law §4.4 (§3.2 below), and what it records is
  the *load*: its prefix is the scope, never part of a declaration's
  identity, so enlarging a prefix changes nothing about a declaration
  it already contained (GPT-6 R47, 2026-09-12). The loader's scope
  shows them under the prefix `Init`, which every file that imports
  `Init` opens. So a
  native file cannot declare a bare `List.foo`: it declares
  `std.list.List.foo`, and the two spellings of `List.foo` are two
  identities that only an explicit citation tells apart (T5: "two
  same-spelled nominal types not conflated"; "an attempt to identify a
  different definition by spelling refused" — there is no form that
  identifies them). When a native declaration shadows an imported
  bare name — `main.Bool` beside `Init`'s `Bool` — the bare citation
  is ambiguous and each is cited in full: `main.Bool` by its module
  path, **`Init.Bool` by the import's prefix**, which the scope
  resolves to the bare imported name (slice 7, §13 item 30; pin
  `same_spelled`).
- The **content hash** is `expr.shard`'s structural hash over the L
  declaration (kind, level parameters, type, value), never over S
  text: whitespace, comments, the order of unrelated forms and the
  root's location change no identity (T8's origin-only change). The
  hash of an `inductive` covers its constructor types; the hash of a
  `realize` covers the E body and the equation identities (§7). It is
  a **fingerprint for records and fixtures, never an authority**:
  nothing is enabled, identified or trusted on hash agreement alone
  (GPT-6 R42, 2026-09-12 — the hash is affine in a trailing literal, so
  a preimage is one subtraction; K's accelerator pins compare reference
  declarations structurally, `add.shard`). The collision-resistant
  identity that law §7.5's content store needs is chosen with the
  store, phase 3.
- Moving a file within the root changes its module path and therefore
  its identity, exactly as a Lean `import` path would; only the root
  moves for free (the flip).

### 3.1 `import` and `use`

```
(import "list.shard")          ; a file, relative to the importing file
(import "../std/list")         ; a directory module: its view (§6.5)
(import Init WellFounded.fix)  ; the pinned export through the named declaration
(use std.list)                 ; open a prefix: std.list.X is citable as X
(use Init.List)                ; open a namespace: List.nil is citable as nil
```

`import` brings identities into the **scope**; it opens nothing.
`use` opens a **prefix**: every identity beginning with `PREFIX.` is
citable by the remainder; `(use PREFIX NAME…)` opens only the named
members (v2's selective `(use (:: m name))`, 1,768 sites in the old
tree; Lean's `open M (a b)`). One mechanism serves modules and
namespaces because both are name prefixes. A citation that resolves under two
opened prefixes is an error naming both identities; it is never a
silent choice, and the full identity always resolves. `(use Init)` is
implicit in every file that imports `Init`, so `List.length` cites the
import's `List.length`; `nil` needs `(use Init.List)` or the qualified
`List.nil` (Stage 1 resolves constructors by expected type; Stage 0
has no expected type).

**The `Init` import is a dependency-ordered prefix of the pinned
export**, K-checked on load, named by its last declaration. Two
prefixes are nested, so a closure that imports `(import Init Nat.decEq)`
in one file and `(import Init WellFounded.fix)` in another checks the
larger once. The loader refuses an export whose `meta` line (the
kernel githash and the exporter version, `v3/README.md`'s pin) is not
the pin it was built against: that check plus the `import` form is the
declared mapping of law §4.4. The *load* is recorded as the pin plus
the prefix name (§3.2); a *declaration's* identity is the pin plus its
name plus its content (§3), so two nested prefixes give their shared
declarations one identity and a small program never needs to know the
incidental last declaration of a convenient prefix. Chunk 0 of the export (its first 20,000 lines, 519
declarations, 8 s on route 3) already holds `ite`, `dite`, `Nat.decLt`,
`List.length`, `List.get`, `Decidable.decide` and `WellFounded.fix` —
everything T1's phase-2 items need. The whole export (12 min compiled,
76 min interpreted) is never a load; a long-lived checked environment
is phase 4's T6, and the law's §3.5 and §7.5 forbid the alternative, a
deserialized receipt.

### 3.2 What the loader records (law §8.1 at phase 2)

At slices 3–5 the record is the driver's line per declaration
(`load.shard`), and the pins compare outcomes, never the line's text:

```
LOAD root=DIR route=3 engine=bootstrap             ; the run: route and engine stamp (flags)
INIT NAME: N declarations admitted                 ; the Init load through NAME (§3.1), so far
ACCEPT M.x module=M hash=H axioms=a,b init=NAME    ; identity, module, fingerprint, closure, the prefix at the check
MODULE M file=F hash=H sees=M1,M2                  ; the module done: its hash over its declarations, its closure
RUNNABLE fn M.f why=OBSTACLE | RUNNABLE extern M.e | RUNNABLE type M.T   ; an E declaration classified (§6.7; R45's status kind: runnable, no L meaning); a fn's obstacle to a definition named (§8.4 rule 1)
DEFINE M.f equations=M.f.eq_1,… [pending=ROOT,…]   ; a fn defined (§8.4, slice 3.13): its definition and equations ACCEPT lines, its E body attached
PARAM M.f.dec_1 obligation   ; a measured fn's decreasing fact admitted as a parameter of the fourth class (§8.4 slice 3.14, §9); the fn's DEFINE carries pending=M.f and PENDING M.f measure follows
PARAM M.ax axiom             ; an axiom the file trusts, a declared parameter (§8.6 slice 4.2 landing 2): the manifest's `param` line, the release gate's PARAM, dischargeable by a fulfills
REALIZE NAME supplied equations=M.NAME.realize_1,… | REALIZE NAME view equations=   ; a realization attached under NAME's identity (§7.5)
REFUSE M.x REASON | REFUSE M.f REASON: …           ; K's refusal; the classifier's, with its message
POLICY M.x outside the policy: a | PENDING M.x sorry | PENDING NAME measure
READ-ERROR F REASON: … | LOAD-ERROR F REASON: …    ; the file's loading ends here
LOAD: modules … accepted … runnable … refused … pending … realized … errors …
```

The environment revision of an `ACCEPT` line is its `init=` prefix
plus the `MODULE` lines of the modules its file sees, each with its
hash. A K refusal, a policy refusal and a `sorry` are verdicts on one
declaration — the file goes on, the environment as it was; a reading
or loading error ends the file, and an importer whose import ended so
ends too.

For every admitted declaration: its identity (K name and content
hash); the module; the **route** (3, the bootstrap; 1, compiled K) and
the engine stamp; its **axiom closure** (`axioms.shard`); the
assumption policy applied and its verdict (§9); the environment
revision it was checked against — the `Init` pin and prefix name and
the hashes of the imported modules; and, for a `realize`, the E body's
hash and the identities of its equations. At phase 2 the record is
what the driver prints per declaration and what fixtures pin; the
content store of law §7.5 is phase 3. `Exhausted` is never a receipt:
an exhausted declaration is reported with its resource and site and
enters no environment.

### 3.3 What the loader fixes (slice 3, 2026-09-12)

The rules §3.1 leaves to the implementation, as `loader.shard` has
them; each is a candidate for §13.

- **Visibility is the transitive import closure** (Lean's rule, and
  v2's flat closure): a citation resolves to a constant only if the
  constant's module is the file's own or reachable through its
  `import`s, or is `Init`'s and some file in the closure imported
  `Init`. A constant in K's environment that no import reaches is
  invisible (`loader_test`: two root files, one importing `b`, the
  other not). The loader keeps declaration → module for every native
  constant; a constant absent from that table is the import's.
- **`(use P a b)`** opens `P` for citations whose first component is
  `a` or `b`; `(use Init.List)` opens `List` (imported names are bare
  in K, §3); `(use Init)` opens nothing new.
- **A `type` form opens its constructors** for the whole of its file
  (§13 item 7, amended at slices 3.4 and 3.9) — the constructors only:
  a record's accessors under the same prefix are cited qualified
  (`Load.env`) or opened by `(use M.T)`; `inductive` and `structure`
  open nothing.
- **The export's meta line** must carry the pin's Lean githash, the
  exporter's name and version and the format version (`init_pin_*`);
  the first record is checked before any declaration streams. The
  chunks are the driver's `--init` arguments in order; a target past
  them is `init_name_not_found`.
- **The policy is per file**: `(trusts NAME…)` names are resolved
  through the file's scope when a closure is checked, so `(trusts ax)`
  in the declaring file covers `M.ax`. A theorem in another file that
  rests on `M.ax` without trusting it is outside that file's policy.
  **A trusted axiom is a declared parameter** (slice 4.2 landing 2):
  admitted as `PARAM M.ax axiom`, listed by the store's manifest, named
  by the release gate's `PARAM` lines, every theorem resting on it
  verified as conditional on it; it enters no load-wide parameter list,
  so the per-file policy above stands (pins `policy_trusts`,
  `policy_cross`).
- **A `sorry` name is pending**: citing it is refused with
  `pending_obligation`, never resolved to an assumption.
- **A file whose loading failed is loaded once** (slice 6): its
  module is recorded as failed after its heads were pre-registered, so
  a later import of it reports `import_failed` instead of re-reading
  it into `duplicate_name` (`v3/pins/loader/retry`).
- **Paths**: an import is relative to the importing file's directory,
  never absolute, never above the root; a module path with a dot in a
  file stem, or headed `Init`, is refused; a path without `.shard` is a
  directory module — its view, slice 4.
- **K's raw entry ingests (law §3.5; GPT-6 R43).** `check` rebuilds
  every node of a submitted declaration from its structure before the
  procedure reads it, so no cached id, hash, range or flag a caller
  wrote reaches a shortcut. Before this, two literals wearing one
  positive id made `0 = 1` a theorem through the raw entry (hostile
  battery 16). The import's records enter through K's own lineage and
  never pay the rebuild. What remains open is the profile's exposure of
  `CheckedEnv`'s constructor, sealed when K is one directory module
  behind a view and its first `meta/` client loads against it (§6.6,
  §13 item 26).
- **The sealed-directory rule (slice 3.8, 2026-09-16).** A file inside
  a directory that has a view — `DIR/mod.req.shard` or
  `DIR/mod.req/mod.req.shard` — is private to that directory: a file
  under `DIR` may import it, any other importer is refused with
  `private_module` before the file is read, and `(import "DIR")` is
  the way in. The directories above the target are tried from the
  root down; the importer's own directory and its ancestors never seal
  against it (`loader.shard` `sealed_above`; pins `private_module`,
  `private_inside`). The private-equality leak's pins (`ev_private_match`,
  `ev_launder`) are retired: from outside the seal refuses the
  import first, and inside the directory the type is concrete, so no
  file can hold a value of the sig type and its constructor at once
  (the classifier's `private_match` check stays as defense in depth).
- **One module across a sealed directory (slice 3.8, landing 2,
  2026-09-17; §13 item 39).** A file directly inside a directory that
  has a view carries the directory's module path as the prefix of its
  declarations: `k/tc.shard` declares `kernel.k.whnf`, the identity
  the view's `(sig fn whnf …)` names, and the implementation file is
  the directory's import list. The file keeps its own path as the tag
  visibility and the E table's registration go by (`Scope` carries
  both, `sc_module` and `sc_tag`), so a consumer that sees `kernel.k`
  sees the view's declarations and never a private file's. A
  subdirectory (`k/test/`) is a module under it, importing the private
  files as any file inside may. The fork check matches a signature by
  the identity's declaration once an import has loaded it
  (`loader.shard` `match_private`, the private files' forms recorded
  at load), the E signatures compared (`check_signature`).
- **E-only view parameters (landing 2; §13 item 40).** A `sig fn`
  whose signature names an E-only type (`Expr`, `Name`, `Declaration`:
  K holds no constant for them, §8.1 rule 1) is an E parameter only —
  the head `run` links to the implementation's fn, no axiom in K,
  recorded `PARAM NAME fn` like any (`sig_e_only`); a `sig type` whose
  implementation is E only is matched by the E type's arity, K
  admitting the sig type as an opaque constant for L citations. K's
  own view is such a view throughout.
- **A view may import a plain file outside its own directory
  (landing 2; §13 item 41)**: `k/mod.req.shard` imports `../expr.shard`
  to state its signatures; a file of the view's own directory — its
  implementation's side — is still `req_scope` (`view_dir`; the pin
  `view_req_scope` now imports `lib.shard`).

## 4. Declarations — the surface keywords, fixed at phase 2

The law's §5.3 names `fn def type inductive structure sig theorem
realize`; this draft adds `axiom` and `opaque` (K's kinds, needed by the
hostile battery and by views) and carries the shard-only vocabulary
`import use extern requirement fulfills trusts measure`, and `bin` since
slice 4.3 landing 1 (the artifact's form, below). Reserved and refused
with the reason named: `requires` outside a `bin` (a clause of the
form), `app` (dropped at slice 4.3, §12.5: a loop in a `bin` is the MVU
story), `instance` `class` (phase 3, Stage 3), `namespace` `section`
`variable` (undecided; the dotted declared name covers the common case).

| form | phase-2 meaning | goes to |
|---|---|---|
| `(import …)`, `(use …)` | §3.1 | the scope |
| `(inductive NAME.{u…} BINDERS TYPE (CTOR TYPE)…)` | Lean's `inductive`: level parameters on the name, parameter binders, the type after the parameters (`(Sort …)` or a `forall` over indices), each constructor's full type after the parameters | K: `InductDecl` |
| `(type (NAME T…) (CTOR FIELD-TYPE…)…)` | today's E data form, sugar for an `inductive` with parameters `T : Type`, result `Type`, non-dependent fields; E-eligible by construction | K (`InductDecl`) **and** E (`EInd`) |
| `(record NAME (ctor CTOR)? (FIELD TYPE)+)` | named-field products (v2's form, docs/LANGUAGE.md "Records"; slice 3.9, §13 item 42): loader-level sugar expanded before any form is read — the positional `(type NAME (CTOR TYPE…))`, CTOR defaulting to `MkNAME`, an accessor `NAME.FIELD` and an updater `NAME.with_FIELD` (value first) per field in the record's namespace, which the file opens as any type's (item 7): `(FIELD r)` bare where no other opened record has the field, `(NAME.FIELD r)` always; NAME may be `(NAME P…)`, the generated fns then binding `(P Type)` first. `(make NAME (FIELD V)…)` — every field exactly once, order-free — and `(with NAME E (FIELD V)…)` — chained updaters, a later entry outermost — both against the current file's records, rewritten in every form of the file, nested values first (`kernel/record.shard`; the bootstrap's `expand_records` the twin, parity the tie). No law family is generated: since Stage 1 the accessors and updaters are definitions and v2's laws are `Eq.refl` over a local (pin `record_laws`). `make` and `with` also reach a parameterless `(structure NAME () (FIELD TYPE)…)` of the file — `NAME.mk` on the values in field order, on each field's entry or its projection of `E` for `with`; a field kept whose type mentions an updated one is the refusal `dependent_update` (§8.4 slice 3.17 rule 4). Layout: as `type`'s until CANON's phase-6 gate | E: as the forms it expands to |
| `(structure NAME.{u…} BINDERS (FIELD TYPE)…)` | one constructor `NAME.mk`; projections `NAME.FIELD` generated as `def`s over `proj` (Lean's projections are `Expr.proj` definitions); fields dependent on earlier fields — an earlier field applied in a later one's type too (a package's law over its operation); the parameters implicit in the constructor and the projections and explicit in the type, as Lean declares them (slice 3.28 landing 2) | K |
| `(def NAME.{u…} BINDERS TYPE VALUE)`; `(def NAME BINDERS TYPE (measure (struct x)) VALUE)` | an L definition; hints `regular` at height 1 + the greatest height it references; with a recursion clause a definition that recurses — the fn pipeline's K half, its equations, no program (§8.6 slice 4.2 landing 0) | K: `DefnDecl` |
| `(abbrev …)` | as `def` with hint `abbrev` | K |
| `(opaque NAME.{u…} BINDERS TYPE VALUE)` | checked, never unfolded | K: `OpaqueDecl` |
| `(theorem NAME.{u…} BINDERS PROP PROOF)` | PROOF is `(exact TERM)` or `sorry` at Stage 0; `sorry` is reported loudly, admits nothing, and every citation of the theorem is a pending obligation, never an assumption | K: `ThmDecl` |
| `(axiom NAME.{u…} BINDERS TYPE)` | admitted only under `(trusts NAME)` in the same file (§9) | K: `AxiomDecl` |
| `(fn NAME BINDERS RET (measure M)? BODY)` | an E function: parameters are E-types, BODY is §5.4's E term language; classified (§6.3); **no L declaration at phase 2** (§0); its definition and equations at Stage 1 (§8.4) | E: `EFn`; K: `DefnDecl` + `NAME.eq_N` when eligible (§8.4) |
| `(extern NAME BINDERS RET)` | a World extern (law §4.7); in L an opaque constant of its declared type whose value is the type's default witness (`kernel/inhabit.shard`; slice 4.3 landing 1 rule 3) — never unfolded, no equations, so a function calling it is defined with its equations and a requirement can name the entry; a file without Init, or a signature with no L reading or no witness, keeps it E only with the obstacle on its record | E: `EExtern`; K: `OpaqueDecl` |
| `(sig fn NAME BINDERS RET)`, `(sig type (NAME T…))` | a view's bodyless signature and opaque type (§6.5) | E: `ESig`, `ESigType`; L: a view parameter |
| `(requirement NAME BINDERS PROP)`, `(fulfills NAME PROOF)` | a view's promised law and its discharge in the implementation (§6.5) | L |
| `(realize NAME …)` | an E body for an admitted L constant (§7) | E: `EFn`; L: the equations |
| `(derive TYPE CAPABILITY…)` | since slice 3.19 (§8.4): equality, an ordering and a rendering for a closed inductive type — `eq`, `(ord structural)`, `render` generated as `fn` forms the loader reads at that point; `(CAPABILITY by NAME)` registers a procedure of the file's. Never in a view | the generated `fn`s as any `fn`; the derivation table (`DERIVE`) |
| `(trusts AXIOM…)` | widens this file's assumption policy (§9) | policy |
| `(bin NAME (entry F) (externs E…) (trusts A…) (requires R…))` | the artifact declaration (slice 4.3 landing 1 rule 4; law §4.7, §9.3), checked at its file's end once every fn settled: the entry a fn in scope with the World last and checked parameters before it (§9); the roster every extern the entry's E closure calls, each listed one declared; the trusts the axioms the requirements and the closure's definitions rest on beyond the standard profile and the load's parameters; each requirement an accepted theorem (MET) or one read and pending (UNMET); every function of the closure `THREAD` ok. Recorded `BIN NAME entry F externs N trusts N requires MET/TOTAL`, then `UNMET R`, `UNTHREADED F VERDICT` and `BAD-BIN NAME REASON: DETAIL` lines, each an error that fails the load; the manifest's `bin` line and the release gate's `BIN NAME ok` / `BAD-BIN` (§9). In its own file beside the program (`examples/calc/calc_bin.shard`) | the loader; the store; the gate |

**Binders** are `((x TYPE) …)`; a binder may carry an info marker,
`(x TYPE implicit)`, `(x TYPE strict)`, `(x TYPE inst)`, recorded as
K's `BinderInfo`. Since slice 3.15 the elaborator inserts an implicit
binder's argument at every citation (§5.1) — `@NAME` passes them
explicitly, Stage 0's spelling. A `type`'s parameters are implicit in
its constructors and a `fn`'s type parameters in its L signature and
equations, as Lean's are and as E cites them (`(Pair2.mk a b)`,
`(len xs)` in a theorem as in a body).

**Universe parameters** are declared by the `.{u v}` suffix on the
declared name and may be cited by the same suffix (§2). Since slice
3.15 a polymorphic constant cited bare takes its levels by inference
(§5.1); one the inputs do not determine is `unsolved_universe` with the
pointer to write `List.{0}`. In **E-type positions** (a `fn`'s binders
and return type, a `type`'s fields, a `realize` signature) every
polymorphic type constructor is instantiated at level 0 by rule, since
E-types live in `Type` (law §4.2) — reconstruction of what the form
determines, not inference.

## 5. Terms

### 5.1 L terms (the argument of `def`, `theorem`, `inductive`, `structure`, `realize`'s equations) — Stage 1 since slice 3.15

```
TERM ::= NAME                          ; a bound variable (innermost binding wins), else a constant
       | NAME.{LEVEL…}                 ; a constant with its universe arguments written
       | @NAME | @NAME.{LEVEL…}        ; a constant with every binder explicit (Lean's @)
       | (TERM TERM…)                  ; application, left-nested
       | (OP TERM TERM) | (not TERM)   ; an operator spelling at the first operand's type (below)
       | (if TERM TERM TERM)           ; ite with the decision of the condition's head; cond on a Bool
       | (fun (BINDER…) TERM)          ; lambda
       | (forall (BINDER…) TERM)       ; Pi
       | (exists (BINDER…) TERM)       ; Exists over the lambda
       | (-> TERM… TERM)               ; non-dependent Pi, right-nested
       | (let ((x TYPE TERM)…) TERM)   ; sequential; one L Let per binding; (x TERM) takes the term's type
       | (match TERM (PAT TERM)…)      ; a generated matcher applied (§8.4 slice 3.16 rule 1)
       | (list TERM…)                  ; the constructor chain of the expected type's list
       | (Sort LEVEL) | Prop | Type | (Type LEVEL)
       | NUMERAL                       ; Nat, or Int at an expected Int (§5.3)
       | "…"                           ; K's String literal
       | (proj S i TERM)               ; the i-th field of structure S (0-based)
OP   ::= + - * / mod % < <= > >= = != and or iff   ; `lt le int_eq` for `< <= =` in the toolchain's own sources only (§8.4 slice 3.17 rule 5)
PAT  ::= _ | NAME | (CTOR PAT…)        ; NAME is the scrutinee type's constructor of that name when it has no fields, else a variable
```

Elaboration (`kernel/elab.shard`; law §5.1 Stage 1, §5.2) is
**bidirectional over K's locals**: every rule takes the expected type
its position gives it and returns the term with its type; names bound
by `fun`, `forall`, `exists` and `let` are K locals (`mk_local`), the
terms are built over them and closed by K's own `mk_binding`, so no
de Bruijn arithmetic exists above K. Unbound names resolve through the
scope (§3.1). `Prop` is `(Sort 0)`, `Type` is `(Sort 1)`, `(Type u)`
is `(Sort (succ u))`. A bound name shadows a constant.

- **Implicit arguments.** A constant's citation walks its type: each
  binder marked `implicit` or `inst` gets a fresh metavariable
  (`unify.shard`'s `MVar`, its telescope the locals in scope), a
  `strict` one only before a written argument, and each written
  argument goes to the next explicit binder — `(List.cons x t)` is
  `@List.cons ?α x t`, a bare `List.nil` is `@List.nil ?α`. `@NAME`
  makes every binder explicit. An argument past the type's binders,
  after K's whnf of the type, is `function_expected`.
- **Unification is first-order** (law §5.1): structural descent
  assigning a metavariable on either side after an occurs check and
  the scope check, the assignment typed (the value's type against the
  metavariable's, which is where a universe is inferred: `?α := α`
  with `?α : Sort ?v` and `α : Sort u` gives `?v := u`); levels
  structurally with `is_equivalent` on closed ones; a closed mismatch
  decided by K's own `is_def_eq` through the view; under a
  metavariable, one retry after K's whnf of both sides. Transactional:
  a failure hands back the state it was given.
- **The order** (Lean's `elabApp`): the result type against the
  expected type first, best effort; then the written arguments left
  to right at their binders' types, **numerals last** — so
  `(List.cons 0 nil)` at `(List Int)` reads `0` at `Int`; then the
  term's type against the expected type, mandatory.
- **Universe inference.** A polymorphic constant cited without `.{…}`
  takes a level metavariable per parameter, solved by the unification
  above; `.{…}` written is checked against the arity as before; in
  an **E-type position** (§4) level 0 by rule, unchanged.
- **What the inputs do not determine is a refusal**, never a default
  (law §5.2): `unsolved_implicit` names the binder with the pointer
  to `@`; `instance_needed` for an `inst` binder (Stage 3 resolves
  them; the pointer is `@` with the instance written);
  `unsolved_universe` points to `NAME.{…}`. The one default is the
  numeral's `Nat` (§5.3).
- **The one coercion.** Where a `Nat` meets an expected `Int` and
  `Init`'s `Int.ofNat` is in scope, the term becomes `(Int.ofNat t)`
  (law §5.2); every other mismatch is `type_mismatch` naming both
  types — an `Int` at a `Nat` included (`Int.toNat` is a named
  conversion, never inserted).
- **`Bool` and propositions** (slice 3.16 rule 3, Lean's way). A
  proposition where a `Bool` is expected is `Decidable.decide p dec`
  with the decision `if` would find (else `no_decision`); a `Bool`
  where a proposition is expected is `b = true`.
- **The operator spellings** take the naming-law identity at the type
  of their first operand — of the first that is not a numeral, so
  `(<= 48 c)` is `c`'s (slice 3.16) — `+ - * / mod` at `Nat` are `Nat.add sub
  mul div mod`, at `Int` `Int.add sub mul div emod`; `< <=` are
  `Nat.lt le` / `Int.lt le`, `> >=` the flipped ones;
  `=` is `Eq`, `!=` is `Ne`, `iff` is `Iff` (their type the
  implicit argument); `and or not` are `And Or Not` on propositions
  and Init's `Bool.and`, `Bool.or`, `Bool.not` on `Bool`s (Lean
  4.33's names; the elaborator cited `and or not`, which the export
  does not have, until slice 3.19). In L nothing runs, so `-` `/`
  `mod` at `Nat` have their identities here where E refuses them
  (§8.4 rule 3, guard 2: a restriction of E, not a dialect).
- **`if`** is `ite C dec T F` where `dec` is the decision of `C`'s
  head the elaborator knows — `Nat.lt le` → `Nat.decLt decLe`,
  `Int.lt le` → `Int.decLt decLe`, `Eq` at `Nat`/`Int`/`Bool` →
  `Nat.decEq`/`Int.decEq`/`instDecidableEqBool`, `Eq` at any other
  type the derivation table's one visible `eq` entry (§8.4 slice 3.19
  rule 7; two are `ambiguous_decision`) — else `no_decision`
  with the pointer to `@ite` with the instance, or to derive; a `Bool` condition
  `c` is the proposition `c = true` with `instDecidableEqBool` (Lean's
  own elaboration; slice 3.16 rule 3 — slice 3.15 had `cond`); a
  condition of another two-constructor inductive is `T.casesOn` with
  a constant motive, its alternatives ignoring their fields, the
  second constructor taking the then-branch (§6.2's tag rule; the
  erasure reads the `if` back; a one- or three-constructor type is
  `if_type`, a view's sig type `private_if` — §6.2's refusals, the
  elaborator's since slice 3.16 landing 3). `(list a b …)` is the constructor chain of the expected
  type's list. A **`match`** takes its type from its position; where
  its scrutinee is a local that type mentions, the motive generalizes
  it and each row is read at the type with its pattern (§8.4 slice
  3.19 rule 9) — a proof by cases, a result typed by its scrutinee. `(exists ((x T)) P)` is `Exists (fun (x : T) => P)`.
- **What K sees** is the closed term with every metavariable
  instantiated; K rechecks it whole (add.shard), so a wrong assignment
  is K's refusal, a missing one this elaborator's with its pointer.
  A type cited before its declaration is admitted (`inductive`'s own
  name in its constructors, a `structure`'s fields through their
  projections) is typed by the elaborator from what the form declares,
  never by K. Stage 0's spelling with every implicit written remains
  readable through `@`.

### 5.2 Levels

```
LEVEL ::= NUMERAL | NAME | (succ LEVEL) | (max LEVEL LEVEL) | (imax LEVEL LEVEL)
```

The same grammar inside `.{…}` and in `(Sort …)`; `NAME` must be a
declared universe parameter of the enclosing declaration
(`level.shard`'s `check_level`). Lean's `u+1` is `(succ u)`.

### 5.3 Literals

A numeral at an expected `Int` is `Int.ofNat n`, a negative one
`Int.negSucc (-n-1)`, where `Init`'s `Int.ofNat` is in scope; at
`Nat`, or where nothing decides, K's `LitNat` (law §5.2's rule, in L
since slice 3.15; a negative numeral elsewhere is `negative_numeral`).
A string is K's `LitStr` and has type `String`, the pinned declaration
(`v3/INVENTORY.md`); `Char` values are `(Char.ofNat 97)`. The toolchain profile reads both differently
(§8). The E realization of `String` (the validated-UTF-8 buffer) is
phase 3; at phase 2 no E body computes with a `String` literal outside
the toolchain profile.

### 5.4 E terms (the body of `fn` and of a supplied `realize`)

```
E ::= NAME                        ; a parameter or pattern variable (innermost wins), else a call head
    | (HEAD E…)                   ; saturated: a constructor, an E function, a primitive (§6.4) or an extern
    | (match E (PAT E)…)          ; first arm whose pattern matches; refused if not exhaustive (§6.3)
    | (let ((x E)…) E)            ; sequential bindings (RULED 2026-09-12, R44); `(let* …)` is not a form
    | (if E E E)                  ; branches on a decision tag (§6.2)
    | NUMERAL | "…" | (quote NAME)
PAT ::= _ | NAME | (CTOR PAT…) | NUMERAL | (quote NAME)
```

A `fn`'s binders are E-types; a `NAME` in head position resolves,
through the scope, to exactly one of: a constructor of an E-eligible
inductive, an `fn`/`sig fn`, a primitive, an extern. A function name in
argument position is the escape rule's refusal (law §4.3); there are no
function values. The E term language is deliberately today's: the
toolchain's own sources, `examples/calc` and every PORT `fn` are
already written in it, and its meaning is `ev` (§6.2). **`let` is
sequential in both L and E** (RULED 2026-09-12, GPT-6 R44): each
binding sees the ones before it, as Lean's `let` does; `ev` binds in
order and Stage 1 lowers an `ELet` to nested one-binding `Let`s. The
bootstrap's evaluator binds in parallel, and the tree was measured
before ruling: of its 30,611 `let` groups, 162 bind two or more names
and **none** has a later right-hand side citing an earlier binder (the
scanner's one hit is a quoted symbol), so every existing source means
the same under both rules, no migration is needed, and the toolchain
profile changes no result while the bootstrap still runs it; the V3
reader (slice 2) gives the toolchain's own sources the sequential rule.

## 6. E — the executable fragment at phase 2

### 6.1 E programs as data: `kernel/prog.shard`

`Prog` is the datatype `ev` runs and, once the toolchain's own sources
are admitted to L (phase 3 and the flip), the L type over which
evaluation reflection is stated (law §4.4: `rfl : ev p args n = some
v`). It is declared once, in the toolchain profile, in
`v3/kernel/prog.shard`; this section is its summary, the file is the
definition. Its shape is the Rust bootstrap's AST (`rust_bootstrap/
src/ast.rs`) with names resolved to identities and call heads
classified:

- `EType`: a type constructor applied to E-types, or a type parameter
  by position.
- `ETerm`: bound variable (de Bruijn, 0 innermost); literal;
  constructor, function call, **primitive** and **extern** — four
  distinct nodes, each with a resolved `Name` and saturated arguments,
  so `ev` never guesses what a head is; `match` with arms; sequential
  `let`; `if`.
- `EPat`: variable (binds the next index), constructor with
  sub-patterns, literal.
- `EDecl`: an E-eligible inductive (name, parameter count,
  constructors with field types); a function (name, type-parameter
  count, parameter types, return type, **recursion structure**, body);
  an extern; a view's signature and opaque type.
- `ERec`: none, structural on parameter *i*, or a measure term — the
  "recursion" of the resolved executable structure the correspondence
  is stated over (law §4.4).
- `Val`: constructor cells, unbounded integers (the realization of
  `Nat` and `Int` alike, `v3/INVENTORY.md`), symbols (§8).
- `EvRes`: a value, out of fuel, a primitive's resource exhausted
  (`nat_size`, `nat_count` — like out of fuel, never a refusal; slice
  9, R51), or stuck with a reason and the subject.

### 6.2 `ev` — the definition of "run"

`ev : Prog → Name → List Val → Int → EvRes`, one shard `fn` in the
toolchain profile (`kernel/ev.shard`; the machine it is built as is
§6.7): entering a function body costs one unit of fuel, walking a term
costs none. **Pure**: an extern node is `EvStuck extern`; the effectful
driver `run` performs the extern through the host's own extern of the
same short name and resumes with its result — the explicit handler
contract of law §4.7 at phase 2 is "the toolchain's six World externs,
performed in order".

Per node: a variable reads its frame slot; a literal is its value; a
constructor evaluates its arguments left to right and builds the cell;
a call evaluates its arguments, then the callee's body in a fresh frame
of exactly those values, fuel less one; a primitive applies §6.4's
table — a guard failure (division by zero for the profile's `/`, a
shift out of range, a negative where a `Nat` is required) is
`EvStuck guard`, never a value, and a resource past K's literal limits
(a count over 32 bits, a result over 2^27 bytes) is `EvExhausted`,
checked before the work as K's `nat_apply` checks it and never
`EvStuck`, since exhaustion says nothing about the input (slice 9,
R51); `match` tries arms in
order and the first matching pattern binds its variables, no arm
matching is `EvStuck`; `let` evaluates its right-hand sides in order,
each in the frame the earlier bindings extended (sequential, RULED
2026-09-12 — R44); `if` evaluates its condition to a cell of a
**transparent two-constructor type** — `Bool`, Init's `Bool`,
`Decidable`, a `type` of the program's — and takes the **then** branch
iff the cell is that type's **second** constructor: `Bool.true`,
`Decidable.isTrue` and the toolchain's `True` all are, which is what
"a decision tag with erased payload" means at run time: the tag
decides, the payload is not there. The observation is the **type's**,
supplied by its public declaration, never the cell's: a value of a
`sig type` is not a condition through the view (`private_if`, the leak
of §6.5), a type of one or three constructors does not inherit the rule
(`if_type`), and the classifier refuses both wherever the declarations
fix the condition's static type (§6.7 item 5; a type parameter is
unchecked at Stage 0, as a match on one is). `ev`'s tag bit (§6.7)
implements this observation on the types the classifier admits and
does not define it — another representation may implement it by a null
test or any justified operation (GPT-6 R56, slice 10). Before slice 10
the condition was untyped and the bit alone decided: an `if` on a
view's opaque handle branched on the implementation's constructor
order, a three-constructor type by ordinal parity, a one-constructor
type never (pins `if_private`, `if_type`, `if_one`, `if_ok`).

`ev` is E at phase 2, not an L constant; its two-sided theorem and the
reflection node are phase 4's T8, stated over this `Prog`. Cost is a
measurement (law §4.4), taken on route 2 — K's fixture check under
`ev` — before any claim about it.

### 6.3 The fragment classifier (law §4.1, §4.3), Stage 0

A structural pass over every `fn` and every `realize`, loud and
untrusted, before `ev` sees a program:

1. every head resolves to one kind (constructor, function, primitive,
   extern) and is saturated;
2. no function name occurs in argument position, a constructor field,
   a return position or a `let` binding — the escape rule;
3. every `match` is exhaustive over the scrutinee's constructors as far
   as the arms' patterns determine it (a variable or `_` arm closes
   it; Stage 0 has declared E types and §6.7's static reconstruction
   from them, not Stage 1's typing, so a scrutinee whose type neither
   a constructor pattern nor the declarations fix is closed only by a
   variable arm);
4. type parameters are static: they occur in binder types only;
5. in a `realize` derived from an L body (§7.1), every binder and every
   argument is classified by **role** from its L type — a `Sort`-typed
   binder is static and erased, a `Prop`-typed one is erased evidence,
   everything else is runtime data — and an erased binder may occur
   only in erased positions of the L body; a runtime position that
   obtains its value solely through an erased or noncomputable term is
   the refusal (law §4.2), with the position named.

The lowering on the post-specialization closure remains the authority
(law §4.3); the classifier is the loud early gate.

### 6.4 The primitive table

One table keyed on **identity**, so the profile's `+` and the naming
law's `+` reach the same entry and `ev`'s table matches the Rust
bootstrap's `prim.rs` operation for operation (execution parity, §10).
At phase 2 the table is the bootstrap's — `+ - * / mod tmod ediv band
bor bxor bshl bshr int_eq sym_eq lt le sym_of_chars chars_of_sym`
(`docs/LANGUAGE.md` §8; its effectful `gen_fresh` is dropped, §12.1:
law §4.7 has no effectful primitive) — under their profile names
(the three comparisons are `= < <=` since slice 3.17, §8.4), plus the naming-law spellings of law §10.3 as **distinct**
identities where the meaning differs: `Int.tdiv` and `Int.tmod` total
with `x / 0 = 0` and `tmod x 0 = x`, `Int.ediv`/`Int.emod` likewise,
`Nat.sub` saturating, `Nat.land lor xor shiftLeft shiftRight` on
non-negative values. The profile's `/` staying stuck at zero and
`Int.tdiv` returning zero are two entries, not one entry with a mode;
the migration table calls that row a behavior change and this is where
the change is visible. **Slice 5b (§7.5):** K's own accelerated `Nat`
set joins the table under its identities — `Nat.add mul div mod gcd
pow beq ble` beside `Nat.sub` and the bitwise five — and the three
decisions `Nat.decEq decLt decLe`; the naming-law entries are L
constants under their own names (`prim_l_identity`), the profile's
spellings have none, and an entry of the table is never `realize`d:
K's rule is its realization — K's reduction rule fixes the entry's
meaning, and the executor's implementation is tied to that meaning by
the primitive suite of §10 item 2, every entry's positive, negative and
boundary case fixed by hand and run under `ev`: an accounted
conformance, not an exemption from correspondence (GPT-6 R58, slice
10). **Slice 9 (R51):**
every entry's outcome is three-valued — a value, the guard failed, a
resource exhausted (`nat_count`, `nat_size`) — the third decided before
the work in K's order (the zero shortcut, the count cap, the size
estimate), never after building a result; `Nat.shiftLeft` had none of
the three under `ev` and looped in 62-bit steps (a shift of zero by
10^12 was 16 billion steps), and K's own `Nat.shiftRight` computed
`2^k` for any `k`; both now K's `n · 2^k` and `n / 2^k` with the
shortcuts. A sum or a product is measured after: its size is bounded
by its operands', which are literals under the cap.

### 6.5 Views (law §8.2) at phase 2

A directory module's interface is an **environment view**: `type`
(transparent), `sig type` (opaque: the name and arity, no
constructors), `sig fn` (a signature, no body), `requirement` (a law,
no proof), and `theorem` (a public lemma with its proof). The three
checked conditions, at Stage 0:

- **view validity**: the view's declarations form a well-formed
  environment on their own — each `sig fn` and `sig type` enters K as a
  **view parameter** (an axiom-kind constant to K, a distinct class to
  the policy of §9), each `requirement` likewise, each `theorem` is
  checked against them;
- **implementation matching**: the implementation files declare a
  `type` for every `sig type` with the same arity, a `fn` (E) for every
  `sig fn` with the same E signature, and a `fulfills` for every
  `requirement` whose proof checks against the implementation's own
  environment, where the implementation's `type` shadows the opaque
  twin (the old tree's structural opacity rule, carried);
- **evidence binding**: an exported theorem's axiom closure names the
  view parameters it rests on; the implementation check reports each
  parameter's status — a `sig type` met by a `type`, a `sig fn`
  **linked** to an E `fn` by signature (the parameter stays a
  parameter: a `fn` has no L meaning at Stage 0, so nothing is
  logically discharged), a `requirement` **proved** by its `fulfills`
  or **pending** — and a theorem whose closure names a parameter no
  implementation discharged is reported, not accepted. Matching a
  signature, linking an implementation and establishing the logical
  instance are three statuses, never one (GPT-6 R57, slice 10): a
  consumer's checked result is instantiated for an implementation only
  by a justified substitution that inherits the assumptions of the
  evidence supplied — the `fulfills` proofs' own closures — and binds
  the realizations selected; that construction, the checked instance
  record, is the phase-3 two-instance gate's (§11). At Stage 0 the
  records name the parts and compose nothing: a consumer's `params=`
  and an implementation's `DISCHARGE` kinds are read together by hand,
  and no record claims the composite.

An interface file imports only other interfaces, bare directory
modules and the kernel, never an implementation file — v2's req-scope
gate, carried, refused before the file is read. A consumer checks
against the view alone: its L environment holds the view parameters,
never the bodies; K cannot unfold a `sig fn`. Its E
programs are linked to the implementation's `EFn`s by `run` at
execution (T5's "with the impl linked"). A consumer that matches a
`sig type`'s constructor or compares two values of a `sig type` for
equality by constructor is refused at classification — the
private-equality leak. This is the module surface phase 1 deferred to:
K itself becomes one directory module behind a view and no client
outside it can build a `CheckedEnv` (law §3.5) — the boundary is K, not
`kernel/env`, for the reason §6.6 gives; §13 item 26 carries the
completion criterion.

### 6.6 The loader's view mechanics (slice 4, 2026-09-12)

What §6.5 leaves to the implementation, as `loader.shard` builds it.
K's environment holds one declaration per name, so the old tree's
structural opacity — two same-named typedefs in one closure, lookup
preferring the one with constructors — cannot be carried as it was.
The rule that replaces it: **one environment per role.** A consumer's
environment holds the view's parameters; the implementation is checked
in a **fork** of the loader's state taken before the view was loaded,
where the view's forms are replayed with the implementation's concrete
declarations substituted at each signature. Nothing is looked up by
preference; K sees one `std.list.List` in either environment.

- **Files.** A directory module `DIR` has its interface at
  `DIR/mod.req.shard` or, the dir form, `DIR/mod.req/mod.req.shard`,
  and its implementation at `DIR/BASE.shard` (`BASE` the directory's
  last component) — the old tree's rule exactly; §3's "the directory's
  other files" is narrowed to that file at this slice, other files in
  `DIR` being ordinary file modules with their own paths. Both carry
  the module path `DIR`. `(import "DIR")` loads the interface; a
  missing one is `missing_interface`. Sibling files under `mod.req/`
  are refused until a consumer needs them.
- **The view's forms.** `(sig type NAME)` or `(sig type (NAME T…))`
  admits the view parameter `DIR.NAME : Type → … → Type`; `(sig fn
  NAME BINDERS RET)` admits `DIR.NAME : ∀ BINDERS, RET`, the binders
  and result read as E-types (§4: level 0 by rule); `(requirement NAME
  BINDERS PROP)` admits `DIR.NAME : ∀ BINDERS, PROP`. Each is an
  axiom-kind constant to K and a **view parameter** to the policy —
  the third class of §9: a declaration whose closure reaches one is
  accepted and its record names it (`params=`). The L forms (`type
  inductive structure def abbrev theorem axiom`) are the view's own,
  transparent; `fn`, `extern`, `realize` and `fulfills` are refused in
  a view (`view_form`). The req-scope gate: a view imports only
  directory modules, `Init` and req-scope files; a plain file import
  is `req_scope`, refused before the file is read.
- **The implementation check** runs when the loader is given `DIR`
  itself (the driver: `load.shard --root R R/DIR`), never when a
  consumer imports it. In the fork, the view's forms are replayed in
  view order: a directive as in the view; an L form admitted; `(sig
  type NAME)` replaced by the implementation's `(type NAME …)` — the
  implementation file's forms are consumed in **file order up to and
  including** the matching form, so a private helper declared before
  it is admitted first — its parameter count compared
  (`impl_type_arity`); `(sig fn NAME …)` matched against the
  implementation's `(fn NAME BINDERS RET …)` by the E signature read
  in the fork (`expr_eq` of the Π-types; `impl_signature`), the
  parameter then admitted again as a parameter — `DISCHARGE NAME fn`
  says linked, not discharged — since a `fn` has no L meaning at
  Stage 0 (§0); under Stage 1 the implementation's definition takes
  the parameter's place in the fork and nowhere else (§8.5): when the
  fork holds a definition under the parameter's identity no parameter
  is admitted over it and the line is `DISCHARGE NAME defined`
  (slice 3.13); `(requirement NAME …)` discharged by the implementation's
  `(fulfills NAME PROOF)` — the statement re-read in the fork, where
  the sig types are concrete, and the proof `(exact TERM)` checked as
  a theorem or `sorry` recorded pending; a view `theorem` is not
  re-checked (it was checked once, against the parameters; what binds
  it is its closure). Missing forms are `impl_missing_type`,
  `impl_missing_fn`, `impl_missing_fulfills`; a `fulfills` of no
  requirement is `fulfills_unknown` — unless it names an obligation of a
  measured `fn` of the implementation, which it discharges as in a plain
  file (slice 3.30 landing 2). After the view's forms, the
  implementation's remaining forms in file order (private types,
  theorems, `fn`s deferred to slice 5). The fork's records print under
  `IMPL`, one `DISCHARGE` line per parameter.
- **Evidence binding** at Stage 0 is by closure: an exported
  theorem's `axioms=` names the parameters it rests on; the
  implementation check's `DISCHARGE` lines say each parameter's status
  — `type`, `fn` (linked), `proved`, `pending` (§6.5's three statuses);
  a consumer that links an implementation at `run` is slice 5's.
- **The seal of `CheckedEnv` (§6.5, law §3.5) is deferred to the
  phase-3 opener (ruled 2026-09-13, slice 6; §13 item 26).**
  `kernel/env`'s view is not the boundary: a `sig type CheckedEnv`
  there would have to export the operations `add`, `tc` and `import`
  build environments with — the entry insert, the quotient flag, the
  pin list, the watermark — and a forged environment then enters
  through the API instead of the constructor. The boundary is K: the
  fifteen files `name level expr decl env intmap tc add inductive
  nested import axioms accel_pins refgen json` as one directory module
  whose view carries the 87 functions and the twenty types the rest of
  the toolchain calls today, a bootstrap resolver that follows a
  directory import so route 3 still runs, and a rule refusing a direct
  import into a sealed directory from outside it. No client outside
  `kernel/` exists at phase 2; the first is `meta/` at phase 3, and the
  view should list what `meta/` needs rather than what today's files
  happen to call. Until then the profile exposes the constructor and
  raw-API callers are reviewed toolchain code (R43, `v3/README.md`).
  **The seal as ruled (2026-09-16; §8.3 slice 3.8, §13 item 26):**
  nine trust files behind the view, not fifteen — `env tc add
  inductive nested import axioms accel_pins refgen` move to
  `kernel/k/`, and the data vocabulary (`name level expr decl intmap
  json`, and the new `verdict.shard`: `Reason`, `Resource`, `(Outcome
  E)`, `Verdict`, `Failure`, `(KRes A)`, which name no sealed type)
  stays public where it is, as Lean's `Expr` is public beside
  `Environment.add`, the ingestion at `check` being what makes the
  data safe; `k/k.shard` a facade of one-line wrappers matching the
  view's signatures; a view may import a plain file outside its own
  directory; the internals' tests move inside the directory, the
  hostile battery stays outside as a client; the sealed-directory rule
  (§3.3). Landing 1 (2026-09-16): the move, the vocabulary split, the
  rule and its pins. **Landing 2 (2026-09-17): the view and the
  consumers — and the facade withdrawn.** The fork check matches a
  `sig type` only against a declaration carrying the view's identity,
  which a wrapper in `k/k.shard` can give a function but never a type
  short of boxing it; ruled instead (the user, option 1 of three):
  one module across the sealed directory (§3.3, §13 item 39), so
  `k/k.shard` holds nine imports and nothing else, and the view
  `k/mod.req.shard` names K's own declarations: seven sig types
  (`CheckedEnv AxMemo Tabs Run Ctx LDecl Loc`) and sixty sig fns, the
  toolchain's whole surface plus `env_empty` (the one way to an
  environment from outside), `ctx_new` and the `loc_*` accessors
  (`erase.shard` had built a `Ctx` and destructured a `Loc` by their
  constructors) and the name constants the outside tests use. Every
  consumer is `(import "k")` with `(use kernel.k)`; the tests of the
  internals (`tc add inductive nested hostile`) sit at `k/test/`. The
  view's signatures name E-only types, which K cannot read: such
  parameters are E parameters (§3.3, item 40). Parity and route 2 give
  the loader `v3/kernel/k` as a second root for K's consumers, the
  directory's own check linking the implementation as the bootstrap's
  flat closure holds it. **Landing 3 (2026-09-17): the clients, and
  the seal complete.** The hostile battery is a client of the view
  (`kernel/test/hostile_test.shard`: `env_empty`, `check`,
  `check_with` under `limits`, the data vocabulary — R52's forged-node
  fixtures through the public entry); its nine cases that call the
  accelerator's matcher directly (`accel_ref`, `accel_ref_closure`,
  `ref_matches`) sit inside, `k/test/accel_test.shard`. The
  raw-construction client fixture is `kernel/test/k_client_test.shard`
  (a raw inductive checked from the empty environment, the result
  inspected, a definition checked over it, a refusal leaving the
  environment untouched) with the pin `k_client_reach` beside it: a
  client naming `CheckedEnv`'s constructor is refused
  (`unknown_head`). `ConstantVal`'s accessors moved from `add.shard`
  to the public `decl.shard`. What the criterion still names — the
  first `meta/` consumer importing the view only — the
  sealed-directory rule now enforces for every file outside `k/`,
  transitively, rather than waiting on a review. Since slice 3.11 K's
  own tests run through the V3 loader at test time
  (`k_clients_test.sh`), so the seal is exercised by every run of the
  suite, not only by parity.

### 6.7 The classifier, `ev` and `run` as built (slice 5, 2026-09-12)

Written before the code, as §6.6 was. §6.1–6.4 stay the specification;
this section fixes what they left to the implementation.

**Reading a `fn` is classifying it.** `kernel/classify.shard` reads a
`fn`, `extern`, `type`, `sig fn` and `sig type` into `prog.shard`'s
data with every head resolved to one identity and one kind; nothing
reaches `ev` unresolved. A file's E heads are **pre-registered** —
name, kind, arity, a `type`'s constructors with their ordinals and
field counts — from the whole file before any body is read, so a body
may cite a function or constructor declared later in its file (mutual
recursion; today's flat rule), and one identity declared twice in a
file is a load error (`duplicate_name`). Resolution goes through one
**suffix table**: an identity `kernel.json.hex_val` is findable by
`hex_val`, `json.hex_val` and its full spelling; a citation's hits are
the visible identities it is a suffix of (visibility = §3.3's closure)
and, in S, those among §3.1's scope candidates — the module's own
declarations, the opened prefixes, the bare name — so the S rule is
exactly the reader's for L. Zero hits is `unknown_head` (a bare name
that is neither bound nor a constructor: `unbound_name`), two is
`ambiguous_head`. The checks of §6.3, with their refusal reasons:

1. **kinds and saturation** — a head is a constructor, a function or
   `sig fn` (`ECall`), a primitive (`EPrim`) or an extern (`EExt`), and
   takes exactly its arity of arguments (`unsaturated`; a constructor
   pattern likewise, `pattern_arity`);
2. **the escape rule** — a function, primitive or extern name in
   argument position, that is, anywhere but a head, is `function_value`;
3. **exhaustiveness** — every `match` is checked by the pattern matrix
   (Maranget's usefulness): a column of constructor patterns must cover
   the inductive those constructors belong to, each constructor's
   specialization recursively; a column of literals, and a scrutinee no
   constructor pattern fixes, is closed only by a variable or `_` row.
   The refusal is `nonexhaustive`, naming the function;
4. **type parameters are static** — at Stage 0 no term carries a
   Stage-1 type (item 5's static types are read off declarations), so
   this holds by construction;
5. **the private-equality leak** — the static E type of a scrutinee is
   read off the binders, the constructor field types, the callees'
   return types and the primitives' result types as far as they reach;
   a scrutinee whose static type is a `sig type` matched against any
   constructor pattern is `private_match`; a scrutinee whose static type
   is a known E inductive, `Int` or `Symbol` matched against another
   type's constructor is `pattern_type` (the matrix alone would pass it).
   An `if`'s condition is typed the same way (§6.2; slice 10): a `sig
   type` is `private_if`, an inductive of other than two constructors,
   `Int`, `Nat` or `Symbol` is `if_type`.
   A constructor's and a callee's type arguments are inferred from their
   arguments' static types, so a sig-typed value passed through a
   polymorphic wrapper keeps its type. A scrutinee whose type is a type
   parameter is not checked at Stage 0 (specialization is phase 3). A
   pattern variable named like a visible function, sig fn or extern is
   `pattern_name` — it would match anything, and the same name in term
   position is already `function_value`. A `measure` term gets the same
   passes as the body, beside it.

A classified `fn` is recorded `RUNNABLE fn NAME` (R45's status kind:
runnable, no L meaning; since slice 3.13 with `why=` the obstacle to
its definition, or `DEFINE NAME equations=…` when it has one, §8.4);
an extern `RUNNABLE extern NAME`; a profile
`type` `RUNNABLE type NAME`; a refusal is `REFUSE NAME reason` like
K's, and counts as one.

**One reader for every file (slice 3.4, 2026-09-14; §8).** Nothing
the classifier does is keyed on a file's directory or imports: the
scope is §3.1's for every file, `(T Type)` binders are the type
parameters, the literals read by §8.1 rule 2, `Int`, `Nat` and
`Symbol` are built-in E types (rule 1), and a `type` whose fields cite
a built-in without an L constant in scope — or another such type — is
E only (`RUNNABLE type`) while every other `type` enters K and E. K's
own sources see no `Init`, so their `type`s over `Int` and `Symbol`
are E only and their field-less types and type-parametric types are
K's inductives too. The toolchain profile that used to differ from S
in ten rows (records §9, slice 3.4) is gone.

**`ev` as built.** `kernel/ev.shard` links a `Prog` into its own
representation — no other file sees it — and runs it as a machine with
an explicit continuation: constructors are integer tags whose low bit
says "second constructor of its type" (the bit implements §6.2's
observation on the transparent two-constructor types the classifier
admits as conditions), functions and externs are indices into a table,
primitives are
operation codes, string and list literals are built once at link. Two
tail-recursive steps (evaluate a term in a frame under a continuation;
return a value to a continuation) with the frames for pending
arguments, a `match`, a sequential `let` and an `if`, so the host's
stack does not grow with the program's recursion depth and the machine
can stop **at an extern** and hand its arguments and continuation to
the driver. `ev` (§6.2's signature, pure) reports that state as
`EvStuck extern`; `run` performs the extern through the host's own
extern of the same short name — `get_args read_file write write_line
write_file exit`, the six of `kernel/host.shard` — and resumes the
continuation with the result (§6.2's `run`).
Fuel is spent on function entry, exhaustion is `EvOut`; the stuck
reasons are `no_arm`, `guard` (a primitive's), `if_tag` (a non-cell
condition), `extern` (under `ev`), `unlinked` (a `sig fn` no
implementation was linked to), each naming the function. The wire
(§12.6, "the extern wire's bytes"): a byte list is the toolchain
prelude's `List` of `Int` cells, an argument list its `List`, a file
read its `Option`, a pair `Pair`, a flag its `Bool` — the identities
of `kernel/prelude.shard` (§8), interned by the linker whether or not
the program declares them, so a program without the prelude in its
closure cannot match a wire cell by pattern (an `if` on a
comparison's result works: the primitive's result type is a
two-constructor type). **Since slice 3.18 the wire is read by declared
type** (§8.4 slice 3.18 rule 4): each value of an extern is built and
read by the E type the program declared — these cells under the
toolchain's signatures, `ByteArray` and `Init`'s `List`, `Option`,
`Prod` and `Bool` under the naming law (`v3/std/host.shard`); `Init`'s
cells are interned as the prelude's are; the entry's `World` argument is its
parameter type's first constructor over zero fields. The comparison
primitives return the prelude's `Bool` cells under the same rule.

**Views at run time.** In the implementation fork the implementation's
`fn` and `type` take the view's `sig fn` and `sig type` identities —
the substitution §6.6 replays — and the fork's E declarations merge
into the main line under those identities: the substitution **is** the
link (T5, "with the impl linked"), no link declaration exists. A
consumer loaded against the view alone keeps the `sig fn` and a call to
it is stuck (`unlinked`).

**The driver.** `load.shard --run MODULE.FN [--fuel N] FILE… -- ARG…`
loads, then links and runs `FN` on the World with `ARG…` as the
program's arguments, performing its externs; a load failure prints the
records and exits 2 before running; out of fuel or a primitive's
resource exhausted exits 3 (`RUN: out of fuel in F`, `RUN: exhausted
nat_size in F`), stuck 4, a
link failure 5, and a program that returns without `exit` exits 0. The
program's `exit` is the host's. **Route 2's byte-tie**
(`kernel/test/route2_test.sh`): the T0 driver's closure loaded from
`--root v3` under the profile and run on the fixture, its output and
exit code byte-identical to route 3's — K interpreted by `ev`, hosted
on the bootstrap.

**History (2026-09-12; records §9, slice 5).** K's own sources — `kernel/t0.shard`'s
closure, 19 modules, 5,670 declarations — load under the profile with
no refusal: every kernel `match` is exhaustive by the matrix, no head
is ambiguous, and the one finding was a missing import
(`import.shard` called `write_line` without `host.shard`; the
bootstrap's flat scope had hidden it). Route 2's byte-tie holds on the
fixture (116 declarations): K under `ev`, hosted on the bootstrap,
prints route 3's 184 lines byte for byte and exits 0 in 26 s against
route 3's 0.33 s — an 80× interpretive overhead, measured, not
compared (§10). `ev_test` (33 cases) covers the per-node rules, the
table's guards and totalizations, fuel (R45's self-recursive candidate
exhausts at any fuel), the extern under pure `ev`, the sig fn stuck
and substituted, and a 200,000-deep recursion without the host's
stack; `v3/pins/loader/` gains 19 classifier cases (53 in all). A
review after the build (an Opus subagent, findings verified) found the
pre-registration counting S's `(T Type)` binders as arguments (fixed,
`ev_type_param`), the `Nat` shifts keeping the host's 64-bit guard
(now total, as Lean's), and the gaps the typed pass now closes:
`pattern_type`, the type-argument inference, `pattern_name`, the
measure term; a file module and a directory module with one path are
`module_collision` instead of the later one silently skipped. Every
kernel entrypoint's closure — the driver and the sixteen tests, the
reader, the loader and the classifier included — now classifies clean
(`loader_test`'s: 25 modules, 6,184 declarations); the classifier's
own first draft had two non-exhaustive matches, the reader one, the
reader kit one, all found by the matrix.

**Dropped:** `gen_fresh` (§12.1's AT RISK row): no V3 file calls it,
the table does not carry it, and a ported source that needs fresh
names threads a counter. **Built at slice 5b (§7.5):** `realize`
in both forms and R45's third test; the E→L translation question
resolved by first-order matching over the callee's telescope with the
classifier's static types as the fallback. §3.2 lists the `RUNNABLE`
and `REALIZE` records, §13 the decisions here (items 15–24).

## 7. `realize` — the surface, fixed at phase 2

`realize` attaches a checked E body to an **existing admitted L
constant** under its existing identity (law §4.4). Two forms, both
live at phase 2:

```
(realize NAME (view))
(realize NAME BINDERS RET (measure M)? BODY)
(realize NAME BINDERS RET (measure M)? BODY (equations THM…))
(realize TYPE (repr E-TYPE) …)      ; a type's runtime representation: form reserved, evidence phase 3
```

### 7.1 The derived view

`(realize NAME (view))` asks the classifier to **erase** `NAME`'s L
body into an E body: `Decidable.casesOn`, `ite` and `dite` become
`if` (the instance argument is the decision, its proof binders
erased); `T.casesOn` and `T.rec` with no recursive minor premise
become `match`; constructor applications drop erased fields (`Fin.mk
i h` is `i` under `Fin`'s realization as `Nat`, `v3/INVENTORY.md`);
constants become calls to their own realizations, which must exist.
Refused, with the reason and the position: a body that is `brecOn` or
`WellFounded.fix` (the executable structure of a recursive L
definition is not readable off the kernel term — §7.3), a function
value in a runtime position, a runtime value obtained through
noncomputable choice, a constant without a realization. The derived
view generates no equations: the E body *is* the erasure and its
correspondence is the erasure rule (law §4.6), stated once — a rule
whose **implementation** (`kernel/erase.shard`, the view half of
`realize.shard`) is a bring-up trust dependency on the TCB's V3 roster
until a checked translation or route 1's proof covers it, not a
correspondence already established. A `REALIZE NAME view` record says
which rule the body follows, never that the translation was verified
(GPT-6 R58, slice 10). The realization registry's rows are lines of
that implementation and trusted with it: an operation's entry (slice
3.16 rule 4), an expression (3.17 rule 7), a type's representation and
a string literal's bytes (3.18 rules 1 and 3).

### 7.2 The supplied body

The second form gives an E body whose **signature is the erasure of
`NAME`'s L type**, binder for binder: a `Sort`-typed binder becomes a
type parameter, a `Prop`-typed binder disappears, the rest are the E
parameters in order, and RET is the erasure of the result type. The
loader checks the correspondence positionally. The correspondence of
the body is by **equations, one per leaf of the case tree** that the
body's leading matches on parameters and pattern variables form
(§7.5; one for a body without a `match`): for a leaf reached through
the pattern `(CTOR x…)` with right-hand side `r`, the L statement

```
forall PARAMS, NAME PARAMS-with-(CTOR x…) = ⟦r⟧
```

where `⟦r⟧` translates the E term into L head for head — a
constructor to its constant applied to the parameters, a call to the
callee's L constant, a primitive to its L identity (§6.4), `let` to
`let`, `if` to `dite` over the condition's L term. The equations are
declared as theorems named `NAME.realize_1 … NAME.realize_N` in the
realizing module, and proven at Stage 0 by `rfl` (K's definitional
equality unfolds `brecOn` applied to a constructor — that is exactly
what Lean's own `eq_N` lemmas are proven by), or by the theorems the
`equations` clause names, in arm order, when `rfl` does not close them.
The recursion structure (`measure`) is recorded on the `EFn`;
well-foundedness of the supplied body's recursion is a separate
obligation — "`f x = f x` justifies no looping implementation" (law
§4.4) — and at Stage 0 it is discharged only for structural recursion
on a parameter whose type is an `inductive` the classifier knows
(`RecStruct`); a `measure` other than `struct` is a reported obligation
until phase 3's tactics. **A realization's evidence is kept in its
parts** (GPT-6 R58, slice 10): the equations are its correspondence,
L facts K checked; progress is its own obligation; execution rests on
the executor's conformance (§6.4, §10). A checked equation says nothing
about termination — `f n = f (Nat.add n 1)` is true of the constant
function `f n = 0` and proves by `rfl` while the body never returns and
the measure `n` increases (pin `realize_pending_via`) — so a
realization under a pending measure is a **retained candidate**:
attached, runnable for development, its obligation recorded once
(`PENDING NAME measure`) and carried by every realization whose body
reaches it (`REALIZE … pending=NAME`, §7.5), never mistaken for a
completed realization through a caller. A completed realization is one
whose pending set is empty; a caller that requires the guarantee — the
lowering, Stage 1's admission — reads the set, and nothing at Stage 0
requires it.

### 7.3 Why both forms, on the evidence

In the pinned export `List.length`, `List.get`, `List.map`, `Nat.add`
and `Nat.sub` are `T.brecOn` applied to a generated `NAME._f`
functional, not `T.rec` with visible minor premises; Lean's compiler
never reads the kernel term either — it compiles the pre-definition.
Lean's own equation lemmas exist in the export only where `Init`
realized them, and late: `List.length.eq_2` at line 1,370,217,
`List.length.eq_1` at 5,465,298, `Nat.add.eq_1` not at all. So the
shared core's operations take the supplied form with `rfl` bridges,
and the derived form serves non-recursive definitions (`ite`, `dite`,
`Decidable.decide`, `Nat.decLt`, the structure projections) — which is
where T1's phase-2 items live: a decision tag with erased payload
(`Nat.decLt` derived), a branch-local proof (a `dite` whose `h` is used
only in a `Fin.mk` field), raw versus checked arguments (§9's
entries). Neither form redefines the constant; `List.length`'s
identity, and every imported theorem about it, is unchanged (T5).

### 7.4 Equation names

`NAME.realize_N` rather than Lean's `NAME.eq_N`: the `eq_N` lemmas are
Lean's, tied to Lean's match structure, and present in the export for
some constants — a same-named native theorem would collide at
`check_name` once an `Init` prefix reaches them. `realize_N` is
guessable from this one rule (law §5.3's grammar) and cannot collide.
A ratification item (§13).

### 7.5 `realize` as built (slice 5b, 2026-09-13)

Written before the code, as §6.6 and §6.7 were. §7.1–7.4 stay the
specification; this section fixes what they left to the
implementation, and three decisions the supplied form needs before a
single equation can be stated (§13 items 20–24).

**Init's inductives are E types.** An inductive of the checked
environment — Lean's own or a native `inductive` — is **E-eligible**
when it has no indices, its result sort is not `Prop`, every parameter
is a type (`Sort` at `Type` once its level is solved) or a proposition
(erased), every constructor field is an E type over the type
parameters or a proposition (erased), and no field is a `Sort` or a
function. `Nat` and `Int` are excluded by name: their realization is
the unbounded integer, and a citation of `Nat.succ` or `Int.ofNat` in
an E position is refused with the pointer to numerals and the
primitives (`nat_constructor`). Levels are solved from the parameters'
sorts (`Sort (u+1)` gives `u := 0`, `Sort u` gives `u := 1`), then the
result sort; an inductive whose levels cannot be solved that way is
not eligible. An eligible inductive enters the E table **on first
citation** — each S form is scanned for its names before it is
classified, every name that resolves to an eligible inductive or one
of its constructors registers the inductive with its constructors
(ordinals, runtime field types) under the module that declares it, or
under `Init` — so `List.cons` is a constructor in a pattern and
`(List Nat)` an E type in a binder, and the pattern matrix, the leak
check and `ev` treat them as any `type`. `Decidable` is E-eligible
with no type parameter and two field-less constructors: a decision
tag with erased payload (§6.2); `Prod` and `Sum`, whose result sort is
a `max` the binders already solve, are; `Fin n` is not — a value
parameter —; since slice 3.18 it is a **row of the registry's types**
with `BitVec`, `UInt8`, `Array`, `ByteArray` and `String` (§8.4 slice
3.18 rule 1): represented by its one runtime field, an E type under
its own name, never an E inductive. A constructor with an erased field can
be **matched** in E and never **built** there (`erased_field`): the
proof is not there. S numerals type as `Nat` (the slice-5 pass typed
them as `Int`).

**The primitive table's L identities.** Entries 0–17 (the operator
spellings) have none of their own: they are the bootstrap's Int
operations. Since slice 3.13 an operator resolves to the naming-law
identity by its operands' static types where `ev` and K agree (§8.4
rule 3: `+ *` at `Nat`, `+ - *` at `Int`, the comparisons as the
decisions), and is refused inside a `realize` body otherwise with the
pointer to the naming-law spelling (`no_l_identity`; `- / mod` at
`Nat` are `nat_operator`). Entries 18 and up are L constants under
their own names. Slice 5b adds K's own accelerated `Nat` set —
`Nat.add Nat.mul Nat.div Nat.mod Nat.gcd Nat.pow Nat.beq Nat.ble`
beside the `Nat.sub` and bitwise entries of slice 5 (`reduce_nat`'s
list; `Nat.div x 0 = 0`, `Nat.mod x 0 = x`, `Nat.pow` stuck where K
exhausts) — and the three decisions `if` needs, `Nat.decEq Nat.decLt
Nat.decLe`. `Nat.beq` and `Nat.ble` return `Init`'s `Bool` cells,
the decisions `Decidable.isFalse`/`isTrue` cells with no fields; the
linker interns those four identities as it interns the wire's. An
entry of the table is never `realize`d (`realize_primitive`): K's rule
is its realization, and a body cites it as a primitive. The equation a
realization states *through* such an entry — `twice n = Nat.add n n`
for a native `def` — is proven by K's literal rule for the same
constant, which is the point: the table's `Nat` entries are K's own
accelerated operations under their identities.

**The supplied form.** `(realize NAME BINDERS RET (measure M)? BODY
(equations THM…)?)`, NAME resolving through the scope to a
**definition** of the checked environment (`realize_kind` for an
axiom, a theorem, a constructor). NAME's L type is walked binder by
binder and each binder classified by role: a `Sort`-typed binder is a
**type parameter** (`Type` once the constant's levels are solved as
above; any other universe is `realize_signature`), a proposition is
**erased** (it becomes a binder of every equation and nothing in E), a
data binder's type must translate to an E type, and a function-typed
binder is `realize_signature` with the binder named (law §4.3). A
Pi over a proposition erases to its codomain, so `dite`'s `t : c → α`
is an E binder of type `α`. The E signature must match positionally:
the `(T Type)` binders are the type parameters in order, the data
binders' E types equal the erasure's, RET the result's; the message
names the first binder that differs. BODY is read exactly as a `fn`
body (§6.7's classifier, every check), the measure as a `fn`'s.

**Equations.** The body's **leading matches on parameters and
pattern variables** form a case tree; its leaves are the equations,
one per leaf. The tree is compiled column by column as Lean's match
compiler does (the pattern matrix of §6.7's exhaustiveness check,
specialized instead of tested): for the first column holding a
constructor pattern, each constructor of the column's inductive gives
a branch — the constructor's fields become fresh binders (the erased
fields too; a runtime field is a new column), the column's variable
is **replaced by the constructor term** everywhere it occurs (the
left-hand side included, so nested patterns state nested terms), a row
whose pattern is a variable binds it to that term and continues into
every branch. A leaf's left-hand side is `NAME` applied to the type
parameters, the erased binders and the parameters with their
constructor terms; its right-hand side is the translation of the
first matching arm's body; its binders, in order, are the type
parameters and erased binders, the parameters not split, then the
fields of each split as it happened (a theorem cites
`List.append.realize_2 a ys x t`). A numeral pattern in the tree is
`equation_form` (Stage 1's numeral rule). The translation `⟦·⟧` of a
body: a bound variable is its binder; a numeral is K's literal; a
constructor, a call and a primitive apply the constant's L identity
to **its whole telescope** — the runtime binders from the E arguments,
the type parameters and erased binders solved by **first-order
matching** of each runtime binder's declared type against the
argument's inferred L type (`xs : List α` against `List Nat` fixes
`α`), with the classifier's static E type as the fallback for a bare
constructor (`List.nil`), and `untyped_subterm` naming the position
when neither determines a binder; a `let` is K's `let` with the
inferred type; a `match` whose scrutinee is not a variable, and an
`if`, are the scrutinee's inductive's **recursor** with a constant
motive — the arms flat constructor patterns each once, a variable arm
expanded into the constructors it covers, the recursor's minor
premises lambdas over every field and induction hypothesis, unused
where E has no name for them. Equation *N* is declared as the theorem
`NAME.realize_N` in the realizing module — `NAME.realize_N` itself for
a constant of that module, the module's prefix in front of NAME's
spelling otherwise — at the constant's solved levels and with no
universe parameters of its own (a theorem at another universe cannot
cite it until Stage 1), proven by `Eq.refl` or by the *N*-th theorem
of the `equations` clause applied to the binders, that theorem taken
at no universe arguments or at the constant's solved levels
(`equations_count` when the clause's length is not the leaf count,
`universe_arity` otherwise),
and admitted through the same path as any theorem (the policy, the
`ACCEPT` record); K's refusal of one equation refuses the form and
attaches nothing — the earlier equations are not admitted either.

**Descent and order.** `(measure (struct x))` is discharged
syntactically: every self-call passes, at `x`'s position, a variable
bound by a constructor pattern under `x` (any depth), so `f x = f x`
is `realize_descent` although its equation proves; `(measure E)` is
recorded `PENDING NAME measure` and the realization attached — except
that a self-call on the parameters themselves is `realize_descent`
under any measure, since no measure decreases on it; no measure with a
self-call is `realize_recursion`. A callee must be an L constant with
a realization **already attached** or a view's `sig fn`; a callee
realized later in the file is `realize_order`, a `fn` or an extern
`no_l_meaning`. And a `fn` or `extern` may not take an admitted
constant's identity at all (`name_taken`, the loader): only a
`realize` attaches an E body under one, and its equations are the
warrant — the review of this slice found a `fn append` in a module
named `List` standing in for Lean's `List.append` in a realization's
body, with the equations stated about Lean's. Realizations therefore
form no cycle but self-recursion, and self-recursion is either
structural — a completed realization — or a retained candidate under a
pending measure (§7.2), whose obligation every later realization
reaching it carries.

**The derived view.** `(realize NAME (view))` erases NAME's value
under the same roles: the leading lambdas are the binders (a value
with fewer lambdas than binders is applied to the rest); a constant
applied to arguments is, by its identity, a constructor of an
E-eligible inductive (the static and erased arguments dropped), a
constant with a realization attached (a call, likewise), a primitive
of the table (`Nat.succ x` is `Nat.add x 1`, `OfNat.ofNat Nat n _` is
the numeral), `T.casesOn` or `T.rec` of an E-eligible `T` (a `match`,
each minor's field lambdas the pattern's variables; a minor that uses
an induction hypothesis is `recursion_structure`), `ite` or `dite`
(an `if`; `dite`'s branches applied to their erased proof); a
projection is a `match` on the structure's constructor (the subject
itself for a row of the registry's types; a projection *function*,
fully applied, is that projection — slice 3.18 rules 1 and 2); a `let` whose
value is a proposition or a type vanishes, any other is an E `let`; a
variable of erased role in a runtime position is `erased_in_runtime`,
a lambda or an unknown-headed application `function_value`,
`brecOn`, `WellFounded.fix`, `Acc.rec`, `Quot.*` and
`Classical.choice` are refused by name (`recursion_structure`,
`noncomputable`), any other constant `no_realization`. The result is
classified as a `fn` body and attached under NAME's identity with no
equations (§7.1).

**Records and the driver.** `REALIZE NAME supplied|view
equations=NAME.realize_1,… [pending=ROOT,…]` counts in the new
`realized` column — `pending=` names the constants whose measure
obligations the realization carries, its own and, transitively, its
callees' (slice 10; R58); an obligation itself is one `PENDING ROOT
measure` line, at its constant; the
equations are `ACCEPT` lines; a K refusal of an equation is `REFUSE
NAME.realize_N reason`; the classifier's refusals of the form are
`REFUSE NAME reason: message` as a `fn`'s. The `LATER` record is
gone: `realize` is built, and a `fulfills` outside an implementation
check is the load error `fulfills_outside_impl` (give the loader the
directory) — since slice 3.30 landing 2 by cause: `fulfills_not_obligation`
for a declaration, `fulfills_refused` for an obligation of a `fn` refused
above, `fulfills_outside_impl` for a name of nothing in the load. R45's third test is the pair: the same body as a `fn` is
`RUNNABLE` with no L meaning, as a `realize` it is `REALIZE` with
accepted equations a theorem can cite and a body `ev` runs under the
L identity.

## 8. One E — the executable fragment for every file (phase 3, RULED 2026-09-14)

**The ruling.** Phase 3 opened by building the Rust bootstrap up to
the whole of E, so that the toolchain's own sources — everything under
`v3/kernel`, later `v3/meta` — are written in the same E as every
other file, and the *toolchain profile* this section used to define is
retired (the user, 2026-09-14, on the sizing in records §9; GPT-6's
R55 dissolves with it — there is no profile left to name). Ruled at
slice 3.1 and built at slices 3.2–3.4 the same day (§8.3); every
difference the old profile table listed is decided below, not
carried (§8.2).

### 8.1 The rules of the one E

1. **Built-in E types.** `Int`, `Nat` and `Symbol` are E types in
   every file. Where `Init` is in scope, `Int` and `Nat` are also its
   constants — one identity, the bare name (§3), exactly as the E table
   keys them. `Symbol` is the interned atom the toolchain compares and
   prints; its L identity is `String` and its realization the atom,
   assigned at phase 3 with `String`'s E realization (§11). L needs no
   `Init`: K checks any L form in any file (the loader's `basic` pin
   declares its own `inductive Nat` and imports nothing). What is
   decided per declaration is a `type`'s L half: a `type` enters K and
   E when its field types are L types, and is **E only** (`RUNNABLE
   type`) when a field cites a built-in with no L constant in scope —
   `Int` or `Nat` without `Init`, `Symbol` until its `String` identity
   — or another E-only type, transitively; the type being declared is
   not counted against itself (`loader.shard` `type_is_e_only`; §13
   item 35). Never the directory and never the file's imports. K's own
   sources see no `Init` by nature (K reads the export as data): their
   `type`s over `Int` and `Symbol` are E only, their field-less and
   type-parametric types are K's inductives, and their `fn`s enter L at
   the flip, as §6.1 says. A type name resolves to the built-ins first,
   then to K's constants through the scope (an ambiguity between a
   native and an imported type is reported there), then to the E-only
   types of the scope (`classify.shard` `type_ident`).
2. **Literals in E.** A numeral is an integer of any sign — `-7` is a
   numeral — and its E type is the binder's, `Int` or `Nat`; the
   reader's literal kind is the sign (`LNat` at or above zero, `LInt`
   below; a non-negative numeral types as `Nat` for the static pass),
   so §10 item 1's omission list loses its entry: the kind is in the
   text. In a `realize` body a numeral is K's `Nat` literal in the
   equations and a negative one has no L identity yet
   (`no_l_identity`; Stage 1's numeral rule, §11). `"…"` is the byte
   list of its UTF-8 encoding as the constructor chain of the `List`
   in scope — the type named `List` the scope resolves, its first
   constructor nil and its second cons: the prelude's in the toolchain,
   `Init`'s in a program that opens it — carried as one literal naming
   those two constructors (`prog.shard` `LStr`) that the linker builds
   once; a string literal counts as a citation of `List` (an `Init`
   inductive registers on it), and a scope without a two-constructor
   `List` refuses the literal (`no_list`). This is the wire's
   convention (§6.7) until `String`'s E realization at phase 3, when the rule flips for every file at once and the toolchain migrates by
   tool (§11). **Flipped at slice 3.18, staged** (§8.4 slice 3.18 rule
   3; §13 item 37 as amended): outside the toolchain's own sources
   `"…"` is a `String` in every position, its E value its UTF-8 bytes
   as `Init`'s `List` cells; the rule above holds in a module whose
   identity begins with `kernel` until route 1's chain is V3's own (re-attributed 2026-10-16, §13 item 69: until Init's types are in the toolchain's closure — phase 5's opener design; the chain reads the emitter's output). `(quote x)` and `'x` are `Symbol` literals; `(list a b
   c)` is the same `List`'s constructor chain (R60's list literal: by
   the scope at Stage 0, by the expected type at Stage 1). In L
   positions nothing changes: a numeral is `LitNat` and a negative one
   is refused (`negative_numeral`), a string is `LitStr` (§5.3).
3. **Binders.** Type parameters are explicit `((T Type) …)` binders in
   every `fn`, `extern`, `sig fn` and `realize` (law §5.3 departure 4,
   now for the toolchain too); the parenthesized head `(fn (append T)
   …)` and the auto-bound bare type variable are gone.
4. **Scope.** `import` never opens and `use` does (§3.1), for every
   file; a `type` opens its own namespace for its **whole** file, its
   constructors pre-registered like the file's heads (§13 item 7 as
   amended at slice 3.4), and another module's constructors are cited
   through `(use M.T)`; `(use M)` opens no constructors (§13 item 7,
   RULED 2026-09-15). The toolchain's files carry `use` lines — first
   written by the migration tool as the flat mirror (slice 3.3), then
   pruned to the lines a citation could need (slice 3.7): the loader
   records `UNUSED MODULE use=P,…` for a `(use P)` no symbol token of
   the file names a declaration under, decided against K and the E
   table once the file is loaded — a lint, never a refusal, and the
   prune tool deletes exactly those lines (979 kept of 1,197: 357
   module opens, 623 type opens). The bootstrap ignores `use` and resolves flat (as it
   does today); the V3 reader enforces the scope, and frontend parity
   is the tie, as for every rule the bootstrap under-checks: the
   bootstrap is E's executor, the V3 reader its gate (§10).
5. **One primitive table.** The operator spellings — `+ - * / mod tmod
   ediv band bor bxor bshl bshr = sym_eq < <= sym_of_chars
   chars_of_sym` (`int_eq lt le` before slice 3.17, and still in the
   toolchain's own sources; §8.4) — are E's `Int` and `Symbol` primitives in every file
   (§2 lexes them), beside the naming-law identities (`Nat.add`,
   `Int.tdiv`, …; §6.4). Their L identities are assigned at phase 3
   with law §10.3's table (`/` stuck at zero stays a distinct entry
   from the total `Int.tdiv`, as §6.4 says); a `realize` body cites the
   L-identified entries only (`no_l_identity`, unchanged).
6. **The wire and the prelude.** Unchanged: `kernel/prelude.shard`'s
   `List`, `Option`, `Bool` and `Pair` are the wire's cells, interned
   by the linker whether or not a program declares them (§6.7); the
   prelude's `Nat` (`Z`/`S`), which no V3 file used, is deleted (slice
   3.4) — `Nat` is a built-in (rule 1). Since slice 3.18 these are the
   cells of the toolchain's signatures: a program under the naming law
   declares its externs over `ByteArray` and `Init`'s types, and the
   driver reads either by the declared type (§8.4 slice 3.18 rule 4).
7. **`let`** is sequential everywhere, the bootstrap included (§5.4's
   measurement: no existing source changes meaning).
8. **The bootstrap reads exactly this — landed at slice 3.2
   (2026-09-14).** Its delta from the profile reader, five reader rules
   in `rust_bootstrap/src/load.rs`, `eval.rs`, `dump.rs` and
   `bin/eval.rs`: a binder whose type is `Type` is a type parameter in
   scope for the binders after it and the result, never a runtime
   parameter — wherever `Type` is the sort, that is, not a declared
   type of the closure (the old tree's `kernel/module.shard` declares a
   data type `Type` and binds `(t Type)` runtime parameters, which the
   first cut dropped: calc's differential, which runs the old kernel on
   the bootstrap, caught it; the parameterized head and the auto-bound
   bare name stay accepted for the old tree); `let` binds in order, in the loader and
   the lowerer alike, so the dump prints indices as loaded; a directory
   import is followed to the module's view (its transparent `type`s)
   and then to `DIR/BASE.shard`, each with its own closure (§13 item
   26's resolver change; the view's `sig` forms are skipped, so a
   consumer's call resolves flat to the implementation's `fn` of the
   same name — the substitution §6.7 performs by identity); a file's
   L forms (`def theorem abbrev opaque inductive structure realize
   trusts`) are skipped, since under the one E they sit beside the E
   forms K reads; a dotted citation — `Stack.mk`, `json.hex_val` —
   canonicalizes to the declared name it ends in, the flat mirror of
   §6.7's suffix table (the bootstrap has no module paths), a citation
   matching nothing kept as written. `use` stays ignored. The dump
   prints `Nat` as a built-in beside `Int` and `Symbol` (rule 1) and a
   type's head by its last component. Frontend parity (§10 item 1)
   stays the gate that keeps its parse honest: 21 closures since 3.2,
   the 21st an S closure with a directory module, a theorem beside its
   E forms and a dotted constructor citation (`v3/pins/loader/
   view_basic`), tied byte for byte. Not in the bootstrap: the
   naming-law primitive identities (`Nat.add`, …; rule 5) — those are
   `ev`'s entries, computed through the bootstrap's operator primitives
   when `ev` runs on it; a toolchain source cites the operator
   spellings, and one that cited `Nat.add` directly would be an unknown
   call under `eval direct`. **Route 1 reads the same E** (2026-09-14,
   after CI): the compiled K is built by the old tree's chain
   (`v3/build.sh`: `tools/lower` on the compiled engine, whose front end
   is the old `kernel/reader.shard`), so that reader carries the same
   binder rule — `(T Type)` is a type parameter unless a data type
   `Type` is in scope, which in that reader means its constructors
   `TCon` and `TVar` from the old tree's `module.shard` are
   (`parse_param_items`, `type_is_sort` over the constructor scope; a
   first cut keyed on the current file's typedefs mis-read every other
   old-tree file, and the Rust-hosted tower caught it where the
   compiled engine, whose loader is compiled in, could not). The local
   suite never builds route 1; CI's `v3/build.sh` is its gate, and it
   caught the omission on slice 3.3's pipeline.

### 8.2 The profile, retired (2026-09-14)

The toolchain profile differed from S in ten rows — the prelude's
names, auto-bound type variables, `"…"` as bytes, `Int` numerals with
`-7`, symbols, `(list …)`, the flat scope, the operator primitives,
`type` as E only, the `kernel/` and `meta/` directories as the
selector. Each row was decided into a rule of §8.1 at slice 3.1 and
the code that carried the distinction — the reader's `profile` flag,
`profile_form`, the layout rule, `MODULE … profile=` — was deleted at
slice 3.4. The table itself is history: records §9 (slice 3.4) keeps
it, and `git show 332b208:v3/LANGUAGE.md` holds the last text with it.

### 8.3 The phase-3 opening slices (the order parity allows)

Every step keeps frontend parity, route 2's byte-tie and T0 green on
the toolchain's closures, so both readers agree at every commit:

- **3.1 the design on disk** — this section, §13 items 34–38, the law's
  §9.2 amended, the plan in `v3/README.md` (2026-09-14).
- **3.2 the bootstrap to the one E — landed 2026-09-14** (rule 8):
  `((T Type))` binders as type parameters (both forms accepted while
  the toolchain migrates), sequential `let`, a directory import
  followed to the view and the implementation, the L forms skipped,
  dotted citations canonicalized; 43 bootstrap tests; parity 21
  closures with the directory-module case.
- **3.3 the toolchain migrated by tool — landed 2026-09-14.** The
  reader first: `(T Type)` is a type binder in every file (the profile
  reader had auto-bound `Type` itself as a variable and kept `T` as an
  argument — a silent arity error parity would have caught). Then one
  deterministic pass over the 50 files under `v3/kernel/` (the kernel
  and its tests; fixtures excluded), gated as a whole by parity, route
  2 and the suite rather than file by file, since the pass is
  mechanical: 1,195 `use` lines in 49 files — one `(use M)` per module
  M of the file's transitive import closure and one `(use M.T)` per
  type T of a closure module whose constructors the file cites bare
  (§13 item 7: a constructor resolves through its type's opened
  namespace; `Cons` needs `kernel.prelude.List` opened, `SNum` needs
  `kernel.sexpr.SExpr`) — the exact mirror of the flat scope, every
  visible declaration reachable; and 13 functions in three files
  (`util`, `intmap`, `nested`) given explicit `(T Type)` binders — the
  sizing's 210 had counted `(a A)` binders over declared types. Both
  readers accept the result: the bootstrap ignores the `use` lines and
  reads the binders (3.2); the profile reader reads the binders and
  keeps resolving flat. Open for ratification with §13 item 7: whether
  `(use M)` should also open M's types' constructors — Lean's `open`
  does not, and the per-type lines are the honest count of what the
  flat rule was hiding; a prune to the prefixes each file actually
  cites is a follow-up once the S reader can report an unused `use`.
  The prelude's `Nat` is deleted at 3.4, when `Nat` becomes a built-in
  of the profile reader too.
- **3.4 the V3 side to the one E — landed 2026-09-14.** The `profile`
  flag deleted from `sexpr`, `classify`, `etable` and `loader`
  (22, 77, 8 and 20 sites), rules 1–2 for every file
  (`string_literal`, `symbol_literal` and `list_sugar` retired, their
  pins turned positive), `profile_form` and the layout rule gone,
  `MODULE … profile=` gone, the prelude's `Nat` gone. Three findings
  on the way (records §9): the first cut keyed "E only" on `Init` in
  scope and the `basic` pin — its own `inductive Nat`, no imports —
  killed it, so the rule is per `type` by its fields (rule 1); a
  `type` opened its namespace only for the rest of its file while
  heads were pre-registered for the whole file, so the kernel's
  forward constructor citations failed until every `type` opens at
  pre-registration (rule 4); and a type name resolved through the E
  table before K's constants, which hid the native-versus-imported
  ambiguity `same_spelled` pins. Parity byte-identical over the 21
  closures, route 2 and calc byte-identical, 22 entrypoints, 0
  failed; the V3 loads ran about twice as long (parity 95 s against
  44 s) — see 3.6.
- **3.5 the documents closed — landed 2026-09-14** with 3.4: §8.2
  reduced to the pointer above, §6.7's profile paragraph replaced,
  §12's rows, §13 items 7 and 35 amended, `CANON.md`'s lexical note,
  `docs/TCB.md`'s bring-up item (2), the kernel README, the pins
  README, records.
- **3.6 the load time recovered — landed 2026-09-14.** The doubling
  was measured before it was explained: the guessed cause — fifty
  candidate names built per citation for the scope check — was
  replaced by a structural check on the hit's identity
  (`etable.shard` `cited_in_scope`: the prefix above the cited suffix
  is the module's, an opened prefix's, or empty) and parity did not
  move (93 s); the cause was `type_ident`'s L resolution per binder
  and field type, which the profile never paid — 11 s against 6 s on
  one closure. The order K-first is needed only where an imported
  type can stand beside a native one, which needs `Init` in scope, so
  it applies exactly there (`init_seen`) and the E table goes first
  elsewhere, with the same answers (the pins, `same_spelled` among
  them, unchanged). Parity 53 s; the structural check stays as the
  simpler code. The `use` lines are still the flat mirror; a prune to
  the prefixes each file cites remains open with §13 item 7.
- **3.7 the `use` lines pruned — landed 2026-09-15.** The loader
  judges each `(use P)` of a file once the file is loaded: unused
  when no symbol token of the file's forms names, under P, a
  declaration K or the E table holds (`loader.shard` `unused_uses`;
  the `UNUSED` record). The test is the flat mirror of resolution —
  a citation is a suffix of its identity and the opened prefix is
  what stands above it — taken over every token rather than every
  citation, so a binder spelled like a declaration keeps a line and
  never drops one; the E table's suffix index answers each token
  with its hits and the native K declarations are bucketed by last
  component, so the check costs nothing measurable (parity 59 s
  against 51 s before it, 80 s under a first cut that scanned
  prefixes against tokens). A one-off tool ran the 20 toolchain
  closures, checked that every closure judged each file the same
  (50 files, 0 disagreements) and deleted the flagged lines: 218
  from 38 files, 979 kept (357 module opens, 623 `(use M.T)` type
  opens — item 7's cost, now counted). Gates: the 90 loader pins,
  parity byte-identical over 21 closures, route 2 and calc, 22
  entrypoints, `v3/build.sh` and the compiled `t0`'s fixture tie; a
  second run of the tool finds nothing (the fixpoint).
- **3.8 the K seal — landing 1 of 3, 2026-09-16 (§13 item 26 as
  ruled; §6.6).** The nine trust files moved to `kernel/k/` and their
  paths and `use` lines rewritten (22 files); the vocabulary of K's
  answers split out to the public `verdict.shard` — `Outcome` made
  parametric, `(Outcome E)`, so a client matches on it without naming
  `CheckedEnv`; `RawEnv`, a type nothing used, deleted; the
  sealed-directory rule in the loader with its two pins; the leak
  pins moved inside the directory. Gates: 92 loader pins, parity
  byte-identical (the move is invisible to the projection), route 2
  and calc, 22 entrypoints, `v3/build.sh` and the compiled `t0`'s tie.
  A rough edge found on the way: an implementation whose `type` is E
  only (a built-in field with no L constant in scope) cannot stand for
  a `sig type`, and the refusal reads `bad_shape: unknown_constant`
  rather than naming the cause; noted for the view landing.
- **3.8 the K seal — landing 2 of 3, 2026-09-17 (§13 items 39–41;
  §6.6).** The facade withdrawn on a fact the design missed — a
  `sig type` is matched only by a declaration with the view's
  identity — and one module across the sealed directory ruled in its
  place: `Scope` carries the identity prefix beside the module tag,
  the fork check matches by lookup once an import has loaded the
  declaration, the E signatures compared. The view `k/mod.req.shard`
  (seven sig types, sixty sig fns), `k/k.shard` the nine imports,
  every consumer through `(import "k")`, the internals' tests at
  `k/test/`. Three more findings, each from a gate: a `sig fn` over
  E-only types is unreadable in K — an E parameter (`sig_e_only`); my
  landing-1 refusal of an E-only implementation type was wrong, since
  K's own types are E-only — matched by the E type's arity, and the
  pin `impl_e_only` flipped to `ok`; `erase.shard` used the
  constructors of `Ctx` and `Loc`, which the view hides — `ctx_new`,
  `loc_ctx`, `loc_st`, `loc_fvar`. The leak pins retired (the seal
  closes the case structurally). Gates: 92 loader pins (K's own check
  discharges 67 parameters, 0 errors), parity byte-identical over 21
  closures and 69,031 lines with K's directory as the consumers'
  second root, route 2 and calc, 22 entrypoints, `v3/build.sh` through
  the old chain with `(import "k")` and the compiled `t0`'s fixture
  tie. Landing 3: the hostile battery as a client through the view,
  the R52 client fixture, the documents.
- **3.8 the K seal — landing 3 of 3, 2026-09-17: the seal complete
  (§13 item 26).** The hostile battery split at the seal: 45 cases
  outside as a client of the view, the matcher's nine inside
  (`accel_test`); `Limits`, `check_with`, `limits` and `nm_string`
  the last additions to the view (eight sig types, 61 sig fns);
  `ConstantVal`'s accessors to `decl.shard`; the client fixture
  `k_client_test` and the pin `k_client_reach` (the pins test gained
  a `;; root:` header for a case that must sit under the package
  root). Gates: 93 loader pins, K's own check 69 discharges and 0
  errors, parity byte-identical over 23 closures and 80,433 lines,
  route 2 and calc, 24 entrypoints, `v3/build.sh` and the compiled
  `t0`'s fixture tie. Next: Stage 1, I.
- **3.9 records — the form, 2026-09-17 (§4, §13 item 42; the first of
  the four follow-ups ruled after the seal: records, a public fixture
  kit, K's clients through the V3 loader at test time, the phase-3
  consolidation).** v2's `(record …)` with its `make` and `with`
  sugar, expanded at the s-expression level in `kernel/record.shard`
  at the loader's two read sites and in the bootstrap's `load.rs` the
  same way; pins `record_basic` (a plain, a `(ctor …)` and a
  parametric record, `make` in any order, `with` chained) and
  `record_make` (a field missing); parity ties the closure between the
  two readers. The accessors landed first under v2's names
  (`FIELD_of`, `with_FIELD`) and were reversed the same day at the
  first real file: the loader's four records share four field names
  and six of their accessor names collide with functions of the
  closure — so the accessors live in the record's namespace
  (`Load.env`, `Load.with_env`; `with` names the record), the dump
  prints a head by its declared spelling (the identity beyond the
  module tag) so the parity projection stays injective, and the
  bootstrap resolves a bare citation of a dotted head by suffix as
  the V3 reader's opened namespace does. Found on the way: three of
  the new file's names collided with the closure's (`fields_of`,
  `Rec`, `RcRes`), one with a pattern variable in the loader (`syms`,
  `pattern_name`), and the generated patterns had bound the field
  names, shadowing the accessors the namespace opens (`fld_FIELD`
  now) — the flat bootstrap shadowed silently where the V3 gate
  refused. Gates: 95 loader pins, parity byte-identical over 24
  closures and 80,705 lines, 24 entrypoints. **Landing b, the same
  day: the loader's `Load` (14 fields), `Fx` (9), `Mod` (7) and `Im`
  (6) as records** — constructions by `make`, the mutators by `with`,
  the hand accessors gone and their 36 call-site names rewritten to
  `Load.env`-style citations across four files. The first migration
  hit eight `pattern_name` refusals — the automatic open of a type's
  namespace had made every field name a reserved word of its file —
  so that open now covers constructors only (item 7 amended). Gates
  unchanged: 95 pins, parity over 24 closures and 80,900 lines, 24
  entrypoints; the lint quiet.
- **3.10 the test kits, 2026-09-17 (the second of the four
  follow-ups).** Two public files under `kernel/test/`, importable on
  both sides of the seal: `case_kit.shard` (`Case` and `run_cases`,
  the report loop seventeen tests had copied) and `decls_kit.shard`
  (the raw declarations of K's tests — `Nat`, `Eq`, the term
  builders, `env_after`/`accepted`/`refused` over any environment
  type — under the kit's own name constants, since the view's
  `nm_nat` and tc's are two identities no test may see both of).
  17 tests on the case kit, 8 on the declarations kit; the
  matcher test inside the seal and the battery outside now share
  their fixtures; `inductive_test` keeps its own variants of three
  builders. 288 lines gone, then the lint's 40 `use` lines. Gates:
  24 entrypoints, parity byte-identical over 24 closures and 81,145
  lines, 95 pins.
- **3.11 K's tests through the V3 loader, 2026-09-17 (the third
  follow-up).** `kernel/test/k_clients_test.sh` loads each K-facing
  test with the V3 loader and runs it under `ev` (`load.shard --run`):
  the two clients outside the directory with `v3/kernel/k` as a second
  root, the five tests inside with the implementation in their
  closure — seven tests in 46 s. Every other entrypoint runs on the
  bootstrap, which resolves flat and enforces no seal, so this is
  where the sealed-directory rule, the view and the E-side visibility
  are load-bearing at test time rather than only at parity.

- **3.12 Stage 1's design on disk, 2026-09-17 (§8.4, §8.5; §13
  items 43–46).** The two guards carried from phase 2 stated; the
  Stage-1 slices cut; the view stance ruled.
- **3.13 `fn` = `def` + `realize`, 2026-09-17 (§8.4, §8.5; §13 items
  43–46 as built).** `kernel/define.shard` and the loader's definition
  path: a classified `fn` whose signature's types and body's heads
  have L identities gets a `DefnDecl` K checks — the leading case tree
  as nested recursors, the structural parameter's split at the top
  with the other parameters abstracted in the motive, an
  immediate-field self-call the induction hypothesis — then its
  equations `NAME.eq_N` by `Eq.refl` through the realize path, the E
  body attached, `DEFINE` recorded; an obstacle leaves it `RUNNABLE
  … why=…`; an eligible failure refuses it. The operator identities
  by operand type and the one `Nat → Int` coercion on numerals; a
  `def` written in the E forms; a definition displacing the view's
  parameter in the fork (`DISCHARGE … defined`); the translation state
  a record. Eight pins (`define_basic`, `define_depth`,
  `define_nat_operator`, `define_runnable`, `define_refused`,
  `def_match`, `view_eq_hidden`, `view_rfl`; the pins stream the
  export through `Int.decEq` now), `kernel/test/define_test.sh` over
  `v3/std/list.shard` (the first std file: `List.sum`, its equations,
  two theorems) and calc's `eval` with its first claim `eval_add`.

- **3.14 course-of-values, measures and the obligation class,
  2026-09-17 (§8.4's slice-3.14 rules as built; §13 item 47).** Every
  structural recursion is `brecOn` with the table generalized at each
  split (3.13's direct recursor deleted); `Init`'s `below`/`brecOn`
  at solved levels, a native inductive's generated on first need and
  admitted first (`DfAux`); a measure is `WellFounded.fix` over the
  parameter tuple under `InvImage Nat.lt_wfRel.rel m`, each self-call's
  decreasing fact an obligation `f.dec_N` admitted as `PARAM …
  obligation` (the fourth policy class, §9), `DEFINE f equations=
  pending=f` and `PENDING f measure`; an `Int` measure, a table
  without `PProd` in scope, a forward or mutual reference are
  obstacles. Four pins (`define_below`, `define_measure`,
  `define_measure_int`, `define_mutual`), `define_depth` now defined
  with its third equation cited; calc's `parse_rest` and `parse`
  defined; the Int fixture through `InvImage.wf` (100,851 lines).

Then, as planned: the rest of Stage 1 (§8.4's slices), then I.

### 8.4 Stage 1 — one grammar, one elaborator, `fn` = `def` + `realize` (phase 3, RULED 2026-09-17)

**The two guards** carried from the phase-2 direction checkpoint
(records §9, 2026-09-13), stated here before code as that ruling
asked:

1. **`def` and `fn` share one term grammar and one elaborator.** The
   surface term language is the union of §5.1's explicit L forms and
   §5.4's E forms — `match`, `let`, `if`, numerals, a bare constant
   head with its type arguments omitted — and one elaborator turns it
   into L: `realize.shard`'s translation (`tr`, §7.5), which already
   solves type parameters by first-order matching against the callee's
   telescope and states a `match` or an `if` as the recursor with a
   constant motive. A `def` may be written in the E forms; a `fn` is a
   `def` whose body also passes the classifier (§6.3) and whose
   realization is attached. If phase 3 elaborated `fn` bodies and left
   `def` bodies in explicit L there would be two dialects for real.
2. **The toolchain profile was bring-up, never a dialect** (§8.2): its
   spellings never gain L identities of their own. Rule 3 below
   assigns identities to the operator spellings by the type they are
   applied at, which is the naming-law identity under another
   spelling, not a new one.

**The slices**, in the order parity allows, each keeping frontend
parity, route 2's byte-tie and T0 green:

| slice | content | first consumer |
|---|---|---|
| 3.12 | this section, the guards, §8.5, §13 items 43–46 | the ratifier |
| 3.13 | `fn` = `def` + `realize` for non-recursive bodies and immediate-field structural recursion; the operator identities by operand type; a `def` in the E forms — records §9.1 "Slice 3.13" | `examples/calc`'s first claim; `v3/std/list.shard` under the naming law (R55's small library: an operation and a theorem) |
| 3.14 | deeper structural recursion by course-of-values (`below`/`brecOn` generated per inductive, as Lean's elaborator does); `(measure E)` to `WellFounded.fix`, the measure a reported obligation; mutual recursion — records §9.1 "Slice 3.14" | every ported `fn` with a measure — a quarter of the old tree's 13,229 |
| 3.15 | the L-side ergonomics of law §5.1–5.2: implicit arguments, universe inference, the numeral rule with the one `Nat → Int` coercion, `noConfusion`/`injection` — records §9.1 "Slice 3.15" | `def` and `theorem` authors |
| 3.16 | one front end (reordered 2026-09-18; the design in the record): a `fn` body through the elaborator into a pre-definition, its E program by erasure and its K value by the 3.13–3.14 compilation; matchers, so a `match` in a statement; `Bool` against `Decidable`; constructors by expected type; list literals — records §9.1 "Slice 3.16" | calc's spec file wholly defined (13 of its 25 functions are `RUNNABLE` at the slice's opening) |
| 3.17 | the porting facilities of §11's R60 row (narrowed by ruling 2026-10-01, item 50): the record laws and `make`/`with` over a `structure` with the dependent-update refusal, numeral rows on the typed route, the fresh-name supply, E's rename of `lt le int_eq` to `< <= =` staged by the tool (the files no v2 tool reads now, `v3/kernel/**` with route 1's chain) — records §9.1 "Slice 3.17" | the broad port |
| 3.18 | bytes and text (narrowed by ruling 2026-10-01, item 51; the design and the as-built in the record): the registry's type rows — `String`, `ByteArray`, `UInt8` and the structures under them, each represented by its one runtime field —, a projection function as its projection, item 37's flip staged (every file but the toolchain's own), the wire read by declared type and the externs over `ByteArray`; symbols and `Name` literals deferred to the toolchain's port — records §9.1 "Slice 3.18" | the first host-facing S library (`v3/std/host.shard`, `v3/std/bytes.shard`); calc's loop against the host |
| 3.19 | deriving under a declared policy (law §5.1; split from 3.18 by the same ruling; ruled 2026-10-02, item 52; the design and the as-built in the record): `(derive TYPE CAPABILITY…)` — equality, an ordering under the one policy `structural`, a rendering in canonical S — generated as `fn` source and found through a derivation table; a hand-written procedure registered with `(CAPABILITY by NAME)` — records §9.1 "Slice 3.19" | a new type usable in an `if`, as a key and in a message (`v3/examples/derive/`); `v3/std/derive.shard` |
| 3.20 | I's opener (ruled 2026-10-05, item 53; the design and the as-built in the record; **landed 2026-10-06**): I's data and `(by STEP…)` in a `theorem`, `elaborate(I)` to P, the forms with no engine behind them — `intro exact rfl have show apply cases induction wf decide unfold reduce rw sorry` — and `goal_of`/`applicable`/`step` as E functions over one derivation — records §9.1 "Slice 3.20" | calc's claims whose lemma closure needs no arithmetic (52 of 100; 44 landed as blocks in five files, `kernel/test/tactic_test.sh`) |
| 3.21 | the two forms that carry an engine (the design and the as-built in the record, item 54; **landed 2026-10-06**): `simp_only` (a bounded rewriter over a lemma list) and `arith` (the Farkas certificate elaborated through `Lean.Omega`'s lemmas, in the export at line 76,223; reconstructed by elimination where the node gives none) — records §9.1 "Slice 3.21" | calc's 100 claims as theorems, the capstone `run_eq_spec` among them (`kernel/test/tactic_test.sh`) |
| 3.21b | GPT-6's trajectory review, findings 1–3 (ruled 2026-10-06, item 55; the design and the as-built in the record; **landed 2026-10-06**): the expected type decides an arithmetic operator; Init's visibility is the module's horizon; `reduce` succeeds unchanged where nothing reduces — records §9.1 "Slice 3.21b" | three pins; `op_expected` proves `(- 1 2) = -1` at `Int` |
| 3.22 | the producers (ruled 2026-10-07, item 56; the design and the as-built in the record; **landed in three landings, 2026-10-07/08; closed on pipeline 552**): `auto` with sidecar replay at build and search only in `prove`; `(arith only …)` and the witness goal; the engine as an E library over the fixed API; the pin store in K's export format with `verify_release` — records §9.1 "Slice 3.22" | the engine's count over calc's 100 claims; `v3/examples/auto/` with a machine-owned sidecar; calc's release bundle verified on CI |
| 3.23 | the Init cache (ruled 2026-10-08 on GPT-6's R75, the feedback cost; item 57; the design and the as-built in the record; **landed 2026-10-08; closed on pipeline 554; retired at slice 3.26 landing 3, 2026-10-09** — a load checking the closure it cites, the receipt saved a tenth to a fifth of it): the loader admits the pinned export under a T0 run's receipt, the typing judgments skipped — K's verdict cached, no second format, the release gate untouched — records §9.1 "Slice 3.23" | a load's Init cost and the loader pins' wall clock |
| 3.24 | the connected path (ruled 2026-10-09 on GPT-6's R76; item 58; the design and the as-built in the record; **landed 2026-10-09; closed on pipeline 556**): `(dif h C T F)`, the dependent if with its hypothesis named — the branch-local proof joint; `examples/path`, law §12.4's first connected path assembled as one test and broken at each joint; its cost measured — records §9.1 "Slice 3.24" | the one phase-3 joint still missing; the composition cost before the broad port |
| 3.25 | the shared Init load (ruled 2026-10-09 at the boundary after R75/R76, item 57's stated lever; item 59; the design and the as-built in the record; **landed 2026-10-09; closed on pipeline 559**): the Init stream replayable — streamed once to its end, every root of a process loaded fresh on it; the module's horizon the only measure of what Init it sees, Init's names Init's; the loader pins' entrypoint from 783 s to 33 s — records §9.1 "Slice 3.25" | the suite's wall clock before the library arc multiplies the pins |
| 3.26 | Init on demand (ruled 2026-10-09 at the boundary after 3.25, on the library arc's first question; item 60; the design in the record; **landings 1–2 landed 2026-10-09, green on pipelines 561 and 563; landing 3 landed 2026-10-09 — the receipt dropped, the user's ruling; closed on pipeline 565**): the export indexed once per environment — a record table and a name table, read by range, never loaded —, a module's closure of what it cites read and fed to K in export order, the horizon law unchanged, Init's names Init's from the index, no fixture — records §9.1 "Slice 3.26" | every citation of Init costs its closure, under one percent of the export; the library arc cites Init wherever Init has the statement |
| 3.27 | the library arc (designed 2026-10-09 at the boundary after 3.26, on the user's "continue with the next arc"; item 61; the design in the record; **ratified 2026-10-10 as designed, with calc's `list.shard` retiring onto `std/list` at landing 3 and the migration tool's tier 0 moved to phase 5; landing 1 built 2026-10-10, green on pipeline 570; landing 2 built 2026-10-10, green on pipeline 572; landing 3 built 2026-10-10, green on pipeline 574; landing 4 built 2026-10-10 — `docs/LEAN.md`, T9 small PASSED, green on pipeline 576; closed 2026-10-10, the as-built notes ratified**): Init is the library — a statement Init has is cited by its name, `v3/std` declares what Init lacks, an old module whose declarations are all Init's becomes a migration record; the fifteen former axioms as theorems (twelve by Init's names, measured); the seams the probes found closed — one spelling at the unifier and the matcher, instances by table, `/` at `Int` Euclidean, the horizon refusal's pointer; `std/list` with the dependent match and the matcher, `std/bits` as bridges; `docs/LEAN.md`; T9 small; T2/T3 and T10 as slices 3.28 and 3.29 — records §9.1 "Slice 3.27" | the bulk port cites this library; calc's `list.shard`; T9's author |
| 3.28 | the lambda profile (designed 2026-10-10 at the boundary after 3.27, on the user's "continue to the next slice"; item 62; the design in the record; **ratified 2026-10-11 as designed; landing 1 built 2026-10-11, green on pipeline 581; landing 2 built 2026-10-11, green on pipeline 583; landing 3 built 2026-10-11, green on pipeline 585; the as-built departures ratified 2026-10-12 — closed**): a function-typed binder is a static parameter and its `fn` a template; instances by shape — the identity and the captured count, never the captured values —, closed lambdas lifted to `CALLER.lam_N`, named partial application, a generated definition per instance with the template's equations instantiated; the shape set closed and a budget; static law-bearing packages as structures with function-typed fields, never E types; `List.map`, `filter`, `foldl`, `foldr`, `any`, `all` as templates in `std/list`; T2 and T3 as `template_test.sh` with the artifact check — records §9.1 "Slice 3.28" | law §5.3's `add_offset` written as the law writes it; the first static package; the bulk port's higher-order list code |
| 3.29 | the tactic library — T10, the proof IR's gate (designed 2026-10-12 at the boundary after 3.28, on the user's "proceed with the next slice"; item 63; the design in the record; **ratified 2026-10-12 as designed; landing 1 built 2026-10-12, green on pipeline 588; landing 2 built 2026-10-12, green on pipeline 589 — the slice closed 2026-10-13, the as-built readings ratified 2026-10-13**): a tactic is a `fn` of an ordinary module over text — the goal in as one S form with its named context, target and the applicable forms, a block of I out — named by the source's delegation `(auto (tactic M.FN) HINT…)`, run by `prove` under `ev` and its answer replayed through `tac_prove` before the theorem is accepted, the sidecar entry it writes indistinguishable from the engine's; the sidecar's schema form `(sidecar N)`; the guarded occurrence `(occ K EXPECT)`; dependents reverted into the motive under `cases` and `induction`; the refusals with the library pointer; the T10 record — records §9.1 "Slice 3.29" | `v3/examples/tactic/` — `bool_split`, a decision procedure for `Bool` goals as an E library, its theorems solved twice (the engine and the library) and both sidecars replayed, stored and verified by K alone; `tactic_lib_test.sh` |
| 3.30 | the phase-3 close-out (designed 2026-10-13 at the boundary after 3.29, on the user's "Let's proceed to the next slice"; item 64; the design below; **ratified 2026-10-13 as designed; landing 1 built 2026-10-13, green on pipeline 593; landing 2 built 2026-10-13 — T9 again passed on the count, two kernel findings fixed, green on pipeline 595; landing 3 built 2026-10-13 — the consolidation; landing 4 built 2026-10-13 — the close: the pass done, the ledger closed, phase 3 closed**): law §12.4 item 3's list audited against the tree — every item covered, the gates T2, T3, T9 small and T10 met — and what the close owes: the gate ledger in records §9 with §11's phase-3 rows rewritten; the carried T5 fixture as pin `realize_imported` (measured: the imported theorem cited before and after a supplied realization, every record `ACCEPT`); the two rows still open for ruling (the two-instance gate, the `String` flip); the phase-3 doc banners; T9 again — nine tasks, the count agreed before the run (landing 2); the LANGUAGE.md consolidation ruled 2026-09-17, its shape for ruling (landing 3); the ratification pass over items 39–42, 44–46, 48 and 64, GPT-6's checkpoint SHA, phase 4 opened (landing 4) | the ratifier; phase 4 |

**The facilities, their rules' current statements (slice 3.30 landing
3, 2026-10-13).** The design and as-built texts of slices 3.13–3.29
moved verbatim to records §9.1 ("Slice 3.N" there, one heading per
slice); what stays here is, per facility, the current statement of
each rule with its as-built note beside it, numbered as its slice
numbered it, so that `§8.4 slice N rule K` — the kernel's comments
and messages, `LEAN.md`'s rows, the pins' READMEs — names a rule here
and its full text in the record. A rule another slice superseded says
so and points on. Slice 3.30, the close-out in progress, keeps its
text below until the close.

**Definitions — slices 3.13–3.16: `fn` = `def` + `realize`, the one
front end** (records §9.1 "Slice 3.13" to "Slice 3.16"; §13 items
43, 47, 48 and the single-frontend memo R63–R71).

*Slice 3.13 (ruled 2026-09-17, item 43):*

1. **Eligibility is automatic and transitive**, as a `type`'s is. A
   `fn` gets its L half when every binder and return type has an L
   identity and every head of its body does; otherwise it stays
   `RUNNABLE` and the record names the first obstacle; once eligible,
   a body that fails translation or K is a refusal, never a fall-back
   to `RUNNABLE`. As built: the obstacles are `e_only_type`,
   `no_l_meaning`, `no_l_identity`, `unknown_constant`,
   `measure_pending`, `numeral_pattern`, `recursion_depth` — `RUNNABLE
   fn NAME why=OBSTACLE` — and every other failure is `REFUSE NAME
   reason`; the equations are stated only where `Eq` and `Eq.refl`
   are in scope; the `fn` enters the E table before its definition is
   attempted, so a self-call types statically.
2. **The definition's value** — the direct-recursor path is superseded
   by 3.14 rule 1; what stands: the equations `NAME.eq_N`, one per
   case-tree leaf (§7.5), proven by `Eq.refl`; the body attached
   through the realize path, so `ev` runs it under the L identity and
   `REALIZE`/`PENDING` are the same for every function; the record
   `DEFINE NAME equations=…` beside the `ACCEPT`s; a self-call not on
   a field of a matched parameter is `realize_descent`.
3. **Operator spellings get L identities by operand type, only where
   `ev` already agrees with K**: `+ *` at `Nat` are `Nat.add`,
   `Nat.mul`; `+ - *` at `Int` are `Int.add`, `Int.sub`, `Int.mul`;
   `= < <=` are `Nat.decEq`/`Nat.decLt`/`Nat.decLe` and the `Int`
   three; `-`, `/` and `mod` at `Nat` were refused `nat_operator` with
   the pointer (`Nat.sub`, `Nat.div`, `Nat.mod`) — **superseded on the
   typed route at 3.16 rule 4**: they resolve to `Nat.sub`, `Nat.div`,
   `Nat.mod` (saturating, total at a zero divisor; `elab.shard`
   `op_at`), the E-first route keeping the refusal for the sources that
   stay on it (R81, records §4.12); bitwise and symbol entries have
   none. As built: the operand types are the classifier's
   static types (`prim_static`); a numeral at an expected `Int` is
   `Int.ofNat n` or `Int.negSucc`, at `Nat` or with no expected type
   K's literal; since 3.21b rule 1 the expected type decides the
   operator where the operands do not.
4. **A non-structural measure** — `measure_pending` at this slice;
   3.14 rule 2 gives it `WellFounded.fix`.
5. **A `def` in the E forms** is elaborated by the same elaborator
   with the declared type expected. As built at 3.16 landing 1: the
   elaborator reads `match`, `if` and a body's untyped `let` itself;
   the fall-back through the classifier is deleted.

*Slice 3.14 (ruled 2026-09-17, item 47):*

1. **One scheme for all structural recursion, Lean's own.** A
   structurally recursive `fn` is `T.brecOn` over a functional whose
   `below` table holds the results at every sub-structure; a self-call
   on any recursive field at any depth is the table entry's value;
   `T.below` and `T.brecOn` are generated for a native inductive on
   first need under Lean's names (two `ACCEPT`s before the function);
   the equations stay `Eq.refl`. As built at 3.14: the table's shape
   is read off the export's `List.below` and `Nat.below` (`PProd`
   pairs, right-nested, no unit), a field's entry derived positionally
   and K's check holding the shape to Lean's, `recursion_depth`
   surviving for a self-call on a variable bound outside the leading
   case tree — **superseded at 3.16** (records §9.1 "Slice 3.16", the
   as-built): the entry is found by type, each table kept with its type
   over an abstract motive and searched through `PProd`'s projections
   with K's normal form (`define.shard` `pj_find`, `pj_search`), so a
   self-call under a match in an argument or a `let`'s value finds its
   entry as one in tail position does and `recursion_depth` is retired;
   no entry is `realize_descent` (R81, records §4.12). A file without
   `Init`'s `PProd` in scope cannot have a table — `below_unreached`.
2. **A measure becomes `WellFounded.fix`, the decreasing facts admitted
   obligations.** `(measure M)` with `M` a `Nat`-valued E term is
   `WellFounded.fix (InvImage.wf m …)` over the data parameters packed
   as a `PProd` nest; each self-call's `f.dec_N` is an obligation of
   the fourth policy class — `PARAM f.dec_N obligation`, `DEFINE f
   equations= pending=f`, `PENDING f measure`, every dependent's
   `params=` reaching it — discharged by `(fulfills f.dec_N PROOF)`;
   no equations from K (`f.eq_N` is I's, by `WellFounded.fix_eq` since
   3.20 as built); an `Int` measure is `measure_type`. As built: the
   obligation is quantified over every local in scope at the call in
   binding order (the type parameters, the tuple, the enclosing
   splits' fields and — since 3.21 — their equations, the `let`s);
   `wf_unreached` where `InvImage.wf` is out of reach. Since 3.30
   landing 2: a `fulfills` naming no obligation is refused by its
   cause (`fulfills_refused`, `fulfills_not_obligation`,
   `fulfills_outside_impl`), and the obligation discharges inside an
   implementation directory too.
3. **Mutual recursion** — revised by 3.15 rule 6: parked, then
   `mutual_recursion` at the file's end.

*Slice 3.15 (item 48; built 2026-09-18):*

1. **Metavariables are nodes** (`MVar`, `LMVar`) that K refuses —
   `mvar_in_type`, `mvar_in_value`, `infer` malformed — and the
   exporter never emits; the first K edit since the seal.
2. **The elaborator is a Meta layer above K** (`kernel/elab.shard`
   over `unify.shard`, `scope.shard`, `kw.shard`): bidirectional,
   first-order unification with typed assignments, K's `whnf` and
   `is_def_eq` through the view for the closed residues; nothing of
   it crosses into K.
3. **The spelling**: an implicit binder's argument is inserted, never
   written (`@NAME` for the explicit spelling); a `type`'s parameters
   are implicit in its constructors and a `fn`'s type parameters in
   its L signature, equations and obligations; a universe not written
   is inferred; a statement's `0` and `(+ a b)` at `Int` are
   `Int.ofNat 0` and `Int.add a b`; a false theorem's proof, a
   universe collapse and a consumer's `Eq.refl` through a view
   parameter are `type_mismatch` before K; `unsolved_universe`.
4. **`if` and the operators in a statement** are §5.1's: `ite` with
   the decision of the proposition's head; the identities by the
   operands' type (by the expected type since 3.21b); `- / mod` at
   `Nat` included in L.
5. **The constructor-structure bundle** is generated after a native
   inductive's admission in Lean 4.33's shapes (`define.shard`'s
   `confusion_bundle`): `T.casesOn` for every native inductive without
   indices, nested or mutual structure whose recursor eliminates into
   every universe, when its generation succeeds (else nothing, the type
   stands; R81, records §4.12); `T.noConfusionType`,
   `T.noConfusion` and `T.c.inj` for a type without parameters,
   indices, universe parameters or dependent fields where `Eq`,
   `Eq.ndrec` and `And` are in scope; the motive of `T.casesOn` found
   from the expected type as a Miller pattern. Pin `elab_confusion`.
6. **A forward reference is parked, never an obstacle**: a `fn` whose
   callee is not yet loaded waits with no record and is retried after
   every definition to a fixpoint; at the file's end the ones still
   parked are `RUNNABLE` with the obstacle they waited on
   (`mutual_recursion`, `no_l_meaning`); a `realize` never waits.
   Pins `define_mutual`, `define_forward`.

As built (2026-09-18): `unify.shard` — `MCtx`, `unify` typed through
`infer` or `infer_light`, `mc_drop_fvars` at every binder close;
`elab.shard` — `El`, `elab`, `tele`, `elab_args`, `check_expected`,
`elab_op`/`op_identity`, `elab_if`/`decision_of`, `el_finish`; the
pins `elab_implicit`, `elab_numeral`, `elab_unsolved`,
`elab_instance`, `elab_no_decision`, `elab_mismatch`,
`refuse_unsolved_universe`.

*Slice 3.16 (designed 2026-09-18 on GPT-6's single-frontend memo
R63–R71, item 49; built in three landings 2026-09-19):* S is
elaborated once into a **pre-definition** (`predef.shard`'s `PreDef`:
the identity and type, the binders' locals, the self-local, the body
as an `Expr`, the matchers, the callee-locals, the obligations) and
two projections consume it — toward E by erasure
(`kernel/erasure.shard`), toward K by Lean's own structural
compilation (`define.shard`); the classifier is the fragment check on
the elaborated term, not a second type system; `tr` and the tie are
deleted.

1. **Matchers, their descriptions bound by regeneration** (R64). A
   `match` elaborates to `OWNER.match_N` applied to a motive, the
   scrutinees and one alternative per row, a definition K checks
   admitted before its owner (`ACCEPT NAME.match_N`); the description
   is the matcher's type, read back by `mt_describe` and regenerated
   and compared by `mt_is_matcher` (`matcher_description` otherwise).
   The E program evaluates the scrutinees once and the first matching
   arm only. A `match` in a statement or a `def` is the same form; a
   dependent motive is 3.19 rule 9's. As built: `kernel/matcher.shard`
   (`mt_generate`, `mt_compile` by first-match column splitting with
   refinement at the leaves; `match_not_exhaustive`, `match_indexed`,
   `match_eliminator`), `elab_match` (the motive from the expected
   type, else `match_motive`), `el_pat` (`literal_pattern` until 3.17,
   `pattern_constructor`, `pattern_arity`), the matcher checked into
   the walk's environment as it is made (`kw.shard`'s `w_admit`);
   `match_in_requirement`.
2. **The self-local, and branch facts in every position** (R65). A
   recursive `fn` is elaborated with its name bound to a local of its
   Pi type; inside it an `ite` under a self-call is compiled as the
   dependent form (`Decidable.rec` with the proof a local of the
   branch) wherever it stands; each `f.dec_N` is closed over every
   local in scope at its call. The narrower guarantee stated: a
   `match` on a computed scrutinee gives no `scrutinee = pattern`
   local — since 3.21 as built a matcher under a measure carries its
   equation (the open item closed there).
3. **`Bool` and propositions meet in the elaborator, Lean's way.** A
   proposition where a `Bool` is expected is `Decidable.decide p` with
   `decision_of`'s decision (else `no_decision`); a `Bool` where a
   proposition is expected is `c = true`; `if` on a `Bool` is `ite (c
   = true)`. Erasure: `decide p inst` is `(if ⟦inst⟧ true false)` over
   Init's `Bool`, the decision of `c = true` in a condition is `c`;
   `Bool` and `Decidable` keep different cells in `ev`.
4. **Realization selection, not an inverse of spellings.** The operator
   table is the static realization registry: a resolved L operation
   at its types → the executable entry, its result cell, its stated
   trust (the registry grew type rows at 3.18 and expression rows at
   3.17). As built: `Nat.add`, `Nat.mul`, `Nat.decEq`/`decLt`/`decLe`
   select the integer entries `+ * int_eq lt le`; `Nat.sub`,
   `Nat.div`, `Nat.mod` their own saturating, total entries;
   `Int.add`/`sub`/`mul` and `Int`'s decisions theirs; since 3.19 a
   decision's entry is its test.
5. **One resolver.** A `fn` body's names resolve as a `def`'s
   (`scope.shard`); an ambiguous bare name with exactly one candidate
   a constructor of the expected type's head inductive is that
   constructor (`resolve_ctor`).
6. **Which route; a fallback weakens the guarantee and never
   reinterprets** (R70). A function takes the typed route when its
   signature elaborates, decided before its body is read; a
   `RUNNABLE` callee is bound as a callee-local (`why=callee_runnable
   route=typed`); a typed-route error is a refusal, never an E-first
   success; a signature with no L reading — the toolchain's own
   sources — is `route=e_first`, the classifier's program, until the
   flip. As built: the `RUNNABLE` record carries the route; after the
   signature elaborates a `(quote S)` or `"…"` in the body (until
   3.18), a literal pattern (until 3.17), a name the environment lacks
   are `why=no_l_identity`; an extern or E-first callee
   `why=no_l_meaning`; parked functions at a file's end are each
   other's callee-locals (`define_mutual`).
7. **`realize` with a supplied body takes the same route**: the body
   elaborated against the realized constant's telescope, the erasure
   its program, the descent checked on it, the equations per leading
   leaf; its matchers owned by `MODULE.NAME.realize`. `tr` deleted
   with `arg_types`, `ctor_etargs`, `apply_const_e`, the E-term case
   tree, `DefRec`/`BelowInfo`/`WfInfo` and the tie;
   `prim_typed_identity` and `c_type` remain the classifier's own.
8. **One descent obligation discharged end to end** (R65, R69).
   `(fulfills f.dec_N PROOF)` outside a view: PROOF read against the
   obligation's statement as the theorem `f.dec_N.proof`, checked by K
   in the file's environment with the obligation standing as the
   parameter it is; the proof's closure — every discharged obligation
   replaced by what its proof rests on, to a fixpoint — must not reach
   `f.dec_N` directly, through `f`, through a theorem or through an
   earlier discharge (`fulfills_cycle`, nothing admitted: the closure is
   the acyclicity check and the circularity defence; R81, records
   §4.12); on
   success `DISCHARGE f.dec_N proved` and every account recomputed —
   each `ACCEPT` that named the obligation names the proof's account,
   a function with no open obligation leaves `PENDING`. Fixture
   `discharge_measure` (`count.dec_1`), `discharge_self_cycle`,
   `discharge_two_cycle`. Dependency-directed wake-up deferred.
9. **Partial outcomes at the shared boundary** (R67). `kw.shard`'s
   wrappers return K's outcome (`WVal`/`WFail`), never `None`;
   `unify` answers `UYes`/`UNo`/`UStuck`/`UExhausted`, reported as
   `unify_stuck`/`unify_exhausted` apart from `type_mismatch`; an
   assignment is typed or tentative (`tentative_assignment` at
   `el_finish`); a closed value K refuses is `UNo` — except one citing
   a constant the form is still declaring (`cites_undeclared`).
   Test `unify_test.shard` (G6).
10. **Holes across a binder** — `mc_drop_fvars` makes the elaborator
    incomplete, never unsound; the contextual-hole contract was
    deferred to I's opener (3.20) and remains unbuilt: no API outside
    `elab.shard` takes an `MCtx` across a binder close.
11. **The small facilities**: `(list a b c)` by the expected type's
    list constructor pair (no expected type: `unsolved_implicit`); a
    negative numeral §5.3's; record update, the fresh-name supply and
    E's rename at 3.17; bytes, text, deriving, `"…"` and symbols in a
    `fn` body at 3.18–3.19.

As built, the K projection (landing 3): a body that never cites the
self-local is the value under its binders; structural recursion is
`T.brecOn` over `λ x below … body` with the matcher kept as a
constant whose motive carries the table (Lean's `MatcherApp.addArg`),
a self-call found by type in the table's normal form, so the shape of
`below` is never assumed and a self-call under a match in an argument
or a `let` finds its entry; measured recursion `WellFounded.fix` over
the data tuple with `rec (tuple) (f.dec_N locals…)`; `ite` under a
self-call `Decidable.rec`; a function with a measure clause that
never calls itself is a plain definition. The equations
(`pj_equations`): the leading matchers' rows against the scrutinee —
bind, clash or split — with no reduction by K; a leaf K cannot decide
by conversion is I's and the function is defined without it
(`define_arg_match`). `if` on a two-constructor inductive is
`T.casesOn`, read back as `EIf` by the erasure (`if_type`,
`private_if` the elaborator's). The gate: calc's 25 of 25 functions
defined, `calc_test` byte-identical with the erasures as the programs,
parity over 27 closures; fixtures G1 `one_meaning`, G4
`discharge_measure`, G7, G8 `route_callee`, G9 `match_in_impl`.

**The porting facilities and bytes — slices 3.17–3.18** (records §9.1
"Slice 3.17", "Slice 3.18"; §13 items 50 and 51, R60's row of §11).

*Slice 3.17 (ruled and built 2026-10-01, item 50):*

1. **`"…"` in a `fn` body** is an L position's `LitStr : String` since
   3.16; it kept `no_l_identity route=e_first` until 3.18's flip gave
   it the one meaning everywhere (rejected: list-literal sugar by the
   expected type).
2. **Symbols** and `Name` literals go with 3.18 (item 36 rules
   `Symbol`'s L identity `String`; the flip carried to phase 4 at
   3.30 rule 3).
3. **Numeral rows on the typed route** are Lean's value patterns: the
   scrutinee once, then `(if (= x K) BODY …)` per row in order, the
   last row `_` or a name; no matcher, the erasure the `if` chain.
   As built: `elab_match_lit`; `match_not_exhaustive` (no last row),
   `literal_rows` (a row after the catch-all, a constructor row
   beside numerals), `pattern_type` (a numeral row at a type other
   than `Nat`/`Int`); `literal_pattern` stays the E-first fallback for
   a numeral under a constructor and a symbol pattern. Rejected: the
   numeral as a `succ` chain through the matcher.
4. **Records**: `make` and `with` reach a `structure` of the file
   (`record.shard`, at the s-expression level: `make` is `NAME.mk` in
   field order, `with` is `NAME.mk` on each field's entry or its
   projection); a kept field whose type's source names an updated
   field is `dependent_update`, naming the fields to supply, never an
   inserted cast; a structure with parameters is out of the sugar's
   reach (`record_make`/`record_with` with the pointer). Pin
   `record_laws` (five theorems by `Eq.refl`).
5. **The rename, staged by who reads the file.** E's table names the
   primitives `= < <=`; the old spellings `lt le int_eq` are read as
   those entries only in a module whose identity begins with `kernel`
   (`scope.shard`'s `toolchain_scope`) and refused elsewhere as
   `renamed_primitive` with the new spelling; the Rust bootstrap reads
   both and dumps the V3 spelling; `v3/tools/rename_cmp.py` migrates
   and checks. `v3/kernel/**` migrates when route 1's chain is V3's
   own (665 sites in 52 files by the tool's count). **Re-attributed 2026-10-16 (§13 item 69): unblocked since slice 4.1 — the emitter prints the chain's spellings from the program (`em_prim`) and the Rust bootstrap reads both; slice 4.4 landing 2, by the tool.**
6. **The fresh-name convention is a library**: `v3/std/fresh.shard` —
   `FreshName`, `Fresh`, `Minted`, `Fresh.start`, `Fresh.next`,
   `Fresh.next_ne` (two successive names differ, no axiom); the prefix
   type a parameter `P`.
7. **The E-first-by-body set**: the registry gained expression rows
   (`erasure.shard`'s `realization_expr`; `Int.natAbs a` erases to
   `(let (a) (if (< #0 0) (- 0 #0) #0))`), so calc's `show_nat` takes
   the measure `(Int.natAbs n)` and is defined; a callee that is a
   view's `sig fn` the environment lacks is `no_l_meaning`
   (`callee_hits`, pin `route_sig_callee`).

*Slice 3.18 (ruled 2026-10-01 on five leans, item 51; built
2026-10-02):* the representation is the byte list — `UInt8` an
integer, `ByteArray` the list of them, `String` its `ByteArray` with
the validity proof erased, no executor gaining a kind of value; the
externs carry `ByteArray`; symbols deferred; the flip staged like the
rename; deriving its own slice.

1. **Type realizations: six rows of the registry.** `Fin`, `BitVec`,
   `UInt8`, `Array`, `ByteArray`, `String` are represented by their
   one runtime field: the type keeps its name in E with its value
   arguments dropped, the constructor is its runtime argument, the
   field's projection its subject, a case analysis binds the field to
   the scrutinee; a row's type is never an E inductive, so the typed
   and E-first routes cannot disagree on the cells; the erasure checks
   the shape at each use (one constructor, one runtime field —
   `no_realization` otherwise); no row for an operation. As built:
   `erase.shard`'s `rep_row`, `l_to_etype`, `row_param_roles`,
   `ind_eligible` no for a row; `erasure.shard`'s `erase_row_cases`,
   `row_field`; the classifier treats a row's type as known with no
   constructor (`pattern_type`, `if_type`); pins `row_types`,
   `row_e_first`, `row_e_match`, `realize_view`.
2. **A projection function is its projection**, for every structure:
   a constant whose value under its arguments' lambdas is `self.i`
   erases as that projection of its last argument (`proj_fn_app`; pin
   `struct_proj`) — what lets a body call `String.toByteArray`,
   `Array.toList`, `Fin.val`, `ByteArray.data` with no `realize`.
3. **The literal.** Outside the toolchain's sources `"…"` is a
   `String` in every position — K's literal on the typed route,
   erased to the one literal over `List.nil`/`List.cons`, the same in
   an E-first body (`no_string` without `String` in the environment);
   bytes that are not UTF-8 are `bad_string` at the elaborator
   (`utf8_ok`); in a module whose identity begins with `kernel` the
   byte list of the `List` in scope until route 1's chain is V3's own. **Re-attributed 2026-10-16 (§13 item 69): until Init's types are in the toolchain's closure, phase 5's opener design; not the chain's.**
4. **The wire by declared type.** The driver builds and reads each
   value of an extern by the E type the program declared — a codec by
   identity (bytes at `(List Int)` or `ByteArray`, a list, an option,
   a pair, a flag), the result's type decided before the host is
   asked; a declared type the host's value does not fit is stuck
   (`RUN: stuck extern`); the checked entry takes a `ByteArray`; the
   six externs under the naming law are `v3/std/host.shard`'s
   (`get_args`, `read_file`, `write`, `write_line`, `write_file`,
   `exit`). Not built: a check at link. Pins `wire_s`, `wire_bad`,
   `wire_bad_result`; `wire_test`.
5. **Symbols.** `(quote x)` in a function whose signature has an L
   reading keeps `no_l_identity route=e_first`; the direction for the
   port: `Symbol` its own type over `String` represented by the atom,
   a seventh row whose simulation is interning (carried to phase 4 at
   3.30 rule 3).
6. **The libraries and the consumer**, built as two libraries:
   `v3/std/bytes.shard` (Init's `BitVec.toNat` and `UInt8.toNat` by
   their views; `Byte.ofNat` with `Byte.ofNat_eq` and `Byte.toNat_ofNat`
   by `Eq.refl`; `Bytes.ofList`/`toList` with both round trips;
   `Bytes.ofNats`/`toNats`; `Bytes.ofString`), `v3/std/host.shard`;
   `v3/examples/calc/calc_main.shard` closes calc's loop against the
   host, byte-identical to the model world (`calc_main_test`).

As built, found and fixed: an E-first body could take a row apart
(closed); `write`/`write_line` ignored the declared result (closed);
a classifier refusal after a typed-route obstacle carries it
(`load_e_after`: "read E-first after the typed route's …"). Open,
stated: the checked decoder (`String.fromUTF8?`), rows for `Char`,
the wider `UInt`s and `USize`, `String`'s own operations, a `realize`
by a proven-equal S function; the six rows and the literal's bytes
are lines of the erasure, trusted with it (§7.1). The second Init
fixture chunk of this slice is gone since 3.26 (Init on demand).

**Deriving — slice 3.19** (records §9.1 "Slice 3.19"; §13 item 52,
ruled 2026-10-02 on five leans and built the same day). A use site
finds a derived operation through a derivation table, no search; one
named policy `structural`; the ordering is `compare` into Init's
`Ordering` with no laws; the rendering is bytes in canonical S;
supported: a type without parameters and Init's `List`, `Option`,
`Prod` at closed arguments.

1. **The form.** `(derive TYPE CAPABILITY…)`, a declaration of its
   file; TYPE a closed type (a name or an inductive applied to closed
   types); a capability `eq`, `(ord structural)`, `render`, or `(eq
   by NAME)`/`(ord by NAME)`/`(render by NAME)` registering a
   hand-written procedure after its type is checked; the generated
   functions live in the deriving module under the type's mangled
   name (`Tok.decEq`, `List_Tok.compare`).
2. **Generation is source.** A derivation writes `fn` forms the
   loader reads as the file's own — the one front end, K, the
   erasures, ordinary `DEFINE` records; nothing trusted, no second
   route (`kernel/derive.shard`; `load_derive`; the generated source
   cites every constant by its identity).
3. **`eq`** is `NS.decEq ((a T) (b T)) (Decidable (= a b))`: a match
   on both, a row per constructor pair; fields decided left to right
   by their procedures through `dite`, `isTrue` by congruence,
   `isFalse` by the constructor's injectivity (`C.inj`, or a
   one-constructor type's projections) and, for different
   constructors, by `NS.ctorIdx` (a derivation of its own, `cap=idx`,
   shared through the table) with `Nat.ne_of_beq_eq_false`. As built,
   wider: a one-constructor type with parameters derives through its
   projections (`derive_params`); Init's parameterless types derive as
   the program's.
4. **`(ord structural)`** is `NS.compare ((a T) (b T)) Ordering`: the
   same constructor's fields left to right, the first not `eq`
   decides; different constructors by `NS.ctorIdx`; a `Nat`/`Int`
   field `(if (< x y) lt (if (= x y) eq gt))`; the declaration order
   is in the value, so reordering changes the function and its hash.
5. **`render`** is `NS.renderOnto ((a T) (rest (List Nat))) (List
   Nat)` and `NS.render ((a T)) ByteArray` over `Bytes.ofNats`
   (`derive_needs` without the library): `(NAME F…)` or `NAME`, a
   field by its entry; an erased or opaque field prints `_`. As
   built, narrower: a function-valued field is `derive_field`, not
   `_`; a list renders by its constructors.
6. **The table.** An entry is (type, capability, procedure, deriving
   module), the type keyed in normal form, visible exactly when its
   deriving module and procedure are (`Load.derived`, `derived_for`);
   a view exports a capability as a `sig fn` and the consumer
   registers it (`(derive lib.Handle (eq by lib.Handle.same))`, pin
   `derive_sig`); `eq` at `Nat`, `Int`, `Bool` the elaborator's own
   rows; `v3/std/derive.shard` derives `Bool`, registers `UInt8`,
   `ByteArray` and `String`'s procedures and the renderers of `Nat`,
   `Int`, `UInt8`, `String`; a program derives its own types and the
   instantiations it uses (E has no function values); the record
   `DERIVE PROC type=T cap=C policy=P fields=…`.
7. **The use site.** `(if (= a b) …)` and `(= a b)` where a `Bool` is
   expected consult the table (`ambiguous_decision`, `no_decision`
   with the pointer to `(derive T eq)`); `compare` and `render` are
   called by name until Stage 3. As built: on the E-first route `=`
   at an inductive is `prim_type` with the pointer (`derive_e_first`).
8. **Refusals**, the type untouched: `derive_type`, `derive_shape`,
   `derive_field`, `derive_needs`, `derive_duplicate`, `derive_by`,
   `name_taken` with `derive_failed`.
9. **The elaborator's general gains**: a `match` on a local the
   expected type mentions generalizes it (the motive `λ d. RET[d]`,
   each row at RET with its pattern; the constant motive the
   fallback) — a proof by cases is a `match` in S (`match_dependent`);
   an operator in an E-type position takes its levels by unification.

As built, found and fixed: an `if` on a self-call under a measure
(`if_rec_call`); an obligation named once per statement
(`measure_if_call`); `Bool.and`/`or`/`not` cited by their real names
with registry rows (`bool_ops`); a decision erases to its test only
where an `if` reads it, `(if TEST isTrue isFalse)` as a value
(`decidable_cells`); the name-keyed rows hold for native declarations
only (`registry_native`); `load_e_skipped` carries a skipped
signature's cause; the second reader's findings a–h. Not built:
Lean's instance constants (Stage 3), the ordering's laws, types with
parameters and two or more constructors, nested and mutual types,
operator spellings for `compare`/`render`; the toolchain's sources
cannot derive until route 1's chain is V3's own. **Re-attributed 2026-10-16 (§13 item 69): until Init's types are in the toolchain's closure, phase 5's opener design; not the chain's.**

**I, the proof IR — slices 3.20, 3.21 and 3.21b** (records §9.1
"Slice 3.20" to "Slice 3.21b"; §13 items 53, 54, 55; law §7). I is a
certificate language: replay elaborates it to P without search, K
checks P; the block in source is I; no subterm order (`induction`
carries its recursor, `wf` is `WellFounded.fix`); computation is
conversion; the API is sequential (`goal_of`, `applicable`, `step`,
`elaborate` over one derivation).

*Slice 3.20 (ruled 2026-10-05 on six leans, item 53; built
2026-10-06):*

1. **The form.** A `theorem`'s proof is `(exact TERM)`, `sorry` or
   `(by STEP…)`; a `fulfills` the same three. A block is a sequence
   of steps on one goal, each but the last turning the goal into one
   goal, the last closing it; a step that opens several goals carries
   each one's block inline (`(cases x ((zero STEP…) (succ (n)
   STEP…)))`); `goal_open`, `goal_closed`; every refusal names the
   step by its index and shows the goal as `goal_of` renders it. As
   built: a premise's one-step block may be written as the step
   (`(apply Eq.symm (exact h))`); `cases`/`induction` arms in one
   list, each arm's names in a list; `(cases TERM H (ARM…))` names
   the equation (`induction` so is `induction_eq`).
2. **A goal is a closed proposition** — a metavariable whose type is
   the statement closed over its named context; a step opens the
   binders as K locals and closes its filling over them, so every
   assignment is closed and R68's delayed filling never arises.
3. **A step is a function of the node and the goal**: `step(goal,
   node)` answers `StepDone(term)` with the subgoals in order, or a
   refusal; `elaborate(I)` runs `step` depth-first and instantiates;
   no node consults a changing table or runs a search.
4. **The vocabulary**: `(intro NAME…)`, `(exact TERM)`, `rfl`,
   `(have NAME PROP STEP…)`, `(show PROP)`, `(apply TERM
   STEP-BLOCK…)` (exactly arity(TERM) − arity(target) binders opened;
   unassigned propositions are child goals in binder order;
   `apply_goals`, `apply_unsolved` with the pointer), `(cases TERM
   ((CTOR (NAME…) STEP…)…))` (a local of an inductive without
   indices; `cases_indexed`, `cases_fields`, `cases_rows`; a term
   that is not a local is generalized), `(induction TERM (…))` over
   `T.rec` (dependents reverted since 3.29 rule 6), `(wf X MEASURE IH
   STEP…)` (`WellFounded.fix` at `InvImage.wf MEASURE
   Nat.lt_wfRel.wf`, `IH : ∀ y, MEASURE[y] < MEASURE[X] → target[y]`),
   `(decide)` (`of_decide_eq_true (Eq.refl true)` at the decision the
   table or the lookup gives), `(unfold NAME…)`, `(reduce SIDE?)`,
   `(rw …)`, `sorry`.
5. **Rewriting is `Eq.mpr`** over the motive that abstracts the
   chosen occurrences: `Eq.mpr (congrArg motive h) ?g'`. `rw` at a
   hypothesis was not built.
6. **`unfold` and `reduce`.** `(unfold f)` rewrites each occurrence
   of `f` at full arity by the first matching equation, once per
   occurrence present at the step's start (`unfold_stuck` with the
   pointer); `(reduce)`, `(reduce lhs)`, `(reduce rhs)` replace the
   target or a side by its normal form — the first redex in
   pre-order by an equation or K's whnf where it progresses, never a
   stuck eliminator, a lambda, an inductive's, a constructor's or a
   proposition's application; a computation rule (a constructor or
   literal in its left side) applies anywhere, an unfolding only to a
   ground occurrence under a budget of 64; a definitional equation
   contributes no term, a propositional one is a rewrite of rule 5
   with a continuation goal; a `def` with no equations by one delta
   step (`delta_once`). A measured function has its equations, each
   leaf's proven by `WellFounded.fix_eq` at the instance (the
   obligations admitted into the walk first); since 3.27 landing 3
   the reducer reads a realization's equations too. Since 4.2 landing
   1: a bare constant is no redex; K's whnf and the delta step fire on
   a closed subterm only — an open occurrence unfolds by its equations,
   by a case step, or by name —; and the case step waits at an open,
   non-constructor discriminant, which reduces by its equations first
   (§8.6 "as built, landing 1" item 3).
7. **`rw`.** `(rw LEMMA)`, `(rw (<- LEMMA))`, LEMMA elaborated
   without an expected type and opened to an equation
   (`rw_not_eq`); the left side matched syntactically
   (`tac_match`: structural descent, whole spines, never by K's
   conversion) against the target's subterms in pre-order, the
   function before its arguments; every occurrence of the first
   instance rewritten; `(occ K)` the K-th occurrence of the first
   instance, `(occ K EXPECT)` guarded (3.29 rule 5,
   `rw_occ_changed`); unassigned premises are positional child
   blocks; `rw_no_match` shows the pattern and the target; `a = a`
   is not closed by `rw`. Init's lemmas speak `HAdd.hAdd` where V3
   writes `Nat.add` — one spelling since 3.21 rule 3.
8. **Universes and levels** by unification, as any constant's.
9. **`sorry` in a block** closes its goal and makes the theorem
   pending (`PENDING NAME sorry goals=K`, the goals rendered); the
   block is elaborated whole.
10. **The E functions** (`kernel/tactic.shard`): `goal_of`,
    `applicable`, `step`, `elaborate`; `Node` is I's data, one
    constructor per form, read by `read_block`; `tac_step`,
    `tac_block`, `tac_prove`; `reader.shard`'s `read_proof` hands a
    `by` to it for a `theorem` and a `fulfills` alike.
11. **Records.** The design's `PROOF NAME nodes=N` was not added:
    `ACCEPT` stands for a theorem proven by a block; the pending
    record carries the count.
12. **Not in 3.20**: `injection`, `subst`, `cases` on an indexed
    family or on `Eq`; `exists`/`constructor`/`left`/`right`; `intro`
    through a `let`; `wf` over an `Int` measure or a lexicographic
    pair; a witness goal (3.22 rule 4 built it); `rw` under binders
    where the motive would capture (`rw_motive`); `by` inside a
    term; the bootstrap reads `by` as `UnknownForm`, so a theorem
    under `v3/kernel/**` stays on `exact` until route 1's chain is
    V3's own (re-attributed 2026-10-16, §13 item 69: the Rust reader's,
    enduring — a reader rule if a port needs it; not the chain's).

As built, the consumer: 44 of calc's 100 claims as blocks in five
files (`tactic_test.sh`), the rest 3.21's. Found by the probes: the
recursor takes its major after the minors and `casesOn` before them;
every name in `tactic.shard` checked against the closure (the
bootstrap's flat resolution). Found by a second reader: a named
equation keeps its local; `(occ K)` over whole spines; an unfolding
only on a ground occurrence under the budget.

*Slice 3.21 (item 54, designed and built 2026-10-06):*

1. **`arith` is a Farkas certificate over stated rows.** `(arith
   FACT…)` closes a goal that follows from linear arithmetic over
   `Int` and `Nat`: the rows are the goal negated, the context's
   linear hypotheses in order, each FACT (`arith_fact` otherwise) and
   the rows the atoms bring; `(arith FACT… (farkas K…))` carries the
   certificate (one integer per row; replay checks the sum,
   `arith_certificate`); without it the node names the default
   reconstruction — Fourier–Motzkin over the rows in atom order under
   512 derived rows (`arith_budget`), exact over the rationals,
   integer-tight only in strict comparisons, no cut, no case split, a
   `≠` no row (`arith_failed` shows the rows in normal form).
   `kernel/arith.shard`; the node `NArith` in `tactic.shard`; an
   equation goal takes a clause per direction.
2. **Linear statements**: `≤ < =` and their negations at `Int` or
   `Nat` under V3's and Init's spellings, the sides from `+`, `-`,
   unary `-`, `*` with a literal, literals and atoms; `False` as a
   goal has no negation row; an equation goal is two certificates
   (`Int.le_antisymm`) unless the sides have one normal form.
3. **One spelling — the proof layer's** (wider than designed): a class
   method at `Int` or `Nat` under Init's instance is its function
   throughout a row, and every new goal's statement enters in the
   source's spelling (`tac_goal`), as do a lemma's two sides in `rw`,
   `simp_only`, `reduce` (`eq_of_lemma`) and a premise `simp_only`
   looks up; K's conversion is the proof. Since 3.27 rule 2 the
   spelling is a table (`sp_canon`), not a transparency.
4. **What an atom brings**: a `Nat` term enters under `Int.ofNat`
   with the row `0 ≤ ↑a`; `x / k` at `Int` with a positive literal
   brings `k * (x / k) ≤ x` and `x < k * (x / k) + k`, `x % k` is
   `x - k * (x / k)`; `Nat` subtraction, division at `Nat`, `min`,
   `max`, `natAbs`, `toNat` are atoms; `Nat.succ x` under the cast is
   `x + 1`.
5. **P is `Lean.Omega`'s**: each side a `LinearCombo` with its
   `eval` proof, rows by `Constraint.addInequality_sat`/
   `addEquality_sat`, the certificate a fold of `combo_sat'`, the
   false constant by `normalize_sat` and
   `Constraint.not_sat'_of_isImpossible`; K's conversion does the
   arithmetic on closed data, no reflection; `arith_unreached` names
   a kit lemma past the horizon.
6. **`simp_only` is a bounded rewriter over a lemma list**: `(simp_only
   RULE…)`, the first rule in list order with a match applied as `rw`
   applies it, to a fixpoint under 256 rewrites (`simp_budget`);
   premises discharged by assumption only; `simp_no_progress`; a
   function's name is no rule; a rule whose right side holds its left
   is `simp_self`.
7. **`applicable`** lists `arith` where the target is linear and
   `simp_only` always.
8. **Not in 3.21**: an `only` restriction (3.22 rule 3 built it); cuts
   and case splits; `Nat` subtraction; non-literal products;
   `simp_only` at a hypothesis or with another discharger.

As built beside the rules: **a matcher under a measure carries its
equation** (`pj_match_eq`: in a measured function a matcher's motive
is `λ s'. s = s' → motive s'`, each row's last binder `h : s =
pattern`, the application at `Eq.refl s`, so an obligation is stated
under it as `measure[args] < measure[params]` at `Nat`); `reduce`'s
two repairs (a case analysis at a decided scrutinee takes one step by
masking the alternatives — `alt_step`; a comparison is left as it
is); a type and its constructor under one name resolve to the type
where a type is expected; the pending record shows its goals; the
consumer: calc's 100 claims are theorems with the capstone
`run_eq_spec` under no parameter (`tactic_test.sh`, 110 checks), 45
`arith` steps and 18 `simp_only`. Found by a second reader:
`simp_self`, `by_ne_level`, `define_match_eq`, generated binder names.

*Slice 3.21b (GPT-6's trajectory review R72–R77, records §4.11; item
55; built 2026-10-06):*

1. **The expected type decides an arithmetic operator**: where
   `+ - * / mod %` is read at an expected type whose normal form is
   `Nat` or `Int`, both operands are read at it (a `Nat` local under
   `Int` cast at the leaf); where it decides nothing, the identity is
   the first operand's that is not numeric-closed, all numeric-closed
   is `Nat` (`op_expected`, `is_num_closed`; pin `op_expected`; the pin
   `one_meaning` restated).
2. **Init's visibility is the module's horizon**: each imported
   declaration carries its ordinal; a module's horizon is the farthest
   of its own `(import Init NAME)` and its imports'; resolution sees
   an imported declaration below the horizon only, whatever the load
   admitted and in whatever order (`VisOnly`'s horizon and ordinal
   table, `init_within`, `Load.inits`, `init_horizon`; pins
   `init_horizon`, `init_horizon_beyond`). Restated at 3.25 rule 2 and
   3.26 rule 4 under the shared and on-demand loads.
3. **`reduce` is a normalization**: it succeeds unchanged where
   nothing reduces; `reduce_stuck` retired, `reduce_budget` stays.

**The producers and the store — slice 3.22** (records §9.1 "Slice
3.22"; §13 item 56, ruled 2026-10-07 on three leans; three landings
2026-10-07/08; law §7.3, §7.5). The engine's output is `(arith only
FACT… (farkas K…))` with the certificate written; the store is a CI
artifact, never committed; the engine's reach is measured over calc's
100 claims.

1. **`auto` is a delegation; the build never searches.** A `theorem`'s
   proof (a `fulfills`' and a discharge's too) may be `auto` or
   `(auto HINT…)`; the loader reads `FILE.auto.shard` once per file
   (`read_sidecar`, `Fx.autos`) — `(proof-for NAME FINGERPRINT (by
   STEP…))` forms after `(sidecar N)` (3.29 rule 4), the file
   machine-owned and rewritten whole by `prove` — and replays the
   entry's block as a hand-written one; `PENDING NAME auto_missing
   fp=N`, `auto_stale fp=N`, `sidecar_malformed`.
2. **The key is the statement fingerprint**: `hash_expr` of the closed
   statement, what the `ACCEPT` record carries, so a changed reading of
   the statement is a different key and a changed strategy is not. It
   is not law §5.2's resolved requirement (R82, records §4.12): a
   referenced declaration's body can change under its name, a policy
   change never enters the key, and an older sidecar schema is replayed
   by the current implementation with no version dispatch — replay
   re-elaborates and K checks, so nothing invalid is accepted by this,
   but a changed task is not detected. The three identities the law
   names — the resolved requirement with its dependencies' identities
   and policy, the reconstruction contract, retained P under the
   canonical encoding (§7.5, display names outside it) — are owed to
   phase 4 before T6's prepared handles (§11).
3. **`(arith only FACT… (farkas K…))`**: the rows are the goal
   negated, the FACTs in order, the atoms' rows — no context row
   (`NArith` carries `only`; pin `arith_only`).
4. **The witness goal**: in `(apply TERM (STEP…)…)` a premise whose
   statement holds an unassigned metavariable is a witness goal; the
   blocks run in order, `exact` and `rfl` assign through unification
   (`premise_goals`, `witness_close`, `tac_open` zonking), a
   metavariable still open after the blocks is `witness_open`; an
   open instance argument `instance_needed`.
5. **The engine is an E library over the fixed API**
   (`kernel/engine.shard`): `engine_prove` composes `goal_of`,
   `applicable` and `step` into a ladder, cheapest first — `intro`,
   `rfl`, `(reduce) rfl`, `decide`, `(arith only …)` over the
   context's comparisons with the certificate found by `ar_prove`,
   `(simp_only EQN… HYP… LEMMA…)` then `rfl` or `arith`, `induction`
   on the hinted local then each inductive local, three levels deep;
   hints `(induct X)`, `(lemmas NAME…)`, `(budget N)`; the induction
   tried before the introductions too (the hypothesis quantifying the
   binders); an unfolding equation whose right side mentions the
   function is no rule; every node confirmed by `tac_step`; a failed
   attempt leaves nothing (the state is a value).
6. **`prove` is a driver** (`kernel/prove.shard`): the loader's mode
   `off`/`solve`/`measure` (`Load.mode`); under `solve` an `auto`
   without a valid entry is the engine's (since 3.29 rule 2, a named
   tactic's), replayed through `tac_prove` before acceptance (`PROVED
   NAME fp=N (by …)`, `PENDING NAME auto_unsolved fp=N GOAL`), the
   sidecars rewritten whole with the engine's version; under
   `measure` every hand-written proof is read as the build reads it
   and the engine (and a named tactic, 3.29) run beside (`ENGINE NAME
   solved|unsolved`). The engine closes 54 of calc's 100 claims
   unaided (`engine_calc.txt`, held exact by `engine_test.sh`);
   `v3/examples/auto/` with its committed sidecar.
7. **The store is K's export format** (`kernel/store.shard`): `load.shard
   --store DIR` writes each accepted declaration and each `PARAM` as
   lean4export's NDJSON records (format 3.1.0, tables numbered from 1
   in first-use order, strict structural sharing, an inductive as its
   block), `DIR/ID.ndjson` per declaration with ID the content
   fingerprint (`FP-K` on a collision of texts), `DIR/order`
   appended, never rewritten; a failed load writes nothing.
   Beside `order`, `DIR/manifest` is the release's fixed target
   (slice 4.0 rule 3, R79): `manifest 1`, then `decl ID KIND NAME` per
   declaration, `param NAME` per parameter the store holds as an axiom
   (a view parameter, an undischarged obligation), `discharge OBL
   PROOF` per obligation the load discharged — appended as `order` is.
   `kernel/verify_release.shard DIR EXPORT INDEX` is K, the host and
   the JSON reader alone: Init's closure of the store's citations on
   demand from the export through its index (3.26; the chunks as one
   stream before it), then each store file in `order`'s order as its own stream
   over the environment so far, each held to the manifest by name,
   fingerprint and kind (`MISTIED`, `MISKIND`), one accepted
   declaration per file (`CARDINALITY`), every order line in the
   manifest and every manifest declaration in `order` with its file
   (`UNLISTED`, `MISSING`); then the policy (rule 3): an accepted axiom
   is a declared parameter or `UNDECLARED`, each `discharge` is held
   at its statement (`DISCHARGE`, else `BAD-DISCHARGE`), and every
   other declaration's assumption closure — K's axiom walk, a verified
   discharge standing for its proof's closure — rests on the export's
   axioms and the declared parameters alone or is `CONDITIONAL` with
   the axioms named; `PARAM` lines report what a conditional bundle
   rests on. `RELEASE: declarations N  accepted A  … conditional Q`;
   the exit is 0 exactly when every failed class counts zero (rule 2,
   R80), else 1, never a subtraction across units. `store_test.sh`
   (the 36th entrypoint; the memo's tampers as pins); `v3/release/`
   the v3 job's artifact. Not built: a store over implementation
   forks; a release across roots; the three persisted identities (R82,
   §11).

**`Init` — slices 3.23–3.26: the cache (retired), the connected path,
the shared load, Init on demand** (records §9.1 "Slice 3.23" to
"Slice 3.26"; §13 items 57 (retired), 58, 59, 60; GPT-6's R75, R76).

*Slice 3.23 (ruled 2026-10-08 on R75, item 57; built 2026-10-08;
RETIRED at 3.26 landing 3, 2026-10-09, by the user's ruling):* the
receipt (`t0.shard --receipt`, `load.shard --init-receipt`,
`init_receipt.sh`) and K's admit mode skipped the typing judgments of
the export's prefix under a receipt of a full replay. Measured under
Init on demand, admission saved a tenth to a fifth of a demanded load
and its only writer was the full replay, so the receipt, the flag and
the currency script were deleted. What stayed — `k/add.shard`'s
`admit_decl_pinning` and the run's admit mode (`run_admitting`),
repurposed for the loader's second admission of records it checked a
moment ago (3.26's `od_admit`) — was deleted at slice 4.0 landing 2
(R78, §8.6): the second admission is the checked transfer
`env_transfer`, under identity; `verify_release` never took a receipt.

*Slice 3.24 (ruled 2026-10-09 on R76, item 58; built 2026-10-09):*
law §12.4's first connected path assembled and broken at each joint.

1. **`(dif h C T F)` — the dependent if, its hypothesis named.** `C`
   elaborated as an `if`'s condition; `T` under `h : C`, `F` under
   `h : ¬C`, each at the form's expected type; the term `dite C dec
   (λ h. T) (λ h. F)`; a two-constructor value is no condition
   (`dif_type`); on the E-first route `dif_needs_init`
   (`elab_dif`; pins `dif_ok`, `dif_type`, `dif_needs_init`,
   `dif_scope`).
2. **The example, `v3/examples/path/`**: an imported Init function
   with its realization attached (`List.length`, `List.get?Internal`
   by supplied bodies with their equations proved), a bounds-checked
   access `at xs i` under `(dif h (< i (List.length xs)) …)` through
   `Fin.mk i h`, the claims `at_some`/`at_none` by `(unfold at) (rw
   (dif_pos h)) rfl`, an entry on raw arguments, the store written and
   replayed by K alone.
3. **The test, `kernel/test/path_test.sh`** (the 37th entrypoint): the
   path in order, then the law's five breaks each failing by name — a
   wrong realized body (`conversion` at the equation), the hypothesis
   outside its branch (`unknown_constant: h`), the `Fin.mk` without
   its proof (`type_mismatch`), a mistied order line (MISTIED), a
   tampered stored proof (REJECT), an invalid raw argument (exit 6).
4. **The measurement**: the path composes; its cost was the Init
   stream four times over (3.25 and 3.26 removed it). Found, open at
   the slice and built at 3.27 landing 3: the dependent match
   (`List.get`'s realization) and the matcher of an Init definition
   realized through its view (`Option.getD`).

*Slice 3.25 (ruled 2026-10-09, item 59; built 2026-10-09):*

1. **The Init stream is a prefix of a load and replayable**: `init_all`
   streams to the end (`INIT (all): N declarations admitted`),
   `ld_reroot` starts a fresh root load from a load's Init state (the
   environment, the stream's state, the ordinals carried; everything
   of the old root reset). Under 3.26 the "stream" is the on-demand
   walk and `ld_reroot` carries K's run of Init alone.
2. **The horizon is the module's, never the stream's**: `(import Init
   NAME)` answered from the ordinal table; every gate that asks
   whether an Init constant is available asks the scope — `el_sees`
   (in the environment and `visible` below the module's horizon) is
   the one availability test, through `first_missing`,
   `int_of_nat_visible`, `arith`'s kit, the measure's kit, a literal's
   `String`, `derive`'s `dv_has`. **Init's names are Init's**: a
   declaration bearing an Init constant's name is `name_taken`
   wherever the environment holds it (from the name index since 3.26
   rule 5). The claim that `std/list`'s `List.sum` collided was wrong
   (declared names carry the module path).
3. **The loader pins' entrypoint streams once**: one load, each case
   on `ld_reroot` of it; 225 cases in 33 s against 783 s at the slice.

*Slice 3.26 (ruled 2026-10-09, item 60; three landings 2026-10-09):*
the unit of an Init load is the line — a demanded declaration's
closure under one percent of the export — so a load reads what it
cites; no fixture, no receipt.

1. **The index** (`kernel/index.shard`; `initindex.shard EXPORT
   INDEX`): one pass over the export writes the record table (one
   fixed-width row per record in export order: kind, byte offset and
   length of its block, the first name, level and expression id it
   holds), the name table (one row per declaration name as
   `show_name` renders it, sorted bytewise with its ordinal) and the
   declaration table (name id → ordinal); the header carries the
   export's byte count, an index over another export refused;
   `ix_verify` reads every record and name back; `init_index.sh`
   keeps `v3/.cache/init.index` current, preferring the route-1
   binary (`v3/bin/initindex`; the export indexed in 36 s, verified
   in 36 s); `index_test.sh` the 38th entrypoint.
2. **The range read**: `host.shard`'s `read_range path off n`, in the
   bootstrap's handler and the chain's runtime; the loader reads the
   export itself.
3. **The load** (`kernel/initload.shard`): `(import Init NAME)` takes
   NAME's ordinal from the name table as the horizon (`init_through`;
   `init_name_not_found`); a module's demand is every token of its
   forms under resolution's candidates plus the gates' kits (`k_kit`,
   `el_kit`, `tc_kit`, `ar_all_kit`, `dv_kit`, `df_kit`) filtered
   below the horizon (`init_prepare`); the walk (`od_open`,
   `od_lookup`, `od_load`) reads each demanded closure by range —
   blocks scheduled highest first, a constant resolved by its name id
   through the declaration table — and feeds K's run of Init alone
   the lines ascending through `process_record`, checked; then
   `od_admit` transfers them into the loader's environment under
   identity (`k/add.shard` `env_transfer`, slice 4.0 rule 4, R78: each
   constant as K's Init run holds it, identical to any constant of its
   name the destination holds, every citation present under the same
   identity or travelling with it, pinned by identity afterwards; the
   typing judgments are the Init run's — before 4.0 the records were
   re-fed in an admit mode that skipped them on the caller's word);
   what K has been fed persists in the process (`OdDone`);
   `ld_reroot` starts a root on that run.
   `INIT NAME: horizon H, N declarations admitted`.
4. **The law unchanged**: visibility is the ordinal below the horizon
   (`init_within` on the true ordinals; `ACCEPT … init=H`); a name no
   kit lists is `unknown_constant` and a defect of the list, found by
   parity; no materialize-and-retry.
5. **Init's names are Init's, from the index**: a file's `Fx.taken` is
   every name the export declares under the file's identity
   (`od_names_under`); a declaration bearing one is `name_taken`; the
   release gate refuses a stored one as `TAKEN`.
6. **No fixture, no receipt**: the prefix fixtures and
   `init_receipt.sh` deleted; every consumer reads the export and its
   index (`--init EXPORT --init-index INDEX`; `verify_release.shard DIR
   EXPORT INDEX` demands every constant the store cites, checked,
   never admitted); the receipt dropped at landing 3 on the
   measurement (a tenth to a fifth of a demanded load; the walk and
   K's ingestion are the cost); the lever that remains is the loader
   native; CI's order export → index driver → index → suite → build →
   replay.

As built, found: K's `unknown_constant` names the declaration, not
the constant it lacks (deferred); the multi-load entrypoints rose on
the runner since each process checks its demand (`wire_test` 155 s →
436 s), the price of no receipt, stated.

**The library — slice 3.27** (records §9.1 "Slice 3.27"; §13 item 61,
ratified 2026-10-10 as designed and as built; four landings
2026-10-10). The arc's principle: the library must not duplicate work
already in Lean's `Init`.

1. **Init is the library.** A statement Init has is cited by Init's
   name; `v3/std` declares only what Init lacks, in Init's namespaces
   under the naming law; an old module every declaration of which is
   Init's or `arith`'s is a migration record and no file (V3's own
   `List.sum` deleted).
2. **One spelling, everywhere — a table, not a transparency.**
   `kernel/spelling.shard`'s `sp_canon` reads a class method at `Nat`,
   `Int`, `Bool`, `List α` under Init's closed instance as its
   function (`HAdd`…`HAppend`, the bitwise classes, `HPow`/`Pow` at
   `Nat` to `Nat.pow` since landing 3); the unifier reads both sides
   through it at a structural mismatch under a metavariable before
   K's whnf (`unify_fallback`); every goal statement and lemma
   equation already enters in the one spelling (3.21), so `rw` and
   `simp_only` follow from the rows. A class the table lacks is the
   door's wake condition. Pin `one_spelling`.
3. **Instances by table, not search** (Stage 3's first step):
   `sp_instance` maps the closed type of an instance-implicit
   metavariable to the pin's instance constant at `Nat`, `Int`,
   `Bool`, `List α`, any type's `BEq` over its `DecidableEq`, the
   heterogeneous classes over the homogeneous row, `OfNat`; asked at
   an application's binders (`el_solve_slots`), at `el_finish` and at
   a tactic opening's premises; anything left `instance_needed` with
   the pointer; `(min a b)`/`(max a b)` operator rows to
   `Min.min`/`Max.max`; `sp_kit` joins the loader's demand. General
   instance search stays Stage 3's door. Pin `instance_table`.
4. **`/` at `Int` is `Int.ediv`** (`op_at`; `%` was `Int.emod`). Pin
   `op_int_div`.
5. **The horizon refusal points**: a cited name the name table holds
   at or above the module's horizon is `above_horizon NAME: ordinal N,
   the horizon H — write (import Init NAME)` (`init_resolve`,
   `Fx.above`); pins `init_above_horizon`, `init_above_horizon_by`.
6. **Init's functions realized by supplied bodies** with their
   equations theorems K proves — by `rfl` where the body follows
   Init's recursion (`List.length`, `append`, `reverseAux`,
   `reverse`), by `cases` on the argument Init recurses on first
   (`take`, `drop`), by cases on the index (`get?Internal`); the
   matcher of an Init definition realized through its own definition
   (`Option.getD (view)`: `beta_head` unfolds `NAME.match_N` with no
   descriptor of this run). A realize's binders are E types (`(i
   Fin)`). Deferred: the derived view of Lean's compiled structural
   recursion.
7. **The dependent match** (`elab.shard`): a `match` on a local
   generalizes, beside the expected type, every local whose type
   mentions the scrutinee, outermost first (Lean's revert), the arms
   re-binding them at the pattern's type; tried first, then the
   expected type's reading, then the constant motive. The refuted row
   is Init's `absurd`/`False.elim`, erased to `EAbsurd` → `LAbsurd`
   (stuck `unreachable` if ever reached; `absurd_needs_init` on the
   E-first route); `alt_with_extra`, `leaf_binders`. `List.get` is
   realized so. Pins `dependent_match`, `absurd_e_first`.
8. **The modules as records** (`v3/std/README.md`, law §10.1): `order`
   (22), `nat` (7), `div` (7), `arith` (7) with no file — their claims
   theorems in `std/migration.shard` (one theorem per row of law
   §10.3, twenty; rows 8 and 14 corrected on the pin: `List.get?` is
   `List.get?Internal`/`List.getD`, `String` a structure over its
   bytes); `std/list.shard` (the realizations, `sum_append`, the
   ghost refinement `Sorted` = `(@Subtype (List Nat) (List.Pairwise
   Nat.le))` with `sorted_insert`, the old 24 requirements as `list_*`
   theorems citing Init); `std/bits.shard` (the old 37 requirements
   citing Init at `Nat`, the width instances at 32 and 64 bits, the
   masks and literal shifts); `Subtype` a row of the registry
   (`Subtype.mk v h` runs as `v`; `tele_of_fields` opens a row
   constructor's parameters unclassified); the carrier named by a
   `def` (a `fn` signature cannot name `@Subtype T P` — a finding, the
   idiom). Calc's `examples/calc/list.shard` retired onto `std/list`;
   the reducer reads a realization's equations (`equations_of`:
   `M.f.realize_k`); the loader marks a `use` needed when a token hits
   an identity the module realized (`mark_hits`). `std_test.sh` the
   39th entrypoint (the std root loaded in one process, stored and
   replayed by K alone).
9. **The fifteen former axioms as theorems** (`v3/std/facts.shard`,
   theorem-only): twelve by Init's names (`Int.emod_nonneg`,
   `Int.emod_lt_of_pos`, `Int.emod_def` under `arith`,
   `Int.ediv_emod_unique`'s halves, the ring laws, the four shift
   lemmas at `Nat`); the three bitwise recurrences by the Euclidean
   split by 2 (`and_div_two`/`or_div_two`/`xor_div_two` and the low
   bit by `and_mod_two_eq_one` and its kin over the four cases, joined
   by `arith` over `Nat.div_add_mod`), stated subtraction-free since
   `arith` reads `Nat.sub` as an atom.
10. **`docs/LEAN.md`** (law §5.4): the three lists as tables — the
    same as Lean, refused with the reason, the ruling and the pointer,
    shard-only — with the loader's command line, the records and
    which refusals end a file in the header; 92 rows at the landing,
    109 since 3.30 landing 2; the refusals the lists name carry their
    pointer (`lean_form`, `inhabited_default`, `lean_symbol`,
    `infix_form`).
11. **T9 small** (`docs/records/T9-small-2026-10-10.md`): pass on the
    count agreed before the run — 0 interventions, 1 refusal before
    success, 7 runs; (ii) measured on route 2 (route 1 compiles the
    kernel's bring-up dialect only — a stated departure); ten gaps,
    seven rows, two kernel fixes (`zero_arity_bare`, the
    `inhabited_default` pointer). T9 ran again at 3.30 landing 2.

As built, found and fixed: the instance table's any-type row
(`sp_inst_typed` → `sp_inst_any`); `arith`'s `lin_same` → `lin_eq` (K
caught an ill-typed `Eq.trans`; pin `arith_eq_bare`). Deferred with
their consumers: the derived view of compiled structural recursion;
the loader native; general instance search; the migration tool's
tier 0 (phase 5's opener, these records its calibration data).

**Templates, lifted lambdas and static packages — slice 3.28 (T2,
T3)** (records §9.1 "Slice 3.28"; §13 item 62, ratified 2026-10-11 as
designed and 2026-10-12 as built; three landings 2026-10-11; law §4.3,
§4.1). The leans: instances named by their flattened shapes, the
budget 1,024, six list templates, packages as structures with no new
form.

1. **The static role.** A binder of a `fn` or a supplied `realize`
   whose type is a function type over runtime data (`erase.shard`'s
   `BStatic k`, `fn_arity`), or of a package type, is a static
   parameter; a declaration with one is a **template**: its L half the
   definition with its equations elaborated once, its E half one
   program per instance; not `RUNNABLE`, no `REALIZE`; checked once at
   its declaration at placeholder shapes (`template.shard`'s
   `check_template`); the record `TEMPLATE NAME static=f,…
   equations=…`. No keyword.
2. **The escape rule**: in a template's body a static parameter
   occurs as a call head at full arity or as a static argument passed
   on; anywhere else — an argument of a non-template, a constructor
   field, a `let` value, a scrutinee, a return position, a partial
   application — `function_value` with law §4.3's pointer, on both
   routes (`unknown_type: ->` and `unknown_head: fun` became it).
3. **Instances, by shape.** A static argument is (a) the name of a
   `fn`, a realized constant or an instance of the parameter's arity,
   (b) a named partial application `(g a …)` to runtime terms, (c) a
   closed lambda lifted to `CALLER.lam_N` with the captured locals its
   leading parameters (`lift_lambda`), (d) a static parameter of the
   enclosing template passed on, (e) a package constant; the shape is
   the identity named and its captured count, the captured values
   runtime data and never part of the key. An instance is the
   template at a tuple of shapes, one E program per distinct tuple
   per module, named `MODULE.T_flat[_TYPE…].of.SHAPE…` (the type
   arguments in the name); its L meaning a generated definition
   definitionally the specialized body, its equations the template's
   instantiated (`NAME.eq_N` from `T.eq_N`), read by the reducer and
   the rewriter; `INSTANCE NAME of=T shapes=g/1,… equations=…`;
   `instance_clash` when a name's demands differ. The E-first instance
   (a body with no L reading): a program and no definition
   (`take_instances`, `read_template_app`, `EBind`), the type
   arguments solved by unification of E types; refused E-first with
   pointers: a lambda, a bound variable, a registry constant, a
   polymorphic fn, a template, a constructor, an extern, a package
   parameter. Pins `template_lift`, `template_named`,
   `template_partial`, `template_passed`, `template_escape`,
   `template_transform`, `template_e_first`.
4. **Finite specialization (T3)**: a static argument is never a term
   built from a static parameter (`function_value`); a template
   calling itself at another shape or at a type built from its type
   parameter is `realize_descent` (pin `template_growth`); the budget
   `instance_budget` 1,024 per template per load (`Load.budget`,
   `--instance-budget N`), past it `specialization_budget` under the
   taker's name (pin `template_budget`); the workload
   `examples/template/workload.shard` takes 64 instances in one file.
5. **Static law-bearing packages**: a `structure` with a function-typed
   field is a package type (`package_of`: one constructor, no indices,
   no E type — no runtime dictionary); a package constant is a `def`
   whose value K's whnf reads as the constructor applied, its function
   fields static arguments, its laws erased, its values closed E terms;
   a binder of a package type is a static parameter of arity 0, its
   shape the constant's identity (`IntAdd/0`); a projection erases at
   the instance as the field's shape applied (`erase_package_proj`,
   `package_subject`, `package_field_shape`); a structure's parameters
   are implicit in its constructor and projections (`implicit_prefix`);
   the generic theorem over `m` is proven once and applies at the
   constant by unification. Pin `template_package`.
6. **Init's higher-order list functions as templates in `std/list`**:
   `List.map`, `filter`, `foldl`, `foldr`, `any`, `all` realized by
   supplied bodies whose function binder is static (`realize_template`,
   `static_signature`), their equations proven by K (`filter`'s through
   the `equations` clause by `List.filter_cons`); law §5.3's
   `add_offset` written as the law writes it.
7. **The gates**: `kernel/test/template_test.sh`, the 40th entrypoint,
   over `v3/examples/template/` (`add_offset`, `named`, `package`,
   `workload`, `template_main`) — the loads, the runs, the artifact
   check over `--dump` (no `fun` and no `->` in any program line while
   the sources hold them); `docs/records/T2-T3-2026-10-11.md` records
   T2 and T3 against law §12.5's risk columns.
8. **Route 1 unchanged**: the artifact check is on the E programs `ev`
   runs; route 1 over user files is phase 4's.

Deferred with their consumers: the dynamic tier (law §4.3's door);
instances shared across modules; an extern as a static argument; a
lambda returning a lambda; a function-typed field of an E type.
Rejected: a runtime closure or dictionary; captured values in the
key; an instance per call site; a `template` keyword; untyped lambda
binders.

**The tactic library — slice 3.29 (T10)** (records §9.1 "Slice 3.29";
§13 item 63, ratified 2026-10-12 as designed and 2026-10-13 as built;
two landings 2026-10-12; law §7.2, §7.3, §7.5, §5.3 departure 6). The
leans: the fixed interface is data at the boundary, one shot; the
producer is named in the source's delegation; the sidecar carries I's
schema; the guarded occurrence carries its expected subterm.

1. **The tactic signature.** A tactic is a `fn` of an ordinary module,
   `(fn NAME ((goal TEXT) (hints TEXT)) TEXT)`, TEXT any byte type the
   entry codec carries, no World; the goal arrives as `(goal ((NAME
   TYPE)…) TARGET (applicable SYM…))` rendered by the one renderer the
   refusals use (`tc_show`, `goal_sx`; binders as source writes them),
   the hints as the S text after `(tactic M.FN)`; the answer a block
   `(STEP…)` or the empty text. The library sees no `El`, no `Expr`,
   no environment.
2. **The delegation names it.** `(auto (tactic M.FN) HINT…)` in a
   `theorem`, a `fulfills` or a discharge; under `prove` (mode
   `solve`) a theorem without a valid entry runs the tactic under `ev`
   on the loaded programs (`reader.shard`'s `Producers`: the sidecar
   entries, `Load.etab`, `Load.fuel` — `default_fuel` or `prove.shard
   --fuel N`; `run_tactic`, `prog_find`, the texts through
   `val_of_text`/`text_of_val`) and replays the answer through
   `tac_prove` before acceptance, the entry it writes any entry;
   `PENDING NAME auto_unsolved fp=N tactic=M.FN: REASON` for an unknown
   name, a wrong signature, a refused step with its goal, no answer,
   an answer that does not read, several forms, `ev` stuck or
   exhausted, two tactics, `(tactic)` without a name; under
   `load.shard` the delegation is unread — the build replays the
   sidecar only; under `--measure` `TACTIC NAME M.FN solved|unsolved`
   beside `ENGINE`.
3. **The refusals**: an unknown step is `unknown_step` with I's roster
   and the pointer "a new tactic is an E library producing these
   forms, named by (auto (tactic M.FN)) (§8.4 slice 3.29)"; `notation`
   and its kin keep `lean_form`'s pointer naming the library route;
   `(tactic …)` inside a hand-written `by` is `unknown_step`.
4. **The schema.** A sidecar's first form is `(sidecar N)`, I's schema
   version, 2 since this slice; a sidecar without it is schema 1,
   replayed and reported once per file (`SIDECAR FILE schema=1
   current=2`); one above the current is the load error
   `sidecar_schema`; `prove` writes the current whenever it rewrites a
   file whole (an entry still valid is never rewritten). Law §7.2's
   four versions in V3: K's rules (pinned by T0), I's schema (this
   form), P's encoding (the store's format 3.1.0), the producer's
   implementation (the sidecar's comment). Pins `auto_schema_old`,
   `auto_schema_future`.
5. **The guard.** `(rw LEMMA (occ K EXPECT) STEP-BLOCK…)`: the
   instance the first match found must be EXPECT — by structure first,
   by the elaborator's unification second (`rw_occ_check`) — else
   `rw_occ_changed` showing both (the source moved under the guard, or
   the expectation written is wrong, since 3.30 landing 2); `(occ K)`
   alone stays Lean's. Pin `by_rw_occ_guard`.
6. **The dependent motive.** Under `cases` and `induction` every local
   whose type mentions the eliminated local (transitively), and the
   equation hypothesis when named, is reverted into the motive — the
   equation first, then the dependents — and each arm re-introduces
   the fields, then the equation, then the dependents under their
   names (`cases_minors`, `ctx_dependents`); the eliminated local
   leaves the arm's context unless an equation names it; an induction
   hypothesis quantifies over the dependents. Pin `by_cases_dependent`.
7. **Sibling names, external P, settings — pinned, nothing built**: two
   arms naming the same fields and `have` (`by_sibling_names`); a
   theorem closed by an `exact` term from outside the elaborator's
   producers (`by_exact_external`); a sidecar replaying unchanged when
   the hints changed or an unrelated lemma module is imported
   (`auto_hints_changed`) — an entry carries its rules, there is no
   ambient simp set, Init's instances are closed by the export.
8. **One solution checked identically**: the example's theorems solved
   by the engine and by the library, each sidecar replayed under
   `load.shard` to the same `ACCEPT` records (the name, module,
   fingerprint, horizon and parameters; the axiom closure the proof's
   — the engine's `arith` rests on `propext` and `Quot.sound`, the
   library's `exact` on nothing), each stored and verified by K alone.
9. **The example and the gate**: `v3/examples/tactic/bool_split.shard`
   (an E-first library over `kernel/sexpr.shard`: the context's `Bool`
   locals split by nested `cases` with `(decide)` at the leaves, a
   reflexive target by an `exact` term), `use_split.shard` with six
   theorems delegated to it and its committed sidecar (a statement
   over Init's names alone fingerprints the same in every module);
   `kernel/test/tactic_lib_test.sh`, the 41st entrypoint (the replay,
   the regeneration byte-identical, the engine's sidecar for the same
   statements, the schema-1 copy reported, the hints-changed copy
   replayed, `--measure` 6 of 6 by each producer); pin `auto_tactic`;
   `docs/records/T10-2026-10-12.md`; `LEAN.md`'s rows for a tactic
   extension, the delegation, the schema form, the guard, `--fuel`.
10. **Not in 3.29**: a library composing `step` in-process (phase 4,
    route 1 over user files; the engine stays the kernel's);
    `attempt`/`commit`, owners and snapshots; incremental goal states
    (T6); `rw` at a hypothesis; a tactic with World access; several
    tactics in one delegation.

**Slice 3.30 — the phase-3 close-out: the gate ledger, the carried
rows, T9 again, the consolidation** (designed 2026-10-13 at the
boundary after 3.29, on the user's "Let's proceed to the next slice";
§13 item 64 — **ratified 2026-10-13 as designed, the leans (a)–(f)
taken (the user: "Those leans sound reasonable: ratify them")**; law
§12.4 item 3, §12.5's T9 row, §10.5). Law §12.4 names
phase 3's content and its gates (T2, T3, T9 small, T10); §5.4 asks for
T9 "again at the end"; the user ruled a LANGUAGE.md consolidation at
the phase-3 close (2026-09-17, records §9); phase 2 closed with a
ledger of its gate items (records §9, 2026-09-13). Where phase 3
stands, measured 2026-10-13 against the law's list:

| item of law §12.4 item 3 | evidence | status |
|---|---|---|
| the loader to the whole of E, the profile retired | slices 3.1–3.5; pipeline 487 | covered |
| Stage 1 | slices 3.13–3.19 | covered |
| Stage 2: the I elaborator, the goal graph (law §7.3), core tactics | slices 3.20–3.22: `goal_of`, `applicable`, `step`, `elaborate` as E functions over one derivation, I's data in the graph's shape; `intro apply exact rfl rw simp_only cases induction have show decide unfold reduce wf arith` — the law's `change` is `show` (the target restated, up to conversion); incremental goal states and holes shared across goals are T6's and T4's, phase 4 | covered |
| certified arithmetic | slice 3.21 (`arith`, a Farkas certificate over Lean.Omega's kit) | covered |
| the `Init` import with E realizations attached | slices 5b, 3.18, 3.26 | covered |
| `std/list`, `order`, `nat`, `div`, `bits`, `arith` under the naming law | slice 3.27: `list` and `bits` as files; `order`, `nat`, `div`, `arith` as migration records and no file (ruled 2026-10-10, item 61) | covered as ruled |
| the fifteen former axioms as theorems | `std/facts.shard` (3.27 landings 2–3) | covered |
| the migration table validated | `std/migration.shard`, `std_test` | covered |
| one arbitrary-`Prop` ghost refinement | `std/list`'s `Pairwise` subtype (3.27 landing 3); T9 small (iv) | covered |
| one static law-bearing package | slice 3.28 (T2) | covered |
| `LEAN.md` | 3.27 landing 4; 108 rows today | covered |
| the first release bundle with retained P | 3.22 landing 3 (`v3/release/`, `verify_release`) | covered |
| the pipeline measured | every pipeline's record, records §9 | covered |
| gates T2, T3 | `records/T2-T3-2026-10-11.md` | met |
| gate T9 small | `records/T9-small-2026-10-10.md` | met; **T9 again owed at the close** |
| gate T10 | `records/T10-2026-10-12.md` | met |
| Stage 3 (typeclasses, coercions) | the fixed instance table (3.27 rule 3), the `Nat → Int` coercion (3.15); search stays the door (item 61) | as ruled; the row stays |
| §11's T1 row, the branch-local proof | slice 3.24 (`dif`, `Fin.mk`; pins `dif_ok`…, `examples/path`) | covered; the row stale |
| §11's T5 row, the imported-theorem form | **measured today**: `List.append_nil` cited before a supplied realization of `List.append`, the realization attached with its equations, the theorem cited after and a theorem citing the earlier one — every record `ACCEPT`, 39 s | closes by a pin |
| §11's T5 row, two instances of one interface; R57's checked instance | not built; no consumer in the tree needs two implementations of one view | **ruling asked** (rule 3) |
| §11's `Symbol` row, the `String` flip | open since 3.18, "with route 1's chain" | **ruling asked** (rule 3) |
| §11's R60 row; the lambda-lifting row | slices 3.17–3.18; slice 3.28 | covered; the rows stale |
| law §10.5's phase-3 doc rows | `LEAN.md` written; `TOTALITY.md`, `REFINEMENT.md`, `STORAGE.md`, `CERT.md` carry no banner (measured: none names FOUNDATION) | owed |
| §13's ratification | items 39–42, 44–46, 48 carry no ruling marker; 43, 47, 49–53 ruled by the user at their slices; 54–63 ratified | owed: eight items |
| the consolidation | 7,579 lines, §8.4 4,437 of them in twenty "as built" sections; the title "at Stage 0 (phase 2 draft)"; §11's rows stale | owed (rule 6) |

**The rules of slice 3.30:**

1. **The gate ledger.** The table above, each item with its evidence,
   becomes records §9's "Phase 3 close-out ledger" as phase 2's was;
   §11's phase-3 rows are rewritten to their state — landed at which
   slice, or carried with the reason and the phase.
2. **The carried fixture built.** Pin `realize_imported`, T5's
   imported-theorem form in the probe's shape (the theorem cited
   before and after the realization, a theorem citing the earlier
   one); the §11 row closes. No kernel change is expected.
3. **The two rows that stay open, for ruling.** (a) T5's two
   validated instances of one interface and R57's checked instance:
   the mechanism for one implementation is built (the fork from the
   view's fork point, `PARAM`/`DISCHARGE`, the substitution as the
   link); a second needs a loader rule naming which file implements
   and the checked-instance record by substitution. The lean: carried
   to phase 5, whose bulk port is the first consumer with two
   implementations of one view — built as a landing when that
   consumer appears. The alternative: built now, one landing with a
   pin of two implementations and one consumer proof under both.
   (b) `Symbol`'s L identity `String` and the flip of `"…"` to a
   `String` value for every file at once: the lean, carried to phase 4
   with route 1's chain, as 3.18 stated it.
4. **The doc rows.** `TOTALITY.md`, `REFINEMENT.md`, `STORAGE.md` and
   `CERT.md` gain the banner `TCB.md` has: the document governs the
   old tree until the flip, the V3 answer named by section (§4.5,
   §4.1, §7.5 and §8.4, §7.5 and §8.3).
5. **T9 again** (landing 2), the protocol of T9 small: a fresh agent
   with `LANGUAGE.md` and `LEAN.md` and nothing else, the loader's and
   `prove`'s command lines, `std/` importable. Nine tasks: (i)–(vi) of
   slice 3.27 rule 11 again over today's tree; (vii) a static
   law-bearing package — a `structure` with a `fn` field and a law —
   used generically (T2's facility, 3.28); (viii) a tactic library of
   one constant tactic, delegated to from a theorem through `(auto
   (tactic M.FN))` under `prove` and replayed from the sidecar by the
   build (T10's facility, 3.29); (ix) the T9 row's diagnostics: a
   lemma cited at two occurrences, the second wrong, the refusal
   naming the occurrence by its goal; a measured `fn` with two
   recursive calls, one not descending, the obligation naming the
   call. **The count, agreed before the run:** 0 interventions; at
   most 2 refusals before success on each task; (vi)'s three pointers
   received; every gap a `LEAN.md` row; a refusal with no pointer or
   a wrong pointer a finding against the kernel; pass = all nine
   accepted with no workaround from a forbidden source. The record
   `docs/records/T9-2026-10-DD.md`, the report verbatim.
6. **The consolidation** (landing 3), its shape for ruling. The lean:
   each slice's design and "as built" sections of §8.4 move verbatim
   to records §9, the build history's home by the status block's own
   words, and §8.4 keeps, per facility, the current statement of each
   rule with its as-built note beside it (R62's principle, slice 10):
   definitions (3.13–3.16), the porting facilities and bytes
   (3.17–3.18), deriving (3.19), I (3.20–3.21b), the producers and the
   store (3.22), `Init` (3.23–3.26), the library (3.27), templates
   (3.28), the tactic library (3.29); the title and the status block
   rewritten for the phase's close ("S, L, E and I as phase 3 built
   them"); the slice table stays as the index, each row pointing at
   its record. Text moved, never rewritten, so the diff reviews as
   moves. The alternative: an index over the chronological §8.4 and
   nothing moved — cheaper, and the document stays a ledger. No
   kernel change; GPT-6's memo over the SHA is the review.
7. **The close** (landing 4): the ratification pass over §13 items
   39–42, 44–46, 48 and 64; the close-out ledger of rule 1 complete;
   the SHA for GPT-6's checkpoint memo, as phase 2's (R49–R54); phase
   4 opens, its first slice designed at the next boundary from law
   §12.4 item 4 (a `tb_len`-class compiler proof; T4 and T7; T6; T8;
   the World-use fixture).

*Not in this slice, stated:* phase 4's items; the two-instance gate
and the `String` flip if ruled carried; the migration tool's tier 0
(phase 5's opener, item 61).

**Slice 3.30 as built, landing 1 (2026-10-13) — the ledger, the rows,
the pin, the banners.** Rules 1–4, with what the build found.

- **Rule 1.** The phase-3 close-out ledger is in records §9 beside
  phase 2's, the table above with each row's state at the landing.
  §11's ten phase-3 rows are rewritten to their state: landed at
  their slices — Stage 2 at 3.20–3.22 (the law's `change` read as
  `show`, the target restated up to conversion; shared holes and
  incremental goal states phase 4's), the `Init` import at 5b, 3.18
  and 3.26, lambda lifting at 3.28, T1's branch-local proof at 3.24,
  R60's facilities at 3.15–3.18 — or carried by rule 3's rulings (the
  two instances and R57's checked instance to phase 5, the `String`
  flip to phase 4), Stage 3 as item 61 ruled it.
- **Rule 2.** Pin `realize_imported` (261 cases): Init's
  `List.append_nil` cited before a supplied realization of
  `List.append` is attached, the same theorem and a theorem that cited
  it before cited after it — every record `ACCEPT`, the three
  statements one fingerprint. The `(view)` form is refused
  `recursion_structure`: a recursive Init body is supplied, as
  `examples/path` supplies `List.length`'s. No kernel change.
- **Rule 4.** A banner after each document's STATUS line, in
  `TCB.md`'s shape: `TOTALITY.md` superseded by law §4.5,
  `REFINEMENT.md` by §4.1, `STORAGE.md` by §7.5 and §8.4, `CERT.md`
  amended (§3 and §7 superseded, §4's validators and §10's laws
  carried), each naming the V3 answer by slice and the old tree as
  the document's until the flip.
- **Gates run:** the loader pins' entrypoint 261 cases, 0 failed,
  134 s (under a load of 31 from other sessions' jobs); the full suite's 41 entrypoints, 0 failed, in 816 s under the same load (parity byte-identical over 31 closures and 136,735 declarations in 383 s; the suite's 548 s of landing 2 on a quiet machine).
  Route 1's kernel build, the full replay and the corpus are CI's.

**Slice 3.30 as built, landing 2 (2026-10-13) — T9 again.** Rule 5,
with what the run found (`docs/records/T9-2026-10-13.md`, the report
verbatim).

- **The run.** A fresh general-purpose agent with `LANGUAGE.md` and
  `LEAN.md` and nothing else (its reading set checked against its
  transcript afterwards: the two documents, `std/`'s README and files,
  the loader's and `prove`'s command lines) wrote the nine tasks in a
  scratch root. **Pass on the count as written:** 9 of 9 accepted; 0
  interventions; refusals before success (iii) 2 — both the author's,
  a guessed lemma applied past its implicit binders and `(- 0 a)` for
  `Neg.neg` — and (ix b) 1 unintended before the intended; (vi)'s
  three pointers received; (ix a)'s refusal names the step, both
  instances and the goal after the first rewrite, (ix b)'s obligation
  names the call by its index and statement with the descending
  call's discharge beside it; every guessed Init name of (iii)
  resolved (eight); 9 loader runs and 1 `prove` run, 232 s, about 50
  minutes end to end. Verified by a second load of the nine accepted
  files in one process: 245 declarations accepted, 0 refused, 0 errors.
- **Two findings against the kernel, fixed with pins.** (1) The
  author's: a `fulfills` of an obligation of a `fn` refused above was
  `fulfills_outside_impl: give the loader the directory (§6.6)` — the
  wrong cause. `loader.shard`'s `fulfills_misnamed` names the cause:
  `fulfills_refused` (a prefix of the name refused above, its
  obligations never stated), `fulfills_not_obligation` (a declaration),
  `fulfills_outside_impl` (nothing of the load); the reader's `RDLater`
  fallback is gone. Pins `fulfills_refused` (under the harness's new
  kind `load-error-after CODE`, the refusal standing first),
  `fulfills_not_obligation`; `fulfills_outside` keeps its code. (2)
  Found while scoping the fix: inside an implementation directory a
  measured `fn`'s obligation could not be discharged — the
  implementation path matched a `fulfills` against the view's
  requirements only (`fulfills_unknown`), the obligation pending. It
  now discharges as in a plain file (pin `view_fulfills_obligation`:
  `DISCHARGE … proved`, the definition's account recomputed). 264
  cases.
- **The gaps.** Fourteen reported: ten `LEAN.md` rows or additions
  (the unary negation row new — `(- a)` is `bad_shape`, measured; the
  `decreasing_by` row's discharge with the goal's shape; `PARAM`,
  `DISCHARGE` and `UNUSED` in the records list; the horizon row's wide
  horizon; a guessed lemma's binders; a theorem's explicit binders
  against a `fn`'s implicit type parameters; `(unfold f)` before
  `(rw (dif_pos h))`; `Subtype.mk` bare or under `@`, both measured
  through a `def`-named carrier; the `(occ K EXPECT)` row's two causes,
  `tactic.shard`'s `rw_occ_changed` message amended to say both; the
  self-call at the parameters themselves); one a spelling in slice
  3.28's rule 5 corrected above (the structure's fields directly after
  the binders); one half the author's reading of §3.2, half the
  records list; one the brief's own wording (a `PROOF` record the tree
  does not have); one finding (1). `LEAN.md` 109 rows.
- **Gates run:** the loader pins' entrypoint 264 cases, 0 failed, 54 s;
  the full suite's 41 entrypoints, 0 failed, in 536 s (parity byte-identical over 31 closures and 136,763 declarations in 326 s — the 28 declarations more the landing's kernel growth; `store_test` 415 s, `tactic_lib_test` 279 s, the loader pins' entrypoint 56 s for 264 cases; a quiet machine). The two
  edited kernel files clean under the V3 loader; `names.py` at its
  baseline. Route 1's kernel build, the full replay and the corpus are
  CI's.

**Slice 3.30 as built, landing 3 (2026-10-13) — the consolidation.**
Rule 6 by its lean (§13 item 64(e)).

- **The move.** The design and as-built texts of slices 3.13–3.29 —
  4,391 lines, from "The rules of slice 3.13" to the line before this
  slice's own text — moved verbatim to records §9.1, one `#### Slice
  3.N` heading per slice added as the record's index and nothing
  else; the moved text's SHA-256 prefix `a500557b5ca452f3`, taken before
  the cut and after the paste, equal. `LANGUAGE.md` 7,818 lines to
  4,544; the records 3,223 to 7,654.
- **What stays.** Per facility, the current statement of each rule
  with its as-built note beside it, numbered as its slice numbered
  it — definitions (3.13–3.16), the porting facilities and bytes
  (3.17–3.18), deriving (3.19), I (3.20–3.21b), the producers and the
  store (3.22), `Init` (3.23–3.26), the library (3.27), templates
  (3.28), the tactic library (3.29) — 1,115 lines, a quarter of the
  moved text, each slice labelled, so that `§8.4 slice N rule K` (151
  pointers in the kernel's comments and messages, `LEAN.md`'s rows,
  the READMEs) names a rule here and its full text in the record; a
  rule another slice superseded says so and points on (3.13 rules 2,
  4 and 5, 3.14 rule 3, 3.23 whole). The slice table stays as the
  index, each row ending in its record's heading, "below" read as
  the record. The title is "S, L, E and I as phase 3 built them" and
  the status block is rewritten for the close. This slice's own text
  stays here until the close.
- **The departure, stated.** The current statements are new prose —
  a compression written from the full texts facility by facility,
  not moved text; where a statement and the record differ the record
  is the authority, and GPT-6's memo over the SHA is the review
  (rule 6). No kernel change and no test run: the landing is
  documents, the kernel's pointers unchanged in text.

**Slice 3.30 as built, landing 4 (2026-10-13) — the close.** Rule 7.

- **The ratification pass.** §13 items 39–42, 44–46 and 48, and item
  64's as-built notes of landings 2 and 3, ratified 2026-10-13 (the
  user: "Read through this, and all of that looks good to ratify to me"), item 46 with its reason stated: "the requirements holding the opaque boundary of the module is an important boundary to keep so that internal mutations that preserve requirements are safe without rippling to all consumers" — the view's
  requirements are the consumer's whole knowledge so that an
  implementation may change under them without a consumer's proof
  moving. Every item of §13 now carries its ruling. Item 42's stated
  cost was found live at the close (route 1's reader expands a record
  under v2's accessor names; the kernel's drivers wait on a pre-pass)
  and is noted at the item.
- **The ledger.** Records §9's phase-3 close-out ledger complete and
  closed beside phase 2's: every item of law §12.4 item 3 covered or
  carried by ruling (T5's two instances and R57's instance to phase 5,
  `Symbol`'s `String` flip to phase 4 with route 1's chain), T2, T3,
  T9 small, T9 again and T10 met.
- **The checkpoint review.** This landing's commit is the SHA for the
  phase-3 checkpoint memo, as phase 2's close-out commit was for
  R49–R54. The review runs through `codex` (GPT-6 by its CLI; the user:
  "feel free to use 'codex' with cli options for an outside perspective
  with a different model's viewpoints"), over the tree at the SHA with
  this document, the law and records §9 as the brief's reading; the
  memo is archived under `docs/archive/foundation-v3/` and answered in
  records §4 as every memo before it.
- **Phase 4 open.** Phase 3 closed at this landing; phase 4 (law §12.4
  item 4) open, its first slice designed at this boundary and written
  for ratification as §13's next item. Not in this landing: that
  design.

No kernel change and no test run: documents only.

### 8.5 Views under Stage 1 — the interface is the whole of a consumer's knowledge (RULED 2026-09-17)

The user's steer at the Stage-1 design: v2's module system was built
so that a consumer of a `mod.req.shard` never resolved the
implementation to reason about its surface — the requirements stood
in for the lemmas about an opaque implementation, and a proof that
needed to unfold a body was the signal that the surface was
incomplete, answered by exporting the fact, never by piercing (memory
`module-system`, the surface-discipline correction). V3 keeps that by
construction, and Stage 1 and I are where it would be crossed, so the
stance is stated now (§13 item 46):

- **As built** (§6.6, `check_impl`): a consumer's environment holds
  each `sig fn`, `sig type` and `requirement` as an axiom-kind
  parameter K cannot unfold; the implementation is checked in a fork;
  the merge carries the fork's records and its E table (so `run`
  links bodies) and never an L declaration — the caller's environment
  is the caller's. That is v2's mode split: bodies for execution, the
  interface alone for reasoning.
- **Stage 1 declares an implementing `fn`'s definition and its
  `eq_N` in the fork only**, under the module's identity (item 39),
  where the definition takes the parameter's place; nothing of it
  crosses the merge. A consumer's environment holds, for anything the
  view declares as `sig`, exactly the parameter — never a definition,
  an equation, a realization record or a derived instance. Pins: a
  consumer citing `DIR.f.eq_1` is `unknown_constant`; a consumer
  theorem about `(DIR.f x)` by `Eq.refl` is K's refusal.
- **The only lemmas about a `sig fn` or a `sig type` are the view's
  `requirement`s and `theorem`s.** A module that wants its defining
  equations public writes them as requirements (v2's weaning rule
  carried: the consumer then depends on a fact the module commits to,
  not on today's body). An automatic re-export of `eq_N` through a
  view is a deliberate later form if ever, never the default.
- **The checked instance (R57, §6.5) is built on P by substitution**:
  the view parameters replaced by the implementation's declarations
  and the `fulfills` evidence, the composite re-checked by K as terms.
  Never by re-elaborating the consumer's source or I against the
  implementation's environment — a proof produced against an opaque
  parameter cannot depend on the body, so the substituted term does
  not either, and no tactic (`rfl`, `decide`, `simp_only`, `unfold`)
  sees a body across a view.
- **Deriving and the constructor-structure facilities** (`noConfusion`,
  `injection`, decidable equality) on a `sig type` are refused in a
  consumer with the pointer to the view; the module derives and
  exports. Since slice 3.19 the export is a `sig fn` of the
  capability's type and the consumer registers it with
  `(CAPABILITY by NAME)` (§8.4 slice 3.19 rule 6; pin `derive_sig`).
- **The E side stays as ruled** (items 18, 40): the substitution links
  bodies for `run` only.
- **I inherits it by construction**: `applicable(goal, env, policy)`
  runs over the consumer's environment and can never offer an
  unfolding of a parameter — opacity by selective loading, as v2's,
  never by a name gate.

### 8.6 Phase 4 — entrenchment (opened 2026-10-13 at phase 3's close)

Law §12.4 item 4 names the phase's content: a `tb_len`-class compiler
proof; coupled program/proof holes and refutation scope (T4, T7);
incremental goal states; prepared invocation, stale handles, buffer
rules and reclamation (T6); evaluation reflection and its cost
comparison (T8); the World-use and model tests before any effectful
certificate is called ported. Carried in by ruling: `Symbol`'s
`String` flip with route 1's chain (slice 3.30 rule 3b, item 64) —
re-attributed 2026-10-16 (§13 item 69) to Init's types in the
toolchain's closure, phase 5's opener design.
Route 1 — K compiled by shard's own lowering (law §9.1) — is the
engine the compiler proof is about, and today it compiles two of the
kernel's drivers; the phase opens by compiling the rest, so that
every later slice of the phase is built and measured on the engine
it entrenches. The slices, in the order built:

| slice | content | first consumer |
|---|---|---|
| 4.0 | the checkpoint memo answered (R78–R85, records §4.12): the document corrections; the release gate's exit, kind and assumption-closure checks against a manifest; the seal's checked transfer and generator run — **ratified 2026-10-14 as designed; landing 0 built 2026-10-14 (the documents); landing 1 built 2026-10-14 (the release gate), green on pipeline 602; landing 2 built 2026-10-14 (the seal), green on pipeline 603 — slice closed** | the ledger; the release; K's clients |
| 4.1 | the native toolchain: route 1 over the kernel's drivers and harnesses through the V3 loader's projection; the byte-tie gate; the suite on the binaries — **ratified 2026-10-14 as amended by R84; landing 1 built 2026-10-14 (the emitter, the build, the tie), green on pipeline 608; landing 2 built 2026-10-14 (the shim, the suite on the binaries, CI's comparison), green on pipeline 610 — slice closed** | every author's load; the suite; the compiler proof |
| 4.2 | the tb_len rung: the per-function certificate of `len` over the call-tier imp machine and the counted-heap runtime (the coverage arc's Theorem B at one function; records §5's provocation), ported by the migration classes under `v3/models/imp` with `std/map` and `std/mem`, stated as an L theorem and proven through I; the count against the old proof's 717 lines and 155-claim closure; then `app` — **designed 2026-10-15; ratified 2026-10-15 as described (the user: "That all makes sense -- agreed. Let's proceed with the tb_len rung as described."); landing 0 built 2026-10-15 (the port, the definition gate, the count), green on pipeline 616; landing 1 built 2026-10-15 (the proof through I, the kit by need, the count), green on pipeline 617; landing 2 built 2026-10-16 (`app`: the runtime's laws as six parameters, the output ghost existential, the count — the as-built readings for ratification at item 67), green on pipeline 620 after the harness fix `7ef6855`; the landing-2 readings ratified 2026-10-16 as built — slice closed** | the per-function certificate family (B-1c, phase 7); the manifest's first model file (phase 5) |
| 4.3 | the program side (law §4.7): the well-threadedness check over the erased E program with the law's fixtures as pins; an extern's L meaning (an opaque constant) so a requirement can name the entry; the `bin` form with `entry`, `externs`, `trusts` and `requires`, its record, manifest line and release check, an unmet requirement failing the load; calc's main the first bin, two requirements over its observables through nine bolts; the effect trace tied four ways (both engines, the model, route 1's binary) and the bolt bundle tested on the model world — **designed 2026-10-16; ratified 2026-10-16 as described (the user: "The program side sounds good as described -- proceed."); landing 0 built 2026-10-16 (the check, nine pins, the kernel's drivers measured: one stale World found and fixed), green on pipeline 625; landing 1 built 2026-10-16 (an extern an opaque with a witness, the `bin` form, calc's bin with its two requirements proven, eight pins, `bin_test.sh`), green on pipeline 628; landing 2 built 2026-10-16 (the trace, the model's trace, the model test, the count; route 1's binary blocked at the chain's representation of Init's types, two emitter refusals pinned — the deferral to phase 5 for ratification), green on pipeline 629 (the landing's `build.sh` index lookup found building the index on the bootstrap in both CI jobs, fixed at the close); slice closed 2026-10-16, the deferral ratified** | phase 5's effectful ports (the toolchain's drivers, `examples/sha256sum`); the goal's `bin shardfmt`; T6's prepared handle |
| 4.4 | the bridge: Init's types onto the chain at the emitter (a string literal as the closure's constructors; the dotted primitives mapped by name or by expression, `Nat.sub` truncating), no edit of the old tree; landing 0 the probe as the kill-test; landing 1 the bridge, the two refusal pins flipped to ties, calc's binary built by route 1 and tied on its observables; landing 2 the rename of the kernel's comparison spellings by tool — **designed 2026-10-16 at slice 4.3's close with three law changes (item 68's deferral reopened on evidence, the flip decoupled from the chain's replacement, the five dialect debts re-attributed); ratified as outlined (the user: "Agreed with the plan changes as outlined; update the docs."); landing 0 built 2026-10-16 (the probe: calc's binary through a scratch bridge ties the driver; the mechanism Init's wire types identified with the chain's prelude); landing 1 built 2026-10-16 (the bridge in `kernel/emit.shard`, the two pins flipped, calc's binary in `bin_test.sh` and the tie), green on pipeline 633** | phase 5's effectful ports' binaries; V3's own chain's first generation (phase 7); the goal's `bin shardfmt` native |

**Slice 4.0 — the checkpoint memo answered: the documents, the release
gate, the seal** (designed 2026-10-14 at phase 4's opening, on the memo
R78–R85 of records §4.12; §13 item 66 — **ratified 2026-10-14 as
designed (the user: "Agreed, go ahead with 4.0")**; law §3.2, §7.5, §12.4 item 4). The
phase-3 checkpoint memo (records §4.12;
`docs/archive/foundation-v3/SHARD_V3_PHASE3_CHECKPOINT_MEMO_v0.1.md`)
found three source-traced defects on the admission and release paths,
two drifts in this document and the ledger, one contract gap and two
risks; every observation was verified against the tree at `bea4345`.
As slice 9 answered R49–R54 before phase 3 opened, this slice answers
R78–R85 before 4.1 builds the engines that would otherwise agree with
the defects. The rules:

1. **The documents (landing 0; R81, R82, R83, R85, R84).** The four
   current statements of §8.4 that kept a superseded rule are rewritten
   to the final behaviour with their supersession pointers: 3.13 rule 3
   (`-`, `/`, `mod` at `Nat` resolve on the typed route, 3.16 rule 4;
   the E-first route keeps `nat_operator`); 3.14 rule 1 (the entry
   found by type, `recursion_depth` retired at 3.16); 3.16 rule 5
   (`T.casesOn` under the bundle's conditions); 3.20 rule 8 (the proof
   checked in the file's environment with the obligation standing, the
   closure the acyclicity check). §11's R60 row no longer says `Name`
   literals landed (item 51's deferral); 3.22's verifier spelling is
   `DIR EXPORT INDEX`. 3.22 rule 2 names its key the statement
   fingerprint and states law §5.2's three identities as owed, with a
   §11 row placing them in phase 4 before T6; a §11 row for occurrence
   provenance (R85) before the incremental goal-state slice. Records
   §9's ledger says "covered" in four terms — established, recorded
   experiment, covered under a ratified restriction, artifact built —
   with the release row pending rules 2–3 and the T9 and T10 rows
   stating their readings. Slice 4.1's design takes R84's points 2–5
   (its rules 1, 3 and 4 amended, rule 8 added) and item 65 notes the
   amendment.
2. **The release gate's exit (landing 1; R80).** `verify_release`
   succeeds exactly when every `order` line's file was read; each file
   yielded exactly one accepted declaration, under its line's name,
   kind and fingerprint; no verdict of a failed class (rejected,
   exhausted, mismatched, unsupported) and no malformed, mistied,
   missing or taken record occurred; and the Init closure's failures
   are zero. Otherwise the exit is 1; the counts stay in the
   `RELEASE:` line. Pins in `store_test.sh`: one file holding an
   accepted and a rejected record; one file holding two accepted
   records under two names; an inductive whose metadata mismatches
   after its acceptance.
3. **The release gate's target and policy (landing 1; R79).** The
   store writes, beside `order`, a `manifest`: per file the
   declaration's kind, name and statement fingerprint, the file's
   declared parameters (`PARAM`) and its discharged obligations with
   their proofs' names; the format's version is bumped. The manifest
   is the fixed target: `order` is the admission sequence only, and a
   manifest entry with no order line is `MISSING`. The verifier refuses
   an accepted record whose kind differs from the manifest's
   (`MISKIND`: a theorem replaced by an axiom). The policy: the
   verifier walks each accepted theorem's assumption closure through
   K's axiom memo (the walk `t0.shard`'s `-a` lines use) and compares
   it with the declared set — Init's axioms, the file's parameters, a
   discharged obligation standing for its proof's own closure, the
   proof required accepted at the obligation's statement — and reports
   a theorem whose closure exceeds the set as `CONDITIONAL NAME:
   AXIOM…`; a bundle verifies as conditional only when its manifest
   declared the parameters it rests on, else the exit is 1. Pins: a
   theorem replaced by a same-name same-type axiom (`MISKIND`); a proof
   rewritten to rest on an axiom the manifest does not declare
   (`CONDITIONAL`, exit 1); a manifest target whose order line and file
   are removed (`MISSING`); a discharge entry removed from the manifest
   while the obligation axiom and its consumers stay (the consumers'
   closures reach the obligation: `CONDITIONAL`, exit 1).
4. **The seal (landing 2; R78).** `run_admitting` leaves the public
   view. In its place K exports a checked transfer, `run_transfer src
   dst names`: the constants `names` of `src`'s environment (the run
   that checked them) enter `dst`'s, each required present in `src`,
   identical to any constant of that name `dst` already holds (kind,
   level parameters, type and value by `expr_eq`), every constant its
   type and value cite present in `dst` under the same identity (the
   identity-closure walk of `pin_if_matches`' shape, fuel-bounded: out
   of fuel is a refusal, never an admission) — else the transfer
   refuses as a whole and `dst` is unchanged; `od_admit` is its one
   caller. `empty_run`'s pins flag leaves the view: the pin table's
   generator (`t0.shard` for `gen_pins.sh`) uses a generator run type
   whose only product is the candidates' identity closures as the
   table's rows, never a `CheckedEnv`; `check_into` pins by
   `pin_if_matches` alone. Every `CheckedEnv` a client can hold is
   then one K checked or transferred under identity. Pins under
   `k_clients_test` (clients through the V3 loader): a client naming
   `run_admitting` (refused: no such service); a transfer of a
   declaration whose destination holds a different constant of that
   name (refused); a transfer whose citation is absent from the
   destination (refused); the generator's run yielding no environment
   (a client asking one is ill-typed).
5. **Gates.** Landing 0 documents only. Landings 1 and 2 each on the
   full gate — the pins' entrypoint, the suite with `store_test` and
   `k_clients_test` carrying the new pins, parity after each kernel
   edit, CI's pipeline — and recorded.

*Landings.* **Landing 0**: the documents (rule 1). **Landing 1**: the
release gate (rules 2–3): `verify_release.shard`, `store.shard`,
`load.shard --store`, `store_test.sh`, the format's version. **Landing
2**: the seal (rule 4): `k/import.shard`, `k/add.shard`,
`k/mod.req.shard`, `initload.shard`, `t0.shard`, `gen_pins.sh`, the
pins through `k_clients_test`.

*Not in this slice, stated:* R82's three identities (phase 4, its own
slice before T6); R85's origin map (before the incremental goal-state
slice); slice 4.1 itself, amended here and built after.

**Slice 4.0 as built, landing 1 (2026-10-14) — the release gate.**
Rules 2 and 3.

- **The exit (rule 2, R80).** `verify_release` counts, beside K's five
  verdict classes, malformed, mistied, missing, taken, unlisted,
  miskind, cardinality, undeclared and conditional; the exit is 0
  exactly when every one is zero, else 1 — the sum of the failed
  classes, never `n − accepted`; the `RELEASE:` line carries all
  fourteen counts. A file yielding other than one accepted declaration
  under its manifest line is `CARDINALITY`; a second record K accepts
  under another name is `MISTIED` against the manifest, which now
  fails the gate where the subtraction cancelled it (the memo's second
  case lands there, not at cardinality: K accepts the copy, the
  manifest does not know it); an inductive whose metadata mismatches
  after its acceptance fails through the mismatched count.
- **The target (rule 3, R79).** `store.shard` writes `DIR/manifest`
  beside `order` (`store_manifest`: the header `manifest 1`, a `decl ID
  KIND NAME` line per declaration listed, a `param NAME` line per
  parameter the store holds as an axiom, a `discharge OBL PROOF` line
  per obligation the load settled as proved — appended, never
  rewritten; `load.shard --store` supplies the parameters as every
  `PARAM` record not discharged and the discharges as every
  `DISCHARGE … proved` record with the proof `OBL.proof`). The
  verifier reads the manifest first (a store without one is exit 2:
  the format's one incompatible change), holds each file to its
  manifest line's name, fingerprint and kind (`MISKIND`), an order
  line the manifest lacks is `UNLISTED` and its file still checked, a
  manifest declaration without an order line is `MISSING`.
- **The policy (rule 3).** After the store: each manifest discharge is
  verified — the obligation an accepted axiom, the proof the accepted
  theorem `OBL.proof` whose type is the obligation's by `expr_eq`
  (`DISCHARGE`, else `BAD-DISCHARGE`, counted conditional); every
  accepted axiom is a declared parameter or a verified discharge's
  obligation, else `UNDECLARED`; every other accepted declaration's
  assumption closure (`axioms_of`, K's walk) is resolved — the export's
  axioms (the Init closure's environment holds them) and the declared
  parameters allowed, a verified discharge replaced by its proof's own
  closure, fuel-bounded with a cycle or exhausted fuel left bad — and
  a remainder is `CONDITIONAL NAME: rests on AX…`; `PARAM` lines name
  every declared parameter. Found in the build: an obligation's axiom
  was first flagged `UNDECLARED` beside its own `DISCHARGE` line (the
  std root's `renderOnto.dec_1`, calc's four, the pin's three) — the
  policy now exempts a verified discharge's obligation.
- **The pins** (`store_test.sh`, 29 checks): the memo's seven — a
  second rejected record in a file and a second accepted one under
  another name (R80), calc's inductive with `isRec` flipped (R80), the
  theorem replaced by a same-name same-type axiom (`MISKIND`), a proof
  rewritten to rest on an axiom whose file is added unlisted
  (`UNLISTED`, `UNDECLARED`, `CONDITIONAL le_step: rests on
  le_step_ax`), a target removed from `order` with its file
  (`MISSING`), and `discharge_measure` stored with three `discharge`
  lines and no parameter, verifying with three `DISCHARGE` lines, then
  its `count.dec_1` discharge removed (`CONDITIONAL count` and
  `count_self`, `UNDECLARED count.dec_1`) — each a nonzero exit; the
  manifest's header and 23 `decl` lines checked on the example.
- **Gates run.** `store_test` 29 checks, 0 failed, 640 s;
  `std_test` 201, 0 failed; `path_test` 18, 0 failed;
  `tactic_lib_test` 30, 0 failed; the full suite's 41 entrypoints, 0
  failed, in 820 s (parity byte-identical over 31 closures and 136,792
  declarations); the three edited kernel files clean under the V3
  loader; names at the baseline. Route 1's kernel build, the full
  replay and the corpus are CI's.

**Slice 4.0 as built, landing 2 (2026-10-14) — the seal.** Rule 4.

- **The transfer.** `k/add.shard` `env_transfer src dst names`: each
  name present in `src`; identical to any constant of that name `dst`
  holds (`ci_identical`: the kind, the level parameters, the type and
  the value by `expr_eq`, an inductive's, a constructor's and a
  recursor's shape — hints are not identity); every constant its type,
  value or rules cite present in `dst` under the same identity as in
  `src` or itself among the names (an inductive's constructors and
  recursor travel with it; `ci_cited` through `expr_consts`); any
  failure refuses the whole and `dst` is unchanged; then the watermark
  raised to both environments' and the names pinned by identity
  (`pin_names`, after the whole transfer). Exported by the view;
  `initload.shard`'s `od_admit` is its caller, over the constants the
  load fed (`OdLoaded`'s list, in feed order), no longer over the
  lines.
- **The deletions.** `run_admitting`, `admit_decl_pinning` and
  `admit_decl_core`, `pin_names_unconditionally`, the run's admit and
  pins flags (`Run` is ten fields), the view's `run_admitting`;
  `empty_run` takes the `-a` flag alone; `check_into` checks and pins
  by identity only. Departure from the design: no generator run type
  — the pin table's generator (`t0.shard --pins`, `gen_pins.sh`) needs
  no privileged mode, since `refgen.shard` prints every candidate
  present in the environment and reads no pin flag; the flag that
  "pinned every candidate as admitted" was unnecessary and is gone.
- **The pins.** `k_client_admit` (265 cases): a client of the view
  naming `run_admitting` is refused `unknown_head`. `k_client_test`'s
  seven cases through the V3 loader: Nat's block and `z` transferred
  into the empty environment hold both and pin the export's `Nat` by
  identity; `Nat` without its constructors refused (the recursor's
  citations absent); `z` alone refused (`Nat` absent); `z` onto an
  environment holding another `z` refused; `z` onto the same `z`
  accepted; a name the source lacks refused.
- **Found in the build.** Pinning each constant as it entered left
  `String` unpinned in every load (pin `ev_string` failed
  `string_literal_unpinned_String`): an inductive's identity closure
  holds its constructors, which enter after it — the names are pinned
  once the whole transfer is in. A pattern variable spelled `n2`
  shadowed `util.n2` (`pattern_name`); renamed. The old lines-based
  path re-decoded the records through the Init run's tables, which
  carried the run's node ids into the loader's environment without
  raising its watermark; the transfer raises it to both.
- **Gates run.** The loader pins' entrypoint 265 cases, 0 failed, 53 s;
  `k_clients_test` 7 tests, 0 failed, 110 s; the full suite's 41
  entrypoints, 0 failed, in 1,381 s (parity byte-identical over 31
  closures and 136,972 declarations); K and the loader clean under the V3
  loader with K's directory as a second root; names at the baseline.
  Route 1's kernel build, the full replay and the corpus are CI's.

**Slice 4.0 closed 2026-10-14** at this landing: the memo's eight items
answered as records §4.12 disposes them — R78–R80 fixed with pins,
R81–R83 and R85 in the documents, R84 in slice 4.1's design; the
ledger's release row amended at the close-out of this landing.

**Slice 4.1 — the native toolchain: route 1 over the kernel's drivers**
(designed 2026-10-13 at phase 3's close, on the user's "Let's find a
near-term slot in the schedule to implement the compiler path for the
v3 kernel, similar to how we did it for v2 … or should we just do it
next?"; §13 item 65 — **ratified 2026-10-14 as amended by R84 (the
user: "Good. Call it ratified and continue.")**; law §9.1 route 1,
§12.4 item 4 by the boundary's ruling, §12.1's "generated files never
hand-patched"). The v2 chain — `tools/lower` to its low dialect,
`tools/codegen` to C, `cc` — compiled v2's kernel into
`bin/shard_eval` and `bin/shard_check` from 2026-07, gen-N building
gen-N+1 with the Rust interpreter as the bootstrap authority and a
byte-tie on a fixture before every fast build (`bin/rebuild.sh`).
Route 1 runs that chain over `v3/kernel/t0.shard` and
`v3/kernel/initindex.shard` today (`v3/build.sh`; CI builds both; the
suite's `init_index.sh` runs the index native). Everything else an
author or the suite runs — the loader, `prove`, `verify_release`,
the 23 harnesses — runs on the Rust bootstrap, route 3. Measured
2026-10-13 at the close, on scratch copies, the lowerer booted by
`bin/shard_eval`:

| what | measurement |
|---|---|
| route 1 today: `t0` + `initindex` closures | 29 files, 13,497 lines; `t0` lowers in 81 s locally |
| the loader's closure (`load.shard`) | 55 files, 32,644 lines; `prove` 53 files, 31,950 — 29 files, 19,450 lines beyond route 1 |
| the lowerer on `load.shard` | refused at source 30 of 55 (`matcher.shard`): v2's reader reads a capitalised local (`M`, the motive) as a constructor and reports the fn malformed; sources 1–29 accepted unchanged; with that one local renamed the whole matcher closure lowers in 50 s |
| capitalised locals in the kernel | 26 binders, 6 names, 4 files (`reader` 19, `matcher` 4, `predef` 2, `tactic` 1); type parameters `(T Type)` are not among them, v2's reader has that rule since slice 3.3 |
| records in the loader's closure | 7 declarations, 73 accessor names, 48 `with`, 11 `make` in 7 files — item 42's stated cost: v2's reader expands a record under v2's accessor names |
| the two dialects otherwise | identical: the two fn shapes (4-part; 5-part with a measure clause, 843 fns outside the route-1 closure against 222 inside), `match`/`let`/`list`/`if`/`quote`; every host extern `host.shard` declares already in `tools/codegen/rt.h` (`read_range` included) |
| the bootstrap's fixed cost | 369 ms to parse the loader's closure; a load's 16–40 s is interpreted work, all of it in scope for the native route |
| the native gain, measured | the index: 952 s on the bootstrap, 68 s native (14×); the replay: 4,591 s against 2,489 s (1.9×, memory-bound on 32 GB of tables) |
| the suite's entrypoints | 41: 23 `.shard` harnesses run as `eval direct`, 18 `.sh` scripts invoking `load.shard` (24 sites), `verify_release` (6), `initindex` (6), `prove` (3), `calc_harness` (2); 1,439 s on the runner, 536 s on the quiet dev box |

The rules:

1. **The pre-pass is the V3 loader's projection, not the bootstrap's
   flat read and not an edit of v2's reader.** `load.shard --emit DIR
   FILE` loads the driver's closure as `--dump` does for parity (§10
   item 1: every toolchain closure is already loaded by the V3 loader
   and projected to one canonical text, the projection's injectivity
   checked per closure) and prints each module of the closure as one
   file of the dialect v2's reader reads: the types, the externs, the
   fns with their bodies — a sequential `let`, `match` over
   constructor patterns, `if`, `list`, `quote`, string literals as
   the byte-list constructor chains both loaders build — under the
   closure's short names (the parity projection's, injective by its
   check), records already expanded by the loader (the constructor
   type, an accessor and an updater per field as plain fns, `make`
   and `with` rewritten — item 42's cost dissolved without a change
   to the language), the dotted heads mangled to one identifier,
   every local printed under a lowercase name (the Prog's binder name
   where it has one, generated otherwise — the capitalised-local
   difference dissolved the same way), the measure clause dropped
   (the lowerer carries the body only; totality is K's and the V3
   loader's, never the chain's), `use` lines dropped (the emitted
   closure is flat), the imports as relative paths. The emitted
   files are derived and untracked (`v3/bin/emit/`, as `*.low.shard*`
   are). Law §9.2's "the Rust loader reads them exactly as it reads
   `kernel/*.shard`" stays true of route 3; route 1 is now built from
   what the V3 loader accepted — item 49's one front end extended to
   the native route. The emitted text is a derivation, never an
   origin: the emitter keeps the map from each emitted form to its
   source occurrence (R85), so a diagnostic from a native run names the
   source, not the emitted file.
2. **The build.** `v3/build.sh SRC OUT` gains the emit step before the
   lowerer (emit → lower → codegen → cc) and a list form,
   `v3/build.sh --all`, building the five drivers (`load`, `prove`,
   `verify_release`, `t0`, `initindex`), `calc_harness` and the 23
   harnesses into `v3/bin/NAME`; the boot engine as today
   (`bin/shard_eval` when present, else the Rust interpreter; CI
   caches the former). `t0` and `initindex` move onto the same path,
   so route 1 has one build and the pre-pass is exercised by the
   replay's byte-tie from the first landing.
3. **The byte-tie gate.** `v3/kernel/test/native_tie_test.sh`, the
   42nd entrypoint, runs a fixed set of the suite's workloads under
   both engines in paired scratch roots of the same name and compares
   them — the loader over the pins' cases and the examples, `prove`
   over `examples/auto` and `examples/tactic`, `verify_release` over
   `v3/release/`, one harness — on four things: stdout, stderr, the
   exit status and every artifact written (a sidecar, a store's files,
   a dump), byte for byte, each engine required to exit with the same
   status and print its verdict line before the comparison (GPT-6 R54,
   `t0_full.sh`'s shape). The normalization is a list, stated in the
   script: the runner's seconds, a store's size line, a scratch root's
   generated name; never a verdict, an assumption closure, a
   fingerprint or a record. A divergence is a miscompile or a semantic
   gap between the bootstrap and the chain, and it fails the gate: the
   binary is never patched, the emitter or the chain is fixed and the
   build rerun. The gate's own discriminating cases (R84, records
   §4.12): a record built and updated, a capitalised local colliding
   with a constructor's name, nested shadowing, a quoted capitalised
   symbol, two dotted names sharing a last component, a sealed client,
   an accepted and a rejected K admission, a malformed input, an
   exhaustion, a nonzero program exit, a sidecar written, a store
   verified; and the gate itself tested — a native verdict altered by
   hand and a binary removed are each detected.
4. **The suite on the binaries, the bootstrap the authority.** An
   engine shim `v3/bin/eval` (built by rule 2) answers `direct FILE
   ARGS…` with `v3/bin/NAME` when that binary exists and its stamp is
   current (rule 8), and fails loudly otherwise — never a silent
   fallback to the bootstrap, so a missing binary cannot satisfy native
   coverage (R84); it passes `dump` and `run` to the bootstrap
   unchanged, the two operations that stay the bootstrap's (parity's
   second reader; the old chain's boot); `v3/test.sh` runs on the shim
   when `V3_ENGINE=native` and on the bootstrap otherwise. CI runs both: the bootstrap suite as today
   (route 3, the authority) and the native suite beside it, their
   per-entrypoint outputs compared as rule 3 compares — two
   independent runs of 41 entrypoints agreeing byte for byte is the
   full tie; the native run's time is the measurement. Locally the
   native run is the default once built; the bootstrap run is the
   authority a landing records before its commit, as the full suite is
   today. Parity (§10 item 1) keeps the bootstrap as its second
   reader; `eval dump` is not compiled.
5. **The old tree untouched.** No edit to `tools/lower`,
   `tools/codegen`, `kernel/reader.shard` or `rt.h`: the emitter
   speaks the dialect those read, found by lowering until the chain
   accepts the closure (each probe under 90 s), and a form the chain
   cannot take is answered in the emitter. If a blocker needs the old
   tree changed — a runtime limit, a missing extern, a reader rule —
   it is a ruling, since the corpus gate fires on every old-tree edit
   and the chain's determinism is what the byte-tie rests on.
6. **The binaries are engines, not proven artifacts.** Route 1's
   status in the law is unchanged: "its proof is still to come"
   (`v3/build.sh`'s header; law §9.1 route 1); the compiler proof is a
   later slice of this phase, over this chain or V3's own. **Amended
   2026-10-16 (§13 item 69): the proof is phase 7's, over V3's own
   chain with the per-function generator, and the chain's replacement
   is phase 7's with it, bootstrapped through slice 4.4's bridge; this
   chain, behind the emitter, is route 1's engine through the flip.** Nothing a
   binary computes is admitted on its own word: K's verdicts, the
   store's records and the release's verification stand on route 3
   until that proof, and every pipeline's bootstrap run says so.
7. **Measured and recorded.** At the close-out: per driver, the
   binary's time against the bootstrap's on the tie's workloads; the
   pins' entrypoint; `store_test`'s calc leaves; the full native suite
   against the bootstrap's; one author's load of a single file. The
   numbers go to records §9 and the slice table.

8. **The roots and the stamps** (R84). The root inventory is written
   in full in `v3/build.sh --all`: the five drivers (`load`, `prove`,
   `verify_release`, `t0`, `initindex`), `calc_harness`, the 18 public
   harnesses of `kernel/test/` and the 5 of `k/test/` — 29 roots,
   parity's list; and every shell entrypoint's invocation has its
   disposition in the script — `direct load.shard … --run M.FN` native
   (the K-client harness), `--dump` and `eval dump` the bootstrap's.
   Each binary carries a stamp: the hash of its closure's sources, of
   the emitter, of `tools/lower`, `tools/codegen` and `rt.h`, and of
   the compiler's flags; the shim refuses a stale stamp. Generated files
   live under `v3/bin/` only; an authored file is never written by the
   build, and the tie checks the tree's authored files byte-identical
   before and after.

*Landings.* **Landing 1**: the emitter (`load.shard --emit`), the
build (rule 2, `--all`), the tie entrypoint (rule 3) over the pins
and the examples with its discriminating cases, the drivers and
harnesses built under rule 8's inventory and stamps, the emitter's
pins (a record, a capitalised local, a dotted head, a measure clause
— each emitted and lowered); the tie green locally and on CI. **Landing
2**: the shim and the suite switch (rule 4), CI's second run and its
comparison, the measurements (rule 7), the close-out.

**As built, landing 1 (2026-10-14):** the emitter `kernel/emit.shard`
and `load.shard --emit DIR FILE…`: after a clean load, one file per
module of the closure under DIR in the load's order, the root's file
importing every other — the old reader's dependency order is the load's,
so a constructor is declared before any module matches on it, and a
consumer never matches on a sig type's constructors, so K's
implementation linked after its consumers is safe behind the root's
imports —; the names the parity projection's (`dm_head`, `dm_short`), a
dotted head mangled to one identifier (`Load.env` → `Load_env`; a
collision the mangling makes refused) and a head the old reader reserves
given a trailing underscore (`chain` → `chain_`, the one such name in the
kernel; the set is its sugar and special forms); every local printed by
its de Bruijn depth, `v0 v1 …` — the Prog carries no binder name, so the
design's "the Prog's binder name where it has one" was moot and the
capitalised-local difference is gone by construction; a many-binding
`let` nested one binding each, so the sequential rule (R44) holds under
either reader; a string literal re-escaped (`\" \\ \n \t \r`, any other
byte raw); a symbol literal the old reader cannot lex as one atom (`t'`
in `define.shard` — its quote macro) built by `sym_of_chars` over its
text; the primitives under the chain's spellings (`int_eq lt le`); the
measure clause dropped. Refused with nothing written: a sig fn or sig
type left unlinked (the implementation's directory not given as a root),
`absurd`, a primitive beyond the profile's eighteen, a string literal
over any list but one spelled `Cons`/`Nil` (what the old reader builds),
a string or unlexable-symbol pattern, a mangling collision. `DIR/map` is
the origin map (R85) at declaration granularity — each emitted name
against its kind, its identity and its module's source file — and
`DIR/sources` each module's file and hash; the emitted file names its
source in its first line. `v3/build.sh` rewritten (rules 2 and 8): emit
→ lower → codegen → cc; `--all` over the inventory written in the
script — the 29 roots, parity's list, K's directory added as a second
root where the bootstrap's dump of the root shows `env_empty` (parity's
rule) — `V3_BUILD_JOBS` at a time, a current binary kept; `--stamp
NAME`, `--list`; the stamp sha256 over the root's closure sources (the
emit's list), the emitter's closure (`kernel/*.shard`, `k/*.shard`, the
view), the chain's three files and the compiler's flags, beside the
binary; every shell entrypoint's invocation of an engine has its
disposition in the script's header. The tie
`kernel/test/native_tie_test.sh` (rule 3): the gate's own test first
(an altered verdict, a differing exit, a differing artifact, a missing
binary — each detected), every binary of the inventory present with a
current stamp, the discriminating cases built fresh and tied, the
loader over four pins, a dump, an example, t0 run under `ev` with and
without fuel and a store written, `prove` over two examples' sidecars
regenerated and a measure run, `verify_release` over the tie's store, a
store without a manifest and the usage line, `t0` over the export's
first 3,000 lines with `-v -a -p`, the wrong-pin fixture and the usage
line, `initindex` building and verifying an index, one harness
(`loader_pins_test`, every loader pin), the tree's authored files
byte-identical before and after; stdout, stderr, the exit status and
the artifacts compared, a verdict line required on both sides. The
normalization list, stated in the script, is empty: no workload prints
a time or a size and both engines use the same scratch paths. The
discriminating cases are `v3/pins/emit/` (its README): nine
self-contained programs tied — a record built and updated, capitalised
locals beside constructors of their spelling, nested shadowing, quoted
symbols (`t'` among them), two records' accessors sharing a last
component, a sealed client (a directory module's view, the
implementation linked), a nonzero exit after output, string escapes, the
profile's primitives at their edges (negative operands of `mod`, `tmod`
and `ediv`, the shifts, a product near 2^62 — the chain's runtime
against the bootstrap's) — and two refusals expected (`refused_symbol_pattern`,
`refused_mangle_collision`), each case's first line the emitter's
message. The entrypoint runs under `V3_NATIVE=1` in `v3/test.sh` (the
42nd; without it the suite's last line says the tie was not run — a
stated omission, never a fallback) and in CI's new `native` job beside
`v3` (the export, `--all` with 16 jobs, the index, the tie), so the
bootstrap suite's job keeps its time. *Departures, stated:* (1) the
emitted files live per root under `v3/bin/emit/NAME/` rather than one
shared tree, so parallel builds never race on a module two roots share;
(2) the tie runs in its own CI job rather than inside the `v3` job's
suite — the build is 29 closures through the chain and the bootstrap
suite is the authority whose time the job records; (3) the pins'
programs live at `CASE/kernel/main.shard`: the toolchain profile (the
bare comparison spellings, the prelude's string) is a property of the
module path's first component `kernel` (`scope.shard` `toolchain_scope`),
so a self-contained program in that dialect is a module under `kernel`
— a limit on where such a program may live, carried as is; (4) `t0` and
`initindex` moved onto the emitted path (rule 2) — the replay's byte-tie
exercises the emitter from this landing. *Found in the build:* the
reserved head `chain` (the old reader's statement-literal sugar;
record.shard's `chain` lowered as a malformed form until the suffix); the
quote-macro symbol `t'` (the closure unreadable at `define.shard` until
`sym_of_chars`); the import order (a first cut imported the modules
reversed and the old reader refused `JErr` as an unknown constructor in
`index.shard`, read before `json.shard` — the dependency order is the
load's, kept); the K-root detection under `pipefail` (`grep -q` closing
the dump's pipe early failed the test, so no consumer got the directory
until the grep read to the end). *Measured on the dev box, under a load
of 30–40 from other sessions' jobs:* `v3/build.sh --all` over the 29 roots in 4,167 s with six roots in flight and at most two — then, for the box's memory beside other sessions' jobs, one — heavy steps at once, and the two interrupted roots in a second pass of 695 s; on the quiet box the loader's closure (49 modules, 8,438 declarations) emits in 22 s, lowers in 73 s, generates in 236 s and compiles in 63 s, the K harnesses (5,560 declarations) in 10, 45, 46 and 16 s, a harness of a few hundred declarations in seconds; the lowerer holds 12 GB and the generator up to 40 GB on a driver-sized closure, so the build caps the heavy steps (`V3_HEAVY_JOBS`: two on the dev box, sixteen on the runner's 2.7 TB); the tie's 68 checks, 0 failed, in 368 s alone (349 s inside the suite); the full suite's 42 entrypoints under `V3_NATIVE=1` in 680 s on the quiet box — 41 green, parity byte-identical over 31 closures and 137,040 declarations in 329 s (the 68 more the emitter and the driver's flag), and the tie failing its store step through its own scaffolding (the store's directory removed between the engines' runs, the native side left nowhere to write); the scaffolding fixed, the tie rerun green alone; the native
gains on the tie's workloads — `verify_release` over calc's release
(467 declarations) 98 s on the bootstrap, 13 s native; `prove`
regenerating `auto`'s sidecar 47 → 7; `prove --measure` over
`calc_ndigit` 121 → 36; the loader's dump of `auto` 43 → 6; `calc_spec`'s
load 61 → 10; t0 over 300 lines under `ev` through the loader 17 → 10 —
the bootstrap's parse of the loader's closure (0.4 s) is not what the
native route saves; the interpreted work is. The chain's cost is the old
tree's: the lowerer and the code generator on `bin/shard_eval`, the
latter superlinear in the closure (t0's 5,697 declarations in 164 s,
the loader's 8,438 in 653 s), so the inventory is built in parallel and
a binary whose stamp is current is never rebuilt. Landing 2 is the shim
and the suite switch (rule 4), CI's second run compared, the
measurements (rule 7), the close-out.

**As built, landing 2 (2026-10-14):** the shim `v3/bin/eval`, a generated file `v3/build.sh --all` writes last (and `--shim` alone): `direct FILE ARGS…` runs `v3/bin/NAME` (NAME the file's stem) when that binary exists and its stamp is current — `v3/build.sh --stamp NAME` against `NAME.stamp`, on every invocation — and otherwise exits 70 with a message (never a silent fallback to the bootstrap, R84: a missing binary and a stale stamp are each refused); `dump` and `run` go to the bootstrap unchanged; any other operation is refused with the usage line. `v3/test.sh` takes `V3_ENGINE`: `bootstrap` (the default; the authority) or `native` (the shim, required present), names the engine in its last line, and keeps every entrypoint's output, exit status and seconds under `V3_TEST_OUT=DIR` (`NAME.out`, `.rc`, `.secs`; `engine`, `wall`). `kernel/test/suite_tie.sh A B` compares two kept runs entrypoint by entrypoint — the same entrypoints on both sides, the same exit status, the same output under a stated normalization: a count of seconds (`N s`), a store's size (`N kB`), a scratch path's generated name (`mktemp`'s); a verdict, a count of checks or declarations, an assumption closure, a fingerprint, a record never — printing both sides' seconds per entrypoint and summed, exit the entrypoints that differ. CI: the `v3` job keeps its bootstrap run (`v3/bin/suite/bootstrap`), the `native` job runs the tie and then the whole suite on the binaries (`v3/bin/suite/native`, 41 entrypoints as the bootstrap's), and a `suite_tie` job in a `tie` stage after both compares the two — seconds, not minutes. The tie (rule 3) gained the shim's checks (present; a current binary run as the bootstrap runs it; a missing binary and a stale stamp refused with exit 70 — the stale case a private copy of a binary without a stamp of its own, never a real stamp touched) and the suite comparison's own test on a fabricated pair (two runs identical up to their seconds pass; an altered verdict, a differing exit and an entrypoint missing on one side each fail): 76 checks from 68; every `tie` line now carries both sides' seconds — rule 7's per-driver measurement from every run of the gate — and the tie names the bootstrap explicitly (`BOOT`, `RUST_EVAL`), since under `V3_ENGINE=native` the suite's `EVAL` is the shim. *Departures from rule 4, stated:* the shim serves `direct`, `dump` and `run` only — the bootstrap's expression mode (`eval FILE EXPR`), which `calc_test` uses for its old-tree side, is named in that script as the bootstrap's (rule 8's disposition, listed in `v3/build.sh`'s header), not passed through by the shim; CI's comparison is a job of its own over the two jobs' kept outputs rather than a second run inside the `native` job, so the two suites run beside each other and the comparison costs seconds; the suite comparison's normalization list is not empty as the tie's is (the suite's scripts print their seconds, a size and a scratch name), and `suite_tie.sh` states it in full; locally the native run is not the default — `V3_ENGINE=native` is explicit, as the no-silent-fallback rule wants in both directions. *Found in the build:* the native suite's first run failed `calc_test` alone — its 21 old-tree cases through `$EVAL` reached the shim, which printed its usage line for each (the disposition above; the harness side had tied); the tie under the native suite would have tied the shim against the binaries (both sides native) until it named the bootstrap; `v3/build.sh` printed its boot-engine note on stdout before every operation, so `--stamp` would have returned two lines wherever `bin/shard_eval` is absent and the tie's and the shim's stamp checks would have failed there — the note now prints only when building. *Measured on the dev box under a load of ~340 from other sessions' Rust builds (the bootstrap suite's wall 1,130 s against 680 s on the quiet box at landing 1), both suites with the tie as their 42nd entrypoint, six at a time:* the whole suite on the binaries 1,519 s of entrypoint time and 532 s of wall against the bootstrap's 3,925 and 1,130 — and the native run's remainder is largely the bootstrap inside it (the tie 432 s runs both engines; `calc_test`'s old-tree side 152 of its 172 s is the bootstrap's expression mode; parity's `eval dump` is the bootstrap's); per entrypoint, bootstrap → native: `store_test` 890 → 259 s (calc's five leaves stored 411 → 133, K's replay of them 82 → 21), `parity_test` 348 → 165, `path_test` 323 → 94 (load, run, store, verify 25, 25, 37, 12 → 4, 4, 12, 5), `tactic_lib_test` 289 → 53, `define_test` 237 → 43, `calc_test` 235 → 172 (the port under `ev` 71 → 20), `engine_test` 188 → 73, `std_test` 162 → 29 (the std root loaded and stored 89 → 16, K's replay 72 → 12), `wire_test` 153 → 22, `template_test` 109 → 18 (load, run, dump 35, 40, 34 → 6, 6, 6), `derive_test` 105 → 23, `calc_main_test` 104 → 34, `k_clients_test` 77 → 45, the loader pins' entrypoint 57 → 13 (265 cases), `tactic_test` 54 → 14, `rows_test` 49 → 9, `route2_test` 28 → 9, `matcher_test` 25 → 4, `project_test` 9 → 2, `index_test` 8 → 0, `loader_test` 7 → 1, `ev_test` 6 → 4, the rest under 2 s on both; per driver on the tie's workloads (the tie's own lines, bootstrap → native): the loader's dump of `auto` 24 → 4, one author's load of a single file (`calc_spec`, 283 lines of output) 34 → 5, t0 over 300 lines under `ev` 11 → 5 and out of fuel 8 → 5, a store written 32 → 8; `prove` over `auto` 25 → 4, over `use_split` 25 → 4, `--measure` over `calc_ndigit` 65 → 20; `verify_release` over the store 29 → 4; `t0` and `initindex` over 3,000 lines a second on either; the loader pins' harness 55 → 11; the shim's own cost under 0.1 s an invocation (`json_test` through it 0.31 s against 0.30 bare). The gate: the tie 76 checks, 0 failed, in 453 s inside the bootstrap suite and 432 s inside the native one; `suite_tie.sh` over the two kept runs 42 entrypoints, 0 differ, byte-identical under both engines on every entrypoint. **Slice 4.1 closed 2026-10-14** (on the full gate: pipeline 610 green on `2b2cde0` — the `native` job's suite run 41 entrypoints, 0 failed, in 471 s against the bootstrap's 2,180; the `suite_tie` job's first run 41 entrypoints, 0 differ; records §9): the five drivers and the 24 harnesses run natively under the suite and compare byte for byte with the bootstrap's run; the bootstrap suite stays the authority (rule 6); the binaries are engines, their proof the phase's later slice (law §12.4 item 4).

*Not in this slice, stated:* the compiler proof (law §12.4 item 4's
first item; after this slice, over the engine it builds); route 1 over
user files and specialization (§11); `Symbol`'s `String` flip (carried
with "route 1's chain is V3's own", which this slice does not make it:
the chain stays v2's, fed by V3's front end); any edit of the old tree
(rule 5); the migration tool's tier 0 (phase 5's opener).

**Slice 4.2 — the tb_len rung: a per-function certificate in V3**
(designed 2026-10-15 at slice 4.1's close, on the user's "Agreed, let's
proceed with the tb_len rung as described"; §13 item 67 — **ratified
2026-10-15 as described (the user: "That all makes sense -- agreed. Let's proceed with the tb_len rung as described.")**,
the leans beyond the description stated at item 67 for objection; law
§12.4 item 4's first item, §4.9, §12.5's count rule; `v3/MANIFEST.md`'s
row "phase 4's `tb_len` rung inputs"; records §5's provocation). Law
§12.4 item 4 names "a `tb_len`-class compiler proof". The thing is the
coverage arc's Theorem B at one function (`docs/COVERAGE.md`, B-1b):
`tb_len` in `models/imp/probes/tb_micro.shard`, the per-function
certificate of `len` over the call-tier imp machine
(`models/imp/imp.shard`) and the counted-heap runtime's table
(`models/imp/rt.shard`) — for a list `av`, a memory `m` representing a
ghost heap `g` under the heap invariant at roots `r`, a word `a` reading
back as `av` in `g`'s cells, the function table holding `impc`'s product
for `len` at index 4, and fuel of at least 24 + 12·|av|: the call
returns, the memory represents the same `g` under the invariant, and
the result word is the immediate of `len av`. Loads only, the ghost
unchanged — the simplest member of the class; `perim` has the same
shape; `app` allocates. Records §5 names it the measured provocation
for V3 — 717 canonical lines by hand, the ghost twins forced by
equation-only conclusions, the DSL's gotchas — and law §4.9 says what
V3 changes: Theorems A and B are L theorems with P terms and
per-function certificates are I emitted by tactics. This slice restates
`tb_len` as an L theorem in V3 and proves it through I, measured
against the old proof. It is not a proof about v2's chain (ARCHIVE at
the flip, `MANIFEST.md`) and not route 1 proven — every K function
certified by the generator (B-1c), Theorem A, the runtime laws
discharged — which is phase 7's (law §12.4 item 7); this section's
opening sentence names that end state. Measured 2026-10-15 at the
boundary, the closure of `tb_len` across the old tree's files (a script
over the forms' citations):

| what | measurement |
|---|---|
| `tb_len`'s proof | 741 lines in the file (717 canonical); well-founded induction on the length; 80 steps — 45 `have`, 43 `rewrite-with`, 40 `rewrite`, 26 `compute`, 21 `inst` |
| lemmas it cites | 37 distinct: 23 of the heap kit (`tbh_kit`), 7 of the runtime kit (`rth_kit`), 4 of the word kit (`tb_kit`), its own fuel law, the machine's `ilen_nonneg`, std's `int_eq_refl` |
| the lemma closure | 155 claims, 10,594 lines: 93 of the runtime kit (6,775), 30 of the word kit (1,993), 27 of the heap kit (1,082), 2 of `tb_micro` (729), 2 of std/order, 1 of the machine; 13 std requirements (bits 10, nat 2, map 1); 8 axioms of `facts.shard` |
| the definition closure | 133 forms in about 1,000 lines: the machine's call tier (7 types, 2 records, 19 functions — the kinds, the operators, the expressions, the call-tier statements, `IpFn`, `IpProg`, the outcomes, the evaluator), the runtime's spelling kit and table (27), the heap vocabulary (3 records — `HCell`, `HRaw`, `GHeap` — and 49 functions), the word kit (9), the readbacks and the post-predicate (9), `Mem` with four operations over the map, std/nat's `int_of_nat`, std/bits' `pow2` |
| runtime laws cited | none: `len` allocates nothing, so C2b's allocation, increment and release laws are outside the rung |
| the product | `len_ipfn` in `tools/impc/fixtures/micro_ipc_out.shard`, 13 locals, compiled from `micro.shard`'s `len : (List Int) → Int` |
| in `v3/` today | nothing of this: no `v3/models`; `v3/std` has no `map` and no `mem` |
| Stage 1 today | a mutually recursive group is `RUNNABLE` with `mutual_recursion` (slice 3.15 rule 6; §11's row): the evaluator's four functions — `ipstmt`, `ipwhile`, `ipstmts`, `ipcall`, one fuel-countdown group — would have no equations in K |

The rules:

1. **The object.** `tb_len`'s statement as B-1b states it, a `theorem`
   in V3 with a `(by …)` proof: the premises the heap representation,
   the invariant at the roots, no raw cell, the argument's readback,
   the table's entry, the memory bounds and the fuel bound; the
   conclusion the post-predicate at the input ghost and the immediate
   of `len av`. The same product, the same table, the same runtime
   statement lists: nothing of the certified object changes, only the
   language it is stated in.
2. **The port is the closure, by the migration classes (law §10.2),
   under `v3/models/imp/`.** The 133 definitions of the table and
   nothing more — not the base tier, the well-kinded gates, the lifts,
   the length-preservation family, `impc`, the other probes (phase 5's
   by manifest, their statements then). Five files: `imp.shard` (the
   machine's call tier), `rt.shard` (the runtime's spelling kit and
   table), `heap.shard` (the cells, the ghost heap, representation and
   invariant), `readback.shard` (the word kit, the readback relations,
   the post-predicate), `len.shard` (the source `len`, the product, the
   theorem); and under `v3/std`, `map.shard` (the association-list
   map — `empty`, `lookup`, `insert` — its four requirements as
   theorems) and `mem.shard` (`Mem` over the map; `mem_get`, `mem_set`,
   `load_le`, `store_le`), with records in `v3/std/README.md` as
   3.27's. The classes applied: a word, an address, a size, a count and
   an index are `Nat` (the typed class — bit patterns and lengths to
   `Nat`; the modulus 2^64 explicit as today, `hi` and `lo` by `/` and
   `%` at 2^63); a source value stays `Int` (the product's immediates
   are the two's-complement i63 of an `Int`), the one bridge
   `imm : Int → Nat`; a predicate that is a specification — the
   representation, the invariant and its conjuncts, the readback
   relations, the post-predicate, `ldok`, `in63`, `graw_none`,
   `tb_okr` — is a `def` to `Prop` (the typed class: the Bool
   comparison to the proposition; Init's `∈`, `List.Pairwise` and
   `List.Forall` where the old file wrote its own), while the machine —
   the operators, `iexp`, the evaluator, the runtime's statement lists,
   `hfget`, `inth`, `rinc` and the other functions over data — stays
   `fn`; a name Init has is cited by Init's (the naming law). **The
   evaluator re-spelled as one function**: `ipeval fuel task`,
   structural on the fuel, over `IpTask` (a statement, a statement
   list, a loop, a call) with the four functions' arms, so that it is a
   definition with equations in K today; the correspondence —
   `ipcall fuel …` is `ipeval fuel (TCall …)`, the four names
   abbreviations — recorded in the file. The alternative, Stage 1
   defining a mutual group structural on one shared argument by a
   joint `brecOn`, is a Stage-1 item the toolchain's own port will need
   (phase 5) and is not this slice's.
3. **Landing 0 is the definition gate, and the count.** The port
   loaded by the V3 loader as a root (`v3/models/imp`, Init on
   demand): every ported function `DEFINE` with its equations, no
   `RUNNABLE`, no refusal, every `def` accepted, the product loaded,
   the theorem stated with `sorry` (pending). A suite entrypoint
   `v3/kernel/test/imp_test.sh` asserts the summary line and the
   definition lines (as `define_test` does) and prints the load's
   seconds. A gap found in Stage 1 — a shape the K projection does not
   define, a form the port needs — is a pin and a finding, a landing's
   rule if it blocks (as 3.29's path found its three). The count of
   rule 6 is written in the record at this landing, before any proof
   step.
4. **Landing 1 is the proof.** The kit laws `tb_len` needs, as V3
   theorems, written by need: the closure's 155 old claims are the
   ceiling, not the plan — an old extraction or peeling law that
   `cases`, `simp_only` or `arith` absorbs is not written, and the
   count of those is a result. `tb_len` through I: `induction` on the
   list (the old well-founded induction on `ilen`), `simp_only` over
   the machine's and the product's equations, `arith` for the fuel and
   word bounds, `decide` where a leaf is ground, `auto` where the
   engine closes a leaf (the engine measured as a closer, as Theorem
   B's own text says). The theorem accepted by K; the root stored and
   verified by K alone (`store`, `verify_release`, slice 4.0): the
   certificate as retained P verified without the elaborator (law
   §12.5 T8's first item). The entrypoint gains the proof's seconds on
   both engines.
5. **Landing 2 is `app`**, the allocating member: the three runtime
   laws it needs — allocation, increment, release, C2b's — as
   parameters through 4.0's `PARAM` and `DISCHARGE` records,
   discharged at phase 7; the output ghost an existential in the
   conclusion, no ghost twin — the test of whether V3's `Prop`
   dissolves the twin that B-1b's equation-only conclusions forced;
   `tb_app`'s 1,148 lines the comparison. Its count written when
   landing 1's is read. `perim` is not built (the shape of `len`).
6. **The count, agreed before the run (law §12.5).** (a) The
   hand-written I lines of `tb_len` against 717 canonical; (b) the
   theorems written for its closure, and their lines, against 155 and
   10,594; (c) the distinct lemma citations in `tb_len` against 37;
   (d) the root's load and check on route 3 (the bootstrap) and route
   1 (the native loader), and the proof's elaboration within it; (e)
   the Stage-1 gaps found at the definition gate, each a pin; (f)
   refusals before success and interventions while authoring, T9's
   protocol; (g) the forms the port needed that this document lacks,
   each a §11 or `LEAN.md` row; (h) the old laws absorbed (rule 4). The
   lean: fewer than half the lines and a fraction of the theorems;
   whatever the numbers, the record reads them against records §5's
   premise at the close. No pass bar is set; the user may set one
   before landing 1.
7. **Both engines, the bootstrap the authority** (4.1 rule 6): the
   entrypoint runs under the suite on the bootstrap and on the shim,
   compared byte for byte as every entrypoint is.

*Not in this slice, stated:* `impc`'s V3 version and the per-function
generator (B-1c, phase 7); the runtime laws' discharge (C2b; their
statements PORT as parameters at landing 2); Theorem A; the base tier
and the ISA legs; `perim`; T8's reflection node (built as a landing if
landing 1's measure demands it — a `compute` the equations cannot
replace); the String flip and route 1's chain; `std/word`'s `UInt*`
representation (phase 5's word line: words here are `Nat` with the
modulus explicit, recorded); the migration tool's tier 0 (phase 5's
opener; this port by hand, every re-spelling it needed recorded for the
tool's design).

**Slice 4.2 as built, landing 0 (2026-10-15) — the definition gate, and
the count.** Rules 2, 3 and 6, with what the build found.

- **The port.** Seven files, 931 lines: `v3/std/map.shard` (the
  association-list map over `Nat` keys — `map_empty`, `map_lookup`,
  `map_insert`, the names prefixed so `std/list`'s `insert` and this
  one never share a short name — and the old interface's four
  requirements as theorems, by `unfold`, `rw` with `if_pos`/`if_neg`
  and `cases` on `Nat.decEq`), `v3/std/mem.shard` (`Mem` a record over
  the map; `mem_empty`, `mem_get`, `mem_set`; `load_le` and `store_le`
  on a `Nat` measure, their descents discharged by `Nat.sub_one_lt`;
  the framing laws with their consumers), and under `v3/models/imp/`
  `imp.shard` (the call tier: 9 types, 2 records, 18 functions),
  `rt.shard` (the spelling kit and the four runtime functions' statement
  lists, 31 functions), `heap.shard` (3 records, `Ext`, 17 functions
  over data, 23 specifications), `readback.shard` (4 functions, 14
  specifications), `len.shard` (the source `len`, the product
  `len_ipfn` constructor for constructor, the theorem with `sorry`).
  The loader accepts the root in one process: 8 modules, 758
  declarations, 152 `DEFINE` with their equations, 0 `RUNNABLE`, 0
  refused, 1 pending — the theorem — and 0 errors;
  `kernel/test/imp_test.sh` (the 42nd entrypoint; 81 checks) asserts
  it, with the evaluator's nine descents discharged and every
  specification's two equations present, and prints the load's
  seconds: 37 s on the bootstrap, 6 s on the native loader. The kernel
  change rebuilt the inventory: 29 binaries in 2,377 s on the dev box
  (six roots in flight, two heavy steps), the loader's chain emit 24 s,
  lower 80, codegen 280, cc 72 over 8,447 declarations — 9 more than
  landing 4.1's 8,438, the two findings' functions.
- **Two findings in Stage 1, each a rule of this landing with its pin.**
  (1) **A nested inductive had no `casesOn`.** `IpStmt` carries
  `List IpStmt` in `IpIf` and `IpWhile`; `define.shard`'s bundle skipped
  every nested type, so no `match` on a statement had an L meaning and
  the evaluator was `RUNNABLE` with `no_l_identity`. The bundle now
  generates `T.casesOn` for a nested type from the recursor's own
  telescope, as Lean does: a motive per auxiliary type set to
  `λ x. PUnit.{u}`, the auxiliary constructors' minors `λ binders.
  PUnit.unit.{u}`, the type's own minors carrying a `PUnit` hypothesis
  per nested field (`rec_app_dep`, `nested_tail`, `open_ihs_dep`); its
  `noConfusion` and injectivity follow. Structural recursion through the
  nesting stays outside (`rec_usable`'s refusal, `pj_struct`). Pin
  `nested_match`. (2) **A `def` could not recurse.** The reader's
  `def` elaborates its value with no self-local, so a specification over
  a list — `hslots_rep`, `rb_list`, twelve more — was `unknown_constant`
  on its own name, and `def` took no measure clause. A `def` with a
  recursion clause, `(def NAME ((x TYPE)…) TYPE (measure (struct x))
  BODY)`, is now the fn pipeline's K half alone (`loader.shard`
  `load_ldef`): read once as a pre-definition with its self-local,
  defined by K's projection with its equations — `brecOn` for structural
  recursion, `WellFounded.fix` with its obligations for a measure —, its
  matchers admitted, no classifier and no program; `DEFINE` with no
  roots. Its binders are E types, as the pre-definition reads them. Pin
  `def_struct`. Both pins in the harness's list; the loader's own
  sources load clean under the V3 loader; the names at the baseline.
- **The classes applied, every re-spelling recorded** (rule 2; for the
  tool's design): words, addresses, counts, tags, arities and indices
  `Nat`, the modulus explicit — the wrapping subtraction `(a + (M − b %
  M)) % M`; a comparison's value `(if (= a b) 1 0)` (the old `ib2i`
  absorbed); `ilen`, `iapp`, `ikn`, `fllen`, `hlen` Init's `List.length`
  and `List.append`; `izeros (ikn ks)` is `izeros_of ks`, structural on
  the kinds; `iw8` the literal 8; the evaluator one function `ipeval`
  over `IpTask` with `IpRes` and `out_of`/`ret_of`, the four old names
  abbreviations of it, its nine descents `(intro p h …)` over the
  matchers' equations then `Nat.sub_one_lt`; `rt_zero_heads` over a
  literal index list; a record built by its constructor `MkIpFn` applied
  in field order (`make` reaches a record of the file only; another
  module's constructors through `(use M.T)`); the extent pair `Ext`;
  `memb` Init's `List.Mem`, `inth` Init's `List.getD`; `count_in` kept
  (Init's `List.count` needs an instance the table does not resolve);
  `andb` chains `And`, an `if … True …` a disjunction; seven `0 ≤`
  conjuncts the type (`cells_ok` 2, `e_brk`, `hinv`, `raw_ok` 2,
  `ldok`); `nS`, `ntl`, `npred` not ported (Nat arithmetic) and
  `ilen_nonneg` absorbed; `imm` and `dbl` through `Int.toNat` of the
  `Int` modulus; `in63` and the post-predicate's `ok` propositions;
  the specifications `def`s with the recursion clause. The author's
  refusals before success: four (a `Nat.succ` pattern in an E function —
  a `Nat` is matched by value; the record's constructor; the descents'
  binder lists; an unbalanced form).
- **Gates run on the dev box** (load 3–8, the box quiet): the loader
  pins 271 cases, 0 failed; `define_test` 12/0, `derive_test` 14/0,
  `std_test` 201/0 over the root's 10 files and 398 declarations
  (loaded and stored in 94 s, replayed by K alone in 92 s); the full
  bootstrap suite under `V3_NATIVE=1`, 43 entrypoints, 0 failed, in
  800 s of wall with six at a time — the tie 76 checks, 0 failed, in
  350 s, parity byte-identical over 31 closures and 137,121
  declarations (81 more than landing 4.1's 137,040: the two findings'
  functions and their equations), `store_test` 572 s, `imp_test` 37;
  the native suite (`V3_ENGINE=native`) 43 entrypoints, 0 failed, in
  553 s of wall and 1,718 s of entrypoint time against the
  bootstrap's 3,314 (`imp_test` 37 → 7, `store_test` 572 → 184,
  `define_test` 231 → 38, `std_test` 151 → 60, parity 320 → 265);
  `suite_tie.sh` over the two kept runs 43 entrypoints, 0 differ,
  byte-identical under both engines on every entrypoint. The gates are
  the landing's; CI's five jobs are the full gate.

**Slice 4.2 as built, landing 1 (2026-10-15) — the proof of `tb_len`
through I, and the count.** Rules 4, 5 and 6, with what the proof found.

*The proof.* `tb_len` follows from its general form `tb_len_aux` — for
every fuel, word and depth, by induction on the list — at the theorem's
binders. The nil case: the product's three statements run on the locals
`(1 0 …)` by one run lemma, the call returns 1 = `imm 0`, the
post-predicate is assembled at the same ghost. The cons case: the facts
of the cell at `a` — its window from the invariant (`cell_window`:
rb + 152 ≤ a, a + 24 ≤ gend ≤ 2^32, so the three loads are inside the
window and the 32-bit band is the identity), its header word's tag from
`cells_ok` (`hword_tag`), the loads of the header and the two slots
through `hcells_rep` (`rd_hdr`, `rd_slot`) —; then the body's run through
the run lemmas, the inner call at fuel `fuel − 24 + 12` by the induction
hypothesis (its premise 24 + 12·|t| ≤ fuel − 24 + 12 from the theorem's
24 + 12·(|t| + 1) ≤ fuel), its result split: no result, a statement
result or a trap contradict the hypothesis (the post-predicate at those is
`False`); a failed call fails the body and the call, and the predicate
holds; a returned word is `imm (len t)` = 2|t| + 1 with |t| < 2^62 from
`in63`, the product's subtract gives 2|t|, its add 3 + 2|t|, its overflow
bit is 0 (the xor-and-shift below 2^63: `Nat.xor_lt_two_pow`,
`Nat.and_le_left`, `Nat.shiftRight_eq_div_pow`) when |t| + 1 < 2^62 — the
return 3 + 2|t| = 2(|t| + 1) + 1 = `imm (len (h :: t))`, `rinc` at an odd
word the same roots — and 1 when |t| = 2^62 − 1 exactly, the fail. The
call laws take d < dmax and the theorem has no premise on the depth, so
both cases split on it: past the depth the call fails with `FStack` and
the predicate holds. The run lemmas (`len_S1`, `len_S2`, `len_S2_one`,
`len_E3a`, `len_E3b`, `len_T9_fail`, `len_T9_ov`, `len_T9_ret`,
`len_body_ret`, `len_body_fail`, `len_body_one`) are the product's
statements over their locals at a fuel base `n` — the if's branch at
n, the if at n + 1, a statement list one level above its head —, the
heap words `hw w0 w1` variables; the theorem instantiates each at
`fuel − 24 + k` (`hf24 : fuel = (fuel − 24) + 24` by
`Nat.sub_add_cancel`). Every step is a step law of `step.shard`: one per
equation of `ipeval` at `FS f` with the sub-runs' results as hypotheses
(`stmts_nil`, `stmts_cons`, `stmts_cons_fail`, `stmt_set`, `stmt_if_z`,
`stmt_if_nz`, `stmt_loadw`, `stmt_call_ret`, `stmt_call_fail`,
`stmt_fail`, `call_ret`, `call_fail`, `call_depth`), each proven by
`unfold` and `rw` with a matcher at a constructor taken by name
(`unfold ipeval.match_N`: one case step), and the same over a Nat fuel
through `fuel_of_succ`. `reduce` is used only on the machine's pure
functions at concrete locals (`iexp`, `ilset`, `ipexps`, `iband_args`),
never on the evaluator. The mechanical text — the locals lists, the fuel
towers, the lemma instantiations — was spelled by a scratch script at
authoring time, not kept: the file is the proof.

*What the proof found* (Stage-1 gaps and the kernel's facts, each with
what was done):

1. *Whole-run `reduce`* — the unfolded product normalized in one step —
   ran past ten minutes and 11 GB on the cons case and was abandoned;
   the proof is bottom-up by the step laws (`step.shard`'s header).
2. *A measured `fn` on a Nat has one unconditional equation*, so
   `reduce` unfolds every inner call it meets. The evaluator is restated
   over the structural fuel (`Fuel`: `FZ`, `FS`; `fuel_of` the
   conversion, its one descent discharged by `Nat.sub_one_lt`): `ipeval`
   matches the task outermost and the fuel per arm, so K defines it by
   its leaves — seventeen equations, one per task and statement kind, no
   descent obligation —; the old names are abbreviations through
   `fuel_of`; the theorem's statement is unchanged.
3. *The reducer's redex* (`find_redex`, `tactic.shard`): (a) a bare
   constant is no redex — its unfolding is a lambda that beta-reduces
   into the application, which was unfolding `Nat.land a 1` to
   `Nat.bitwise Bool.and a 1`; (b) K's whnf and the delta step fire on a
   closed subterm only — on an open term K's unfolding of an imported
   definition is no computation (`a + 8` to eight successors) and buries
   the term a later rewrite must match; an open occurrence unfolds by
   its equations, by a case step, or by name (`unfold`); (c) the case
   step (`alt_step`) waits at an open, non-constructor discriminant —
   K's whnf there is a search through the definitions the discriminant
   is built from: the tag test over `hword (hfget cs a)` went into
   `hfind`'s accessibility proofs for ten minutes and the box's memory
   (69 GB); with the step waiting, and the word a variable in the lemma,
   the same fact takes 15 s. `tactic_test` 110/0, `tactic_lib_test`
   30/0, `calc_test` byte-identical, `std_test` 201/0, `define_test`
   12/0, `derive_test` 14/0 and the 267 loader pins hold under the
   three (§5 rule 6).
4. *K's heartbeat budget.* The whole proof as one theorem ran through I
   in 365 s on the binary and K refused it `exhausted` (10^8 ticks per
   declaration, add.shard's `default_limits`; the loader said only
   `exhausted`). The loader's refusal now names the budget and K's site
   (`exhausted_heartbeats_at_SITE`); the proof is split into the run
   lemmas, each a K check of its own — the shape rule 5 asked for
   anyway. K is untouched.
5. *A pending theorem's record carries its open goals in a symbol*
   (`sorry goals=N [GOAL]…`, the `by_sorry` pin's form); the native
   runtime's symbols hold 4 KB (`tools/codegen/rt.h`, `sym_of_chars`),
   the bootstrap's any length — a `sorry` deep in the case split trapped
   the binary where the bootstrap ran, an engine divergence. The text is
   bounded at 2,000 characters with an ellipsis (`goals_bounded`,
   `reader.shard`'s two sites); the pin is unchanged.
6. *A constructor not in scope reads as a variable pattern* (found while
   writing item 4's message with `kernel.verdict`'s type in scope and
   not its constructors): the arms after it unreachable, the function
   accepted by the bootstrap with the first arm's meaning, and the C of
   the dead arms uncompilable on route 1 (`redefinition of v_6`) — the
   loud failure only on the native side. Fixed by the `use`; the refusal
   of an unreachable arm is issue #42.
7. *`cases` on a term* generalizes the goal and not the earlier
   hypotheses: a fact about the scrutinee is stated over an arbitrary
   result with its equation (`∀ R, EV = R → tbh (ret_of R) …`) and
   instantiated in each arm; `cases` on a variable substitutes
   everywhere, the arm's equation then already closed. And the tactic
   facts the proof leaned on: a hypothesis is never rewritten — restate
   it by `have … (exact h)` or rewrite in the goal before `intro`; a
   conjunction splits into rows by `And.left`/`And.right` `have`s;
   `arith` has no parity, no `mod` and no Nat subtraction — the parity
   facts go through `Nat.and_one_is_mod` and the hypothesis, the
   wrapped subtract through `Nat.add_mod_right` and
   `Nat.mod_eq_of_lt`; a negative numeral needs an `Int` context; a
   theorem binder named like a definition (`hinv`) shadows it in later
   propositions (renamed `hinvr`).

*The count* (rule 6; the record's numbers before any proof step: 741
lines of proof in the old file, 80 steps, 37 distinct lemmas cited, 155
claims in the lemma closure, 93 of them the runtime kit's). The proof
text: 337 lines — the run lemmas 189, `tb_len_aux` 133, `tb_len` 15 —
against the old 717; 268 step forms in the general form and 353 in the
lemmas against the old 80 (the old proof's steps were whole-run
normalizations; these are one statement each). The kit written by
need: 55 theorems — the step laws 27 (`step.shard`, 159 lines), the cell
kit 12 (`cell.shard`, 184), the word kit 4 (`word.shard`, 27), the run
lemmas and `len_ofNat` 12 — against the 155-claim ceiling; no runtime
law, `len` allocating nothing. Cited by the proof: 34 of the kit and 24
of Init's lemmas, against the old 37. Refusals on the way: K's
`exhausted` (item 4), nothing else; hand-lowering: none; the kernel's
interventions: the reducer's three rules, the loader's refusal text,
the reader's bound — no K change, no new syntax, no new form. The root
loads and checks — Init on demand, 11 modules, 838 declarations, `tb_len`
on Init's three axioms — in 276 s on the bootstrap and 48 s on the
binary (landing 0's definition gate was 37 and 7 on the quiet dev box),
the records identical under both engines; `imp_test.sh` asserts it, the
root stored and verified by K alone (rule 4). Gates on the quiet dev box: the loader's sources clean under the V3 loader, names at the baseline; the loader pins 267/0; `tactic_test` 110/0, `tactic_lib_test` 30/0, `calc_test` byte-identical over 21 inputs, `std_test` 201/0, `define_test` 12/0, `derive_test` 14/0 on the bootstrap under the reducer's rules; `v3/build.sh --all` 29 roots, 28 built, 1 current, 0 failed, in 2,188 s (two heavy jobs; the loader's chain emit 22 s, lower 69, codegen 221, cc 56 over 8,452 declarations); the bootstrap suite under `V3_NATIVE=1` 43 entrypoints, 0 failed, in 712 s of wall and 3,418 s of entrypoint time (`imp_test` 419 s: the root loaded and stored in 319 s, K's replay of the store 100 s, 143 checks; the tie 76 checks, 0 failed, in 347 s; parity byte-identical over 31 closures and 137,172 declarations in 352 s — the 51 more the landing's kernel growth; `store_test` 573, `tactic_lib_test` 286, `define_test` 239, `path_test` 235, `calc_test` 231); the native suite 43 entrypoints, 0 failed, in 411 s of wall (`imp_test` 89 s: the root loaded and stored in 69 s, K's replay 19 s; the tie 347 s; `calc_test` 193, parity 170, `store_test` 98); `suite_tie.sh` over the two kept runs 43 entrypoints, 0 differ, byte-identical under both engines on every entrypoint. CI's five jobs are the full gate.

**Slice 4.2 as built, landing 2 (2026-10-16) — `app`, the allocating
member: the runtime's laws as parameters, the output ghost existential,
and the count.** Rule 5, with what the proof found.

*The object.* `tb_app` (`v3/models/imp/app.shard`): for lists `av` and
`bv` read back at the words `a` and `b` in a memory representing the
ghost `g` under the invariant at the roots `r`, no raw cell, the
table's entry 5 the product `app_ipfn` (the old compiler's output,
constructor for constructor: the match skeleton, the recursive call at
5, the allocation at 1, the increment at 2, the two slot stores), the
window bounds and fuel at least 25 + 12·|av| — there exist a ghost `g2`
and a word `w` such that the call satisfies the post-predicate at `g2`
and `w` with the roots `r`, and `w` reads back as `app av bv` in `g2`'s
cells. No ghost twin: the old `app_tw` with `pg`, `pw` and
`app_tw_cons` (four forms) is dissolved into the existential, as rule 5
asked. The fuel is this evaluator's accounting (the old statement's 30
+ 12·|av| was the old evaluator's).

*The parameters* (`v3/models/imp/rtlaw.shard`, six `axiom` forms under
`trusts`). Rule 5 names "the three runtime laws — allocation,
increment, release"; read against C2b's own slicing
(`docs/records/COVERAGE.md`: C2b-3 "rt_alloc + fill/seal/read", C2b-4
rt_inc), the allocation law is the raw-cell protocol — the call, the
fill, the seal — and the release law is not needed, since `app` never
calls `dec`: two of C2b's three, as six statements. The restatement
rule, recorded for the tool: a run law whose old conclusion named a
memory the runtime kit constructs (`alloc_out`'s `m_alloc`, `m_inc`) is
stated with that memory existential and its representation and
invariant laws folded in — `rt_alloc_run` (the call fails for want of
memory, or returns the address of an empty raw cell of the tag and
arity over a memory representing a ghost — existential too, its cells,
end and raw cell stated — under the invariant at the same roots;
`rth_alloc_run`, `hr_alloc`, `hinv_alloc`, `gh_alloc_raw`, `gcells_alloc`,
`gend_alloc` in one), `rt_inc_run` (the call returns 0 over a memory
representing `gh_inc g v` under the invariant at the roots plus the
word; `rth_inc_run`, `hr_inc`, `hinv_inc`); a law over the model's own
functions ports one to one — `hr_fill`, `hinv_fill` (the old
`hr_fill_k`, `hinv_fill`), `hr_seal`, `hinv_seal` (`hinv_seal_r`); every
parameter holds for any fuel of at least 12, the runtime function's
own cost, and the certificate instantiates it at the level its call
runs. The typed class throughout. The mechanism: an `axiom` the file
trusts is now recorded `PARAM NAME axiom` (`loader.shard` `admit_read`,
`admit_axiom_param`) — the store's manifest lists it as a parameter,
the release gate prints it as `PARAM` and verifies every theorem
resting on it as conditional on a named parameter, a `fulfills` at
phase 7 discharges it as an obligation — and is not entered in the
load's parameters, so the policy stays per file (pin `policy_cross`
unchanged; pin `policy_trusts` sharpened to `expect: main.ax param
axiom`, the harness's new form `NAME param KIND`). Before this a
trusted axiom was `ACCEPT`, the manifest listed no parameter, and a
store with one would have verified `UNDECLARED` and every consumer
`CONDITIONAL` with exit 1: 4.0's records had no producer of a declared
parameter but a view's and a measured fn's. `tb_app`'s closure is
Init's three axioms and the six parameters, nothing else; the release
prints six `PARAM` lines and the three measured descents' `DISCHARGE`
lines, no `CONDITIONAL`, no `UNDECLARED`.

*The proof.* `tb_app` follows from `tb_app_aux` — for every fuel, word
and depth, by induction on the first list, the second list's word and
readback fixed. The nil case: the increment on `b` by `rt_inc_run` at
fuel − 7 (`b` an immediate or a live address by `rbl_slot_ok`, below
2^32 by `rbl_bound`), the body by `app_body_nil`, the post-predicate at
`gh_inc g b` and `b` with the readback carried by `rbl_inc` and the end
and raw cell by `gend_inc`, `graw_inc`; past the depth the call fails
`FStack` and the predicate holds with any witnesses. The cons case:
the cell facts as `len`'s; the recursive call at fuel − 11 by the
induction hypothesis, its existential opened over the result record
(`∀ R, EV = R → ∃ g1 w1, …`, landing 1's trick) and split four ways —
no result, a statement result and a trap contradict it, a failure fails
the body and the call; a returned word `w1` with its ghost `g1`, then
the depth split, the allocation by `rt_alloc_run` at fuel − 12 and its
two outcomes — `FOom` fails the body —, and the success leaf
`app_cons_ok`: the raw cell's place from the invariant (`raw_window`:
rb + 152 ≤ p, p + 24 ≤ gend, 8-aligned, so even and the two stores
inside the window), the increment on the head word by `rt_inc_run` at
fuel − 13 with the word's parity and bound from the word kit
(`imm_odd`, `imm_lt`: the immediate is 2v + 1 less a multiple of 2^64,
so odd and below 2^64), which leaves the ghost (`gh_inc_odd`) and the
roots (`rinc_odd`); the ghost chain `app_ghost` — the first fill by
`hr_fill` and `hinv_fill` at an immediate, the second at `w1` with the
root it moves from the roots into the slot (`root_rinc`, `rdec_rinc`),
the seal by `hr_seal` and `hinv_seal` with the new cell a root, the
cells, end and raw cell after each step by the ghost kit, the address
fresh by `raw_fresh` —; the body's run by `app_E3_ret` and
`app_body_ret` through the step laws with the store (`stmt_storew`);
the new cell's readback (`hfget_mk`, `haddrs_mk`, the two slots, the
tail by `rbl_seal`); the post-predicate at the sealed ghost and `p`,
`rinc r p = p :: r` since `p` is even. The twelve run lemmas are the
product's statements over their locals at a fuel base, as `len`'s, the
calls' results variables (`app_E3_ret` takes the three calls' results
and the two stores' address facts as hypotheses). The mechanical text
was spelled by a scratch script at authoring time, not kept.

*What the proof found* (each with what was done):

1. *K's recursion depth.* The general form with the success leaf
   inline — about 190 `have`s on the deepest path — was refused
   `exhausted_rec_depth_at_infer` (`budget_depth` 8,192 in
   `k/inductive.shard`); landing 1 met the other budget. The leaf is
   `app_cons_ok` and its ghost chain `app_ghost`, each a K check of its
   own; the general form keeps the cell facts and the case splits (193
   lines). K untouched.
2. *The existential's introduction.* `(exact (Exists.intro w h))` with
   `h` a hypothesis whose type is known is refused `unify_stuck …
   blocked on p`: the elaborator checks `h` against the predicate
   applied to the witness before the result type meets the goal, a
   higher-order problem its first-order unifier cannot solve; the `@`
   form with the predicate written out is refused the same way when
   the proof argument is a local (it is accepted when the argument is
   itself an application, `probe_ex2`'s `Eq.refl`). The form that
   works: `(apply (Exists.intro w) ((show P) …))` — `apply` unifies the
   result with the goal first and opens the predicate at the witness
   unreduced, `show` converts it. Recorded as a Stage-1 gap of the
   elaborator (§11's row): the witness introduction of an existential
   goes through `apply`.
3. *Init's casts.* `↑n` in Init is `Nat.cast` at `instNatCastInt`, not
   `Int.ofNat`; V3 spells it `(@Nat.cast Int instNatCastInt n)` (law
   §5.1's `@` form), and the unifier takes `Int.ofNat n` for it by
   definitional unfolding only where no metavariable is in play — so
   the cast lemmas (`Int.toNat_of_nonneg`, `Int.natCast_emod`,
   `Int.ofNat_inj`, `Int.ofNat_lt`) are used through `exact` with the
   intermediate facts stated by `have`, never through `rw`. The
   parity of the immediate goes through `Int.emod_def` and linear
   arithmetic, not divisibility (`Int.instDvdInt` is no name of the
   export). Two implicit-argument changes of the pinned Lean:
   `Nat.succ_ne_zero`'s `n` and `List.mem_cons_self`'s both arguments.
4. *The reducer at an `if` over an open condition* leaves the
   projections of a constructor in the condition alone, so a rewrite
   by `if_pos` finds no `(= ad a)`; the constructor forms are stated
   as a small rewriting kit (`hincc_mk`, `hfget_mk`, `haddrs_mk`,
   `hsig_mk`, `haddr_mk`) and the inductions rewrite by them. And
   `unfold` of a definition that occurs inside the ghost term too
   (`rb_slot` inside `gh_fill g2 (rb_slot …)`) unfolds every
   occurrence and breaks the later match: `show` with the one
   occurrence spelled, as for the post-predicate.
5. *The existential's frame.* The twin named the output ghost; the
   existential says nothing of the input's cells surviving. `app`'s
   own proof needed no frame — every word it reasons about is the
   fresh call's result or an immediate whose parity the word kit
   proves, and the one place a frame would have served (the increment
   on the head word at the induction hypothesis's ghost) closes by
   parity — so rule 5's test is answered for the per-function
   certificate: `Prop` dissolves the twin. A caller composing
   certificates will need a stated frame (the old cells' addresses, or
   their readbacks, survive): phase 7's composition question, noted
   for B-1c.
6. *The loader's parameter record* (above): the one kernel-side change,
   with its pin and the harness's form; `ret_of_some` (`step.shard`)
   bridges a parameter stated over `ipcall` to the step laws stated
   over the result record.

*The count* (rule 6, read against landing 1's as rule 5 asked; the old
file: `tb_app` 1,148 lines, 774 step forms — 100 `have`, 90
`rewrite-with`, 287 `rewrite`, 106 `compute`, 191 `inst` —, 86 distinct
lemmas; its closure 364 claims in 35,547 lines, 278 of them the runtime
kit's in 30,514; the twin's four forms). The proof text: 489 lines —
the run lemmas 174, `app_ghost` 31, `app_cons_ok` 75, `tb_app_aux` 193,
`tb_app` 16 — against 1,148; 906 step forms, one statement each,
against 774. The kit written by need: 55 theorems and 6 parameters —
the ghost kit 49 (`ghost.shard`, 298 lines), the word kit 3 (`imm_word`,
`imm_odd`, `imm_lt`), the step laws 3 (`stmt_storew`, `stmt_storew_n`,
`ret_of_some`), the runtime's laws 6 (`rtlaw.shard`, 83 lines, standing
for the runtime kit's 278 claims until phase 7) — and seven definitions
(`hincc`, `gh_inc`, `gh_fill`, `gh_seal`, `hsig`, `irem`, `rdec`) beside
the source and the product. Cited by the proof: 55 of the kit (the
ghost kit 27, the step laws 14, the cell kit 6, the parameters 6, the
word kit 2), 15 of its own lemmas and 8 of Init's, against the old 86.
Refusals on the way: K's `exhausted_rec_depth_at_infer` (item 1) and
the elaborator's `unify_stuck` (item 2); hand-lowering: none; the
kernel's interventions: the loader's parameter record and the harness
form — no K change, no new syntax, no new form. The root — Init on
demand, 14 modules, the two roots `len.shard` and `app.shard` — loads
and checks with `tb_app` on Init's three axioms and the six parameters
in 126 s on the binary and 786 s on the bootstrap (K's replay of the
store 24 and 146 s), the records identical under both engines;
`imp_test.sh` asserts the gate, the parameters' records, the closure
and the release's `PARAM` and `DISCHARGE` lines, the store verified by
K alone (rule 4). Gates on the quiet dev box: the kernel-side files clean under the V3 loader, names at the baseline; the loader pins 267/0 on both engines (the sharpened pin among them); `v3/build.sh --all` 29 roots, 27 built, 2 current, 0 failed, in 2,123 s (two heavy jobs; the loader's chain emit 23 s, lower 74, codegen 247, cc 60 over 8,454 declarations); the bootstrap suite under `V3_NATIVE=1` 43 entrypoints, 0 failed, in 963 s of wall and 4,715 s of entrypoint time (`imp_test` 933 s: the two roots loaded and stored in 786 s, K's replay of the store 146 s, 236 checks; the tie 76 checks, 0 failed, in 350 s; parity byte-identical over 31 closures and 137,191 declarations in 409 s — the 19 more the loader's growth; `store_test` 613, `tactic_lib_test` 332, `define_test` 291, `path_test` 281, `calc_test` 272); the native suite under `V3_NATIVE=1` 43 entrypoints, 0 failed, in 406 s of wall and 1,328 s of entrypoint time (`imp_test` 150 s: the two roots loaded and stored in 126 s, K's replay 24 s; the tie 340 s; `calc_test` 187, parity 171, `store_test` 97); `suite_tie.sh` over the two kept runs 43 entrypoints, 0 differ, byte-identical under both engines on every entrypoint. CI's five jobs are the full gate.

**Slice 4.2 closed 2026-10-16** at this landing: the two readings of
rule 5 ratified as built (the user: "That makes sense. Ratify the two readings, close the slice, and let's open the next one.") —
the "three runtime laws" read against C2b's slicing, the allocation law
the raw-cell protocol (the call, the fill, the seal) and the increment,
no release law since `app` never calls `dec`; a run law whose old
conclusion named the runtime kit's memory restated with that memory
existential and its representation and invariant laws folded in, the
fill and seal laws one to one. The parameters' discharge is C2b's work
at phase 7 with the per-function generator; the frame conjunct a
composing caller needs is carried there (§11); the Stage-1 gaps found
(the existential's witness through `apply`, the mutual group, structural
recursion through a nesting) are the toolchain port's, phase 5. The
slice's three landings hold on the full gate (pipelines 616, 617, 620).

**Slice 4.3 — the program side: `bin`, `requires`, the well-threadedness
check, effect traces** (designed 2026-10-16 at slice 4.2's close, on the
user's "let's open the next one"; §13 item 68 — **ratified 2026-10-16 as
described (the user: "The program side sounds good as described --
proceed.")**, the leans beyond the description stated at item 68 for
objection; law §4.7, §9.3, §12.4 item 4's last item, §12.5's count rule;
§11's row "`bin`, `requires`, the World-use check, effect traces";
§12.5's deferred `(bin …)` row). Law §4.7 says what a faithful effectful
program is: effects are externs that thread a World token, uninterpreted
in proofs, given by bolt axioms and performed by the host; threading
alone is not the faithfulness argument — it takes (i) a
**well-threadedness check** on every `bin` closure, affine use of each
World token along every branch, mutually exclusive branches sharing an
incoming token, two sequential uses refused, ownership tracked through
supported aggregates, projections, helper calls and branches so that
distinct names establish nothing (a World boxed once and unwrapped twice
is one token used twice), the check following the staged, erased
executable structure so that a reference used only in an erased proof
consumes nothing, unsupported patterns refused conservatively and never
bypassed through a supposedly pure wrapper, a conservative local
discipline with explicit function summaries the initial fragment; (ii) a
coherent pure model of state, histories and observations from which the
effect laws are derived where possible, assumed law bundles tested on
small concrete models; (iii) a trace relation as the effect semantics,
so that no transformation duplicates, eliminates, memoizes or reorders a
host call because it looks pure — lowering preserves the ownership
discipline and the trace. Law §12.4 item 4 names the fixture: the
negative hides duplication behind structure (one World boxed, unwrapped
twice, written twice) and a helper boundary; exclusive branches each
consuming the incoming token once are accepted, as are erased proof
references; the World-alias fixture lands before any effectful
certificate counts as ported — phase 5's bulk port has the toolchain's
own drivers in it, so this slice gates it. It is also the goal's shape:
"shardfmt as a proven native bin" is a `bin` with a `requires`. Measured
2026-10-16 at the boundary, what V3 has:

| what | today |
|---|---|
| the forms | `bin`, `requires` and `app` are reserved and refused with the phase named (§4); the old tree's `(bin NAME (entry F) (externs …) (trusts …) (requires …))` has 16 uses; its driver reports `BIN name: met/total`, an UNMET requirement failing the run, `entry`/`externs`/`trusts` parsed and not enforced |
| the World | `kernel/host.shard`: `(type World (World Int))` and seven externs over the prelude's cells; `v3/std/host.shard`: `(type World (World Nat))` and six over `ByteArray` and Init's `List`, `Option`, `Prod`, `Bool` (slice 3.18) |
| an extern's L meaning | none: a function that calls one is `RUNNABLE … no_l_meaning` (`loader.shard`, the callee state), its equations never formed; `ev` stops at `EExt` (`EvStuck extern`); `run_prog` performs the extern through the host by the declared types' codec, the World returned at the type it was passed at |
| the entry | `run_prog` (slice 7, §9): the World last, every parameter before it a checked entry; `--run MODULE.FN FILE… -- ARG…` |
| requirements | `(requirement NAME …)` in a view, `(fulfills NAME PROOF)` in its implementation, discharged or pending (slices 3.14, 3.30); `trusts` the per-file policy, a trusted axiom a `PARAM NAME axiom` (4.2 landing 2); the release gate's `PARAM`, `DISCHARGE`, `CONDITIONAL`, `UNDECLARED` lines against the manifest (4.0) |
| the check, the trace | neither exists; the driver's output is the host's; `calc_main_test.sh` ties the program's output bytes to the model world's lines |
| the program | `examples/calc/calc_main.shard`: `main (w World)` reads the arguments, drives `step` over the lines, performs each `Action` (`write_line`, `exit`, nothing); the model `calc_app_world.shard`: `CalcWorld` (the lines written), `apply_action`, `run_world`, the claim `run_eq_spec_world` (slice 3.21) |
| the old contracts | `tools/shardfmt/mod.req/mod.req.shard`: 4 `sig fn` (the observables `w_reads`, `w_output`, `w_exit`, and `main`), 12 bolt axioms (one per effect × observable), 11 requirements, 217 lines; `examples/snake_game_3/mod.req/mod.req.shard`: 5, 12, 7, 214 lines (`BOUNDARIES.md`'s pattern) |
| route 1 | the kernel's drivers and harnesses (slice 4.1); no user program compiled to a binary yet (§11: specialization over user files is route 1's at phase 4) |

The rules:

1. **The check (law §4.7 (i)): well-threadedness over the E program.** A
   function `kernel/thread.shard` over `Prog` — the classifier's output,
   the staged erased structure, so a proof reference has already
   consumed nothing — for every E function that reaches an extern
   (directly or through a callee), in the closure's dependency order. A
   **token** is a value of a World type: the type an extern in scope
   takes as its last parameter, by identity (`kernel/host.shard`'s or
   `std/host.shard`'s, or a program's own). Along every path each token
   is used at most once: an `EExt` consumes its World argument and binds
   the result's token — the World result, or the World component of the
   `Prod`/`Pair` an extern returns, read by `EMatch` on the pair's
   constructor; a token boxed in a constructor is owned by the box, and
   a projection of the box yields the token again, so one box unwrapped
   twice is one token used twice; a call `ECall f` passing a token
   consumes it by `f`'s **summary** — computed from `f`'s own body
   first: a function that takes a token and returns one (or a pair
   carrying one) consumes and yields; one that takes a token and returns
   none consumes; a recursive call reads the function's own signature;
   `EMatch`/`EIf` branches are mutually exclusive, each may consume the
   incoming token once, and the branches' yields agree; `ELet` is
   sequential; a token left unused on a path is affine use. Unsupported
   ownership — a token in a list cell, under a template's or a lifted
   lambda's argument, passed to a primitive, in a constructor the check
   cannot follow — is refused conservatively, never passed through a
   pure-looking wrapper. The verdict is a record: `THREAD f ok`, or
   `THREAD f REASON DETAIL` with `world_twice` (the token and its two
   uses named by position), `world_unsupported` (the pattern named),
   `world_leak` (a token yielded twice). At this landing the record is a
   report on every extern-reaching function of a load, the kernel's own
   drivers among them (a measurement: how much of the toolchain's
   threading the initial fragment accepts, for phase 5's port); the
   refusal comes with the `bin` form (rule 4) on the bin's closure.

2. **Landing 0 is the check with the law's fixtures as pins.** Under
   `v3/pins/loader`, the harness's two new expect forms `NAME threaded`
   and `NAME unthreaded REASON`: the law's negative — one World boxed in
   a pair, unwrapped twice, written twice (`world_twice`); the same
   duplication behind a helper boundary (a helper that writes through
   the token it was given and also returns it, its caller writing again
   — the summary catches it); two sequential writes on one token; the
   positives — exclusive branches each consuming the incoming token
   once; an erased proof reference (a `theorem` over the World beside
   the program, consuming nothing); a helper threading correctly through
   a pair result; a token returned unused; and one conservative refusal
   (a token in a list, `world_unsupported`). Pins 267 → about 275. The
   kernel-side files under the V3 loader report their `THREAD` lines,
   the count written in the record.

3. **The World's L meaning.** `(extern NAME BINDERS RET)` is, in L, an
   **opaque constant** of its declared type at its identity (K's
   `opaque` kind, §4: a constant with a type and no equations), admitted
   beside its E declaration; a function that calls one is then `DEFINE`
   with its equations where today it is `RUNNABLE … no_l_meaning` — the
   externs' E side unchanged: `ev` still stops at an extern (pure
   evaluation refuses reachable externs, law §4.7) and `run_prog`
   performs it. The effect laws — the bolt axioms, one per effect ×
   observable pair as `BOUNDARIES.md`'s pattern and the old contracts
   have them — are `axiom`s a `bin` trusts; an observable (`w_output`,
   `w_reads`, `w_exit`) is an opaque constant over the World in the
   program's own file. The World type is the program's declared one (the
   driver builds its first constructor, 3.18), the extern's last
   parameter. This is what `std/host.shard`'s header calls phase 4's
   World: it is the prerequisite of a `requires` that names the entry at
   all.

4. **The `bin` form (landing 1).** `(bin NAME (entry F) (externs E…)
   (trusts A…) (requires R…))`, the old form's clauses, read by the V3
   reader (the reserved word released) and handled by the loader after
   the file's declarations: **entry** — `F` a `fn` of the file's closure
   with the World last and checked parameters before it (3.18's shape;
   `bad_entry` otherwise); **externs** — the roster: every `EExt` head
   in `F`'s E closure is in the list (`extern_outside` names the extern
   and the function) and every listed extern is declared; **trusts** —
   the closure's policy (§9): every axiom the closure's declarations
   rest on is Init's three or a trusted one, the trusted ones `PARAM`;
   **requires** — names in the closure, each an accepted `theorem` (or a
   discharged `requirement`) MET, else UNMET; and the check of rule 1
   over the closure, every function `THREAD … ok`. The record: `BIN NAME
   entry F externs N trusts N requires MET/TOTAL`, then `UNMET NAME` and
   `UNTHREADED F REASON` lines; an UNMET requirement, an unthreaded
   function, an extern outside the roster or an untrusted axiom **fails
   the load** (the old form's rule: pending is never silent). The
   store's manifest gains a `bin NAME F E… R…` line (4.0 rule 3); the
   release gate verifies that every required name is an accepted theorem
   of the store and that the closure's assumptions are the manifest's
   parameters — `BIN NAME ok` or `BAD-BIN NAME REASON` — the threading
   verdict the loader's (an E property K does not see), re-derived by
   every load of the file, so by `v3/build.sh` before any emit. The bin
   lives in its own file beside the program
   (`examples/calc/calc_bin.shard`, as
   `snake_game_3/mod.req/bin.shard`): a consumer of the loop never trips
   the artifact's gates.

5. **Landing 1's consumer: calc's main as the first bin.**
   `calc_bin.shard`: `(bin calc (entry main) (externs get_args
   write_line exit) (trusts …) (requires main_output main_reads))` over
   `calc_main.shard`; the observables `w_output` (the chunks written so
   far, `(List ByteArray)`), `w_reads`, `w_exit` as opaque constants;
   the bolts for the three externs calc touches (nine axioms: each
   effect's action on its own observable, preservation of the other two;
   `get_args` preserving all three); the requirements over `main`'s
   observables only (the old principle: never an internal function or
   state field) — `main_output`: for every `w`, `w_output (main w)` is
   `w_output w` followed by the model world's lines for the arguments,
   through `run_world` and slice 3.21's `run_eq_spec_world` to the spec;
   `main_reads`: `w_reads (main w) = w_reads w`. Proven through I by
   symbolic execution: induction on the lines, `drive_host` against
   `run_world`, the bolts rewriting the observables. The load reports
   `BIN calc … requires 2/2`; `bin_test.sh` (the 43rd entrypoint)
   asserts the record, the store's `bin` line, the gate's `BIN calc ok`,
   the two `PARAM`-free requirements' closures (Init's three and the
   nine bolts), the program run on both engines through the driver and
   tied to the model world as `calc_main_test.sh` does, and a negative:
   the same bin with a requirement left `sorry` fails the load with
   `UNMET`, with a write duplicated fails with `UNTHREADED`.

6. **Landing 2 is the effect trace, the model test, route 1's binary,
   the count.** (a) **The trace** (law §4.7 (iii)): `run_prog` records
   every host call in order — the extern's identity, its arguments and
   its result by the wire's codec — and returns it with the result;
   `--run … --trace FILE` writes it, one line per call. (b) **The
   model's trace**: the pure model computes the same list — calc's
   `apply_action` extended to emit the call a `Print` and an `Exit`
   stand for — so the trace is a function of the arguments. (c) **Route
   1's binary of the bin**: `v3/build.sh v3/examples/calc/calc_bin.shard
   v3/bin/calc` over the user root (the emitter's first user program:
   Init's `List`, `Prod`, `ByteArray` and `UInt8` representations and
   `std/host.shard`'s externs through the wire by declared type — a gap
   here is a finding with its pin, a landing's rule if it blocks), its
   `--trace` the same file. (d) **The tie, four ways, byte-identical**:
   the bootstrap's host trace, the native loader's, the model's trace
   under `ev`, and the binary's — the law's "no transformation
   duplicates, eliminates, memoizes or reorders a host call" measured at
   one program. (e) **The model test** (law §4.7 (ii)): the bolt bundle
   calc trusts, instantiated at the model world (`CalcWorld` with its
   observables as functions), proven as theorems — the assumed laws
   derived where possible — and the ground case of each run under `ev`
   on a three-call world (`decide`): assumed law bundles tested on a
   small concrete model. (f) The count of rule 7 read against the old
   contracts.

7. **The count, agreed before the run (law §12.5).** (a)
   `thread.shard`'s lines and its records; the pins written; (b) the
   loader's growth — the opaque extern, the `bin` form, its record, the
   manifest line, the gate's check — in lines; (c) calc's contract:
   observables, bolts, requirements, proof lines, against shardfmt's
   4/12/11 in 217 lines and snake's 5/12/7 in 214; (d) `bin_test.sh`'s
   seconds on both engines, the proof's within them; (e) the `THREAD`
   verdicts over the kernel's own drivers (ok, twice, unsupported, by
   count) — phase 5's port reads them; (f) the trace tie: four traces,
   their length, identical; (g) route 1's binary: the chain's seconds,
   the binary's run against the interpreter's; (h) the emitter's and
   Stage 1's gaps found, each a pin; (i) refusals before success and
   interventions, T9's protocol. No pass bar; the user may set one
   before landing 1.

8. **Both engines, the bootstrap the authority** (4.1 rule 6):
   `bin_test.sh` under the suite on the bootstrap and on the shim,
   compared byte for byte as every entrypoint is; the native tie gains
   the binary's trace.

*Not in this slice, stated:* T6's prepared handle and the long-lived
environment (R82's three identities first, its own slice); the MVU `app`
driver (§12.5's dropped row: a loop in a `bin` is the story, no second
form); the String flip (route 1's chain, phase 5); bolts for
`read_file`, `read_range` and `write_file` (a roster is per bin; the
toolchain's own bins write theirs at phase 5); the transitive
trust-ledger tool (`BOUNDARIES.md`'s audit ledger — the release gate's
`PARAM` lines are V3's); the trace relation as a theorem about route 1's
lowering (phase 7, with route 1's proof: here it is a measured tie);
effect handlers other than the host (a model world is the pure model,
not a handler); tokens through templates, lifted lambdas and list cells
(refused conservatively; a §11 row when a port needs one);
`examples/sha256sum` and the other bins (phase 5 by manifest).


**Slice 4.3 as built, landing 0 (2026-10-16) — the well-threadedness
check, and the law's fixtures as pins.** Rules 1 and 2.
`v3/kernel/thread.shard` (331 lines): the check over an E function's
program as rule 1 states it, with these refinements found at the build.
(a) **The kinds in scope are a fixpoint, not a walk.** A first version
decided whether a type carries a token by walking its constructors'
fields with a depth fuel and answering yes when the fuel ran out; the
kernel's record types are deeper than eight layers, so every function
over `W`, `Cx` or `Load` became token-handling and 2,249 of the loader's
own functions read as `world_twice`. As built, `th_kinds` computes once
per file end the World types (the last parameter type of every extern of
the table) and the **carriers** — the least set of inductives one of
whose constructors has a field whose type names a World or a carrier at
any head; a parametric field says nothing, its instantiation judged
where the type is used (`(Prod Int World)` names the World) — and
`carries` is a linear walk of a type expression over the two lists. (b)
**A branch yields in its own state.** The arms of a match and the sides
of an if each consume the value they yield before the join, so a token
used in one arm and yielded by a sibling is used once on either path;
the value of the whole is fresh, of the arms' kind, allocated after the
join, and each arm's identities continue from the join so far so that no
two arms share one. The first version consumed the arms' values in the
joined state and started each arm from the previous arm's counter:
eleven of the loader's functions (`run_loop`, `load_forms`, `print_recs`
among them) read as `world_twice` on a value no path uses twice. (c)
**Constructors are boxes, uniformly.** A token boxed in any constructor
— a pair, a list cell, a user record — is owned by the box, consumed
when the box is passed on and yielded again by one match on it; the
design's example of a token in a list cell is supported, not refused.
The unsupported class is a token at a polymorphic or primitive
parameter, the World itself matched, and a callee the table lacks; a
World built by its constructor is `world_forged`. (d) **The verdict's
detail names the origin and both uses** (`parameter 2 used twice:
main.write_line, boxed in Prod.mk`; `a Prod.mk used twice: matched,
matched`). The loader records `THREAD NAME ok` or `THREAD NAME REASON:
DETAIL` at a file's end for every E function the table gained since the
file's imports (`Fx.ecount` captured after the imports load — captured
at the file's start, every importer re-reported its imports' functions)
that carries a World in its signature or reaches an extern in its body;
`rec_count` leaves the summary line as it was. The harness's two forms,
`NAME threaded` and `NAME unthreaded REASON`, the first in the two-word
branch of `matches` beside `read-error REASON`.

**The pins (nine, 267 → 276).** The law's negative —
`thread_pair_twice`: one World boxed in a pair, unwrapped twice, written
twice, `a Prod.mk used twice: matched, matched`; the duplication behind
a helper boundary — `thread_helper_twice`: a helper that writes through
the token it was given and also returns it, the fault in the helper's
own body (`parameter 2 used twice: write_line, boxed in Prod.mk`), its
caller threaded; `thread_sequential`: two writes on one token. The
positives — `thread_branches`: the arms of a match on `read_file`'s pair
and the sides of an if on `write_file`'s flag each consuming the
incoming token once; `thread_helper_ok`: a helper returning the token
inside a pair, a recursive `echo` consuming and yielding along one path;
`thread_unused`: a token dropped (`drop`) and the arguments unused —
affine. The check's own classes — `thread_poly`: a token passed to a
polymorphic `wrap`, `world_unsupported`; `thread_forged`: `(World 0)`
built in a program; `thread_match_world`: the World itself taken apart.
Not written at this landing: the law's **erased proof reference** — a
function calling an extern is read E-first today (`no_l_meaning`), so a
proof term inside it is no E term at all; the fixture lands with the
World's L meaning (landing 1, rule 3), where the typed route erases the
proof and the check sees the token once.

**The kernel's own drivers measured** (rule 2's count, on the
bootstrap): the loader's closure (`load.shard`, 42 modules, 3,046
runnable) 87 token-handling functions, 87 ok, in 12 s; `prove` 80 of 80;
`verify_release` 51 of 51; `initindex` 25 of 25; `t0` 2 of 2. **One
finding against the loader, fixed:** `load_file_module`'s failure branch
after pre-registration returned the World `sealed_ident` had already
consumed (`w1`) instead of the one the sidecar's read returned —
harmless under the interpreter, where the token is a datum, and exactly
the fault law §4.7 names; the check found it on its first run over the
kernel. **One finding against the kernel's naming:** the bootstrap
dispatches a call by its bare name across every module of the process,
so a function name defined in two kernel files is a collision whether or
not either imports the other — the check's association lookup shared
`assoc_int` with `ev.shard` and the loader's run reached the evaluator's
version (a match on an integer with no arm); renamed, and the kernel's
function names are unique across its files, the drivers' `main`
excepted. The names check extended to that rule.
**Slice 4.3 as built, landing 1 (2026-10-16) — the World's L meaning,
the `bin` form, calc's bin.** Rules 3, 4 and 5, with these refinements
found at the build. (a) **An extern's L constant is an opaque with a
witness.** K's opaque kind checks a value and never unfolds it — Lean's
`opaque` takes `default` of an `Inhabited` instance — so the constant's
value is the declared type's default witness, built structurally by
`kernel/inhabit.shard` (98 lines): a function type a lambda whose body
is the codomain's witness, an inductive its first constructor whose
fields all close applied to the type's parameters and the fields'
witnesses in order; a sort, a proposition or a type without a closing
row has none and the extern stays E only, `extern_uninhabited` on its
record. The reader's `read_extern` builds the declaration; the loader's
`load_extern` admits it (an ACCEPT) before the classifier's RUNNABLE, a
file without Init keeping the extern E only by construction; the
erasure sends a constant the E table holds as an extern to `EExt`.
Every extern of `std/host.shard` and of the pins under the naming law
has its constant, and every function calling one — calc's `perform`,
`drive_host` and `main`, the derive, path and template mains, the wire
pins — is defined with its equations where it was `RUNNABLE …
no_l_meaning`. (b) **A World is a type a module declares.** An extern
whose last parameter is `Nat` — the pins' stand-in for a foreign
function — made `Nat` a World type to landing 0's check and every
function over `Nat` token-handling (`define_runnable`'s `succ` read as
`world_unsupported: a token at the primitive +`); as built, Init's bare
names and the prelude's are never Worlds (`world_name`). (c) **Three
pins supplied their E-first route through an extern** and now take the
typed route: `row_e_first` and `derive_e_first` call a fn over a
`Symbol` for it, `str_e_first` expects `ok`, its function defined;
`define_runnable` and `zero_arity_bare` keep their outcome with their
headers corrected; `template_main`'s E-first instance — slice 3.28 rule
3's last sentence's fixture — is pin `template_e_first`'s now, its body
E-first through a fn over a `Symbol`, `template_test.sh` reading both.
(d) **The bin's policy admits the load's
parameters.** Calc's bin rested on calc's three pending descents
(`show_nat.dec_1`, `parse_tail.dec_1`, `parse_tail.dec_2`) and read as
`untrusted` on its first load; a measured function's obligation and a
view's parameter are §9's third and fourth classes, permitted as the
file's policy permits them (`outside_policy`), the manifest's `param`
lines carrying them — the bin's `trusts` is the roster of *axioms*. (e)
**Two findings the bootstrap's bare-name dispatch hid**, found by the
drivers' measurement and the gate's first run: the program types'
constructors were not opened in `loader.shard` (four of the bin check's
functions refused `unknown_head` under the V3 loader, clean under the
bootstrap — three `use` lines); and `name_of_sym` lies outside
`verify_release`'s closure, where the bootstrap treats an unknown head
as an effect ("expected a World as the last argument") — the gate
resolves the manifest's names through the store's accepted names
(`vr_find_name`), and a string literal as a match pattern is the
bootstrap reader's refusal (an `if` instead). (f) **The form as built.**
`(bin NAME (entry F) (externs E…) (trusts A…) (requires R…))`, read as
a later form and stashed per file (`Fx.bins`), checked at the file's end
after the threading verdicts (`rec_bins`): the entry resolved among the
E values in scope, its shape §9's (the World last — `is_world` by the
check's kinds —, `Int`, `Nat` or a byte-list codec before it); the
roster against the heads of the entry's E closure (`bin_closure`,
`bin_heads`), an extern outside it named with its function; each
requirement a theorem in scope or a discharged requirement's
`NAME.proof`, one read and pending UNMET, counted; each trusted name an
axiom in scope, then every requirement's and every defined function's
assumption closure (`axioms_of`) within the policy; every function of
the closure `th_fn` ok. The records `RBin` (`BIN NAME entry F externs N
trusts N requires MET/TOTAL`) and `RBinErr` (`UNMET R`, `UNTHREADED F
VERDICT`, `BAD-BIN NAME REASON: DETAIL` — unmet, unthreaded, bad_entry,
extern_undeclared, extern_outside, unknown_requirement, unknown_trust,
untrusted), each error failing the load; the manifest's `bin NAME entry
F externs E… requires R…` (`stored_bins`, `bin_lines`); the gate's
`vr_bins` — the entry an accepted def, every extern an accepted opaque,
every requirement an accepted theorem whose closure `vr_undeclared`
clears — `BIN NAME ok` or `BAD-BIN NAME: REASON`, counted with the
conditional declarations. The harness's forms `NAME bin ok` and `NAME
bad-bin REASON` (the latter any record, a PENDING standing before its
UNMET). (g) **Calc's bin** (`examples/calc/calc_bin.shard`, 142 lines):
three observables as opaques (`w_output`, `w_reads`, `w_exit`), nine
bolts — `get_args` in the premise form over the pair it returns, as
`BOUNDARIES.md`'s pattern had it; `write_line` appending its line,
`exit` recording its code, each preserving the other two — and the two
requirements over `main`'s observables only, `main_output` (the lines
written are the model world's lines for `Prod.fst (get_args w)`, through
`run_world` and 3.21's `run_eq_spec_world` to `run_world_spec`) and
`main_reads`, proven in 53 lines through seven lemmas: `out_bytes` over
an append, the action's lines as a plain function (`action_lines` —
stated first as `world_lines (apply_action (CalcWorld nil) a)`, a
rewrite rule whose right side holds its left, which `rw` then nested),
the model's accumulator `run_world_acc` (induction on the events, `out`
and the state generalized, `cases` on the emitted action), `perform`
against the bolts, and `drive_host` against `run_world` (induction on
the lines, the state and the World generalized; `rw` by the equations
of `drive_host`, `codes_of_lines`, `run_world`, `emitted` and
`next_state`, then `cases` on `step s (codes_of (Bytes.toList l))`
and `reduce`); the requirements by `main.eq_1`, `cases` on `(get_args
w)` with its equation `hg` for the premise-form bolt, `reduce`, the
lemmas. The closures: `main_output` rests on Init's two and its three
bolts, `main_reads` on Init's two and its three — Init's `Quot.sound`
through the matchers' `casesOn`, `propext` through `if_pos` — and the
closure's definitions on calc's three descents through the parameters.
`BIN examples.calc.calc_bin.calc entry examples.calc.calc_main.main
externs 3 trusts 9 requires 2/2`; the program runs through the driver
with the bin file as the root file, byte-identical to the model world.
Against the old contracts' 4/12/11 in 217 lines (shardfmt) and 5/12/7
in 214 (snake): 3/9/2 in 142, the proofs in it.

**The pins (eight, 276 → 284)** — `thread_proof_ref`, the law's erased
proof reference (§12.4 item 4): a proof about the token under a typed
`let`, erased, the token used once (deferred from landing 0, where an
extern-calling body was read E-first); `bin_ok` (`BIN main.hello entry
main.main externs 1 trusts 1 requires 1/1`, the requirement through the
entry's equation and the bolt); `bin_unmet` (`sorry`: `requires 0/1`,
`UNMET`), `bin_unthreaded` (two writes through one token), `bin_extern_outside`
(`main.exit is called in main.finish and is not in the roster`),
`bin_bad_entry` (no World last), `bin_untrusted` (the requirement rests
on a bolt the bin does not list); `template_e_first` (the E-first
instance with no equations). **`bin_test.sh`**, the 44th
entrypoint under `V3_NATIVE=1` (27 checks): the record, the
closure's verdicts, the nine PARAM records, the externs' constants, the
two requirements' closures exactly, the manifest's `bin` line, the
gate's `BIN … ok` and its acceptance of every stored declaration, a
tampered manifest naming a definition as a requirement `BAD-BIN`, the
program through the bin tied to the model world over the differential's
21 inputs, and the two negatives over a scratch root seeing the library
through a link — `main_reads` left `sorry` (`requires 1/2`, `UNMET`), a
write duplicated in `perform` (`UNTHREADED … world_twice`). Both
engines, compared byte for byte by the suite.

**The kernel's own drivers measured** (rule 7 (e), on the bootstrap,
under the refined World rule): the loader's closure (43 modules, 3,092
runnable) 87 token-handling functions, 87 ok; `prove` 80 of 80;
`verify_release` 52 of 52 (`vr_bins` the one more); `initindex` 25 of
25; `t0` 2 of 2. The growth: `loader.shard` +282 lines, `verify_release`
+110, `reader` +39, `store` +21, `thread` +15, `load` +9, `erasure` +8,
`inhabit.shard` 98 new; the harness +24. Gates on the quiet dev box:
the kernel-side files clean under the V3 loader, names at the baseline; the loader pins 284 cases, 0 failed, on both engines; `v3/build.sh --all` 29 roots, 29 built, 0 failed, in 2,516 s (two heavy jobs; the loader's chain emit 24 s, lower 82, codegen 319, cc 71 over 8,565 declarations, 51 modules — 46 more than landing 0's; then the pins harness's and the reader test's roots, edited after the build, rebuilt in 440 s, 2 built, 27 current); the bootstrap suite under `V3_NATIVE=1` 44 entrypoints, 0 failed, in 882 s of wall and 4,368 s of entrypoint time (bin_test 296 s; imp_test 832 s: the two roots loaded and stored in 692 s, K's replay 140 s, 236 checks; the tie 76 checks, 0 failed, in 332 s; parity byte-identical over 31 closures and 138,195 declarations in 344 s — the 414 more the landing's growth across the closures, the externs' constants among them; store_test 544, calc_test 228); the native suite under `V3_NATIVE=1` 44 entrypoints, 0 failed, in 408 s of wall and 1,361 s of entrypoint time (bin_test 66 s; imp_test 146 s: the two roots loaded and stored in 123 s, K's replay 22 s; the tie 335 s; parity 168 s; calc_test 185, store_test 94; the loader pins' entrypoint 13 s); `suite_tie.sh` over the two kept runs 44 entrypoints, 0 differ, byte-identical under both engines on every entrypoint.

**Slice 4.3 as built, landing 2 (2026-10-16) — the effect trace, the
model's trace, the model test, the count; route 1's binary blocked at
the chain, deferred for ratification.** Rule 6 (a), (b), (d), (e) and
(f); rule 6 (c) not built, stated below. (a) **The trace** (law §4.7
(iii)): `run_prog_traced` (`kernel/ev.shard`) records every host call
the run performs, in order, as one line — the extern's short name, its
arguments as the host saw them, the host's answer where it carries data:
`get_args "1+2" "10 - 3"`, `read_file "p" -> (some "…")` or `-> none`,
`write "…"`, `write_line "…"`, `write_file "p" "…" -> true`, `exit 3`;
bytes quoted with `\"`, `\\`, `\n`, `\t`, `\r` and `\xHH` for any
other byte outside 32–126 (`tr_quote`). `perform` returns the line
beside the value; `run_loop` accumulates and writes the file at the
run's end, and before an `exit` is performed — the host's exit never
returns, so the trace of a run that exits is written first. The driver's
`--run … --trace FILE` (`load.shard`); `run_prog` unchanged for its
other callers. (b) **The model's trace** (`examples/calc/calc_app_world.shard`):
`Call` — `CArgs`, `CWrite`, `CExit` — and `trace_of`, the arguments then
the call each action stands for through `emitted` and `next_state`,
a function of the arguments. `kernel/test/calc_harness.shard` evaluates
it under `ev` on request (a fourth argument, `trace`; `calc_test`'s tie
to the old tree reads the output without it) and prints one `* trace:`
line per call rendered by the driver's own functions, so what the tie
compares is the two call lists.
(c) **Route 1's binary of the bin is not built.** The emitter refuses
calc's closure before the chain sees it, as rule 1 of slice 4.1 has it
refuse what the chain cannot take: a string literal over Init's
`List.cons` — the chain builds its closure's `Cons` and `Nil`, the old
reader's string — and the primitive `Nat.sub`, reached through
`List.take`, beyond the chain's eighteen and with the integers'
semantics there. The two are pinned under `v3/pins/emit` as
`refused_init_list` and `refused_init_prim`, the tie expecting their
messages (`v3/build.sh` passes the pinned export to the emit when the
fixtures hold it, so a naming-law root can be emitted at all; the tie
takes a case at its root beside the `kernel/main.shard` layout, since
the `kernel` module is the toolchain's scope, where a string literal is
refused before the emitter). The gap is the chain's representation of
Init's types — `List`, `Prod`, `Option`, `Bool`, `ByteArray`, `UInt8`,
`Nat` — against its prelude's cells and primitives, the same ground the
String flip waits on (§13 item 68, "not in this slice"); mapping Init's
types onto the old chain's cells is work on the chain phase 5 replaces.
**The lean, for ratification:** the binary of a naming-law program is
phase 5's, with the chain's replacement, and the tie of this landing is
three ways — the bootstrap's driver, the native loader's (`v3/bin/load`,
the same driver compiled), the model's — not four. (d) **The model
test** (`examples/calc/calc_bin_model.shard`, 55 lines; law §4.7 (ii)):
a pure host — the arguments, the lines written, the files read, the
exit, as data — with the three externs calc touches as functions over
it and the three observables as projections; the nine bolts the bin
trusts instantiated at it and proven, each by `cases` on the host and
computation (the `get_args` three in their premise form: the premise
restated through a `have` for the elaborator's first-order unifier,
which does not unfold `h_get_args` to the pair to apply `congrArg
Prod.snd` — §11's Stage-1 gap again; the constructor cited `Host.Host`
in a term, T5's rule), and a ground run of three calls computed — a
user type has no decidable equality for `decide`, so `reduce` and
`rfl`; the ten accepted on no axioms. The bundle is consistent because
this model satisfies it. (e) **The tie, in `bin_test.sh`**: the
program through the bin under `--trace`, the trace one `get_args` and
one `write_line` per input — 22 calls over the differential's 21 — byte
for byte the model's; the model host's ten theorems; 42 checks now,
both engines under the suite. **The count (rule 7).** (a) `thread.shard`
346 lines, its verdicts over the drivers 87/80/52/25/2 ok, none twice or
unsupported; the pins 9 at landing 0, 8 at landing 1. (b) The loader's
growth (landing 1): `loader.shard` +282, `verify_release` +110, `reader`
+39, `store` +21, `thread` +15, `load` +9, `erasure` +8, `inhabit` 98
new; landing 2: `ev.shard` +104/−59 (the trace), `load.shard` +48/−33
(`--trace`), `calc_app_world` +19, `calc_harness` +37, `bin_test.sh`
+24, the tie +7, `build.sh` +5. (c) Calc's contract 3/9/2 in 142 lines,
53 of proof through seven lemmas, against shardfmt's 4/12/11 in 217 and
snake's 5/12/7 in 214; the model host 55 lines, ten theorems. (d)
`bin_test.sh` 320 s on the bootstrap, 68 s on the binaries. (e) The `THREAD` verdicts over the kernel's own
drivers: the loader's closure 87 ok, `prove` 80, `verify_release` 52,
`initindex` 25, `t0` 2 — no `world_twice`, no `world_unsupported`. (f)
The trace tie: three traces of 22 lines, identical. (g) Route 1's
binary: none — the two refusals. (h) The emitter's gaps: two, pinned;
Stage 1's: the unifier at an unfolded definition (as §11 records). (i)
Refusals before success: landing 1's proof one (a rewrite rule whose
right side held its left), the model host's three (the horizon of
`Prod.mk.inj`, the unifier, the constructor's spelling), `bin_test.sh`'s
scratch root two (the store directory, the loader's relative paths),
two build roots stale from edits after the build; at this landing
`calc_app_world`'s trace twice (the state constructor's and `Option`'s
namespaces unopened: `CalcState` the type where the constructor was
meant, `none` unbound) and the harness's `trace` flag once (`False`
unopened in a kernel-profile root — clean under the bootstrap's
bare-name dispatch, refused by the emit under the V3 loader, the class
landing 1 found twice), and `calc_test`'s tie to the old tree once (the
trace lines made opt-in). Gates on the quiet
dev box: the kernel-side files clean under the V3 loader, names at the baseline; the loader pins 284 cases, 0 failed, on both engines; `v3/build.sh --all` 29 roots, 29 built, 0 failed, in 2,426 s (two heavy jobs; the loader's chain emit 27 s, lower 78, codegen 312, cc 74 over 8,575 declarations; then the harness's root, edited after the build, rebuilt in 401 s); the bootstrap suite under `V3_NATIVE=1` 44 entrypoints, 0 failed, in 866 s of wall and 4,336 s of entrypoint time (bin_test 320 s; imp_test 817 s: the two roots loaded and stored in 677 s, K's replay 140 s, 236 checks; the tie 76 checks, 0 failed, in 349 s, the two refusal pins among its cases; parity byte-identical over 31 closures and 138,333 declarations in 337 s — the 138 more this landing's growth across the closures, calc's five leaves 485 declarations against 467; store_test 537, calc_test 224); the native suite under `V3_NATIVE=1` 44 entrypoints, 0 failed, in 416 s of wall and 1,343 s of entrypoint time (bin_test 68 s; imp_test 142 s: the two roots loaded and stored in 120 s, K's replay 22 s; the tie 348 s; parity 165 s; calc_test 182, store_test 90; the loader pins' entrypoint 13 s); `suite_tie.sh` over the two kept runs 44 entrypoints, 0 differ, byte-identical under both engines on every entrypoint.

**Slice 4.3 closed 2026-10-16** at this landing: the deferral ratified
(the user: "Option 1 sounds like the way to go.") — the binary of a
naming-law program is phase 5's, with the chain's replacement (law
§12.4 item 5; `v3/MANIFEST.md`'s row for `tools/lower` and
`tools/codegen`: ARCHIVE at the flip, route 1 replacing it), and the tie
of this landing is three ways — the bootstrap's driver, the native
loader's, the model's. Rejected: extending the old chain now — Init's
`List`, `Prod`, `Option`, `Bool`, `ByteArray`, `UInt8` and `Nat` mapped
onto its prelude's cells, a truncating helper for `Nat.sub`, a trace in
its C runtime — days of work on the chain phase 5 replaces. Carried to
"route 1's chain is V3's own" (phase 5), a list of five now: the
kernel's string literals as byte lists (§13 item 51), the comparison
primitives' old spellings under `v3/kernel/**` (§8.4 slice 3.17 rule 5),
`Symbol`'s `String` flip (item 64), `derive` and `by` in the toolchain's
sources, and the binary of a naming-law program (this slice); the trace
relation as a theorem about the lowering is phase 7's, with route 1's
proof. Noted at the close, for the boundary: the replacement itself has
no designed slice — the manifest names it by row (the lowering-side
toolchain as new code, its certificates I), slice 4.2's "not in this
slice" puts `impc`'s V3 version beside the generator, and the imp path
has compiled the probes' functions and `sha256` only, never a closure of
the loader's size with its GC, bignum and symbols; the sizing is the
question the deferral leans on. The slice's three landings hold on the
full gate (pipelines 625, 628, 629 — the last about 1,950 s longer in
each heavy job for this landing's `build.sh` index lookup, which built
the index on the bootstrap before the native driver existed; found in
the traces and fixed at the close: `init_index.sh --current`, never a
build inside a root's build). What the slice built, in one line: law
§4.7's faithful effectful program in V3 — the check, the `bin` with its
contract proven through I, calc's main the first bin, the trace tied
three ways and the bolts tested on a model host.

**Reopened the same day at the boundary (§13 item 69, ratified):**
the deferral stands for the proven binary only — phase 7's, with V3's
own chain; the binary of a naming-law program comes at slice 4.4
through the bridge, because V3's own chain is itself a naming-law
program and nothing else compiles one. The flip no longer waits for the
chain's replacement, and of the five debts listed above one was the
chain's.

**Slice 4.4 — the bridge: Init's types onto the chain at the emitter;
the kernel's comparison spellings renamed** (designed 2026-10-16 at
slice 4.3's close, on the user's question at the boundary — "Is there
any of this ordering you think we should rearrange or reprioritize?
Anything likely to become a chicken-and-egg problem?" — and the three
changes answered; §13 item 69 — **ratified 2026-10-16 as outlined (the
user: "Agreed with the plan changes as outlined; update the docs.")**;
law §9.1 route 1, §12.4 as amended 2026-10-16, §12.1's "generated
files never hand-patched"; slice 4.1 rules 1, 5 and 6). The finding:
V3's own chain is a naming-law program — its AST and machine types are
the ported models' under `v3/models/imp`, on Init's `Nat` and
`std/list` — and the only compiler of naming-law programs is the chain
being replaced, which refuses them (4.3 landing 2); the bootstrap
interprets, at hours per generation of the loader's closure. So the
replacement is phase 7's with its proof, bootstrapped through this
bridge, and the temporary chain is route 1's engine through the flip.
Measured at the boundary against `kernel/emit.shard` (418 lines): the
two refusals are emitter-side — `em_check_lit` refuses a string literal
over any list but one spelled `Cons`/`Nil` (the chain's reader builds
its prelude's for `"…"`), `em_check_term` refuses every primitive whose
name carries a dot (Init's realized primitives; `Nat.sub` the first
calc reaches, through `List.take`) —; every inductive of the closure is
emitted as a `(type …)` (`em_decl`'s `EInd` arm: Init's `List`,
`Option`, `Prod` and `Bool` among them, under their short names); the
primitives print under the chain's spellings from the program whatever
the source spelled (`em_prim`: `=` → `int_eq`). The rules:

1. **Landing 0, the probe — the kill-test, documents and a
   measurement.** On a scratch copy of the emitter (never the tree's:
   §12.1), the literal printed as the closure's constructors and
   `Nat.sub` as a truncating expression, calc's closure lowered until
   the chain accepts it or the next refusal names itself; the list of
   every dotted primitive calc's closure and the std libraries'
   realizations reach, each with its chain spelling where the semantics
   agree on the values E reaches or its expression where they differ
   (`Nat.sub` truncating at zero; a division by zero per the migration
   table, `x / 0 = 0` and `x % 0 = x`, where the chain is stuck or
   traps); the chain's seconds on calc's closure. Recorded before
   landing 1 is built. A primitive with no expression (a width's
   arithmetic, a character) re-designs the slice here rather than
   pushing through.
2. **Landing 1, the bridge.** `kernel/emit.shard` prints a string
   literal as nested applications of the closure's own list
   constructors under their emitted names (Init's `List.cons`/`List.nil`
   or the prelude's, whichever the literal's type is), and maps each
   dotted primitive by landing 0's table — by name to one of the
   chain's, or to an expression over them. No edit of the old tree
   (4.1 rule 5 stands); the stamp covers the emitter, so every binary
   rebuilds. The two refusal pins become ties (`init_list`,
   `init_prim`, a `Nat.sub` underflow among the cases); calc's main
   built by route 1 (`calc_main.shard` the root, as the drivers are
   built, its bin's contract beside it) and tied on its observables —
   stdout and exit status over the differential's inputs against the
   bootstrap driver's run and the native loader's; the drivers tied to
   the model on the trace as 4.3 has it. The chain's runtime gets no
   trace: the binary's trace relation is phase 7's, with route 1's
   proof. `bin_test.sh` and the native tie gain the binary; the pins'
   README's refused rows become tied rows.
3. **Landing 2, the rename.** `v3/tools/rename_cmp.py` over
   `v3/kernel/**` (665 sites in 52 files at 3.17's count): `lt le
   int_eq` → `< <= =`, then `scope.shard`'s `toolchain_scope` exception
   retired so the old spellings are `renamed_primitive` everywhere.
   The Rust bootstrap reads both spellings already (3.17 rule 5) and
   the emitter prints the chain's; no semantic change. The gate:
   parity (both readers project the same), the loader pins, the suite
   on both engines and the tie. One commit.
4. **Not in this slice, stated:** Init's types in the toolchain's
   closure — the string flip for the kernel's 1,649 sites (item 51),
   `derive` there (3.19), `Symbol` as `String` (item 64) — phase 5's
   opener design beside the migration tool, with one constraint: the
   index driver's closure stays Init-free, since it builds the index
   that makes Init loadable; the `by` form in the kernel's theorems
   (the Rust reader's, enduring; a reader rule if a port needs it);
   V3's own chain and its sizing (phase 7 with its proof; the sizing
   memo at phase 5's opener); T6, the next of law §12.4 item 4's list,
   after this slice.
5. **The count, agreed before the run, no pass bar:** `emit.shard`'s
   growth in lines; the dotted primitives mapped, by name and by
   expression; the chain's seconds on calc's closure and the binary's
   run against the interpreter's over the differential; the rename's
   sites and files; refusals before success and interventions, T9's
   protocol.

**As built, landing 0 (2026-10-16): the probe.** On a scratch copy of
the kernel's sources (`v3/models/probe44_…`, deleted after; the tree
untouched), the emitter patched four ways and calc's closure taken
through the chain until the chain accepted it — three refusals, each
named by the emitter before any file was written, then the lowerer,
the generator and `cc` in under two seconds on `bin/shard_eval`:
(1) the dotted primitives let through and mapped by name (`Nat.sub` →
`-`, `Nat.div` → `/`, `Nat.mod` → `mod`, the bitwise five → `band bor
bxor bshl bshr`, `Nat.beq`/`Nat.ble` → `int_eq`/`le`, `Int.tdiv`,
`tmod`, `ediv`, `emod` → `/ tmod ediv mod`; `Nat.gcd` and `Nat.pow` left
under their names for the chain to refuse if reached); (2) **Init's wire
types identified with the chain's prelude — the mechanism the design
did not foresee, and the chain requires**: the string literal needed no
change at all, since the chain's reader builds `Cons`/`Nil` for `"…"`
and Init's `List` is that list once `List.cons`/`List.nil` print as
`Cons`/`Nil`; the runtime's extern shims build `Pair`, `Some`/`None`,
`Cons`/`Nil` and `True`/`False` by those names (`rt.h`'s `rt_pair`,
`rt_some`, `rt_cons`), so the program's values must be those cells; and
a pattern name the old reader does not read as capitalised binds a
variable (`kernel/reader.shard`'s `sym_capitalized`), so Init's
lowercase constructors cannot be emitted as they are. Hence `List.cons`,
`List.nil`, `Prod.mk`, `Option.some`, `Option.none`, `Bool.true`,
`Bool.false`, `Decidable.isTrue` and `Decidable.isFalse` print as the
chain's constructors in terms, patterns and the four type declarations
— `(type (List T0) (Nil) (Cons T0 (List T0)))`, `(type (Option T0)
(None) (Some T0))`, `(type (Pair T0 T1) (Pair T0 T1))`, `(type Bool
(False) (True))`, emitted from Init's inductives in the `Init` module as
the kernel's prelude emits its own; `Decidable`, `Nat` and `Int` not
declared, their constructors never reaching E —; (3) the types mapped:
`Nat`, `UInt8`–`UInt64`, `USize`, `Fin` and `Char` → `Int`; `String` and
`ByteArray` → `(List Int)`; `Decidable` → `Bool`; `Subtype T p` → `T`;
`Prod` → `Pair` (`BitVec` missed by the probe's map and passed through
unchecked — the chain is untyped past its reader —, on landing 1's
list); (4) `absurd` printed as `(match 0)`, the chain's no-match sink
(`low_no_match`, a loud trap that never fires: K proved the branch
unreachable — `List.get` at `[]`). What calc's closure reaches: 61
declarations in 7 emitted modules, 119 lines (the lowered form 180, the
C 991); the dotted primitives `Nat.sub` ×4 (`List.take`, `List.drop`,
`List.get?Internal`, `List.get`, each under the match on zero),
`Nat.mod` ×1 (`Byte.ofNat`, by 256), `Int.emod` ×1 and `Int.ediv` ×1
(calc's show); `Int.natAbs` as its expression row. The std root's
realizations (`std/*.shard`, dumped) add `Nat.div` and further `Nat.sub`
and `Nat.mod` sites, nothing else: no `Nat.gcd`, `Nat.pow`, bitwise,
`beq`/`ble`, `tdiv`/`tmod` anywhere in calc's or std's realizations. The
tie: the binary (31,160 bytes; the emit 33 s on the bootstrap, the
chain's three steps under 2 s) over the differential's 21 inputs
against the bootstrap driver's run — stdout byte for byte, exit 0 both —
in 1 ms against the driver's 32 s. The two refusal pins through the scratch bridge: `refused_init_list` emits (2 modules, 4 declarations) and its binary prints `hello` as the V3 driver's `--run` does — while the bootstrap's flat read of the file fails (`String.toByteArray: expected a World as the last argument`, the bare-name dispatch's reading of an unknown head), so a naming-law pin's reference is the driver's run, never the flat read, which the tie uses for the kernel-profile pins; `refused_init_prim` emits and ties (exit 0 on both). An underflow case beside them — `(- n 1)` at `Nat` with `n = 0` as the exit code — shows the name map's gap: the V3 driver exits 0 (`Nat.sub` truncates), the probe's binary 255 (the chain's integer `-`), the flat read 255 too; landing 1's expression closes it and the case joins the pins.
What landing 1 builds, by this: the identification and the type map as
the bridge's mechanism, the design's literal rewrite not needed;
`Nat.sub`, `Nat.div`, `Int.tdiv`, `Int.ediv`, `Nat.mod`, `Int.emod` and
`Int.tmod` as `let`-bound expressions — truncation at zero, the
migration table's division by zero (`x / 0 = 0`, `x % 0 = x`) —, the
bitwise five and `beq`/`ble` by name, `Nat.gcd` and `Nat.pow` as emitted
helper functions only if a closure reaches them (none does today);
`BitVec` in the type map; the tie's reference for a naming-law pin the
V3 driver's `--run`, never the bootstrap's flat read.

**As built, landing 1 (2026-10-16): the bridge.** `kernel/emit.shard`
(+106/−19 lines), not a line of the old tree: the identification
(`em_cname`, nine constructors — Init's `List.cons`/`nil`, `Prod.mk`,
`Option.some`/`none`, `Bool.true`/`false`, `Decidable.isTrue`/`isFalse` as
`Cons`/`Nil`, `Pair`, `Some`/`None`, `True`/`False` — in terms, patterns
and the type declarations, the four wire types declared from Init's
inductives under those spellings in the `Init` module and `Decidable`,
`Nat` and `Int` not declared, `em_init_skip`); the type map (`em_type`:
the nine names the chain holds as its integer — `Nat`, `UInt8`–`UInt64`,
`USize`, `Fin`, `Char`, `BitVec` —, `String` and `ByteArray` as `(List
Int)`, `Decidable` as `Bool`, `Subtype` as its base, `Prod` as `Pair`
through `em_tname`); the primitives (`em_prim`: seven rows by name —
the bitwise five, `beq`, `ble`; `em_prim_expr`: seven rows as an
expression, both operands bound once at two fresh depths, the bodies
`em_prim_body`'s — `Nat.sub` truncating, the six divisions through
`em_div_body`'s zero guard, `Nat.div`/`Int.tdiv`/`Int.ediv` zero and
`Nat.mod`/`Int.emod`/`Int.tmod` the dividend at a zero divisor —);
`absurd` as `(match 0)`. The checks: a dotted primitive without a row
refused as before with the message naming the bridge's rows (`Nat.gcd`,
`Nat.pow`: no closure reaches them, so no helper is emitted), `absurd`
allowed, the literal check through the identification. The two pins
flipped: `init_list` (the refused case as it was, tied now: `hello`) and
`init_prim` (new, 21 rows — `Nat.sub` at an underflow and plain,
`Nat.div` and `Nat.mod` by zero and plain, the four `Int` divisions on
negative operands and by zero, the bitwise five, `beq`, `ble` — each
row silent when the naming law's result holds and its name printed
otherwise, `rows done` last; the driver under `ev` prints the last line
only, its entries being K's, and the binary the same). The tie: a case
at the root is loaded with Init and its reference is the driver's
`--run`, the flat read staying the kernel-profile cases'; step (2b)
builds calc's main through `v3/build.sh` and ties it to the driver's
run over the differential's 21 inputs on stdout, stderr and the exit
status. `bin_test.sh` (3d) builds the same binary and ties it to the
program's run through the bin, four checks more (46). The pins' README
rows, the kernel README's emitter row, the two scripts' headers.
*Departures from the design, stated:* rule 2's literal rewrite replaced
by the identification (landing 0's finding); the `Nat.gcd`/`Nat.pow`
helpers not built; the type map one row longer (`BitVec`); rule 3's "run
under both engines" read, for a naming-law case, as the driver's run and
the binary. *Found on the way:* one `)` too many in the emitter's new
text, caught by the escape-aware paren scan before any run;
`(import Init NAME)` takes one name per form (`bad_import` otherwise);
the driver's `--run` writes the program's output alone to stdout and
nothing to stderr, so the tie's compare needs no normalization for it.
Gates on the quiet dev box: the kernel-side files clean under the V3 loader (the emitter's module 90 accepted, 0 refused), names at the baseline; `v3/build.sh --all` 29 roots, 29 built, 0 failed, in 2,765 s (two heavy jobs; the loader's chain emit 29 s, lower 79, codegen 285, cc 84 over 8,586 declarations — 11 more, the bridge's); `bin_test.sh` alone 46 checks, 0 failed, in 395 s (calc's binary 35,264 bytes, built in 35 s, run over the 21 inputs in under a second); the tie alone 79 checks, 0 failed, in 535 s (`init_list` and `init_prim` tied; calc's binary against the driver's run, 36 s against 0); the bootstrap suite under `V3_NATIVE=1` 44 entrypoints, 0 failed, in 1,129 s of wall and 6,055 s of entrypoint time (`imp_test` 1,078 s: the two roots loaded and stored in 935 s, K's replay 141 s; `store_test` 768, calc's five leaves 261 and K's replay 38; the tie 615 s with 79 checks; parity byte-identical over 31 closures and 138,344 declarations in 490 s — the 11 more the bridge's; `bin_test` 484, the binary built in 61 s; `tactic_lib_test` 416; `calc_test` 261); the native suite under `V3_NATIVE=1` 44 entrypoints, 0 failed, in 632 s of wall and 1,640 s of entrypoint time (the tie 543 s; `calc_test` 191; parity 178; `imp_test` 153: the roots loaded and stored in 127 s, K's replay 26; `bin_test` 107, the binary built in 37 s; `store_test` 96); `suite_tie.sh` over the two kept runs 44 entrypoints, 0 differ, byte-identical under both engines on every entrypoint. CI's five jobs are the full gate.

## 9. Assumption policy and entries

**Policy** (law §8.1, §3.2): a declaration is accepted only if its
axiom closure (`axioms.shard`) lies within the file's policy. The
default policy is the standard profile — `propext`, `Quot.sound`,
`Classical.choice`. `Init`'s other four axioms (`Lean.trustCompiler`,
`Lean.ofReduceNat`, `Lean.ofReduceBool`, `sorryAx`; the closures of eight of its
constants reach one of them, records §8) are outside it, so a native declaration whose
closure reaches one is refused by policy, its closure printed, under
an identical proposition (T8). `(trusts NAME…)` widens the file's
policy by name; a file that declares an `axiom` must trust it. View
parameters (§6.5) are a third class: permitted while a consumer checks
against the view, discharged at link, reported if never discharged.
**Obligations** (§8.4, slice 3.14) are a fourth: a measured
function's decreasing facts `f.dec_N`, admitted so its definition
exists, named by `PENDING f measure` and by every dependent's
`params=`, discharged by a proof when I exists — never by a policy.

**A bin's policy (slice 4.3 landing 1; §4's `bin` row).** A bin's
`trusts` names the axioms its requirements and its closure's
definitions may rest on beyond the standard profile; the load's
parameters — a view's, a measured function's pending descent — are
permitted as the file's policy permits them (the manifest's `param`
lines carry them), so calc's bin rests on calc's three pending descents
through the parameters and on its nine bolts through its trusts. The
loader's record is `BIN NAME entry F externs N trusts N requires
MET/TOTAL`; the store's manifest gains `bin NAME entry F externs E…
requires R…`; the release gate prints `BIN NAME ok` when the entry is an
accepted def of the store, every extern an accepted opaque (its L
constant) and every requirement an accepted theorem whose assumption
closure rests on the export's axioms, the parameters and the verified
discharges alone, else `BAD-BIN NAME: REASON`, counted with the
conditional declarations; the threading verdict is the loader's, an E
property K does not see, re-derived by every load of the file.

**Entries** (law §9.3): every way a value from outside enters an E
program is **checked** or **preconditioned**. At phase 2 the entries
are the driver's — command-line arguments and file contents arrive as
raw byte lists through the World externs and are validated against the
entry function's E signature before `ev` is invoked (an `Int`
parameter parses or the entry refuses with the argument's origin; a
`(List Int)` is bytes) — and a `realize` body's parameters, whose
precondition is the L type's erased evidence, recorded in the
realization record. "Raw versus checked arguments" (T1) is one fixture
on each: a malformed argument refused at a checked entry with an
artifact origin and no fabricated file span, and a preconditioned call
inside E carrying no runtime proof.

**The checked entry as built (slice 7, 2026-09-13; §13 item 29).**
`run_prog`'s entry takes the World **last**; every parameter before it
is a checked entry, and the driver's `-- ARG…` are validated against
them in order before `ev` is invoked. `Int` parses a decimal with an
optional leading `-`; `Nat` parses a decimal (a sign is refused); a
parameter whose type is a **byte-list codec by identity** — the
prelude's `(List Int)`, Init's `(List Nat)` or `(List Int)`, and since
slice 3.18 `ByteArray`, whose representation is Init's `List` cells —
takes
the argument's bytes as that list, each byte 0–255 an element (slice
10; GPT-6 R59: before, any two-constructor type with a nullary first
and a binary second constructor took them, so a `(type Tree (Empty)
(Branch Tree Tree))` entry received `Branch` cells over integers — a
value of no type, stuck at the first match on it; pin `entry_shape`,
`entry_test`); any other parameter type is refused at the entry
(`bad_entry`: no command-line argument supplies it), as is an entry
whose last parameter is not a nullary-constructor type. Successful
decoding establishes the codec's type and nothing more. **The World's
identity is a stated Stage-0 limit:** the last parameter's type is
taken as the World and its first constructor built over zero fields,
whatever the type; the handler contract that names the World type and
the capability behind it is phase 4's (`bin`, law §4.7; §11). The count must match (`argument_count`)
once an entry has a checked parameter at all; an entry with the World
alone takes any argument list, raw. A malformed argument is refused as
`RunArg POSITION REASON TEXT` —
`not_an_int`, `not_a_nat` — its origin the argument's 1-based position
on the command line and its text, never a file span; the driver
prints `RUN: argument N REASON: TEXT` and exits 6. An entry with the
World alone — the T0 driver's, with its `-a FIX` — reads the raw
argument list through `get_args` and validates it itself: that is the entry
declaring its own precondition, the other of §9.3's two kinds, and the
raw list stays available to every entry. The preconditioned half of
T1's fixture is a `realize` body's erased binders (§7.5: `BErased`
carries no runtime value; the `REALIZE` record). Pins `entry` (the
profile: `Int`, `(List Int)`, the World, `get_args`) and `entry_s` (S:
`Nat`, Init's `(List Nat)`); `kernel/test/entry_test.shard`.

## 10. Conformance at phase 2 (records §4.1 B10)

Four suites, agreed before any result is read:

1. **Frontend parity.** The V3 reader over `v3/kernel/*.shard` and the
   Rust loader over the same files produce the same `Prog`, compared as
   one canonical text: the reader prints its `Prog`; the bootstrap
   gains a `dump` mode that prints its `Module` in the same text.
   Byte-identical over the toolchain's closure retires TCB bring-up
   item (2), the Rust loader's parsing role. **As built (slice 6,
   2026-09-13; §13 item 25).** The canonical text is one line per
   declaration, the lines sorted bytewise: `type NAME n (CTOR T…)…`,
   `fn NAME n (T…) T BODY`, `extern NAME n (T…) T`, with `n` the
   type-parameter count and every name its last component (a
   primitive's its full spelling); types `Int`, `Symbol`, `(NAME T…)`
   and `?i` for the i-th type parameter, numbered by the parameterized
   head first and then by first occurrence across the binders and the
   result; terms `#i` for a bound variable, integers in decimal, `'x`
   for a symbol, `(NAME E…)` for a constructor, a call, a primitive or
   an extern alike, `(match E (PAT E)…)` with `_` for a variable
   pattern, `(let (E…) E)` at the **sequential** indices — the
   bootstrap binds in parallel and its dump shifts each right-hand
   side's outer indices, which §5.4's measurement says changes no
   toolchain `let` — and `(if E E E)`; a string and a `(list …)` are
   the constructor chains both loaders build from them. The measure
   clause is outside the text, since the bootstrap drops it at load.
   `kernel/dump.shard` prints a `Prog`, `load.shard --dump FILE` after
   a clean load; the bootstrap's `eval dump FILE` prints the same
   closure (`rust_bootstrap/src/dump.rs`). `kernel/test/parity_test.sh`
   runs both over every toolchain entrypoint — the driver, the T0
   driver, the calc harness, every test — and compares: **byte-identical
   over 18 closures, 61,080 declaration lines, in 37 s (2026-09-13)**,
   after one finding — the driver's own closure had never been
   classified, and `rz_open_ctor`'s wildcard arm sat one match too deep
   (a real non-exhaustive match the bootstrap tolerated because its
   callers pass constructors only; fixed). Green, it retires TCB
   bring-up item 2 (`docs/TCB.md`, the V3 roster): the Rust loader
   still parses the toolchain for route 3, and the V3 reader's
   agreement is the gate that keeps it honest, as route 1's byte-tie
   keeps compiled K. **The projection's scope (slice 9; GPT-6 R53;
   §13 item 33).** The text is a projection: a name prints as its last
   component and a constructor, a call and an extern print alike, so
   byte agreement is evidence exactly where the projection is
   injective. `parity_test.sh` checks each closure for one declaration
   per short name among the heads (`fn`, `extern`, `sig`), among the
   types and among the constructors, and no constructor named like a
   head, and fails a closure that is not before comparing its dumps —
   the first sweep found one twin, `take_line` in `json.shard` and
   `test/reader_kit.shard` with different bodies, in three closures
   whose dumps carried both lines while every call printed the same
   (the kit's is `first_line` now). Outside the text and stated as
   such: the measure clause (a runtime-only comparison; recursion
   obligations are Stage 1's); a literal's kind is its sign since slice
   3.4 and is in the text. The text is not P, not a store format and
   not the embedding's representation.
2. **Execution parity.** The same `Prog` under `ev` (hosted on route 3)
   and under the Rust evaluator: K's test entrypoints and the T0
   fixture — route 2, K interpreted by `ev` — with byte-identical
   verdict lines — **the fixture's tie landed at slice 5**
   (`kernel/test/route2_test.sh`, §6.7); `examples/calc`'s program half under `ev` against the
   old tree's evaluator on a fixed input set; every primitive's
   positive, negative and boundary cases. **Calc as built (slice 6;
   §13 item 27).** `v3/examples/calc/` is the program half in S, one
   file per old file (the claims a header line each, phase 3): the 51
   functions and 9 types verbatim under Init's `Int`, `List`, `Option`
   and `Bool` — `(import Init Int)` over the 17,812-line fixture
   `kernel/test/fixtures/init_prefix_int.ndjson`, the export through
   the `Int` inductive — the measure proofs reduced to their terms,
   `ediv`/`mod` as `Int.ediv`/`Int.emod`, `append` and `len` as the
   port's own `list.shard` until `v3/std` (phase 3), the pair `drive`
   returns declared in the port because `Prod` lies past the fixture.
   Numerals are `Nat` at Stage 0 and the arithmetic runs on them as
   integers (§12.6's numeral rows). An S program cannot name the wire's
   cells — they are the prelude's, and a file that opens the prelude's
   `List` can no longer name Init's under §3.1's candidate rule — so the
   differential's drivers sit outside the program: `kernel/test/
   calc_harness.shard` loads the package, calls `ev` on each entry with
   values built as data and renders the results in the old tree's
   value syntax; the old side is the tower's expression mode (`eval
   MODULE EXPR`, `kernel/eval.shard`'s evaluator and printer — `eval
   direct`'s flat resolver cannot follow `std/list`'s directory-module
   imports) over `examples/calc/calc_differential.shard`, a pure module
   in the old tree adding only the derived inputs (LAYOUT: no file
   under `v3/` imports the old tree), driven case by case by
   `kernel/test/calc_test.sh`. Both sides read
   `kernel/test/fixtures/calc_inputs.txt` and produce one line per
   input and case — `lex`, `parse`, `run`, the spec parser's eleven
   functions, `spec_run`, `step` and `step_spec` with the state
   threaded, the trace and world folds, `show`/`valI`, `show_ascii`,
   the digit functions: 34 cases per line and 13 folds — and the script
   compares the two outputs byte for byte. **Byte-identical over 21
   inputs, 727 lines a side (2026-09-13): the port under `ev` in 7 s,
   the old tree in 154 s.** Left out on both sides: `calc_ndigit`'s
   `codes`, shadowed in the old tree's flat closure by
   `calc_show_run`'s (first definition wins), covered by `code`.
   **The primitive suite (slice 7):** `kernel/test/prims_test.shard`
   is one table over every entry of §6.4's table — the 18 profile
   spellings and the 21 naming-law identities — with a positive, a
   negative and a boundary case each, the expected values fixed by
   hand: the guards (the profile's `/`, `mod`, `tmod`, `ediv` at zero;
   a shift by 64; a `Nat` entry on a negative operand — `Nat.sub`
   included since slice 7, which had let a negative through), the
   totalizations (`Int.tdiv x 0 = 0`, `Int.tmod x 0 = x`, `Nat.div`,
   `Nat.mod`, the saturating `Nat.sub`, the total `Nat.shiftLeft` past
   64 bits), the cells (`Bool`, Init's `Bool`, `Decidable`), the
   symbol round trip and the byte guard on `sym_of_chars`, and the
   arithmetic past 64 bits. The test runs the table under `ev` on
   route 3, so each entry is checked against the hand-fixed value
   through the same host arithmetic both routes share. **Its two
   findings (2026-09-13):** the host's refusal of a primitive call is
   not a stuck but a fatal error (the bootstrap falls through to its
   effect handler), so every guard is `ev`'s to check first —
   `Nat.pow x 0` divided its size estimate by the exponent through a
   strict `bool_and` and died, and `sym_of_chars` guarded "bytes"
   where the host decodes UTF-8, so a lone 255 died too; now
   `utf8_ok`, and both are stuck. The suite's 120 cases are green.
3. **Checker parity.** Phase 1's byte-tie of routes 1 and 3, carried
   unchanged.
4. **Independent pins.** Expected outputs fixed by hand, never
   generated by either engine under comparison — the `t0_expected.txt`
   precedent.

Wall-clock, memory and allocation counts are measured and recorded,
never compared as verdicts.

## 11. Deferred, by phase

| capability | law | phase |
|---|---|---|
| implicit arguments, first-order unification, universe inference, the numeral rule of §5.2, coercion `Nat → Int` | §5.1 Stage 1, §5.2 | 3 — **landed slice 3.15** (§5.1, §8.4) |
| match compilation, structural recursion to recursors, `f.eq_N`, `noConfusion`, `WellFounded.fix` from `measure`, `fn` = `def` + `realize` | §5.1 Stage 1, §4.5; §8.4 | 3 — slices 3.13 (immediate-field recursion, `eq_N`), 3.14 (course-of-values, `WellFounded.fix`), 3.15 (`casesOn`, `noConfusion`, `c.inj` for a type without parameters; the parametric `HEq` shape later) |
| deriving under a declared policy | §5.1 | 3 — **landed slice 3.19** (§8.4): a type without parameters and Init's `List`, `Option`, `Prod` at closed arguments; types with parameters, nested and mutual types, the ordering's laws and the instance constants later |
| tactic blocks, the I elaborator, the goal graph, `sorry` as a hole | §5.1 Stage 2, §7 | 3 — **landed slices 3.20–3.22** (§8.4): `(by STEP…)` as I, `goal_of`, `applicable`, `step` and `elaborate` as E functions over one derivation, `sorry` the pending hole (3.20 rule 9), the producers (3.22); the law's `change` is `show`; holes shared across goals and incremental goal states are phase 4's (T4, T6) — slice 3.30's ledger |
| typeclasses, instances, coercions | §5.1 Stage 3 | 3 — instance arguments at the library's types from a fixed table at slice 3.27 landing 1 (§8.4 rule 3, designed 2026-10-09), the `Nat → Int` coercion at 3.15; search stays the door — the phase-3 form by item 61's ruling (slice 3.30's ledger) |
| the `Init` import with E realizations attached; `String`, `Array`, `ByteArray` representations (Init's E-eligible inductives are E types since slice 5b, §7.5; a `realize` attaches a body per constant) — **the byte-list representations landed at slice 3.18** (§8.4: `String`, `ByteArray`, `Array`, `UInt8`, `BitVec`, `Fin`); the packed buffers are the lowering's, behind the same types | §4.4, INVENTORY | 3 — **landed** (slices 5b, 3.18, 3.26: the on-demand load, a module's horizon the only measure) |
| lambda lifting, templates, specialization | §4.3 | 3 — **landed slice 3.28** (templates by shape, lifted lambdas, static packages, the budget; T2 and T3); specialization over user files is route 1's at phase 4 |
| `bin`, `requires`, the World-use check, effect traces | §4.7 | 4 — **slice 4.3 (designed 2026-10-16, §8.6)**: the check over `Prog` with the law's fixtures as pins, the extern an opaque L constant, the `bin` form and its record, the trace tied four ways |
| prepared handles, long-lived environments | §9.3, T6 | 4 |
| evaluation reflection: `ev`'s theorem, the `rfl` node | §4.4, T8 | 4 |
| the canonical S form: CANON's rule set rewritten for S — **landed slice 8, `v3/CANON.md`**; its recognizer, formatter and gate | §5.1 "One canonical S", §10.5 | the rule set 2 (done); the gate 6 |
| T1's branch-local proof: a `dite` whose `h` is used only in a `Fin.mk` field — `Fin n` over erased bounds needs a **value** parameter at E, which §7.5's eligibility rule (types and propositions only) does not admit; `Fin`, `dite` and `Nat.decLt` are all inside the `Int` fixture, so the export is not what blocks it | law §4.1, §4.2 | 3 — **landed slice 3.24** (`(dif h C T F)`, the `Fin.mk` field under its hypothesis; pins `dif_ok`, `dif_type`, `dif_needs_init`, `dif_scope`; `examples/path`, `path_test.sh`) |
| T5's "two validated instances of one interface" — §6.6 binds one implementation per view directory (§13 item 14) | law §8.2, T5 | **carried to phase 5** (ruled 2026-10-13, slice 3.30 rule 3a): built as a landing when the port's first consumer needs two implementations of one view — a loader rule naming the implementing file and R57's checked-instance record by substitution; the mechanism for one implementation is built (§6.6) |
| T5's "an imported theorem about the original still usable after a realization" with an **imported** theorem — every theorem about a realizable constant lies past the `Int` fixture (`ite_self` at export line 18,116); the native form of the fixture is pinned (`realize_theorem`) | law §4.4, T5 | 3 — **landed slice 3.30 landing 1**: pin `realize_imported` (Init's `List.append_nil` cited before and after a supplied realization of `List.append`, under the on-demand load) |
| `CheckedEnv` sealed: K one directory module behind a view (§6.6); complete when the first `meta/` consumer is behind it (§13 item 26, R52) | §3.5, §8.2 | 3, the opener |
| the Stage-1 authoring facilities ordinary source needs before the broad port (GPT-6 R60): list literals with expected-type-driven empty lists; a negative-numeral rule with no silent `Int → Nat`; named-field construction and update, a changed dependent field an explicit obligation, never hidden by the sugar; a `Name` literal or construction that forges no declaration, node or environment identity (§3.3); a scoped fresh-name supply in ordinary code where `gen_fresh` was; the public byte and text adapters (`ByteArray` for raw bytes, `String` for text, the prelude's cells an internal adapter) decided with the first host-facing S library — each with a first example before the port pays for its absence | §5.1 Stage 1; §12.6 | 3 — **landed**: list literals and the negative numeral (3.15–3.16), `make`/`with` and the fresh-name supply (3.17), the byte and text adapters (3.18); `Name` literals and symbols **deferred** to the toolchain's port (item 51; 3.18 rule 5: `(quote x)` keeps `no_l_identity route=e_first`; corrected at slice 4.0 landing 0, R81) |
| the checked module instance (GPT-6 R57): the substitution from view parameters to implementation declarations and evidence recorded, the `fulfills` proofs' assumptions inherited by the instantiated consumer, a stricter policy refusing the result, one consumer proof used with two implementations without cloning it, revisions bound so spellings alone mix nothing | law §8.2, T5; §6.5 | **carried to phase 5** with the two-instance gate (ruled 2026-10-13, slice 3.30 rule 3a) |
| programmatic validation (GPT-6 R61): a client that builds a `Prog` or declarations as data validates them through the public services without a source round trip — item 15's fusion is an implementation choice, the dump of §10 a conformance format, neither the representation | §9.3, T6 | 4 |
| the one E (§8, RULED 2026-09-14): the bootstrap to the whole of E, the toolchain migrated by tool, the profile flag deleted — slices 3.2–3.5 | §8.3; law §9.2 as amended | 3, first |
| `Symbol`'s L identity `String` and the flip of `"…"` from the byte list to a `String` value in E, for every file at once, the toolchain migrated by tool | §8.1 rules 1–2; INVENTORY | **carried to phase 4** (ruled 2026-10-13, slice 3.30 rule 3b): with route 1's chain, as slice 3.18 stated |
| the three persisted identities (law §5.2, §7.2, §7.5; R82, records §4.12): the resolved requirement — the statement with its referenced declarations' identities, selected instances and coercions, policy and environment dependencies —, the reconstruction contract with its version, retained P under the canonical encoding with display names outside it; today the sidecar's key is the statement fingerprint and the store's sharing compares binder names | §8.4 3.22 rule 2 | 4, its own slice before T6's prepared handles and before phase 5's tools police frozen requirements |
| occurrence provenance (law §4 "provenance is structural and occurrence-aware"; R85, records §4.12): occurrence identifiers and an origin map across parsing, record expansion, generated obligations and I nodes, K's terms location-free, a diagnostic following the occurrence; today `SExpr` carries no span after a parse and a tactic failure names its step and goal | §4; §8.4 3.20 | 4, before the incremental goal-state slice; slice 4.1's emitter keeps the map |
| user notation | §5.3 departure 6 | never in v1 |
| an existential's witness introduction through `exact` — `(exact (Exists.intro w h))` with `h` a hypothesis of known type, or the `@` form with the predicate written, is `unify_stuck … blocked on p`: the first-order unifier meets the predicate applied to the witness before the result type meets the goal (slice 4.2 landing 2, §8.6 item 2); the form that works is `(apply (Exists.intro w) ((show P) …))` | §5.1 Stage 1 | 5 — a Stage-1 item: the goal's result type unified first, or a pattern case for a metavariable applied to a witness |

## 12. Changes from v2 — the compatibility ledger

v2 is today's tree (`docs/LANGUAGE.md` and the layered systems above
it). This section lists every v2 feature with its fate, so that
compatibility breaks are decided rather than discovered and nothing is
lost by omission. Fates: **carried** (same form, same meaning);
**re-spelled** (same capability, a different form — the migration tool's
job, law §12.3); **changed** (the meaning differs); **deferred** (not at
phase 2; the phase named); **dropped** (deliberately, the replacement
named); **AT RISK** (nothing in the law or this draft provides it yet —
the rows to watch). Counts are from the tree at `5b9cc0c`. The
migration table of law §10.3 owns the name and behavior changes of the
*operations*; this ledger owns the *forms and semantics*.

### 12.1 The object language

| v2 | fate | where / note |
|---|---|---|
| `(type (NAME T…) (CTOR F…)…)` | carried | §4; constructors gain the identity `NAME.CTOR`; a bare constructor citation needs the declaring file or a `use` of the type's namespace (§13 item 7: `(use m.Exp)` for `Num`); a constructor bearing its type's name — `(type World (World Int))` — resolves to the type in an E-type position and to the constructor elsewhere (§13 item 28, slice 6). Since slice 5b `Init`'s E-eligible inductives (`List`, `Option`, `Bool`, `Decidable`, `Array`, …) are E types with their constructors in S (§7.5); `Nat.succ` is refused with the pointer to numerals |
| `(fn NAME PARAMS RET BODY)` | carried as E; **changed** | §0: no L meaning at phase 2 — in v2 a `fn` was also the object of `unfold`/`simp` in proofs; restored at phase 3 |
| polymorphic head `(fn (append T) …)`; bare type variables in binders auto-bound (`(xs (List T))`) | re-spelled | explicit `((T Type) …)` binders everywhere — law §5.3 departure (4); the profile kept both v2 spellings until the one-E ruling (§8, 2026-09-14): the toolchain migrates by tool at phase 3 |
| `(extern NAME PARAMS RET)`, polymorphic externs | carried | §4; the roster is the host's (§12.4) |
| return types unchecked (no load-time typing) | **changed at slice 3.16** | a `fn` whose signature has an L reading is elaborated against it (§8.4 slice 3.16 rule 6): an ill-typed body is a refusal (`type_mismatch`), never an E-first success. A `fn` whose signature has no L reading — the toolchain's own sources, until they port — is still the classifier's, unchanged |
| `if` on `True`/`False` by constructor name | carried, generalized | §6.2's tag rule; v2's `(type Bool (False) (True))` has Init's constructor order |
| `match`: first match wins, nested patterns, integer and `(quote S)` patterns, `_`, bare 0-ary constructors | carried in E | symbol patterns profile only; Stage 1's match compilation must keep first-match semantics (Lean's does); **in L since slice 3.16** a `match` is a generated matcher (§8.4 slice 3.16 rule 1): first match wins, nested patterns, `_` and bare field-less constructors carried; an integer or `(quote S)` pattern is `literal_pattern` there until a consumer under the naming law has one |
| parallel `let`, no `let*` | **changed**: sequential in L and E (RULED 2026-09-12, R44) | §5.4; 0 of the tree's 30,611 `let` groups depend on parallel binding, so no source changes meaning; the bootstrap evaluator's parallel rule gives identical results on all of them until the V3 reader replaces it (slice 2) |
| `(quote S)`, `'S`, the `Symbol` type, `sym_eq`, `sym_of_chars`, `chars_of_sym` | carried — **decided 2026-09-14** (§8.1 rule 1) | `Symbol` is a built-in E type in every file, the interned atom; its L identity is **deferred to the toolchain's port** (§8.4 slice 3.18 rule 5, item 36 as amended: `String` is a byte list since that slice, so the atom is not its representation); K's `Name` values are built from its atoms as today. The S-side refusal `symbol_literal` retired at slice 3.4 (2026-09-14) |
| `(list a b c)` (9,455 uses outside `v3/`) | carried — **decided 2026-09-14** (§8.1 rule 2) | the constructor chain of the `List` in scope, by the scope at Stage 0 and by the expected type at Stage 1 (R60); the S-side refusal `list_sugar` retired at slice 3.4 (2026-09-14) |
| `"…"` = UTF-8 bytes as `(List Int)`, on the extern wire too | carried in E — **decided 2026-09-14** (§8.1 rule 2); **changed** at `String`'s realization | in an E body the byte list of the `List` in scope for every file (the S-side refusal `string_literal` retired at slice 3.4, 2026-09-14); in L positions K's `String` literal (§5.3). **Changed at slice 3.18** (§8.4 slice 3.18 rules 1, 3 and 4; decided 2026-10-01): outside the toolchain's own sources `"…"` is a `String` in every position, represented by its UTF-8 bytes as `Init`'s `List` cells, and a v2 program that built text as `(List Int)` converts through `v3/std/bytes.shard`; the toolchain's sources keep the byte list until route 1's chain is V3's own, then migrate by tool. The wire under the naming law is `ByteArray`; the toolchain's signatures keep the prelude's cells (§6.7) |
| `Int` numerals everywhere, `-7` | carried in E — **decided 2026-09-14** (§8.1 rule 2); in L since slice 3.15 (§5.3) | in an E body a numeral is an integer of any sign whose type is the binder's; in L a numeral takes the type its position expects — `Int.ofNat n` / `Int.negSucc (-n-1)` at `Int`, `Nat` otherwise (law §5.2's rule) |
| unbound identifier = `FVar` (proof-time opened variables) | dropped | an unbound name is a resolution error; K refuses free variables; I's named context replaces the use (phase 3) |
| primitive dispatch by name, trie-first, bodyless-name collision = stuck (`pins/lang/prim_shadow_rejects`) | **changed** — landed slice 5 | heads classified at load into four node kinds; a primitive is an identity (§6.4); a declared name shadows the table's (the scope resolves first); an unknown head is refused at load |
| the primitive table | carried under the profile names; **changed** under the naming-law names | §6.4; `/` stuck at zero versus `Int.tdiv` total are two entries |
| `gen_fresh`, the one effectful primitive | **dropped** (slice 5) | law §4.7 has no effectful primitives: pure `ev` refuses reachable externs. No V3 file calls it and the table does not carry it; a ported source that needs fresh names threads a supply — `v3/std/fresh.shard` since slice 3.17: `Fresh.next` returns the name and the advanced supply, two successive names differ (ten old-tree kernel files: types, canon, sequent, reduce, tactics, proof_reader, proof, trace, the two lowered twins) |
| `Nat` former: `Z`/`S` packed to literals, patterns match literals by view, proof-facing normalizers never pack, bare literals do not type as `Nat` | **changed** | `Nat` is Init's; K's literal rules (offset, `Nat.zero` ≡ `0`, accelerators) replace the former; numerals type as `Nat` by rule (§5.3). The profile keeps the prelude's `Nat` for measures; `ev` does not carry the bootstrap's `Z`/`S` view over integer literals (no V3 file matches on them; a literal matches `Z` never) |
| `(refine BASE PRED)`, `refine_val`, `refine_try`, `refine-fact`, `(returns …)` (37 types, 43 `refine-fact` sites) | re-spelled; deferred | `Subtype` over any `Prop` (law §4.1): `refine_val` → `Subtype.val`, `refine_try` → a `decide`-bridged constructor, `refine-fact` → `Subtype.property`; the return obligation is a Stage-1 elaboration obligation. Phase 3 (REFINEMENT.md superseded) |
| `(record …)`, `make`, `with`, `F_of`/`with_F`, the six-law family, `NAME_eta` (21 files, 237 `with_F` sites) | re-spelled; **AT RISK** | `structure` with projections (§4): `F_of` → `NAME.F`; `NAME_eta` is K's structure eta for free; the laws are `rfl`. **No Stage-0 form for `with_F` updaters or order-free `make`**: Lean's `{ s with f := v }` is elaborator sugar — Stage 1 must add it or every update site spells the constructor. **Carried since:** `record`/`make`/`with` at slice 3.9 (item 42); at slice 3.17 the laws are `Eq.refl` with no generator (pin `record_laws`) and `make`/`with` reach a parameterless `structure`, a dependent field kept across an update refused (`dependent_update`) |
| `std/word` (`U8`…`I32`, opaque), `std/bytes`, `std/str` | re-spelled | `UInt*`/`BitVec`, `ByteArray`, `String` (law §10.3, INVENTORY), phase 3–5. **AT RISK:** widths beyond 64 — INVENTORY realizes `BitVec w` only for static `w ≤ 64`; the 2026-09-02 ruling kept `std/word`'s unused widths as a facility |
| `S^`, `inline`, `chain` (2,733 / 1,851 / 320 uses) | dropped with the v2 proof language | their reason — claim statements must be literal spellings that match CBV residues — does not survive: I's `rw`/`simp_only` match terms, and Nat towers are K literals. Phase 3 confirms; **AT RISK** if some I form still needs a literal tower |
| `(import "x.shard")` opens the file's names (flat scope) | **changed** | `import` never opens, `use` does (§3.1) — for every file since the one-E ruling (§8.1 rule 4, 2026-09-14): the toolchain's files gain their `use` lines by tool at slice 3.3; the bootstrap keeps resolving flat and parity is the tie |
| `(use (:: m *))`, selective `(use (:: m name))` (1,768 selective sites) | re-spelled | `(use m)` and `(use m name…)` (§3.1) |
| `use-module` (2 uses) | dropped | vestigial |

### 12.2 Modules and interfaces

| v2 | fate | note |
|---|---|---|
| `mod.req.shard`, the `mod.req/` dir form, `sig fn`, `sig type` (private constructors), `requirement`/`fulfills`, requirements granted to consumers, structural opacity per closure, mode-aware resolution (check = interface, run = implementation), canonical closure dedup | carried | §6.5 — with one change: a `sig fn`/`sig type`/`requirement` is a **view parameter**, an axiom-kind constant to K and a policy class, so evidence binding is checked on closures rather than by the loader's granted flag |
| the req-scope gate (an interface file imports only interfaces and the kernel) | carried | §6.5 |
| back-compat shims (`std/nat.shard` → `(import "nat")`) | dropped | V3's files are new |
| `(bin …)` (16), `trusts`, `requires` | **landed** (slice 4.3 landing 1, 2026-10-16) | `(bin NAME (entry F) (externs E…) (trusts A…) (requires R…))` (§4), the first `examples/calc/calc_bin.shard`; `trusts` the axiom policy since phase 2 (§9) |
| `(app …)`, `(cli …)` (recognized by the reader; no uses in the tree) | dropped | the MVU driver is phase 4's `bin` story |
| `(lib …)` (4 uses, `tools/lowcheck` fixtures) | **AT RISK** | the lib-build arc's form; the law names the lowering-side toolchain "new code where toolchain" (MANIFEST) and never mentions `lib`; a phase-5 decision |

### 12.3 The proof language

Everything here becomes I (law §7) at phase 3; the v2 proof DSL is
the re-spelling tier of the port, not a stage (law §5.1). The rows
say what each v2 form most plausibly becomes, so the phase-3 roster
can be checked against them; "phase 3 decides" is implicit in every
row.

| v2 | becomes | note |
|---|---|---|
| `(claim NAME (binders) GOAL PROOF)` with `(goal (premises) concl)` sequents, named hypotheses, type parameters inferred from binders | `theorem` with explicit binders, premises as `->` or named binders | conclusions were equations between object terms; V3 admits any `Prop` (law §10.2: "Bool predicate → proposition plus decision") |
| `axiom` (49; kernel, app and bin scopes only) | `axiom` under `trusts` | §9 |
| `refl` | `rfl` | |
| `steps` with `reduce` / `simp` / `simp (stop …)` / `compute` / `compute (stop …)` / `unfold` / `rewrite … OCC` / `inspect` | `simp_only`, `unfold`, `rw` with a guarded occurrence path, `decide`/`reflect` for ground computation; `inspect` → `goal_of` renderers | `compute` becomes evaluation reflection (§6.2, phase 4), not a rewrite |
| `induct`, `case-on` | `induction`, `cases` | |
| `wf-induct MEASURE`, `subterm-induct`, `(below)` | `wf` over `WellFounded` / `sizeOf` | **AT RISK:** the strong IH citable at any proper subterm with `(below)` discharging the `⊰` premise syntactically — Lean has `sizeOf`-based termination; whether I keeps a subterm-order rule is a phase-3 decision |
| `have`, named `have`, `(premise NAME)`, `(hyp K)`, `(lemma NAME)` | `have`, the named context, `exact` | |
| `fin-split VAR LO HI` | `decide` over a bounded `Fin`/interval, or `omega` | **AT RISK:** bounded enumeration as one primitive step |
| `div-facts TERM D Q` | `arith`: the quotient and the remainder by a positive literal bring their rows (slice 3.21, §8.4 rule 4); `omega` for what needs a cut (owns the `Nat`/`Int` seam, law §5.2) | |
| `inject`, `absurd` | `injection`, `noConfusion`/`absurd` | `T.noConfusion` and `T.c.inj` generated after a native inductive without parameters (slice 3.15, §8.4 rule 5); `injection` the tactic is I's |
| `rewrite-with EQREF DIR SIDE (INST…) (PROOF…)` | `rw` carrying lemma identity, instantiation, direction, guarded occurrence path | law §7.2 |
| `(by arith (list …))` — tautology, Farkas certificate (5,105 uses) | `arith` with the checked certificate as I data; `omega` as the producer | law §7.2 lists `arith`; slice 3.21: `(arith FACT… (farkas K…))` over the goal negated, the context's linear hypotheses and the facts — the old list's order but for the context's; without the clause the weights are reconstructed |
| `refine-fact` | `Subtype.property` | |
| `(admit)` truncation | `sorry`, reported loudly, never accepted | law §7.5 |
| `.auto.shard` sidecars, `(proof-for NAME PROOF)` (1,495), the `auto` delegation | the sidecar and the pin store keyed by the resolved requirement; engine-authored I | law §7.5; T8's stale-pin rule |
| the tracer, focus mode, `tools/explain` | renderers of goal states | law §7.3 |
| corpus pins (`pins/`, `pin_run`, `fails-base.txt`, the tiers) | `v3/pins`, the hostile battery first | phase 5; `run_corpus.sh` never reaches `v3/` (phase 1 header) |

### 12.4 Semantics and the host

| v2 | fate | note |
|---|---|---|
| division by zero stuck | **changed** under the naming law: `x / 0 = 0`, `x % 0 = x` | law §10.4; the profile's `/` stays stuck (§6.4) |
| bitwise ops on `Int` premised `0 ≤` | re-spelled | `Nat.land` … (law §10.3) |
| sizes and indices `Int` | **changed** | `Nat` where a size, saturating subtraction — the migration tool's typed class, reviewed per use (law §10.2) |
| `int_eq`/`lt`/`le` return `Bool` | re-spelled | `Decidable` propositions with `decide` bridges; `==` for `Bool` values. E's names are `= < <=` since slice 3.17: the old spellings are `renamed_primitive` outside the toolchain's own sources, which migrate with route 1's chain (`v3/tools/rename_cmp.py`) |
| World threading with no use check | **changed at phase 4** | the well-threadedness check (law §4.7): a v2 program that uses one World token twice will be refused; the World-alias fixture lands before any effectful port |
| the extern roster `get_args read_file read_dir read_key write write_file write_line exit` | partly carried | `kernel/host.shard` declares six; `read_dir` (the old loader, codegen) and `read_key` (snake) return when a V3 consumer needs them. the wire under the naming law is `v3/std/host.shard`'s, over `ByteArray` (slice 3.18; decided 2026-10-01) |
| evaluator errors `NoMatchArm`, `IfNonBool`, `UnknownCall`; no fuel | **changed** — landed slice 5 | `EvStuck` with a reason (`no_arm`, `if_tag`, `guard`, `extern`, `unlinked`) and the function; `EvOut` is new — v2 relied on the totality gate instead of fuel (§6.2); `EvExhausted` (`nat_size`, `nat_count`) since slice 9 — a primitive's resource, distinct from a guard (R51); an unknown call is refused at load, never reached |
| `(measure (struct x))` (3,175), `(measure (- …))` (29), size-function measures (about 20) | carried as `ERec` | the obligation: v2's measure gate and the offline `admit` classifier on every checked `fn` → law §4.5's tactic-discharged obligations (phase 3). **AT RISK during phase 2:** no totality check on any `fn` — exactly `eval direct`'s situation today, and only for the duration of Stage 0 |
| mutual recursion (the measure gate's SCCs) | deferred | "mutually recursive groups need a joint well-founded argument" (law §4.4), Stage 1 |
| a user `(type Nat …)` shadows the core one | carried by identity | two `Nat`s are two identities; no shadowing rule is needed |

### 12.5 Tooling surfaces

| v2 | fate | note |
|---|---|---|
| `bin/check`, `bin/shard_check`, `bin/shard_eval`, `eval direct` | the V3 driver (`kernel/load.shard`, slice 3) replaces `check`; `eval direct` stays the bootstrap's; the compiled chain is route 1 | law §9.1; `tools/lower` ARCHIVE at the flip |
| shardfmt and CANON's rule set | rewritten for S — `v3/CANON.md` (slice 8, 2026-09-13) | law §10.5; the fmt gate on V3 at phase 6 (`v3/CANON.md` §8) |
| `tools/digest`, `explain`, `prove`, `search` | new code onto law §7.3 | MANIFEST |
| `tools/zed-shard`, `shard-viewer` keyword lists | the V3 keywords (§4) | law §10.5, phase 2 |

### 12.6 The AT RISK rows: owner, consumer, regression (R47, 2026-09-12)

Each row above marked AT RISK, with who decides it, which consumer
first needs it, the small regression that shows the gap or the fix,
and the disposition this draft intends.

| row | owner | first consumer | regression | intended disposition |
|---|---|---|---|---|
| symbols in S (`quote`, `Symbol`, `sym_eq`) | phase 3, Stage 1 (the reader) | the toolchain's own sources (ten kernel files) when they port to L | a `fn` using `(quote x)` and `sym_eq` under the V3 reader is refused with a named reason until decided | a `Name` literal, or symbols stay profile-only — **deferred at slice 3.18** (rule 5): E-only until the toolchain ports; the direction a type of its own over `String`, represented by the atom |
| `(list a b c)` | phase 3, Stage 1 | every ported `fn` (9,455 sites); calc's program half, ported to S at slice 6, spells its list literals as constructor chains | `(list 1 2)` under the V3 reader refused by name | a list literal at Stage 1 |
| the extern wire's bytes | slice 5 (`ev`'s extern boundary — **bytes as the prelude's cells, landed**); phase 3 for the L type | `sha256sum`'s bin; calc's app step — an S program names no wire cell at phase 2, so calc's differential keeps its drivers outside the program (slice 6, §10) | `write_line` of a literal round-trips its bytes through the driver (route 2's byte-tie) | bytes at phase 2; **decided 2026-10-01: `ByteArray`**, represented by `Init`'s `List` cells, each extern read by its declared type (slice 3.18 rule 4; `kernel/test/wire_test.sh`, `calc_main_test.sh`) |
| negative numerals | phase 3, Stage 1 | calc (negative `Int` results); `std/div` | `-7` under the V3 reader = the constructor term at Stage 0; calc's port (slice 6) writes no negative literal and computes its negative results from `-` at run time, its `Nat` numerals running as integers | Stage 1 special-cases `-` on a numeral; ruled then |
| `gen_fresh` | **decided slice 5: dropped** | the ten old-tree kernel files (canon, tactics) as they port | a `fn` citing `gen_fresh` is refused at load (`unknown_head`) | a threaded supply in the ported toolchain (`v3/std/fresh.shard`, slice 3.17) |
| `with_F` updaters, order-free `make` | phase 3, Stage 1 | 237 sites (`models/imp`, the tools) | `(with_F s v)` refused by name until the update form exists | Stage 1 record-update sugar (Lean's `{ s with f := v }`) |
| `std/word` widths beyond 64 | phase 5 (the word/float line) | none today (the 2026-09-02 ruling kept the widths as a facility) | INVENTORY's static `w ≤ 64` bound on `BitVec w` | static `w ≤ 64` unless a consumer appears |
| `S^`, `inline`, `chain` | phase 3 (I) | the PORT theorem corpus | a claim whose statement needs a literal tower under I's `rw` | dropped; phase 3 confirms |
| `(lib …)` | phase 5 | `tools/lowcheck`'s fixtures (4 uses) | the fixtures under the profile | decided with the lowering-side toolchain |
| `subterm-induct`/`(below)`, `fin-split` | phase 3 (I steps) | the regenerated certificate kits; the std proofs that use them | one theorem each: `tb_len`'s strong induction; one bounded enumeration | `wf` over `sizeOf` and `decide`/`omega`; a subterm rule only if a ported proof needs it |
| no totality check on any `fn` during Stage 0 | phase 3, Stage 1 (law §4.5) | every `fn`; calc's 51 functions, `RUNNABLE` since slice 6 with their measures reduced to terms | R45's self-recursive candidate exhausts and gains no equations (`ev_test`, landed slice 5); a `realize` is checked for structural descent and a looping body refused (`realize_loop`, landed 5b) | measure obligations discharged at Stage 1; the runnable-only status visible in the driver's output meanwhile (`RUNNABLE`, R45, landed); a `realize`'s `(measure E)` a reported obligation (`PENDING … measure`, 5b) |

## 13. For ratification — decisions made here beyond the law's text

The canonical form's own decisions are `v3/CANON.md` §9 (six ruled
2026-09-13, five open); they ratify with these.

1. **Native K names carry the module path** (`std.list.List.sum`);
   imported names do not (`List.length`), the import being their
   identity. Alternative: prefix imports too (`Init.List.length` in K),
   which renames every constant in every imported term at import time
   and changes the accelerator pins' identity hashes. **Clarified
   (GPT-6 R61, slice 10):** the construction is not collision-free;
   a collision is refused, never conflated (§3).
2. **`.{u v}` universe suffix** on declared and cited names, lexed as
   a level list with the `(Sort …)` grammar. Alternative: a separate
   `(@ NAME LEVEL…)` form.
3. **`axiom` and `opaque` added** to the keyword set; `abbrev` as a
   hint spelling. They are K's kinds; the hostile battery and views
   need them.
4. **`NAME.realize_N`** for a supplied realization's equations (§7.4).
5. **`let` is sequential in both L and E** — RULED 2026-09-12 (GPT-6
   R44; the measurement in §5.4).
6. **`(import Init NAME)`**, a dependency-ordered prefix named by its
   last declaration, as the whole import mechanism at phase 2.
   Alternative: closure imports by name set, which need an index of
   the export and a union of closures re-ordered per load. **Ratified
   as bring-up (2026-09-15):** the prefix is a fixture technique (the
   prefix to `Int` is 17,812 lines); the name-set alternative is
   expected at the first library (phase 3's `v3/std/`).
7. **Constructor citations resolve through opened prefixes only** at
   Stage 0; the `type` form opens its own namespace for the rest of
   its file so today's bare `Nil`/`Cons` keep working. **Amended
   (slice 3.4, 2026-09-14):** for its **whole** file — a `type`'s
   constructors are pre-registered like the file's heads, so a body
   cites them before the declaration as it calls a later function
   (`loader.shard` `open_types`); another module's constructors are
   cited through `(use M.T)`, which the migration wrote for the
   toolchain (1,195 lines). **Amended (slice 3.9, 2026-09-17):** the
   automatic open covers the type's constructors only (`OpenSome`),
   never every declaration under its prefix — a record's accessors
   (`Load.env`) would otherwise make each field name a refused
   pattern variable of the file (`pattern_name`; eight refusals in the
   loader at the first migration); `(use M.T)` opens the whole prefix,
   accessors included, as a file's explicit choice. **RULED 2026-09-15
   (the user): `(use M)`
   does not open M's types' constructors** — as Lean's `open` does
   not; an implicit opening would make every same-named constructor
   pair across modules (`Nil`/`Cons` in two list types) a collision
   that arrives with the `use` line rather than with the citation. The
   cost is one `(use M.T)` per foreign type a file matches on: after
   the prune (slice 3.7) the toolchain keeps 623 such lines beside
   357 module opens, in 49 files.
8. **The toolchain profile is a property of the package layout**, not
   of a marker form (§8). **Amended (GPT-6 R55, slice 10):** the layout
   selects a named, bounded compatibility profile recorded per module
   (`profile=toolchain`); placement grants no privilege in K, the
   profile is bring-up and never a dialect, ordinary S is the
   destination of the toolchain's sources (§8, §6.7). **WITHDRAWN
   2026-09-14:** the profile is retired by item 34; until slice 3.4
   lands the layout rule is what the loader applies.
9. **`ev`'s `if` rule**: the then-branch iff the condition's cell is the
   second constructor of its type — one rule for `Bool`, `Decidable`
   and the prelude's `Bool`. **Amended (GPT-6 R56, slice 10):** the
   condition's type must be a transparent two-constructor type — the
   observation is the type's, `ev`'s tag bit its implementation — and
   the classifier refuses a `sig type` (`private_if`) or any other
   arity (`if_type`) wherever the declarations fix the type; a type
   parameter is unchecked at Stage 0, stated (§6.2, §6.7).
10. **Visibility is the transitive import closure** (§3.3, slice 3).
    Alternative: Rust-style explicit re-export, which v2 never had and
    the toolchain's own files would need lines for.
11. **A file's loading ends at a reading or loading error and goes on
    past a K refusal, a policy refusal and a `sorry`** (§3.2): the
    former say the text is not what its author thinks, the latter are
    verdicts on one declaration with the environment unchanged.
12. **One environment per role** (§6.6, slice 4): the implementation
    is checked in a fork of the loader's state from the view's fork
    point, the view replayed with the implementation's declarations
    substituted at each signature, the implementation's forms consumed
    in file order up to the implementing one. Alternative: the old
    tree's two same-named typedefs in one closure with a preferring
    lookup, which K's one-name environment cannot carry. **Amended
    (GPT-6 R57, slice 10):** the fork's `DISCHARGE` kinds are three
    statuses — a signature matched (`type`), an implementation linked
    (`fn`: the parameter stays a parameter), a law `proved` or
    `pending` — and none is the logical instance; that construction is
    phase 3's (§6.5, §11).
13. **A view's theorem is not re-checked in the fork**: checked once
    against the parameters, it is bound by its closure; the fork
    discharges parameters, not theorems. **Amended (R57):** the
    no-recheck is warranted by the instantiation construction, whose
    result inherits the supplied evidence's assumptions; at Stage 0
    nothing is instantiated and the records name the parts (§6.5).
14. **The implementation is `DIR/BASE.shard`** (the old tree's rule),
    not every file in the directory; `mod.req/` siblings wait for a
    consumer.
15. **Reading a `fn` is classifying it** (§6.7, slice 5): heads
    resolve through one suffix table over pre-registered file heads,
    so forward references and mutual recursion work as today; a
    declared name shadows a primitive's; exhaustiveness is the pattern
    matrix; the leak check reads static types as far as the
    declarations determine them. Alternative: a separate pass after
    reading, which would re-walk every body. **Clarified (R61):** the
    fusion is an implementation choice; a programmatic client's
    validation without a text round trip is T6's (§11).
16. **The profile is the `kernel/` and `meta/` directories**, E only,
    flat scope — every visible declaration by any suffix, no `use`.
    Alternative: `use` lines in the toolchain's own files, which the
    flat rule makes unnecessary at Stage 0 (records §7 expected them).
    **Amended (R55):** E only and flat scope are the profile's current
    coverage, not these libraries' permanent authoring restriction
    (§8). **WITHDRAWN 2026-09-14:** item 34 — the toolchain's files
    carry `use` lines like every file (§8.1 rule 4) and E-only is
    keyed on `Init` in scope (rule 1).
17. **`ev` is a machine with an explicit continuation** over a private
    linked representation (§6.7); the comparison primitives return the
    prelude's `Bool`, the wire's cells are the prelude's.
    Alternative: §6.2's direct recursion, which grows the host's stack
    with the program's and cannot stop at an extern.
18. **The substitution is the link**: the implementation's `fn` and
    `type` take the view's identities in the fork and at merge; no
    link declaration, no run-time resolution. Alternative: an `ELink`
    declaration resolved by the linker, tried and removed — the fork
    already replays the view with the implementation substituted.
    **Amended (R57):** the substitution is the operational link; the
    checked instance record is explicit when it exists (phase 3) and
    linkage never upgrades a parameter's logical status.
19. **`gen_fresh` is dropped**; `realize` (both forms, §7) and R45's
    third test moved to slice 5b (§7.5, items 20–24).
20. **The equations' types come from the classifier and K, not Stage
    1** (§7.5, slice 5b): a callee's telescope is filled by first-order
    matching of each runtime binder's type against its argument's
    inferred type, the classifier's static E type the fallback for a
    bare constructor; an undetermined binder is `untyped_subterm`.
    Alternative: waiting for Stage 1's typing, which would have left
    `realize` unbuilt until phase 3.
21. **Init's E-eligible inductives are E types on first citation**
    (§7.5): no indices, not in Prop, parameters types or propositions,
    fields E types or propositions; `Nat` and `Int` excluded by name.
    Alternative: registering every inductive of the prefix at import,
    thousands of telescope walks for the few a program cites.
22. **The primitive table's `Nat` set is K's accelerated set** under
    its identities, plus the three decisions; an entry is never
    realized. Alternative: one entry per operator with the identity
    chosen by argument type — the mode per entry §6.4 refused.
    **Amended (GPT-6 R58, slice 10):** an entry's meaning is K's rule;
    the executor's correspondence is the primitive suite's conformance,
    accounted and not exempt (§6.4).
23. **A Stage-0 equation per leaf of the case tree**, the tree the
    body's leading matches on parameters and pattern variables
    compiled column by column; a match elsewhere and an `if` are the
    recursor with a constant motive; numeral patterns are Stage 1's.
    Descent is syntactic; a callee must be realized earlier in the
    file. Alternative: one equation per arm of the outermost match
    only, which cannot be `rfl` for a nested pattern. **Amended (GPT-6
    R58, slice 10):** the equations are the correspondence only;
    progress is a separate obligation, recorded once and carried by
    every realization reaching it (`pending=`), and a retained
    candidate is never a completed realization (§7.2, §7.5).
24. **The `LATER` record is gone** with `realize` built; a `fulfills`
    outside an implementation check is the load error
    `fulfills_outside_impl` (by cause since slice 3.30 landing 2:
    `fulfills_not_obligation`, `fulfills_refused`, `fulfills_outside_impl`).
25. **The canonical dump text** (§10 item 1, slice 6): the measure
    clause is outside the text; names print as their last component,
    primitives in full. **Amended (slice 3.2, 2026-09-14):** both
    loaders bind `let` sequentially (item 5; §8.1 rule 7) and the
    bootstrap's dump prints its indices as loaded — the
    parallel-to-sequential renumbering this item once described is
    deleted (`dump.rs`). Alternative at the time: the V3 printer
    converting to parallel indices, which cannot represent a
    right-hand side citing an earlier binder.
26. **The seal is deferred to the phase-3 opener** (§6.6): the boundary
    is K, not `kernel/env`, and sealing K is a directory module of
    fifteen files, a view of 87 signatures, a bootstrap resolver change
    and a `private_module` rule, for no client that exists at phase 2.
    Alternative: sealing now against the 87 calls today's files make,
    then re-cutting the view for `meta/`. **Completion criterion
    (GPT-6 R52, slice 9):** the seal is done when the first `meta/`
    consumer imports K's view only — never `CheckedEnv`'s constructor,
    `env_pin`, the admission path or a session's memo state,
    transitively or through a profile shortcut — with the hostile
    battery's forged-node fixtures exercising the public entry and a
    raw-construction client fixture beside them (a small raw
    declaration checked, its result inspected, a second check without
    reaching around the API); a view file whose consumers still import
    the implementation is not the seal. `RUNNABLE`, `REALIZE` and the
    pending obligations stay separately represented across it, and a
    runnable self-recursive Stage-0 body discharges no totality or
    realization obligation by terminating on one input. **Order
    (ratified 2026-09-15):** the seal lands before the first `meta/`
    consumer is written — Stage 1 and I are that consumer, and written
    against the implementation first they become the reason the seal
    never closes; it follows the `use`-line prune directly. **In
    progress (2026-09-16, slice 3.8):** the four leans ruled — nine
    files sealed and six data files public, `k/k.shard` a facade, a
    view may import a plain file outside its directory, the internals'
    tests inside; landing 1 (the move, `verdict.shard`, the
    sealed-directory rule) landed; landing 2 (the view, one module
    across the directory in place of the facade — item 39 — the
    consumers, the tests inside) landed 2026-09-17; landing 3 (the
    hostile battery as a client, the R52 fixture) landed the same
    day — **THE SEAL IS COMPLETE (2026-09-17):** the criterion's
    structural parts are built and pinned, and its consumer half is
    enforced by the sealed-directory rule for every file outside `k/`.
27. **Calc's differential drivers sit outside the program** (§10 item
    2): the harness calls `ev` with values built as data and renders
    the results; an S program names no wire cell at phase 2.
    Alternative: the S port importing the prelude for the wire's cells,
    which the candidate rule of §3.1 refuses beside Init's `List`.
28. **An E-type position takes the type** when a citation resolves to
    both a type and its same-named constructor (the `type` form opens
    its own namespace, item 7, so `(type World (World Int))` opens
    `World` and `World.World`): a `fn`'s binders and result and a
    `type`'s fields are type positions; a term or pattern position
    still takes the constructor. Reconstruction of what the position
    determines, as level 0 is (§4). Alternative: refusing the idiom,
    which is v2's commonest; or a Stage-1 expected-kind resolution.

29. **The checked entry's argument types are `Int`, `Nat`, the
    byte-list codecs by identity — the prelude's `(List Int)`, Init's
    `(List Nat)` and `(List Int)` — and the World last** (§9, slice
    7); the origin of a refused argument is its position and text.
    Alternative: a typed argument grammar (`--int N`), which the
    entry's signature already states; or every entry reading
    `get_args` raw, which is the preconditioned kind and leaves T1's
    checked half unbuilt. **Amended (GPT-6 R59, slice 10):** by
    identity and element type, never by constructor shape (a `Tree`
    with the list arities was a codec, and its entry received cells
    over bytes); the World's identity is a stated Stage-0 limit until
    phase 4's handler contract (§9).
30. **`Init.NAME` cites the imported NAME explicitly** (§3.1, slice
    7): the scope adds the bare name as a candidate when the citation's
    first component is `Init` and the file sees `Init` (the reader's
    `init_cited`; the E table's `eresolve`). Alternative: no explicit
    spelling, under which a same-spelled native type makes the
    imported one unnameable in that file (T5's `same_spelled`).
31. **Accelerator authorization dispatches on the expected kind before
    any exemption** (`kernel/k/add.shard`, slice 9; GPT-6 R49): a name
    the reference table holds as a definition, opaque, axiom or
    inductive must be admitted as that kind; an exempt kind (theorem,
    quotient, constructor, recursor — exempt from BODY comparison)
    passes only under a name the table has no row for; a root
    candidate must have a row. Before: the matcher said True on a
    theorem without consulting the row, and a theorem named `Nat.add`
    in an environment without one was pinned — never applicable, since
    K types a head before it reduces an application (`function_expected`
    on every use, probed), but authorized. Alternative: refusing the
    candidate names for other kinds; declined — the name is the
    author's wherever the namespace policy allows it, and only the pin
    is refused.
32. **A primitive's outcome is three-valued** (§6.2, §6.4, slice 9;
    GPT-6 R51): a value, a guard failure (`EvStuck guard`), a resource
    exhausted (`EvExhausted nat_size | nat_count`; exit 3 beside fuel),
    the third decided before the work as K's `nat_apply` decides it.
    Alternative: exhaustion as `guard`; declined — a guard says the
    input is outside the operation's domain, exhaustion says nothing
    about it, and a consumer must never read a limit as invalidity
    (law §3.3's `Exhausted`, §9.4).
33. **Frontend parity's text is a stated projection, injective by
    check** (§10 item 1, slice 9; GPT-6 R53): one declaration per
    short name per kind in every compared closure, enforced by the
    harness before the tie, and the omitted distinctions listed.
    Alternative: full names in the text; declined — the two loaders
    spell names differently by design (the reader root-relative, the
    bootstrap by path), so a mapping between them would be a third
    artifact to validate.
34. **One E for every file — RULED 2026-09-14 (the user; §8).** Phase 3
    opens by building the Rust bootstrap up to the whole of E, the
    toolchain's own sources are written in it, and the toolchain
    profile is retired (items 8 and 16 withdrawn; GPT-6 R55 dissolved).
    Alternative: the profile kept to the flip as law §9.2 had it,
    which leaves the kernel an oddball dialect through phases 3–5 and
    every E rule stated twice; declined on the sizing (records §9,
    2026-09-14: the bootstrap's delta is three reader changes, the
    migration is `use` lines and 210 binder rewrites by tool).
35. **A `type` is E only by its fields, never by its directory or its
    file's imports** (§8.1 rule 1; slice 3.4): it enters K and E when
    its field types are L types, and is E only when a field cites a
    built-in with no L constant in scope or another E-only type; K
    checks any L form in any file. Alternatives: a marker form,
    refused by §8's old reason (the bootstrap refuses unknown forms);
    and the first cut, "a file that sees no `Init` has no L", killed
    the same day by the loader's `basic` pin, which declares its own
    `inductive Nat` and imports nothing. **The rule to watch
    (ratified 2026-09-15):** `(type Foo (MkFoo Nat))` is E only until
    its file gains an import that brings an L `Nat` into scope, and
    then the same text enters K — pinned both ways (`type_e_only`: the
    `def` citing the E-only type is `unknown_constant`; `type_flip`:
    with `(import Init Nat.add)` the type is `ACCEPT`ed and the `def`
    with it).
36. **`Symbol` is a built-in E type in every file**, the interned atom,
    with L identity `String` assigned at phase 3 (§8.1 rule 1); K's
    `Name` values are built from its atoms as today. Alternative: a
    `Name` literal in place of symbols (R60's row), which is circular —
    K's `Name` holds the atoms — and rewrites 1,494 sites for no
    change of meaning. **Amended 2026-10-01 (item 51):** the L identity
    is not assigned at slice 3.18. `String`'s representation is its
    bytes (§8.4 slice 3.18 rule 1), which an atom is not, and one type
    has one representation; symbols stay E-only until the toolchain's
    port needs them, the direction recorded there (rule 5): a type of
    its own over `String`, represented by the atom.
37. **E's literal rules are the profile's, for every file** (§8.1 rule
    2): a numeral of any sign typed by its binder, its kind the sign;
    `"…"` the constructor chain of the `List` in scope over its bytes,
    carried as one literal naming that type's nil and cons, until
    `String`'s E realization flips it for every file at once; `(list
    …)` the same `List`'s chain. **The flip is a sized slice
    (ratified 2026-09-15):** it changes the type of every `"…"` in
    every file from a byte list to K's `String` at once — the
    toolchain's 1,649 string sites and the byte-oriented helpers over
    them — and lands as its own slice with the parity and calc gates,
    never as an automatic consequence of `String`'s realization.
    **Amended 2026-10-01 (item 51): staged, as the rename was.** The
    flip landed at slice 3.18 for every file but the toolchain's own
    sources — `v3/kernel/**` is compiled on route 1 by the v2 chain,
    which reads `"…"` as a byte list and is not changed —, which flip
    when route 1's chain is V3's own. **Re-attributed 2026-10-16 (item
    69): the chain reads the emitter's output since slice 4.1; the flip
    for the toolchain's sources waits on Init's types in the toolchain's
    closure, phase 5's opener design.**
    Alternative: keep S's refusals (`string_literal`, `symbol_literal`,
    `list_sugar`) and give the kernel its own rules — the dialect
    again.
38. **One primitive table, both spellings** (§8.1 rule 5): the operator
    spellings are E primitives in every file beside the naming-law
    identities; the operators' L identities are assigned at phase 3
    with law §10.3's table. Alternative: rewrite the toolchain's 1,245
    operator sites to the naming-law spellings — a migration with no
    semantic content, and `/` would still need its own entry. **Open
    for Stage 1 (ratified 2026-09-15):** the operator spellings are
    `Int`'s primitives (rule 5), so `+` on two `Nat` values is either
    an `Int` operation or a type error and Stage 0 does not say which;
    Stage 1's typing answers it. **Answered 2026-09-17 (§8.4 rule 3,
    item 44):** by operand type, only where `ev` and K agree.
39. **One module across a sealed directory** (§3.3; the K seal,
    landing 2, 2026-09-17): a file directly inside a directory that
    has a view declares under the directory's module path, keeping its
    own path as the visibility tag, so the view's signatures name the
    private files' declarations and the implementation file is the
    import list. Alternatives: a facade of wrappers in `k/k.shard` —
    the design's lean, withdrawn because the fork check matches a
    `sig type` only against a declaration with the view's identity,
    which a wrapper cannot give a type short of boxing it (an
    allocation per call, every answer type crossing the box); identity
    aliasing in the fork — two names on one declaration, against the
    one-identity rule K's environment rests on. **Ratified 2026-10-13 at the close (the user: "Read through this, and all of that looks good to ratify to me").**
40. **E-only view parameters** (§3.3; landing 2): a `sig fn` whose
    signature names an E-only type is an E parameter only — no axiom
    in K, the head linked at `run` — and a `sig type` whose
    implementation is E-only is matched by its arity, K admitting the
    sig type as an opaque constant. Alternative: making the data
    vocabulary K types by importing `Init` into `name`, `expr`, `decl`
    — `Symbol`'s L identity is phase 3's later slice, and every
    identity and the load's cost would change for no client's gain. **Ratified 2026-10-13 at the close (the user: "Read through this, and all of that looks good to ratify to me").**
41. **A view may import a plain file outside its own directory**
    (§6.6's req-scope gate refined; landing 2): the K view states its
    signatures over the public data files. A file of the view's own
    directory stays `req_scope` — the implementation's side. Alternative:
    the data files under `k/mod.req/` as req-scope siblings, which
    moves six public files behind a path nothing else needs. **Ratified 2026-10-13 at the close (the user: "Read through this, and all of that looks good to ratify to me").**
42. **Records are v2's form, expanded at the s-expression level in
    both readers, with the accessors in the record's namespace** (§4;
    slice 3.9): `NAME.FIELD` and `NAME.with_FIELD` with the value
    first, opened for the file as any type's namespace is (item 7);
    `MkNAME` the default constructor; `make` and `with` name the
    record and resolve against the current file's; no law family
    until Stage 1 (none generated since: the laws are `Eq.refl`,
    slice 3.17). v2's `FIELD_of`/`with_FIELD` was the first cut and
    reversed the same day on the pilot's evidence: the loader's four
    records share `init`, `hash`, `view` and `file`, and six of their
    accessor names (`root_of`, `env_of`, `params_of`, `module_of`,
    `parts_of`, `closure_of`) are functions of the closure already —
    closure-wide accessor names do not scale past one record per
    file. The cost: the parity projection prints a head by its
    declared spelling (the identity beyond its module tag, `dump.shard`
    `dm_head`), the bootstrap resolves a bare citation of a dotted
    head by suffix, and route 1's old reader, which generates v2's
    names, cannot lower a file that uses a record until the V3
    lowering replaces it — no such file is in `t0`'s closure. The
    flat bootstrap resolves `make` and `with` against the closure's
    records, a looseness the V3 gate refuses. The accessors are cited
    qualified unless a file opens the record with `(use M.T)`: a
    type's automatic open covers its constructors only (item 7 as
    amended), or every field name would be a refused pattern variable
    of its file. **Ratified 2026-10-13 at the close (the user: "Read through this, and all of that looks good to ratify to me").** The cost named above is live: route 1's
    reader expands a record under v2's accessor names, so the kernel's
    drivers cannot be lowered until a pre-pass emits the accessors under
    V3's names (probed 2026-10-13 at the close; phase 4's first slice).
43. **`fn` = `def` + `realize`, automatic and transitive** (§8.4,
    slice 3.13; the user's ruling of 2026-09-17 on the five leans): a
    `fn` enters K when its signature's types and its body's heads all
    have L identities, and stays `RUNNABLE` with the obstacle named
    otherwise; once eligible, a failure is a refusal. The definition's
    value is the recursor directly (nested `T.rec`, the induction
    hypothesis for an immediate-field self-call), the equations
    `NAME.eq_N` by `Eq.refl`, the body attached through the realize
    path. Alternatives: an opt-in marker per `fn` (contradicts law
    §4.4, where `fn` requests admission and realization together); a
    silent fall-back to `RUNNABLE` on a failed body (hides a typo
    until a theorem fails to cite the function); `brecOn` from the
    first slice (Lean's encoding, needed only for deeper structural
    recursion — 3.14's). Deeper recursion and measures are 3.14's.
    **As built (slice 3.13):** `recursion_depth` is an obstacle, not a
    refusal — it is 3.14's shape, and calc's `parse_rest` has it; the
    equations need `Eq` in scope; in the fork a definition displaces
    the view's parameter (`DISCHARGE … defined`, §8.5); the fn is in
    the E table before its definition so a self-call types.
44. **Operator identities by operand type, only where `ev` already
    agrees with K** (§8.4 rule 3; closes item 38's open question):
    `+ *` at `Nat`, `+ - *` at `Int`, the three comparisons at both
    as the decisions; `-`, `/`, `mod` at `Nat` refused with the
    naming-law pointer. Alternative: resolve `-` at `Nat` to `Nat.sub`
    in the E program — the bootstrap, which has no static types,
    would compute the integer difference where K truncates, and
    parity and route 2 could not tie the two. **As built (3.13):**
    `+ - *` on two `Nat` operands type as `Nat` in the static pass;
    the comparisons keep the prelude's `Bool` statically (the
    translation reads the decision off K); a numeral at an expected
    `Int` is `Int.ofNat n` / `Int.negSucc (-n-1)` — law §5.2's one
    coercion, in E only until 3.15. **Ratified 2026-10-13 at the close (the user: "Read through this, and all of that looks good to ratify to me").**
45. **One term grammar for `def` and `fn`, one elaborator** (§8.4
    guard 1, carried from the phase-2 direction checkpoint): the
    surface is the union of the explicit-L and E forms, `tr` the
    elaborator, a `fn` a `def` that also passes the classifier. At
    3.13 the statement and one pin; the reader is unchanged. **Ratified 2026-10-13 at the close (the user: "Read through this, and all of that looks good to ratify to me").**
46. **The interface is the whole of a consumer's knowledge** (§8.5;
    the user's steer of 2026-09-17): nothing L crosses a view's merge;
    a `sig fn`'s definition and equations exist in the fork only; the
    requirements are the lemmas; the checked instance is a
    substitution over P, never a re-elaboration; deriving on a `sig
    type` is the module's. Alternative: exporting `eq_N` through the
    view automatically — every consumer proof would then weld to
    today's body, which is exactly what v2's weaning rule forbade.
    **Ratified 2026-10-13 at the close, with the reason stated (the
    user: "the requirements holding the opaque boundary of the module is an important boundary to keep so that internal mutations that preserve requirements are safe without rippling to all consumers"): the view's requirements are the consumer's whole
    knowledge so that an implementation may change under them without
    a consumer's proof moving.**
47. **Course-of-values, measures and the obligation class** (§8.4's
    slice-3.14 rules; the user's ruling of 2026-09-17): every
    structural recursion is `brecOn` with a generalized table, Lean's
    scheme, one scheme (3.13's direct recursor deleted); a native
    inductive's `below`/`brecOn` generated on first need under Lean's
    names; a measure is `WellFounded.fix` over the parameter tuple
    under `InvImage Nat.lt_wfRel.rel`, its decreasing facts admitted
    as obligations — the fourth policy class, visible in every
    dependent's `params=`, discharged by a later proof; no equations
    for a measured function until I; an `Int` measure and mutual
    recursion are obstacles. Alternatives: nested recursors with
    paired motives (the same construction by hand per definition);
    keeping measured functions `RUNNABLE` until Stage 2 can prove the
    facts (a quarter of the old tree's functions uncitable meanwhile);
    `sorry` for a decreasing fact (it admits nothing, so the
    definition could not exist). **As built (2026-09-17):** the
    table's shape read off the export and derived positionally; the
    obligation over the scope's locals; `PProd` reachability an
    obstacle; calc's `parse_rest` the first real course-of-values
    definition; the fixture through `InvImage.wf`.
48. **The elaborator above K** (§5.1, §8.4's slice-3.15 rules; the
    user's ruling of 2026-09-17 on the design's leans): metavariables
    as nodes K refuses (law §6), rather than an encoding K never sees
    (reserved fvars or constants: no level analogue, and I's holes
    need the node anyway); first-order unification with K's own
    definitional equality on closed residues (an over-eager assignment
    is K's refusal, never a theorem), rather than a second
    definitional equality above K; "numerals last" as the whole
    postponement discipline, rather than Lean's postponement queue;
    every undetermined argument or universe a refusal with a pointer,
    rather than a default; the one coercion at every check against an
    expected type; the operator spellings' identities by the first
    operand's type in statements, `-` `/` `mod` at `Nat` included
    there (nothing runs) while E keeps refusing them (guard 2); `if`
    as `ite` with a decision read off the proposition's head; a
    `type`'s parameters and a `fn`'s type parameters implicit, so L
    cites what E cites (guard 1). The phase-2 spelling of implicit
    arguments is gone from the pins and stands under `@`. **As built
    (2026-09-18):** see records §9.1 "Slice 3.15" (the as-built text
    moved there at slice 3.30 landing 3). **Ratified 2026-10-13 at the close (the user: "Read through this, and all of that looks good to ratify to me").**
49. **One front end: the pre-definition and its two projections**
    (§8.4's slice-3.16 design; the user's ruling of 2026-09-18 on the
    reorder and the shape, after the probe recorded there): S
    elaborated once into an L term with the self-name a local and
    every `match` a generated matcher constant, rather than the
    classifier's E term translated up (`tr`, deleted) or the E
    program read off K's finished value (it loses the recursion and
    the patterns — §7.3); matchers as constants K checks, Lean's
    shape, rather than a second term language with a `match` node
    (the pre-definition stays `Expr`, and a `match` in a statement is
    the same form); the erasure **is** the program for a function on
    the route, rather than a tie kept beside the classifier's (two
    resolvers would remain); `if` on a `Bool` as `ite (c = true)`,
    reversing slice 3.15's `cond` (rejected because: not Lean's
    elaboration, and past every fixture); an obstacle sends a
    function back to the E-first route whole while a type error is a
    refusal; the E-first route stays for the toolchain's sources
    until they port. Slices 3.17–3.18 take the rest of R60's row; I
    is 3.20+ (3.19 is deriving, since item 51).
    **Revision 2 (2026-09-18, GPT-6 R63–R71, records §4.10; the
    user's ruling on the three leans):** the pre-definition a named
    record, never an admitted declaration; matcher descriptions bound
    by regeneration and comparison, rather than trusted by name;
    `decide` erased to an explicit conversion, since `Bool` and
    `Decidable` have different cells (revision 1's identity erasure
    was wrong beyond an `if`); `ite` compiled dependently inside a
    recursive function, so a descent obligation keeps its branch
    fact (revision 1's plain `ite` would have stated `count`'s
    obligation falsely at zero); `Nat.sub`, `Nat.div`, `Nat.mod`
    selected once resolved (revision 1's refusal reversed); a
    `RUNNABLE` callee a local, so a fallback never re-resolves — the
    E-first fallback kept, flagged `route=e_first`, only for a body
    head without an L identity until 3.17 (rejected: making those
    functions unrunnable meanwhile); one descent obligation
    discharged with an acyclicity check, in this slice rather than
    at I; K's outcomes and `UStuck`/`UExhausted` at the shared
    boundary; contextual holes across binders deferred to I's opener
    with the narrower guarantee stated (rejected for now: building it
    in landing 1, no consumer in this slice); value hashes an alarm
    to explain, not a gate (revision 1's unchanged-hash gate
    contradicted its own `ite` rule).
50. **Slice 3.17's scope (design 2026-09-19; RULED 2026-10-01 — the
    user agreed with the leans; built the same day, §8.4's as-built).**
    Measured by probes before code: records and their laws already
    work on the typed route; `< <= =` already elaborate. The leans:
    `"…"`, symbols and `Name` literals move to slice 3.18 with the
    flip (a `fn` body is an L position, where `"…"` is already a
    `String`; `Symbol` as `String` now would settle the wire's cell
    by a side effect) — *this narrows the slices table's row and is
    the user's to rule*; literal patterns on the typed route; the
    rename for the files no v2 tool reads, `v3/kernel/**` when route
    1's chain is V3's own (rejected: an alias in the old trusted
    kernel, where `=` is the claim language's; a rewriting shim in
    `build.sh`); the fresh-name supply a library; `dependent_update`
    a refusal; a registry row for `Int.natAbs` so calc's `show_nat`
    is defined. Landing 3's as-built misnamed `show_nat`'s cause (no
    `"…"`: `Int.ediv` past the prefix) — corrected in place.
    **Built differently from the design, each with its reason in
    §8.4:** numeral rows are the `ite` on the decision at `Nat` too
    (rejected: the constructor reading through the matcher — a `succ`
    chain as deep as the numeral, and nothing E can run); the
    fresh-name records take the prefix's type as a parameter;
    `make`/`with` reach a parameterless `structure` only; the
    bootstrap's `= < <=` are native-only names beside the old
    kernel's table.
51. **Slice 3.18's ruling and scope (RULED 2026-10-01 — the user
    agreed with five leans; built 2026-10-02, §8.4's as-built).** The
    wire's type, which the slices table put first. (1) `String`,
    `ByteArray` and `UInt8` run as **the byte list**: each of `Fin`,
    `BitVec`, `UInt8`, `Array`, `ByteArray`, `String` represented by
    its one runtime field (the registry's type rows), no executor
    changed. Rejected: a packed buffer now (a fourth value in two
    executors and a table of primitives trusted to match Lean's
    functions, for a benefit that is the lowering's); the interned
    atom (item 36 read literally: every string built is interned for
    good). (2) The externs carry `ByteArray`. Rejected: `List UInt8`
    (list cells in the host contract), `String` for paths and
    arguments (validation inside the boundary). (3) Symbols and `Name`
    literals are **deferred** to the toolchain's port — item 36
    amended; the slices table's row loses them. (4) The flip is
    **staged**, as the rename was — item 37 amended. (5) Deriving is
    slice 3.19; I is 3.20+. **Built beyond the leans, each in §8.4:**
    a projection function is its projection (a structure's field was
    refused in a `fn` body); the driver reads an extern by its
    declared type; a classifier refusal after a typed-route obstacle
    names the obstacle. **Not built:** the checked decoder — no
    consumer turns input bytes into a `String`, and its proof is I's;
    until then text enters a program as its literals.
52. **Slice 3.19's ruling and scope (RULED 2026-10-02 — the user
    agreed with five leans; built the same day, §8.4's design and
    as-built).** Deriving under a declared policy, law §5.1. (1) A use
    site finds a derived operation through **a derivation table** —
    one entry per type and capability, visible where its procedure
    is, no search; `=` in an `if` consults it. Rejected: named
    functions only; instance resolution pulled forward. (2) **One
    named policy, `structural`, written at the derivation.** Rejected:
    a package default. (3) **The ordering is `compare` into Init's
    `Ordering`, without laws.** Rejected: the laws before I. (4) **The
    rendering is bytes in canonical S.** Rejected: Lean's `Repr`; a
    `String` before `String.append` has its equation; deferring it.
    (5) **Supported: a type without parameters and Init's `List`,
    `Option`, `Prod` at closed arguments** — each instantiation its
    own derivation, E having no function values. Rejected: types with
    parameters and nested types now. **Decided in the design:** the
    form `(derive TYPE CAPABILITY…)` with `(CAPABILITY by NAME)` for a
    hand-written procedure; generation as `fn` source through the one
    front end; the mangled namespace (`List_Tok.decEq`). **Departures
    from the leans as built, each in §8.4:** Lean's instance constants
    are not generated (`T.decEq`'s type is `DecidableEq T` unfolded;
    `Ord T` holds a function and waits for Stage 3); a function-valued
    field is refused under `render`, not printed `_`; wider than
    asked — a one-constructor type with parameters derives, and the
    library gives `UInt8`, `ByteArray` and `String` their orderings
    and (but `ByteArray`) renderings beside equality. **Built beyond
    the leans:** a `match` generalizes a local scrutinee in its
    expected type (rule 9); the registry's `Bool` rows; an
    obligation named once per statement; `prim_type`, the
    classifier's refusal of an integer comparison at an inductive.
    **Found by a second reader and fixed (§8.4's as-built):** a
    registry row spoke for any constant bearing its name; a
    `Decidable` value had two runtime representations (older than
    the slice) — a decision is its test only as an `if`'s condition;
    a derivation took a same-named constant for its own; a registered
    entry's visibility depended on load order.
53. **Slice 3.20's ruling and scope (RULED 2026-10-05 — the user
    agreed with six leans; the design 2026-10-06, §8.4).** I's
    opener. (1) **Three slices** — 3.20 the certificate core, 3.21
    `simp_only` and `arith`, 3.22 the producers. Rejected: one slice;
    the forms before the API. (2) **The tactic block in source is I**:
    `(by …)` holds I's concrete syntax, a node omitting a consequential
    choice names the versioned default reconstruction, the pin records
    what was written. Rejected: a tactic surface distinct from I.
    (3) **No subterm order**: `induction` carries its recursor, `wf` is
    `WellFounded.fix` over `(measure f)` with every descent fact a
    goal. Rejected: v2's `subterm-induct` and `(below)`; the porting
    cost (60 sites) accepted. (4) **Computation is conversion**:
    `reduce` is a `show` of the computed normal form, `unfold` is `rw`
    by the equations, `decide` is `of_decide_eq_true (Eq.refl true)`.
    Rejected: a `simp` with implicit lemma sets. (5) **Calc's claims by
    hand** as the worked-examples set; the migration tool with the
    broad port. (6) **The API is sequential at the opener**, I's data
    in the graph's shape; the transactional graph and the store wait
    for the producers. *Designed beyond the leans:* a goal is a
    closed proposition and an occurrence is the hole applied to its
    telescope, which settles R68 (slice 3.16 rule 10) by construction;
    a block is a sequence on one goal with children inline (§8.4 rule
    1); v2's `reduce`, `simp` and `compute` are one node. *Built
    2026-10-06 (§8.4's as-built):* as designed, with `(wf X MEASURE IH
    …)`, the arms of a split in one list with an optional equation
    name (`(cases X H (ARM…))`), a one-step premise block written as
    the step, `rw` matching syntactically, `reduce` applying an
    unfolding equation only to a closed occurrence and `unfold` once
    per occurrence, the applications of functions masked under K's
    whnf; **a measured function's equations** by `WellFounded.fix_eq`
    (`define.shard`), which the design did not list and calc's
    `parse_tail` needed. Not built: a witness goal, `rw` at a
    hypothesis, the `PROOF` record (`ACCEPT` serves).
54. **Slice 3.21's design (under item 53's ruling; the design and the
    as-built 2026-10-06, §8.4) — for ratification.** The second of
    the three slices: `arith` and `simp_only`. *Decided here beyond
    the ruling:* (a) **`arith` takes the context's linear hypotheses
    and the facts given**, as `omega` does; a certificate indexes the
    rows in that order. Rejected for now: the facts alone (`linarith
    only`) — an `only` restriction waits for a budget refusal that
    needs it. (b) **An omitted certificate names a reconstruction**
    (item 53's second lean): Fourier–Motzkin over the stated rows in
    atom order, bounded at 512 derived rows — a function of the node
    and the goal, no table consulted; cuts and case splits stay a
    producer's. (c) **P through `Lean.Omega`'s lemma kit**, the
    arithmetic K's conversion on closed data; no reflection of terms.
    (d) **The quotient and the remainder by a positive literal are
    linear** (v2's `div-facts`; §12.3's row). (e) **One spelling**: a
    goal's statement and a lemma's equation enter in the source's
    spelling of Init's class methods at `Int` and `Nat`, for every
    form. (f) **`simp_only` discharges a rule's premises by
    assumption** and has nothing implicit. *Built beyond the design:*
    (g) **a matcher under a measure carries its equation** — slice
    3.14's obligations ("over every local in scope") are now stated
    under `scrutinee = pattern` for each enclosing row and as the
    comparison `measure[arguments] < measure[parameters]` at `Nat`,
    and a measured function's equations show the matcher applied to
    `Eq.refl`; (h)
    `reduce` leaves a proposition as it is and takes a decided case
    analysis one step; (i) a type's name wins over its constructor's
    where a type is expected (item 28); (j) the pending record carries
    the sorried goals (slice 3.20's rule 9). *Left open, stated:*
    Init's visibility is the load's, not the module's (an open can
    turn ambiguous when another file imports further; **closed by item
    55**); a measured function's obligations are stated over the
    arguments' tuple; `reduce_stuck` on an already normal target
    (**closed by item 55**). **Ratified 2026-10-08.**

55. **Slice 3.21b (ruled 2026-10-06 on GPT-6's trajectory review,
    records §4.11; the design and the as-built, §8.4) — for
    ratification with item 54.** (a) **The expected type decides an
    arithmetic operator** (rule 1): an elaborator fix under law §5.2's
    order; the review's finding 1, where `(- 1 2)` at `Int` was 0 and
    K proved it. (b) **Init's visibility is the module's horizon**
    (rule 2): a module's meaning is independent of the load's order;
    the review's finding 2. (c) **`reduce` succeeds unchanged where
    nothing reduces** (rule 3): item 54's left-open `reduce_stuck`,
    reversed on the review's finding 3 — a proof was tied to how far
    the step before it went. *Carried to 3.22's design, by the same
    ruling:* a stored certificate keyed by its rows' statements, not
    their positions; the dependent witness goal. **Ratified 2026-10-08.**

56. **Slice 3.22's design (ruled 2026-10-07 on three leans; §8.4) —
    for ratification.** The producers. (a) **The engine emits `(arith
    only FACT… (farkas K…))`, the certificate written** — the rows named,
    nothing from the context; rejected: reconstruction at every replay.
    (b) **The store is a CI artifact**, the repository holds sidecars;
    rejected: committing P. (c) **The engine's reach is measured over
    calc's 100 claims**, the hand proofs kept; replaces the B-1c
    consumer. *Decided in the rules beyond the leans:* the sidecar's
    key is the ACCEPT fingerprint (rule 2); a witness goal is a premise
    with an open metavariable, blocks in order, `witness_open` at the
    step's end (rule 4); no `attempt`/`commit` API until a concurrent
    caller (rule 5); the store in K's export format and
    `verify_release` as T0's replay over it (rule 7). *Landed in three
    landings (2026-10-07/08; the as-built sections of §8.4).* Decided at
    landing 3: each store file is its own stream over the environment so
    far, and the gate ties each accepted declaration to its order line's
    name and fingerprint — K's acceptance is the guarantee, the tie says
    the bundle is the build's; the implementation forks of views are not
    stored (open until a consumer needs it). **Ratified 2026-10-08.**
57. **Slice 3.23, the Init cache (ruled 2026-10-08 on GPT-6's R75;
    §8.4) — for ratification.** The user's ruling: a snapshot cache is
    a practical need, kept shallow enough to rework as the pipeline is
    optimized. *Decided here:* (a) **the cache is K's verdict, not a
    second format** — the pinned export stays the only on-disk form
    of Init, and a receipt written by a T0 run that accepted every
    record names the chunks by path and byte count; rejected: a
    serialized environment (a deserializer behind the seal, a format
    to keep in step, and a ceiling of a few seconds since the tables
    must be built either way — measured below). (b) **Admission skips
    the typing judgments of axioms, definitions, theorems and
    opaques and nothing else**: an inductive block and the quotient
    take the checked path, since their constants are what that path
    generates. (c) **A receipt is explicit and a stale one is
    refused**, never checked silently: `--init-receipt` names it, a
    chunk not listed at its byte count is `init_receipt_stale`. (d)
    **The receipt is tied to K by the writer, not the reader**: the
    suite's helper hashes the sealed module's sources and the fixture
    into the receipt's header and regenerates on a change; the loader
    reads only the chunk lines. (e) **`verify_release` takes no
    receipt** — the release gate is K alone over the export as before.
    *Left open, stated:* the loader pins' entrypoint re-streams Init
    once per case in one process; a load shared across cases (the
    persistent session, T9) is the next lever, not this one.
    **Ratified 2026-10-09. Retired at slice 3.26 landing 3
    (2026-10-09, the user's ruling):** the receipt dropped and its
    flags deleted — with a load checking the closure it cites (item
    60), admission saved a tenth to a fifth of a demanded load, and the
    receipt's only writer would have been the full replay; (b)'s admit
    path stays as the loader's second admission of records it checked
    (item 60(b) as amended); (a), (c), (d) have no mechanism now.
58. **Slice 3.24, the connected path (ruled 2026-10-09 on GPT-6's R76;
    §8.4) — for ratification.** (a) **`(dif h C T F)`** is the source
    form of the dependent if, the hypothesis named first; `if` is not
    overloaded with a fourth argument. (b) **The path is one example
    and one test**, the five breaks of law §12.4 required to fail by
    name, the three slice 3.22 tests re-exercised rather than
    re-implemented. (c) **The prepared handle and the World-alias
    fixture stay phase 4's.** *Found at the build, open:* the dependent
    match (a local whose type mentions the scrutinee generalized),
    the consumer `List.get`'s realization; a view of an Init
    definition whose matcher K does not regenerate (`Option.getD`).
    **Ratified 2026-10-09.**
59. **Slice 3.25, the shared Init load (ruled 2026-10-09 at the
    boundary after R75 and R76; §8.4) — for ratification.** (a) **The
    Init stream is replayable**: streamed once to its end, every root
    of a process loaded fresh on it (`init_all`, `ld_reroot`); a
    driver's lazy stream unchanged. (b) **The horizon is the module's,
    never the stream's**: every availability gate asks the scope, and
    an `(import Init NAME)` is answered from the ordinal table. (c)
    **Init's names are Init's**: a declaration named as an Init
    constant is refused wherever the environment holds it, which the
    release gate always does; a lazy driver that stopped short accepts
    what the gate refuses — stated as the one disagreement, closed at
    the gate. A name index of the export for the driver (a second file
    beside the receipt) was rejected at the build and reopened the same
    day on what was taken for evidence — `v3/std/list.shard`'s `(fn
    List.sum …)` against Init's `List.sum` — and that reading was wrong
    (found at slice 3.26 landing 2, by running it): the file declares
    `std.list.List.sum`, every declared name carrying the module path
    (§3.1), and K holds both. The index stands on its own ground
    instead: slice 3.26 rule 5, the driver and the gate refusing from
    the name table alike, closes (c) for the one live shape, a module
    named as an Init namespace (`registry_native`'s `Bool`). (d)
    **The loader pins' entrypoint streams once** — 783 s to 33 s — and
    the three pins that leaned on the stream stopping short are
    restated, one of them (`registry_native`) now the refusal of (c).
    *Amended at slice 3.26 landing 3 (2026-10-09):* (a)'s mechanism —
    the export streamed to its end by `init_all` — went at 3.26 landing
    2; what (a) ratifies is the principle, a process loading each
    record of Init once, carried now by K's run of Init alone
    persisting across the roots of a process (`ld_reroot` over the
    loader's Init state). (d)'s number is 42 s by the walk, every
    record checked (3.25's 33 s was the stream admitted under the
    receipt, since dropped). **Ratified 2026-10-09, as amended.**
60. **Slice 3.26, Init on demand (ruled 2026-10-09 at the boundary
    after slice 3.25; §8.4) — for ratification.** (a) **The export is
    indexed, never streamed, by a load**: a record table and a name
    table built once per environment, read by `read_range`, the
    export's node ids kept. (b) **A module materializes the closure of
    what it cites**: its identifiers' candidates and the gates' kit
    lists below the horizon, walked by range reads, fed to K as a
    sub-stream in export order; checked without a receipt, admitted
    under one. (c) **The horizon law stands**: visibility is the
    ordinal below the horizon; a name not materialized is absent to
    resolution; a constructed name missing from a kit list is a refusal
    by name and a defect of the list, found by parity — no retry. (d)
    **Init's names are Init's from the name table**, closing item
    59(c). (e) **The fixtures go**; the receipt stays a speed lever.
    (f) **Deferred with consumers named**: a theorem admitted by its
    statement, the loader native, a serialized environment. *Measured
    at the ruling:* line-level closures under one percent of the
    export; block-level closures up to a third (the unit is the line).
    *Amended at landing 2 (2026-10-09, as built):* (a) the index is
    one file of three tables — the record table, the name table and a
    **declaration table** (9 bytes a name id: the ordinal of the record
    declaring the constant of that name), which the walk needs when an
    expression line cites a constant by its name id; (b) K's run of
    Init alone persists in the loader's Init state and is carried
    across the roots of a process (`ld_reroot`), each demand checked
    once into it and admitted once into the loader's own environment,
    which holds the natives beside Init — the alternative, re-rooting
    from the state before any demand, re-checked every case's closure
    (the loader pins: 1,761 s against 42 s); (c) the kit lists are each
    gate's own (`el_kit`, `tc_kit`, `ar_all_kit`, `dv_kit`, `df_kit`,
    K's and the pins' in the loader) and an inductive block's names
    demand their satellites (`casesOn`, `noConfusionType`,
    `noConfusion`, `below`, `brecOn`, a constructor's `inj`); (d) item
    59(c)'s `List.sum` evidence was a misreading (the declared name is
    `std.list.List.sum`): rule 5 stands on `registry_native`'s shape.
    *Amended at landing 3 (2026-10-09, the user's ruling):* (b) every
    demand is checked — the receipt is dropped, measured at a tenth to
    a fifth of a demanded load (the walk and K's ingestion are the
    cost); the admission that remains is the second one, of records
    this process checked, into the loader's own environment; (e) the
    fixtures and the receipt go, and item 57 is retired; (f) the loader
    native is the lever that pays: the suite's entrypoints of many
    short loads doubled on the runner at landing 2 (`wire_test` 155 s
    to 436), each process walking its demand on the bootstrap. **Ratified 2026-10-09, as amended.**
61. **Slice 3.27, the library arc (designed 2026-10-09 at the boundary
    after slice 3.26; §8.4).**
    (a) **Init is the library**: a statement Init has is cited by
    Init's name; `v3/std` declares only what Init lacks; an old module
    every declaration of which is Init's or `arith`'s becomes a
    migration record and no file (`std/order`, `nat`, `div`, `arith`);
    V3's own `List.sum` goes. (b) **One spelling, everywhere**: the
    unifier and the rewriter's matcher unfold Init's instance constants
    and class projections as `arith` already reads them (slice 3.21
    rule 3), so a lemma in Init's spelling applies to a term in V3's —
    the probe's `witness_open` on `apply Nat.zero_and` is the
    regression. (c) **Instances by table, not search**: an
    instance-implicit binder at the library's types resolves from a
    fixed table of the pin's instance constants and composed terms;
    `min`/`max` are `Min.min`/`Max.max` under it; anything else is
    `instance_needed` with the `@` pointer; search stays Stage 3's door.
    (d) **`/` at `Int` is `Int.ediv`** — the operator row says
    `Int.div`, which the pin lacks; a defect, fixed with a pin, law
    §10.3's row validated. (e) **The horizon refusal points**: a name
    above the horizon is refused with its ordinal and the import to
    write; the horizon law unchanged. (f) **Supplied bodies with
    equations by `rfl` or `cases`** for Init's functions until the
    derived view reads compiled recursion; the matcher of an Init
    definition realized through its own definition, the supplied body
    the fallback. (g) **The dependent match lands at `List.get`**
    (slice 3.24's open item; slice 3.13 rule 2's narrower guarantee
    gets its consumer). (h) **The fifteen as theorems** in
    `std/facts.shard`, twelve by Init's names (proved by the probe),
    the three bitwise recurrences from Init's `testBit` kit or one
    unfolding of `Nat.bitwise`. (i) **The records** in `std/README.md`
    and `std/migration.shard` (one theorem per row of law §10.3), the
    39th entrypoint `std_test.sh` with the store's gate. (j) **Calc's
    `list.shard` retires onto `std/list`** as the first consumer, its
    claims re-spelled and its sidecar regenerated. (k) **`LEAN.md` and
    T9 small at landing 4**; T9 again at phase 3's close. (l) **T2/T3
    and T10 are slices 3.28 and 3.29**, language slices with their
    designs at this arc's close; the migration tool's tier 0 is phase
    5's opener with these records as its calibration data — a stated
    departure from law §12.3's "calibrated on `std` in phase 3".
    **Ratified 2026-10-10 as designed: (j) and the tier-0 departure in
    (l) taken as proposed.** *As built at landing 3 (2026-10-10), for
    the user's eye — four readings the rules left open:* (i) the refuted
    row of a dependent match is Init's `absurd` (or `False.elim`), an L
    term of the row's type, erased to a stuck row; the E-first route
    refuses it by name; (ii) a `realize`'s binders are E types, so a
    `Fin` index is written `Fin`; (iii) `Subtype` is a row of the
    registry (law §10.3 row 11), the refinement running as its value;
    (iv) `HPow`/`Pow` at `Nat` is `Nat.pow` in the one spelling. Beside
    them two facilities the retirement of calc's `list.shard` needed:
    the reducer reads a realization's equations where the realizing
    module is open, and the loader marks that `use` needed. *As built
    at landing 4 (2026-10-10), for the ratifier's eye:* (k) as proposed
    — `docs/LEAN.md` in 92 rows and T9 small passed (0 interventions, 1
    refusal before success, ten gaps disposed of); one departure: T9's
    (ii) measured on route 2, since route 1 compiles the kernel's
    bring-up dialect only; two findings fixed in the kernel beside the
    pointers the lists name — a bare zero-parameter `fn` is its call on
    both routes, and the `get!` pointer names the Option form this pin
    declares. **As built, ratified 2026-10-10: landing 3's four
    readings, landing 4's departure and the two kernel fixes, without
    reservation; slice 3.27 closed on pipeline 576.**
62. **Slice 3.28, the lambda profile (designed 2026-10-10 at the boundary
    after slice 3.27; §8.4).** (a) **The static role**: a function-typed
    binder of a `fn` or of a supplied `realize` is a static parameter
    and the declaration a template, with no keyword; its L half as
    today, its E half one program per instance. (b) **The escape rule**
    at today's refusal sites, `function_value` with the pointer on both
    routes. (c) **Instances by shape**: a static argument's identity and
    captured count, never the captured values; a closed lambda lifted to
    `CALLER.lam_N` with the captured locals leading; the instance owned
    by the taking module and named by the derive precedent's
    flattening (`main.List_map.of.main_add_offset_lam_1_1`), one
    program per distinct tuple per module; its L meaning a generated
    definition unfolding to the template applied, its equations the
    template's instantiated and checked by K. (d) **Finite
    specialization**: a static argument is never built from a static
    parameter (passed on, not transformed), so the shape set is closed;
    `realize_descent` extended to static shapes; `instance_budget`,
    1,024 per template per load, as law §4.3's budget. (e) **Packages**:
    a structure with a function-typed field is a package type and never
    an E type; a package constant read by whnf to its constructor; its
    projections erase to the fields' shapes; the generic theorem proven
    once. (f) **Six list templates** in `std/list`: `map`, `filter`,
    `foldl`, `foldr`, `any`, `all`. (g) **The gates**: `template_test.sh`
    with the artifact check over `--dump`, nine pins, `LEAN.md`'s rows,
    T2 and T3 recorded against law §12.5's risk columns. (h) **Three
    landings.** (i) **The limit**: route 1 unchanged; the artifact check
    is on the E programs. **Ratified 2026-10-11 as designed** (the
    user: "That looks reasonable -- ratify it and let's begin
    implementing."), the four leans taken: instances named by their
    flattened shapes under the taking module, not by an ordinal; the
    budget 1,024; six list templates, not `map` alone; a package as a
    structure with a function-typed field, no new form. *As built,
    landing 1 (2026-10-11), for ratification at the close-out:* the
    instance's name flattens the template **with its type arguments**
    (`main.main_twice.of.main_add_off_lam_1_1` for a template without
    type parameters, as designed); a second taking under one name at
    other type arguments or captured types is refused `instance_clash`
    rather than reused; the escape rule and the self-call at other
    shapes are checked at the template's declaration by one erasure at
    placeholder shapes; a polymorphic taker — an instance at an open
    type, a lambda over a type parameter — is refused by name as this
    landing's limit; the E-first instance and the realize template are
    landing 2's. *Landing 2 (2026-10-11), for the same ratification:* a lambda
    field of a package applied as a static argument is lifted under the
    taking body's name, not the constant's; a realize template's written
    static type is checked by its E reading against the L type's; the
    structure reader follows Lean's binder infos (parameters implicit in
    the constructor and the projections); the E-first instance moves to
    landing 3. *Landing 3 (2026-10-11), for the same ratification:* the
    E-first instance takes static arguments by name and by named partial
    application only — a lambda, a registry constant, a polymorphic fn and
    a package are refused with pointers (the E-first reader has no L half:
    nothing lifts, no type arguments are written) — and has no equations;
    the budget is a parameter of the load (`--instance-budget`), the pin
    exercising it at 2; a tenth pin, `template_e_first`. **The close-out
    (2026-10-11): the three landings are built; the as-built departures
    above — landing 1's instance names with the type arguments and
    `instance_clash`, the declaration-time check at placeholder shapes;
    landing 2's lifted package field under the taker's name, the written
    static type checked by its E reading, Lean's binder infos in the
    structure reader; landing 3's E-first limits and the budget's parameter
    — ratified 2026-10-12 without reservation (the user: "Those departures
    are fine"); slice 3.28 closed on pipeline 585.**

63. **Slice 3.29, the tactic library — T10 (designed 2026-10-12 at the
    boundary after slice 3.28; §8.4) — ratified 2026-10-12 as designed
    (the user: "This looks good, and the limit is okay for now"), the
    four leans taken and rule 10's limit accepted.** (a) **The
    fixed interface is data at the boundary, one shot**: a tactic is a
    `fn` over text — the goal in as `(goal CTX TARGET (applicable …))`,
    a block of I out — and the kernel replays what comes back as a
    hand-written block; rejected: the elaborator's state crossing into
    the library, a host callback, a round-trip protocol; in-process
    composition of `step` stays the kernel engine's until route 1
    compiles user files. (b) **The producer is named in the source's
    delegation**, `(auto (tactic M.FN) HINT…)`, run by `prove` and never
    by the build; rejected: a driver flag, a `by` form run at build.
    (c) **The sidecar carries I's schema**, `(sidecar N)`, 2 from this
    slice; an older sidecar is reported, a newer refused; rejected: the
    engine's version as the schema, a version per entry. (d) **The
    guarded occurrence carries its expected subterm**, `(occ K EXPECT)`,
    `rw_occ_changed` when the source moved; rejected: a positional path.
    *Decided in the rules beyond the leans:* dependents reverted into
    the motive under `cases` and `induction` (the probe's gap); the
    refusal pointers naming the library route; sibling names, external
    P and the settings clause pinned, nothing built; both producers'
    solutions replayed, stored and verified by K alone; the example a
    `Bool` decision procedure; two landings. *As built, landing 1
    (2026-10-12), for ratification at the close-out:* the guard
    compares by structure, then by unification (a literal's spelling
    never trips it); the equation hypothesis precedes the reverted
    dependents in the motive and in the arm; the committed sidecars
    regenerated from nothing by `prove` (an entry still valid is never
    rewritten), the engine closing `auto_core`'s `u` by `rfl` now; the
    pins list's duplicated entry removed (259 cases). *Landing 2
    (2026-10-12), for the same ratification:* one renderer for the goal
    a tactic reads and the goal a refusal shows, its binders spelled as
    source writes them in every message; the producers' inputs as one
    value (`Producers`: the entries, the programs, the fuel), the
    driver's `--fuel N`; the example library E-first over the kernel's
    S reader, six theorems (two beyond the design's four), the engine
    closing the same six; a statement over Init's names alone
    fingerprints the same in every module, so the gate compares the
    regenerated sidecar byte for byte; the two producers' `ACCEPT`
    records agree but for the axiom closure, the proof's (the engine's
    `arith` rests on `propext` and `Quot.sound`, the library's `exact`
    on nothing) — rule 8 read as the name, module, fingerprint, horizon
    and parameters; a tenth pin, `auto_tactic`, over a constant tactic
    of its root (260 cases). **The close-out
    (2026-10-12): both landings built; these readings ratified
    2026-10-13 without reservation (the user: "Those look reasonable;
    ratify them"); slice 3.29 closed on pipelines 588 and 589.**

64. **Slice 3.30, the phase-3 close-out (designed 2026-10-13 at the
    boundary after 3.29; §8.4 "Slice 3.30"; law §12.4 item 3, §12.5
    T9, §10.5; for ratification before landing 1):** the leans taken
    in the design. (a) The gate ledger's readings: Stage 2 covered by
    slices 3.20–3.22 with the law's `change` read as `show` and the
    shared holes and incremental goal states phase 4's (T4, T6); Stage
    3 as ruled at item 61 (the fixed instance table, search the door);
    `order`, `nat`, `div`, `arith` covered as migration records by the
    same ruling. (b) T5's two instances and R57's checked instance
    carried to phase 5, built when the port's first consumer needs two
    implementations of one view; the alternative, built now as a
    landing, stated. (c) `Symbol`'s `String` flip carried to phase 4
    with route 1's chain (re-attributed 2026-10-16, item 69: Init's
    types in the toolchain's closure, phase 5's opener design). (d) T9 again as nine tasks — rule 11's six,
    a static package, a one-tactic library, the T9 row's two
    diagnostics — with the count written before the run (rule 5).
    (e) The consolidation by moving each slice's design and as-built
    text verbatim to records §9 and keeping per-facility current
    statements in §8.4, the title and status block rewritten; the
    alternative, an index over the chronology, stated. (f) The doc
    banners in `TCB.md`'s shape on the four phase-3 documents.
    **Ratified 2026-10-13 as designed (the user: "Those leans sound
    reasonable: ratify them. We can proceed to Landing 1").** **As
    built, landing 2 (2026-10-13), for ratification at the close:** T9
    again passed on the count as written — 9 of 9 accepted, 0
    interventions, at most 2 refusals before success on any task (both
    of (iii)'s the author's), (vi)'s three pointers, both of (ix)'s
    refusals naming what rule 5 asks; two findings against the kernel
    fixed with pins (a `fulfills` naming no obligation refused by its
    cause; a measured `fn`'s obligation discharged inside an
    implementation directory, which the implementation path had not
    allowed); ten of the fourteen reported gaps `LEAN.md` rows, one a
    spelling in slice 3.28's rule 5 corrected, the rest the author's
    reading or the brief's wording (`docs/records/T9-2026-10-13.md`).
    **As built, landing 3 (2026-10-13), for ratification at the
    close:** the consolidation by the lean — 4,391 lines moved
    verbatim to records §9.1 (the digest equal before and after),
    1,115 lines of per-facility current statements kept in §8.4,
    numbered as the slices numbered their rules; the slice table the
    index; the title and the status block rewritten. The current
    statements are new prose, the record the authority where the two
    differ. **The as-built notes of landings 2 and 3 ratified
    2026-10-13 (the user: "Read through this, and all of that looks good to ratify to me").** **As built, landing 4 (2026-10-13),
    the close:** the pass done — items 39–42, 44–46 and 48 and this
    item's notes ratified, item 46 with its reason recorded; records
    §9's phase-3 close-out ledger closed; the checkpoint memo over this
    landing's SHA through `codex` (the user's steer), archived and
    answered in records §4 as every memo before it; phase 4 open, its
    first slice designed at the boundary. Slice 3.30 closed; phase 3
    closed.

65. **Slice 4.1, the native toolchain (designed 2026-10-13 at phase 3's
    close; §8.6 "Slice 4.1"; law §9.1 route 1, §12.4 item 4; for
    ratification before landing 1):** the leans taken in the design.
    (a) Phase 4 opens with this slice, ahead of law §12.4 item 4's
    list: route 1 is the engine the compiler proof is about, and the
    turnaround it buys (a load's 16–40 s interpreted; the index 14×
    faster native) is what every later slice and every author pays
    today; the alternative, the compiler proof first on today's two
    binaries, stated. (b) The pre-pass is the V3 loader's projection
    (`--emit`, rule 1), so the native route is built from what the V3
    loader accepted — item 49's one front end — rather than from the
    bootstrap's flat read, and rather than teaching v2's reader V3's
    records and capitalised locals (an old-tree edit under the corpus
    gate; item 42's cost dissolves in the emitter instead).
    (c) CI keeps the bootstrap suite as the authority and adds the
    native run compared to it byte for byte (rule 4); the alternative,
    the native suite alone with a subset tie, is cheaper in CI minutes
    and weaker, since a miscompile outside the subset would hide a
    kernel bug until the next bootstrap run. (d) The binaries are
    engines, route 1's proof still to come (rule 6). (e) The old tree
    untouched; a needed old-tree change is a ruling (rule 5). (f)
    `Symbol`'s `String` flip stays carried (slice 3.30 rule 3b): the
    chain remains v2's. **Amended 2026-10-14 by R84 (records §4.12;
    slice 4.0 landing 0): rules 1, 3 and 4 and landing 1 as they now
    read, rule 8 added — the comparison boundary, no silent fallback,
    the root inventory and the stamps, the discriminating cases; point
    1 of R84 was the design as committed.** **Ratified 2026-10-14 as
    amended (the user, on the report of slice 4.0 closed on pipeline 603:
    "Good. Call it ratified and continue.").** **As built, landing 1
    (2026-10-14):** the emitter, the build and the tie as §8.6 "as built,
    landing 1" states them; the departures stated there — per-root
    emit directories, the tie in its own CI job, the pins' programs
    under `kernel/` (the toolchain profile keyed on the module path), `t0`
    and `initindex` on the emitted path from this landing. **As built,
    landing 2 (2026-10-14):** the shim, the suite switch, CI's comparison
    and the measurements as §8.6 "as built, landing 2" states them; the
    departures stated there — the bootstrap's expression mode named in
    `calc_test` as the bootstrap's rather than passed through by the
    shim, CI's comparison a job of its own over both suites' kept
    outputs, the suite comparison's normalization list (a count of
    seconds, a size, a scratch name) stated in `suite_tie.sh`, the
    native run explicit locally. **Slice 4.1 closed 2026-10-14; on the full gate, pipeline 610 green on `2b2cde0`.**

66. **Slice 4.0, the checkpoint memo answered (designed 2026-10-14 at
    phase 4's opening; §8.6 "Slice 4.0"; records §4.12; for
    ratification before landing 0):** the dispositions of R78–R85 as
    records §4.12 proposes them — all eight accepted, R84's premise
    amended — and the leans: (a) the slice before 4.1, as slice 9 came
    before phase 3; the alternative, 4.1 first for the turnaround with
    the byte-tie agreeing with the defects, stated. (b) The release
    gate's exit an explicit conjunction (rule 2). (c) The fixed target
    a manifest the store writes beside `order` — kinds, names,
    fingerprints, the parameters, the discharges — and the policy an
    assumption-closure check through K's axiom walk, a conditional
    bundle verifying only as declared (rule 3). (d) The seal's admit
    flag replaced by a checked transfer that re-derives identity rather
    than trusting a Boolean, the pins flag by a generator run that
    yields no environment (rule 4). (e) R82's identities and R85's
    origin map carried as §11 rows, not built here. **Ratified
    2026-10-14 as designed (the user: "Agreed, go ahead with 4.0").**
    **As built, landing 1 (2026-10-14):** the manifest beside `order`
    (`decl`, `param`, `discharge` lines) the fixed target, the
    verifier's exit the sum of fourteen failed classes, the kind and
    cardinality checks, the policy through `axioms_of` with verified
    discharges standing for their proofs' closures; the memo's seven
    tampers as pins; the memo's second cardinality case lands as
    `MISTIED` against the manifest, which fails the gate as asked.
    **As built, landing 2 (2026-10-14):** the checked transfer
    `env_transfer` in place of the admit mode, the admit and pins flags
    deleted outright (no generator run type: the generator needed no
    privileged mode), the pin `k_client_admit` and seven client cases;
    the names pinned after the whole transfer (found: `String`
    unpinned when pinned one by one). Slice closed.

67. **Slice 4.2, the tb_len rung (designed 2026-10-15 at slice 4.1's
    close; §8.6 "Slice 4.2"; law §12.4 item 4, §4.9, §12.5; ratified
    2026-10-15 as described — the user: "That all makes sense -- agreed. Let's proceed with the tb_len rung as described."):**
    the leans beyond the description, stated for objection. (a) The
    item read as Theorem B at one function restated in V3 — not a
    proof of v2's chain, not route 1 proven (phase 7); the
    alternative, route 1's own chain first, stated at the boundary
    and declined: phase 5 and 7's bulk, which `tb_len` does not need.
    (b) `len` first, `app` second, `perim` not built; the alternative,
    `app` first for the twin, declined — `len` needs no runtime law
    and no twin, so its number is the clean one. (c) The closure only,
    133 definitions, not the manifest's four files; the rest at phase
    5. (d) The product as data; `impc` not ported. (e) Predicates as
    `Prop` definitions and words as `Nat` with the modulus explicit;
    the alternatives — Bool predicates with extraction laws as the old
    tree has them, words as `Int`, or `UInt64` — stated: the first
    re-creates the provocation, the second keeps the `0 ≤` premises
    the typed class removes, the third is phase 5's word line. (f)
    The evaluator re-spelled as one function over a task type, since
    Stage 1 leaves a mutual group `RUNNABLE`; the alternative, mutual
    structural recursion in Stage 1, a Stage-1 item for the
    toolchain's own port. (g) The kit laws by need, not one for one.
    (h) The hand port ahead of the migration tool, every re-spelling
    recorded. (i) The certificate stored and verified by K alone at
    landing 1 (T8's first item). (j) The count of rule 6 with the lean
    stated and no pass bar: the law asks for the count, not a
    threshold; the user may set one before landing 1. **As built,
    landing 0 (2026-10-15):** the port, the definition gate and the
    count as §8.6 "as built, landing 0" states them; two Stage-1 gaps
    found and closed — a nested inductive's `casesOn`, a `def` that
    recurses — each a rule with its pin. **As built, landing 1
    (2026-10-15):** the proof through I as §8.6 "as built, landing 1"
    states it — the run lemmas each a K check of its own, the step laws
    the vocabulary, the count written beside the record's numbers; lean
    (i) kept (the root stored and verified by K alone in `imp_test.sh`);
    the kernel's interventions the reducer's three rules, the loader's
    named exhaustion and the reader's bounded pending record, K
    untouched; one backlog item (issue #42, the unreachable arm). **As
    built, landing 2 (2026-10-16):** `app` as §8.6 "as built, landing
    2" states it — the output ghost and word existential, no twin (rule
    5's test answered for the per-function certificate; the frame a
    caller needs noted for phase 7); the runtime's laws as six
    parameters, read against C2b's slicing (the allocation law the
    raw-cell protocol — the call, the fill, the seal; the increment; no
    release, `app` never calling `dec`), the run laws restated with
    their memories existential and their representation and invariant
    laws folded in, the fill and seal laws one to one — *for
    ratification*: that reading of rule 5's "three laws", and the
    restatement rule; the loader's record of a trusted axiom as `PARAM
    NAME axiom` (the mechanism rule 5 named reached); the count written
    beside the record's numbers; the general form split at K's
    recursion depth, the existential introduced through `apply` (a
    Stage-1 gap recorded); K untouched. **Ratified 2026-10-16 as built,
    both readings (the user: "That makes sense. Ratify the two readings, close the slice, and let's open the next one.");
    slice 4.2 closed.**

68. **Slice 4.3, the program side (designed 2026-10-16 at slice
    4.2's close; §8.6 "Slice 4.3"; law §4.7, §9.3, §12.4 item 4;
    ratified 2026-10-16 as described — the user: "The program side
    sounds good as described -- proceed."):** the leans beyond the
    description, stated for objection. (a) The World's L meaning as
    landing 1's first item — an extern an opaque constant of its
    declared type, so that a function calling one has equations and a
    requirement can name the entry; the alternative, requirements over a
    `sig` twin of `main` as the old contracts had, declined: the entry
    itself is the subject. (b) Route 1's binary of the bin at landing 2
    beside the trace, not at landing 1 as described: the trace relation
    is the binary's gate, so the two land together; landing 1 runs the
    bin on both engines through the driver. (c) The check over `Prog`
    after erasure with per-function summaries in dependency order, the
    pair of an extern's result the one supported aggregate beside a user
    constructor, everything else refused conservatively; the verdict a
    record on every extern-reaching function at landing 0 (the kernel's
    drivers measured), a refusal on a bin's closure from landing 1. (d)
    The `bin` in its own file beside the program; its record, the
    manifest's `bin` line, the gate's `BIN`/`BAD-BIN`; the threading
    verdict the loader's, re-derived at every load. (e) Calc's contract:
    three observables, nine bolts, two requirements (`main_output`
    through the model and 3.21's claim, `main_reads`), the old principle
    kept — requirements over the entry's observables only. (f) The trace
    as the driver's record (`--trace FILE`) and the model's as a
    function, tied four ways. (g) The model test as the bundle
    instantiated at the model world and proven, plus a ground `decide`.
    (h) The count with no pass bar; `bin_test.sh` the 43rd entrypoint.
    **Landing 1 built 2026-10-16** (§8.6 "as built, landing 1"): (a)
    as described, the opaque's value the declared type's default
    witness (`inhabit.shard`, Lean's `default`); (d) as described, the
    bin's policy admitting the load's parameters as the file's does;
    (e) as described, the three `get_args` bolts in the premise form;
    (h) `bin_test.sh` the 44th entrypoint under `V3_NATIVE=1`, the
    native tie being the 43rd. **Landing 2 built 2026-10-16** (§8.6 "as
    built, landing 2"): (b) **not as leaned** — route 1's binary of the
    bin is not built: the emitter refuses calc's closure (a string
    literal over Init's `List.cons`, the primitive `Nat.sub`), the
    chain's representation of Init's types the gap; the lean now, for
    ratification, is phase 5's with the chain's replacement, the tie
    three ways (the bootstrap's driver, the native loader's, the
    model's); (f) and (g) as described, the ground case computed rather
    than decided (no decidable equality at a user type). **Ratified
    2026-10-16 as built, the deferral at (b) (the user: "Option 1 sounds
    like the way to go."); slice 4.3 closed.** **Reopened 2026-10-16 at
    the close (item 69, ratified): the deferral stands for the proven
    binary only — the binary of a naming-law program comes at slice 4.4
    through the bridge, since V3's own chain is itself a naming-law
    program and nothing else compiles one.**

69. **Slice 4.4, the bridge (designed 2026-10-16 at slice 4.3's close;
    §8.6 "Slice 4.4"; law §9.1 route 1, §12.4 as amended 2026-10-16;
    ratified as outlined — the user, on the three changes: "Agreed with
    the plan changes as outlined; update the docs."):** the three law
    changes and the leans. The occasion: the user's question at the
    boundary, "Is there any of this ordering you think we should
    rearrange or reprioritize? Anything likely to become a
    chicken-and-egg problem?" (a) **Item 68's deferral reopened the day
    it was ruled, on evidence.** V3's own chain is a naming-law program
    — its AST and machine types are the ported models' under
    `v3/models/imp`, on Init's `Nat` and `std/list` — and the only
    compiler of naming-law programs is the chain being replaced, which
    refuses them (slice 4.3 landing 2); the bootstrap interprets, at
    hours per generation of the loader's closure. Chose: the bridge now
    — Init's types onto the chain at the emitter, no edit of the old
    tree — so that calc's binary, phase 5's effectful ports and the new
    chain's first generation compile; the deferral stands for the
    proven binary (phase 7, with V3's own chain). Rejected-because, of
    the day's option 1 as ruled: it priced the mapping as days on the
    old tree, where the two refusals are emitter-side (`em_check_lit`,
    `em_check_term`) and every inductive of the closure is emitted
    already (`em_decl`'s `EInd` arm). (b) **The flip decoupled from the
    chain's replacement.** Law §12.4 items 5 and 6 and the manifest's
    row archived the temporary chain at the flip with "route 1
    replaces it", which serialized a tree move behind the arc's
    largest build; now the temporary chain, behind the V3 emitter, is
    route 1's engine through the flip and is archived when V3's own
    chain compiles itself — phase 7, with the per-function generator
    (4.1 rule 6 and 4.2's "not in this slice" read the same way now).
    (c) **The five "until route 1's chain is V3's own" debts
    re-attributed.** Since slice 4.1 the chain reads the emitter's
    output, never the kernel's sources: the comparison spellings'
    rename across `v3/kernel/**` (3.17 rule 5) is blocked by nothing —
    the emitter prints the chain's spellings from the program and the
    Rust bootstrap reads both — and is this slice's landing 2, by the
    tool; the `by` form in the kernel's theorems is the Rust reader's,
    enduring; the string flip for the toolchain's sources (item 51),
    `derive` there (3.19) and `Symbol`'s `String` flip (item 64) need
    Init's types in the toolchain's closure — phase 5's opener design
    beside the migration tool, with one constraint: the index driver's
    closure stays Init-free, since it builds the index that makes Init
    loadable —; the binary of a naming-law program is the one debt
    that was the chain's, and the bridge pays it. The leans of the
    design (§8.6): landing 0 a probe as the kill-test before any build,
    the slice re-designed there if a primitive has no expression;
    landing 1 the bridge, the pins flipped, calc's binary tied on its
    observables (stdout and exit status against both drivers; the
    drivers tied to the model on the trace as at 4.3 — the chain's
    runtime gets no trace, the binary's trace relation being phase
    7's); landing 2 the rename; T6 after this slice; the sizing of
    V3's own chain a memo at phase 5's opener. The count agreed before
    the run, no pass bar (§8.6 rule 5). **Landing 0 built 2026-10-16**
    (§8.6 "as built, landing 0"): the probe passed its kill-test — calc's
    binary through a scratch bridge ties the driver byte for byte — and
    found the bridge's mechanism to be the identification of Init's four
    wire types with the chain's prelude (the shims and the old reader
    force it) plus a type map, the literal rewrite of the design not
    needed; the dotted primitives reached are `Nat.sub`, `Nat.div`,
    `Nat.mod`, `Int.ediv` and `Int.emod`, each with an expression; the
    bootstrap's flat read is not a naming-law program's reference.
    **Landing 1 built 2026-10-16** (§8.6 "as built, landing 1"): the
    bridge in the emitter as the probe found it — the identification,
    the type map, the seven expression rows and seven by name, `absurd`
    the no-match sink —, the two pins flipped (`init_prim` 21 rows), calc's
    binary in `bin_test.sh` and the tie against the driver's run; rule
    2's literal rewrite not needed, the `gcd`/`pow` helpers not built.
