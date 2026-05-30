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

;; ---------------------------------------------------------------------------
;; len-decrease lemmas — the termination obligations for loop_eq's WfInduct.
;; Each function (skip_ws, take_digits) branches on ONE predicate, which we
;; CaseOn directly (via CaseB-named head c) — no char-class disjointness here.
;; ---------------------------------------------------------------------------

;; skip_ws never grows the list.
(claim len_skipws_le
  (Goal (list (Param 'cs (ty List Int))) (list)
    (Equation (Call 'le (list (Call 'len (list (Call 'skip_ws (list (FVar 'cs)))))
                              (Call 'len (list (FVar 'cs)))))
              (Ctor 'True (list))))
  (Induct 'cs
    (list
      (Case 'Nil (Steps (list (Simp Lhs)) Refl))
      (CaseB 'Cons (list 'c 'rest)
        (CaseOn (Call 'is_ws (list (FVar 'c))) 'Bool
          (list
            (Case 'True
              (Steps (list (Simp Lhs) (Rewrite (Hyp 0) Lr Lhs True (list)) (Simp Lhs))
                (RewriteWith (Lemma 'le_succ_l) Lr Lhs (list)
                  (list (Steps (list (Rewrite (Hyp 1) Lr Lhs True (list))) Refl))
                  Refl)))
            (Case 'False
              (Steps (list (Simp Lhs) (Rewrite (Hyp 0) Lr Lhs True (list)) (Simp Lhs)
                          (Rewrite (Lemma 'le_refl) Lr Lhs True (list)))
                Refl))))))))

;; take_digits' leftover never grows the list.
(claim len_takedigits_le
  (Goal (list (Param 'cs (ty List Int)) (Param 'acc (ty Int))) (list)
    (Equation (Call 'le (list
                (Call 'len (list (Call 'numr_rest (list (Call 'take_digits (list (FVar 'cs) (FVar 'acc)))))))
                (Call 'len (list (FVar 'cs)))))
              (Ctor 'True (list))))
  (Induct 'cs
    (list
      (Case 'Nil (Steps (list (Simp Lhs)) Refl))
      (CaseB 'Cons (list 'c 'rest)
        (CaseOn (Call 'is_digit (list (FVar 'c))) 'Bool
          (list
            (Case 'True
              (Steps (list (Simp Lhs) (Rewrite (Hyp 0) Lr Lhs True (list)) (Simp Lhs))
                (RewriteWith (Lemma 'le_succ_l) Lr Lhs (list)
                  (list (Steps (list (Rewrite (Hyp 1) Lr Lhs True (list))) Refl))
                  Refl)))
            (Case 'False
              (Steps (list (Simp Lhs) (Rewrite (Hyp 0) Lr Lhs True (list)) (Simp Lhs)
                          (Rewrite (Lemma 'le_refl) Lr Lhs True (list)))
                Refl))))))))

;; The strict decrease loop_eq needs: a number starting with a digit leaves a
;; strictly shorter suffix. len_takedigits_le bound + lt_from_le_succ.
(claim len_takedigits_lt
  (Goal (list (Param 'c (ty Int)) (Param 'rest (ty List Int)))
    (list (Equation (Call 'is_digit (list (FVar 'c))) (Ctor 'True (list))))
    (Equation (Call 'lt (list
                (Call 'len (list (Call 'numr_rest (list (Call 'take_digits
                  (list (FVar 'rest) (Call 'digit_val (list (FVar 'c)))))))))
                (Call 'len (list (Ctor 'Cons (list (FVar 'c) (FVar 'rest)))))))
              (Ctor 'True (list))))
  (Steps (list (Simp Lhs))
    (RewriteWith (Lemma 'lt_from_le_succ) Lr Lhs (list)
      (list (Steps (list (Rewrite (Lemma 'len_takedigits_le) Lr Lhs True (list))) Refl))
      Refl)))

;; ---------------------------------------------------------------------------
;; char-class disjointness — a whitespace byte (c <= 32, the unfolded is_ws)
;; is not a digit and not an operator. All one-step farkas now (le=False and
;; int_eq=False goals), since is_ws is a range. These let the lexer/parser
;; reduce past their guards when the head is whitespace.
;; ---------------------------------------------------------------------------
(claim le32_le48_false        ; c <= 32  ⊢  ¬(48 <= c)
  (Goal (list (Param 'c (ty Int)))
    (list (Equation (Call 'le (list (FVar 'c) (IntLit 32))) (Ctor 'True (list))))
    (Equation (Call 'le (list (IntLit 48) (FVar 'c))) (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

(claim le32_not_43           ; c <= 32  ⊢  c ≠ 43
  (Goal (list (Param 'c (ty Int)))
    (list (Equation (Call 'le (list (FVar 'c) (IntLit 32))) (Ctor 'True (list))))
    (Equation (Call 'int_eq (list (FVar 'c) (IntLit 43))) (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

(claim le32_not_45           ; c <= 32  ⊢  c ≠ 45
  (Goal (list (Param 'c (ty Int)))
    (list (Equation (Call 'le (list (FVar 'c) (IntLit 32))) (Ctor 'True (list))))
    (Equation (Call 'int_eq (list (FVar 'c) (IntLit 45))) (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

(claim le32_not_digit        ; c <= 32  ⊢  is_digit c = False
  (Goal (list (Param 'c (ty Int)))
    (list (Equation (Call 'le (list (FVar 'c) (IntLit 32))) (Ctor 'True (list))))
    (Equation (Call 'is_digit (list (FVar 'c))) (Ctor 'False (list))))
  (Steps (list (Unfold 'is_digit Lhs))
    (RewriteWith (Lemma 'le32_le48_false) Lr Lhs (list)
      (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
    (Steps (list (Simp Lhs)) Refl))))

;; Lexing from None over a whitespace head skips it (the lexer's else-chain
;; falls through is_digit/+/- to the is_ws branch). Uses the disjointness
;; lemmas to drive past each guard.
(claim lex_ws_none
  (Goal (list (Param 'c (ty Int)) (Param 'rest (ty List Int)))
    (list (Equation (Call 'le (list (FVar 'c) (IntLit 32))) (Ctor 'True (list))))
    (Equation
      (Call 'lex_go (list (Ctor 'Cons (list (FVar 'c) (FVar 'rest))) (Ctor 'None (list))))
      (Call 'lex_go (list (FVar 'rest) (Ctor 'None (list))))))
  (Steps (list (Unfold 'lex_go Lhs) (Simp Lhs))
    (RewriteWith (Lemma 'le32_not_digit) Lr Lhs (list)
      (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
    (Steps (list (Simp Lhs))
    (RewriteWith (Lemma 'le32_not_43) Lr Lhs (list)
      (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
    (Steps (list (Simp Lhs))
    (RewriteWith (Lemma 'le32_not_45) Lr Lhs (list)
      (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
    (Steps (list (Simp Lhs) (Unfold 'is_ws Lhs)
                 (Rewrite (Premise 0) Lr Lhs True (list)) (Simp Lhs))
    Refl))))))))

;; ws_lex: leading whitespace doesn't affect lexing from the None state.
;;   lex_go cs None = lex_go (skip_ws cs) None
;; Induct cs, CaseOn (le c 32) (= the unfolded is_ws guard). The whitespace
;; case rewrites both sides to lex_go rest None / lex_go (skip_ws rest) None
;; (lex_ws_none on the left, skip_ws reduction on the right) — that's the IH.
(claim ws_lex
  (Goal (list (Param 'cs (ty List Int))) (list)
    (Equation
      (Call 'lex_go (list (FVar 'cs) (Ctor 'None (list))))
      (Call 'lex_go (list (Call 'skip_ws (list (FVar 'cs))) (Ctor 'None (list))))))
  (Induct 'cs
    (list
      (Case 'Nil (Steps (list (Simp Both)) Refl))
      (CaseB 'Cons (list 'c 'rest)
        (CaseOn (Call 'le (list (FVar 'c) (IntLit 32))) 'Bool
          (list
            ;; whitespace: skip_ws drops c; lex_ws_none drops c on the left; IH closes.
            (Case 'True
              (RewriteWith (Lemma 'lex_ws_none) Lr Lhs (list)
                (list (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
              (Steps (list (Unfold 'skip_ws Rhs) (Simp Rhs) (Unfold 'is_ws Rhs)
                           (Rewrite (Hyp 0) Lr Rhs True (list)) (Simp Rhs)
                           (Rewrite (Hyp 1) Lr Lhs True (list)))
                Refl)))
            ;; non-whitespace: skip_ws returns the list unchanged; both sides identical.
            ;; Reduce (ι-only) so the recovered lex_go (Cons c rest) None stays FOLDED
            ;; (matching the LHS) rather than being unfolded into its if-chain.
            (Case 'False
              (Steps (list (Unfold 'skip_ws Rhs) (Reduce Rhs) (Unfold 'is_ws Rhs)
                           (Rewrite (Hyp 0) Lr Rhs True (list)) (Reduce Rhs))
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
