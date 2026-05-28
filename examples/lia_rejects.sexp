;;; Negative example. The kernel should REJECT this claim and the
;;; binary should exit non-zero. Used to demonstrate the failure path
;;; — not part of the "everything works" suite.
;;;
;;; (+ x y) = (+ x 1): LIA normalizes lhs - rhs to (y - 1), which is
;;; non-zero in general. Expect FAIL.

(claim plus_x_y_equals_plus_x_1
  (Goal
    (Cons (Param (quote x) (TCon (quote Int) Nil))
      (Cons (Param (quote y) (TCon (quote Int) Nil))
        Nil))
    Nil
    (Equation
      (Call (quote +)
        (Cons (FVar (quote x))
          (Cons (FVar (quote y)) Nil)))
      (Call (quote +)
        (Cons (FVar (quote x))
          (Cons (IntLit 1) Nil)))))
  (ByTheory (quote lia) (Cert (quote lia) Nil)))
