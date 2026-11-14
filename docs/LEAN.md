# shard for a reader who knows Lean 4

The logic of shard is Lean 4's type theory, and its base library is Lean's `Init`, imported from an
export of Lean v4.33.1 and checked by shard's own checker, K. Names, binder conventions and the
grammar of theorem names are Lean's. The surface is not: a file is s-expressions, and there is no
notation, no type-class search and no search inside a proof. This document holds three lists — what
is the same as Lean, what is refused and what to write instead, what exists only in shard — so that
an author who knows Lean 4 can write shard directly. `LANGUAGE.md` is the reference, `AUTHORING.md`
the how-to (it says how to check and run a file), `DECISIONS.md` says why a thing is this way.

Paths below are relative to `v3/`; "pin `X`" is the file `pins/loader/X/main.shard`.

## The one shape

Everything is a parenthesised prefix form, `(HEAD ARG…)`. An application is `(f x y)`; an operator
is a head like any other, `(+ n 1)`; a top-level form is `(KEYWORD …)`. A binder list is
`((x T) (y U))` — every binder with its type, `()` when there is none — and a binder is implicit
when written `(x T implicit)` (also `strict`, `inst`). Universe parameters are written on the name,
`NAME.{u v}`, where it is declared and where it is cited. `match` takes one scrutinee and one row
`(PATTERN BODY)` per case. A comment runs from `;` to the end of the line. Nothing is inferred that
the written inputs do not determine: a missing implicit argument, instance or universe is a refusal,
never a default. The one default: a bare numeral is a `Nat`.

| Lean 4 | shard |
|---|---|
| `f x y` | `(f x y)` |
| `n + 1` | `(+ n 1)` |
| `fun (x : Int) => x + n` | `(fun ((x Int)) (+ x n))` |
| `∀ (z : Nat), z ∈ t → y ≤ z` | `(forall ((z Nat)) (-> (List.Mem z t) (<= y z)))` |
| `let m := n + n; m + 1` | `(let ((m (+ n n))) (+ m 1))` |
| `match xs with \| [] => 0 \| _ :: r => len r + 1` | `(match xs (List.nil 0) ((List.cons _ r) (+ (len r) 1)))` |
| `def up1 (n : Nat) : Int := n + 1` | `(def up1 ((n Nat)) Int (+ n 1))` |
| `theorem t2 (n : Nat) (h : n = 1) : n = 1 := by exact h` | `(theorem t2 ((n Nat) (h (= n 1))) (= n 1) (by (exact h)))` |
| `inductive Eq.{u} {α : Sort u} (x : α) : α → Prop \| refl : Eq x x` | `(inductive Eq.{u} ((α (Sort u) implicit) (x α)) (-> α Prop) (refl (Eq x x)))` |
| `@Eq.refl.{1} Nat one` | `(@Eq.refl.{1} Nat one)` |

## Reading a refusal

The loader checks a file and prints one record per declaration. A refusal names its reason, and its
message says what to write. The second column of list 2 is that reason.

- `REFUSE NAME REASON: MESSAGE` — a `fn` or a `derive` is refused; the file goes on.
- `READ-ERROR FILE REASON: MESSAGE` — a top-level form, a `def` or a `theorem` is refused; the
  reading of the file ends there. A refused proof step reads `step N: … — the goal: …`.
- `LOAD-ERROR FILE REASON: MESSAGE` — an `import`, a `fulfills` or a sidecar (a file of stored
  proofs, list 3) is refused.
- `RUNNABLE fn NAME why=REASON` — no refusal: the function has its program and no definition in the
  logic, so no theorem can cite it.
- `PENDING NAME REASON` — read, and it admits nothing yet (`sorry`, `measure`, `auto_missing`).

## 1. The same as Lean

| Lean | shard | note |
|---|---|---|
| `List.length`, `Option.getD`, `Nat.sub_lt`, `Fin.isLt` | the same names | every constant of the `Init` export under its own name, with the binders the export declares (an implicit binder stays implicit); a namespace is a name prefix |
| the grammar of theorem names: `List.length_append`, `Nat.zero_and`, `List.mem_cons` | the same | guess a lemma's name as in Lean: `(by (apply List.length_append))` (pin `one_spelling`). A name the export lacks is `unknown_constant`, with no suggestion; one past the file's horizon is `above_horizon` (list 2). `AUTHORING.md` §15 shows how to search the export's names and read a lemma's statement |
| `open M`, `open M (a b)` | `(use M)`, `(use M a b)` | opens a name prefix: `(use Init.List cons nil)` (`examples/path/path_main.shard`). Init's own names need no `use`; a constructor is a name under its type, so `nil` needs `(use Init.List)` or is written `List.nil`. A `type` opens its own constructors for the rest of its file; in a pattern a bare constructor resolves by the scrutinee's type |
| `@f`, `f.{u}` | `@f`, `f.{u}` | `@` makes every binder explicit; without it implicit and instance binders are filled and a written argument goes to the next explicit binder (pin `elab_implicit`) |
| `fun (x : T) => e`, `∀ (x : T), P`, `∃ (x : T), P`, `A → B → C`, `let x : T := v; e` | `(fun ((x T)) e)`, `(forall ((x T)) P)`, `(exists ((x T)) P)`, `(-> A B C)`, `(let ((x T v)) e)` | `->` nests to the right; a `let` is sequential and may leave the type out, `(let ((x v)) e)` (pin `elab_bridge`) |
| `a = b`, `a ≠ b`, `a ≤ b`, `a < b`, `a ≥ b`, `a > b`, `P ∧ Q`, `P ∨ Q`, `¬P`, `P ↔ Q` | `(= a b)`, `(!= a b)`, `(<= a b)`, `(< a b)`, `(>= a b)`, `(> a b)`, `(and P Q)`, `(or P Q)`, `(not P)`, `(iff P Q)` | `Eq`, `Ne`, `Nat.le` or `Int.le`, `Nat.lt` or `Int.lt`, `And`, `Or`, `Not`, `Iff`; `>` and `>=` are `<` and `<=` with the operands swapped. A comparison is at `Nat` or `Int` by its first operand that is not a bare number (pin `op_expected`) |
| `true`, `false`, `a && b`, `a \|\| b`, `!a`, `a == b` | `Bool.true`, `Bool.false`, `(and a b)`, `(or a b)`, `(not a)`, `(BEq.beq a b)` | at `Bool` operands `and`, `or`, `not` are `Bool.and`, `Bool.or`, `Bool.not` (pin `bool_ops`); `true` and `false` are bare after `(use Init.Bool)`. Lean's two bridges hold: a proposition where a `Bool` is expected is `Decidable.decide`d; a `Bool` given as an argument where a proposition is expected (to `Not`, `And`, `iff`, as an `if`'s condition) is `(= b true)`. As a theorem's statement, a binder's type, a side of `->` or a `forall`'s body a `Bool` is refused `not_a_type`: write `(= b true)` |
| `if c then t else e` | `(if c t e)` | `ite`; the decision is found for `<` and `<=` at `Nat` and `Int`, and for `=` at `Nat`, `Int`, `Bool` and a type with a derived equality; a `Bool` condition is `c = true` (pins `elab_numeral`, `rename_ok`) |
| `if h : c then t else e` | `(dif h c t e)` | `dite`, with `h : c` in `t` and `h : ¬c` in `e`: `(dif h (< i n) (Option.some (Fin.mk i h)) Option.none)`; in a proof `(rw (dif_pos h))`, `(rw (dif_neg h))` (pin `dif_ok`) |
| `match xs with \| [] => a \| y :: t => b` | `(match xs (List.nil a) ((List.cons y t) b))` | `_` is the wildcard, the first matching row wins, patterns nest; two scrutinees are two nested matches (`tools/shardfmt/shardfmt.shard`, `same_bytes`). Numeral rows at `Nat` or `Int`, closed by `_` or a name, are Lean's value patterns: `(match n (0 10) (5 20) (_ 30))` (pin `literal_pattern`). In a `fn` they, or an `if`, are the way to take a `Nat` apart: the patterns `Nat.zero` and `(Nat.succ k)` are accepted in a `def` and refused in a `fn` (`unknown_head: Nat.succ is not a constructor`). A local whose type mentions the scrutinee is generalized, as in Lean (pin `dependent_match`) |
| `[a, b, c]`, `[]`, `x :: xs`, `xs ++ ys` | `(list a b c)`, `List.nil`, `(List.cons x xs)`, `(List.append xs ys)` | `(list …)` takes its type from its position (pin `elab_bridge`) |
| `0`, `42`, `(7 : Int)`, `(-7 : Int)`, `↑n` | `0`, `42`, `7`, `-7`, `n` | a numeral is a `Nat` unless an `Int` is expected (`Int.ofNat 7`, `Int.negSucc 6`). The one coercion inserted: a `Nat` where an `Int` is expected becomes `(Int.ofNat n)` (pin `elab_numeral`) |
| `a + b`, `a - b`, `a * b`, `a / b`, `a % b` | `(+ a b)`, `(- a b)`, `(* a b)`, `(/ a b)`, `(% a b)` | at `Nat`: `Nat.add`, `Nat.sub` (truncating), `Nat.mul`, `Nat.div`, `Nat.mod`; at `Int`: `Int.add`, `Int.sub`, `Int.mul` and the Euclidean `Int.ediv`, `Int.emod`. An expected `Nat` or `Int` decides, else the first operand that is not a bare number (pins `op_expected`, `op_int_div`) |
| `2 ^ k`, `a &&& b`, `a \|\|\| b`, `a ^^^ b`, `a <<< k`, `a >>> k` | `(Nat.pow 2 k)`, `(Nat.land a b)`, `(Nat.lor a b)`, `(Nat.xor a b)`, `(Nat.shiftLeft a k)`, `(Nat.shiftRight a k)` | by name. Init's lemmas stated in the notation apply all the same — a class method under Init's instance is read as its function: `(apply Nat.zero_and)` proves `(= (Nat.land 0 b) 0)` (pin `one_spelling`) |
| `min a b`, `max a b` | `(min a b)`, `(max a b)` | `Min.min`, `Max.max` at `Nat` and `Int` (pin `instance_table`). `max` at `Nat` needs `(import Init Nat.instMax)`; below it the refusal is `unknown_constant` or `unknown_head: max` and names no import |
| `l[i]?`, `l.getD i d`, `l.get i`, `l.take n`, `l.length`, `l.reverse` | `(List.get?Internal l i)`, `(List.getD l i d)`, `(List.get l i)`, `(List.take n l)`, `(List.length l)`, `(List.reverse l)` | call form, the arguments in the order of Init's signature; `std/list.shard` gives these their programs, so a `fn` can call them |
| `List.map f xs`, `List.filter`, `List.foldl`, `List.foldr`, `List.any`, `List.all` | the same: `(List.map (fun ((x Int)) (+ x n)) xs)` | in a `fn` too (`examples/template/add_offset.shard`); the function argument is compiled away (list 3, template) |
| instances: `instAddNat`, `Int.instAdd`, `instDecidableEqNat`, `List.instAppend` | filled from a fixed table | an instance binder at `Nat`, `Int`, `Bool` or `List` takes Init's instance, or a composition of them (`BEq` from a decidable equality): `(List.sum xs)` at `Int` (pin `instance_table`). Any other is `instance_needed` (list 2) |
| `Decidable`, `Nat.decEq`, `Nat.decLt`, `by decide` | the same; the step `decide` | `(theorem d2 () (< 1 2) (by decide))` (pin `by_core`); a decision is cited by name, `(Nat.decEq n 0)` |
| `structure Span where lo : Nat; hi : Nat; ok : lo ≤ hi` | `(structure Span () (lo Nat) (hi Nat) (ok (Nat.le lo hi)))` | constructor `Span.mk`, projections `Span.lo`; the list after the name holds the parameters. `{ lo := 1, … }` is `(Span.mk 1 1 h)` or `(make Span (lo 1) (hi 1) (ok h))`; `{ s with hi := h }` is `(with Span s (hi h) (ok p))` — `make` and `with` only for a structure without parameters (pins `struct_make`, `struct_params`) |
| `inductive`, with parameters, indices, universes | `(inductive Nat () Type (zero Nat) (succ (-> Nat Nat)))` | `(inductive NAME.{u} BINDERS TYPE (CTOR TYPE)…)`; plain data has the short form `(type Tree (Leaf) (Node Tree Tree))`. `T.rec`, `T.casesOn`, `T.noConfusion`, `T.CTOR.inj` exist as in Lean (pin `elab_confusion`) |
| `Subtype`, `⟨v, h⟩`, `s.val`, `s.property`; `Fin n`, `i.val`, `i.isLt` | `(Subtype.mk v h)`, `(Subtype.val s)`, `(Subtype.property s)`; `(Fin n)`, `(Fin.mk i h)`, `(Fin.val i)`, `(Fin.isLt i)` | at run time a `Subtype` is its value and a `Fin` its number. In a `fn` signature a subtype goes by a name: `(def Sorted () Type (@Subtype (List Nat) (List.Pairwise Nat.le)))`, then `(fn sorted_insert ((x Nat) (s Sorted)) Sorted …)` (`std/list.shard`) |
| `def up (n : Nat) : Int := n` | `(def up ((n Nat)) Int n)` | a `def` is logical: K checks it and nothing runs it; what runs is a `fn` (list 3). A structurally recursive `def` names its argument: `(def allz ((xs (List Nat))) Prop (measure (struct xs)) (match xs …))` (pin `def_struct`) |
| `theorem t … := proof`, `:= by …`, `:= sorry` | `(theorem band_zero_r ((a Nat)) (= (Nat.land a 0) 0) (exact (Nat.and_zero a)))` (`std/bits.shard`), `(by STEP…)`, `sorry` | `sorry` admits nothing: the record is `PENDING NAME sorry`, and citing the theorem is `pending_obligation` (pin `sorry`). A theorem's `(a Type)` binder is explicit where the theorem is cited, `(rw (list_drop_len a n xs))`, unless written `(a Type implicit)`; a `fn`'s type parameters are implicit at its calls, `(fold_m IntAdd xs)` |
| `abbrev`, `opaque`, `axiom` | `(abbrev NAME BINDERS TYPE VALUE)`, `(opaque w_output ((w World)) (List ByteArray) List.nil)`, `(axiom NAME BINDERS TYPE)` | `propext`, `Quot.sound` and `Classical.choice` may stand in a proof's closure. Any other axiom needs `(trusts NAME)` in the file (pin `bin_ok`); without it the record is `POLICY NAME outside the policy: …` (pin `policy_axiom`) |
| `f.eq_1`, `f.eq_def` | `f.eq_N` | a `fn`'s equations, one per leaf of its leading matches: `(rw (insert.eq_2 x y t))` (`std/list.shard`); `(unfold f)` applies them |
| `intro`, `exact`, `rfl`, `have`, `show`, `apply`, `cases`, `induction`, `decide`, `unfold`, `rw [h]`, `rw [← h]`, `simp only [l₁, l₂]`, `omega`, `sorry` | `(intro a b)`, `(exact t)`, `rfl`, `(have h P STEP…)`, `(show P)`, `(apply t BLOCK…)`, `(cases x (ARM…))`, `(induction x (ARM…))`, `decide`, `(unfold f)`, `(rw h)`, `(rw (<- h))`, `(simp_only l1 l2)`, `arith`, `sorry` | with `wf` and `reduce` (list 3) these are all the steps there are. An arm is `(CTOR (FIELD… IH…) STEP…)`, its name list written even when empty: `(induction xs ((nil () rfl) (cons (x r ih) (reduce lhs) (rw ih) rfl)))`. `apply` and `rw` take one block per open premise, `(rw if_pos (exact h))`; `(rw h (occ 2))` rewrites one occurrence (pin `by_core`) |
| `cases h : x`, `rcases Nat.lt_or_ge y x with h \| h`, `obtain ⟨w, hw⟩ := h` | `(cases x h (ARM…))`, `(cases (Nat.lt_or_ge y x) ((inl (hlt) …) (inr (hge) …)))`, `(cases h ((intro (w hw) …)))` | `cases` takes a local or a term and splits it by its constructors (`std/list.shard`, `models/imp/app.shard`) |
| `⟨w, h⟩`, `⟨h1, h2⟩`, `Or.inl h` | `(Exists.intro w h)`, `(And.intro h1 h2)`, `(Or.inl h)` | the anonymous constructor is written as the named one, under `exact` (`std/list.shard`) |
| `if_pos`, `if_neg`, `dif_pos`, `dif_neg`, `absurd`, `False.elim`, `Eq.symm`, `Eq.trans`, `congrArg` | the same | Init's, by name: `(rw (if_neg (Iff.mpr (@Nat.not_le x y) hlt)))` rewrites a refuted `if` (`std/list.shard`) |
| `induction xs generalizing h` | `(induction xs (ARM…))` | a hypothesis whose type mentions `xs` is generalized by the step itself; to generalize another, state it as an implication and `(intro h)` in each arm (`std/list.shard`, `insert_sorted`) |
| `"text"` | `"text"` | a `String` literal, with the escapes `\n \t \r \\ \" \xHH \u{HHHH}`; programs handle text as `ByteArray` (`std/bytes.shard`) |
| `def twice (f : Int → Int) (x : Int) := f (f x)` | `(fn twice ((f (-> Int Int)) (x Int)) Int (f (f x)))` | a function-typed parameter is allowed, and a theorem quantifies over `f` as in Lean (pin `template_named`); how it runs is list 3's template |
| a `structure` of operations and laws, a theorem over an arbitrary one (`Monoid α`) | `(structure Monoid ((a Type)) (op (-> a a a)) (unit a) (op_assoc …) (unit_left …))`, `(def IntAdd () (Monoid Int) (Monoid.mk Int.add 0 Int.add_assoc Int.zero_add))` | passed as an argument where Lean would search an instance: `(fold_m IntAdd xs)`; the theorem over an arbitrary `m` applies at `IntAdd` (pin `template_package`) |

## 2. Refused, and what to write instead

| Lean | the loader prints | write instead |
|---|---|---|
| `partial def`; a recursive function with no termination argument | `lean_form`; `realize_recursion: f calls itself without a measure` | a measure: `(fn count_down ((n Nat)) Nat (measure n) (if (= n 0) 0 (count_down (- n 1))))` (pin `by_wf`); structural recursion is `(measure (struct xs))` |
| `termination_by`, `decreasing_by` | `lean_form` | `(measure M)` after the return type, `M` a `Nat`. Each recursive call's descent is an obligation `f.dec_N`, proved after the `fn`: `(fulfills count_down.dec_1 (by (intro n h) (exact (Nat.sub_lt (Nat.pos_of_ne_zero h) (Nat.lt_succ_self 0)))))` (pin `by_wf`; list 3) |
| a measure of type `Int` | `RUNNABLE fn f why=measure_type` | a `Nat` measure: `(measure (Int.natAbs n))` (pin `measure_natabs`) |
| a recursive call on the parameters themselves | `realize_descent: … calls itself on its own parameters — no measure decreases` | pass a smaller argument (pin `realize_measure_loop`) |
| `mutual … end` | `lean_form` | no keyword. Write the functions one after another, each with `(measure N)`; the loader finds the group: `(fn ping ((n Nat)) Nat (measure n) (if (= n 0) 0 (pong (Nat.sub n 1))))`, `pong` likewise, then `(fulfills ping.dec_1 …)` and `(fulfills pong.dec_1 …)` (pin `mutual_measure`) |
| a member of such a group with `(measure (struct xs))` or no measure; with a type parameter | `RUNNABLE fn f why=group_measure`; `why=group_shape` | `(measure N)` with `N` a `Nat` on every member — `(derive T size)` gives one for a data type (list 3); no type parameters in a group (pins `define_mutual`, `mutual_no_measure`, `mutual_shape`) |
| `unsafe` | `lean_form` | nothing: every declaration is checked |
| `notation`, `infix`, `infixl`, `infixr`, `prefix`, `postfix`, `syntax`, `macro`, `macro_rules`, `elab`, `elab_rules`, `declare_syntax_cat` | `lean_form` | a `fn` or a `def` under an ordinary name (pin `lean_notation`); the surface is fixed. A proof procedure is a library, never a syntax extension (the tactic rows below) |
| `open M` | `lean_form` | `(use M)`; an `import` opens nothing (pin `no_open`) |
| `lemma`, `example` | `lean_form` | `(theorem NAME BINDERS PROP PROOF)`: every theorem has a name |
| `noncomputable`, `private`, `protected` | `lean_form` | no modifiers. A `def` never runs; what a directory module exports is its interface file (list 3) |
| `universe u`; `variable`, `namespace`, `section` | `lean_form`; `reserved_form` | universes on the name and binders on each declaration: `(inductive Eq.{u} ((α (Sort u) implicit) (x α)) (-> α Prop) (refl (Eq x x)))` (pin `policy_axiom`). A namespace is a dotted name: `(fn Byte.ofNat ((n Nat)) UInt8 …)` (`std/bytes.shard`) |
| `deriving DecidableEq, Ord, Repr` | `lean_form` | after the type: `(derive Suit eq (ord structural) render)` (`examples/derive/derive_main.shard`) |
| `@[implemented_by g]` | `lean_form` | `(realize NAME BINDERS RET BODY)` gives a constant its program, the equations proved (list 3) |
| `attribute [simp]`, `set_option` | `lean_form` | none: `rw` and `simp_only` take their lemmas by name, and the elaborator has one behaviour |
| `instance : Add T`, `class C` | `reserved_form` | there are no user classes. Pass the structure as an argument (list 1, `Monoid`); for equality, ordering and printing use `derive` |
| `import Init.Data.List.Basic` | `LOAD-ERROR … bad_import: (import "FILE") or (import Init NAME)` | `(import Init List.take_succ_cons)` (`std/list.shard`): Init is imported up to a named declaration of the export, the file's **horizon**. A file that imports a shard file inherits that file's horizon |
| a name of Init past the horizon | `above_horizon: X is declaration N of the export, at or above the module's horizon H …: write (import Init X)` | what the message says (pin `init_above_horizon`) |
| `#eval e`, `#check e`, `#print n` | `unknown_form` | a theorem by evaluation: `(theorem kind_7 () (= (kind 7) 2) (exact (Eq.refl 2)))` (pin `literal_pattern`). Running and printing a program: `AUTHORING.md` |
| `xs[i]!`, `xs.head!`, `o.get!` — any name ending in `!` | `inhabited_default` | the `Option` form or a supplied default: `(List.get?Internal xs i)`, `(List.getD xs i d)`, `(Option.getD o d)` (pin `inhabited_default`) |
| infix: `(n + 1)` | `infix_form: S is prefix notation: write (+ A B) …` | `(+ n 1)` (pin `infix_form`) |
| `-a` | `bad_shape: (OP a b)` | `(- 0 a)`, or Init's `(Int.neg a)`; a negative literal is written `-3` (pins `op_expected`, `elab_numeral`) |
| `a == b`, `a && b`, `a \|\| b`, `!a`; `∀ ∃ λ → ↔ ¬ ∧ ∨ ≤ ≥ ≠ ℕ ℤ ×` | `lean_symbol: X is Lean's spelling: write …` | the spelling the message gives: `(BEq.beq a b)` or the proposition `(= a b)`; `(and a b)`, `(or a b)`, `(not a)`; `(forall ((x T)) P)`, `(-> A B)`, `Nat`, `(Prod A B)` (pin `lean_symbol`) |
| `(lt a b)`, `(le a b)`, `(int_eq a b)` — the comparison spellings of older shard sources | `renamed_primitive` | `(< a b)`, `(<= a b)`, `(= a b)` (pins `rename_refused`, `rename_ok`). Init's `Nat.lt` and `Nat.le` stay citable by name |
| `(e : T)` | no such form | the position's expected type decides; to state one, `(let ((r T e)) r)` |
| `(3 : Fin 5)` | `type_mismatch: expected (Fin 5), found Nat` | `(Fin.mk 3 h)` with `h` a named theorem: `(theorem one_lt_two () (< 1 …) (by (decide)))`, cited in `(@Fin.mk … 1 one_lt_two)` (`std/list.shard`) |
| `⟨3, by decide⟩` — `by` inside a term | `lean_symbol` | prove the fact as a named theorem and cite it (pin `lean_by_term`) |
| `if p ∧ q then …`, `if a ≠ b then …`, an `if` on any proposition without a known decision | `no_decision` | nest the tests, `(if (<= 48 c) (<= c 57) false)` (pin `elab_bridge`); test `=` and swap the branches; or `(@ite T C DEC t e)` with the instance written. For `=` at a type of your own, `(derive T eq)` first |
| a `match` where no type is expected | `match_motive` | write it as a definition's value or an argument, or `(let ((r T (match …))) r)` (pin `match_motive`) |
| a numeral or a string under a constructor in a pattern | `literal_pattern` in a `def` or a statement. In a `fn` a numeral there is not refused: the function has its program and no definition (`RUNNABLE fn F why=no_l_identity`) | bind the field and compare with an `if` (pin `match_literal`) |
| a missing case | `match_not_exhaustive` | add the case, or a `_` row (pin `match_not_exhaustive`) |
| an instance outside the table: `ite True 1 2`, a user class | `instance_needed` | `@` and the instance: `(@ite Nat (Nat.lt a b) (Nat.decLt a b) b a)` (pins `elab_instance`, `elab_numeral`) |
| an implicit argument nothing determines: `[] = []` | `unsolved_implicit` | `@` and the argument: `(@List.nil a)` (pin `elab_unsolved`; `std/list.shard`) |
| an `Int` where a `Nat` is expected | `type_mismatch: expected Nat, found Int` | a conversion by name, `(Int.toNat i)`; only `Nat` to `Int` is inserted (pin `elab_mismatch`) |
| a lemma applied to more arguments than it has explicit binders | `function_expected: applied to N more arguments than its type has` | the export's binders: cite it bare and let the goal fill the implicits, `(apply NAME)`, or write `@NAME` with every argument |
| `omega` | `unknown_step` | `arith`, its extra facts written as terms: `(arith (strip_le a))` (pin `derive_size_plain`). It is linear arithmetic over `Int` and `Nat` with no case split: a `≠` hypothesis is no row, and `Nat.sub`, `Nat.pow` and a product of variables are opaque. When no combination of the rows refutes them it prints `arith_failed` with the rows: split first with `cases`, state the claim without subtraction, rewrite a power to its literal |
| `simp`, `simp [*]`, `simp_all`, `norm_num`, `ring`, `linarith`, `aesop`, `exact?`, `constructor`, `rcases`, `obtain`, `calc`, `subst`, `injection`, `assumption`, `contradiction` | `unknown_step` | the steps of list 1: `(simp_only L…)` with the lemmas named, `(unfold f)`, `(reduce)` for computation, `(exact (And.intro h1 h2))`, `(exact (Eq.trans h1 h2))`, `(exact (absurd h hn))`. Search runs offline: `auto` as the proof (list 3) |
| `macro "my_tac" : tactic => …`, `elab … : tactic` | `lean_form` | an ordinary `fn` from the goal's text to a block of steps, named in the theorem: `(auto (tactic examples.tactic.bool_split.solve))` (`examples/tactic/use_split.shard`) |
| `cases h` with `h : List.Mem a l` or `h : a = b` — an indexed family | `cases_indexed` | Init's characterizing lemma: `(cases (Iff.mp List.mem_cons h) ((inl (e) …) (inr (m) …)))` (`std/list.shard`); an equation is used by `rw` |
| a lemma about `2 ^ n` applied to a hypothesis about a literal | `type_mismatch` | state the hypothesis at the power, since the exponent is not read off a literal: `(have ha2 (< a (Nat.pow 2 32)) (exact ha1))` (`std/bits.shard`) |
| a lambda, a partial application or a function parameter as a **value** in a `fn` — a `let`, a constructor's field, a result | `function_value` | pass it as an argument of a `fn` with a function-typed binder (list 3, template), or write the loop out. In a `def` or a `theorem` a `fun` is fine (pin `template_escape`) |
| in such a `fn`, an argument built from its own function parameter: `(twice (fun ((y Int)) (f (f y))) x)` | `function_value: … a static parameter is passed on, never transformed` | pass the parameter on unchanged (pins `template_transform`, `template_passed`) |
| polymorphic recursion: `f` at `a` calling itself at `List a` | `realize_descent` | recurse over a value (pin `template_growth`) |
| a `def` or an Init function called from a `fn` when it has no program | `no_realization: NAME: no realization attached — realize it first` | write the callee as a `fn`, or `(realize NAME …)` it (list 3) |
| `{ xs : List Nat // P xs }` written in a `fn`'s signature | `(@Subtype (List Nat) (List.Pairwise Nat.le))` there is `REFUSE F unknown_type: @Subtype — its signature has no L reading: type_mismatch: expected (Sort 0), found (Sort 1)`; `(Subtype (List.Pairwise Nat.le))` is not refused, and the function is a program only (`RUNNABLE fn F why=e_only_type`) | name the carrier by a `def` (list 1, `Subtype`) |
| `IO`, `do` | no such forms | an effect is a function that takes the `World` and returns it (list 3) |
| in a body that has no reading in the logic — it mentions a `Symbol`, or its file imports no Init — a `dif`, `absurd`, a `fun`, `-` at `Nat` | `dif_needs_init`, `absurd_needs_init`, `function_value`, `nat_operator` | give the function a signature over Init's types (pins `dif_needs_init`, `absurd_e_first`); `Symbol` is the toolchain's own type |

## 3. Only in shard

| form | meaning | example |
|---|---|---|
| `(fn NAME BINDERS RET (measure M)? BODY)` | a `def` and the program that runs, in one. K gets a definition with its equations `NAME.eq_N` (record `DEFINE`); the compiler gets the body with types and proofs erased. Where no definition can be built the function still runs and no theorem can cite it (`RUNNABLE`) | `(fn insert ((x Nat) (xs (List Nat))) (List Nat) (measure (struct xs)) (match xs …))` (`std/list.shard`) |
| `(measure M)`, `(fulfills f.dec_N PROOF)` | well-founded recursion on a `Nat`. Each recursive call states the obligation `f.dec_N` (`PARAM f.dec_N obligation`); the function is `PENDING f measure` until each is proved (`DISCHARGE f.dec_N proved`). The goal: the parameters — one `PProd` tuple when a function with several calls only itself, by name in a group —, then for each enclosing `match` row its fields and its equation, for each enclosing `if` its hypothesis, and last the measure at the call `<` the measure at the parameters | pins `by_wf`, `discharge_measure`, `mutual_three` |
| functions that call each other | no keyword. The loader finds the group when its last member is read — a call to a function defined later in the file is allowed — and defines it as one well-founded recursion; each member keeps its own equations and obligations | pins `mutual_measure`, `mutual_forward` |
| `(derive T size)` | `T.size`, a `Nat` built from the type's recursor, with equations `T.size.eq_N`: the measure for recursion over a type nested through `List` (`List_T.size` comes with it). A size has no program. It is derived by a form of its own, not beside `eq` | `(derive Tree size)`, `(measure (Tree.size t))` (pin `derive_size`) |
| `(derive T eq (ord structural) render)`, `(derive T (eq by NAME))` | generates the functions `T.decEq`, `T.compare`, `T.render`; after it `(if (= a b) …)` works at `T`. An instantiation is derived on its own, `(derive (List Card) eq)`; `(CAPABILITY by NAME)` registers a function you wrote | `examples/derive/derive_main.shard`, `std/derive.shard` |
| `(realize NAME BINDERS RET BODY (equations THM…)?)`, `(realize NAME (view))` | gives a constant that exists — an Init function, a `def` — its program. The body's equations `NAME.realize_N` are proved by `rfl` or by the theorems named; `(view)` reads the program off Init's own definition | `(realize Option.getD (view))` (`std/list.shard`) |
| `(type T (C FIELD…)…)`, `(record R (f T)…)`, `(make R (f v)…)`, `(with R r (f v)…)` | data: a `type` is an inductive with positional fields; a `record` adds accessors `R.f` and updates | `(record Point (x Int) (y Int))`, `(make Point (y 0) (x 0))` (pin `record_basic`) |
| `(if x t e)` on a value | the condition may be a value of a type with exactly two constructors; the second constructor takes `t`, in a `fn` under the evaluator and in a native binary alike | `(type Dec (No) (Yes))`, `(def flip ((d Dec)) Dec (if d No Yes))` (pin `elab_bridge`) |
| `(import "FILE.shard")`, `(import "DIR")`, `(use MODULE)` | a file by its path, a directory module through its interface. A module's name is its path from the package root, `.` for `/` | `(import "../../std/host.shard")`, `(use std.host)` (`examples/path/path_main.shard`) |
| `mod.req.shard`: `(sig type T)`, `(sig fn NAME BINDERS RET)`, `(requirement NAME BINDERS PROP)`; `(fulfills NAME PROOF)` | a directory module's interface: opaque types, signatures without bodies, laws. The implementation proves each law with `fulfills`; a consumer sees the interface alone | pin `view_basic` |
| `(extern NAME BINDERS RET)`, `World`, `main` | effects: an extern takes the `World` and returns it. The entry is a `fn` whose last parameter is the `World`; under the loader's `--run` the ones before it are read from the command line, and a native binary's `main` takes the `World` alone and reads its arguments with `get_args`. `IO.println (toString n)` is `(write_line (Bytes.ofNats (Nat.renderOnto n nil)) w)` | `(fn main ((i Nat) (w World)) World (write_line (show_at i) w))` (`examples/path/path_main.shard`) |
| `(bin NAME (entry F) (externs E…) (trusts A…) (requires THM…))` | a program together with the theorems it must satisfy and the axioms about externs they rest on | pin `bin_ok` |
| `(by STEP…)` | the block is the proof: each step is replayed, none searches. Beyond list 1: `(wf X MEASURE IH STEP…)`, well-founded induction on a `Nat` measure; `(reduce)`, `(reduce lhs)`, `(reduce rhs)` run every `fn` by its equations; `(reduce (using FACT…))` also rewrites by the facts as it runs — a fact is given its premises, `(if_pos h)`, and one that rewrote nothing is refused (`reduce_rule_unused`); `(arith only FACT…)` takes its rows from the goal and the named facts alone, and `(farkas K…)` states the certificate; `(rw L (occ K EXPECT))` names the instance it expects at the occurrence (`rw_occ_changed`) | pins `by_wf`, `reduce_using`, `arith_only`, `by_rw_occ_guard` |
| `auto` as a proof; `FILE.auto.shard` | the proof is found offline by `prove.shard` and stored in a sidecar beside the file. Loading replays it and never searches; without an entry the record is `PENDING NAME auto_missing …`. It works as a theorem's proof, not in a `fulfills` (`OPEN.md` §1) | `(theorem two_three () (= (+ 2 3) 5) auto)` (`examples/auto/auto.shard`) |
| `(auto (tactic M.FN) HINT…)` | the same with a procedure of your own: `M.FN` is a `fn` from the goal's text to a block of steps | `examples/tactic/` |
| a template | a `fn` with a function-typed binder (record `TEMPLATE`). It is defined in the logic as any `fn` and has no program of its own: each call at a named function, a partial application `(addk 3)` or a lambda gets a specialized copy (record `INSTANCE`), so no closure exists at run time. One load builds at most 1,024 instances of a template (`specialization_budget`; `--instance-budget N` changes it) | pins `template_lift`, `template_named`, `template_partial`, `template_budget` |
| a package | a `structure` with a function-typed field. It is never a run-time value; a binder of it is specialized away as a function is | pin `template_package` |
| what a type is at run time | a `Fin` is its number, a `Subtype` its value, and `UInt8`, `Array`, `ByteArray`, `String` each their one run-time field; proofs and types are erased | `std/bytes.shard` |
