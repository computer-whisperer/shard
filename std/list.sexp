;;; std/list — polymorphic (List T): append/rev/fast + the reverse tower.

;;; List operations over polymorphic (List T). Definitions for the
;;; reverse proof tower (rev = fast). Slice 31 moved these from
;;; monomorphic (List Int) to (List T) — the proofs hold for any
;;; element type and the kernel handles polymorphic Goals natively
;;; (the loader change is what surfaces the capability).
;;;
;;; All three fns are structurally recursive on their first list arg,
;;; so they're total in the kernel's accepted fragment.

;; append: (append xs ys) is xs ++ ys.
(fn (append T) ((xs (List T)) (ys (List T))) (List T)
  (match xs
    (Nil          ys)
    ((Cons h t)   (Cons h (append t ys)))))

;; rev: naive O(n^2) reverse. Each Cons re-appends a singleton.
(fn (rev T) ((xs (List T))) (List T)
  (match xs
    (Nil          Nil)
    ((Cons h t)   (append (rev t) (Cons h Nil)))))

;; fast: accumulator-passing O(n) reverse.
(fn (fast T) ((xs (List T)) (acc (List T))) (List T)
  (match xs
    (Nil          acc)
    ((Cons h t)   (fast t (Cons h acc)))))

;; ---- lemmas ----
;;; The reverse proof tower: prove (fast xs Nil) = (rev xs) by chaining
;;; auxiliary lemmas. Direct port of v1's capstone result (TRANSFER.md,
;;; "Optimized — linear reverse. Proven fast = rev for all lists").
;;;
;;; Slice 31 update — polymorphic over (List T). Each claim is stated
;;; once over a type variable T (written `(tv T)` in claim bodies);
;;; the kernel's pattern matching is type-agnostic so a single proof
;;; serves all element types. The `_at_int` and `_at_sym` claims at
;;; the bottom demonstrate citation at concrete instantiations — the
;;; v2 mandate's headline polymorphism use case.
;;;
;;; Slice 30 update — Simp guarding. The original tower needed per-
;;; ctor helper lemmas (append_nil_step, fast_cons_step, etc.) to
;;; route around the kernel's unguarded reducer. With gated δ in
;;; Simp, those helpers collapse into single Simp steps.
;;;
;;; Lemma chain:
;;;   1. append_nil_right
;;;   2. append_assoc
;;;   3. fast_acc_lemma  (the key — induction with IH used at
;;;                       non-Nil acc, citing append_assoc)
;;;   4. fast_eq_rev     (capstone — no induction, just chain
;;;                       fast_acc_lemma + append_nil_right)
;;;   5. fast_eq_rev_at_int   (reuse demo — cite (4) at (List Int))
;;;   6. fast_eq_rev_at_sym   (reuse demo — cite (4) at (List Symbol))


;; ---------------------------------------------------------------------------
;; Lemma 1: ∀ xs : List T. (append xs Nil) = xs.
;; ---------------------------------------------------------------------------

(claim append_nil_right
  (Goal
    (list (Param 'xs (ty List (tv T))))
    (list)
    (Equation
      (Call 'append (list (FVar 'xs) (Ctor 'Nil (list))))
      (FVar 'xs)))
  (Induct 'xs
    (list
      (Case 'Nil
        ;; (append Nil Nil): Simp drives it to Nil.
        (Steps (list (Simp Lhs)) Refl))
      (Case 'Cons
        ;; (append (Cons _f1 _f2) Nil): Simp reduces to
        ;; (Cons _f1 (append _f2 Nil)); IH at Hyp 0 closes.
        (Steps
          (list (Simp Lhs)
                (Rewrite (Hyp 0) Lr Lhs True (list)))
          Refl)))))

;; ---------------------------------------------------------------------------
;; Lemma 2: ∀ xs ys zs : List T.
;;   (append (append xs ys) zs) = (append xs (append ys zs)).
;; ---------------------------------------------------------------------------

(claim append_assoc
  (Goal
    (list (Param 'xs (ty List (tv T)))
          (Param 'ys (ty List (tv T)))
          (Param 'zs (ty List (tv T))))
    (list)
    (Equation
      (Call 'append
        (list (Call 'append (list (FVar 'xs) (FVar 'ys)))
              (FVar 'zs)))
      (Call 'append
        (list (FVar 'xs)
              (Call 'append (list (FVar 'ys) (FVar 'zs)))))))
  (Induct 'xs
    (list
      (Case 'Nil
        (Steps (list (Simp Both)) Refl))
      (Case 'Cons
        (Steps (list (Simp Both)
                     (Rewrite (Hyp 0) Lr Lhs True (list)))
               Refl)))))

;; ---------------------------------------------------------------------------
;; Lemma 3 (key): ∀ xs acc : List T. (fast xs acc) = (append (rev xs) acc).
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
                     (Rewrite (Hyp 0)                Lr Lhs True (list))
                     (Simp Rhs)
                     (Rewrite (Lemma 'append_assoc) Lr Rhs True (list))
                     (Simp Rhs))
               Refl)))))

;; ---------------------------------------------------------------------------
;; Lemma 4 (CAPSTONE): ∀ xs : List T. (fast xs Nil) = (rev xs).
;;
;; Polymorphic port of v1's headline result. Chain fast_acc_lemma at
;; acc := Nil + append_nil_right; Refl. No element-type assumption.
;; ---------------------------------------------------------------------------

(claim fast_eq_rev
  (Goal
    (list (Param 'xs (ty List (tv T))))
    (list)
    (Equation
      (Call 'fast (list (FVar 'xs) (Ctor 'Nil (list))))
      (Call 'rev (list (FVar 'xs)))))
  (Steps (list (Rewrite (Lemma 'fast_acc_lemma)    Lr Lhs True (list))
               (Rewrite (Lemma 'append_nil_right) Lr Lhs True (list)))
         Refl))

;; ---------------------------------------------------------------------------
;; Reuse demos: cite the polymorphic capstone at concrete element
;; types. Each is a one-step Rewrite — pat-var Rewrite matches the
;; polymorphic LHS pattern (fast a Nil) against the concrete-typed
;; goal lhs (the rewriter is type-agnostic) and substitutes the RHS.
;; This is the proof-reuse story TRANSFER mandates: one proof, many
;; element types.
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
