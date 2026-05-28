# The Narrow Language

Concentrated reference for the v2 object language as currently
implemented. The "narrow" form is a deliberately restricted subset of
the eventual "full" language — restricted enough to host a small,
trusted Rust evaluator over it, expressive enough that the proof
checker kernel is written *in* this form (see TRANSFER.md).

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

Three top-level forms:

```
(type        NAME-OR-PARAMETERIZED  CTORDEF …)
(fn          NAME ((P TYPE) …) RET-TYPE  BODY-EXPR)
(extern      NAME ((P TYPE) …) RET-TYPE)
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
| `Int`               | arbitrary-precision integer [REVISIT — currently i64] |
| `Symbol`            | interned identifier                  |
| `Bool`              | user-defined (in stdlib), `True`/`False` ctor |
| `BareName`          | reference to a declared `type`       |
| `(TyCon T1 T2 …)`   | type application                     |

Examples: `(List Int)`, `(Option (Pair Symbol Expr))`,
`(Map Symbol Type)`.

`Int` and `Symbol` are primitive; `Bool` is part of the stdlib.


## 4. Expressions

Every expression at runtime evaluates to a value (also an `Expr`,
in normal form). The evaluator is call-by-value.

### 4.1 Literals

- **Integer literal**: `42`, `-7`, `0`. Parses to `IntLit`.
- **Symbol literal**: `(quote foo)`. Parses to `SymLit`. The unquoted
  form `foo` is treated as a variable reference, not a symbol value.

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
| `+ - * /`    | `Int × Int → Int`          | `/` rejects div-by-zero     |
| `mod`        | `Int × Int → Int`          | Euclidean (result ≥ 0)      |
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


## 9. Stdlib types

Defined in `kernel/stdlib.sexp`:

```sexp
(type (List T)   (Nil) (Cons T (List T)))
(type (Option T) (None) (Some T))
(type Bool       (False) (True))
(type (Pair A B) (Pair A B))
```

Used throughout the kernel; no privileged status — just user types
that happen to be ubiquitous.


## 10. Proof-file surface sugars

Beyond the narrow source forms above, the loader recognizes a small
set of sugars used inside `(claim …)` bodies — most of them produce
runtime Expr/Goal *values* rather than source-level terms (see
REVISIT, *Expr-value vs source-Expr distinction*).

| Form              | Expands to                                  | Slice |
|-------------------|---------------------------------------------|-------|
| `'foo`            | `(quote foo)` → `SymLit foo`                 | 25    |
| `(list a b c)`    | `(Cons a (Cons b (Cons c Nil)))`             | 25    |
| `(ty NAME a1 a2)` | `(TCon 'NAME (list a1 a2))`; bare symbols inside become 0-ary TCons | 28 |
| `(tv T)`          | `(TVar 'T)` — type variable               | 31    |

`'foo` is handled by `lexpr`'s reader; the rest are recognized in
`src/load.rs::load_expr`. The (claim NAME GOAL PROOF) form is itself
recognized by `src/bin/check.rs`, not by the kernel.

## 11. The narrow / full distinction

The "narrow" language described above is what the v2 loader and
evaluator accept. The "full" language — the eventual surface for
users writing proofs and refinements — will add:

- Higher-order: lambdas, partial application, `apply$` defunctionalization
- Effect-as-data: `Action` trees built from `Pure` / `Bind` / `Yield`
- Bridging axioms tying externs to in-language models
- Finite maps / collections with their lemma library
- Measure / well-founded recursion (termination as discharged obligation)
- Mutual recursion and mutual induction
- (Possibly) sequential `let*` and pattern syntax sugar
- (Possibly) module imports / visibility

The full language will be **compiled down to the narrow language**.
The narrow kernel will load and check certificates produced by that
compilation, never the full language directly. See TRANSFER.md and
docs/BOUNDARIES.md for the broader picture.


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
- No imports / namespaces; all definitions live in a flat module.
- No mutability of any kind.
- No string literals (use Symbol via `quote`).
- No floats.

These are constraints the narrow form imposes; the full language
will lift several of them.
