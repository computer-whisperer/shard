;;; Reverse correctness: the accumulator-passing O(n) reverse equals the
;;; naive O(n^2) reverse, for lists of ANY element type.
;;;
;;;   (fast xs Nil) = (rev xs)        ∀ xs : List T
;;;
;;; This is v1's headline capstone (TRANSFER.md, "Optimized — linear
;;; reverse. Proven fast = rev for all lists"), ported to polymorphic
;;; (List T). `rev` and the append algebra (append_nil_right, append_assoc)
;;; live in std/list as standard library content; `fast` and its
;;; correctness proof live HERE as a worked example, since nothing in the
;;; library depends on them.
;;;
;;; The proof is a four-link chain:
;;;   1. append_nil_right   (std/list)
;;;   2. append_assoc       (std/list)
;;;   3. fast_acc_lemma     — the key: induction whose IH fires at a
;;;                           non-Nil accumulator, citing append_assoc.
;;;   4. fast_eq_rev        — the capstone: no induction, just chain
;;;                           fast_acc_lemma at acc:=Nil with append_nil_right.
;;; The two trailing claims re-cite the polymorphic capstone at concrete
;;; element types — the proof-reuse story: one proof, many element types.

(import "../std/list.sexp")   ; append/len/rev + append_nil_right, append_assoc

;; fast: accumulator-passing O(n) reverse. Structural on the first list arg.
(fn (fast T) ((xs (List T)) (acc (List T))) (List T)
  (match xs
    (Nil          acc)
    ((Cons h t)   (fast t (Cons h acc)))))

;; ---------------------------------------------------------------------------
;; fast_acc_lemma (key): ∀ xs acc : List T. (fast xs acc) = (append (rev xs) acc).
;; Induction on xs; the IH fires at the EXTENDED accumulator (Cons h acc),
;; and append_assoc reconciles the two association shapes.
;; ---------------------------------------------------------------------------

(claim fast_acc_lemma
  (Goal
    (list (Param 'xs  (ty List (tv T)))
          (Param 'acc (ty List (tv T))))
    (list)
    (Equation
      (Call 'fast (list (FVar 'xs) (FVar 'acc)))
      (Call 'append (list (Call 'rev (list (FVar 'xs))) (FVar 'acc)))))
  (Induct 'xs
    (list
      (Case 'Nil
        (Steps (list (Simp Both)) Refl))
      (Case 'Cons
        (Steps (list (Simp Lhs)
                     (Rewrite (Hyp 'ih)             Lr Lhs True (list))
                     (Simp Rhs)
                     (Rewrite (Lemma 'append_assoc) Lr Rhs True (list))
                     (Simp Rhs))
               Refl)))))

;; ---------------------------------------------------------------------------
;; fast_eq_rev (CAPSTONE): ∀ xs : List T. (fast xs Nil) = (rev xs).
;; Chain fast_acc_lemma at acc := Nil + append_nil_right; Refl. No element
;; type assumption.
;; ---------------------------------------------------------------------------

(claim fast_eq_rev
  (Goal
    (list (Param 'xs (ty List (tv T))))
    (list)
    (Equation
      (Call 'fast (list (FVar 'xs) (Ctor 'Nil (list))))
      (Call 'rev (list (FVar 'xs)))))
  (Steps (list (Rewrite (Lemma 'fast_acc_lemma)   Lr Lhs True (list))
               (Rewrite (Lemma 'append_nil_right) Lr Lhs True (list)))
         Refl))

;; ---------------------------------------------------------------------------
;; Reuse demos: cite the polymorphic capstone at concrete element types.
;; Each is a one-step Rewrite — the pat-var rewriter matches the polymorphic
;; LHS pattern (fast a Nil) against the concrete-typed goal lhs (it is
;; type-agnostic) and substitutes the RHS. One proof, many element types.
;; ---------------------------------------------------------------------------

(claim fast_eq_rev_at_int
  (Goal
    (list (Param 'xs (ty List Int)))
    (list)
    (Equation
      (Call 'fast (list (FVar 'xs) (Ctor 'Nil (list))))
      (Call 'rev (list (FVar 'xs)))))
  (Steps (list (Rewrite (Lemma 'fast_eq_rev) Lr Lhs True (list)))
         Refl))

(claim fast_eq_rev_at_sym
  (Goal
    (list (Param 'xs (ty List Symbol)))
    (list)
    (Equation
      (Call 'fast (list (FVar 'xs) (Ctor 'Nil (list))))
      (Call 'rev (list (FVar 'xs)))))
  (Steps (list (Rewrite (Lemma 'fast_eq_rev) Lr Lhs True (list)))
         Refl))
