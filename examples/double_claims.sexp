;;; Claims about the `double` fn declared in double_lib.sexp.
;;;
;;; The (use-module …) form below loads double_lib.sexp as a user
;;; module; subsequent (claim …) forms get that module passed as the
;;; `m` arg to check_sequent, so a Simp step can unfold `double`.

(use-module "double_lib.sexp")

;; (double 5) = 10. Simp Lhs unfolds double then fires +; Refl
;; closes 10 = 10. Mirrors the Rust test
;; check_seq_proves_double_5_equals_10.
(claim double_5_is_10
  (Goal (list) (list)
    (Equation
      (Call 'double (list (IntLit 5)))
      (IntLit 10)))
  (Steps (list (Simp (Lhs))) (Refl)))

;; (double 4) = (+ 3 5). Simp Both reduces lhs (double 4 → 8) and
;; rhs (3 + 5 → 8); Refl closes 8 = 8.
(claim double_4_meets_3_plus_5
  (Goal (list) (list)
    (Equation
      (Call 'double (list (IntLit 4)))
      (Call '+ (list (IntLit 3) (IntLit 5)))))
  (Steps (list (Simp (Both))) (Refl)))
