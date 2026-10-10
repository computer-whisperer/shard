# `docs/LEAN.md` — V3 for a reader who knows Lean 4

Law §5.3 adopts Lean's names, conventions and theorem-naming grammar
by explicit decision; §5.4 asks for this document: **three lists** —
what is the same as Lean, what is refused (why, since when, and what
to write instead), what is shard-only — in dozens of rows, so that an
author who knows Lean 4 and reads only `v3/LANGUAGE.md` and this file
writes V3 without folklore (the T9 gate). The pin is Lean **v4.33.1**
(`v3/README.md`); every name below is that export's. The normative text
is `v3/LANGUAGE.md` (§ numbers below are its); the law is
`docs/FOUNDATION.md` (cited as "law §N"). Written at slice 3.27 landing
4 (2026-10-10) from the probes and the landings; a row is added when
an author trips where the document was silent.

**The one shape.** A file is s-expressions, prefix everywhere: `n + 1`
is `(+ n 1)`, `f x y` is `(f x y)`, `fun x => e` is `(fun ((x T)) e)`.
A top-level form is `(KEYWORD …)`. Nothing is inferred that the
inputs do not determine (law §5.2): a missing implicit, instance or
universe is a refusal with the pointer, never a default — the one
default is a numeral's `Nat`.

**Checking a file** (`v3/README.md`): the loader is the checker.

```
read EXPORT INDEX < <(v3/kernel/test/init_index.sh)        # the pinned export and its index
./rust_bootstrap/target/release/eval direct v3/kernel/load.shard \
    --root ROOT --init $EXPORT --init-index $INDEX ROOT/FILE.shard   # paths repo-relative
```

`ROOT` is a directory under `v3/` (the package root; a file's module
name is its path from the root, `/` as `.`; `--root v3` and
`(import "../std/list.shard")` with `(use std.list)` for a file beside
`std/`); `--run M.FN FILE -- ARG…` runs a `fn` whose last parameter is
the `World` — a successful run prints the program's output and **no
records** (load the file without `--run` to see them); `--dump` prints
what was defined. The records: `ACCEPT NAME` (K checked it), `DEFINE
NAME equations=…` (a `fn`'s definition and equations), `REALIZE NAME`,
`RUNNABLE NAME why=…` (runs, no L meaning yet — every `main` that calls
an extern is `no_l_meaning`), `REFUSE NAME REASON` (the declaration
refused, the file continues), `READ-ERROR FILE REASON` (the file stops
at its first), `PENDING NAME sorry|measure`, `INIT NAME: horizon N`
(the export through NAME, N the ordinal past it; `INIT -:` when the
horizon is inherited from an imported file). **Which refusals end the
file:** a top-level form the reader refuses (`lean_form`,
`reserved_form`, `unknown_form`, `bad_import`) and any refusal inside a
`def` or a `theorem` (`above_horizon`, `inhabited_default`,
`lean_symbol`, `type_mismatch`, a step's) are read errors; a refusal
of a `fn`'s body is a `REFUSE` record and the file goes on — so one
refused Lean form per file shows them all.

## 1. The same as Lean

| Lean | here | note |
|---|---|---|
| `List.length`, `Option.getD`, `Subtype.val`, `Nat.lt_of_lt_of_le`, `Fin.isLt` | the same name | every constant of the export under its name, with the binders the export declares (implicit stays implicit); namespaces are name prefixes |
| the theorem-naming grammar (`List.length_append`, `Nat.add_comm`, `Nat.le_of_lt_succ`, `List.mem_cons`, `f.eq_1`) | the same | a never-seen lemma is cited by guess; a wrong guess is `init_name_not_found` (the export has no such name), a right guess past the module's horizon is `above_horizon` with the `(import Init NAME)` to write (§3.1) |
| `open M`, `open M (a b)` | `(use M)`, `(use M a b)` | opens a name prefix; `use Init` is implicit in a file that imports Init; **`use` opens no constructors** — `nil`, `zero`, `some` bare need `(use Init.List nil cons)` or the qualified `List.nil` (§13 item 7) |
| `@f` | `@f` | every binder explicit; otherwise implicit and instance binders are filled at each citation and a written argument goes to the next explicit binder (§5.1) |
| `fun (x : T) => e`, `∀ (x : T), P`, `∃ (x : T), P`, `A → B` | `(fun ((x T)) e)`, `(forall ((x T)) P)`, `(exists ((x T)) P)`, `(-> A B)` | binders are `((x T) (y U))`; `(-> A B C)` is right-nested; `let x : T := v; e` is `(let ((x T v)) e)` and sequential (R44) |
| `a = b`, `a ≠ b`, `a ≤ b`, `a < b`, `P ∧ Q`, `P ∨ Q`, `¬P`, `P ↔ Q`, `True`, `False` | `(= a b)`, `(!= a b)`, `(<= a b)`, `(< a b)`, `(and P Q)`, `(or P Q)`, `(not P)`, `(iff P Q)`, `True`, `False` | the identities `Eq Ne LE.le LT.lt And Or Not Iff` at the first operand's type; `>`/`>=` are the flipped `<`/`<=` |
| `Bool`: `true`, `false`, `a && b`, `a \|\| b`, `!a`, `a == b` | `Bool.true`, `Bool.false`, `(and a b)`, `(or a b)`, `(not a)` at `Bool` operands (`Bool.and or not`); `(BEq.beq a b)` at `Nat`/`Int` | `&&`, `\|\|`, `!`, `==` are lexed but are no operators of the elaborator — `(and …)` dispatches on the operand's type; a proposition where a `Bool` is expected is `decide`d, a `Bool` where a proposition is expected is `(= b true)` (Lean's coercions, §5.1) |
| `if c then t else e` | `(if c t e)` | `ite` with the decision the elaborator knows: `<`, `<=`, `=` at `Nat`/`Int`/`Bool`, a derived `eq`; else `no_decision` with the pointer to `@ite` |
| `if h : c then t else e` | `(dif h c t e)` | `dite`; `h : c` in `t`, `h : ¬c` in `e` (§8.4 slice 3.24); `(rw (dif_pos h))`, `(rw (dif_neg h))` in proofs |
| `match x with \| nil => a \| cons y t => b` | `(match x (List.nil a) ((List.cons y t) b))` | a generated matcher `OWNER.match_N` as Lean's; first match wins; a constructor's bare name resolves in a **pattern** by the scrutinee's type (`nil`, `(cons y t)`); a local whose type mentions the scrutinee is generalized (Lean's revert; §8.4 slice 3.27 rule 7) — so `i : Fin (List.length xs)` reaches the `nil` row as `Fin (List.length List.nil)` |
| `[a, b, c]`, `[]` | `(list a b c)`, `List.nil` | the constructor chain of the expected type's list (§5.1) |
| `absurd h hn`, `False.elim h` | `(absurd h hn)`, `(False.elim h)` | Init's; in a `fn` the refuted row is one of these, erased to an unreachable row (slice 3.27 rule 7) |
| numerals `0`, `42`, `(7 : Int)`, `-7` | `0`, `42`; `7` and `-7` where the position expects `Int` | `Nat` unless the expected type is `Int` (`Int.ofNat 7`, `Int.negSucc 6`); numerals are elaborated last, so `(List.cons 0 nil)` at `(List Int)` reads `0` at `Int` (§5.1, §5.3) |
| `↑n`, `(n : Int)` | inserted | the one coercion: a `Nat` at an expected `Int` becomes `(Int.ofNat n)`; nothing else is inserted — an `Int` at a `Nat` is `type_mismatch` (`Int.toNat` by name) |
| `a + b`, `a - b`, `a * b`, `a / b`, `a % b` | `(+ a b)`, `(- a b)`, `(* a b)`, `(/ a b)`, `(% a b)` | at `Nat`: `Nat.add sub mul div mod` (subtraction saturating); at `Int`: `Int.add sub mul ediv emod` — `/` and `%` **Euclidean**, `x / 0 = 0`, `x % 0 = x` (law §10.3); `Int.tdiv`/`Int.tmod` by name; the expected type decides an operator whose operands are numerals (slice 3.21b) |
| `2 ^ k`, `a &&& b`, `a \|\|\| b`, `a ^^^ b`, `a <<< k`, `a >>> k`, `xs ++ ys` | `(Nat.pow 2 k)`, `(Nat.land a b)`, `(Nat.lor a b)`, `(Nat.xor a b)`, `(Nat.shiftLeft a k)`, `(Nat.shiftRight a k)`, `(List.append xs ys)` | **one spelling**: Init's `HPow.hPow`/`HAnd.hAnd`/… under the closed instance and the function are the same term to the unifier and the rewriter (`kernel/spelling.shard`, slice 3.27 rule 2), so Init's lemmas stated with `^` apply to `Nat.pow` goals and back |
| `min a b`, `max a b` | `(min a b)`, `(max a b)` | `Min.min`/`Max.max` under the table's instance |
| `l[i]?`, `l.getD i d`, `l.get i`, `l.take n`, `l.drop n`, `l.reverse`, `l.length` | `(List.get?Internal l i)`, `(List.getD l i d)`, `(List.get l i)` with `i : Fin`, `(List.take n l)`, `(List.drop n l)`, `(List.reverse l)`, `(List.length l)` | `l[i]?` is `GetElem?.getElem?` under `List`'s instance, `List.get?Internal` by definition (law §10.3 row 8); all realized in `v3/std/list.shard` so a `fn` can call them; `List.length (x :: t)` is `length t + 1` as in Lean |
| instances: `instAddNat`, `Int.instAdd`, `instDecidableEqNat`, `List.instAppend`, `instOfNatNat n`… | found | an instance-implicit binder at `Nat`, `Int`, `Bool`, `List` is filled from a table of the export's instances (slice 3.27 rule 3); one the table lacks is `instance_needed` → write `@NAME` with the instance |
| `Decidable`, `decide`, `Nat.decLt`, `Nat.decEq`, `instDecidableEqBool` | the same | `(decide)` in a proof is `of_decide_eq_true (Eq.refl true)`, K's conversion the work; a `Decidable` instance written by hand is the export's name |
| `structure P where x : Nat; y : Nat` | `(structure P () (x Nat) (y Nat))` | `P.mk`, projections `P.x` as Lean's `Expr.proj` definitions; `{ x := 1, y := 2 }` is `(P.mk 1 2)`; a data-only structure may be `(record P (x Nat) (y Nat))` with `(make P (x 1) (y 2))` and `(with P p (x 3))` |
| `inductive T \| a \| b : T` | `(inductive T () Type (a T) (b T))`, or the sugar `(type T (a) (b))` for data | `T.{u}` declares universe parameters on the name; a `type`'s fields are non-dependent and E-eligible by construction |
| `Subtype`, `⟨v, h⟩`, `s.val`, `s.property`; `Fin n`, `⟨i, h⟩`, `i.val`, `i.isLt` | `(@Subtype T P)`, `(Subtype.mk v h)`, `(Subtype.val s)`, `(Subtype.property s)`; `(Fin n)`, `(Fin.mk i h)`, `(Fin.val i)`, `(Fin.isLt i)` | at runtime a `Subtype` is its value and a `Fin` its `Nat` (the proof erased; the registry's rows); see list 2 for a `fn` signature |
| `def f (x : T) : U := e` | `(def f ((x T)) U e)` | a **logical** definition: K checks it, nothing runs it (see list 3's `fn`) |
| `theorem t (x : T) : P := proof`, `… := by tac` | `(theorem t ((x T)) P (exact proof))`, `(theorem t ((x T)) P (by STEP…))` | binders `()` when none; `sorry` admits nothing and is reported |
| `abbrev`, `opaque`, `axiom` | `(abbrev …)`, `(opaque …)`, `(axiom NAME BINDERS TYPE)` under `(trusts NAME)` | §4, §9 |
| `f.eq_1`, `f.eq_def` | `f.eq_N` for a `fn`'s equations (`(rw (f.eq_2 x t))`, `(unfold f)`); `NAME.realize_N` for a realization's | one per leaf of the body's case tree (§7.2, §8.4); the reducer applies them where K's whnf would unfold Lean's `brecOn` encoding (slice 3.27 landing 3) |
| `propext`, `Quot.sound`, `Classical.choice` | the standard policy | a proof may use them; `sorryAx`, `Lean.ofReduceBool`, … are refused by policy (§9) |
| `intro`, `exact`, `rfl`, `have`, `show`, `apply`, `cases`, `induction`, `decide`, `unfold`, `rw`, `rw [← h]`, `simp only [l₁, l₂]`, `omega`, `sorry` | `(intro a b)`, `(exact t)`, `rfl`, `(have h P STEP…)`, `(show P)`, `(apply t STEP-BLOCK…)`, `(cases x (ARM…))`, `(induction x (ARM…))`, `(decide)`, `(unfold f)`, `(rw h)`, `(rw (<- h))`, `(simp_only l1 l2)`, `(arith FACT…)`, `sorry` | the steps of `(by …)` (§8.4 slices 3.20–3.21); an arm is `(ctor (field… ih…) STEP…)` — the fields in the constructor's order, then an induction hypothesis per recursive field, `(cons (x t ih) …)`; the list written even when empty, `(nil () …)`; `(rw h (occ 2))` for one occurrence; a step's refusal shows the goal it faced |
| `cases h : x with …`, `rcases Nat.lt_or_ge a b with h \| h` | `(cases x h (ARM…))`, `(cases (Nat.lt_or_ge a b) ((inl (h) …) (inr (h) …)))` | `cases` takes a local or a **term** — Init's `Or`/`Iff`/`Decidable` results split by their constructors; `(cases (Iff.mp List.mem_cons h) …)` for a membership |
| `⟨w, h⟩`, `⟨h1, h2⟩`, `Or.inl h` | `(Exists.intro w h)`, `(And.intro h1 h2)`, `(Or.inl h)` under `exact` | the anonymous constructor is the named one |
| `if_pos h`, `if_neg h`, `dif_pos h`, `dif_neg h`, `Nat.not_le`, `Nat.lt_or_ge`, `Nat.mod_two_eq_zero_or_one` | the same, by `rw`/`cases`/`exact` | Init's; `(rw (if_neg (Iff.mpr (@Nat.not_le x y) hlt)))` is the idiom for a refuted `if` |
| `calc` | `(have e1 …) (have e2 …) (rw e1) (rw e2) …`, or `(exact (Eq.trans h1 h2))` | there is no `calc` step |
| `show f (x :: t) = x + f t from rfl` — a `fn`'s step by conversion | `(have e (= (f (List.cons x t)) (+ x (f t))) (exact (Eq.refl (+ x (f t)))))` then `(rw e)` | K's conversion unfolds a `fn` and a realization at a constructor; `(unfold f)` and `(rw (f.eq_2 x t))` are the named routes (`v3/std/list.shard`'s `sum_append`) |
| a `String` literal, `Char` | `"…"` is K's `String` literal; `(Char.ofNat 97)` | §5.3; at runtime text is `ByteArray` (`v3/std/bytes.shard`) |

## 2. Refused — why, since when, and what to write

The record's reason is in the second column; every refusal carries its
pointer in the message. "Law" is `docs/FOUNDATION.md` §5.3's
departures unless another section is named.

| Lean | refusal | since | write instead |
|---|---|---|---|
| `partial def f …` | `lean_form` | law §5.3 departure 4 (ruled 2026-09-06); slice 3.14 | `(fn f BINDERS RET (measure M) BODY)` — the measure is the totality obligation; a structural one is `(measure (struct xs))` |
| `unsafe …` | `lean_form` | departure 4 | nothing: every declaration is K-checked |
| `notation`, `infix`, `infixl`, `infixr`, `prefix`, `postfix`, `syntax`, `macro`, `macro_rules`, `elab`, `elab_rules`, `declare_syntax_cat` | `lean_form` | departure 6 (ruled 2026-09-06): a notation is a private dialect an agent cannot guess | a `fn`, a `def` or a tactic; the surface is fixed with the elaborator |
| `open M` | `lean_form` | §3.1 | `(use M)`; `import` opens nothing |
| `lemma`, `example` | `lean_form` | §4 | `(theorem NAME BINDERS PROP PROOF)`: every theorem is named |
| `mutual … end` | `lean_form` | deferred (§12.4) | one measured `fn` at a time |
| `termination_by`, `decreasing_by` | `lean_form` | §4 | `(measure M)` inside the `fn`, after the return type |
| `noncomputable`, `private`, `protected` | `lean_form` | §8.4, §6.5 | no modifiers: a `def` never runs and a `fn` is a `def` with its realization; a module's public surface is its `sig` view |
| `universe u` | `lean_form` | §4 | on the name: `(def f.{u} …)` |
| `deriving Repr, DecidableEq` | `lean_form` | slice 3.19 | `(derive T eq (ord structural) render)` after the type |
| `@[implemented_by g]`, `@[extern "c"]` | `lean_form` (`implemented_by`) | §7 | `(realize NAME BINDERS RET BODY)` attaches the executable body to a declared constant, its equations proved |
| `attribute [simp] …`, `@[simp]`, `set_option …` | `lean_form` | slice 3.21 | none: `simp_only` and `rw` take their lemmas by name; the elaborator has one behavior |
| `instance : Add T := …`, `class C …` | `reserved_form: phase 3 (Stage 3)` | §4 | Init's instances at `Nat`, `Int`, `Bool`, `List` are the table's; a user class waits for Stage 3 |
| `namespace`, `section`, `variable` | `reserved_form` (undecided) | §4 | the dotted declared name: `(def List.foo …)` |
| `import Init.Data.List.Basic` | `bad_import: (import "FILE") or (import Init NAME)` | §3.1 | `(import Init NAME)` — a declaration, not a file: the export's prefix through NAME is the module's **horizon**; cite the highest-ordinal constant you use |
| `xs.get! i`, `xs.head!`, `o.get!` — any `NAME!` | `inhabited_default` | departure 4: an `Inhabited` default is not a value | `NAME?` (an `Option`) or `NAMED` (a supplied default): `(List.get?Internal xs i)`, `(List.getD xs i d)`, `(Option.getD o d)` |
| `n + 1`, `f x + g y` (infix) | `infix_form` | §2: prefix everywhere | `(+ n 1)`, `(+ (f x) (g y))` |
| `a == b`, `a && b`, `!a`, `a \|\| b`; `∀ ∃ λ → ↔ ¬ ∧ ∨ ≤ ≥ ≠ ℕ ℤ ×` | `lean_symbol` (lexed as a symbol, no operator of the elaborator; the pointer names the spelling) | law §5.3 (3), §2 | `(BEq.beq a b)` at `Nat`/`Int` or the proposition `(= a b)`; `(and a b)`, `(not a)`, `(or a b)` at `Bool` |
| `(3 : Fin 5)`, a numeral at `Fin n` | `type_mismatch: expected (Fin n), found Nat` | law §5.2 (`Fin` numerals ruled 2026-09-06): no wrapping literal | `(Fin.mk 3 h)` with the bound's proof `h` — a theorem `(theorem three_lt_five () (< 3 5) (by (decide)))` cited by name, or an Init lemma |
| `⟨3, by decide⟩`, `by` inside a term | `lean_symbol` (`by` is a theorem's proof, not a term; slice 3.20 rule 12) | §8.4 | prove the fact as a named theorem and cite it; or the term `(@of_decide_eq_true (< 3 5) (Nat.decLt 3 5) (Eq.refl Bool.true))` |
| `(n : Nat)` used at `Int` by `omega`'s cast reasoning | — | law §5.2: the one coercion inserted, never reasoned backwards | state the fact at the type it is proved at; `Int.ofNat` is a term you can write |
| `omega` on `a - b` at `Nat`, on `2 ^ k` | `arith_failed` | slice 3.21: `arith` is a Farkas certificate over the rows as written; `Nat.sub` and `Nat.pow` are atoms | state the claim subtraction-free (move the term across); rewrite the power to a literal first (`(rw pow2_32)`, `v3/std/bits.shard`) or carry a `(Nat.pow 2 32)` hypothesis; a product of variables is an atom |
| `omega` deciding a case split or a cut | `arith_failed` (no cut, no case split) | slice 3.21 | split first: `(cases (Nat.lt_or_ge a b) …)`, then `arith` in each arm |
| `simp`, `simp [*]`, `simp_all`, `norm_num`, `ring`, `linarith`, `aesop`, `exact?` | not forms of I | law §7: a step is a function of the node and the goal, no search at replay | `(simp_only L…)` with the lemmas named; `(arith …)` for linear facts; `(reduce)` for computation; the engine (`auto`, `v3/kernel/prove.shard`) searches **offline** and writes the block into the sidecar |
| `cases h` on `h : List.Mem a l`, `h : List.Pairwise R l`, on an `Eq` | `cases_indexed` | slice 3.20 (no `injection`/`subst` yet) | Init's iff: `(cases (Iff.mp List.mem_cons h) …)`, `(And.left (Iff.mp List.pairwise_cons h))`; an equation is used by `rw` |
| `induction xs generalizing h` | — (the hypothesis stays in the context, not generalized) | slice 3.20: `cases`/`induction` generalize the target only | state the theorem as an implication `(-> (P xs) (Q xs))`, `(intro h)` in each arm; `v3/std/list.shard`'s `insert_sorted` is the idiom |
| `@Nat.lt_of_lt_of_le a b c h1 h2` with a guessed implicit order | `type_mismatch` / `function_expected` | the binder order is the export's | cite bare and let unification fill the implicits from the hypotheses; write `@` only where every binder is known |
| `exact (Nat.le_refl a b)` — a theorem applied past its binders | `function_expected: applied to N more arguments than its type has` | §5.1 | the export's arity; `(apply NAME)` takes the goal's shape and opens the premises as goals |
| `Nat.or_lt_two_pow ha hb` with `ha : a < 4294967296` — a lemma stated at `2 ^ n` applied at a literal | `type_mismatch: expected (Nat.le (Nat.succ ?m) (HPow.hPow 2 ?n)), found (… 4294967296)` — the exponent is not inferred from a literal | slice 3.27 landing 3 | state the hypothesis at the power: `(have ha2 (< a (Nat.pow 2 32)) (exact ha))` — `exact` converts the power to the literal by K's computation once no metavariable is open; `(theorem t () (= (Nat.pow 2 32) 4294967296) (by (decide)))` holds |
| `fun x => x + k` passed to `List.map` in a `fn`; `f ∘ g`; `List.map f`; a partial application (`f` of arity 2 written bare) | `function_value: f in argument position (law §4.3)` — a zero-parameter `fn` written bare is its call on both routes (T9's finding) | E has no function values; closed lambdas as template arguments are slice 3.28 (T2/T3) | a named `fn` with the captured value as a parameter and the loop written out; in a `def` or `theorem` a `fun` is fine |
| a `def` called from a `fn` | `no_realization` (the callee has no E body) | §8.4: a `def` is logical | write the callee as a `fn` (its definition and equations are the same), or `(realize NAME …)` an Init constant |
| `def Sorted := { xs : List Nat // List.Pairwise (· ≤ ·) xs }` then `(fn f ((s Sorted)) …)` | `@Subtype T P` in a `fn` signature: a universe refusal today | slice 3.27 landing 3 (item 61) | name the carrier by a `def` and use the name: `(def Sorted () Type (@Subtype (List Nat) (List.Pairwise Nat.le)))`, `(fn f ((s Sorted)) …)` |
| `absurd`, `False.elim` in a `fn` whose signature has no L reading | `absurd_needs_init` | slice 3.27 landing 3 | give the `fn` an L signature (Init types) — the refuted row is an L term; `dif` likewise (`dif_needs_init`) |
| `xs.length - 1` at `Nat` in a `fn` whose signature has no L reading | `nat_operator` | slice 3.13 rule 3 (E-first route only) | `(Nat.sub a b)`, `(Nat.div a b)`, `(Nat.mod a b)`; on the typed route `(- a b)` at `Nat` is `Nat.sub` |
| `Array`, `HashMap`, `String` operations, `IO`, `do`, `Task` | not realized (`no_realization`), or phase 4–5 | §11 | `List`; `ByteArray` and `v3/std/bytes.shard` for text; World-threading externs for effects (list 3) |
| a cited name past the file's `(import Init NAME)` | `above_horizon: X is declaration N of the export, at or above the module's horizon H: write (import Init X)` | slice 3.27 rule 5 | as the message says — the highest ordinal the file cites; a file that imports a V3 file **inherits** its horizon (slice 3.21b rule 2) and may carry no `(import Init …)` of its own |
| `#eval e`, `#check e`, `#print n` | `unknown_form: a top-level form is (KEYWORD …)` | the driver's | `--run M.FN FILE -- ARG…` on a World entry; `--dump` |

## 3. Shard-only

| form | meaning | where |
|---|---|---|
| `(fn NAME BINDERS RET (measure M)? BODY)` | `def` + `realize` in one: an L definition (`DEFINE`, equations `NAME.eq_N`) and the E body that runs (`ev`), classified and erased; binders are **E types** (`Nat`, `Int`, `Bool`, `(List T)`, `Fin`, a `type`, a realized structure); `(measure (struct x))` structural, `(measure TERM)` a `Nat` measure with its descent obligations (`PENDING NAME measure` until proved) | §4, §8.4 |
| `(realize NAME (view))`, `(realize NAME BINDERS RET BODY (equations THM…)?)` | attach an executable body to an Init constant under its own identity: the derived view erases Init's definition (`ite`, `casesOn`, a matcher); the supplied body states equations `NAME.realize_N` proved by `rfl` or by the named theorems | §7; `v3/std/list.shard` |
| `IO.println (toString n)` | `(write_line (Bytes.ofNats (Nat.renderOnto n List.nil)) w)` — `v3/std/host.shard`'s `write_line` over a `ByteArray`, `v3/std/bytes.shard`'s `Bytes.ofNats`, `v3/std/derive.shard`'s `Nat.renderOnto` (the registered renderer; `(derive T render)` generates `T.render` for a `type` of the file) | the program's one output idiom; `(import "../std/host.shard")` etc. with `(use std.host)` |
| `(extern NAME BINDERS RET)`, `World`, `(fn main ((a T)… (w World)) World …)` | the host's effects as World-threading externs (`write_line`, `read_file`, `get_args`, …, `v3/std/host.shard`); the entry's last parameter is the World, the ones before it checked entries decoded from the command line | §9; `v3/examples/path/path_main.shard` |
| `(import "FILE.shard")`, `(import "DIR")`, `(use MODULE)` | a file by path (relative to the importer); a directory module by its `mod.req` view; the module's prefix opened | §3.1 |
| `(import Init NAME)` | the module's **horizon**: the export's prefix through NAME, K-checked on demand; `Init.X` cites an export name explicitly | §3.1, slice 3.26 |
| `(sig fn …)`, `(sig type …)`, `mod.req.shard`, `(requirement NAME BINDERS PROP)`, `(fulfills NAME PROOF)` | a module's interface: bodyless signatures, opaque types, promised laws discharged by the implementation | §6.5 |
| `(trusts AXIOM…)` | widens the file's assumption policy by name | §9 |
| `(type T (C F…)…)`, `(record R (f T)…)`, `(make R (f v)…)`, `(with R r (f v)…)` | data sugar over `inductive`/`structure` | §4 |
| `(derive T eq (ord structural) render)`, `(CAPABILITY by NAME)` | equality, ordering and rendering generated as `fn`s under a declared policy | §8.4 slice 3.19 |
| `(dif h C T F)` | Lean's `if h : c` with the hypothesis named first | §8.4 slice 3.24 |
| `(by STEP…)` as I | the tactic block **is** the certificate: each step a function of the node and the goal, replayed without search; `(wf X MEASURE IH STEP…)` well-founded induction on a `Nat` measure; `(reduce)`/`(reduce lhs)` the normal form by K's conversion and the equations; `(arith FACT…)` the Farkas certificate reconstructed, `(arith only FACT… (farkas K…))` the certificate written; `(have NAME PROP STEP…)` the cut; `(show P)`; `(unfold f)` | §8.4 slices 3.20–3.22 |
| `auto` as a theorem's proof, `FILE.auto.shard` | the engine's block, stored beside the file and replayed; `v3/kernel/prove.shard` regenerates it | §8.4 slice 3.22; `v3/examples/auto/` |
| `load.shard --store DIR`, `verify_release.shard` | the release: every declaration stored in K's export format and re-accepted by K alone | §8.4 slice 3.22 |
| the one-spelling tables (`kernel/spelling.shard`) | a class method under Init's closed instance **is** its function (`HAdd.hAdd … instHAdd` is `Nat.add`); an instance-implicit at `Nat`/`Int`/`Bool`/`List` is the table's | §8.4 slice 3.27 rules 2–3 |
| the registry's rows (`Nat`, `Int` as the integer; `Fin` as `Nat`; `Subtype` as its value; `UInt8`, `ByteArray`, `String` by their one runtime field) | what an L type is at runtime, stated once | §8.4 slices 3.16–3.18, 3.27 |
| the refusal vocabulary | `unknown_constant`, `above_horizon`, `init_name_not_found`, `unsolved_implicit`, `instance_needed`, `unsolved_universe`, `type_mismatch`, `function_expected`, `infix_form`, `inhabited_default`, `lean_form`, `reserved_form`, `unknown_form`, `function_value`, `no_realization`, `nat_operator`, `cases_indexed`, `arith_failed`, `rw_no_match`, `rfl_failed`, `unfold_stuck`, `goal_open`, `goal_closed`, `match_not_exhaustive`, `match_motive` — each with its pointer in the message, the step's index and the goal it faced in a block | `v3/pins/loader/README.md` lists each with its pin |
