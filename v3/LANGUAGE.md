# The V3 language — S, L and E at Stage 0 (phase 2 draft)

> **STATUS (2026-10-10): the design of S, L, E and I as phase 3 builds
> them — §13 items 1–38 ratified 2026-09-15, items 54–56 ratified
> 2026-10-08, items 57–60 ratified 2026-10-09 (59 and 60 as amended
> at slice 3.26 landing 3), item 61 ratified 2026-10-10, each later
> item for ratification as its slice lands.** Normative parent:
> `docs/FOUNDATION.md`. Scope: the surface S, the executable fragment E
> and `ev` as phase 2 built them — the reader (§2, §4–5), the loader
> (§3), views (§6.5–6.6), the classifier and `ev` (§6.2–6.4, §6.7),
> `realize` (§7), the one E (§8, ruled and built 2026-09-14), entries
> (§9), conformance (§10). Each semantic rule has one current statement here with its
> Stage-0 limit beside it; a later "as built" section fixes what the
> rule left to the implementation and never overrides it (GPT-6 R62,
> slice 10). The build history — which slice built what, the
> measurements, the findings — is `docs/records/FOUNDATION.md` §9; the
> review correspondence is its §4. The proof IR **I** is §8.4's since
> slice 3.20 (law §7). What Stages 1–3 add is §11 with its phase;
> §12 is the ledger of changes from v2, the AT RISK rows the ones to
> watch. Decisions this draft makes beyond the law's text are §13;
> until ratified they are the implementation's working assumptions,
> not law. This document supersedes `docs/LANGUAGE.md` for the `v3/`
> tree (FOUNDATION §10.5); the old document keeps describing the old
> tree until the flip.

The normative contract is `docs/FOUNDATION.md`; this document
specifies the surface and the executable fragment as phase 2 builds
them and does not restate the law's reasons. K's data — names, levels,
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
`import use extern requirement fulfills trusts measure`. Reserved for
later phases and refused with the phase named: `by` (phase 3), `bin`
`requires` `app` (phase 4), `instance` `class` (phase 3), `namespace`
`section` `variable` (undecided; the dotted declared name covers the
common case).

| form | phase-2 meaning | goes to |
|---|---|---|
| `(import …)`, `(use …)` | §3.1 | the scope |
| `(inductive NAME.{u…} BINDERS TYPE (CTOR TYPE)…)` | Lean's `inductive`: level parameters on the name, parameter binders, the type after the parameters (`(Sort …)` or a `forall` over indices), each constructor's full type after the parameters | K: `InductDecl` |
| `(type (NAME T…) (CTOR FIELD-TYPE…)…)` | today's E data form, sugar for an `inductive` with parameters `T : Type`, result `Type`, non-dependent fields; E-eligible by construction | K (`InductDecl`) **and** E (`EInd`) |
| `(record NAME (ctor CTOR)? (FIELD TYPE)+)` | named-field products (v2's form, docs/LANGUAGE.md "Records"; slice 3.9, §13 item 42): loader-level sugar expanded before any form is read — the positional `(type NAME (CTOR TYPE…))`, CTOR defaulting to `MkNAME`, an accessor `NAME.FIELD` and an updater `NAME.with_FIELD` (value first) per field in the record's namespace, which the file opens as any type's (item 7): `(FIELD r)` bare where no other opened record has the field, `(NAME.FIELD r)` always; NAME may be `(NAME P…)`, the generated fns then binding `(P Type)` first. `(make NAME (FIELD V)…)` — every field exactly once, order-free — and `(with NAME E (FIELD V)…)` — chained updaters, a later entry outermost — both against the current file's records, rewritten in every form of the file, nested values first (`kernel/record.shard`; the bootstrap's `expand_records` the twin, parity the tie). No law family is generated: since Stage 1 the accessors and updaters are definitions and v2's laws are `Eq.refl` over a local (pin `record_laws`). `make` and `with` also reach a parameterless `(structure NAME () (FIELD TYPE)…)` of the file — `NAME.mk` on the values in field order, on each field's entry or its projection of `E` for `with`; a field kept whose type mentions an updated one is the refusal `dependent_update` (§8.4 slice 3.17 rule 4). Layout: as `type`'s until CANON's phase-6 gate | E: as the forms it expands to |
| `(structure NAME.{u…} BINDERS (FIELD TYPE)…)` | one constructor `NAME.mk`; projections `NAME.FIELD` generated as `def`s over `proj` (Lean's projections are `Expr.proj` definitions); fields dependent on earlier fields | K |
| `(def NAME.{u…} BINDERS TYPE VALUE)` | an L definition; hints `regular` at height 1 + the greatest height it references | K: `DefnDecl` |
| `(abbrev …)` | as `def` with hint `abbrev` | K |
| `(opaque NAME.{u…} BINDERS TYPE VALUE)` | checked, never unfolded | K: `OpaqueDecl` |
| `(theorem NAME.{u…} BINDERS PROP PROOF)` | PROOF is `(exact TERM)` or `sorry` at Stage 0; `sorry` is reported loudly, admits nothing, and every citation of the theorem is a pending obligation, never an assumption | K: `ThmDecl` |
| `(axiom NAME.{u…} BINDERS TYPE)` | admitted only under `(trusts NAME)` in the same file (§9) | K: `AxiomDecl` |
| `(fn NAME BINDERS RET (measure M)? BODY)` | an E function: parameters are E-types, BODY is §5.4's E term language; classified (§6.3); **no L declaration at phase 2** (§0); its definition and equations at Stage 1 (§8.4) | E: `EFn`; K: `DefnDecl` + `NAME.eq_N` when eligible (§8.4) |
| `(extern NAME BINDERS RET)` | a World extern (law §4.7) | E: `EExtern` |
| `(sig fn NAME BINDERS RET)`, `(sig type (NAME T…))` | a view's bodyless signature and opaque type (§6.5) | E: `ESig`, `ESigType`; L: a view parameter |
| `(requirement NAME BINDERS PROP)`, `(fulfills NAME PROOF)` | a view's promised law and its discharge in the implementation (§6.5) | L |
| `(realize NAME …)` | an E body for an admitted L constant (§7) | E: `EFn`; L: the equations |
| `(derive TYPE CAPABILITY…)` | since slice 3.19 (§8.4): equality, an ordering and a rendering for a closed inductive type — `eq`, `(ord structural)`, `render` generated as `fn` forms the loader reads at that point; `(CAPABILITY by NAME)` registers a procedure of the file's. Never in a view | the generated `fn`s as any `fn`; the derivation table (`DERIVE`) |
| `(trusts AXIOM…)` | widens this file's assumption policy (§9) | policy |

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
  requirement is `fulfills_unknown`. After the view's forms, the
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
directory). R45's third test is the pair: the same body as a `fn` is
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
   identity begins with `kernel` until route 1's chain is V3's own. `(quote x)` and `'x` are `Symbol` literals; `(list a b
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
| 3.13 | `fn` = `def` + `realize` for non-recursive bodies and immediate-field structural recursion; the operator identities by operand type; a `def` in the E forms | `examples/calc`'s first claim; `v3/std/list.shard` under the naming law (R55's small library: an operation and a theorem) |
| 3.14 | deeper structural recursion by course-of-values (`below`/`brecOn` generated per inductive, as Lean's elaborator does); `(measure E)` to `WellFounded.fix`, the measure a reported obligation; mutual recursion | every ported `fn` with a measure — a quarter of the old tree's 13,229 |
| 3.15 | the L-side ergonomics of law §5.1–5.2: implicit arguments, universe inference, the numeral rule with the one `Nat → Int` coercion, `noConfusion`/`injection` | `def` and `theorem` authors |
| 3.16 | one front end (reordered 2026-09-18; the design below): a `fn` body through the elaborator into a pre-definition, its E program by erasure and its K value by the 3.13–3.14 compilation; matchers, so a `match` in a statement; `Bool` against `Decidable`; constructors by expected type; list literals | calc's spec file wholly defined (13 of its 25 functions are `RUNNABLE` at the slice's opening) |
| 3.17 | the porting facilities of §11's R60 row (narrowed by ruling 2026-10-01, item 50): the record laws and `make`/`with` over a `structure` with the dependent-update refusal, numeral rows on the typed route, the fresh-name supply, E's rename of `lt le int_eq` to `< <= =` staged by the tool (the files no v2 tool reads now, `v3/kernel/**` with route 1's chain) | the broad port |
| 3.18 | bytes and text (narrowed by ruling 2026-10-01, item 51; the design and the as-built below): the registry's type rows — `String`, `ByteArray`, `UInt8` and the structures under them, each represented by its one runtime field —, a projection function as its projection, item 37's flip staged (every file but the toolchain's own), the wire read by declared type and the externs over `ByteArray`; symbols and `Name` literals deferred to the toolchain's port | the first host-facing S library (`v3/std/host.shard`, `v3/std/bytes.shard`); calc's loop against the host |
| 3.19 | deriving under a declared policy (law §5.1; split from 3.18 by the same ruling; ruled 2026-10-02, item 52; the design and the as-built below): `(derive TYPE CAPABILITY…)` — equality, an ordering under the one policy `structural`, a rendering in canonical S — generated as `fn` source and found through a derivation table; a hand-written procedure registered with `(CAPABILITY by NAME)` | a new type usable in an `if`, as a key and in a message (`v3/examples/derive/`); `v3/std/derive.shard` |
| 3.20 | I's opener (ruled 2026-10-05, item 53; the design and the as-built below; **landed 2026-10-06**): I's data and `(by STEP…)` in a `theorem`, `elaborate(I)` to P, the forms with no engine behind them — `intro exact rfl have show apply cases induction wf decide unfold reduce rw sorry` — and `goal_of`/`applicable`/`step` as E functions over one derivation | calc's claims whose lemma closure needs no arithmetic (52 of 100; 44 landed as blocks in five files, `kernel/test/tactic_test.sh`) |
| 3.21 | the two forms that carry an engine (the design and the as-built below, item 54; **landed 2026-10-06**): `simp_only` (a bounded rewriter over a lemma list) and `arith` (the Farkas certificate elaborated through `Lean.Omega`'s lemmas, in the export at line 76,223; reconstructed by elimination where the node gives none) | calc's 100 claims as theorems, the capstone `run_eq_spec` among them (`kernel/test/tactic_test.sh`) |
| 3.21b | GPT-6's trajectory review, findings 1–3 (ruled 2026-10-06, item 55; the design and the as-built below; **landed 2026-10-06**): the expected type decides an arithmetic operator; Init's visibility is the module's horizon; `reduce` succeeds unchanged where nothing reduces | three pins; `op_expected` proves `(- 1 2) = -1` at `Int` |
| 3.22 | the producers (ruled 2026-10-07, item 56; the design and the as-built below; **landed in three landings, 2026-10-07/08; closed on pipeline 552**): `auto` with sidecar replay at build and search only in `prove`; `(arith only …)` and the witness goal; the engine as an E library over the fixed API; the pin store in K's export format with `verify_release` | the engine's count over calc's 100 claims; `v3/examples/auto/` with a machine-owned sidecar; calc's release bundle verified on CI |
| 3.23 | the Init cache (ruled 2026-10-08 on GPT-6's R75, the feedback cost; item 57; the design and the as-built below; **landed 2026-10-08; closed on pipeline 554; retired at slice 3.26 landing 3, 2026-10-09** — a load checking the closure it cites, the receipt saved a tenth to a fifth of it): the loader admits the pinned export under a T0 run's receipt, the typing judgments skipped — K's verdict cached, no second format, the release gate untouched | a load's Init cost and the loader pins' wall clock |
| 3.24 | the connected path (ruled 2026-10-09 on GPT-6's R76; item 58; the design and the as-built below; **landed 2026-10-09; closed on pipeline 556**): `(dif h C T F)`, the dependent if with its hypothesis named — the branch-local proof joint; `examples/path`, law §12.4's first connected path assembled as one test and broken at each joint; its cost measured | the one phase-3 joint still missing; the composition cost before the broad port |
| 3.25 | the shared Init load (ruled 2026-10-09 at the boundary after R75/R76, item 57's stated lever; item 59; the design and the as-built below; **landed 2026-10-09; closed on pipeline 559**): the Init stream replayable — streamed once to its end, every root of a process loaded fresh on it; the module's horizon the only measure of what Init it sees, Init's names Init's; the loader pins' entrypoint from 783 s to 33 s | the suite's wall clock before the library arc multiplies the pins |
| 3.26 | Init on demand (ruled 2026-10-09 at the boundary after 3.25, on the library arc's first question; item 60; the design below; **landings 1–2 landed 2026-10-09, green on pipelines 561 and 563; landing 3 landed 2026-10-09 — the receipt dropped, the user's ruling; closed on pipeline 565**): the export indexed once per environment — a record table and a name table, read by range, never loaded —, a module's closure of what it cites read and fed to K in export order, the horizon law unchanged, Init's names Init's from the index, no fixture | every citation of Init costs its closure, under one percent of the export; the library arc cites Init wherever Init has the statement |
| 3.27 | the library arc (designed 2026-10-09 at the boundary after 3.26, on the user's "continue with the next arc"; item 61; the design below; **ratified 2026-10-10 as designed, with calc's `list.shard` retiring onto `std/list` at landing 3 and the migration tool's tier 0 moved to phase 5; landing 1 built 2026-10-10, green on pipeline 570**): Init is the library — a statement Init has is cited by its name, `v3/std` declares what Init lacks, an old module whose declarations are all Init's becomes a migration record; the fifteen former axioms as theorems (twelve by Init's names, measured); the seams the probes found closed — one spelling at the unifier and the matcher, instances by table, `/` at `Int` Euclidean, the horizon refusal's pointer; `std/list` with the dependent match and the matcher, `std/bits` as bridges; `docs/LEAN.md`; T9 small; T2/T3 and T10 as slices 3.28 and 3.29 | the bulk port cites this library; calc's `list.shard`; T9's author |

**The rules of slice 3.13**, decided here (the user's ruling of
2026-09-17 on the five leans; §13 item 43):

1. **Eligibility is automatic and transitive, as a `type`'s is** (§8.1
   rule 1). A `fn` gets its L half when every binder and return type
   has an L identity and every head of its body does — a constructor
   of an E-eligible inductive, a `fn` with a definition, a realized
   constant, a `sig fn` with an L parameter (not an E parameter, item
   40), a table entry with an identity under rule 3. Otherwise it stays
   `RUNNABLE` as today and the record names the first obstacle. Once
   eligible, a body that fails translation or K is a **refusal**,
   never a silent fall-back to `RUNNABLE`: a typo must not quietly
   degrade a definition to a body no theorem can cite. K's own sources
   import no `Init`, so nothing inside the seal changes at this slice
   (their `fn`s enter L at the flip, rule 1). **As built:** the
   obstacles are `e_only_type` (a binder or result type with no L
   identity here), `no_l_meaning` (a callee that is `RUNNABLE`, an
   extern), `no_l_identity` (an operator with no identity at its
   operand types, a string or symbol literal), `unknown_constant` (a
   constructor of an E-only type; a primitive's identity past the
   Init prefix), `measure_pending` (rule 4), `numeral_pattern` (slice
   3.15's) and `recursion_depth` (slice 3.14's; rule 2) — `RUNNABLE fn
   NAME why=OBSTACLE`; every other failure is `REFUSE NAME reason`
   through the classifier's record. The equations are stated only where
   `Eq` and `Eq.refl` are in scope (an Init prefix that ends before
   them gives `DEFINE NAME equations=` empty: the definition alone).
   The fn enters the E table before its definition is attempted, so a
   self-call types statically.
2. **The definition's value uses the recursor directly.** The binders
   are lambdas; the leading case tree (§7.5's `compile`) is nested
   `T.rec` with a constant motive, the reverse of the derived view's
   erasure of a recursor; a self-call on an **immediate** constructor
   field of the matched parameter is the induction hypothesis of the
   same recursor. A self-call on a deeper field (`List.concat` on `t`
   under `(cons x (cons z t))`) is `recursion_depth` at this slice —
   an **obstacle** (`RUNNABLE … why=recursion_depth`), since it is
   slice 3.14's shape and not an author's error (calc's `parse_rest`
   is one); a self-call not on a field of a matched parameter is
   `realize_descent` as today, a refusal. Nested and non-leading
   matches split by the recursor with a constant motive and open their
   hypotheses unused; a match on a variable whose constructor an
   enclosing split fixed is resolved statically (the matrix knows it),
   never a second recursor. The
   equations are `NAME.eq_N`, one per leaf as §7.5 states them, proven
   by `Eq.refl` — K unfolds the definition and iota-reduces the
   recursor on the constructor-headed left-hand side — and the body is
   attached through the realize path, so `ev` runs it under the L
   identity and the `REALIZE`/`PENDING` machinery (R58) is the same
   for every function. The record is `DEFINE NAME equations=NAME.eq_1,…`
   beside the `ACCEPT` of the definition and of each equation.
3. **Operator spellings get L identities by operand type, only where
   `ev` already agrees with K** (settles item 38): `+` and `*` at
   `Nat` are `Nat.add` and `Nat.mul`; `+`, `-` and `*` at `Int` are
   `Int.add`, `Int.sub` and `Int.mul`; `int_eq`, `lt` and `le`
   (`=`, `<`, `<=` since slice 3.17) are
   `Nat.decEq`/`Nat.decLt`/`Nat.decLe` at `Nat` and `Int.decEq`/
   `Int.decLt`/`Int.decLe` at `Int`. `-` at `Nat` is refused
   (`nat_operator`, pointer `Nat.sub`): `ev` computes the integer
   difference and `Nat.sub` truncates, and no dump could tie the two.
   `/` and `mod` at `Nat` are refused likewise (`ev`'s `/` is stuck at
   zero, `Nat.div` is total); at `Int` they have no identity (rule 5,
   unchanged). Bitwise and symbol entries have none. **As built:** the
   operand types are the classifier's static types, and `+ - *` on two
   `Nat` operands type as `Nat` statically (`prim_static`), so a nested
   `(+ n (+ n n))` resolves; the comparisons' static result type stays
   the prelude's `Bool` — the translation of an `if` reads the
   decision's inductive off K's type of `Nat.decLt a b`, so no flip
   was needed (`ev`'s cells unchanged, item 17). **Numerals** (law
   §5.2's one coercion): a non-negative numeral at an expected `Int`
   is `Int.ofNat n` and a negative one `Int.negSucc (-n-1)`, where
   `Init`'s `Int.ofNat` is in scope; at `Nat`, or with no expected
   type, K's `Nat` literal as before — so `(match xs (nil 0) …)` at
   `Int` states `Int.ofNat 0`; since slice 3.15 a theorem about it
   writes `0` too (§5.3).
4. **A `fn` with a non-structural measure stays `RUNNABLE`** at this
   slice, reason `measure_pending`; slice 3.14 gives it
   `WellFounded.fix`.
5. **A `def` in the E forms** is elaborated by the same `tr` with the
   declared type as the expected type; the explicit-L path is
   unchanged. **As built:** the reader's explicit-L path is tried
   first; when it fails on a `def`, the form is read as a `fn`'s
   signature and body by the classifier and defined the same way (the
   definition only — no E body, no equations); when that fails too the
   reader's own error stands. A body mixing explicit-L forms inside
   the E forms is not 3.15's either (its §5.1 `if` and operators are
   the L side's; a `match` in a statement stays refused). The pin
   `def_match` shows a `def` with a `match`. **Superseded at slice
   3.16 landing 1 (2026-09-19):** the elaborator reads `match`, `if`
   and a body's untyped `let` itself, so the fall-back through the
   classifier is **deleted** (`def_e_forms`; it had begun to hide the
   reader's own error behind a second reading) and `def_match` goes
   through a matcher.

**Not in 3.13:** nested and numeral patterns beyond the leading case
tree (`equation_form`, as for a `realize`), mutual recursion,
lambda lifting, any change to the reader.

**The rules of slice 3.14** (the user's ruling of 2026-09-17 on the
three leans; §13 item 47):

1. **One scheme for all structural recursion, Lean's own.** A
   structurally recursive `fn` is `T.brecOn` applied to a functional
   `λ x below . body` whose table `below : T.below motive x` holds the
   results at every sub-structure; every runtime split of a variable
   that owns a table generalizes the table in the split's motive (`λ
   x' . T.below motive x' → R`), so inside a branch the table's type
   reduces to the pair nest and each recursive field's entry — its
   value and its own sub-table — is a typed projection; a self-call on
   any such field, at any depth, is the entry's value applied to the
   call's other arguments. 3.13's direct-recursor path is deleted, not
   kept beside it. `Init`'s inductives use `Init`'s `below` and
   `brecOn` at solved levels; a native inductive gets `T.below` and
   `T.brecOn` generated as monomorphic definitions on first need,
   under Lean's names, admitted before the function (Lean generates
   them at declaration time and only for recursive types, which is why
   the export holds no `Option.below`). The equations stay `Eq.refl`:
   `brecOn` unfolds to the recursor and reduces on a constructor-headed
   argument. `recursion_depth` survives only for a self-call on a
   variable bound outside the leading case tree. **As built:** the
   table's shape is Lean's, read off the export's `List.below` and
   `Nat.below` — the pairs `PProd (motive r_k) (below r_k)` over a
   constructor's recursive fields, right-nested by `PProd` with no
   unit, one field's pair the table itself, no field `PUnit`; a
   field's entry is derived positionally (K's instantiation
   beta-reduces the motive applications in the reduced table type, so
   the field cannot be read off it), and K's check of the definition
   is what holds the shape to Lean's. `Init`'s `brecOn` goes through a
   `brecOn.go` helper the definition never sees. A generated pair
   (`T.below`, `T.brecOn`) is two `ACCEPT` records before the first
   function that needs it; a file without `Init`'s `PProd` in scope
   cannot have a table, so its structural recursions stay `RUNNABLE`
   (`below_unreached`) — K's own sources included, whose types are
   E-only first anyway.
2. **A measure becomes `WellFounded.fix`, the decreasing facts
   admitted obligations.** `(fn f PARAMS RET (measure M) BODY)` with
   `M` a `Nat`-valued E term over the parameters is `WellFounded.fix
   (InvImage.wf m (WellFoundedRelation.wf Nat.lt_wfRel)) F p` over
   the data parameters packed as a `PProd` nest `p` (a single
   parameter is itself), `m` the measure over the tuple and `F : ∀ p,
   (∀ q, InvImage … q p → RET) → RET` the body with each self-call
   `rec (tuple of the arguments) f.dec_N …`. Each `f.dec_N` is an
   **obligation**: an axiom-kind constant `∀ (the locals in scope at
   the call), InvImage Nat.lt_wfRel.rel m (tuple) p`, admitted before
   the definition as the **fourth policy class** beside view
   parameters (§9) — `PARAM f.dec_N obligation`; the function's
   record is `DEFINE f equations= pending=f` with `PENDING f measure`
   naming the obligations, every dependent's `params=` reaches them
   (R57/R58: nothing is claimed that is not visible), and a later
   `(fulfills f.dec_N PROOF)` discharges one as an implementation
   discharges a requirement (the form is I's, phase 3). No equations:
   `WellFounded.fix` reduces through `Acc.rec` on a proof, which K
   does not compute, so `f.eq_N` for a measured function is I's
   (`WellFounded.fix_eq`). An `Int`-valued measure is the obstacle
   `measure_type` with the pointer to a `Nat` measure — the law never
   inserts `Int.toNat` (§5.2). **As built:** the obligation is
   quantified over every local in scope at the call, in binding order
   — the type parameters, the tuple, the fields of the enclosing
   splits (each split's equation `scrutinee = pattern` after its
   fields, since slice 3.21: a matcher under a measure carries it),
   the `let`-bound values as `let`s — and the proof term
   applies the constant to the lambda-bound ones; the obligations a
   branch collects survive the branch's context being restored; the
   Int fixture runs through `InvImage.wf` so the pins reach it; a
   prefix that does not is the obstacle `wf_unreached`.
3. **Mutual recursion is an obstacle** (`mutual_recursion`: a callee
   pre-registered in the file and not yet defined), taken with slice
   3.15. A forward reference to a later-defined function is the same
   obstacle.

**The rules of slice 3.15** (the L-side ergonomics of law §5.1–5.2;
the user's ruling of 2026-09-17 on the leans, §13 item 48; built
2026-09-18):

1. **Metavariables are nodes** — `MVar` in `Expr`, `LMVar` in `Level`
   (law §6: native metavariables that K refuses). K gained refusing
   arms only: `infer` on the node is `Malformed metavariable`
   (type_checker.cpp l. 342), the raw entry refuses a declaration
   carrying one (`mvar_in_type`, `mvar_in_value`, beside the fvar
   checks), the node data carries `has_mvar` (levels included), and
   the exporter never emits one, so the import is unchanged. The
   first K edit since the seal (§13 item 26); hostile pins 17a–d.
2. **The elaborator is a Meta layer above K** (`kernel/elab.shard`
   over `unify.shard`, `scope.shard`, `kw.shard`): bidirectional, over
   K's locals, first-order unification with typed assignments, K's
   `whnf` and `is_def_eq` through the view for the closed residues —
   §5.1 states the rules. The reader (`reader.shard`) reads the forms
   over it; nothing of it crosses into K.
3. **The spelling changes it makes** (the phase-2 pins rewritten, 45
   files): an implicit binder's argument is inserted, never written —
   `(Eq.{1} Nat a b)` is `(Eq a b)`, `(Eq.refl.{1} Nat a)` is
   `(Eq.refl a)`, `(List.cons.{0} a x t)` is `(List.cons x t)`;
   `@NAME` where the old spelling is wanted. A `type`'s parameters are
   implicit in its constructors and a `fn`'s type parameters in its L
   signature, equations and obligations, so a theorem cites
   `(len xs)` and `(Pair2.mk a b)` as a body does (guard 1). A
   universe not written is inferred. A statement's `0` at `Int` is
   `Int.ofNat 0`, its `(+ a b)` at `Int` is `Int.add a b`, its `(= a
   b)` is `Eq` with the type inferred (`std/list.shard`, calc's
   `eval_add`). Two refusals moved from K to the elaborator: a false
   theorem's proof and a universe collapse are `type_mismatch` before
   K sees them (K's own refusals stay pinned in the hostile battery),
   as is a consumer's `Eq.refl` through a view parameter (§8.5's
   `view_rfl`: the unifier asks K's `is_def_eq` over the consumer's
   environment, where the parameter is opaque). `universe_args_required`
   is gone; `unsolved_universe` is its Stage-1 counterpart.
4. **`if` and the operators in a statement** are §5.1's: `ite` with
   the decision of the proposition's head, the identities by the
   first operand's type, `-` `/` `mod` at `Nat` included in L.
   **Not in 3.15:** a `match` inside a statement (the pointer is
   `T.casesOn` or a helper `def`), `Bool` against `Decidable` bridging
   (`decide`; calc's `is_digit` stays `RUNNABLE` for it, slice 3.16),
   E's rename of `lt le int_eq` to `< <= =` (3.17 with the tool
   migration), `UInt*`/`Fin` numerals (with their E realizations),
   explicit holes `_`.
5. **The constructor-structure bundle is generated after a native
   inductive's admission**, in Lean 4.33's shapes read off the export
   (`define.shard`'s `confusion_bundle`; `loader.shard`'s
   `bundle_after`): `T.casesOn` (the recursor with the hypotheses
   unused, universe polymorphic) for every native inductive whose
   recursor eliminates into every universe; and, for a type without
   parameters, indices, universe parameters or dependent fields, where
   `Init`'s `Eq`, `Eq.refl`, `Eq.ndrec`, `And` and `And.intro` are in
   scope, `T.noConfusionType` (two nested `casesOn`s: the same
   constructor gives `(∀ f_eqs, P) → P` with `Eq` on each field pair,
   different ones `P`), `T.noConfusion` (`Eq.ndrec` over a `casesOn`
   whose minors apply the continuation to `Eq.refl`s) and `T.c.inj`
   per constructor with fields (`Eq (c fs) (c fs') → And …`, a term
   proof through `noConfusion`, the fields implicit as Lean's). Each is
   an `ACCEPT` record K checked; where the conditions fail nothing is
   generated and the type stands (K's own sources: no `Eq`, as Lean's
   Prelude before it). A parametric type's shape — the parameters
   generalized twice, `HEq` on the fields, `eq_of_heq` — is a later
   slice's; `injection` is I's tactic over these. A theorem cites
   `(Color.noConfusion h)` at `False`, `(Tree.Node.inj h)`, or
   `(Tree.casesOn t …)` with the motive found from the expected type
   (a Miller pattern `?motive t` in the unifier, the bound-variable
   arguments elaborated before the propagation as Lean's
   `elabAsElim` orders them). Pin `elab_confusion`.
6. **A forward reference is parked, never an obstacle** (3.14 rule 3
   revised): a `fn` whose callee is not yet loaded (`mutual_recursion`
   from the translation) or whose callee is itself parked
   (`callee_pending`, the callee named) enters the E table and waits
   with no record; after every definition the parked ones are tried
   again to a fixpoint; at the file's end the ones still parked are
   `RUNNABLE` with the obstacle they waited on — a true mutual pair
   stays `mutual_recursion` (pin `define_mutual`), a fn behind a
   `RUNNABLE` callee `no_l_meaning`. A `realize` never waits (it is
   its own declaration's): a pending callee is `no_l_meaning` there.
   Pin `define_forward`. Mutual structural recursion proper (Lean's
   packed `brecOn`) has no consumer yet and stays out.

**As built (2026-09-18, the elaborator):** `unify.shard` — `MCtx`
(declarations with name, type, telescope and kind; assignments;
`inst_mvars`/`inst_level` the instantiation; `unify` with `assign`
typed through K's `infer` on a closed value and `infer_light` — a
constant, local or metavariable head's telescope walked — on an open
one; `mc_drop_fvars` at every binder close, so a metavariable that
survives a `fun` cannot later capture its variable). `elab.shard` —
`El` (K's world, the metavariable context, the scope context with the
bound names and their fvars), `elab` returning term and type, `tele`
(the telescope into slots: an inserted metavariable or a written
argument's placeholder), `elab_args` (propagate, elaborate, numerals
last, instantiate), `check_expected` (unify, else the coercion, else
`type_mismatch`), `elab_op`/`op_identity`, `elab_if`/`decision_of`,
`el_finish` (the closed term; the first unsolved metavariable named).
The pins `elab_implicit`, `elab_numeral`, `elab_unsolved`,
`elab_instance`, `elab_no_decision`, `elab_mismatch`; the reader pin
`refuse_unsolved_universe`.

**Slice 3.16 — one front end: the pre-definition and its two
projections** (DESIGN, 2026-09-18, revision 2 the same day after
GPT-6's single-frontend memo R63–R71, records §4.10; the user's
rulings of that day on the reorder, the shape and the memo's leans;
§13 item 49; built in landings, each with its as-built below).

*Why.* Slice 3.15 built the elaborator law §5.1 describes, and left a
`fn`'s body on the older route: the classifier types it with E's own
first-order typing (`c_type`), and `realize.shard`'s `tr` translates
the E term **up** into L. The law's direction is the other one (§2,
§4: E is a fragment of L; the classifier is the *fragment*
classifier, a Stage-1 item after elaboration). The upward route
exists because the Stage-0 ruling of 2026-09-07 made `fn` E-only, so
E needed a reader and a typing before any L existed for it. Guard 1
above was written against `tr` and was met in wording only. The
symptoms are two type systems and two resolvers: at the slice's
opening calc's spec file has 12 functions defined and 13 `RUNNABLE`,
from three roots — `is_digit` and `is_ws` (a comparison is a `Bool`
in E and a proposition in L), `digit_val` (`(- c 48)`: a literal has
no type in `c_type`, so the operator finds no identity) — and `Add`
names calc's constructor in a `fn` body and is `ambiguous_name` with
Init's class in a `def`.

*The probe* (2026-09-18; a scratch file through the loader with
`--dump`, each function once as a `fn` and once as a `def` with the
derived view of §7.1). A non-recursive `match` (`flush`): the two E
programs byte-identical, the two L values one hash. Structural
recursion: K's value is `brecOn` and §7.1 refuses it, as §7.3 said it
must (Lean compiles its pre-definition, never the kernel term); the
hand-written pre-definition — `casesOn`, the self-call a constant —
erases to the classifier's program up to the primitive's spelling
(`Nat.add` for `+`). At `Int` the erasure refuses `Int.add`: the
operator table has no inverse. Nested patterns (`parse_rest`): the
erased `casesOn` tree means the same by reading (not run) and is not
the program — four arms become a tree with the fall-through eight
times. So neither extreme: E cannot be read off K's finished term,
and L should not be translated up from E.

*The invariant* (R63's wording, adopted): resolve and type the
intended computation **once**; preserve the structure needed to
justify it and to execute it; derive the mathematical and the
executable representation from that common construction. Neither
output is reconstructed from the other after information has been
discarded. The common input is not itself a proof that the two
projections correspond: the erasure rule's implementation stays a
stated bring-up trust dependency (§7.1, R58) until a checked
translation or route 1's proof covers it.

*The shape.* S is elaborated once, by `elab.shard`, into a
**pre-definition** — a named record (`PreDef`), not a bare body
(R63): the declared identity and type; the binder telescope (K's
locals); the body, an ordinary `Expr` over them; the **self-local**
when the function is recursive (its name bound to a local of the
declared Pi type); the **matcher descriptions** the body's matchers
were generated from (rule 1); the **callee-locals** standing for
`RUNNABLE` callees (rule 6); the obligations owed; the resolved
environment it was elaborated against. A pre-definition is never an
admitted declaration: a body well typed *assuming* a local for its
own name is provisional until the K projection admits it, and no
operation takes a `PreDef` where a `ConstantInfo` is wanted. Two
projections consume the record — neither re-resolves a name or
re-reads S:

- **toward E** — erasure (§7.1's rule, generalized): types, proofs
  and erased binders dropped, a matcher application back to the
  `EMatch` of its description, `ite`/`dite` to `EIf`, the self-local
  and a callee-local to the `ECall`, a resolved operation to its
  realization (rule 4). This **is** the function's program: for a
  function on this route the classifier's typing and `tr` do not
  run.
- **toward K** — the matchers expanded to their `casesOn` trees and
  the recursion compiled as slices 3.13–3.14 ruled (`T.rec` with a
  constant motive, `T.brecOn` with the table generalized at each
  split, `WellFounded.fix` with the `f.dec_N` obligations),
  `define.shard`'s machinery re-pointed from E terms and E types to
  the pre-definition and K's types. Unavailable while a callee-local
  is open (rule 6).

The classifier becomes what the law calls it: a check on the
elaborated term that it lies in the fragment (§6.3's refusals — a
function value in a runtime position, a noncomputable head, a callee
without a realization — raised by the erasure), not a second type
system. There is no new term language: the body is `Expr`.

**The rules of slice 3.16:**

1. **Matchers, their descriptions bound by regeneration** (R64). A
   `match` elaborates to `OWNER.match_N` applied to a motive, the
   scrutinees and one alternative per row (Lean's shape: `(motive :
   D… → Sort v) → (d : D)… → (alt_i : ∀ pattern variables, motive
   pattern_i)… → motive d…`; an alternative without variables is
   `motive pattern_i`). The matcher is a definition K checks,
   admitted before its owner with an `ACCEPT` record as the bundle's
   constants are: a `casesOn` tree by first-match column splitting
   (the compilation `df_matrix` does today, stated over K's types), a
   fall-through row's alternative applied at every leaf it reaches.
   The **description** — the scrutinee types, the rows' patterns in
   order, each alternative's telescope — **is the matcher's type**
   (as built, landing 1; the design's side table dropped): each
   alternative's type spells its row's patterns as a term over its
   variables, so the rows are read back off the admitted declaration
   (`mt_describe`) and nothing beside it can go stale, be lost in a
   view's fork (§6.6) or be trusted by a name's spelling. A projection
   that meets a matcher application **regenerates** the definition
   from the rows read back and compares it with the admitted value
   (`mt_is_matcher`); a constant that fails the comparison is not a
   matcher — `matcher_description` to a projection, never a
   name-based guess and never an eager call. The generator is
   deterministic, so the binding is a check, not a trust.
   **Forcing:** the E program evaluates the scrutinees once, left to
   right, then the first matching row's arm only — `EMatch`'s rule
   (§6.2); an alternative is control flow in E and an argument only in
   L, where nothing runs. The elaborator cites a matcher with the
   motive from the expected type (slice 3.15 rule 5's Miller pattern
   gives the constant motive); a dependent motive is a later slice —
   slice 3.19 (rule 9) for a scrutinee that is a local the expected
   type mentions.
   **A `match` in a statement or a `def` is the same form** — slice
   3.15's open item closes here. Literal patterns are the obstacle
   `literal_pattern` until a consumer under the naming law has one.
2. **The self-local, and branch facts in every position** (R65). A
   `fn` with a `(measure …)` is elaborated with its name bound to a
   local of its Pi type. The K projection keeps today's guarantee and
   extends it: inside a recursive function an `ite c inst a b` is
   compiled as the **dependent** form (`dite`; today's `Decidable.rec`
   with its proof field in scope — Lean's well-founded preprocessing
   does the same), so `h : c` or `h : ¬c` is a local of the branch
   wherever the `if` stands — under a `let`, in an argument, under a
   matcher's alternative — and a matcher's alternative has its
   pattern variables in scope the same way. Each `f.dec_N` is closed
   over **every** local in scope at its call (slice 3.14 rule 2's
   as-built, now position-independent); sibling branches share
   nothing. **Narrower guarantee, stated:** for a `match` on a
   computed scrutinee the equation `scrutinee = pattern` is not yet a
   local (a variable scrutinee is substituted, as today); an
   obligation that needs it is reported with the pointer to bind the
   scrutinee in a `let` — built when a consumer has one. Outside a
   recursive function `ite` stays `ite`.
3. **`Bool` and propositions meet in the elaborator, Lean's way, and
   the conversion is explicit in E** (R66). A proposition where a
   `Bool` is expected is `Decidable.decide p` with the decision
   `decision_of` finds (else `no_decision`); a `Bool` where a
   proposition is expected — an `if`'s condition — is `c = true` with
   `Bool`'s `decEq`. This **changes slice 3.15 rule 4**: `if` on a
   `Bool` is `ite (c = true) …`, not `cond` (Lean's own elaboration;
   `cond` sits at export line 151,585, past every fixture). Erasure:
   `Bool` and `Decidable` have **different cells** in `ev`
   (`init_bool_cell`, `dec_cell`), and only the second-constructor
   rule of `EIf` lets either drive an `if`. So `decide p inst` erases
   to the conversion `(if ⟦inst⟧ true false)` over Init's `Bool`
   constructors — no new primitive — and is the identity only where
   the realization of `inst` already returns a `Bool` cell (the table
   says which: rule 4); the decision of `c = true` in a condition
   position erases to `c`. A shared two-valued representation is a
   later optimization, never assumed.
4. **Realization selection, not an inverse of spellings** (R66). The
   operator table is the first form of a small static realization
   registry: a resolved L operation at its instantiated types → the
   executable entry, its result cell (an integer, a `Bool` cell, a
   `Decidable` cell) and its stated trust. (Since slice 3.19 a
   decision's entry is its *test*, read where the decision is an
   `if`'s condition; as a value the decision is `(if TEST isTrue
   isFalse)`, `Decidable`'s own cells — a match by constructor on the
   bare test took the wrong row.) `Int.add`, `Int.sub`,
   `Int.mul`, `Int`'s and `Nat`'s decisions map to the entries §8.4
   rule 3 maps from; the `Nat` identities to the entries under their
   own names. **Once `-` `/` `mod` at `Nat` have resolved to
   `Nat.sub`, `Nat.div`, `Nat.mod`, the erasure selects those
   entries** (saturating; total at a zero divisor) — this **reverses
   revision 1's refusal**, which protected the E-first reading of the
   spelling and has no reason to survive the direction change. The
   E-first route keeps its own semantics and its `nat_operator`
   refusal for the sources that stay on it (rule 6). An operator
   means the same in a `fn`, a `def` and a theorem once its operand
   type is fixed.
5. **One resolver.** A `fn` body's names resolve as a `def`'s do
   (`scope.shard`). Where a bare name is ambiguous and exactly one
   candidate is a constructor of the expected type's head inductive,
   that constructor is meant (§3.1's "Stage 1 resolves constructors
   by expected type", now built) — `Add` in calc's bodies and in its
   theorem alike.
6. **Which route; a fallback weakens the guarantee and never
   reinterprets** (R70). A function takes this route when its
   signature elaborates (every binder and return type has an L
   identity), decided **before** its body is read. On the route:
   - a callee that is `RUNNABLE` (no K constant) is bound as a
     **callee-local** of its elaborated signature type — the body
     still elaborates once, with every other choice resolved as it
     would be, and erases; the K projection is unavailable, the
     record is `RUNNABLE … why=callee_runnable route=typed` with the
     callee named, and nothing is re-read by the classifier. A callee
     whose own signature has no L reading is the head obstacle below;
   - a **head without an L identity before slice 3.17** — a symbol or
     a `"…"` literal in the body, a toolchain-prelude spelling — is
     the one case that still falls to the E-first route whole, because
     no L term exists to project: `RUNNABLE … why=no_l_identity
     route=e_first`, machine-readable, the set of such functions in
     the tree named in the as-built and **emptied by 3.17** (then this
     bullet is deleted with its code);
   - anything else that fails is a **refusal** (`type_mismatch` and
     the rest of §5.1's): a typed-route error never becomes an
     E-first success, and a `fn` body is typed against its signature
     from this slice on (§12.1's "return types unchecked" row).
   A function whose signature has no L reading — every function of
   the toolchain's own sources, which name the prelude's types —
   never enters the route: `route=e_first` is theirs until they port
   under the naming law, and the route is deleted then (guard 2; the
   named consumers are `v3/kernel/**` and the old tree's ports, no
   per-feature switch). Frontend parity and route 2's byte-tie are
   over those sources and do not move. A request for an admitted
   definition (a theorem citing the function, `realize`'s callee
   check) refuses a `RUNNABLE` result as today (`no_l_meaning`).
7. **`realize` with a supplied body** takes the same route: the body
   elaborated against the realized constant's type, its program the
   erasure, its equations per leaf as §7.5 has them. With rules 1–6
   `tr` has no caller and is **deleted**, with `c_type`'s use as a
   typing for L (`arg_types`, `prim_typed_identity`, `ctor_etargs`).
   The derived view is the E projection applied to a constant's
   value, unchanged in what it accepts.
8. **One descent obligation discharged end to end** (R65, R69).
   `(fulfills f.dec_N PROOF)` after the function, outside a view:
   PROOF elaborated as a term against the obligation's statement and
   checked by K in the environment **without** `f.dec_N` and without
   anything whose `params=` reaches it — the closure instrument
   (`axioms.shard`) over the proof's constants is the acyclicity
   check, so a proof resting on its own obligation, directly or
   through a second obligation or a theorem, is `fulfills_cycle` and
   closes nothing. On success the obligation's record is
   `DISCHARGE f.dec_N`, the function's `pending=` and every
   dependent's `params=` lose exactly that name (the closure
   recomputed, never a suppressed warning), and any assumption the
   proof itself used stays in the account. The fixture is
   `define_measure`'s `count`: its `dec_1` printed with `h : ¬(n =
   0)` in the statement, discharged by a handwritten term over Init's
   library (no `omega`, no I), the pending sets empty after. An
   obligation stays an axiom-kind constant of the fourth class for
   provisional checking and is never an ambient axiom (slice 3.14
   rule 2). Dependency-directed wake-up of parked work is **deferred**
   (the retry fixpoint of slice 3.15 rule 6 stands while loads are
   small); what a parked function waits for — a declaration, a proof
   — is already distinct in its record.
9. **Partial outcomes at the shared boundary** (R67). `kw.shard`'s
   wrappers return K's outcome — the value, or K's refusal with its
   reason, or exhaustion — not `None`. `unify` gains two results
   beside `UYes`/`UNo`: `UStuck` (blocked on a named metavariable;
   retry after assignment) and `UExhausted` (the depth or K's
   budget); neither is ever read as inequality, and `check_expected`
   reports them as `unify_stuck`/`unify_exhausted`, not
   `type_mismatch`. An assignment is recorded **typed** or
   **tentative**: a value with a metavariable inside whose type
   `infer_light` cannot give is tentative, its typing constraint kept
   and checked when the value closes; a **closed** value K refuses to
   type is `UNo` with K's reason (today it is installed — corrected
   here). `el_finish` refuses an outstanding tentative assignment as
   it refuses an unsolved metavariable. Failure leaves the context as
   it was (the transaction of slice 3.15 stands). The vocabulary is
   for I and search to share; private `Option` helpers below the
   boundary stay.
10. **Holes across a binder: the narrower guarantee, stated** (R68,
    **deferred to I's opener**, slice 3.20 — 3.19 when written; deriving took that number at item 51). Today `mc_drop_fvars`
    removes a closing binder's local from every unsolved
    metavariable's telescope, so a hole cannot be filled with that
    variable after its binder closes. That is **incomplete, never
    unsound** (the assignment is refused; K checks the closed term),
    and the elaborator of this slice does not need more. The
    contextual-hole contract — a stable creation telescope, each
    occurrence an instantiation of it, delayed assignment — is built
    once, in the shared `MCtx`, as the first item of I; until then no
    API outside `elab.shard` may take an `MCtx` across a binder
    close, which is what keeps callers off the deferred capability.
11. **The facilities that are small once there is one place for
    them:** `(list a b c)` by the expected type's `List` (no expected
    type: `unsolved_implicit`, never a default element type); a
    negative numeral is already §5.3's. Record update, the fresh-name
    convention and E's rename of `lt le int_eq` are slice 3.17; the
    byte and text adapters, deriving and — moved there by item 50's
    ruling — `"…"` in a `fn` body, symbols and `Name` literals are
    slice 3.18, the wire's type ruled first.
    The three consumers after it — a branch-local refinement, a
    dependent-safe record update, a contextual proof step — use the
    `PreDef` record, the branch scopes of rule 2 and the outcomes of
    rule 9; none gets a representation of its own (R71).

**Landings and gates** (R71: exact agreement is the first migration
alarm where the representation is meant to be unchanged; every
mismatch is investigated; an intended change is accepted with its
stated comparison and the migrated references, never by editing the
expected output until the test passes). (1) The `PreDef` record;
matchers with descriptions and the regeneration check; `match` in L
terms; constructors by expected type; rule 9's wrappers and outcomes.
(2) The E projection generalized (matcher, self-local, callee-local,
the `decide` conversion, realization selection, `Bool`'s `if`) with a
**tie test**: for every function defined today in calc's files and
`v3/std/list.shard`, the erasure of the pre-definition against the
classifier's program, byte for byte up to rule 4's entries. (3) The
flip: `fn` and supplied `realize` on the route, `define.shard`
re-pointed, `tr` deleted, rule 8's discharge; then `(list …)`. Every
landing keeps the loader pins, parity, route 2, K's clients and T0
green. **The slice's gate:** calc's spec file with all 25 functions
defined; `calc_test` byte-identical against the old tree with the
programs now the erasures; the equations' names unchanged; a value
hash that moves is listed with its cause (a function whose body has
an `if` is expected to keep `Decidable.rec`'s shape and its hash —
rule 2 — where it is recursive, and to move to `ite` where it is
not); and the boundary fixtures:

| fixture | pins | asks |
|---|---|---|
| G1 one source meaning | the same scoped expression in a `fn`, a `def` and a theorem: numerals, `Add`, a nested `match`, `-` at `Nat` and at `Int` | R63, R66, R70 |
| G2 matcher identity and forcing | a swapped description refused (`matcher_description`); a missing one refused; an unselected arm that would exhaust is not run; a computed scrutinee evaluated once | R64 |
| G3 `Bool` beyond `if` | both values of a `decide` returned, stored in a field, matched against `true`/`false`, passed to `and`, used as a condition | R66 |
| G4 a completed measured definition | `count.dec_1` with its hypothesis; discharged; pending sets empty; the same call moved under a `let` and into an argument | R65, R69 |
| G6 outcomes | a low depth gives `unify_exhausted` and a higher one succeeds; a blocked unification names its metavariable; a refused closed assignment leaves the context unchanged | R67 |
| G7 acyclic closure | a proof citing its own obligation, and a two-obligation cycle, close nothing | R69 |
| G8 no reinterpretation | a `RUNNABLE` callee added to a body changes the record and no resolved operator, literal or constructor in the dump; a typed-route error stays a refusal | R70 |
| G9 descriptions through a view | a function with a `match` defined inside an implementation check: projected in the fork, the description present | R63, R64 |

G5 (contextual delayed filling) is rule 10's and opens slice 3.20 (I's opener).

**Risks named before code.** The table generalization of slice 3.14
was written over E patterns and E types; re-pointing it is the
largest piece and the landing where a full context window may go.
A matcher's universe (`Sort v`) is one more level for the elaborator
to infer at each `match`. Rule 8 needs a lemma of Init's library that
the fixture prefix may not reach (`Nat.sub_one_lt` or its kin): the
fixture is extended or the proof goes through what the prefix has.
The slice is larger than revision 1 by rules 8 and 9 and the
fixtures; landing 3 may split in two. If the tie test of landing 2
fails on a shape the probe did not cover, the design returns here
before landing 3.

**As built, landing 1 (2026-09-19): matchers, `match` in L terms, the
constructor by expected type, the outcomes.** `kernel/matcher.shard` —
`MtPat` (a variable with its slot, a constructor on its fields),
`mt_generate` (the motive, the scrutinees and the alternatives as K
locals; `mt_compile`, the `casesOn` tree by first-match column
splitting — a row with a variable at the split column kept under every
constructor with the variable bound to the constructor term, and at a
leaf every variable bound before a later split **refined** by it, so
the alternative's argument is the term the motive's argument has
there; `match_not_exhaustive`, `match_indexed`, `match_eliminator`),
the definition an `abbrev` over the owner's universe parameters and
one more for the motive; `mt_describe`/`mt_is_matcher` (rule 1's
read-back and regeneration check). `elab.shard` — `elab_match` (the
constant motive from the expected type, which must be determined:
`match_motive` otherwise, with the pointer), `el_pat` (a pattern at its
type: `_`, a name — the type's field-less constructor of that name,
else a variable —, a constructor on its sub-patterns;
`literal_pattern`, `pattern_constructor`, `pattern_arity`), the
matcher's parameters the binders in scope its scrutinee's type
mentions, the matcher **checked into the walk's environment as it is
made** (`kw.shard`'s `w_admit`: K's `check` seeded above the walk's
node ids, the walk's then raised above the new environment's) and
handed to the reader with the declaration (`El`'s `Gen`, `el_aux`,
`reader.shard`'s `rd_ok`), so the loader admits `OWNER.match_N` before
its owner with an `ACCEPT` record of its own. A `match` in a
requirement's statement is `match_in_requirement` (a view parameter is
one declaration); in an inductive's or a structure's types K's
`unknown_constant` answers (no consumer). Rule 5: `scope.shard`'s
`resolve_ctor` (the name under the inductive's own prefix first, then
the scope's candidates) — every pattern head, and a term's head where
the plain resolution is ambiguous and the expected type's head is an
inductive. A body's untyped `let` binding `(x TERM)` is read. Rule 9:
`kw.shard`'s `w_infer_r`, `w_whnf_r`, `w_is_def_eq_r` answer `WVal` or
`WFail` with K's `Failure`; `unify.shard`'s `URes` has `UStuck` (a
failure under an unassigned metavariable applied to what is no
pattern) and `UExhausted` (the depth, or a spent resource of K's);
an assignment is typed or **tentative** (`mc_tentative`,
`tentative_bad` asked by `el_finish`: `tentative_assignment`); a
closed value K refuses is no longer installed — except one citing a
constant the form is still declaring (`cites_undeclared`: an
inductive's name in its constructors, a structure's in its
projections), which K cannot type before the admission and the
admission checks; `check_expected` and `elab_slot` report
`unify_stuck` / `unify_exhausted` apart from `type_mismatch`. **Not
in landing 1:** the `PreDef` record (its first consumer is landing
2's erasure). Tests: `kernel/test/unify_test.shard` (G6),
`kernel/test/matcher_test.shard` (G2's identity half: the pins'
matchers recognized with their rows, a hand-written equal one
recognized, an imposter of the same type and name not); pins
`match_def`, `match_statement`, `match_poly`, `match_imposter`,
`match_not_exhaustive`, `match_literal`, `match_motive`,
`ctor_expected` (G1's constructor half).

**As built, landing 2 (2026-09-19): the pre-definition, the E
projection, the tie.** `kernel/predef.shard` — `PreDef` (identity,
declared type, the binders' locals, the **self-local**, the finished
body, the elaboration state with the matchers already in its
environment) and `read_predef` (a `fn` form read once: a `(T Type)`
binder implicit, the function's short name bound to a local of the
declared type, the body at the result type; `PdSkip` a signature with
no L reading, `PdErr` a refusal on the route); never a `Declaration`
for the owner. The erasure (still in `realize.shard` beside `tr`,
which it shares its state with; its own file when `tr` goes, landing
3) gained: a **matcher application** → the `EMatch` of the rows
`mt_is_matcher` read back and checked, each alternative's lambdas the
row's variables, an erased field's variable dropped (`erase_matcher`,
`mt_epat`); the **self-local** → `ECall` with the binders' runtime
flags (`EKCall`); `Decidable.decide p inst` → `(if ⟦inst⟧ true false)`
over Init's `Bool` — always, no identity case yet (rule 3's "where the
entry returns a `Bool` cell" waits for a consumer: `ev` has three
two-valued cells, the scope's `Bool`, Init's and `Decidable`'s);
`instDecidableEqBool c true` → `⟦c⟧`; `Int.ofNat n` → `⟦n⟧` and
`Int.negSucc` of a literal → the negative literal (one integer at run
time); rule 4's `Int` rows (`realization_of`: `Int.add`/`sub`/`mul`,
`Int.decEq`/`decLt`/`decLe` → `+ - * int_eq lt le`). `erase_predef` —
the binders' roles from their locals' types, the body erased, the
function's `EFn`. In the elaborator: a proposition where a `Bool` is
expected is `decide` with `decision_of`'s decision, a `Bool` where a
proposition is expected is `c = true` (both in `check_expected`,
beside the `Nat → Int` coercion); `if` on a `Bool` is `ite (c = true)`
with `instDecidableEqBool`; **`if` on a value of a two-constructor
inductive of the program's is the `match` it abbreviates** (§6.2's tag
rule in L: the second constructor takes the then-branch — found by
the tie, on `kernel.util.bool_and` over the prelude's `Bool`); an
operator whose first operand is a numeral takes its identity from the
other operand's type (`(le 48 c)` at `c`'s `Int` — slice 3.15 rule 3's
"first operand" amended: the first that is not a numeral); `(list a
b …)` is the constructor chain of the expected type's list (an
inductive with a field-less and a two-field constructor; no expected
type: `unsolved_implicit`); a bound head inserts its implicit binders
as a constant does (a polymorphic self-call). **The tie** (the
loader's `define_try`, `realize.shard`'s `tie_check`; gone at landing
3): every function the old route defines is also read as a
pre-definition and erased, and the erasure compared with the
classifier's program term for term — a primitive by the operation it
performs (`+` and `Nat.add` one addition), an `if` on a two-constructor
type equal to the match it abbreviates; a difference, or a stage of
the new route failing, is the refusal `tie_differs`. **It holds for
every function defined in calc's files, `v3/std/list.shard` and all
125 loader pins**, nested patterns, course-of-values and measured
functions included. Tests: `kernel/test/project_test.shard` (G3: both
values of a `decide` returned, matched, stored, used as a condition
and as a branch; G2's forcing: the chosen arm only); pins
`project_bool`, `elab_bridge` (`is_digit 53 = true` by `Eq.refl`
through `decide` and `ite (c = true)`). G1 and G8 wait for the flip:
today a `fn` with `(le 48 c)` has no L half to compare.

**As built, landing 3 (2026-09-19): the flip.** A `fn` is read once and
its program is the erasure; the classifier types no function that has
an L reading. *Decided before the old route was deleted:* the design's
"`define.shard`'s machinery re-pointed" is Lean's own structural
compilation rather than a port of slice 3.14's table bookkeeping, which
was written over E patterns and a matrix the pre-definition no longer
has.

- **The loader** (`load_fn`): `read_predef`, then `erase_predef`'s
  `EFn` — the program — then `define_pd`. `DEFINE` with the
  equations; or `RUNNABLE … why=… route=typed` when K's half cannot be
  reached (`below_unreached`, `wf_unreached`, `measure_type`,
  `callee_runnable`) — the program is still the erasure; or a refusal
  (`type_mismatch`, `match_not_exhaustive`, `ambiguous_name`,
  `pattern_constructor`, `if_type`, `private_if`, `realize_descent`,
  `realize_recursion` …: the classifier's `unsaturated`,
  `function_value`, `nonexhaustive`, `ambiguous_head`, `pattern_type`
  are its own, for the sources still on its route). The `RUNNABLE`
  record carries the route (`route=typed`, `route=e_first`, none for
  an extern or an E-only type). The matchers a body generated are
  admitted before their owner (`ACCEPT NAME.match_N`), the auxiliaries
  of a native inductive before the function is read again.
- **Rule 6 as built.** `route=e_first`, the classifier's program: a
  signature with no L reading (`why=e_only_type` — every function of
  `v3/kernel/**` but a handful); after the signature elaborates, a
  `(quote S)` or a `"…"` anywhere in the body (checked on the form,
  before the body is read), a literal pattern, a name the environment
  lacks that is no function — an operator's identity past the Init
  prefix (`Nat.add` under `(import Init Nat)`, `ite` under a prefix
  that stops before it, a `casesOn` not yet generated), a toolchain
  spelling — all `why=no_l_identity`; a callee that is an extern or
  was itself E-first by signature, `why=no_l_meaning`. A name that is
  nothing at all is still a refusal, the classifier's
  (`unknown_head`). **The set in the tree today:** calc's `show_nat`
  (`Int.ediv` and `Int.emod` lie past its file's Init prefix — the
  first cut of this sentence said a `"…"`, which the function does not
  contain; corrected 2026-09-19 at slice 3.17's scoping),
  `kernel.define.has_eq` and `confusion_reachable` (K's view's
  `env_find` answers an E-only type); slice 3.17 below says
  what empties it and what does not (as built there: `show_nat` is
  defined; `has_eq` reads `no_l_meaning`; `confusion_reachable`
  quotes symbols and stays `no_l_identity` until slice 3.18). A
  **callee-local**: a callee with a program and no K constant whose
  signature elaborates — `RUNNABLE` on the typed route, or E-first
  for its body alone — is bound, under the name the body writes, to a
  local of its signature's type, the signature elaborated in the
  callee's own scope inside the caller's walk; the body is read again
  with it, erased with the call in it, and recorded `why=callee_runnable
  route=typed`. calc's `show_ascii`, `step` and the trace functions
  are such (their root `show_nat`). A callee later in the file parks
  the function (slice 3.15's retry); at the file's end the parked
  functions are each other's callee-locals — mutual recursion is two
  programs by erasure and no definition (`define_mutual`).
- **The K projection** (`define.shard`, rebuilt; the E-term matrix,
  `Known`, the table splits and `df_*` deleted). A body that never
  cites the self-local is the value under its binders, matchers and
  `ite` as elaborated. **Structural:** `T.brecOn` applied to `λ x
  below others . body'` over the binders' own locals. **The matcher
  stays a constant and its motive carries the table** (Lean's
  `MatcherApp.addArg`): a matcher application whose scrutinee is a
  local that a table's type mentions is re-instantiated at `λ s .
  B[s] → motive s` — its universe K's sort of that arrow — each
  alternative takes the refined table `B[pattern_i]` as a last
  binder, and the application is applied to the table; nothing is
  unfolded. **A self-call is found by type:** each table is kept with
  its type over an *abstract* motive `C` (the real motive is constant
  in its argument, so `motive r` cannot tell one entry from another);
  the search reads K's normal form — `C y'` with `y'` the argument
  (syntactically, else by K's conversion) is the entry, a `PProd` is
  searched through both projections, a `T.below C t` unfolds exactly
  when `t` is a constructor application — so the shape of `below` is
  never assumed, and a self-call under a match in an argument or a
  `let`'s value finds its entry as one in tail position does (slice
  3.14's `recursion_depth` obstacle is gone). No entry:
  `realize_descent`. **Measured:**
  `WellFounded.fix` over the data binders' tuple, the body with each
  binder the tuple's component; a self-call `rec (tuple args)
  (f.dec_N locals…)` over every local in scope — pattern variables,
  `let`s (as `let`s in the statement), branch proofs; the measure's
  type is checked (`measure_type`) before `WellFounded`'s
  reachability (`wf_unreached`). **`ite` inside a recursive function**
  is `Decidable.rec` with a constant motive and the proof a local of
  each branch, wherever it stands — but only where a self-call sits
  under it: a subterm that does not cite the self-local is left as
  elaborated. A function with a measure clause that never calls
  itself is a plain definition, not `PENDING`.
- **The equations** (`realize.shard`'s `pj_equations`, shared with
  `realize`): the leading matchers' rows against the scrutinee term —
  bind, clash (anywhere in the row: the row is gone from the
  specialized matrix), or stuck on a universally quantified local,
  which is split over its constructors in order — no reduction by K.
  The leaves and the names `f.eq_N` are slice 3.13's; the statements
  now cite the matcher constants, so their hashes moved with the
  definitions'. **A `fn`'s leaf is stated where K decides it by
  conversion** (the definition is checked into the walk's environment
  first): a structural recursion whose leaf leaves the structural
  argument a local, or keeps a self-call under a matcher still stuck,
  unfolds only by a case analysis — that equation is I's, the leaf
  keeps its number, and the function is defined without it
  (`define_arg_match`; slice 3.14 left such a function `RUNNABLE`).
  A realization's equations are all stated (`realize_wrong` is K's
  refusal).
- **`if` on a two-constructor inductive is `T.casesOn`**, not a
  matcher (landing 2's as-built reversed): the erasure reads a
  two-constructor case analysis whose alternatives ignore their
  fields back as `EIf`, so the toolchain's `bool_and`/`bool_or` keep
  the old front end's program byte for byte, and the equations do not
  split on an `if`. The elaborator refuses `if_type` and `private_if`
  itself and checks that the `Bool` bridge's constants (`Eq`,
  `Decidable`, `instDecidableEqBool`, `Decidable.decide`) exist
  before it cites them (a short Init prefix is `no_l_identity`, not
  K's refusal at admission).
- **Rule 4 as built:** `Nat.add`, `Nat.mul`, `Nat.decEq`, `Nat.decLt`,
  `Nat.decLe` select the integer entries `+ * int_eq lt le` — one
  operation on naturals, and the classifier's spelling, so frontend
  parity is byte for byte; `Nat.sub`, `Nat.div`, `Nat.mod` keep their
  own (truncating, total) entries. `define_nat_operator` is `ok`.
- **Rule 7:** a supplied `realize` binds its signature's names to the
  realized constant's own telescope (an erased binder has none),
  elaborates the body at the result's L type, takes the erasure as
  the program, checks descent on it, and states the equations of its
  leading matchers (`Eq.refl`, or the clause's theorem); its matchers
  are owned by `MODULE.NAME.realize` and admitted first. A `RUNNABLE`
  callee is `no_l_meaning`, an E primitive without an L identity
  `no_l_identity`, a callee without a realization the erasure's
  `no_realization` (was `realize_order`). **`tr` is deleted** with
  what only it used — the typed application through `c_type`
  (`arg_types`, `ctor_etargs`, `apply_const_e`, the first-order
  matching of binder types), the E-term case tree, the old equations,
  `DefRec`/`BelowInfo`/`WfInfo` — and the tie. The erasure is
  `kernel/erasure.shard` (with the walk's state `Tr`);
  `realize.shard` is the supplied form, the descent check and the
  equations; `prim_typed_identity` and `c_type` remain the
  classifier's own.
- **Rule 8 as built.** `(fulfills f.dec_N PROOF)` in a file outside a
  view: PROOF read against the obligation's statement as the theorem
  `f.dec_N.proof`, checked by K. Every closure the loader computes
  replaces a discharged obligation by what its proof rests on (to a
  fixpoint), and the theorem's closure is the acyclicity check: it
  reaches `f.dec_N` — directly, through `f` (whose value cites it),
  through a theorem, or through an obligation discharged earlier by a
  proof that rests on this one — `fulfills_cycle`, nothing admitted.
  On success `DISCHARGE f.dec_N proved`, and the account is
  recomputed in the records: every `ACCEPT` that named the obligation
  names the proof's own account instead, and a function with no open
  obligation is no longer `PENDING` nor in any `pending=`. The
  fixture: `count.dec_1 : ∀ p (h : ¬ p.1 = 0), InvImage … ⟨p.1 - 1,
  p.2 + 1⟩ p`, by `Nat.sub_lt (Nat.pos_of_ne_zero h)
  (Nat.zero_lt_succ 0)` — both in the prefix through `InvImage.wf`.
- **The gate.** calc's spec file: 25 of 25 functions defined
  (`parse_tail` by its measure, two obligations `PENDING`; the file's
  Init import now reaches `InvImage.wf`); `calc_test` byte-identical
  over 21 inputs with the erasures as the programs; frontend parity
  byte-identical over 27 closures; the equations' names unchanged.
  **Hashes:** every `fn`'s definition and equations moved — the value
  cites `NAME.match_N` where it held a `T.rec` tree (the cause, for
  all of them); K's own constants and Init's are untouched (T0).
  Fixtures: G1 `one_meaning`, G4 `discharge_measure` (+ the account
  read in `define_test.sh`), G7 `discharge_self_cycle` and
  `discharge_two_cycle`, G8 `route_callee` (+ the dumps compared in
  `define_test.sh`), G9 `match_in_impl`; G2, G3, G6 are landings 1–2's;
  G5 opens slice 3.20 (I's opener). Eleven pins moved to the elaborator's reason or
  the new outcome, each with its cause in its header.
- **Open:** a `match` on a computed scrutinee inside a measured
  function states its obligation without `scrutinee = pattern` (rule
  2's narrower guarantee, unchanged); mutual recursion is not
  compiled; dependency-directed wake-up is deferred (rule 8).

**Slice 3.17 — the porting facilities** (design 2026-09-19, scoped by
probes through the loader before any code; RULED 2026-10-01 — the
user agreed with the three leans, rules 1, 2 and 5; built the same
day, the as-built below; §13 item 50). The slices table gives this slice R60's row (§11): record
update and order-free construction, `Name` literals, symbols and `"…"`
in a body, the fresh-name convention, E's rename of `lt le int_eq`.
Measured against the tree after slice 3.16, the row splits into what
already works, what is small, and two entries that do not belong here.

*What the probes found* (scratch files, the pins' Init prefix):

- **Records work on the typed route today.** `(record Pt (x Nat) (y
  Nat))` with `make` out of order and `with`: six functions `DEFINE`d,
  a `def` using `make` accepted, and v2's law family needs no
  generator — `(= (Pt.x (Pt.with_x v p)) v)` and `(= (Pt.y (bump p))
  (Pt.y p))` over a *local* `p` are `Eq.refl` (K unfolds a structure's
  recursor on a local through its projections, Lean's rule). What is
  left of R60's entry is its caveat — a changed dependent field an
  explicit obligation — which no `record` can state (its fields are
  positional, non-dependent types); it becomes real when `make`/`with`
  reach a `structure`.
- **`lt le int_eq` and `< <= =` already elaborate to one term**
  (`lt3` and `lt3b`: one hash). The classifier knows only the old
  spellings (`unknown_head: =` in an E-first function).
- **A literal pattern** (`(match n (0 1) (_ 2))`, at `Nat` and at
  `Int`) is `no_l_identity route=e_first`. 0 rows in `v3/`, 53 in the
  old tree.
- **`Symbol` has no L reading** (`e_only_type`); 2,905 `(quote …)`
  sites in `v3/`, every one in a function that is E-first by its
  signature anyway (K's sources see no `Init`, so `Int` has no L
  constant there). Outside the toolchain the old tree names `Symbol`
  in 4 files. `String` is in the pins' prefix and `"zero"` is a
  `String` in a `def`; `String.decEq` is past the prefix.
- **The E-first-by-body set is not what landing 3's as-built said.**
  calc's `show_nat` has no `"…"`: `Int.ediv`/`Int.emod` lie past
  `(import Init Int.decEq)`; with `(import Init Int.emod)` it is on the
  typed route (`measure_type`: its measure is an `Int`). `has_eq` and
  `confusion_reachable` call `env_find`, whose signature has no L
  reading until K's sources port — no facility of this slice changes
  that, and their record should read `no_l_meaning`, as the as-built
  describes that case, not `no_l_identity`.
- **The rename reaches the v2 chain.** `v3/kernel/**` is hosted by the
  Rust bootstrap (one evaluator for both trees) and compiled on route
  1 by `tools/lower` and `tools/codegen`, which read it with the old
  tree's kernel: 73 sites in 10 files of `kernel/` and 3 in
  `codegen.shard` name the three primitives, and `=` there is the
  claim language's equality. 943 sites in 60 files of `v3/` would
  move.

*The rules* (the design's leans, ruled 2026-10-01; rule 3 was built
differently from this text and is rewritten in place, the first
reading kept as rejected):

1. **`"…"` in a `fn` body moves to slice 3.18, with the flip.** Since
   slice 3.16 a `fn` body is an L position, and in an L position
   `"…"` is already `LitStr : String` (§5.3). Reading it as the byte
   list in a `fn` and a `String` in a `def` is two meanings for one
   text under one elaborator — guard 1. It keeps `no_l_identity
   route=e_first` until item 37's flip gives it the one meaning
   everywhere. Rejected: list-literal sugar over the bytes by the
   expected type (twenty lines, and wrong for exactly one slice).
2. **Symbols move to slice 3.18 too.** Item 36 rules `Symbol`'s L
   identity `String`. Assigned now, a `String` binder erases to the
   atom cell; at 3.18 `"…"` is a `String` as well, so the atom would
   have to *be* `String`'s E representation — the wire's ruling made
   by a side effect. No function in the tree would change route by
   it. `Name` literals go with them (K's `Name` holds the atoms; a
   literal over an E-only type helps no typed function), stated there
   as loader-level sugar over the `Name` in scope — plain data, no
   node, declaration or environment identity to forge (`name.shard`
   carries none). Rejected: a native `structure Symbol` (reverses
   item 36 for no consumer).
3. **Numeral rows on the typed route** are Lean's value patterns, one
   mechanism at `Nat` and at `Int`: the scrutinee once, then `(if (= x
   K) BODY …)` per row in order — the `ite` on the type's decision, as
   any `if` is read — and the last row `_` or a name for the
   scrutinee. No matcher is generated, and the erasure is the `if`
   chain (`(if (= #0 0) 10 (if (= #0 5) 20 30))`), the scrutinee
   bound by a `let` when it is not a local. A match of numeral rows
   with no last row is `match_not_exhaustive`. *Rejected, the design's
   first reading:* a numeral at `Nat` as the constructor pattern
   through the matcher, both types read back as E's literal row —
   because a numeral is then a `succ` chain as deep as its value (48
   nested `casesOn` for an ASCII code, against the pattern reader's
   depth), E has no successor pattern so the constructor reading buys
   a program nothing, and reading an `ite` chain back as a literal
   row is recognition of a shape a user also writes by hand. The
   classifier's literal rows stay what an E-first function runs.
4. **Records: the first example is the deliverable.** A pin
   (`record_laws`) with the three probes above as theorems; `make` and
   `with` over a `structure` whose later field's type mentions an
   earlier one: updating the earlier field without the later is the
   refusal `dependent_update`, naming the fields to supply — never an
   inserted cast.
5. **The rename, staged by who reads the file.** E's table takes `<
   <= =` as the primitives' names and the classifier accepts them; the
   Rust bootstrap accepts both spellings (it hosts the old tree); the
   files no v2 tool reads — `v3/std`, `v3/examples`, `v3/pins` —
   migrate by tool now, and the old spellings are refused there.
   `v3/kernel/**` migrates when route 1's chain is V3's own, since
   until then the old tree's kernel reads it: teaching the v2 kernel
   an alias whose `=` collides with the claim language is a change to
   the old trusted base for a spelling. Rejected: a rewriting shim in
   `build.sh` (a derived source between the text and the compiler);
   the full migration now (route 1 is testable only on CI).
6. **The fresh-name convention** is a library, not a form:
   `v3/std/fresh.shard` — a `Fresh` record (a prefix and a counter),
   `Fresh.next` returning the name and the advanced supply — with a
   theorem that two successive names differ, and the convention
   (§12.1's `gen_fresh` row) that a function needing names takes and
   returns the supply.
7. **The E-first-by-body set.** calc's app file imports through
   `Int.emod`, which puts `show_nat` on the typed route as
   `measure_type`. Its definition needs a `Nat` measure, and the
   probe found the gap: the measure is erased with the body, the
   prefix holds `Int.natAbs` (not `Int.toNat`), and `Int.natAbs` has
   no program — `(realize Int.natAbs (view))` is `no_realization`,
   its value matching on `Int`'s constructors, which E's `Int` does
   not have. The lean: a realization-registry row for `Int.natAbs`
   (and `Int.toNat` where the prefix holds it) as an E expression over
   the existing primitives — no new primitive — so `show_nat` is
   defined with its obligations `PENDING`. A callee whose signature
   has no L reading is recorded `no_l_meaning` and is the port's, not
   this slice's; rule 6's bullet stays for `"…"` and symbols until
   3.18 and is deleted there.

*Gates:* the loader pins with the new ones (`literal_pattern`,
`record_laws`, `dependent_update`, `rename_refused`), `define_test`
with calc's app file added, calc's differential and frontend parity
byte-identical, the full suite, CI for route 1.

**Slice 3.17 as built** (2026-10-01):

- **Rule 3.** `elab.shard`'s `elab_match_lit`: a `match` with a
  numeral row is rewritten to the `if` chain over the scrutinee's name
  — a local is its own name, anything else is bound once under a name
  no source symbol can spell — and elaborated as any term, in a `fn`
  and in a `def` alike. Refusals on the route: `match_not_exhaustive`
  (no last row), `literal_rows` (a row after the catch-all, a
  constructor row beside the numerals), `pattern_type` (a numeral row
  at a type other than `Nat` or `Int` — the classifier never checked a
  literal against the scrutinee's type, so `(match b (0 1) (_ 2))` at
  a `Bool` was `RUNNABLE` before this slice and is refused now where
  the signature has an L reading). `literal_pattern` stays the
  E-first fallback for a numeral *under a constructor* and for a
  symbol pattern (the old tree has 53 numeral rows and one nested
  numeral). A `fn` whose body is a literal match has one equation,
  as a body `if` does; the rows compute in K (`Eq.refl`).
- **Rule 4.** Pin `record_laws`: get after set, get of another field,
  set after set, a field through a `with` in a function, a field of a
  `make` — five theorems, each `Eq.refl`. `make` and `with` reach a
  `structure` of the file (`record.shard`, at the s-expression level
  as for a record): `make` is `NAME.mk` on the values in field order;
  `with` is `NAME.mk` on each field's entry or its projection of the
  subject, the subject bound once when it is not a name.
  `dependent_update` is syntactic and conservative: a field kept whose
  type's *source* names an updated field (a binder inside the type
  that reuses the field's name is refused too). It is a read error,
  like `record_make`: the expansion precedes the reading. **Limit:**
  a structure with parameters is out of the sugar's reach — its
  constructor takes them — and `make`/`with` on it are
  `record_make`/`record_with` with that pointer. The bootstrap reads
  no `structure` (an L form), so parity is untouched.
- **Rule 5.** E's table names the three entries `= < <=`
  (`prog.shard`); the erasure's registry rows emit them. The
  spellings before the rename are read as those entries in a module
  whose identity begins with `kernel` (`scope.shard`'s
  `toolchain_scope` — the toolchain's sources, the one tree route 1's
  v2 chain compiles) and are refused everywhere else as
  `renamed_primitive` with the new spelling, by the elaborator on the
  typed route and by the classifier on the E-first one; they stay
  reserved there, not free names, until the toolchain migrates. The
  Rust bootstrap reads both (`prim.rs`, `eval.rs`) and `eval dump`
  prints the V3 spelling whichever the source wrote, so frontend
  parity compares one name. **Found while building:** the bootstrap's
  native table and `kernel/reduce.shard`'s object table are tied by a
  conformance sweep (`lib.rs`); `= < <=` are native-only names, which
  the old kernel's table does not carry — no old-tree `fn` body
  writes them, and `=` there is a claim's head, which neither table
  sees. Stated in `prim.rs`; the sweep's lists are unchanged. The
  tool is `v3/tools/rename_cmp.py` (a list head only, never inside a
  string, a comment or a `quote`; `--check` for a migrated tree): 27
  sites in 16 files of `v3/std`, `v3/examples` and `v3/pins`. By its
  count `v3/kernel` has 665 sites in 52 files still to move (the
  scoping's 943 in 60 was a grep that also counted comments, strings
  and quoted symbols).
- **Rule 6.** `v3/std/fresh.shard`: the records `FreshName`, `Fresh`
  and `Minted`, `Fresh.start`, `Fresh.next`, and `Fresh.next_ne` —
  the names of two successive steps differ, through
  `Nat.succ_ne_self` and `congrArg` on the index (no axiom). The
  prefix's type is a parameter `P`, not the design's unstated one:
  text has no L reading before slice 3.18, and the toolchain's port
  will instantiate it with K's `Name`. 14 functions defined.
- **Rule 7.** The registry has expression rows (`erasure.shard`'s
  `realization_expr`): `Int.natAbs a` erases to `(let (a) (if (< #0 0)
  (- 0 #0) #0))`. calc's app file imports through `InvImage.wf` and
  `show_nat` takes the measure `(Int.natAbs n)`: 17 of 17 functions
  defined, `show_nat.dec_1` the one pending obligation, nothing
  `RUNNABLE`. `Int.toNat` has no row: it lies at line 552,397 of the export,
  far past the test prefix (corrected at slice 3.18 — this said it was
  not in the export).
  A callee that is a view's `sig fn` the environment lacks is
  `no_l_meaning` (`loader.shard`'s `callee_hits`; pin
  `route_sig_callee`): `has_eq` reads so now. `confusion_reachable`
  stays `no_l_identity`, correctly — its body quotes symbols; the
  scoping note above named it beside `has_eq` for the callee alone.
- **Gates run:** 146 loader pins (14 new: `literal_pattern`,
  `literal_open`, `literal_type`, `literal_rows`, `literal_nested`,
  `rename_refused`, `rename_refused_e`, `rename_ok`, `record_laws`,
  `struct_make`, `dependent_update`, `struct_params`,
  `measure_natabs`, `route_sig_callee`; `match_literal` narrowed to
  the nested numeral), `define_test` (7 checks: calc's app file and
  the fresh-name library added), calc's differential byte-identical
  over 21 inputs, the full suite and the bootstrap's unit tests
  (records §9 for the figures). Route 1 is CI's.

**Slice 3.18 — bytes and text: the wire's type, `String`'s
realization, the flip** (the user's ruling of 2026-10-01 on five
leans, the design the same day; §13 item 51). The slices table gave
this slice the byte and text adapters, item 37's flip, the literals
moved from 3.17 and deriving, "the wire's type ruled first".

*The ruling.* (1) **The representation is the byte list**: `UInt8`
runs as an integer, a `ByteArray` as the list of them, a `String` as
its `ByteArray` with the validity proof erased — no executor gains a
kind of value. Rejected: a packed buffer now (a fourth value in the
bootstrap and in `ev`, and a table of primitives each *trusted* to
match Lean's function; its benefit is the lowering's, behind the same
types); the interned atom (item 36's reading taken literally: every
string ever built interned, a file's contents included). (2) **The
externs carry `ByteArray`**, all six; text leaves by its bytes and
enters by a checked decoder. Rejected: `List UInt8` (list cells in the
host contract for good), `String` for paths and arguments (UTF-8
validation inside the boundary). (3) **Symbols are deferred** — they
and `Name` literals stay out of a `fn` body; item 36 is amended.
(4) **The flip is staged like the rename** — every file outside the
toolchain's own sources now, `v3/kernel/**` with route 1's chain; item
37's "at once" is amended. (5) **Deriving is its own slice**, 3.19; I
moves to 3.20.

*What the probes found* (scratch files; the pins' prefix and a longer
one cut from the export):

- **In a `def`, `"…"` is already a `String`**; a `fn` whose signature
  names `String`, `ByteArray` or `UInt8` is `no_l_identity
  route=e_first`. The obstacle is one binder: `UInt8` is a structure
  over `BitVec 8`, `BitVec w` over `Fin (2 ^ w)`, and a *value*
  parameter is neither a type nor a proposition, so §7.5's eligibility
  refuses `BitVec` and everything built on it.
- **A structure's projection in a `fn` body is refused today**:
  `(fn lo2 ((s Span)) Nat (Span.lo s))` is `REFUSE unknown_head:
  Span.lo` — the erasure has no realization for the projection
  function, the function falls to the classifier, and the classifier
  knows no L constant. Slice 3.17's structures were reachable from a
  `fn` by constructor and by pattern only.
- **An import costs its position in the export.** `(import Init
  NAME)` admits the prefix through NAME: `String` is line 9,848 (277
  declarations), `String.toByteArray` 78,604 — both inside the pins'
  prefix (100,851 lines, 5 s). `ByteArray.data` is 169,395,
  `UInt8.toBitVec` 250,325, `String.casesOn` 250,602, `String.append`
  253,597, `UInt8.casesOn` 263,243, `UInt8.toNat` 263,515: a load
  through there admits 3,400 declarations in 40 s on the bootstrap.
  `ByteArray.validateUTF8` is line 2,619,670 of 6,490,422.
- **Init's operations on these types have no derived view**: they are
  stated through instance projections (`HMod.hMod`, `++`) and loops
  the erasure refuses (`recursion_structure`). `UInt8.ofNat n` is
  `⟨BitVec.ofNat 8 n⟩`, which unfolds to `⟨⟨⟨n % 2 ^ 8, _⟩⟩⟩`.
- **The driver performs an extern by its short name alone**
  (`ev.shard`'s `perform`): prelude cells whatever the program
  declared.
- **Outside the toolchain's sources one function holds a string
  literal** (the pin `ev_string`); `v3/std` and `v3/examples` hold
  none.

*The rules:*

1. **Type realizations: six rows of the registry.** `Fin`, `BitVec`,
   `UInt8`, `Array`, `ByteArray` and `String` are **represented by
   their one runtime field** (law §4.4, "a runtime representation
   different from the constructors"; the simulation is the identity on
   that field, the erased fields its invariant). In E: the type keeps
   its name (`ByteArray`, not `(List UInt8)` — the lowering may give
   it another layout behind the same signatures), its type arguments
   kept and its value arguments dropped (`Fin n` is `Fin`); the
   constructor is its runtime argument, the field's projection its
   subject, a case analysis or a constructor pattern binds the field
   to the scrutinee. So a `UInt8` is an integer below 256, a
   `ByteArray` and an `Array α` are `Init`'s `List` cells, a `String`
   is its UTF-8 bytes. A row's type is **never an E inductive**: the
   classifier cannot cite its constructor (`unknown_head`), so the
   typed route and the E-first route cannot disagree on the cells.
   The erasure checks the shape at each use — one constructor, exactly
   one runtime field — and is `no_realization` otherwise; a row cannot
   misdescribe its type. No row for an **operation**: an operation is
   an S function over the constructors and projections, tied to
   Init's by a theorem where one is wanted. Rejected: the rule for
   every one-field structure (a native `structure` or `type` would
   change cells under the E-first functions that build it); rows by
   operation (`UInt8.ofNat` ↦ `mod`, each a trusted line).
2. **A projection function is its projection**, for every structure:
   a constant whose value is `λ params self. self.i`, fully applied,
   erases as that projection of its last argument — the match on the
   constructor for an E inductive, the subject for a row of rule 1.
3. **The literal.** Outside the toolchain's sources `"…"` is a
   `String` in every position. On the typed route it is K's literal,
   erased to its bytes as `Init`'s `List` cells — one literal, built
   once at link; in an E-first body the same literal, its static type
   `String`; `no_string` where the environment has no `String`. A
   literal whose bytes are not UTF-8 is `bad_string`: K reads a
   literal by decoding it, and only on valid bytes is the encoding of
   the decoded text the bytes written. In a module whose identity
   begins with `kernel` the rule before the flip holds (the byte list
   of the `List` in scope; a body holding one stays E-first), until
   route 1's chain is V3's own and the tool migrates the toolchain.
4. **The wire by declared type.** The driver builds and reads each
   value of an extern by the E type the program declared — a codec by
   identity, as §9's entry is: bytes at the prelude's `(List Int)` or
   at `ByteArray`, a list at the prelude's `List` or `Init`'s, an
   option, a pair (`Pair` or `Prod`), a flag (either `Bool`). The
   toolchain's signatures are `kernel/host.shard`'s, unchanged; a
   program under the naming law declares
   `get_args : World → Prod (List ByteArray) World`,
   `read_file : ByteArray → World → Prod (Option ByteArray) World`,
   `write`, `write_line : ByteArray → World → World`,
   `write_file : ByteArray → ByteArray → World → Prod Bool World`,
   `exit : Int → World → World`. A declared type the host's value
   does not fit is stuck (`extern`), as a value that was no byte list
   is today. The checked entry takes a `ByteArray` parameter.
5. **Symbols.** `(quote x)` in a function whose signature has an L
   reading keeps `no_l_identity route=e_first`. The direction, for
   the ruling that the toolchain's port will need: `Symbol` its own
   type over `String`, represented by the atom — a seventh row whose
   simulation is interning — rather than `String` itself, which rule
   1 makes a byte list.
6. **The libraries and the consumer.** `v3/std/bytes.shard` (bytes
   and numbers, a byte array and its list), `v3/std/text.shard` (a
   text's bytes; built into `bytes.shard`, the as-built below),
   `v3/std/host.shard` (the `World` and the six
   externs of rule 4), and calc's loop closed against the host:
   `v3/examples/calc/calc_main.shard`, an S program that reads its
   arguments and prints each line's result — the consumer §12.6 names
   ("calc's app step"). Its gate: its output on the differential's
   inputs is the model world's (`run_world`, which the differential
   already ties to the old tree), byte for byte.

*Not in this slice:* the checked decoder (`String.fromUTF8?` needs a
validator with its proof; no consumer turns input bytes into a
`String`, and the proof is I's); rows for `Char` and the wider
`UInt`s; `String`'s own operations (append, length, the decision);
a packed buffer; deriving (slice 3.19).

**Slice 3.18 as built** (2026-10-02):

- **Rule 1.** `erase.shard`'s `rep_row` names the six types.
  `l_to_etype` gives a row its own name over its type arguments
  (`row_param_roles`: a value or a proposition parameter dropped) and
  `ind_eligible` answers no for it, so the E table never holds its
  constructor. `erasure.shard`: the constructor is its runtime
  argument, a `Proj` its subject, `T.casesOn` and `T.rec` a `let` of
  the scrutinee (`erase_row_cases`), a constructor pattern in a
  matcher's row its field's pattern — `row_field` checking at each use
  that the structure has one constructor and one runtime field. The
  classifier's pass treats a row's type as a known type with no
  constructor of its own: a constructor pattern on it is
  `pattern_type`, an `if` on it `if_type` (pin `row_e_match`) — an
  E-first body may pass a row's value on, never read its cells. As
  dumped: `(UInt8.ofBitVec (BitVec.ofFin (Fin.mk (Nat.mod n 256)
  PROOF)))` is `(Nat.mod #0 256)`; `ByteArray.mk (Array.mk l)`, its
  pattern and `String.toByteArray s` are `#0`. `Array` was an E
  inductive with a `mk` cell since slice 5b and is a row now (pin
  `realize_view`: `Array.toList` and `Array.push` by their views, as
  before).
- **Rule 2.** `proj_fn_app`: a definition whose value under exactly
  as many lambdas as it has arguments is `self.i`. `(Span.lo s)` in a
  `fn` is `(match #0 ((mk _ _) #1))` and the function is defined (pin
  `struct_proj`). The rule is also what lets a body call Init's
  `String.toByteArray`, `Array.toList`, `Fin.val` and
  `ByteArray.data` with no `realize`.
- **Rule 3.** A `LitStr` erases to the one literal over `List.nil` and
  `List.cons`. A literal whose bytes are not UTF-8 is `bad_string` at
  the **elaborator**, in a `def` as in a `fn` (one text, one meaning;
  `utf8_ok`, moved from `ev.shard` to `util.shard`), and the erasure
  keeps the check for the bytes it emits; the classifier reads `"…"`
  the same way outside the toolchain's sources, `no_string` without a
  `String` in the environment; the linker interns Init's
  cells whether or not the program declares them. The pre-definition
  keeps a string-holding body E-first in the toolchain's sources only.
  No file of the toolchain changed meaning and the bootstrap is
  untouched: parity compares what it compared. Migrated: the pin
  `ev_string`, the one function outside the toolchain that held a
  literal (it was `(fn g () (List Nat) "abc")`, `type_mismatch` now).
- **Rule 4.** `ev.shard`: a linked extern carries its declared types
  and `perform` builds and reads each value by them (`wb_tags` for
  bytes, `wl_tags`, `wo_tags`, `wp_first`, `wf_tags`); the result's
  type is decided **before the host is asked** — its codec, and the
  World's token returned at the type it was passed at (the extern's
  last parameter's): a `write_file` whose declared result has no
  codec writes nothing, a `write_line` declared to return a pair
  prints nothing. An extern whose declared type does not fit is
  stuck, `RUN: stuck extern in main.write_line` (pins `wire_bad`,
  `wire_bad_result`). The checked entry takes a `ByteArray` (pin
  `wire_s`'s `copy`). **Not built: a check at link.** An extern's
  signature is checked where it is performed, because `ev` is pure and
  stops at any extern — a program may declare one the host does not
  have (`define_runnable`'s `read_byte`, `ev_test`'s `shout`).
- **Rule 5.** As designed; the elaborator's and the pre-definition's
  messages name the deferral.
- **Rule 6, built as two libraries, not three.** `v3/std/bytes.shard`
  holds text's one adapter too (`Bytes.ofString`): a `text.shard` of
  one function was a file for its name. In it: Init's `BitVec.toNat`
  and `UInt8.toNat` by their derived views (`(fn UInt8.toNat 0 (UInt8)
  Nat (BitVec.toNat 8 #0))` — the width is an argument of Init's
  function and stays one); `Byte.ofNat`, with `Byte.ofNat_eq :
  Byte.ofNat n = UInt8.ofNat n` and `Byte.toNat_ofNat`, both
  `Eq.refl` (K unfolds Init's instance projections, which the erasure
  cannot follow); `Bytes.ofList` and `Bytes.toList` with both round
  trips (`ofList_toList` is K's structure eta); `Bytes.ofNats`,
  `Bytes.toNats`. Two realized, eight defined, four theorems, no
  axiom. `v3/std/host.shard`: the `World` and the six externs.
  `v3/examples/calc/calc_main.shard`: `codes_of` and `bytes_of`
  defined, `drive_host` typed with a callee-local, `perform` and
  `main` `no_l_meaning`. Run with the differential's 21 lines as its
  arguments it prints the model world's lines, byte for byte
  (`kernel/test/calc_main_test.sh` decodes `run_world`'s value from the
  harness).
- **Found by review** (a second reader over the kernel's diff, its
  probes re-run before the fixes). *An E-first body could take a row
  apart*: `(match b (List.nil 0) ((List.cons h t) h))` on a
  `ByteArray` was `RUNNABLE`, the classifier's pass skipping a type
  it had no entry for — the cells agreed, so nothing ran wrong, but
  the layout was no longer the registry's to change. *`write` and
  `write_line` ignored the declared result*: declared to return a
  pair, the line was printed and the program stuck at the next
  `match`; and a pair's second component was never compared with the
  World's type. Both closed as above. Left as it was, and older than
  this slice: an E-first body is not checked against its declared
  result (§12.1's "return types unchecked" row), so one can return a
  `(List Nat)` as a `ByteArray` and `write` emits each number's low
  byte.
- **Found while building.**
  *The fallback hid the obstacle.* `(fn cat … (ByteArray.mk (Array.mk
  (List.append a b))))` was `REFUSE unknown_head: ByteArray.mk`: the
  typed route's obstacle was `List.append` with no realization, the
  function fell to the classifier, and the classifier's refusal — a
  row's constructor — was true and beside the point. A classifier
  refusal after a typed-route obstacle now carries it (`loader.shard`'s
  `load_e_after`): `… — read E-first after the typed route's
  no_realization: List.append: no realization attached`.
  *An import costs its position.* The libraries import through
  `UInt8.toNat` (40 s a load on the bootstrap, against 5 s): the test
  prefix has a second chunk, `fixtures/init_prefix_str_tail.ndjson`
  (lines 100,852 to 263,515 of the export, 8.6 MB; to line 274,616
  since slice 3.21, for `arith`'s product and quotient), given after the
  first by the three tests that load them; the pins stay on the first.
- **Open, stated.** The checked decoder (`String.fromUTF8?`: a
  validator and its proof against `ByteArray.IsValidUTF8`, with I).
  Rows for `Char`, the wider `UInt`s and `USize`. `String`'s own
  operations. A `realize` by a proven-equal S function, so that a body
  calls Init's name (`UInt8.ofNat`) rather than a sibling
  (`Byte.ofNat`) — the supplied form's body is E and cannot build a
  row's value. The toolchain's flip and symbols, with route 1's chain.
  The six rows and the literal's bytes are lines of the erasure and
  trusted with it (§7.1).
- **Gates run:** 156 loader pins (10 new: `row_types`, `row_e_first`, `row_e_match`, `struct_proj`, `str_e_first`, `str_no_string`, `str_bad_utf8`, `wire_s`, `wire_bad`, `wire_bad_result`; `ev_string` rewritten), `rows_test` (10 values under `ev`), `wire_test` (15 checks against the host), `calc_main_test` (byte-identical to the model world over 21 inputs), `define_test` (9 checks: the two libraries added), calc's differential byte-identical over 21 inputs, parity byte-identical over 28 closures and 111,681 declarations, route 2, K's clients, the full suite's 32 entrypoints with 0 failed. `v3/build.sh` built these sources into a scratch binary that byte-ties the interpreter on the 3,000-line fixture.

**Slice 3.19 — deriving under a declared policy** (the user's ruling
of 2026-10-02 on five leans, the design the same day; §13 item 52).
Law §5.1 gives a new type decidable equality, an ordering and a
diagnostic rendering without a hand-written instance, as *generation
under a declared policy*. Before this slice `(if (= a b) …)` at a
type of the program's was `no_decision`: the elaborator's lookup knew
`Nat`, `Int` and `Bool`.

*The ruling.* (1) **A use site finds a derived operation through a
derivation table**: one entry per type and capability, no search, a
second visible entry a refusal; `=` in an `if` or where a `Bool` is
expected consults it. Rejected: named functions only (the type is
not "usable in an `if`", and every site is rewritten at Stage 3);
instance resolution pulled forward (Stage 3 is a slice of its own).
(2) **One named policy, `structural`, written at the derivation** —
constructors in declaration order, fields left to right,
lexicographic: Lean's own `deriving Ord`. Rejected: a package default
(V3 has no package-level form, and one policy needs no default).
(3) **The ordering is `compare`, returning Init's `Ordering`, with no
laws.** Rejected: the laws now — each is a proof term over the squared
or cubed constructor cases without I, an ordinary tactic proof with
it; they come with the first verified keyed map. (4) **The rendering
is bytes in canonical S** — `(TPair (TNum 3) TPlus)`. Rejected:
Lean's `Repr` (a pretty-printer document type and its layout engine);
a `String` (a computed `String` needs `String.append`, whose equation
is I's); deferring it. (5) **Supported now: a type without
parameters, directly recursive or not, and Init's `List`, `Option`
and `Prod` at closed arguments.** Rejected: user types with
parameters and types nested through `List` now (the constructor-
structure bundle of a parametric type and recursion through a nested
recursor first — the toolchain's `SExpr` and `Expr` wait for them).

*The rules.*

1. **The form.** `(derive TYPE CAPABILITY…)`, a declaration of the
   file it stands in. TYPE is a closed type: a name, or an inductive
   applied to closed types — `Tok`, `(List Tok)`, `(Prod Nat Tok)`.
   A capability is `eq`, `(ord structural)` or `render`, each
   generated; or `(eq by NAME)`, `(ord by NAME)`, `(render by NAME)`,
   each registering a procedure written by hand, after its type is
   checked (`derive_by`) — at any closed type, since it needs no
   constructor. The generated functions live in the
   deriving module under the type's mangled name — its head's last
   component, then its arguments' — so `Tok.decEq`, `List_Tok.compare`.
2. **Generation is source.** A derivation writes `fn` forms and the
   loader reads them as if the file held them at that point: the one
   front end elaborates them, K checks the definitions and their
   equations, the programs are their erasures, and each has its
   ordinary `DEFINE` record. Nothing is trusted and no second route
   to a program exists (guard 1). What the generator reads of the
   type it reads from K's environment — the constructors and their
   fields' types at the type's arguments.
3. **`eq`** is `NS.decEq ((a T) (b T)) (Decidable (= a b))`: a match
   on `a`, then on `b`, a row for every pair of constructors. The
   same constructor decides its fields left to right by their
   procedures — `Nat.decEq`, `Int.decEq`, `instDecidableEqBool`, the
   table's entry, or the function itself on a recursive field —
   through `dite`, the result `isTrue` by congruence from the fields'
   equations and `isFalse` by the constructor's injectivity (its
   `C.inj`, or the projections of a one-constructor type); two
   different constructors are `isFalse` through `NS.ctorIdx`, the
   constructor's index, generated beside it (Lean 4.33 gives every
   inductive one) as a derivation of its own, `cap=idx`, which later
   derivations at the type share through the table. The rows are
   quadratic in the constructors, as Lean's handler was before its
   index rewrite. A generated variable is no symbol a source can
   write (`x 1`), so no constructor of a field's type captures it.
4. **`(ord structural)`** is `NS.compare ((a T) (b T)) Ordering`:
   the same constructor compares its fields left to right, the first
   that is not `eq` decides; different constructors compare by
   `NS.ctorIdx`. A `Nat` or `Int` field is
   `(if (< x y) lt (if (= x y) eq gt))`, which is Init's `compare`
   there unfolded; any other field its table entry. The declaration
   order *is* the convention: it is in the value (the indices), so
   reordering the constructors changes the function, its hash and
   every theorem that rests on it.
5. **`render`** is `NS.renderOnto ((a T) (rest (List Nat))) (List Nat)`
   — the value's text before `rest`, linear in its size — and
   `NS.render ((a T)) ByteArray`, which is `Bytes.ofNats` of it on
   the empty rest (`v3/std/bytes.shard`; without the library in scope
   the derivation is `derive_needs`). A constructor with fields is
   `(NAME F…)`, one without is `NAME`, the name its last component. A
   field renders by its table entry; `Nat`, `Int`, `UInt8` and
   `String` have theirs in the library. A field that is erased — a
   proof, a type — or whose type is a view's opaque type prints `_`
   (law §5.1: "an opaque placeholder rather than claim to inspect
   every value").
6. **The table.** An entry is (type, capability, procedure, deriving
   module); its policy is in its record. The type is keyed in normal
   form — a definition that stands for a type is the type. An entry
   is visible in a file exactly when its deriving module and its
   procedure are — both in the file's import closure, whatever was
   loaded before the file — so it resolves as a name does and
   crosses no view (§8.5). A view exports a
   capability as what it is, a `sig fn` of the capability's type, and
   a consumer registers it: `(derive lib.Handle (eq by
   lib.Handle.same))`. `eq` at `Nat`, `Int` and `Bool` are the
   elaborator's own rows, as before; `ord` at `Nat` and `Int` is
   rule 4's expression. Everything else is an entry a `derive` made:
   the library derives `Bool`, registers `UInt8`, `ByteArray` and
   `String`'s hand-written procedures and the renderers of `Nat` and
   `Int` (`v3/std/derive.shard`); a program derives its own types and
   the instantiations it uses — `(derive (List Tok) eq)` — because E
   has no function values: `List`'s equality cannot take its
   element's as an argument, so each instantiation is its own
   first-order function. Each derivation is recorded: `DERIVE PROC
   type=T cap=C policy=P fields=PROC,…` — the field procedures it
   selected, which the definition also cites, so they are in its
   dependencies and its hash (law §5.2: a selection is recorded,
   never rediscovered).
7. **The use site.** `(if (= a b) …)` and `(= a b)` where a `Bool`
   is expected, at a type with a visible `eq` entry, take the entry's
   procedure as the decision; two visible entries are
   `ambiguous_decision`; none is `no_decision`, now with the pointer
   to `(derive T eq)`. `compare` and `render` have no operator
   spelling: a program calls `Tok.compare` and `Tok.render` by name
   until Stage 3 resolves them.
8. **Refusals**, the type untouched in every case (law §5.1):
   `derive_type` — TYPE is not a closed inductive type visible here
   (a view's sig type has nothing to generate from: the module derives
   and exports, the consumer registers, §8.5), or it is a
   proposition, `Nat` or `Int` (the integer), or an inductive applied
   to a value, `(Fin 3)`;
   `derive_shape` — an indexed or mutual type, a type without
   constructors, a type nested through another (a field `(List T)`
   of `T`), a field whose type depends on
   an earlier one, an erased field under `eq` or `ord`; `derive_field`
   — a field's type has no entry for the capability, naming the
   constructor, the type and the form to write; a function-valued
   field (no decidable equality; and the type has no program to
   render); `derive_needs` — a constant the generated source cites is
   not in the environment (`Decidable`, `Ordering`, `congrArg`, a
   constructor's `inj` for a type with parameters, the library);
   `derive_duplicate` — an entry for the type and capability is
   already visible; `derive_by` — the named procedure's type is not
   the capability's; `name_taken` with `derive_failed` — a constant
   already bears a name the derivation generates: it is never taken
   for the derivation's, and nothing is registered.
9. **What the elaborator gains for it**, each a general rule. A
   `match` on a local the expected type mentions generalizes it —
   the motive `λ d. RET[d]`, each row read at RET with its pattern
   (Lean's rule for the expected type; the motive was constant
   before, so a result typed by its scrutinee could not be written) —
   which is how `decEq`'s `Decidable (= a b)` is refined row by row.
   A hypothesis about the scrutinee is not generalized (Lean reverts
   those too): a match whose rows cannot be read at the generalized
   type is read with the constant motive, as before. An operator inside an
   E-type position — a `fn`'s `(Decidable (= a b))` — takes its
   levels by unification, not the position's level 0.

**Slice 3.19 as built** (2026-10-02):

- **Rules 1, 2 and 6.** `kernel/derive.shard` is the generator: it
  reads the form, elaborates TYPE, opens each constructor at the
  type's arguments through K, selects a procedure per capability and
  field, and writes the `fn` forms as s-expressions.
  `loader.shard`'s `load_derive` hands each to `load_decl` — the path
  of a form of the file — after registering its head as the file's
  own heads were (an E-first body finds the function too), and
  registers the entry once K has the definition: the table is
  `Load.derived`, carried to the elaborator in the scope's visible set
  (`scope.shard` `Derived`, `derived_for`), the record `DERIVE`. A
  generated function that was not defined registers nothing
  (`derive_failed`, beside its own record). The generated source cites
  every constant by its identity — an imported one under `Init.` —
  and the type the same way, so it reads the same whatever the file
  opened: a type whose constructor bears its name
  (`(type Card (Card …))`) is ambiguous written bare in a term. A
  `derive` stands before its first use; all its capabilities are
  analysed before anything is generated, so one refused field refuses
  the form.
- **Rule 3.** As designed; `(derive Tok eq)` at
  `(type Tok (TNum Nat) (TPlus) (TPair Tok Tok))` erases to the match
  on both arguments an author would write, `(if (Tok.decEq #3 #1) (if
  (Tok.decEq #2 #0) (isTrue) (isFalse)) (isFalse))` under the pair's
  row. Distinct constructors: `Nat.ne_of_beq_eq_false` at the two
  index terms, `congrArg NS.ctorIdx h` its premise — no `noConfusion`,
  so Init's parametric types need none of the bundle. Injectivity:
  `C.inj` where the environment has it (the bundle's, Init's
  `List.cons.inj` and `Option.some.inj`), else the projections of a
  one-constructor type (`Prod`, whose `Prod.mk.inj` lies at line
  1,985,485 of the export). **Wider than the ruling's fifth lean:** a
  one-constructor type with parameters derives through its
  projections (pin `derive_params`'s `Wrap`); with two constructors it
  is `derive_needs`, naming the missing `inj`. Init's own
  parameterless types derive as the program's do (`Bool`, `Ordering`).
- **Rule 4.** As designed; the wildcard row of a constructor that is
  neither first nor last compares `(NS.ctorIdx b)` with its own index.
  `Ordering.casesOn` is among the constants the generated source needs
  (line 172,785 of the export).
- **Rule 5.** As designed. A list renders by its constructors,
  `(cons a (cons b nil))`, not as `(list a b)`. **Narrower than the
  ruling's fourth lean:** a function-valued field is refused
  (`derive_field`), not printed `_` — a type that holds a function is
  not an E type, so it has no program and no value to render.
- **Rule 7.** As designed, and one refusal the design did not have:
  on the E-first route — a body that calls an extern, until phase 4's
  World — `=` is the classifier's integer primitive, and at an
  inductive it was accepted and stuck at run. It is `prim_type` now,
  with the pointer to call the decision by name (pin
  `derive_e_first`).
- **The library** (`v3/std/derive.shard`, importing through
  `UInt8.toNat`): `Bool`'s ordering and rendering derived; `UInt8`'s
  equality by its bit vector's number (`BitVec.eq_of_toNat_eq` and K's
  structure eta), `ByteArray`'s by its list's — `(derive (List UInt8)
  eq (ord structural))` — and `String`'s by its bytes'
  (`String.toByteArray_inj`), each with its ordering, registered by
  `(CAPABILITY by NAME)`; the renderers of `Nat` (decimal, its measure
  discharged by a term over `Nat.div_lt_self`), `Int`, `UInt8` and
  `String` (quoted, the reader's escapes). Fourteen procedures and
  one constructor index, 26 functions defined, no pending obligation
  and no axiom of the library's — `propext` reaches the decimal
  renderer and its two callers through Init's `Nat` lemmas in the
  descent's proof. **Wider than the
  lean**, which gave these three types equality only. `ByteArray` has
  no renderer.
- **Not built.** Lean's instance constants: `T.decEq` has `DecidableEq
  T`'s type unfolded and is the constant Stage 3 registers; `instOrdT
  : Ord T` would hold a function, has no program and no reader before
  Stage 3, which writes it in one line over `T.compare` — the lean
  said the generated constants would have Lean's instance types, and
  for the ordering that waits. The ordering's laws (the ruling). An
  entry crossing a view by itself: the view states the procedure and
  the consumer registers it (below). Types with parameters and two
  constructors or more, nested and mutual types (the ruling). A
  package default for the policy. Operator spellings for `compare`
  and `render`. **The toolchain's own sources cannot derive yet**:
  the bootstrap reads their E half and generates nothing (`derive` is
  its `UnknownForm`), so `v3/kernel/**` keeps its hand-written
  `*_eq` and `show_*` until route 1's chain is V3's own — as the flip
  and the rename wait (items 37, 50).
- **Found while building.** (a) *An `if` whose condition is a
  self-call was refused*: `(if (f x) …)` is `ite (f x = true) inst …`,
  and `define.shard`'s `pj_ite` rewrote the decision and left the
  proposition holding the self-local — `K refused the definition
  (fvar_in_value)`. Both go through the walk now (pin `if_rec_call`).
  (b) *Under a measure that call asked for two proofs of one fact* —
  it stands in the proposition and in the decision: an obligation is
  named once per statement (`obl_stating`), so `f.dec_2` no longer
  duplicates `f.dec_1`, nor do two identical calls in one scope (pin
  `measure_if_call`). (c) *`and`, `or`, `not` on `Bool`s cited
  constants the export does not have*: Lean 4.33's are `Bool.and`,
  `Bool.or`, `Bool.not`; a `def` was `unknown_constant: and`, a `fn`
  `unknown_head: and`. The elaborator cites the real names and the
  registry has their rows — `(if a b false)`, `(if a true b)`, `(if a
  false true)`, the `if` each definition is — and `a = b` at `Bool`
  in general position, the test `(if a b (if b false true))`, which a
  derived equality's `Bool` field needs (it was `no_realization` off
  `= true`). Evidence as the registry's other
  rows have it: every row by `Eq.refl` on K's side (pin `bool_ops`),
  the programs run by `derive_test.sh`. (d) *A `fn`'s result
  `(Decidable (= a b))` could not be read*: an operator inside an
  E-type position took the position's level 0 for `Eq` (rule 9). (e)
  *The refusal for (d) named the wrong thing*: a signature with no L
  reading falls to the classifier, whose `unknown_type: =` hid the
  cause; a classifier refusal after a skipped signature now carries
  what the signature's reading met (`load_e_skipped`), as slice 3.18
  did for a body; so does one after an unknown constant that is no
  function — `(or a b)` under a prefix that stops before `Bool.or`
  was `unknown_head: and` and now names `unknown_constant: Bool.or`. (f) *An explicit numeral argument is still
  unassigned when a later argument is checked* (§5.1's "numerals
  last"): `(@Nat.ne_of_beq_eq_false 0 1 (Eq.refl false) h)` is
  `type_mismatch` at the third argument. **Left as it is**; the
  generator writes the index terms, which are closed.
- **Across a view** (rule 6; §8.5's "the module derives and
  exports", pin `derive_sig`): the implementation derives its type
  and defines `Handle.same` as the derived decision; the view states
  `(sig fn Handle.same ((a Handle) (b Handle)) (Decidable (= a b)))`;
  the consumer registers it with `(eq by lib.Handle.same)` — a
  parameter there, accepted as any constant of the capability's type —
  and its own `Job` over a `Handle` derives through the entry, its
  decision calling the parameter. For the view to hold that signature
  the classifier's type reader takes `(Decidable P)` as the type's two
  cells, its proposition unread (the erasure's reading; it was
  `unknown_type: =`).
- **Rule 9** is general: a proof by cases is a `match` in S now (pin
  `match_dependent`: `not (not b) = b`, one row per constructor).
- **Found by a second reader over the diff**, each reproduced, fixed
  and pinned. (a) *A registry row spoke for any constant bearing its
  name*: a module `Bool` declaring `or` under a prefix that stops
  before Init's ran as the row's `(if a true b)` while K proved the
  module's function — a clean load, a program that disagreed with
  its definition. The name-keyed rows now hold for a constant that is
  not a native declaration (`erase_const`'s `k`; pin
  `registry_native`). (b) *A `Decidable` had two runtime
  representations, since before the slice*: the registry's decisions
  erased to the bare test in every position, so a match by
  constructor on `(Nat.decEq a b)` took the catch-all row while K
  proved the other; the slice made such values ordinary (a `fn` may
  return one). A decision is now its test only where an `if` reads it
  — `ite`, `dite`, `decide`, a `casesOn` read back as an `if`
  (`erase_cond`) — and `(if TEST isTrue isFalse)` as a value; the
  programs of every existing `if` are unchanged (pin
  `decidable_cells`, run on the host against K's theorems). (c) *A
  derivation took a constant that bore its name for its own*: a
  hand-written `T.ctorIdx` or `T.compare` was reused or registered
  under `policy=structural`. A generated name that is taken is
  `name_taken`, the index is shared through the table only, and a
  failed form registers nothing (pin `derive_name_taken`). (d) *An
  entry registered by `(CAPABILITY by NAME)` was visible wherever the
  procedure was*, so a file's reading depended on what the loader had
  been given before it; an entry carries its deriving module (pin
  `derive_visible`). (e) *Rule 9 broke rows it should read and proofs
  that loaded before*: a `_` sub-pattern's local was outside the
  scope a metavariable's value may mention (it is bound now, under a
  name no source spells); a row that used a hypothesis about the
  scrutinee no longer typed (the constant motive is the fallback).
  (f) *`eq` at a container of a `Prod` was refused by its own
  source*: `(@List.nil (Prod Nat Nat))` asks `Type ?u` of `Sort (max
  1 1)`, and level unification compared a closed level unnormalized —
  older than the slice, fixed in `unify_level`. (g) *A self-call in a
  branch under a self-call condition, under a measure*: its
  obligation quantified the branch's hypothesis, which states the
  function's own value (`dec_2 fvar_in_type`); the obligation is
  stated without such locals. (h) What had no derivation was refused
  late or obscurely: a type without constructors, a proposition, an
  argument that is a value, `Nat` — each refused up front with its
  reason (pin `derive_refusals`); a generated variable could be read
  as a constructor of the field's type; the table's key was not
  normalized. **Left, and known:** a let bound to a self-call under
  a measure still leaves a later call's obligation open-termed
  (`fvar_in_type`, as before the slice); a measured function whose
  result type depends on its arguments is `app_type_mismatch` with no
  pointer; `erase_bool_eq` writes its second operand's code twice
  (one path runs).
- **Cleanup.** `realize.shard` opened `kernel.prog.EType` and used
  none of its constructors since slice 3.18 moved `etype_eq` (the
  loader's `UNUSED` record said so on every load); removed.
- **Gates run.** 176 loader pins (20 new: `derive_eq`,
  `derive_containers`, `derive_field`, `derive_nested`,
  `derive_params`, `derive_policy`, `derive_duplicate`, `derive_by`,
  `derive_needs`, `derive_e_first`, `derive_sig`, `derive_name_taken`,
  `derive_refusals`, `derive_visible`, `registry_native`,
  `decidable_cells`, `match_dependent`, `if_rec_call`,
  `measure_if_call`, `bool_ops`); `define_test.sh` (12 checks);
  `derive_test.sh` (14 checks: the library's 14 procedures, the
  example program's ten lines on the host — every row of Init's Bool
  connectives and of `Bool`'s equality among them, the same list K
  decides; `v3/examples/derive/derive_main.shard` — and
  `convention.shard`: two types with the same constructors in
  opposite order compare oppositely, and a type over a hand-written
  ordering records the procedure it selected, T1's fixture); the
  gates of every slice (records §9 for the figures).

**Slice 3.20 — I's opener: the certificate core** (the user's ruling
of 2026-10-05 on six leans, the design 2026-10-06; §13 item 53). Law
§7 fixes the shape: I is a certificate language, replay elaborates it
to P without search and K checks P; §7.2 names the vocabulary and the
replay contract; §7.3 wants the elaborator exposed as E functions.
Before this slice a `theorem`'s proof was `(exact TERM)` or `sorry`
(131 theorems in V3, every one an explicit term) and `by` was
`reserved_form`.

*The ruling.* (1) **Three slices**, a gate between each: 3.20 I's
data, `(by …)` in a `theorem`, `elaborate(I)` to P, the forms with no
engine behind them, and `goal_of`/`step` as E functions over one
derivation; 3.21 the two forms that carry an engine — `simp_only` (a
bounded rewriter over a lemma list) and `arith` (the Farkas
certificate through `Lean.Omega`'s lemmas) — and calc's 100 claims
closed; 3.22 the producers — `tools/prove` re-pointed at I, the
sidecar and the pin store (law §7.5), the engine. Rejected: one slice
(no gate for a full window or more); the forms before the API (the
API is cheap when I's data is shaped for it from the start). (2) **The
tactic block in source is I.** What an author writes inside `(by …)`
is I's concrete syntax; a node that omits a consequential choice
names the versioned default reconstruction (`(rw LEMMA)` with no
occurrence: the first match in the traversal of rule 7, every
occurrence of it), and the pin records what was written. Rejected: a
tactic surface distinct from I, translated down (two languages, two
versions). The user's condition — "if it doesn't invite too many
compromises" — is rule 2's test: a form is in I only if its elaboration
is a function of the node and the goal. (3) **No subterm order.**
`induction` carries its recursor; `wf` is `WellFounded.fix` over
`(measure f)` through `InvImage.wf`, the strong hypothesis stated as
`∀ y, f y < f x → P y`, every descent fact a goal. Rejected: v2's
`subterm-induct` and `(below)` (a V3-only logical rule in a
Lean-rule-exact target; `(below)` discharged `⊰` syntactically, which K
cannot check). Porting cost accepted by the user: 52 `(below)` and 8
`subterm-induct` sites re-spell at port time. (4) **Computation is
conversion.** `reduce` is a `show` of the normal form the elaborator
computes, checked by K's conversion — no `Eq.refl` chain, no
reflection until phase 4; `unfold f` is `rw` by `f`'s equations;
`decide` is `of_decide_eq_true (Eq.refl true)` with the decision the
derivation table or the elaborator's lookup gives. Rejected: a `simp`
with implicit lemma sets (search at replay). Porting cost accepted:
v2's `reduce`/`simp`/`compute` (38,000 sites) are one node here.
(5) **Calc's claims by hand**, re-spelled in V3's idiom — Prop
hypotheses `(<= 0 x)` where v2 wrote `(= (le 0 x) True)` — as the
worked-examples set; §12.3's migration tool waits for the 4,900-claim
port and is calibrated on `std`. (6) **The API is sequential at the
opener**: I's data has the law's graph shape (explicit child goals,
scope-safe locals, display names aside), the API is `goal_of`,
`applicable`, `step`, `elaborate` over one derivation; the
transactional `attempt`/`commit`, snapshots, owners and parallel
regions of law §6 wait for the engine's slice, where they have a
caller; nothing is stored — K checks P at every build, the content
store comes with the producers.

*The rules.*

1. **The form.** A `theorem`'s proof is `(exact TERM)`, `sorry`, or
   `(by STEP…)`; a `fulfills` takes the same three. A block is a
   sequence of steps on **one goal**: each step but the last turns the
   goal into one goal; the last closes it. A step that opens several
   goals carries each one's block inline, so a derivation is a tree
   whose children are explicit (law §7.3): `(cases x ((zero STEP…)
   (succ (n) STEP…)))`. A block that ends with the goal open is
   `goal_open`; a step after a closing one is `goal_closed`. Every
   refusal names the step by its index in its block and shows the goal
   it faced, as `goal_of` renders it (rule 10).
2. **A goal is a closed proposition.** `Goal` is a metavariable whose
   type is the goal's statement closed over its named context — `∀
   (x : Nat) (h : x = 1), P x` — and an occurrence of a goal in a term
   is the metavariable applied to the context's locals (R68, rule 10
   of slice 3.16: the stable creation telescope, each occurrence an
   instantiation). Closing a binder over a goal then abstracts the
   occurrence's arguments, never the hole, so a goal is filled after
   its binders closed with no delayed assignment: the filling is a
   lambda over the telescope, beta-reduced at instantiation. The
   named context is the Pi's binders; display names are the binders'
   names and identity is the fvar a step opens them as. The theorem's
   own binders are in the first goal's context, as Lean's are.
3. **A step is a function of the node and the goal.** `step(goal,
   node)` opens the goal's context as locals, acts on the target, and
   answers `StepDone(term)` — the goal's filling, a closed term over
   the subgoals' metavariables applied to their telescopes (the
   construction record of law §7.3, as data) — with the subgoals in
   order, or a refusal. `elaborate(I)` runs `step` on each node of
   the tree depth-first and instantiates; the result is P, one closed
   proof term K checks as the theorem's value. No node consults a
   changing table or runs a search: `rw`'s match, `apply`'s
   unification and `reduce`'s normal form are deterministic in the
   node, the goal and the environment.
4. **The vocabulary of 3.20.** `(intro NAME…)` — the target is a Pi
   after whnf, each name takes one binder (`intro_no_pi`). `(exact
   TERM)` — the term at the target; `rfl` — `Eq.refl` at `a = b` by
   K's conversion (`rfl_failed` shows both sides after whnf); `(have
   NAME PROP STEP…)` — the cut: PROP a proposition, its block a child
   goal under the current context, the rest of the block continues
   with `NAME : PROP` in the context; `(show PROP)` — the target
   replaced by PROP where K's conversion agrees (`show_mismatch`);
   `(apply TERM STEP-BLOCK…)` — TERM elaborated without an expected
   type, its type's Pi-telescope opened with a metavariable per
   binder for exactly arity(TERM) − arity(target) binders after whnf,
   the conclusion unified with the target; each metavariable still
   unassigned whose type is a proposition is a child goal in binder
   order, its block the next in the list (`apply_goals` names the
   count and the premises when the list is short or long); a
   non-proposition left unassigned is `apply_unsolved` (a witness
   goal is the graph's, after 3.20); `(cases TERM ((CTOR (NAME…)
   STEP…)…))` — TERM a local of an inductive type without indices
   (`cases_indexed` otherwise: `injection`/`subst` are later forms),
   the motive the target generalized over it (slice 3.19's rule 9),
   `T.casesOn` applied, each minor premise's telescope opened under
   the names given (`cases_fields` on a count mismatch), one block
   per constructor in declaration order, each constructor named once
   (`cases_rows`); `(induction TERM ((CTOR (NAME… IH…) STEP…)…))` —
   the same over `T.rec`, the recursive fields' hypotheses named
   after the fields; `(wf MEASURE TERM IH STEP…)` — TERM a local, the
   measure a term over it into `Nat`, `WellFounded.fix (InvImage.wf
   MEASURE Nat.lt_wfRel.wf)` with the target as motive, the block
   under `IH : ∀ y, MEASURE[y] < MEASURE[TERM] → target[y]`;
   `(decide)` — the target's decision through the table of slice 3.19
   or the elaborator's lookup at `Nat`, `Int`, `Bool`, the term
   `of_decide_eq_true (Eq.refl true)`, K's conversion the work
   (`decide_failed`, `no_decision`); `(unfold NAME…)`, `(reduce
   SIDE?)` and `(rw …)` — rules 6 and 7; `sorry` — rule 9.
5. **Rewriting is `Eq.mpr`.** A rewrite of the target by `h : a = b`
   abstracts every chosen occurrence of `a` into a motive `λ x.
   target[x]` and fills the goal with `Eq.mpr (congrArg motive h)
   ?g'` where `?g' : target[b]`; the occurrence path is the motive —
   the law's guarded path is a position in a term view with its
   expected subterm, and the motive carries both. `congrArg` and
   `Eq.mpr` are Init's (lines 3,464 and 14,083 of the export, inside
   the test prefix). A rewrite at a hypothesis (`(rw LEMMA (at H))`)
   is Eq.mp on the hypothesis under a `have` of the same name (later
   in 3.20 if calc needs it; otherwise 3.21).
6. **`unfold` and `reduce`.** `(unfold f)` rewrites each occurrence of
   `f` at full arity by the first of `f`'s equations (`f.eq_N`, slice
   3.13) whose left side matches it first-order, to a fixpoint under a
   budget; an equation proven by `Eq.refl` is applied by conversion
   and contributes no term, an equation that is propositional (a
   measured `fn`'s, slice 3.14) is a rewrite by rule 5; a `def` with
   no equations is delta-reduced by conversion. No match anywhere is
   `unfold_stuck`, with the pointer "no equation of f matches; cases
   on its argument first". `(reduce)`, `(reduce lhs)`, `(reduce rhs)`
   — the whole target or one side of an equation replaced by its
   normal form: K's whnf at every position (beta, iota, projections,
   `Nat` literals through K's accelerators), every `fn` by its
   equations as `unfold` applies them (never by delta: a `fn`'s value
   is the recursor form, the equations are its meaning), every `def`
   and `abbrev` by delta, to a fixpoint under a budget
   (`reduce_budget`). The normal form is `show`n; a propositional
   equation on the way makes the step a rewrite of rule 5 and the
   `show` follows. This is one node for v2's `reduce`, `simp` and
   `compute`, whose distinction was the old evaluator's.
7. **`rw`.** `(rw LEMMA)`, `(rw (<- LEMMA))`: LEMMA a term — a name,
   or a name applied to arguments — elaborated without an expected
   type; its type after whnf opened as `apply` opens, to an equation
   `a = b` (or `b = a` under `<-`; `rw_not_eq` otherwise). `a`, with
   its metavariables, is matched first-order against the target's
   subterms in **pre-order, left to right, the function before its
   arguments**, the first match instantiating the metavariables, and
   every occurrence of the instantiated `a` is rewritten (Lean's
   `rw`), by rule 5. `(rw LEMMA (occ K))` rewrites the K-th matching
   occurrence only. The lemma's premises left unassigned are child
   goals, positional blocks as `apply`'s: `(rw LEMMA (occ K)? STEP-BLOCK…)`.
   `rw_no_match` shows the pattern and the target. A rewrite whose
   result is `a = a` is not closed by `rw` (Lean's `rw` tries `rfl`;
   here the author writes it — one node, one meaning).
8. **Universes and levels.** A goal's statement is a `Prop`; a
   motive over a Pi into `Prop`; `congrArg`'s levels come from the
   types of `a` and the target by unification, as any constant's do
   (slice 3.15).
9. **`sorry` in a block** closes its goal and makes the theorem
   pending (`PENDING NAME sorry`), as a top-level `sorry` does:
   nothing is declared, every citation is a pending obligation. The
   block is elaborated whole, so every other step is checked, and the
   record shows each sorried goal as `goal_of` renders it.
10. **The E functions** (`kernel/tactic.shard`, exported through the
    loader's module): `goal_of : Goal → the context (name, type)
    list and the target`, rendered in S by the dumper; `applicable :
    Goal → the node heads whose precondition holds` (a Pi target
    admits `intro`, an equation admits `rfl` and `rw`, an inductive
    local admits `cases`…; not a completeness claim); `step : Goal →
    Node → StepRes`; `elaborate : Goal → I → P or refusal`. All are
    `fn`s, so an engine in E can compose them (3.22). `Node` is I's
    data type: one constructor per form of rule 4 with its arguments
    as data (names, S terms, child blocks), read from the S form by
    `read_node` — the concrete syntax of rule 1 and nothing else.
11. **Records.** `PROOF NAME nodes=N` on an accepted theorem proven by
    a block (the ACCEPT record stands); `PENDING NAME sorry goals=K`
    with the goals under it. The DEFINE/REALIZE records are unchanged.
12. **Not in 3.20**, each named: `simp_only`, `arith` (3.21);
    `injection`, `subst`, `cases` on an indexed family or on `Eq`;
    `exists`/`constructor`/`left`/`right` (an `exact` with the
    constructor serves); `intro` through a `let`; `wf` over a measure
    into `Int` or over a lexicographic pair; a witness goal (data
    hole); `rw` under binders where the motive would capture
    (`rw_motive`: Lean's "motive is not type correct"); `by` inside a
    term (`(by …)` as an L term); the Rust bootstrap reads `by` as
    `UnknownForm`, so a theorem under `v3/kernel/**` stays on
    `exact` until route 1's chain is V3's own.

**Slice 3.20 as built** (2026-10-06):

- **Rules 1 and 10.** `kernel/tactic.shard` is the opener: `Node` is
  I's data (one constructor per form, the arms and the premise blocks
  nodes of their own kinds), `read_block` reads `(by STEP…)`,
  `tac_step` runs one node on one goal, `tac_block` runs a block's
  nodes depth-first — the children of a branching node from the
  blocks the step pairs with them, in the goals' order — and
  `tac_prove` instantiates the root; `reader.shard`'s `read_proof`
  hands a `by` to it, for a `theorem` and for a `fulfills` alike.
  `goal_of` renders a goal (`(x : Nat) (h : (= x 1)) ⊢ (= x 1)` —
  bound names from the binders, `Eq` and the comparisons under their
  operator spellings); `applicable` lists the forms whose
  precondition the goal meets. A refusal names the step's index in
  its block and shows the goal it faced. **A premise's one-step block
  may be written as the step:** `(apply Eq.symm (exact h))` — a list
  headed by a form's name is one step, so the common case needs no
  second parenthesis. The arms of `cases` and `induction` come in one
  list, `(cases x ((zero STEP…) (succ (n) STEP…)))`, and may name the
  equation: `(cases (is_digit c) hd (ARM…))` gives each arm `hd :
  (= (is_digit c) false)` and so on — the motive is `λ d. x = d →
  target[d]` and the eliminator's result is applied to `Eq.refl x`
  (Lean's `cases h : x`); calc's lexer lemmas need it where the split
  term reappears under the unfolding of a function. The record of
  rule 11 was not added: `ACCEPT` stands for a theorem proven by a
  block as for one proven by a term, and the pending record carries
  the count (`PENDING NAME sorry goals=2`).
- **Rule 2.** As designed. The goal's metavariable has the closed
  statement as its type and no telescope; a step opens the first
  `nctx` binders as K locals under the binders' names, acts on the
  target and closes the filling over them, so every assignment is a
  closed term and R68's delayed filling never arises. A split or an
  induction on a local drops it from the children's context when no
  other local's type mentions it.
- **Rules 3 and 4.** As designed, with these shapes: `(wf X MEASURE IH
  STEP…)` — X the local first, the measure a `Nat` term over it, the
  hypothesis `IH : ∀ y, (< MEASURE[y] MEASURE[X']) → target[y]` stated
  with `Nat.lt` so it reads as the arithmetic a descent fact proves;
  `WellFounded.fix.{u,0}` at `InvImage.wf MEASURE Nat.lt_wfRel.wf`
  as `define.shard` builds it for a function. `apply` opens exactly
  arity(TERM) − arity(target) binders and refuses a premise whose
  statement mentions an argument the target leaves open
  (`apply_unsolved`, with the pointer: give it through the first
  premise — `(apply (Eq.trans h1) (exact h2))`); `decide` builds
  `of_decide_eq_true` at the decision the elaborator's lookup or the
  derivation table gives and checks `Decidable.decide` against
  `true` by K's conversion before K sees the theorem.
- **Rule 5.** As designed. `rw` matches the lemma's side
  **syntactically** (`tac_match`: structural descent, a metavariable
  assigned through unify.shard's typed `assign`, levels structurally)
  — never by K's conversion, which would make `app nil ?ys` an
  instance of `app (cons x r) nil` by computing both sides, a match
  and a useless one. An application is matched with its whole spine;
  a prefix of a spine is no subterm. A rewrite at a hypothesis (`(rw
  LEMMA (at H))`) was not built: calc's one need (`skipws_head_nonws`)
  is met by `show` of the convertible target and a reversed rewrite.
- **Rule 6, and the equations of a measured function.** `reduce`
  finds the first redex of the target in pre-order: an application of
  a constant with equations by the first equation that matches; any
  other application, a projection or a redex by K's whnf where it
  progresses — the result differs, its head is neither a stuck
  eliminator (a recursor, a case analysis, `brecOn` and its helpers,
  a matcher, a fixpoint, a projection of one) nor a lambda (a
  function constant's value is no reduction of it) — and never an
  inductive's or a constructor's application, a theorem's, or a
  proposition's. K's whnf reduces a head as far as it goes, past the
  one step wanted — an `ite` on a decided condition to its branch,
  then the branch's function unfolded to its recursor form — so every
  application of a constant with equations inside the subterm is
  masked by a local during the whnf and restored after. **Two kinds
  of equation:** a computation rule has a constructor or a literal in
  its left side (`app (cons x r) ys = …`) and `reduce` applies it
  anywhere; an unfolding (`count_down n = ite …`, a measured
  function's one leaf) `reduce` applies only to a closed occurrence —
  a ground computation ends, the budget bounding it — and `unfold`
  applies once to each occurrence present when the step starts, never
  to one a rewrite introduces. A definitional equation (by `Eq.refl`)
  is applied by conversion and contributes no term; a propositional
  one is a rewrite of rule 5 with a continuation goal, the previous
  goal assigned. **A measured function now has its equations:**
  `define.shard` states each leaf's as before and proves it by
  `WellFounded.fix_eq` at the instance — the definition's value at the
  leaf's arguments is `WellFounded.fix α C r hwf F t`, and `fix_eq`
  there has the type `fix … t = F t (λ y _. fix … y)`, whose sides K
  converts to the leaf's (the left by delta, the right by beta, the
  tuple's projections and the matcher at a constructor, each self-call
  by delta); the descent obligations are admitted into the walk before
  the definition. `WellFounded.fix_eq` lies at line 107,407 of the
  export, past the pins' prefix, so the pins test and the files that
  need it stream the second fixture; `calc_spec.shard` imports through
  it, and `parse_tail` has `eq_1`.
- **Rule 7.** As designed: `(rw LEMMA)`, `(rw (<- LEMMA))`, `(occ K)`
  with the count in the refusal, the premises as positional blocks
  (`(rw if_pos (exact h))`; `if_pos`'s instance argument is assigned
  by the match). The traversal is pre-order with the function before
  its arguments over whole applications.
- **Rules 8 and 9.** As designed.
- **Rule 12.** As listed; `cases` on a term that is not a local
  generalizes its occurrences in the target (an `if`'s condition,
  after `reduce` has exposed it).
- **Not built, found on the way:** a witness goal (`apply Eq.trans`
  without the middle term); `rw` at a hypothesis; `induction …
  generalizing` (the statement quantifies what the hypothesis must
  vary over, and `intro` follows the split — `lex_num`); `show`'s
  display of a stuck `Decidable.rec` is long where a `Bool` decision
  is unfolded by hand; the elaborator's refusal of an unsolved
  universe inside a block carries no step index.
- **Found by the probes:** the recursor takes its major after the
  minors and `casesOn` before them; a name shared with another kernel
  file's (`form_head`, `open_fields`, `sort_level`, `StepRes`, `RwOk`,
  `mem_expr`, `RdOk`) shadows it under the bootstrap's flat resolution
  and fails far from the collision — every name in `tactic.shard` is
  checked against the closure.
- **Found by a second reader over the diff, each reproduced and
  fixed:** `(cases X H …)` on a local dropped `X` from the arm's
  context while `H` still named it (K: `fvar_in_value`, no step index)
  — the local stays when an equation is named; `(induction X H …)`
  would guard every hypothesis by the equation, so it is
  `induction_eq`; `unfold` and `reduce` of a `def` with no equations
  were refused where rule 6 promised delta — K's whnf takes `double
  x` past `x + x` to a stuck recursor, so one delta step is made by
  hand (`delta_once`) where whnf makes no progress; `(occ K)` counted
  prefixes of application spines the traversal never offers — an
  occurrence is a whole spine now; `reduce` on a closed occurrence
  whose condition no reduction decides (`count_down (g 5)` with `g`
  opaque) ran for minutes — an unfolding applies only to a **ground**
  occurrence (no local, every constant a definition, a constructor, a
  recursor, an inductive or a quotient), under a budget of 64
  unfoldings; a refusal rendered its goal twice and a term as a
  sequent. Two pin comments were wrong. *Noted, not changed:* `(occ
  K)` counts the occurrences of the **first** instance, as Lean's `rw`
  does; Init's arithmetic lemmas speak `HAdd.hAdd` where V3 writes
  `Nat.add`, so a rewrite by them needs a `show` first — syntactic
  matching by design; `(by)` with no step is `bad_proof`.
- **The consumer.** 44 of calc's 100 claims are theorems by blocks in
  five files (`tactic_test.sh`): the nine ground spec tests and the
  eleven reconciliation tests by `(reduce) rfl` (parse_tail, measured,
  by its `fix_eq` equation at closed arguments — 20 s for the nine
  under the bootstrap, against K's own conversion through `Acc.rec`
  before the equation existed); the lexer's structural lemmas
  (`lex_num` by induction with the accumulator quantified and the
  split's equation named, `lex_digit_head`, the three head lemmas by
  `if_pos`/`if_neg`, `skip_ws_idem`, `head_skipws_false`,
  `skipws_head_nonws`, `parse_tail_nil_test`); the digit type's
  (`is_digit_code` a ten-way split closed by `decide`); `codes_append`
  by induction. Of the 52 whose lemma closure needs no arithmetic the
  rest wait on `show`'s `Int` measure, Init's inequality lemmas or
  3.21; the 48 others need `arith`. The worked-examples file of the
  ruling's fifth lean is these five, one claim of each shape among
  them.

**Slice 3.21 — the two forms that carry an engine: `arith` and
`simp_only`** (under the ruling of 2026-10-05, item 53's first lean;
the design 2026-10-06; §13 item 54). Slice 3.20 left calc's claims
that need arithmetic (48 of 100) and every chain of rewrites written
one step at a time. Law §7.1 draws the line this slice works on:
whatever searches is a producer of I and what it emits is the result;
replay computes and matches, and K checks P.

*The rules.*

1. **`arith` is a Farkas certificate over stated rows.** `(arith
   FACT…)` closes a goal that follows from linear arithmetic over
   `Int` and `Nat`. Its **rows**, in order: the goal negated; each
   hypothesis of the goal's context whose statement is linear (rule
   2), in context order; each FACT — a term, a hypothesis' name or a
   lemma applied — whose type is linear (`arith_fact` otherwise); and
   the rows the atoms bring (rule 4). A certificate is one integer per
   row — non-negative on an inequality, any sign on an equation —
   under which the rows sum to a constant inequality that is false
   (`0 ≤ -1`). `(arith FACT… (farkas K…))` carries the certificate (a
   row past the list's end has multiplier 0): replay checks the sum
   and builds P, no elimination runs (`arith_certificate` shows the
   sum that is left). Without the clause the node names the
   **default reconstruction** (item 53's second lean): Fourier–Motzkin
   elimination over the rows in atom order under a budget of 512
   derived rows (`arith_budget`), the equations eliminated first by
   substitution; what it finds is the certificate, and the two forms
   elaborate through the same builder. `arith_failed` shows the rows
   in normal form. The elimination is exact over the rationals and
   integer-tight only in the strict comparisons (`a < b` is `a + 1 ≤
   b`): no cut, no case split — `2x = 1` is not refuted, a `≠`
   hypothesis is no row. Those are a producer's (`omega`, 3.22),
   which emits this node.
2. **Linear statements.** `a ≤ b`, `a < b`, `a = b` and their
   negations at `Int` or `Nat`, under V3's spellings (`Int.le`,
   `Nat.lt`, `Eq`) and Init's (`LE.le`, `LT.lt`, `GE.ge`, `GT.gt`,
   `Ne`, `Not`), where the sides are built from `+`, `-` (at `Int`),
   unary `-`, `*` with a literal on one side, literals, and **atoms**
   — any other term, two atoms the same when they are the same term
   after the spellings of rule 3 are made one. `False` as a goal has
   no negation row. A goal `a = b` is two certificates (`a ≤ b` and `b
   ≤ a`, `Int.le_antisymm`) unless the sides have one normal form,
   when it is their two evaluations composed and no certificate; a
   goal `¬ P` is P's row and the certificate.
3. **One spelling.** Init's lemmas speak `HAdd.hAdd Int Int Int _ a
   b`, `OfNat.ofNat Int 5 _`, `Nat.cast n`; V3 writes `Int.add a b`,
   `Int.ofNat 5`. `arith` reads both: a class method at `Int` or `Nat`
   under Init's instance is its function, throughout a row — inside
   an atom too — and K's conversion is the proof that the two are one
   (the instances unfold). So `(arith (Int.emod_nonneg n h))` and a
   row from the context meet on the same atoms.
4. **What an atom brings.** A `Nat` term enters the integers under
   `Int.ofNat` (`Int.natCast_add`, `Int.natCast_mul` push the cast to
   the atoms), and each `Nat` atom `a` brings the row `0 ≤ ↑a`. `x /
   k` at `Int` with `k` a positive literal is an atom that brings `k *
   (x / k) ≤ x` and `x < k * (x / k) + k` (`Int.mul_ediv_self_le`,
   `Int.lt_mul_ediv_self_add`), and `x % k` is `x - k * (x / k)`
   (`Int.emod_def`): the quotient and the remainder by a literal are
   linear arithmetic, as in `omega` and as v2's `div-facts` gave
   them. `Nat` subtraction, division at `Nat`, `min`, `max`, `natAbs`
   and `toNat` are atoms. These rows come after the facts, in the
   order the atoms first occur.
5. **P is `Lean.Omega`'s.** The atoms are a list `v`; each side `t`
   gets a linear combination `l` (a constant and a coefficient list,
   closed data) and a proof `t = l.eval v` built on the term's
   structure from `LinearCombo.coordinate_eval`, `add_eval`,
   `sub_eval`, `neg_eval`, `mul_eval_of_const_left/right` and the
   congruences of `Lean.Omega.Int`; a row `0 ≤ l.eval v` or `l.eval v
   = 0` is `Constraint.addInequality_sat` or `addEquality_sat`; the
   certificate is a fold of `combo_sat'`; the false constant is
   `normalize_sat` and `Constraint.not_sat'_of_isImpossible (Eq.refl
   true)`. K's conversion does the arithmetic on the closed data
   (each intermediate combination is stated as a literal, so one step
   is one bounded computation) and nothing else: no ring normalizer,
   no reflection of the terms. The kit lies in the export through
   line 91,131 but for the product (`mul_eval_of_const_left`, line
   271,373) and the quotient's bounds (line 274,616): a file whose
   rows have neither imports as before; one that has them imports
   through `Int.lt_mul_ediv_self_add` (`arith_unreached` names the
   constant and the line). The second fixture grows to that line.
6. **`simp_only` is a bounded rewriter over a lemma list.**
   `(simp_only RULE…)`, a RULE a LEMMA or `(<- LEMMA)` as `rw` takes
   them: until no rule applies, the first rule in list order with a
   match in the target is applied as `rw` applies it — the first
   match in the traversal, every occurrence of that instance — under
   a budget of 256 rewrites (`simp_budget`: a rule that rewrites its
   own result). A rule's premises are discharged by **assumption** —
   a hypothesis of the context with the premise's statement — and a
   rule with a premise no hypothesis states does not apply there
   (law §7.2's side-condition policy: stated, not searched). No rule
   applies at the start is `simp_no_progress`. Nothing is implicit:
   no lemma set, no arithmetic, no computation — `(reduce)` before or
   after is its own node. A function's name is no rule (`unfold`, or
   name its equation `f.eq_N`).
7. **`applicable`** lists `arith` where the target is linear and
   `simp_only` always; a premise's one-step block may be either.
8. **Not in 3.21**, each named: an `only` restriction of `arith`'s
   context rows (a budget refusal is the signal); cuts and case
   splits; `Nat` subtraction; non-literal products; `simp_only` at a
   hypothesis, with side selectors or with a discharger other than
   assumption; the producers (3.22).

**Slice 3.21 as built** (2026-10-06):

- **Rules 1, 2 and 5.** As designed. `kernel/arith.shard` is the
  arithmetic — the spelling (rule 3), the rows, the certificate's
  check, the elimination, and P from `Lean.Omega`'s lemmas —, and
  `tactic.shard` holds the node (`NArith`: the facts as S terms, the
  certificates) and the step, which elaborates the facts, hands the
  context's locals over (a local whose statement is no comparison is
  passed by) and has K type the term before it is the goal's filling
  (`arith_internal` would name the step; none has been seen). The
  kit's last lemma in the plain case is `Lean.Omega.Int.ofNat_lt_of_lt`
  (line 91,131 of the export), inside the pins' first fixture. An
  equation goal takes a `(farkas …)` clause per direction, the `≤`
  first (a direction without one is reconstructed); a weight of zero
  leaves its row out. A refusal shows the rows in normal form over the atoms: `[0 <= -1 +
  -1*x + 1*y] [0 <= 0 + -1*x + 1*y]`.
- **Rule 3, wider than designed: one spelling is the proof layer's.**
  The first `(rw (if_neg hn))` after `(cases (Int.decLt n 10) …)`
  found no match: the hypothesis the split brings is Init's `¬ (LT.lt
  Int Int.instLTInt n 10)` and the target's condition is `Int.lt n
  10`. So every **new goal's statement** enters in the source's
  spelling (`tac_goal`: the context and the target, K converting the
  occurrence), and so do the two sides of **a lemma's equation** in
  `rw`, `simp_only` and `reduce` (`eq_of_lemma`) and a premise
  `simp_only` looks up. The 3.20 note that a rewrite by one of Init's
  arithmetic lemmas "needs a `show` first" no longer holds.
- **Rule 4.** As designed, and `Nat.succ x` under the cast is `x + 1`
  (K converts the two; `Nat.lt a b` unfolds to it).
- **Rule 6.** As designed: `NSimp`, `simp_loop`; a match whose
  premise no hypothesis states is passed by and the next subterm is
  tried; a rule whose match leaves an argument unassigned, changes
  nothing or has a motive K does not type is passed by for the next
  rule.
- **Rule 7.** As designed.
- **A matcher under a measure carries its equation** (`define.shard`,
  `realize.shard`; not in the design — calc's `parse_tail` could not
  be made total without it). Slice 3.14 stated a self-call's descent
  obligation over every local in scope; in a row of `(match (skip_ws
  cs) …)` the locals are the pattern's variables and nothing ties them
  to the scrutinee, so `parse_tail.dec_1` read `∀ p c rest, … → ∀ n
  rest2, len rest2 < len p.2` — false, and `parse_tail` could never
  leave `PENDING measure` (v2 gave each site the premise `skip_ws cs =
  Cons c rest`). Now, in a measured function, a matcher's motive is `λ
  s' . s = s' → motive s'`, each row's last binder `h : s = pattern`,
  and the application is applied to `Eq.refl s` — Lean's `match h : s
  with`, by `addArg` on the matcher K checked (`pj_match_eq`) — so `h`
  is a local in scope and the obligation is stated under it — and as
  the comparison it is, `measure[arguments] < measure[parameters]` at
  `Nat`, where slice 3.14 stated the relation's instance `InvImage …
  (tuple) p`; the rows' variables carry the source's names and a
  function of one binder its binder's: `∀ xs x r, xs = cons x r → ∀ y
  s, r = cons y s → len3 s < len3 xs`. The
  function's equations are stated over the same shape (`PkEqn`: the
  body walked once more with self-calls left as calls, so K converts
  `F t (λ y _. fix y)` to the leaf's body as before); the leaf walker
  reads the extra argument, so a leading matcher on a binder still
  gives one equation per constructor. The program is untouched (it is
  the erasure of the body as written). `parse_tail`, `show` (now under
  `(measure (Int.natAbs n))`) and `show_nat` are `DISCHARGE … proved`:
  **nothing a claim of calc's cites is pending.** Slice 3.20's
  theorems over `parse_tail` had been accepted under those two
  parameters.
- **`reduce`, two repairs** the capstone's proofs needed. (1) A case
  analysis at a decided scrutinee takes one step: a matcher at `cons c
  rst` whose row is an `if` on an open condition was no progress (K's
  whnf goes on into the row and ends at a stuck `Decidable.rec`); the
  alternatives — an `ite`'s branches, a matcher's rows — are masked by
  locals while K picks one (`alt_step`). (2) A proposition is no
  computation: `reduce` unfolded `Nat.lt a b` to `Nat.le (Nat.succ a)
  b`; it leaves a comparison as it is now (`unfold` by name still
  unfolds one). Three of 3.20's proofs lost a last `(reduce lhs)` that
  had nothing left to do — `reduce_stuck` on an already normal target
  makes a proof depend on how far the step before went (*noted*).
- **A type and its constructor under one name** (`(type CalcState
  (CalcState …))`, §13 item 28) were `ambiguous_name` in a theorem's
  binder; where a type is expected — a binder's type, an argument at a
  sort — the name is the type's (`elab.shard`).
- **The pending record shows its goals** (slice 3.20's rule 9, which
  that slice left at the count): `PENDING main.t sorry goals=2 [(n :
  Nat) ⊢ (= n n)] [(n : Nat) ⊢ (= 1 1)]`. It is how a block is
  written: a `sorry`, a load, the goal. Goals print `Int` literals and
  `+ - *` as the source writes them.
- **The fixture.** `init_prefix_str_tail.ndjson` grows by 11,101
  lines, to line 274,616 of the export (`Int.lt_mul_ediv_self_add`).
- **The consumer: calc's 100 claims are theorems**, file for file
  (`kernel/test/tactic_test.sh`: five loads cover the fourteen files,
  110 checks), with the capstone `run_eq_spec : run cs = spec_run cs`
  under no parameter. Re-spelled in V3's idiom: a hypothesis is a
  proposition (`(<= c 32)`, `(Not (= c 43))`); an induction's
  statement quantifies what its hypothesis must vary over and `intro`
  follows the split; an `if` on a comparison is split on its decision
  (`(cases (Int.decEq c 43) ((isFalse (h) …) (isTrue (h) …)))`); a
  hypothesis about a split term is an implication introduced after
  the split. `loop_eq` is restructured around `parse_num`'s result
  (`arm_plus`, `arm_minus` take the lexer through one operator and one
  number), so its hypothesis is cited under `ptd`, the fact that
  discharges `parse_tail`'s own descent. Twelve lemmas the old proofs
  inlined or took from `std` are stated: `append_nil_right`,
  `append_assoc` (`list.shard`), `natAbs_of_nonneg`, `natAbs_lt`
  (`natabs.shard`: Init's are at line 672,885, past the fixtures),
  `is_ws_true`, `is_ws_false`, three `pr_*_none`, `nonnum_none`, the
  two arms. 45 steps are `arith`, none with a certificate written,
  and 18 are `simp_only`.
- **Found on the way, not changed.** Init's names are visible as far
  as *any* loaded module imported: `calc_proof.shard`'s
  `decide_eq_true` became `ambiguous_name` (with
  `Bool.decide_eq_true`) once another file in the load imported
  further; it cites `Init.decide_eq_true` now. A measured function of
  several binders has its obligations stated over the arguments'
  tuple `p` (`len rest2 < len (proj PProd 1 p)`; `PProd.snd` is at
  line 2,429,884). There is no `by_cases`: the
  split on a decision instance serves.
- **Found by a second reader over the diff, each reproduced and
  fixed, the first three pinned.** `simp_only` with a rule whose right
  side holds its left (`r = mk (fst r) (snd r)` given without its
  argument) put the term back twice at every step: the target doubled,
  no budget of rewrites was reached and the load did not end — it is
  refused at once (`simp_self`). `Ne a b` was rewritten to `Not
  (Eq.{1} …)` whatever Ne's universe, and `intro` then built a term K
  refused (a statement over `Ne Nat Int`; `by_ne_level`). A descent
  obligation printed every pattern variable as `x` and its fact as the
  relation's instance, where `reduce` stepped into the relation and
  `arith` read no comparison (the bullet above; `define_match_eq`). A
  generated binder's name printed as a bare number (`(fun (20 : Nat)
  …)`: `x20` now). A class method at a local instance was rewritten
  as Init's (it stays as written). `v3/test.sh`'s fallback for a
  missing result printed the shell's error. *Noted, not changed:* a
  type and its constructor of one name are still ambiguous where
  nothing is expected (the first operand of `=`); a compound literal
  as a first operand elaborates at `Nat` (`(<= (- 0 5) x)` — the
  elaborator's, older than the slice).
- **Not built**, as rule 8 lists; and no pin for 3.20's fifth finding
  (a ground occurrence under an opaque constant): a pin has no way to
  declare one.

**Slice 3.21b — the trajectory review's findings 1–3** (GPT-6's
review of 2026-10-06 over the tree at `7e99261`, records §4.11; the
user's ruling 2026-10-06 on the leans; §13 item 55). Three of the
review's six findings are the three items slice 3.21's as-built left
open, with what the review added: a specification and its
implementation inherit one unintended numeric reading while both
proofs check; a module's meaning depends on the load's order; a proof
is tied to how far the step before it went.

*The rules.*

1. **The expected type decides an arithmetic operator.** Where `+ - *
   / mod %` is read at an expected type whose normal form is `Nat` or
   `Int`, the operator's identity is that type's and both operands are
   read at it — a `Nat` local under `Int` is cast at the leaf
   (`Int.add (Int.ofNat n) 1`), as Lean's `binop%` reads `(n + 1 :
   Int)`. Where the expected type decides nothing — none, a
   metavariable, not numeric, a comparison's operands — the identity
   is the first operand's that is not **numeric-closed** (a numeral, or
   an arithmetic operator over numeric-closed operands), the other
   operand read at it; all numeric-closed is law §5.2's default,
   `Nat`. A boundary the author writes holds: `(Int.ofNat (- 1 2))` is
   `Nat`'s subtraction, 0. Before, the operation was chosen from the
   first operand that was not a numeral and the result coerced:
   `(- 1 2)` at `Int` was `Int.ofNat (Nat.sub 1 2)`, 0, and `(<= (- 0
   5) x)` read at `Nat` against x's `Int`. The rule is an elaborator
   fix under law §5.2's order (constraints before defaulting), not a
   design change.
2. **Init's visibility is the module's horizon.** Each imported
   declaration carries its ordinal at admission (the loader's
   declaration → ordinal table, keyed as the environment is). A
   module's horizon is the count admitted at the farthest of its own
   `(import Init NAME)` and its imports' horizons; a module that imports
   no Init has none. Resolution sees an imported declaration below the
   horizon only — whatever else the load admitted, in whatever order —
   and a constant with no ordinal (K's own, admitted with the
   environment) wherever Init is. The checked environment stays one and
   shared; what changes is what a scope sees of it. The slice-3.21
   workaround (`Init.decide_eq_true` in calc_proof) stays as written.
3. **`reduce` is a normalization: it succeeds unchanged where nothing
   reduces.** The refusal `reduce_stuck` is retired; `reduce_budget`
   stays. No step requires progress today, so none is given a
   progress-requiring variant (the review's "separate operation" waits
   for a procedure that needs the contract).

**Slice 3.21b as built** (2026-10-06):

- **Rule 1.** `elab.shard`: `op_expected` (the identity from the
  expected type, for an arithmetic operator), `is_arith_op`,
  `is_num_closed`/`all_num_closed` (in `is_numeral`'s place in the
  operand rule). The pin `op_expected`: `(- 1 2)` at `Int` is `Int.sub
  1 2` and equals `-1`; `(+ n 1)` at `Int` with `n : Nat` is `Int.add
  (Int.ofNat n) 1`; `(<= (- 0 5) x)` is `Int.le (Int.sub 0 5) x`;
  `(Int.ofNat (- 1 2))` is `Int.ofNat (Nat.sub 1 2)` and equals 0;
  `(- n 1)` at `Nat` is `Nat.sub n 1`. One file of the closure changed
  meaning under the rule, and the full suite found it: the pin
  `one_meaning` (slice 3.16's G1) stated `(+ a (- n 1))` at `Int` with
  `n : Nat` as a truncating `Nat` difference cast after (2 at `n = 0`);
  it is `a + (↑n - 1)` now (1), as Lean reads it, and the pin says so.
  Parity byte-identical, calc's 100 claims and 21 inputs as before.
- **Rule 2.** `scope.shard`: `VisOnly`'s second field is the horizon
  (an `Int`; `-1` none) and a sixth field the ordinal table;
  `init_within`, `ords_find`, `ords_add`. `loader.shard`: `Load.inits`
  filled at each accepted record of the stream (`ld_add_inits`, the
  record's constants at the record's ordinal), `init_horizon` (the
  ordinal past the import's target, or the count admitted where the
  target has none), `Fx.init` and `Mod.init` the horizon, `fx_see`
  taking the farther. A tail-fixture load measured at 16.1 s with the
  table against 16.4 s without: no cost. The pins `init_horizon` (two
  roots, the wider loaded first — the review's failing order) and
  `init_horizon_beyond` (a declaration past the module's horizon,
  admitted by another module, is `unknown_constant`).
- **Rule 3.** `tactic.shard`'s `reduce_loop` ends at a target with no
  redex; its `made` count is gone. The pin `by_reduce` expects `ok`
  now (reduce at a variable leaves the target; the proof closes by
  `rfl`).
- **Not built**: a per-module Init prefix held as the review's
  "indexed declaration dependencies"; a progress-requiring `reduce`.

**Slice 3.22 — the producers: `auto`, the sidecar, the engine and the
store** (the user's ruling of 2026-10-07 on three leans; §13 item 56;
law §7.3, §7.5, §6). The third of the slices ruled on 2026-10-05. Law
§7.5 fixes the shape: sources carry tactic blocks or an `auto`
delegation; the build elaborates I to P and K checks P; engine-written
I lives in the sidecar and the pin store, keyed by the resolved
requirement; no tool writes into a source file; a stale pin is a
pending obligation, never re-searched at verification (T8). Two items
carried from GPT-6's trajectory review (item 55): a stored certificate
keyed by its rows' statements, and the dependent witness goal.

*The ruling.* (1) **The engine's output is `(arith only FACT… (farkas
K…))`, the certificate written**: the rows are the goal and the named
facts in list order, nothing from the context, so an unrelated `have`
added later shifts no weight. Rejected: emitting `(arith)` and
reconstructing at every replay — cheaper to write, but it moves a
consequential choice into the reconstruction (law §7.2 carries such
choices). (2) **The store is a CI artifact, not committed**: the build
writes it when asked, CI verifies the bundle, the repository holds
sidecars only. Rejected: committing P (megabytes that change at every
kernel edit). (3) **The engine's reach is measured over calc's 100
claim statements**, the hand proofs staying as the worked examples;
this replaces the slice table's B-1c consumer, the old tree's
generator.

*The rules.*

1. **`auto` is a delegation; the build never searches.** A `theorem`'s
   proof may be `auto` or `(auto HINT…)`. The loader reads the sidecar
   `FILE.auto.shard` beside the source — `(proof-for NAME FINGERPRINT
   (by STEP…))` forms, the file machine-owned and rewritten whole by
   `prove` — and replays the entry's block as it would a hand-written
   one, under the same reader and the same checks. No entry is
   `PENDING NAME auto_missing`; an entry whose fingerprint is not the
   theorem's is `PENDING NAME auto_stale`; a block that fails is the
   refusal it would be inline. Nothing in the loader searches, and no
   tool writes into a source file (law §7.5).
2. **The key is the resolved requirement.** The entry's FINGERPRINT is
   the theorem's content fingerprint — the elaborated statement closed
   over its binders, as the ACCEPT record carries it — so a changed
   numeric reading, coercion or parameter is a different requirement
   and a changed proof strategy is not.
3. **`(arith only FACT… (farkas K…))`.** The rows are the goal negated,
   then each FACT in list order, then the rows the atoms bring (slice
   3.21 rule 4); the context's hypotheses enter only as facts named. A
   certificate indexes those rows. Everything else is slice 3.21's
   `arith`.
4. **The witness goal.** In `(apply TERM (STEP…)…)`, a premise whose
   statement holds an unassigned metavariable is a witness goal: the
   premises' blocks run in the order given, `exact` and `rfl` assign
   through unification, and a metavariable still open when the step's
   blocks have run is `witness_open`. `apply Eq.trans` without its
   middle term has it assigned by the first premise's `exact`.
5. **The engine is an E library over the fixed API.**
   `kernel/engine.shard` composes `goal_of`, `applicable` and `step`
   (law §7.3) into a ladder, cheapest first: `intro`; `rfl`; `(reduce)
   rfl`; `decide`; `(arith only …)` over the context's comparisons;
   `(simp_only EQN…)` over the equations of the functions in the goal,
   then `rfl` or `arith`; `induction` on each inductive parameter in
   turn, the arms closed by the same ladder with the hypothesis among
   the rules. Hints accelerate and add no language: `(induct X)`,
   `(lemmas NAME…)`, `(budget N)`. A failed attempt leaves nothing: the
   elaborator's state is a value, so the transactional invariant of
   law §6 holds without an `attempt`/`commit` API, which waits for a
   concurrent caller.
6. **`prove` is a driver.** `kernel/prove.shard` loads the module as
   `load.shard` does, runs the engine on each `auto` theorem without a
   valid entry, rewrites the sidecar whole (entries it did not solve
   kept, unsolved theorems left pending), and prints a record per
   theorem: `PROVED NAME` with the block, or `UNSOLVED NAME` with the
   last goal. The entry carries the engine's version; a version other
   than the current is reported, not refused (law §7.2).
7. **The store is K's export format.** `load.shard --store DIR` writes
   each accepted declaration as records in the format K ingests
   (`kernel/k/import.shard`), one file per declaration named by its
   content fingerprint, deduplicated; `verify_release DIR` is K
   replaying the store's records after the pinned Init export with no
   elaborator loaded — the T0 machinery over a second stream.

*Landings.* 1: rules 1–4 (`only`, the witness goal, `auto` with
sidecar replay, the pending reasons); gate: the pins and the
hand-written calc claims unchanged. 2: rules 5–6 (the engine, `prove`,
`v3/examples/auto/` with its machine-owned sidecar, the count over
calc). 3: rule 7 (the store and `verify_release` on calc, in CI).

**Slice 3.22 as built, landing 1** (2026-10-07):

- **Rule 1.** `reader.shard`: a proof is `(exact TERM)`, `(by STEP…)`,
  `auto`, `(auto HINT…)` or `sorry` (`read_auto`); `read_decl_autos`
  takes the file's entries, `read_fulfills` and `read_discharge` too —
  a `fulfills` and a descent's discharge may be `auto` (the entry for
  a discharge names the obligation, `f.dec_1`). `loader.shard` reads
  `FILE.auto.shard` once per file whose forms hold an `auto` proof
  (`read_sidecar`; `Fx.autos`); a sidecar that does not read or holds
  a form of another shape is the file's `sidecar_malformed`, the
  theorem pending. The pending reasons carry the fingerprint an entry
  needs: `auto_missing fp=N`, `auto_stale fp=N`.
- **Rule 2.** The fingerprint is `hash_expr` of the closed statement —
  what `identity_hash` gives a theorem, so the ACCEPT record and the
  sidecar agree.
- **Rule 3.** `NArith` carries `only`; under it `step_arith` hands no
  context row to `ar_prove`. The pin `arith_only` is the review's
  probe D with the certificate kept across the unrelated `have`.
- **Rule 4.** `premise_goals` closes each unassigned datum
  metavariable over the context as a goal is closed (`witness_close`:
  the closed metavariable of kind `witness`, the original assigned its
  application to the context), then makes the proposition premises
  goals — their statements may hold the witness's application now;
  the implicit arguments the elaborator inserted (`Eq.trans`'s `{b}`)
  are not among the opened binders and are closed from the applied
  term; `tac_open` zonks a goal's statement, so a block after the
  assigning one reads the witness's value; `tac_block` refuses
  `witness_open` after a step's child blocks have run. A pattern
  (`?B x1 … xn` against a term over the opened locals) is what the
  unifier assigns, and the assignment closes. An instance argument
  left open is `instance_needed`, as before.
- **Found on the way:** `(apply Eq.trans sorry sorry)` reads `sorry`
  where a premise's block must be a list: `(sorry)`.

**Slice 3.22 as built, landing 2** (2026-10-07):

- **Rule 5.** `kernel/engine.shard`: `engine_prove` takes the closed
  statement and the hints and answers a block or a refusal's text;
  `eng_solve` is the ladder — `eng_intro` over every leading binder
  (named after the binder, `hK` where it has none), `eng_close` (`rfl`;
  `(reduce)` then `rfl`; `decide`; `eng_arith`: `(arith only FACT…)`
  with the context's comparisons as the facts and the certificate
  `ar_prove` found written — `ARes` carries the certificates now),
  `eng_simp_close` (`(simp_only EQN… HYP… LEMMA…)` then `rfl` or
  `arith`), `eng_induct` on the hinted local then on each local of an
  inductive type the target mentions, the arms named after the
  constructors' fields with `ih_FIELD` for a recursive one, each arm
  by the ladder one level down (three levels). Two decisions beyond
  the design, each from a case: *the induction is tried before the
  introductions as well* — `len (rev_onto xs acc) = len xs + len acc`
  quantifies the accumulator, and introducing it first left a
  hypothesis too weak, so the binders are kept in the target and the
  hypothesis quantifies them (Lean's `generalizing`); *the rule set
  excludes an unfolding equation whose right side mentions the
  function* — `show n = … show (n / 10) …`, every argument a variable,
  matched its own result without end and calc_show's measurement did
  not finish (an equation with a constructor pattern, `len (cons x r)
  = 1 + len r`, stays: it stops at a variable). A quantified
  hypothesis is a rule too. Every node is confirmed by `tac_step` as
  it is chosen; `node_sx` renders the block.
- **Rule 6.** `kernel/prove.shard`: the loader's mode (`Load.mode`:
  `off`, `solve`, `measure`; `read_decl_autos` and the reader's proof
  path take it). Under `solve` an `auto` without a valid entry is the
  engine's and its block is replayed through `tac_prove` before the
  theorem is accepted (`RDSolved` → the record `PROVED NAME fp=N (by
  …)`); unsolved is `PENDING NAME auto_unsolved fp=N GOAL`. The
  sidecars are rewritten whole (`write_sidecars`: the file's existing
  entries not superseded, then the new ones in the file's order, under
  a header with the engine's version), the exit code refused + errors
  + unsolved. Under `measure` every hand-written proof is read as the
  build reads it and the engine is also run on the statement (`ENGINE
  NAME solved|unsolved`, `ENGINE: solved S of N`).
- **The example.** `v3/examples/auto/auto.shard`: twelve theorems,
  every proof `auto`, one with the hint `(induct xs)`; eleven closed
  (`rfl`, `decide`, `arith only` with the certificate, three
  inductions with `simp_only` over the equations and the hypothesis,
  the generalized accumulator), `beyond` (`x ≤ x * x`) pending
  `auto_missing` at build with the fingerprint an entry needs. The
  sidecar `auto.auto.shard` is committed; `kernel/test/engine_test.sh`
  replays it under `load.shard` (no `PROVED` record: the build does
  not search), regenerates it into a scratch copy and requires the
  blocks byte-identical (the fingerprints masked: a statement's
  fingerprint carries its module's name, and the copy is another
  module — found by the test).
- **The count over calc** (the third ruling): the engine closes
  **54 of calc's 100 claims** unaided (`engine_calc.txt` lists them;
  `engine_test.sh` requires that exact set, so a change in the
  engine's reach is a report, not a drift). What it does not close:
  the lemmas over `show`'s well-founded recursion, the case analyses
  that need an equation name, the chains of rewrites with lemmas the
  statement does not mention, the descent obligations. The claim list
  `calc_claims.txt` is shared with `tactic_test.sh`.
- **Not built:** `(lemmas …)` beyond the facts and rules it adds
  (no instantiation); a `cases` form in the ladder; an engine-written
  `fulfills` in the example.

**Slice 3.22 as built, landing 3** (2026-10-08):

- **Rule 7, the store.** `kernel/store.shard` renders an admitted
  constant of K's environment as the records K ingests — lean4export's
  NDJSON, format 3.1.0, field for field what `k/import.shard` reads:
  the name, level and expression tables the declaration cites,
  numbered from 1 in first-use order (0 is `anonymous` and `zero`, as
  the exporter's), then the declaration record. Table nodes are shared
  by *strict* structural identity (binder names and infos compared,
  which `expr_eq` ignores), so a record carries the term as elaborated
  and two runs over one elaboration write one text. An inductive type
  is written as its block — every type, constructor and recursor K
  generated for it (`T.rec`, then `MAIN.rec_1…` for the nested
  auxiliaries), with the fields the import validates against K's own
  regeneration, T0's validation a second time. `load.shard --store
  DIR`: after a clean load, each ACCEPT's constant and each PARAM K
  holds (a view's parameter, the axiom K admitted for it) is written
  in admission order — `DIR/ID.ndjson` per declaration, `DIR/order`
  listing `ID NAME`. ID is the content fingerprint (`identity_hash`,
  a report's fingerprint, never an authority), or `FP-K` for the
  first free K when a file of that name holds another text; a file
  already holding the text is shared and listed once; `order` is
  appended, never rewritten, so several loads fill one store in
  sequence and a shared module's declarations come from the first
  load that admitted them. A failed load writes nothing; nothing in
  the loader changed, the writer reads the records and the
  environment after the load.
- **Rule 7, `verify_release`.** `kernel/verify_release.shard [-v] DIR
  CHUNK…` imports K, the host and the JSON reader and nothing else:
  the Init chunks replay as one stream (the T0 machinery), then each
  store file in `order`'s order as *its own stream over the
  environment so far* — fresh tables, as the file was written, so a
  record a file lacks is MALFORMED there and never read from an
  earlier file's table (found by the test: with the tables persisting
  across files a dropped expression record resolved to the previous
  file's node). Each accepted declaration is held to its order line:
  the name and the content fingerprint must be the line's, else
  MISTIED — a dropped name record lands a theorem under a shorter
  name, the import reading a missing table index as anonymous. The
  gate's guarantee is K's acceptance; the tie says the bundle is the
  build's. `RELEASE: declarations N  accepted A  rejected … malformed
  X  mistied Y  missing Z`; the exit code is the declarations not
  accepted + malformed + mistied + missing + the chunks' failures.
- **The gate.** `kernel/test/store_test.sh` (the suite's 36th
  entrypoint): the example's 23 declarations stored once, a second
  load sharing all 23, K alone accepting them after the Init fixture;
  a theorem whose value is replaced by its statement REJECTED, an
  expression record removed MALFORMED, a file removed MISSING, a name
  record removed MISTIED, each with a non-zero exit; then calc's five
  leaves stored in sequence into `v3/release/calc` and replayed —
  **376 declarations, every one accepted by K alone**, 3.5 MB. The
  store is the v3 job's artifact (`.gitlab-ci.yml`), ignored by git,
  never committed (the ruling's second lean).
- **Found on the way:** three matchers of the example share a
  fingerprint (`len.match_1`, `app.match_1`, `rev_onto.match_1` are
  one term under three names — the hash covers the type and the value,
  not the name), so the collision suffix is exercised on the first
  store written.
- **Not built:** the implementation forks of views (§6.6) are checked
  and recorded under IMPL but not stored — their constants live in
  the fork, not the load's environment, and a linear stream cannot
  redeclare a parameter's name as its implementation; a store over
  forks is a branch per view, open until a consumer needs it. The
  store is per root; a release across roots is not a case yet.

**Slice 3.23 — the Init cache: K's verdict on the pinned export,
admitted under a receipt** (ruled 2026-10-08 on GPT-6's R75; §13 item
57; records §4.11). The boundary after 3.22 held two items accepted
from the trajectory review with their work deferred: the feedback
cost (R75) and the connected path (R76). The user ruled the cache
first, shallow — "something I expect us to iterate as we optimize and
rework things" — and the path after it.

*Retired at slice 3.26 landing 3 (2026-10-09, the user's ruling).* The
cache answered the cost of streaming the export's prefix through K,
which slice 3.26 removed: a load reads and checks the closure it
cites, seconds. Measured there with a hand-written receipt over the
export, admission saved a tenth to a fifth of a demanded load (calc 9 s
against 10 s, `std/bytes` 20 s against 25 s): the walk and K's
ingestion are the cost, and a receipt skips neither. Its only writer
would have been the full replay, which nobody runs locally and which
CI runs after the suite. Rejected because: a cache whose measured
value is seconds a load for a developer holding a 41-minute replay,
kept at the price of an admit-unchecked switch in K's run with no
consumer. What stays of the slice: `admit_decl_pinning` and the run's
admit mode, repurposed at 3.26 landing 2 for the loader's second
admission of records it checked a moment ago (`od_admit`). The rules
below are the slice as it was built.

*What the cost is.* Every load streams the pinned export through K up
to the deepest name its modules cite (§3.1), and K checks every
declaration on the way. Measured on the development machine over the
two fixture chunks (3,523 declarations, 14 MB):

| step | wall |
|---|---|
| reading and JSON-parsing every record | 9.2 s |
| and building the name, level and expression tables | 13.1 s |
| and K's check of every declaration (the T0 driver) | 47.2 s |
| the same with the typing judgments skipped (admit mode, below) | 17.2 s |

The check is 72 percent of a load's Init cost; the parse and the
tables are the rest, and any on-disk form must build the same tables.
A serialized environment therefore had a ceiling of a few seconds per
load against a second format behind K's seal; the lever taken skips
the check and keeps the export as the only form.

**The rules of slice 3.23:**

1. **The receipt.** `t0.shard --receipt FILE`, after a run that
   accepted every record of its chunks (nothing rejected, exhausted,
   mismatched or unsupported), writes one line per chunk: `PATH
   BYTES`. A run with anything not accepted writes nothing. A line
   starting with `#` is the writer's and is not read by the loader.
2. **Admission.** `k/add.shard`'s `admit_decl_pinning` is the
   environment effect of `check_decl_pinning` with the typing
   judgments of an axiom, definition, theorem or opaque skipped: the
   constant is inserted as the declaration states it, the node
   watermark and the accelerator pins exactly as on the checked path.
   An inductive block and the quotient take the checked path. The
   import's run state carries the mode (`run_admitting`); nothing else
   in K changes, and `check` — the raw entry of §3.5 — is untouched.
3. **The loader.** `load.shard --init-receipt FILE` (and
   `prove.shard`'s, the loader pins test's) sets the receipt on the
   load; a chunk listed with its byte count streams in admit mode, a
   chunk not listed or listed with another byte count is refused
   (`init_receipt_stale`) before any record of it is read. Without a
   receipt every chunk is checked, as before. The records and the
   counts print exactly as without the flag — the loader pins' 212
   expected outputs are the equivalence gate.
4. **Currency.** `kernel/test/init_receipt.sh` keeps
   `v3/.cache/init.receipt` current and prints its path: K's identity
   for the receipt is a hash of the sealed module's sources, the
   kernel files it imports and the fixture, written as the receipt's
   `# k=` header; a mismatch regenerates (one T0 run over the
   fixture). `v3/test.sh` runs it once and exports `INIT_RECEIPT`;
   the shell tests add `--init-receipt` when it is set. The cache
   directory is ignored by git; the receipt is never committed and
   never an artifact.
5. **The gate is untouched.** `verify_release` takes no receipt: the
   release bundle is replayed by K alone over the export, as rule 7 of
   slice 3.22 states. The T0 full replay on CI checks the whole export
   as before; the receipt is a cache of that verdict for the
   development loop and the suite.

**Slice 3.23 as built** (2026-10-08): as the rules. The auto example
(`--root v3`, Init streamed 1,509 declarations into the second chunk)
loads in 8.3 s under the receipt against 13.7 s without, the records
byte-identical; a receipt with a wrong byte count refuses the load
(`LOAD-ERROR … init_receipt_stale`, modules 0, exit 1); an unreadable
receipt exits 2 before any load. The 221 loader pins pass byte-identical under the receipt in 809 s on the development machine (beside parity and the define test; 1,361 s without the receipt beside the full suite — the clean comparison is pipeline 553's against 552's 3,327 s on the runner); parity byte-identical over 30 closures and 130,445 declarations; route 1's native driver, rebuilt locally, ties the interpreter on the first chunk and writes the receipt; the full suite's 36 entrypoints 0 failed in 927 s, the pins entrypoint its wall clock still.

**Slice 3.24 — the connected path: law §12.4's path assembled, the
branch-local proof joint built, each joint broken** (ruled 2026-10-09 on
GPT-6's R76; §13 item 58; law §12.4, §12.5 T1, T5). The second boundary
item after 3.22. Law §12.4 names the first connected path — *one
imported logical declaration → one checked E realization → one caller
using a branch-local proof → a claim constructed through I → its
retained P verified without the elaborator → repeated execution through
a prepared handle* — assembled incrementally as each capability lands,
then broken deliberately at each joint, "to learn whether the
interfaces compose at tolerable cost before a large library, the full
engine or a certified host exists". Where the joints stand: an
imported declaration enters by citation and a `(realize NAME (view))`
attaches Init's own body (slices 5b, 3.18); a claim through I is
slices 3.20–3.21; retained P verified by K alone is slice 3.22's rule
7 with its tamper cases; the prepared handle is phase 4's (T6). The
one phase-3 joint still missing is the **caller with a branch-local
proof** — T1's "a `dite` whose `h` is used only in a `Fin.mk` field",
carried since slice 7 because `Fin n` needed a value parameter at E.
Slice 3.18 removed that obstacle (`Fin n` is `Fin`, `Fin.mk i h` erases
to `i`); what is still missing is the source form — an `if` elaborates
to `ite`, so its branches have no hypothesis a program can cite, and
the dependent form is elaborated only inside a recursive function for
the descent obligations (slice 3.13 rule 2), its hypothesis never
named in source. The erasure already reads `dite` back as an `if`
with the branches applied to their erased proof.

**The rules of slice 3.24:**

1. **`(dif h C T F)` — the dependent if, its hypothesis named.** `C`
   is elaborated as an `if`'s condition is (a proposition with a
   decision the elaborator knows, or a `Bool` as `C = true` with
   `Bool`'s decidable equality); `T` is elaborated under a local
   `h : C` and `F` under `h : ¬C`, each at the form's expected type
   (the then-branch's own type when the position leaves it open, as
   an `if`'s); the term is `dite C dec (λ h. T) (λ h. F)`, Lean's
   elaboration of `if h : c then t else e`. The name comes first, as
   `fn` and `let` put a binder's name first; `if` is not overloaded
   with a fourth argument, which a reader would have to count. A
   value of a two-constructor inductive is not a condition here
   (`dif_type`): there is no proposition to name. On the E-first
   route (a file without Init) the form is refused by name
   (`dif_needs_init`): `Decidable` and `dite` are Init's. The
   erasure is the existing one; the equations of a measured
   function see the branch as they see an `if`'s.
2. **The example, `v3/examples/path/`.** The path on one program: an
   imported Init function with its realization attached; a
   bounds-checked access `at xs i` that returns the element under
   `(dif h (< i (List.length xs)) …)` through `Fin.mk i h` and
   `none` otherwise; a theorem about it constructed through I; an
   entry that runs it on raw arguments; the store written from the
   load and replayed by K alone.
3. **The test, `kernel/test/path_test.sh`, one entrypoint.** The path
   in order — load, run on a valid argument, store, verify by K alone
   — then the law's five breaks, each required to fail by name: a
   wrong executable body in a `realize` (refused at the equation
   check), missing bound evidence (the `Fin.mk` outside its branch,
   or with the hypothesis dropped, refused by the elaborator), a
   mismatched revision (the sidecar's entry stale, the store's order
   line mistied), a tampered result (a stored proof replaced, rejected
   by K), an invalid raw argument (refused at the entry, exit 6). The
   breaks slice 3.22 already tests are re-exercised on this example,
   not re-implemented.
4. **The measurement.** The cost of the whole path on the example,
   load through verify, is R76's number; the as-built reports it.

*Not in this slice, stated:* the prepared handle (T6) and the
World-alias fixture are phase 4's; the example stays pure.

**Slice 3.24 as built** (2026-10-09): as the rules, with what the
path found.

- **Rule 1.** `elab_dif` in `kernel/elab.shard`: the condition as
  `elab_if` reads it; each branch elaborated under its hypothesis as
  an `el_local`, closed by `el_lams`, the result type held to be free
  of the hypothesis (`dif_type` otherwise); `dite` applied by
  `op_apply_explicit` at the expected type. The classifier refuses
  the form by name on the E-first route (`dif_needs_init`). Pins:
  `dif_ok` (a `Fin.mk` field, a `Bool` condition, both branches'
  hypotheses in theorems by `dif_pos`/`dif_neg`), `dif_type`,
  `dif_needs_init`, `dif_scope` (the hypothesis outside its branch:
  the typed route's `unknown_constant: h`, carried by the E-first
  refusal that follows).
- **Rule 2, the example.** `at` returns `Option (Fin (List.length
  xs))`; `pick` takes the index, its parameter type dependent, and
  reads through `List.get?Internal`; `read` composes them; the entry
  prints the digit at a raw index of a fixed list. `List.length` and
  `List.get?Internal` are realized by supplied bodies with their
  equations proved: `get?Internal`'s structural recursion on the
  index is an `if` (`Nat`'s constructors are not E patterns), its
  second equation by cases on the index, named in the `equations`
  clause. The claims `at_some`, `at_none` are `(by (unfold at) (rw
  (dif_pos h)) rfl)` and its `dif_neg` twin. Fifty-six declarations
  stored, every one accepted by K alone.
- **Rule 3, the test.** `kernel/test/path_test.sh`, the suite's 37th
  entrypoint, 18 checks: the path in order, then the five breaks — a
  wrong body refused at `List.length.realize_2` (`conversion`), the
  hypothesis cited outside its branch (`unknown_constant: h`), the
  `Fin.mk` without its proof (a `type_mismatch`: a function of the
  bound where a `Fin` is expected), the order line naming another
  declaration (MISTIED), the stored proof replaced by its statement
  (REJECT), a non-number at the entry (exit 6).
- **Rule 4, the cost.** On the development machine under the Init
  receipt: load 18 s, load and run 23 s, load and store 18 s, verify
  by K alone 47 s (45 s of it the Init fixture's replay, the 56
  declarations the rest); the test 279 s. The path composes; its cost
  is the Init stream four times over, which slice 3.23 halved and a
  load shared across a process's roots (slice 3.25) removes for the
  pins; a driver pays it once per load.
- **Found on the way, open:** `List.get`'s own realization — the
  access with the `Fin` as its argument — needs a match that
  generalizes a local whose type mentions the scrutinee (`i : Fin
  (List.length xs)` under `match xs`), so that the nil row can refute
  its bound; today only the expected type is generalized (slice 3.19
  rule 9), and slice 3.13 rule 2's narrower guarantee states the
  gap. The path reads through `List.get?Internal` on the index's
  value instead, the proof carried by the type. The dependent match
  has its consumer now; it is not built here. Also: a `(realize NAME
  (view))` of an Init definition whose body is a matcher
  (`Option.getD`) is refused for want of the matcher's realization —
  Lean's `Option.getD.match_1` is not what K regenerates from its
  type — so the example writes its own `or_zero`.

**Slice 3.25 — the shared Init load: the stream replayable, the
horizon the module's, Init's names Init's** (ruled 2026-10-09 at the
boundary after R75 and R76, the lever item 57 stated; §13 item 59). The
loader pins' entrypoint re-streamed Init once per case in one process:
2,227 s of the v3 job's 5,501 s on the runner, 783 s locally, the
suite's wall clock since slice 3.20 and the first cost the library arc
would multiply. The export is one fixed library; a process that loads
many roots has no reason to read it more than once. Three rules:

1. **The Init stream is a prefix of a load and replayable.** `init_all`
   streams every chunk to its end (the record `INIT (all): N
   declarations admitted`); `ld_reroot` starts a fresh root load from a
   load's Init state — the checked environment, the stream's state, the
   ordinal table, the chunks and the receipt carried, everything of the
   old root reset to `load0`'s. A driver's lazy stream is unchanged: a
   program pays the prefix its imports reach, once.
2. **The horizon is the module's, never the stream's.** `(import Init
   NAME)` is answered from the ordinal table — NAME admitted = NAME in
   the table — and a module's horizon is its imports' ordinals and its
   imported modules' horizons (slice 3.21b rule 2), with no fall-back
   to how far the process happened to stream. Every gate that asks
   whether an Init constant is available to a module asks the scope
   (`visible`: below the module's horizon), not the environment: the
   elaborator's `first_missing` (an `if`'s `Decidable`, `dite`, `Not`,
   `Int.ofNat`), `arith`'s kit, the measure's `WellFounded` kit, a
   `"…"` literal's `String`. **Init's names are Init's:** a declaration
   bearing the name of an Init constant is refused (`name_taken`)
   wherever the environment holds that constant, as K admits no second
   constant under a name (`already_declared`) and the release gate
   holds the whole export; a driver whose lazy stream stopped short
   accepts what the gate refuses — the one place the lazy stream and the
   gate disagree, stated, closed at the gate. (The claim made the same
   day that `v3/std/list.shard`'s `List.sum` was such a case was wrong:
   that file declares `std.list.List.sum`, as §3.1 says every declared
   name carries the module path, and K holds it beside Init's
   `List.sum`; the live shape is a module named as an Init namespace —
   the pin `registry_native`'s `Bool` — which slice 3.26 rule 5 refuses
   from the name table wherever it loads.)
3. **The loader pins' entrypoint streams once.** The two fixtures are
   streamed to their end under the receipt, each case loaded fresh on
   that state by `ld_reroot`; a case's records are its own module's,
   by rule 2. The three cases that leaned on the stream stopping short
   are restated: `str_no_string` and `arith_unreached` hold by the
   horizon, `registry_native` is the refusal of rule 2 (the finding it
   pinned stands for a lazy driver, `define_test.sh` reads that dump).

**Slice 3.25 as built** (2026-10-09): as the rules.

- **Rule 1.** `kernel/loader.shard`: `init_all` is `init_through` with
  the target `Anon`, which the stream's end answers; `ld_reroot l root`
  is `load0` with the environment, the Init state, the ordinals and the
  receipt of `l`. Nothing of a driver changed (`load.shard`,
  `prove.shard` stream lazily as before).
- **Rule 2.** `init_through` consults `ords_find` on the ordinal table,
  not `env_find`; `init_horizon` has no fall-back (a name not in the
  table leaves the module's horizon). `elab.shard` `el_sees` (in the
  environment and `visible` from the module's scope) is the one
  availability test: `first_missing` and `int_of_nat_visible` through
  it, `tactic.shard`'s `has_env_const` replaced by it, `arith.shard`'s
  `ar_has` and the linearization's environment parameter replaced by
  the elaborator state, `classify.shard`'s `read_estr` asking
  `visible` before the environment. `derive.shard`'s `dv_has` already
  asked both. The loader's `name_taken` stays the environment's: K's
  rule.
- **Rule 3.** `kernel/test/loader_pins_test.shard` builds one load of
  the two fixtures under the receipt, streams it by `init_all` (its
  records printed first, `INIT (all): 3523 declarations admitted`),
  and runs each case on `ld_reroot` of it through the kit's
  `load_all`; the three pins restated as rule 3 says. The entrypoint:
  225 cases in 33 s under the receipt against 783 s (the Init stream
  18 s of it; the cases 15 s), and 57 s without a receipt, the
  checked stream once.
- **Not changed, stated:** the shell tests are one load each and stay
  lazy; `path_test`'s four loads of the Init stream (slice 3.24 rule
  4) are four processes, which a shared load in one process does not
  reach — the persistent session of law §9.3's prepared handle is
  phase 4's.

**Slice 3.26 — Init on demand: the export indexed, a module's closure
read by range, no fixture** (ruled 2026-10-09 at the boundary after
slice 3.25, on the library arc's first question; §13 item 60). The
library arc cites Init wherever Init has the statement, and an import
costs its position in the export (slice 3.17's measurement above): a
far lemma such as `Int.ediv_emod_unique` at line 3,409,839 would admit
half the export. The loader parses the export in shard under the
bootstrap at 0.8 MB/s (the two fixtures, 14.3 MB, in 18 s under the
receipt); the export is 333 MB; the whole of it checked under the
interpreter is 76 minutes and 8 GB resident; and the receipt caches
K's verdict, not the environment. Measured on the export: the closure
of `Int.ediv_emod_unique` — every line its type and value reach,
through the hash-consed nodes — is 690 records and 35,076 lines, under
one percent of the export; `Nat.and_comm` 746 and 42,468; `List.sum`
25 and 686. A closure cut at the granularity of a record's block (the
lines a record emits before its own) is 15,482 blocks and 2,240,409
lines for the same lemma, a third of the export: a shared node lives in
whichever block first needed it. So the unit is the line, and a load
reads what it cites. Six rules:

1. **The index.** One pass over the export (`kernel/index.shard`;
   `initindex.shard EXPORT INDEX NAMES`) writes two tables beside the
   receipt, never loaded whole: the record table — one fixed-width row
   per record in export order: its kind, the byte offset and length of
   its block, and the first name, level and expression id the block
   holds (or, holding none, the next id the export assigns), so the
   block holding an id is the last row whose first id is at most it —
   and the name table, one fixed-width row per declaration name as
   `show_name` renders it, sorted bytewise with its ordinal, so a name
   is a binary search of range reads. Node ids are the export's,
   assigned in emission order (checked over the export), which is what
   makes a row per record enough. The header rows carry the export's
   byte count; an index over another export is refused. The pass
   refuses an ambiguous rendering (two names rendering alike) and a
   name past the row's width.
2. **The range read.** `host.shard` gains `read_range path off n`: at
   most n bytes from off, fewer at the file's end, `None` where the
   file cannot be read — beside `read_file`, in the bootstrap's handler
   and the chain's runtime. The loader reads the export itself
   (`init.ndjson`); the 20,000-line chunks stay the replay's.
3. **The load.** `(import Init NAME)` takes NAME's ordinal from the
   name table as the module's horizon (slice 3.21b rule 2 unchanged).
   A module's demand is every identifier of its forms under the
   candidates resolution tries (`scope_candidates`: the module's
   prefix, the opened prefixes, `Init.NAME`, the bare name) and the kit
   lists the gates hold (the elaborator's `first_missing` lists, the
   `WellFounded` kit, `arith`'s kit, the derivations', `String`),
   filtered to the names the table places below the horizon. The
   loader walks each demanded declaration's closure by range reads —
   its record line, the nodes its type and value reach, their names
   and levels, the records of the constants they name — sorts the
   lines into export order and feeds K that sub-stream through the
   path the prefix stream took (`process_record`), checked (admitted
   under a receipt, as ruled; the receipt was dropped at landing 3).
   Node ids stay the export's; K's tables are sparse; the records are
   byte-identical to the stream's.
4. **The law unchanged.** Visibility is the ordinal below the horizon
   (`init_within`); a materialized record carries its true ordinal, so
   a far name one module pulled in stays invisible to a module with a
   nearer horizon. Resolution asks the environment first
   (`const_arity`), so a name not materialized is simply absent; a
   name the elaborator constructs that no kit list names is a refusal
   by that name (`unknown_constant`) and a defect of the list, which
   frontend parity finds. No materialize-and-retry: one mechanism.
5. **Init's names are Init's, from the index.** A native declaration
   bearing a name the name table holds is refused (`name_taken`)
   wherever it loads — the driver and the release gate agree; §13 item
   59(c)'s index, as reopened on `List.sum`.
6. **No fixture.** The prefix fixtures (`init_prefix_int.ndjson`,
   `init_prefix_str_tail.ndjson`, `init_prefix_3000.ndjson`) and their
   dependents go to the export, which `v3/export.sh` provides on every
   machine; a closure checked costs seconds (K's check about 7 ms a
   record: the fixture's 24 s over 3,523), so no test needs the
   receipt (ruled a speed lever here; dropped at landing 3, measured at
   a tenth to a fifth of a demanded load); `init_index.sh` keeps the
   index current as `init_receipt.sh` kept the receipt.

Three landings: (1) the index, the range read, a test that reads every
record back by its offset and every name by its search; (2) the loader
on demand — the prefix stream, the stream state and the chunk list
deleted, the demand set, the walk and the sub-stream, `name_taken` from
the table, the fixtures deleted — under the loader pins, parity and the
suite; (3) the receipt over the export, CI in the order export, build,
index, suite, replay. Weighed and deferred, each with the consumer that
would reopen it: a theorem admitted by its statement alone (its closure
five to ten times smaller — 143 records and 3,098 lines for
`Int.ediv_emod_unique` — but a change to K's admit path, which builds
the declaration from the node tables); the loader native (its files are
not route 1's); a serialized environment (Lean's `.olean`), whose only
consumer, the replay, checks. Rejected: a native parse primitive (a
second JSON reader for a second per load), a materialize-and-retry loop
(a second mechanism), the exporter emitting the index (upstream at a
pinned commit; offsets are a property of the text held here).

**Slice 3.26 as built, landing 1** (2026-10-09): the index and the
range read, as rules 1 and 2.

- **Rule 1.** `kernel/index.shard` is the library — the two formats,
  the pass (`ix_build`: windows of the export by range, every line
  placed, the name lines entered, the record rows rendered, the name
  rows sorted), the readers the loader will use (`ix_read_header`,
  `ix_read_row`, `ix_lookup`: the binary search of the name table;
  `ix_block_of_id`: the last row whose first id is at most the id) and
  the check (`ix_verify`: a second pass over the export comparing every
  record's row and looking every name of it up, the export's byte count
  against the header's, the name table's order). `kernel/initindex.shard`
  is the driver, `test/init_index.sh` keeps `v3/.cache/init.index` and
  `init.names` current over the export (stamped by the export's pin and
  size and the index's sources: a kernel edit does not stale it).
  Measured: the two fixtures (3,523 records, 3,985 names, 14.3 MB)
  build in 46 s and verify in 63 s; the export (347,714,179 bytes, 57,977
  records, 59,433 names) builds in 952 s and verifies in 866 s on the
  bootstrap — once per environment. Compiled by route 1 (`v3/build.sh v3/kernel/initindex.shard
  v3/bin/initindex`, 4 s with `bin/shard_eval` booting the chain; the
  pass's state is a plain type with accessors, as K's `Run`, since the
  chain compiles no record sugar) the driver indexes the export in 68 s
  and verifies it in 71 s, the tables byte-identical to the bootstrap's;
  `init_index.sh` prefers the binary when it is newer than the sources. The index is 62 bytes a record and
  218 a name: 3.6 MB and 13 MB over the export.
- **Rule 2.** `read_range` in `host.shard`, the bootstrap's handler
  (`eval.rs`) and the chain's runtime (`tools/codegen/rt.h`,
  `rt_read_range`); the export's single file is read, never a chunk.
- **The test.** `test/index_test.sh` (the suite's 38th entrypoint) builds
  the fixtures' index, verifies it, requires the record count K's stream
  admits and the name rows of a definition, an inductive block's type
  and its recursor, and refuses three tampered inputs by name: a name
  row's ordinal changed, a record row's length changed, the export a
  byte longer than the header says. The driver is in parity's closures.
- **Found on the way:** export lines carry their keys in alphabetical
  order, so an expression line reads `{"app":…,"ie":N}` and the id is
  found by its key, not its position; the fixtures' records are 3,523
  as K counts them, and the quotient's four records each name their own
  constant.

**Slice 3.26 as built, landing 2** (2026-10-09): the loader on demand,
as rules 3–6; the fixtures deleted.

- **Rule 3, the walk** (`kernel/initload.shard`): the export and its
  index opened at a process's first need (`od_open`: the meta line
  against the pin, the index's header, the export's size the header's);
  a name's ordinal by binary search, memoized, the search's top ten
  levels of rows cached (`od_lookup`); the closure of a set of
  demanded records walked by range reads (`od_load`): the blocks
  scheduled highest first, a block read once and scanned from its
  record line backwards, every pending line parsed and its name, level
  and expression ids pushed — a child precedes its parent, so in the
  same block it comes later in the scan and in an earlier block it
  schedules that block — and a constant an expression cites resolved by
  its name id through the **declaration table** (the third table, 9
  bytes a name id, added here: without it a constant's record is found
  only by rendering its name, which needs the name's ancestors, which
  come later in the walk) to the record declaring it, which is pushed.
  The lines come out ascending (sorted only when a push went upward)
  and go through K's `process_record`: node lines into K's tables,
  sparse, the ids the export's; records checked, or admitted under a
  receipt. What K has been fed persists (`OdDone`), so a line is fed
  once in a process. **The demand** (`loader.shard` `init_prepare`,
  after a file's directives): every token of its forms under
  resolution's candidates (`scope_candidates`, the token's `@` sigil
  dropped, the candidates under a prefix the export declares nothing
  under skipped — the file's own identity, an opened native module),
  the gates' kits — K's and the pins' (`k_kit`), the elaborator's
  (`el_kit`), the tactics' (`tc_kit`), arith's (`ar_all_kit`), the
  derivations' (`dv_kit`), the definer's (`df_kit`) — and, for an
  inductive block's names among them, their satellites; the names the
  table places below the horizon, their closure through K. `(import
  Init NAME)` takes NAME's ordinal from the name table (`init_through`;
  a name the export does not declare is `init_name_not_found`) and the
  record reads `INIT NAME: horizon H, N declarations admitted`.
- **K's run of Init alone.** The loader's Init state holds K's run fed
  Init's records and nothing else; each demand is checked into it
  (`od_load`) and then admitted into the loader's own environment,
  which holds the natives beside Init (`od_admit`: the same lines over
  the run's tables, without the typing judgments, as under a receipt).
  `ld_reroot` starts a fresh root on that run's environment, so a
  process loading many roots loads each record once: the loader pins'
  entrypoint runs its 225 cases in 42 s, every record checked, against
  33 s admitted under the receipt and 57 s checked at slice 3.25; the
  first form of this landing, re-rooting from the state before any
  demand, took 1,761 s.
- **Rule 4.** Unchanged in code: `init_within` on the true ordinals
  (`ld_add_inits` enters each admitted constant with its ordinal);
  `ACCEPT … init=H` carries the module's horizon. Three constructed
  names the kits lacked were found as the law says, by a refusal:
  `Lean.Omega.Int.mul_congr` (arith composes it from the operator;
  `calc_app`'s descent proof and the `arith_div` pin), the `@` sigil
  on a token (`std/derive.shard`'s `@String.toByteArray_inj`), and the
  `init_not_found` pin, which asked for `Int.tdiv` past the fixture's
  end and now asks for a name the export lacks.
- **Rule 5** (`name_taken`): a file's `Fx.taken` is every name the
  export declares under the file's identity (`od_names_under`, one
  search and a forward read), and a declaration — the file's own, a
  derivation's — bearing one is refused `name_taken`; the release gate
  looks each stored name up in the table and refuses it as `TAKEN`.
  Found running it: the `List.sum` case item 59(c) was reopened on is
  no case — `v3/std/list.shard` declares `std.list.List.sum` (§3.1), K
  holds both — and the live shape is `registry_native`'s module `Bool`;
  `define_test` now requires that refusal where it used to read the
  lazy driver's acceptance.
- **Rule 6.** The three prefix fixtures and `init_receipt.sh` are
  deleted; every consumer reads the export and its index (`test/
  init_index.sh` prints both paths; `v3/test.sh` keeps the index
  current where it kept the receipt): the shell tests through
  `--init EXPORT --init-index INDEX`, the loader kit's tests through
  `kit_load0`, `calc_harness.shard EXPORT INDEX INPUTS`,
  `verify_release.shard [-v] DIR EXPORT INDEX` (its demand: every
  constant the store's records cite, resolved through the index and
  checked, never admitted), `route2_test` and `t0_fixture_test` on the
  export's first 3,000 lines against the full oracle, `index_test` on
  its first 20,000 (519 records, 691 names, 3,153 name ids; 9 s
  against 125 s). The receipt stays a flag (`--init-receipt`, the
  export listed at its byte count), with no writer until landing 3.
- **Measured** (bootstrap, checked, no receipt; the loads in parallel):
  `examples/calc/calc.shard` 12 s (its demand through `Int.decEq`: 667
  records), `calc_spec` 29 s, `calc_app` 28 s, `natabs` 26 s,
  `calc_proof` 34 s (through `Lean.Omega.Int.ofNat_lt_of_lt`: 1,274
  records), `examples/auto` 24 s, `std/bytes` 24 s (through
  `UInt8.toNat`: 1,305), `std/list` 10 s, `std/derive` 26 s,
  `examples/path` 22 s (through `List.get?Internal`: 1,155); the
  nested-import pin 1 s. The index is one file of 19,208,769 bytes,
  built by the route-1 driver in 36 s and verified in 36 s (the two
  tables of landing 1: 68 s and 71 s).
- **Gates run:** the 225 loader pins 0 failed in 42 s, `loader_test`
  0 failed, the full suite's 38 entrypoints with 0 failed in 455 s against 613 s at landing 1 (parity byte-identical over 31 closures and 133,465 declarations in 267 s; `store_test` 350 s, the suite's wall clock; `path_test` 228 s; `define_test` 230 s; `calc_test` 215 s; `k_clients_test` 69 s; the loader pins' entrypoint 42 s). None of the files route 1 compiles changed; the full replay and the corpus are CI's.
- **Found on the way:** K's `unknown_constant` refusal names the
  declaration, not the constant it lacks (a kit's defect was found by
  instrumenting a scratch copy of K to carry the name) — a diagnostic
  worth K's refusal carrying it, deferred; the block offset of landing
  1's record table was the newline before a block's first line (one
  byte early, the verify tolerating it), fixed with the format.

**Slice 3.26 as built, landing 3** (2026-10-09): the receipt dropped,
the slice closed. The landing as planned gave the receipt its writer
over the export; pipeline 563 and a measurement retired it instead.

- **The measurement.** With every load checking the closure it cites,
  the entrypoints made of many short loads doubled on the runner at
  landing 2 (`wire_test` 155 s to 436, `define_test` 337 to 657,
  `engine_test` 267 to 392; `wire_test` 65 s to 150 locally), each
  process checking its demand where the receipt had admitted the
  fixture's prefix. A hand-written receipt over the export (`PATH
  BYTES`) admitted a demand in nine tenths to four fifths of the
  checked time: calc 9 s against 10 s, `std/bytes` 20 s against 25 s.
  The walk on the bootstrap and K's ingestion are the cost; a receipt
  skips neither. The user ruled the receipt dropped.
- **Deleted.** `load.shard --init-receipt` and `prove.shard`'s, the
  loader's receipt (`Load.receipt`, `receipt_mode`, `receipt_parse`,
  `load_with_receipt`), `t0.shard --receipt` and its writer, the
  loader pins test's flag, `v3/test.sh`'s `INIT_RECEIPT` and the
  shell tests' pass-through. A driver starts from `ld_start` (the
  export and its index named, or no Init). `init_receipt_stale` is no
  refusal. K's admit mode (`admit_decl_pinning`, `run_admitting`)
  stays for the loader's second admission of records it checked into
  K's run of Init a moment ago (`od_admit`), its comments saying so;
  slice 3.23's text stands as history, marked retired, and §13 item
  57 with it.
- **The lever that remains** is item 60(f)'s loader native: the walk
  is the cost of a demanded load, and its files are not route 1's.
- **CI's order** stands as landing 2 forced it: export, the index
  driver, the index, the suite, the kernel build, the replay.
- **Gates run:** the 225 loader pins 0 failed in 46 s (43 s inside the suite), `loader_test` 0 failed in 8 s, `t0_fixture_test` and `route2_test` byte-identical (route 2 in 26 s), the full suite's 38 entrypoints with 0 failed in 620 s against 455 s at landing 2 — every entrypoint about a third slower while the pins' entrypoint held at 43 s against 42 s, so contention on the machine, the per-load cost unchanged (parity byte-identical over 31 closures and 133,392 declarations in 354 s; `store_test` 484 s, the suite's wall clock; `path_test` 314 s; `define_test` 303 s). `t0.shard` is route 1's: the kernel build, the full replay and the corpus are CI's.

**Slice 3.27 — the library arc: Init is the library, the old modules
become records, the seams the probes found are closed** (designed
2026-10-09 at the boundary after slice 3.26, on the user's "continue
with the next arc"; §13 item 61 — **ratified 2026-10-10 as designed,
both leans taken: calc's `list.shard` retires onto `std/list` at
landing 3; the migration tool's tier 0 is phase 5's opener**). What law §12.4's phase 3 still owes after slices 3.9–3.26:
`std/list`, `order`, `nat`, `div`, `bits`, `arith` under the naming
law; the fifteen former axioms of `kernel/facts.shard` as theorems; the
migration table validated; one arbitrary-`Prop` ghost refinement; one
static law-bearing package; `docs/LEAN.md`; the gates T2, T3, T9
(small) and T10. The user's hesitation at the 3.25 boundary stands as
the arc's principle: the library must not duplicate work already in
Lean's `Init`. Measured by probes before this design (three scratch
files under the on-demand loader, the bootstrap, 2026-10-09): twelve of
the fifteen former axioms are theorems by Init's names in one file of
twelve lines (47 s; 1,395 declarations admitted through the horizon
`Nat.shiftRight_zero`, ordinal 55,829); six of Init's list functions
realized by supplied bodies and seven of its list theorems cited (29 s;
1,385 declarations); five of `std/bits`' statements by Init's names
(54 s; 1,887 declarations). Three seams found on the way, each a defect
or a gap of the elaborator, not of the law: `/` at `Int` elaborates to
`Int.div`, which this pin does not hold (`Int.ediv` is Euclidean
division; `%` already goes to `Int.emod`); a lemma Init states through
an instance (`0 &&& x = 0` is `HAnd.hAnd Nat Nat Nat _ 0 x = 0`) cannot
be `apply`d to a goal in V3's spelling (`Nat.land 0 b = 0`) — the
argument under the instance application is "not determined by the
goal" (`witness_open`), while `(exact (Nat.zero_and b))` goes through
K's conversion, which unfolds the instance; and a cited name above the
module's horizon is `unknown_constant`, so the author must know the
export's ordinals to write the import (`Nat.zero_and` is 23,368,
`List.take_cons` 54,202, `Nat.min` 10,228). Found beside them: Init's
instance-polymorphic functions (`List.sum` under `[Add α] [Zero α]`)
are refused `instance_needed` at a concrete type, and `List.length_take`
states `min`, for which V3 has no operator. Eleven rules:

1. **Init is the library.** A statement Init has is cited by Init's
   name; `v3/std` declares only what Init lacks, in Init's namespaces
   under the naming law (law §5.3); an old module every declaration of
   which is Init's or `arith`'s becomes a migration record and no file.
   V3's own `List.sum` (slice 3.13's first library file) goes: it is
   Init's `List.sum` under rule 3.
2. **One spelling, everywhere.** Slice 3.21 rule 3 made `arith` read a
   class method at `Int` or `Nat` under Init's instance as its
   function; the elaborator's unifier and the rewriter's matcher
   (`apply`, `rw`, `simp_only`, the engine's ladder) do the same: an
   instance constant and a class projection applied to it unfold during
   unification and matching, as Lean's `instances` transparency does,
   so `HAdd.hAdd Nat Nat Nat (instHAdd Nat instAddNat) a b` meets
   `Nat.add a b`, `HAnd.hAnd … Nat.instAndOp` meets `Nat.land`,
   `HAppend.hAppend … List.instAppend` meets `List.append`, and a lemma
   in either spelling applies to a term in the other. V3's operator rows
   keep their function spellings (slice 3.13 rule 3); the probe's
   `witness_open` is this rule's regression.
3. **Instances by table, not search** (Stage 3's first step, law §5.1).
   An instance-implicit binder `[C α]` at a concrete `α` of the
   library's types (`Nat`, `Int`, `Bool`, `List`, `Option`, `Prod`,
   `String`, `Char`, `Fin`, `UInt8`, `ByteArray`) resolves from a fixed
   table keyed by class and type to the pin's instance constant
   (`instAddNat`, `Int.instAdd`, `instMinNat`, `List.instAppend`,
   `instDecidableEqNat`, `Int.instDecidableEq`, `instBEqOfDecidableEq`
   over a decidable equality) or to a composed term the table spells
   (`Zero Int` is `Zero.ofOfNat0` over `instOfNat`); `(min a b)` and
   `(max a b)` at `Nat` and `Int` are `Min.min`/`Max.max` under the
   table's instance — Init's one spelling, an atom to `arith` as slice
   3.21 rule 4 has it. A class or type the table lacks is
   `instance_needed` with today's pointer (write `@` and the instance).
   General instance search stays Stage 3's door, wake condition = a
   class the table cannot hold.
4. **`/` at `Int` is `Int.ediv`.** The operator row (`op_at`) says
   `Int.div`; the pin has no such constant and law §10.3's row says
   Euclidean. Fixed with a pin; the row validated by `std/migration.shard`.
5. **The horizon refusal points.** A cited name the name table holds
   above the module's horizon is refused `above_horizon NAME: ordinal
   N, the horizon H — (import Init NAME)`, the import to write, where
   today it is `unknown_constant`. The horizon law (slice 3.21b rule 2,
   3.26 rule 4) is unchanged: the author chooses the import; the
   loader, which has the table, says which.
6. **Init's functions realized by supplied bodies**, slice 3.24's
   precedent, with their equations theorems K proves: by `rfl` where
   the body follows Init's recursion (`List.length`, `List.append`,
   `List.reverseAux`, `List.reverse`), by `cases` on the argument Init
   recurses on first where the body recurses on another (`List.take`
   and `List.drop` recurse on the count first: `take n [] = []` is by
   cases on `n`) — until the derived view reads Lean's compiled
   recursion (deferred below). The matcher of an Init definition
   (`Option.getD.match_1`, slice 3.24's open item) is realized through
   the matcher's own definition — a definition over `casesOn`, a view
   of it derived as any definition's — measured at landing 3; the
   fallback is a supplied body as today.
7. **The dependent match**, slice 3.24's other open item, lands at
   `List.get`: a `match` generalizes the locals whose types mention the
   scrutinee (slice 3.19 rule 9 generalizes the expected type only), so
   that under `match xs` with `i : Fin (List.length xs)` the nil row
   refutes `i`'s bound; slice 3.13 rule 2's narrower guarantee gets its
   consumer. `List.get`, `List.getD`, `List.get?Internal` and
   `List.getElem?` are then realized as rule 6 has them.
8. **The modules as records.** For each of the six, a per-interface
   migration record (law §10.1's "per-interface record") in
   `v3/std/README.md`: every old declaration, its V3 spelling or Init's
   name, the connecting evidence (a theorem of `std/facts.shard` or
   `std/migration.shard`, or "cited by name"), the class of law §10.2.
   `std/order` (22 claims about `Int`'s order: `Int.le_refl`,
   `Int.lt_irrefl`, `Int.le_trans`, …, the `succ`/`pred` shifts by
   `arith`), `std/nat` (`add_nat`, `int_of_nat`, `half_nat` are
   `Nat.add`, `Int.ofNat`, `Nat.div n 2`), `std/div` (seven
   requirements over the literal divisor 10: `arith`'s quotient and
   remainder rows, slice 3.21 rule 4; `div_nonneg` is
   `Int.ediv_nonneg`) and `std/arith` (seven index identities: `arith`)
   have no V3 file. `std/list.shard` holds the realizations of rules 6
   and 7, `List.Pairwise`'s ghost refinement (the arbitrary-`Prop`
   `Subtype` phase 3 owes: a sorted list as `Subtype (List.Pairwise
   (· ≤ ·))` with an insertion keeping it) and what Init lacks;
   `std/bits.shard` holds the three bitwise recurrences and the
   width material Init lacks (`2^32`, `2^64` instances of
   `Nat.or_lt_two_pow`, `Nat.xor_lt_two_pow`,
   `Nat.and_two_pow_sub_one_eq_mod`), the old 2,135 lines gone: Init
   has `Nat.zero_and`, `Nat.and_zero`, `Nat.zero_or`, `Nat.zero_xor`,
   `Nat.xor_self`, `Nat.and_le_left`, `Nat.and_le_right`,
   `Nat.shiftLeft_eq`, `Nat.shiftRight_eq_div_pow` and the masks.
9. **The fifteen former axioms as theorems**, `v3/std/facts.shard`,
   the successor of `kernel/facts.shard` and theorem-only — the old
   statement on the new spelling, its proof the citation:

   | former axiom | the theorem's evidence | status |
   |---|---|---|
   | `mod_lo` (`0 < d → 0 ≤ n mod d`) | `Int.emod_nonneg` (its premise `d ≠ 0` by `Int.ne_of_gt`) | proved by the probe |
   | `mod_hi` | `Int.emod_lt_of_pos` | proved |
   | `ediv_mod_id` | `Int.emod_def` under `arith` | proved |
   | `div_unique`, `mod_unique` | `Int.ediv_emod_unique`, its two halves | proved |
   | `mul_comm`, `mul_assoc`, `mul_dist` | `Int.mul_comm`, `Int.mul_assoc`, `Int.mul_add` | proved |
   | `bshl_z`, `bshl_s`, `bshr_z`, `bshr_s` | `Nat.shiftLeft_zero`, `Nat.shiftLeft_succ`, `Nat.shiftRight_zero`, `Nat.shiftRight_succ_inside` — at `Nat`, the `0 ≤` premises gone (the typed class of law §10.2) | proved |
   | `band_rec`, `bor_rec`, `bxor_rec` (the low bit arithmetically, `a = 2·(a/2) + a%2`) | by `Nat.eq_of_testBit_eq` with `Nat.testBit_and`/`or`/`xor`, `Nat.testBit_succ`, `Nat.testBit_zero` and `Nat.mod_two_eq_zero_or_one`; or one unfolding of `Nat.bitwise` by its `WellFounded` equation (`Nat.bitwise_rec_lemma`), then `cases` on the two low bits and `arith` — the shorter route is `LEAN.md`'s example of a well-founded definition's equation | landing 3 |

10. **`docs/LEAN.md`** (law §5.4): the three lists — the same as Lean,
    refused with the pointer and the phase, shard-only — in dozens of
    rows, written from the probes' findings and the landings': Init's
    binders are the pin's (`(exact (NAME a b))` over-applied is
    `function_expected` with the type shown; `(apply NAME)` takes any
    shape); the horizon import by the refusal's pointer; `@` with the
    instance where rule 3's table has none; a `cases` arm `(zero ()
    STEP…)` names its fields in a list even when there are none; `/`
    and `%` Euclidean; `=` a proposition, `==` a `Bool`; numerals by
    position; `fn` = `def` + `realize`; `measure`; `dif`; `use` =
    `open`; `import Init NAME` = the horizon. The seam document's length
    is a symptom (law §5.4).
11. **T9 small** (law §5.4, §12.5), at landing 4: a fresh agent with
    `LANGUAGE.md` and `LEAN.md` and nothing else — no transcript, no
    kernel source — (i) proves a `sum_list_append`-class theorem (law
    §5.3's schematic) over `std/list`; (ii) writes a `fn` that lowers
    (route 1's `v3/build.sh` on its file); (iii) cites one Init theorem
    by guess from the naming grammar; (iv) states a ghost invariant
    (`Subtype` over `List.Pairwise`); (v) proves a branch under `dif`;
    (vi) writes a refused Lean form (`notation`, `partial`, `get!`) and
    receives the pointer. Scored by the refusals and interventions on
    the record, the count agreed before the run (law §12.5's rule);
    each finding a `LEAN.md` row. T9 runs again at phase 3's close.

Four landings. (1) **The seams**: rules 2–5 in the elaborator, the
unifier and the loader, each with a pin (`apply` of `Nat.zero_and`;
`List.sum` at `Int`; `(min a b)`; `(/ n d)` at `Int`; a name above the
horizon), under the loader pins, parity and the suite. (2) **The
records**: `std/facts.shard` (the twelve), `std/migration.shard` (one
theorem per row of law §10.3 — `Int.ediv_zero`, `Int.emod_zero`,
`Nat.sub` saturating, `Int.tdiv`/`Int.tmod` named, the `Decidable`
bridges, `Nat.land` on `Nat`), `std/README.md`'s six records,
`kernel/test/std_test.sh` as the 39th entrypoint — the std root loaded
in one process (the Init run shared, slice 3.25), every file stored and
accepted by K alone (slice 3.22's gate), `define_test`'s `List.sum`
expectation replaced. (3) **`std/list` and `std/bits`**: rules 6–9 —
the realizations, `List.get` under the dependent match, the matcher,
`List.sum` from Init under rule 3, the ghost refinement, the three
recurrences; calc's `examples/calc/list.shard` (its own `append` and
`len` "until `v3/std` exists") retires onto `std/list` as the library's
first consumer — calc's claims re-spelled `List.length`/`List.append`,
the `auto` sidecar regenerated by `prove.shard`, byte-identical
thereafter. (4) **`LEAN.md` and T9 small**, rules 10–11; the findings
folded in.

Weighed and deferred, each with the consumer that reopens it: **T2 and
T3** (law §4.3's lambda profile — closed lambdas as template arguments,
named partial application, a captured value as a runtime parameter;
`add_offset`; the static law-bearing package with no runtime
dictionary; the type-growing recursion refused loudly) are a language
slice, not a library one, and become **slice 3.28** with its design at
this arc's close; **T10** (an ordinary E library contributing an
I-producing tactic through law §7.3's API — `engine.shard` is that
library today, inside the kernel) becomes **slice 3.29**; the derived
view of Lean's compiled structural recursion (rule 6's lever; consumer:
the count of supplied bodies, twelve at landing 3, growing with the
bulk port); the loader native (item 60(f); the std entrypoint will be
the next multi-load cost); general instance search (rule 3's door); the
migration tool's tier 0 (law §12.3) — phase 5's opener, these records
its calibration data: the law's "calibrated on `std` in phase 3" is
read so, because the six modules mostly vanish under rule 1 and a tool
that re-spells them would mostly delete, stated here as a departure.
Rejected: a V3 declaration beside an Init one for an executable
spelling (`List.sum`, calc's `len` and `append`; law §4.4's "no second
mathematical declaration"); a V3 function spelling for `min`
(`Nat.min` is Init's own second spelling, ordinal 10,228, not the
instance's); porting `std/bits`' proofs (Init has the theorems); a
materialize-and-retry at the horizon (slice 3.26's one mechanism).

**Slice 3.27 as built, landing 1 (2026-10-10) — the seams.** Rules
2–5, in the elaborator, the unifier and the loader, each with a pin.
- **Rule 2 as built — the one spelling is a table, not a transparency.**
  Slice 3.21 rule 3's reading of a class method under Init's closed
  instance as its function (`ar_canon`, arith.shard) is the shared
  module `kernel/spelling.shard` (`sp_canon`), with the rows the
  library needs added: `HAnd`/`AndOp`, `HOr`/`OrOp`, `HXor`/`XorOp`,
  `HShiftLeft`/`ShiftLeft`, `HShiftRight`/`ShiftRight` at `Nat` to
  `Nat.land`, `lor`, `xor`, `shiftLeft`, `shiftRight`; `HAppend`/`Append`
  at `List α` to `List.append`. The unifier reads both sides through it
  at a structural mismatch under a metavariable, before K's whnf
  (`unify_fallback`): K's whnf of `0 &&& ?n` against `Nat.land 0 b`
  computes the closed side to `0` and the metavariable is never
  assigned — the probe's `witness_open`; read as `Nat.land 0 ?n` the two
  unify structurally. The rewriter needed nothing new: since slice 3.21
  every goal's statement (`tac_goal`) and every lemma's equation
  (`eq_of_lemma`) enter in the one spelling, so `rw` and `simp_only`
  across the two spellings follow from the rows alone. The mechanism
  the rule's text names — an instance constant and a class projection
  unfolding, Lean's `instances` transparency — would need the
  environment at every mismatch and would unfold Init's definitions
  past the instance (K's whnf has no smart unfolding: `Nat.add a ?b`
  becomes a stuck `Nat.rec`); the table is the same reading `arith`
  has trusted since 3.21, closed over the pin's names, and K rechecks
  every term it shapes. A class the table lacks is the door's wake
  condition, as rule 3 has it for instances.
- **Rule 3 as built.** `sp_instance`: the type of an instance-implicit
  metavariable, closed, to the pin's instance — at `Nat`: `instAddNat`,
  `instSubNat`, `instMulNat`, `Nat.instDiv`, `Nat.instMod`, `instLTNat`,
  `instLENat`, `instMinNat`, `Nat.instMax`, `instDecidableEqNat`,
  `Nat.instAndOp`/`instOrOp`/`instXorOp`/`instShiftLeft`/`instShiftRight`,
  `Zero` as `Zero.ofOfNat0 Nat (instOfNatNat 0)`; at `Int`:
  `Int.instAdd`, `instSub`, `instMul`, `instDiv`, `instMod`, `instNegInt`,
  `instLTInt`, `instLEInt`, `instMin`, `instMax`, `instDecidableEq`,
  `Zero` as `Zero.ofOfNat0 Int (instOfNat 0)`; at `Bool`:
  `instDecidableEqBool`; at `List α`: `List.instAppend`,
  `instDecidableEqList` over the element's; at any type `BEq` as
  `instBEqOfDecidableEq` over its `DecidableEq`; the heterogeneous
  `HAdd`…`HAppend` at one type as `instHAdd`…`instHAppendOfAppend` over
  the homogeneous row; `OfNat Nat n`/`OfNat Int n` as `instOfNatNat n`/
  `instOfNat n`. Asked (`el_solve_slots`) for an application's own
  binders once the expected type has had its say, for every
  metavariable when a declaration finishes (`el_finish`), and for a
  tactic opening's premises (`witness_close`); anything left is
  `instance_needed` with the pointer as before (`elab_instance`: `ite`'s
  `Decidable True` is no row — `if` has its own decisions, slice 3.16).
  `(min a b)`/`(max a b)` are operator rows to `Min.min`/`Max.max`, their
  instance from the table; `min` and `max` join the operator symbols,
  which take precedence over a declared constant of the same name as
  `+` does (no V3 file declares either). The table's constants are no
  token of the file that needs them: `sp_kit` lists every name the two
  tables write and joins the loader's demand (`init_kit`), loaded where
  the module's horizon admits it — 39 declarations on the probe file.
- **Rule 4 as built.** `op_at`: `/` at `Int` is `Int.ediv`; `%` was
  `Int.emod` already; at `Nat` the pair stays `Nat.div`/`Nat.mod`.
- **Rule 5 as built.** `init_resolve` keeps the names the name table
  places at or above the module's horizon (`Fx.above`, with their
  ordinals; the walk already looked them up to skip them), and a
  reading's `unknown_constant` on one of them is `above_horizon`:
  `NAME is declaration N of the export, at or above the module's
  horizon H (import Init X): write (import Init NAME)`. The name is
  matched as a token of the refusal's message (the elaborator's
  detail, or a tactic step's `step 1: NAME — the goal: …`), itself or
  under a prefix an opened namespace supplies (`zero_and` under `(use
  Init.Nat)` names `Nat.zero_and`). `init_horizon_beyond` now expects
  `above_horizon`.
- **Cleanup beside the landing:** `mk_app_n` moved from matcher.shard
  to expr.shard, where its users are; arith's `ar_cast`, `ar_not`,
  `ar_carrier` are the shared module's `sp_cast`, `sp_not`, `sp_carrier`.
- **Pins:** `op_int_div`, `one_spelling` (`apply`, `rw`, `simp_only`
  across the two spellings at the bitwise operators and `List.append`),
  `instance_table` (`List.sum` at `Int` and `Nat`, `min`/`max`,
  `List.length_take`), `init_above_horizon` (never loaded, cited bare),
  `init_above_horizon_by` (inside a by block); `unify_test` gains the
  one-spelling case in both orders. **Gates run:** the 230 loader pins 0 failed in 54 s, `unify_test` 0 failed, `tactic_test` 110 checks (calc's 100 claims) 0 failed in 44 s, `define_test` 12 checks 0 failed in 241 s, `engine_test` 24 checks 0 failed (54 of calc's 100 claims unaided, unchanged), `derive_test` 14 checks, `wire_test` 15 checks, `k_clients_test` 7 tests in 72 s, parity byte-identical over 31 closures and 133,762 declarations in 350 s, the full suite's 38 entrypoints with 0 failed in 553 s (under contention with the targeted gates; `store_test` 400 s, `parity_test` 333 s, `define_test` 308 s, `calc_test` 298 s, `path_test` 289 s). Route 1's kernel build, the full replay and the corpus are CI's.

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
| tactic blocks, the I elaborator, the goal graph, `sorry` as a hole | §5.1 Stage 2, §7 | 3 |
| typeclasses, instances, coercions | §5.1 Stage 3 | 3 — instance arguments at the library's types from a fixed table at slice 3.27 landing 1 (§8.4 rule 3, designed 2026-10-09); search stays the door |
| the `Init` import with E realizations attached; `String`, `Array`, `ByteArray` representations (Init's E-eligible inductives are E types since slice 5b, §7.5; a `realize` attaches a body per constant) — **the byte-list representations landed at slice 3.18** (§8.4: `String`, `ByteArray`, `Array`, `UInt8`, `BitVec`, `Fin`); the packed buffers are the lowering's, behind the same types | §4.4, INVENTORY | 3 |
| lambda lifting, templates, specialization | §4.3 | 3–4 |
| `bin`, `requires`, the World-use check, effect traces | §4.7 | 4 |
| prepared handles, long-lived environments | §9.3, T6 | 4 |
| evaluation reflection: `ev`'s theorem, the `rfl` node | §4.4, T8 | 4 |
| the canonical S form: CANON's rule set rewritten for S — **landed slice 8, `v3/CANON.md`**; its recognizer, formatter and gate | §5.1 "One canonical S", §10.5 | the rule set 2 (done); the gate 6 |
| T1's branch-local proof: a `dite` whose `h` is used only in a `Fin.mk` field — `Fin n` over erased bounds needs a **value** parameter at E, which §7.5's eligibility rule (types and propositions only) does not admit; `Fin`, `dite` and `Nat.decLt` are all inside the `Int` fixture, so the export is not what blocks it | law §4.1, §4.2 | 3, with the `Init` realizations (ruled 2026-09-13: carried as a gate item, not built early without Stage 1's typing) |
| T5's "two validated instances of one interface" — §6.6 binds one implementation per view directory (§13 item 14) | law §8.2, T5 | 3, with item 14's `mod.req/` siblings, when a consumer needs two |
| T5's "an imported theorem about the original still usable after a realization" with an **imported** theorem — every theorem about a realizable constant lies past the `Int` fixture (`ite_self` at export line 18,116); the native form of the fixture is pinned (`realize_theorem`) | law §4.4, T5 | 3, when the prefix grows for the `Init` realizations |
| `CheckedEnv` sealed: K one directory module behind a view (§6.6); complete when the first `meta/` consumer is behind it (§13 item 26, R52) | §3.5, §8.2 | 3, the opener |
| the Stage-1 authoring facilities ordinary source needs before the broad port (GPT-6 R60): list literals with expected-type-driven empty lists; a negative-numeral rule with no silent `Int → Nat`; named-field construction and update, a changed dependent field an explicit obligation, never hidden by the sugar; a `Name` literal or construction that forges no declaration, node or environment identity (§3.3); a scoped fresh-name supply in ordinary code where `gen_fresh` was; the public byte and text adapters (`ByteArray` for raw bytes, `String` for text, the prelude's cells an internal adapter) decided with the first host-facing S library — each with a first example before the port pays for its absence | §5.1 Stage 1; §12.6 | 3, before the broad migration |
| the checked module instance (GPT-6 R57): the substitution from view parameters to implementation declarations and evidence recorded, the `fulfills` proofs' assumptions inherited by the instantiated consumer, a stricter policy refusing the result, one consumer proof used with two implementations without cloning it, revisions bound so spellings alone mix nothing | law §8.2, T5; §6.5 | 3, the two-instance gate |
| programmatic validation (GPT-6 R61): a client that builds a `Prog` or declarations as data validates them through the public services without a source round trip — item 15's fusion is an implementation choice, the dump of §10 a conformance format, neither the representation | §9.3, T6 | 4 |
| the one E (§8, RULED 2026-09-14): the bootstrap to the whole of E, the toolchain migrated by tool, the profile flag deleted — slices 3.2–3.5 | §8.3; law §9.2 as amended | 3, first |
| `Symbol`'s L identity `String` and the flip of `"…"` from the byte list to a `String` value in E, for every file at once, the toolchain migrated by tool | §8.1 rules 1–2; INVENTORY | 3, with `String`'s E realization |
| user notation | §5.3 departure 6 | never in v1 |

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
| `(bin …)` (16), `trusts`, `requires` | deferred | phase 4 (§4 reserved forms); `trusts` already carries the axiom policy at phase 2 (§9) |
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
    `fulfills_outside_impl`.
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
    when route 1's chain is V3's own.
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
    one-identity rule K's environment rests on.
40. **E-only view parameters** (§3.3; landing 2): a `sig fn` whose
    signature names an E-only type is an E parameter only — no axiom
    in K, the head linked at `run` — and a `sig type` whose
    implementation is E-only is matched by its arity, K admitting the
    sig type as an opaque constant. Alternative: making the data
    vocabulary K types by importing `Init` into `name`, `expr`, `decl`
    — `Symbol`'s L identity is phase 3's later slice, and every
    identity and the load's cost would change for no client's gain.
41. **A view may import a plain file outside its own directory**
    (§6.6's req-scope gate refined; landing 2): the K view states its
    signatures over the public data files. A file of the view's own
    directory stays `req_scope` — the implementation's side. Alternative:
    the data files under `k/mod.req/` as req-scope siblings, which
    moves six public files behind a path nothing else needs.
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
    of its file.
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
    coercion, in E only until 3.15.
45. **One term grammar for `def` and `fn`, one elaborator** (§8.4
    guard 1, carried from the phase-2 direction checkpoint): the
    surface is the union of the explicit-L and E forms, `tr` the
    elaborator, a `fn` a `def` that also passes the classifier. At
    3.13 the statement and one pin; the reader is unchanged.
46. **The interface is the whole of a consumer's knowledge** (§8.5;
    the user's steer of 2026-09-17): nothing L crosses a view's merge;
    a `sig fn`'s definition and equations exist in the fork only; the
    requirements are the lemmas; the checked instance is a
    substitution over P, never a re-elaboration; deriving on a `sig
    type` is the module's. Alternative: exporting `eq_N` through the
    view automatically — every consumer proof would then weld to
    today's body, which is exactly what v2's weaning rule forbade.
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
    (2026-09-18):** see §8.4's as-built.
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
    (l) taken as proposed.**
