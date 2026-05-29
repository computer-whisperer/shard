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
