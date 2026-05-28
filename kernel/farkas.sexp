;;; farkas: linear-integer ENTAILMENT via a Farkas-style certificate.
;;;
;;; The fourth ByTheory backend. Where lia/eqdec/ord decide TAUTOLOGIES
;;; (validity with no hypotheses), farkas decides ENTAILMENT:
;;;
;;;     premises ⊢ (lt a b) = True      premises ⊢ (le a b) = True
;;;
;;; — i.e. the goal's conditional premises (a conjunction of linear
;;; (in)equalities) imply a linear order conclusion. This is what the
;;; M3 loop invariant needs (e.g. `p < i ⊢ p < i+1`), which the
;;; tautology backends cannot do because they ignore premises.
;;;
;;; CERTIFICATE, NOT SEARCH. Refuting `premises ∧ ¬goal` over linear
;;; integers needs a Farkas combination: nonnegative multipliers on the
;;; constraints whose weighted sum is a manifest `0 >= 1`. FINDING the
;;; multipliers is the untrusted proposer's job; the kernel only CHECKS
;;; them. So this backend is the first real user of the `Cert` payload:
;;;
;;;     (Cert 'farkas (list G M0 M1 ...))    ; narrow Ints
;;;       G       — multiplier on the negated goal   (must be >= 0)
;;;       Mk      — multiplier on premise k, in order (0 to skip)
;;;
;;; Each constraint is normalized to `expr >= 0` (for inequalities) or
;;; `expr = 0` (for equalities). The negated goal is also `expr >= 0`.
;;; The checker forms  G*neg_goal + Σ Mk*premise_k , canonicalizes, and
;;; accepts iff the result is a CONSTANT c with c < 0 — because a
;;; nonnegative combination of true `>= 0` facts (plus signed multiples
;;; of `= 0` facts) is itself `>= 0`, so a negative constant is a
;;; contradiction, refuting the assumption ¬goal.
;;;
;;; TWO SOUNDNESS-CRITICAL GUARDS (the review focus):
;;;   1. INEQUALITY constraints may only take NONNEGATIVE multipliers
;;;      (a negative multiple of `e >= 0` flips it to `<= 0` — unsound).
;;;      Equality constraints (`e = 0`) may take ANY sign. The goal
;;;      multiplier G must be >= 0.
;;;   2. The combination must canonicalize to a lone CONSTANT (all
;;;      variables cancel) that is strictly < 0.
;;;
;;; Conclusions are order facts (`lt`/`le` = True) OR disequalities
;;; (`int_eq a b` = False). A disequality goal is refuted by assuming
;;; the EQUALITY `a = b` (`a - b = 0`, an any-sign constraint) and
;;; combining it with the premises to a negative constant — e.g.
;;; `(lt a b)=True ⊢ (int_eq a b)=False` (from a < b, a ≠ b), which is
;;; what the M3 swap framing needs to turn loop bounds into the
;;; `(int_eq p i)=False` premises read_swap_other consumes.
;;;
;;; Scope / caveats (same family as lia): EQUALITY conclusions
;;; (`a = b`) stay with lia (a tautology) or would need a two-sided
;;; combination (future). Opaque
;;; atoms are assumed integer-typed (inherited from lia_collect). A
;;; premise that isn't a recognized linear (in)equality is usable only
;;; with multiplier 0; a nonzero multiplier on an uninterpretable
;;; premise is rejected. See REVISIT — "farkas — linear entailment".

;; ---------------------------------------------------------------------------
;; Polynomial builders (a Polynomial is lia's (List (Pair Int (Option Expr)))).
;; ---------------------------------------------------------------------------

;; b - a, as a polynomial.
(fn poly_sub ((b Expr) (a Expr)) (List (Pair Int (Option Expr)))
  (lia_concat (lia_collect b 1) (lia_collect a (- 0 1))))

;; p - 1 (prepend a -1 constant monomial; canonical merges later).
(fn poly_minus1 ((p (List (Pair Int (Option Expr))))) (List (Pair Int (Option Expr)))
  (Cons (Pair (- 0 1) None) p))

;; ---------------------------------------------------------------------------
;; Constraint interpretation. Each returns the `>= 0` (ineq) or `= 0`
;; (eq) polynomial plus a Bool `needs_nonneg` (True ⟹ the multiplier
;; must be nonnegative).
;; ---------------------------------------------------------------------------

;; (cmp a b) = rhs, cmp = le (is_le True) or lt (is_le False). Both
;; orientations of rhs are linear inequalities:
;;   le True : b - a >= 0      le False (a>b): a - b - 1 >= 0
;;   lt True : b - a - 1 >= 0  lt False (a>=b): a - b >= 0
(fn cmp_constraint ((a Expr) (b Expr) (rhs Expr) (is_le Bool))
                   (Option (Pair (List (Pair Int (Option Expr))) Bool))
  (match rhs
    ((Ctor tn _)
      (if (sym_eq tn (quote True))
          (Some (Pair (if is_le (poly_sub b a) (poly_minus1 (poly_sub b a))) True))
          (if (sym_eq tn (quote False))
              (Some (Pair (if is_le (poly_minus1 (poly_sub a b)) (poly_sub a b)) True))
              None)))                                ; rhs neither True nor False
    (_ None)))

;; (int_eq a b) = True  →  a - b = 0 (equality, any-sign multiplier).
;; (int_eq a b) = False is a DISEQUALITY (not linear) → None.
(fn inteq_constraint ((a Expr) (b Expr) (rhs Expr))
                     (Option (Pair (List (Pair Int (Option Expr))) Bool))
  (match rhs
    ((Ctor tn _)
      (if (sym_eq tn (quote True))
          (Some (Pair (poly_sub a b) False))
          None))
    (_ None)))

;; A plain equation lhs = rhs treated as the equality lhs - rhs = 0.
;; (Sound: the premise is assumed true in the sequent. Non-integer
;; sides become opaque atoms that either cancel or block the constant
;; contradiction — never unsound, just incomplete.)
(fn plain_eq_constraint ((lhs Expr) (rhs Expr))
                        (Pair (List (Pair Int (Option Expr))) Bool)
  (Pair (poly_sub lhs rhs) False))

(fn premise_to_constraint ((eq Equation))
                          (Option (Pair (List (Pair Int (Option Expr))) Bool))
  (match eq
    ((Equation lhs rhs)
      (match lhs
        ((Call f args)
          (match args
            ((Cons a (Cons b Nil))
              (if (sym_eq f (quote le))
                  (cmp_constraint a b rhs True)
                  (if (sym_eq f (quote lt))
                      (cmp_constraint a b rhs False)
                      (if (sym_eq f (quote int_eq))
                          (inteq_constraint a b rhs)
                          (Some (plain_eq_constraint lhs rhs))))))
            (_ (Some (plain_eq_constraint lhs rhs)))))
        (_ (Some (plain_eq_constraint lhs rhs)))))))

;; The negated goal, as a constraint polynomial paired with its
;; `needs_nonneg` flag (the goal-multiplier's sign rule):
;;   goal (le a b)=True   → ¬ is  a - b - 1 >= 0   (inequality, nonneg)
;;   goal (lt a b)=True   → ¬ is  a - b     >= 0   (inequality, nonneg)
;;   goal (int_eq a b)=False → ¬ is the EQUALITY a - b = 0 (any sign)
;; A disequality goal is refuted by assuming a = b and combining with
;; the (linear) premises; since the assumed a=b is an equality, its
;; multiplier may take any sign — hence needs_nonneg False.
(fn neg_goal_poly ((goal Equation))
                  (Option (Pair (List (Pair Int (Option Expr))) Bool))
  (match goal
    ((Equation lhs rhs)
      (match lhs
        ((Call f args)
          (match args
            ((Cons a (Cons b Nil))
              (match rhs
                ((Ctor tn _)
                  (if (sym_eq tn (quote True))
                      (if (sym_eq f (quote le))
                          (Some (Pair (poly_minus1 (poly_sub a b)) True))
                          (if (sym_eq f (quote lt))
                              (Some (Pair (poly_sub a b) True))
                              None))
                      (if (sym_eq tn (quote False))
                          (if (sym_eq f (quote int_eq))
                              (Some (Pair (poly_sub a b) False))
                              None)
                          None)))
                (_ None)))
            (_ None)))
        (_ None)))))

;; ---------------------------------------------------------------------------
;; Combine premises with their multipliers, accumulating into `acc`.
;; GUARD 1 lives here: an inequality (needs_nonneg) premise with a
;; negative multiplier is rejected (None); a nonzero multiplier on an
;; uninterpretable premise is rejected.
;; ---------------------------------------------------------------------------

(fn farkas_combine ((premises (List Equation)) (mults (List Int))
                    (acc (List (Pair Int (Option Expr)))))
                   (Option (List (Pair Int (Option Expr))))
  (match premises
    (Nil (Some acc))
    ((Cons prem prest)
      (match mults
        (Nil (farkas_combine prest Nil acc))         ; missing multiplier = 0, skip
        ((Cons m mrest)
          (if (int_eq m 0)
              (farkas_combine prest mrest acc)        ; skip premise
              (match (premise_to_constraint prem)
                (None None)                           ; nonzero mult, uninterpretable → reject
                ((Some (Pair poly needs_nonneg))
                  (if needs_nonneg
                      (if (lt m 0)
                          None                        ; GUARD 1: neg mult on inequality
                          (farkas_combine prest mrest
                            (lia_concat (lia_scale m poly) acc)))
                      (farkas_combine prest mrest
                        (lia_concat (lia_scale m poly) acc)))))))))))

;; GUARD 2: the canonicalized combination is a lone constant c < 0.
(fn farkas_contradiction ((p (List (Pair Int (Option Expr))))) Bool
  (match (lia_const_value p)
    ((Some c) (lt c 0))
    (None     False)))

;; Form the combination and test for contradiction (shared by both
;; goal-multiplier sign regimes below).
(fn farkas_finish ((premises (List Equation)) (prem_mults (List Int))
                   (ng (List (Pair Int (Option Expr)))) (goal_mult Int)) Bool
  (match (farkas_combine premises prem_mults (lia_scale goal_mult ng))
    (None False)
    ((Some total) (farkas_contradiction (lia_canonical total)))))

;; ---------------------------------------------------------------------------
;; Entry point. cert payload = (list G M0 M1 ...): G is the negated-goal
;; multiplier, Mk the premise-k multipliers (aligned, in order).
;; ---------------------------------------------------------------------------

(fn farkas_check ((premises (List Equation)) (goal Equation) (cert Cert)) Bool
  (match cert
    ((Cert _ payload)
      (match payload
        ((Cons goal_mult prem_mults)
          (match (neg_goal_poly goal)
            (None False)                             ; goal not an order/diseq conclusion
            ((Some (Pair ng goal_needs_nonneg))
              ;; GUARD 1 on the goal multiplier: required nonneg only
              ;; when the negated goal is an inequality (lt/le goals);
              ;; an int_eq=False goal negates to an EQUALITY, any sign.
              (if goal_needs_nonneg
                  (if (lt goal_mult 0)
                      False
                      (farkas_finish premises prem_mults ng goal_mult))
                  (farkas_finish premises prem_mults ng goal_mult)))))
        (_ False)))))                                ; payload not a non-empty list
