;;; Standard library: generic data types used throughout the kernel.
;;;
;;; No special evaluator support — ordinary user-defined types. All
;;; polymorphism is erased at runtime in narrow; type variables are
;;; annotations the full-language checker will eventually validate.
;;;
;;; Primitives expected from the evaluator (defined in Rust, exposed
;;; as ordinary function symbols here):
;;;   (int_eq Int Int) -> Bool
;;;   (sym_eq Symbol Symbol) -> Bool
;;;   (+ - * / mod : Int Int -> Int)
;;;   (lt le : Int Int -> Bool)
;;;   (band bor bxor bshl bshr : Int Int -> Int)
;;;
;;; The primitive comparison ops return values of the user-defined
;;; `Bool` type below — see REVISIT.md (Primitive comparisons return
;;; user Bool).

(type (List T)
  (Nil)
  (Cons T (List T)))

(type (Option T)
  (None)
  (Some T))

(type Bool
  (False)
  (True))

(type (Pair A B)
  (Pair A B))
