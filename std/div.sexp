;;; std/div — decimal division facts: the foundation for measure /
;;; well-founded recursion on integers.
;;;
;;; The reducer's `/` and `mod` are opaque on a symbolic n: nothing
;;; in-kernel relates (/ n 10) to n. So we AXIOMATIZE the Euclidean
;;; semantics the runtime's truncated integer division already obeys,
;;; then DERIVE the decrease lemma  (/ n 10) < n  (for n > 0) and the
;;; sign lemma  0 <= (/ n 10)  (for n >= 0) by linear arithmetic.
;;;
;;; Divisor is the literal 10 throughout (decimal): that keeps
;;; 10*(/ n 10) a `const * var` term, which lia/farkas handle linearly.
;;; A symbolic divisor d would make d*(/ n d) nonlinear (opaque) and
;;; break the linear derivation — general division is out of scope here.
;;;
;;; (/ n 10) and (mod n 10) appear to farkas as OPAQUE ATOMS, recognized
;;; across premises/goal by structural equality — so the identity below,
;;; fed as a premise, lets farkas eliminate them.

;;; ====================================================================
;;; TRUSTED AXIOMS about the `/` and `mod` primitives (audit boundary;
;;; see docs/BOUNDARIES.md). Each is a true property of the runtime's
;;; truncated integer division, not derivable in-kernel.
;;; ====================================================================

;; Euclidean identity — UNCONDITIONAL for truncated division:
;;   n = 10 * (n / 10) + (n mod 10).
(axiom div_mod_10_id
  (Goal
    (list (Param 'n (ty Int)))
    (list)
    (Equation
      (FVar 'n)
      (Call '+ (list
        (Call '* (list (IntLit 10) (Call '/ (list (FVar 'n) (IntLit 10)))))
        (Call 'mod (list (FVar 'n) (IntLit 10))))))))

;; Remainder is non-negative when n is:  0 <= n  |-  0 <= n mod 10.
(axiom mod_10_lo
  (Goal
    (list (Param 'n (ty Int)))
    (list (Equation (Call 'le (list (IntLit 0) (FVar 'n))) (Ctor 'True (list))))
    (Equation
      (Call 'le (list (IntLit 0) (Call 'mod (list (FVar 'n) (IntLit 10)))))
      (Ctor 'True (list)))))

;; Remainder is below the divisor when n is non-negative:
;;   0 <= n  |-  n mod 10 <= 9.
(axiom mod_10_hi
  (Goal
    (list (Param 'n (ty Int)))
    (list (Equation (Call 'le (list (IntLit 0) (FVar 'n))) (Ctor 'True (list))))
    (Equation
      (Call 'le (list (Call 'mod (list (FVar 'n) (IntLit 10))) (IntLit 9)))
      (Ctor 'True (list)))))

;;; ====================================================================
;;; Pure-linear helpers (q, r are ordinary Int vars — farkas, no axioms).
;;; ====================================================================

;; If n = 10q + r with 0 <= r and 0 < n, then q < n.
;; Refutation of (q >= n): 10*(q-n) + 1*(n-10q-r) + 1*(r) + 9*(n-1) = -9 < 0.
(claim div_lt_aux
  (Goal
    (list (Param 'n (ty Int)) (Param 'q (ty Int)) (Param 'r (ty Int)))
    (list
      (Equation (FVar 'n) (Call '+ (list (Call '* (list (IntLit 10) (FVar 'q))) (FVar 'r))))
      (Equation (Call 'le (list (IntLit 0) (FVar 'r))) (Ctor 'True (list)))
      (Equation (Call 'lt (list (IntLit 0) (FVar 'n))) (Ctor 'True (list))))
    (Equation (Call 'lt (list (FVar 'q) (FVar 'n))) (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 10 1 1 9))))

;; If n = 10q + r with 0 <= r <= 9 and 0 <= n, then 0 <= q.
;; Refutation of (q < 0): G=10 on neg-goal, M0=-1 on the (equality)
;; identity, M2=1 on (r<=9), M3=1 on (0<=n) ⟹ constant -1 < 0.
(claim div_nonneg_aux
  (Goal
    (list (Param 'n (ty Int)) (Param 'q (ty Int)) (Param 'r (ty Int)))
    (list
      (Equation (FVar 'n) (Call '+ (list (Call '* (list (IntLit 10) (FVar 'q))) (FVar 'r))))
      (Equation (Call 'le (list (IntLit 0) (FVar 'r))) (Ctor 'True (list)))
      (Equation (Call 'le (list (FVar 'r) (IntLit 9))) (Ctor 'True (list)))
      (Equation (Call 'le (list (IntLit 0) (FVar 'n))) (Ctor 'True (list))))
    (Equation (Call 'le (list (IntLit 0) (FVar 'q))) (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 10 -1 0 1 1))))

;;; ====================================================================
;;; The decrease + sign lemmas: instantiate the helpers at q := (/ n 10),
;;; r := (mod n 10), discharging the algebraic premises from the axioms.
;;; ====================================================================

;; THE DECREASE LEMMA:  0 < n  |-  (/ n 10) < n.
(claim div_lt
  (Goal
    (list (Param 'n (ty Int)))
    (list (Equation (Call 'lt (list (IntLit 0) (FVar 'n))) (Ctor 'True (list))))
    (Equation (Call 'lt (list (Call '/ (list (FVar 'n) (IntLit 10))) (FVar 'n))) (Ctor 'True (list))))
  (RewriteWith (Lemma 'div_lt_aux) Lr Lhs
    (list (Inst 'n (FVar 'n))
          (Inst 'q (Call '/ (list (FVar 'n) (IntLit 10))))
          (Inst 'r (Call 'mod (list (FVar 'n) (IntLit 10)))))
    (list
      ;; P0: n = 10*(/ n 10) + (mod n 10)  — the identity axiom (fold RHS to n).
      (Steps (list (Rewrite (Lemma 'div_mod_10_id) Rl Rhs True (list))) Refl)
      ;; P1: 0 <= (mod n 10)  — mod_10_lo, its premise 0<=n from 0<n.
      (RewriteWith (Lemma 'mod_10_lo) Lr Lhs (list)
        (list (ByTheory 'farkas (Cert 'farkas (list 1 1))))
        Refl)
      ;; P2: 0 < n  — the goal premise.
      (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
    Refl))

;; THE SIGN LEMMA:  0 <= n  |-  0 <= (/ n 10).
(claim div_nonneg
  (Goal
    (list (Param 'n (ty Int)))
    (list (Equation (Call 'le (list (IntLit 0) (FVar 'n))) (Ctor 'True (list))))
    (Equation (Call 'le (list (IntLit 0) (Call '/ (list (FVar 'n) (IntLit 10))))) (Ctor 'True (list))))
  ;; Pin n explicitly: the conclusion (le 0 q) does not mention the
  ;; lemma's n, so without this Inst it stays an unresolved pattern var
  ;; and the n-premises (identity, 0<=n) couldn't be discharged.
  (RewriteWith (Lemma 'div_nonneg_aux) Lr Lhs
    (list (Inst 'n (FVar 'n))
          (Inst 'q (Call '/ (list (FVar 'n) (IntLit 10))))
          (Inst 'r (Call 'mod (list (FVar 'n) (IntLit 10)))))
    (list
      ;; P0: n = 10*(/ n 10) + (mod n 10)
      (Steps (list (Rewrite (Lemma 'div_mod_10_id) Rl Rhs True (list))) Refl)
      ;; P1: 0 <= (mod n 10)  — mod_10_lo, its premise 0<=n is our premise.
      (RewriteWith (Lemma 'mod_10_lo) Lr Lhs (list)
        (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
        Refl)
      ;; P2: (mod n 10) <= 9  — mod_10_hi, its premise 0<=n is our premise.
      (RewriteWith (Lemma 'mod_10_hi) Lr Lhs (list)
        (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
        Refl)
      ;; P3: 0 <= n  — the goal premise.
      (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
    Refl))
