;;; Linear-entailment examples — the `farkas` ByTheory backend (slice 37).
;;;
;;; Unlike ord (tautologies), farkas decides `premises ⊢ conclusion`:
;;; the goal's conditional premises imply a linear order fact. The Cert
;;; payload carries the Farkas multipliers (list G M0 M1 ...): G scales
;;; the negated goal, Mk scales premise k. The kernel CHECKS that the
;;; combination cancels to a negative constant (a manifest 0 >= 1);
;;; finding the multipliers is this comment's job, not the kernel's.
;;;
;;; lt_succ_from_lt is THE obligation the M3 loop invariant generates
;;; (feeding the IH at the shrunk segment i+1). The others are the
;;; staple order-entailment shapes the invariant will lean on.

;; ∀ p i. (lt p i) = True ⊢ (lt p (+ i 1)) = True.
;; ¬goal: p-i-1 >= 0;  premise: i-p-1 >= 0;  1·¬goal + 1·prem = -2 < 0.
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
