# The language

What a shard source file may say, and what the system does with it. This is reference: look a form
up here. To learn how to write a program and prove something about it, start with `AUTHORING.md`.

The text was written from the code, the tests and the pins (small pinned examples under
`v3/pins/`).

The order follows a file through the system: the source text and how files become modules;
declarations, terms, and how a function becomes a definition and a program; running a program;
proofs; what makes a result accepted; and, last, every record the loader prints.

Refusal texts are quoted as the system prints them.

## Source text

A source file is a sequence of s-expressions in a file named `*.shard`. One reader serves every
file (`v3/kernel/sexpr.shard`). It works on bytes, knows no keyword and gives no name a meaning.

### Tokens

| Token | Example | Rule |
|---|---|---|
| whitespace | | space, tab, carriage return, line feed; separates tokens |
| comment | `; text` | from `;` to the end of the line |
| list | `(f x)` | `(` … `)`; the only brackets — `[` and `]` are symbol characters |
| numeral | `42`, `-7` | decimal digits, with an optional leading `-` |
| string | `"hi\n"` | the bytes between double quotes, escapes decoded |
| symbol | `List.length`, `f'`, `<=`, `α` | any other run of bytes up to a delimiter |
| universe list | `Prod.{u v}` | a symbol ending in `.`, followed at once by `{` … `}` |
| quote | `'x` | reads as `(quote x)` |

A delimiter ends a numeral or a symbol: whitespace, `(`, `)`, `"`, `;`, `{`, `}`. A `'` is the
quote only at the start of a token; inside a symbol it is an ordinary character (`f'`).

**Numerals.** A token that starts with a digit, or with `-` and a digit, is a numeral and must be
digits throughout (`7x` is refused). `-` alone and `-x` are symbols. There is no fraction, exponent
or radix prefix. The reader keeps the sign; a term then gives the numeral the type its position
expects, `Nat` or `Int`, and a negative numeral where a `Nat` is expected is refused
`negative_numeral`. Examples: `(def five () Nat 5)` (`v3/pins/reader/world.shard`),
`(fn neg () Int -7)` (`v3/pins/loader/ev_profile/kernel/k.shard`).

**Strings.** The escapes are `\n`, `\t`, `\r`, `\\`, `\"`, `\xHH` (one byte, two hex digits) and
`\u{H…}` (a code point of one to six hex digits, stored as UTF-8). Every other byte is taken as
written, a line break included. The reader yields the byte sequence; what it means depends on
where the literal stands:

| Where | What the literal is | Refused when |
|---|---|---|
| a term or a function body in an ordinary file: `(fn g () String "abc")` (`v3/pins/loader/ev_string/main.shard`) | a value of Init's `String` | the bytes are not valid UTF-8 (`bad_string`); `String` is not visible in the file (`no_string`) |
| a function body in a module whose path begins `kernel`: `(fn hello () (List Int) "hi")` (`v3/pins/loader/ev_profile/kernel/k.shard`) | the list of its bytes, built from the two-constructor `List` in scope | no such `List` (`no_list`) |
| `(import "…")` | a path (see "Files, modules and names") | |

**Characters.** There is no character literal. A byte is written as its number — the reader's own
source tests `(= c 40)` for `(` — and text is a `String` or a list of bytes.

**Quote.** `(quote NAME)` is a literal of the program type `Symbol`: `(fn g () Symbol (quote x))`
(`v3/pins/loader/ev_symbol/main.shard`). It has no counterpart in the logic: a function whose body
holds one is a program without a definition (`RUNNABLE … why=no_l_identity`), and as a pattern in a
term it is refused `literal_pattern`. `'(a b)` reads, but no form accepts a quoted list.

### Names

A symbol is read as a name by splitting it at each `.`: `List.length` has two components. Every
component is non-empty; a component of digits only is a number.

- `@NAME` in a term cites NAME with every argument written, the implicit ones included:
  `(@Decidable.decide p d)` (`v3/pins/loader/init_horizon/small.shard`).
- A universe list after a *declared* name names its universe parameters:
  `(inductive Eq.{u} …)` (`v3/pins/reader/world.shard`). After a *cited* name it gives the universe
  arguments as level terms: `(PProd.{1 1} Nat Nat)` (`v3/pins/loader/discharge_measure/main.shard`).
  A wrong count is `universe_arity`; a bound variable or a program term takes none
  (`bad_universe_suffix`). Where no list is written the levels are inferred.

### What the reader refuses

A file that does not read produces one record, `READ-ERROR FILE REASON: at byte N` — N is the
offset where reading stopped — and none of its forms is loaded.

| Reason | Cause |
|---|---|
| `unterminated_list` | the file ends inside `( …` |
| `unexpected_close` | a `)` with no list open |
| `unterminated_string` | the file ends inside `"…` |
| `bad_escape` | an escape not listed above, or a malformed `\x` or `\u{…}` |
| `bad_numeral` | a token that starts as a numeral and is not digits throughout |
| `stray_brace` | `{` or `}` at the start of a token |
| `bad_universe_suffix` | a symbol ending in `.` with no `{` after it, or `{` after a symbol not ending in `.` |
| `unterminated_universe_list` | the file ends inside `.{ …` |
| `quote_at_end` | `'` with nothing after it |
| `reader_fuel` | more than 100,000,000 reading steps |

### Top-level forms

Every top-level s-expression is a list headed by a keyword ("Declarations" lists them); anything
else is
`unknown_form`: `(frob two)` (`v3/pins/reader/refuse_unknown_form.shard`). Two families are refused
with a pointer: `instance class requires app namespace section variable` are `reserved_form`, and
a Lean keyword with no form here (`partial`, `notation`, `open`, `lemma`, `mutual`, …) is
`lean_form`, its message naming what to write (`v3/pins/loader/lean_form/main.shard`). The layout
of a file — line width, indentation, where a form breaks — is the layout law, `docs/CANON.md`.

## Files, modules and names

### A file is a module

The loader is given a package root — a directory — and files under it. A file's **module path** is
its path from the root with `/` read as `.` and `.shard` dropped: under the root `v3`,
`v3/std/bytes.shard` is the module `std.bytes`. A module is loaded once per load.

- A file stem holds no dot (`bad_module_name`); so `FILE.auto.shard`, the proof sidecar beside
  `FILE.shard`, is not a module. No module at the root is named `Init`. A path that does not end
  in `.shard` names a directory module ("Views").
- Two files that yield one module path — `lib.shard` and the directory `lib/` — collide: the second
  is `module_collision` (`v3/pins/loader/collision`).

### Identity

A declaration's **identity** is its file's module path followed by the name it declares. The
declared name may itself be dotted: `(fn Byte.ofNat …)` in `v3/std/bytes.shard` is
`std.bytes.Byte.ofNat`. Declarations imported from Lean's `Init` keep Lean's names with no prefix:
`List.length`. The checker's environment holds one declaration per identity.

### `import`

| Form | Loads | Example |
|---|---|---|
| `(import "FILE.shard")` | a file; the path is relative to the importing file's directory | `(import "a.shard")` (`v3/pins/loader/basic/main.shard`) |
| `(import "DIR")` | a directory module: its view and nothing else | `(import "lib")` (`v3/pins/loader/directory/main.shard`) |
| `(import Init NAME)` | Lean's `Init`, through its declaration NAME | `(import Init Eq)` (`v3/pins/loader/init/main.shard`) |

An import makes identities **visible**; it opens no name — after `(import "a.shard")` a bare `Nat`
is still `unknown_constant` (`v3/pins/loader/no_open`). A file sees its own module, every module it
imports, and everything those see (`v3/pins/loader/visible`). A declaration that is in the
environment but outside this closure cannot be cited.

Each of these ends the importing file with a `LOAD-ERROR`: an absolute path (`absolute_import`), a
path climbing above the root (`import_outside_root`), no such file (`missing_file`), a cycle
(`import_cycle`), nesting deeper than 1,000 (`import_depth`), an imported file whose own loading
failed (`import_failed`; it is not read a second time), an `(import Init …)` that follows a
declaration and would raise the horizon (`late_init_import`, under "`use`").

**Importing `Init`.** The loader is given one pinned export of Lean's `Init` library and an index
of it (`--init`, `--init-index`). The export lists Init's declarations in dependency order, each
with an ordinal. `(import Init NAME)` sets the file's **horizon** just past NAME: the file sees
every Init declaration up to and including NAME and none after it. Nothing is loaded wholesale.
Once a file's directives are read, the loader collects the names its forms mention (and the names
the elaborator and the proof steps build on their own), keeps those the export declares below the
horizon, and has the checker admit them and what they depend on — once per load.

- A file may have several `(import Init …)` lines: the farthest sets its horizon, whatever their
  order. A file also sees Init through its imports: its horizon is the farthest of its own and of
  each imported module's (`v3/pins/loader/init_horizon`). A file with no horizon sees no Init.
- Citing a declaration at or above the horizon is `READ-ERROR … above_horizon`; the message names
  the declaration, its ordinal, the horizon with the import that set it — the Init line
  (`v3/pins/loader/init_two_imports`), or the imported file the horizon is inherited through
  (`init_horizon_inherited`) — and the import to write (`init_above_horizon`). The same refusal
  answers a name the source does not spell: an instance the fixed table supplies (`max` at `Nat`
  is `Max.max` under `Nat.instMax`; `init_above_instance`), and a lemma a proof step builds on
  where K refused the theorem for lack of it — the farthest such constant is named, so one import
  reaches them all.
- A NAME the export does not declare is `init_name_not_found`; no export given is
  `init_unavailable`; an export whose first line is not the pinned Lean version and exporter is
  `export_pin_mismatch`; a missing index, or one built over another export, is `init_index_stale`.

### `use`

`(use PREFIX)` opens a prefix for the forms after it: an identity `PREFIX.X` may be cited as `X`.
`(use PREFIX A B)` opens it only for citations whose first component is `A` or `B`
(`v3/pins/loader/selective`). Modules and namespaces are both prefixes, so one form serves both:
`(use a)` opens the module `a`; `(use Init.Nat)` opens Lean's namespace `Nat` — the leading `Init`
is dropped, Init's names having no prefix (`v3/pins/loader/init/main.shard`). A constructor is a
name under its type, so opening a module does not open the constructors of its types: after
`(use a)` the constructor `a.Tok.TPlus` is written `Tok.TPlus`, or bare after `(use a.Tok)`; a
bare `TPlus` is `unbound_name` in a `fn` and `unknown_constant` in a `def`. In the same way
`(use Init.Bool)` opens `true` and `false`, which are otherwise `Bool.true` and `Bool.false`; a
bare `true` without it is `unknown_constant: true: Init's constructor is Bool.true — write
Bool.true, or open the namespace with (use Init.Bool)` (pin `bool_bare`).
Besides, every `type` a file declares opens its constructors for that whole file. A `(use PREFIX)`
that no name in the file could need is reported as `UNUSED MODULE use=PREFIX,…` (an Init namespace
is listed without `Init.`); it is not an error.

`import`, `use` and `trusts` are the **directives**; `(trusts NAME…)` widens its own file's
assumption policy ("Acceptance: axioms and policy"). By convention the directives lead the file;
nothing checks their place or their order. A `use`, a file `import` and a `trusts` that stand
after a declaration are accepted and act on the forms after them. Init's declarations are fetched
once, after the leading directives, at the horizon those set. An `(import Init …)` after a
declaration that would raise the horizon is refused, and the file's loading ends: `LOAD-ERROR FILE
late_init_import: (import Init Int.decEq) raises the module's horizon from 70 (import Init Eq) to
836 after the file's Init declarations were fetched: an import of Init that raises the horizon
stands before the file's first declaration — move it there` (`v3/pins/loader/init_import_late`;
`init_import_late_none` for a file whose leading directives import no Init). One that does not
raise the horizon is accepted and changes nothing (`init_import_late_same`). An implementation is
read at its view's horizon: its own directives stand after the view's declarations, so an
`(import Init …)` of the implementation past the view's horizon is refused the same way, and the
import belongs in the view (`impl_import_late`). A *file* import after a declaration that brings a
farther horizon is accepted, and the names this file cites between the two horizons are not
fetched for it: one is `above_horizon` with the horizon of the fetch and the import to move — when
the file is loaded alone (`OPEN.md` §1).

### How a name resolves

A name bound in the term — a binder, a pattern variable — wins. Otherwise a cited name N is tried
as each of these candidates:

1. `MODULE.N`, for the file's own module;
2. `P.N`, for each opened prefix P;
3. N without its leading `Init.`, when it has one and the file sees Init;
4. N itself — a full identity, or an Init name.

A candidate is a hit when the environment holds it and the file sees it. One hit is the answer; none
is `unknown_constant`; two different hits are `ambiguous_name`, naming both
(`v3/pins/loader/ambiguous`) — never a silent choice. Three positions decide between two hits:
where a type is expected and the hits are a type and its own constructor of the same name, the
type; where the expected type is an inductive type, its constructor of that name (a pattern's
constructor is looked up under the matched type first); in a function's signature and a `type`'s
fields, the hit that is a type — two types there are `ambiguous_type`. A full identity always
resolves: `a.Nat`, `main.Bool`, and `Init.Bool` for Init's `Bool` beside a native one
(`v3/pins/loader/same_spelled`). Citing a theorem whose proof is `sorry` is `pending_obligation`.
Programs resolve function and constructor names by the same candidates; their refusals are
`unknown_head` and `ambiguous_head`.

### The naming law

Lean's declarations keep Lean's names, and nothing else may take them. A file gives an Init
function a body that runs by attaching one (`realize`), never by declaring a namesake.

| Rule | What is refused |
|---|---|
| A file's declarations live under its module path; no form declares a bare name | — |
| A declaration may not take a name Init declares | the module `Classical` declaring `choice`: `REFUSE Classical.choice name_taken` (`v3/pins/loader/init_name_taken_l`) |
| A `fn` or `extern` may not take the name of a declared constant, nor a declaration the name of a function that has no definition | `name_taken` (`v3/pins/loader/def_takes_fn`) |
| Two files may not construct one identity (`a.shard` declaring `b.c`, `a/b.shard` declaring `c`) | the second: `duplicate_name` for a program, `already_declared` for the checker (`v3/pins/loader/qualified_collision`, `qualified_collision_l`) |
| Lean's keywords and operator spellings that have no form here | `lean_form`, `lean_symbol`, each with the spelling to write |

### Toolchain files and ordinary files

The system's own sources (`v3/kernel/`, module paths beginning `kernel`) are written in the same
language, but none imports Init. The loader makes two distinctions: one by the module path (the
first row), one by what a file sees, whatever its path (the other rows).

| | Ordinary file, seeing Init | Toolchain file, seeing no Init |
|---|---|---|
| a string literal in a function body | a `String` | the list of its bytes (decided by the path `kernel…`) |
| data it works on | Init's: `List.cons`, `Bool.true`, `Nat` | its own: `v3/kernel/prelude.shard` declares `List` (`Nil`, `Cons`), `Option`, `Bool`, `Pair`; `Int` and `Symbol` are built in |
| a `fn` | a definition the checker holds where it can be defined, and a program | a program only |
| an `extern` | an opaque constant the checker holds, and a program | a program only |
| a `type` whose fields cite `Int` or `Nat` | an inductive type the checker holds | a program type only; a `def` citing it is `unknown_constant` (`v3/pins/loader/type_e_only`) |

An ordinary file may import a toolchain file and call it (`v3/tools/shardfmt/reads/reads.shard`
imports `kernel/sexpr.shard`); what it calls stays a program without a definition.

## Views

A **directory module** is a directory DIR that has a **view**, the file `DIR/mod.req.shard` (or
`DIR/mod.req/mod.req.shard`). `(import "DIR")` loads the view and nothing else; the module's path is
DIR's. The **implementation** is `DIR/BASE.shard`, BASE being DIR's last component
(`lib/lib.shard`). A directory without a view cannot be imported (`missing_interface`).

### What a view holds

| Form | Declares | In a consumer |
|---|---|---|
| `(sig type NAME)`, `(sig type (NAME T…))` | an opaque type and its number of type parameters | `PARAM DIR.NAME type arity=N`; no constructor exists |
| `(sig fn NAME BINDERS RET)` | a function's signature, without a body | `PARAM DIR.NAME fn`; the checker cannot unfold it |
| `(requirement NAME BINDERS PROP)` | a law the implementation must prove | `PARAM DIR.NAME requirement`; cited like a theorem |
| `(theorem NAME BINDERS PROP PROOF)` | a public lemma, proved from what the view declares | an accepted theorem |
| `type`, `def`, `abbrev`, `opaque`, `structure`, `inductive`, a trusted `axiom` | themselves, in full | as declared: a consumer's function matches on the `type`'s constructors, and its definitions and theorems over the others are accepted |

A view holds no `fn`, `extern`, `realize`, `derive`, `fulfills` or `bin` (`LOAD-ERROR … view_form`),
nor a `record`, which is refused as the functions it expands to (`view_form: fn: a view holds
declarations and parameters, never bodies`). `sig` and `requirement` stand nowhere but in a view
(`sig_outside_view`). A view may import
Init, directory modules and files outside its own directory; importing a file of its own directory
is `req_scope`. Example (`v3/pins/loader/view_basic/lib/mod.req.shard`):

```
(import Init Eq)
(sig type Stack)
(sig fn push ((n Nat) (s Stack)) Stack)
(requirement stack_refl ((s Stack)) (Eq s s))
(theorem push_refl ((n Nat) (s Stack)) (Eq (push n s) (push n s)) (exact (stack_refl (push n s))))
```

To the checker a parameter is a constant with a type and no value. A declaration that rests on
parameters is accepted, and its record lists them (`params=lib.stack_refl,lib.push,lib.Stack`).
The view is therefore all a consumer knows: citing an implementation's equation (`lib.step.eq_1`)
is `unknown_constant` (`v3/pins/loader/view_eq_hidden`), a proof by computation through a `sig fn`
is `type_mismatch` (`view_rfl`), applying a `sig type`'s constructor is refused (`k_client_reach`).
A module that wants a fact public states it as a `requirement` or a `theorem`. A `sig fn` whose
signature names a type the checker does not hold is a parameter for programs only, recorded alike.

### The sealed directory

The files inside a directory that has a view are private to it. A file under DIR may import them; a
file import from anywhere else is `LOAD-ERROR … private_module` (`v3/pins/loader/private_module`).
A directory module nested inside DIR, with a view of its own, is private to DIR in the same way:
DIR's files import it, and `(import "DIR/INNER")` from anywhere else — or an import of INNER's
view file by its path — is `private_module` (`v3/pins/loader/private_dir`).
A file directly inside DIR declares under DIR's module path — `lib/types.shard` declares `lib.T`
(`v3/pins/loader/sealed_module`) — so the directory is one module, and its implementation file may
hold nothing but imports of its private files.

### Checking an implementation

The check runs when the loader is given the directory itself, as an extra root beside the consumer —
`load.shard --root R R/main.shard R/lib` — never when a consumer imports it. It runs in a **fork**
of the load taken after the view's directives: the fork replays the view's forms in order and, at
each parameter, consumes the implementation's forms in file order up to the one that implements it;
the rest follow.

| View form | Implemented by | Compared | Record |
|---|---|---|---|
| `sig type` | `(type NAME …)` | the number of type parameters | `DISCHARGE NAME type` |
| `sig fn` | `(fn NAME …)` | the program signatures: type parameters, parameter types, result type | `DISCHARGE NAME defined` when the fn has a definition, which takes the parameter's place in the fork; `DISCHARGE NAME fn` when it is a program only — linked, nothing proved through it |
| `requirement` | `(fulfills NAME PROOF)` | PROOF against the statement, re-read in the fork where the sig types are the implementation's | `DISCHARGE NAME proved`; or `IMPL PENDING NAME …` and `DISCHARGE NAME pending` |
| `theorem` | — | not checked again | — |

A failure is `IMPL-ERROR NAME REASON: …`, an error of the load: `impl_missing_type`,
`impl_missing_fn`, `impl_missing_fulfills`, `impl_type_arity`, `impl_signature`, and
`fulfills_unknown` for a `fulfills` that names neither a requirement nor an obligation of one of the
implementation's own functions. The implementation's own declarations print as their usual records
behind `IMPL `: `IMPL DEFINE lib.push equations=lib.push.eq_1`. A record of a form of the
implementation names the implementation's file — `IMPL READ-ERROR lib/lib.shard …`, `IMPL
LOAD-ERROR lib/lib.shard …` (`v3/pins/loader/impl_read_error`, `impl_load_error`) — and an import
written there resolves from the implementation's directory, also where the view is in the
directory form (`view_dir_form_private`).

The fork's environment is dropped when the check ends: in the main load the parameters stay
parameters (`PARAM lib.stack_refl requirement` is still there after `DISCHARGE … proved`). Only the
fork's programs are merged, each implementing `fn` in the place of its `sig fn`. That merge is what
lets `--run`, `--dump` and `--emit` execute a consumer: without the directory as a root, a
consumer's program is stuck at the `sig fn` (`unlinked`).

Limit: no record joins a consumer's `params=` to an implementation's `DISCHARGE` lines. A consumer's
theorem stays conditional on the parameters it names; reading the two together is left to the reader.

## Declarations

A source file is a sequence of s-expression forms: directives (`import`, `use`, `trusts`), which
by convention lead the file, and declarations `(KEYWORD NAME …)`. The loader reads the
declarations in order and prints one or more *records* for each: a line whose first word is the
record's kind (`ACCEPT`, `DEFINE`, `RUNNABLE`, `REFUSE`, …). A directive may stand after a
declaration: a `use`, a file `import` and a `trusts` are accepted there and act on the forms after
them. An `(import Init …)` there that would raise the horizon is `late_init_import` ("`use`").

| form | syntax | introduces | example (file) |
|---|---|---|---|
| `import` | `(import "FILE")`, `(import Init NAME)` | a module made visible; the file's Init horizon | `(import Init Int.decEq)` (`v3/examples/calc/calc.shard`) |
| `use` | `(use PREFIX NAME…)` | a prefix opened for the rest of the file | `(use Init.List)` (same) |
| `trusts` | `(trusts AXIOM…)` | axioms the file's policy admits | `(trusts write_line_output)` (`v3/pins/loader/bin_ok/main.shard`) |
| `type` | `(type NAME (CTOR FIELD-TYPE…)…)`, `(type (NAME T…) …)` | a data type in the logic and in programs | `(type Exp (Num Int) (Add Exp Exp) (Sub Exp Exp))` (`calc.shard`) |
| `record` | `(record NAME (ctor CTOR)? (FIELD TYPE)…)` | a one-constructor `type`, an accessor and an updater per field | `(record Point (x Int) (y Int))` (`v3/pins/loader/record_basic/main.shard`) |
| `inductive` | `(inductive NAME.{u…} BINDERS TYPE (CTOR TYPE)…)` | an inductive type, each constructor with its full type | `(inductive Id.{u} ((α (Type u))) (Type u) (mk (-> α (Id α))))` (`v3/pins/reader/world.shard`) |
| `structure` | `(structure NAME.{u…} BINDERS (FIELD TYPE)…)` | one constructor `NAME.mk` and a projection `NAME.FIELD` per field | `(structure Span () (lo Nat) (hi Nat) (ok (Nat.le lo hi)))` (`v3/pins/loader/struct_proj/main.shard`) |
| `def` | `(def NAME.{u…} BINDERS TYPE (measure …)? VALUE)` | a definition of the logic; no program | `(def as_bool ((a Tok) (b Tok)) Bool (= a b))` (`v3/pins/loader/derive_eq/main.shard`) |
| `abbrev` | as `def` | a definition unfolded first | `(abbrev two2 () Nat two)` (`v3/pins/reader/world.shard`) |
| `opaque` | `(opaque NAME.{u…} BINDERS TYPE VALUE)` | a constant K checks and never unfolds | `(opaque w_output ((w World)) (List ByteArray) List.nil)` (`bin_ok`) |
| `theorem` | `(theorem NAME.{u…} BINDERS PROP PROOF)` | a proven statement | `(theorem twice_eq ((n Nat)) (Eq (twice n) (Nat.add n n)) (exact (twice.eq_1 n)))` (`v3/pins/loader/define_basic/main.shard`) |
| `axiom` | `(axiom NAME.{u…} BINDERS TYPE)` | an assumption, only under `(trusts NAME)` | `(axiom exit_output ((code Int) (w World)) (= (w_output (exit code w)) (w_output w)))` (`v3/examples/calc/calc_bin.shard`) |
| `fn` | `(fn NAME BINDERS RET (measure …)? BODY)` | a definition, its equations and a program | `(fn twice ((n Nat)) Nat (+ n n))` (`define_basic`) |
| `extern` | `(extern NAME BINDERS RET)` | a host operation: an opaque constant and a program head | `(extern exit ((code Int) (w World)) World)` (`v3/std/host.shard`) |
| `realize` | `(realize NAME BINDERS RET (measure …)? BODY (equations THM…)?)`, `(realize NAME (view))` | a program for a definition that exists | `(realize Option.getD (view))` (`v3/std/list.shard`) |
| `derive` | `(derive TYPE CAPABILITY…)` | generated functions for a type | `(derive CNode size)` (`v3/tools/shardfmt/print/cst.shard`) |
| `fulfills` | `(fulfills NAME PROOF)` | the proof of an obligation | `(fulfills skip.dec_1 (by (intro xs acc) (arith)))` (`v3/pins/loader/mutual_three/main.shard`) |
| `bin` | `(bin NAME (entry F) (externs E…) (trusts A…) (requires R…))` | an executable's declaration | `(bin hello (entry main) (externs write_line) (trusts write_line_output) (requires main_output))` (`bin_ok`) |
| `sig`, `requirement` | `(sig type NAME)`, `(sig fn NAME BINDERS RET)`, `(requirement NAME BINDERS PROP)` | the parameters of a view (an interface file `mod.req.shard`) | `(sig fn push ((n Nat) (s Stack)) Stack)` (`v3/pins/loader/view_basic/lib/mod.req.shard`) |

**Binders** are `((x TYPE) …)`. In `def`, `theorem`, `axiom`, `opaque`, `inductive`, `structure`
and inside terms a binder may carry a marker: `(x TYPE implicit)`, `(x TYPE strict)`,
`(x TYPE inst)`. A `fn` binder is `(x TYPE)` only; one whose type is a sort, such as `(a Type)`,
is a type parameter and is implicit at every call: `(fn len ((a Type) (xs (List a))) Nat …)` is
called `(len xs)`. A `type`'s parameters are likewise implicit in its constructors. A function of
no parameters, `(fn sorted_nil () Sorted …)` (`v3/std/list.shard`), is cited by its bare name;
`(sorted_nil)` is read the same.
**Universe parameters** are declared on the name, `NAME.{u v}`, and cited the same way. `fn`,
`extern`, `type` and `realize` take none: in their signatures every type constructor is at level 0.

**`type` and `inductive`.** `type` is the data form: its parameters are types and its fields do
not depend on each other. The loader opens a `type`'s constructors for the whole file, so `(Num
3)` is written bare. The type itself is declared where its form stands: a `fn` above it that names
it is a program without a definition (`RUNNABLE fn F why=e_only_type route=e_first`), and a `def`
there is `unknown_constant`. `inductive` is the general form (indices, dependent constructor types,
universe parameters). For either, beside the type and its recursor, the loader generates and has K
check `T.casesOn`; for a type without parameters or indices, where Init's `Eq` and `And` are
visible, also `T.noConfusionType`, `T.noConfusion` and `T.C.inj` per constructor with fields; and,
the first time a function recurses structurally over it, `T.below` and `T.brecOn`. Each is an
`ACCEPT` record. A `type` with a field that has no type in the logic (`Symbol`; `Int` or `Nat` in
a file with no `(import Init …)`; another such type) exists in programs only: `RUNNABLE type
NAME`, and a `def` or `theorem` naming it is `READ-ERROR … unknown_constant` (pin `type_e_only`).

**`record`** is rewritten, before anything is read, into `(type NAME (CTOR TYPE…))` (CTOR defaults
to `MkNAME`), `(fn NAME.FIELD …)` and `(fn NAME.with_FIELD …)` (the new value first). Two term
forms come with it, for the records of the same file: `(make NAME (FIELD V)…)` — every field once,
any order — and `(with NAME E (FIELD V)…)`. Both also reach a `structure` without parameters.
Refusals are `READ-ERROR`s: `record_form`, `record_make`, `record_with`, and `dependent_update`
when `with` keeps a field whose type mentions an updated one.

**`def`, `theorem`, `axiom`.** A `def` is logical: it never runs. With `(measure (struct x))` or
`(measure M)` it may call itself and is then defined as a `fn` is (below), with its equations,
still without a program (`v3/pins/loader/def_struct/main.shard`); it gets a program from
`realize`. A theorem's PROOF is `(exact TERM)`, `(by STEP…)`, `auto` / `(auto HINT…)` or `sorry`.
`sorry` admits nothing: the record is `PENDING NAME sorry` and a citation of the name is refused
`pending_obligation`. An axiom the file lists under `trusts` is recorded `PARAM NAME axiom`; one
it does not list is refused, `POLICY NAME outside the policy: NAME`.

**`realize`** attaches a program to a definition K already holds, typically one of Init's. The
supplied form states the signature (checked against the erasure of the definition's type) and a
body; one theorem per leaf of the body's leading matches, `NAME.realize_N`, states that the
definition equals the body there, proven by `Eq.refl` or by the theorems an `(equations THM…)`
clause names. `(realize NAME (view))` erases the definition's own value. The record is `REALIZE
NAME supplied|view equations=… pending=…`. Refusals (`REFUSE NAME REASON: …`) include
`realize_kind` (not a definition), `realize_signature`, `realize_recursion`, `realize_descent`,
`recursion_structure` (a recursive value has no readable program: supply the body) and
`no_l_meaning` (the body calls a function that has no definition).

**`fulfills`** has two uses: after a measured function it proves one of its obligations
(`F.dec_N`, below); in the implementation of a view it proves one of the view's `requirement`s.
Naming anything else is a load error that says which: `fulfills_not_obligation`, `fulfills_refused`
(the function was refused above and stated no obligation), `fulfills_outside_impl` (also the
answer when the function stayed a program and stated no obligation, as under `why=wf_unreached`).

**Names and refused forms.** A declaration may not take a name K already holds, nor a name of Init
under the file's module path, and a `def` and a program-only `fn` may not share a name: `REFUSE
NAME name_taken`. A Lean keyword with no form here (`partial`, `mutual`, `termination_by`,
`noncomputable`, `universe`, `deriving`, `open`, `lemma`, `notation`, …) is `READ-ERROR FILE
lean_form` with the spelling to use; `instance`, `class`, `namespace`, `section`, `variable` are
`reserved_form`; anything else is `unknown_form`. In a view, `fn`, `extern`, `realize`, `derive`,
`fulfills`, `bin` are `LOAD-ERROR … view_form`; outside one, `sig` and `requirement` are refused.

## Terms

One term language serves statements, definitions and function bodies. A function body has the
further limits of *Programs* below: a `fun` only as the argument for a function-typed parameter,
no function used as a value.

```
TERM ::= NAME | NAME.{LEVEL…} | @NAME            a variable, else a constant; @ = no binder implicit
       | (TERM TERM…)                            application
       | (fun (BINDER…) TERM) | (forall (BINDER…) TERM) | (exists (BINDER…) TERM) | (-> TERM… TERM)
       | (let ((x TYPE TERM)…) TERM)             sequential; (x TERM) takes the term's type
       | (match TERM (PAT TERM)…)
       | (if TERM TERM TERM) | (dif h TERM TERM TERM)
       | (OP TERM TERM) | (not TERM)
       | (list TERM…) | NUMERAL | "…" | (proj STRUCT i TERM)
       | Prop | Type | (Type LEVEL) | (Sort LEVEL)
PAT   ::= _ | NAME | (CTOR PAT…) | NUMERAL       a NUMERAL only as a whole row
LEVEL ::= NUMERAL | NAME | (succ LEVEL) | (max LEVEL LEVEL) | (imax LEVEL LEVEL)
```

**Names.** A bound name shadows a constant. An unbound name resolves through the file's scope. Two
candidates are `ambiguous_name`, unless the expected type is an inductive type and one candidate
is its constructor, or a type is expected and one candidate is a type whose constructor is the
other. No candidate is `unknown_constant`; when the name is a declaration of Init above the file's
horizon the refusal is `above_horizon` and names the import to write.

**Application, implicit arguments, levels.** A constant's binders marked `implicit` or `inst` are
filled by the elaborator; written arguments go to the explicit binders, left to right. The
unknowns — and the universe levels of a constant cited without `.{…}` — are solved by first-order
unification: the result type against the expected type first, then the arguments, numerals last,
so `(List.cons 0 nil)` at `(List Int)` reads `0` as an `Int`. `@NAME` makes every binder explicit:
`(@List.nil a)`. What the inputs do not determine is refused, never defaulted: `unsolved_implicit`
(write the application with `@`), `unsolved_universe` (write `NAME.{…}`). Too many arguments is
`function_expected`, which names the head as it was resolved — `the head resolved to Add — Init's
declaration; …` where a bare name is Init's and a constructor of the load was meant (pin
`shadowed_head`); infix text such as `(n + 1)` is `infix_form` (write `(+ n 1)`). `Prop` is
`(Sort 0)`, `Type` is `(Sort 1)`, `(Type u)` is `(Sort (succ u))`.

**Instances.** There is no instance search and no `class` or `instance` declaration. An `inst`
binder is filled from a fixed table of Init's instances at `Nat`, `Int`, `Bool` and `List` (the
arithmetic, order, bitwise, `DecidableEq`, `BEq`, `Zero`, `OfNat` and `Append` classes). Anything
else is `instance_needed`: write the application with `@` and the instance (pin `elab_instance`).

**Literals.** A numeral is a `Nat`. Where an `Int` is expected it is `Int.ofNat n`, a negative one
`Int.negSucc`; a negative numeral anywhere else is `negative_numeral`. `"…"` is a `String`; bytes
that are not UTF-8 are `bad_string`. `(list a b …)` builds the list of its expected type and needs
one (`unsolved_implicit` otherwise).
**Coercions.** Three, each inserted where the expected type calls for it: a `Nat` where an `Int`
is expected becomes `(Int.ofNat t)`; a proposition where a `Bool` is expected becomes
`Decidable.decide` with the decision `if` would find; a `Bool` given as an argument where a
proposition is expected (to `Not`, `And`, `iff`, as the condition of `if`) becomes `b = true`. As
a theorem's statement, a binder's type, a side of `->` or the body of a `forall`, a `Bool` is
refused `not_a_type: ?f0 : Bool`: write `(= b true)`. Every other mismatch is `type_mismatch:
expected X, found Y` — an `Int` where a `Nat` is expected included: write a named conversion such
as `(Int.natAbs i)`.

**Operators** are prefix and take their meaning from an operand's type:

| spelling | at `Nat` | at `Int` | at a proposition | at `Bool` |
|---|---|---|---|---|
| `+` `*` | `Nat.add` `Nat.mul` | `Int.add` `Int.mul` | | |
| `-` | `Nat.sub` (stops at 0) | `Int.sub` | | |
| `/` | `Nat.div` | `Int.ediv` | | |
| `mod`, `%` | `Nat.mod` | `Int.emod` | | |
| `<` `<=` | `Nat.lt` `Nat.le` | `Int.lt` `Int.le` | | |
| `>` `>=` | `<`, `<=` with the operands swapped | same | | |
| `min` `max` | `Min.min` `Max.max` | same | | |
| `and` `or` | | | `And` `Or` | `Bool.and` `Bool.or` |
| `not`, `iff` | | | `Not`, `Iff` | `Bool.not` |
| `=` `!=` | `Eq`, `Ne` at any type | | | |

The type that decides: for `+ - * / mod %`, an expected type of `Nat` or `Int` decides and both
operands are read at it, so `(- 1 2)` where an `Int` is expected is `Int.sub 1 2`, that is −1.
Otherwise the first operand's type decides — or the second's when the first is built from numerals
alone, so `(<= 48 c)` is read at `c`'s type. A type with no row is `no_operator_identity`.

No operator is refused at `Nat`: `(- a b)` there is `Nat.sub`, which stops at zero, in the
definition and in its program alike (pin `define_nat_operator`); to go below zero, work at `Int`.
`/` and `mod` are Init's and total: `7 / 0` is 0 at `Nat` and at `Int`, `7 % 0` is 7 at `Nat`,
`-7 mod 0` is −7 at `Int`, under `--run` and in a binary alike. `max` at `Nat` needs
`(import Init Nat.instMax)`, declaration 3,958 of the export, past `UInt8.toNat` and
`Int.lt_mul_ediv_self_add`. Below it the refusal is `above_horizon: Nat.instMax is declaration
3958 of the export, … : write (import Init Nat.instMax)`, a `READ-ERROR` in a theorem and a
`REFUSE` in a `fn` (pins `init_above_instance`, `init_above_instance_fn`). `min` at `Nat`, and
`min` and `max` at `Int`, are in reach of `UInt8.toNat`.
The spellings `lt`, `le`, `int_eq` are `renamed_primitive` (write `<`, `<=`, `=`). Lean's `==`,
`&&`, `||`, `!x`, `∀`, `→`, `≤` and the like are `lean_symbol`, each with its spelling here; a
name ending in `!` (`List.get!`) is `inhabited_default`.

**`if`.** `(if C T F)` takes three kinds of condition: a `Bool`, read as `C = true`; a proposition
whose decision the elaborator knows — `<` and `<=` at `Nat` and `Int`, `=` at `Nat`, `Int`, `Bool`,
and `=` at a type with exactly one visible derived equality (`(derive T eq)`); a value of an
inductive type with exactly two constructors, the second taking the then-branch (`Yes` of
`(type Dec (No) (Yes))`, `some`, `cons`), under the evaluator and in a native binary alike.
Otherwise: `no_decision` (write `(@ite C DEC T F)`, or derive equality), `ambiguous_decision`,
`if_type` (not two constructors), `private_if` (a view's `sig type`). `(dif h C T F)` takes a
`Bool` or a proposition and binds `h` to the condition's proof in T and to its negation's in F.

**`match`.** The scrutinee's type must be an inductive type without indices. The match takes its
type from its position; where the position gives none it is `match_motive` — bind it: `(let ((r
TYPE (match …))) r)`. Rows are tried in order. A row that matches every value — `_`, or a NAME
that is no field-less constructor of the scrutinee's type — is the last row: with rows after it
the match is `unreachable_arm`, which names the row and, for a name, says that no constructor of
that name is in scope. A misspelt constructor is such a name, and its row would take every value
(pins `match_dead_row`, `match_dead_row_def`, `ev_dead_row`). Patterns:

| pattern | meaning |
|---|---|
| `_` | anything |
| NAME | the scrutinee type's field-less constructor of that name if there is one, else a variable |
| `(CTOR PAT…)` | a constructor on patterns for all its fields, nested to any depth |
| NUMERAL | at `Nat` or `Int`, as a whole row: the rows are read as a chain of `(if (= x K) …)` and must end in a `_` or NAME row |

Not allowed in a `def` or a statement: a numeral under a constructor, a string, a `(quote X)` —
`literal_pattern` (bind the field and compare with `if`). In a `fn` a numeral under a constructor
is not refused: the function is a program without a definition (`RUNNABLE fn F why=no_l_identity
route=e_first`); a string row there is `pattern_literal`. Numeral rows with no closing row —
`match_not_exhaustive: numeral rows cover no type: close the match with a `_` row or a name`;
numeral rows beside constructor rows, or a row after the catch-all — `literal_rows`; a numeral row
at another type — `pattern_type`. A constructor left uncovered — `match_not_exhaustive: a
constructor no row covers: add the case or a `_` row`; a wrong head — `pattern_constructor`; a
wrong field count — `pattern_arity`. In a `fn`, a `Nat` is not matched by its constructors:
`(match n (Nat.zero A) ((Nat.succ k) B))` is `REFUSE F unknown_head: Nat.succ is not a
constructor — …`, though a `def` accepts it; a program takes a `Nat` apart by numeral rows or by
`if`. When the scrutinee is a variable
that the expected type or other variables' types mention, each row is read with its pattern in
the variable's place, so a row can be typed or refuted by its constructor (pin `dependent_match`).

## From a function to a definition

`(fn NAME BINDERS RET (measure …)? BODY)` is elaborated once, as a term of the logic over its
binders and over a variable standing for its own name. Two things are made from that one term: a
**definition** K checks, with **equations** `NAME.eq_N` as theorems; and a **program**, the term's
erasure (see *Programs*). The record is `DEFINE NAME equations=NAME.eq_1,…`. When only the program
can be made the record is `RUNNABLE fn NAME why=REASON route=ROUTE`; when the function is wrong it
is `REFUSE NAME REASON: MESSAGE` (both are tabled at the end of this section). A function may call
one declared later in the file: it waits, and is defined as soon as its callee is (pin
`define_forward`).

### Recursion

A function that calls itself says why it terminates, by one of two clauses.
**Structural: `(measure (struct x))`**, `x` a binder. Each self-call must pass, in `x`'s position,
a proper part of `x`: a variable bound under a constructor by a `match` on `x`, at any depth. The
other arguments are free (an accumulator may change). The definition is built on `T.brecOn`.

```
(fn sum_acc ((xs (List Nat)) (acc Nat)) Nat
  (measure (struct xs))
  (match xs (List.nil acc) ((List.cons x t) (sum_acc t (+ acc x)))))     ; …/define_basic/main.shard
```

It works on an inductive type whose recursor has one motive and eliminates into `Type`: Init's
`List`, `Option` and the like, and a `type` of the program. It does not work on a *nested* type —
one with a field that applies another type to it, `(type Tree (Leaf Nat) (Node (List Tree)))` —
which is refused (`equation_form`, below): use a measure and `(derive Tree size)`.

**By measure: `(measure M)`**, `M` a `Nat` over the binders — for an `Int` argument write
`(Int.natAbs n)` (pin `measure_natabs`); `M` may cite a function that has no program, such as a
derived size. The definition is built on `WellFounded.fix`. Every self-call states an
**obligation** `NAME.dec_N`: the measure at the call's arguments is less than at the parameters.

```
(fn count ((n Nat) (acc Nat)) Nat (measure n) (if (= n 0) acc (count (Nat.sub n 1) (+ acc 1))))
;; count.dec_1, writing p0 for (proj PProd 0 p): for all p and h : (Not (= p0 0)), (< (- p0 1) p0)
(fulfills count.dec_1
  (exact
    (fun ((p (PProd.{1 1} Nat Nat)) (h (Not (= (proj PProd 0 p) 0))))
      (@Nat.sub_lt (proj PProd 0 p) 1 (Nat.pos_of_ne_zero h) (Nat.zero_lt_succ 0)))))
                                                   ; v3/pins/loader/discharge_measure/main.shard
```

- *Statement.* A `<` at `Nat`, under every variable in scope at the call, outermost first: the
  parameters — one data parameter stands under its own name, several stand as a single tuple `p :
  PProd …` with components `(proj PProd 0 p)`, `(proj PProd 1 p)`, … —, then pattern variables and
  `let`s, and the facts the enclosing branches give: under an `if`, `h : C` in the then-branch and
  `h : ¬C` in the else-branch; under a `match` row, `h : scrutinee = pattern`. Left out is what
  states the function's own values: a `let` bound to a recursive call, a `let` computed from one,
  and a fact about either — so a call in the scope of such a `let` has its obligation without it
  (pins `define_let_call`, `mutual_let_call`). A call whose *argument* cites such a `let`, or
  holds a recursive call, is nested recursion and gets no definition: `REFUSE NAME
  nested_recursion` (pin `define_nested`).
- *Numbering.* From 1, in the order the body is walked: arguments left to right, the rows of a
  `match` in order, and for an `if` the condition, the else-branch, then the then-branch. Two calls
  that state the same fact share one obligation. The records: `PARAM NAME.dec_N obligation` for
  each, then `DEFINE NAME equations=… pending=NAME` and `PENDING NAME measure`.
- *What pending means.* The function is defined and can be cited, run and reasoned about. K has
  checked it with each obligation standing as an unproven assumption, and the account is open:
  every `ACCEPT` record of a theorem that rests on the function lists the obligation under
  `axioms=` and `params=`, and every `DEFINE` or `REALIZE` of a caller lists the function under
  `pending=`. A pending obligation does not fail the load.
- *Discharge.* `(fulfills NAME.dec_N PROOF)`, after the function, PROOF as in a `theorem`. K
  checks it as the theorem `NAME.dec_N.proof`; the record is `DISCHARGE NAME.dec_N proved`. Every
  record that named the obligation is rewritten to name what the proof rests on, and once the
  function has no open obligation its `PENDING` line and every `pending=NAME` are removed. A
  proof that rests on the obligation it proves is `REFUSE NAME.dec_N fulfills_cycle`; a second
  discharge is `fulfills_twice`. To read an obligation's statement, give it `(by sorry)`: the
  record `PENDING NAME.dec_N.proof sorry goals=1 […]` shows the goal, and the load goes on. The
  word `sorry` alone prints `PENDING NAME.dec_N.proof sorry`, with no goal.

### A group of functions that call each other

Functions of one file that call each other in a cycle form a *group*. There is no keyword: the
loader finds a group. A function whose callee is not yet declared waits; after each function is
read the loader looks whether a waiting function's wait leads back to itself, reads every waiting
function it reaches with the others standing as variables, and settles the smallest cycle among
them. So a group is defined when its last member has been read, and later forms can cite it.

A group is compiled into **one** definition by well-founded recursion, on three conditions: every
member has `(measure M)` with `M` a `Nat` over its own binders (`(measure (struct x))` does not
count); no member has a type parameter; no member's result type mentions its parameters.

```
(fn ping ((n Nat)) Nat (measure n) (if (= n 0) 0 (pong (Nat.sub n 1))))
(fn pong ((n Nat)) Nat (measure n) (if (= n 0) 7 (ping (Nat.sub n 1))))
(fulfills ping.dec_1 (by (intro n h) …))             ; v3/pins/loader/mutual_measure/main.shard
```

For members `f_1 … f_n` in file order the loader generates, all under the *first* member's name:

| name | what it is | record |
|---|---|---|
| `f_1.args` | an inductive type with one constructor per member, `f_1.args.of_K`, whose fields are member K's data binders | `ACCEPT` |
| `f_1.mutual` | the one recursive function, from `f_1.args` to the member's result type | `ACCEPT` |
| `f_1.mutual.eq` | the theorem that unfolds `f_1.mutual` once | `ACCEPT` |
| `f_i` | each member: `f_1.mutual` at its constructor | `DEFINE f_i equations=… pending=…`, `PENDING f_i measure` |
| `f_i.dec_N` | one obligation per call from member `i` to a member, itself included | `PARAM f_i.dec_N obligation` |
| `f_i.eq_N` | the member's equations, calls to members left as calls | `ACCEPT` |

An obligation here compares the *callee's* measure at the arguments with the *caller's* at its
parameters, is numbered in the caller's own count, and is stated over the caller's binders by name
(no tuple). A call that hands its argument on unchanged is paid for by a rank in the measures:
`(measure (+ (+ (len xs) (len xs)) 2))` for the caller against `(+ (+ (len xs) (len xs)) 1)` for
the callee (pin `mutual_three`). Each member lists under `pending=` every member that still has an
open obligation. The members' programs are their own bodies, calling each other directly.

A group that does not meet the conditions is not an error: every member is `RUNNABLE fn … why=…
route=typed` with the reason, and so is everything that calls them.

### `derive`

`(derive TYPE CAPABILITY…)` generates functions for a closed inductive type — no variables in it,
applied to types only, such as `Tok` or `(List Tok)`. The generated functions are source forms the
loader reads at that point as if the file held them, each with its own `DEFINE` record (`size`,
whose functions are `def`s, prints `ACCEPT` records only: one per function and one per equation);
then the procedure is registered: `DERIVE PROC type=T cap=C policy=P fields=…`. `eq` on a type of
two or more constructors registers two, `cap=idx` for `NS.ctorIdx` and then `cap=eq`. NS below is
the type's name with its arguments joined by `_`: `Tok`, `List_Tok`, `Prod_Nat_Tok`.

| capability | generates | used for |
|---|---|---|
| `eq` | `NS.decEq : (a b : T) → Decidable (a = b)` and, for two or more constructors, `NS.ctorIdx` | `(if (= a b) …)` and `=` as a `Bool` at T |
| `(ord structural)` | `NS.compare : T → T → Ordering`: constructors in declaration order, fields left to right; `NS.ctorIdx` | fields of later derivations |
| `render` | `NS.renderOnto : T → List Nat → List Nat` and `NS.render : T → ByteArray`, the value as canonical source text | printing; import `v3/std/derive.shard` |
| `(eq by F)`, `(ord by F)`, `(render by F)` | nothing: registers the file's own `F` after checking that its type is the capability's | as the generated one |
| `size` | `NS.size : T → Nat`, 1 plus the sizes of the recursive fields, with `NS.size.eq_K` per constructor; for a nested type also one per auxiliary type (`List_T.size`) | measures |

`size` is written alone, `(derive T size)`, and is the only capability that takes a nested type.
Its functions are `def`s: they have no program. For the others, a field's procedure is the
built-in one at `Nat` and `Int` (and `Bool` for `eq`), the function itself at a recursive field,
else the one derivation visible at the field's type — so derive the field types first. A
derivation is visible wherever its module is. Refusals are `REFUSE MODULE.derive REASON: …`:

| reason | cause |
|---|---|
| `derive_type` | not a type here; `Nat` or `Int` (register a procedure with `by`); a proposition; not an inductive type; not closed; an argument that is a value, as in `(Fin 3)` |
| `derive_shape` | an indexed family; a mutual type; a nested type (for `eq`, `ord`, `render`); no constructors; a field that is a proof or a type, or whose type depends on an earlier field (supply the procedure with `by`) |
| `derive_field` | a function-valued field; no derivation at a field's type (`derive it first — (derive T CAP)`); two visible |
| `derive_needs` | the generated source cites a constant the file cannot see: raise the Init import; for `eq` on a program's `type` with parameters, `C.inj` is not generated |
| `derive_policy` | bare `ord`, or a policy other than `structural` |
| `derive_duplicate`, `derive_by`, `bad_shape` | derived already; `F` has another type or universe parameters; an unknown capability, or `size` beside others |

A generated name that is already declared is `REFUSE NAME name_taken`; a generated function that
was not defined ends the form with `derive_failed`.

### Equations

`NAME.eq_N` are the facts a proof uses to unfold a function: `(rw len.eq_2)`, `(exact (twice.eq_1
n))`. A body with no leading `match` has one, `NAME.eq_1`, stating `NAME binders = BODY`. A body
that begins with a `match` on a parameter (and matches directly inside its rows) has one per leaf
of the case tree, numbered from 1 in constructor order; a catch-all row gives one equation per
constructor it covers. An `if` is not split. Each states `NAME lhs = rhs` with the constructor in
the split parameter's place; its binders are the parameters not split, then the constructor's
fields: `len.eq_2` states `len (List.cons x t) = 1 + len t` and is cited `(len.eq_2 x t)`.

- A structurally recursive function's equation is stated only where K decides it by computation. A
  leaf in which the structural argument is still a variable has no equation and its number is
  skipped; prove that case by `cases`.
- A measured function's and a group member's equations are all stated, proven through
  `WellFounded.fix_eq`; a `match` in them carries its equation, as in the obligations.
- No equations are generated where Init's `Eq` is not visible, nor for a measured function where
  `WellFounded.fix_eq` is not; the `DEFINE` record then shows `equations=` empty.

### When a function is only a program, or refused

`RUNNABLE fn NAME why=REASON route=ROUTE` — the function runs, has no definition, and no theorem
can name it. `route=typed`: the body was elaborated and the program is its erasure.
`route=e_first`: the body could not be read as a term of the logic and was read as a program.

| `why=` | route | cause | what to do |
|---|---|---|---|
| `callee_runnable` | typed | the body calls a function that is itself only a program | give the callee a definition; this one follows |
| `measure_type` | typed | `(measure M)` and `M` is not a `Nat` | write a `Nat` measure, e.g. `(Int.natAbs n)` |
| `group_measure` | typed | a member of a group has no measure, or `(measure (struct x))` | `(measure M)` on every member |
| `group_shape` | typed | a member of a group has a type parameter, or a result type over its parameters | make the members monomorphic and non-dependent |
| `wf_unreached` | typed | `WellFounded.fix`, `InvImage.wf`, `Nat.lt_wfRel` (for a group also `WellFounded.fix_eq`) lie above the file's Init horizon | `(import Init WellFounded.fix_eq)` or later |
| `below_unreached` | typed | structural recursion over an Init type whose `below` lies above the horizon, or over a `type` where `PProd`, `PUnit` do | raise the Init import |
| `no_l_meaning` | e_first | the body calls something whose signature the logic cannot read: an `e_only_type` function, an extern without a constant, such a `sig fn` | give the callee a signature the logic reads |
| `no_l_identity` | e_first | the body uses something with no form in the logic: `(quote X)`, a numeral under a constructor pattern, a bit-level primitive (`band`), a constant above the Init horizon | rewrite the construct; raise the import |
| `e_only_type` | e_first | the signature cannot be read: the file has no `(import Init …)`, or a binder is `Symbol`, a program-only `type`, or a `type` declared later in the file | import Init; declare the type first; the normal state of a file that sees no Init |
| `callee_undeclared` | e_first | at the file's end the function still waits on a function of the file that never got a declaration, because that one was refused | mend the refusal of the callee above |

`RUNNABLE extern NAME` and `RUNNABLE type NAME` are the ordinary records of an extern and of a
program-only type; `RUNNABLE extern NAME why=REASON` is an extern that got no constant.

`REFUSE NAME REASON: MESSAGE` — nothing is declared. The reasons that come from turning a function
into a definition:

| reason | message | what to do |
|---|---|---|
| `realize_recursion` | `NAME calls itself without a measure` | add `(measure (struct x))` or `(measure M)` |
| `realize_descent` | `NAME: a recursive call does not pass a proper subterm of the structural parameter — ARG` | pass a pattern variable of a match on the parameter, or use `(measure M)` |
| `realize_descent` | `NAME calls itself on its own parameters — no measure decreases` | change an argument |
| `realize_descent` | `NAME: a recursive call at other type arguments (polymorphic recursion)` | keep the type arguments |
| `bad_measure` | `X is not a parameter` | `(struct x)` names a binder |
| `untyped_subterm` | `the structural parameter's type is not an inductive type` | use `(measure M)` |
| `equation_form` | `T: a nested inductive's recursor has a motive per auxiliary type, and structural recursion takes one — write a Nat measure: (derive TYPE size) gives a size to measure by`; `T eliminates into Prop only`; `T has no recursor` | `(measure (T.size x))` after `(derive T size)` |
| `function_value` | `NAME: its own name is not applied to its structural argument (no function values)`; `… not applied to all its arguments …`; `the function's own name in a type` | apply the function fully |
| `nested_recursion` | `NAME: the argument of a recursive call is computed by the recursion itself — it cites X, a let bound to the result of a recursive call`; `… — a recursive call stands inside it` | pass an argument computed without a recursive call; nested recursion gets no definition |
| `definition_refused` | `NAME: K refused the definition (WHY)`; for a group also `… the group's argument type`, `… the group's fixpoint`, `… the fixpoint's unfolding` | a defect of the definer: report it |
| `define_matrix`, `define_depth`, `define_loop` | internal shapes; `a body too deep`; `the auxiliaries did not settle` | split the function; report |
| `name_taken` | `NAME is an admitted constant: realize it, or declare another name` | rename, or `realize` |

Two more families refuse a `fn`: any refusal of the elaborator on the signature, the measure or
the body (*Terms*) — `type_mismatch`, `match_not_exhaustive`, `unsolved_implicit`, … — and any
refusal of the erasure (*Programs*) — `function_value`, `erased_in_runtime`,
`recursion_structure`, `noncomputable`. When a body falls to `route=e_first` and is refused there
too, the record carries both: `REFUSE NAME REASON: MESSAGE — read E-first after the typed route's
REASON2: MESSAGE2`. A refused group gives every member the same record.

## Programs

### What a program is

The program of a load is a list of declarations under their full names: an *inductive* (from a
`type`, or an Init type on first use) with its constructors' runtime field types; a *function*
(from a `fn`, a `realize`, an instance of a template, a lifted `fun`) with parameter types, result
type, recursion clause and body; an *extern*, a signature the host performs; and a view's `sig fn`
or `sig type`, linked to the implementation's function or type. A body is built from ten term
forms: a variable, a literal, a constructor applied to its fields, a call of a function, a
primitive operation, an extern call, `match`, sequential `let`, `if`, and *absurd* (a row proven
unreachable). Every call is saturated and its head is known at load time. Values are constructor
cells, unbounded integers and symbols: `Nat` and `Int` are both the unbounded integer. `if` takes
the then-branch when the condition's value is the *second* constructor of its type (`Bool.true`,
`Decidable.isTrue`).

A program is run from an entry with `load.shard … --run MODULE.FN FILE -- ARG…`. A run ends with
the program's own exit, or `RUN: out of fuel in F` (exit 3), `RUN: stuck REASON in F` (4), `RUN:
link …` (5), `RUN: argument N …` (6); a failed load exits 2 and nothing runs.

### Erasure, and what has no program

The program of a `fn` is the erasure of its elaborated body. Erased, leaving nothing at run time:
types and type arguments; proofs — any term whose type is a proposition, a binder that is one, a
`let` that binds one; universe levels. Kept: constructors and calls at their runtime arguments.
Rewritten: a generated matcher becomes a `match` on its rows; `ite` and `dite` become `if` on the
decision; a case analysis of a two-constructor type that ignores the fields becomes `if`; an
operator becomes its primitive; a string literal becomes its bytes; Init's `absurd` becomes
*absurd*. Some Init structures are represented by their one runtime field and have no cell of
their own: `Fin`, `Subtype`, `BitVec`, `UInt8`, `Array`, `ByteArray`, `String`.

A type can appear in a program when it is `Nat`, `Int`, one of those structures, a view's `sig
type`, a type parameter, or an inductive type with no indices, not a proposition, whose parameters
are types or propositions and whose fields are such types or propositions.

**What has no program:** a `def`, a `theorem`, an `axiom`, an `opaque` constant, a derived `size`,
and every function of Init that is neither a primitive nor given a body by a `realize`. A measure
may cite one freely:
a measure is never run. A function body that calls one in a runtime position cannot be erased:
`no_realization: NAME: no realization attached — realize it first, or import the file that does`. The body is then read
as a program, where the name is unknown, so the usual record is `REFUSE … unknown_head … — read
E-first after the typed route's no_realization …`. `v3/std/list.shard` realizes the list
functions: import it. `Classical.choice`, `Acc.rec` and the `Quot` primitives in a runtime
position are `noncomputable`; a recursor, `T.brecOn` or `T.below` of an inductive type, or
`WellFounded.fix`, written in a body is `recursion_structure` — the generated constants, never a
function that only bears such a name (pin `fn_named_below`); a type or a proof in a runtime
position is `erased_in_runtime`.

### Functions as arguments: templates

A program has no function values. A `fn` (or a supplied `realize`) with a function-typed binder is
a **template**: it is defined in the logic like any function — `TEMPLATE NAME static=f
equations=…` — and has no program of its own. Each call of it takes an **instance**, a separate
function specialised to the *shape* of each function argument:

| argument written | shape | example |
|---|---|---|
| a function's name | that function | `main.inc/0` |
| a named function applied to some of its arguments | that function and the count of arguments given | `main.addk/1` |
| `(fun (…) …)` | the lambda, lifted to a function `CALLER.lam_N` whose leading parameters are the variables it uses | `main.add_off.lam_1/1` |
| a parameter of the enclosing template, passed on | the shape the enclosing instance was given | |

```
(fn twice ((f (-> Int Int)) (x Int)) Int (f (f x)))
(fn add_off ((off Int) (x Int)) Int (twice (fun ((y Int)) (+ y off)) x))
;; TEMPLATE main.twice static=f …    DEFINE main.add_off.lam_1 equations=
;; INSTANCE main.main_twice.of.main_add_off_lam_1_1 of=main.twice shapes=main.add_off.lam_1/1 …
                                                    ; v3/pins/loader/template_lift/main.shard
```

The values a shape captures are passed at run time and are not part of the instance's name, so
`(addk 3)` and `(addk k)` share one instance. In the logic an instance is the template applied: a
theorem about the template applies to it, and it carries the template's equations. A `structure`
with a function-typed field is a *package*: a constant of it (a `def`) is passed like a function's
name and its fields are resolved in the instance (pin `template_package`).

Refused, each `function_value` unless noted: a `fun` anywhere but as such an argument (a `let`, a
field, a result); a function-typed parameter used other than by calling it at its full arity or
passing it on; a template called with fewer arguments; a template calling itself at other shapes
or type arguments (`realize_descent`); more than 1,024 instances of one template in a load, or
`--instance-budget N` (`specialization_budget`).

### `World` and the use check

Effects go through a token. A **World type** is the type of the last parameter of an `extern`,
when that type is declared by a module (`std.host.World`, or a program's own `(type World (World
Nat))`). A value of a World type is a *token*; a value whose type contains a World type — a pair
with one, a `type` with such a field — is a *box*.

The rule: **along every path through a function's program, each token and each box is used at most
once.** It is checked on the erased program, for every function that has a World type in its
signature or calls an extern, and recorded `THREAD NAME ok` or `THREAD NAME REASON: DETAIL`.

- An `extern` uses its World argument and yields a fresh token (its result, or the one inside the
  pair it returns). A call uses a token passed where the callee's parameter is a World type, and
  a box passed where the parameter's type contains one; its result is fresh.
- Building a constructor around tokens uses them and yields a box. Matching a box uses the box and
  yields its fields fresh — so unwrapping one pair twice is a second use.
- The arms of a `match` and the branches of an `if` are separate paths. `let` is sequential; a
  `let` of a variable is another name for the same token.
- Returning a token is a use. Leaving one unused is allowed. A reference inside a proof does not
  count: proofs are erased before the check.

| verdict | cause |
|---|---|
| `world_twice` | a second use: `parameter 2 used twice: std.host.write_line, std.host.write_line` |
| `world_forged` | a World built by its constructor: `the token comes from the entry` |
| `world_unsupported` | ownership the check cannot follow: a token at a parameter that is not a World type (a type parameter included), at a primitive, as an `if` condition, `the World itself matched`, passed to a function the program lacks |

A verdict other than `ok` is a report on its own; it fails the load when the function is in the
closure of a `bin`.

### `extern` and `opaque`

`(extern NAME BINDERS RET)` declares an operation the host performs. In a program it is an extern
call. In the logic it is an *opaque* constant of the declared type: K never unfolds it and it has
no equations, so a function that calls it is defined as usual and theorems can speak about the
call. Records: `ACCEPT NAME …` then `RUNNABLE extern NAME`. The constant needs a value to exist;
the loader builds one from the result type (for an inductive type, the first constructor whose
fields can all be built). A type with none — a proposition, a sort — is `extern_uninhabited`; that
extern, one whose signature the logic cannot read, and any extern of a file without Init get no
constant, and their callers are programs only (`no_l_meaning`). The host's operations are declared
in `v3/std/host.shard`: `get_args`, `read_file`, `write`, `write_line`, `write_file`, `exit`.

`(opaque NAME BINDERS TYPE VALUE)` is the logic's own form of the same idea: VALUE shows the type
is inhabited and is never looked at again; there is no program. Its use here is to name what can
be observed of a World — `(opaque w_output ((w World)) (List ByteArray) List.nil)` — with axioms
stating what each extern does to it.

### `bin`

`(bin NAME (entry F) (externs E…) (trusts A…) (requires R…))` declares an executable; `entry` is
required and appears once. The form is checked at the end of its file:

| check | failure line |
|---|---|
| F is one `fn` in scope, its last parameter a World type, every other parameter `Int`, `Nat`, `ByteArray`, `(List Nat)` or `(List Int)` | `BAD-BIN NAME bad_entry: F does not take a World last` |
| every name under `externs` is an extern in scope | `BAD-BIN NAME extern_undeclared: …` |
| every extern called by a function reachable from F is listed | `BAD-BIN NAME extern_outside: X is called in G and is not in the roster` |
| every name under `requires` is an accepted theorem | pending (`sorry`): `UNMET R`; unknown: `BAD-BIN NAME unknown_requirement: …` |
| every name under `trusts` is an axiom in scope | `BAD-BIN NAME unknown_trust: …` |
| the requirements and the reachable functions' definitions rest only on Init's three standard axioms, the load's parameters (a view's; a pending obligation) and the listed axioms | `BAD-BIN NAME untrusted: D rests on A, which the bin does not trust` |
| every reachable function is `THREAD … ok` | `UNTHREADED G REASON: DETAIL` |

The record is `BIN NAME entry F externs N trusts N requires MET/TOTAL`, for example `BIN main.hello
entry main.main externs 1 trusts 1 requires 1/1`. Every failure line is an error: the load fails.
A requirement is a theorem about the entry, usually about an opaque observable of the World the
entry returns. By practice the `bin` and its requirements stand in a file of their own beside the
program (`v3/examples/calc/calc_bin.shard`), so importing the program does not run these checks.
`v3/build.sh` never reads a `bin` form: it compiles the `main` of the file it is given, so a
binary is the program a contract speaks of only when it is built from the file that declares the
contract's entry. An entry with parameters before the World passes the first check and is refused
by the build of a native binary ("As a native binary").

### Effect traces

`--run … --trace FILE` writes one line per host call, in order: the extern's short name, its
arguments as the host saw them, and the answer where it carries data — `get_args "a" "b"`,
`read_file "p" -> none` or `-> (some "…")`, `write "…"`, `write_line "…"`, `write_file "p" "…" ->
true`, `exit 3`. Bytes are quoted with `\"`, `\\`, `\n`, `\t`, `\r`, and `\xHH` for any other byte
outside 32–126. A model of the program can produce the same lines; `v3/kernel/test/bin_test.sh`
compares the two files byte for byte.

## Running a program

A *program* is what a clean load yields: the `fn`, `type` and `extern` declarations of the loaded
files, every name resolved. An *entry* is a `fn` whose last parameter is the World (the token
that orders effects) and whose result is the World. The evaluator `ev` (`v3/kernel/ev.shard`) is
the definition of "run". A program runs under it from the command line or through a prepared
handle, or as a native binary, which is held to `ev` by tests.

### Under the evaluator: the loader's `--run`

```sh
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)      # the Init export and its index
rust_bootstrap/target/release/eval direct v3/kernel/load.shard --root v3 \
  --init $EXPORT --init-index $INDEX \
  --run examples.path.path_main.main v3/examples/path/path_main.shard -- 2
```

This loads the file, then runs `(fn main ((i Nat) (w World)) World …)` and prints `4`
(`v3/kernel/test/path_test.sh`). `v3/bin/load ARGS…` is the same command on the compiled loader.
The two differ at one edge: an integer past 2^3840 — in a run, or in a theorem K evaluates that
far — ends the compiled loader with `rt trap: bignum too large (exceeds 2^3840)`, exit 3, where
the loader on the bootstrap goes on. Nothing is printed but the program's own output.

| option | meaning |
|---|---|
| `--run MODULE.FN` | the entry, by its full name |
| `-- ARG…` | the entry's arguments; the whole list is also what `get_args` answers |
| `--fuel N` | one unit is spent each time a function is entered; default 1,000,000,000,000 |
| `--trace FILE` | one line per host call is written to FILE: `write_line "3"`, `read_file "p" -> none` |
| `--lowered` | run the back end's restricted form of the program in its place (a test of the lowering) |

The parameters before the World are read from the arguments in order and validated before the
program starts: `Int` (`-7`), `Nat` (`7`), and the byte-list types — the toolchain prelude's
`(List Int)`, Init's `(List Nat)` and `(List Int)`, `ByteArray` — which take the argument's bytes.
An entry with the World alone takes any arguments and reads them through `get_args`.

| exit | line printed | meaning |
|---|---|---|
| 0 | — | the entry returned |
| CODE | — | the program called `exit CODE` |
| 2 | the load's records, then `LOAD: …` | the load failed; nothing ran |
| 3 | `RUN: out of fuel in F`, `RUN: exhausted nat_size in F` | fuel, or a `Nat` result past 2^27 bytes (`nat_count`: a shift or power count past 32 bits) |
| 4 | `RUN: stuck REASON in F` | `no_arm` (no match arm), `guard` (a toolchain primitive outside its domain, in a file that sees no Init: the bare `/` by zero, `bshl` or `bshr` by a count outside 0–63; in an ordinary file `/` and `mod` are Init's and total, and `Nat.shiftLeft`, `Nat.shiftRight` are computed at a count of 64 and more), `if_tag`, `extern` (an extern the host cannot perform at its declared types), `unlinked` (a `sig fn` whose implementation was not loaded), `unreachable` |
| 5 | `RUN: link REASON: TEXT` | the entry cannot be prepared: `unknown_head`, `unlinked`, `bad_entry` (no World last; a parameter type no argument supplies), `preconditioned`, `lowering` |
| 6 | `RUN: argument N REASON: TEXT` | `argument_count`, `not_an_int`, `not_a_nat`; e.g. `RUN: argument 1 not_a_nat: x` |

`preconditioned` refuses an entry whose arguments cannot be validated in full: a parameter that is
evidence (`(h (Nat.lt n 10))` in `guarded`, `v3/pins/session/data.shard`), a constructor with a
proposition field, a type parameter, `Fin`, `BitVec` or `Subtype`.

### As a native binary

```sh
v3/build.sh v3/examples/calc/calc_main.shard /tmp/calc --root v3     # needs a current Init index
/tmp/calc "1+2" "10 - 3"
```

The binary starts at `main` of the root file's module, called on the World alone; its command-line
arguments arrive through `get_args`. A `main` with parameters before the World is a `--run` entry
only: the emit refuses it (`EMIT: MODULE.main takes N parameters: a binary's main takes the World
alone (a main with parameters before the World runs under the loader's --run only)`), and
`v3/build.sh` prints `FAIL: NAME: the emit (exit 2)` with that line. The other refusals of the emit
are listed under "The back end" in KERNEL.md: among them a constructor of more than 125 fields, and
a constructor named `True`, `False`, `Nil`, `Cons`, `None`, `Some`, `Pair` or `World` at another
place in its type than the runtime's cell of that name (`True`, `Cons`, `Some` second; the others
not second). There is no fuel. Exit status: 0 when `main` returns, the program's `exit` code, 3 on a
runtime trap (`rt trap: MESSAGE` on standard error; the traps are listed under "The back end" in
KERNEL.md). With `SHARD_STATS=1` the binary prints call, allocation and collection counts to
standard error.

Wherever the output path is, the build also writes its log to `v3/bin/NAME.build.log` and the
generated C to `v3/bin/emit/NAME/`, NAME being the output path's last component. On a C compiler
failure the console shows `FAIL: NAME: cc:` with the compiler's first error line, and the path of
the log. A source edited while its build runs fails the build (`FAIL: NAME: changed during the
build: FILE…`) and no stamp is written (KERNEL.md, "Build and gates").

A binary is held to the evaluator by tests, not by proof, and differs from it at known edges
(`OPEN.md` §1). An integer of 2^3840 or more traps, so a `Nat.shiftLeft` to such a result does. A
program stuck under the evaluator (exit 4) traps (exit 3).

### Prepared handles and sessions

A host written in shard runs programs in its own process, with no command line between steps.
A *handle* (`ev.shard`) is a program linked once with one entry bound; it is invoked any number of
times on arguments built as data. A *session* (`session.shard`) owns a table of *revisions* (each
one load, or an earlier revision extended by declarations built as data) and a table of handles.
A client holds ordinals, never the values. `v3/kernel/test/session_test.shard` is the worked host.

| operation | answer |
|---|---|
| `(policy_of DOOR EFFECTS TRACE FUEL)` | a policy: `Checked` validates every argument against its parameter's type and refuses a `preconditioned` entry; `Assumed` takes the caller's word and records what was assumed |
| `(prepare PROG ENTRY POLICY REV)`, `(invoke H ARGS)`, `(invoke_run H ARGS ARGV W)` | a handle on a program alone; a run without effects (`InvRan`, `InvArg`, `InvNo`); a run with effects |
| `(ses_start EXPORT INDEX)` | a session |
| `(ses_introduce S ROOT FILES W)` | the files loaded as a new revision: `IntroOk N`, or the load's records |
| `(ses_extend S REV MODULE DECLS VIEWS)` | a new revision: REV plus declarations built as data |
| `(ses_prepare S REV ENTRY POLICY)` | `PreparedOk H`, or `PreparedNo REASON TEXT` |
| `(ses_invoke S H ARGS)`, `(ses_run S H ARGS ARGV W)` | the handle's run; `ses_invoke_at` / `ses_run_at` also require a revision |
| `(ses_release S H)`, `(ses_revoke S H WHY)`, `(ses_withdraw S REV WHY)` | the handle gone; every handle of REV and of the revisions extending it revoked |

A handle keeps running the revision it was prepared on, whatever revisions come after. The
refusals: `unknown_revision`, `withdrawn`, `unknown_handle`, `released`, `revoked`, `revision`
(the invocation requires another one), `policy` (the handle was prepared for the other kind of
run). Limits: no host in another language; fuel is the only budget; no revision is ever removed.

### Host calls

A program reaches the host through `extern` declarations; the runner finds the host's operation
by the extern's short name and builds and reads each value by the declared type. The six of
`v3/std/host.shard`, over `(type World (World Nat))`:

| extern | signature | effect |
|---|---|---|
| `get_args` | `((w World)) (Prod (List ByteArray) World)` | the command-line arguments |
| `read_file` | `((path ByteArray) (w World)) (Prod (Option ByteArray) World)` | a file's bytes; `none` when it cannot be read |
| `write` | `((bytes ByteArray) (w World)) World` | the bytes to standard output |
| `write_line` | `((bytes ByteArray) (w World)) World` | the bytes and a newline |
| `write_file` | `((path ByteArray) (bytes ByteArray) (w World)) (Prod Bool World)` | a file written whole; whether it succeeded |
| `exit` | `((code Int) (w World)) World` | the process ends with the code |

The toolchain's own sources declare the same six over the prelude's `(List Int)`, `Option`, `Pair`
and `Bool` (`v3/kernel/host.shard`), and a seventh, `read_range path off n`: at most `n` bytes from
offset `off`. Under `--run` an extern outside the six, or one declared at types the host's values
do not fit, is `RUN: stuck extern in F` before the host is asked (`v3/kernel/test/wire_test.sh`).
A native binary also performs `read_range`; an extern its runtime lacks refuses the build.

## Proofs

A proof is the last argument of a `theorem` and of a `fulfills`. It has one of four forms; anything
else is refused `bad_proof: a proof is (exact TERM), (by STEP…), auto or sorry`.

| Proof | Meaning |
|---|---|
| `(exact TERM)` | TERM is elaborated at the statement, over the binders, and is the proof. |
| `(by STEP…)` | A proof script, called a *block*: its steps are elaborated in order into one term. |
| `sorry` | No proof; the theorem is pending (see "Pending proofs"). |
| `auto`, `(auto HINT…)` | The block comes from the file's sidecar (see "`auto`, the engine and sidecars"). |

```
(theorem t4 ((a Nat) (b Nat) (h (= a b))) (= b a)
  (by (have h2 (= b a) (exact (Eq.symm h))) (exact h2)))
```
(v3/pins/loader/by_core/main.shard)

A bare term is not a proof: write `(exact TERM)`. `by` is not a term: `(Fin.mk 3 (by (decide)))`
is refused `lean_symbol`; state a named theorem and cite it. Whatever the form, the outcome is one
closed term of L that K checks as the theorem's value. The step elaborator is not trusted: a step
that builds a wrong term is K's refusal of the theorem.

### Goals and blocks

A *goal* is a proposition to prove, the *target*, under a *context*: an ordered list of named
locals, each a variable with its type or a hypothesis with its statement. A theorem's binders are
the first goal's context and its PROP the target. A `fulfills` of a view's requirement starts the
same way, under the requirement's binders. A `fulfills` of a descent obligation `f.dec_N` starts
with an empty context and the whole obligation as the target, so its block opens with `intro`.
A goal is printed as `(n : Nat) (h : (= n 1)) ⊢ (= n 1)`. The printed terms are close to source
syntax and are not source: every implicit argument is written and a projection prints as
`(proj i t)`, as in `(< (List.length Nat t) (List.length Nat (proj 0 p)))`. Pasted back, such a
term is refused (`function_expected`, `bad_shape`); to reuse one, put `@` before each constant
that shows a type argument and write `(proj STRUCT i t)`.

A step is a function of its own text, the goal and the environment: nothing is searched, and no
table that changes between loads is consulted. A step *continues* (the goal becomes one new goal
for the next step), *closes* the goal (it is then the last step of its block), or *branches* (it
opens child goals and carries one block for each, written inside the step).

The steps of a `have`, of a `wf` and of an arm are written inline after the fixed arguments. Each
child block of `apply` and `rw` is one list, `(STEP…)`. A list headed by a step's name is read as
a one-step block, so `(apply Eq.symm (exact h))` needs no second parenthesis. A block of several
steps is wrapped, and a first step that is a bare word takes its list form: `((reduce) rfl)`.

A refused step is the file's read error, and the file's reading ends there:
`READ-ERROR FILE REASON: step N: MESSAGE — the goal: (x : T) … ⊢ TARGET`. N counts from 1 within
the innermost block the step stands in; the record does not say which block. Steps build their
terms from constants of Lean's `Init` (`Eq.mpr`, `congrArg`, `WellFounded.fix`, `Lean.Omega`'s
lemmas), so the module's `(import Init NAME)` must reach them. `wf` and `arith` name what is out
of reach (`wf_unreached`, `arith_unreached`). `rw`, `simp_only`, `unfold` and a `reduce` that
computes build on `Eq.mpr`, declaration 383 of the export: below it K refuses the theorem with
the bare record `REFUSE NAME unknown_constant`, which names neither the step nor the constant, and
the file goes on. `decide` below its decision's instance (`Nat.decEq`) reports `decide_failed: the
decision does not evaluate to true`. `intro`, `exact`, `rfl`, `have`, `show`, `apply`, `cases` and
`induction` need nothing past `Eq`.

### The steps

These sixteen forms are the whole language; `rfl`, `decide`, `reduce`, `arith` and `sorry` may be
written bare or in parentheses. Examples: v3/pins/loader/by_core unless a pin or file is named.

| Step | What it does | Example |
|---|---|---|
| `(intro NAME…)` | The target, reduced to weak head normal form, is a `forall` or an implication; each NAME becomes a local for the next binder. Continues. | `(by (intro h) rfl)` |
| `(exact TERM)` | TERM, elaborated at the target, closes the goal. An argument left undetermined is refused. | `(by (exact h))` |
| `rfl` | The target is an equation whose sides K finds convertible. Closes by `Eq.refl`. | `(by rfl)` |
| `(have NAME PROP STEP…)` | The inner steps prove PROP in the current context; the block continues with `NAME : PROP` added. | `(have h2 (= b a) (exact (Eq.symm h)))` |
| `(show PROP)` | Replaces the target by PROP when K finds the two convertible. Continues. | `(by (show (= (+ n 0) n)) rfl)` |
| `(apply TERM BLOCK…)` | Opens as many leading binders of TERM as its type has more than the target, and unifies its conclusion with the target. Each premise left open is a child goal, in binder order, one BLOCK each. A data argument the target does not fix is assigned by the first block that determines it. | `(by (apply (Eq.trans h1) (exact h2)))` |
| `(cases X (ARM…))`, `(cases X H (ARM…))` | Case analysis on X, a local or any term whose type is an inductive type without indices. One ARM `(CTOR (NAME…) STEP…)` per constructor, named in full or by its last component; the NAMEs are its fields. With H, each arm has `H : X = CTOR fields`. | `(cases t ((TNum (n) decide) (TPlus () decide) (TMinus () decide)))` |
| `(induction X (ARM…))` | As `cases`, by the type's recursor: an arm names the fields, then one induction hypothesis per recursive field. | `(induction xs ((nil () rfl) (cons (x r ih) (reduce lhs) (rw ih) rfl)))` |
| `(wf X MEASURE NAME STEP…)` | Well-founded induction on the local X under MEASURE, a term of type `Nat`. The inner steps prove the target with `NAME : ∀ y, MEASURE[y] < MEASURE[X] → target[y]`. | `(wf n (Int.natAbs n) ih (intro h) …)` (v3/examples/calc/calc_show.shard) |
| `decide` | Closes a target whose decision K evaluates to `true`: `<`, `<=` at `Nat` and `Int`; `=` at `Nat`, `Int`, `Bool` and a type with a derived equality. No conjunction, disjunction or negation. | `(by decide)` at `(< 1 2)` |
| `(unfold NAME…)` | Rewrites each occurrence of the named constants present when the step starts, once: by the first of the function's equations that matches, else by its definition. Continues. | `(by (unfold tok_size) rfl)` |
| `reduce`, `(reduce SIDE)`, `(reduce SIDE? (using RULE…))` | Normalizes the target by computation; with SIDE (`lhs`, `rhs`), one side of an equation. With `using`, also rewrites by the given facts. Continues; succeeds unchanged when nothing reduces. | `(by (reduce) rfl)`; `(reduce lhs (using (if_pos h)))` (reduce_using) |
| `(rw LEMMA BLOCK…)`, `(rw (<- LEMMA) …)`, `(rw LEMMA (occ K EXPECT?) BLOCK…)` | Rewrites the target by an equation, left to right, or right to left under `<-`. Each premise of LEMMA left open is a child goal with its BLOCK. Continues. | `(rw h (occ 2))`; `(rw (<- h))`; `(rw if_pos (exact h))` |
| `(simp_only RULE…)` | Rewrites by the listed equations, each RULE a lemma or `(<- LEMMA)`, until none applies. Continues. | `(simp_only (if_neg h2) (if_pos h3))` (simp_core) |
| `arith`, `(arith FACT…)`, `(arith only FACT…)`, `(arith … (farkas K…))` | Closes a target that follows by linear arithmetic over `Int` and `Nat` from the context and the FACTs. | `(by arith)`; `(arith only h (farkas 1 1))` (arith_only) |
| `sorry` | Closes its goal without a proof; the theorem is pending. | `(cases b ((false () sorry) …))` (by_sorry) |

### Case analysis and induction

- X may be a term; its occurrences in the target are replaced by the constructor form in each arm.
  A proof of a disjunction is split this way, `(cases (Nat.mod_two_eq_zero_or_one a) ((inl (ha) …)
  (inr (ha) …)))` (v3/std/facts.shard), and so is a decision: `(cases (Nat.decEq n 0) ((isFalse (h)
  …) (isTrue (h) …)))` gives one arm the proposition and the other its negation. This is how a
  proof splits on a condition.
- When X is a local, every hypothesis whose type mentions it is restated in each arm at the
  constructor form, and an induction hypothesis quantifies over those dependents. X itself leaves
  the context unless H names the equation.
- There is no `generalizing` clause. A variable that must vary in the induction hypothesis stays
  quantified in the statement and is introduced inside each arm:
  `(cons (x r ih) (intro acc) (reduce) (rw (ih (+ (* 10 acc) x))) rfl)` (calc_show.shard).
- `wf` restates no hypothesis. A hypothesis about X that the induction needs is written as an
  implication in the statement and introduced inside the `wf`. The measure is a `Nat`; an `Int` is
  measured through `Int.natAbs`; there is no lexicographic measure.

### Rewriting: `rw`, `simp_only`, `unfold`, `reduce`

All four change the target only, never a hypothesis, and none closes a goal: an equation that has
become `a = a` still needs `rfl`. A LEMMA or RULE is a term whose type, under its binders, is an
equation: a hypothesis, a theorem's name, or a theorem applied to arguments, `(if_pos h)`. The
side to replace is matched against the target's subterms *syntactically*, in pre-order, the
function before its arguments. A match never unfolds a definition: `(+ a b)` does not match
`(+ b a)`, and a lemma about `(f x)` does not match a term that only computes to it. Lean's
class-method spellings and this language's function spellings are read as one (`HAdd.hAdd` at
`Nat` is `Nat.add`), so Init's lemmas match source terms. A subterm that mentions a variable bound
inside the target is never rewritten.

- `rw` takes the first match and rewrites every occurrence of that instance. `(occ K)` rewrites the
  K-th occurrence alone, counting whole applications from 1. `(occ K EXPECT)` also states the
  instance expected there, and is refused when another is found.
- `simp_only` repeats, at most 256 times: the first rule in list order that has a match, and whose
  premises are literally hypotheses of the context, is applied as `rw` applies it.
- `unfold` uses the equations `f.eq_1`, `f.eq_2`, … that a `fn` receives when it is defined. A
  function defined by cases has no equation that matches at a variable: do `cases` on it first.
- `reduce` rewrites the first reducible subterm, to a fixpoint, within 2,000 rewrites. An equation
  with a constructor or a literal in its left side applies anywhere. An equation `f n = …` (a
  measured function's) applies only at closed arguments, at most 64 times; at a variable use
  `unfold`. An `if` or a `match` steps once its condition or scrutinee is decided or headed by a
  constructor. Other definitions unfold only inside closed subterms. A comparison such as
  `(< a b)` is left as written.
- `(reduce (using RULE…))` also rewrites by each RULE where its left side stands, a rule before a
  computation step. A function that a rule's left side applies is not unfolded while anything else
  can step. A rule is given its premises, `(if_pos h)`; every rule must rewrite at least once; a
  function's name is not a rule, since equations apply unnamed.

### `arith`

`arith` refutes the negated target together with a set of *rows*, each a linear comparison: the
context's hypotheses that are comparisons (the others are passed over), then each FACT (a term, for
example a quantified hypothesis at an instance, `(arith (mono a))`), then what the atoms bring.
Under `only` the context gives no row.

| Part | What is read |
|---|---|
| Relations | `<=`, `<`, `=` at `Int` or `Nat`, the negation of one, and `False` as a target. Init's `≥` and `>` are read reversed. |
| Linear terms | `+`, `-`, unary `-`, `*` with a numeral on one side, numerals. At `Int`, `x / k` and `x % k` for a positive numeral k. |
| `Nat` | A `Nat` comparison is cast into `Int`; every `Nat` atom brings `0 ≤ a`. Sums, successors and products with a numeral are opened under the cast. |
| Atoms | Every other term is opaque: a product of two variables, a function call, truncated `Nat` subtraction, `Nat` division and remainder, `Int.natAbs`. |

The search is Fourier–Motzkin elimination over the rationals, with `a < b` read as `a + 1 ≤ b`;
it neither splits cases nor cuts. So `arith` does not use a hypothesis `a ≠ b`, does not prove
what holds over the integers only by divisibility, and proves nothing nonlinear: `(<= x (* x x))`
is out of reach (v3/examples/auto/auto.shard). It does not split a conjunction or instantiate a
quantified hypothesis; give the instance as a FACT. `(farkas K…)` supplies the certificate in
place of the search: one integer per row, in the order negated target, context rows, FACTs, atoms'
rows. It is checked, never repaired: the weighted rows must cancel every atom and leave a false
constant, and an inequality takes no negative weight. An equation that needs both directions takes
two `(farkas …)` forms.

### Refusals

| Reason | Step | Message |
|---|---|---|
| `unknown_step` | block | `NAME: not a form of I (intro exact rfl have show apply cases induction wf decide unfold reduce rw simp_only arith sorry); a new tactic is an E library producing these forms, named by (auto (tactic M.FN))` |
| `bad_step` | block | the form's shape: `(exact TERM)`; `(have NAME PROP STEP…): the block that proves PROP is empty`; `an arm is (CTOR (NAME…) STEP…), with the names in a list`; … |
| `goal_open` | block | `the block ends with the goal open: GOAL`; `a child goal has no block: GOAL` |
| `goal_closed` | block | `step N closed the goal and steps follow it` |
| `intro_no_pi` | intro | `the target is not a forall or an implication after whnf` |
| `rfl_not_eq` | rfl | `rfl closes an equation; the target is T` |
| `rfl_failed` | rfl | `the sides are not convertible: A and B`; after `convertible` there may stand ` — a case analysis waits at C` |
| `unify_stuck` | rfl, apply | `blocked on NAME: an argument the elaborator cannot solve first-order — write the application with @` |
| `show_mismatch` | show | `not convertible to the target: PROP` |
| `apply_mismatch` | apply | `the conclusion C does not unify with the target T` |
| `apply_goals`, `rw_goals` | apply, rw | `apply opens N goal(s) and the step gives M block(s): [GOAL] …` |
| `witness_open` | apply, rw | `the argument NAME is not determined by the goal and no block assigned it: give it through a premise's exact` |
| `instance_needed` | apply, rw | `the instance argument NAME is not filled by the instance table (there is no instance search): write the application with @ and the instance` |
| `cases_type` | cases, induction | `NAME is not an inductive type`; `not an inductive type: T`; `K could not type NAME.casesOn at this motive` |
| `cases_indexed` | cases, induction | `NAME is an indexed family: cases and induction take a type without indices (there is no injection or subst step) — use Init's lemmas (T.CTOR.inj, Eq.subst) in a term` |
| `cases_rows`, `induction_rows` | | `no arm for the constructor C`; `C is not a constructor of the type, or is named twice` |
| `cases_fields`, `induction_fields` | | `C's arm names N binder(s) and its case has M: name the constructor's K field(s) — (x : T) …`; under `induction`, `…, then J induction hypothesis name(s), one for each recursive field — (x : T) … (_ : P)`. The binders listed are the case's, each under the arm's name for it, `_` where the arm gave none |
| `induction_eq` | induction | `induction takes no equation name: the hypotheses would be guarded by it (Lean has none either); name the equation under cases, or state the equation and induct on the local` |
| `wf_local` | wf | `wf takes a local of the context (a variable, not a term)`; `the local's type is not a type` |
| `wf_unreached` | wf | `WellFounded.fix, InvImage.wf or Nat.lt_wfRel is past the Init prefix in scope` |
| `no_decision` | decide | `decide: the target has no decision`; at an equation it adds `, or its derived equality — a (derive TYPE eq) before this form` |
| `decide_failed` | decide | `the decision does not evaluate to true: T` |
| `unfold_stuck` | unfold | `no occurrence of the named constant unfolds — no equation of it matches (cases on its argument first), or its one delta step is a stuck recursor: T` |
| `reduce_budget` | reduce | `the normalization ran out of budget (2000 rewrites)`; `64 unfoldings of a function at closed arguments did not end the computation: T` |
| `reduce_rule_open` | reduce | `rule N has an argument its left side does not fix, of type T: a rule is given its premises, as (if_pos h) — reduce looks for none` |
| `reduce_rule_unused` | reduce | `rule N rewrote nothing: no subterm of the run matched its left side, L, in K rewrites` |
| `rw_not_eq` | rw, simp_only, reduce | `the lemma is not an equation: T`; for a function's name it ends in a pointer (`leave it out`, or `its equations are reduce's`) |
| `rw_no_match` | rw | `no subterm of the target matches L in T`; `the target has N occurrence(s) of L` |
| `rw_occ_changed` | rw | `(occ K EXPECT): the occurrence found is A, not the expected B — the source moved under the guard, or the expectation is wrong` |
| `rw_motive` | rw, reduce | `the motive is not type correct (the occurrence is under a dependency): T` |
| `simp_no_progress` | simp_only | `no rule rewrites the target (a rule with a premise applies where a hypothesis states it): T` |
| `simp_budget` | simp_only | `256 rewrites did not reach a fixpoint (a rule rewrites its own result): T` |
| `simp_self` | simp_only | `rule N rewrites A to a term that holds it, B: no fixpoint — give the rule its arguments, or use rw for one step` |
| `arith_goal` | arith | `the target is not a linear comparison over Int or Nat (≤, <, =, a negation of one, or False)` |
| `arith_fact` | arith | `a fact's statement is not a linear comparison over Int or Nat (≤, <, = and the negations of ≤ and <; a ≠ is no row)` |
| `arith_failed` | arith | `no weighting of the rows refutes them (linear arithmetic over the rationals with integer-tight strictness: no cut, no case split, a ≠ hypothesis is no row) — a fact is missing, or the goal is not linear in these atoms — rows: [0 <= -1 + -1*x + 1*y] …` |
| `arith_budget` | arith | `the elimination passed 512 derived rows: give the certificate — (arith … (farkas K…)), a weight per row in the order shown` |
| `arith_certificate` | arith | `the weights do not refute the rows: the coefficients must cancel and the constant be negative, no inequality's weight negative (a zero leaves its row out)`; `the certificate has N weights and the goal has M rows` |
| `arith_unreached` | arith | `NAME is past the Init prefix in scope: arith builds its proof from Lean.Omega's lemmas — import Init through Lean.Omega.Int.ofNat_lt_of_lt …, or through Int.lt_mul_ediv_self_add … where a row has a product or a quotient` |

`exact`, and every TERM, LEMMA and FACT inside a step, can also meet the term elaborator's own
refusals (`type_mismatch`, `unknown_constant`, `unsolved_implicit`, …). In `simp_only` and
`reduce` these are prefixed `rule N: `.

### What a Lean user will not find

| Lean (each is `unknown_step` here) | Instead |
|---|---|
| `simp` with a default lemma set | `(simp_only RULE…)`, every rule written; `reduce` for computation. |
| `omega`, `linarith`, `norm_num` | `arith`, within the fragment above. |
| `rw … at h`, `simp at h`, any step on a hypothesis | None. State the new fact with `have`, or build it as a term. |
| `injection`, `subst`, `cases` on an indexed family or an equation | None; such a `cases` is refused `cases_indexed`. Use Init's lemmas (`T.CTOR.inj`, `Eq.subst`) in a term. |
| `constructor`, `exists`, `left`, `right`, `exfalso`, `contradiction`, `assumption` | `(exact TERM)` or `(apply TERM …)` with `And.intro`, `Exists.intro`, `Or.inl`, `absurd`, the hypothesis's name. |
| `by_cases`, `split`; `induction … generalizing`, `revert`, `generalize` | `(cases (Nat.decEq n 0) (…))` on the decision; dependents are restated automatically, otherwise quantify in the statement. |
| `change`; `calc`, `refine`, `congr`, `funext` | `(show PROP)`; a term under `exact`. |
| `<;>`, `first`, `try`, `repeat`, `all_goals` | None. A block is linear and each branch has its own block. |
| A user-defined tactic or macro | A library function that produces a block, run by `prove` (below). |

## Pending proofs

A proof that is absent is *pending*. A pending theorem declares nothing: K holds no constant under
its name. The load records it and goes on with the file (v3/pins/loader/by_sorry/main.shard):

```
PENDING main.t sorry goals=2 [(n : Nat) ⊢ (= n n)] [(n : Nat) ⊢ (= 1 1)]
```

| Record | Cause |
|---|---|
| `PENDING NAME sorry` | The proof is the word `sorry`. |
| `PENDING NAME sorry goals=K [GOAL]…` | A block holds K `sorry` steps. The whole block is still elaborated, so every other step must be right. The goals are cut at 2,000 characters. |
| `PENDING NAME auto_missing fp=S` | The proof is `auto` and the sidecar has no entry for NAME. |
| `PENDING NAME auto_stale CAUSE,… fp=S` | The sidecar's entry was made for another requirement. |
| `PENDING NAME auto_unsolved fp=S REASON` | Under `prove` only: no producer found a block. |
| `PENDING NAME measure` | A function defined under a measure, with a descent obligation `NAME.dec_N` not yet proven. |

Pending is not failure. The `LOAD:` line counts it (`pending N`); the load's exit code, the number
of refusals and errors, does not. What rests on a pending proof is reported, never assumed:

| Dependent | What happens |
|---|---|
| A term that cites a pending theorem | `READ-ERROR FILE pending_obligation: NAME is pending: it admits nothing, and citing it is the obligation`, and the file's reading ends (v3/pins/loader/sorry). The text is the same whatever the cause, which the name's `PENDING` record states. In a `fn` the same citation is `REFUSE F pending_obligation: …`, and the file goes on. |
| A view's requirement whose `fulfills` is pending | `PENDING NAME sorry`, then `DISCHARGE NAME pending`. The requirement stays a parameter; every theorem that rests on it lists it in `params=` (v3/pins/loader/view_pending). |
| A measured function | Each descent is a parameter, `PARAM f.dec_N obligation`. The function is defined, with its equations; its record and those of its callers carry `pending=f`; a theorem that cites it is accepted with `params=f.dec_N`. `(fulfills f.dec_N PROOF)` proves the descent as the theorem `f.dec_N.proof`: `DISCHARGE f.dec_N proved`, and the load's records are rewritten so that `PENDING f measure`, `pending=f` and the `params=` entry are gone (v3/pins/loader/discharge_measure). |
| A `bin` that `requires` a pending theorem | `UNMET NAME`, an error: the load fails (v3/pins/loader/bin_unmet). This is the one place where pending fails a load. |
| The store | A pending theorem is absent. An undischarged obligation or a view parameter is a `param` line of the manifest, and the verifier prints it. |

A discharge that rests on the obligation it discharges, directly or through another, is refused
`fulfills_cycle` and closes nothing (v3/pins/loader/discharge_two_cycle).

## `auto`, the engine and sidecars

`auto` delegates a proof to a tool. The source says only `auto` or `(auto HINT…)`; the block lives
beside the source in a *sidecar* file that tools write and people do not edit. No tool writes into
a source file.

```
(theorem le_chain ((x Int) (y Int) (z Int) (h1 (<= x y)) (h2 (<= y z))) (<= x z) auto)
```
(v3/examples/auto/auto.shard)
```
(sidecar 3 2)
(proof-for le_chain (req 1140731689746303130 0 0 1510067385901704) (by (arith only h1 h2 (farkas 1 1 1))))
```
(v3/examples/auto/auto.auto.shard)

### At load time

A load never searches. When a file has a `theorem` or a `fulfills` whose proof is `auto`, the
loader reads `FILE.auto.shard` beside `FILE.shard`; no such file means no entries. For each `auto`
proof it computes the key of the statement and looks up the entry under the name the source
writes: a theorem's own, and for `(fulfills dn.dec_1 auto)` the obligation's, `dn.dec_1`. Where
the key agrees in every part, the entry's block is elaborated exactly as a block written in the
source would be: the theorem is accepted, or the step's refusal is the file's read error. Where
there is no entry, the record is `PENDING NAME auto_missing fp=S`. Where a part of the key
differs, it is `PENDING NAME auto_stale CAUSE,… fp=S`, and the entry is not tried, even if its
block would still succeed.

The hints are not read at load time: changing them, or renaming a binder, changes nothing
(v3/pins/loader/auto_strategy_free).

An obligation delegated this way is discharged as a hand-written `fulfills` is: `prove` prints
`PROVED F.dec_N.proof …` and writes `(proof-for F.dec_N …)`; a load replays the block and prints
`DISCHARGE F.dec_N proved`; without an entry the record is `PENDING F.dec_N.proof auto_missing`
(`v3/pins/loader/auto_fulfills`; `engine_test.sh`). Inside the implementation of a view `auto` is
read the same way, in a `theorem`, in a requirement's `fulfills` and in an obligation's: the
sidecar stands beside the implementation file, `DIR/DIR.auto.shard`, and `prove` writes it when it
is given the directory, alone or after a consumer (`v3/pins/loader/auto_impl`).

An entry is keyed by the *resolved requirement*: four numbers, `(req S D P E)`, each a 61-bit
fingerprint. They detect a changed task and prove nothing: the block is always replayed, and K
checks the result.

| Part | Cause printed | What it covers | Pin |
|---|---|---|---|
| S | `statement` | The elaborated statement closed over its binders. Binder names are outside it. | auto_stale |
| D | `deps` | Every declaration of the load that the statement cites, directly or through other declarations, each by its content fingerprint. Init's declarations are not walked. 0 when there is none. | auto_stale_deps |
| P | `policy` | The axioms the file `trusts`, and every parameter the file can see that the load has admitted so far: view parameters and descent obligations, discharged or not, of the file and of the modules in its closure. A parameter of another root is no part of it. | auto_stale_policy, auto_other_root |
| E | `env` | The module's Init horizon and the identity of the Init export (its record count and byte length). 0 for a module without Init. | auto_stale_env |

### The sidecar file

The first form is `(sidecar SCHEMA RECON)`. SCHEMA is the version of the entry syntax (3). RECON
is the *reconstruction version*, the version of the elaboration from a block to a term (2), under
which the entries were last replayed. Every other form is
`(proof-for NAME (req S D P E) (by STEP…))`, NAME as the source writes it.

| Condition | Report |
|---|---|
| An older schema (no first form is schema 1; schemas 1 and 2 key an entry by S alone) | `SIDECAR FILE schema=N current=3`; the entries are replayed. |
| A newer schema | `LOAD-ERROR FILE sidecar_schema: PATH: schema 4 is past this loader's 3 — regenerate the sidecar with prove.shard` |
| Another reconstruction version | `RECON FILE version=N current=2`; the entries are replayed. |
| A form of another shape | `LOAD-ERROR FILE sidecar_malformed: PATH: an entry is (proof-for NAME (req S D P E) (by STEP…))` |

### `prove`

`prove` fills sidecars. It is `v3/kernel/prove.shard`, built as `v3/bin/prove`:

```
prove --root DIR [--init EXPORT --init-index INDEX] [--fuel N] FILE…
prove --root DIR [--init EXPORT --init-index INDEX] --measure FILE…
```

It loads each FILE as the loader does, with one difference: an `auto` proof — a theorem's, or an
obligation's `fulfills` — without a fitting entry, missing or stale, is given to a *producer*, and
the block the producer returns is replayed before anything is recorded. Then every file that
gained a block is rewritten whole: the entries it held for other theorems, then the new ones. A
sidecar read at an older schema or another reconstruction version is rewritten at the current
ones, without a search. `prove` prints the load's records, with `PROVED NAME req=S.D.P.E (by …)`
for each new block, and ends with
`PROVE: modules … errors 0  proved 11  unsolved 1  sidecars written 1`. The exit code is refusals
plus errors plus unsolved theorems. An entry is replaced only when its theorem is proven anew;
entries of theorems that no longer exist stay in the file.

`--measure` writes nothing. Every hand-written proof is read as usual and the producers are also
run on its statement: `ENGINE NAME solved|unsolved`, `TACTIC NAME M.FN solved|unsolved`, and the
total `ENGINE: solved S of N`.

### The engine

The default producer is the *engine* (`v3/kernel/engine.shard`). It tries the following, cheapest
first, and confirms every step by running it:

1. `intro` over every leading binder (named after the binder, else `h1`, `h2`, …), then the rest;
2. `rfl`; then `(reduce)` and `rfl`; then `decide`;
3. `(arith only FACT… (farkas K…))` over the context's comparisons and the hinted lemmas, the
   certificate written out;
4. `(simp_only RULE…)` over the equations of the functions in the target, the context's equations
   (an induction hypothesis among them) and the hinted lemmas, then `rfl` or step 3;
5. `(induction X (…))` on the hinted local, then on each local of an inductive type that the
   target mentions, every arm by this same list; inductions nest two deep.

It emits no `cases`, `have`, `rw`, `unfold`, `apply`, `exact` or `wf`. The hints are `(induct X)`,
`(lemmas NAME…)` and `(budget N)`, the number of steps it may try, 400 by default; any other hint
is `auto_unsolved`. Over the 100 theorems of v3/examples/calc the engine closes 54 unaided
(v3/kernel/test/engine_calc.txt).

### A tactic library

`(auto (tactic M.FN) HINT…)` names another producer, a function of a loaded module, for example
`(fn solve ((goal (List Int)) (hints (List Int))) (List Int) …)` in
v3/examples/tactic/bool_split.shard. It receives the goal as text, `(goal ((NAME TYPE)…) TARGET
(applicable STEP…))`, and the hints after `(tactic M.FN)` as text, and answers one block `(STEP…)`
as text, or nothing. The three types are byte lists (`(List Int)`, `(List Nat)`) or `ByteArray`.
The function sees no elaborator state and performs no effect. Only `prove` runs it, in the
evaluator, under `--fuel` (10^12 by default); a load replays the sidecar and never runs the
library. A failure is `PENDING NAME auto_unsolved fp=S tactic=M.FN: REASON`: an unknown name, a
wrong signature, no answer, an answer that is not one block, a refused step, an evaluator stuck or
out of fuel. One delegation names one tactic.

## The store and identities

A plain load persists nothing. Every load reads, elaborates and checks again, and prints one
record per declaration; for an accepted one,
`ACCEPT NAME module=M hash=H axioms=A,… init=N params=P,… req=S.D.P.E`. `hash=` is the
declaration's *content fingerprint*: a 61-bit structural hash of its type, for a definition of its
value too, for an inductive type of its constructors' types. A theorem's is its statement's alone.
Binder names are outside it. It names files and detects change; nothing is accepted on its
strength. `axioms=` is the axiom closure of the proof as written, so two proofs of one statement
may differ in it. `init=` is the module's Init horizon, `params=` the parameters the declaration
rests on, and `req=`, on theorems only, the key of the previous section.

### Writing a store

`load --root DIR [--init EXPORT --init-index INDEX] --store OUT FILE…`: after a load without
refusals or errors, every constant the load gave to K (each `ACCEPT`, each `PARAM` K holds) is
written under OUT. Pending theorems do not block the store and are not in it.

| File | Content |
|---|---|
| `OUT/ID.ndjson` | One declaration as the records K reads: lean4export's NDJSON, format 3.1.0. First the name, level and expression tables it cites, numbered from 1; then the declaration record. An inductive type is written with its constructors and recursors. Every binder is anonymous. ID is the content fingerprint in decimal, or `FP-K` when another text already has that name. |
| `OUT/ID.names` | Display only: a line `IE NAME` for each binder record that had a name. |
| `OUT/order` | `ID NAME` per declaration, in admission order. |
| `OUT/manifest` | What the store claims to hold: the verifier's target. |

The manifest's lines are `manifest 1`; `recon N`, the reconstruction version the proofs were
elaborated under; `decl ID KIND NAME`, KIND one of `axiom`, `def`, `thm`, `opaque`, `inductive`;
`param NAME` for each parameter held as an axiom (a view parameter, an undischarged obligation);
`discharge OBL OBL.proof` for each obligation the load proved; and `bin NAME entry F externs E…
requires R…` for each `bin`.

`order` and `manifest` are appended, never rewritten, so several loads fill one store, and a file
that already holds the same text is shared: the last line is `STORE: OUT new=N shared=M`. Because
the records carry no binder names, two proofs that differ only in names write one text under one
ID. A store holds one reconstruction version; a load at another is refused: `STORE: refused
OUT/manifest: recon 0 is not this loader's 2 — a store holds one reconstruction version; the
migration is a new store`. The declarations of a view's implementation are checked and not stored.
The test suite writes the store of v3/examples/calc to `v3/release/calc` (derived, untracked).

| Version | Value | Where it stands | A mismatch |
|---|---|---|---|
| Sidecar schema | 3 | `(sidecar 3 …)` | older: the `SIDECAR` report; newer: `sidecar_schema` |
| Reconstruction version | 2 | `(sidecar … 2)`; the manifest's `recon 2` | the `RECON` report; `STORE: refused`; named by `--reconstruct` |
| Engine version | 1 | the sidecar's first comment line; the `PROVE` header | none: it is not read back |
| Manifest | 1 | `manifest 1` | the verifier exits 2 |

### Reconstruction

`load --root DIR [--init …] --reconstruct OUT FILE…`: after a clean load, each theorem listed in
OUT's manifest that this load elaborated is rendered as the store would write it and compared,
byte for byte, with `OUT/ID.ndjson`. Nothing is written.

| Line | Meaning |
|---|---|
| `RECONSTRUCT NAME same recon=2` | The same proof term. |
| `EVIDENCE-CHANGE NAME recon=2` | A different term; `missing` follows NAME when the file is gone. |
| `RECONSTRUCT NAME not-loaded` | The load did not elaborate this theorem. |
| `RECONSTRUCT: theorems N  same S  changed C  not-loaded A recon=2` | The total. |

`recon=A→B` on every line means the manifest's version is A and this loader's is B. The exit code
is the number of changes. A change is no error of the proof, since K accepted both terms: the
stored evidence is no longer what the sources produce, and the remedy is a new store.

### The verifier

`verify_release [-v] OUT EXPORT INDEX` (`v3/kernel/verify_release.shard`, built as
`v3/bin/verify_release`) contains K, the file primitives, the JSON reader and the Init index
reader, and no elaborator. It checks the part of Init that the store cites, then gives each stored
file to K in `order`'s order, each file with fresh tables over the environment so far, and holds
the result to the manifest:

| Line | Meaning |
|---|---|
| `REJECT …`, `EXHAUSTED …`, `MISMATCH …`, `UNSUPPORTED …`, `MALFORMED …` | K's own verdict on a record. `ACCEPT NAME` is printed under `-v`. |
| `MISTIED WANT accepted as NAME fp=FP` | Accepted under another name or fingerprint than the manifest's. |
| `MISKIND NAME: the manifest says thm, the record is axiom` | Accepted as another kind. |
| `CARDINALITY ID: N declarations accepted, one expected` | A file with more or fewer than one. |
| `UNLISTED ID NAME`, `MISSING ID NAME` | An order line the manifest lacks; a manifest declaration without its file or its order line. |
| `TAKEN NAME is a declaration of the export` | A stored name that is Init's. |
| `UNDECLARED NAME: …` | An axiom that is neither a `param` nor a discharged obligation. |
| `PARAM NAME` | A declared parameter: what the store's theorems are conditional on. |
| `DISCHARGE OBL by PROOF`, `BAD-DISCHARGE OBL: …` | The store holds, or does not hold, the theorem `OBL.proof` at the obligation's statement. |
| `CONDITIONAL NAME: rests on A …` | The axiom closure reaches an axiom that is not Init's, not a `param` and not a verified discharge. |
| `BIN NAME ok`, `BAD-BIN NAME: REASON` | A `bin` line's entry, externs and requirements are, or are not, accepted declarations of the right kind, the requirements unconditional. |

The last line gives every count:

```
RELEASE: declarations 23  accepted 23  rejected 0  exhausted 0  mismatched 0  unsupported 0  malformed 0  mistied 0  missing 0  taken 0  unlisted 0  miskind 0  cardinality 0  undeclared 0  conditional 0
```

The exit code is 0 exactly when every count after `accepted` is zero and Init's part checked
clean, 1 otherwise, and 2 when the store or the arguments cannot be read. The verifier does not
know which theorems a source states: a theorem that was pending when the store was written is not
in the manifest, and the store verifies without it. A `bin`'s `requires` makes a theorem mandatory.

Each tool runs under the bootstrap evaluator (`eval direct v3/kernel/load.shard ARGS…`) and as a
native binary under `v3/bin/`. The tests: v3/kernel/test/tactic_test.sh (blocks), engine_test.sh
and tactic_lib_test.sh (`prove`, sidecars), store_test.sh (store, reconstruction, verifier).

## Acceptance: axioms and policy

### When a declaration is accepted

A *declaration* is an axiom, a definition, a theorem, an opaque constant or an inductive block.
The loader takes each through five steps and prints `ACCEPT`, or the record of the first step
that fails.

| step | what is done | record when it fails |
|---|---|---|
| 1 | the reader elaborates the form into a raw declaration | `READ-ERROR FILE REASON: DETAIL` |
| 2 | K checks it against the environment so far (KERNEL.md, "K") | `REFUSE NAME REASON` |
| 3 | no constant it adds has a name Init declares under the file's module name | `REFUSE NAME name_taken` |
| 4 | K computes its axiom closure | `LOAD-ERROR FILE closure_bound: NAME` |
| 5 | the file's policy permits every axiom of the closure | `POLICY NAME outside the policy: A,…` |

The *axiom closure* of a declaration is the set of axioms reached from it through the type and
value of every constant it cites, transitively. A declaration that fails is not in the environment
afterwards (a failure at step 5 drops it again), so a later citation of it fails too. A budget K
spent at step 2 prints `REFUSE NAME exhausted_RESOURCE_at_SITE`: a refusal of this run, not a
judgment on the declaration. An accepted declaration prints

    ACCEPT NAME module=M hash=H axioms=A,… init=N params=P,… req=S.D.P.E

| field | meaning |
|---|---|
| `hash` | a 61-bit fingerprint of its type and value, for comparing reports; nothing is decided by it |
| `axioms` | its axiom closure, each discharged obligation replaced by what its proof rests on |
| `init` | the module's Init horizon at the check (KERNEL.md, "How Init gets in"; `-`: no Init imported) |
| `params` | the members of `axioms` that are parameters of the load (classes 2 and 3 below) |
| `module`, `req` | the declaring module; on a theorem, the key of its statement (used to match stored proofs) |

### The policy of a file

An axiom `a` of a closure is permitted when one of these holds:

1. `a` is in the standard profile and is the constant the load imported from Init under that name;
2. `a` is a parameter of the load (a view parameter or an obligation, below);
3. a `(trusts …)` line of this file names `a`.

The **standard profile** is three axioms. Init declares seven:

| axiom | in the profile | note |
|---|---|---|
| `propext`, `Quot.sound`, `Classical.choice` | yes | |
| `sorryAx` | no | V3's own `sorry` never uses it (class 4 below) |
| `Lean.trustCompiler`, `Lean.ofReduceBool`, `Lean.ofReduceNat` | no | K refuses the reductions they justify: `native_reduction` |

A name alone is no warrant. A module `Classical` that declares its own `(axiom choice …)` is
refused at step 3 (`REFUSE Classical.choice name_taken`, v3/pins/loader/init_name_taken_l).

### `(trusts NAME…)`

`trusts` is a directive, by convention at the head of a file beside `import` and `use`; it acts
on the forms after it. Each name resolves as a
citation would. It widens the policy of that file only: a file that imports a trusting file and
rests on the same axiom must trust it too, or its theorem is
`POLICY main.t2 outside the policy: a.ax` (v3/pins/loader/policy_cross).

    (trusts ax)
    (axiom ax () (Eq two two))
    (theorem t () (Eq two two) (exact ax))          (v3/pins/loader/policy_trusts/main.shard)

An axiom declared in a file that trusts it is recorded `PARAM NAME axiom` instead of `ACCEPT`.
Without the `trusts` line the axiom itself is outside the policy:
`POLICY main.ax outside the policy: main.ax` (v3/pins/loader/policy_axiom). A `bin` form has a
`(trusts A…)` clause of its own; a requirement that rests on an axiom outside it is
`BAD-BIN NAME untrusted: D rests on A, which the bin does not trust`.

### The four classes of assumption

| class | what K holds | where it comes from | how it is reported | how it ends |
|---|---|---|---|---|
| 1 axiom | an axiom | `(axiom …)`; Init | `axioms=` of every dependent; `PARAM NAME axiom` or `POLICY` at its declaration | never: it stays an assumption |
| 2 view parameter | an axiom-kind constant | `sig type`, `sig fn`, `requirement` in a view | `PARAM NAME type arity=N`, `PARAM NAME fn`, `PARAM NAME requirement`; `params=` of every dependent | the implementation is checked: `DISCHARGE NAME type`, `fn`, `defined` or `proved` |
| 3 obligation | an axiom-kind constant `f.dec_N` | each recursive call of a `fn` with `(measure …)` | `PARAM f.dec_N obligation`; `PENDING f measure`; `params=` and `pending=` of dependents | `(fulfills f.dec_N PROOF)`: `DISCHARGE f.dec_N proved` |
| 4 pending proof | nothing | `sorry` in place of a proof | `PENDING NAME sorry` (in a proof block: `sorry goals=N [GOAL]…`) | a proof is written |

Classes 2 and 3 are assumptions while open: a result whose `params=` is not empty is conditional
on them. When an obligation is discharged the account is recomputed: every record that named it
names what its proof rests on instead, and a proof that rests on its own obligation is refused
(`fulfills_cycle`). Class 4 admits nothing: after `(theorem t3 () (Eq two Nat.zero) sorry)`
(v3/pins/loader/sorry/main.shard) `t3` is not in the environment. A citation of it is
`READ-ERROR FILE pending_obligation: main.t3 is pending: …`, a `fulfills` by `sorry` is
`DISCHARGE NAME pending`, and a `bin` requirement left pending fails the load with `UNMET`.

The release gate (`verify_release.shard`) reports the same classes over a stored bundle with K
alone: `PARAM NAME`, `DISCHARGE OBL by PROOF`, `UNDECLARED NAME` (an axiom the manifest does not
declare) and `CONDITIONAL NAME: rests on A…` (a closure that reaches such an axiom).

## What the loader prints

### The command line

The driver is `v3/kernel/load.shard`: `eval direct v3/kernel/load.shard ARG…` on the bootstrap
evaluator, `v3/bin/load ARG…` as a native binary.

| Argument | Meaning |
|---|---|
| `--root DIR` | the package root; required (`load: --root DIR is required`, exit 2) |
| `--init EXPORT` | the pinned export of Lean's `Init`, one file |
| `--init-index INDEX` | the export's index (built by `v3/kernel/initindex.shard`) |
| `--instance-budget N` | how many instances of one template the load may build (default 1,024) |
| `--state` | also judge each function that takes or returns the checker's state (`STATE` records) |
| `--store DIR` | after a clean load, write every admitted constant under DIR: `STORE: DIR new=N shared=M` |
| `--reconstruct DIR` | after a clean load, compare each theorem's proof with the one stored under DIR, byte for byte; nothing is written |
| `--dump` | after a clean load, print the loaded program as canonical text, and nothing else |
| `--emit DIR` | after a clean load, write the program's C text under DIR: `EMIT: DIR entry=FILE modules=N declarations=M` |
| `--run MODULE.FN` | after a clean load, run the program from that function; only its own output is printed |
| `--fuel N` | with `--run`: the run's step budget (default 1,000,000,000,000) |
| `--trace FILE` | with `--run`: write the run's effect trace to FILE |
| `--lowered` | with `--run`: run the program's lowered form in its place |
| `--` | ends the options; what follows is the program's arguments under `--run` |
| anything else | a file or a directory to load, in order; it lies under the root and is spelled from the same base (else `outside_root`) |

`--dump` wins over `--emit`, which wins over `--run`. With none of the three the output is a header
line, one record per event in the order it happened, and a line of counts:

```
LOAD root=DIR
…records…
LOAD: modules 10  accepted 404  runnable 6  refused 0  pending 0  realized 12  defined 61  errors 0
```

The exit code is `refused` plus `errors`; a pending declaration does not fail a load. With
`--reconstruct` a clean load exits with the number of changed proofs. Under `--dump`, `--emit` and
`--run` a failed load prints its records and the counts and exits 2. A run exits 3 out of fuel, 4
stuck, 5 on a link failure, 6 on a refused argument (`RUN: argument 1 not_a_nat: x`), else as its
program.

### The records

A `REFUSE`, `POLICY` or `PENDING` record concerns one declaration: the environment stays as it was
and the file goes on. A `READ-ERROR` or `LOAD-ERROR` ends its file, and every file importing it ends
with `import_failed`. "Counts as" names the column of the `LOAD:` line. Every `hash=` and every part
of `req=` is a fingerprint: it tells records apart and detects a change, and nothing is accepted or
trusted because two fingerprints agree.

| Record | Fields | Meaning | Counts as |
|---|---|---|---|
| `MODULE` | `M file=F hash=H sees=M1,…` | a module is done: its file, a fingerprint over its accepted declarations, the modules it sees | modules |
| `INIT` | `NAME: horizon H, K declarations admitted` | a file's Init demand: the target of the `(import Init …)` line that set its horizon (`-` when the horizon is inherited through a file import); the horizon; how many Init declarations were admitted for it | — |
| `ACCEPT` | `N module=M hash=H axioms=A,… init=H params=P,… [req=S.D.P.E]` | the checker admitted N within the file's policy: its file's module, its content fingerprint, the axioms it rests on, the module's horizon (`-`: none), the parameters among those axioms; for a theorem, the key of its statement | accepted |
| `PARAM` | `N KIND [arity=K]` | N admitted as a named assumption: a view's `type`, `fn` or `requirement`; an `obligation` — a measured function's descent `F.dec_N`; an `axiom` the file trusts | accepted |
| `DEFINE` | `N equations=E,… [pending=F,…]` | a function defined in the checker with its equations; `pending=` names the functions whose open obligations it rests on | defined |
| `RUNNABLE` | `KIND N [why=W] [route=R]` | a `fn`, `extern` or `type` that is a program with no definition; `why=` the obstacle, `route=` `typed` or `e_first` | runnable |
| `REALIZE` | `N KIND equations=E,… [pending=F,…]` | a body attached to an existing constant N; KIND is `supplied` or `view` | realized |
| `TEMPLATE` | `N static=P,… equations=E,…` | a function with function-typed parameters, defined; it has no program until instantiated | defined |
| `INSTANCE` | `N of=T shapes=S,… equations=E,…` | the template T built at particular function arguments | defined |
| `DERIVE` | `N type=T cap=C policy=P fields=F,…` | the derived procedure N registered for type T and capability C | — |
| `PENDING` | `N REASON …` | read, nothing admitted: `sorry` (with its open goals), `measure` (descent obligations open), `auto_missing`, `auto_stale`, `auto_unsolved` | pending |
| `REFUSE` | `N REASON[: MESSAGE]` | the checker refused N (a bare reason), or the program reader or the definer did (with a message) | refused |
| `POLICY` | `N outside the policy: A,…` | N rests on axioms its file does not trust; N is dropped | refused |
| `DISCHARGE` | `N KIND` | a view parameter met by the implementation (`type`, `fn`, `defined`, `proved`, `pending`), or an obligation proved by a `fulfills` (`proved`) | — |
| `IMPL-ERROR` | `N REASON: MESSAGE` | the implementation check failed at the parameter N | errors |
| `IMPL` | another record | a record from the implementation's fork | as that record |
| `READ-ERROR` | `FILE REASON: MESSAGE` | a form could not be read | errors |
| `LOAD-ERROR` | `FILE REASON: MESSAGE` | the file could not be loaded: imports, module names, directives, the sidecar | errors |
| `UNUSED` | `M use=P,…` | `use` lines of M that no name in it needs | — |
| `THREAD` | `N VERDICT[: DETAIL]` | whether the function N uses each World token once: `ok`, `world_twice`, `world_unsupported`, `world_forged` | — |
| `STATE` | `N VERDICT[: DETAIL]` | under `--state`, N's use of the checker's state: `ok`, `primitive`, `reused`, `unsupported` | — |
| `BIN` | `N entry F externs K trusts K requires MET/TOTAL` | a `bin` form checked | — |
| `UNMET`, `UNTHREADED`, `BAD-BIN` | `REQUIREMENT`; `FN VERDICT: DETAIL`; `N REASON: MESSAGE` | a bin's failure: a requirement still pending; a function it reaches is not threaded; any other (`bad_entry`, `extern_undeclared`, `extern_outside`, `unknown_requirement`, `unknown_trust`, `untrusted`) | errors |
| `SIDECAR` | `FILE schema=N current=M` | the file's proof sidecar is of an older schema; it is replayed all the same | — |
| `RECON` | `FILE version=V current=W` | the sidecar was made under another reconstruction version; replayed all the same | — |
| `PROVED`, `ENGINE`, `TACTIC` | `N req=… (by …)`; `N solved`; `N FN solved` | never under `load.shard`; printed by `v3/kernel/prove.shard`, which searches for proofs | — |

One example of each, as printed:

```
MODULE main file=main.shard hash=119538086569699698 sees=a,main
INIT Int.add_comm: horizon 45786, 2 declarations admitted
ACCEPT a.two_rfl module=a hash=1718361179517844046 axioms= init=- params= req=1718361179517844046.2053102220271226362.0.0
PARAM lib.stack_refl requirement
DEFINE main.four equations=main.four.eq_1
RUNNABLE fn main.main why=no_l_identity route=e_first
REALIZE main.f supplied equations=main.f.realize_1 pending=main.f
TEMPLATE List.foldl static=f equations=std.list.List.foldl.realize_1,std.list.List.foldl.realize_2
INSTANCE main.main_mapi.of.main_digit_0 of=main.mapi shapes=main.digit/0 equations=
DERIVE main.Tree.size type=main.Tree cap=size policy=structural fields=main.List_Tree.size
PENDING main.t sorry goals=2 [(n : Nat) ⊢ (= n n)] [(n : Nat) ⊢ (= 1 1)]
REFUSE Bool.or name_taken
POLICY main.t2 outside the policy: a.ax
DISCHARGE lib.Stack type
IMPL-ERROR lib.Box impl_type_arity: the view's 1, the implementation's 0
IMPL DEFINE lib.push equations=lib.push.eq_1
READ-ERROR main.shard unterminated_list: at byte 109
LOAD-ERROR a.shard import_cycle: a
UNUSED kernel.prog use=kernel.intmap.IntMap
THREAD tools.shardfmt.shardfmt.main ok
STATE pins.state.main.ok_chain ok
BIN main.hello entry main.main externs 1 trusts 1 requires 1/1
UNMET main.main_output
UNTHREADED main.main world_twice: parameter 1 used twice: main.write_line, main.write_line
BAD-BIN main.hello bad_entry: main.main does not take a World last
SIDECAR main.shard schema=1 current=3
RECON main.shard version=0 current=2
PROVED examples.tactic.use_split.eq_self req=1419798584914156123.0.0.3821081251922503 (by (exact (Eq.refl n)))
```

(Sources: `loader_test.shard`, `template_test.sh`, `derive_test.sh`, `define_test.sh` and
`state_test.sh` under `v3/kernel/test/`; the pins `by_sorry`, `policy_cross`, `view_arity`,
`read_error`, `cycle`, `bin_*`, `auto_schema_old`, `auto_recon_old`. The INIT, TEMPLATE, UNUSED,
THREAD and PROVED lines are from recorded runs; no test states them whole.)
