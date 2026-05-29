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

;; is_digit fires True on a digit codepoint, given the precondition.
;; Unfold exposes (if (le 48 (48+x)) (le (48+x) 57) False); rewrite the
;; lower guard True (discharging its premise from Hyp 0), Reduce the if,
;; rewrite the upper guard True (from Hyp 1). This is the "collapse a
;; data-dependent guard via theory" move the run proof leans on.
(claim is_digit_of_digit
  (Goal
    (list (Param 'x (ty Int)))
    (list (Equation (Call 'le (list (IntLit 0) (FVar 'x))) (Ctor 'True (list)))
          (Equation (Call 'le (list (FVar 'x) (IntLit 9))) (Ctor 'True (list))))
    (Equation
      (Call 'is_digit (list (Call '+ (list (IntLit 48) (FVar 'x)))))
      (Ctor 'True (list))))
  (Steps
    (list (Unfold 'is_digit Lhs))
    (RewriteWith (Lemma 'digit_ge_lo) Lr Lhs
      (list)
      (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
      (Steps
        (list (Reduce Lhs))
        (RewriteWith (Lemma 'digit_le_hi) Lr Lhs
          (list)
          (list (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl))
          Refl)))))

;;; --------------------------------------------------------------------
;;; The headline theorem (PENDING — blocked on a kernel reducer gap):
;;;
;;;   ∀ x y.  0<=x<=9, 0<=y<=9  ⊢
;;;     run (list (+ 48 x) 43 (+ 48 y))  =  Some (+ x y)
;;;
;;; The lemmas above prove the interesting technique — collapsing a
;;; data-dependent guard (is_digit on a symbolic codepoint) via theory
;;; entailment. But driving `run` end-to-end requires REDUCING through
;;; the lexer, and both Simp and Reduce CRASH (EvalError::NoMatchArm)
;;; when reduction reaches a primitive applied to a symbolic argument
;;; like (+ 48 x) inside lex_go. try_step_prim itself is total (returns
;;; None when args don't fit — reduce.sexp:144), so this is a partiality
;;; elsewhere in the reducer's stuck-term path: a trusted-core
;;; robustness gap, not a usage error. is_digit_of_digit only succeeds
;;; because it rewrites the guard BEFORE reduction touches it.
;;;
;;; Unblocking it (make the reducer leave stuck terms stuck instead of
;;; crashing) is a kernel fix tracked separately; the theorem lands once
;;; that's in.
;;; --------------------------------------------------------------------
