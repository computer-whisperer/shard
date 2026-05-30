;;; std/list — polymorphic (List T): append / len / rev + their algebra.

;;; List operations over polymorphic (List T). Slice 31 moved these from
;;; monomorphic (List Int) to (List T) — the proofs hold for any element
;;; type and the kernel handles polymorphic Goals natively (the loader
;;; change is what surfaces the capability).
;;;
;;; Each fn is structurally recursive on its first list arg, so they are
;;; total in the kernel's accepted fragment.
;;;
;;; The accumulator-reverse correctness proof (fast = rev) that these
;;; lemmas were originally built for now lives in examples/reverse_proof.sexp
;;; — nothing in the library depends on `fast`, so it ships as an example.

(import "order.sexp")   ; len_nonneg cites le0_succ

;; append: (append xs ys) is xs ++ ys.
(fn (append T) ((xs (List T)) (ys (List T))) (List T)
  (match xs
    (Nil          ys)
    ((Cons h t)   (Cons h (append t ys)))))

;; len: Int-valued list length. Int (not Nat) so it can serve directly as a
;; WfInduct measure (μ : params → Int). Structural on the spine — total.
(fn (len T) ((xs (List T))) Int
  (match xs
    (Nil          0)
    ((Cons h t)   (+ 1 (len t)))))

;; 0 <= len xs — discharges WfInduct's measure>=0 obligation. Induction on the
;; spine; the step bumps the IH's lower bound by one (le0_succ, farkas).
(claim len_nonneg
  (Goal
    (list (Param 'xs (ty List (tv T))))
    (list)
    (Equation (Call 'le (list (IntLit 0) (Call 'len (list (FVar 'xs)))))
              (Ctor 'True (list))))
  (Induct 'xs
    (list
      (Case 'Nil  (Steps (list (Simp Lhs)) Refl))
      (Case 'Cons
        (Steps (list (Simp Lhs))
          (RewriteWith (Lemma 'le0_succ) Lr Lhs (list)
            (list (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
            Refl))))))

;; rev: naive O(n^2) reverse. Each Cons re-appends a singleton.
(fn (rev T) ((xs (List T))) (List T)
  (match xs
    (Nil          Nil)
    ((Cons h t)   (append (rev t) (Cons h Nil)))))

;; ---- append algebra ----
;;; The structural lemmas about append (identity, associativity) plus the
;;; length/reverse interaction tower below. Slice 31: each claim is stated
;;; once over a type variable T (written `(tv T)`); the kernel's pattern
;;; matching is type-agnostic, so a single proof serves all element types.
;;; Slice 30: gated δ in Simp collapsed the old per-ctor helper lemmas
;;; (append_nil_step, etc.) into single Simp steps.

;; ---------------------------------------------------------------------------
;; append_nil_right: ∀ xs : List T. (append xs Nil) = xs.
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
;; append_assoc: ∀ xs ys zs : List T.
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

;; ===========================================================================
;; Length / reverse interaction tower.
;;
;; The keystone is len_append: len distributes over append as integer
;; addition. From it len_rev follows; rev_append + rev_rev complete the
;; reverse algebra. These are the generic structural-recursion lemmas any
;; list-shaped proof reuses (e.g. a parser that recurses on len) — the
;; calc proof had to route around their absence with bespoke farkas
;; decreases.  Authored with named hypotheses ((Hyp 'ih) — the Induct IH)
;; rather than positional indices.
;; ===========================================================================

;; ---------------------------------------------------------------------------
;; len_append: ∀ xs ys. (len (append xs ys)) = (len xs) + (len ys).
;; Induct on xs; the Cons step rewrites by the IH then reassociates the
;; leading +1 with lia (len t, len ys are opaque atoms to the theory).
;; ---------------------------------------------------------------------------

(claim len_append
  (Goal
    (list (Param 'xs (ty List (tv T)))
          (Param 'ys (ty List (tv T))))
    (list)
    (Equation
      (Call 'len (list (Call 'append (list (FVar 'xs) (FVar 'ys)))))
      (Call '+ (list (Call 'len (list (FVar 'xs)))
                     (Call 'len (list (FVar 'ys)))))))
  (Induct 'xs
    (list
      (Case 'Nil
        (Steps (list (Simp Both)) (ByTheory 'lia (Cert 'lia (list)))))
      (Case 'Cons
        (Steps (list (Simp Both)
                     (Rewrite (Hyp 'ih) Lr Lhs True (list)))
               (ByTheory 'lia (Cert 'lia (list))))))))

;; ---------------------------------------------------------------------------
;; len_rev: ∀ xs. (len (rev xs)) = (len xs). The Cons step distributes len
;; over the singleton re-append (len_append), folds the IH, and reassociates.
;; ---------------------------------------------------------------------------

(claim len_rev
  (Goal
    (list (Param 'xs (ty List (tv T))))
    (list)
    (Equation
      (Call 'len (list (Call 'rev (list (FVar 'xs)))))
      (Call 'len (list (FVar 'xs)))))
  (Induct 'xs
    (list
      (Case 'Nil (Steps (list (Simp Both)) Refl))
      (Case 'Cons
        (Steps (list (Simp Lhs)
                     (Rewrite (Lemma 'len_append) Lr Lhs True (list))
                     (Rewrite (Hyp 'ih)           Lr Lhs True (list))
                     (Simp Both))
               (ByTheory 'lia (Cert 'lia (list))))))))

;; ---------------------------------------------------------------------------
;; rev_append: ∀ xs ys. (rev (append xs ys)) = (append (rev ys) (rev xs)).
;; rev distributes over append, flipping order. Nil uses append_nil_right;
;; the Cons step folds the IH then reassociates with append_assoc.
;; ---------------------------------------------------------------------------

(claim rev_append
  (Goal
    (list (Param 'xs (ty List (tv T)))
          (Param 'ys (ty List (tv T))))
    (list)
    (Equation
      (Call 'rev (list (Call 'append (list (FVar 'xs) (FVar 'ys)))))
      (Call 'append (list (Call 'rev (list (FVar 'ys)))
                          (Call 'rev (list (FVar 'xs)))))))
  (Induct 'xs
    (list
      (Case 'Nil
        (Steps (list (Simp Both)
                     (Rewrite (Lemma 'append_nil_right) Lr Rhs True (list)))
               Refl))
      (Case 'Cons
        (Steps (list (Simp Lhs)
                     (Rewrite (Hyp 'ih)             Lr Lhs True (list))
                     (Rewrite (Lemma 'append_assoc) Lr Lhs True (list))
                     (Simp Rhs))
               Refl)))))

;; ---------------------------------------------------------------------------
;; rev_rev: ∀ xs. (rev (rev xs)) = xs — reverse is an involution. The Cons
;; step turns the nested rev into an append via rev_append, folds the IH,
;; then Simp collapses the singleton append back to (Cons h t).
;; ---------------------------------------------------------------------------

(claim rev_rev
  (Goal
    (list (Param 'xs (ty List (tv T))))
    (list)
    (Equation
      (Call 'rev (list (Call 'rev (list (FVar 'xs)))))
      (FVar 'xs)))
  (Induct 'xs
    (list
      (Case 'Nil (Steps (list (Simp Both)) Refl))
      (Case 'Cons
        (Steps (list (Simp Lhs)
                     (Rewrite (Lemma 'rev_append) Lr Lhs True (list))
                     (Rewrite (Hyp 'ih)           Lr Lhs True (list))
                     (Simp Lhs))
               Refl)))))
