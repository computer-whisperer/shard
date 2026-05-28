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
