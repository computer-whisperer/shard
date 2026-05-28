;;; Unary natural numbers. Standard Z / S encoding, with structural
;;; addition recursing on the first argument.
;;;
;;; Loaded as a user module by add_nat_zero.sexp.

(type Nat
  (Z)
  (S Nat))

(fn add_nat ((a Nat) (b Nat)) Nat
  (match a
    (Z       b)
    ((S k)   (S (add_nat k b)))))
