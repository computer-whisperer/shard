;;; Toward the universal equivalence  ∀ cs. run cs = spec_run cs.
;;; This file builds the bridge + structural lemmas bottom-up; the heart
;;; (loop_eq, by WfInduct on len) and the top assembly (CORE, run_eq_spec)
;;; land in the next slice.
;;;
;;; Dependency order:
;;;   numr_val/numr_rest      — project NumRes
;;;   lex_num                 — the digit-run bridge (workhorse)
;;;   skipws_idem, ws_lex     — whitespace alignment
;;;   [next] loop_eq, CORE, run_eq_spec

(import "calc_spec.sexp")   ; brings calc.sexp (lex_go/flush/acc_digit/is_ws/Token)
                            ; + spec (take_digits/NumRes/skip_ws/parse_num)
(import "../../std/list.sexp")   ; len, len_nonneg

;; project a parsed-number result.
(fn numr_val  ((r NumRes)) Int        (match r ((NumR v rest) v)))
(fn numr_rest ((r NumRes)) (List Int) (match r ((NumR v rest) rest)))

;; ---------------------------------------------------------------------------
;; lex_num — THE DIGIT-RUN BRIDGE. Lexing from the mid-number state (Some acc)
;; reads exactly the digit run that take_digits reads, flushes it as one
;; TNum carrying take_digits' value, then lexes the leftover from None:
;;
;;   lex_go cs (Some acc)
;;     = TNum (numr_val (take_digits cs acc)) :: lex_go (numr_rest (take_digits cs acc)) None
;;
;; Induction on cs, CaseOn (is_digit c): the digit case fires the IH at the
;; STEPPED accumulator (empty-inst match, the valI_go_snoc / lex_show_run
;; shape); the non-digit case is `flush (Some acc) X = TNum acc :: X` against
;; `flush None X = X` (the lexer's else-chain `X` is identical on both sides).
;; ---------------------------------------------------------------------------
(claim lex_num
  (Goal
    (list (Param 'cs (ty List Int)) (Param 'acc (ty Int)))
    (list)
    (Equation
      (Call 'lex_go (list (FVar 'cs) (Ctor 'Some (list (FVar 'acc)))))
      (Ctor 'Cons (list
        (Ctor 'TNum (list (Call 'numr_val (list (Call 'take_digits (list (FVar 'cs) (FVar 'acc)))))))
        (Call 'lex_go (list
          (Call 'numr_rest (list (Call 'take_digits (list (FVar 'cs) (FVar 'acc)))))
          (Ctor 'None (list))))))))
  (Induct 'cs
    (list
      (Case 'Nil
        (Steps (list (Simp Both)) Refl))
      (CaseB 'Cons (list 'c 'rest)        ; name the head/tail so we can split on is_digit c
        (CaseOn (Call 'is_digit (list (FVar 'c))) 'Bool
          (list
            ;; digit: extend the number; IH (Hyp 1) at the stepped accumulator.
            (Case 'True
              (Steps
                (list (Simp Lhs)
                      (Rewrite (Hyp 0) Lr Lhs True (list))
                      (Simp Lhs)
                      (Simp Rhs)
                      (Rewrite (Hyp 0) Lr Rhs True (list))
                      (Simp Rhs)
                      (Rewrite (Hyp 1) Lr Lhs True (list)))
                Refl))
            ;; non-digit: flush. Both sides reduce to (TNum acc :: <lexer else-chain>);
            ;; is_digit c is rewritten False at each level it surfaces.
            (Case 'False
              (Steps
                (list (Simp Lhs)
                      (Rewrite (Hyp 0) Lr Lhs True (list))
                      (Simp Lhs)
                      (Simp Rhs)
                      (Rewrite (Hyp 0) Lr Rhs True (list))
                      (Simp Rhs)
                      (Rewrite (Hyp 0) Lr Rhs True (list))
                      (Simp Rhs))
                Refl))))))))

;; quick Compute validation: lex_go [50 43] (Some 1) = [TNum 12, TPlus]
;; and the RHS computes the same.
(claim lex_num_check
  (Goal (list) (list)
    (Equation
      (Call 'lex_go (list (Ctor 'Cons (list (IntLit 50) (Ctor 'Cons (list (IntLit 43) (Ctor 'Nil (list)))))) (Ctor 'Some (list (IntLit 1)))))
      (Ctor 'Cons (list
        (Ctor 'TNum (list (Call 'numr_val (list (Call 'take_digits (list (Ctor 'Cons (list (IntLit 50) (Ctor 'Cons (list (IntLit 43) (Ctor 'Nil (list)))))) (IntLit 1)))))))
        (Call 'lex_go (list (Call 'numr_rest (list (Call 'take_digits (list (Ctor 'Cons (list (IntLit 50) (Ctor 'Cons (list (IntLit 43) (Ctor 'Nil (list)))))) (IntLit 1))))) (Ctor 'None (list))))))))
  (Steps (list (Compute Both)) Refl))
