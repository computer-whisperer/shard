;;; Stage-0 calculator demo — the spec proof.
;;;
;;; The requirement: for single decimal digits x, y, the calculator
;;; evaluates the text "x+y" (and "x-y") to x+y (resp. x-y), going
;;; through the REAL lexer — no shortcut around parsing.
;;;
;;; "Single digit" = the precondition 0 <= x <= 9, with x's codepoint
;;; written (+ 48 x) (since '0' = 48). This keeps the input symbolic yet
;;; lexable without a show : Int -> String formatter (which would need
;;; well-founded recursion — see the file header discussion / roadmap).
;;;
;;; The interesting part: Simp alone can't discharge is_digit's range
;;; guards on a symbolic codepoint, so the proof interleaves reduction
;;; with farkas/lia entailment. These foundation lemmas isolate that.

(import "calc.sexp")

;; digit_val inverts the codepoint offset:  digit_val (48+x) = x.
;; Simp unfolds digit_val to (- (+ 48 x) 48); lia closes the linear eq.
(claim digit_val_of_digit
  (Goal
    (list (Param 'x (ty Int)))
    (list)
    (Equation
      (Call 'digit_val (list (Call '+ (list (IntLit 48) (FVar 'x)))))
      (FVar 'x)))
  (Steps (list (Unfold 'digit_val Lhs)) (ByTheory 'lia (Cert 'lia (list)))))

;; Lower guard:  0 <= x  ⊢  (le 48 (48+x)) = True.
;; ¬goal: 48-(48+x)-1 = -x-1 >= 0;  premise: x >= 0;  sum = -1 < 0.
(claim digit_ge_lo
  (Goal
    (list (Param 'x (ty Int)))
    (list (Equation (Call 'le (list (IntLit 0) (FVar 'x))) (Ctor 'True (list))))
    (Equation
      (Call 'le (list (IntLit 48) (Call '+ (list (IntLit 48) (FVar 'x)))))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; Upper guard:  x <= 9  ⊢  (le (48+x) 57) = True.
;; ¬goal: (48+x)-57-1 = x-10 >= 0;  premise: 9-x >= 0;  sum = -1 < 0.
(claim digit_le_hi
  (Goal
    (list (Param 'x (ty Int)))
    (list (Equation (Call 'le (list (FVar 'x) (IntLit 9))) (Ctor 'True (list))))
    (Equation
      (Call 'le (list (Call '+ (list (IntLit 48) (FVar 'x))) (IntLit 57)))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))
