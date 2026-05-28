;;; Example proof file for the `check` binary.
;;;
;;; Each (claim NAME GOAL PROOF) form is a theorem with its proof.
;;; The `check` binary loads the bundled kernel, lifts each Goal to a
;;; Sequent (no hyps), and runs check_sequent. Successful claims are
;;; consed onto a running Theory so later ones could cite them via
;;; (Lemma NAME) — though these LIA examples don't yet use that.
;;;
;;; Surface syntax notes:
;;;   - Bare symbols are FVars by default; (quote x) makes a SymLit.
;;;     `Param`, `TCon`, `Equation`, etc. are ctors so they resolve
;;;     correctly without quoting.
;;;   - Lists are written as explicit Cons-Nil chains. A `list`
;;;     helper would help; that's a follow-up.
;;;   - (IntLit 5) is the narrow Expr value for the literal 5;
;;;     the outer IntLit is a ctor of the kernel's Expr ADT.

;; ∀ x y : Int. x + y = y + x   (commutativity of +)
(claim plus_comm
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
        (Cons (FVar (quote y))
          (Cons (FVar (quote x)) Nil)))))
  (ByTheory (quote lia) (Cert (quote lia) Nil)))

;; 1 + (2 + 3) = 6   (closed arithmetic constant)
(claim sums_constants
  (Goal
    Nil
    Nil
    (Equation
      (Call (quote +)
        (Cons (IntLit 1)
          (Cons (Call (quote +)
                  (Cons (IntLit 2)
                    (Cons (IntLit 3) Nil)))
            Nil)))
      (IntLit 6)))
  (ByTheory (quote lia) (Cert (quote lia) Nil)))

;; ∀ x : Int. (x + 1) - x = 1   (mixed atom + constant cancellation)
(claim plus_one_minus_self
  (Goal
    (Cons (Param (quote x) (TCon (quote Int) Nil)) Nil)
    Nil
    (Equation
      (Call (quote -)
        (Cons (Call (quote +)
                (Cons (FVar (quote x))
                  (Cons (IntLit 1) Nil)))
          (Cons (FVar (quote x)) Nil)))
      (IntLit 1)))
  (ByTheory (quote lia) (Cert (quote lia) Nil)))

