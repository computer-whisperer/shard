;;; std/map — finite (Map V) over Int keys: lookup/insert + extensional lemmas.

;;; Finite maps over Int keys, polymorphic in the value type V.
;;;
;;; Representation: association list with PREPEND insert and FIRST-MATCH
;;; lookup. The simplest representation that fits a first-order, total,
;;; erased-polymorphism language — no key ordering, no balancing, and
;;; the lemma proofs stay tractable. Keys are Int (indexing/arrays, and
;;; reflexivity of `int_eq` falls out of the LIA backend via eqdec);
;;; generalizing to (Map K V) waits on a key-equality mechanism (the
;;; defunctionalized-HOF roadmap item).
;;;
;;; Consequence of prepend-insert: `insert` may leave shadowed entries
;;; in place, so two observationally-equal maps need not be structurally
;;; equal. The lemma library (map_lemmas.sexp) therefore states map
;;; facts EXTENSIONALLY — quantified over a probe key `j` under
;;; `lookup` — rather than as structural map equalities. That is the
;;; honest mathematical content and the standard first-order treatment.
;;;
;;; `lookup` is structurally recursive on the map; `insert` is a
;;; non-recursive constructor wrapper. Both are total in the kernel's
;;; accepted fragment.

(type (Map V)
  (MEmpty)
  (MCons Int V (Map V)))

;; lookup: first entry whose key `int_eq`s k, else None.
(fn (lookup V) ((k Int) (m (Map V))) (Option V)
  (match m
    (MEmpty None)
    ((MCons k2 v rest)
      (if (int_eq k k2) (Some v) (lookup k rest)))))

;; insert: prepend a fresh (k, v) binding. Shadows any prior k.
;;
;; A bare constructor — note this is exactly the shape Simp's gated δ
;; will NOT unfold (a ctor head is not a redex), so proofs peel it with
;; an explicit `Unfold 'insert` step before `Simp`.
(fn (insert V) ((k Int) (v V) (m (Map V))) (Map V)
  (MCons k v m))

;; ---- lemmas ----
;;; Finite-map lemma library over (Map V) with Int keys (map_lib.sexp).
;;;
;;; Slice 33 — first finite-maps slice. Demonstrates:
;;;   - the `eqdec` ByTheory backend discharging reflexivity of int_eq
;;;     on a variable (`int_eq_refl`), a fact the reducer leaves stuck;
;;;   - the EXTENSIONAL stating discipline: map facts are quantified
;;;     over a probe key `j` under `lookup`, never as structural map
;;;     equalities (prepend-insert leaves shadowed entries, so two equal
;;;     maps need not be structurally equal — see map_lib.sexp);
;;;   - `insert` peeled by an explicit `Unfold` step, since its bare-
;;;     constructor body is exactly what Simp's gated δ declines to
;;;     unfold (slice 30).
;;;
;;; Lemma chain:
;;;   1. int_eq_refl        (eqdec — reflexivity of int_eq on a var)
;;;   2. lookup_empty       (Simp — base case)
;;;   3. lookup_insert_eq   (the headline: read back what you wrote)
;;;   4. lookup_insert_neq  (conditional: a different key is unaffected)
;;;   5. insert_shadow      (extensional capstone: a re-insert at the
;;;                          same key shadows, observed via lookup)


;; ---------------------------------------------------------------------------
;; Lemma 1: ∀ k : Int. (int_eq k k) = True.
;;
;; A general arithmetic fact (not map-specific), but it's the enabling
;; lemma for lookup_insert_eq, so it lives here for now. Decided by the
;; eqdec backend (which delegates int_eq to lia_decide): the diff
;; k - k canonicalizes to zero, so the equality holds. Proven, not
;; axiomatized — the audit ledger stays empty.
;; ---------------------------------------------------------------------------

(claim int_eq_refl
  (Goal
    (list (Param 'k (ty Int)))
    (list)
    (Equation
      (Call 'int_eq (list (FVar 'k) (FVar 'k)))
      (Ctor 'True (list))))
  (ByTheory 'eqdec (Cert 'eqdec (list))))

;; ---------------------------------------------------------------------------
;; Lemma 2: ∀ k : Int. (lookup k MEmpty) = None.
;;
;; The empty map has no bindings. Simp unfolds lookup, fires the MEmpty
;; match arm to None. (No insert to peel, so no Unfold needed.)
;; ---------------------------------------------------------------------------

(claim lookup_empty
  (Goal
    (list (Param 'k (ty Int)))
    (list)
    (Equation
      (Call 'lookup (list (FVar 'k) (Ctor 'MEmpty (list))))
      (Ctor 'None (list))))
  (Steps (list (Simp Lhs)) Refl))

;; ---------------------------------------------------------------------------
;; Lemma 3 (headline): ∀ k : Int, v : V, m : (Map V).
;;   (lookup k (insert k v m)) = (Some v).
;;
;; "Read back the value you just wrote." Peel insert (Unfold), Simp to
;; expose (if (int_eq k k) (Some v) (lookup k m)), rewrite the stuck
;; (int_eq k k) to True via int_eq_refl, Simp fires the if to (Some v).
;; ---------------------------------------------------------------------------

(claim lookup_insert_eq
  (Goal
    (list (Param 'k (ty Int))
          (Param 'v (tv V))
          (Param 'm (ty Map (tv V))))
    (list)
    (Equation
      (Call 'lookup
        (list (FVar 'k) (Call 'insert (list (FVar 'k) (FVar 'v) (FVar 'm)))))
      (Ctor 'Some (list (FVar 'v)))))
  (Steps
    (list (Unfold 'insert Lhs)                              ; (lookup k (MCons k v m))
          (Simp Lhs)                                        ; (if (int_eq k k) (Some v) (lookup k m))
          (Rewrite (Lemma 'int_eq_refl) Lr Lhs True (list)) ; (if True (Some v) ...)
          (Simp Lhs))                                       ; (Some v)
    Refl))

;; ---------------------------------------------------------------------------
;; Lemma 4 (conditional): ∀ j k : Int, v : V, m : (Map V).
;;   (int_eq j k) = False  ⊢  (lookup j (insert k v m)) = (lookup j m).
;;
;; Inserting at key k does not change what a DIFFERENT key j observes.
;; The disequality is a premise (never proven here — eqdec only proves
;; the `= True` direction); the proof consumes it with a Rewrite of
;; (Premise 0) to collapse the if's else-branch.
;; ---------------------------------------------------------------------------

(claim lookup_insert_neq
  (Goal
    (list (Param 'j (ty Int))
          (Param 'k (ty Int))
          (Param 'v (tv V))
          (Param 'm (ty Map (tv V))))
    (list
      (Equation (Call 'int_eq (list (FVar 'j) (FVar 'k))) (Ctor 'False (list))))
    (Equation
      (Call 'lookup
        (list (FVar 'j) (Call 'insert (list (FVar 'k) (FVar 'v) (FVar 'm)))))
      (Call 'lookup (list (FVar 'j) (FVar 'm)))))
  (Steps
    (list (Unfold 'insert Lhs)                            ; (lookup j (MCons k v m))
          (Simp Lhs)                                      ; (if (int_eq j k) (Some v) (lookup j m))
          (Rewrite (Premise 0) Lr Lhs True (list))        ; (if False (Some v) (lookup j m))
          (Simp Lhs))                                     ; (lookup j m)
    Refl))

;; ---------------------------------------------------------------------------
;; Lemma 5 (extensional capstone): ∀ j k : Int, v1 v2 : V, m : (Map V).
;;   (lookup j (insert k v2 (insert k v1 m))) = (lookup j (insert k v2 m)).
;;
;; A second insert at the same key shadows the first. This is NOT a
;; structural map equality (the LHS map literally carries the stale
;; (k,v1) entry) — it holds only under observation, so it's stated
;; extensionally over the probe key j and proven by case-splitting on
;; whether j hits k.
;;
;;   True  branch (j = k): both sides look up to (Some v2) — the shadow
;;                         entry is never reached on either side.
;;   False branch (j ≠ k): both sides skip the k-entries entirely and
;;                         look up to (lookup j m).
;;
;; Each `Rewrite (Hyp 0)` consumes the case hypothesis (int_eq j k) =
;; True/False that CaseOn introduced.
;; ---------------------------------------------------------------------------

(claim insert_shadow
  (Goal
    (list (Param 'j  (ty Int))
          (Param 'k  (ty Int))
          (Param 'v1 (tv V))
          (Param 'v2 (tv V))
          (Param 'm  (ty Map (tv V))))
    (list)
    (Equation
      (Call 'lookup
        (list (FVar 'j)
              (Call 'insert
                (list (FVar 'k) (FVar 'v2)
                      (Call 'insert (list (FVar 'k) (FVar 'v1) (FVar 'm)))))))
      (Call 'lookup
        (list (FVar 'j)
              (Call 'insert (list (FVar 'k) (FVar 'v2) (FVar 'm)))))))
  (CaseOn (Call 'int_eq (list (FVar 'j) (FVar 'k))) 'Bool
    (list
      ;; j = k: LHS outer entry (k,v2) is the hit; RHS entry (k,v2) too.
      (Case 'True
        (Steps
          (list (Unfold 'insert Lhs)                      ; LHS: (lookup j (MCons k v2 (insert k v1 m)))
                (Simp Lhs)                                ;      (if (int_eq j k) (Some v2) (lookup j (insert k v1 m)))
                (Rewrite (Hyp 0) Lr Lhs True (list))      ;      (if True (Some v2) ...)
                (Simp Lhs)                                ;      (Some v2)
                (Unfold 'insert Rhs)                      ; RHS: (lookup j (MCons k v2 m))
                (Simp Rhs)                                ;      (if (int_eq j k) (Some v2) (lookup j m))
                (Rewrite (Hyp 0) Lr Rhs True (list))      ;      (if True (Some v2) ...)
                (Simp Rhs))                               ;      (Some v2)
          Refl))
      ;; j ≠ k: skip both k-entries on the LHS, the one on the RHS.
      (Case 'False
        (Steps
          (list (Unfold 'insert Lhs)                      ; LHS: (lookup j (MCons k v2 (insert k v1 m)))
                (Simp Lhs)                                ;      (if (int_eq j k) (Some v2) (lookup j (insert k v1 m)))
                (Rewrite (Hyp 0) Lr Lhs True (list))      ;      (if False ... (lookup j (insert k v1 m)))
                (Simp Lhs)                                ;      (lookup j (insert k v1 m))
                (Unfold 'insert Lhs)                      ;      (lookup j (MCons k v1 m))
                (Simp Lhs)                                ;      (if (int_eq j k) (Some v1) (lookup j m))
                (Rewrite (Hyp 0) Lr Lhs True (list))      ;      (if False ... (lookup j m))
                (Simp Lhs)                                ;      (lookup j m)
                (Unfold 'insert Rhs)                      ; RHS: (lookup j (MCons k v2 m))
                (Simp Rhs)                                ;      (if (int_eq j k) (Some v2) (lookup j m))
                (Rewrite (Hyp 0) Lr Rhs True (list))      ;      (if False ... (lookup j m))
                (Simp Rhs))                               ;      (lookup j m)
          Refl)))))
