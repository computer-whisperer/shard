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
