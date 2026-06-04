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
(`mod.req.shard` interface files; MODE-AWARE resolution — proof checking
sees a module's interface, running code gets the impl bodies; a loader
gate that rejects reaching past another module's interface); that system
is beyond the scope of this document.

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
| `BareName`          | reference to a declared `type`       |
| `(TyCon T1 T2 …)`   | type application                     |
| `(Word W S)`        | fixed-width modular integer (see "Words" below) |
| `u8 … u64, i8 … i64` | reader aliases for `(Word 8 Unsigned)` etc. |

Examples: `(List Int)`, `(Option (Pair Symbol Expr))`,
`(Map Symbol Type)`, `(List u8)`.

`Int` and `Symbol` are primitive; `Bool` is part of the stdlib.

### Words — fixed-width modular integers

`(Word W S)` is a built-in type former: `W` a **literal type** — a
numeric token in type position, width 1..64 — or a type variable; `S`
a signedness marker (`Unsigned`/`Signed`, ordinary empty stdlib types
used as phantom indices) or a type variable. Prefer the aliases
(`u8`, `i32`, …); they expand in the reader, so the kernel only ever
sees the former.

A word value is a bare bit pattern (width + unsigned residue,
canonical: `0 ≤ raw < 2^W`). Signedness is **not** stored in the
value — it lives in the op names where semantics differ (`uval`/
`sval`, `udiv`/`sdiv`, `ult`/`slt`, `ushr`/`sshr`) and in the type
index, which keeps signed ops off unsigned terms. `Word` has no
typedef: values are produced only by the word primitives, the type
checker enforces canonicity on any literal reaching a goal, and
induction/case-on over it is impossible. Construction is
`(uwrap K e)` / `(swrap K e)` where `K` **must be an integer
literal** — it becomes the type index.

All word ops are **total**, with explicit conventions (semantics live
in the operator name, never in context):

- `wadd wsub wmul wneg` — value mod `2^W` (wrapping; hardware
  semantics, reasoned about rather than guarded).
- `udiv/urem`, `sdiv/srem` — the RISC-V M / SMT-LIB profile:
  `x/0 = all-ones`, `x rem 0 = x`, `INT_MIN/-1 = INT_MIN` (wraps),
  `INT_MIN rem -1 = 0`. Signed division **truncates** toward zero.
- `wshl ushr sshr` — shift amount is an `Int`; amounts outside
  `[0, W)` saturate (`wshl`/`ushr` → 0, `sshr` → sign fill). `sshr`
  rounds toward −∞.
- `wand wor wxor wnot` — bitwise.
- `weq`, `ult ule`, `slt sle` — comparisons, returning `Bool`.
- `uval sval` — the value as an `Int` (unsigned residue / two's
  complement); `wbits` — the raw residue of any word (explicit
  reinterpretation); `wwidth` — the width as an `Int`.

These primitives are implemented once, in `kernel/reduce.shard`'s
table (not natively), so the proof reducer and the hosted evaluator
share one definition by construction.


## 4. Expressions

Every expression at runtime evaluates to a value (also an `Expr`,
in normal form). The evaluator is call-by-value.

### 4.1 Literals

- **Integer literal**: `42`, `-7`, `0`. Parses to `IntLit`.
- **Symbol literal**: `(quote foo)`. Parses to `SymLit`. The unquoted
  form `foo` is treated as a variable reference, not a symbol value.
- **String literal**: `"x+y"`. Sugar for the `(List Int)` of its
  Unicode-scalar codepoints — `(Cons 120 (Cons 43 (Cons 121 Nil)))`.
  `String ≡ (List Int)`; there is no distinct string type, so the
  `std/list` lemma library applies to strings unchanged. Valid only in
  expression position (match against strings by destructuring Cons/Nil).

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


## 9. Stdlib types

Defined in `kernel/stdlib.shard`:

```sexp
(type (List T)   (Nil) (Cons T (List T)))
(type (Option T) (None) (Some T))
(type Bool       (False) (True))
(type (Pair A B) (Pair A B))
```

Used throughout the kernel; no privileged status — just user types
that happen to be ubiquitous.


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
| `(induct2 VAR (CASE…))`                      | `Induct2`   | two-step (parity) induction; `SS` case carries one IH          |
| `(case-on TERM TYPE (CASE…))`                | `CaseOn`    | split on the constructor of `TERM` (of named `TYPE`); no IH    |
| `(wf-induct MEASURE PROOF)`                  | `WfInduct`  | well-founded induction on the Int `MEASURE`; prepends IH `ih`  |
| `(have EQ PROOF₁ PROOF₂)`                    | `Have`      | the CUT rule: prove `EQ` by `PROOF₁`, then continue with `PROOF₂` under `EQ` as a fresh premise |
| `(fin-split VAR LO HI (CASE…))`              | `FinSplit`  | bounded-Int enumeration: `LO`/`HI` cite range premises for `VAR`; one `(case INT PROOF)` per value |
| `(rewrite-with EQREF DIR SIDE (INST…) (PROOF…) PROOF)` | `RewriteWith` | rewrite by a cited equation whose own premises are discharged by the sub-`PROOF`s, then continue |
| `(absurd EQREF)`                             | `Absurd`    | close the goal from a contradictory hypothesis                 |
| `(by THEORY PAYLOAD)`                         | `ByTheory`  | discharge via a decision procedure (§10.7)                     |

### 10.4 Cases

A `CASE` (under `induct`/`induct2`/`case-on`) names a constructor and
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
| `(unfold FN SIDE)`                    | `Unfold`  | unfold ONE application of `FN`; does not descend into `match` (not even the scrutinee) — once in match-land, step via equation lemmas + `rewrite` instead. |
| `(rewrite EQREF DIR SIDE ALL (INST…))`| `Rewrite` | rewrite SIDE by the cited equation (§10.6)              |

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
| `(hyp NAME)`   | `(HypName NAME)`| a hypothesis by name — desugared to positional (below) |
| `(premise K)`  | `(Premise K)`   | the goal's `K`-th premise                              |
| `(lemma NAME)` | `(Lemma NAME)`  | an admitted axiom or previously-proven claim           |

**Named hypotheses** are a parse-time convenience. The reader emits
`(HypName NAME)`; a separate pass (`kernel/desugar.shard`,
`desugar_proof_named`) simulates the kernel's hyp stack and rewrites each
to its positional `(Hyp K)`. The induction hypotheses are auto-named
`ih`, `ih1`, `ih2`, …: `wf-induct` prepends one `ih`; each
`induct`/`induct2` case appends one IH per recursive constructor field
(in `do_induct` order). An unbound name is left as `(HypName NAME)` and
fails cleanly at resolution. The reader stays a pure parser — it does not
know induction semantics — which is why naming is resolved in a later
pass, not during parsing.

### 10.7 Theories (`by`)

`(by THEORY PAYLOAD)` discharges the current sequent with a decision
procedure. Four are registered (`kernel/checker.shard`):

| THEORY   | Decides                                                            | Payload          |
|----------|-------------------------------------------------------------------|------------------|
| `lia`    | linear-integer identities (normalize both sides; lhs−rhs ≡ 0)     | `(list)` — unused |
| `eqdec`  | `(int_eq a b) = True` / `(sym_eq a b) = True` reflexivity facts   | `(list)` — unused |
| `ord`    | `(lt a b) = True` / `(le a b) = True` when `b−a` is a constant    | `(list)` — unused |
| `farkas` | linear-integer **entailment**: premises ⊢ `(lt|le a b) = True`    | Farkas cert (below) |

Only `farkas` reads its payload. The payload is **native data** read
directly by the checker, *not* an object-term snippet: `(list 1 1 -2)`
becomes the native list of bare `Int`s `(Cons 1 (Cons 1 (Cons -2 Nil)))`
(the equality case nests two lists, `(list le_mults ge_mults)`). The
other three take an empty `(list)`.

### 10.8 Object-snippet sugars

Inside the object-term snippets a proof embeds (equation sides, `inst`
terms, measures), the ordinary object-language literal sugars apply:

| Form           | Expands to                                  |
|----------------|---------------------------------------------|
| `'foo`         | `(quote foo)` → `SymLit foo`                 |
| `(list a b c)` | `(Cons a (Cons b (Cons c Nil)))`             |
| `"x+y"`        | `(list 120 43 121)` — codepoints; `String ≡ (List Int)` |

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
  `(List Int)` of codepoints (§4.1, §10), not as an opaque primitive.
- No floats.

These are constraints the narrow form imposes; the full language
will lift several of them.
