;;; Example proof file for the `check` binary.
;;;
;;; Each (claim NAME GOAL PROOF) form is a theorem with its proof.
;;; The `check` binary loads the bundled kernel, lifts each Goal to a
;;; Sequent (no hyps), and runs check_sequent. Successful claims are
;;; consed onto a running Theory so later ones could cite them via
;;; (Lemma NAME) — though these LIA examples don't yet use that.
;;;
;;; Surface syntax notes:
;;;   - 'foo is reader sugar for (quote foo) → SymLit foo (via lexpr).
;;;   - (list a b c) expands at parse time to (Cons a (Cons b (Cons c Nil))).
;;;   - Ctor names like Param, TCon, Equation, FVar, IntLit resolve
;;;     against the kernel's ctor set automatically.

;; ∀ x y : Int. x + y = y + x   (commutativity of +)
(claim plus_comm
  (Goal
    (list (Param 'x (ty Int))
          (Param 'y (ty Int)))
    (list)
    (Equation
      (Call '+ (list (FVar 'x) (FVar 'y)))
      (Call '+ (list (FVar 'y) (FVar 'x)))))
  (ByTheory 'lia (Cert 'lia (list))))

;; 1 + (2 + 3) = 6   (closed arithmetic constant)
(claim sums_constants
  (Goal
    (list)
    (list)
    (Equation
      (Call '+ (list (IntLit 1)
                     (Call '+ (list (IntLit 2) (IntLit 3)))))
      (IntLit 6)))
  (ByTheory 'lia (Cert 'lia (list))))

;; ∀ x : Int. (x + 1) - x = 1   (mixed atom + constant cancellation)
(claim plus_one_minus_self
  (Goal
    (list (Param 'x (ty Int)))
    (list)
    (Equation
      (Call '-
        (list (Call '+ (list (FVar 'x) (IntLit 1)))
              (FVar 'x)))
      (IntLit 1)))
  (ByTheory 'lia (Cert 'lia (list))))
