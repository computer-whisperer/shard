;;; The arbitrary-integer round trip: show : Int -> digits, and
;;;   0 <= n  |-  valI (show n) = n
;;; proved by WfInduct (measure n) — the first real use of measure
;;; recursion. `show` recurses on n/10, which Induct cannot handle.
;;;
;;; Representation: digits are Int VALUES 0..9 here (not the Digit type),
;;; so the round trip needs no Int->Digit conversion (that conversion,
;;; dval(digit_of k)=k for symbolic k, would require enumerating k=0..9 —
;;; awkward without literal-pattern case analysis). Bridging this to the
;;; Digit-based run_ndigit_adds (calc_ndigit.sexp) is a separate step.

(import "../../std/div.sexp")   ; div_lt, div_nonneg, div_mod_10_id (+ axioms)
(import "../../std/list.sexp")  ; append

;; MSD-first Horner value of a digit-value list, with an accumulator.
;; (* 10 acc) — the literal-10-first order matches div_mod_10_id, so the
;; inductive step closes against it with no commutativity shuffle.
(fn valI_go ((ds (List Int)) (acc Int)) Int
  (match ds
    (Nil acc)
    ((Cons d rest) (valI_go rest (+ (* 10 acc) d)))))

(fn valI ((ds (List Int))) Int (valI_go ds 0))

;; show n: decimal digit-VALUE list, most-significant first.
;;   n < 10 : [n]                          (single digit)
;;   n >= 10: show (n/10) ++ [n mod 10]     (recurse on the quotient)
(fn show ((n Int)) (List Int)
  (if (lt n 10)
      (Cons n Nil)
      (append (show (/ n 10)) (Cons (mod n 10) Nil))))

;; n >= 10 implies n > 0 — bridges the loop guard (a CaseOn hyp) to the
;; (lt 0 n) that div_lt needs as a premise. farkas: (n-10) + (-n) = -10 < 0.
(claim lt10f_pos
  (Goal
    (list (Param 'n (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'n) (IntLit 10))) (Ctor 'False (list))))
    (Equation (Call 'lt (list (IntLit 0) (FVar 'n))) (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; Appending a digit shifts the accumulator value by 10 and adds it.
;; Accumulator induction on ds (acc generalized), citing the IH at the
;; stepped accumulator — same shape as std/list fast_acc_lemma.
(claim valI_go_snoc
  (Goal
    (list (Param 'ds (ty List Int)) (Param 'd (ty Int)) (Param 'acc (ty Int)))
    (list)
    (Equation
      (Call 'valI_go (list
        (Call 'append (list (FVar 'ds) (Ctor 'Cons (list (FVar 'd) (Ctor 'Nil (list))))))
        (FVar 'acc)))
      (Call '+ (list
        (Call '* (list (IntLit 10) (Call 'valI_go (list (FVar 'ds) (FVar 'acc)))))
        (FVar 'd)))))
  (Induct 'ds
    (list
      (Case 'Nil (Steps (list (Simp Both)) Refl))
      (Case 'Cons
        (Steps (list (Simp Lhs) (Rewrite (Hyp 0) Lr Lhs True (list)) (Simp Rhs)) Refl)))))

;; valI form of the snoc law (acc := 0).
(claim valI_snoc
  (Goal
    (list (Param 'ds (ty List Int)) (Param 'd (ty Int)))
    (list)
    (Equation
      (Call 'valI (list
        (Call 'append (list (FVar 'ds) (Ctor 'Cons (list (FVar 'd) (Ctor 'Nil (list))))))))
      (Call '+ (list
        (Call '* (list (IntLit 10) (Call 'valI (list (FVar 'ds)))))
        (FVar 'd)))))
  (Steps
    (list (Unfold 'valI Lhs) (Unfold 'valI Rhs)
          (Rewrite (Lemma 'valI_go_snoc) Lr Lhs True (list)))
    Refl))

;; THE ROUND TRIP:  0 <= n  |-  valI (show n) = n.
;; WfInduct on measure n. CaseOn the loop guard (lt n 10):
;;   base  (n<10): show n = [n], valI [n] = n.
;;   step  (n>=10): show n = show(n/10) ++ [n mod 10];
;;     valI = 10*valI(show(n/10)) + (n mod 10)  [valI_snoc]
;;          = 10*(n/10) + (n mod 10)            [IH at n/10]
;;          = n                                  [div_mod_10_id].
;; The IH (Hyp 1) discharges 0<=n/10 (div_nonneg) twice and n/10<n
;; (div_lt, whose 0<n comes from the n>=10 hyp via lt10f_pos).
(claim show_correct
  (Goal
    (list (Param 'n (ty Int)))
    (list (Equation (Call 'le (list (IntLit 0) (FVar 'n))) (Ctor 'True (list))))
    (Equation (Call 'valI (list (Call 'show (list (FVar 'n))))) (FVar 'n)))
  (WfInduct (FVar 'n)
    (CaseOn (Call 'lt (list (FVar 'n) (IntLit 10))) 'Bool
      (list
        (Case 'True
          (Steps
            (list (Unfold 'show Lhs)
                  (Rewrite (Hyp 0) Lr Lhs True (list))   ; lt n 10 -> True
                  (Simp Lhs)                             ; if True .. -> (Cons n Nil)
                  (Unfold 'valI Lhs)
                  (Simp Lhs))                            ; valI [n] -> (+ 0 n)
            (ByTheory 'lia (Cert 'lia (list)))))         ; (+ 0 n) = n
        (Case 'False
          (Steps
            (list (Unfold 'show Lhs)
                  (Rewrite (Hyp 0) Lr Lhs True (list))   ; lt n 10 -> False
                  (Simp Lhs)                             ; if False .. -> append (show (n/10)) [n mod 10]
                  (Rewrite (Lemma 'valI_snoc) Lr Lhs True (list)))
            (RewriteWith (Hyp 1) Lr Lhs (list)           ; IH: valI(show(n/10)) -> n/10
              (list
                ;; 0 <= n/10  (goal premise, at n')
                (RewriteWith (Lemma 'div_nonneg) Lr Lhs (list)
                  (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)) Refl)
                ;; 0 <= n/10  (measure >= 0)
                (RewriteWith (Lemma 'div_nonneg) Lr Lhs (list)
                  (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)) Refl)
                ;; n/10 < n   (measure decrease); 0<n from the n>=10 hyp
                (RewriteWith (Lemma 'div_lt) Lr Lhs (list)
                  (list (RewriteWith (Lemma 'lt10f_pos) Lr Lhs (list)
                          (list (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
                          Refl))
                  Refl))
              ;; 10*(n/10) + (n mod 10) = n
              (Steps (list (Rewrite (Lemma 'div_mod_10_id) Rl Lhs True
                             (list (Inst 'n (FVar 'n))))) Refl))))))))
