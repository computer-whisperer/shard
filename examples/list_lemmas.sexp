;;; The reverse proof tower: prove (fast xs Nil) = (rev xs) by chaining
;;; auxiliary lemmas. Direct port of v1's capstone result (TRANSFER.md,
;;; "Optimized — linear reverse. Proven fast = rev for all lists").
;;;
;;; Stated over (List Int) — v2's erased polymorphism (REVISIT)
;;; means each lemma is monomorphic at one element type.
;;;
;;; Lemma chain:
;;;   1. append_nil_right
;;;   2. append_cons_singleton  (LCF helper: targets a reduction
;;;                              that Unfold/Reduce can't reach
;;;                              when the outer call has a stuck arg)
;;;   3. append_assoc
;;;   4. fast_acc_lemma         (the key — induction with IH used
;;;                              at non-Nil acc, citing append_assoc)
;;;   5. fast_eq_rev            (capstone — no induction, just chain
;;;                              fast_acc_lemma + append_nil_right)

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
        ;; (append Nil Nil): unfold + reduce → Nil. Refl on Nil = Nil.
        (Steps (list (Unfold 'append Lhs) (Reduce Lhs)) Refl))
      (Case 'Cons
        ;; (append (Cons _f1 _f2) Nil): unfold + reduce →
        ;; (Cons _f1 (append _f2 Nil)). IH at Hyp 0 says
        ;; (append _f2 Nil) = _f2. Rewrite Hyp 0 Lr Lhs → (Cons _f1 _f2).
        ;; Rhs is (Cons _f1 _f2). Refl.
        (Steps
          (list (Unfold 'append Lhs)
                (Reduce Lhs)
                (Rewrite (Hyp 0) Lr Lhs True (list)))
          Refl)))))

;; ---------------------------------------------------------------------------
;; Lemmas 2 / 3 (helpers): the two ctor-arm bodies of `append` stated
;; as standalone lemmas. Both provable by Unfold + Reduce — no
;; induction.
;;
;; LCF discipline (forced by v2 kernel limits): the kernel's Unfold
;; step is greedy on the outermost matching call, and Reduce ι
;; doesn't descend into stuck Calls. When a proof has nested
;; (append (append _ _) _) anywhere, we can't unfold the inner one
;; through the kernel's stepping. Per recursive fn, we lift each
;; ctor branch to a lemma so the rewriter can do targeted
;; reductions at any depth.
;; ---------------------------------------------------------------------------

(claim append_nil_step
  (Goal
    (list (Param 'ys (ty List Int)))
    (list)
    (Equation
      (Call 'append (list (Ctor 'Nil (list)) (FVar 'ys)))
      (FVar 'ys)))
  (Steps (list (Unfold 'append Lhs) (Reduce Lhs)) Refl))

(claim append_cons_step
  (Goal
    (list (Param 'x  (ty Int))
          (Param 'xs (ty List Int))
          (Param 'ys (ty List Int)))
    (list)
    (Equation
      (Call 'append (list (Ctor 'Cons (list (FVar 'x) (FVar 'xs)))
                          (FVar 'ys)))
      (Ctor 'Cons (list (FVar 'x)
                        (Call 'append (list (FVar 'xs) (FVar 'ys)))))))
  (Steps (list (Unfold 'append Lhs) (Reduce Lhs)) Refl))

;; ---------------------------------------------------------------------------
;; Lemma 3: ∀ xs ys zs : List Int.
;;   (append (append xs ys) zs) = (append xs (append ys zs)).
;; Three ∀-vars; induction on xs. The IH becomes a quantified Hyp
;; over the OTHER two vars (ys, zs), so the Rewrite (Hyp 0) Lr Lhs
;; True (list) call exercises pat-var Rewrite with multi-var IH.
;; ---------------------------------------------------------------------------

;; ---------------------------------------------------------------------------
;; Per-ctor step lemmas for rev and fast (same LCF discipline).
;; ---------------------------------------------------------------------------

(claim rev_nil_step
  (Goal (list) (list)
    (Equation
      (Call 'rev (list (Ctor 'Nil (list))))
      (Ctor 'Nil (list))))
  (Steps (list (Unfold 'rev Lhs) (Reduce Lhs)) Refl))

(claim rev_cons_step
  (Goal
    (list (Param 'h (ty Int))
          (Param 't (ty List Int)))
    (list)
    (Equation
      (Call 'rev (list (Ctor 'Cons (list (FVar 'h) (FVar 't)))))
      (Call 'append
        (list (Call 'rev (list (FVar 't)))
              (Ctor 'Cons (list (FVar 'h) (Ctor 'Nil (list))))))))
  (Steps (list (Unfold 'rev Lhs) (Reduce Lhs)) Refl))

(claim fast_nil_step
  (Goal
    (list (Param 'acc (ty List Int)))
    (list)
    (Equation
      (Call 'fast (list (Ctor 'Nil (list)) (FVar 'acc)))
      (FVar 'acc)))
  (Steps (list (Unfold 'fast Lhs) (Reduce Lhs)) Refl))

(claim fast_cons_step
  (Goal
    (list (Param 'h   (ty Int))
          (Param 't   (ty List Int))
          (Param 'acc (ty List Int)))
    (list)
    (Equation
      (Call 'fast (list (Ctor 'Cons (list (FVar 'h) (FVar 't))) (FVar 'acc)))
      (Call 'fast
        (list (FVar 't)
              (Ctor 'Cons (list (FVar 'h) (FVar 'acc)))))))
  (Steps (list (Unfold 'fast Lhs) (Reduce Lhs)) Refl))

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
        ;; lhs: (append (append Nil ys) zs).
        ;;   Cite append_nil_step Lr Lhs (matches inner (append Nil ys)).
        ;;   Lhs becomes (append ys zs).
        ;; rhs: (append Nil (append ys zs)).
        ;;   Unfold + Reduce (outer first arg is Nil, fires Nil arm).
        ;;   Rhs becomes (append ys zs).
        ;; Refl.
        (Steps (list (Rewrite (Lemma 'append_nil_step) Lr Lhs True (list))
                     (Unfold 'append Rhs) (Reduce Rhs))
               Refl))
      (Case 'Cons
        ;; lhs: (append (append (Cons _f1 _f2) ys) zs).
        ;;   Cite append_cons_step Lr Lhs (matches inner append since
        ;;   its first arg is Cons-headed; outer can't match — first
        ;;   arg is a Call). Lhs becomes
        ;;     (append (Cons _f1 (append _f2 ys)) zs).
        ;;   Cite append_cons_step Lr Lhs again (now outer matches):
        ;;     (Cons _f1 (append (append _f2 ys) zs)).
        ;;   Cite IH (Hyp 0) Lr Lhs all=True. IH pattern is
        ;;     (append (append _f2 a) b); captures a := ys, b := zs.
        ;;   Lhs becomes (Cons _f1 (append _f2 (append ys zs))).
        ;; rhs: (append (Cons _f1 _f2) (append ys zs)).
        ;;   Cite append_cons_step Lr Rhs:
        ;;     (Cons _f1 (append _f2 (append ys zs))).
        ;; Refl.
        (Steps (list (Rewrite (Lemma 'append_cons_step)
                              Lr Lhs True (list))
                     (Rewrite (Lemma 'append_cons_step)
                              Lr Lhs True (list))
                     (Rewrite (Hyp 0) Lr Lhs True (list))
                     (Rewrite (Lemma 'append_cons_step)
                              Lr Rhs True (list)))
               Refl)))))

;; ---------------------------------------------------------------------------
;; Lemma 4 (key): ∀ xs acc : List Int. (fast xs acc) = (append (rev xs) acc).
;; Induct on xs. Cons case uses the IH non-trivially (instantiates
;; the IH's acc' to (Cons _f1 acc), exercising pat-var Rewrite with
;; a captured non-FVar binding) AND cites append_assoc. This is the
;; biggest single proof in the tower.
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
        ;; lhs: (fast Nil acc) -> via fast_nil_step Lhs -> acc.
        ;; rhs: (append (rev Nil) acc).
        ;;   rev_nil_step Rhs: (rev Nil) -> Nil.
        ;;   rhs becomes (append Nil acc).
        ;;   append_nil_step Rhs: -> acc.
        ;; Refl.
        (Steps (list (Rewrite (Lemma 'fast_nil_step)   Lr Lhs True (list))
                     (Rewrite (Lemma 'rev_nil_step)    Lr Rhs True (list))
                     (Rewrite (Lemma 'append_nil_step) Lr Rhs True (list)))
               Refl))
      (Case 'Cons
        ;; IH (Hyp 0): ∀ acc'. (fast _f2 acc') = (append (rev _f2) acc').
        ;;
        ;; lhs: (fast (Cons _f1 _f2) acc).
        ;;   fast_cons_step Lhs -> (fast _f2 (Cons _f1 acc)).
        ;;   Hyp 0 (IH) Lhs: pat (fast _f2 fresh_a) captures
        ;;     fresh_a := (Cons _f1 acc). Replaces with
        ;;     (append (rev _f2) (Cons _f1 acc)).
        ;;   lhs is now (append (rev _f2) (Cons _f1 acc)).
        ;;
        ;; rhs: (append (rev (Cons _f1 _f2)) acc).
        ;;   rev_cons_step Rhs (matches inner (rev (Cons _f1 _f2))):
        ;;     rhs becomes (append (append (rev _f2) (Cons _f1 Nil)) acc).
        ;;   append_assoc Lr Rhs (3-var pat-var match captures
        ;;     a := (rev _f2), b := (Cons _f1 Nil), c := acc):
        ;;     rhs becomes (append (rev _f2) (append (Cons _f1 Nil) acc)).
        ;;   append_cons_step Lr Rhs (matches inner (append (Cons …) acc)):
        ;;     inner becomes (Cons _f1 (append Nil acc)).
        ;;     rhs becomes (append (rev _f2) (Cons _f1 (append Nil acc))).
        ;;   append_nil_step Lr Rhs:
        ;;     rhs becomes (append (rev _f2) (Cons _f1 acc)).
        ;; Lhs == Rhs. Refl.
        (Steps (list (Rewrite (Lemma 'fast_cons_step)   Lr Lhs True (list))
                     (Rewrite (Hyp 0)                   Lr Lhs True (list))
                     (Rewrite (Lemma 'rev_cons_step)    Lr Rhs True (list))
                     (Rewrite (Lemma 'append_assoc)     Lr Rhs True (list))
                     (Rewrite (Lemma 'append_cons_step) Lr Rhs True (list))
                     (Rewrite (Lemma 'append_nil_step)  Lr Rhs True (list)))
               Refl)))))

;; ---------------------------------------------------------------------------
;; Lemma 5 (CAPSTONE): ∀ xs : List Int. (fast xs Nil) = (rev xs).
;;
;; Direct port of v1's headline result: accumulator-passing linear
;; reverse equals the naive O(n²) reverse, for all lists.
;;
;; No induction needed — just chain two prior lemmas:
;;   fast_acc_lemma at acc := Nil: (fast xs Nil) = (append (rev xs) Nil).
;;   append_nil_right: (append (rev xs) Nil) = (rev xs).
;; Transitivity → (fast xs Nil) = (rev xs). Refl.
;; ---------------------------------------------------------------------------

(claim fast_eq_rev
  (Goal
    (list (Param 'xs (ty List Int)))
    (list)
    (Equation
      (Call 'fast (list (FVar 'xs) (Ctor 'Nil (list))))
      (Call 'rev (list (FVar 'xs)))))
  ;; fast_acc_lemma Lr Lhs: pat (fast a b), captures a=xs, b=Nil.
  ;;   replaces lhs with (append (rev xs) Nil).
  ;; append_nil_right Lr Lhs: pat (append a Nil), captures a=(rev xs).
  ;;   replaces lhs with (rev xs).
  ;; Refl on (rev xs) = (rev xs).
  (Steps (list (Rewrite (Lemma 'fast_acc_lemma)    Lr Lhs True (list))
               (Rewrite (Lemma 'append_nil_right) Lr Lhs True (list)))
         Refl))
