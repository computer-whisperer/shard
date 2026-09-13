# The V3 language — S, L and E at Stage 0 (phase 2 draft)

> **STATUS (2026-09-12): DRAFT — slices 1–5 of phase 2 (FOUNDATION
> §12.4 item 2; `docs/records/FOUNDATION.md` §9).** Written before the
> reader existed, as the design the reader, the loader, the views, the
> fragment classifier and `ev` are built to. **Slice 2 built §2 and the
> S → L half of §4–§5:** `v3/kernel/sexpr.shard` (the s-expression
> reader, the profile's lexical rules as a flag) and
> `v3/kernel/reader.shard` (the Stage-0 reader into K's `Declaration`:
> terms, levels, binders, `structure` projections over `proj`, scope
> resolution as data for the loader), with `v3/pins/reader/` as the
> first corpus of the new tree. **Slice 3 built §3, §3.1–3.3 and §9:**
> `v3/kernel/loader.shard` (package root, modules, `import`/`use`/
> `trusts`, the `Init` prefix import against the pin, the per-file
> policy, the acceptance records) with `v3/kernel/load.shard` as the
> driver and `v3/pins/loader/` as its corpus; K's raw entry now ingests
> what it is handed (§3.3; GPT-6 R43). **Slice 4 built §6.5 as §6.6:**
> directory modules and their views — `sig type`, `sig fn` and
> `requirement` as view parameters (the policy's third class), the
> req-scope gate, the implementation checked in a fork of the loader's
> state where the view is replayed with the implementation substituted
> at each signature, `fulfills` proved or pending, one `DISCHARGE` per
> parameter. **Slice 5 built §6.2–6.4 as §6.7:** `v3/kernel/classify.shard`
> (reading a `fn` is classifying it: heads by identity and kind, saturated,
> the escape rule, exhaustiveness by the pattern matrix, the private-equality
> leak; the toolchain profile as read) and `v3/kernel/ev.shard` (`ev` as a
> machine with an explicit continuation, `run_prog` performing the host's
> externs); K's own sources load under the profile and K interpreted by `ev`
> byte-ties route 3 on the fixture (route 2). `realize` waits for slice 5b.
> It supersedes `docs/LANGUAGE.md` for the `v3/`
> tree (FOUNDATION §10.5); the old document keeps describing the old
> tree until the flip. The proof IR **I** is phase 3's chapter and is
> not here. Everything below is **Stage 0** of the law's §5.1: explicit
> L, no inference. What Stages 1–3 add is listed in §11 with its phase.
> §12 is the ledger of changes from v2 — every v2 feature with its fate,
> the AT RISK rows being the ones to watch. Decisions this draft makes
> beyond the law's text are collected in §13
> for ratification; until ratified they are the implementation's
> working assumptions, not law.

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
sources — K, `ev`, the reader — are E in the *toolchain profile* (§8)
and are read by the Rust bootstrap until frontend parity retires that
role (§10).

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
  (`name.shard`'s `Name`), so two modules can never collide in K and
  `check_name` needs no help.
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
  identifies them).
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
REFUSE M.x REASON | REFUSE M.f REASON: …           ; K's refusal; the classifier's, with its message
POLICY M.x outside the policy: a | PENDING M.x sorry | LATER realize M.f
READ-ERROR F REASON: … | LOAD-ERROR F REASON: …    ; the file's loading ends here
LOAD: modules … accepted … runnable … refused … pending … later … errors …
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
  `CheckedEnv`'s constructor, sealed when `kernel/env` has a view
  (§6.5) and the toolchain loads under it.

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
- `EvRes`: a value, out of fuel, or stuck with a reason and the
  subject.

### 6.2 `ev` — the definition of "run"

`ev : Prog → Name → List Val → Int → EvRes`. One shard `fn` in the
toolchain profile, structurally recursive on the fuel first and the
term second: entering a function body costs one unit of fuel, walking
a term costs none. **Pure**: an extern node is `EvStuck`; the
effectful driver `run` performs externs through the host's own externs
by the effect-frontier loop of the old tree's env machine
(`kernel/evm.shard` `run_app`: find the innermost stuck extern, perform
it, substitute, continue) — the explicit handler contract of law §4.7 at
phase 2 is "the toolchain's six World externs, performed in order".

Per node: a variable reads its frame slot; a literal is its value; a
constructor evaluates its arguments left to right and builds the cell;
a call evaluates its arguments, then the callee's body in a fresh frame
of exactly those values, fuel less one; a primitive applies §6.4's
table — a guard failure (division by zero for the profile's `/`, a
shift out of range) is `EvStuck`, never a value; `match` tries arms in
order and the first matching pattern binds its variables, no arm
matching is `EvStuck`; `let` evaluates its right-hand sides in order,
each in the frame the earlier bindings extended (sequential, RULED
2026-09-12 — R44); `if` evaluates its condition to a
cell and takes the **then** branch iff the cell's constructor is the
**second** constructor of its type — `Bool.true`, `Decidable.isTrue`
and the toolchain's `True` all are, which is what "a decision tag with
erased payload" means at run time: the tag decides, the payload is not
there.

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
   it; Stage 0 has no types, so a scrutinee whose type is not fixed by
   a constructor pattern is closed only by a variable arm);
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
bor bxor bshl bshr int_eq sym_eq lt le sym_of_chars chars_of_sym` and
the effectful `gen_fresh` (`docs/LANGUAGE.md` §8) — under their profile
names, plus the naming-law spellings of law §10.3 as **distinct**
identities where the meaning differs: `Int.tdiv` and `Int.tmod` total
with `x / 0 = 0` and `tmod x 0 = x`, `Int.ediv`/`Int.emod` likewise,
`Nat.sub` saturating, `Nat.land lor xor shiftLeft shiftRight` on
non-negative values. The profile's `/` staying stuck at zero and
`Int.tdiv` returning zero are two entries, not one entry with a mode;
the migration table calls that row a behavior change and this is where
the change is visible. Every entry has a positive, a negative and a
boundary case in the execution-parity suite.

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
  view parameters it rests on; linking an implementation discharges
  exactly those, and a theorem whose closure names a parameter no
  implementation discharged is reported, not accepted.

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
`CheckedEnv` becomes a `sig type` of `kernel/env`'s view and no client
outside `kernel` can build one (law §3.5).

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
  parameter then admitted, since a `fn` has no L meaning at Stage 0
  (§0); `(requirement NAME …)` discharged by the implementation's
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
  implementation check's `DISCHARGE` lines say which the implementation
  discharged and which are pending; a consumer that links an
  implementation at `run` is slice 5's. `CheckedEnv` behind
  `kernel/env`'s view waits for the toolchain to load under this
  loader (slice 6); until then the profile exposes its constructor
  (R43, `v3/README.md`).

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
4. **type parameters are static** — at Stage 0 no term carries a type,
   so this holds by construction;
5. **the private-equality leak** — the static E type of a scrutinee is
   read off the binders, the constructor field types, the callees'
   return types and the primitives' result types as far as they reach;
   a scrutinee whose static type is a `sig type` matched against any
   constructor pattern is `private_match`; a scrutinee whose static type
   is a known E inductive, `Int` or `Symbol` matched against another
   type's constructor is `pattern_type` (the matrix alone would pass it).
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

**The profile as read.** A file is in the toolchain profile iff its
root-relative path begins with `kernel/` or `meta/` (§8, §13 item 8).
A profile file has **no L**: `type` declares its E inductive only,
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
says "second constructor of its type" (§6.2's `if` rule is one bit
test), functions and externs are indices into a table, primitives are
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
continuation with the result: §6.2's frontier loop, without a search.
Fuel is spent on function entry, exhaustion is `EvOut`; the stuck
reasons are `no_arm`, `guard` (a primitive's), `if_tag` (a non-cell
condition), `extern` (under `ev`), `unlinked` (a `sig fn` no
implementation was linked to), each naming the function. The wire
(§12.6, "the extern wire's bytes"): a byte list is the toolchain
prelude's `List` of `Int` cells, an argument list its `List`, a file
read its `Option`, a pair `Pair`, a flag its `Bool` — the identities
of `kernel/prelude.shard` (§8), interned by the linker whether or not
the program declares them, so a program without the prelude in its
closure cannot match a wire cell by pattern (`if` works by the bit;
S's own story is phase 3's, §12.4); the entry's `World` argument is its
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
records and exits 2 before running; out of fuel exits 3, stuck 4, a
link failure 5, and a program that returns without `exit` exits 0. The
program's `exit` is the host's. **Route 2's byte-tie**
(`kernel/test/route2_test.sh`): the T0 driver's closure loaded from
`--root v3` under the profile and run on the fixture, its output and
exit code byte-identical to route 3's — K interpreted by `ev`, hosted
on the bootstrap.

**As built (2026-09-12).** K's own sources — `kernel/t0.shard`'s
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
names threads a counter. **Deferred to its own slice (5b):** `realize`
in both forms and R45's third test (a checked realization
distinguishable from a linked function). The supplied form's equations
(§7.2) translate an E body into L head for head, which needs the L type
of every runtime subterm — a constructor's parameters, a callee's
universe arguments — and Stage 0 has types only on binders; slice 5b
decides whether the equations are stated for bodies whose subterms'
types the binders determine, or wait for Stage 1's typing. §3.2 now
lists the `RUNNABLE` record, §13 the decisions here (items 15–19).

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
view is exact by construction: no equations are generated, the E body
*is* the erasure, and the correspondence is the classifier's erasure
rule (law §4.6), stated once.

### 7.2 The supplied body

The second form gives an E body whose **signature is the erasure of
`NAME`'s L type**, binder for binder: a `Sort`-typed binder becomes a
type parameter, a `Prop`-typed binder disappears, the rest are the E
parameters in order, and RET is the erasure of the result type. The
loader checks the correspondence positionally. The correspondence of
the body is by **equations, one per arm of the body's outermost
`match`** (or one for a body without a `match`): for the arm with
pattern `(CTOR x…)` and right-hand side `r`, the L statement

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
until phase 3's tactics.

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

## 8. The toolchain profile

The toolchain's own sources — everything under `v3/kernel`, later
`v3/meta` — are E in the profile the Rust bootstrap reads
(`docs/LANGUAGE.md` narrow; law §9.2; records §7), and the V3 reader
carries the profile so that it can re-parse them (§10). The profile is
S's E term language (§5.4) with these differences, each a reader
rule, none a semantic mode of `ev`:

| in the profile | in S |
|---|---|
| the prelude's names `Nil Cons True False Some None Z S Pair` are the toolchain's own E types (`kernel/prelude.shard`), unrelated to `Init`'s | `Init`'s `List.nil` … |
| type parameters by the parenthesized head `(fn (append T) …)` or by a bare type variable in a binder type (`(xs (List T))`), auto-bound | explicit `((T Type) …)` binders (law §5.3 departure 4) |
| `"…"` is the `(List Int)` of its UTF-8 bytes | K's `String` literal |
| numerals are `Int`; `-7` is a numeral | numerals are `Nat`; negatives are constructor terms |
| `(quote X)` and `'X` are `Symbol` literals; `sym_eq`, `sym_of_chars`, `chars_of_sym` | no symbols: a name is a `Name` constructor value |
| `(list a b c)` is list sugar | none (Stage 1 may add it) |
| a file `import` also opens the imported module (today's flat scope) | `import` never opens; `use` does — the profile files gain `use` lines when the V3 reader first reads them (records §7: "`use` lines come in phase 2") |
| the primitive names of `docs/LANGUAGE.md` §8 | the naming-law spellings, same table (§6.4) |
| a `type` is E only; there is no L in the profile — `def`, `theorem`, `inductive` are refused (`profile_form`) | a `type` enters K and E (§4) |
| the profile is the `kernel/` and `meta/` directories of the root (slice 5; §13 item 8) | every other file is S |

Which files are in the profile is decided by the loader from the
package layout (the toolchain directories, named in `v3/README.md`'s
layout), not by a marker form: the Rust loader refuses any top-level
form it does not know, so a `(profile toolchain)` marker would need a
one-word bootstrap change; the parity slice (§10) may still prefer the
marker. A ratification item (§13).

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

## 10. Conformance at phase 2 (records §4.1 B10)

Four suites, agreed before any result is read:

1. **Frontend parity.** The V3 reader over `v3/kernel/*.shard` and the
   Rust loader over the same files produce the same `Prog`, compared as
   one canonical text: the reader prints its `Prog`; the bootstrap
   gains a `dump` mode that prints its `Module` in the same text.
   Byte-identical over the toolchain's closure retires TCB bring-up
   item (2), the Rust loader's parsing role.
2. **Execution parity.** The same `Prog` under `ev` (hosted on route 3)
   and under the Rust evaluator: K's test entrypoints and the T0
   fixture — route 2, K interpreted by `ev` — with byte-identical
   verdict lines — **the fixture's tie landed at slice 5**
   (`kernel/test/route2_test.sh`, §6.7); `examples/calc`'s program half under `ev` against the
   old tree's evaluator on a fixed input set; every primitive's
   positive, negative and boundary cases.
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
| the `Init` import with E realizations attached; `String`, `Array`, `ByteArray` representations | §4.4, INVENTORY | 3 |
| lambda lifting, templates, specialization | §4.3 | 3–4 |
| `bin`, `requires`, the World-use check, effect traces | §4.7 | 4 |
| prepared handles, long-lived environments | §9.3, T6 | 4 |
| evaluation reflection: `ev`'s theorem, the `rfl` node | §4.4, T8 | 4 |
| the canonical S form: CANON's rule set rewritten for S | §5.1 "One canonical S", §10.5 | 2, its own slice |
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
| `(type (NAME T…) (CTOR F…)…)` | carried | §4; constructors gain the identity `NAME.CTOR`; a bare constructor citation needs the declaring file or a `use` (§13 item 7) |
| `(fn NAME PARAMS RET BODY)` | carried as E; **changed** | §0: no L meaning at phase 2 — in v2 a `fn` was also the object of `unfold`/`simp` in proofs; restored at phase 3 |
| polymorphic head `(fn (append T) …)`; bare type variables in binders auto-bound (`(xs (List T))`) | re-spelled in S; carried in the profile | S: explicit `((T Type) …)` binders — law §5.3 departure (4), "auto-bound implicits → explicit binders"; the profile keeps both v2 spellings (§8) |
| `(extern NAME PARAMS RET)`, polymorphic externs | carried | §4; the roster is the host's (§12.4) |
| return types unchecked (no load-time typing) | **changed at phase 3** | Stage 1 types every `fn` body against its signature; v2 code that runs only because nothing checked it will be refused then. At phase 2 unchanged |
| `if` on `True`/`False` by constructor name | carried, generalized | §6.2's tag rule; v2's `(type Bool (False) (True))` has Init's constructor order |
| `match`: first match wins, nested patterns, integer and `(quote S)` patterns, `_`, bare 0-ary constructors | carried in E | symbol patterns profile only; Stage 1's match compilation must keep first-match semantics (Lean's does) |
| parallel `let`, no `let*` | **changed**: sequential in L and E (RULED 2026-09-12, R44) | §5.4; 0 of the tree's 30,611 `let` groups depend on parallel binding, so no source changes meaning; the bootstrap evaluator's parallel rule gives identical results on all of them until the V3 reader replaces it (slice 2) |
| `(quote S)`, `'S`, the `Symbol` type, `sym_eq`, `sym_of_chars`, `chars_of_sym` | profile only; **AT RISK in S** — refused by name in S since slice 5 (`symbol_literal`) | S has no symbol type: a name is a `Name` value. The toolchain (ten kernel files) and the tools use symbols as tags and identifiers; whether V3 source gets a `Name` literal is a phase-3 decision |
| `(list a b c)` (9,455 uses outside `v3/`) | profile only; **AT RISK in S** — refused by name in S since slice 5 (`list_sugar`) | Lean's `[a, b, c]` is elaborator sugar; Stage 1 should add a list literal or every ported `fn` spells `cons` chains |
| `"…"` = UTF-8 bytes as `(List Int)`, on the extern wire too | **changed** in S | K's `String` literal, its E realization phase 3 (§5.3) — in an S E body refused since slice 5 (`string_literal`). **AT RISK:** the extern wire's byte convention under the naming law (`List UInt8`? `ByteArray`?) is undecided; the profile keeps bytes, and at phase 2 the wire's cells are the toolchain prelude's `List`, `Option`, `Pair` and `Bool` (§6.7) |
| `Int` numerals everywhere, `-7` | **changed** in S | numerals are `Nat`; negatives are constructor terms at Stage 0 (§2); the numeral rule of law §5.2 is Stage 1 and `-7` as `Neg.neg` is a Stage-3 instance — **AT RISK:** negative literals stay verbose until then unless Stage 1 special-cases `-` |
| unbound identifier = `FVar` (proof-time opened variables) | dropped | an unbound name is a resolution error; K refuses free variables; I's named context replaces the use (phase 3) |
| primitive dispatch by name, trie-first, bodyless-name collision = stuck (`pins/lang/prim_shadow_rejects`) | **changed** — landed slice 5 | heads classified at load into four node kinds; a primitive is an identity (§6.4); a declared name shadows the table's (the scope resolves first); an unknown head is refused at load |
| the primitive table | carried under the profile names; **changed** under the naming-law names | §6.4; `/` stuck at zero versus `Int.tdiv` total are two entries |
| `gen_fresh`, the one effectful primitive | **dropped** (slice 5) | law §4.7 has no effectful primitives: pure `ev` refuses reachable externs. No V3 file calls it and the table does not carry it; a ported source that needs fresh names threads a counter (ten old-tree kernel files: types, canon, sequent, reduce, tactics, proof_reader, proof, trace, the two lowered twins) |
| `Nat` former: `Z`/`S` packed to literals, patterns match literals by view, proof-facing normalizers never pack, bare literals do not type as `Nat` | **changed** | `Nat` is Init's; K's literal rules (offset, `Nat.zero` ≡ `0`, accelerators) replace the former; numerals type as `Nat` by rule (§5.3). The profile keeps the prelude's `Nat` for measures; `ev` does not carry the bootstrap's `Z`/`S` view over integer literals (no V3 file matches on them; a literal matches `Z` never) |
| `(refine BASE PRED)`, `refine_val`, `refine_try`, `refine-fact`, `(returns …)` (37 types, 43 `refine-fact` sites) | re-spelled; deferred | `Subtype` over any `Prop` (law §4.1): `refine_val` → `Subtype.val`, `refine_try` → a `decide`-bridged constructor, `refine-fact` → `Subtype.property`; the return obligation is a Stage-1 elaboration obligation. Phase 3 (REFINEMENT.md superseded) |
| `(record …)`, `make`, `with`, `F_of`/`with_F`, the six-law family, `NAME_eta` (21 files, 237 `with_F` sites) | re-spelled; **AT RISK** | `structure` with projections (§4): `F_of` → `NAME.F`; `NAME_eta` is K's structure eta for free; the laws are `rfl`. **No Stage-0 form for `with_F` updaters or order-free `make`**: Lean's `{ s with f := v }` is elaborator sugar — Stage 1 must add it or every update site spells the constructor |
| `std/word` (`U8`…`I32`, opaque), `std/bytes`, `std/str` | re-spelled | `UInt*`/`BitVec`, `ByteArray`, `String` (law §10.3, INVENTORY), phase 3–5. **AT RISK:** widths beyond 64 — INVENTORY realizes `BitVec w` only for static `w ≤ 64`; the 2026-09-02 ruling kept `std/word`'s unused widths as a facility |
| `S^`, `inline`, `chain` (2,733 / 1,851 / 320 uses) | dropped with the v2 proof language | their reason — claim statements must be literal spellings that match CBV residues — does not survive: I's `rw`/`simp_only` match terms, and Nat towers are K literals. Phase 3 confirms; **AT RISK** if some I form still needs a literal tower |
| `(import "x.shard")` opens the file's names (flat scope) | **changed** | `import` never opens, `use` does (§3.1); the profile keeps flat scope until its `use` lines are added (slice 6) |
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
| evaluator errors `NoMatchArm`, `IfNonBool`, `UnknownCall`; no fuel | **changed** — landed slice 5 | `EvStuck` with a reason (`no_arm`, `if_tag`, `guard`, `extern`, `unlinked`) and the function; `EvOut` is new — v2 relied on the totality gate instead of fuel (§6.2); an unknown call is refused at load, never reached |
| `(measure (struct x))` (3,175), `(measure (- …))` (29), size-function measures (about 20) | carried as `ERec` | the obligation: v2's measure gate and the offline `admit` classifier on every checked `fn` → law §4.5's tactic-discharged obligations (phase 3). **AT RISK during phase 2:** no totality check on any `fn` — exactly `eval direct`'s situation today, and only for the duration of Stage 0 |
| mutual recursion (the measure gate's SCCs) | deferred | "mutually recursive groups need a joint well-founded argument" (law §4.4), Stage 1 |
| a user `(type Nat …)` shadows the core one | carried by identity | two `Nat`s are two identities; no shadowing rule is needed |

### 12.5 Tooling surfaces

| v2 | fate | note |
|---|---|---|
| `bin/check`, `bin/shard_check`, `bin/shard_eval`, `eval direct` | the V3 driver (`kernel/load.shard`, slice 3) replaces `check`; `eval direct` stays the bootstrap's; the compiled chain is route 1 | law §9.1; `tools/lower` ARCHIVE at the flip |
| shardfmt and CANON's rule set | rewritten for S | law §10.5: phase 2, its own slice; the fmt gate on V3 at phase 6 |
| `tools/digest`, `explain`, `prove`, `search` | new code onto law §7.3 | MANIFEST |
| `tools/zed-shard`, `shard-viewer` keyword lists | the V3 keywords (§4) | law §10.5, phase 2 |

### 12.6 The AT RISK rows: owner, consumer, regression (R47, 2026-09-12)

Each row above marked AT RISK, with who decides it, which consumer
first needs it, the small regression that shows the gap or the fix,
and the disposition this draft intends.

| row | owner | first consumer | regression | intended disposition |
|---|---|---|---|---|
| symbols in S (`quote`, `Symbol`, `sym_eq`) | phase 3, Stage 1 (the reader) | the toolchain's own sources (ten kernel files) when they port to L | a `fn` using `(quote x)` and `sym_eq` under the V3 reader is refused with a named reason until decided | a `Name` literal, or symbols stay profile-only |
| `(list a b c)` | phase 3, Stage 1 | every ported `fn` (9,455 sites); calc's program half runs under the profile at slice 6 | `(list 1 2)` under the V3 reader refused by name | a list literal at Stage 1 |
| the extern wire's bytes | slice 5 (`ev`'s extern boundary — **bytes as the prelude's cells, landed**); phase 3 for the L type | `sha256sum`'s bin; calc's app step | `write_line` of a literal round-trips its bytes through the driver (route 2's byte-tie) | bytes at phase 2; `ByteArray` or `List UInt8` decided with §5.3's `String` realization |
| negative numerals | phase 3, Stage 1 | calc (negative `Int` results); `std/div` | `-7` under the V3 reader = the constructor term at Stage 0 | Stage 1 special-cases `-` on a numeral; ruled then |
| `gen_fresh` | **decided slice 5: dropped** | the ten old-tree kernel files (canon, tactics) as they port | a `fn` citing `gen_fresh` is refused at load (`unknown_head`) | a threaded counter in the ported toolchain |
| `with_F` updaters, order-free `make` | phase 3, Stage 1 | 237 sites (`models/imp`, the tools) | `(with_F s v)` refused by name until the update form exists | Stage 1 record-update sugar (Lean's `{ s with f := v }`) |
| `std/word` widths beyond 64 | phase 5 (the word/float line) | none today (the 2026-09-02 ruling kept the widths as a facility) | INVENTORY's static `w ≤ 64` bound on `BitVec w` | static `w ≤ 64` unless a consumer appears |
| `S^`, `inline`, `chain` | phase 3 (I) | the PORT theorem corpus | a claim whose statement needs a literal tower under I's `rw` | dropped; phase 3 confirms |
| `(lib …)` | phase 5 | `tools/lowcheck`'s fixtures (4 uses) | the fixtures under the profile | decided with the lowering-side toolchain |
| `subterm-induct`/`(below)`, `fin-split` | phase 3 (I steps) | the regenerated certificate kits; the std proofs that use them | one theorem each: `tb_len`'s strong induction; one bounded enumeration | `wf` over `sizeOf` and `decide`/`omega`; a subterm rule only if a ported proof needs it |
| no totality check on any `fn` during Stage 0 | phase 3, Stage 1 (law §4.5) | every `fn`; calc's program half at slice 6 | R45's self-recursive candidate exhausts and gains no equations (`ev_test`, landed slice 5) | measure obligations discharged at Stage 1; the runnable-only status visible in the driver's output meanwhile (`RUNNABLE`, R45, landed) |

## 13. For ratification — decisions made here beyond the law's text

1. **Native K names carry the module path** (`std.list.List.sum`);
   imported names do not (`List.length`), the import being their
   identity. Alternative: prefix imports too (`Init.List.length` in K),
   which renames every constant in every imported term at import time
   and changes the accelerator pins' identity hashes.
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
   of a marker form (§8).
9. **`ev`'s `if` rule**: the then-branch iff the condition's cell is the
   second constructor of its type — one rule for `Bool`, `Decidable`
   and the profile's `Bool`.
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
    lookup, which K's one-name environment cannot carry.
13. **A view's theorem is not re-checked in the fork**: checked once
    against the parameters, it is bound by its closure; the fork
    discharges parameters, not theorems.
14. **The implementation is `DIR/BASE.shard`** (the old tree's rule),
    not every file in the directory; `mod.req/` siblings wait for a
    consumer.
15. **Reading a `fn` is classifying it** (§6.7, slice 5): heads
    resolve through one suffix table over pre-registered file heads,
    so forward references and mutual recursion work as today; a
    declared name shadows a primitive's; exhaustiveness is the pattern
    matrix; the leak check reads static types as far as the
    declarations determine them. Alternative: a separate pass after
    reading, which would re-walk every body.
16. **The profile is the `kernel/` and `meta/` directories**, E only,
    flat scope — every visible declaration by any suffix, no `use`.
    Alternative: `use` lines in the toolchain's own files, which the
    flat rule makes unnecessary at Stage 0 (records §7 expected them).
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
19. **`gen_fresh` is dropped**; `realize` (both forms, §7) and R45's
    third test move to slice 5b with the E→L translation question
    (§6.7).
