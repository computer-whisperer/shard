;;; List operations over (List Int). Definitions for the reverse
;;; proof tower (rev = fast).
;;;
;;; All three fns are structurally recursive on their first list arg,
;;; so they're total in the kernel's accepted fragment.
;;;
;;; Monomorphic on Int. The proofs hold for any element type, but
;;; v2's erased polymorphism means each lemma is currently stated
;;; over (List Int) — see REVISIT, "Erased polymorphism in narrow".

;; append: (append xs ys) is xs ++ ys.
(fn append ((xs (List Int)) (ys (List Int))) (List Int)
  (match xs
    (Nil          ys)
    ((Cons h t)   (Cons h (append t ys)))))

;; rev: naive O(n^2) reverse. Each Cons re-appends a singleton.
(fn rev ((xs (List Int))) (List Int)
  (match xs
    (Nil          Nil)
    ((Cons h t)   (append (rev t) (Cons h Nil)))))

;; fast: accumulator-passing O(n) reverse.
(fn fast ((xs (List Int)) (acc (List Int))) (List Int)
  (match xs
    (Nil          acc)
    ((Cons h t)   (fast t (Cons h acc)))))
