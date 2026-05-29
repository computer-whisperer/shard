;;; Stage-0 calculator demo — the lexer.
;;;
;;; Grammar (flat, left-to-right):  INT (('+' | '-') INT)*
;;;
;;; This file turns a String — i.e. a (List Int) of codepoints (slice 55)
;;; — into a (List Token). The parser, evaluator, reference spec, and the
;;; spec-equivalence proof land in later slices; this is the first link of
;;; the pipeline (text → tokens).
;;;
;;; Depends on the kernel stdlib (List / Option / Bool), which the `check`
;;; binary auto-loads. No claims yet — those arrive with the spec.

(type Token
  (TNum Int)     ; a literal number, already folded from its digits
  (TPlus)        ; '+'
  (TMinus))      ; '-'

;; '0'..'9' are codepoints 48..57; `is_digit` is "48 <= c AND c <= 57".
(fn is_digit ((c Int)) Bool
  (if (le 48 c) (le c 57) False))

(fn digit_val ((c Int)) Int
  (- c 48))

;; Fold one digit codepoint into the number being built (None = start fresh).
(fn acc_digit ((acc (Option Int)) (c Int)) Int
  (match acc
    (None      (digit_val c))
    ((Some n)  (+ (* n 10) (digit_val c)))))

;; Emit any pending accumulated number as a leading TNum, else pass through.
(fn flush ((acc (Option Int)) (toks (List Token))) (List Token)
  (match acc
    (None      toks)
    ((Some n)  (Cons (TNum n) toks))))

;; A single structural pass over the input, threading `acc` = the number
;; currently being built (None when not inside a number). Digits extend
;; acc; '+' (43) / '-' (45) flush acc then emit the operator; any other
;; char (e.g. space, 32) flushes and is skipped. Recursing only on the
;; tail `rest` keeps this structurally recursive — hence total, and
;; amenable to `Induct` for the universal proofs to come.
(fn lex_go ((cs (List Int)) (acc (Option Int))) (List Token)
  (match cs
    (Nil  (flush acc Nil))
    ((Cons c rest)
      (if (is_digit c)
          (lex_go rest (Some (acc_digit acc c)))
          (flush acc
            (if (int_eq c 43)
                (Cons TPlus  (lex_go rest None))
                (if (int_eq c 45)
                    (Cons TMinus (lex_go rest None))
                    (lex_go rest None))))))))

(fn lex ((cs (List Int))) (List Token)
  (lex_go cs None))

;;; --------------------------------------------------------------------
;;; The AST, evaluator, and parser.
;;;
;;; Grammar:  INT (('+' | '-') INT)*  — left-associative, no precedence.
;;; (Named `Exp`, not `Expr`, to avoid colliding with the kernel's own
;;; Expr term type when both are loaded into one module.)
;;; --------------------------------------------------------------------

(type Exp
  (Num Int)
  (Add Exp Exp)
  (Sub Exp Exp))

;; Tree-walking evaluator — the meaning of an Exp. Structural recursion
;; on the subexpressions, so total.
(fn eval ((e Exp)) Int
  (match e
    ((Num n)   n)
    ((Add a b) (+ (eval a) (eval b)))
    ((Sub a b) (- (eval a) (eval b)))))

;; Fold a run of `(op number)` pairs into a left-leaning tree, given the
;; expression parsed so far (`acc`). Returns None on a malformed tail.
;; Each recursive call drops two Cons cells, so `rest` is a structural
;; subterm of `toks` — structural recursion, hence total.
(fn parse_rest ((toks (List Token)) (acc Exp)) (Option Exp)
  (match toks
    (Nil (Some acc))
    ((Cons TPlus  (Cons (TNum n) rest)) (parse_rest rest (Add acc (Num n))))
    ((Cons TMinus (Cons (TNum n) rest)) (parse_rest rest (Sub acc (Num n))))
    ((Cons _ _) None)))

;; A well-formed expression starts with a number, then a (possibly empty)
;; run of operator/number pairs.
(fn parse ((toks (List Token))) (Option Exp)
  (match toks
    ((Cons (TNum n) rest) (parse_rest rest (Num n)))
    ((Cons _ _) None)
    (Nil        None)))

;; The whole pipeline: text → tokens → AST → value. None on any failure
;; to parse (the lexer never fails; the parser is the only gate).
(fn run ((cs (List Int))) (Option Int)
  (match (parse (lex cs))
    (None     None)
    ((Some e) (Some (eval e)))))
