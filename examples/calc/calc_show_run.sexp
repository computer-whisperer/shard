;;; The calculator's digit spec over Int VALUES (option (b)): codes maps a
;;; digit-value list to codepoints, and we prove the lexer correct DIRECTLY
;;; for show n via WfInduct (recursion in terms of n/10, mod n 10 — all
;;; named, so no Induct fresh-field-name pinning). Builds toward
;;;   run (codes (show n) ++ "+" ++ codes (show m)) = Some (n + m).

(import "calc_show.sexp")    ; show, valI, valI_go, valI_go_snoc, show_correct
(import "calc_proof.sexp")   ; is_digit_of_digit (0<=d,d<=9 |- is_digit(48+d)) + calc.sexp
(import "../../std/div.sexp")
(import "../../std/list.sexp")

;; digit-value list -> codepoint list.
(fn codes ((ds (List Int))) (List Int)
  (match ds
    (Nil Nil)
    ((Cons d rest) (Cons (+ 48 d) (codes rest)))))

(claim codes_cons
  (Goal (list (Param 'd (ty Int)) (Param 'ds (ty List Int))) (list)
    (Equation
      (Call 'codes (list (Ctor 'Cons (list (FVar 'd) (FVar 'ds)))))
      (Ctor 'Cons (list (Call '+ (list (IntLit 48) (FVar 'd))) (Call 'codes (list (FVar 'ds)))))))
  (Steps (list (Unfold 'codes Lhs) (Reduce Lhs)) Refl))

(claim append_int_cons
  (Goal (list (Param 'h (ty Int)) (Param 't (ty List Int)) (Param 'ys (ty List Int))) (list)
    (Equation
      (Call 'append (list (Ctor 'Cons (list (FVar 'h) (FVar 't))) (FVar 'ys)))
      (Ctor 'Cons (list (FVar 'h) (Call 'append (list (FVar 't) (FVar 'ys)))))))
  (Steps (list (Simp Lhs)) Refl))

;; codes distributes over append (structural induction on xs; matched IH).
(claim codes_append
  (Goal (list (Param 'xs (ty List Int)) (Param 'ys (ty List Int))) (list)
    (Equation
      (Call 'codes (list (Call 'append (list (FVar 'xs) (FVar 'ys)))))
      (Call 'append (list (Call 'codes (list (FVar 'xs))) (Call 'codes (list (FVar 'ys)))))))
  (Induct 'xs
    (list
      (Case 'Nil (Steps (list (Simp Both)) Refl))
      (Case 'Cons (Steps (list (Simp Lhs) (Rewrite (Hyp 0) Lr Lhs True (list)) (Simp Rhs)) Refl)))))

(claim mul_comm10
  (Goal (list (Param 'a (ty Int))) (list)
    (Equation (Call '* (list (FVar 'a) (IntLit 10))) (Call '* (list (IntLit 10) (FVar 'a)))))
  (ByTheory 'lia (Cert 'lia (list))))

(claim digit_val_id
  (Goal (list (Param 'd (ty Int))) (list)
    (Equation (Call 'digit_val (list (Call '+ (list (IntLit 48) (FVar 'd))))) (FVar 'd)))
  (Steps (list (Unfold 'digit_val Lhs)) (ByTheory 'lia (Cert 'lia (list)))))

;; One digit step (acc = Some), given the codepoint is a digit char.
;; Produces (* 10 acc) order (matching valI_go) by bridging acc_digit's
;; (* acc 10) via mul_comm10 inside the still-open Some argument.
(claim lex_go_digit
  (Goal
    (list (Param 'd (ty Int)) (Param 'cs (ty List Int)) (Param 'acc (ty Int)))
    (list (Equation (Call 'is_digit (list (Call '+ (list (IntLit 48) (FVar 'd))))) (Ctor 'True (list))))
    (Equation
      (Call 'lex_go (list (Ctor 'Cons (list (Call '+ (list (IntLit 48) (FVar 'd))) (FVar 'cs))) (Ctor 'Some (list (FVar 'acc)))))
      (Call 'lex_go (list (FVar 'cs) (Ctor 'Some (list (Call '+ (list (Call '* (list (IntLit 10) (FVar 'acc))) (FVar 'd)))))))))
  (Steps
    (list (Unfold 'lex_go Lhs) (Reduce Lhs)
          (Rewrite (Premise 0) Lr Lhs True (list))     ; is_digit(48+d) -> True
          (Reduce Lhs)                                 ; if True .. -> lex_go cs (Some (acc_digit (Some acc) (48+d)))
          (Unfold 'acc_digit Lhs) (Reduce Lhs)         ; -> Some (+ (* acc 10) (digit_val (48+d)))
          (Rewrite (Lemma 'digit_val_id) Lr Lhs True (list))   ; digit_val(48+d) -> d
          (Rewrite (Lemma 'mul_comm10)  Lr Lhs True (list)))   ; (* acc 10) -> (* 10 acc)
    Refl))

;; n < 10 (loop-guard hyp) gives n <= 9, for is_digit_of_digit's upper bound.
(claim lt10_le9
  (Goal (list (Param 'n (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'n) (IntLit 10))) (Ctor 'True (list))))
    (Equation (Call 'le (list (FVar 'n) (IntLit 9))) (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))
