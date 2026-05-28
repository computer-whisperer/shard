;;; Negative example. The kernel should REJECT this claim and the
;;; binary should exit non-zero. Used to demonstrate the failure path
;;; — not part of the "everything works" suite.
;;;
;;; (+ x y) = (+ x 1): LIA normalizes lhs - rhs to (y - 1), which is
;;; non-zero in general. Expect FAIL.

(claim plus_x_y_equals_plus_x_1
  (Goal
    (list (Param 'x (ty Int))
          (Param 'y (ty Int)))
    (list)
    (Equation
      (Call '+ (list (FVar 'x) (FVar 'y)))
      (Call '+ (list (FVar 'x) (IntLit 1)))))
  (ByTheory 'lia (Cert 'lia (list))))
