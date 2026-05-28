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

;; ---------------------------------------------------------------------------
;; Slice 32 demo — Insts pre-instantiation.
;;
;; The lemma `pad_with_pivot` states ∀ pivot a : Int. a = (a - pivot) + pivot.
;; LIA decides it as a polynomial identity. Its LHS is just `a`, so the
;; rewriter's conclusion-match path can pin `a` from any goal LHS but
;; the `pivot` ∀-binder is invisible to the match. The only way to cite
;; this lemma is with an Inst that pre-instantiates pivot.
;; ---------------------------------------------------------------------------

(claim pad_with_pivot
  (Goal
    (list (Param 'pivot (ty Int))
          (Param 'a     (ty Int)))
    (list)
    (Equation
      (FVar 'a)
      (Call '+ (list (Call '- (list (FVar 'a) (FVar 'pivot)))
                     (FVar 'pivot)))))
  (ByTheory 'lia (Cert 'lia (list))))

;; Cite pad_with_pivot Lr Lhs at goal 5 = (5 - 3) + 3, pinning pivot := 3.
;; Without (Inst 'pivot (IntLit 3)) the rewriter would substitute `pivot`
;; with an FVar that never appears in the goal, leaving the equation
;; structurally non-equal. With the Inst, the substituted RHS exactly
;; matches the goal's RHS, and Refl closes.
(claim pad_5_with_3
  (Goal (list) (list)
    (Equation
      (IntLit 5)
      (Call '+ (list (Call '- (list (IntLit 5) (IntLit 3)))
                     (IntLit 3)))))
  (Steps
    (list (Rewrite (Lemma 'pad_with_pivot) Lr Lhs True
                   (list (Inst 'pivot (IntLit 3)))))
    Refl))
