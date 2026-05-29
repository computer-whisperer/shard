;;; ============================================================================
;;; THE REQUIREMENT — the full input/output relation for the calculator,
;;; written as a trusted reference BEFORE (in spirit) the implementation.
;;;
;;; This is the "spec-first" artifact: a human+agent author writes `spec_run`
;;; as the obvious, auditable description of intended behavior over the raw
;;; input string (a List Int of codepoints), and the headline obligation is
;;;
;;;        REQUIREMENT:   for all cs : List Int,   run cs = spec_run cs
;;;
;;; That single equation pins `run` on EVERY input — valid or not — so it
;;; captures both halves of the intent at once:
;;;   * accept:  a string that fits the grammar evaluates to its value;
;;;   * reject:  a string that does NOT fit returns None.
;;;
;;; The grammar (intended structure), with optional whitespace between tokens:
;;;
;;;        expr   ::=  ws* number ( ws* ('+'|'-') ws* number )* ws*
;;;        number ::=  digit+                      (digit = '0'..'9')
;;;        ws     ::=  ' ' | '\t' | '\n'
;;;
;;; Left-associative, no precedence (only +/- at one level) — matching what
;;; the implementation's parser already accepts. Numbers are non-negative
;;; LITERALS; negative results arise only from binary '-'.
;;;
;;; NOTE — this requirement is deliberately the HARD/representative shape:
;;; a recursive-descent reference parser that threads the UNCONSUMED SUFFIX
;;; and is total only by a length measure (not structural). It is exactly
;;; the kind of spec a real project would write, and proving `run = spec_run`
;;; against it is a genuine reference-model equivalence (two structurally
;;; different programs — staged lex+parse vs. fused recursive descent —
;;; computing the same I/O relation).
;;;
;;; NOTE — as discussed, `run = spec_run` is currently FALSE: the live lexer
;;; is liberal (it silently skips ANY unrecognized byte as a separator), so
;;; e.g. run "1@+2" = Some 3 while spec_run "1@+2" = None. The spec is doing
;;; its job: it pins down that "1@+2" is off-grammar. Reconciling the lexer
;;; to this spec (reject non-ws/non-digit/non-op bytes) is the first thing
;;; the requirement forces.
;;; ============================================================================

(import "calc.sexp")   ; shared vocabulary: Exp, eval, eval_opt, is_digit,
                       ; digit_val  (and the implementation: lex / parse / run)

;; ---------------------------------------------------------------------------
;; Spec-internal plumbing: a parse step returns what it produced together with
;; the still-unconsumed suffix of the input. (No generic Pair in std, and a
;; named result type reads better in a requirement anyway.)
;; ---------------------------------------------------------------------------
(type NumRes (NumR Int (List Int)))   ; a parsed number value + the rest
(type ExpRes (ExpR Exp (List Int)))   ; a parsed expression  + the rest

;; `is_ws` (whitespace = space/tab/newline) is shared vocabulary, defined
;; in calc.sexp alongside is_digit — the lexer and this spec agree on
;; exactly which bytes are skippable separators.

;; drop a leading run of whitespace (structural on the tail — total).
(fn skip_ws ((cs (List Int))) (List Int)
  (match cs
    (Nil Nil)
    ((Cons c rest) (if (is_ws c) (skip_ws rest) cs))))

;; fold a maximal run of digit codepoints into `acc`, MSD-first Horner.
;; Stops at the first non-digit; returns (value, rest). Structural — total.
(fn take_digits ((cs (List Int)) (acc Int)) NumRes
  (match cs
    (Nil (NumR acc Nil))
    ((Cons c rest)
      (if (is_digit c)
          (take_digits rest (+ (* acc 10) (digit_val c)))
          (NumR acc cs)))))

;; a number is one-or-more digits. None if the head is not a digit.
(fn parse_num ((cs (List Int))) (Option NumRes)
  (match cs
    (Nil None)
    ((Cons c rest)
      (if (is_digit c)
          (Some (take_digits rest (digit_val c)))
          None))))

;; build the operator node (avoids passing a constructor as a value).
(fn combine ((is_add Bool) (a Exp) (b Exp)) Exp
  (if is_add (Add a b) (Sub a b)))

;; the tail of an expression: given the left expr `acc` parsed so far and the
;; remaining input, consume zero or more  ( ws* op ws* number )  groups.
;;   * end of input (after trailing ws)        -> success, nothing left
;;   * an operator, then (ws*) a required number -> extend left-assoc, recurse
;;   * anything else                            -> reject (None)
;; Recurses on a STRICT SUFFIX (at least the operator byte is consumed before
;; the recursive call), so it is total by the length measure, not structurally.
(fn parse_tail ((acc Exp) (cs (List Int))) (Option ExpRes)
  (match (skip_ws cs)
    (Nil (Some (ExpR acc Nil)))
    ((Cons c rest)
      (if (int_eq c 43)
          (match (parse_num (skip_ws rest))
            (None None)
            ((Some (NumR n rest2)) (parse_tail (combine True acc (Num n)) rest2)))
          (if (int_eq c 45)
              (match (parse_num (skip_ws rest))
                (None None)
                ((Some (NumR n rest2)) (parse_tail (combine False acc (Num n)) rest2)))
              None)))))

;; top level: ws* number <tail> ws*, and the input must be FULLY consumed.
(fn parse_expr ((cs (List Int))) (Option Exp)
  (match (parse_num (skip_ws cs))
    (None None)
    ((Some (NumR n rest))
      (match (parse_tail (Num n) rest)
        (None None)
        ((Some (ExpR e rest2))
          (match rest2
            (Nil (Some e))
            ((Cons _ _) None)))))))   ; leftover after a complete parse -> reject

;; THE REFERENCE: text -> value, None off-grammar. Shares `eval`/`eval_opt`
;; with the implementation (the meaning of an Exp is not in dispute; the
;; parsing structure is what the spec defines independently).
(fn spec_run ((cs (List Int))) (Option Int)
  (eval_opt (parse_expr cs)))
