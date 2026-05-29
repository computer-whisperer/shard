;;; std/nat — the Nat type and basic Nat arithmetic.
;;; Foundational: imports nothing.

;;; addition recursing on the first argument.
;;;
;;; Loaded as a user module by add_nat_zero.sexp.

(import "order.sexp")    ; for le0_succ (used by int_of_nat_nonneg)

(type Nat
  (Z)
  (S Nat))

(fn add_nat ((a Nat) (b Nat)) Nat
  (match a
    (Z       b)
    ((S k)   (S (add_nat k b)))))

;; ---- int_of_nat / half_nat (hoisted from std/mem, slice 53) ----

;; Nat → Int (structural on the Nat).
(fn int_of_nat ((n Nat)) Int
  (match n
    (Z 0)
    ((S k) (+ 1 (int_of_nat k)))))

;; floor(n/2) as a Nat (structural, two S's at a time).
(fn half_nat ((n Nat)) Nat
  (match n
    (Z Z)
    ((S k)
      (match k
        (Z Z)
        ((S k2) (S (half_nat k2)))))))

;; ---- their lemmas ----

;; int_of_nat is nonnegative. Induction on n; S case steps le0_succ on
;; the IH (which farkas can't see directly — it lives in the hyps, so we
;; thread it through le0_succ's premise).
(claim int_of_nat_nonneg
  (Goal (list (Param 'n (ty Nat))) (list)
    (Equation (Call 'le (list (IntLit 0) (Call 'int_of_nat (list (FVar 'n)))))
              (Ctor 'True (list))))
  (Induct 'n
    (list
      (Case 'Z (Steps (list (Simp Lhs)) Refl))
      (Case 'S
        (Steps (list (Simp Lhs))   ; (le 0 (+ 1 (int_of_nat k)))
          (RewriteWith (Lemma 'le0_succ) Lr Lhs (list)
            (list (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
            Refl))))))

;; --- the loop runs far enough: n-1 <= 2*floor(n/2) -----------------------

;; half_step: the two-step inductive step for half_bound, with the IH's
;; inequality supplied as a premise (farkas can't read the Induct2 hyp
;; directly). X = int_of_nat k, Y = int_of_nat (half_nat k); the goal is
;; the Simp-normalized half_bound at S (S k).
(claim half_step
  (Goal
    (list (Param 'X (ty Int)) (Param 'Y (ty Int)))
    (list
      (Equation (Call 'le (list (Call '- (list (FVar 'X) (IntLit 1)))
                                (Call '* (list (IntLit 2) (FVar 'Y)))))
                (Ctor 'True (list))))
    (Equation
      (Call 'le (list (Call '- (list (Call '+ (list (IntLit 1)
                                                    (Call '+ (list (IntLit 1) (FVar 'X)))))
                                     (IntLit 1)))
                      (Call '* (list (IntLit 2) (Call '+ (list (IntLit 1) (FVar 'Y)))))))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; half_bound: int_of_nat m - 1 <= 2 * int_of_nat (half_nat m), for ALL m.
;; This is "the loop's floor(m/2) swaps cover the segment of length m"
;; (tight at both parities). half_nat recurses two-at-a-time, so this is
;; the first user of the kernel's two-step induction (Induct2): the Z and
;; (S Z) arms close by Simp; the (S (S k)) arm Simp-normalizes and hands
;; the IH (at k) to half_step.
(claim half_bound
  (Goal
    (list (Param 'm (ty Nat)))
    (list)
    (Equation
      (Call 'le (list (Call '- (list (Call 'int_of_nat (list (FVar 'm))) (IntLit 1)))
                      (Call '* (list (IntLit 2)
                                     (Call 'int_of_nat (list (Call 'half_nat (list (FVar 'm)))))))))
      (Ctor 'True (list))))
  (Induct2 'm
    (list
      (Case 'Z  (Steps (list (Simp Lhs)) Refl))
      (Case 'SZ (Steps (list (Simp Lhs)) Refl))
      (Case 'SS
        (Steps (list (Simp Lhs))
          (RewriteWith (Lemma 'half_step) Lr Lhs (list)
            (list (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
            Refl))))))
