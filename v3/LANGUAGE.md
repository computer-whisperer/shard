# The V3 language — S, L and E at Stage 0 (phase 2 draft)

> **STATUS (2026-09-14): DRAFT at the phase-2 boundary — Stage 0 of
> law §5.1 (explicit L, no inference), pending the ratification pass
> over §13 below and `v3/CANON.md` §9.** Normative parent:
> `docs/FOUNDATION.md`. Scope: the surface S, the executable fragment E
> and `ev` as phase 2 built them — the reader (§2, §4–5), the loader
> (§3), views (§6.5–6.6), the classifier and `ev` (§6.2–6.4, §6.7),
> `realize` (§7), the one E and the profile it retires (§8, ruled
> 2026-09-14), entries (§9), conformance (§10). Each semantic rule has one current statement here with its
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
E's executor and by the V3 reader as its gate (§10); until phase 3's
opening slices land they are read under the toolchain profile §8
retires.

## 2. Lexical syntax

The on-disk format is s-expressions.

- **Whitespace** separates tokens; otherwise insignificant.
- **Comments** begin with `;` and run to the end of the line (`;`
  trailing, `;;` line, `;;;` file and section headers — carried).
- **Numerals**: decimal digit strings, `42`. A leading `-` is a
  numeral only in the toolchain profile (§8); in S a negative integer
  is written `(Int.neg (Int.ofNat 7))` or `(Int.negSucc 6)`, exactly
  the L constructors it denotes (the law's §5.2 numeral rule is Stage
  1).
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
RUNNABLE fn M.f | RUNNABLE extern M.e | RUNNABLE type M.T   ; an E declaration classified (§6.7; R45's status kind: runnable, no L meaning)
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
- **A `type` form opens its own namespace** for the rest of its file
  (§13 item 7); `inductive` and `structure` do not.
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
| `(structure NAME.{u…} BINDERS (FIELD TYPE)…)` | one constructor `NAME.mk`; projections `NAME.FIELD` generated as `def`s over `proj` (Lean's projections are `Expr.proj` definitions); fields dependent on earlier fields | K |
| `(def NAME.{u…} BINDERS TYPE VALUE)` | an L definition; hints `regular` at height 1 + the greatest height it references | K: `DefnDecl` |
| `(abbrev …)` | as `def` with hint `abbrev` | K |
| `(opaque NAME.{u…} BINDERS TYPE VALUE)` | checked, never unfolded | K: `OpaqueDecl` |
| `(theorem NAME.{u…} BINDERS PROP PROOF)` | PROOF is `(exact TERM)` or `sorry` at Stage 0; `sorry` is reported loudly, admits nothing, and every citation of the theorem is a pending obligation, never an assumption | K: `ThmDecl` |
| `(axiom NAME.{u…} BINDERS TYPE)` | admitted only under `(trusts NAME)` in the same file (§9) | K: `AxiomDecl` |
| `(fn NAME BINDERS RET (measure M)? BODY)` | an E function: parameters are E-types, BODY is §5.4's E term language; classified (§6.3); **no L declaration at phase 2** (§0) | E: `EFn` |
| `(extern NAME BINDERS RET)` | a World extern (law §4.7) | E: `EExtern` |
| `(sig fn NAME BINDERS RET)`, `(sig type (NAME T…))` | a view's bodyless signature and opaque type (§6.5) | E: `ESig`, `ESigType`; L: a view parameter |
| `(requirement NAME BINDERS PROP)`, `(fulfills NAME PROOF)` | a view's promised law and its discharge in the implementation (§6.5) | L |
| `(realize NAME …)` | an E body for an admitted L constant (§7) | E: `EFn`; L: the equations |
| `(trusts AXIOM…)` | widens this file's assumption policy (§9) | policy |

**Binders** are `((x TYPE) …)`; a binder may carry an info marker,
`(x TYPE implicit)`, `(x TYPE strict)`, `(x TYPE inst)`, recorded as
K's `BinderInfo` (display and Stage-1 data; no rule depends on it).
At Stage 0 every argument is passed explicitly regardless of the
marker.

**Universe parameters** are declared by the `.{u v}` suffix on the
declared name and cited by the same suffix (§2). A citation without a
suffix is a constant with **zero** universe parameters; citing a
polymorphic constant bare is a Stage-0 error with the pointer to write
`List.{0}` — except in **E-type positions** (a `fn`'s binders and return
type, a `type`'s fields, a `realize` signature), where every
polymorphic type constructor is instantiated at level 0 by rule, since
E-types live in `Type` (law §4.2). That is reconstruction of what the
form determines, not inference.

## 5. Terms

### 5.1 Explicit L (the argument of `def`, `theorem`, `inductive`, `structure`, `realize`'s equations)

```
TERM ::= NAME                          ; a bound variable (innermost binding wins), else a constant
       | NAME.{LEVEL…}                 ; a constant with universe arguments
       | (TERM TERM…)                  ; application, left-nested
       | (fun (BINDER…) TERM)          ; lambda
       | (forall (BINDER…) TERM)       ; Pi
       | (-> TERM… TERM)               ; non-dependent Pi, right-nested
       | (let ((x TYPE TERM)…) TERM)   ; sequential; one L Let per binding
       | (Sort LEVEL) | Prop | Type | (Type LEVEL)
       | NUMERAL                       ; K's Nat literal
       | "…"                           ; K's String literal
       | (proj S i TERM)               ; the i-th field of structure S (0-based)
```

Elaboration is term for term into `expr.shard`'s `Expr`: names bound
by `fun`, `forall` and `let` become `BVar` indices (locally nameless,
0 the innermost; the binder's display name is kept on the node);
unbound names resolve through the scope (§3.1) to `Const` with the
universe arguments written; `Prop` is `(Sort 0)`, `Type` is `(Sort 1)`,
`(Type u)` is `(Sort (succ u))`. A bound name shadows a constant. No
argument is inserted, no metavariable exists, no universe is inferred:
`(ite.{1} Nat (Nat.lt i n) (Nat.decLt i n) a b)` is what an author
writes for `if i < n then a else b` at Stage 0. This is verbose by
design — the machinery is the deliverable, Stage 1 is the ergonomics.

### 5.2 Levels

```
LEVEL ::= NUMERAL | NAME | (succ LEVEL) | (max LEVEL LEVEL) | (imax LEVEL LEVEL)
```

The same grammar inside `.{…}` and in `(Sort …)`; `NAME` must be a
declared universe parameter of the enclosing declaration
(`level.shard`'s `check_level`). Lean's `u+1` is `(succ u)`.

### 5.3 Literals

A numeral is K's `LitNat`; a string is K's `LitStr` and has type
`String`, the pinned declaration (`v3/INVENTORY.md`); `Char` values
are `(Char.ofNat 97)`. The toolchain profile reads both differently
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
  Stage 0 (§0); `(requirement NAME …)` discharged by the implementation's
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
runnable, no L meaning); an extern `RUNNABLE extern NAME`; a profile
`type` `RUNNABLE type NAME`; a refusal is `REFUSE NAME reason` like
K's, and counts as one.

**The profile as read (superseded 2026-09-14 by §8's one-E ruling;
retired by phase 3's slices 3.2–3.5).** A file is in the toolchain
profile iff its root-relative path begins with `kernel/` or `meta/`
(§8, §13 item 8);
the selection is recorded on the module's record (`MODULE …
profile=toolchain`, slice 10), so a load's records say which profile
read each file. The profile is a named, bounded source-compatibility
mechanism for bring-up and never a dialect (§8; GPT-6 R55): placement
under `kernel/` grants no privilege in K, and the destination of the
toolchain's sources is ordinary S. A profile file has **no L**: `type` declares its E inductive only,
`fn` and `extern` are E, any other declaration form is refused
(`profile_form`); the scope is **flat** — every declaration of every
visible module, constructors included, is citable by any suffix of its
identity, with no `use` — a bare type name in a binder that resolves to
nothing is an auto-bound type parameter (`(xs (List T))`), `Int` and
`Symbol` are the built-in E types, `"…"` is `(List Int)`, numerals are
`Int` with `-7` a numeral, `(quote X)` a symbol, `(list …)` sugar, and
the primitives are the bare names of §6.4. In S the same forms are
refused by name with the phase that decides them (§12.6): `(quote X)`
`symbol_literal`, `"…"` `string_literal`, `(list …)` `list_sugar`;
numerals are `Nat`; an E-type name resolves as an L constant does
(`Nat`, `Int`, `List` of `Init`; a `type`; a `sig type`).

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

**The primitive table's L identities.** Entries 0–17 (the profile's
spellings) have none: they are the bootstrap's Int operations and are
refused inside a `realize` body with the pointer to the naming-law
spelling (`no_l_identity`). Entries 18 and up are L constants under
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

**The ruling.** Phase 3 opens by building the Rust bootstrap up to the
whole of E, so that the toolchain's own sources — everything under
`v3/kernel`, later `v3/meta` — are written in the same E as every
other file, and the *toolchain profile* this section used to define
is retired (the user, 2026-09-14, on the sizing in records §9; GPT-6's
R55 dissolves with it — there is no profile left to name). Until the
retirement lands (the phase-3 slices below) the profile's rules of
§8.2 are what the reader applies to `kernel/` and `meta/`; the
text here is the rule set the profile converges to, and every
difference the old table listed is decided below, not carried.

### 8.1 The rules of the one E

1. **Built-in E types.** `Int`, `Nat` and `Symbol` are E types in
   every file. Where `Init` is in scope, `Int` and `Nat` are also its
   constants — one identity, the bare name (§3), exactly as the E table
   keys them today. `Symbol` is the interned atom the toolchain
   compares and prints; its L identity is `String` and its realization
   the atom, assigned at phase 3 with `String`'s E realization (§11).
   A file that sees no `Init` has no L: its `type` forms are E only
   (`RUNNABLE type`, as today's profile), and `def`, `theorem`,
   `inductive`, `structure`, `axiom`, `opaque` and `realize` in it are
   refused `no_init` — the rule that replaces `profile_form`, keyed on
   the file's scope, never on its directory. K's own sources see no
   `Init` by nature (K reads the export as data) and enter L at the
   flip, as §6.1 already says.
2. **Literals in E.** A numeral is an integer of any sign — `-7` is a
   numeral — and its E type is the binder's, `Int` or `Nat`; the
   reader carries one literal kind (`LInt`; §10 item 1's omission list
   loses one entry). In a `realize` body a numeral is K's `Nat` literal
   in the equations and a negative one is `equation_form` (Stage 1's
   numeral rule, §11). `"…"` is the byte list of its UTF-8 encoding as
   the `List` in scope — the prelude's in the toolchain, `Init`'s in a
   program that opens it — the convention of the wire (§6.7), until
   `String`'s E realization at phase 3, when the rule flips for every
   file at once and the toolchain migrates by tool (§11). `(quote x)`
   and `'x` are `Symbol` literals; `(list a b c)` is the constructor
   chain of the `List` in scope (R60's list literal: by the scope at
   Stage 0, by the expected type at Stage 1). In L positions nothing
   changes: a numeral is `LitNat`, a string `LitStr` (§5.3).
3. **Binders.** Type parameters are explicit `((T Type) …)` binders in
   every `fn`, `extern`, `sig fn` and `realize` (law §5.3 departure 4,
   now for the toolchain too); the parenthesized head `(fn (append T)
   …)` and the auto-bound bare type variable are gone.
4. **Scope.** `import` never opens and `use` does (§3.1), for every
   file. The toolchain's files carry `use` lines — one per import, one
   per prelude type whose constructors they cite bare — written by the
   migration tool. The bootstrap ignores `use` and resolves flat (as it
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
   prelude's `Nat` (`Z`/`S`), which no V3 file uses, is deleted at the
   migration.
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
   call under `eval direct`.

### 8.2 The profile's rules while it lasts (retired by the slices below)

| in the profile | in S | decided (§8.1) |
|---|---|---|
| the prelude's names `Nil Cons True False Some None Z S Pair` are the toolchain's own E types (`kernel/prelude.shard`), unrelated to `Init`'s | `Init`'s `List.nil` … | rule 6: unchanged; cited under `use` (rule 4) |
| type parameters by the parenthesized head `(fn (append T) …)` or by a bare type variable in a binder type (`(xs (List T))`), auto-bound | explicit `((T Type) …)` binders (law §5.3 departure 4) | rule 3: explicit binders everywhere |
| `"…"` is the `(List Int)` of its UTF-8 bytes | K's `String` literal | rule 2: the byte list in E for every file until `String`'s realization |
| numerals are `Int`; `-7` is a numeral | numerals are `Nat`; negatives are constructor terms | rule 2: any integer, the binder's type |
| `(quote X)` and `'X` are `Symbol` literals; `sym_eq`, `sym_of_chars`, `chars_of_sym` | no symbols: a name is a `Name` constructor value | rule 1: `Symbol` is a built-in E type, L identity `String` |
| `(list a b c)` is list sugar | none (Stage 1 may add it) | rule 2: the `List` in scope's chain |
| a file `import` also opens the imported module (today's flat scope) | `import` never opens; `use` does | rule 4: `use` lines everywhere |
| the primitive names of `docs/LANGUAGE.md` §8 | the naming-law spellings, same table (§6.4) | rule 5: one table, both spellings |
| a `type` is E only; there is no L in the profile — `def`, `theorem`, `inductive` are refused (`profile_form`) | a `type` enters K and E (§4) | rule 1: keyed on `Init` in scope (`no_init`), never on the directory |
| the profile is the `kernel/` and `meta/` directories of the root (slice 5; §13 item 8) | every other file is S | withdrawn: one E for every file |

Which files are in the profile is decided today by the loader from the
package layout, not by a marker form (the Rust loader refuses any
top-level form it does not know); the selection is recorded on the
module's record (`MODULE … profile=toolchain`, slice 10). Both go with
the profile.

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
- **3.3 the toolchain migrated by tool** — `use` lines and explicit
  binders in the 30 kernel files (2026-09-14 count: 210 functions with
  an auto-bound type variable; 7,710 bare constructor citations need
  only the `use`), file by file under the gates; the prelude's `Nat`
  deleted.
- **3.4 the V3 side to the one E** — the `profile` flag deleted from
  `sexpr`, `classify`, `etable` and `loader` (2026-09-14: 22, 77, 8
  and 20 sites), `no_init` in place of `profile_form`, rules 1–2 for
  every file (`string_literal`, `symbol_literal` and `list_sugar`
  retired; their pins turned positive or `no_init`), one literal kind
  in the dump, `MODULE … profile=` gone.
- **3.5 the documents closed** — §8.2 and §6.7's profile paragraph
  removed, §12's "profile only" rows decided, `CANON.md`'s note on the
  profile's symbols, `docs/TCB.md`'s bring-up item (2), records.

Then the opener as planned: the K seal under item 26's criterion,
Stage 1, I.

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
   obligations are Stage 1's) and a literal's kind (`LNat` and `LInt`
   print alike; the bootstrap has one integer type and the profile's
   literals are `Int` in both loaders). The text is not P, not a store
   format and not the embedding's representation.
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
| implicit arguments, first-order unification, universe inference, the numeral rule of §5.2, coercion `Nat → Int` | §5.1 Stage 1, §5.2 | 3 |
| match compilation, structural recursion to recursors, `f.eq_N`, `noConfusion`, `WellFounded.fix` from `measure`, `fn` = `def` + `realize` | §5.1 Stage 1, §4.5 | 3 |
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
| `match`: first match wins, nested patterns, integer and `(quote S)` patterns, `_`, bare 0-ary constructors | carried in E | symbol patterns profile only; Stage 1's match compilation must keep first-match semantics (Lean's does) |
| parallel `let`, no `let*` | **changed**: sequential in L and E (RULED 2026-09-12, R44) | §5.4; 0 of the tree's 30,611 `let` groups depend on parallel binding, so no source changes meaning; the bootstrap evaluator's parallel rule gives identical results on all of them until the V3 reader replaces it (slice 2) |
| `(quote S)`, `'S`, the `Symbol` type, `sym_eq`, `sym_of_chars`, `chars_of_sym` | carried — **decided 2026-09-14** (§8.1 rule 1) | `Symbol` is a built-in E type in every file, the interned atom, L identity `String` at phase 3; K's `Name` values are built from its atoms as today. The S-side refusal `symbol_literal` retires at slice 3.4 |
| `(list a b c)` (9,455 uses outside `v3/`) | carried — **decided 2026-09-14** (§8.1 rule 2) | the constructor chain of the `List` in scope, by the scope at Stage 0 and by the expected type at Stage 1 (R60); the S-side refusal `list_sugar` retires at slice 3.4 |
| `"…"` = UTF-8 bytes as `(List Int)`, on the extern wire too | carried in E — **decided 2026-09-14** (§8.1 rule 2); **changed** at `String`'s realization | in an E body the byte list of the `List` in scope for every file (the S-side refusal `string_literal` retires at slice 3.4); in L positions K's `String` literal (§5.3). At `String`'s E realization the E rule flips for every file at once, the toolchain migrated by tool (§11). **AT RISK:** the extern wire's byte convention under the naming law (`List UInt8`? `ByteArray`?) is undecided; at phase 2 the wire's cells are the toolchain prelude's `List`, `Option`, `Pair` and `Bool` (§6.7) |
| `Int` numerals everywhere, `-7` | carried in E — **decided 2026-09-14** (§8.1 rule 2) | in an E body a numeral is an integer of any sign whose type is the binder's; in L positions `LitNat`, negatives constructor terms (§2); a `realize` body's numeral is K's `Nat` literal, a negative one `equation_form` until Stage 1's numeral rule (law §5.2) |
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
| `inject`, `absurd` | `injection`, `noConfusion`/`absurd` | Stage 1 generates `noConfusion` (law §5.1) |
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
   the export and a union of closures re-ordered per load.
7. **Constructor citations resolve through opened prefixes only** at
   Stage 0; the `type` form opens its own namespace for the rest of
   its file so today's bare `Nil`/`Cons` keep working.
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
   and the profile's `Bool`. **Amended (GPT-6 R56, slice 10):** the
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
    toolchain prelude's `Bool`, the wire's cells are the prelude's.
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
25. **The canonical dump text and the sequential normalization** (§10
    item 1, slice 6): the bootstrap's dump converts its parallel `let`
    to the sequential indices; the measure clause is outside the text;
    names print as their last component, primitives in full.
    Alternative: the V3 printer converting to parallel indices, which
    cannot represent a right-hand side citing an earlier binder.
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
    realization obligation by terminating on one input.
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
    any exemption** (`kernel/add.shard`, slice 9; GPT-6 R49): a name
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
35. **E-only is keyed on `Init` in scope, never on the directory** (§8.1
    rule 1): a file that sees no `Init` has no L, its `type` forms are
    E only, and an L form in it is `no_init`. Alternative: a marker
    form, refused by §8's old reason (the bootstrap refuses unknown
    forms; a marker is a bootstrap change for nothing the scope does
    not already say).
36. **`Symbol` is a built-in E type in every file**, the interned atom,
    with L identity `String` assigned at phase 3 (§8.1 rule 1); K's
    `Name` values are built from its atoms as today. Alternative: a
    `Name` literal in place of symbols (R60's row), which is circular —
    K's `Name` holds the atoms — and rewrites 1,494 sites for no
    change of meaning.
37. **E's literal rules are the profile's, for every file** (§8.1 rule
    2): a numeral of any sign typed by its binder, `"…"` the byte list
    of the `List` in scope until `String`'s E realization flips it for
    every file at once, `(list …)` the `List` in scope's chain.
    Alternative: keep S's refusals (`string_literal`, `symbol_literal`,
    `list_sugar`) and give the kernel its own rules — the dialect
    again.
38. **One primitive table, both spellings** (§8.1 rule 5): the operator
    spellings are E primitives in every file beside the naming-law
    identities; the operators' L identities are assigned at phase 3
    with law §10.3's table. Alternative: rewrite the toolchain's 1,245
    operator sites to the naming-law spellings — a migration with no
    semantic content, and `/` would still need its own entry.
