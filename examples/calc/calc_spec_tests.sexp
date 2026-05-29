;;; Behavioral validation of the spec by GROUND EVALUATION (the Compute
;;; tactic). These are the "run the spec on examples" checks that let a
;;; human sign off that the requirement IS the intent. Each is one Compute:
;;; the input is fully concrete, so Compute drives spec_run to a value.
(import "calc_spec.sexp")

;; "1+2" = [49 43 50] -> Some 3
(claim t_1plus2
  (Goal (list) (list)
    (Equation (Call 'spec_run (list (Ctor 'Cons (list (IntLit 49) (Ctor 'Cons (list (IntLit 43) (Ctor 'Cons (list (IntLit 50) (Ctor 'Nil (list))))))))))
              (Ctor 'Some (list (IntLit 3)))))
  (Steps (list (Compute Lhs)) Refl))

;; "1 + 2" = [49 32 43 32 50] -> Some 3   (whitespace allowed)
(claim t_ws
  (Goal (list) (list)
    (Equation (Call 'spec_run (list (Ctor 'Cons (list (IntLit 49) (Ctor 'Cons (list (IntLit 32) (Ctor 'Cons (list (IntLit 43) (Ctor 'Cons (list (IntLit 32) (Ctor 'Cons (list (IntLit 50) (Ctor 'Nil (list))))))))))))))
              (Ctor 'Some (list (IntLit 3)))))
  (Steps (list (Compute Lhs)) Refl))

;; "12-3" = [49 50 45 51] -> Some 9
(claim t_sub
  (Goal (list) (list)
    (Equation (Call 'spec_run (list (Ctor 'Cons (list (IntLit 49) (Ctor 'Cons (list (IntLit 50) (Ctor 'Cons (list (IntLit 45) (Ctor 'Cons (list (IntLit 51) (Ctor 'Nil (list))))))))))))
              (Ctor 'Some (list (IntLit 9)))))
  (Steps (list (Compute Lhs)) Refl))

;; "1+2-3" = [49 43 50 45 51] -> Some 0   (left-assoc chain)
(claim t_chain
  (Goal (list) (list)
    (Equation (Call 'spec_run (list (Ctor 'Cons (list (IntLit 49) (Ctor 'Cons (list (IntLit 43) (Ctor 'Cons (list (IntLit 50) (Ctor 'Cons (list (IntLit 45) (Ctor 'Cons (list (IntLit 51) (Ctor 'Nil (list))))))))))))))
              (Ctor 'Some (list (IntLit 0)))))
  (Steps (list (Compute Lhs)) Refl))

;; "1@+2" = [49 64 43 50] -> None   (garbage byte off-grammar; the live lexer ACCEPTS this)
(claim t_garbage
  (Goal (list) (list)
    (Equation (Call 'spec_run (list (Ctor 'Cons (list (IntLit 49) (Ctor 'Cons (list (IntLit 64) (Ctor 'Cons (list (IntLit 43) (Ctor 'Cons (list (IntLit 50) (Ctor 'Nil (list))))))))))))
              (Ctor 'None (list))))
  (Steps (list (Compute Lhs)) Refl))

;; "+1" = [43 49] -> None   (leading operator)
(claim t_leadop
  (Goal (list) (list)
    (Equation (Call 'spec_run (list (Ctor 'Cons (list (IntLit 43) (Ctor 'Cons (list (IntLit 49) (Ctor 'Nil (list))))))))
              (Ctor 'None (list))))
  (Steps (list (Compute Lhs)) Refl))

;; "" = [] -> None   (empty)
(claim t_empty
  (Goal (list) (list)
    (Equation (Call 'spec_run (list (Ctor 'Nil (list)))) (Ctor 'None (list))))
  (Steps (list (Compute Lhs)) Refl))

;; "1+" = [49 43] -> None   (trailing operator)
(claim t_trailop
  (Goal (list) (list)
    (Equation (Call 'spec_run (list (Ctor 'Cons (list (IntLit 49) (Ctor 'Cons (list (IntLit 43) (Ctor 'Nil (list))))))))
              (Ctor 'None (list))))
  (Steps (list (Compute Lhs)) Refl))

;; "1 2" = [49 32 50] -> None   (two numbers, no operator)
(claim t_twonum
  (Goal (list) (list)
    (Equation (Call 'spec_run (list (Ctor 'Cons (list (IntLit 49) (Ctor 'Cons (list (IntLit 32) (Ctor 'Cons (list (IntLit 50) (Ctor 'Nil (list))))))))))
              (Ctor 'None (list))))
  (Steps (list (Compute Lhs)) Refl))
