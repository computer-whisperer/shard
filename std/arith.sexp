;;; std/arith — pure linear-integer index identities (lia backend).
;;; Generic Int tautologies used to reconcile term shapes that the
;;; reducer doesn't canonicalize. Imports nothing.

;; interior index normalization (pure tautology, no premises):
;;   (i+1)+(j-1)-p = i+j-p.   The IH returns the left shape; the outer
;;   read wants the right. lia canonicalizes both to i+j-p.
(claim idx_inner_simp
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list)
    (Equation
      (Call '- (list (Call '+ (list (Call '+ (list (FVar 'i) (IntLit 1)))
                                    (Call '- (list (FVar 'j) (IntLit 1)))))
                     (FVar 'p)))
      (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'p)))))
  (ByTheory 'lia (Cert 'lia (list))))

;; --- the front/back flip: rev (dump …) = rdump … ---------------------------

;; lia index tautologies used to reconcile the int_of_nat index shapes
;; that dump/rdump recursion produces.
(claim sub_zero
  (Goal (list (Param 'x (ty Int))) (list)
    (Equation (Call '- (list (FVar 'x) (IntLit 0))) (FVar 'x)))
  (ByTheory 'lia (Cert 'lia (list))))

(claim sub_sub_one
  (Goal (list (Param 'x (ty Int)) (Param 'y (ty Int))) (list)
    (Equation (Call '- (list (Call '- (list (FVar 'x) (IntLit 1))) (FVar 'y)))
              (Call '- (list (FVar 'x) (Call '+ (list (IntLit 1) (FVar 'y)))))))
  (ByTheory 'lia (Cert 'lia (list))))

;; two more lia index tautologies for the flip's recursion-shape glue.
(claim reassoc_succ
  (Goal (list (Param 'b (ty Int)) (Param 'y (ty Int))) (list)
    (Equation
      (Call '- (list (Call '+ (list (FVar 'b) (Call '+ (list (IntLit 1) (FVar 'y))))) (IntLit 1)))
      (Call '- (list (Call '+ (list (Call '+ (list (FVar 'b) (IntLit 1))) (FVar 'y))) (IntLit 1)))))
  (ByTheory 'lia (Cert 'lia (list))))

(claim idx_cancel
  (Goal (list (Param 'b (ty Int)) (Param 'y (ty Int))) (list)
    (Equation
      (Call '- (list (Call '- (list (Call '+ (list (Call '+ (list (FVar 'b) (IntLit 1))) (FVar 'y)))
                                    (IntLit 1)))
                     (FVar 'y)))
      (FVar 'b)))
  (ByTheory 'lia (Cert 'lia (list))))

;; lia: (0+(x-1))-0 = (0+x)-1  (reconcile dump_R_rdump's top with flip's).
(claim cap_idx
  (Goal (list (Param 'x (ty Int))) (list)
    (Equation
      (Call '- (list (Call '+ (list (IntLit 0) (Call '- (list (FVar 'x) (IntLit 1))))) (IntLit 0)))
      (Call '- (list (Call '+ (list (IntLit 0) (FVar 'x))) (IntLit 1)))))
  (ByTheory 'lia (Cert 'lia (list))))

;; lia: s-(base+1) = (s-base)-1  (reconcile the rdump recursion index).
(claim idx_pred
  (Goal (list (Param 's (ty Int)) (Param 'base (ty Int))) (list)
    (Equation
      (Call '- (list (FVar 's) (Call '+ (list (FVar 'base) (IntLit 1)))))
      (Call '- (list (Call '- (list (FVar 's) (FVar 'base))) (IntLit 1)))))
  (ByTheory 'lia (Cert 'lia (list))))
