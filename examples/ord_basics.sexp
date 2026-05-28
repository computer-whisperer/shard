;;; Order-reflection examples — the `ord` ByTheory backend (slice 35).
;;;
;;; ord decides `(lt a b) = True` / `(le a b) = True` when the
;;; difference (b - a) canonicalizes to a constant of the right sign
;;; (>= 1 for strict, >= 0 for non-strict). These are the order
;;; TAUTOLOGIES — facts true for every integer assignment. Conditional
;;; order facts (under a hypothesis i < j) are not proven here; they're
;;; consumed as premises, like eqdec's disequalities.
;;;
;;; These are the arithmetic the M3 loop invariant needs (i < i+1,
;;; bounds that differ by a constant); the loop-invariant proof that
;;; consumes them is mem_lemmas.sexp's next slice.

;; ∀ a : Int. (lt a (+ a 1)) = True.    (a < a+1 always; diff = 1)
(claim lt_succ
  (Goal
    (list (Param 'a (ty Int)))
    (list)
    (Equation
      (Call 'lt (list (FVar 'a) (Call '+ (list (FVar 'a) (IntLit 1)))))
      (Ctor 'True (list))))
  (ByTheory 'ord (Cert 'ord (list))))

;; ∀ a : Int. (le a a) = True.          (reflexivity of ≤; diff = 0)
(claim le_refl
  (Goal
    (list (Param 'a (ty Int)))
    (list)
    (Equation
      (Call 'le (list (FVar 'a) (FVar 'a)))
      (Ctor 'True (list))))
  (ByTheory 'ord (Cert 'ord (list))))

;; ∀ a : Int. (le a (+ a 1)) = True.    (non-strict successor; diff = 1 ≥ 0)
(claim le_succ
  (Goal
    (list (Param 'a (ty Int)))
    (list)
    (Equation
      (Call 'le (list (FVar 'a) (Call '+ (list (FVar 'a) (IntLit 1)))))
      (Ctor 'True (list))))
  (ByTheory 'ord (Cert 'ord (list))))

;; ∀ a b : Int. (le a (+ b (+ a (- 1 b)))) = True.
;; A less-trivial case: the RHS is (b + a + (1 - b)) = a + 1, so
;; diff = (a+1) - a = 1 ≥ 0 — the b's cancel under canonicalization.
;; Demonstrates ord seeing through linear rearrangement (it reuses LIA).
(claim le_linear_cancel
  (Goal
    (list (Param 'a (ty Int)) (Param 'b (ty Int)))
    (list)
    (Equation
      (Call 'le
        (list (FVar 'a)
              (Call '+ (list (FVar 'b)
                             (Call '+ (list (FVar 'a)
                                            (Call '- (list (IntLit 1) (FVar 'b)))))))))
      (Ctor 'True (list))))
  (ByTheory 'ord (Cert 'ord (list))))
