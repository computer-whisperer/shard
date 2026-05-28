;;; The reverse proof tower: prove (fast xs Nil) = (rev xs) by chaining
;;; auxiliary lemmas. Direct port of v1's capstone result (TRANSFER.md,
;;; "Optimized — linear reverse. Proven fast = rev for all lists").
;;;
;;; Stated over (List Int) — v2's erased polymorphism (REVISIT)
;;; means each lemma is monomorphic at one element type.
;;;
;;; Slice 30 update — Simp guarding lands. The original tower needed
;;; per-ctor helper lemmas (append_nil_step, fast_cons_step, etc.) to
;;; route around the kernel's unguarded reducer. With gated δ in
;;; Simp, those helpers collapse into single Simp steps. The proof
;;; tower shrank from 10 lemmas to 4. The bridging Unfold in the
;;; append_assoc Nil case is the only residual surface-form
;;; reconciliation step.
;;;
;;; Lemma chain:
;;;   1. append_nil_right
;;;   2. append_assoc
;;;   3. fast_acc_lemma  (the key — induction with IH used at
;;;                       non-Nil acc, citing append_assoc)
;;;   4. fast_eq_rev     (capstone — no induction, just chain
;;;                       fast_acc_lemma + append_nil_right)

(use-module "list_lib.sexp")

;; ---------------------------------------------------------------------------
;; Lemma 1: ∀ xs : List Int. (append xs Nil) = xs.
;; Standard structural induction on xs.
;; ---------------------------------------------------------------------------

(claim append_nil_right
  (Goal
    (list (Param 'xs (ty List Int)))
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
        ;; (append (Cons _f1 _f2) Nil): Simp reduces the outer call
        ;; (gate passes — Cons-headed scrutinee fires the Match) and
        ;; stops at the recursive sub-call (append _f2 Nil) where the
        ;; Match scrutinee is the FVar _f2 (gate fails). Lhs becomes
        ;; (Cons _f1 (append _f2 Nil)). IH at Hyp 0 says
        ;; (append _f2 Nil) = _f2. Rewrite Hyp 0 Lr Lhs → (Cons _f1 _f2).
        (Steps
          (list (Simp Lhs)
                (Rewrite (Hyp 0) Lr Lhs True (list)))
          Refl)))))

;; ---------------------------------------------------------------------------
;; Lemma 2: ∀ xs ys zs : List Int.
;;   (append (append xs ys) zs) = (append xs (append ys zs)).
;;
;; Three ∀-vars; induction on xs. The IH becomes a quantified Hyp
;; over the OTHER two vars (ys, zs), so the Rewrite (Hyp 0) Lr Lhs
;; call exercises pat-var Rewrite with a multi-var IH.
;;
;; Both ctor cases close with `(Simp Both) [...]; Refl`. The head-only
;; gate keeps Simp from tunneling past stuck Calls, so both sides
;; converge on the same surface form (Nil case → `(append ys zs)`;
;; Cons case → `(Cons _f1 ...)` with associativity-shifted tails).
;; The Cons-case IH bridges the associativity difference.
;; ---------------------------------------------------------------------------

(claim append_assoc
  (Goal
    (list (Param 'xs (ty List Int))
          (Param 'ys (ty List Int))
          (Param 'zs (ty List Int)))
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
        ;; Both sides Simp to (append ys zs); head-only gate refuses
        ;; to commit past the Match-on-FVar body of the surviving call.
        (Steps (list (Simp Both)) Refl))
      (Case 'Cons
        ;; Lhs Simp → (Cons _f1 (append (append _f2 ys) zs)).
        ;; Rhs Simp → (Cons _f1 (append _f2 (append ys zs))).
        ;; IH (Hyp 0): ∀ a b. (append (append _f2 a) b)
        ;;                = (append _f2 (append a b)).
        ;; Rewrite Hyp 0 Lr Lhs captures a=ys, b=zs. Lhs becomes Rhs.
        (Steps (list (Simp Both)
                     (Rewrite (Hyp 0) Lr Lhs True (list)))
               Refl)))))

;; ---------------------------------------------------------------------------
;; Lemma 3 (key): ∀ xs acc : List Int. (fast xs acc) = (append (rev xs) acc).
;; Induct on xs. Cons case uses the IH non-trivially (instantiates
;; the IH's acc' to (Cons _f1 acc), exercising pat-var Rewrite with
;; a captured non-FVar binding) AND cites append_assoc.
;; ---------------------------------------------------------------------------

(claim fast_acc_lemma
  (Goal
    (list (Param 'xs  (ty List Int))
          (Param 'acc (ty List Int)))
    (list)
    (Equation
      (Call 'fast (list (FVar 'xs) (FVar 'acc)))
      (Call 'append (list (Call 'rev (list (FVar 'xs))) (FVar 'acc)))))
  (Induct 'xs
    (list
      (Case 'Nil
        ;; Lhs Simp: (fast Nil acc) → acc.
        ;; Rhs Simp: (append (rev Nil) acc) → (append Nil acc) → acc.
        (Steps (list (Simp Both)) Refl))
      (Case 'Cons
        ;; IH (Hyp 0): ∀ acc'. (fast _f2 acc') = (append (rev _f2) acc').
        ;;
        ;; Lhs Simp: (fast (Cons _f1 _f2) acc) → (fast _f2 (Cons _f1 acc))
        ;;   (the recursive call's body Match is gated stuck on FVar _f2,
        ;;   so Simp stops at the surface form ready for IH application).
        ;; Hyp 0 Lr Lhs: pat (fast _f2 fresh_a) captures fresh_a :=
        ;;   (Cons _f1 acc). Lhs → (append (rev _f2) (Cons _f1 acc)).
        ;;
        ;; Rhs Simp: (append (rev (Cons _f1 _f2)) acc)
        ;;   → (append (append (rev _f2) (Cons _f1 Nil)) acc).
        ;; append_assoc Lr Rhs (3-var pat-var match captures
        ;;   a := (rev _f2), b := (Cons _f1 Nil), c := acc):
        ;;   Rhs → (append (rev _f2) (append (Cons _f1 Nil) acc)).
        ;; Rhs Simp again: (append (Cons _f1 Nil) acc) → (Cons _f1 acc).
        ;;   Rhs → (append (rev _f2) (Cons _f1 acc)).
        ;; Refl.
        (Steps (list (Simp Lhs)
                     (Rewrite (Hyp 0)                Lr Lhs True (list))
                     (Simp Rhs)
                     (Rewrite (Lemma 'append_assoc) Lr Rhs True (list))
                     (Simp Rhs))
               Refl)))))

;; ---------------------------------------------------------------------------
;; Lemma 4 (CAPSTONE): ∀ xs : List Int. (fast xs Nil) = (rev xs).
;;
;; Direct port of v1's headline result: accumulator-passing linear
;; reverse equals the naive O(n²) reverse, for all lists.
;;
;; No induction needed — just chain two prior lemmas:
;;   fast_acc_lemma at acc := Nil: (fast xs Nil) = (append (rev xs) Nil).
;;   append_nil_right:              (append (rev xs) Nil) = (rev xs).
;; Transitivity → (fast xs Nil) = (rev xs). Refl.
;; ---------------------------------------------------------------------------

(claim fast_eq_rev
  (Goal
    (list (Param 'xs (ty List Int)))
    (list)
    (Equation
      (Call 'fast (list (FVar 'xs) (Ctor 'Nil (list))))
      (Call 'rev (list (FVar 'xs)))))
  (Steps (list (Rewrite (Lemma 'fast_acc_lemma)    Lr Lhs True (list))
               (Rewrite (Lemma 'append_nil_right) Lr Lhs True (list)))
         Refl))
