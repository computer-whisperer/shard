# The V3 language — S, L and E at Stage 0 (phase 2 draft)

> **STATUS (2026-09-14): DRAFT at the phase-2 boundary — Stage 0 of
> law §5.1 (explicit L, no inference), pending the ratification pass
> over §13 below and `v3/CANON.md` §9.** Normative parent:
> `docs/FOUNDATION.md`. Scope: the surface S, the executable fragment E
> and `ev` as phase 2 built them — the reader (§2, §4–5), the loader
> (§3), views (§6.5–6.6), the classifier and `ev` (§6.2–6.4, §6.7),
> `realize` (§7), the one E (§8, ruled and built 2026-09-14), entries
> (§9), conformance (§10). Each semantic rule has one current statement here with its
> Stage-0 limit beside it; a later "as built" section fixes what the
> rule left to the implementation and never overrides it (GPT-6 R62,
> slice 10). The build history — which slice built what, the
> measurements, the findings — is `docs/records/FOUNDATION.md` §9; the
> review correspondence is its §4. The proof IR **I** is phase 3's
> chapter and is not here. What Stages 1–3 add is §11 with its phase;
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
| `(record NAME (ctor CTOR)? (FIELD TYPE)+)` | named-field products (v2's form, docs/LANGUAGE.md "Records"; slice 3.9, §13 item 42): loader-level sugar expanded before any form is read — the positional `(type NAME (CTOR TYPE…))`, CTOR defaulting to `MkNAME`, an accessor `NAME.FIELD` and an updater `NAME.with_FIELD` (value first) per field in the record's namespace, which the file opens as any type's (item 7): `(FIELD r)` bare where no other opened record has the field, `(NAME.FIELD r)` always; NAME may be `(NAME P…)`, the generated fns then binding `(P Type)` first. `(make NAME (FIELD V)…)` — every field exactly once, order-free — and `(with NAME E (FIELD V)…)` — chained updaters, a later entry outermost — both against the current file's records, rewritten in every form of the file, nested values first (`kernel/record.shard`; the bootstrap's `expand_records` the twin, parity the tie). No law family at Stage 0: a fn has no L meaning; v2's laws return with Stage 1. Layout: as `type`'s until CANON's phase-6 gate | E: as the forms it expands to |
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
OP   ::= + - * / mod % lt < le <= > >= int_eq = != and or iff
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
  `(le 48 c)` is `c`'s (slice 3.16) — `+ - * / mod` at `Nat` are `Nat.add sub
  mul div mod`, at `Int` `Int.add sub mul div emod`; `lt le` (also
  `< <=`) `Nat.lt le` / `Int.lt le`, `> >=` the flipped ones;
  `int_eq`/`=` is `Eq`, `!=` is `Ne`, `iff` is `Iff` (their type the
  implicit argument); `and or not` are `And Or Not` on propositions
  and Init's `and or not` on `Bool`s. In L nothing runs, so `-` `/`
  `mod` at `Nat` have their identities here where E refuses them
  (§8.4 rule 3, guard 2: a restriction of E, not a dialect).
- **`if`** is `ite C dec T F` where `dec` is the decision of `C`'s
  head the elaborator knows — `Nat.lt le` → `Nat.decLt decLe`,
  `Int.lt le` → `Int.decLt decLe`, `Eq` at `Nat`/`Int`/`Bool` →
  `Nat.decEq`/`Int.decEq`/`instDecidableEqBool` — else `no_decision`
  with the pointer to `@ite` with the instance; a `Bool` condition
  `c` is the proposition `c = true` with `instDecidableEqBool` (Lean's
  own elaboration; slice 3.16 rule 3 — slice 3.15 had `cond`); a
  condition of another two-constructor inductive is the `match` it
  abbreviates, the second constructor taking the then-branch (§6.2's
  tag rule). `(list a b …)` is the constructor chain of the expected
  type's list. `(exists ((x T)) P)` is `Exists (fun (x : T) => P)`.
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
law §4.7 has no effectful primitive) — under their profile names, plus the naming-law spellings of law §10.3 as **distinct**
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
two-constructor type; S's own story is phase 3's, §12.4); the entry's `World` argument is its
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
(GPT-6 R58, slice 10).

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
parameter — so §7.1's `Fin.mk` example waits for the reserved
type-representation form (phase 3). A constructor with an erased field can
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
projection is a `match` on the structure's constructor; a `let` whose
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
   convention (§6.7) until `String`'s E realization at phase 3, when
   the rule flips for every file at once and the toolchain migrates by
   tool (§11). `(quote x)` and `'x` are `Symbol` literals; `(list a b
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
   ediv band bor bxor bshl bshr int_eq sym_eq lt le sym_of_chars
   chars_of_sym` — are E's `Int` and `Symbol` primitives in every file
   (§2 lexes them), beside the naming-law identities (`Nat.add`,
   `Int.tdiv`, …; §6.4). Their L identities are assigned at phase 3
   with law §10.3's table (`/` stuck at zero stays a distinct entry
   from the total `Int.tdiv`, as §6.4 says); a `realize` body cites the
   L-identified entries only (`no_l_identity`, unchanged).
6. **The wire and the prelude.** Unchanged: `kernel/prelude.shard`'s
   `List`, `Option`, `Bool` and `Pair` are the wire's cells, interned
   by the linker whether or not a program declares them (§6.7); the
   prelude's `Nat` (`Z`/`S`), which no V3 file used, is deleted (slice
   3.4) — `Nat` is a built-in (rule 1).
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
| 3.17 | the porting facilities of §11's R60 row: record update and order-free construction, `Name` literals, symbols and `"…"` in a body, the fresh-name convention, E's rename of `lt le int_eq` with the tool migration | the broad port |
| 3.18 | the byte and text adapters (`String`'s E realization, item 37's flip; the wire's type ruled first); deriving under a declared policy | the first host-facing S library |
| 3.19+ | I: the node vocabulary (law §7.2), `elaborate(I)` to P, the goal-graph API as E functions, `by` blocks, the core tactics; then `tools/prove` and the engine as I producers | calc's 100 claims; the coverage arc's B-1c |

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
   `Int.add`, `Int.sub` and `Int.mul`; `int_eq`, `lt` and `le` are
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
   splits, the `let`-bound values as `let`s — and the proof term
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
   gives the constant motive); a dependent motive is a later slice.
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
   `Decidable` cell) and its stated trust. `Int.add`, `Int.sub`,
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
    **deferred to I's opener**, slice 3.19). Today `mc_drop_fvars`
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
    negative numeral is already §5.3's. `"…"` in a `fn` body, symbols,
    `Name` literals, record update, the fresh-name convention and E's
    rename of `lt le int_eq` are slice 3.17; the byte and text
    adapters and deriving are slice 3.18, the wire's type ruled first.
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

G5 (contextual delayed filling) is rule 10's and opens slice 3.19.

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
  exports.
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
prelude's `(List Int)`, Init's `(List Nat)` or `(List Int)` — takes
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
| deriving under a declared policy | §5.1 | 3 |
| tactic blocks, the I elaborator, the goal graph, `sorry` as a hole | §5.1 Stage 2, §7 | 3 |
| typeclasses, instances, coercions | §5.1 Stage 3 | 3 |
| the `Init` import with E realizations attached; `String`, `Array`, `ByteArray` representations (Init's E-eligible inductives are E types since slice 5b, §7.5; a `realize` attaches a body per constant) | §4.4, INVENTORY | 3 |
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
| return types unchecked (no load-time typing) | **changed at phase 3** | Stage 1 types every `fn` body against its signature; v2 code that runs only because nothing checked it will be refused then. At phase 2 unchanged |
| `if` on `True`/`False` by constructor name | carried, generalized | §6.2's tag rule; v2's `(type Bool (False) (True))` has Init's constructor order |
| `match`: first match wins, nested patterns, integer and `(quote S)` patterns, `_`, bare 0-ary constructors | carried in E | symbol patterns profile only; Stage 1's match compilation must keep first-match semantics (Lean's does); **in L since slice 3.16** a `match` is a generated matcher (§8.4 slice 3.16 rule 1): first match wins, nested patterns, `_` and bare field-less constructors carried; an integer or `(quote S)` pattern is `literal_pattern` there until a consumer under the naming law has one |
| parallel `let`, no `let*` | **changed**: sequential in L and E (RULED 2026-09-12, R44) | §5.4; 0 of the tree's 30,611 `let` groups depend on parallel binding, so no source changes meaning; the bootstrap evaluator's parallel rule gives identical results on all of them until the V3 reader replaces it (slice 2) |
| `(quote S)`, `'S`, the `Symbol` type, `sym_eq`, `sym_of_chars`, `chars_of_sym` | carried — **decided 2026-09-14** (§8.1 rule 1) | `Symbol` is a built-in E type in every file, the interned atom, L identity `String` at phase 3; K's `Name` values are built from its atoms as today. The S-side refusal `symbol_literal` retired at slice 3.4 (2026-09-14) |
| `(list a b c)` (9,455 uses outside `v3/`) | carried — **decided 2026-09-14** (§8.1 rule 2) | the constructor chain of the `List` in scope, by the scope at Stage 0 and by the expected type at Stage 1 (R60); the S-side refusal `list_sugar` retired at slice 3.4 (2026-09-14) |
| `"…"` = UTF-8 bytes as `(List Int)`, on the extern wire too | carried in E — **decided 2026-09-14** (§8.1 rule 2); **changed** at `String`'s realization | in an E body the byte list of the `List` in scope for every file (the S-side refusal `string_literal` retired at slice 3.4, 2026-09-14); in L positions K's `String` literal (§5.3). At `String`'s E realization the E rule flips for every file at once, the toolchain migrated by tool (§11). **AT RISK:** the extern wire's byte convention under the naming law (`List UInt8`? `ByteArray`?) is undecided; at phase 2 the wire's cells are the toolchain prelude's `List`, `Option`, `Pair` and `Bool` (§6.7) |
| `Int` numerals everywhere, `-7` | carried in E — **decided 2026-09-14** (§8.1 rule 2); in L since slice 3.15 (§5.3) | in an E body a numeral is an integer of any sign whose type is the binder's; in L a numeral takes the type its position expects — `Int.ofNat n` / `Int.negSucc (-n-1)` at `Int`, `Nat` otherwise (law §5.2's rule) |
| unbound identifier = `FVar` (proof-time opened variables) | dropped | an unbound name is a resolution error; K refuses free variables; I's named context replaces the use (phase 3) |
| primitive dispatch by name, trie-first, bodyless-name collision = stuck (`pins/lang/prim_shadow_rejects`) | **changed** — landed slice 5 | heads classified at load into four node kinds; a primitive is an identity (§6.4); a declared name shadows the table's (the scope resolves first); an unknown head is refused at load |
| the primitive table | carried under the profile names; **changed** under the naming-law names | §6.4; `/` stuck at zero versus `Int.tdiv` total are two entries |
| `gen_fresh`, the one effectful primitive | **dropped** (slice 5) | law §4.7 has no effectful primitives: pure `ev` refuses reachable externs. No V3 file calls it and the table does not carry it; a ported source that needs fresh names threads a counter (ten old-tree kernel files: types, canon, sequent, reduce, tactics, proof_reader, proof, trace, the two lowered twins) |
| `Nat` former: `Z`/`S` packed to literals, patterns match literals by view, proof-facing normalizers never pack, bare literals do not type as `Nat` | **changed** | `Nat` is Init's; K's literal rules (offset, `Nat.zero` ≡ `0`, accelerators) replace the former; numerals type as `Nat` by rule (§5.3). The profile keeps the prelude's `Nat` for measures; `ev` does not carry the bootstrap's `Z`/`S` view over integer literals (no V3 file matches on them; a literal matches `Z` never) |
| `(refine BASE PRED)`, `refine_val`, `refine_try`, `refine-fact`, `(returns …)` (37 types, 43 `refine-fact` sites) | re-spelled; deferred | `Subtype` over any `Prop` (law §4.1): `refine_val` → `Subtype.val`, `refine_try` → a `decide`-bridged constructor, `refine-fact` → `Subtype.property`; the return obligation is a Stage-1 elaboration obligation. Phase 3 (REFINEMENT.md superseded) |
| `(record …)`, `make`, `with`, `F_of`/`with_F`, the six-law family, `NAME_eta` (21 files, 237 `with_F` sites) | re-spelled; **AT RISK** | `structure` with projections (§4): `F_of` → `NAME.F`; `NAME_eta` is K's structure eta for free; the laws are `rfl`. **No Stage-0 form for `with_F` updaters or order-free `make`**: Lean's `{ s with f := v }` is elaborator sugar — Stage 1 must add it or every update site spells the constructor |
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
| `div-facts TERM D Q` | `omega` (owns the `Nat`/`Int` seam, law §5.2) | |
| `inject`, `absurd` | `injection`, `noConfusion`/`absurd` | `T.noConfusion` and `T.c.inj` generated after a native inductive without parameters (slice 3.15, §8.4 rule 5); `injection` the tactic is I's |
| `rewrite-with EQREF DIR SIDE (INST…) (PROOF…)` | `rw` carrying lemma identity, instantiation, direction, guarded occurrence path | law §7.2 |
| `(by arith (list …))` — tautology, Farkas certificate (5,105 uses) | `arith` with the checked certificate as I data; `omega` as the producer | law §7.2 lists `arith` |
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
| `int_eq`/`lt`/`le` return `Bool` | re-spelled | `Decidable` propositions with `decide` bridges; `==` for `Bool` values |
| World threading with no use check | **changed at phase 4** | the well-threadedness check (law §4.7): a v2 program that uses one World token twice will be refused; the World-alias fixture lands before any effectful port |
| the extern roster `get_args read_file read_dir read_key write write_file write_line exit` | partly carried | `kernel/host.shard` declares six; `read_dir` (the old loader, codegen) and `read_key` (snake) return when a V3 consumer needs them. **AT RISK:** the wire convention (above) |
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
| symbols in S (`quote`, `Symbol`, `sym_eq`) | phase 3, Stage 1 (the reader) | the toolchain's own sources (ten kernel files) when they port to L | a `fn` using `(quote x)` and `sym_eq` under the V3 reader is refused with a named reason until decided | a `Name` literal, or symbols stay profile-only |
| `(list a b c)` | phase 3, Stage 1 | every ported `fn` (9,455 sites); calc's program half, ported to S at slice 6, spells its list literals as constructor chains | `(list 1 2)` under the V3 reader refused by name | a list literal at Stage 1 |
| the extern wire's bytes | slice 5 (`ev`'s extern boundary — **bytes as the prelude's cells, landed**); phase 3 for the L type | `sha256sum`'s bin; calc's app step — an S program names no wire cell at phase 2, so calc's differential keeps its drivers outside the program (slice 6, §10) | `write_line` of a literal round-trips its bytes through the driver (route 2's byte-tie) | bytes at phase 2; `ByteArray` or `List UInt8` decided with §5.3's `String` realization |
| negative numerals | phase 3, Stage 1 | calc (negative `Int` results); `std/div` | `-7` under the V3 reader = the constructor term at Stage 0; calc's port (slice 6) writes no negative literal and computes its negative results from `-` at run time, its `Nat` numerals running as integers | Stage 1 special-cases `-` on a numeral; ruled then |
| `gen_fresh` | **decided slice 5: dropped** | the ten old-tree kernel files (canon, tactics) as they port | a `fn` citing `gen_fresh` is refused at load (`unknown_head`) | a threaded counter in the ported toolchain |
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
    change of meaning.
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
    until Stage 1. v2's `FIELD_of`/`with_FIELD` was the first cut and
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
    is 3.19+.
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
