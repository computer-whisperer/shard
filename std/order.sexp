;;; std/order — Int order & disequality entailment lemmas, decided by the
;;; ord / farkas backends. The reusable arithmetic vocabulary for proofs
;;; about indices, bounds, and comparisons. Imports nothing.

;; ===== order tautologies (ord backend) =====
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

;; ===== conditional entailment (farkas backend) =====
(claim lt_succ_from_lt
  (Goal
    (list (Param 'p (ty Int)) (Param 'i (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'p) (FVar 'i))) (Ctor 'True (list))))
    (Equation
      (Call 'lt (list (FVar 'p) (Call '+ (list (FVar 'i) (IntLit 1)))))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; ∀ a b. (lt a b) = True ⊢ (le a b) = True.   (strict implies non-strict)
;; ¬goal: a-b-1 >= 0;  premise: b-a-1 >= 0;  sum = -2 < 0.
(claim lt_implies_le
  (Goal
    (list (Param 'a (ty Int)) (Param 'b (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'a) (FVar 'b))) (Ctor 'True (list))))
    (Equation
      (Call 'le (list (FVar 'a) (FVar 'b)))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; ∀ a b c. (le a b) = True, (le b c) = True ⊢ (le a c) = True.  (transitivity)
;; ¬goal: a-c-1 >= 0;  prem0: b-a >= 0;  prem1: c-b >= 0;  sum = -1 < 0.
(claim le_trans
  (Goal
    (list (Param 'a (ty Int)) (Param 'b (ty Int)) (Param 'c (ty Int)))
    (list (Equation (Call 'le (list (FVar 'a) (FVar 'b))) (Ctor 'True (list)))
          (Equation (Call 'le (list (FVar 'b) (FVar 'c))) (Ctor 'True (list))))
    (Equation
      (Call 'le (list (FVar 'a) (FVar 'c)))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1 1))))

;; ∀ a b. (int_eq b a) = True ⊢ (le a b) = True.  (equality premise → order)
;; The equality b=a enters as the (any-sign) constraint b-a = 0;
;; ¬goal: a-b-1 >= 0;  1·¬goal + 1·(b-a) = -1 < 0.
(claim le_from_eq
  (Goal
    (list (Param 'a (ty Int)) (Param 'b (ty Int)))
    (list (Equation (Call 'int_eq (list (FVar 'b) (FVar 'a))) (Ctor 'True (list))))
    (Equation
      (Call 'le (list (FVar 'a) (FVar 'b)))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; ∀ a b. (lt a b) = True ⊢ (int_eq a b) = False.  (DISEQUALITY conclusion)
;; The M3 enabler: a strict bound yields a ≠ b. The goal negates to the
;; equality a = b (a-b = 0, any-sign multiplier); 1·(a-b)[¬goal] +
;; 1·(b-a-1)[premise] = -1 < 0.
(claim lt_implies_neq
  (Goal
    (list (Param 'a (ty Int)) (Param 'b (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'a) (FVar 'b))) (Ctor 'True (list))))
    (Equation
      (Call 'int_eq (list (FVar 'a) (FVar 'b)))
      (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; ∀ a b. (le a b)=True, (le b a)=True ⊢ (int_eq a b)=True.  (antisymmetry)
;; EQUALITY conclusion — two-sided. payload = (list le_mults ge_mults):
;;   a<=b: ¬(a<=b)=a-b-1; 1·¬ + 1·(le a b)[b-a] = -1 < 0  → le_mults (1 1 0)
;;   b<=a: ¬(b<=a)=b-a-1; 1·¬ + 1·(le b a)[a-b] = -1 < 0  → ge_mults (1 0 1)
(claim eq_from_le_both
  (Goal
    (list (Param 'a (ty Int)) (Param 'b (ty Int)))
    (list (Equation (Call 'le (list (FVar 'a) (FVar 'b))) (Ctor 'True (list)))
          (Equation (Call 'le (list (FVar 'b) (FVar 'a))) (Ctor 'True (list))))
    (Equation
      (Call 'int_eq (list (FVar 'a) (FVar 'b)))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list (list 1 1 0) (list 1 0 1)))))
