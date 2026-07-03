# Narrow shard

> The language is **shard**. This document specifies **narrow shard** —
> the deliberately restricted subset that the disposable Rust bootstrap
> parses and evaluates, and the form the kernel, engine, parser, and
> tools are themselves written in. The richer **full shard** (where
> language features actually grow) is implemented *by* the shard engine
> on top of this floor; see §11 for the relationship.

Concentrated reference for narrow shard as currently implemented —
restricted enough to host a small, trusted Rust evaluator over it,
expressive enough that the proof checker kernel is written *in* this
form (see TRANSFER.md).

This document describes only what the v2 loader and evaluator
actually accept and run today. Things planned but not implemented are
flagged `[future]`. Things deliberately deferred are flagged
`[REVISIT]` and discussed in `docs/REVISIT.md`.


## 1. Lexical syntax

The on-disk format is s-expressions, parsed by the `lexpr` crate.

- **Whitespace** (spaces, tabs, newlines) separates tokens; otherwise
  insignificant.
- **Atoms** are integers (`42`, `-7`) or symbols (`foo`, `+`, `bind_x`,
  `do_induct`, `PVar`). Symbols admit a wide character class; in
  practice the kernel uses `[A-Za-z_][A-Za-z0-9_]*` plus the operator
  symbols `+ - * / < > = ?`.
- **Lists** are parenthesized sequences: `(head arg arg …)`.
- **Comments** begin with `;` and run to end of line. Convention:
  - `;` — trailing inline comments
  - `;;` — regular line comments
  - `;;;` — file-header / section docstrings


## 2. File and module structure

A file is a sequence of top-level forms. Forms may appear in any
order; the loader does two passes (type and ctor names first, bodies
second), so forward references across files are fine. Cross-file
loading concatenates the file list in load order.

A file may also carry `(import "path")` declarations naming the files it
depends on; these drive dependency tracking and load order — the loader
itself ignores them when collecting types/fns, since the assembled set is
still concatenated as above. A directory-based module system with
interface/visibility rules is layered on top of this floor
(`mod.req.shard` interface files, expandable Rust-style into a
`mod.req/` directory whose entry keeps the name; MODE-AWARE resolution —
proof checking sees a module's interface, running code gets the impl
bodies; a loader gate that rejects reaching past another module's
interface; and a req-scope gate — an interface file may import only
other req-scope files, bare module interfaces, and the kernel, never an
implementation file); that system is beyond the scope of this document.

Four top-level definitional forms:

```
(type        NAME-OR-PARAMETERIZED  CTORDEF …)
(fn          NAME ((P TYPE) …) RET-TYPE  BODY-EXPR)
(extern      NAME ((P TYPE) …) RET-TYPE)
(sig fn      NAME ((P TYPE) …) RET-TYPE)   ; bodyless signature — opaque in
                                           ;   proofs (stuck like an extern);
                                           ;   an impl body may shadow it.
                                           ;   (sig type …) likewise declares
                                           ;   an opaque type (private ctors).
```

### 2.1 `type` — algebraic data types

Non-parametric:
```sexp
(type Bool
  (False)
  (True))
```

Parametric — the head is itself a list `(NAME TYPEVAR …)`:
```sexp
(type (List T)
  (Nil)
  (Cons T (List T)))
```

Each ctor declaration is `(NAME FIELD-TYPE …)`. A zero-field ctor is
`(NAME)`. Type parameters are erased at runtime [REVISIT — Erased
polymorphism in narrow].

### 2.2 `fn` — user-defined function

Monomorphic:
```sexp
(fn add ((a Int) (b Int)) Int
  (+ a b))
```

Polymorphic (slice 31) — head is a list `(NAME TPARAM …)`, mirroring
the `type` syntax. Bare TPARAM symbols inside parameter or return
types become `TVar`s:
```sexp
(fn (append T) ((xs (List T)) (ys (List T))) (List T)
  (match xs
    (Nil          ys)
    ((Cons h t)   (Cons h (append t ys)))))
```

Parameter list is `((NAME TYPE) …)`; can be empty. The body sees
parameters as locally-bound (see §7). Return type is for
documentation in narrow — the v2 evaluator does no type checking
[REVISIT — Trusted-by-review Rust component].

### 2.3 `extern` — foreign function declaration

```sexp
(extern wall_clock_ns () Int)
```

Same parameterized-head form for polymorphic externs:
```sexp
(extern (read_at T) ((p Path) (n Int)) (Option T))
```

A signature with no body. The evaluator treats calls to extern
symbols as stuck; the Rust runtime is responsible for intercepting
and dispatching them. See `docs/BOUNDARIES.md`.


## 3. Types

The `Type` grammar (as values internally, and as written in
parameter/return positions):

| form                | meaning                              |
|---------------------|--------------------------------------|
| `Int`               | arbitrary-precision integer (`BigInt` in the bootstrap) |
| `Symbol`            | interned identifier                  |
| `Bool`              | user-defined (in stdlib), `True`/`False` ctor |
| `Nat`               | Peano naturals, literal-backed (see "Nat" below) |
| `BareName`          | reference to a declared `type`       |
| `(TyCon T1 T2 …)`   | type application                     |
| `(refine BASE PRED)` | refinement of `BASE` by a `Bool` predicate (see "Refined types" below) |

Examples: `(List Int)`, `(Option (Pair Symbol Expr))`,
`(Map Symbol Type)`.

`Int` and `Symbol` are primitive; `Bool` is part of the stdlib.

### Nat — the literal-backed Peano type

```sexp
(type Nat (Z) (S Nat))       ; declared in kernel/stdlib.shard
```

`Nat` is declared as an ordinary inductive in the stdlib, but the
kernel gives its **ground** values a packed representation (the Nat
former, 2026-07-03):

- **The ground normal form is a nonneg `Int` literal.**
  Evaluation-grade reduction (`compute`, the RUN engines) *packs*
  ground constructions: `Z` computes to `0`, `(S lit)` to `lit+1` —
  so `(S (S Z))` evaluates to `2`, and a `10^6`-deep fuel value is
  one literal, never a million-cell ctor tree.
- **`Z`/`S` patterns match literals by view.** A literal scrutinee
  `0` matches `Z`; `n ≥ 1` matches `(S p)` with `p` matched against
  the literal `n-1`, recursing — deep patterns like `(S (S k))`
  work. A negative literal in `Nat` position is *stuck*, never a
  match failure.
- **Symbolic terms are ordinary ctors.** `(S x)` with `x` symbolic
  reduces and sticks exactly like a user ctor: `induct` over `Nat`
  works, and `S`-towers in claim statements survive verbatim.
- **Proof-facing normalizers never pack.** `(reduce …)` and
  `(simp …)` fire matches and prims but leave ground `Z`/`S`
  spellings alone — packing is computation, `compute` territory.
  This is what keeps a goal's `Z`/`S` spelling matching your IHs and
  lemma statements; pinned by `examples/nat_prim.shard`
  (`nat_iota_no_pack` / `nat_simp_no_pack`).
- **Bare `Int` literals do not (yet) type as `Nat`**: `(fn f () Nat 3)`
  is a type error. Construct through `Z`/`S` (or a converter fn);
  the literal typing rule is a designed follow-up.

Arithmetic over `Nat` (`add_nat`, `int_of_nat`, `half_nat`, and their
lemma family) lives in `std/nat`; the kernel supplies only the type,
the packing, and the views. A local user `(type Nat …)` shadows the
core one (resolution is local > import > core) and gets no special
treatment.

### Words — opaque `std/word` constructions

Fixed-width modular integers are **not** a kernel type. The
`(Word W S)` kernel former was revoked (trusted-core contraction,
issue #15); `std/word` supplies opaque per-width types instead:
`U8`/`U16`/`U32` and `I8`/`I16`/`I32` behind `sig type`, constructed
only through the makers — `(u8 n)` stores `n mod 2^8`; `(i8 n)` the
shifted-mod image in `[-2^7, 2^7)` — and observed through
`u8_val`/`i8_val`/…. Each op (`u8_add`, `u8_and`, `u8_shl`, `u8_lt`,
…) carries a val-image law in the interface
(e.g. `u8_add_val : (u8_val (u8_add a b)) = (mod (+ (u8_val a)
(u8_val b)) 256)`); consumers reason exclusively through those
requirements. There are no word primitives in any kernel table.
`u64`/`i64` are deferred on the compiled dev engine's i63 debt.

### Bytes — an opaque `std/bytes` construction

`Bytes` is **not** a kernel type. It is an **opaque `std/bytes` module**
over `(List U8)` (`std/word`'s unsigned byte) — the trusted-core
contraction (issue #15), the same move that revoked the `Word` former.
A consumer sees an abstract `Bytes` with the packed-style ops; the
`(List U8)` representation is hidden behind the `sig type`. The kernel
has no `Bytes` former, no `bytes` primitives, and no `bytes-fact` proof
step — `std/bytes` rests on `std/list` + `std/word` (→ `std/div`'s 2
euclidean axioms), with **no bytes-specific axiom**.

The ops (`std/bytes/mod.req.shard`), all **total**:

- `(bytes_of_list L)` — the maker: `(List Int) → Bytes`, each element
  masked mod 256 (it maps `u8` over the list).
- `(list_of_bytes b)` — the model projection: `Bytes → (List Int)`.
  On a `bytes_ok` list, `(list_of_bytes (bytes_of_list L)) = L`
  (`of_list_id`, a theorem).
- `(blen b)` — the length (`= len (list_of_bytes b)`, `blen_is_len`).
- `(bidx b i)` — element `i`, re-canonicalized through `u8`; **0 outside
  `[0, blen b)`**, and `0 ≤ (bidx b i) < 256` unconditionally
  (`idx_lo`/`idx_hi`).
- `(bcat a b)` — concatenation (`list_of_cat`, `cat_len`).
- `(bslice b i j)` — the clamped window `[max 0 i, min (blen b) j)`;
  exact length `j - i` on a valid window (`slice_len_exact`).

The list-model bridge laws (the `len`/`append` homomorphisms, the
guarded round trip, the exact slice length) are all **theorems** proven
in `std/bytes/bytes.shard`. A consumer reasons about a `Bytes` only
through these laws and the `std/list` vocabulary on `(list_of_bytes b)`.

### Refined types

```sexp
(type NAME (refine BASE PRED))   ; e.g. (type Small (refine Int is_small))
```

declares `NAME` as `BASE` restricted by `PRED`, a total, already-defined
fn `BASE → Bool`. Full treatment: `docs/REFINEMENT.md`. The surface:

- **Intro is an obligation.** A fn whose *return type* is a refinement
  is admitted only with the proof obligation
  `∀ args, (= (PRED (refine_val (f args))) True)` — the body is a bare
  `BASE` value, the checker demands the predicate.
- **`(refine_val s)`** — the projection `NAME → BASE`. Identity at
  runtime; in a goal it marks where the refined value is being read at
  base type.
- **`(refine_try NAME e)`** — the decidable downcast,
  `BASE → (Option NAME)`: `Some` iff `PRED` holds. The I/O-boundary
  validator idiom (`utf8_decode b = (refine_try Str b)` in `std/str`).
- **`(refine-fact NAME TERM PROOF)`** — the proof form that
  materializes `(= (PRED (refine_val TERM)) True)` as a premise for
  `PROOF` (a cut, like `have`): how a consumer *recovers* the invariant
  a refined value carries.

`std/str` is the worked example: `(type Str (refine Bytes utf8_valid))`,
an opaque module whose interface exports the validity recovery as a lemma.

### Records — named-field products

```lisp
(record NAME (ctor CTOR)? (FIELD TYPE)+)
```

is **loader-level sugar** (expanded at the s-expr level, before the
collector passes, identically for check and run — the kernel never
sees a record form, and `load.rs` rejects the head, so kernel files
stay hand-positional). One record form generates:

- the positional single-ctor type `(type NAME (CTOR T…))` — `CTOR`
  defaults to `MkNAME`; `(ctor GS)` as the first entry overrides it
  (`ctor` is therefore reserved as a field name);
- an accessor `FIELD_of : NAME → T` per field;
- an updater `with_FIELD : T → NAME → NAME` per field (value first);
- the **law family**, ordinary claims with machine proofs, named
  mechanically so they can be cited without lookup:

  | law                | statement                                     |
  |--------------------|-----------------------------------------------|
  | `FIELD_of_def`     | `(FIELD_of (CTOR f…)) = FIELD`                |
  | `with_FIELD_def`   | `(with_FIELD v (CTOR f…)) = (CTOR …v…)`       |
  | `F_of_with_F`      | select-over-update, same field (`= v`)        |
  | `G_of_with_F`      | select-over-update, cross (`= (G_of r)`)      |
  | `with_F_with_F`    | update-over-update collapse                   |
  | `NAME_eta`         | `r = (CTOR (F0_of r) … (Fn_of r))` — use this |
  |                    | instead of `case-on` over the record          |

Construction and update sugar (rewritten everywhere in the file —
bodies, goals, proofs; nested values first):

```lisp
(make NAME (FIELD V)…)   ; → (CTOR V0 … Vn) — named, order-free;
                         ;   ALL fields required exactly once
(with E (FIELD V)…)      ; → (with_FIELDn Vn (… (with_FIELD0 V0 E)))
                         ;   later entry outermost
```

`with` is purely syntactic (it only manufactures updater names), so it
works on records defined in other files; `make` needs the field order
and resolves against the **current file's** records only. Duplicate or
missing fields refuse the file loudly. A proof that touches the record
only through the accessors/updaters and the law family is **textually
invariant under field addition** — adding a field changes the record
form and each `make` site (one new entry), nothing else. Pilot:
`GameState` in `examples/snake_game_3/game/game.shard`; shape pin:
`examples/record_proto.shard`.


## 4. Expressions

Every expression at runtime evaluates to a value (also an `Expr`,
in normal form). The evaluator is call-by-value.

### 4.1 Literals

- **Integer literal**: `42`, `-7`, `0`. Parses to `IntLit`.
- **Symbol literal**: `(quote foo)`. Parses to `SymLit`. The unquoted
  form `foo` is treated as a variable reference, not a symbol value.
- **String literal**: `"x+y"`. Sugar for the `(List Int)` of its
  **UTF-8 bytes** — `(Cons 120 (Cons 43 (Cons 121 Nil)))` (for ASCII,
  bytes and codepoints coincide). `String ≡ (List Int)`; there is no
  distinct string type, so the `std/list` lemma library applies to
  strings unchanged. Valid only in expression position (match against
  strings by destructuring Cons/Nil).

  **One meaning end to end (issue #2 Phase 3).** Text-shaped
  `(List Int)` values are **byte sequences** everywhere: string
  literals, the extern wire (`get_args`/`read_file`/`write`/
  `write_file`/`write_line` — the host performs no encoding or
  decoding; reads are binary-safe, writes emit the list verbatim with
  elements masked mod 256 like `bytes_of_list`), `read_key`'s single
  byte, the `sym_of_chars`/`chars_of_sym` bridge (a symbol's name as
  its UTF-8 bytes), and the compiled chain's `rt.h`. Per-character
  work on non-ASCII text needs an explicit UTF-8 decode in-language;
  the opaque `std/bytes` construction (§3) is the typed companion.

### 4.2 Variables

A bare identifier resolves, in order:
1. **Local binding** — if a binder above (fn parameter, pattern
   variable, let binding) introduced this name, it becomes a `BVar`
   with the appropriate de Bruijn index (see §7).
2. **Zero-arity ctor** — if the identifier matches a ctor declared
   with no fields (e.g. `Nil`, `True`, `Empty`), it becomes a
   `Ctor NAME ()`.
3. **Free variable** — otherwise, `FVar NAME`. Used during proof
   checking to represent opened ∀-bound variables.

### 4.3 Constructor application

```sexp
(CTORNAME arg-expr …)
```

If `CTORNAME` was declared as a constructor (in any loaded type), the
form builds a `Ctor` value. Arity must match the declaration. Zero-arg
ctors may also be written bare (e.g. `Nil` ≡ `(Nil)`).

### 4.4 Function call

```sexp
(FNNAME arg-expr …)
```

Any list head that isn't a ctor, a special form, or a primitive is
treated as a function call. The evaluator looks up the fn in the
module; unknown calls are tried as primitives; if neither matches,
the call is *stuck* (`EvalError::UnknownCall`).

### 4.5 `if`

```sexp
(if COND THEN ELSE)
```

`COND` must reduce to `(True)` or `(False)` — the specific ctor names
are hardcoded [REVISIT — Primitive comparisons return user Bool]. Any
other ctor (or non-ctor value) at the head produces an `IfNonBool`
error.

### 4.6 `match`

```sexp
(match SCRUT
  (PAT-1 BODY-1)
  (PAT-2 BODY-2)
  …)
```

Arms are tried in source order; first match wins. The matched
pattern's bindings are introduced into the body (see §5). No
fall-through; failure to match any arm is an `EvalError::NoMatchArm`.

### 4.7 `let` — parallel bindings

```sexp
(let ((N1 E1)
      (N2 E2)
      …)
  BODY)
```

All RHSs (`E1`, `E2`, …) are evaluated in the *outer* scope; the
resulting values are bound simultaneously in `BODY` [REVISIT —
Parallel let only]. There is no sequential `let*`.

### 4.8 `quote`

```sexp
(quote SYMBOL)
```

The only form that produces a `SymLit` value. There is no general
quotation of expressions — `quote` exists solely to write symbol
constants.


## 5. Patterns

A `match` arm's pattern is one of:

| form                       | name    | meaning                                |
|----------------------------|---------|----------------------------------------|
| `IDENT` (not a ctor)       | `PVar`  | bind any value to a fresh local        |
| `_`                        | `PVar`  | conventional ignored binding           |
| `(CTORNAME SUB-PAT …)`     | `PCtor` | match ctor with this name, recurse     |
| `BareCtorName`             | `PCtor` | shorthand for `(CTORNAME)` — 0-ary    |
| `42`                       | `PInt`  | match an `IntLit` of this value        |
| `(quote SYM)`              | `PSym`  | match a `SymLit` of this name          |

PVars bind in source order; the LAST PVar in the pattern becomes
`BVar 0` in the arm body (see §7).


## 6. Bool encoding

The stdlib defines `Bool` as an ordinary user type:

```sexp
(type Bool
  (False)
  (True))
```

By convention:

- Primitive comparison operations (`int_eq`, `sym_eq`, `lt`, `le`)
  return `(True)` or `(False)` — these specific ctor names are
  hardcoded in `src/prim.rs` and in `step`'s `If` arm.
- The `if` expression dispatches on these two ctor names.

This couples the runtime to the stdlib's Bool definition. A future
module-header directive will let the names be configurable
[REVISIT — Primitive comparisons return user Bool].


## 7. Binding conventions

### Locally-nameless

Free variables carry names (`FVar Symbol`); bound variables are
de Bruijn indices (`BVar Int`). Substitution does not need
α-renaming; capture-avoidance is structural.

### Innermost-first

The most recently introduced binder is `BVar 0`. Higher indices
refer to outer binders.

- For a `(fn f ((a T) (b T)) … BODY)`, `BODY` sees `b` as `BVar 0`
  and `a` as `BVar 1`.
- For a pattern `(Cons head tail)`, the body sees `tail` as `BVar 0`
  and `head` as `BVar 1`.
- For `(let ((x A) (y B) (z C)) BODY)`, `BODY` sees `z` as `BVar 0`,
  `y` as `BVar 1`, `x` as `BVar 2`.

`open_many` expects bindings in innermost-first order:
`bindings[0]` fills `BVar 0`.

[REVISIT — Pattern binding order: innermost-first]


## 8. Primitives

Provided by the trusted Rust runtime, exposed as ordinary function
symbols. Calls remain stuck in narrow until the runtime intercepts
them ("stuck-and-intercept" — see REVISIT).

| name         | signature                  | notes                       |
|--------------|----------------------------|-----------------------------|
| `+ - * /`    | `Int × Int → Int`          | `/` truncates toward zero; rejects div-by-zero |
| `mod`        | `Int × Int → Int`          | Euclidean (result ≥ 0)      |
| `tmod`       | `Int × Int → Int`          | truncating remainder — pairs with `/` |
| `ediv`       | `Int × Int → Int`          | Euclidean quotient — pairs with `mod` |
| `band`       | `Int × Int → Int`          | bitwise AND                 |
| `bor`        | `Int × Int → Int`          | bitwise OR                  |
| `bxor`       | `Int × Int → Int`          | bitwise XOR                 |
| `bshl`       | `Int × Int → Int`          | shift by < 64               |
| `bshr`       | `Int × Int → Int`          | shift by < 64               |
| `int_eq`     | `Int × Int → Bool`         |                             |
| `sym_eq`     | `Symbol × Symbol → Bool`   |                             |
| `lt`         | `Int × Int → Bool`         |                             |
| `le`         | `Int × Int → Bool`         |                             |
| `gen_fresh`  | `() → Symbol`              | effectful; unique per call  |

`gen_fresh` is the lone effectful primitive [REVISIT — Fresh-symbol
generation as an effectful primitive].

The `Word` and `Bytes` formers were both revoked (trusted-core
contraction, issue #15): there are no `wadd …` or `blen`/`bidx`/… byte
primitives in any table. Fixed-width modular ints and byte sequences are
now opaque `std/word` / `std/bytes` constructions (§3).


## 9. Stdlib types

Defined in `kernel/stdlib.shard`:

```sexp
(type (List T)   (Nil) (Cons T (List T)))
(type (Option T) (None) (Some T))
(type Bool       (False) (True))
(type (Pair A B) (Pair A B))
(type Nat        (Z) (S Nat))
```

Used throughout the kernel; no privileged status — just user types
that happen to be ubiquitous. The one exception is `Nat`, whose
**ground** values the kernel packs to `Int` literals and matches by
view (§3, "Nat"); its symbolic behavior is that of an ordinary
inductive.


## 10. The proof language

> **A distinct language.** Everything above specifies the *object*
> language — the total, pure, first-order form the kernel reasons
> *about*. The **proof language** is a separate language with its own
> grammar; it carries none of the object language's constraints (it is
> not total, not the thing being compiled) and grows whatever forms the
> checking task needs. It is documented here, in the same file, only for
> convenience. The two-languages split is the central design point — see
> `docs/archive/TRANSFER.md`.
>
> A proof s-expression is parsed **straight to the native
> `Goal`/`Proof`/`Step` ADTs** the kernel consumes
> (`kernel/proof_reader.shard`) — there is no reflect-as-Expr-then-
> un-reflect roundtrip. The proof language *embeds* object-language
> snippets (the terms an equation relates, a rewrite's instantiation
> terms, a measure); those snippets — and only those — are parsed by the
> object reader's `elaborate`, against the module's constructor set, so a
> binder name that isn't a constructor becomes a free variable (`FVar`).

### 10.1 Top-level proof declarations

These four forms are recognized in source order by
`kernel/reader.shard::collect_decls` (the front-end, in-language — *not*
by the Rust harness). The driver (`kernel/driver.shard`) threads a
growing theory across the whole file list: a passing claim, an admitted
axiom, and a discharged requirement are each added so later proofs can
cite them.

```sexp
(claim       NAME GOAL PROOF)   ; prove GOAL by PROOF; admitted as NAME if it checks
(axiom       NAME GOAL)         ; admit GOAL without proof (a trusted boundary)
(requirement NAME GOAL)         ; state an obligation; pending until fulfilled
(fulfills    NAME PROOF)        ; discharge requirement NAME — its goal is looked
                                ;   up from the contract, never restated here
```

`requirement`/`fulfills` split a contract from its proof (single source
of truth for the goal); see `docs/BOUNDARIES.md`. An axiom or a passing
claim/fulfillment is stored closed for citation as a `(lemma NAME)`.

A fifth declaration ties contracts to an executable:

```sexp
(bin NAME (entry MAIN) (externs …) (trusts …) (requires …))
```

declares a binary artifact: `entry` its main function, `externs` the
I/O boundary it may touch, `trusts` the bolt axioms that are its trust
surface, and `requires` the requirement names forming its acceptance
contract — `check` reports each one MET or UNMET (nothing pending is
silent).

**`auto` — sidecar proofs.** In a `claim` or `fulfills`, the PROOF
position may be the symbol `auto` (or `(auto HINT…)` — the tail is
reserved for proof-solver hints and ignored by the checker). The real
proof then comes from the file's *sidecar* `<file>.auto.shard`, a list
of `(proof-for NAME PROOF)` forms generated offline by proof-search
tooling and committed alongside the source. The checker only ever
**replays** a sidecar entry — it is spliced in and goes through the
same parse/desugar/check path as an inline proof, so checking stays
deterministic (check time never searches) and the sidecar is untrusted
input: a wrong or stale entry simply fails. A missing entry is a hard
failure (`examples/auto_missing_rejects.shard` pins this; the demo is
`examples/auto_demo.shard`).

The solver (`tools/prove`) understands these hints, all optional
accelerators — bare `auto` searches unhinted (flat closers, the arith
backend, Farkas certificate search for premised goals (single- and
two-sided, weight-ordered so the first hit is a minimal cert), lemma
rewrites over earlier theory entries, **conditional citation** — a
premised lemma applied via `rewrite-with`, its instantiated premises
discharged by a mini ladder — **premise mining** — a linear-fact lemma
conclusion instantiated at the goal's own arithmetic atoms and
materialized as a `have` cut, facts accumulated until the Farkas search
closes (the `mod_lo`/`mod_hi` idiom; pin
`examples/prove_cond_mine.shard`) — **hypothesis promotion** — every
closed linear-fact case hypothesis restated as a trivial `have` so the
Farkas side can read it (the IH-consuming idiom); parameterized
hypotheses (∀-closed induction IHs) instead join the mining pool citable
as `(hyp K)`, their binders bound by the key/atom match (so an IH
instantiated at `(- n 1)` is mined like any lemma) — **normalizing
prefixes** — on a stuck case the terminal stages re-run behind small
step prefixes applied natively (`simp`, case-hyp and goal-premise
rewrites outermost-first, unfold chains), persisted as
`(steps (PFX) TERMINAL)`
— and structural induction on each goal parameter. Generated induct
cases NAME their field binders (`(case Cons (c0 c1) …)`) so promoted
and mined facts about a case's own fields render; a `fulfills`' goal is
recovered from the target file or the same-module `mod.req.shard`
(interface file form), so module impls get the full search):

- `(induct VAR)` — synthesize a structural induction on VAR.
- `(case-on TERM TYPE)` — synthesize a case split on a computed term
  (e.g. the `(lt a b)` an `If` is stuck on).
- Hints chain: `(auto (induct n) (induct m))` nests — a case of the
  first synthesis that resists its ladder gets the next hint
  synthesized on its own subgoal.

### 10.2 Goals and equations

```sexp
(goal (BINDER…) (PREMISE…) CONCLUSION)
```

- **`BINDER`** is `(name TYPE)`, e.g. `(x Int)` or `(xs (List T))`. The
  binders are the goal's universally-quantified variables.
- **Type parameters are inferred**: any symbol appearing in a binder
  *type* that is not a known type constructor (e.g. the `T` in
  `(List T)`) is collected as a type variable. There is no separate
  `(tv …)` form.
- **`PREMISE…`** is a (possibly empty) list of equations assumed as
  hypotheses; **`CONCLUSION`** is the single equation to prove.
- An **equation** is `(= L R)`, where `L` and `R` are object-language
  term snippets (parsed by `elaborate`).

```sexp
;; ∀ x : Int.  x - 0 = x      (no premises)
(goal ((x Int)) () (= (- x 0) x))

;; ∀ n : Nat.  (add_nat n Z) = n
(goal ((n Nat)) () (= (add_nat n Z) n))
```

### 10.3 Proofs

| Form                                         | Native      | Meaning                                                        |
|----------------------------------------------|-------------|----------------------------------------------------------------|
| `refl`                                       | `Refl`      | the two sides are already syntactically equal                  |
| `(steps (STEP…) PROOF)`                      | `Steps`     | apply rewriting STEPs to the sequent, then continue with PROOF |
| `(induct VAR (CASE…))`                       | `Induct`    | structural induction on `VAR` (one IH per recursive field)     |
| `(case-on TERM TYPE (CASE…))`                | `CaseOn`    | split on the constructor of `TERM` (of named `TYPE`); no IH    |
| `(wf-induct MEASURE PROOF)`                  | `WfInduct`  | well-founded induction on the Int `MEASURE`; prepends IH `ih`  |
| `(subterm-induct VAR PROOF)`                 | `SubtermInduct` | well-founded induction along the structural **subterm order** of `VAR` (a goal parameter of inductive type); prepends a *strong* IH `ih`, citable at any proper subterm — the tool for two-level/nested recursion (subsumed the old `Induct2`) and mutual AST-size fns. Citing the IH leaves a `⊰` ordering premise, discharged by `(below)` |
| `(below)`                                    | `Below`     | discharges a proper-subterm (`⊰`) ordering premise from a `subterm-induct` IH citation by syntactic subterm check — used in the premise-proof slot of `rewrite-with` |
| `(refine-fact TYPE TERM PROOF)`              | `RefineFact` | materialize `(= (PRED (refine_val TERM)) True)` for the refinement `TYPE` as a premise, then continue with `PROOF` (§3, "Refined types") |
| `(have EQ PROOF₁ PROOF₂)`                    | `Have`      | the CUT rule: prove `EQ` by `PROOF₁`, then continue with `PROOF₂` under `EQ` as a fresh premise |
| `(have NAME EQ PROOF₁ PROOF₂)`               | `Have`      | named cut: as above, and `PROOF₂` may cite the fact as `(premise NAME)` — rewritten to the positional 3-arg form before parsing (§10.6), so inserting earlier haves can't break later citations |
| `(fin-split VAR LO HI (CASE…))`              | `FinSplit`  | bounded-Int enumeration: `LO`/`HI` cite range premises for `VAR`; one `(case INT PROOF)` per value |
| `(div-facts TERM D Q PROOF)`                 | `DivFacts`  | inject the Euclidean triple for `TERM` at literal divisor `D` (`n = D·Q + mod n D`, mod ranges), with quotient `Q` a fresh ∀-param — `fin-split Q` then supplies the integrality step the rational Farkas side cannot (see `std/bytes/bytes.shard`'s `mod_byte_id` for the mod-elimination idiom) |
| `(rewrite-with EQREF DIR SIDE (INST…) (PROOF…) PROOF)` | `RewriteWith` | rewrite by a cited equation whose own premises are discharged by the sub-`PROOF`s, then continue |
| `(absurd EQREF)`                             | `Absurd`    | close the goal from a contradictory hypothesis                 |
| `(by THEORY PAYLOAD)`                         | `ByTheory`  | discharge via a decision procedure (§10.7)                     |

### 10.4 Cases

A `CASE` (under `induct`/`case-on`) names a constructor and
gives the sub-proof for that arm:

```sexp
(case CTOR PROOF)              ; constructor with no field binders needed
(case CTOR (FIELD…) PROOF)     ; bind the constructor's fields by name
```

### 10.5 Steps

A `STEP` (inside `(steps …)`) transforms the current sequent. Each takes
a **side** — `lhs`, `rhs`, or `both`:

| Form                                  | Native    | Meaning                                                 |
|---------------------------------------|-----------|---------------------------------------------------------|
| `(reduce SIDE)`                       | `Reduce`  | ι-only: fire matches/ifs on constructor or literal scrutinees, descending everywhere — NEVER unfolds calls (not even ground primitives). The safe workhorse for symbolic proofs. |
| `(simp SIDE)`                         | `Simp`    | full δ+ι small-step to fixpoint — unfolds user fns and fires primitives. Powerful but can concretize terms a symbolic proof wanted left abstract. |
| `(compute SIDE)`                      | `Compute` | ungated big-step evaluation (CBV); unfolds everything incl. nullary fns, leaves genuinely stuck subterms stuck. The ground-fact closer. |
| `(unfold FN SIDE)`                    | `Unfold`  | unfold ONE application of `FN` (leftmost-outermost). Descends into `match` **scrutinees** (binder-free) but never into arm bodies or `let` — for those, step via equation lemmas + `rewrite` instead (the failure trace says which case you're in). |
| `(rewrite EQREF DIR SIDE ALL (INST…))`| `Rewrite` | rewrite SIDE by the cited equation (§10.6)              |
| `(inspect)`                           | `Inspect` | identity — dev aid. Forces the claim's trace block and dumps the in-scope premises/hyps (with citation indices) at this point, even when the claim passes or is `(admit)`-truncated. The authoring idiom: drop it where you are blind, read the dump, finish, delete. |

In `rewrite`: **`DIR`** is `lr` (left-to-right) or `rl`; **`ALL`** is
`true`/`false` (rewrite every match vs. the first); **`INST`** is
`(inst NAME TERM)`, instantiating a variable of the cited equation to an
object-term snippet. When pattern and replacement are closed (no bound
variables), `rewrite` also descends into `match` arm bodies.

### 10.6 Equation references (`EQREF`)

What a `rewrite`/`rewrite-with`/`absurd` cites:

| Form           | Native          | Refers to                                              |
|----------------|-----------------|--------------------------------------------------------|
| `(hyp K)`      | `(Hyp K)`       | hypothesis at positional index `K` (innermost = 0)     |
| `(hyp NAME)`   | —               | a hypothesis by name — rewritten to `(hyp K)` (below)  |
| `(premise K)`  | `(Premise K)`   | the goal's `K`-th premise                              |
| `(premise NAME)`| —              | a named have's fact — rewritten to `(premise K)` (below) |
| `(lemma NAME)` | `(Lemma NAME)`  | an admitted axiom or previously-proven claim           |

**Named citations** are surface sugar only: the loader's s-expr desugar
pass (`kernel/desugar.shard`, `ds_proof`) simulates the kernel's hyp stack
and premise list over the proof **source** and rewrites every name to its
positional form *before* the proof is parsed — the native AST is purely
positional, and no name reaches the reader or the checker. The induction
hypotheses are auto-named `ih`, `ih1`, `ih2`, …: `wf-induct` and
`subterm-induct` prepend one `ih`; each `induct` case appends one IH per
recursive constructor field (in `do_induct` order). Each `case-on` arm
prepends its case equation at `Hyp 0`, auto-named after the arm's
**constructor** — `(hyp Alive)`, `(hyp True)`, `(hyp Some)` — with inner
arms shadowing outer; named citations of outer hyps stay valid inside arms
because the desugarer's simulated stack tracks the prepend. An unbound
name fails LOUDLY at desugaring, naming the missing binding. The pass is
untrusted: a wrong index fails at the checker's citation gate.

### 10.7 Theories (`by`)

`(by THEORY PAYLOAD)` discharges the current sequent with a decision
procedure. ONE theory is registered (`kernel/checker.shard`): `arith` —
linear-integer arithmetic + equality reflection, unified
(`kernel/arith.shard`). Dispatch is **cert-only** on the payload shape,
with no fallback between the two sides:

| Form                     | Side                | Decides                                                            |
|--------------------------|---------------------|--------------------------------------------------------------------|
| `(by arith (list))`      | tautology/decision  | plain linear-integer identities (lhs−rhs ≡ 0); `(int_eq a b) = True` / `(sym_eq a b) = True` reflexivity; `(lt a b) = True` / `(le a b) = True` when `b−a` is a constant of the right sign; `(= a b)` from an in-scope `(int_eq\|sym_eq a b) = True` premise/hyp (the reflect scan) |
| `(by arith (list G M0 …))` | Farkas entailment | premises ⊢ `(lt\|le a b) = True/False`, `(int_eq a b) = True/False`, or a plain `L = R`, by a **checked** multiplier certificate |

The payload is **checker data**, *not* an object-term snippet:
`(list 1 1 -2)` parses into the kernel's small cert grammar (`CData` —
ints and nested lists). A single-sided cert is `(list G M0 M1 …)` — `G`
multiplies the negated goal, `Mk` premise `k`; an equality conclusion
(`int_eq…=True` or plain `L = R`) takes the two-sided
`(list le_mults ge_mults)`, two independent refutations. Inequality
premises take nonnegative multipliers; equality premises take either
sign. An EMPTY `(list)` selects the decision side — deterministic
procedures, no search, premises ignored except by the reflect scan.

> History: until 2026-07 these were five backend names
> (`lia`/`eqdec`/`reflect`/`ord`/`farkas`) over the same polynomial
> machinery; they collapsed into `arith` (REVISIT — "arith — the
> backends unified").

### 10.8 Object-snippet sugars

Inside the object-term snippets a proof embeds (equation sides, `inst`
terms, measures), the ordinary object-language literal sugars apply:

| Form           | Expands to                                  |
|----------------|---------------------------------------------|
| `'foo`         | `(quote foo)` → `SymLit foo`                 |
| `(list a b c)` | `(Cons a (Cons b (Cons c Nil)))`             |
| `"x+y"`        | `(list 120 43 121)` — UTF-8 bytes; `String ≡ (List Int)` |

The retired reflected surface had additional `(ty …)`/`(tv …)` sugars
for building reflected `Type` *values*; the proof language no longer
needs them — binder types are written as ordinary object types (§3) and
type variables are inferred (§10.2).

### 10.9 Worked example

The axiom / requirement / fulfills triple, exercising `induct`, `case`,
`steps`, `unfold`, `reduce`, and a `rewrite` citing the induction
hypothesis (`examples/contract_demo.shard`):

```sexp
(import "../std/nat.shard")

;; admitted without proof, available to later citations:
(axiom add_zero_left (goal ((n Nat)) () (= (add_nat Z n) n)))

;; the obligation, stated once:
(requirement add_zero_right (goal ((n Nat)) () (= (add_nat n Z) n)))

;; its fulfillment — the goal is looked up from the contract:
(fulfills add_zero_right
  (induct n
    ((case Z
       (steps ((unfold add_nat lhs) (reduce lhs)) refl))
     (case S
       (steps ((unfold add_nat lhs)
               (reduce lhs)
               (rewrite (hyp 0) lr lhs true ()))   ; (hyp 0) = the IH
              refl)))))
```

## 11. The narrow / full distinction

`narrow` and `full` are two forms of the same language, **shard**.

- **Narrow shard** is what the Rust bootstrap parses and evaluates — the
  minimal subset described above. It is the **bootstrap floor**: the
  kernel, the evaluator, the parser/front-end (`kernel/reader.shard`),
  and the tools are all written in it, so the small trusted Rust host
  can run them. Narrow grows **reluctantly** — a feature is added to the
  Rust backend only when the engine itself genuinely needs it expressed
  at that level.
- **Full shard** is the richer language, *implemented by the shard
  engine* (which is itself written in narrow). This is where features
  actually accrue — a new sugar or construct is added to the shard
  front-end first. Candidate additions: effect-as-data trees, bridging
  axioms, richer collections, measure / well-founded recursion, mutual
  recursion, `let*` and pattern sugar, module visibility.

**There is no full→narrow lowering, and no certificate scheme.** Narrow
is not a compilation target; it is the floor the system is bootstrapped
from. The engine interprets full shard directly today, and the eventual
compile story is **full shard straight to a machine target** (wasm,
x86) — see `docs/OVERVIEW.md`.

### Two constraints that govern what `full` may add

1. **Compile-to-bare-metal.** A serious shard application is *compiled*
   to a standalone binary with **no runtime, no GC, no interpreter, no
   kernel sidecar** (the snake demo reduces to a bare x86 executable —
   just its `step` function plus IO). "Programs are data" is a
   *build-time* power used by the prover and compiler; it is **not** a
   runtime capability an application gets. So a feature is admissible
   only if it compiles fully away. **Lambdas / first-class closures are
   the cautionary case**: a closure is a heap environment + indirect
   call — a runtime — so they may be added only if they
   defunctionalize / inline / monomorphize away completely (hence the
   `apply$` defunctionalization note in the roadmap, not closures as
   runtime values).
2. **Provable lowering.** Each step from full toward the metal is an
   explicit, separately *proven* refinement (`spec ⊑ … ⊑ machine`), not
   a "sufficiently smart compiler." See TRANSFER.md and
   `docs/OVERVIEW.md` for the broader picture.


## 12. Worked example: a fragment

```sexp
;; List-typed length.
(type (List T) (Nil) (Cons T (List T)))

(fn length ((xs (List Int))) Int
  (match xs
    (Nil 0)
    ((Cons _ rest)                 ; first PVar ignored, `rest` = BVar 0
      (+ 1 (length rest)))))

;; Bool-returning, exercises if.
(fn even ((n Int)) Bool
  (if (int_eq (mod n 2) 0) True False))

;; Parallel let.
(fn swap_sum ((a Int) (b Int)) Int
  (let ((x b) (y a))               ; x = b, y = a (parallel; outer scope)
    (+ x y)))
```


## 13. Things deliberately not in narrow

- No lambdas (use top-level fns).
- No partial application; calls must be saturated.
- No type checking at load or eval time (Trust comes from review of
  the trusted Rust component — see REVISIT, "Trusted-by-review Rust
  component").
- No first-class namespaces *within* the evaluator — once a file set is
  assembled it loads into one flat module. Files do, however, declare
  their dependencies with `(import "path")`, and a directory-based module
  system with interface/visibility rules is layered on top (see §2); that
  system is not specified in this doc.
- No mutability of any kind.
- No *distinct* string type: string literals `"…"` exist as sugar for
  the `(List Int)` of their UTF-8 bytes (§4.1, §10), not as an opaque
  primitive.
  (The opaque `std/bytes` construction (§3) is the *byte*-sequence type
  — the text type over it is future work; issue #2.)
- No floats.

These are constraints the narrow form imposes; the full language
will lift several of them.
